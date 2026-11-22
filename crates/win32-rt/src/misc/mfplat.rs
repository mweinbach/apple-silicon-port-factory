//! `mfplat.dll`: Native Microsoft Media Foundation platform implementation.
//!
//! Provides real ref-counted Media Foundation COM objects (with every vtable slot
//! registered via [`crate::register_native_va`]):
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

use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex, OnceLock};
use std::thread::ThreadId;

use crate::d3d12::com::{com_add_ref, com_query, com_release, guid, Guid, IID_IUnknown};
use crate::misc::ole32::{CoTaskMemAlloc, CoTaskMemFree};
use crate::types::{BOOL, E_FAIL, E_INVALIDARG, E_NOINTERFACE, E_NOTIMPL, E_OUTOFMEMORY, E_POINTER, FALSE, HANDLE, HRESULT, S_FALSE, S_OK, TRUE};

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

const VT_EMPTY: u16 = 0;
const VT_R8: u16 = 5;
const VT_UNKNOWN: u16 = 13;
const VT_UI4: u16 = 19;
const VT_UI8: u16 = 21;
const VT_LPWSTR: u16 = 31;
const VT_BLOB: u16 = 65;
const VT_CLSID: u16 = 72;
const VT_VECTOR_UI1: u16 = 0x1011; // 4113

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

// Common uncompressed media subtype GUIDs (for IMFMediaType::IsCompressedFormat)
const MF_AUDIO_FORMAT_PCM: Guid = guid("00000001-0000-0010-8000-00aa00389b71");
const MF_AUDIO_FORMAT_FLOAT: Guid = guid("00000003-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_RGB32: Guid = guid("00000016-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_ARGB32: Guid = guid("00000015-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_RGB24: Guid = guid("00000014-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_NV12: Guid = guid("3231564e-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_YUY2: Guid = guid("32595559-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_YV12: Guid = guid("32315659-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_IYUV: Guid = guid("56555949-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_I420: Guid = guid("30323449-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_UYVY: Guid = guid("59565955-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_P010: Guid = guid("30313050-0000-0010-8000-00aa00389b71");
const MF_VIDEO_FORMAT_AYUV: Guid = guid("56555941-0000-0010-8000-00aa00389b71");

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
// Platform Lifecycle (MFStartup / MFShutdown)
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

// ============================================================================
// Vtable Registration & Unaligned Guest Memory Helpers
// ============================================================================

fn register_vtable<const N: usize>(slots: [usize; N]) -> [usize; N] {
    for &addr in &slots {
        if addr != 0 {
            crate::register_native_va(addr as u64, addr);
        }
    }
    slots
}

#[inline]
unsafe fn read_guid(p: *const Guid) -> Option<Guid> {
    if p.is_null() {
        None
    } else {
        Some(std::ptr::read_unaligned(p))
    }
}

#[inline]
unsafe fn write_null_out(pp: *mut *mut c_void) {
    if !pp.is_null() {
        std::ptr::write_unaligned(pp, std::ptr::null_mut());
    }
}

unsafe fn read_wide_z(ptr: *const u16) -> Vec<u16> {
    if ptr.is_null() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut i = 0usize;
    loop {
        let ch = std::ptr::read_unaligned(ptr.add(i));
        if ch == 0 {
            break;
        }
        out.push(ch);
        i += 1;
    }
    out
}

/// Strided 2D image copy supporting both positive (top-down) and negative (bottom-up) strides.
unsafe fn copy_image_rows(
    dst: *mut u8,
    dst_stride: i32,
    src: *const u8,
    src_stride: i32,
    width_bytes: u32,
    lines: u32,
) -> HRESULT {
    if width_bytes == 0 || lines == 0 {
        return S_OK;
    }
    if dst.is_null() || src.is_null() {
        return E_INVALIDARG;
    }
    let w = width_bytes as usize;
    let ds = dst_stride as isize;
    let ss = src_stride as isize;
    for y in 0..lines as isize {
        let d_row = dst.offset(y * ds);
        let s_row = src.offset(y * ss);
        std::ptr::copy(s_row, d_row, w);
    }
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
    copy_image_rows(p_dest, l_dest_stride, p_src, l_src_stride, dw_width_in_bytes, dw_lines)
}

// ============================================================================
// Reentrant Lock & IMFAttributes Key-Value Store
// ============================================================================

/// Reentrant mutex for `IMFAttributes::LockStore` / `UnlockStore` so the owning
/// thread can freely call `Get*`/`Set*` while holding `LockStore`.
struct ReentrantLock {
    state: Mutex<(Option<ThreadId>, u32)>,
    cv: Condvar,
}

impl ReentrantLock {
    fn new() -> Self {
        Self {
            state: Mutex::new((None, 0)),
            cv: Condvar::new(),
        }
    }

    fn lock_raw(&self) {
        let me = std::thread::current().id();
        let mut st = self.state.lock().unwrap();
        while let Some(owner) = st.0 {
            if owner == me {
                st.1 += 1;
                return;
            }
            st = self.cv.wait(st).unwrap();
        }
        st.0 = Some(me);
        st.1 = 1;
    }

    fn unlock_raw(&self) -> bool {
        let me = std::thread::current().id();
        let mut st = self.state.lock().unwrap();
        if st.0 != Some(me) || st.1 == 0 {
            return false;
        }
        st.1 -= 1;
        if st.1 == 0 {
            st.0 = None;
            self.cv.notify_one();
        }
        true
    }

    fn with_lock<R>(&self, f: impl FnOnce() -> R) -> R {
        self.lock_raw();
        struct Guard<'a>(&'a ReentrantLock);
        impl Drop for Guard<'_> {
            fn drop(&mut self) {
                self.0.unlock_raw();
            }
        }
        let _g = Guard(self);
        f()
    }
}

#[derive(Clone)]
enum AttrValue {
    Uint32(u32),
    Uint64(u64),
    Double(f64),
    Guid(Guid),
    String(Vec<u16>), // UTF-16 without trailing NUL
    Blob(Vec<u8>),
    Unknown(usize), // AddRef'd COM pointer (or 0 for null)
}

impl AttrValue {
    fn mf_type(&self) -> u32 {
        match self {
            AttrValue::Uint32(_) => MF_ATTRIBUTE_UINT32,
            AttrValue::Uint64(_) => MF_ATTRIBUTE_UINT64,
            AttrValue::Double(_) => MF_ATTRIBUTE_DOUBLE,
            AttrValue::Guid(_) => MF_ATTRIBUTE_GUID,
            AttrValue::String(_) => MF_ATTRIBUTE_STRING,
            AttrValue::Blob(_) => MF_ATTRIBUTE_BLOB,
            AttrValue::Unknown(_) => MF_ATTRIBUTE_IUNKNOWN,
        }
    }

    fn equals(&self, other: &AttrValue) -> bool {
        match (self, other) {
            (AttrValue::Uint32(a), AttrValue::Uint32(b)) => a == b,
            (AttrValue::Uint64(a), AttrValue::Uint64(b)) => a == b,
            (AttrValue::Double(a), AttrValue::Double(b)) => a == b,
            (AttrValue::Guid(a), AttrValue::Guid(b)) => a == b,
            (AttrValue::String(a), AttrValue::String(b)) => a == b,
            (AttrValue::Blob(a), AttrValue::Blob(b)) => a == b,
            (AttrValue::Unknown(a), AttrValue::Unknown(b)) => a == b,
            _ => false,
        }
    }

    unsafe fn release_if_unk(&self) {
        if let AttrValue::Unknown(p) = *self {
            if p != 0 {
                com_release(p as *mut c_void);
            }
        }
    }

    unsafe fn clone_retained(&self) -> AttrValue {
        if let AttrValue::Unknown(p) = *self {
            if p != 0 {
                com_add_ref(p as *mut c_void);
            }
        }
        self.clone()
    }
}

/// Write an [`AttrValue`] into a 24-byte Win64 `PROPVARIANT` buffer.
unsafe fn write_propvariant(pv: *mut u8, val: &AttrValue) -> HRESULT {
    std::ptr::write_bytes(pv, 0, 24);
    match val {
        AttrValue::Uint32(v) => {
            std::ptr::write_unaligned(pv as *mut u16, VT_UI4);
            std::ptr::write_unaligned(pv.add(8) as *mut u32, *v);
            S_OK
        }
        AttrValue::Uint64(v) => {
            std::ptr::write_unaligned(pv as *mut u16, VT_UI8);
            std::ptr::write_unaligned(pv.add(8) as *mut u64, *v);
            S_OK
        }
        AttrValue::Double(v) => {
            std::ptr::write_unaligned(pv as *mut u16, VT_R8);
            std::ptr::write_unaligned(pv.add(8) as *mut f64, *v);
            S_OK
        }
        AttrValue::Guid(g) => {
            let mem = CoTaskMemAlloc(16) as *mut Guid;
            if mem.is_null() {
                return E_OUTOFMEMORY;
            }
            std::ptr::write_unaligned(mem, *g);
            std::ptr::write_unaligned(pv as *mut u16, VT_CLSID);
            std::ptr::write_unaligned(pv.add(8) as *mut *mut Guid, mem);
            S_OK
        }
        AttrValue::String(units) => {
            let bytes = (units.len() + 1) * 2;
            let mem = CoTaskMemAlloc(bytes) as *mut u16;
            if mem.is_null() {
                return E_OUTOFMEMORY;
            }
            if !units.is_empty() {
                std::ptr::copy_nonoverlapping(units.as_ptr(), mem, units.len());
            }
            std::ptr::write_unaligned(mem.add(units.len()), 0u16);
            std::ptr::write_unaligned(pv as *mut u16, VT_LPWSTR);
            std::ptr::write_unaligned(pv.add(8) as *mut *mut u16, mem);
            S_OK
        }
        AttrValue::Blob(bytes) => {
            let mem = if bytes.is_empty() {
                std::ptr::null_mut()
            } else {
                let p = CoTaskMemAlloc(bytes.len()) as *mut u8;
                if p.is_null() {
                    return E_OUTOFMEMORY;
                }
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), p, bytes.len());
                p
            };
            std::ptr::write_unaligned(pv as *mut u16, VT_VECTOR_UI1);
            std::ptr::write_unaligned(pv.add(8) as *mut u32, bytes.len() as u32);
            std::ptr::write_unaligned(pv.add(16) as *mut *mut u8, mem);
            S_OK
        }
        AttrValue::Unknown(p) => {
            if *p != 0 {
                com_add_ref(*p as *mut c_void);
            }
            std::ptr::write_unaligned(pv as *mut u16, VT_UNKNOWN);
            std::ptr::write_unaligned(pv.add(8) as *mut usize, *p);
            S_OK
        }
    }
}

/// Parse a Win64 `PROPVARIANT` (24 bytes) into an [`AttrValue`].
/// When `retain_unknown` is true, `AddRef`s any `VT_UNKNOWN` interface pointer.
unsafe fn read_propvariant(pv: *const u8, retain_unknown: bool) -> Result<AttrValue, HRESULT> {
    if pv.is_null() {
        return Err(E_POINTER);
    }
    let vt = std::ptr::read_unaligned(pv as *const u16);
    match vt {
        VT_UI4 => Ok(AttrValue::Uint32(std::ptr::read_unaligned(pv.add(8) as *const u32))),
        VT_UI8 => Ok(AttrValue::Uint64(std::ptr::read_unaligned(pv.add(8) as *const u64))),
        VT_R8 => Ok(AttrValue::Double(std::ptr::read_unaligned(pv.add(8) as *const f64))),
        VT_CLSID => {
            let p = std::ptr::read_unaligned(pv.add(8) as *const *const Guid);
            if p.is_null() {
                return Err(E_INVALIDARG);
            }
            Ok(AttrValue::Guid(std::ptr::read_unaligned(p)))
        }
        VT_LPWSTR => {
            let p = std::ptr::read_unaligned(pv.add(8) as *const *const u16);
            if p.is_null() {
                return Err(E_INVALIDARG);
            }
            Ok(AttrValue::String(read_wide_z(p)))
        }
        VT_VECTOR_UI1 | VT_BLOB => {
            let len = std::ptr::read_unaligned(pv.add(8) as *const u32) as usize;
            let p = std::ptr::read_unaligned(pv.add(16) as *const *const u8);
            if len > 0 && p.is_null() {
                return Err(E_INVALIDARG);
            }
            let mut v = vec![0u8; len];
            if len > 0 {
                std::ptr::copy_nonoverlapping(p, v.as_mut_ptr(), len);
            }
            Ok(AttrValue::Blob(v))
        }
        VT_UNKNOWN => {
            let p = std::ptr::read_unaligned(pv.add(8) as *const usize);
            if retain_unknown && p != 0 {
                com_add_ref(p as *mut c_void);
            }
            Ok(AttrValue::Unknown(p))
        }
        VT_EMPTY => Err(E_INVALIDARG),
        _ => Err(E_INVALIDARG),
    }
}

/// Release any heap/COM payload owned by a temporary `PROPVARIANT`.
unsafe fn clear_propvariant_local(pv: *mut u8) {
    let vt = std::ptr::read_unaligned(pv as *const u16);
    match vt {
        VT_CLSID | VT_LPWSTR => {
            let p = std::ptr::read_unaligned(pv.add(8) as *const *mut c_void);
            if !p.is_null() {
                CoTaskMemFree(p);
            }
        }
        VT_VECTOR_UI1 | VT_BLOB => {
            let p = std::ptr::read_unaligned(pv.add(16) as *const *mut c_void);
            if !p.is_null() {
                CoTaskMemFree(p);
            }
        }
        VT_UNKNOWN => {
            let p = std::ptr::read_unaligned(pv.add(8) as *const *mut c_void);
            if !p.is_null() {
                com_release(p);
            }
        }
        _ => {}
    }
    std::ptr::write_bytes(pv, 0, 24);
}

struct AttrStore {
    lock: ReentrantLock,
    items: Mutex<Vec<(Guid, AttrValue)>>,
}

impl AttrStore {
    fn new(cap: usize) -> Self {
        Self {
            lock: ReentrantLock::new(),
            items: Mutex::new(Vec::with_capacity(cap.min(256))),
        }
    }

    fn set_item(&self, key: Guid, val: AttrValue) {
        self.lock.with_lock(|| {
            let mut items = self.items.lock().unwrap();
            if let Some((_, existing)) = items.iter_mut().find(|(k, _)| *k == key) {
                let old = std::mem::replace(existing, val);
                unsafe { old.release_if_unk() };
            } else {
                items.push((key, val));
            }
        });
    }

    fn delete_item(&self, key: &Guid) {
        self.lock.with_lock(|| {
            let mut items = self.items.lock().unwrap();
            if let Some(idx) = items.iter().position(|(k, _)| k == key) {
                let (_, old) = items.remove(idx);
                unsafe { old.release_if_unk() };
            }
        });
    }

    fn delete_all(&self) {
        self.lock.with_lock(|| {
            let mut items = self.items.lock().unwrap();
            for (_, old) in items.drain(..) {
                unsafe { old.release_if_unk() };
            }
        });
    }

    fn get_clone(&self, key: &Guid) -> Option<AttrValue> {
        self.lock.with_lock(|| {
            let items = self.items.lock().unwrap();
            items
                .iter()
                .find(|(k, _)| k == key)
                .map(|(_, v)| unsafe { v.clone_retained() })
        })
    }

    fn snapshot_keys_and_values(&self) -> Vec<(Guid, AttrValue)> {
        self.lock.with_lock(|| {
            let items = self.items.lock().unwrap();
            items
                .iter()
                .map(|(k, v)| (*k, unsafe { v.clone_retained() }))
                .collect()
        })
    }
}

impl Drop for AttrStore {
    fn drop(&mut self) {
        if let Ok(mut items) = self.items.lock() {
            for (_, v) in items.drain(..) {
                unsafe { v.release_if_unk() };
            }
        }
    }
}

/// Common header for `IMFAttributes`, `IMFMediaType`, and `IMFSample` so all 30
/// attribute vtable slots share one zero-overhead implementation.
#[repr(C)]
struct AttrHeader {
    vtbl: *const usize,
    refs: AtomicU32,
    attrs: AttrStore,
}

#[repr(C)]
struct AttrObj {
    hdr: AttrHeader,
}

#[repr(C)]
struct MediaTypeObj {
    hdr: AttrHeader,
}

#[repr(C)]
struct SampleObj {
    hdr: AttrHeader,
    sample: Mutex<SampleState>,
}

#[derive(Default)]
struct SampleState {
    flags: u32,
    sample_time: Option<i64>,
    sample_duration: Option<i64>,
    buffers: Vec<usize>, // AddRef'd IMFMediaBuffer*
}

impl Drop for SampleState {
    fn drop(&mut self) {
        for p in self.buffers.drain(..) {
            if p != 0 {
                unsafe { com_release(p as *mut c_void) };
            }
        }
    }
}

#[inline]
unsafe fn hdr_ref<'a>(this: usize) -> Option<&'a AttrHeader> {
    if this == 0 {
        None
    } else {
        Some(&*(this as *const AttrHeader))
    }
}

fn is_our_attr_vtable(vt: *const usize) -> bool {
    vt == attr_vtable().as_ptr()
        || vt == media_type_vtable().as_ptr()
        || vt == sample_vtable().as_ptr()
}

unsafe fn qi_attr_common(this: usize, riid: *const Guid, ppv: *mut *mut c_void, extra_iid: Option<Guid>) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid(riid) else {
        return E_POINTER;
    };
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    if iid == IID_IUnknown || iid == IID_IMFAttributes || extra_iid.is_some_and(|e| e == iid) {
        h.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn attr_qi(this: usize, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT {
    qi_attr_common(this, riid, ppv, None)
}

unsafe extern "win64" fn mediatype_qi(this: usize, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT {
    qi_attr_common(this, riid, ppv, Some(IID_IMFMediaType))
}

unsafe extern "win64" fn sample_qi(this: usize, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT {
    qi_attr_common(this, riid, ppv, Some(IID_IMFSample))
}

unsafe extern "win64" fn attr_add_ref(this: usize) -> u32 {
    let Some(h) = hdr_ref(this) else {
        return 0;
    };
    h.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn attr_release_obj<T>(this: usize) -> u32 {
    let Some(h) = hdr_ref(this) else {
        return 0;
    };
    let prev = h.refs.fetch_sub(1, Ordering::AcqRel);
    let next = prev.saturating_sub(1);
    if next == 0 {
        drop(Box::from_raw(this as *mut T));
    }
    next
}

// ---- IMFAttributes methods (slots 3..32) ----

unsafe extern "win64" fn attr_get_item(this: usize, key: *const Guid, pv: *mut u8) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    let hr = if pv.is_null() {
        S_OK
    } else {
        write_propvariant(pv, &v)
    };
    v.release_if_unk();
    hr
}

unsafe extern "win64" fn attr_get_item_type(this: usize, key: *const Guid, out_type: *mut u32) -> HRESULT {
    if out_type.is_null() {
        return E_POINTER;
    }
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    let ty = v.mf_type();
    v.release_if_unk();
    std::ptr::write_unaligned(out_type, ty);
    S_OK
}

unsafe extern "win64" fn attr_compare_item(
    this: usize,
    key: *const Guid,
    value: *const u8,
    result: *mut BOOL,
) -> HRESULT {
    if result.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(result, FALSE);
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Ok(rhs) = read_propvariant(value, false) else {
        return E_INVALIDARG;
    };
    let Some(lhs) = h.attrs.get_clone(&k) else {
        return S_OK;
    };
    let eq = lhs.equals(&rhs);
    lhs.release_if_unk();
    std::ptr::write_unaligned(result, if eq { TRUE } else { FALSE });
    S_OK
}

/// Read all items from any `IMFAttributes*` (fast-path for ours, COM vtable fallback for foreign).
unsafe fn snapshot_any_attributes(ptr: *mut c_void) -> Result<Vec<(Guid, AttrValue)>, HRESULT> {
    if ptr.is_null() {
        return Err(E_POINTER);
    }
    let vt = std::ptr::read_unaligned(ptr as *const *const usize);
    if is_our_attr_vtable(vt) {
        let h = &*(ptr as *const AttrHeader);
        return Ok(h.attrs.snapshot_keys_and_values());
    }
    let get_count: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(30)));
    let get_by_idx: unsafe extern "win64" fn(*mut c_void, u32, *mut Guid, *mut u8) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(31)));
    let mut count = 0u32;
    let hr = get_count(ptr, &mut count);
    if hr < 0 {
        return Err(hr);
    }
    let mut out: Vec<(Guid, AttrValue)> = Vec::with_capacity(count as usize);
    for i in 0..count {
        let mut g = Guid::default();
        let mut pv = [0u8; 24];
        let hr = get_by_idx(ptr, i, &mut g, pv.as_mut_ptr());
        if hr < 0 {
            for (_, v) in out {
                v.release_if_unk();
            }
            return Err(hr);
        }
        let parsed = read_propvariant(pv.as_ptr(), true);
        clear_propvariant_local(pv.as_mut_ptr());
        match parsed {
            Ok(v) => out.push((g, v)),
            Err(e) => {
                for (_, v) in out {
                    v.release_if_unk();
                }
                return Err(e);
            }
        }
    }
    Ok(out)
}

fn compare_attr_lists(ours: &[(Guid, AttrValue)], theirs: &[(Guid, AttrValue)], match_type: u32) -> Result<bool, HRESULT> {
    let subset_match = |a: &[(Guid, AttrValue)], b: &[(Guid, AttrValue)]| -> bool {
        a.iter()
            .all(|(k, v)| b.iter().find(|(bk, _)| bk == k).is_some_and(|(_, bv)| v.equals(bv)))
    };
    match match_type {
        MF_ATTRIBUTES_MATCH_OUR_ITEMS => Ok(subset_match(ours, theirs)),
        MF_ATTRIBUTES_MATCH_THEIR_ITEMS => Ok(subset_match(theirs, ours)),
        MF_ATTRIBUTES_MATCH_ALL_ITEMS => Ok(ours.len() == theirs.len() && subset_match(ours, theirs)),
        MF_ATTRIBUTES_MATCH_INTERSECTION => Ok(ours.iter().all(|(k, v)| {
            theirs
                .iter()
                .find(|(bk, _)| bk == k)
                .is_none_or(|(_, bv)| v.equals(bv))
        })),
        MF_ATTRIBUTES_MATCH_SMALLER => {
            if ours.len() <= theirs.len() {
                Ok(subset_match(ours, theirs))
            } else {
                Ok(subset_match(theirs, ours))
            }
        }
        _ => Err(E_INVALIDARG),
    }
}

unsafe extern "win64" fn attr_compare(
    this: usize,
    theirs: *mut c_void,
    match_type: u32,
    result: *mut BOOL,
) -> HRESULT {
    if result.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(result, FALSE);
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    if theirs.is_null() {
        return E_INVALIDARG;
    }
    let ours = h.attrs.snapshot_keys_and_values();
    let theirs_snap = match snapshot_any_attributes(theirs) {
        Ok(v) => v,
        Err(hr) => {
            for (_, v) in ours {
                v.release_if_unk();
            }
            return hr;
        }
    };
    let cmp = compare_attr_lists(&ours, &theirs_snap, match_type);
    for (_, v) in ours {
        v.release_if_unk();
    }
    for (_, v) in theirs_snap {
        v.release_if_unk();
    }
    match cmp {
        Ok(b) => {
            std::ptr::write_unaligned(result, if b { TRUE } else { FALSE });
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn attr_get_uint32(this: usize, key: *const Guid, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    match v {
        AttrValue::Uint32(val) => {
            std::ptr::write_unaligned(out, val);
            S_OK
        }
        other => {
            other.release_if_unk();
            MF_E_ATTRIBUTENOTFOUND
        }
    }
}

unsafe extern "win64" fn attr_get_uint64(this: usize, key: *const Guid, out: *mut u64) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    match v {
        AttrValue::Uint64(val) => {
            std::ptr::write_unaligned(out, val);
            S_OK
        }
        other => {
            other.release_if_unk();
            MF_E_ATTRIBUTENOTFOUND
        }
    }
}

unsafe extern "win64" fn attr_get_double(this: usize, key: *const Guid, out: *mut f64) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    match v {
        AttrValue::Double(val) => {
            std::ptr::write_unaligned(out, val);
            S_OK
        }
        other => {
            other.release_if_unk();
            MF_E_ATTRIBUTENOTFOUND
        }
    }
}

unsafe extern "win64" fn attr_get_guid(this: usize, key: *const Guid, out: *mut Guid) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    match v {
        AttrValue::Guid(val) => {
            std::ptr::write_unaligned(out, val);
            S_OK
        }
        other => {
            other.release_if_unk();
            MF_E_ATTRIBUTENOTFOUND
        }
    }
}

unsafe extern "win64" fn attr_get_string_length(this: usize, key: *const Guid, out_len: *mut u32) -> HRESULT {
    if out_len.is_null() {
        return E_POINTER;
    }
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    match v {
        AttrValue::String(units) => {
            std::ptr::write_unaligned(out_len, units.len() as u32);
            S_OK
        }
        other => {
            other.release_if_unk();
            MF_E_ATTRIBUTENOTFOUND
        }
    }
}

unsafe extern "win64" fn attr_get_string(
    this: usize,
    key: *const Guid,
    buf: *mut u16,
    buf_cch: u32,
    out_len: *mut u32,
) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    let AttrValue::String(units) = v else {
        v.release_if_unk();
        return MF_E_ATTRIBUTENOTFOUND;
    };
    if !out_len.is_null() {
        std::ptr::write_unaligned(out_len, units.len() as u32);
    }
    if buf.is_null() || (buf_cch as usize) < units.len() + 1 {
        return STRSAFE_E_INSUFFICIENT_BUFFER;
    }
    if !units.is_empty() {
        std::ptr::copy_nonoverlapping(units.as_ptr(), buf, units.len());
    }
    std::ptr::write_unaligned(buf.add(units.len()), 0u16);
    S_OK
}

unsafe extern "win64" fn attr_get_allocated_string(
    this: usize,
    key: *const Guid,
    out_ptr: *mut *mut u16,
    out_len: *mut u32,
) -> HRESULT {
    if out_ptr.is_null() || out_len.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out_ptr, std::ptr::null_mut());
    std::ptr::write_unaligned(out_len, 0);
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    let AttrValue::String(units) = v else {
        v.release_if_unk();
        return MF_E_ATTRIBUTENOTFOUND;
    };
    let mem = CoTaskMemAlloc((units.len() + 1) * 2) as *mut u16;
    if mem.is_null() {
        return E_OUTOFMEMORY;
    }
    if !units.is_empty() {
        std::ptr::copy_nonoverlapping(units.as_ptr(), mem, units.len());
    }
    std::ptr::write_unaligned(mem.add(units.len()), 0u16);
    std::ptr::write_unaligned(out_ptr, mem);
    std::ptr::write_unaligned(out_len, units.len() as u32);
    S_OK
}

unsafe extern "win64" fn attr_get_blob_size(this: usize, key: *const Guid, out_size: *mut u32) -> HRESULT {
    if out_size.is_null() {
        return E_POINTER;
    }
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    match v {
        AttrValue::Blob(b) => {
            std::ptr::write_unaligned(out_size, b.len() as u32);
            S_OK
        }
        other => {
            other.release_if_unk();
            MF_E_ATTRIBUTENOTFOUND
        }
    }
}

unsafe extern "win64" fn attr_get_blob(
    this: usize,
    key: *const Guid,
    buf: *mut u8,
    buf_size: u32,
    out_size: *mut u32,
) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    let AttrValue::Blob(bytes) = v else {
        v.release_if_unk();
        return MF_E_ATTRIBUTENOTFOUND;
    };
    if !out_size.is_null() {
        std::ptr::write_unaligned(out_size, bytes.len() as u32);
    }
    if (buf_size as usize) < bytes.len() {
        return MF_E_BUFFERTOOSMALL;
    }
    if !bytes.is_empty() {
        if buf.is_null() {
            return E_POINTER;
        }
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, bytes.len());
    }
    S_OK
}

unsafe extern "win64" fn attr_get_allocated_blob(
    this: usize,
    key: *const Guid,
    out_buf: *mut *mut u8,
    out_size: *mut u32,
) -> HRESULT {
    if out_buf.is_null() || out_size.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out_buf, std::ptr::null_mut());
    std::ptr::write_unaligned(out_size, 0);
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    let AttrValue::Blob(bytes) = v else {
        v.release_if_unk();
        return MF_E_ATTRIBUTENOTFOUND;
    };
    let mem = CoTaskMemAlloc(bytes.len().max(1)) as *mut u8;
    if mem.is_null() {
        return E_OUTOFMEMORY;
    }
    if !bytes.is_empty() {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), mem, bytes.len());
    }
    std::ptr::write_unaligned(out_buf, mem);
    std::ptr::write_unaligned(out_size, bytes.len() as u32);
    S_OK
}

unsafe extern "win64" fn attr_get_unknown(
    this: usize,
    key: *const Guid,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let (Some(h), Some(k), Some(iid)) = (hdr_ref(this), read_guid(key), read_guid(riid)) else {
        return E_INVALIDARG;
    };
    let Some(v) = h.attrs.get_clone(&k) else {
        return MF_E_ATTRIBUTENOTFOUND;
    };
    let AttrValue::Unknown(p) = v else {
        v.release_if_unk();
        return MF_E_ATTRIBUTENOTFOUND;
    };
    if p == 0 {
        return E_NOINTERFACE;
    }
    let hr = com_query(p as *mut c_void, &iid, ppv);
    com_release(p as *mut c_void);
    hr
}

unsafe extern "win64" fn attr_set_item(this: usize, key: *const Guid, pv: *const u8) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    match read_propvariant(pv, true) {
        Ok(val) => {
            h.attrs.set_item(k, val);
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn attr_delete_item(this: usize, key: *const Guid) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    h.attrs.delete_item(&k);
    S_OK
}

unsafe extern "win64" fn attr_delete_all_items(this: usize) -> HRESULT {
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    h.attrs.delete_all();
    S_OK
}

unsafe extern "win64" fn attr_set_uint32(this: usize, key: *const Guid, value: u32) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    h.attrs.set_item(k, AttrValue::Uint32(value));
    S_OK
}

unsafe extern "win64" fn attr_set_uint64(this: usize, key: *const Guid, value: u64) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    h.attrs.set_item(k, AttrValue::Uint64(value));
    S_OK
}

unsafe extern "win64" fn attr_set_double(this: usize, key: *const Guid, value: f64) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    h.attrs.set_item(k, AttrValue::Double(value));
    S_OK
}

// Slot 23 takes `f64` in Win64 slot 3 (`xmm2`) after two integer/pointer slots.
crate::win64_bridge!(fn attr_set_double(usize, *const Guid, f64) -> HRESULT);

unsafe extern "win64" fn attr_set_guid(this: usize, key: *const Guid, value: *const Guid) -> HRESULT {
    let (Some(h), Some(k), Some(v)) = (hdr_ref(this), read_guid(key), read_guid(value)) else {
        return E_INVALIDARG;
    };
    h.attrs.set_item(k, AttrValue::Guid(v));
    S_OK
}

unsafe extern "win64" fn attr_set_string(this: usize, key: *const Guid, value: *const u16) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    if value.is_null() {
        return E_INVALIDARG;
    }
    h.attrs.set_item(k, AttrValue::String(read_wide_z(value)));
    S_OK
}

unsafe extern "win64" fn attr_set_blob(this: usize, key: *const Guid, buf: *const u8, size: u32) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    if size > 0 && buf.is_null() {
        return E_POINTER;
    }
    let mut data = vec![0u8; size as usize];
    if size > 0 {
        std::ptr::copy_nonoverlapping(buf, data.as_mut_ptr(), size as usize);
    }
    h.attrs.set_item(k, AttrValue::Blob(data));
    S_OK
}

unsafe extern "win64" fn attr_set_unknown(this: usize, key: *const Guid, unk: *mut c_void) -> HRESULT {
    let (Some(h), Some(k)) = (hdr_ref(this), read_guid(key)) else {
        return E_INVALIDARG;
    };
    if !unk.is_null() {
        com_add_ref(unk);
    }
    h.attrs.set_item(k, AttrValue::Unknown(unk as usize));
    S_OK
}

unsafe extern "win64" fn attr_lock_store(this: usize) -> HRESULT {
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    h.attrs.lock.lock_raw();
    S_OK
}

unsafe extern "win64" fn attr_unlock_store(this: usize) -> HRESULT {
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    if h.attrs.lock.unlock_raw() {
        S_OK
    } else {
        E_FAIL
    }
}

unsafe extern "win64" fn attr_get_count(this: usize, out_count: *mut u32) -> HRESULT {
    if out_count.is_null() {
        return E_POINTER;
    }
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    let n = h.attrs.lock.with_lock(|| h.attrs.items.lock().unwrap().len() as u32);
    std::ptr::write_unaligned(out_count, n);
    S_OK
}

unsafe extern "win64" fn attr_get_item_by_index(
    this: usize,
    index: u32,
    out_key: *mut Guid,
    out_pv: *mut u8,
) -> HRESULT {
    if out_key.is_null() {
        return E_POINTER;
    }
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    let entry = h.attrs.lock.with_lock(|| {
        let items = h.attrs.items.lock().unwrap();
        items
            .get(index as usize)
            .map(|(k, v)| (*k, v.clone_retained()))
    });
    let Some((k, v)) = entry else {
        return E_INVALIDARG;
    };
    std::ptr::write_unaligned(out_key, k);
    let hr = if out_pv.is_null() {
        S_OK
    } else {
        write_propvariant(out_pv, &v)
    };
    v.release_if_unk();
    hr
}

unsafe extern "win64" fn attr_copy_all_items(this: usize, dest: *mut c_void) -> HRESULT {
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    if dest.is_null() {
        return E_POINTER;
    }
    if dest as usize == this {
        return S_OK;
    }
    let snap = h.attrs.snapshot_keys_and_values();
    let dst_vt = std::ptr::read_unaligned(dest as *const *const usize);
    if is_our_attr_vtable(dst_vt) {
        let dh = &*(dest as *const AttrHeader);
        dh.attrs.delete_all();
        for (k, v) in snap {
            dh.attrs.set_item(k, v);
        }
        return S_OK;
    }
    let del_all: unsafe extern "win64" fn(*mut c_void) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(dst_vt.add(20)));
    let set_item: unsafe extern "win64" fn(*mut c_void, *const Guid, *const u8) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(dst_vt.add(18)));
    let hr = del_all(dest);
    if hr < 0 {
        for (_, v) in snap {
            v.release_if_unk();
        }
        return hr;
    }
    let mut final_hr = S_OK;
    for (k, v) in snap {
        let mut pv = [0u8; 24];
        let whr = write_propvariant(pv.as_mut_ptr(), &v);
        v.release_if_unk();
        if whr < 0 {
            final_hr = whr;
            continue;
        }
        let shr = set_item(dest, &k, pv.as_ptr());
        clear_propvariant_local(pv.as_mut_ptr());
        if shr < 0 && final_hr == S_OK {
            final_hr = shr;
        }
    }
    final_hr
}

fn fill_attr_slots<T>(qi: usize) -> [usize; 33] {
    [
        qi,
        attr_add_ref as *const () as usize,
        attr_release_obj::<T> as *const () as usize,
        attr_get_item as *const () as usize,
        attr_get_item_type as *const () as usize,
        attr_compare_item as *const () as usize,
        attr_compare as *const () as usize,
        attr_get_uint32 as *const () as usize,
        attr_get_uint64 as *const () as usize,
        attr_get_double as *const () as usize,
        attr_get_guid as *const () as usize,
        attr_get_string_length as *const () as usize,
        attr_get_string as *const () as usize,
        attr_get_allocated_string as *const () as usize,
        attr_get_blob_size as *const () as usize,
        attr_get_blob as *const () as usize,
        attr_get_allocated_blob as *const () as usize,
        attr_get_unknown as *const () as usize,
        attr_set_item as *const () as usize,
        attr_delete_item as *const () as usize,
        attr_delete_all_items as *const () as usize,
        attr_set_uint32 as *const () as usize,
        attr_set_uint64 as *const () as usize,
        crate::bridge_entry!(attr_set_double) as usize,
        attr_set_guid as *const () as usize,
        attr_set_string as *const () as usize,
        attr_set_blob as *const () as usize,
        attr_set_unknown as *const () as usize,
        attr_lock_store as *const () as usize,
        attr_unlock_store as *const () as usize,
        attr_get_count as *const () as usize,
        attr_get_item_by_index as *const () as usize,
        attr_copy_all_items as *const () as usize,
    ]
}

fn attr_vtable() -> &'static [usize; 33] {
    static VTBL: OnceLock<[usize; 33]> = OnceLock::new();
    VTBL.get_or_init(|| register_vtable(fill_attr_slots::<AttrObj>(attr_qi as *const () as usize)))
}

pub unsafe extern "win64" fn MFCreateAttributes(
    pp_mf_attributes: *mut *mut c_void,
    c_initial_size: u32,
) -> HRESULT {
    if pp_mf_attributes.is_null() {
        return E_POINTER;
    }
    let obj = Box::new(AttrObj {
        hdr: AttrHeader {
            vtbl: attr_vtable().as_ptr(),
            refs: AtomicU32::new(1),
            attrs: AttrStore::new(c_initial_size as usize),
        },
    });
    std::ptr::write_unaligned(pp_mf_attributes, Box::into_raw(obj) as *mut c_void);
    S_OK
}

// ============================================================================
// IMFMediaType (inherits IMFAttributes, slots 33..37)
// ============================================================================

fn is_uncompressed_subtype(g: &Guid) -> bool {
    *g == MF_AUDIO_FORMAT_PCM
        || *g == MF_AUDIO_FORMAT_FLOAT
        || *g == MF_VIDEO_FORMAT_RGB32
        || *g == MF_VIDEO_FORMAT_ARGB32
        || *g == MF_VIDEO_FORMAT_RGB24
        || *g == MF_VIDEO_FORMAT_NV12
        || *g == MF_VIDEO_FORMAT_YUY2
        || *g == MF_VIDEO_FORMAT_YV12
        || *g == MF_VIDEO_FORMAT_IYUV
        || *g == MF_VIDEO_FORMAT_I420
        || *g == MF_VIDEO_FORMAT_UYVY
        || *g == MF_VIDEO_FORMAT_P010
        || *g == MF_VIDEO_FORMAT_AYUV
}

unsafe extern "win64" fn mediatype_get_major_type(this: usize, out: *mut Guid) -> HRESULT {
    attr_get_guid(this, &MF_MT_MAJOR_TYPE, out)
}

unsafe extern "win64" fn mediatype_is_compressed_format(this: usize, out: *mut BOOL) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    // Explicit MF_MT_ALL_SAMPLES_INDEPENDENT == 0 or MF_MT_FIXED_SIZE_SAMPLES == 0 => compressed.
    if let Some(AttrValue::Uint32(v)) = h.attrs.get_clone(&MF_MT_ALL_SAMPLES_INDEPENDENT) {
        if v == 0 {
            std::ptr::write_unaligned(out, TRUE);
            return S_OK;
        }
    }
    if let Some(AttrValue::Uint32(v)) = h.attrs.get_clone(&MF_MT_FIXED_SIZE_SAMPLES) {
        if v == 0 {
            std::ptr::write_unaligned(out, TRUE);
            return S_OK;
        }
    }
    if let Some(AttrValue::Guid(sub)) = h.attrs.get_clone(&MF_MT_SUBTYPE) {
        let compressed = !is_uncompressed_subtype(&sub);
        std::ptr::write_unaligned(out, if compressed { TRUE } else { FALSE });
        return S_OK;
    }
    std::ptr::write_unaligned(out, FALSE);
    S_OK
}

unsafe extern "win64" fn mediatype_is_equal(
    this: usize,
    other: *mut c_void,
    out_flags: *mut u32,
) -> HRESULT {
    if other.is_null() || out_flags.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(out_flags, 0);
    let Some(h) = hdr_ref(this) else {
        return E_INVALIDARG;
    };
    let ours = h.attrs.snapshot_keys_and_values();
    let theirs = match snapshot_any_attributes(other) {
        Ok(v) => v,
        Err(hr) => {
            for (_, v) in ours {
                v.release_if_unk();
            }
            return hr;
        }
    };
    fn find_attr<'a>(list: &'a [(Guid, AttrValue)], key: &Guid) -> Option<&'a AttrValue> {
        list.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }
    let mut flags = 0u32;
    let major_eq = match (find_attr(&ours, &MF_MT_MAJOR_TYPE), find_attr(&theirs, &MF_MT_MAJOR_TYPE)) {
        (Some(a), Some(b)) => a.equals(b),
        _ => false,
    };
    if major_eq {
        flags |= MF_MEDIATYPE_EQUAL_MAJOR_TYPES;
    }
    let sub_eq = match (find_attr(&ours, &MF_MT_SUBTYPE), find_attr(&theirs, &MF_MT_SUBTYPE)) {
        (Some(a), Some(b)) => a.equals(b),
        _ => false,
    };
    if major_eq && sub_eq {
        flags |= MF_MEDIATYPE_EQUAL_FORMAT_TYPES;
    }
    let user_eq = match (find_attr(&ours, &MF_MT_USER_DATA), find_attr(&theirs, &MF_MT_USER_DATA)) {
        (None, None) => true,
        (Some(a), Some(b)) => a.equals(b),
        _ => false,
    };
    if user_eq {
        flags |= MF_MEDIATYPE_EQUAL_FORMAT_USER_DATA;
    }
    let all_non_user_match = {
        let ours_f: Vec<_> = ours.iter().filter(|(k, _)| *k != MF_MT_USER_DATA).collect();
        let theirs_f: Vec<_> = theirs.iter().filter(|(k, _)| *k != MF_MT_USER_DATA).collect();
        ours_f.len() == theirs_f.len()
            && ours_f
                .iter()
                .all(|(k, v)| theirs_f.iter().find(|(bk, _)| bk == k).is_some_and(|(_, bv)| v.equals(bv)))
    };
    if major_eq && sub_eq && all_non_user_match {
        flags |= MF_MEDIATYPE_EQUAL_FORMAT_DATA;
    }
    for (_, v) in ours {
        v.release_if_unk();
    }
    for (_, v) in theirs {
        v.release_if_unk();
    }
    std::ptr::write_unaligned(out_flags, flags);
    const ALL_EQUAL: u32 = MF_MEDIATYPE_EQUAL_MAJOR_TYPES
        | MF_MEDIATYPE_EQUAL_FORMAT_TYPES
        | MF_MEDIATYPE_EQUAL_FORMAT_DATA
        | MF_MEDIATYPE_EQUAL_FORMAT_USER_DATA;
    if flags == ALL_EQUAL {
        S_OK
    } else {
        S_FALSE
    }
}

unsafe extern "win64" fn mediatype_get_representation(
    _this: usize,
    _guid_repr: *const Guid,
    out_repr: *mut *mut c_void,
) -> HRESULT {
    write_null_out(out_repr);
    E_NOTIMPL
}

unsafe extern "win64" fn mediatype_free_representation(
    _this: usize,
    _guid_repr: *const Guid,
    _repr: *const c_void,
) -> HRESULT {
    S_OK
}

fn media_type_vtable() -> &'static [usize; 38] {
    static VTBL: OnceLock<[usize; 38]> = OnceLock::new();
    VTBL.get_or_init(|| {
        let base = fill_attr_slots::<MediaTypeObj>(mediatype_qi as *const () as usize);
        let mut full = [0usize; 38];
        full[..33].copy_from_slice(&base);
        full[33] = mediatype_get_major_type as *const () as usize;
        full[34] = mediatype_is_compressed_format as *const () as usize;
        full[35] = mediatype_is_equal as *const () as usize;
        full[36] = mediatype_get_representation as *const () as usize;
        full[37] = mediatype_free_representation as *const () as usize;
        register_vtable(full)
    })
}

pub unsafe extern "win64" fn MFCreateMediaType(pp_mf_type: *mut *mut c_void) -> HRESULT {
    if pp_mf_type.is_null() {
        return E_POINTER;
    }
    let obj = Box::new(MediaTypeObj {
        hdr: AttrHeader {
            vtbl: media_type_vtable().as_ptr(),
            refs: AtomicU32::new(1),
            attrs: AttrStore::new(16),
        },
    });
    std::ptr::write_unaligned(pp_mf_type, Box::into_raw(obj) as *mut c_void);
    S_OK
}

// ============================================================================
// IMFMediaBuffer + IMF2DBuffer / IMF2DBuffer2 (Memory Buffers)
// ============================================================================

struct MemBufState {
    /// Backing allocation; never reallocated after creation so `Lock` pointers stay stable.
    storage: Vec<u8>,
    data_offset: usize,
    max_len: u32,
    cur_len: u32,
    row_bytes: u32,
    height: u32,
    stride: u32, // positive magnitude per row
    bottom_up: bool,
    lock_count: u32,
}

impl MemBufState {
    fn new_1d(max_len: u32, alignment: usize) -> Self {
        let align = alignment.max(16).next_power_of_two();
        let alloc_len = (max_len as usize).saturating_add(align);
        let storage = vec![0u8; alloc_len];
        let base = storage.as_ptr() as usize;
        let aligned = (base + align - 1) & !(align - 1);
        let data_offset = aligned - base;
        Self {
            storage,
            data_offset,
            max_len,
            cur_len: 0,
            row_bytes: max_len,
            height: 1,
            stride: max_len,
            bottom_up: false,
            lock_count: 0,
        }
    }

    fn new_2d(row_bytes: u32, height: u32, stride: u32, bottom_up: bool) -> Result<Self, HRESULT> {
        let max_len = stride.checked_mul(height).ok_or(E_INVALIDARG)?;
        let mut s = Self::new_1d(max_len, 16);
        s.row_bytes = row_bytes;
        s.height = height.max(1);
        s.stride = stride;
        s.bottom_up = bottom_up;
        Ok(s)
    }

    fn data_ptr(&mut self) -> *mut u8 {
        unsafe { self.storage.as_mut_ptr().add(self.data_offset) }
    }

    fn scanline0_and_pitch(&mut self) -> (*mut u8, i32) {
        let base = self.data_ptr();
        if self.bottom_up && self.height > 1 {
            let off = (self.height as usize - 1) * (self.stride as usize);
            (unsafe { base.add(off) }, -(self.stride as i32))
        } else {
            (base, self.stride as i32)
        }
    }

    fn contiguous_len(&self) -> u32 {
        self.row_bytes.saturating_mul(self.height)
    }

    fn is_contiguous(&self) -> bool {
        !self.bottom_up && self.stride == self.row_bytes
    }
}

#[repr(C)]
struct MemBufObj {
    vtbl_media: *const usize, // offset 0: IMFMediaBuffer (canonical IUnknown)
    vtbl_2d: *const usize,    // offset 8: IMF2DBuffer2
    refs: AtomicU32,
    state: Mutex<MemBufState>,
}

const MEMBUF_2D_OFFSET: usize = std::mem::offset_of!(MemBufObj, vtbl_2d);

#[inline]
unsafe fn membuf_from_primary<'a>(this: usize) -> Option<&'a MemBufObj> {
    if this == 0 {
        None
    } else {
        Some(&*(this as *const MemBufObj))
    }
}

#[inline]
unsafe fn membuf_from_2d<'a>(this: usize) -> Option<&'a MemBufObj> {
    if this < MEMBUF_2D_OFFSET {
        None
    } else {
        Some(&*((this - MEMBUF_2D_OFFSET) as *const MemBufObj))
    }
}

unsafe fn membuf_qi_inner(obj: &MemBufObj, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid(riid) else {
        return E_POINTER;
    };
    let base = obj as *const MemBufObj as usize;
    if iid == IID_IUnknown || iid == IID_IMFMediaBuffer {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, base as *mut c_void);
        S_OK
    } else if iid == IID_IMF2DBuffer || iid == IID_IMF2DBuffer2 {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, (base + MEMBUF_2D_OFFSET) as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe fn membuf_release_inner(obj: *mut MemBufObj) -> u32 {
    let prev = (*obj).refs.fetch_sub(1, Ordering::AcqRel);
    let next = prev.saturating_sub(1);
    if next == 0 {
        drop(Box::from_raw(obj));
    }
    next
}

// Primary IMFMediaBuffer vtable methods (slots 0..7)
unsafe extern "win64" fn membuf_qi(this: usize, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT {
    let Some(obj) = membuf_from_primary(this) else {
        write_null_out(ppv);
        return E_INVALIDARG;
    };
    membuf_qi_inner(obj, riid, ppv)
}

unsafe extern "win64" fn membuf_add_ref(this: usize) -> u32 {
    let Some(obj) = membuf_from_primary(this) else {
        return 0;
    };
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn membuf_release(this: usize) -> u32 {
    if this == 0 {
        return 0;
    }
    membuf_release_inner(this as *mut MemBufObj)
}

unsafe extern "win64" fn membuf_lock(
    this: usize,
    ppb_buffer: *mut *mut u8,
    pcb_max_length: *mut u32,
    pcb_current_length: *mut u32,
) -> HRESULT {
    if ppb_buffer.is_null() {
        return E_INVALIDARG;
    }
    let Some(obj) = membuf_from_primary(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    let ptr = st.data_ptr();
    st.lock_count += 1;
    std::ptr::write_unaligned(ppb_buffer, ptr);
    if !pcb_max_length.is_null() {
        std::ptr::write_unaligned(pcb_max_length, st.max_len);
    }
    if !pcb_current_length.is_null() {
        std::ptr::write_unaligned(pcb_current_length, st.cur_len);
    }
    S_OK
}

unsafe extern "win64" fn membuf_unlock(this: usize) -> HRESULT {
    let Some(obj) = membuf_from_primary(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    st.lock_count = st.lock_count.saturating_sub(1);
    S_OK
}

unsafe extern "win64" fn membuf_get_current_length(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    let Some(obj) = membuf_from_primary(this) else {
        return E_INVALIDARG;
    };
    std::ptr::write_unaligned(out, obj.state.lock().unwrap().cur_len);
    S_OK
}

unsafe extern "win64" fn membuf_set_current_length(this: usize, len: u32) -> HRESULT {
    let Some(obj) = membuf_from_primary(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    if len > st.max_len {
        return E_INVALIDARG;
    }
    st.cur_len = len;
    S_OK
}

unsafe extern "win64" fn membuf_get_max_length(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    let Some(obj) = membuf_from_primary(this) else {
        return E_INVALIDARG;
    };
    std::ptr::write_unaligned(out, obj.state.lock().unwrap().max_len);
    S_OK
}

// Secondary IMF2DBuffer / IMF2DBuffer2 vtable methods (slots 0..11)
unsafe extern "win64" fn membuf2d_qi(this: usize, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT {
    let Some(obj) = membuf_from_2d(this) else {
        write_null_out(ppv);
        return E_INVALIDARG;
    };
    membuf_qi_inner(obj, riid, ppv)
}

unsafe extern "win64" fn membuf2d_add_ref(this: usize) -> u32 {
    let Some(obj) = membuf_from_2d(this) else {
        return 0;
    };
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn membuf2d_release(this: usize) -> u32 {
    if this < MEMBUF_2D_OFFSET {
        return 0;
    }
    membuf_release_inner((this - MEMBUF_2D_OFFSET) as *mut MemBufObj)
}

unsafe extern "win64" fn membuf2d_lock2d(
    this: usize,
    ppb_scanline0: *mut *mut u8,
    pl_pitch: *mut i32,
) -> HRESULT {
    if ppb_scanline0.is_null() || pl_pitch.is_null() {
        return E_POINTER;
    }
    let Some(obj) = membuf_from_2d(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    let (scan0, pitch) = st.scanline0_and_pitch();
    st.lock_count += 1;
    std::ptr::write_unaligned(ppb_scanline0, scan0);
    std::ptr::write_unaligned(pl_pitch, pitch);
    S_OK
}

unsafe extern "win64" fn membuf2d_unlock2d(this: usize) -> HRESULT {
    let Some(obj) = membuf_from_2d(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    st.lock_count = st.lock_count.saturating_sub(1);
    S_OK
}

unsafe extern "win64" fn membuf2d_get_scanline0_and_pitch(
    this: usize,
    ppb_scanline0: *mut *mut u8,
    pl_pitch: *mut i32,
) -> HRESULT {
    if ppb_scanline0.is_null() || pl_pitch.is_null() {
        return E_POINTER;
    }
    let Some(obj) = membuf_from_2d(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    if st.lock_count == 0 {
        return MF_E_INVALIDREQUEST;
    }
    let (scan0, pitch) = st.scanline0_and_pitch();
    std::ptr::write_unaligned(ppb_scanline0, scan0);
    std::ptr::write_unaligned(pl_pitch, pitch);
    S_OK
}

unsafe extern "win64" fn membuf2d_is_contiguous_format(this: usize, out: *mut BOOL) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(obj) = membuf_from_2d(this) else {
        return E_INVALIDARG;
    };
    let st = obj.state.lock().unwrap();
    std::ptr::write_unaligned(out, if st.is_contiguous() { TRUE } else { FALSE });
    S_OK
}

unsafe extern "win64" fn membuf2d_get_contiguous_length(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(obj) = membuf_from_2d(this) else {
        return E_INVALIDARG;
    };
    let st = obj.state.lock().unwrap();
    std::ptr::write_unaligned(out, st.contiguous_len());
    S_OK
}

unsafe extern "win64" fn membuf2d_contiguous_copy_to(
    this: usize,
    dest: *mut u8,
    dest_len: u32,
) -> HRESULT {
    if dest.is_null() {
        return E_POINTER;
    }
    let Some(obj) = membuf_from_2d(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    let needed = st.contiguous_len();
    if dest_len < needed {
        return E_INVALIDARG;
    }
    let (scan0, pitch) = st.scanline0_and_pitch();
    let row_bytes = st.row_bytes;
    let height = st.height;
    copy_image_rows(dest, row_bytes as i32, scan0, pitch, row_bytes, height)
}

unsafe extern "win64" fn membuf2d_contiguous_copy_from(
    this: usize,
    src: *const u8,
    src_len: u32,
) -> HRESULT {
    if src.is_null() {
        return E_POINTER;
    }
    let Some(obj) = membuf_from_2d(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    let needed = st.contiguous_len();
    if src_len < needed {
        return E_INVALIDARG;
    }
    let (scan0, pitch) = st.scanline0_and_pitch();
    let row_bytes = st.row_bytes;
    let height = st.height;
    let hr = copy_image_rows(scan0, pitch, src, row_bytes as i32, row_bytes, height);
    if hr >= 0 {
        st.cur_len = st.max_len;
    }
    hr
}

unsafe extern "win64" fn membuf2d_lock2d_size(
    this: usize,
    _lock_flags: u32,
    ppb_scanline0: *mut *mut u8,
    pl_pitch: *mut i32,
    ppb_buffer_start: *mut *mut u8,
    pcb_buffer_length: *mut u32,
) -> HRESULT {
    if ppb_scanline0.is_null() || pl_pitch.is_null() || ppb_buffer_start.is_null() || pcb_buffer_length.is_null() {
        return E_POINTER;
    }
    let Some(obj) = membuf_from_2d(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    let base = st.data_ptr();
    let (scan0, pitch) = st.scanline0_and_pitch();
    let max_len = st.max_len;
    st.lock_count += 1;
    std::ptr::write_unaligned(ppb_scanline0, scan0);
    std::ptr::write_unaligned(pl_pitch, pitch);
    std::ptr::write_unaligned(ppb_buffer_start, base);
    std::ptr::write_unaligned(pcb_buffer_length, max_len);
    S_OK
}

unsafe extern "win64" fn membuf2d_copy2d_to(this: usize, dest_2d: *mut c_void) -> HRESULT {
    if dest_2d.is_null() {
        return E_POINTER;
    }
    let Some(obj) = membuf_from_2d(this) else {
        return E_INVALIDARG;
    };
    let (src_scan0, src_pitch, row_bytes, height) = {
        let mut st = obj.state.lock().unwrap();
        let (s0, p) = st.scanline0_and_pitch();
        (s0, p, st.row_bytes, st.height)
    };
    let dvt = std::ptr::read_unaligned(dest_2d as *const *const usize);
    let lock2d: unsafe extern "win64" fn(*mut c_void, *mut *mut u8, *mut i32) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(dvt.add(3)));
    let unlock2d: unsafe extern "win64" fn(*mut c_void) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(dvt.add(4)));
    let mut dst_scan0 = std::ptr::null_mut();
    let mut dst_pitch = 0i32;
    let hr = lock2d(dest_2d, &mut dst_scan0, &mut dst_pitch);
    if hr < 0 {
        return hr;
    }
    let copy_hr = copy_image_rows(dst_scan0, dst_pitch, src_scan0, src_pitch, row_bytes, height);
    let _ = unlock2d(dest_2d);
    copy_hr
}

fn membuf_media_vtable() -> &'static [usize; 8] {
    static VTBL: OnceLock<[usize; 8]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            membuf_qi as *const () as usize,
            membuf_add_ref as *const () as usize,
            membuf_release as *const () as usize,
            membuf_lock as *const () as usize,
            membuf_unlock as *const () as usize,
            membuf_get_current_length as *const () as usize,
            membuf_set_current_length as *const () as usize,
            membuf_get_max_length as *const () as usize,
        ])
    })
}

fn membuf_2d_vtable() -> &'static [usize; 12] {
    static VTBL: OnceLock<[usize; 12]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            membuf2d_qi as *const () as usize,
            membuf2d_add_ref as *const () as usize,
            membuf2d_release as *const () as usize,
            membuf2d_lock2d as *const () as usize,
            membuf2d_unlock2d as *const () as usize,
            membuf2d_get_scanline0_and_pitch as *const () as usize,
            membuf2d_is_contiguous_format as *const () as usize,
            membuf2d_get_contiguous_length as *const () as usize,
            membuf2d_contiguous_copy_to as *const () as usize,
            membuf2d_contiguous_copy_from as *const () as usize,
            membuf2d_lock2d_size as *const () as usize,
            membuf2d_copy2d_to as *const () as usize,
        ])
    })
}

fn create_membuf_obj(state: MemBufState) -> *mut c_void {
    let obj = Box::new(MemBufObj {
        vtbl_media: membuf_media_vtable().as_ptr(),
        vtbl_2d: membuf_2d_vtable().as_ptr(),
        refs: AtomicU32::new(1),
        state: Mutex::new(state),
    });
    Box::into_raw(obj) as *mut c_void
}

pub unsafe extern "win64" fn MFCreateMemoryBuffer(
    cb_max_length: u32,
    pp_buffer: *mut *mut c_void,
) -> HRESULT {
    if pp_buffer.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(pp_buffer, create_membuf_obj(MemBufState::new_1d(cb_max_length, 16)));
    S_OK
}

pub unsafe extern "win64" fn MFCreateAlignedMemoryBuffer(
    cb_max_length: u32,
    cb_alignment: u32,
    pp_buffer: *mut *mut c_void,
) -> HRESULT {
    if pp_buffer.is_null() {
        return E_POINTER;
    }
    // MF_*_BYTE_ALIGNMENT constants are `align - 1` masks (e.g. MF_16_BYTE_ALIGNMENT = 15).
    let align = (cb_alignment as usize).saturating_add(1).clamp(16, 4096);
    std::ptr::write_unaligned(pp_buffer, create_membuf_obj(MemBufState::new_1d(cb_max_length, align)));
    S_OK
}

pub unsafe extern "win64" fn MFCreate2DMediaBuffer(
    dw_width: u32,
    dw_height: u32,
    dw_fourcc: u32,
    f_bottom_up: BOOL,
    pp_buffer: *mut *mut c_void,
) -> HRESULT {
    if pp_buffer.is_null() {
        return E_POINTER;
    }
    write_null_out(pp_buffer);
    if dw_width == 0 || dw_height == 0 {
        return E_INVALIDARG;
    }
    // Common FOURCC / D3DFORMAT bytes-per-pixel & height multipliers.
    let (row_bytes, total_rows) = match dw_fourcc {
        // D3DFMT_A8R8G8B8 (21), D3DFMT_X8R8G8B8 (22), AYUV
        21 | 22 | 0x5655_5941 => (dw_width.checked_mul(4), dw_height),
        // D3DFMT_R8G8B8 (20)
        20 => (dw_width.checked_mul(3), dw_height),
        // YUY2, UYVY, D3DFMT_R5G6B5 (23)
        0x3259_5559 | 0x5956_5955 | 23 => (dw_width.checked_mul(2), dw_height),
        // NV12, YV12, IYUV, I420 (4:2:0 planar: height * 3 / 2 rows of width bytes)
        0x3231_564E | 0x3231_5659 | 0x5655_5949 | 0x3032_3449 => {
            (Some(dw_width), dw_height.saturating_mul(3) / 2)
        }
        // P010 (16-bit 4:2:0)
        0x3031_3050 => (dw_width.checked_mul(2), dw_height.saturating_mul(3) / 2),
        _ => (dw_width.checked_mul(4), dw_height),
    };
    let Some(row_bytes) = row_bytes else {
        return E_INVALIDARG;
    };
    let stride = row_bytes.next_multiple_of(16);
    let state = match MemBufState::new_2d(row_bytes, total_rows, stride, f_bottom_up != 0) {
        Ok(s) => s,
        Err(hr) => return hr,
    };
    std::ptr::write_unaligned(pp_buffer, create_membuf_obj(state));
    S_OK
}

// ============================================================================
// IMFSample (inherits IMFAttributes, slots 33..46)
// ============================================================================

#[inline]
unsafe fn sample_ref<'a>(this: usize) -> Option<&'a SampleObj> {
    if this == 0 {
        None
    } else {
        Some(&*(this as *const SampleObj))
    }
}

/// Invoke `IMFMediaBuffer` methods through a buffer COM pointer's vtable.
unsafe fn buf_lock(buf: *mut c_void) -> Result<(*mut u8, u32, u32), HRESULT> {
    let vt = std::ptr::read_unaligned(buf as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut *mut u8, *mut u32, *mut u32) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(3)));
    let mut ptr = std::ptr::null_mut();
    let mut max_len = 0u32;
    let mut cur_len = 0u32;
    let hr = f(buf, &mut ptr, &mut max_len, &mut cur_len);
    if hr < 0 {
        Err(hr)
    } else {
        Ok((ptr, max_len, cur_len))
    }
}

unsafe fn buf_unlock(buf: *mut c_void) -> HRESULT {
    let vt = std::ptr::read_unaligned(buf as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(4)));
    f(buf)
}

unsafe fn buf_get_current_length(buf: *mut c_void) -> Result<u32, HRESULT> {
    let vt = std::ptr::read_unaligned(buf as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(5)));
    let mut len = 0u32;
    let hr = f(buf, &mut len);
    if hr < 0 { Err(hr) } else { Ok(len) }
}

unsafe fn buf_set_current_length(buf: *mut c_void, len: u32) -> HRESULT {
    let vt = std::ptr::read_unaligned(buf as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(6)));
    f(buf, len)
}

unsafe fn buf_get_max_length(buf: *mut c_void) -> Result<u32, HRESULT> {
    let vt = std::ptr::read_unaligned(buf as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(7)));
    let mut len = 0u32;
    let hr = f(buf, &mut len);
    if hr < 0 { Err(hr) } else { Ok(len) }
}

unsafe extern "win64" fn sample_get_flags(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    std::ptr::write_unaligned(out, s.sample.lock().unwrap().flags);
    S_OK
}

unsafe extern "win64" fn sample_set_flags(this: usize, flags: u32) -> HRESULT {
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    s.sample.lock().unwrap().flags = flags;
    S_OK
}

unsafe extern "win64" fn sample_get_time(this: usize, out: *mut i64) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    match s.sample.lock().unwrap().sample_time {
        Some(t) => {
            std::ptr::write_unaligned(out, t);
            S_OK
        }
        None => MF_E_NO_SAMPLE_TIMESTAMP,
    }
}

unsafe extern "win64" fn sample_set_time(this: usize, t: i64) -> HRESULT {
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    s.sample.lock().unwrap().sample_time = Some(t);
    S_OK
}

unsafe extern "win64" fn sample_get_duration(this: usize, out: *mut i64) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    match s.sample.lock().unwrap().sample_duration {
        Some(d) => {
            std::ptr::write_unaligned(out, d);
            S_OK
        }
        None => MF_E_NO_SAMPLE_DURATION,
    }
}

unsafe extern "win64" fn sample_set_duration(this: usize, d: i64) -> HRESULT {
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    s.sample.lock().unwrap().sample_duration = Some(d);
    S_OK
}

unsafe extern "win64" fn sample_get_buffer_count(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    std::ptr::write_unaligned(out, s.sample.lock().unwrap().buffers.len() as u32);
    S_OK
}

unsafe extern "win64" fn sample_get_buffer_by_index(
    this: usize,
    index: u32,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    let st = s.sample.lock().unwrap();
    let Some(&buf) = st.buffers.get(index as usize) else {
        return E_INVALIDARG;
    };
    com_add_ref(buf as *mut c_void);
    std::ptr::write_unaligned(out, buf as *mut c_void);
    S_OK
}

unsafe fn flatten_buffers_into(bufs: &[usize], dest: *mut c_void) -> HRESULT {
    let mut total = 0u32;
    for &b in bufs {
        match buf_get_current_length(b as *mut c_void) {
            Ok(len) => match total.checked_add(len) {
                Some(t) => total = t,
                None => return E_OUTOFMEMORY,
            },
            Err(hr) => return hr,
        }
    }
    let max_len = match buf_get_max_length(dest) {
        Ok(m) => m,
        Err(hr) => return hr,
    };
    if max_len < total {
        return MF_E_BUFFERTOOSMALL;
    }
    let (dst_ptr, _, _) = match buf_lock(dest) {
        Ok(x) => x,
        Err(hr) => return hr,
    };
    let mut offset = 0usize;
    let mut copy_hr = S_OK;
    for &b in bufs {
        let bp = b as *mut c_void;
        match buf_lock(bp) {
            Ok((src_ptr, _, cur_len)) => {
                let n = cur_len as usize;
                if n > 0 && !src_ptr.is_null() && !dst_ptr.is_null() {
                    std::ptr::copy_nonoverlapping(src_ptr, dst_ptr.add(offset), n);
                }
                offset += n;
                let _ = buf_unlock(bp);
            }
            Err(hr) => {
                copy_hr = hr;
                break;
            }
        }
    }
    let _ = buf_unlock(dest);
    if copy_hr < 0 {
        return copy_hr;
    }
    buf_set_current_length(dest, total)
}

unsafe extern "win64" fn sample_convert_to_contiguous_buffer(
    this: usize,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    let snapshot: Vec<usize> = {
        let st = s.sample.lock().unwrap();
        if st.buffers.is_empty() {
            return E_UNEXPECTED;
        }
        if st.buffers.len() == 1 {
            let single = st.buffers[0] as *mut c_void;
            com_add_ref(single);
            std::ptr::write_unaligned(out, single);
            return S_OK;
        }
        for &b in &st.buffers {
            com_add_ref(b as *mut c_void);
        }
        st.buffers.clone()
    };

    let mut total = 0u32;
    let mut total_hr = S_OK;
    for &b in &snapshot {
        match buf_get_current_length(b as *mut c_void) {
            Ok(len) => match total.checked_add(len) {
                Some(t) => total = t,
                None => {
                    total_hr = E_OUTOFMEMORY;
                    break;
                }
            },
            Err(hr) => {
                total_hr = hr;
                break;
            }
        }
    }
    if total_hr < 0 {
        for b in snapshot {
            com_release(b as *mut c_void);
        }
        return total_hr;
    }

    let mut new_buf: *mut c_void = std::ptr::null_mut();
    let hr = MFCreateMemoryBuffer(total, &mut new_buf);
    if hr < 0 {
        for b in snapshot {
            com_release(b as *mut c_void);
        }
        return hr;
    }
    let fhr = flatten_buffers_into(&snapshot, new_buf);
    for b in snapshot {
        com_release(b as *mut c_void);
    }
    if fhr < 0 {
        com_release(new_buf);
        return fhr;
    }

    // Replace sample's buffer list with [new_buf] (keep 1 ref in sample, 1 ref for caller).
    com_add_ref(new_buf);
    let old_bufs = {
        let mut st = s.sample.lock().unwrap();
        std::mem::replace(&mut st.buffers, vec![new_buf as usize])
    };
    for old in old_bufs {
        if old != 0 {
            com_release(old as *mut c_void);
        }
    }
    std::ptr::write_unaligned(out, new_buf);
    S_OK
}

unsafe extern "win64" fn sample_add_buffer(this: usize, buf: *mut c_void) -> HRESULT {
    if buf.is_null() {
        return E_POINTER;
    }
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    com_add_ref(buf);
    s.sample.lock().unwrap().buffers.push(buf as usize);
    S_OK
}

unsafe extern "win64" fn sample_remove_buffer_by_index(this: usize, index: u32) -> HRESULT {
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    let removed = {
        let mut st = s.sample.lock().unwrap();
        if (index as usize) >= st.buffers.len() {
            return E_INVALIDARG;
        }
        st.buffers.remove(index as usize)
    };
    if removed != 0 {
        com_release(removed as *mut c_void);
    }
    S_OK
}

unsafe extern "win64" fn sample_remove_all_buffers(this: usize) -> HRESULT {
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    let old = {
        let mut st = s.sample.lock().unwrap();
        std::mem::take(&mut st.buffers)
    };
    for p in old {
        if p != 0 {
            com_release(p as *mut c_void);
        }
    }
    S_OK
}

unsafe extern "win64" fn sample_get_total_length(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    let snapshot: Vec<usize> = {
        let st = s.sample.lock().unwrap();
        for &b in &st.buffers {
            com_add_ref(b as *mut c_void);
        }
        st.buffers.clone()
    };
    let mut total = 0u32;
    let mut hr = S_OK;
    for b in snapshot {
        if hr >= 0 {
            match buf_get_current_length(b as *mut c_void) {
                Ok(len) => match total.checked_add(len) {
                    Some(t) => total = t,
                    None => hr = E_OUTOFMEMORY,
                },
                Err(e) => hr = e,
            }
        }
        com_release(b as *mut c_void);
    }
    if hr >= 0 {
        std::ptr::write_unaligned(out, total);
    }
    hr
}

unsafe extern "win64" fn sample_copy_to_buffer(this: usize, dest: *mut c_void) -> HRESULT {
    if dest.is_null() {
        return E_POINTER;
    }
    let Some(s) = sample_ref(this) else {
        return E_INVALIDARG;
    };
    let snapshot: Vec<usize> = {
        let st = s.sample.lock().unwrap();
        for &b in &st.buffers {
            com_add_ref(b as *mut c_void);
        }
        st.buffers.clone()
    };
    let hr = flatten_buffers_into(&snapshot, dest);
    for b in snapshot {
        com_release(b as *mut c_void);
    }
    hr
}

fn sample_vtable() -> &'static [usize; 47] {
    static VTBL: OnceLock<[usize; 47]> = OnceLock::new();
    VTBL.get_or_init(|| {
        let base = fill_attr_slots::<SampleObj>(sample_qi as *const () as usize);
        let mut full = [0usize; 47];
        full[..33].copy_from_slice(&base);
        full[33] = sample_get_flags as *const () as usize;
        full[34] = sample_set_flags as *const () as usize;
        full[35] = sample_get_time as *const () as usize;
        full[36] = sample_set_time as *const () as usize;
        full[37] = sample_get_duration as *const () as usize;
        full[38] = sample_set_duration as *const () as usize;
        full[39] = sample_get_buffer_count as *const () as usize;
        full[40] = sample_get_buffer_by_index as *const () as usize;
        full[41] = sample_convert_to_contiguous_buffer as *const () as usize;
        full[42] = sample_add_buffer as *const () as usize;
        full[43] = sample_remove_buffer_by_index as *const () as usize;
        full[44] = sample_remove_all_buffers as *const () as usize;
        full[45] = sample_get_total_length as *const () as usize;
        full[46] = sample_copy_to_buffer as *const () as usize;
        register_vtable(full)
    })
}

pub unsafe extern "win64" fn MFCreateSample(pp_imf_sample: *mut *mut c_void) -> HRESULT {
    if pp_imf_sample.is_null() {
        return E_POINTER;
    }
    let obj = Box::new(SampleObj {
        hdr: AttrHeader {
            vtbl: sample_vtable().as_ptr(),
            refs: AtomicU32::new(1),
            attrs: AttrStore::new(8),
        },
        sample: Mutex::new(SampleState::default()),
    });
    std::ptr::write_unaligned(pp_imf_sample, Box::into_raw(obj) as *mut c_void);
    S_OK
}

// ============================================================================
// IMFDXGIDeviceManager (MFCreateDXGIDeviceManager)
// ============================================================================

static NEXT_RESET_TOKEN: AtomicU32 = AtomicU32::new(1);
static NEXT_DEV_HANDLE: AtomicUsize = AtomicUsize::new(0x4D46_0001);

struct DxgiDevMgrState {
    device: usize, // AddRef'd IUnknown*
    generation: u64,
    handles: HashMap<usize, u64>, // handle -> generation at open time
    locked_by: Option<(usize, ThreadId, u32)>,
}

impl Drop for DxgiDevMgrState {
    fn drop(&mut self) {
        if self.device != 0 {
            unsafe { com_release(self.device as *mut c_void) };
            self.device = 0;
        }
    }
}

#[repr(C)]
struct DxgiDevMgrObj {
    vtbl: *const usize,
    refs: AtomicU32,
    reset_token: u32,
    state: Mutex<DxgiDevMgrState>,
    unlock_cv: Condvar,
}

#[inline]
unsafe fn devmgr_ref<'a>(this: usize) -> Option<&'a DxgiDevMgrObj> {
    if this == 0 {
        None
    } else {
        Some(&*(this as *const DxgiDevMgrObj))
    }
}

unsafe extern "win64" fn devmgr_qi(this: usize, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let (Some(obj), Some(iid)) = (devmgr_ref(this), read_guid(riid)) else {
        return E_POINTER;
    };
    if iid == IID_IUnknown || iid == IID_IMFDXGIDeviceManager {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe extern "win64" fn devmgr_add_ref(this: usize) -> u32 {
    let Some(obj) = devmgr_ref(this) else {
        return 0;
    };
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn devmgr_release(this: usize) -> u32 {
    let Some(obj) = devmgr_ref(this) else {
        return 0;
    };
    let prev = obj.refs.fetch_sub(1, Ordering::AcqRel);
    let next = prev.saturating_sub(1);
    if next == 0 {
        drop(Box::from_raw(this as *mut DxgiDevMgrObj));
    }
    next
}

// Slot 3: CloseDeviceHandle
unsafe extern "win64" fn devmgr_close_handle(this: usize, handle: HANDLE) -> HRESULT {
    let Some(obj) = devmgr_ref(this) else {
        return E_INVALIDARG;
    };
    let h = handle as usize;
    let mut st = obj.state.lock().unwrap();
    if st.handles.remove(&h).is_none() {
        return E_HANDLE;
    }
    if st.locked_by.is_some_and(|(lh, _, _)| lh == h) {
        st.locked_by = None;
        obj.unlock_cv.notify_all();
    }
    S_OK
}

// Slot 4: GetVideoService
unsafe extern "win64" fn devmgr_get_video_service(
    this: usize,
    handle: HANDLE,
    riid: *const Guid,
    pp_service: *mut *mut c_void,
) -> HRESULT {
    if pp_service.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(pp_service, std::ptr::null_mut());
    let (Some(obj), Some(iid)) = (devmgr_ref(this), read_guid(riid)) else {
        return E_INVALIDARG;
    };
    let dev = {
        let st = obj.state.lock().unwrap();
        let Some(&h_gen) = st.handles.get(&(handle as usize)) else {
            return E_HANDLE;
        };
        if st.device == 0 {
            return MF_E_DXGI_DEVICE_NOT_INITIALIZED;
        }
        if h_gen != st.generation {
            return MF_E_DXGI_NEW_VIDEO_DEVICE;
        }
        com_add_ref(st.device as *mut c_void);
        st.device as *mut c_void
    };
    let hr = com_query(dev, &iid, pp_service);
    com_release(dev);
    hr
}

// Slot 5: LockDevice
unsafe extern "win64" fn devmgr_lock_device(
    this: usize,
    handle: HANDLE,
    riid: *const Guid,
    pp_unk_device: *mut *mut c_void,
    f_block: BOOL,
) -> HRESULT {
    if pp_unk_device.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(pp_unk_device, std::ptr::null_mut());
    let (Some(obj), Some(iid)) = (devmgr_ref(this), read_guid(riid)) else {
        return E_INVALIDARG;
    };
    let h = handle as usize;
    let me = std::thread::current().id();
    let dev = {
        let mut st = obj.state.lock().unwrap();
        loop {
            let Some(&h_gen) = st.handles.get(&h) else {
                return E_HANDLE;
            };
            if st.device == 0 {
                return MF_E_DXGI_DEVICE_NOT_INITIALIZED;
            }
            if h_gen != st.generation {
                return MF_E_DXGI_NEW_VIDEO_DEVICE;
            }
            match st.locked_by {
                None => {
                    st.locked_by = Some((h, me, 1));
                    com_add_ref(st.device as *mut c_void);
                    break st.device as *mut c_void;
                }
                Some((lh, ltid, depth)) if lh == h && ltid == me => {
                    st.locked_by = Some((lh, ltid, depth + 1));
                    com_add_ref(st.device as *mut c_void);
                    break st.device as *mut c_void;
                }
                Some(_) => {
                    if f_block == 0 {
                        return MF_E_DXGI_VIDEO_DEVICE_LOCKED;
                    }
                    st = obj.unlock_cv.wait(st).unwrap();
                }
            }
        }
    };
    let hr = com_query(dev, &iid, pp_unk_device);
    com_release(dev);
    if hr < 0 {
        let mut st = obj.state.lock().unwrap();
        if let Some((lh, ltid, depth)) = st.locked_by {
            if lh == h && ltid == me {
                if depth <= 1 {
                    st.locked_by = None;
                    obj.unlock_cv.notify_all();
                } else {
                    st.locked_by = Some((lh, ltid, depth - 1));
                }
            }
        }
    }
    hr
}

// Slot 6: OpenDeviceHandle
unsafe extern "win64" fn devmgr_open_handle(this: usize, ph_device: *mut HANDLE) -> HRESULT {
    if ph_device.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ph_device, std::ptr::null_mut());
    let Some(obj) = devmgr_ref(this) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    if st.device == 0 {
        return MF_E_DXGI_DEVICE_NOT_INITIALIZED;
    }
    let h = NEXT_DEV_HANDLE.fetch_add(1, Ordering::Relaxed);
    let cur_gen = st.generation;
    st.handles.insert(h, cur_gen);
    std::ptr::write_unaligned(ph_device, h as HANDLE);
    S_OK
}

// Slot 7: ResetDevice
unsafe extern "win64" fn devmgr_reset_device(
    this: usize,
    p_unk_device: *mut c_void,
    reset_token: u32,
) -> HRESULT {
    let Some(obj) = devmgr_ref(this) else {
        return E_INVALIDARG;
    };
    if p_unk_device.is_null() || reset_token != obj.reset_token {
        return E_INVALIDARG;
    }
    com_add_ref(p_unk_device);
    let old_dev = {
        let mut st = obj.state.lock().unwrap();
        let old = st.device;
        st.device = p_unk_device as usize;
        st.generation = st.generation.wrapping_add(1).max(1);
        st.locked_by = None;
        obj.unlock_cv.notify_all();
        old
    };
    if old_dev != 0 {
        com_release(old_dev as *mut c_void);
    }
    S_OK
}

// Slot 8: TestDevice
unsafe extern "win64" fn devmgr_test_device(this: usize, handle: HANDLE) -> HRESULT {
    let Some(obj) = devmgr_ref(this) else {
        return E_INVALIDARG;
    };
    let st = obj.state.lock().unwrap();
    let Some(&h_gen) = st.handles.get(&(handle as usize)) else {
        return E_HANDLE;
    };
    if st.device == 0 {
        return MF_E_DXGI_DEVICE_NOT_INITIALIZED;
    }
    if h_gen != st.generation {
        return MF_E_DXGI_NEW_VIDEO_DEVICE;
    }
    S_OK
}

// Slot 9: UnlockDevice
unsafe extern "win64" fn devmgr_unlock_device(
    this: usize,
    handle: HANDLE,
    _f_save_state: BOOL,
) -> HRESULT {
    let Some(obj) = devmgr_ref(this) else {
        return E_INVALIDARG;
    };
    let h = handle as usize;
    let mut st = obj.state.lock().unwrap();
    if !st.handles.contains_key(&h) {
        return E_HANDLE;
    }
    if let Some((lh, ltid, depth)) = st.locked_by {
        if lh == h {
            if depth <= 1 {
                st.locked_by = None;
                obj.unlock_cv.notify_all();
            } else {
                st.locked_by = Some((lh, ltid, depth - 1));
            }
            return S_OK;
        }
    }
    E_INVALIDARG
}

fn devmgr_vtable() -> &'static [usize; 10] {
    static VTBL: OnceLock<[usize; 10]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            devmgr_qi as *const () as usize,
            devmgr_add_ref as *const () as usize,
            devmgr_release as *const () as usize,
            devmgr_close_handle as *const () as usize,
            devmgr_get_video_service as *const () as usize,
            devmgr_lock_device as *const () as usize,
            devmgr_open_handle as *const () as usize,
            devmgr_reset_device as *const () as usize,
            devmgr_test_device as *const () as usize,
            devmgr_unlock_device as *const () as usize,
        ])
    })
}

pub unsafe extern "win64" fn MFCreateDXGIDeviceManager(
    reset_token: *mut u32,
    pp_device_manager: *mut *mut c_void,
) -> HRESULT {
    if reset_token.is_null() || pp_device_manager.is_null() {
        write_null_out(pp_device_manager);
        return E_POINTER;
    }
    let token = NEXT_RESET_TOKEN.fetch_add(1, Ordering::Relaxed).max(1);
    let obj = Box::new(DxgiDevMgrObj {
        vtbl: devmgr_vtable().as_ptr(),
        refs: AtomicU32::new(1),
        reset_token: token,
        state: Mutex::new(DxgiDevMgrState {
            device: 0,
            generation: 0,
            handles: HashMap::new(),
            locked_by: None,
        }),
        unlock_cv: Condvar::new(),
    });
    std::ptr::write_unaligned(reset_token, token);
    std::ptr::write_unaligned(pp_device_manager, Box::into_raw(obj) as *mut c_void);
    S_OK
}

// ============================================================================
// MFCreateDXGISurfaceBuffer (IMFMediaBuffer + IMF2DBuffer2 + IMFDXGIBuffer)
// ============================================================================

struct DxgiSurfBufState {
    resource: usize,        // AddRef'd surface/resource COM pointer
    dxgi_surface: usize,    // Optional AddRef'd IDXGISurface* for live Map/Unmap
    subresource_index: u32,
    bottom_up: bool,
    height: u32,
    row_bytes: u32,
    max_len: u32,
    cur_len: u32,
    locked_ptr: usize,
    locked_base: usize,
    locked_pitch: i32,
    mapped_dxgi: bool,
    lock_count: u32,
    fallback_buf: Vec<u8>,
    unknowns: HashMap<Guid, usize>, // AddRef'd IUnknown* keyed by Guid
}

impl Drop for DxgiSurfBufState {
    fn drop(&mut self) {
        if self.mapped_dxgi && self.dxgi_surface != 0 {
            unsafe { dxgi_surface_unmap(self.dxgi_surface as *mut c_void) };
            self.mapped_dxgi = false;
        }
        if self.dxgi_surface != 0 {
            unsafe { com_release(self.dxgi_surface as *mut c_void) };
            self.dxgi_surface = 0;
        }
        if self.resource != 0 {
            unsafe { com_release(self.resource as *mut c_void) };
            self.resource = 0;
        }
        for (_, p) in self.unknowns.drain() {
            if p != 0 {
                unsafe { com_release(p as *mut c_void) };
            }
        }
    }
}

#[repr(C)]
struct DxgiSurfBufObj {
    vtbl_media: *const usize, // offset 0: IMFMediaBuffer (canonical IUnknown)
    vtbl_2d: *const usize,    // offset 8: IMF2DBuffer2
    vtbl_dxgi: *const usize,  // offset 16: IMFDXGIBuffer
    refs: AtomicU32,
    state: Mutex<DxgiSurfBufState>,
}

const DXGI_SURF_2D_OFFSET: usize = std::mem::offset_of!(DxgiSurfBufObj, vtbl_2d);
const DXGI_SURF_DXGI_OFFSET: usize = std::mem::offset_of!(DxgiSurfBufObj, vtbl_dxgi);

#[repr(C)]
#[derive(Clone, Copy, Default)]
#[allow(dead_code)]
struct DxgiSurfaceDescRaw {
    width: u32,
    height: u32,
    format: u32,
    sample_count: u32,
    sample_quality: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct DxgiMappedRectRaw {
    pitch: i32,
    bits: *mut u8,
}

unsafe fn dxgi_surface_get_desc(surf: *mut c_void) -> Option<DxgiSurfaceDescRaw> {
    let vt = std::ptr::read_unaligned(surf as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut DxgiSurfaceDescRaw) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(8)));
    let mut desc = DxgiSurfaceDescRaw::default();
    if f(surf, &mut desc) >= 0 {
        Some(desc)
    } else {
        None
    }
}

unsafe fn dxgi_surface_map(surf: *mut c_void, flags: u32) -> Result<DxgiMappedRectRaw, HRESULT> {
    let vt = std::ptr::read_unaligned(surf as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut DxgiMappedRectRaw, u32) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(9)));
    let mut rect = DxgiMappedRectRaw {
        pitch: 0,
        bits: std::ptr::null_mut(),
    };
    let hr = f(surf, &mut rect, flags);
    if hr < 0 { Err(hr) } else { Ok(rect) }
}

unsafe fn dxgi_surface_unmap(surf: *mut c_void) -> HRESULT {
    let vt = std::ptr::read_unaligned(surf as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(vt.add(10)));
    f(surf)
}

#[inline]
unsafe fn dxgibuf_from_offset<'a>(this: usize, off: usize) -> Option<&'a DxgiSurfBufObj> {
    if this < off {
        None
    } else {
        Some(&*((this - off) as *const DxgiSurfBufObj))
    }
}

unsafe fn dxgibuf_qi_inner(obj: &DxgiSurfBufObj, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(iid) = read_guid(riid) else {
        return E_POINTER;
    };
    let base = obj as *const DxgiSurfBufObj as usize;
    if iid == IID_IUnknown || iid == IID_IMFMediaBuffer {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, base as *mut c_void);
        S_OK
    } else if iid == IID_IMF2DBuffer || iid == IID_IMF2DBuffer2 {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, (base + DXGI_SURF_2D_OFFSET) as *mut c_void);
        S_OK
    } else if iid == IID_IMFDXGIBuffer {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, (base + DXGI_SURF_DXGI_OFFSET) as *mut c_void);
        S_OK
    } else {
        E_NOINTERFACE
    }
}

unsafe fn dxgibuf_release_inner(obj: *mut DxgiSurfBufObj) -> u32 {
    let prev = (*obj).refs.fetch_sub(1, Ordering::AcqRel);
    let next = prev.saturating_sub(1);
    if next == 0 {
        drop(Box::from_raw(obj));
    }
    next
}

impl DxgiSurfBufState {
    unsafe fn acquire_lock(&mut self) -> Result<(*mut u8, i32, *mut u8, u32), HRESULT> {
        if self.lock_count > 0 && self.locked_ptr != 0 {
            self.lock_count += 1;
            return Ok((
                self.locked_ptr as *mut u8,
                self.locked_pitch,
                self.locked_base as *mut u8,
                self.max_len,
            ));
        }
        // Prefer live IDXGISurface::Map(READ | WRITE) when wrapping a real DXGI surface.
        if self.dxgi_surface != 0 {
            const DXGI_MAP_READ_WRITE: u32 = 1 | 2;
            if let Ok(rect) = dxgi_surface_map(self.dxgi_surface as *mut c_void, DXGI_MAP_READ_WRITE) {
                self.mapped_dxgi = true;
                self.lock_count = 1;
                let abs_pitch = rect.pitch.unsigned_abs();
                let total = abs_pitch.saturating_mul(self.height.max(1));
                self.max_len = self.max_len.max(total);
                let (scan0, pitch) = if self.bottom_up && self.height > 1 {
                    let off = (self.height as usize - 1) * (abs_pitch as usize);
                    (rect.bits.add(off), -(abs_pitch as i32))
                } else {
                    (rect.bits, rect.pitch)
                };
                self.locked_ptr = scan0 as usize;
                self.locked_base = rect.bits as usize;
                self.locked_pitch = pitch;
                return Ok((scan0, pitch, rect.bits, self.max_len));
            }
        }
        // Fallback: if resource is a CPU-mappable D3D12 buffer, lock directly.
        if let Some(d3d_res) = crate::d3d12::resource::from_ptr(self.resource as *mut c_void) {
            if let Some(p) = d3d_res.r.cpu_ptr() {
                let len = d3d_res.r.desc.Width.min(u32::MAX as u64) as u32;
                self.max_len = len;
                self.lock_count = 1;
                self.locked_ptr = p as usize;
                self.locked_base = p as usize;
                self.locked_pitch = self.row_bytes.max(len) as i32;
                return Ok((p, self.locked_pitch, p, len));
            }
        }
        // Fallback shadow linear buffer.
        let need = self.max_len.max(self.row_bytes.saturating_mul(self.height.max(1))) as usize;
        if self.fallback_buf.len() < need {
            self.fallback_buf.resize(need, 0);
        }
        let base = self.fallback_buf.as_mut_ptr();
        let stride = self.row_bytes.max(1);
        let (scan0, pitch) = if self.bottom_up && self.height > 1 {
            let off = (self.height as usize - 1) * (stride as usize);
            (base.add(off), -(stride as i32))
        } else {
            (base, stride as i32)
        };
        self.lock_count = 1;
        self.locked_ptr = scan0 as usize;
        self.locked_base = base as usize;
        self.locked_pitch = pitch;
        Ok((scan0, pitch, base, self.max_len))
    }

    unsafe fn release_lock(&mut self) {
        if self.lock_count == 0 {
            return;
        }
        self.lock_count -= 1;
        if self.lock_count == 0 {
            if self.mapped_dxgi && self.dxgi_surface != 0 {
                let _ = dxgi_surface_unmap(self.dxgi_surface as *mut c_void);
                self.mapped_dxgi = false;
            }
            self.locked_ptr = 0;
            self.locked_base = 0;
        }
    }
}

// Primary IMFMediaBuffer slots (0..7)
unsafe extern "win64" fn dxgisurf_media_qi(this: usize, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT {
    let Some(obj) = dxgibuf_from_offset(this, 0) else {
        write_null_out(ppv);
        return E_INVALIDARG;
    };
    dxgibuf_qi_inner(obj, riid, ppv)
}

unsafe extern "win64" fn dxgisurf_media_add_ref(this: usize) -> u32 {
    let Some(obj) = dxgibuf_from_offset(this, 0) else {
        return 0;
    };
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn dxgisurf_media_release(this: usize) -> u32 {
    if this == 0 {
        return 0;
    }
    dxgibuf_release_inner(this as *mut DxgiSurfBufObj)
}

unsafe extern "win64" fn dxgisurf_media_lock(
    this: usize,
    ppb_buffer: *mut *mut u8,
    pcb_max_length: *mut u32,
    pcb_current_length: *mut u32,
) -> HRESULT {
    if ppb_buffer.is_null() {
        return E_INVALIDARG;
    }
    let Some(obj) = dxgibuf_from_offset(this, 0) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    match st.acquire_lock() {
        Ok((_, _, base, max_len)) => {
            std::ptr::write_unaligned(ppb_buffer, base);
            if !pcb_max_length.is_null() {
                std::ptr::write_unaligned(pcb_max_length, max_len);
            }
            if !pcb_current_length.is_null() {
                std::ptr::write_unaligned(pcb_current_length, st.cur_len);
            }
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn dxgisurf_media_unlock(this: usize) -> HRESULT {
    let Some(obj) = dxgibuf_from_offset(this, 0) else {
        return E_INVALIDARG;
    };
    obj.state.lock().unwrap().release_lock();
    S_OK
}

unsafe extern "win64" fn dxgisurf_media_get_current_length(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    let Some(obj) = dxgibuf_from_offset(this, 0) else {
        return E_INVALIDARG;
    };
    std::ptr::write_unaligned(out, obj.state.lock().unwrap().cur_len);
    S_OK
}

unsafe extern "win64" fn dxgisurf_media_set_current_length(this: usize, len: u32) -> HRESULT {
    let Some(obj) = dxgibuf_from_offset(this, 0) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    if st.max_len > 0 && len > st.max_len {
        return E_INVALIDARG;
    }
    st.cur_len = len;
    S_OK
}

unsafe extern "win64" fn dxgisurf_media_get_max_length(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    let Some(obj) = dxgibuf_from_offset(this, 0) else {
        return E_INVALIDARG;
    };
    std::ptr::write_unaligned(out, obj.state.lock().unwrap().max_len);
    S_OK
}

// Secondary IMF2DBuffer2 slots (0..11)
unsafe extern "win64" fn dxgisurf_2d_qi(this: usize, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT {
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        write_null_out(ppv);
        return E_INVALIDARG;
    };
    dxgibuf_qi_inner(obj, riid, ppv)
}

unsafe extern "win64" fn dxgisurf_2d_add_ref(this: usize) -> u32 {
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return 0;
    };
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn dxgisurf_2d_release(this: usize) -> u32 {
    if this < DXGI_SURF_2D_OFFSET {
        return 0;
    }
    dxgibuf_release_inner((this - DXGI_SURF_2D_OFFSET) as *mut DxgiSurfBufObj)
}

unsafe extern "win64" fn dxgisurf_2d_lock2d(
    this: usize,
    ppb_scanline0: *mut *mut u8,
    pl_pitch: *mut i32,
) -> HRESULT {
    if ppb_scanline0.is_null() || pl_pitch.is_null() {
        return E_POINTER;
    }
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    match st.acquire_lock() {
        Ok((scan0, pitch, _, _)) => {
            std::ptr::write_unaligned(ppb_scanline0, scan0);
            std::ptr::write_unaligned(pl_pitch, pitch);
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn dxgisurf_2d_unlock2d(this: usize) -> HRESULT {
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return E_INVALIDARG;
    };
    obj.state.lock().unwrap().release_lock();
    S_OK
}

unsafe extern "win64" fn dxgisurf_2d_get_scanline0_and_pitch(
    this: usize,
    ppb_scanline0: *mut *mut u8,
    pl_pitch: *mut i32,
) -> HRESULT {
    if ppb_scanline0.is_null() || pl_pitch.is_null() {
        return E_POINTER;
    }
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return E_INVALIDARG;
    };
    let st = obj.state.lock().unwrap();
    if st.lock_count == 0 || st.locked_ptr == 0 {
        return MF_E_INVALIDREQUEST;
    }
    std::ptr::write_unaligned(ppb_scanline0, st.locked_ptr as *mut u8);
    std::ptr::write_unaligned(pl_pitch, st.locked_pitch);
    S_OK
}

unsafe extern "win64" fn dxgisurf_2d_is_contiguous_format(this: usize, out: *mut BOOL) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return E_INVALIDARG;
    };
    let st = obj.state.lock().unwrap();
    let contig = !st.bottom_up && st.dxgi_surface == 0;
    std::ptr::write_unaligned(out, if contig { TRUE } else { FALSE });
    S_OK
}

unsafe extern "win64" fn dxgisurf_2d_get_contiguous_length(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return E_INVALIDARG;
    };
    let st = obj.state.lock().unwrap();
    let len = st.row_bytes.saturating_mul(st.height.max(1)).max(st.max_len);
    std::ptr::write_unaligned(out, len);
    S_OK
}

unsafe extern "win64" fn dxgisurf_2d_contiguous_copy_to(
    this: usize,
    dest: *mut u8,
    dest_len: u32,
) -> HRESULT {
    if dest.is_null() {
        return E_POINTER;
    }
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    let needed = st.row_bytes.saturating_mul(st.height.max(1));
    if dest_len < needed {
        return E_INVALIDARG;
    }
    let (scan0, pitch, _, _) = match st.acquire_lock() {
        Ok(x) => x,
        Err(hr) => return hr,
    };
    let hr = copy_image_rows(dest, st.row_bytes as i32, scan0, pitch, st.row_bytes, st.height.max(1));
    st.release_lock();
    hr
}

unsafe extern "win64" fn dxgisurf_2d_contiguous_copy_from(
    this: usize,
    src: *const u8,
    src_len: u32,
) -> HRESULT {
    if src.is_null() {
        return E_POINTER;
    }
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    let needed = st.row_bytes.saturating_mul(st.height.max(1));
    if src_len < needed {
        return E_INVALIDARG;
    }
    let (scan0, pitch, _, _) = match st.acquire_lock() {
        Ok(x) => x,
        Err(hr) => return hr,
    };
    let hr = copy_image_rows(scan0, pitch, src, st.row_bytes as i32, st.row_bytes, st.height.max(1));
    st.release_lock();
    if hr >= 0 {
        st.cur_len = needed;
    }
    hr
}

unsafe extern "win64" fn dxgisurf_2d_lock2d_size(
    this: usize,
    _flags: u32,
    ppb_scanline0: *mut *mut u8,
    pl_pitch: *mut i32,
    ppb_buffer_start: *mut *mut u8,
    pcb_buffer_length: *mut u32,
) -> HRESULT {
    if ppb_scanline0.is_null() || pl_pitch.is_null() || ppb_buffer_start.is_null() || pcb_buffer_length.is_null() {
        return E_POINTER;
    }
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    match st.acquire_lock() {
        Ok((scan0, pitch, base, max_len)) => {
            std::ptr::write_unaligned(ppb_scanline0, scan0);
            std::ptr::write_unaligned(pl_pitch, pitch);
            std::ptr::write_unaligned(ppb_buffer_start, base);
            std::ptr::write_unaligned(pcb_buffer_length, max_len);
            S_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn dxgisurf_2d_copy2d_to(this: usize, dest_2d: *mut c_void) -> HRESULT {
    if dest_2d.is_null() {
        return E_POINTER;
    }
    if dest_2d as usize == this {
        return S_OK;
    }
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_2D_OFFSET) else {
        return E_INVALIDARG;
    };
    let mut st = obj.state.lock().unwrap();
    let (src_scan0, src_pitch, _, _) = match st.acquire_lock() {
        Ok(x) => x,
        Err(hr) => return hr,
    };
    let dvt = std::ptr::read_unaligned(dest_2d as *const *const usize);
    let lock2d: unsafe extern "win64" fn(*mut c_void, *mut *mut u8, *mut i32) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(dvt.add(3)));
    let unlock2d: unsafe extern "win64" fn(*mut c_void) -> HRESULT =
        std::mem::transmute(std::ptr::read_unaligned(dvt.add(4)));
    let mut dst_scan0 = std::ptr::null_mut();
    let mut dst_pitch = 0i32;
    let lhr = lock2d(dest_2d, &mut dst_scan0, &mut dst_pitch);
    let hr = if lhr < 0 {
        lhr
    } else {
        let chr = copy_image_rows(
            dst_scan0,
            dst_pitch,
            src_scan0,
            src_pitch,
            st.row_bytes,
            st.height.max(1),
        );
        let _ = unlock2d(dest_2d);
        chr
    };
    st.release_lock();
    hr
}

// Secondary IMFDXGIBuffer slots (0..6)
unsafe extern "win64" fn dxgisurf_dxgi_qi(this: usize, riid: *const Guid, ppv: *mut *mut c_void) -> HRESULT {
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_DXGI_OFFSET) else {
        write_null_out(ppv);
        return E_INVALIDARG;
    };
    dxgibuf_qi_inner(obj, riid, ppv)
}

unsafe extern "win64" fn dxgisurf_dxgi_add_ref(this: usize) -> u32 {
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_DXGI_OFFSET) else {
        return 0;
    };
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn dxgisurf_dxgi_release(this: usize) -> u32 {
    if this < DXGI_SURF_DXGI_OFFSET {
        return 0;
    }
    dxgibuf_release_inner((this - DXGI_SURF_DXGI_OFFSET) as *mut DxgiSurfBufObj)
}

unsafe extern "win64" fn dxgisurf_get_resource(
    this: usize,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let (Some(obj), Some(iid)) = (dxgibuf_from_offset(this, DXGI_SURF_DXGI_OFFSET), read_guid(riid)) else {
        return E_INVALIDARG;
    };
    let res = {
        let st = obj.state.lock().unwrap();
        if st.resource == 0 {
            return E_FAIL;
        }
        com_add_ref(st.resource as *mut c_void);
        st.resource as *mut c_void
    };
    let hr = com_query(res, &iid, ppv);
    com_release(res);
    hr
}

unsafe extern "win64" fn dxgisurf_get_subresource_index(this: usize, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    let Some(obj) = dxgibuf_from_offset(this, DXGI_SURF_DXGI_OFFSET) else {
        return E_INVALIDARG;
    };
    std::ptr::write_unaligned(out, obj.state.lock().unwrap().subresource_index);
    S_OK
}

unsafe extern "win64" fn dxgisurf_get_unknown(
    this: usize,
    guid_key: *const Guid,
    riid: *const Guid,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let (Some(obj), Some(k), Some(iid)) = (
        dxgibuf_from_offset(this, DXGI_SURF_DXGI_OFFSET),
        read_guid(guid_key),
        read_guid(riid),
    ) else {
        return E_INVALIDARG;
    };
    let unk = {
        let st = obj.state.lock().unwrap();
        match st.unknowns.get(&k).copied() {
            Some(p) if p != 0 => {
                com_add_ref(p as *mut c_void);
                p as *mut c_void
            }
            _ => return MF_E_NOT_FOUND,
        }
    };
    let hr = com_query(unk, &iid, ppv);
    com_release(unk);
    hr
}

unsafe extern "win64" fn dxgisurf_set_unknown(
    this: usize,
    guid_key: *const Guid,
    p_unk: *mut c_void,
) -> HRESULT {
    let (Some(obj), Some(k)) = (dxgibuf_from_offset(this, DXGI_SURF_DXGI_OFFSET), read_guid(guid_key)) else {
        return E_INVALIDARG;
    };
    if !p_unk.is_null() {
        com_add_ref(p_unk);
    }
    let old = {
        let mut st = obj.state.lock().unwrap();
        if p_unk.is_null() {
            st.unknowns.remove(&k)
        } else {
            st.unknowns.insert(k, p_unk as usize)
        }
    };
    if let Some(prev) = old {
        if prev != 0 {
            com_release(prev as *mut c_void);
        }
    }
    S_OK
}

fn dxgisurf_media_vtable() -> &'static [usize; 8] {
    static VTBL: OnceLock<[usize; 8]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            dxgisurf_media_qi as *const () as usize,
            dxgisurf_media_add_ref as *const () as usize,
            dxgisurf_media_release as *const () as usize,
            dxgisurf_media_lock as *const () as usize,
            dxgisurf_media_unlock as *const () as usize,
            dxgisurf_media_get_current_length as *const () as usize,
            dxgisurf_media_set_current_length as *const () as usize,
            dxgisurf_media_get_max_length as *const () as usize,
        ])
    })
}

fn dxgisurf_2d_vtable() -> &'static [usize; 12] {
    static VTBL: OnceLock<[usize; 12]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            dxgisurf_2d_qi as *const () as usize,
            dxgisurf_2d_add_ref as *const () as usize,
            dxgisurf_2d_release as *const () as usize,
            dxgisurf_2d_lock2d as *const () as usize,
            dxgisurf_2d_unlock2d as *const () as usize,
            dxgisurf_2d_get_scanline0_and_pitch as *const () as usize,
            dxgisurf_2d_is_contiguous_format as *const () as usize,
            dxgisurf_2d_get_contiguous_length as *const () as usize,
            dxgisurf_2d_contiguous_copy_to as *const () as usize,
            dxgisurf_2d_contiguous_copy_from as *const () as usize,
            dxgisurf_2d_lock2d_size as *const () as usize,
            dxgisurf_2d_copy2d_to as *const () as usize,
        ])
    })
}

fn dxgisurf_dxgi_vtable() -> &'static [usize; 7] {
    static VTBL: OnceLock<[usize; 7]> = OnceLock::new();
    VTBL.get_or_init(|| {
        register_vtable([
            dxgisurf_dxgi_qi as *const () as usize,
            dxgisurf_dxgi_add_ref as *const () as usize,
            dxgisurf_dxgi_release as *const () as usize,
            dxgisurf_get_resource as *const () as usize,
            dxgisurf_get_subresource_index as *const () as usize,
            dxgisurf_get_unknown as *const () as usize,
            dxgisurf_set_unknown as *const () as usize,
        ])
    })
}

pub unsafe extern "win64" fn MFCreateDXGISurfaceBuffer(
    riid: *const Guid,
    punk_surface: *mut c_void,
    u_subresource_index: u32,
    f_bottom_up_when_linear: BOOL,
    pp_buffer: *mut *mut c_void,
) -> HRESULT {
    if pp_buffer.is_null() {
        return E_POINTER;
    }
    write_null_out(pp_buffer);
    let Some(req_iid) = read_guid(riid) else {
        return E_INVALIDARG;
    };
    if punk_surface.is_null() {
        return E_INVALIDARG;
    }

    // Verify the surface supports the requested IID (e.g., ID3D11Texture2D, IDXGISurface, ID3D12Resource).
    let mut queried: *mut c_void = std::ptr::null_mut();
    let qhr = com_query(punk_surface, &req_iid, &mut queried);
    if qhr < 0 || queried.is_null() {
        return if qhr < 0 { qhr } else { E_NOINTERFACE };
    }

    // Also probe IDXGISurface for live 2D dimensions & Map/Unmap support.
    let mut dxgi_surf: *mut c_void = std::ptr::null_mut();
    let _ = com_query(punk_surface, &crate::d3d11::types::IID_IDXGISurface, &mut dxgi_surf);

    let (height, row_bytes, max_len) = if !dxgi_surf.is_null() {
        if let Some(d) = dxgi_surface_get_desc(dxgi_surf) {
            let fi = d3d12_metal::format::info(d.format);
            let bpp = if fi.bytes > 0 { fi.bytes as u32 } else { 4 };
            let rb = d.width.saturating_mul(bpp);
            let total = rb.saturating_mul(d.height.max(1));
            (d.height.max(1), rb, total)
        } else {
            (1, 0, 0)
        }
    } else if let Some(d3d_res) = crate::d3d12::resource::from_ptr(queried) {
        let w = d3d_res.r.desc.Width.min(u32::MAX as u64) as u32;
        let h = d3d_res.r.desc.Height.max(1);
        (h, w, w.saturating_mul(h))
    } else {
        (1, 0, 0)
    };

    let obj = Box::new(DxgiSurfBufObj {
        vtbl_media: dxgisurf_media_vtable().as_ptr(),
        vtbl_2d: dxgisurf_2d_vtable().as_ptr(),
        vtbl_dxgi: dxgisurf_dxgi_vtable().as_ptr(),
        refs: AtomicU32::new(1),
        state: Mutex::new(DxgiSurfBufState {
            resource: queried as usize,
            dxgi_surface: dxgi_surf as usize,
            subresource_index: u_subresource_index,
            bottom_up: f_bottom_up_when_linear != 0,
            height,
            row_bytes,
            max_len,
            cur_len: max_len,
            locked_ptr: 0,
            locked_base: 0,
            locked_pitch: row_bytes as i32,
            mapped_dxgi: false,
            lock_count: 0,
            fallback_buf: Vec::new(),
            unknowns: HashMap::new(),
        }),
    });
    std::ptr::write_unaligned(pp_buffer, Box::into_raw(obj) as *mut c_void);
    S_OK
}

// ============================================================================
// Unit Tests (must remain at the bottom for win32-rt/build.rs export scanner)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::d3d11::types::{D3D11_TEXTURE2D_DESC, DXGI_SAMPLE_DESC, IID_ID3D11Texture2D, IID_IDXGISurface};

    unsafe fn vslot<F: Copy>(obj: *mut c_void, idx: usize) -> F {
        let vt = std::ptr::read_unaligned(obj as *const *const usize);
        std::mem::transmute_copy(&std::ptr::read_unaligned(vt.add(idx)))
    }

    #[test]
    fn startup_and_shutdown_refcounted() {
        unsafe {
            assert_eq!(MFStartup(0, 0), MF_E_BAD_STARTUP_VERSION);
            assert_eq!(MFStartup(0x0002_0001, 0), MF_E_BAD_STARTUP_VERSION);
            let before = startup_ref_count();
            assert_eq!(MFStartup(MF_VERSION, MFSTARTUP_FULL), S_OK);
            assert_eq!(startup_ref_count(), before + 1);
            assert_eq!(MFStartup((1 << 16) | MF_API_VERSION, MFSTARTUP_NOSOCKET), S_OK);
            assert_eq!(startup_ref_count(), before + 2);
            assert_eq!(MFShutdown(), S_OK);
            assert_eq!(startup_ref_count(), before + 1);
            assert_eq!(MFShutdown(), S_OK);
            assert_eq!(startup_ref_count(), before);
        }
    }

    #[test]
    fn copy_image_positive_and_negative_strides() {
        unsafe {
            // 4x3 image with 2 bytes per row padding in src (stride=6) -> bottom-up dst (stride=-4).
            let src: [u8; 18] = [
                1, 2, 3, 4, 0xAA, 0xBB, // row 0
                5, 6, 7, 8, 0xCC, 0xDD, // row 1
                9, 10, 11, 12, 0xEE, 0xFF, // row 2
            ];
            let mut dst = [0u8; 12];
            // Top-down copy first
            assert_eq!(MFCopyImage(dst.as_mut_ptr(), 4, src.as_ptr(), 6, 4, 3), S_OK);
            assert_eq!(dst, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);

            // Negative destination stride (bottom-up flip): row 0 goes to end, row 2 goes to start.
            let mut flipped = [0u8; 12];
            let dst_last_row = flipped.as_mut_ptr().add(8);
            assert_eq!(MFCopyImage(dst_last_row, -4, src.as_ptr(), 6, 4, 3), S_OK);
            assert_eq!(flipped, [9, 10, 11, 12, 5, 6, 7, 8, 1, 2, 3, 4]);

            // Negative source stride back to positive destination restores original order.
            let mut restored = [0u8; 12];
            assert_eq!(
                MFCopyImage(restored.as_mut_ptr(), 4, flipped.as_ptr().add(8), -4, 4, 3),
                S_OK
            );
            assert_eq!(restored, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
        }
    }

    #[test]
    fn memory_buffer_and_2d_buffer_roundtrip() {
        unsafe {
            let mut buf: *mut c_void = std::ptr::null_mut();
            assert_eq!(MFCreateMemoryBuffer(64, &mut buf), S_OK);
            assert!(!buf.is_null());

            let lock: unsafe extern "win64" fn(*mut c_void, *mut *mut u8, *mut u32, *mut u32) -> HRESULT = vslot(buf, 3);
            let unlock: unsafe extern "win64" fn(*mut c_void) -> HRESULT = vslot(buf, 4);
            let get_cur: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT = vslot(buf, 5);
            let set_cur: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT = vslot(buf, 6);
            let get_max: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT = vslot(buf, 7);

            let mut max_len = 0u32;
            let mut cur_len = 99u32;
            assert_eq!(get_max(buf, &mut max_len), S_OK);
            assert_eq!(max_len, 64);
            assert_eq!(get_cur(buf, &mut cur_len), S_OK);
            assert_eq!(cur_len, 0);
            assert_eq!(set_cur(buf, 65), E_INVALIDARG);
            assert_eq!(set_cur(buf, 16), S_OK);

            let mut ptr: *mut u8 = std::ptr::null_mut();
            assert_eq!(lock(buf, &mut ptr, &mut max_len, &mut cur_len), S_OK);
            assert_eq!(max_len, 64);
            assert_eq!(cur_len, 16);
            for i in 0..16u8 {
                *ptr.add(i as usize) = i.wrapping_mul(3);
            }
            assert_eq!(unlock(buf), S_OK);

            // QI for IMF2DBuffer and verify COM identity round-trip + Lock2D/ContiguousCopyTo.
            let mut buf2d: *mut c_void = std::ptr::null_mut();
            assert_eq!(com_query(buf, &IID_IMF2DBuffer, &mut buf2d), S_OK);
            assert_ne!(buf2d, buf);
            let mut unk_from_2d: *mut c_void = std::ptr::null_mut();
            assert_eq!(com_query(buf2d, &IID_IUnknown, &mut unk_from_2d), S_OK);
            assert_eq!(unk_from_2d, buf);
            assert_eq!(com_release(unk_from_2d), 2);

            let lock2d: unsafe extern "win64" fn(*mut c_void, *mut *mut u8, *mut i32) -> HRESULT = vslot(buf2d, 3);
            let unlock2d: unsafe extern "win64" fn(*mut c_void) -> HRESULT = vslot(buf2d, 4);
            let get_s0: unsafe extern "win64" fn(*mut c_void, *mut *mut u8, *mut i32) -> HRESULT = vslot(buf2d, 5);
            let copy_to: unsafe extern "win64" fn(*mut c_void, *mut u8, u32) -> HRESULT = vslot(buf2d, 8);

            let mut s0 = std::ptr::null_mut();
            let mut pitch = 0i32;
            assert_eq!(get_s0(buf2d, &mut s0, &mut pitch), MF_E_INVALIDREQUEST);
            assert_eq!(lock2d(buf2d, &mut s0, &mut pitch), S_OK);
            assert_eq!(pitch, 64);
            assert_eq!(*s0.add(5), 15);
            assert_eq!(get_s0(buf2d, &mut s0, &mut pitch), S_OK);
            assert_eq!(unlock2d(buf2d), S_OK);

            let mut out = [0u8; 64];
            assert_eq!(copy_to(buf2d, out.as_mut_ptr(), 64), S_OK);
            assert_eq!(out[5], 15);

            assert_eq!(com_release(buf2d), 1);
            assert_eq!(com_release(buf), 0);
        }
    }

    #[test]
    fn attributes_and_media_type_all_value_types() {
        unsafe {
            let mut mt: *mut c_void = std::ptr::null_mut();
            assert_eq!(MFCreateMediaType(&mut mt), S_OK);

            let k_u32 = guid("10000001-0000-0000-0000-000000000001");
            let k_u64 = guid("10000002-0000-0000-0000-000000000002");
            let k_f64 = guid("10000003-0000-0000-0000-000000000003");
            let k_guid = guid("10000004-0000-0000-0000-000000000004");
            let k_str = guid("10000005-0000-0000-0000-000000000005");
            let k_blob = guid("10000006-0000-0000-0000-000000000006");
            let k_unk = guid("10000007-0000-0000-0000-000000000007");

            let get_u32: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u32) -> HRESULT = vslot(mt, 7);
            let get_u64: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u64) -> HRESULT = vslot(mt, 8);
            let get_f64: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut f64) -> HRESULT = vslot(mt, 9);
            let get_guid: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut Guid) -> HRESULT = vslot(mt, 10);
            let get_str: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u16, u32, *mut u32) -> HRESULT = vslot(mt, 12);
            let get_blob: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u8, u32, *mut u32) -> HRESULT = vslot(mt, 15);
            let get_unk: unsafe extern "win64" fn(*mut c_void, *const Guid, *const Guid, *mut *mut c_void) -> HRESULT = vslot(mt, 17);
            let set_u32: unsafe extern "win64" fn(*mut c_void, *const Guid, u32) -> HRESULT = vslot(mt, 21);
            let set_u64: unsafe extern "win64" fn(*mut c_void, *const Guid, u64) -> HRESULT = vslot(mt, 22);
            let set_guid: unsafe extern "win64" fn(*mut c_void, *const Guid, *const Guid) -> HRESULT = vslot(mt, 24);
            let set_str: unsafe extern "win64" fn(*mut c_void, *const Guid, *const u16) -> HRESULT = vslot(mt, 25);
            let set_blob: unsafe extern "win64" fn(*mut c_void, *const Guid, *const u8, u32) -> HRESULT = vslot(mt, 26);
            let set_unk: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut c_void) -> HRESULT = vslot(mt, 27);
            let lock_store: unsafe extern "win64" fn(*mut c_void) -> HRESULT = vslot(mt, 28);
            let unlock_store: unsafe extern "win64" fn(*mut c_void) -> HRESULT = vslot(mt, 29);
            let get_count: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT = vslot(mt, 30);
            let copy_all: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT = vslot(mt, 32);
            let get_major: unsafe extern "win64" fn(*mut c_void, *mut Guid) -> HRESULT = vslot(mt, 33);
            let is_comp: unsafe extern "win64" fn(*mut c_void, *mut BOOL) -> HRESULT = vslot(mt, 34);
            let is_eq: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut u32) -> HRESULT = vslot(mt, 35);

            // Reentrant LockStore should not deadlock nested writes.
            assert_eq!(lock_store(mt), S_OK);
            let slot23: usize = vslot(mt, 23);
            assert_eq!(slot23, crate::bridge_entry!(attr_set_double) as usize);
            assert_eq!(set_u32(mt, &k_u32, 0x1234_5678), S_OK);
            assert_eq!(set_u64(mt, &k_u64, 0xDEAD_BEEF_CAFE_BABE), S_OK);
            assert_eq!(attr_set_double(mt as usize, &k_f64, 3.25), S_OK);
            assert_eq!(set_guid(mt, &k_guid, &MF_VIDEO_FORMAT_NV12), S_OK);
            assert_eq!(set_guid(mt, &MF_MT_MAJOR_TYPE, &k_guid), S_OK);
            assert_eq!(set_guid(mt, &MF_MT_SUBTYPE, &MF_VIDEO_FORMAT_NV12), S_OK);
            let w: Vec<u16> = "HelloMF\0".encode_utf16().collect();
            assert_eq!(set_str(mt, &k_str, w.as_ptr()), S_OK);
            let blob = [10u8, 20, 30, 40, 50];
            assert_eq!(set_blob(mt, &k_blob, blob.as_ptr(), blob.len() as u32), S_OK);

            let mut sub_attr: *mut c_void = std::ptr::null_mut();
            assert_eq!(MFCreateAttributes(&mut sub_attr, 4), S_OK);
            assert_eq!(set_unk(mt, &k_unk, sub_attr), S_OK);
            assert_eq!(unlock_store(mt), S_OK);

            let mut v32 = 0u32;
            assert_eq!(get_u32(mt, &k_u32, &mut v32), S_OK);
            assert_eq!(v32, 0x1234_5678);
            let mut v64 = 0u64;
            assert_eq!(get_u64(mt, &k_u64, &mut v64), S_OK);
            assert_eq!(v64, 0xDEAD_BEEF_CAFE_BABE);
            let mut vf64 = 0.0f64;
            assert_eq!(get_f64(mt, &k_f64, &mut vf64), S_OK);
            assert!((vf64 - 3.25).abs() < 1e-12);
            let mut vg = Guid::default();
            assert_eq!(get_guid(mt, &k_guid, &mut vg), S_OK);
            assert_eq!(vg, MF_VIDEO_FORMAT_NV12);

            let mut sbuf = [0u16; 16];
            let mut slen = 0u32;
            assert_eq!(get_str(mt, &k_str, sbuf.as_mut_ptr(), 16, &mut slen), S_OK);
            assert_eq!(slen, 7);
            assert_eq!(String::from_utf16_lossy(&sbuf[..7]), "HelloMF");

            let mut bbuf = [0u8; 8];
            let mut blen = 0u32;
            assert_eq!(get_blob(mt, &k_blob, bbuf.as_mut_ptr(), 2, &mut blen), MF_E_BUFFERTOOSMALL);
            assert_eq!(blen, 5);
            assert_eq!(get_blob(mt, &k_blob, bbuf.as_mut_ptr(), 8, &mut blen), S_OK);
            assert_eq!(&bbuf[..5], &blob);

            let mut got_unk: *mut c_void = std::ptr::null_mut();
            assert_eq!(get_unk(mt, &k_unk, &IID_IMFAttributes, &mut got_unk), S_OK);
            assert_eq!(got_unk, sub_attr);
            assert_eq!(com_release(got_unk), 2);

            // MediaType methods
            let mut major = Guid::default();
            assert_eq!(get_major(mt, &mut major), S_OK);
            assert_eq!(major, k_guid);
            let mut comp = TRUE;
            assert_eq!(is_comp(mt, &mut comp), S_OK);
            assert_eq!(comp, FALSE); // NV12 is uncompressed

            // CopyAllItems into another IMFMediaType + IsEqual
            let mut mt2: *mut c_void = std::ptr::null_mut();
            assert_eq!(MFCreateMediaType(&mut mt2), S_OK);
            assert_eq!(copy_all(mt, mt2), S_OK);
            let mut cnt = 0u32;
            assert_eq!(get_count(mt2, &mut cnt), S_OK);
            assert_eq!(cnt, 9);
            let mut eq_flags = 0u32;
            assert_eq!(is_eq(mt, mt2, &mut eq_flags), S_OK);
            assert_eq!(
                eq_flags,
                MF_MEDIATYPE_EQUAL_MAJOR_TYPES
                    | MF_MEDIATYPE_EQUAL_FORMAT_TYPES
                    | MF_MEDIATYPE_EQUAL_FORMAT_DATA
                    | MF_MEDIATYPE_EQUAL_FORMAT_USER_DATA
            );

            // Exercise GetItem/SetItem (PROPVARIANT), GetItemType, Compare, CompareItem,
            // GetAllocatedString, GetAllocatedBlob, GetItemByIndex, DeleteItem, DeleteAllItems.
            let get_item: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u8) -> HRESULT = vslot(mt, 3);
            let get_type: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u32) -> HRESULT = vslot(mt, 4);
            let cmp_item: unsafe extern "win64" fn(*mut c_void, *const Guid, *const u8, *mut BOOL) -> HRESULT = vslot(mt, 5);
            let cmp_all: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, *mut BOOL) -> HRESULT = vslot(mt, 6);
            let get_str_len: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u32) -> HRESULT = vslot(mt, 11);
            let get_alloc_str: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut u16, *mut u32) -> HRESULT = vslot(mt, 13);
            let get_blob_sz: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u32) -> HRESULT = vslot(mt, 14);
            let get_alloc_blob: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut u8, *mut u32) -> HRESULT = vslot(mt, 16);
            let set_item: unsafe extern "win64" fn(*mut c_void, *const Guid, *const u8) -> HRESULT = vslot(mt, 18);
            let del_item: unsafe extern "win64" fn(*mut c_void, *const Guid) -> HRESULT = vslot(mt, 19);
            let del_all: unsafe extern "win64" fn(*mut c_void) -> HRESULT = vslot(mt, 20);
            let get_by_idx: unsafe extern "win64" fn(*mut c_void, u32, *mut Guid, *mut u8) -> HRESULT = vslot(mt, 31);

            let mut ty = 0u32;
            assert_eq!(get_type(mt, &k_str, &mut ty), S_OK);
            assert_eq!(ty, MF_ATTRIBUTE_STRING);
            let mut pv = [0u8; 24];
            assert_eq!(get_item(mt, &k_str, pv.as_mut_ptr()), S_OK);
            let mut eq_b = FALSE;
            assert_eq!(cmp_item(mt, &k_str, pv.as_ptr(), &mut eq_b), S_OK);
            assert_eq!(eq_b, TRUE);
            let k_copy = guid("10000099-0000-0000-0000-000000000099");
            assert_eq!(set_item(mt2, &k_copy, pv.as_ptr()), S_OK);
            clear_propvariant_local(pv.as_mut_ptr());

            let mut match_b = FALSE;
            assert_eq!(cmp_all(mt, mt2, MF_ATTRIBUTES_MATCH_OUR_ITEMS, &mut match_b), S_OK);
            assert_eq!(match_b, TRUE);
            assert_eq!(cmp_all(mt, mt2, MF_ATTRIBUTES_MATCH_ALL_ITEMS, &mut match_b), S_OK);
            assert_eq!(match_b, FALSE);

            let mut slen2 = 0u32;
            assert_eq!(get_str_len(mt, &k_str, &mut slen2), S_OK);
            assert_eq!(slen2, 7);
            let mut alloc_s: *mut u16 = std::ptr::null_mut();
            let mut alloc_slen = 0u32;
            assert_eq!(get_alloc_str(mt, &k_str, &mut alloc_s, &mut alloc_slen), S_OK);
            assert_eq!(alloc_slen, 7);
            assert_eq!(String::from_utf16_lossy(std::slice::from_raw_parts(alloc_s, 7)), "HelloMF");
            CoTaskMemFree(alloc_s as *mut c_void);

            let mut bsz = 0u32;
            assert_eq!(get_blob_sz(mt, &k_blob, &mut bsz), S_OK);
            assert_eq!(bsz, 5);
            let mut alloc_b: *mut u8 = std::ptr::null_mut();
            let mut alloc_bsz = 0u32;
            assert_eq!(get_alloc_blob(mt, &k_blob, &mut alloc_b, &mut alloc_bsz), S_OK);
            assert_eq!(alloc_bsz, 5);
            assert_eq!(std::slice::from_raw_parts(alloc_b, 5), &blob);
            CoTaskMemFree(alloc_b as *mut c_void);

            let mut idx_key = Guid::default();
            assert_eq!(get_by_idx(mt, 0, &mut idx_key, std::ptr::null_mut()), S_OK);
            assert_eq!(idx_key, k_u32);

            assert_eq!(del_item(mt2, &k_copy), S_OK);
            assert_eq!(get_type(mt2, &k_copy, &mut ty), MF_E_ATTRIBUTENOTFOUND);
            assert_eq!(del_all(mt2), S_OK);
            assert_eq!(get_count(mt2, &mut cnt), S_OK);
            assert_eq!(cnt, 0);

            assert_eq!(com_release(mt2), 0);
            assert_eq!(com_release(mt), 0);
            assert_eq!(com_release(sub_attr), 0);
        }
    }

    #[test]
    fn aligned_and_2d_media_buffer_top_down_and_bottom_up() {
        unsafe {
            let mut abuf = std::ptr::null_mut();
            assert_eq!(MFCreateAlignedMemoryBuffer(128, 255, &mut abuf), S_OK);
            let (aptr, amax, _) = buf_lock(abuf).unwrap();
            assert_eq!(amax, 128);
            assert_eq!((aptr as usize) % 256, 0);
            assert_eq!(buf_unlock(abuf), S_OK);
            assert_eq!(com_release(abuf), 0);

            // 4x3 RGBA8 bottom-up 2D buffer (row_bytes = 16, height = 3)
            let mut bu_m = std::ptr::null_mut();
            let mut td_m = std::ptr::null_mut();
            assert_eq!(MFCreate2DMediaBuffer(4, 3, 21, TRUE, &mut bu_m), S_OK);
            assert_eq!(MFCreate2DMediaBuffer(4, 3, 21, FALSE, &mut td_m), S_OK);

            let mut bu2d = std::ptr::null_mut();
            let mut td2d = std::ptr::null_mut();
            assert_eq!(com_query(bu_m, &IID_IMF2DBuffer2, &mut bu2d), S_OK);
            assert_eq!(com_query(td_m, &IID_IMF2DBuffer2, &mut td2d), S_OK);

            let is_contig: unsafe extern "win64" fn(*mut c_void, *mut BOOL) -> HRESULT = vslot(bu2d, 6);
            let get_clen: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT = vslot(bu2d, 7);
            let copy_to: unsafe extern "win64" fn(*mut c_void, *mut u8, u32) -> HRESULT = vslot(bu2d, 8);
            let copy_from: unsafe extern "win64" fn(*mut c_void, *const u8, u32) -> HRESULT = vslot(bu2d, 9);
            let lock2d_sz: unsafe extern "win64" fn(*mut c_void, u32, *mut *mut u8, *mut i32, *mut *mut u8, *mut u32) -> HRESULT = vslot(bu2d, 10);
            let unlock2d: unsafe extern "win64" fn(*mut c_void) -> HRESULT = vslot(bu2d, 4);
            let copy2d_to: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT = vslot(bu2d, 11);

            let mut cflag = TRUE;
            assert_eq!(is_contig(bu2d, &mut cflag), S_OK);
            assert_eq!(cflag, FALSE);
            assert_eq!(is_contig(td2d, &mut cflag), S_OK);
            assert_eq!(cflag, TRUE);

            let mut clen = 0u32;
            assert_eq!(get_clen(bu2d, &mut clen), S_OK);
            assert_eq!(clen, 48);

            let src: Vec<u8> = (0u8..48).collect();
            assert_eq!(copy_from(bu2d, src.as_ptr(), 48), S_OK);

            let mut scan0 = std::ptr::null_mut();
            let mut pitch = 0i32;
            let mut start = std::ptr::null_mut();
            let mut blen = 0u32;
            assert_eq!(lock2d_sz(bu2d, 0, &mut scan0, &mut pitch, &mut start, &mut blen), S_OK);
            assert_eq!(pitch, -16);
            assert_eq!(blen, 48);
            assert_eq!(scan0, start.add(32));
            assert_eq!(std::slice::from_raw_parts(scan0, 16), &src[0..16]);
            assert_eq!(std::slice::from_raw_parts(start, 16), &src[32..48]);
            assert_eq!(unlock2d(bu2d), S_OK);

            // Copy2DTo from bottom-up -> top-down, then ContiguousCopyTo back out.
            assert_eq!(copy2d_to(bu2d, td2d), S_OK);
            let mut out = [0u8; 48];
            assert_eq!(copy_to(td2d, out.as_mut_ptr(), 48), S_OK);
            assert_eq!(out.as_slice(), src.as_slice());

            assert_eq!(com_release(bu2d), 1);
            assert_eq!(com_release(bu_m), 0);
            assert_eq!(com_release(td2d), 1);
            assert_eq!(com_release(td_m), 0);
        }
    }

    #[test]
    fn sample_buffers_flatten_timestamps_and_attributes() {
        unsafe {
            let mut sample: *mut c_void = std::ptr::null_mut();
            assert_eq!(MFCreateSample(&mut sample), S_OK);

            let get_flags: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT = vslot(sample, 33);
            let set_flags: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT = vslot(sample, 34);
            let get_time: unsafe extern "win64" fn(*mut c_void, *mut i64) -> HRESULT = vslot(sample, 35);
            let set_time: unsafe extern "win64" fn(*mut c_void, i64) -> HRESULT = vslot(sample, 36);
            let get_dur: unsafe extern "win64" fn(*mut c_void, *mut i64) -> HRESULT = vslot(sample, 37);
            let set_dur: unsafe extern "win64" fn(*mut c_void, i64) -> HRESULT = vslot(sample, 38);
            let get_cnt: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT = vslot(sample, 39);
            let get_by_idx: unsafe extern "win64" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT = vslot(sample, 40);
            let convert: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT = vslot(sample, 41);
            let add_buf: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT = vslot(sample, 42);
            let rem_by_idx: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT = vslot(sample, 43);
            let rem_all: unsafe extern "win64" fn(*mut c_void) -> HRESULT = vslot(sample, 44);
            let get_total: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT = vslot(sample, 45);
            let copy_to_buf: unsafe extern "win64" fn(*mut c_void, *mut c_void) -> HRESULT = vslot(sample, 46);
            let set_u32: unsafe extern "win64" fn(*mut c_void, *const Guid, u32) -> HRESULT = vslot(sample, 21);
            let get_u32: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut u32) -> HRESULT = vslot(sample, 7);

            let mut flags = 99u32;
            assert_eq!(get_flags(sample, &mut flags), S_OK);
            assert_eq!(flags, 0);
            assert_eq!(set_flags(sample, 0x5A), S_OK);
            assert_eq!(get_flags(sample, &mut flags), S_OK);
            assert_eq!(flags, 0x5A);

            let mut t = 0i64;
            assert_eq!(get_time(sample, &mut t), MF_E_NO_SAMPLE_TIMESTAMP);
            assert_eq!(get_dur(sample, &mut t), MF_E_NO_SAMPLE_DURATION);
            assert_eq!(set_time(sample, 100_000), S_OK);
            assert_eq!(set_dur(sample, 333_333), S_OK);
            assert_eq!(get_time(sample, &mut t), S_OK);
            assert_eq!(t, 100_000);
            assert_eq!(get_dur(sample, &mut t), S_OK);
            assert_eq!(t, 333_333);

            // Inherited IMFAttributes on IMFSample
            let k = guid("aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee");
            assert_eq!(set_u32(sample, &k, 77), S_OK);
            let mut v = 0u32;
            assert_eq!(get_u32(sample, &k, &mut v), S_OK);
            assert_eq!(v, 77);

            // Create two buffers and flatten via ConvertToContiguousBuffer
            let mut b1 = std::ptr::null_mut();
            let mut b2 = std::ptr::null_mut();
            assert_eq!(MFCreateMemoryBuffer(8, &mut b1), S_OK);
            assert_eq!(MFCreateMemoryBuffer(8, &mut b2), S_OK);
            let (p1, _, _) = buf_lock(b1).unwrap();
            std::ptr::copy_nonoverlapping([1u8, 2, 3, 4].as_ptr(), p1, 4);
            assert_eq!(buf_unlock(b1), S_OK);
            assert_eq!(buf_set_current_length(b1, 4), S_OK);

            let (p2, _, _) = buf_lock(b2).unwrap();
            std::ptr::copy_nonoverlapping([5u8, 6, 7].as_ptr(), p2, 3);
            assert_eq!(buf_unlock(b2), S_OK);
            assert_eq!(buf_set_current_length(b2, 3), S_OK);

            assert_eq!(add_buf(sample, b1), S_OK);
            assert_eq!(add_buf(sample, b2), S_OK);
            assert_eq!(com_release(b1), 1);
            assert_eq!(com_release(b2), 1);

            let mut cnt = 0u32;
            assert_eq!(get_cnt(sample, &mut cnt), S_OK);
            assert_eq!(cnt, 2);
            let mut total = 0u32;
            assert_eq!(get_total(sample, &mut total), S_OK);
            assert_eq!(total, 7);

            // GetBufferByIndex + CopyToBuffer (small buffer fails, 8-byte succeeds)
            let mut got0 = std::ptr::null_mut();
            assert_eq!(get_by_idx(sample, 0, &mut got0), S_OK);
            assert_eq!(got0, b1);
            assert_eq!(com_release(got0), 1);

            let mut small_dst = std::ptr::null_mut();
            let mut ok_dst = std::ptr::null_mut();
            assert_eq!(MFCreateMemoryBuffer(4, &mut small_dst), S_OK);
            assert_eq!(MFCreateMemoryBuffer(8, &mut ok_dst), S_OK);
            assert_eq!(copy_to_buf(sample, small_dst), MF_E_BUFFERTOOSMALL);
            assert_eq!(copy_to_buf(sample, ok_dst), S_OK);
            let (po, _, cur_o) = buf_lock(ok_dst).unwrap();
            assert_eq!(cur_o, 7);
            assert_eq!(std::slice::from_raw_parts(po, 7), &[1, 2, 3, 4, 5, 6, 7]);
            assert_eq!(buf_unlock(ok_dst), S_OK);
            assert_eq!(com_release(small_dst), 0);
            assert_eq!(com_release(ok_dst), 0);

            let mut flat = std::ptr::null_mut();
            assert_eq!(convert(sample, &mut flat), S_OK);
            assert_eq!(get_cnt(sample, &mut cnt), S_OK);
            assert_eq!(cnt, 1);

            let (pf, _, cur) = buf_lock(flat).unwrap();
            assert_eq!(cur, 7);
            assert_eq!(std::slice::from_raw_parts(pf, 7), &[1, 2, 3, 4, 5, 6, 7]);
            assert_eq!(buf_unlock(flat), S_OK);

            assert_eq!(rem_by_idx(sample, 0), S_OK);
            assert_eq!(get_cnt(sample, &mut cnt), S_OK);
            assert_eq!(cnt, 0);
            assert_eq!(add_buf(sample, flat), S_OK);
            assert_eq!(rem_all(sample), S_OK);
            assert_eq!(get_cnt(sample, &mut cnt), S_OK);
            assert_eq!(cnt, 0);

            assert_eq!(com_release(flat), 0);
            assert_eq!(com_release(sample), 0);
        }
    }

    #[test]
    fn dxgi_device_manager_and_surface_buffer() {
        unsafe {
            // Create a real D3D11 device + Texture2D (skipped if no Metal device).
            if d3d12_metal::metal::device().is_none() {
                return;
            }
            let mut d3d11_dev: *mut c_void = std::ptr::null_mut();
            let hr = crate::d3d11::D3D11CreateDevice(
                std::ptr::null_mut(),
                1, // D3D_DRIVER_TYPE_HARDWARE
                std::ptr::null_mut(),
                0,
                std::ptr::null(),
                0,
                7,
                &mut d3d11_dev,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert_eq!(hr, S_OK);

            // IMFDXGIDeviceManager lifecycle
            let mut token = 0u32;
            let mut mgr: *mut c_void = std::ptr::null_mut();
            assert_eq!(MFCreateDXGIDeviceManager(&mut token, &mut mgr), S_OK);
            assert_ne!(token, 0);

            let close_h: unsafe extern "win64" fn(*mut c_void, HANDLE) -> HRESULT = vslot(mgr, 3);
            let get_svc: unsafe extern "win64" fn(*mut c_void, HANDLE, *const Guid, *mut *mut c_void) -> HRESULT = vslot(mgr, 4);
            let lock_dev: unsafe extern "win64" fn(*mut c_void, HANDLE, *const Guid, *mut *mut c_void, BOOL) -> HRESULT = vslot(mgr, 5);
            let open_h: unsafe extern "win64" fn(*mut c_void, *mut HANDLE) -> HRESULT = vslot(mgr, 6);
            let reset_dev: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32) -> HRESULT = vslot(mgr, 7);
            let test_dev: unsafe extern "win64" fn(*mut c_void, HANDLE) -> HRESULT = vslot(mgr, 8);
            let unlock_dev: unsafe extern "win64" fn(*mut c_void, HANDLE, BOOL) -> HRESULT = vslot(mgr, 9);

            let mut h1: HANDLE = std::ptr::null_mut();
            assert_eq!(open_h(mgr, &mut h1), MF_E_DXGI_DEVICE_NOT_INITIALIZED);
            assert_eq!(reset_dev(mgr, d3d11_dev, token ^ 0xFF), E_INVALIDARG);
            assert_eq!(reset_dev(mgr, d3d11_dev, token), S_OK);
            assert_eq!(open_h(mgr, &mut h1), S_OK);
            assert_eq!(test_dev(mgr, h1), S_OK);

            let mut svc: *mut c_void = std::ptr::null_mut();
            assert_eq!(get_svc(mgr, h1, &IID_IUnknown, &mut svc), S_OK);
            assert!(!svc.is_null());
            com_release(svc);

            let mut locked_dev: *mut c_void = std::ptr::null_mut();
            assert_eq!(lock_dev(mgr, h1, &IID_IUnknown, &mut locked_dev, FALSE), S_OK);
            assert!(!locked_dev.is_null());

            // Second handle cannot non-blocking lock while h1 holds it.
            let mut h2: HANDLE = std::ptr::null_mut();
            assert_eq!(open_h(mgr, &mut h2), S_OK);
            let mut locked2: *mut c_void = std::ptr::null_mut();
            assert_eq!(lock_dev(mgr, h2, &IID_IUnknown, &mut locked2, FALSE), MF_E_DXGI_VIDEO_DEVICE_LOCKED);

            assert_eq!(unlock_dev(mgr, h1, FALSE), S_OK);
            assert_eq!(com_release(locked_dev), 2); // d3d11_dev + mgr

            // ResetDevice invalidates existing handles.
            assert_eq!(reset_dev(mgr, d3d11_dev, token), S_OK);
            assert_eq!(test_dev(mgr, h1), MF_E_DXGI_NEW_VIDEO_DEVICE);
            assert_eq!(close_h(mgr, h1), S_OK);
            assert_eq!(close_h(mgr, h2), S_OK);
            assert_eq!(close_h(mgr, h1), E_HANDLE);

            // Create a real 4x4 RGBA8 D3D11 Texture2D and wrap with MFCreateDXGISurfaceBuffer.
            let create_tex2d: unsafe extern "win64" fn(
                *mut c_void,
                *const D3D11_TEXTURE2D_DESC,
                *const c_void,
                *mut *mut c_void,
            ) -> HRESULT = vslot(d3d11_dev, 5);
            let tex_desc = D3D11_TEXTURE2D_DESC {
                Width: 4,
                Height: 4,
                MipLevels: 1,
                ArraySize: 1,
                Format: 28, // DXGI_FORMAT_R8G8B8A8_UNORM
                SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
                Usage: 0,
                BindFlags: 8, // SHADER_RESOURCE
                CPUAccessFlags: 0,
                MiscFlags: 0,
            };
            let mut tex: *mut c_void = std::ptr::null_mut();
            assert_eq!(create_tex2d(d3d11_dev, &tex_desc, std::ptr::null(), &mut tex), S_OK);

            let mut mf_buf: *mut c_void = std::ptr::null_mut();
            assert_eq!(MFCreateDXGISurfaceBuffer(&IID_ID3D11Texture2D, tex, 0, FALSE, &mut mf_buf), S_OK);

            let mut dxgi_buf: *mut c_void = std::ptr::null_mut();
            assert_eq!(com_query(mf_buf, &IID_IMFDXGIBuffer, &mut dxgi_buf), S_OK);

            let get_res: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dxgi_buf, 3);
            let get_sub: unsafe extern "win64" fn(*mut c_void, *mut u32) -> HRESULT = vslot(dxgi_buf, 4);
            let get_unk: unsafe extern "win64" fn(*mut c_void, *const Guid, *const Guid, *mut *mut c_void) -> HRESULT = vslot(dxgi_buf, 5);
            let set_unk: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut c_void) -> HRESULT = vslot(dxgi_buf, 6);

            let mut got_surf: *mut c_void = std::ptr::null_mut();
            assert_eq!(get_res(dxgi_buf, &IID_IDXGISurface, &mut got_surf), S_OK);
            assert!(!got_surf.is_null());
            com_release(got_surf);

            let mut sub_idx = 99u32;
            assert_eq!(get_sub(dxgi_buf, &mut sub_idx), S_OK);
            assert_eq!(sub_idx, 0);

            let uk = guid("22222222-3333-4444-5555-666666666666");
            assert_eq!(set_unk(dxgi_buf, &uk, mgr), S_OK);
            let mut got_mgr: *mut c_void = std::ptr::null_mut();
            assert_eq!(get_unk(dxgi_buf, &uk, &IID_IMFDXGIDeviceManager, &mut got_mgr), S_OK);
            assert_eq!(got_mgr, mgr);
            com_release(got_mgr);

            // Lock2D through real IDXGISurface::Map/Unmap
            let mut buf2d: *mut c_void = std::ptr::null_mut();
            assert_eq!(com_query(mf_buf, &IID_IMF2DBuffer, &mut buf2d), S_OK);
            let lock2d: unsafe extern "win64" fn(*mut c_void, *mut *mut u8, *mut i32) -> HRESULT = vslot(buf2d, 3);
            let unlock2d: unsafe extern "win64" fn(*mut c_void) -> HRESULT = vslot(buf2d, 4);
            let mut scan0 = std::ptr::null_mut();
            let mut pitch = 0i32;
            assert_eq!(lock2d(buf2d, &mut scan0, &mut pitch), S_OK);
            assert!(!scan0.is_null());
            assert!(pitch >= 16);
            *scan0 = 0xAB;
            assert_eq!(unlock2d(buf2d), S_OK);

            com_release(buf2d);
            com_release(dxgi_buf);
            com_release(mf_buf);
            com_release(tex);
            com_release(mgr);
            com_release(d3d11_dev);
        }
    }
}
