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

use crate::audio::backend::{self, Backend, OutputStream, StreamControl, StreamShared};
use crate::audio::com::{
    GUID, IID_IUnknown, KSDATAFORMAT_SUBTYPE_IEEE_FLOAT, KSDATAFORMAT_SUBTYPE_PCM,
};
use crate::audio::format::{
    self, SPEAKER_BACK_CENTER, SPEAKER_BACK_LEFT, SPEAKER_BACK_RIGHT, SPEAKER_FRONT_CENTER,
    SPEAKER_FRONT_LEFT, SPEAKER_FRONT_LEFT_OF_CENTER, SPEAKER_FRONT_RIGHT,
    SPEAKER_FRONT_RIGHT_OF_CENTER, SPEAKER_LOW_FREQUENCY, SPEAKER_SIDE_LEFT, SPEAKER_SIDE_RIGHT,
    WAVE_FORMAT_EXTENSIBLE, WAVE_FORMAT_IEEE_FLOAT, WAVE_FORMAT_PCM,
};
use crate::audio::ring::Ring;
use std::collections::{HashMap, VecDeque};
use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4, PI, TAU};
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{mpsc, Arc, Condvar, Mutex, Once, Weak};
use std::thread::ThreadId;
use std::time::Duration;

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

// ============================================================================
// Constants & HRESULTs
// ============================================================================

const S_OK: i32 = 0;
const E_NOINTERFACE: i32 = 0x8000_4002u32 as i32;
const E_POINTER: i32 = 0x8000_4003u32 as i32;
const E_OUTOFMEMORY: i32 = 0x8007_000Eu32 as i32;
const E_INVALIDARG: i32 = 0x8007_0057u32 as i32;
const REGDB_E_CLASSNOTREG: i32 = 0x8004_0154u32 as i32;
const XAUDIO2_E_INVALID_CALL: i32 = 0x8896_0001u32 as i32;
const XAPO_E_FORMAT_UNSUPPORTED: i32 = 0x8897_0001u32 as i32;

const IID_IXAUDIO2: GUID = GUID::new(
    0x2B02_E3CF,
    0x2E0B,
    0x4EC3,
    [0xBE, 0x45, 0x1B, 0x2A, 0x3F, 0xE7, 0x21, 0x0D],
);
const IID_IXAUDIO2_7: GUID = GUID::new(
    0x8BCF_1F58,
    0x9FE7,
    0x4583,
    [0x8A, 0xC6, 0xE2, 0xAD, 0xC4, 0x65, 0xC8, 0xBB],
);
const IID_IXAPO: GUID = GUID::new(
    0xA410_B984,
    0x9839,
    0x4819,
    [0xA0, 0xBE, 0x28, 0x56, 0xAE, 0x6B, 0x3A, 0xDB],
);
const IID_IXAPO_PARAMETERS: GUID = GUID::new(
    0x26D9_5C66,
    0x80F2,
    0x499A,
    [0xAD, 0x54, 0x5A, 0xE7, 0xF0, 0x1C, 0x6D, 0x98],
);

const CLSID_AUDIO_REVERB: GUID = GUID::new(
    0xC263_3B16,
    0x471B,
    0x4498,
    [0xB8, 0xC5, 0x4F, 0x09, 0x59, 0xE2, 0xEC, 0x09],
);
const CLSID_AUDIO_VOLUME_METER: GUID = GUID::new(
    0x4FC3_B166,
    0x972A,
    0x40CF,
    [0xBC, 0x37, 0x7D, 0xB0, 0x3D, 0xB2, 0xFB, 0xA3],
);
const CLSID_FX_EQ: GUID = GUID::new(
    0xF5E0_1117,
    0xD6C4,
    0x485A,
    [0xA3, 0xF5, 0x69, 0x51, 0x96, 0xF3, 0xDB, 0xFA],
);
const CLSID_FX_MASTERING_LIMITER: GUID = GUID::new(
    0xC413_7916,
    0x2BE1,
    0x46FD,
    [0x85, 0x99, 0x44, 0x15, 0x36, 0xF4, 0x98, 0x56],
);
const CLSID_FX_REVERB: GUID = GUID::new(
    0x7D9A_CA56,
    0xCB68,
    0x4807,
    [0xB6, 0x32, 0xB1, 0x37, 0x35, 0x2E, 0x85, 0x96],
);
const CLSID_FX_ECHO: GUID = GUID::new(
    0x5039_D740,
    0xF736,
    0x449A,
    [0x84, 0xD3, 0xA5, 0x62, 0x02, 0x55, 0xF4, 0x87],
);
// Legacy DirectX SDK (XAudio2.7 / XAPOFX1_5) CLSIDs.
const CLSID_FX_EQ_27: GUID = GUID::new(
    0xA90B_C001,
    0xE897,
    0xE897,
    [0x74, 0x39, 0x43, 0x55, 0x00, 0x00, 0x00, 0x00],
);
const CLSID_FX_MASTERING_LIMITER_27: GUID = GUID::new(
    0xA90B_C001,
    0xE897,
    0xE897,
    [0x74, 0x39, 0x43, 0x55, 0x00, 0x00, 0x00, 0x01],
);
const CLSID_FX_REVERB_27: GUID = GUID::new(
    0xA90B_C001,
    0xE897,
    0xE897,
    [0x74, 0x39, 0x43, 0x55, 0x00, 0x00, 0x00, 0x02],
);
const CLSID_FX_ECHO_27: GUID = GUID::new(
    0xA90B_C001,
    0xE897,
    0xE897,
    [0x74, 0x39, 0x43, 0x55, 0x00, 0x00, 0x00, 0x03],
);

const XAUDIO2_MAX_AUDIO_CHANNELS: u32 = 64;
const XAUDIO2_MIN_SAMPLE_RATE: u32 = 1_000;
const XAUDIO2_MAX_SAMPLE_RATE: u32 = 768_000;
const XAUDIO2_MAX_VOLUME_LEVEL: f32 = 16_777_216.0;
const XAUDIO2_MIN_FREQ_RATIO: f32 = 1.0 / 1024.0;
const XAUDIO2_MAX_FREQ_RATIO: f32 = 1024.0;
const XAUDIO2_DEFAULT_FREQ_RATIO: f32 = 2.0;
const XAUDIO2_MAX_QUEUED_BUFFERS: usize = 1024;

const XAUDIO2_VOICE_NOPITCH: u32 = 0x0002;
const XAUDIO2_PLAY_TAILS: u32 = 0x0020;
const XAUDIO2_END_OF_STREAM: u32 = 0x0040;
const XAUDIO2_VOICE_NOSAMPLESPLAYED: u32 = 0x0100;
const XAUDIO2_LOOP_INFINITE: u32 = 255;

const XAPO_REGISTRATION_STRING_LENGTH: usize = 256;
const XAPO_FLAG_CHANNELS_MUST_MATCH: u32 = 0x0000_0001;
const XAPO_FLAG_FRAMERATE_MUST_MATCH: u32 = 0x0000_0002;
const XAPO_FLAG_BITSPERSAMPLE_MUST_MATCH: u32 = 0x0000_0004;
const XAPO_FLAG_BUFFERCOUNT_MUST_MATCH: u32 = 0x0000_0008;
const XAPO_FLAG_INPLACE_SUPPORTED: u32 = 0x0000_0010;
const XAPOBASE_DEFAULT_FLAG: u32 = XAPO_FLAG_CHANNELS_MUST_MATCH
    | XAPO_FLAG_FRAMERATE_MUST_MATCH
    | XAPO_FLAG_BITSPERSAMPLE_MUST_MATCH
    | XAPO_FLAG_BUFFERCOUNT_MUST_MATCH
    | XAPO_FLAG_INPLACE_SUPPORTED;

const XAPO_BUFFER_SILENT: u32 = 0;
const XAPO_BUFFER_VALID: u32 = 1;

const FXEQ_MIN_FREQUENCY_CENTER: f32 = 20.0;
const FXEQ_MAX_FREQUENCY_CENTER: f32 = 20_000.0;
const FXEQ_MIN_GAIN: f32 = 0.126;
const FXEQ_MAX_GAIN: f32 = 7.94;
const FXEQ_MIN_BANDWIDTH: f32 = 0.1;
const FXEQ_MAX_BANDWIDTH: f32 = 2.0;

const FXMASTERINGLIMITER_MIN_RELEASE: u32 = 1;
const FXMASTERINGLIMITER_MAX_RELEASE: u32 = 20;
const FXMASTERINGLIMITER_DEFAULT_RELEASE: u32 = 6;
const FXMASTERINGLIMITER_MIN_LOUDNESS: u32 = 1;
const FXMASTERINGLIMITER_MAX_LOUDNESS: u32 = 1800;
const FXMASTERINGLIMITER_DEFAULT_LOUDNESS: u32 = 1000;

const FXREVERB_MIN_DIFFUSION: f32 = 0.0;
const FXREVERB_MAX_DIFFUSION: f32 = 1.0;
const FXREVERB_DEFAULT_DIFFUSION: f32 = 0.9;
const FXREVERB_MIN_ROOMSIZE: f32 = 0.0001;
const FXREVERB_MAX_ROOMSIZE: f32 = 1.0;
const FXREVERB_DEFAULT_ROOMSIZE: f32 = 0.6;

const FXECHO_MIN_WETDRYMIX: f32 = 0.0;
const FXECHO_MAX_WETDRYMIX: f32 = 1.0;
const FXECHO_DEFAULT_WETDRYMIX: f32 = 0.5;
const FXECHO_MIN_FEEDBACK: f32 = 0.0;
const FXECHO_MAX_FEEDBACK: f32 = 1.0;
const FXECHO_DEFAULT_FEEDBACK: f32 = 0.5;
const FXECHO_MIN_DELAY: f32 = 1.0;
const FXECHO_MAX_DELAY: f32 = 2000.0;
const FXECHO_DEFAULT_DELAY: f32 = 500.0;

const FILTER_LOW_PASS: u32 = 0;
const FILTER_BAND_PASS: u32 = 1;
const FILTER_HIGH_PASS: u32 = 2;
const FILTER_NOTCH: u32 = 3;
const FILTER_LOW_PASS_ONE_POLE: u32 = 4;
const FILTER_HIGH_PASS_ONE_POLE: u32 = 5;

const X3DAUDIO_HANDLE_MAGIC: u32 = 0x5833_4441; // "X3DA"
const X3DAUDIO_SPEED_OF_SOUND: f32 = 343.5;

const X3DAUDIO_CALCULATE_MATRIX: u32 = 0x0000_0001;
const X3DAUDIO_CALCULATE_DELAY: u32 = 0x0000_0002;
const X3DAUDIO_CALCULATE_LPF_DIRECT: u32 = 0x0000_0004;
const X3DAUDIO_CALCULATE_LPF_REVERB: u32 = 0x0000_0008;
const X3DAUDIO_CALCULATE_REVERB: u32 = 0x0000_0010;
const X3DAUDIO_CALCULATE_DOPPLER: u32 = 0x0000_0020;
const X3DAUDIO_CALCULATE_EMITTER_ANGLE: u32 = 0x0000_0040;
const X3DAUDIO_CALCULATE_ZEROCENTER: u32 = 0x0001_0000;
const X3DAUDIO_CALCULATE_REDIRECT_TO_LFE: u32 = 0x0002_0000;

// ============================================================================
// Windows ABI Structures (XAudio2 & X3DAudio headers use `#pragma pack(push, 1)`)
// ============================================================================

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct WaveFormatEx {
    format_tag: u16,
    channels: u16,
    samples_per_sec: u32,
    avg_bytes_per_sec: u32,
    block_align: u16,
    bits_per_sample: u16,
    cb_size: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct WaveFormatExtensible {
    format: WaveFormatEx,
    valid_bits_per_sample: u16,
    channel_mask: u32,
    sub_format: GUID,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
struct XAudio2VoiceDetails {
    creation_flags: u32,
    active_flags: u32,
    input_channels: u32,
    input_sample_rate: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct XAudio2SendDescriptor {
    flags: u32,
    output_voice: *mut VoiceHeader,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct XAudio2VoiceSends {
    send_count: u32,
    sends: *const XAudio2SendDescriptor,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct XAudio2EffectDescriptor {
    effect: *mut c_void,
    initial_state: i32,
    output_channels: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct XAudio2EffectChain {
    effect_count: u32,
    effects: *const XAudio2EffectDescriptor,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
struct XapoRegistrationProperties {
    clsid: GUID,
    friendly_name: [u16; XAPO_REGISTRATION_STRING_LENGTH],
    copyright_info: [u16; XAPO_REGISTRATION_STRING_LENGTH],
    major_version: u32,
    minor_version: u32,
    flags: u32,
    min_input_buffer_count: u32,
    max_input_buffer_count: u32,
    min_output_buffer_count: u32,
    max_output_buffer_count: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
struct XapoLockForProcessBufferParameters {
    format: *const WaveFormatEx,
    max_frame_count: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct XapoProcessBufferParameters {
    buffer: *mut c_void,
    buffer_flags: u32,
    valid_frame_count: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
struct XAudio2FxReverbParameters {
    wet_dry_mix: f32,
    reflections_delay: u32,
    reverb_delay: u8,
    rear_delay: u8,
    side_delay: u8,
    position_left: u8,
    position_right: u8,
    position_matrix_left: u8,
    position_matrix_right: u8,
    early_diffusion: u8,
    late_diffusion: u8,
    low_eq_gain: u8,
    low_eq_cutoff: u8,
    high_eq_gain: u8,
    high_eq_cutoff: u8,
    room_filter_freq: f32,
    room_filter_main: f32,
    room_filter_hf: f32,
    reflections_gain: f32,
    reverb_gain: f32,
    decay_time: f32,
    density: f32,
    room_size: f32,
    disable_late_field: i32,
}

impl Default for XAudio2FxReverbParameters {
    fn default() -> Self {
        Self {
            wet_dry_mix: 100.0,
            reflections_delay: 5,
            reverb_delay: 5,
            rear_delay: 5,
            side_delay: 5,
            position_left: 6,
            position_right: 6,
            position_matrix_left: 27,
            position_matrix_right: 27,
            early_diffusion: 8,
            late_diffusion: 8,
            low_eq_gain: 8,
            low_eq_cutoff: 4,
            high_eq_gain: 8,
            high_eq_cutoff: 4,
            room_filter_freq: 5000.0,
            room_filter_main: 0.0,
            room_filter_hf: 0.0,
            reflections_gain: 0.0,
            reverb_gain: 0.0,
            decay_time: 1.0,
            density: 100.0,
            room_size: 100.0,
            disable_late_field: 0,
        }
    }
}

/// XAudio 2.8 layout (no `side_delay`, has `disable_late_field`).
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct XAudio2FxReverbParameters28 {
    wet_dry_mix: f32,
    reflections_delay: u32,
    reverb_delay: u8,
    rear_delay: u8,
    position_left: u8,
    position_right: u8,
    position_matrix_left: u8,
    position_matrix_right: u8,
    early_diffusion: u8,
    late_diffusion: u8,
    low_eq_gain: u8,
    low_eq_cutoff: u8,
    high_eq_gain: u8,
    high_eq_cutoff: u8,
    room_filter_freq: f32,
    room_filter_main: f32,
    room_filter_hf: f32,
    reflections_gain: f32,
    reverb_gain: f32,
    decay_time: f32,
    density: f32,
    room_size: f32,
    disable_late_field: i32,
}

/// XAudio 2.7 layout (no `side_delay` and no `disable_late_field`).
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct XAudio2FxReverbParameters27 {
    wet_dry_mix: f32,
    reflections_delay: u32,
    reverb_delay: u8,
    rear_delay: u8,
    position_left: u8,
    position_right: u8,
    position_matrix_left: u8,
    position_matrix_right: u8,
    early_diffusion: u8,
    late_diffusion: u8,
    low_eq_gain: u8,
    low_eq_cutoff: u8,
    high_eq_gain: u8,
    high_eq_cutoff: u8,
    room_filter_freq: f32,
    room_filter_main: f32,
    room_filter_hf: f32,
    reflections_gain: f32,
    reverb_gain: f32,
    decay_time: f32,
    density: f32,
    room_size: f32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct XAudio2FxVolumeMeterLevels {
    peak_levels: *mut f32,
    rms_levels: *mut f32,
    channel_count: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
struct FxEqParameters {
    frequency_center0: f32,
    gain0: f32,
    bandwidth0: f32,
    frequency_center1: f32,
    gain1: f32,
    bandwidth1: f32,
    frequency_center2: f32,
    gain2: f32,
    bandwidth2: f32,
    frequency_center3: f32,
    gain3: f32,
    bandwidth3: f32,
}

impl Default for FxEqParameters {
    fn default() -> Self {
        Self {
            frequency_center0: 100.0,
            gain0: 1.0,
            bandwidth0: 1.0,
            frequency_center1: 800.0,
            gain1: 1.0,
            bandwidth1: 1.0,
            frequency_center2: 2000.0,
            gain2: 1.0,
            bandwidth2: 1.0,
            frequency_center3: 10_000.0,
            gain3: 1.0,
            bandwidth3: 1.0,
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
struct FxMasteringLimiterParameters {
    release: u32,
    loudness: u32,
}

impl Default for FxMasteringLimiterParameters {
    fn default() -> Self {
        Self {
            release: FXMASTERINGLIMITER_DEFAULT_RELEASE,
            loudness: FXMASTERINGLIMITER_DEFAULT_LOUDNESS,
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
struct FxReverbParameters {
    diffusion: f32,
    room_size: f32,
}

impl Default for FxReverbParameters {
    fn default() -> Self {
        Self {
            diffusion: FXREVERB_DEFAULT_DIFFUSION,
            room_size: FXREVERB_DEFAULT_ROOMSIZE,
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
struct FxEchoParameters {
    wet_dry_mix: f32,
    feedback: f32,
    delay: f32,
}

impl Default for FxEchoParameters {
    fn default() -> Self {
        Self {
            wet_dry_mix: FXECHO_DEFAULT_WETDRYMIX,
            feedback: FXECHO_DEFAULT_FEEDBACK,
            delay: FXECHO_DEFAULT_DELAY,
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
struct FxEchoInitData {
    max_delay: f32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
struct XAudio2FilterParameters {
    filter_type: u32,
    frequency: f32,
    one_over_q: f32,
}

impl Default for XAudio2FilterParameters {
    fn default() -> Self {
        Self {
            filter_type: FILTER_LOW_PASS,
            frequency: 1.0,
            one_over_q: 1.0,
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct XAudio2Buffer {
    flags: u32,
    audio_bytes: u32,
    audio_data: *const u8,
    play_begin: u32,
    play_length: u32,
    loop_begin: u32,
    loop_length: u32,
    loop_count: u32,
    context: usize,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
struct XAudio2VoiceState {
    current_buffer_context: usize,
    buffers_queued: u32,
    samples_played: u64,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Default)]
#[allow(dead_code)]
struct XAudio2PerformanceData {
    audio_cycles_since_last_query: u64,
    total_cycles_since_last_query: u64,
    minimum_cycles_per_quantum: u32,
    maximum_cycles_per_quantum: u32,
    memory_usage_in_bytes: u32,
    current_latency_in_samples: u32,
    glitches_since_engine_started: u32,
    active_source_voice_count: u32,
    total_source_voice_count: u32,
    active_submix_voice_count: u32,
    active_resampler_count: u32,
    active_matrix_mix_count: u32,
    active_xma_source_voices: u32,
    active_xma_streams: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Default)]
struct X3DAudioVector {
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct X3DAudioDistanceCurvePoint {
    distance: f32,
    dsp_setting: f32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct X3DAudioDistanceCurve {
    points: *const X3DAudioDistanceCurvePoint,
    point_count: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct X3DAudioCone {
    inner_angle: f32,
    outer_angle: f32,
    inner_volume: f32,
    outer_volume: f32,
    inner_lpf: f32,
    outer_lpf: f32,
    inner_reverb: f32,
    outer_reverb: f32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct X3DAudioListener {
    orient_front: X3DAudioVector,
    orient_top: X3DAudioVector,
    position: X3DAudioVector,
    velocity: X3DAudioVector,
    cone: *const X3DAudioCone,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct X3DAudioEmitter {
    cone: *const X3DAudioCone,
    orient_front: X3DAudioVector,
    orient_top: X3DAudioVector,
    position: X3DAudioVector,
    velocity: X3DAudioVector,
    inner_radius: f32,
    inner_radius_angle: f32,
    channel_count: u32,
    channel_radius: f32,
    channel_azimuths: *const f32,
    volume_curve: *const X3DAudioDistanceCurve,
    lfe_curve: *const X3DAudioDistanceCurve,
    lpf_direct_curve: *const X3DAudioDistanceCurve,
    lpf_reverb_curve: *const X3DAudioDistanceCurve,
    reverb_curve: *const X3DAudioDistanceCurve,
    curve_distance_scaler: f32,
    doppler_scaler: f32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
struct X3DAudioDspSettings {
    matrix_coefficients: *mut f32,
    delay_times: *mut f32,
    src_channel_count: u32,
    dst_channel_count: u32,
    lpf_direct_coefficient: f32,
    lpf_reverb_coefficient: f32,
    reverb_level: f32,
    doppler_factor: f32,
    emitter_to_listener_angle: f32,
    emitter_to_listener_distance: f32,
    emitter_velocity_component: f32,
    listener_velocity_component: f32,
}

const _: () = assert!(std::mem::size_of::<WaveFormatEx>() == 18);
const _: () = assert!(std::mem::size_of::<WaveFormatExtensible>() == 40);
const _: () = assert!(std::mem::size_of::<XAudio2VoiceDetails>() == 16);
const _: () = assert!(std::mem::size_of::<XAudio2SendDescriptor>() == 12);
const _: () = assert!(std::mem::size_of::<XAudio2VoiceSends>() == 12);
const _: () = assert!(std::mem::size_of::<XAudio2EffectDescriptor>() == 16);
const _: () = assert!(std::mem::size_of::<XAudio2EffectChain>() == 12);
const _: () = assert!(std::mem::size_of::<XapoRegistrationProperties>() == 1068);
const _: () = assert!(std::mem::size_of::<XapoLockForProcessBufferParameters>() == 12);
const _: () = assert!(std::mem::size_of::<XapoProcessBufferParameters>() == 16);
const _: () = assert!(std::mem::size_of::<XAudio2FxReverbParameters>() == 57);
const _: () = assert!(std::mem::size_of::<XAudio2FxReverbParameters28>() == 56);
const _: () = assert!(std::mem::size_of::<XAudio2FxReverbParameters27>() == 52);
const _: () = assert!(std::mem::size_of::<XAudio2FxVolumeMeterLevels>() == 20);
const _: () = assert!(std::mem::size_of::<FxEqParameters>() == 48);
const _: () = assert!(std::mem::size_of::<FxMasteringLimiterParameters>() == 8);
const _: () = assert!(std::mem::size_of::<FxReverbParameters>() == 8);
const _: () = assert!(std::mem::size_of::<FxEchoParameters>() == 12);
const _: () = assert!(std::mem::size_of::<FxEchoInitData>() == 4);
const _: () = assert!(std::mem::size_of::<XAudio2FilterParameters>() == 12);
const _: () = assert!(std::mem::size_of::<XAudio2Buffer>() == 44);
const _: () = assert!(std::mem::size_of::<XAudio2VoiceState>() == 20);
const _: () = assert!(std::mem::size_of::<XAudio2PerformanceData>() == 64);
const _: () = assert!(std::mem::size_of::<X3DAudioVector>() == 12);
const _: () = assert!(std::mem::size_of::<X3DAudioDistanceCurvePoint>() == 8);
const _: () = assert!(std::mem::size_of::<X3DAudioDistanceCurve>() == 12);
const _: () = assert!(std::mem::size_of::<X3DAudioCone>() == 32);
const _: () = assert!(std::mem::size_of::<X3DAudioListener>() == 56);
const _: () = assert!(std::mem::size_of::<X3DAudioEmitter>() == 128);
const _: () = assert!(std::mem::size_of::<X3DAudioDspSettings>() == 56);

// ============================================================================
// COM Vtables
// ============================================================================

#[repr(C, align(16))]
struct Vtable<const N: usize>([crate::Ptr; N]);
unsafe impl<const N: usize> Sync for Vtable<N> {}

static XAUDIO2_VTBL: Vtable<13> = Vtable([
    crate::Ptr(xa2_query_interface as *const ()),               // 0 (+0x00)
    crate::Ptr(xa2_add_ref as *const ()),                       // 1 (+0x08)
    crate::Ptr(xa2_release as *const ()),                       // 2 (+0x10)
    crate::Ptr(xa2_register_callbacks as *const ()),            // 3 (+0x18)
    crate::Ptr(xa2_unregister_callbacks as *const ()),          // 4 (+0x20)
    crate::Ptr(crate::bridge_entry!(xa2_create_source_voice)),  // 5 (+0x28)
    crate::Ptr(xa2_create_submix_voice as *const ()),           // 6 (+0x30)
    crate::Ptr(xa2_create_mastering_voice as *const ()),        // 7 (+0x38)
    crate::Ptr(xa2_start_engine as *const ()),                  // 8 (+0x40)
    crate::Ptr(xa2_stop_engine as *const ()),                   // 9 (+0x48)
    crate::Ptr(xa2_commit_changes as *const ()),                // 10 (+0x50)
    crate::Ptr(xa2_get_perf_data as *const ()),                 // 11 (+0x58)
    crate::Ptr(xa2_set_debug_config as *const ()),              // 12 (+0x60)
]);

static MASTER_VOICE_VTBL: Vtable<20> = Vtable([
    crate::Ptr(voice_get_voice_details as *const ()),           // 0 (+0x00)
    crate::Ptr(voice_set_output_voices as *const ()),           // 1 (+0x08)
    crate::Ptr(voice_set_effect_chain as *const ()),            // 2 (+0x10)
    crate::Ptr(voice_enable_effect as *const ()),               // 3 (+0x18)
    crate::Ptr(voice_disable_effect as *const ()),              // 4 (+0x20)
    crate::Ptr(voice_get_effect_state as *const ()),            // 5 (+0x28)
    crate::Ptr(voice_set_effect_parameters as *const ()),       // 6 (+0x30)
    crate::Ptr(voice_get_effect_parameters as *const ()),       // 7 (+0x38)
    crate::Ptr(voice_set_filter_parameters as *const ()),       // 8 (+0x40)
    crate::Ptr(voice_get_filter_parameters as *const ()),       // 9 (+0x48)
    crate::Ptr(voice_set_output_filter_parameters as *const ()),// 10 (+0x50)
    crate::Ptr(voice_get_output_filter_parameters as *const ()),// 11 (+0x58)
    crate::Ptr(crate::bridge_entry!(voice_set_volume)),         // 12 (+0x60)
    crate::Ptr(voice_get_volume as *const ()),                  // 13 (+0x68)
    crate::Ptr(voice_set_channel_volumes as *const ()),         // 14 (+0x70)
    crate::Ptr(voice_get_channel_volumes as *const ()),         // 15 (+0x78)
    crate::Ptr(voice_set_output_matrix as *const ()),           // 16 (+0x80)
    crate::Ptr(voice_get_output_matrix as *const ()),           // 17 (+0x88)
    crate::Ptr(voice_destroy_voice as *const ()),               // 18 (+0x90)
    crate::Ptr(mv_get_channel_mask as *const ()),               // 19 (+0x98)
]);

static SUBMIX_VOICE_VTBL: Vtable<19> = Vtable([
    crate::Ptr(voice_get_voice_details as *const ()),           // 0 (+0x00)
    crate::Ptr(voice_set_output_voices as *const ()),           // 1 (+0x08)
    crate::Ptr(voice_set_effect_chain as *const ()),            // 2 (+0x10)
    crate::Ptr(voice_enable_effect as *const ()),               // 3 (+0x18)
    crate::Ptr(voice_disable_effect as *const ()),              // 4 (+0x20)
    crate::Ptr(voice_get_effect_state as *const ()),            // 5 (+0x28)
    crate::Ptr(voice_set_effect_parameters as *const ()),       // 6 (+0x30)
    crate::Ptr(voice_get_effect_parameters as *const ()),       // 7 (+0x38)
    crate::Ptr(voice_set_filter_parameters as *const ()),       // 8 (+0x40)
    crate::Ptr(voice_get_filter_parameters as *const ()),       // 9 (+0x48)
    crate::Ptr(voice_set_output_filter_parameters as *const ()),// 10 (+0x50)
    crate::Ptr(voice_get_output_filter_parameters as *const ()),// 11 (+0x58)
    crate::Ptr(crate::bridge_entry!(voice_set_volume)),         // 12 (+0x60)
    crate::Ptr(voice_get_volume as *const ()),                  // 13 (+0x68)
    crate::Ptr(voice_set_channel_volumes as *const ()),         // 14 (+0x70)
    crate::Ptr(voice_get_channel_volumes as *const ()),         // 15 (+0x78)
    crate::Ptr(voice_set_output_matrix as *const ()),           // 16 (+0x80)
    crate::Ptr(voice_get_output_matrix as *const ()),           // 17 (+0x88)
    crate::Ptr(voice_destroy_voice as *const ()),               // 18 (+0x90)
]);

static SOURCE_VOICE_VTBL: Vtable<29> = Vtable([
    crate::Ptr(voice_get_voice_details as *const ()),           // 0 (+0x00)
    crate::Ptr(voice_set_output_voices as *const ()),           // 1 (+0x08)
    crate::Ptr(voice_set_effect_chain as *const ()),            // 2 (+0x10)
    crate::Ptr(voice_enable_effect as *const ()),               // 3 (+0x18)
    crate::Ptr(voice_disable_effect as *const ()),              // 4 (+0x20)
    crate::Ptr(voice_get_effect_state as *const ()),            // 5 (+0x28)
    crate::Ptr(voice_set_effect_parameters as *const ()),       // 6 (+0x30)
    crate::Ptr(voice_get_effect_parameters as *const ()),       // 7 (+0x38)
    crate::Ptr(voice_set_filter_parameters as *const ()),       // 8 (+0x40)
    crate::Ptr(voice_get_filter_parameters as *const ()),       // 9 (+0x48)
    crate::Ptr(voice_set_output_filter_parameters as *const ()),// 10 (+0x50)
    crate::Ptr(voice_get_output_filter_parameters as *const ()),// 11 (+0x58)
    crate::Ptr(crate::bridge_entry!(voice_set_volume)),         // 12 (+0x60)
    crate::Ptr(voice_get_volume as *const ()),                  // 13 (+0x68)
    crate::Ptr(voice_set_channel_volumes as *const ()),         // 14 (+0x70)
    crate::Ptr(voice_get_channel_volumes as *const ()),         // 15 (+0x78)
    crate::Ptr(voice_set_output_matrix as *const ()),           // 16 (+0x80)
    crate::Ptr(voice_get_output_matrix as *const ()),           // 17 (+0x88)
    crate::Ptr(voice_destroy_voice as *const ()),               // 18 (+0x90)
    crate::Ptr(sv_start as *const ()),                          // 19 (+0x98)
    crate::Ptr(sv_stop as *const ()),                           // 20 (+0xa0)
    crate::Ptr(sv_submit_source_buffer as *const ()),           // 21 (+0xa8)
    crate::Ptr(sv_flush_source_buffers as *const ()),           // 22 (+0xb0)
    crate::Ptr(sv_discontinuity as *const ()),                  // 23 (+0xb8)
    crate::Ptr(sv_exit_loop as *const ()),                      // 24 (+0xc0)
    crate::Ptr(sv_get_state as *const ()),                      // 25 (+0xc8)
    crate::Ptr(crate::bridge_entry!(sv_set_freq_ratio)),        // 26 (+0xd0)
    crate::Ptr(sv_get_freq_ratio as *const ()),                 // 27 (+0xd8)
    crate::Ptr(sv_set_source_sample_rate as *const ()),         // 28 (+0xe0)
]);

static XAPO_VTBL: Vtable<13> = Vtable([
    crate::Ptr(xapo_query_interface as *const ()),            // 0 (+0x00)
    crate::Ptr(xapo_add_ref as *const ()),                    // 1 (+0x08)
    crate::Ptr(xapo_release as *const ()),                    // 2 (+0x10)
    crate::Ptr(xapo_get_registration_properties as *const ()),// 3 (+0x18)
    crate::Ptr(xapo_is_input_format_supported as *const ()),  // 4 (+0x20)
    crate::Ptr(xapo_is_output_format_supported as *const ()), // 5 (+0x28)
    crate::Ptr(xapo_initialize as *const ()),                 // 6 (+0x30)
    crate::Ptr(xapo_reset as *const ()),                      // 7 (+0x38)
    crate::Ptr(xapo_lock_for_process as *const ()),           // 8 (+0x40)
    crate::Ptr(xapo_unlock_for_process as *const ()),         // 9 (+0x48)
    crate::Ptr(xapo_process as *const ()),                    // 10 (+0x50)
    crate::Ptr(xapo_calc_input_frames as *const ()),          // 11 (+0x58)
    crate::Ptr(xapo_calc_output_frames as *const ()),         // 12 (+0x60)
]);

static XAPO_PARAMS_VTBL: Vtable<5> = Vtable([
    crate::Ptr(xapo_params_query_interface as *const ()),     // 0 (+0x00)
    crate::Ptr(xapo_params_add_ref as *const ()),             // 1 (+0x08)
    crate::Ptr(xapo_params_release as *const ()),             // 2 (+0x10)
    crate::Ptr(xapo_params_set_parameters as *const ()),      // 3 (+0x18)
    crate::Ptr(xapo_params_get_parameters as *const ()),      // 4 (+0x20)
]);

fn ensure_vtbls_registered() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        for p in &XAUDIO2_VTBL.0 {
            let addr = p.0 as usize;
            crate::register_native_va(addr as u64, addr);
        }
        for p in &MASTER_VOICE_VTBL.0 {
            let addr = p.0 as usize;
            crate::register_native_va(addr as u64, addr);
        }
        for p in &SUBMIX_VOICE_VTBL.0 {
            let addr = p.0 as usize;
            crate::register_native_va(addr as u64, addr);
        }
        for p in &SOURCE_VOICE_VTBL.0 {
            let addr = p.0 as usize;
            crate::register_native_va(addr as u64, addr);
        }
        for p in &XAPO_VTBL.0 {
            let addr = p.0 as usize;
            crate::register_native_va(addr as u64, addr);
        }
        for p in &XAPO_PARAMS_VTBL.0 {
            let addr = p.0 as usize;
            crate::register_native_va(addr as u64, addr);
        }
    });
}

// ============================================================================
// PCM Format & Sample Decoding
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PcmSampleKind {
    U8,
    I16,
    I24,
    I32,
    F32,
    F64,
}

#[derive(Clone, Copy, Debug)]
struct ParsedWaveFormat {
    channels: usize,
    sample_rate: u32,
    block_align: usize,
    sample_kind: PcmSampleKind,
}

unsafe fn parse_wave_format(ptr: *const WaveFormatEx) -> Result<ParsedWaveFormat, i32> {
    if ptr.is_null() {
        return Err(E_POINTER);
    }
    let wfx = std::ptr::read_unaligned(ptr);
    let ch = wfx.channels as u32;
    let sr = wfx.samples_per_sec;
    let bits = wfx.bits_per_sample;
    let cb_size = wfx.cb_size;
    let raw_tag = wfx.format_tag;
    let _avg_bps = wfx.avg_bytes_per_sec;

    if ch == 0 || ch > XAUDIO2_MAX_AUDIO_CHANNELS {
        return Err(XAUDIO2_E_INVALID_CALL);
    }
    if !(XAUDIO2_MIN_SAMPLE_RATE..=XAUDIO2_MAX_SAMPLE_RATE).contains(&sr) {
        return Err(XAUDIO2_E_INVALID_CALL);
    }

    let tag = if raw_tag == WAVE_FORMAT_EXTENSIBLE {
        if cb_size < 22 {
            return Err(XAUDIO2_E_INVALID_CALL);
        }
        let ext = std::ptr::read_unaligned(ptr as *const WaveFormatExtensible);
        let sub = { ext.sub_format };
        let valid_bits = ext.valid_bits_per_sample;
        let _mask = ext.channel_mask;
        let _fmt_copy = ext.format;
        if valid_bits > bits {
            return Err(XAUDIO2_E_INVALID_CALL);
        }
        if sub == KSDATAFORMAT_SUBTYPE_PCM {
            WAVE_FORMAT_PCM
        } else if sub == KSDATAFORMAT_SUBTYPE_IEEE_FLOAT {
            WAVE_FORMAT_IEEE_FLOAT
        } else {
            crate::stub!("XAudio2 CreateSourceVoice: unsupported WAVE_FORMAT_EXTENSIBLE SubFormat {sub:?}");
            return Err(XAUDIO2_E_INVALID_CALL);
        }
    } else {
        raw_tag
    };

    let sample_kind = match (tag, bits) {
        (WAVE_FORMAT_PCM, 8) => PcmSampleKind::U8,
        (WAVE_FORMAT_PCM, 16) => PcmSampleKind::I16,
        (WAVE_FORMAT_PCM, 24) => PcmSampleKind::I24,
        (WAVE_FORMAT_PCM, 32) => PcmSampleKind::I32,
        (WAVE_FORMAT_IEEE_FLOAT, 32) => PcmSampleKind::F32,
        (WAVE_FORMAT_IEEE_FLOAT, 64) => PcmSampleKind::F64,
        _ => {
            crate::stub!("XAudio2 CreateSourceVoice: unsupported format tag {tag:#x} bits {bits}");
            return Err(XAUDIO2_E_INVALID_CALL);
        }
    };

    let expected_align = (ch as usize) * ((bits as usize) / 8);
    let block_align = if wfx.block_align == 0 {
        expected_align
    } else if (wfx.block_align as usize) != expected_align {
        return Err(XAUDIO2_E_INVALID_CALL);
    } else {
        wfx.block_align as usize
    };

    Ok(ParsedWaveFormat {
        channels: ch as usize,
        sample_rate: sr,
        block_align,
        sample_kind,
    })
}

fn decode_pcm_to_f32(fmt: ParsedWaveFormat, raw: &[u8]) -> (Vec<f32>, usize) {
    let frames = raw.len() / fmt.block_align.max(1);
    let total_samples = frames * fmt.channels;
    let mut out = Vec::with_capacity(total_samples);
    match fmt.sample_kind {
        PcmSampleKind::U8 => {
            for &b in &raw[..total_samples] {
                out.push((b as f32 - 128.0) / 128.0);
            }
        }
        PcmSampleKind::I16 => {
            for c in raw[..total_samples * 2].chunks_exact(2) {
                let s = i16::from_le_bytes([c[0], c[1]]) as f32 / 32768.0;
                out.push(s);
            }
        }
        PcmSampleKind::I24 => {
            for c in raw[..total_samples * 3].chunks_exact(3) {
                let s = (i32::from_le_bytes([0, c[0], c[1], c[2]]) >> 8) as f32 / 8_388_608.0;
                out.push(s);
            }
        }
        PcmSampleKind::I32 => {
            for c in raw[..total_samples * 4].chunks_exact(4) {
                let s = i32::from_le_bytes([c[0], c[1], c[2], c[3]]) as f32 / 2_147_483_648.0;
                out.push(s);
            }
        }
        PcmSampleKind::F32 => {
            for c in raw[..total_samples * 4].chunks_exact(4) {
                let v = f32::from_le_bytes([c[0], c[1], c[2], c[3]]);
                out.push(if v.is_finite() { v } else { 0.0 });
            }
        }
        PcmSampleKind::F64 => {
            for c in raw[..total_samples * 8].chunks_exact(8) {
                let v = f64::from_le_bytes([c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]]) as f32;
                out.push(if v.is_finite() { v } else { 0.0 });
            }
        }
    }
    (out, frames)
}

// ============================================================================
// Voice Filter (Cytomic TPT State-Variable Filter + 1-Pole)
// ============================================================================

#[derive(Clone, Copy, Default)]
struct ChannelFilterState {
    ic1eq: f32,
    ic2eq: f32,
    one_pole: f32,
}

#[derive(Clone)]
struct VoiceFilter {
    params: XAudio2FilterParameters,
    active: bool,
    channels: Vec<ChannelFilterState>,
}

impl VoiceFilter {
    fn new(channel_count: usize) -> Self {
        Self {
            params: XAudio2FilterParameters::default(),
            active: false,
            channels: vec![ChannelFilterState::default(); channel_count.max(1)],
        }
    }

    fn set(&mut self, raw: XAudio2FilterParameters) -> Result<(), i32> {
        let ftype = raw.filter_type;
        let freq = raw.frequency;
        let ooq = raw.one_over_q;
        if ftype > FILTER_HIGH_PASS_ONE_POLE || !freq.is_finite() || !ooq.is_finite() {
            return Err(XAUDIO2_E_INVALID_CALL);
        }
        let clamped_freq = freq.clamp(0.0, 1.0);
        let clamped_ooq = if ftype == FILTER_LOW_PASS_ONE_POLE || ftype == FILTER_HIGH_PASS_ONE_POLE {
            ooq.clamp(0.0, 1.5)
        } else if ooq <= 0.0 {
            return Err(XAUDIO2_E_INVALID_CALL);
        } else {
            ooq.clamp(0.001, 1.5)
        };
        self.params = XAudio2FilterParameters {
            filter_type: ftype,
            frequency: clamped_freq,
            one_over_q: clamped_ooq,
        };
        let is_bypass = (ftype == FILTER_LOW_PASS
            && clamped_freq >= 1.0
            && (clamped_ooq - 1.0).abs() < 1e-6)
            || (ftype == FILTER_LOW_PASS_ONE_POLE && clamped_freq >= 1.0);
        self.active = !is_bypass;
        Ok(())
    }

    fn ensure_channels(&mut self, n: usize) {
        if self.channels.len() != n {
            self.channels.resize(n.max(1), ChannelFilterState::default());
        }
    }

    /// Process interleaved `samples` (`frames * ch`) in-place.
    /// `rate_scale` is `voice_rate / master_rate` when a voice's nominal rate differs from the
    /// mixer rate (1.0 in the common case).
    fn process(&mut self, samples: &mut [f32], ch: usize, rate_scale: f32) {
        if !self.active || ch == 0 || samples.is_empty() {
            return;
        }
        self.ensure_channels(ch);
        let ftype = self.params.filter_type;
        let freq = self.params.frequency;
        let k = self.params.one_over_q.clamp(0.001, 1.5);

        if ftype == FILTER_LOW_PASS_ONE_POLE || ftype == FILTER_HIGH_PASS_ONE_POLE {
            let a = (freq * rate_scale).clamp(0.0, 1.0);
            for frame in samples.chunks_exact_mut(ch) {
                for (c, s) in frame.iter_mut().enumerate() {
                    let st = &mut self.channels[c];
                    let v0 = *s;
                    st.one_pole += a * (v0 - st.one_pole);
                    if st.one_pole.abs() < 1e-20 {
                        st.one_pole = 0.0;
                    }
                    *s = if ftype == FILTER_LOW_PASS_ONE_POLE {
                        st.one_pole
                    } else {
                        v0 - st.one_pole
                    };
                }
            }
            return;
        }

        // XAudio2 `Frequency = 2 * sin(pi * fc / fs)` in [0, 1].
        let sin_half = (freq * 0.5).clamp(0.0, 0.999);
        let omega_half = (sin_half.asin() * rate_scale).clamp(0.0, FRAC_PI_2 - 1e-4);
        let g = omega_half.tan();
        let a1 = 1.0 / (1.0 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;

        for frame in samples.chunks_exact_mut(ch) {
            for (c, s) in frame.iter_mut().enumerate() {
                let st = &mut self.channels[c];
                let v0 = *s;
                let v3 = v0 - st.ic2eq;
                let v1 = a1 * st.ic1eq + a2 * v3;
                let v2 = st.ic2eq + a2 * st.ic1eq + a3 * v3;
                st.ic1eq = 2.0 * v1 - st.ic1eq;
                st.ic2eq = 2.0 * v2 - st.ic2eq;
                if st.ic1eq.abs() < 1e-20 {
                    st.ic1eq = 0.0;
                }
                if st.ic2eq.abs() < 1e-20 {
                    st.ic2eq = 0.0;
                }
                *s = match ftype {
                    FILTER_LOW_PASS => v2,
                    FILTER_BAND_PASS => k * v1,
                    FILTER_HIGH_PASS => v0 - k * v1 - v2,
                    FILTER_NOTCH => v0 - k * v1,
                    _ => v0,
                };
            }
        }
    }
}

// ============================================================================
// Send Matrix Helpers (XAudio2 uses destination-major `matrix[dst * src_ch + src]`)
// ============================================================================

/// Build default XAudio2 destination-major `[dst * src_ch + src]` routing matrix.
fn default_xaudio2_matrix(src_ch: usize, dst_ch: usize) -> Vec<f32> {
    if src_ch == 0 || dst_ch == 0 {
        return Vec::new();
    }
    if src_ch == dst_ch {
        let mut m = vec![0.0f32; src_ch * dst_ch];
        for i in 0..src_ch {
            m[i * src_ch + i] = 1.0;
        }
        return m;
    }
    let src_u16 = src_ch.min(32) as u16;
    let dst_u16 = dst_ch.min(32) as u16;
    let row_major = format::channel_matrix(
        src_u16,
        format::default_mask(src_u16),
        dst_u16,
        format::default_mask(dst_u16),
    );
    // Transpose `[src * dst_u16 + dst]` into XAudio2's `[dst * src_ch + src]`.
    let mut m = vec![0.0f32; src_ch * dst_ch];
    for s in 0..(src_u16 as usize) {
        for d in 0..(dst_u16 as usize) {
            m[d * src_ch + s] = row_major[s * (dst_u16 as usize) + d];
        }
    }
    m
}

#[derive(Clone)]
struct VoiceSend {
    dest_id: u64,
    dest_channels: usize,
    matrix: Vec<f32>,
    output_filter: VoiceFilter,
}

impl VoiceSend {
    fn new(dest_id: u64, src_ch: usize, dest_ch: usize) -> Self {
        Self {
            dest_id,
            dest_channels: dest_ch,
            matrix: default_xaudio2_matrix(src_ch, dest_ch),
            output_filter: VoiceFilter::new(dest_ch),
        }
    }
}

/// Mix `src` (`frames * src_ch`) through XAudio2 destination-major `send.matrix` (`dst_ch * src_ch`),
/// optionally applying `send.output_filter`, and accumulate into `dst` (`frames * dst_ch`).
fn accumulate_send(
    send: &mut VoiceSend,
    src: &[f32],
    src_ch: usize,
    dst: &mut [f32],
    dst_ch: usize,
    frames: usize,
    scratch: &mut Vec<f32>,
) {
    if src_ch == 0 || dst_ch == 0 || frames == 0 {
        return;
    }
    if send.matrix.len() != src_ch * dst_ch {
        send.matrix = default_xaudio2_matrix(src_ch, dst_ch);
    }
    if send.output_filter.active {
        let total = frames * dst_ch;
        scratch.clear();
        scratch.resize(total, 0.0);
        for f in 0..frames {
            let s_frame = &src[f * src_ch..(f + 1) * src_ch];
            let d_frame = &mut scratch[f * dst_ch..(f + 1) * dst_ch];
            for d in 0..dst_ch {
                let row = &send.matrix[d * src_ch..(d + 1) * src_ch];
                let mut acc = 0.0f32;
                for s in 0..src_ch {
                    acc += s_frame[s] * row[s];
                }
                d_frame[d] = acc;
            }
        }
        send.output_filter.process(scratch, dst_ch, 1.0);
        for (o, &v) in dst[..total].iter_mut().zip(scratch.iter()) {
            *o += v;
        }
    } else {
        for f in 0..frames {
            let s_frame = &src[f * src_ch..(f + 1) * src_ch];
            let d_frame = &mut dst[f * dst_ch..(f + 1) * dst_ch];
            for d in 0..dst_ch {
                let row = &send.matrix[d * src_ch..(d + 1) * src_ch];
                let mut acc = 0.0f32;
                for s in 0..src_ch {
                    acc += s_frame[s] * row[s];
                }
                d_frame[d] += acc;
            }
        }
    }
}

// ============================================================================
// Guest Callback Dispatch (Dedicated Win32 Thread — Never on CoreAudio IO Thread)
// ============================================================================

#[derive(Clone)]
enum CallbackEvent {
    EnginePassStart(Vec<usize>),
    EnginePassEnd(Vec<usize>),
    VoiceBufferStart {
        cb: usize,
        alive: Arc<AtomicBool>,
        ctx: usize,
    },
    VoiceBufferEnd {
        cb: usize,
        alive: Arc<AtomicBool>,
        ctx: usize,
    },
    VoiceLoopEnd {
        cb: usize,
        alive: Arc<AtomicBool>,
        ctx: usize,
    },
    VoiceStreamEnd {
        cb: usize,
        alive: Arc<AtomicBool>,
    },
}

enum CallbackMsg {
    Events(Vec<CallbackEvent>),
    Barrier(mpsc::SyncSender<()>),
}

struct CallbackDispatcher {
    tx: mpsc::Sender<CallbackMsg>,
    thread_id: Arc<Mutex<Option<ThreadId>>>,
}

unsafe fn read_vtable_slot(obj: usize, slot_idx: usize) -> usize {
    if obj == 0 {
        return 0;
    }
    let vtbl = std::ptr::read_unaligned(obj as *const usize);
    if vtbl == 0 {
        return 0;
    }
    std::ptr::read_unaligned((vtbl + slot_idx * std::mem::size_of::<usize>()) as *const usize)
}

unsafe fn invoke_callback_event(ev: &CallbackEvent) {
    match ev {
        CallbackEvent::EnginePassStart(cbs) => {
            for &cb in cbs {
                let f = read_vtable_slot(cb, 0);
                if f != 0 {
                    crate::call_guest8(f, cb as u64, 0, 0, 0, 0, 0, 0, 0);
                }
            }
        }
        CallbackEvent::EnginePassEnd(cbs) => {
            for &cb in cbs {
                let f = read_vtable_slot(cb, 1);
                if f != 0 {
                    crate::call_guest8(f, cb as u64, 0, 0, 0, 0, 0, 0, 0);
                }
            }
        }
        CallbackEvent::VoiceStreamEnd { cb, alive } => {
            if alive.load(Ordering::Acquire) {
                let f = read_vtable_slot(*cb, 2);
                if f != 0 {
                    crate::call_guest8(f, *cb as u64, 0, 0, 0, 0, 0, 0, 0);
                }
            }
        }
        CallbackEvent::VoiceBufferStart { cb, alive, ctx } => {
            if alive.load(Ordering::Acquire) {
                let f = read_vtable_slot(*cb, 3);
                if f != 0 {
                    crate::call_guest8(f, *cb as u64, *ctx as u64, 0, 0, 0, 0, 0, 0);
                }
            }
        }
        CallbackEvent::VoiceBufferEnd { cb, alive, ctx } => {
            if alive.load(Ordering::Acquire) {
                let f = read_vtable_slot(*cb, 4);
                if f != 0 {
                    crate::call_guest8(f, *cb as u64, *ctx as u64, 0, 0, 0, 0, 0, 0);
                }
            }
        }
        CallbackEvent::VoiceLoopEnd { cb, alive, ctx } => {
            if alive.load(Ordering::Acquire) {
                let f = read_vtable_slot(*cb, 5);
                if f != 0 {
                    crate::call_guest8(f, *cb as u64, *ctx as u64, 0, 0, 0, 0, 0, 0);
                }
            }
        }
    }
}

impl CallbackDispatcher {
    fn spawn() -> Option<Self> {
        let (tx, rx) = mpsc::channel::<CallbackMsg>();
        let tid_cell = Arc::new(Mutex::new(None));
        let tid_worker = tid_cell.clone();
        let body = Box::new(move || {
            *tid_worker.lock().unwrap() = Some(std::thread::current().id());
            while let Ok(msg) = rx.recv() {
                match msg {
                    CallbackMsg::Events(events) => {
                        for ev in &events {
                            unsafe { invoke_callback_event(ev) };
                        }
                    }
                    CallbackMsg::Barrier(done) => {
                        let _ = done.send(());
                    }
                }
            }
            0
        });
        match crate::thread::spawn(body, 256 * 1024, false, false) {
            Ok(t) => {
                *t.name.lock().unwrap() = "XAudio2 callback thread".into();
                Some(Self {
                    tx,
                    thread_id: tid_cell,
                })
            }
            Err(e) => {
                log::warn!(target: "audio", "xaudio2: failed to spawn callback thread: {e}");
                None
            }
        }
    }
}

// ============================================================================
// XAPO DSP Implementations & Builtin COM Objects (IXAPO + IXAPOParameters)
// ============================================================================

const BUILTIN_XAPO_MAGIC: u64 = 0x5841_504F_4255_494C; // "XAPOBUIL"
const XAPO_DEFAULT_MAX_FRAMES: u32 = 4096;

#[inline]
fn flush_denormal(x: f32) -> f32 {
    if !x.is_finite() || x.abs() < 1e-20 {
        0.0
    } else {
        x
    }
}

#[inline]
fn db_to_lin(db: f32) -> f32 {
    if !db.is_finite() || db <= -100.0 {
        0.0
    } else {
        10.0_f32.powf(db * 0.05)
    }
}

/// 16-byte aligned reusable `f32` buffer for XAPO `Process` calls (XAPO requires
/// 16-byte aligned `pBuffer` pointers for SIMD/SSE guest effects).
#[repr(C, align(16))]
#[derive(Clone, Copy, Default)]
struct AlignedF32Chunk([f32; 4]);

#[derive(Default)]
struct AlignedF32Buf {
    chunks: Vec<AlignedF32Chunk>,
    len: usize,
}

impl AlignedF32Buf {
    fn resize_zero(&mut self, len: usize) {
        let needed = len.div_ceil(4);
        self.chunks.clear();
        self.chunks.resize(needed, AlignedF32Chunk([0.0; 4]));
        self.len = len;
    }

    fn copy_from_slice(&mut self, src: &[f32]) {
        let needed = src.len().div_ceil(4);
        if self.chunks.len() < needed {
            self.chunks.resize(needed, AlignedF32Chunk([0.0; 4]));
        } else if needed > 0 {
            self.chunks[needed - 1] = AlignedF32Chunk([0.0; 4]);
        }
        self.len = src.len();
        if !src.is_empty() {
            self.as_mut_slice().copy_from_slice(src);
        }
    }

    fn as_slice(&self) -> &[f32] {
        if self.len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.chunks.as_ptr() as *const f32, self.len) }
        }
    }

    fn as_mut_slice(&mut self) -> &mut [f32] {
        if self.len == 0 {
            &mut []
        } else {
            unsafe {
                std::slice::from_raw_parts_mut(self.chunks.as_mut_ptr() as *mut f32, self.len)
            }
        }
    }

    fn as_mut_ptr(&mut self) -> *mut f32 {
        if self.chunks.is_empty() {
            self.chunks.push(AlignedF32Chunk([0.0; 4]));
        }
        self.chunks.as_mut_ptr() as *mut f32
    }
}

fn adapt_channels(src: &[f32], src_ch: usize, dst: &mut [f32], dst_ch: usize, frames: usize) {
    if frames == 0 || src_ch == 0 || dst_ch == 0 {
        return;
    }
    if src_ch == dst_ch {
        let n = frames * src_ch;
        dst[..n].copy_from_slice(&src[..n]);
        return;
    }
    if src_ch == 1 {
        for f in 0..frames {
            let v = src[f];
            let d = &mut dst[f * dst_ch..(f + 1) * dst_ch];
            d.fill(v);
        }
        return;
    }
    if dst_ch == 1 {
        let inv = 1.0 / (src_ch as f32);
        for f in 0..frames {
            let s = &src[f * src_ch..(f + 1) * src_ch];
            let mut acc = 0.0f32;
            for &v in s {
                acc += v;
            }
            dst[f] = acc * inv;
        }
        return;
    }
    let copy_ch = src_ch.min(dst_ch);
    for f in 0..frames {
        let s = &src[f * src_ch..(f + 1) * src_ch];
        let d = &mut dst[f * dst_ch..(f + 1) * dst_ch];
        d[..copy_ch].copy_from_slice(&s[..copy_ch]);
        for v in &mut d[copy_ch..] {
            *v = 0.0;
        }
    }
}

fn write_wide_fixed<const N: usize>(dst: &mut [u16; N], s: &str) {
    dst.fill(0);
    for (slot, unit) in dst.iter_mut().take(N.saturating_sub(1)).zip(s.encode_utf16()) {
        *slot = unit;
    }
}

fn make_float_wfx(channels: usize, sample_rate: u32) -> WaveFormatEx {
    let ch = channels.clamp(1, XAUDIO2_MAX_AUDIO_CHANNELS as usize) as u16;
    let block_align = ch * 4;
    WaveFormatEx {
        format_tag: WAVE_FORMAT_IEEE_FLOAT,
        channels: ch,
        samples_per_sec: sample_rate,
        avg_bytes_per_sec: sample_rate.saturating_mul(block_align as u32),
        block_align,
        bits_per_sample: 32,
        cb_size: 0,
    }
}

unsafe fn parse_xapo_float_format(ptr: *const WaveFormatEx) -> Result<(usize, u32), i32> {
    let fmt = parse_wave_format(ptr).map_err(|_| XAPO_E_FORMAT_UNSUPPORTED)?;
    if fmt.sample_kind != PcmSampleKind::F32 {
        return Err(XAPO_E_FORMAT_UNSUPPORTED);
    }
    Ok((fmt.channels, fmt.sample_rate))
}

// --- Delay Line, Comb & Allpass Primitives ---

#[derive(Clone)]
struct DelayLine {
    buf: Vec<f32>,
    write_idx: usize,
}

impl DelayLine {
    fn new(max_samples: usize) -> Self {
        Self {
            buf: vec![0.0; max_samples.max(2)],
            write_idx: 0,
        }
    }

    fn reset(&mut self) {
        self.buf.fill(0.0);
        self.write_idx = 0;
    }

    #[inline]
    fn write(&mut self, sample: f32) {
        self.buf[self.write_idx] = flush_denormal(sample);
        self.write_idx += 1;
        if self.write_idx >= self.buf.len() {
            self.write_idx = 0;
        }
    }

    #[inline]
    fn read_int(&self, delay_samples: usize) -> f32 {
        let len = self.buf.len();
        let d = delay_samples.min(len - 1);
        let idx = if self.write_idx >= d {
            self.write_idx - d
        } else {
            len + self.write_idx - d
        };
        self.buf[idx]
    }

    #[inline]
    fn read_frac(&self, delay_samples: f32) -> f32 {
        let len = self.buf.len();
        let max_d = (len - 2) as f32;
        let d = delay_samples.clamp(0.0, max_d);
        let d0 = d.floor() as usize;
        let frac = d - d0 as f32;
        let d1 = d0 + 1;
        let s0 = self.read_int(d0);
        let s1 = self.read_int(d1);
        s0 + (s1 - s0) * frac
    }
}

#[derive(Clone)]
struct CombFilter {
    buf: Vec<f32>,
    active_len: usize,
    idx: usize,
    filter_store: f32,
}

impl CombFilter {
    fn new(max_len: usize, active_len: usize) -> Self {
        let cap = max_len.max(16);
        Self {
            buf: vec![0.0; cap],
            active_len: active_len.clamp(8, cap),
            idx: 0,
            filter_store: 0.0,
        }
    }

    fn set_active_len(&mut self, new_len: usize) {
        self.active_len = new_len.clamp(8, self.buf.len());
        if self.idx >= self.active_len {
            self.idx = 0;
        }
    }

    fn reset(&mut self) {
        self.buf.fill(0.0);
        self.idx = 0;
        self.filter_store = 0.0;
    }

    #[inline]
    fn process(&mut self, input: f32, feedback: f32, damp1: f32, damp2: f32) -> f32 {
        let out = self.buf[self.idx];
        self.filter_store = flush_denormal(out * damp2 + self.filter_store * damp1);
        self.buf[self.idx] = flush_denormal(input + self.filter_store * feedback);
        self.idx += 1;
        if self.idx >= self.active_len {
            self.idx = 0;
        }
        out
    }
}

#[derive(Clone)]
struct AllpassFilter {
    buf: Vec<f32>,
    idx: usize,
}

impl AllpassFilter {
    fn new(len: usize) -> Self {
        Self {
            buf: vec![0.0; len.max(8)],
            idx: 0,
        }
    }

    fn reset(&mut self) {
        self.buf.fill(0.0);
        self.idx = 0;
    }

    #[inline]
    fn process(&mut self, input: f32, feedback: f32) -> f32 {
        let buf_out = self.buf[self.idx];
        let out = buf_out - input * feedback;
        self.buf[self.idx] = flush_denormal(input + buf_out * feedback);
        self.idx += 1;
        if self.idx >= self.buf.len() {
            self.idx = 0;
        }
        out
    }
}

// --- Schroeder/Freeverb Multichannel Reverb Engine (AudioReverb & FXReverb) ---

const FREEVERB_COMB_TUNINGS: [usize; 8] = [1116, 1188, 1277, 1356, 1422, 1491, 1557, 1617];
const FREEVERB_ALLPASS_TUNINGS: [usize; 4] = [556, 441, 341, 225];
const EARLY_TAP_MS: [f32; 4] = [0.0, 7.3, 17.1, 31.7];
const EARLY_TAP_GAINS: [f32; 4] = [0.55, -0.40, 0.30, -0.22];

struct ReverbChannel {
    predelay: DelayLine,
    room_lp: f32,
    low_shelf_lp: f32,
    combs: Vec<CombFilter>,
    allpasses: Vec<AllpassFilter>,
    out_delay: DelayLine,
}

impl ReverbChannel {
    fn new(sample_rate: u32, ch_idx: usize) -> Self {
        let sr_scale = (sample_rate as f32) / 44_100.0;
        let predelay_samples = ((sample_rate as f32) * 0.45).ceil() as usize + 64;
        let spread = (ch_idx % 8) * 23;
        let ap_spread = (ch_idx % 8) * 13;
        let combs = FREEVERB_COMB_TUNINGS
            .iter()
            .map(|&base| {
                let full_len = (((base + spread) as f32) * sr_scale).round() as usize;
                CombFilter::new(full_len.max(16), full_len.max(16))
            })
            .collect();
        let allpasses = FREEVERB_ALLPASS_TUNINGS
            .iter()
            .map(|&base| {
                let len = (((base + ap_spread) as f32) * sr_scale).round() as usize;
                AllpassFilter::new(len.max(8))
            })
            .collect();
        let out_delay_cap = ((sample_rate as f32) * 0.01).ceil() as usize + 8;
        Self {
            predelay: DelayLine::new(predelay_samples),
            room_lp: 0.0,
            low_shelf_lp: 0.0,
            combs,
            allpasses,
            out_delay: DelayLine::new(out_delay_cap),
        }
    }

    fn reset(&mut self) {
        self.predelay.reset();
        self.room_lp = 0.0;
        self.low_shelf_lp = 0.0;
        for c in &mut self.combs {
            c.reset();
        }
        for a in &mut self.allpasses {
            a.reset();
        }
        self.out_delay.reset();
    }
}

struct ReverbConfig {
    wet: f32,
    dry: f32,
    late_delay_samples: usize,
    early_tap_offsets: [usize; 4],
    early_gain: f32,
    late_gain: f32,
    room_main: f32,
    room_hf_gain: f32,
    room_lp_alpha: f32,
    low_eq_alpha: f32,
    low_eq_scale: f32,
    decay_time: f32,
    damp1: f32,
    allpass_feedback: f32,
    size_scale: f32,
    rear_delay_samples: usize,
    side_delay_samples: usize,
    disable_late_field: bool,
}

struct SchroederReverbDsp {
    sample_rate: u32,
    channels: Vec<ReverbChannel>,
    cfg: ReverbConfig,
    tail_frames_remaining: usize,
}

impl SchroederReverbDsp {
    fn new(cfg: ReverbConfig) -> Self {
        Self {
            sample_rate: 48_000,
            channels: Vec::new(),
            cfg,
            tail_frames_remaining: 0,
        }
    }

    fn configure(&mut self, sample_rate: u32, out_channels: usize) {
        self.sample_rate = sample_rate.max(XAUDIO2_MIN_SAMPLE_RATE);
        self.channels = (0..out_channels.max(1))
            .map(|c| ReverbChannel::new(self.sample_rate, c))
            .collect();
        self.tail_frames_remaining = 0;
        self.apply_comb_lengths();
    }

    fn reset(&mut self) {
        for ch in &mut self.channels {
            ch.reset();
        }
        self.tail_frames_remaining = 0;
    }

    fn set_config(&mut self, cfg: ReverbConfig) {
        self.cfg = cfg;
        self.apply_comb_lengths();
    }

    fn apply_comb_lengths(&mut self) {
        let sr_scale = (self.sample_rate as f32) / 44_100.0;
        let size_scale = self.cfg.size_scale.clamp(0.25, 1.0);
        for (ch_idx, ch) in self.channels.iter_mut().enumerate() {
            let spread = (ch_idx % 8) * 23;
            for (c, &base) in ch.combs.iter_mut().zip(FREEVERB_COMB_TUNINGS.iter()) {
                let target = (((base + spread) as f32) * sr_scale * size_scale).round() as usize;
                c.set_active_len(target.max(12));
            }
        }
    }

    /// Process reverb from `src` (or in-place from `dst` when `src` is `None` and `in_ch == out_ch`)
    /// into `dst` (`frames * out_ch`). Zero-allocation on the steady-state audio thread.
    fn process(
        &mut self,
        src: Option<&[f32]>,
        in_ch: usize,
        dst: &mut [f32],
        out_ch: usize,
        frames: usize,
        in_silent: bool,
    ) -> bool {
        if frames == 0 || out_ch == 0 {
            return false;
        }
        if self.channels.len() != out_ch {
            self.configure(self.sample_rate, out_ch);
        }
        if in_silent && self.tail_frames_remaining == 0 {
            dst[..frames * out_ch].fill(0.0);
            return false;
        }

        let sr = self.sample_rate as f32;
        let wet = self.cfg.wet;
        let dry = self.cfg.dry;
        let damp1 = self.cfg.damp1.clamp(0.0, 0.85);
        let damp2 = 1.0 - damp1;
        let ap_fb = self.cfg.allpass_feedback.clamp(0.0, 0.75);
        let decay = self.cfg.decay_time.max(0.05);
        let early_gain = self.cfg.early_gain;
        let late_gain = self.cfg.late_gain;
        let room_main = self.cfg.room_main;
        let room_hf_gain = self.cfg.room_hf_gain;
        let room_lp_alpha = self.cfg.room_lp_alpha;
        let low_eq_alpha = self.cfg.low_eq_alpha;
        let low_eq_scale = self.cfg.low_eq_scale;
        let disable_late = self.cfg.disable_late_field;
        let early_offsets = self.cfg.early_tap_offsets;
        let late_delay = self.cfg.late_delay_samples;
        let rear_delay = self.cfg.rear_delay_samples;
        let side_delay = self.cfg.side_delay_samples;

        let mut input_energy = 0.0f32;
        let mut wet_peak = 0.0f32;

        for f in 0..frames {
            for c in 0..out_ch {
                let x = if in_silent || in_ch == 0 {
                    0.0
                } else {
                    let s = match src {
                        Some(s_buf) => s_buf[f * in_ch + (c % in_ch)],
                        None => dst[f * out_ch + c],
                    };
                    let s = if s.is_finite() { s } else { 0.0 };
                    input_energy = input_energy.max(s.abs());
                    s
                };

                let ch = &mut self.channels[c];
                // Input room tone filter (high-shelf / LPF).
                ch.room_lp = flush_denormal(ch.room_lp + room_lp_alpha * (x - ch.room_lp));
                let filtered = room_main * (room_hf_gain * x + (1.0 - room_hf_gain) * ch.room_lp);

                ch.predelay.write(filtered);

                // Early reflections multi-tap sum (`+ 1` accounts for post-write index advance).
                let mut early = 0.0f32;
                for i in 0..4 {
                    early += ch.predelay.read_int(early_offsets[i] + 1) * EARLY_TAP_GAINS[i];
                }
                early *= early_gain;

                // Late field: 8 parallel damped Schroeder/Freeverb combs + 4 series unitary allpasses.
                let late = if disable_late {
                    0.0
                } else {
                    let late_in = ch.predelay.read_int(late_delay + 1);
                    let mut comb_sum = 0.0f32;
                    for comb in &mut ch.combs {
                        let fb = 10.0_f32
                            .powf(-3.0 * (comb.active_len as f32) / (decay * sr))
                            .clamp(0.0, 0.995);
                        comb_sum += comb.process(late_in, fb, damp1, damp2);
                    }
                    let mut diffused = comb_sum * 0.15;
                    for ap in &mut ch.allpasses {
                        diffused = ap.process(diffused, ap_fb);
                    }
                    // Low-frequency EQ shelf on late tail.
                    ch.low_shelf_lp =
                        flush_denormal(ch.low_shelf_lp + low_eq_alpha * (diffused - ch.low_shelf_lp));
                    let shaped = diffused + (low_eq_scale - 1.0) * ch.low_shelf_lp;
                    shaped * late_gain
                };

                let mut wet_sample = early + late;
                // Surround rear (channels 4, 5) and side (channels 6, 7) output delay.
                let extra_delay = match c {
                    4 | 5 => rear_delay,
                    6 | 7 => side_delay,
                    _ => 0,
                };
                if extra_delay > 0 {
                    ch.out_delay.write(wet_sample);
                    wet_sample = ch.out_delay.read_int(extra_delay + 1);
                }

                wet_peak = wet_peak.max(wet_sample.abs());
                let y = dry * x + wet * wet_sample;
                dst[f * out_ch + c] = if y.is_finite() { y } else { 0.0 };
            }
        }

        if input_energy > 1e-7 {
            let max_early = early_offsets[3];
            let pre_total = late_delay.max(max_early) + rear_delay.max(side_delay);
            let decay_frames = if disable_late {
                (sr * 0.05) as usize
            } else {
                (decay * 1.5 * sr) as usize + 4096
            };
            self.tail_frames_remaining = pre_total + decay_frames;
            true
        } else if wet_peak > 1e-7 {
            self.tail_frames_remaining = self.tail_frames_remaining.saturating_sub(frames);
            true
        } else {
            self.tail_frames_remaining = self.tail_frames_remaining.saturating_sub(frames);
            self.tail_frames_remaining > 0
        }
    }
}

fn reverb_config_from_xaudio2(p: &XAudio2FxReverbParameters, sample_rate: u32) -> ReverbConfig {
    let sr = sample_rate.max(XAUDIO2_MIN_SAMPLE_RATE) as f32;
    let wet = (p.wet_dry_mix * 0.01).clamp(0.0, 1.0);
    let dry = 1.0 - wet;
    let room_norm = (p.room_size * 0.01).clamp(0.0, 1.0);
    let density_norm = (p.density * 0.01).clamp(0.0, 1.0);
    let size_scale = (0.45 + 0.35 * room_norm + 0.20 * density_norm).clamp(0.35, 1.0);

    let refl_delay = ((p.reflections_delay as f32) * 0.001 * sr).round() as usize;
    let rev_delay = ((p.reverb_delay as f32) * 0.001 * sr).round() as usize;
    let rear_delay = ((p.rear_delay as f32) * 0.001 * sr).round() as usize;
    let side_delay = ((p.side_delay as f32) * 0.001 * sr).round() as usize;

    let mut early_tap_offsets = [0usize; 4];
    let early_scale = 0.4 + 0.6 * room_norm;
    for i in 0..4 {
        let ms = EARLY_TAP_MS[i] * early_scale;
        early_tap_offsets[i] = refl_delay + ((ms * 0.001 * sr).round() as usize);
    }
    let late_delay_samples = refl_delay + rev_delay;

    let room_freq = p.room_filter_freq.clamp(20.0, 0.45 * sr);
    let room_lp_alpha = (1.0 - (-TAU * room_freq / sr).exp()).clamp(0.001, 1.0);
    let room_main = db_to_lin(p.room_filter_main);
    let room_hf_gain = db_to_lin(p.room_filter_hf);

    // HighEQGain is 0..=8 where 8 = 0 dB (no extra HF damping), 0 = -8 dB (strong damping).
    let hf_eq_cut = (8u8.saturating_sub(p.high_eq_gain) as f32) / 8.0;
    let hf_room_cut = (1.0 - room_hf_gain).clamp(0.0, 1.0);
    let damp1 = (0.12 + 0.55 * hf_eq_cut + 0.25 * hf_room_cut).clamp(0.05, 0.85);

    // LowEQGain is 0..=12 where 8 = 0 dB (-8..+4 dB), LowEQCutoff is 0..=9 (50..500 Hz).
    let low_db = (p.low_eq_gain as f32) - 8.0;
    let low_eq_scale = db_to_lin(low_db).clamp(0.25, 2.0);
    let low_freq = (50.0 + 50.0 * (p.low_eq_cutoff as f32)).clamp(50.0, 500.0);
    let low_eq_alpha = (1.0 - (-TAU * low_freq / sr).exp()).clamp(0.001, 1.0);

    let early_diff_scale = 0.6 + 0.4 * ((p.early_diffusion as f32) / 15.0);
    let allpass_feedback = 0.20 + 0.45 * ((p.late_diffusion as f32) / 15.0);

    ReverbConfig {
        wet,
        dry,
        late_delay_samples,
        early_tap_offsets,
        early_gain: db_to_lin(p.reflections_gain) * early_diff_scale,
        late_gain: db_to_lin(p.reverb_gain),
        room_main,
        room_hf_gain,
        room_lp_alpha,
        low_eq_alpha,
        low_eq_scale,
        decay_time: p.decay_time.max(0.1),
        damp1,
        allpass_feedback,
        size_scale,
        rear_delay_samples: rear_delay,
        side_delay_samples: side_delay,
        disable_late_field: p.disable_late_field != 0,
    }
}

fn reverb_config_from_fxreverb(p: &FxReverbParameters, sample_rate: u32) -> ReverbConfig {
    let sr = sample_rate.max(XAUDIO2_MIN_SAMPLE_RATE) as f32;
    let diff = p.diffusion.clamp(FXREVERB_MIN_DIFFUSION, FXREVERB_MAX_DIFFUSION);
    let room = p.room_size.clamp(FXREVERB_MIN_ROOMSIZE, FXREVERB_MAX_ROOMSIZE);
    let size_scale = (0.35 + 0.65 * room).clamp(0.35, 1.0);
    let decay_time = 0.25 + 2.5 * room;
    let refl_delay = (0.004 * sr).round() as usize;
    let mut early_tap_offsets = [0usize; 4];
    for i in 0..4 {
        let ms = EARLY_TAP_MS[i] * (0.35 + 0.65 * room);
        early_tap_offsets[i] = refl_delay + ((ms * 0.001 * sr).round() as usize);
    }
    let late_delay_samples = refl_delay + ((0.008 * sr).round() as usize);
    let room_lp_alpha = (1.0 - (-TAU * 6000.0 / sr).exp()).clamp(0.001, 1.0);
    let low_eq_alpha = (1.0 - (-TAU * 250.0 / sr).exp()).clamp(0.001, 1.0);

    ReverbConfig {
        wet: 0.55,
        dry: 0.65,
        late_delay_samples,
        early_tap_offsets,
        early_gain: 0.55,
        late_gain: 0.95,
        room_main: 1.0,
        room_hf_gain: 0.85,
        room_lp_alpha,
        low_eq_alpha,
        low_eq_scale: 1.0,
        decay_time,
        damp1: 0.25,
        allpass_feedback: 0.15 + 0.55 * diff,
        size_scale,
        rear_delay_samples: 0,
        side_delay_samples: 0,
        disable_late_field: false,
    }
}

fn validate_xaudio2_reverb_params(p: &XAudio2FxReverbParameters) -> bool {
    let wet = p.wet_dry_mix;
    let refl_d = p.reflections_delay;
    let rev_d = p.reverb_delay;
    let rear_d = p.rear_delay;
    let side_d = p.side_delay;
    let rff = p.room_filter_freq;
    let rfm = p.room_filter_main;
    let rfh = p.room_filter_hf;
    let rg = p.reflections_gain;
    let rvg = p.reverb_gain;
    let dt = p.decay_time;
    let dens = p.density;
    let rsz = p.room_size;
    wet.is_finite()
        && (0.0..=100.0).contains(&wet)
        && refl_d <= 300
        && rev_d <= 85
        && rear_d <= 5
        && side_d <= 5
        && p.position_left <= 30
        && p.position_right <= 30
        && p.position_matrix_left <= 30
        && p.position_matrix_right <= 30
        && p.early_diffusion <= 15
        && p.late_diffusion <= 15
        && p.low_eq_gain <= 12
        && p.low_eq_cutoff <= 9
        && p.high_eq_gain <= 8
        && p.high_eq_cutoff <= 14
        && rff.is_finite()
        && (20.0..=20_000.0).contains(&rff)
        && rfm.is_finite()
        && (-100.0..=0.0).contains(&rfm)
        && rfh.is_finite()
        && (-100.0..=0.0).contains(&rfh)
        && rg.is_finite()
        && (-100.0..=20.0).contains(&rg)
        && rvg.is_finite()
        && (-100.0..=20.0).contains(&rvg)
        && dt.is_finite()
        && dt >= 0.1
        && dens.is_finite()
        && (0.0..=100.0).contains(&dens)
        && rsz.is_finite()
        && (0.0..=100.0).contains(&rsz)
}

// --- Volume Meter DSP ---

struct VolumeMeterDsp {
    channels: usize,
    peak: Vec<f32>,
    rms: Vec<f32>,
}

impl VolumeMeterDsp {
    fn new() -> Self {
        Self {
            channels: 0,
            peak: Vec::new(),
            rms: Vec::new(),
        }
    }

    fn configure(&mut self, channels: usize) {
        self.channels = channels;
        self.peak = vec![0.0; channels];
        self.rms = vec![0.0; channels];
    }

    fn reset(&mut self) {
        self.peak.fill(0.0);
        self.rms.fill(0.0);
    }

    fn process(&mut self, src: &[f32], ch: usize, frames: usize, in_silent: bool) {
        if ch == 0 {
            return;
        }
        if self.channels != ch {
            self.configure(ch);
        }
        if in_silent || frames == 0 {
            self.peak.fill(0.0);
            self.rms.fill(0.0);
            return;
        }
        self.peak.fill(0.0);
        self.rms.fill(0.0);
        for f in 0..frames {
            let base = f * ch;
            for c in 0..ch {
                let s = src[base + c];
                let a = if s.is_finite() { s.abs() } else { 0.0 };
                if a > self.peak[c] {
                    self.peak[c] = a;
                }
                self.rms[c] += a * a;
            }
        }
        let inv_n = 1.0 / (frames as f32);
        for c in 0..ch {
            self.rms[c] = (self.rms[c] * inv_n).sqrt();
        }
    }
}

// --- FXEQ 4-Band Parametric Biquad EQ ---

#[derive(Clone, Copy)]
struct BiquadCoeffs {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    bypass: bool,
}

impl Default for BiquadCoeffs {
    fn default() -> Self {
        Self {
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            bypass: true,
        }
    }
}

#[derive(Clone, Copy, Default)]
struct BiquadState {
    z1: f32,
    z2: f32,
}

fn design_peaking_eq(freq_hz: f32, linear_gain: f32, bandwidth_oct: f32, sr: f32) -> BiquadCoeffs {
    if (linear_gain - 1.0).abs() < 1e-5 {
        return BiquadCoeffs::default();
    }
    let fc = freq_hz.clamp(20.0, (sr * 0.48).max(20.0));
    let bw = bandwidth_oct.clamp(FXEQ_MIN_BANDWIDTH, FXEQ_MAX_BANDWIDTH);
    let a = linear_gain.clamp(FXEQ_MIN_GAIN, FXEQ_MAX_GAIN).sqrt();
    let w0 = TAU * fc / sr;
    let sin_w0 = w0.sin();
    let cos_w0 = w0.cos();
    let alpha = sin_w0 * ((std::f32::consts::LN_2 * 0.5) * bw * (w0 / sin_w0.max(1e-6))).sinh();
    let a0 = 1.0 + alpha / a;
    if !a0.is_finite() || a0.abs() < 1e-12 {
        return BiquadCoeffs::default();
    }
    let inv_a0 = 1.0 / a0;
    BiquadCoeffs {
        b0: (1.0 + alpha * a) * inv_a0,
        b1: (-2.0 * cos_w0) * inv_a0,
        b2: (1.0 - alpha * a) * inv_a0,
        a1: (-2.0 * cos_w0) * inv_a0,
        a2: (1.0 - alpha / a) * inv_a0,
        bypass: false,
    }
}

struct FxEqDsp {
    params: FxEqParameters,
    sample_rate: u32,
    coeffs: [BiquadCoeffs; 4],
    states: Vec<[BiquadState; 4]>,
}

impl FxEqDsp {
    fn new() -> Self {
        let mut s = Self {
            params: FxEqParameters::default(),
            sample_rate: 48_000,
            coeffs: [BiquadCoeffs::default(); 4],
            states: Vec::new(),
        };
        s.update_coeffs();
        s
    }

    fn configure(&mut self, sample_rate: u32, channels: usize) {
        self.sample_rate = sample_rate.max(XAUDIO2_MIN_SAMPLE_RATE);
        self.states = vec![[BiquadState::default(); 4]; channels.max(1)];
        self.update_coeffs();
    }

    fn reset(&mut self) {
        for ch in &mut self.states {
            *ch = [BiquadState::default(); 4];
        }
    }

    fn validate(p: &FxEqParameters) -> bool {
        let bands = [
            (p.frequency_center0, p.gain0, p.bandwidth0),
            (p.frequency_center1, p.gain1, p.bandwidth1),
            (p.frequency_center2, p.gain2, p.bandwidth2),
            (p.frequency_center3, p.gain3, p.bandwidth3),
        ];
        bands.iter().all(|&(f, g, b)| {
            f.is_finite()
                && (FXEQ_MIN_FREQUENCY_CENTER..=FXEQ_MAX_FREQUENCY_CENTER).contains(&f)
                && g.is_finite()
                && (FXEQ_MIN_GAIN..=FXEQ_MAX_GAIN).contains(&g)
                && b.is_finite()
                && (FXEQ_MIN_BANDWIDTH..=FXEQ_MAX_BANDWIDTH).contains(&b)
        })
    }

    fn update_coeffs(&mut self) {
        let sr = self.sample_rate as f32;
        let p = self.params;
        self.coeffs[0] = design_peaking_eq(p.frequency_center0, p.gain0, p.bandwidth0, sr);
        self.coeffs[1] = design_peaking_eq(p.frequency_center1, p.gain1, p.bandwidth1, sr);
        self.coeffs[2] = design_peaking_eq(p.frequency_center2, p.gain2, p.bandwidth2, sr);
        self.coeffs[3] = design_peaking_eq(p.frequency_center3, p.gain3, p.bandwidth3, sr);
    }

    fn has_active_state(&self) -> bool {
        self.states
            .iter()
            .any(|ch| ch.iter().any(|s| s.z1.abs() > 1e-7 || s.z2.abs() > 1e-7))
    }

    fn process(&mut self, buf: &mut [f32], ch: usize, frames: usize, in_silent: bool) -> bool {
        if frames == 0 || ch == 0 {
            return false;
        }
        if self.states.len() != ch {
            self.states.resize(ch, [BiquadState::default(); 4]);
        }
        if in_silent && !self.has_active_state() {
            buf[..frames * ch].fill(0.0);
            return false;
        }
        if self.coeffs.iter().all(|c| c.bypass) {
            return !in_silent;
        }
        let mut out_peak = 0.0f32;
        for f in 0..frames {
            let base = f * ch;
            for c in 0..ch {
                let mut x = if in_silent { 0.0 } else { buf[base + c] };
                if !x.is_finite() {
                    x = 0.0;
                }
                let st_arr = &mut self.states[c];
                for b in 0..4 {
                    let co = self.coeffs[b];
                    if co.bypass {
                        continue;
                    }
                    let st = &mut st_arr[b];
                    let y = co.b0 * x + st.z1;
                    st.z1 = flush_denormal(co.b1 * x - co.a1 * y + st.z2);
                    st.z2 = flush_denormal(co.b2 * x - co.a2 * y);
                    x = y;
                }
                out_peak = out_peak.max(x.abs());
                buf[base + c] = x;
            }
        }
        !in_silent || out_peak > 1e-7
    }
}

// --- FXMasteringLimiter Linked Multichannel Peak/Envelope Limiter ---

struct FxMasteringLimiterDsp {
    params: FxMasteringLimiterParameters,
    sample_rate: u32,
    envelope: f32,
}

impl FxMasteringLimiterDsp {
    fn new() -> Self {
        Self {
            params: FxMasteringLimiterParameters::default(),
            sample_rate: 48_000,
            envelope: 0.0,
        }
    }

    fn configure(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate.max(XAUDIO2_MIN_SAMPLE_RATE);
    }

    fn reset(&mut self) {
        self.envelope = 0.0;
    }

    fn validate(p: &FxMasteringLimiterParameters) -> bool {
        let rel = p.release;
        let loud = p.loudness;
        (FXMASTERINGLIMITER_MIN_RELEASE..=FXMASTERINGLIMITER_MAX_RELEASE).contains(&rel)
            && (FXMASTERINGLIMITER_MIN_LOUDNESS..=FXMASTERINGLIMITER_MAX_LOUDNESS).contains(&loud)
    }

    fn process(&mut self, buf: &mut [f32], ch: usize, frames: usize, in_silent: bool) -> bool {
        if frames == 0 || ch == 0 {
            return false;
        }
        let rel_ms = self.params.release.clamp(
            FXMASTERINGLIMITER_MIN_RELEASE,
            FXMASTERINGLIMITER_MAX_RELEASE,
        ) as f32;
        let loud = self.params.loudness.clamp(
            FXMASTERINGLIMITER_MIN_LOUDNESS,
            FXMASTERINGLIMITER_MAX_LOUDNESS,
        ) as f32;
        let release_samples = (rel_ms * 0.001 * (self.sample_rate as f32)).max(1.0);
        let release_coeff = (-1.0 / release_samples).exp();

        if in_silent {
            // Advance envelope release during silence so subsequent bursts recover properly.
            self.envelope = flush_denormal(self.envelope * release_coeff.powi(frames as i32));
            buf[..frames * ch].fill(0.0);
            return false;
        }

        // Loudness <= 1000 lowers ceiling proportionally (1000 = 1.0, 500 = 0.5);
        // Loudness > 1000 applies makeup drive while maintaining the 1.0 brickwall ceiling.
        let (makeup, threshold) = if loud <= 1000.0 {
            (1.0, (loud * 0.001).max(0.001))
        } else {
            (loud * 0.001, 1.0)
        };

        for f in 0..frames {
            let base = f * ch;
            let mut frame_peak = 0.0f32;
            for c in 0..ch {
                let s = buf[base + c];
                let driven = if s.is_finite() { s * makeup } else { 0.0 };
                buf[base + c] = driven;
                frame_peak = frame_peak.max(driven.abs());
            }
            if frame_peak > self.envelope {
                self.envelope = frame_peak;
            } else {
                self.envelope =
                    flush_denormal(frame_peak + release_coeff * (self.envelope - frame_peak));
            }
            let gain = if self.envelope > threshold {
                threshold / self.envelope
            } else {
                1.0
            };
            for c in 0..ch {
                buf[base + c] *= gain;
            }
        }
        true
    }
}

// --- FXEcho Multichannel Delay + Feedback ---

struct FxEchoDsp {
    max_delay_ms: f32,
    params: FxEchoParameters,
    sample_rate: u32,
    lines: Vec<DelayLine>,
    tail_frames_remaining: usize,
}

impl FxEchoDsp {
    fn new() -> Self {
        Self {
            max_delay_ms: FXECHO_MAX_DELAY,
            params: FxEchoParameters::default(),
            sample_rate: 48_000,
            lines: Vec::new(),
            tail_frames_remaining: 0,
        }
    }

    fn set_max_delay(&mut self, max_delay_ms: f32) -> Result<(), i32> {
        if !max_delay_ms.is_finite()
            || !(FXECHO_MIN_DELAY..=FXECHO_MAX_DELAY).contains(&max_delay_ms)
        {
            return Err(E_INVALIDARG);
        }
        self.max_delay_ms = max_delay_ms;
        if self.params.delay > max_delay_ms {
            self.params.delay = max_delay_ms;
        }
        if !self.lines.is_empty() {
            let ch = self.lines.len();
            self.configure(self.sample_rate, ch);
        }
        Ok(())
    }

    fn configure(&mut self, sample_rate: u32, channels: usize) {
        self.sample_rate = sample_rate.max(XAUDIO2_MIN_SAMPLE_RATE);
        let max_samples =
            ((self.max_delay_ms * 0.001 * (self.sample_rate as f32)).ceil() as usize) + 4;
        self.lines = (0..channels.max(1))
            .map(|_| DelayLine::new(max_samples))
            .collect();
        self.tail_frames_remaining = 0;
    }

    fn reset(&mut self) {
        for line in &mut self.lines {
            line.reset();
        }
        self.tail_frames_remaining = 0;
    }

    fn validate(&self, p: &FxEchoParameters) -> bool {
        let wet = p.wet_dry_mix;
        let fb = p.feedback;
        let del = p.delay;
        wet.is_finite()
            && (FXECHO_MIN_WETDRYMIX..=FXECHO_MAX_WETDRYMIX).contains(&wet)
            && fb.is_finite()
            && (FXECHO_MIN_FEEDBACK..=FXECHO_MAX_FEEDBACK).contains(&fb)
            && del.is_finite()
            && (FXECHO_MIN_DELAY..=self.max_delay_ms).contains(&del)
    }

    fn process(&mut self, buf: &mut [f32], ch: usize, frames: usize, in_silent: bool) -> bool {
        if frames == 0 || ch == 0 {
            return false;
        }
        if self.lines.len() != ch {
            self.configure(self.sample_rate, ch);
        }
        if in_silent && self.tail_frames_remaining == 0 {
            buf[..frames * ch].fill(0.0);
            return false;
        }

        let wet = self
            .params
            .wet_dry_mix
            .clamp(FXECHO_MIN_WETDRYMIX, FXECHO_MAX_WETDRYMIX);
        let dry = 1.0 - wet;
        let fb = self
            .params
            .feedback
            .clamp(FXECHO_MIN_FEEDBACK, FXECHO_MAX_FEEDBACK);
        let delay_ms = self.params.delay.clamp(FXECHO_MIN_DELAY, self.max_delay_ms);
        let delay_samples = (delay_ms * 0.001 * (self.sample_rate as f32)).max(1.0);

        let mut input_peak = 0.0f32;
        let mut out_peak = 0.0f32;

        for f in 0..frames {
            let base = f * ch;
            for c in 0..ch {
                let x = if in_silent {
                    0.0
                } else {
                    let v = buf[base + c];
                    let v = if v.is_finite() { v } else { 0.0 };
                    input_peak = input_peak.max(v.abs());
                    v
                };
                let line = &mut self.lines[c];
                let delayed = line.read_frac(delay_samples);
                let out = dry * x + wet * delayed;
                let store = (x + delayed * fb).clamp(-16.0, 16.0);
                line.write(store);
                out_peak = out_peak.max(out.abs());
                buf[base + c] = out;
            }
        }

        let d_int = delay_samples.ceil() as usize;
        if input_peak > 1e-7 {
            let repeats = if fb <= 0.001 {
                1usize
            } else if fb >= 0.999 {
                32usize
            } else {
                ((-6.5 / fb.log10()).ceil() as usize).clamp(1, 32)
            };
            self.tail_frames_remaining = d_int.saturating_mul(repeats).saturating_add(64);
            true
        } else if out_peak > 1e-7 {
            self.tail_frames_remaining = self.tail_frames_remaining.saturating_sub(frames);
            true
        } else {
            self.tail_frames_remaining = self.tail_frames_remaining.saturating_sub(frames);
            self.tail_frames_remaining > 0
        }
    }
}

// --- Builtin XAPO State & COM Object ---

enum BuiltinXapoKind {
    AudioReverb {
        params: XAudio2FxReverbParameters,
        dsp: SchroederReverbDsp,
    },
    VolumeMeter(VolumeMeterDsp),
    FxEq(FxEqDsp),
    FxMasteringLimiter(FxMasteringLimiterDsp),
    FxReverb {
        params: FxReverbParameters,
        dsp: SchroederReverbDsp,
    },
    FxEcho(FxEchoDsp),
}

struct BuiltinXapoState {
    kind: BuiltinXapoKind,
    locked: bool,
    sample_rate: u32,
    in_channels: usize,
    out_channels: usize,
}

impl BuiltinXapoState {
    fn new(kind: BuiltinXapoKind) -> Self {
        Self {
            kind,
            locked: false,
            sample_rate: 48_000,
            in_channels: 2,
            out_channels: 2,
        }
    }

    fn clsid(&self) -> GUID {
        match self.kind {
            BuiltinXapoKind::AudioReverb { .. } => CLSID_AUDIO_REVERB,
            BuiltinXapoKind::VolumeMeter(_) => CLSID_AUDIO_VOLUME_METER,
            BuiltinXapoKind::FxEq(_) => CLSID_FX_EQ,
            BuiltinXapoKind::FxMasteringLimiter(_) => CLSID_FX_MASTERING_LIMITER,
            BuiltinXapoKind::FxReverb { .. } => CLSID_FX_REVERB,
            BuiltinXapoKind::FxEcho(_) => CLSID_FX_ECHO,
        }
    }

    fn friendly_name(&self) -> &'static str {
        match self.kind {
            BuiltinXapoKind::AudioReverb { .. } => "XAudio2 Reverb",
            BuiltinXapoKind::VolumeMeter(_) => "XAudio2 Volume Meter",
            BuiltinXapoKind::FxEq(_) => "FXEQ",
            BuiltinXapoKind::FxMasteringLimiter(_) => "FXMasteringLimiter",
            BuiltinXapoKind::FxReverb { .. } => "FXReverb",
            BuiltinXapoKind::FxEcho(_) => "FXEcho",
        }
    }

    fn reg_flags(&self) -> u32 {
        match self.kind {
            BuiltinXapoKind::AudioReverb { .. } | BuiltinXapoKind::FxReverb { .. } => {
                XAPO_FLAG_FRAMERATE_MUST_MATCH
                    | XAPO_FLAG_BITSPERSAMPLE_MUST_MATCH
                    | XAPO_FLAG_BUFFERCOUNT_MUST_MATCH
                    | XAPO_FLAG_INPLACE_SUPPORTED
            }
            BuiltinXapoKind::VolumeMeter(_)
            | BuiltinXapoKind::FxEq(_)
            | BuiltinXapoKind::FxMasteringLimiter(_)
            | BuiltinXapoKind::FxEcho(_) => XAPOBASE_DEFAULT_FLAG,
        }
    }

    fn channels_must_match(&self) -> bool {
        (self.reg_flags() & XAPO_FLAG_CHANNELS_MUST_MATCH) != 0
    }

    fn registration_properties(&self) -> XapoRegistrationProperties {
        let mut friendly_name = [0u16; XAPO_REGISTRATION_STRING_LENGTH];
        let mut copyright_info = [0u16; XAPO_REGISTRATION_STRING_LENGTH];
        write_wide_fixed(&mut friendly_name, self.friendly_name());
        write_wide_fixed(&mut copyright_info, "Builtin XAudio2 XAPO");
        XapoRegistrationProperties {
            clsid: self.clsid(),
            friendly_name,
            copyright_info,
            major_version: 1,
            minor_version: 0,
            flags: self.reg_flags(),
            min_input_buffer_count: 1,
            max_input_buffer_count: 1,
            min_output_buffer_count: 1,
            max_output_buffer_count: 1,
        }
    }

    unsafe fn initialize(&mut self, p_data: *const c_void, byte_size: u32) -> i32 {
        match &mut self.kind {
            BuiltinXapoKind::FxEcho(echo) => {
                if p_data.is_null() && byte_size == 0 {
                    return S_OK;
                }
                if p_data.is_null() || (byte_size as usize) < std::mem::size_of::<FxEchoInitData>()
                {
                    return E_INVALIDARG;
                }
                let init = std::ptr::read_unaligned(p_data as *const FxEchoInitData);
                match echo.set_max_delay(init.max_delay) {
                    Ok(()) => S_OK,
                    Err(hr) => hr,
                }
            }
            _ => {
                if !p_data.is_null() && byte_size > 0 {
                    // Builtin effects without init data accept optional init cleanly.
                    S_OK
                } else {
                    S_OK
                }
            }
        }
    }

    fn reset(&mut self) {
        match &mut self.kind {
            BuiltinXapoKind::AudioReverb { dsp, .. } => dsp.reset(),
            BuiltinXapoKind::VolumeMeter(vm) => vm.reset(),
            BuiltinXapoKind::FxEq(eq) => eq.reset(),
            BuiltinXapoKind::FxMasteringLimiter(lim) => lim.reset(),
            BuiltinXapoKind::FxReverb { dsp, .. } => dsp.reset(),
            BuiltinXapoKind::FxEcho(echo) => echo.reset(),
        }
    }

    fn lock_for_process(&mut self, sample_rate: u32, in_ch: usize, out_ch: usize) -> i32 {
        if in_ch == 0
            || out_ch == 0
            || in_ch > XAUDIO2_MAX_AUDIO_CHANNELS as usize
            || out_ch > XAUDIO2_MAX_AUDIO_CHANNELS as usize
            || !(XAUDIO2_MIN_SAMPLE_RATE..=XAUDIO2_MAX_SAMPLE_RATE).contains(&sample_rate)
        {
            return XAPO_E_FORMAT_UNSUPPORTED;
        }
        if self.channels_must_match() && in_ch != out_ch {
            return XAPO_E_FORMAT_UNSUPPORTED;
        }
        self.sample_rate = sample_rate;
        self.in_channels = in_ch;
        self.out_channels = out_ch;
        self.locked = true;
        match &mut self.kind {
            BuiltinXapoKind::AudioReverb { params, dsp } => {
                dsp.set_config(reverb_config_from_xaudio2(params, sample_rate));
                dsp.configure(sample_rate, out_ch);
            }
            BuiltinXapoKind::VolumeMeter(vm) => vm.configure(in_ch),
            BuiltinXapoKind::FxEq(eq) => eq.configure(sample_rate, in_ch),
            BuiltinXapoKind::FxMasteringLimiter(lim) => lim.configure(sample_rate),
            BuiltinXapoKind::FxReverb { params, dsp } => {
                dsp.set_config(reverb_config_from_fxreverb(params, sample_rate));
                dsp.configure(sample_rate, out_ch);
            }
            BuiltinXapoKind::FxEcho(echo) => echo.configure(sample_rate, in_ch),
        }
        S_OK
    }

    fn unlock_for_process(&mut self) {
        self.locked = false;
    }

    fn has_active_tail(&self) -> bool {
        match &self.kind {
            BuiltinXapoKind::AudioReverb { dsp, .. } => dsp.tail_frames_remaining > 0,
            BuiltinXapoKind::FxReverb { dsp, .. } => dsp.tail_frames_remaining > 0,
            BuiltinXapoKind::FxEcho(echo) => echo.tail_frames_remaining > 0,
            BuiltinXapoKind::FxEq(eq) => eq.has_active_state(),
            BuiltinXapoKind::VolumeMeter(_) | BuiltinXapoKind::FxMasteringLimiter(_) => false,
        }
    }

    /// Process audio in-place (`in_ch == out_ch`). Returns `true` if output is valid (non-silent).
    fn process_in_place(
        &mut self,
        buf: &mut [f32],
        ch: usize,
        frames: usize,
        in_silent: bool,
        is_enabled: bool,
    ) -> bool {
        if frames == 0 || ch == 0 {
            return false;
        }
        if !is_enabled {
            if in_silent {
                buf[..frames * ch].fill(0.0);
                return false;
            }
            return true;
        }
        match &mut self.kind {
            BuiltinXapoKind::AudioReverb { dsp, .. } | BuiltinXapoKind::FxReverb { dsp, .. } => {
                dsp.process(None, ch, buf, ch, frames, in_silent)
            }
            BuiltinXapoKind::VolumeMeter(vm) => {
                vm.process(buf, ch, frames, in_silent);
                if in_silent {
                    buf[..frames * ch].fill(0.0);
                    false
                } else {
                    true
                }
            }
            BuiltinXapoKind::FxEq(eq) => eq.process(buf, ch, frames, in_silent),
            BuiltinXapoKind::FxMasteringLimiter(lim) => lim.process(buf, ch, frames, in_silent),
            BuiltinXapoKind::FxEcho(echo) => echo.process(buf, ch, frames, in_silent),
        }
    }

    /// Process audio from `src` (`frames * in_ch`) into `dst` (`frames * out_ch`).
    /// Returns `true` if `dst` contains valid (non-silent) audio.
    fn process_out_of_place(
        &mut self,
        src: &[f32],
        in_ch: usize,
        dst: &mut [f32],
        out_ch: usize,
        frames: usize,
        in_silent: bool,
        is_enabled: bool,
    ) -> bool {
        if frames == 0 || out_ch == 0 {
            return false;
        }
        if !is_enabled {
            if in_silent {
                dst[..frames * out_ch].fill(0.0);
                return false;
            }
            adapt_channels(src, in_ch, dst, out_ch, frames);
            return true;
        }
        match &mut self.kind {
            BuiltinXapoKind::AudioReverb { dsp, .. } | BuiltinXapoKind::FxReverb { dsp, .. } => {
                dsp.process(Some(src), in_ch, dst, out_ch, frames, in_silent)
            }
            BuiltinXapoKind::VolumeMeter(vm) => {
                if in_silent {
                    vm.process(src, in_ch, frames, true);
                    dst[..frames * out_ch].fill(0.0);
                    false
                } else {
                    vm.process(src, in_ch, frames, false);
                    adapt_channels(src, in_ch, dst, out_ch, frames);
                    true
                }
            }
            BuiltinXapoKind::FxEq(eq) => {
                if !in_silent {
                    adapt_channels(src, in_ch, dst, out_ch, frames);
                }
                eq.process(dst, out_ch, frames, in_silent)
            }
            BuiltinXapoKind::FxMasteringLimiter(lim) => {
                if !in_silent {
                    adapt_channels(src, in_ch, dst, out_ch, frames);
                }
                lim.process(dst, out_ch, frames, in_silent)
            }
            BuiltinXapoKind::FxEcho(echo) => {
                if !in_silent {
                    adapt_channels(src, in_ch, dst, out_ch, frames);
                }
                echo.process(dst, out_ch, frames, in_silent)
            }
        }
    }

    unsafe fn set_parameters(&mut self, ptr: *const c_void, byte_size: u32) -> i32 {
        match &mut self.kind {
            BuiltinXapoKind::AudioReverb { params, dsp } => {
                if ptr.is_null() {
                    return E_POINTER;
                }
                let sz = byte_size as usize;
                let parsed = if sz == std::mem::size_of::<XAudio2FxReverbParameters>() {
                    std::ptr::read_unaligned(ptr as *const XAudio2FxReverbParameters)
                } else if sz == std::mem::size_of::<XAudio2FxReverbParameters28>() {
                    let p28 = std::ptr::read_unaligned(ptr as *const XAudio2FxReverbParameters28);
                    XAudio2FxReverbParameters {
                        wet_dry_mix: p28.wet_dry_mix,
                        reflections_delay: p28.reflections_delay,
                        reverb_delay: p28.reverb_delay,
                        rear_delay: p28.rear_delay,
                        side_delay: 5,
                        position_left: p28.position_left,
                        position_right: p28.position_right,
                        position_matrix_left: p28.position_matrix_left,
                        position_matrix_right: p28.position_matrix_right,
                        early_diffusion: p28.early_diffusion,
                        late_diffusion: p28.late_diffusion,
                        low_eq_gain: p28.low_eq_gain,
                        low_eq_cutoff: p28.low_eq_cutoff,
                        high_eq_gain: p28.high_eq_gain,
                        high_eq_cutoff: p28.high_eq_cutoff,
                        room_filter_freq: p28.room_filter_freq,
                        room_filter_main: p28.room_filter_main,
                        room_filter_hf: p28.room_filter_hf,
                        reflections_gain: p28.reflections_gain,
                        reverb_gain: p28.reverb_gain,
                        decay_time: p28.decay_time,
                        density: p28.density,
                        room_size: p28.room_size,
                        disable_late_field: p28.disable_late_field,
                    }
                } else if sz == std::mem::size_of::<XAudio2FxReverbParameters27>() {
                    let p27 = std::ptr::read_unaligned(ptr as *const XAudio2FxReverbParameters27);
                    XAudio2FxReverbParameters {
                        wet_dry_mix: p27.wet_dry_mix,
                        reflections_delay: p27.reflections_delay,
                        reverb_delay: p27.reverb_delay,
                        rear_delay: p27.rear_delay,
                        side_delay: 5,
                        position_left: p27.position_left,
                        position_right: p27.position_right,
                        position_matrix_left: p27.position_matrix_left,
                        position_matrix_right: p27.position_matrix_right,
                        early_diffusion: p27.early_diffusion,
                        late_diffusion: p27.late_diffusion,
                        low_eq_gain: p27.low_eq_gain,
                        low_eq_cutoff: p27.low_eq_cutoff,
                        high_eq_gain: p27.high_eq_gain,
                        high_eq_cutoff: p27.high_eq_cutoff,
                        room_filter_freq: p27.room_filter_freq,
                        room_filter_main: p27.room_filter_main,
                        room_filter_hf: p27.room_filter_hf,
                        reflections_gain: p27.reflections_gain,
                        reverb_gain: p27.reverb_gain,
                        decay_time: p27.decay_time,
                        density: p27.density,
                        room_size: p27.room_size,
                        disable_late_field: 0,
                    }
                } else {
                    return E_INVALIDARG;
                };
                if !validate_xaudio2_reverb_params(&parsed) {
                    return E_INVALIDARG;
                }
                *params = parsed;
                dsp.set_config(reverb_config_from_xaudio2(params, self.sample_rate));
                S_OK
            }
            BuiltinXapoKind::VolumeMeter(_) => {
                // VolumeMeter has no writable parameters; SetParameters is a no-op.
                S_OK
            }
            BuiltinXapoKind::FxEq(eq) => {
                if ptr.is_null() {
                    return E_POINTER;
                }
                if (byte_size as usize) != std::mem::size_of::<FxEqParameters>() {
                    return E_INVALIDARG;
                }
                let p = std::ptr::read_unaligned(ptr as *const FxEqParameters);
                if !FxEqDsp::validate(&p) {
                    return E_INVALIDARG;
                }
                eq.params = p;
                eq.update_coeffs();
                S_OK
            }
            BuiltinXapoKind::FxMasteringLimiter(lim) => {
                if ptr.is_null() {
                    return E_POINTER;
                }
                if (byte_size as usize) != std::mem::size_of::<FxMasteringLimiterParameters>() {
                    return E_INVALIDARG;
                }
                let p = std::ptr::read_unaligned(ptr as *const FxMasteringLimiterParameters);
                if !FxMasteringLimiterDsp::validate(&p) {
                    return E_INVALIDARG;
                }
                lim.params = p;
                S_OK
            }
            BuiltinXapoKind::FxReverb { params, dsp } => {
                if ptr.is_null() {
                    return E_POINTER;
                }
                if (byte_size as usize) != std::mem::size_of::<FxReverbParameters>() {
                    return E_INVALIDARG;
                }
                let p = std::ptr::read_unaligned(ptr as *const FxReverbParameters);
                let diff = p.diffusion;
                let room = p.room_size;
                if !diff.is_finite()
                    || !(FXREVERB_MIN_DIFFUSION..=FXREVERB_MAX_DIFFUSION).contains(&diff)
                    || !room.is_finite()
                    || !(FXREVERB_MIN_ROOMSIZE..=FXREVERB_MAX_ROOMSIZE).contains(&room)
                {
                    return E_INVALIDARG;
                }
                *params = p;
                dsp.set_config(reverb_config_from_fxreverb(params, self.sample_rate));
                S_OK
            }
            BuiltinXapoKind::FxEcho(echo) => {
                if ptr.is_null() {
                    return E_POINTER;
                }
                if (byte_size as usize) != std::mem::size_of::<FxEchoParameters>() {
                    return E_INVALIDARG;
                }
                let p = std::ptr::read_unaligned(ptr as *const FxEchoParameters);
                if !echo.validate(&p) {
                    return E_INVALIDARG;
                }
                echo.params = p;
                S_OK
            }
        }
    }

    unsafe fn get_parameters(&self, ptr: *mut c_void, byte_size: u32) -> i32 {
        if ptr.is_null() {
            return E_POINTER;
        }
        match &self.kind {
            BuiltinXapoKind::AudioReverb { params, .. } => {
                let sz = byte_size as usize;
                if sz == std::mem::size_of::<XAudio2FxReverbParameters>() {
                    std::ptr::write_unaligned(ptr as *mut XAudio2FxReverbParameters, *params);
                    S_OK
                } else if sz == std::mem::size_of::<XAudio2FxReverbParameters28>() {
                    let p28 = XAudio2FxReverbParameters28 {
                        wet_dry_mix: params.wet_dry_mix,
                        reflections_delay: params.reflections_delay,
                        reverb_delay: params.reverb_delay,
                        rear_delay: params.rear_delay,
                        position_left: params.position_left,
                        position_right: params.position_right,
                        position_matrix_left: params.position_matrix_left,
                        position_matrix_right: params.position_matrix_right,
                        early_diffusion: params.early_diffusion,
                        late_diffusion: params.late_diffusion,
                        low_eq_gain: params.low_eq_gain,
                        low_eq_cutoff: params.low_eq_cutoff,
                        high_eq_gain: params.high_eq_gain,
                        high_eq_cutoff: params.high_eq_cutoff,
                        room_filter_freq: params.room_filter_freq,
                        room_filter_main: params.room_filter_main,
                        room_filter_hf: params.room_filter_hf,
                        reflections_gain: params.reflections_gain,
                        reverb_gain: params.reverb_gain,
                        decay_time: params.decay_time,
                        density: params.density,
                        room_size: params.room_size,
                        disable_late_field: params.disable_late_field,
                    };
                    std::ptr::write_unaligned(ptr as *mut XAudio2FxReverbParameters28, p28);
                    S_OK
                } else if sz == std::mem::size_of::<XAudio2FxReverbParameters27>() {
                    let p27 = XAudio2FxReverbParameters27 {
                        wet_dry_mix: params.wet_dry_mix,
                        reflections_delay: params.reflections_delay,
                        reverb_delay: params.reverb_delay,
                        rear_delay: params.rear_delay,
                        position_left: params.position_left,
                        position_right: params.position_right,
                        position_matrix_left: params.position_matrix_left,
                        position_matrix_right: params.position_matrix_right,
                        early_diffusion: params.early_diffusion,
                        late_diffusion: params.late_diffusion,
                        low_eq_gain: params.low_eq_gain,
                        low_eq_cutoff: params.low_eq_cutoff,
                        high_eq_gain: params.high_eq_gain,
                        high_eq_cutoff: params.high_eq_cutoff,
                        room_filter_freq: params.room_filter_freq,
                        room_filter_main: params.room_filter_main,
                        room_filter_hf: params.room_filter_hf,
                        reflections_gain: params.reflections_gain,
                        reverb_gain: params.reverb_gain,
                        decay_time: params.decay_time,
                        density: params.density,
                        room_size: params.room_size,
                    };
                    std::ptr::write_unaligned(ptr as *mut XAudio2FxReverbParameters27, p27);
                    S_OK
                } else {
                    E_INVALIDARG
                }
            }
            BuiltinXapoKind::VolumeMeter(vm) => {
                if (byte_size as usize) < std::mem::size_of::<XAudio2FxVolumeMeterLevels>() {
                    return E_INVALIDARG;
                }
                let levels = std::ptr::read_unaligned(ptr as *const XAudio2FxVolumeMeterLevels);
                let count = levels.channel_count as usize;
                if count == 0 || count > XAUDIO2_MAX_AUDIO_CHANNELS as usize {
                    return E_INVALIDARG;
                }
                if self.locked && count != vm.channels {
                    return E_INVALIDARG;
                }
                if !levels.peak_levels.is_null() {
                    for i in 0..count {
                        let v = vm.peak.get(i).copied().unwrap_or(0.0);
                        std::ptr::write_unaligned(levels.peak_levels.add(i), v);
                    }
                }
                if !levels.rms_levels.is_null() {
                    for i in 0..count {
                        let v = vm.rms.get(i).copied().unwrap_or(0.0);
                        std::ptr::write_unaligned(levels.rms_levels.add(i), v);
                    }
                }
                S_OK
            }
            BuiltinXapoKind::FxEq(eq) => {
                if (byte_size as usize) != std::mem::size_of::<FxEqParameters>() {
                    return E_INVALIDARG;
                }
                std::ptr::write_unaligned(ptr as *mut FxEqParameters, eq.params);
                S_OK
            }
            BuiltinXapoKind::FxMasteringLimiter(lim) => {
                if (byte_size as usize) != std::mem::size_of::<FxMasteringLimiterParameters>() {
                    return E_INVALIDARG;
                }
                std::ptr::write_unaligned(ptr as *mut FxMasteringLimiterParameters, lim.params);
                S_OK
            }
            BuiltinXapoKind::FxReverb { params, .. } => {
                if (byte_size as usize) != std::mem::size_of::<FxReverbParameters>() {
                    return E_INVALIDARG;
                }
                std::ptr::write_unaligned(ptr as *mut FxReverbParameters, *params);
                S_OK
            }
            BuiltinXapoKind::FxEcho(echo) => {
                if (byte_size as usize) != std::mem::size_of::<FxEchoParameters>() {
                    return E_INVALIDARG;
                }
                std::ptr::write_unaligned(ptr as *mut FxEchoParameters, echo.params);
                S_OK
            }
        }
    }
}

/// Dual-interface COM object implementing both `IXAPO` (at offset 0) and
/// `IXAPOParameters` (at `offset_of!(BuiltinXapoCom, params_vtbl)`).
#[repr(C)]
struct BuiltinXapoCom {
    #[allow(dead_code)]
    xapo_vtbl: *const [crate::Ptr; 13],
    #[allow(dead_code)]
    params_vtbl: *const [crate::Ptr; 5],
    magic: u64,
    refs: AtomicU32,
    inner: Arc<Mutex<BuiltinXapoState>>,
}

impl BuiltinXapoCom {
    fn alloc(kind: BuiltinXapoKind) -> *mut Self {
        ensure_vtbls_registered();
        Box::into_raw(Box::new(Self {
            xapo_vtbl: &XAPO_VTBL.0,
            params_vtbl: &XAPO_PARAMS_VTBL.0,
            magic: BUILTIN_XAPO_MAGIC,
            refs: AtomicU32::new(1),
            inner: Arc::new(Mutex::new(BuiltinXapoState::new(kind))),
        }))
    }
}

#[inline]
unsafe fn xapo_params_interface_ptr(base: *mut BuiltinXapoCom) -> *mut c_void {
    (base as *mut u8).add(std::mem::offset_of!(BuiltinXapoCom, params_vtbl)) as *mut c_void
}

#[inline]
unsafe fn xapo_from_params_ptr(this_params: *mut c_void) -> *mut BuiltinXapoCom {
    if this_params.is_null() {
        return std::ptr::null_mut();
    }
    (this_params as *mut u8).sub(std::mem::offset_of!(BuiltinXapoCom, params_vtbl))
        as *mut BuiltinXapoCom
}

unsafe fn try_identify_builtin_xapo(p_unknown: *mut c_void) -> Option<*mut BuiltinXapoCom> {
    if p_unknown.is_null() {
        return None;
    }
    let vtbl = std::ptr::read_unaligned(p_unknown as *const usize);
    if vtbl == XAPO_VTBL.0.as_ptr() as usize {
        let base = p_unknown as *mut BuiltinXapoCom;
        if std::ptr::read_unaligned(std::ptr::addr_of!((*base).magic)) == BUILTIN_XAPO_MAGIC {
            return Some(base);
        }
    } else if vtbl == XAPO_PARAMS_VTBL.0.as_ptr() as usize {
        let base = xapo_from_params_ptr(p_unknown);
        if std::ptr::read_unaligned(std::ptr::addr_of!((*base).magic)) == BUILTIN_XAPO_MAGIC {
            return Some(base);
        }
    }
    None
}

unsafe fn xapo_qi_common(
    base: *mut BuiltinXapoCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> i32 {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    if base.is_null() {
        return E_NOINTERFACE;
    }
    let Some(iid) = GUID::read(riid) else {
        return E_POINTER;
    };
    let obj = &*base;
    if iid == IID_IUnknown || iid == IID_IXAPO {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, base as *mut c_void);
        return S_OK;
    }
    if iid == IID_IXAPO_PARAMETERS {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, xapo_params_interface_ptr(base));
        return S_OK;
    }
    E_NOINTERFACE
}

unsafe extern "win64" fn xapo_query_interface(
    this: *mut BuiltinXapoCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> i32 {
    xapo_qi_common(this, riid, ppv)
}

unsafe extern "win64" fn xapo_add_ref(this: *mut BuiltinXapoCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn xapo_release(this: *mut BuiltinXapoCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let prev = {
        let obj = &*this;
        obj.refs.fetch_sub(1, Ordering::AcqRel)
    };
    if prev == 1 {
        drop(Box::from_raw(this));
        0
    } else {
        prev - 1
    }
}

unsafe extern "win64" fn xapo_get_registration_properties(
    this: *mut BuiltinXapoCom,
    pp_props: *mut *mut XapoRegistrationProperties,
) -> i32 {
    if this.is_null() || pp_props.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(pp_props, std::ptr::null_mut());
    let obj = &*this;
    let props = obj.inner.lock().unwrap().registration_properties();
    let mem = crate::audio::com::co_alloc(std::mem::size_of::<XapoRegistrationProperties>())
        as *mut XapoRegistrationProperties;
    if mem.is_null() {
        return E_OUTOFMEMORY;
    }
    std::ptr::write_unaligned(mem, props);
    std::ptr::write_unaligned(pp_props, mem);
    S_OK
}

unsafe fn xapo_check_format_supported(
    this: *mut BuiltinXapoCom,
    other_format: *const WaveFormatEx,
    requested_format: *const WaveFormatEx,
    pp_supported: *mut *mut WaveFormatEx,
) -> i32 {
    if !pp_supported.is_null() {
        std::ptr::write_unaligned(pp_supported, std::ptr::null_mut());
    }
    if this.is_null() || requested_format.is_null() {
        return E_POINTER;
    }
    let obj = &*this;
    let must_match_ch = obj.inner.lock().unwrap().channels_must_match();
    let other_parsed = if other_format.is_null() {
        None
    } else {
        parse_xapo_float_format(other_format).ok()
    };
    let req_ok = match parse_xapo_float_format(requested_format) {
        Ok((req_ch, req_sr)) => match other_parsed {
            Some((other_ch, other_sr)) => {
                req_sr == other_sr && (!must_match_ch || req_ch == other_ch)
            }
            None => true,
        },
        Err(_) => false,
    };
    if req_ok {
        return S_OK;
    }
    if !pp_supported.is_null() {
        let req_raw = std::ptr::read_unaligned(requested_format);
        let fallback_ch = other_parsed
            .map(|(ch, _)| ch)
            .unwrap_or_else(|| (req_raw.channels as usize).clamp(1, XAUDIO2_MAX_AUDIO_CHANNELS as usize));
        let fallback_sr = other_parsed
            .map(|(_, sr)| sr)
            .unwrap_or_else(|| req_raw.samples_per_sec.clamp(XAUDIO2_MIN_SAMPLE_RATE, XAUDIO2_MAX_SAMPLE_RATE));
        let mem = crate::audio::com::co_alloc(std::mem::size_of::<WaveFormatEx>())
            as *mut WaveFormatEx;
        if !mem.is_null() {
            std::ptr::write_unaligned(mem, make_float_wfx(fallback_ch, fallback_sr));
            std::ptr::write_unaligned(pp_supported, mem);
        }
    }
    XAPO_E_FORMAT_UNSUPPORTED
}

unsafe extern "win64" fn xapo_is_input_format_supported(
    this: *mut BuiltinXapoCom,
    p_output_format: *const WaveFormatEx,
    p_requested_input_format: *const WaveFormatEx,
    pp_supported_input_format: *mut *mut WaveFormatEx,
) -> i32 {
    xapo_check_format_supported(
        this,
        p_output_format,
        p_requested_input_format,
        pp_supported_input_format,
    )
}

unsafe extern "win64" fn xapo_is_output_format_supported(
    this: *mut BuiltinXapoCom,
    p_input_format: *const WaveFormatEx,
    p_requested_output_format: *const WaveFormatEx,
    pp_supported_output_format: *mut *mut WaveFormatEx,
) -> i32 {
    xapo_check_format_supported(
        this,
        p_input_format,
        p_requested_output_format,
        pp_supported_output_format,
    )
}

unsafe extern "win64" fn xapo_initialize(
    this: *mut BuiltinXapoCom,
    p_data: *const c_void,
    data_byte_size: u32,
) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let obj = &*this;
    obj.inner.lock().unwrap().initialize(p_data, data_byte_size)
}

unsafe extern "win64" fn xapo_reset(this: *mut BuiltinXapoCom) {
    if this.is_null() {
        return;
    }
    let obj = &*this;
    obj.inner.lock().unwrap().reset();
}

unsafe extern "win64" fn xapo_lock_for_process(
    this: *mut BuiltinXapoCom,
    in_count: u32,
    p_in_params: *const XapoLockForProcessBufferParameters,
    out_count: u32,
    p_out_params: *const XapoLockForProcessBufferParameters,
) -> i32 {
    if this.is_null() || p_in_params.is_null() || p_out_params.is_null() {
        return E_POINTER;
    }
    if in_count != 1 || out_count != 1 {
        return E_INVALIDARG;
    }
    let in_p = std::ptr::read_unaligned(p_in_params);
    let out_p = std::ptr::read_unaligned(p_out_params);
    let Ok((in_ch, in_sr)) = parse_xapo_float_format(in_p.format) else {
        return XAPO_E_FORMAT_UNSUPPORTED;
    };
    let Ok((out_ch, out_sr)) = parse_xapo_float_format(out_p.format) else {
        return XAPO_E_FORMAT_UNSUPPORTED;
    };
    if in_sr != out_sr {
        return XAPO_E_FORMAT_UNSUPPORTED;
    }
    let obj = &*this;
    obj.inner.lock().unwrap().lock_for_process(in_sr, in_ch, out_ch)
}

unsafe extern "win64" fn xapo_unlock_for_process(this: *mut BuiltinXapoCom) {
    if this.is_null() {
        return;
    }
    let obj = &*this;
    obj.inner.lock().unwrap().unlock_for_process();
}

unsafe extern "win64" fn xapo_process(
    this: *mut BuiltinXapoCom,
    in_count: u32,
    p_in_params: *const XapoProcessBufferParameters,
    out_count: u32,
    p_out_params: *mut XapoProcessBufferParameters,
    is_enabled: i32,
) {
    if this.is_null()
        || in_count == 0
        || out_count == 0
        || p_in_params.is_null()
        || p_out_params.is_null()
    {
        return;
    }
    let in_p = std::ptr::read_unaligned(p_in_params);
    let mut out_p = std::ptr::read_unaligned(p_out_params);
    let frames = in_p.valid_frame_count as usize;
    let in_silent = in_p.buffer_flags == XAPO_BUFFER_SILENT;
    let enabled = is_enabled != 0;

    let obj = &*this;
    let mut st = obj.inner.lock().unwrap();
    let in_ch = st.in_channels;
    let out_ch = st.out_channels;

    let out_valid = if frames == 0 || out_p.buffer.is_null() || (!in_silent && in_p.buffer.is_null())
    {
        false
    } else if in_p.buffer == out_p.buffer && in_ch == out_ch {
        let buf = std::slice::from_raw_parts_mut(out_p.buffer as *mut f32, frames * out_ch);
        st.process_in_place(buf, out_ch, frames, in_silent, enabled)
    } else {
        let src_slice = if in_silent || in_p.buffer.is_null() {
            &[]
        } else {
            std::slice::from_raw_parts(in_p.buffer as *const f32, frames * in_ch)
        };
        let dst_slice = std::slice::from_raw_parts_mut(out_p.buffer as *mut f32, frames * out_ch);
        st.process_out_of_place(src_slice, in_ch, dst_slice, out_ch, frames, in_silent, enabled)
    };

    out_p.valid_frame_count = in_p.valid_frame_count;
    out_p.buffer_flags = if out_valid {
        XAPO_BUFFER_VALID
    } else {
        XAPO_BUFFER_SILENT
    };
    std::ptr::write_unaligned(p_out_params, out_p);
}

unsafe extern "win64" fn xapo_calc_input_frames(
    _this: *mut BuiltinXapoCom,
    output_frame_count: u32,
) -> u32 {
    output_frame_count
}

unsafe extern "win64" fn xapo_calc_output_frames(
    _this: *mut BuiltinXapoCom,
    input_frame_count: u32,
) -> u32 {
    input_frame_count
}

unsafe extern "win64" fn xapo_params_query_interface(
    this_params: *mut c_void,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> i32 {
    xapo_qi_common(xapo_from_params_ptr(this_params), riid, ppv)
}

unsafe extern "win64" fn xapo_params_add_ref(this_params: *mut c_void) -> u32 {
    xapo_add_ref(xapo_from_params_ptr(this_params))
}

unsafe extern "win64" fn xapo_params_release(this_params: *mut c_void) -> u32 {
    xapo_release(xapo_from_params_ptr(this_params))
}

unsafe extern "win64" fn xapo_params_set_parameters(
    this_params: *mut c_void,
    p_params: *const c_void,
    byte_size: u32,
) {
    let base = xapo_from_params_ptr(this_params);
    if base.is_null() {
        return;
    }
    let obj = &*base;
    let _ = obj.inner.lock().unwrap().set_parameters(p_params, byte_size);
}

unsafe extern "win64" fn xapo_params_get_parameters(
    this_params: *mut c_void,
    p_params: *mut c_void,
    byte_size: u32,
) {
    let base = xapo_from_params_ptr(this_params);
    if base.is_null() {
        return;
    }
    let obj = &*base;
    let _ = obj.inner.lock().unwrap().get_parameters(p_params, byte_size);
}

// --- Voice Effect Chain Attachments (Builtin Fast-Path + Arbitrary Guest COM IXAPO) ---

struct BuiltinXapoAttachment {
    com_ptr: usize,
    state: Arc<Mutex<BuiltinXapoState>>,
}

impl Drop for BuiltinXapoAttachment {
    fn drop(&mut self) {
        self.state.lock().unwrap().unlock_for_process();
        if self.com_ptr != 0 {
            unsafe {
                xapo_release(self.com_ptr as *mut BuiltinXapoCom);
            }
        }
    }
}

struct GuestXapoHandle {
    xapo_ptr: usize,
    params_ptr: usize,
    process_lock: Mutex<()>,
}

impl Drop for GuestXapoHandle {
    fn drop(&mut self) {
        let _g = self.process_lock.lock().unwrap();
        unsafe {
            if self.xapo_ptr != 0 {
                let unlock_fn = read_vtable_slot(self.xapo_ptr, 9);
                if unlock_fn != 0 {
                    crate::call_guest8(unlock_fn, self.xapo_ptr as u64, 0, 0, 0, 0, 0, 0, 0);
                }
            }
            if self.params_ptr != 0 {
                let rel_fn = read_vtable_slot(self.params_ptr, 2);
                if rel_fn != 0 {
                    crate::call_guest8(rel_fn, self.params_ptr as u64, 0, 0, 0, 0, 0, 0, 0);
                }
            }
            if self.xapo_ptr != 0 {
                let rel_fn = read_vtable_slot(self.xapo_ptr, 2);
                if rel_fn != 0 {
                    crate::call_guest8(rel_fn, self.xapo_ptr as u64, 0, 0, 0, 0, 0, 0, 0);
                }
            }
        }
    }
}

#[derive(Clone)]
enum EffectBackend {
    Builtin(Arc<BuiltinXapoAttachment>),
    Guest(Arc<GuestXapoHandle>),
}

struct VoiceEffectSlot {
    backend: EffectBackend,
    enabled: bool,
    in_channels: usize,
    output_channels: usize,
}

impl VoiceEffectSlot {
    fn has_active_tail(&self) -> bool {
        if !self.enabled {
            return false;
        }
        match &self.backend {
            EffectBackend::Builtin(b) => b.state.lock().unwrap().has_active_tail(),
            EffectBackend::Guest(_) => true,
        }
    }
}

/// Build and lock an entire XAudio2 effect chain strictly OUTSIDE `EngineCore.state` lock so
/// guest `QueryInterface`/`LockForProcess` calls can never deadlock against XAudio2 state.
unsafe fn build_effect_chain(
    effect_chain: *const XAudio2EffectChain,
    voice_in_channels: usize,
    sample_rate: u32,
    is_master: bool,
) -> Result<Vec<VoiceEffectSlot>, i32> {
    if effect_chain.is_null() {
        return Ok(Vec::new());
    }
    let chain = std::ptr::read_unaligned(effect_chain);
    let count = chain.effect_count as usize;
    if count == 0 {
        return Ok(Vec::new());
    }
    if chain.effects.is_null() {
        return Err(E_POINTER);
    }

    let mut descriptors = Vec::with_capacity(count);
    for i in 0..count {
        let desc = std::ptr::read_unaligned(chain.effects.add(i));
        let eff = desc.effect;
        let out_ch = desc.output_channels as usize;
        if eff.is_null() {
            return Err(E_POINTER);
        }
        if out_ch == 0 || out_ch > XAUDIO2_MAX_AUDIO_CHANNELS as usize {
            return Err(XAUDIO2_E_INVALID_CALL);
        }
        descriptors.push((eff, desc.initial_state != 0, out_ch));
    }

    if is_master {
        if let Some(&(_, _, last_out_ch)) = descriptors.last() {
            if last_out_ch != voice_in_channels {
                return Err(XAUDIO2_E_INVALID_CALL);
            }
        }
    }

    let mut slots = Vec::with_capacity(count);
    let mut cur_ch = voice_in_channels;

    for (eff_ptr, initial_enabled, out_ch) in descriptors {
        if let Some(base) = try_identify_builtin_xapo(eff_ptr) {
            let state_arc = {
                let obj = &*base;
                obj.refs.fetch_add(1, Ordering::AcqRel);
                obj.inner.clone()
            };
            let hr = state_arc
                .lock()
                .unwrap()
                .lock_for_process(sample_rate, cur_ch, out_ch);
            if hr < 0 {
                xapo_release(base);
                return Err(hr);
            }
            let att = Arc::new(BuiltinXapoAttachment {
                com_ptr: base as usize,
                state: state_arc,
            });
            slots.push(VoiceEffectSlot {
                backend: EffectBackend::Builtin(att),
                enabled: initial_enabled,
                in_channels: cur_ch,
                output_channels: out_ch,
            });
        } else {
            // Arbitrary guest COM object: QI for IXAPO (required) and IXAPOParameters (optional).
            let qi_fn = read_vtable_slot(eff_ptr as usize, 0);
            if qi_fn == 0 {
                return Err(XAUDIO2_E_INVALID_CALL);
            }
            let mut xapo_ptr: usize = 0;
            let hr_xapo = crate::call_guest8(
                qi_fn,
                eff_ptr as u64,
                &IID_IXAPO as *const GUID as u64,
                &mut xapo_ptr as *mut usize as u64,
                0,
                0,
                0,
                0,
                0,
            ) as i32;
            if hr_xapo < 0 || xapo_ptr == 0 {
                return Err(if hr_xapo < 0 {
                    hr_xapo
                } else {
                    XAUDIO2_E_INVALID_CALL
                });
            }

            let mut params_ptr: usize = 0;
            let hr_params = crate::call_guest8(
                qi_fn,
                eff_ptr as u64,
                &IID_IXAPO_PARAMETERS as *const GUID as u64,
                &mut params_ptr as *mut usize as u64,
                0,
                0,
                0,
                0,
                0,
            ) as i32;
            if hr_params < 0 {
                params_ptr = 0;
            }

            let in_wfx = make_float_wfx(cur_ch, sample_rate);
            let out_wfx = make_float_wfx(out_ch, sample_rate);
            let in_lock = XapoLockForProcessBufferParameters {
                format: &in_wfx,
                max_frame_count: XAPO_DEFAULT_MAX_FRAMES,
            };
            let out_lock = XapoLockForProcessBufferParameters {
                format: &out_wfx,
                max_frame_count: XAPO_DEFAULT_MAX_FRAMES,
            };
            let lock_fn = read_vtable_slot(xapo_ptr, 8);
            let hr_lock = if lock_fn != 0 {
                crate::call_guest8(
                    lock_fn,
                    xapo_ptr as u64,
                    1,
                    &in_lock as *const XapoLockForProcessBufferParameters as u64,
                    1,
                    &out_lock as *const XapoLockForProcessBufferParameters as u64,
                    0,
                    0,
                    0,
                ) as i32
            } else {
                XAUDIO2_E_INVALID_CALL
            };
            if hr_lock < 0 {
                if params_ptr != 0 {
                    let rel = read_vtable_slot(params_ptr, 2);
                    if rel != 0 {
                        crate::call_guest8(rel, params_ptr as u64, 0, 0, 0, 0, 0, 0, 0);
                    }
                }
                let rel = read_vtable_slot(xapo_ptr, 2);
                if rel != 0 {
                    crate::call_guest8(rel, xapo_ptr as u64, 0, 0, 0, 0, 0, 0, 0);
                }
                return Err(hr_lock);
            }

            let handle = Arc::new(GuestXapoHandle {
                xapo_ptr,
                params_ptr,
                process_lock: Mutex::new(()),
            });
            slots.push(VoiceEffectSlot {
                backend: EffectBackend::Guest(handle),
                enabled: initial_enabled,
                in_channels: cur_ch,
                output_channels: out_ch,
            });
        }
        cur_ch = out_ch;
    }

    Ok(slots)
}

/// Execute a voice's XAPO effect chain in order on `buf` (`frames * initial_ch`).
///
/// Updates `buf` in place (resizing if an effect changes the channel count) and returns
/// `(final_channels, output_is_valid)`.
fn process_effect_chain(
    effects: &[VoiceEffectSlot],
    buf: &mut Vec<f32>,
    initial_ch: usize,
    frames: usize,
    in_silent: bool,
    fx_a: &mut AlignedF32Buf,
    fx_b: &mut AlignedF32Buf,
) -> (usize, bool) {
    if effects.is_empty() || frames == 0 {
        return (initial_ch, !in_silent);
    }

    let mut cur_ch = initial_ch;
    let mut cur_valid = !in_silent;
    // Use `fx_a` as current input and `fx_b` as output, ping-ponging across slots.
    let mut active_is_a = true;
    if cur_valid {
        fx_a.copy_from_slice(&buf[..frames * cur_ch]);
    } else {
        fx_a.resize_zero(frames * cur_ch);
    }

    for slot in effects {
        let in_ch = slot.in_channels;
        let out_ch = slot.output_channels;
        if in_ch != cur_ch {
            // Re-adapt defensively if channel count ever diverged.
            let (src_buf, dst_buf) = if active_is_a {
                (&*fx_a, &mut *fx_b)
            } else {
                (&*fx_b, &mut *fx_a)
            };
            dst_buf.resize_zero(frames * in_ch);
            if cur_valid {
                adapt_channels(src_buf.as_slice(), cur_ch, dst_buf.as_mut_slice(), in_ch, frames);
            }
            active_is_a = !active_is_a;
            cur_ch = in_ch;
        }

        match &slot.backend {
            EffectBackend::Builtin(att) => {
                if !slot.enabled && in_ch == out_ch {
                    // Disabled in-place builtin effect: pure pass-through.
                    continue;
                }
                let mut st = att.state.lock().unwrap();
                if in_ch == out_ch {
                    let cur_buf = if active_is_a {
                        fx_a.as_mut_slice()
                    } else {
                        fx_b.as_mut_slice()
                    };
                    cur_valid =
                        st.process_in_place(cur_buf, out_ch, frames, !cur_valid, slot.enabled);
                } else {
                    let (src_buf, dst_buf) = if active_is_a {
                        (&*fx_a, &mut *fx_b)
                    } else {
                        (&*fx_b, &mut *fx_a)
                    };
                    dst_buf.resize_zero(frames * out_ch);
                    cur_valid = st.process_out_of_place(
                        src_buf.as_slice(),
                        in_ch,
                        dst_buf.as_mut_slice(),
                        out_ch,
                        frames,
                        !cur_valid,
                        slot.enabled,
                    );
                    active_is_a = !active_is_a;
                    cur_ch = out_ch;
                }
            }
            EffectBackend::Guest(guest) => {
                let _g = guest.process_lock.lock().unwrap();
                let (in_ptr, out_ptr, switched) = if in_ch == out_ch {
                    let cur = if active_is_a { &mut *fx_a } else { &mut *fx_b };
                    let p = cur.as_mut_ptr() as *mut c_void;
                    (p, p, false)
                } else {
                    let (src_buf, dst_buf) = if active_is_a {
                        (&mut *fx_a, &mut *fx_b)
                    } else {
                        (&mut *fx_b, &mut *fx_a)
                    };
                    dst_buf.resize_zero(frames * out_ch);
                    (
                        src_buf.as_mut_ptr() as *mut c_void,
                        dst_buf.as_mut_ptr() as *mut c_void,
                        true,
                    )
                };
                let in_param = XapoProcessBufferParameters {
                    buffer: in_ptr,
                    buffer_flags: if cur_valid {
                        XAPO_BUFFER_VALID
                    } else {
                        XAPO_BUFFER_SILENT
                    },
                    valid_frame_count: frames as u32,
                };
                let mut out_param = XapoProcessBufferParameters {
                    buffer: out_ptr,
                    buffer_flags: if cur_valid {
                        XAPO_BUFFER_VALID
                    } else {
                        XAPO_BUFFER_SILENT
                    },
                    valid_frame_count: frames as u32,
                };
                unsafe {
                    let proc_fn = read_vtable_slot(guest.xapo_ptr, 10);
                    if proc_fn != 0 {
                        crate::call_guest8(
                            proc_fn,
                            guest.xapo_ptr as u64,
                            1,
                            &in_param as *const XapoProcessBufferParameters as u64,
                            1,
                            &mut out_param as *mut XapoProcessBufferParameters as u64,
                            if slot.enabled { 1 } else { 0 },
                            0,
                            0,
                        );
                    }
                }
                cur_valid = out_param.buffer_flags != XAPO_BUFFER_SILENT;
                if switched {
                    active_is_a = !active_is_a;
                    cur_ch = out_ch;
                }
                if !cur_valid {
                    let cur = if active_is_a { &mut *fx_a } else { &mut *fx_b };
                    cur.as_mut_slice().fill(0.0);
                }
            }
        }
    }

    let final_slice = if active_is_a {
        fx_a.as_slice()
    } else {
        fx_b.as_slice()
    };
    let final_len = frames * cur_ch;
    buf.clear();
    if cur_valid {
        buf.extend_from_slice(&final_slice[..final_len]);
    } else {
        buf.resize(final_len, 0.0);
    }
    (cur_ch, cur_valid)
}

// ============================================================================
// Engine & Voice State
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
enum VoiceKind {
    Mastering = 0,
    Submix = 1,
    Source = 2,
}

#[repr(C)]
struct VoiceHeader {
    #[allow(dead_code)]
    vtbl: *const crate::Ptr,
    id: u64,
    kind: VoiceKind,
    engine: Arc<EngineCore>,
}

#[repr(transparent)]
struct MasteringVoice(VoiceHeader);

#[repr(transparent)]
struct SubmixVoice(#[allow(dead_code)] VoiceHeader);

#[repr(transparent)]
struct SourceVoice(VoiceHeader);

#[repr(C)]
pub struct XAudio2Engine {
    pub vtbl: *const [crate::Ptr; 13],
    refs: AtomicU32,
    core: Arc<EngineCore>,
}

struct QueuedSourceBuffer {
    samples: Vec<f32>,
    play_begin: usize,
    play_end: usize,
    loop_begin: usize,
    loop_end: usize,
    loops_remaining: u32,
    end_of_stream: bool,
    context: usize,
    started: bool,
    pos: f64,
}

struct SourceVoiceState {
    creation_flags: u32,
    format: ParsedWaveFormat,
    max_freq_ratio: f32,
    freq_ratio: f32,
    volume: f32,
    channel_volumes: Vec<f32>,
    filter: VoiceFilter,
    effects: Vec<VoiceEffectSlot>,
    use_default_master_send: bool,
    sends: Vec<VoiceSend>,
    callback: usize,
    callback_alive: Arc<AtomicBool>,
    running: bool,
    play_tails: bool,
    discontinuity: bool,
    samples_played: u64,
    samples_played_frac: f64,
    queue: VecDeque<QueuedSourceBuffer>,
}

impl SourceVoiceState {
    fn output_channels(&self) -> usize {
        self.effects
            .last()
            .map(|e| e.output_channels)
            .unwrap_or(self.format.channels)
    }
}

struct SubmixVoiceState {
    creation_flags: u32,
    channels: usize,
    sample_rate: u32,
    processing_stage: u32,
    volume: f32,
    channel_volumes: Vec<f32>,
    filter: VoiceFilter,
    effects: Vec<VoiceEffectSlot>,
    use_default_master_send: bool,
    sends: Vec<VoiceSend>,
    mix_buf: Vec<f32>,
}

impl SubmixVoiceState {
    fn output_channels(&self) -> usize {
        self.effects
            .last()
            .map(|e| e.output_channels)
            .unwrap_or(self.channels)
    }
}

struct MasteringVoiceState {
    id: u64,
    creation_flags: u32,
    channels: usize,
    sample_rate: u32,
    channel_mask: u32,
    volume: f32,
    channel_volumes: Vec<f32>,
    filter: VoiceFilter,
    effects: Vec<VoiceEffectSlot>,
}

struct EngineState {
    master: Option<MasteringVoiceState>,
    submixes: HashMap<u64, SubmixVoiceState>,
    sources: HashMap<u64, SourceVoiceState>,
    engine_callbacks: Vec<usize>,
    source_glitches: u32,
    master_buf: Vec<f32>,
    source_scratch: Vec<f32>,
    send_scratch: Vec<f32>,
    submix_scratch: Vec<f32>,
    fx_buf_a: AlignedF32Buf,
    fx_buf_b: AlignedF32Buf,
}

impl EngineState {
    fn new() -> Self {
        Self {
            master: None,
            submixes: HashMap::new(),
            sources: HashMap::new(),
            engine_callbacks: Vec::new(),
            source_glitches: 0,
            master_buf: Vec::new(),
            source_scratch: Vec::new(),
            send_scratch: Vec::new(),
            submix_scratch: Vec::new(),
            fx_buf_a: AlignedF32Buf::default(),
            fx_buf_b: AlignedF32Buf::default(),
        }
    }

    fn dest_channels(&self, dest_id: u64) -> Option<usize> {
        if let Some(m) = &self.master {
            if m.id == dest_id {
                return Some(m.channels);
            }
        }
        self.submixes.get(&dest_id).map(|s| s.channels)
    }

    fn sync_default_send(
        master: Option<&MasteringVoiceState>,
        src_ch: usize,
        use_default: bool,
        sends: &mut Vec<VoiceSend>,
    ) {
        if !use_default {
            return;
        }
        let Some(m) = master else {
            return;
        };
        if sends.len() == 1 && sends[0].dest_id == m.id && sends[0].dest_channels == m.channels {
            return;
        }
        sends.clear();
        sends.push(VoiceSend::new(m.id, src_ch, m.channels));
    }
}

struct MasteringRuntime {
    stop_flag: Arc<AtomicBool>,
    wake: Arc<(Mutex<()>, Condvar)>,
    shared: Arc<StreamShared>,
    control: StreamControl,
    mixer_thread: Option<std::thread::JoinHandle<()>>,
    stream: Option<OutputStream>,
}

impl MasteringRuntime {
    fn shutdown(mut self) {
        self.stop_flag.store(true, Ordering::Release);
        {
            let _g = self.wake.0.lock().unwrap();
            self.wake.1.notify_all();
        }
        if let Some(t) = self.mixer_thread.take() {
            let _ = t.join();
        }
        self.control.stop();
        drop(self.stream.take());
    }
}

struct EngineCore {
    next_voice_id: AtomicU64,
    running: AtomicBool,
    state: Mutex<EngineState>,
    runtime: Mutex<Option<MasteringRuntime>>,
    callbacks: Mutex<Option<CallbackDispatcher>>,
}

impl EngineCore {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            next_voice_id: AtomicU64::new(1),
            running: AtomicBool::new(true),
            state: Mutex::new(EngineState::new()),
            runtime: Mutex::new(None),
            callbacks: Mutex::new(None),
        })
    }

    fn alloc_voice_id(&self) -> u64 {
        self.next_voice_id.fetch_add(1, Ordering::Relaxed)
    }

    fn dispatch_callbacks(&self, events: Vec<CallbackEvent>) {
        if events.is_empty() {
            return;
        }
        let mut guard = self.callbacks.lock().unwrap();
        if guard.is_none() {
            *guard = CallbackDispatcher::spawn();
        }
        if let Some(disp) = guard.as_ref() {
            let _ = disp.tx.send(CallbackMsg::Events(events));
        }
    }

    fn sync_callbacks(&self) {
        let info = {
            let guard = self.callbacks.lock().unwrap();
            guard
                .as_ref()
                .map(|d| (d.tx.clone(), d.thread_id.clone()))
        };
        if let Some((tx, tid_cell)) = info {
            let on_cb_thread = *tid_cell.lock().unwrap() == Some(std::thread::current().id());
            if !on_cb_thread {
                let (done_tx, done_rx) = mpsc::sync_channel(1);
                if tx.send(CallbackMsg::Barrier(done_tx)).is_ok() {
                    let _ = done_rx.recv_timeout(Duration::from_millis(500));
                }
            }
        }
    }

    fn shutdown_runtime(&self) {
        let rt = self.runtime.lock().unwrap().take();
        if let Some(r) = rt {
            r.shutdown();
        }
    }

    fn shutdown_callbacks(&self) {
        let disp = self.callbacks.lock().unwrap().take();
        drop(disp);
    }

    /// Mix `frames` interleaved f32 frames at the mastering voice's sample rate & channel count
    /// into `out` (`out.len() >= frames * master_channels`).
    ///
    /// Returns any `IXAudio2VoiceCallback` / `IXAudio2EngineCallback` events produced during this
    /// quantum so the caller can dispatch them strictly outside any engine lock.
    fn mix_quantum(&self, frames: usize, out: &mut [f32]) -> Vec<CallbackEvent> {
        let mut events = Vec::new();
        let mut st_guard = self.state.lock().unwrap();
        let st: &mut EngineState = &mut *st_guard;
        let Some(master_info) = st.master.as_ref().map(|m| (m.id, m.channels, m.sample_rate)) else {
            out.fill(0.0);
            return events;
        };
        let (master_id, master_ch, master_rate) = master_info;
        let out_samples = frames * master_ch;
        if out.len() < out_samples || out_samples == 0 {
            return events;
        }

        if !st.engine_callbacks.is_empty() {
            events.push(CallbackEvent::EnginePassStart(st.engine_callbacks.clone()));
        }

        // Split borrows via local extraction of scratch buffers.
        let mut master_buf = std::mem::take(&mut st.master_buf);
        let mut source_scratch = std::mem::take(&mut st.source_scratch);
        let mut send_scratch = std::mem::take(&mut st.send_scratch);
        let mut submix_scratch = std::mem::take(&mut st.submix_scratch);
        let mut fx_buf_a = std::mem::take(&mut st.fx_buf_a);
        let mut fx_buf_b = std::mem::take(&mut st.fx_buf_b);

        master_buf.clear();
        master_buf.resize(out_samples, 0.0);

        // Clear all submix accumulation buffers for this quantum.
        for sub in st.submixes.values_mut() {
            let n = frames * sub.channels;
            sub.mix_buf.clear();
            sub.mix_buf.resize(n, 0.0);
        }

        // 1. Mix all active SourceVoices into their target submixes / mastering bus.
        let source_ids: Vec<u64> = st.sources.keys().copied().collect();
        for sid in source_ids {
            let Some(src) = st.sources.get_mut(&sid) else {
                continue;
            };
            let has_enabled_fx = src.effects.iter().any(|e| e.enabled);
            let has_tail = src.effects.iter().any(|e| e.has_active_tail());
            let run_audio = src.running && !src.queue.is_empty();
            let run_silent_fx =
                !run_audio && has_enabled_fx && (src.running || (src.play_tails && has_tail));
            if !run_audio && !run_silent_fx {
                continue;
            }
            let out_ch_expected = src.output_channels();
            EngineState::sync_default_send(
                st.master.as_ref(),
                out_ch_expected,
                src.use_default_master_send,
                &mut src.sends,
            );

            let src_ch = src.format.channels;
            let step = (src.format.sample_rate as f64 / master_rate.max(1) as f64)
                * (src.freq_ratio as f64);
            let total_src_samples = frames * src_ch;
            source_scratch.clear();
            source_scratch.resize(total_src_samples, 0.0);

            let mut produced_any = false;
            if run_audio {
                for f in 0..frames {
                    // Pop any empty/finished buffers and emit lifecycle events before reading.
                    let has_audio = loop {
                        let Some(front) = src.queue.front_mut() else {
                            break false;
                        };
                        if !front.started {
                            front.started = true;
                            if src.callback != 0 {
                                events.push(CallbackEvent::VoiceBufferStart {
                                    cb: src.callback,
                                    alive: src.callback_alive.clone(),
                                    ctx: front.context,
                                });
                            }
                        }
                        if front.play_begin >= front.play_end || front.pos >= front.play_end as f64
                        {
                            let overshoot = if front.play_end > front.play_begin {
                                (front.pos - front.play_end as f64).max(0.0)
                            } else {
                                0.0
                            };
                            let finished = src.queue.pop_front().unwrap();
                            if finished.end_of_stream {
                                src.discontinuity = true;
                            }
                            if src.callback != 0 {
                                events.push(CallbackEvent::VoiceBufferEnd {
                                    cb: src.callback,
                                    alive: src.callback_alive.clone(),
                                    ctx: finished.context,
                                });
                                if finished.end_of_stream {
                                    events.push(CallbackEvent::VoiceStreamEnd {
                                        cb: src.callback,
                                        alive: src.callback_alive.clone(),
                                    });
                                }
                            }
                            if let Some(next) = src.queue.front_mut() {
                                next.pos = (next.play_begin as f64) + overshoot;
                            }
                            continue;
                        }
                        break true;
                    };

                    if !has_audio {
                        if !src.discontinuity {
                            st.source_glitches = st.source_glitches.saturating_add(1);
                            src.discontinuity = true;
                        }
                        break;
                    }

                    produced_any = true;
                    let front = src.queue.front_mut().unwrap();
                    let i0 = (front.pos.floor() as usize).clamp(front.play_begin, front.play_end - 1);
                    let frac = (front.pos - i0 as f64).clamp(0.0, 1.0) as f32;
                    let i1 = if front.loops_remaining > 0 && i0 + 1 >= front.loop_end {
                        front.loop_begin
                    } else if i0 + 1 < front.play_end {
                        i0 + 1
                    } else {
                        i0
                    };

                    let base0 = i0 * src_ch;
                    let base1 = i1 * src_ch;
                    let out_frame = &mut source_scratch[f * src_ch..(f + 1) * src_ch];
                    for c in 0..src_ch {
                        let s0 = front.samples[base0 + c];
                        let s1 = front.samples[base1 + c];
                        out_frame[c] = s0 + (s1 - s0) * frac;
                    }

                    front.pos += step;
                    src.samples_played_frac += step;
                    let whole = src.samples_played_frac.floor() as u64;
                    if whole > 0 {
                        src.samples_played += whole;
                        src.samples_played_frac -= whole as f64;
                    }

                    // Handle loop wrap-around.
                    if front.loops_remaining > 0
                        && front.loop_end > front.loop_begin
                        && front.pos >= front.loop_end as f64
                    {
                        let loop_len = (front.loop_end - front.loop_begin) as f64;
                        let wraps_f = ((front.pos - front.loop_begin as f64) / loop_len).floor();
                        let needed = wraps_f.max(1.0) as u32;
                        if front.loops_remaining == XAUDIO2_LOOP_INFINITE {
                            front.pos -= wraps_f * loop_len;
                            if src.callback != 0 {
                                events.push(CallbackEvent::VoiceLoopEnd {
                                    cb: src.callback,
                                    alive: src.callback_alive.clone(),
                                    ctx: front.context,
                                });
                            }
                        } else {
                            let actual = needed.min(front.loops_remaining);
                            front.pos -= (actual as f64) * loop_len;
                            front.loops_remaining -= actual;
                            if src.callback != 0 {
                                for _ in 0..actual {
                                    events.push(CallbackEvent::VoiceLoopEnd {
                                        cb: src.callback,
                                        alive: src.callback_alive.clone(),
                                        ctx: front.context,
                                    });
                                }
                            }
                        }
                    }

                    // Finish buffer immediately if cursor crossed play_end on the final frame.
                    if front.pos >= front.play_end as f64 {
                        let overshoot = (front.pos - front.play_end as f64).max(0.0);
                        let finished = src.queue.pop_front().unwrap();
                        if finished.end_of_stream {
                            src.discontinuity = true;
                        }
                        if src.callback != 0 {
                            events.push(CallbackEvent::VoiceBufferEnd {
                                cb: src.callback,
                                alive: src.callback_alive.clone(),
                                ctx: finished.context,
                            });
                            if finished.end_of_stream {
                                events.push(CallbackEvent::VoiceStreamEnd {
                                    cb: src.callback,
                                    alive: src.callback_alive.clone(),
                                });
                            }
                        }
                        if let Some(next) = src.queue.front_mut() {
                            next.pos = (next.play_begin as f64) + overshoot;
                        }
                    }
                }
            }

            if !produced_any && !has_enabled_fx {
                continue;
            }

            // Source voice pipeline: built-in filter -> effect chain -> volume & channel volumes -> sends.
            if produced_any {
                let rate_scale = (src.format.sample_rate as f32) / (master_rate.max(1) as f32);
                src.filter.process(&mut source_scratch, src_ch, rate_scale);
            }
            let (eff_ch, eff_valid) = if !src.effects.is_empty() {
                process_effect_chain(
                    &src.effects,
                    &mut source_scratch,
                    src_ch,
                    frames,
                    !produced_any,
                    &mut fx_buf_a,
                    &mut fx_buf_b,
                )
            } else {
                (src_ch, produced_any)
            };

            if !eff_valid {
                continue;
            }

            let vol = src.volume;
            for frame in source_scratch.chunks_exact_mut(eff_ch) {
                for (c, s) in frame.iter_mut().enumerate() {
                    let ch_v = src.channel_volumes.get(c).copied().unwrap_or(1.0);
                    *s *= vol * ch_v;
                }
            }

            // Route to each send target (extract sends temporarily to avoid aliasing `st.submixes`).
            let mut sends = std::mem::take(&mut src.sends);
            for send in &mut sends {
                if send.dest_id == master_id {
                    accumulate_send(
                        send,
                        &source_scratch,
                        eff_ch,
                        &mut master_buf,
                        master_ch,
                        frames,
                        &mut send_scratch,
                    );
                } else if let Some(sub) = st.submixes.get_mut(&send.dest_id) {
                    let dst_ch = sub.channels;
                    accumulate_send(
                        send,
                        &source_scratch,
                        eff_ch,
                        &mut sub.mix_buf,
                        dst_ch,
                        frames,
                        &mut send_scratch,
                    );
                }
            }
            if let Some(src_back) = st.sources.get_mut(&sid) {
                src_back.sends = sends;
            }
        }

        // 2. Process SubmixVoices in ascending `(processing_stage, id)` order.
        let mut submix_order: Vec<(u32, u64)> = st
            .submixes
            .iter()
            .map(|(&id, s)| (s.processing_stage, id))
            .collect();
        submix_order.sort_unstable();

        for (stage, sub_id) in submix_order {
            let (out_ch, ch_vols, mut sends, eff_valid) = {
                let Some(sub) = st.submixes.get_mut(&sub_id) else {
                    continue;
                };
                let sub_out_ch = sub.output_channels();
                EngineState::sync_default_send(
                    st.master.as_ref(),
                    sub_out_ch,
                    sub.use_default_master_send,
                    &mut sub.sends,
                );
                // Submix pipeline: overall volume -> filter -> effect chain -> channel volumes -> sends.
                let vol = sub.volume;
                let mut sub_in_silent = true;
                for s in &mut sub.mix_buf {
                    *s *= vol;
                    if s.abs() > 1e-20 {
                        sub_in_silent = false;
                    }
                }
                let rate_scale = (sub.sample_rate as f32) / (master_rate.max(1) as f32);
                sub.filter.process(&mut sub.mix_buf, sub.channels, rate_scale);
                submix_scratch.clear();
                submix_scratch.extend_from_slice(&sub.mix_buf);
                let (eff_ch, valid) = if !sub.effects.is_empty() {
                    process_effect_chain(
                        &sub.effects,
                        &mut submix_scratch,
                        sub.channels,
                        frames,
                        sub_in_silent,
                        &mut fx_buf_a,
                        &mut fx_buf_b,
                    )
                } else {
                    (sub.channels, !sub_in_silent)
                };
                (
                    eff_ch,
                    sub.channel_volumes.clone(),
                    std::mem::take(&mut sub.sends),
                    valid,
                )
            };

            if eff_valid {
                for frame in submix_scratch.chunks_exact_mut(out_ch) {
                    for (c, s) in frame.iter_mut().enumerate() {
                        let ch_v = ch_vols.get(c).copied().unwrap_or(1.0);
                        *s *= ch_v;
                    }
                }

                for send in &mut sends {
                    if send.dest_id == master_id {
                        accumulate_send(
                            send,
                            &submix_scratch,
                            out_ch,
                            &mut master_buf,
                            master_ch,
                            frames,
                            &mut send_scratch,
                        );
                    } else if let Some(dst_sub) = st.submixes.get_mut(&send.dest_id) {
                        if dst_sub.processing_stage > stage {
                            let dst_ch = dst_sub.channels;
                            accumulate_send(
                                send,
                                &submix_scratch,
                                out_ch,
                                &mut dst_sub.mix_buf,
                                dst_ch,
                                frames,
                                &mut send_scratch,
                            );
                        }
                    }
                }
            }

            if let Some(sub_back) = st.submixes.get_mut(&sub_id) {
                sub_back.sends = sends;
            }
        }

        // 3. MasteringVoice pipeline: overall volume -> filter -> effect chain -> channel volumes.
        if let Some(master) = st.master.as_mut() {
            let vol = master.volume;
            let mut master_in_silent = true;
            for s in &mut master_buf[..out_samples] {
                *s *= vol;
                if s.abs() > 1e-20 {
                    master_in_silent = false;
                }
            }
            master.filter.process(&mut master_buf, master_ch, 1.0);
            if !master.effects.is_empty() {
                let _ = process_effect_chain(
                    &master.effects,
                    &mut master_buf,
                    master_ch,
                    frames,
                    master_in_silent,
                    &mut fx_buf_a,
                    &mut fx_buf_b,
                );
            }
            for frame in master_buf[..out_samples].chunks_exact_mut(master_ch) {
                for (c, s) in frame.iter_mut().enumerate() {
                    let ch_v = master.channel_volumes.get(c).copied().unwrap_or(1.0);
                    let v = *s * ch_v;
                    *s = if v.is_finite() { v } else { 0.0 };
                }
            }
        }

        out[..out_samples].copy_from_slice(&master_buf[..out_samples]);

        if !st.engine_callbacks.is_empty() {
            events.push(CallbackEvent::EnginePassEnd(st.engine_callbacks.clone()));
        }

        // Put scratch buffers back for zero-allocation steady-state mixing.
        st.master_buf = master_buf;
        st.source_scratch = source_scratch;
        st.send_scratch = send_scratch;
        st.submix_scratch = submix_scratch;
        st.fx_buf_a = fx_buf_a;
        st.fx_buf_b = fx_buf_b;

        events
    }
}

impl Drop for EngineCore {
    fn drop(&mut self) {
        self.shutdown_runtime();
        self.shutdown_callbacks();
    }
}

fn choose_output_device() -> backend::DeviceInfo {
    let headless = std::env::var("PORT_HEADLESS").is_ok_and(|v| v == "1")
        || std::env::var("AW2_HEADLESS").is_ok_and(|v| v == "1");
    if headless || backend::backend() == Backend::Null {
        return backend::null_device();
    }
    backend::output_devices()
        .into_iter()
        .next()
        .unwrap_or_else(backend::null_device)
}

fn spawn_mastering_runtime(
    engine: &Arc<EngineCore>,
    sample_rate: u32,
    channels: usize,
) -> Result<MasteringRuntime, String> {
    let dev = choose_output_device();
    let io_request: u32 = 512;
    let quantum_frames = (sample_rate as usize / 100).clamp(64, 2048);
    let ring_cap = (2 * backend::MAX_SLICE as usize)
        .max(sample_rate as usize / 4)
        .max(quantum_frames * 8);
    let shared = StreamShared::new(Ring::new(ring_cap, channels));

    let mut stream = match OutputStream::open(&dev, sample_rate, channels as u16, shared.clone(), io_request) {
        Ok(s) => s,
        Err(e) => {
            log::warn!(
                target: "audio",
                "xaudio2: CoreAudio open failed on '{}' ({e}); falling back to Null backend",
                dev.name
            );
            OutputStream::open(
                &backend::null_device(),
                sample_rate,
                channels as u16,
                shared.clone(),
                quantum_frames as u32,
            )?
        }
    };

    let io_frames = stream.io_frames.max(128) as usize;
    let mut control = stream.control();
    if engine.running.load(Ordering::Acquire) {
        if let Err(e) = control.start() {
            log::warn!(
                target: "audio",
                "xaudio2: CoreAudio start failed ({e}); falling back to Null backend"
            );
            drop(stream);
            stream = OutputStream::open(
                &backend::null_device(),
                sample_rate,
                channels as u16,
                shared.clone(),
                quantum_frames as u32,
            )?;
            control = stream.control();
            let _ = control.start();
        }
    }

    let stop_flag = Arc::new(AtomicBool::new(false));
    let wake = Arc::new((Mutex::new(()), Condvar::new()));
    let weak_engine: Weak<EngineCore> = Arc::downgrade(engine);
    let stop_worker = stop_flag.clone();
    let wake_worker = wake.clone();
    let shared_worker = shared.clone();
    let target_buffered = (io_frames * 2 + quantum_frames).min(ring_cap / 2).max(quantum_frames * 2);

    let mixer_thread = std::thread::Builder::new()
        .name("xaudio2-mixer".into())
        .spawn(move || {
            let mut mix_buf = vec![0.0f32; quantum_frames * channels];
            let poll_dur = Duration::from_millis(4);
            while !stop_worker.load(Ordering::Acquire) {
                let Some(eng) = weak_engine.upgrade() else {
                    break;
                };
                if eng.running.load(Ordering::Acquire) {
                    let mut quantums = 0usize;
                    while !stop_worker.load(Ordering::Acquire)
                        && eng.running.load(Ordering::Acquire)
                        && shared_worker.ring.len() + quantum_frames <= target_buffered
                        && quantums < 4
                    {
                        let events = eng.mix_quantum(quantum_frames, &mut mix_buf);
                        let pushed = shared_worker.ring.push(&mix_buf);
                        if !events.is_empty() {
                            eng.dispatch_callbacks(events);
                        }
                        if pushed < quantum_frames {
                            break;
                        }
                        quantums += 1;
                    }
                }
                if stop_worker.load(Ordering::Acquire) {
                    break;
                }
                let g = wake_worker.0.lock().unwrap();
                let _ = wake_worker.1.wait_timeout(g, poll_dur).unwrap();
            }
        })
        .map_err(|e| e.to_string())?;

    log::info!(
        target: "audio",
        "xaudio2: mastering voice opened {} Hz {} ch ({:?}, io={} frames, quantum={} frames)",
        sample_rate,
        channels,
        stream.kind,
        stream.io_frames,
        quantum_frames
    );

    Ok(MasteringRuntime {
        stop_flag,
        wake,
        shared,
        control,
        mixer_thread: Some(mixer_thread),
        stream: Some(stream),
    })
}

// ============================================================================
// Send-List Parsing Helper
// ============================================================================

unsafe fn parse_send_list(
    engine: &Arc<EngineCore>,
    st: &EngineState,
    self_id: Option<u64>,
    self_stage: Option<u32>,
    src_ch: usize,
    send_list: *const XAudio2VoiceSends,
) -> Result<(bool, Vec<VoiceSend>), i32> {
    if send_list.is_null() {
        let mut sends = Vec::new();
        EngineState::sync_default_send(st.master.as_ref(), src_ch, true, &mut sends);
        return Ok((true, sends));
    }
    let list = std::ptr::read_unaligned(send_list);
    if list.send_count == 0 {
        return Ok((false, Vec::new()));
    }
    if list.sends.is_null() {
        return Err(E_POINTER);
    }
    let count = list.send_count as usize;
    let mut sends = Vec::with_capacity(count);
    for i in 0..count {
        let desc = std::ptr::read_unaligned(list.sends.add(i));
        let _flags = desc.flags;
        let dest_ptr = desc.output_voice;
        if dest_ptr.is_null() {
            return Err(XAUDIO2_E_INVALID_CALL);
        }
        let dest_hdr = &*dest_ptr;
        if !Arc::ptr_eq(&dest_hdr.engine, engine) {
            return Err(XAUDIO2_E_INVALID_CALL);
        }
        if dest_hdr.kind == VoiceKind::Source {
            return Err(XAUDIO2_E_INVALID_CALL);
        }
        if self_id == Some(dest_hdr.id) {
            return Err(XAUDIO2_E_INVALID_CALL);
        }
        if let Some(my_stage) = self_stage {
            if let Some(dst_sub) = st.submixes.get(&dest_hdr.id) {
                if dst_sub.processing_stage <= my_stage {
                    return Err(XAUDIO2_E_INVALID_CALL);
                }
            }
        }
        let Some(dst_ch) = st.dest_channels(dest_hdr.id) else {
            return Err(XAUDIO2_E_INVALID_CALL);
        };
        sends.push(VoiceSend::new(dest_hdr.id, src_ch, dst_ch));
    }
    Ok((false, sends))
}

// ============================================================================
// IXAudio2 Methods
// ============================================================================

unsafe extern "win64" fn xa2_query_interface(
    this: *mut XAudio2Engine,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> i32 {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    if this.is_null() {
        return E_NOINTERFACE;
    }
    let Some(iid) = GUID::read(riid) else {
        return E_POINTER;
    };
    if iid == IID_IUnknown || iid == IID_IXAUDIO2 || iid == IID_IXAUDIO2_7 {
        (*this).refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        return S_OK;
    }
    E_NOINTERFACE
}

unsafe extern "win64" fn xa2_add_ref(this: *mut XAudio2Engine) -> u32 {
    if this.is_null() {
        return 0;
    }
    (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn xa2_release(this: *mut XAudio2Engine) -> u32 {
    if this.is_null() {
        return 0;
    }
    let prev = (*this).refs.fetch_sub(1, Ordering::AcqRel);
    if prev == 1 {
        let engine = Box::from_raw(this);
        engine.core.shutdown_runtime();
        engine.core.shutdown_callbacks();
        drop(engine);
        0
    } else {
        prev - 1
    }
}

unsafe extern "win64" fn xa2_register_callbacks(this: *mut XAudio2Engine, cb: usize) -> i32 {
    if this.is_null() || cb == 0 {
        return E_POINTER;
    }
    let eng = &*this;
    let mut st = eng.core.state.lock().unwrap();
    if !st.engine_callbacks.contains(&cb) {
        st.engine_callbacks.push(cb);
    }
    S_OK
}

unsafe extern "win64" fn xa2_unregister_callbacks(this: *mut XAudio2Engine, cb: usize) {
    if this.is_null() || cb == 0 {
        return;
    }
    let eng = &*this;
    {
        let mut st = eng.core.state.lock().unwrap();
        st.engine_callbacks.retain(|&c| c != cb);
    }
    eng.core.sync_callbacks();
}

unsafe extern "win64" fn xa2_create_source_voice(
    this: *mut XAudio2Engine,
    pp_source_voice: *mut *mut SourceVoice,
    source_format: *const WaveFormatEx,
    flags: u32,
    max_freq_ratio: f32,
    callback: usize,
    send_list: *const XAudio2VoiceSends,
    effect_chain: *const XAudio2EffectChain,
) -> i32 {
    if pp_source_voice.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(pp_source_voice, std::ptr::null_mut());
    if this.is_null() {
        return E_POINTER;
    }
    let fmt = match parse_wave_format(source_format) {
        Ok(f) => f,
        Err(hr) => return hr,
    };

    let max_ratio = if !max_freq_ratio.is_finite() || max_freq_ratio == 0.0 {
        XAUDIO2_DEFAULT_FREQ_RATIO
    } else if max_freq_ratio < 1.0 || max_freq_ratio > XAUDIO2_MAX_FREQ_RATIO {
        return XAUDIO2_E_INVALID_CALL;
    } else {
        max_freq_ratio
    };

    let effects = match build_effect_chain(effect_chain, fmt.channels, fmt.sample_rate, false) {
        Ok(fx) => fx,
        Err(hr) => return hr,
    };
    let out_ch = effects
        .last()
        .map(|e| e.output_channels)
        .unwrap_or(fmt.channels);

    let core = {
        let eng = &*this;
        eng.core.clone()
    };
    let voice_id = core.alloc_voice_id();
    {
        let mut st = core.state.lock().unwrap();
        let (use_default, sends) = match parse_send_list(&core, &st, None, None, out_ch, send_list)
        {
            Ok(v) => v,
            Err(hr) => {
                drop(st);
                drop(effects);
                return hr;
            }
        };
        st.sources.insert(
            voice_id,
            SourceVoiceState {
                creation_flags: flags,
                format: fmt,
                max_freq_ratio: max_ratio,
                freq_ratio: 1.0,
                volume: 1.0,
                channel_volumes: vec![1.0; out_ch],
                filter: VoiceFilter::new(fmt.channels),
                effects,
                use_default_master_send: use_default,
                sends,
                callback,
                callback_alive: Arc::new(AtomicBool::new(true)),
                running: false,
                play_tails: false,
                discontinuity: false,
                samples_played: 0,
                samples_played_frac: 0.0,
                queue: VecDeque::with_capacity(16),
            },
        );
    }

    let hdr = Box::new(VoiceHeader {
        vtbl: SOURCE_VOICE_VTBL.0.as_ptr(),
        id: voice_id,
        kind: VoiceKind::Source,
        engine: core,
    });
    std::ptr::write_unaligned(pp_source_voice, Box::into_raw(hdr) as *mut SourceVoice);
    S_OK
}
crate::win64_bridge!(fn xa2_create_source_voice(
    *mut XAudio2Engine,
    *mut *mut SourceVoice,
    *const WaveFormatEx,
    u32,
    f32,
    usize,
    *const XAudio2VoiceSends,
    *const XAudio2EffectChain,
) -> i32);

unsafe extern "win64" fn xa2_create_submix_voice(
    this: *mut XAudio2Engine,
    pp_submix_voice: *mut *mut SubmixVoice,
    input_channels: u32,
    input_sample_rate: u32,
    flags: u32,
    processing_stage: u32,
    send_list: *const XAudio2VoiceSends,
    effect_chain: *const XAudio2EffectChain,
) -> i32 {
    if pp_submix_voice.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(pp_submix_voice, std::ptr::null_mut());
    if this.is_null() {
        return E_POINTER;
    }
    if input_channels == 0 || input_channels > XAUDIO2_MAX_AUDIO_CHANNELS {
        return XAUDIO2_E_INVALID_CALL;
    }
    if !(XAUDIO2_MIN_SAMPLE_RATE..=XAUDIO2_MAX_SAMPLE_RATE).contains(&input_sample_rate) {
        return XAUDIO2_E_INVALID_CALL;
    }
    let ch = input_channels as usize;
    let effects = match build_effect_chain(effect_chain, ch, input_sample_rate, false) {
        Ok(fx) => fx,
        Err(hr) => return hr,
    };
    let out_ch = effects.last().map(|e| e.output_channels).unwrap_or(ch);

    let core = {
        let eng = &*this;
        eng.core.clone()
    };
    let voice_id = core.alloc_voice_id();
    {
        let mut st = core.state.lock().unwrap();
        let (use_default, sends) = match parse_send_list(
            &core,
            &st,
            None,
            Some(processing_stage),
            out_ch,
            send_list,
        ) {
            Ok(v) => v,
            Err(hr) => {
                drop(st);
                drop(effects);
                return hr;
            }
        };
        st.submixes.insert(
            voice_id,
            SubmixVoiceState {
                creation_flags: flags,
                channels: ch,
                sample_rate: input_sample_rate,
                processing_stage,
                volume: 1.0,
                channel_volumes: vec![1.0; out_ch],
                filter: VoiceFilter::new(ch),
                effects,
                use_default_master_send: use_default,
                sends,
                mix_buf: Vec::new(),
            },
        );
    }

    let hdr = Box::new(VoiceHeader {
        vtbl: SUBMIX_VOICE_VTBL.0.as_ptr(),
        id: voice_id,
        kind: VoiceKind::Submix,
        engine: core,
    });
    std::ptr::write_unaligned(pp_submix_voice, Box::into_raw(hdr) as *mut SubmixVoice);
    S_OK
}

unsafe extern "win64" fn xa2_create_mastering_voice(
    this: *mut XAudio2Engine,
    pp_mastering_voice: *mut *mut MasteringVoice,
    input_channels: u32,
    input_sample_rate: u32,
    flags: u32,
    _device_id: *const u16,
    effect_chain: *const XAudio2EffectChain,
    _stream_category: u32,
) -> i32 {
    if pp_mastering_voice.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(pp_mastering_voice, std::ptr::null_mut());
    if this.is_null() {
        return E_POINTER;
    }
    if input_channels > XAUDIO2_MAX_AUDIO_CHANNELS {
        return XAUDIO2_E_INVALID_CALL;
    }
    if input_sample_rate != 0
        && !(XAUDIO2_MIN_SAMPLE_RATE..=XAUDIO2_MAX_SAMPLE_RATE).contains(&input_sample_rate)
    {
        return XAUDIO2_E_INVALID_CALL;
    }
    let core = {
        let eng = &*this;
        eng.core.clone()
    };
    let channels = if input_channels == 0 { 2 } else { input_channels as usize };
    let sample_rate = if input_sample_rate == 0 { 48_000 } else { input_sample_rate };
    let channel_mask = format::default_mask(channels.min(32) as u16);

    if core.state.lock().unwrap().master.is_some() {
        return XAUDIO2_E_INVALID_CALL;
    }

    let effects = match build_effect_chain(effect_chain, channels, sample_rate, true) {
        Ok(fx) => fx,
        Err(hr) => return hr,
    };

    let voice_id = core.alloc_voice_id();
    {
        let mut st = core.state.lock().unwrap();
        if st.master.is_some() {
            drop(st);
            drop(effects);
            return XAUDIO2_E_INVALID_CALL;
        }
        st.master = Some(MasteringVoiceState {
            id: voice_id,
            creation_flags: flags,
            channels,
            sample_rate,
            channel_mask,
            volume: 1.0,
            channel_volumes: vec![1.0; channels],
            filter: VoiceFilter::new(channels),
            effects,
        });
    }

    match spawn_mastering_runtime(&core, sample_rate, channels) {
        Ok(rt) => {
            *core.runtime.lock().unwrap() = Some(rt);
        }
        Err(e) => {
            log::error!(target: "audio", "xaudio2: failed to initialize output stream: {e}");
            let removed = core.state.lock().unwrap().master.take();
            drop(removed);
            return XAUDIO2_E_INVALID_CALL;
        }
    }

    let hdr = Box::new(VoiceHeader {
        vtbl: MASTER_VOICE_VTBL.0.as_ptr(),
        id: voice_id,
        kind: VoiceKind::Mastering,
        engine: core,
    });
    std::ptr::write_unaligned(pp_mastering_voice, Box::into_raw(hdr) as *mut MasteringVoice);
    S_OK
}

unsafe extern "win64" fn xa2_start_engine(this: *mut XAudio2Engine) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let core = &(*this).core;
    core.running.store(true, Ordering::Release);
    let ctrl_and_wake = {
        let rt = core.runtime.lock().unwrap();
        rt.as_ref().map(|r| (r.control.clone(), r.wake.clone()))
    };
    if let Some((ctrl, wake)) = ctrl_and_wake {
        if let Err(e) = ctrl.start() {
            log::warn!(target: "audio", "xaudio2: StartEngine failed to start output stream: {e}");
        }
        let _g = wake.0.lock().unwrap();
        wake.1.notify_all();
    }
    S_OK
}

unsafe extern "win64" fn xa2_stop_engine(this: *mut XAudio2Engine) {
    if this.is_null() {
        return;
    }
    let core = &(*this).core;
    core.running.store(false, Ordering::Release);
    let ctrl = {
        let rt = core.runtime.lock().unwrap();
        rt.as_ref().map(|r| r.control.clone())
    };
    if let Some(c) = ctrl {
        c.stop();
    }
}

unsafe extern "win64" fn xa2_commit_changes(_this: *mut XAudio2Engine, _op_set: u32) -> i32 {
    S_OK
}

unsafe extern "win64" fn xa2_get_perf_data(
    this: *mut XAudio2Engine,
    perf: *mut XAudio2PerformanceData,
) {
    if perf.is_null() {
        return;
    }
    if this.is_null() {
        std::ptr::write_unaligned(perf, XAudio2PerformanceData::default());
        return;
    }
    let core = &(*this).core;
    let (latency, glitches) = {
        let rt = core.runtime.lock().unwrap();
        rt.as_ref()
            .map(|r| {
                (
                    r.shared.ring.len() as u32,
                    r.shared.underrun_frames.load(Ordering::Relaxed) as u32,
                )
            })
            .unwrap_or((0, 0))
    };

    let st = core.state.lock().unwrap();
    let master_rate = st.master.as_ref().map(|m| m.sample_rate).unwrap_or(48_000);
    let total_sources = st.sources.len() as u32;
    let mut active_sources = 0u32;
    let mut active_resamplers = 0u32;
    let mut mem_bytes = 0usize;
    for s in st.sources.values() {
        for b in &s.queue {
            mem_bytes += b.samples.len() * std::mem::size_of::<f32>();
        }
        if s.running && !s.queue.is_empty() {
            active_sources += 1;
            if s.format.sample_rate != master_rate || (s.freq_ratio - 1.0).abs() > 1e-6 {
                active_resamplers += 1;
            }
        }
    }
    let active_submixes = st.submixes.len() as u32;
    std::ptr::write_unaligned(
        perf,
        XAudio2PerformanceData {
            audio_cycles_since_last_query: 0,
            total_cycles_since_last_query: 0,
            minimum_cycles_per_quantum: 0,
            maximum_cycles_per_quantum: 0,
            memory_usage_in_bytes: mem_bytes.min(u32::MAX as usize) as u32,
            current_latency_in_samples: latency,
            glitches_since_engine_started: glitches.saturating_add(st.source_glitches),
            active_source_voice_count: active_sources,
            total_source_voice_count: total_sources,
            active_submix_voice_count: active_submixes,
            active_resampler_count: active_resamplers,
            active_matrix_mix_count: active_sources + active_submixes,
            active_xma_source_voices: 0,
            active_xma_streams: 0,
        },
    );
}

unsafe extern "win64" fn xa2_set_debug_config(
    _this: *mut XAudio2Engine,
    _cfg: usize,
    _reserved: usize,
) {
}

// ============================================================================
// Shared IXAudio2Voice Methods (Mastering, Submix, Source)
// ============================================================================

fn find_send_mut(sends: &mut [VoiceSend], dest_id: Option<u64>) -> Option<&mut VoiceSend> {
    match dest_id {
        Some(id) => sends.iter_mut().find(|s| s.dest_id == id),
        None => {
            if sends.len() == 1 {
                Some(&mut sends[0])
            } else {
                None
            }
        }
    }
}

fn find_send(sends: &[VoiceSend], dest_id: Option<u64>) -> Option<&VoiceSend> {
    match dest_id {
        Some(id) => sends.iter().find(|s| s.dest_id == id),
        None => {
            if sends.len() == 1 {
                Some(&sends[0])
            } else {
                None
            }
        }
    }
}

unsafe fn resolve_dest_id(
    this: &VoiceHeader,
    p_dest: *mut VoiceHeader,
) -> Result<Option<u64>, i32> {
    if p_dest.is_null() {
        return Ok(None);
    }
    let dest = &*p_dest;
    if !Arc::ptr_eq(&dest.engine, &this.engine) {
        return Err(XAUDIO2_E_INVALID_CALL);
    }
    Ok(Some(dest.id))
}

unsafe extern "win64" fn voice_get_voice_details(
    this: *mut VoiceHeader,
    details: *mut XAudio2VoiceDetails,
) {
    if details.is_null() {
        return;
    }
    if this.is_null() {
        std::ptr::write_unaligned(
            details,
            XAudio2VoiceDetails {
                creation_flags: 0,
                active_flags: 0,
                input_channels: 0,
                input_sample_rate: 0,
            },
        );
        return;
    }
    let hdr = &*this;
    let st = hdr.engine.state.lock().unwrap();
    let (flags, ch, sr) = match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_ref()
            .filter(|m| m.id == hdr.id)
            .map(|m| (m.creation_flags, m.channels as u32, m.sample_rate))
            .unwrap_or((0, 2, 48_000)),
        VoiceKind::Submix => st
            .submixes
            .get(&hdr.id)
            .map(|s| (s.creation_flags, s.channels as u32, s.sample_rate))
            .unwrap_or((0, 2, 48_000)),
        VoiceKind::Source => st
            .sources
            .get(&hdr.id)
            .map(|s| (s.creation_flags, s.format.channels as u32, s.format.sample_rate))
            .unwrap_or((0, 2, 48_000)),
    };
    std::ptr::write_unaligned(
        details,
        XAudio2VoiceDetails {
            creation_flags: flags,
            active_flags: flags,
            input_channels: ch,
            input_sample_rate: sr,
        },
    );
}

unsafe extern "win64" fn voice_set_output_voices(
    this: *mut VoiceHeader,
    send_list: *const XAudio2VoiceSends,
) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let hdr = &*this;
    if hdr.kind == VoiceKind::Mastering {
        return XAUDIO2_E_INVALID_CALL;
    }
    let mut st = hdr.engine.state.lock().unwrap();
    match hdr.kind {
        VoiceKind::Source => {
            let Some(src_ch) = st.sources.get(&hdr.id).map(|s| s.output_channels()) else {
                return XAUDIO2_E_INVALID_CALL;
            };
            let (use_default, sends) =
                match parse_send_list(&hdr.engine, &st, Some(hdr.id), None, src_ch, send_list) {
                    Ok(v) => v,
                    Err(hr) => return hr,
                };
            if let Some(s) = st.sources.get_mut(&hdr.id) {
                s.use_default_master_send = use_default;
                s.sends = sends;
            }
            S_OK
        }
        VoiceKind::Submix => {
            let Some((src_ch, stage)) = st
                .submixes
                .get(&hdr.id)
                .map(|s| (s.output_channels(), s.processing_stage))
            else {
                return XAUDIO2_E_INVALID_CALL;
            };
            let (use_default, sends) = match parse_send_list(
                &hdr.engine,
                &st,
                Some(hdr.id),
                Some(stage),
                src_ch,
                send_list,
            ) {
                Ok(v) => v,
                Err(hr) => return hr,
            };
            if let Some(s) = st.submixes.get_mut(&hdr.id) {
                s.use_default_master_send = use_default;
                s.sends = sends;
            }
            S_OK
        }
        VoiceKind::Mastering => XAUDIO2_E_INVALID_CALL,
    }
}

fn resync_voice_output_channels(
    new_out_ch: usize,
    channel_volumes: &mut Vec<f32>,
    sends: &mut [VoiceSend],
) {
    if channel_volumes.len() != new_out_ch {
        channel_volumes.resize(new_out_ch, 1.0);
    }
    for send in sends {
        if send.matrix.len() != new_out_ch * send.dest_channels {
            send.matrix = default_xaudio2_matrix(new_out_ch, send.dest_channels);
        }
    }
}

unsafe extern "win64" fn voice_set_effect_chain(
    this: *mut VoiceHeader,
    effect_chain: *const XAudio2EffectChain,
) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let hdr = &*this;
    let voice_info = {
        let st = hdr.engine.state.lock().unwrap();
        match hdr.kind {
            VoiceKind::Mastering => st
                .master
                .as_ref()
                .filter(|m| m.id == hdr.id)
                .map(|m| (m.channels, m.sample_rate, true)),
            VoiceKind::Submix => st
                .submixes
                .get(&hdr.id)
                .map(|s| (s.channels, s.sample_rate, false)),
            VoiceKind::Source => st
                .sources
                .get(&hdr.id)
                .map(|s| (s.format.channels, s.format.sample_rate, false)),
        }
    };
    let Some((in_ch, sample_rate, is_master)) = voice_info else {
        return XAUDIO2_E_INVALID_CALL;
    };

    let new_effects = match build_effect_chain(effect_chain, in_ch, sample_rate, is_master) {
        Ok(fx) => fx,
        Err(hr) => return hr,
    };
    let new_out_ch = new_effects
        .last()
        .map(|e| e.output_channels)
        .unwrap_or(in_ch);

    let old_effects = {
        let mut st = hdr.engine.state.lock().unwrap();
        match hdr.kind {
            VoiceKind::Mastering => {
                let Some(m) = st.master.as_mut().filter(|m| m.id == hdr.id) else {
                    drop(st);
                    drop(new_effects);
                    return XAUDIO2_E_INVALID_CALL;
                };
                std::mem::replace(&mut m.effects, new_effects)
            }
            VoiceKind::Submix => {
                let Some(s) = st.submixes.get_mut(&hdr.id) else {
                    drop(st);
                    drop(new_effects);
                    return XAUDIO2_E_INVALID_CALL;
                };
                let old = std::mem::replace(&mut s.effects, new_effects);
                resync_voice_output_channels(new_out_ch, &mut s.channel_volumes, &mut s.sends);
                old
            }
            VoiceKind::Source => {
                let Some(s) = st.sources.get_mut(&hdr.id) else {
                    drop(st);
                    drop(new_effects);
                    return XAUDIO2_E_INVALID_CALL;
                };
                let old = std::mem::replace(&mut s.effects, new_effects);
                resync_voice_output_channels(new_out_ch, &mut s.channel_volumes, &mut s.sends);
                old
            }
        }
    };
    // Drop old effects strictly outside EngineCore.state lock.
    drop(old_effects);
    S_OK
}

unsafe extern "win64" fn voice_enable_effect(
    this: *mut VoiceHeader,
    effect_index: u32,
    _op_set: u32,
) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let hdr = &*this;
    let idx = effect_index as usize;
    let mut st = hdr.engine.state.lock().unwrap();
    let slot = match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_mut()
            .filter(|m| m.id == hdr.id)
            .and_then(|m| m.effects.get_mut(idx)),
        VoiceKind::Submix => st
            .submixes
            .get_mut(&hdr.id)
            .and_then(|s| s.effects.get_mut(idx)),
        VoiceKind::Source => st
            .sources
            .get_mut(&hdr.id)
            .and_then(|s| s.effects.get_mut(idx)),
    };
    match slot {
        Some(s) => {
            s.enabled = true;
            S_OK
        }
        None => XAUDIO2_E_INVALID_CALL,
    }
}

unsafe extern "win64" fn voice_disable_effect(
    this: *mut VoiceHeader,
    effect_index: u32,
    _op_set: u32,
) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let hdr = &*this;
    let idx = effect_index as usize;
    let mut st = hdr.engine.state.lock().unwrap();
    let slot = match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_mut()
            .filter(|m| m.id == hdr.id)
            .and_then(|m| m.effects.get_mut(idx)),
        VoiceKind::Submix => st
            .submixes
            .get_mut(&hdr.id)
            .and_then(|s| s.effects.get_mut(idx)),
        VoiceKind::Source => st
            .sources
            .get_mut(&hdr.id)
            .and_then(|s| s.effects.get_mut(idx)),
    };
    match slot {
        Some(s) => {
            s.enabled = false;
            S_OK
        }
        None => XAUDIO2_E_INVALID_CALL,
    }
}

unsafe extern "win64" fn voice_get_effect_state(
    this: *mut VoiceHeader,
    effect_index: u32,
    p_enabled: *mut i32,
) {
    if p_enabled.is_null() {
        return;
    }
    if this.is_null() {
        std::ptr::write_unaligned(p_enabled, 0);
        return;
    }
    let hdr = &*this;
    let idx = effect_index as usize;
    let st = hdr.engine.state.lock().unwrap();
    let enabled = match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_ref()
            .filter(|m| m.id == hdr.id)
            .and_then(|m| m.effects.get(idx))
            .map(|e| e.enabled)
            .unwrap_or(false),
        VoiceKind::Submix => st
            .submixes
            .get(&hdr.id)
            .and_then(|s| s.effects.get(idx))
            .map(|e| e.enabled)
            .unwrap_or(false),
        VoiceKind::Source => st
            .sources
            .get(&hdr.id)
            .and_then(|s| s.effects.get(idx))
            .map(|e| e.enabled)
            .unwrap_or(false),
    };
    std::ptr::write_unaligned(p_enabled, if enabled { 1 } else { 0 });
}

fn clone_voice_effect_backend(hdr: &VoiceHeader, idx: usize) -> Option<EffectBackend> {
    let st = hdr.engine.state.lock().unwrap();
    match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_ref()
            .filter(|m| m.id == hdr.id)
            .and_then(|m| m.effects.get(idx))
            .map(|e| e.backend.clone()),
        VoiceKind::Submix => st
            .submixes
            .get(&hdr.id)
            .and_then(|s| s.effects.get(idx))
            .map(|e| e.backend.clone()),
        VoiceKind::Source => st
            .sources
            .get(&hdr.id)
            .and_then(|s| s.effects.get(idx))
            .map(|e| e.backend.clone()),
    }
}

unsafe extern "win64" fn voice_set_effect_parameters(
    this: *mut VoiceHeader,
    effect_index: u32,
    params: *const c_void,
    byte_size: u32,
    _op_set: u32,
) -> i32 {
    if this.is_null() || (params.is_null() && byte_size > 0) {
        return E_POINTER;
    }
    let hdr = &*this;
    let Some(backend) = clone_voice_effect_backend(hdr, effect_index as usize) else {
        return XAUDIO2_E_INVALID_CALL;
    };
    match backend {
        EffectBackend::Builtin(att) => att.state.lock().unwrap().set_parameters(params, byte_size),
        EffectBackend::Guest(guest) => {
            if guest.params_ptr == 0 {
                return XAUDIO2_E_INVALID_CALL;
            }
            let _g = guest.process_lock.lock().unwrap();
            let set_fn = read_vtable_slot(guest.params_ptr, 3);
            if set_fn == 0 {
                return XAUDIO2_E_INVALID_CALL;
            }
            crate::call_guest8(
                set_fn,
                guest.params_ptr as u64,
                params as u64,
                byte_size as u64,
                0,
                0,
                0,
                0,
                0,
            );
            S_OK
        }
    }
}

unsafe extern "win64" fn voice_get_effect_parameters(
    this: *mut VoiceHeader,
    effect_index: u32,
    params: *mut c_void,
    byte_size: u32,
) -> i32 {
    if this.is_null() || (params.is_null() && byte_size > 0) {
        return E_POINTER;
    }
    let hdr = &*this;
    let Some(backend) = clone_voice_effect_backend(hdr, effect_index as usize) else {
        return XAUDIO2_E_INVALID_CALL;
    };
    match backend {
        EffectBackend::Builtin(att) => att.state.lock().unwrap().get_parameters(params, byte_size),
        EffectBackend::Guest(guest) => {
            if guest.params_ptr == 0 {
                return XAUDIO2_E_INVALID_CALL;
            }
            let _g = guest.process_lock.lock().unwrap();
            let get_fn = read_vtable_slot(guest.params_ptr, 4);
            if get_fn == 0 {
                return XAUDIO2_E_INVALID_CALL;
            }
            crate::call_guest8(
                get_fn,
                guest.params_ptr as u64,
                params as u64,
                byte_size as u64,
                0,
                0,
                0,
                0,
                0,
            );
            S_OK
        }
    }
}

unsafe extern "win64" fn voice_set_filter_parameters(
    this: *mut VoiceHeader,
    p_params: *const XAudio2FilterParameters,
    _op_set: u32,
) -> i32 {
    if this.is_null() || p_params.is_null() {
        return E_POINTER;
    }
    let hdr = &*this;
    let params = std::ptr::read_unaligned(p_params);
    let mut st = hdr.engine.state.lock().unwrap();
    let res = match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_mut()
            .filter(|m| m.id == hdr.id)
            .map(|m| m.filter.set(params))
            .unwrap_or(Err(XAUDIO2_E_INVALID_CALL)),
        VoiceKind::Submix => st
            .submixes
            .get_mut(&hdr.id)
            .map(|s| s.filter.set(params))
            .unwrap_or(Err(XAUDIO2_E_INVALID_CALL)),
        VoiceKind::Source => st
            .sources
            .get_mut(&hdr.id)
            .map(|s| s.filter.set(params))
            .unwrap_or(Err(XAUDIO2_E_INVALID_CALL)),
    };
    match res {
        Ok(()) => S_OK,
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn voice_get_filter_parameters(
    this: *mut VoiceHeader,
    p_params: *mut XAudio2FilterParameters,
) {
    if p_params.is_null() {
        return;
    }
    if this.is_null() {
        std::ptr::write_unaligned(p_params, XAudio2FilterParameters::default());
        return;
    }
    let hdr = &*this;
    let st = hdr.engine.state.lock().unwrap();
    let params = match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_ref()
            .filter(|m| m.id == hdr.id)
            .map(|m| m.filter.params)
            .unwrap_or_default(),
        VoiceKind::Submix => st
            .submixes
            .get(&hdr.id)
            .map(|s| s.filter.params)
            .unwrap_or_default(),
        VoiceKind::Source => st
            .sources
            .get(&hdr.id)
            .map(|s| s.filter.params)
            .unwrap_or_default(),
    };
    std::ptr::write_unaligned(p_params, params);
}

unsafe extern "win64" fn voice_set_output_filter_parameters(
    this: *mut VoiceHeader,
    p_dest: *mut VoiceHeader,
    p_params: *const XAudio2FilterParameters,
    _op_set: u32,
) -> i32 {
    if this.is_null() || p_params.is_null() {
        return E_POINTER;
    }
    let hdr = &*this;
    if hdr.kind == VoiceKind::Mastering {
        return XAUDIO2_E_INVALID_CALL;
    }
    let dest_id = match resolve_dest_id(hdr, p_dest) {
        Ok(id) => id,
        Err(hr) => return hr,
    };
    let params = std::ptr::read_unaligned(p_params);
    let mut st = hdr.engine.state.lock().unwrap();
    let master_snap = st.master.as_ref().map(|m| (m.id, m.channels));
    let send_opt = match hdr.kind {
        VoiceKind::Source => st.sources.get_mut(&hdr.id).and_then(|s| {
            let out_ch = s.output_channels();
            if s.use_default_master_send {
                if let Some((mid, mch)) = master_snap {
                    if s.sends.len() != 1 || s.sends[0].dest_id != mid || s.sends[0].dest_channels != mch {
                        s.sends = vec![VoiceSend::new(mid, out_ch, mch)];
                    }
                }
            }
            find_send_mut(&mut s.sends, dest_id)
        }),
        VoiceKind::Submix => st.submixes.get_mut(&hdr.id).and_then(|s| {
            let out_ch = s.output_channels();
            if s.use_default_master_send {
                if let Some((mid, mch)) = master_snap {
                    if s.sends.len() != 1 || s.sends[0].dest_id != mid || s.sends[0].dest_channels != mch {
                        s.sends = vec![VoiceSend::new(mid, out_ch, mch)];
                    }
                }
            }
            find_send_mut(&mut s.sends, dest_id)
        }),
        VoiceKind::Mastering => None,
    };
    match send_opt {
        Some(send) => match send.output_filter.set(params) {
            Ok(()) => S_OK,
            Err(hr) => hr,
        },
        None => XAUDIO2_E_INVALID_CALL,
    }
}

unsafe extern "win64" fn voice_get_output_filter_parameters(
    this: *mut VoiceHeader,
    p_dest: *mut VoiceHeader,
    p_params: *mut XAudio2FilterParameters,
) {
    if p_params.is_null() {
        return;
    }
    if this.is_null() {
        std::ptr::write_unaligned(p_params, XAudio2FilterParameters::default());
        return;
    }
    let hdr = &*this;
    let Ok(dest_id) = resolve_dest_id(hdr, p_dest) else {
        std::ptr::write_unaligned(p_params, XAudio2FilterParameters::default());
        return;
    };
    let st = hdr.engine.state.lock().unwrap();
    let params = match hdr.kind {
        VoiceKind::Source => st
            .sources
            .get(&hdr.id)
            .and_then(|s| find_send(&s.sends, dest_id))
            .map(|s| s.output_filter.params)
            .unwrap_or_default(),
        VoiceKind::Submix => st
            .submixes
            .get(&hdr.id)
            .and_then(|s| find_send(&s.sends, dest_id))
            .map(|s| s.output_filter.params)
            .unwrap_or_default(),
        VoiceKind::Mastering => XAudio2FilterParameters::default(),
    };
    std::ptr::write_unaligned(p_params, params);
}

unsafe extern "win64" fn voice_set_volume(
    this: *mut VoiceHeader,
    volume: f32,
    _op_set: u32,
) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    if !volume.is_finite() {
        return XAUDIO2_E_INVALID_CALL;
    }
    let v = volume.clamp(-XAUDIO2_MAX_VOLUME_LEVEL, XAUDIO2_MAX_VOLUME_LEVEL);
    let hdr = &*this;
    let mut st = hdr.engine.state.lock().unwrap();
    match hdr.kind {
        VoiceKind::Mastering => {
            if let Some(m) = st.master.as_mut().filter(|m| m.id == hdr.id) {
                m.volume = v;
            }
        }
        VoiceKind::Submix => {
            if let Some(s) = st.submixes.get_mut(&hdr.id) {
                s.volume = v;
            }
        }
        VoiceKind::Source => {
            if let Some(s) = st.sources.get_mut(&hdr.id) {
                s.volume = v;
            }
        }
    }
    S_OK
}
crate::win64_bridge!(fn voice_set_volume(*mut VoiceHeader, f32, u32) -> i32);

unsafe extern "win64" fn voice_get_volume(this: *mut VoiceHeader, out: *mut f32) {
    if out.is_null() {
        return;
    }
    if this.is_null() {
        std::ptr::write_unaligned(out, 1.0);
        return;
    }
    let hdr = &*this;
    let st = hdr.engine.state.lock().unwrap();
    let v = match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_ref()
            .filter(|m| m.id == hdr.id)
            .map(|m| m.volume)
            .unwrap_or(1.0),
        VoiceKind::Submix => st.submixes.get(&hdr.id).map(|s| s.volume).unwrap_or(1.0),
        VoiceKind::Source => st.sources.get(&hdr.id).map(|s| s.volume).unwrap_or(1.0),
    };
    std::ptr::write_unaligned(out, v);
}

unsafe extern "win64" fn voice_set_channel_volumes(
    this: *mut VoiceHeader,
    channels: u32,
    p_volumes: *const f32,
    _op_set: u32,
) -> i32 {
    if this.is_null() || p_volumes.is_null() {
        return E_POINTER;
    }
    let hdr = &*this;
    let n = channels as usize;
    let mut vols = Vec::with_capacity(n);
    for i in 0..n {
        let v = std::ptr::read_unaligned(p_volumes.add(i));
        if !v.is_finite() {
            return XAUDIO2_E_INVALID_CALL;
        }
        vols.push(v.clamp(-XAUDIO2_MAX_VOLUME_LEVEL, XAUDIO2_MAX_VOLUME_LEVEL));
    }

    let mut st = hdr.engine.state.lock().unwrap();
    let target = match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_mut()
            .filter(|m| m.id == hdr.id)
            .map(|m| &mut m.channel_volumes),
        VoiceKind::Submix => st.submixes.get_mut(&hdr.id).map(|s| &mut s.channel_volumes),
        VoiceKind::Source => st.sources.get_mut(&hdr.id).map(|s| &mut s.channel_volumes),
    };
    let Some(dst) = target else {
        return XAUDIO2_E_INVALID_CALL;
    };
    if dst.len() != n {
        return XAUDIO2_E_INVALID_CALL;
    }
    dst.copy_from_slice(&vols);
    S_OK
}

unsafe extern "win64" fn voice_get_channel_volumes(
    this: *mut VoiceHeader,
    channels: u32,
    p_volumes: *mut f32,
) {
    if this.is_null() || p_volumes.is_null() {
        return;
    }
    let hdr = &*this;
    let st = hdr.engine.state.lock().unwrap();
    let src = match hdr.kind {
        VoiceKind::Mastering => st
            .master
            .as_ref()
            .filter(|m| m.id == hdr.id)
            .map(|m| m.channel_volumes.as_slice()),
        VoiceKind::Submix => st
            .submixes
            .get(&hdr.id)
            .map(|s| s.channel_volumes.as_slice()),
        VoiceKind::Source => st
            .sources
            .get(&hdr.id)
            .map(|s| s.channel_volumes.as_slice()),
    };
    if let Some(vols) = src {
        let count = (channels as usize).min(vols.len());
        for (i, &v) in vols[..count].iter().enumerate() {
            std::ptr::write_unaligned(p_volumes.add(i), v);
        }
    }
}

unsafe extern "win64" fn voice_set_output_matrix(
    this: *mut VoiceHeader,
    p_dest: *mut VoiceHeader,
    src_channels: u32,
    dst_channels: u32,
    p_matrix: *const f32,
    _op_set: u32,
) -> i32 {
    if this.is_null() || p_matrix.is_null() {
        return E_POINTER;
    }
    let hdr = &*this;
    if hdr.kind == VoiceKind::Mastering || src_channels == 0 || dst_channels == 0 {
        return XAUDIO2_E_INVALID_CALL;
    }
    let dest_id = match resolve_dest_id(hdr, p_dest) {
        Ok(id) => id,
        Err(hr) => return hr,
    };
    let s_ch = src_channels as usize;
    let d_ch = dst_channels as usize;
    let total = match s_ch.checked_mul(d_ch) {
        Some(n) => n,
        None => return XAUDIO2_E_INVALID_CALL,
    };
    let mut mat = Vec::with_capacity(total);
    for i in 0..total {
        let v = std::ptr::read_unaligned(p_matrix.add(i));
        if !v.is_finite() {
            return XAUDIO2_E_INVALID_CALL;
        }
        mat.push(v.clamp(-XAUDIO2_MAX_VOLUME_LEVEL, XAUDIO2_MAX_VOLUME_LEVEL));
    }

    let mut st = hdr.engine.state.lock().unwrap();
    let master_snap = st.master.as_ref().map(|m| (m.id, m.channels));
    let (voice_ch, send_opt) = match hdr.kind {
        VoiceKind::Source => match st.sources.get_mut(&hdr.id) {
            Some(s) => {
                let out_ch = s.output_channels();
                if s.use_default_master_send {
                    if let Some((mid, mch)) = master_snap {
                        if s.sends.len() != 1
                            || s.sends[0].dest_id != mid
                            || s.sends[0].dest_channels != mch
                        {
                            s.sends = vec![VoiceSend::new(mid, out_ch, mch)];
                        }
                    }
                }
                (out_ch, find_send_mut(&mut s.sends, dest_id))
            }
            None => return XAUDIO2_E_INVALID_CALL,
        },
        VoiceKind::Submix => match st.submixes.get_mut(&hdr.id) {
            Some(s) => {
                let out_ch = s.output_channels();
                if s.use_default_master_send {
                    if let Some((mid, mch)) = master_snap {
                        if s.sends.len() != 1
                            || s.sends[0].dest_id != mid
                            || s.sends[0].dest_channels != mch
                        {
                            s.sends = vec![VoiceSend::new(mid, out_ch, mch)];
                        }
                    }
                }
                (out_ch, find_send_mut(&mut s.sends, dest_id))
            }
            None => return XAUDIO2_E_INVALID_CALL,
        },
        VoiceKind::Mastering => return XAUDIO2_E_INVALID_CALL,
    };
    if voice_ch != s_ch {
        return XAUDIO2_E_INVALID_CALL;
    }
    let Some(send) = send_opt else {
        return XAUDIO2_E_INVALID_CALL;
    };
    if send.dest_channels != d_ch {
        return XAUDIO2_E_INVALID_CALL;
    }
    send.matrix = mat;
    S_OK
}

unsafe extern "win64" fn voice_get_output_matrix(
    this: *mut VoiceHeader,
    p_dest: *mut VoiceHeader,
    src_channels: u32,
    dst_channels: u32,
    p_matrix: *mut f32,
) {
    if this.is_null() || p_matrix.is_null() || src_channels == 0 || dst_channels == 0 {
        return;
    }
    let hdr = &*this;
    let Ok(dest_id) = resolve_dest_id(hdr, p_dest) else {
        return;
    };
    let s_ch = src_channels as usize;
    let d_ch = dst_channels as usize;
    let total = s_ch.saturating_mul(d_ch);

    let mut st = hdr.engine.state.lock().unwrap();
    let master_snap = st.master.as_ref().map(|m| (m.id, m.channels));
    let mat_opt = match hdr.kind {
        VoiceKind::Source => st.sources.get_mut(&hdr.id).and_then(|s| {
            let out_ch = s.output_channels();
            if s.use_default_master_send {
                if let Some((mid, mch)) = master_snap {
                    if s.sends.len() != 1
                        || s.sends[0].dest_id != mid
                        || s.sends[0].dest_channels != mch
                    {
                        s.sends = vec![VoiceSend::new(mid, out_ch, mch)];
                    }
                }
            }
            find_send(&s.sends, dest_id).map(|send| send.matrix.clone())
        }),
        VoiceKind::Submix => st.submixes.get_mut(&hdr.id).and_then(|s| {
            let out_ch = s.output_channels();
            if s.use_default_master_send {
                if let Some((mid, mch)) = master_snap {
                    if s.sends.len() != 1
                        || s.sends[0].dest_id != mid
                        || s.sends[0].dest_channels != mch
                    {
                        s.sends = vec![VoiceSend::new(mid, out_ch, mch)];
                    }
                }
            }
            find_send(&s.sends, dest_id).map(|send| send.matrix.clone())
        }),
        VoiceKind::Mastering => None,
    };
    if let Some(mat) = mat_opt {
        let count = total.min(mat.len());
        for (i, &v) in mat[..count].iter().enumerate() {
            std::ptr::write_unaligned(p_matrix.add(i), v);
        }
    }
}

unsafe extern "win64" fn voice_destroy_voice(this: *mut VoiceHeader) {
    if this.is_null() {
        return;
    }
    let hdr = Box::from_raw(this);
    let mut need_cb_sync = false;
    let old_effects: Vec<VoiceEffectSlot> = match hdr.kind {
        VoiceKind::Source => {
            let mut st = hdr.engine.state.lock().unwrap();
            if let Some(mut src) = st.sources.remove(&hdr.id) {
                src.callback_alive.store(false, Ordering::Release);
                need_cb_sync = src.callback != 0;
                std::mem::take(&mut src.effects)
            } else {
                Vec::new()
            }
        }
        VoiceKind::Submix => {
            let mut st = hdr.engine.state.lock().unwrap();
            st.submixes
                .remove(&hdr.id)
                .map(|mut s| std::mem::take(&mut s.effects))
                .unwrap_or_default()
        }
        VoiceKind::Mastering => {
            hdr.engine.shutdown_runtime();
            let mut st = hdr.engine.state.lock().unwrap();
            if st.master.as_ref().is_some_and(|m| m.id == hdr.id) {
                st.master
                    .take()
                    .map(|mut m| std::mem::take(&mut m.effects))
                    .unwrap_or_default()
            } else {
                Vec::new()
            }
        }
    };
    drop(old_effects);
    if need_cb_sync {
        hdr.engine.sync_callbacks();
    }
}

// ============================================================================
// IXAudio2MasteringVoice-Specific Methods
// ============================================================================

unsafe extern "win64" fn mv_get_channel_mask(
    this: *const MasteringVoice,
    mask: *mut u32,
) -> i32 {
    if mask.is_null() {
        return E_POINTER;
    }
    let m = if this.is_null() {
        0x3
    } else {
        let hdr = &(*this).0;
        let st = hdr.engine.state.lock().unwrap();
        st.master
            .as_ref()
            .filter(|mv| mv.id == hdr.id)
            .map(|mv| mv.channel_mask)
            .unwrap_or(0x3)
    };
    std::ptr::write_unaligned(mask, m);
    S_OK
}

// ============================================================================
// IXAudio2SourceVoice-Specific Methods
// ============================================================================

unsafe extern "win64" fn sv_start(this: *mut SourceVoice, _flags: u32, _op_set: u32) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let hdr = &(*this).0;
    let mut st = hdr.engine.state.lock().unwrap();
    if let Some(src) = st.sources.get_mut(&hdr.id) {
        src.running = true;
        src.play_tails = false;
    }
    S_OK
}

unsafe extern "win64" fn sv_stop(this: *mut SourceVoice, flags: u32, _op_set: u32) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let hdr = &(*this).0;
    let mut st = hdr.engine.state.lock().unwrap();
    if let Some(src) = st.sources.get_mut(&hdr.id) {
        src.running = false;
        src.play_tails = (flags & XAUDIO2_PLAY_TAILS) != 0;
    }
    S_OK
}

unsafe extern "win64" fn sv_submit_source_buffer(
    this: *mut SourceVoice,
    buf: *const XAudio2Buffer,
    _wma: usize,
) -> i32 {
    if this.is_null() || buf.is_null() {
        return E_POINTER;
    }
    let b = std::ptr::read_unaligned(buf);
    if b.audio_bytes > 0 && b.audio_data.is_null() {
        return E_INVALIDARG;
    }
    let hdr = &(*this).0;
    let fmt = {
        let st = hdr.engine.state.lock().unwrap();
        let Some(src) = st.sources.get(&hdr.id) else {
            return XAUDIO2_E_INVALID_CALL;
        };
        if src.queue.len() >= XAUDIO2_MAX_QUEUED_BUFFERS {
            return XAUDIO2_E_INVALID_CALL;
        }
        src.format
    };

    let (samples, total_frames) = if b.audio_bytes == 0 {
        (Vec::new(), 0usize)
    } else {
        let raw = std::slice::from_raw_parts(b.audio_data, b.audio_bytes as usize);
        decode_pcm_to_f32(fmt, raw)
    };

    let play_begin = (b.play_begin as usize).min(total_frames);
    let play_end = if b.play_length == 0 {
        total_frames
    } else {
        play_begin.saturating_add(b.play_length as usize).min(total_frames)
    };

    let (loop_begin, loop_end, loops_remaining) = if b.loop_count > 0 && total_frames > 0 {
        let lb = (b.loop_begin as usize).min(total_frames);
        let le = if b.loop_length == 0 {
            play_end
        } else {
            lb.saturating_add(b.loop_length as usize).min(total_frames)
        };
        if le > lb {
            (lb, le, b.loop_count)
        } else {
            (0, 0, 0)
        }
    } else {
        (0, 0, 0)
    };

    let queued = QueuedSourceBuffer {
        samples,
        play_begin,
        play_end,
        loop_begin,
        loop_end,
        loops_remaining,
        end_of_stream: (b.flags & XAUDIO2_END_OF_STREAM) != 0,
        context: b.context,
        started: false,
        pos: play_begin as f64,
    };

    let mut st = hdr.engine.state.lock().unwrap();
    if let Some(src) = st.sources.get_mut(&hdr.id) {
        src.discontinuity = false;
        src.queue.push_back(queued);
    }
    S_OK
}

unsafe extern "win64" fn sv_flush_source_buffers(this: *mut SourceVoice) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let hdr = &(*this).0;
    let mut events = Vec::new();
    {
        let mut st = hdr.engine.state.lock().unwrap();
        if let Some(src) = st.sources.get_mut(&hdr.id) {
            let keep_front =
                src.running && src.queue.front().is_some_and(|b| b.started);
            let drain_start = if keep_front { 1 } else { 0 };
            while src.queue.len() > drain_start {
                if let Some(removed) = src.queue.remove(drain_start) {
                    if src.callback != 0 {
                        events.push(CallbackEvent::VoiceBufferEnd {
                            cb: src.callback,
                            alive: src.callback_alive.clone(),
                            ctx: removed.context,
                        });
                    }
                }
            }
        }
    }
    if !events.is_empty() {
        hdr.engine.dispatch_callbacks(events);
    }
    S_OK
}

unsafe extern "win64" fn sv_discontinuity(this: *mut SourceVoice) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let hdr = &(*this).0;
    let mut st = hdr.engine.state.lock().unwrap();
    if let Some(src) = st.sources.get_mut(&hdr.id) {
        src.discontinuity = true;
    }
    S_OK
}

unsafe extern "win64" fn sv_exit_loop(this: *mut SourceVoice, _op_set: u32) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let hdr = &(*this).0;
    let mut st = hdr.engine.state.lock().unwrap();
    if let Some(src) = st.sources.get_mut(&hdr.id) {
        if let Some(front) = src.queue.front_mut() {
            front.loops_remaining = 0;
        }
    }
    S_OK
}

unsafe extern "win64" fn sv_get_state(
    this: *mut SourceVoice,
    state: *mut XAudio2VoiceState,
    flags: u32,
) {
    if state.is_null() {
        return;
    }
    if this.is_null() {
        std::ptr::write_unaligned(
            state,
            XAudio2VoiceState {
                current_buffer_context: 0,
                buffers_queued: 0,
                samples_played: 0,
            },
        );
        return;
    }
    let hdr = &(*this).0;
    let st = hdr.engine.state.lock().unwrap();
    let (ctx, queued, played) = st
        .sources
        .get(&hdr.id)
        .map(|src| {
            let ctx = src.queue.front().map(|b| b.context).unwrap_or(0);
            let played = if (flags & XAUDIO2_VOICE_NOSAMPLESPLAYED) != 0 {
                0
            } else {
                src.samples_played
            };
            (ctx, src.queue.len() as u32, played)
        })
        .unwrap_or((0, 0, 0));
    std::ptr::write_unaligned(
        state,
        XAudio2VoiceState {
            current_buffer_context: ctx,
            buffers_queued: queued,
            samples_played: played,
        },
    );
}

unsafe extern "win64" fn sv_set_freq_ratio(
    this: *mut SourceVoice,
    ratio: f32,
    _op_set: u32,
) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    if !ratio.is_finite() {
        return XAUDIO2_E_INVALID_CALL;
    }
    let hdr = &(*this).0;
    let mut st = hdr.engine.state.lock().unwrap();
    let Some(src) = st.sources.get_mut(&hdr.id) else {
        return XAUDIO2_E_INVALID_CALL;
    };
    if (src.creation_flags & XAUDIO2_VOICE_NOPITCH) != 0 {
        if (ratio - 1.0).abs() > 1e-6 {
            return XAUDIO2_E_INVALID_CALL;
        }
        src.freq_ratio = 1.0;
        return S_OK;
    }
    if ratio < XAUDIO2_MIN_FREQ_RATIO || ratio > src.max_freq_ratio {
        return XAUDIO2_E_INVALID_CALL;
    }
    src.freq_ratio = ratio;
    S_OK
}
crate::win64_bridge!(fn sv_set_freq_ratio(*mut SourceVoice, f32, u32) -> i32);

unsafe extern "win64" fn sv_get_freq_ratio(this: *const SourceVoice, out: *mut f32) {
    if out.is_null() {
        return;
    }
    let ratio = if this.is_null() {
        1.0
    } else {
        let hdr = &(*this).0;
        let st = hdr.engine.state.lock().unwrap();
        st.sources.get(&hdr.id).map(|s| s.freq_ratio).unwrap_or(1.0)
    };
    std::ptr::write_unaligned(out, ratio);
}

unsafe extern "win64" fn sv_set_source_sample_rate(
    this: *mut SourceVoice,
    new_sample_rate: u32,
) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    if !(XAUDIO2_MIN_SAMPLE_RATE..=XAUDIO2_MAX_SAMPLE_RATE).contains(&new_sample_rate) {
        return XAUDIO2_E_INVALID_CALL;
    }
    let hdr = &(*this).0;
    let mut st = hdr.engine.state.lock().unwrap();
    let Some(src) = st.sources.get_mut(&hdr.id) else {
        return XAUDIO2_E_INVALID_CALL;
    };
    if !src.queue.is_empty() {
        return XAUDIO2_E_INVALID_CALL;
    }
    src.format.sample_rate = new_sample_rate;
    S_OK
}

// ============================================================================
// X3DAudio Math & Implementation
// ============================================================================

#[derive(Clone, Copy)]
struct Vec3(f32, f32, f32);

impl Vec3 {
    fn from_x3d(v: X3DAudioVector) -> Self {
        Self(v.x, v.y, v.z)
    }
    fn sub(self, o: Self) -> Self {
        Self(self.0 - o.0, self.1 - o.1, self.2 - o.2)
    }
    fn add(self, o: Self) -> Self {
        Self(self.0 + o.0, self.1 + o.1, self.2 + o.2)
    }
    fn scale(self, s: f32) -> Self {
        Self(self.0 * s, self.1 * s, self.2 * s)
    }
    fn dot(self, o: Self) -> f32 {
        self.0 * o.0 + self.1 * o.1 + self.2 * o.2
    }
    fn cross(self, o: Self) -> Self {
        Self(
            self.1 * o.2 - self.2 * o.1,
            self.2 * o.0 - self.0 * o.2,
            self.0 * o.1 - self.1 * o.0,
        )
    }
    fn len(self) -> f32 {
        self.dot(self).sqrt()
    }
    fn normalize_or(self, fallback: Self) -> Self {
        let l = self.len();
        if l.is_finite() && l > 1e-6 {
            self.scale(1.0 / l)
        } else {
            fallback
        }
    }
}

struct X3DHandleData {
    speaker_mask: u32,
    speed_of_sound: f32,
    channel_count: u32,
    lfe_index: i32,
}

impl X3DHandleData {
    fn encode(&self) -> [u8; 20] {
        let mut out = [0u8; 20];
        out[0..4].copy_from_slice(&X3DAUDIO_HANDLE_MAGIC.to_le_bytes());
        out[4..8].copy_from_slice(&self.speaker_mask.to_le_bytes());
        out[8..12].copy_from_slice(&self.speed_of_sound.to_le_bytes());
        out[12..16].copy_from_slice(&self.channel_count.to_le_bytes());
        out[16..20].copy_from_slice(&self.lfe_index.to_le_bytes());
        out
    }

    fn decode(bytes: [u8; 20]) -> Self {
        let magic = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        if magic != X3DAUDIO_HANDLE_MAGIC {
            return Self {
                speaker_mask: 0x3,
                speed_of_sound: X3DAUDIO_SPEED_OF_SOUND,
                channel_count: 2,
                lfe_index: -1,
            };
        }
        let speaker_mask = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let speed_of_sound = f32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let channel_count = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
        let lfe_index = i32::from_le_bytes(bytes[16..20].try_into().unwrap());
        Self {
            speaker_mask,
            speed_of_sound: if speed_of_sound.is_finite() && speed_of_sound > 0.0 {
                speed_of_sound
            } else {
                X3DAUDIO_SPEED_OF_SOUND
            },
            channel_count: channel_count.max(1),
            lfe_index,
        }
    }
}

unsafe fn eval_distance_curve(p_curve: *const X3DAudioDistanceCurve, dist: f32) -> Option<f32> {
    if p_curve.is_null() {
        return None;
    }
    let curve = std::ptr::read_unaligned(p_curve);
    let count = (curve.point_count as usize).min(4096);
    if count == 0 || curve.points.is_null() {
        return None;
    }
    let first = std::ptr::read_unaligned(curve.points);
    if count == 1 || dist <= first.distance {
        return Some(first.dsp_setting);
    }
    let mut prev = first;
    for i in 1..count {
        let cur = std::ptr::read_unaligned(curve.points.add(i));
        if dist <= cur.distance {
            let span = cur.distance - prev.distance;
            if span <= 1e-6 {
                return Some(cur.dsp_setting);
            }
            let t = ((dist - prev.distance) / span).clamp(0.0, 1.0);
            return Some(prev.dsp_setting + (cur.dsp_setting - prev.dsp_setting) * t);
        }
        prev = cur;
    }
    Some(prev.dsp_setting)
}

unsafe fn eval_cone(p_cone: *const X3DAudioCone, angle: f32) -> (f32, f32, f32) {
    if p_cone.is_null() {
        return (1.0, 0.0, 1.0);
    }
    let c = std::ptr::read_unaligned(p_cone);
    let inner_half = (c.inner_angle * 0.5).clamp(0.0, PI);
    let outer_half = (c.outer_angle * 0.5).clamp(inner_half, PI);
    if angle <= inner_half {
        (c.inner_volume, c.inner_lpf, c.inner_reverb)
    } else if angle >= outer_half {
        (c.outer_volume, c.outer_lpf, c.outer_reverb)
    } else {
        let t = ((angle - inner_half) / (outer_half - inner_half).max(1e-6)).clamp(0.0, 1.0);
        (
            c.inner_volume + (c.outer_volume - c.inner_volume) * t,
            c.inner_lpf + (c.outer_lpf - c.inner_lpf) * t,
            c.inner_reverb + (c.outer_reverb - c.inner_reverb) * t,
        )
    }
}

/// Speaker bit -> clockwise azimuth from `+Z` front in `[0, 2*pi)`.
fn speaker_azimuth(bit: u32) -> Option<f32> {
    match bit {
        SPEAKER_FRONT_CENTER => Some(0.0),
        SPEAKER_FRONT_RIGHT_OF_CENTER => Some(PI / 8.0),
        SPEAKER_FRONT_RIGHT => Some(FRAC_PI_4),
        SPEAKER_SIDE_RIGHT => Some(FRAC_PI_2),
        SPEAKER_BACK_RIGHT => Some(3.0 * FRAC_PI_4),
        SPEAKER_BACK_CENTER => Some(PI),
        SPEAKER_BACK_LEFT => Some(5.0 * FRAC_PI_4),
        SPEAKER_SIDE_LEFT => Some(3.0 * FRAC_PI_2),
        SPEAKER_FRONT_LEFT => Some(7.0 * FRAC_PI_4),
        SPEAKER_FRONT_LEFT_OF_CENTER => Some(15.0 * PI / 8.0),
        _ => None,
    }
}

/// Compute per-destination-channel panning gains (excluding LFE, before distance attenuation)
/// for a point at listener-local unit direction `(lx, lz)` (`+lx` = right, `+lz` = front).
fn pan_direction_into(
    lx: f32,
    lz: f32,
    omni_blend: f32,
    speaker_mask: u32,
    dst_ch: usize,
    zero_center: bool,
    gains: &mut [f32],
) {
    gains.fill(0.0);
    if dst_ch == 0 {
        return;
    }
    if dst_ch == 1 {
        gains[0] = 1.0;
        return;
    }
    if dst_ch == 2 {
        // Stereo constant-power panning: -pi/2 = hard left (1, 0), 0 = center (0.707, 0.707),
        // +pi/2 = hard right (0, 1).
        let angle = lx.atan2(lz.abs());
        let theta = ((angle + FRAC_PI_2) * 0.5).clamp(0.0, FRAC_PI_2);
        let dir_l = theta.cos();
        let dir_r = theta.sin();
        gains[0] = dir_l + (FRAC_1_SQRT_2 - dir_l) * omni_blend;
        gains[1] = dir_r + (FRAC_1_SQRT_2 - dir_r) * omni_blend;
        return;
    }

    // Collect non-LFE directional speakers with their output channel indices and azimuths.
    let mut speakers: Vec<(usize, f32)> = Vec::with_capacity(dst_ch);
    let mut mask = speaker_mask;
    for ch_idx in 0..dst_ch {
        if mask == 0 {
            break;
        }
        let bit = mask & mask.wrapping_neg();
        mask &= !bit;
        if bit == SPEAKER_LOW_FREQUENCY {
            continue;
        }
        if zero_center && bit == SPEAKER_FRONT_CENTER {
            continue;
        }
        if let Some(az) = speaker_azimuth(bit) {
            speakers.push((ch_idx, az));
        }
    }
    if speakers.is_empty() {
        let eq = 1.0 / (dst_ch as f32).sqrt();
        gains.fill(eq);
        return;
    }
    if speakers.len() == 1 {
        gains[speakers[0].0] = 1.0;
        return;
    }

    speakers.sort_by(|a, b| a.1.total_cmp(&b.1));
    let mut az = lx.atan2(lz);
    if az < 0.0 {
        az += TAU;
    }

    // Find adjacent speakers bracketing `az` on the [0, 2*pi) circle.
    let n = speakers.len();
    let mut left_idx = n - 1;
    let mut right_idx = 0;
    for i in 0..n {
        let a0 = speakers[i].1;
        let next = (i + 1) % n;
        let a1 = if next == 0 { speakers[0].1 + TAU } else { speakers[next].1 };
        let az_adj = if az < a0 && next == 0 { az + TAU } else { az };
        if az_adj >= a0 && az_adj <= a1 {
            left_idx = i;
            right_idx = next;
            break;
        }
    }

    let (ch0, a0) = speakers[left_idx];
    let (ch1, mut a1) = speakers[right_idx];
    let mut az_adj = az;
    if a1 <= a0 {
        a1 += TAU;
        if az_adj < a0 {
            az_adj += TAU;
        }
    }
    let span = (a1 - a0).max(1e-5);
    let t = ((az_adj - a0) / span).clamp(0.0, 1.0);
    let g0 = (t * FRAC_PI_2).cos();
    let g1 = (t * FRAC_PI_2).sin();

    if omni_blend > 0.0 {
        let omni = 1.0 / (speakers.len() as f32).sqrt();
        for &(ch_idx, _) in &speakers {
            let dir_g = if ch_idx == ch0 {
                g0
            } else if ch_idx == ch1 {
                g1
            } else {
                0.0
            };
            gains[ch_idx] = dir_g + (omni - dir_g) * omni_blend;
        }
    } else {
        gains[ch0] = g0;
        gains[ch1] = g1;
    }
}

unsafe extern "win64" fn x3daudio_initialize_impl(
    speaker_mask: u32,
    speed_of_sound: f32,
    instance: *mut [u8; 20],
) -> i32 {
    if instance.is_null() {
        return E_POINTER;
    }
    if speaker_mask == 0 || !speed_of_sound.is_finite() || speed_of_sound <= 0.0 {
        return E_INVALIDARG;
    }
    let channel_count = speaker_mask.count_ones();
    let lfe_index = if (speaker_mask & SPEAKER_LOW_FREQUENCY) != 0 {
        (speaker_mask & (SPEAKER_LOW_FREQUENCY - 1)).count_ones() as i32
    } else {
        -1
    };
    let handle = X3DHandleData {
        speaker_mask,
        speed_of_sound,
        channel_count,
        lfe_index,
    };
    std::ptr::write_unaligned(instance, handle.encode());
    S_OK
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
    std::ptr::write_unaligned(pp_xaudio2, Box::into_raw(engine));
    S_OK
}

pub unsafe extern "win64" fn CreateAudioReverb(ppv: *mut usize) -> i32 {
    ensure_vtbls_registered();
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, 0);
    let params = XAudio2FxReverbParameters::default();
    let cfg = reverb_config_from_xaudio2(&params, 48_000);
    let obj = BuiltinXapoCom::alloc(BuiltinXapoKind::AudioReverb {
        params,
        dsp: SchroederReverbDsp::new(cfg),
    });
    std::ptr::write_unaligned(ppv, obj as usize);
    S_OK
}

pub unsafe extern "win64" fn CreateAudioVolumeMeter(ppv: *mut usize) -> i32 {
    ensure_vtbls_registered();
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, 0);
    let obj = BuiltinXapoCom::alloc(BuiltinXapoKind::VolumeMeter(VolumeMeterDsp::new()));
    std::ptr::write_unaligned(ppv, obj as usize);
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
    std::ptr::write_unaligned(ppv, 0);
    if clsid.is_null() {
        return E_POINTER;
    }
    let guid = std::ptr::read_unaligned(clsid as *const GUID);
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
    std::ptr::write_unaligned(ppv, obj as usize);
    S_OK
}

pub unsafe extern "win64" fn X3DAudioCalculate(
    instance: *const [u8; 20],
    p_listener: *const c_void,
    p_emitter: *const c_void,
    flags: u32,
    p_dsp_settings: *mut c_void,
) {
    if instance.is_null() || p_listener.is_null() || p_emitter.is_null() || p_dsp_settings.is_null()
    {
        return;
    }
    let handle = X3DHandleData::decode(std::ptr::read_unaligned(instance));
    let listener = std::ptr::read_unaligned(p_listener as *const X3DAudioListener);
    let emitter = std::ptr::read_unaligned(p_emitter as *const X3DAudioEmitter);
    let p_dsp_settings = p_dsp_settings as *mut X3DAudioDspSettings;
    let mut out = std::ptr::read_unaligned(p_dsp_settings);

    // Listener orthonormal basis (left-handed: +X right, +Y up, +Z front).
    let l_pos = Vec3::from_x3d(listener.position);
    let l_vel = Vec3::from_x3d(listener.velocity);
    let l_front = Vec3::from_x3d(listener.orient_front).normalize_or(Vec3(0.0, 0.0, 1.0));
    let l_top_raw = Vec3::from_x3d(listener.orient_top).normalize_or(Vec3(0.0, 1.0, 0.0));
    let l_right = l_top_raw.cross(l_front).normalize_or(Vec3(1.0, 0.0, 0.0));

    // Emitter basis & position.
    let e_pos = Vec3::from_x3d(emitter.position);
    let e_vel = Vec3::from_x3d(emitter.velocity);
    let e_front = Vec3::from_x3d(emitter.orient_front).normalize_or(Vec3(0.0, 0.0, 1.0));
    let e_top_raw = Vec3::from_x3d(emitter.orient_top).normalize_or(Vec3(0.0, 1.0, 0.0));
    let e_right = e_top_raw.cross(e_front).normalize_or(Vec3(1.0, 0.0, 0.0));

    let delta = e_pos.sub(l_pos);
    let distance = delta.len();
    out.emitter_to_listener_distance = distance;

    let dir = if distance > 1e-6 {
        delta.scale(1.0 / distance)
    } else {
        l_front
    };

    // Angles & cones.
    let emitter_to_listener_cos = e_front.dot(dir.scale(-1.0)).clamp(-1.0, 1.0);
    let emitter_angle = if distance > 1e-6 {
        emitter_to_listener_cos.acos()
    } else {
        0.0
    };
    if (flags & X3DAUDIO_CALCULATE_EMITTER_ANGLE) != 0 {
        out.emitter_to_listener_angle = emitter_angle;
    }

    let listener_angle = if distance > 1e-6 {
        l_front.dot(dir).clamp(-1.0, 1.0).acos()
    } else {
        0.0
    };

    let (e_cone_vol, e_cone_lpf, e_cone_rev) = eval_cone(emitter.cone, emitter_angle);
    let (l_cone_vol, l_cone_lpf, l_cone_rev) = eval_cone(listener.cone, listener_angle);
    let cone_vol = (e_cone_vol * l_cone_vol).max(0.0);
    let cone_lpf = (e_cone_lpf + l_cone_lpf).clamp(0.0, 1.0);
    let cone_rev = (e_cone_rev * l_cone_rev).max(0.0);

    // Distance scaling & attenuation curves.
    let scaler = if emitter.curve_distance_scaler.is_finite() && emitter.curve_distance_scaler > 0.0
    {
        emitter.curve_distance_scaler
    } else {
        1.0
    };
    let scaled_dist = distance / scaler;

    let base_vol = eval_distance_curve(emitter.volume_curve, scaled_dist).unwrap_or_else(|| {
        if scaled_dist <= 1.0 {
            1.0
        } else {
            1.0 / scaled_dist
        }
    });
    let volume_att = (base_vol * cone_vol).max(0.0);

    // Doppler shift.
    if (flags & X3DAUDIO_CALCULATE_DOPPLER) != 0 {
        let (v_l, v_e) = if distance > 1e-6 {
            (l_vel.dot(dir), e_vel.dot(dir))
        } else {
            (0.0, 0.0)
        };
        out.listener_velocity_component = v_l;
        out.emitter_velocity_component = v_e;
        let d_scale = if emitter.doppler_scaler.is_finite() && emitter.doppler_scaler >= 0.0 {
            emitter.doppler_scaler
        } else {
            1.0
        };
        let c = handle.speed_of_sound;
        if d_scale == 0.0 || c <= 0.0 {
            out.doppler_factor = 1.0;
        } else {
            let vls = (v_l * d_scale).clamp(-c * 0.99, c * 0.99);
            let ves = (v_e * d_scale).clamp(-c * 0.99, c * 0.99);
            out.doppler_factor = ((c + vls) / (c + ves)).clamp(XAUDIO2_MIN_FREQ_RATIO, XAUDIO2_MAX_FREQ_RATIO);
        }
    }

    // LPF direct / reverb & reverb send level.
    if (flags & X3DAUDIO_CALCULATE_LPF_DIRECT) != 0 {
        let curve_lpf = eval_distance_curve(emitter.lpf_direct_curve, scaled_dist)
            .unwrap_or_else(|| 1.0 - 0.25 * scaled_dist.clamp(0.0, 1.0));
        out.lpf_direct_coefficient = (curve_lpf * (1.0 - cone_lpf)).clamp(0.0, 1.0);
    }
    if (flags & X3DAUDIO_CALCULATE_LPF_REVERB) != 0 {
        let curve_lpf_rev = eval_distance_curve(emitter.lpf_reverb_curve, scaled_dist)
            .unwrap_or(0.75);
        out.lpf_reverb_coefficient = (curve_lpf_rev * (1.0 - cone_lpf)).clamp(0.0, 1.0);
    }
    if (flags & X3DAUDIO_CALCULATE_REVERB) != 0 {
        let curve_rev = eval_distance_curve(emitter.reverb_curve, scaled_dist)
            .unwrap_or_else(|| (1.0 - scaled_dist).clamp(0.0, 1.0));
        out.reverb_level = (curve_rev * cone_rev).max(0.0);
    }

    // Speaker panning matrix (`pMatrixCoefficients[dst * src_ch + src]`).
    let src_ch = if out.src_channel_count > 0 {
        out.src_channel_count as usize
    } else {
        emitter.channel_count.max(1) as usize
    };
    let dst_ch = if out.dst_channel_count > 0 {
        out.dst_channel_count as usize
    } else {
        handle.channel_count as usize
    };

    if (flags & X3DAUDIO_CALCULATE_MATRIX) != 0
        && !out.matrix_coefficients.is_null()
        && src_ch > 0
        && dst_ch > 0
    {
        let total = src_ch * dst_ch;
        for i in 0..total {
            std::ptr::write_unaligned(out.matrix_coefficients.add(i), 0.0);
        }

        let zero_center = (flags & X3DAUDIO_CALCULATE_ZEROCENTER) != 0;
        let redirect_lfe = (flags & X3DAUDIO_CALCULATE_REDIRECT_TO_LFE) != 0;
        let lfe_att = eval_distance_curve(emitter.lfe_curve, scaled_dist).unwrap_or(volume_att);

        let inner_radius = if emitter.inner_radius.is_finite() && emitter.inner_radius > 0.0 {
            emitter.inner_radius
        } else {
            0.0
        };
        let inner_angle_spread = if emitter.inner_radius_angle.is_finite() {
            (emitter.inner_radius_angle.clamp(0.0, FRAC_PI_4)) / FRAC_PI_4
        } else {
            0.0
        };
        let omni_blend = if distance <= 1e-6 {
            1.0
        } else if inner_radius > 0.0 && distance < inner_radius {
            let radial = 1.0 - (distance / inner_radius);
            (radial + inner_angle_spread * (1.0 - radial) * 0.25).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let mut ch_gains = vec![0.0f32; dst_ch];
        let use_multi_azimuth = src_ch > 1
            && !emitter.channel_azimuths.is_null()
            && emitter.channel_radius.is_finite()
            && emitter.channel_radius > 0.0;

        if use_multi_azimuth {
            for s in 0..src_ch {
                let az = std::ptr::read_unaligned(emitter.channel_azimuths.add(s));
                // X3DAUDIO_2PI marks an LFE source channel.
                if (az - TAU).abs() < 1e-3 {
                    if handle.lfe_index >= 0 && (handle.lfe_index as usize) < dst_ch {
                        let d = handle.lfe_index as usize;
                        std::ptr::write_unaligned(
                            out.matrix_coefficients.add(d * src_ch + s),
                            lfe_att,
                        );
                    }
                    continue;
                }
                let offset = e_right
                    .scale(az.sin() * emitter.channel_radius)
                    .add(e_front.scale(az.cos() * emitter.channel_radius));
                let ch_delta = e_pos.add(offset).sub(l_pos);
                let ch_dist = ch_delta.len();
                let ch_dir = if ch_dist > 1e-6 {
                    ch_delta.scale(1.0 / ch_dist)
                } else {
                    l_front
                };
                let lx = ch_dir.dot(l_right);
                let lz = ch_dir.dot(l_front);
                pan_direction_into(
                    lx,
                    lz,
                    omni_blend,
                    handle.speaker_mask,
                    dst_ch,
                    zero_center,
                    &mut ch_gains,
                );
                for (d, &g) in ch_gains.iter().enumerate() {
                    std::ptr::write_unaligned(
                        out.matrix_coefficients.add(d * src_ch + s),
                        g * volume_att,
                    );
                }
                if redirect_lfe && handle.lfe_index >= 0 && (handle.lfe_index as usize) < dst_ch {
                    let d = handle.lfe_index as usize;
                    std::ptr::write_unaligned(
                        out.matrix_coefficients.add(d * src_ch + s),
                        lfe_att / (src_ch as f32),
                    );
                }
            }
        } else {
            let lx = if distance > 1e-6 { dir.dot(l_right) } else { 0.0 };
            let lz = if distance > 1e-6 { dir.dot(l_front) } else { 1.0 };
            pan_direction_into(
                lx,
                lz,
                omni_blend,
                handle.speaker_mask,
                dst_ch,
                zero_center,
                &mut ch_gains,
            );
            let norm = 1.0 / (src_ch as f32).sqrt();
            for s in 0..src_ch {
                for (d, &g) in ch_gains.iter().enumerate() {
                    std::ptr::write_unaligned(
                        out.matrix_coefficients.add(d * src_ch + s),
                        g * volume_att * norm,
                    );
                }
                if (redirect_lfe || !emitter.lfe_curve.is_null())
                    && handle.lfe_index >= 0
                    && (handle.lfe_index as usize) < dst_ch
                {
                    let d = handle.lfe_index as usize;
                    std::ptr::write_unaligned(
                        out.matrix_coefficients.add(d * src_ch + s),
                        lfe_att * norm,
                    );
                }
            }
        }
    }

    if (flags & X3DAUDIO_CALCULATE_DELAY) != 0 && !out.delay_times.is_null() && dst_ch > 0 {
        for d in 0..dst_ch {
            std::ptr::write_unaligned(out.delay_times.add(d), 0.0);
        }
    }

    std::ptr::write_unaligned(p_dsp_settings, out);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pcm_wfx(channels: u16, sample_rate: u32, bits: u16) -> WaveFormatEx {
        let block_align = channels * (bits / 8);
        WaveFormatEx {
            format_tag: WAVE_FORMAT_PCM,
            channels,
            samples_per_sec: sample_rate,
            avg_bytes_per_sec: sample_rate * block_align as u32,
            block_align,
            bits_per_sample: bits,
            cb_size: 0,
        }
    }

    fn float_wfx(channels: u16, sample_rate: u32) -> WaveFormatEx {
        let block_align = channels * 4;
        WaveFormatEx {
            format_tag: WAVE_FORMAT_IEEE_FLOAT,
            channels,
            samples_per_sec: sample_rate,
            avg_bytes_per_sec: sample_rate * block_align as u32,
            block_align,
            bits_per_sample: 32,
            cb_size: 0,
        }
    }

    fn make_stopped_test_engine(
        master_ch: u32,
        master_rate: u32,
    ) -> (*mut XAudio2Engine, *mut MasteringVoice) {
        unsafe {
            let mut xa2: *mut XAudio2Engine = std::ptr::null_mut();
            assert_eq!(XAudio2Create(&mut xa2, 0, 1), S_OK);
            // Stop before creating mastering voice so the background mixer thread stays idle
            // while unit tests pull deterministic quantums via `mix_quantum`.
            xa2_stop_engine(xa2);
            let mut mv: *mut MasteringVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_mastering_voice(
                    xa2,
                    &mut mv,
                    master_ch,
                    master_rate,
                    0,
                    std::ptr::null(),
                    std::ptr::null(),
                    6
                ),
                S_OK
            );
            (xa2, mv)
        }
    }

    #[test]
    fn xaudio2_bink_lifecycle() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let mut xa2: *mut XAudio2Engine = std::ptr::null_mut();
            assert_eq!(XAudio2Create(&mut xa2, 0, 1), S_OK);
            assert!(!xa2.is_null());

            // QueryInterface for IXAudio2 and unknown IID.
            let mut qi_out: *mut c_void = std::ptr::null_mut();
            assert_eq!(xa2_query_interface(xa2, &IID_IXAUDIO2, &mut qi_out), S_OK);
            assert_eq!(qi_out, xa2 as *mut c_void);
            assert_eq!(xa2_release(xa2), 1);

            let unknown = GUID::new(0x1111_2222, 0x3333, 0x4444, [5; 8]);
            assert_eq!(xa2_query_interface(xa2, &unknown, &mut qi_out), E_NOINTERFACE);
            assert!(qi_out.is_null());

            xa2_stop_engine(xa2);
            let mut mv: *mut MasteringVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_mastering_voice(
                    xa2,
                    &mut mv,
                    0,
                    0,
                    0,
                    std::ptr::null(),
                    std::ptr::null(),
                    6
                ),
                S_OK
            );
            assert!(!mv.is_null());

            let mut details = XAudio2VoiceDetails {
                creation_flags: 0,
                active_flags: 0,
                input_channels: 0,
                input_sample_rate: 0,
            };
            voice_get_voice_details(mv as *mut VoiceHeader, &mut details);
            assert_eq!({ details.input_channels }, 2);
            assert_eq!({ details.input_sample_rate }, 48_000);

            let mut mask = 0u32;
            assert_eq!(mv_get_channel_mask(mv, &mut mask), S_OK);
            assert_eq!(mask, 0x3);

            let wfx = pcm_wfx(2, 48_000, 16);
            let mut sv: *mut SourceVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_source_voice(
                    xa2,
                    &mut sv,
                    &wfx,
                    XAUDIO2_VOICE_NOPITCH,
                    2.0,
                    0,
                    std::ptr::null(),
                    std::ptr::null()
                ),
                S_OK
            );
            assert!(!sv.is_null());

            let pcm = vec![0u8; 3000];
            let buf = XAudio2Buffer {
                flags: 0,
                audio_bytes: pcm.len() as u32,
                audio_data: pcm.as_ptr(),
                play_begin: 0,
                play_length: 0,
                loop_begin: 0,
                loop_length: 0,
                loop_count: 0,
                context: 0x1234,
            };
            assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
            assert_eq!(sv_start(sv, 0, 0), S_OK);

            let mut st = XAudio2VoiceState {
                current_buffer_context: 0,
                buffers_queued: 16,
                samples_played: 0,
            };
            sv_get_state(sv, &mut st, 0);
            assert_eq!({ st.buffers_queued }, 1);
            assert_eq!({ st.current_buffer_context }, 0x1234);

            voice_destroy_voice(sv as *mut VoiceHeader);
            voice_destroy_voice(mv as *mut VoiceHeader);
            assert_eq!(xa2_release(xa2), 0);
        }
    }

    #[test]
    fn pcm_formats_and_additive_mixing() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let (xa2, mv) = make_stopped_test_engine(2, 48_000);

            // Voice 1: 16-bit signed PCM stereo (L=+0.5, R=-0.5).
            let wfx16 = pcm_wfx(2, 48_000, 16);
            let mut sv16: *mut SourceVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_source_voice(
                    xa2,
                    &mut sv16,
                    &wfx16,
                    0,
                    2.0,
                    0,
                    std::ptr::null(),
                    std::ptr::null()
                ),
                S_OK
            );
            let pcm16: Vec<u8> = [16384i16, -16384i16, 16384i16, -16384i16]
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect();
            let buf16 = XAudio2Buffer {
                flags: 0,
                audio_bytes: pcm16.len() as u32,
                audio_data: pcm16.as_ptr(),
                play_begin: 0,
                play_length: 0,
                loop_begin: 0,
                loop_length: 0,
                loop_count: 0,
                context: 1,
            };
            assert_eq!(sv_submit_source_buffer(sv16, &buf16, 0), S_OK);
            assert_eq!(sv_start(sv16, 0, 0), S_OK);

            // Voice 2: WAVE_FORMAT_EXTENSIBLE 32-bit IEEE float stereo (L=+0.25, R=+0.75).
            let ext_wfx = WaveFormatExtensible {
                format: WaveFormatEx {
                    format_tag: WAVE_FORMAT_EXTENSIBLE,
                    channels: 2,
                    samples_per_sec: 48_000,
                    avg_bytes_per_sec: 48_000 * 8,
                    block_align: 8,
                    bits_per_sample: 32,
                    cb_size: 22,
                },
                valid_bits_per_sample: 32,
                channel_mask: 0x3,
                sub_format: KSDATAFORMAT_SUBTYPE_IEEE_FLOAT,
            };
            let mut svf: *mut SourceVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_source_voice(
                    xa2,
                    &mut svf,
                    &ext_wfx as *const WaveFormatExtensible as *const WaveFormatEx,
                    0,
                    2.0,
                    0,
                    std::ptr::null(),
                    std::ptr::null()
                ),
                S_OK
            );
            let fdata: Vec<u8> = [0.25f32, 0.75f32, 0.25f32, 0.75f32]
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect();
            let buff = XAudio2Buffer {
                flags: 0,
                audio_bytes: fdata.len() as u32,
                audio_data: fdata.as_ptr(),
                play_begin: 0,
                play_length: 0,
                loop_begin: 0,
                loop_length: 0,
                loop_count: 0,
                context: 2,
            };
            assert_eq!(sv_submit_source_buffer(svf, &buff, 0), S_OK);
            assert_eq!(sv_start(svf, 0, 0), S_OK);

            let mut out = [0.0f32; 8];
            let _ = (*xa2).core.mix_quantum(4, &mut out);
            // Frames 0..2: L = 0.5 + 0.25 = 0.75, R = -0.5 + 0.75 = 0.25; frames 2..4 silent.
            assert!((out[0] - 0.75).abs() < 1e-5);
            assert!((out[1] - 0.25).abs() < 1e-5);
            assert!((out[2] - 0.75).abs() < 1e-5);
            assert!((out[3] - 0.25).abs() < 1e-5);
            assert_eq!(out[4], 0.0);
            assert_eq!(out[5], 0.0);

            // Also verify 8-bit, 24-bit, and 32-bit PCM decoding directly.
            let fmt8 = parse_wave_format(&pcm_wfx(1, 48_000, 8)).unwrap();
            let (d8, n8) = decode_pcm_to_f32(fmt8, &[0, 128, 192]);
            assert_eq!(n8, 3);
            assert!((d8[0] - (-1.0)).abs() < 1e-5);
            assert!((d8[1] - 0.0).abs() < 1e-5);
            assert!((d8[2] - 0.5).abs() < 1e-5);

            let fmt24 = parse_wave_format(&pcm_wfx(1, 48_000, 24)).unwrap();
            let (d24, n24) = decode_pcm_to_f32(fmt24, &[0x00, 0x00, 0x80, 0x00, 0x00, 0x40]);
            assert_eq!(n24, 2);
            assert!((d24[0] - (-1.0)).abs() < 1e-5);
            assert!((d24[1] - 0.5).abs() < 1e-5);

            let fmt32 = parse_wave_format(&pcm_wfx(1, 48_000, 32)).unwrap();
            let raw32: Vec<u8> = [i32::MIN, 1 << 30]
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect();
            let (d32, n32) = decode_pcm_to_f32(fmt32, &raw32);
            assert_eq!(n32, 2);
            assert!((d32[0] - (-1.0)).abs() < 1e-5);
            assert!((d32[1] - 0.5).abs() < 1e-5);

            voice_destroy_voice(sv16 as *mut VoiceHeader);
            voice_destroy_voice(svf as *mut VoiceHeader);
            voice_destroy_voice(mv as *mut VoiceHeader);
            assert_eq!(xa2_release(xa2), 0);
        }
    }

    #[test]
    fn resampling_and_frequency_ratio() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let (xa2, mv) = make_stopped_test_engine(1, 48_000);
            // 24 kHz mono source -> 2x upsampling to 48 kHz master.
            let wfx = float_wfx(1, 24_000);
            let mut sv: *mut SourceVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_source_voice(
                    xa2,
                    &mut sv,
                    &wfx,
                    0,
                    4.0,
                    0,
                    std::ptr::null(),
                    std::ptr::null()
                ),
                S_OK
            );

            let src_samples = [0.0f32, 1.0, 2.0, 3.0];
            let raw: Vec<u8> = src_samples.iter().flat_map(|v| v.to_le_bytes()).collect();
            let buf = XAudio2Buffer {
                flags: 0,
                audio_bytes: raw.len() as u32,
                audio_data: raw.as_ptr(),
                play_begin: 0,
                play_length: 0,
                loop_begin: 0,
                loop_length: 0,
                loop_count: 0,
                context: 0,
            };
            assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
            assert_eq!(sv_start(sv, 0, 0), S_OK);

            let mut out = [0.0f32; 8];
            let _ = (*xa2).core.mix_quantum(8, &mut out);
            assert!((out[0] - 0.0).abs() < 1e-5);
            assert!((out[1] - 0.5).abs() < 1e-5);
            assert!((out[2] - 1.0).abs() < 1e-5);
            assert!((out[3] - 1.5).abs() < 1e-5);
            assert!((out[4] - 2.0).abs() < 1e-5);
            assert!((out[5] - 2.5).abs() < 1e-5);
            assert!((out[6] - 3.0).abs() < 1e-5);

            // Change source sample rate to 48 kHz and set frequency ratio = 2.0 -> skips every other sample.
            assert_eq!(sv_set_source_sample_rate(sv, 48_000), S_OK);
            assert_eq!(sv_set_freq_ratio(sv, 2.0, 0), S_OK);
            let mut got_ratio = 0.0f32;
            sv_get_freq_ratio(sv, &mut got_ratio);
            assert_eq!(got_ratio, 2.0);

            assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
            let mut out2 = [0.0f32; 4];
            let _ = (*xa2).core.mix_quantum(4, &mut out2);
            assert!((out2[0] - 0.0).abs() < 1e-5);
            assert!((out2[1] - 2.0).abs() < 1e-5);
            assert_eq!(out2[2], 0.0);

            voice_destroy_voice(sv as *mut VoiceHeader);
            voice_destroy_voice(mv as *mut VoiceHeader);
            assert_eq!(xa2_release(xa2), 0);
        }
    }

    #[test]
    fn play_region_looping_and_exit_loop() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let (xa2, mv) = make_stopped_test_engine(1, 48_000);
            let wfx = float_wfx(1, 48_000);
            let mut sv: *mut SourceVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_source_voice(
                    xa2,
                    &mut sv,
                    &wfx,
                    0,
                    2.0,
                    0,
                    std::ptr::null(),
                    std::ptr::null()
                ),
                S_OK
            );

            let data: Vec<f32> = (0..10).map(|i| i as f32).collect();
            let raw: Vec<u8> = data.iter().flat_map(|v| v.to_le_bytes()).collect();
            let buf = XAudio2Buffer {
                flags: 0,
                audio_bytes: raw.len() as u32,
                audio_data: raw.as_ptr(),
                play_begin: 1,
                play_length: 8, // 1..9
                loop_begin: 2,
                loop_length: 4, // 2..6
                loop_count: 2,  // plays 2..6 three times total
                context: 0xAA,
            };
            assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
            assert_eq!(sv_start(sv, 0, 0), S_OK);

            let mut out = [0.0f32; 20];
            let _ = (*xa2).core.mix_quantum(20, &mut out);
            let expected = [
                1.0, 2.0, 3.0, 4.0, 5.0, 2.0, 3.0, 4.0, 5.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            for (i, (&got, &exp)) in out.iter().zip(expected.iter()).enumerate() {
                assert!((got - exp).abs() < 1e-5, "frame {i}: got {got} expected {exp}");
            }

            let mut st = XAudio2VoiceState {
                current_buffer_context: 1,
                buffers_queued: 9,
                samples_played: 0,
            };
            sv_get_state(sv, &mut st, 0);
            assert_eq!({ st.buffers_queued }, 0);
            assert_eq!({ st.samples_played }, 16);

            // Infinite loop + ExitLoop.
            let inf_buf = XAudio2Buffer {
                flags: 0,
                audio_bytes: raw.len() as u32,
                audio_data: raw.as_ptr(),
                play_begin: 0,
                play_length: 4,
                loop_begin: 0,
                loop_length: 4,
                loop_count: XAUDIO2_LOOP_INFINITE,
                context: 0xBB,
            };
            assert_eq!(sv_submit_source_buffer(sv, &inf_buf, 0), S_OK);
            let mut loop_out = [0.0f32; 16];
            let _ = (*xa2).core.mix_quantum(16, &mut loop_out);
            sv_get_state(sv, &mut st, 0);
            assert_eq!({ st.buffers_queued }, 1);
            assert_eq!({ st.current_buffer_context }, 0xBB);

            assert_eq!(sv_exit_loop(sv, 0), S_OK);
            let mut tail_out = [0.0f32; 8];
            let _ = (*xa2).core.mix_quantum(8, &mut tail_out);
            sv_get_state(sv, &mut st, 0);
            assert_eq!({ st.buffers_queued }, 0);

            // FlushSourceBuffers when stopped empties queue.
            assert_eq!(sv_stop(sv, 0, 0), S_OK);
            assert_eq!(sv_submit_source_buffer(sv, &inf_buf, 0), S_OK);
            assert_eq!(sv_flush_source_buffers(sv), S_OK);
            sv_get_state(sv, &mut st, 0);
            assert_eq!({ st.buffers_queued }, 0);

            voice_destroy_voice(sv as *mut VoiceHeader);
            voice_destroy_voice(mv as *mut VoiceHeader);
            assert_eq!(xa2_release(xa2), 0);
        }
    }

    #[test]
    fn volume_matrix_panning_and_filter() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let (xa2, mv) = make_stopped_test_engine(2, 48_000);
            let wfx = float_wfx(1, 48_000);
            let mut sv: *mut SourceVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_source_voice(
                    xa2,
                    &mut sv,
                    &wfx,
                    0,
                    2.0,
                    0,
                    std::ptr::null(),
                    std::ptr::null()
                ),
                S_OK
            );

            assert_eq!(voice_set_volume(sv as *mut VoiceHeader, 0.5, 0), S_OK);
            let mut vol = 0.0f32;
            voice_get_volume(sv as *mut VoiceHeader, &mut vol);
            assert_eq!(vol, 0.5);

            let ch_vols = [0.5f32];
            assert_eq!(
                voice_set_channel_volumes(sv as *mut VoiceHeader, 1, ch_vols.as_ptr(), 0),
                S_OK
            );
            let mut got_ch_vols = [0.0f32; 1];
            voice_get_channel_volumes(sv as *mut VoiceHeader, 1, got_ch_vols.as_mut_ptr());
            assert_eq!(got_ch_vols, ch_vols);
            assert_eq!(
                voice_set_channel_volumes(sv as *mut VoiceHeader, 1, [1.0f32].as_ptr(), 0),
                S_OK
            );

            // Pan mono -> stereo: Left=0.25, Right=0.75 (XAudio2 layout: [D=0,S=0]=0.25, [D=1,S=0]=0.75).
            let mat = [0.25f32, 0.75f32];
            assert_eq!(
                voice_set_output_matrix(
                    sv as *mut VoiceHeader,
                    std::ptr::null_mut(),
                    1,
                    2,
                    mat.as_ptr(),
                    0
                ),
                S_OK
            );
            let mut got_mat = [0.0f32; 2];
            voice_get_output_matrix(
                sv as *mut VoiceHeader,
                std::ptr::null_mut(),
                1,
                2,
                got_mat.as_mut_ptr(),
            );
            assert_eq!(got_mat, mat);

            let ones: Vec<u8> = [1.0f32; 4].iter().flat_map(|v| v.to_le_bytes()).collect();
            let buf = XAudio2Buffer {
                flags: 0,
                audio_bytes: ones.len() as u32,
                audio_data: ones.as_ptr(),
                play_begin: 0,
                play_length: 0,
                loop_begin: 0,
                loop_length: 0,
                loop_count: 0,
                context: 0,
            };
            assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
            assert_eq!(sv_start(sv, 0, 0), S_OK);

            let mut out = [0.0f32; 8];
            let _ = (*xa2).core.mix_quantum(4, &mut out);
            assert!((out[0] - 0.125).abs() < 1e-5); // 1.0 * 0.5 * 0.25
            assert!((out[1] - 0.375).abs() < 1e-5); // 1.0 * 0.5 * 0.75

            // Low-pass filter should attenuate Nyquist (+1/-1) far more than DC (+1).
            let lpf = XAudio2FilterParameters {
                filter_type: FILTER_LOW_PASS,
                frequency: 0.1,
                one_over_q: 1.0,
            };
            assert_eq!(voice_set_filter_parameters(sv as *mut VoiceHeader, &lpf, 0), S_OK);
            let mut got_lpf = XAudio2FilterParameters::default();
            voice_get_filter_parameters(sv as *mut VoiceHeader, &mut got_lpf);
            assert_eq!({ got_lpf.filter_type }, FILTER_LOW_PASS);
            assert!((got_lpf.frequency - 0.1).abs() < 1e-6);

            let mut alt = vec![0.0f32; 128];
            for (i, v) in alt.iter_mut().enumerate() {
                *v = if i % 2 == 0 { 1.0 } else { -1.0 };
            }
            let alt_raw: Vec<u8> = alt.iter().flat_map(|v| v.to_le_bytes()).collect();
            let alt_buf = XAudio2Buffer {
                flags: 0,
                audio_bytes: alt_raw.len() as u32,
                audio_data: alt_raw.as_ptr(),
                play_begin: 0,
                play_length: 0,
                loop_begin: 0,
                loop_length: 0,
                loop_count: 0,
                context: 0,
            };
            assert_eq!(voice_set_volume(sv as *mut VoiceHeader, 1.0, 0), S_OK);
            assert_eq!(
                voice_set_output_matrix(
                    sv as *mut VoiceHeader,
                    std::ptr::null_mut(),
                    1,
                    2,
                    [1.0f32, 1.0f32].as_ptr(),
                    0
                ),
                S_OK
            );
            assert_eq!(sv_submit_source_buffer(sv, &alt_buf, 0), S_OK);
            let mut nyq_out = vec![0.0f32; 256];
            let _ = (*xa2).core.mix_quantum(128, &mut nyq_out);
            let steady_peak = nyq_out[128..]
                .iter()
                .map(|v| v.abs())
                .fold(0.0f32, f32::max);
            assert!(steady_peak < 0.05, "LPF failed to attenuate Nyquist: peak={steady_peak}");

            voice_destroy_voice(sv as *mut VoiceHeader);
            voice_destroy_voice(mv as *mut VoiceHeader);
            assert_eq!(xa2_release(xa2), 0);
        }
    }

    #[test]
    fn submix_voice_multi_stage_routing() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let (xa2, mv) = make_stopped_test_engine(2, 48_000);

            // Stage 1 submix -> Mastering voice.
            let mut sub1: *mut SubmixVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_submix_voice(
                    xa2,
                    &mut sub1,
                    2,
                    48_000,
                    0,
                    1,
                    std::ptr::null(),
                    std::ptr::null()
                ),
                S_OK
            );

            // Stage 0 submix -> Stage 1 submix.
            let sub0_send = XAudio2SendDescriptor {
                flags: 0,
                output_voice: sub1 as *mut VoiceHeader,
            };
            let sub0_sends = XAudio2VoiceSends {
                send_count: 1,
                sends: &sub0_send,
            };
            let mut sub0: *mut SubmixVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_submix_voice(
                    xa2,
                    &mut sub0,
                    2,
                    48_000,
                    0,
                    0,
                    &sub0_sends,
                    std::ptr::null()
                ),
                S_OK
            );

            // Source -> Stage 0 submix.
            let src_send = XAudio2SendDescriptor {
                flags: 0,
                output_voice: sub0 as *mut VoiceHeader,
            };
            let src_sends = XAudio2VoiceSends {
                send_count: 1,
                sends: &src_send,
            };
            let wfx = float_wfx(2, 48_000);
            let mut sv: *mut SourceVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_source_voice(
                    xa2,
                    &mut sv,
                    &wfx,
                    0,
                    2.0,
                    0,
                    &src_sends,
                    std::ptr::null()
                ),
                S_OK
            );

            assert_eq!(voice_set_volume(sub0 as *mut VoiceHeader, 0.5, 0), S_OK);
            assert_eq!(voice_set_volume(sub1 as *mut VoiceHeader, 0.5, 0), S_OK);
            assert_eq!(voice_set_volume(mv as *mut VoiceHeader, 0.8, 0), S_OK);

            let raw: Vec<u8> = [1.0f32, -1.0, 1.0, -1.0]
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect();
            let buf = XAudio2Buffer {
                flags: 0,
                audio_bytes: raw.len() as u32,
                audio_data: raw.as_ptr(),
                play_begin: 0,
                play_length: 0,
                loop_begin: 0,
                loop_length: 0,
                loop_count: 0,
                context: 0,
            };
            assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
            assert_eq!(sv_start(sv, 0, 0), S_OK);

            let mut out = [0.0f32; 4];
            let _ = (*xa2).core.mix_quantum(2, &mut out);
            // 1.0 * 0.5 * 0.5 * 0.8 = 0.2
            assert!((out[0] - 0.2).abs() < 1e-5);
            assert!((out[1] - (-0.2)).abs() < 1e-5);

            // Route source to empty send list -> silent.
            let empty_sends = XAudio2VoiceSends {
                send_count: 0,
                sends: std::ptr::null(),
            };
            assert_eq!(
                voice_set_output_voices(sv as *mut VoiceHeader, &empty_sends),
                S_OK
            );
            assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
            let mut silent = [9.0f32; 4];
            let _ = (*xa2).core.mix_quantum(2, &mut silent);
            assert_eq!(silent, [0.0; 4]);

            voice_destroy_voice(sv as *mut VoiceHeader);
            voice_destroy_voice(sub0 as *mut VoiceHeader);
            voice_destroy_voice(sub1 as *mut VoiceHeader);
            voice_destroy_voice(mv as *mut VoiceHeader);
            assert_eq!(xa2_release(xa2), 0);
        }
    }

    #[test]
    fn voice_callbacks_delivered_safely() {
        #[repr(C)]
        #[allow(dead_code)]
        struct MockVoiceCbVtbl {
            on_pass_start: unsafe extern "win64" fn(*mut MockVoiceCb, u32),
            on_pass_end: unsafe extern "win64" fn(*mut MockVoiceCb),
            on_stream_end: unsafe extern "win64" fn(*mut MockVoiceCb),
            on_buf_start: unsafe extern "win64" fn(*mut MockVoiceCb, usize),
            on_buf_end: unsafe extern "win64" fn(*mut MockVoiceCb, usize),
            on_loop_end: unsafe extern "win64" fn(*mut MockVoiceCb, usize),
            on_error: unsafe extern "win64" fn(*mut MockVoiceCb, usize, i32),
        }

        #[repr(C)]
        #[allow(dead_code)]
        struct MockVoiceCb {
            vtbl: *const MockVoiceCbVtbl,
            starts: AtomicU32,
            ends: AtomicU32,
            loops: AtomicU32,
            streams: AtomicU32,
            last_ctx: AtomicU64,
        }

        unsafe extern "win64" fn on_pass_start(_this: *mut MockVoiceCb, _bytes: u32) {}
        unsafe extern "win64" fn on_pass_end(_this: *mut MockVoiceCb) {}
        unsafe extern "win64" fn on_stream_end(this: *mut MockVoiceCb) {
            (*this).streams.fetch_add(1, Ordering::AcqRel);
        }
        unsafe extern "win64" fn on_buf_start(this: *mut MockVoiceCb, ctx: usize) {
            (*this).starts.fetch_add(1, Ordering::AcqRel);
            (*this).last_ctx.store(ctx as u64, Ordering::Release);
        }
        unsafe extern "win64" fn on_buf_end(this: *mut MockVoiceCb, ctx: usize) {
            (*this).ends.fetch_add(1, Ordering::AcqRel);
            (*this).last_ctx.store(ctx as u64, Ordering::Release);
        }
        unsafe extern "win64" fn on_loop_end(this: *mut MockVoiceCb, ctx: usize) {
            (*this).loops.fetch_add(1, Ordering::AcqRel);
            (*this).last_ctx.store(ctx as u64, Ordering::Release);
        }
        unsafe extern "win64" fn on_error(_this: *mut MockVoiceCb, _ctx: usize, _hr: i32) {}

        static MOCK_VTBL: MockVoiceCbVtbl = MockVoiceCbVtbl {
            on_pass_start,
            on_pass_end,
            on_stream_end,
            on_buf_start,
            on_buf_end,
            on_loop_end,
            on_error,
        };

        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let (xa2, mv) = make_stopped_test_engine(1, 48_000);
            let cb = Box::new(MockVoiceCb {
                vtbl: &MOCK_VTBL,
                starts: AtomicU32::new(0),
                ends: AtomicU32::new(0),
                loops: AtomicU32::new(0),
                streams: AtomicU32::new(0),
                last_ctx: AtomicU64::new(0),
            });
            let cb_ptr = &*cb as *const MockVoiceCb as usize;

            let wfx = float_wfx(1, 48_000);
            let mut sv: *mut SourceVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_source_voice(
                    xa2,
                    &mut sv,
                    &wfx,
                    0,
                    2.0,
                    cb_ptr,
                    std::ptr::null(),
                    std::ptr::null()
                ),
                S_OK
            );

            let raw: Vec<u8> = [0.25f32; 4].iter().flat_map(|v| v.to_le_bytes()).collect();
            let buf = XAudio2Buffer {
                flags: XAUDIO2_END_OF_STREAM,
                audio_bytes: raw.len() as u32,
                audio_data: raw.as_ptr(),
                play_begin: 0,
                play_length: 4,
                loop_begin: 0,
                loop_length: 4,
                loop_count: 1,
                context: 0xCAFE,
            };
            assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
            assert_eq!(sv_start(sv, 0, 0), S_OK);

            let mut out = [0.0f32; 12];
            let events = (*xa2).core.mix_quantum(12, &mut out);
            (*xa2).core.dispatch_callbacks(events);
            (*xa2).core.sync_callbacks();

            assert_eq!(cb.starts.load(Ordering::Acquire), 1);
            assert_eq!(cb.loops.load(Ordering::Acquire), 1);
            assert_eq!(cb.ends.load(Ordering::Acquire), 1);
            assert_eq!(cb.streams.load(Ordering::Acquire), 1);
            assert_eq!(cb.last_ctx.load(Ordering::Acquire), 0xCAFE);

            voice_destroy_voice(sv as *mut VoiceHeader);
            voice_destroy_voice(mv as *mut VoiceHeader);
            assert_eq!(xa2_release(xa2), 0);
        }
    }

    #[test]
    fn realtime_null_backend_mixer_drains_buffers() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let mut xa2: *mut XAudio2Engine = std::ptr::null_mut();
            assert_eq!(XAudio2Create(&mut xa2, 0, 1), S_OK);
            let mut mv: *mut MasteringVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_mastering_voice(
                    xa2,
                    &mut mv,
                    2,
                    48_000,
                    0,
                    std::ptr::null(),
                    std::ptr::null(),
                    6
                ),
                S_OK
            );
            let wfx = pcm_wfx(2, 48_000, 16);
            let mut sv: *mut SourceVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_source_voice(
                    xa2,
                    &mut sv,
                    &wfx,
                    0,
                    2.0,
                    0,
                    std::ptr::null(),
                    std::ptr::null()
                ),
                S_OK
            );

            // 960 stereo frames = 20 ms at 48 kHz.
            let pcm = vec![0u8; 960 * 4];
            let buf = XAudio2Buffer {
                flags: XAUDIO2_END_OF_STREAM,
                audio_bytes: pcm.len() as u32,
                audio_data: pcm.as_ptr(),
                play_begin: 0,
                play_length: 0,
                loop_begin: 0,
                loop_length: 0,
                loop_count: 0,
                context: 0x55,
            };
            assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
            assert_eq!(sv_start(sv, 0, 0), S_OK);

            let deadline = std::time::Instant::now() + Duration::from_millis(500);
            let mut st = XAudio2VoiceState {
                current_buffer_context: 0,
                buffers_queued: 1,
                samples_played: 0,
            };
            while std::time::Instant::now() < deadline {
                sv_get_state(sv, &mut st, 0);
                if st.buffers_queued == 0 {
                    break;
                }
                std::thread::sleep(Duration::from_millis(5));
            }
            assert_eq!({ st.buffers_queued }, 0);
            assert_eq!({ st.samples_played }, 960);

            let mut perf = XAudio2PerformanceData::default();
            xa2_get_perf_data(xa2, &mut perf);
            assert_eq!({ perf.total_source_voice_count }, 1);

            voice_destroy_voice(sv as *mut VoiceHeader);
            voice_destroy_voice(mv as *mut VoiceHeader);
            assert_eq!(xa2_release(xa2), 0);
        }
    }

    #[test]
    fn x3daudio_attenuation_panning_doppler_and_surround() {
        unsafe {
            let mut handle = [0u8; 20];
            assert_eq!(X3DAudioInitialize(0x3, X3DAUDIO_SPEED_OF_SOUND, &mut handle), S_OK);

            let listener = X3DAudioListener {
                orient_front: X3DAudioVector { x: 0.0, y: 0.0, z: 1.0 },
                orient_top: X3DAudioVector { x: 0.0, y: 1.0, z: 0.0 },
                position: X3DAudioVector { x: 0.0, y: 0.0, z: 0.0 },
                velocity: X3DAudioVector { x: 0.0, y: 0.0, z: 0.0 },
                cone: std::ptr::null(),
            };

            let base_emitter = X3DAudioEmitter {
                cone: std::ptr::null(),
                orient_front: X3DAudioVector { x: 0.0, y: 0.0, z: -1.0 },
                orient_top: X3DAudioVector { x: 0.0, y: 1.0, z: 0.0 },
                position: X3DAudioVector { x: -10.0, y: 0.0, z: 0.0 },
                velocity: X3DAudioVector { x: 0.0, y: 0.0, z: 0.0 },
                inner_radius: 0.0,
                inner_radius_angle: 0.0,
                channel_count: 1,
                channel_radius: 0.0,
                channel_azimuths: std::ptr::null(),
                volume_curve: std::ptr::null(),
                lfe_curve: std::ptr::null(),
                lpf_direct_curve: std::ptr::null(),
                lpf_reverb_curve: std::ptr::null(),
                reverb_curve: std::ptr::null(),
                curve_distance_scaler: 2.0,
                doppler_scaler: 1.0,
            };

            let mut mat = [0.0f32; 2];
            let mut dsp = X3DAudioDspSettings {
                matrix_coefficients: mat.as_mut_ptr(),
                delay_times: std::ptr::null_mut(),
                src_channel_count: 1,
                dst_channel_count: 2,
                lpf_direct_coefficient: 0.0,
                lpf_reverb_coefficient: 0.0,
                reverb_level: 0.0,
                doppler_factor: 1.0,
                emitter_to_listener_angle: 0.0,
                emitter_to_listener_distance: 0.0,
                emitter_velocity_component: 0.0,
                listener_velocity_component: 0.0,
            };

            let flags = X3DAUDIO_CALCULATE_MATRIX
                | X3DAUDIO_CALCULATE_DOPPLER
                | X3DAUDIO_CALCULATE_LPF_DIRECT
                | X3DAUDIO_CALCULATE_EMITTER_ANGLE;

            let call_x3d = |h: &[u8; 20],
                            l: &X3DAudioListener,
                            e: &X3DAudioEmitter,
                            f: u32,
                            d: &mut X3DAudioDspSettings| {
                X3DAudioCalculate(
                    h,
                    l as *const X3DAudioListener as *const c_void,
                    e as *const X3DAudioEmitter as *const c_void,
                    f,
                    d as *mut X3DAudioDspSettings as *mut c_void,
                );
            };

            // Hard left at distance 10 with scaler 2 -> attenuation = 2 / 10 = 0.2.
            call_x3d(&handle, &listener, &base_emitter, flags, &mut dsp);
            assert!((dsp.emitter_to_listener_distance - 10.0).abs() < 1e-4);
            assert!((mat[0] - 0.2).abs() < 1e-4, "left={}", mat[0]);
            assert!(mat[1].abs() < 1e-4, "right={}", mat[1]);
            assert!((dsp.lpf_direct_coefficient - 0.75).abs() < 1e-4);

            // Hard right at distance 10.
            let right_emitter = X3DAudioEmitter {
                position: X3DAudioVector { x: 10.0, y: 0.0, z: 0.0 },
                ..base_emitter
            };
            call_x3d(&handle, &listener, &right_emitter, flags, &mut dsp);
            assert!(mat[0].abs() < 1e-4, "left={}", mat[0]);
            assert!((mat[1] - 0.2).abs() < 1e-4, "right={}", mat[1]);

            // Center front at distance 2 (within CurveDistanceScaler) + approaching Doppler.
            let approach_emitter = X3DAudioEmitter {
                position: X3DAudioVector { x: 0.0, y: 0.0, z: 2.0 },
                velocity: X3DAudioVector { x: 0.0, y: 0.0, z: -34.35 },
                ..base_emitter
            };
            call_x3d(&handle, &listener, &approach_emitter, flags, &mut dsp);
            assert!((mat[0] - FRAC_1_SQRT_2).abs() < 1e-4);
            assert!((mat[1] - FRAC_1_SQRT_2).abs() < 1e-4);
            assert!((dsp.doppler_factor - (343.5 / (343.5 - 34.35))).abs() < 1e-3);

            // Custom volume curve: (0->1.0, 5->0.5, 10->0.0), evaluated at scaled_dist = 2.5 -> 0.75.
            let pts = [
                X3DAudioDistanceCurvePoint { distance: 0.0, dsp_setting: 1.0 },
                X3DAudioDistanceCurvePoint { distance: 5.0, dsp_setting: 0.5 },
                X3DAudioDistanceCurvePoint { distance: 10.0, dsp_setting: 0.0 },
            ];
            let curve = X3DAudioDistanceCurve {
                points: pts.as_ptr(),
                point_count: pts.len() as u32,
            };
            let custom_emitter = X3DAudioEmitter {
                position: X3DAudioVector { x: 0.0, y: 0.0, z: 2.5 },
                curve_distance_scaler: 1.0,
                volume_curve: &curve,
                ..base_emitter
            };
            call_x3d(&handle, &listener, &custom_emitter, flags, &mut dsp);
            let expected_ch = 0.75 * FRAC_1_SQRT_2;
            assert!((mat[0] - expected_ch).abs() < 1e-4);
            assert!((mat[1] - expected_ch).abs() < 1e-4);

            // 5.1 surround (0x3F = FL, FR, FC, LFE, BL, BR): sound straight ahead -> FC (index 2).
            let mut handle51 = [0u8; 20];
            assert_eq!(X3DAudioInitialize(0x3F, X3DAUDIO_SPEED_OF_SOUND, &mut handle51), S_OK);
            let mut mat51 = [0.0f32; 6];
            let mut dsp51 = X3DAudioDspSettings {
                matrix_coefficients: mat51.as_mut_ptr(),
                dst_channel_count: 6,
                ..dsp
            };
            let front_emitter = X3DAudioEmitter {
                position: X3DAudioVector { x: 0.0, y: 0.0, z: 1.0 },
                curve_distance_scaler: 1.0,
                ..base_emitter
            };
            call_x3d(&handle51, &listener, &front_emitter, X3DAUDIO_CALCULATE_MATRIX, &mut dsp51);
            assert!((mat51[2] - 1.0).abs() < 1e-4, "FC={}", mat51[2]);
            assert!(mat51[4].abs() < 1e-4 && mat51[5].abs() < 1e-4);
        }
    }

    fn submit_f32_buf(sv: *mut SourceVoice, samples: &[f32]) {
        unsafe {
            let raw: Vec<u8> = samples.iter().flat_map(|v| v.to_le_bytes()).collect();
            let buf = XAudio2Buffer {
                flags: XAUDIO2_END_OF_STREAM,
                audio_bytes: raw.len() as u32,
                audio_data: raw.as_ptr(),
                play_begin: 0,
                play_length: 0,
                loop_begin: 0,
                loop_length: 0,
                loop_count: 0,
                context: 0,
            };
            assert_eq!(sv_submit_source_buffer(sv, &buf, 0), S_OK);
        }
    }

    fn rms(samples: &[f32]) -> f32 {
        if samples.is_empty() {
            return 0.0;
        }
        let sum_sq: f32 = samples.iter().map(|&v| v * v).sum();
        (sum_sq / samples.len() as f32).sqrt()
    }

    #[test]
    fn xapo_com_interfaces_and_createfx_clsids() {
        unsafe {
            // Null pointer checks.
            assert_eq!(CreateAudioReverb(std::ptr::null_mut()), E_POINTER);
            assert_eq!(CreateAudioVolumeMeter(std::ptr::null_mut()), E_POINTER);
            let mut out = 0xDEADusize;
            assert_eq!(CreateFX(std::ptr::null(), &mut out, 0, 0), E_POINTER);
            assert_eq!(out, 0);

            // Unknown CLSID -> REGDB_E_CLASSNOTREG.
            let unknown_clsid = GUID::new(0x1234_5678, 0x9ABC, 0xDEF0, [1, 2, 3, 4, 5, 6, 7, 8]);
            out = 0xDEAD;
            assert_eq!(
                CreateFX(&unknown_clsid as *const GUID as *const u8, &mut out, 0, 0),
                REGDB_E_CLASSNOTREG
            );
            assert_eq!(out, 0);

            // Create all 6 builtin effects (including legacy 2.7 XAPOFX CLSIDs) and test COM QI.
            let mut objs: Vec<(usize, GUID)> = Vec::new();
            let mut rev = 0usize;
            assert_eq!(CreateAudioReverb(&mut rev), S_OK);
            assert_ne!(rev, 0);
            objs.push((rev, CLSID_AUDIO_REVERB));

            let mut vm = 0usize;
            assert_eq!(CreateAudioVolumeMeter(&mut vm), S_OK);
            assert_ne!(vm, 0);
            objs.push((vm, CLSID_AUDIO_VOLUME_METER));

            for &(clsid, expected_clsid) in &[
                (CLSID_FX_EQ, CLSID_FX_EQ),
                (CLSID_FX_MASTERING_LIMITER, CLSID_FX_MASTERING_LIMITER),
                (CLSID_FX_REVERB, CLSID_FX_REVERB),
                (CLSID_FX_ECHO, CLSID_FX_ECHO),
                (CLSID_FX_EQ_27, CLSID_FX_EQ),
                (CLSID_FX_MASTERING_LIMITER_27, CLSID_FX_MASTERING_LIMITER),
                (CLSID_FX_REVERB_27, CLSID_FX_REVERB),
                (CLSID_FX_ECHO_27, CLSID_FX_ECHO),
            ] {
                let mut fx = 0usize;
                assert_eq!(
                    CreateFX(&clsid as *const GUID as *const u8, &mut fx, 0, 0),
                    S_OK
                );
                assert_ne!(fx, 0);
                objs.push((fx, expected_clsid));
            }

            let bad_iid = GUID::new(0xAAAA_BBBB, 0xCCCC, 0xDDDD, [0xEE; 8]);
            let f32_2ch = float_wfx(2, 48_000);
            let pcm16_2ch = pcm_wfx(2, 48_000, 16);

            for (obj, expected_clsid) in objs {
                let xapo = obj as *mut BuiltinXapoCom;
                // QI for IXAPO returns primary interface pointer.
                let mut p_xapo: *mut c_void = std::ptr::null_mut();
                assert_eq!(xapo_query_interface(xapo, &IID_IXAPO, &mut p_xapo), S_OK);
                assert_eq!(p_xapo as usize, obj);

                // QI for IXAPOParameters returns distinct secondary interface pointer.
                let mut p_params: *mut c_void = std::ptr::null_mut();
                assert_eq!(
                    xapo_query_interface(xapo, &IID_IXAPO_PARAMETERS, &mut p_params),
                    S_OK
                );
                assert!(!p_params.is_null());
                assert_ne!(p_params as usize, obj);

                // Round-trip QI from IXAPOParameters -> IUnknown & IXAPO obeys COM identity.
                let mut p_unk_back: *mut c_void = std::ptr::null_mut();
                assert_eq!(
                    xapo_params_query_interface(p_params, &IID_IUnknown, &mut p_unk_back),
                    S_OK
                );
                assert_eq!(p_unk_back as usize, obj);

                let mut p_xapo_back: *mut c_void = std::ptr::null_mut();
                assert_eq!(
                    xapo_params_query_interface(p_params, &IID_IXAPO, &mut p_xapo_back),
                    S_OK
                );
                assert_eq!(p_xapo_back as usize, obj);

                let mut p_bad: *mut c_void = 0x1234 as *mut c_void;
                assert_eq!(
                    xapo_params_query_interface(p_params, &bad_iid, &mut p_bad),
                    E_NOINTERFACE
                );
                assert!(p_bad.is_null());

                // Registration properties.
                let mut props_ptr: *mut XapoRegistrationProperties = std::ptr::null_mut();
                assert_eq!(xapo_get_registration_properties(xapo, &mut props_ptr), S_OK);
                assert!(!props_ptr.is_null());
                let props = std::ptr::read_unaligned(props_ptr);
                assert_eq!({ props.clsid }, expected_clsid);
                assert_eq!({ props.min_input_buffer_count }, 1);
                assert_eq!({ props.max_output_buffer_count }, 1);
                let friendly = { props.friendly_name };
                assert_ne!(friendly[0], 0);
                crate::heap::free(props_ptr as *mut c_void);

                // Format negotiation & frame counts.
                let mut sup: *mut WaveFormatEx = std::ptr::null_mut();
                assert_eq!(
                    xapo_is_input_format_supported(xapo, &f32_2ch, &f32_2ch, &mut sup),
                    S_OK
                );
                assert_eq!(
                    xapo_is_output_format_supported(xapo, &f32_2ch, &pcm16_2ch, &mut sup),
                    XAPO_E_FORMAT_UNSUPPORTED
                );
                assert!(!sup.is_null());
                let suggested = std::ptr::read_unaligned(sup);
                assert_eq!({ suggested.format_tag }, WAVE_FORMAT_IEEE_FLOAT);
                assert_eq!({ suggested.bits_per_sample }, 32);
                crate::heap::free(sup as *mut c_void);

                assert_eq!(xapo_calc_input_frames(xapo, 480), 480);
                assert_eq!(xapo_calc_output_frames(xapo, 480), 480);

                // Release all 5 refs (initial + p_xapo + p_params + p_unk_back + p_xapo_back).
                assert_eq!(xapo_params_release(p_params), 4);
                assert_eq!(xapo_release(p_xapo_back as *mut BuiltinXapoCom), 3);
                assert_eq!(xapo_release(p_unk_back as *mut BuiltinXapoCom), 2);
                assert_eq!(xapo_release(p_xapo as *mut BuiltinXapoCom), 1);
                assert_eq!(xapo_release(xapo), 0);
            }
        }
    }

    #[test]
    fn audio_reverb_and_fxreverb_decaying_tail() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let (xa2, mv) = make_stopped_test_engine(2, 48_000);
            let mut rev_ptr = 0usize;
            assert_eq!(CreateAudioReverb(&mut rev_ptr), S_OK);

            let desc = XAudio2EffectDescriptor {
                effect: rev_ptr as *mut c_void,
                initial_state: 1,
                output_channels: 2,
            };
            let chain = XAudio2EffectChain {
                effect_count: 1,
                effects: &desc,
            };

            // Attach AudioReverb directly via CreateSubmixVoice.
            let mut sub: *mut SubmixVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_submix_voice(xa2, &mut sub, 2, 48_000, 0, 0, std::ptr::null(), &chain),
                S_OK
            );
            // Caller releases its own COM ref; voice still holds the effect alive.
            assert_eq!(xapo_release(rev_ptr as *mut BuiltinXapoCom), 1);

            // Configure reverb parameters and verify round-trip (2.9 + 2.7 struct sizes).
            let rev_params = XAudio2FxReverbParameters {
                wet_dry_mix: 100.0,
                reflections_delay: 2,
                reverb_delay: 4,
                decay_time: 0.6,
                density: 100.0,
                room_size: 80.0,
                ..XAudio2FxReverbParameters::default()
            };
            assert_eq!(
                voice_set_effect_parameters(
                    sub as *mut VoiceHeader,
                    0,
                    &rev_params as *const XAudio2FxReverbParameters as *const c_void,
                    std::mem::size_of::<XAudio2FxReverbParameters>() as u32,
                    0
                ),
                S_OK
            );
            let mut got_rev = XAudio2FxReverbParameters::default();
            assert_eq!(
                voice_get_effect_parameters(
                    sub as *mut VoiceHeader,
                    0,
                    &mut got_rev as *mut XAudio2FxReverbParameters as *mut c_void,
                    std::mem::size_of::<XAudio2FxReverbParameters>() as u32,
                ),
                S_OK
            );
            assert!((got_rev.decay_time - 0.6).abs() < 1e-6);
            assert!((got_rev.room_size - 80.0).abs() < 1e-6);

            let mut got_rev27 = std::mem::zeroed::<XAudio2FxReverbParameters27>();
            assert_eq!(
                voice_get_effect_parameters(
                    sub as *mut VoiceHeader,
                    0,
                    &mut got_rev27 as *mut XAudio2FxReverbParameters27 as *mut c_void,
                    std::mem::size_of::<XAudio2FxReverbParameters27>() as u32,
                ),
                S_OK
            );
            assert!((got_rev27.decay_time - 0.6).abs() < 1e-6);

            // Route a stereo source voice into the reverb submix and fire a brief impulse burst.
            let send = XAudio2SendDescriptor {
                flags: 0,
                output_voice: sub as *mut VoiceHeader,
            };
            let sends = XAudio2VoiceSends {
                send_count: 1,
                sends: &send,
            };
            let wfx = float_wfx(2, 48_000);
            let mut sv: *mut SourceVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_source_voice(xa2, &mut sv, &wfx, 0, 2.0, 0, &sends, std::ptr::null()),
                S_OK
            );
            let mut burst = vec![0.0f32; 64 * 2];
            for i in 0..8 {
                burst[i * 2] = 0.8;
                burst[i * 2 + 1] = 0.8;
            }
            submit_f32_buf(sv, &burst);
            assert_eq!(sv_start(sv, 0, 0), S_OK);

            // Mix 1.0 second total across 100 quantums of 480 frames (10 ms each).
            let mut blocks = Vec::new();
            for _ in 0..100 {
                let mut q = vec![0.0f32; 480 * 2];
                let _ = (&*xa2).core.mix_quantum(480, &mut q);
                blocks.push(q);
            }
            // Source burst finished inside block 0; blocks 3..15 are early/mid tail, blocks 40..60 late tail.
            let early_rms = rms(&blocks[5]);
            let mid_rms = rms(&blocks[25]);
            let late_rms = rms(&blocks[75]);
            assert!(early_rms > 0.01, "expected audible reverb tail, early_rms={early_rms}");
            assert!(mid_rms > 0.0005, "expected sustained mid tail, mid_rms={mid_rms}");
            assert!(
                early_rms > mid_rms && mid_rms > late_rms,
                "expected decaying reverb tail: early={early_rms} mid={mid_rms} late={late_rms}"
            );
            // All samples finite and bounded.
            for b in &blocks {
                for &s in b {
                    assert!(s.is_finite() && s.abs() < 4.0);
                }
            }

            // DisableLateField should silence the late tail.
            let no_late = XAudio2FxReverbParameters {
                disable_late_field: 1,
                ..rev_params
            };
            assert_eq!(
                voice_set_effect_parameters(
                    sub as *mut VoiceHeader,
                    0,
                    &no_late as *const XAudio2FxReverbParameters as *const c_void,
                    std::mem::size_of::<XAudio2FxReverbParameters>() as u32,
                    0
                ),
                S_OK
            );
            let mut after_disable = vec![0.0f32; 480 * 2];
            for _ in 0..10 {
                let _ = (&*xa2).core.mix_quantum(480, &mut after_disable);
            }
            assert!(
                rms(&after_disable) < 1e-5,
                "DisableLateField should eliminate late tail, got {}",
                rms(&after_disable)
            );

            // Now swap in CreateFX(CLSID_FX_REVERB) on the submix and confirm decaying tail too.
            let mut fxrev_ptr = 0usize;
            assert_eq!(
                CreateFX(&CLSID_FX_REVERB as *const GUID as *const u8, &mut fxrev_ptr, 0, 0),
                S_OK
            );
            let fxdesc = XAudio2EffectDescriptor {
                effect: fxrev_ptr as *mut c_void,
                initial_state: 1,
                output_channels: 2,
            };
            let fxchain = XAudio2EffectChain {
                effect_count: 1,
                effects: &fxdesc,
            };
            assert_eq!(voice_set_effect_chain(sub as *mut VoiceHeader, &fxchain), S_OK);
            assert_eq!(xapo_release(fxrev_ptr as *mut BuiltinXapoCom), 1);

            let fx_params = FxReverbParameters {
                diffusion: 0.9,
                room_size: 0.85,
            };
            assert_eq!(
                voice_set_effect_parameters(
                    sub as *mut VoiceHeader,
                    0,
                    &fx_params as *const FxReverbParameters as *const c_void,
                    std::mem::size_of::<FxReverbParameters>() as u32,
                    0
                ),
                S_OK
            );
            let mut got_fx = FxReverbParameters::default();
            assert_eq!(
                voice_get_effect_parameters(
                    sub as *mut VoiceHeader,
                    0,
                    &mut got_fx as *mut FxReverbParameters as *mut c_void,
                    std::mem::size_of::<FxReverbParameters>() as u32,
                ),
                S_OK
            );
            assert!((got_fx.room_size - 0.85).abs() < 1e-6);

            submit_f32_buf(sv, &burst);
            let mut fx_blocks = Vec::new();
            for _ in 0..80 {
                let mut q = vec![0.0f32; 480 * 2];
                let _ = (&*xa2).core.mix_quantum(480, &mut q);
                fx_blocks.push(q);
            }
            let fx_early = rms(&fx_blocks[5]);
            let fx_late = rms(&fx_blocks[65]);
            assert!(fx_early > 0.005, "FXReverb early tail too weak: {fx_early}");
            assert!(fx_early > fx_late, "FXReverb tail should decay: early={fx_early} late={fx_late}");

            voice_destroy_voice(sv as *mut VoiceHeader);
            voice_destroy_voice(sub as *mut VoiceHeader);
            voice_destroy_voice(mv as *mut VoiceHeader);
            assert_eq!(xa2_release(xa2), 0);
        }
    }

    #[test]
    fn volume_meter_live_peak_and_rms() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let (xa2, mv) = make_stopped_test_engine(2, 48_000);
            let mut vm_ptr = 0usize;
            assert_eq!(CreateAudioVolumeMeter(&mut vm_ptr), S_OK);

            let desc = XAudio2EffectDescriptor {
                effect: vm_ptr as *mut c_void,
                initial_state: 1,
                output_channels: 2,
            };
            let chain = XAudio2EffectChain {
                effect_count: 1,
                effects: &desc,
            };
            assert_eq!(voice_set_effect_chain(mv as *mut VoiceHeader, &chain), S_OK);
            assert_eq!(xapo_release(vm_ptr as *mut BuiltinXapoCom), 1);

            let wfx = float_wfx(2, 48_000);
            let mut sv: *mut SourceVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_source_voice(
                    xa2,
                    &mut sv,
                    &wfx,
                    0,
                    2.0,
                    0,
                    std::ptr::null(),
                    std::ptr::null()
                ),
                S_OK
            );

            // 480 stereo frames: L = constant +0.5, R = 10 full cycles of sine amp 0.8.
            let frames = 480usize;
            let mut stereo = Vec::with_capacity(frames * 2);
            for i in 0..frames {
                let phase = TAU * 10.0 * (i as f32) / (frames as f32);
                stereo.push(0.5);
                stereo.push(0.8 * phase.sin());
            }
            submit_f32_buf(sv, &stereo);
            assert_eq!(sv_start(sv, 0, 0), S_OK);

            let mut out = vec![0.0f32; frames * 2];
            let _ = (&*xa2).core.mix_quantum(frames, &mut out);

            // Volume meter must pass audio through unmodified.
            assert!((out[0] - 0.5).abs() < 1e-5);
            assert!((out[25] - stereo[25]).abs() < 1e-5);

            // Query both peak and RMS.
            let mut peaks = [0.0f32; 2];
            let mut rms_vals = [0.0f32; 2];
            let mut levels = XAudio2FxVolumeMeterLevels {
                peak_levels: peaks.as_mut_ptr(),
                rms_levels: rms_vals.as_mut_ptr(),
                channel_count: 2,
            };
            assert_eq!(
                voice_get_effect_parameters(
                    mv as *mut VoiceHeader,
                    0,
                    &mut levels as *mut XAudio2FxVolumeMeterLevels as *mut c_void,
                    std::mem::size_of::<XAudio2FxVolumeMeterLevels>() as u32,
                ),
                S_OK
            );
            assert!((peaks[0] - 0.5).abs() < 1e-4, "L peak={}", peaks[0]);
            assert!((rms_vals[0] - 0.5).abs() < 1e-4, "L rms={}", rms_vals[0]);
            assert!((peaks[1] - 0.8).abs() < 1e-2, "R peak={}", peaks[1]);
            assert!(
                (rms_vals[1] - (0.8 * FRAC_1_SQRT_2)).abs() < 1e-2,
                "R rms={}",
                rms_vals[1]
            );

            // Peak-only query (`rms_levels = null`) must succeed.
            let mut peaks_only = [0.0f32; 2];
            let mut lv_peak_only = XAudio2FxVolumeMeterLevels {
                peak_levels: peaks_only.as_mut_ptr(),
                rms_levels: std::ptr::null_mut(),
                channel_count: 2,
            };
            assert_eq!(
                voice_get_effect_parameters(
                    mv as *mut VoiceHeader,
                    0,
                    &mut lv_peak_only as *mut XAudio2FxVolumeMeterLevels as *mut c_void,
                    std::mem::size_of::<XAudio2FxVolumeMeterLevels>() as u32,
                ),
                S_OK
            );
            assert!((peaks_only[0] - 0.5).abs() < 1e-4);

            // Next silent quantum resets reported levels to 0.0.
            let _ = (&*xa2).core.mix_quantum(frames, &mut out);
            assert_eq!(
                voice_get_effect_parameters(
                    mv as *mut VoiceHeader,
                    0,
                    &mut levels as *mut XAudio2FxVolumeMeterLevels as *mut c_void,
                    std::mem::size_of::<XAudio2FxVolumeMeterLevels>() as u32,
                ),
                S_OK
            );
            assert_eq!(peaks, [0.0, 0.0]);
            assert_eq!(rms_vals, [0.0, 0.0]);

            voice_destroy_voice(sv as *mut VoiceHeader);
            voice_destroy_voice(mv as *mut VoiceHeader);
            assert_eq!(xa2_release(xa2), 0);
        }
    }

    #[test]
    fn fxeq_four_band_parametric_biquad_dsp() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let (xa2, mv) = make_stopped_test_engine(1, 48_000);
            let mut eq_ptr = 0usize;
            assert_eq!(
                CreateFX(&CLSID_FX_EQ as *const GUID as *const u8, &mut eq_ptr, 0, 0),
                S_OK
            );
            let desc = XAudio2EffectDescriptor {
                effect: eq_ptr as *mut c_void,
                initial_state: 1,
                output_channels: 1,
            };
            let chain = XAudio2EffectChain {
                effect_count: 1,
                effects: &desc,
            };
            let wfx = float_wfx(1, 48_000);
            let mut sv: *mut SourceVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_source_voice(xa2, &mut sv, &wfx, 0, 2.0, 0, std::ptr::null(), &chain),
                S_OK
            );
            assert_eq!(xapo_release(eq_ptr as *mut BuiltinXapoCom), 1);

            // Invalid out-of-range gain must fail.
            let bad_eq = FxEqParameters {
                gain0: 99.0,
                ..FxEqParameters::default()
            };
            assert_eq!(
                voice_set_effect_parameters(
                    sv as *mut VoiceHeader,
                    0,
                    &bad_eq as *const FxEqParameters as *const c_void,
                    std::mem::size_of::<FxEqParameters>() as u32,
                    0
                ),
                E_INVALIDARG
            );

            // Boost 2000 Hz by 4x (+12 dB) and cut 100 Hz by 0.25x (-12 dB).
            let eq = FxEqParameters {
                frequency_center0: 100.0,
                gain0: 0.25,
                bandwidth0: 0.5,
                frequency_center2: 2000.0,
                gain2: 4.0,
                bandwidth2: 0.5,
                ..FxEqParameters::default()
            };
            assert_eq!(
                voice_set_effect_parameters(
                    sv as *mut VoiceHeader,
                    0,
                    &eq as *const FxEqParameters as *const c_void,
                    std::mem::size_of::<FxEqParameters>() as u32,
                    0
                ),
                S_OK
            );
            let mut got_eq = FxEqParameters::default();
            assert_eq!(
                voice_get_effect_parameters(
                    sv as *mut VoiceHeader,
                    0,
                    &mut got_eq as *mut FxEqParameters as *mut c_void,
                    std::mem::size_of::<FxEqParameters>() as u32,
                ),
                S_OK
            );
            assert!((got_eq.gain2 - 4.0).abs() < 1e-6);

            let n = 4800usize;
            let sine_2k: Vec<f32> = (0..n)
                .map(|i| 0.2 * (TAU * 2000.0 * (i as f32) / 48_000.0).sin())
                .collect();
            let sine_100: Vec<f32> = (0..n)
                .map(|i| 0.2 * (TAU * 100.0 * (i as f32) / 48_000.0).sin())
                .collect();

            let in_2k_rms = rms(&sine_2k[n / 2..]);
            let in_100_rms = rms(&sine_100[n / 2..]);

            submit_f32_buf(sv, &sine_2k);
            assert_eq!(sv_start(sv, 0, 0), S_OK);
            let mut out_2k = vec![0.0f32; n];
            let _ = (&*xa2).core.mix_quantum(n, &mut out_2k);
            let gain_2k = rms(&out_2k[n / 2..]) / in_2k_rms;
            assert!(
                (gain_2k - 4.0).abs() < 0.35,
                "expected ~4x boost at 2000 Hz, got {gain_2k}"
            );

            submit_f32_buf(sv, &sine_100);
            let mut out_100 = vec![0.0f32; n];
            let _ = (&*xa2).core.mix_quantum(n, &mut out_100);
            let gain_100 = rms(&out_100[n / 2..]) / in_100_rms;
            assert!(
                (gain_100 - 0.25).abs() < 0.08,
                "expected ~0.25x cut at 100 Hz, got {gain_100}"
            );

            voice_destroy_voice(sv as *mut VoiceHeader);
            voice_destroy_voice(mv as *mut VoiceHeader);
            assert_eq!(xa2_release(xa2), 0);
        }
    }

    #[test]
    fn fx_mastering_limiter_peak_envelope_and_release() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let (xa2, mv) = make_stopped_test_engine(2, 48_000);
            let mut lim_ptr = 0usize;
            assert_eq!(
                CreateFX(
                    &CLSID_FX_MASTERING_LIMITER as *const GUID as *const u8,
                    &mut lim_ptr,
                    0,
                    0
                ),
                S_OK
            );
            let desc = XAudio2EffectDescriptor {
                effect: lim_ptr as *mut c_void,
                initial_state: 1,
                output_channels: 2,
            };
            let chain = XAudio2EffectChain {
                effect_count: 1,
                effects: &desc,
            };
            assert_eq!(voice_set_effect_chain(mv as *mut VoiceHeader, &chain), S_OK);
            assert_eq!(xapo_release(lim_ptr as *mut BuiltinXapoCom), 1);

            let wfx = float_wfx(2, 48_000);
            let mut sv: *mut SourceVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_source_voice(
                    xa2,
                    &mut sv,
                    &wfx,
                    0,
                    2.0,
                    0,
                    std::ptr::null(),
                    std::ptr::null()
                ),
                S_OK
            );
            assert_eq!(sv_start(sv, 0, 0), S_OK);

            // Hot signal (amplitude 3.0) with default Loudness = 1000 -> brickwall ceiling <= 1.0.
            let hot = vec![3.0f32; 256 * 2];
            submit_f32_buf(sv, &hot);
            let mut out = vec![0.0f32; 256 * 2];
            let _ = (&*xa2).core.mix_quantum(256, &mut out);
            let max_hot = out.iter().map(|v| v.abs()).fold(0.0f32, f32::max);
            assert!(
                max_hot <= 1.001 && max_hot >= 0.99,
                "limiter ceiling expected ~1.0, got {max_hot}"
            );

            // Lower Loudness to 500 -> ceiling drops to ~0.5.
            let p500 = FxMasteringLimiterParameters {
                release: 1,
                loudness: 500,
            };
            assert_eq!(
                voice_set_effect_parameters(
                    mv as *mut VoiceHeader,
                    0,
                    &p500 as *const FxMasteringLimiterParameters as *const c_void,
                    std::mem::size_of::<FxMasteringLimiterParameters>() as u32,
                    0
                ),
                S_OK
            );
            let med = vec![0.9f32; 512 * 2];
            submit_f32_buf(sv, &med);
            let mut out500 = vec![0.0f32; 512 * 2];
            let _ = (&*xa2).core.mix_quantum(512, &mut out500);
            let max_500 = out500.iter().map(|v| v.abs()).fold(0.0f32, f32::max);
            assert!(
                max_500 <= 0.501 && max_500 >= 0.49,
                "limiter ceiling at Loudness=500 expected ~0.5, got {max_500}"
            );

            // Release speed test: after a 4.0 transient, a 0.2 signal recovers faster with Release=1ms than 20ms.
            let measure_recovery = |release_ms: u32| -> f32 {
                let p = FxMasteringLimiterParameters {
                    release: release_ms,
                    loudness: 1000,
                };
                assert_eq!(
                    voice_set_effect_parameters(
                        mv as *mut VoiceHeader,
                        0,
                        &p as *const FxMasteringLimiterParameters as *const c_void,
                        std::mem::size_of::<FxMasteringLimiterParameters>() as u32,
                        0
                    ),
                    S_OK
                );
                // Drain silence first so envelope starts clean.
                let mut sil = vec![0.0f32; 4800 * 2];
                let _ = (&*xa2).core.mix_quantum(4800, &mut sil);
                // 10 frames of 4.0 followed by 240 frames (5 ms) of 0.2.
                let mut sig = vec![0.2f32; 250 * 2];
                for i in 0..20 {
                    sig[i] = 4.0;
                }
                submit_f32_buf(sv, &sig);
                let mut res = vec![0.0f32; 250 * 2];
                let _ = (&*xa2).core.mix_quantum(250, &mut res);
                res[249 * 2]
            };
            let fast_rec = measure_recovery(1);
            let slow_rec = measure_recovery(20);
            assert!(
                fast_rec > slow_rec + 0.03,
                "Release=1ms ({fast_rec}) should recover faster than Release=20ms ({slow_rec})"
            );

            voice_destroy_voice(sv as *mut VoiceHeader);
            voice_destroy_voice(mv as *mut VoiceHeader);
            assert_eq!(xa2_release(xa2), 0);
        }
    }

    #[test]
    fn fx_echo_delay_and_feedback_dsp() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let (xa2, mv) = make_stopped_test_engine(1, 48_000);
            let init = FxEchoInitData { max_delay: 200.0 };
            let mut echo_ptr = 0usize;
            assert_eq!(
                CreateFX(
                    &CLSID_FX_ECHO as *const GUID as *const u8,
                    &mut echo_ptr,
                    &init as *const FxEchoInitData as usize,
                    std::mem::size_of::<FxEchoInitData>() as u32
                ),
                S_OK
            );
            let desc = XAudio2EffectDescriptor {
                effect: echo_ptr as *mut c_void,
                initial_state: 1,
                output_channels: 1,
            };
            let chain = XAudio2EffectChain {
                effect_count: 1,
                effects: &desc,
            };
            let wfx = float_wfx(1, 48_000);
            let mut sv: *mut SourceVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_source_voice(xa2, &mut sv, &wfx, 0, 2.0, 0, std::ptr::null(), &chain),
                S_OK
            );
            assert_eq!(xapo_release(echo_ptr as *mut BuiltinXapoCom), 1);

            // Delay > MaxDelay (200 ms) must be rejected.
            let too_long = FxEchoParameters {
                wet_dry_mix: 0.5,
                feedback: 0.5,
                delay: 300.0,
            };
            assert_eq!(
                voice_set_effect_parameters(
                    sv as *mut VoiceHeader,
                    0,
                    &too_long as *const FxEchoParameters as *const c_void,
                    std::mem::size_of::<FxEchoParameters>() as u32,
                    0
                ),
                E_INVALIDARG
            );

            // 10 ms = exactly 480 samples at 48 kHz.
            let echo_p = FxEchoParameters {
                wet_dry_mix: 0.5,
                feedback: 0.5,
                delay: 10.0,
            };
            assert_eq!(
                voice_set_effect_parameters(
                    sv as *mut VoiceHeader,
                    0,
                    &echo_p as *const FxEchoParameters as *const c_void,
                    std::mem::size_of::<FxEchoParameters>() as u32,
                    0
                ),
                S_OK
            );

            // Submit a single-frame impulse and mix across multiple quantums (including empty queue tail).
            submit_f32_buf(sv, &[1.0f32]);
            assert_eq!(sv_start(sv, 0, 0), S_OK);

            let mut all = Vec::with_capacity(2000);
            for _ in 0..10 {
                let mut q = vec![0.0f32; 200];
                let _ = (&*xa2).core.mix_quantum(200, &mut q);
                all.extend_from_slice(&q);
            }
            assert!((all[0] - 0.5).abs() < 1e-4, "dry impulse={}", all[0]);
            assert!(all[100].abs() < 1e-6, "between echoes={}", all[100]);
            assert!((all[480] - 0.5).abs() < 1e-4, "echo 1={}", all[480]);
            assert!((all[960] - 0.25).abs() < 1e-4, "echo 2={}", all[960]);
            assert!((all[1440] - 0.125).abs() < 1e-4, "echo 3={}", all[1440]);
            assert!((all[1920] - 0.0625).abs() < 1e-4, "echo 4={}", all[1920]);

            voice_destroy_voice(sv as *mut VoiceHeader);
            voice_destroy_voice(mv as *mut VoiceHeader);
            assert_eq!(xa2_release(xa2), 0);
        }
    }

    #[test]
    fn live_effect_chain_ordering_toggling_and_guest_com_xapo() {
        // Custom guest COM object implementing IXAPO + IXAPOParameters with real Win64 vtables.
        #[repr(C)]
        struct MockGuestXapo {
            #[allow(dead_code)]
            xapo_vtbl: *const [crate::Ptr; 13],
            #[allow(dead_code)]
            params_vtbl: *const [crate::Ptr; 5],
            refs: AtomicU32,
            locked_calls: AtomicU32,
            unlocked_calls: Arc<AtomicU32>,
            process_calls: AtomicU32,
            last_enabled: AtomicU32,
            gain_bits: AtomicU32,
            destroyed: Arc<AtomicBool>,
        }

        unsafe fn from_params(p: *mut c_void) -> *mut MockGuestXapo {
            (p as *mut u8).sub(std::mem::offset_of!(MockGuestXapo, params_vtbl))
                as *mut MockGuestXapo
        }

        unsafe extern "win64" fn g_qi(
            this: *mut MockGuestXapo,
            riid: *const GUID,
            ppv: *mut *mut c_void,
        ) -> i32 {
            std::ptr::write_unaligned(ppv, std::ptr::null_mut());
            let iid = std::ptr::read_unaligned(riid);
            let obj = &*this;
            if iid == IID_IUnknown || iid == IID_IXAPO {
                obj.refs.fetch_add(1, Ordering::AcqRel);
                std::ptr::write_unaligned(ppv, this as *mut c_void);
                S_OK
            } else if iid == IID_IXAPO_PARAMETERS {
                obj.refs.fetch_add(1, Ordering::AcqRel);
                let p = (this as *mut u8)
                    .add(std::mem::offset_of!(MockGuestXapo, params_vtbl))
                    as *mut c_void;
                std::ptr::write_unaligned(ppv, p);
                S_OK
            } else {
                E_NOINTERFACE
            }
        }
        unsafe extern "win64" fn g_add_ref(this: *mut MockGuestXapo) -> u32 {
            let obj = &*this;
            obj.refs.fetch_add(1, Ordering::AcqRel) + 1
        }
        unsafe extern "win64" fn g_release(this: *mut MockGuestXapo) -> u32 {
            let prev = {
                let obj = &*this;
                obj.refs.fetch_sub(1, Ordering::AcqRel)
            };
            if prev == 1 {
                let boxed = Box::from_raw(this);
                boxed.destroyed.store(true, Ordering::Release);
                0
            } else {
                prev - 1
            }
        }
        unsafe extern "win64" fn g_get_reg(
            _this: *mut MockGuestXapo,
            _pp: *mut *mut XapoRegistrationProperties,
        ) -> i32 {
            E_NOINTERFACE
        }
        unsafe extern "win64" fn g_fmt_ok(
            _this: *mut MockGuestXapo,
            _a: *const WaveFormatEx,
            _b: *const WaveFormatEx,
            _c: *mut *mut WaveFormatEx,
        ) -> i32 {
            S_OK
        }
        unsafe extern "win64" fn g_init(
            _this: *mut MockGuestXapo,
            _d: *const c_void,
            _s: u32,
        ) -> i32 {
            S_OK
        }
        unsafe extern "win64" fn g_reset(_this: *mut MockGuestXapo) {}
        unsafe extern "win64" fn g_lock(
            this: *mut MockGuestXapo,
            _ic: u32,
            _ip: *const XapoLockForProcessBufferParameters,
            _oc: u32,
            _op: *const XapoLockForProcessBufferParameters,
        ) -> i32 {
            let obj = &*this;
            obj.locked_calls.fetch_add(1, Ordering::AcqRel);
            S_OK
        }
        unsafe extern "win64" fn g_unlock(this: *mut MockGuestXapo) {
            let obj = &*this;
            obj.unlocked_calls.fetch_add(1, Ordering::AcqRel);
        }
        unsafe extern "win64" fn g_process(
            this: *mut MockGuestXapo,
            _ic: u32,
            p_in: *const XapoProcessBufferParameters,
            _oc: u32,
            p_out: *mut XapoProcessBufferParameters,
            is_enabled: i32,
        ) {
            let obj = &*this;
            obj.process_calls.fetch_add(1, Ordering::AcqRel);
            obj.last_enabled.store(is_enabled as u32, Ordering::Release);
            let in_p = std::ptr::read_unaligned(p_in);
            let mut out_p = std::ptr::read_unaligned(p_out);
            // Verify 16-byte alignment required by XAPO spec.
            assert_eq!((in_p.buffer as usize) % 16, 0);
            assert_eq!((out_p.buffer as usize) % 16, 0);
            let frames = in_p.valid_frame_count as usize;
            if in_p.buffer_flags == XAPO_BUFFER_SILENT {
                out_p.buffer_flags = XAPO_BUFFER_SILENT;
                out_p.valid_frame_count = in_p.valid_frame_count;
                std::ptr::write_unaligned(p_out, out_p);
                return;
            }
            let gain = if is_enabled != 0 {
                f32::from_bits(obj.gain_bits.load(Ordering::Acquire))
            } else {
                1.0
            };
            let src = std::slice::from_raw_parts(in_p.buffer as *const f32, frames);
            let dst = std::slice::from_raw_parts_mut(out_p.buffer as *mut f32, frames);
            for i in 0..frames {
                dst[i] = src[i] * gain;
            }
            out_p.buffer_flags = XAPO_BUFFER_VALID;
            out_p.valid_frame_count = in_p.valid_frame_count;
            std::ptr::write_unaligned(p_out, out_p);
        }
        unsafe extern "win64" fn g_frames(_this: *mut MockGuestXapo, n: u32) -> u32 {
            n
        }
        unsafe extern "win64" fn gp_qi(
            p: *mut c_void,
            riid: *const GUID,
            ppv: *mut *mut c_void,
        ) -> i32 {
            g_qi(from_params(p), riid, ppv)
        }
        unsafe extern "win64" fn gp_add_ref(p: *mut c_void) -> u32 {
            g_add_ref(from_params(p))
        }
        unsafe extern "win64" fn gp_release(p: *mut c_void) -> u32 {
            g_release(from_params(p))
        }
        unsafe extern "win64" fn gp_set_params(
            p: *mut c_void,
            data: *const c_void,
            size: u32,
        ) {
            if !data.is_null() && size >= 4 {
                let g = std::ptr::read_unaligned(data as *const f32);
                let obj = &*from_params(p);
                obj.gain_bits.store(g.to_bits(), Ordering::Release);
            }
        }
        unsafe extern "win64" fn gp_get_params(
            p: *mut c_void,
            data: *mut c_void,
            size: u32,
        ) {
            if !data.is_null() && size >= 4 {
                let obj = &*from_params(p);
                let g = f32::from_bits(obj.gain_bits.load(Ordering::Acquire));
                std::ptr::write_unaligned(data as *mut f32, g);
            }
        }

        static GUEST_XAPO_VTBL: Vtable<13> = Vtable([
            crate::Ptr(g_qi as *const ()),
            crate::Ptr(g_add_ref as *const ()),
            crate::Ptr(g_release as *const ()),
            crate::Ptr(g_get_reg as *const ()),
            crate::Ptr(g_fmt_ok as *const ()),
            crate::Ptr(g_fmt_ok as *const ()),
            crate::Ptr(g_init as *const ()),
            crate::Ptr(g_reset as *const ()),
            crate::Ptr(g_lock as *const ()),
            crate::Ptr(g_unlock as *const ()),
            crate::Ptr(g_process as *const ()),
            crate::Ptr(g_frames as *const ()),
            crate::Ptr(g_frames as *const ()),
        ]);
        static GUEST_PARAMS_VTBL: Vtable<5> = Vtable([
            crate::Ptr(gp_qi as *const ()),
            crate::Ptr(gp_add_ref as *const ()),
            crate::Ptr(gp_release as *const ()),
            crate::Ptr(gp_set_params as *const ()),
            crate::Ptr(gp_get_params as *const ()),
        ]);

        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let (xa2, mv) = make_stopped_test_engine(1, 48_000);
            let wfx = float_wfx(1, 48_000);
            let mut sv: *mut SourceVoice = std::ptr::null_mut();
            assert_eq!(
                xa2_create_source_voice(
                    xa2,
                    &mut sv,
                    &wfx,
                    0,
                    2.0,
                    0,
                    std::ptr::null(),
                    std::ptr::null()
                ),
                S_OK
            );

            // Chain: [0] Guest gain XAPO (3.0x) -> [1] Builtin FXMasteringLimiter (ceiling 1.0) -> [2] Builtin VolumeMeter.
            let destroyed = Arc::new(AtomicBool::new(false));
            let unlocked_calls = Arc::new(AtomicU32::new(0));
            let guest = Box::into_raw(Box::new(MockGuestXapo {
                xapo_vtbl: &GUEST_XAPO_VTBL.0,
                params_vtbl: &GUEST_PARAMS_VTBL.0,
                refs: AtomicU32::new(1),
                locked_calls: AtomicU32::new(0),
                unlocked_calls: unlocked_calls.clone(),
                process_calls: AtomicU32::new(0),
                last_enabled: AtomicU32::new(99),
                gain_bits: AtomicU32::new(3.0f32.to_bits()),
                destroyed: destroyed.clone(),
            }));

            let mut lim_ptr = 0usize;
            assert_eq!(
                CreateFX(
                    &CLSID_FX_MASTERING_LIMITER as *const GUID as *const u8,
                    &mut lim_ptr,
                    0,
                    0
                ),
                S_OK
            );
            let mut vm_ptr = 0usize;
            assert_eq!(CreateAudioVolumeMeter(&mut vm_ptr), S_OK);

            let descs = [
                XAudio2EffectDescriptor {
                    effect: guest as *mut c_void,
                    initial_state: 1,
                    output_channels: 1,
                },
                XAudio2EffectDescriptor {
                    effect: lim_ptr as *mut c_void,
                    initial_state: 1,
                    output_channels: 1,
                },
                XAudio2EffectDescriptor {
                    effect: vm_ptr as *mut c_void,
                    initial_state: 1,
                    output_channels: 1,
                },
            ];
            let chain = XAudio2EffectChain {
                effect_count: 3,
                effects: descs.as_ptr(),
            };
            assert_eq!(voice_set_effect_chain(sv as *mut VoiceHeader, &chain), S_OK);
            let guest_ref = &*guest;
            assert_eq!(guest_ref.locked_calls.load(Ordering::Acquire), 1);

            // Caller releases its own refs; voice keeps all three alive.
            assert_eq!(g_release(guest), 2); // IXAPO + IXAPOParameters refs held by voice
            assert_eq!(xapo_release(lim_ptr as *mut BuiltinXapoCom), 1);
            assert_eq!(xapo_release(vm_ptr as *mut BuiltinXapoCom), 1);

            // Update guest gain parameter to 4.0 via IXAudio2Voice::SetEffectParameters & read back.
            let new_gain = 4.0f32;
            assert_eq!(
                voice_set_effect_parameters(
                    sv as *mut VoiceHeader,
                    0,
                    &new_gain as *const f32 as *const c_void,
                    4,
                    0
                ),
                S_OK
            );
            let mut read_gain = 0.0f32;
            assert_eq!(
                voice_get_effect_parameters(
                    sv as *mut VoiceHeader,
                    0,
                    &mut read_gain as *mut f32 as *mut c_void,
                    4
                ),
                S_OK
            );
            assert_eq!(read_gain, 4.0);

            // Check initial enabled states.
            let mut en = 0i32;
            voice_get_effect_state(sv as *mut VoiceHeader, 0, &mut en);
            assert_eq!(en, 1);
            voice_get_effect_state(sv as *mut VoiceHeader, 1, &mut en);
            assert_eq!(en, 1);

            // Input 0.5 -> Guest (4.0x = 2.0) -> Limiter (clamped to 1.0) -> VolumeMeter (measures 1.0).
            assert_eq!(sv_start(sv, 0, 0), S_OK);
            submit_f32_buf(sv, &[0.5f32; 32]);
            let mut out = [0.0f32; 32];
            let _ = (&*xa2).core.mix_quantum(32, &mut out);
            assert!((out[16] - 1.0).abs() < 1e-4, "chained limited output={}", out[16]);
            assert_eq!(guest_ref.last_enabled.load(Ordering::Acquire), 1);

            let mut peak = [0.0f32; 1];
            let mut lv = XAudio2FxVolumeMeterLevels {
                peak_levels: peak.as_mut_ptr(),
                rms_levels: std::ptr::null_mut(),
                channel_count: 1,
            };
            assert_eq!(
                voice_get_effect_parameters(
                    sv as *mut VoiceHeader,
                    2,
                    &mut lv as *mut XAudio2FxVolumeMeterLevels as *mut c_void,
                    std::mem::size_of::<XAudio2FxVolumeMeterLevels>() as u32
                ),
                S_OK
            );
            assert!((peak[0] - 1.0).abs() < 1e-4, "meter after limiter={}", peak[0]);

            // Disable limiter (slot 1): now 0.5 -> Guest (4.0x = 2.0) -> bypass -> meter reads 2.0!
            assert_eq!(voice_disable_effect(sv as *mut VoiceHeader, 1, 0), S_OK);
            voice_get_effect_state(sv as *mut VoiceHeader, 1, &mut en);
            assert_eq!(en, 0);
            submit_f32_buf(sv, &[0.5f32; 32]);
            let _ = (&*xa2).core.mix_quantum(32, &mut out);
            assert!((out[16] - 2.0).abs() < 1e-4, "limiter disabled output={}", out[16]);

            // Disable guest XAPO (slot 0) as well: passes 0.5 straight through with IsEnabled=0.
            assert_eq!(voice_disable_effect(sv as *mut VoiceHeader, 0, 0), S_OK);
            submit_f32_buf(sv, &[0.5f32; 32]);
            let _ = (&*xa2).core.mix_quantum(32, &mut out);
            assert!((out[16] - 0.5).abs() < 1e-4, "guest disabled output={}", out[16]);
            assert_eq!(guest_ref.last_enabled.load(Ordering::Acquire), 0);

            // Re-enable both (slot 0 & slot 1): output returns to limited 1.0!
            assert_eq!(voice_enable_effect(sv as *mut VoiceHeader, 0, 0), S_OK);
            assert_eq!(voice_enable_effect(sv as *mut VoiceHeader, 1, 0), S_OK);
            submit_f32_buf(sv, &[0.5f32; 32]);
            let _ = (&*xa2).core.mix_quantum(32, &mut out);
            assert!((out[16] - 1.0).abs() < 1e-4, "re-enabled output={}", out[16]);

            // Out-of-bounds effect index returns XAUDIO2_E_INVALID_CALL.
            assert_eq!(
                voice_enable_effect(sv as *mut VoiceHeader, 9, 0),
                XAUDIO2_E_INVALID_CALL
            );

            // Clearing the effect chain unlocks & releases guest COM object cleanly.
            assert_eq!(guest_ref.process_calls.load(Ordering::Acquire), 4);
            assert!(!destroyed.load(Ordering::Acquire));
            assert_eq!(voice_set_effect_chain(sv as *mut VoiceHeader, std::ptr::null()), S_OK);
            assert_eq!(unlocked_calls.load(Ordering::Acquire), 1);
            assert!(destroyed.load(Ordering::Acquire));

            voice_destroy_voice(sv as *mut VoiceHeader);
            voice_destroy_voice(mv as *mut VoiceHeader);
            assert_eq!(xa2_release(xa2), 0);
        }
    }
}
