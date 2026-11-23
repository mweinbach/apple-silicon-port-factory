//! PCM, RIFF/WAV, and FSB5 (PCM8/16/24/32, Float, IMA-ADPCM, FADPCM) decoders
//! for FMOD Core and FMOD Studio.

use super::core::{
    FMOD_ERR_FORMAT, FMOD_SOUND_FORMAT_PCM16, FMOD_SOUND_FORMAT_PCM24, FMOD_SOUND_FORMAT_PCM32,
    FMOD_SOUND_FORMAT_PCM8, FMOD_SOUND_FORMAT_PCMFLOAT,
};
use crate::audio::format::{SampleType, decode_pcm_buffer_f32};
use crate::strings::{le_u16, le_u32, le_u64};
use std::sync::Arc;

// ============================================================================
// PCM & RIFF/WAV decoding
// ============================================================================

#[inline]
pub(super) fn fmod_format_to_sample_type(format: u32) -> Option<SampleType> {
    match format {
        FMOD_SOUND_FORMAT_PCM8 => Some(SampleType::U8),
        FMOD_SOUND_FORMAT_PCM16 => Some(SampleType::I16),
        FMOD_SOUND_FORMAT_PCM24 => Some(SampleType::I24),
        FMOD_SOUND_FORMAT_PCM32 => Some(SampleType::I32),
        FMOD_SOUND_FORMAT_PCMFLOAT => Some(SampleType::F32),
        _ => None,
    }
}

pub(super) fn format_bits_and_bytes(format: u32) -> Option<(u32, usize)> {
    let st = fmod_format_to_sample_type(format)?;
    let bps = st.bytes_per_sample();
    Some(((bps * 8) as u32, bps))
}

pub(super) fn decode_pcm_to_f32(raw: &[u8], format: u32) -> Option<Vec<f32>> {
    let st = fmod_format_to_sample_type(format)?;
    let mut out = Vec::new();
    decode_pcm_buffer_f32(st, raw, &mut out);
    Some(out)
}

pub(super) struct DecodedWav {
    pub(super) format: u32,
    pub(super) channels: u32,
    pub(super) sample_rate: u32,
    pub(super) bits_per_sample: u32,
    pub(super) raw_pcm: Vec<u8>,
}

pub(super) fn identify_container(bytes: &[u8]) -> &'static str {
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

pub(super) fn parse_wav(bytes: &[u8]) -> Result<DecodedWav, i32> {
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
        let chunk_size = le_u32(bytes, pos + 4).unwrap_or(0) as usize;
        pos += 8;
        let payload_end = pos.saturating_add(chunk_size).min(bytes.len());
        let payload = &bytes[pos..payload_end];

        if chunk_id == b"fmt " {
            if payload.len() < 16 {
                return Err(FMOD_ERR_FORMAT);
            }
            let mut audio_format = le_u16(payload, 0).ok_or(FMOD_ERR_FORMAT)?;
            let channels = le_u16(payload, 2).ok_or(FMOD_ERR_FORMAT)? as u32;
            let sample_rate = le_u32(payload, 4).ok_or(FMOD_ERR_FORMAT)?;
            let bits = le_u16(payload, 14).ok_or(FMOD_ERR_FORMAT)? as u32;

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

pub(super) const FSB5_MODE_PCM8: u32 = 1;
pub(super) const FSB5_MODE_PCM16: u32 = 2;
pub(super) const FSB5_MODE_PCM24: u32 = 3;
pub(super) const FSB5_MODE_PCM32: u32 = 4;
pub(super) const FSB5_MODE_PCMFLOAT: u32 = 5;
pub(super) const FSB5_MODE_IMAADPCM: u32 = 7;
pub(super) const FSB5_MODE_FADPCM: u32 = 16;

pub(super) const FSB5_CHUNK_CHANNELS: u32 = 1;
pub(super) const FSB5_CHUNK_FREQUENCY: u32 = 2;
pub(super) const FSB5_CHUNK_LOOP: u32 = 3;

pub(super) const FSB5_SAMPLE_RATES: [u32; 16] = [
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
pub(super) struct DecodedSample {
    pub(super) name: String,
    pub(super) channels: u32,
    pub(super) sample_rate: u32,
    pub(super) num_frames: usize,
    pub(super) loop_start: usize,
    pub(super) loop_end: usize,
    pub(super) has_loop: bool,
    pub(super) samples: Arc<Vec<f32>>,
    pub(super) fmod_format: u32,
    pub(super) bits_per_sample: u32,
    pub(super) raw_pcm: Vec<u8>,
}

pub(super) struct ParsedFsb5 {
    pub(super) total_bytes: usize,
    pub(super) samples: Vec<DecodedSample>,
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
pub(super) fn decode_fsb5_ima_adpcm(data: &[u8], channels: usize, num_frames: usize) -> Vec<f32> {
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
pub(super) fn decode_fsb5_fadpcm(data: &[u8], channels: usize, num_frames: usize) -> Vec<f32> {
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

pub(super) fn parse_fsb5(bytes: &[u8]) -> Result<ParsedFsb5, i32> {
    if bytes.len() < 28 || &bytes[0..4] != b"FSB5" {
        return Err(FMOD_ERR_FORMAT);
    }
    let version = le_u32(bytes, 4).ok_or(FMOD_ERR_FORMAT)?;
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

    let num_samples = le_u32(bytes, 8).ok_or(FMOD_ERR_FORMAT)? as usize;
    let sample_headers_size = le_u32(bytes, 12).ok_or(FMOD_ERR_FORMAT)? as usize;
    let name_table_size = le_u32(bytes, 16).ok_or(FMOD_ERR_FORMAT)? as usize;
    let data_size = le_u32(bytes, 20).ok_or(FMOD_ERR_FORMAT)? as usize;
    let mode = le_u32(bytes, 24).ok_or(FMOD_ERR_FORMAT)?;

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
        let raw = le_u64(sh_slice, sh_pos).ok_or(FMOD_ERR_FORMAT)?;
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
            let chdr = le_u32(sh_slice, sh_pos).ok_or(FMOD_ERR_FORMAT)?;
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
                    sample_rate = le_u32(cpayload, 0).unwrap_or(sample_rate);
                }
                FSB5_CHUNK_LOOP if cpayload.len() >= 8 => {
                    loop_start = le_u32(cpayload, 0).unwrap_or(0) as usize;
                    loop_end = le_u32(cpayload, 4).unwrap_or(0) as usize;
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
            let off = le_u32(nt_slice, i * 4).unwrap_or(u32::MAX) as usize;
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
