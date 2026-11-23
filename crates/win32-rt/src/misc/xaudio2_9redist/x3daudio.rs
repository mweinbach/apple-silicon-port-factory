//! X3DAudio math, 20-byte handle codec, attenuation curves, cones, Doppler, and speaker panning
//! backed by `crate::audio::format` 3D vector primitives.

use super::mixer::{
    E_INVALIDARG, E_POINTER, S_OK, XAUDIO2_MAX_FREQ_RATIO, XAUDIO2_MIN_FREQ_RATIO,
};
use crate::audio::format::{
    vec3_add, vec3_cross, vec3_dot, vec3_len, vec3_normalize, vec3_scale, vec3_sub, Vec3,
    SPEAKER_BACK_CENTER, SPEAKER_BACK_LEFT, SPEAKER_BACK_RIGHT, SPEAKER_FRONT_CENTER,
    SPEAKER_FRONT_LEFT, SPEAKER_FRONT_LEFT_OF_CENTER, SPEAKER_FRONT_RIGHT,
    SPEAKER_FRONT_RIGHT_OF_CENTER, SPEAKER_LOW_FREQUENCY, SPEAKER_SIDE_LEFT, SPEAKER_SIDE_RIGHT,
};
use crate::strings::{rd, wr};
use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_4, PI, TAU};
use std::ffi::c_void;

// ============================================================================
// Constants & ABI Structures
// ============================================================================

const X3DAUDIO_HANDLE_MAGIC: u32 = 0x5833_4441; // "X3DA"
pub(super) const X3DAUDIO_SPEED_OF_SOUND: f32 = 343.5;

pub(super) const X3DAUDIO_CALCULATE_MATRIX: u32 = 0x0000_0001;
const X3DAUDIO_CALCULATE_DELAY: u32 = 0x0000_0002;
pub(super) const X3DAUDIO_CALCULATE_LPF_DIRECT: u32 = 0x0000_0004;
const X3DAUDIO_CALCULATE_LPF_REVERB: u32 = 0x0000_0008;
const X3DAUDIO_CALCULATE_REVERB: u32 = 0x0000_0010;
pub(super) const X3DAUDIO_CALCULATE_DOPPLER: u32 = 0x0000_0020;
pub(super) const X3DAUDIO_CALCULATE_EMITTER_ANGLE: u32 = 0x0000_0040;
const X3DAUDIO_CALCULATE_ZEROCENTER: u32 = 0x0001_0000;
const X3DAUDIO_CALCULATE_REDIRECT_TO_LFE: u32 = 0x0002_0000;

pub(super) type X3DAudioVector = Vec3;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub(super) struct X3DAudioDistanceCurvePoint {
    pub(super) distance: f32,
    pub(super) dsp_setting: f32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub(super) struct X3DAudioDistanceCurve {
    pub(super) points: *const X3DAudioDistanceCurvePoint,
    pub(super) point_count: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub(super) struct X3DAudioCone {
    pub(super) inner_angle: f32,
    pub(super) outer_angle: f32,
    pub(super) inner_volume: f32,
    pub(super) outer_volume: f32,
    pub(super) inner_lpf: f32,
    pub(super) outer_lpf: f32,
    pub(super) inner_reverb: f32,
    pub(super) outer_reverb: f32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub(super) struct X3DAudioListener {
    pub(super) orient_front: X3DAudioVector,
    pub(super) orient_top: X3DAudioVector,
    pub(super) position: X3DAudioVector,
    pub(super) velocity: X3DAudioVector,
    pub(super) cone: *const X3DAudioCone,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub(super) struct X3DAudioEmitter {
    pub(super) cone: *const X3DAudioCone,
    pub(super) orient_front: X3DAudioVector,
    pub(super) orient_top: X3DAudioVector,
    pub(super) position: X3DAudioVector,
    pub(super) velocity: X3DAudioVector,
    pub(super) inner_radius: f32,
    pub(super) inner_radius_angle: f32,
    pub(super) channel_count: u32,
    pub(super) channel_radius: f32,
    pub(super) channel_azimuths: *const f32,
    pub(super) volume_curve: *const X3DAudioDistanceCurve,
    pub(super) lfe_curve: *const X3DAudioDistanceCurve,
    pub(super) lpf_direct_curve: *const X3DAudioDistanceCurve,
    pub(super) lpf_reverb_curve: *const X3DAudioDistanceCurve,
    pub(super) reverb_curve: *const X3DAudioDistanceCurve,
    pub(super) curve_distance_scaler: f32,
    pub(super) doppler_scaler: f32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(super) struct X3DAudioDspSettings {
    pub(super) matrix_coefficients: *mut f32,
    pub(super) delay_times: *mut f32,
    pub(super) src_channel_count: u32,
    pub(super) dst_channel_count: u32,
    pub(super) lpf_direct_coefficient: f32,
    pub(super) lpf_reverb_coefficient: f32,
    pub(super) reverb_level: f32,
    pub(super) doppler_factor: f32,
    pub(super) emitter_to_listener_angle: f32,
    pub(super) emitter_to_listener_distance: f32,
    pub(super) emitter_velocity_component: f32,
    pub(super) listener_velocity_component: f32,
}

const _: () = assert!(std::mem::size_of::<X3DAudioVector>() == 12);
const _: () = assert!(std::mem::size_of::<X3DAudioDistanceCurvePoint>() == 8);
const _: () = assert!(std::mem::size_of::<X3DAudioDistanceCurve>() == 12);
const _: () = assert!(std::mem::size_of::<X3DAudioCone>() == 32);
const _: () = assert!(std::mem::size_of::<X3DAudioListener>() == 56);
const _: () = assert!(std::mem::size_of::<X3DAudioEmitter>() == 128);
const _: () = assert!(std::mem::size_of::<X3DAudioDspSettings>() == 56);

// ============================================================================
// Handle Codec & 3D Math Helpers
// ============================================================================

#[inline]
fn normalize_or(v: Vec3, fallback: Vec3) -> Vec3 {
    vec3_normalize(v).unwrap_or(fallback)
}

struct X3DHandleData {
    speaker_mask: u32,
    speed_of_sound: f32,
    channel_count: u32,
    lfe_index: i32,
}

impl X3DHandleData {
    fn encode(&self) -> [u8; 20] {
        let mut out = [0u8; 20];
        out[0..4].copy_from_slice(&X3DAUDIO_HANDLE_MAGIC.to_le_bytes());
        out[4..8].copy_from_slice(&self.speaker_mask.to_le_bytes());
        out[8..12].copy_from_slice(&self.speed_of_sound.to_le_bytes());
        out[12..16].copy_from_slice(&self.channel_count.to_le_bytes());
        out[16..20].copy_from_slice(&self.lfe_index.to_le_bytes());
        out
    }

    fn decode(bytes: [u8; 20]) -> Self {
        let magic = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        if magic != X3DAUDIO_HANDLE_MAGIC {
            return Self {
                speaker_mask: 0x3,
                speed_of_sound: X3DAUDIO_SPEED_OF_SOUND,
                channel_count: 2,
                lfe_index: -1,
            };
        }
        let speaker_mask = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let speed_of_sound = f32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let channel_count = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
        let lfe_index = i32::from_le_bytes(bytes[16..20].try_into().unwrap());
        Self {
            speaker_mask,
            speed_of_sound: if speed_of_sound.is_finite() && speed_of_sound > 0.0 {
                speed_of_sound
            } else {
                X3DAUDIO_SPEED_OF_SOUND
            },
            channel_count: channel_count.max(1),
            lfe_index,
        }
    }
}

unsafe fn eval_distance_curve(p_curve: *const X3DAudioDistanceCurve, dist: f32) -> Option<f32> {
    if p_curve.is_null() {
        return None;
    }
    let curve = rd(p_curve);
    let count = (curve.point_count as usize).min(4096);
    if count == 0 || curve.points.is_null() {
        return None;
    }
    let first = rd(curve.points);
    if count == 1 || dist <= first.distance {
        return Some(first.dsp_setting);
    }
    let mut prev = first;
    for i in 1..count {
        let cur = rd(curve.points.add(i));
        if dist <= cur.distance {
            let span = cur.distance - prev.distance;
            if span <= 1e-6 {
                return Some(cur.dsp_setting);
            }
            let t = ((dist - prev.distance) / span).clamp(0.0, 1.0);
            return Some(prev.dsp_setting + (cur.dsp_setting - prev.dsp_setting) * t);
        }
        prev = cur;
    }
    Some(prev.dsp_setting)
}

unsafe fn eval_cone(p_cone: *const X3DAudioCone, angle: f32) -> (f32, f32, f32) {
    if p_cone.is_null() {
        return (1.0, 0.0, 1.0);
    }
    let c = rd(p_cone);
    let inner_half = (c.inner_angle * 0.5).clamp(0.0, PI);
    let outer_half = (c.outer_angle * 0.5).clamp(inner_half, PI);
    if angle <= inner_half {
        (c.inner_volume, c.inner_lpf, c.inner_reverb)
    } else if angle >= outer_half {
        (c.outer_volume, c.outer_lpf, c.outer_reverb)
    } else {
        let t = ((angle - inner_half) / (outer_half - inner_half).max(1e-6)).clamp(0.0, 1.0);
        (
            c.inner_volume + (c.outer_volume - c.inner_volume) * t,
            c.inner_lpf + (c.outer_lpf - c.inner_lpf) * t,
            c.inner_reverb + (c.outer_reverb - c.inner_reverb) * t,
        )
    }
}

/// Speaker bit -> clockwise azimuth from `+Z` front in `[0, 2*pi)`.
fn speaker_azimuth(bit: u32) -> Option<f32> {
    match bit {
        SPEAKER_FRONT_CENTER => Some(0.0),
        SPEAKER_FRONT_RIGHT_OF_CENTER => Some(PI / 8.0),
        SPEAKER_FRONT_RIGHT => Some(FRAC_PI_4),
        SPEAKER_SIDE_RIGHT => Some(FRAC_PI_2),
        SPEAKER_BACK_RIGHT => Some(3.0 * FRAC_PI_4),
        SPEAKER_BACK_CENTER => Some(PI),
        SPEAKER_BACK_LEFT => Some(5.0 * FRAC_PI_4),
        SPEAKER_SIDE_LEFT => Some(3.0 * FRAC_PI_2),
        SPEAKER_FRONT_LEFT => Some(7.0 * FRAC_PI_4),
        SPEAKER_FRONT_LEFT_OF_CENTER => Some(15.0 * PI / 8.0),
        _ => None,
    }
}

/// Compute per-destination-channel panning gains (excluding LFE, before distance attenuation)
/// for a point at listener-local unit direction `(lx, lz)` (`+lx` = right, `+lz` = front).
fn pan_direction_into(
    lx: f32,
    lz: f32,
    omni_blend: f32,
    speaker_mask: u32,
    dst_ch: usize,
    zero_center: bool,
    gains: &mut [f32],
) {
    gains.fill(0.0);
    if dst_ch == 0 {
        return;
    }
    if dst_ch == 1 {
        gains[0] = 1.0;
        return;
    }
    if dst_ch == 2 {
        // Stereo constant-power panning: -pi/2 = hard left (1, 0), 0 = center (0.707, 0.707),
        // +pi/2 = hard right (0, 1).
        let angle = lx.atan2(lz.abs());
        let theta = ((angle + FRAC_PI_2) * 0.5).clamp(0.0, FRAC_PI_2);
        let dir_l = theta.cos();
        let dir_r = theta.sin();
        gains[0] = dir_l + (FRAC_1_SQRT_2 - dir_l) * omni_blend;
        gains[1] = dir_r + (FRAC_1_SQRT_2 - dir_r) * omni_blend;
        return;
    }

    // Collect non-LFE directional speakers with their output channel indices and azimuths.
    let mut speakers: Vec<(usize, f32)> = Vec::with_capacity(dst_ch);
    let mut mask = speaker_mask;
    for ch_idx in 0..dst_ch {
        if mask == 0 {
            break;
        }
        let bit = mask & mask.wrapping_neg();
        mask &= !bit;
        if bit == SPEAKER_LOW_FREQUENCY {
            continue;
        }
        if zero_center && bit == SPEAKER_FRONT_CENTER {
            continue;
        }
        if let Some(az) = speaker_azimuth(bit) {
            speakers.push((ch_idx, az));
        }
    }
    if speakers.is_empty() {
        let eq = 1.0 / (dst_ch as f32).sqrt();
        gains.fill(eq);
        return;
    }
    if speakers.len() == 1 {
        gains[speakers[0].0] = 1.0;
        return;
    }

    speakers.sort_by(|a, b| a.1.total_cmp(&b.1));
    let mut az = lx.atan2(lz);
    if az < 0.0 {
        az += TAU;
    }

    // Find adjacent speakers bracketing `az` on the [0, 2*pi) circle.
    let n = speakers.len();
    let mut left_idx = n - 1;
    let mut right_idx = 0;
    for i in 0..n {
        let a0 = speakers[i].1;
        let next = (i + 1) % n;
        let a1 = if next == 0 {
            speakers[0].1 + TAU
        } else {
            speakers[next].1
        };
        let az_adj = if az < a0 && next == 0 { az + TAU } else { az };
        if az_adj >= a0 && az_adj <= a1 {
            left_idx = i;
            right_idx = next;
            break;
        }
    }

    let (ch0, a0) = speakers[left_idx];
    let (ch1, mut a1) = speakers[right_idx];
    let mut az_adj = az;
    if a1 <= a0 {
        a1 += TAU;
        if az_adj < a0 {
            az_adj += TAU;
        }
    }
    let span = (a1 - a0).max(1e-5);
    let t = ((az_adj - a0) / span).clamp(0.0, 1.0);
    let g0 = (t * FRAC_PI_2).cos();
    let g1 = (t * FRAC_PI_2).sin();

    if omni_blend > 0.0 {
        let omni = 1.0 / (speakers.len() as f32).sqrt();
        for &(ch_idx, _) in &speakers {
            let dir_g = if ch_idx == ch0 {
                g0
            } else if ch_idx == ch1 {
                g1
            } else {
                0.0
            };
            gains[ch_idx] = dir_g + (omni - dir_g) * omni_blend;
        }
    } else {
        gains[ch0] = g0;
        gains[ch1] = g1;
    }
}

// ============================================================================
// X3DAudioInitialize & X3DAudioCalculate Core Implementations
// ============================================================================

pub(super) unsafe fn x3daudio_initialize_core(
    speaker_mask: u32,
    speed_of_sound: f32,
    instance: *mut [u8; 20],
) -> i32 {
    if instance.is_null() {
        return E_POINTER;
    }
    if speaker_mask == 0 || !speed_of_sound.is_finite() || speed_of_sound <= 0.0 {
        return E_INVALIDARG;
    }
    let channel_count = speaker_mask.count_ones();
    let lfe_index = if (speaker_mask & SPEAKER_LOW_FREQUENCY) != 0 {
        (speaker_mask & (SPEAKER_LOW_FREQUENCY - 1)).count_ones() as i32
    } else {
        -1
    };
    let handle = X3DHandleData {
        speaker_mask,
        speed_of_sound,
        channel_count,
        lfe_index,
    };
    wr(instance, handle.encode());
    S_OK
}

pub(super) unsafe fn x3daudio_calculate_impl(
    instance: *const [u8; 20],
    p_listener: *const c_void,
    p_emitter: *const c_void,
    flags: u32,
    p_dsp_settings: *mut c_void,
) {
    if instance.is_null() || p_listener.is_null() || p_emitter.is_null() || p_dsp_settings.is_null()
    {
        return;
    }
    let handle = X3DHandleData::decode(rd(instance));
    let listener = rd(p_listener as *const X3DAudioListener);
    let emitter = rd(p_emitter as *const X3DAudioEmitter);
    let p_dsp_settings = p_dsp_settings as *mut X3DAudioDspSettings;
    let mut out = rd(p_dsp_settings);

    // Listener orthonormal basis (left-handed: +X right, +Y up, +Z front).
    let l_pos = listener.position;
    let l_vel = listener.velocity;
    let l_front = normalize_or(listener.orient_front, Vec3::new(0.0, 0.0, 1.0));
    let l_top_raw = normalize_or(listener.orient_top, Vec3::new(0.0, 1.0, 0.0));
    let l_right = normalize_or(vec3_cross(l_top_raw, l_front), Vec3::new(1.0, 0.0, 0.0));

    // Emitter basis & position.
    let e_pos = emitter.position;
    let e_vel = emitter.velocity;
    let e_front = normalize_or(emitter.orient_front, Vec3::new(0.0, 0.0, 1.0));
    let e_top_raw = normalize_or(emitter.orient_top, Vec3::new(0.0, 1.0, 0.0));
    let e_right = normalize_or(vec3_cross(e_top_raw, e_front), Vec3::new(1.0, 0.0, 0.0));

    let delta = vec3_sub(e_pos, l_pos);
    let distance = vec3_len(delta);
    out.emitter_to_listener_distance = distance;

    let dir = if distance > 1e-6 {
        vec3_scale(delta, 1.0 / distance)
    } else {
        l_front
    };

    // Angles & cones.
    let emitter_to_listener_cos = vec3_dot(e_front, vec3_scale(dir, -1.0)).clamp(-1.0, 1.0);
    let emitter_angle = if distance > 1e-6 {
        emitter_to_listener_cos.acos()
    } else {
        0.0
    };
    if (flags & X3DAUDIO_CALCULATE_EMITTER_ANGLE) != 0 {
        out.emitter_to_listener_angle = emitter_angle;
    }

    let listener_angle = if distance > 1e-6 {
        vec3_dot(l_front, dir).clamp(-1.0, 1.0).acos()
    } else {
        0.0
    };

    let (e_cone_vol, e_cone_lpf, e_cone_rev) = eval_cone(emitter.cone, emitter_angle);
    let (l_cone_vol, l_cone_lpf, l_cone_rev) = eval_cone(listener.cone, listener_angle);
    let cone_vol = (e_cone_vol * l_cone_vol).max(0.0);
    let cone_lpf = (e_cone_lpf + l_cone_lpf).clamp(0.0, 1.0);
    let cone_rev = (e_cone_rev * l_cone_rev).max(0.0);

    // Distance scaling & attenuation curves.
    let scaler = if emitter.curve_distance_scaler.is_finite() && emitter.curve_distance_scaler > 0.0
    {
        emitter.curve_distance_scaler
    } else {
        1.0
    };
    let scaled_dist = distance / scaler;

    let base_vol = eval_distance_curve(emitter.volume_curve, scaled_dist).unwrap_or_else(|| {
        if scaled_dist <= 1.0 {
            1.0
        } else {
            1.0 / scaled_dist
        }
    });
    let volume_att = (base_vol * cone_vol).max(0.0);

    // Doppler shift.
    if (flags & X3DAUDIO_CALCULATE_DOPPLER) != 0 {
        let (v_l, v_e) = if distance > 1e-6 {
            (vec3_dot(l_vel, dir), vec3_dot(e_vel, dir))
        } else {
            (0.0, 0.0)
        };
        out.listener_velocity_component = v_l;
        out.emitter_velocity_component = v_e;
        let d_scale = if emitter.doppler_scaler.is_finite() && emitter.doppler_scaler >= 0.0 {
            emitter.doppler_scaler
        } else {
            1.0
        };
        let c = handle.speed_of_sound;
        if d_scale == 0.0 || c <= 0.0 {
            out.doppler_factor = 1.0;
        } else {
            let vls = (v_l * d_scale).clamp(-c * 0.99, c * 0.99);
            let ves = (v_e * d_scale).clamp(-c * 0.99, c * 0.99);
            out.doppler_factor =
                ((c + vls) / (c + ves)).clamp(XAUDIO2_MIN_FREQ_RATIO, XAUDIO2_MAX_FREQ_RATIO);
        }
    }

    // LPF direct / reverb & reverb send level.
    if (flags & X3DAUDIO_CALCULATE_LPF_DIRECT) != 0 {
        let curve_lpf = eval_distance_curve(emitter.lpf_direct_curve, scaled_dist)
            .unwrap_or_else(|| 1.0 - 0.25 * scaled_dist.clamp(0.0, 1.0));
        out.lpf_direct_coefficient = (curve_lpf * (1.0 - cone_lpf)).clamp(0.0, 1.0);
    }
    if (flags & X3DAUDIO_CALCULATE_LPF_REVERB) != 0 {
        let curve_lpf_rev =
            eval_distance_curve(emitter.lpf_reverb_curve, scaled_dist).unwrap_or(0.75);
        out.lpf_reverb_coefficient = (curve_lpf_rev * (1.0 - cone_lpf)).clamp(0.0, 1.0);
    }
    if (flags & X3DAUDIO_CALCULATE_REVERB) != 0 {
        let curve_rev = eval_distance_curve(emitter.reverb_curve, scaled_dist)
            .unwrap_or_else(|| (1.0 - scaled_dist).clamp(0.0, 1.0));
        out.reverb_level = (curve_rev * cone_rev).max(0.0);
    }

    // Speaker panning matrix (`pMatrixCoefficients[dst * src_ch + src]`).
    let src_ch = if out.src_channel_count > 0 {
        out.src_channel_count as usize
    } else {
        emitter.channel_count.max(1) as usize
    };
    let dst_ch = if out.dst_channel_count > 0 {
        out.dst_channel_count as usize
    } else {
        handle.channel_count as usize
    };

    if (flags & X3DAUDIO_CALCULATE_MATRIX) != 0
        && !out.matrix_coefficients.is_null()
        && src_ch > 0
        && dst_ch > 0
    {
        let total = src_ch * dst_ch;
        for i in 0..total {
            wr(out.matrix_coefficients.add(i), 0.0);
        }

        let zero_center = (flags & X3DAUDIO_CALCULATE_ZEROCENTER) != 0;
        let redirect_lfe = (flags & X3DAUDIO_CALCULATE_REDIRECT_TO_LFE) != 0;
        let lfe_att = eval_distance_curve(emitter.lfe_curve, scaled_dist).unwrap_or(volume_att);

        let inner_radius = if emitter.inner_radius.is_finite() && emitter.inner_radius > 0.0 {
            emitter.inner_radius
        } else {
            0.0
        };
        let inner_angle_spread = if emitter.inner_radius_angle.is_finite() {
            (emitter.inner_radius_angle.clamp(0.0, FRAC_PI_4)) / FRAC_PI_4
        } else {
            0.0
        };
        let omni_blend = if distance <= 1e-6 {
            1.0
        } else if inner_radius > 0.0 && distance < inner_radius {
            let radial = 1.0 - (distance / inner_radius);
            (radial + inner_angle_spread * (1.0 - radial) * 0.25).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let mut ch_gains = vec![0.0f32; dst_ch];
        let use_multi_azimuth = src_ch > 1
            && !emitter.channel_azimuths.is_null()
            && emitter.channel_radius.is_finite()
            && emitter.channel_radius > 0.0;

        if use_multi_azimuth {
            for s in 0..src_ch {
                let az = rd(emitter.channel_azimuths.add(s));
                // X3DAUDIO_2PI marks an LFE source channel.
                if (az - TAU).abs() < 1e-3 {
                    if handle.lfe_index >= 0 && (handle.lfe_index as usize) < dst_ch {
                        let d = handle.lfe_index as usize;
                        wr(out.matrix_coefficients.add(d * src_ch + s), lfe_att);
                    }
                    continue;
                }
                let offset = vec3_add(
                    vec3_scale(e_right, az.sin() * emitter.channel_radius),
                    vec3_scale(e_front, az.cos() * emitter.channel_radius),
                );
                let ch_delta = vec3_sub(vec3_add(e_pos, offset), l_pos);
                let ch_dist = vec3_len(ch_delta);
                let ch_dir = if ch_dist > 1e-6 {
                    vec3_scale(ch_delta, 1.0 / ch_dist)
                } else {
                    l_front
                };
                let lx = vec3_dot(ch_dir, l_right);
                let lz = vec3_dot(ch_dir, l_front);
                pan_direction_into(
                    lx,
                    lz,
                    omni_blend,
                    handle.speaker_mask,
                    dst_ch,
                    zero_center,
                    &mut ch_gains,
                );
                for (d, &g) in ch_gains.iter().enumerate() {
                    wr(out.matrix_coefficients.add(d * src_ch + s), g * volume_att);
                }
                if redirect_lfe && handle.lfe_index >= 0 && (handle.lfe_index as usize) < dst_ch {
                    let d = handle.lfe_index as usize;
                    wr(
                        out.matrix_coefficients.add(d * src_ch + s),
                        lfe_att / (src_ch as f32),
                    );
                }
            }
        } else {
            let lx = if distance > 1e-6 {
                vec3_dot(dir, l_right)
            } else {
                0.0
            };
            let lz = if distance > 1e-6 {
                vec3_dot(dir, l_front)
            } else {
                1.0
            };
            pan_direction_into(
                lx,
                lz,
                omni_blend,
                handle.speaker_mask,
                dst_ch,
                zero_center,
                &mut ch_gains,
            );
            let norm = 1.0 / (src_ch as f32).sqrt();
            for s in 0..src_ch {
                for (d, &g) in ch_gains.iter().enumerate() {
                    wr(
                        out.matrix_coefficients.add(d * src_ch + s),
                        g * volume_att * norm,
                    );
                }
                if (redirect_lfe || !emitter.lfe_curve.is_null())
                    && handle.lfe_index >= 0
                    && (handle.lfe_index as usize) < dst_ch
                {
                    let d = handle.lfe_index as usize;
                    wr(out.matrix_coefficients.add(d * src_ch + s), lfe_att * norm);
                }
            }
        }
    }

    if (flags & X3DAUDIO_CALCULATE_DELAY) != 0 && !out.delay_times.is_null() && dst_ch > 0 {
        for d in 0..dst_ch {
            wr(out.delay_times.add(d), 0.0);
        }
    }

    wr(p_dsp_settings, out);
}
