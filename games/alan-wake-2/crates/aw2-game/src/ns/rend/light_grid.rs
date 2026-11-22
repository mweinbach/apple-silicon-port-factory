//! Clustered (froxel) light grid, rebuilt every frame by `renderWorldInternal::<lambda_1>`
//! 0x14228b130.
//!
//! The grid object (`this`):
//! - `+0x30/+0x38`: array of `0x50`-byte light records (u32 count). Record `+0x00` id, `+0x18`
//!   world AABB, `+0x30` f32 LOD scale, `+0x38` optional `*mut f32` that receives the light's
//!   screen-size LOD ([`light_lod`]).
//! - `+0x40/+0x48`: array of `0xb0`-byte GPU light records (u32 count). `+0x00` 4x4 box-to-world
//!   matrix (row vectors), `+0x94` max view depth, `+0x98` enable (> 0).
//! - `+0x50`: dirty flag (records added/removed: sort, compact and re-upload; see
//!   [`light_grid_update`]).
//! - `+0x58`: GPU buffer of the `0xb0` records.
//! - `+0x60`: `u8 count[16][12][20]` per cell (z, y, x).
//! - `+0xf60`: `u32 index[64][16][12][20]` per-cell light lists (slot-major), terminated by
//!   `0xffff` when a cell has fewer than 64 lights.

use crate::exe::{self, rd, wr};

/// `j_memset` import thunk.
const MEMSET_VA: u64 = 0x1_4183_acc2;
/// `r::Frustum` from the view's double view-projection (`view + 0x1d0`), 6 planes.
const FRUSTUM_FROM_VIEW_VA: u64 = 0x1_423f_d3c0;
/// `double[16]` -> `float[16]` matrix conversion `(out, in)`.
const MAT_D2F_VA: u64 = 0x1_403b_3de0;
/// `j_log2f` / `j_tanf` import thunks.
const LOG2F_VA: u64 = 0x1_4254_76d0;
const TANF_VA: u64 = 0x1_4183_ad8e;

// .rdata constants (read through the image, not copied).
const K_HALF: u64 = 0x1_4342_6780; // 0.5
const K_ONE: u64 = 0x1_4342_68d0; // 1.0
const K_FLT_MAX: u64 = 0x1_4342_6fd0; // FLT_MAX
const K_NEG_FLT_MAX: u64 = 0x1_4342_7174; // -FLT_MAX
const K_NEG_ONE: u64 = 0x1_4342_7080; // -1.0
const K_16: u64 = 0x1_4342_6e08; // grid depth slices
const K_12: u64 = 0x1_4342_6dec; // grid rows
const K_20: u64 = 0x1_4342_6e1c; // grid columns
const K_EPS: u64 = 0x1_4342_6460; // FLT_EPSILON
const K_ABS_MASK: u64 = 0x1_4342_8720; // 4 x 0x7fffffff
const K_SIGN_MASK: u64 = 0x1_4342_8760; // 4 x 0x80000000

// Globals.
/// `{x, y, z, radius, min_distance}` (0x14 bytes) array of LOD reference points (camera
/// positions), and its u32 count.
const G_LOD_POINTS: u64 = 0x1_43bb_ddd8;
const G_LOD_POINT_COUNT: u64 = 0x1_43bb_dde0;
/// Vertical FOV (radians) and output height (i32) used by the LOD metric.
const G_FOV: u64 = 0x1_4384_170c;
const G_HEIGHT: u64 = 0x1_43a3_395c;
/// Grid depth range (log2-distributed slices between near and far).
const G_GRID_NEAR: u64 = 0x1_43a0_0124;
const G_GRID_FAR: u64 = 0x1_43a0_0148;

#[inline(always)]
unsafe fn g<T: Copy>(va: u64) -> T {
    rd::<T>(exe::ptr::<u8>(va) as usize)
}

/// x86 `minss a, b`: `a < b ? a : b` (the second operand on NaN).
#[inline(always)]
fn minss(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

/// x86 `maxss a, b`: `a > b ? a : b` (the second operand on NaN).
#[inline(always)]
fn maxss(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

/// Calls a `float f(float)` exe function (CRT import thunk) through its VA.
#[inline(always)]
unsafe fn call_f32(va: u64, x: f32) -> f32 {
    #[cfg(target_arch = "x86_64")]
    {
        let f: unsafe extern "win64" fn(f32) -> f32 = exe::func(va);
        f(x)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let (_, x0) =
            crate::simd_ops::call_va8_xmm(va, 0, 0, 0, 0, 0, 0, 0, 0, [x.to_bits() as u128, 0, 0, 0]);
        f32::from_bits(x0 as u32)
    }
}

/// Screen-space LOD of a light: distance from the AABB center to the nearest LOD reference
/// point (a point whose radius is below the distance counts as `FLT_MAX`, clamped below by the
/// point's minimum distance), minus the AABB half-diagonal (at least `FLT_EPSILON`); returns
/// `log2((diag / (tan(fov / 2) * dist) * height)^2 / scale) * 0.5 + 1`.
///
/// Original: 0x1423bdc80 (460 bytes, leaf apart from `aabb_half_extents` 0x14024ae10 and the
/// `tanf`/`log2f` import thunks; FLOAT_RETURN, `scale` in xmm1). Its only caller is
/// [`light_grid_update`]; the hook registration (lifted, `cauldron_hot_zb08`) is unchanged.
/// Operation order follows the disassembly (`vminss`/`vmaxss` operand order, `vcmpltss` +
/// `vblendvps`).
unsafe fn light_lod(aabb: usize, scale: f32) -> f32 {
    let half = g::<f32>(K_HALF);
    let flt_max = g::<f32>(K_FLT_MAX);
    let cx = rd::<f32>(aabb) * half + rd::<f32>(aabb + 0xc) * half;
    let cy = rd::<f32>(aabb + 4) * half + rd::<f32>(aabb + 0x10) * half;
    let cz = half * rd::<f32>(aabb + 8) + half * rd::<f32>(aabb + 0x14);
    let n = g::<u32>(G_LOD_POINT_COUNT) as usize;
    let mut p = g::<usize>(G_LOD_POINTS);
    let end = p.wrapping_add(n * 0x14);
    let mut dmin = flt_max;
    while p != end {
        let dy = rd::<f32>(p + 4) - cy;
        let dx = rd::<f32>(p) - cx;
        let dz = rd::<f32>(p + 8) - cz;
        let d = ((dy * dy + dx * dx) + dz * dz).sqrt();
        let v = if rd::<f32>(p + 0xc) < d { flt_max } else { d };
        dmin = minss(dmin, maxss(v, rd::<f32>(p + 0x10)));
        p += 0x14;
    }
    let mut he = [0f32; 4];
    let h = crate::ns::r::aabb::aabb_half_extents(aabb, he.as_mut_ptr() as usize);
    let (h0, h1, h2) = (rd::<f32>(h), rd::<f32>(h + 4), rd::<f32>(h + 8));
    let len = ((h1 * h1 + h0 * h0) + h2 * h2).sqrt();
    let half_fov = half * g::<f32>(G_FOV);
    let dist = maxss(dmin - len, g::<f32>(K_EPS));
    let diag = len + len;
    let t = call_f32(TANF_VA, half_fov);
    let px = diag / (t * dist) * (g::<i32>(G_HEIGHT) as f32);
    call_f32(LOG2F_VA, (px * px) / scale) * half + g::<f32>(K_ONE)
}

/// Conservative AABB-vs-frustum test: false for an empty box (min > max on all three axes)
/// or when the box is entirely behind one plane (`dot(n, c) + d < -dot(|n|, e)`).
///
/// Original: 0x1424b8b00 (313 bytes; calls `aabb_half_extents` 0x14024ae10). The frustum is
/// `float4 plane[24]` + i32 count at `+0x180`. The original uses `vdpps 0x7f` (products of
/// lanes 0..2, lane 3 = +0.0, summed `(p0 + p1) + (p2 + 0)`) and an all-lanes `vcmpltps`
/// against the per-lane sign-flipped radius; reproduced lane by lane. Returns `al` only.
pub unsafe extern "win64" fn aabb_in_frustum(aabb: usize, frustum: usize) -> u8 {
    let a = |o: usize| rd::<f32>(aabb + o);
    if a(0) > a(0xc) && a(4) > a(0x10) && a(8) > a(0x14) {
        return 0;
    }
    let half = g::<f32>(K_HALF);
    let cz = half * a(8) + half * a(0x14);
    let cy = a(4) * half + a(0x10) * half;
    let cx = a(0) * half + a(0xc) * half;
    let mut he = [0f32; 4];
    let h = crate::ns::r::aabb::aabb_half_extents(aabb, he.as_mut_ptr() as usize);
    let (h0, h1, h2) = (rd::<f32>(h), rd::<f32>(h + 4), rd::<f32>(h + 8));
    let n = rd::<i32>(frustum + 0x180) as i64;
    if n <= 0 {
        return 1;
    }
    let abs_mask: [u32; 4] = g(K_ABS_MASK);
    let sign_mask: [u32; 4] = g(K_SIGN_MASK);
    let mut k = 0i64;
    while k < n {
        let pl = frustum + k as usize * 16;
        let p: [u32; 4] = rd(pl);
        let ap = |i: usize| f32::from_bits(p[i] & abs_mask[i]);
        let pf = |i: usize| f32::from_bits(p[i]);
        let r = (ap(0) * h0 + ap(1) * h1) + (ap(2) * h2 + 0.0);
        let s = pf(3) + ((pf(0) * cx + pf(1) * cy) + (pf(2) * cz + 0.0));
        let mut mask = 0u32;
        for (i, sm) in sign_mask.iter().enumerate() {
            if s < f32::from_bits(r.to_bits() ^ sm) {
                mask |= 1 << i;
            }
        }
        if mask == 0xf {
            return 0;
        }
        k += 1;
    }
    1
}

#[repr(C, align(16))]
struct Frustum([u8; 0x190]);

#[repr(C, align(16))]
struct Mat4([f32; 16]);

/// Per-frame clustered light grid update.
///
/// 1. Each `0x50` light record gets its screen LOD ([`light_lod`]) written through its `+0x38`
///    pointer.
/// 2. Dirty grids (`+0x50`) are first re-sorted, compacted into new record arrays, re-indexed
///    and re-uploaded to the GPU buffer at `+0x58`; that path (allocations, refcounted handle
///    moves, an STL stable sort, the upload and its FH4 unwind map) runs in the original code
///    (the lifted shard, or the emulator without shards) for the whole call.
/// 3. Clears the per-cell counts, builds the view frustum and float view-projection, then for
///    every visible `0xb0` record projects its 8 box corners: view depth range `[wmin, wmax]`
///    and NDC x/y/z bounds (any corner with clip z < 0 widens x/y to [-1, 1] and z min to 0).
///    Records beyond their `+0x94` max depth, disabled (`+0x98 <= 0`) or with an empty NDC box
///    are skipped. The rest are binned into a 20 x 12 x 16 grid (x, y flipped, log2 depth
///    slices between [`G_GRID_NEAR`] and [`G_GRID_FAR`]), at most 64 lights per cell.
/// 4. Terminates every non-full cell list with `0xffff`.
///
/// Original: 0x1423bf1b0 (4554 bytes, FH4 handler for the dirty path's vectors). Called once
/// per frame (`<lambda_1>` 0x14228b130); jobprof showed 181 us per call in the lifted version.
pub unsafe extern "win64" fn light_grid_update(this: usize, view: usize) {
    let n_lights = rd::<u32>(this + 0x38) as usize;
    if n_lights == 0 {
        return;
    }
    if rd::<u8>(this + 0x50) != 0 {
        run_original(this, view);
        return;
    }
    let lights = rd::<usize>(this + 0x30);
    for i in 0..n_lights {
        let e = lights + i * 0x50;
        let lod = light_lod(e + 0x18, rd::<f32>(e + 0x30));
        let out = rd::<usize>(e + 0x38);
        if out != 0 {
            wr::<f32>(out, lod);
        }
    }

    exe::call_va!(MEMSET_VA, (this + 0x60, 0u32, 0xf00u64));
    let mut fr = Frustum([0; 0x190]);
    let fp = fr.0.as_mut_ptr() as usize;
    for k in 0..0x18 {
        wr::<[f32; 4]>(fp + k * 16, [0.0, 1.0, 0.0, 0.0]);
    }
    wr::<u32>(fp + 0x180, 0);
    exe::call_native!(
        FRUSTUM_FROM_VIEW_VA,
        crate::ns::rend::pass_hot_helpers::frustum_from_view_proj,
        (view, fp) -> ()
    );
    let mut vm = Mat4([0.0; 16]);
    let _ = exe::call_native!(
        MAT_D2F_VA,
        crate::ns::rend::pass_hot_helpers::mat4_d2f,
        (vm.0.as_mut_ptr() as usize, view + 0x1d0) -> usize
    );
    let v = vm.0;

    let one = g::<f32>(K_ONE);
    let half = g::<f32>(K_HALF);
    let ln = call_f32(LOG2F_VA, g::<f32>(G_GRID_NEAR));
    let lf = call_f32(LOG2F_VA, g::<f32>(G_GRID_FAR));
    let zscale = one / (lf - ln);
    let zbias = f32::from_bits(ln.to_bits() ^ g::<u32>(K_SIGN_MASK)) * zscale;
    let (flt_max, neg_flt_max, neg_one) =
        (g::<f32>(K_FLT_MAX), g::<f32>(K_NEG_FLT_MAX), g::<f32>(K_NEG_ONE));
    let (k16, k12, k20) = (g::<f32>(K_16), g::<f32>(K_12), g::<f32>(K_20));

    let mut i = 0usize;
    while i < rd::<u32>(this + 0x48) as usize {
        let aabb = rd::<usize>(this + 0x30) + i * 0x50 + 0x18;
        if aabb_in_frustum(aabb, fp) != 0 {
            let m_addr = rd::<usize>(this + 0x40) + i * 0xb0;
            let m: [f32; 16] = rd(m_addr);
            let (mut xmin, mut ymin, mut zmin) = (flt_max, flt_max, flt_max);
            let (mut xmax, mut ymax, mut zmax) = (neg_flt_max, neg_flt_max, neg_flt_max);
            let (mut wmin, mut wmax) = (flt_max, neg_flt_max);
            for c in 0..8u32 {
                let b0 = (c & 1) as f32;
                let sx = (b0 + b0) - one;
                let sy = ((c & 2) as f32) - one;
                let sz = ((c & 4) as f32) * half - one;
                let py = ((m[1] * sx + sy * m[5]) + m[9] * sz) + m[13];
                let pz = ((m[2] * sx + sy * m[6]) + m[10] * sz) + m[14];
                let pw = ((sx * m[3] + sy * m[7]) + sz * m[11]) + m[15];
                let px = ((sx * m[0] + sy * m[4]) + sz * m[8]) + m[12];
                let c2 = ((px * v[2] + py * v[6]) + pz * v[10]) + pw * v[14];
                let c3 = ((px * v[3] + py * v[7]) + pz * v[11]) + pw * v[15];
                let c0 = ((px * v[0] + py * v[4]) + pz * v[8]) + pw * v[12];
                wmin = minss(wmin, c3);
                wmax = maxss(wmax, c3);
                if !(c2 >= 0.0) {
                    xmin = neg_one;
                    ymin = neg_one;
                    zmin = 0.0;
                    xmax = one;
                    ymax = one;
                } else {
                    let inv = one / c3;
                    let z = inv * c2;
                    zmin = minss(zmin, z);
                    let c1 = ((px * v[1] + py * v[5]) + pz * v[9]) + pw * v[13];
                    let y = c1 * inv;
                    ymin = minss(ymin, y);
                    let x = inv * c0;
                    xmin = minss(xmin, x);
                    zmax = maxss(zmax, z);
                    ymax = maxss(ymax, y);
                    xmax = maxss(xmax, x);
                }
            }
            let visible = !(wmin > rd::<f32>(m_addr + 0x94))
                && !(0.0 >= rd::<f32>(m_addr + 0x98))
                && !(xmin >= xmax)
                && !(ymin >= ymax)
                && !(zmin >= zmax);
            if visible {
                let lw = call_f32(LOG2F_VA, wmin);
                let z0 = minss(maxss((lw * zscale + zbias) * k16, 0.0), k16);
                let y0 = minss(maxss((half - ymax * half) * k12, 0.0), k12);
                let x0 = minss(maxss((xmin * half + half) * k20, 0.0), k20);
                let (x0, y0, z0) = (x0.floor() as i32, y0.floor() as i32, z0.floor() as i32);
                let lw = call_f32(LOG2F_VA, wmax);
                let z1 = minss(maxss((lw * zscale + zbias) * k16 + one, 0.0), k16);
                let y1 = minss(maxss((half - ymin * half) * k12 + one, 0.0), k12);
                let x1 = minss(maxss((xmax * half + half) * k20 + one, 0.0), k20);
                let (x1, y1, z1) = (x1.floor() as i32, y1.floor() as i32, z1.floor() as i32);
                for z in z0..z1 {
                    for y in y0..y1 {
                        for x in x0..x1 {
                            let cell = this + 0x60 + (x + 20 * (y + 12 * z)) as usize;
                            let cnt = rd::<u8>(cell);
                            if cnt < 0x40 {
                                let idx = x + 20 * (y + 12 * ((cnt as i32) * 16 + z));
                                wr::<u32>(this + 0xf60 + idx as usize * 4, i as u32);
                                wr::<u8>(cell, cnt + 1);
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }

    for z in 0..16i32 {
        for y in 0..12i32 {
            for x in 0..20i32 {
                let cnt = rd::<u8>(this + 0x60 + (x + 20 * (y + 12 * z)) as usize);
                if cnt < 0x40 {
                    let idx = x + 20 * (y + 12 * ((cnt as i32) * 16 + z));
                    wr::<u32>(this + 0xf60 + idx as usize * 4, 0xffff);
                }
            }
        }
    }
}

/// Runs the original 0x1423bf1b0 for the whole call (dirty grids).
#[cold]
#[inline(never)]
unsafe fn run_original(this: usize, view: usize) {
    #[cfg(feature = "shards")]
    {
        let _ = crate::shards::subsystems::fn_1423bf1b0(this as u64, view as u64, 0, 0, 0, 0, 0, 0);
    }
    #[cfg(not(feature = "shards"))]
    {
        const LIGHT_GRID_UPDATE_VA: u64 = 0x1_423b_f1b0;
        let entry = exe::ptr::<u8>(LIGHT_GRID_UPDATE_VA) as u64;
        let _ = crate::emu::emu_run(entry, [this as u64, view as u64, 0, 0, 0, 0, 0, 0]);
    }
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

/// Restores exe globals on drop (also on an early `Err` return).
struct SavedGlobals(Vec<(usize, u64)>);

impl SavedGlobals {
    unsafe fn save(vas: &[u64]) -> Self {
        Self(vas.iter().map(|&va| {
            let a = exe::ptr::<u8>(va) as usize;
            (a, rd::<u64>(a))
        }).collect())
    }
}

impl Drop for SavedGlobals {
    fn drop(&mut self) {
        for &(a, v) in self.0.iter().rev() {
            unsafe { wr::<u64>(a, v) };
        }
    }
}

fn unit(rng: &mut crate::difftest::Rng) -> f32 {
    (rng.next_u32() >> 8) as f32 / 16_777_216.0
}

fn range(rng: &mut crate::difftest::Rng, lo: f32, hi: f32) -> f32 {
    lo + (hi - lo) * unit(rng)
}

/// Random frustums (0..24 planes, negative counts, NaN planes) and boxes (inverted, NaN,
/// degenerate), compared on `al`.
pub unsafe fn diff_aabb_in_frustum(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let (fo, fnew): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = crate::difftest::Rng::new(0x1424_b8b0);
    let mut fr = Frustum([0; 0x190]);
    let fp = fr.0.as_mut_ptr() as usize;
    let mut buf = [0u32; 8];
    let mut cases = 0u32;
    for round in 0..600u32 {
        let n: i32 = match round % 7 {
            0 => -(rng.below(3) as i32),
            1 => 6,
            _ => rng.below(25) as i32,
        };
        for k in 0..0x18 {
            let mut pl = [0f32; 4];
            for c in pl.iter_mut().take(3) {
                *c = range(&mut rng, -1.0, 1.0);
            }
            pl[3] = range(&mut rng, -50.0, 50.0);
            if round % 37 == 5 && k == 1 {
                pl[rng.below(4) as usize] = f32::NAN;
            }
            if round % 23 == 3 {
                pl[rng.below(4) as usize] = 0.0;
            }
            wr::<[f32; 4]>(fp + k * 16, pl);
        }
        wr::<i32>(fp + 0x180, n);
        // Under-aligned box (offset 4) in some rounds.
        let ab = buf.as_mut_ptr() as usize + if round % 2 == 0 { 0 } else { 4 };
        let c = [range(&mut rng, -60.0, 60.0), range(&mut rng, -60.0, 60.0), range(&mut rng, -60.0, 60.0)];
        let e = [range(&mut rng, 0.0, 30.0), range(&mut rng, 0.0, 30.0), range(&mut rng, 0.0, 30.0)];
        let mut b = [c[0] - e[0], c[1] - e[1], c[2] - e[2], c[0] + e[0], c[1] + e[1], c[2] + e[2]];
        match round % 11 {
            1 => b.swap(0, 3),
            2 => {
                b.swap(0, 3);
                b.swap(1, 4);
                b.swap(2, 5);
            }
            3 => b[rng.below(6) as usize] = f32::NAN,
            4 => b[3] = b[0],
            _ => {}
        }
        wr::<[f32; 6]>(ab, b);
        let o = fo(ab, fp);
        let nn = fnew(ab, fp);
        crate::diff_eq!(o, nn, "round {round} n={n} box={b:?}");
        cases += 1;
    }
    Ok(cases)
}

const GRID_SIZE: usize = 0xf60 + 64 * 16 * 12 * 20 * 4;

/// Row-vector perspective (LH, depth 0..1) times a yaw rotation, in doubles at `view + 0x1d0`.
fn view_proj(rng: &mut crate::difftest::Rng, round: u32) -> [f64; 16] {
    let (zn, zf) = (0.1f64, 3000.0f64);
    let ys = 1.0 / (0.5f64 * range(rng, 0.6, 1.4) as f64).tan();
    let xs = ys / range(rng, 1.0, 2.4) as f64;
    let a = zf / (zf - zn);
    let p = [xs, 0.0, 0.0, 0.0, 0.0, ys, 0.0, 0.0, 0.0, 0.0, a, 1.0, 0.0, 0.0, -zn * a, 0.0];
    let th = if round % 3 == 0 { 0.0 } else { range(rng, -0.6, 0.6) as f64 };
    let (s, c) = th.sin_cos();
    // r = rotation about y (row vectors), then t = r * p.
    let r = [c, 0.0, -s, 0.0, 0.0, 1.0, 0.0, 0.0, s, 0.0, c, 0.0, 0.0, range(rng, -2.0, 2.0) as f64, 0.0, 1.0];
    let mut t = [0f64; 16];
    for i in 0..4 {
        for j in 0..4 {
            t[i * 4 + j] = (0..4).map(|k| r[i * 4 + k] * p[k * 4 + j]).sum();
        }
    }
    t
}

/// Grids of 0..100 lights in front of / around / behind a perspective camera (some huge, to
/// fill cells past 64; NaN and disabled records; under max depth or not), random LOD
/// reference points and grid depth ranges. Compares the whole grid object (counts, all 64
/// list slots incl. untouched ones, terminators) and every LOD output. Count 0 checks the
/// early return; the dirty path is not covered (it runs the original).
pub unsafe fn diff_light_grid_update(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (fo, fnew): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let _saved = SavedGlobals::save(&[
        G_LOD_POINTS,
        G_LOD_POINT_COUNT,
        G_FOV,
        G_HEIGHT,
        G_GRID_NEAR,
        G_GRID_FAR,
    ]);
    let mut rng = crate::difftest::Rng::new(0x1423_bf1b);
    let mut grid_o = vec![0u64; GRID_SIZE / 8 + 1];
    let mut grid_n = vec![0u64; GRID_SIZE / 8 + 1];
    let mut init = vec![0u8; GRID_SIZE];
    let mut view = vec![0u64; 0x60];
    let mut pts = vec![0f32; 5 * 8];
    let mut cases = 0u32;
    let (mut binned, mut full, mut lods) = (0usize, 0usize, 0usize);
    for round in 0..40u32 {
        let n = match round {
            0 => 0,
            1 => 1,
            r if r % 5 == 4 => 100,
            _ => 1 + rng.below(60) as usize,
        };
        // Globals.
        let np = rng.below(9) as usize;
        for k in 0..np {
            pts[k * 5] = range(&mut rng, -30.0, 30.0);
            pts[k * 5 + 1] = range(&mut rng, -5.0, 5.0);
            pts[k * 5 + 2] = range(&mut rng, -30.0, 30.0);
            pts[k * 5 + 3] = if k % 3 == 2 { 1e9 } else { range(&mut rng, 0.0, 200.0) };
            pts[k * 5 + 4] = range(&mut rng, 0.0, 20.0);
        }
        wr::<usize>(exe::ptr::<u8>(G_LOD_POINTS) as usize, pts.as_ptr() as usize);
        wr::<u32>(exe::ptr::<u8>(G_LOD_POINT_COUNT) as usize, np as u32);
        wr::<f32>(exe::ptr::<u8>(G_FOV) as usize, range(&mut rng, 0.7, 1.6));
        wr::<i32>(exe::ptr::<u8>(G_HEIGHT) as usize, [1080, 720, 2160][round as usize % 3]);
        let near = [0.1f32, 1.0, 0.5][round as usize % 3];
        wr::<f32>(exe::ptr::<u8>(G_GRID_NEAR) as usize, near);
        wr::<f32>(exe::ptr::<u8>(G_GRID_FAR) as usize, range(&mut rng, 100.0, 4000.0));

        // View.
        let vp = view_proj(&mut rng, round);
        let vaddr = view.as_mut_ptr() as usize;
        wr::<[f64; 16]>(vaddr + 0x1d0, vp);

        // Records.
        let mut recs = vec![0u64; n * 10 + 1];
        let mut gpu = vec![0u64; n * 22 + 1];
        let mut outs = vec![0f32; n + 1];
        let ra = recs.as_mut_ptr() as usize;
        let ga = gpu.as_mut_ptr() as usize;
        for i in 0..n {
            let r = ra + i * 0x50;
            let gr = ga + i * 0xb0;
            let big = round % 5 == 4 && i % 4 != 3;
            let depth = match rng.below(8) {
                0 => range(&mut rng, -40.0, 0.5),
                1 => range(&mut rng, 0.5, 4.0),
                _ => range(&mut rng, 2.0, 400.0),
            };
            let (tx, ty, tz) = (
                range(&mut rng, -1.2, 1.2) * depth.abs().max(1.0),
                range(&mut rng, -0.8, 0.8) * depth.abs().max(1.0),
                depth,
            );
            let size = if big { range(&mut rng, 100.0, 900.0) } else { range(&mut rng, 0.2, 40.0) };
            let mut m = [0f32; 16];
            m[0] = size * range(&mut rng, 0.5, 1.0);
            m[5] = size * range(&mut rng, 0.5, 1.0);
            m[10] = size * range(&mut rng, 0.5, 1.0);
            if i % 3 == 1 {
                m[1] = range(&mut rng, -0.3, 0.3) * size;
                m[4] = range(&mut rng, -0.3, 0.3) * size;
            }
            m[12] = tx;
            m[13] = ty;
            m[14] = tz;
            m[15] = 1.0;
            if round % 13 == 7 && i == 0 {
                m[rng.below(16) as usize] = f32::NAN;
            }
            wr::<[f32; 16]>(gr, m);
            for k in 16..0x2c {
                wr::<u32>(gr + k * 4, rng.next_u32());
            }
            let maxd = match rng.below(6) {
                0 => range(&mut rng, 1.0, 50.0),
                1 => f32::INFINITY,
                _ => range(&mut rng, 100.0, 5000.0),
            };
            wr::<f32>(gr + 0x94, maxd);
            wr::<f32>(gr + 0x98, if rng.below(10) == 0 { range(&mut rng, -1.0, 0.0) } else { range(&mut rng, 0.1, 3.0) });

            wr::<u64>(r, rng.next_u64());
            let ext = size * 1.8;
            wr::<[f32; 6]>(r + 0x18, [tx - ext, ty - ext, tz - ext, tx + ext, ty + ext, tz + ext]);
            wr::<f32>(r + 0x30, range(&mut rng, 0.5, 8.0));
            wr::<usize>(r + 0x38, if i % 5 == 2 { 0 } else { outs.as_mut_ptr() as usize + i * 4 });
        }

        // Grid object.
        for (k, b) in init.iter_mut().enumerate() {
            *b = if k < 0xf60 { rng.next_u32() as u8 } else { (k as u8) ^ 0xa5 };
        }
        wr::<usize>(init.as_mut_ptr() as usize + 0x30, ra);
        wr::<u32>(init.as_mut_ptr() as usize + 0x38, n as u32);
        wr::<usize>(init.as_mut_ptr() as usize + 0x40, ga);
        wr::<u32>(init.as_mut_ptr() as usize + 0x48, n as u32);
        wr::<u8>(init.as_mut_ptr() as usize + 0x50, 0);

        let run = |f: F, grid: &mut Vec<u64>, outs: &mut Vec<f32>| -> Vec<f32> {
            outs.iter_mut().for_each(|o| *o = -7.0);
            let gp = grid.as_mut_ptr() as *mut u8;
            core::ptr::copy_nonoverlapping(init.as_ptr(), gp, GRID_SIZE);
            f(gp as usize, vaddr);
            outs.clone()
        };
        let oo = run(fo, &mut grid_o, &mut outs);
        let on = run(fnew, &mut grid_n, &mut outs);
        let bo = core::slice::from_raw_parts(grid_o.as_ptr() as *const u8, GRID_SIZE);
        let bn = core::slice::from_raw_parts(grid_n.as_ptr() as *const u8, GRID_SIZE);
        if bo != bn {
            let k = bo.iter().zip(bn).position(|(a, b)| a != b).unwrap_or(0);
            return Err(format!(
                "round {round} n={n}: grid differs at +{k:#x} (orig {:#x}, new {:#x})",
                bo[k], bn[k]
            ));
        }
        let ob: Vec<u32> = oo.iter().map(|f| f.to_bits()).collect();
        let nb: Vec<u32> = on.iter().map(|f| f.to_bits()).collect();
        crate::diff_eq!(ob, nb, "round {round} n={n}: LOD outputs");
        if n != 0 {
            let counts = &bo[0x60..0xf60];
            binned += counts.iter().filter(|&&c| c != 0).count();
            full += counts.iter().filter(|&&c| c == 0x40).count();
            lods += oo.iter().filter(|f| f.is_finite() && **f != -7.0).count();
        }
        cases += 1;
    }
    // The inputs must exercise binning, the 64-light cap and the LOD metric.
    if binned < 2000 || full == 0 || lods < 100 {
        return Err(format!("weak coverage: {binned} binned cells, {full} full cells, {lods} finite LODs"));
    }
    Ok(cases)
}
