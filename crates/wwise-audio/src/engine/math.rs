//! Numeric primitives shared by the offline renderer.
//!
//! Every formula is tagged with where it comes from. [V] = decompiled from AlanWake2.exe (AK
//! 2022.1.4), [A] = assumption not yet checked against the original.

/// AK frame size in sample frames (`AkInitSettings::uNumSamplesPerFrame`, 512, observed by the A1
/// harness in `IAudioClient::Initialize`) [V].
pub const FRAME: usize = 512;
/// Engine sample rate (`DAT_1438152ac`; every A1 capture runs at 48 kHz) [V].
pub const RATE: u32 = 48_000;

/// AK's fast dB -> linear conversion (`AkMath::dBToLin` / `FastPow10`), as inlined e.g. in
/// `FUN_1415295b0` [V]:
/// ```text
/// x = dB * 0.05;  if x < -37 -> 0
/// u = (u32)(i64)(x * 27866352.0f + 1065353216.0f)          // cvttss2si, all f32 ops
/// m = bits((u & 0x7fffff) + 0x3f800000)                     // mantissa in [1,2)
/// lin = ((m * 0.32518977f + 0.020805772f) * m + 0.65304345f) * bits(u & 0xff800000)
/// ```
/// Relative error vs `10^(dB/20)` is up to ~1e-3 (0 dB maps to 0.999039), so using exact `powf` would leave a residual
/// well above the -90 dBFS target. Which call sites use this vs `powf(10, dB*0.05)` (functions with
/// constants `0.05, 10`, e.g. `0x1414ecb50`) is [A] per site; the voice/bus volume path is assumed
/// to use this one until verified.
#[inline]
pub fn db_to_lin(db: f32) -> f32 {
    fast_pow10(db * 0.05f32)
}

/// `FastPow10(x) ~ 10^x` (the core of [`db_to_lin`], also used directly by the Log curve scaling
/// in `FUN_141529ed0`) [V].
#[inline]
pub fn fast_pow10(x: f32) -> f32 {
    if !(x >= -37.0f32) {
        return 0.0;
    }
    let t = x * 27866352.0f32 + 1.0653532e9f32;
    let u = (t as i64) as u32;
    let m = f32::from_bits((u & 0x7f_ffff).wrapping_add(0x3f80_0000));
    let e = f32::from_bits(u & 0xff80_0000);
    ((m * 0.32518977f32 + 0.020805772f32) * m + 0.65304345f32) * e
}

/// Exact `powf(10, dB * 0.05)` (the AK `0.05, 10` variant).
#[inline]
pub fn db_to_lin_exact(db: f32) -> f32 {
    10f32.powf(db * 0.05)
}

/// AK fast linear -> dB (`AkMath::FastLinTodB`, inlined in `FUN_1415295b0`) [V]:
/// `m = bits((u & 0x7fffff) + 0x3f800000); ((m^3 - 1) * 1.1581186 / (m+1)^3) * 10 + (e - 127) * 3.0103002`
/// where `u = bits(power)` (input is a power/energy value, so the result is 10*log10).
#[inline]
pub fn fast_pow_to_db(p: f32) -> f32 {
    if p <= 0.0 {
        return -3.4028235e38;
    }
    let u = p.to_bits();
    let m = f32::from_bits((u & 0x7f_ffff) + 0x3f80_0000);
    let m1 = m + 1.0;
    (((m * m * m - 1.0) * 1.1581186) / (m1 * m1 * m1)) * 10.0 + (((u >> 23) & 0xff) as f32 - 127.0) * 3.0103002
}

/// Shape of one curve segment, `AkCurveInterpolation` (0 Log3, 1 Sine, 2 Log1, 3 InvSCurve,
/// 4 Linear, 5 SCurve, 6 Exp1, 7 SineRecip, 8 Exp3, 9 Constant).
///
/// Linear and Constant are exact by definition. The other shapes follow the published Wwise fade
/// curve definitions [A: AK's `CAkInterpolation` table/approximation not yet decompiled].
pub fn interp_shape(shape: u32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    use std::f32::consts::FRAC_PI_2;
    match shape {
        0 => 1.0 - (1.0 - t).powi(3),             // Log3
        1 => (t * FRAC_PI_2).sin(),               // Sine (constant power fade-in)
        2 => 1.0 - (1.0 - t).powf(1.41),          // Log1
        3 => {
            // Inverted S-curve
            if t < 0.5 {
                0.5 * (1.0 - (1.0 - 2.0 * t).powi(2))
            } else {
                0.5 + 0.5 * (2.0 * t - 1.0).powi(2)
            }
        }
        5 => 0.5 * (1.0 - (std::f32::consts::PI * t).cos()), // S-curve
        6 => t.powf(1.41),                                    // Exp1
        7 => 1.0 - (t * FRAC_PI_2).cos(),                     // Sine reciprocal
        8 => t.powi(3),                                       // Exp3
        9 => 0.0,                                             // Constant (hold start value)
        _ => t,                                               // 4 Linear (and unknown)
    }
}

/// Evaluate an RTPC / conversion graph at `x`: `CAkConversionTable::Convert` (`FUN_141529ed0`)
/// with the per-segment coefficients its loader `FUN_14152a3c0` precomputes [V].
///
/// Loader: the points are framed by sentinels (`from = -FLT_MAX` / `+FLT_MAX`, Constant, holding
/// the first / last value). Per segment `i`: `inv = 1/(from[i+1]-from[i])`, `dy = to[i+1]-to[i]`;
/// Constant: `inv = 0`; Linear: `inv = dy*inv` (slope); Sine/SineRecip: `inv *= pi/2`;
/// SCurve/InvSCurve: `inv *= pi`; Log1: `dy *= -0.5`; Exp1: `dy *= 0.5`.
///
/// Convert: segment with `from[i] <= x < from[i+1]`, `t = (x-from[i])*inv`; Linear/Constant
/// return `t + to[i]`, the other shapes use the polynomial approximations below (constants from
/// `.rdata 0x142676604..0x142676648`). Scaling: 2 (dB) maps the curve value `y` (clamped to
/// [-1,1]) through `20*log10(1+y)` for `y<0` and `-20*log10(1-y)` for `y>=0` with AK's fast log
/// (`(m^3-1)*1.1581186/(m+1)^3*20 + (e-127)*6.0206`); 3 (Log) `FastPow10(y)`; 4 (dBToLin)
/// `FastPow10(y*0.05)`.
pub fn eval_graph(points: &[crate::bnk::hirc::GraphPoint], scaling: u8, x: f32) -> f32 {
    if points.is_empty() {
        return 0.0;
    }
    // sentinel-framed segment search
    let n = points.len();
    let from = |k: usize| if k == 0 { f32::MIN } else if k > n { f32::MAX } else { points[k - 1].from };
    let to = |k: usize| if k == 0 { points[0].to } else if k > n { points[n - 1].to } else { points[k - 1].to };
    let interp = |k: usize| if k == 0 || k > n { 9 } else { points[k - 1].interp };
    let mut i = 0usize;
    while i + 1 <= n && x >= from(i + 1) {
        i += 1;
    }
    let (ya, yb) = (to(i), to(i + 1));
    let shape = interp(i);
    let mut inv = 1.0f32 / (from(i + 1) - from(i));
    let mut dy = yb - ya;
    match shape {
        9 => inv = 0.0,
        4 => inv *= dy,
        1 | 7 => inv *= 1.5707964,
        3 | 5 => inv *= std::f32::consts::PI,
        2 => dy *= -0.5,
        6 => dy *= 0.5,
        _ => {}
    }
    let t = (x - from(i)) * inv;
    let y = match shape {
        4 | 9 => t + ya,
        0 => {
            let u = 1.0 - t;
            yb - u * u * (dy * u)
        }
        1 => {
            let mut a = f32::from_bits(0x3ebac154) - t * f32::from_bits(0x3dedc8ba);
            let mut b = f32::from_bits(0x3dedc8b9) - t * f32::from_bits(0x3d1760c7);
            a *= t;
            b = b * t * t;
            a -= b;
            a += f32::from_bits(0x3f7c9c57) - t * f32::from_bits(0x3ea0d125);
            a * t * dy + ya
        }
        2 => (t - 3.0) * t * dy + ya,
        3 => {
            let s = |u: f32| {
                let u2 = u * u;
                (((f32::from_bits(0x3b881741) - u2 * f32::from_bits(0x38c08e8f)) * u2 - f32::from_bits(0x3daaa5d9)) * u2 + f32::from_bits(0x3effffc7)) * u
            };
            if t > 1.5707964 {
                (1.0 - s(std::f32::consts::PI - t)) * dy + ya
            } else {
                s(t) * dy + ya
            }
        }
        5 => {
            let t2 = t * t;
            let p = ((t2 * f32::from_bits(0x39fe3151) - f32::from_bits(0x3ca0ad34)) * t2 + f32::from_bits(0x3e7d9e76)) * t2 + f32::from_bits(0x3a36a2e4);
            p.min(1.0) * dy + ya
        }
        6 => (t + 1.0) * t * dy + ya,
        7 => {
            let t2 = t * t;
            let c = ((f32::from_bits(0x3d29ef0f) - t2 * f32::from_bits(0x3aa69eb6)) * t2 - f32::from_bits(0x3efff486)) * t2 + f32::from_bits(0x3f7fff90);
            yb - c * dy
        }
        8 => dy * t * (t * t) + ya,
        _ => 0.0,
    };
    match scaling {
        2 => {
            let (s, m) = if y >= 0.0 { (-1.0f32, y.min(1.0)) } else { (1.0f32, if y >= -1.0 { y.min(1.0) } else { -1.0 }) };
            let x0 = s * m + 1.0;
            let u = x0.to_bits();
            let mm = f32::from_bits((u & 0x7f_ffff) + 0x3f80_0000);
            let m1 = mm + 1.0;
            let num = (mm * mm * mm - 1.0) * f32::from_bits(0x3f943d3b);
            let r = (num / (m1 * m1 * m1)) * 20.0 + (((u >> 23) & 0xff) as f32 - 127.0) * f32::from_bits(0x40c0a8c2);
            r * s
        }
        3 => fast_pow10(y),
        4 => fast_pow10(y * 0.05),
        _ => y,
    }
}

/// `AkInterpolation::InterpolateNoCheck(t, from, to, curve)` = `FUN_1415ad4d0` (jump table
/// `0x1415ad800`), operation order as in the asm (SSE, no FMA) [V]. `t` in [0,1].
pub fn ak_interpolate(t: f32, a: f32, b: f32, curve: u32) -> f32 {
    let c = |h: u32| f32::from_bits(h);
    match curve {
        0 => {
            // Log3
            let u = 1.0 - t;
            (a - b) * u * (u * u) + b
        }
        1 => {
            // Sine
            let x3 = t * c(0x3fc90fdb);
            let x2 = c(0x3e92ad5c) - x3 * c(0x3dbac154);
            let x0 = t * c(0x40490fdb);
            let t2 = t * t;
            let mut x4 = t * x2 * x0;
            let x1 = x3 * x2 * t2;
            x4 -= x1;
            let y2 = (c(0x3f7c9c57) - x3 * c(0x3ea0d125)) * x3;
            (x4 + y2) * (b - a) + a
        }
        2 => (t - 3.0) * t * ((a - b) * 0.5) + a, // Log1
        3 => {
            // InvSCurve
            let pi = c(0x40490fdb);
            let poly = |u2: f32| ((c(0x3b881741) - u2 * c(0x38c08e8f)) * u2 - c(0x3daaa5d9)) * u2 + c(0x3effffc7);
            let x2 = t * pi;
            if t > 0.5 {
                let x3 = pi - x2;
                (1.0 - poly(x3 * x3) * x3) * (b - a) + a
            } else {
                let t2 = t * t * c(0x411de9e7);
                poly(t2) * x2 * (b - a) + a
            }
        }
        4 => (b - a) * t + a, // Linear
        5 => {
            // SCurve
            let t2 = t * t * c(0x411de9e7);
            let mut x0 = ((t2 * c(0x39fe3151) - c(0x3ca0ad34)) * t2 + c(0x3e7d9e76)) * t2 + c(0x3a36a2e4);
            if x0 >= 1.0 {
                x0 = 1.0;
            }
            (b - a) * x0 + a
        }
        6 => (t + 1.0) * t * ((b - a) * 0.5) + a, // Exp1
        7 => {
            // SineRecip
            let t2 = t * t * c(0x401de9e7);
            (((c(0x3d29ef0f) - t2 * c(0x3aa69eb6)) * t2 - c(0x3efff486)) * t2 + c(0x3f7fff90)) * (a - b) + b
        }
        8 => (b - a) * t * (t * t) + a, // Exp3
        _ => 0.0,
    }
}

/// `g_msPerBufferTick` (`DAT_1438152a8`, static init 10 = 512 samples / 48 per ms, integer) [V].
pub const MS_PER_BUFFER_TICK: u32 = 10;

/// Transition duration in buffer ticks: `(g_msPerBufferTick - 1 + ms) / g_msPerBufferTick`
/// (`FUN_1415ada30`) [V]. One tick is one 512-sample frame, so a 1250 ms fade lasts 125 frames
/// (1.333 s). `len` is the fade length in samples at 48 kHz (ms * 48).
pub fn transition_ticks(len_samples: u64) -> u64 {
    let per = (MS_PER_BUFFER_TICK * RATE / 1000) as u64; // 480 samples per nominal tick-ms unit
    len_samples.div_ceil(per)
}

/// Value of a transition `from -> to` with `curve`, `elapsed` ticks after it started, of
/// `ticks` total (`FUN_1415ad0e0`) [V]: done (exact target) once `elapsed >= ticks`, else
/// `t = (f32)elapsed / (f32)ticks`. `FUN_1415ada30` mirrors the curve (`8 - curve`) when
/// `to <= from` unless it is SCurve or InvSCurve.
pub fn transition_value(from: f32, to: f32, curve: u32, elapsed: u64, ticks: u64) -> f32 {
    if elapsed >= ticks {
        return to;
    }
    let t = if elapsed == 0 { 0.0 } else { elapsed as u32 as f32 / ticks as u32 as f32 };
    let curve = if to <= from && curve != 3 && curve != 5 && curve <= 8 { 8 - curve } else { curve };
    ak_interpolate(t, from, to, curve)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fast_db_to_lin_is_close_to_pow() {
        for i in -960..=240 {
            let db = i as f32 * 0.1;
            let a = db_to_lin(db);
            let b = db_to_lin_exact(db);
            assert!(((a - b) / b).abs() < 3e-3, "{} dB: {} vs {}", db, a, b);
        }
        assert_eq!(db_to_lin(-800.0), 0.0);
    }

    #[test]
    fn fast_db_to_lin_unity() {
        // 0 dB -> u = 0x3f800000 exactly -> m = 1, e = 1 -> the polynomial at 1, which is NOT 1.0:
        // AK's unity gain is 0.999039 (-0.0084 dB). No special case for 0 dB in the inlined code.
        let v = db_to_lin(0.0);
        assert_eq!(v, (0.32518977f32 + 0.020805772f32) * 1.0 + 0.65304345f32);
    }
}
