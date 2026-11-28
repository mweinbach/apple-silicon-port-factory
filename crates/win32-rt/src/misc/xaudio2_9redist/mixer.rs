//! XAudio2 software mixer core: PCM format parsing/decoding, Cytomic TPT state-variable &
//! 1-pole filters, send matrices, engine/voice state, CoreAudio/Null mastering output runtime,
//! and Win32 callback thread dispatch.

use super::xapo::{process_effect_chain, AlignedF32Buf, VoiceEffectSlot};
use crate::audio::backend::{self, OutputStream, StreamControl, StreamShared};
use crate::audio::com::{GUID, KSDATAFORMAT_SUBTYPE_IEEE_FLOAT, KSDATAFORMAT_SUBTYPE_PCM};
use crate::audio::format::{
    self, decode_pcm_buffer_f32, SampleType, WAVEFORMATEX, WAVEFORMATEXTENSIBLE,
    WAVE_FORMAT_EXTENSIBLE, WAVE_FORMAT_IEEE_FLOAT, WAVE_FORMAT_PCM,
};
use crate::audio::ring::Ring;
use crate::com::com_call_slot_raw;
use crate::strings::rd;
use std::collections::{HashMap, VecDeque};
use std::f32::consts::FRAC_PI_2;
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{mpsc, Arc, Condvar, Mutex, Weak};
use std::thread::ThreadId;
use std::time::Duration;

// ============================================================================
// Constants & HRESULTs
// ============================================================================

pub(super) const S_OK: i32 = 0;
pub(super) const E_NOINTERFACE: i32 = 0x8000_4002u32 as i32;
pub(super) const E_POINTER: i32 = 0x8000_4003u32 as i32;
pub(super) const E_OUTOFMEMORY: i32 = 0x8007_000Eu32 as i32;
pub(super) const E_INVALIDARG: i32 = 0x8007_0057u32 as i32;
pub(super) const XAUDIO2_E_INVALID_CALL: i32 = 0x8896_0001u32 as i32;

pub(super) const IID_IXAUDIO2: GUID = GUID::new(
    0x2B02_E3CF,
    0x2E0B,
    0x4EC3,
    [0xBE, 0x45, 0x1B, 0x2A, 0x3F, 0xE7, 0x21, 0x0D],
);
pub(super) const IID_IXAUDIO2_7: GUID = GUID::new(
    0x8BCF_1F58,
    0x9FE7,
    0x4583,
    [0x8A, 0xC6, 0xE2, 0xAD, 0xC4, 0x65, 0xC8, 0xBB],
);

pub(super) const XAUDIO2_MAX_AUDIO_CHANNELS: u32 = 64;
pub(super) const XAUDIO2_MIN_SAMPLE_RATE: u32 = 1_000;
pub(super) const XAUDIO2_MAX_SAMPLE_RATE: u32 = 768_000;
pub(super) const XAUDIO2_MAX_VOLUME_LEVEL: f32 = 16_777_216.0;
pub(super) const XAUDIO2_MIN_FREQ_RATIO: f32 = 1.0 / 1024.0;
pub(super) const XAUDIO2_MAX_FREQ_RATIO: f32 = 1024.0;
pub(super) const XAUDIO2_DEFAULT_FREQ_RATIO: f32 = 2.0;
pub(super) const XAUDIO2_MAX_QUEUED_BUFFERS: usize = 1024;

pub(super) const XAUDIO2_VOICE_NOPITCH: u32 = 0x0002;
pub(super) const XAUDIO2_PLAY_TAILS: u32 = 0x0020;
pub(super) const XAUDIO2_END_OF_STREAM: u32 = 0x0040;
pub(super) const XAUDIO2_VOICE_NOSAMPLESPLAYED: u32 = 0x0100;
pub(super) const XAUDIO2_LOOP_INFINITE: u32 = 255;

pub(super) const FILTER_LOW_PASS: u32 = 0;
pub(super) const FILTER_BAND_PASS: u32 = 1;
pub(super) const FILTER_HIGH_PASS: u32 = 2;
pub(super) const FILTER_NOTCH: u32 = 3;
pub(super) const FILTER_LOW_PASS_ONE_POLE: u32 = 4;
pub(super) const FILTER_HIGH_PASS_ONE_POLE: u32 = 5;

// ============================================================================
// Windows ABI Structures (XAudio2 headers use `#pragma pack(push, 1)`)
// ============================================================================

#[repr(C, packed)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(super) struct XAudio2VoiceDetails {
    pub(super) creation_flags: u32,
    pub(super) active_flags: u32,
    pub(super) input_channels: u32,
    pub(super) input_sample_rate: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub(super) struct XAudio2SendDescriptor {
    pub(super) flags: u32,
    pub(super) output_voice: *mut VoiceHeader,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub(super) struct XAudio2VoiceSends {
    pub(super) send_count: u32,
    pub(super) sends: *const XAudio2SendDescriptor,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub(super) struct XAudio2FilterParameters {
    pub(super) filter_type: u32,
    pub(super) frequency: f32,
    pub(super) one_over_q: f32,
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
pub(super) struct XAudio2Buffer {
    pub(super) flags: u32,
    pub(super) audio_bytes: u32,
    pub(super) audio_data: *const u8,
    pub(super) play_begin: u32,
    pub(super) play_length: u32,
    pub(super) loop_begin: u32,
    pub(super) loop_length: u32,
    pub(super) loop_count: u32,
    pub(super) context: usize,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(super) struct XAudio2VoiceState {
    pub(super) current_buffer_context: usize,
    pub(super) buffers_queued: u32,
    pub(super) samples_played: u64,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Default)]
#[allow(dead_code)]
pub(super) struct XAudio2PerformanceData {
    pub(super) audio_cycles_since_last_query: u64,
    pub(super) total_cycles_since_last_query: u64,
    pub(super) minimum_cycles_per_quantum: u32,
    pub(super) maximum_cycles_per_quantum: u32,
    pub(super) memory_usage_in_bytes: u32,
    pub(super) current_latency_in_samples: u32,
    pub(super) glitches_since_engine_started: u32,
    pub(super) active_source_voice_count: u32,
    pub(super) total_source_voice_count: u32,
    pub(super) active_submix_voice_count: u32,
    pub(super) active_resampler_count: u32,
    pub(super) active_matrix_mix_count: u32,
    pub(super) active_xma_source_voices: u32,
    pub(super) active_xma_streams: u32,
}

const _: () = assert!(std::mem::size_of::<XAudio2VoiceDetails>() == 16);
const _: () = assert!(std::mem::size_of::<XAudio2SendDescriptor>() == 12);
const _: () = assert!(std::mem::size_of::<XAudio2VoiceSends>() == 12);
const _: () = assert!(std::mem::size_of::<XAudio2FilterParameters>() == 12);
const _: () = assert!(std::mem::size_of::<XAudio2Buffer>() == 44);
const _: () = assert!(std::mem::size_of::<XAudio2VoiceState>() == 20);
const _: () = assert!(std::mem::size_of::<XAudio2PerformanceData>() == 64);

// ============================================================================
// PCM Format & Sample Decoding (backed by `crate::audio::format`)
// ============================================================================

const WAVE_FORMAT_ADPCM: u16 = 0x0002;
const WAVE_FORMAT_ALAW: u16 = 0x0006;
const WAVE_FORMAT_MULAW: u16 = 0x0007;
const WAVE_FORMAT_IMA_ADPCM: u16 = 0x0011;

const KSDATAFORMAT_SUBTYPE_ADPCM: GUID =
    GUID::new(0x0000_0002, 0x0000, 0x0010, [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71]);
const KSDATAFORMAT_SUBTYPE_ALAW: GUID =
    GUID::new(0x0000_0006, 0x0000, 0x0010, [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71]);
const KSDATAFORMAT_SUBTYPE_MULAW: GUID =
    GUID::new(0x0000_0007, 0x0000, 0x0010, [0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71]);

const MS_ADPCM_ADAPTATION_TABLE: [i32; 16] = [
    230, 230, 230, 230, 307, 409, 512, 614, 768, 614, 512, 409, 307, 230, 230, 230,
];
const MS_ADPCM_DEFAULT_COEFS: [(i16, i16); 7] = [
    (256, 0),
    (512, -256),
    (0, 0),
    (192, 64),
    (240, 0),
    (460, -208),
    (392, -232),
];

const IMA_INDEX_TABLE: [i32; 16] = [-1, -1, -1, -1, 2, 4, 6, 8, -1, -1, -1, -1, 2, 4, 6, 8];
const IMA_STEP_TABLE: [i32; 89] = [
    7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 19, 21, 23, 25, 28, 31, 34, 37, 41, 45, 50, 55, 60, 66,
    73, 80, 88, 97, 107, 118, 130, 143, 157, 173, 190, 209, 230, 253, 279, 307, 337, 371, 408, 449,
    494, 544, 598, 658, 724, 796, 876, 963, 1060, 1166, 1282, 1411, 1552, 1707, 1878, 2066, 2272,
    2499, 2749, 3024, 3327, 3660, 4026, 4428, 4871, 5358, 5894, 6484, 7132, 7845, 8630, 9493,
    10442, 11487, 12635, 13899, 15289, 16818, 18500, 20350, 22385, 24623, 27086, 29794, 32767,
];

#[derive(Clone, Copy, Debug)]
pub(super) enum WaveEncoding {
    Pcm(SampleType),
    MsAdpcm {
        samples_per_block: usize,
        coefs: [(i16, i16); 16],
        num_coefs: usize,
    },
    ImaAdpcm {
        samples_per_block: usize,
    },
    ALaw,
    MuLaw,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct ParsedWaveFormat {
    pub(super) channels: usize,
    pub(super) sample_rate: u32,
    pub(super) block_align: usize,
    pub(super) sample_kind: SampleType,
    pub(super) encoding: WaveEncoding,
}

pub(super) unsafe fn parse_wave_format(
    ptr: *const WAVEFORMATEX,
) -> Result<ParsedWaveFormat, i32> {
    if ptr.is_null() {
        return Err(E_POINTER);
    }
    let wfx = rd(ptr);
    let ch = wfx.nChannels as u32;
    let sr = wfx.nSamplesPerSec;
    let bits = wfx.wBitsPerSample;
    let cb_size = wfx.cbSize;
    let raw_tag = wfx.wFormatTag;
    let _avg_bps = wfx.nAvgBytesPerSec;

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
        let ext = rd(ptr as *const WAVEFORMATEXTENSIBLE);
        let sub = { ext.SubFormat };
        let valid_bits = ext.wValidBitsPerSample;
        let _mask = ext.dwChannelMask;
        let _fmt_copy = ext.Format;
        if valid_bits > bits {
            return Err(XAUDIO2_E_INVALID_CALL);
        }
        if sub == KSDATAFORMAT_SUBTYPE_PCM {
            WAVE_FORMAT_PCM
        } else if sub == KSDATAFORMAT_SUBTYPE_IEEE_FLOAT {
            WAVE_FORMAT_IEEE_FLOAT
        } else if sub == KSDATAFORMAT_SUBTYPE_ADPCM {
            WAVE_FORMAT_ADPCM
        } else if sub == KSDATAFORMAT_SUBTYPE_ALAW {
            WAVE_FORMAT_ALAW
        } else if sub == KSDATAFORMAT_SUBTYPE_MULAW {
            WAVE_FORMAT_MULAW
        } else {
            // Desktop Windows XAudio2.9 rejects unsupported WAVE_FORMAT_EXTENSIBLE SubFormats
            // (including Xbox-only XMA2) with XAUDIO2_E_INVALID_CALL.
            log::debug!(
                target: "audio",
                "XAudio2 CreateSourceVoice: rejecting unsupported WAVE_FORMAT_EXTENSIBLE SubFormat {sub:?} -> XAUDIO2_E_INVALID_CALL"
            );
            return Err(XAUDIO2_E_INVALID_CALL);
        }
    } else {
        raw_tag
    };

    match (tag, bits) {
        (WAVE_FORMAT_PCM, 8)
        | (WAVE_FORMAT_PCM, 16)
        | (WAVE_FORMAT_PCM, 24)
        | (WAVE_FORMAT_PCM, 32)
        | (WAVE_FORMAT_IEEE_FLOAT, 32)
        | (WAVE_FORMAT_IEEE_FLOAT, 64) => {
            let sample_kind = match (tag, bits) {
                (WAVE_FORMAT_PCM, 8) => SampleType::U8,
                (WAVE_FORMAT_PCM, 16) => SampleType::I16,
                (WAVE_FORMAT_PCM, 24) => SampleType::I24,
                (WAVE_FORMAT_PCM, 32) => SampleType::I32,
                (WAVE_FORMAT_IEEE_FLOAT, 32) => SampleType::F32,
                _ => SampleType::F64,
            };
            let expected_align = (ch as usize) * ((bits as usize) / 8);
            let block_align = if wfx.nBlockAlign == 0 {
                expected_align
            } else if (wfx.nBlockAlign as usize) != expected_align {
                return Err(XAUDIO2_E_INVALID_CALL);
            } else {
                wfx.nBlockAlign as usize
            };
            Ok(ParsedWaveFormat {
                channels: ch as usize,
                sample_rate: sr,
                block_align,
                sample_kind,
                encoding: WaveEncoding::Pcm(sample_kind),
            })
        }
        (WAVE_FORMAT_ADPCM, 4) => {
            if ch > 2 || wfx.nBlockAlign < (ch as u16) * 7 {
                return Err(XAUDIO2_E_INVALID_CALL);
            }
            let block_align = wfx.nBlockAlign as usize;
            let extra_ptr = (ptr as *const u8).add(std::mem::size_of::<WAVEFORMATEX>());
            let mut coefs = [(0i16, 0i16); 16];
            for (i, &c) in MS_ADPCM_DEFAULT_COEFS.iter().enumerate() {
                coefs[i] = c;
            }
            let mut num_coefs = MS_ADPCM_DEFAULT_COEFS.len();
            let mut samples_per_block = ((block_align - 7 * ch as usize) * 2 / ch as usize) + 2;
            if raw_tag == WAVE_FORMAT_ADPCM && cb_size >= 4 {
                let spb = std::ptr::read_unaligned(extra_ptr as *const u16) as usize;
                let nc = std::ptr::read_unaligned(extra_ptr.add(2) as *const u16) as usize;
                if spb >= 2 {
                    samples_per_block = spb;
                }
                if nc > 0 && cb_size as usize >= 4 + nc * 4 {
                    num_coefs = nc.min(coefs.len());
                    for i in 0..num_coefs {
                        let c1 = std::ptr::read_unaligned(extra_ptr.add(4 + i * 4) as *const i16);
                        let c2 = std::ptr::read_unaligned(extra_ptr.add(6 + i * 4) as *const i16);
                        coefs[i] = (c1, c2);
                    }
                }
            }
            Ok(ParsedWaveFormat {
                channels: ch as usize,
                sample_rate: sr,
                block_align,
                sample_kind: SampleType::I16,
                encoding: WaveEncoding::MsAdpcm {
                    samples_per_block,
                    coefs,
                    num_coefs,
                },
            })
        }
        (WAVE_FORMAT_IMA_ADPCM, 4) => {
            if ch > 2 || (wfx.nBlockAlign as usize) < (ch as usize) * 4 {
                return Err(XAUDIO2_E_INVALID_CALL);
            }
            let block_align = wfx.nBlockAlign as usize;
            let mut samples_per_block = ((block_align - 4 * ch as usize) * 2 / ch as usize) + 1;
            if raw_tag == WAVE_FORMAT_IMA_ADPCM && cb_size >= 2 {
                let extra_ptr = (ptr as *const u8).add(std::mem::size_of::<WAVEFORMATEX>());
                let spb = std::ptr::read_unaligned(extra_ptr as *const u16) as usize;
                if spb >= 1 {
                    samples_per_block = spb;
                }
            }
            Ok(ParsedWaveFormat {
                channels: ch as usize,
                sample_rate: sr,
                block_align,
                sample_kind: SampleType::I16,
                encoding: WaveEncoding::ImaAdpcm { samples_per_block },
            })
        }
        (WAVE_FORMAT_ALAW, 8) | (WAVE_FORMAT_MULAW, 8) => {
            let expected_align = ch as usize;
            let block_align = if wfx.nBlockAlign == 0 {
                expected_align
            } else if (wfx.nBlockAlign as usize) != expected_align {
                return Err(XAUDIO2_E_INVALID_CALL);
            } else {
                wfx.nBlockAlign as usize
            };
            Ok(ParsedWaveFormat {
                channels: ch as usize,
                sample_rate: sr,
                block_align,
                sample_kind: SampleType::I16,
                encoding: if tag == WAVE_FORMAT_ALAW {
                    WaveEncoding::ALaw
                } else {
                    WaveEncoding::MuLaw
                },
            })
        }
        _ => {
            // Desktop Windows XAudio2.9 rejects unsupported/invalid format tags and bit depths
            // with XAUDIO2_E_INVALID_CALL (Microsoft Learn: IXAudio2::CreateSourceVoice).
            log::debug!(
                target: "audio",
                "XAudio2 CreateSourceVoice: rejecting unsupported format tag {tag:#x} bits {bits} -> XAUDIO2_E_INVALID_CALL"
            );
            Err(XAUDIO2_E_INVALID_CALL)
        }
    }
}

#[inline]
fn decode_alaw_sample(b: u8) -> f32 {
    let x = b ^ 0x55;
    let mut t = ((x & 0x0F) as i32) << 4;
    let seg = ((x & 0x70) >> 4) as i32;
    match seg {
        0 => t += 8,
        1 => t += 0x108,
        _ => {
            t += 0x108;
            t <<= seg - 1;
        }
    }
    let pcm = if (x & 0x80) != 0 { t } else { -t };
    (pcm as f32) / 32768.0
}

#[inline]
fn decode_mulaw_sample(b: u8) -> f32 {
    let x = !b;
    let sign = x & 0x80;
    let exp = ((x >> 4) & 0x07) as i32;
    let mant = (x & 0x0F) as i32;
    let mut t = ((mant << 3) + 0x84) << exp;
    t -= 0x84;
    let pcm = if sign != 0 { -t } else { t };
    (pcm as f32) / 32768.0
}

fn decode_ms_adpcm(
    raw: &[u8],
    channels: usize,
    block_align: usize,
    samples_per_block: usize,
    coefs: &[(i16, i16)],
) -> (Vec<f32>, usize) {
    // Match FFmpeg's `libavcodec/adpcm.c` upper bound (`INT_MAX / 768`) so sustained
    // max-magnitude nibbles cannot overflow `adaptation_table[nibble] * delta`.
    const MS_ADPCM_MAX_DELTA: i64 = (i32::MAX / 768) as i64;

    let num_blocks = raw.len() / block_align.max(1);
    let max_spb = 2usize.saturating_add(
        block_align
            .saturating_sub(7 * channels)
            .saturating_mul(2)
            / channels.max(1),
    );
    let cap_frames = samples_per_block.min(max_spb);
    let mut out = Vec::with_capacity(
        num_blocks
            .saturating_mul(cap_frames)
            .saturating_mul(channels),
    );
    let mut total_frames = 0usize;

    for blk_idx in 0..num_blocks {
        let blk = &raw[blk_idx * block_align..(blk_idx + 1) * block_align];
        if blk.len() < 7 * channels {
            break;
        }
        let mut pred = [0usize; 2];
        let mut delta = [0i32; 2];
        let mut samp1 = [0i32; 2];
        let mut samp2 = [0i32; 2];
        let mut pos = 0usize;
        for c in 0..channels {
            pred[c] = (blk[pos] as usize).min(coefs.len().saturating_sub(1));
            pos += 1;
        }
        for c in 0..channels {
            delta[c] = i16::from_le_bytes([blk[pos], blk[pos + 1]]) as i32;
            pos += 2;
        }
        for c in 0..channels {
            samp1[c] = i16::from_le_bytes([blk[pos], blk[pos + 1]]) as i32;
            pos += 2;
        }
        for c in 0..channels {
            samp2[c] = i16::from_le_bytes([blk[pos], blk[pos + 1]]) as i32;
            pos += 2;
        }

        // First two samples per channel come directly from the block header (samp2 then samp1).
        for c in 0..channels {
            out.push((samp2[c] as f32) / 32768.0);
        }
        total_frames += 1;
        if samples_per_block >= 2 {
            for c in 0..channels {
                out.push((samp1[c] as f32) / 32768.0);
            }
            total_frames += 1;
        }

        let mut frames_in_blk = 2usize;
        let mut ch_cur = 0usize;
        for &byte in &blk[pos..] {
            for shift in [4u8, 0u8] {
                if frames_in_blk >= samples_per_block {
                    break;
                }
                let nib_u = ((byte >> shift) & 0x0F) as usize;
                let nib_s = if nib_u >= 8 { nib_u as i64 - 16 } else { nib_u as i64 };
                let (c1, c2) = coefs[pred[ch_cur]];
                let predicted =
                    ((samp1[ch_cur] as i64 * c1 as i64) + (samp2[ch_cur] as i64 * c2 as i64)) >> 8;
                let new_samp = (predicted + nib_s * delta[ch_cur] as i64).clamp(-32768, 32767) as i32;
                samp2[ch_cur] = samp1[ch_cur];
                samp1[ch_cur] = new_samp;
                delta[ch_cur] = (((MS_ADPCM_ADAPTATION_TABLE[nib_u] as i64)
                    * (delta[ch_cur] as i64))
                    >> 8)
                .clamp(16, MS_ADPCM_MAX_DELTA) as i32;
                out.push((new_samp as f32) / 32768.0);
                ch_cur += 1;
                if ch_cur == channels {
                    ch_cur = 0;
                    frames_in_blk += 1;
                    total_frames += 1;
                }
            }
        }
    }
    (out, total_frames)
}

fn decode_ima_adpcm(
    raw: &[u8],
    channels: usize,
    block_align: usize,
    samples_per_block: usize,
) -> (Vec<f32>, usize) {
    let num_blocks = raw.len() / block_align.max(1);
    let mut out = Vec::with_capacity(num_blocks * samples_per_block * channels);
    let mut total_frames = 0usize;

    let step_nibble = |nibble: u8, pred: &mut i32, idx: &mut i32| -> i32 {
        let step = IMA_STEP_TABLE[*idx as usize];
        let mut diff = step >> 3;
        if (nibble & 1) != 0 { diff += step >> 2; }
        if (nibble & 2) != 0 { diff += step >> 1; }
        if (nibble & 4) != 0 { diff += step; }
        if (nibble & 8) != 0 { *pred -= diff; } else { *pred += diff; }
        *pred = (*pred).clamp(-32768, 32767);
        *idx = (*idx + IMA_INDEX_TABLE[(nibble & 0x0F) as usize]).clamp(0, 88);
        *pred
    };

    for blk_idx in 0..num_blocks {
        let blk = &raw[blk_idx * block_align..(blk_idx + 1) * block_align];
        if blk.len() < 4 * channels {
            break;
        }
        let mut pred = [0i32; 2];
        let mut idx = [0i32; 2];
        for c in 0..channels {
            let off = c * 4;
            pred[c] = i16::from_le_bytes([blk[off], blk[off + 1]]) as i32;
            idx[c] = (blk[off + 2] as i32).clamp(0, 88);
        }
        for c in 0..channels {
            out.push((pred[c] as f32) / 32768.0);
        }
        let mut frames_in_blk = 1usize;
        total_frames += 1;

        let mut pos = 4 * channels;
        if channels == 1 {
            while pos < blk.len() && frames_in_blk < samples_per_block {
                let byte = blk[pos];
                pos += 1;
                let s0 = step_nibble(byte & 0x0F, &mut pred[0], &mut idx[0]);
                out.push((s0 as f32) / 32768.0);
                frames_in_blk += 1;
                total_frames += 1;
                if frames_in_blk < samples_per_block {
                    let s1 = step_nibble(byte >> 4, &mut pred[0], &mut idx[0]);
                    out.push((s1 as f32) / 32768.0);
                    frames_in_blk += 1;
                    total_frames += 1;
                }
            }
        } else {
            // Stereo IMA ADPCM interleaves 4-byte (8-sample) chunks per channel.
            while pos + 8 <= blk.len() && frames_in_blk < samples_per_block {
                let mut l_chunk = [0.0f32; 8];
                let mut r_chunk = [0.0f32; 8];
                for i in 0..4 {
                    let bl = blk[pos + i];
                    l_chunk[i * 2] = (step_nibble(bl & 0x0F, &mut pred[0], &mut idx[0]) as f32) / 32768.0;
                    l_chunk[i * 2 + 1] = (step_nibble(bl >> 4, &mut pred[0], &mut idx[0]) as f32) / 32768.0;
                    let br = blk[pos + 4 + i];
                    r_chunk[i * 2] = (step_nibble(br & 0x0F, &mut pred[1], &mut idx[1]) as f32) / 32768.0;
                    r_chunk[i * 2 + 1] = (step_nibble(br >> 4, &mut pred[1], &mut idx[1]) as f32) / 32768.0;
                }
                pos += 8;
                for i in 0..8 {
                    if frames_in_blk >= samples_per_block {
                        break;
                    }
                    out.push(l_chunk[i]);
                    out.push(r_chunk[i]);
                    frames_in_blk += 1;
                    total_frames += 1;
                }
            }
        }
    }
    (out, total_frames)
}

pub(super) fn decode_pcm_to_f32(fmt: &ParsedWaveFormat, raw: &[u8]) -> (Vec<f32>, usize) {
    match &fmt.encoding {
        WaveEncoding::Pcm(kind) => {
            let frames = raw.len() / fmt.block_align.max(1);
            let mut out = Vec::new();
            decode_pcm_buffer_f32(*kind, &raw[..frames * fmt.block_align], &mut out);
            (out, frames)
        }
        WaveEncoding::MsAdpcm {
            samples_per_block,
            coefs,
            num_coefs,
        } => decode_ms_adpcm(raw, fmt.channels, fmt.block_align, *samples_per_block, &coefs[..*num_coefs]),
        WaveEncoding::ImaAdpcm { samples_per_block } => {
            decode_ima_adpcm(raw, fmt.channels, fmt.block_align, *samples_per_block)
        }
        WaveEncoding::ALaw => {
            let frames = raw.len() / fmt.block_align.max(1);
            let used = frames * fmt.block_align;
            let out = raw[..used].iter().map(|&b| decode_alaw_sample(b)).collect();
            (out, frames)
        }
        WaveEncoding::MuLaw => {
            let frames = raw.len() / fmt.block_align.max(1);
            let used = frames * fmt.block_align;
            let out = raw[..used].iter().map(|&b| decode_mulaw_sample(b)).collect();
            (out, frames)
        }
    }
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
pub(super) struct VoiceFilter {
    pub(super) params: XAudio2FilterParameters,
    active: bool,
    channels: Vec<ChannelFilterState>,
}

impl VoiceFilter {
    pub(super) fn new(channel_count: usize) -> Self {
        Self {
            params: XAudio2FilterParameters::default(),
            active: false,
            channels: vec![ChannelFilterState::default(); channel_count.max(1)],
        }
    }

    pub(super) fn set(&mut self, raw: XAudio2FilterParameters) -> Result<(), i32> {
        let ftype = raw.filter_type;
        let freq = raw.frequency;
        let ooq = raw.one_over_q;
        if ftype > FILTER_HIGH_PASS_ONE_POLE || !freq.is_finite() || !ooq.is_finite() {
            return Err(XAUDIO2_E_INVALID_CALL);
        }
        let clamped_freq = freq.clamp(0.0, 1.0);
        let clamped_ooq =
            if ftype == FILTER_LOW_PASS_ONE_POLE || ftype == FILTER_HIGH_PASS_ONE_POLE {
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
pub(super) fn default_xaudio2_matrix(src_ch: usize, dst_ch: usize) -> Vec<f32> {
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
pub(super) struct VoiceSend {
    pub(super) dest_id: u64,
    pub(super) dest_channels: usize,
    pub(super) matrix: Vec<f32>,
    pub(super) output_filter: VoiceFilter,
}

impl VoiceSend {
    pub(super) fn new(dest_id: u64, src_ch: usize, dest_ch: usize) -> Self {
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
pub(super) enum CallbackEvent {
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

unsafe fn invoke_callback_event(ev: &CallbackEvent) {
    match ev {
        CallbackEvent::EnginePassStart(cbs) => {
            for &cb in cbs {
                let _ = com_call_slot_raw(cb as *mut c_void, 0, &[]);
            }
        }
        CallbackEvent::EnginePassEnd(cbs) => {
            for &cb in cbs {
                let _ = com_call_slot_raw(cb as *mut c_void, 1, &[]);
            }
        }
        CallbackEvent::VoiceStreamEnd { cb, alive } => {
            if alive.load(Ordering::Acquire) {
                let _ = com_call_slot_raw(*cb as *mut c_void, 2, &[]);
            }
        }
        CallbackEvent::VoiceBufferStart { cb, alive, ctx } => {
            if alive.load(Ordering::Acquire) {
                let _ = com_call_slot_raw(*cb as *mut c_void, 3, &[*ctx as u64]);
            }
        }
        CallbackEvent::VoiceBufferEnd { cb, alive, ctx } => {
            if alive.load(Ordering::Acquire) {
                let _ = com_call_slot_raw(*cb as *mut c_void, 4, &[*ctx as u64]);
            }
        }
        CallbackEvent::VoiceLoopEnd { cb, alive, ctx } => {
            if alive.load(Ordering::Acquire) {
                let _ = com_call_slot_raw(*cb as *mut c_void, 5, &[*ctx as u64]);
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
// Engine & Voice State
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub(super) enum VoiceKind {
    Mastering = 0,
    Submix = 1,
    Source = 2,
}

#[repr(C)]
pub(super) struct VoiceHeader {
    #[allow(dead_code)]
    pub(super) vtbl: *const crate::Ptr,
    pub(super) id: u64,
    pub(super) kind: VoiceKind,
    pub(super) engine: Arc<EngineCore>,
}

#[repr(transparent)]
pub(super) struct MasteringVoice(pub(super) VoiceHeader);

#[repr(transparent)]
pub(super) struct SubmixVoice(#[allow(dead_code)] VoiceHeader);

#[repr(transparent)]
pub(super) struct SourceVoice(pub(super) VoiceHeader);

#[repr(C)]
pub struct XAudio2Engine {
    pub vtbl: *const [crate::Ptr; 13],
    pub(super) refs: AtomicU32,
    pub(super) core: Arc<EngineCore>,
}

pub(super) struct QueuedSourceBuffer {
    pub(super) samples: Vec<f32>,
    pub(super) play_begin: usize,
    pub(super) play_end: usize,
    pub(super) loop_begin: usize,
    pub(super) loop_end: usize,
    pub(super) loops_remaining: u32,
    pub(super) end_of_stream: bool,
    pub(super) context: usize,
    pub(super) started: bool,
    pub(super) pos: f64,
}

pub(super) struct SourceVoiceState {
    pub(super) creation_flags: u32,
    pub(super) format: ParsedWaveFormat,
    pub(super) max_freq_ratio: f32,
    pub(super) freq_ratio: f32,
    pub(super) volume: f32,
    pub(super) channel_volumes: Vec<f32>,
    pub(super) filter: VoiceFilter,
    pub(super) effects: Vec<VoiceEffectSlot>,
    pub(super) use_default_master_send: bool,
    pub(super) sends: Vec<VoiceSend>,
    pub(super) callback: usize,
    pub(super) callback_alive: Arc<AtomicBool>,
    pub(super) running: bool,
    pub(super) play_tails: bool,
    pub(super) discontinuity: bool,
    pub(super) samples_played: u64,
    pub(super) samples_played_frac: f64,
    pub(super) queue: VecDeque<QueuedSourceBuffer>,
}

impl SourceVoiceState {
    pub(super) fn output_channels(&self) -> usize {
        self.effects
            .last()
            .map(|e| e.output_channels)
            .unwrap_or(self.format.channels)
    }
}

pub(super) struct SubmixVoiceState {
    pub(super) creation_flags: u32,
    pub(super) channels: usize,
    pub(super) sample_rate: u32,
    pub(super) processing_stage: u32,
    pub(super) volume: f32,
    pub(super) channel_volumes: Vec<f32>,
    pub(super) filter: VoiceFilter,
    pub(super) effects: Vec<VoiceEffectSlot>,
    pub(super) use_default_master_send: bool,
    pub(super) sends: Vec<VoiceSend>,
    pub(super) mix_buf: Vec<f32>,
}

impl SubmixVoiceState {
    pub(super) fn output_channels(&self) -> usize {
        self.effects
            .last()
            .map(|e| e.output_channels)
            .unwrap_or(self.channels)
    }
}

pub(super) struct MasteringVoiceState {
    pub(super) id: u64,
    pub(super) creation_flags: u32,
    pub(super) channels: usize,
    pub(super) sample_rate: u32,
    pub(super) channel_mask: u32,
    pub(super) volume: f32,
    pub(super) channel_volumes: Vec<f32>,
    pub(super) filter: VoiceFilter,
    pub(super) effects: Vec<VoiceEffectSlot>,
}

pub(super) struct EngineState {
    pub(super) master: Option<MasteringVoiceState>,
    pub(super) submixes: HashMap<u64, SubmixVoiceState>,
    pub(super) sources: HashMap<u64, SourceVoiceState>,
    pub(super) engine_callbacks: Vec<usize>,
    pub(super) source_glitches: u32,
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

pub(super) struct MasteringRuntime {
    stop_flag: Arc<AtomicBool>,
    pub(super) wake: Arc<(Mutex<()>, Condvar)>,
    pub(super) shared: Arc<StreamShared>,
    pub(super) control: StreamControl,
    #[allow(dead_code)]
    pub(super) device_uid: String,
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

pub(super) struct EngineCore {
    next_voice_id: AtomicU64,
    pub(super) running: AtomicBool,
    pub(super) state: Mutex<EngineState>,
    pub(super) runtime: Mutex<Option<MasteringRuntime>>,
    callbacks: Mutex<Option<CallbackDispatcher>>,
}

impl EngineCore {
    pub(super) fn new() -> Arc<Self> {
        Arc::new(Self {
            next_voice_id: AtomicU64::new(1),
            running: AtomicBool::new(true),
            state: Mutex::new(EngineState::new()),
            runtime: Mutex::new(None),
            callbacks: Mutex::new(None),
        })
    }

    pub(super) fn alloc_voice_id(&self) -> u64 {
        self.next_voice_id.fetch_add(1, Ordering::Relaxed)
    }

    pub(super) fn dispatch_callbacks(&self, events: Vec<CallbackEvent>) {
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

    pub(super) fn sync_callbacks(&self) {
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

    pub(super) fn shutdown_runtime(&self) {
        let rt = self.runtime.lock().unwrap().take();
        if let Some(r) = rt {
            r.shutdown();
        }
    }

    pub(super) fn shutdown_callbacks(&self) {
        let disp = self.callbacks.lock().unwrap().take();
        drop(disp);
    }

    /// Mix `frames` interleaved f32 frames at the mastering voice's sample rate & channel count
    /// into `out` (`out.len() >= frames * master_channels`).
    ///
    /// Returns any `IXAudio2VoiceCallback` / `IXAudio2EngineCallback` events produced during this
    /// quantum so the caller can dispatch them strictly outside any engine lock.
    pub(super) fn mix_quantum(&self, frames: usize, out: &mut [f32]) -> Vec<CallbackEvent> {
        let mut events = Vec::new();
        let mut st_guard = self.state.lock().unwrap();
        let st: &mut EngineState = &mut *st_guard;
        let Some(master_info) = st.master.as_ref().map(|m| (m.id, m.channels, m.sample_rate))
        else {
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
                    let i0 =
                        (front.pos.floor() as usize).clamp(front.play_begin, front.play_end - 1);
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
                sub.filter
                    .process(&mut sub.mix_buf, sub.channels, rate_scale);
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

pub(super) fn choose_output_device(device_id: Option<&str>) -> backend::DeviceInfo {
    let headless = std::env::var("PORT_HEADLESS").is_ok_and(|v| v == "1")
        || std::env::var("AW2_HEADLESS").is_ok_and(|v| v == "1");
    if headless {
        return backend::null_device();
    }
    let mut devs = backend::output_devices();
    if let Some(req) = device_id.map(str::trim).filter(|s| !s.is_empty()) {
        let req_lower = req.to_ascii_lowercase();
        let is_swd_path = req_lower.starts_with(r"\\?\swd#mmdevapi#");
        let matched_idx = devs
            .iter()
            .position(|d| {
                let ep_id = crate::audio::mmdevapi::render_endpoint_id_for_uid(&d.uid);
                req_lower == ep_id || (is_swd_path && req_lower.contains(&ep_id))
            })
            .or_else(|| devs.iter().position(|d| d.uid == req))
            .or_else(|| devs.iter().position(|d| d.name.eq_ignore_ascii_case(req)));
        if let Some(idx) = matched_idx {
            let mut dev = devs.remove(idx);
            if idx != 0 {
                // Explicitly requested non-default endpoint: bind HALOutput to this device
                // rather than tracking system default changes.
                dev.is_default = false;
            }
            return dev;
        }
        log::debug!(
            target: "audio",
            "xaudio2: requested szDeviceId {req:?} not found; falling back to default output"
        );
    }
    devs.into_iter().next().unwrap_or_else(backend::null_device)
}

pub(super) fn spawn_mastering_runtime(
    engine: &Arc<EngineCore>,
    dev: &backend::DeviceInfo,
    sample_rate: u32,
    channels: usize,
) -> Result<MasteringRuntime, String> {
    let io_request: u32 = 512;
    let quantum_frames = (sample_rate as usize / 100).clamp(64, 2048);
    let ring_cap = (2 * backend::MAX_SLICE as usize)
        .max(sample_rate as usize / 4)
        .max(quantum_frames * 8);
    let shared = StreamShared::new(Ring::new(ring_cap, channels));

    let mut stream =
        match OutputStream::open(&dev, sample_rate, channels as u16, shared.clone(), io_request) {
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
    let target_buffered = (io_frames * 2 + quantum_frames)
        .min(ring_cap / 2)
        .max(quantum_frames * 2);

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
        "xaudio2: mastering voice opened '{}' ({}) {} Hz {} ch ({:?}, io={} frames, quantum={} frames)",
        dev.name,
        dev.uid,
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
        device_uid: dev.uid.clone(),
        mixer_thread: Some(mixer_thread),
        stream: Some(stream),
    })
}

// ============================================================================
// Send-List Parsing Helper
// ============================================================================

pub(super) unsafe fn parse_send_list(
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
    let list = rd(send_list);
    if list.send_count == 0 {
        return Ok((false, Vec::new()));
    }
    if list.sends.is_null() {
        return Err(E_POINTER);
    }
    let count = list.send_count as usize;
    let mut sends = Vec::with_capacity(count);
    for i in 0..count {
        let desc = rd(list.sends.add(i));
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
