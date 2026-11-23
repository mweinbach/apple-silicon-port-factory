//! RTPC modulation for Wwise FX plugin parameters.

use super::params::{EqFilterType, FxParams};
use crate::bnk::hirc::Rtpc;
use crate::engine::math::eval_graph;
use crate::engine::params::{accum_value, rtpc_accum, GameState};
use crate::engine::project::Project;

/// Apply an FX object's RTPC curves (`FxBase::rtpcs`) to its parsed [`FxParams`].
pub fn apply_fx_rtpcs(project: &Project, gs: &GameState, rtpcs: &[Rtpc], params: &mut FxParams) {
    if rtpcs.is_empty() {
        return;
    }
    let eval_rtpc = |r: &Rtpc, dst: &mut f32, is_db: bool| {
        if r.rtpc_type != 0 {
            return;
        }
        let x = gs.rtpc_value(project, r.rtpc_id);
        let y = eval_graph(&r.points, r.scaling, x);
        let eff_accum = if (r.scaling == 4 || r.scaling == 3)
            && r.accum == rtpc_accum::ADDITIVE
            && is_db
        {
            rtpc_accum::MULTIPLY
        } else {
            r.accum
        };
        accum_value(dst, y, eff_accum, is_db);
    };

    for r in rtpcs {
        match params {
            FxParams::Gain(p) => match r.param_id {
                0 => eval_rtpc(r, &mut p.fullband_gain_db, true),
                1 => eval_rtpc(r, &mut p.lfe_gain_db, true),
                _ => {}
            },
            FxParams::ParametricEq(p) => match r.param_id {
                0..=14 => {
                    let band = (r.param_id / 5) as usize;
                    let field = r.param_id % 5;
                    if let Some(b) = p.bands.get_mut(band) {
                        match field {
                            0 => {
                                let mut v = b.filter_type as u32 as f32;
                                eval_rtpc(r, &mut v, false);
                                b.filter_type = EqFilterType::from_u32(v.round().max(0.0) as u32);
                            }
                            1 => eval_rtpc(r, &mut b.gain_db, true),
                            2 => eval_rtpc(r, &mut b.freq_hz, false),
                            3 => eval_rtpc(r, &mut b.q_factor, false),
                            4 => {
                                let mut v = if b.on_off { 1.0 } else { 0.0 };
                                eval_rtpc(r, &mut v, false);
                                b.on_off = v >= 0.5;
                            }
                            _ => {}
                        }
                    }
                }
                15 => eval_rtpc(r, &mut p.output_level_db, true),
                _ => {}
            },
            FxParams::Delay(p) => match r.param_id {
                0 => eval_rtpc(r, &mut p.delay_time_s, false),
                1 => eval_rtpc(r, &mut p.feedback, false),
                2 => eval_rtpc(r, &mut p.wet_dry_mix, false),
                3 => eval_rtpc(r, &mut p.output_level_db, true),
                _ => {}
            },
            FxParams::StereoDelay(p) => match r.param_id {
                1 => eval_rtpc(r, &mut p.dry_level_db, true),
                2 => eval_rtpc(r, &mut p.wet_level_db, true),
                3 => eval_rtpc(r, &mut p.front_rear_balance, false),
                9 => eval_rtpc(r, &mut p.channels[0].delay_time_s, false),
                10 => eval_rtpc(r, &mut p.channels[0].feedback_db, true),
                11 => eval_rtpc(r, &mut p.channels[0].crossfeed_db, true),
                12 => eval_rtpc(r, &mut p.channels[0].feedback_db, true),
                13 => eval_rtpc(r, &mut p.channels[1].delay_time_s, false),
                14 => eval_rtpc(r, &mut p.channels[1].feedback_db, true),
                15 => eval_rtpc(r, &mut p.channels[1].crossfeed_db, true),
                16 => eval_rtpc(r, &mut p.channels[1].feedback_db, true),
                _ => {}
            },
            FxParams::Dynamics(p) => match r.param_id {
                0 => eval_rtpc(r, &mut p.threshold_db, true),
                1 => eval_rtpc(r, &mut p.ratio, false),
                2 => eval_rtpc(r, &mut p.attack_or_lookahead_s, false),
                3 => eval_rtpc(r, &mut p.release_s, false),
                4 => eval_rtpc(r, &mut p.output_level_db, true),
                _ => {}
            },
            FxParams::Tremolo(p) => match r.param_id {
                0 | 1 => eval_rtpc(r, &mut p.mod_depth, false),
                2 => eval_rtpc(r, &mut p.lfo_freq_hz, false),
                8 => eval_rtpc(r, &mut p.output_level_db, true),
                _ => {}
            },
            FxParams::Flanger(p) => match r.param_id {
                0 => eval_rtpc(r, &mut p.delay_time_ms, false),
                1 => eval_rtpc(r, &mut p.mod_depth, false),
                2 | 6 => eval_rtpc(r, &mut p.fback_level, false),
                5 | 9 => eval_rtpc(r, &mut p.lfo_freq_hz, false),
                12 => eval_rtpc(r, &mut p.output_level_db, true),
                13 => eval_rtpc(r, &mut p.wet_dry_mix, false),
                _ => {}
            },
            FxParams::GuitarDistortion(p) => match r.param_id {
                60 => {
                    let mut v = p.distortion_type as f32;
                    eval_rtpc(r, &mut v, false);
                    p.distortion_type = v.round().max(0.0) as u32;
                }
                61 => eval_rtpc(r, &mut p.drive, false),
                62 => eval_rtpc(r, &mut p.tone, false),
                63 => eval_rtpc(r, &mut p.rectification, false),
                64 => eval_rtpc(r, &mut p.output_level_db, true),
                65 => eval_rtpc(r, &mut p.wet_dry_mix, false),
                _ => {}
            },
            FxParams::RoomVerb(p) => match r.param_id {
                0 => eval_rtpc(r, &mut p.decay_time_s, false),
                1 => eval_rtpc(r, &mut p.hf_damping, false),
                2 => eval_rtpc(r, &mut p.diffusion, false),
                3 => eval_rtpc(r, &mut p.stereo_width, false),
                60 => eval_rtpc(r, &mut p.er_level_db, true),
                61 => eval_rtpc(r, &mut p.reverb_level_db, true),
                62 => eval_rtpc(r, &mut p.dry_level_db, true),
                _ => {}
            },
            FxParams::MatrixReverb(p) => match r.param_id {
                0 => eval_rtpc(r, &mut p.reverb_time_s, false),
                1 => eval_rtpc(r, &mut p.hf_ratio, false),
                3 => eval_rtpc(r, &mut p.dry_level_db, true),
                4 => eval_rtpc(r, &mut p.wet_level_db, true),
                5 => eval_rtpc(r, &mut p.pre_delay_s, false),
                _ => {}
            },
            FxParams::PitchShifter(p) => match r.param_id {
                3 => eval_rtpc(r, &mut p.dry_level_db, true),
                4 => eval_rtpc(r, &mut p.wet_level_db, true),
                5 => eval_rtpc(r, &mut p.delay_time_ms, false),
                6 => eval_rtpc(r, &mut p.pitch_cents, false),
                8 => eval_rtpc(r, &mut p.filter_gain_db, true),
                9 => eval_rtpc(r, &mut p.filter_freq_hz, false),
                10 => eval_rtpc(r, &mut p.filter_q, false),
                _ => {}
            },
            FxParams::Harmonizer(p) => match r.param_id {
                3 => eval_rtpc(r, &mut p.dry_level_db, true),
                4 => eval_rtpc(r, &mut p.wet_level_db, true),
                7 => eval_rtpc(r, &mut p.voices[0].pitch_cents, false),
                8 => eval_rtpc(r, &mut p.voices[0].gain_db, true),
                10 => eval_rtpc(r, &mut p.voices[0].filter_gain_db, true),
                11 => eval_rtpc(r, &mut p.voices[0].filter_freq_hz, false),
                12 => eval_rtpc(r, &mut p.voices[0].filter_q, false),
                14 => eval_rtpc(r, &mut p.voices[1].pitch_cents, false),
                15 => eval_rtpc(r, &mut p.voices[1].gain_db, true),
                17 => eval_rtpc(r, &mut p.voices[1].filter_gain_db, true),
                18 => eval_rtpc(r, &mut p.voices[1].filter_freq_hz, false),
                19 => eval_rtpc(r, &mut p.voices[1].filter_q, false),
                _ => {}
            },
            FxParams::TimeStretch(p) => match r.param_id {
                1 => eval_rtpc(r, &mut p.time_stretch, false),
                2 => eval_rtpc(r, &mut p.output_gain_db, true),
                4 => eval_rtpc(r, &mut p.pitch_shift_cents, false),
                _ => {}
            },
            FxParams::ConvolutionReverb(p) => match r.param_id {
                0 => eval_rtpc(r, &mut p.pre_delay_ms, false),
                1 => eval_rtpc(r, &mut p.front_rear_delay_ms, false),
                2 => eval_rtpc(r, &mut p.stereo_width, false),
                20 => eval_rtpc(r, &mut p.front_level_db, true),
                21 => eval_rtpc(r, &mut p.rear_level_db, true),
                30 => eval_rtpc(r, &mut p.dry_level_db, true),
                31 => eval_rtpc(r, &mut p.wet_level_db, true),
                34 => eval_rtpc(r, &mut p.threshold_db, true),
                _ => {}
            },
            FxParams::McDspFutzBox(p) => match r.param_id {
                1 => {
                    let mut v = p.dist_mode as f32;
                    eval_rtpc(r, &mut v, false);
                    p.dist_mode = v.round().max(0.0) as u32;
                }
                2 => eval_rtpc(r, &mut p.dist_amount, false),
                3 => eval_rtpc(r, &mut p.dist_intensity, false),
                4 => eval_rtpc(r, &mut p.dist_rectify, false),
                40 => eval_rtpc(r, &mut p.input_gain_db, true),
                41 => eval_rtpc(r, &mut p.output_gain_db, true),
                42 => eval_rtpc(r, &mut p.balance_pct, false),
                52 => eval_rtpc(r, &mut p.lpf_freq_hz, false),
                55 => eval_rtpc(r, &mut p.hpf_freq_hz, false),
                62 => eval_rtpc(r, &mut p.eq_freq_hz, false),
                63 => eval_rtpc(r, &mut p.eq_gain_db, true),
                _ => {}
            },
            FxParams::IzotopeTrash(p) => match r.param_id {
                0 => eval_rtpc(r, &mut p.stages[0].input_gain_db, true),
                1 => eval_rtpc(r, &mut p.stages[0].overdrive, false),
                2 => eval_rtpc(r, &mut p.stages[0].trash, false),
                3 => eval_rtpc(r, &mut p.stages[0].mix_pct, false),
                4 => eval_rtpc(r, &mut p.stages[0].output_gain_db, true),
                6 => eval_rtpc(r, &mut p.stages[1].input_gain_db, true),
                7 => eval_rtpc(r, &mut p.stages[1].overdrive, false),
                8 => eval_rtpc(r, &mut p.stages[1].trash, false),
                9 => eval_rtpc(r, &mut p.stages[1].mix_pct, false),
                10 => eval_rtpc(r, &mut p.stages[1].output_gain_db, true),
                _ => {}
            },
            FxParams::MasteringSuite(p) => match r.param_id {
                200 => eval_rtpc(r, &mut p.stereo_width, false),
                400 => eval_rtpc(r, &mut p.limiter_threshold_db, true),
                403 => eval_rtpc(r, &mut p.limiter_ceiling_db, true),
                404 => eval_rtpc(r, &mut p.master_output_gain_db, true),
                _ => {}
            },
            FxParams::Meter(_) | FxParams::QAnalyzer(_) | FxParams::Passthrough { .. } => {}
        }
    }
}
