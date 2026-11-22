//! Minimal COM plumbing for the audio objects: `GUID`, the IIDs/CLSIDs we
//! serve, and a generic ref-counted object header.
//!
//! Every COM object here is `#[repr(C)] ComObj<T> { vtbl, refs, inner }`, so
//! the interface pointer handed to the game is the object pointer itself
//! (`*mut ComObj<T>`). Vtables are `static`s of typed `extern "win64"`
//! function pointers laid out exactly like the Windows SDK declarations
//! (IUnknown first, then the interface's methods in header order).

pub use crate::types::GUID;
use crate::types::{E_POINTER, HRESULT, S_OK};
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};

// `GUID` itself (layout + Debug formatting) lives in `crate::types`; the audio
// code additionally needs construction in consts, unaligned reads and equality.
impl GUID {
    pub const fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        GUID { data1, data2, data3, data4 }
    }
    pub unsafe fn read(p: *const GUID) -> Option<GUID> {
        if p.is_null() {
            None
        } else {
            Some(std::ptr::read_unaligned(p))
        }
    }
}

impl PartialEq for GUID {
    fn eq(&self, o: &Self) -> bool {
        self.data1 == o.data1 && self.data2 == o.data2 && self.data3 == o.data3 && self.data4 == o.data4
    }
}
impl Eq for GUID {}

// --- Interface / class identifiers (Windows SDK: Unknwn.h, mmdeviceapi.h,
// audioclient.h, audiopolicy.h, spatialaudioclient.h, propsys.h, ks.h). Values
// cross-checked against the copies embedded in AlanWake2.exe where it has them
// (see docs/AUDIO.md §2.3 / module docs of `audio`).

pub const IID_IUnknown: GUID = GUID::new(0x00000000, 0x0000, 0x0000, [0xC0, 0, 0, 0, 0, 0, 0, 0x46]);
pub const CLSID_MMDeviceEnumerator: GUID =
    GUID::new(0xBCDE0395, 0xE52F, 0x467C, [0x8E, 0x3D, 0xC4, 0x57, 0x92, 0x91, 0x69, 0x2E]);
pub const IID_IMMDeviceEnumerator: GUID =
    GUID::new(0xA95664D2, 0x9614, 0x4F35, [0xA7, 0x46, 0xDE, 0x8D, 0xB6, 0x36, 0x17, 0xE6]);
pub const IID_IMMDeviceCollection: GUID =
    GUID::new(0x0BD7A1BE, 0x7A1A, 0x44DB, [0x83, 0x97, 0xCC, 0x53, 0x92, 0x38, 0x7B, 0x5E]);
pub const IID_IMMDevice: GUID = GUID::new(0xD666063F, 0x1587, 0x4E43, [0x81, 0xF1, 0xB9, 0x48, 0xE8, 0x07, 0x36, 0x3F]);
pub const IID_IMMEndpoint: GUID = GUID::new(0x1BE09788, 0x6894, 0x4089, [0x85, 0x86, 0x9A, 0x2A, 0x6C, 0x26, 0x5A, 0xC5]);
pub const IID_IPropertyStore: GUID =
    GUID::new(0x886D8EEB, 0x8CF2, 0x4446, [0x8D, 0x02, 0xCD, 0xBA, 0x1D, 0xBD, 0xCF, 0x99]);
pub const IID_IAudioClient: GUID = GUID::new(0x1CB9AD4C, 0xDBFA, 0x4C32, [0xB1, 0x78, 0xC2, 0xF5, 0x68, 0xA7, 0x03, 0xB2]);
pub const IID_IAudioClient2: GUID = GUID::new(0x726778CD, 0xF60A, 0x4EDA, [0x82, 0xDE, 0xE4, 0x76, 0x10, 0xCD, 0x78, 0xAA]);
pub const IID_IAudioClient3: GUID = GUID::new(0x7ED4EE07, 0x8E67, 0x4CD4, [0x8C, 0x1A, 0x2B, 0x7A, 0x59, 0x87, 0xAD, 0x42]);
pub const IID_IAudioRenderClient: GUID =
    GUID::new(0xF294ACFC, 0x3146, 0x4483, [0xA7, 0xBF, 0xAD, 0xDC, 0xA7, 0xC2, 0x60, 0xE2]);
pub const IID_IAudioCaptureClient: GUID =
    GUID::new(0xC8ADBD64, 0xE71E, 0x48A0, [0xA4, 0xDE, 0x18, 0x5C, 0x39, 0x5C, 0xD3, 0x17]);
pub const IID_IAudioClock: GUID = GUID::new(0xCD63314F, 0x3FBA, 0x4A1B, [0x81, 0x2C, 0xEF, 0x96, 0x35, 0x87, 0x28, 0xE7]);
pub const IID_IAudioClock2: GUID = GUID::new(0x6F49FF73, 0x6727, 0x49AC, [0xA0, 0x08, 0xD9, 0x8C, 0xF5, 0xE7, 0x00, 0x48]);
pub const IID_ISimpleAudioVolume: GUID =
    GUID::new(0x87CE5498, 0x68D6, 0x44E5, [0x92, 0x15, 0x6D, 0xA4, 0x7E, 0xF8, 0x83, 0xD8]);
pub const IID_IAudioStreamVolume: GUID =
    GUID::new(0x93014887, 0x242D, 0x4068, [0x8A, 0x15, 0xCF, 0x5E, 0x93, 0xB9, 0x0F, 0xE3]);
pub const IID_IAudioSessionControl: GUID =
    GUID::new(0xF4B1A599, 0x7266, 0x4319, [0xA8, 0xCA, 0xE7, 0x0A, 0xCB, 0x11, 0xE8, 0xCD]);
pub const IID_IAudioSessionControl2: GUID =
    GUID::new(0xBFB7FF88, 0x7239, 0x4FC9, [0x8F, 0xA2, 0x07, 0xC9, 0x50, 0xBE, 0x9C, 0x6D]);
pub const IID_IAudioSessionManager: GUID =
    GUID::new(0xBFA971F1, 0x4D5E, 0x40BB, [0x93, 0x5E, 0x96, 0x70, 0x39, 0xBF, 0xBE, 0xE4]);
pub const IID_IAudioSessionManager2: GUID =
    GUID::new(0x77AA99A0, 0x1BD6, 0x484F, [0x8B, 0xC7, 0x2C, 0x65, 0x4C, 0x9A, 0x9B, 0x6F]);
pub const IID_IAudioEndpointVolume: GUID =
    GUID::new(0x5CDF2C82, 0x841E, 0x4546, [0x97, 0x22, 0x0C, 0xF7, 0x40, 0x78, 0x22, 0x9A]);
pub const IID_ISpatialAudioClient: GUID =
    GUID::new(0xBBF8E066, 0xAAAA, 0x49BE, [0x9A, 0x4D, 0xFD, 0x2A, 0x85, 0x8E, 0xA2, 0x7F]);

pub const KSDATAFORMAT_SUBTYPE_PCM: GUID = GUID::new(0x00000001, 0x0000, 0x0010, [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71]);
pub const KSDATAFORMAT_SUBTYPE_IEEE_FLOAT: GUID =
    GUID::new(0x00000003, 0x0000, 0x0010, [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71]);

// --- HRESULTs (winerror.h / audioclient.h AUDCLNT_ERR(n) = 0x88890000 | n).
pub const CLASS_E_NOAGGREGATION: HRESULT = 0x8004_0110u32 as i32;
pub const E_NOTFOUND: HRESULT = 0x8007_0490u32 as i32; // HRESULT_FROM_WIN32(ERROR_NOT_FOUND)
pub const STG_E_ACCESSDENIED: HRESULT = 0x8003_0005u32 as i32;
pub const AUDCLNT_E_NOT_INITIALIZED: HRESULT = 0x8889_0001u32 as i32;
pub const AUDCLNT_E_ALREADY_INITIALIZED: HRESULT = 0x8889_0002u32 as i32;
pub const AUDCLNT_E_WRONG_ENDPOINT_TYPE: HRESULT = 0x8889_0003u32 as i32;
pub const AUDCLNT_E_DEVICE_INVALIDATED: HRESULT = 0x8889_0004u32 as i32;
pub const AUDCLNT_E_NOT_STOPPED: HRESULT = 0x8889_0005u32 as i32;
pub const AUDCLNT_E_BUFFER_TOO_LARGE: HRESULT = 0x8889_0006u32 as i32;
pub const AUDCLNT_E_OUT_OF_ORDER: HRESULT = 0x8889_0007u32 as i32;
pub const AUDCLNT_E_UNSUPPORTED_FORMAT: HRESULT = 0x8889_0008u32 as i32;
pub const AUDCLNT_E_INVALID_SIZE: HRESULT = 0x8889_0009u32 as i32;
pub const AUDCLNT_E_BUFFER_OPERATION_PENDING: HRESULT = 0x8889_000Bu32 as i32;
pub const AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED: HRESULT = 0x8889_000Eu32 as i32;
pub const AUDCLNT_E_EVENTHANDLE_NOT_EXPECTED: HRESULT = 0x8889_0011u32 as i32;
pub const AUDCLNT_E_EVENTHANDLE_NOT_SET: HRESULT = 0x8889_0014u32 as i32;
pub const AUDCLNT_E_ENDPOINT_CREATE_FAILED: HRESULT = 0x8889_000Fu32 as i32;
pub const AUDCLNT_E_INVALID_STREAM_FLAG: HRESULT = 0x8889_0021u32 as i32;
pub const AUDCLNT_S_BUFFER_EMPTY: HRESULT = 0x0889_0001u32 as i32;

/// Header shared by all our COM objects.
#[repr(C)]
pub struct ComObj<V: 'static, T> {
    pub vtbl: &'static V,
    pub refs: AtomicU32,
    pub inner: T,
}

impl<V: 'static, T> ComObj<V, T> {
    /// Heap-allocate with refcount 1 and return the interface pointer.
    pub fn create(vtbl: &'static V, inner: T) -> *mut Self {
        Box::into_raw(Box::new(ComObj { vtbl, refs: AtomicU32::new(1), inner }))
    }
    pub unsafe fn add_ref(this: *mut Self) -> u32 {
        (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
    }
    pub unsafe fn release(this: *mut Self) -> u32 {
        let n = (*this).refs.fetch_sub(1, Ordering::AcqRel) - 1;
        if n == 0 {
            drop(Box::from_raw(this));
        }
        n
    }
    pub unsafe fn inner<'a>(this: *mut Self) -> &'a T {
        &(*this).inner
    }
}

/// Generic IUnknown method bodies: `$iids` is the list of IIDs this object answers.
macro_rules! com_iunknown {
    ($ty:ty, [$($iid:expr),* $(,)?]) => {
        pub(crate) unsafe extern "win64" fn qi(this: *mut $ty, riid: *const $crate::audio::com::GUID, out: *mut *mut ::std::ffi::c_void) -> $crate::types::HRESULT {
            if out.is_null() {
                return $crate::types::E_POINTER;
            }
            ::std::ptr::write_unaligned(out, ::std::ptr::null_mut());
            let Some(iid) = $crate::audio::com::GUID::read(riid) else { return $crate::types::E_POINTER };
            if iid == $crate::audio::com::IID_IUnknown $(|| iid == $iid)* {
                <$ty>::add_ref(this);
                ::std::ptr::write_unaligned(out, this as *mut ::std::ffi::c_void);
                return $crate::types::S_OK;
            }
            ::log::debug!(target: "audio", "{}: QueryInterface({:?}) -> E_NOINTERFACE", stringify!($ty), iid);
            $crate::types::E_NOINTERFACE
        }
        pub(crate) unsafe extern "win64" fn add_ref(this: *mut $ty) -> u32 {
            <$ty>::add_ref(this)
        }
        pub(crate) unsafe extern "win64" fn release(this: *mut $ty) -> u32 {
            <$ty>::release(this)
        }
    };
}
pub(crate) use com_iunknown;

/// Allocate `n` bytes with `CoTaskMemAlloc` semantics (freed by the game via
/// `CoTaskMemFree` / `PropVariantClear`).
pub unsafe fn co_alloc(n: usize) -> *mut u8 {
    crate::heap::alloc(n.max(1), false) as *mut u8
}

/// `CoTaskMemAlloc`'d, NUL-terminated UTF-16 copy of `s`.
pub unsafe fn co_wstr(s: &str) -> *mut u16 {
    let w = crate::strings::to_wide_z(s);
    let p = co_alloc(w.len() * 2) as *mut u16;
    if !p.is_null() {
        std::ptr::copy_nonoverlapping(w.as_ptr(), p, w.len());
    }
    p
}

pub unsafe fn write_out<T>(out: *mut T, v: T) -> HRESULT {
    if out.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(out, v);
    S_OK
}

pub type RawPtr = *mut c_void;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn guid_layout_and_format() {
        assert_eq!(std::mem::size_of::<GUID>(), 16);
        assert_eq!(format!("{:?}", CLSID_MMDeviceEnumerator), "{BCDE0395-E52F-467C-8E3D-C4579291692E}");
        // Same bytes as the copy in AlanWake2.exe @ 0x143413a20.
        let b: [u8; 16] = unsafe { std::mem::transmute(CLSID_MMDeviceEnumerator) };
        assert_eq!(b, [0x95, 0x03, 0xde, 0xbc, 0x2f, 0xe5, 0x7c, 0x46, 0x8e, 0x3d, 0xc4, 0x57, 0x92, 0x91, 0x69, 0x2e]);
    }
}
