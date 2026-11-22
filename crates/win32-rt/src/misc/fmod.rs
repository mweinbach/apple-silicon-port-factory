//! Builtin `fmod.dll` / `fmod64.dll` / `fmodstudio.dll` / `fmodstudio64.dll`
//! (FMOD Core and FMOD Studio) audio middleware backed by `crate::audio::backend`
//! (CoreAudio AUHAL on macOS, RemoteIO on iOS, or Null when `AW2_AUDIO_BACKEND=null`).
//!
//! (OpenAL lives in `openal32.rs`.)

#![allow(non_snake_case)]

use crate::audio::backend::{self, OutputStream, StreamShared};
use crate::audio::ring::Ring;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::time::Duration;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ============================================================================
// FMOD 2.02 constants & ABI types
// ============================================================================

/// FMOD Core API version reported by `FMOD_System_GetVersion` (2.02.00).
pub const FMOD_VERSION: u32 = 0x0002_0200;

// FMOD_RESULT
pub const FMOD_OK: i32 = 0;
pub const FMOD_ERR_FILE_BAD: i32 = 13;
pub const FMOD_ERR_FILE_EOF: i32 = 16;
pub const FMOD_ERR_FILE_NOTFOUND: i32 = 18;
pub const FMOD_ERR_FORMAT: i32 = 19;
pub const FMOD_ERR_INITIALIZED: i32 = 27;
pub const FMOD_ERR_INVALID_FLOAT: i32 = 29;
pub const FMOD_ERR_INVALID_HANDLE: i32 = 30;
pub const FMOD_ERR_INVALID_PARAM: i32 = 31;
pub const FMOD_ERR_INVALID_POSITION: i32 = 32;
pub const FMOD_ERR_OUTPUT_INIT: i32 = 51;
pub const FMOD_ERR_TOOMANYCHANNELS: i32 = 64;
pub const FMOD_ERR_TRUNCATED: i32 = 65;
pub const FMOD_ERR_UNINITIALIZED: i32 = 67;
pub const FMOD_ERR_EVENT_ALREADY_LOADED: i32 = 70;
pub const FMOD_ERR_EVENT_NOTFOUND: i32 = 74;
pub const FMOD_ERR_STUDIO_UNINITIALIZED: i32 = 75;
pub const FMOD_ERR_STUDIO_NOT_LOADED: i32 = 76;
pub const FMOD_ERR_INVALID_STRING: i32 = 77;
pub const FMOD_ERR_ALREADY_LOCKED: i32 = 78;
pub const FMOD_ERR_NOT_LOCKED: i32 = 79;

// FMOD_MODE
pub const FMOD_DEFAULT: u32 = 0x0000_0000;
pub const FMOD_LOOP_OFF: u32 = 0x0000_0001;
pub const FMOD_LOOP_NORMAL: u32 = 0x0000_0002;
pub const FMOD_LOOP_BIDI: u32 = 0x0000_0004;
pub const FMOD_2D: u32 = 0x0000_0008;
pub const FMOD_3D: u32 = 0x0000_0010;
pub const FMOD_CREATESTREAM: u32 = 0x0000_0080;
pub const FMOD_OPENUSER: u32 = 0x0000_0400;
pub const FMOD_OPENMEMORY: u32 = 0x0000_0800;
pub const FMOD_OPENRAW: u32 = 0x0000_1000;
pub const FMOD_3D_HEADRELATIVE: u32 = 0x0004_0000;
pub const FMOD_3D_WORLDRELATIVE: u32 = 0x0008_0000;
pub const FMOD_3D_INVERSEROLLOFF: u32 = 0x0010_0000;
pub const FMOD_3D_LINEARROLLOFF: u32 = 0x0020_0000;
pub const FMOD_3D_LINEARSQUAREROLLOFF: u32 = 0x0040_0000;
pub const FMOD_OPENMEMORY_POINT: u32 = 0x1000_0000;

// FMOD_SOUND_TYPE
pub const FMOD_SOUND_TYPE_UNKNOWN: u32 = 0;
pub const FMOD_SOUND_TYPE_FSB: u32 = 5;
pub const FMOD_SOUND_TYPE_RAW: u32 = 12;
pub const FMOD_SOUND_TYPE_USER: u32 = 14;
pub const FMOD_SOUND_TYPE_WAV: u32 = 15;

// FMOD_SOUND_FORMAT
pub const FMOD_SOUND_FORMAT_NONE: u32 = 0;
pub const FMOD_SOUND_FORMAT_PCM8: u32 = 1;
pub const FMOD_SOUND_FORMAT_PCM16: u32 = 2;
pub const FMOD_SOUND_FORMAT_PCM24: u32 = 3;
pub const FMOD_SOUND_FORMAT_PCM32: u32 = 4;
pub const FMOD_SOUND_FORMAT_PCMFLOAT: u32 = 5;

// FMOD_SPEAKERMODE
pub const FMOD_SPEAKERMODE_DEFAULT: u32 = 0;
pub const FMOD_SPEAKERMODE_RAW: u32 = 1;
pub const FMOD_SPEAKERMODE_MONO: u32 = 2;
pub const FMOD_SPEAKERMODE_STEREO: u32 = 3;
pub const FMOD_SPEAKERMODE_QUAD: u32 = 4;
pub const FMOD_SPEAKERMODE_SURROUND: u32 = 5;
pub const FMOD_SPEAKERMODE_5POINT1: u32 = 6;
pub const FMOD_SPEAKERMODE_7POINT1: u32 = 7;

// FMOD_TIMEUNIT
pub const FMOD_TIMEUNIT_MS: u32 = 0x0000_0001;
pub const FMOD_TIMEUNIT_PCM: u32 = 0x0000_0002;
pub const FMOD_TIMEUNIT_PCMBYTES: u32 = 0x0000_0004;
pub const FMOD_TIMEUNIT_RAWBYTES: u32 = 0x0000_0008;

// FMOD_OUTPUTTYPE
pub const FMOD_OUTPUTTYPE_AUTODETECT: u32 = 0;
pub const FMOD_OUTPUTTYPE_UNKNOWN: u32 = 1;
pub const FMOD_OUTPUTTYPE_NOSOUND: u32 = 2;
pub const FMOD_OUTPUTTYPE_WAVWRITER: u32 = 3;
pub const FMOD_OUTPUTTYPE_NOSOUND_NRT: u32 = 4;
pub const FMOD_OUTPUTTYPE_WAVWRITER_NRT: u32 = 5;
pub const FMOD_OUTPUTTYPE_WASAPI: u32 = 6;
pub const FMOD_OUTPUTTYPE_COREAUDIO: u32 = 10;

// FMOD_STUDIO_LOADING_STATE
pub const FMOD_STUDIO_LOADING_STATE_UNLOADING: u32 = 0;
pub const FMOD_STUDIO_LOADING_STATE_UNLOADED: u32 = 1;
pub const FMOD_STUDIO_LOADING_STATE_LOADING: u32 = 2;
pub const FMOD_STUDIO_LOADING_STATE_LOADED: u32 = 3;
pub const FMOD_STUDIO_LOADING_STATE_ERROR: u32 = 4;

// FMOD_STUDIO_PLAYBACK_STATE
pub const FMOD_STUDIO_PLAYBACK_PLAYING: u32 = 0;
pub const FMOD_STUDIO_PLAYBACK_SUSTAINING: u32 = 1;
pub const FMOD_STUDIO_PLAYBACK_STOPPED: u32 = 2;
pub const FMOD_STUDIO_PLAYBACK_STARTING: u32 = 3;
pub const FMOD_STUDIO_PLAYBACK_STOPPING: u32 = 4;

// FMOD_STUDIO_STOP_MODE
pub const FMOD_STUDIO_STOP_ALLOWFADEOUT: u32 = 0;
pub const FMOD_STUDIO_STOP_IMMEDIATE: u32 = 1;

// FMOD_STUDIO_LOAD_MEMORY_MODE
pub const FMOD_STUDIO_LOAD_MEMORY: u32 = 0;
pub const FMOD_STUDIO_LOAD_MEMORY_POINT: u32 = 1;

// FMOD_STUDIO_LOAD_BANK_FLAGS
pub const FMOD_STUDIO_LOAD_BANK_NORMAL: u32 = 0x0000_0000;
pub const FMOD_STUDIO_LOAD_BANK_NONBLOCKING: u32 = 0x0000_0001;
pub const FMOD_STUDIO_LOAD_BANK_DECOMPRESS_SAMPLES: u32 = 0x0000_0002;
pub const FMOD_STUDIO_LOAD_BANK_UNENCRYPTED: u32 = 0x0000_0004;

// FMOD_STUDIO_EVENT_PROPERTY
pub const FMOD_STUDIO_EVENT_PROPERTY_CHANNELPRIORITY: u32 = 0;
pub const FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_DELAY: u32 = 1;
pub const FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_LOOKAHEAD: u32 = 2;
pub const FMOD_STUDIO_EVENT_PROPERTY_MINIMUM_DISTANCE: u32 = 3;
pub const FMOD_STUDIO_EVENT_PROPERTY_MAXIMUM_DISTANCE: u32 = 4;
pub const FMOD_STUDIO_EVENT_PROPERTY_COOLDOWN: u32 = 5;
pub const FMOD_STUDIO_EVENT_PROPERTY_MAX: u32 = 6;

// FMOD_STUDIO_PARAMETER_TYPE
pub const FMOD_STUDIO_PARAMETER_GAME_CONTROLLED: u32 = 0;
pub const FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE: u32 = 1;

// FMOD_STUDIO_PARAMETER_FLAGS
pub const FMOD_STUDIO_PARAMETER_READONLY: u32 = 0x0000_0001;
pub const FMOD_STUDIO_PARAMETER_AUTOMATIC: u32 = 0x0000_0002;
pub const FMOD_STUDIO_PARAMETER_GLOBAL: u32 = 0x0000_0004;
pub const FMOD_STUDIO_PARAMETER_DISCRETE: u32 = 0x0000_0008;
pub const FMOD_STUDIO_PARAMETER_LABELED: u32 = 0x0000_0010;

/// Mixer output format (48 kHz stereo interleaved f32).
const MIX_RATE: u32 = 48_000;
const MIX_CHANNELS: u16 = 2;
const MIX_IO_FRAMES: u32 = 512;
const MIX_RING_FRAMES: usize = 4096;
const MIX_BLOCK_FRAMES: usize = 256;
const MIX_TARGET_BUFFER_FRAMES: usize = 1536;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FmodVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct FmodGuid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl FmodGuid {
    pub const ZERO: Self = Self {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };

    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }

    pub fn from_le_bytes(b: [u8; 16]) -> Self {
        Self {
            data1: u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            data2: u16::from_le_bytes([b[4], b[5]]),
            data3: u16::from_le_bytes([b[6], b[7]]),
            data4: [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]],
        }
    }

    pub fn to_le_bytes(self) -> [u8; 16] {
        let mut out = [0u8; 16];
        out[0..4].copy_from_slice(&self.data1.to_le_bytes());
        out[4..6].copy_from_slice(&self.data2.to_le_bytes());
        out[6..8].copy_from_slice(&self.data3.to_le_bytes());
        out[8..16].copy_from_slice(&self.data4);
        out
    }

    pub fn parse_str(s: &str) -> Option<Self> {
        let trimmed = s.trim();
        let inner = trimmed
            .strip_prefix('{')
            .and_then(|v| v.strip_suffix('}'))?;
        let parts: Vec<&str> = inner.split('-').collect();
        if parts.len() != 5
            || parts[0].len() != 8
            || parts[1].len() != 4
            || parts[2].len() != 4
            || parts[3].len() != 4
            || parts[4].len() != 12
        {
            return None;
        }
        let data1 = u32::from_str_radix(parts[0], 16).ok()?;
        let data2 = u16::from_str_radix(parts[1], 16).ok()?;
        let data3 = u16::from_str_radix(parts[2], 16).ok()?;
        let mut data4 = [0u8; 8];
        data4[0] = u8::from_str_radix(&parts[3][0..2], 16).ok()?;
        data4[1] = u8::from_str_radix(&parts[3][2..4], 16).ok()?;
        for i in 0..6 {
            data4[2 + i] = u8::from_str_radix(&parts[4][i * 2..i * 2 + 2], 16).ok()?;
        }
        Some(Self {
            data1,
            data2,
            data3,
            data4,
        })
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fmod3DAttributes {
    pub position: FmodVector,
    pub velocity: FmodVector,
    pub forward: FmodVector,
    pub up: FmodVector,
}

impl Default for Fmod3DAttributes {
    fn default() -> Self {
        Self {
            position: FmodVector { x: 0.0, y: 0.0, z: 0.0 },
            velocity: FmodVector { x: 0.0, y: 0.0, z: 0.0 },
            forward: FmodVector { x: 0.0, y: 0.0, z: 1.0 },
            up: FmodVector { x: 0.0, y: 1.0, z: 0.0 },
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct FmodStudioParameterId {
    pub data1: u32,
    pub data2: u32,
}

impl FmodStudioParameterId {
    #[inline]
    pub fn from_u64(raw: u64) -> Self {
        Self {
            data1: raw as u32,
            data2: (raw >> 32) as u32,
        }
    }

    #[inline]
    pub fn to_u64(self) -> u64 {
        (self.data1 as u64) | ((self.data2 as u64) << 32)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FmodStudioParameterDescription {
    pub name: *const u8,
    pub id: FmodStudioParameterId,
    pub minimum: f32,
    pub maximum: f32,
    pub defaultvalue: f32,
    pub param_type: u32,
    pub flags: u32,
    pub guid: FmodGuid,
}

/// First 24 bytes of `FMOD_CREATESOUNDEXINFO` (identical layout across FMOD 1.x / 2.x Win64).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct FmodCreateSoundExInfoHeader {
    pub cbsize: i32,
    pub length: u32,
    pub fileoffset: u32,
    pub numchannels: i32,
    pub defaultfrequency: i32,
    pub format: u32,
}

// ============================================================================
// Guest-memory unaligned pointer helpers
// ============================================================================

#[inline]
unsafe fn write_out<T>(ptr: *mut T, val: T) {
    if !ptr.is_null() {
        std::ptr::write_unaligned(ptr, val);
    }
}

#[inline]
unsafe fn read_in<T>(ptr: *const T) -> Option<T> {
    if ptr.is_null() {
        None
    } else {
        Some(std::ptr::read_unaligned(ptr))
    }
}

unsafe fn read_exinfo(exinfo: usize) -> Result<Option<FmodCreateSoundExInfoHeader>, i32> {
    if exinfo == 0 {
        return Ok(None);
    }
    let p = exinfo as *const u8;
    let cbsize = std::ptr::read_unaligned(p as *const i32);
    if cbsize < 4 {
        return Err(FMOD_ERR_INVALID_PARAM);
    }
    let mut info = FmodCreateSoundExInfoHeader {
        cbsize,
        ..Default::default()
    };
    if cbsize >= 8 {
        info.length = std::ptr::read_unaligned(p.add(4) as *const u32);
    }
    if cbsize >= 12 {
        info.fileoffset = std::ptr::read_unaligned(p.add(8) as *const u32);
    }
    if cbsize >= 16 {
        info.numchannels = std::ptr::read_unaligned(p.add(12) as *const i32);
    }
    if cbsize >= 20 {
        info.defaultfrequency = std::ptr::read_unaligned(p.add(16) as *const i32);
    }
    if cbsize >= 24 {
        info.format = std::ptr::read_unaligned(p.add(20) as *const u32);
    }
    Ok(Some(info))
}

// ============================================================================
// PCM & RIFF/WAV decoding
// ============================================================================

fn format_bits_and_bytes(format: u32) -> Option<(u32, usize)> {
    match format {
        FMOD_SOUND_FORMAT_PCM8 => Some((8, 1)),
        FMOD_SOUND_FORMAT_PCM16 => Some((16, 2)),
        FMOD_SOUND_FORMAT_PCM24 => Some((24, 3)),
        FMOD_SOUND_FORMAT_PCM32 => Some((32, 4)),
        FMOD_SOUND_FORMAT_PCMFLOAT => Some((32, 4)),
        _ => None,
    }
}

fn decode_pcm_to_f32(raw: &[u8], format: u32) -> Option<Vec<f32>> {
    match format {
        FMOD_SOUND_FORMAT_PCM8 => Some(
            raw.iter()
                .map(|&b| ((b as f32) - 128.0) / 128.0)
                .collect(),
        ),
        FMOD_SOUND_FORMAT_PCM16 => Some(
            raw.chunks_exact(2)
                .map(|c| (i16::from_le_bytes([c[0], c[1]]) as f32) / 32768.0)
                .collect(),
        ),
        FMOD_SOUND_FORMAT_PCM24 => Some(
            raw.chunks_exact(3)
                .map(|c| {
                    let raw24 = (c[0] as i32) | ((c[1] as i32) << 8) | ((c[2] as i32) << 16);
                    let signed = (raw24 << 8) >> 8;
                    (signed as f32) / 8_388_608.0
                })
                .collect(),
        ),
        FMOD_SOUND_FORMAT_PCM32 => Some(
            raw.chunks_exact(4)
                .map(|c| {
                    let s = i32::from_le_bytes([c[0], c[1], c[2], c[3]]);
                    ((s as f64) / 2_147_483_648.0) as f32
                })
                .collect(),
        ),
        FMOD_SOUND_FORMAT_PCMFLOAT => Some(
            raw.chunks_exact(4)
                .map(|c| {
                    let v = f32::from_le_bytes([c[0], c[1], c[2], c[3]]);
                    if v.is_finite() { v } else { 0.0 }
                })
                .collect(),
        ),
        _ => None,
    }
}

struct DecodedWav {
    format: u32,
    channels: u32,
    sample_rate: u32,
    bits_per_sample: u32,
    raw_pcm: Vec<u8>,
}

fn identify_container(bytes: &[u8]) -> &'static str {
    if bytes.len() >= 4 {
        match &bytes[0..4] {
            b"FSB5" | b"FSB4" | b"FSB3" => return "FSB bank",
            b"OggS" => return "Ogg/Vorbis/Opus",
            b"fLaC" => return "FLAC",
            b"FORM" => return "AIFF",
            b"RIFF" => return "RIFF (non-WAVE)",
            _ => {}
        }
    }
    if bytes.len() >= 3 && &bytes[0..3] == b"ID3" {
        return "MP3/ID3";
    }
    if bytes.len() >= 2 && bytes[0] == 0xFF && (bytes[1] & 0xE0) == 0xE0 {
        return "MPEG/MP3";
    }
    "unknown/proprietary container"
}

fn parse_wav(bytes: &[u8]) -> Result<DecodedWav, i32> {
    if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        let kind = identify_container(bytes);
        log::warn!(
            target: "audio",
            "FMOD_System_CreateSound: unsupported audio container ({kind}, {} bytes) -> FMOD_ERR_FORMAT",
            bytes.len()
        );
        return Err(FMOD_ERR_FORMAT);
    }

    const SUBTYPE_PCM: [u8; 16] = [
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B,
        0x71,
    ];
    const SUBTYPE_IEEE_FLOAT: [u8; 16] = [
        0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B,
        0x71,
    ];

    let mut fmt_info: Option<(u32, u32, u32, u32)> = None;
    let mut data_slice: Option<&[u8]> = None;

    let mut pos = 12usize;
    while pos + 8 <= bytes.len() {
        let chunk_id = &bytes[pos..pos + 4];
        let chunk_size = u32::from_le_bytes([
            bytes[pos + 4],
            bytes[pos + 5],
            bytes[pos + 6],
            bytes[pos + 7],
        ]) as usize;
        pos += 8;
        let payload_end = pos.saturating_add(chunk_size).min(bytes.len());
        let payload = &bytes[pos..payload_end];

        if chunk_id == b"fmt " {
            if payload.len() < 16 {
                return Err(FMOD_ERR_FORMAT);
            }
            let mut audio_format = u16::from_le_bytes([payload[0], payload[1]]);
            let channels = u16::from_le_bytes([payload[2], payload[3]]) as u32;
            let sample_rate = u32::from_le_bytes([payload[4], payload[5], payload[6], payload[7]]);
            let bits = u16::from_le_bytes([payload[14], payload[15]]) as u32;

            if audio_format == 0xFFFE {
                if payload.len() < 40 {
                    return Err(FMOD_ERR_FORMAT);
                }
                let subformat = &payload[24..40];
                if subformat == SUBTYPE_PCM {
                    audio_format = 1;
                } else if subformat == SUBTYPE_IEEE_FLOAT {
                    audio_format = 3;
                } else {
                    log::warn!(
                        target: "audio",
                        "FMOD_System_CreateSound: unsupported WAV WAVE_FORMAT_EXTENSIBLE subformat ({bits}-bit) -> FMOD_ERR_FORMAT"
                    );
                    return Err(FMOD_ERR_FORMAT);
                }
            }

            let fmod_fmt = match (audio_format, bits) {
                (1, 8) => FMOD_SOUND_FORMAT_PCM8,
                (1, 16) => FMOD_SOUND_FORMAT_PCM16,
                (1, 24) => FMOD_SOUND_FORMAT_PCM24,
                (1, 32) => FMOD_SOUND_FORMAT_PCM32,
                (3, 32) => FMOD_SOUND_FORMAT_PCMFLOAT,
                _ => {
                    log::warn!(
                        target: "audio",
                        "FMOD_System_CreateSound: unsupported WAV format tag 0x{audio_format:04x} ({bits}-bit) -> FMOD_ERR_FORMAT"
                    );
                    return Err(FMOD_ERR_FORMAT);
                }
            };

            if channels == 0 || channels > 32 || sample_rate == 0 {
                return Err(FMOD_ERR_FORMAT);
            }
            fmt_info = Some((fmod_fmt, channels, sample_rate, bits));
        } else if chunk_id == b"data" {
            data_slice = Some(payload);
        }

        let padded = chunk_size.saturating_add(chunk_size & 1);
        match pos.checked_add(padded) {
            Some(next) if next <= bytes.len() => pos = next,
            _ => break,
        }
    }

    let (format, channels, sample_rate, bits_per_sample) = fmt_info.ok_or(FMOD_ERR_FORMAT)?;
    let data = data_slice.ok_or(FMOD_ERR_FORMAT)?;
    let (_, bps) = format_bits_and_bytes(format).ok_or(FMOD_ERR_FORMAT)?;
    let frame_bytes = (channels as usize) * bps;
    let valid_len = (data.len() / frame_bytes) * frame_bytes;

    Ok(DecodedWav {
        format,
        channels,
        sample_rate,
        bits_per_sample,
        raw_pcm: data[..valid_len].to_vec(),
    })
}

// ============================================================================
// Pure-Rust FSB5 parser & audio decoders (PCM8/16/24/32, Float, IMA-ADPCM, FADPCM)
// ============================================================================

const FSB5_MODE_PCM8: u32 = 1;
const FSB5_MODE_PCM16: u32 = 2;
const FSB5_MODE_PCM24: u32 = 3;
const FSB5_MODE_PCM32: u32 = 4;
const FSB5_MODE_PCMFLOAT: u32 = 5;
const FSB5_MODE_IMAADPCM: u32 = 7;
const FSB5_MODE_FADPCM: u32 = 16;

const FSB5_CHUNK_CHANNELS: u32 = 1;
const FSB5_CHUNK_FREQUENCY: u32 = 2;
const FSB5_CHUNK_LOOP: u32 = 3;

const FSB5_SAMPLE_RATES: [u32; 16] = [
    4_000, 8_000, 11_000, 11_025, 16_000, 22_050, 24_000, 32_000, 44_100, 48_000, 96_000, 0, 0, 0,
    0, 0,
];

const IMA_STEP_TABLE: [i32; 89] = [
    7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 19, 21, 23, 25, 28, 31, 34, 37, 41, 45, 50, 55, 60, 66,
    73, 80, 88, 97, 107, 118, 130, 143, 157, 173, 190, 209, 230, 253, 279, 307, 337, 371, 408, 449,
    494, 544, 598, 658, 724, 796, 876, 963, 1060, 1166, 1282, 1411, 1552, 1707, 1878, 2066, 2272,
    2499, 2749, 3024, 3327, 3660, 4026, 4428, 4871, 5358, 5894, 6484, 7132, 7845, 8630, 9493,
    10442, 11487, 12635, 13899, 15289, 16818, 18500, 20350, 22385, 24623, 27086, 29794, 32767,
];

const IMA_INDEX_TABLE: [i32; 16] = [
    -1, -1, -1, -1, 2, 4, 6, 8, -1, -1, -1, -1, 2, 4, 6, 8,
];

const FADPCM_COEFS: [[i32; 2]; 8] = [
    [0, 0],
    [60, 0],
    [122, 60],
    [115, 52],
    [98, 55],
    [0, 0],
    [0, 0],
    [0, 0],
];

#[derive(Clone)]
struct DecodedSample {
    name: String,
    channels: u32,
    sample_rate: u32,
    num_frames: usize,
    loop_start: usize,
    loop_end: usize,
    has_loop: bool,
    samples: Arc<Vec<f32>>,
    fmod_format: u32,
    bits_per_sample: u32,
    raw_pcm: Vec<u8>,
}

struct ParsedFsb5 {
    total_bytes: usize,
    samples: Vec<DecodedSample>,
}

#[inline]
fn decode_ima_nibble(nibble: u8, predictor: &mut i32, step_index: &mut i32) -> f32 {
    let step = IMA_STEP_TABLE[(*step_index).clamp(0, 88) as usize];
    let mut diff = step >> 3;
    if (nibble & 1) != 0 {
        diff += step >> 2;
    }
    if (nibble & 2) != 0 {
        diff += step >> 1;
    }
    if (nibble & 4) != 0 {
        diff += step;
    }
    if (nibble & 8) != 0 {
        *predictor -= diff;
    } else {
        *predictor += diff;
    }
    *predictor = (*predictor).clamp(-32_768, 32_767);
    *step_index = (*step_index + IMA_INDEX_TABLE[(nibble & 0x0F) as usize]).clamp(0, 88);
    (*predictor as f32) / 32_768.0
}

/// Decode FMOD FSB5 IMA-ADPCM (`36 * channels` bytes per 64-frame block).
/// Each block contains `4 * channels` bytes of per-channel headers (`i16` predictor,
/// `u8` step_index, `u8` pad) followed by 8 sub-blocks of `4 * channels` bytes
/// (4 bytes = 8 nibbles = 8 samples per channel per sub-block).
fn decode_fsb5_ima_adpcm(data: &[u8], channels: usize, num_frames: usize) -> Vec<f32> {
    if channels == 0 || num_frames == 0 {
        return Vec::new();
    }
    let block_bytes = 36 * channels;
    let total_blocks = data.len() / block_bytes;
    let mut out = Vec::with_capacity(num_frames.saturating_mul(channels));
    let mut predictors = vec![0i32; channels];
    let mut step_indices = vec![0i32; channels];
    let mut sub_buf = vec![0.0f32; 8 * channels];

    for b in 0..total_blocks {
        let blk = &data[b * block_bytes..(b + 1) * block_bytes];
        for c in 0..channels {
            let hdr = &blk[c * 4..c * 4 + 4];
            predictors[c] = i16::from_le_bytes([hdr[0], hdr[1]]) as i32;
            step_indices[c] = (hdr[2] as i32).clamp(0, 88);
        }

        let mut data_off = 4 * channels;
        for _sub in 0..8 {
            for c in 0..channels {
                let word = &blk[data_off..data_off + 4];
                data_off += 4;
                for byte_idx in 0..4 {
                    let byte = word[byte_idx];
                    let s0 = decode_ima_nibble(byte & 0x0F, &mut predictors[c], &mut step_indices[c]);
                    let s1 = decode_ima_nibble((byte >> 4) & 0x0F, &mut predictors[c], &mut step_indices[c]);
                    sub_buf[(byte_idx * 2) * channels + c] = s0;
                    sub_buf[(byte_idx * 2 + 1) * channels + c] = s1;
                }
            }
            for f in 0..8 {
                if out.len() >= num_frames * channels {
                    return out;
                }
                out.extend_from_slice(&sub_buf[f * channels..(f + 1) * channels]);
            }
        }
    }
    out.truncate(num_frames * channels);
    out
}

/// Decode FMOD FADPCM (4-bit ADPCM, 140 bytes per 256 samples per channel,
/// frame-interleaved across channels: `channels * 140` bytes per 256-frame block).
fn decode_fsb5_fadpcm(data: &[u8], channels: usize, num_frames: usize) -> Vec<f32> {
    if channels == 0 || num_frames == 0 {
        return Vec::new();
    }
    const FRAME_BYTES: usize = 140;
    const FRAME_SAMPLES: usize = 256;
    let block_bytes = FRAME_BYTES * channels;
    let total_blocks = data.len() / block_bytes;
    let mut out = Vec::with_capacity(num_frames.saturating_mul(channels));
    let mut ch_decoded = vec![0.0f32; FRAME_SAMPLES * channels];

    for b in 0..total_blocks {
        let blk = &data[b * block_bytes..(b + 1) * block_bytes];
        for c in 0..channels {
            let fbytes = &blk[c * FRAME_BYTES..(c + 1) * FRAME_BYTES];
            let coefs = u32::from_le_bytes([fbytes[0], fbytes[1], fbytes[2], fbytes[3]]);
            let shifts = u32::from_le_bytes([fbytes[4], fbytes[5], fbytes[6], fbytes[7]]);
            let mut hist1 = i16::from_le_bytes([fbytes[8], fbytes[9]]) as i32;
            let mut hist2 = i16::from_le_bytes([fbytes[10], fbytes[11]]) as i32;

            let mut sample_idx = 0usize;
            for sec in 0..8 {
                let coef_idx = ((coefs >> (sec * 4)) & 0x07) as usize;
                let raw_shift = (shifts >> (sec * 4)) & 0x0F;
                let shift = 22u32.saturating_sub(raw_shift);
                let coef1 = FADPCM_COEFS[coef_idx][0];
                let coef2 = FADPCM_COEFS[coef_idx][1];

                for w in 0..4 {
                    let off = 12 + sec * 16 + w * 4;
                    let nibbles = u32::from_le_bytes([
                        fbytes[off],
                        fbytes[off + 1],
                        fbytes[off + 2],
                        fbytes[off + 3],
                    ]);
                    for k in 0..8 {
                        let nib = (nibbles >> (k * 4)) & 0x0F;
                        let residual = ((nib << 28) as i32) >> shift;
                        let s = ((residual - hist2 * coef2 + hist1 * coef1 + 32) >> 6)
                            .clamp(-32_768, 32_767);
                        hist2 = hist1;
                        hist1 = s;
                        ch_decoded[sample_idx * channels + c] = (s as f32) / 32_768.0;
                        sample_idx += 1;
                    }
                }
            }
        }

        let remaining_frames = num_frames.saturating_sub(out.len() / channels);
        let take_frames = remaining_frames.min(FRAME_SAMPLES);
        out.extend_from_slice(&ch_decoded[..take_frames * channels]);
        if out.len() >= num_frames * channels {
            break;
        }
    }
    out.truncate(num_frames * channels);
    out
}

fn f32_samples_to_le_bytes(samples: &[f32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(samples.len() * 4);
    for &s in samples {
        out.extend_from_slice(&s.to_le_bytes());
    }
    out
}

fn parse_fsb5(bytes: &[u8]) -> Result<ParsedFsb5, i32> {
    if bytes.len() < 28 || &bytes[0..4] != b"FSB5" {
        return Err(FMOD_ERR_FORMAT);
    }
    let version = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let header_size: usize = match version {
        0 => 64,
        1 => 60,
        _ => {
            log::warn!(target: "audio", "FSB5: unsupported header version {version}");
            return Err(FMOD_ERR_FORMAT);
        }
    };
    if bytes.len() < header_size {
        return Err(FMOD_ERR_FORMAT);
    }

    let num_samples = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]) as usize;
    let sample_headers_size =
        u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]) as usize;
    let name_table_size =
        u32::from_le_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]) as usize;
    let data_size = u32::from_le_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]) as usize;
    let mode = u32::from_le_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]);

    let sh_start = header_size;
    let sh_end = sh_start.checked_add(sample_headers_size).ok_or(FMOD_ERR_FORMAT)?;
    let nt_start = sh_end;
    let nt_end = nt_start.checked_add(name_table_size).ok_or(FMOD_ERR_FORMAT)?;
    let data_start = nt_end;
    let data_end = data_start.checked_add(data_size).ok_or(FMOD_ERR_FORMAT)?;
    if data_end > bytes.len() {
        return Err(FMOD_ERR_FORMAT);
    }

    struct RawFsbSampleMeta {
        channels: u32,
        sample_rate: u32,
        data_offset: usize,
        num_frames: usize,
        loop_start: usize,
        loop_end: usize,
        has_loop: bool,
    }

    let sh_slice = &bytes[sh_start..sh_end];
    let mut sh_pos = 0usize;
    let mut metas = Vec::with_capacity(num_samples.min(4096));

    for _ in 0..num_samples {
        if sh_pos + 8 > sh_slice.len() {
            return Err(FMOD_ERR_FORMAT);
        }
        let raw = u64::from_le_bytes([
            sh_slice[sh_pos],
            sh_slice[sh_pos + 1],
            sh_slice[sh_pos + 2],
            sh_slice[sh_pos + 3],
            sh_slice[sh_pos + 4],
            sh_slice[sh_pos + 5],
            sh_slice[sh_pos + 6],
            sh_slice[sh_pos + 7],
        ]);
        sh_pos += 8;

        let mut more_chunks = (raw & 1) != 0;
        let freq_idx = ((raw >> 1) & 0x0F) as usize;
        let ch_bits = ((raw >> 5) & 0x03) as usize;
        let data_offset = (((raw >> 7) & 0x07FF_FFFF) as usize)
            .checked_mul(16)
            .ok_or(FMOD_ERR_FORMAT)?;
        let num_frames = ((raw >> 34) & 0x3FFF_FFFF) as usize;

        let mut sample_rate = FSB5_SAMPLE_RATES[freq_idx];
        let mut channels: u32 = match ch_bits {
            0 => 1,
            1 => 2,
            2 => 6,
            _ => 8,
        };
        let mut loop_start = 0usize;
        let mut loop_end = num_frames;
        let mut has_loop = false;

        while more_chunks {
            if sh_pos + 4 > sh_slice.len() {
                return Err(FMOD_ERR_FORMAT);
            }
            let chdr = u32::from_le_bytes([
                sh_slice[sh_pos],
                sh_slice[sh_pos + 1],
                sh_slice[sh_pos + 2],
                sh_slice[sh_pos + 3],
            ]);
            sh_pos += 4;
            more_chunks = (chdr & 1) != 0;
            let csize = ((chdr >> 1) & 0x00FF_FFFF) as usize;
            let ctype = (chdr >> 25) & 0x7F;
            let cend = sh_pos.checked_add(csize).ok_or(FMOD_ERR_FORMAT)?;
            if cend > sh_slice.len() {
                return Err(FMOD_ERR_FORMAT);
            }
            let cpayload = &sh_slice[sh_pos..cend];
            sh_pos = cend;

            match ctype {
                FSB5_CHUNK_CHANNELS if !cpayload.is_empty() => {
                    channels = cpayload[0] as u32;
                }
                FSB5_CHUNK_FREQUENCY if cpayload.len() >= 4 => {
                    sample_rate =
                        u32::from_le_bytes([cpayload[0], cpayload[1], cpayload[2], cpayload[3]]);
                }
                FSB5_CHUNK_LOOP if cpayload.len() >= 8 => {
                    loop_start =
                        u32::from_le_bytes([cpayload[0], cpayload[1], cpayload[2], cpayload[3]])
                            as usize;
                    loop_end =
                        u32::from_le_bytes([cpayload[4], cpayload[5], cpayload[6], cpayload[7]])
                            as usize;
                    has_loop = true;
                }
                _ => {}
            }
        }

        if channels == 0 || channels > 32 || sample_rate == 0 || data_offset > data_size {
            return Err(FMOD_ERR_FORMAT);
        }

        metas.push(RawFsbSampleMeta {
            channels,
            sample_rate,
            data_offset,
            num_frames,
            loop_start,
            loop_end,
            has_loop,
        });
    }

    // Parse sample name table if present.
    let mut names = vec![String::new(); num_samples];
    if name_table_size > 0 {
        let nt_slice = &bytes[nt_start..nt_end];
        let table_bytes = num_samples.checked_mul(4).ok_or(FMOD_ERR_FORMAT)?;
        if table_bytes > nt_slice.len() {
            return Err(FMOD_ERR_FORMAT);
        }
        for (i, slot) in names.iter_mut().enumerate() {
            let off = u32::from_le_bytes([
                nt_slice[i * 4],
                nt_slice[i * 4 + 1],
                nt_slice[i * 4 + 2],
                nt_slice[i * 4 + 3],
            ]) as usize;
            if off < nt_slice.len() {
                let tail = &nt_slice[off..];
                let end = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
                *slot = String::from_utf8_lossy(&tail[..end]).into_owned();
            }
        }
    }

    let data_slice = &bytes[data_start..data_end];
    let mut decoded_samples = Vec::with_capacity(num_samples);

    for (i, meta) in metas.iter().enumerate() {
        let start = meta.data_offset;
        let end = if i + 1 < num_samples && metas[i + 1].data_offset >= start {
            metas[i + 1].data_offset
        } else {
            data_size
        };
        if end > data_slice.len() || end < start {
            return Err(FMOD_ERR_FORMAT);
        }
        let sample_raw = &data_slice[start..end];
        let ch = meta.channels as usize;

        let (fmod_format, bits_per_sample, decoded_f32, raw_pcm) = match mode {
            FSB5_MODE_PCM8
            | FSB5_MODE_PCM16
            | FSB5_MODE_PCM24
            | FSB5_MODE_PCM32
            | FSB5_MODE_PCMFLOAT => {
                let (bits, bps) = format_bits_and_bytes(mode).ok_or(FMOD_ERR_FORMAT)?;
                let expected_bytes = meta.num_frames.saturating_mul(ch).saturating_mul(bps);
                let frame_bytes = ch * bps;
                let usable = (sample_raw.len().min(expected_bytes) / frame_bytes) * frame_bytes;
                let pcm_slice = &sample_raw[..usable];
                let dec = decode_pcm_to_f32(pcm_slice, mode).ok_or(FMOD_ERR_FORMAT)?;
                (mode, bits, dec, pcm_slice.to_vec())
            }
            FSB5_MODE_IMAADPCM => {
                let dec = decode_fsb5_ima_adpcm(sample_raw, ch, meta.num_frames);
                let raw = f32_samples_to_le_bytes(&dec);
                (FMOD_SOUND_FORMAT_PCMFLOAT, 32, dec, raw)
            }
            FSB5_MODE_FADPCM => {
                let dec = decode_fsb5_fadpcm(sample_raw, ch, meta.num_frames);
                let raw = f32_samples_to_le_bytes(&dec);
                (FMOD_SOUND_FORMAT_PCMFLOAT, 32, dec, raw)
            }
            other => {
                log::warn!(target: "audio", "FSB5: unsupported audio codec mode {other} -> FMOD_ERR_FORMAT");
                return Err(FMOD_ERR_FORMAT);
            }
        };

        let actual_frames = decoded_f32.len() / ch;
        let (has_loop, loop_start, loop_end) = if meta.has_loop && actual_frames > 0 {
            let ls = meta.loop_start.min(actual_frames);
            // FSB5 loop_end is an inclusive sample index.
            let le = meta.loop_end.saturating_add(1).min(actual_frames);
            if le > ls {
                (true, ls, le)
            } else {
                (false, 0, actual_frames)
            }
        } else {
            (false, 0, actual_frames)
        };

        decoded_samples.push(DecodedSample {
            name: names[i].clone(),
            channels: meta.channels,
            sample_rate: meta.sample_rate,
            num_frames: actual_frames,
            loop_start,
            loop_end,
            has_loop,
            samples: Arc::new(decoded_f32),
            fmod_format,
            bits_per_sample,
            raw_pcm,
        });
    }

    Ok(ParsedFsb5 {
        total_bytes: data_end,
        samples: decoded_samples,
    })
}

// ============================================================================
// Pure-Rust FMOD Studio `.bank` (RIFF `FEV `) parser
// ============================================================================

#[derive(Clone, Debug)]
struct StudioParameterDef {
    name: String,
    id: FmodStudioParameterId,
    guid: FmodGuid,
    minimum: f32,
    maximum: f32,
    default_value: f32,
    param_type: u32,
    flags: u32,
}

struct ParsedBus {
    guid: FmodGuid,
    parent_guid: Option<FmodGuid>,
    path: String,
    default_volume: f32,
    mute: bool,
    paused: bool,
}

struct ParsedVca {
    guid: FmodGuid,
    path: String,
    default_volume: f32,
    target_bus_guids: Vec<FmodGuid>,
    target_event_guids: Vec<FmodGuid>,
}

struct ParsedEvent {
    guid: FmodGuid,
    bus_guid: Option<FmodGuid>,
    path: String,
    sample_index: Option<usize>,
    is_3d: bool,
    is_oneshot: bool,
    is_stream: bool,
    is_snapshot: bool,
    default_volume: f32,
    default_pitch: f32,
    min_distance: f32,
    max_distance: f32,
    length_ms: i32,
    parameters: Vec<StudioParameterDef>,
}

struct ParsedBank {
    guid: FmodGuid,
    has_explicit_guid: bool,
    path: String,
    strings: Vec<(FmodGuid, String)>,
    samples: Vec<DecodedSample>,
    buses: Vec<ParsedBus>,
    vcas: Vec<ParsedVca>,
    events: Vec<ParsedEvent>,
    global_params: Vec<StudioParameterDef>,
}

#[inline]
fn rd_u32_le(b: &[u8], off: usize) -> Option<u32> {
    let s = b.get(off..off + 4)?;
    Some(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}

#[inline]
fn rd_i32_le(b: &[u8], off: usize) -> Option<i32> {
    rd_u32_le(b, off).map(|v| v as i32)
}

#[inline]
fn rd_u64_le(b: &[u8], off: usize) -> Option<u64> {
    let s = b.get(off..off + 8)?;
    Some(u64::from_le_bytes([
        s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7],
    ]))
}

#[inline]
fn rd_f32_le(b: &[u8], off: usize) -> Option<f32> {
    rd_u32_le(b, off).map(f32::from_bits)
}

#[inline]
fn rd_guid(b: &[u8], off: usize) -> Option<FmodGuid> {
    let s = b.get(off..off + 16)?;
    let mut arr = [0u8; 16];
    arr.copy_from_slice(s);
    Some(FmodGuid::from_le_bytes(arr))
}

fn rd_len_string(b: &[u8], off: &mut usize) -> Option<String> {
    let len = rd_u32_le(b, *off)? as usize;
    *off += 4;
    let end = off.checked_add(len)?;
    let slice = b.get(*off..end)?;
    *off = end;
    let trimmed = slice.strip_suffix(&[0]).unwrap_or(slice);
    Some(String::from_utf8_lossy(trimmed).into_owned())
}

/// Parse an FMOD Studio radix trie (`STRR` or `STRT` type 1).
/// Layout:
/// - `guid_count: u32`, `node_count: u32`, `pool_size: u32`
/// - `guid_count` x 16-byte `FmodGuid`
/// - `node_count` x 16-byte nodes `(parent_idx: i32, guid_idx: i32, str_off: u32, str_len: u32)`
/// - `pool_size` bytes of UTF-8 edge string slices
fn parse_radix_trie_strings(payload: &[u8], out: &mut Vec<(FmodGuid, String)>) -> bool {
    let Some(guid_count) = rd_u32_le(payload, 0).map(|v| v as usize) else {
        return false;
    };
    let Some(node_count) = rd_u32_le(payload, 4).map(|v| v as usize) else {
        return false;
    };
    let Some(pool_size) = rd_u32_le(payload, 8).map(|v| v as usize) else {
        return false;
    };

    let guids_off = 12usize;
    let Some(guids_bytes) = guid_count.checked_mul(16) else {
        return false;
    };
    let Some(nodes_off) = guids_off.checked_add(guids_bytes) else {
        return false;
    };
    let Some(nodes_bytes) = node_count.checked_mul(16) else {
        return false;
    };
    let Some(pool_off) = nodes_off.checked_add(nodes_bytes) else {
        return false;
    };
    let Some(pool_end) = pool_off.checked_add(pool_size) else {
        return false;
    };
    if pool_end > payload.len() {
        return false;
    }

    let mut guids = Vec::with_capacity(guid_count);
    for i in 0..guid_count {
        let Some(g) = rd_guid(payload, guids_off + i * 16) else {
            return false;
        };
        guids.push(g);
    }

    struct TrieNode {
        parent: i32,
        guid_idx: i32,
        str_off: usize,
        str_len: usize,
    }
    let mut nodes = Vec::with_capacity(node_count);
    for i in 0..node_count {
        let base = nodes_off + i * 16;
        let (Some(parent), Some(guid_idx), Some(so), Some(sl)) = (
            rd_i32_le(payload, base),
            rd_i32_le(payload, base + 4),
            rd_u32_le(payload, base + 8),
            rd_u32_le(payload, base + 12),
        ) else {
            return false;
        };
        nodes.push(TrieNode {
            parent,
            guid_idx,
            str_off: so as usize,
            str_len: sl as usize,
        });
    }

    let pool = &payload[pool_off..pool_end];
    for (idx, node) in nodes.iter().enumerate() {
        if node.guid_idx < 0 || (node.guid_idx as usize) >= guids.len() {
            continue;
        }
        let mut parts: Vec<&[u8]> = Vec::new();
        let mut cur = idx as i32;
        let mut hops = 0usize;
        while cur >= 0 && (cur as usize) < nodes.len() && hops < 128 {
            hops += 1;
            let n = &nodes[cur as usize];
            if n.str_len > 0 {
                let Some(end) = n.str_off.checked_add(n.str_len) else {
                    break;
                };
                if end <= pool.len() {
                    parts.push(&pool[n.str_off..end]);
                }
            }
            cur = n.parent;
        }
        parts.reverse();
        let mut full = Vec::new();
        for p in parts {
            full.extend_from_slice(p);
        }
        let path = String::from_utf8_lossy(&full).into_owned();
        if !path.is_empty() {
            out.push((guids[node.guid_idx as usize], path));
        }
    }
    true
}

/// Parse an offset-pool string table (`STRI` or `STRT` type 2):
/// `count: u32`, `count` x `(guid: [u8; 16], str_offset: u32)`, then NUL-terminated string pool.
fn parse_offset_pool_strings(payload: &[u8], out: &mut Vec<(FmodGuid, String)>) -> bool {
    let Some(count) = rd_u32_le(payload, 0).map(|v| v as usize) else {
        return false;
    };
    let Some(table_bytes) = count.checked_mul(20) else {
        return false;
    };
    let Some(pool_off) = 4usize.checked_add(table_bytes) else {
        return false;
    };
    if pool_off > payload.len() {
        return false;
    }
    let pool = &payload[pool_off..];
    for i in 0..count {
        let base = 4 + i * 20;
        let (Some(guid), Some(str_off)) = (rd_guid(payload, base), rd_u32_le(payload, base + 16))
        else {
            return false;
        };
        let off = str_off as usize;
        if off >= pool.len() {
            return false;
        }
        let tail = &pool[off..];
        let end = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
        let path = String::from_utf8_lossy(&tail[..end]).into_owned();
        out.push((guid, path));
    }
    true
}

/// Parse a flat inline string table (`STRT` type 0 or `GUID`):
/// `count: u32`, `count` x `(guid: [u8; 16], path_len: u32, path_bytes)`.
fn parse_inline_strings(payload: &[u8], out: &mut Vec<(FmodGuid, String)>) -> bool {
    let Some(count) = rd_u32_le(payload, 0).map(|v| v as usize) else {
        return false;
    };
    let mut off = 4usize;
    let mut entries = Vec::with_capacity(count.min(4096));
    for _ in 0..count {
        let Some(guid) = rd_guid(payload, off) else {
            return false;
        };
        off += 16;
        let Some(path) = rd_len_string(payload, &mut off) else {
            return false;
        };
        entries.push((guid, path));
    }
    out.extend(entries);
    true
}

fn parse_strt_chunk(payload: &[u8], out: &mut Vec<(FmodGuid, String)>) -> bool {
    if payload.len() < 4 {
        return false;
    }
    let table_type = u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]);
    match table_type {
        0 => parse_inline_strings(&payload[4..], out),
        1 => parse_radix_trie_strings(&payload[4..], out),
        2 => parse_offset_pool_strings(&payload[4..], out),
        _ => {
            // Fallback: try interpreting without a leading type tag.
            parse_inline_strings(payload, out) || parse_offset_pool_strings(payload, out)
        }
    }
}

fn parse_buss_chunk(payload: &[u8], out: &mut Vec<ParsedBus>) -> Result<(), i32> {
    let count = rd_u32_le(payload, 0).ok_or(FMOD_ERR_FORMAT)? as usize;
    let mut off = 4usize;
    for _ in 0..count {
        let guid = rd_guid(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 16;
        let parent_raw = rd_guid(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 16;
        let default_volume = rd_f32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let flags = rd_u32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let path = rd_len_string(payload, &mut off).ok_or(FMOD_ERR_FORMAT)?;
        out.push(ParsedBus {
            guid,
            parent_guid: (!parent_raw.is_zero()).then_some(parent_raw),
            path,
            default_volume: if default_volume.is_finite() {
                default_volume
            } else {
                1.0
            },
            mute: (flags & 1) != 0,
            paused: (flags & 2) != 0,
        });
    }
    Ok(())
}

fn parse_vcas_chunk(payload: &[u8], out: &mut Vec<ParsedVca>) -> Result<(), i32> {
    let count = rd_u32_le(payload, 0).ok_or(FMOD_ERR_FORMAT)? as usize;
    let mut off = 4usize;
    for _ in 0..count {
        let guid = rd_guid(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 16;
        let default_volume = rd_f32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let path = rd_len_string(payload, &mut off).ok_or(FMOD_ERR_FORMAT)?;
        let bus_count = rd_u32_le(payload, off).ok_or(FMOD_ERR_FORMAT)? as usize;
        off += 4;
        let mut target_bus_guids = Vec::with_capacity(bus_count.min(256));
        for _ in 0..bus_count {
            let g = rd_guid(payload, off).ok_or(FMOD_ERR_FORMAT)?;
            off += 16;
            target_bus_guids.push(g);
        }
        let ev_count = rd_u32_le(payload, off).ok_or(FMOD_ERR_FORMAT)? as usize;
        off += 4;
        let mut target_event_guids = Vec::with_capacity(ev_count.min(256));
        for _ in 0..ev_count {
            let g = rd_guid(payload, off).ok_or(FMOD_ERR_FORMAT)?;
            off += 16;
            target_event_guids.push(g);
        }
        out.push(ParsedVca {
            guid,
            path,
            default_volume: if default_volume.is_finite() {
                default_volume
            } else {
                1.0
            },
            target_bus_guids,
            target_event_guids,
        });
    }
    Ok(())
}

fn parse_evnt_chunk(payload: &[u8], out: &mut Vec<ParsedEvent>) -> Result<(), i32> {
    let count = rd_u32_le(payload, 0).ok_or(FMOD_ERR_FORMAT)? as usize;
    let mut off = 4usize;
    for _ in 0..count {
        let guid = rd_guid(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 16;
        let bus_raw = rd_guid(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 16;
        let sample_idx = rd_u32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let flags = rd_u32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let default_volume = rd_f32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let default_pitch = rd_f32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let min_distance = rd_f32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let max_distance = rd_f32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let length_ms = rd_u32_le(payload, off).ok_or(FMOD_ERR_FORMAT)? as i32;
        off += 4;
        let path = rd_len_string(payload, &mut off).ok_or(FMOD_ERR_FORMAT)?;

        out.push(ParsedEvent {
            guid,
            bus_guid: (!bus_raw.is_zero()).then_some(bus_raw),
            path,
            sample_index: (sample_idx != u32::MAX).then_some(sample_idx as usize),
            is_3d: (flags & 0x01) != 0,
            is_oneshot: (flags & 0x02) != 0,
            is_stream: (flags & 0x04) != 0,
            is_snapshot: (flags & 0x08) != 0,
            default_volume: if default_volume.is_finite() {
                default_volume
            } else {
                1.0
            },
            default_pitch: if default_pitch.is_finite() && default_pitch > 0.0 {
                default_pitch
            } else {
                1.0
            },
            min_distance: if min_distance.is_finite() && min_distance >= 0.0 {
                min_distance
            } else {
                1.0
            },
            max_distance: if max_distance.is_finite() && max_distance >= min_distance.max(0.0) {
                max_distance
            } else {
                20.0
            },
            length_ms,
            parameters: Vec::new(),
        });
    }
    Ok(())
}

fn parse_parm_chunk(
    payload: &[u8],
    out: &mut Vec<(FmodGuid, StudioParameterDef)>,
) -> Result<(), i32> {
    let count = rd_u32_le(payload, 0).ok_or(FMOD_ERR_FORMAT)? as usize;
    let mut off = 4usize;
    for _ in 0..count {
        let owner_guid = rd_guid(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 16;
        let param_id = FmodStudioParameterId::from_u64(rd_u64_le(payload, off).ok_or(FMOD_ERR_FORMAT)?);
        off += 8;
        let minimum = rd_f32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let maximum = rd_f32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let default_value = rd_f32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let flags = rd_u32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let param_type = rd_u32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let name = rd_len_string(payload, &mut off).ok_or(FMOD_ERR_FORMAT)?;
        let pguid = FmodGuid::from_le_bytes(guid_from_uid(&format!("param:{name}:{:x}:{:x}", param_id.data1, param_id.data2)));
        out.push((
            owner_guid,
            StudioParameterDef {
                name,
                id: param_id,
                guid: pguid,
                minimum: if minimum.is_finite() { minimum } else { 0.0 },
                maximum: if maximum.is_finite() { maximum } else { 1.0 },
                default_value: if default_value.is_finite() {
                    default_value
                } else {
                    0.0
                },
                param_type,
                flags,
            },
        ));
    }
    Ok(())
}

fn parse_embedded_fsb5_chunk(payload: &[u8], samples_out: &mut Vec<DecodedSample>) -> Result<(), i32> {
    let mut off = 0usize;
    let mut found_any = false;
    while off + 4 <= payload.len() {
        if &payload[off..off + 4] == b"FSB5" {
            let fsb = parse_fsb5(&payload[off..])?;
            samples_out.extend(fsb.samples);
            found_any = true;
            let step = fsb.total_bytes.max(4);
            off = off.saturating_add(step);
        } else {
            off += 1;
        }
    }
    if !found_any && payload.len() >= 4 {
        return Err(FMOD_ERR_FORMAT);
    }
    Ok(())
}

struct FevWalkState {
    recognized_chunks: usize,
    bank_guid: FmodGuid,
    has_explicit_guid: bool,
    strings: Vec<(FmodGuid, String)>,
    samples: Vec<DecodedSample>,
    buses: Vec<ParsedBus>,
    vcas: Vec<ParsedVca>,
    events: Vec<ParsedEvent>,
    raw_params: Vec<(FmodGuid, StudioParameterDef)>,
}

fn walk_fev_chunks(slice: &[u8], st: &mut FevWalkState, depth: usize) -> Result<(), i32> {
    if depth > 16 {
        return Err(FMOD_ERR_FORMAT);
    }
    let mut pos = 0usize;
    while pos < slice.len() {
        if pos + 8 > slice.len() {
            return Err(FMOD_ERR_FORMAT);
        }
        let chunk_id = &slice[pos..pos + 4];
        let chunk_size = u32::from_le_bytes([
            slice[pos + 4],
            slice[pos + 5],
            slice[pos + 6],
            slice[pos + 7],
        ]) as usize;
        pos += 8;
        let payload_end = pos.checked_add(chunk_size).ok_or(FMOD_ERR_FORMAT)?;
        if payload_end > slice.len() {
            return Err(FMOD_ERR_FORMAT);
        }
        let payload = &slice[pos..payload_end];

        match chunk_id {
            b"FMT " => {
                if payload.len() < 4 {
                    return Err(FMOD_ERR_FORMAT);
                }
                st.recognized_chunks += 1;
            }
            b"LIST" => {
                if payload.len() < 4 {
                    return Err(FMOD_ERR_FORMAT);
                }
                st.recognized_chunks += 1;
                walk_fev_chunks(&payload[4..], st, depth + 1)?;
            }
            b"BNKI" => {
                if payload.len() < 16 {
                    return Err(FMOD_ERR_FORMAT);
                }
                if let Some(g) = rd_guid(payload, 0) {
                    if !g.is_zero() {
                        st.bank_guid = g;
                        st.has_explicit_guid = true;
                    }
                }
                st.recognized_chunks += 1;
            }
            b"STRT" => {
                if !parse_strt_chunk(payload, &mut st.strings) {
                    return Err(FMOD_ERR_FORMAT);
                }
                st.recognized_chunks += 1;
            }
            b"STRR" => {
                if !parse_radix_trie_strings(payload, &mut st.strings) {
                    return Err(FMOD_ERR_FORMAT);
                }
                st.recognized_chunks += 1;
            }
            b"STRI" => {
                if !parse_offset_pool_strings(payload, &mut st.strings) {
                    return Err(FMOD_ERR_FORMAT);
                }
                st.recognized_chunks += 1;
            }
            b"GUID" => {
                if !parse_inline_strings(payload, &mut st.strings) {
                    return Err(FMOD_ERR_FORMAT);
                }
                st.recognized_chunks += 1;
            }
            b"BUSS" => {
                parse_buss_chunk(payload, &mut st.buses)?;
                st.recognized_chunks += 1;
            }
            b"VCAS" => {
                parse_vcas_chunk(payload, &mut st.vcas)?;
                st.recognized_chunks += 1;
            }
            b"EVNT" => {
                parse_evnt_chunk(payload, &mut st.events)?;
                st.recognized_chunks += 1;
            }
            b"PARM" => {
                parse_parm_chunk(payload, &mut st.raw_params)?;
                st.recognized_chunks += 1;
            }
            b"SNDH" => {
                if payload.len() < 4 {
                    return Err(FMOD_ERR_FORMAT);
                }
                st.recognized_chunks += 1;
            }
            b"SND " | b"SNDA" | b"FSB5" => {
                parse_embedded_fsb5_chunk(payload, &mut st.samples)?;
                st.recognized_chunks += 1;
            }
            _ => {
                // Ignore unrecognized auxiliary RIFF chunks gracefully.
            }
        }

        let padded = chunk_size.saturating_add(chunk_size & 1);
        pos = pos.saturating_add(padded).min(slice.len());
    }
    Ok(())
}

fn parse_studio_bank(bytes: &[u8], source_path: &str) -> Result<ParsedBank, i32> {
    if bytes.len() >= 4 && &bytes[0..4] == b"FSB5" {
        let fsb = parse_fsb5(bytes)?;
        if fsb.samples.is_empty() {
            return Err(FMOD_ERR_FORMAT);
        }
        let bank_path = if source_path.is_empty() {
            "bank:/FSB5".to_string()
        } else {
            source_path.to_string()
        };
        let mut strings = Vec::with_capacity(fsb.samples.len());
        let mut events = Vec::with_capacity(fsb.samples.len());
        for (i, smp) in fsb.samples.iter().enumerate() {
            let ev_path = if smp.name.is_empty() {
                format!("event:/sample_{i}")
            } else if smp.name.starts_with("event:/") {
                smp.name.clone()
            } else {
                format!("event:/{}", smp.name.trim_start_matches('/'))
            };
            let ev_guid = FmodGuid::from_le_bytes(guid_from_uid(&ev_path));
            let length_ms = ((smp.num_frames as u64).saturating_mul(1000)
                / (smp.sample_rate.max(1) as u64)) as i32;
            strings.push((ev_guid, ev_path.clone()));
            events.push(ParsedEvent {
                guid: ev_guid,
                bus_guid: None,
                path: ev_path,
                sample_index: Some(i),
                is_3d: false,
                is_oneshot: !smp.has_loop,
                is_stream: false,
                is_snapshot: false,
                default_volume: 1.0,
                default_pitch: 1.0,
                min_distance: 1.0,
                max_distance: 20.0,
                length_ms,
                parameters: Vec::new(),
            });
        }
        return Ok(ParsedBank {
            guid: FmodGuid::ZERO,
            has_explicit_guid: false,
            path: bank_path,
            strings,
            samples: fsb.samples,
            buses: Vec::new(),
            vcas: Vec::new(),
            events,
            global_params: Vec::new(),
        });
    }

    if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"FEV " {
        log::warn!(
            target: "audio",
            "FMOD Studio LoadBank: invalid bank container ({}, {} bytes) -> FMOD_ERR_FORMAT",
            identify_container(bytes),
            bytes.len()
        );
        return Err(FMOD_ERR_FORMAT);
    }

    let riff_size = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as usize;
    let container_end = 8usize.checked_add(riff_size).ok_or(FMOD_ERR_FORMAT)?;
    if riff_size < 4 || container_end > bytes.len() {
        return Err(FMOD_ERR_FORMAT);
    }

    let mut st = FevWalkState {
        recognized_chunks: 0,
        bank_guid: FmodGuid::ZERO,
        has_explicit_guid: false,
        strings: Vec::new(),
        samples: Vec::new(),
        buses: Vec::new(),
        vcas: Vec::new(),
        events: Vec::new(),
        raw_params: Vec::new(),
    };
    walk_fev_chunks(&bytes[12..container_end], &mut st, 0)?;

    if st.recognized_chunks == 0 {
        return Err(FMOD_ERR_FORMAT);
    }

    // Build local GUID -> path map from this bank's string entries.
    let mut local_g2p: HashMap<FmodGuid, String> = HashMap::new();
    for (g, p) in &st.strings {
        local_g2p.insert(*g, p.clone());
    }

    let bank_path = if st.has_explicit_guid {
        local_g2p
            .get(&st.bank_guid)
            .cloned()
            .unwrap_or_else(|| source_path.to_string())
    } else {
        st.strings
            .iter()
            .find(|(_, p)| p.to_ascii_lowercase().starts_with("bank:/"))
            .map(|(_, p)| p.clone())
            .unwrap_or_else(|| source_path.to_string())
    };

    for bus in &mut st.buses {
        if bus.path.is_empty() {
            if let Some(p) = local_g2p.get(&bus.guid) {
                bus.path = p.clone();
            }
        }
    }
    for vca in &mut st.vcas {
        if vca.path.is_empty() {
            if let Some(p) = local_g2p.get(&vca.guid) {
                vca.path = p.clone();
            }
        }
    }
    for ev in &mut st.events {
        if ev.path.is_empty() {
            if let Some(p) = local_g2p.get(&ev.guid) {
                ev.path = p.clone();
            }
        }
    }

    // Attach parsed parameters either to matching events or global parameter list.
    let mut global_params = Vec::new();
    for (owner, pdef) in st.raw_params {
        if owner.is_zero() || (pdef.flags & FMOD_STUDIO_PARAMETER_GLOBAL) != 0 {
            global_params.push(pdef);
        } else if let Some(ev) = st.events.iter_mut().find(|e| e.guid == owner) {
            ev.parameters.push(pdef);
        } else {
            global_params.push(pdef);
        }
    }

    // If the bank carries embedded FSB5 samples but no explicit EVNT chunk, synthesize
    // events from any event:/ string entries or FSB5 sample names so real audio banks
    // with stripped/opaque MODL metadata still resolve and mix audio.
    if st.events.is_empty() && !st.samples.is_empty() {
        let mut matched_event_strings: Vec<(FmodGuid, String)> = st
            .strings
            .iter()
            .filter(|(_, p)| p.to_ascii_lowercase().starts_with("event:/"))
            .cloned()
            .collect();
        if !matched_event_strings.is_empty() {
            for (idx, (guid, path)) in matched_event_strings.drain(..).enumerate() {
                let leaf = path.rsplit('/').next().unwrap_or("");
                let sample_index = st
                    .samples
                    .iter()
                    .position(|s| !s.name.is_empty() && s.name.eq_ignore_ascii_case(leaf))
                    .or_else(|| (idx < st.samples.len()).then_some(idx));
                let length_ms = sample_index
                    .and_then(|si| st.samples.get(si))
                    .map(|s| {
                        ((s.num_frames as u64).saturating_mul(1000) / (s.sample_rate.max(1) as u64))
                            as i32
                    })
                    .unwrap_or(0);
                st.events.push(ParsedEvent {
                    guid,
                    bus_guid: None,
                    path,
                    sample_index,
                    is_3d: false,
                    is_oneshot: true,
                    is_stream: false,
                    is_snapshot: false,
                    default_volume: 1.0,
                    default_pitch: 1.0,
                    min_distance: 1.0,
                    max_distance: 20.0,
                    length_ms,
                    parameters: Vec::new(),
                });
            }
        } else {
            for (i, smp) in st.samples.iter().enumerate() {
                let path = if smp.name.is_empty() {
                    format!("event:/sample_{i}")
                } else if smp.name.starts_with("event:/") {
                    smp.name.clone()
                } else {
                    format!("event:/{}", smp.name.trim_start_matches('/'))
                };
                let guid = FmodGuid::from_le_bytes(guid_from_uid(&path));
                let length_ms = ((smp.num_frames as u64).saturating_mul(1000)
                    / (smp.sample_rate.max(1) as u64)) as i32;
                st.strings.push((guid, path.clone()));
                st.events.push(ParsedEvent {
                    guid,
                    bus_guid: None,
                    path,
                    sample_index: Some(i),
                    is_3d: false,
                    is_oneshot: !smp.has_loop,
                    is_stream: false,
                    is_snapshot: false,
                    default_volume: 1.0,
                    default_pitch: 1.0,
                    min_distance: 1.0,
                    max_distance: 20.0,
                    length_ms,
                    parameters: Vec::new(),
                });
            }
        }
    }

    Ok(ParsedBank {
        guid: st.bank_guid,
        has_explicit_guid: st.has_explicit_guid,
        path: bank_path,
        strings: st.strings,
        samples: st.samples,
        buses: st.buses,
        vcas: st.vcas,
        events: st.events,
        global_params,
    })
}

fn intern_cstr(s: &str) -> *const u8 {
    static INTERNED: OnceLock<Mutex<HashMap<String, usize>>> = OnceLock::new();
    let map = INTERNED.get_or_init(|| Mutex::new(HashMap::new()));
    let mut guard = map.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(&ptr) = guard.get(s) {
        return ptr as *const u8;
    }
    let mut bytes = Vec::with_capacity(s.len() + 1);
    bytes.extend_from_slice(s.as_bytes());
    bytes.push(0);
    let leaked: &'static [u8] = Box::leak(bytes.into_boxed_slice());
    let ptr = leaked.as_ptr() as usize;
    guard.insert(s.to_string(), ptr);
    ptr as *const u8
}

fn normalize_studio_path(path: &str) -> String {
    let lower = path.trim().to_ascii_lowercase();
    if lower.is_empty() || lower == "bus:/" {
        return lower;
    }
    lower.trim_end_matches('/').to_string()
}

unsafe fn write_studio_path(
    src: &str,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    let needed = (src.len() + 1) as i32;
    write_out(retrieved_out, needed);

    if size < 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    if path_out.is_null() {
        if size == 0 && !retrieved_out.is_null() {
            return FMOD_OK;
        }
        return FMOD_ERR_INVALID_PARAM;
    }
    if size == 0 {
        return FMOD_ERR_TRUNCATED;
    }

    let bytes = src.as_bytes();
    let max_copy = (size as usize) - 1;
    let copy_len = bytes.len().min(max_copy);
    if copy_len > 0 {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), path_out, copy_len);
    }
    std::ptr::write_unaligned(path_out.add(copy_len), 0u8);

    if (size as usize) < bytes.len() + 1 {
        FMOD_ERR_TRUNCATED
    } else {
        FMOD_OK
    }
}

unsafe fn write_handle_list(
    items: &[usize],
    array_out: *mut usize,
    capacity: i32,
    count_out: *mut i32,
) -> i32 {
    if capacity < 0 || (capacity > 0 && array_out.is_null()) {
        return FMOD_ERR_INVALID_PARAM;
    }
    let n = items.len().min(capacity as usize);
    for (i, &h) in items.iter().take(n).enumerate() {
        std::ptr::write_unaligned(array_out.add(i), h);
    }
    write_out(count_out, n as i32);
    FMOD_OK
}

fn resolve_sound_path(raw_path: &str) -> Option<PathBuf> {
    if let Some(host) = crate::path::to_host(raw_path) {
        if host.is_file() {
            return Some(host);
        }
    }
    let direct = Path::new(raw_path);
    if direct.is_file() {
        return Some(direct.to_path_buf());
    }
    None
}

fn guid_from_uid(uid: &str) -> [u8; 16] {
    let mut h1: u64 = 0xcbf2_9ce4_8422_2325;
    let mut h2: u64 = 0x8422_2325_cbf2_9ce4;
    for &b in uid.as_bytes() {
        h1 ^= b as u64;
        h1 = h1.wrapping_mul(0x0000_0100_0000_01b3);
        h2 = h2.wrapping_mul(0x0000_0100_0000_01b3);
        h2 ^= b as u64;
    }
    let mut g = [0u8; 16];
    g[0..8].copy_from_slice(&h1.to_le_bytes());
    g[8..16].copy_from_slice(&h2.to_le_bytes());
    g[7] = (g[7] & 0x0F) | 0x50;
    g[8] = (g[8] & 0x3F) | 0x80;
    g
}

fn speakermode_for_channels(channels: u32) -> u32 {
    match channels {
        1 => FMOD_SPEAKERMODE_MONO,
        2 => FMOD_SPEAKERMODE_STEREO,
        4 => FMOD_SPEAKERMODE_QUAD,
        6 => FMOD_SPEAKERMODE_5POINT1,
        8 => FMOD_SPEAKERMODE_7POINT1,
        _ => FMOD_SPEAKERMODE_STEREO,
    }
}

// ============================================================================
// Mixer state, 3D spatializer & CoreAudio output pump
// ============================================================================

#[derive(Clone, Copy, Debug)]
struct Listener3D {
    pos: FmodVector,
    vel: FmodVector,
    forward: FmodVector,
    up: FmodVector,
}

impl Default for Listener3D {
    fn default() -> Self {
        Self {
            pos: FmodVector { x: 0.0, y: 0.0, z: 0.0 },
            vel: FmodVector { x: 0.0, y: 0.0, z: 0.0 },
            forward: FmodVector { x: 0.0, y: 0.0, z: 1.0 },
            up: FmodVector { x: 0.0, y: 1.0, z: 0.0 },
        }
    }
}

impl Listener3D {
    /// Unit right vector in FMOD's left-handed coordinate system (`up x forward`).
    fn right_axis(&self) -> FmodVector {
        let rx = self.up.y * self.forward.z - self.up.z * self.forward.y;
        let ry = self.up.z * self.forward.x - self.up.x * self.forward.z;
        let rz = self.up.x * self.forward.y - self.up.y * self.forward.x;
        let len = (rx * rx + ry * ry + rz * rz).sqrt();
        if len > 1e-6 && len.is_finite() {
            FmodVector { x: rx / len, y: ry / len, z: rz / len }
        } else {
            FmodVector { x: 1.0, y: 0.0, z: 0.0 }
        }
    }
}

struct Sound {
    name: String,
    sound_type: u32,
    format: u32,
    channels: u32,
    sample_rate: u32,
    bits_per_sample: u32,
    default_frequency: f32,
    default_priority: i32,
    mode: u32,
    loop_count: i32,
    loop_start: usize,
    loop_end: usize,
    /// Pinned raw PCM buffer in `format` (never reallocated; safe to expose via Lock/Unlock).
    raw_pcm: Box<[u8]>,
    /// Decoded interleaved f32 samples (length = `num_frames * channels`).
    samples: Arc<Vec<f32>>,
    num_frames: usize,
    min_distance: f32,
    max_distance: f32,
    locked: bool,
    /// Decoded sub-sounds when created from a multi-sample FSB5 container.
    fsb_subsounds: Vec<DecodedSample>,
    /// Lazily materialized `Sound` handles for `FMOD_Sound_GetSubSound`.
    subsound_handles: Vec<Option<usize>>,
}

struct ChannelGroup {
    name: String,
    parent: Option<usize>,
    volume: f32,
    mute: bool,
    paused: bool,
    pitch: f32,
}

struct Channel {
    sound_id: usize,
    group_id: usize,
    samples: Arc<Vec<f32>>,
    sound_channels: usize,
    sound_sample_rate: u32,
    sound_frame_bytes: usize,
    num_frames: usize,
    loop_start: usize,
    loop_end: usize,
    play_pos: f64,
    frequency: f32,
    pitch: f32,
    volume: f32,
    pan: f32,
    mute: bool,
    paused: bool,
    stopped: bool,
    mode: u32,
    loop_count: i32,
    loops_remaining: i32,
    pos_3d: FmodVector,
    vel_3d: FmodVector,
    min_distance: f32,
    max_distance: f32,
}

struct EffectiveGroup {
    volume: f32,
    mute: bool,
    paused: bool,
    pitch: f32,
}

fn resolve_group_chain(groups: &HashMap<usize, ChannelGroup>, start: usize) -> EffectiveGroup {
    let mut volume = 1.0f32;
    let mut mute = false;
    let mut paused = false;
    let mut pitch = 1.0f32;
    let mut cur = Some(start);
    let mut hops = 0usize;
    while let Some(gid) = cur {
        if hops >= 64 {
            break;
        }
        hops += 1;
        let Some(g) = groups.get(&gid) else {
            break;
        };
        volume *= g.volume;
        mute |= g.mute;
        paused |= g.paused;
        pitch *= g.pitch;
        cur = g.parent;
    }
    EffectiveGroup { volume, mute, paused, pitch }
}

fn group_has_ancestor(groups: &HashMap<usize, ChannelGroup>, group_id: usize, ancestor: usize) -> bool {
    let mut cur = Some(group_id);
    let mut hops = 0usize;
    while let Some(gid) = cur {
        if gid == ancestor {
            return true;
        }
        if hops >= 64 {
            break;
        }
        hops += 1;
        cur = groups.get(&gid).and_then(|g| g.parent);
    }
    false
}

/// FMOD linear balance pan law: center (`0.0`) is unity `(1.0, 1.0)`,
/// `-1.0` is full left `(1.0, 0.0)`, `+1.0` is full right `(0.0, 1.0)`.
#[inline]
fn pan_gains(pan: f32) -> (f32, f32) {
    let p = pan.clamp(-1.0, 1.0);
    if p < 0.0 {
        (1.0, (1.0 + p).clamp(0.0, 1.0))
    } else if p > 0.0 {
        ((1.0 - p).clamp(0.0, 1.0), 1.0)
    } else {
        (1.0, 1.0)
    }
}

#[inline]
fn read_stereo_frame(samples: &[f32], channels: usize, frame: usize) -> (f32, f32) {
    let base = frame * channels;
    match channels {
        1 => {
            let s = samples[base];
            (s, s)
        }
        2 => (samples[base], samples[base + 1]),
        4 => (
            (samples[base] + samples[base + 2]) * 0.5,
            (samples[base + 1] + samples[base + 3]) * 0.5,
        ),
        n if n >= 6 => {
            const SQRT_HALF: f32 = core::f32::consts::FRAC_1_SQRT_2;
            let mut l = samples[base]
                + samples[base + 2] * SQRT_HALF
                + samples[base + 4] * SQRT_HALF;
            let mut r = samples[base + 1]
                + samples[base + 2] * SQRT_HALF
                + samples[base + 5] * SQRT_HALF;
            if n >= 8 {
                l += samples[base + 6] * SQRT_HALF;
                r += samples[base + 7] * SQRT_HALF;
            }
            (l, r)
        }
        _ => {
            let mut l = 0.0f32;
            let mut r = 0.0f32;
            let mut lc = 0u32;
            let mut rc = 0u32;
            for c in 0..channels {
                if c % 2 == 0 {
                    l += samples[base + c];
                    lc += 1;
                } else {
                    r += samples[base + c];
                    rc += 1;
                }
            }
            (
                if lc > 0 { l / lc as f32 } else { 0.0 },
                if rc > 0 { r / rc as f32 } else { 0.0 },
            )
        }
    }
}

struct MixerState {
    master_group_id: usize,
    initialized: bool,
    max_channels: usize,
    output_type: u32,
    driver_id: i32,
    doppler_scale: f32,
    distance_factor: f32,
    rolloff_scale: f32,
    listener: Listener3D,
    sounds: HashMap<usize, Sound>,
    groups: HashMap<usize, ChannelGroup>,
    channels: HashMap<usize, Channel>,
}

impl MixerState {
    fn new(master_group_id: usize) -> Self {
        let mut groups = HashMap::new();
        groups.insert(
            master_group_id,
            ChannelGroup {
                name: "Master".into(),
                parent: None,
                volume: 1.0,
                mute: false,
                paused: false,
                pitch: 1.0,
            },
        );
        Self {
            master_group_id,
            initialized: false,
            max_channels: 64,
            output_type: FMOD_OUTPUTTYPE_AUTODETECT,
            driver_id: 0,
            doppler_scale: 1.0,
            distance_factor: 1.0,
            rolloff_scale: 1.0,
            listener: Listener3D::default(),
            sounds: HashMap::new(),
            groups,
            channels: HashMap::new(),
        }
    }

    fn mix_frames(&mut self, out: &mut [f32]) -> usize {
        let frames = out.len() / 2;
        if frames == 0 {
            return 0;
        }
        out[..frames * 2].fill(0.0);

        let listener = self.listener;
        let right = listener.right_axis();
        let distance_factor = self.distance_factor.max(1e-6);
        let rolloff_scale = self.rolloff_scale.max(0.0);
        let groups = &self.groups;

        for ch in self.channels.values_mut() {
            if ch.stopped {
                continue;
            }
            if ch.num_frames == 0 || ch.sound_channels == 0 {
                ch.stopped = true;
                continue;
            }

            let grp = resolve_group_chain(groups, ch.group_id);
            if ch.paused || grp.paused {
                continue;
            }

            let effective_pitch = (ch.pitch * grp.pitch).max(0.0);
            let step = ((ch.frequency.abs() as f64) * (effective_pitch as f64)) / (MIX_RATE as f64);
            if !step.is_finite() || step <= 0.0 {
                continue;
            }

            let muted = ch.mute || grp.mute;
            let effective_vol = ch.volume * grp.volume;
            let is_3d = (ch.mode & FMOD_3D) != 0;

            let (gain_l, gain_r) = if muted || effective_vol == 0.0 {
                (0.0, 0.0)
            } else if is_3d {
                let rel = if (ch.mode & FMOD_3D_HEADRELATIVE) != 0 {
                    ch.pos_3d
                } else {
                    FmodVector {
                        x: ch.pos_3d.x - listener.pos.x,
                        y: ch.pos_3d.y - listener.pos.y,
                        z: ch.pos_3d.z - listener.pos.z,
                    }
                };
                let dist = (rel.x * rel.x + rel.y * rel.y + rel.z * rel.z).sqrt();
                let min_d = (ch.min_distance * distance_factor).max(1e-6);
                let max_d = (ch.max_distance * distance_factor).max(min_d);

                let atten = if dist <= min_d || rolloff_scale == 0.0 {
                    1.0
                } else if (ch.mode & FMOD_3D_LINEARROLLOFF) != 0 {
                    (1.0 - (dist - min_d) / (max_d - min_d).max(1e-6)).clamp(0.0, 1.0)
                } else if (ch.mode & FMOD_3D_LINEARSQUAREROLLOFF) != 0 {
                    let t = (1.0 - (dist - min_d) / (max_d - min_d).max(1e-6)).clamp(0.0, 1.0);
                    t * t
                } else {
                    let d = dist.min(max_d);
                    min_d / (min_d + rolloff_scale * (d - min_d))
                };

                let pan_3d = if dist > 1e-6 {
                    let inv = 1.0 / dist;
                    ((rel.x * inv) * right.x + (rel.y * inv) * right.y + (rel.z * inv) * right.z)
                        .clamp(-1.0, 1.0)
                } else {
                    0.0
                };
                let (pl, pr) = pan_gains((pan_3d + ch.pan).clamp(-1.0, 1.0));
                (effective_vol * atten * pl, effective_vol * atten * pr)
            } else {
                let (pl, pr) = pan_gains(ch.pan);
                (effective_vol * pl, effective_vol * pr)
            };

            let samples = &ch.samples;
            let nframes = ch.num_frames;
            let nch = ch.sound_channels;
            let loop_start = ch.loop_start.min(nframes);
            let loop_end = if ch.loop_end > loop_start && ch.loop_end <= nframes {
                ch.loop_end
            } else {
                nframes
            };
            let loop_len = (loop_end - loop_start).max(1) as f64;

            for f in 0..frames {
                while !ch.stopped {
                    let bound = if ch.loops_remaining != 0 {
                        loop_end as f64
                    } else {
                        nframes as f64
                    };
                    if ch.play_pos < bound {
                        break;
                    }
                    if ch.loops_remaining == -1 {
                        ch.play_pos = (loop_start as f64) + ((ch.play_pos - loop_end as f64) % loop_len);
                    } else if ch.loops_remaining > 0 {
                        ch.loops_remaining -= 1;
                        ch.play_pos -= loop_len;
                    } else {
                        ch.play_pos = nframes as f64;
                        ch.stopped = true;
                    }
                }
                if ch.stopped {
                    break;
                }

                if !muted && (gain_l != 0.0 || gain_r != 0.0) {
                    let idx0 = (ch.play_pos.floor() as usize).min(nframes - 1);
                    let frac = (ch.play_pos - idx0 as f64) as f32;
                    let idx1 = if ch.loops_remaining != 0 && idx0 + 1 >= loop_end {
                        loop_start
                    } else if idx0 + 1 < nframes {
                        idx0 + 1
                    } else if ch.loops_remaining != 0 {
                        loop_start
                    } else {
                        idx0
                    };

                    let (l0, r0) = read_stereo_frame(samples, nch, idx0);
                    let (l1, r1) = read_stereo_frame(samples, nch, idx1);
                    let sl = l0 + (l1 - l0) * frac;
                    let sr = r0 + (r1 - r0) * frac;

                    if is_3d {
                        let mono = (sl + sr) * 0.5;
                        out[f * 2] += mono * gain_l;
                        out[f * 2 + 1] += mono * gain_r;
                    } else {
                        out[f * 2] += sl * gain_l;
                        out[f * 2 + 1] += sr * gain_r;
                    }
                }

                ch.play_pos += step;
                while !ch.stopped {
                    let bound = if ch.loops_remaining != 0 {
                        loop_end as f64
                    } else {
                        nframes as f64
                    };
                    if ch.play_pos < bound {
                        break;
                    }
                    if ch.loops_remaining == -1 {
                        ch.play_pos = (loop_start as f64) + ((ch.play_pos - loop_end as f64) % loop_len);
                    } else if ch.loops_remaining > 0 {
                        ch.loops_remaining -= 1;
                        ch.play_pos -= loop_len;
                    } else {
                        ch.play_pos = nframes as f64;
                        ch.stopped = true;
                    }
                }
            }
        }

        for s in &mut out[..frames * 2] {
            *s = if s.is_finite() { s.clamp(-1.0, 1.0) } else { 0.0 };
        }
        frames
    }
}

struct AudioOutput {
    stream: OutputStream,
    stop: Arc<AtomicBool>,
    wake: Arc<(Mutex<bool>, Condvar)>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Drop for AudioOutput {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        {
            let (lock, cv) = &*self.wake;
            if let Ok(mut stopped) = lock.lock() {
                *stopped = true;
                cv.notify_all();
            }
        }
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
        self.stream.stop();
    }
}

struct FmodSystem {
    mixer: Arc<Mutex<MixerState>>,
    audio: Mutex<Option<AudioOutput>>,
    mixed_frames: Arc<AtomicU64>,
}

struct Registry {
    systems: HashMap<usize, Arc<FmodSystem>>,
    studios: HashMap<usize, Arc<StudioSystem>>,
    studio_banks: HashMap<usize, Arc<StudioSystem>>,
    studio_events: HashMap<usize, Arc<StudioSystem>>,
    studio_instances: HashMap<usize, Arc<StudioSystem>>,
    studio_buses: HashMap<usize, Arc<StudioSystem>>,
    studio_vcas: HashMap<usize, Arc<StudioSystem>>,
    sounds: HashMap<usize, Arc<FmodSystem>>,
    groups: HashMap<usize, Arc<FmodSystem>>,
    channels: HashMap<usize, Arc<FmodSystem>>,
}

impl Registry {
    fn new() -> Self {
        Self {
            systems: HashMap::new(),
            studios: HashMap::new(),
            studio_banks: HashMap::new(),
            studio_events: HashMap::new(),
            studio_instances: HashMap::new(),
            studio_buses: HashMap::new(),
            studio_vcas: HashMap::new(),
            sounds: HashMap::new(),
            groups: HashMap::new(),
            channels: HashMap::new(),
        }
    }
}

static NEXT_FMOD_HANDLE: AtomicUsize = AtomicUsize::new(0xF60D_0001);

fn next_handle() -> usize {
    NEXT_FMOD_HANDLE.fetch_add(1, Ordering::Relaxed)
}

fn registry() -> &'static Mutex<Registry> {
    static REG: OnceLock<Mutex<Registry>> = OnceLock::new();
    REG.get_or_init(|| Mutex::new(Registry::new()))
}

fn lookup_system(handle: usize) -> Result<Arc<FmodSystem>, i32> {
    let reg = registry().lock().unwrap_or_else(|e| e.into_inner());
    reg.systems.get(&handle).cloned().ok_or(FMOD_ERR_INVALID_HANDLE)
}

fn lookup_sound_system(handle: usize) -> Result<Arc<FmodSystem>, i32> {
    let reg = registry().lock().unwrap_or_else(|e| e.into_inner());
    reg.sounds.get(&handle).cloned().ok_or(FMOD_ERR_INVALID_HANDLE)
}

fn lookup_group_system(handle: usize) -> Result<Arc<FmodSystem>, i32> {
    let reg = registry().lock().unwrap_or_else(|e| e.into_inner());
    reg.groups.get(&handle).cloned().ok_or(FMOD_ERR_INVALID_HANDLE)
}

fn lookup_channel_system(handle: usize) -> Result<Arc<FmodSystem>, i32> {
    let reg = registry().lock().unwrap_or_else(|e| e.into_inner());
    reg.channels.get(&handle).cloned().ok_or(FMOD_ERR_INVALID_HANDLE)
}

fn lookup_studio(studio: usize) -> Result<Arc<StudioSystem>, i32> {
    let reg = registry().lock().unwrap_or_else(|e| e.into_inner());
    reg.studios.get(&studio).cloned().ok_or(FMOD_ERR_INVALID_HANDLE)
}

fn lookup_studio_bank(bank: usize) -> Result<Arc<StudioSystem>, i32> {
    let reg = registry().lock().unwrap_or_else(|e| e.into_inner());
    reg.studio_banks.get(&bank).cloned().ok_or(FMOD_ERR_INVALID_HANDLE)
}

fn lookup_studio_event(event: usize) -> Result<Arc<StudioSystem>, i32> {
    let reg = registry().lock().unwrap_or_else(|e| e.into_inner());
    reg.studio_events.get(&event).cloned().ok_or(FMOD_ERR_INVALID_HANDLE)
}

fn lookup_studio_instance(instance: usize) -> Result<Arc<StudioSystem>, i32> {
    let reg = registry().lock().unwrap_or_else(|e| e.into_inner());
    reg.studio_instances
        .get(&instance)
        .cloned()
        .ok_or(FMOD_ERR_INVALID_HANDLE)
}

fn lookup_studio_bus(bus: usize) -> Result<Arc<StudioSystem>, i32> {
    let reg = registry().lock().unwrap_or_else(|e| e.into_inner());
    reg.studio_buses.get(&bus).cloned().ok_or(FMOD_ERR_INVALID_HANDLE)
}

fn lookup_studio_vca(vca: usize) -> Result<Arc<StudioSystem>, i32> {
    let reg = registry().lock().unwrap_or_else(|e| e.into_inner());
    reg.studio_vcas.get(&vca).cloned().ok_or(FMOD_ERR_INVALID_HANDLE)
}

fn create_core_system_arc() -> (usize, usize, Arc<FmodSystem>) {
    let sys_id = next_handle();
    let master_id = next_handle();
    let sys = Arc::new(FmodSystem {
        mixer: Arc::new(Mutex::new(MixerState::new(master_id))),
        audio: Mutex::new(None),
        mixed_frames: Arc::new(AtomicU64::new(0)),
    });
    let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
    reg.systems.insert(sys_id, sys.clone());
    reg.groups.insert(master_id, sys.clone());
    (sys_id, master_id, sys)
}

fn create_core_system() -> usize {
    create_core_system_arc().0
}

fn start_audio_output(
    sys: &Arc<FmodSystem>,
    output_type: u32,
    driver_id: i32,
) -> Result<Option<AudioOutput>, i32> {
    if output_type == FMOD_OUTPUTTYPE_NOSOUND_NRT {
        return Ok(None);
    }

    let dev = if output_type == FMOD_OUTPUTTYPE_NOSOUND {
        backend::null_device()
    } else {
        let devs = backend::output_devices();
        if driver_id >= 0 {
            devs.get(driver_id as usize)
                .or_else(|| devs.first())
                .cloned()
                .unwrap_or_else(backend::null_device)
        } else {
            devs.into_iter().next().unwrap_or_else(backend::null_device)
        }
    };

    let open_and_start =
        |d: &backend::DeviceInfo| -> Result<(OutputStream, Arc<StreamShared>), String> {
            let ring = Ring::new(MIX_RING_FRAMES, MIX_CHANNELS as usize);
            let shared = StreamShared::new(ring);
            let mut s = OutputStream::open(d, MIX_RATE, MIX_CHANNELS, shared.clone(), MIX_IO_FRAMES)?;
            s.start()?;
            Ok((s, shared))
        };

    let (stream, shared) = match open_and_start(&dev) {
        Ok(pair) => pair,
        Err(e) => {
            log::warn!(
                target: "audio",
                "fmod: audio stream init/start failed on '{}' ({e}); falling back to null output",
                dev.name
            );
            let null_dev = backend::null_device();
            open_and_start(&null_dev).map_err(|_| FMOD_ERR_OUTPUT_INIT)?
        }
    };

    log::info!(
        target: "audio",
        "fmod: initialized {} mixer at {} Hz stereo ({:?})",
        dev.name,
        MIX_RATE,
        stream.kind
    );

    let stop = Arc::new(AtomicBool::new(false));
    let wake = Arc::new((Mutex::new(false), Condvar::new()));
    let stop_t = stop.clone();
    let wake_t = wake.clone();
    let mixer_arc = sys.mixer.clone();
    let mixed_counter = sys.mixed_frames.clone();

    let thread = std::thread::Builder::new()
        .name("fmod-mixer".into())
        .spawn(move || {
            let mut block = [0.0f32; MIX_BLOCK_FRAMES * 2];
            while !stop_t.load(Ordering::Acquire) {
                let mut queued = shared.ring.len();
                while queued < MIX_TARGET_BUFFER_FRAMES && !stop_t.load(Ordering::Acquire) {
                    {
                        let mut m = mixer_arc.lock().unwrap_or_else(|e| e.into_inner());
                        m.mix_frames(&mut block);
                    }
                    let pushed = shared.ring.push(&block);
                    if pushed == 0 {
                        break;
                    }
                    mixed_counter.fetch_add(pushed as u64, Ordering::Relaxed);
                    queued += pushed;
                }
                let (lock, cv) = &*wake_t;
                let guard = lock.lock().unwrap_or_else(|e| e.into_inner());
                if !*guard {
                    let _ = cv.wait_timeout(guard, Duration::from_millis(4));
                }
            }
        })
        .map_err(|_| FMOD_ERR_OUTPUT_INIT)?;

    Ok(Some(AudioOutput {
        stream,
        stop,
        wake,
        thread: Some(thread),
    }))
}

/// Pull mixed interleaved 48 kHz stereo float32 frames directly from `system`.
/// Primarily used for non-realtime (`FMOD_OUTPUTTYPE_NOSOUND_NRT`) mixing and tests.
pub fn mix_system_frames(system: usize, out_interleaved_stereo: &mut [f32]) -> Result<usize, i32> {
    let sys = lookup_system(system)?;
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let n = m.mix_frames(out_interleaved_stereo);
    sys.mixed_frames.fetch_add(n as u64, Ordering::Relaxed);
    Ok(n)
}

// ============================================================================
// Internal implementations shared across x86_64 and aarch64 ABI adapters
// ============================================================================

fn system_init_impl(system: usize, maxchannels: i32, _flags: u32, _extradriverdata: usize) -> i32 {
    if maxchannels <= 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let (output_type, driver_id) = {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        if m.initialized {
            return FMOD_ERR_INITIALIZED;
        }
        m.max_channels = maxchannels as usize;
        m.initialized = true;
        (m.output_type, m.driver_id)
    };

    match start_audio_output(&sys, output_type, driver_id) {
        Ok(audio_opt) => {
            *sys.audio.lock().unwrap_or_else(|e| e.into_inner()) = audio_opt;
            FMOD_OK
        }
        Err(e) => {
            let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
            m.initialized = false;
            e
        }
    }
}

fn system_close_impl(system: usize) -> i32 {
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let audio = sys.audio.lock().unwrap_or_else(|e| e.into_inner()).take();
    drop(audio);

    let channel_ids: Vec<usize> = {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        let ids = m.channels.keys().copied().collect();
        m.channels.clear();
        m.initialized = false;
        ids
    };
    {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        for cid in channel_ids {
            reg.channels.remove(&cid);
        }
    }
    log::debug!(
        target: "audio",
        "fmod: closed system {system:#x} (mixed {} frames)",
        sys.mixed_frames.load(Ordering::Relaxed)
    );
    FMOD_OK
}

fn system_release_impl(system: usize) -> i32 {
    let sys = {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        match reg.systems.remove(&system) {
            Some(s) => s,
            None => return FMOD_ERR_INVALID_HANDLE,
        }
    };
    let audio = sys.audio.lock().unwrap_or_else(|e| e.into_inner()).take();
    drop(audio);

    let (sound_ids, group_ids, channel_ids): (Vec<usize>, Vec<usize>, Vec<usize>) = {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        let s = m.sounds.keys().copied().collect();
        let g = m.groups.keys().copied().collect();
        let c = m.channels.keys().copied().collect();
        m.sounds.clear();
        m.groups.clear();
        m.channels.clear();
        m.initialized = false;
        (s, g, c)
    };
    {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        for id in sound_ids {
            reg.sounds.remove(&id);
        }
        for id in group_ids {
            reg.groups.remove(&id);
        }
        for id in channel_ids {
            reg.channels.remove(&id);
        }
    }
    log::debug!(
        target: "audio",
        "fmod: released system {system:#x} (mixed {} frames)",
        sys.mixed_frames.load(Ordering::Relaxed)
    );
    FMOD_OK
}

fn system_update_impl(system: usize) -> i32 {
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let pruned: Vec<usize> = {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        if m.channels.len() > 4096 {
            let dead: Vec<usize> = m
                .channels
                .iter()
                .filter(|(_, ch)| ch.stopped)
                .map(|(&id, _)| id)
                .collect();
            for id in &dead {
                m.channels.remove(id);
            }
            dead
        } else {
            Vec::new()
        }
    };
    if !pruned.is_empty() {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        for id in pruned {
            reg.channels.remove(&id);
        }
    }
    FMOD_OK
}

fn system_set_3d_settings_impl(
    system: usize,
    dopplerscale: f32,
    distancefactor: f32,
    rolloffscale: f32,
) -> i32 {
    if !dopplerscale.is_finite()
        || !distancefactor.is_finite()
        || !rolloffscale.is_finite()
        || dopplerscale < 0.0
        || distancefactor <= 0.0
        || rolloffscale < 0.0
    {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    m.doppler_scale = dopplerscale;
    m.distance_factor = distancefactor;
    m.rolloff_scale = rolloffscale;
    FMOD_OK
}

fn sound_set_defaults_impl(sound: usize, frequency: f32, priority: i32) -> i32 {
    if !frequency.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get_mut(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    s.default_frequency = frequency;
    s.default_priority = priority.clamp(0, 256);
    FMOD_OK
}

fn sound_set_3d_min_max_distance_impl(sound: usize, min: f32, max: f32) -> i32 {
    if !min.is_finite() || !max.is_finite() || min < 0.0 || max < min {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get_mut(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    s.min_distance = min;
    s.max_distance = max;
    FMOD_OK
}

fn channel_set_volume_impl(channel: usize, volume: f32) -> i32 {
    if !volume.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.volume = volume;
    FMOD_OK
}

fn channel_set_pitch_impl(channel: usize, pitch: f32) -> i32 {
    if !pitch.is_finite() || pitch < 0.0 {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.pitch = pitch;
    FMOD_OK
}

fn channel_set_pan_impl(channel: usize, pan: f32) -> i32 {
    if !pan.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.pan = pan.clamp(-1.0, 1.0);
    FMOD_OK
}

fn channel_set_frequency_impl(channel: usize, frequency: f32) -> i32 {
    if !frequency.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.frequency = frequency;
    FMOD_OK
}

fn channel_set_3d_min_max_distance_impl(channel: usize, mindistance: f32, maxdistance: f32) -> i32 {
    if !mindistance.is_finite()
        || !maxdistance.is_finite()
        || mindistance < 0.0
        || maxdistance < mindistance
    {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.min_distance = mindistance;
    ch.max_distance = maxdistance;
    FMOD_OK
}

fn channel_group_set_volume_impl(channelgroup: usize, volume: f32) -> i32 {
    if !volume.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get_mut(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    g.volume = volume;
    FMOD_OK
}

fn channel_group_set_pitch_impl(channelgroup: usize, pitch: f32) -> i32 {
    if !pitch.is_finite() || pitch < 0.0 {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get_mut(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    g.pitch = pitch;
    FMOD_OK
}

// ============================================================================
// FMOD Core (`fmod.dll` / `fmod64.dll`) — System
// ============================================================================

pub unsafe extern "win64" fn FMOD_System_Create(
    system_out: *mut usize,
    _header_version: u32,
) -> i32 {
    if system_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let id = create_core_system();
    write_out(system_out, id);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_System_Init(
    system: usize,
    maxchannels: i32,
    flags: u32,
    extradriverdata: usize,
) -> i32 {
    system_init_impl(system, maxchannels, flags, extradriverdata)
}

pub unsafe extern "win64" fn FMOD_System_Close(system: usize) -> i32 {
    system_close_impl(system)
}

pub unsafe extern "win64" fn FMOD_System_Release(system: usize) -> i32 {
    system_release_impl(system)
}

pub unsafe extern "win64" fn FMOD_System_Update(system: usize) -> i32 {
    system_update_impl(system)
}

pub unsafe extern "win64" fn FMOD_System_GetVersion(
    system: usize,
    version_out: *mut u32,
) -> i32 {
    if lookup_system(system).is_err() {
        return FMOD_ERR_INVALID_HANDLE;
    }
    if version_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    write_out(version_out, FMOD_VERSION);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_System_SetOutput(system: usize, output: u32) -> i32 {
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    m.output_type = output;
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_System_GetOutput(
    system: usize,
    output_out: *mut u32,
) -> i32 {
    if output_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let out = sys.mixer.lock().unwrap_or_else(|e| e.into_inner()).output_type;
    write_out(output_out, out);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_System_GetNumDrivers(
    system: usize,
    numdrivers_out: *mut i32,
) -> i32 {
    if lookup_system(system).is_err() {
        return FMOD_ERR_INVALID_HANDLE;
    }
    if numdrivers_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let devs = backend::output_devices();
    write_out(numdrivers_out, devs.len() as i32);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_System_GetDriverInfo(
    system: usize,
    id: i32,
    name_out: *mut u8,
    namelen: i32,
    guid: usize,
    systemrate: *mut i32,
    speakermode: *mut u32,
    speakermodechannels: *mut i32,
) -> i32 {
    if lookup_system(system).is_err() {
        return FMOD_ERR_INVALID_HANDLE;
    }
    if id < 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let devs = backend::output_devices();
    let Some(dev) = devs.get(id as usize) else {
        return FMOD_ERR_INVALID_PARAM;
    };

    if !name_out.is_null() && namelen > 0 {
        let bytes = dev.name.as_bytes();
        let copy_len = bytes.len().min((namelen as usize) - 1);
        if copy_len > 0 {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), name_out, copy_len);
        }
        std::ptr::write_unaligned(name_out.add(copy_len), 0u8);
    }
    if guid != 0 {
        let g = guid_from_uid(&dev.uid);
        std::ptr::copy_nonoverlapping(g.as_ptr(), guid as *mut u8, 16);
    }
    write_out(systemrate, dev.rate.max(8000) as i32);
    let ch = dev.channels.max(1);
    write_out(speakermode, speakermode_for_channels(ch));
    write_out(speakermodechannels, ch as i32);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_System_SetDriver(system: usize, driver: i32) -> i32 {
    if driver < 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let devs = backend::output_devices();
    if (driver as usize) >= devs.len() {
        return FMOD_ERR_INVALID_PARAM;
    }
    sys.mixer.lock().unwrap_or_else(|e| e.into_inner()).driver_id = driver;
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_System_GetDriver(
    system: usize,
    driver_out: *mut i32,
) -> i32 {
    if driver_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let d = sys.mixer.lock().unwrap_or_else(|e| e.into_inner()).driver_id;
    write_out(driver_out, d);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_System_GetMasterChannelGroup(
    system: usize,
    group_out: *mut usize,
) -> i32 {
    if group_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let master_id = sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .master_group_id;
    write_out(group_out, master_id);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_System_CreateChannelGroup(
    system: usize,
    name: *const u8,
    group_out: *mut usize,
) -> i32 {
    if group_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let group_name = if name.is_null() {
        String::new()
    } else {
        crate::strings::from_ansi(name)
    };
    let group_id = next_handle();
    {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        let master_id = m.master_group_id;
        m.groups.insert(
            group_id,
            ChannelGroup {
                name: group_name,
                parent: Some(master_id),
                volume: 1.0,
                mute: false,
                paused: false,
                pitch: 1.0,
            },
        );
    }
    {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        reg.groups.insert(group_id, sys);
    }
    write_out(group_out, group_id);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_System_Set3DListenerAttributes(
    system: usize,
    listener: i32,
    pos: *const FmodVector,
    vel: *const FmodVector,
    forward: *const FmodVector,
    up: *const FmodVector,
) -> i32 {
    if listener < 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let p_opt = read_in(pos);
    let v_opt = read_in(vel);
    let f_opt = read_in(forward);
    let u_opt = read_in(up);
    for vec in [p_opt, v_opt, f_opt, u_opt].into_iter().flatten() {
        if !vec.x.is_finite() || !vec.y.is_finite() || !vec.z.is_finite() {
            return FMOD_ERR_INVALID_FLOAT;
        }
    }
    if listener == 0 {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(p) = p_opt {
            m.listener.pos = p;
        }
        if let Some(v) = v_opt {
            m.listener.vel = v;
        }
        if let Some(f) = f_opt {
            m.listener.forward = f;
        }
        if let Some(u) = u_opt {
            m.listener.up = u;
        }
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_System_Get3DListenerAttributes(
    system: usize,
    listener: i32,
    pos: *mut FmodVector,
    vel: *mut FmodVector,
    forward: *mut FmodVector,
    up: *mut FmodVector,
) -> i32 {
    if listener < 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let l = sys.mixer.lock().unwrap_or_else(|e| e.into_inner()).listener;
    write_out(pos, l.pos);
    write_out(vel, l.vel);
    write_out(forward, l.forward);
    write_out(up, l.up);
    FMOD_OK
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_System_Set3DSettings(
    system: usize,
    dopplerscale: f32,
    distancefactor: f32,
    rolloffscale: f32,
) -> i32 {
    system_set_3d_settings_impl(system, dopplerscale, distancefactor, rolloffscale)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_System_Set3DSettings(
    system: usize,
    _xmm0: f32,
    dopplerscale: f32,
    distancefactor: f32,
    rolloffscale: f32,
) -> i32 {
    system_set_3d_settings_impl(system, dopplerscale, distancefactor, rolloffscale)
}

pub unsafe extern "win64" fn FMOD_System_Get3DSettings(
    system: usize,
    dopplerscale: *mut f32,
    distancefactor: *mut f32,
    rolloffscale: *mut f32,
) -> i32 {
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    write_out(dopplerscale, m.doppler_scale);
    write_out(distancefactor, m.distance_factor);
    write_out(rolloffscale, m.rolloff_scale);
    FMOD_OK
}

// ============================================================================
// FMOD Core — Sound creation & Sound methods
// ============================================================================

fn sound_from_decoded_sample(sample: &DecodedSample, mode: u32) -> Sound {
    let mut effective_mode = mode;
    let loop_count = if (mode & FMOD_LOOP_OFF) != 0 {
        0
    } else if (mode & (FMOD_LOOP_NORMAL | FMOD_LOOP_BIDI)) != 0 {
        -1
    } else if sample.has_loop {
        effective_mode |= FMOD_LOOP_NORMAL;
        -1
    } else {
        0
    };
    Sound {
        name: sample.name.clone(),
        sound_type: FMOD_SOUND_TYPE_FSB,
        format: sample.fmod_format,
        channels: sample.channels,
        sample_rate: sample.sample_rate,
        bits_per_sample: sample.bits_per_sample,
        default_frequency: sample.sample_rate as f32,
        default_priority: 128,
        mode: effective_mode,
        loop_count,
        loop_start: sample.loop_start,
        loop_end: sample.loop_end,
        raw_pcm: sample.raw_pcm.clone().into_boxed_slice(),
        samples: sample.samples.clone(),
        num_frames: sample.num_frames,
        min_distance: 1.0,
        max_distance: 10_000.0,
        locked: false,
        fsb_subsounds: Vec::new(),
        subsound_handles: Vec::new(),
    }
}

pub unsafe extern "win64" fn FMOD_System_CreateSound(
    system: usize,
    name_or_data: *const u8,
    mode: u32,
    exinfo: usize,
    sound_out: *mut usize,
) -> i32 {
    write_out(sound_out, 0);
    if sound_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let ex = match read_exinfo(exinfo) {
        Ok(v) => v,
        Err(e) => return e,
    };

    let open_user = (mode & FMOD_OPENUSER) != 0;
    let open_mem = (mode & (FMOD_OPENMEMORY | FMOD_OPENMEMORY_POINT)) != 0;
    let open_raw = (mode & FMOD_OPENRAW) != 0;

    let sound = if open_user {
        let Some(info) = ex else {
            return FMOD_ERR_INVALID_PARAM;
        };
        if info.numchannels <= 0 || info.numchannels > 32 || info.defaultfrequency <= 0 || info.length == 0 {
            return FMOD_ERR_INVALID_PARAM;
        }
        let Some((bits, bps)) = format_bits_and_bytes(info.format) else {
            log::warn!(target: "audio", "FMOD_System_CreateSound(OPENUSER): unsupported PCM format {}", info.format);
            return FMOD_ERR_FORMAT;
        };
        let frame_bytes = (info.numchannels as usize) * bps;
        let aligned_len = ((info.length as usize) / frame_bytes) * frame_bytes;
        let raw = if info.format == FMOD_SOUND_FORMAT_PCM8 {
            vec![128u8; aligned_len]
        } else {
            vec![0u8; aligned_len]
        };
        let Some(decoded) = decode_pcm_to_f32(&raw, info.format) else {
            return FMOD_ERR_FORMAT;
        };
        let num_frames = decoded.len() / (info.numchannels as usize).max(1);
        let loop_count = if (mode & (FMOD_LOOP_NORMAL | FMOD_LOOP_BIDI)) != 0 { -1 } else { 0 };
        Sound {
            name: String::new(),
            sound_type: FMOD_SOUND_TYPE_USER,
            format: info.format,
            channels: info.numchannels as u32,
            sample_rate: info.defaultfrequency as u32,
            bits_per_sample: bits,
            default_frequency: info.defaultfrequency as f32,
            default_priority: 128,
            mode,
            loop_count,
            loop_start: 0,
            loop_end: num_frames,
            raw_pcm: raw.into_boxed_slice(),
            samples: Arc::new(decoded),
            num_frames,
            min_distance: 1.0,
            max_distance: 10_000.0,
            locked: false,
            fsb_subsounds: Vec::new(),
            subsound_handles: Vec::new(),
        }
    } else {
        let mut sound_name = String::new();
        let source_bytes: Vec<u8> = if open_mem {
            let Some(info) = ex else {
                return FMOD_ERR_INVALID_PARAM;
            };
            if name_or_data.is_null() || info.length == 0 {
                return FMOD_ERR_INVALID_PARAM;
            }
            let start = name_or_data.add(info.fileoffset as usize);
            std::slice::from_raw_parts(start, info.length as usize).to_vec()
        } else {
            if name_or_data.is_null() {
                return FMOD_ERR_INVALID_PARAM;
            }
            let path_str = crate::strings::from_ansi(name_or_data);
            sound_name = path_str.clone();
            let Some(host_path) = resolve_sound_path(&path_str) else {
                return FMOD_ERR_FILE_NOTFOUND;
            };
            let data = match std::fs::read(&host_path) {
                Ok(d) => d,
                Err(_) => return FMOD_ERR_FILE_BAD,
            };
            if let Some(info) = ex {
                let off = info.fileoffset as usize;
                if off > data.len() {
                    return FMOD_ERR_FILE_EOF;
                }
                let rem = &data[off..];
                if info.length > 0 {
                    rem[..(info.length as usize).min(rem.len())].to_vec()
                } else {
                    rem.to_vec()
                }
            } else {
                data
            }
        };

        if open_raw {
            let Some(info) = ex else {
                return FMOD_ERR_INVALID_PARAM;
            };
            if info.numchannels <= 0 || info.numchannels > 32 || info.defaultfrequency <= 0 {
                return FMOD_ERR_INVALID_PARAM;
            }
            let Some((bits, bps)) = format_bits_and_bytes(info.format) else {
                log::warn!(target: "audio", "FMOD_System_CreateSound(OPENRAW): unsupported PCM format {}", info.format);
                return FMOD_ERR_FORMAT;
            };
            let frame_bytes = (info.numchannels as usize) * bps;
            let aligned_len = (source_bytes.len() / frame_bytes) * frame_bytes;
            let raw = source_bytes[..aligned_len].to_vec();
            let Some(decoded) = decode_pcm_to_f32(&raw, info.format) else {
                return FMOD_ERR_FORMAT;
            };
            let num_frames = decoded.len() / (info.numchannels as usize).max(1);
            let loop_count = if (mode & (FMOD_LOOP_NORMAL | FMOD_LOOP_BIDI)) != 0 { -1 } else { 0 };
            Sound {
                name: sound_name,
                sound_type: FMOD_SOUND_TYPE_RAW,
                format: info.format,
                channels: info.numchannels as u32,
                sample_rate: info.defaultfrequency as u32,
                bits_per_sample: bits,
                default_frequency: info.defaultfrequency as f32,
                default_priority: 128,
                mode,
                loop_count,
                loop_start: 0,
                loop_end: num_frames,
                raw_pcm: raw.into_boxed_slice(),
                samples: Arc::new(decoded),
                num_frames,
                min_distance: 1.0,
                max_distance: 10_000.0,
                locked: false,
                fsb_subsounds: Vec::new(),
                subsound_handles: Vec::new(),
            }
        } else if source_bytes.len() >= 4 && &source_bytes[0..4] == b"FSB5" {
            let fsb = match parse_fsb5(&source_bytes) {
                Ok(f) => f,
                Err(e) => return e,
            };
            if fsb.samples.is_empty() {
                return FMOD_ERR_FORMAT;
            }
            let mut snd = sound_from_decoded_sample(&fsb.samples[0], mode);
            if snd.name.is_empty() {
                snd.name = sound_name;
            }
            let nsub = fsb.samples.len();
            snd.fsb_subsounds = fsb.samples;
            snd.subsound_handles = vec![None; nsub];
            snd
        } else {
            let wav = match parse_wav(&source_bytes) {
                Ok(w) => w,
                Err(e) => return e,
            };
            let Some(decoded) = decode_pcm_to_f32(&wav.raw_pcm, wav.format) else {
                return FMOD_ERR_FORMAT;
            };
            let num_frames = decoded.len() / (wav.channels as usize).max(1);
            let loop_count = if (mode & (FMOD_LOOP_NORMAL | FMOD_LOOP_BIDI)) != 0 { -1 } else { 0 };
            Sound {
                name: sound_name,
                sound_type: FMOD_SOUND_TYPE_WAV,
                format: wav.format,
                channels: wav.channels,
                sample_rate: wav.sample_rate,
                bits_per_sample: wav.bits_per_sample,
                default_frequency: wav.sample_rate as f32,
                default_priority: 128,
                mode,
                loop_count,
                loop_start: 0,
                loop_end: num_frames,
                raw_pcm: wav.raw_pcm.into_boxed_slice(),
                samples: Arc::new(decoded),
                num_frames,
                min_distance: 1.0,
                max_distance: 10_000.0,
                locked: false,
                fsb_subsounds: Vec::new(),
                subsound_handles: Vec::new(),
            }
        }
    };

    let sound_id = next_handle();
    {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        m.sounds.insert(sound_id, sound);
    }
    {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        reg.sounds.insert(sound_id, sys);
    }
    write_out(sound_out, sound_id);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_System_CreateStream(
    system: usize,
    name_or_data: *const u8,
    mode: u32,
    exinfo: usize,
    sound_out: *mut usize,
) -> i32 {
    FMOD_System_CreateSound(system, name_or_data, mode | FMOD_CREATESTREAM, exinfo, sound_out)
}

pub unsafe extern "win64" fn FMOD_Sound_Release(sound: usize) -> i32 {
    let sys = {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        match reg.sounds.remove(&sound) {
            Some(s) => s,
            None => return FMOD_ERR_INVALID_HANDLE,
        }
    };
    let sub_ids: Vec<usize> = {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        let subs: Vec<usize> = m
            .sounds
            .remove(&sound)
            .map(|s| s.subsound_handles.into_iter().flatten().collect())
            .unwrap_or_default();
        for &sub in &subs {
            m.sounds.remove(&sub);
        }
        for ch in m.channels.values_mut() {
            if ch.sound_id == sound || subs.contains(&ch.sound_id) {
                ch.stopped = true;
            }
        }
        subs
    };
    if !sub_ids.is_empty() {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        for id in sub_ids {
            reg.sounds.remove(&id);
        }
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Sound_GetNumSubSounds(
    sound: usize,
    numsubsounds_out: *mut i32,
) -> i32 {
    if numsubsounds_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(numsubsounds_out, s.fsb_subsounds.len() as i32);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Sound_GetSubSound(
    sound: usize,
    index: i32,
    subsound_out: *mut usize,
) -> i32 {
    write_out(subsound_out, 0);
    if subsound_out.is_null() || index < 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let idx = index as usize;
    let (sub_id, newly_created) = {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        let Some(parent) = m.sounds.get(&sound) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        if idx >= parent.fsb_subsounds.len() {
            return FMOD_ERR_INVALID_PARAM;
        }
        if let Some(existing) = parent.subsound_handles.get(idx).copied().flatten() {
            if m.sounds.contains_key(&existing) {
                (existing, false)
            } else {
                let sample = parent.fsb_subsounds[idx].clone();
                let mode = parent.mode;
                let sub_snd = sound_from_decoded_sample(&sample, mode);
                let new_id = next_handle();
                m.sounds.insert(new_id, sub_snd);
                if let Some(p_mut) = m.sounds.get_mut(&sound) {
                    p_mut.subsound_handles[idx] = Some(new_id);
                }
                (new_id, true)
            }
        } else {
            let sample = parent.fsb_subsounds[idx].clone();
            let mode = parent.mode;
            let sub_snd = sound_from_decoded_sample(&sample, mode);
            let new_id = next_handle();
            m.sounds.insert(new_id, sub_snd);
            if let Some(p_mut) = m.sounds.get_mut(&sound) {
                p_mut.subsound_handles[idx] = Some(new_id);
            }
            (new_id, true)
        }
    };
    if newly_created {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        reg.sounds.insert(sub_id, sys);
    }
    write_out(subsound_out, sub_id);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Sound_GetName(
    sound: usize,
    name_out: *mut u8,
    namelen: i32,
) -> i32 {
    if name_out.is_null() || namelen <= 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let bytes = s.name.as_bytes();
    let copy_len = bytes.len().min((namelen as usize) - 1);
    if copy_len > 0 {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), name_out, copy_len);
    }
    std::ptr::write_unaligned(name_out.add(copy_len), 0u8);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Sound_GetLength(
    sound: usize,
    length_out: *mut u32,
    lengthtype: u32,
) -> i32 {
    if length_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let val = match lengthtype {
        FMOD_TIMEUNIT_MS => {
            ((s.num_frames as u64).saturating_mul(1000) / (s.sample_rate.max(1) as u64)) as u32
        }
        FMOD_TIMEUNIT_PCM => s.num_frames as u32,
        FMOD_TIMEUNIT_PCMBYTES => {
            let bps = (s.bits_per_sample as usize) / 8;
            (s.num_frames * (s.channels as usize) * bps) as u32
        }
        FMOD_TIMEUNIT_RAWBYTES => s.raw_pcm.len() as u32,
        _ => return FMOD_ERR_INVALID_PARAM,
    };
    write_out(length_out, val);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Sound_GetFormat(
    sound: usize,
    sound_type_out: *mut u32,
    format_out: *mut u32,
    channels_out: *mut i32,
    bits_out: *mut i32,
) -> i32 {
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(sound_type_out, s.sound_type);
    write_out(format_out, s.format);
    write_out(channels_out, s.channels as i32);
    write_out(bits_out, s.bits_per_sample as i32);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Sound_GetDefaults(
    sound: usize,
    frequency_out: *mut f32,
    priority_out: *mut i32,
) -> i32 {
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(frequency_out, s.default_frequency);
    write_out(priority_out, s.default_priority);
    FMOD_OK
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_Sound_SetDefaults(
    sound: usize,
    frequency: f32,
    priority: i32,
) -> i32 {
    sound_set_defaults_impl(sound, frequency, priority)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_Sound_SetDefaults(
    sound: usize,
    _rdx: u64,
    priority: i32,
    _xmm0: f32,
    frequency: f32,
) -> i32 {
    sound_set_defaults_impl(sound, frequency, priority)
}

pub unsafe extern "win64" fn FMOD_Sound_SetMode(sound: usize, mode: u32) -> i32 {
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get_mut(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    s.mode = mode;
    if (mode & FMOD_LOOP_OFF) != 0 {
        s.loop_count = 0;
    } else if (mode & (FMOD_LOOP_NORMAL | FMOD_LOOP_BIDI)) != 0 && s.loop_count == 0 {
        s.loop_count = -1;
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Sound_GetMode(sound: usize, mode_out: *mut u32) -> i32 {
    if mode_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(mode_out, s.mode);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Sound_SetLoopCount(sound: usize, loopcount: i32) -> i32 {
    if loopcount < -1 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get_mut(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    s.loop_count = loopcount;
    if loopcount == 0 {
        s.mode = (s.mode & !(FMOD_LOOP_NORMAL | FMOD_LOOP_BIDI)) | FMOD_LOOP_OFF;
    } else if (s.mode & FMOD_LOOP_BIDI) == 0 {
        s.mode = (s.mode & !FMOD_LOOP_OFF) | FMOD_LOOP_NORMAL;
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Sound_GetLoopCount(
    sound: usize,
    loopcount_out: *mut i32,
) -> i32 {
    if loopcount_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(loopcount_out, s.loop_count);
    FMOD_OK
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_Sound_Set3DMinMaxDistance(
    sound: usize,
    min: f32,
    max: f32,
) -> i32 {
    sound_set_3d_min_max_distance_impl(sound, min, max)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_Sound_Set3DMinMaxDistance(
    sound: usize,
    _xmm0: f32,
    min: f32,
    max: f32,
) -> i32 {
    sound_set_3d_min_max_distance_impl(sound, min, max)
}

pub unsafe extern "win64" fn FMOD_Sound_Get3DMinMaxDistance(
    sound: usize,
    min_out: *mut f32,
    max_out: *mut f32,
) -> i32 {
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(min_out, s.min_distance);
    write_out(max_out, s.max_distance);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Sound_Lock(
    sound: usize,
    offset: u32,
    length: u32,
    ptr1: *mut *mut u8,
    ptr2: *mut *mut u8,
    len1: *mut u32,
    len2: *mut u32,
) -> i32 {
    if ptr1.is_null() || len1.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get_mut(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    if s.locked {
        return FMOD_ERR_ALREADY_LOCKED;
    }
    let total = s.raw_pcm.len();
    let off = offset as usize;
    let req = length as usize;
    if total == 0 || off >= total || req == 0 || req > total {
        return FMOD_ERR_INVALID_PARAM;
    }

    let first = req.min(total - off);
    let second = req - first;
    let base = s.raw_pcm.as_mut_ptr();
    s.locked = true;

    write_out(ptr1, base.add(off));
    write_out(len1, first as u32);
    if second > 0 {
        write_out(ptr2, base);
        write_out(len2, second as u32);
    } else {
        write_out(ptr2, std::ptr::null_mut());
        write_out(len2, 0);
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Sound_Unlock(
    sound: usize,
    _ptr1: *mut u8,
    _ptr2: *mut u8,
    len1: u32,
    len2: u32,
) -> i32 {
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let (new_samples, new_frames, new_loop_start, new_loop_end) = {
        let Some(s) = m.sounds.get_mut(&sound) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        if !s.locked {
            return FMOD_ERR_NOT_LOCKED;
        }
        if (len1 as usize).saturating_add(len2 as usize) > s.raw_pcm.len() {
            return FMOD_ERR_INVALID_PARAM;
        }
        s.locked = false;
        let Some(decoded) = decode_pcm_to_f32(&s.raw_pcm, s.format) else {
            return FMOD_ERR_FORMAT;
        };
        let nframes = decoded.len() / (s.channels as usize).max(1);
        let arc = Arc::new(decoded);
        s.samples = arc.clone();
        if s.loop_end == s.num_frames {
            s.loop_end = nframes;
        } else {
            s.loop_end = s.loop_end.min(nframes);
            s.loop_start = s.loop_start.min(s.loop_end);
        }
        s.num_frames = nframes;
        (arc, nframes, s.loop_start, s.loop_end)
    };

    for ch in m.channels.values_mut() {
        if ch.sound_id == sound {
            ch.samples = new_samples.clone();
            ch.num_frames = new_frames;
            ch.loop_start = new_loop_start;
            ch.loop_end = new_loop_end;
        }
    }
    FMOD_OK
}

// ============================================================================
// FMOD Core — Playback & Channel methods
// ============================================================================

pub unsafe extern "win64" fn FMOD_System_PlaySound(
    system: usize,
    sound: usize,
    channelgroup: usize,
    paused: i32,
    channel_out: *mut usize,
) -> i32 {
    write_out(channel_out, 0);
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };

    let channel_id = next_handle();
    {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        if !m.initialized {
            return FMOD_ERR_UNINITIALIZED;
        }
        let target_group = if channelgroup == 0 {
            m.master_group_id
        } else if m.groups.contains_key(&channelgroup) {
            channelgroup
        } else {
            return FMOD_ERR_INVALID_HANDLE;
        };

        let active_count = m.channels.values().filter(|c| !c.stopped).count();
        if active_count >= m.max_channels {
            return FMOD_ERR_TOOMANYCHANNELS;
        }

        let Some(s) = m.sounds.get(&sound) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        let frame_bytes = (s.channels as usize) * ((s.bits_per_sample as usize) / 8).max(1);
        let ch = Channel {
            sound_id: sound,
            group_id: target_group,
            samples: s.samples.clone(),
            sound_channels: s.channels as usize,
            sound_sample_rate: s.sample_rate,
            sound_frame_bytes: frame_bytes,
            num_frames: s.num_frames,
            loop_start: s.loop_start,
            loop_end: s.loop_end,
            play_pos: 0.0,
            frequency: s.default_frequency,
            pitch: 1.0,
            volume: 1.0,
            pan: 0.0,
            mute: false,
            paused: paused != 0,
            stopped: false,
            mode: s.mode,
            loop_count: s.loop_count,
            loops_remaining: s.loop_count,
            pos_3d: FmodVector::default(),
            vel_3d: FmodVector::default(),
            min_distance: s.min_distance,
            max_distance: s.max_distance,
        };
        m.channels.insert(channel_id, ch);
    }
    {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        reg.channels.insert(channel_id, sys);
    }
    write_out(channel_out, channel_id);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Channel_SetPaused(channel: usize, paused: i32) -> i32 {
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.paused = paused != 0;
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Channel_GetPaused(
    channel: usize,
    paused_out: *mut i32,
) -> i32 {
    if paused_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(paused_out, if ch.paused { 1 } else { 0 });
    FMOD_OK
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_Channel_SetVolume(channel: usize, volume: f32) -> i32 {
    channel_set_volume_impl(channel, volume)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_Channel_SetVolume(
    channel: usize,
    _xmm0: f32,
    volume: f32,
) -> i32 {
    channel_set_volume_impl(channel, volume)
}

pub unsafe extern "win64" fn FMOD_Channel_GetVolume(
    channel: usize,
    volume_out: *mut f32,
) -> i32 {
    if volume_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(volume_out, ch.volume);
    FMOD_OK
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_Channel_SetPitch(channel: usize, pitch: f32) -> i32 {
    channel_set_pitch_impl(channel, pitch)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_Channel_SetPitch(
    channel: usize,
    _xmm0: f32,
    pitch: f32,
) -> i32 {
    channel_set_pitch_impl(channel, pitch)
}

pub unsafe extern "win64" fn FMOD_Channel_GetPitch(
    channel: usize,
    pitch_out: *mut f32,
) -> i32 {
    if pitch_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(pitch_out, ch.pitch);
    FMOD_OK
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_Channel_SetPan(channel: usize, pan: f32) -> i32 {
    channel_set_pan_impl(channel, pan)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_Channel_SetPan(channel: usize, _xmm0: f32, pan: f32) -> i32 {
    channel_set_pan_impl(channel, pan)
}

pub unsafe extern "win64" fn FMOD_Channel_SetMute(channel: usize, mute: i32) -> i32 {
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.mute = mute != 0;
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Channel_GetMute(channel: usize, mute_out: *mut i32) -> i32 {
    if mute_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(mute_out, if ch.mute { 1 } else { 0 });
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Channel_SetMode(channel: usize, mode: u32) -> i32 {
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.mode = mode;
    if (mode & FMOD_LOOP_OFF) != 0 {
        ch.loop_count = 0;
        ch.loops_remaining = 0;
    } else if (mode & (FMOD_LOOP_NORMAL | FMOD_LOOP_BIDI)) != 0 && ch.loops_remaining == 0 {
        ch.loop_count = -1;
        ch.loops_remaining = -1;
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Channel_GetMode(channel: usize, mode_out: *mut u32) -> i32 {
    if mode_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(mode_out, ch.mode);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Channel_Set3DAttributes(
    channel: usize,
    pos: *const FmodVector,
    vel: *const FmodVector,
) -> i32 {
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let p_opt = read_in(pos);
    let v_opt = read_in(vel);
    for vec in [p_opt, v_opt].into_iter().flatten() {
        if !vec.x.is_finite() || !vec.y.is_finite() || !vec.z.is_finite() {
            return FMOD_ERR_INVALID_FLOAT;
        }
    }
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    if let Some(p) = p_opt {
        ch.pos_3d = p;
    }
    if let Some(v) = v_opt {
        ch.vel_3d = v;
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Channel_Get3DAttributes(
    channel: usize,
    pos_out: *mut FmodVector,
    vel_out: *mut FmodVector,
) -> i32 {
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(pos_out, ch.pos_3d);
    write_out(vel_out, ch.vel_3d);
    FMOD_OK
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_Channel_Set3DMinMaxDistance(
    channel: usize,
    mindistance: f32,
    maxdistance: f32,
) -> i32 {
    channel_set_3d_min_max_distance_impl(channel, mindistance, maxdistance)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_Channel_Set3DMinMaxDistance(
    channel: usize,
    _xmm0: f32,
    mindistance: f32,
    maxdistance: f32,
) -> i32 {
    channel_set_3d_min_max_distance_impl(channel, mindistance, maxdistance)
}

pub unsafe extern "win64" fn FMOD_Channel_Get3DMinMaxDistance(
    channel: usize,
    mindistance_out: *mut f32,
    maxdistance_out: *mut f32,
) -> i32 {
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(mindistance_out, ch.min_distance);
    write_out(maxdistance_out, ch.max_distance);
    FMOD_OK
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_Channel_SetFrequency(channel: usize, frequency: f32) -> i32 {
    channel_set_frequency_impl(channel, frequency)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_Channel_SetFrequency(
    channel: usize,
    _xmm0: f32,
    frequency: f32,
) -> i32 {
    channel_set_frequency_impl(channel, frequency)
}

pub unsafe extern "win64" fn FMOD_Channel_GetFrequency(
    channel: usize,
    frequency_out: *mut f32,
) -> i32 {
    if frequency_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(frequency_out, ch.frequency);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Channel_SetLoopCount(channel: usize, loopcount: i32) -> i32 {
    if loopcount < -1 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.loop_count = loopcount;
    ch.loops_remaining = loopcount;
    if loopcount == 0 {
        ch.mode = (ch.mode & !(FMOD_LOOP_NORMAL | FMOD_LOOP_BIDI)) | FMOD_LOOP_OFF;
    } else if (ch.mode & FMOD_LOOP_BIDI) == 0 {
        ch.mode = (ch.mode & !FMOD_LOOP_OFF) | FMOD_LOOP_NORMAL;
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Channel_GetLoopCount(
    channel: usize,
    loopcount_out: *mut i32,
) -> i32 {
    if loopcount_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(loopcount_out, ch.loop_count);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Channel_SetPosition(
    channel: usize,
    position: u32,
    postype: u32,
) -> i32 {
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let frame = match postype {
        FMOD_TIMEUNIT_PCM => position as usize,
        FMOD_TIMEUNIT_MS => {
            ((position as u64).saturating_mul(ch.sound_sample_rate as u64) / 1000) as usize
        }
        FMOD_TIMEUNIT_PCMBYTES => (position as usize) / ch.sound_frame_bytes.max(1),
        _ => return FMOD_ERR_INVALID_PARAM,
    };
    if frame > ch.num_frames {
        return FMOD_ERR_INVALID_POSITION;
    }
    ch.play_pos = frame as f64;
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Channel_GetPosition(
    channel: usize,
    position_out: *mut u32,
    postype: u32,
) -> i32 {
    if position_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let cur_frame = (ch.play_pos.max(0.0).floor() as usize).min(ch.num_frames);
    let val = match postype {
        FMOD_TIMEUNIT_PCM => cur_frame as u32,
        FMOD_TIMEUNIT_MS => {
            ((cur_frame as u64).saturating_mul(1000) / (ch.sound_sample_rate.max(1) as u64)) as u32
        }
        FMOD_TIMEUNIT_PCMBYTES => (cur_frame * ch.sound_frame_bytes) as u32,
        _ => return FMOD_ERR_INVALID_PARAM,
    };
    write_out(position_out, val);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Channel_IsPlaying(
    channel: usize,
    isplaying_out: *mut i32,
) -> i32 {
    if isplaying_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => {
            write_out(isplaying_out, 0);
            return e;
        }
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        write_out(isplaying_out, 0);
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(isplaying_out, if ch.stopped { 0 } else { 1 });
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Channel_Stop(channel: usize) -> i32 {
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.stopped = true;
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Channel_SetChannelGroup(
    channel: usize,
    channelgroup: usize,
) -> i32 {
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let target_group = if channelgroup == 0 {
        m.master_group_id
    } else if m.groups.contains_key(&channelgroup) {
        channelgroup
    } else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.group_id = target_group;
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Channel_GetChannelGroup(
    channel: usize,
    channelgroup_out: *mut usize,
) -> i32 {
    if channelgroup_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(channelgroup_out, ch.group_id);
    FMOD_OK
}

// ============================================================================
// FMOD Core — ChannelGroup methods
// ============================================================================

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_ChannelGroup_SetVolume(
    channelgroup: usize,
    volume: f32,
) -> i32 {
    channel_group_set_volume_impl(channelgroup, volume)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_ChannelGroup_SetVolume(
    channelgroup: usize,
    _xmm0: f32,
    volume: f32,
) -> i32 {
    channel_group_set_volume_impl(channelgroup, volume)
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_GetVolume(
    channelgroup: usize,
    volume_out: *mut f32,
) -> i32 {
    if volume_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(volume_out, g.volume);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_SetMute(channelgroup: usize, mute: i32) -> i32 {
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get_mut(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    g.mute = mute != 0;
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_GetMute(
    channelgroup: usize,
    mute_out: *mut i32,
) -> i32 {
    if mute_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(mute_out, if g.mute { 1 } else { 0 });
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_SetPaused(
    channelgroup: usize,
    paused: i32,
) -> i32 {
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get_mut(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    g.paused = paused != 0;
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_GetPaused(
    channelgroup: usize,
    paused_out: *mut i32,
) -> i32 {
    if paused_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(paused_out, if g.paused { 1 } else { 0 });
    FMOD_OK
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_ChannelGroup_SetPitch(channelgroup: usize, pitch: f32) -> i32 {
    channel_group_set_pitch_impl(channelgroup, pitch)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_ChannelGroup_SetPitch(
    channelgroup: usize,
    _xmm0: f32,
    pitch: f32,
) -> i32 {
    channel_group_set_pitch_impl(channelgroup, pitch)
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_GetPitch(
    channelgroup: usize,
    pitch_out: *mut f32,
) -> i32 {
    if pitch_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(pitch_out, g.pitch);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_Stop(channelgroup: usize) -> i32 {
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    if !m.groups.contains_key(&channelgroup) {
        return FMOD_ERR_INVALID_HANDLE;
    }
    let affected: Vec<usize> = m
        .groups
        .keys()
        .copied()
        .filter(|&gid| group_has_ancestor(&m.groups, gid, channelgroup))
        .collect();
    for ch in m.channels.values_mut() {
        if affected.contains(&ch.group_id) {
            ch.stopped = true;
        }
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_AddGroup(
    channelgroup: usize,
    group: usize,
    _propagatedspclock: i32,
    connection: *mut usize,
) -> i32 {
    write_out(connection, 0);
    let parent_sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let child_sys = match lookup_group_system(group) {
        Ok(s) => s,
        Err(e) => return e,
    };
    if !Arc::ptr_eq(&parent_sys, &child_sys) {
        return FMOD_ERR_INVALID_PARAM;
    }
    let mut m = parent_sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    if channelgroup == group || group == m.master_group_id {
        return FMOD_ERR_INVALID_PARAM;
    }
    if !m.groups.contains_key(&channelgroup) || !m.groups.contains_key(&group) {
        return FMOD_ERR_INVALID_HANDLE;
    }
    if group_has_ancestor(&m.groups, channelgroup, group) {
        return FMOD_ERR_INVALID_PARAM;
    }
    if let Some(child) = m.groups.get_mut(&group) {
        child.parent = Some(channelgroup);
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_Release(channelgroup: usize) -> i32 {
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        if channelgroup == m.master_group_id {
            return FMOD_ERR_INVALID_PARAM;
        }
        if m.groups.remove(&channelgroup).is_none() {
            return FMOD_ERR_INVALID_HANDLE;
        }
        let master_id = m.master_group_id;
        for g in m.groups.values_mut() {
            if g.parent == Some(channelgroup) {
                g.parent = Some(master_id);
            }
        }
        for ch in m.channels.values_mut() {
            if ch.group_id == channelgroup {
                ch.group_id = master_id;
            }
        }
    }
    {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        reg.groups.remove(&channelgroup);
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_GetParentGroup(
    channelgroup: usize,
    group_out: *mut usize,
) -> i32 {
    if group_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(group_out, g.parent.unwrap_or(0));
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_GetName(
    channelgroup: usize,
    name_out: *mut u8,
    namelen: i32,
) -> i32 {
    if name_out.is_null() || namelen <= 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let bytes = g.name.as_bytes();
    let copy_len = bytes.len().min((namelen as usize) - 1);
    if copy_len > 0 {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), name_out, copy_len);
    }
    std::ptr::write_unaligned(name_out.add(copy_len), 0u8);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_IsPlaying(
    channelgroup: usize,
    isplaying_out: *mut i32,
) -> i32 {
    if isplaying_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => {
            write_out(isplaying_out, 0);
            return e;
        }
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    if !m.groups.contains_key(&channelgroup) {
        write_out(isplaying_out, 0);
        return FMOD_ERR_INVALID_HANDLE;
    }
    let playing = m
        .channels
        .values()
        .any(|ch| !ch.stopped && group_has_ancestor(&m.groups, ch.group_id, channelgroup));
    write_out(isplaying_out, if playing { 1 } else { 0 });
    FMOD_OK
}

// ============================================================================
// FMOD Studio (`fmodstudio.dll` / `fmodstudio64.dll`) — Runtime state
// ============================================================================

#[derive(Clone, Copy)]
struct StudioListener {
    attributes: Fmod3DAttributes,
    attenuation_pos: Option<FmodVector>,
    weight: f32,
}

impl Default for StudioListener {
    fn default() -> Self {
        Self {
            attributes: Fmod3DAttributes::default(),
            attenuation_pos: None,
            weight: 1.0,
        }
    }
}

struct StudioBank {
    guid: FmodGuid,
    has_explicit_guid: bool,
    path: String,
    loading_state: u32,
    sample_loading_state: u32,
    strings: Vec<(FmodGuid, String)>,
    event_handles: Vec<usize>,
    bus_handles: Vec<usize>,
    vca_handles: Vec<usize>,
}

struct StudioBus {
    guid: FmodGuid,
    path: String,
    parent_guid: Option<FmodGuid>,
    channel_group_id: usize,
    volume: f32,
    mute: bool,
    paused: bool,
    locked_group: bool,
    is_master: bool,
    ref_banks: Vec<usize>,
}

struct StudioVca {
    guid: FmodGuid,
    path: String,
    volume: f32,
    target_bus_guids: Vec<FmodGuid>,
    target_event_guids: Vec<FmodGuid>,
    ref_banks: Vec<usize>,
}

struct StudioEventDesc {
    bank_handle: usize,
    guid: FmodGuid,
    path: String,
    bus_guid: Option<FmodGuid>,
    sample: Option<DecodedSample>,
    is_3d: bool,
    is_oneshot: bool,
    is_stream: bool,
    is_snapshot: bool,
    has_sustain_point: bool,
    default_volume: f32,
    default_pitch: f32,
    min_distance: f32,
    max_distance: f32,
    sound_size: f32,
    length_ms: i32,
    parameters: Vec<StudioParameterDef>,
    sample_loading_state: u32,
    instances: Vec<usize>,
}

struct StudioEventInstance {
    desc_handle: usize,
    event_guid: FmodGuid,
    bank_handle: usize,
    channel_group_id: usize,
    channel_id: Option<usize>,
    volume: f32,
    pitch: f32,
    paused: bool,
    state: u32,
    attributes_3d: Fmod3DAttributes,
    min_distance: f32,
    max_distance: f32,
    timeline_ms: i32,
    reverb_levels: [f32; 4],
    properties: [f32; 6],
    param_defs: HashMap<String, StudioParameterDef>,
    param_id_to_key: HashMap<FmodStudioParameterId, String>,
    param_values: HashMap<String, f32>,
    sample: Option<DecodedSample>,
    is_3d: bool,
    is_oneshot: bool,
    desc_default_volume: f32,
    desc_default_pitch: f32,
}

struct OrphanInstance {
    bank_handle: usize,
    channel_group_id: usize,
    channel_id: usize,
}

struct StudioState {
    master_bus_handle: usize,
    master_bus_guid: FmodGuid,
    num_listeners: usize,
    listeners: [StudioListener; 8],
    banks: HashMap<usize, StudioBank>,
    bank_order: Vec<usize>,
    bank_by_guid: HashMap<FmodGuid, usize>,
    events: HashMap<usize, StudioEventDesc>,
    event_by_guid: HashMap<FmodGuid, usize>,
    instances: HashMap<usize, StudioEventInstance>,
    orphans: Vec<OrphanInstance>,
    buses: HashMap<usize, StudioBus>,
    bus_by_guid: HashMap<FmodGuid, usize>,
    vcas: HashMap<usize, StudioVca>,
    vca_by_guid: HashMap<FmodGuid, usize>,
    path_to_guid: HashMap<String, FmodGuid>,
    guid_to_path: HashMap<FmodGuid, String>,
    global_param_defs: HashMap<String, StudioParameterDef>,
    global_param_id_to_key: HashMap<FmodStudioParameterId, String>,
    global_param_values: HashMap<String, f32>,
}

struct StudioSystem {
    core_id: usize,
    core_sys: Arc<FmodSystem>,
    state: Mutex<StudioState>,
}

impl StudioState {
    fn new(master_bus_handle: usize, master_group_id: usize) -> Self {
        let master_guid = FmodGuid::from_le_bytes(guid_from_uid("bus:/"));
        let mut buses = HashMap::new();
        buses.insert(
            master_bus_handle,
            StudioBus {
                guid: master_guid,
                path: "bus:/".into(),
                parent_guid: None,
                channel_group_id: master_group_id,
                volume: 1.0,
                mute: false,
                paused: false,
                locked_group: false,
                is_master: true,
                ref_banks: Vec::new(),
            },
        );
        let mut bus_by_guid = HashMap::new();
        bus_by_guid.insert(master_guid, master_bus_handle);
        let mut path_to_guid = HashMap::new();
        path_to_guid.insert("bus:/".into(), master_guid);
        path_to_guid.insert(String::new(), master_guid);
        let mut guid_to_path = HashMap::new();
        guid_to_path.insert(master_guid, "bus:/".into());

        Self {
            master_bus_handle,
            master_bus_guid: master_guid,
            num_listeners: 1,
            listeners: [StudioListener::default(); 8],
            banks: HashMap::new(),
            bank_order: Vec::new(),
            bank_by_guid: HashMap::new(),
            events: HashMap::new(),
            event_by_guid: HashMap::new(),
            instances: HashMap::new(),
            orphans: Vec::new(),
            buses,
            bus_by_guid,
            vcas: HashMap::new(),
            vca_by_guid: HashMap::new(),
            path_to_guid,
            guid_to_path,
            global_param_defs: HashMap::new(),
            global_param_id_to_key: HashMap::new(),
            global_param_values: HashMap::new(),
        }
    }

    fn rebuild_string_maps(&mut self) {
        self.path_to_guid.clear();
        self.guid_to_path.clear();

        self.path_to_guid.insert("bus:/".into(), self.master_bus_guid);
        self.path_to_guid.insert(String::new(), self.master_bus_guid);
        self.guid_to_path.insert(self.master_bus_guid, "bus:/".into());

        for &bid in &self.bank_order {
            if let Some(bank) = self.banks.get(&bid) {
                for (g, p) in &bank.strings {
                    if !p.is_empty() {
                        self.path_to_guid.insert(normalize_studio_path(p), *g);
                        self.guid_to_path.insert(*g, p.clone());
                    }
                }
            }
        }
        for bus in self.buses.values() {
            if !bus.path.is_empty() {
                self.path_to_guid
                    .entry(normalize_studio_path(&bus.path))
                    .or_insert(bus.guid);
                self.guid_to_path
                    .entry(bus.guid)
                    .or_insert_with(|| bus.path.clone());
            }
        }
        for vca in self.vcas.values() {
            if !vca.path.is_empty() {
                self.path_to_guid
                    .entry(normalize_studio_path(&vca.path))
                    .or_insert(vca.guid);
                self.guid_to_path
                    .entry(vca.guid)
                    .or_insert_with(|| vca.path.clone());
            }
        }
        for ev in self.events.values() {
            if !ev.path.is_empty() {
                self.path_to_guid
                    .entry(normalize_studio_path(&ev.path))
                    .or_insert(ev.guid);
                self.guid_to_path
                    .entry(ev.guid)
                    .or_insert_with(|| ev.path.clone());
            }
        }
    }

    fn resolve_path_or_guid(&self, path_or_id: &str) -> Option<FmodGuid> {
        if let Some(g) = FmodGuid::parse_str(path_or_id) {
            return Some(g);
        }
        let norm = normalize_studio_path(path_or_id);
        self.path_to_guid.get(&norm).copied()
    }

    fn bus_vca_multiplier(&self, bus_guid: FmodGuid) -> f32 {
        let mut mult = 1.0f32;
        for vca in self.vcas.values() {
            if vca.target_bus_guids.contains(&bus_guid) {
                mult *= vca.volume;
            }
        }
        mult
    }

    fn event_vca_multiplier(&self, event_guid: FmodGuid) -> f32 {
        let mut mult = 1.0f32;
        for vca in self.vcas.values() {
            if vca.target_event_guids.contains(&event_guid) {
                mult *= vca.volume;
            }
        }
        mult
    }

    fn sync_bus_and_instance_gains(&self, mixer: &mut MixerState) {
        for bus in self.buses.values() {
            let vca_mult = self.bus_vca_multiplier(bus.guid);
            if let Some(cg) = mixer.groups.get_mut(&bus.channel_group_id) {
                cg.volume = bus.volume * vca_mult;
                cg.mute = bus.mute;
                cg.paused = bus.paused;
            }
        }
        for inst in self.instances.values() {
            let ev_vca = self.event_vca_multiplier(inst.event_guid);
            if let Some(cg) = mixer.groups.get_mut(&inst.channel_group_id) {
                cg.volume = inst.volume * inst.desc_default_volume * ev_vca;
                cg.pitch = (inst.pitch * inst.desc_default_pitch).max(0.0);
                cg.paused = inst.paused;
            }
        }
    }

    fn relink_bus_parents(&self, mixer: &mut MixerState) {
        let master_cg = mixer.master_group_id;
        for bus in self.buses.values() {
            if bus.is_master {
                continue;
            }
            let parent_from_guid = bus
                .parent_guid
                .and_then(|pg| self.bus_by_guid.get(&pg).copied())
                .and_then(|bh| self.buses.get(&bh))
                .map(|pb| pb.channel_group_id);
            let parent_from_path = if parent_from_guid.is_none() && !bus.path.is_empty() {
                let norm = normalize_studio_path(&bus.path);
                norm.rsplit_once('/').and_then(|(prefix, _)| {
                    if prefix == "bus:" || prefix.is_empty() {
                        Some(master_cg)
                    } else {
                        self.buses
                            .values()
                            .find(|b| normalize_studio_path(&b.path) == prefix)
                            .map(|b| b.channel_group_id)
                    }
                })
            } else {
                None
            };
            let parent_cg = parent_from_guid.or(parent_from_path).unwrap_or(master_cg);
            if parent_cg != bus.channel_group_id
                && !group_has_ancestor(&mixer.groups, parent_cg, bus.channel_group_id)
            {
                if let Some(cg) = mixer.groups.get_mut(&bus.channel_group_id) {
                    cg.parent = Some(parent_cg);
                }
            }
        }
    }

    fn resolve_event_bus_cg(&self, bus_guid: Option<FmodGuid>, master_cg: usize) -> usize {
        bus_guid
            .and_then(|bg| self.bus_by_guid.get(&bg).copied())
            .and_then(|bh| self.buses.get(&bh))
            .map(|b| b.channel_group_id)
            .unwrap_or(master_cg)
    }

    fn compute_listener_distance(&self, pos: FmodVector) -> f32 {
        let lp = self.listeners[0]
            .attenuation_pos
            .unwrap_or(self.listeners[0].attributes.position);
        let dx = pos.x - lp.x;
        let dy = pos.y - lp.y;
        let dz = pos.z - lp.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

fn load_parsed_bank_into_studio(
    studio_arc: &Arc<StudioSystem>,
    parsed: ParsedBank,
) -> Result<usize, i32> {
    let bank_handle = next_handle();
    let mut new_event_handles = Vec::new();
    let mut new_bus_handles = Vec::new();
    let mut new_vca_handles = Vec::new();
    let mut new_group_handles = Vec::new();

    {
        let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());

        if parsed.has_explicit_guid && st.bank_by_guid.contains_key(&parsed.guid) {
            return Err(FMOD_ERR_EVENT_ALREADY_LOADED);
        }

        let bank_guid = if parsed.has_explicit_guid {
            parsed.guid
        } else {
            FmodGuid::from_le_bytes(guid_from_uid(&format!("bank:{bank_handle:#x}:{}", parsed.path)))
        };

        // 1) Merge this bank's string entries first so bus/VCA/event paths can resolve cross-bank.
        for (g, p) in &parsed.strings {
            if !p.is_empty() {
                st.path_to_guid.insert(normalize_studio_path(p), *g);
                st.guid_to_path.insert(*g, p.clone());
            }
        }

        // Back-fill paths for previously loaded events/buses/VCAs whose strings just arrived.
        let g2p_snapshot = st.guid_to_path.clone();
        for ev in st.events.values_mut() {
            if ev.path.is_empty() {
                if let Some(p) = g2p_snapshot.get(&ev.guid) {
                    ev.path = p.clone();
                }
            }
        }
        for bus in st.buses.values_mut() {
            if bus.path.is_empty() {
                if let Some(p) = g2p_snapshot.get(&bus.guid) {
                    bus.path = p.clone();
                }
            }
        }
        for vca in st.vcas.values_mut() {
            if vca.path.is_empty() {
                if let Some(p) = g2p_snapshot.get(&vca.guid) {
                    vca.path = p.clone();
                }
            }
        }

        // 2) Register buses from BUSS chunks.
        let mut bank_bus_handles = Vec::new();
        for pb in parsed.buses {
            let resolved_path = if !pb.path.is_empty() {
                pb.path
            } else {
                st.guid_to_path.get(&pb.guid).cloned().unwrap_or_default()
            };
            let norm = normalize_studio_path(&resolved_path);
            let is_master_path = norm == "bus:/";

            if is_master_path {
                let mh = st.master_bus_handle;
                if !pb.guid.is_zero() {
                    st.bus_by_guid.insert(pb.guid, mh);
                }
                if let Some(mb) = st.buses.get_mut(&mh) {
                    mb.volume = pb.default_volume;
                    mb.mute = pb.mute;
                    mb.paused = pb.paused;
                    if !mb.ref_banks.contains(&bank_handle) {
                        mb.ref_banks.push(bank_handle);
                    }
                }
                bank_bus_handles.push(mh);
            } else if let Some(&existing_h) = st.bus_by_guid.get(&pb.guid) {
                if let Some(b) = st.buses.get_mut(&existing_h) {
                    if b.path.is_empty() && !resolved_path.is_empty() {
                        b.path = resolved_path;
                    }
                    if b.parent_guid.is_none() {
                        b.parent_guid = pb.parent_guid;
                    }
                    if !b.ref_banks.contains(&bank_handle) {
                        b.ref_banks.push(bank_handle);
                    }
                }
                bank_bus_handles.push(existing_h);
            } else {
                let bh = next_handle();
                let cg_id = next_handle();
                let master_cg = mixer.master_group_id;
                mixer.groups.insert(
                    cg_id,
                    ChannelGroup {
                        name: if resolved_path.is_empty() {
                            format!("bus:{bh:#x}")
                        } else {
                            resolved_path.clone()
                        },
                        parent: Some(master_cg),
                        volume: pb.default_volume,
                        mute: pb.mute,
                        paused: pb.paused,
                        pitch: 1.0,
                    },
                );
                new_group_handles.push(cg_id);
                st.buses.insert(
                    bh,
                    StudioBus {
                        guid: pb.guid,
                        path: resolved_path,
                        parent_guid: pb.parent_guid,
                        channel_group_id: cg_id,
                        volume: pb.default_volume,
                        mute: pb.mute,
                        paused: pb.paused,
                        locked_group: false,
                        is_master: false,
                        ref_banks: vec![bank_handle],
                    },
                );
                st.bus_by_guid.insert(pb.guid, bh);
                new_bus_handles.push(bh);
                bank_bus_handles.push(bh);
            }
        }

        st.relink_bus_parents(&mut mixer);

        // 3) Register VCAs from VCAS chunks.
        let mut bank_vca_handles = Vec::new();
        for pv in parsed.vcas {
            let resolved_path = if !pv.path.is_empty() {
                pv.path
            } else {
                st.guid_to_path.get(&pv.guid).cloned().unwrap_or_default()
            };
            if let Some(&existing_h) = st.vca_by_guid.get(&pv.guid) {
                if let Some(v) = st.vcas.get_mut(&existing_h) {
                    if v.path.is_empty() && !resolved_path.is_empty() {
                        v.path = resolved_path;
                    }
                    for bg in pv.target_bus_guids {
                        if !v.target_bus_guids.contains(&bg) {
                            v.target_bus_guids.push(bg);
                        }
                    }
                    for eg in pv.target_event_guids {
                        if !v.target_event_guids.contains(&eg) {
                            v.target_event_guids.push(eg);
                        }
                    }
                    if !v.ref_banks.contains(&bank_handle) {
                        v.ref_banks.push(bank_handle);
                    }
                }
                bank_vca_handles.push(existing_h);
            } else {
                let vh = next_handle();
                st.vcas.insert(
                    vh,
                    StudioVca {
                        guid: pv.guid,
                        path: resolved_path,
                        volume: pv.default_volume,
                        target_bus_guids: pv.target_bus_guids,
                        target_event_guids: pv.target_event_guids,
                        ref_banks: vec![bank_handle],
                    },
                );
                st.vca_by_guid.insert(pv.guid, vh);
                new_vca_handles.push(vh);
                bank_vca_handles.push(vh);
            }
        }

        // 4) Register global parameters.
        for gp in parsed.global_params {
            let key = gp.name.to_ascii_lowercase();
            let def_val = gp.default_value;
            st.global_param_id_to_key.insert(gp.id, key.clone());
            st.global_param_values.entry(key.clone()).or_insert(def_val);
            st.global_param_defs.insert(key, gp);
        }

        // 5) Register events from EVNT chunks (or synthesized FSB5 events).
        let mut bank_event_handles = Vec::new();
        for pe in parsed.events {
            let resolved_path = if !pe.path.is_empty() {
                pe.path
            } else {
                st.guid_to_path.get(&pe.guid).cloned().unwrap_or_default()
            };
            let sample = pe
                .sample_index
                .and_then(|idx| parsed.samples.get(idx))
                .cloned();
            let length_ms = if pe.length_ms > 0 {
                pe.length_ms
            } else if let Some(ref s) = sample {
                ((s.num_frames as u64).saturating_mul(1000) / (s.sample_rate.max(1) as u64)) as i32
            } else {
                0
            };
            let eh = next_handle();
            st.events.insert(
                eh,
                StudioEventDesc {
                    bank_handle,
                    guid: pe.guid,
                    path: resolved_path,
                    bus_guid: pe.bus_guid,
                    sample,
                    is_3d: pe.is_3d,
                    is_oneshot: pe.is_oneshot,
                    is_stream: pe.is_stream,
                    is_snapshot: pe.is_snapshot,
                    has_sustain_point: false,
                    default_volume: pe.default_volume,
                    default_pitch: pe.default_pitch,
                    min_distance: pe.min_distance,
                    max_distance: pe.max_distance,
                    sound_size: 0.0,
                    length_ms,
                    parameters: pe.parameters,
                    sample_loading_state: FMOD_STUDIO_LOADING_STATE_LOADED,
                    instances: Vec::new(),
                },
            );
            st.event_by_guid.insert(pe.guid, eh);
            new_event_handles.push(eh);
            bank_event_handles.push(eh);
        }

        // 6) Record the bank itself.
        let bank_path = if !parsed.path.is_empty() {
            parsed.path
        } else {
            st.guid_to_path.get(&bank_guid).cloned().unwrap_or_default()
        };
        st.banks.insert(
            bank_handle,
            StudioBank {
                guid: bank_guid,
                has_explicit_guid: parsed.has_explicit_guid,
                path: bank_path,
                loading_state: FMOD_STUDIO_LOADING_STATE_LOADED,
                sample_loading_state: FMOD_STUDIO_LOADING_STATE_LOADED,
                strings: parsed.strings,
                event_handles: bank_event_handles,
                bus_handles: bank_bus_handles,
                vca_handles: bank_vca_handles,
            },
        );
        st.bank_order.push(bank_handle);
        st.bank_by_guid.insert(bank_guid, bank_handle);
        st.rebuild_string_maps();
        st.sync_bus_and_instance_gains(&mut mixer);
    }

    // Publish all new handles in global registry without holding studio/mixer locks.
    {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        reg.studio_banks.insert(bank_handle, studio_arc.clone());
        for eh in new_event_handles {
            reg.studio_events.insert(eh, studio_arc.clone());
        }
        for bh in new_bus_handles {
            reg.studio_buses.insert(bh, studio_arc.clone());
        }
        for vh in new_vca_handles {
            reg.studio_vcas.insert(vh, studio_arc.clone());
        }
        for cg in new_group_handles {
            reg.groups.insert(cg, studio_arc.core_sys.clone());
        }
    }

    Ok(bank_handle)
}

fn unload_bank_impl(studio_arc: &Arc<StudioSystem>, bank_handle: usize) -> i32 {
    let mut rm_events = Vec::new();
    let mut rm_instances = Vec::new();
    let mut rm_buses = Vec::new();
    let mut rm_vcas = Vec::new();
    let mut rm_groups = Vec::new();
    let mut rm_channels = Vec::new();

    {
        let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let Some(bank) = st.banks.remove(&bank_handle) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());

        st.bank_order.retain(|&h| h != bank_handle);
        if bank.has_explicit_guid || st.bank_by_guid.get(&bank.guid) == Some(&bank_handle) {
            st.bank_by_guid.remove(&bank.guid);
        }

        // Stop & remove all live instances owned by this bank.
        let inst_ids: Vec<usize> = st
            .instances
            .iter()
            .filter(|(_, inst)| inst.bank_handle == bank_handle)
            .map(|(&id, _)| id)
            .collect();
        for id in inst_ids {
            if let Some(inst) = st.instances.remove(&id) {
                rm_instances.push(id);
                if let Some(cid) = inst.channel_id {
                    if let Some(ch) = mixer.channels.remove(&cid) {
                        let _ = ch;
                    }
                    rm_channels.push(cid);
                }
                mixer.groups.remove(&inst.channel_group_id);
                rm_groups.push(inst.channel_group_id);
            }
        }

        // Stop & remove any fire-and-forget orphan instances owned by this bank.
        let mut kept_orphans = Vec::new();
        for orph in st.orphans.drain(..) {
            if orph.bank_handle == bank_handle {
                mixer.channels.remove(&orph.channel_id);
                mixer.groups.remove(&orph.channel_group_id);
                rm_channels.push(orph.channel_id);
                rm_groups.push(orph.channel_group_id);
            } else {
                kept_orphans.push(orph);
            }
        }
        st.orphans = kept_orphans;

        // Remove all EventDescriptions owned by this bank.
        for eh in bank.event_handles {
            if let Some(ev) = st.events.remove(&eh) {
                if st.event_by_guid.get(&ev.guid) == Some(&eh) {
                    st.event_by_guid.remove(&ev.guid);
                }
                rm_events.push(eh);
            }
        }

        // Decrement bank refs on non-master buses and remove unreferenced ones.
        let master_cg = mixer.master_group_id;
        for bh in bank.bus_handles {
            let remove_bus = if let Some(b) = st.buses.get_mut(&bh) {
                b.ref_banks.retain(|&id| id != bank_handle);
                !b.is_master && b.ref_banks.is_empty()
            } else {
                false
            };
            if remove_bus {
                if let Some(b) = st.buses.remove(&bh) {
                    st.bus_by_guid.retain(|_, v| *v != bh);
                    mixer.groups.remove(&b.channel_group_id);
                    for g in mixer.groups.values_mut() {
                        if g.parent == Some(b.channel_group_id) {
                            g.parent = Some(master_cg);
                        }
                    }
                    rm_buses.push(bh);
                    rm_groups.push(b.channel_group_id);
                }
            }
        }

        // Decrement bank refs on VCAs and remove unreferenced ones.
        for vh in bank.vca_handles {
            let remove_vca = if let Some(v) = st.vcas.get_mut(&vh) {
                v.ref_banks.retain(|&id| id != bank_handle);
                v.ref_banks.is_empty()
            } else {
                false
            };
            if remove_vca {
                if let Some(v) = st.vcas.remove(&vh) {
                    st.vca_by_guid.retain(|_, val| *val != vh);
                    let _ = v;
                    rm_vcas.push(vh);
                }
            }
        }

        st.rebuild_string_maps();
        st.relink_bus_parents(&mut mixer);
        st.sync_bus_and_instance_gains(&mut mixer);
    }

    {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        reg.studio_banks.remove(&bank_handle);
        for id in rm_events {
            reg.studio_events.remove(&id);
        }
        for id in rm_instances {
            reg.studio_instances.remove(&id);
        }
        for id in rm_buses {
            reg.studio_buses.remove(&id);
        }
        for id in rm_vcas {
            reg.studio_vcas.remove(&id);
        }
        for id in rm_groups {
            reg.groups.remove(&id);
        }
        for id in rm_channels {
            reg.channels.remove(&id);
        }
    }
    FMOD_OK
}

fn studio_system_set_listener_weight_impl(system: usize, listener: i32, weight: f32) -> i32 {
    if !weight.is_finite() || weight < 0.0 {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if listener < 0 || (listener as usize) >= st.num_listeners {
        return FMOD_ERR_INVALID_PARAM;
    }
    st.listeners[listener as usize].weight = weight;
    FMOD_OK
}

fn studio_system_set_param_by_name_impl(
    system: usize,
    name: *const u8,
    value: f32,
    _ignoreseekspeed: i32,
) -> i32 {
    if name.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    if !value.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let key = unsafe { crate::strings::from_ansi(name) }.to_ascii_lowercase();
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(def) = st.global_param_defs.get(&key) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    let clamped = if def.maximum > def.minimum {
        value.clamp(def.minimum, def.maximum)
    } else {
        value
    };
    st.global_param_values.insert(key, clamped);
    FMOD_OK
}

fn studio_system_set_param_by_id_impl(
    system: usize,
    id: u64,
    value: f32,
    _ignoreseekspeed: i32,
) -> i32 {
    if !value.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let pid = FmodStudioParameterId::from_u64(id);
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(key) = st.global_param_id_to_key.get(&pid).cloned() else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    let Some(def) = st.global_param_defs.get(&key) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    let clamped = if def.maximum > def.minimum {
        value.clamp(def.minimum, def.maximum)
    } else {
        value
    };
    st.global_param_values.insert(key, clamped);
    FMOD_OK
}

fn studio_event_instance_set_volume_impl(instance: usize, volume: f32) -> i32 {
    if !volume.is_finite() || volume < 0.0 {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let (cg_id, ev_guid, desc_vol) = {
        let Some(inst) = st.instances.get_mut(&instance) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        inst.volume = volume;
        (inst.channel_group_id, inst.event_guid, inst.desc_default_volume)
    };
    let ev_vca = st.event_vca_multiplier(ev_guid);
    let mut mixer = studio_arc
        .core_sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(cg) = mixer.groups.get_mut(&cg_id) {
        cg.volume = volume * desc_vol * ev_vca;
    }
    FMOD_OK
}

fn studio_event_instance_set_pitch_impl(instance: usize, pitch: f32) -> i32 {
    if !pitch.is_finite() || pitch < 0.0 {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let (cg_id, desc_pitch) = {
        let Some(inst) = st.instances.get_mut(&instance) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        inst.pitch = pitch;
        (inst.channel_group_id, inst.desc_default_pitch)
    };
    let mut mixer = studio_arc
        .core_sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(cg) = mixer.groups.get_mut(&cg_id) {
        cg.pitch = (pitch * desc_pitch).max(0.0);
    }
    FMOD_OK
}

fn studio_event_instance_set_property_impl(instance: usize, index: u32, value: f32) -> i32 {
    if index >= FMOD_STUDIO_EVENT_PROPERTY_MAX {
        return FMOD_ERR_INVALID_PARAM;
    }
    if !value.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get_mut(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    inst.properties[index as usize] = value;
    let mut update_dist = false;
    if index == FMOD_STUDIO_EVENT_PROPERTY_MINIMUM_DISTANCE && value >= 0.0 {
        inst.min_distance = value;
        if inst.max_distance < inst.min_distance {
            inst.max_distance = inst.min_distance;
        }
        update_dist = true;
    } else if index == FMOD_STUDIO_EVENT_PROPERTY_MAXIMUM_DISTANCE && value >= 0.0 {
        inst.max_distance = value.max(inst.min_distance);
        update_dist = true;
    }
    if update_dist {
        if let Some(cid) = inst.channel_id {
            let min_d = inst.min_distance;
            let max_d = inst.max_distance;
            let mut mixer = studio_arc
                .core_sys
                .mixer
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            if let Some(ch) = mixer.channels.get_mut(&cid) {
                ch.min_distance = min_d;
                ch.max_distance = max_d;
            }
        }
    }
    FMOD_OK
}

fn studio_event_instance_set_reverb_level_impl(instance: usize, index: i32, reverb: f32) -> i32 {
    if !(0..4).contains(&index) {
        return FMOD_ERR_INVALID_PARAM;
    }
    if !reverb.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get_mut(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    inst.reverb_levels[index as usize] = reverb;
    FMOD_OK
}

fn studio_event_instance_set_param_by_name_impl(
    instance: usize,
    name: *const u8,
    value: f32,
    _ignoreseekspeed: i32,
) -> i32 {
    if name.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    if !value.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let key = unsafe { crate::strings::from_ansi(name) }.to_ascii_lowercase();
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(inst) = st.instances.get_mut(&instance) {
        if let Some(def) = inst.param_defs.get(&key) {
            let clamped = if def.maximum > def.minimum {
                value.clamp(def.minimum, def.maximum)
            } else {
                value
            };
            inst.param_values.insert(key, clamped);
            return FMOD_OK;
        }
    } else {
        return FMOD_ERR_INVALID_HANDLE;
    }
    if let Some(def) = st.global_param_defs.get(&key) {
        let clamped = if def.maximum > def.minimum {
            value.clamp(def.minimum, def.maximum)
        } else {
            value
        };
        st.global_param_values.insert(key, clamped);
        return FMOD_OK;
    }
    FMOD_ERR_EVENT_NOTFOUND
}

fn studio_event_instance_set_param_by_id_impl(
    instance: usize,
    id: u64,
    value: f32,
    _ignoreseekspeed: i32,
) -> i32 {
    if !value.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let pid = FmodStudioParameterId::from_u64(id);
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(inst) = st.instances.get_mut(&instance) {
        if let Some(key) = inst.param_id_to_key.get(&pid).cloned() {
            if let Some(def) = inst.param_defs.get(&key) {
                let clamped = if def.maximum > def.minimum {
                    value.clamp(def.minimum, def.maximum)
                } else {
                    value
                };
                inst.param_values.insert(key, clamped);
                return FMOD_OK;
            }
        }
    } else {
        return FMOD_ERR_INVALID_HANDLE;
    }
    if let Some(key) = st.global_param_id_to_key.get(&pid).cloned() {
        if let Some(def) = st.global_param_defs.get(&key) {
            let clamped = if def.maximum > def.minimum {
                value.clamp(def.minimum, def.maximum)
            } else {
                value
            };
            st.global_param_values.insert(key, clamped);
            return FMOD_OK;
        }
    }
    FMOD_ERR_EVENT_NOTFOUND
}

fn studio_bus_set_volume_impl(bus: usize, volume: f32) -> i32 {
    if !volume.is_finite() || volume < 0.0 {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let (cg_id, bus_guid) = {
        let Some(b) = st.buses.get_mut(&bus) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        b.volume = volume;
        (b.channel_group_id, b.guid)
    };
    let vca_mult = st.bus_vca_multiplier(bus_guid);
    let mut mixer = studio_arc
        .core_sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(cg) = mixer.groups.get_mut(&cg_id) {
        cg.volume = volume * vca_mult;
    }
    FMOD_OK
}

fn studio_vca_set_volume_impl(vca: usize, volume: f32) -> i32 {
    if !volume.is_finite() || volume < 0.0 {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio_vca(vca) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(v) = st.vcas.get_mut(&vca) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    v.volume = volume;
    let mut mixer = studio_arc
        .core_sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    st.sync_bus_and_instance_gains(&mut mixer);
    FMOD_OK
}

fn param_def_to_abi(def: &StudioParameterDef) -> FmodStudioParameterDescription {
    FmodStudioParameterDescription {
        name: intern_cstr(&def.name),
        id: def.id,
        minimum: def.minimum,
        maximum: def.maximum,
        defaultvalue: def.default_value,
        param_type: def.param_type,
        flags: def.flags,
        guid: def.guid,
    }
}

// ============================================================================
// FMOD Studio C API exports — Utility & System
// ============================================================================

pub unsafe extern "win64" fn FMOD_Studio_ParseID(
    idstring: *const u8,
    id_out: *mut FmodGuid,
) -> i32 {
    if idstring.is_null() || id_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let s = crate::strings::from_ansi(idstring);
    let Some(guid) = FmodGuid::parse_str(&s) else {
        return FMOD_ERR_FORMAT;
    };
    write_out(id_out, guid);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_System_Create(
    system_out: *mut usize,
    _header_version: u32,
) -> i32 {
    if system_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let (core_id, master_group_id, core_sys) = create_core_system_arc();
    let studio_id = next_handle();
    let master_bus_handle = next_handle();
    let studio_arc = Arc::new(StudioSystem {
        core_id,
        core_sys,
        state: Mutex::new(StudioState::new(master_bus_handle, master_group_id)),
    });
    {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        reg.studios.insert(studio_id, studio_arc.clone());
        reg.studio_buses.insert(master_bus_handle, studio_arc);
    }
    write_out(system_out, studio_id);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_System_IsValid(system: usize) -> i32 {
    if lookup_studio(system).is_ok() { 1 } else { 0 }
}

pub unsafe extern "win64" fn FMOD_Studio_System_Initialize(
    system: usize,
    maxchannels: i32,
    _studioflags: u32,
    flags: u32,
    extradriverdata: usize,
) -> i32 {
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    system_init_impl(studio_arc.core_id, maxchannels, flags, extradriverdata)
}

pub unsafe extern "win64" fn FMOD_Studio_System_Update(system: usize) -> i32 {
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };

    let mut rm_groups = Vec::new();
    let mut rm_channels = Vec::new();
    {
        let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());

        // Sync playback state & timeline positions for active event instances.
        for inst in st.instances.values_mut() {
            if let Some(cid) = inst.channel_id {
                if let Some(ch) = mixer.channels.get(&cid) {
                    inst.timeline_ms = ((ch.play_pos.max(0.0) * 1000.0)
                        / (ch.sound_sample_rate.max(1) as f64))
                        as i32;
                    if ch.stopped && inst.state == FMOD_STUDIO_PLAYBACK_PLAYING {
                        inst.state = FMOD_STUDIO_PLAYBACK_STOPPED;
                    }
                } else if inst.state == FMOD_STUDIO_PLAYBACK_PLAYING {
                    inst.state = FMOD_STUDIO_PLAYBACK_STOPPED;
                    inst.channel_id = None;
                }
            }
        }

        // Reap released fire-and-forget instances whose Core channel has finished playing.
        let mut alive_orphans = Vec::new();
        for orph in st.orphans.drain(..) {
            let finished = mixer
                .channels
                .get(&orph.channel_id)
                .map(|c| c.stopped)
                .unwrap_or(true);
            if finished {
                mixer.channels.remove(&orph.channel_id);
                mixer.groups.remove(&orph.channel_group_id);
                rm_channels.push(orph.channel_id);
                rm_groups.push(orph.channel_group_id);
            } else {
                alive_orphans.push(orph);
            }
        }
        st.orphans = alive_orphans;
    }

    if !rm_groups.is_empty() || !rm_channels.is_empty() {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        for g in rm_groups {
            reg.groups.remove(&g);
        }
        for c in rm_channels {
            reg.channels.remove(&c);
        }
    }

    system_update_impl(studio_arc.core_id)
}

pub unsafe extern "win64" fn FMOD_Studio_System_Release(system: usize) -> i32 {
    let studio_arc = {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        match reg.studios.remove(&system) {
            Some(s) => s,
            None => return FMOD_ERR_INVALID_HANDLE,
        }
    };

    let (bank_ids, event_ids, inst_ids, bus_ids, vca_ids) = {
        let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let b: Vec<usize> = st.banks.keys().copied().collect();
        let e: Vec<usize> = st.events.keys().copied().collect();
        let i: Vec<usize> = st.instances.keys().copied().collect();
        let bu: Vec<usize> = st.buses.keys().copied().collect();
        let v: Vec<usize> = st.vcas.keys().copied().collect();
        st.banks.clear();
        st.bank_order.clear();
        st.bank_by_guid.clear();
        st.events.clear();
        st.event_by_guid.clear();
        st.instances.clear();
        st.orphans.clear();
        st.buses.clear();
        st.bus_by_guid.clear();
        st.vcas.clear();
        st.vca_by_guid.clear();
        (b, e, i, bu, v)
    };
    {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        for id in bank_ids {
            reg.studio_banks.remove(&id);
        }
        for id in event_ids {
            reg.studio_events.remove(&id);
        }
        for id in inst_ids {
            reg.studio_instances.remove(&id);
        }
        for id in bus_ids {
            reg.studio_buses.remove(&id);
        }
        for id in vca_ids {
            reg.studio_vcas.remove(&id);
        }
    }

    system_release_impl(studio_arc.core_id)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetCoreSystem(
    system: usize,
    coresystem_out: *mut usize,
) -> i32 {
    if coresystem_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => {
            write_out(coresystem_out, 0);
            return e;
        }
    };
    write_out(coresystem_out, studio_arc.core_id);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_System_LoadBankFile(
    system: usize,
    filename: *const u8,
    _flags: u32,
    bank_out: *mut usize,
) -> i32 {
    write_out(bank_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    if filename.is_null() || bank_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let raw_path = crate::strings::from_ansi(filename);
    let Some(host_path) = resolve_sound_path(&raw_path) else {
        return FMOD_ERR_FILE_NOTFOUND;
    };
    let bytes = match std::fs::read(&host_path) {
        Ok(b) => b,
        Err(_) => return FMOD_ERR_FILE_BAD,
    };
    let parsed = match parse_studio_bank(&bytes, &raw_path) {
        Ok(p) => p,
        Err(e) => return e,
    };
    match load_parsed_bank_into_studio(&studio_arc, parsed) {
        Ok(h) => {
            write_out(bank_out, h);
            FMOD_OK
        }
        Err(e) => e,
    }
}

pub unsafe extern "win64" fn FMOD_Studio_System_LoadBankMemory(
    system: usize,
    buffer: *const u8,
    length: i32,
    mode: u32,
    _flags: u32,
    bank_out: *mut usize,
) -> i32 {
    write_out(bank_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    if buffer.is_null() || length <= 0 || bank_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    if mode != FMOD_STUDIO_LOAD_MEMORY && mode != FMOD_STUDIO_LOAD_MEMORY_POINT {
        return FMOD_ERR_INVALID_PARAM;
    }
    let slice = std::slice::from_raw_parts(buffer, length as usize);
    let parsed = match parse_studio_bank(slice, "") {
        Ok(p) => p,
        Err(e) => return e,
    };
    match load_parsed_bank_into_studio(&studio_arc, parsed) {
        Ok(h) => {
            write_out(bank_out, h);
            FMOD_OK
        }
        Err(e) => e,
    }
}

pub unsafe extern "win64" fn FMOD_Studio_System_UnloadAll(system: usize) -> i32 {
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let bank_ids: Vec<usize> = {
        let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        st.bank_order.clone()
    };
    for bid in bank_ids {
        let _ = unload_bank_impl(&studio_arc, bid);
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_System_FlushCommands(system: usize) -> i32 {
    match lookup_studio(system) {
        Ok(_) => FMOD_OK,
        Err(e) => e,
    }
}

pub unsafe extern "win64" fn FMOD_Studio_System_FlushSampleLoading(system: usize) -> i32 {
    match lookup_studio(system) {
        Ok(_) => FMOD_OK,
        Err(e) => e,
    }
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetBank(
    system: usize,
    path: *const u8,
    bank_out: *mut usize,
) -> i32 {
    write_out(bank_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    if path.is_null() || bank_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let raw = crate::strings::from_ansi(path);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(g) = st.resolve_path_or_guid(&raw) {
        if let Some(&h) = st.bank_by_guid.get(&g) {
            write_out(bank_out, h);
            return FMOD_OK;
        }
    }
    let norm = normalize_studio_path(&raw);
    for (&h, bank) in &st.banks {
        if normalize_studio_path(&bank.path) == norm {
            write_out(bank_out, h);
            return FMOD_OK;
        }
    }
    FMOD_ERR_EVENT_NOTFOUND
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetBankByID(
    system: usize,
    id: *const FmodGuid,
    bank_out: *mut usize,
) -> i32 {
    write_out(bank_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let Some(guid) = read_in(id) else {
        return FMOD_ERR_INVALID_PARAM;
    };
    if bank_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(&h) = st.bank_by_guid.get(&guid) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_out(bank_out, h);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetBankCount(
    system: usize,
    count_out: *mut i32,
) -> i32 {
    if count_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    write_out(count_out, st.banks.len() as i32);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetBankList(
    system: usize,
    array: *mut usize,
    capacity: i32,
    count_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    write_handle_list(&st.bank_order, array, capacity, count_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetEvent(
    system: usize,
    path: *const u8,
    event_out: *mut usize,
) -> i32 {
    write_out(event_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    if path.is_null() || event_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let raw = crate::strings::from_ansi(path);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(g) = st.resolve_path_or_guid(&raw) {
        if let Some(&h) = st.event_by_guid.get(&g) {
            write_out(event_out, h);
            return FMOD_OK;
        }
    }
    let norm = normalize_studio_path(&raw);
    for (&h, ev) in &st.events {
        if !ev.path.is_empty() && normalize_studio_path(&ev.path) == norm {
            write_out(event_out, h);
            return FMOD_OK;
        }
    }
    FMOD_ERR_EVENT_NOTFOUND
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetEventByID(
    system: usize,
    id: *const FmodGuid,
    event_out: *mut usize,
) -> i32 {
    write_out(event_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let Some(guid) = read_in(id) else {
        return FMOD_ERR_INVALID_PARAM;
    };
    if event_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(&h) = st.event_by_guid.get(&guid) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_out(event_out, h);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetBus(
    system: usize,
    path: *const u8,
    bus_out: *mut usize,
) -> i32 {
    write_out(bus_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    if path.is_null() || bus_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let raw = crate::strings::from_ansi(path);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let norm = normalize_studio_path(&raw);
    if norm.is_empty() || norm == "bus:/" {
        write_out(bus_out, st.master_bus_handle);
        return FMOD_OK;
    }
    if let Some(g) = st.resolve_path_or_guid(&raw) {
        if let Some(&h) = st.bus_by_guid.get(&g) {
            write_out(bus_out, h);
            return FMOD_OK;
        }
    }
    for (&h, b) in &st.buses {
        if !b.path.is_empty() && normalize_studio_path(&b.path) == norm {
            write_out(bus_out, h);
            return FMOD_OK;
        }
    }
    FMOD_ERR_EVENT_NOTFOUND
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetBusByID(
    system: usize,
    id: *const FmodGuid,
    bus_out: *mut usize,
) -> i32 {
    write_out(bus_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let Some(guid) = read_in(id) else {
        return FMOD_ERR_INVALID_PARAM;
    };
    if bus_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(&h) = st.bus_by_guid.get(&guid) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_out(bus_out, h);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetVCA(
    system: usize,
    path: *const u8,
    vca_out: *mut usize,
) -> i32 {
    write_out(vca_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    if path.is_null() || vca_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let raw = crate::strings::from_ansi(path);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(g) = st.resolve_path_or_guid(&raw) {
        if let Some(&h) = st.vca_by_guid.get(&g) {
            write_out(vca_out, h);
            return FMOD_OK;
        }
    }
    let norm = normalize_studio_path(&raw);
    for (&h, v) in &st.vcas {
        if !v.path.is_empty() && normalize_studio_path(&v.path) == norm {
            write_out(vca_out, h);
            return FMOD_OK;
        }
    }
    FMOD_ERR_EVENT_NOTFOUND
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetVCAByID(
    system: usize,
    id: *const FmodGuid,
    vca_out: *mut usize,
) -> i32 {
    write_out(vca_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let Some(guid) = read_in(id) else {
        return FMOD_ERR_INVALID_PARAM;
    };
    if vca_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(&h) = st.vca_by_guid.get(&guid) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_out(vca_out, h);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_System_LookupID(
    system: usize,
    path: *const u8,
    id_out: *mut FmodGuid,
) -> i32 {
    if path.is_null() || id_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let raw = crate::strings::from_ansi(path);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = st.resolve_path_or_guid(&raw) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_out(id_out, g);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_System_LookupPath(
    system: usize,
    id: *const FmodGuid,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let Some(guid) = read_in(id) else {
        return FMOD_ERR_INVALID_PARAM;
    };
    let found = {
        let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        st.guid_to_path.get(&guid).cloned()
    };
    let Some(p) = found else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_studio_path(&p, path_out, size, retrieved_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_SetNumListeners(
    system: usize,
    numlisteners: i32,
) -> i32 {
    if !(1..=8).contains(&numlisteners) {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    studio_arc
        .state
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .num_listeners = numlisteners as usize;
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetNumListeners(
    system: usize,
    numlisteners_out: *mut i32,
) -> i32 {
    if numlisteners_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let n = studio_arc
        .state
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .num_listeners;
    write_out(numlisteners_out, n as i32);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_System_SetListenerAttributes(
    system: usize,
    listener: i32,
    attributes: *const Fmod3DAttributes,
    attenuationposition: *const FmodVector,
) -> i32 {
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let Some(attrs) = read_in(attributes) else {
        return FMOD_ERR_INVALID_PARAM;
    };
    let atten_opt = read_in(attenuationposition);
    for v in [attrs.position, attrs.velocity, attrs.forward, attrs.up]
        .into_iter()
        .chain(atten_opt)
    {
        if !v.x.is_finite() || !v.y.is_finite() || !v.z.is_finite() {
            return FMOD_ERR_INVALID_FLOAT;
        }
    }
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if listener < 0 || (listener as usize) >= st.num_listeners {
        return FMOD_ERR_INVALID_PARAM;
    }
    st.listeners[listener as usize].attributes = attrs;
    st.listeners[listener as usize].attenuation_pos = atten_opt;
    if listener == 0 {
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        mixer.listener = Listener3D {
            pos: atten_opt.unwrap_or(attrs.position),
            vel: attrs.velocity,
            forward: attrs.forward,
            up: attrs.up,
        };
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetListenerAttributes(
    system: usize,
    listener: i32,
    attributes_out: *mut Fmod3DAttributes,
    attenuationposition_out: *mut FmodVector,
) -> i32 {
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if listener < 0 || (listener as usize) >= st.num_listeners {
        return FMOD_ERR_INVALID_PARAM;
    }
    let l = st.listeners[listener as usize];
    write_out(attributes_out, l.attributes);
    if !attenuationposition_out.is_null() {
        write_out(
            attenuationposition_out,
            l.attenuation_pos.unwrap_or(l.attributes.position),
        );
    }
    FMOD_OK
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_Studio_System_SetListenerWeight(
    system: usize,
    listener: i32,
    weight: f32,
) -> i32 {
    studio_system_set_listener_weight_impl(system, listener, weight)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_Studio_System_SetListenerWeight(
    system: usize,
    listener: i32,
    _xmm0: f32,
    _xmm1: f32,
    weight: f32,
) -> i32 {
    studio_system_set_listener_weight_impl(system, listener, weight)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetListenerWeight(
    system: usize,
    listener: i32,
    weight_out: *mut f32,
) -> i32 {
    if weight_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if listener < 0 || (listener as usize) >= st.num_listeners {
        return FMOD_ERR_INVALID_PARAM;
    }
    write_out(weight_out, st.listeners[listener as usize].weight);
    FMOD_OK
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_Studio_System_SetParameterByName(
    system: usize,
    name: *const u8,
    value: f32,
    ignoreseekspeed: i32,
) -> i32 {
    studio_system_set_param_by_name_impl(system, name, value, ignoreseekspeed)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_Studio_System_SetParameterByName(
    system: usize,
    name: *const u8,
    _r8: u64,
    ignoreseekspeed: i32,
    _xmm0: f32,
    _xmm1: f32,
    value: f32,
) -> i32 {
    studio_system_set_param_by_name_impl(system, name, value, ignoreseekspeed)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetParameterByName(
    system: usize,
    name: *const u8,
    value_out: *mut f32,
    finalvalue_out: *mut f32,
) -> i32 {
    if name.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let key = crate::strings::from_ansi(name).to_ascii_lowercase();
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(&val) = st.global_param_values.get(&key) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_out(value_out, val);
    write_out(finalvalue_out, val);
    FMOD_OK
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_Studio_System_SetParameterByID(
    system: usize,
    id: u64,
    value: f32,
    ignoreseekspeed: i32,
) -> i32 {
    studio_system_set_param_by_id_impl(system, id, value, ignoreseekspeed)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_Studio_System_SetParameterByID(
    system: usize,
    id: u64,
    _r8: u64,
    ignoreseekspeed: i32,
    _xmm0: f32,
    _xmm1: f32,
    value: f32,
) -> i32 {
    studio_system_set_param_by_id_impl(system, id, value, ignoreseekspeed)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetParameterByID(
    system: usize,
    id: u64,
    value_out: *mut f32,
    finalvalue_out: *mut f32,
) -> i32 {
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let pid = FmodStudioParameterId::from_u64(id);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(key) = st.global_param_id_to_key.get(&pid) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    let Some(&val) = st.global_param_values.get(key) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_out(value_out, val);
    write_out(finalvalue_out, val);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetParameterDescriptionByName(
    system: usize,
    name: *const u8,
    parameter_out: *mut FmodStudioParameterDescription,
) -> i32 {
    if name.is_null() || parameter_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let key = crate::strings::from_ansi(name).to_ascii_lowercase();
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(def) = st.global_param_defs.get(&key) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_out(parameter_out, param_def_to_abi(def));
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetParameterDescriptionByID(
    system: usize,
    id: u64,
    parameter_out: *mut FmodStudioParameterDescription,
) -> i32 {
    if parameter_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let pid = FmodStudioParameterId::from_u64(id);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(key) = st.global_param_id_to_key.get(&pid) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    let Some(def) = st.global_param_defs.get(key) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_out(parameter_out, param_def_to_abi(def));
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetParameterDescriptionCount(
    system: usize,
    count_out: *mut i32,
) -> i32 {
    if count_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    write_out(count_out, st.global_param_defs.len() as i32);
    FMOD_OK
}

// ============================================================================
// FMOD Studio C API exports — Bank
// ============================================================================

pub unsafe extern "win64" fn FMOD_Studio_Bank_IsValid(bank: usize) -> i32 {
    let Ok(studio_arc) = lookup_studio_bank(bank) else {
        return 0;
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if st.banks.contains_key(&bank) { 1 } else { 0 }
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetID(bank: usize, id_out: *mut FmodGuid) -> i32 {
    if id_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(id_out, b.guid);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetPath(
    bank: usize,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let path = {
        let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let Some(b) = st.banks.get(&bank) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        if !b.path.is_empty() {
            b.path.clone()
        } else {
            st.guid_to_path.get(&b.guid).cloned().unwrap_or_default()
        }
    };
    write_studio_path(&path, path_out, size, retrieved_out)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_Unload(bank: usize) -> i32 {
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    unload_bank_impl(&studio_arc, bank)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_LoadSampleData(bank: usize) -> i32 {
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let evs = {
        let Some(b) = st.banks.get_mut(&bank) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        b.sample_loading_state = FMOD_STUDIO_LOADING_STATE_LOADED;
        b.event_handles.clone()
    };
    for eh in evs {
        if let Some(ev) = st.events.get_mut(&eh) {
            ev.sample_loading_state = FMOD_STUDIO_LOADING_STATE_LOADED;
        }
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_UnloadSampleData(bank: usize) -> i32 {
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let evs = {
        let Some(b) = st.banks.get_mut(&bank) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        b.sample_loading_state = FMOD_STUDIO_LOADING_STATE_UNLOADED;
        b.event_handles.clone()
    };
    for eh in evs {
        if let Some(ev) = st.events.get_mut(&eh) {
            ev.sample_loading_state = FMOD_STUDIO_LOADING_STATE_UNLOADED;
        }
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetLoadingState(
    bank: usize,
    state_out: *mut u32,
) -> i32 {
    if state_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(state_out, b.loading_state);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetSampleLoadingState(
    bank: usize,
    state_out: *mut u32,
) -> i32 {
    if state_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(state_out, b.sample_loading_state);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetStringCount(
    bank: usize,
    count_out: *mut i32,
) -> i32 {
    if count_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(count_out, b.strings.len() as i32);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetStringInfo(
    bank: usize,
    index: i32,
    id_out: *mut FmodGuid,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    if index < 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let entry = {
        let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let Some(b) = st.banks.get(&bank) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        b.strings.get(index as usize).cloned()
    };
    let Some((guid, path)) = entry else {
        return FMOD_ERR_INVALID_PARAM;
    };
    write_out(id_out, guid);
    if path_out.is_null() && size == 0 && retrieved_out.is_null() {
        return FMOD_OK;
    }
    write_studio_path(&path, path_out, size, retrieved_out)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetEventCount(
    bank: usize,
    count_out: *mut i32,
) -> i32 {
    if count_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(count_out, b.event_handles.len() as i32);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetEventList(
    bank: usize,
    array: *mut usize,
    capacity: i32,
    count_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_handle_list(&b.event_handles, array, capacity, count_out)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetBusCount(
    bank: usize,
    count_out: *mut i32,
) -> i32 {
    if count_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(count_out, b.bus_handles.len() as i32);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetBusList(
    bank: usize,
    array: *mut usize,
    capacity: i32,
    count_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_handle_list(&b.bus_handles, array, capacity, count_out)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetVCACount(
    bank: usize,
    count_out: *mut i32,
) -> i32 {
    if count_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(count_out, b.vca_handles.len() as i32);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetVCAList(
    bank: usize,
    array: *mut usize,
    capacity: i32,
    count_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_handle_list(&b.vca_handles, array, capacity, count_out)
}

// ============================================================================
// FMOD Studio C API exports — EventDescription
// ============================================================================

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_IsValid(event_desc: usize) -> i32 {
    let Ok(studio_arc) = lookup_studio_event(event_desc) else {
        return 0;
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if st.events.contains_key(&event_desc) { 1 } else { 0 }
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetID(
    event_desc: usize,
    id_out: *mut FmodGuid,
) -> i32 {
    if id_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(id_out, ev.guid);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetPath(
    event_desc: usize,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let path = {
        let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let Some(ev) = st.events.get(&event_desc) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        if !ev.path.is_empty() {
            ev.path.clone()
        } else {
            st.guid_to_path.get(&ev.guid).cloned().unwrap_or_default()
        }
    };
    write_studio_path(&path, path_out, size, retrieved_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetParameterDescriptionCount(
    event_desc: usize,
    count_out: *mut i32,
) -> i32 {
    if count_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(count_out, ev.parameters.len() as i32);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetParameterDescriptionByIndex(
    event_desc: usize,
    index: i32,
    parameter_out: *mut FmodStudioParameterDescription,
) -> i32 {
    if index < 0 || parameter_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let Some(def) = ev.parameters.get(index as usize) else {
        return FMOD_ERR_INVALID_PARAM;
    };
    write_out(parameter_out, param_def_to_abi(def));
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetParameterDescriptionByName(
    event_desc: usize,
    name: *const u8,
    parameter_out: *mut FmodStudioParameterDescription,
) -> i32 {
    if name.is_null() || parameter_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let target = crate::strings::from_ansi(name);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    if let Some(def) = ev
        .parameters
        .iter()
        .find(|p| p.name.eq_ignore_ascii_case(&target))
    {
        write_out(parameter_out, param_def_to_abi(def));
        return FMOD_OK;
    }
    if let Some(def) = st.global_param_defs.get(&target.to_ascii_lowercase()) {
        write_out(parameter_out, param_def_to_abi(def));
        return FMOD_OK;
    }
    FMOD_ERR_EVENT_NOTFOUND
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetParameterDescriptionByID(
    event_desc: usize,
    id: u64,
    parameter_out: *mut FmodStudioParameterDescription,
) -> i32 {
    if parameter_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let pid = FmodStudioParameterId::from_u64(id);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    if let Some(def) = ev.parameters.iter().find(|p| p.id == pid) {
        write_out(parameter_out, param_def_to_abi(def));
        return FMOD_OK;
    }
    if let Some(key) = st.global_param_id_to_key.get(&pid) {
        if let Some(def) = st.global_param_defs.get(key) {
            write_out(parameter_out, param_def_to_abi(def));
            return FMOD_OK;
        }
    }
    FMOD_ERR_EVENT_NOTFOUND
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetLength(
    event_desc: usize,
    length_out: *mut i32,
) -> i32 {
    if length_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(length_out, ev.length_ms);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetMinMaxDistance(
    event_desc: usize,
    min_out: *mut f32,
    max_out: *mut f32,
) -> i32 {
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(min_out, ev.min_distance);
    write_out(max_out, ev.max_distance);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetSoundSize(
    event_desc: usize,
    size_out: *mut f32,
) -> i32 {
    if size_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(size_out, ev.sound_size);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_IsSnapshot(
    event_desc: usize,
    snapshot_out: *mut i32,
) -> i32 {
    if snapshot_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(snapshot_out, if ev.is_snapshot { 1 } else { 0 });
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_IsOneshot(
    event_desc: usize,
    oneshot_out: *mut i32,
) -> i32 {
    if oneshot_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(oneshot_out, if ev.is_oneshot { 1 } else { 0 });
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_IsStream(
    event_desc: usize,
    isstream_out: *mut i32,
) -> i32 {
    if isstream_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(isstream_out, if ev.is_stream { 1 } else { 0 });
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_Is3D(
    event_desc: usize,
    is3d_out: *mut i32,
) -> i32 {
    if is3d_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(is3d_out, if ev.is_3d { 1 } else { 0 });
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_HasSustainPoint(
    event_desc: usize,
    sustainpoint_out: *mut i32,
) -> i32 {
    if sustainpoint_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(sustainpoint_out, if ev.has_sustain_point { 1 } else { 0 });
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_CreateInstance(
    event_desc: usize,
    instance_out: *mut usize,
) -> i32 {
    write_out(instance_out, 0);
    if instance_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };

    let inst_handle = next_handle();
    let cg_id = next_handle();
    {
        let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());

        let Some(ev) = st.events.get(&event_desc) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        let parent_cg = st.resolve_event_bus_cg(ev.bus_guid, mixer.master_group_id);
        let ev_vca = st.event_vca_multiplier(ev.guid);

        let mut param_defs = HashMap::new();
        let mut param_id_to_key = HashMap::new();
        let mut param_values = HashMap::new();
        for p in &ev.parameters {
            let key = p.name.to_ascii_lowercase();
            param_id_to_key.insert(p.id, key.clone());
            param_values.insert(key.clone(), p.default_value);
            param_defs.insert(key, p.clone());
        }

        let mut properties = [-1.0f32; 6];
        properties[FMOD_STUDIO_EVENT_PROPERTY_MINIMUM_DISTANCE as usize] = ev.min_distance;
        properties[FMOD_STUDIO_EVENT_PROPERTY_MAXIMUM_DISTANCE as usize] = ev.max_distance;

        let cg_name = if ev.path.is_empty() {
            format!("event:{inst_handle:#x}")
        } else {
            ev.path.clone()
        };
        mixer.groups.insert(
            cg_id,
            ChannelGroup {
                name: cg_name,
                parent: Some(parent_cg),
                volume: ev.default_volume * ev_vca,
                mute: false,
                paused: false,
                pitch: ev.default_pitch,
            },
        );

        let inst = StudioEventInstance {
            desc_handle: event_desc,
            event_guid: ev.guid,
            bank_handle: ev.bank_handle,
            channel_group_id: cg_id,
            channel_id: None,
            volume: 1.0,
            pitch: 1.0,
            paused: false,
            state: FMOD_STUDIO_PLAYBACK_STOPPED,
            attributes_3d: Fmod3DAttributes::default(),
            min_distance: ev.min_distance,
            max_distance: ev.max_distance,
            timeline_ms: 0,
            reverb_levels: [0.0; 4],
            properties,
            param_defs,
            param_id_to_key,
            param_values,
            sample: ev.sample.clone(),
            is_3d: ev.is_3d,
            is_oneshot: ev.is_oneshot,
            desc_default_volume: ev.default_volume,
            desc_default_pitch: ev.default_pitch,
        };
        st.instances.insert(inst_handle, inst);
        if let Some(ev_mut) = st.events.get_mut(&event_desc) {
            ev_mut.instances.push(inst_handle);
        }
    }
    {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        reg.studio_instances.insert(inst_handle, studio_arc.clone());
        reg.groups.insert(cg_id, studio_arc.core_sys.clone());
    }
    write_out(instance_out, inst_handle);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetInstanceCount(
    event_desc: usize,
    count_out: *mut i32,
) -> i32 {
    if count_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(count_out, ev.instances.len() as i32);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetInstanceList(
    event_desc: usize,
    array: *mut usize,
    capacity: i32,
    count_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_handle_list(&ev.instances, array, capacity, count_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_LoadSampleData(
    event_desc: usize,
) -> i32 {
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get_mut(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ev.sample_loading_state = FMOD_STUDIO_LOADING_STATE_LOADED;
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_UnloadSampleData(
    event_desc: usize,
) -> i32 {
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get_mut(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ev.sample_loading_state = FMOD_STUDIO_LOADING_STATE_UNLOADED;
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetSampleLoadingState(
    event_desc: usize,
    state_out: *mut u32,
) -> i32 {
    if state_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(state_out, ev.sample_loading_state);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_ReleaseAllInstances(
    event_desc: usize,
) -> i32 {
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let inst_ids: Vec<usize> = {
        let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let Some(ev) = st.events.get(&event_desc) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        ev.instances.clone()
    };
    for ih in inst_ids {
        let _ = FMOD_Studio_EventInstance_Release(ih);
    }
    FMOD_OK
}

// ============================================================================
// FMOD Studio C API exports — EventInstance
// ============================================================================

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_IsValid(instance: usize) -> i32 {
    let Ok(studio_arc) = lookup_studio_instance(instance) else {
        return 0;
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if st.instances.contains_key(&instance) { 1 } else { 0 }
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetDescription(
    instance: usize,
    description_out: *mut usize,
) -> i32 {
    if description_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(description_out, inst.desc_handle);
    FMOD_OK
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_Studio_EventInstance_SetVolume(
    instance: usize,
    volume: f32,
) -> i32 {
    studio_event_instance_set_volume_impl(instance, volume)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_Studio_EventInstance_SetVolume(
    instance: usize,
    _xmm0: f32,
    volume: f32,
) -> i32 {
    studio_event_instance_set_volume_impl(instance, volume)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetVolume(
    instance: usize,
    volume_out: *mut f32,
    finalvolume_out: *mut f32,
) -> i32 {
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let vol = inst.volume;
    let final_vol = {
        let mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        resolve_group_chain(&mixer.groups, inst.channel_group_id).volume
    };
    write_out(volume_out, vol);
    write_out(finalvolume_out, final_vol);
    FMOD_OK
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_Studio_EventInstance_SetPitch(
    instance: usize,
    pitch: f32,
) -> i32 {
    studio_event_instance_set_pitch_impl(instance, pitch)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_Studio_EventInstance_SetPitch(
    instance: usize,
    _xmm0: f32,
    pitch: f32,
) -> i32 {
    studio_event_instance_set_pitch_impl(instance, pitch)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetPitch(
    instance: usize,
    pitch_out: *mut f32,
    finalpitch_out: *mut f32,
) -> i32 {
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let pitch = inst.pitch;
    let final_pitch = {
        let mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        resolve_group_chain(&mixer.groups, inst.channel_group_id).pitch
    };
    write_out(pitch_out, pitch);
    write_out(finalpitch_out, final_pitch);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_Set3DAttributes(
    instance: usize,
    attributes: *const Fmod3DAttributes,
) -> i32 {
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let Some(attrs) = read_in(attributes) else {
        return FMOD_ERR_INVALID_PARAM;
    };
    for v in [attrs.position, attrs.velocity, attrs.forward, attrs.up] {
        if !v.x.is_finite() || !v.y.is_finite() || !v.z.is_finite() {
            return FMOD_ERR_INVALID_FLOAT;
        }
    }
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get_mut(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    inst.attributes_3d = attrs;
    if let Some(cid) = inst.channel_id {
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if let Some(ch) = mixer.channels.get_mut(&cid) {
            ch.pos_3d = attrs.position;
            ch.vel_3d = attrs.velocity;
        }
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_Get3DAttributes(
    instance: usize,
    attributes_out: *mut Fmod3DAttributes,
) -> i32 {
    if attributes_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(attributes_out, inst.attributes_3d);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_SetPaused(
    instance: usize,
    paused: i32,
) -> i32 {
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get_mut(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    inst.paused = paused != 0;
    let cg_id = inst.channel_group_id;
    let mut mixer = studio_arc
        .core_sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(cg) = mixer.groups.get_mut(&cg_id) {
        cg.paused = paused != 0;
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetPaused(
    instance: usize,
    paused_out: *mut i32,
) -> i32 {
    if paused_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(paused_out, if inst.paused { 1 } else { 0 });
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_Start(instance: usize) -> i32 {
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };

    let mut old_channel = None;
    let mut new_channel = None;

    {
        let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());

        if !mixer.initialized {
            return FMOD_ERR_STUDIO_UNINITIALIZED;
        }
        let Some(inst) = st.instances.get_mut(&instance) else {
            return FMOD_ERR_INVALID_HANDLE;
        };

        if let Some(prev_cid) = inst.channel_id.take() {
            mixer.channels.remove(&prev_cid);
            old_channel = Some(prev_cid);
        }

        if let Some(ref smp) = inst.sample {
            if smp.num_frames > 0 && smp.channels > 0 {
                let active_count = mixer.channels.values().filter(|c| !c.stopped).count();
                if active_count >= mixer.max_channels {
                    return FMOD_ERR_TOOMANYCHANNELS;
                }
                let looping = !inst.is_oneshot || smp.has_loop;
                let mut mode = if inst.is_3d { FMOD_3D } else { FMOD_2D };
                mode |= if looping {
                    FMOD_LOOP_NORMAL
                } else {
                    FMOD_LOOP_OFF
                };
                let loop_count = if looping { -1 } else { 0 };
                let frame_bytes =
                    (smp.channels as usize) * ((smp.bits_per_sample as usize) / 8).max(1);
                let start_frame = if inst.timeline_ms > 0 {
                    ((inst.timeline_ms as f64) * (smp.sample_rate as f64) / 1000.0)
                        .clamp(0.0, smp.num_frames as f64)
                } else {
                    0.0
                };

                let cid = next_handle();
                mixer.channels.insert(
                    cid,
                    Channel {
                        sound_id: 0,
                        group_id: inst.channel_group_id,
                        samples: smp.samples.clone(),
                        sound_channels: smp.channels as usize,
                        sound_sample_rate: smp.sample_rate,
                        sound_frame_bytes: frame_bytes,
                        num_frames: smp.num_frames,
                        loop_start: if smp.has_loop { smp.loop_start } else { 0 },
                        loop_end: if smp.has_loop {
                            smp.loop_end
                        } else {
                            smp.num_frames
                        },
                        play_pos: start_frame,
                        frequency: smp.sample_rate as f32,
                        pitch: 1.0,
                        volume: 1.0,
                        pan: 0.0,
                        mute: false,
                        paused: false,
                        stopped: false,
                        mode,
                        loop_count,
                        loops_remaining: loop_count,
                        pos_3d: inst.attributes_3d.position,
                        vel_3d: inst.attributes_3d.velocity,
                        min_distance: inst.min_distance,
                        max_distance: inst.max_distance,
                    },
                );
                inst.channel_id = Some(cid);
                new_channel = Some(cid);
            }
        }
        inst.state = FMOD_STUDIO_PLAYBACK_PLAYING;
    }

    if old_channel.is_some() || new_channel.is_some() {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        if let Some(old_cid) = old_channel {
            reg.channels.remove(&old_cid);
        }
        if let Some(new_cid) = new_channel {
            reg.channels.insert(new_cid, studio_arc.core_sys.clone());
        }
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_Stop(
    instance: usize,
    mode: u32,
) -> i32 {
    if mode != FMOD_STUDIO_STOP_ALLOWFADEOUT && mode != FMOD_STUDIO_STOP_IMMEDIATE {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get_mut(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    inst.state = FMOD_STUDIO_PLAYBACK_STOPPED;
    if let Some(cid) = inst.channel_id {
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if let Some(ch) = mixer.channels.get_mut(&cid) {
            ch.stopped = true;
        }
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetTimelinePosition(
    instance: usize,
    position_out: *mut i32,
) -> i32 {
    if position_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get_mut(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    if let Some(cid) = inst.channel_id {
        let mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if let Some(ch) = mixer.channels.get(&cid) {
            inst.timeline_ms = ((ch.play_pos.max(0.0) * 1000.0)
                / (ch.sound_sample_rate.max(1) as f64)) as i32;
        }
    }
    write_out(position_out, inst.timeline_ms);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_SetTimelinePosition(
    instance: usize,
    position: i32,
) -> i32 {
    if position < 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get_mut(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    inst.timeline_ms = position;
    if let Some(cid) = inst.channel_id {
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if let Some(ch) = mixer.channels.get_mut(&cid) {
            let frame = ((position as f64) * (ch.sound_sample_rate as f64) / 1000.0)
                .clamp(0.0, ch.num_frames as f64);
            ch.play_pos = frame;
        }
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetPlaybackState(
    instance: usize,
    state_out: *mut u32,
) -> i32 {
    if state_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get_mut(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    if inst.state == FMOD_STUDIO_PLAYBACK_PLAYING {
        if let Some(cid) = inst.channel_id {
            let mixer = studio_arc
                .core_sys
                .mixer
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            let finished = mixer
                .channels
                .get(&cid)
                .map(|c| c.stopped)
                .unwrap_or(true);
            if finished {
                inst.state = FMOD_STUDIO_PLAYBACK_STOPPED;
            }
        }
    }
    write_out(state_out, inst.state);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetChannelGroup(
    instance: usize,
    group_out: *mut usize,
) -> i32 {
    if group_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(group_out, inst.channel_group_id);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_Release(instance: usize) -> i32 {
    let studio_arc = {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        match reg.studio_instances.remove(&instance) {
            Some(s) => s,
            None => return FMOD_ERR_INVALID_HANDLE,
        }
    };

    let mut rm_group = None;
    let mut rm_channel = None;
    {
        let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let Some(inst) = st.instances.remove(&instance) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        if let Some(ev) = st.events.get_mut(&inst.desc_handle) {
            ev.instances.retain(|&h| h != instance);
        }

        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let still_playing = inst
            .channel_id
            .and_then(|cid| mixer.channels.get(&cid))
            .is_some_and(|ch| !ch.stopped);

        if still_playing {
            // Fire-and-forget semantics (`Start` -> `Release`): let the live mixer finish
            // playing the active channel and reap it on `Studio_System_Update` / `Bank_Unload`.
            if let Some(cid) = inst.channel_id {
                st.orphans.push(OrphanInstance {
                    bank_handle: inst.bank_handle,
                    channel_group_id: inst.channel_group_id,
                    channel_id: cid,
                });
            }
        } else {
            if let Some(cid) = inst.channel_id {
                mixer.channels.remove(&cid);
                rm_channel = Some(cid);
            }
            mixer.groups.remove(&inst.channel_group_id);
            rm_group = Some(inst.channel_group_id);
        }
    }

    if rm_group.is_some() || rm_channel.is_some() {
        let mut reg = registry().lock().unwrap_or_else(|e| e.into_inner());
        if let Some(cg) = rm_group {
            reg.groups.remove(&cg);
        }
        if let Some(cid) = rm_channel {
            reg.channels.remove(&cid);
        }
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_IsVirtual(
    instance: usize,
    virtualstate_out: *mut i32,
) -> i32 {
    if virtualstate_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if !st.instances.contains_key(&instance) {
        return FMOD_ERR_INVALID_HANDLE;
    }
    write_out(virtualstate_out, 0);
    FMOD_OK
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_Studio_EventInstance_SetParameterByName(
    instance: usize,
    name: *const u8,
    value: f32,
    ignoreseekspeed: i32,
) -> i32 {
    studio_event_instance_set_param_by_name_impl(instance, name, value, ignoreseekspeed)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_Studio_EventInstance_SetParameterByName(
    instance: usize,
    name: *const u8,
    _r8: u64,
    ignoreseekspeed: i32,
    _xmm0: f32,
    _xmm1: f32,
    value: f32,
) -> i32 {
    studio_event_instance_set_param_by_name_impl(instance, name, value, ignoreseekspeed)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetParameterByName(
    instance: usize,
    name: *const u8,
    value_out: *mut f32,
    finalvalue_out: *mut f32,
) -> i32 {
    if name.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let key = crate::strings::from_ansi(name).to_ascii_lowercase();
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    if let Some(def) = inst.param_defs.get(&key) {
        let set_val = inst
            .param_values
            .get(&key)
            .copied()
            .unwrap_or(def.default_value);
        let final_val = if def.param_type == FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE {
            let d = st.compute_listener_distance(inst.attributes_3d.position);
            if def.maximum > def.minimum {
                d.clamp(def.minimum, def.maximum)
            } else {
                d
            }
        } else {
            set_val
        };
        write_out(value_out, set_val);
        write_out(finalvalue_out, final_val);
        return FMOD_OK;
    }
    if let Some(&val) = st.global_param_values.get(&key) {
        write_out(value_out, val);
        write_out(finalvalue_out, val);
        return FMOD_OK;
    }
    FMOD_ERR_EVENT_NOTFOUND
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_Studio_EventInstance_SetParameterByID(
    instance: usize,
    id: u64,
    value: f32,
    ignoreseekspeed: i32,
) -> i32 {
    studio_event_instance_set_param_by_id_impl(instance, id, value, ignoreseekspeed)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_Studio_EventInstance_SetParameterByID(
    instance: usize,
    id: u64,
    _r8: u64,
    ignoreseekspeed: i32,
    _xmm0: f32,
    _xmm1: f32,
    value: f32,
) -> i32 {
    studio_event_instance_set_param_by_id_impl(instance, id, value, ignoreseekspeed)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetParameterByID(
    instance: usize,
    id: u64,
    value_out: *mut f32,
    finalvalue_out: *mut f32,
) -> i32 {
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let pid = FmodStudioParameterId::from_u64(id);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    if let Some(key) = inst.param_id_to_key.get(&pid) {
        if let Some(def) = inst.param_defs.get(key) {
            let set_val = inst
                .param_values
                .get(key)
                .copied()
                .unwrap_or(def.default_value);
            let final_val = if def.param_type == FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE {
                let d = st.compute_listener_distance(inst.attributes_3d.position);
                if def.maximum > def.minimum {
                    d.clamp(def.minimum, def.maximum)
                } else {
                    d
                }
            } else {
                set_val
            };
            write_out(value_out, set_val);
            write_out(finalvalue_out, final_val);
            return FMOD_OK;
        }
    }
    if let Some(key) = st.global_param_id_to_key.get(&pid) {
        if let Some(&val) = st.global_param_values.get(key) {
            write_out(value_out, val);
            write_out(finalvalue_out, val);
            return FMOD_OK;
        }
    }
    FMOD_ERR_EVENT_NOTFOUND
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetMinMaxDistance(
    instance: usize,
    min_out: *mut f32,
    max_out: *mut f32,
) -> i32 {
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(min_out, inst.min_distance);
    write_out(max_out, inst.max_distance);
    FMOD_OK
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_Studio_EventInstance_SetProperty(
    instance: usize,
    index: u32,
    value: f32,
) -> i32 {
    studio_event_instance_set_property_impl(instance, index, value)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_Studio_EventInstance_SetProperty(
    instance: usize,
    index: u32,
    _xmm0: f32,
    _xmm1: f32,
    value: f32,
) -> i32 {
    studio_event_instance_set_property_impl(instance, index, value)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetProperty(
    instance: usize,
    index: u32,
    value_out: *mut f32,
) -> i32 {
    if index >= FMOD_STUDIO_EVENT_PROPERTY_MAX || value_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(value_out, inst.properties[index as usize]);
    FMOD_OK
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_Studio_EventInstance_SetReverbLevel(
    instance: usize,
    index: i32,
    reverb: f32,
) -> i32 {
    studio_event_instance_set_reverb_level_impl(instance, index, reverb)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_Studio_EventInstance_SetReverbLevel(
    instance: usize,
    index: i32,
    _xmm0: f32,
    _xmm1: f32,
    reverb: f32,
) -> i32 {
    studio_event_instance_set_reverb_level_impl(instance, index, reverb)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetReverbLevel(
    instance: usize,
    index: i32,
    reverb_out: *mut f32,
) -> i32 {
    if !(0..4).contains(&index) || reverb_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(reverb_out, inst.reverb_levels[index as usize]);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_KeyOff(instance: usize) -> i32 {
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get_mut(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    if inst.state == FMOD_STUDIO_PLAYBACK_SUSTAINING {
        inst.state = FMOD_STUDIO_PLAYBACK_PLAYING;
    }
    if let Some(cid) = inst.channel_id {
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if let Some(ch) = mixer.channels.get_mut(&cid) {
            ch.loop_count = 0;
            ch.loops_remaining = 0;
            ch.mode = (ch.mode & !(FMOD_LOOP_NORMAL | FMOD_LOOP_BIDI)) | FMOD_LOOP_OFF;
        }
    }
    FMOD_OK
}

// ============================================================================
// FMOD Studio C API exports — Bus
// ============================================================================

pub unsafe extern "win64" fn FMOD_Studio_Bus_IsValid(bus: usize) -> i32 {
    let Ok(studio_arc) = lookup_studio_bus(bus) else {
        return 0;
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if st.buses.contains_key(&bus) { 1 } else { 0 }
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_GetID(bus: usize, id_out: *mut FmodGuid) -> i32 {
    if id_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.buses.get(&bus) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(id_out, b.guid);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_GetPath(
    bus: usize,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let path = {
        let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let Some(b) = st.buses.get(&bus) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        if !b.path.is_empty() {
            b.path.clone()
        } else {
            st.guid_to_path.get(&b.guid).cloned().unwrap_or_default()
        }
    };
    write_studio_path(&path, path_out, size, retrieved_out)
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_Studio_Bus_SetVolume(bus: usize, volume: f32) -> i32 {
    studio_bus_set_volume_impl(bus, volume)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_Studio_Bus_SetVolume(
    bus: usize,
    _xmm0: f32,
    volume: f32,
) -> i32 {
    studio_bus_set_volume_impl(bus, volume)
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_GetVolume(
    bus: usize,
    volume_out: *mut f32,
    finalvolume_out: *mut f32,
) -> i32 {
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.buses.get(&bus) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let vol = b.volume;
    let final_vol = {
        let mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        resolve_group_chain(&mixer.groups, b.channel_group_id).volume
    };
    write_out(volume_out, vol);
    write_out(finalvolume_out, final_vol);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_SetPaused(bus: usize, paused: i32) -> i32 {
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.buses.get_mut(&bus) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    b.paused = paused != 0;
    let cg_id = b.channel_group_id;
    let mut mixer = studio_arc
        .core_sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(cg) = mixer.groups.get_mut(&cg_id) {
        cg.paused = paused != 0;
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_GetPaused(
    bus: usize,
    paused_out: *mut i32,
) -> i32 {
    if paused_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.buses.get(&bus) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(paused_out, if b.paused { 1 } else { 0 });
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_SetMute(bus: usize, mute: i32) -> i32 {
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.buses.get_mut(&bus) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    b.mute = mute != 0;
    let cg_id = b.channel_group_id;
    let mut mixer = studio_arc
        .core_sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(cg) = mixer.groups.get_mut(&cg_id) {
        cg.mute = mute != 0;
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_GetMute(bus: usize, mute_out: *mut i32) -> i32 {
    if mute_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.buses.get(&bus) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(mute_out, if b.mute { 1 } else { 0 });
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_StopAllEvents(bus: usize, mode: u32) -> i32 {
    if mode != FMOD_STUDIO_STOP_ALLOWFADEOUT && mode != FMOD_STUDIO_STOP_IMMEDIATE {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(bus_cg) = st.buses.get(&bus).map(|b| b.channel_group_id) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let mut mixer = studio_arc
        .core_sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let affected_groups: Vec<usize> = mixer
        .groups
        .keys()
        .copied()
        .filter(|&gid| group_has_ancestor(&mixer.groups, gid, bus_cg))
        .collect();
    for ch in mixer.channels.values_mut() {
        if affected_groups.contains(&ch.group_id) {
            ch.stopped = true;
        }
    }
    for inst in st.instances.values_mut() {
        if affected_groups.contains(&inst.channel_group_id) {
            inst.state = FMOD_STUDIO_PLAYBACK_STOPPED;
        }
    }
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_LockChannelGroup(bus: usize) -> i32 {
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.buses.get_mut(&bus) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    b.locked_group = true;
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_UnlockChannelGroup(bus: usize) -> i32 {
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.buses.get_mut(&bus) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    b.locked_group = false;
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_GetChannelGroup(
    bus: usize,
    group_out: *mut usize,
) -> i32 {
    if group_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.buses.get(&bus) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(group_out, b.channel_group_id);
    FMOD_OK
}

// ============================================================================
// FMOD Studio C API exports — VCA
// ============================================================================

pub unsafe extern "win64" fn FMOD_Studio_VCA_IsValid(vca: usize) -> i32 {
    let Ok(studio_arc) = lookup_studio_vca(vca) else {
        return 0;
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if st.vcas.contains_key(&vca) { 1 } else { 0 }
}

pub unsafe extern "win64" fn FMOD_Studio_VCA_GetID(vca: usize, id_out: *mut FmodGuid) -> i32 {
    if id_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_vca(vca) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(v) = st.vcas.get(&vca) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(id_out, v.guid);
    FMOD_OK
}

pub unsafe extern "win64" fn FMOD_Studio_VCA_GetPath(
    vca: usize,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio_vca(vca) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let path = {
        let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let Some(v) = st.vcas.get(&vca) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        if !v.path.is_empty() {
            v.path.clone()
        } else {
            st.guid_to_path.get(&v.guid).cloned().unwrap_or_default()
        }
    };
    write_studio_path(&path, path_out, size, retrieved_out)
}

#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn FMOD_Studio_VCA_SetVolume(vca: usize, volume: f32) -> i32 {
    studio_vca_set_volume_impl(vca, volume)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn FMOD_Studio_VCA_SetVolume(
    vca: usize,
    _xmm0: f32,
    volume: f32,
) -> i32 {
    studio_vca_set_volume_impl(vca, volume)
}

pub unsafe extern "win64" fn FMOD_Studio_VCA_GetVolume(
    vca: usize,
    volume_out: *mut f32,
    finalvolume_out: *mut f32,
) -> i32 {
    let studio_arc = match lookup_studio_vca(vca) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(v) = st.vcas.get(&vca) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_out(volume_out, v.volume);
    write_out(finalvolume_out, v.volume);
    FMOD_OK
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::audio_test_lock;
    use crate::audio::backend::{Backend, DeviceInfo};

    // Architecture-uniform wrappers for exports with float parameters.
    unsafe fn call_set_channel_volume(ch: usize, vol: f32) -> i32 {
        #[cfg(target_arch = "aarch64")]
        {
            FMOD_Channel_SetVolume(ch, 0.0, vol)
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            FMOD_Channel_SetVolume(ch, vol)
        }
    }

    unsafe fn call_set_channel_pitch(ch: usize, pitch: f32) -> i32 {
        #[cfg(target_arch = "aarch64")]
        {
            FMOD_Channel_SetPitch(ch, 0.0, pitch)
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            FMOD_Channel_SetPitch(ch, pitch)
        }
    }

    unsafe fn call_set_channel_pan(ch: usize, pan: f32) -> i32 {
        #[cfg(target_arch = "aarch64")]
        {
            FMOD_Channel_SetPan(ch, 0.0, pan)
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            FMOD_Channel_SetPan(ch, pan)
        }
    }

    unsafe fn call_set_channel_3d_minmax(ch: usize, min: f32, max: f32) -> i32 {
        #[cfg(target_arch = "aarch64")]
        {
            FMOD_Channel_Set3DMinMaxDistance(ch, 0.0, min, max)
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            FMOD_Channel_Set3DMinMaxDistance(ch, min, max)
        }
    }

    unsafe fn call_set_group_volume(g: usize, vol: f32) -> i32 {
        #[cfg(target_arch = "aarch64")]
        {
            FMOD_ChannelGroup_SetVolume(g, 0.0, vol)
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            FMOD_ChannelGroup_SetVolume(g, vol)
        }
    }

    unsafe fn call_set_3d_settings(sys: usize, doppler: f32, dist: f32, rolloff: f32) -> i32 {
        #[cfg(target_arch = "aarch64")]
        {
            FMOD_System_Set3DSettings(sys, 0.0, doppler, dist, rolloff)
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            FMOD_System_Set3DSettings(sys, doppler, dist, rolloff)
        }
    }

    fn build_wav(audio_format: u16, channels: u16, sample_rate: u32, bits: u16, pcm: &[u8]) -> Vec<u8> {
        let block_align = channels * (bits / 8);
        let byte_rate = sample_rate * block_align as u32;
        let data_len = pcm.len() as u32;
        let riff_size = 4 + (8 + 16) + (8 + data_len + (data_len & 1));
        let mut out = Vec::with_capacity((riff_size + 8) as usize);
        out.extend_from_slice(b"RIFF");
        out.extend_from_slice(&riff_size.to_le_bytes());
        out.extend_from_slice(b"WAVE");
        out.extend_from_slice(b"fmt ");
        out.extend_from_slice(&16u32.to_le_bytes());
        out.extend_from_slice(&audio_format.to_le_bytes());
        out.extend_from_slice(&channels.to_le_bytes());
        out.extend_from_slice(&sample_rate.to_le_bytes());
        out.extend_from_slice(&byte_rate.to_le_bytes());
        out.extend_from_slice(&block_align.to_le_bytes());
        out.extend_from_slice(&bits.to_le_bytes());
        out.extend_from_slice(b"data");
        out.extend_from_slice(&data_len.to_le_bytes());
        out.extend_from_slice(pcm);
        if (pcm.len() & 1) != 0 {
            out.push(0);
        }
        out
    }

    unsafe fn create_nrt_system() -> usize {
        let mut sys = 0usize;
        assert_eq!(FMOD_System_Create(&mut sys, FMOD_VERSION), FMOD_OK);
        assert_ne!(sys, 0);
        assert_eq!(FMOD_System_SetOutput(sys, FMOD_OUTPUTTYPE_NOSOUND_NRT), FMOD_OK);
        assert_eq!(FMOD_System_Init(sys, 32, 0, 0), FMOD_OK);
        sys
    }

    #[test]
    fn fmod_creates_handle_and_version() {
        unsafe {
            let mut sys = 0usize;
            assert_eq!(FMOD_System_Create(&mut sys, 0), FMOD_OK);
            assert_ne!(sys, 0);
            let mut ver = 0u32;
            assert_eq!(FMOD_System_GetVersion(sys, &mut ver), FMOD_OK);
            assert_eq!(ver, 0x0002_0200);
            assert_eq!(FMOD_System_Release(sys), FMOD_OK);
        }
    }

    #[test]
    fn fmod_enumerates_real_backend_drivers() {
        let _g = audio_test_lock();
        backend::force_output_devices(Some(vec![
            DeviceInfo {
                id: 42,
                uid: "ca-uid-studio-display".into(),
                name: "Studio Display Speakers".into(),
                rate: 48_000,
                channels: 6,
                io_frames: 512,
                latency_frames: 512,
                headphones: false,
                is_default: true,
            },
            DeviceInfo {
                id: 99,
                uid: "ca-uid-airpods".into(),
                name: "AirPods Pro".into(),
                rate: 44_100,
                channels: 2,
                io_frames: 256,
                latency_frames: 512,
                headphones: true,
                is_default: false,
            },
        ]));

        unsafe {
            let mut sys = 0usize;
            assert_eq!(FMOD_System_Create(&mut sys, FMOD_VERSION), FMOD_OK);

            let mut count = 0i32;
            assert_eq!(FMOD_System_GetNumDrivers(sys, &mut count), FMOD_OK);
            assert_eq!(count, 2);

            let mut name_buf = [0u8; 64];
            let mut guid = [0u8; 16];
            let mut rate = 0i32;
            let mut mode = 0u32;
            let mut ch = 0i32;

            assert_eq!(
                FMOD_System_GetDriverInfo(
                    sys,
                    0,
                    name_buf.as_mut_ptr(),
                    name_buf.len() as i32,
                    guid.as_mut_ptr() as usize,
                    &mut rate,
                    &mut mode,
                    &mut ch,
                ),
                FMOD_OK
            );
            assert_eq!(crate::strings::from_ansi(name_buf.as_ptr()), "Studio Display Speakers");
            assert_eq!(rate, 48_000);
            assert_eq!(mode, FMOD_SPEAKERMODE_5POINT1);
            assert_eq!(ch, 6);
            assert_ne!(guid, [0u8; 16]);

            assert_eq!(
                FMOD_System_GetDriverInfo(
                    sys,
                    1,
                    name_buf.as_mut_ptr(),
                    name_buf.len() as i32,
                    0,
                    &mut rate,
                    &mut mode,
                    &mut ch,
                ),
                FMOD_OK
            );
            assert_eq!(crate::strings::from_ansi(name_buf.as_ptr()), "AirPods Pro");
            assert_eq!(rate, 44_100);
            assert_eq!(mode, FMOD_SPEAKERMODE_STEREO);
            assert_eq!(ch, 2);

            // Out-of-range driver index fails cleanly.
            assert_eq!(
                FMOD_System_GetDriverInfo(sys, 2, name_buf.as_mut_ptr(), 64, 0, &mut rate, &mut mode, &mut ch),
                FMOD_ERR_INVALID_PARAM
            );

            assert_eq!(FMOD_System_Release(sys), FMOD_OK);
        }
    }

    #[test]
    fn fmod_wav_formats_memory_and_file_mix_exact_waveforms() {
        unsafe {
            let sys = create_nrt_system();

            // 1) 16-bit stereo WAV from memory
            let mut pcm16 = Vec::new();
            for _ in 0..16 {
                pcm16.extend_from_slice(&16384i16.to_le_bytes()); // L = +0.5
                pcm16.extend_from_slice(&(-8192i16).to_le_bytes()); // R = -0.25
            }
            let wav16 = build_wav(1, 2, 48_000, 16, &pcm16);
            let ex16 = FmodCreateSoundExInfoHeader {
                cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
                length: wav16.len() as u32,
                ..Default::default()
            };
            let mut snd16 = 0usize;
            assert_eq!(
                FMOD_System_CreateSound(
                    sys,
                    wav16.as_ptr(),
                    FMOD_OPENMEMORY,
                    &ex16 as *const _ as usize,
                    &mut snd16,
                ),
                FMOD_OK
            );

            let (mut stype, mut fmt, mut chs, mut bits) = (0u32, 0u32, 0i32, 0i32);
            assert_eq!(FMOD_Sound_GetFormat(snd16, &mut stype, &mut fmt, &mut chs, &mut bits), FMOD_OK);
            assert_eq!((stype, fmt, chs, bits), (FMOD_SOUND_TYPE_WAV, FMOD_SOUND_FORMAT_PCM16, 2, 16));

            let (mut len_pcm, mut len_bytes, mut len_ms) = (0u32, 0u32, 0u32);
            assert_eq!(FMOD_Sound_GetLength(snd16, &mut len_pcm, FMOD_TIMEUNIT_PCM), FMOD_OK);
            assert_eq!(FMOD_Sound_GetLength(snd16, &mut len_bytes, FMOD_TIMEUNIT_PCMBYTES), FMOD_OK);
            assert_eq!(FMOD_Sound_GetLength(snd16, &mut len_ms, FMOD_TIMEUNIT_MS), FMOD_OK);
            assert_eq!(len_pcm, 16);
            assert_eq!(len_bytes, 64);
            assert_eq!(len_ms, 0);

            let (mut def_freq, mut def_prio) = (0.0f32, 0i32);
            assert_eq!(FMOD_Sound_GetDefaults(snd16, &mut def_freq, &mut def_prio), FMOD_OK);
            assert_eq!(def_freq, 48_000.0);
            assert_eq!(def_prio, 128);

            let mut ch16 = 0usize;
            assert_eq!(FMOD_System_PlaySound(sys, snd16, 0, 0, &mut ch16), FMOD_OK);

            let mut mix = [0.0f32; 16 * 2];
            assert_eq!(mix_system_frames(sys, &mut mix), Ok(16));
            for f in 0..16 {
                assert!((mix[f * 2] - 0.5).abs() < 1e-4, "L[{f}]={}", mix[f * 2]);
                assert!((mix[f * 2 + 1] - (-0.25)).abs() < 1e-4, "R[{f}]={}", mix[f * 2 + 1]);
            }
            // One-shot sound finished after 16 frames.
            let mut playing = 1i32;
            assert_eq!(FMOD_Channel_IsPlaying(ch16, &mut playing), FMOD_OK);
            assert_eq!(playing, 0);

            // 2) 8-bit, 24-bit, 32-bit PCM, and 32-bit float WAVs (including loading from file!)
            let wav8 = build_wav(1, 1, 48_000, 8, &[192u8; 4]); // (192-128)/128 = +0.5
            let mut pcm24 = Vec::new();
            for _ in 0..4 {
                pcm24.extend_from_slice(&[0x00, 0x00, 0xC0]); // -0.5 in signed 24-bit LE
            }
            let wav24 = build_wav(1, 1, 48_000, 24, &pcm24);
            let mut pcm32 = Vec::new();
            for _ in 0..4 {
                pcm32.extend_from_slice(&(i32::MAX / 2 + 1).to_le_bytes()); // +0.5
            }
            let wav32 = build_wav(1, 1, 48_000, 32, &pcm32);
            let mut pcmf = Vec::new();
            for _ in 0..4 {
                pcmf.extend_from_slice(&0.75f32.to_le_bytes());
            }
            let wavf = build_wav(3, 1, 48_000, 32, &pcmf);

            for (wav_bytes, expected_fmt, expected_amp) in [
                (&wav8, FMOD_SOUND_FORMAT_PCM8, 0.5f32),
                (&wav24, FMOD_SOUND_FORMAT_PCM24, -0.5f32),
                (&wav32, FMOD_SOUND_FORMAT_PCM32, 0.5f32),
                (&wavf, FMOD_SOUND_FORMAT_PCMFLOAT, 0.75f32),
            ] {
                let ex = FmodCreateSoundExInfoHeader {
                    cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
                    length: wav_bytes.len() as u32,
                    ..Default::default()
                };
                let mut snd = 0usize;
                assert_eq!(
                    FMOD_System_CreateSound(
                        sys,
                        wav_bytes.as_ptr(),
                        FMOD_OPENMEMORY,
                        &ex as *const _ as usize,
                        &mut snd,
                    ),
                    FMOD_OK
                );
                let mut f_out = 0u32;
                assert_eq!(FMOD_Sound_GetFormat(snd, std::ptr::null_mut(), &mut f_out, std::ptr::null_mut(), std::ptr::null_mut()), FMOD_OK);
                assert_eq!(f_out, expected_fmt);

                let mut ch = 0usize;
                assert_eq!(FMOD_System_PlaySound(sys, snd, 0, 0, &mut ch), FMOD_OK);
                let mut out = [0.0f32; 4 * 2];
                assert_eq!(mix_system_frames(sys, &mut out), Ok(4));
                assert!((out[0] - expected_amp).abs() < 1e-3, "fmt={expected_fmt} got={}", out[0]);
                assert!((out[1] - expected_amp).abs() < 1e-3);
                assert_eq!(FMOD_Sound_Release(snd), FMOD_OK);
            }

            // 3) File-based WAV via Windows path mount (`crate::path`)
            let tmp_dir = std::env::temp_dir().join("win32_rt_fmod_wav_test");
            let _ = std::fs::create_dir_all(&tmp_dir);
            let file_path = tmp_dir.join("tone.wav");
            std::fs::write(&file_path, &wavf).unwrap();
            crate::path::add_mount("F:\\FmodTest", &tmp_dir);

            let win_path = b"F:\\FmodTest\\tone.wav\0";
            let mut file_snd = 0usize;
            assert_eq!(
                FMOD_System_CreateStream(sys, win_path.as_ptr(), FMOD_DEFAULT, 0, &mut file_snd),
                FMOD_OK
            );
            let mut file_ch = 0usize;
            assert_eq!(FMOD_System_PlaySound(sys, file_snd, 0, 0, &mut file_ch), FMOD_OK);
            let mut out = [0.0f32; 4 * 2];
            assert_eq!(mix_system_frames(sys, &mut out), Ok(4));
            assert!((out[0] - 0.75).abs() < 1e-4);

            let _ = std::fs::remove_file(&file_path);
            let _ = std::fs::remove_dir(&tmp_dir);
            assert_eq!(FMOD_Sound_Release(file_snd), FMOD_OK);
            assert_eq!(FMOD_Sound_Release(snd16), FMOD_OK);
            assert_eq!(FMOD_System_Release(sys), FMOD_OK);
        }
    }

    #[test]
    fn fmod_raw_pcm_channel_group_hierarchy_volume_mute_and_lock() {
        unsafe {
            let sys = create_nrt_system();

            // Create raw float mono sound with constant +0.8 amplitude.
            let samples = [0.8f32; 64];
            let raw_bytes = std::slice::from_raw_parts(
                samples.as_ptr() as *const u8,
                std::mem::size_of_val(&samples),
            );
            let ex = FmodCreateSoundExInfoHeader {
                cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
                length: raw_bytes.len() as u32,
                fileoffset: 0,
                numchannels: 1,
                defaultfrequency: 48_000,
                format: FMOD_SOUND_FORMAT_PCMFLOAT,
            };
            let mut snd = 0usize;
            assert_eq!(
                FMOD_System_CreateSound(
                    sys,
                    raw_bytes.as_ptr(),
                    FMOD_OPENMEMORY | FMOD_OPENRAW | FMOD_LOOP_NORMAL,
                    &ex as *const _ as usize,
                    &mut snd,
                ),
                FMOD_OK
            );

            // Build hierarchy: child_grp -> parent_grp -> master
            let mut master = 0usize;
            assert_eq!(FMOD_System_GetMasterChannelGroup(sys, &mut master), FMOD_OK);
            let mut parent_grp = 0usize;
            let mut child_grp = 0usize;
            assert_eq!(FMOD_System_CreateChannelGroup(sys, b"SFX\0".as_ptr(), &mut parent_grp), FMOD_OK);
            assert_eq!(FMOD_System_CreateChannelGroup(sys, b"Footsteps\0".as_ptr(), &mut child_grp), FMOD_OK);
            assert_eq!(FMOD_ChannelGroup_AddGroup(parent_grp, child_grp, 0, std::ptr::null_mut()), FMOD_OK);

            // Cycle detection must reject reparenting parent under its own child.
            assert_eq!(
                FMOD_ChannelGroup_AddGroup(child_grp, parent_grp, 0, std::ptr::null_mut()),
                FMOD_ERR_INVALID_PARAM
            );

            let mut ch = 0usize;
            assert_eq!(FMOD_System_PlaySound(sys, snd, child_grp, 0, &mut ch), FMOD_OK);

            // Effective gain = 0.8 (sample) * 0.5 (ch) * 0.5 (child) * 0.5 (parent) * 0.5 (master) = 0.05
            assert_eq!(call_set_channel_volume(ch, 0.5), FMOD_OK);
            assert_eq!(call_set_group_volume(child_grp, 0.5), FMOD_OK);
            assert_eq!(call_set_group_volume(parent_grp, 0.5), FMOD_OK);
            assert_eq!(call_set_group_volume(master, 0.5), FMOD_OK);

            let mut out = [0.0f32; 8 * 2];
            assert_eq!(mix_system_frames(sys, &mut out), Ok(8));
            assert!((out[0] - 0.05).abs() < 1e-4, "got {}", out[0]);
            assert!((out[1] - 0.05).abs() < 1e-4);

            // Mute parent group -> silence, while play position still advances.
            assert_eq!(FMOD_ChannelGroup_SetMute(parent_grp, 1), FMOD_OK);
            let mut muted = 0i32;
            assert_eq!(FMOD_ChannelGroup_GetMute(parent_grp, &mut muted), FMOD_OK);
            assert_eq!(muted, 1);

            let mut pos_before = 0u32;
            assert_eq!(FMOD_Channel_GetPosition(ch, &mut pos_before, FMOD_TIMEUNIT_PCM), FMOD_OK);
            assert_eq!(mix_system_frames(sys, &mut out), Ok(8));
            assert!(out.iter().all(|&v| v == 0.0));
            let mut pos_after = 0u32;
            assert_eq!(FMOD_Channel_GetPosition(ch, &mut pos_after, FMOD_TIMEUNIT_PCM), FMOD_OK);
            assert_eq!(pos_after, pos_before + 8);

            // Pause parent group -> silence AND play position must NOT advance.
            assert_eq!(FMOD_ChannelGroup_SetMute(parent_grp, 0), FMOD_OK);
            assert_eq!(FMOD_ChannelGroup_SetPaused(parent_grp, 1), FMOD_OK);
            let mut grp_paused = 0i32;
            assert_eq!(FMOD_ChannelGroup_GetPaused(parent_grp, &mut grp_paused), FMOD_OK);
            assert_eq!(grp_paused, 1);
            assert_eq!(mix_system_frames(sys, &mut out), Ok(8));
            assert!(out.iter().all(|&v| v == 0.0));
            let mut pos_paused = 0u32;
            assert_eq!(FMOD_Channel_GetPosition(ch, &mut pos_paused, FMOD_TIMEUNIT_PCM), FMOD_OK);
            assert_eq!(pos_paused, pos_after);
            assert_eq!(FMOD_ChannelGroup_SetPaused(parent_grp, 0), FMOD_OK);

            // Reassign channel to parent_grp directly so muting child_grp does not silence it.
            assert_eq!(FMOD_Channel_SetChannelGroup(ch, parent_grp), FMOD_OK);
            assert_eq!(FMOD_ChannelGroup_SetMute(child_grp, 1), FMOD_OK);
            assert_eq!(call_set_channel_volume(ch, 1.0), FMOD_OK);
            assert_eq!(call_set_group_volume(child_grp, 1.0), FMOD_OK);
            assert_eq!(call_set_group_volume(parent_grp, 1.0), FMOD_OK);
            assert_eq!(call_set_group_volume(master, 1.0), FMOD_OK);

            let (mut ch_vol, mut grp_vol) = (0.0f32, 0.0f32);
            assert_eq!(FMOD_Channel_GetVolume(ch, &mut ch_vol), FMOD_OK);
            assert_eq!(FMOD_ChannelGroup_GetVolume(parent_grp, &mut grp_vol), FMOD_OK);
            assert!((ch_vol - 1.0).abs() < 1e-5 && (grp_vol - 1.0).abs() < 1e-5);

            assert_eq!(mix_system_frames(sys, &mut out), Ok(8));
            assert!((out[0] - 0.8).abs() < 1e-4);
            assert_eq!(FMOD_ChannelGroup_SetMute(child_grp, 0), FMOD_OK);
            assert_eq!(FMOD_Channel_SetChannelGroup(ch, child_grp), FMOD_OK);

            // Test Sound_Lock / Sound_Unlock mutation.
            let (mut p1, mut p2): (*mut u8, *mut u8) = (std::ptr::null_mut(), std::ptr::null_mut());
            let (mut l1, mut l2) = (0u32, 0u32);
            assert_eq!(
                FMOD_Sound_Lock(snd, 0, raw_bytes.len() as u32, &mut p1, &mut p2, &mut l1, &mut l2),
                FMOD_OK
            );
            assert!(!p1.is_null());
            assert_eq!(l1 as usize, raw_bytes.len());
            assert_eq!(l2, 0);
            for i in 0..64 {
                let bytes = (-0.4f32).to_le_bytes();
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), p1.add(i * 4), 4);
            }
            assert_eq!(FMOD_Sound_Unlock(snd, p1, p2, l1, l2), FMOD_OK);

            assert_eq!(mix_system_frames(sys, &mut out), Ok(8));
            assert!((out[0] - (-0.4)).abs() < 1e-4, "post-unlock got {}", out[0]);

            // Stopping parent group stops channels in descendant child_grp!
            assert_eq!(FMOD_ChannelGroup_Stop(parent_grp), FMOD_OK);
            let mut playing = 1i32;
            assert_eq!(FMOD_Channel_IsPlaying(ch, &mut playing), FMOD_OK);
            assert_eq!(playing, 0);

            // Releasing child_grp succeeds; releasing master fails cleanly.
            assert_eq!(FMOD_ChannelGroup_Release(child_grp), FMOD_OK);
            assert_eq!(FMOD_ChannelGroup_Release(master), FMOD_ERR_INVALID_PARAM);
            assert_eq!(FMOD_System_Release(sys), FMOD_OK);
        }
    }

    #[test]
    fn fmod_pause_stop_seek_looping_and_pitch_resampling() {
        unsafe {
            let sys = create_nrt_system();

            // 8-sample ramp [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8] at 48 kHz mono.
            let ramp: [f32; 8] = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
            let ex = FmodCreateSoundExInfoHeader {
                cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
                length: std::mem::size_of_val(&ramp) as u32,
                numchannels: 1,
                defaultfrequency: 48_000,
                format: FMOD_SOUND_FORMAT_PCMFLOAT,
                ..Default::default()
            };
            let mut snd = 0usize;
            assert_eq!(
                FMOD_System_CreateSound(
                    sys,
                    ramp.as_ptr() as *const u8,
                    FMOD_OPENMEMORY | FMOD_OPENRAW,
                    &ex as *const _ as usize,
                    &mut snd,
                ),
                FMOD_OK
            );

            // Start paused: must output silence and stay at position 0.
            let mut ch = 0usize;
            assert_eq!(FMOD_System_PlaySound(sys, snd, 0, 1, &mut ch), FMOD_OK);
            let mut paused = 0i32;
            assert_eq!(FMOD_Channel_GetPaused(ch, &mut paused), FMOD_OK);
            assert_eq!(paused, 1);

            let mut out = [0.0f32; 4 * 2];
            assert_eq!(mix_system_frames(sys, &mut out), Ok(4));
            assert!(out.iter().all(|&v| v == 0.0));
            let mut pos = 99u32;
            assert_eq!(FMOD_Channel_GetPosition(ch, &mut pos, FMOD_TIMEUNIT_PCM), FMOD_OK);
            assert_eq!(pos, 0);

            // Loop once (plays twice = 16 frames total), unpause, pull 12 frames.
            assert_eq!(FMOD_Channel_SetLoopCount(ch, 1), FMOD_OK);
            assert_eq!(FMOD_Channel_SetPaused(ch, 0), FMOD_OK);
            let mut out12 = [0.0f32; 12 * 2];
            assert_eq!(mix_system_frames(sys, &mut out12), Ok(12));
            assert!((out12[0] - 0.1).abs() < 1e-4);
            assert!((out12[7 * 2] - 0.8).abs() < 1e-4);
            // Frame 8 wrapped back to ramp[0] = 0.1!
            assert!((out12[8 * 2] - 0.1).abs() < 1e-4);
            assert!((out12[11 * 2] - 0.4).abs() < 1e-4);

            // Still playing (4 frames left on second pass), then finishes after 4 more frames.
            let mut playing = 0i32;
            assert_eq!(FMOD_Channel_IsPlaying(ch, &mut playing), FMOD_OK);
            assert_eq!(playing, 1);
            assert_eq!(mix_system_frames(sys, &mut out), Ok(4));
            assert!((out[3 * 2] - 0.8).abs() < 1e-4);
            assert_eq!(FMOD_Channel_IsPlaying(ch, &mut playing), FMOD_OK);
            assert_eq!(playing, 0);

            // Linear pitch resampling at 0.5x pitch: frames advance 0.0, 0.5, 1.0, 1.5 -> 0.1, 0.15, 0.2, 0.25
            let mut ch2 = 0usize;
            assert_eq!(FMOD_System_PlaySound(sys, snd, 0, 1, &mut ch2), FMOD_OK);
            assert_eq!(call_set_channel_pitch(ch2, 0.5), FMOD_OK);
            let mut p_out = 0.0f32;
            assert_eq!(FMOD_Channel_GetPitch(ch2, &mut p_out), FMOD_OK);
            assert!((p_out - 0.5).abs() < 1e-5);
            assert_eq!(FMOD_Channel_SetPosition(ch2, 2, FMOD_TIMEUNIT_PCM), FMOD_OK);
            assert_eq!(FMOD_Channel_SetPaused(ch2, 0), FMOD_OK);
            assert_eq!(mix_system_frames(sys, &mut out), Ok(4));
            // Starting at frame 2 (0.3), step 0.5 -> 0.30, 0.35, 0.40, 0.45
            assert!((out[0] - 0.30).abs() < 1e-4, "got {}", out[0]);
            assert!((out[2] - 0.35).abs() < 1e-4, "got {}", out[2]);
            assert!((out[4] - 0.40).abs() < 1e-4, "got {}", out[4]);
            assert!((out[6] - 0.45).abs() < 1e-4, "got {}", out[6]);

            // Explicit Stop halts immediately.
            assert_eq!(FMOD_Channel_Stop(ch2), FMOD_OK);
            assert_eq!(FMOD_Channel_IsPlaying(ch2, &mut playing), FMOD_OK);
            assert_eq!(playing, 0);
            assert_eq!(mix_system_frames(sys, &mut out), Ok(4));
            assert!(out.iter().all(|&v| v == 0.0));

            assert_eq!(FMOD_System_Release(sys), FMOD_OK);
        }
    }

    #[test]
    fn fmod_2d_pan_and_3d_panning_plus_distance_attenuation() {
        unsafe {
            let sys = create_nrt_system();
            assert_eq!(call_set_3d_settings(sys, 1.0, 1.0, 1.0), FMOD_OK);

            let listener_pos = FmodVector { x: 0.0, y: 0.0, z: 0.0 };
            let listener_vel = FmodVector { x: 0.0, y: 0.0, z: 0.0 };
            let listener_fwd = FmodVector { x: 0.0, y: 0.0, z: 1.0 };
            let listener_up = FmodVector { x: 0.0, y: 1.0, z: 0.0 };
            assert_eq!(
                FMOD_System_Set3DListenerAttributes(
                    sys,
                    0,
                    &listener_pos,
                    &listener_vel,
                    &listener_fwd,
                    &listener_up,
                ),
                FMOD_OK
            );

            let samples = [0.8f32; 64];
            let ex = FmodCreateSoundExInfoHeader {
                cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
                length: std::mem::size_of_val(&samples) as u32,
                numchannels: 1,
                defaultfrequency: 48_000,
                format: FMOD_SOUND_FORMAT_PCMFLOAT,
                ..Default::default()
            };

            // 2D panning test
            let mut snd2d = 0usize;
            assert_eq!(
                FMOD_System_CreateSound(
                    sys,
                    samples.as_ptr() as *const u8,
                    FMOD_OPENMEMORY | FMOD_OPENRAW | FMOD_LOOP_NORMAL | FMOD_2D,
                    &ex as *const _ as usize,
                    &mut snd2d,
                ),
                FMOD_OK
            );
            let mut ch2d = 0usize;
            assert_eq!(FMOD_System_PlaySound(sys, snd2d, 0, 0, &mut ch2d), FMOD_OK);
            assert_eq!(call_set_channel_pan(ch2d, -1.0), FMOD_OK);
            let mut out = [0.0f32; 4 * 2];
            assert_eq!(mix_system_frames(sys, &mut out), Ok(4));
            assert!((out[0] - 0.8).abs() < 1e-4);
            assert!(out[1].abs() < 1e-4);
            assert_eq!(FMOD_Channel_Stop(ch2d), FMOD_OK);

            // 3D panning + distance attenuation test
            let mut snd3d = 0usize;
            assert_eq!(
                FMOD_System_CreateSound(
                    sys,
                    samples.as_ptr() as *const u8,
                    FMOD_OPENMEMORY | FMOD_OPENRAW | FMOD_LOOP_NORMAL | FMOD_3D,
                    &ex as *const _ as usize,
                    &mut snd3d,
                ),
                FMOD_OK
            );
            let mut ch3d = 0usize;
            assert_eq!(FMOD_System_PlaySound(sys, snd3d, 0, 0, &mut ch3d), FMOD_OK);
            assert_eq!(call_set_channel_3d_minmax(ch3d, 1.0, 100.0), FMOD_OK);

            // Source 2m to listener's right (+X): atten = 1/2 = 0.5, pan = +1.0 -> L = 0, R = 0.4
            let pos_right = FmodVector { x: 2.0, y: 0.0, z: 0.0 };
            assert_eq!(FMOD_Channel_Set3DAttributes(ch3d, &pos_right, std::ptr::null()), FMOD_OK);
            assert_eq!(mix_system_frames(sys, &mut out), Ok(4));
            assert!(out[0].abs() < 1e-4, "expected L=0, got {}", out[0]);
            assert!((out[1] - 0.4).abs() < 1e-4, "expected R=0.4, got {}", out[1]);

            // Source 4m to listener's left (-X): atten = 1/4 = 0.25, pan = -1.0 -> L = 0.2, R = 0
            let pos_left = FmodVector { x: -4.0, y: 0.0, z: 0.0 };
            assert_eq!(FMOD_Channel_Set3DAttributes(ch3d, &pos_left, std::ptr::null()), FMOD_OK);
            assert_eq!(mix_system_frames(sys, &mut out), Ok(4));
            assert!((out[0] - 0.2).abs() < 1e-4, "expected L=0.2, got {}", out[0]);
            assert!(out[1].abs() < 1e-4, "expected R=0, got {}", out[1]);

            assert_eq!(FMOD_System_Release(sys), FMOD_OK);
        }
    }

    #[test]
    fn fmod_realtime_mixer_thread_pumps_audio_backend() {
        let _g = audio_test_lock();
        backend::force_backend(Some(Backend::Null));

        unsafe {
            let mut sys = 0usize;
            assert_eq!(FMOD_System_Create(&mut sys, FMOD_VERSION), FMOD_OK);
            assert_eq!(FMOD_System_Init(sys, 16, 0, 0), FMOD_OK);

            let samples = vec![0.25f32; 48_000];
            let ex = FmodCreateSoundExInfoHeader {
                cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
                length: (samples.len() * std::mem::size_of::<f32>()) as u32,
                numchannels: 1,
                defaultfrequency: 48_000,
                format: FMOD_SOUND_FORMAT_PCMFLOAT,
                ..Default::default()
            };
            let mut snd = 0usize;
            assert_eq!(
                FMOD_System_CreateSound(
                    sys,
                    samples.as_ptr() as *const u8,
                    FMOD_OPENMEMORY | FMOD_OPENRAW | FMOD_LOOP_NORMAL,
                    &ex as *const _ as usize,
                    &mut snd,
                ),
                FMOD_OK
            );
            let mut ch = 0usize;
            assert_eq!(FMOD_System_PlaySound(sys, snd, 0, 0, &mut ch), FMOD_OK);

            // Poll briefly until the real mixer thread pushes blocks into the backend ring.
            let mut mixed = 0u64;
            let mut pos = 0u32;
            for _ in 0..50 {
                std::thread::sleep(Duration::from_millis(10));
                mixed = lookup_system(sys).unwrap().mixed_frames.load(Ordering::Relaxed);
                assert_eq!(FMOD_Channel_GetPosition(ch, &mut pos, FMOD_TIMEUNIT_PCM), FMOD_OK);
                if mixed >= MIX_BLOCK_FRAMES as u64 && pos > 0 {
                    break;
                }
            }
            assert!(mixed >= MIX_BLOCK_FRAMES as u64, "mixer thread did not pump frames (mixed={mixed})");
            assert!(pos > 0, "channel play position did not advance on mixer thread");

            assert_eq!(FMOD_System_Close(sys), FMOD_OK);
            assert_eq!(FMOD_System_Release(sys), FMOD_OK);
        }
    }

    #[test]
    fn fmod_honest_errors_on_unsupported_containers_and_studio_banks() {
        unsafe {
            let mut studio = 0usize;
            assert_eq!(FMOD_Studio_System_Create(&mut studio, FMOD_VERSION), FMOD_OK);
            assert_ne!(studio, 0);

            let mut core1 = 0usize;
            let mut core2 = 0usize;
            assert_eq!(FMOD_Studio_System_GetCoreSystem(studio, &mut core1), FMOD_OK);
            assert_eq!(FMOD_Studio_System_GetCoreSystem(studio, &mut core2), FMOD_OK);
            assert_ne!(core1, 0);
            assert_eq!(core1, core2, "GetCoreSystem must return the same linked Core handle");

            // Configure linked Core system NRT and initialize via Studio.
            assert_eq!(FMOD_System_SetOutput(core1, FMOD_OUTPUTTYPE_NOSOUND_NRT), FMOD_OK);
            assert_eq!(FMOD_Studio_System_Initialize(studio, 32, 0, 0, 0), FMOD_OK);
            assert_eq!(FMOD_Studio_System_Update(studio), FMOD_OK);

            // Unsupported proprietary FSB5 bank passed to Core CreateSound -> FMOD_ERR_FORMAT (19)
            let fsb5_blob = b"FSB5\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
            let ex = FmodCreateSoundExInfoHeader {
                cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
                length: fsb5_blob.len() as u32,
                ..Default::default()
            };
            let mut bad_snd = 0xDEADusize;
            assert_eq!(
                FMOD_System_CreateSound(
                    core1,
                    fsb5_blob.as_ptr(),
                    FMOD_OPENMEMORY,
                    &ex as *const _ as usize,
                    &mut bad_snd,
                ),
                FMOD_ERR_FORMAT
            );
            assert_eq!(bad_snd, 0);

            // Non-existent Studio .bank file -> FMOD_ERR_FILE_NOTFOUND (18), null handle
            let mut bank = 0xDEADusize;
            assert_eq!(
                FMOD_Studio_System_LoadBankFile(studio, b"Master.bank\0".as_ptr(), 0, &mut bank),
                FMOD_ERR_FILE_NOTFOUND
            );
            assert_eq!(bank, 0);

            // Corrupt/truncated buffer -> FMOD_ERR_FORMAT (19), null handle
            bank = 0xDEADusize;
            assert_eq!(
                FMOD_Studio_System_LoadBankMemory(
                    studio,
                    fsb5_blob.as_ptr(),
                    fsb5_blob.len() as i32,
                    FMOD_STUDIO_LOAD_MEMORY,
                    FMOD_STUDIO_LOAD_BANK_NORMAL,
                    &mut bank,
                ),
                FMOD_ERR_FORMAT
            );
            assert_eq!(bank, 0);

            // Missing Studio event -> FMOD_ERR_EVENT_NOTFOUND (74), null handle
            let mut ev = 0xDEADusize;
            assert_eq!(
                FMOD_Studio_System_GetEvent(studio, b"event:/SFX/Explosion\0".as_ptr(), &mut ev),
                FMOD_ERR_EVENT_NOTFOUND
            );
            assert_eq!(ev, 0);

            // Creating/controlling non-existent Studio event instances fails honestly.
            let mut inst = 0xDEADusize;
            assert_eq!(
                FMOD_Studio_EventDescription_CreateInstance(0, &mut inst),
                FMOD_ERR_INVALID_HANDLE
            );
            assert_eq!(inst, 0);
            assert_eq!(FMOD_Studio_EventInstance_Start(0), FMOD_ERR_INVALID_HANDLE);
            assert_eq!(FMOD_Studio_EventInstance_Stop(0, 0), FMOD_ERR_INVALID_HANDLE);
            assert_eq!(FMOD_Studio_EventInstance_Release(0), FMOD_ERR_INVALID_HANDLE);

            // Releasing Studio also releases its linked Core system.
            assert_eq!(FMOD_Studio_System_Release(studio), FMOD_OK);
            assert_eq!(FMOD_System_Update(core1), FMOD_ERR_INVALID_HANDLE);
        }
    }

    // ------------------------------------------------------------------------
    // Studio float-ABI test wrappers & synthetic FSB5 / RIFF FEV bank builders
    // ------------------------------------------------------------------------

    unsafe fn call_studio_inst_set_volume(inst: usize, vol: f32) -> i32 {
        #[cfg(target_arch = "aarch64")]
        {
            FMOD_Studio_EventInstance_SetVolume(inst, 0.0, vol)
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            FMOD_Studio_EventInstance_SetVolume(inst, vol)
        }
    }

    unsafe fn call_studio_inst_set_pitch(inst: usize, pitch: f32) -> i32 {
        #[cfg(target_arch = "aarch64")]
        {
            FMOD_Studio_EventInstance_SetPitch(inst, 0.0, pitch)
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            FMOD_Studio_EventInstance_SetPitch(inst, pitch)
        }
    }

    unsafe fn call_studio_inst_set_param_name(
        inst: usize,
        name: *const u8,
        val: f32,
        ignore: i32,
    ) -> i32 {
        #[cfg(target_arch = "aarch64")]
        {
            FMOD_Studio_EventInstance_SetParameterByName(inst, name, 0, ignore, 0.0, 0.0, val)
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            FMOD_Studio_EventInstance_SetParameterByName(inst, name, val, ignore)
        }
    }

    unsafe fn call_studio_inst_set_param_id(
        inst: usize,
        id: FmodStudioParameterId,
        val: f32,
        ignore: i32,
    ) -> i32 {
        #[cfg(target_arch = "aarch64")]
        {
            FMOD_Studio_EventInstance_SetParameterByID(inst, id.to_u64(), 0, ignore, 0.0, 0.0, val)
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            FMOD_Studio_EventInstance_SetParameterByID(inst, id.to_u64(), val, ignore)
        }
    }

    unsafe fn call_studio_sys_set_param_name(
        sys: usize,
        name: *const u8,
        val: f32,
        ignore: i32,
    ) -> i32 {
        #[cfg(target_arch = "aarch64")]
        {
            FMOD_Studio_System_SetParameterByName(sys, name, 0, ignore, 0.0, 0.0, val)
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            FMOD_Studio_System_SetParameterByName(sys, name, val, ignore)
        }
    }

    unsafe fn call_studio_sys_set_listener_weight(sys: usize, idx: i32, weight: f32) -> i32 {
        #[cfg(target_arch = "aarch64")]
        {
            FMOD_Studio_System_SetListenerWeight(sys, idx, 0.0, 0.0, weight)
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            FMOD_Studio_System_SetListenerWeight(sys, idx, weight)
        }
    }

    unsafe fn call_studio_bus_set_volume(bus: usize, vol: f32) -> i32 {
        #[cfg(target_arch = "aarch64")]
        {
            FMOD_Studio_Bus_SetVolume(bus, 0.0, vol)
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            FMOD_Studio_Bus_SetVolume(bus, vol)
        }
    }

    unsafe fn call_studio_vca_set_volume(vca: usize, vol: f32) -> i32 {
        #[cfg(target_arch = "aarch64")]
        {
            FMOD_Studio_VCA_SetVolume(vca, 0.0, vol)
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            FMOD_Studio_VCA_SetVolume(vca, vol)
        }
    }

    struct TestFsbSample {
        name: String,
        channels: u32,
        sample_rate: u32,
        num_frames: u32,
        loop_points: Option<(u32, u32)>,
        data: Vec<u8>,
    }

    fn build_fsb5(version: u32, mode: u32, samples: &[TestFsbSample]) -> Vec<u8> {
        let mut sh = Vec::new();
        let mut data_block = Vec::new();
        let mut has_names = false;
        for s in samples {
            if !s.name.is_empty() {
                has_names = true;
            }
        }

        for s in samples {
            // Align each sample's data offset to 16 bytes.
            while data_block.len() % 16 != 0 {
                data_block.push(0);
            }
            let data_off_units = (data_block.len() / 16) as u64;
            data_block.extend_from_slice(&s.data);

            let freq_idx_opt = FSB5_SAMPLE_RATES
                .iter()
                .position(|&r| r == s.sample_rate && r > 0);
            let freq_idx = freq_idx_opt.unwrap_or(0) as u64;

            let (ch_bits, need_ch_chunk): (u64, bool) = match s.channels {
                1 => (0, false),
                2 => (1, false),
                6 => (2, false),
                8 => (3, false),
                _ => (0, true),
            };
            let need_freq_chunk = freq_idx_opt.is_none();

            let mut chunks: Vec<(u32, Vec<u8>)> = Vec::new();
            if need_ch_chunk {
                chunks.push((FSB5_CHUNK_CHANNELS, vec![s.channels as u8]));
            }
            if need_freq_chunk {
                chunks.push((FSB5_CHUNK_FREQUENCY, s.sample_rate.to_le_bytes().to_vec()));
            }
            if let Some((ls, le)) = s.loop_points {
                let mut lp = Vec::with_capacity(8);
                lp.extend_from_slice(&ls.to_le_bytes());
                lp.extend_from_slice(&le.to_le_bytes());
                chunks.push((FSB5_CHUNK_LOOP, lp));
            }

            let has_chunks = if chunks.is_empty() { 0u64 } else { 1u64 };
            let raw_hdr: u64 = has_chunks
                | ((freq_idx & 0x0F) << 1)
                | ((ch_bits & 0x03) << 5)
                | ((data_off_units & 0x07FF_FFFF) << 7)
                | (((s.num_frames as u64) & 0x3FFF_FFFF) << 34);
            sh.extend_from_slice(&raw_hdr.to_le_bytes());

            for (ci, (ctype, payload)) in chunks.iter().enumerate() {
                let more = if ci + 1 < chunks.len() { 1u32 } else { 0u32 };
                let chdr = more
                    | (((payload.len() as u32) & 0x00FF_FFFF) << 1)
                    | ((ctype & 0x7F) << 25);
                sh.extend_from_slice(&chdr.to_le_bytes());
                sh.extend_from_slice(payload);
            }
        }

        let mut name_table = Vec::new();
        if has_names {
            let mut str_pool = Vec::new();
            let base_off = (samples.len() * 4) as u32;
            for s in samples {
                let off = base_off + str_pool.len() as u32;
                name_table.extend_from_slice(&off.to_le_bytes());
                str_pool.extend_from_slice(s.name.as_bytes());
                str_pool.push(0);
            }
            name_table.extend_from_slice(&str_pool);
        }

        let header_size = if version == 0 { 64usize } else { 60usize };
        let mut out = Vec::with_capacity(header_size + sh.len() + name_table.len() + data_block.len());
        out.extend_from_slice(b"FSB5");
        out.extend_from_slice(&version.to_le_bytes());
        out.extend_from_slice(&(samples.len() as u32).to_le_bytes());
        out.extend_from_slice(&(sh.len() as u32).to_le_bytes());
        out.extend_from_slice(&(name_table.len() as u32).to_le_bytes());
        out.extend_from_slice(&(data_block.len() as u32).to_le_bytes());
        out.extend_from_slice(&mode.to_le_bytes());
        out.resize(header_size, 0);
        out.extend_from_slice(&sh);
        out.extend_from_slice(&name_table);
        out.extend_from_slice(&data_block);
        out
    }

    /// Build a single 140-byte mono (or 280-byte stereo) FADPCM frame encoding constant
    /// levels per channel using `coef_idx = 0` (`[0, 0]`) and `raw_shift = 12` (`shift = 10`,
    /// nibble * 4096 => +4 = +16384 = +0.5, -2 = -8192 = -0.25).
    fn build_fadpcm_constant_frame(channel_nibbles: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(140 * channel_nibbles.len());
        for &nib in channel_nibbles {
            let n = nib & 0x0F;
            let coefs = 0u32; // filter 0 = [0, 0] for all 8 sections
            let shifts = 0xCCCC_CCCCu32; // raw_shift = 12 for all 8 sections
            out.extend_from_slice(&coefs.to_le_bytes());
            out.extend_from_slice(&shifts.to_le_bytes());
            out.extend_from_slice(&0i16.to_le_bytes()); // hist1
            out.extend_from_slice(&0i16.to_le_bytes()); // hist2
            let byte = n | (n << 4);
            out.extend(std::iter::repeat(byte).take(128));
        }
        out
    }

    /// Build a single 36-byte/channel FMOD IMA-ADPCM block (64 samples per channel).
    /// Setting `step_index = 0` (step = 7 => diff = 7 >> 3 = 0 for nibble 0) preserves
    /// `predictor` exactly across all 64 decoded samples!
    fn build_ima_constant_block(channel_predictors: &[i16]) -> Vec<u8> {
        let mut out = Vec::with_capacity(36 * channel_predictors.len());
        for &pred in channel_predictors {
            out.extend_from_slice(&pred.to_le_bytes());
            out.push(0); // step_index = 0
            out.push(0); // reserved
        }
        out.resize(36 * channel_predictors.len(), 0); // all nibbles = 0
        out
    }

    fn riff_chunk(fourcc: &[u8; 4], payload: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(8 + payload.len() + (payload.len() & 1));
        out.extend_from_slice(fourcc);
        out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        out.extend_from_slice(payload);
        if (payload.len() & 1) != 0 {
            out.push(0);
        }
        out
    }

    fn riff_list(list_type: &[u8; 4], chunks: &[Vec<u8>]) -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(list_type);
        for c in chunks {
            body.extend_from_slice(c);
        }
        riff_chunk(b"LIST", &body)
    }

    fn wrap_fev_bank(chunks: &[Vec<u8>]) -> Vec<u8> {
        let mut body = Vec::new();
        body.extend_from_slice(b"FEV ");
        for c in chunks {
            body.extend_from_slice(c);
        }
        let mut out = Vec::with_capacity(8 + body.len());
        out.extend_from_slice(b"RIFF");
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out.extend_from_slice(&body);
        out
    }

    fn push_len_str(buf: &mut Vec<u8>, s: &str) {
        buf.extend_from_slice(&(s.len() as u32).to_le_bytes());
        buf.extend_from_slice(s.as_bytes());
    }

    #[test]
    fn fmod_fsb5_codecs_core_create_sound_and_subsounds() {
        unsafe {
            let sys = create_nrt_system();

            // 1) FSB5 v0 with PCM16 + custom frequency chunk (24 kHz) + 4-ch quad CHANNELS chunk + LOOP chunk + name table
            let mut quad_pcm16 = Vec::new();
            for frame in 0..8 {
                let amp = if frame < 4 { 8192i16 } else { 16384i16 }; // 0.25 then 0.5
                for _ in 0..4 {
                    quad_pcm16.extend_from_slice(&amp.to_le_bytes());
                }
            }
            let fsb_v0 = build_fsb5(
                0,
                FSB5_MODE_PCM16,
                &[
                    TestFsbSample {
                        name: "QuadLoop".into(),
                        channels: 4,
                        sample_rate: 48_000,
                        num_frames: 8,
                        loop_points: Some((4, 7)), // loops over frames 4..=7 (+0.5)
                        data: quad_pcm16,
                    },
                    TestFsbSample {
                        name: "CustomRate".into(),
                        channels: 1,
                        sample_rate: 30_000, // forces CHUNK_FREQUENCY
                        num_frames: 4,
                        loop_points: None,
                        data: [16384i16, 16384, 16384, 16384]
                            .iter()
                            .flat_map(|v| v.to_le_bytes())
                            .collect(),
                    },
                ],
            );

            let ex0 = FmodCreateSoundExInfoHeader {
                cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
                length: fsb_v0.len() as u32,
                ..Default::default()
            };
            let mut snd_v0 = 0usize;
            assert_eq!(
                FMOD_System_CreateSound(
                    sys,
                    fsb_v0.as_ptr(),
                    FMOD_OPENMEMORY,
                    &ex0 as *const _ as usize,
                    &mut snd_v0,
                ),
                FMOD_OK
            );

            let (mut stype, mut fmt, mut chs, mut bits) = (0u32, 0u32, 0i32, 0i32);
            assert_eq!(
                FMOD_Sound_GetFormat(snd_v0, &mut stype, &mut fmt, &mut chs, &mut bits),
                FMOD_OK
            );
            assert_eq!((stype, fmt, chs, bits), (FMOD_SOUND_TYPE_FSB, FMOD_SOUND_FORMAT_PCM16, 4, 16));

            let mut name_buf = [0u8; 64];
            assert_eq!(FMOD_Sound_GetName(snd_v0, name_buf.as_mut_ptr(), 64), FMOD_OK);
            assert_eq!(crate::strings::from_ansi(name_buf.as_ptr()), "QuadLoop");

            let mut nsubs = 0i32;
            assert_eq!(FMOD_Sound_GetNumSubSounds(snd_v0, &mut nsubs), FMOD_OK);
            assert_eq!(nsubs, 2);

            let mut sub1 = 0usize;
            assert_eq!(FMOD_Sound_GetSubSound(snd_v0, 1, &mut sub1), FMOD_OK);
            assert_ne!(sub1, 0);
            assert_eq!(FMOD_Sound_GetName(sub1, name_buf.as_mut_ptr(), 64), FMOD_OK);
            assert_eq!(crate::strings::from_ansi(name_buf.as_ptr()), "CustomRate");
            let mut sub1_freq = 0.0f32;
            assert_eq!(FMOD_Sound_GetDefaults(sub1, &mut sub1_freq, std::ptr::null_mut()), FMOD_OK);
            assert_eq!(sub1_freq, 30_000.0);

            // Play QuadLoop (has LOOP chunk 4..=7): frames 0..4 are 0.25, frames 4..12 loop over 0.5!
            let mut ch0 = 0usize;
            assert_eq!(FMOD_System_PlaySound(sys, snd_v0, 0, 0, &mut ch0), FMOD_OK);
            let mut mix = [0.0f32; 12 * 2];
            assert_eq!(mix_system_frames(sys, &mut mix), Ok(12));
            assert!((mix[0] - 0.25).abs() < 1e-3, "frame 0 got {}", mix[0]);
            assert!((mix[4 * 2] - 0.5).abs() < 1e-3, "frame 4 got {}", mix[4 * 2]);
            assert!((mix[10 * 2] - 0.5).abs() < 1e-3, "looped frame 10 got {}", mix[10 * 2]);
            assert_eq!(FMOD_Channel_Stop(ch0), FMOD_OK);

            // 2) FSB5 v1 FADPCM stereo (+0.5 L, -0.25 R)
            let fadpcm_data = build_fadpcm_constant_frame(&[4, 0xE]); // +4*4096=+16384 (+0.5), -2*4096=-8192 (-0.25)
            let fsb_fadpcm = build_fsb5(
                1,
                FSB5_MODE_FADPCM,
                &[TestFsbSample {
                    name: "FadpcmTone".into(),
                    channels: 2,
                    sample_rate: 48_000,
                    num_frames: 64,
                    loop_points: None,
                    data: fadpcm_data,
                }],
            );
            let ex_fad = FmodCreateSoundExInfoHeader {
                cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
                length: fsb_fadpcm.len() as u32,
                ..Default::default()
            };
            let mut snd_fad = 0usize;
            assert_eq!(
                FMOD_System_CreateSound(
                    sys,
                    fsb_fadpcm.as_ptr(),
                    FMOD_OPENMEMORY,
                    &ex_fad as *const _ as usize,
                    &mut snd_fad,
                ),
                FMOD_OK
            );
            let mut ch_fad = 0usize;
            assert_eq!(FMOD_System_PlaySound(sys, snd_fad, 0, 0, &mut ch_fad), FMOD_OK);
            let mut out_fad = [0.0f32; 8 * 2];
            assert_eq!(mix_system_frames(sys, &mut out_fad), Ok(8));
            for f in 0..8 {
                assert!((out_fad[f * 2] - 0.5).abs() < 1e-3, "FADPCM L[{f}]={}", out_fad[f * 2]);
                assert!((out_fad[f * 2 + 1] - (-0.25)).abs() < 1e-3, "FADPCM R[{f}]={}", out_fad[f * 2 + 1]);
            }
            assert_eq!(FMOD_Channel_Stop(ch_fad), FMOD_OK);

            // 3) FSB5 v1 IMA-ADPCM stereo (+0.75 L, -0.5 R)
            let ima_data = build_ima_constant_block(&[24_576, -16_384]);
            let fsb_ima = build_fsb5(
                1,
                FSB5_MODE_IMAADPCM,
                &[TestFsbSample {
                    name: "ImaTone".into(),
                    channels: 2,
                    sample_rate: 48_000,
                    num_frames: 64,
                    loop_points: None,
                    data: ima_data,
                }],
            );
            let ex_ima = FmodCreateSoundExInfoHeader {
                cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
                length: fsb_ima.len() as u32,
                ..Default::default()
            };
            let mut snd_ima = 0usize;
            assert_eq!(
                FMOD_System_CreateSound(
                    sys,
                    fsb_ima.as_ptr(),
                    FMOD_OPENMEMORY,
                    &ex_ima as *const _ as usize,
                    &mut snd_ima,
                ),
                FMOD_OK
            );
            let mut ch_ima = 0usize;
            assert_eq!(FMOD_System_PlaySound(sys, snd_ima, 0, 0, &mut ch_ima), FMOD_OK);
            let mut out_ima = [0.0f32; 8 * 2];
            assert_eq!(mix_system_frames(sys, &mut out_ima), Ok(8));
            for f in 0..8 {
                assert!((out_ima[f * 2] - 0.75).abs() < 1e-3, "IMA L[{f}]={}", out_ima[f * 2]);
                assert!((out_ima[f * 2 + 1] - (-0.5)).abs() < 1e-3, "IMA R[{f}]={}", out_ima[f * 2 + 1]);
            }
            assert_eq!(FMOD_Channel_Stop(ch_ima), FMOD_OK);

            // 4) FSB5 PCM8, PCM24, PCM32, and PCMFloat codecs
            let fsb_pcm8 = build_fsb5(
                1,
                FSB5_MODE_PCM8,
                &[TestFsbSample {
                    name: "Pcm8".into(),
                    channels: 1,
                    sample_rate: 48_000,
                    num_frames: 4,
                    loop_points: None,
                    data: vec![192u8; 4], // (192-128)/128 = +0.5
                }],
            );
            let fsb_pcm24 = build_fsb5(
                1,
                FSB5_MODE_PCM24,
                &[TestFsbSample {
                    name: "Pcm24".into(),
                    channels: 1,
                    sample_rate: 48_000,
                    num_frames: 4,
                    loop_points: None,
                    data: [0x00u8, 0x00, 0xC0].repeat(4), // -0.5
                }],
            );
            let fsb_pcm32 = build_fsb5(
                1,
                FSB5_MODE_PCM32,
                &[TestFsbSample {
                    name: "Pcm32".into(),
                    channels: 1,
                    sample_rate: 48_000,
                    num_frames: 4,
                    loop_points: None,
                    data: (i32::MAX / 2 + 1).to_le_bytes().repeat(4), // +0.5
                }],
            );
            let fsb_pcmf = build_fsb5(
                1,
                FSB5_MODE_PCMFLOAT,
                &[TestFsbSample {
                    name: "PcmFloat".into(),
                    channels: 1,
                    sample_rate: 48_000,
                    num_frames: 4,
                    loop_points: None,
                    data: 0.625f32.to_le_bytes().repeat(4),
                }],
            );
            for (blob, expected_amp) in [
                (&fsb_pcm8, 0.5f32),
                (&fsb_pcm24, -0.5f32),
                (&fsb_pcm32, 0.5f32),
                (&fsb_pcmf, 0.625f32),
            ] {
                let ex_p = FmodCreateSoundExInfoHeader {
                    cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
                    length: blob.len() as u32,
                    ..Default::default()
                };
                let mut snd_p = 0usize;
                assert_eq!(
                    FMOD_System_CreateSound(
                        sys,
                        blob.as_ptr(),
                        FMOD_OPENMEMORY,
                        &ex_p as *const _ as usize,
                        &mut snd_p,
                    ),
                    FMOD_OK
                );
                let mut ch_p = 0usize;
                assert_eq!(FMOD_System_PlaySound(sys, snd_p, 0, 0, &mut ch_p), FMOD_OK);
                let mut out_p = [0.0f32; 4 * 2];
                assert_eq!(mix_system_frames(sys, &mut out_p), Ok(4));
                assert!((out_p[0] - expected_amp).abs() < 1e-3, "got {} expected {expected_amp}", out_p[0]);
                assert_eq!(FMOD_Sound_Release(snd_p), FMOD_OK);
            }

            // 5) Non-trivial FADPCM filter prediction (coef_idx=1: [60, 0], hist1=16384, raw_shift=10, nib=+1 -> 16384)
            let mut pred_fadpcm = Vec::with_capacity(140);
            pred_fadpcm.extend_from_slice(&0x1111_1111u32.to_le_bytes()); // coef_idx = 1 ([60, 0])
            pred_fadpcm.extend_from_slice(&0xAAAA_AAAAu32.to_le_bytes()); // raw_shift = 10 (shift = 12)
            pred_fadpcm.extend_from_slice(&16_384i16.to_le_bytes()); // hist1 = 16384
            pred_fadpcm.extend_from_slice(&0i16.to_le_bytes()); // hist2 = 0
            pred_fadpcm.extend(std::iter::repeat(0x11u8).take(128)); // nibble = +1 -> residual = 1024 -> (1024*64 + 16384*60 + 32) >> 6 = 16384!
            let dec_pred = decode_fsb5_fadpcm(&pred_fadpcm, 1, 16);
            assert_eq!(dec_pred.len(), 16);
            for (i, &v) in dec_pred.iter().enumerate() {
                assert!((v - 0.5).abs() < 1e-4, "FADPCM filter1[{i}]={v}");
            }

            // 6) Non-trivial IMA-ADPCM adaptive step transitions (predictor=0, step_index=10, nibs 5 then 10 -> 25 then 8)
            let mut adapt_ima = vec![0u8; 36];
            adapt_ima[2] = 10; // step_index = 10 (step = 19)
            adapt_ima[4] = 0xA5; // low nibble = 5 (+25, idx->14 step=28), high nibble = 10 (-17 -> 8)
            let dec_ima = decode_fsb5_ima_adpcm(&adapt_ima, 1, 2);
            assert!((dec_ima[0] - (25.0 / 32_768.0)).abs() < 1e-5, "IMA[0]={}", dec_ima[0]);
            assert!((dec_ima[1] - (8.0 / 32_768.0)).abs() < 1e-5, "IMA[1]={}", dec_ima[1]);

            // 7) Unsupported codec mode inside an otherwise valid FSB5 fails cleanly with FMOD_ERR_FORMAT
            let fsb_bad_codec = build_fsb5(
                1,
                15, // Vorbis (unsupported here)
                &[TestFsbSample {
                    name: "Bad".into(),
                    channels: 1,
                    sample_rate: 48_000,
                    num_frames: 4,
                    loop_points: None,
                    data: vec![0u8; 16],
                }],
            );
            let ex_bad = FmodCreateSoundExInfoHeader {
                cbsize: std::mem::size_of::<FmodCreateSoundExInfoHeader>() as i32,
                length: fsb_bad_codec.len() as u32,
                ..Default::default()
            };
            let mut bad_snd = 0usize;
            assert_eq!(
                FMOD_System_CreateSound(
                    sys,
                    fsb_bad_codec.as_ptr(),
                    FMOD_OPENMEMORY,
                    &ex_bad as *const _ as usize,
                    &mut bad_snd,
                ),
                FMOD_ERR_FORMAT
            );

            assert_eq!(FMOD_Sound_Release(snd_v0), FMOD_OK);
            assert_eq!(FMOD_Sound_Release(snd_fad), FMOD_OK);
            assert_eq!(FMOD_Sound_Release(snd_ima), FMOD_OK);
            assert_eq!(FMOD_System_Release(sys), FMOD_OK);
        }
    }

    #[test]
    fn fmod_studio_riff_fev_bank_playback_buses_vcas_3d_params_and_unload() {
        unsafe {
            let mut studio = 0usize;
            assert_eq!(FMOD_Studio_System_Create(&mut studio, FMOD_VERSION), FMOD_OK);
            assert_eq!(FMOD_Studio_System_IsValid(studio), 1);

            let mut core = 0usize;
            assert_eq!(FMOD_Studio_System_GetCoreSystem(studio, &mut core), FMOD_OK);
            assert_eq!(FMOD_System_SetOutput(core, FMOD_OUTPUTTYPE_NOSOUND_NRT), FMOD_OK);
            assert_eq!(FMOD_Studio_System_Initialize(studio, 64, 0, 0, 0), FMOD_OK);

            // Deterministic GUIDs for strings bank, SFX bank, buses, VCA, and 3 events.
            let strings_bank_guid = FmodGuid::parse_str("{11111111-1111-1111-1111-111111111111}").unwrap();
            let sfx_bank_guid = FmodGuid::parse_str("{22222222-2222-2222-2222-222222222222}").unwrap();
            let bus_sfx_guid = FmodGuid::parse_str("{33333333-3333-3333-3333-333333333333}").unwrap();
            let bus_weapons_guid = FmodGuid::parse_str("{44444444-4444-4444-4444-444444444444}").unwrap();
            let vca_sfx_guid = FmodGuid::parse_str("{55555555-5555-5555-5555-555555555555}").unwrap();
            let ev_pcm16_guid = FmodGuid::parse_str("{aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa}").unwrap();
            let ev_fadpcm_guid = FmodGuid::parse_str("{bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb}").unwrap();
            let ev_ima3d_guid = FmodGuid::parse_str("{cccccccc-cccc-cccc-cccc-cccccccccccc}").unwrap();

            // 1) Synthesize Master.strings.bank using a real STRR Radix Trie + STRT flat entries,
            //    and load it from a virtual Windows path (`crate::path`).
            //    Radix trie nodes:
            //      node 0: parent=-1, guid=-1, "event:/SFX/"
            //      node 1: parent=0,  guid=0,  "TonePCM16"  -> "event:/SFX/TonePCM16"
            //      node 2: parent=0,  guid=1,  "ToneFADPCM" -> "event:/SFX/ToneFADPCM"
            //      node 3: parent=0,  guid=2,  "SpatialIMA" -> "event:/SFX/SpatialIMA"
            let pool = b"event:/SFX/TonePCM16ToneFADPCMSpatialIMA";
            let mut strr = Vec::new();
            strr.extend_from_slice(&3u32.to_le_bytes()); // guid_count
            strr.extend_from_slice(&4u32.to_le_bytes()); // node_count
            strr.extend_from_slice(&(pool.len() as u32).to_le_bytes());
            strr.extend_from_slice(&ev_pcm16_guid.to_le_bytes());
            strr.extend_from_slice(&ev_fadpcm_guid.to_le_bytes());
            strr.extend_from_slice(&ev_ima3d_guid.to_le_bytes());
            let nodes: [(i32, i32, u32, u32); 4] = [
                (-1, -1, 0, 11),  // "event:/SFX/"
                (0, 0, 11, 9),    // "TonePCM16"
                (0, 1, 20, 10),   // "ToneFADPCM"
                (0, 2, 30, 10),   // "SpatialIMA"
            ];
            for (p, gi, off, len) in nodes {
                strr.extend_from_slice(&p.to_le_bytes());
                strr.extend_from_slice(&gi.to_le_bytes());
                strr.extend_from_slice(&off.to_le_bytes());
                strr.extend_from_slice(&len.to_le_bytes());
            }
            strr.extend_from_slice(pool);

            // Flat STRT (type 0) for buses, VCA, and bank paths.
            let mut strt = Vec::new();
            strt.extend_from_slice(&0u32.to_le_bytes()); // type = 0 (inline)
            strt.extend_from_slice(&5u32.to_le_bytes()); // count = 5
            for (g, p) in [
                (strings_bank_guid, "bank:/Master.strings"),
                (sfx_bank_guid, "bank:/SFX"),
                (bus_sfx_guid, "bus:/SFX"),
                (bus_weapons_guid, "bus:/SFX/Weapons"),
                (vca_sfx_guid, "vca:/MasterSFX"),
            ] {
                strt.extend_from_slice(&g.to_le_bytes());
                push_len_str(&mut strt, p);
            }

            let strings_bank_bytes = wrap_fev_bank(&[
                riff_chunk(b"FMT ", &0x0000_0089u32.to_le_bytes()),
                riff_list(
                    b"PROJ",
                    &[
                        riff_chunk(b"BNKI", &strings_bank_guid.to_le_bytes()),
                        riff_chunk(b"STRR", &strr),
                        riff_chunk(b"STRT", &strt),
                    ],
                ),
            ]);

            let tmp_dir = std::env::temp_dir().join("win32_rt_fmod_studio_bank_test");
            let _ = std::fs::create_dir_all(&tmp_dir);
            let strings_file = tmp_dir.join("Master.strings.bank");
            std::fs::write(&strings_file, &strings_bank_bytes).unwrap();
            crate::path::add_mount("G:\\FmodStudio", &tmp_dir);

            let mut strings_bank = 0usize;
            assert_eq!(
                FMOD_Studio_System_LoadBankFile(
                    studio,
                    b"G:\\FmodStudio\\Master.strings.bank\0".as_ptr(),
                    FMOD_STUDIO_LOAD_BANK_NORMAL,
                    &mut strings_bank,
                ),
                FMOD_OK
            );
            assert_eq!(FMOD_Studio_Bank_IsValid(strings_bank), 1);

            // Duplicate load of the same BNKI GUID returns FMOD_ERR_EVENT_ALREADY_LOADED.
            let mut dup_bank = 0xDEADusize;
            assert_eq!(
                FMOD_Studio_System_LoadBankMemory(
                    studio,
                    strings_bank_bytes.as_ptr(),
                    strings_bank_bytes.len() as i32,
                    FMOD_STUDIO_LOAD_MEMORY,
                    FMOD_STUDIO_LOAD_BANK_NORMAL,
                    &mut dup_bank,
                ),
                FMOD_ERR_EVENT_ALREADY_LOADED
            );
            assert_eq!(dup_bank, 0);

            // LookupID / LookupPath works from Master.strings.bank before SFX.bank is loaded,
            // while GetEvent still honestly returns FMOD_ERR_EVENT_NOTFOUND!
            let mut looked_up = FmodGuid::ZERO;
            assert_eq!(
                FMOD_Studio_System_LookupID(studio, b"event:/SFX/TonePCM16\0".as_ptr(), &mut looked_up),
                FMOD_OK
            );
            assert_eq!(looked_up, ev_pcm16_guid);
            let mut missing_ev = 0usize;
            assert_eq!(
                FMOD_Studio_System_GetEvent(studio, b"event:/SFX/TonePCM16\0".as_ptr(), &mut missing_ev),
                FMOD_ERR_EVENT_NOTFOUND
            );

            // 2) Synthesize SFX.bank (RIFF FEV) with:
            //    - BUSS: bus:/SFX (parent=master), bus:/SFX/Weapons (parent=bus:/SFX)
            //    - VCAS: vca:/MasterSFX controlling bus:/SFX
            //    - EVNT: 3 events referenced ONLY by GUID (paths resolved cross-bank via Master.strings.bank!)
            //    - PARM: local event param "Intensity" + automatic "Distance" + global "TimeOfDay"
            //    - SND : 3 embedded FSB5 sub-banks (PCM16, FADPCM, IMA-ADPCM) with padding!
            let mut buss = Vec::new();
            buss.extend_from_slice(&2u32.to_le_bytes());
            // bus:/SFX (empty inline path -> resolved from strings bank!)
            buss.extend_from_slice(&bus_sfx_guid.to_le_bytes());
            buss.extend_from_slice(&FmodGuid::ZERO.to_le_bytes());
            buss.extend_from_slice(&1.0f32.to_le_bytes());
            buss.extend_from_slice(&0u32.to_le_bytes());
            push_len_str(&mut buss, "");
            // bus:/SFX/Weapons (parent = bus:/SFX)
            buss.extend_from_slice(&bus_weapons_guid.to_le_bytes());
            buss.extend_from_slice(&bus_sfx_guid.to_le_bytes());
            buss.extend_from_slice(&1.0f32.to_le_bytes());
            buss.extend_from_slice(&0u32.to_le_bytes());
            push_len_str(&mut buss, "");

            let mut vcas = Vec::new();
            vcas.extend_from_slice(&1u32.to_le_bytes());
            vcas.extend_from_slice(&vca_sfx_guid.to_le_bytes());
            vcas.extend_from_slice(&1.0f32.to_le_bytes());
            push_len_str(&mut vcas, "");
            vcas.extend_from_slice(&1u32.to_le_bytes()); // target_bus_count = 1
            vcas.extend_from_slice(&bus_sfx_guid.to_le_bytes());
            vcas.extend_from_slice(&0u32.to_le_bytes()); // target_event_count = 0

            let mut evnt = Vec::new();
            evnt.extend_from_slice(&3u32.to_le_bytes());
            // Event 0: PCM16 stereo (+0.8 L, +0.4 R), 2D oneshot, routed to bus:/SFX/Weapons
            evnt.extend_from_slice(&ev_pcm16_guid.to_le_bytes());
            evnt.extend_from_slice(&bus_weapons_guid.to_le_bytes());
            evnt.extend_from_slice(&0u32.to_le_bytes()); // sample_index 0
            evnt.extend_from_slice(&0x02u32.to_le_bytes()); // oneshot
            evnt.extend_from_slice(&1.0f32.to_le_bytes()); // vol
            evnt.extend_from_slice(&1.0f32.to_le_bytes()); // pitch
            evnt.extend_from_slice(&1.0f32.to_le_bytes()); // min_dist
            evnt.extend_from_slice(&50.0f32.to_le_bytes()); // max_dist
            evnt.extend_from_slice(&0u32.to_le_bytes()); // derive length from sample
            push_len_str(&mut evnt, "");
            // Event 1: FADPCM mono (+0.5), 2D looping, routed to bus:/SFX
            evnt.extend_from_slice(&ev_fadpcm_guid.to_le_bytes());
            evnt.extend_from_slice(&bus_sfx_guid.to_le_bytes());
            evnt.extend_from_slice(&1u32.to_le_bytes()); // sample_index 1
            evnt.extend_from_slice(&0x00u32.to_le_bytes()); // looping (not oneshot)
            evnt.extend_from_slice(&1.0f32.to_le_bytes());
            evnt.extend_from_slice(&1.0f32.to_le_bytes());
            evnt.extend_from_slice(&1.0f32.to_le_bytes());
            evnt.extend_from_slice(&50.0f32.to_le_bytes());
            evnt.extend_from_slice(&0u32.to_le_bytes());
            push_len_str(&mut evnt, "");
            // Event 2: IMA-ADPCM mono (+0.8), 3D looping, routed to bus:/SFX
            evnt.extend_from_slice(&ev_ima3d_guid.to_le_bytes());
            evnt.extend_from_slice(&bus_sfx_guid.to_le_bytes());
            evnt.extend_from_slice(&2u32.to_le_bytes()); // sample_index 2
            evnt.extend_from_slice(&0x01u32.to_le_bytes()); // 3D + looping
            evnt.extend_from_slice(&1.0f32.to_le_bytes());
            evnt.extend_from_slice(&1.0f32.to_le_bytes());
            evnt.extend_from_slice(&1.0f32.to_le_bytes());
            evnt.extend_from_slice(&100.0f32.to_le_bytes());
            evnt.extend_from_slice(&0u32.to_le_bytes());
            push_len_str(&mut evnt, "");

            let intensity_id = FmodStudioParameterId { data1: 0x1001, data2: 0x2002 };
            let distance_id = FmodStudioParameterId { data1: 0x3003, data2: 0x4004 };
            let tod_id = FmodStudioParameterId { data1: 0x5005, data2: 0x6006 };
            let mut parm = Vec::new();
            parm.extend_from_slice(&3u32.to_le_bytes());
            for (owner, pid, min_v, max_v, def_v, flags, ptype, pname) in [
                (ev_pcm16_guid, intensity_id, 0.0f32, 10.0f32, 2.5f32, 0u32, FMOD_STUDIO_PARAMETER_GAME_CONTROLLED, "Intensity"),
                (ev_ima3d_guid, distance_id, 0.0f32, 100.0f32, 0.0f32, FMOD_STUDIO_PARAMETER_AUTOMATIC, FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE, "Distance"),
                (FmodGuid::ZERO, tod_id, 0.0f32, 24.0f32, 12.0f32, FMOD_STUDIO_PARAMETER_GLOBAL, FMOD_STUDIO_PARAMETER_GAME_CONTROLLED, "TimeOfDay"),
            ] {
                parm.extend_from_slice(&owner.to_le_bytes());
                parm.extend_from_slice(&pid.to_u64().to_le_bytes());
                parm.extend_from_slice(&min_v.to_le_bytes());
                parm.extend_from_slice(&max_v.to_le_bytes());
                parm.extend_from_slice(&def_v.to_le_bytes());
                parm.extend_from_slice(&flags.to_le_bytes());
                parm.extend_from_slice(&ptype.to_le_bytes());
                push_len_str(&mut parm, pname);
            }

            // Build 3 embedded FSB5 sub-banks inside a single SND chunk (with leading alignment padding).
            let mut pcm16_data = Vec::new();
            for _ in 0..480 {
                // 10 ms at 48 kHz: L = +0.8 (26214), R = +0.4 (13107)
                pcm16_data.extend_from_slice(&26_214i16.to_le_bytes());
                pcm16_data.extend_from_slice(&13_107i16.to_le_bytes());
            }
            let fsb_pcm16 = build_fsb5(
                1,
                FSB5_MODE_PCM16,
                &[TestFsbSample {
                    name: "TonePCM16".into(),
                    channels: 2,
                    sample_rate: 48_000,
                    num_frames: 480,
                    loop_points: None,
                    data: pcm16_data,
                }],
            );
            let fsb_fadpcm = build_fsb5(
                1,
                FSB5_MODE_FADPCM,
                &[TestFsbSample {
                    name: "ToneFADPCM".into(),
                    channels: 1,
                    sample_rate: 48_000,
                    num_frames: 256,
                    loop_points: None,
                    data: build_fadpcm_constant_frame(&[4]), // +0.5 mono
                }],
            );
            let fsb_ima = build_fsb5(
                1,
                FSB5_MODE_IMAADPCM,
                &[TestFsbSample {
                    name: "SpatialIMA".into(),
                    channels: 1,
                    sample_rate: 48_000,
                    num_frames: 64,
                    loop_points: None,
                    data: build_ima_constant_block(&[26_214]), // +0.8 mono
                }],
            );

            let mut snd_payload = vec![0u8; 16]; // 16-byte alignment padding before first FSB5
            snd_payload.extend_from_slice(&fsb_pcm16);
            while snd_payload.len() % 16 != 0 {
                snd_payload.push(0);
            }
            snd_payload.extend_from_slice(&fsb_fadpcm);
            while snd_payload.len() % 16 != 0 {
                snd_payload.push(0);
            }
            snd_payload.extend_from_slice(&fsb_ima);

            let sfx_bank_bytes = wrap_fev_bank(&[
                riff_chunk(b"FMT ", &0x0000_0089u32.to_le_bytes()),
                riff_chunk(b"BNKI", &sfx_bank_guid.to_le_bytes()),
                riff_list(
                    b"MODL",
                    &[
                        riff_chunk(b"BUSS", &buss),
                        riff_chunk(b"VCAS", &vcas),
                        riff_chunk(b"EVNT", &evnt),
                        riff_chunk(b"PARM", &parm),
                    ],
                ),
                riff_chunk(b"SNDH", &1u32.to_le_bytes()),
                riff_chunk(b"SND ", &snd_payload),
            ]);

            let mut sfx_bank = 0usize;
            assert_eq!(
                FMOD_Studio_System_LoadBankMemory(
                    studio,
                    sfx_bank_bytes.as_ptr(),
                    sfx_bank_bytes.len() as i32,
                    FMOD_STUDIO_LOAD_MEMORY,
                    FMOD_STUDIO_LOAD_BANK_NORMAL,
                    &mut sfx_bank,
                ),
                FMOD_OK
            );
            assert_eq!(FMOD_Studio_Bank_IsValid(sfx_bank), 1);

            let mut load_state = 0u32;
            assert_eq!(FMOD_Studio_Bank_GetLoadingState(sfx_bank, &mut load_state), FMOD_OK);
            assert_eq!(load_state, FMOD_STUDIO_LOADING_STATE_LOADED);

            // Check Bank list queries.
            let mut ev_count = 0i32;
            assert_eq!(FMOD_Studio_Bank_GetEventCount(sfx_bank, &mut ev_count), FMOD_OK);
            assert_eq!(ev_count, 3);
            let mut ev_list = [0usize; 4];
            let mut got = 0i32;
            assert_eq!(FMOD_Studio_Bank_GetEventList(sfx_bank, ev_list.as_mut_ptr(), 4, &mut got), FMOD_OK);
            assert_eq!(got, 3);

            // Look up events by path (case-insensitive) and by {GUID} string!
            let mut ev_pcm = 0usize;
            let mut ev_fad = 0usize;
            let mut ev_ima = 0usize;
            assert_eq!(
                FMOD_Studio_System_GetEvent(studio, b"event:/sfx/tonepcm16\0".as_ptr(), &mut ev_pcm),
                FMOD_OK
            );
            assert_eq!(
                FMOD_Studio_System_GetEvent(studio, b"{bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb}\0".as_ptr(), &mut ev_fad),
                FMOD_OK
            );
            assert_eq!(
                FMOD_Studio_System_GetEventByID(studio, &ev_ima3d_guid, &mut ev_ima),
                FMOD_OK
            );

            // Verify EventDescription metadata & path queries (including FMOD_ERR_TRUNCATED!).
            let mut path_buf = [0u8; 64];
            let mut retrieved = 0i32;
            assert_eq!(
                FMOD_Studio_EventDescription_GetPath(ev_pcm, path_buf.as_mut_ptr(), 8, &mut retrieved),
                FMOD_ERR_TRUNCATED
            );
            assert_eq!(retrieved as usize, "event:/SFX/TonePCM16".len() + 1);
            assert_eq!(
                FMOD_Studio_EventDescription_GetPath(ev_pcm, path_buf.as_mut_ptr(), 64, &mut retrieved),
                FMOD_OK
            );
            assert_eq!(crate::strings::from_ansi(path_buf.as_ptr()), "event:/SFX/TonePCM16");

            let (mut len_ms, mut is_oneshot, mut is_3d) = (0i32, 0i32, 0i32);
            assert_eq!(FMOD_Studio_EventDescription_GetLength(ev_pcm, &mut len_ms), FMOD_OK);
            assert_eq!(len_ms, 10); // 480 frames @ 48 kHz = 10 ms
            assert_eq!(FMOD_Studio_EventDescription_IsOneshot(ev_pcm, &mut is_oneshot), FMOD_OK);
            assert_eq!(is_oneshot, 1);
            assert_eq!(FMOD_Studio_EventDescription_Is3D(ev_ima, &mut is_3d), FMOD_OK);
            assert_eq!(is_3d, 1);

            // Look up Buses and VCA.
            let (mut bus_master, mut bus_sfx, mut bus_weapons, mut vca_sfx) = (0usize, 0usize, 0usize, 0usize);
            assert_eq!(FMOD_Studio_System_GetBus(studio, b"bus:/\0".as_ptr(), &mut bus_master), FMOD_OK);
            assert_eq!(FMOD_Studio_System_GetBus(studio, b"bus:/SFX\0".as_ptr(), &mut bus_sfx), FMOD_OK);
            assert_eq!(FMOD_Studio_System_GetBus(studio, b"bus:/SFX/Weapons\0".as_ptr(), &mut bus_weapons), FMOD_OK);
            assert_eq!(FMOD_Studio_System_GetVCA(studio, b"vca:/MasterSFX\0".as_ptr(), &mut vca_sfx), FMOD_OK);

            // --- A) PCM16 event + Bus hierarchy + VCA scaling + timeline + playback state ---
            let mut inst_pcm = 0usize;
            assert_eq!(FMOD_Studio_EventDescription_CreateInstance(ev_pcm, &mut inst_pcm), FMOD_OK);
            let mut state = 99u32;
            assert_eq!(FMOD_Studio_EventInstance_GetPlaybackState(inst_pcm, &mut state), FMOD_OK);
            assert_eq!(state, FMOD_STUDIO_PLAYBACK_STOPPED);

            assert_eq!(FMOD_Studio_EventInstance_Start(inst_pcm), FMOD_OK);
            assert_eq!(FMOD_Studio_EventInstance_GetPlaybackState(inst_pcm, &mut state), FMOD_OK);
            assert_eq!(state, FMOD_STUDIO_PLAYBACK_PLAYING);

            // Mix 48 frames (1 ms) at unity gain: L = 0.8, R = 0.4
            let mut out = [0.0f32; 48 * 2];
            assert_eq!(mix_system_frames(core, &mut out), Ok(48));
            assert!((out[0] - 0.8).abs() < 1e-3, "PCM16 L got {}", out[0]);
            assert!((out[1] - 0.4).abs() < 1e-3, "PCM16 R got {}", out[1]);

            let mut tpos = -1i32;
            assert_eq!(FMOD_Studio_EventInstance_GetTimelinePosition(inst_pcm, &mut tpos), FMOD_OK);
            assert_eq!(tpos, 1);

            // Scale volume: inst=0.5 * bus_weapons=0.5 * bus_sfx=0.5 * vca_sfx=0.5 = 0.0625
            // Expected L = 0.8 * 0.0625 = 0.05, R = 0.4 * 0.0625 = 0.025
            assert_eq!(call_studio_inst_set_volume(inst_pcm, 0.5), FMOD_OK);
            assert_eq!(call_studio_bus_set_volume(bus_weapons, 0.5), FMOD_OK);
            assert_eq!(call_studio_bus_set_volume(bus_sfx, 0.5), FMOD_OK);
            assert_eq!(call_studio_vca_set_volume(vca_sfx, 0.5), FMOD_OK);

            let (mut bvol, mut bfinal) = (0.0f32, 0.0f32);
            assert_eq!(FMOD_Studio_Bus_GetVolume(bus_weapons, &mut bvol, &mut bfinal), FMOD_OK);
            assert!((bvol - 0.5).abs() < 1e-5);
            assert!((bfinal - 0.125).abs() < 1e-5, "bus_weapons final={bfinal}");

            assert_eq!(mix_system_frames(core, &mut out), Ok(48));
            assert!((out[0] - 0.05).abs() < 1e-3, "scaled L got {}", out[0]);
            assert!((out[1] - 0.025).abs() < 1e-3, "scaled R got {}", out[1]);

            // Mute bus_sfx -> silence while timeline still advances.
            assert_eq!(FMOD_Studio_Bus_SetMute(bus_sfx, 1), FMOD_OK);
            assert_eq!(mix_system_frames(core, &mut out), Ok(48));
            assert!(out.iter().all(|&v| v == 0.0));
            assert_eq!(FMOD_Studio_Bus_SetMute(bus_sfx, 0), FMOD_OK);

            // Pause instance -> silence AND timeline stays frozen.
            let mut pos_before_pause = 0i32;
            assert_eq!(FMOD_Studio_EventInstance_GetTimelinePosition(inst_pcm, &mut pos_before_pause), FMOD_OK);
            assert_eq!(FMOD_Studio_EventInstance_SetPaused(inst_pcm, 1), FMOD_OK);
            assert_eq!(mix_system_frames(core, &mut out), Ok(48));
            assert!(out.iter().all(|&v| v == 0.0));
            let mut pos_during_pause = 0i32;
            assert_eq!(FMOD_Studio_EventInstance_GetTimelinePosition(inst_pcm, &mut pos_during_pause), FMOD_OK);
            assert_eq!(pos_during_pause, pos_before_pause);
            assert_eq!(FMOD_Studio_EventInstance_SetPaused(inst_pcm, 0), FMOD_OK);

            // Seek to 9 ms (432 frames) and mix 64 frames -> one-shot finishes cleanly!
            assert_eq!(FMOD_Studio_EventInstance_SetTimelinePosition(inst_pcm, 9), FMOD_OK);
            let mut tail = [0.0f32; 64 * 2];
            assert_eq!(mix_system_frames(core, &mut tail), Ok(64));
            assert_eq!(FMOD_Studio_EventInstance_GetPlaybackState(inst_pcm, &mut state), FMOD_OK);
            assert_eq!(state, FMOD_STUDIO_PLAYBACK_STOPPED);

            // Local & Global Parameters (by name and by ID, with clamping + error on unknown).
            let (mut pv, mut pfv) = (0.0f32, 0.0f32);
            assert_eq!(FMOD_Studio_EventInstance_GetParameterByName(inst_pcm, b"Intensity\0".as_ptr(), &mut pv, &mut pfv), FMOD_OK);
            assert!((pv - 2.5).abs() < 1e-5);
            assert_eq!(call_studio_inst_set_param_name(inst_pcm, b"Intensity\0".as_ptr(), 99.0, 0), FMOD_OK);
            assert_eq!(FMOD_Studio_EventInstance_GetParameterByID(inst_pcm, intensity_id.to_u64(), &mut pv, &mut pfv), FMOD_OK);
            assert!((pv - 10.0).abs() < 1e-5, "expected clamped max 10.0, got {pv}");
            assert_eq!(call_studio_inst_set_param_id(inst_pcm, intensity_id, 4.25, 1), FMOD_OK);
            assert_eq!(FMOD_Studio_EventInstance_GetParameterByName(inst_pcm, b"Intensity\0".as_ptr(), &mut pv, &mut pfv), FMOD_OK);
            assert!((pv - 4.25).abs() < 1e-5);
            assert_eq!(
                call_studio_inst_set_param_name(inst_pcm, b"NoSuchParam\0".as_ptr(), 1.0, 0),
                FMOD_ERR_EVENT_NOTFOUND
            );
            assert_eq!(call_studio_sys_set_param_name(studio, b"TimeOfDay\0".as_ptr(), 18.5, 0), FMOD_OK);
            assert_eq!(FMOD_Studio_System_GetParameterByName(studio, b"TimeOfDay\0".as_ptr(), &mut pv, &mut pfv), FMOD_OK);
            assert!((pv - 18.5).abs() < 1e-5);

            assert_eq!(FMOD_Studio_EventInstance_Release(inst_pcm), FMOD_OK);
            assert_eq!(FMOD_Studio_EventInstance_IsValid(inst_pcm), 0);

            // Reset bus & VCA volumes to unity.
            assert_eq!(call_studio_bus_set_volume(bus_weapons, 1.0), FMOD_OK);
            assert_eq!(call_studio_bus_set_volume(bus_sfx, 1.0), FMOD_OK);
            assert_eq!(call_studio_vca_set_volume(vca_sfx, 1.0), FMOD_OK);

            // --- B) FADPCM looping event + pitch + StopAllEvents + fire-and-forget Start+Release ---
            let mut inst_fad = 0usize;
            assert_eq!(FMOD_Studio_EventDescription_CreateInstance(ev_fad, &mut inst_fad), FMOD_OK);
            assert_eq!(call_studio_inst_set_pitch(inst_fad, 2.0), FMOD_OK);
            let mut pitch_out = 0.0f32;
            assert_eq!(FMOD_Studio_EventInstance_GetPitch(inst_fad, &mut pitch_out, std::ptr::null_mut()), FMOD_OK);
            assert!((pitch_out - 2.0).abs() < 1e-5);
            assert_eq!(FMOD_Studio_EventInstance_Start(inst_fad), FMOD_OK);

            let mut out_fad = [0.0f32; 16 * 2];
            assert_eq!(mix_system_frames(core, &mut out_fad), Ok(16));
            assert!((out_fad[0] - 0.5).abs() < 1e-3, "Studio FADPCM L got {}", out_fad[0]);
            assert!((out_fad[1] - 0.5).abs() < 1e-3, "Studio FADPCM R got {}", out_fad[1]);

            assert_eq!(FMOD_Studio_Bus_StopAllEvents(bus_sfx, FMOD_STUDIO_STOP_IMMEDIATE), FMOD_OK);
            assert_eq!(FMOD_Studio_EventInstance_GetPlaybackState(inst_fad, &mut state), FMOD_OK);
            assert_eq!(state, FMOD_STUDIO_PLAYBACK_STOPPED);
            assert_eq!(mix_system_frames(core, &mut out_fad), Ok(16));
            assert!(out_fad.iter().all(|&v| v == 0.0));
            assert_eq!(FMOD_Studio_EventInstance_Release(inst_fad), FMOD_OK);

            // Fire-and-forget one-shot (`Start` then immediate `Release`) keeps mixing until done!
            let mut inst_ff = 0usize;
            assert_eq!(FMOD_Studio_EventDescription_CreateInstance(ev_pcm, &mut inst_ff), FMOD_OK);
            assert_eq!(FMOD_Studio_EventInstance_Start(inst_ff), FMOD_OK);
            assert_eq!(FMOD_Studio_EventInstance_Release(inst_ff), FMOD_OK);
            assert_eq!(FMOD_Studio_EventInstance_IsValid(inst_ff), 0);
            let mut ff_mix = [0.0f32; 16 * 2];
            assert_eq!(mix_system_frames(core, &mut ff_mix), Ok(16));
            assert!((ff_mix[0] - 0.8).abs() < 1e-3, "fire-and-forget L got {}", ff_mix[0]);
            assert_eq!(FMOD_Studio_Bus_StopAllEvents(bus_master, FMOD_STUDIO_STOP_IMMEDIATE), FMOD_OK);
            assert_eq!(FMOD_Studio_System_Update(studio), FMOD_OK);

            // --- C) IMA-ADPCM 3D event + Studio 3D listener + automatic Distance param ---
            let listener_attrs = Fmod3DAttributes {
                position: FmodVector { x: 0.0, y: 0.0, z: 0.0 },
                velocity: FmodVector { x: 0.0, y: 0.0, z: 0.0 },
                forward: FmodVector { x: 0.0, y: 0.0, z: 1.0 },
                up: FmodVector { x: 0.0, y: 1.0, z: 0.0 },
            };
            assert_eq!(
                FMOD_Studio_System_SetListenerAttributes(studio, 0, &listener_attrs, std::ptr::null()),
                FMOD_OK
            );
            assert_eq!(call_studio_sys_set_listener_weight(studio, 0, 1.0), FMOD_OK);

            let mut inst_3d = 0usize;
            assert_eq!(FMOD_Studio_EventDescription_CreateInstance(ev_ima, &mut inst_3d), FMOD_OK);
            // Position 2m to listener's right (+X): atten = 1/2 = 0.5, pan = +1.0 -> L = 0.0, R = 0.8 * 0.5 = 0.4
            let src_attrs = Fmod3DAttributes {
                position: FmodVector { x: 2.0, y: 0.0, z: 0.0 },
                ..Default::default()
            };
            assert_eq!(FMOD_Studio_EventInstance_Set3DAttributes(inst_3d, &src_attrs), FMOD_OK);
            assert_eq!(FMOD_Studio_EventInstance_Start(inst_3d), FMOD_OK);

            // Automatic Distance parameter reflects 3D distance = 2.0!
            assert_eq!(FMOD_Studio_EventInstance_GetParameterByName(inst_3d, b"Distance\0".as_ptr(), &mut pv, &mut pfv), FMOD_OK);
            assert!((pfv - 2.0).abs() < 1e-4, "expected auto Distance=2.0, got {pfv}");

            let mut out_3d = [0.0f32; 8 * 2];
            assert_eq!(mix_system_frames(core, &mut out_3d), Ok(8));
            assert!(out_3d[0].abs() < 1e-3, "3D IMA L expected 0, got {}", out_3d[0]);
            assert!((out_3d[1] - 0.4).abs() < 1e-3, "3D IMA R expected 0.4, got {}", out_3d[1]);

            // --- D) Bank Unload stops active audio and invalidates bank/event/instance/bus/VCA ---
            assert_eq!(FMOD_Studio_Bank_Unload(sfx_bank), FMOD_OK);
            assert_eq!(FMOD_Studio_Bank_IsValid(sfx_bank), 0);
            assert_eq!(FMOD_Studio_EventDescription_IsValid(ev_ima), 0);
            assert_eq!(FMOD_Studio_EventInstance_IsValid(inst_3d), 0);
            assert_eq!(FMOD_Studio_Bus_IsValid(bus_sfx), 0);
            assert_eq!(FMOD_Studio_VCA_IsValid(vca_sfx), 0);
            // Master bus stays valid!
            assert_eq!(FMOD_Studio_Bus_IsValid(bus_master), 1);

            // Audio is immediately silent after bank unload.
            assert_eq!(mix_system_frames(core, &mut out_3d), Ok(8));
            assert!(out_3d.iter().all(|&v| v == 0.0));

            let mut ev_after = 0usize;
            assert_eq!(
                FMOD_Studio_System_GetEvent(studio, b"event:/SFX/SpatialIMA\0".as_ptr(), &mut ev_after),
                FMOD_ERR_EVENT_NOTFOUND
            );

            let _ = std::fs::remove_file(&strings_file);
            let _ = std::fs::remove_dir(&tmp_dir);
            assert_eq!(FMOD_Studio_System_Release(studio), FMOD_OK);
        }
    }

    #[test]
    fn fmod_studio_standalone_fsb5_bank_and_parse_id() {
        unsafe {
            let mut parsed_guid = FmodGuid::ZERO;
            assert_eq!(
                FMOD_Studio_ParseID(
                    b"{01234567-89ab-cdef-0123-456789abcdef}\0".as_ptr(),
                    &mut parsed_guid,
                ),
                FMOD_OK
            );
            assert_eq!(parsed_guid.data1, 0x0123_4567);
            assert_eq!(parsed_guid.data2, 0x89ab);
            assert_eq!(parsed_guid.data3, 0xcdef);
            assert_eq!(parsed_guid.data4, [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]);
            assert_eq!(
                FMOD_Studio_ParseID(b"not-a-guid\0".as_ptr(), &mut parsed_guid),
                FMOD_ERR_FORMAT
            );

            let mut studio = 0usize;
            let mut core = 0usize;
            assert_eq!(FMOD_Studio_System_Create(&mut studio, FMOD_VERSION), FMOD_OK);
            assert_eq!(FMOD_Studio_System_GetCoreSystem(studio, &mut core), FMOD_OK);
            assert_eq!(FMOD_System_SetOutput(core, FMOD_OUTPUTTYPE_NOSOUND_NRT), FMOD_OK);
            assert_eq!(FMOD_Studio_System_Initialize(studio, 16, 0, 0, 0), FMOD_OK);

            // Load a standalone FSB5 bank directly into FMOD Studio.
            let standalone_fsb = build_fsb5(
                1,
                FSB5_MODE_FADPCM,
                &[TestFsbSample {
                    name: "UI/Click".into(),
                    channels: 1,
                    sample_rate: 48_000,
                    num_frames: 64,
                    loop_points: None,
                    data: build_fadpcm_constant_frame(&[2]), // +2 * 4096 = +8192 = +0.25
                }],
            );
            let mut bank = 0usize;
            assert_eq!(
                FMOD_Studio_System_LoadBankMemory(
                    studio,
                    standalone_fsb.as_ptr(),
                    standalone_fsb.len() as i32,
                    FMOD_STUDIO_LOAD_MEMORY,
                    FMOD_STUDIO_LOAD_BANK_NORMAL,
                    &mut bank,
                ),
                FMOD_OK
            );

            let mut ev = 0usize;
            assert_eq!(
                FMOD_Studio_System_GetEvent(studio, b"event:/UI/Click\0".as_ptr(), &mut ev),
                FMOD_OK
            );
            let mut inst = 0usize;
            assert_eq!(FMOD_Studio_EventDescription_CreateInstance(ev, &mut inst), FMOD_OK);
            assert_eq!(FMOD_Studio_EventInstance_Start(inst), FMOD_OK);

            let mut mix = [0.0f32; 8 * 2];
            assert_eq!(mix_system_frames(core, &mut mix), Ok(8));
            for f in 0..8 {
                assert!((mix[f * 2] - 0.25).abs() < 1e-3, "L[{f}]={}", mix[f * 2]);
                assert!((mix[f * 2 + 1] - 0.25).abs() < 1e-3, "R[{f}]={}", mix[f * 2 + 1]);
            }

            assert_eq!(FMOD_Studio_System_UnloadAll(studio), FMOD_OK);
            assert_eq!(FMOD_Studio_Bank_IsValid(bank), 0);
            assert_eq!(FMOD_Studio_System_Release(studio), FMOD_OK);
        }
    }
}
