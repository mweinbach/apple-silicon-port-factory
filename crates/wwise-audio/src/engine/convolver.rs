//! Wwise Convolution Reverb preprocessed IR media parser (`00 04 02 00` / `00 04 00 00` and
//! `RIFF`/`WAVE`) and partitioned overlap-add FFT convolution engine.
//!
//! Format verified from `AkConvolutionReverbFX::Init` (`0x1414ef170`),
//! `CAkHalfPrecisionConvolutionEngine::Init` / `Execute` (`0x1414f3480` / `0x1414f2f90`),
//! `CAkPartitionedConvolutionEngine::Init` / `Execute` (`0x1414f2a10` / `0x1414f2570`), and the
//! bfloat16 pack/MAC kernels (`0x1414f6c50` / `0x1414f69e0`):
//!
//! ```text
//! Offset  Size  Type   Field
//! 0x00    2     u16    uBlockLength (e.g. 0x0400 = 1024 samples per partition)
//! 0x02    2     u16    uMediaType (0 = Float32 partitioned FFT, 2 = BFloat16 half-precision FFT)
//! 0x04    4     u32    uFFTLength (power of two, 2 * uBlockLength, e.g. 2048)
//! 0x08    4     u32    uSampleRate (e.g. 48000 Hz)
//! 0x0c    4     u32    uChannelConfig (AkChannelConfig; low byte = channel count)
//! 0x10    4     u32    uOrigPartitions / flags
//! 0x14    4     f32    fIrDurationOrMetric
//! 0x18    4     f32    fIrLevelDb (authoring normalization metadata; already baked into bins)
//! 0x1c    4     u32    uNumPartitions (> 0)
//! 0x20    4     u32    uChannelStrideBins (total complex bins stored per channel = sum(part_bins))
//! 0x24    2     u16    uMaxBinsPerPartition (typically uFFTLength / 2, e.g. 1024)
//! 0x26    10    bytes  reserved
//! 0x30    2*P   u16[]  per-partition stored complex bin counts (high-freq tail truncation)
//! align16(...)         channel 0..C partitioned FFT spectra, each channel occupying
//!                      uChannelStrideBins * (4 for BFloat16, 8 for Float32) bytes:
//!                      - BFloat16 (type 2): per bin `(re_bf16: u16, im_bf16: u16)`, unpacked via
//!                        `f32::from_bits((bits as u32) << 16)` (upper 16 bits of IEEE-754 f32).
//!                      - Float32  (type 0): per bin `(re: f32, im: f32)`.
//! ```
//!
//! Each partition spectrum is the standard unscaled forward real-to-complex DFT
//! `H[k] = sum_{n=0}^{B-1} h[n] * exp(-j * 2*pi*k*n / N)` of the zero-padded `B`-sample
//! time-domain IR slice (`N = uFFTLength = 2 * B`), convolved via overlap-add with `1/N` IFFT.

use std::collections::VecDeque;
use std::f32::consts::PI;

/// Header size of Wwise's preprocessed Convolution Reverb IR binary format.
pub const WWISE_IR_HEADER_SIZE: usize = 0x30;

/// Spectral bin storage precision inside a Wwise preprocessed IR blob (`uMediaType`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrPrecision {
    /// `uMediaType == 0`: IEEE-754 `f32` complex pairs `(re, im)` (8 bytes/bin).
    Float32 = 0,
    /// `uMediaType == 2`: bfloat16 (`f32` upper 16 bits) complex pairs `(re, im)` (4 bytes/bin).
    BFloat16 = 2,
}

impl IrPrecision {
    pub fn from_media_type(media_type: u16) -> Option<Self> {
        match media_type {
            0 => Some(Self::Float32),
            2 => Some(Self::BFloat16),
            _ => None,
        }
    }

    pub fn bytes_per_bin(self) -> usize {
        match self {
            Self::Float32 => 8,
            Self::BFloat16 => 4,
        }
    }
}

#[inline]
pub fn bf16_to_f32(bits: u16) -> f32 {
    f32::from_bits((bits as u32) << 16)
}

#[inline]
pub fn f32_to_bf16(v: f32) -> u16 {
    (v.to_bits() >> 16) as u16
}

/// Complex number `(re, im)` used by the partitioned FFT convolver.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Complex32 {
    pub re: f32,
    pub im: f32,
}

impl Complex32 {
    #[inline]
    pub const fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }
}

/// Parsed impulse response ready for partitioned overlap-add FFT convolution.
#[derive(Debug, Clone)]
pub struct PreprocessedIr {
    pub block_length: usize,
    pub precision: IrPrecision,
    pub fft_length: usize,
    pub sample_rate: u32,
    pub channel_config: u32,
    pub num_channels: usize,
    pub orig_partitions: u32,
    pub ir_metric: f32,
    pub ir_level_db: f32,
    pub num_partitions: usize,
    pub channel_stride_bins: usize,
    pub max_bins_per_partition: usize,
    /// Number of stored complex frequency bins for each partition `0..num_partitions`.
    pub partition_bins: Vec<u16>,
    /// Partition start bin offset within each channel's flat spectrum buffer (`len == num_partitions`).
    pub partition_offsets: Vec<usize>,
    /// Decoded complex spectra per channel: `channels[ch][partition_offsets[p] + k]` for `k < partition_bins[p]`.
    pub channels: Vec<Vec<Complex32>>,
}

impl PreprocessedIr {
    /// Parse a Convolution Reverb IR asset from either:
    /// - Wwise's preprocessed partitioned FFT format (`00 04 02 00` bfloat16 or `00 04 00 00` float32), or
    /// - A standard `RIFF`/`WAVE` impulse response (PCM16, IEEE float32, WAVEFORMATEXTENSIBLE, or Wwise `.wem`).
    pub fn parse(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WAVE" {
            return Self::parse_riff_wave(bytes, 1024);
        }
        Self::parse_wwise_preprocessed(bytes)
    }

    /// Parse Wwise's 48-byte-header preprocessed Convolution Reverb IR media (`uMediaType` 0 or 2).
    pub fn parse_wwise_preprocessed(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() <= WWISE_IR_HEADER_SIZE {
            return Err(format!(
                "Wwise IR media too small ({} bytes, need > {})",
                bytes.len(),
                WWISE_IR_HEADER_SIZE
            ));
        }
        let u16_at = |off: usize| u16::from_le_bytes(bytes[off..off + 2].try_into().unwrap());
        let u32_at = |off: usize| u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
        let f32_at = |off: usize| f32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());

        let block_length = u16_at(0x00) as usize;
        let media_type = u16_at(0x02);
        let precision = IrPrecision::from_media_type(media_type)
            .ok_or_else(|| format!("unsupported Wwise IR media type {media_type}"))?;
        let fft_length = u32_at(0x04) as usize;
        let sample_rate = u32_at(0x08);
        let channel_config = u32_at(0x0c);
        let num_channels = (channel_config & 0xFF) as usize;
        let orig_partitions = u32_at(0x10);
        let ir_metric = f32_at(0x14);
        let ir_level_db = f32_at(0x18);
        let num_partitions = u32_at(0x1c) as usize;
        let channel_stride_bins = u32_at(0x20) as usize;
        let max_bins_per_partition = u16_at(0x24) as usize;

        if block_length == 0 || !block_length.is_power_of_two() {
            return Err(format!("invalid Wwise IR block_length {block_length}"));
        }
        if fft_length < block_length * 2 || !fft_length.is_power_of_two() || fft_length > 65_536 {
            return Err(format!(
                "invalid Wwise IR fft_length {fft_length} for block_length {block_length}"
            ));
        }
        if sample_rate == 0 {
            return Err("invalid Wwise IR sample_rate 0".into());
        }
        if num_channels == 0 || num_channels > 16 {
            return Err(format!("invalid Wwise IR channel count {num_channels} (cfg {channel_config:#x})"));
        }
        if num_partitions == 0 || num_partitions > 65_536 {
            return Err(format!("invalid Wwise IR num_partitions {num_partitions}"));
        }

        let table_bytes = num_partitions
            .checked_mul(2)
            .ok_or_else(|| "Wwise IR partition table overflow".to_string())?;
        let data_offset = WWISE_IR_HEADER_SIZE
            .checked_add((table_bytes + 15) & !15)
            .ok_or_else(|| "Wwise IR data offset overflow".to_string())?;
        if bytes.len() < data_offset {
            return Err(format!(
                "truncated Wwise IR partition table (len {}, need >= {})",
                bytes.len(),
                data_offset
            ));
        }

        let mut partition_bins = Vec::with_capacity(num_partitions);
        let mut partition_offsets = Vec::with_capacity(num_partitions);
        let mut total_bins = 0usize;
        let bin_cap = fft_length / 2 + 1;
        for p in 0..num_partitions {
            let nb = u16_at(WWISE_IR_HEADER_SIZE + p * 2);
            if (nb as usize) > bin_cap {
                return Err(format!(
                    "Wwise IR partition {p} bin count {nb} exceeds FFT half-spectrum {bin_cap}"
                ));
            }
            partition_offsets.push(total_bins);
            partition_bins.push(nb);
            total_bins = total_bins
                .checked_add(nb as usize)
                .ok_or_else(|| "Wwise IR bin count overflow".to_string())?;
        }
        if channel_stride_bins < total_bins {
            return Err(format!(
                "Wwise IR channel_stride_bins {channel_stride_bins} < sum(partition_bins) {total_bins}"
            ));
        }

        let bpb = precision.bytes_per_bin();
        let ch_bytes = channel_stride_bins
            .checked_mul(bpb)
            .ok_or_else(|| "Wwise IR channel byte size overflow".to_string())?;
        let payload_bytes = num_channels
            .checked_mul(ch_bytes)
            .ok_or_else(|| "Wwise IR payload size overflow".to_string())?;
        let needed = data_offset
            .checked_add(payload_bytes)
            .ok_or_else(|| "Wwise IR total size overflow".to_string())?;
        if bytes.len() < needed {
            return Err(format!(
                "truncated Wwise IR spectral payload (len {}, need {})",
                bytes.len(),
                needed
            ));
        }

        let mut channels = Vec::with_capacity(num_channels);
        for c in 0..num_channels {
            let ch_base = data_offset + c * ch_bytes;
            let mut bins = Vec::with_capacity(total_bins);
            match precision {
                IrPrecision::BFloat16 => {
                    for k in 0..total_bins {
                        let off = ch_base + k * 4;
                        let re = bf16_to_f32(u16_at(off));
                        let im = bf16_to_f32(u16_at(off + 2));
                        bins.push(Complex32::new(re, im));
                    }
                }
                IrPrecision::Float32 => {
                    for k in 0..total_bins {
                        let off = ch_base + k * 8;
                        let re = f32_at(off);
                        let im = f32_at(off + 4);
                        bins.push(Complex32::new(re, im));
                    }
                }
            }
            channels.push(bins);
        }

        Ok(Self {
            block_length,
            precision,
            fft_length,
            sample_rate,
            channel_config,
            num_channels,
            orig_partitions,
            ir_metric,
            ir_level_db,
            num_partitions,
            channel_stride_bins,
            max_bins_per_partition: if max_bins_per_partition == 0 {
                fft_length / 2
            } else {
                max_bins_per_partition
            },
            partition_bins,
            partition_offsets,
            channels,
        })
    }

    /// Parse a standard `RIFF`/`WAVE` (or Wwise `.wem`) impulse response into partitioned FFT form.
    pub fn parse_riff_wave(bytes: &[u8], block_length: usize) -> Result<Self, String> {
        let mut pos = 12usize;
        let mut format_tag = 0u16;
        let mut num_channels = 0u16;
        let mut sample_rate = 0u32;
        let mut bits_per_sample = 0u16;
        let mut subformat_tag = 0u16;
        let mut data_slice: Option<&[u8]> = None;

        while pos + 8 <= bytes.len() {
            let id = &bytes[pos..pos + 4];
            let sz = u32::from_le_bytes(bytes[pos + 4..pos + 8].try_into().unwrap()) as usize;
            let body_start = pos + 8;
            let body_end = body_start.saturating_add(sz).min(bytes.len());
            let body = &bytes[body_start..body_end];
            if id == b"fmt " && body.len() >= 16 {
                format_tag = u16::from_le_bytes(body[0..2].try_into().unwrap());
                num_channels = u16::from_le_bytes(body[2..4].try_into().unwrap());
                sample_rate = u32::from_le_bytes(body[4..8].try_into().unwrap());
                bits_per_sample = u16::from_le_bytes(body[14..16].try_into().unwrap());
                if format_tag == 0xFFFE && body.len() >= 26 {
                    subformat_tag = u16::from_le_bytes(body[24..26].try_into().unwrap());
                }
            } else if id == b"data" {
                data_slice = Some(body);
            }
            pos = body_start.saturating_add(sz + (sz & 1));
        }

        let eff_tag = if format_tag == 0xFFFE && subformat_tag != 0 {
            subformat_tag
        } else {
            format_tag
        };

        // Decode standard PCM16 / IEEE float32 directly, or delegate Wwise-coded `.wem` to `decode_wem`.
        let (sr, planar) = match (eff_tag, bits_per_sample) {
            (1, 16) | (0xFFFE, 16) => {
                let nch = num_channels as usize;
                let data = data_slice.ok_or_else(|| "RIFF/WAVE IR missing data chunk".to_string())?;
                if nch == 0 || sample_rate == 0 {
                    return Err("RIFF/WAVE IR missing valid fmt chunk".into());
                }
                let frames = data.len() / (nch * 2);
                let mut chs = vec![Vec::with_capacity(frames); nch];
                for f in 0..frames {
                    for (c, ch) in chs.iter_mut().enumerate() {
                        let off = (f * nch + c) * 2;
                        let s = i16::from_le_bytes(data[off..off + 2].try_into().unwrap());
                        ch.push(s as f32 / 32768.0);
                    }
                }
                (sample_rate, chs)
            }
            (3, 32) => {
                let nch = num_channels as usize;
                let data = data_slice.ok_or_else(|| "RIFF/WAVE IR missing data chunk".to_string())?;
                if nch == 0 || sample_rate == 0 {
                    return Err("RIFF/WAVE IR missing valid fmt chunk".into());
                }
                let frames = data.len() / (nch * 4);
                let mut chs = vec![Vec::with_capacity(frames); nch];
                for f in 0..frames {
                    for (c, ch) in chs.iter_mut().enumerate() {
                        let off = (f * nch + c) * 4;
                        let s = f32::from_le_bytes(data[off..off + 4].try_into().unwrap());
                        ch.push(s);
                    }
                }
                (sample_rate, chs)
            }
            _ => {
                let dec = crate::codec::decode_wem(bytes)?;
                let nch = dec.channels as usize;
                if nch == 0 {
                    return Err("decoded WEM IR has 0 channels".into());
                }
                let frames = dec.samples.len() / nch;
                let mut chs = vec![Vec::with_capacity(frames); nch];
                for f in 0..frames {
                    for (c, ch) in chs.iter_mut().enumerate() {
                        ch.push(dec.samples[f * nch + c]);
                    }
                }
                (dec.sample_rate, chs)
            }
        };

        Self::from_time_domain(sr, block_length, &planar, IrPrecision::Float32)
    }

    /// Build a [`PreprocessedIr`] by partitioning planar time-domain impulse responses into
    /// `block_length`-sample zero-padded FFT blocks (`fft_length = 2 * block_length`).
    pub fn from_time_domain(
        sample_rate: u32,
        block_length: usize,
        time_channels: &[Vec<f32>],
        precision: IrPrecision,
    ) -> Result<Self, String> {
        if !block_length.is_power_of_two() || block_length < 16 {
            return Err(format!("block_length must be a power of two >= 16, got {block_length}"));
        }
        if time_channels.is_empty() {
            return Err("time-domain IR must have at least 1 channel".into());
        }
        let max_len = time_channels.iter().map(|c| c.len()).max().unwrap_or(0);
        if max_len == 0 {
            return Err("time-domain IR has 0 samples".into());
        }

        let fft_length = block_length * 2;
        let num_bins = fft_length / 2; // matches Wwise's uMaxBinsPerPartition = N/2
        let num_partitions = max_len.div_ceil(block_length);
        let fft = Radix2Fft::new(fft_length);

        let partition_bins = vec![num_bins as u16; num_partitions];
        let partition_offsets: Vec<usize> = (0..num_partitions).map(|p| p * num_bins).collect();
        let channel_stride_bins = num_partitions * num_bins;

        let mut channels = Vec::with_capacity(time_channels.len());
        let mut buf = vec![Complex32::default(); fft_length];
        for tc in time_channels {
            let mut ch_spec = Vec::with_capacity(channel_stride_bins);
            for p in 0..num_partitions {
                buf.fill(Complex32::default());
                let start = p * block_length;
                if start < tc.len() {
                    let take = (tc.len() - start).min(block_length);
                    for i in 0..take {
                        buf[i].re = tc[start + i];
                    }
                }
                fft.forward(&mut buf);
                // Packed real-FFT layout (`uMaxBinsPerPartition = N/2`): bin 0 carries
                // `(Re(DC), Re(Nyquist))` and bins `1..N/2` carry standard complex pairs.
                buf[0].im = buf[num_bins].re;
                for &bin in &buf[..num_bins] {
                    match precision {
                        IrPrecision::Float32 => ch_spec.push(bin),
                        IrPrecision::BFloat16 => ch_spec.push(Complex32::new(
                            bf16_to_f32(f32_to_bf16(bin.re)),
                            bf16_to_f32(f32_to_bf16(bin.im)),
                        )),
                    }
                }
            }
            channels.push(ch_spec);
        }

        let nch = time_channels.len() as u32;
        let mask = match nch {
            1 => 0x0004,
            2 => 0x0003,
            4 => 0x0033,
            6 => 0x060F,
            _ => 0,
        };
        let channel_config = nch | (1 << 8) | (mask << 12);

        Ok(Self {
            block_length,
            precision,
            fft_length,
            sample_rate,
            channel_config,
            num_channels: time_channels.len(),
            orig_partitions: num_partitions as u32,
            ir_metric: max_len as f32 / sample_rate.max(1) as f32,
            ir_level_db: 0.0,
            num_partitions,
            channel_stride_bins,
            max_bins_per_partition: num_bins,
            partition_bins,
            partition_offsets,
            channels,
        })
    }

    /// Serialize this IR in Wwise's exact binary preprocessed IR container format
    /// (`00 04 02 00` for `BFloat16` with `block_length == 1024`, or `00 04 00 00` for `Float32`).
    pub fn encode_wwise_bytes(&self, precision: IrPrecision) -> Vec<u8> {
        let table_bytes = self.num_partitions * 2;
        let data_offset = WWISE_IR_HEADER_SIZE + ((table_bytes + 15) & !15);
        let bpb = precision.bytes_per_bin();
        let total_size = data_offset + self.num_channels * self.channel_stride_bins * bpb;
        let mut out = vec![0u8; total_size];

        out[0x00..0x02].copy_from_slice(&(self.block_length as u16).to_le_bytes());
        out[0x02..0x04].copy_from_slice(&(precision as u16).to_le_bytes());
        out[0x04..0x08].copy_from_slice(&(self.fft_length as u32).to_le_bytes());
        out[0x08..0x0c].copy_from_slice(&self.sample_rate.to_le_bytes());
        out[0x0c..0x10].copy_from_slice(&self.channel_config.to_le_bytes());
        out[0x10..0x14].copy_from_slice(&self.orig_partitions.to_le_bytes());
        out[0x14..0x18].copy_from_slice(&self.ir_metric.to_le_bytes());
        out[0x18..0x1c].copy_from_slice(&self.ir_level_db.to_le_bytes());
        out[0x1c..0x20].copy_from_slice(&(self.num_partitions as u32).to_le_bytes());
        out[0x20..0x24].copy_from_slice(&(self.channel_stride_bins as u32).to_le_bytes());
        out[0x24..0x26].copy_from_slice(&(self.max_bins_per_partition as u16).to_le_bytes());

        for (p, &nb) in self.partition_bins.iter().enumerate() {
            let off = WWISE_IR_HEADER_SIZE + p * 2;
            out[off..off + 2].copy_from_slice(&nb.to_le_bytes());
        }

        let ch_bytes = self.channel_stride_bins * bpb;
        for (c, ch) in self.channels.iter().enumerate() {
            let ch_base = data_offset + c * ch_bytes;
            match precision {
                IrPrecision::BFloat16 => {
                    for (k, bin) in ch.iter().enumerate() {
                        let off = ch_base + k * 4;
                        out[off..off + 2].copy_from_slice(&f32_to_bf16(bin.re).to_le_bytes());
                        out[off + 2..off + 4].copy_from_slice(&f32_to_bf16(bin.im).to_le_bytes());
                    }
                }
                IrPrecision::Float32 => {
                    for (k, bin) in ch.iter().enumerate() {
                        let off = ch_base + k * 8;
                        out[off..off + 4].copy_from_slice(&bin.re.to_le_bytes());
                        out[off + 4..off + 8].copy_from_slice(&bin.im.to_le_bytes());
                    }
                }
            }
        }
        out
    }

    /// Total time-domain impulse length in samples (`block_length * num_partitions`).
    pub fn total_samples(&self) -> usize {
        self.block_length * self.num_partitions
    }
}

/// Iterative radix-2 Cooley-Tukey FFT/IFFT with precomputed bit-reversal and twiddle tables.
#[derive(Debug, Clone)]
pub struct Radix2Fft {
    n: usize,
    bit_rev: Vec<usize>,
    /// Stage twiddles for forward (`exp(-j 2*pi*k/len)`); inverse uses conjugate.
    twiddles_fwd: Vec<Vec<Complex32>>,
}

impl Radix2Fft {
    pub fn new(n: usize) -> Self {
        assert!(n.is_power_of_two() && n >= 2, "FFT size must be a power of 2 >= 2");
        let bits = n.trailing_zeros();
        let bit_rev: Vec<usize> = (0..n)
            .map(|i| (i.reverse_bits()) >> (usize::BITS - bits))
            .collect();

        let mut twiddles_fwd = Vec::with_capacity(bits as usize);
        let mut half = 1usize;
        while half < n {
            let len = half * 2;
            let step = -2.0 * PI / (len as f32);
            let stage: Vec<Complex32> = (0..half)
                .map(|k| {
                    let ang = step * (k as f32);
                    Complex32::new(ang.cos(), ang.sin())
                })
                .collect();
            twiddles_fwd.push(stage);
            half = len;
        }
        Self { n, bit_rev, twiddles_fwd }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.n
    }

    /// In-place unscaled forward DFT: `X[k] = sum_n x[n] * exp(-j 2*pi*k*n / N)`.
    pub fn forward(&self, data: &mut [Complex32]) {
        self.transform(data, false);
    }

    /// In-place scaled inverse DFT: `x[n] = (1/N) * sum_k X[k] * exp(+j 2*pi*k*n / N)`.
    pub fn inverse(&self, data: &mut [Complex32]) {
        self.transform(data, true);
        let inv_n = 1.0f32 / (self.n as f32);
        for c in data.iter_mut() {
            c.re *= inv_n;
            c.im *= inv_n;
        }
    }

    fn transform(&self, data: &mut [Complex32], inverse: bool) {
        debug_assert_eq!(data.len(), self.n);
        for i in 0..self.n {
            let j = self.bit_rev[i];
            if i < j {
                data.swap(i, j);
            }
        }
        let mut half = 1usize;
        for stage in &self.twiddles_fwd {
            let len = half * 2;
            let mut base = 0usize;
            while base < self.n {
                for k in 0..half {
                    let w = stage[k];
                    let w_im = if inverse { -w.im } else { w.im };
                    let u = data[base + k];
                    let v = data[base + k + half];
                    let t_re = v.re * w.re - v.im * w_im;
                    let t_im = v.re * w_im + v.im * w.re;
                    data[base + k] = Complex32::new(u.re + t_re, u.im + t_im);
                    data[base + k + half] = Complex32::new(u.re - t_re, u.im - t_im);
                }
                base += len;
            }
            half = len;
        }
    }
}

/// Uniformly partitioned overlap-add FFT convolution engine matching Wwise's
/// `CAkHalfPrecisionConvolutionEngine` / `CAkPartitionedConvolutionEngine`.
#[derive(Debug, Clone)]
pub struct PartitionedConvolver {
    ir: PreprocessedIr,
    fft: Radix2Fft,
    /// Number of half-spectrum bins tracked in the FDL (`fft_length / 2 + 1`).
    half_bins: usize,
    /// Per-IR-channel input staging buffer (accumulates up to `block_length` samples).
    in_buf: Vec<Vec<f32>>,
    in_len: usize,
    /// Per-IR-channel ready output sample FIFO.
    out_queue: Vec<VecDeque<f32>>,
    /// Per-IR-channel frequency-domain delay line: `fdl[ch][slot][bin]`.
    fdl: Vec<Vec<Vec<Complex32>>>,
    /// Per-IR-channel energy flag per FDL slot (skip zero input partitions like AK's silence check).
    fdl_active: Vec<Vec<bool>>,
    /// Next write slot in `fdl` (`0..num_partitions`).
    fdl_head: usize,
    /// Per-IR-channel overlap tail (`block_length` samples).
    overlap: Vec<Vec<f32>>,
    /// Scratch FFT buffer (`fft_length` complex samples).
    fft_scratch: Vec<Complex32>,
    /// Scratch complex accumulator (`half_bins` complex samples).
    acc_scratch: Vec<Complex32>,
}

impl PartitionedConvolver {
    pub fn new(ir: PreprocessedIr) -> Self {
        let nch = ir.num_channels;
        let b = ir.block_length;
        let n = ir.fft_length;
        let half_bins = n / 2 + 1;
        let np = ir.num_partitions;
        let fft = Radix2Fft::new(n);

        Self {
            ir,
            fft,
            half_bins,
            in_buf: vec![vec![0.0f32; b]; nch],
            in_len: 0,
            out_queue: vec![VecDeque::with_capacity(b * 2); nch],
            fdl: vec![vec![vec![Complex32::default(); half_bins]; np]; nch],
            fdl_active: vec![vec![false; np]; nch],
            fdl_head: 0,
            overlap: vec![vec![0.0f32; b]; nch],
            fft_scratch: vec![Complex32::default(); n],
            acc_scratch: vec![Complex32::default(); half_bins],
        }
    }

    pub fn ir(&self) -> &PreprocessedIr {
        &self.ir
    }

    pub fn num_channels(&self) -> usize {
        self.ir.num_channels
    }

    pub fn block_length(&self) -> usize {
        self.ir.block_length
    }

    pub fn reset(&mut self) {
        self.in_len = 0;
        self.fdl_head = 0;
        for b in &mut self.in_buf {
            b.fill(0.0);
        }
        for q in &mut self.out_queue {
            q.clear();
        }
        for ch in &mut self.fdl {
            for slot in ch {
                slot.fill(Complex32::default());
            }
        }
        for ch in &mut self.fdl_active {
            ch.fill(false);
        }
        for ov in &mut self.overlap {
            ov.fill(0.0);
        }
    }

    /// Convolve `input[ch][0..frames]` into `output[ch][0..frames]` (one channel per IR channel).
    ///
    /// When `frames < block_length` (e.g. 512-sample engine frames with a 1024-sample Wwise IR
    /// block), the first `block_length - frames` samples output silence while the initial block
    /// accumulates, matching `AkConvolutionReverbFX`'s buffering latency (`uLatency = B - frames`).
    /// When `frames >= block_length` and is a multiple of `block_length`, latency is zero.
    pub fn process_channels(&mut self, input: &[Vec<f32>], output: &mut [Vec<f32>], frames: usize) {
        let nch = self.ir.num_channels;
        debug_assert_eq!(input.len(), nch);
        debug_assert_eq!(output.len(), nch);
        if frames == 0 {
            return;
        }
        let b = self.ir.block_length;
        let mut cursor = 0usize;
        while cursor < frames {
            let space = b - self.in_len;
            let take = (frames - cursor).min(space);
            for c in 0..nch {
                self.in_buf[c][self.in_len..self.in_len + take]
                    .copy_from_slice(&input[c][cursor..cursor + take]);
            }
            self.in_len += take;
            cursor += take;
            if self.in_len == b {
                self.process_partition_block();
                self.in_len = 0;
            }
        }

        for c in 0..nch {
            let q = &mut self.out_queue[c];
            let out_ch = &mut output[c];
            if q.len() >= frames {
                for s in out_ch.iter_mut().take(frames) {
                    *s = q.pop_front().unwrap_or(0.0);
                }
            } else {
                // Initial priming frames before the first full `block_length` input block completes.
                let pad = frames - q.len();
                for s in out_ch.iter_mut().take(pad) {
                    *s = 0.0;
                }
                for s in out_ch.iter_mut().skip(pad).take(frames - pad) {
                    *s = q.pop_front().unwrap_or(0.0);
                }
            }
        }
    }

    fn process_partition_block(&mut self) {
        let nch = self.ir.num_channels;
        let b = self.ir.block_length;
        let n = self.ir.fft_length;
        let np = self.ir.num_partitions;
        let head = self.fdl_head;

        for c in 0..nch {
            let is_silent = self.in_buf[c].iter().all(|&x| x == 0.0);
            self.fdl_active[c][head] = !is_silent;
            if !is_silent {
                // Zero-pad input block [in[0..B], 0..0] to fft_length and transform.
                for i in 0..b {
                    self.fft_scratch[i] = Complex32::new(self.in_buf[c][i], 0.0);
                }
                for i in b..n {
                    self.fft_scratch[i] = Complex32::default();
                }
                self.fft.forward(&mut self.fft_scratch);
                // Pack `(Re(DC), Re(Nyquist))` into bin 0 to match Wwise's `N/2`-bin real-FFT layout.
                self.fft_scratch[0].im = self.fft_scratch[n / 2].re;
                self.fdl[c][head].copy_from_slice(&self.fft_scratch[..self.half_bins]);
            }

            // Complex multiply-accumulate across all partitions in the FDL ring.
            self.acc_scratch.fill(Complex32::default());
            let mut any_active = false;
            let ir_ch = &self.ir.channels[c];
            let mut slot = head;
            for p in 0..np {
                if self.fdl_active[c][slot] {
                    let nb = self.ir.partition_bins[p] as usize;
                    if nb > 0 {
                        any_active = true;
                        let ir_off = self.ir.partition_offsets[p];
                        let x_spec = &self.fdl[c][slot];
                        let h_spec = &ir_ch[ir_off..ir_off + nb];
                        // Bin 0 stores two real values: `(DC, Nyquist)`.
                        self.acc_scratch[0].re += x_spec[0].re * h_spec[0].re;
                        self.acc_scratch[0].im += x_spec[0].im * h_spec[0].im;
                        for k in 1..nb {
                            let x = x_spec[k];
                            let h = h_spec[k];
                            self.acc_scratch[k].re += x.re * h.re - x.im * h.im;
                            self.acc_scratch[k].im += x.re * h.im + x.im * h.re;
                        }
                    }
                }
                slot = if slot == 0 { np - 1 } else { slot - 1 };
            }

            let ov = &mut self.overlap[c];
            let q = &mut self.out_queue[c];
            if !any_active {
                for &s in ov.iter() {
                    q.push_back(s);
                }
                ov.fill(0.0);
            } else {
                // Reconstruct full Hermitian-symmetric spectrum and inverse FFT (scaled by 1/N).
                self.fft_scratch[0] = Complex32::new(self.acc_scratch[0].re, 0.0);
                let nyq = n / 2;
                for k in 1..nyq {
                    let y = self.acc_scratch[k];
                    self.fft_scratch[k] = y;
                    self.fft_scratch[n - k] = Complex32::new(y.re, -y.im);
                }
                self.fft_scratch[nyq] = Complex32::new(self.acc_scratch[0].im, 0.0);
                self.fft.inverse(&mut self.fft_scratch);

                // Overlap-add: emit first half + prior overlap, store second half as next overlap.
                for i in 0..b {
                    q.push_back(self.fft_scratch[i].re + ov[i]);
                    ov[i] = self.fft_scratch[b + i].re;
                }
            }
        }

        self.fdl_head = (head + 1) % np;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wwise_half_precision_00_04_02_00_roundtrip_and_exact_convolution() {
        // Build a synthetic 2-channel, 3-partition impulse response at block_length = 1024
        // (matching Wwise's exact `00 04 02 00` magic header!).
        let block_len = 1024usize;
        let ir_len = block_len * 2 + 300; // spans 3 partitions
        let mut ir_l = vec![0.0f32; ir_len];
        let mut ir_r = vec![0.0f32; ir_len];
        // Partition 0 impulses
        ir_l[0] = 0.8;
        ir_l[137] = -0.35;
        ir_r[12] = 0.65;
        ir_r[419] = 0.25;
        // Partition 1 impulse (at sample 1200)
        ir_l[1200] = 0.4;
        ir_r[1500] = -0.3;
        // Partition 2 impulse (at sample 2200)
        ir_l[2200] = 0.2;
        ir_r[2300] = 0.15;

        let ir = PreprocessedIr::from_time_domain(48_000, block_len, &[ir_l.clone(), ir_r.clone()], IrPrecision::BFloat16)
            .unwrap();
        let blob = ir.encode_wwise_bytes(IrPrecision::BFloat16);

        // Verify exact Wwise magic `00 04 02 00` and 48-byte header + 16-byte aligned payload!
        assert_eq!(&blob[0..4], &[0x00, 0x04, 0x02, 0x00]);
        let expected_data_off = 0x30 + ((3 * 2 + 15) & !15); // 0x40
        assert_eq!(expected_data_off, 0x40);
        assert_eq!(blob.len(), expected_data_off + 2 * (3 * 1024) * 4);

        // Parse back from raw `00 04 02 00` bytes
        let parsed = PreprocessedIr::parse(&blob).unwrap();
        assert_eq!(parsed.block_length, 1024);
        assert_eq!(parsed.fft_length, 2048);
        assert_eq!(parsed.sample_rate, 48_000);
        assert_eq!(parsed.precision, IrPrecision::BFloat16);
        assert_eq!(parsed.num_channels, 2);
        assert_eq!(parsed.num_partitions, 3);

        // Convolve a unit impulse streamed in 512-sample engine frames (latency = 1024 - 512 = 512).
        let mut conv = PartitionedConvolver::new(parsed);
        let total_frames = 8; // 8 * 512 = 4096 samples
        let mut out_l = Vec::new();
        let mut out_r = Vec::new();
        for f in 0..total_frames {
            let mut in_l = vec![0.0f32; 512];
            let mut in_r = vec![0.0f32; 512];
            if f == 0 {
                in_l[0] = 1.0;
                in_r[0] = 1.0;
            }
            let mut o = vec![vec![0.0f32; 512], vec![0.0f32; 512]];
            conv.process_channels(&[in_l, in_r], &mut o, 512);
            out_l.extend_from_slice(&o[0]);
            out_r.extend_from_slice(&o[1]);
        }

        // First 512 samples are the priming latency; samples 512..512+ir_len must match ir_l/ir_r!
        assert!(out_l[..512].iter().all(|&x| x == 0.0));
        let mut max_err_l = 0.0f32;
        let mut max_err_r = 0.0f32;
        for i in 0..ir_len {
            max_err_l = max_err_l.max((out_l[512 + i] - ir_l[i]).abs());
            max_err_r = max_err_r.max((out_r[512 + i] - ir_r[i]).abs());
        }
        assert!(
            max_err_l < 5e-3 && max_err_r < 5e-3,
            "bfloat16 partitioned convolution max err too large: L={max_err_l}, R={max_err_r}"
        );
        // Spot-check all multi-partition impulses landed at exact sample indices:
        assert!((out_l[512 + 0] - 0.8).abs() < 5e-3);
        assert!((out_l[512 + 137] - -0.35).abs() < 5e-3);
        assert!((out_l[512 + 1200] - 0.4).abs() < 5e-3);
        assert!((out_l[512 + 2200] - 0.2).abs() < 5e-3);
        assert!((out_r[512 + 12] - 0.65).abs() < 5e-3);
        assert!((out_r[512 + 1500] - -0.3).abs() < 5e-3);
        assert!((out_r[512 + 2300] - 0.15).abs() < 5e-3);
    }

    #[test]
    fn wwise_float32_00_04_00_00_and_truncated_tail_partitions_convolve_accurately() {
        // Test type 0 (Float32) with variable per-partition bin counts (tail frequency truncation)
        // and non-trivial multi-sample input signal compared against direct time-domain convolution.
        let block_len = 256usize;
        let ir_ch = {
            let mut h = vec![0.0f32; 400];
            for (i, s) in h.iter_mut().enumerate() {
                // Smooth low-frequency decaying IR so tail bin truncation stays accurate
                let t = i as f32 / 48_000.0;
                *s = (-t * 150.0).exp() * (2.0 * PI * 300.0 * t).cos() * 0.25;
            }
            h
        };
        let mut ir = PreprocessedIr::from_time_domain(48_000, block_len, &[ir_ch.clone()], IrPrecision::Float32)
            .unwrap();
        // Truncate partition 1 to 64 bins (out of 256) to verify variable `partition_bins[p]`!
        let p0_bins = ir.partition_bins[0] as usize;
        ir.partition_bins[1] = 64;
        ir.channels[0].truncate(p0_bins + 64);
        ir.channel_stride_bins = p0_bins + 64;

        let blob = ir.encode_wwise_bytes(IrPrecision::Float32);
        assert_eq!(&blob[0..4], &[0x00, 0x01, 0x00, 0x00]); // block=256 (0x0100), type=0 (Float32)
        let parsed = PreprocessedIr::parse(&blob).unwrap();
        assert_eq!(parsed.partition_bins, vec![256, 64]);
        assert_eq!(parsed.channel_stride_bins, 320);

        // Convolve a 512-sample low-freq sine burst (frames = 512 = 2 * block_len -> 0 latency!)
        let mut x = vec![0.0f32; 1024];
        for i in 0..512 {
            x[i] = (2.0 * PI * 220.0 * i as f32 / 48_000.0).sin();
        }
        let mut conv = PartitionedConvolver::new(parsed);
        let mut y = vec![vec![0.0f32; 512]];
        let mut out_all = Vec::new();
        for chunk in x.chunks(512) {
            conv.process_channels(&[chunk.to_vec()], &mut y, 512);
            out_all.extend_from_slice(&y[0]);
        }

        // Compare against direct convolution in partition 0 region (exact to f32 precision!)
        for n in 0..256 {
            let mut exact = 0.0f32;
            for k in 0..=n.min(ir_ch.len() - 1) {
                exact += x[n - k] * ir_ch[k];
            }
            assert!(
                (out_all[n] - exact).abs() < 1e-4,
                "sample {n}: conv={} exact={exact}",
                out_all[n]
            );
        }
    }

    #[test]
    fn riff_wave_ir_and_malformed_inputs() {
        // IEEE float32 stereo WAV IR: L = [0.5, 0.25], R = [0.75, -0.125]
        let mut wav = Vec::new();
        let data_bytes: Vec<u8> = [0.5f32, 0.75, 0.25, -0.125]
            .iter()
            .flat_map(|f| f.to_le_bytes())
            .collect();
        let riff_size = (4 + 8 + 16 + 8 + data_bytes.len()) as u32;
        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&riff_size.to_le_bytes());
        wav.extend_from_slice(b"WAVE");
        wav.extend_from_slice(b"fmt ");
        wav.extend_from_slice(&16u32.to_le_bytes());
        wav.extend_from_slice(&3u16.to_le_bytes()); // IEEE_FLOAT
        wav.extend_from_slice(&2u16.to_le_bytes()); // stereo
        wav.extend_from_slice(&48_000u32.to_le_bytes());
        wav.extend_from_slice(&(48_000u32 * 8).to_le_bytes());
        wav.extend_from_slice(&8u16.to_le_bytes());
        wav.extend_from_slice(&32u16.to_le_bytes());
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&(data_bytes.len() as u32).to_le_bytes());
        wav.extend_from_slice(&data_bytes);

        let ir = PreprocessedIr::parse(&wav).unwrap();
        assert_eq!(ir.sample_rate, 48_000);
        assert_eq!(ir.num_channels, 2);
        let mut conv = PartitionedConvolver::new(ir);
        let mut inp = vec![vec![0.0f32; 1024], vec![0.0f32; 1024]];
        inp[0][0] = 1.0;
        inp[1][0] = 1.0;
        let mut out = vec![vec![0.0f32; 1024], vec![0.0f32; 1024]];
        conv.process_channels(&inp, &mut out, 1024);
        assert!((out[0][0] - 0.5).abs() < 1e-4);
        assert!((out[0][1] - 0.25).abs() < 1e-4);
        assert!((out[1][0] - 0.75).abs() < 1e-4);
        assert!((out[1][1] - -0.125).abs() < 1e-4);

        // Reject garbage and truncated headers honestly
        assert!(PreprocessedIr::parse(b"NOPE").is_err());
        let mut trunc = [0u8; 48];
        trunc[0..4].copy_from_slice(&[0x00, 0x04, 0x02, 0x00]);
        assert!(PreprocessedIr::parse(&trunc).is_err());
    }
}
