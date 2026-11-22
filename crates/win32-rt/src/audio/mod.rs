//! WASAPI on CoreAudio: the COM audio surface the game's statically linked
//! Wwise 2022.1.4 (`AkSinkCoreAudio.cpp`) and Remedy's device selection use.
//!
//! | Module | Interfaces |
//! |---|---|
//! | [`mmdevapi`] | `IMMDeviceEnumerator`, `IMMDeviceCollection`, `IMMDevice`, `IMMEndpoint` |
//! | [`propsys`] | `IPropertyStore` (DeviceFormat blob, friendly name, container id, ...) |
//! | [`audioclient`] | `IAudioClient`/`2`/`3` (shared mode), `IAudioRenderClient`, `IAudioCaptureClient`, `IAudioClock` |
//! | [`spatial`] | `ISpatialAudioClient`, `IAudioFormatEnumerator`, `ISpatialAudioObjectRenderStream`, `ISpatialAudioObject` (3D spatial audio + static beds on CoreAudio) |
//! | [`backend`] | CoreAudio AUHAL output + null device + read-only input probe, realtime ring consumer, event pump, device listeners |
//! | [`devnotify`] | `IMMNotificationClient` delivery (hot-plug, default device), device-loss invalidation |
//! | [`activate`] | `mmdevapi!ActivateAudioInterfaceAsync` (builtin `mmdevapi.dll`, used by xaudio2_9redist) |
//! | [`ring`], [`format`], [`tap`] | SPSC sample ring, WAVEFORMATEX + conversion/downmix, `AW2_AUDIO_TAP` |
//!
//! Entry point: `ole32!CoCreateInstance(CLSID_MMDeviceEnumerator)` via
//! [`create_instance`]. Capture endpoints (`eCapture` / `eAll`) enumerate
//! honestly (read-only CoreAudio default input query or `"Default Microphone"`)
//! and serve paced silent PCM packets via `IAudioCaptureClient`. Spatial audio
//! (`ISpatialAudioClient`) is activated via `IMMDevice::Activate` and mixes
//! static beds + up to 128 dynamic 3D objects to stereo on CoreAudio. Not
//! provided (honest `E_NOINTERFACE` + `STUB` log): `IAudioEndpointVolume`.
//! Bink's XAudio2 path runs Microsoft's xaudio2_9redist.dll from the game dir
//! on top of this WASAPI (builtin `mmdevapi.dll` ordinal 17 + `avrt.dll`);
//! long term (M4) XAudio2 becomes a FAudio-backed builtin.
//!
//! Environment:
//! * `AW2_AUDIO_TAP=out/audio_tap.wav` — record what each render client submits (+ `.tsv` sidecar).
//! * `AW2_AUDIO_BACKEND=null` — no host audio; a real-time null device consumes the stream.
//! * `AW2_AUDIO_RATE=device|<hz>` — mix format rate (default 48000; AUHAL resamples).
//! * `AW2_AUDIO_CHANNELS=device|1|2|4|6|8` — mix format channels (default 2).
//! * `AW2_AUDIO_DEVICE=<name substring|UID>` — output device for the default endpoint.
//! * `AW2_AUDIO_MIN_BUFFER_FRAMES=<n>` — floor for `GetBufferSize`.
//! * `AW2_AUDIO_LOW_LATENCY=0` — report true padding (client keeps the whole
//!   ~180 ms buffer queued) instead of adaptive headroom (`audioclient::Stream::padding`).
//! * `AW2_AUDIO_RT=0` — don't promote the render thread to a Mach time-constraint thread.
//! * `AW2_AUDIO_DEVICE_EVENTS=0` — don't watch CoreAudio device changes.

pub mod activate;
#[cfg(feature = "wwise-bridge")]
pub mod akoracle;
#[cfg(feature = "wwise-bridge")]
pub mod akrec;
#[cfg(feature = "wwise-bridge")]
pub mod aktrace;
pub mod audioclient;
pub mod backend;
pub mod com;
pub mod devnotify;
pub mod format;
#[cfg(feature = "wwise-bridge")]
pub mod harness;
pub mod mmdevapi;
pub mod propsys;
pub mod ring;
pub mod spatial;
pub mod tap;
pub mod wavcmp;

use crate::types::*;
use std::ffi::c_void;

/// COM class registry for the audio classes. `None` = not an audio class.
pub unsafe fn create_instance(clsid: &com::GUID, iid: &com::GUID, out: *mut *mut c_void) -> Option<HRESULT> {
    if *clsid == com::CLSID_MMDeviceEnumerator {
        log::debug!(target: "audio", "CoCreateInstance(MMDeviceEnumerator, {iid:?})");
        return Some(mmdevapi::create_enumerator(iid, out));
    }
    None
}

/// `HRESULT` as hex for logs.
pub fn hr(h: HRESULT) -> String {
    format!("{:#010x}", h as u32)
}

#[cfg(test)]
pub(crate) struct AudioTestGuard {
    _guard: std::sync::MutexGuard<'static, ()>,
}

#[cfg(test)]
impl Drop for AudioTestGuard {
    fn drop(&mut self) {
        tap::override_next(None);
        backend::force_output_devices(None);
        backend::force_input_devices(None);
        backend::force_backend(None);
        devnotify::clear_removed_for_test();
        #[cfg(feature = "wwise-bridge")]
        akrec::set_native_mode(None);
    }
}

/// Serializes unit tests that mutate shared process-global audio singletons
/// (`backend::FORCED*`, `tap::OVERRIDE`, `devnotify`, `akrec`), resetting all
/// overrides automatically on drop (including on panic).
///
/// Lock order rule: if a test also acquires `crate::user32::TEST_SERIAL`,
/// acquire `TEST_SERIAL` *before* calling `audio_test_lock()`.
#[cfg(test)]
pub(crate) fn audio_test_lock() -> AudioTestGuard {
    static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    let guard = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    AudioTestGuard { _guard: guard }
}

#[cfg(test)]
mod tests;
