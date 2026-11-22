//! AK's voice resampler / pitch shifter, reimplemented from AlanWake2.exe (Wwise 2022.1.4) [V].
//!
//! State layout of the original object (offsets into the AK resampler):
//!
//! | off | field | here |
//! |---|---|---|
//! | +0x20 | ptr to per-channel "previous sample" (last consumed input sample) | `prev` |
//! | +0x30 | 16.16 fixed-point read index, **initialised to 0x10000** (`FUN_141643100`) | `idx` |
//! | +0x34 / +0x38 | current / target step (16.16) | `step` / `target` |
//! | +0x3c | pitch-ramp counter (0..1024) | `counter` |
//! | +0x40 | ramp increment per output sample = engine rate / output rate (`FUN_141643200`) | `ramp_inc` |
//! | +0x60 | mode: 0 bypass, 1 fixed, 2 interpolating | `mode` |
//! | +0x64 | `(float)srcRate / (float)outRate` (`FUN_141643200`) | `ratio` |
//! | +0x68 | last pitch in cents | `last_cents` |
//! | +0x6e | "first SetPitch" flag | `first` |
//!
//! Dispatch table `0x143931f90[mode*6 + type]` (types 3/4/5 = native float 1/2/N channels):
//! bypass `0x141668640`, fixed `0x141663920`/`0x141663c60`/`0x141664140` (AVX2+FMA variants
//! `0x1416670e0`/`0x141667550`/`0x141667b90`), interpolating `0x141665040`/`0x141665280`/`0x141668bf0`.
//! The executor `FUN_141643130` switches interpolating -> fixed (cur = target) once the counter
//! exceeds 0x3ff.
//!
//! Index semantics: output sample at index `idx` interpolates between input `i-1` and `i` with
//! `i = idx >> 16`, where input `-1` is `prev`. With `idx` starting at 0x10000 the first output is
//! exactly `in[0]` (no delay). At the end of a call the consumed input count is
//! `min(idx >> 16, n_in)`, `prev = in[consumed-1]`, `idx -= consumed << 16`.
//!
//! The 2ch/Nch variants are assumed to apply the identical per-channel arithmetic [A: only the
//! 1ch fixed and interpolating routines have been read in full].

use super::math::RATE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Bypass,
    Fixed,
    Interpolating,
}

#[derive(Debug, Clone)]
pub struct Resampler {
    pub idx: u32,
    pub step: u32,
    pub target: u32,
    pub counter: u32,
    pub ramp_inc: u32,
    pub mode: Mode,
    pub ratio: f32,
    pub last_cents: f32,
    pub first: bool,
    pub prev: Vec<f32>,
    /// Use the AVX2 (`vfmadd231ps`) arithmetic of the fixed-step routines instead of the SSE one.
    pub fma: bool,
}

/// `step = (u32)(i64)(powf(2, cents/1200) * ratio * 65536 + 0.5)`, 0 -> 1, >= 2^31 -> 0x7fffffff
/// (`FUN_141643700` / `FUN_1416438a0`) [V]. `powf` is the CRT's; ours may differ by an ulp,
/// which only matters if the product lands within an ulp of a .5 boundary.
pub fn pitch_to_step(cents: f32, ratio: f32) -> u32 {
    let f = 2f32.powf(cents * 0.000_833_333_35) * ratio * 65536.0 + 0.5;
    if f < 2.147_483_6e9 {
        let s = (f as i64) as u32;
        if s == 0 { 1 } else { s }
    } else {
        0x7fff_ffff
    }
}

#[inline(always)]
fn lerp_sse(a: f32, b: f32, frac: u32) -> f32 {
    (b - a) * (frac as f32) * 1.525_878_9e-5 + a
}

#[inline(always)]
fn lerp_fma(a: f32, b: f32, frac: u32) -> f32 {
    (b - a).mul_add(frac as f32 * 1.525_878_9e-5, a)
}

impl Resampler {
    /// Constructor (`FUN_141643100`) + `FUN_141643200` (rates, channel count).
    pub fn new(channels: usize, src_rate: u32, fma: bool) -> Self {
        Resampler {
            idx: 0x10000,
            step: 0,
            target: 0,
            counter: 0,
            ramp_inc: RATE / RATE,
            mode: Mode::Bypass,
            ratio: src_rate as f32 / RATE as f32,
            last_cents: 0.0,
            first: true,
            prev: vec![0.0; channels],
            fma,
        }
    }

    pub fn reset(&mut self) {
        self.idx = 0x10000;
        self.prev.fill(0.0);
    }

    /// `FUN_141643700(res, cents, interpolate)` [V]. The ramp only applies when `interpolate`
    /// is set and the pitch actually changes after the first call.
    pub fn set_pitch(&mut self, cents: f32, interpolate: bool) {
        let (mut cur, mut tgt);
        if self.first {
            let s = pitch_to_step(cents, self.ratio);
            self.last_cents = cents;
            self.step = s;
            self.target = s;
            self.counter = 0x400;
            self.first = false;
            cur = s;
            tgt = s;
        } else {
            cur = self.step;
            tgt = self.target;
        }
        if cents != self.last_cents {
            if self.mode == Mode::Interpolating {
                let d = (tgt.wrapping_sub(cur) as i32).wrapping_mul(self.counter as i32);
                // (d + (d >> 31 & 0x3ff)) >> 10 == d / 1024 rounded toward zero
                cur = ((d.wrapping_add((d >> 31) & 0x3ff)) >> 10) as u32;
                cur = cur.wrapping_add(self.step);
                cur = cur.clamp(1, 0x7fff_ffff);
                self.step = cur;
            }
            self.counter = 0;
            tgt = pitch_to_step(cents, self.ratio);
            self.last_cents = cents;
            self.target = tgt;
            if !interpolate {
                self.step = tgt;
                cur = tgt;
            }
        }
        self.mode = if cur != tgt {
            Mode::Interpolating
        } else if tgt != 0x10000 {
            Mode::Fixed
        } else {
            Mode::Bypass
        };
    }

    /// One call of the dispatched routine plus the executor's mode switch. `input[c]` holds the
    /// remaining input frames of channel c (all equal length), `out[c][start..]` receives output.
    /// Returns (input frames consumed, output frames produced).
    pub fn process(&mut self, input: &[&[f32]], out: &mut [Vec<f32>], start: usize, max_out: usize) -> (usize, usize) {
        let r = match self.mode {
            Mode::Bypass => self.bypass(input, out, start, max_out),
            Mode::Fixed => self.fixed(input, out, start, max_out),
            Mode::Interpolating => self.interpolating(input, out, start, max_out),
        };
        if self.mode == Mode::Interpolating && self.counter > 0x3ff {
            self.step = self.target;
            self.mode = Mode::Fixed;
        }
        r
    }

    /// `FUN_141668640`: memcpy, prev = last copied sample, idx = 0x10000.
    fn bypass(&mut self, input: &[&[f32]], out: &mut [Vec<f32>], start: usize, max_out: usize) -> (usize, usize) {
        let n_in = input.first().map_or(0, |c| c.len());
        let n = n_in.min(max_out);
        for (c, inp) in input.iter().enumerate() {
            out[c][start..start + n].copy_from_slice(&inp[..n]);
            if n > 0 {
                self.prev[c] = inp[n - 1];
            }
        }
        self.idx = 0x10000;
        (n, n)
    }

    /// `FUN_141663920` (SSE) / `FUN_1416670e0` (AVX2, FMA) for float input.
    fn fixed(&mut self, input: &[&[f32]], out: &mut [Vec<f32>], start: usize, max_out: usize) -> (usize, usize) {
        let n_in = input.first().map_or(0, |c| c.len()) as u64;
        let step = self.step as u64;
        let idx0 = self.idx as u64;
        // phase 1: idx < 0x10000 (interpolate from prev to in[0])
        let n1 = if idx0 < 0x10000 { ((0x10000 - idx0) + step - 1) / step } else { 0 };
        let n1 = n1.min(max_out as u64) as usize;
        // phase 2: idx < n_in << 16
        let idx1 = idx0 + n1 as u64 * step;
        let lim = n_in << 16;
        let n2 = if idx1 < lim { (lim - idx1 + step - 1) / step } else { 0 };
        let n2 = n2.min((max_out - n1) as u64) as usize;
        if n_in == 0 {
            return (0, 0);
        }
        let lerp = if self.fma { lerp_fma } else { lerp_sse };
        for (c, inp) in input.iter().enumerate() {
            let o = &mut out[c][start..start + n1 + n2];
            let prev = self.prev[c];
            let mut idx = idx0 as u32;
            for s in o[..n1].iter_mut() {
                *s = lerp_sse(prev, inp[0], idx & 0xffff); // phase 1 is scalar in both variants [A]
                idx = idx.wrapping_add(self.step);
            }
            for s in o[n1..].iter_mut() {
                let i = (idx >> 16) as usize;
                *s = lerp(inp[i - 1], inp[i], idx & 0xffff);
                idx = idx.wrapping_add(self.step);
            }
        }
        let idx_end = idx0 + (n1 + n2) as u64 * step;
        let consumed = (idx_end >> 16).min(n_in) as usize;
        if consumed > 0 {
            for (c, inp) in input.iter().enumerate() {
                self.prev[c] = inp[consumed - 1];
            }
        }
        self.idx = (idx_end - ((consumed as u64) << 16)) as u32;
        (consumed, n1 + n2)
    }

    /// `FUN_141665040`: step ramps linearly from `step` to `target` over 1024 ramp units.
    fn interpolating(&mut self, input: &[&[f32]], out: &mut [Vec<f32>], start: usize, max_out: usize) -> (usize, usize) {
        let n_in = input.first().map_or(0, |c| c.len()) as u32;
        let channels = input.len();
        let d = self.target.wrapping_sub(self.step) as i32;
        let base = self.step.wrapping_mul(0x400);
        let inc = self.ramp_inc.max(1);
        let mut produced = 0usize;
        let mut idx = self.idx;
        let mut counter = self.counter;
        let mut acc = (d.wrapping_mul(counter as i32) as u32).wrapping_add(base);
        let mut emit = |idx: u32, produced: usize, phase1: bool| {
            let frac = idx & 0xffff;
            for c in 0..channels {
                let v = if phase1 {
                    lerp_sse(self.prev[c], input[c][0], frac)
                } else {
                    let i = (idx >> 16) as usize;
                    lerp_sse(input[c][i - 1], input[c][i], frac)
                };
                out[c][start + produced] = v;
            }
        };
        if n_in > 0 {
            // phase 1
            if idx >> 16 == 0 {
                let mut n = ((0x400 - counter) / inc).min(max_out as u32);
                while n > 0 && idx >> 16 == 0 {
                    n -= 1;
                    acc = acc.wrapping_add((d as u32).wrapping_mul(inc));
                    counter += inc;
                    emit(idx, produced, true);
                    produced += 1;
                    idx = idx.wrapping_add(acc >> 10);
                }
            }
            // phase 2
            let mut n = ((0x400 - counter.min(0x400)) / inc).min((max_out - produced) as u32);
            if idx >> 16 != 0 && idx >> 16 <= n_in - 1 {
                acc = (d.wrapping_mul(counter as i32) as u32).wrapping_add(base);
                while n > 0 && idx >> 16 <= n_in - 1 {
                    n -= 1;
                    acc = acc.wrapping_add((d as u32).wrapping_mul(inc));
                    counter += inc;
                    emit(idx, produced, false);
                    produced += 1;
                    idx = idx.wrapping_add(acc >> 10);
                }
            }
        }
        self.counter = counter;
        let consumed = (idx >> 16).min(n_in);
        if consumed > 0 {
            for c in 0..channels {
                self.prev[c] = input[c][consumed as usize - 1];
            }
        }
        self.idx = idx.wrapping_sub(consumed << 16);
        (consumed as usize, produced)
    }

    /// Output frames a fixed-step resampler produces from `n_in` frames when fed to the end
    /// (fresh state): outputs while `0x10000 + k*step < n_in << 16`.
    pub fn output_len(n_in: u64, step: u32) -> u64 {
        if n_in == 0 {
            return 0;
        }
        if step == 0x10000 {
            return n_in;
        }
        let lim = n_in << 16;
        (lim - 0x10000 + step as u64 - 1) / step as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps_for_common_rates() {
        assert_eq!(pitch_to_step(0.0, 44100.0 / 48000.0), 60211);
        assert_eq!(pitch_to_step(0.0, 32000.0 / 48000.0), 43691);
        assert_eq!(pitch_to_step(0.0, 22050.0 / 48000.0), 30106);
        assert_eq!(pitch_to_step(0.0, 16000.0 / 48000.0), 21845); // 0.33333334f * 65536 + 0.5 = 21845.83
        assert_eq!(pitch_to_step(0.0, 1.0), 0x10000);
        assert_eq!(pitch_to_step(1200.0, 1.0), 0x20000);
    }

    #[test]
    fn first_output_is_first_input_and_blocks_are_seamless() {
        let src: Vec<f32> = (0..1000).map(|i| ((i as f32) * 0.37).sin()).collect();
        let mut whole = Resampler::new(1, 44100, false);
        whole.set_pitch(0.0, false);
        assert_eq!(whole.mode, Mode::Fixed);
        let mut o1 = vec![vec![0.0; 2000]];
        let (c1, p1) = whole.process(&[&src], &mut o1, 0, 2000);
        assert_eq!(c1, 1000);
        assert_eq!(o1[0][0], src[0]);
        assert_eq!(p1 as u64, Resampler::output_len(1000, whole.step));
        // same input in uneven blocks, uneven output limits
        let mut r = Resampler::new(1, 44100, false);
        r.set_pitch(0.0, false);
        let mut o2 = vec![vec![0.0; 2000]];
        let (mut pos, mut produced) = (0usize, 0usize);
        let blocks = [7usize, 100, 1, 333, 559];
        let mut bi = 0;
        while pos < src.len() {
            let end = (pos + blocks[bi % blocks.len()]).min(src.len());
            bi += 1;
            let mut sub = pos;
            while sub < end {
                let (c, p) = r.process(&[&src[sub..end]], &mut o2, produced, 37.min(2000 - produced));
                sub += c;
                produced += p;
                if c == 0 && p == 0 {
                    break;
                }
            }
            pos = end;
        }
        assert_eq!(produced, p1);
        assert_eq!(&o1[0][..p1], &o2[0][..p1]);
    }

    #[test]
    fn bypass_copies() {
        let src: Vec<f32> = (0..100).map(|i| i as f32).collect();
        let mut r = Resampler::new(1, 48000, false);
        r.set_pitch(0.0, false);
        assert_eq!(r.mode, Mode::Bypass);
        let mut o = vec![vec![0.0; 100]];
        assert_eq!(r.process(&[&src], &mut o, 0, 100), (100, 100));
        assert_eq!(o[0], src);
    }

    #[test]
    fn pitch_ramp_reaches_target() {
        let src: Vec<f32> = (0..20000).map(|i| (i as f32 * 0.01).sin()).collect();
        let mut r = Resampler::new(1, 48000, false);
        r.set_pitch(0.0, true);
        r.set_pitch(1200.0, true);
        assert_eq!(r.mode, Mode::Interpolating);
        let mut o = vec![vec![0.0; 4096]];
        let mut pos = 0;
        let mut produced = 0;
        while produced < 2048 {
            let (c, p) = r.process(&[&src[pos..]], &mut o, produced, 512);
            pos += c;
            produced += p;
        }
        assert_eq!(r.mode, Mode::Fixed);
        assert_eq!(r.step, 0x20000);
    }
}
