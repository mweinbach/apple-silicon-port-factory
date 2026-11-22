//! AK 3D speaker panner (`CAkSpeakerPan`): spread point sets, focus, per-channel rotation,
//! VBAP accumulation and power->amplitude normalisation [V: decompiled; verified bit-close
//! against the exe under emulation, `tests/spatial_vectors.rs`].
//!
//! Entry `0x1415dc9f0` (`AK::SpeakerVolumes` positioning API, also used by the per-connection
//! path `0x1415d6570`): spatialization mode 0 -> plain 2D panner; otherwise per ray
//! `0x1415d5a00` (height spread, 3D matrix `0x1415e1980`, blend with the 2D matrix by
//! `spat_mix`). Coordinates are listener-relative: `x` right, `y` up, `z` front.
//!
//! Precision: AK normalises spread points with `rsqrtps` (12-bit); VBAP is scale invariant so
//! only `0x1415da060`'s elevation remap sees it (~1e-4 relative). We use exact `1/sqrt`.

use super::vbap::{vbap_accumulate, PanningRule, SpeakerSetup, PAIR_TABLE, PAIR_TABLE_NOC};

pub type V3 = [f32; 3];
pub type M3 = [f32; 9];

/// Speaker bits (`AkChannelConfig` mask).
pub const FC: u32 = 0x4;
pub const LFE: u32 = 0x8;
/// Planar speakers AK's 3D panner handles for standard inputs (`0x1415dc9f0`: mask & 0x637).
pub const PLANAR_IN: u32 = 0x637;

/// Per-channel source directions for multichannel spread, `float[8][7][3]` @0x14268ceb0
/// (row = active channel count - 1, channels in pair-table order).
const CHANNEL_DIRS: [[[f32; 3]; 7]; 7] = {
    const H: f32 = 0.707_106_77;
    [
        [[0.0, 0.0, 1.0], [0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3]],
        [[-1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3]],
        [[-1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3]],
        [[-H, 0.0, -H], [-H, 0.0, H], [H, 0.0, H], [H, 0.0, -H], [0.0; 3], [0.0; 3], [0.0; 3]],
        [
            [-0.587_785_24, 0.0, -0.809_017],
            [-0.951_056_5, 0.0, 0.309_017],
            [0.0, 0.0, 1.0],
            [0.951_056_5, 0.0, 0.309_017],
            [0.587_785_24, 0.0, -0.809_017],
            [0.0; 3],
            [0.0; 3],
        ],
        [
            [-0.5, 0.0, -0.866_025_4],
            [-1.0, 0.0, 0.0],
            [-0.5, 0.0, 0.866_025_4],
            [0.5, 0.0, 0.866_025_4],
            [1.0, 0.0, 0.0],
            [0.5, 0.0, -0.866_025_4],
            [0.0; 3],
        ],
        [
            [-0.433_883_73, 0.0, -0.900_968_9],
            [-0.974_927_9, 0.0, -0.222_520_93],
            [-0.781_831_5, 0.0, 0.623_489_8],
            [0.0, 0.0, 1.0],
            [0.781_831_5, 0.0, 0.623_489_8],
            [0.974_927_9, 0.0, -0.222_520_93],
            [0.433_883_73, 0.0, -0.900_968_9],
        ],
    ]
};

/// A channel configuration (`AkChannelConfig`): count, type (1 standard), speaker mask.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cfg {
    pub n: u32,
    pub kind: u32,
    pub mask: u32,
}

impl Cfg {
    pub const fn std(mask: u32) -> Cfg {
        Cfg { n: mask.count_ones(), kind: 1, mask }
    }
    pub fn word(&self) -> u32 {
        self.n | (self.kind << 8) | (self.mask << 12)
    }
    pub fn from_word(w: u32) -> Cfg {
        Cfg { n: w & 0xff, kind: (w >> 8) & 0xf, mask: w >> 12 }
    }
    /// Planar (no height / LFE) standard config.
    fn is_planar_std(&self) -> bool {
        self.kind == 1 && self.mask & 0xff9c0 == 0
    }
    fn without(&self, bits: u32) -> Cfg {
        let mask = self.mask & !bits;
        Cfg { n: self.n - (self.mask & bits).count_ones(), kind: self.kind, mask }
    }
}

/// `Ak3dData` (0x30 bytes): emitter orientation and listener-relative position, spread/focus
/// in [0, 1], channel mask.
#[derive(Debug, Clone, Copy)]
pub struct Ray3d {
    pub front: V3,
    pub top: V3,
    pub pos: V3,
    pub spread: f32,
    pub focus: f32,
    pub mask: u32,
}

/// `AkPositioningData` tail (after the `Ak3dData`).
#[derive(Debug, Clone, Copy)]
pub struct PanParams {
    pub center: f32,
    pub pan_lr: f32,
    pub pan_bf: f32,
    pub pan_du: f32,
    pub spat_mix: f32,
    /// 0 none (2D only), 1 position only (orientation forced identity), 2 position+orientation.
    pub spat_mode: u8,
    pub panner_type: u8,
    pub height_spread: bool,
}

impl Default for PanParams {
    fn default() -> Self {
        PanParams {
            center: 1.0,
            pan_lr: 0.0,
            pan_bf: 0.0,
            pan_du: 0.0,
            spat_mix: 1.0,
            spat_mode: 2,
            panner_type: 0,
            height_spread: false,
        }
    }
}

#[inline]
fn xf(m: &M3, p: V3) -> V3 {
    [
        p[1] * m[3] + p[0] * m[0] + p[2] * m[6],
        p[1] * m[4] + p[0] * m[1] + p[2] * m[7],
        p[1] * m[5] + p[0] * m[2] + p[2] * m[8],
    ]
}

/// `0x1415db9a0`: rotation whose third row is `normalize(dir)` (Rodrigues about the axis
/// perpendicular to +Z and `dir`).
pub fn dir_rotation(d: V3) -> M3 {
    let (x, y, z) = (d[0], d[1], d[2]);
    let n2 = y * y + x * x + z * z;
    if !(n2 > 0.0) {
        return [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
    }
    let r = 1.0 / n2.sqrt();
    let mut x = r * x;
    let z = r * z;
    let mut ny = -(r * y);
    let q = x * x + ny * ny;
    if q <= 1.192_092_9e-7 {
        let s = if z >= 0.0 { 1.0 } else { -1.0 };
        return [1.0, 0.0, 0.0, 0.0, s, 0.0, 0.0, 0.0, s];
    }
    let s = q.sqrt();
    let c1 = 1.0 - z;
    x *= 1.0 / s;
    ny *= 1.0 / s;
    let f9 = x * s;
    let f4 = ny * s;
    let f8 = x * ny * c1;
    let f5 = ny * ny * c1 + z;
    let f6 = x * x * c1 + z;
    [f5, f8, -f9, f8, f6, f4, f9, -f4, z]
}

/// `0x1415db790`: (R, B) where `R = dir_rotation(pos)` and `B` maps emitter-local points
/// (x side, y top, z front) into the source-direction frame.
fn ray_bases(ray: &Ray3d) -> (M3, M3) {
    let r = dir_rotation(ray.pos);
    let [f6, f5, f4] = ray.front;
    let [f1, f2, f3] = ray.top;
    let f9 = f4 * f2 - f5 * f3;
    let f8 = f6 * f3 - f1 * f4;
    let f7 = f1 * f5 - f6 * f2;
    let b = [
        f8 * r[1] + f9 * r[0] + f7 * r[2],
        f9 * r[3] + f8 * r[4] + f7 * r[5],
        f9 * r[6] + f8 * r[7] + f7 * r[8],
        f1 * r[0] + f2 * r[1] + f3 * r[2],
        f1 * r[3] + f2 * r[4] + f3 * r[5],
        f1 * r[6] + f2 * r[7] + f3 * r[8],
        f6 * r[0] + f5 * r[1] + f4 * r[2],
        f6 * r[3] + f5 * r[4] + f4 * r[5],
        f5 * r[7] + f6 * r[6] + f4 * r[8],
    ];
    (r, b)
}

/// `0x1415def20` planar case: `n` channels' worth of fan points around +Z for the given
/// density (count rounded up to a multiple of 4).
pub fn fan_points(n: u32, density: f32) -> Vec<V3> {
    let w = (1.0 / n as f32) * std::f32::consts::PI;
    let mut cnt = (w * density + 1.01) as u32;
    if cnt & 3 != 0 {
        cnt += 4 - (cnt & 3);
    }
    let step = w / cnt as f32;
    let mut a = (cnt - 1) as f32 * step;
    let mut v = Vec::with_capacity(cnt as usize);
    for _ in 0..cnt {
        let (s, c) = (-a).sin_cos();
        v.push([s, 0.0, c]);
        a -= step + step;
    }
    v
}

#[inline]
fn fast_sin_recip(s: f32) -> f32 {
    let a = s * std::f32::consts::PI;
    let f = a * 1.273_239_5 - a.abs() * a * 0.405_284_7;
    1.0 / ((f.abs() * f - f) * 0.225 + f + 0.0001)
}

/// Elevation remap shared by `0x1415da060` / `0x1415d9ce0` (spread/focus > 0.5): returns the
/// new (xy scale, z).
#[inline]
fn remap_wide(x: f32, y: f32, z: f32, s: f32) -> (f32, f32) {
    let base = (1.0 - s) * 1.570_796_4;
    let z = z.max(-1.0);
    let r2 = y * y + x * x;
    let zz = if r2 - 0.0001 >= 0.0 {
        z
    } else if z >= 0.0 {
        1.0
    } else {
        -1.0
    };
    let ang = ((zz * zz * 0.719_088 + -0.35786) * zz * zz + 1.085_865) * zz * s + base;
    let sn = ((ang * ang * 0.008_332_16 + -0.166_666_55) * ang * ang + 1.0) * ang;
    let c2 = -sn * sn + 1.0;
    let t = (c2 * r2).max(0.0001);
    (c2 / t.sqrt(), sn)
}

/// `0x1415d9ce0` / `0x1415d9f00`: focus applied to the base fan (`s = 1 - focus`).
fn focus_points(pts: &[V3], s: f32) -> Vec<V3> {
    if s > 0.5 {
        pts.iter()
            .map(|p| {
                let (f, sn) = remap_wide(p[0], p[1], p[2], s);
                [p[0] * f, p[1] * f, sn]
            })
            .collect()
    } else {
        let k = fast_sin_recip(s);
        pts.iter()
            .map(|p| {
                let z = p[2].max(-0.999) + k;
                let r = 1.0 / ((p[0] * p[0] + p[1] * p[1]) + z * z).sqrt();
                [p[0] * r, p[1] * r, z * r]
            })
            .collect()
    }
}

/// `0x1415de1a0`: replicate the fan per channel, rotated about Y to each channel direction.
fn rotate_per_channel(n: usize, pts: &[V3]) -> Vec<V3> {
    let mut out = Vec::with_capacity(n * pts.len());
    for c in 0..n {
        let [s, yy, cc] = CHANNEL_DIRS[n - 1][c];
        for p in pts {
            out.push([p[2] * s + p[0] * cc, p[1] + p[2] * yy, p[2] * cc + p[0] * -s]);
        }
    }
    out
}

/// `0x1415e6c90` (`0x1415da460` / `0x1415da060`): orient the points with the emitter, then
/// squeeze them towards the source direction by `spread`.
fn spread_points(pts: &[V3], ray: &Ray3d, spread: f32) -> Vec<V3> {
    let (r, b) = ray_bases(ray);
    if spread > 0.5 {
        pts.iter()
            .map(|p| {
                let q = xf(&b, *p);
                let (f, sn) = remap_wide(q[0], q[1], q[2], spread);
                xf(&r, [q[0] * f, q[1] * f, sn])
            })
            .collect()
    } else {
        let k = fast_sin_recip(spread);
        pts.iter()
            .map(|p| {
                let q = xf(&b, *p);
                let z = q[2].max(-0.999) + k;
                let rr = 1.0 / (q[1] * q[1] + q[0] * q[0] + z * z).sqrt();
                xf(&r, [q[0] * rr, q[1] * rr, z * rr])
            })
            .collect()
    }
}

/// `0x1415e1980` for a standard input and a standard planar output: the 3D matrix
/// `[in_channel][out_channel]` (amplitudes) of one ray.
pub fn matrix_3d(
    setup: &SpeakerSetup,
    ray: &Ray3d,
    center: f32,
    spread: f32,
    focus: f32,
    in_cfg: Cfg,
    out_cfg: Cfg,
) -> Vec<Vec<f32>> {
    let mut mx = vec![vec![0.0f32; out_cfg.n as usize]; in_cfg.n as usize];
    let in_mask = in_cfg.mask & PLANAR_IN;
    let n_in = in_mask.count_ones().min(7);
    let active = in_mask & ray.mask;
    let n_act = active.count_ones() & 7;
    if n_act == 0 || out_cfg.kind != 1 || !out_cfg.without(LFE).is_planar_std() {
        return mx; // TODO(A4): height / ambisonic / anonymous outputs
    }
    let mut spread = spread;
    let (pts, count) = if spread > 0.0 {
        let base = fan_points(n_act, setup.density);
        let count = base.len();
        let s = 1.0 - focus;
        let pts = if n_act == 1 {
            spread *= s;
            base
        } else {
            let f = if s < 1.0 { focus_points(&base, s) } else { base };
            rotate_per_channel(n_act as usize, &f)
        };
        (spread_points(&pts, ray, spread), count)
    } else {
        let (r, _) = (dir_rotation(ray.pos), ());
        (vec![[r[6], r[7], r[8]]; n_act as usize], 1)
    };
    let out_pl = out_cfg.without(LFE);
    let n_out = out_pl.n as usize;
    if !(2..=7).contains(&n_out) {
        return mx; // TODO(A4): mono output
    }
    let pairs = setup.pairs(n_out);
    let has_fc = out_pl.mask & FC != 0;
    let pairs_noc = if has_fc && center < 1.0 { setup.pairs(n_out - 1) } else { vec![] };
    let scale = (1.0 / n_act as f32) / count as f32;
    let in_row = &PAIR_TABLE[n_in as usize - 1];
    let headphones = setup.rule == PanningRule::Headphones && out_pl.mask == 3;
    let mut j = 0usize;
    for k in 0..n_in as usize {
        let ent = in_row[k];
        if ent & active & 0xffff == 0 {
            continue;
        }
        let row = &mut mx[(ent >> 16) as usize];
        for p in &pts[j * count..(j + 1) * count] {
            if headphones {
                let x = p[0];
                let r = (x * 0.5 + 0.707_106_77) - x * 0.207_106_77 * x;
                let r = r * r;
                row[0] += if 1.0 < r { 0.0 } else { 1.0 - r };
                row[1] += r;
            } else if pairs_noc.is_empty() {
                vbap_accumulate(&pairs, *p, 1.0, &PAIR_TABLE[n_out - 1], row);
            } else {
                vbap_accumulate(&pairs, *p, center, &PAIR_TABLE[n_out - 1], row);
                vbap_accumulate(&pairs_noc, *p, 1.0 - center, &PAIR_TABLE_NOC[n_out - 2], row);
            }
        }
        for v in row.iter_mut() {
            *v = (*v * scale).sqrt();
        }
        j += 1;
    }
    mx
}

/// `0x1415d5a00` for one ray: height spread, the 3D matrix, and the blend with the 2D panner
/// (`m2d`, required when `spat_mix < 1`).
pub fn ray_matrix(
    setup: &SpeakerSetup,
    ray: &Ray3d,
    pp: &PanParams,
    in_cfg: Cfg,
    out_cfg: Cfg,
    m2d: Option<&[Vec<f32>]>,
) -> Vec<Vec<f32>> {
    let mut spread = ray.spread;
    if pp.height_spread && out_cfg.kind == 1 {
        let [x, y, z] = ray.pos;
        if out_cfg.mask & 0xff9c0 == 0 {
            let y2 = y * y;
            let n2 = x * x + y2 + z * z;
            if n2 > 0.0 {
                let r = y2 / n2;
                if r > spread {
                    spread = r;
                }
            }
        } else if y < 0.0 {
            let n2 = x * x + y * y + z * z;
            if n2 > 0.0 {
                let v = (-1.0 / n2.sqrt()) * y;
                let a = v.abs();
                let mut s = (((((a * 84.31466 - 242.71996) * a + 262.81305) * a - 131.11235) * a
                    + 29.66154)
                    * a
                    - 1.4518384)
                    * a
                    + 0.0328437;
                if v < 0.0 {
                    s = -s;
                }
                let s = s * 0.318_309_87;
                if s > spread {
                    spread = s;
                }
            }
        }
    }
    let mut mx = matrix_3d(setup, ray, pp.center, spread, ray.focus, in_cfg, out_cfg);
    if pp.spat_mix < 1.0 {
        if let Some(m2) = m2d {
            let a = pp.spat_mix;
            let b = 1.0 - a;
            for (r, r2) in mx.iter_mut().zip(m2) {
                for (v, w) in r.iter_mut().zip(r2) {
                    *v = *w * b + a * *v;
                }
            }
        }
    }
    mx
}

/// `0x1415dc9f0`: full positioning matrix for one emitter (single ray), including the
/// LFE->LFE passthrough. `m2d` is the 2D-panner matrix (needed for `spat_mode == 0` or
/// `spat_mix < 1`).
pub fn positioning_matrix(
    setup: &SpeakerSetup,
    ray: &Ray3d,
    pp: &PanParams,
    in_cfg: Cfg,
    out_cfg: Cfg,
    m2d: Option<&[Vec<f32>]>,
) -> Vec<Vec<f32>> {
    if pp.spat_mode == 0 {
        return m2d.map(|m| m.to_vec()).unwrap_or_else(|| vec![vec![0.0; out_cfg.n as usize]; in_cfg.n as usize]);
    }
    let mut r = *ray;
    if pp.spat_mode == 1 {
        r.front = [0.0, 0.0, 1.0];
        r.top = [0.0, 1.0, 0.0];
    }
    let adj = if in_cfg.kind == 1 { Cfg::std(in_cfg.mask & PLANAR_IN) } else { in_cfg };
    let mut mx = if adj.n != 0 {
        let m = ray_matrix(setup, &r, pp, in_cfg, out_cfg, m2d);
        m
    } else {
        vec![vec![0.0; out_cfg.n as usize]; in_cfg.n as usize]
    };
    if in_cfg.mask & LFE != 0 && out_cfg.mask & LFE != 0 {
        mx[in_cfg.n as usize - 1][out_cfg.n as usize - 1] = 1.0;
    }
    mx
}
