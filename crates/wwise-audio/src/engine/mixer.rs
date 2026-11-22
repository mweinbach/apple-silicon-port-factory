//! AK's software mixer core, reimplemented from `FUN_141658fc0` (reached from the exported
//! `AkMixer::MixOneInNChannels` / `MixNinNChannels` SSE paths `0x141657660` / `0x141657300`) [V].
//!
//! For each (input channel, output channel) pair the gain ramps linearly from
//! `prev_matrix * prev_gain` to `next_matrix * next_gain` over the frame:
//! ```text
//! start = prevMx[j] * prevGain;  diff = nextMx[j] * nextGain - start;  delta = diff * (1/frames)
//! if |diff| > FLT_EPSILON:  g[k] = delta*(k+1) + start, k = 0..15; out[n] += in[n]*g[n%16];
//!                            after every 16 samples g[k] += delta*16      (accumulated, f32)
//! elif |start| > FLT_EPSILON: out[n] += in[n]*start
//! ```
//! The AVX2 variants (`0x14165fca0`, `0x141660020`, selected when `0x143931f80 == 1`) are not yet
//! decompiled; `fma = true` assumes they fuse `in*g + out` [A].

const EPS: f32 = 1.192_092_9e-7;

/// Mix one input channel into one output channel with AK's per-frame gain ramp.
pub fn mix_channel(input: &[f32], out: &mut [f32], start: f32, target: f32, fma: bool) {
    let frames = input.len();
    debug_assert_eq!(frames, out.len());
    if frames == 0 {
        return;
    }
    let inv = 1.0f32 / frames as f32;
    let diff = target - start;
    if diff.abs() > EPS {
        let delta = diff * inv;
        let mut g = [0f32; 16];
        for (k, gk) in g.iter_mut().enumerate() {
            *gk = delta * (k as f32 + 1.0) + start;
        }
        let d16 = delta * 16.0;
        let d4 = delta * 4.0;
        let full = frames & !15;
        let mut n = 0;
        while n < full {
            for k in 0..16 {
                out[n + k] = mul_add(input[n + k], g[k], out[n + k], fma);
                g[k] += d16;
            }
            n += 16;
        }
        // tail in groups of 4 using lanes 0..3 (increment delta*4)
        while n < frames {
            for k in 0..4.min(frames - n) {
                out[n + k] = mul_add(input[n + k], g[k], out[n + k], fma);
                g[k] += d4;
            }
            n += 4;
        }
    } else if start.abs() > EPS {
        for n in 0..frames {
            out[n] = mul_add(input[n], start, out[n], fma);
        }
    }
}

#[inline(always)]
fn mul_add(x: f32, g: f32, acc: f32, fma: bool) -> f32 {
    if fma { x.mul_add(g, acc) } else { x * g + acc }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_gain_and_ramp() {
        let input = vec![1.0f32; 512];
        let mut out = vec![0.0f32; 512];
        mix_channel(&input, &mut out, 0.5, 0.5, false);
        assert!(out.iter().all(|&v| v == 0.5));
        let mut out = vec![0.0f32; 512];
        mix_channel(&input, &mut out, 0.0, 1.0, false);
        assert_eq!(out[0], 1.0 / 512.0);
        assert!((out[511] - 1.0).abs() < 1e-6);
    }
}
