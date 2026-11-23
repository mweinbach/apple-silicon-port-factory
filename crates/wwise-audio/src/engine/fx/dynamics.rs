//! Dynamics (Compressor, Expander, Peak Limiter), Meter, and QAnalyzer DSP processors.

use super::params::{DynamicsKind, DynamicsParams, MeterParams};
use crate::engine::math::{db_to_lin, fast_pow_to_db, RATE};

pub(super) fn process_dynamics(
    p: &DynamicsParams,
    env: &mut Vec<f32>,
    buf: &mut [Vec<f32>],
    lfe_channel: Option<usize>,
) {
    let num_ch = buf.len();
    let frames = buf[0].len();
    if env.len() < num_ch {
        env.resize(num_ch, 0.0);
    }
    let att_s = p.attack_or_lookahead_s.max(0.0001);
    let rel_s = p.release_s.max(0.0001);
    let alpha_att = (-1.0 / (att_s * RATE as f32)).exp();
    let alpha_rel = (-1.0 / (rel_s * RATE as f32)).exp();
    let ratio = p.ratio.max(1.0);
    let thresh_db = p.threshold_db;
    let out_lin = db_to_lin(p.output_level_db);

    let gain_for_env = |e: f32| -> f32 {
        let lvl_db = if e <= 1e-6 { -96.3 } else { fast_pow_to_db(e * e) };
        let gr_db = match p.kind {
            DynamicsKind::Compressor | DynamicsKind::PeakLimiter => {
                if lvl_db > thresh_db {
                    (thresh_db - lvl_db) * (1.0 - 1.0 / ratio)
                } else {
                    0.0
                }
            }
            DynamicsKind::Expander => {
                if lvl_db < thresh_db {
                    (lvl_db - thresh_db) * (ratio - 1.0)
                } else {
                    0.0
                }
            }
        };
        db_to_lin(gr_db) * out_lin
    };

    if p.channel_link {
        let mut linked_env = env.first().copied().unwrap_or(0.0);
        for i in 0..frames {
            let mut peak = 0.0f32;
            for (c, ch_buf) in buf.iter().enumerate() {
                if lfe_channel == Some(c) && !p.process_lfe {
                    continue;
                }
                peak = peak.max(ch_buf[i].abs());
            }
            let a = if peak > linked_env { alpha_att } else { alpha_rel };
            linked_env = a * linked_env + (1.0 - a) * peak;
            let g = gain_for_env(linked_env);
            for (c, ch_buf) in buf.iter_mut().enumerate() {
                if lfe_channel == Some(c) && !p.process_lfe {
                    continue;
                }
                ch_buf[i] *= g;
            }
        }
        for e in env.iter_mut() {
            *e = linked_env;
        }
    } else {
        for (c, ch_buf) in buf.iter_mut().enumerate() {
            if lfe_channel == Some(c) && !p.process_lfe {
                continue;
            }
            let e = &mut env[c];
            for s in ch_buf.iter_mut() {
                let x = s.abs();
                let a = if x > *e { alpha_att } else { alpha_rel };
                *e = a * *e + (1.0 - a) * x;
                *s *= gain_for_env(*e);
            }
        }
    }
}

pub(super) fn process_meter(
    p: &MeterParams,
    env_db: &mut f32,
    hold_remaining_s: &mut f32,
    held_peak_db: &mut f32,
    buf: &[Vec<f32>],
) -> Option<(u32, f32)> {
    let num_ch = buf.len();
    let frames = buf[0].len();
    // Compute block Peak or RMS level across channels in dB
    let mut sum_sq = 0.0f32;
    let mut peak = 0.0f32;
    let total_samples = (num_ch * frames).max(1) as f32;
    for ch_buf in buf.iter() {
        for &s in ch_buf.iter() {
            let a = s.abs();
            if a > peak {
                peak = a;
            }
            sum_sq += s * s;
        }
    }
    let block_db = if p.mode == 1 {
        let rms_sq = sum_sq / total_samples;
        if rms_sq <= 1e-12 {
            -96.3
        } else {
            fast_pow_to_db(rms_sq)
        }
    } else if peak <= 1e-6 {
        -96.3
    } else {
        fast_pow_to_db(peak * peak)
    };

    let dt = frames as f32 / RATE as f32;
    if p.infinite_hold {
        if block_db > *env_db {
            *env_db = block_db;
        }
    } else if block_db >= *env_db {
        // Attack toward higher level
        let att = p.attack_s.max(0.0);
        if att <= 1e-5 {
            *env_db = block_db;
        } else {
            let alpha = (-dt / att).exp();
            *env_db = alpha * *env_db + (1.0 - alpha) * block_db;
        }
        *held_peak_db = *env_db;
        *hold_remaining_s = p.hold_s.max(0.0);
    } else if *hold_remaining_s > 0.0 {
        *hold_remaining_s = (*hold_remaining_s - dt).max(0.0);
        *env_db = *held_peak_db;
    } else {
        let rel = p.release_s.max(0.0);
        if rel <= 1e-5 {
            *env_db = block_db;
        } else {
            let alpha = (-dt / rel).exp();
            *env_db = alpha * *env_db + (1.0 - alpha) * block_db;
        }
    }
    let clamped = env_db.clamp(p.min_db, p.max_db);
    *env_db = clamped;
    if p.game_param_id != 0 {
        Some((p.game_param_id, clamped))
    } else {
        None
    }
}

pub(super) fn process_q_analyzer(peak_db: &mut f32, buf: &[Vec<f32>]) {
    let mut peak = 0.0f32;
    for ch_buf in buf.iter() {
        for &s in ch_buf.iter() {
            peak = peak.max(s.abs());
        }
    }
    *peak_db = if peak <= 1e-6 {
        -96.3
    } else {
        fast_pow_to_db(peak * peak)
    };
}
