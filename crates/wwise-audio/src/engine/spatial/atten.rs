//! Emitter-listener geometry and attenuation (`CAkAttenuation`, ray gains / filters)
//! [V: decompiled; end-to-end verification pending positional oracle traces].
//!
//! * Ray geometry `0x1415d5250`: `d = emitter - listener` (f64 difference converted to f32),
//!   `dist = |d|`; emitter angle `acosAK(-(d.front_e)/dist)`, listener angle
//!   `acosAK((d.front_l)/dist)`; scaling = listener scaling x emitter scaling (when the emitter
//!   is not the listener's own game object); curve distance = `dist / scaling`.
//! * Ray gains `0x1415d3d00`: dry / game-aux / user-aux volume curves at the curve distance
//!   (none -> 1; a missing curve body -> 0; aux slots sharing the dry curve reuse its value),
//!   cone attenuation on the dry gain (`FastPow10(t * cone_vol * 0.05)`), occlusion and
//!   transmission on all three, obstruction and diffraction on the dry gain only.
//! * Ray filters `0x1415d3350`: LPF = max(LPF curve, t*cone LPF, occlusion LPF, transmission
//!   LPF, [dry] obstruction LPF, diffraction LPF); HPF likewise.
//! * Loader `0x1416140f0`: cone angles are stored as half angles (`deg * 0.008726646`); curves
//!   used by a *volume* slot are converted: scaling 0 -> 4 (dB to linear), scaling 2 (dB) ->
//!   `y += 1` and scaling 0 (linear amplitude).

use super::panner::V3;
use crate::bnk::hirc::{Attenuation, GraphPoint};
use crate::engine::math::{eval_graph, fast_pow10};

/// Attenuation curve slots (AkAttenuationCurveType).
pub mod slot {
    pub const VOLUME_DRY: usize = 0;
    pub const VOLUME_AUX_GAME: usize = 1;
    pub const VOLUME_AUX_USER: usize = 2;
    pub const LPF: usize = 3;
    pub const HPF: usize = 4;
    pub const SPREAD: usize = 5;
    pub const FOCUS: usize = 6;
    pub const OBS_VOLUME: usize = 7;
    pub const OBS_LPF: usize = 8;
    pub const OBS_HPF: usize = 9;
    pub const OCC_VOLUME: usize = 10;
    pub const OCC_LPF: usize = 11;
    pub const OCC_HPF: usize = 12;
    pub const DIFF_VOLUME: usize = 13;
    pub const DIFF_LPF: usize = 14;
    pub const DIFF_HPF: usize = 15;
    pub const TRANS_VOLUME: usize = 16;
    pub const TRANS_LPF: usize = 17;
    pub const TRANS_HPF: usize = 18;
}

const VOLUME_SLOTS: [usize; 7] = [0, 1, 2, 7, 10, 13, 16];

/// A loaded conversion curve (after the attenuation loader's volume-slot conversion).
#[derive(Debug, Clone)]
pub struct Curve {
    pub scaling: u8,
    pub points: Vec<GraphPoint>,
}

impl Curve {
    pub fn eval(&self, x: f32) -> f32 {
        if self.points.is_empty() {
            return 0.0;
        }
        eval_graph(&self.points, self.scaling, x)
    }

    /// The loader's conversion for curves referenced by a volume slot.
    fn as_volume(scaling: u8, points: &[GraphPoint]) -> Curve {
        match scaling {
            0 => Curve { scaling: 4, points: points.to_vec() },
            2 => Curve {
                scaling: 0,
                points: points.iter().map(|p| GraphPoint { from: p.from, to: p.to + 1.0, interp: p.interp }).collect(),
            },
            s => Curve { scaling: s, points: points.to_vec() },
        }
    }
}

/// Project-wide obstruction/occlusion curves (Init.bnk `ENVS`, `DAT_143930f48`):
/// [obs vol, obs LPF, obs HPF, occ vol, occ LPF, occ HPF]; `None` when disabled.
#[derive(Debug, Clone, Default)]
pub struct ProjectCurves {
    pub curves: [Option<Curve>; 6],
}

impl ProjectCurves {
    /// From `Bank::env_curves` of Init.bnk. Volume curves get the same conversion as in
    /// attenuations [A: the ENVS loader was not decompiled].
    pub fn from_env(env: &[crate::bnk::EnvCurve]) -> ProjectCurves {
        let mut p = ProjectCurves::default();
        for (i, c) in env.iter().take(6).enumerate() {
            if c.enabled && !c.points.is_empty() {
                p.curves[i] = Some(if i % 3 == 0 {
                    Curve::as_volume(c.scaling, &c.points)
                } else {
                    Curve { scaling: c.scaling, points: c.points.clone() }
                });
            }
        }
        p
    }
    fn get(&self, i: usize) -> Option<&Curve> {
        self.curves[i].as_ref()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cone {
    /// Half angles in radians.
    pub inside: f32,
    pub outside: f32,
    /// Outside volume (dB), LPF and HPF values.
    pub volume: f32,
    pub lpf: f32,
    pub hpf: f32,
}

#[derive(Debug, Clone)]
enum CurveRef {
    None,
    Own(usize),
    Project,
}

/// A loaded `CAkAttenuation` (curves converted as by the loader).
#[derive(Debug, Clone)]
pub struct AttenModel {
    pub height_spread: bool,
    pub cone: Option<Cone>,
    slots: Vec<CurveRef>,
    curves: Vec<Curve>,
}

impl AttenModel {
    pub fn new(a: &Attenuation) -> AttenModel {
        let slots: Vec<CurveRef> = a
            .curve_to_use
            .iter()
            .map(|&c| match c {
                -2 => CurveRef::Project,
                c if c >= 0 && (c as usize) < a.curves.len() => CurveRef::Own(c as usize),
                _ => CurveRef::None,
            })
            .collect();
        // A curve is converted if any volume slot references it (the loader converts it in
        // place, so non-volume slots sharing it see the converted curve too).
        let curves = a
            .curves
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if VOLUME_SLOTS.iter().any(|&s| a.curve_to_use[s] == i as i8) {
                    Curve::as_volume(c.scaling, &c.points)
                } else {
                    Curve { scaling: c.scaling, points: c.points.clone() }
                }
            })
            .collect();
        AttenModel {
            height_spread: a.height_spread_enabled,
            cone: a.cone.as_ref().map(|c| Cone {
                inside: c.inside_degrees * 0.008_726_646,
                outside: c.outside_degrees * 0.008_726_646,
                volume: c.outside_volume,
                lpf: c.lo_pass,
                hpf: c.hi_pass,
            }),
            slots,
            curves,
        }
    }

    fn own(&self, s: usize) -> Option<usize> {
        match self.slots[s] {
            CurveRef::Own(i) => Some(i),
            _ => None,
        }
    }

    /// The curve for a slot; project curves resolve through `proj` (index `pi`).
    fn curve<'a>(&'a self, s: usize, proj: &'a ProjectCurves, pi: Option<usize>) -> Option<&'a Curve> {
        match self.slots[s] {
            CurveRef::None => None,
            CurveRef::Own(i) => Some(&self.curves[i]),
            CurveRef::Project => pi.and_then(|i| proj.get(i)),
        }
    }

    pub fn has_curve(&self, s: usize) -> bool {
        !matches!(self.slots[s], CurveRef::None)
    }

    /// Spread (slot 5) or focus (slot 6) curve value at `dist`, if the slot has a curve.
    pub fn eval_slot(&self, s: usize, dist: f32) -> Option<f32> {
        self.own(s).map(|i| self.curves[i].eval(dist))
    }
}

/// World transform (`AkWorldTransform`): orientation front/top, f64 position.
#[derive(Debug, Clone, Copy)]
pub struct WorldTransform {
    pub front: V3,
    pub top: V3,
    pub pos: [f64; 3],
}

impl Default for WorldTransform {
    fn default() -> Self {
        WorldTransform { front: [0.0, 0.0, 1.0], top: [0.0, 1.0, 0.0], pos: [0.0; 3] }
    }
}

fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// AK's `acos` approximation (`0x1415d5250`).
pub fn acos_ak(x: f32) -> f32 {
    let c = x.clamp(-1.0, 1.0);
    let a = c.abs();
    let r = 1.537_952_7 - (((((a * 84.31466 - 242.71996) * a + 262.81305) * a - 131.11235) * a + 29.66154) * a - 1.451_838_4) * a;
    if c < 0.0 {
        std::f32::consts::PI - r
    } else {
        r
    }
}

/// Emitter-listener pair (`AkEmitterListenerPair` + AK's extras), one per emitter position.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    /// Curve distance (`dist / scaling`).
    pub distance: f32,
    pub emitter_angle: f32,
    pub listener_angle: f32,
    pub scaling: f32,
    pub dry: f32,
    pub game_aux: f32,
    pub user_aux: f32,
    pub occlusion: f32,
    pub obstruction: f32,
    pub diffraction: f32,
    pub transmission: f32,
    /// Cone interpolation factor (0 inside, 1 outside).
    pub cone_t: f32,
    pub lpf: f32,
    pub hpf: f32,
    /// Listener-space position (x right, y up, z front) divided by the scaling.
    pub pos_l: V3,
    /// Emitter orientation in listener space.
    pub front_l: V3,
    pub top_l: V3,
}

/// `0x1415d5250` + the listener-space transform of `0x1415d4800`.
pub fn ray_geometry(listener: &WorldTransform, listener_scaling: f32, emitter: &WorldTransform, emitter_scaling: f32) -> Ray {
    let d = [
        (emitter.pos[0] - listener.pos[0]) as f32,
        (emitter.pos[1] - listener.pos[1]) as f32,
        (emitter.pos[2] - listener.pos[2]) as f32,
    ];
    let dist = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    let (ea, la) = if dist <= 0.0 {
        (0.0, 0.0)
    } else {
        let fe = emitter.front;
        let fl = listener.front;
        let ce = -((d[1] * fe[1] + d[0] * fe[0] + d[2] * fe[2]) / dist);
        let cl = (d[1] * fl[1] + d[0] * fl[0] + d[2] * fl[2]) / dist;
        (acos_ak(ce), acos_ak(cl))
    };
    let scaling = listener_scaling * emitter_scaling;
    let side = cross(listener.top, listener.front);
    let up = listener.top;
    let fr = listener.front;
    let inv = 1.0 / scaling;
    let dot = |a: V3, b: V3| a[1] * b[1] + a[0] * b[0] + a[2] * b[2];
    let pos_l = [dot(d, side) * inv, dot(d, up) * inv, dot(d, fr) * inv];
    let rot = |v: V3| [dot(v, side), dot(v, up), dot(v, fr)];
    Ray {
        distance: dist / scaling,
        emitter_angle: ea,
        listener_angle: la,
        scaling,
        dry: 1.0,
        game_aux: 1.0,
        user_aux: 1.0,
        occlusion: 0.0,
        obstruction: 0.0,
        diffraction: 0.0,
        transmission: 0.0,
        cone_t: 0.0,
        lpf: 0.0,
        hpf: 0.0,
        pos_l,
        front_l: rot(emitter.front),
        top_l: rot(emitter.top),
    }
}

/// Per-voice attenuation inputs that come from the voice context (`ctx+0x180..0x188` are the
/// cone values after RTPC; default to the attenuation's).
#[derive(Debug, Clone, Copy, Default)]
pub struct ConeOverride {
    pub volume: Option<f32>,
    pub lpf: Option<f32>,
    pub hpf: Option<f32>,
}

/// `0x1415d3d00` (gains) + `0x1415d3350` (filters, `dry_connection` = connection type 0).
/// `obstruction_enabled` is `FUN_1415907b0(ctx)` (true unless spatial-audio rooms say
/// otherwise).
pub fn ray_gains(ray: &mut Ray, att: Option<&AttenModel>, proj: &ProjectCurves, cone_ovr: ConeOverride, dry_connection: bool) {
    let obstruction_enabled = true;
    let conv = |c: Option<&Curve>, x: f32| c.map(|c| c.eval(x));
    // ---- gains
    let (obs_c, diff_c, occ_c, trans_c) = match att {
        Some(a) => {
            let dry_c = a.curve(slot::VOLUME_DRY, proj, None);
            let dry = conv(dry_c, ray.distance).unwrap_or(1.0);
            ray.dry = dry;
            let same = |s: usize| a.own(s).is_some() && a.own(s) == a.own(slot::VOLUME_DRY);
            ray.game_aux = if !a.has_curve(slot::VOLUME_AUX_GAME) {
                1.0
            } else if same(slot::VOLUME_AUX_GAME) {
                dry
            } else {
                conv(a.curve(slot::VOLUME_AUX_GAME, proj, None), ray.distance).unwrap_or(1.0)
            };
            ray.user_aux = if !a.has_curve(slot::VOLUME_AUX_USER) {
                1.0
            } else if same(slot::VOLUME_AUX_USER) {
                dry
            } else {
                conv(a.curve(slot::VOLUME_AUX_USER, proj, None), ray.distance).unwrap_or(1.0)
            };
            if let Some(cone) = &a.cone {
                let vol = cone_ovr.volume.unwrap_or(cone.volume);
                if cone.inside < ray.emitter_angle {
                    let t = if ray.emitter_angle < cone.outside {
                        (ray.emitter_angle - cone.inside) / (cone.outside - cone.inside)
                    } else {
                        1.0
                    };
                    ray.cone_t = t;
                    ray.dry *= fast_pow10(t * vol * 0.05);
                } else {
                    ray.cone_t = 0.0;
                }
            }
            (
                if obstruction_enabled { a.curve(slot::OBS_VOLUME, proj, Some(0)) } else { None },
                if obstruction_enabled { a.curve(slot::DIFF_VOLUME, proj, Some(0)) } else { None },
                a.curve(slot::OCC_VOLUME, proj, Some(3)),
                a.curve(slot::TRANS_VOLUME, proj, Some(3)),
            )
        }
        None => (
            if obstruction_enabled { proj.get(0) } else { None },
            None,
            proj.get(3),
            proj.get(3),
        ),
    };
    if ray.occlusion != 0.0 {
        if let Some(f) = conv(occ_c, ray.occlusion) {
            ray.dry *= f;
            ray.game_aux *= f;
            ray.user_aux *= f;
        }
    }
    if ray.transmission != 0.0 {
        if let Some(f) = conv(trans_c, ray.transmission) {
            ray.dry *= f;
            ray.game_aux *= f;
            ray.user_aux *= f;
        }
    }
    if ray.obstruction != 0.0 {
        if let Some(f) = conv(obs_c, ray.obstruction) {
            ray.dry *= f;
        }
    }
    if ray.diffraction != 0.0 {
        if let Some(f) = conv(diff_c, ray.diffraction) {
            ray.dry *= f;
        }
    }
    // ---- filters
    let (lpf_c, hpf_c, cone_lpf, cone_hpf, occ_l, occ_h, tr_l, tr_h, obs_l, obs_h, df_l, df_h) = match att {
        Some(a) => (
            a.curve(slot::LPF, proj, None),
            a.curve(slot::HPF, proj, None),
            a.cone.map(|c| cone_ovr.lpf.unwrap_or(c.lpf)).unwrap_or(0.0),
            a.cone.map(|c| cone_ovr.hpf.unwrap_or(c.hpf)).unwrap_or(0.0),
            a.curve(slot::OCC_LPF, proj, Some(4)),
            a.curve(slot::OCC_HPF, proj, Some(5)),
            a.curve(slot::TRANS_LPF, proj, Some(4)),
            a.curve(slot::TRANS_HPF, proj, Some(5)),
            if obstruction_enabled { a.curve(slot::OBS_LPF, proj, Some(1)) } else { None },
            if obstruction_enabled { a.curve(slot::OBS_HPF, proj, Some(2)) } else { None },
            if obstruction_enabled { a.curve(slot::DIFF_LPF, proj, Some(1)) } else { None },
            if obstruction_enabled { a.curve(slot::DIFF_HPF, proj, Some(2)) } else { None },
        ),
        None => (
            None,
            None,
            0.0,
            0.0,
            proj.get(4),
            proj.get(5),
            None,
            None,
            if obstruction_enabled { proj.get(1) } else { None },
            if obstruction_enabled { proj.get(2) } else { None },
            None,
            None,
        ),
    };
    let mut lpf = conv(lpf_c, ray.distance).unwrap_or(0.0).max(ray.cone_t * cone_lpf);
    let mut hpf = conv(hpf_c, ray.distance).unwrap_or(0.0).max(ray.cone_t * cone_hpf);
    let mx = |v: &mut f32, c: Option<&Curve>, x: f32| {
        if x != 0.0 {
            if let Some(f) = conv(c, x) {
                *v = v.max(f);
            }
        }
    };
    mx(&mut lpf, occ_l, ray.occlusion);
    mx(&mut hpf, occ_h, ray.occlusion);
    mx(&mut lpf, tr_l, ray.transmission);
    mx(&mut hpf, tr_h, ray.transmission);
    if dry_connection {
        mx(&mut lpf, obs_l, ray.obstruction);
        mx(&mut hpf, obs_h, ray.obstruction);
        mx(&mut lpf, df_l, ray.diffraction);
        mx(&mut hpf, df_h, ray.diffraction);
    }
    ray.lpf = lpf;
    ray.hpf = hpf;
}
