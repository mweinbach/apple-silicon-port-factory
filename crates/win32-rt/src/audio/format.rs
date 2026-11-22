//! WAVEFORMATEX / WAVEFORMATEXTENSIBLE handling, sample conversion and the
//! channel-matrix policy used for `AUDCLNT_STREAMFLAGS_AUTOCONVERTPCM`.
//!
//! Downmix/upmix policy (only used when a client initializes with a channel
//! layout different from the endpoint mix format, which requires
//! AUTOCONVERTPCM; Wwise never does this because it renders in the mix
//! format's channel count — `FUN_141653fc0` / `FUN_141654e50`):
//! * a speaker present in both layouts is copied at unity gain;
//! * a mono source (1 channel, any mask) goes to FL+FR (or FC if that is the
//!   only output) at unity gain, like the Windows audio engine;
//! * missing speakers fold to their nearest neighbours: FC -> FL+FR at -3 dB;
//!   back <-> side (same side) at unity; side/back -> front same side at
//!   -3 dB; FLC/FRC -> FL/FR; BC -> BL+BR (or SL+SR) at -3 dB, else FL+FR at
//!   -6 dB; top speakers -> the ear-level speaker below them at -3 dB
//!   (TFL/TBL -> FL/BL..., TC -> FC); LFE is dropped unless the target has
//!   one (no bass management);
//! * no normalization: the result can exceed 0 dBFS (CoreAudio clips).

use super::com::{GUID, KSDATAFORMAT_SUBTYPE_IEEE_FLOAT, KSDATAFORMAT_SUBTYPE_PCM};

pub const WAVE_FORMAT_PCM: u16 = 1;
pub const WAVE_FORMAT_IEEE_FLOAT: u16 = 3;
pub const WAVE_FORMAT_EXTENSIBLE: u16 = 0xFFFE;

// SPEAKER_* bits (ksmedia.h). CoreAudio's AudioChannelBitmap uses the same bit order.
pub const SPEAKER_FRONT_LEFT: u32 = 0x1;
pub const SPEAKER_FRONT_RIGHT: u32 = 0x2;
pub const SPEAKER_FRONT_CENTER: u32 = 0x4;
pub const SPEAKER_LOW_FREQUENCY: u32 = 0x8;
pub const SPEAKER_BACK_LEFT: u32 = 0x10;
pub const SPEAKER_BACK_RIGHT: u32 = 0x20;
pub const SPEAKER_FRONT_LEFT_OF_CENTER: u32 = 0x40;
pub const SPEAKER_FRONT_RIGHT_OF_CENTER: u32 = 0x80;
pub const SPEAKER_BACK_CENTER: u32 = 0x100;
pub const SPEAKER_SIDE_LEFT: u32 = 0x200;
pub const SPEAKER_SIDE_RIGHT: u32 = 0x400;
pub const SPEAKER_TOP_CENTER: u32 = 0x800;
pub const SPEAKER_TOP_FRONT_LEFT: u32 = 0x1000;
pub const SPEAKER_TOP_FRONT_CENTER: u32 = 0x2000;
pub const SPEAKER_TOP_FRONT_RIGHT: u32 = 0x4000;
pub const SPEAKER_TOP_BACK_LEFT: u32 = 0x8000;
pub const SPEAKER_TOP_BACK_CENTER: u32 = 0x10000;
pub const SPEAKER_TOP_BACK_RIGHT: u32 = 0x20000;

/// Default `dwChannelMask` Windows uses for a mix format with `n` channels.
pub fn default_mask(n: u16) -> u32 {
    match n {
        1 => SPEAKER_FRONT_CENTER,
        2 => 0x3,
        3 => 0x7,
        4 => 0x33,           // KSAUDIO_SPEAKER_QUAD
        5 => 0x37,           // 5.0 (back)
        6 => 0x3F,           // KSAUDIO_SPEAKER_5POINT1
        7 => 0x13F,          // 6.1
        8 => 0x63F,          // KSAUDIO_SPEAKER_7POINT1_SURROUND
        12 => 0x2D63F,       // 7.1.4
        n if n < 32 => (1u32 << n) - 1,
        _ => 0,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampleType {
    F32,
    I16,
    I24,     // 3-byte packed
    I32,     // 32-bit container (valid bits may be 24)
    U8,
}

/// Parsed, validated stream format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WaveFormat {
    pub rate: u32,
    pub channels: u16,
    pub bits: u16,
    pub valid_bits: u16,
    pub block_align: u16,
    pub mask: u32,
    pub sample: SampleType,
    pub extensible: bool,
}

/// `WAVEFORMATEX` (18 bytes, packed: cbSize ends at 18).
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default, Debug)]
pub struct WAVEFORMATEX {
    pub wFormatTag: u16,
    pub nChannels: u16,
    pub nSamplesPerSec: u32,
    pub nAvgBytesPerSec: u32,
    pub nBlockAlign: u16,
    pub wBitsPerSample: u16,
    pub cbSize: u16,
}

/// `WAVEFORMATEXTENSIBLE` (40 bytes).
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug)]
pub struct WAVEFORMATEXTENSIBLE {
    pub Format: WAVEFORMATEX,
    pub wValidBitsPerSample: u16,
    pub dwChannelMask: u32,
    pub SubFormat: GUID,
}

const _: () = assert!(std::mem::size_of::<WAVEFORMATEX>() == 18);
const _: () = assert!(std::mem::size_of::<WAVEFORMATEXTENSIBLE>() == 40);

#[derive(Debug, PartialEq, Eq)]
pub enum FormatError {
    Null,
    Unsupported(String),
}

impl WaveFormat {
    /// Parse a client-supplied `WAVEFORMATEX*`.
    pub unsafe fn from_ptr(p: *const WAVEFORMATEX) -> Result<WaveFormat, FormatError> {
        if p.is_null() {
            return Err(FormatError::Null);
        }
        let wf = std::ptr::read_unaligned(p);
        let (tag, valid, mask) = if wf.wFormatTag == WAVE_FORMAT_EXTENSIBLE {
            if wf.cbSize < 22 {
                return Err(FormatError::Unsupported(format!("EXTENSIBLE with cbSize {}", { wf.cbSize })));
            }
            let x = std::ptr::read_unaligned(p as *const WAVEFORMATEXTENSIBLE);
            let sub = { x.SubFormat };
            let tag = if sub == KSDATAFORMAT_SUBTYPE_IEEE_FLOAT {
                WAVE_FORMAT_IEEE_FLOAT
            } else if sub == KSDATAFORMAT_SUBTYPE_PCM {
                WAVE_FORMAT_PCM
            } else {
                return Err(FormatError::Unsupported(format!("SubFormat {:?}", { x.SubFormat })));
            };
            let valid = if x.wValidBitsPerSample == 0 { wf.wBitsPerSample } else { x.wValidBitsPerSample };
            (tag, valid, x.dwChannelMask)
        } else {
            (wf.wFormatTag, wf.wBitsPerSample, default_mask(wf.nChannels))
        };
        let bits = wf.wBitsPerSample;
        let sample = match (tag, bits) {
            (WAVE_FORMAT_IEEE_FLOAT, 32) => SampleType::F32,
            (WAVE_FORMAT_PCM, 8) => SampleType::U8,
            (WAVE_FORMAT_PCM, 16) => SampleType::I16,
            (WAVE_FORMAT_PCM, 24) => SampleType::I24,
            (WAVE_FORMAT_PCM, 32) => SampleType::I32,
            _ => return Err(FormatError::Unsupported(format!("tag {tag:#x} bits {bits}"))),
        };
        let ch = wf.nChannels;
        if ch == 0 || ch > 32 || wf.nSamplesPerSec < 1000 || wf.nSamplesPerSec > 768_000 {
            return Err(FormatError::Unsupported(format!("{} ch @ {} Hz", ch, { wf.nSamplesPerSec })));
        }
        if wf.nBlockAlign as u32 != ch as u32 * bits as u32 / 8 || valid > bits {
            return Err(FormatError::Unsupported(format!("blockAlign {} / valid bits {}", { wf.nBlockAlign }, valid)));
        }
        Ok(WaveFormat {
            rate: wf.nSamplesPerSec,
            channels: ch,
            bits,
            valid_bits: valid,
            block_align: wf.nBlockAlign,
            mask,
            sample,
            extensible: wf.wFormatTag == WAVE_FORMAT_EXTENSIBLE,
        })
    }

    /// The float32 WAVEFORMATEXTENSIBLE mix format Windows reports.
    pub fn float_mix(rate: u32, channels: u16, mask: u32) -> WaveFormat {
        WaveFormat {
            rate,
            channels,
            bits: 32,
            valid_bits: 32,
            block_align: channels * 4,
            mask,
            sample: SampleType::F32,
            extensible: true,
        }
    }

    pub fn to_extensible(&self) -> WAVEFORMATEXTENSIBLE {
        let sub = if self.sample == SampleType::F32 { KSDATAFORMAT_SUBTYPE_IEEE_FLOAT } else { KSDATAFORMAT_SUBTYPE_PCM };
        WAVEFORMATEXTENSIBLE {
            Format: WAVEFORMATEX {
                wFormatTag: WAVE_FORMAT_EXTENSIBLE,
                nChannels: self.channels,
                nSamplesPerSec: self.rate,
                nAvgBytesPerSec: self.rate * self.block_align as u32,
                nBlockAlign: self.block_align,
                wBitsPerSample: self.bits,
                cbSize: 22,
            },
            wValidBitsPerSample: self.valid_bits,
            dwChannelMask: self.mask,
            SubFormat: sub,
        }
    }

    /// Bytes of the extensible struct (for blobs / WAV headers).
    pub fn extensible_bytes(&self) -> [u8; 40] {
        unsafe { std::mem::transmute(self.to_extensible()) }
    }
}

/// Convert `frames` interleaved frames of `fmt` from `src` to f32 (same channel count).
pub fn to_f32(fmt: &WaveFormat, src: &[u8], frames: usize, out: &mut Vec<f32>) {
    let n = frames * fmt.channels as usize;
    out.clear();
    out.reserve(n);
    match fmt.sample {
        SampleType::F32 => {
            out.extend(src[..n * 4].chunks_exact(4).map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]])));
        }
        SampleType::I16 => {
            out.extend(src[..n * 2].chunks_exact(2).map(|b| i16::from_le_bytes([b[0], b[1]]) as f32 / 32768.0));
        }
        SampleType::I24 => {
            out.extend(src[..n * 3].chunks_exact(3).map(|b| (i32::from_le_bytes([0, b[0], b[1], b[2]]) >> 8) as f32 / 8_388_608.0));
        }
        SampleType::I32 => {
            out.extend(src[..n * 4].chunks_exact(4).map(|b| i32::from_le_bytes([b[0], b[1], b[2], b[3]]) as f32 / 2_147_483_648.0));
        }
        SampleType::U8 => {
            out.extend(src[..n].iter().map(|&b| (b as f32 - 128.0) / 128.0));
        }
    }
}

const M3DB: f32 = std::f32::consts::FRAC_1_SQRT_2;

type Alt = &'static [(u32, f32)];

/// Ordered fallback alternatives for a speaker the output layout lacks. The
/// first alternative whose targets are all present wins; otherwise the first
/// alternative is resolved recursively (bounded depth).
fn fold(bit: u32) -> &'static [Alt] {
    const FL: u32 = SPEAKER_FRONT_LEFT;
    const FR: u32 = SPEAKER_FRONT_RIGHT;
    const FC: u32 = SPEAKER_FRONT_CENTER;
    const BL: u32 = SPEAKER_BACK_LEFT;
    const BR: u32 = SPEAKER_BACK_RIGHT;
    const SL: u32 = SPEAKER_SIDE_LEFT;
    const SR: u32 = SPEAKER_SIDE_RIGHT;
    match bit {
        FC => &[&[(FL, M3DB), (FR, M3DB)]],
        FL | FR => &[&[(FC, M3DB)]],
        BL => &[&[(SL, 1.0)], &[(FL, M3DB)]],
        BR => &[&[(SR, 1.0)], &[(FR, M3DB)]],
        SL => &[&[(BL, 1.0)], &[(FL, M3DB)]],
        SR => &[&[(BR, 1.0)], &[(FR, M3DB)]],
        SPEAKER_FRONT_LEFT_OF_CENTER => &[&[(FL, 1.0)]],
        SPEAKER_FRONT_RIGHT_OF_CENTER => &[&[(FR, 1.0)]],
        SPEAKER_BACK_CENTER => &[&[(BL, M3DB), (BR, M3DB)], &[(SL, M3DB), (SR, M3DB)], &[(FL, 0.5), (FR, 0.5)]],
        SPEAKER_TOP_CENTER | SPEAKER_TOP_FRONT_CENTER => &[&[(FC, M3DB)]],
        SPEAKER_TOP_FRONT_LEFT => &[&[(FL, M3DB)]],
        SPEAKER_TOP_FRONT_RIGHT => &[&[(FR, M3DB)]],
        SPEAKER_TOP_BACK_LEFT => &[&[(BL, M3DB)]],
        SPEAKER_TOP_BACK_RIGHT => &[&[(BR, M3DB)]],
        SPEAKER_TOP_BACK_CENTER => &[&[(SPEAKER_BACK_CENTER, M3DB)]],
        _ => &[], // LFE and unknown bits: dropped
    }
}

/// Channel bit for each interleaved position of a layout (`mask` lists the
/// first channels; channels beyond the mask's popcount get bit 0 = unmapped).
fn positions(channels: u16, mask: u32) -> Vec<u32> {
    let mut v = Vec::with_capacity(channels as usize);
    let mut m = mask;
    for _ in 0..channels {
        if m == 0 {
            v.push(0);
        } else {
            let b = m & m.wrapping_neg();
            v.push(b);
            m &= !b;
        }
    }
    v
}

fn route(bit: u32, gain: f32, out_bits: &[u32], row: &mut [f32], depth: u32) {
    if let Some(j) = out_bits.iter().position(|&b| b == bit && b != 0) {
        row[j] += gain;
        return;
    }
    if depth >= 3 {
        return;
    }
    let alts = fold(bit);
    let present = |a: &Alt| a.iter().all(|(t, _)| out_bits.contains(t));
    if let Some(a) = alts.iter().find(|a| present(a)).or(alts.first()) {
        for &(t, g) in a.iter() {
            route(t, gain * g, out_bits, row, depth + 1);
        }
    }
}

/// Row-major `[in][out]` gain matrix converting `in_ch/in_mask` to `out_ch/out_mask`.
pub fn channel_matrix(in_ch: u16, in_mask: u32, out_ch: u16, out_mask: u32) -> Vec<f32> {
    let (ni, no) = (in_ch as usize, out_ch as usize);
    let mut m = vec![0.0f32; ni * no];
    let ib = positions(in_ch, in_mask);
    let ob = positions(out_ch, out_mask);
    if ni == 1 {
        // Mono: duplicate to front L/R at unity (Windows engine behaviour).
        let l = ob.iter().position(|&b| b == SPEAKER_FRONT_LEFT);
        let r = ob.iter().position(|&b| b == SPEAKER_FRONT_RIGHT);
        match (l, r) {
            (Some(l), Some(r)) => {
                m[l] = 1.0;
                m[r] = 1.0;
            }
            _ => m[ob.iter().position(|&b| b == SPEAKER_FRONT_CENTER).unwrap_or(0)] = 1.0,
        }
        return m;
    }
    for (i, &bit) in ib.iter().enumerate() {
        let row = &mut m[i * no..(i + 1) * no];
        if bit == 0 {
            // Unmapped input channel: pass through by index if the output has it.
            if i < no && ob[i] == 0 {
                row[i] = 1.0;
            }
            continue;
        }
        if no == 1 {
            // Mono output: everything except LFE at -3 dB (L/R) / unity (C).
            row[0] = match bit {
                SPEAKER_LOW_FREQUENCY => 0.0,
                SPEAKER_FRONT_CENTER => 1.0,
                _ => M3DB,
            };
            continue;
        }
        route(bit, 1.0, &ob, row, 0);
    }
    m
}

/// Apply `matrix` (from [`channel_matrix`]) to interleaved f32 frames.
pub fn apply_matrix(matrix: &[f32], in_ch: usize, out_ch: usize, src: &[f32], out: &mut Vec<f32>) {
    let frames = src.len() / in_ch;
    out.clear();
    out.resize(frames * out_ch, 0.0);
    for f in 0..frames {
        let s = &src[f * in_ch..(f + 1) * in_ch];
        let d = &mut out[f * out_ch..(f + 1) * out_ch];
        for (i, &x) in s.iter().enumerate() {
            if x == 0.0 {
                continue;
            }
            let row = &matrix[i * out_ch..(i + 1) * out_ch];
            for (o, &g) in row.iter().enumerate() {
                d[o] += x * g;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mix_format_roundtrip() {
        let f = WaveFormat::float_mix(48000, 2, 3);
        let x = f.to_extensible();
        let g = unsafe { WaveFormat::from_ptr(&x.Format) }.unwrap();
        assert_eq!(f, g);
        assert_eq!({ x.Format.nAvgBytesPerSec }, 48000 * 8);
        // The tail of IEEE_FLOAT's GUID is the constant Wwise writes itself
        // (0x719b3800aa000080 in FUN_141653fc0).
        let b = f.extensible_bytes();
        assert_eq!(u64::from_le_bytes(b[32..40].try_into().unwrap()), 0x719b3800aa000080);
    }

    #[test]
    fn rejects_garbage() {
        let mut x = WaveFormat::float_mix(48000, 2, 3).to_extensible();
        x.Format.nBlockAlign = 7;
        assert!(unsafe { WaveFormat::from_ptr(&x.Format) }.is_err());
        let p = WAVEFORMATEX { wFormatTag: 2, nChannels: 2, nSamplesPerSec: 44100, nAvgBytesPerSec: 0, nBlockAlign: 4, wBitsPerSample: 16, cbSize: 0 };
        assert!(unsafe { WaveFormat::from_ptr(&p) }.is_err());
        let p = WAVEFORMATEX { wFormatTag: 1, nChannels: 2, nSamplesPerSec: 44100, nAvgBytesPerSec: 176400, nBlockAlign: 4, wBitsPerSample: 16, cbSize: 0 };
        let f = unsafe { WaveFormat::from_ptr(&p) }.unwrap();
        assert_eq!(f.sample, SampleType::I16);
        assert_eq!(f.mask, 3);
    }

    #[test]
    fn sample_conversion() {
        let f = WaveFormat { rate: 48000, channels: 1, bits: 16, valid_bits: 16, block_align: 2, mask: 4, sample: SampleType::I16, extensible: false };
        let src: Vec<u8> = [i16::MIN, 0, 16384].iter().flat_map(|v| v.to_le_bytes()).collect();
        let mut out = Vec::new();
        to_f32(&f, &src, 3, &mut out);
        assert_eq!(out, vec![-1.0, 0.0, 0.5]);
        let f24 = WaveFormat { bits: 24, valid_bits: 24, block_align: 3, sample: SampleType::I24, ..f };
        to_f32(&f24, &[0x00, 0x00, 0x80, 0xff, 0xff, 0x7f], 2, &mut out);
        assert_eq!(out[0], -1.0);
        assert!((out[1] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn downmix_51_to_stereo() {
        let m = channel_matrix(6, 0x3F, 2, 0x3);
        // rows: FL FR FC LFE BL BR
        assert_eq!(&m[0..2], &[1.0, 0.0]);
        assert_eq!(&m[2..4], &[0.0, 1.0]);
        assert!((m[4] - M3DB).abs() < 1e-6 && (m[5] - M3DB).abs() < 1e-6);
        assert_eq!(&m[6..8], &[0.0, 0.0]); // LFE dropped
        assert!((m[8] - M3DB).abs() < 1e-6 && m[9] == 0.0); // BL -> FL -3 dB
        assert!(m[10] == 0.0 && (m[11] - M3DB).abs() < 1e-6);
    }

    #[test]
    fn downmix_714_to_71_and_identity() {
        let m = channel_matrix(12, 0x2D63F, 8, 0x63F);
        let id = channel_matrix(8, 0x63F, 8, 0x63F);
        for i in 0..8 {
            for o in 0..8 {
                assert_eq!(id[i * 8 + o], if i == o { 1.0 } else { 0.0 });
                assert_eq!(m[i * 8 + o], if i == o { 1.0 } else { 0.0 });
            }
        }
        // 7.1.4 positions 8..12: TFL TFR TBL TBR -> FL FR BL BR at -3 dB.
        let expect = [(8, 0), (9, 1), (10, 4), (11, 5)];
        for (i, o) in expect {
            assert!((m[i * 8 + o] - M3DB).abs() < 1e-6, "row {i}");
        }
        let mut out = Vec::new();
        apply_matrix(&m, 12, 8, &[1.0; 12], &mut out);
        assert!((out[0] - (1.0 + M3DB)).abs() < 1e-6);
        assert_eq!(out[3], 1.0);
    }

    #[test]
    fn mono_upmix() {
        let m = channel_matrix(1, 4, 2, 3);
        assert_eq!(m, vec![1.0, 1.0]);
    }
}
