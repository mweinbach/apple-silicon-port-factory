//! LPF / HPF filtering as Wwise 2022.1 does it on a mixing connection (voice -> bus, bus ->
//! parent bus).
//!
//! Source: the connection mixer `FUN_14163dbc0` (per connection: LPF values prev/next at +4/+8,
//! HPF at +0x10/+0x14, bypass bit 4 in bytes +0xd / +0x19). It filters each input channel (LPF,
//! then HPF) into a scratch buffer and then mixes it into the output channels
//! (`AkMixer::MixOneInNChannels`). When the value changes over a frame it computes one coefficient
//! set per sub-block (`DAT_1438152b8` sets of `DAT_1438152bc` samples), interpolating the value
//! linearly to each sub-block's end. A constant value reuses the cached set. [V: decompile]
//!
//! Coefficients: the value (0..100) maps to a cutoff with `FUN_141643050`, then a 2nd-order
//! Butterworth bilinear design. The kernel is `AkMixer::ProcessInterpBiquadFilter` ->
//! `0x141658c80` (SSE; the AVX variant `0x141661430` is selected by `DAT_143931f80 == 1`). With
//! one coefficient set it goes to `0x1416586a0`, which runs the 1-channel block-of-4 kernel
//! `0x141657850`. That kernel computes 4 outputs at once from precomputed impulse-response
//! vectors, and its exact operation order is ported in [`Biquad::process`] [V].

use std::f32::consts::PI;

/// Cutoff frequency for an LPF value `v` (0..100), `FUN_141643050(v, rate)` [V]. An HPF uses
/// `cutoff(100 - v)`.
///
/// For `v >= 30` it is `FastPow10`-style: `u = (u32)(i64)((100 - v) * 1042939.94 + 1065353216)`,
/// `m = bits((u & 0x7fffff) | 1.0)`, `((m * 0.32518977 + 0.020805772) * m + 0.65304345) *
/// bits(u & 0xff800000) * 16.797443`. Below 30 it is `(30 - v) * 433.33334 + 7000`. The result is
/// clamped to `rate * 0.45`.
pub fn cutoff(v: f32, rate: u32) -> f32 {
    let f = if v >= 30.0 {
        let t = (100.0 - v) * 1042939.94f32 + 1.0653532e9f32;
        let u = (t as i64) as u32;
        let m = f32::from_bits((u & 0x7f_ffff).wrapping_add(0x3f80_0000));
        ((m * 0.32518977f32 + 0.020805772f32) * m + 0.65304345f32) * f32::from_bits(u & 0xff80_0000) * 16.797443f32
    } else {
        (30.0 - v) * 433.33334f32 + 7000.0
    };
    let lim = rate as f32 * 0.45;
    if f <= lim {
        f
    } else {
        lim
    }
}

/// Biquad coefficients as AK stores them (`b0, b1, b2, a1, a2`; `y = b0 x + b1 x1 + b2 x2 - a1 y1
/// - a2 y2`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Biquad {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,
}

impl Biquad {
    /// Butterworth low-pass, `FUN_14163dbc0` [V]: `K = 1/tan(fc/fs*pi)`, `n = 1/(K*sqrt2 + 1 + K^2)`,
    /// `b = (n, 2n, n)`, `a1 = ((1-K^2)+(1-K^2))*n`, `a2 = ((1 - K*sqrt2) + K^2)*n`.
    pub fn lowpass(fc: f32, rate: u32) -> Biquad {
        let k = 1.0 / ((fc / rate as f32) * PI).tan();
        let k2 = k * k;
        let n = 1.0 / (k * 1.4142135 + 1.0 + k2);
        Biquad { b0: n, b1: n + n, b2: n, a1: ((1.0 - k2) + (1.0 - k2)) * n, a2: ((1.0 - k * 1.4142135) + k2) * n }
    }

    /// Butterworth high-pass, `FUN_14163dbc0` [V]: `K = tan(fc/fs*pi)`, `t = K^2 + 1`,
    /// `n = 1/(t + K*sqrt2)`, `b = (n, -2n, n)`, `a1 = (1-K^2)*n*-2`, `a2 = (t - K*sqrt2)*n`.
    pub fn highpass(fc: f32, rate: u32) -> Biquad {
        let k = ((fc / rate as f32) * PI).tan();
        let t = k * k + 1.0;
        let n = 1.0 / (t + k * 1.4142135);
        Biquad { b0: n, b1: n * -2.0, b2: n, a1: (1.0 - k * k) * n * -2.0, a2: (t - k * 1.4142135) * n }
    }

    /// The block-of-4 vectors of kernel `0x141657850`: responses of `y[n..n+3]` to
    /// x[n-2], x[n-1], x[n], x[n+1], x[n+2], y[n-1], y[n-2], plus b0 for the direct term.
    /// Operation order as in the asm [V].
    fn vectors(&self) -> Vectors {
        let (b0, b1, b2) = (self.b0, self.b1, self.b2);
        let a1 = -self.a1; // the kernel negates a1 and a2 (xorps with the sign mask)
        let a2 = -self.a2;
        let h1 = a1 * b0 + b1;
        let h2 = (h1 * a1 + b2) + a2 * b0;
        let h3 = (h2 * a1 + (a2 * a1) * b0) + a2 * b1;
        // response to x[n-1]
        let g1 = a1 * b1 + b2;
        let g2 = g1 * a1 + a2 * b1;
        let g3 = (g2 + a2 * b1) * a1 + a2 * b2;
        // response to x[n-2]
        let e1 = a1 * b2;
        let e2 = e1 * a1 + a2 * b2;
        let e3 = ((b2 + b2) * a2 + e1 * a1) * a1;
        // response to y[n-1]
        let a1a1 = a1 * a1;
        let p1 = a1a1 + a2;
        let p2 = ((a2 + a2) + a1a1) * a1;
        let p3 = ((a2 * 3.0 + a1a1) * a1) * a1 + a2 * a2;
        // response to y[n-2]
        let a2a1 = a2 * a1;
        let q2 = a2a1 * a1 + a2 * a2;
        let q3 = ((a2 + a2) * a2 + a2a1 * a1) * a1;
        Vectors {
            xm2: [b2, e1, e2, e3],
            xm1: [b1, g1, g2, g3],
            b0,
            x2: [0.0, 0.0, 0.0, h1],
            x1: [0.0, 0.0, h1, h2],
            x0: [0.0, h1, h2, h3],
            ym1: [a1, p1, p2, p3],
            ym2: [a2, a2a1, q2, q3],
        }
    }

    /// Filter `inp` into `out` (same length, a multiple of 4) with state `mem` =
    /// `[y[n-2], y[n-1], x[n-2], x[n-1]]`, as kernel `0x141657850` does [V].
    pub fn process(&self, mem: &mut [f32; 4], inp: &[f32], out: &mut [f32]) {
        let v = self.vectors();
        let (mut ym2, mut ym1, mut xm2, mut xm1) = (mem[0], mem[1], mem[2], mem[3]);
        let n = inp.len().min(out.len());
        let mut i = 0;
        while i + 4 <= n {
            let x = [inp[i], inp[i + 1], inp[i + 2], inp[i + 3]];
            for l in 0..4 {
                let mut acc = v.xm2[l] * xm2 + v.xm1[l] * xm1;
                acc += v.b0 * x[l];
                acc += v.x2[l] * x[2];
                acc += v.x1[l] * x[1];
                acc += v.x0[l] * x[0];
                acc += v.ym1[l] * ym1;
                acc += v.ym2[l] * ym2;
                out[i + l] = acc;
            }
            ym2 = out[i + 2];
            ym1 = out[i + 3];
            xm2 = x[2];
            xm1 = x[3];
            i += 4;
        }
        *mem = [ym2, ym1, xm2, xm1];
    }
}

struct Vectors {
    xm2: [f32; 4],
    xm1: [f32; 4],
    b0: f32,
    x2: [f32; 4],
    x1: [f32; 4],
    x0: [f32; 4],
    ym1: [f32; 4],
    ym2: [f32; 4],
}

/// Per-channel LPF + HPF state of one mixing connection with constant values.
#[derive(Debug, Clone)]
pub struct ConnectionFilter {
    lpf: Option<Biquad>,
    hpf: Option<Biquad>,
    lpf_mem: Vec<[f32; 4]>,
    hpf_mem: Vec<[f32; 4]>,
}

impl ConnectionFilter {
    /// `lpf` / `hpf` values 0..100 (0 = bypassed), `channels` input channels.
    pub fn new(lpf: f32, hpf: f32, channels: usize, rate: u32) -> ConnectionFilter {
        ConnectionFilter {
            lpf: (lpf > 0.0).then(|| Biquad::lowpass(cutoff(lpf, rate), rate)),
            hpf: (hpf > 0.0).then(|| Biquad::highpass(cutoff(100.0 - hpf, rate), rate)),
            lpf_mem: vec![[0.0; 4]; channels],
            hpf_mem: vec![[0.0; 4]; channels],
        }
    }

    pub fn is_active(&self) -> bool {
        self.lpf.is_some() || self.hpf.is_some()
    }

    /// Filter channel `ch` in place (LPF first, then HPF, like `FUN_14163dbc0`).
    pub fn process(&mut self, ch: usize, buf: &mut [f32]) {
        let mut tmp = buf.to_vec();
        if let Some(b) = self.lpf {
            b.process(&mut self.lpf_mem[ch], &tmp, buf);
            tmp.copy_from_slice(buf);
        }
        if let Some(b) = self.hpf {
            b.process(&mut self.hpf_mem[ch], &tmp, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The block kernel equals the direct-form recursion up to rounding.
    #[test]
    fn block_kernel_matches_direct_form() {
        let b = Biquad::lowpass(cutoff(17.0, 48000), 48000);
        let x: Vec<f32> = (0..512).map(|i| ((i * 7919) % 1000) as f32 / 500.0 - 1.0).collect();
        let mut y = vec![0.0; 512];
        let mut mem = [0.0; 4];
        b.process(&mut mem, &x, &mut y);
        let (mut x1, mut x2, mut y1, mut y2) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
        for i in 0..512 {
            let xn = x[i] as f64;
            let yn = b.b0 as f64 * xn + b.b1 as f64 * x1 + b.b2 as f64 * x2 - b.a1 as f64 * y1 - b.a2 as f64 * y2;
            assert!((yn - y[i] as f64).abs() < 1e-5, "sample {i}: {yn} vs {}", y[i]);
            x2 = x1;
            x1 = xn;
            y2 = y1;
            y1 = yn;
        }
    }

    #[test]
    fn cutoff_mapping() {
        assert_eq!(cutoff(0.0, 48000), 20000.0);
        assert!((cutoff(17.0, 48000) - 12633.333).abs() < 0.01);
        assert!((cutoff(100.0, 48000) - 16.79).abs() < 0.1);
    }
}
