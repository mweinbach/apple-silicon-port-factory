//! Modulation, distortion, pitch-shifting & time-stretch DSP processors:
//! Tremolo, Flanger, Guitar Distortion, Pitch Shifter, Harmonizer, and Time Stretch.

use super::biquad::{DelayLine, DspBiquad};
use super::params::{
    EqBandParams, EqFilterType, FlangerParams, GuitarDistortionParams, HarmonizerParams, PitchShifterParams,
    TimeStretchParams, TremoloParams,
};
use crate::engine::math::{db_to_lin, RATE};
use std::f32::consts::PI;

#[derive(Debug, Clone, Default)]
pub(super) struct GuitarDistortionState {
    pre_states: Vec<[[f32; 2]; 3]>,
    post_states: Vec<[[f32; 2]; 3]>,
    tone_states: Vec<[f32; 2]>,
}

#[derive(Debug, Clone)]
pub(super) struct PitchShifterState {
    lines: Vec<DelayLine>,
    phase: f32,
    filter_states: Vec<[f32; 2]>,
}

impl PitchShifterState {
    pub(super) fn new() -> Self {
        Self {
            lines: vec![DelayLine::new(8192); 2],
            phase: 0.0,
            filter_states: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct HarmonizerState {
    lines: Vec<DelayLine>,
    phases: [f32; 2],
    filter_states: Vec<[[f32; 2]; 2]>,
}

impl HarmonizerState {
    pub(super) fn new() -> Self {
        Self {
            lines: vec![DelayLine::new(8192); 2],
            phases: [0.0; 2],
            filter_states: Vec::new(),
        }
    }
}

#[inline]
fn eval_lfo(waveform: u32, phase: f32, pwm_pct: f32) -> f32 {
    let p = phase.rem_euclid(1.0);
    match waveform {
        0 => (2.0 * PI * p).sin(), // Sine
        1 => {
            // Triangle
            if p < 0.5 {
                4.0 * p - 1.0
            } else {
                3.0 - 4.0 * p
            }
        }
        2 => {
            // Square with PWM
            let duty = (pwm_pct * 0.01).clamp(0.05, 0.95);
            if p < duty {
                1.0
            } else {
                -1.0
            }
        }
        3 => 2.0 * p - 1.0, // Sawtooth up
        4 => 1.0 - 2.0 * p, // Sawtooth down
        _ => (2.0 * PI * p).sin(),
    }
}

pub(super) fn process_tremolo(
    p: &TremoloParams,
    phase: &mut f32,
    buf: &mut [Vec<f32>],
    lfe_channel: Option<usize>,
) {
    let num_ch = buf.len();
    let frames = buf[0].len();
    let depth = (p.mod_depth * 0.01).clamp(0.0, 1.0);
    let out_lin = db_to_lin(p.output_level_db);
    let dphase = p.lfo_freq_hz.max(0.0) / RATE as f32;
    for i in 0..frames {
        let ph_base = (*phase + p.phase_offset_deg / 360.0).rem_euclid(1.0);
        for (c, ch_buf) in buf.iter_mut().enumerate() {
            if lfe_channel == Some(c) && !p.process_lfe {
                continue;
            }
            let ch_spread = if p.phase_mode != 0 && num_ch > 1 {
                (c as f32 / (num_ch - 1) as f32) * (p.phase_spread_deg / 360.0)
            } else {
                0.0
            };
            let ph = (ph_base + ch_spread).rem_euclid(1.0);
            let lfo = eval_lfo(p.lfo_waveform, ph, p.lfo_pwm); // in -1..=1
            // Amplitude modulation between (1 - depth) and 1.0
            let mod_gain = 1.0 - depth * 0.5 * (1.0 - lfo);
            ch_buf[i] *= mod_gain * out_lin;
        }
        *phase = (*phase + dphase).rem_euclid(1.0);
    }
}

pub(super) fn process_flanger(
    p: &FlangerParams,
    lines: &mut Vec<DelayLine>,
    phase: &mut f32,
    buf: &mut [Vec<f32>],
    lfe_channel: Option<usize>,
) {
    let num_ch = buf.len();
    let frames = buf[0].len();
    if lines.len() < num_ch {
        lines.resize(num_ch, DelayLine::new((0.05 * RATE as f32) as usize + 256));
    }
    let base_delay_samples = (p.delay_time_ms.clamp(0.1, 40.0) * 0.001 * RATE as f32).max(1.0);
    let depth = (p.mod_depth * 0.01).clamp(0.0, 1.0);
    let dphase = if p.enable_lfo {
        p.lfo_freq_hz.max(0.0) / RATE as f32
    } else {
        0.0
    };
    let wet = (p.wet_dry_mix * 0.01).clamp(0.0, 1.0);
    let dry = 1.0 - wet;
    let out_lin = db_to_lin(p.output_level_db);
    let fback = p.fback_level.clamp(-0.98, 0.98);
    let ffwd = p.ffwd_level.clamp(-1.0, 1.0);
    let dry_inner = p.dry_level.clamp(0.0, 1.0);

    for i in 0..frames {
        let ph_base = (*phase + p.phase_offset_deg / 360.0).rem_euclid(1.0);
        for (c, ch_buf) in buf.iter_mut().enumerate() {
            if lfe_channel == Some(c) && !p.process_lfe {
                continue;
            }
            let ch_spread = if p.phase_mode != 0 && num_ch > 1 {
                (c as f32 / (num_ch - 1) as f32) * (p.phase_spread_deg / 360.0)
            } else {
                0.0
            };
            let lfo = if p.enable_lfo {
                eval_lfo(p.lfo_waveform, (ph_base + ch_spread).rem_euclid(1.0), p.lfo_pwm)
            } else {
                0.0
            };
            let cur_delay = (base_delay_samples * (1.0 + depth * 0.5 * (1.0 + lfo))).max(1.0);
            let x = ch_buf[i];
            let delayed = lines[c].read_frac(cur_delay);
            lines[c].write_and_advance((x + delayed * fback).clamp(-8.0, 8.0));
            let y_flange = x * dry_inner + delayed * ffwd;
            ch_buf[i] = (x * dry + y_flange * wet) * out_lin;
        }
        *phase = (*phase + dphase).rem_euclid(1.0);
    }
}

pub(super) fn process_guitar_distortion(
    p: &GuitarDistortionParams,
    st: &mut GuitarDistortionState,
    buf: &mut [Vec<f32>],
) {
    let num_ch = buf.len();
    if st.pre_states.len() < num_ch {
        st.pre_states.resize(num_ch, [[0.0; 2]; 3]);
        st.post_states.resize(num_ch, [[0.0; 2]; 3]);
        st.tone_states.resize(num_ch, [0.0; 2]);
    }
    let pre_bq = [
        DspBiquad::design(&p.pre_eq[0], RATE),
        DspBiquad::design(&p.pre_eq[1], RATE),
        DspBiquad::design(&p.pre_eq[2], RATE),
    ];
    let post_bq = [
        DspBiquad::design(&p.post_eq[0], RATE),
        DspBiquad::design(&p.post_eq[1], RATE),
        DspBiquad::design(&p.post_eq[2], RATE),
    ];
    // Tone control (-100..100) maps to a high-shelf tilt around 1.2 kHz
    let tone_bq = (p.tone.abs() > 0.1).then(|| {
        DspBiquad::design(
            &EqBandParams {
                filter_type: EqFilterType::HighShelf,
                gain_db: (p.tone * 0.15).clamp(-15.0, 15.0),
                freq_hz: 1200.0,
                q_factor: 0.707,
                on_off: true,
            },
            RATE,
        )
    });
    let drive_norm = (p.drive * 0.01).clamp(0.0, 1.0);
    let drive_gain = 1.0 + drive_norm * 24.0;
    let rect = (p.rectification * 0.01).clamp(0.0, 1.0);
    let wet = (p.wet_dry_mix * 0.01).clamp(0.0, 1.0);
    let dry = 1.0 - wet;
    let out_lin = db_to_lin(p.output_level_db);

    for (c, ch_buf) in buf.iter_mut().enumerate() {
        for s in ch_buf.iter_mut() {
            let dry_x = *s;
            let mut x = dry_x;
            for b in 0..3 {
                if p.pre_eq[b].on_off {
                    x = pre_bq[b].tick(x, &mut st.pre_states[c][b]);
                }
            }
            if p.drive > 0.0 || p.rectification > 0.0 {
                let driven = x * drive_gain;
                let mut shaped = match p.distortion_type {
                    0 => driven.tanh(), // Overdrive
                    1 => {
                        // Heavy: steeper cubic/tanh saturation
                        (driven * 1.5).tanh()
                    }
                    2 => {
                        // Fuzz: asymmetric waveshaper
                        if driven >= 0.0 {
                            1.0 - (-driven * 1.8).exp()
                        } else {
                            -(1.0 - (driven * 1.2).exp()) * 0.8
                        }
                    }
                    _ => driven.clamp(-1.0, 1.0), // Hard clip
                };
                if rect > 0.0 {
                    shaped = shaped * (1.0 - rect) + shaped.abs() * rect;
                }
                x = shaped;
            }
            if let Some(tb) = &tone_bq {
                x = tb.tick(x, &mut st.tone_states[c]);
            }
            for b in 0..3 {
                if p.post_eq[b].on_off {
                    x = post_bq[b].tick(x, &mut st.post_states[c][b]);
                }
            }
            *s = (dry_x * dry + x * wet) * out_lin;
        }
    }
}

pub(super) fn process_pitch_shifter(
    p: &PitchShifterParams,
    st: &mut PitchShifterState,
    buf: &mut [Vec<f32>],
    lfe_channel: Option<usize>,
) {
    let num_ch = buf.len();
    let frames = buf[0].len();
    if st.lines.len() < num_ch {
        st.lines.resize(num_ch, DelayLine::new(8192));
    }
    if st.filter_states.len() < num_ch {
        st.filter_states.resize(num_ch, [0.0; 2]);
    }
    let win_samples = (p.delay_time_ms.clamp(1.0, 150.0) * 0.001 * RATE as f32).clamp(64.0, 4096.0);
    let pitch_ratio = 2.0f32.powf(p.pitch_cents.clamp(-2400.0, 2400.0) / 1200.0);
    let dphase = (1.0 - pitch_ratio) / win_samples;
    let dry_lin = db_to_lin(p.dry_level_db);
    let wet_lin = db_to_lin(p.wet_level_db);
    let flt = (p.filter_type > 0).then(|| {
        let ft = match p.filter_type {
            1 => EqFilterType::LowShelf,
            2 => EqFilterType::Peaking,
            3 => EqFilterType::HighShelf,
            4 => EqFilterType::LowPass,
            5 => EqFilterType::HighPass,
            6 => EqFilterType::BandPass,
            _ => EqFilterType::Notch,
        };
        DspBiquad::design(
            &EqBandParams {
                filter_type: ft,
                gain_db: p.filter_gain_db,
                freq_hz: p.filter_freq_hz,
                q_factor: p.filter_q,
                on_off: true,
            },
            RATE,
        )
    });
    let sync_delay = (win_samples * 0.5) as usize;

    for i in 0..frames {
        let ph0 = st.phase.rem_euclid(1.0);
        let ph1 = (ph0 + 0.5).rem_euclid(1.0);
        let w0 = 0.5 * (1.0 - (2.0 * PI * ph0).cos());
        let w1 = 1.0 - w0;
        let d0 = ph0 * win_samples + 1.0;
        let d1 = ph1 * win_samples + 1.0;

        for (c, ch_buf) in buf.iter_mut().enumerate() {
            if lfe_channel == Some(c) && !p.process_lfe {
                continue;
            }
            let x = ch_buf[i];
            st.lines[c].write_and_advance(x);
            let mut wet = st.lines[c].read_frac(d0) * w0 + st.lines[c].read_frac(d1) * w1;
            if let Some(b) = &flt {
                wet = b.tick(wet, &mut st.filter_states[c]);
            }
            let dry_x = if p.sync_dry {
                st.lines[c].read_delay(sync_delay)
            } else {
                x
            };
            ch_buf[i] = dry_x * dry_lin + wet * wet_lin;
        }
        st.phase = (ph0 + dphase).rem_euclid(1.0);
    }
}

pub(super) fn process_harmonizer(
    p: &HarmonizerParams,
    st: &mut HarmonizerState,
    buf: &mut [Vec<f32>],
    lfe_channel: Option<usize>,
) {
    let num_ch = buf.len();
    let frames = buf[0].len();
    if st.lines.len() < num_ch {
        st.lines.resize(num_ch, DelayLine::new(8192));
    }
    if st.filter_states.len() < num_ch {
        st.filter_states.resize(num_ch, [[0.0; 2]; 2]);
    }
    let win_samples = (p.window_size.clamp(64, 4096)) as f32;
    let dry_lin = db_to_lin(p.dry_level_db);
    let wet_lin = db_to_lin(p.wet_level_db);
    let sync_delay = (win_samples * 0.5) as usize;

    let voice_cfg: [(bool, f32, f32, Option<DspBiquad>); 2] = std::array::from_fn(|v| {
        let vp = &p.voices[v];
        let ratio = 2.0f32.powf(vp.pitch_cents.clamp(-2400.0, 2400.0) / 1200.0);
        let dph = (1.0 - ratio) / win_samples;
        let gain = db_to_lin(vp.gain_db);
        let flt = (vp.enable && vp.filter_type > 0).then(|| {
            let ft = match vp.filter_type {
                1 => EqFilterType::LowShelf,
                2 => EqFilterType::Peaking,
                3 => EqFilterType::HighShelf,
                4 => EqFilterType::LowPass,
                5 => EqFilterType::HighPass,
                6 => EqFilterType::BandPass,
                _ => EqFilterType::Notch,
            };
            DspBiquad::design(
                &EqBandParams {
                    filter_type: ft,
                    gain_db: vp.filter_gain_db,
                    freq_hz: vp.filter_freq_hz,
                    q_factor: vp.filter_q,
                    on_off: true,
                },
                RATE,
            )
        });
        (vp.enable, dph, gain, flt)
    });

    for i in 0..frames {
        let ph_v0 = st.phases[0].rem_euclid(1.0);
        let ph_v1 = st.phases[1].rem_euclid(1.0);
        for (c, ch_buf) in buf.iter_mut().enumerate() {
            if lfe_channel == Some(c) && !p.process_lfe {
                continue;
            }
            let x = ch_buf[i];
            st.lines[c].write_and_advance(x);
            let mut wet_sum = 0.0f32;
            for (v, &(en, _, gain, ref flt)) in voice_cfg.iter().enumerate() {
                if !en {
                    continue;
                }
                let ph0 = if v == 0 { ph_v0 } else { ph_v1 };
                let ph1 = (ph0 + 0.5).rem_euclid(1.0);
                let w0 = 0.5 * (1.0 - (2.0 * PI * ph0).cos());
                let w1 = 1.0 - w0;
                let d0 = ph0 * win_samples + 1.0;
                let d1 = ph1 * win_samples + 1.0;
                let mut y = (st.lines[c].read_frac(d0) * w0 + st.lines[c].read_frac(d1) * w1) * gain;
                if let Some(b) = flt {
                    y = b.tick(y, &mut st.filter_states[c][v]);
                }
                wet_sum += y;
            }
            let dry_x = if p.sync_dry {
                st.lines[c].read_delay(sync_delay)
            } else {
                x
            };
            ch_buf[i] = dry_x * dry_lin + wet_sum * wet_lin;
        }
        st.phases[0] = (ph_v0 + voice_cfg[0].1).rem_euclid(1.0);
        st.phases[1] = (ph_v1 + voice_cfg[1].1).rem_euclid(1.0);
    }
}

pub(super) fn process_time_stretch(
    p: &TimeStretchParams,
    lines: &mut Vec<DelayLine>,
    phase: &mut f32,
    buf: &mut [Vec<f32>],
) {
    let num_ch = buf.len();
    let frames = buf[0].len();
    if lines.len() < num_ch {
        lines.resize(num_ch, DelayLine::new(8192));
    }
    let win_samples = (p.window_size.clamp(128, 4096)) as f32;
    let stretch_ratio = if p.time_stretch > 10.0 {
        (p.time_stretch * 0.01).clamp(0.25, 4.0)
    } else if p.time_stretch > 0.01 {
        p.time_stretch.clamp(0.25, 4.0)
    } else {
        1.0
    };
    let pitch_ratio = 2.0f32.powf(p.pitch_shift_cents.clamp(-2400.0, 2400.0) / 1200.0);
    let eff_ratio = (pitch_ratio / stretch_ratio).clamp(0.25, 4.0);
    let dphase = (1.0 - eff_ratio) / win_samples;
    let out_lin = db_to_lin(p.output_gain_db);

    for i in 0..frames {
        let ph0 = phase.rem_euclid(1.0);
        let ph1 = (ph0 + 0.5).rem_euclid(1.0);
        let w0 = 0.5 * (1.0 - (2.0 * PI * ph0).cos());
        let w1 = 1.0 - w0;
        let d0 = ph0 * win_samples + 1.0;
        let d1 = ph1 * win_samples + 1.0;

        for (c, ch_buf) in buf.iter_mut().enumerate() {
            let x = ch_buf[i];
            lines[c].write_and_advance(x);
            let y = if p.enable || (eff_ratio - 1.0).abs() > 1e-4 {
                lines[c].read_frac(d0) * w0 + lines[c].read_frac(d1) * w1
            } else {
                x
            };
            ch_buf[i] = y * out_lin;
        }
        *phase = (ph0 + dphase).rem_euclid(1.0);
    }
}
