//! AK speaker setup and planar VBAP (`CAkSpeakerPan`) [V: decompiled, verified against the exe
//! under emulation, see `tests/spatial_vectors.rs`].
//!
//! * `SetSpeakerAngles` impl `0x1415e58a0`: angles (degrees, per-speaker azimuth, front < 90)
//!   are stored as radians (`deg * 0.017453294`, `0x1415de040`), the height angle likewise; the
//!   spread point density is `1 / (min_gap_deg * 0.017453294)` with `min_gap` = min(front,
//!   successive differences, `360 - 2*last`) (validator `0x1415e6d20`).
//! * Pair setup `0x1415de330`: speaker rings per planar channel count (rows below, `x` = right,
//!   `z` = front), pair inverse `[b.z, -b.x, -a.z, a.x] / (a.x*b.z - a.z*b.x)`;
//!   `npairs = n + 2` for `n <= 3` (virtual rear speakers), else `n`.
//! * Gains `0x1415da8d0`: first pair with both gains >= 0 (fallback: last pair);
//!   `g += FLT_EPSILON`, power `w * g_k^2 / (g1^2 + g2^2)` accumulated at the table's output
//!   index. The caller later scales and takes the square root.

/// deg -> rad as AK computes it (`0x3c8efa36`).
pub const DEG2RAD: f32 = 0.017_453_294;

/// Pair -> (speaker bit, output index) tables `float[8][8]` at 0x143815500 (full layer) and
/// 0x1438155e0 (same layer without the centre channel, used for centre% blending). Entry:
/// low 16 bits = speaker bit, high 16 bits = output channel index. Row = planar count - 1.
pub const PAIR_TABLE: [[u32; 8]; 7] = [
    [0x4, 0, 0, 0, 0, 0, 0, 0],
    [0x1, 0x10002, 0x10002, 0x1, 0x1, 0, 0, 0],
    [0x1, 0x20004, 0x10002, 0x10002, 0x1, 0x1, 0, 0],
    [0x20200, 0x1, 0x10002, 0x30400, 0x20200, 0, 0, 0],
    [0x30200, 0x1, 0x20004, 0x10002, 0x40400, 0x30200, 0, 0],
    [0x20010, 0x40200, 0x1, 0x10002, 0x50400, 0x30020, 0x20010, 0],
    [0x30010, 0x50200, 0x1, 0x20004, 0x10002, 0x60400, 0x40020, 0x30010],
];
pub const PAIR_TABLE_NOC: [[u32; 8]; 7] = [
    [0x4, 0, 0, 0, 0, 0, 0, 0],
    [0x1, 0x10002, 0x10002, 0x1, 0x1, 0, 0, 0],
    [0x1, 0x20004, 0x10002, 0x10002, 0x1, 0x1, 0, 0],
    [0x30200, 0x1, 0x10002, 0x40400, 0x30200, 0, 0, 0],
    [0x30200, 0x1, 0x20004, 0x10002, 0x40400, 0x30200, 0, 0],
    [0x30010, 0x50200, 0x1, 0x10002, 0x60400, 0x40020, 0x30010, 0],
    [0x30010, 0x50200, 0x1, 0x20004, 0x10002, 0x60400, 0x40020, 0x30010],
];

/// Panning rule (`AkPanningRule`): AW2 leaves `settingsMainOutput.ePanningRule = 0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanningRule {
    Speakers,
    Headphones,
}

/// The device panner state (`device + 200`): speaker angles, height angle, point density.
#[derive(Debug, Clone)]
pub struct SpeakerSetup {
    /// Per-speaker azimuths in radians (front, side, rear, ...).
    pub angles: Vec<f32>,
    /// Height-layer angle in radians.
    pub height: f32,
    /// Spread point density (points per radian).
    pub density: f32,
    pub rule: PanningRule,
}

impl SpeakerSetup {
    /// `AK::SoundEngine::SetSpeakerAngles(deg, n, height_deg)`. Returns `None` where AK would
    /// reject the angles (and keep the previous setup).
    pub fn new(angles_deg: &[f32], height_deg: f32) -> Option<SpeakerSetup> {
        let n = angles_deg.len();
        if n == 0 || angles_deg.iter().any(|&a| a >= 180.0) || angles_deg[0] >= 90.0 {
            return None;
        }
        let mut gap = angles_deg[0];
        for k in 1..n {
            let d = angles_deg[k] - angles_deg[k - 1];
            if angles_deg[k] < angles_deg[k - 1] || d == 0.0 || d >= 180.0 {
                return None;
            }
            gap = gap.min(d);
        }
        if n > 1 {
            gap = (360.0 - (angles_deg[n - 1] + angles_deg[n - 1])).min(gap);
        }
        if gap < 5.0 {
            return None;
        }
        Some(SpeakerSetup {
            angles: angles_deg.iter().map(|a| a * DEG2RAD).collect(),
            height: height_deg * DEG2RAD,
            density: 1.0 / (gap * DEG2RAD),
            rule: PanningRule::Speakers,
        })
    }

    /// What AW2 sets at startup (Remedy `0x1424cbf20`): `{60, 101, 145}`, 54.7.
    pub fn aw2() -> SpeakerSetup {
        SpeakerSetup::new(&[60.0, 101.0, 145.0], 54.7).unwrap()
    }

    fn angle(&self, i: usize) -> f32 {
        // AK copies the user angles into a fixed array of 3; missing entries stay as set.
        self.angles.get(i).copied().unwrap_or(*self.angles.last().unwrap())
    }

    /// `0x1415de330`: VBAP pair inverses for `n` planar output channels (LFE excluded).
    pub fn pairs(&self, n: usize) -> Vec<[f32; 4]> {
        if n < 2 || n > 7 {
            return vec![];
        }
        let (sf, cf) = (self.angle(0).sin(), self.angle(0).cos());
        let (ss, cs) = (self.angle(1).sin(), self.angle(1).cos());
        let mid = (self.angle(2) - self.angle(1)) * 0.5 + self.angle(1);
        let (sm, cm) = (mid.sin(), mid.cos());
        let (sr, cr) = (self.angle(2).sin(), self.angle(2).cos());
        let fl = [-sf, cf];
        let fr = [sf, cf];
        let c = [0.0, 1.0];
        let ring: Vec<[f32; 2]> = match n {
            2 => vec![fl, fr, [sr, cr], [-sr, cr], fl],
            3 => vec![fl, c, fr, [sr, cr], [-sr, cr], fl],
            4 => vec![[-sm, cm], fl, fr, [sm, cm], [-sm, cm]],
            5 => vec![[-sm, cm], fl, c, fr, [sm, cm], [-sm, cm]],
            6 => vec![[-sr, cr], [-ss, cs], fl, fr, [ss, cs], [sr, cr], [-sr, cr]],
            _ => vec![[-sr, cr], [-ss, cs], fl, c, fr, [ss, cs], [sr, cr], [-sr, cr]],
        };
        let npairs = if n <= 3 { n + 2 } else { n };
        (0..npairs)
            .map(|i| {
                let (a, b) = (ring[i], ring[i + 1]);
                let inv = 1.0 / (a[0] * b[1] - a[1] * b[0]);
                [b[1] * inv, -(b[0] * inv), -(a[1] * inv), a[0] * inv]
            })
            .collect()
    }
}

/// `0x1415da8d0`: accumulate the VBAP powers of direction `p` (x right, z front) into `out`
/// (indexed by output channel) with weight `w`.
pub fn vbap_accumulate(pairs: &[[f32; 4]], p: [f32; 3], w: f32, table: &[u32; 8], out: &mut [f32]) {
    let mut i = 0usize;
    let (mut g1, mut g2);
    loop {
        let m = &pairs[i];
        let a = p[2] * m[1];
        let b = p[0] * m[0];
        let c = p[0] * m[2];
        let d = p[2] * m[3];
        g1 = (a, b);
        g2 = (c, d);
        if -a <= b && -d <= c {
            break;
        }
        if i + 1 >= pairs.len() {
            break;
        }
        i += 1;
    }
    let mut ga = g2.0 + g2.1 + 1.192_092_9e-7;
    let mut gb = g1.0 + g1.1 + 1.192_092_9e-7;
    ga *= ga;
    gb *= gb;
    let w = w / (ga + gb);
    let o1 = (table[i] >> 16) as usize;
    out[o1] += w * gb;
    let o2 = (table[i + 1] >> 16) as usize;
    out[o2] += w * ga;
}
