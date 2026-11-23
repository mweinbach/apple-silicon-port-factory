//! Builtin `xaudio2_9redist.dll` (`xaudio2_9` / `xaudio2_8` / `xaudio2_7`) + X3DAudio runtime.
//!
//! Implements a real software mixer backed by `crate::audio::backend`'s CoreAudio / Null output
//! stream (macOS AUHAL and iOS RemoteIO, with automatic `Backend::Null` fallback when headless or
//! `AW2_AUDIO_BACKEND=null`):
//! * `IXAudio2`: engine lifecycle (`StartEngine`, `StopEngine`, `CommitChanges`,
//!   `GetPerformanceData`, `RegisterForCallbacks`/`UnregisterForCallbacks`), voice creation.
//! * `IXAudio2MasteringVoice`: owns the host output stream and mixer thread, supports volume,
//!   channel volumes, voice filter, live XAPO effect chains, and `GetChannelMask`.
//! * `IXAudio2SubmixVoice`: real multi-stage submix buses (`ProcessingStage`-ordered) with volume,
//!   channel volumes, output matrix, send lists, live XAPO effect chains, and per-voice/per-send
//!   filters.
//! * `IXAudio2SourceVoice`: decodes `WAVE_FORMAT_PCM` (8/16/24/32-bit), `WAVE_FORMAT_IEEE_FLOAT`
//!   (32/64-bit), and `WAVE_FORMAT_EXTENSIBLE` into normalized `f32`, performs linear-interpolated
//!   sample-rate & pitch (`SetFrequencyRatio`) resampling, honors `PlayBegin`/`PlayLength`,
//!   `LoopBegin`/`LoopLength`/`LoopCount` (`XAUDIO2_LOOP_INFINITE`), `ExitLoop`,
//!   `FlushSourceBuffers`, `Discontinuity`, `SetSourceSampleRate`, state-variable & 1-pole filters,
//!   live XAPO effect chains (builtin fast-path & arbitrary guest COM `IXAPO`), volume/channel-
//!   volume/output-matrix routing, accurate `GetState`, and safely marshaled
//!   `IXAudio2VoiceCallback` delivery on a dedicated Win32 callback thread.
//! * Builtin XAPO COM effects (`IXAPO` + `IXAPOParameters`): `CreateAudioReverb` (multichannel
//!   Schroeder/Freeverb), `CreateAudioVolumeMeter` (per-channel peak/RMS), and `CreateFX`
//!   (`FXEQ` 4-band parametric biquad EQ, `FXMasteringLimiter` envelope peak limiter, `FXReverb`,
//!   and `FXEcho` fractional delay + feedback).
//! * `X3DAudioInitialize` & `X3DAudioCalculate`: 20-byte handle encoding, 3D distance & inverse-
//!   distance / custom curve attenuation, listener/emitter cones, Doppler shift, LPF coefficients,
//!   and stereo/surround speaker panning into `pMatrixCoefficients`.

mod mixer;
mod voices;
mod x3daudio;
mod xapo;

pub use mixer::XAudio2Engine;

use crate::audio::com::GUID;
use crate::strings::{rd, wr};
use mixer::{EngineCore, E_POINTER, S_OK};
use std::ffi::c_void;
use std::sync::atomic::AtomicU32;
use voices::{ensure_vtbls_registered, XAUDIO2_VTBL};
use xapo::{
    reverb_config_from_fxreverb, reverb_config_from_xaudio2, xapo_initialize, xapo_release,
    BuiltinXapoCom, BuiltinXapoKind, FxEchoDsp, FxEqDsp, FxMasteringLimiterDsp, FxReverbParameters,
    SchroederReverbDsp, VolumeMeterDsp, XAudio2FxReverbParameters, CLSID_FX_ECHO, CLSID_FX_ECHO_27,
    CLSID_FX_EQ, CLSID_FX_EQ_27, CLSID_FX_MASTERING_LIMITER, CLSID_FX_MASTERING_LIMITER_27,
    CLSID_FX_REVERB, CLSID_FX_REVERB_27, REGDB_E_CLASSNOTREG,
};

pub static ORDINALS: &[(u16, &str)] = &[
    (1, "XAudio2Create"),
    (2, "CreateAudioReverb"),
    (3, "CreateAudioVolumeMeter"),
    (4, "CreateFX"),
    (5, "X3DAudioInitialize"),
    (6, "X3DAudioCalculate"),
];

pub static EXTRA_EXPORTS: &[crate::Export] = &[crate::Export {
    name: "X3DAudioInitialize",
    ptr: crate::Ptr(crate::bridge_entry!(x3daudio_initialize_impl)),
}];

unsafe extern "win64" fn x3daudio_initialize_impl(
    speaker_mask: u32,
    speed_of_sound: f32,
    instance: *mut [u8; 20],
) -> i32 {
    x3daudio::x3daudio_initialize_core(speaker_mask, speed_of_sound, instance)
}
crate::win64_bridge!(fn x3daudio_initialize_impl(u32, f32, *mut [u8; 20]) -> i32);

/// Rust-callable wrapper for `X3DAudioInitialize` (guest calls go through `EXTRA_EXPORTS` +
/// `win64_bridge!` so `speed_of_sound: f32` is read from Win64 XMM1 on ARM64).
pub unsafe fn X3DAudioInitialize(
    speaker_mask: u32,
    speed_of_sound: f32,
    instance: *mut [u8; 20],
) -> i32 {
    x3daudio_initialize_impl(speaker_mask, speed_of_sound, instance)
}

// ============================================================================
// Public DLL Exports
// ============================================================================

pub unsafe extern "win64" fn XAudio2Create(
    pp_xaudio2: *mut *mut XAudio2Engine,
    _flags: u32,
    _processor: u32,
) -> i32 {
    ensure_vtbls_registered();
    if pp_xaudio2.is_null() {
        return E_POINTER;
    }
    log::info!(target: "audio", "xaudio2_9redist (builtin): XAudio2Create -> S_OK");
    let engine = Box::new(XAudio2Engine {
        vtbl: &XAUDIO2_VTBL.0,
        refs: AtomicU32::new(1),
        core: EngineCore::new(),
    });
    wr(pp_xaudio2, Box::into_raw(engine));
    S_OK
}

pub unsafe extern "win64" fn CreateAudioReverb(ppv: *mut usize) -> i32 {
    ensure_vtbls_registered();
    if ppv.is_null() {
        return E_POINTER;
    }
    wr(ppv, 0);
    let params = XAudio2FxReverbParameters::default();
    let cfg = reverb_config_from_xaudio2(&params, 48_000);
    let obj = BuiltinXapoCom::alloc(BuiltinXapoKind::AudioReverb {
        params,
        dsp: SchroederReverbDsp::new(cfg),
    });
    wr(ppv, obj as usize);
    S_OK
}

pub unsafe extern "win64" fn CreateAudioVolumeMeter(ppv: *mut usize) -> i32 {
    ensure_vtbls_registered();
    if ppv.is_null() {
        return E_POINTER;
    }
    wr(ppv, 0);
    let obj = BuiltinXapoCom::alloc(BuiltinXapoKind::VolumeMeter(VolumeMeterDsp::new()));
    wr(ppv, obj as usize);
    S_OK
}

pub unsafe extern "win64" fn CreateFX(
    clsid: *const u8,
    ppv: *mut usize,
    init: usize,
    sz: u32,
) -> i32 {
    ensure_vtbls_registered();
    if ppv.is_null() {
        return E_POINTER;
    }
    wr(ppv, 0);
    if clsid.is_null() {
        return E_POINTER;
    }
    let guid = rd(clsid as *const GUID);
    let kind = if guid == CLSID_FX_EQ || guid == CLSID_FX_EQ_27 {
        BuiltinXapoKind::FxEq(FxEqDsp::new())
    } else if guid == CLSID_FX_MASTERING_LIMITER || guid == CLSID_FX_MASTERING_LIMITER_27 {
        BuiltinXapoKind::FxMasteringLimiter(FxMasteringLimiterDsp::new())
    } else if guid == CLSID_FX_REVERB || guid == CLSID_FX_REVERB_27 {
        let params = FxReverbParameters::default();
        let cfg = reverb_config_from_fxreverb(&params, 48_000);
        BuiltinXapoKind::FxReverb {
            params,
            dsp: SchroederReverbDsp::new(cfg),
        }
    } else if guid == CLSID_FX_ECHO || guid == CLSID_FX_ECHO_27 {
        BuiltinXapoKind::FxEcho(FxEchoDsp::new())
    } else {
        return REGDB_E_CLASSNOTREG;
    };

    let obj = BuiltinXapoCom::alloc(kind);
    if init != 0 || sz > 0 {
        let hr = xapo_initialize(obj, init as *const c_void, sz);
        if hr < 0 {
            xapo_release(obj);
            return hr;
        }
    }
    wr(ppv, obj as usize);
    S_OK
}

pub unsafe extern "win64" fn X3DAudioCalculate(
    instance: *const [u8; 20],
    p_listener: *const c_void,
    p_emitter: *const c_void,
    flags: u32,
    p_dsp_settings: *mut c_void,
) {
    x3daudio::x3daudio_calculate_impl(instance, p_listener, p_emitter, flags, p_dsp_settings);
}

#[cfg(test)]
mod tests;
