//! Pure-Rust Bink Audio 1.0 / 2.0 (DCT & RDFT) decoder and CoreAudio playback sink.
//!
//! Implements the open Bink Audio specification (matching FFmpeg `libavcodec/binkaudio.c`):
//! - LSB-first bitstream reader with 29-bit Bink floating-point values (`5`-bit exponent,
//!   `23`-bit mantissa, `1`-bit sign) and Bink 1.0b (`BIKb`) 32-bit float compatibility.
//! - 25-band Bark-scale critical frequency quantization (`BINK_CRITICAL_FREQS`), exponential
//!   band scale factors (`exp(i * 0.0664 / log10(e))`), RLE zero runs (`RLE_LENGTH_TAB`),
//!   and variable-width signed transform coefficients.
//! - Fast O(N log N) radix-2 FFT backing both Bink inverse DCT-III (`BINK_AUD_USEDCT`) and
//!   Bink inverse RDFT (`!BINK_AUD_USEDCT`, including interleaved stereo RDFT).
//! - Linear crossfade overlap-add (`overlap_len = frame_len / 16`) across consecutive blocks.
//! - Multi-track mixing with Bink volume (`0..32768..65536`) and pan (`0..32768..65536`)
//!   feeding `crate::audio::backend::OutputStream` (CoreAudio AUHAL on macOS, RemoteIO on
//!   iOS, or null fallback when headless / unavailable).

use crate::audio::backend::{self, OutputStream, StreamShared};
use crate::audio::ring::Ring;
use std::f64::consts::PI;
use std::sync::Arc;

/// Bink audio track flag: prefer 16-bit signed PCM output (`0x4000` in upper 16 bits of track type).
pub const BINK_AUD_16BITS: u16 = 0x4000;
/// Bink audio track flag: stereo (2 channels) instead of mono (1 channel).
pub const BINK_AUD_STEREO: u16 = 0x2000;
/// Bink audio track flag: use DCT-III transform instead of RDFT.
pub const BINK_AUD_USEDCT: u16 = 0x1000;

/// Bink volume scale (`32768` = 1.0 unity gain, `65536` = 2.0).
pub const BINK_UNITY_VOLUME: i32 = 32768;
pub const BINK_MAX_VOLUME: i32 = 65536;
/// Bink pan scale (`0` = full left, `32768` = center, `65536` = full right).
pub const BINK_CENTER_PAN: i32 = 32768;
pub const BINK_MAX_PAN: i32 = 65536;

/// 25 Bark-scale critical frequency boundaries (Hz) used by Bink Audio 1.0 / 2.0.
pub const BINK_CRITICAL_FREQS: [u32; 25] = [
    100, 200, 300, 400, 510, 630, 770, 920, 1080, 1270, 1480, 1720, 2000, 2320, 2700, 3150, 3700,
    4400, 5300, 6400, 7700, 9500, 12000, 15500, 24500,
];

/// Run-length multiplier table (in groups of 8 coefficients). Note: `7` is intentionally absent.
pub const RLE_LENGTH_TAB: [u8; 16] = [2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64];

/// `0.066399999 / log10(e)` for the 96-entry exponential band quantization scale factor table.
const QUANT_EXP_STEP: f64 = 0.152_891_647_872_219_54;

/// Maximum uncompressed PCM byte count accepted from a single container packet header
/// (guards against corrupt `reported_size` values causing excessive work or memory usage).
const MAX_PACKET_REPORTED_BYTES: usize = 16 * 1024 * 1024;

/// Parsed per-track metadata from the Bink 1 / Bink 2 file header table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AudioTrackHeader {
    /// Maximum uncompressed audio bytes for one video frame (`tracksizes[i]`).
    pub max_size: u32,
    /// Raw 32-bit track type dword (`sample_rate | ((flags as u32) << 16)`).
    pub raw_type: u32,
    /// Sample rate in Hz (lower 16 bits of `raw_type`).
    pub sample_rate: u32,
    /// Track flags (`BINK_AUD_*`, upper 16 bits of `raw_type`).
    pub flags: u16,
    /// Track identifier (`trackIDs[i]`).
    pub track_id: u32,
}

impl AudioTrackHeader {
    #[inline]
    pub fn from_tables(max_size: u32, raw_type: u32, track_id: u32) -> Self {
        Self {
            max_size,
            raw_type,
            sample_rate: raw_type & 0xFFFF,
            flags: ((raw_type >> 16) & 0xFFFF) as u16,
            track_id,
        }
    }

    #[inline]
    pub fn is_16bit(&self) -> bool {
        (self.flags & BINK_AUD_16BITS) != 0
    }

    #[inline]
    pub fn is_stereo(&self) -> bool {
        (self.flags & BINK_AUD_STEREO) != 0
    }

    #[inline]
    pub fn use_dct(&self) -> bool {
        (self.flags & BINK_AUD_USEDCT) != 0
    }

    #[inline]
    pub fn channels(&self) -> u16 {
        if self.is_stereo() { 2 } else { 1 }
    }
}

/// Parse `num_tracks` audio track headers from the `3 * num_tracks` leading u32s of
/// a Bink 1 / Bink 2 header table (`max_sizes`, `sample_rate | flags<<16`, `track_ids`).
pub fn parse_audio_track_headers(num_tracks: usize, tables: &[u32]) -> Vec<AudioTrackHeader> {
    if num_tracks == 0 || tables.len() < num_tracks.saturating_mul(3) {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(num_tracks);
    for i in 0..num_tracks {
        out.push(AudioTrackHeader::from_tables(
            tables[i],
            tables[num_tracks + i],
            tables[num_tracks * 2 + i],
        ));
    }
    out
}

/// LSB-first bitstream reader with strict bounds checking (never panics on truncated input).
pub struct BitReader<'a> {
    data: &'a [u8],
    bit_pos: usize,
    total_bits: usize,
}

impl<'a> BitReader<'a> {
    #[inline]
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            bit_pos: 0,
            total_bits: data.len().saturating_mul(8),
        }
    }

    #[inline]
    pub fn bits_left(&self) -> usize {
        self.total_bits.saturating_sub(self.bit_pos)
    }

    /// Read `n` bits (`0..=32`) LSB-first. Returns `None` if fewer than `n` bits remain.
    #[inline]
    pub fn read_bits(&mut self, n: u32) -> Option<u32> {
        if n == 0 {
            return Some(0);
        }
        debug_assert!(n <= 32);
        let n_usize = n as usize;
        if self.bit_pos.saturating_add(n_usize) > self.total_bits {
            self.bit_pos = self.total_bits;
            return None;
        }
        let byte_idx = self.bit_pos >> 3;
        let bit_off = self.bit_pos & 7;
        let avail = (self.data.len() - byte_idx).min(8);
        let mut word = 0u64;
        for i in 0..avail {
            word |= (self.data[byte_idx + i] as u64) << (i * 8);
        }
        self.bit_pos += n_usize;
        let mask = if n == 32 { u32::MAX } else { (1u32 << n) - 1 };
        Some(((word >> bit_off) as u32) & mask)
    }

    /// Skip `n` bits. Returns `false` if fewer than `n` bits remain.
    #[inline]
    pub fn skip_bits(&mut self, n: usize) -> bool {
        if self.bit_pos.saturating_add(n) > self.total_bits {
            self.bit_pos = self.total_bits;
            return false;
        }
        self.bit_pos += n;
        true
    }

    /// Advance the bit position to the next 32-bit boundary.
    #[inline]
    pub fn align32(&mut self) {
        let rem = self.bit_pos & 31;
        if rem != 0 {
            self.bit_pos = self.bit_pos.saturating_add(32 - rem).min(self.total_bits);
        }
    }

    /// Read Bink Audio's 29-bit float: 5-bit exponent (`power - 23`), 23-bit mantissa, 1-bit sign.
    #[inline]
    pub fn read_bink_float(&mut self) -> Option<f32> {
        let power = self.read_bits(5)? as i32;
        let mantissa = self.read_bits(23)?;
        let sign = self.read_bits(1)?;
        let mut val = (mantissa as f32) * 2.0f32.powi(power - 23);
        if sign != 0 {
            val = -val;
        }
        Some(val)
    }
}

/// Stateful per-track Bink Audio 1.0 / 2.0 decoder (DCT-III or RDFT, mono or stereo).
pub struct BinkAudioDecoder {
    channels: u16,
    decode_channels: usize,
    use_dct: bool,
    version_b: bool,
    frame_len: usize,
    overlap_len: usize,
    num_bands: usize,
    bands: [usize; 26],
    root: f32,
    quant_table: [f32; 96],
    first: bool,
    // Precomputed FFT / DCT tables (zero allocations per block).
    bit_rev: Vec<usize>,
    fft_twiddles: Vec<(f32, f32)>,
    dct_pre: Vec<(f32, f32)>,
    fft_buf: Vec<(f32, f32)>,
    // Per-decode-channel working buffers and overlap history.
    block_buf: [Vec<f32>; 2],
    previous: [Vec<f32>; 2],
}

impl BinkAudioDecoder {
    /// Create a new Bink Audio decoder for a single audio track.
    pub fn new(sample_rate: u32, channels: u16, use_dct: bool, version_b: bool) -> Self {
        let base_rate = sample_rate.clamp(300, 192_000);
        let ch = if channels <= 1 { 1u16 } else { 2u16 };

        let mut frame_len_bits: u32 = if base_rate < 22050 {
            9
        } else if base_rate < 44100 {
            10
        } else {
            11
        };

        // In RDFT mode, stereo audio is interleaved inside one double-length transform.
        let (eff_rate, decode_channels) = if use_dct {
            (base_rate, ch as usize)
        } else {
            if ch == 2 {
                frame_len_bits += 1;
            }
            (base_rate.saturating_mul(ch as u32), 1usize)
        };

        let frame_len = 1usize << frame_len_bits;
        let overlap_len = frame_len >> 4;
        let sample_rate_half = (eff_rate.saturating_add(1)) >> 1;

        // Calculate number of critical frequency bands (1..=25).
        let mut num_bands = 1usize;
        while num_bands < 25 {
            if sample_rate_half <= BINK_CRITICAL_FREQS[num_bands - 1] {
                break;
            }
            num_bands += 1;
        }

        let mut bands = [0usize; 26];
        bands[0] = 2;
        let half_len = (frame_len >> 1) as u64;
        let sr_half = (sample_rate_half.max(1)) as u64;
        for i in 1..num_bands {
            let edge = ((BINK_CRITICAL_FREQS[i - 1] as u64) * half_len / sr_half) as usize * 2;
            bands[i] = edge.clamp(2, frame_len);
        }
        bands[num_bands] = frame_len;

        // Normalization factor into [-1.0, 1.0] float audio.
        let root = (2.0f64 / ((frame_len as f64).sqrt() * 32768.0)) as f32;
        let mut quant_table = [0.0f32; 96];
        for (i, slot) in quant_table.iter_mut().enumerate() {
            *slot = (((i as f64) * QUANT_EXP_STEP).exp() as f32) * root;
        }

        // Precompute bit-reversal indices and radix-2 FFT twiddles for length `frame_len`.
        let shift = usize::BITS - frame_len_bits;
        let bit_rev: Vec<usize> = (0..frame_len).map(|i| i.reverse_bits() >> shift).collect();
        let fft_twiddles: Vec<(f32, f32)> = (0..(frame_len >> 1))
            .map(|m| {
                let angle = -2.0 * PI * (m as f64) / (frame_len as f64);
                (angle.cos() as f32, angle.sin() as f32)
            })
            .collect();
        let dct_pre: Vec<(f32, f32)> = if use_dct {
            (0..frame_len)
                .map(|k| {
                    let angle = -PI * (k as f64) / (2.0 * frame_len as f64);
                    (angle.cos() as f32, angle.sin() as f32)
                })
                .collect()
        } else {
            Vec::new()
        };

        Self {
            channels: ch,
            decode_channels,
            use_dct,
            version_b,
            frame_len,
            overlap_len,
            num_bands,
            bands,
            root,
            quant_table,
            first: true,
            bit_rev,
            fft_twiddles,
            dct_pre,
            fft_buf: vec![(0.0, 0.0); frame_len],
            block_buf: [vec![0.0; frame_len], vec![0.0; frame_len]],
            previous: [vec![0.0; overlap_len], vec![0.0; overlap_len]],
        }
    }

    /// Number of output frames (per channel) emitted by one decoded Bink audio block.
    #[inline]
    pub fn output_frames_per_block(&self) -> usize {
        let per_decode_ch = self.frame_len - self.overlap_len;
        if self.use_dct {
            per_decode_ch
        } else {
            per_decode_ch / (self.channels as usize)
        }
    }

    /// Reset overlap-add state (called on `BinkGoto` / seek).
    pub fn reset(&mut self) {
        self.first = true;
        for prev in &mut self.previous {
            prev.fill(0.0);
        }
    }

    /// In-place radix-2 Cooley-Tukey forward FFT: `Y[n] = sum_{k=0..N-1} X[k] * exp(-i*2*pi*k*n/N)`.
    fn fft_forward_in_place(&mut self) {
        let n = self.frame_len;
        for i in 0..n {
            let j = self.bit_rev[i];
            if i < j {
                self.fft_buf.swap(i, j);
            }
        }
        let mut len = 2usize;
        while len <= n {
            let half = len >> 1;
            let stride = n / len;
            let mut i = 0usize;
            while i < n {
                for j in 0..half {
                    let (wr, wi) = self.fft_twiddles[j * stride];
                    let (ur, ui) = self.fft_buf[i + j];
                    let (vr, vi) = self.fft_buf[i + j + half];
                    let tr = vr * wr - vi * wi;
                    let ti = vr * wi + vi * wr;
                    self.fft_buf[i + j] = (ur + tr, ui + ti);
                    self.fft_buf[i + j + half] = (ur - tr, ui - ti);
                }
                i += len;
            }
            len <<= 1;
        }
    }

    /// Bink inverse DCT-III via Makhoul's N-point FFT permutation:
    /// `y[n] = C[0] + sum_{k=1..N-1} C[k] * cos(pi * k * (2*n + 1) / (2*N))`.
    fn transform_idct3(&mut self, ch: usize) {
        let n = self.frame_len;
        let coeffs = &self.block_buf[ch];
        for k in 0..n {
            let c = coeffs[k];
            let (pr, pi) = self.dct_pre[k];
            self.fft_buf[k] = (c * pr, c * pi);
        }
        self.fft_forward_in_place();
        let half = n >> 1;
        let out = &mut self.block_buf[ch];
        for r in 0..half {
            out[2 * r] = self.fft_buf[r].0;
        }
        for r in half..n {
            let idx = 2 * (n - 1 - r) + 1;
            out[idx] = self.fft_buf[r].0;
        }
    }

    /// Bink inverse RDFT via Hermitian-symmetric N-point FFT:
    /// `y[n] = 0.5 * C[0] + 0.5 * (-1)^n * C[1] + sum_{k=1..N/2-1} (C[2k]*cos(2pi*k*n/N) + C[2k+1]*sin(2pi*k*n/N))`.
    fn transform_irdft(&mut self, ch: usize) {
        let n = self.frame_len;
        let half = n >> 1;
        let coeffs = &self.block_buf[ch];
        self.fft_buf[0] = (0.5 * coeffs[0], 0.0);
        self.fft_buf[half] = (0.5 * coeffs[1], 0.0);
        for k in 1..half {
            let re = 0.5 * coeffs[2 * k];
            let im = 0.5 * coeffs[2 * k + 1];
            self.fft_buf[k] = (re, im);
            self.fft_buf[n - k] = (re, -im);
        }
        self.fft_forward_in_place();
        let out = &mut self.block_buf[ch];
        for i in 0..n {
            out[i] = self.fft_buf[i].0;
        }
    }

    /// Decode one Bink Audio block from `br` and append interleaved normalized `f32` samples
    /// (`[-1.0, 1.0]`) to `out_interleaved`. Returns `Some(samples_written)` on success or
    /// `None` if the bitstream was truncated (without corrupting overlap state).
    pub fn decode_block(
        &mut self,
        br: &mut BitReader<'_>,
        out_interleaved: &mut Vec<f32>,
    ) -> Option<usize> {
        if self.use_dct && !br.skip_bits(2) {
            return None;
        }

        let mut quant = [0.0f32; 25];
        for ch in 0..self.decode_channels {
            let c0 = if self.version_b {
                f32::from_bits(br.read_bits(32)?) * self.root
            } else {
                br.read_bink_float()? * self.root
            };
            let c1 = if self.version_b {
                f32::from_bits(br.read_bits(32)?) * self.root
            } else {
                br.read_bink_float()? * self.root
            };

            for band_q in quant.iter_mut().take(self.num_bands) {
                if self.version_b {
                    *band_q = br.read_bink_float()? * self.root;
                } else {
                    let idx = br.read_bits(8)? as usize;
                    *band_q = self.quant_table[idx.min(95)];
                }
            }

            {
                let coeffs = &mut self.block_buf[ch];
                coeffs[0] = if c0.is_finite() { c0 } else { 0.0 };
                coeffs[1] = if c1.is_finite() { c1 } else { 0.0 };

                let mut k = 0usize;
                let mut q = quant[0];
                let mut pos = 2usize;
                while pos < self.frame_len {
                    let run_len = if br.read_bits(1)? != 0 {
                        let rle_idx = br.read_bits(4)? as usize;
                        (RLE_LENGTH_TAB[rle_idx] as usize) * 8
                    } else {
                        8
                    };
                    let end = pos.saturating_add(run_len).min(self.frame_len);
                    let width = br.read_bits(4)?;
                    if width == 0 {
                        coeffs[pos..end].fill(0.0);
                        pos = end;
                    } else {
                        while pos < end {
                            while k < self.num_bands && pos >= self.bands[k] {
                                q = quant[k];
                                k += 1;
                            }
                            let v = br.read_bits(width)?;
                            if v != 0 {
                                let sign = br.read_bits(1)?;
                                let val = (v as f32) * q;
                                let signed = if sign != 0 { -val } else { val };
                                coeffs[pos] = if signed.is_finite() { signed } else { 0.0 };
                            } else {
                                coeffs[pos] = 0.0;
                            }
                            pos += 1;
                        }
                    }
                }
            }

            if self.use_dct {
                self.transform_idct3(ch);
            } else {
                self.transform_irdft(ch);
            }
        }

        // Linear crossfade overlap-add with previous block's tail (skipped on first block).
        let out_per_decode_ch = self.frame_len - self.overlap_len;
        if !self.first {
            let olen = self.overlap_len;
            let inv_olen = 1.0f32 / (olen as f32);
            for ch in 0..self.decode_channels {
                let prev = &self.previous[ch];
                let cur = &mut self.block_buf[ch];
                for i in 0..olen {
                    let w_prev = ((olen - i) as f32) * inv_olen;
                    let w_cur = (i as f32) * inv_olen;
                    cur[i] = prev[i] * w_prev + cur[i] * w_cur;
                }
            }
        }
        self.first = false;

        for ch in 0..self.decode_channels {
            self.previous[ch][..self.overlap_len]
                .copy_from_slice(&self.block_buf[ch][out_per_decode_ch..self.frame_len]);
        }

        // Emit interleaved samples into `out_interleaved`.
        let before = out_interleaved.len();
        if self.decode_channels == 1 {
            // Mono (DCT/RDFT) or already-interleaved stereo (RDFT).
            out_interleaved.extend_from_slice(&self.block_buf[0][..out_per_decode_ch]);
        } else {
            // Stereo DCT: interleave channel 0 (L) and channel 1 (R).
            out_interleaved.reserve(out_per_decode_ch * 2);
            for i in 0..out_per_decode_ch {
                out_interleaved.push(self.block_buf[0][i]);
                out_interleaved.push(self.block_buf[1][i]);
            }
        }
        Some(out_interleaved.len() - before)
    }

    /// Decode a full Bink container audio packet (`[u32_le reported_bytes][blocks...]`) into
    /// interleaved normalized `f32` samples (clears `out_interleaved` first).
    /// Returns the number of interleaved `f32` samples produced. Never panics on malformed input.
    pub fn decode_packet(&mut self, packet: &[u8], out_interleaved: &mut Vec<f32>) -> usize {
        out_interleaved.clear();
        if packet.len() < 4 {
            return 0;
        }
        let reported_bytes =
            u32::from_le_bytes([packet[0], packet[1], packet[2], packet[3]]) as usize;
        if reported_bytes == 0 {
            return 0;
        }
        let capped_bytes = reported_bytes.min(MAX_PACKET_REPORTED_BYTES);
        let ch = self.channels as usize;
        let reported_samples = (capped_bytes / 2 / ch) * ch;
        if reported_samples == 0 {
            return 0;
        }

        let payload = &packet[4..];
        let mut br = BitReader::new(payload);
        // Reserve conservatively based on actual compressed payload size so corrupt headers never OOM.
        let approx_cap = reported_samples.min(payload.len().saturating_mul(64).max(4096));
        out_interleaved.reserve(approx_cap);

        while br.bits_left() >= 32 && out_interleaved.len() < reported_samples {
            if self.decode_block(&mut br, out_interleaved).is_none() {
                break;
            }
            br.align32();
        }

        if out_interleaved.len() > reported_samples {
            out_interleaved.truncate(reported_samples);
        }
        out_interleaved.len()
    }

    /// Decode a Bink container audio packet directly into interleaved signed 16-bit PCM samples.
    pub fn decode_packet_i16(&mut self, packet: &[u8], out_pcm: &mut Vec<i16>) -> usize {
        let mut tmp = Vec::new();
        let n = self.decode_packet(packet, &mut tmp);
        if n == 0 {
            out_pcm.clear();
            return 0;
        }
        f32_to_i16_pcm(&tmp, out_pcm);
        out_pcm.len()
    }
}

/// Convert Bink volume (`0..=65536`, `32768` = unity 1.0) to linear gain.
#[inline]
pub fn volume_to_gain(volume: i32) -> f32 {
    (volume.clamp(0, BINK_MAX_VOLUME) as f32) / (BINK_UNITY_VOLUME as f32)
}

/// Convert Bink pan (`0..=65536`, `0` = full left, `32768` = center, `65536` = full right)
/// to `(left_gain, right_gain)` using Bink's balance curve.
#[inline]
pub fn pan_to_gains(pan: i32) -> (f32, f32) {
    let p = pan.clamp(0, BINK_MAX_PAN);
    if p <= BINK_CENTER_PAN {
        (1.0, (p as f32) / (BINK_CENTER_PAN as f32))
    } else {
        (
            ((BINK_MAX_PAN - p) as f32) / (BINK_CENTER_PAN as f32),
            1.0,
        )
    }
}

/// Convert normalized `f32` audio (`[-1.0, 1.0]`) to signed 16-bit PCM.
pub fn f32_to_i16_pcm(src: &[f32], dst: &mut Vec<i16>) {
    dst.clear();
    dst.reserve(src.len());
    for &s in src {
        let v = if s.is_finite() {
            (s * 32768.0).round().clamp(-32768.0, 32767.0) as i16
        } else {
            0
        };
        dst.push(v);
    }
}

/// Mix mono or stereo decoded samples with Bink `volume` and `pan` into a stereo accumulator buffer,
/// linearly resampling if `src_rate != dst_rate`.
pub fn mix_track_into_stereo(
    src: &[f32],
    src_channels: u16,
    src_rate: u32,
    dst_rate: u32,
    volume: i32,
    pan: i32,
    dst_stereo: &mut Vec<f32>,
) {
    let ch = (src_channels.max(1)) as usize;
    let src_frames = src.len() / ch;
    if src_frames == 0 {
        return;
    }
    let gain = volume_to_gain(volume);
    let (pl, pr) = pan_to_gains(pan);
    let gl = gain * pl;
    let gr = gain * pr;

    let sample_lr = |frame_idx: usize| -> (f32, f32) {
        if ch == 1 {
            let s = src[frame_idx];
            let s = if s.is_finite() { s } else { 0.0 };
            (s, s)
        } else {
            let l = src[frame_idx * 2];
            let r = src[frame_idx * 2 + 1];
            (
                if l.is_finite() { l } else { 0.0 },
                if r.is_finite() { r } else { 0.0 },
            )
        }
    };

    if src_rate == dst_rate || src_rate == 0 || dst_rate == 0 {
        let need = src_frames * 2;
        if dst_stereo.len() < need {
            dst_stereo.resize(need, 0.0);
        }
        for f in 0..src_frames {
            let (l, r) = sample_lr(f);
            dst_stereo[f * 2] += l * gl;
            dst_stereo[f * 2 + 1] += r * gr;
        }
    } else {
        let dst_frames = (((src_frames as u64) * (dst_rate as u64)) / (src_rate as u64)).max(1) as usize;
        let need = dst_frames * 2;
        if dst_stereo.len() < need {
            dst_stereo.resize(need, 0.0);
        }
        let step = (src_rate as f64) / (dst_rate as f64);
        let max_idx = src_frames - 1;
        for f in 0..dst_frames {
            let pos = (f as f64) * step;
            let idx0 = (pos as usize).min(max_idx);
            let idx1 = (idx0 + 1).min(max_idx);
            let frac = (pos - (idx0 as f64)).clamp(0.0, 1.0) as f32;
            let (l0, r0) = sample_lr(idx0);
            let (l1, r1) = sample_lr(idx1);
            let l = l0 + (l1 - l0) * frac;
            let r = r0 + (r1 - r0) * frac;
            dst_stereo[f * 2] += l * gl;
            dst_stereo[f * 2 + 1] += r * gr;
        }
    }
}

/// One active playback audio track owned by a [`BinkAudioPlayer`].
pub struct ActiveAudioTrack {
    pub file_track_index: usize,
    pub track_id: u32,
    pub sample_rate: u32,
    pub channels: u16,
    pub volume: i32,
    pub pan: i32,
    pub decoder: BinkAudioDecoder,
}

/// Per-`BinkInstance` audio decoder + CoreAudio playback stream.
pub struct BinkAudioPlayer {
    tracks: Vec<ActiveAudioTrack>,
    output_sample_rate: u32,
    sound_on: bool,
    paused: bool,
    shared: Option<Arc<StreamShared>>,
    stream: Option<OutputStream>,
    tap: Option<crate::audio::tap::Tap>,
    decode_scratch: Vec<f32>,
    mix_scratch: Vec<f32>,
}

impl BinkAudioPlayer {
    pub fn new(tracks: Vec<ActiveAudioTrack>, sound_on: bool) -> Self {
        let output_sample_rate = tracks
            .first()
            .map(|t| t.sample_rate)
            .filter(|&r| r >= 300)
            .unwrap_or(48_000);
        Self {
            tracks,
            output_sample_rate,
            sound_on,
            paused: false,
            shared: None,
            stream: None,
            tap: None,
            decode_scratch: Vec::new(),
            mix_scratch: Vec::new(),
        }
    }

    #[inline]
    pub fn is_sound_on(&self) -> bool {
        self.sound_on
    }

    pub fn set_volume(&mut self, track_id: u32, volume: i32) {
        let v = volume.clamp(0, BINK_MAX_VOLUME);
        if track_id == u32::MAX {
            for t in &mut self.tracks {
                t.volume = v;
            }
            return;
        }
        let mut matched = false;
        for t in &mut self.tracks {
            if t.track_id == track_id {
                t.volume = v;
                matched = true;
            }
        }
        if !matched && track_id == 0 {
            if let Some(first) = self.tracks.first_mut() {
                first.volume = v;
            }
        }
    }

    pub fn set_pan(&mut self, track_id: u32, pan: i32) {
        let p = pan.clamp(0, BINK_MAX_PAN);
        if track_id == u32::MAX {
            for t in &mut self.tracks {
                t.pan = p;
            }
            return;
        }
        let mut matched = false;
        for t in &mut self.tracks {
            if t.track_id == track_id {
                t.pan = p;
                matched = true;
            }
        }
        if !matched && track_id == 0 {
            if let Some(first) = self.tracks.first_mut() {
                first.pan = p;
            }
        }
    }

    pub fn set_sound_on(&mut self, on: bool) -> bool {
        self.sound_on = on;
        if !on {
            if let Some(ref mut s) = self.stream {
                s.stop();
            }
            if let Some(ref sh) = self.shared {
                sh.ring.clear();
            }
        } else if !self.paused {
            if let Some(ref mut s) = self.stream {
                let _ = s.start();
            }
        }
        self.sound_on
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
        if let Some(ref mut s) = self.stream {
            if paused {
                s.stop();
            } else if self.sound_on {
                let _ = s.start();
            }
        }
    }

    /// Stop the output stream, flush all queued samples, and reset all track decoders' overlap state.
    pub fn flush_and_reset(&mut self) {
        if let Some(ref mut s) = self.stream {
            s.stop();
        }
        if let Some(ref sh) = self.shared {
            sh.ring.clear();
        }
        for t in &mut self.tracks {
            t.decoder.reset();
        }
    }

    fn ensure_stream(&mut self) {
        if self.shared.is_some() || self.tracks.is_empty() {
            return;
        }
        let rate = self.output_sample_rate.clamp(8000, 192_000);
        let channels = 2u16;
        // 4-second ring absorbs Bink frame-1 audio pre-buffering (~0.75-1.5 s) without drops.
        let ring_cap = (rate as usize).saturating_mul(4).max(16_384);
        let shared = StreamShared::new(Ring::new(ring_cap, channels as usize));

        if self.tap.is_none() {
            let fmt = crate::audio::format::WaveFormat::float_mix(
                rate,
                channels,
                crate::audio::format::default_mask(channels),
            );
            self.tap = crate::audio::tap::next_path().and_then(|p| {
                match crate::audio::tap::Tap::create_tagged(p.clone(), fmt, "bink2") {
                    Ok(t) => Some(t),
                    Err(e) => {
                        log::error!(target: "audio", "bink2w64: audio tap {}: {e}", p.display());
                        None
                    }
                }
            });
        }

        let dev = if backend::use_null_in_headless_mode() {
            backend::null_device()
        } else {
            backend::output_devices()
                .into_iter()
                .next()
                .unwrap_or_else(backend::null_device)
        };

        let io_req = 512u32;
        let stream = match OutputStream::open(&dev, rate, channels, shared.clone(), io_req) {
            Ok(s) => {
                log::info!(
                    target: "audio",
                    "bink2w64: audio opened '{}' ({}) {} Hz {} ch ({:?}, io={} frames)",
                    dev.name,
                    dev.uid,
                    rate,
                    channels,
                    s.kind,
                    s.io_frames
                );
                Some(s)
            }
            Err(e) => {
                log::warn!(
                    target: "audio",
                    "bink2w64: audio open failed on '{}': {e}; falling back to null output",
                    dev.name
                );
                OutputStream::open(&backend::null_device(), rate, channels, shared.clone(), io_req)
                    .ok()
            }
        };

        self.shared = Some(shared);
        self.stream = stream;
    }

    /// Decode and submit one video frame's audio packets to the CoreAudio playback ring.
    pub fn submit_frame_packets(&mut self, frame_buf: &[u8], track_spans: &[(usize, usize)]) {
        if self.tracks.is_empty() {
            return;
        }
        self.mix_scratch.clear();
        let out_rate = self.output_sample_rate;
        let mut any_samples = false;

        for track in &mut self.tracks {
            let Some(&(start, end)) = track_spans.get(track.file_track_index) else {
                continue;
            };
            if end <= start || end > frame_buf.len() {
                continue;
            }
            let pkt = &frame_buf[start..end];
            let n = track.decoder.decode_packet(pkt, &mut self.decode_scratch);
            if n == 0 || !self.sound_on {
                continue;
            }
            mix_track_into_stereo(
                &self.decode_scratch,
                track.channels,
                track.sample_rate,
                out_rate,
                track.volume,
                track.pan,
                &mut self.mix_scratch,
            );
            any_samples = true;
        }

        if !self.sound_on || !any_samples || self.mix_scratch.is_empty() {
            return;
        }

        for s in &mut self.mix_scratch {
            *s = if s.is_finite() { s.clamp(-1.0, 1.0) } else { 0.0 };
        }

        self.ensure_stream();
        let padding_before = self.shared.as_ref().map_or(0, |sh| sh.ring.len() as u32);
        if let Some(ref sh) = self.shared {
            sh.ring.push(&self.mix_scratch);
        }
        if let Some(ref mut t) = self.tap {
            let frames = (self.mix_scratch.len() / 2) as u32;
            t.submit_f32(frames, padding_before, &self.mix_scratch);
        }
        if !self.paused {
            if let Some(ref mut s) = self.stream {
                if !self
                    .shared
                    .as_ref()
                    .is_some_and(|sh| sh.started.load(std::sync::atomic::Ordering::Acquire))
                {
                    let _ = s.start();
                }
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn ring_queued_frames(&self) -> usize {
        self.shared.as_ref().map_or(0, |s| s.ring.len())
    }

    #[cfg(test)]
    pub(crate) fn track_count(&self) -> usize {
        self.tracks.len()
    }

    #[cfg(test)]
    pub(crate) fn track_volume_pan(&self, idx: usize) -> Option<(i32, i32)> {
        self.tracks.get(idx).map(|t| (t.volume, t.pan))
    }
}

#[cfg(test)]
pub(crate) mod test_util {
    use super::*;

    /// LSB-first bitstream writer for constructing valid Bink Audio test blocks & packets.
    pub struct BitWriter {
        bytes: Vec<u8>,
        bit_pos: usize,
    }

    impl BitWriter {
        pub fn new() -> Self {
            Self {
                bytes: Vec::new(),
                bit_pos: 0,
            }
        }

        pub fn write_bits(&mut self, val: u32, n: u32) {
            for b in 0..n {
                let byte_idx = self.bit_pos >> 3;
                let bit_off = self.bit_pos & 7;
                if byte_idx >= self.bytes.len() {
                    self.bytes.push(0);
                }
                if ((val >> b) & 1) != 0 {
                    self.bytes[byte_idx] |= 1u8 << bit_off;
                }
                self.bit_pos += 1;
            }
        }

        /// Write a 29-bit Bink float representing `sign * mantissa * 2^(power - 23)`.
        pub fn write_bink_float(&mut self, power: u32, mantissa: u32, negative: bool) {
            self.write_bits(power & 0x1F, 5);
            self.write_bits(mantissa & 0x7F_FFFF, 23);
            self.write_bits(u32::from(negative), 1);
        }

        pub fn align32(&mut self) {
            let rem = self.bit_pos & 31;
            if rem != 0 {
                self.write_bits(0, (32 - rem) as u32);
            }
        }

        pub fn into_bytes(self) -> Vec<u8> {
            self.bytes
        }
    }

    /// Encode one raw Bink Audio block with explicit `c0`/`c1` 29-bit floats, uniform band
    /// quant index `quant_idx`, and optional single non-zero quantized coefficient `(bin_pos, q_val, neg)`.
    pub fn encode_simple_block(
        bw: &mut BitWriter,
        sample_rate: u32,
        channels: u16,
        use_dct: bool,
        c0: (u32, u32, bool),
        c1: (u32, u32, bool),
        quant_idx: u8,
        bin_coeff: Option<(usize, u32, bool)>,
    ) {
        let dec = BinkAudioDecoder::new(sample_rate, channels, use_dct, false);
        if use_dct {
            bw.write_bits(0, 2);
        }
        for _ch in 0..dec.decode_channels {
            bw.write_bink_float(c0.0, c0.1, c0.2);
            bw.write_bink_float(c1.0, c1.1, c1.2);
            for _ in 0..dec.num_bands {
                bw.write_bits(quant_idx as u32, 8);
            }
            let mut pos = 2usize;
            while pos < dec.frame_len {
                let end = (pos + 8).min(dec.frame_len);
                let hit = bin_coeff.filter(|&(idx, _, _)| idx >= pos && idx < end);
                // RLE flag = 0 -> chunk of 8 coefficients
                bw.write_bits(0, 1);
                if let Some((target, qval, neg)) = hit {
                    let width = (32 - qval.leading_zeros()).clamp(1, 15);
                    bw.write_bits(width, 4);
                    for p in pos..end {
                        if p == target {
                            bw.write_bits(qval, width);
                            bw.write_bits(u32::from(neg), 1);
                        } else {
                            bw.write_bits(0, width);
                        }
                    }
                } else {
                    // width = 0 -> zero chunk
                    bw.write_bits(0, 4);
                }
                pos = end;
            }
        }
        bw.align32();
    }

    /// Wrap one or more encoded blocks with the 4-byte little-endian `reported_bytes` header.
    pub fn wrap_audio_packet(reported_bytes: u32, payload: &[u8]) -> Vec<u8> {
        let mut pkt = Vec::with_capacity(4 + payload.len());
        pkt.extend_from_slice(&reported_bytes.to_le_bytes());
        pkt.extend_from_slice(payload);
        pkt
    }
}

#[cfg(test)]
mod tests {
    use super::test_util::*;
    use super::*;

    fn naive_bink_idct3(coeffs: &[f32]) -> Vec<f32> {
        let n = coeffs.len();
        let mut out = vec![0.0f32; n];
        for (i, slot) in out.iter_mut().enumerate() {
            let mut sum = coeffs[0] as f64;
            for (k, &c) in coeffs.iter().enumerate().skip(1) {
                let angle = PI * (k as f64) * ((2 * i + 1) as f64) / ((2 * n) as f64);
                sum += (c as f64) * angle.cos();
            }
            *slot = sum as f32;
        }
        out
    }

    fn naive_bink_irdft(coeffs: &[f32]) -> Vec<f32> {
        let n = coeffs.len();
        let half = n / 2;
        let mut out = vec![0.0f32; n];
        for (i, slot) in out.iter_mut().enumerate() {
            let sign_nyq = if (i & 1) == 0 { 1.0f64 } else { -1.0f64 };
            let mut sum = 0.5 * (coeffs[0] as f64) + 0.5 * sign_nyq * (coeffs[1] as f64);
            for k in 1..half {
                let angle = 2.0 * PI * (k as f64) * (i as f64) / (n as f64);
                sum += (coeffs[2 * k] as f64) * angle.cos()
                    + (coeffs[2 * k + 1] as f64) * angle.sin();
            }
            *slot = sum as f32;
        }
        out
    }

    #[test]
    fn audio_track_header_parsing_bink1_and_bink2() {
        // 2 tracks:
        // track 0: max_size=16384, 44100 Hz, 16-bit stereo DCT, id=100
        // track 1: max_size=8192,  22050 Hz, 16-bit mono RDFT,  id=200
        let t0_type = 44100u32 | (((BINK_AUD_16BITS | BINK_AUD_STEREO | BINK_AUD_USEDCT) as u32) << 16);
        let t1_type = 22050u32 | ((BINK_AUD_16BITS as u32) << 16);
        let tables = [16384u32, 8192u32, t0_type, t1_type, 100u32, 200u32];
        let parsed = parse_audio_track_headers(2, &tables);
        assert_eq!(parsed.len(), 2);

        assert_eq!(parsed[0].max_size, 16384);
        assert_eq!(parsed[0].sample_rate, 44100);
        assert_eq!(parsed[0].track_id, 100);
        assert!(parsed[0].is_16bit());
        assert!(parsed[0].is_stereo());
        assert!(parsed[0].use_dct());
        assert_eq!(parsed[0].channels(), 2);

        assert_eq!(parsed[1].max_size, 8192);
        assert_eq!(parsed[1].sample_rate, 22050);
        assert_eq!(parsed[1].track_id, 200);
        assert!(parsed[1].is_16bit());
        assert!(!parsed[1].is_stereo());
        assert!(!parsed[1].use_dct());
        assert_eq!(parsed[1].channels(), 1);
    }

    #[test]
    fn fast_idct3_matches_exact_reference() {
        for sample_rate in [16000u32, 22050, 44100] {
            let mut dec = BinkAudioDecoder::new(sample_rate, 1, true, false);
            let n = dec.frame_len;
            let mut input = vec![0.0f32; n];
            // Populate deterministic non-trivial spectrum across DC, low, mid, and high bins.
            for (k, slot) in input.iter_mut().enumerate() {
                if k < 16 || k == n / 3 || k == n - 1 {
                    *slot = (((k as f32) * 0.73).sin()) * 0.25;
                }
            }
            let expected = naive_bink_idct3(&input);
            dec.block_buf[0].copy_from_slice(&input);
            dec.transform_idct3(0);
            let max_err = dec.block_buf[0]
                .iter()
                .zip(expected.iter())
                .map(|(a, b)| (a - b).abs())
                .fold(0.0f32, f32::max);
            assert!(
                max_err < 1e-4,
                "DCT-III mismatch for N={n} (sr={sample_rate}): max_err={max_err}"
            );
        }
    }

    #[test]
    fn fast_irdft_matches_exact_reference() {
        for (sample_rate, ch) in [(16000u32, 1u16), (22050, 1), (44100, 1), (44100, 2)] {
            let mut dec = BinkAudioDecoder::new(sample_rate, ch, false, false);
            let n = dec.frame_len;
            let mut input = vec![0.0f32; n];
            for (k, slot) in input.iter_mut().enumerate() {
                if k < 20 || k == n / 2 - 2 || k == n - 1 {
                    *slot = (((k as f32) * 1.17 + 0.3).cos()) * 0.2;
                }
            }
            let expected = naive_bink_irdft(&input);
            dec.block_buf[0].copy_from_slice(&input);
            dec.transform_irdft(0);
            let max_err = dec.block_buf[0]
                .iter()
                .zip(expected.iter())
                .map(|(a, b)| (a - b).abs())
                .fold(0.0f32, f32::max);
            assert!(
                max_err < 1e-4,
                "RDFT mismatch for N={n} (sr={sample_rate}, ch={ch}): max_err={max_err}"
            );
        }
    }

    #[test]
    fn dct_and_rdft_silence_and_nonzero_synthesis() {
        // 1. DCT silence & non-zero tone (44100 Hz stereo)
        let mut dct = BinkAudioDecoder::new(44100, 2, true, false);
        let mut bw_sil = BitWriter::new();
        encode_simple_block(&mut bw_sil, 44100, 2, true, (0, 0, false), (0, 0, false), 0, None);
        let pkt_sil = wrap_audio_packet((1920 * 2 * 2) as u32, &bw_sil.into_bytes());
        let mut out = Vec::new();
        assert_eq!(dct.decode_packet(&pkt_sil, &mut out), 1920 * 2);
        assert!(out.iter().all(|&s| s == 0.0));

        // Reset and encode a single quantized spectral bin (bin 10, val=12, quant_idx=65).
        dct.reset();
        let mut bw_tone = BitWriter::new();
        encode_simple_block(
            &mut bw_tone,
            44100,
            2,
            true,
            (0, 0, false),
            (0, 0, false),
            65,
            Some((10, 12, false)),
        );
        let pkt_tone = wrap_audio_packet((1920 * 2 * 2) as u32, &bw_tone.into_bytes());
        assert_eq!(dct.decode_packet(&pkt_tone, &mut out), 1920 * 2);
        let expected_amp = (12.0f32 * dct.quant_table[65]).abs();
        let peak = out.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
        assert!(
            (peak - expected_amp).abs() < 1e-3 && peak > 0.1,
            "DCT tone peak={peak} expected={expected_amp}"
        );
        // Verify cosine waveform at sample 0 matches cos(pi * 10 * 1 / 4096) * expected_amp.
        let exp0 = expected_amp * ((PI * 10.0 / 4096.0).cos() as f32);
        assert!(
            (out[0] - exp0).abs() < 1e-3 && (out[1] - exp0).abs() < 1e-3,
            "DCT sample 0 got L={} R={} expected={exp0}",
            out[0],
            out[1]
        );

        // 2. RDFT non-zero tone (22050 Hz mono, frame_len=1024, output=960)
        let mut rdft = BinkAudioDecoder::new(22050, 1, false, false);
        let mut bw_rdft = BitWriter::new();
        // Bin k=4 => real part at index 8, imag part at index 9; put val=15 at index 9 (pure sine).
        encode_simple_block(
            &mut bw_rdft,
            22050,
            1,
            false,
            (0, 0, false),
            (0, 0, false),
            62,
            Some((9, 15, false)),
        );
        let pkt_rdft = wrap_audio_packet((960 * 2) as u32, &bw_rdft.into_bytes());
        assert_eq!(rdft.decode_packet(&pkt_rdft, &mut out), 960);
        let expected_sin_amp = 15.0f32 * rdft.quant_table[62];
        // At n=0, sin(0) == 0; at n = N / (4*k) = 1024 / 16 = 64, sin(pi/2) == 1.
        assert!(out[0].abs() < 1e-4, "RDFT sine at n=0 should be ~0, got {}", out[0]);
        assert!(
            (out[64] - expected_sin_amp).abs() < 1e-3 && expected_sin_amp > 0.1,
            "RDFT sine at n=64 got {} expected {expected_sin_amp}",
            out[64]
        );

        // 3. RDFT stereo (44100 Hz stereo => interleaved N=4096, overlap=256, output=3840 samples = 1920 frames)
        let mut rdft_st = BinkAudioDecoder::new(44100, 2, false, false);
        assert_eq!(rdft_st.frame_len, 4096);
        assert_eq!(rdft_st.overlap_len, 256);
        assert_eq!(rdft_st.output_frames_per_block(), 1920);
        let mut bw_st = BitWriter::new();
        // Bin k=2 (index 4 = cos(2*pi*2*n/4096)), negative sign bit, quant_idx=68, val=10.
        encode_simple_block(
            &mut bw_st,
            44100,
            2,
            false,
            (0, 0, false),
            (0, 0, false),
            68,
            Some((4, 10, true)),
        );
        let pkt_st = wrap_audio_packet((1920 * 2 * 2) as u32, &bw_st.into_bytes());
        assert_eq!(rdft_st.decode_packet(&pkt_st, &mut out), 3840);
        let exp_st0 = -(10.0f32 * rdft_st.quant_table[68]);
        assert!(
            (out[0] - exp_st0).abs() < 1e-3 && exp_st0.abs() > 0.1,
            "RDFT stereo n=0 got {} expected {exp_st0}",
            out[0]
        );
    }

    #[test]
    fn rle_runs_negative_coeffs_and_multiband_quant_switching() {
        // Construct a DCT mono 16000 Hz block (frame_len=512) using RLE flag=1 runs and
        // distinct band quantizers across bands, verifying band switching and signed coefficients.
        let mut dec = BinkAudioDecoder::new(16000, 1, true, false);
        assert!(dec.num_bands >= 10);
        let target_band = 5usize;
        let target_bin = dec.bands[target_band];
        assert!(target_bin > 16 && target_bin < dec.frame_len - 16);

        let mut bw = BitWriter::new();
        bw.write_bits(0, 2); // DCT skip
        bw.write_bink_float(0, 0, false); // c0 = 0
        bw.write_bink_float(0, 0, false); // c1 = 0
        for b in 0..dec.num_bands {
            // Put a distinct quant index (62) only in `target_band`, 0 elsewhere.
            let qidx = if b == target_band { 62u32 } else { 0u32 };
            bw.write_bits(qidx, 8);
        }

        // Encode coefficients 2..512 using RLE flag=1 (index 0 => 2*8=16 coeffs) for zero runs,
        // switching to a non-zero 8-coeff chunk containing `target_bin` with val=9, sign=1 (negative).
        let mut pos = 2usize;
        while pos < dec.frame_len {
            if target_bin >= pos && target_bin < pos + 8 {
                bw.write_bits(0, 1); // non-RLE: 8 coeffs
                bw.write_bits(4, 4); // width = 4 bits
                for p in pos..(pos + 8).min(dec.frame_len) {
                    if p == target_bin {
                        bw.write_bits(9, 4);
                        bw.write_bits(1, 1); // negative
                    } else {
                        bw.write_bits(0, 4);
                    }
                }
                pos += 8;
            } else if pos + 16 <= target_bin || pos > target_bin {
                bw.write_bits(1, 1); // RLE flag = 1
                bw.write_bits(0, 4); // RLE_LENGTH_TAB[0] = 2 => 16 zeros
                bw.write_bits(0, 4); // width = 0
                pos += 16;
            } else {
                bw.write_bits(0, 1); // 8 zeros
                bw.write_bits(0, 4);
                pos += 8;
            }
        }
        bw.align32();

        let pkt = wrap_audio_packet((480 * 2) as u32, &bw.into_bytes());
        let mut out = Vec::new();
        assert_eq!(dec.decode_packet(&pkt, &mut out), 480);

        // Verify sample 0 equals -9 * quant_table[62] * cos(pi * target_bin * 1 / (2 * 512)).
        let expected_coeff = -9.0f32 * dec.quant_table[62];
        let exp0 = expected_coeff * ((PI * (target_bin as f64) / 1024.0).cos() as f32);
        assert!(
            (out[0] - exp0).abs() < 1e-3 && exp0.abs() > 0.1,
            "multiband RLE sample 0 got {} expected {exp0} (band {target_band} bin {target_bin})",
            out[0]
        );
    }

    #[test]
    fn overlap_add_crossfade_and_reset() {
        // 16000 Hz mono DCT => frame_len=512, overlap_len=32, output_frames=480.
        let mut dec = BinkAudioDecoder::new(16000, 1, true, false);
        // Choose c0 so that c0 * root = 0.5 => c0 = 0.5 / root = 0.25 * sqrt(512) * 32768 = 185363.8...
        let target_dc = 0.5f32;
        let raw_c0 = (target_dc / dec.root).round() as u32; // power=23 => mantissa = raw_c0
        let actual_dc = (raw_c0 as f32) * dec.root;

        let mut bw = BitWriter::new();
        // Block 0: constant +actual_dc
        encode_simple_block(&mut bw, 16000, 1, true, (23, raw_c0, false), (0, 0, false), 0, None);
        // Block 1: constant -actual_dc
        encode_simple_block(&mut bw, 16000, 1, true, (23, raw_c0, true), (0, 0, false), 0, None);
        let pkt = wrap_audio_packet((480 * 2 * 2) as u32, &bw.into_bytes());

        let mut out = Vec::new();
        assert_eq!(dec.decode_packet(&pkt, &mut out), 960);

        // Block 0 (samples 0..480): no overlap fade on first block => all +actual_dc.
        for (i, &s) in out[..480].iter().enumerate() {
            assert!(
                (s - actual_dc).abs() < 1e-3,
                "block 0 sample {i}: got {s}, expected {actual_dc}"
            );
        }
        // Block 1 overlap region (samples 480..512): linear crossfade from +actual_dc to -actual_dc.
        let olen = dec.overlap_len;
        assert_eq!(olen, 32);
        for i in 0..olen {
            let expected = (actual_dc * ((olen - i) as f32) + (-actual_dc) * (i as f32)) / (olen as f32);
            let got = out[480 + i];
            assert!(
                (got - expected).abs() < 1e-3,
                "block 1 overlap sample {i}: got {got}, expected {expected}"
            );
        }
        // Block 1 post-overlap (samples 512..960): steady -actual_dc.
        for (i, &s) in out[480 + olen..960].iter().enumerate() {
            assert!(
                (s - (-actual_dc)).abs() < 1e-3,
                "block 1 steady sample {i}: got {s}, expected {}",
                -actual_dc
            );
        }

        // After reset(), a new block with +actual_dc must NOT fade from previous -actual_dc.
        dec.reset();
        let mut bw2 = BitWriter::new();
        encode_simple_block(&mut bw2, 16000, 1, true, (23, raw_c0, false), (0, 0, false), 0, None);
        let pkt2 = wrap_audio_packet((480 * 2) as u32, &bw2.into_bytes());
        assert_eq!(dec.decode_packet(&pkt2, &mut out), 480);
        assert!((out[0] - actual_dc).abs() < 1e-3);
    }

    #[test]
    fn volume_and_pan_scaling() {
        assert!((volume_to_gain(0) - 0.0).abs() < 1e-6);
        assert!((volume_to_gain(16384) - 0.5).abs() < 1e-6);
        assert!((volume_to_gain(32768) - 1.0).abs() < 1e-6);
        assert!((volume_to_gain(65536) - 2.0).abs() < 1e-6);
        assert!((volume_to_gain(-500) - 0.0).abs() < 1e-6);
        assert!((volume_to_gain(999_999) - 2.0).abs() < 1e-6);

        assert_eq!(pan_to_gains(0), (1.0, 0.0));
        assert_eq!(pan_to_gains(32768), (1.0, 1.0));
        assert_eq!(pan_to_gains(65536), (0.0, 1.0));
        let (l_left, r_left) = pan_to_gains(16384);
        assert!((l_left - 1.0).abs() < 1e-6 && (r_left - 0.5).abs() < 1e-6);
        let (l_right, r_right) = pan_to_gains(49152);
        assert!((l_right - 0.5).abs() < 1e-6 && (r_right - 1.0).abs() < 1e-6);

        // Verify mono-to-stereo mix honors volume + pan.
        let mut dst = Vec::new();
        mix_track_into_stereo(&[0.5, -0.5], 1, 48000, 48000, 16384, 0, &mut dst);
        assert_eq!(dst.len(), 4);
        assert!((dst[0] - 0.25).abs() < 1e-6); // L: 0.5 * 0.5 * 1.0
        assert!((dst[1] - 0.0).abs() < 1e-6); // R: 0.5 * 0.5 * 0.0
        assert!((dst[2] - (-0.25)).abs() < 1e-6);
        assert!((dst[3] - 0.0).abs() < 1e-6);
    }

    #[test]
    fn truncated_and_malformed_packets_never_panic() {
        for use_dct in [true, false] {
            let mut dec = BinkAudioDecoder::new(44100, 2, use_dct, false);
            let mut out = Vec::new();

            // Empty & sub-header packets
            assert_eq!(dec.decode_packet(&[], &mut out), 0);
            assert_eq!(dec.decode_packet(&[1, 2, 3], &mut out), 0);
            // Zero reported size
            assert_eq!(dec.decode_packet(&[0, 0, 0, 0, 0xFF, 0xFF, 0xFF, 0xFF], &mut out), 0);
            // Huge corrupt reported_size with tiny payload
            assert_eq!(dec.decode_packet(&[0xFF, 0xFF, 0xFF, 0xFF, 0xAA, 0xBB], &mut out), 0);

            // Valid block truncated at every byte prefix must fail gracefully without panicking.
            let mut bw = BitWriter::new();
            encode_simple_block(
                &mut bw,
                44100,
                2,
                use_dct,
                (15, 1000, false),
                (15, 2000, true),
                30,
                Some((12, 7, false)),
            );
            let full_pkt = wrap_audio_packet((1920 * 2 * 2) as u32, &bw.into_bytes());
            for cut in 0..full_pkt.len() {
                dec.reset();
                let _ = dec.decode_packet(&full_pkt[..cut], &mut out);
                assert!(out.iter().all(|s| s.is_finite()));
            }
        }
    }

    #[test]
    fn bink_audio_player_writes_tap_wav() {
        let _audio_guard = crate::audio::audio_test_lock();
        backend::force_backend(Some(backend::Backend::Null));

        let dir = std::env::temp_dir().join(format!("bink2-tap-test-{}", std::process::id()));
        let wav = dir.join("bink2.wav");
        crate::audio::tap::override_next(Some(wav.clone()));

        let mut bw = BitWriter::new();
        encode_simple_block(
            &mut bw,
            48_000,
            1,
            true,
            (0, 0, false),
            (0, 0, false),
            65,
            Some((10, 12, false)),
        );
        let pkt = wrap_audio_packet((1920 * 2) as u32, &bw.into_bytes());

        {
            let track = ActiveAudioTrack {
                file_track_index: 0,
                track_id: 0,
                sample_rate: 48_000,
                channels: 1,
                volume: BINK_UNITY_VOLUME,
                pan: 0, // full left
                decoder: BinkAudioDecoder::new(48_000, 1, true, false),
            };
            let mut player = BinkAudioPlayer::new(vec![track], true);
            player.submit_frame_packets(&pkt, &[(0, pkt.len())]);
        }

        let bytes = std::fs::read(&wav).expect("bink2 tap wav must exist");
        assert_eq!(&bytes[0..4], b"RIFF");
        let data = &bytes[crate::audio::tap::DATA_OFFSET as usize..];
        assert_eq!(data.len(), 1920 * 2 * 4);
        let samples: Vec<f32> = data
            .chunks_exact(4)
            .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
            .collect();
        let peak_l = samples.iter().step_by(2).map(|v| v.abs()).fold(0.0f32, f32::max);
        let peak_r = samples.iter().skip(1).step_by(2).map(|v| v.abs()).fold(0.0f32, f32::max);
        assert!(peak_l > 0.1, "expected non-silent tapped L audio, got {peak_l}");
        assert!(peak_r < 1e-6, "expected full-left pan to silence R, got {peak_r}");
        let _ = std::fs::remove_dir_all(dir);
    }
}
