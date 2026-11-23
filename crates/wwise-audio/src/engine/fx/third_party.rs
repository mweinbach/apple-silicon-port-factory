//! Third-party & mastering DSP processors: McDSP FutzBox, iZotope Trash, and Mastering Suite.

use super::biquad::DspBiquad;
use super::params::{EqBandParams, EqFilterType, IzotopeTrashParams, MasteringSuiteParams, McDspFutzBoxParams};
use crate::engine::math::{db_to_lin, fast_pow_to_db, RATE};

#[derive(Debug, Clone, Default)]
pub(super) struct FutzBoxState {
    hpf_states: Vec<[f32; 2]>,
    lpf_states: Vec<[f32; 2]>,
    sim_states: Vec<[f32; 2]>,
    eq_states: Vec<[f32; 2]>,
    gate_env: Vec<f32>,
    decim_hold: Vec<f32>,
    decim_phase: f32,
}

#[derive(Debug, Clone, Default)]
pub(super) struct MasteringSuiteState {
    eq_states: Vec<[[f32; 2]; 6]>,
    comp_env: f32,
    lim_env: f32,
}

pub(super) fn process_mcdsp_futzbox(p: &McDspFutzBoxParams, st: &mut FutzBoxState, buf: &mut [Vec<f32>]) {
    let num_ch = buf.len();
    let frames = buf[0].len();
    if st.hpf_states.len() < num_ch {
        st.hpf_states.resize(num_ch, [0.0; 2]);
        st.lpf_states.resize(num_ch, [0.0; 2]);
        st.sim_states.resize(num_ch, [0.0; 2]);
        st.eq_states.resize(num_ch, [0.0; 2]);
        st.gate_env.resize(num_ch, 0.0);
        st.decim_hold.resize(num_ch, 0.0);
    }
    let in_lin = db_to_lin(p.input_gain_db);
    let out_lin = db_to_lin(p.output_gain_db);
    let wet = (p.balance_pct * 0.01).clamp(0.0, 1.0);
    let dry = 1.0 - wet;

    let hpf = p.filter_enable.then(|| {
        DspBiquad::design(
            &EqBandParams {
                filter_type: EqFilterType::HighPass,
                gain_db: 0.0,
                freq_hz: p.hpf_freq_hz.clamp(20.0, 8000.0),
                q_factor: (0.707 + p.hpf_q.clamp(-12.0, 12.0) * 0.05).clamp(0.2, 10.0),
                on_off: true,
            },
            RATE,
        )
    });
    let lpf = p.filter_enable.then(|| {
        DspBiquad::design(
            &EqBandParams {
                filter_type: EqFilterType::LowPass,
                gain_db: 0.0,
                freq_hz: p.lpf_freq_hz.clamp(200.0, 20000.0),
                q_factor: (0.707 + p.lpf_q.clamp(-12.0, 12.0) * 0.05).clamp(0.2, 10.0),
                on_off: true,
            },
            RATE,
        )
    });
    // SIM synthetic speaker/telephone bandpass resonance
    let sim_bq = p.sim_enable.then(|| {
        let tune_scale = 2.0f32.powf(p.sim_tuning.clamp(-24.0, 24.0) / 12.0);
        let base_freq = (900.0 + (p.sim_index % 16) as f32 * 140.0) * tune_scale;
        DspBiquad::design(
            &EqBandParams {
                filter_type: EqFilterType::BandPass,
                gain_db: 0.0,
                freq_hz: base_freq.clamp(300.0, 6000.0),
                q_factor: 1.4,
                on_off: true,
            },
            RATE,
        )
    });
    let eq_bq = p.eq_enable.then(|| {
        let ft = match p.eq_mode {
            0 => EqFilterType::LowShelf,
            2 => EqFilterType::HighShelf,
            3 => EqFilterType::BandPass,
            4 => EqFilterType::Notch,
            _ => EqFilterType::Peaking,
        };
        DspBiquad::design(
            &EqBandParams {
                filter_type: ft,
                gain_db: p.eq_gain_db,
                freq_hz: p.eq_freq_hz.clamp(40.0, 18000.0),
                q_factor: p.eq_q.abs().clamp(0.2, 12.0),
                on_off: true,
            },
            RATE,
        )
    });

    let drive = if p.dist_enable {
        1.0 + (p.dist_amount * 0.01).clamp(0.0, 1.0) * 12.0
            + (p.dist_intensity * 0.01).clamp(0.0, 1.0) * 12.0
    } else {
        1.0
    };
    let rect = (p.dist_rectify * 0.01).clamp(0.0, 1.0);
    let gate_thresh = db_to_lin(p.gate_thresh_db);
    let gate_range = db_to_lin(p.gate_range_db.min(0.0));
    let gate_att = (-1.0 / ((p.gate_attack_ms.max(0.1) * 0.001) * RATE as f32)).exp();
    let gate_rel = (-1.0 / ((p.gate_release_ms.max(1.0) * 0.001) * RATE as f32)).exp();
    let decim_step = if p.sim_enable { 0.5f32 } else { 1.0f32 };

    for i in 0..frames {
        st.decim_phase += decim_step;
        let sample_tick = st.decim_phase >= 1.0;
        if sample_tick {
            st.decim_phase -= 1.0;
        }
        for (c, ch_buf) in buf.iter_mut().enumerate() {
            let dry_x = ch_buf[i];
            let mut x = dry_x * in_lin;
            if p.gate_enable {
                let a = if x.abs() > st.gate_env[c] { gate_att } else { gate_rel };
                st.gate_env[c] = a * st.gate_env[c] + (1.0 - a) * x.abs();
                if st.gate_env[c] < gate_thresh {
                    x *= gate_range;
                }
            }
            if let Some(b) = &hpf {
                x = b.tick(x, &mut st.hpf_states[c]);
            }
            if let Some(b) = &lpf {
                x = b.tick(x, &mut st.lpf_states[c]);
            }
            if let Some(b) = &sim_bq {
                let resonated = b.tick(x, &mut st.sim_states[c]) * 1.6;
                if sample_tick {
                    // Quantize to 12-bit lo-fi step when SIM is active
                    st.decim_hold[c] = (resonated * 2048.0).round() / 2048.0;
                }
                x = 0.35 * x + 0.65 * st.decim_hold[c];
            }
            if p.dist_enable {
                let d = x * drive;
                let mut shaped = match p.dist_mode {
                    0 => d.tanh(),
                    1 => (d * 1.4).tanh(),
                    2 => d.clamp(-0.85, 0.85),
                    _ => d / (1.0 + d.abs()),
                };
                if rect > 0.0 {
                    shaped = shaped * (1.0 - rect) + shaped.abs() * rect;
                }
                x = shaped;
            }
            if let Some(b) = &eq_bq {
                x = b.tick(x, &mut st.eq_states[c]);
            }
            ch_buf[i] = dry_x * dry + x * out_lin * wet;
        }
    }
}

pub(super) fn process_izotope_trash(
    p: &IzotopeTrashParams,
    filter_states: &mut Vec<[[f32; 2]; 2]>,
    buf: &mut [Vec<f32>],
) {
    let num_ch = buf.len();
    if filter_states.len() < num_ch {
        filter_states.resize(num_ch, [[0.0; 2]; 2]);
    }
    let post_lpf = DspBiquad::design(
        &EqBandParams {
            filter_type: EqFilterType::LowPass,
            gain_db: 0.0,
            freq_hz: p.filter_freq_hz.clamp(500.0, 20000.0),
            q_factor: p.filter_q.clamp(0.2, 10.0),
            on_off: true,
        },
        RATE,
    );
    let dc_hpf = DspBiquad::design(
        &EqBandParams {
            filter_type: EqFilterType::HighPass,
            gain_db: 0.0,
            freq_hz: 25.0,
            q_factor: 0.707,
            on_off: true,
        },
        RATE,
    );

    for (c, ch_buf) in buf.iter_mut().enumerate() {
        for s in ch_buf.iter_mut() {
            let mut x = *s;
            for st in &p.stages {
                let mix = (st.mix_pct * 0.01).clamp(0.0, 1.0);
                if mix <= 1e-5 {
                    continue;
                }
                let in_g = db_to_lin(st.input_gain_db);
                let out_g = db_to_lin(st.output_gain_db);
                let drive = (1.0 + st.overdrive.clamp(0.0, 30.0) * 1.2).max(0.1);
                let trash = st.trash.clamp(0.0, 1.0);
                let u = x * in_g * drive;
                let shaped = match st.algo_type % 4 {
                    0 => {
                        // Warm tube / tanh + asymmetric trash harmonic
                        let sym = u.tanh();
                        let asym = ((u + trash * 0.4).tanh() - (trash * 0.4).tanh()).clamp(-1.0, 1.0);
                        sym * (1.0 - trash) + asym * trash
                    }
                    1 => {
                        // Soft-knee algebraic saturator + foldback
                        let soft = u / (1.0 + u.abs());
                        let fold = (u * (1.0 + trash)).sin();
                        soft * (1.0 - 0.5 * trash) + fold * (0.5 * trash)
                    }
                    2 => {
                        // Fuzz / rectifier
                        let fz = (u * 1.5).tanh();
                        fz * (1.0 - 0.4 * trash) + fz.abs() * (0.4 * trash)
                    }
                    _ => u.clamp(-1.0, 1.0),
                };
                x = x * (1.0 - mix) + shaped * out_g * mix;
            }
            x = dc_hpf.tick(x, &mut filter_states[c][0]);
            x = post_lpf.tick(x, &mut filter_states[c][1]);
            *s = x.clamp(-4.0, 4.0);
        }
    }
}

pub(super) fn process_mastering_suite(
    p: &MasteringSuiteParams,
    st: &mut MasteringSuiteState,
    buf: &mut [Vec<f32>],
) {
    let num_ch = buf.len();
    let frames = buf[0].len();
    if st.eq_states.len() < num_ch {
        st.eq_states.resize(num_ch, [[0.0; 2]; 6]);
    }
    let eq_biquads: [DspBiquad; 6] = std::array::from_fn(|b| DspBiquad::design(&p.eq_bands[b], RATE));
    let comp_att = (-1.0 / (p.comp_attack_s.max(0.0001) * RATE as f32)).exp();
    let comp_rel = (-1.0 / (p.comp_release_s.max(0.001) * RATE as f32)).exp();
    let comp_ratio = p.comp_ratio.max(1.0);
    let comp_makeup = db_to_lin(p.comp_makeup_db);
    let lim_rel = (-1.0 / (p.limiter_release_s.max(0.001) * RATE as f32)).exp();
    let lim_thresh = db_to_lin(p.limiter_threshold_db);
    let lim_ceil = db_to_lin(p.limiter_ceiling_db);
    let master_gain = db_to_lin(p.master_output_gain_db);
    let width = if p.width_enable {
        (p.stereo_width.clamp(-100.0, 100.0) + 100.0) * 0.01
    } else {
        1.0
    };

    for i in 0..frames {
        // 1. 6-band Parametric EQ
        if p.eq_enable {
            for (c, ch_buf) in buf.iter_mut().enumerate() {
                let mut s = ch_buf[i];
                for b in 0..6 {
                    if p.eq_bands[b].on_off {
                        s = eq_biquads[b].tick(s, &mut st.eq_states[c][b]);
                    }
                }
                ch_buf[i] = s;
            }
        }

        // 2. Linked Compressor
        if p.comp_enable {
            let mut peak = 0.0f32;
            for ch_buf in buf.iter() {
                peak = peak.max(ch_buf[i].abs());
            }
            let a = if peak > st.comp_env { comp_att } else { comp_rel };
            st.comp_env = a * st.comp_env + (1.0 - a) * peak;
            let env_db = if st.comp_env <= 1e-6 {
                -96.3
            } else {
                fast_pow_to_db(st.comp_env * st.comp_env)
            };
            let gr_db = if env_db > p.comp_threshold_db {
                (p.comp_threshold_db - env_db) * (1.0 - 1.0 / comp_ratio)
            } else {
                0.0
            };
            let g = db_to_lin(gr_db) * comp_makeup;
            for ch_buf in buf.iter_mut() {
                ch_buf[i] *= g;
            }
        }

        // 3. Stereo Width
        if p.width_enable && num_ch >= 2 {
            let l = buf[0][i];
            let r = buf[1][i];
            let mid = 0.5 * (l + r);
            let side = 0.5 * (l - r) * width;
            buf[0][i] = mid + side;
            buf[1][i] = mid - side;
        }

        // 4. Brickwall Limiter + Master Output Gain
        let mut out_peak = 0.0f32;
        for ch_buf in buf.iter_mut() {
            ch_buf[i] *= master_gain;
            out_peak = out_peak.max(ch_buf[i].abs());
        }
        if p.limiter_enable {
            if out_peak > st.lim_env {
                st.lim_env = out_peak;
            } else {
                st.lim_env = lim_rel * st.lim_env + (1.0 - lim_rel) * out_peak;
            }
            let lim_g = if st.lim_env > lim_thresh && st.lim_env > 1e-6 {
                (lim_ceil / st.lim_env).min(1.0)
            } else if lim_thresh > 1e-6 {
                (lim_ceil / lim_thresh).min(1.0)
            } else {
                1.0
            };
            for ch_buf in buf.iter_mut() {
                ch_buf[i] = (ch_buf[i] * lim_g).clamp(-lim_ceil.max(0.01), lim_ceil.max(0.01));
            }
        }
    }
}
