//! Offline comparison of two WAV captures of the same audio (our WASAPI tap
//! vs Wwise's `AK::SoundEngine::StartOutputCapture`): they start at different
//! times, so the second is aligned onto the first before diffing.

use std::fmt;

#[derive(Debug, Clone)]
pub struct Wav {
    pub channels: u16,
    pub rate: u32,
    /// Bits per sample of the source data and whether it was float.
    pub bits: u16,
    pub float: bool,
    /// Interleaved samples, full scale = 1.0.
    pub samples: Vec<f32>,
}

impl Wav {
    pub fn frames(&self) -> usize {
        self.samples.len() / self.channels.max(1) as usize
    }
}

/// Parse a RIFF/WAVE file (PCM 8/16/24/32, IEEE float 32, EXTENSIBLE). The
/// data chunk size is clamped to the file (capture files may be unfinished).
pub fn read_wav(b: &[u8]) -> Result<Wav, String> {
    if b.len() < 12 || &b[0..4] != b"RIFF" || &b[8..12] != b"WAVE" {
        return Err("not a RIFF/WAVE file".into());
    }
    let u16_at = |o: usize| u16::from_le_bytes([b[o], b[o + 1]]);
    let u32_at = |o: usize| u32::from_le_bytes(b[o..o + 4].try_into().unwrap());
    let mut pos = 12;
    let mut fmt: Option<(u16, u16, u32, u16, bool)> = None;
    while pos + 8 <= b.len() {
        let id = &b[pos..pos + 4];
        let size = u32_at(pos + 4) as usize;
        let body = pos + 8;
        if id == b"fmt " {
            if body + 16 > b.len() {
                return Err("truncated fmt chunk".into());
            }
            let mut tag = u16_at(body);
            let ch = u16_at(body + 2);
            let rate = u32_at(body + 4);
            let bits = u16_at(body + 14);
            if tag == 0xFFFE && size >= 40 && body + 40 <= b.len() {
                tag = u16_at(body + 24); // SubFormat GUID's first 2 bytes = format tag
            }
            let float = match tag {
                1 => false,
                3 => true,
                t => return Err(format!("unsupported format tag {t:#x}")),
            };
            fmt = Some((tag, ch, rate, bits, float));
        } else if id == b"data" {
            let (_, ch, rate, bits, float) = fmt.ok_or("data before fmt")?;
            let end = (body + size).min(b.len());
            let d = &b[body..end];
            let samples: Vec<f32> = match (float, bits) {
                (true, 32) => d.chunks_exact(4).map(|c| f32::from_le_bytes(c.try_into().unwrap())).collect(),
                (false, 16) => d.chunks_exact(2).map(|c| i16::from_le_bytes([c[0], c[1]]) as f32 / 32768.0).collect(),
                (false, 24) => d.chunks_exact(3).map(|c| (i32::from_le_bytes([0, c[0], c[1], c[2]]) >> 8) as f32 / 8_388_608.0).collect(),
                (false, 32) => d.chunks_exact(4).map(|c| i32::from_le_bytes(c.try_into().unwrap()) as f32 / 2_147_483_648.0).collect(),
                (false, 8) => d.iter().map(|&v| (v as f32 - 128.0) / 128.0).collect(),
                _ => return Err(format!("unsupported sample format: {bits} bit float={float}")),
            };
            let n = samples.len() - samples.len() % ch.max(1) as usize;
            let mut samples = samples;
            samples.truncate(n);
            return Ok(Wav { channels: ch, rate, bits, float, samples });
        }
        pos = body + size + (size & 1);
    }
    Err("no data chunk".into())
}

#[derive(Debug, Clone, Default)]
pub struct Comparison {
    /// `b` frame `i` corresponds to `a` frame `i + offset`.
    pub offset: i64,
    pub overlap_frames: usize,
    /// Overlap frames where either signal is non-silent.
    pub active_frames: usize,
    pub exact_samples: usize,
    pub total_samples: usize,
    pub max_abs_diff: f32,
    pub rms_diff: f64,
    pub rms_signal: f64,
    /// Smallest quantization step of the two sources (0 for float/float).
    pub lsb: f32,
    /// float `a` vs 16-bit `b`: `a` converted the way Wwise's output capture
    /// converts (`(i16)(x * 32767.0f)`, truncating, saturating), compared
    /// exactly: (samples equal, max |difference| in LSB).
    pub wwise_i16: Option<(usize, i32)>,
    pub note: String,
}

impl Comparison {
    /// Identical samples, either directly or (float vs 16-bit capture) after
    /// Wwise's own float->int16 conversion.
    pub fn bit_exact(&self) -> bool {
        self.active_frames > 0 && (self.max_abs_diff == 0.0 || self.wwise_i16.is_some_and(|(eq, _)| eq == self.total_samples))
    }
    pub fn residual_dbfs(&self) -> f64 {
        if self.rms_diff == 0.0 {
            f64::NEG_INFINITY
        } else {
            20.0 * self.rms_diff.log10()
        }
    }
}

impl fmt::Display for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "offset {} frames, overlap {} frames ({} active), exact samples {}/{}, max|d| {:.3e}{}, residual {:.1} dBFS (signal {:.1} dBFS){}",
            self.offset,
            self.overlap_frames,
            self.active_frames,
            self.exact_samples,
            self.total_samples,
            self.max_abs_diff,
            if self.lsb > 0.0 { format!(" ({:.2} LSB)", self.max_abs_diff / self.lsb) } else { String::new() },
            self.residual_dbfs(),
            if self.rms_signal > 0.0 { 20.0 * self.rms_signal.log10() } else { f64::NEG_INFINITY },
            if self.note.is_empty() { String::new() } else { format!(" [{}]", self.note) }
        )?;
        if let Some((eq, max)) = self.wwise_i16 {
            write!(f, "; after Wwise float->int16 ((i16)(x*32767.0f)): {eq}/{} samples identical, max |d| {max} LSB", self.total_samples)?;
        }
        Ok(())
    }
}

const SILENT: f32 = 1.0e-5;

fn first_active(w: &Wav) -> Option<usize> {
    let ch = w.channels as usize;
    w.samples.chunks_exact(ch).position(|fr| fr.iter().any(|v| v.abs() > SILENT))
}

/// Sum of |a - b| over `len` frames starting at b-frame `start` for `offset`.
fn cost(a: &Wav, b: &Wav, offset: i64, start: usize, len: usize) -> Option<f64> {
    let ch = a.channels as usize;
    let mut s = 0.0f64;
    for i in start..start + len {
        let j = i as i64 + offset;
        if j < 0 || j as usize >= a.frames() || i >= b.frames() {
            return None;
        }
        for c in 0..ch {
            s += (a.samples[j as usize * ch + c] - b.samples[i * ch + c]).abs() as f64;
        }
    }
    Some(s)
}

/// Align `b` onto `a` (search ±`search` frames around the first-onset guess,
/// preferring an exact match) and diff the overlap.
pub fn compare(a: &Wav, b: &Wav, search: i64) -> Comparison {
    let mut r = Comparison::default();
    if a.channels != b.channels || a.rate != b.rate {
        r.note = format!("format mismatch: {} ch {} Hz vs {} ch {} Hz", a.channels, a.rate, b.channels, b.rate);
        return r;
    }
    let (Some(fa), Some(fb)) = (first_active(a), first_active(b)) else {
        r.note = "one of the files is entirely silent".into();
        return r;
    };
    let guess = fa as i64 - fb as i64;
    let win = 4096.min(b.frames() - fb);
    let mut best = (f64::INFINITY, guess);
    // Nearest offsets first so an exact match near the onset guess ends the search.
    for k in 0..=2 * search {
        let d = if k % 2 == 0 { k / 2 } else { -(k + 1) / 2 };
        if let Some(c) = cost(a, b, guess + d, fb, win) {
            if c < best.0 {
                best = (c, guess + d);
                if c == 0.0 {
                    break;
                }
            }
        }
    }
    r.offset = best.1;
    let ch = a.channels as usize;
    let start = 0i64.max(-r.offset) as usize;
    let end = (b.frames() as i64).min(a.frames() as i64 - r.offset).max(start as i64) as usize;
    r.overlap_frames = end - start;
    let lsb = |w: &Wav| if w.float { 0.0 } else { 1.0 / (1u64 << (w.bits - 1)) as f32 };
    r.lsb = lsb(a).max(lsb(b));
    let quantize = a.float && !b.float && b.bits == 16;
    let (mut q_eq, mut q_max) = (0usize, 0i32);
    let (mut se, mut ss) = (0.0f64, 0.0f64);
    for i in start..end {
        let j = (i as i64 + r.offset) as usize;
        let fa = &a.samples[j * ch..j * ch + ch];
        let fb = &b.samples[i * ch..i * ch + ch];
        if fa.iter().chain(fb).any(|v| v.abs() > SILENT) {
            r.active_frames += 1;
        }
        for c in 0..ch {
            let d = (fa[c] - fb[c]).abs();
            if d == 0.0 {
                r.exact_samples += 1;
            }
            r.max_abs_diff = r.max_abs_diff.max(d);
            se += (d as f64) * (d as f64);
            if quantize {
                let qa = (fa[c] * 32767.0f32) as i16 as i32; // `as` truncates toward zero and saturates
                let qb = (fb[c] * 32768.0f32) as i32; // exact: b was i16 / 32768
                let dq = (qa - qb).abs();
                if dq == 0 {
                    q_eq += 1;
                }
                q_max = q_max.max(dq);
            }
            ss += (fa[c] as f64) * (fa[c] as f64);
        }
    }
    r.total_samples = r.overlap_frames * ch;
    if quantize {
        r.wwise_i16 = Some((q_eq, q_max));
    }
    if r.total_samples > 0 {
        r.rms_diff = (se / r.total_samples as f64).sqrt();
        r.rms_signal = (ss / r.total_samples as f64).sqrt();
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wav_bytes(ch: u16, rate: u32, float: bool, samples: &[f32]) -> Vec<u8> {
        let bits: u16 = if float { 32 } else { 16 };
        let data: Vec<u8> = if float {
            samples.iter().flat_map(|v| v.to_le_bytes()).collect()
        } else {
            samples.iter().flat_map(|v| ((v * 32768.0).round().clamp(-32768.0, 32767.0) as i16).to_le_bytes()).collect()
        };
        let mut b = Vec::new();
        b.extend_from_slice(b"RIFF");
        b.extend_from_slice(&(36 + data.len() as u32).to_le_bytes());
        b.extend_from_slice(b"WAVEfmt ");
        b.extend_from_slice(&16u32.to_le_bytes());
        b.extend_from_slice(&(if float { 3u16 } else { 1 }).to_le_bytes());
        b.extend_from_slice(&ch.to_le_bytes());
        b.extend_from_slice(&rate.to_le_bytes());
        b.extend_from_slice(&(rate * ch as u32 * bits as u32 / 8).to_le_bytes());
        b.extend_from_slice(&(ch * bits / 8).to_le_bytes());
        b.extend_from_slice(&bits.to_le_bytes());
        b.extend_from_slice(b"data");
        b.extend_from_slice(&(data.len() as u32).to_le_bytes());
        b.extend_from_slice(&data);
        b
    }

    #[test]
    fn aligns_shifted_capture_exactly() {
        let sig: Vec<f32> = (0..20000).map(|i| if i < 3000 { 0.0 } else { ((i as f32) * 0.01).sin() * 0.5 }).collect();
        // a = 1536 frames of extra silence (prefill) + signal; b = signal from frame 1000.
        let mut a = vec![0.0f32; 1536 * 2];
        for v in &sig {
            a.push(*v);
            a.push(-*v);
        }
        let b: Vec<f32> = a[(1536 + 1000) * 2..].to_vec();
        let wa = read_wav(&wav_bytes(2, 48000, true, &a)).unwrap();
        let wb = read_wav(&wav_bytes(2, 48000, true, &b)).unwrap();
        let r = compare(&wa, &wb, 1024);
        assert_eq!(r.offset, 2536);
        assert!(r.bit_exact(), "{r}");
        // 16-bit capture made with round-to-nearest: within 1 LSB, not Wwise-exact.
        let wb16 = read_wav(&wav_bytes(2, 48000, false, &b)).unwrap();
        let r = compare(&wa, &wb16, 1024);
        assert_eq!(r.offset, 2536);
        assert!(r.max_abs_diff <= r.lsb, "{r}");
        assert!(!r.bit_exact());
        // 16-bit capture made like Wwise ((i16)(x * 32767.0f)): bit-exact.
        let wwise: Vec<f32> = b.iter().map(|v| ((v * 32767.0f32) as i16) as f32 / 32768.0).collect();
        let mut bytes = wav_bytes(2, 48000, false, &[]);
        let data: Vec<u8> = wwise.iter().flat_map(|v| ((v * 32768.0) as i16).to_le_bytes()).collect();
        let n = bytes.len();
        bytes[n - 4..].copy_from_slice(&(data.len() as u32).to_le_bytes());
        bytes.extend_from_slice(&data);
        let r = compare(&wa, &read_wav(&bytes).unwrap(), 1024);
        assert_eq!(r.offset, 2536);
        assert!(r.bit_exact(), "{r}");
    }
}
