//! DirectSound 8 constants, Win64 ABI structures, PCM format parsing/storage,
//! device GUID resolution/enumeration, and buffer/capture state types.

use crate::audio::backend::{self, DeviceInfo};
use crate::audio::com::{
    GUID, KSDATAFORMAT_SUBTYPE_IEEE_FLOAT, KSDATAFORMAT_SUBTYPE_PCM,
};
use crate::audio::format::{
    self, decode_pcm_sample_f32, SampleType, Vec3, WaveFormat, WAVEFORMATEX,
    WAVEFORMATEXTENSIBLE, WAVE_FORMAT_EXTENSIBLE, WAVE_FORMAT_IEEE_FLOAT, WAVE_FORMAT_PCM,
};
use crate::strings::{rd, wr};
use crate::types::*;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::time::Instant;

// ============================================================================
// HRESULTs, GUIDs & DirectSound Constants
// ============================================================================

pub(super) const DS_OK: HRESULT = S_OK;
pub(super) const DSERR_CONTROLUNAVAIL: HRESULT = 0x8878_001Eu32 as HRESULT;
pub(super) const DSERR_INVALIDPARAM: HRESULT = E_INVALIDARG;
pub(super) const DSERR_INVALIDCALL: HRESULT = 0x8878_0032u32 as HRESULT;
pub(super) const DSERR_PRIOLEVELNEEDED: HRESULT = 0x8878_0046u32 as HRESULT;
pub(super) const DSERR_BADFORMAT: HRESULT = 0x8878_0064u32 as HRESULT;
pub(super) const DSERR_NODRIVER: HRESULT = 0x8878_0078u32 as HRESULT;
pub(super) const DSERR_ALREADYINITIALIZED: HRESULT = 0x8878_0082u32 as HRESULT;
pub(super) const DSERR_NOAGGREGATION: HRESULT = 0x8004_0110u32 as HRESULT;
pub(super) const DSERR_UNINITIALIZED: HRESULT = 0x8878_00AAu32 as HRESULT;
pub(super) const DSERR_FXUNAVAILABLE: HRESULT = 0x8878_00DCu32 as HRESULT;
pub(super) const DSERR_OBJECTNOTFOUND: HRESULT = 0x8878_1161u32 as HRESULT;
pub(super) const CLASS_E_CLASSNOTAVAILABLE: HRESULT = 0x8004_0111u32 as HRESULT;

pub(super) const GUID_NULL: GUID = GUID::new(0, 0, 0, [0; 8]);
pub(super) const IID_ICLASSFACTORY: GUID =
    GUID::new(0x0000_0001, 0x0000, 0x0000, [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46]);

pub(super) const CLSID_DIRECTSOUND: GUID =
    GUID::new(0x47D4_D946, 0x62E8, 0x11CF, [0x93, 0xBC, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00]);
pub(super) const CLSID_DIRECTSOUND8: GUID =
    GUID::new(0x3901_CC3F, 0x84B5, 0x4FA4, [0xBA, 0x35, 0xAA, 0x81, 0x72, 0xB8, 0xA0, 0x9B]);
pub(super) const CLSID_DIRECTSOUNDCAPTURE: GUID =
    GUID::new(0xB021_0780, 0x89CD, 0x11D0, [0xAF, 0x08, 0x00, 0xA0, 0xC9, 0x25, 0xCD, 0x16]);
pub(super) const CLSID_DIRECTSOUNDCAPTURE8: GUID =
    GUID::new(0xE4BC_AC13, 0x7F99, 0x4908, [0x9A, 0x8E, 0x74, 0xE3, 0xBF, 0x24, 0xB6, 0xE1]);
pub(super) const CLSID_DIRECTSOUNDFULLDUPLEX: GUID =
    GUID::new(0xFEA4_300C, 0x7959, 0x4147, [0xB2, 0x6A, 0x23, 0x77, 0xB9, 0xE7, 0xA9, 0x1D]);

pub(super) const DSDEVID_DEFAULTPLAYBACK: GUID =
    GUID::new(0xDEF0_0000, 0x9C6D, 0x47ED, [0xAA, 0xF1, 0x4D, 0xDA, 0x8F, 0x2B, 0x5C, 0x03]);
pub(super) const DSDEVID_DEFAULTCAPTURE: GUID =
    GUID::new(0xDEF0_0001, 0x9C6D, 0x47ED, [0xAA, 0xF1, 0x4D, 0xDA, 0x8F, 0x2B, 0x5C, 0x03]);
pub(super) const DSDEVID_DEFAULTVOICEPLAYBACK: GUID =
    GUID::new(0xDEF0_0002, 0x9C6D, 0x47ED, [0xAA, 0xF1, 0x4D, 0xDA, 0x8F, 0x2B, 0x5C, 0x03]);
pub(super) const DSDEVID_DEFAULTVOICECAPTURE: GUID =
    GUID::new(0xDEF0_0003, 0x9C6D, 0x47ED, [0xAA, 0xF1, 0x4D, 0xDA, 0x8F, 0x2B, 0x5C, 0x03]);

pub(super) const IID_IDIRECTSOUND: GUID =
    GUID::new(0x279A_FA83, 0x4981, 0x11CE, [0xA5, 0x21, 0x00, 0x20, 0xAF, 0x0B, 0xE5, 0x60]);
pub(super) const IID_IDIRECTSOUND8: GUID =
    GUID::new(0xC50A_7E93, 0xF395, 0x4834, [0x9E, 0xF6, 0x7F, 0xA9, 0x9D, 0xE5, 0x09, 0x66]);
pub(super) const IID_IDIRECTSOUNDBUFFER: GUID =
    GUID::new(0x279A_FA85, 0x4981, 0x11CE, [0xA5, 0x21, 0x00, 0x20, 0xAF, 0x0B, 0xE5, 0x60]);
pub(super) const IID_IDIRECTSOUNDBUFFER8: GUID =
    GUID::new(0x6825_A449, 0x7524, 0x4D82, [0x92, 0x0F, 0x50, 0xE3, 0x6A, 0xB3, 0xAB, 0x1E]);
pub(super) const IID_IDIRECTSOUND3DLISTENER: GUID =
    GUID::new(0x279A_FA84, 0x4981, 0x11CE, [0xA5, 0x21, 0x00, 0x20, 0xAF, 0x0B, 0xE5, 0x60]);
pub(super) const IID_IDIRECTSOUND3DBUFFER: GUID =
    GUID::new(0x279A_FA86, 0x4981, 0x11CE, [0xA5, 0x21, 0x00, 0x20, 0xAF, 0x0B, 0xE5, 0x60]);
pub(super) const IID_IDIRECTSOUNDCAPTURE: GUID =
    GUID::new(0xB021_0781, 0x89CD, 0x11D0, [0xAF, 0x08, 0x00, 0xA0, 0xC9, 0x25, 0xCD, 0x16]);
pub(super) const IID_IDIRECTSOUNDCAPTURE8: GUID = IID_IDIRECTSOUNDCAPTURE;
pub(super) const IID_IDIRECTSOUNDCAPTUREBUFFER: GUID =
    GUID::new(0xB021_0782, 0x89CD, 0x11D0, [0xAF, 0x08, 0x00, 0xA0, 0xC9, 0x25, 0xCD, 0x16]);
pub(super) const IID_IDIRECTSOUNDCAPTUREBUFFER8: GUID =
    GUID::new(0x0099_0DF4, 0x0DBB, 0x4872, [0x83, 0x3E, 0x6D, 0x30, 0x3E, 0x80, 0xAE, 0xB6]);
pub(super) const IID_IDIRECTSOUNDNOTIFY: GUID =
    GUID::new(0xB021_0783, 0x89CD, 0x11D0, [0xAF, 0x08, 0x00, 0xA0, 0xC9, 0x25, 0xCD, 0x16]);
pub(super) const IID_IDIRECTSOUNDFULLDUPLEX: GUID =
    GUID::new(0xEDCB_4C7A, 0xDAAB, 0x4216, [0xA4, 0x2E, 0x6C, 0x50, 0x59, 0x6D, 0xAD, 0xC1]);

// Cooperative levels
pub(super) const DSSCL_NORMAL: u32 = 1;
pub(super) const DSSCL_PRIORITY: u32 = 2;
pub(super) const DSSCL_WRITEPRIMARY: u32 = 4;

// Speaker configuration defaults
const DSSPEAKER_STEREO: u32 = 4;
const DSSPEAKER_GEOMETRY_WIDE: u32 = 0x14;
pub(super) const DEFAULT_SPEAKER_CONFIG: u32 = DSSPEAKER_STEREO | (DSSPEAKER_GEOMETRY_WIDE << 16);

// DSCAPS flags
pub(super) const DSCAPS_PRIMARYMONO: u32 = 0x0000_0001;
pub(super) const DSCAPS_PRIMARYSTEREO: u32 = 0x0000_0002;
pub(super) const DSCAPS_PRIMARY8BIT: u32 = 0x0000_0004;
pub(super) const DSCAPS_PRIMARY16BIT: u32 = 0x0000_0008;
pub(super) const DSCAPS_CONTINUOUSRATE: u32 = 0x0000_0010;
pub(super) const DSCAPS_CERTIFIED: u32 = 0x0000_0040;
pub(super) const DSCAPS_SECONDARYMONO: u32 = 0x0000_0100;
pub(super) const DSCAPS_SECONDARYSTEREO: u32 = 0x0000_0200;
pub(super) const DSCAPS_SECONDARY8BIT: u32 = 0x0000_0400;
pub(super) const DSCAPS_SECONDARY16BIT: u32 = 0x0000_0800;
pub(super) const DS_CERTIFIED: u32 = 0;

// DSBCAPS flags
pub(super) const DSBCAPS_PRIMARYBUFFER: u32 = 0x0000_0001;
pub(super) const DSBCAPS_CTRL3D: u32 = 0x0000_0010;
pub(super) const DSBCAPS_CTRLFREQUENCY: u32 = 0x0000_0020;
pub(super) const DSBCAPS_CTRLPAN: u32 = 0x0000_0040;
pub(super) const DSBCAPS_CTRLVOLUME: u32 = 0x0000_0080;
pub(super) const DSBCAPS_CTRLPOSITIONNOTIFY: u32 = 0x0000_0100;
pub(super) const DSBCAPS_CTRLFX: u32 = 0x0000_0200;
pub(super) const DSBCAPS_MUTE3DATMAXDISTANCE: u32 = 0x0002_0000;

// DSBPLAY / DSBSTATUS / DSBLOCK flags
pub(super) const DSBPLAY_LOOPING: u32 = 0x0000_0001;
pub(super) const DSBSTATUS_PLAYING: u32 = 0x0000_0001;
pub(super) const DSBSTATUS_LOOPING: u32 = 0x0000_0004;
pub(super) const DSBLOCK_FROMWRITECURSOR: u32 = 0x0000_0001;
pub(super) const DSBLOCK_ENTIREBUFFER: u32 = 0x0000_0002;

// Buffer parameter bounds
pub(super) const DSBFREQUENCY_ORIGINAL: u32 = 0;
pub(super) const DSBFREQUENCY_MIN: u32 = 100;
pub(super) const DSBFREQUENCY_MAX: u32 = 200_000;
pub(super) const DSBPAN_LEFT: i32 = -10_000;
pub(super) const DSBPAN_CENTER: i32 = 0;
pub(super) const DSBPAN_RIGHT: i32 = 10_000;
pub(super) const DSBVOLUME_MIN: i32 = -10_000;
pub(super) const DSBVOLUME_MAX: i32 = 0;
pub(super) const DSBSIZE_MIN: u32 = 4;
pub(super) const DSBSIZE_MAX: u32 = 0x0FFF_FFFF;
pub(super) const DSBPN_OFFSETSTOP: u32 = 0xFFFF_FFFF;
pub(super) const DSFX_LOCHARDWARE: u32 = 1;
pub(super) const DSFXR_LOCSOFTWARE: u32 = 2;
pub(super) const DSFXR_FAILED: u32 = 4;
pub(super) const DSFXR_UNKNOWN: u32 = 5;

// Standard DirectSound 8 FX CLSIDs (<dsound.h>)
pub(super) const GUID_DSFX_STANDARD_GARGLE: GUID =
    GUID::new(0xDAFD_8210, 0x5711, 0x4B91, [0x9F, 0xE3, 0xF7, 0x5B, 0x7A, 0xE2, 0x79, 0xBF]);
pub(super) const GUID_DSFX_STANDARD_CHORUS: GUID =
    GUID::new(0xEFE6_629C, 0x81F7, 0x4281, [0xBD, 0x91, 0xC9, 0xD6, 0x04, 0xA9, 0x5A, 0xF6]);
pub(super) const GUID_DSFX_STANDARD_FLANGER: GUID =
    GUID::new(0xEFCA_3D92, 0xDFD8, 0x4672, [0xA6, 0x03, 0x74, 0x20, 0x89, 0x4B, 0xAD, 0x98]);
pub(super) const GUID_DSFX_STANDARD_ECHO: GUID =
    GUID::new(0xEF3E_932C, 0xD40B, 0x4F51, [0x8C, 0xCF, 0x3F, 0x98, 0xF1, 0xB2, 0x9D, 0x5D]);
pub(super) const GUID_DSFX_STANDARD_DISTORTION: GUID =
    GUID::new(0xEF11_4C90, 0xCD1D, 0x484E, [0x96, 0xE5, 0x09, 0xCF, 0xAF, 0x91, 0x2A, 0x21]);
pub(super) const GUID_DSFX_STANDARD_COMPRESSOR: GUID =
    GUID::new(0xEF01_1F79, 0x4000, 0x406D, [0x87, 0xAF, 0xBF, 0xFB, 0x3F, 0xC3, 0x9D, 0x57]);
pub(super) const GUID_DSFX_STANDARD_PARAMEQ: GUID =
    GUID::new(0x120C_ED89, 0x3BF4, 0x4173, [0xA1, 0x32, 0x3C, 0xB4, 0x06, 0xCF, 0x32, 0x31]);
pub(super) const GUID_DSFX_STANDARD_I3DL2REVERB: GUID =
    GUID::new(0xEF98_5E71, 0xD5C7, 0x42D4, [0xBA, 0x4D, 0x2D, 0x07, 0x3E, 0x2E, 0x96, 0xF4]);
pub(super) const GUID_DSFX_WAVES_REVERB: GUID =
    GUID::new(0x87FC_0268, 0x9A55, 0x4360, [0x95, 0xAA, 0x00, 0x4A, 0x1D, 0x9D, 0xE2, 0x6C]);

// DirectSound 8 FX parameter interface IIDs (<dsound.h>)
pub(super) const IID_IDIRECTSOUNDFXGARGLE: GUID =
    GUID::new(0xD616_F352, 0xD622, 0x11CE, [0xAA, 0xC5, 0x00, 0x20, 0xAF, 0x0B, 0x99, 0xA3]);
pub(super) const IID_IDIRECTSOUNDFXCHORUS: GUID =
    GUID::new(0x8808_42E3, 0x145F, 0x43E6, [0xA9, 0x34, 0xA7, 0x18, 0x06, 0xE5, 0x05, 0x47]);
pub(super) const IID_IDIRECTSOUNDFXFLANGER: GUID =
    GUID::new(0x903E_9878, 0x2C92, 0x4072, [0x9B, 0x2C, 0xEA, 0x68, 0xF5, 0x39, 0x67, 0x83]);
pub(super) const IID_IDIRECTSOUNDFXECHO: GUID =
    GUID::new(0x8BD2_8EDF, 0x50DB, 0x4E92, [0xA2, 0xBD, 0x44, 0x54, 0x88, 0xD1, 0xED, 0x42]);
pub(super) const IID_IDIRECTSOUNDFXDISTORTION: GUID =
    GUID::new(0x8ECF_4326, 0x455F, 0x4D8B, [0xBD, 0xA9, 0x8D, 0x5D, 0x3E, 0x9E, 0x3E, 0x0B]);
pub(super) const IID_IDIRECTSOUNDFXCOMPRESSOR: GUID =
    GUID::new(0x4BBD_1154, 0x62F6, 0x4E2C, [0xA1, 0x5C, 0xD3, 0xB6, 0xC4, 0x17, 0xF7, 0xA0]);
pub(super) const IID_IDIRECTSOUNDFXPARAMEQ: GUID =
    GUID::new(0xC03C_A9FE, 0xFE90, 0x4204, [0x80, 0x78, 0x82, 0x33, 0x4C, 0xD1, 0x77, 0xDA]);
pub(super) const IID_IDIRECTSOUNDFXI3DL2REVERB: GUID =
    GUID::new(0x4B16_6A6A, 0x0D66, 0x43F3, [0x80, 0xE3, 0xEE, 0x62, 0x80, 0xDE, 0xE1, 0xA4]);
pub(super) const IID_IDIRECTSOUNDFXWAVESREVERB: GUID =
    GUID::new(0x4685_8C3A, 0x0DC6, 0x45E3, [0xB7, 0x60, 0xD4, 0xEE, 0xF1, 0x6C, 0xB3, 0x25]);

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct DsEffectDesc {
    pub(super) dw_size: u32,
    pub(super) dw_flags: u32,
    pub(super) guid_dsfx_class: GUID,
    pub(super) dw_reserved1: usize,
    pub(super) dw_reserved2: usize,
}
const _: () = assert!(std::mem::size_of::<DsEffectDesc>() == 40);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum DsFxKind {
    Gargle,
    Chorus,
    Flanger,
    Echo,
    Distortion,
    Compressor,
    ParamEq,
    I3dl2Reverb,
    WavesReverb,
}

impl DsFxKind {
    pub(super) fn from_clsid(g: GUID) -> Option<Self> {
        if g == GUID_DSFX_STANDARD_GARGLE { Some(Self::Gargle) }
        else if g == GUID_DSFX_STANDARD_CHORUS { Some(Self::Chorus) }
        else if g == GUID_DSFX_STANDARD_FLANGER { Some(Self::Flanger) }
        else if g == GUID_DSFX_STANDARD_ECHO { Some(Self::Echo) }
        else if g == GUID_DSFX_STANDARD_DISTORTION { Some(Self::Distortion) }
        else if g == GUID_DSFX_STANDARD_COMPRESSOR { Some(Self::Compressor) }
        else if g == GUID_DSFX_STANDARD_PARAMEQ { Some(Self::ParamEq) }
        else if g == GUID_DSFX_STANDARD_I3DL2REVERB { Some(Self::I3dl2Reverb) }
        else if g == GUID_DSFX_WAVES_REVERB { Some(Self::WavesReverb) }
        else { None }
    }

    pub(super) fn clsid(self) -> GUID {
        match self {
            Self::Gargle => GUID_DSFX_STANDARD_GARGLE,
            Self::Chorus => GUID_DSFX_STANDARD_CHORUS,
            Self::Flanger => GUID_DSFX_STANDARD_FLANGER,
            Self::Echo => GUID_DSFX_STANDARD_ECHO,
            Self::Distortion => GUID_DSFX_STANDARD_DISTORTION,
            Self::Compressor => GUID_DSFX_STANDARD_COMPRESSOR,
            Self::ParamEq => GUID_DSFX_STANDARD_PARAMEQ,
            Self::I3dl2Reverb => GUID_DSFX_STANDARD_I3DL2REVERB,
            Self::WavesReverb => GUID_DSFX_WAVES_REVERB,
        }
    }

    pub(super) fn param_iid(self) -> GUID {
        match self {
            Self::Gargle => IID_IDIRECTSOUNDFXGARGLE,
            Self::Chorus => IID_IDIRECTSOUNDFXCHORUS,
            Self::Flanger => IID_IDIRECTSOUNDFXFLANGER,
            Self::Echo => IID_IDIRECTSOUNDFXECHO,
            Self::Distortion => IID_IDIRECTSOUNDFXDISTORTION,
            Self::Compressor => IID_IDIRECTSOUNDFXCOMPRESSOR,
            Self::ParamEq => IID_IDIRECTSOUNDFXPARAMEQ,
            Self::I3dl2Reverb => IID_IDIRECTSOUNDFXI3DL2REVERB,
            Self::WavesReverb => IID_IDIRECTSOUNDFXWAVESREVERB,
        }
    }

    pub(super) fn default_params(self) -> Vec<u8> {
        let mut v = Vec::new();
        match self {
            Self::Gargle => {
                // DSFXGargle: dwRateHz(20), dwWaveShape(0=TRIANGLE)
                v.extend_from_slice(&20u32.to_le_bytes());
                v.extend_from_slice(&0u32.to_le_bytes());
            }
            Self::Chorus => {
                // DSFXChorus (28 bytes): fWetDryMix(50), fDepth(10), fFeedback(25), fFrequency(1.1), lWaveform(1), fDelay(16), lPhase(3)
                for f in [50.0f32, 10.0, 25.0, 1.1] { v.extend_from_slice(&f.to_le_bytes()); }
                v.extend_from_slice(&1i32.to_le_bytes());
                v.extend_from_slice(&16.0f32.to_le_bytes());
                v.extend_from_slice(&3i32.to_le_bytes());
            }
            Self::Flanger => {
                // DSFXFlanger (28 bytes): fWetDryMix(50), fDepth(100), fFeedback(-50), fFrequency(0.25), lWaveform(1), fDelay(2), lPhase(2)
                for f in [50.0f32, 100.0, -50.0, 0.25] { v.extend_from_slice(&f.to_le_bytes()); }
                v.extend_from_slice(&1i32.to_le_bytes());
                v.extend_from_slice(&2.0f32.to_le_bytes());
                v.extend_from_slice(&2i32.to_le_bytes());
            }
            Self::Echo => {
                // DSFXEcho (20 bytes): fWetDryMix(50), fFeedback(50), fLeftDelay(500), fRightDelay(500), lPanDelay(0)
                for f in [50.0f32, 50.0, 500.0, 500.0] { v.extend_from_slice(&f.to_le_bytes()); }
                v.extend_from_slice(&0i32.to_le_bytes());
            }
            Self::Distortion => {
                // DSFXDistortion (20 bytes): fGain(-18), fEdge(15), fPostEQCenterFrequency(2400), fPostEQBandwidth(2400), fPreLowpassCutoff(8000)
                for f in [-18.0f32, 15.0, 2400.0, 2400.0, 8000.0] { v.extend_from_slice(&f.to_le_bytes()); }
            }
            Self::Compressor => {
                // DSFXCompressor (24 bytes): fGain(0), fAttack(10), fRelease(200), fThreshold(-20), fRatio(3), fPredelay(4)
                for f in [0.0f32, 10.0, 200.0, -20.0, 3.0, 4.0] { v.extend_from_slice(&f.to_le_bytes()); }
            }
            Self::ParamEq => {
                // DSFXParamEq (12 bytes): fCenter(8000), fBandwidth(12), fGain(0)
                for f in [8000.0f32, 12.0, 0.0] { v.extend_from_slice(&f.to_le_bytes()); }
            }
            Self::I3dl2Reverb => {
                // DSFXI3DL2Reverb (48 bytes)
                v.extend_from_slice(&(-1000i32).to_le_bytes()); // lRoom
                v.extend_from_slice(&(-100i32).to_le_bytes()); // lRoomHF
                v.extend_from_slice(&0.0f32.to_le_bytes()); // flRoomRolloffFactor
                v.extend_from_slice(&1.49f32.to_le_bytes()); // flDecayTime
                v.extend_from_slice(&0.83f32.to_le_bytes()); // flDecayHFRatio
                v.extend_from_slice(&(-2602i32).to_le_bytes()); // lReflections
                v.extend_from_slice(&0.007f32.to_le_bytes()); // flReflectionsDelay
                v.extend_from_slice(&200i32.to_le_bytes()); // lReverb
                v.extend_from_slice(&0.011f32.to_le_bytes()); // flReverbDelay
                v.extend_from_slice(&100.0f32.to_le_bytes()); // flDiffusion
                v.extend_from_slice(&100.0f32.to_le_bytes()); // flDensity
                v.extend_from_slice(&5000.0f32.to_le_bytes()); // flHFReference
            }
            Self::WavesReverb => {
                // DSFXWavesReverb (16 bytes): fInGain(0), fReverbMix(0), fReverbTime(1000), fHighFreqRTRatio(0.001)
                for f in [0.0f32, 0.0, 1000.0, 0.001] { v.extend_from_slice(&f.to_le_bytes()); }
            }
        }
        v
    }
}

pub(super) struct BufferEffect {
    pub(super) kind: DsFxKind,
    pub(super) params: Vec<u8>,
    delay_l: Vec<f32>,
    delay_r: Vec<f32>,
    delay_pos: usize,
    phase: f32,
    env: f32,
    z1_l: f32,
    z2_l: f32,
    z1_r: f32,
    z2_r: f32,
}

impl BufferEffect {
    pub(super) fn new(kind: DsFxKind) -> Self {
        let cap = match kind {
            DsFxKind::Echo | DsFxKind::WavesReverb | DsFxKind::I3dl2Reverb => 96_000,
            DsFxKind::Chorus | DsFxKind::Flanger => 4_096,
            _ => 0,
        };
        Self {
            kind,
            params: kind.default_params(),
            delay_l: vec![0.0; cap],
            delay_r: vec![0.0; cap],
            delay_pos: 0,
            phase: 0.0,
            env: 0.0,
            z1_l: 0.0,
            z2_l: 0.0,
            z1_r: 0.0,
            z2_r: 0.0,
        }
    }

    #[inline]
    fn param_f32(&self, off: usize) -> f32 {
        self.params
            .get(off..off + 4)
            .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .unwrap_or(0.0)
    }

    #[inline]
    fn param_u32(&self, off: usize) -> u32 {
        self.params
            .get(off..off + 4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .unwrap_or(0)
    }

    /// Process one stereo sample through this DirectSound 8 software DSP effect.
    pub(super) fn process_sample(&mut self, l: f32, r: f32, sample_rate: f32) -> (f32, f32) {
        let sr = sample_rate.max(8_000.0);
        match self.kind {
            DsFxKind::Echo => {
                let wet = (self.param_f32(0).clamp(0.0, 100.0)) * 0.01;
                let fb = (self.param_f32(4).clamp(0.0, 99.0)) * 0.01;
                let dl = ((self.param_f32(8).clamp(1.0, 2000.0) * 0.001 * sr) as usize)
                    .clamp(1, self.delay_l.len().saturating_sub(1));
                let dr = ((self.param_f32(12).clamp(1.0, 2000.0) * 0.001 * sr) as usize)
                    .clamp(1, self.delay_r.len().saturating_sub(1));
                let cap = self.delay_l.len();
                let rl = self.delay_l[(self.delay_pos + cap - dl) % cap];
                let rr = self.delay_r[(self.delay_pos + cap - dr) % cap];
                self.delay_l[self.delay_pos] = (l + rl * fb).clamp(-4.0, 4.0);
                self.delay_r[self.delay_pos] = (r + rr * fb).clamp(-4.0, 4.0);
                self.delay_pos = (self.delay_pos + 1) % cap;
                (l * (1.0 - wet) + rl * wet, r * (1.0 - wet) + rr * wet)
            }
            DsFxKind::Chorus | DsFxKind::Flanger => {
                let wet = (self.param_f32(0).clamp(0.0, 100.0)) * 0.01;
                let depth = (self.param_f32(4).clamp(0.0, 100.0)) * 0.01;
                let fb = (self.param_f32(8).clamp(-99.0, 99.0)) * 0.01;
                let freq = self.param_f32(12).clamp(0.0, 10.0);
                let base_ms = self.param_f32(20).clamp(0.0, 20.0);
                let cap = self.delay_l.len();
                let lfo = (self.phase * std::f32::consts::TAU).sin() * 0.5 + 0.5;
                self.phase = (self.phase + freq / sr).fract();
                let d_samp = ((base_ms * (0.5 + 0.5 * depth * lfo) * 0.001 * sr) as usize)
                    .clamp(1, cap - 1);
                let rl = self.delay_l[(self.delay_pos + cap - d_samp) % cap];
                let rr = self.delay_r[(self.delay_pos + cap - d_samp) % cap];
                self.delay_l[self.delay_pos] = (l + rl * fb).clamp(-4.0, 4.0);
                self.delay_r[self.delay_pos] = (r + rr * fb).clamp(-4.0, 4.0);
                self.delay_pos = (self.delay_pos + 1) % cap;
                (l * (1.0 - wet) + rl * wet, r * (1.0 - wet) + rr * wet)
            }
            DsFxKind::Distortion => {
                let gain = 10.0f32.powf(self.param_f32(0).clamp(-60.0, 0.0) / 20.0);
                let edge = 1.0 + (self.param_f32(4).clamp(0.0, 100.0) * 0.1);
                ((l * edge).tanh() * gain, (r * edge).tanh() * gain)
            }
            DsFxKind::Gargle => {
                let rate_hz = self.param_u32(0).clamp(1, 1000) as f32;
                let shape = self.param_u32(4);
                let p = self.phase;
                self.phase = (self.phase + rate_hz / sr).fract();
                let mod_gain = if shape == 1 {
                    if p < 0.5 { 1.0 } else { 0.0 }
                } else if p < 0.5 {
                    p * 2.0
                } else {
                    (1.0 - p) * 2.0
                };
                (l * mod_gain, r * mod_gain)
            }
            DsFxKind::Compressor => {
                let gain = 10.0f32.powf(self.param_f32(0).clamp(-60.0, 60.0) / 20.0);
                let thresh = 10.0f32.powf(self.param_f32(12).clamp(-60.0, 0.0) / 20.0);
                let ratio = self.param_f32(16).clamp(1.0, 100.0);
                let peak = l.abs().max(r.abs());
                let coeff = if peak > self.env { 0.01 } else { 0.001 };
                self.env += coeff * (peak - self.env);
                let comp = if self.env > thresh && self.env > 1e-6 {
                    (thresh + (self.env - thresh) / ratio) / self.env
                } else {
                    1.0
                };
                (l * comp * gain, r * comp * gain)
            }
            DsFxKind::ParamEq => {
                let fc = self.param_f32(0).clamp(80.0, (sr * 0.45).min(16_000.0));
                let bw_semi = self.param_f32(4).clamp(1.0, 36.0);
                let gain_db = self.param_f32(8).clamp(-15.0, 15.0);
                if gain_db.abs() < 1e-4 {
                    return (l, r);
                }
                let a = 10.0f32.powf(gain_db / 40.0);
                let w0 = std::f32::consts::TAU * fc / sr;
                let cos_w0 = w0.cos();
                let sin_w0 = w0.sin();
                let q = (1.0 / (2.0 * (bw_semi * 0.0833333 * 0.693147 * 0.5).sinh())).clamp(0.1, 20.0);
                let alpha = sin_w0 / (2.0 * q);
                let b0 = 1.0 + alpha * a;
                let b1 = -2.0 * cos_w0;
                let b2 = 1.0 - alpha * a;
                let a0 = 1.0 + alpha / a;
                let a1 = -2.0 * cos_w0;
                let a2 = 1.0 - alpha / a;
                let inv_a0 = 1.0 / a0;
                let yl = (b0 * l + self.z1_l) * inv_a0;
                self.z1_l = b1 * l - a1 * yl + self.z2_l;
                self.z2_l = b2 * l - a2 * yl;
                let yr = (b0 * r + self.z1_r) * inv_a0;
                self.z1_r = b1 * r - a1 * yr + self.z2_r;
                self.z2_r = b2 * r - a2 * yr;
                (yl, yr)
            }
            DsFxKind::WavesReverb | DsFxKind::I3dl2Reverb => {
                let (in_gain, wet, decay_ms) = if self.kind == DsFxKind::WavesReverb {
                    let ig = 10.0f32.powf(self.param_f32(0).clamp(-96.0, 0.0) / 20.0);
                    let mix = 10.0f32.powf(self.param_f32(4).clamp(-96.0, 0.0) / 20.0).min(1.0);
                    let rt = self.param_f32(8).clamp(1.0, 3000.0);
                    (ig, mix * 0.5, rt)
                } else {
                    let room = 10.0f32.powf((self.param_u32(0) as i32).clamp(-10000, 0) as f32 / 2000.0);
                    let rt = self.param_f32(12).clamp(0.1, 20.0) * 1000.0;
                    (1.0, (room * 0.5).min(0.8), rt)
                };
                let cap = self.delay_l.len();
                let dl = ((0.037 * sr) as usize).clamp(1, cap - 1);
                let dr = ((0.041 * sr) as usize).clamp(1, cap - 1);
                let fb = 10.0f32.powf((-3.0 * 37.0) / decay_ms.max(40.0)).clamp(0.0, 0.92);
                let rl = self.delay_l[(self.delay_pos + cap - dl) % cap];
                let rr = self.delay_r[(self.delay_pos + cap - dr) % cap];
                // Subtle cross-feed + 1-pole damping for natural stereo tail.
                self.z1_l += 0.35 * (rl - self.z1_l);
                self.z1_r += 0.35 * (rr - self.z1_r);
                self.delay_l[self.delay_pos] = (l * in_gain + self.z1_r * fb).clamp(-4.0, 4.0);
                self.delay_r[self.delay_pos] = (r * in_gain + self.z1_l * fb).clamp(-4.0, 4.0);
                self.delay_pos = (self.delay_pos + 1) % cap;
                (l * in_gain + self.z1_l * wet, r * in_gain + self.z1_r * wet)
            }
        }
    }
}

// 3D constants
pub(super) const DS3DMODE_NORMAL: u32 = 0;
pub(super) const DS3DMODE_HEADRELATIVE: u32 = 1;
pub(super) const DS3DMODE_DISABLE: u32 = 2;
pub(super) const DS3D_IMMEDIATE: u32 = 0;
pub(super) const DS3D_DEFERRED: u32 = 1;
pub(super) const DS3D_MINROLLOFFFACTOR: f32 = 0.0;
pub(super) const DS3D_MAXROLLOFFFACTOR: f32 = 10.0;
pub(super) const DS3D_MINDOPPLERFACTOR: f32 = 0.0;
pub(super) const DS3D_MAXDOPPLERFACTOR: f32 = 10.0;
pub(super) const DS3D_DEFAULTMINDISTANCE: f32 = 1.0;
pub(super) const DS3D_DEFAULTMAXDISTANCE: f32 = 1_000_000_000.0;
pub(super) const DS3D_MAXCONEANGLE: u32 = 360;
pub(super) const SPEED_OF_SOUND: f32 = 343.5;

// Capture constants
pub(super) const DSCCAPS_MULTIPLECAPTURE: u32 = 0x0000_0001;
pub(super) const DSCCAPS_CERTIFIED: u32 = 0x0000_0040;
pub(super) const DSCBSTART_LOOPING: u32 = 0x0000_0001;
pub(super) const DSCBLOCK_ENTIREBUFFER: u32 = 0x0000_0001;
pub(super) const DSCBSTATUS_CAPTURING: u32 = 0x0000_0001;
pub(super) const DSCBSTATUS_LOOPING: u32 = 0x0000_0002;
pub(super) const STANDARD_CAPTURE_FORMATS: u32 = 0x000F_FFFF;

// ============================================================================
// Windows ABI Structures (Win64 natural alignment; WAVEFORMAT* packed(2))
// ============================================================================

/// Legacy 16-byte `PCMWAVEFORMAT` prefix used to parse non-extensible formats
/// without reading past caller-allocated 16-byte buffers.
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

pub(super) type D3dVector = Vec3;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(super) struct DsCaps {
    pub(super) dw_size: u32,
    pub(super) dw_flags: u32,
    pub(super) dw_min_secondary_sample_rate: u32,
    pub(super) dw_max_secondary_sample_rate: u32,
    pub(super) dw_primary_buffers: u32,
    pub(super) dw_max_hw_mixing_all_buffers: u32,
    pub(super) dw_max_hw_mixing_static_buffers: u32,
    pub(super) dw_max_hw_mixing_streaming_buffers: u32,
    pub(super) dw_free_hw_mixing_all_buffers: u32,
    pub(super) dw_free_hw_mixing_static_buffers: u32,
    pub(super) dw_free_hw_mixing_streaming_buffers: u32,
    pub(super) dw_max_hw_3d_all_buffers: u32,
    pub(super) dw_max_hw_3d_static_buffers: u32,
    pub(super) dw_max_hw_3d_streaming_buffers: u32,
    pub(super) dw_free_hw_3d_all_buffers: u32,
    pub(super) dw_free_hw_3d_static_buffers: u32,
    pub(super) dw_free_hw_3d_streaming_buffers: u32,
    pub(super) dw_total_hw_mem_bytes: u32,
    pub(super) dw_free_hw_mem_bytes: u32,
    pub(super) dw_max_contig_free_hw_mem_bytes: u32,
    pub(super) dw_unlock_transfer_rate_hw_buffers: u32,
    pub(super) dw_play_cpu_overhead_sw_buffers: u32,
    pub(super) dw_reserved1: u32,
    pub(super) dw_reserved2: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(super) struct DsbCaps {
    pub(super) dw_size: u32,
    pub(super) dw_flags: u32,
    pub(super) dw_buffer_bytes: u32,
    pub(super) dw_unlock_transfer_rate: u32,
    pub(super) dw_play_cpu_overhead: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct DsBufferDesc1 {
    pub(super) dw_size: u32,
    pub(super) dw_flags: u32,
    pub(super) dw_buffer_bytes: u32,
    pub(super) dw_reserved: u32,
    pub(super) lpwfx_format: *const u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct DsBufferDesc {
    pub(super) dw_size: u32,
    pub(super) dw_flags: u32,
    pub(super) dw_buffer_bytes: u32,
    pub(super) dw_reserved: u32,
    pub(super) lpwfx_format: *const u8,
    pub(super) guid_3d_algorithm: GUID,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(super) struct DscCaps {
    pub(super) dw_size: u32,
    pub(super) dw_flags: u32,
    pub(super) dw_formats: u32,
    pub(super) dw_channels: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(super) struct DscbCaps {
    pub(super) dw_size: u32,
    pub(super) dw_flags: u32,
    pub(super) dw_buffer_bytes: u32,
    pub(super) dw_reserved: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct DscBufferDesc1 {
    pub(super) dw_size: u32,
    pub(super) dw_flags: u32,
    pub(super) dw_buffer_bytes: u32,
    pub(super) dw_reserved: u32,
    pub(super) lpwfx_format: *const u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct DscBufferDesc {
    pub(super) dw_size: u32,
    pub(super) dw_flags: u32,
    pub(super) dw_buffer_bytes: u32,
    pub(super) dw_reserved: u32,
    pub(super) lpwfx_format: *const u8,
    pub(super) dw_fx_count: u32,
    pub(super) lp_dsc_fx_desc: *const c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct Ds3dBuffer {
    pub(super) dw_size: u32,
    pub(super) v_position: D3dVector,
    pub(super) v_velocity: D3dVector,
    pub(super) dw_inside_cone_angle: u32,
    pub(super) dw_outside_cone_angle: u32,
    pub(super) v_cone_orientation: D3dVector,
    pub(super) l_cone_outside_volume: i32,
    pub(super) fl_min_distance: f32,
    pub(super) fl_max_distance: f32,
    pub(super) dw_mode: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct Ds3dListener {
    pub(super) dw_size: u32,
    pub(super) v_position: D3dVector,
    pub(super) v_velocity: D3dVector,
    pub(super) v_orient_front: D3dVector,
    pub(super) v_orient_top: D3dVector,
    pub(super) fl_distance_factor: f32,
    pub(super) fl_rolloff_factor: f32,
    pub(super) fl_doppler_factor: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct DsbPositionNotify {
    pub(super) dw_offset: u32,
    pub(super) h_event_notify: HANDLE,
}

const _: () = assert!(std::mem::size_of::<PcmWaveFormat>() == 16);
const _: () = assert!(std::mem::size_of::<WAVEFORMATEX>() == 18);
const _: () = assert!(std::mem::size_of::<WAVEFORMATEXTENSIBLE>() == 40);
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
// PCM Format Parsing & Sample Storage (backed by `crate::audio::format`)
// ============================================================================

pub(super) fn default_primary_format() -> WaveFormat {
    WaveFormat {
        rate: 48_000,
        channels: 2,
        bits: 16,
        valid_bits: 16,
        block_align: 4,
        mask: 0x3,
        sample: SampleType::I16,
        extensible: false,
    }
}

/// Parse a DirectSound `PCMWAVEFORMAT` (16B), `WAVEFORMATEX` (18B), or
/// `WAVEFORMATEXTENSIBLE` (40B) into [`WaveFormat`].
pub(super) unsafe fn parse_ds_format(ptr: *const u8) -> Result<WaveFormat, HRESULT> {
    if ptr.is_null() {
        return Err(DSERR_INVALIDPARAM);
    }
    let pcm = rd(ptr as *const PcmWaveFormat);
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
        let wfx = rd(ptr as *const WAVEFORMATEX);
        if wfx.cbSize < 22 {
            return Err(DSERR_BADFORMAT);
        }
        let ext = rd(ptr as *const WAVEFORMATEXTENSIBLE);
        let sub = { ext.SubFormat };
        let sub_tag = if sub == KSDATAFORMAT_SUBTYPE_PCM {
            WAVE_FORMAT_PCM
        } else if sub == KSDATAFORMAT_SUBTYPE_IEEE_FLOAT {
            WAVE_FORMAT_IEEE_FLOAT
        } else {
            return Err(DSERR_BADFORMAT);
        };
        let vb = if ext.wValidBitsPerSample == 0 {
            bits
        } else {
            ext.wValidBitsPerSample
        };
        if vb > bits {
            return Err(DSERR_BADFORMAT);
        }
        (sub_tag, vb, ext.dwChannelMask, true)
    } else {
        (raw_tag, bits, format::default_mask(channels), false)
    };

    let sample = match (tag, bits) {
        (WAVE_FORMAT_PCM, 8) => SampleType::U8,
        (WAVE_FORMAT_PCM, 16) => SampleType::I16,
        (WAVE_FORMAT_PCM, 24) => SampleType::I24,
        (WAVE_FORMAT_PCM, 32) => SampleType::I32,
        (WAVE_FORMAT_IEEE_FLOAT, 32) => SampleType::F32,
        (WAVE_FORMAT_IEEE_FLOAT, 64) => SampleType::F64,
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

    Ok(WaveFormat {
        rate: sample_rate,
        channels,
        bits,
        valid_bits,
        block_align: align,
        mask: channel_mask,
        sample,
        extensible,
    })
}

/// Write a DirectSound buffer format (`GetFormat`), delegating serialization to
/// [`WaveFormat::write_to`].
pub(super) unsafe fn write_ds_format(
    fmt: &WaveFormat,
    out: *mut u8,
    allocated: u32,
    written_out: *mut u32,
) -> HRESULT {
    let req = fmt.encoded_size() as u32;
    if out.is_null() {
        if written_out.is_null() {
            return DSERR_INVALIDPARAM;
        }
        wr(written_out, req);
        return DS_OK;
    }
    let Some(n) = fmt.write_to(out, allocated as usize) else {
        return DSERR_INVALIDPARAM;
    };
    if !written_out.is_null() {
        wr(written_out, n as u32);
    }
    DS_OK
}

/// Fixed-address circular PCM storage shared across duplicated buffers.
///
/// Using `Box<[AtomicU8]>` guarantees pointer stability across `Lock`/`Unlock`
/// and eliminates Rust data-race UB when the guest streams new PCM bytes into
/// a locked write region while the background mixer reads ahead of the play cursor.
pub(super) struct SampleMemory {
    data: Box<[AtomicU8]>,
    pub(super) format: WaveFormat,
    pub(super) total_frames: usize,
    /// Row-major `[in_ch][2]` matrix mapping `format.channels` -> stereo `(L, R)`.
    stereo_matrix: Vec<f32>,
}

impl SampleMemory {
    pub(super) fn new(byte_len: usize, format: WaveFormat) -> Arc<Self> {
        let silence_byte = if format.sample == SampleType::U8 {
            0x80u8
        } else {
            0x00u8
        };
        let data: Box<[AtomicU8]> = (0..byte_len).map(|_| AtomicU8::new(silence_byte)).collect();
        let block = (format.block_align as usize).max(1);
        let total_frames = byte_len / block;
        let stereo_matrix = if format.channels > 2 {
            format::channel_matrix(format.channels, format.mask, 2, 0x3)
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
    pub(super) fn byte_len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub(super) fn base_mut_ptr(&self) -> *mut u8 {
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
        let bps = self.format.sample.bytes_per_sample();
        let off = frame_idx * (self.format.block_align as usize) + ch * bps;
        let mut raw = [0u8; 8];
        for (i, slot) in raw[..bps].iter_mut().enumerate() {
            *slot = self.load_u8(off + i);
        }
        decode_pcm_sample_f32(self.format.sample, &raw[..bps])
    }

    /// Read one frame as stereo `(L, R)` `f32`.
    #[inline]
    pub(super) fn read_stereo_frame(&self, frame_idx: usize) -> (f32, f32) {
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
// Device GUID Derivation, Resolution & Enumeration
// ============================================================================

pub(super) fn device_guid_for(uid: &str, capture: bool) -> GUID {
    // Deterministic 128-bit FNV-1a derivation so GUIDs are stable across calls
    // and never collide with GUID_NULL or DSDEVID_Default*.
    let tag: &[u8] = if capture {
        b"dsound-capture:"
    } else {
        b"dsound-render:"
    };
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

pub(super) fn is_headless() -> bool {
    std::env::var("PORT_HEADLESS").is_ok_and(|v| v == "1")
        || std::env::var("AW2_HEADLESS").is_ok_and(|v| v == "1")
}

pub(super) fn resolve_render_device(guid_opt: Option<GUID>) -> Result<DeviceInfo, HRESULT> {
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

pub(super) fn resolve_capture_device(guid_opt: Option<GUID>) -> Result<DeviceInfo, HRESULT> {
    let devs = backend::input_devices();
    let default_dev = devs
        .first()
        .cloned()
        .unwrap_or_else(backend::null_capture_device);
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

pub(super) unsafe fn enumerate_devices_impl(
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
// 3D Parameters, Notifications, Locks & Buffer State
// ============================================================================

#[derive(Clone, Copy)]
pub(super) struct Listener3dParams {
    pub(super) position: D3dVector,
    pub(super) velocity: D3dVector,
    pub(super) orient_front: D3dVector,
    pub(super) orient_top: D3dVector,
    pub(super) distance_factor: f32,
    pub(super) rolloff_factor: f32,
    pub(super) doppler_factor: f32,
}

impl Default for Listener3dParams {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            orient_front: Vec3::new(0.0, 0.0, 1.0),
            orient_top: Vec3::new(0.0, 1.0, 0.0),
            distance_factor: 1.0,
            rolloff_factor: 1.0,
            doppler_factor: 1.0,
        }
    }
}

impl Listener3dParams {
    pub(super) fn to_abi(self) -> Ds3dListener {
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
pub(super) struct Buffer3dParams {
    pub(super) position: D3dVector,
    pub(super) velocity: D3dVector,
    pub(super) inside_cone_angle: u32,
    pub(super) outside_cone_angle: u32,
    pub(super) cone_orientation: D3dVector,
    pub(super) cone_outside_volume: i32,
    pub(super) min_distance: f32,
    pub(super) max_distance: f32,
    pub(super) mode: u32,
}

impl Default for Buffer3dParams {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            inside_cone_angle: DS3D_MAXCONEANGLE,
            outside_cone_angle: DS3D_MAXCONEANGLE,
            cone_orientation: Vec3::new(0.0, 0.0, 1.0),
            cone_outside_volume: DSBVOLUME_MAX,
            min_distance: DS3D_DEFAULTMINDISTANCE,
            max_distance: DS3D_DEFAULTMAXDISTANCE,
            mode: DS3DMODE_NORMAL,
        }
    }
}

impl Buffer3dParams {
    pub(super) fn to_abi(self) -> Ds3dBuffer {
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

#[derive(Clone)]
pub(super) struct NotifyEntry {
    pub(super) offset: u32,
    pub(super) event: Arc<crate::handles::Event>,
}

#[derive(Clone, Copy)]
pub(super) struct ActiveLock {
    pub(super) ptr1: usize,
    pub(super) len1: u32,
    pub(super) ptr2: usize,
    pub(super) len2: u32,
}

pub(super) fn check_offset_crossed(
    prev: u32,
    next: u32,
    buf_len: u32,
    wrapped: bool,
    target: u32,
) -> bool {
    if target >= buf_len {
        return false;
    }
    if !wrapped {
        target >= prev && target < next
    } else {
        target >= prev || target < next
    }
}

pub(super) fn signal_events(events: Vec<Arc<crate::handles::Event>>) {
    for ev in events {
        ev.set();
    }
}

pub(super) struct SecondaryBufferState {
    pub(super) flags: u32,
    pub(super) sample_mem: Arc<SampleMemory>,
    pub(super) frequency: u32,
    pub(super) volume_mb: i32,
    pub(super) pan_mb: i32,
    pub(super) playing: bool,
    pub(super) looping: bool,
    pub(super) play_pos_frames: f64,
    pub(super) d3d_current: Buffer3dParams,
    pub(super) d3d_deferred: Buffer3dParams,
    pub(super) locks: Vec<ActiveLock>,
    pub(super) notifies: Vec<NotifyEntry>,
    pub(super) effects: Vec<Arc<std::sync::Mutex<BufferEffect>>>,
}

impl SecondaryBufferState {
    pub(super) fn new(flags: u32, sample_mem: Arc<SampleMemory>) -> Self {
        let freq = sample_mem.format.rate;
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
            effects: Vec::new(),
        }
    }

    pub(super) fn current_cursors(&self) -> (u32, u32) {
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

    pub(super) fn collect_stop_events(&self, out: &mut Vec<Arc<crate::handles::Event>>) {
        for n in &self.notifies {
            if n.offset == DSBPN_OFFSETSTOP {
                out.push(n.event.clone());
            }
        }
    }
}

pub(super) struct CaptureBufferState {
    pub(super) flags: u32,
    pub(super) sample_mem: Arc<SampleMemory>,
    pub(super) capturing: bool,
    pub(super) looping: bool,
    pub(super) base_frames: u64,
    pub(super) start_instant: Option<Instant>,
    pub(super) locks: Vec<ActiveLock>,
    pub(super) notifies: Vec<NotifyEntry>,
}

impl CaptureBufferState {
    pub(super) fn new(flags: u32, sample_mem: Arc<SampleMemory>) -> Self {
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

    pub(super) fn advance_and_cursors(&mut self) -> (u32, u32, Vec<Arc<crate::handles::Event>>) {
        let mut events = Vec::new();
        let total = self.sample_mem.total_frames;
        let block = self.sample_mem.format.block_align as u32;
        let buf_len = self.sample_mem.byte_len() as u32;
        if total == 0 || block == 0 || buf_len == 0 {
            return (0, 0, events);
        }
        let sr = self.sample_mem.format.rate.max(1) as f64;
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
        let lead = ((self.sample_mem.format.rate as usize) / 100).min(total / 4);
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
