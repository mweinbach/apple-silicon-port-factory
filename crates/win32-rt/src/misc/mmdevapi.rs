//! mmdevapi.dll: the one flat export, `ActivateAudioInterfaceAsync`
//! (ordinal 17, as in Windows / Wine's mmdevapi.spec). xaudio2_9redist.dll
//! delay-imports exactly `MMDevAPI.DLL!#17`. The COM classes
//! (`MMDeviceEnumerator`) are served by ole32!CoCreateInstance -> crate::audio.

use crate::types::*;
use std::ffi::c_void;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[(17, "ActivateAudioInterfaceAsync")];

pub unsafe extern "win64" fn ActivateAudioInterfaceAsync(
    path: *const u16,
    riid: *const crate::audio::com::GUID,
    params: *const u8,
    handler: *mut c_void,
    op: *mut *mut c_void,
) -> HRESULT {
    crate::audio::activate::activate_audio_interface_async(path, riid, params, handler, op)
}
