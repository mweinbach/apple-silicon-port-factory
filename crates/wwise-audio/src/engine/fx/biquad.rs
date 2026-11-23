//! Core DSP primitives (`DspBiquad`, `DelayLine`) and Gain, Parametric EQ, Delay & Stereo Delay processors.

use super::params::{DelayParams, EqBandParams, EqFilterType, GainParams, ParametricEqParams, StereoDelayParams};
use crate::engine::math::{db_to_lin, RATE};
use std::f32::consts::PI;

/// Standard Audio EQ Cookbook biquad filter (`Direct Form II Transposed`) supporting all 7 Wwise
/// filter curves (`LowPass`, `HighPass`, `BandPass`, `Notch`, `LowShelf`, `HighShelf`, `Peaking`).
#[derive(Debug, Clone, Copy)]
pub struct DspBiquad {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
}

impl Default for DspBiquad {
    fn default() -> Self {
        Self {
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
        }
    }
}

impl DspBiquad {
    pub fn design(band: &EqBandParams, sample_rate: u32) -> Self {
        if !band.on_off {
            return Self::default();
        }
        let fs = sample_rate.max(1) as f32;
        let f0 = band.freq_hz.clamp(10.0, fs * 0.49);
        let q = band.q_factor.clamp(0.05, 40.0);
        let w0 = 2.0 * PI * f0 / fs;
        let cos_w0 = w0.cos();
        let sin_w0 = w0.sin();
        let alpha = sin_w0 / (2.0 * q);
        let a = 10.0f32.powf(band.gain_db.clamp(-48.0, 48.0) / 40.0);

        let (b0, b1, b2, a0, a1, a2) = match band.filter_type {
            EqFilterType::LowPass => {
                let b1 = 1.0 - cos_w0;
                let b0 = b1 * 0.5;
                (b0, b1, b0, 1.0 + alpha, -2.0 * cos_w0, 1.0 - alpha)
            }
            EqFilterType::HighPass => {
                let b0 = (1.0 + cos_w0) * 0.5;
                let b1 = -(1.0 + cos_w0);
                (b0, b1, b0, 1.0 + alpha, -2.0 * cos_w0, 1.0 - alpha)
            }
            EqFilterType::BandPass => (
                alpha,
                0.0,
                -alpha,
                1.0 + alpha,
                -2.0 * cos_w0,
                1.0 - alpha,
            ),
            EqFilterType::Notch => (
                1.0,
                -2.0 * cos_w0,
                1.0,
                1.0 + alpha,
                -2.0 * cos_w0,
                1.0 - alpha,
            ),
            EqFilterType::Peaking => (
                1.0 + alpha * a,
                -2.0 * cos_w0,
                1.0 - alpha * a,
                1.0 + alpha / a,
                -2.0 * cos_w0,
                1.0 - alpha / a,
            ),
            EqFilterType::LowShelf => {
                let sq = 2.0 * a.sqrt() * (sin_w0 * 0.5 * (2.0f32).sqrt());
                let ap1 = a + 1.0;
                let am1 = a - 1.0;
                (
                    a * (ap1 - am1 * cos_w0 + sq),
                    2.0 * a * (am1 - ap1 * cos_w0),
                    a * (ap1 - am1 * cos_w0 - sq),
                    ap1 + am1 * cos_w0 + sq,
                    -2.0 * (am1 + ap1 * cos_w0),
                    ap1 + am1 * cos_w0 - sq,
                )
            }
            EqFilterType::HighShelf => {
                let sq = 2.0 * a.sqrt() * (sin_w0 * 0.5 * (2.0f32).sqrt());
                let ap1 = a + 1.0;
                let am1 = a - 1.0;
                (
                    a * (ap1 + am1 * cos_w0 + sq),
                    -2.0 * a * (am1 + ap1 * cos_w0),
                    a * (ap1 + am1 * cos_w0 - sq),
                    ap1 - am1 * cos_w0 + sq,
                    2.0 * (am1 - ap1 * cos_w0),
                    ap1 - am1 * cos_w0 - sq,
                )
            }
        };

        let inv_a0 = if a0.abs() > 1e-12 { 1.0 / a0 } else { 1.0 };
        Self {
            b0: b0 * inv_a0,
            b1: b1 * inv_a0,
            b2: b2 * inv_a0,
            a1: a1 * inv_a0,
            a2: a2 * inv_a0,
        }
    }

    #[inline]
    pub fn tick(&self, x: f32, state: &mut [f32; 2]) -> f32 {
        let y = self.b0 * x + state[0];
        state[0] = self.b1 * x - self.a1 * y + state[1];
        state[1] = self.b2 * x - self.a2 * y;
        y
    }

    pub fn process_slice(&self, state: &mut [f32; 2], buf: &mut [f32]) {
        for s in buf.iter_mut() {
            *s = self.tick(*s, state);
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct DelayLine {
    pub(super) buf: Vec<f32>,
    write_pos: usize,
}

impl DelayLine {
    pub(super) fn new(max_samples: usize) -> Self {
        Self {
            buf: vec![0.0; max_samples.max(4)],
            write_pos: 0,
        }
    }

    #[inline]
    pub(super) fn read_delay(&self, delay_samples: usize) -> f32 {
        let len = self.buf.len();
        let d = delay_samples.min(len - 1);
        self.buf[(self.write_pos + len - d) % len]
    }

    #[inline]
    pub(super) fn read_frac(&self, delay_samples: f32) -> f32 {
        let len = self.buf.len();
        let d = delay_samples.clamp(0.0, (len - 2) as f32);
        let d_int = d as usize;
        let frac = d - d_int as f32;
        let i0 = (self.write_pos + len - d_int) % len;
        let i1 = (self.write_pos + len - d_int - 1) % len;
        self.buf[i0] * (1.0 - frac) + self.buf[i1] * frac
    }

    #[inline]
    pub(super) fn write_and_advance(&mut self, val: f32) {
        self.buf[self.write_pos] = val;
        self.write_pos += 1;
        if self.write_pos >= self.buf.len() {
            self.write_pos = 0;
        }
    }
}

pub(super) fn process_gain(p: &GainParams, buf: &mut [Vec<f32>], lfe_channel: Option<usize>) {
    let fb_lin = db_to_lin(p.fullband_gain_db);
    let lfe_lin = db_to_lin(p.lfe_gain_db);
    for (c, ch_buf) in buf.iter_mut().enumerate() {
        let g = if lfe_channel == Some(c) { lfe_lin } else { fb_lin };
        if (g - 1.0).abs() > 1e-6 {
            for s in ch_buf.iter_mut() {
                *s *= g;
            }
        }
    }
}

pub(super) fn process_parametric_eq(
    p: &ParametricEqParams,
    states: &mut Vec<[[f32; 2]; 3]>,
    buf: &mut [Vec<f32>],
    lfe_channel: Option<usize>,
) {
    let num_ch = buf.len();
    if states.len() < num_ch {
        states.resize(num_ch, [[0.0; 2]; 3]);
    }
    let biquads = [
        DspBiquad::design(&p.bands[0], RATE),
        DspBiquad::design(&p.bands[1], RATE),
        DspBiquad::design(&p.bands[2], RATE),
    ];
    let out_lin = db_to_lin(p.output_level_db);
    for (c, ch_buf) in buf.iter_mut().enumerate() {
        if lfe_channel == Some(c) && !p.process_lfe {
            continue;
        }
        for b in 0..3 {
            if p.bands[b].on_off {
                biquads[b].process_slice(&mut states[c][b], ch_buf);
            }
        }
        if (out_lin - 1.0).abs() > 1e-6 {
            for s in ch_buf.iter_mut() {
                *s *= out_lin;
            }
        }
    }
}

pub(super) fn process_delay(
    p: &DelayParams,
    lines: &mut Vec<DelayLine>,
    buf: &mut [Vec<f32>],
    lfe_channel: Option<usize>,
) {
    let num_ch = buf.len();
    let delay_samples = ((p.delay_time_s.clamp(0.0001, 5.0) * RATE as f32).round() as usize).max(1);
    if lines.len() < num_ch {
        lines.resize(num_ch, DelayLine::new(delay_samples + 4096));
    }
    for l in lines.iter_mut() {
        if l.buf.len() <= delay_samples + 2 {
            l.buf.resize(delay_samples + 256, 0.0);
        }
    }
    let fb = if p.feedback_enabled {
        (p.feedback * 0.01).clamp(0.0, 0.995)
    } else {
        0.0
    };
    let wet = (p.wet_dry_mix * 0.01).clamp(0.0, 1.0);
    let dry = 1.0 - wet;
    let out_lin = db_to_lin(p.output_level_db);
    for (c, ch_buf) in buf.iter_mut().enumerate() {
        if lfe_channel == Some(c) && !p.process_lfe {
            continue;
        }
        let line = &mut lines[c];
        for s in ch_buf.iter_mut() {
            let x = *s;
            let delayed = line.read_delay(delay_samples);
            line.write_and_advance((x + delayed * fb).clamp(-8.0, 8.0));
            *s = (x * dry + delayed * wet) * out_lin;
        }
    }
}

pub(super) fn process_stereo_delay(
    p: &StereoDelayParams,
    lines: &mut [DelayLine; 2],
    fb_filter_state: &mut [[f32; 2]; 2],
    buf: &mut [Vec<f32>],
) {
    let num_ch = buf.len();
    let frames = buf[0].len();
    let d0 = ((p.channels[0].delay_time_s.clamp(0.0001, 5.0) * RATE as f32).round() as usize).max(1);
    let d1 = ((p.channels[1].delay_time_s.clamp(0.0001, 5.0) * RATE as f32).round() as usize).max(1);
    if lines[0].buf.len() <= d0 + 2 {
        lines[0].buf.resize(d0 + 256, 0.0);
    }
    if lines[1].buf.len() <= d1 + 2 {
        lines[1].buf.resize(d1 + 256, 0.0);
    }
    let fb0 = if p.enable_feedback {
        db_to_lin(p.channels[0].feedback_db).min(0.995)
    } else {
        0.0
    };
    let fb1 = if p.enable_feedback {
        db_to_lin(p.channels[1].feedback_db).min(0.995)
    } else {
        0.0
    };
    let cf0 = if p.enable_crossfeed {
        db_to_lin(p.channels[0].crossfeed_db).min(0.995)
    } else {
        0.0
    };
    let cf1 = if p.enable_crossfeed {
        db_to_lin(p.channels[1].crossfeed_db).min(0.995)
    } else {
        0.0
    };
    let dry_lin = db_to_lin(p.dry_level_db);
    let wet_lin = db_to_lin(p.wet_level_db);
    let flt = if p.filter_type > 0 {
        let ft = match p.filter_type {
            1 => EqFilterType::LowShelf,
            2 => EqFilterType::Peaking,
            3 => EqFilterType::HighShelf,
            4 => EqFilterType::LowPass,
            5 => EqFilterType::HighPass,
            6 => EqFilterType::BandPass,
            _ => EqFilterType::Notch,
        };
        Some(DspBiquad::design(
            &EqBandParams {
                filter_type: ft,
                gain_db: p.filter_gain_db,
                freq_hz: p.filter_freq_hz,
                q_factor: p.filter_q,
                on_off: true,
            },
            RATE,
        ))
    } else {
        None
    };

    for i in 0..frames {
        let x0 = buf[0][i];
        let x1 = if num_ch > 1 { buf[1][i] } else { x0 };
        let y0 = lines[0].read_delay(d0);
        let y1 = lines[1].read_delay(d1);
        let mut in0 = (x0 + y0 * fb0 + y1 * cf1).clamp(-8.0, 8.0);
        let mut in1 = (x1 + y1 * fb1 + y0 * cf0).clamp(-8.0, 8.0);
        if let Some(b) = &flt {
            in0 = b.tick(in0, &mut fb_filter_state[0]);
            in1 = b.tick(in1, &mut fb_filter_state[1]);
        }
        lines[0].write_and_advance(in0);
        lines[1].write_and_advance(in1);
        buf[0][i] = x0 * dry_lin + y0 * wet_lin;
        if num_ch > 1 {
            buf[1][i] = x1 * dry_lin + y1 * wet_lin;
        }
    }
}
