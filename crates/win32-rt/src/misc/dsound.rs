//! `dsound.dll`: DirectSound 8 & DirectSoundCapture 8 runtime backed by
//! `crate::audio::backend`'s CoreAudio / Null output stream.
//!
//! * Device enumeration (`DirectSoundEnumerateA/W`, `DirectSoundCaptureEnumerateA/W`,
//!   `GetDeviceID`) backed by `crate::audio::backend::output_devices()` and
//!   `input_devices()`, invoking guest enumeration callbacks via `crate::call_guest8`.
//! * Real Win64 COM objects with vtables registered in `VA_DISPATCH` via
//!   `crate::register_native_va`:
//!   - `IDirectSound` / `IDirectSound8`
//!   - `IDirectSoundBuffer` / `IDirectSoundBuffer8` (circular PCM buffer supporting
//!     PCM 8/16/24/32-bit, IEEE float 32/64-bit, and `WAVE_FORMAT_EXTENSIBLE`,
//!     wrap-around `Lock`/`Unlock`, one-shot & looping `Play`/`Stop`, live play/write
//!     cursor tracking, linear-interpolated resampling `SetFrequency`, dB `SetVolume`,
//!     stereo `SetPan`, `DuplicateSoundBuffer` sharing sample memory)
//!   - `IDirectSound3DListener` & `IDirectSound3DBuffer` via `QueryInterface`
//!     (inverse-distance rolloff, `DSBCAPS_MUTE3DATMAXDISTANCE`, sound cones,
//!     left-handed stereo panning, Doppler shift, immediate & deferred settings)
//!   - `IDirectSoundNotify` via `QueryInterface` (offset & stop event signaling)
//!   - `IDirectSoundCapture` / `IDirectSoundCapture8` & `IDirectSoundCaptureBuffer8`
//!   - `IDirectSoundFullDuplex` & `DirectSoundFullDuplexCreate`
//!   - `IClassFactory` via `DllGetClassObject` (`CLSID_DirectSound*`,
//!     `CLSID_DirectSoundCapture*`, `CLSID_DirectSoundFullDuplex`)
//! * Background stereo `f32` software mixer writing into `OutputStream` with
//!   automatic `Backend::Null` fallback when headless (`PORT_HEADLESS=1` /
//!   `AW2_HEADLESS=1`) or `AW2_AUDIO_BACKEND=null`, plus deterministic
//!   `mix_dsound_frames` helper for unit tests.

use crate::audio::backend::{self, Backend, DeviceInfo, OutputStream, StreamControl, StreamShared};
use crate::audio::com::{
    GUID, IID_IUnknown, KSDATAFORMAT_SUBTYPE_IEEE_FLOAT, KSDATAFORMAT_SUBTYPE_PCM,
};
use crate::audio::format::{
    self, WAVE_FORMAT_EXTENSIBLE, WAVE_FORMAT_IEEE_FLOAT, WAVE_FORMAT_PCM,
};
use crate::audio::ring::Ring;
use crate::types::*;
use std::collections::HashMap;
use std::f32::consts::FRAC_PI_4;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicU8, Ordering};
use std::sync::{Arc, Condvar, Mutex, Once, Weak};
use std::time::{Duration, Instant};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[
    (1, "DirectSoundCreate"),
    (2, "DirectSoundEnumerateA"),
    (3, "DirectSoundEnumerateW"),
    (4, "DllCanUnloadNow"),
    (5, "DllGetClassObject"),
    (6, "DirectSoundCaptureCreate"),
    (7, "DirectSoundCaptureEnumerateA"),
    (8, "DirectSoundCaptureEnumerateW"),
    (9, "GetDeviceID"),
    (10, "DirectSoundFullDuplexCreate"),
    (11, "DirectSoundCreate8"),
    (12, "DirectSoundCaptureCreate8"),
];

// ============================================================================
// HRESULTs, GUIDs & DirectSound Constants
// ============================================================================

const DS_OK: HRESULT = S_OK;
const DSERR_CONTROLUNAVAIL: HRESULT = 0x8878_001Eu32 as HRESULT;
const DSERR_INVALIDPARAM: HRESULT = E_INVALIDARG;
const DSERR_INVALIDCALL: HRESULT = 0x8878_0032u32 as HRESULT;
const DSERR_PRIOLEVELNEEDED: HRESULT = 0x8878_0046u32 as HRESULT;
const DSERR_BADFORMAT: HRESULT = 0x8878_0064u32 as HRESULT;
const DSERR_NODRIVER: HRESULT = 0x8878_0078u32 as HRESULT;
const DSERR_ALREADYINITIALIZED: HRESULT = 0x8878_0082u32 as HRESULT;
const DSERR_NOAGGREGATION: HRESULT = 0x8004_0110u32 as HRESULT;
const DSERR_UNINITIALIZED: HRESULT = 0x8878_00AAu32 as HRESULT;
const DSERR_FXUNAVAILABLE: HRESULT = 0x8878_00DCu32 as HRESULT;
const DSERR_OBJECTNOTFOUND: HRESULT = 0x8878_1161u32 as HRESULT;
const CLASS_E_CLASSNOTAVAILABLE: HRESULT = 0x8004_0111u32 as HRESULT;

const GUID_NULL: GUID = GUID::new(0, 0, 0, [0; 8]);
const IID_ICLASSFACTORY: GUID =
    GUID::new(0x0000_0001, 0x0000, 0x0000, [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46]);

const CLSID_DIRECTSOUND: GUID =
    GUID::new(0x47D4_D946, 0x62E8, 0x11CF, [0x93, 0xBC, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00]);
const CLSID_DIRECTSOUND8: GUID =
    GUID::new(0x3901_CC3F, 0x84B5, 0x4FA4, [0xBA, 0x35, 0xAA, 0x81, 0x72, 0xB8, 0xA0, 0x9B]);
const CLSID_DIRECTSOUNDCAPTURE: GUID =
    GUID::new(0xB021_0780, 0x89CD, 0x11D0, [0xAF, 0x08, 0x00, 0xA0, 0xC9, 0x25, 0xCD, 0x16]);
const CLSID_DIRECTSOUNDCAPTURE8: GUID =
    GUID::new(0xE4BC_AC13, 0x7F99, 0x4908, [0x9A, 0x8E, 0x74, 0xE3, 0xBF, 0x24, 0xB6, 0xE1]);
const CLSID_DIRECTSOUNDFULLDUPLEX: GUID =
    GUID::new(0xFEA4_300C, 0x7959, 0x4147, [0xB2, 0x6A, 0x23, 0x77, 0xB9, 0xE7, 0xA9, 0x1D]);

const DSDEVID_DEFAULTPLAYBACK: GUID =
    GUID::new(0xDEF0_0000, 0x9C6D, 0x47ED, [0xAA, 0xF1, 0x4D, 0xDA, 0x8F, 0x2B, 0x5C, 0x03]);
const DSDEVID_DEFAULTCAPTURE: GUID =
    GUID::new(0xDEF0_0001, 0x9C6D, 0x47ED, [0xAA, 0xF1, 0x4D, 0xDA, 0x8F, 0x2B, 0x5C, 0x03]);
const DSDEVID_DEFAULTVOICEPLAYBACK: GUID =
    GUID::new(0xDEF0_0002, 0x9C6D, 0x47ED, [0xAA, 0xF1, 0x4D, 0xDA, 0x8F, 0x2B, 0x5C, 0x03]);
const DSDEVID_DEFAULTVOICECAPTURE: GUID =
    GUID::new(0xDEF0_0003, 0x9C6D, 0x47ED, [0xAA, 0xF1, 0x4D, 0xDA, 0x8F, 0x2B, 0x5C, 0x03]);

const IID_IDIRECTSOUND: GUID =
    GUID::new(0x279A_FA83, 0x4981, 0x11CE, [0xA5, 0x21, 0x00, 0x20, 0xAF, 0x0B, 0xE5, 0x60]);
const IID_IDIRECTSOUND8: GUID =
    GUID::new(0xC50A_7E93, 0xF395, 0x4834, [0x9E, 0xF6, 0x7F, 0xA9, 0x9D, 0xE5, 0x09, 0x66]);
const IID_IDIRECTSOUNDBUFFER: GUID =
    GUID::new(0x279A_FA85, 0x4981, 0x11CE, [0xA5, 0x21, 0x00, 0x20, 0xAF, 0x0B, 0xE5, 0x60]);
const IID_IDIRECTSOUNDBUFFER8: GUID =
    GUID::new(0x6825_A449, 0x7524, 0x4D82, [0x92, 0x0F, 0x50, 0xE3, 0x6A, 0xB3, 0xAB, 0x1E]);
const IID_IDIRECTSOUND3DLISTENER: GUID =
    GUID::new(0x279A_FA84, 0x4981, 0x11CE, [0xA5, 0x21, 0x00, 0x20, 0xAF, 0x0B, 0xE5, 0x60]);
const IID_IDIRECTSOUND3DBUFFER: GUID =
    GUID::new(0x279A_FA86, 0x4981, 0x11CE, [0xA5, 0x21, 0x00, 0x20, 0xAF, 0x0B, 0xE5, 0x60]);
const IID_IDIRECTSOUNDCAPTURE: GUID =
    GUID::new(0xB021_0781, 0x89CD, 0x11D0, [0xAF, 0x08, 0x00, 0xA0, 0xC9, 0x25, 0xCD, 0x16]);
const IID_IDIRECTSOUNDCAPTURE8: GUID = IID_IDIRECTSOUNDCAPTURE;
const IID_IDIRECTSOUNDCAPTUREBUFFER: GUID =
    GUID::new(0xB021_0782, 0x89CD, 0x11D0, [0xAF, 0x08, 0x00, 0xA0, 0xC9, 0x25, 0xCD, 0x16]);
const IID_IDIRECTSOUNDCAPTUREBUFFER8: GUID =
    GUID::new(0x0099_0DF4, 0x0DBB, 0x4872, [0x83, 0x3E, 0x6D, 0x30, 0x3E, 0x80, 0xAE, 0xB6]);
const IID_IDIRECTSOUNDNOTIFY: GUID =
    GUID::new(0xB021_0783, 0x89CD, 0x11D0, [0xAF, 0x08, 0x00, 0xA0, 0xC9, 0x25, 0xCD, 0x16]);
const IID_IDIRECTSOUNDFULLDUPLEX: GUID =
    GUID::new(0xEDCB_4C7A, 0xDAAB, 0x4216, [0xA4, 0x2E, 0x6C, 0x50, 0x59, 0x6D, 0xAD, 0xC1]);

// Cooperative levels
const DSSCL_NORMAL: u32 = 1;
const DSSCL_PRIORITY: u32 = 2;
const DSSCL_WRITEPRIMARY: u32 = 4;

// Speaker configuration defaults
const DSSPEAKER_STEREO: u32 = 4;
const DSSPEAKER_GEOMETRY_WIDE: u32 = 0x14;
const DEFAULT_SPEAKER_CONFIG: u32 = DSSPEAKER_STEREO | (DSSPEAKER_GEOMETRY_WIDE << 16);

// DSCAPS flags
const DSCAPS_PRIMARYMONO: u32 = 0x0000_0001;
const DSCAPS_PRIMARYSTEREO: u32 = 0x0000_0002;
const DSCAPS_PRIMARY8BIT: u32 = 0x0000_0004;
const DSCAPS_PRIMARY16BIT: u32 = 0x0000_0008;
const DSCAPS_CONTINUOUSRATE: u32 = 0x0000_0010;
const DSCAPS_CERTIFIED: u32 = 0x0000_0040;
const DSCAPS_SECONDARYMONO: u32 = 0x0000_0100;
const DSCAPS_SECONDARYSTEREO: u32 = 0x0000_0200;
const DSCAPS_SECONDARY8BIT: u32 = 0x0000_0400;
const DSCAPS_SECONDARY16BIT: u32 = 0x0000_0800;
const DS_CERTIFIED: u32 = 0;

// DSBCAPS flags
const DSBCAPS_PRIMARYBUFFER: u32 = 0x0000_0001;
const DSBCAPS_CTRL3D: u32 = 0x0000_0010;
const DSBCAPS_CTRLFREQUENCY: u32 = 0x0000_0020;
const DSBCAPS_CTRLPAN: u32 = 0x0000_0040;
const DSBCAPS_CTRLVOLUME: u32 = 0x0000_0080;
const DSBCAPS_CTRLPOSITIONNOTIFY: u32 = 0x0000_0100;
const DSBCAPS_MUTE3DATMAXDISTANCE: u32 = 0x0002_0000;

// DSBPLAY / DSBSTATUS / DSBLOCK flags
const DSBPLAY_LOOPING: u32 = 0x0000_0001;
const DSBSTATUS_PLAYING: u32 = 0x0000_0001;
const DSBSTATUS_LOOPING: u32 = 0x0000_0004;
const DSBLOCK_FROMWRITECURSOR: u32 = 0x0000_0001;
const DSBLOCK_ENTIREBUFFER: u32 = 0x0000_0002;

// Buffer parameter bounds
const DSBFREQUENCY_ORIGINAL: u32 = 0;
const DSBFREQUENCY_MIN: u32 = 100;
const DSBFREQUENCY_MAX: u32 = 200_000;
const DSBPAN_LEFT: i32 = -10_000;
const DSBPAN_CENTER: i32 = 0;
const DSBPAN_RIGHT: i32 = 10_000;
const DSBVOLUME_MIN: i32 = -10_000;
const DSBVOLUME_MAX: i32 = 0;
const DSBSIZE_MIN: u32 = 4;
const DSBSIZE_MAX: u32 = 0x0FFF_FFFF;
const DSBPN_OFFSETSTOP: u32 = 0xFFFF_FFFF;
const DSFXR_FAILED: u32 = 4;

// 3D constants
const DS3DMODE_NORMAL: u32 = 0;
const DS3DMODE_HEADRELATIVE: u32 = 1;
const DS3DMODE_DISABLE: u32 = 2;
const DS3D_IMMEDIATE: u32 = 0;
const DS3D_DEFERRED: u32 = 1;
const DS3D_MINROLLOFFFACTOR: f32 = 0.0;
const DS3D_MAXROLLOFFFACTOR: f32 = 10.0;
const DS3D_MINDOPPLERFACTOR: f32 = 0.0;
const DS3D_MAXDOPPLERFACTOR: f32 = 10.0;
const DS3D_DEFAULTMINDISTANCE: f32 = 1.0;
const DS3D_DEFAULTMAXDISTANCE: f32 = 1_000_000_000.0;
const DS3D_MAXCONEANGLE: u32 = 360;
const SPEED_OF_SOUND: f32 = 343.5;

// Capture constants
const DSCCAPS_MULTIPLECAPTURE: u32 = 0x0000_0001;
const DSCCAPS_CERTIFIED: u32 = 0x0000_0040;
const DSCBSTART_LOOPING: u32 = 0x0000_0001;
const DSCBLOCK_ENTIREBUFFER: u32 = 0x0000_0001;
const DSCBSTATUS_CAPTURING: u32 = 0x0000_0001;
const DSCBSTATUS_LOOPING: u32 = 0x0000_0002;
const STANDARD_CAPTURE_FORMATS: u32 = 0x000F_FFFF;

// Global DLL lifetime tracking for DllCanUnloadNow
static ACTIVE_OBJECTS: AtomicU32 = AtomicU32::new(0);
static SERVER_LOCKS: AtomicU32 = AtomicU32::new(0);

// ============================================================================
// Windows ABI Structures (Win64 natural alignment; WAVEFORMAT* packed(2))
// ============================================================================

#[repr(C, packed(2))]
#[derive(Clone, Copy)]
struct PcmWaveFormat {
    format_tag: u16,
    channels: u16,
    samples_per_sec: u32,
    avg_bytes_per_sec: u32,
    block_align: u16,
    bits_per_sample: u16,
}

#[repr(C, packed(2))]
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

#[repr(C, packed(2))]
#[derive(Clone, Copy)]
struct WaveFormatExtensible {
    format: WaveFormatEx,
    valid_bits_per_sample: u16,
    channel_mask: u32,
    sub_format: GUID,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct DsCaps {
    dw_size: u32,
    dw_flags: u32,
    dw_min_secondary_sample_rate: u32,
    dw_max_secondary_sample_rate: u32,
    dw_primary_buffers: u32,
    dw_max_hw_mixing_all_buffers: u32,
    dw_max_hw_mixing_static_buffers: u32,
    dw_max_hw_mixing_streaming_buffers: u32,
    dw_free_hw_mixing_all_buffers: u32,
    dw_free_hw_mixing_static_buffers: u32,
    dw_free_hw_mixing_streaming_buffers: u32,
    dw_max_hw_3d_all_buffers: u32,
    dw_max_hw_3d_static_buffers: u32,
    dw_max_hw_3d_streaming_buffers: u32,
    dw_free_hw_3d_all_buffers: u32,
    dw_free_hw_3d_static_buffers: u32,
    dw_free_hw_3d_streaming_buffers: u32,
    dw_total_hw_mem_bytes: u32,
    dw_free_hw_mem_bytes: u32,
    dw_max_contig_free_hw_mem_bytes: u32,
    dw_unlock_transfer_rate_hw_buffers: u32,
    dw_play_cpu_overhead_sw_buffers: u32,
    dw_reserved1: u32,
    dw_reserved2: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct DsbCaps {
    dw_size: u32,
    dw_flags: u32,
    dw_buffer_bytes: u32,
    dw_unlock_transfer_rate: u32,
    dw_play_cpu_overhead: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct DsBufferDesc1 {
    dw_size: u32,
    dw_flags: u32,
    dw_buffer_bytes: u32,
    dw_reserved: u32,
    lpwfx_format: *const u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct DsBufferDesc {
    dw_size: u32,
    dw_flags: u32,
    dw_buffer_bytes: u32,
    dw_reserved: u32,
    lpwfx_format: *const u8,
    guid_3d_algorithm: GUID,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct DscCaps {
    dw_size: u32,
    dw_flags: u32,
    dw_formats: u32,
    dw_channels: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct DscbCaps {
    dw_size: u32,
    dw_flags: u32,
    dw_buffer_bytes: u32,
    dw_reserved: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct DscBufferDesc1 {
    dw_size: u32,
    dw_flags: u32,
    dw_buffer_bytes: u32,
    dw_reserved: u32,
    lpwfx_format: *const u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct DscBufferDesc {
    dw_size: u32,
    dw_flags: u32,
    dw_buffer_bytes: u32,
    dw_reserved: u32,
    lpwfx_format: *const u8,
    dw_fx_count: u32,
    lp_dsc_fx_desc: *const c_void,
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq)]
struct D3dVector {
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Ds3dBuffer {
    dw_size: u32,
    v_position: D3dVector,
    v_velocity: D3dVector,
    dw_inside_cone_angle: u32,
    dw_outside_cone_angle: u32,
    v_cone_orientation: D3dVector,
    l_cone_outside_volume: i32,
    fl_min_distance: f32,
    fl_max_distance: f32,
    dw_mode: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Ds3dListener {
    dw_size: u32,
    v_position: D3dVector,
    v_velocity: D3dVector,
    v_orient_front: D3dVector,
    v_orient_top: D3dVector,
    fl_distance_factor: f32,
    fl_rolloff_factor: f32,
    fl_doppler_factor: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct DsbPositionNotify {
    dw_offset: u32,
    h_event_notify: HANDLE,
}

const _: () = assert!(std::mem::size_of::<PcmWaveFormat>() == 16);
const _: () = assert!(std::mem::size_of::<WaveFormatEx>() == 18);
const _: () = assert!(std::mem::size_of::<WaveFormatExtensible>() == 40);
const _: () = assert!(std::mem::size_of::<DsCaps>() == 96);
const _: () = assert!(std::mem::size_of::<DsbCaps>() == 20);
const _: () = assert!(std::mem::size_of::<DsBufferDesc1>() == 24);
const _: () = assert!(std::mem::size_of::<DsBufferDesc>() == 40);
const _: () = assert!(std::mem::size_of::<DscCaps>() == 16);
const _: () = assert!(std::mem::size_of::<DscbCaps>() == 16);
const _: () = assert!(std::mem::size_of::<DscBufferDesc1>() == 24);
const _: () = assert!(std::mem::size_of::<DscBufferDesc>() == 40);
const _: () = assert!(std::mem::size_of::<D3dVector>() == 12);
const _: () = assert!(std::mem::size_of::<Ds3dBuffer>() == 64);
const _: () = assert!(std::mem::size_of::<Ds3dListener>() == 64);
const _: () = assert!(std::mem::size_of::<DsbPositionNotify>() == 16);

// ============================================================================
// PCM Format Parsing & Sample Storage
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PcmKind {
    U8,
    I16,
    I24,
    I32,
    F32,
    F64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct DsFormat {
    sample_rate: u32,
    channels: u16,
    bits_per_sample: u16,
    valid_bits_per_sample: u16,
    block_align: u16,
    channel_mask: u32,
    kind: PcmKind,
    extensible: bool,
}

impl DsFormat {
    fn default_primary() -> Self {
        Self {
            sample_rate: 48_000,
            channels: 2,
            bits_per_sample: 16,
            valid_bits_per_sample: 16,
            block_align: 4,
            channel_mask: 0x3,
            kind: PcmKind::I16,
            extensible: false,
        }
    }

    unsafe fn parse(ptr: *const u8) -> Result<Self, HRESULT> {
        if ptr.is_null() {
            return Err(DSERR_INVALIDPARAM);
        }
        let pcm = std::ptr::read_unaligned(ptr as *const PcmWaveFormat);
        let raw_tag = pcm.format_tag;
        let channels = pcm.channels;
        let sample_rate = pcm.samples_per_sec;
        let bits = pcm.bits_per_sample;
        let block_align = pcm.block_align;

        if channels == 0 || channels > 32 {
            return Err(DSERR_BADFORMAT);
        }
        if !(DSBFREQUENCY_MIN..=768_000).contains(&sample_rate) {
            return Err(DSERR_BADFORMAT);
        }

        let (tag, valid_bits, channel_mask, extensible) = if raw_tag == WAVE_FORMAT_EXTENSIBLE {
            let wfx = std::ptr::read_unaligned(ptr as *const WaveFormatEx);
            if wfx.cb_size < 22 {
                return Err(DSERR_BADFORMAT);
            }
            let ext = std::ptr::read_unaligned(ptr as *const WaveFormatExtensible);
            let sub = { ext.sub_format };
            let sub_tag = if sub == KSDATAFORMAT_SUBTYPE_PCM {
                WAVE_FORMAT_PCM
            } else if sub == KSDATAFORMAT_SUBTYPE_IEEE_FLOAT {
                WAVE_FORMAT_IEEE_FLOAT
            } else {
                return Err(DSERR_BADFORMAT);
            };
            let vb = if ext.valid_bits_per_sample == 0 {
                bits
            } else {
                ext.valid_bits_per_sample
            };
            if vb > bits {
                return Err(DSERR_BADFORMAT);
            }
            (sub_tag, vb, ext.channel_mask, true)
        } else {
            (raw_tag, bits, format::default_mask(channels), false)
        };

        let kind = match (tag, bits) {
            (WAVE_FORMAT_PCM, 8) => PcmKind::U8,
            (WAVE_FORMAT_PCM, 16) => PcmKind::I16,
            (WAVE_FORMAT_PCM, 24) => PcmKind::I24,
            (WAVE_FORMAT_PCM, 32) => PcmKind::I32,
            (WAVE_FORMAT_IEEE_FLOAT, 32) => PcmKind::F32,
            (WAVE_FORMAT_IEEE_FLOAT, 64) => PcmKind::F64,
            _ => return Err(DSERR_BADFORMAT),
        };

        let expected_align = (channels as u32) * (bits as u32 / 8);
        if expected_align == 0 || expected_align > u16::MAX as u32 {
            return Err(DSERR_BADFORMAT);
        }
        let align = if block_align == 0 {
            expected_align as u16
        } else if block_align as u32 != expected_align {
            return Err(DSERR_BADFORMAT);
        } else {
            block_align
        };

        Ok(Self {
            sample_rate,
            channels,
            bits_per_sample: bits,
            valid_bits_per_sample: valid_bits,
            block_align: align,
            channel_mask,
            kind,
            extensible,
        })
    }

    fn encoded_size(&self) -> u32 {
        if self.extensible {
            std::mem::size_of::<WaveFormatExtensible>() as u32
        } else {
            std::mem::size_of::<WaveFormatEx>() as u32
        }
    }

    unsafe fn write_to(
        &self,
        out: *mut u8,
        allocated: u32,
        written_out: *mut u32,
    ) -> HRESULT {
        let req = self.encoded_size();
        if out.is_null() {
            if written_out.is_null() {
                return DSERR_INVALIDPARAM;
            }
            std::ptr::write_unaligned(written_out, req);
            return DS_OK;
        }
        let tag = match self.kind {
            PcmKind::F32 | PcmKind::F64 => WAVE_FORMAT_IEEE_FLOAT,
            _ => WAVE_FORMAT_PCM,
        };
        if !self.extensible && tag == WAVE_FORMAT_PCM && allocated == 16 {
            let pcm = PcmWaveFormat {
                format_tag: WAVE_FORMAT_PCM,
                channels: self.channels,
                samples_per_sec: self.sample_rate,
                avg_bytes_per_sec: self.sample_rate.saturating_mul(self.block_align as u32),
                block_align: self.block_align,
                bits_per_sample: self.bits_per_sample,
            };
            std::ptr::write_unaligned(out as *mut PcmWaveFormat, pcm);
            if !written_out.is_null() {
                std::ptr::write_unaligned(written_out, 16);
            }
            return DS_OK;
        }
        if allocated < req {
            return DSERR_INVALIDPARAM;
        }
        if self.extensible {
            let sub = if tag == WAVE_FORMAT_IEEE_FLOAT {
                KSDATAFORMAT_SUBTYPE_IEEE_FLOAT
            } else {
                KSDATAFORMAT_SUBTYPE_PCM
            };
            let ext = WaveFormatExtensible {
                format: WaveFormatEx {
                    format_tag: WAVE_FORMAT_EXTENSIBLE,
                    channels: self.channels,
                    samples_per_sec: self.sample_rate,
                    avg_bytes_per_sec: self.sample_rate.saturating_mul(self.block_align as u32),
                    block_align: self.block_align,
                    bits_per_sample: self.bits_per_sample,
                    cb_size: 22,
                },
                valid_bits_per_sample: self.valid_bits_per_sample,
                channel_mask: self.channel_mask,
                sub_format: sub,
            };
            std::ptr::write_unaligned(out as *mut WaveFormatExtensible, ext);
        } else {
            let wfx = WaveFormatEx {
                format_tag: tag,
                channels: self.channels,
                samples_per_sec: self.sample_rate,
                avg_bytes_per_sec: self.sample_rate.saturating_mul(self.block_align as u32),
                block_align: self.block_align,
                bits_per_sample: self.bits_per_sample,
                cb_size: 0,
            };
            std::ptr::write_unaligned(out as *mut WaveFormatEx, wfx);
        }
        if !written_out.is_null() {
            std::ptr::write_unaligned(written_out, req);
        }
        DS_OK
    }
}

/// Fixed-address circular PCM storage shared across duplicated buffers.
///
/// Using `Box<[AtomicU8]>` guarantees pointer stability across `Lock`/`Unlock`
/// and eliminates Rust data-race UB when the guest streams new PCM bytes into
/// a locked write region while the background mixer reads ahead of the play cursor.
struct SampleMemory {
    data: Box<[AtomicU8]>,
    format: DsFormat,
    total_frames: usize,
    /// Row-major `[in_ch][2]` matrix mapping `format.channels` -> stereo `(L, R)`.
    stereo_matrix: Vec<f32>,
}

impl SampleMemory {
    fn new(byte_len: usize, format: DsFormat) -> Arc<Self> {
        let silence_byte = if format.kind == PcmKind::U8 { 0x80u8 } else { 0x00u8 };
        let data: Box<[AtomicU8]> = (0..byte_len).map(|_| AtomicU8::new(silence_byte)).collect();
        let block = (format.block_align as usize).max(1);
        let total_frames = byte_len / block;
        let stereo_matrix = if format.channels > 2 {
            format::channel_matrix(format.channels, format.channel_mask, 2, 0x3)
        } else {
            Vec::new()
        };
        Arc::new(Self {
            data,
            format,
            total_frames,
            stereo_matrix,
        })
    }

    #[inline]
    fn byte_len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    fn base_mut_ptr(&self) -> *mut u8 {
        if self.data.is_empty() {
            std::ptr::null_mut()
        } else {
            self.data[0].as_ptr()
        }
    }

    #[inline]
    fn load_u8(&self, idx: usize) -> u8 {
        self.data[idx].load(Ordering::Relaxed)
    }

    #[inline]
    fn read_channel_f32(&self, frame_idx: usize, ch: usize) -> f32 {
        let bps = (self.format.bits_per_sample as usize) / 8;
        let off = frame_idx * (self.format.block_align as usize) + ch * bps;
        match self.format.kind {
            PcmKind::U8 => (self.load_u8(off) as f32 - 128.0) / 128.0,
            PcmKind::I16 => {
                let s = i16::from_le_bytes([self.load_u8(off), self.load_u8(off + 1)]);
                (s as f32) / 32768.0
            }
            PcmKind::I24 => {
                let s = i32::from_le_bytes([
                    0,
                    self.load_u8(off),
                    self.load_u8(off + 1),
                    self.load_u8(off + 2),
                ]) >> 8;
                (s as f32) / 8_388_608.0
            }
            PcmKind::I32 => {
                let s = i32::from_le_bytes([
                    self.load_u8(off),
                    self.load_u8(off + 1),
                    self.load_u8(off + 2),
                    self.load_u8(off + 3),
                ]);
                (s as f32) / 2_147_483_648.0
            }
            PcmKind::F32 => {
                let v = f32::from_le_bytes([
                    self.load_u8(off),
                    self.load_u8(off + 1),
                    self.load_u8(off + 2),
                    self.load_u8(off + 3),
                ]);
                if v.is_finite() { v } else { 0.0 }
            }
            PcmKind::F64 => {
                let v = f64::from_le_bytes([
                    self.load_u8(off),
                    self.load_u8(off + 1),
                    self.load_u8(off + 2),
                    self.load_u8(off + 3),
                    self.load_u8(off + 4),
                    self.load_u8(off + 5),
                    self.load_u8(off + 6),
                    self.load_u8(off + 7),
                ]) as f32;
                if v.is_finite() { v } else { 0.0 }
            }
        }
    }

    /// Read one frame as stereo `(L, R)` `f32`.
    #[inline]
    fn read_stereo_frame(&self, frame_idx: usize) -> (f32, f32) {
        match self.format.channels {
            0 => (0.0, 0.0),
            1 => {
                let s = self.read_channel_f32(frame_idx, 0);
                (s, s)
            }
            2 => (
                self.read_channel_f32(frame_idx, 0),
                self.read_channel_f32(frame_idx, 1),
            ),
            n => {
                let mut l = 0.0f32;
                let mut r = 0.0f32;
                for c in 0..(n as usize) {
                    let s = self.read_channel_f32(frame_idx, c);
                    l += s * self.stereo_matrix[c * 2];
                    r += s * self.stereo_matrix[c * 2 + 1];
                }
                (l, r)
            }
        }
    }
}

// ============================================================================
// Device GUID Derivation & Host Device Resolution
// ============================================================================

fn device_guid_for(uid: &str, capture: bool) -> GUID {
    // Deterministic 128-bit FNV-1a derivation so GUIDs are stable across calls
    // and never collide with GUID_NULL or DSDEVID_Default*.
    let tag: &[u8] = if capture { b"dsound-capture:" } else { b"dsound-render:" };
    let mut h1: u64 = 0xcbf2_9ce4_8422_2325;
    let mut h2: u64 = 0x8422_2325_cbf2_9ce4;
    for &b in tag.iter().chain(uid.as_bytes().iter()) {
        h1 ^= b as u64;
        h1 = h1.wrapping_mul(0x0000_0100_0000_01b3);
        h2 = h2.wrapping_mul(0x0000_0100_0000_01b3);
        h2 ^= (b as u64).wrapping_add(0x5A);
    }
    let b1 = h1.to_le_bytes();
    let b2 = h2.to_le_bytes();
    let data1 = u32::from_le_bytes([b1[0], b1[1], b1[2], b1[3]]);
    let data2 = u16::from_le_bytes([b1[4], b1[5]]);
    // Mark version 5 (0x5xxx) and RFC 4122 variant (0x80..0xBF) so we never clash with DSDEVID_*.
    let data3 = (u16::from_le_bytes([b1[6], b1[7]]) & 0x0FFF) | 0x5000;
    let mut data4 = b2;
    data4[0] = (data4[0] & 0x3F) | 0x80;
    GUID::new(data1, data2, data3, data4)
}

fn is_headless() -> bool {
    std::env::var("PORT_HEADLESS").is_ok_and(|v| v == "1")
        || std::env::var("AW2_HEADLESS").is_ok_and(|v| v == "1")
}

fn resolve_render_device(guid_opt: Option<GUID>) -> Result<DeviceInfo, HRESULT> {
    let devs = backend::output_devices();
    let default_dev = devs.first().cloned().unwrap_or_else(backend::null_device);
    match guid_opt {
        None => Ok(default_dev),
        Some(g)
            if g == GUID_NULL
                || g == DSDEVID_DEFAULTPLAYBACK
                || g == DSDEVID_DEFAULTVOICEPLAYBACK =>
        {
            Ok(default_dev)
        }
        Some(g) => devs
            .into_iter()
            .find(|d| device_guid_for(&d.uid, false) == g)
            .ok_or(DSERR_NODRIVER),
    }
}

fn resolve_capture_device(guid_opt: Option<GUID>) -> Result<DeviceInfo, HRESULT> {
    let devs = backend::input_devices();
    let default_dev = devs.first().cloned().unwrap_or_else(backend::null_capture_device);
    match guid_opt {
        None => Ok(default_dev),
        Some(g)
            if g == GUID_NULL
                || g == DSDEVID_DEFAULTCAPTURE
                || g == DSDEVID_DEFAULTVOICECAPTURE =>
        {
            Ok(default_dev)
        }
        Some(g) => devs
            .into_iter()
            .find(|d| device_guid_for(&d.uid, true) == g)
            .ok_or(DSERR_NODRIVER),
    }
}

// ============================================================================
// Audio DSP & 3D Math Helpers
// ============================================================================

#[inline]
fn ds_mb_to_gain(mb: i32) -> f32 {
    if mb <= DSBVOLUME_MIN {
        0.0
    } else if mb >= DSBVOLUME_MAX {
        1.0
    } else {
        10.0f32.powf((mb as f32) / 2000.0)
    }
}

#[inline]
fn ds_pan_gains(pan_mb: i32) -> (f32, f32) {
    let p = pan_mb.clamp(DSBPAN_LEFT, DSBPAN_RIGHT);
    if p < 0 {
        (1.0, ds_mb_to_gain(p))
    } else if p > 0 {
        (ds_mb_to_gain(-p), 1.0)
    } else {
        (1.0, 1.0)
    }
}

#[inline]
fn vec_sub(a: D3dVector, b: D3dVector) -> D3dVector {
    D3dVector { x: a.x - b.x, y: a.y - b.y, z: a.z - b.z }
}

#[inline]
fn vec_dot(a: D3dVector, b: D3dVector) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

#[inline]
fn vec_cross(a: D3dVector, b: D3dVector) -> D3dVector {
    D3dVector {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

#[inline]
fn vec_len(v: D3dVector) -> f32 {
    vec_dot(v, v).sqrt()
}

#[inline]
fn vec_normalize(v: D3dVector) -> Option<D3dVector> {
    let l = vec_len(v);
    if !l.is_finite() || l < 1e-6 {
        None
    } else {
        Some(D3dVector { x: v.x / l, y: v.y / l, z: v.z / l })
    }
}

#[inline]
fn vec_is_finite(v: D3dVector) -> bool {
    v.x.is_finite() && v.y.is_finite() && v.z.is_finite()
}

#[derive(Clone, Copy)]
struct Listener3dParams {
    position: D3dVector,
    velocity: D3dVector,
    orient_front: D3dVector,
    orient_top: D3dVector,
    distance_factor: f32,
    rolloff_factor: f32,
    doppler_factor: f32,
}

impl Default for Listener3dParams {
    fn default() -> Self {
        Self {
            position: D3dVector { x: 0.0, y: 0.0, z: 0.0 },
            velocity: D3dVector { x: 0.0, y: 0.0, z: 0.0 },
            orient_front: D3dVector { x: 0.0, y: 0.0, z: 1.0 },
            orient_top: D3dVector { x: 0.0, y: 1.0, z: 0.0 },
            distance_factor: 1.0,
            rolloff_factor: 1.0,
            doppler_factor: 1.0,
        }
    }
}

impl Listener3dParams {
    fn to_abi(self) -> Ds3dListener {
        Ds3dListener {
            dw_size: std::mem::size_of::<Ds3dListener>() as u32,
            v_position: self.position,
            v_velocity: self.velocity,
            v_orient_front: self.orient_front,
            v_orient_top: self.orient_top,
            fl_distance_factor: self.distance_factor,
            fl_rolloff_factor: self.rolloff_factor,
            fl_doppler_factor: self.doppler_factor,
        }
    }
}

#[derive(Clone, Copy)]
struct Buffer3dParams {
    position: D3dVector,
    velocity: D3dVector,
    inside_cone_angle: u32,
    outside_cone_angle: u32,
    cone_orientation: D3dVector,
    cone_outside_volume: i32,
    min_distance: f32,
    max_distance: f32,
    mode: u32,
}

impl Default for Buffer3dParams {
    fn default() -> Self {
        Self {
            position: D3dVector { x: 0.0, y: 0.0, z: 0.0 },
            velocity: D3dVector { x: 0.0, y: 0.0, z: 0.0 },
            inside_cone_angle: DS3D_MAXCONEANGLE,
            outside_cone_angle: DS3D_MAXCONEANGLE,
            cone_orientation: D3dVector { x: 0.0, y: 0.0, z: 1.0 },
            cone_outside_volume: DSBVOLUME_MAX,
            min_distance: DS3D_DEFAULTMINDISTANCE,
            max_distance: DS3D_DEFAULTMAXDISTANCE,
            mode: DS3DMODE_NORMAL,
        }
    }
}

impl Buffer3dParams {
    fn to_abi(self) -> Ds3dBuffer {
        Ds3dBuffer {
            dw_size: std::mem::size_of::<Ds3dBuffer>() as u32,
            v_position: self.position,
            v_velocity: self.velocity,
            dw_inside_cone_angle: self.inside_cone_angle,
            dw_outside_cone_angle: self.outside_cone_angle,
            v_cone_orientation: self.cone_orientation,
            l_cone_outside_volume: self.cone_outside_volume,
            fl_min_distance: self.min_distance,
            fl_max_distance: self.max_distance,
            dw_mode: self.mode,
        }
    }
}

/// Evaluate DirectSound 3D spatialization: returns `(gain_left, gain_right, doppler_ratio)`.
fn compute_3d_spatial(
    buf: &Buffer3dParams,
    buf_flags: u32,
    listener: &Listener3dParams,
) -> (f32, f32, f32) {
    if buf.mode == DS3DMODE_DISABLE {
        return (1.0, 1.0, 1.0);
    }

    let rel = if buf.mode == DS3DMODE_HEADRELATIVE {
        buf.position
    } else {
        vec_sub(buf.position, listener.position)
    };
    let dist = vec_len(rel);

    // 1. Inverse-distance rolloff attenuation.
    let dist_gain = if (buf_flags & DSBCAPS_MUTE3DATMAXDISTANCE) != 0 && dist >= buf.max_distance {
        0.0
    } else {
        let min_d = buf.min_distance.max(1e-6);
        let d = dist.clamp(min_d, buf.max_distance.max(min_d));
        let rolloff = listener.rolloff_factor.clamp(DS3D_MINROLLOFFFACTOR, DS3D_MAXROLLOFFFACTOR);
        if rolloff <= 0.0 || d <= min_d {
            1.0
        } else {
            let eff = min_d + rolloff * (d - min_d);
            (min_d / eff).clamp(0.0, 1.0)
        }
    };

    if dist_gain == 0.0 {
        return (0.0, 0.0, 1.0);
    }

    // 2. Sound cone attenuation.
    let cone_gain = if (buf.inside_cone_angle < 360 || buf.outside_cone_angle < 360) && dist > 1e-6 {
        let to_listener = D3dVector {
            x: -rel.x / dist,
            y: -rel.y / dist,
            z: -rel.z / dist,
        };
        if let Some(cone_dir) = vec_normalize(buf.cone_orientation) {
            let full_angle_deg =
                vec_dot(cone_dir, to_listener).clamp(-1.0, 1.0).acos().to_degrees() * 2.0;
            let inside = buf.inside_cone_angle as f32;
            let outside = (buf.outside_cone_angle.max(buf.inside_cone_angle)) as f32;
            if full_angle_deg <= inside {
                1.0
            } else if full_angle_deg >= outside || (outside - inside) < 1e-4 {
                ds_mb_to_gain(buf.cone_outside_volume)
            } else {
                let t = (full_angle_deg - inside) / (outside - inside);
                let atten_mb = (t * (buf.cone_outside_volume as f32)).round() as i32;
                ds_mb_to_gain(atten_mb)
            }
        } else {
            1.0
        }
    } else {
        1.0
    };

    // 3. Stereo panning in DirectSound's left-handed basis (+X right, +Y up, +Z forward).
    let (pan_l, pan_r) = if dist <= 1e-6 {
        (std::f32::consts::FRAC_1_SQRT_2, std::f32::consts::FRAC_1_SQRT_2)
    } else {
        let dir = D3dVector { x: rel.x / dist, y: rel.y / dist, z: rel.z / dist };
        let pan_x = if buf.mode == DS3DMODE_HEADRELATIVE {
            dir.x
        } else {
            let front = vec_normalize(listener.orient_front)
                .unwrap_or(D3dVector { x: 0.0, y: 0.0, z: 1.0 });
            let top = vec_normalize(listener.orient_top)
                .unwrap_or(D3dVector { x: 0.0, y: 1.0, z: 0.0 });
            let right = vec_normalize(vec_cross(top, front))
                .unwrap_or(D3dVector { x: 1.0, y: 0.0, z: 0.0 });
            vec_dot(dir, right)
        };
        let theta = (pan_x.clamp(-1.0, 1.0) + 1.0) * FRAC_PI_4;
        (theta.cos(), theta.sin())
    };

    // 4. Doppler pitch ratio.
    let doppler = if listener.doppler_factor > 0.0 && dist > 1e-6 {
        let dir = D3dVector { x: rel.x / dist, y: rel.y / dist, z: rel.z / dist };
        let df = listener.doppler_factor * listener.distance_factor;
        let v_l = if buf.mode == DS3DMODE_HEADRELATIVE {
            0.0
        } else {
            vec_dot(listener.velocity, dir) * df
        };
        let v_s = vec_dot(buf.velocity, dir) * df;
        let denom = (SPEED_OF_SOUND + v_s).max(SPEED_OF_SOUND * 0.125);
        let num = (SPEED_OF_SOUND + v_l).max(SPEED_OF_SOUND * 0.125);
        (num / denom).clamp(0.125, 8.0)
    } else {
        1.0
    };

    let gain = dist_gain * cone_gain;
    (gain * pan_l, gain * pan_r, doppler)
}

// ============================================================================
// Notification & Active Lock Helpers
// ============================================================================

#[derive(Clone)]
struct NotifyEntry {
    offset: u32,
    event: Arc<crate::handles::Event>,
}

#[derive(Clone, Copy)]
struct ActiveLock {
    ptr1: usize,
    len1: u32,
    ptr2: usize,
    len2: u32,
}

fn check_offset_crossed(prev: u32, next: u32, buf_len: u32, wrapped: bool, target: u32) -> bool {
    if target >= buf_len {
        return false;
    }
    if !wrapped {
        target >= prev && target < next
    } else {
        target >= prev || target < next
    }
}

fn signal_events(events: Vec<Arc<crate::handles::Event>>) {
    for ev in events {
        ev.set();
    }
}

// ============================================================================
// Playback Device Core & Software Mixer
// ============================================================================

struct SecondaryBufferState {
    flags: u32,
    sample_mem: Arc<SampleMemory>,
    frequency: u32,
    volume_mb: i32,
    pan_mb: i32,
    playing: bool,
    looping: bool,
    play_pos_frames: f64,
    d3d_current: Buffer3dParams,
    d3d_deferred: Buffer3dParams,
    locks: Vec<ActiveLock>,
    notifies: Vec<NotifyEntry>,
}

impl SecondaryBufferState {
    fn new(flags: u32, sample_mem: Arc<SampleMemory>) -> Self {
        let freq = sample_mem.format.sample_rate;
        Self {
            flags,
            sample_mem,
            frequency: freq,
            volume_mb: DSBVOLUME_MAX,
            pan_mb: DSBPAN_CENTER,
            playing: false,
            looping: false,
            play_pos_frames: 0.0,
            d3d_current: Buffer3dParams::default(),
            d3d_deferred: Buffer3dParams::default(),
            locks: Vec::new(),
            notifies: Vec::new(),
        }
    }

    fn current_cursors(&self) -> (u32, u32) {
        let total = self.sample_mem.total_frames;
        let block = self.sample_mem.format.block_align as usize;
        let buf_len = self.sample_mem.byte_len();
        if total == 0 || buf_len == 0 {
            return (0, 0);
        }
        let play_frame = (self.play_pos_frames.floor() as usize) % total;
        let play_bytes = (play_frame * block) as u32;
        if !self.playing {
            return (play_bytes, play_bytes);
        }
        // Safe write cursor leads play cursor by ~10 ms (clamped to half the buffer).
        let lead_frames = ((self.frequency as usize) / 100)
            .max(1)
            .min((total / 2).max(1));
        let write_frame = (play_frame + lead_frames) % total;
        let write_bytes = (write_frame * block) as u32;
        (play_bytes, write_bytes)
    }

    fn collect_stop_events(&self, out: &mut Vec<Arc<crate::handles::Event>>) {
        for n in &self.notifies {
            if n.offset == DSBPN_OFFSETSTOP {
                out.push(n.event.clone());
            }
        }
    }
}

struct DeviceMixState {
    device: DeviceInfo,
    cooperative_level: u32,
    speaker_config: u32,
    primary_format: DsFormat,
    primary_volume_mb: i32,
    primary_pan_mb: i32,
    primary_playing: bool,
    primary_frames_mixed: u64,
    listener_current: Listener3dParams,
    listener_deferred: Listener3dParams,
    buffers: HashMap<u64, SecondaryBufferState>,
}

impl DeviceMixState {
    fn new(device: DeviceInfo) -> Self {
        let mut primary_format = DsFormat::default_primary();
        if (DSBFREQUENCY_MIN..=DSBFREQUENCY_MAX).contains(&device.rate) {
            primary_format.sample_rate = device.rate;
        }
        Self {
            device,
            cooperative_level: DSSCL_NORMAL,
            speaker_config: DEFAULT_SPEAKER_CONFIG,
            primary_format,
            primary_volume_mb: DSBVOLUME_MAX,
            primary_pan_mb: DSBPAN_CENTER,
            primary_playing: false,
            primary_frames_mixed: 0,
            listener_current: Listener3dParams::default(),
            listener_deferred: Listener3dParams::default(),
            buffers: HashMap::new(),
        }
    }
}

/// Mix `frames` stereo `f32` frames from all playing DirectSound secondary buffers
/// into `out[..frames * 2]`, advancing play cursors and returning any
/// `IDirectSoundNotify` events triggered during this block.
fn mix_dsound_frames(
    st: &mut DeviceMixState,
    frames: usize,
    out: &mut [f32],
) -> Vec<Arc<crate::handles::Event>> {
    let mut events = Vec::new();
    let total_samples = frames.saturating_mul(2);
    if frames == 0 || out.len() < total_samples {
        return events;
    }
    out[..total_samples].fill(0.0);

    let mix_rate = st.primary_format.sample_rate.max(1) as f64;
    let listener = st.listener_current;
    let master_vol = ds_mb_to_gain(st.primary_volume_mb);
    let (master_pan_l, master_pan_r) = ds_pan_gains(st.primary_pan_mb);

    // Pair with the Release fence in Unlock() so unlocked guest writes are visible.
    std::sync::atomic::fence(Ordering::Acquire);

    for buf in st.buffers.values_mut() {
        if !buf.playing {
            continue;
        }
        let total_frames = buf.sample_mem.total_frames;
        let block_align = buf.sample_mem.format.block_align as u32;
        let buf_len = buf.sample_mem.byte_len() as u32;
        if total_frames == 0 || block_align == 0 || buf_len == 0 {
            buf.playing = false;
            buf.looping = false;
            buf.collect_stop_events(&mut events);
            continue;
        }

        let vol_gain = ds_mb_to_gain(buf.volume_mb);
        let is_3d_active =
            (buf.flags & DSBCAPS_CTRL3D) != 0 && buf.d3d_current.mode != DS3DMODE_DISABLE;
        let (gain_l, gain_r, doppler) = if is_3d_active {
            let (l3, r3, dop) = compute_3d_spatial(&buf.d3d_current, buf.flags, &listener);
            (vol_gain * l3, vol_gain * r3, dop)
        } else {
            let (pl, pr) = ds_pan_gains(buf.pan_mb);
            (vol_gain * pl, vol_gain * pr, 1.0)
        };

        let step = ((buf.frequency as f64) / mix_rate) * (doppler as f64);
        let total_frames_f = total_frames as f64;

        for f in 0..frames {
            if !buf.playing {
                break;
            }
            let pos = buf.play_pos_frames;
            let i0_raw = pos.floor() as usize;
            let i0 = i0_raw % total_frames;
            let frac = (pos - pos.floor()).clamp(0.0, 1.0) as f32;
            let i1 = if i0 + 1 < total_frames {
                i0 + 1
            } else if buf.looping {
                0
            } else {
                i0
            };

            let (l0, r0) = buf.sample_mem.read_stereo_frame(i0);
            let (l1, r1) = buf.sample_mem.read_stereo_frame(i1);
            let mut sl = l0 + (l1 - l0) * frac;
            let mut sr = r0 + (r1 - r0) * frac;
            if is_3d_active && buf.sample_mem.format.channels > 1 {
                let mono = 0.5 * (sl + sr);
                sl = mono;
                sr = mono;
            }

            out[f * 2] += sl * gain_l;
            out[f * 2 + 1] += sr * gain_r;

            let prev_byte = (i0 as u32).saturating_mul(block_align);
            let next_pos = pos + step;

            if next_pos >= total_frames_f {
                if buf.looping {
                    let wrapped_pos = next_pos.rem_euclid(total_frames_f);
                    let full_wrap = (next_pos - pos) >= total_frames_f;
                    let new_byte =
                        ((wrapped_pos.floor() as usize) % total_frames) as u32 * block_align;
                    for n in &buf.notifies {
                        if n.offset != DSBPN_OFFSETSTOP
                            && (full_wrap
                                || check_offset_crossed(prev_byte, new_byte, buf_len, true, n.offset))
                        {
                            events.push(n.event.clone());
                        }
                    }
                    buf.play_pos_frames = wrapped_pos;
                } else {
                    for n in &buf.notifies {
                        if n.offset != DSBPN_OFFSETSTOP
                            && check_offset_crossed(prev_byte, buf_len, buf_len, false, n.offset)
                        {
                            events.push(n.event.clone());
                        }
                    }
                    buf.play_pos_frames = 0.0;
                    buf.playing = false;
                    buf.looping = false;
                    buf.collect_stop_events(&mut events);
                }
            } else {
                let new_byte = ((next_pos.floor() as usize) % total_frames) as u32 * block_align;
                if new_byte != prev_byte {
                    for n in &buf.notifies {
                        if n.offset != DSBPN_OFFSETSTOP
                            && check_offset_crossed(prev_byte, new_byte, buf_len, false, n.offset)
                        {
                            events.push(n.event.clone());
                        }
                    }
                }
                buf.play_pos_frames = next_pos;
            }
        }
    }

    let ml = master_vol * master_pan_l;
    let mr = master_vol * master_pan_r;
    for f in 0..frames {
        let l = out[f * 2] * ml;
        let r = out[f * 2 + 1] * mr;
        out[f * 2] = if l.is_finite() { l } else { 0.0 };
        out[f * 2 + 1] = if r.is_finite() { r } else { 0.0 };
    }
    st.primary_frames_mixed = st.primary_frames_mixed.wrapping_add(frames as u64);

    events
}

struct MixerRuntime {
    sample_rate: u32,
    stop_flag: Arc<AtomicBool>,
    wake: Arc<(Mutex<()>, Condvar)>,
    control: StreamControl,
    mixer_thread: Option<std::thread::JoinHandle<()>>,
    stream: Option<OutputStream>,
}

impl MixerRuntime {
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

struct DsDeviceCore {
    initialized: AtomicBool,
    is_ds8: bool,
    mixer_paused: AtomicBool,
    next_buf_id: AtomicU64,
    state: Mutex<DeviceMixState>,
    runtime: Mutex<Option<MixerRuntime>>,
}

impl DsDeviceCore {
    fn new(device: DeviceInfo, initialized: bool, is_ds8: bool) -> Arc<Self> {
        Arc::new(Self {
            initialized: AtomicBool::new(initialized),
            is_ds8,
            mixer_paused: AtomicBool::new(false),
            next_buf_id: AtomicU64::new(1),
            state: Mutex::new(DeviceMixState::new(device)),
            runtime: Mutex::new(None),
        })
    }

    fn alloc_buf_id(&self) -> u64 {
        self.next_buf_id.fetch_add(1, Ordering::Relaxed)
    }

    fn mix_frames(&self, frames: usize, out: &mut [f32]) {
        let events = {
            let mut st = self.state.lock().unwrap();
            mix_dsound_frames(&mut st, frames, out)
        };
        signal_events(events);
    }

    fn ensure_mixer_started(self: &Arc<Self>) {
        if self.mixer_paused.load(Ordering::Acquire) {
            return;
        }
        let (dev, sample_rate) = {
            let st = self.state.lock().unwrap();
            (st.device.clone(), st.primary_format.sample_rate)
        };
        let mut rt_guard = self.runtime.lock().unwrap();
        if let Some(existing) = rt_guard.as_ref() {
            if existing.sample_rate == sample_rate {
                let _g = existing.wake.0.lock().unwrap();
                existing.wake.1.notify_all();
                return;
            }
            if let Some(old) = rt_guard.take() {
                drop(rt_guard);
                old.shutdown();
                rt_guard = self.runtime.lock().unwrap();
            }
        }
        if rt_guard.is_some() {
            return;
        }
        match spawn_dsound_mixer(self, &dev, sample_rate) {
            Ok(rt) => *rt_guard = Some(rt),
            Err(e) => {
                log::warn!(target: "audio", "dsound: failed to spawn mixer runtime: {e}");
            }
        }
    }

    fn shutdown_runtime(&self) {
        let rt = self.runtime.lock().unwrap().take();
        if let Some(r) = rt {
            r.shutdown();
        }
    }

    fn restart_mixer_if_running(self: &Arc<Self>, new_rate: u32) {
        let running = {
            let rt = self.runtime.lock().unwrap();
            rt.as_ref().is_some_and(|r| r.sample_rate != new_rate)
        };
        if running {
            self.shutdown_runtime();
            self.ensure_mixer_started();
        }
    }
}

impl Drop for DsDeviceCore {
    fn drop(&mut self) {
        self.shutdown_runtime();
    }
}

fn spawn_dsound_mixer(
    core: &Arc<DsDeviceCore>,
    requested_dev: &DeviceInfo,
    sample_rate: u32,
) -> Result<MixerRuntime, String> {
    let dev = if is_headless() || backend::backend() == Backend::Null {
        backend::null_device()
    } else {
        requested_dev.clone()
    };
    let channels: usize = 2;
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
                "dsound: CoreAudio open failed on '{}' ({e}); falling back to Null backend",
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
    if let Err(e) = control.start() {
        log::warn!(
            target: "audio",
            "dsound: CoreAudio start failed ({e}); falling back to Null backend"
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

    let stop_flag = Arc::new(AtomicBool::new(false));
    let wake = Arc::new((Mutex::new(()), Condvar::new()));
    let weak_core: Weak<DsDeviceCore> = Arc::downgrade(core);
    let stop_worker = stop_flag.clone();
    let wake_worker = wake.clone();
    let shared_worker = shared.clone();
    let target_buffered = (io_frames * 2 + quantum_frames)
        .min(ring_cap / 2)
        .max(quantum_frames * 2);

    let mixer_thread = std::thread::Builder::new()
        .name("dsound-mixer".into())
        .spawn(move || {
            let mut mix_buf = vec![0.0f32; quantum_frames * channels];
            let poll_dur = Duration::from_millis(4);
            while !stop_worker.load(Ordering::Acquire) {
                let Some(eng) = weak_core.upgrade() else {
                    break;
                };
                if !eng.mixer_paused.load(Ordering::Acquire) {
                    let mut quantums = 0usize;
                    while !stop_worker.load(Ordering::Acquire)
                        && !eng.mixer_paused.load(Ordering::Acquire)
                        && shared_worker.ring.len() + quantum_frames <= target_buffered
                        && quantums < 4
                    {
                        eng.mix_frames(quantum_frames, &mut mix_buf);
                        let pushed = shared_worker.ring.push(&mix_buf);
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
        "dsound: mixer started {} Hz stereo ({:?}, io={} frames, quantum={} frames)",
        sample_rate,
        stream.kind,
        stream.io_frames,
        quantum_frames
    );

    Ok(MixerRuntime {
        sample_rate,
        stop_flag,
        wake,
        control,
        mixer_thread: Some(mixer_thread),
        stream: Some(stream),
    })
}

// ============================================================================
// Capture Core & Buffer State
// ============================================================================

struct DsCaptureCore {
    initialized: AtomicBool,
    device: Mutex<DeviceInfo>,
}

impl DsCaptureCore {
    fn new(device: DeviceInfo, initialized: bool) -> Arc<Self> {
        Arc::new(Self {
            initialized: AtomicBool::new(initialized),
            device: Mutex::new(device),
        })
    }
}

struct CaptureBufferState {
    flags: u32,
    sample_mem: Arc<SampleMemory>,
    capturing: bool,
    looping: bool,
    base_frames: u64,
    start_instant: Option<Instant>,
    locks: Vec<ActiveLock>,
    notifies: Vec<NotifyEntry>,
}

impl CaptureBufferState {
    fn new(flags: u32, sample_mem: Arc<SampleMemory>) -> Self {
        Self {
            flags,
            sample_mem,
            capturing: false,
            looping: false,
            base_frames: 0,
            start_instant: None,
            locks: Vec::new(),
            notifies: Vec::new(),
        }
    }

    fn advance_and_cursors(&mut self) -> (u32, u32, Vec<Arc<crate::handles::Event>>) {
        let mut events = Vec::new();
        let total = self.sample_mem.total_frames;
        let block = self.sample_mem.format.block_align as u32;
        let buf_len = self.sample_mem.byte_len() as u32;
        if total == 0 || block == 0 || buf_len == 0 {
            return (0, 0, events);
        }
        let sr = self.sample_mem.format.sample_rate.max(1) as f64;
        if !self.capturing {
            let f = (self.base_frames as usize) % total;
            let b = (f as u32).saturating_mul(block);
            return (b, b, events);
        }

        let prev_total = self.base_frames;
        let elapsed_frames = self
            .start_instant
            .map(|t| (t.elapsed().as_secs_f64() * sr).floor() as u64)
            .unwrap_or(0);
        let cur_total = prev_total.saturating_add(elapsed_frames);

        if !self.looping && cur_total >= total as u64 {
            let prev_byte = ((prev_total as usize) % total) as u32 * block;
            for n in &self.notifies {
                if n.offset != DSBPN_OFFSETSTOP
                    && check_offset_crossed(prev_byte, buf_len, buf_len, false, n.offset)
                {
                    events.push(n.event.clone());
                } else if n.offset == DSBPN_OFFSETSTOP {
                    events.push(n.event.clone());
                }
            }
            self.base_frames = 0;
            self.capturing = false;
            self.looping = false;
            self.start_instant = None;
            return (0, 0, events);
        }

        if elapsed_frames > 0 {
            let prev_byte = ((prev_total as usize) % total) as u32 * block;
            let new_byte = ((cur_total as usize) % total) as u32 * block;
            let wrapped = (cur_total / total as u64) > (prev_total / total as u64);
            for n in &self.notifies {
                if n.offset != DSBPN_OFFSETSTOP
                    && check_offset_crossed(prev_byte, new_byte, buf_len, wrapped, n.offset)
                {
                    events.push(n.event.clone());
                }
            }
            self.base_frames = cur_total;
            self.start_instant = Some(Instant::now());
        }

        let cap_frame = (self.base_frames as usize) % total;
        let lead = ((self.sample_mem.format.sample_rate as usize) / 100)
            .min(total / 4);
        let read_frame = if self.base_frames >= lead as u64 {
            (cap_frame + total - lead) % total
        } else {
            0
        };
        (
            (cap_frame as u32).saturating_mul(block),
            (read_frame as u32).saturating_mul(block),
            events,
        )
    }
}

// ============================================================================
// COM Object Layouts & Vtables
// ============================================================================

#[repr(C, align(16))]
struct Vtable<const N: usize>([crate::Ptr; N]);
unsafe impl<const N: usize> Sync for Vtable<N> {}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FactoryKind {
    DirectSound,
    DirectSound8,
    DirectSoundCapture,
    DirectSoundCapture8,
    DirectSoundFullDuplex,
}

#[repr(C)]
struct ClassFactoryCom {
    vtbl: *const crate::Ptr,
    refs: AtomicU32,
    kind: FactoryKind,
}

#[repr(C)]
struct DsDeviceCom {
    vtbl: *const crate::Ptr,
    refs: AtomicU32,
    core: Arc<DsDeviceCore>,
}

/// Multi-interface COM object for `IDirectSoundBuffer8` + (`IDirectSound3DBuffer`
/// or `IDirectSound3DListener`) + `IDirectSoundNotify`.
#[repr(C)]
struct DsBufferCom {
    vtbl_buf: *const crate::Ptr,
    vtbl_3d: *const crate::Ptr,
    vtbl_notify: *const crate::Ptr,
    refs: AtomicU32,
    device: Arc<DsDeviceCore>,
    is_primary: bool,
    flags: u32,
    buffer_id: u64,
}

#[repr(C)]
struct DsCaptureCom {
    vtbl: *const crate::Ptr,
    refs: AtomicU32,
    core: Arc<DsCaptureCore>,
}

/// Multi-interface COM object for `IDirectSoundCaptureBuffer8` + `IDirectSoundNotify`.
#[repr(C)]
struct DsCaptureBufferCom {
    vtbl_cap_buf: *const crate::Ptr,
    vtbl_notify: *const crate::Ptr,
    refs: AtomicU32,
    capture: Arc<DsCaptureCore>,
    state: Mutex<CaptureBufferState>,
}

#[repr(C)]
struct DsFullDuplexCom {
    vtbl: *const crate::Ptr,
    refs: AtomicU32,
    initialized: AtomicBool,
    capture: Mutex<Option<Arc<DsCaptureCore>>>,
    render: Mutex<Option<Arc<DsDeviceCore>>>,
}

// --- Vtable instances ---

static CLASS_FACTORY_VTBL: Vtable<5> = Vtable([
    crate::Ptr(cf_query_interface as *const ()), // 0
    crate::Ptr(cf_add_ref as *const ()),         // 1
    crate::Ptr(cf_release as *const ()),         // 2
    crate::Ptr(cf_create_instance as *const ()), // 3
    crate::Ptr(cf_lock_server as *const ()),     // 4
]);

static DSOUND8_VTBL: Vtable<12> = Vtable([
    crate::Ptr(ds_query_interface as *const ()),      // 0
    crate::Ptr(ds_add_ref as *const ()),              // 1
    crate::Ptr(ds_release as *const ()),              // 2
    crate::Ptr(ds_create_sound_buffer as *const ()),  // 3
    crate::Ptr(ds_get_caps as *const ()),             // 4
    crate::Ptr(ds_duplicate_sound_buffer as *const ()), // 5
    crate::Ptr(ds_set_cooperative_level as *const ()), // 6
    crate::Ptr(ds_compact as *const ()),              // 7
    crate::Ptr(ds_get_speaker_config as *const ()),   // 8
    crate::Ptr(ds_set_speaker_config as *const ()),   // 9
    crate::Ptr(ds_initialize as *const ()),           // 10
    crate::Ptr(ds_verify_certification as *const ()), // 11 (IDirectSound8)
]);

static DSBUFFER8_VTBL: Vtable<24> = Vtable([
    crate::Ptr(dsb_query_interface as *const ()),      // 0
    crate::Ptr(dsb_add_ref as *const ()),              // 1
    crate::Ptr(dsb_release as *const ()),              // 2
    crate::Ptr(dsb_get_caps as *const ()),             // 3
    crate::Ptr(dsb_get_current_position as *const ()), // 4
    crate::Ptr(dsb_get_format as *const ()),           // 5
    crate::Ptr(dsb_get_volume as *const ()),           // 6
    crate::Ptr(dsb_get_pan as *const ()),              // 7
    crate::Ptr(dsb_get_frequency as *const ()),        // 8
    crate::Ptr(dsb_get_status as *const ()),           // 9
    crate::Ptr(dsb_initialize as *const ()),           // 10
    crate::Ptr(dsb_lock as *const ()),                 // 11
    crate::Ptr(dsb_play as *const ()),                 // 12
    crate::Ptr(dsb_set_current_position as *const ()), // 13
    crate::Ptr(dsb_set_format as *const ()),           // 14
    crate::Ptr(dsb_set_volume as *const ()),           // 15
    crate::Ptr(dsb_set_pan as *const ()),              // 16
    crate::Ptr(dsb_set_frequency as *const ()),        // 17
    crate::Ptr(dsb_stop as *const ()),                 // 18
    crate::Ptr(dsb_unlock as *const ()),               // 19
    crate::Ptr(dsb_restore as *const ()),              // 20
    crate::Ptr(dsb_set_fx as *const ()),               // 21 (IDirectSoundBuffer8)
    crate::Ptr(dsb_acquire_resources as *const ()),    // 22 (IDirectSoundBuffer8)
    crate::Ptr(dsb_get_object_in_path as *const ()),   // 23 (IDirectSoundBuffer8)
]);

static DS3DBUFFER_VTBL: Vtable<21> = Vtable([
    crate::Ptr(ds3db_query_interface as *const ()),                 // 0
    crate::Ptr(ds3db_add_ref as *const ()),                         // 1
    crate::Ptr(ds3db_release as *const ()),                         // 2
    crate::Ptr(ds3db_get_all_parameters as *const ()),              // 3
    crate::Ptr(ds3db_get_cone_angles as *const ()),                 // 4
    crate::Ptr(ds3db_get_cone_orientation as *const ()),            // 5
    crate::Ptr(ds3db_get_cone_outside_volume as *const ()),         // 6
    crate::Ptr(ds3db_get_max_distance as *const ()),                // 7
    crate::Ptr(ds3db_get_min_distance as *const ()),                // 8
    crate::Ptr(ds3db_get_mode as *const ()),                        // 9
    crate::Ptr(ds3db_get_position as *const ()),                    // 10
    crate::Ptr(ds3db_get_velocity as *const ()),                    // 11
    crate::Ptr(ds3db_set_all_parameters as *const ()),              // 12
    crate::Ptr(ds3db_set_cone_angles as *const ()),                 // 13
    crate::Ptr(crate::bridge_entry!(ds3db_set_cone_orientation)),   // 14
    crate::Ptr(ds3db_set_cone_outside_volume as *const ()),         // 15
    crate::Ptr(crate::bridge_entry!(ds3db_set_max_distance)),       // 16
    crate::Ptr(crate::bridge_entry!(ds3db_set_min_distance)),       // 17
    crate::Ptr(ds3db_set_mode as *const ()),                        // 18
    crate::Ptr(crate::bridge_entry!(ds3db_set_position)),           // 19
    crate::Ptr(crate::bridge_entry!(ds3db_set_velocity)),           // 20
]);

static DS3DLISTENER_VTBL: Vtable<18> = Vtable([
    crate::Ptr(ds3dl_query_interface as *const ()),                 // 0
    crate::Ptr(ds3dl_add_ref as *const ()),                         // 1
    crate::Ptr(ds3dl_release as *const ()),                         // 2
    crate::Ptr(ds3dl_get_all_parameters as *const ()),              // 3
    crate::Ptr(ds3dl_get_distance_factor as *const ()),             // 4
    crate::Ptr(ds3dl_get_doppler_factor as *const ()),              // 5
    crate::Ptr(ds3dl_get_orientation as *const ()),                 // 6
    crate::Ptr(ds3dl_get_position as *const ()),                    // 7
    crate::Ptr(ds3dl_get_rolloff_factor as *const ()),              // 8
    crate::Ptr(ds3dl_get_velocity as *const ()),                    // 9
    crate::Ptr(ds3dl_set_all_parameters as *const ()),              // 10
    crate::Ptr(crate::bridge_entry!(ds3dl_set_distance_factor)),    // 11
    crate::Ptr(crate::bridge_entry!(ds3dl_set_doppler_factor)),     // 12
    crate::Ptr(crate::bridge_entry!(ds3dl_set_orientation)),        // 13
    crate::Ptr(crate::bridge_entry!(ds3dl_set_position)),           // 14
    crate::Ptr(crate::bridge_entry!(ds3dl_set_rolloff_factor)),     // 15
    crate::Ptr(crate::bridge_entry!(ds3dl_set_velocity)),           // 16
    crate::Ptr(ds3dl_commit_deferred_settings as *const ()),        // 17
]);

static DSNOTIFY_VTBL: Vtable<4> = Vtable([
    crate::Ptr(dsn_query_interface as *const ()),            // 0
    crate::Ptr(dsn_add_ref as *const ()),                    // 1
    crate::Ptr(dsn_release as *const ()),                    // 2
    crate::Ptr(dsn_set_notification_positions as *const ()), // 3
]);

static DSCAPTURE8_VTBL: Vtable<6> = Vtable([
    crate::Ptr(dsc_query_interface as *const ()),       // 0
    crate::Ptr(dsc_add_ref as *const ()),               // 1
    crate::Ptr(dsc_release as *const ()),               // 2
    crate::Ptr(dsc_create_capture_buffer as *const ()), // 3
    crate::Ptr(dsc_get_caps as *const ()),              // 4
    crate::Ptr(dsc_initialize as *const ()),            // 5
]);

static DSCAPTUREBUFFER8_VTBL: Vtable<14> = Vtable([
    crate::Ptr(dscb_query_interface as *const ()),      // 0
    crate::Ptr(dscb_add_ref as *const ()),              // 1
    crate::Ptr(dscb_release as *const ()),              // 2
    crate::Ptr(dscb_get_caps as *const ()),             // 3
    crate::Ptr(dscb_get_current_position as *const ()), // 4
    crate::Ptr(dscb_get_format as *const ()),           // 5
    crate::Ptr(dscb_get_status as *const ()),           // 6
    crate::Ptr(dscb_initialize as *const ()),           // 7
    crate::Ptr(dscb_lock as *const ()),                 // 8
    crate::Ptr(dscb_start as *const ()),                // 9
    crate::Ptr(dscb_stop as *const ()),                 // 10
    crate::Ptr(dscb_unlock as *const ()),               // 11
    crate::Ptr(dscb_get_object_in_path as *const ()),   // 12 (IDirectSoundCaptureBuffer8)
    crate::Ptr(dscb_get_fx_status as *const ()),        // 13 (IDirectSoundCaptureBuffer8)
]);

static DSCAPTURE_NOTIFY_VTBL: Vtable<4> = Vtable([
    crate::Ptr(dscn_query_interface as *const ()),            // 0
    crate::Ptr(dscn_add_ref as *const ()),                    // 1
    crate::Ptr(dscn_release as *const ()),                    // 2
    crate::Ptr(dscn_set_notification_positions as *const ()), // 3
]);

static DSFULLDUPLEX_VTBL: Vtable<4> = Vtable([
    crate::Ptr(dsfd_query_interface as *const ()), // 0
    crate::Ptr(dsfd_add_ref as *const ()),         // 1
    crate::Ptr(dsfd_release as *const ()),         // 2
    crate::Ptr(dsfd_initialize as *const ()),      // 3
]);

fn ensure_vtbls_registered() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let tables: [&[crate::Ptr]; 10] = [
            &CLASS_FACTORY_VTBL.0,
            &DSOUND8_VTBL.0,
            &DSBUFFER8_VTBL.0,
            &DS3DBUFFER_VTBL.0,
            &DS3DLISTENER_VTBL.0,
            &DSNOTIFY_VTBL.0,
            &DSCAPTURE8_VTBL.0,
            &DSCAPTUREBUFFER8_VTBL.0,
            &DSCAPTURE_NOTIFY_VTBL.0,
            &DSFULLDUPLEX_VTBL.0,
        ];
        for tbl in tables {
            for p in tbl {
                let addr = p.0 as usize;
                crate::register_native_va(addr as u64, addr);
            }
        }
    });
}

// ============================================================================
// COM Object Allocation Helpers
// ============================================================================

fn alloc_ds_device(device: DeviceInfo, initialized: bool, is_ds8: bool) -> *mut DsDeviceCom {
    ensure_vtbls_registered();
    ACTIVE_OBJECTS.fetch_add(1, Ordering::AcqRel);
    Box::into_raw(Box::new(DsDeviceCom {
        vtbl: DSOUND8_VTBL.0.as_ptr(),
        refs: AtomicU32::new(1),
        core: DsDeviceCore::new(device, initialized, is_ds8),
    }))
}

fn alloc_ds_buffer(
    device: Arc<DsDeviceCore>,
    is_primary: bool,
    flags: u32,
    buffer_id: u64,
) -> *mut DsBufferCom {
    ensure_vtbls_registered();
    ACTIVE_OBJECTS.fetch_add(1, Ordering::AcqRel);
    let vtbl_3d = if is_primary {
        DS3DLISTENER_VTBL.0.as_ptr()
    } else {
        DS3DBUFFER_VTBL.0.as_ptr()
    };
    Box::into_raw(Box::new(DsBufferCom {
        vtbl_buf: DSBUFFER8_VTBL.0.as_ptr(),
        vtbl_3d,
        vtbl_notify: DSNOTIFY_VTBL.0.as_ptr(),
        refs: AtomicU32::new(1),
        device,
        is_primary,
        flags,
        buffer_id,
    }))
}

fn alloc_ds_capture(device: DeviceInfo, initialized: bool) -> *mut DsCaptureCom {
    ensure_vtbls_registered();
    ACTIVE_OBJECTS.fetch_add(1, Ordering::AcqRel);
    Box::into_raw(Box::new(DsCaptureCom {
        vtbl: DSCAPTURE8_VTBL.0.as_ptr(),
        refs: AtomicU32::new(1),
        core: DsCaptureCore::new(device, initialized),
    }))
}

fn alloc_ds_capture_buffer(
    capture: Arc<DsCaptureCore>,
    flags: u32,
    sample_mem: Arc<SampleMemory>,
) -> *mut DsCaptureBufferCom {
    ensure_vtbls_registered();
    ACTIVE_OBJECTS.fetch_add(1, Ordering::AcqRel);
    Box::into_raw(Box::new(DsCaptureBufferCom {
        vtbl_cap_buf: DSCAPTUREBUFFER8_VTBL.0.as_ptr(),
        vtbl_notify: DSCAPTURE_NOTIFY_VTBL.0.as_ptr(),
        refs: AtomicU32::new(1),
        capture,
        state: Mutex::new(CaptureBufferState::new(flags, sample_mem)),
    }))
}

fn alloc_ds_full_duplex() -> *mut DsFullDuplexCom {
    ensure_vtbls_registered();
    ACTIVE_OBJECTS.fetch_add(1, Ordering::AcqRel);
    Box::into_raw(Box::new(DsFullDuplexCom {
        vtbl: DSFULLDUPLEX_VTBL.0.as_ptr(),
        refs: AtomicU32::new(1),
        initialized: AtomicBool::new(false),
        capture: Mutex::new(None),
        render: Mutex::new(None),
    }))
}

fn alloc_class_factory(kind: FactoryKind) -> *mut ClassFactoryCom {
    ensure_vtbls_registered();
    ACTIVE_OBJECTS.fetch_add(1, Ordering::AcqRel);
    Box::into_raw(Box::new(ClassFactoryCom {
        vtbl: CLASS_FACTORY_VTBL.0.as_ptr(),
        refs: AtomicU32::new(1),
        kind,
    }))
}

// ============================================================================
// IClassFactory Methods
// ============================================================================

unsafe extern "win64" fn cf_query_interface(
    this: *mut ClassFactoryCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
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
    if iid == IID_IUnknown || iid == IID_ICLASSFACTORY {
        let obj = &*this;
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        return DS_OK;
    }
    E_NOINTERFACE
}

unsafe extern "win64" fn cf_add_ref(this: *mut ClassFactoryCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn cf_release(this: *mut ClassFactoryCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let prev = {
        let obj = &*this;
        obj.refs.fetch_sub(1, Ordering::AcqRel)
    };
    if prev == 1 {
        ACTIVE_OBJECTS.fetch_sub(1, Ordering::AcqRel);
        drop(Box::from_raw(this));
        0
    } else {
        prev - 1
    }
}

unsafe extern "win64" fn cf_create_instance(
    this: *mut ClassFactoryCom,
    p_unk_outer: *mut c_void,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if !p_unk_outer.is_null() {
        return DSERR_NOAGGREGATION;
    }
    let Some(iid) = GUID::read(riid) else {
        return E_POINTER;
    };
    let kind = {
        let obj = &*this;
        obj.kind
    };
    match kind {
        FactoryKind::DirectSound | FactoryKind::DirectSound8 => {
            let is_ds8 = kind == FactoryKind::DirectSound8;
            let raw = alloc_ds_device(backend::null_device(), false, is_ds8);
            let hr = ds_query_interface(raw, &iid, ppv);
            ds_release(raw);
            hr
        }
        FactoryKind::DirectSoundCapture | FactoryKind::DirectSoundCapture8 => {
            let raw = alloc_ds_capture(backend::null_capture_device(), false);
            let hr = dsc_query_interface(raw, &iid, ppv);
            dsc_release(raw);
            hr
        }
        FactoryKind::DirectSoundFullDuplex => {
            let raw = alloc_ds_full_duplex();
            let hr = dsfd_query_interface(raw, &iid, ppv);
            dsfd_release(raw);
            hr
        }
    }
}

unsafe extern "win64" fn cf_lock_server(_this: *mut ClassFactoryCom, f_lock: BOOL) -> HRESULT {
    if f_lock != 0 {
        SERVER_LOCKS.fetch_add(1, Ordering::AcqRel);
    } else {
        let _ = SERVER_LOCKS.fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| {
            Some(n.saturating_sub(1))
        });
    }
    DS_OK
}

// ============================================================================
// IDirectSound / IDirectSound8 Methods
// ============================================================================

unsafe extern "win64" fn ds_query_interface(
    this: *mut DsDeviceCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
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
    let obj = &*this;
    if iid == IID_IUnknown
        || iid == IID_IDIRECTSOUND
        || (obj.core.is_ds8 && iid == IID_IDIRECTSOUND8)
    {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        return DS_OK;
    }
    E_NOINTERFACE
}

unsafe extern "win64" fn ds_add_ref(this: *mut DsDeviceCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn ds_release(this: *mut DsDeviceCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let prev = {
        let obj = &*this;
        obj.refs.fetch_sub(1, Ordering::AcqRel)
    };
    if prev == 1 {
        ACTIVE_OBJECTS.fetch_sub(1, Ordering::AcqRel);
        drop(Box::from_raw(this));
        0
    } else {
        prev - 1
    }
}

unsafe fn create_sound_buffer_core(
    core: &Arc<DsDeviceCore>,
    desc_ptr: *const c_void,
    pp_buf: *mut *mut c_void,
    p_unk_outer: *mut c_void,
) -> HRESULT {
    if pp_buf.is_null() {
        return DSERR_INVALIDPARAM;
    }
    std::ptr::write_unaligned(pp_buf, std::ptr::null_mut());
    if !p_unk_outer.is_null() {
        return DSERR_NOAGGREGATION;
    }
    if desc_ptr.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if !core.initialized.load(Ordering::Acquire) {
        return DSERR_UNINITIALIZED;
    }

    let dw_size = std::ptr::read_unaligned(desc_ptr as *const u32);
    let (flags, buffer_bytes, reserved, wfx_ptr) = if dw_size == 24 {
        let d = std::ptr::read_unaligned(desc_ptr as *const DsBufferDesc1);
        (d.dw_flags, d.dw_buffer_bytes, d.dw_reserved, d.lpwfx_format)
    } else if dw_size == 40 {
        let d = std::ptr::read_unaligned(desc_ptr as *const DsBufferDesc);
        (d.dw_flags, d.dw_buffer_bytes, d.dw_reserved, d.lpwfx_format)
    } else {
        return DSERR_INVALIDPARAM;
    };

    if reserved != 0 {
        return DSERR_INVALIDPARAM;
    }

    let is_primary = (flags & DSBCAPS_PRIMARYBUFFER) != 0;
    if is_primary {
        if buffer_bytes != 0 || !wfx_ptr.is_null() {
            return DSERR_INVALIDPARAM;
        }
        let buf = alloc_ds_buffer(core.clone(), true, flags, 0);
        std::ptr::write_unaligned(pp_buf, buf as *mut c_void);
        return DS_OK;
    }

    if !(DSBSIZE_MIN..=DSBSIZE_MAX).contains(&buffer_bytes) {
        return DSERR_INVALIDPARAM;
    }
    let fmt = match DsFormat::parse(wfx_ptr) {
        Ok(f) => f,
        Err(hr) => return hr,
    };
    if (buffer_bytes as usize) < (fmt.block_align as usize) {
        return DSERR_INVALIDPARAM;
    }

    let sample_mem = SampleMemory::new(buffer_bytes as usize, fmt);
    let buf_id = core.alloc_buf_id();
    {
        let mut st = core.state.lock().unwrap();
        st.buffers.insert(buf_id, SecondaryBufferState::new(flags, sample_mem));
    }
    let buf = alloc_ds_buffer(core.clone(), false, flags, buf_id);
    std::ptr::write_unaligned(pp_buf, buf as *mut c_void);
    DS_OK
}

unsafe extern "win64" fn ds_create_sound_buffer(
    this: *mut DsDeviceCom,
    desc: *const c_void,
    pp_buf: *mut *mut c_void,
    p_unk_outer: *mut c_void,
) -> HRESULT {
    if this.is_null() {
        if !pp_buf.is_null() {
            std::ptr::write_unaligned(pp_buf, std::ptr::null_mut());
        }
        return DSERR_INVALIDPARAM;
    }
    let core = {
        let obj = &*this;
        obj.core.clone()
    };
    create_sound_buffer_core(&core, desc, pp_buf, p_unk_outer)
}

unsafe extern "win64" fn ds_get_caps(this: *mut DsDeviceCom, caps: *mut DsCaps) -> HRESULT {
    if this.is_null() || caps.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if !obj.core.initialized.load(Ordering::Acquire) {
        return DSERR_UNINITIALIZED;
    }
    let sz = std::ptr::read_unaligned(caps as *const u32);
    if sz != std::mem::size_of::<DsCaps>() as u32 {
        return DSERR_INVALIDPARAM;
    }
    let out = DsCaps {
        dw_size: sz,
        dw_flags: DSCAPS_PRIMARYMONO
            | DSCAPS_PRIMARYSTEREO
            | DSCAPS_PRIMARY8BIT
            | DSCAPS_PRIMARY16BIT
            | DSCAPS_CONTINUOUSRATE
            | DSCAPS_CERTIFIED
            | DSCAPS_SECONDARYMONO
            | DSCAPS_SECONDARYSTEREO
            | DSCAPS_SECONDARY8BIT
            | DSCAPS_SECONDARY16BIT,
        dw_min_secondary_sample_rate: DSBFREQUENCY_MIN,
        dw_max_secondary_sample_rate: DSBFREQUENCY_MAX,
        dw_primary_buffers: 1,
        dw_play_cpu_overhead_sw_buffers: 1,
        ..DsCaps::default()
    };
    std::ptr::write_unaligned(caps, out);
    DS_OK
}

unsafe extern "win64" fn ds_duplicate_sound_buffer(
    this: *mut DsDeviceCom,
    orig_ptr: *mut c_void,
    pp_dup: *mut *mut c_void,
) -> HRESULT {
    if pp_dup.is_null() {
        return DSERR_INVALIDPARAM;
    }
    std::ptr::write_unaligned(pp_dup, std::ptr::null_mut());
    if this.is_null() || orig_ptr.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let core = {
        let obj = &*this;
        if !obj.core.initialized.load(Ordering::Acquire) {
            return DSERR_UNINITIALIZED;
        }
        obj.core.clone()
    };

    // Validate that `orig_ptr` is one of our DirectSound buffers belonging to this device.
    let orig_vtbl = std::ptr::read_unaligned(orig_ptr as *const *const crate::Ptr);
    if orig_vtbl != DSBUFFER8_VTBL.0.as_ptr() {
        return DSERR_INVALIDPARAM;
    }
    let orig = &*(orig_ptr as *const DsBufferCom);
    if !Arc::ptr_eq(&orig.device, &core) {
        return DSERR_INVALIDPARAM;
    }
    if orig.is_primary {
        return DSERR_INVALIDCALL;
    }

    let dup_id = core.alloc_buf_id();
    let flags = {
        let mut st = core.state.lock().unwrap();
        let Some(src) = st.buffers.get(&orig.buffer_id) else {
            return DSERR_INVALIDPARAM;
        };
        let mut dup_state = SecondaryBufferState::new(src.flags, src.sample_mem.clone());
        dup_state.frequency = src.frequency;
        dup_state.volume_mb = src.volume_mb;
        dup_state.pan_mb = src.pan_mb;
        dup_state.d3d_current = src.d3d_current;
        dup_state.d3d_deferred = src.d3d_deferred;
        let f = dup_state.flags;
        st.buffers.insert(dup_id, dup_state);
        f
    };

    let dup_buf = alloc_ds_buffer(core, false, flags, dup_id);
    std::ptr::write_unaligned(pp_dup, dup_buf as *mut c_void);
    DS_OK
}

unsafe extern "win64" fn ds_set_cooperative_level(
    this: *mut DsDeviceCom,
    _hwnd: HWND,
    dw_level: u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if !obj.core.initialized.load(Ordering::Acquire) {
        return DSERR_UNINITIALIZED;
    }
    if !(DSSCL_NORMAL..=DSSCL_WRITEPRIMARY).contains(&dw_level) {
        return DSERR_INVALIDPARAM;
    }
    obj.core.state.lock().unwrap().cooperative_level = dw_level;
    DS_OK
}

unsafe extern "win64" fn ds_compact(this: *mut DsDeviceCom) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if !obj.core.initialized.load(Ordering::Acquire) {
        return DSERR_UNINITIALIZED;
    }
    let coop = obj.core.state.lock().unwrap().cooperative_level;
    if coop < DSSCL_PRIORITY {
        return DSERR_PRIOLEVELNEEDED;
    }
    DS_OK
}

unsafe extern "win64" fn ds_get_speaker_config(
    this: *mut DsDeviceCom,
    out: *mut u32,
) -> HRESULT {
    if this.is_null() || out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if !obj.core.initialized.load(Ordering::Acquire) {
        return DSERR_UNINITIALIZED;
    }
    let cfg = obj.core.state.lock().unwrap().speaker_config;
    std::ptr::write_unaligned(out, cfg);
    DS_OK
}

unsafe extern "win64" fn ds_set_speaker_config(this: *mut DsDeviceCom, cfg: u32) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if !obj.core.initialized.load(Ordering::Acquire) {
        return DSERR_UNINITIALIZED;
    }
    obj.core.state.lock().unwrap().speaker_config = cfg;
    DS_OK
}

unsafe extern "win64" fn ds_initialize(this: *mut DsDeviceCom, guid_ptr: *const GUID) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if obj.core.initialized.load(Ordering::Acquire) {
        return DSERR_ALREADYINITIALIZED;
    }
    let guid_opt = GUID::read(guid_ptr);
    let dev = match resolve_render_device(guid_opt) {
        Ok(d) => d,
        Err(hr) => return hr,
    };
    {
        let mut st = obj.core.state.lock().unwrap();
        if (DSBFREQUENCY_MIN..=DSBFREQUENCY_MAX).contains(&dev.rate) {
            st.primary_format.sample_rate = dev.rate;
        }
        st.device = dev;
    }
    obj.core.initialized.store(true, Ordering::Release);
    DS_OK
}

unsafe extern "win64" fn ds_verify_certification(
    this: *mut DsDeviceCom,
    certified: *mut u32,
) -> HRESULT {
    if this.is_null() || certified.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if !obj.core.initialized.load(Ordering::Acquire) {
        return DSERR_UNINITIALIZED;
    }
    std::ptr::write_unaligned(certified, DS_CERTIFIED);
    DS_OK
}

// ============================================================================
// IDirectSoundBuffer / IDirectSoundBuffer8 Methods + Sub-Interface Dispatch
// ============================================================================

unsafe fn dsbuf_qi_common(
    base: *mut DsBufferCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
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
    if iid == IID_IUnknown
        || iid == IID_IDIRECTSOUNDBUFFER
        || (!obj.is_primary && obj.device.is_ds8 && iid == IID_IDIRECTSOUNDBUFFER8)
    {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, base as *mut c_void);
        return DS_OK;
    }
    if (obj.flags & DSBCAPS_CTRL3D) != 0
        && ((obj.is_primary && iid == IID_IDIRECTSOUND3DLISTENER)
            || (!obj.is_primary && iid == IID_IDIRECTSOUND3DBUFFER))
    {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        let sub = (base as *mut u8).add(std::mem::offset_of!(DsBufferCom, vtbl_3d));
        std::ptr::write_unaligned(ppv, sub as *mut c_void);
        return DS_OK;
    }
    if (obj.flags & DSBCAPS_CTRLPOSITIONNOTIFY) != 0 && iid == IID_IDIRECTSOUNDNOTIFY {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        let sub = (base as *mut u8).add(std::mem::offset_of!(DsBufferCom, vtbl_notify));
        std::ptr::write_unaligned(ppv, sub as *mut c_void);
        return DS_OK;
    }
    E_NOINTERFACE
}

unsafe fn dsbuf_add_ref_common(base: *mut DsBufferCom) -> u32 {
    if base.is_null() {
        return 0;
    }
    let obj = &*base;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe fn dsbuf_release_common(base: *mut DsBufferCom) -> u32 {
    if base.is_null() {
        return 0;
    }
    let prev = {
        let obj = &*base;
        obj.refs.fetch_sub(1, Ordering::AcqRel)
    };
    if prev == 1 {
        let boxed = Box::from_raw(base);
        if !boxed.is_primary {
            boxed.device.state.lock().unwrap().buffers.remove(&boxed.buffer_id);
        }
        ACTIVE_OBJECTS.fetch_sub(1, Ordering::AcqRel);
        drop(boxed);
        0
    } else {
        prev - 1
    }
}

#[inline]
unsafe fn dsbuf_from_3d(this: *mut c_void) -> *mut DsBufferCom {
    if this.is_null() {
        std::ptr::null_mut()
    } else {
        (this as *mut u8).sub(std::mem::offset_of!(DsBufferCom, vtbl_3d)) as *mut DsBufferCom
    }
}

#[inline]
unsafe fn dsbuf_from_notify(this: *mut c_void) -> *mut DsBufferCom {
    if this.is_null() {
        std::ptr::null_mut()
    } else {
        (this as *mut u8).sub(std::mem::offset_of!(DsBufferCom, vtbl_notify)) as *mut DsBufferCom
    }
}

unsafe extern "win64" fn dsb_query_interface(
    this: *mut DsBufferCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    dsbuf_qi_common(this, riid, ppv)
}

unsafe extern "win64" fn dsb_add_ref(this: *mut DsBufferCom) -> u32 {
    dsbuf_add_ref_common(this)
}

unsafe extern "win64" fn dsb_release(this: *mut DsBufferCom) -> u32 {
    dsbuf_release_common(this)
}

unsafe extern "win64" fn dsb_get_caps(this: *mut DsBufferCom, caps: *mut DsbCaps) -> HRESULT {
    if this.is_null() || caps.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let sz = std::ptr::read_unaligned(caps as *const u32);
    if sz != std::mem::size_of::<DsbCaps>() as u32 {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let st = obj.device.state.lock().unwrap();
    let byte_len = if obj.is_primary {
        // Primary buffers report a nominal buffer size (40 ms of primary mix format).
        (st.primary_format.sample_rate / 25) * (st.primary_format.block_align as u32)
    } else {
        match st.buffers.get(&obj.buffer_id) {
            Some(b) => b.sample_mem.byte_len() as u32,
            None => return DSERR_INVALIDCALL,
        }
    };
    std::ptr::write_unaligned(
        caps,
        DsbCaps {
            dw_size: sz,
            dw_flags: obj.flags,
            dw_buffer_bytes: byte_len,
            dw_unlock_transfer_rate: 0,
            dw_play_cpu_overhead: 0,
        },
    );
    DS_OK
}

unsafe extern "win64" fn dsb_get_current_position(
    this: *mut DsBufferCom,
    play_out: *mut u32,
    write_out: *mut u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let st = obj.device.state.lock().unwrap();
    let (play_pos, write_pos) = if obj.is_primary {
        let prim_frames = (st.primary_format.sample_rate / 25).max(1) as u64;
        let block = st.primary_format.block_align as u32;
        if st.primary_playing {
            let pf = (st.primary_frames_mixed % prim_frames) as u32;
            let lead = ((st.primary_format.sample_rate / 100).max(1)).min((prim_frames as u32 / 2).max(1));
            let wf = ((pf + lead) % prim_frames as u32) * block;
            (pf * block, wf)
        } else {
            let pf = (st.primary_frames_mixed % prim_frames) as u32 * block;
            (pf, pf)
        }
    } else {
        let Some(b) = st.buffers.get(&obj.buffer_id) else {
            return DSERR_INVALIDCALL;
        };
        b.current_cursors()
    };
    if !play_out.is_null() {
        std::ptr::write_unaligned(play_out, play_pos);
    }
    if !write_out.is_null() {
        std::ptr::write_unaligned(write_out, write_pos);
    }
    DS_OK
}

unsafe extern "win64" fn dsb_get_format(
    this: *mut DsBufferCom,
    wfx_out: *mut u8,
    allocated: u32,
    written_out: *mut u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let fmt = {
        let st = obj.device.state.lock().unwrap();
        if obj.is_primary {
            st.primary_format
        } else {
            let Some(b) = st.buffers.get(&obj.buffer_id) else {
                return DSERR_INVALIDCALL;
            };
            b.sample_mem.format
        }
    };
    fmt.write_to(wfx_out, allocated, written_out)
}

unsafe extern "win64" fn dsb_get_volume(this: *mut DsBufferCom, out: *mut i32) -> HRESULT {
    if this.is_null() || out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let st = obj.device.state.lock().unwrap();
    let vol = if obj.is_primary {
        st.primary_volume_mb
    } else {
        let Some(b) = st.buffers.get(&obj.buffer_id) else {
            return DSERR_INVALIDCALL;
        };
        b.volume_mb
    };
    std::ptr::write_unaligned(out, vol);
    DS_OK
}

unsafe extern "win64" fn dsb_get_pan(this: *mut DsBufferCom, out: *mut i32) -> HRESULT {
    if this.is_null() || out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let st = obj.device.state.lock().unwrap();
    let pan = if obj.is_primary {
        st.primary_pan_mb
    } else {
        let Some(b) = st.buffers.get(&obj.buffer_id) else {
            return DSERR_INVALIDCALL;
        };
        b.pan_mb
    };
    std::ptr::write_unaligned(out, pan);
    DS_OK
}

unsafe extern "win64" fn dsb_get_frequency(this: *mut DsBufferCom, out: *mut u32) -> HRESULT {
    if this.is_null() || out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let st = obj.device.state.lock().unwrap();
    let freq = if obj.is_primary {
        st.primary_format.sample_rate
    } else {
        let Some(b) = st.buffers.get(&obj.buffer_id) else {
            return DSERR_INVALIDCALL;
        };
        b.frequency
    };
    std::ptr::write_unaligned(out, freq);
    DS_OK
}

unsafe extern "win64" fn dsb_get_status(this: *mut DsBufferCom, out: *mut u32) -> HRESULT {
    if this.is_null() || out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let st = obj.device.state.lock().unwrap();
    let status = if obj.is_primary {
        if st.primary_playing {
            DSBSTATUS_PLAYING | DSBSTATUS_LOOPING
        } else {
            0
        }
    } else {
        let Some(b) = st.buffers.get(&obj.buffer_id) else {
            return DSERR_INVALIDCALL;
        };
        let mut s = 0u32;
        if b.playing {
            s |= DSBSTATUS_PLAYING;
            if b.looping {
                s |= DSBSTATUS_LOOPING;
            }
        }
        s
    };
    std::ptr::write_unaligned(out, status);
    DS_OK
}

unsafe extern "win64" fn dsb_initialize(
    _this: *mut DsBufferCom,
    _ds: *mut c_void,
    _desc: *const c_void,
) -> HRESULT {
    DSERR_ALREADYINITIALIZED
}

unsafe extern "win64" fn dsb_lock(
    this: *mut DsBufferCom,
    dw_offset: u32,
    dw_bytes: u32,
    ppv_ptr1: *mut *mut c_void,
    pdw_bytes1: *mut u32,
    ppv_ptr2: *mut *mut c_void,
    pdw_bytes2: *mut u32,
    dw_flags: u32,
) -> HRESULT {
    if !ppv_ptr1.is_null() {
        std::ptr::write_unaligned(ppv_ptr1, std::ptr::null_mut());
    }
    if !pdw_bytes1.is_null() {
        std::ptr::write_unaligned(pdw_bytes1, 0);
    }
    if !ppv_ptr2.is_null() {
        std::ptr::write_unaligned(ppv_ptr2, std::ptr::null_mut());
    }
    if !pdw_bytes2.is_null() {
        std::ptr::write_unaligned(pdw_bytes2, 0);
    }
    if this.is_null() || ppv_ptr1.is_null() || pdw_bytes1.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if ppv_ptr2.is_null() != pdw_bytes2.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if obj.is_primary {
        return DSERR_INVALIDCALL;
    }

    let mut st = obj.device.state.lock().unwrap();
    let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
        return DSERR_INVALIDCALL;
    };
    let buf_len = b.sample_mem.byte_len() as u32;
    let offset = if (dw_flags & DSBLOCK_FROMWRITECURSOR) != 0 {
        b.current_cursors().1
    } else {
        dw_offset
    };
    let bytes = if (dw_flags & DSBLOCK_ENTIREBUFFER) != 0 {
        buf_len
    } else {
        dw_bytes
    };

    if offset >= buf_len || bytes == 0 || bytes > buf_len {
        return DSERR_INVALIDPARAM;
    }

    let base = b.sample_mem.base_mut_ptr();
    let first_len = bytes.min(buf_len - offset);
    let second_len = bytes - first_len;
    let ptr1 = base.add(offset as usize) as *mut c_void;
    let (ptr2, actual_len2) = if second_len > 0 && !ppv_ptr2.is_null() {
        (base as *mut c_void, second_len)
    } else {
        (std::ptr::null_mut(), 0)
    };

    b.locks.push(ActiveLock {
        ptr1: ptr1 as usize,
        len1: first_len,
        ptr2: ptr2 as usize,
        len2: actual_len2,
    });

    std::ptr::write_unaligned(ppv_ptr1, ptr1);
    std::ptr::write_unaligned(pdw_bytes1, first_len);
    if !ppv_ptr2.is_null() {
        std::ptr::write_unaligned(ppv_ptr2, ptr2);
        std::ptr::write_unaligned(pdw_bytes2, actual_len2);
    }
    DS_OK
}

unsafe extern "win64" fn dsb_play(
    this: *mut DsBufferCom,
    dw_reserved1: u32,
    _dw_priority: u32,
    dw_flags: u32,
) -> HRESULT {
    if this.is_null() || dw_reserved1 != 0 {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let looping = (dw_flags & DSBPLAY_LOOPING) != 0;
    {
        let mut st = obj.device.state.lock().unwrap();
        if obj.is_primary {
            if !looping {
                return DSERR_INVALIDPARAM;
            }
            st.primary_playing = true;
        } else {
            let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
                return DSERR_INVALIDCALL;
            };
            b.playing = true;
            b.looping = looping;
        }
    }
    obj.device.ensure_mixer_started();
    DS_OK
}

unsafe extern "win64" fn dsb_set_current_position(
    this: *mut DsBufferCom,
    dw_new_position: u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if obj.is_primary {
        return DSERR_INVALIDCALL;
    }
    let mut st = obj.device.state.lock().unwrap();
    let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
        return DSERR_INVALIDCALL;
    };
    let buf_len = b.sample_mem.byte_len() as u32;
    if dw_new_position >= buf_len {
        return DSERR_INVALIDPARAM;
    }
    let block = (b.sample_mem.format.block_align as u32).max(1);
    b.play_pos_frames = (dw_new_position / block) as f64;
    DS_OK
}

unsafe extern "win64" fn dsb_set_format(this: *mut DsBufferCom, wfx: *const u8) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if !obj.is_primary {
        return DSERR_INVALIDCALL;
    }
    let fmt = match DsFormat::parse(wfx) {
        Ok(f) => f,
        Err(hr) => return hr,
    };
    {
        let mut st = obj.device.state.lock().unwrap();
        if st.cooperative_level < DSSCL_PRIORITY {
            return DSERR_PRIOLEVELNEEDED;
        }
        st.primary_format = fmt;
    }
    obj.device.restart_mixer_if_running(fmt.sample_rate);
    DS_OK
}

unsafe extern "win64" fn dsb_set_volume(this: *mut DsBufferCom, l_volume: i32) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if !(DSBVOLUME_MIN..=DSBVOLUME_MAX).contains(&l_volume) {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if (obj.flags & DSBCAPS_CTRLVOLUME) == 0 {
        return DSERR_CONTROLUNAVAIL;
    }
    let mut st = obj.device.state.lock().unwrap();
    if obj.is_primary {
        st.primary_volume_mb = l_volume;
    } else {
        let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
            return DSERR_INVALIDCALL;
        };
        b.volume_mb = l_volume;
    }
    DS_OK
}

unsafe extern "win64" fn dsb_set_pan(this: *mut DsBufferCom, l_pan: i32) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if !(DSBPAN_LEFT..=DSBPAN_RIGHT).contains(&l_pan) {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if (obj.flags & DSBCAPS_CTRLPAN) == 0 {
        return DSERR_CONTROLUNAVAIL;
    }
    let mut st = obj.device.state.lock().unwrap();
    if obj.is_primary {
        st.primary_pan_mb = l_pan;
    } else {
        let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
            return DSERR_INVALIDCALL;
        };
        b.pan_mb = l_pan;
    }
    DS_OK
}

unsafe extern "win64" fn dsb_set_frequency(this: *mut DsBufferCom, dw_frequency: u32) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if obj.is_primary {
        return DSERR_CONTROLUNAVAIL;
    }
    if (obj.flags & DSBCAPS_CTRLFREQUENCY) == 0 {
        return DSERR_CONTROLUNAVAIL;
    }
    if dw_frequency != DSBFREQUENCY_ORIGINAL
        && !(DSBFREQUENCY_MIN..=DSBFREQUENCY_MAX).contains(&dw_frequency)
    {
        return DSERR_INVALIDPARAM;
    }
    let mut st = obj.device.state.lock().unwrap();
    let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
        return DSERR_INVALIDCALL;
    };
    b.frequency = if dw_frequency == DSBFREQUENCY_ORIGINAL {
        b.sample_mem.format.sample_rate
    } else {
        dw_frequency
    };
    DS_OK
}

unsafe extern "win64" fn dsb_stop(this: *mut DsBufferCom) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let mut events = Vec::new();
    {
        let mut st = obj.device.state.lock().unwrap();
        if obj.is_primary {
            st.primary_playing = false;
        } else {
            let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
                return DSERR_INVALIDCALL;
            };
            let was_playing = b.playing;
            b.playing = false;
            b.looping = false;
            if was_playing {
                b.collect_stop_events(&mut events);
            }
        }
    }
    signal_events(events);
    DS_OK
}

unsafe extern "win64" fn dsb_unlock(
    this: *mut DsBufferCom,
    pv_ptr1: *mut c_void,
    dw_bytes1: u32,
    pv_ptr2: *mut c_void,
    dw_bytes2: u32,
) -> HRESULT {
    if this.is_null() || pv_ptr1.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if obj.is_primary {
        return DSERR_INVALIDCALL;
    }
    let mut st = obj.device.state.lock().unwrap();
    let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
        return DSERR_INVALIDCALL;
    };
    let pos = b.locks.iter().position(|lk| {
        lk.ptr1 == (pv_ptr1 as usize)
            && dw_bytes1 <= lk.len1
            && (dw_bytes2 == 0 || (lk.ptr2 == (pv_ptr2 as usize) && dw_bytes2 <= lk.len2))
    });
    let Some(idx) = pos else {
        return DSERR_INVALIDPARAM;
    };
    b.locks.swap_remove(idx);
    std::sync::atomic::fence(Ordering::Release);
    DS_OK
}

unsafe extern "win64" fn dsb_restore(this: *mut DsBufferCom) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    DS_OK
}

unsafe extern "win64" fn dsb_set_fx(
    this: *mut DsBufferCom,
    dw_effects_count: u32,
    _p_dsfx_desc: *const c_void,
    pdw_result_codes: *mut u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if dw_effects_count == 0 {
        let st = obj.device.state.lock().unwrap();
        if let Some(b) = st.buffers.get(&obj.buffer_id) {
            if b.playing {
                return DSERR_INVALIDCALL;
            }
        }
        return DS_OK;
    }
    if !pdw_result_codes.is_null() {
        for i in 0..(dw_effects_count as usize) {
            std::ptr::write_unaligned(pdw_result_codes.add(i), DSFXR_FAILED);
        }
    }
    crate::stub!("IDirectSoundBuffer8::SetFX({dw_effects_count} effects) -> DSERR_FXUNAVAILABLE");
    DSERR_FXUNAVAILABLE
}

unsafe extern "win64" fn dsb_acquire_resources(
    this: *mut DsBufferCom,
    _dw_flags: u32,
    dw_effects_count: u32,
    pdw_result_codes: *mut u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if dw_effects_count == 0 {
        return DS_OK;
    }
    if !pdw_result_codes.is_null() {
        for i in 0..(dw_effects_count as usize) {
            std::ptr::write_unaligned(pdw_result_codes.add(i), DSFXR_FAILED);
        }
    }
    DSERR_FXUNAVAILABLE
}

unsafe extern "win64" fn dsb_get_object_in_path(
    this: *mut DsBufferCom,
    _rguid_object: *const GUID,
    _dw_index: u32,
    _rguid_interface: *const GUID,
    pp_object: *mut *mut c_void,
) -> HRESULT {
    if !pp_object.is_null() {
        std::ptr::write_unaligned(pp_object, std::ptr::null_mut());
    }
    if this.is_null() || pp_object.is_null() {
        return DSERR_INVALIDPARAM;
    }
    DSERR_OBJECTNOTFOUND
}

// ============================================================================
// IDirectSound3DBuffer Methods
// ============================================================================

unsafe extern "win64" fn ds3db_query_interface(
    this: *mut c_void,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    dsbuf_qi_common(dsbuf_from_3d(this), riid, ppv)
}

unsafe extern "win64" fn ds3db_add_ref(this: *mut c_void) -> u32 {
    dsbuf_add_ref_common(dsbuf_from_3d(this))
}

unsafe extern "win64" fn ds3db_release(this: *mut c_void) -> u32 {
    dsbuf_release_common(dsbuf_from_3d(this))
}

unsafe fn with_3d_buf<R>(
    this: *mut c_void,
    f: impl FnOnce(&SecondaryBufferState) -> R,
) -> Result<R, HRESULT> {
    let base = dsbuf_from_3d(this);
    if base.is_null() {
        return Err(DSERR_INVALIDPARAM);
    }
    let obj = &*base;
    let st = obj.device.state.lock().unwrap();
    let Some(b) = st.buffers.get(&obj.buffer_id) else {
        return Err(DSERR_INVALIDCALL);
    };
    Ok(f(b))
}

unsafe fn with_3d_buf_mut(
    this: *mut c_void,
    dw_apply: u32,
    f: impl FnOnce(&mut Buffer3dParams) -> HRESULT,
) -> HRESULT {
    if dw_apply != DS3D_IMMEDIATE && dw_apply != DS3D_DEFERRED {
        return DSERR_INVALIDPARAM;
    }
    let base = dsbuf_from_3d(this);
    if base.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*base;
    let mut st = obj.device.state.lock().unwrap();
    let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
        return DSERR_INVALIDCALL;
    };
    let mut next = if dw_apply == DS3D_DEFERRED {
        b.d3d_deferred
    } else {
        b.d3d_current
    };
    let hr = f(&mut next);
    if hr != DS_OK {
        return hr;
    }
    b.d3d_deferred = next;
    if dw_apply == DS3D_IMMEDIATE {
        b.d3d_current = next;
    }
    DS_OK
}

unsafe extern "win64" fn ds3db_get_all_parameters(
    this: *mut c_void,
    out: *mut Ds3dBuffer,
) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let sz = std::ptr::read_unaligned(out as *const u32);
    if sz != std::mem::size_of::<Ds3dBuffer>() as u32 {
        return DSERR_INVALIDPARAM;
    }
    match with_3d_buf(this, |b| b.d3d_current.to_abi()) {
        Ok(v) => {
            std::ptr::write_unaligned(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3db_get_cone_angles(
    this: *mut c_void,
    inside_out: *mut u32,
    outside_out: *mut u32,
) -> HRESULT {
    if inside_out.is_null() || outside_out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_3d_buf(this, |b| {
        (b.d3d_current.inside_cone_angle, b.d3d_current.outside_cone_angle)
    }) {
        Ok((i, o)) => {
            std::ptr::write_unaligned(inside_out, i);
            std::ptr::write_unaligned(outside_out, o);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3db_get_cone_orientation(
    this: *mut c_void,
    out: *mut D3dVector,
) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_3d_buf(this, |b| b.d3d_current.cone_orientation) {
        Ok(v) => {
            std::ptr::write_unaligned(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3db_get_cone_outside_volume(
    this: *mut c_void,
    out: *mut i32,
) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_3d_buf(this, |b| b.d3d_current.cone_outside_volume) {
        Ok(v) => {
            std::ptr::write_unaligned(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3db_get_max_distance(this: *mut c_void, out: *mut f32) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_3d_buf(this, |b| b.d3d_current.max_distance) {
        Ok(v) => {
            std::ptr::write_unaligned(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3db_get_min_distance(this: *mut c_void, out: *mut f32) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_3d_buf(this, |b| b.d3d_current.min_distance) {
        Ok(v) => {
            std::ptr::write_unaligned(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3db_get_mode(this: *mut c_void, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_3d_buf(this, |b| b.d3d_current.mode) {
        Ok(v) => {
            std::ptr::write_unaligned(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3db_get_position(this: *mut c_void, out: *mut D3dVector) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_3d_buf(this, |b| b.d3d_current.position) {
        Ok(v) => {
            std::ptr::write_unaligned(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3db_get_velocity(this: *mut c_void, out: *mut D3dVector) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_3d_buf(this, |b| b.d3d_current.velocity) {
        Ok(v) => {
            std::ptr::write_unaligned(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3db_set_all_parameters(
    this: *mut c_void,
    params_ptr: *const Ds3dBuffer,
    dw_apply: u32,
) -> HRESULT {
    if params_ptr.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let p = std::ptr::read_unaligned(params_ptr);
    if p.dw_size != std::mem::size_of::<Ds3dBuffer>() as u32 {
        return DSERR_INVALIDPARAM;
    }
    if !vec_is_finite(p.v_position)
        || !vec_is_finite(p.v_velocity)
        || !vec_is_finite(p.v_cone_orientation)
        || p.dw_inside_cone_angle > DS3D_MAXCONEANGLE
        || p.dw_outside_cone_angle > DS3D_MAXCONEANGLE
        || p.dw_inside_cone_angle > p.dw_outside_cone_angle
        || !(DSBVOLUME_MIN..=DSBVOLUME_MAX).contains(&p.l_cone_outside_volume)
        || !p.fl_min_distance.is_finite()
        || !p.fl_max_distance.is_finite()
        || p.fl_min_distance <= 0.0
        || p.fl_max_distance < p.fl_min_distance
        || p.dw_mode > DS3DMODE_DISABLE
    {
        return DSERR_INVALIDPARAM;
    }
    let cone_dir = vec_normalize(p.v_cone_orientation)
        .unwrap_or(D3dVector { x: 0.0, y: 0.0, z: 1.0 });
    with_3d_buf_mut(this, dw_apply, |dst| {
        *dst = Buffer3dParams {
            position: p.v_position,
            velocity: p.v_velocity,
            inside_cone_angle: p.dw_inside_cone_angle,
            outside_cone_angle: p.dw_outside_cone_angle,
            cone_orientation: cone_dir,
            cone_outside_volume: p.l_cone_outside_volume,
            min_distance: p.fl_min_distance,
            max_distance: p.fl_max_distance,
            mode: p.dw_mode,
        };
        DS_OK
    })
}

unsafe extern "win64" fn ds3db_set_cone_angles(
    this: *mut c_void,
    inside: u32,
    outside: u32,
    dw_apply: u32,
) -> HRESULT {
    if inside > DS3D_MAXCONEANGLE || outside > DS3D_MAXCONEANGLE || inside > outside {
        return DSERR_INVALIDPARAM;
    }
    with_3d_buf_mut(this, dw_apply, |dst| {
        dst.inside_cone_angle = inside;
        dst.outside_cone_angle = outside;
        DS_OK
    })
}

unsafe extern "win64" fn ds3db_set_cone_orientation(
    this: *mut c_void,
    x: f32,
    y: f32,
    z: f32,
    dw_apply: u32,
) -> HRESULT {
    let v = D3dVector { x, y, z };
    let Some(norm) = vec_normalize(v) else {
        return DSERR_INVALIDPARAM;
    };
    with_3d_buf_mut(this, dw_apply, |dst| {
        dst.cone_orientation = norm;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3db_set_cone_orientation(*mut c_void, f32, f32, f32, u32) -> HRESULT);

unsafe extern "win64" fn ds3db_set_cone_outside_volume(
    this: *mut c_void,
    vol: i32,
    dw_apply: u32,
) -> HRESULT {
    if !(DSBVOLUME_MIN..=DSBVOLUME_MAX).contains(&vol) {
        return DSERR_INVALIDPARAM;
    }
    with_3d_buf_mut(this, dw_apply, |dst| {
        dst.cone_outside_volume = vol;
        DS_OK
    })
}

unsafe extern "win64" fn ds3db_set_max_distance(
    this: *mut c_void,
    max_d: f32,
    dw_apply: u32,
) -> HRESULT {
    if !max_d.is_finite() || max_d <= 0.0 {
        return DSERR_INVALIDPARAM;
    }
    with_3d_buf_mut(this, dw_apply, |dst| {
        if max_d < dst.min_distance {
            return DSERR_INVALIDPARAM;
        }
        dst.max_distance = max_d;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3db_set_max_distance(*mut c_void, f32, u32) -> HRESULT);

unsafe extern "win64" fn ds3db_set_min_distance(
    this: *mut c_void,
    min_d: f32,
    dw_apply: u32,
) -> HRESULT {
    if !min_d.is_finite() || min_d <= 0.0 {
        return DSERR_INVALIDPARAM;
    }
    with_3d_buf_mut(this, dw_apply, |dst| {
        if min_d > dst.max_distance {
            return DSERR_INVALIDPARAM;
        }
        dst.min_distance = min_d;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3db_set_min_distance(*mut c_void, f32, u32) -> HRESULT);

unsafe extern "win64" fn ds3db_set_mode(
    this: *mut c_void,
    mode: u32,
    dw_apply: u32,
) -> HRESULT {
    if mode > DS3DMODE_DISABLE {
        return DSERR_INVALIDPARAM;
    }
    with_3d_buf_mut(this, dw_apply, |dst| {
        dst.mode = mode;
        DS_OK
    })
}

unsafe extern "win64" fn ds3db_set_position(
    this: *mut c_void,
    x: f32,
    y: f32,
    z: f32,
    dw_apply: u32,
) -> HRESULT {
    let v = D3dVector { x, y, z };
    if !vec_is_finite(v) {
        return DSERR_INVALIDPARAM;
    }
    with_3d_buf_mut(this, dw_apply, |dst| {
        dst.position = v;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3db_set_position(*mut c_void, f32, f32, f32, u32) -> HRESULT);

unsafe extern "win64" fn ds3db_set_velocity(
    this: *mut c_void,
    x: f32,
    y: f32,
    z: f32,
    dw_apply: u32,
) -> HRESULT {
    let v = D3dVector { x, y, z };
    if !vec_is_finite(v) {
        return DSERR_INVALIDPARAM;
    }
    with_3d_buf_mut(this, dw_apply, |dst| {
        dst.velocity = v;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3db_set_velocity(*mut c_void, f32, f32, f32, u32) -> HRESULT);

// ============================================================================
// IDirectSound3DListener Methods
// ============================================================================

unsafe extern "win64" fn ds3dl_query_interface(
    this: *mut c_void,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    dsbuf_qi_common(dsbuf_from_3d(this), riid, ppv)
}

unsafe extern "win64" fn ds3dl_add_ref(this: *mut c_void) -> u32 {
    dsbuf_add_ref_common(dsbuf_from_3d(this))
}

unsafe extern "win64" fn ds3dl_release(this: *mut c_void) -> u32 {
    dsbuf_release_common(dsbuf_from_3d(this))
}

unsafe fn with_listener<R>(
    this: *mut c_void,
    f: impl FnOnce(&Listener3dParams) -> R,
) -> Result<R, HRESULT> {
    let base = dsbuf_from_3d(this);
    if base.is_null() {
        return Err(DSERR_INVALIDPARAM);
    }
    let obj = &*base;
    let st = obj.device.state.lock().unwrap();
    Ok(f(&st.listener_current))
}

unsafe fn with_listener_mut(
    this: *mut c_void,
    dw_apply: u32,
    f: impl FnOnce(&mut Listener3dParams) -> HRESULT,
) -> HRESULT {
    if dw_apply != DS3D_IMMEDIATE && dw_apply != DS3D_DEFERRED {
        return DSERR_INVALIDPARAM;
    }
    let base = dsbuf_from_3d(this);
    if base.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*base;
    let mut st = obj.device.state.lock().unwrap();
    let mut next = if dw_apply == DS3D_DEFERRED {
        st.listener_deferred
    } else {
        st.listener_current
    };
    let hr = f(&mut next);
    if hr != DS_OK {
        return hr;
    }
    st.listener_deferred = next;
    if dw_apply == DS3D_IMMEDIATE {
        st.listener_current = next;
    }
    DS_OK
}

unsafe extern "win64" fn ds3dl_get_all_parameters(
    this: *mut c_void,
    out: *mut Ds3dListener,
) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let sz = std::ptr::read_unaligned(out as *const u32);
    if sz != std::mem::size_of::<Ds3dListener>() as u32 {
        return DSERR_INVALIDPARAM;
    }
    match with_listener(this, |l| l.to_abi()) {
        Ok(v) => {
            std::ptr::write_unaligned(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3dl_get_distance_factor(
    this: *mut c_void,
    out: *mut f32,
) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_listener(this, |l| l.distance_factor) {
        Ok(v) => {
            std::ptr::write_unaligned(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3dl_get_doppler_factor(
    this: *mut c_void,
    out: *mut f32,
) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_listener(this, |l| l.doppler_factor) {
        Ok(v) => {
            std::ptr::write_unaligned(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3dl_get_orientation(
    this: *mut c_void,
    front_out: *mut D3dVector,
    top_out: *mut D3dVector,
) -> HRESULT {
    if front_out.is_null() && top_out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_listener(this, |l| (l.orient_front, l.orient_top)) {
        Ok((f, t)) => {
            if !front_out.is_null() {
                std::ptr::write_unaligned(front_out, f);
            }
            if !top_out.is_null() {
                std::ptr::write_unaligned(top_out, t);
            }
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3dl_get_position(this: *mut c_void, out: *mut D3dVector) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_listener(this, |l| l.position) {
        Ok(v) => {
            std::ptr::write_unaligned(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3dl_get_rolloff_factor(
    this: *mut c_void,
    out: *mut f32,
) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_listener(this, |l| l.rolloff_factor) {
        Ok(v) => {
            std::ptr::write_unaligned(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3dl_get_velocity(this: *mut c_void, out: *mut D3dVector) -> HRESULT {
    if out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    match with_listener(this, |l| l.velocity) {
        Ok(v) => {
            std::ptr::write_unaligned(out, v);
            DS_OK
        }
        Err(hr) => hr,
    }
}

unsafe extern "win64" fn ds3dl_set_all_parameters(
    this: *mut c_void,
    params_ptr: *const Ds3dListener,
    dw_apply: u32,
) -> HRESULT {
    if params_ptr.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let p = std::ptr::read_unaligned(params_ptr);
    if p.dw_size != std::mem::size_of::<Ds3dListener>() as u32 {
        return DSERR_INVALIDPARAM;
    }
    let Some(front) = vec_normalize(p.v_orient_front) else {
        return DSERR_INVALIDPARAM;
    };
    let Some(top) = vec_normalize(p.v_orient_top) else {
        return DSERR_INVALIDPARAM;
    };
    if vec_normalize(vec_cross(top, front)).is_none()
        || !vec_is_finite(p.v_position)
        || !vec_is_finite(p.v_velocity)
        || !p.fl_distance_factor.is_finite()
        || p.fl_distance_factor <= 0.0
        || !p.fl_rolloff_factor.is_finite()
        || !(DS3D_MINROLLOFFFACTOR..=DS3D_MAXROLLOFFFACTOR).contains(&p.fl_rolloff_factor)
        || !p.fl_doppler_factor.is_finite()
        || !(DS3D_MINDOPPLERFACTOR..=DS3D_MAXDOPPLERFACTOR).contains(&p.fl_doppler_factor)
    {
        return DSERR_INVALIDPARAM;
    }
    with_listener_mut(this, dw_apply, |dst| {
        *dst = Listener3dParams {
            position: p.v_position,
            velocity: p.v_velocity,
            orient_front: front,
            orient_top: top,
            distance_factor: p.fl_distance_factor,
            rolloff_factor: p.fl_rolloff_factor,
            doppler_factor: p.fl_doppler_factor,
        };
        DS_OK
    })
}

unsafe extern "win64" fn ds3dl_set_distance_factor(
    this: *mut c_void,
    factor: f32,
    dw_apply: u32,
) -> HRESULT {
    if !factor.is_finite() || factor <= 0.0 {
        return DSERR_INVALIDPARAM;
    }
    with_listener_mut(this, dw_apply, |dst| {
        dst.distance_factor = factor;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3dl_set_distance_factor(*mut c_void, f32, u32) -> HRESULT);

unsafe extern "win64" fn ds3dl_set_doppler_factor(
    this: *mut c_void,
    factor: f32,
    dw_apply: u32,
) -> HRESULT {
    if !factor.is_finite() || !(DS3D_MINDOPPLERFACTOR..=DS3D_MAXDOPPLERFACTOR).contains(&factor) {
        return DSERR_INVALIDPARAM;
    }
    with_listener_mut(this, dw_apply, |dst| {
        dst.doppler_factor = factor;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3dl_set_doppler_factor(*mut c_void, f32, u32) -> HRESULT);

unsafe extern "win64" fn ds3dl_set_orientation(
    this: *mut c_void,
    x_front: f32,
    y_front: f32,
    z_front: f32,
    x_top: f32,
    y_top: f32,
    z_top: f32,
    dw_apply: u32,
) -> HRESULT {
    let Some(front) = vec_normalize(D3dVector { x: x_front, y: y_front, z: z_front }) else {
        return DSERR_INVALIDPARAM;
    };
    let Some(top) = vec_normalize(D3dVector { x: x_top, y: y_top, z: z_top }) else {
        return DSERR_INVALIDPARAM;
    };
    if vec_normalize(vec_cross(top, front)).is_none() {
        return DSERR_INVALIDPARAM;
    }
    with_listener_mut(this, dw_apply, |dst| {
        dst.orient_front = front;
        dst.orient_top = top;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3dl_set_orientation(*mut c_void, f32, f32, f32, f32, f32, f32, u32) -> HRESULT);

unsafe extern "win64" fn ds3dl_set_position(
    this: *mut c_void,
    x: f32,
    y: f32,
    z: f32,
    dw_apply: u32,
) -> HRESULT {
    let v = D3dVector { x, y, z };
    if !vec_is_finite(v) {
        return DSERR_INVALIDPARAM;
    }
    with_listener_mut(this, dw_apply, |dst| {
        dst.position = v;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3dl_set_position(*mut c_void, f32, f32, f32, u32) -> HRESULT);

unsafe extern "win64" fn ds3dl_set_rolloff_factor(
    this: *mut c_void,
    factor: f32,
    dw_apply: u32,
) -> HRESULT {
    if !factor.is_finite() || !(DS3D_MINROLLOFFFACTOR..=DS3D_MAXROLLOFFFACTOR).contains(&factor) {
        return DSERR_INVALIDPARAM;
    }
    with_listener_mut(this, dw_apply, |dst| {
        dst.rolloff_factor = factor;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3dl_set_rolloff_factor(*mut c_void, f32, u32) -> HRESULT);

unsafe extern "win64" fn ds3dl_set_velocity(
    this: *mut c_void,
    x: f32,
    y: f32,
    z: f32,
    dw_apply: u32,
) -> HRESULT {
    let v = D3dVector { x, y, z };
    if !vec_is_finite(v) {
        return DSERR_INVALIDPARAM;
    }
    with_listener_mut(this, dw_apply, |dst| {
        dst.velocity = v;
        DS_OK
    })
}
crate::win64_bridge!(fn ds3dl_set_velocity(*mut c_void, f32, f32, f32, u32) -> HRESULT);

unsafe extern "win64" fn ds3dl_commit_deferred_settings(this: *mut c_void) -> HRESULT {
    let base = dsbuf_from_3d(this);
    if base.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*base;
    let mut st = obj.device.state.lock().unwrap();
    st.listener_current = st.listener_deferred;
    for buf in st.buffers.values_mut() {
        buf.d3d_current = buf.d3d_deferred;
    }
    DS_OK
}

// ============================================================================
// IDirectSoundNotify (Playback Buffer)
// ============================================================================

unsafe fn parse_notifies(
    count: u32,
    notifies: *const DsbPositionNotify,
    buf_len: u32,
) -> Result<Vec<NotifyEntry>, HRESULT> {
    if count == 0 {
        return Ok(Vec::new());
    }
    if notifies.is_null() {
        return Err(DSERR_INVALIDPARAM);
    }
    let mut out = Vec::with_capacity(count as usize);
    for i in 0..(count as usize) {
        let entry = std::ptr::read_unaligned(notifies.add(i));
        if entry.dw_offset != DSBPN_OFFSETSTOP && entry.dw_offset >= buf_len {
            return Err(DSERR_INVALIDPARAM);
        }
        let Some(ev) = crate::handles::get_as::<crate::handles::Event>(entry.h_event_notify) else {
            return Err(DSERR_INVALIDPARAM);
        };
        out.push(NotifyEntry { offset: entry.dw_offset, event: ev });
    }
    Ok(out)
}

unsafe extern "win64" fn dsn_query_interface(
    this: *mut c_void,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    dsbuf_qi_common(dsbuf_from_notify(this), riid, ppv)
}

unsafe extern "win64" fn dsn_add_ref(this: *mut c_void) -> u32 {
    dsbuf_add_ref_common(dsbuf_from_notify(this))
}

unsafe extern "win64" fn dsn_release(this: *mut c_void) -> u32 {
    dsbuf_release_common(dsbuf_from_notify(this))
}

unsafe extern "win64" fn dsn_set_notification_positions(
    this: *mut c_void,
    count: u32,
    notifies: *const DsbPositionNotify,
) -> HRESULT {
    let base = dsbuf_from_notify(this);
    if base.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*base;
    if obj.is_primary {
        return DSERR_INVALIDCALL;
    }
    let mut st = obj.device.state.lock().unwrap();
    let Some(b) = st.buffers.get_mut(&obj.buffer_id) else {
        return DSERR_INVALIDCALL;
    };
    if b.playing {
        return DSERR_INVALIDCALL;
    }
    let buf_len = b.sample_mem.byte_len() as u32;
    let parsed = match parse_notifies(count, notifies, buf_len) {
        Ok(v) => v,
        Err(hr) => return hr,
    };
    b.notifies = parsed;
    DS_OK
}

// ============================================================================
// IDirectSoundCapture / IDirectSoundCapture8 Methods
// ============================================================================

unsafe extern "win64" fn dsc_query_interface(
    this: *mut DsCaptureCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
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
    if iid == IID_IUnknown || iid == IID_IDIRECTSOUNDCAPTURE || iid == IID_IDIRECTSOUNDCAPTURE8 {
        let obj = &*this;
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        return DS_OK;
    }
    E_NOINTERFACE
}

unsafe extern "win64" fn dsc_add_ref(this: *mut DsCaptureCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn dsc_release(this: *mut DsCaptureCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let prev = {
        let obj = &*this;
        obj.refs.fetch_sub(1, Ordering::AcqRel)
    };
    if prev == 1 {
        ACTIVE_OBJECTS.fetch_sub(1, Ordering::AcqRel);
        drop(Box::from_raw(this));
        0
    } else {
        prev - 1
    }
}

unsafe fn create_capture_buffer_core(
    core: &Arc<DsCaptureCore>,
    desc_ptr: *const c_void,
    pp_buf: *mut *mut c_void,
    p_unk_outer: *mut c_void,
) -> HRESULT {
    if pp_buf.is_null() {
        return DSERR_INVALIDPARAM;
    }
    std::ptr::write_unaligned(pp_buf, std::ptr::null_mut());
    if !p_unk_outer.is_null() {
        return DSERR_NOAGGREGATION;
    }
    if desc_ptr.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if !core.initialized.load(Ordering::Acquire) {
        return DSERR_UNINITIALIZED;
    }

    let dw_size = std::ptr::read_unaligned(desc_ptr as *const u32);
    let (flags, buffer_bytes, reserved, wfx_ptr) = if dw_size == 24 {
        let d = std::ptr::read_unaligned(desc_ptr as *const DscBufferDesc1);
        (d.dw_flags, d.dw_buffer_bytes, d.dw_reserved, d.lpwfx_format)
    } else if dw_size == 40 {
        let d = std::ptr::read_unaligned(desc_ptr as *const DscBufferDesc);
        (d.dw_flags, d.dw_buffer_bytes, d.dw_reserved, d.lpwfx_format)
    } else {
        return DSERR_INVALIDPARAM;
    };

    if reserved != 0 || !(DSBSIZE_MIN..=DSBSIZE_MAX).contains(&buffer_bytes) {
        return DSERR_INVALIDPARAM;
    }
    let fmt = match DsFormat::parse(wfx_ptr) {
        Ok(f) => f,
        Err(hr) => return hr,
    };
    if (buffer_bytes as usize) < (fmt.block_align as usize) {
        return DSERR_INVALIDPARAM;
    }

    let sample_mem = SampleMemory::new(buffer_bytes as usize, fmt);
    let buf = alloc_ds_capture_buffer(core.clone(), flags, sample_mem);
    std::ptr::write_unaligned(pp_buf, buf as *mut c_void);
    DS_OK
}

unsafe extern "win64" fn dsc_create_capture_buffer(
    this: *mut DsCaptureCom,
    desc: *const c_void,
    pp_buf: *mut *mut c_void,
    p_unk_outer: *mut c_void,
) -> HRESULT {
    if this.is_null() {
        if !pp_buf.is_null() {
            std::ptr::write_unaligned(pp_buf, std::ptr::null_mut());
        }
        return DSERR_INVALIDPARAM;
    }
    let core = {
        let obj = &*this;
        obj.core.clone()
    };
    create_capture_buffer_core(&core, desc, pp_buf, p_unk_outer)
}

unsafe extern "win64" fn dsc_get_caps(this: *mut DsCaptureCom, caps: *mut DscCaps) -> HRESULT {
    if this.is_null() || caps.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if !obj.core.initialized.load(Ordering::Acquire) {
        return DSERR_UNINITIALIZED;
    }
    let sz = std::ptr::read_unaligned(caps as *const u32);
    if sz != std::mem::size_of::<DscCaps>() as u32 {
        return DSERR_INVALIDPARAM;
    }
    let ch = obj.core.device.lock().unwrap().channels.max(2);
    std::ptr::write_unaligned(
        caps,
        DscCaps {
            dw_size: sz,
            dw_flags: DSCCAPS_CERTIFIED | DSCCAPS_MULTIPLECAPTURE,
            dw_formats: STANDARD_CAPTURE_FORMATS,
            dw_channels: ch,
        },
    );
    DS_OK
}

unsafe extern "win64" fn dsc_initialize(
    this: *mut DsCaptureCom,
    guid_ptr: *const GUID,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if obj.core.initialized.load(Ordering::Acquire) {
        return DSERR_ALREADYINITIALIZED;
    }
    let guid_opt = GUID::read(guid_ptr);
    let dev = match resolve_capture_device(guid_opt) {
        Ok(d) => d,
        Err(hr) => return hr,
    };
    *obj.core.device.lock().unwrap() = dev;
    obj.core.initialized.store(true, Ordering::Release);
    DS_OK
}

// ============================================================================
// IDirectSoundCaptureBuffer / IDirectSoundCaptureBuffer8 + Notify
// ============================================================================

unsafe fn dscbuf_qi_common(
    base: *mut DsCaptureBufferCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
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
    if iid == IID_IUnknown
        || iid == IID_IDIRECTSOUNDCAPTUREBUFFER
        || iid == IID_IDIRECTSOUNDCAPTUREBUFFER8
    {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, base as *mut c_void);
        return DS_OK;
    }
    if iid == IID_IDIRECTSOUNDNOTIFY {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        let sub = (base as *mut u8).add(std::mem::offset_of!(DsCaptureBufferCom, vtbl_notify));
        std::ptr::write_unaligned(ppv, sub as *mut c_void);
        return DS_OK;
    }
    E_NOINTERFACE
}

unsafe fn dscbuf_add_ref_common(base: *mut DsCaptureBufferCom) -> u32 {
    if base.is_null() {
        return 0;
    }
    let obj = &*base;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe fn dscbuf_release_common(base: *mut DsCaptureBufferCom) -> u32 {
    if base.is_null() {
        return 0;
    }
    let prev = {
        let obj = &*base;
        obj.refs.fetch_sub(1, Ordering::AcqRel)
    };
    if prev == 1 {
        ACTIVE_OBJECTS.fetch_sub(1, Ordering::AcqRel);
        drop(Box::from_raw(base));
        0
    } else {
        prev - 1
    }
}

#[inline]
unsafe fn dscbuf_from_notify(this: *mut c_void) -> *mut DsCaptureBufferCom {
    if this.is_null() {
        std::ptr::null_mut()
    } else {
        (this as *mut u8).sub(std::mem::offset_of!(DsCaptureBufferCom, vtbl_notify))
            as *mut DsCaptureBufferCom
    }
}

unsafe extern "win64" fn dscb_query_interface(
    this: *mut DsCaptureBufferCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    dscbuf_qi_common(this, riid, ppv)
}

unsafe extern "win64" fn dscb_add_ref(this: *mut DsCaptureBufferCom) -> u32 {
    dscbuf_add_ref_common(this)
}

unsafe extern "win64" fn dscb_release(this: *mut DsCaptureBufferCom) -> u32 {
    dscbuf_release_common(this)
}

unsafe extern "win64" fn dscb_get_caps(
    this: *mut DsCaptureBufferCom,
    caps: *mut DscbCaps,
) -> HRESULT {
    if this.is_null() || caps.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let sz = std::ptr::read_unaligned(caps as *const u32);
    if sz != std::mem::size_of::<DscbCaps>() as u32 {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let st = obj.state.lock().unwrap();
    std::ptr::write_unaligned(
        caps,
        DscbCaps {
            dw_size: sz,
            dw_flags: st.flags,
            dw_buffer_bytes: st.sample_mem.byte_len() as u32,
            dw_reserved: 0,
        },
    );
    DS_OK
}

unsafe extern "win64" fn dscb_get_current_position(
    this: *mut DsCaptureBufferCom,
    cap_out: *mut u32,
    read_out: *mut u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let (cap_pos, read_pos, events) = {
        let mut st = obj.state.lock().unwrap();
        st.advance_and_cursors()
    };
    signal_events(events);
    if !cap_out.is_null() {
        std::ptr::write_unaligned(cap_out, cap_pos);
    }
    if !read_out.is_null() {
        std::ptr::write_unaligned(read_out, read_pos);
    }
    DS_OK
}

unsafe extern "win64" fn dscb_get_format(
    this: *mut DsCaptureBufferCom,
    wfx_out: *mut u8,
    allocated: u32,
    written_out: *mut u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let fmt = obj.state.lock().unwrap().sample_mem.format;
    fmt.write_to(wfx_out, allocated, written_out)
}

unsafe extern "win64" fn dscb_get_status(
    this: *mut DsCaptureBufferCom,
    status_out: *mut u32,
) -> HRESULT {
    if this.is_null() || status_out.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let (status, events) = {
        let mut st = obj.state.lock().unwrap();
        let (_, _, ev) = st.advance_and_cursors();
        let mut s = 0u32;
        if st.capturing {
            s |= DSCBSTATUS_CAPTURING;
            if st.looping {
                s |= DSCBSTATUS_LOOPING;
            }
        }
        (s, ev)
    };
    signal_events(events);
    std::ptr::write_unaligned(status_out, status);
    DS_OK
}

unsafe extern "win64" fn dscb_initialize(
    _this: *mut DsCaptureBufferCom,
    _dsc: *mut c_void,
    _desc: *const c_void,
) -> HRESULT {
    DSERR_ALREADYINITIALIZED
}

unsafe extern "win64" fn dscb_lock(
    this: *mut DsCaptureBufferCom,
    dw_offset: u32,
    dw_bytes: u32,
    ppv_ptr1: *mut *mut c_void,
    pdw_bytes1: *mut u32,
    ppv_ptr2: *mut *mut c_void,
    pdw_bytes2: *mut u32,
    dw_flags: u32,
) -> HRESULT {
    if !ppv_ptr1.is_null() {
        std::ptr::write_unaligned(ppv_ptr1, std::ptr::null_mut());
    }
    if !pdw_bytes1.is_null() {
        std::ptr::write_unaligned(pdw_bytes1, 0);
    }
    if !ppv_ptr2.is_null() {
        std::ptr::write_unaligned(ppv_ptr2, std::ptr::null_mut());
    }
    if !pdw_bytes2.is_null() {
        std::ptr::write_unaligned(pdw_bytes2, 0);
    }
    if this.is_null() || ppv_ptr1.is_null() || pdw_bytes1.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if ppv_ptr2.is_null() != pdw_bytes2.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let mut st = obj.state.lock().unwrap();
    let buf_len = st.sample_mem.byte_len() as u32;
    let bytes = if (dw_flags & DSCBLOCK_ENTIREBUFFER) != 0 {
        buf_len
    } else {
        dw_bytes
    };
    if dw_offset >= buf_len || bytes == 0 || bytes > buf_len {
        return DSERR_INVALIDPARAM;
    }
    let base = st.sample_mem.base_mut_ptr();
    let first_len = bytes.min(buf_len - dw_offset);
    let second_len = bytes - first_len;
    let ptr1 = base.add(dw_offset as usize) as *mut c_void;
    let (ptr2, actual_len2) = if second_len > 0 && !ppv_ptr2.is_null() {
        (base as *mut c_void, second_len)
    } else {
        (std::ptr::null_mut(), 0)
    };
    st.locks.push(ActiveLock {
        ptr1: ptr1 as usize,
        len1: first_len,
        ptr2: ptr2 as usize,
        len2: actual_len2,
    });
    std::ptr::write_unaligned(ppv_ptr1, ptr1);
    std::ptr::write_unaligned(pdw_bytes1, first_len);
    if !ppv_ptr2.is_null() {
        std::ptr::write_unaligned(ppv_ptr2, ptr2);
        std::ptr::write_unaligned(pdw_bytes2, actual_len2);
    }
    DS_OK
}

unsafe extern "win64" fn dscb_start(this: *mut DsCaptureBufferCom, dw_flags: u32) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let mut st = obj.state.lock().unwrap();
    st.capturing = true;
    st.looping = (dw_flags & DSCBSTART_LOOPING) != 0;
    st.start_instant = Some(Instant::now());
    DS_OK
}

unsafe extern "win64" fn dscb_stop(this: *mut DsCaptureBufferCom) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let events = {
        let mut st = obj.state.lock().unwrap();
        let (_, _, mut ev) = st.advance_and_cursors();
        if st.capturing {
            st.capturing = false;
            st.looping = false;
            st.start_instant = None;
            for n in &st.notifies {
                if n.offset == DSBPN_OFFSETSTOP {
                    ev.push(n.event.clone());
                }
            }
        }
        ev
    };
    signal_events(events);
    DS_OK
}

unsafe extern "win64" fn dscb_unlock(
    this: *mut DsCaptureBufferCom,
    pv_ptr1: *mut c_void,
    dw_bytes1: u32,
    pv_ptr2: *mut c_void,
    dw_bytes2: u32,
) -> HRESULT {
    if this.is_null() || pv_ptr1.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    let mut st = obj.state.lock().unwrap();
    let pos = st.locks.iter().position(|lk| {
        lk.ptr1 == (pv_ptr1 as usize)
            && dw_bytes1 <= lk.len1
            && (dw_bytes2 == 0 || (lk.ptr2 == (pv_ptr2 as usize) && dw_bytes2 <= lk.len2))
    });
    let Some(idx) = pos else {
        return DSERR_INVALIDPARAM;
    };
    st.locks.swap_remove(idx);
    std::sync::atomic::fence(Ordering::Release);
    DS_OK
}

unsafe extern "win64" fn dscb_get_object_in_path(
    this: *mut DsCaptureBufferCom,
    _rguid_object: *const GUID,
    _dw_index: u32,
    _rguid_interface: *const GUID,
    pp_object: *mut *mut c_void,
) -> HRESULT {
    if !pp_object.is_null() {
        std::ptr::write_unaligned(pp_object, std::ptr::null_mut());
    }
    if this.is_null() || pp_object.is_null() {
        return DSERR_INVALIDPARAM;
    }
    DSERR_OBJECTNOTFOUND
}

unsafe extern "win64" fn dscb_get_fx_status(
    this: *mut DsCaptureBufferCom,
    dw_fx_count: u32,
    pdw_fx_status: *mut u32,
) -> HRESULT {
    if this.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if dw_fx_count == 0 {
        return DS_OK;
    }
    if pdw_fx_status.is_null() {
        return DSERR_INVALIDPARAM;
    }
    DSERR_INVALIDPARAM
}

unsafe extern "win64" fn dscn_query_interface(
    this: *mut c_void,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    dscbuf_qi_common(dscbuf_from_notify(this), riid, ppv)
}

unsafe extern "win64" fn dscn_add_ref(this: *mut c_void) -> u32 {
    dscbuf_add_ref_common(dscbuf_from_notify(this))
}

unsafe extern "win64" fn dscn_release(this: *mut c_void) -> u32 {
    dscbuf_release_common(dscbuf_from_notify(this))
}

unsafe extern "win64" fn dscn_set_notification_positions(
    this: *mut c_void,
    count: u32,
    notifies: *const DsbPositionNotify,
) -> HRESULT {
    let base = dscbuf_from_notify(this);
    if base.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*base;
    let mut st = obj.state.lock().unwrap();
    if st.capturing {
        return DSERR_INVALIDCALL;
    }
    let buf_len = st.sample_mem.byte_len() as u32;
    let parsed = match parse_notifies(count, notifies, buf_len) {
        Ok(v) => v,
        Err(hr) => return hr,
    };
    st.notifies = parsed;
    DS_OK
}

// ============================================================================
// IDirectSoundFullDuplex Methods
// ============================================================================

unsafe extern "win64" fn dsfd_query_interface(
    this: *mut DsFullDuplexCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
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
    if iid == IID_IUnknown || iid == IID_IDIRECTSOUNDFULLDUPLEX {
        let obj = &*this;
        obj.refs.fetch_add(1, Ordering::AcqRel);
        std::ptr::write_unaligned(ppv, this as *mut c_void);
        return DS_OK;
    }
    E_NOINTERFACE
}

unsafe extern "win64" fn dsfd_add_ref(this: *mut DsFullDuplexCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

unsafe extern "win64" fn dsfd_release(this: *mut DsFullDuplexCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let prev = {
        let obj = &*this;
        obj.refs.fetch_sub(1, Ordering::AcqRel)
    };
    if prev == 1 {
        ACTIVE_OBJECTS.fetch_sub(1, Ordering::AcqRel);
        drop(Box::from_raw(this));
        0
    } else {
        prev - 1
    }
}

unsafe extern "win64" fn dsfd_initialize(
    this: *mut DsFullDuplexCom,
    capture_guid: *const GUID,
    render_guid: *const GUID,
    dsc_desc: *const c_void,
    ds_desc: *const c_void,
    _hwnd: HWND,
    dw_level: u32,
    pp_dscb8: *mut *mut c_void,
    pp_dsb8: *mut *mut c_void,
) -> HRESULT {
    if !pp_dscb8.is_null() {
        std::ptr::write_unaligned(pp_dscb8, std::ptr::null_mut());
    }
    if !pp_dsb8.is_null() {
        std::ptr::write_unaligned(pp_dsb8, std::ptr::null_mut());
    }
    if this.is_null()
        || dsc_desc.is_null()
        || ds_desc.is_null()
        || pp_dscb8.is_null()
        || pp_dsb8.is_null()
    {
        return DSERR_INVALIDPARAM;
    }
    if !(DSSCL_NORMAL..=DSSCL_WRITEPRIMARY).contains(&dw_level) {
        return DSERR_INVALIDPARAM;
    }
    let obj = &*this;
    if obj.initialized.load(Ordering::Acquire) {
        return DSERR_ALREADYINITIALIZED;
    }

    let cap_dev = match resolve_capture_device(GUID::read(capture_guid)) {
        Ok(d) => d,
        Err(hr) => return hr,
    };
    let ren_dev = match resolve_render_device(GUID::read(render_guid)) {
        Ok(d) => d,
        Err(hr) => return hr,
    };

    let cap_core = DsCaptureCore::new(cap_dev, true);
    let ren_core = DsDeviceCore::new(ren_dev, true, true);
    ren_core.state.lock().unwrap().cooperative_level = dw_level;

    let mut raw_cap_buf: *mut c_void = std::ptr::null_mut();
    let hr_cap = create_capture_buffer_core(&cap_core, dsc_desc, &mut raw_cap_buf, std::ptr::null_mut());
    if hr_cap != DS_OK {
        return hr_cap;
    }

    let mut raw_ren_buf: *mut c_void = std::ptr::null_mut();
    let hr_ren = create_sound_buffer_core(&ren_core, ds_desc, &mut raw_ren_buf, std::ptr::null_mut());
    if hr_ren != DS_OK {
        dscb_release(raw_cap_buf as *mut DsCaptureBufferCom);
        return hr_ren;
    }

    let mut dscb8: *mut c_void = std::ptr::null_mut();
    let hr_qi_cap = dscb_query_interface(
        raw_cap_buf as *mut DsCaptureBufferCom,
        &IID_IDIRECTSOUNDCAPTUREBUFFER8,
        &mut dscb8,
    );
    dscb_release(raw_cap_buf as *mut DsCaptureBufferCom);
    if hr_qi_cap != DS_OK {
        dsb_release(raw_ren_buf as *mut DsBufferCom);
        return hr_qi_cap;
    }

    let mut dsb8: *mut c_void = std::ptr::null_mut();
    let hr_qi_ren = dsb_query_interface(
        raw_ren_buf as *mut DsBufferCom,
        &IID_IDIRECTSOUNDBUFFER8,
        &mut dsb8,
    );
    dsb_release(raw_ren_buf as *mut DsBufferCom);
    if hr_qi_ren != DS_OK {
        dscb_release(dscb8 as *mut DsCaptureBufferCom);
        return hr_qi_ren;
    }

    *obj.capture.lock().unwrap() = Some(cap_core);
    *obj.render.lock().unwrap() = Some(ren_core);
    obj.initialized.store(true, Ordering::Release);

    std::ptr::write_unaligned(pp_dscb8, dscb8);
    std::ptr::write_unaligned(pp_dsb8, dsb8);
    DS_OK
}

// ============================================================================
// Device Enumeration Helper
// ============================================================================

unsafe fn enumerate_devices_impl(
    cb: *const c_void,
    ctx: *mut c_void,
    capture: bool,
    wide: bool,
) -> HRESULT {
    if cb.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let cb_addr = cb as usize;
    let ctx_u64 = ctx as usize as u64;
    let primary_desc = if capture {
        "Primary Sound Capture Driver"
    } else {
        "Primary Sound Driver"
    };

    let invoke_one = |guid_ptr: u64, desc: &str, module: &str| -> bool {
        let ret = if wide {
            let desc_w = crate::strings::to_wide_z(desc);
            let mod_w = crate::strings::to_wide_z(module);
            crate::call_guest8(
                cb_addr,
                guid_ptr,
                desc_w.as_ptr() as usize as u64,
                mod_w.as_ptr() as usize as u64,
                ctx_u64,
                0,
                0,
                0,
                0,
            )
        } else {
            let mut desc_a = desc.as_bytes().to_vec();
            desc_a.push(0);
            let mut mod_a = module.as_bytes().to_vec();
            mod_a.push(0);
            crate::call_guest8(
                cb_addr,
                guid_ptr,
                desc_a.as_ptr() as usize as u64,
                mod_a.as_ptr() as usize as u64,
                ctx_u64,
                0,
                0,
                0,
                0,
            )
        };
        (ret as u32) != 0
    };

    // First entry is always the Primary Driver with lpGuid == NULL.
    if !invoke_one(0, primary_desc, "") {
        return DS_OK;
    }

    let devices = if capture {
        backend::input_devices()
    } else {
        backend::output_devices()
    };

    for dev in devices {
        let guid = device_guid_for(&dev.uid, capture);
        if !invoke_one(&guid as *const GUID as usize as u64, &dev.name, &dev.uid) {
            break;
        }
    }
    DS_OK
}

// ============================================================================
// Exported DLL Entry Points (Ordinals 1..12)
// ============================================================================

pub unsafe extern "win64" fn DirectSoundCreate(
    guid: *const u8,
    pp_ds: *mut *mut c_void,
    unk: *mut c_void,
) -> HRESULT {
    if !pp_ds.is_null() {
        std::ptr::write_unaligned(pp_ds, std::ptr::null_mut());
    }
    if pp_ds.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if !unk.is_null() {
        return DSERR_NOAGGREGATION;
    }
    let dev = match resolve_render_device(GUID::read(guid as *const GUID)) {
        Ok(d) => d,
        Err(hr) => return hr,
    };
    let obj = alloc_ds_device(dev, true, false);
    std::ptr::write_unaligned(pp_ds, obj as *mut c_void);
    DS_OK
}

pub unsafe extern "win64" fn DirectSoundCreate8(
    guid: *const u8,
    pp_ds8: *mut *mut c_void,
    unk: *mut c_void,
) -> HRESULT {
    if !pp_ds8.is_null() {
        std::ptr::write_unaligned(pp_ds8, std::ptr::null_mut());
    }
    if pp_ds8.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if !unk.is_null() {
        return DSERR_NOAGGREGATION;
    }
    let dev = match resolve_render_device(GUID::read(guid as *const GUID)) {
        Ok(d) => d,
        Err(hr) => return hr,
    };
    let obj = alloc_ds_device(dev, true, true);
    std::ptr::write_unaligned(pp_ds8, obj as *mut c_void);
    DS_OK
}

pub unsafe extern "win64" fn DirectSoundEnumerateA(
    cb: *const c_void,
    ctx: *mut c_void,
) -> HRESULT {
    enumerate_devices_impl(cb, ctx, false, false)
}

pub unsafe extern "win64" fn DirectSoundEnumerateW(
    cb: *const c_void,
    ctx: *mut c_void,
) -> HRESULT {
    enumerate_devices_impl(cb, ctx, false, true)
}

pub unsafe extern "win64" fn DirectSoundCaptureCreate(
    guid: *const u8,
    pp_dsc: *mut *mut c_void,
    unk: *mut c_void,
) -> HRESULT {
    if !pp_dsc.is_null() {
        std::ptr::write_unaligned(pp_dsc, std::ptr::null_mut());
    }
    if pp_dsc.is_null() {
        return DSERR_INVALIDPARAM;
    }
    if !unk.is_null() {
        return DSERR_NOAGGREGATION;
    }
    let dev = match resolve_capture_device(GUID::read(guid as *const GUID)) {
        Ok(d) => d,
        Err(hr) => return hr,
    };
    let obj = alloc_ds_capture(dev, true);
    std::ptr::write_unaligned(pp_dsc, obj as *mut c_void);
    DS_OK
}

pub unsafe extern "win64" fn DirectSoundCaptureCreate8(
    guid: *const u8,
    pp_dsc8: *mut *mut c_void,
    unk: *mut c_void,
) -> HRESULT {
    DirectSoundCaptureCreate(guid, pp_dsc8, unk)
}

pub unsafe extern "win64" fn DirectSoundCaptureEnumerateA(
    cb: *const c_void,
    ctx: *mut c_void,
) -> HRESULT {
    enumerate_devices_impl(cb, ctx, true, false)
}

pub unsafe extern "win64" fn DirectSoundCaptureEnumerateW(
    cb: *const c_void,
    ctx: *mut c_void,
) -> HRESULT {
    enumerate_devices_impl(cb, ctx, true, true)
}

pub unsafe extern "win64" fn DirectSoundFullDuplexCreate(
    capture_guid: *const u8,
    render_guid: *const u8,
    dsc_desc: *const c_void,
    ds_desc: *const c_void,
    hwnd: HWND,
    level: u32,
    pp_dsfd: *mut *mut c_void,
    pp_dscb8: *mut *mut c_void,
    pp_dsb8: *mut *mut c_void,
    unk: *mut c_void,
) -> HRESULT {
    for p in [pp_dsfd, pp_dscb8, pp_dsb8] {
        if !p.is_null() {
            std::ptr::write_unaligned(p, std::ptr::null_mut());
        }
    }
    if !unk.is_null() {
        return DSERR_NOAGGREGATION;
    }
    if pp_dsfd.is_null() || pp_dscb8.is_null() || pp_dsb8.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let fd = alloc_ds_full_duplex();
    let hr = dsfd_initialize(
        fd,
        capture_guid as *const GUID,
        render_guid as *const GUID,
        dsc_desc,
        ds_desc,
        hwnd,
        level,
        pp_dscb8,
        pp_dsb8,
    );
    if hr != DS_OK {
        dsfd_release(fd);
        return hr;
    }
    std::ptr::write_unaligned(pp_dsfd, fd as *mut c_void);
    DS_OK
}

pub unsafe extern "win64" fn GetDeviceID(src: *const u8, dst: *mut u8) -> HRESULT {
    if src.is_null() || dst.is_null() {
        return DSERR_INVALIDPARAM;
    }
    let g = std::ptr::read_unaligned(src as *const GUID);
    if g == GUID_NULL {
        return DSERR_INVALIDPARAM;
    }
    if g == DSDEVID_DEFAULTPLAYBACK || g == DSDEVID_DEFAULTVOICEPLAYBACK {
        let devs = backend::output_devices();
        let dev = devs.first().cloned().unwrap_or_else(backend::null_device);
        std::ptr::write_unaligned(dst as *mut GUID, device_guid_for(&dev.uid, false));
        return DS_OK;
    }
    if g == DSDEVID_DEFAULTCAPTURE || g == DSDEVID_DEFAULTVOICECAPTURE {
        let devs = backend::input_devices();
        let dev = devs.first().cloned().unwrap_or_else(backend::null_capture_device);
        std::ptr::write_unaligned(dst as *mut GUID, device_guid_for(&dev.uid, true));
        return DS_OK;
    }
    for dev in backend::output_devices() {
        let dg = device_guid_for(&dev.uid, false);
        if dg == g {
            std::ptr::write_unaligned(dst as *mut GUID, dg);
            return DS_OK;
        }
    }
    for dev in backend::input_devices() {
        let dg = device_guid_for(&dev.uid, true);
        if dg == g {
            std::ptr::write_unaligned(dst as *mut GUID, dg);
            return DS_OK;
        }
    }
    DSERR_NODRIVER
}

pub unsafe extern "win64" fn DllGetClassObject(
    rclsid: *const u8,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    let Some(clsid) = GUID::read(rclsid as *const GUID) else {
        return E_POINTER;
    };
    let Some(iid) = GUID::read(riid as *const GUID) else {
        return E_POINTER;
    };
    let kind = if clsid == CLSID_DIRECTSOUND {
        FactoryKind::DirectSound
    } else if clsid == CLSID_DIRECTSOUND8 {
        FactoryKind::DirectSound8
    } else if clsid == CLSID_DIRECTSOUNDCAPTURE {
        FactoryKind::DirectSoundCapture
    } else if clsid == CLSID_DIRECTSOUNDCAPTURE8 {
        FactoryKind::DirectSoundCapture8
    } else if clsid == CLSID_DIRECTSOUNDFULLDUPLEX {
        FactoryKind::DirectSoundFullDuplex
    } else {
        return CLASS_E_CLASSNOTAVAILABLE;
    };

    let cf = alloc_class_factory(kind);
    let hr = cf_query_interface(cf, &iid, ppv);
    cf_release(cf);
    hr
}

pub unsafe extern "win64" fn DllCanUnloadNow() -> HRESULT {
    if ACTIVE_OBJECTS.load(Ordering::Acquire) == 0 && SERVER_LOCKS.load(Ordering::Acquire) == 0 {
        S_OK
    } else {
        S_FALSE
    }
}

// ============================================================================
// Unit Tests (must remain at bottom for win32-rt/build.rs export scanner)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn pcm_wfx(channels: u16, sample_rate: u32, bits: u16) -> WaveFormatEx {
        let align = channels * (bits / 8);
        WaveFormatEx {
            format_tag: WAVE_FORMAT_PCM,
            channels,
            samples_per_sec: sample_rate,
            avg_bytes_per_sec: sample_rate * (align as u32),
            block_align: align,
            bits_per_sample: bits,
            cb_size: 0,
        }
    }

    fn float_wfx(channels: u16, sample_rate: u32) -> WaveFormatEx {
        let align = channels * 4;
        WaveFormatEx {
            format_tag: WAVE_FORMAT_IEEE_FLOAT,
            channels,
            samples_per_sec: sample_rate,
            avg_bytes_per_sec: sample_rate * (align as u32),
            block_align: align,
            bits_per_sample: 32,
            cb_size: 0,
        }
    }

    fn make_desc(flags: u32, buffer_bytes: u32, wfx: *const u8) -> DsBufferDesc {
        DsBufferDesc {
            dw_size: std::mem::size_of::<DsBufferDesc>() as u32,
            dw_flags: flags,
            dw_buffer_bytes: buffer_bytes,
            dw_reserved: 0,
            lpwfx_format: wfx,
            guid_3d_algorithm: GUID_NULL,
        }
    }

    fn make_cap_desc(flags: u32, buffer_bytes: u32, wfx: *const u8) -> DscBufferDesc {
        DscBufferDesc {
            dw_size: std::mem::size_of::<DscBufferDesc>() as u32,
            dw_flags: flags,
            dw_buffer_bytes: buffer_bytes,
            dw_reserved: 0,
            lpwfx_format: wfx,
            dw_fx_count: 0,
            lp_dsc_fx_desc: std::ptr::null(),
        }
    }

    unsafe fn make_paused_ds8() -> *mut DsDeviceCom {
        let mut raw: *mut c_void = std::ptr::null_mut();
        assert_eq!(DirectSoundCreate8(std::ptr::null(), &mut raw, std::ptr::null_mut()), DS_OK);
        assert!(!raw.is_null());
        let ds = raw as *mut DsDeviceCom;
        (&*ds).core.mixer_paused.store(true, Ordering::Release);
        assert_eq!(ds_set_cooperative_level(ds, std::ptr::null_mut(), DSSCL_PRIORITY), DS_OK);
        ds
    }

    unsafe fn write_all_bytes(buf: *mut DsBufferCom, data: &[u8]) {
        let mut p1: *mut c_void = std::ptr::null_mut();
        let mut l1 = 0u32;
        let mut p2: *mut c_void = std::ptr::null_mut();
        let mut l2 = 0u32;
        assert_eq!(
            dsb_lock(
                buf,
                0,
                data.len() as u32,
                &mut p1,
                &mut l1,
                &mut p2,
                &mut l2,
                DSBLOCK_ENTIREBUFFER,
            ),
            DS_OK
        );
        assert_eq!(l1 as usize, data.len());
        assert_eq!(l2, 0);
        std::ptr::copy_nonoverlapping(data.as_ptr(), p1 as *mut u8, data.len());
        assert_eq!(dsb_unlock(buf, p1, l1, p2, l2), DS_OK);
    }

    #[test]
    fn ordinals_and_class_factory_lifecycle() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        assert_eq!(ORDINALS.len(), 12);
        for (idx, &(ord, _)) in ORDINALS.iter().enumerate() {
            assert_eq!(ord as usize, idx + 1);
        }

        unsafe {
            // Unknown CLSID fails cleanly.
            let mut bad: *mut c_void = 0x1234 as *mut c_void;
            assert_eq!(
                DllGetClassObject(
                    &GUID_NULL as *const GUID as *const u8,
                    &IID_ICLASSFACTORY as *const GUID as *const u8,
                    &mut bad,
                ),
                CLASS_E_CLASSNOTAVAILABLE
            );
            assert!(bad.is_null());

            // CLSID_DirectSound8 -> IClassFactory -> uninitialized IDirectSound8.
            let mut cf_raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                DllGetClassObject(
                    &CLSID_DIRECTSOUND8 as *const GUID as *const u8,
                    &IID_ICLASSFACTORY as *const GUID as *const u8,
                    &mut cf_raw,
                ),
                DS_OK
            );
            assert_eq!(DllCanUnloadNow(), S_FALSE);

            let cf = cf_raw as *mut ClassFactoryCom;
            let mut ds8_raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                cf_create_instance(cf, std::ptr::null_mut(), &IID_IDIRECTSOUND8, &mut ds8_raw),
                DS_OK
            );
            let ds8 = ds8_raw as *mut DsDeviceCom;

            // Calling methods before Initialize returns DSERR_UNINITIALIZED.
            let mut caps = DsCaps { dw_size: std::mem::size_of::<DsCaps>() as u32, ..DsCaps::default() };
            assert_eq!(ds_get_caps(ds8, &mut caps), DSERR_UNINITIALIZED);
            assert_eq!(ds_initialize(ds8, std::ptr::null()), DS_OK);
            assert_eq!(ds_initialize(ds8, std::ptr::null()), DSERR_ALREADYINITIALIZED);
            assert_eq!(ds_get_caps(ds8, &mut caps), DS_OK);
            assert_eq!(caps.dw_primary_buffers, 1);

            let mut cert = 99u32;
            assert_eq!(ds_verify_certification(ds8, &mut cert), DS_OK);
            assert_eq!(cert, DS_CERTIFIED);

            // LockServer increments/decrements server locks cleanly.
            assert_eq!(cf_lock_server(cf, TRUE), DS_OK);
            assert_eq!(ds_release(ds8), 0);
            assert_eq!(cf_release(cf), 0);
            assert_eq!(DllCanUnloadNow(), S_FALSE);
            assert_eq!(cf_lock_server(std::ptr::null_mut(), FALSE), DS_OK);
            assert_eq!(DllCanUnloadNow(), S_OK);
        }
    }

    #[test]
    fn enumerate_devices_and_get_device_id() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));
        backend::force_output_devices(Some(vec![
            DeviceInfo {
                id: 10,
                uid: "test-out-1".into(),
                name: "Studio Monitors".into(),
                rate: 48_000,
                channels: 2,
                io_frames: 256,
                latency_frames: 256,
                headphones: false,
                is_default: true,
            },
            DeviceInfo {
                id: 11,
                uid: "test-out-2".into(),
                name: "USB Headphones".into(),
                rate: 48_000,
                channels: 2,
                io_frames: 256,
                latency_frames: 256,
                headphones: true,
                is_default: false,
            },
        ]));
        backend::force_input_devices(Some(vec![DeviceInfo {
            id: 20,
            uid: "test-in-1".into(),
            name: "Studio Mic".into(),
            rate: 48_000,
            channels: 1,
            io_frames: 256,
            latency_frames: 256,
            headphones: false,
            is_default: true,
        }]));

        struct EnumRec {
            entries: Vec<(Option<GUID>, String, String)>,
            stop_after: usize,
        }

        unsafe extern "win64" fn enum_cb_a(
            lp_guid: *const GUID,
            desc: *const u8,
            module: *const u8,
            ctx: *mut c_void,
        ) -> BOOL {
            let rec = &mut *(ctx as *mut EnumRec);
            rec.entries.push((
                GUID::read(lp_guid),
                crate::strings::from_ansi(desc),
                crate::strings::from_ansi(module),
            ));
            if rec.entries.len() >= rec.stop_after { FALSE } else { TRUE }
        }

        unsafe extern "win64" fn enum_cb_w(
            lp_guid: *const GUID,
            desc: *const u16,
            module: *const u16,
            ctx: *mut c_void,
        ) -> BOOL {
            let rec = &mut *(ctx as *mut EnumRec);
            rec.entries.push((
                GUID::read(lp_guid),
                crate::strings::from_wide(desc),
                crate::strings::from_wide(module),
            ));
            if rec.entries.len() >= rec.stop_after { FALSE } else { TRUE }
        }

        unsafe {
            // Render ANSI enumeration (all 3 entries: Primary + 2 output devices).
            let mut rec_a = EnumRec { entries: Vec::new(), stop_after: 99 };
            assert_eq!(
                DirectSoundEnumerateA(
                    enum_cb_a as *const c_void,
                    &mut rec_a as *mut EnumRec as *mut c_void,
                ),
                DS_OK
            );
            assert_eq!(rec_a.entries.len(), 3);
            assert!(rec_a.entries[0].0.is_none());
            assert_eq!(rec_a.entries[0].1, "Primary Sound Driver");
            assert_eq!(rec_a.entries[1].1, "Studio Monitors");
            assert_eq!(rec_a.entries[1].2, "test-out-1");
            assert_eq!(rec_a.entries[2].1, "USB Headphones");

            // Render Wide enumeration with early stop after 2 entries.
            let mut rec_w = EnumRec { entries: Vec::new(), stop_after: 2 };
            assert_eq!(
                DirectSoundEnumerateW(
                    enum_cb_w as *const c_void,
                    &mut rec_w as *mut EnumRec as *mut c_void,
                ),
                DS_OK
            );
            assert_eq!(rec_w.entries.len(), 2);
            assert_eq!(rec_w.entries[1].0, rec_a.entries[1].0);

            // Capture ANSI & Wide enumeration.
            let mut cap_a = EnumRec { entries: Vec::new(), stop_after: 99 };
            assert_eq!(
                DirectSoundCaptureEnumerateA(
                    enum_cb_a as *const c_void,
                    &mut cap_a as *mut EnumRec as *mut c_void,
                ),
                DS_OK
            );
            assert_eq!(cap_a.entries.len(), 2);
            assert!(cap_a.entries[0].0.is_none());
            assert_eq!(cap_a.entries[0].1, "Primary Sound Capture Driver");
            assert_eq!(cap_a.entries[1].1, "Studio Mic");

            let mut cap_w = EnumRec { entries: Vec::new(), stop_after: 99 };
            assert_eq!(
                DirectSoundCaptureEnumerateW(
                    enum_cb_w as *const c_void,
                    &mut cap_w as *mut EnumRec as *mut c_void,
                ),
                DS_OK
            );
            assert_eq!(cap_w.entries.len(), 2);
            assert_eq!(cap_w.entries[1].0, cap_a.entries[1].0);

            // GetDeviceID resolves default playback, default capture, specific GUIDs, and rejects unknown.
            let mut out_guid = GUID_NULL;
            assert_eq!(
                GetDeviceID(
                    &DSDEVID_DEFAULTPLAYBACK as *const GUID as *const u8,
                    &mut out_guid as *mut GUID as *mut u8,
                ),
                DS_OK
            );
            assert_eq!(Some(out_guid), rec_a.entries[1].0);

            let mut cap_guid = GUID_NULL;
            assert_eq!(
                GetDeviceID(
                    &DSDEVID_DEFAULTCAPTURE as *const GUID as *const u8,
                    &mut cap_guid as *mut GUID as *mut u8,
                ),
                DS_OK
            );
            assert_eq!(Some(cap_guid), cap_a.entries[1].0);

            let second_guid = rec_a.entries[2].0.unwrap();
            let mut echo_guid = GUID_NULL;
            assert_eq!(
                GetDeviceID(
                    &second_guid as *const GUID as *const u8,
                    &mut echo_guid as *mut GUID as *mut u8,
                ),
                DS_OK
            );
            assert_eq!(echo_guid, second_guid);

            let unknown = GUID::new(0x1111_2222, 0x3333, 0x4444, [5; 8]);
            assert_eq!(
                GetDeviceID(
                    &unknown as *const GUID as *const u8,
                    &mut echo_guid as *mut GUID as *mut u8,
                ),
                DSERR_NODRIVER
            );

            // Opening DirectSound8 on an enumerated specific device GUID succeeds.
            let mut ds_raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                DirectSoundCreate8(
                    &second_guid as *const GUID as *const u8,
                    &mut ds_raw,
                    std::ptr::null_mut(),
                ),
                DS_OK
            );
            assert_eq!(ds_release(ds_raw as *mut DsDeviceCom), 0);
        }
    }

    #[test]
    fn circular_lock_unlock_wrap_around() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let ds = make_paused_ds8();
            let wfx = pcm_wfx(1, 48_000, 16);
            let desc = make_desc(0, 64, &wfx as *const WaveFormatEx as *const u8);
            let mut buf_raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                ds_create_sound_buffer(
                    ds,
                    &desc as *const DsBufferDesc as *const c_void,
                    &mut buf_raw,
                    std::ptr::null_mut(),
                ),
                DS_OK
            );
            let buf = buf_raw as *mut DsBufferCom;

            // Wrap-around lock: offset 56, length 24 -> 8 bytes at [56..64], 16 bytes at [0..16].
            let mut p1: *mut c_void = std::ptr::null_mut();
            let mut l1 = 0u32;
            let mut p2: *mut c_void = std::ptr::null_mut();
            let mut l2 = 0u32;
            assert_eq!(dsb_lock(buf, 56, 24, &mut p1, &mut l1, &mut p2, &mut l2, 0), DS_OK);
            assert_eq!(l1, 8);
            assert_eq!(l2, 16);
            assert!(!p1.is_null() && !p2.is_null());
            std::slice::from_raw_parts_mut(p1 as *mut u8, 8).fill(0xAA);
            std::slice::from_raw_parts_mut(p2 as *mut u8, 16).fill(0x55);
            assert_eq!(dsb_unlock(buf, p1, 8, p2, 16), DS_OK);

            // Unlocking an already-unlocked pointer fails cleanly.
            assert_eq!(dsb_unlock(buf, p1, 8, p2, 16), DSERR_INVALIDPARAM);

            // Entire-buffer lock verifies exact wrapped layout.
            assert_eq!(
                dsb_lock(
                    buf,
                    0,
                    0,
                    &mut p1,
                    &mut l1,
                    &mut p2,
                    &mut l2,
                    DSBLOCK_ENTIREBUFFER,
                ),
                DS_OK
            );
            assert_eq!(l1, 64);
            assert_eq!(l2, 0);
            assert!(p2.is_null());
            let all = std::slice::from_raw_parts(p1 as *const u8, 64);
            assert!(all[0..16].iter().all(|&b| b == 0x55));
            assert!(all[16..56].iter().all(|&b| b == 0x00));
            assert!(all[56..64].iter().all(|&b| b == 0xAA));
            assert_eq!(dsb_unlock(buf, p1, 64, std::ptr::null_mut(), 0), DS_OK);

            // Out-of-bounds offset/size rejected.
            assert_eq!(dsb_lock(buf, 64, 4, &mut p1, &mut l1, &mut p2, &mut l2, 0), DSERR_INVALIDPARAM);
            assert_eq!(dsb_lock(buf, 0, 68, &mut p1, &mut l1, &mut p2, &mut l2, 0), DSERR_INVALIDPARAM);

            assert_eq!(dsb_release(buf), 0);
            assert_eq!(ds_release(ds), 0);
        }
    }

    #[test]
    fn playback_waveforms_pcm8_16_24_32_float_oneshot_and_looping() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let ds = make_paused_ds8();

            // Helper to test a single mono 4-frame waveform across formats.
            let verify_format = |wfx_bytes: &[u8], payload: &[u8], expected: [f32; 4], tol: f32| {
                let desc = make_desc(0, payload.len() as u32, wfx_bytes.as_ptr());
                let mut raw: *mut c_void = std::ptr::null_mut();
                assert_eq!(
                    ds_create_sound_buffer(
                        ds,
                        &desc as *const DsBufferDesc as *const c_void,
                        &mut raw,
                        std::ptr::null_mut(),
                    ),
                    DS_OK
                );
                let buf = raw as *mut DsBufferCom;
                write_all_bytes(buf, payload);
                assert_eq!(dsb_play(buf, 0, 0, 0), DS_OK);

                let mut status = 0u32;
                assert_eq!(dsb_get_status(buf, &mut status), DS_OK);
                assert_eq!(status, DSBSTATUS_PLAYING);

                // Mix 6 frames: 4 frames of signal + 2 trailing silence after one-shot stops.
                let mut out = [0.0f32; 12];
                (&*ds).core.mix_frames(6, &mut out);
                for i in 0..4 {
                    assert!(
                        (out[i * 2] - expected[i]).abs() <= tol,
                        "frame {i}: got {}, want {}",
                        out[i * 2],
                        expected[i]
                    );
                    assert!((out[i * 2 + 1] - expected[i]).abs() <= tol);
                }
                assert_eq!(&out[8..12], &[0.0; 4]);
                assert_eq!(dsb_get_status(buf, &mut status), DS_OK);
                assert_eq!(status, 0);
                assert_eq!(dsb_release(buf), 0);
            };

            // 1. PCM 8-bit unsigned: [128 -> 0.0, 192 -> 0.5, 64 -> -0.5, 0 -> -1.0]
            let w8 = pcm_wfx(1, 48_000, 8);
            let w8_bytes = std::slice::from_raw_parts(&w8 as *const _ as *const u8, 18);
            verify_format(w8_bytes, &[128, 192, 64, 0], [0.0, 0.5, -0.5, -1.0], 1e-4);

            // 2. PCM 16-bit signed LE
            let w16 = pcm_wfx(1, 48_000, 16);
            let w16_bytes = std::slice::from_raw_parts(&w16 as *const _ as *const u8, 18);
            let p16: Vec<u8> = [0i16, 16384, -16384, -32768]
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect();
            verify_format(w16_bytes, &p16, [0.0, 0.5, -0.5, -1.0], 1e-4);

            // 3. PCM 24-bit signed packed LE
            let w24 = pcm_wfx(1, 48_000, 24);
            let w24_bytes = std::slice::from_raw_parts(&w24 as *const _ as *const u8, 18);
            let p24: Vec<u8> = [0i32, 4_194_304, -4_194_304, -8_388_608]
                .iter()
                .flat_map(|v| {
                    let b = v.to_le_bytes();
                    [b[0], b[1], b[2]]
                })
                .collect();
            verify_format(w24_bytes, &p24, [0.0, 0.5, -0.5, -1.0], 1e-4);

            // 4. PCM 32-bit signed LE
            let w32 = pcm_wfx(1, 48_000, 32);
            let w32_bytes = std::slice::from_raw_parts(&w32 as *const _ as *const u8, 18);
            let p32: Vec<u8> = [0i32, 1_073_741_824, -1_073_741_824, i32::MIN]
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect();
            verify_format(w32_bytes, &p32, [0.0, 0.5, -0.5, -1.0], 1e-4);

            // 5. IEEE float32
            let wf32 = float_wfx(1, 48_000);
            let wf32_bytes = std::slice::from_raw_parts(&wf32 as *const _ as *const u8, 18);
            let pf32: Vec<u8> = [0.25f32, -0.75, 0.5, -0.25]
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect();
            verify_format(wf32_bytes, &pf32, [0.25, -0.75, 0.5, -0.25], 1e-6);

            // 6. WAVE_FORMAT_EXTENSIBLE (float32) + looping wrap + Stop + cursor tracking.
            let ext = WaveFormatExtensible {
                format: WaveFormatEx {
                    format_tag: WAVE_FORMAT_EXTENSIBLE,
                    channels: 1,
                    samples_per_sec: 48_000,
                    avg_bytes_per_sec: 48_000 * 4,
                    block_align: 4,
                    bits_per_sample: 32,
                    cb_size: 22,
                },
                valid_bits_per_sample: 32,
                channel_mask: 0x4,
                sub_format: KSDATAFORMAT_SUBTYPE_IEEE_FLOAT,
            };
            let desc = make_desc(0, pf32.len() as u32, &ext as *const _ as *const u8);
            let mut raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                ds_create_sound_buffer(
                    ds,
                    &desc as *const DsBufferDesc as *const c_void,
                    &mut raw,
                    std::ptr::null_mut(),
                ),
                DS_OK
            );
            let loop_buf = raw as *mut DsBufferCom;
            write_all_bytes(loop_buf, &pf32);
            assert_eq!(dsb_play(loop_buf, 0, 0, DSBPLAY_LOOPING), DS_OK);

            let mut status = 0u32;
            assert_eq!(dsb_get_status(loop_buf, &mut status), DS_OK);
            assert_eq!(status, DSBSTATUS_PLAYING | DSBSTATUS_LOOPING);

            let mut loop_out = [0.0f32; 14]; // 7 frames -> wraps around 4-frame buffer
            (&*ds).core.mix_frames(7, &mut loop_out);
            let want = [0.25f32, -0.75, 0.5, -0.25, 0.25, -0.75, 0.5];
            for i in 0..7 {
                assert!((loop_out[i * 2] - want[i]).abs() < 1e-5);
            }
            let (mut play_c, mut write_c) = (99u32, 99u32);
            assert_eq!(dsb_get_current_position(loop_buf, &mut play_c, &mut write_c), DS_OK);
            assert_eq!(play_c, 3 * 4); // 7 % 4 == 3 frames * 4 bytes
            assert_ne!(write_c, play_c); // write cursor leads play cursor while playing

            assert_eq!(dsb_stop(loop_buf), DS_OK);
            assert_eq!(dsb_get_current_position(loop_buf, &mut play_c, &mut write_c), DS_OK);
            assert_eq!(play_c, 12);
            assert_eq!(write_c, 12); // stopped: write cursor == play cursor

            assert_eq!(dsb_release(loop_buf), 0);
            assert_eq!(ds_release(ds), 0);
        }
    }

    #[test]
    fn volume_pan_frequency_and_primary_master_volume() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let ds = make_paused_ds8();

            // Primary buffer with CTRLVOLUME so we can verify master scaling too.
            let p_desc = make_desc(DSBCAPS_PRIMARYBUFFER | DSBCAPS_CTRLVOLUME, 0, std::ptr::null());
            let mut prim_raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                ds_create_sound_buffer(
                    ds,
                    &p_desc as *const DsBufferDesc as *const c_void,
                    &mut prim_raw,
                    std::ptr::null_mut(),
                ),
                DS_OK
            );
            let prim = prim_raw as *mut DsBufferCom;

            // Secondary mono float32 buffer at 24 kHz (half of 48 kHz mix rate).
            let wfx = float_wfx(1, 24_000);
            let samples: Vec<u8> = [0.0f32, 1.0, 0.0, 1.0]
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect();
            let s_desc = make_desc(
                DSBCAPS_CTRLVOLUME | DSBCAPS_CTRLPAN | DSBCAPS_CTRLFREQUENCY,
                samples.len() as u32,
                &wfx as *const WaveFormatEx as *const u8,
            );
            let mut sec_raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                ds_create_sound_buffer(
                    ds,
                    &s_desc as *const DsBufferDesc as *const c_void,
                    &mut sec_raw,
                    std::ptr::null_mut(),
                ),
                DS_OK
            );
            let sec = sec_raw as *mut DsBufferCom;
            write_all_bytes(sec, &samples);

            // Resampling at 24 kHz -> 48 kHz (step = 0.5): frames 0,1,2 -> 0.0, 0.5, 1.0.
            assert_eq!(dsb_play(sec, 0, 0, DSBPLAY_LOOPING), DS_OK);
            let mut out = [0.0f32; 6];
            (&*ds).core.mix_frames(3, &mut out);
            assert!((out[0] - 0.0).abs() < 1e-5);
            assert!((out[2] - 0.5).abs() < 1e-5);
            assert!((out[4] - 1.0).abs() < 1e-5);

            // SetFrequency(48_000) -> 1:1 stepping; SetFrequency(0) resets to 24_000.
            assert_eq!(dsb_set_frequency(sec, 48_000), DS_OK);
            let mut got_f = 0u32;
            assert_eq!(dsb_get_frequency(sec, &mut got_f), DS_OK);
            assert_eq!(got_f, 48_000);
            assert_eq!(dsb_set_frequency(sec, DSBFREQUENCY_ORIGINAL), DS_OK);
            assert_eq!(dsb_get_frequency(sec, &mut got_f), DS_OK);
            assert_eq!(got_f, 24_000);

            // Constant 1.0 buffer for exact volume & pan verification.
            let ones: Vec<u8> = [1.0f32; 4].iter().flat_map(|v| v.to_le_bytes()).collect();
            write_all_bytes(sec, &ones);
            assert_eq!(dsb_set_current_position(sec, 0), DS_OK);

            // -2000 mB = -20 dB = 0.1 linear; hard left pan -> L=0.1, R=0.0.
            assert_eq!(dsb_set_volume(sec, -2000), DS_OK);
            assert_eq!(dsb_set_pan(sec, DSBPAN_LEFT), DS_OK);
            let mut got_v = 0i32;
            let mut got_p = 0i32;
            assert_eq!(dsb_get_volume(sec, &mut got_v), DS_OK);
            assert_eq!(dsb_get_pan(sec, &mut got_p), DS_OK);
            assert_eq!(got_v, -2000);
            assert_eq!(got_p, DSBPAN_LEFT);

            let mut vp_out = [0.0f32; 4];
            (&*ds).core.mix_frames(2, &mut vp_out);
            assert!((vp_out[0] - 0.1).abs() < 1e-4, "L={}", vp_out[0]);
            assert!(vp_out[1].abs() < 1e-6, "R={}", vp_out[1]);

            // Hard right pan + primary master volume -600 mB (~0.5012) -> L=0.0, R≈0.05012.
            assert_eq!(dsb_set_pan(sec, DSBPAN_RIGHT), DS_OK);
            assert_eq!(dsb_set_volume(prim, -600), DS_OK);
            (&*ds).core.mix_frames(2, &mut vp_out);
            assert!(vp_out[0].abs() < 1e-6);
            let want_r = 0.1 * ds_mb_to_gain(-600);
            assert!((vp_out[1] - want_r).abs() < 1e-4, "R={} want={}", vp_out[1], want_r);

            // DSBVOLUME_MIN -> exact silence.
            assert_eq!(dsb_set_volume(sec, DSBVOLUME_MIN), DS_OK);
            (&*ds).core.mix_frames(2, &mut vp_out);
            assert_eq!(vp_out, [0.0; 4]);

            assert_eq!(dsb_release(sec), 0);
            assert_eq!(dsb_release(prim), 0);
            assert_eq!(ds_release(ds), 0);
        }
    }

    #[test]
    fn duplicate_sound_buffer_shares_sample_memory_with_independent_playback() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let ds = make_paused_ds8();
            let wfx = float_wfx(1, 48_000);
            let raw_init: Vec<u8> = [0.25f32; 4].iter().flat_map(|v| v.to_le_bytes()).collect();
            let desc = make_desc(
                DSBCAPS_CTRLVOLUME | DSBCAPS_CTRLPAN,
                raw_init.len() as u32,
                &wfx as *const WaveFormatEx as *const u8,
            );
            let mut orig_raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                ds_create_sound_buffer(
                    ds,
                    &desc as *const DsBufferDesc as *const c_void,
                    &mut orig_raw,
                    std::ptr::null_mut(),
                ),
                DS_OK
            );
            let orig = orig_raw as *mut DsBufferCom;
            write_all_bytes(orig, &raw_init);

            let mut dup_raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(ds_duplicate_sound_buffer(ds, orig_raw, &mut dup_raw), DS_OK);
            let dup = dup_raw as *mut DsBufferCom;

            // Pan orig hard left and dup hard right; both mix simultaneously!
            assert_eq!(dsb_set_pan(orig, DSBPAN_LEFT), DS_OK);
            assert_eq!(dsb_set_pan(dup, DSBPAN_RIGHT), DS_OK);
            assert_eq!(dsb_play(orig, 0, 0, DSBPLAY_LOOPING), DS_OK);
            assert_eq!(dsb_play(dup, 0, 0, DSBPLAY_LOOPING), DS_OK);

            let mut out = [0.0f32; 4];
            (&*ds).core.mix_frames(2, &mut out);
            assert!((out[0] - 0.25).abs() < 1e-5);
            assert!((out[1] - 0.25).abs() < 1e-5);

            // Mutating sample memory through `dup` is immediately visible to `orig`.
            let updated: Vec<u8> = [0.75f32; 4].iter().flat_map(|v| v.to_le_bytes()).collect();
            write_all_bytes(dup, &updated);
            assert_eq!(dsb_stop(dup), DS_OK);
            (&*ds).core.mix_frames(2, &mut out);
            assert!((out[0] - 0.75).abs() < 1e-5);
            assert!(out[1].abs() < 1e-6);

            // Releasing `orig` leaves `dup`'s shared sample memory alive and playable.
            assert_eq!(dsb_release(orig), 0);
            assert_eq!(dsb_play(dup, 0, 0, DSBPLAY_LOOPING), DS_OK);
            (&*ds).core.mix_frames(2, &mut out);
            assert!(out[0].abs() < 1e-6);
            assert!((out[1] - 0.75).abs() < 1e-5);

            assert_eq!(dsb_release(dup), 0);
            assert_eq!(ds_release(ds), 0);
        }
    }

    #[test]
    fn three_d_attenuation_panning_cone_and_deferred_commit() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let ds = make_paused_ds8();

            // Primary buffer + IDirectSound3DListener via QueryInterface.
            let p_desc = make_desc(DSBCAPS_PRIMARYBUFFER | DSBCAPS_CTRL3D, 0, std::ptr::null());
            let mut prim_raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                ds_create_sound_buffer(
                    ds,
                    &p_desc as *const DsBufferDesc as *const c_void,
                    &mut prim_raw,
                    std::ptr::null_mut(),
                ),
                DS_OK
            );
            let prim = prim_raw as *mut DsBufferCom;
            let mut listener: *mut c_void = std::ptr::null_mut();
            assert_eq!(dsb_query_interface(prim, &IID_IDIRECTSOUND3DLISTENER, &mut listener), DS_OK);
            assert!(!listener.is_null());

            // Secondary 3D mono buffer + IDirectSound3DBuffer via QueryInterface.
            let wfx = float_wfx(1, 48_000);
            let ones: Vec<u8> = [1.0f32; 4].iter().flat_map(|v| v.to_le_bytes()).collect();
            let s_desc = make_desc(
                DSBCAPS_CTRL3D | DSBCAPS_MUTE3DATMAXDISTANCE,
                ones.len() as u32,
                &wfx as *const WaveFormatEx as *const u8,
            );
            let mut sec_raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                ds_create_sound_buffer(
                    ds,
                    &s_desc as *const DsBufferDesc as *const c_void,
                    &mut sec_raw,
                    std::ptr::null_mut(),
                ),
                DS_OK
            );
            let sec = sec_raw as *mut DsBufferCom;
            write_all_bytes(sec, &ones);

            let mut buf3d: *mut c_void = std::ptr::null_mut();
            assert_eq!(dsb_query_interface(sec, &IID_IDIRECTSOUND3DBUFFER, &mut buf3d), DS_OK);
            assert!(!buf3d.is_null());

            // Configure min_distance = 2.0, max_distance = 10.0.
            assert_eq!(ds3db_set_min_distance(buf3d, 2.0, DS3D_IMMEDIATE), DS_OK);
            assert_eq!(ds3db_set_max_distance(buf3d, 10.0, DS3D_IMMEDIATE), DS_OK);
            assert_eq!(dsb_play(sec, 0, 0, DSBPLAY_LOOPING), DS_OK);

            // Hard left at distance 4.0 -> attenuation = 2.0 / 4.0 = 0.5, pan = (1.0, 0.0).
            assert_eq!(ds3db_set_position(buf3d, -4.0, 0.0, 0.0, DS3D_IMMEDIATE), DS_OK);
            let mut out = [0.0f32; 4];
            (&*ds).core.mix_frames(2, &mut out);
            assert!((out[0] - 0.5).abs() < 1e-4, "left={}", out[0]);
            assert!(out[1].abs() < 1e-4, "right={}", out[1]);

            // Deferred move to hard right at distance 8.0 -> attenuation = 0.25, NOT active yet!
            assert_eq!(ds3db_set_position(buf3d, 8.0, 0.0, 0.0, DS3D_DEFERRED), DS_OK);
            (&*ds).core.mix_frames(2, &mut out);
            assert!((out[0] - 0.5).abs() < 1e-4);

            // CommitDeferredSettings applies the deferred move.
            assert_eq!(ds3dl_commit_deferred_settings(listener), DS_OK);
            (&*ds).core.mix_frames(2, &mut out);
            assert!(out[0].abs() < 1e-4, "left={}", out[0]);
            assert!((out[1] - 0.25).abs() < 1e-4, "right={}", out[1]);

            // Beyond max_distance (12.0 >= 10.0) with DSBCAPS_MUTE3DATMAXDISTANCE -> silence.
            assert_eq!(ds3db_set_position(buf3d, 12.0, 0.0, 0.0, DS3D_IMMEDIATE), DS_OK);
            (&*ds).core.mix_frames(2, &mut out);
            assert_eq!(out, [0.0; 4]);

            // Sound cone: emitter at (0, 0, 2) pointing away (+Z) with 60/120 cone & -2000 mB outside.
            assert_eq!(ds3db_set_position(buf3d, 0.0, 0.0, 2.0, DS3D_IMMEDIATE), DS_OK);
            assert_eq!(ds3db_set_cone_orientation(buf3d, 0.0, 0.0, 1.0, DS3D_IMMEDIATE), DS_OK);
            assert_eq!(ds3db_set_cone_angles(buf3d, 60, 120, DS3D_IMMEDIATE), DS_OK);
            assert_eq!(ds3db_set_cone_outside_volume(buf3d, -2000, DS3D_IMMEDIATE), DS_OK);
            (&*ds).core.mix_frames(2, &mut out);
            let expected_rear = 0.1 * std::f32::consts::FRAC_1_SQRT_2;
            assert!((out[0] - expected_rear).abs() < 1e-4, "rear L={}", out[0]);
            assert!((out[1] - expected_rear).abs() < 1e-4, "rear R={}", out[1]);

            // DS3DMODE_DISABLE bypasses 3D attenuation -> unity center (1.0, 1.0).
            assert_eq!(ds3db_set_mode(buf3d, DS3DMODE_DISABLE, DS3D_IMMEDIATE), DS_OK);
            (&*ds).core.mix_frames(2, &mut out);
            assert!((out[0] - 1.0).abs() < 1e-4);
            assert!((out[1] - 1.0).abs() < 1e-4);

            // IUnknown identity across sub-interfaces.
            let mut unk_a: *mut c_void = std::ptr::null_mut();
            let mut unk_b: *mut c_void = std::ptr::null_mut();
            assert_eq!(dsb_query_interface(sec, &IID_IUnknown, &mut unk_a), DS_OK);
            assert_eq!(ds3db_query_interface(buf3d, &IID_IUnknown, &mut unk_b), DS_OK);
            assert_eq!(unk_a, unk_b);
            dsb_release(unk_a as *mut DsBufferCom);
            dsb_release(unk_b as *mut DsBufferCom);

            assert_eq!(ds3db_release(buf3d), 1);
            assert_eq!(dsb_release(sec), 0);
            assert_eq!(ds3dl_release(listener), 1);
            assert_eq!(dsb_release(prim), 0);
            assert_eq!(ds_release(ds), 0);
        }
    }

    #[test]
    fn capture_buffers_and_full_duplex_create() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let mut dsc_raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                DirectSoundCaptureCreate8(std::ptr::null(), &mut dsc_raw, std::ptr::null_mut()),
                DS_OK
            );
            let dsc = dsc_raw as *mut DsCaptureCom;

            let mut caps = DscCaps { dw_size: std::mem::size_of::<DscCaps>() as u32, ..DscCaps::default() };
            assert_eq!(dsc_get_caps(dsc, &mut caps), DS_OK);
            assert!(caps.dw_channels >= 2);
            assert_ne!(caps.dw_formats, 0);

            let wfx = pcm_wfx(2, 48_000, 16);
            let buf_bytes = 48_000 * 4 / 10; // 100 ms circular capture buffer
            let cdesc = make_cap_desc(0, buf_bytes, &wfx as *const WaveFormatEx as *const u8);
            let mut cb_raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                dsc_create_capture_buffer(
                    dsc,
                    &cdesc as *const DscBufferDesc as *const c_void,
                    &mut cb_raw,
                    std::ptr::null_mut(),
                ),
                DS_OK
            );
            let cb = cb_raw as *mut DsCaptureBufferCom;

            // Wrap-around Lock/Unlock on capture buffer verifies digital silence & circularity.
            let mut p1: *mut c_void = std::ptr::null_mut();
            let mut l1 = 0u32;
            let mut p2: *mut c_void = std::ptr::null_mut();
            let mut l2 = 0u32;
            assert_eq!(
                dscb_lock(cb, buf_bytes - 16, 32, &mut p1, &mut l1, &mut p2, &mut l2, 0),
                DS_OK
            );
            assert_eq!(l1, 16);
            assert_eq!(l2, 16);
            assert!(std::slice::from_raw_parts(p1 as *const u8, 16).iter().all(|&b| b == 0));
            assert!(std::slice::from_raw_parts(p2 as *const u8, 16).iter().all(|&b| b == 0));
            assert_eq!(dscb_unlock(cb, p1, 16, p2, 16), DS_OK);

            // Start looping capture and observe cursor advance.
            assert_eq!(dscb_start(cb, DSCBSTART_LOOPING), DS_OK);
            let mut status = 0u32;
            assert_eq!(dscb_get_status(cb, &mut status), DS_OK);
            assert_eq!(status, DSCBSTATUS_CAPTURING | DSCBSTATUS_LOOPING);

            std::thread::sleep(Duration::from_millis(15));
            let (mut cap_pos, mut read_pos) = (0u32, 0u32);
            assert_eq!(dscb_get_current_position(cb, &mut cap_pos, &mut read_pos), DS_OK);
            assert!(cap_pos > 0, "capture cursor did not advance");

            assert_eq!(dscb_stop(cb), DS_OK);
            assert_eq!(dscb_get_status(cb, &mut status), DS_OK);
            assert_eq!(status, 0);

            assert_eq!(dscb_release(cb), 0);
            assert_eq!(dsc_release(dsc), 0);

            // DirectSoundFullDuplexCreate builds full-duplex + capture8 + render8 buffers together.
            let rdesc = make_desc(DSBCAPS_CTRLVOLUME, 4096, &wfx as *const WaveFormatEx as *const u8);
            let (mut fd_raw, mut fd_cb, mut fd_rb) =
                (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
            assert_eq!(
                DirectSoundFullDuplexCreate(
                    std::ptr::null(),
                    std::ptr::null(),
                    &cdesc as *const DscBufferDesc as *const c_void,
                    &rdesc as *const DsBufferDesc as *const c_void,
                    std::ptr::null_mut(),
                    DSSCL_PRIORITY,
                    &mut fd_raw,
                    &mut fd_cb,
                    &mut fd_rb,
                    std::ptr::null_mut(),
                ),
                DS_OK
            );
            assert!(!fd_raw.is_null() && !fd_cb.is_null() && !fd_rb.is_null());

            let mut out_wfx = WaveFormatEx {
                format_tag: 0,
                channels: 0,
                samples_per_sec: 0,
                avg_bytes_per_sec: 0,
                block_align: 0,
                bits_per_sample: 0,
                cb_size: 0,
            };
            let mut written = 0u32;
            assert_eq!(
                dsb_get_format(
                    fd_rb as *mut DsBufferCom,
                    &mut out_wfx as *mut WaveFormatEx as *mut u8,
                    18,
                    &mut written,
                ),
                DS_OK
            );
            assert_eq!(written, 18);
            assert_eq!({ out_wfx.samples_per_sec }, 48_000);

            assert_eq!(dscb_release(fd_cb as *mut DsCaptureBufferCom), 0);
            assert_eq!(dsb_release(fd_rb as *mut DsBufferCom), 0);
            assert_eq!(dsfd_release(fd_raw as *mut DsFullDuplexCom), 0);
        }
    }

    #[test]
    fn realtime_null_backend_mixer_plays_and_completes_oneshot() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let mut ds_raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(DirectSoundCreate8(std::ptr::null(), &mut ds_raw, std::ptr::null_mut()), DS_OK);
            let ds = ds_raw as *mut DsDeviceCom;

            // 960 stereo frames = 20 ms at 48 kHz.
            let wfx = pcm_wfx(2, 48_000, 16);
            let pcm = vec![0u8; 960 * 4];
            let desc = make_desc(
                DSBCAPS_CTRLPOSITIONNOTIFY,
                pcm.len() as u32,
                &wfx as *const WaveFormatEx as *const u8,
            );
            let mut buf_raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                ds_create_sound_buffer(
                    ds,
                    &desc as *const DsBufferDesc as *const c_void,
                    &mut buf_raw,
                    std::ptr::null_mut(),
                ),
                DS_OK
            );
            let buf = buf_raw as *mut DsBufferCom;
            write_all_bytes(buf, &pcm);

            // Attach a DSBPN_OFFSETSTOP event notification via IDirectSoundNotify.
            let ev_handle = crate::kernel32::threads::CreateEventW(
                std::ptr::null_mut(),
                TRUE,
                FALSE,
                std::ptr::null(),
            );
            assert!(!ev_handle.is_null());
            let mut notify: *mut c_void = std::ptr::null_mut();
            assert_eq!(dsb_query_interface(buf, &IID_IDIRECTSOUNDNOTIFY, &mut notify), DS_OK);
            let pn = DsbPositionNotify { dw_offset: DSBPN_OFFSETSTOP, h_event_notify: ev_handle };
            assert_eq!(dsn_set_notification_positions(notify, 1, &pn), DS_OK);

            // Start real-time playback on Null OutputStream background mixer thread.
            assert_eq!(dsb_play(buf, 0, 0, 0), DS_OK);

            let wait = crate::kernel32::threads::WaitForSingleObject(ev_handle, 500);
            assert_eq!(wait, WAIT_OBJECT_0, "background mixer did not complete one-shot buffer");

            let mut status = 99u32;
            assert_eq!(dsb_get_status(buf, &mut status), DS_OK);
            assert_eq!(status, 0);

            crate::kernel32::threads::CloseHandle(ev_handle);
            assert_eq!(dsn_release(notify), 1);
            assert_eq!(dsb_release(buf), 0);
            assert_eq!(ds_release(ds), 0);
        }
    }

    #[test]
    fn ds7_compat_controls_format_listener_orientation_doppler_and_notify_offset() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            // DirectSoundCreate (DS7) exposes IDirectSound but NOT IDirectSound8.
            let mut ds7_raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(DirectSoundCreate(std::ptr::null(), &mut ds7_raw, std::ptr::null_mut()), DS_OK);
            let ds7 = ds7_raw as *mut DsDeviceCom;
            let mut qi_out: *mut c_void = std::ptr::null_mut();
            assert_eq!(ds_query_interface(ds7, &IID_IDIRECTSOUND8, &mut qi_out), E_NOINTERFACE);
            assert_eq!(ds_query_interface(ds7, &IID_IDIRECTSOUND, &mut qi_out), DS_OK);
            assert_eq!(ds_release(qi_out as *mut DsDeviceCom), 1);

            // Primary buffer SetFormat requires DSSCL_PRIORITY+.
            let p_desc = make_desc(DSBCAPS_PRIMARYBUFFER | DSBCAPS_CTRL3D, 0, std::ptr::null());
            let mut prim_raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                ds_create_sound_buffer(
                    ds7,
                    &p_desc as *const DsBufferDesc as *const c_void,
                    &mut prim_raw,
                    std::ptr::null_mut(),
                ),
                DS_OK
            );
            let prim = prim_raw as *mut DsBufferCom;
            let fmt44 = pcm_wfx(2, 44_100, 16);
            assert_eq!(
                dsb_set_format(prim, &fmt44 as *const WaveFormatEx as *const u8),
                DSERR_PRIOLEVELNEEDED
            );
            assert_eq!(ds_set_cooperative_level(ds7, std::ptr::null_mut(), DSSCL_PRIORITY), DS_OK);
            assert_eq!(dsb_set_format(prim, &fmt44 as *const WaveFormatEx as *const u8), DS_OK);

            // GetFormat(NULL) queries required byte size.
            let mut req_sz = 0u32;
            assert_eq!(dsb_get_format(prim, std::ptr::null_mut(), 0, &mut req_sz), DS_OK);
            assert_eq!(req_sz, 18);

            // Secondary buffer without CTRL flags rejects SetVolume/SetPan/SetFrequency & SetFormat.
            let wfx = float_wfx(1, 44_100);
            let s_desc = make_desc(0, 64, &wfx as *const WaveFormatEx as *const u8);
            let mut sec_raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                ds_create_sound_buffer(
                    ds7,
                    &s_desc as *const DsBufferDesc as *const c_void,
                    &mut sec_raw,
                    std::ptr::null_mut(),
                ),
                DS_OK
            );
            let sec = sec_raw as *mut DsBufferCom;
            assert_eq!(dsb_query_interface(sec, &IID_IDIRECTSOUNDBUFFER8, &mut qi_out), E_NOINTERFACE);
            assert_eq!(dsb_set_volume(sec, -100), DSERR_CONTROLUNAVAIL);
            assert_eq!(dsb_set_pan(sec, 100), DSERR_CONTROLUNAVAIL);
            assert_eq!(dsb_set_frequency(sec, 22_050), DSERR_CONTROLUNAVAIL);
            assert_eq!(
                dsb_set_format(sec, &fmt44 as *const WaveFormatEx as *const u8),
                DSERR_INVALIDCALL
            );

            // DSBLOCK_FROMWRITECURSOR locks from the current write cursor.
            assert_eq!(dsb_set_current_position(sec, 16), DS_OK);
            let (mut p1, mut l1, mut p2, mut l2) = (std::ptr::null_mut(), 0u32, std::ptr::null_mut(), 0u32);
            assert_eq!(
                dsb_lock(sec, 0, 8, &mut p1, &mut l1, &mut p2, &mut l2, DSBLOCK_FROMWRITECURSOR),
                DS_OK
            );
            assert_eq!(l1, 8);
            assert_eq!(l2, 0);
            assert_eq!(dsb_unlock(sec, p1, l1, p2, l2), DS_OK);

            assert_eq!(dsb_release(sec), 0);
            assert_eq!(dsb_release(prim), 0);
            assert_eq!(ds_release(ds7), 0);

            // 3D listener orientation, Doppler shift, HEADRELATIVE mode, and mid-buffer Notify.
            let ds = make_paused_ds8();
            let p_desc = make_desc(DSBCAPS_PRIMARYBUFFER | DSBCAPS_CTRL3D, 0, std::ptr::null());
            let mut prim_raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                ds_create_sound_buffer(
                    ds,
                    &p_desc as *const DsBufferDesc as *const c_void,
                    &mut prim_raw,
                    std::ptr::null_mut(),
                ),
                DS_OK
            );
            let prim = prim_raw as *mut DsBufferCom;
            let mut listener: *mut c_void = std::ptr::null_mut();
            assert_eq!(dsb_query_interface(prim, &IID_IDIRECTSOUND3DLISTENER, &mut listener), DS_OK);

            // Rotate listener to face +X (front=(1,0,0), top=(0,1,0)) -> right = cross(top, front) = (0,0,-1).
            // So an emitter at (0,0,-4) is hard RIGHT!
            assert_eq!(
                ds3dl_set_orientation(listener, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, DS3D_IMMEDIATE),
                DS_OK
            );
            assert_eq!(ds3dl_set_distance_factor(listener, 1.0, DS3D_IMMEDIATE), DS_OK);
            assert_eq!(ds3dl_set_rolloff_factor(listener, 1.0, DS3D_IMMEDIATE), DS_OK);
            assert_eq!(ds3dl_set_doppler_factor(listener, 1.0, DS3D_IMMEDIATE), DS_OK);
            assert_eq!(ds3dl_set_position(listener, 0.0, 0.0, 0.0, DS3D_IMMEDIATE), DS_OK);
            assert_eq!(ds3dl_set_velocity(listener, 0.0, 0.0, 0.0, DS3D_IMMEDIATE), DS_OK);

            let mut lparams = Ds3dListener {
                dw_size: std::mem::size_of::<Ds3dListener>() as u32,
                v_position: D3dVector::default(),
                v_velocity: D3dVector::default(),
                v_orient_front: D3dVector::default(),
                v_orient_top: D3dVector::default(),
                fl_distance_factor: 0.0,
                fl_rolloff_factor: 0.0,
                fl_doppler_factor: 0.0,
            };
            assert_eq!(ds3dl_get_all_parameters(listener, &mut lparams), DS_OK);
            assert!((lparams.v_orient_front.x - 1.0).abs() < 1e-5);

            // 8-frame mono ramp buffer with CTRL3D + CTRLPOSITIONNOTIFY.
            let wfx48 = float_wfx(1, 48_000);
            let ramp: Vec<u8> = [0.0f32, 0.125, 0.25, 0.375, 0.5, 0.625, 0.75, 0.875]
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect();
            let b_desc = make_desc(
                DSBCAPS_CTRL3D | DSBCAPS_CTRLPOSITIONNOTIFY,
                ramp.len() as u32,
                &wfx48 as *const WaveFormatEx as *const u8,
            );
            let mut b_raw: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                ds_create_sound_buffer(
                    ds,
                    &b_desc as *const DsBufferDesc as *const c_void,
                    &mut b_raw,
                    std::ptr::null_mut(),
                ),
                DS_OK
            );
            let buf = b_raw as *mut DsBufferCom;
            write_all_bytes(buf, &ramp);

            let mut buf3d: *mut c_void = std::ptr::null_mut();
            assert_eq!(dsb_query_interface(buf, &IID_IDIRECTSOUND3DBUFFER, &mut buf3d), DS_OK);
            assert_eq!(ds3db_set_min_distance(buf3d, 2.0, DS3D_IMMEDIATE), DS_OK);
            assert_eq!(ds3db_set_position(buf3d, 0.0, 0.0, -4.0, DS3D_IMMEDIATE), DS_OK);

            // Attach offset notification at byte 16 (frame 4).
            let ev_mid = crate::kernel32::threads::CreateEventW(
                std::ptr::null_mut(),
                TRUE,
                FALSE,
                std::ptr::null(),
            );
            let mut notify: *mut c_void = std::ptr::null_mut();
            assert_eq!(dsb_query_interface(buf, &IID_IDIRECTSOUNDNOTIFY, &mut notify), DS_OK);
            let pn = DsbPositionNotify { dw_offset: 16, h_event_notify: ev_mid };
            assert_eq!(dsn_set_notification_positions(notify, 1, &pn), DS_OK);

            assert_eq!(dsb_play(buf, 0, 0, DSBPLAY_LOOPING), DS_OK);
            let mut out = [0.0f32; 12]; // 6 frames crosses byte 16!
            (&*ds).core.mix_frames(6, &mut out);
            // Frame 4 (ramp=0.5) at distance 4 with min_distance 2 -> 0.5 * 0.5 = 0.25 on RIGHT, 0 on LEFT!
            assert!(out[8].abs() < 1e-4, "left={}", out[8]);
            assert!((out[9] - 0.25).abs() < 1e-4, "right={}", out[9]);
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(ev_mid, 0), WAIT_OBJECT_0);

            // DS3DMODE_HEADRELATIVE ignores listener rotation: (-2, 0, 0) is hard LEFT at min_distance.
            assert_eq!(ds3db_set_mode(buf3d, DS3DMODE_HEADRELATIVE, DS3D_IMMEDIATE), DS_OK);
            assert_eq!(ds3db_set_position(buf3d, -2.0, 0.0, 0.0, DS3D_IMMEDIATE), DS_OK);
            assert_eq!(dsb_set_current_position(buf, 16), DS_OK); // frame 4 -> 0.5
            let mut hr_out = [0.0f32; 2];
            (&*ds).core.mix_frames(1, &mut hr_out);
            assert!((hr_out[0] - 0.5).abs() < 1e-4, "headrel L={}", hr_out[0]);
            assert!(hr_out[1].abs() < 1e-4, "headrel R={}", hr_out[1]);

            // Approaching velocity causes Doppler speedup (> 1.0 frames advanced per frame).
            assert_eq!(ds3db_set_mode(buf3d, DS3DMODE_NORMAL, DS3D_IMMEDIATE), DS_OK);
            assert_eq!(ds3db_set_position(buf3d, 0.0, 0.0, 10.0, DS3D_IMMEDIATE), DS_OK);
            assert_eq!(ds3db_set_velocity(buf3d, 0.0, 0.0, -SPEED_OF_SOUND * 0.5, DS3D_IMMEDIATE), DS_OK);
            assert_eq!(dsb_set_current_position(buf, 0), DS_OK);
            let mut dop_out = [0.0f32; 4];
            (&*ds).core.mix_frames(2, &mut dop_out);
            let (mut play_c, mut write_c) = (0u32, 0u32);
            assert_eq!(dsb_get_current_position(buf, &mut play_c, &mut write_c), DS_OK);
            // Speedup is 2x: 2 mixed frames advance 4 source frames (16 bytes)!
            assert_eq!(play_c, 16);

            crate::kernel32::threads::CloseHandle(ev_mid);
            assert_eq!(dsn_release(notify), 2);
            assert_eq!(ds3db_release(buf3d), 1);
            assert_eq!(dsb_release(buf), 0);
            assert_eq!(ds3dl_release(listener), 1);
            assert_eq!(dsb_release(prim), 0);
            assert_eq!(ds_release(ds), 0);
        }
    }
}
