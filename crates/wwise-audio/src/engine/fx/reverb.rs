//! Reverb DSP processors: Matrix Reverb, RoomVerb, and Convolution Reverb (partitioned FFT + FDN fallback).

use super::biquad::{DelayLine, DspBiquad};
use super::params::{ConvolutionReverbParams, MatrixReverbParams, RoomVerbParams};
use crate::engine::convolver::{PartitionedConvolver, PreprocessedIr};
use crate::engine::math::{db_to_lin, RATE};
use crate::engine::project::Project;

#[derive(Debug, Clone)]
pub(super) struct ReverbState {
    predelay: DelayLine,
    er_line: DelayLine,
    fdn_lines: [DelayLine; 4],
    fdn_lp: [f32; 4],
    tone_states: Vec<[[f32; 2]; 3]>,
}

impl ReverbState {
    pub(super) fn new() -> Self {
        // Prime FDN delay lengths (~29.7 ms, 37.1 ms, 41.1 ms, 43.7 ms at 48 kHz)
        let fdn_lines = [
            DelayLine::new(1427),
            DelayLine::new(1783),
            DelayLine::new(1973),
            DelayLine::new(2099),
        ];
        Self {
            predelay: DelayLine::new((0.5 * RATE as f32) as usize + 256),
            er_line: DelayLine::new((0.15 * RATE as f32) as usize + 256),
            fdn_lines,
            fdn_lp: [0.0; 4],
            tone_states: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct ConvolutionReverbState {
    pub(super) ir_media_id: Option<u32>,
    pub(super) convolver: Option<Box<PartitionedConvolver>>,
    predelay_ch: Vec<DelayLine>,
    predelay: DelayLine,
    er_line: DelayLine,
    fdn_lines: [DelayLine; 4],
    fdn_lp: [f32; 4],
    rear_delay: DelayLine,
}

impl ConvolutionReverbState {
    pub(super) fn new() -> Self {
        Self {
            ir_media_id: None,
            convolver: None,
            predelay_ch: Vec::new(),
            predelay: DelayLine::new((0.5 * RATE as f32) as usize + 256),
            er_line: DelayLine::new((0.15 * RATE as f32) as usize + 256),
            fdn_lines: [
                DelayLine::new(1303),
                DelayLine::new(1699),
                DelayLine::new(1999),
                DelayLine::new(2311),
            ],
            fdn_lp: [0.0; 4],
            rear_delay: DelayLine::new((0.15 * RATE as f32) as usize + 256),
        }
    }
}

pub(super) fn process_matrix_reverb(
    p: &MatrixReverbParams,
    st: &mut ReverbState,
    buf: &mut [Vec<f32>],
    lfe_channel: Option<usize>,
) {
    let num_ch = buf.len();
    let frames = buf[0].len();
    let pre_samples = ((p.pre_delay_s.clamp(0.0, 0.45) * RATE as f32).round() as usize).max(1);
    let rt60 = p.reverb_time_s.clamp(0.05, 20.0);
    let damp = (1.0 - (1.0 / p.hf_ratio.clamp(1.0, 10.0))).clamp(0.05, 0.85);
    let dry_lin = db_to_lin(p.dry_level_db);
    let wet_lin = db_to_lin(p.wet_level_db) * 0.35;

    let lens = [
        st.fdn_lines[0].buf.len() - 1,
        st.fdn_lines[1].buf.len() - 1,
        st.fdn_lines[2].buf.len() - 1,
        st.fdn_lines[3].buf.len() - 1,
    ];
    let gains = lens.map(|l| 10.0f32.powf(-3.0 * (l as f32 / RATE as f32) / rt60).clamp(0.0, 0.985));

    for i in 0..frames {
        let mut mono_in = 0.0f32;
        for (c, ch_buf) in buf.iter().enumerate() {
            if lfe_channel == Some(c) && !p.process_lfe {
                continue;
            }
            mono_in += ch_buf[i];
        }
        mono_in /= num_ch.max(1) as f32;
        let pre_out = st.predelay.read_delay(pre_samples);
        st.predelay.write_and_advance(mono_in);

        let mut y = [0.0f32; 4];
        for k in 0..4 {
            let raw = st.fdn_lines[k].read_delay(lens[k]);
            st.fdn_lp[k] = st.fdn_lp[k] * damp + raw * (1.0 - damp);
            y[k] = st.fdn_lp[k] * gains[k];
        }
        // Householder feedback matrix: M * y = y - (2/4) * sum(y)
        let sum_y = 0.5 * (y[0] + y[1] + y[2] + y[3]);
        for k in 0..4 {
            let fb = sum_y - y[k];
            st.fdn_lines[k].write_and_advance((pre_out + fb).clamp(-8.0, 8.0));
        }
        let wet_l = (y[0] + y[2]) * wet_lin;
        let wet_r = (y[1] + y[3]) * wet_lin;
        for (c, ch_buf) in buf.iter_mut().enumerate() {
            if lfe_channel == Some(c) && !p.process_lfe {
                continue;
            }
            let w = if c % 2 == 0 { wet_l } else { wet_r };
            ch_buf[i] = ch_buf[i] * dry_lin + w;
        }
    }
}

pub(super) fn process_room_verb(
    p: &RoomVerbParams,
    st: &mut ReverbState,
    buf: &mut [Vec<f32>],
    lfe_channel: Option<usize>,
) {
    let num_ch = buf.len();
    let frames = buf[0].len();
    if st.tone_states.len() < num_ch {
        st.tone_states.resize(num_ch, [[0.0; 2]; 3]);
    }
    let pre_samples = ((p.reverb_delay_ms.clamp(0.0, 400.0) * 0.001 * RATE as f32).round() as usize).max(1);
    let rt60 = p.decay_time_s.clamp(0.05, 20.0);
    let damp = (p.hf_damping / 10.0).clamp(0.05, 0.85);
    let dry_lin = db_to_lin(p.dry_level_db);
    let er_lin = if p.enable_early_reflections {
        db_to_lin(p.er_level_db) * 0.35
    } else {
        0.0
    };
    let rev_lin = db_to_lin(p.reverb_level_db) * 0.35;
    let front_lin = db_to_lin(p.front_level_db);
    let width = (p.stereo_width / 180.0).clamp(0.0, 1.0);

    let lens = [
        st.fdn_lines[0].buf.len() - 1,
        st.fdn_lines[1].buf.len() - 1,
        st.fdn_lines[2].buf.len() - 1,
        st.fdn_lines[3].buf.len() - 1,
    ];
    let gains = lens.map(|l| 10.0f32.powf(-3.0 * (l as f32 / RATE as f32) / rt60).clamp(0.0, 0.985));
    let er_taps = [347usize, 683, 1103, 1597];
    let tone_bq = [
        DspBiquad::design(&p.filter_bands[0], RATE),
        DspBiquad::design(&p.filter_bands[1], RATE),
        DspBiquad::design(&p.filter_bands[2], RATE),
    ];

    for i in 0..frames {
        let mut mono_in = 0.0f32;
        for (c, ch_buf) in buf.iter().enumerate() {
            if lfe_channel == Some(c) {
                continue;
            }
            mono_in += ch_buf[i];
        }
        mono_in /= num_ch.max(1) as f32;

        // Early reflections tap network
        let er_l = if er_lin > 0.0 {
            (st.er_line.read_delay(er_taps[0]) * 0.7 + st.er_line.read_delay(er_taps[2]) * 0.5) * er_lin
        } else {
            0.0
        };
        let er_r = if er_lin > 0.0 {
            (st.er_line.read_delay(er_taps[1]) * 0.7 + st.er_line.read_delay(er_taps[3]) * 0.5) * er_lin
        } else {
            0.0
        };
        st.er_line.write_and_advance(mono_in);

        // Pre-delay + FDN reverb tank
        let pre_out = st.predelay.read_delay(pre_samples);
        st.predelay.write_and_advance(mono_in);

        let mut y = [0.0f32; 4];
        for k in 0..4 {
            let raw = st.fdn_lines[k].read_delay(lens[k]);
            st.fdn_lp[k] = st.fdn_lp[k] * damp + raw * (1.0 - damp);
            y[k] = st.fdn_lp[k] * gains[k];
        }
        let sum_y = 0.5 * (y[0] + y[1] + y[2] + y[3]);
        for k in 0..4 {
            let fb = sum_y - y[k];
            st.fdn_lines[k].write_and_advance((pre_out + fb).clamp(-8.0, 8.0));
        }
        let rev_l = (y[0] + y[2]) * rev_lin;
        let rev_r = (y[1] + y[3]) * rev_lin;
        let mid = 0.5 * ((er_l + rev_l) + (er_r + rev_r));
        let side = 0.5 * ((er_l + rev_l) - (er_r + rev_r)) * width;
        let wet_l = (mid + side) * front_lin;
        let wet_r = (mid - side) * front_lin;

        for (c, ch_buf) in buf.iter_mut().enumerate() {
            if lfe_channel == Some(c) {
                continue;
            }
            let mut w = if c % 2 == 0 { wet_l } else { wet_r };
            if p.enable_tone_controls {
                for b in 0..3 {
                    if p.filter_bands[b].on_off {
                        w = tone_bq[b].tick(w, &mut st.tone_states[c][b]);
                    }
                }
            }
            ch_buf[i] = ch_buf[i] * dry_lin + w;
        }
    }
}

pub(super) fn process_convolution_reverb(
    p: &ConvolutionReverbParams,
    st: &mut ConvolutionReverbState,
    project: &Project,
    buf: &mut [Vec<f32>],
    lfe_channel: Option<usize>,
) {
    let num_ch = buf.len();
    let frames = buf[0].len();

    // Lazy IR hydration: banks carrying DIDX IR media (e.g. Global.bnk) may load after
    // Init.bnk creates the bus/FX object.
    if st.convolver.is_none() {
        if let Some(mid) = st.ir_media_id {
            if let Some(bytes) = project.read_media_by_id(mid) {
                if let Ok(ir) = PreprocessedIr::parse(&bytes) {
                    st.convolver = Some(Box::new(PartitionedConvolver::new(ir)));
                }
            }
        }
    }

    let dry_lin = db_to_lin(p.dry_level_db);
    let front_lin = db_to_lin(p.front_level_db);
    let rear_lin = db_to_lin(p.rear_level_db);
    let center_lin = db_to_lin(p.center_level_db);
    let width = (p.stereo_width / 180.0).clamp(0.0, 1.0);

    if let Some(conv) = st.convolver.as_deref_mut() {
        let ir_ch = conv.num_channels();
        let pre_samples = (p.pre_delay_ms.clamp(0.0, 400.0) * 0.001 * RATE as f32).round() as usize;
        let wet_lin = db_to_lin(p.wet_level_db);
        let in_ctr_lin = db_to_lin(p.input_center_level_db);
        let in_lfe_lin = db_to_lin(p.input_lfe_level_db);

        if pre_samples > 0 && st.predelay_ch.len() < ir_ch {
            let max_pre = (0.5 * RATE as f32) as usize + 256;
            st.predelay_ch.resize_with(ir_ch, || DelayLine::new(max_pre));
        }

        let non_lfe: Vec<usize> = (0..num_ch).filter(|&c| lfe_channel != Some(c)).collect();
        let mut wet_in = vec![vec![0.0f32; frames]; ir_ch];
        if !non_lfe.is_empty() {
            for ic in 0..ir_ch {
                let dst = &mut wet_in[ic];
                if p.algo_type == 0 && non_lfe.len() != ir_ch {
                    if ir_ch == 1 {
                        let scale = 1.0 / (non_lfe.len() as f32);
                        for &sc in &non_lfe {
                            for i in 0..frames {
                                dst[i] += buf[sc][i] * scale;
                            }
                        }
                    } else if non_lfe.len() == 2 && ir_ch == 6 {
                        // Stereo input -> 5.1 IR (L, R, C, LFE, RL, RR)
                        let (l_ch, r_ch) = (non_lfe[0], non_lfe[1]);
                        match ic {
                            0 | 4 => dst.copy_from_slice(&buf[l_ch][..frames]),
                            1 | 5 => dst.copy_from_slice(&buf[r_ch][..frames]),
                            2 => {
                                let g = 0.5 * in_ctr_lin;
                                for i in 0..frames {
                                    dst[i] = (buf[l_ch][i] + buf[r_ch][i]) * g;
                                }
                            }
                            3 => {
                                if let Some(lc) = lfe_channel {
                                    for i in 0..frames {
                                        dst[i] = buf[lc][i] * in_lfe_lin;
                                    }
                                }
                            }
                            _ => {}
                        }
                    } else {
                        let sc = non_lfe[ic % non_lfe.len()];
                        dst.copy_from_slice(&buf[sc][..frames]);
                    }
                } else {
                    let sc = non_lfe[ic % non_lfe.len()];
                    dst.copy_from_slice(&buf[sc][..frames]);
                }
                if pre_samples > 0 {
                    let dl = &mut st.predelay_ch[ic];
                    for s in dst.iter_mut() {
                        let d = dl.read_delay(pre_samples);
                        dl.write_and_advance(*s);
                        *s = d;
                    }
                }
            }
        }

        let mut wet_out = vec![vec![0.0f32; frames]; ir_ch];
        conv.process_channels(&wet_in, &mut wet_out, frames);

        if non_lfe.len() == 2 && ir_ch >= 2 {
            let (l_ch, r_ch) = (non_lfe[0], non_lfe[1]);
            for i in 0..frames {
                let mut raw_l = wet_out[0][i] * front_lin;
                let mut raw_r = wet_out[1][i] * front_lin;
                if ir_ch == 4 {
                    raw_l += wet_out[2][i] * rear_lin * 0.707_106_77;
                    raw_r += wet_out[3][i] * rear_lin * 0.707_106_77;
                } else if ir_ch >= 6 {
                    let c_term = wet_out[2][i] * center_lin * 0.707_106_77;
                    raw_l += c_term + wet_out[4][i] * rear_lin * 0.707_106_77;
                    raw_r += c_term + wet_out[5][i] * rear_lin * 0.707_106_77;
                }
                let mid = 0.5 * (raw_l + raw_r);
                let side = 0.5 * (raw_l - raw_r) * width;
                buf[l_ch][i] = buf[l_ch][i] * dry_lin + (mid + side) * wet_lin;
                buf[r_ch][i] = buf[r_ch][i] * dry_lin + (mid - side) * wet_lin;
            }
        } else if non_lfe.len() == 1 {
            let ch0 = non_lfe[0];
            let scale = front_lin * wet_lin / (ir_ch as f32);
            for i in 0..frames {
                let sum: f32 = wet_out.iter().map(|w| w[i]).sum();
                buf[ch0][i] = buf[ch0][i] * dry_lin + sum * scale;
            }
        } else {
            for (idx, &c) in non_lfe.iter().enumerate() {
                let ch_gain = if idx < 2 {
                    front_lin
                } else if idx == 2 && non_lfe.len() >= 5 {
                    center_lin
                } else {
                    rear_lin
                } * wet_lin;
                let w_ch = &wet_out[idx % ir_ch];
                for i in 0..frames {
                    buf[c][i] = buf[c][i] * dry_lin + w_ch[i] * ch_gain;
                }
            }
        }
        return;
    }

    // Fallback parameterized FDN when no IR media asset is attached/loaded.
    let pre_samples = ((p.pre_delay_ms.clamp(0.0, 400.0) * 0.001 * RATE as f32).round() as usize).max(1);
    let rear_samples = ((p.front_rear_delay_ms.clamp(0.0, 120.0) * 0.001 * RATE as f32).round() as usize).max(1);
    let wet_lin = db_to_lin(p.wet_level_db) * 0.35;

    // Multi-tap early-reflection FIR + FDN impulse response model
    let er_taps_l = [(193usize, 0.65f32), (467, -0.45), (853, 0.35), (1361, -0.25)];
    let er_taps_r = [(241usize, 0.60f32), (557, -0.42), (977, 0.32), (1489, -0.22)];
    let lens = [
        st.fdn_lines[0].buf.len() - 1,
        st.fdn_lines[1].buf.len() - 1,
        st.fdn_lines[2].buf.len() - 1,
        st.fdn_lines[3].buf.len() - 1,
    ];
    let rt60 = 1.6f32;
    let damp = 0.32f32;
    let gains = lens.map(|l| 10.0f32.powf(-3.0 * (l as f32 / RATE as f32) / rt60).clamp(0.0, 0.985));

    for i in 0..frames {
        let mut mono_in = 0.0f32;
        for (c, ch_buf) in buf.iter().enumerate() {
            if lfe_channel == Some(c) {
                continue;
            }
            mono_in += ch_buf[i];
        }
        mono_in /= num_ch.max(1) as f32;

        let pre_out = st.predelay.read_delay(pre_samples);
        st.predelay.write_and_advance(mono_in);

        let mut er_l = 0.0f32;
        let mut er_r = 0.0f32;
        for &(t, g) in &er_taps_l {
            er_l += st.er_line.read_delay(t) * g;
        }
        for &(t, g) in &er_taps_r {
            er_r += st.er_line.read_delay(t) * g;
        }
        st.er_line.write_and_advance(pre_out);

        let mut y = [0.0f32; 4];
        for k in 0..4 {
            let raw = st.fdn_lines[k].read_delay(lens[k]);
            st.fdn_lp[k] = st.fdn_lp[k] * damp + raw * (1.0 - damp);
            y[k] = st.fdn_lp[k] * gains[k];
        }
        let sum_y = 0.5 * (y[0] + y[1] + y[2] + y[3]);
        for k in 0..4 {
            let fb = sum_y - y[k];
            st.fdn_lines[k].write_and_advance((pre_out + fb).clamp(-8.0, 8.0));
        }

        let rear_out = st.rear_delay.read_delay(rear_samples);
        st.rear_delay.write_and_advance(0.5 * (y[2] + y[3]));

        let raw_l = (er_l + y[0] + y[2]) * front_lin + rear_out * rear_lin * 0.5;
        let raw_r = (er_r + y[1] + y[3]) * front_lin - rear_out * rear_lin * 0.5;
        let mid = 0.5 * (raw_l + raw_r);
        let side = 0.5 * (raw_l - raw_r) * width;
        let wet_l = (mid + side) * wet_lin;
        let wet_r = (mid - side) * wet_lin;

        for (c, ch_buf) in buf.iter_mut().enumerate() {
            if lfe_channel == Some(c) {
                continue;
            }
            let w = if c % 2 == 0 { wet_l } else { wet_r };
            ch_buf[i] = ch_buf[i] * dry_lin + w;
        }
    }
}
