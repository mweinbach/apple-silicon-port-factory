//! Clean-room bit-exact DSP primitives for the Bink 2 (`KB2f`..`KB2n`, integer path) video decoder.
//!
//! All stride parameters are expressed in **elements** (`u8` for byte planes, `i16` for residual
//! buffers). Arithmetic matches the reference x86-64 SSE2/SSSE3 routines bit-for-bit, including
//! `i16` wrapping operations, rounding biases, and fixed-point shifts.

use super::tables::{DEBLOCK_RESIDUAL_MASK, DEBLOCK_WEIGHTS};

/// 1D 8-point integer IDCT butterfly using `i16` wrapping arithmetic and arithmetic right shifts.
#[cfg_attr(target_arch = "aarch64", allow(dead_code))]
#[inline]
fn idct1d(x: [i16; 8]) -> [i16; 8] {
    let a0 = x[0].wrapping_add(x[4]);
    let a1 = x[0].wrapping_sub(x[4]);
    let a2 = (x[6] >> 1).wrapping_add(x[2]).wrapping_add(x[2] >> 2);
    let a3 = (x[2] >> 1).wrapping_sub(x[6].wrapping_add(x[6] >> 2));
    let b0 = a0.wrapping_add(a2);
    let b3 = a0.wrapping_sub(a2);
    let b1 = a1.wrapping_add(a3);
    let b2 = a1.wrapping_sub(a3);

    let p0 = x[3].wrapping_add(x[5]);
    let p1 = x[3].wrapping_sub(x[5]);
    let t1 = x[1].wrapping_add(p0);
    let t3 = x[1].wrapping_sub(p0);
    let t7 = p1.wrapping_add(x[7]);
    let t5 = p1.wrapping_sub(x[7]);

    let o0 = (t7 >> 2).wrapping_add(t1.wrapping_add(t1 >> 2).wrapping_sub(t1 >> 4));
    let o3 = (t1 >> 2).wrapping_sub(t7.wrapping_add(t7 >> 2).wrapping_sub(t7 >> 4));
    let o1 = t5.wrapping_sub(t5 >> 2).wrapping_sub(t5 >> 4).wrapping_add(t3);
    let o2 = t3.wrapping_sub(t3 >> 2).wrapping_sub(t3 >> 4).wrapping_sub(t5);

    [
        b0.wrapping_add(o0),
        b1.wrapping_add(o1),
        b2.wrapping_add(o2),
        b3.wrapping_add(o3),
        b3.wrapping_sub(o3),
        b2.wrapping_sub(o2),
        b1.wrapping_sub(o1),
        b0.wrapping_sub(o0),
    ]
}

/// 8x8 integer IDCT (scalar reference).
#[cfg_attr(target_arch = "aarch64", allow(dead_code))]
pub(crate) fn idct8x8_scalar(
    coeffs: &[i16; 64],
    out: &mut [i16],
    out_off: usize,
    out_stride: usize,
) {
    let mut p1 = [[0i16; 8]; 8];
    for v in 0..8 {
        let col_in = [
            coeffs[v],
            coeffs[8 + v],
            coeffs[16 + v],
            coeffs[24 + v],
            coeffs[32 + v],
            coeffs[40 + v],
            coeffs[48 + v],
            coeffs[56 + v],
        ];
        p1[v] = idct1d(col_in);
    }

    for x in 0..8 {
        let row_in = [
            p1[0][x], p1[1][x], p1[2][x], p1[3][x], p1[4][x], p1[5][x], p1[6][x], p1[7][x],
        ];
        let p2 = idct1d(row_in);
        for y in 0..8 {
            out[out_off + y * out_stride + x] = p2[y] >> 6;
        }
    }
}

/// 8x8 integer IDCT.
///
/// `coeffs` is indexed as `[u * 8 + v]` where `u` is X-frequency and `v` is Y-frequency.
/// Writes spatial `[y][x]` outputs (shifted by `>> 6`, no rounding bias) to `out`.
pub fn idct8x8(coeffs: &[i16; 64], out: &mut [i16], out_off: usize, out_stride: usize) {
    #[cfg(target_arch = "aarch64")]
    {
        super::dsp_neon::idct8x8_neon(coeffs, out, out_off, out_stride);
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        idct8x8_scalar(coeffs, out, out_off, out_stride);
    }
}

/// Fast-path DC-only 8x8 IDCT (scalar reference).
#[cfg_attr(target_arch = "aarch64", allow(dead_code))]
pub(crate) fn idct8x8_dc_scalar(dc: i16, out: &mut [i16], out_off: usize, out_stride: usize) {
    let val = dc >> 6;
    for y in 0..8 {
        let row_start = out_off + y * out_stride;
        out[row_start..row_start + 8].fill(val);
    }
}

/// Fast-path DC-only 8x8 IDCT: fills an 8x8 tile with `dc >> 6`.
pub fn idct8x8_dc(dc: i16, out: &mut [i16], out_off: usize, out_stride: usize) {
    #[cfg(target_arch = "aarch64")]
    {
        super::dsp_neon::idct8x8_dc_neon(dc, out, out_off, out_stride);
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        idct8x8_dc_scalar(dc, out, out_off, out_stride);
    }
}

/// Clamps a 16x8 `i16` block to `0..=255` and stores it into `dst` (scalar reference).
#[cfg_attr(target_arch = "aarch64", allow(dead_code))]
pub(crate) fn clamp_store_16x8_scalar(
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
    src: &[i16],
    src_off: usize,
    src_stride: usize,
) {
    for r in 0..8 {
        let d_off = dst_off + r * dst_stride;
        let s_off = src_off + r * src_stride;
        let d_row = &mut dst[d_off..d_off + 16];
        let s_row = &src[s_off..s_off + 16];
        for c in 0..16 {
            d_row[c] = s_row[c].clamp(0, 255) as u8;
        }
    }
}

/// Clamps a 16x8 `i16` block to `0..=255` and stores it into `dst`.
pub fn clamp_store_16x8(
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
    src: &[i16],
    src_off: usize,
    src_stride: usize,
) {
    #[cfg(target_arch = "aarch64")]
    {
        super::dsp_neon::clamp_store_16x8_neon(dst, dst_off, dst_stride, src, src_off, src_stride);
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        clamp_store_16x8_scalar(dst, dst_off, dst_stride, src, src_off, src_stride);
    }
}

/// Adds a 16x8 `i16` residual to an 8-bit prediction block (scalar reference).
#[allow(clippy::too_many_arguments)]
#[cfg_attr(target_arch = "aarch64", allow(dead_code))]
pub(crate) fn add_clamp_16x8_scalar(
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
    res: &[i16],
    res_off: usize,
    res_stride: usize,
    pred: &[u8],
    pred_off: usize,
    pred_stride: usize,
) {
    for r in 0..8 {
        let d_off = dst_off + r * dst_stride;
        let r_off = res_off + r * res_stride;
        let p_off = pred_off + r * pred_stride;
        let d_row = &mut dst[d_off..d_off + 16];
        let r_row = &res[r_off..r_off + 16];
        let p_row = &pred[p_off..p_off + 16];
        for c in 0..16 {
            d_row[c] = (p_row[c] as i16).wrapping_add(r_row[c]).clamp(0, 255) as u8;
        }
    }
}

/// Adds a 16x8 `i16` residual to an 8-bit prediction block (wrapping in `i16`), clamps to
/// `0..=255`, and stores into `dst`.
#[allow(clippy::too_many_arguments)]
pub fn add_clamp_16x8(
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
    res: &[i16],
    res_off: usize,
    res_stride: usize,
    pred: &[u8],
    pred_off: usize,
    pred_stride: usize,
) {
    #[cfg(target_arch = "aarch64")]
    {
        super::dsp_neon::add_clamp_16x8_neon(
            dst,
            dst_off,
            dst_stride,
            res,
            res_off,
            res_stride,
            pred,
            pred_off,
            pred_stride,
        );
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        add_clamp_16x8_scalar(
            dst,
            dst_off,
            dst_stride,
            res,
            res_off,
            res_stride,
            pred,
            pred_off,
            pred_stride,
        );
    }
}

/// Copies a 16x16 byte block from `src` to `dst` (scalar reference).
pub(crate) fn copy_16x16_scalar(
    src: &[u8],
    src_off: usize,
    src_stride: usize,
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
) {
    for r in 0..16 {
        let s = src_off + r * src_stride;
        let d = dst_off + r * dst_stride;
        dst[d..d + 16].copy_from_slice(&src[s..s + 16]);
    }
}

/// Copies a 16x16 byte block from `src` to `dst`.
pub fn copy_16x16(
    src: &[u8],
    src_off: usize,
    src_stride: usize,
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
) {
    #[cfg(target_arch = "aarch64")]
    {
        super::dsp_neon::copy_16x16_neon(src, src_off, src_stride, dst, dst_off, dst_stride);
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        copy_16x16_scalar(src, src_off, src_stride, dst, dst_off, dst_stride);
    }
}

/// Copies a 16x8 byte block from `src` to `dst` (scalar reference).
#[cfg_attr(target_arch = "aarch64", allow(dead_code))]
pub(crate) fn copy_16x8_scalar(
    src: &[u8],
    src_off: usize,
    src_stride: usize,
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
) {
    for r in 0..8 {
        let s = src_off + r * src_stride;
        let d = dst_off + r * dst_stride;
        dst[d..d + 16].copy_from_slice(&src[s..s + 16]);
    }
}

/// Copies a 16x8 byte block from `src` to `dst`.
pub fn copy_16x8(
    src: &[u8],
    src_off: usize,
    src_stride: usize,
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
) {
    #[cfg(target_arch = "aarch64")]
    {
        super::dsp_neon::copy_16x8_neon(src, src_off, src_stride, dst, dst_off, dst_stride);
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        copy_16x8_scalar(src, src_off, src_stride, dst, dst_off, dst_stride);
    }
}

/// Copies an 8x8 byte block from `src` to `dst` (scalar reference).
pub(crate) fn copy_8x8_scalar(
    src: &[u8],
    src_off: usize,
    src_stride: usize,
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
) {
    for r in 0..8 {
        let s = src_off + r * src_stride;
        let d = dst_off + r * dst_stride;
        dst[d..d + 8].copy_from_slice(&src[s..s + 8]);
    }
}

/// Copies an 8x8 byte block from `src` to `dst`.
pub fn copy_8x8(
    src: &[u8],
    src_off: usize,
    src_stride: usize,
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
) {
    #[cfg(target_arch = "aarch64")]
    {
        super::dsp_neon::copy_8x8_neon(src, src_off, src_stride, dst, dst_off, dst_stride);
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        copy_8x8_scalar(src, src_off, src_stride, dst, dst_off, dst_stride);
    }
}

#[inline]
fn sample_clamped(src: &[u8], src_stride: usize, src_h: usize, x: i64, y: i64) -> u8 {
    let cx = x.clamp(0, (src_stride as i64) - 1) as usize;
    let cy = y.clamp(0, (src_h as i64) - 1) as usize;
    src[cy * src_stride + cx]
}

/// Exact 6-tap half-pel interpolation filter (`[1, -4, 19, 19, -4, 1] / 32` with per-pair `>> 1`).
#[inline]
fn filter6_i16(a: i16, b: i16, c: i16, d: i16, e: i16, f: i16) -> i16 {
    let t_af = a.wrapping_add(f) >> 1;
    let t_cd = c.wrapping_add(d).wrapping_mul(19) >> 1;
    let t_be = b.wrapping_add(e) << 1;
    t_af.wrapping_add(t_cd)
        .wrapping_sub(t_be)
        .wrapping_add(8)
        >> 4
}

/// 16x16 luma motion compensation with 6-tap half-pel interpolation (scalar reference).
#[allow(clippy::too_many_arguments)]
pub(crate) fn mc_luma_16x16_scalar(
    src: &[u8],
    src_stride: usize,
    src_x: i32,
    src_y: i32,
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
    hx: bool,
    hy: bool,
) {
    if src.is_empty() || src_stride == 0 {
        return;
    }
    let src_h = src.len() / src_stride;
    if src_h == 0 {
        return;
    }
    let sx0 = src_x as i64;
    let sy0 = src_y as i64;

    match (hx, hy) {
        (false, false) => {
            if src_x >= 0
                && src_y >= 0
                && (src_x as usize) + 16 <= src_stride
                && (src_y as usize) + 16 <= src_h
            {
                let src_off = (src_y as usize) * src_stride + (src_x as usize);
                copy_16x16_scalar(src, src_off, src_stride, dst, dst_off, dst_stride);
            } else {
                for r in 0..16 {
                    let d_off = dst_off + r * dst_stride;
                    let d_row = &mut dst[d_off..d_off + 16];
                    let y = sy0 + r as i64;
                    for c in 0..16 {
                        d_row[c] = sample_clamped(src, src_stride, src_h, sx0 + c as i64, y);
                    }
                }
            }
        }
        (false, true) => {
            if src_x >= 0
                && src_y >= 2
                && (src_x as usize) + 16 <= src_stride
                && (src_y as usize) + 19 <= src_h
            {
                let x0 = src_x as usize;
                let y0 = (src_y as usize) - 2;
                for r in 0..16 {
                    let base = (y0 + r) * src_stride + x0;
                    let r0 = &src[base..base + 16];
                    let r1 = &src[base + src_stride..base + src_stride + 16];
                    let r2 = &src[base + 2 * src_stride..base + 2 * src_stride + 16];
                    let r3 = &src[base + 3 * src_stride..base + 3 * src_stride + 16];
                    let r4 = &src[base + 4 * src_stride..base + 4 * src_stride + 16];
                    let r5 = &src[base + 5 * src_stride..base + 5 * src_stride + 16];
                    let d_off = dst_off + r * dst_stride;
                    let d_row = &mut dst[d_off..d_off + 16];
                    for c in 0..16 {
                        d_row[c] = filter6_i16(
                            r0[c] as i16,
                            r1[c] as i16,
                            r2[c] as i16,
                            r3[c] as i16,
                            r4[c] as i16,
                            r5[c] as i16,
                        )
                        .clamp(0, 255) as u8;
                    }
                }
            } else {
                for r in 0..16 {
                    let d_off = dst_off + r * dst_stride;
                    let d_row = &mut dst[d_off..d_off + 16];
                    let y = sy0 + r as i64;
                    for c in 0..16 {
                        let x = sx0 + c as i64;
                        let a = sample_clamped(src, src_stride, src_h, x, y - 2) as i16;
                        let b = sample_clamped(src, src_stride, src_h, x, y - 1) as i16;
                        let s0 = sample_clamped(src, src_stride, src_h, x, y) as i16;
                        let s1 = sample_clamped(src, src_stride, src_h, x, y + 1) as i16;
                        let e = sample_clamped(src, src_stride, src_h, x, y + 2) as i16;
                        let f = sample_clamped(src, src_stride, src_h, x, y + 3) as i16;
                        d_row[c] = filter6_i16(a, b, s0, s1, e, f).clamp(0, 255) as u8;
                    }
                }
            }
        }
        (true, false) => {
            if src_x >= 2
                && src_y >= 0
                && (src_x as usize) + 19 <= src_stride
                && (src_y as usize) + 16 <= src_h
            {
                let x0 = (src_x as usize) - 2;
                let y0 = src_y as usize;
                for r in 0..16 {
                    let base = (y0 + r) * src_stride + x0;
                    let s_row = &src[base..base + 21];
                    let d_off = dst_off + r * dst_stride;
                    let d_row = &mut dst[d_off..d_off + 16];
                    for c in 0..16 {
                        d_row[c] = filter6_i16(
                            s_row[c] as i16,
                            s_row[c + 1] as i16,
                            s_row[c + 2] as i16,
                            s_row[c + 3] as i16,
                            s_row[c + 4] as i16,
                            s_row[c + 5] as i16,
                        )
                        .clamp(0, 255) as u8;
                    }
                }
            } else {
                for r in 0..16 {
                    let d_off = dst_off + r * dst_stride;
                    let d_row = &mut dst[d_off..d_off + 16];
                    let y = sy0 + r as i64;
                    for c in 0..16 {
                        let x = sx0 + c as i64;
                        let a = sample_clamped(src, src_stride, src_h, x - 2, y) as i16;
                        let b = sample_clamped(src, src_stride, src_h, x - 1, y) as i16;
                        let s0 = sample_clamped(src, src_stride, src_h, x, y) as i16;
                        let s1 = sample_clamped(src, src_stride, src_h, x + 1, y) as i16;
                        let e = sample_clamped(src, src_stride, src_h, x + 2, y) as i16;
                        let f = sample_clamped(src, src_stride, src_h, x + 3, y) as i16;
                        d_row[c] = filter6_i16(a, b, s0, s1, e, f).clamp(0, 255) as u8;
                    }
                }
            }
        }
        (true, true) => {
            let mut tmp = [[0i16; 16]; 21];
            if src_x >= 2
                && src_y >= 2
                && (src_x as usize) + 19 <= src_stride
                && (src_y as usize) + 19 <= src_h
            {
                let x0 = (src_x as usize) - 2;
                let y0 = (src_y as usize) - 2;
                for ry in 0..21 {
                    let base = (y0 + ry) * src_stride + x0;
                    let s_row = &src[base..base + 21];
                    let t_row = &mut tmp[ry];
                    for c in 0..16 {
                        t_row[c] = filter6_i16(
                            s_row[c] as i16,
                            s_row[c + 1] as i16,
                            s_row[c + 2] as i16,
                            s_row[c + 3] as i16,
                            s_row[c + 4] as i16,
                            s_row[c + 5] as i16,
                        );
                    }
                }
            } else {
                for ry in 0..21 {
                    let y = sy0 + ry as i64 - 2;
                    let t_row = &mut tmp[ry];
                    for c in 0..16 {
                        let x = sx0 + c as i64;
                        let a = sample_clamped(src, src_stride, src_h, x - 2, y) as i16;
                        let b = sample_clamped(src, src_stride, src_h, x - 1, y) as i16;
                        let s0 = sample_clamped(src, src_stride, src_h, x, y) as i16;
                        let s1 = sample_clamped(src, src_stride, src_h, x + 1, y) as i16;
                        let e = sample_clamped(src, src_stride, src_h, x + 2, y) as i16;
                        let f = sample_clamped(src, src_stride, src_h, x + 3, y) as i16;
                        t_row[c] = filter6_i16(a, b, s0, s1, e, f);
                    }
                }
            }

            for r in 0..16 {
                let d_off = dst_off + r * dst_stride;
                let d_row = &mut dst[d_off..d_off + 16];
                for c in 0..16 {
                    d_row[c] = filter6_i16(
                        tmp[r][c],
                        tmp[r + 1][c],
                        tmp[r + 2][c],
                        tmp[r + 3][c],
                        tmp[r + 4][c],
                        tmp[r + 5][c],
                    )
                    .clamp(0, 255) as u8;
                }
            }
        }
    }
}

/// 16x16 luma motion compensation with 6-tap half-pel interpolation.
///
/// Out-of-bounds reference coordinates are defensively clamped to the source plane extents.
#[allow(clippy::too_many_arguments)]
pub fn mc_luma_16x16(
    src: &[u8],
    src_stride: usize,
    src_x: i32,
    src_y: i32,
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
    hx: bool,
    hy: bool,
) {
    #[cfg(target_arch = "aarch64")]
    {
        super::dsp_neon::mc_luma_16x16_neon(
            src, src_stride, src_x, src_y, dst, dst_off, dst_stride, hx, hy,
        );
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        mc_luma_16x16_scalar(
            src, src_stride, src_x, src_y, dst, dst_off, dst_stride, hx, hy,
        );
    }
}

/// Exact fixed-point `.4` 1D quarter-pel interpolation (`0..4` fraction).
#[inline]
fn qpel_lerp(a: i16, b: i16, frac: u32) -> i16 {
    match frac & 3 {
        0 => a,
        1 => a.wrapping_add(b.wrapping_sub(a) >> 2),
        2 => ((a as u16 as u32 + b as u16 as u32 + 1) >> 1) as i16,
        _ => b.wrapping_add(a.wrapping_sub(b) >> 2),
    }
}

/// 8x8 chroma quarter-pel bilinear motion compensation (scalar reference).
#[allow(clippy::too_many_arguments)]
pub(crate) fn mc_chroma_8x8_scalar(
    src: &[u8],
    src_stride: usize,
    src_x: i32,
    src_y: i32,
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
    dx: u32,
    dy: u32,
) {
    if src.is_empty() || src_stride == 0 {
        return;
    }
    let src_h = src.len() / src_stride;
    if src_h == 0 {
        return;
    }
    let dx = dx & 3;
    let dy = dy & 3;
    let sx0 = src_x as i64;
    let sy0 = src_y as i64;

    if dx == 0 && dy == 0 {
        if src_x >= 0
            && src_y >= 0
            && (src_x as usize) + 8 <= src_stride
            && (src_y as usize) + 8 <= src_h
        {
            let src_off = (src_y as usize) * src_stride + (src_x as usize);
            copy_8x8_scalar(src, src_off, src_stride, dst, dst_off, dst_stride);
        } else {
            for r in 0..8 {
                let d_off = dst_off + r * dst_stride;
                let d_row = &mut dst[d_off..d_off + 8];
                let y = sy0 + r as i64;
                for c in 0..8 {
                    d_row[c] = sample_clamped(src, src_stride, src_h, sx0 + c as i64, y);
                }
            }
        }
        return;
    }

    let need_x = if dx != 0 { 9 } else { 8 };
    let need_y = if dy != 0 { 9 } else { 8 };

    if src_x >= 0
        && src_y >= 0
        && (src_x as usize) + need_x <= src_stride
        && (src_y as usize) + need_y <= src_h
    {
        let x0 = src_x as usize;
        let y0 = src_y as usize;
        for r in 0..8 {
            let row0_base = (y0 + r) * src_stride + x0;
            let row1_base = if dy != 0 {
                row0_base + src_stride
            } else {
                row0_base
            };
            let s0 = &src[row0_base..row0_base + need_x];
            let s1 = &src[row1_base..row1_base + need_x];
            let d_off = dst_off + r * dst_stride;
            let d_row = &mut dst[d_off..d_off + 8];
            for c in 0..8 {
                let a = (s0[c] as i16) << 4;
                let b = if dx != 0 { (s0[c + 1] as i16) << 4 } else { a };
                let c0 = (s1[c] as i16) << 4;
                let d0 = if dx != 0 { (s1[c + 1] as i16) << 4 } else { c0 };
                let h0 = qpel_lerp(a, b, dx);
                let h1 = qpel_lerp(c0, d0, dx);
                let v = qpel_lerp(h0, h1, dy);
                d_row[c] = (v.wrapping_add(8) >> 4).clamp(0, 255) as u8;
            }
        }
    } else {
        for r in 0..8 {
            let d_off = dst_off + r * dst_stride;
            let d_row = &mut dst[d_off..d_off + 8];
            let y = sy0 + r as i64;
            for c in 0..8 {
                let x = sx0 + c as i64;
                let a = (sample_clamped(src, src_stride, src_h, x, y) as i16) << 4;
                let b = (sample_clamped(src, src_stride, src_h, x + 1, y) as i16) << 4;
                let c0 = (sample_clamped(src, src_stride, src_h, x, y + 1) as i16) << 4;
                let d0 = (sample_clamped(src, src_stride, src_h, x + 1, y + 1) as i16) << 4;
                let h0 = qpel_lerp(a, b, dx);
                let h1 = qpel_lerp(c0, d0, dx);
                let v = qpel_lerp(h0, h1, dy);
                d_row[c] = (v.wrapping_add(8) >> 4).clamp(0, 255) as u8;
            }
        }
    }
}

/// 8x8 chroma quarter-pel bilinear motion compensation.
///
/// `dx` and `dy` are X and Y quarter-pel fractions (`0..4`, masked with `& 3`).
/// Out-of-bounds reference coordinates are defensively clamped to the source plane extents.
#[allow(clippy::too_many_arguments)]
pub fn mc_chroma_8x8(
    src: &[u8],
    src_stride: usize,
    src_x: i32,
    src_y: i32,
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
    dx: u32,
    dy: u32,
) {
    #[cfg(target_arch = "aarch64")]
    {
        super::dsp_neon::mc_chroma_8x8_neon(
            src, src_stride, src_x, src_y, dst, dst_off, dst_stride, dx, dy,
        );
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        mc_chroma_8x8_scalar(
            src, src_stride, src_x, src_y, dst, dst_off, dst_stride, dx, dy,
        );
    }
}

/// Signed 16x16 -> high 16 bits (`PMULHW`).
#[cfg_attr(target_arch = "aarch64", allow(dead_code))]
#[inline]
fn pmulhw(a: i16, b: i16) -> i16 {
    ((a as i32 * b as i32) >> 16) as i16
}

/// 8-tap smooth DC interpolation kernel matching SSE2 `pmulhw` + `psraw`.
#[cfg_attr(target_arch = "aarch64", allow(dead_code))]
#[inline]
fn interp8(base: i16, d_prev: i16, d_next: i16) -> [i16; 8] {
    [
        base.wrapping_add(pmulhw(d_prev, 0x6800)),
        base.wrapping_add(pmulhw(d_prev, 0x4800))
            .wrapping_add(d_next >> 7),
        base.wrapping_add(pmulhw(d_prev, 0x3000))
            .wrapping_add(d_next >> 5),
        base.wrapping_add(pmulhw(d_prev, 0x1c00))
            .wrapping_add(d_next >> 4),
        base.wrapping_add(pmulhw(d_next, 0x1c00))
            .wrapping_add(d_prev >> 4),
        base.wrapping_add(pmulhw(d_next, 0x3000))
            .wrapping_add(d_prev >> 5),
        base.wrapping_add(pmulhw(d_next, 0x4800))
            .wrapping_add(d_prev >> 7),
        base.wrapping_add(pmulhw(d_next, 0x6800)),
    ]
}

#[cfg_attr(target_arch = "aarch64", allow(dead_code))]
#[inline]
fn gate_dc_neighbor(n_raw: i16, c_raw: i16, thresh: i16, fallback: i16) -> i16 {
    let d1 = n_raw.wrapping_sub(c_raw);
    let d2 = c_raw.wrapping_sub(n_raw);
    if d1.max(d2) < thresh {
        ((n_raw as u16) & 0x1fff) as i16
    } else {
        fallback
    }
}

#[allow(clippy::too_many_arguments)]
#[cfg_attr(target_arch = "aarch64", allow(dead_code))]
#[inline]
pub(crate) fn smooth_dc_block_scalar(
    row_above: [u16; 4],
    row_curr: [u16; 4],
    row_below: [u16; 4],
    k: usize,
    thresh: i16,
    dst: &mut [u8],
    block_off: usize,
    dst_stride: usize,
) {
    let c_raw = row_curr[k] as i16;
    let l_raw = row_curr[k - 1] as i16;
    let r_raw = row_curr[k + 1] as i16;
    let t_raw = row_above[k] as i16;
    let b_raw = row_below[k] as i16;
    let tl_raw = row_above[k - 1] as i16;
    let tr_raw = row_above[k + 1] as i16;
    let bl_raw = row_below[k - 1] as i16;
    let br_raw = row_below[k + 1] as i16;

    let c_dc = ((c_raw as u16) & 0x1fff) as i16;
    let l = gate_dc_neighbor(l_raw, c_raw, thresh, c_dc);
    let r = gate_dc_neighbor(r_raw, c_raw, thresh, c_dc);
    let t = gate_dc_neighbor(t_raw, c_raw, thresh, c_dc);
    let b = gate_dc_neighbor(b_raw, c_raw, thresh, c_dc);
    let tl = gate_dc_neighbor(tl_raw, c_raw, thresh, l);
    let tr = gate_dc_neighbor(tr_raw, c_raw, thresh, r);
    let bl = gate_dc_neighbor(bl_raw, c_raw, thresh, l);
    let br = gate_dc_neighbor(br_raw, c_raw, thresh, r);

    let row_c = interp8(c_dc.wrapping_add(4), l.wrapping_sub(c_dc), r.wrapping_sub(c_dc));
    let row_t = interp8(t.wrapping_add(4), tl.wrapping_sub(t), tr.wrapping_sub(t));
    let row_b = interp8(b.wrapping_add(4), bl.wrapping_sub(b), br.wrapping_sub(b));

    for x in 0..8 {
        let col = interp8(
            row_c[x],
            row_t[x].wrapping_sub(row_c[x]),
            row_b[x].wrapping_sub(row_c[x]),
        );
        for y in 0..8 {
            dst[block_off + y * dst_stride + x] = (col[y] >> 3).clamp(0, 255) as u8;
        }
    }
}

#[allow(clippy::too_many_arguments)]
#[inline]
fn smooth_dc_block(
    row_above: [u16; 4],
    row_curr: [u16; 4],
    row_below: [u16; 4],
    k: usize,
    thresh: i16,
    dst: &mut [u8],
    block_off: usize,
    dst_stride: usize,
) {
    #[cfg(target_arch = "aarch64")]
    {
        super::dsp_neon::smooth_dc_block_neon(
            row_above, row_curr, row_below, k, thresh, dst, block_off, dst_stride,
        );
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        smooth_dc_block_scalar(
            row_above, row_curr, row_below, k, thresh, dst, block_off, dst_stride,
        );
    }
}

/// Smooths a pair of adjacent DC-only 8x8 blocks using their 3x4 neighbor DC grid.
///
/// - `mask == 0`: no-op
/// - `mask & 1 != 0`: smooth left 8x8 block at `dst_off`
/// - `mask & 2 != 0`: smooth right 8x8 block at `dst_off + 8`
pub fn smooth_dc_pair(
    row_above: [u16; 4],
    row_curr: [u16; 4],
    row_below: [u16; 4],
    thresh: i16,
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
    mask: u32,
) {
    if (mask & 1) != 0 {
        smooth_dc_block(row_above, row_curr, row_below, 1, thresh, dst, dst_off, dst_stride);
    }
    if (mask & 2) != 0 {
        smooth_dc_block(
            row_above,
            row_curr,
            row_below,
            2,
            thresh,
            dst,
            dst_off + 8,
            dst_stride,
        );
    }
}

/// Vertical edge deblocking filter across 8 rows.
///
/// `dst_off` points to `(x_boundary, y_top)`; modifies columns `x - 2, x - 1, x, x + 1`.
pub fn deblock_vert_8(dst: &mut [u8], dst_off: usize, dst_stride: usize, weight_idx: usize) {
    let w = DEBLOCK_WEIGHTS[weight_idx & 15];
    if !w[0] && !w[1] && !w[2] && !w[3] {
        return;
    }
    for r in 0..8 {
        let p = dst_off + r * dst_stride;
        let d = dst[p] as i16 - dst[p - 1] as i16;
        let d1 = (d + 2) >> 2;
        let d2 = (d + 4) >> 3;
        if w[0] {
            dst[p - 2] = (dst[p - 2] as i16 + d2).clamp(0, 255) as u8;
        }
        if w[1] {
            dst[p - 1] = (dst[p - 1] as i16 + d1).clamp(0, 255) as u8;
        }
        if w[2] {
            dst[p] = (dst[p] as i16 - d1).clamp(0, 255) as u8;
        }
        if w[3] {
            dst[p + 1] = (dst[p + 1] as i16 - d2).clamp(0, 255) as u8;
        }
    }
}

/// Horizontal edge deblocking filter across 16 columns (scalar reference).
#[cfg_attr(target_arch = "aarch64", allow(dead_code))]
pub(crate) fn deblock_horiz_16_scalar(
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
    left_weight_idx: usize,
    right_weight_idx: usize,
) {
    let wl = DEBLOCK_WEIGHTS[left_weight_idx & 15];
    let wr = DEBLOCK_WEIGHTS[right_weight_idx & 15];

    let apply_half = |dst: &mut [u8], base: usize, w: [bool; 4]| {
        if !w[0] && !w[1] && !w[2] && !w[3] {
            return;
        }
        for c in 0..8 {
            let r0 = base + c;
            let r1 = r0 + dst_stride;
            let r2 = r1 + dst_stride;
            let r3 = r2 + dst_stride;
            let d = dst[r2] as i16 - dst[r1] as i16;
            let d1 = (d + 2) >> 2;
            let d2 = (d + 4) >> 3;
            if w[0] {
                dst[r0] = (dst[r0] as i16 + d2).clamp(0, 255) as u8;
            }
            if w[1] {
                dst[r1] = (dst[r1] as i16 + d1).clamp(0, 255) as u8;
            }
            if w[2] {
                dst[r2] = (dst[r2] as i16 - d1).clamp(0, 255) as u8;
            }
            if w[3] {
                dst[r3] = (dst[r3] as i16 - d2).clamp(0, 255) as u8;
            }
        }
    };

    apply_half(dst, dst_off, wl);
    apply_half(dst, dst_off + 8, wr);
}

/// Horizontal edge deblocking filter across 16 columns (two 8-column halves).
///
/// `dst_off` points to `(x_left, y_boundary - 2)`; modifies rows `y - 2, y - 1, y, y + 1`.
pub fn deblock_horiz_16(
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
    left_weight_idx: usize,
    right_weight_idx: usize,
) {
    #[cfg(target_arch = "aarch64")]
    {
        super::dsp_neon::deblock_horiz_16_neon(
            dst,
            dst_off,
            dst_stride,
            left_weight_idx,
            right_weight_idx,
        );
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        deblock_horiz_16_scalar(
            dst,
            dst_off,
            dst_stride,
            left_weight_idx,
            right_weight_idx,
        );
    }
}

#[inline]
fn should_smooth_inter(a: u16, b: u16) -> bool {
    if (a | b) <= 0x1fff {
        return false;
    }
    if (a & b) >= 0x6000 {
        let diff = (a ^ 0x1000).wrapping_sub(b ^ 0x1000) as i16 as i32;
        if diff.wrapping_add(0x20) as u32 <= 0x40 {
            return false;
        }
    }
    true
}

/// Smooths a vertical boundary between two 8x8 inter-residual blocks horizontally across 8 rows.
///
/// `off` points to `p[-2]` of the 4-element horizontal boundary (`[off, off+1, off+2, off+3]`).
pub fn smooth_inter_horiz(
    buf: &mut [i16],
    off: usize,
    stride: usize,
    info_left: u16,
    info_right: u16,
) {
    if !should_smooth_inter(info_left, info_right) {
        return;
    }
    let sl = ((info_left >> 13) & 3) as usize;
    let sr = ((info_right >> 13) & 3) as usize;
    let m0 = DEBLOCK_RESIDUAL_MASK[sl];
    let m1 = DEBLOCK_RESIDUAL_MASK[sl + 1];
    let m2 = DEBLOCK_RESIDUAL_MASK[sr + 1];
    let m3 = DEBLOCK_RESIDUAL_MASK[sr];
    if !m0 && !m1 && !m2 && !m3 {
        return;
    }

    for r in 0..8 {
        let p = off + r * stride;
        let d = buf[p + 2].wrapping_sub(buf[p + 1]);
        let d1 = d.wrapping_add(2) >> 2;
        let d2 = d.wrapping_add(4) >> 3;
        if m0 {
            buf[p] = buf[p].wrapping_add(d2);
        }
        if m1 {
            buf[p + 1] = buf[p + 1].wrapping_add(d1);
        }
        if m2 {
            buf[p + 2] = buf[p + 2].wrapping_sub(d1);
        }
        if m3 {
            buf[p + 3] = buf[p + 3].wrapping_sub(d2);
        }
    }
}

/// Smooths a horizontal boundary between two 8x8 inter-residual blocks vertically (scalar reference).
#[cfg_attr(target_arch = "aarch64", allow(dead_code))]
pub(crate) fn smooth_inter_vert_scalar(
    buf: &mut [i16],
    off: usize,
    stride: usize,
    info_top: u16,
    info_bot: u16,
) {
    if !should_smooth_inter(info_top, info_bot) {
        return;
    }
    let st = ((info_top >> 13) & 3) as usize;
    let sb = ((info_bot >> 13) & 3) as usize;
    let m0 = DEBLOCK_RESIDUAL_MASK[st];
    let m1 = DEBLOCK_RESIDUAL_MASK[st + 1];
    let m2 = DEBLOCK_RESIDUAL_MASK[sb + 1];
    let m3 = DEBLOCK_RESIDUAL_MASK[sb];
    if !m0 && !m1 && !m2 && !m3 {
        return;
    }

    for c in 0..8 {
        let r0 = off + c;
        let r1 = r0 + stride;
        let r2 = r1 + stride;
        let r3 = r2 + stride;
        let d = buf[r2].wrapping_sub(buf[r1]);
        let d1 = d.wrapping_add(2) >> 2;
        let d2 = d.wrapping_add(4) >> 3;
        if m0 {
            buf[r0] = buf[r0].wrapping_add(d2);
        }
        if m1 {
            buf[r1] = buf[r1].wrapping_add(d1);
        }
        if m2 {
            buf[r2] = buf[r2].wrapping_sub(d1);
        }
        if m3 {
            buf[r3] = buf[r3].wrapping_sub(d2);
        }
    }
}

/// Smooths a horizontal boundary between two 8x8 inter-residual blocks vertically across 8 columns.
///
/// `off` points to row `y_boundary - 2`; modifies rows `r0, r1, r2, r3` separated by `stride`.
pub fn smooth_inter_vert(
    buf: &mut [i16],
    off: usize,
    stride: usize,
    info_top: u16,
    info_bot: u16,
) {
    if !should_smooth_inter(info_top, info_bot) {
        return;
    }
    #[cfg(target_arch = "aarch64")]
    {
        super::dsp_neon::smooth_inter_vert_neon(buf, off, stride, info_top, info_bot);
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        smooth_inter_vert_scalar(buf, off, stride, info_top, info_bot);
    }
}

#[cfg_attr(target_arch = "aarch64", allow(dead_code))]
#[inline]
fn pavgb(a: u8, b: u8) -> u8 {
    ((a as u16 + b as u16 + 1) >> 1) as u8
}

/// Computes the 8x8 block average using the exact hierarchical `pavgb` tree (scalar reference).
#[cfg_attr(target_arch = "aarch64", allow(dead_code))]
pub(crate) fn avg_8x8_pavgb_scalar(src: &[u8], off: usize, stride: usize) -> i32 {
    let r0 = &src[off..off + 8];
    let r1 = &src[off + stride..off + stride + 8];
    let r2 = &src[off + 2 * stride..off + 2 * stride + 8];
    let r3 = &src[off + 3 * stride..off + 3 * stride + 8];
    let r4 = &src[off + 4 * stride..off + 4 * stride + 8];
    let r5 = &src[off + 5 * stride..off + 5 * stride + 8];
    let r6 = &src[off + 6 * stride..off + 6 * stride + 8];
    let r7 = &src[off + 7 * stride..off + 7 * stride + 8];

    let mut sum = 0i32;
    for c in 0..8 {
        let a01 = pavgb(r0[c], r1[c]);
        let a23 = pavgb(r2[c], r3[c]);
        let a45 = pavgb(r4[c], r5[c]);
        let a67 = pavgb(r6[c], r7[c]);
        let a03 = pavgb(a01, a23);
        let a47 = pavgb(a45, a67);
        sum += pavgb(a03, a47) as i32;
    }
    sum
}

/// Computes the 8x8 block average using the exact hierarchical `pavgb` tree (`0..=2040`).
pub fn avg_8x8_pavgb(src: &[u8], off: usize, stride: usize) -> i32 {
    #[cfg(target_arch = "aarch64")]
    {
        super::dsp_neon::avg_8x8_pavgb_neon(src, off, stride)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        avg_8x8_pavgb_scalar(src, off, stride)
    }
}

/// Computes `pavgb`-tree averages for two adjacent 8x8 blocks (16x8).
pub fn avg_16x8_pavgb(src: &[u8], off: usize, stride: usize) -> [i32; 2] {
    #[cfg(target_arch = "aarch64")]
    {
        super::dsp_neon::avg_16x8_pavgb_neon(src, off, stride)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        [
            avg_8x8_pavgb_scalar(src, off, stride),
            avg_8x8_pavgb_scalar(src, off + 8, stride),
        ]
    }
}

/// Computes `pavgb`-tree averages for four adjacent 8x8 blocks (32x8).
pub fn avg_32x8_pavgb(src: &[u8], off: usize, stride: usize) -> [i32; 4] {
    #[cfg(target_arch = "aarch64")]
    {
        super::dsp_neon::avg_32x8_pavgb_neon(src, off, stride)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        [
            avg_8x8_pavgb_scalar(src, off, stride),
            avg_8x8_pavgb_scalar(src, off + 8, stride),
            avg_8x8_pavgb_scalar(src, off + 16, stride),
            avg_8x8_pavgb_scalar(src, off + 24, stride),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idct8x8_dc_matches_fast_path() {
        for dc in [0i16, 63, 64, -1, -64, -65, 16320, -16384, i16::MAX, i16::MIN] {
            let mut coeffs = [0i16; 64];
            coeffs[0] = dc;
            let mut out_full = [0x5555i16; 16 * 10];
            let mut out_dc = [0x5555i16; 16 * 10];
            idct8x8(&coeffs, &mut out_full, 19, 16);
            idct8x8_dc(dc, &mut out_dc, 19, 16);
            assert_eq!(out_full, out_dc, "mismatch for dc={dc}");
            for y in 0..8 {
                for x in 0..8 {
                    assert_eq!(out_full[19 + y * 16 + x], dc >> 6);
                }
            }
            assert_eq!(out_full[18], 0x5555);
            assert_eq!(out_full[19 + 8], 0x5555);
        }
    }

    #[test]
    fn idct8x8_orientation_horizontal_and_vertical() {
        let expected_wave = [1i16, 1, 0, 0, -1, -1, -1, -2];

        // Pure horizontal frequency u=1, v=0 => coeffs[8] = 64
        let mut coeffs_h = [0i16; 64];
        coeffs_h[8] = 64;
        let mut out_h = [0i16; 64];
        idct8x8(&coeffs_h, &mut out_h, 0, 8);
        for y in 0..8 {
            assert_eq!(&out_h[y * 8..y * 8 + 8], &expected_wave);
        }

        // Pure vertical frequency u=0, v=1 => coeffs[1] = 64
        let mut coeffs_v = [0i16; 64];
        coeffs_v[1] = 64;
        let mut out_v = [0i16; 64];
        idct8x8(&coeffs_v, &mut out_v, 0, 8);
        for y in 0..8 {
            for x in 0..8 {
                assert_eq!(out_v[y * 8 + x], expected_wave[y]);
            }
        }

        // Pure horizontal frequency u=2, v=0 => coeffs[16] = 64
        let mut coeffs_h2 = [0i16; 64];
        coeffs_h2[16] = 64;
        let mut out_h2 = [0i16; 64];
        idct8x8(&coeffs_h2, &mut out_h2, 0, 8);
        let expected_h2 = [1i16, 0, -1, -2, -2, -1, 0, 1];
        for y in 0..8 {
            assert_eq!(&out_h2[y * 8..y * 8 + 8], &expected_h2);
        }
    }

    #[test]
    fn clamp_store_and_add_clamp_16x8() {
        let mut src = [0i16; 20 * 8];
        for r in 0..8 {
            for c in 0..16 {
                src[r * 20 + c] = match c {
                    0 => -500,
                    1 => -1,
                    2 => 0,
                    3 => 128,
                    4 => 255,
                    5 => 256,
                    6 => i16::MAX,
                    7 => i16::MIN,
                    _ => (r * 16 + c) as i16,
                };
            }
        }
        let mut dst = [0xAAu8; 24 * 8];
        clamp_store_16x8(&mut dst, 2, 24, &src, 0, 20);
        for r in 0..8 {
            let row = &dst[2 + r * 24..2 + r * 24 + 16];
            assert_eq!(row[0], 0);
            assert_eq!(row[1], 0);
            assert_eq!(row[2], 0);
            assert_eq!(row[3], 128);
            assert_eq!(row[4], 255);
            assert_eq!(row[5], 255);
            assert_eq!(row[6], 255);
            assert_eq!(row[7], 0);
            assert_eq!(dst[1 + r * 24], 0xAA);
        }

        let mut pred = [0u8; 24 * 8];
        let mut res = [0i16; 20 * 8];
        pred[2] = 200;
        res[0] = 100; // 300 -> 255
        pred[3] = 10;
        res[1] = -50; // -40 -> 0
        pred[4] = 100;
        res[2] = 23; // 123
        pred[5] = 1;
        res[3] = i16::MAX; // wrapping i16::MIN -> 0
        add_clamp_16x8(&mut dst, 2, 24, &res, 0, 20, &pred, 2, 24);
        assert_eq!(dst[2], 255);
        assert_eq!(dst[3], 0);
        assert_eq!(dst[4], 123);
        assert_eq!(dst[5], 0);
    }

    #[test]
    fn block_copies_respect_strides() {
        let mut src = vec![0u8; 32 * 24];
        for (i, b) in src.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(13).wrapping_add(7);
        }

        let mut dst16 = vec![0xEEu8; 32 * 24];
        copy_16x16(&src, 32 * 2 + 5, 32, &mut dst16, 32 * 3 + 4, 32);
        for r in 0..16 {
            assert_eq!(
                &dst16[(3 + r) * 32 + 4..(3 + r) * 32 + 20],
                &src[(2 + r) * 32 + 5..(2 + r) * 32 + 21]
            );
            assert_eq!(dst16[(3 + r) * 32 + 3], 0xEE);
            assert_eq!(dst16[(3 + r) * 32 + 20], 0xEE);
        }

        let mut dst16x8 = vec![0xDDu8; 24 * 12];
        copy_16x8(&src, 32 + 2, 32, &mut dst16x8, 24 + 3, 24);
        for r in 0..8 {
            assert_eq!(
                &dst16x8[(1 + r) * 24 + 3..(1 + r) * 24 + 19],
                &src[(1 + r) * 32 + 2..(1 + r) * 32 + 18]
            );
        }
        assert_eq!(dst16x8[9 * 24 + 3], 0xDD);

        let mut dst8 = vec![0xCCu8; 16 * 12];
        copy_8x8(&src, 32 + 1, 32, &mut dst8, 16 + 2, 16);
        for r in 0..8 {
            assert_eq!(
                &dst8[(1 + r) * 16 + 2..(1 + r) * 16 + 10],
                &src[(1 + r) * 32 + 1..(1 + r) * 32 + 9]
            );
        }
        assert_eq!(dst8[9 * 16 + 2], 0xCC);
    }

    #[test]
    fn mc_luma_16x16_constant_impulse_and_2d_unclamped() {
        // Constant plane invariant across all 4 modes and out-of-bounds coordinates.
        let const_src = vec![137u8; 32 * 32];
        let mut dst = vec![0u8; 16 * 16];
        for &(hx, hy) in &[(false, false), (false, true), (true, false), (true, true)] {
            for &(sx, sy) in &[(8, 8), (0, 0), (-5, -5), (30, 30)] {
                mc_luma_16x16(&const_src, 32, sx, sy, &mut dst, 0, 16, hx, hy);
                assert!(dst.iter().all(|&p| p == 137));
            }
        }

        // 1D horizontal impulse response on vertical stripe at x = 12 (value 100).
        let mut stripe_src = vec![0u8; 32 * 32];
        for y in 0..32 {
            stripe_src[y * 32 + 12] = 100;
        }
        mc_luma_16x16(&stripe_src, 32, 8, 4, &mut dst, 0, 16, true, false);
        for r in 0..16 {
            let row = &dst[r * 16..r * 16 + 16];
            assert_eq!(row[0], 0);
            assert_eq!(row[1], 3);
            assert_eq!(row[2], 0);
            assert_eq!(row[3], 59);
            assert_eq!(row[4], 59);
            assert_eq!(row[5], 0);
            assert_eq!(row[6], 3);
            assert_eq!(row[7], 0);
        }

        // 2D impulse at (12, 12) proves unclamped negative horizontal intermediate (`-12`)
        // feeds into vertical `-4` tap to produce `(-(-12)*2 + 8) >> 4 = 2` at (2, 2).
        let mut impulse_src = vec![0u8; 32 * 32];
        impulse_src[12 * 32 + 12] = 100;
        mc_luma_16x16(&impulse_src, 32, 8, 8, &mut dst, 0, 16, true, true);
        assert_eq!(dst[2 * 16 + 2], 2);
        assert_eq!(dst[3 * 16 + 3], 35);
        assert_eq!(dst[4 * 16 + 4], 35);
    }

    #[test]
    fn mc_chroma_8x8_exact_qpel_rounding() {
        let const_src = vec![91u8; 16 * 16];
        let mut dst = [0u8; 64];
        for dx in 0..4 {
            for dy in 0..4 {
                mc_chroma_8x8(&const_src, 16, 4, 4, &mut dst, 0, 8, dx, dy);
                assert!(dst.iter().all(|&p| p == 91));
                mc_chroma_8x8(&const_src, 16, -2, -2, &mut dst, 0, 8, dx, dy);
                assert!(dst.iter().all(|&p| p == 91));
            }
        }

        let mut src = vec![0u8; 16 * 16];
        src[0] = 0;
        src[1] = 100;
        src[16] = 200;
        src[17] = 255;

        mc_chroma_8x8(&src, 16, 0, 0, &mut dst, 0, 8, 1, 0);
        assert_eq!(dst[0], 25);
        mc_chroma_8x8(&src, 16, 0, 0, &mut dst, 0, 8, 2, 0);
        assert_eq!(dst[0], 50);
        mc_chroma_8x8(&src, 16, 0, 0, &mut dst, 0, 8, 3, 0);
        assert_eq!(dst[0], 75);
        mc_chroma_8x8(&src, 16, 0, 0, &mut dst, 0, 8, 1, 1);
        assert_eq!(dst[0], 72);
        mc_chroma_8x8(&src, 16, 0, 0, &mut dst, 0, 8, 2, 2);
        assert_eq!(dst[0], 139);
        mc_chroma_8x8(&src, 16, 0, 0, &mut dst, 0, 8, 3, 3);
        assert_eq!(dst[0], 200);
    }

    #[test]
    fn smooth_dc_pair_masks_gating_and_ramp() {
        let flat = [0x6000 | 800; 4];

        let mut dst = [0xAAu8; 16 * 8];
        smooth_dc_pair(flat, flat, flat, 256, &mut dst, 0, 16, 0);
        assert!(dst.iter().all(|&p| p == 0xAA));

        smooth_dc_pair(flat, flat, flat, 256, &mut dst, 0, 16, 1);
        for y in 0..8 {
            assert!(dst[y * 16..y * 16 + 8].iter().all(|&p| p == 100));
            assert!(dst[y * 16 + 8..y * 16 + 16].iter().all(|&p| p == 0xAA));
        }

        let mut dst2 = [0xAAu8; 16 * 8];
        smooth_dc_pair(flat, flat, flat, 256, &mut dst2, 0, 16, 2);
        for y in 0..8 {
            assert!(dst2[y * 16..y * 16 + 8].iter().all(|&p| p == 0xAA));
            assert!(dst2[y * 16 + 8..y * 16 + 16].iter().all(|&p| p == 100));
        }

        // Mode mismatch in upper 3 bits causes all neighbors to be gated out.
        let other_mode = [0x2000 | 1600; 4];
        let curr = [0x2000 | 1600, 0x6000 | 800, 0x6000 | 800, 0x2000 | 1600];
        let mut dst_gated = [0u8; 16 * 8];
        smooth_dc_pair(other_mode, curr, other_mode, 256, &mut dst_gated, 0, 16, 3);
        assert!(dst_gated.iter().all(|&p| p == 100));

        // Smooth horizontal DC ramp: [400, 800, 1200, 1600].
        let ramp = [0x6000 | 400, 0x6000 | 800, 0x6000 | 1200, 0x6000 | 1600];
        let mut dst_ramp = [0u8; 16 * 8];
        smooth_dc_pair(ramp, ramp, ramp, 600, &mut dst_ramp, 0, 16, 1);
        for y in 0..8 {
            let row = &dst_ramp[y * 16..y * 16 + 8];
            assert_eq!(row[0], 80);
            assert_eq!(row[7], 120);
            for x in 0..7 {
                assert!(row[x] < row[x + 1]);
            }
        }
    }

    #[test]
    fn deblock_vert_and_horiz_weights() {
        let mut buf = [0u8; 16 * 8];
        for r in 0..8 {
            for c in 0..8 {
                buf[r * 16 + c] = 100;
            }
            for c in 8..16 {
                buf[r * 16 + c] = 132;
            }
        }
        deblock_vert_8(&mut buf, 8, 16, 10); // [true; 4]
        for r in 0..8 {
            assert_eq!(buf[r * 16 + 6], 104);
            assert_eq!(buf[r * 16 + 7], 108);
            assert_eq!(buf[r * 16 + 8], 124);
            assert_eq!(buf[r * 16 + 9], 128);
        }

        let mut hbuf = [0u8; 16 * 4];
        hbuf[..32].fill(100);
        hbuf[32..].fill(132);
        // left_weight_idx = 10 ([T,T,T,T]), right_weight_idx = 4 ([F,F,T,F])
        deblock_horiz_16(&mut hbuf, 0, 16, 10, 4);
        for c in 0..8 {
            assert_eq!(hbuf[c], 104);
            assert_eq!(hbuf[16 + c], 108);
            assert_eq!(hbuf[32 + c], 124);
            assert_eq!(hbuf[48 + c], 128);
        }
        for c in 8..16 {
            assert_eq!(hbuf[c], 100);
            assert_eq!(hbuf[16 + c], 100);
            assert_eq!(hbuf[32 + c], 124);
            assert_eq!(hbuf[48 + c], 132);
        }
    }

    #[test]
    fn smooth_inter_guards_and_filtering() {
        assert!(!should_smooth_inter(0x1000, 0x1500)); // both sparsity 0
        assert!(!should_smooth_inter(0x6100, 0x6120)); // both sparsity 3, |diff| == 32
        assert!(!should_smooth_inter(0x6120, 0x6100)); // both sparsity 3, |diff| == 32
        assert!(should_smooth_inter(0x6100, 0x6121)); // |diff| == 33
        assert!(should_smooth_inter(0x6121, 0x6100)); // |diff| == 33

        // Horizontal inter smoothing: sl=1 ([F,T]), sr=2 ([T,T]) => masks [F, T, T, T].
        let mut buf = [0i16; 8 * 8];
        for r in 0..8 {
            buf[r * 8 + 2] = -40;
            buf[r * 8 + 3] = -40;
            buf[r * 8 + 4] = 40;
            buf[r * 8 + 5] = 40;
        }
        smooth_inter_horiz(&mut buf, 2, 8, 1 << 13, 2 << 13);
        for r in 0..8 {
            assert_eq!(buf[r * 8 + 2], -40);
            assert_eq!(buf[r * 8 + 3], -20);
            assert_eq!(buf[r * 8 + 4], 20);
            assert_eq!(buf[r * 8 + 5], 30);
        }

        // Vertical inter smoothing: st=2 ([T,T]), sb=1 ([T,F]) => masks [T, T, T, F].
        let mut vbuf = [0i16; 8 * 4];
        vbuf[0..16].fill(-40);
        vbuf[16..32].fill(40);
        smooth_inter_vert(&mut vbuf, 0, 8, 2 << 13, 1 << 13);
        for c in 0..8 {
            assert_eq!(vbuf[c], -30);
            assert_eq!(vbuf[8 + c], -20);
            assert_eq!(vbuf[16 + c], 20);
            assert_eq!(vbuf[24 + c], 40);
        }
    }

    #[test]
    fn avg_pavgb_hierarchical_tree() {
        let uniform = [73u8; 8 * 8];
        assert_eq!(avg_8x8_pavgb(&uniform, 0, 8), 73 * 8);

        // Alternating 0 and 1 rows: pavgb(0, 1) = 1 at leaf level => col_avg = 1 => sum = 8.
        let mut alt = [0u8; 8 * 8];
        for r in 0..8 {
            if r & 1 == 1 {
                alt[r * 8..r * 8 + 8].fill(1);
            }
        }
        assert_eq!(avg_8x8_pavgb(&alt, 0, 8), 8);

        let mut wide = [0u8; 32 * 8];
        for r in 0..8 {
            wide[r * 32..r * 32 + 8].fill(10);
            wide[r * 32 + 8..r * 32 + 16].fill(20);
            wide[r * 32 + 16..r * 32 + 24].fill(30);
            wide[r * 32 + 24..r * 32 + 32].fill(40);
        }
        assert_eq!(avg_16x8_pavgb(&wide, 0, 32), [80, 160]);
        assert_eq!(avg_32x8_pavgb(&wide, 0, 32), [80, 160, 240, 320]);
    }

    #[cfg(target_arch = "aarch64")]
    mod neon_equivalence {
        use super::super::super::dsp_neon;
        use super::*;

        struct XorShift64(u64);

        impl XorShift64 {
            fn next_u64(&mut self) -> u64 {
                let mut x = self.0;
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                self.0 = x;
                x
            }
            fn next_u8(&mut self) -> u8 {
                self.next_u64() as u8
            }
            fn next_i16(&mut self) -> i16 {
                self.next_u64() as i16
            }
            fn next_u16(&mut self) -> u16 {
                self.next_u64() as u16
            }
        }

        #[test]
        fn idct8x8_neon_matches_scalar_random_and_edges() {
            let mut rng = XorShift64(0xB14C_1DC7_0000_0001);
            let edge_blocks: Vec<[i16; 64]> = vec![
                [0; 64],
                [i16::MAX; 64],
                [i16::MIN; 64],
                [1; 64],
                [-1; 64],
                std::array::from_fn(|i| if i % 2 == 0 { i16::MAX } else { i16::MIN }),
                std::array::from_fn(|i| ((i as i16).wrapping_mul(1021)).wrapping_sub(16000)),
            ];

            for (case_idx, coeffs) in edge_blocks.iter().enumerate() {
                let mut out_s = [0x3333i16; 24 * 10];
                let mut out_n = [0x3333i16; 24 * 10];
                idct8x8_scalar(coeffs, &mut out_s, 17, 24);
                dsp_neon::idct8x8_neon(coeffs, &mut out_n, 17, 24);
                assert_eq!(out_n, out_s, "idct8x8 edge case {case_idx}");
            }

            for iter in 0..512 {
                let coeffs: [i16; 64] = std::array::from_fn(|_| {
                    if iter % 3 == 0 {
                        rng.next_i16()
                    } else {
                        (rng.next_i16() >> 4).wrapping_mul(3)
                    }
                });
                let stride = 8 + (iter % 19);
                let off = iter % 7;
                let len = off + 8 * stride + 8;
                let mut out_s = vec![0x7777i16; len];
                let mut out_n = vec![0x7777i16; len];
                idct8x8_scalar(&coeffs, &mut out_s, off, stride);
                dsp_neon::idct8x8_neon(&coeffs, &mut out_n, off, stride);
                assert_eq!(out_n, out_s, "idct8x8 random iter {iter}");
            }

            for dc in [0i16, 1, -1, 63, 64, -64, -65, i16::MIN, i16::MAX] {
                let mut out_s = [0x1234i16; 16 * 10];
                let mut out_n = [0x1234i16; 16 * 10];
                idct8x8_dc_scalar(dc, &mut out_s, 13, 16);
                dsp_neon::idct8x8_dc_neon(dc, &mut out_n, 13, 16);
                assert_eq!(out_n, out_s, "idct8x8_dc dc={dc}");
            }
        }

        #[test]
        fn clamp_and_add_clamp_neon_matches_scalar() {
            let mut rng = XorShift64(0xB14C_C1A4_0000_0002);
            for iter in 0..256 {
                let d_stride = 16 + (iter % 17);
                let s_stride = 16 + (iter % 13);
                let p_stride = 16 + (iter % 11);
                let d_off = iter % 5;
                let s_off = iter % 7;
                let p_off = iter % 3;

                let mut src: Vec<i16> = (0..s_off + 8 * s_stride + 16)
                    .map(|_| rng.next_i16())
                    .collect();
                if iter == 0 {
                    for (i, v) in [i16::MIN, -300, -1, 0, 1, 255, 256, i16::MAX]
                        .iter()
                        .enumerate()
                    {
                        src[s_off + i] = *v;
                    }
                }
                let pred: Vec<u8> = (0..p_off + 8 * p_stride + 16)
                    .map(|_| rng.next_u8())
                    .collect();

                let mut dst_s = vec![0x5Au8; d_off + 8 * d_stride + 16];
                let mut dst_n = dst_s.clone();
                clamp_store_16x8_scalar(&mut dst_s, d_off, d_stride, &src, s_off, s_stride);
                dsp_neon::clamp_store_16x8_neon(&mut dst_n, d_off, d_stride, &src, s_off, s_stride);
                assert_eq!(dst_n, dst_s, "clamp_store iter {iter}");

                dst_s.fill(0xA5);
                dst_n.fill(0xA5);
                add_clamp_16x8_scalar(
                    &mut dst_s, d_off, d_stride, &src, s_off, s_stride, &pred, p_off, p_stride,
                );
                dsp_neon::add_clamp_16x8_neon(
                    &mut dst_n, d_off, d_stride, &src, s_off, s_stride, &pred, p_off, p_stride,
                );
                assert_eq!(dst_n, dst_s, "add_clamp iter {iter}");
            }
        }

        #[test]
        fn mc_luma_16x16_neon_matches_scalar_all_modes_and_edges() {
            let mut rng = XorShift64(0xB14C_104A_0000_0003);
            let stride = 48usize;
            let height = 40usize;
            let mut src = vec![0u8; stride * height];
            for b in &mut src {
                *b = rng.next_u8();
            }

            // Extreme checkerboard pattern to stress filter6 rounding/clamping.
            let mut checker = vec![0u8; stride * height];
            for y in 0..height {
                for x in 0..stride {
                    checker[y * stride + x] = if (x ^ y) & 1 == 0 { 0 } else { 255 };
                }
            }

            let coords = [
                (8, 8),
                (2, 2),
                (0, 0),
                (1, 1),
                (-3, -2),
                (29, 21),
                (30, 22),
                (40, 35),
                (15, 11),
            ];

            for plane in [&src, &checker] {
                for &(hx, hy) in &[(false, false), (false, true), (true, false), (true, true)] {
                    for &(sx, sy) in &coords {
                        let dst_stride = 23usize;
                        let dst_off = 5usize;
                        let mut dst_s = vec![0xCCu8; dst_off + 16 * dst_stride + 16];
                        let mut dst_n = dst_s.clone();
                        mc_luma_16x16_scalar(
                            plane, stride, sx, sy, &mut dst_s, dst_off, dst_stride, hx, hy,
                        );
                        dsp_neon::mc_luma_16x16_neon(
                            plane, stride, sx, sy, &mut dst_n, dst_off, dst_stride, hx, hy,
                        );
                        assert_eq!(
                            dst_n, dst_s,
                            "mc_luma mismatch hx={hx} hy={hy} sx={sx} sy={sy}"
                        );
                    }
                }
            }
        }

        #[test]
        fn mc_chroma_8x8_neon_matches_scalar_all_qpel_and_edges() {
            let mut rng = XorShift64(0xB14C_C40A_0000_0004);
            let stride = 32usize;
            let height = 28usize;
            let mut src = vec![0u8; stride * height];
            for b in &mut src {
                *b = rng.next_u8();
            }
            let mut extremes = vec![0u8; stride * height];
            for (i, b) in extremes.iter_mut().enumerate() {
                *b = match i % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 254,
                    _ => 255,
                };
            }

            let coords = [(0, 0), (5, 7), (23, 19), (24, 20), (-2, 3), (30, 26)];
            for plane in [&src, &extremes] {
                for dx in 0..4 {
                    for dy in 0..4 {
                        for &(sx, sy) in &coords {
                            let dst_stride = 19usize;
                            let dst_off = 3usize;
                            let mut dst_s = vec![0x99u8; dst_off + 8 * dst_stride + 8];
                            let mut dst_n = dst_s.clone();
                            mc_chroma_8x8_scalar(
                                plane, stride, sx, sy, &mut dst_s, dst_off, dst_stride, dx, dy,
                            );
                            dsp_neon::mc_chroma_8x8_neon(
                                plane, stride, sx, sy, &mut dst_n, dst_off, dst_stride, dx, dy,
                            );
                            assert_eq!(
                                dst_n, dst_s,
                                "mc_chroma mismatch dx={dx} dy={dy} sx={sx} sy={sy}"
                            );
                        }
                    }
                }
            }
        }

        #[test]
        fn smooth_dc_block_neon_matches_scalar_random_and_edges() {
            let mut rng = XorShift64(0xB14C_DC50_0000_0005);
            for iter in 0..512 {
                let above: [u16; 4] = std::array::from_fn(|_| {
                    if iter % 4 == 0 {
                        rng.next_u16()
                    } else {
                        0x6000 | (rng.next_u16() & 0x07ff)
                    }
                });
                let curr: [u16; 4] = std::array::from_fn(|_| {
                    if iter % 4 == 0 {
                        rng.next_u16()
                    } else {
                        0x6000 | (rng.next_u16() & 0x07ff)
                    }
                });
                let below: [u16; 4] = std::array::from_fn(|_| {
                    if iter % 4 == 0 {
                        rng.next_u16()
                    } else {
                        0x6000 | (rng.next_u16() & 0x07ff)
                    }
                });
                let thresh = match iter % 5 {
                    0 => 0,
                    1 => 128,
                    2 => 512,
                    3 => 2040,
                    _ => i16::MAX,
                };
                for k in [1usize, 2] {
                    let stride = 16 + (iter % 13);
                    let off = iter % 9;
                    let mut dst_s = vec![0x42u8; off + 8 * stride + 8];
                    let mut dst_n = dst_s.clone();
                    smooth_dc_block_scalar(above, curr, below, k, thresh, &mut dst_s, off, stride);
                    dsp_neon::smooth_dc_block_neon(
                        above, curr, below, k, thresh, &mut dst_n, off, stride,
                    );
                    assert_eq!(dst_n, dst_s, "smooth_dc iter={iter} k={k} thresh={thresh}");
                }
            }
        }

        #[test]
        fn deblock_inter_smooth_and_pavgb_neon_match_scalar() {
            let mut rng = XorShift64(0xB14C_DEB0_0000_0006);

            // deblock_horiz_16 across all 16x16 weight pairs
            for lw in 0..16 {
                for rw in 0..16 {
                    let stride = 24usize;
                    let off = 3usize;
                    let mut buf_s: Vec<u8> =
                        (0..off + 4 * stride + 16).map(|_| rng.next_u8()).collect();
                    let mut buf_n = buf_s.clone();
                    deblock_horiz_16_scalar(&mut buf_s, off, stride, lw, rw);
                    dsp_neon::deblock_horiz_16_neon(&mut buf_n, off, stride, lw, rw);
                    assert_eq!(buf_n, buf_s, "deblock_horiz lw={lw} rw={rw}");
                }
            }

            // smooth_inter_vert across all sparsity combinations & wrapping i16 edges
            for st in 0..4u16 {
                for sb in 0..4u16 {
                    let info_top = (st << 13) | 0x0100;
                    let info_bot = (sb << 13) | 0x0500;
                    let stride = 13usize;
                    let off = 2usize;
                    let mut buf_s: Vec<i16> =
                        (0..off + 4 * stride + 8).map(|_| rng.next_i16()).collect();
                    buf_s[off + stride] = i16::MIN;
                    buf_s[off + 2 * stride] = i16::MAX;
                    let mut buf_n = buf_s.clone();
                    smooth_inter_vert_scalar(&mut buf_s, off, stride, info_top, info_bot);
                    if should_smooth_inter(info_top, info_bot) {
                        dsp_neon::smooth_inter_vert_neon(
                            &mut buf_n, off, stride, info_top, info_bot,
                        );
                    }
                    assert_eq!(buf_n, buf_s, "smooth_inter_vert st={st} sb={sb}");
                }
            }

            // avg_8x8 / 16x8 / 32x8 pavgb
            for iter in 0..128 {
                let stride = 32 + (iter % 17);
                let off = iter % 9;
                let src: Vec<u8> = (0..off + 8 * stride + 32).map(|_| rng.next_u8()).collect();
                assert_eq!(
                    dsp_neon::avg_8x8_pavgb_neon(&src, off, stride),
                    avg_8x8_pavgb_scalar(&src, off, stride),
                    "avg_8x8 iter={iter}"
                );
                assert_eq!(
                    dsp_neon::avg_16x8_pavgb_neon(&src, off, stride),
                    [
                        avg_8x8_pavgb_scalar(&src, off, stride),
                        avg_8x8_pavgb_scalar(&src, off + 8, stride),
                    ],
                    "avg_16x8 iter={iter}"
                );
                assert_eq!(
                    dsp_neon::avg_32x8_pavgb_neon(&src, off, stride),
                    [
                        avg_8x8_pavgb_scalar(&src, off, stride),
                        avg_8x8_pavgb_scalar(&src, off + 8, stride),
                        avg_8x8_pavgb_scalar(&src, off + 16, stride),
                        avg_8x8_pavgb_scalar(&src, off + 24, stride),
                    ],
                    "avg_32x8 iter={iter}"
                );
            }
        }
    }
}
