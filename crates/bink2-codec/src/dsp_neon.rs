//! ARM64 NEON implementations of Bink 2 DSP primitives.
//!
//! Every routine is bit-exact with its scalar reference counterpart in [`super::dsp`],
//! preserving `i16` wrapping semantics, rounding biases, arithmetic right shifts, and
//! clamping behavior across all inputs.

#![cfg(target_arch = "aarch64")]

use core::arch::aarch64::*;

use super::dsp::{mc_chroma_8x8_scalar, mc_luma_16x16_scalar};
use super::tables::{DEBLOCK_RESIDUAL_MASK, DEBLOCK_WEIGHTS};

/// 8-lane vectorized 1D 8-point integer IDCT butterfly.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
unsafe fn idct1d_neon(
    x0: int16x8_t,
    x1: int16x8_t,
    x2: int16x8_t,
    x3: int16x8_t,
    x4: int16x8_t,
    x5: int16x8_t,
    x6: int16x8_t,
    x7: int16x8_t,
) -> (
    int16x8_t,
    int16x8_t,
    int16x8_t,
    int16x8_t,
    int16x8_t,
    int16x8_t,
    int16x8_t,
    int16x8_t,
) {
    let a0 = vaddq_s16(x0, x4);
    let a1 = vsubq_s16(x0, x4);
    let a2 = vaddq_s16(vaddq_s16(vshrq_n_s16::<1>(x6), x2), vshrq_n_s16::<2>(x2));
    let a3 = vsubq_s16(vshrq_n_s16::<1>(x2), vaddq_s16(x6, vshrq_n_s16::<2>(x6)));
    let b0 = vaddq_s16(a0, a2);
    let b3 = vsubq_s16(a0, a2);
    let b1 = vaddq_s16(a1, a3);
    let b2 = vsubq_s16(a1, a3);

    let p0 = vaddq_s16(x3, x5);
    let p1 = vsubq_s16(x3, x5);
    let t1 = vaddq_s16(x1, p0);
    let t3 = vsubq_s16(x1, p0);
    let t7 = vaddq_s16(p1, x7);
    let t5 = vsubq_s16(p1, x7);

    let o0 = vaddq_s16(
        vshrq_n_s16::<2>(t7),
        vsubq_s16(vaddq_s16(t1, vshrq_n_s16::<2>(t1)), vshrq_n_s16::<4>(t1)),
    );
    let o3 = vsubq_s16(
        vshrq_n_s16::<2>(t1),
        vsubq_s16(vaddq_s16(t7, vshrq_n_s16::<2>(t7)), vshrq_n_s16::<4>(t7)),
    );
    let o1 = vaddq_s16(
        vsubq_s16(vsubq_s16(t5, vshrq_n_s16::<2>(t5)), vshrq_n_s16::<4>(t5)),
        t3,
    );
    let o2 = vsubq_s16(
        vsubq_s16(vsubq_s16(t3, vshrq_n_s16::<2>(t3)), vshrq_n_s16::<4>(t3)),
        t5,
    );

    (
        vaddq_s16(b0, o0),
        vaddq_s16(b1, o1),
        vaddq_s16(b2, o2),
        vaddq_s16(b3, o3),
        vsubq_s16(b3, o3),
        vsubq_s16(b2, o2),
        vsubq_s16(b1, o1),
        vsubq_s16(b0, o0),
    )
}

/// In-register 8x8 `int16x8_t` matrix transpose.
#[allow(clippy::too_many_arguments)]
#[inline(always)]
unsafe fn transpose_8x8_s16(
    r0: int16x8_t,
    r1: int16x8_t,
    r2: int16x8_t,
    r3: int16x8_t,
    r4: int16x8_t,
    r5: int16x8_t,
    r6: int16x8_t,
    r7: int16x8_t,
) -> (
    int16x8_t,
    int16x8_t,
    int16x8_t,
    int16x8_t,
    int16x8_t,
    int16x8_t,
    int16x8_t,
    int16x8_t,
) {
    let a0 = vreinterpretq_s32_s16(vtrn1q_s16(r0, r1));
    let a1 = vreinterpretq_s32_s16(vtrn2q_s16(r0, r1));
    let a2 = vreinterpretq_s32_s16(vtrn1q_s16(r2, r3));
    let a3 = vreinterpretq_s32_s16(vtrn2q_s16(r2, r3));
    let a4 = vreinterpretq_s32_s16(vtrn1q_s16(r4, r5));
    let a5 = vreinterpretq_s32_s16(vtrn2q_s16(r4, r5));
    let a6 = vreinterpretq_s32_s16(vtrn1q_s16(r6, r7));
    let a7 = vreinterpretq_s32_s16(vtrn2q_s16(r6, r7));

    let b0 = vreinterpretq_s64_s32(vtrn1q_s32(a0, a2));
    let b2 = vreinterpretq_s64_s32(vtrn2q_s32(a0, a2));
    let b1 = vreinterpretq_s64_s32(vtrn1q_s32(a1, a3));
    let b3 = vreinterpretq_s64_s32(vtrn2q_s32(a1, a3));
    let b4 = vreinterpretq_s64_s32(vtrn1q_s32(a4, a6));
    let b6 = vreinterpretq_s64_s32(vtrn2q_s32(a4, a6));
    let b5 = vreinterpretq_s64_s32(vtrn1q_s32(a5, a7));
    let b7 = vreinterpretq_s64_s32(vtrn2q_s32(a5, a7));

    (
        vreinterpretq_s16_s64(vtrn1q_s64(b0, b4)),
        vreinterpretq_s16_s64(vtrn1q_s64(b1, b5)),
        vreinterpretq_s16_s64(vtrn1q_s64(b2, b6)),
        vreinterpretq_s16_s64(vtrn1q_s64(b3, b7)),
        vreinterpretq_s16_s64(vtrn2q_s64(b0, b4)),
        vreinterpretq_s16_s64(vtrn2q_s64(b1, b5)),
        vreinterpretq_s16_s64(vtrn2q_s64(b2, b6)),
        vreinterpretq_s16_s64(vtrn2q_s64(b3, b7)),
    )
}

/// NEON 8x8 integer IDCT, bit-exact with [`super::dsp::idct8x8_scalar`].
pub(crate) fn idct8x8_neon(coeffs: &[i16; 64], out: &mut [i16], out_off: usize, out_stride: usize) {
    assert!(
        out_off + 7 * out_stride + 8 <= out.len(),
        "idct8x8 output slice out of bounds"
    );
    unsafe {
        let cp = coeffs.as_ptr();
        let r0 = vld1q_s16(cp);
        let r1 = vld1q_s16(cp.add(8));
        let r2 = vld1q_s16(cp.add(16));
        let r3 = vld1q_s16(cp.add(24));
        let r4 = vld1q_s16(cp.add(32));
        let r5 = vld1q_s16(cp.add(40));
        let r6 = vld1q_s16(cp.add(48));
        let r7 = vld1q_s16(cp.add(56));

        // Pass 1: horizontal frequency transform across all 8 vertical frequencies in parallel.
        let (p0, p1, p2, p3, p4, p5, p6, p7) = idct1d_neon(r0, r1, r2, r3, r4, r5, r6, r7);

        // Transpose so each register holds p1[v][0..8] (all 8 spatial X columns for a fixed v).
        let (c0, c1, c2, c3, c4, c5, c6, c7) = transpose_8x8_s16(p0, p1, p2, p3, p4, p5, p6, p7);

        // Pass 2: vertical frequency transform; outputs are directly in spatial [y][x] row order.
        let (y0, y1, y2, y3, y4, y5, y6, y7) = idct1d_neon(c0, c1, c2, c3, c4, c5, c6, c7);

        let dp = out.as_mut_ptr().add(out_off);
        vst1q_s16(dp, vshrq_n_s16::<6>(y0));
        vst1q_s16(dp.add(out_stride), vshrq_n_s16::<6>(y1));
        vst1q_s16(dp.add(out_stride * 2), vshrq_n_s16::<6>(y2));
        vst1q_s16(dp.add(out_stride * 3), vshrq_n_s16::<6>(y3));
        vst1q_s16(dp.add(out_stride * 4), vshrq_n_s16::<6>(y4));
        vst1q_s16(dp.add(out_stride * 5), vshrq_n_s16::<6>(y5));
        vst1q_s16(dp.add(out_stride * 6), vshrq_n_s16::<6>(y6));
        vst1q_s16(dp.add(out_stride * 7), vshrq_n_s16::<6>(y7));
    }
}

/// NEON DC-only 8x8 IDCT fill.
pub(crate) fn idct8x8_dc_neon(dc: i16, out: &mut [i16], out_off: usize, out_stride: usize) {
    assert!(
        out_off + 7 * out_stride + 8 <= out.len(),
        "idct8x8_dc output slice out of bounds"
    );
    unsafe {
        let v = vdupq_n_s16(dc >> 6);
        let dp = out.as_mut_ptr().add(out_off);
        vst1q_s16(dp, v);
        vst1q_s16(dp.add(out_stride), v);
        vst1q_s16(dp.add(out_stride * 2), v);
        vst1q_s16(dp.add(out_stride * 3), v);
        vst1q_s16(dp.add(out_stride * 4), v);
        vst1q_s16(dp.add(out_stride * 5), v);
        vst1q_s16(dp.add(out_stride * 6), v);
        vst1q_s16(dp.add(out_stride * 7), v);
    }
}

/// NEON 16x8 clamp (`0..=255`) and store.
pub(crate) fn clamp_store_16x8_neon(
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
    src: &[i16],
    src_off: usize,
    src_stride: usize,
) {
    assert!(dst_off + 7 * dst_stride + 16 <= dst.len());
    assert!(src_off + 7 * src_stride + 16 <= src.len());
    unsafe {
        let mut dp = dst.as_mut_ptr().add(dst_off);
        let mut sp = src.as_ptr().add(src_off);
        for _ in 0..8 {
            let lo = vld1q_s16(sp);
            let hi = vld1q_s16(sp.add(8));
            let packed = vcombine_u8(vqmovun_s16(lo), vqmovun_s16(hi));
            vst1q_u8(dp, packed);
            dp = dp.add(dst_stride);
            sp = sp.add(src_stride);
        }
    }
}

/// NEON 16x8 wrapping residual add, clamp (`0..=255`), and store.
#[allow(clippy::too_many_arguments)]
pub(crate) fn add_clamp_16x8_neon(
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
    assert!(dst_off + 7 * dst_stride + 16 <= dst.len());
    assert!(res_off + 7 * res_stride + 16 <= res.len());
    assert!(pred_off + 7 * pred_stride + 16 <= pred.len());
    unsafe {
        let mut dp = dst.as_mut_ptr().add(dst_off);
        let mut rp = res.as_ptr().add(res_off);
        let mut pp = pred.as_ptr().add(pred_off);
        for _ in 0..8 {
            let p = vld1q_u8(pp);
            let p_lo = vreinterpretq_s16_u16(vmovl_u8(vget_low_u8(p)));
            let p_hi = vreinterpretq_s16_u16(vmovl_high_u8(p));
            let r_lo = vld1q_s16(rp);
            let r_hi = vld1q_s16(rp.add(8));
            let s_lo = vaddq_s16(p_lo, r_lo);
            let s_hi = vaddq_s16(p_hi, r_hi);
            let packed = vcombine_u8(vqmovun_s16(s_lo), vqmovun_s16(s_hi));
            vst1q_u8(dp, packed);
            dp = dp.add(dst_stride);
            rp = rp.add(res_stride);
            pp = pp.add(pred_stride);
        }
    }
}

/// NEON 16x16 byte block copy.
pub(crate) fn copy_16x16_neon(
    src: &[u8],
    src_off: usize,
    src_stride: usize,
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
) {
    assert!(src_off + 15 * src_stride + 16 <= src.len());
    assert!(dst_off + 15 * dst_stride + 16 <= dst.len());
    unsafe {
        let mut sp = src.as_ptr().add(src_off);
        let mut dp = dst.as_mut_ptr().add(dst_off);
        for _ in 0..16 {
            vst1q_u8(dp, vld1q_u8(sp));
            sp = sp.add(src_stride);
            dp = dp.add(dst_stride);
        }
    }
}

/// NEON 16x8 byte block copy.
pub(crate) fn copy_16x8_neon(
    src: &[u8],
    src_off: usize,
    src_stride: usize,
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
) {
    assert!(src_off + 7 * src_stride + 16 <= src.len());
    assert!(dst_off + 7 * dst_stride + 16 <= dst.len());
    unsafe {
        let mut sp = src.as_ptr().add(src_off);
        let mut dp = dst.as_mut_ptr().add(dst_off);
        for _ in 0..8 {
            vst1q_u8(dp, vld1q_u8(sp));
            sp = sp.add(src_stride);
            dp = dp.add(dst_stride);
        }
    }
}

/// NEON 8x8 byte block copy.
pub(crate) fn copy_8x8_neon(
    src: &[u8],
    src_off: usize,
    src_stride: usize,
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
) {
    assert!(src_off + 7 * src_stride + 8 <= src.len());
    assert!(dst_off + 7 * dst_stride + 8 <= dst.len());
    unsafe {
        let mut sp = src.as_ptr().add(src_off);
        let mut dp = dst.as_mut_ptr().add(dst_off);
        for _ in 0..8 {
            vst1_u8(dp, vld1_u8(sp));
            sp = sp.add(src_stride);
            dp = dp.add(dst_stride);
        }
    }
}

/// Exact 8-lane NEON 6-tap half-pel filter (`[1, -4, 19, 19, -4, 1] / 32` with per-pair `>> 1`).
#[allow(clippy::too_many_arguments)]
#[inline(always)]
unsafe fn filter6_neon(
    a: int16x8_t,
    b: int16x8_t,
    c: int16x8_t,
    d: int16x8_t,
    e: int16x8_t,
    f: int16x8_t,
    c19: int16x8_t,
    c8: int16x8_t,
) -> int16x8_t {
    let t_af = vshrq_n_s16::<1>(vaddq_s16(a, f));
    let t_cd = vshrq_n_s16::<1>(vmulq_s16(vaddq_s16(c, d), c19));
    let t_be = vshlq_n_s16::<1>(vaddq_s16(b, e));
    vshrq_n_s16::<4>(vaddq_s16(vsubq_s16(vaddq_s16(t_af, t_cd), t_be), c8))
}

#[inline(always)]
unsafe fn load_row16_s16(ptr: *const u8) -> (int16x8_t, int16x8_t) {
    let v = vld1q_u8(ptr);
    (
        vreinterpretq_s16_u16(vmovl_u8(vget_low_u8(v))),
        vreinterpretq_s16_u16(vmovl_high_u8(v)),
    )
}

#[inline(always)]
unsafe fn filter6_horiz_row16(
    ptr: *const u8,
    c19: int16x8_t,
    c8: int16x8_t,
) -> (int16x8_t, int16x8_t) {
    let (a_lo, a_hi) = load_row16_s16(ptr);
    let (b_lo, b_hi) = load_row16_s16(ptr.add(1));
    let (c_lo, c_hi) = load_row16_s16(ptr.add(2));
    let (d_lo, d_hi) = load_row16_s16(ptr.add(3));
    let (e_lo, e_hi) = load_row16_s16(ptr.add(4));
    let (f_lo, f_hi) = load_row16_s16(ptr.add(5));
    (
        filter6_neon(a_lo, b_lo, c_lo, d_lo, e_lo, f_lo, c19, c8),
        filter6_neon(a_hi, b_hi, c_hi, d_hi, e_hi, f_hi, c19, c8),
    )
}

/// NEON 16x16 luma motion compensation with 6-tap half-pel interpolation.
#[allow(clippy::too_many_arguments)]
pub(crate) fn mc_luma_16x16_neon(
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

    match (hx, hy) {
        (false, false) => {
            if src_x >= 0
                && src_y >= 0
                && (src_x as usize) + 16 <= src_stride
                && (src_y as usize) + 16 <= src_h
            {
                let src_off = (src_y as usize) * src_stride + (src_x as usize);
                copy_16x16_neon(src, src_off, src_stride, dst, dst_off, dst_stride);
                return;
            }
        }
        (false, true) => {
            if src_x >= 0
                && src_y >= 2
                && (src_x as usize) + 16 <= src_stride
                && (src_y as usize) + 19 <= src_h
            {
                assert!(dst_off + 15 * dst_stride + 16 <= dst.len());
                let x0 = src_x as usize;
                let y0 = (src_y as usize) - 2;
                unsafe {
                    let c19 = vdupq_n_s16(19);
                    let c8 = vdupq_n_s16(8);
                    let mut sp = src.as_ptr().add(y0 * src_stride + x0);
                    let mut dp = dst.as_mut_ptr().add(dst_off);

                    let (mut r0_lo, mut r0_hi) = load_row16_s16(sp);
                    sp = sp.add(src_stride);
                    let (mut r1_lo, mut r1_hi) = load_row16_s16(sp);
                    sp = sp.add(src_stride);
                    let (mut r2_lo, mut r2_hi) = load_row16_s16(sp);
                    sp = sp.add(src_stride);
                    let (mut r3_lo, mut r3_hi) = load_row16_s16(sp);
                    sp = sp.add(src_stride);
                    let (mut r4_lo, mut r4_hi) = load_row16_s16(sp);
                    sp = sp.add(src_stride);

                    for _ in 0..16 {
                        let (r5_lo, r5_hi) = load_row16_s16(sp);
                        sp = sp.add(src_stride);
                        let out_lo =
                            filter6_neon(r0_lo, r1_lo, r2_lo, r3_lo, r4_lo, r5_lo, c19, c8);
                        let out_hi =
                            filter6_neon(r0_hi, r1_hi, r2_hi, r3_hi, r4_hi, r5_hi, c19, c8);
                        vst1q_u8(dp, vcombine_u8(vqmovun_s16(out_lo), vqmovun_s16(out_hi)));
                        dp = dp.add(dst_stride);

                        r0_lo = r1_lo;
                        r0_hi = r1_hi;
                        r1_lo = r2_lo;
                        r1_hi = r2_hi;
                        r2_lo = r3_lo;
                        r2_hi = r3_hi;
                        r3_lo = r4_lo;
                        r3_hi = r4_hi;
                        r4_lo = r5_lo;
                        r4_hi = r5_hi;
                    }
                }
                return;
            }
        }
        (true, false) => {
            if src_x >= 2
                && src_y >= 0
                && (src_x as usize) + 19 <= src_stride
                && (src_y as usize) + 16 <= src_h
            {
                assert!(dst_off + 15 * dst_stride + 16 <= dst.len());
                let x0 = (src_x as usize) - 2;
                let y0 = src_y as usize;
                unsafe {
                    let c19 = vdupq_n_s16(19);
                    let c8 = vdupq_n_s16(8);
                    let mut sp = src.as_ptr().add(y0 * src_stride + x0);
                    let mut dp = dst.as_mut_ptr().add(dst_off);
                    for _ in 0..16 {
                        let (out_lo, out_hi) = filter6_horiz_row16(sp, c19, c8);
                        vst1q_u8(dp, vcombine_u8(vqmovun_s16(out_lo), vqmovun_s16(out_hi)));
                        sp = sp.add(src_stride);
                        dp = dp.add(dst_stride);
                    }
                }
                return;
            }
        }
        (true, true) => {
            if src_x >= 2
                && src_y >= 2
                && (src_x as usize) + 19 <= src_stride
                && (src_y as usize) + 19 <= src_h
            {
                assert!(dst_off + 15 * dst_stride + 16 <= dst.len());
                let x0 = (src_x as usize) - 2;
                let y0 = (src_y as usize) - 2;
                unsafe {
                    let c19 = vdupq_n_s16(19);
                    let c8 = vdupq_n_s16(8);
                    let mut sp = src.as_ptr().add(y0 * src_stride + x0);
                    let mut dp = dst.as_mut_ptr().add(dst_off);

                    let (mut t0_lo, mut t0_hi) = filter6_horiz_row16(sp, c19, c8);
                    sp = sp.add(src_stride);
                    let (mut t1_lo, mut t1_hi) = filter6_horiz_row16(sp, c19, c8);
                    sp = sp.add(src_stride);
                    let (mut t2_lo, mut t2_hi) = filter6_horiz_row16(sp, c19, c8);
                    sp = sp.add(src_stride);
                    let (mut t3_lo, mut t3_hi) = filter6_horiz_row16(sp, c19, c8);
                    sp = sp.add(src_stride);
                    let (mut t4_lo, mut t4_hi) = filter6_horiz_row16(sp, c19, c8);
                    sp = sp.add(src_stride);

                    for _ in 0..16 {
                        let (t5_lo, t5_hi) = filter6_horiz_row16(sp, c19, c8);
                        sp = sp.add(src_stride);
                        let out_lo =
                            filter6_neon(t0_lo, t1_lo, t2_lo, t3_lo, t4_lo, t5_lo, c19, c8);
                        let out_hi =
                            filter6_neon(t0_hi, t1_hi, t2_hi, t3_hi, t4_hi, t5_hi, c19, c8);
                        vst1q_u8(dp, vcombine_u8(vqmovun_s16(out_lo), vqmovun_s16(out_hi)));
                        dp = dp.add(dst_stride);

                        t0_lo = t1_lo;
                        t0_hi = t1_hi;
                        t1_lo = t2_lo;
                        t1_hi = t2_hi;
                        t2_lo = t3_lo;
                        t2_hi = t3_hi;
                        t3_lo = t4_lo;
                        t3_hi = t4_hi;
                        t4_lo = t5_lo;
                        t4_hi = t5_hi;
                    }
                }
                return;
            }
        }
    }

    mc_luma_16x16_scalar(
        src, src_stride, src_x, src_y, dst, dst_off, dst_stride, hx, hy,
    );
}

#[inline(always)]
unsafe fn qpel_lerp_neon<const FRAC: u32>(a: int16x8_t, b: int16x8_t) -> int16x8_t {
    match FRAC {
        0 => a,
        1 => vaddq_s16(a, vshrq_n_s16::<2>(vsubq_s16(b, a))),
        2 => vreinterpretq_s16_u16(vrhaddq_u16(
            vreinterpretq_u16_s16(a),
            vreinterpretq_u16_s16(b),
        )),
        _ => vaddq_s16(b, vshrq_n_s16::<2>(vsubq_s16(a, b))),
    }
}

#[inline(always)]
unsafe fn chroma_horiz_row<const DX: u32>(ptr: *const u8) -> int16x8_t {
    let a = vshlq_n_s16::<4>(vreinterpretq_s16_u16(vmovl_u8(vld1_u8(ptr))));
    if DX == 0 {
        a
    } else {
        let b = vshlq_n_s16::<4>(vreinterpretq_s16_u16(vmovl_u8(vld1_u8(ptr.add(1)))));
        qpel_lerp_neon::<DX>(a, b)
    }
}

#[inline(always)]
unsafe fn mc_chroma_8x8_kernel<const DX: u32, const DY: u32>(
    mut sp: *const u8,
    src_stride: usize,
    mut dp: *mut u8,
    dst_stride: usize,
) {
    let c8 = vdupq_n_s16(8);
    if DY == 0 {
        for _ in 0..8 {
            let v = chroma_horiz_row::<DX>(sp);
            let out = vqmovun_s16(vshrq_n_s16::<4>(vaddq_s16(v, c8)));
            vst1_u8(dp, out);
            sp = sp.add(src_stride);
            dp = dp.add(dst_stride);
        }
    } else {
        let mut h0 = chroma_horiz_row::<DX>(sp);
        sp = sp.add(src_stride);
        for _ in 0..8 {
            let h1 = chroma_horiz_row::<DX>(sp);
            sp = sp.add(src_stride);
            let v = qpel_lerp_neon::<DY>(h0, h1);
            let out = vqmovun_s16(vshrq_n_s16::<4>(vaddq_s16(v, c8)));
            vst1_u8(dp, out);
            dp = dp.add(dst_stride);
            h0 = h1;
        }
    }
}

/// NEON 8x8 chroma quarter-pel bilinear motion compensation.
#[allow(clippy::too_many_arguments)]
pub(crate) fn mc_chroma_8x8_neon(
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

    if dx == 0 && dy == 0 {
        if src_x >= 0
            && src_y >= 0
            && (src_x as usize) + 8 <= src_stride
            && (src_y as usize) + 8 <= src_h
        {
            let src_off = (src_y as usize) * src_stride + (src_x as usize);
            copy_8x8_neon(src, src_off, src_stride, dst, dst_off, dst_stride);
            return;
        }
        mc_chroma_8x8_scalar(
            src, src_stride, src_x, src_y, dst, dst_off, dst_stride, dx, dy,
        );
        return;
    }

    let need_x = if dx != 0 { 9 } else { 8 };
    let need_y = if dy != 0 { 9 } else { 8 };
    if src_x >= 0
        && src_y >= 0
        && (src_x as usize) + need_x <= src_stride
        && (src_y as usize) + need_y <= src_h
    {
        assert!(dst_off + 7 * dst_stride + 8 <= dst.len());
        let sp = unsafe {
            src.as_ptr()
                .add((src_y as usize) * src_stride + (src_x as usize))
        };
        let dp = unsafe { dst.as_mut_ptr().add(dst_off) };
        unsafe {
            match (dx, dy) {
                (0, 1) => mc_chroma_8x8_kernel::<0, 1>(sp, src_stride, dp, dst_stride),
                (0, 2) => mc_chroma_8x8_kernel::<0, 2>(sp, src_stride, dp, dst_stride),
                (0, 3) => mc_chroma_8x8_kernel::<0, 3>(sp, src_stride, dp, dst_stride),
                (1, 0) => mc_chroma_8x8_kernel::<1, 0>(sp, src_stride, dp, dst_stride),
                (1, 1) => mc_chroma_8x8_kernel::<1, 1>(sp, src_stride, dp, dst_stride),
                (1, 2) => mc_chroma_8x8_kernel::<1, 2>(sp, src_stride, dp, dst_stride),
                (1, 3) => mc_chroma_8x8_kernel::<1, 3>(sp, src_stride, dp, dst_stride),
                (2, 0) => mc_chroma_8x8_kernel::<2, 0>(sp, src_stride, dp, dst_stride),
                (2, 1) => mc_chroma_8x8_kernel::<2, 1>(sp, src_stride, dp, dst_stride),
                (2, 2) => mc_chroma_8x8_kernel::<2, 2>(sp, src_stride, dp, dst_stride),
                (2, 3) => mc_chroma_8x8_kernel::<2, 3>(sp, src_stride, dp, dst_stride),
                (3, 0) => mc_chroma_8x8_kernel::<3, 0>(sp, src_stride, dp, dst_stride),
                (3, 1) => mc_chroma_8x8_kernel::<3, 1>(sp, src_stride, dp, dst_stride),
                (3, 2) => mc_chroma_8x8_kernel::<3, 2>(sp, src_stride, dp, dst_stride),
                (3, 3) => mc_chroma_8x8_kernel::<3, 3>(sp, src_stride, dp, dst_stride),
                _ => unreachable!(),
            }
        }
        return;
    }

    mc_chroma_8x8_scalar(
        src, src_stride, src_x, src_y, dst, dst_off, dst_stride, dx, dy,
    );
}

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

/// Half-scaled `interp8` weights for `vqdmulhq_s16` (`(d * w_half * 2) >> 16 == (d * w) >> 16`).
/// Exact for every `i16` because `w_half > 0` (never `-32768`).
const INTERP8_W_PREV_HALF: [i16; 8] = [
    0x3400, 0x2400, 0x1800, 0x0e00, 0x0800, 0x0400, 0x0100, 0x0000,
];
const INTERP8_W_NEXT_HALF: [i16; 8] = [
    0x0000, 0x0100, 0x0400, 0x0800, 0x0e00, 0x1800, 0x2400, 0x3400,
];

#[inline(always)]
unsafe fn interp8_horiz_neon(
    base: i16,
    d_prev: i16,
    d_next: i16,
    w_prev: int16x8_t,
    w_next: int16x8_t,
) -> int16x8_t {
    vaddq_s16(
        vaddq_s16(vdupq_n_s16(base), vqdmulhq_s16(vdupq_n_s16(d_prev), w_prev)),
        vqdmulhq_s16(vdupq_n_s16(d_next), w_next),
    )
}

/// NEON smooth DC block interpolation, bit-exact with [`super::dsp::smooth_dc_block_scalar`].
#[allow(clippy::too_many_arguments)]
pub(crate) fn smooth_dc_block_neon(
    row_above: [u16; 4],
    row_curr: [u16; 4],
    row_below: [u16; 4],
    k: usize,
    thresh: i16,
    dst: &mut [u8],
    block_off: usize,
    dst_stride: usize,
) {
    assert!(block_off + 7 * dst_stride + 8 <= dst.len());
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

    unsafe {
        let w_prev = vld1q_s16(INTERP8_W_PREV_HALF.as_ptr());
        let w_next = vld1q_s16(INTERP8_W_NEXT_HALF.as_ptr());

        let vc = interp8_horiz_neon(
            c_dc.wrapping_add(4),
            l.wrapping_sub(c_dc),
            r.wrapping_sub(c_dc),
            w_prev,
            w_next,
        );
        let vt = interp8_horiz_neon(
            t.wrapping_add(4),
            tl.wrapping_sub(t),
            tr.wrapping_sub(t),
            w_prev,
            w_next,
        );
        let vb = interp8_horiz_neon(
            b.wrapping_add(4),
            bl.wrapping_sub(b),
            br.wrapping_sub(b),
            w_prev,
            w_next,
        );

        let dt = vsubq_s16(vt, vc);
        let db = vsubq_s16(vb, vc);

        let y0 = vaddq_s16(vc, vqdmulhq_n_s16(dt, 0x3400));
        let y1 = vaddq_s16(
            vaddq_s16(vc, vqdmulhq_n_s16(dt, 0x2400)),
            vshrq_n_s16::<7>(db),
        );
        let y2 = vaddq_s16(
            vaddq_s16(vc, vqdmulhq_n_s16(dt, 0x1800)),
            vshrq_n_s16::<5>(db),
        );
        let y3 = vaddq_s16(
            vaddq_s16(vc, vqdmulhq_n_s16(dt, 0x0e00)),
            vshrq_n_s16::<4>(db),
        );
        let y4 = vaddq_s16(
            vaddq_s16(vc, vqdmulhq_n_s16(db, 0x0e00)),
            vshrq_n_s16::<4>(dt),
        );
        let y5 = vaddq_s16(
            vaddq_s16(vc, vqdmulhq_n_s16(db, 0x1800)),
            vshrq_n_s16::<5>(dt),
        );
        let y6 = vaddq_s16(
            vaddq_s16(vc, vqdmulhq_n_s16(db, 0x2400)),
            vshrq_n_s16::<7>(dt),
        );
        let y7 = vaddq_s16(vc, vqdmulhq_n_s16(db, 0x3400));

        let dp = dst.as_mut_ptr().add(block_off);
        vst1_u8(dp, vqmovun_s16(vshrq_n_s16::<3>(y0)));
        vst1_u8(dp.add(dst_stride), vqmovun_s16(vshrq_n_s16::<3>(y1)));
        vst1_u8(dp.add(dst_stride * 2), vqmovun_s16(vshrq_n_s16::<3>(y2)));
        vst1_u8(dp.add(dst_stride * 3), vqmovun_s16(vshrq_n_s16::<3>(y3)));
        vst1_u8(dp.add(dst_stride * 4), vqmovun_s16(vshrq_n_s16::<3>(y4)));
        vst1_u8(dp.add(dst_stride * 5), vqmovun_s16(vshrq_n_s16::<3>(y5)));
        vst1_u8(dp.add(dst_stride * 6), vqmovun_s16(vshrq_n_s16::<3>(y6)));
        vst1_u8(dp.add(dst_stride * 7), vqmovun_s16(vshrq_n_s16::<3>(y7)));
    }
}

/// NEON horizontal edge deblocking filter across 16 columns (two 8-column halves).
pub(crate) fn deblock_horiz_16_neon(
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
    left_weight_idx: usize,
    right_weight_idx: usize,
) {
    let wl = DEBLOCK_WEIGHTS[left_weight_idx & 15];
    let wr = DEBLOCK_WEIGHTS[right_weight_idx & 15];
    let do_left = wl[0] || wl[1] || wl[2] || wl[3];
    let do_right = wr[0] || wr[1] || wr[2] || wr[3];
    if !do_left && !do_right {
        return;
    }

    #[inline(always)]
    unsafe fn apply_half(base: *mut u8, stride: usize, w: [bool; 4]) {
        let p0 = base;
        let p1 = p0.add(stride);
        let p2 = p1.add(stride);
        let p3 = p2.add(stride);
        let r1 = vreinterpretq_s16_u16(vmovl_u8(vld1_u8(p1)));
        let r2 = vreinterpretq_s16_u16(vmovl_u8(vld1_u8(p2)));
        let d = vsubq_s16(r2, r1);
        if w[0] || w[3] {
            let d2 = vshrq_n_s16::<3>(vaddq_s16(d, vdupq_n_s16(4)));
            if w[0] {
                let r0 = vreinterpretq_s16_u16(vmovl_u8(vld1_u8(p0)));
                vst1_u8(p0, vqmovun_s16(vaddq_s16(r0, d2)));
            }
            if w[3] {
                let r3 = vreinterpretq_s16_u16(vmovl_u8(vld1_u8(p3)));
                vst1_u8(p3, vqmovun_s16(vsubq_s16(r3, d2)));
            }
        }
        if w[1] || w[2] {
            let d1 = vshrq_n_s16::<2>(vaddq_s16(d, vdupq_n_s16(2)));
            if w[1] {
                vst1_u8(p1, vqmovun_s16(vaddq_s16(r1, d1)));
            }
            if w[2] {
                vst1_u8(p2, vqmovun_s16(vsubq_s16(r2, d1)));
            }
        }
    }

    if do_left {
        assert!(dst_off + 3 * dst_stride + 8 <= dst.len());
        unsafe { apply_half(dst.as_mut_ptr().add(dst_off), dst_stride, wl) };
    }
    if do_right {
        assert!(dst_off + 8 + 3 * dst_stride + 8 <= dst.len());
        unsafe { apply_half(dst.as_mut_ptr().add(dst_off + 8), dst_stride, wr) };
    }
}

/// NEON vertical inter-residual boundary smoother across 8 columns.
pub(crate) fn smooth_inter_vert_neon(
    buf: &mut [i16],
    off: usize,
    stride: usize,
    info_top: u16,
    info_bot: u16,
) {
    let st = ((info_top >> 13) & 3) as usize;
    let sb = ((info_bot >> 13) & 3) as usize;
    let m0 = DEBLOCK_RESIDUAL_MASK[st];
    let m1 = DEBLOCK_RESIDUAL_MASK[st + 1];
    let m2 = DEBLOCK_RESIDUAL_MASK[sb + 1];
    let m3 = DEBLOCK_RESIDUAL_MASK[sb];
    if !m0 && !m1 && !m2 && !m3 {
        return;
    }
    assert!(off + 3 * stride + 8 <= buf.len());
    unsafe {
        let p0 = buf.as_mut_ptr().add(off);
        let p1 = p0.add(stride);
        let p2 = p1.add(stride);
        let p3 = p2.add(stride);
        let r1 = vld1q_s16(p1);
        let r2 = vld1q_s16(p2);
        let d = vsubq_s16(r2, r1);
        if m0 || m3 {
            let d2 = vshrq_n_s16::<3>(vaddq_s16(d, vdupq_n_s16(4)));
            if m0 {
                vst1q_s16(p0, vaddq_s16(vld1q_s16(p0), d2));
            }
            if m3 {
                vst1q_s16(p3, vsubq_s16(vld1q_s16(p3), d2));
            }
        }
        if m1 || m2 {
            let d1 = vshrq_n_s16::<2>(vaddq_s16(d, vdupq_n_s16(2)));
            if m1 {
                vst1q_s16(p1, vaddq_s16(r1, d1));
            }
            if m2 {
                vst1q_s16(p2, vsubq_s16(r2, d1));
            }
        }
    }
}

/// NEON hierarchical `pavgb` (`vrhadd_u8`) 8x8 block average (`0..=2040`).
pub(crate) fn avg_8x8_pavgb_neon(src: &[u8], off: usize, stride: usize) -> i32 {
    assert!(off + 7 * stride + 8 <= src.len());
    unsafe {
        let sp = src.as_ptr().add(off);
        let a01 = vrhadd_u8(vld1_u8(sp), vld1_u8(sp.add(stride)));
        let a23 = vrhadd_u8(vld1_u8(sp.add(stride * 2)), vld1_u8(sp.add(stride * 3)));
        let a45 = vrhadd_u8(vld1_u8(sp.add(stride * 4)), vld1_u8(sp.add(stride * 5)));
        let a67 = vrhadd_u8(vld1_u8(sp.add(stride * 6)), vld1_u8(sp.add(stride * 7)));
        let a03 = vrhadd_u8(a01, a23);
        let a47 = vrhadd_u8(a45, a67);
        let a07 = vrhadd_u8(a03, a47);
        vaddlv_u8(a07) as i32
    }
}

/// NEON hierarchical `pavgb` (`vrhaddq_u8`) 16x8 dual block average.
pub(crate) fn avg_16x8_pavgb_neon(src: &[u8], off: usize, stride: usize) -> [i32; 2] {
    assert!(off + 7 * stride + 16 <= src.len());
    unsafe {
        let sp = src.as_ptr().add(off);
        let a01 = vrhaddq_u8(vld1q_u8(sp), vld1q_u8(sp.add(stride)));
        let a23 = vrhaddq_u8(vld1q_u8(sp.add(stride * 2)), vld1q_u8(sp.add(stride * 3)));
        let a45 = vrhaddq_u8(vld1q_u8(sp.add(stride * 4)), vld1q_u8(sp.add(stride * 5)));
        let a67 = vrhaddq_u8(vld1q_u8(sp.add(stride * 6)), vld1q_u8(sp.add(stride * 7)));
        let a03 = vrhaddq_u8(a01, a23);
        let a47 = vrhaddq_u8(a45, a67);
        let a07 = vrhaddq_u8(a03, a47);
        [
            vaddlv_u8(vget_low_u8(a07)) as i32,
            vaddlv_u8(vget_high_u8(a07)) as i32,
        ]
    }
}

/// NEON hierarchical `pavgb` (`vrhaddq_u8`) 32x8 quad block average.
pub(crate) fn avg_32x8_pavgb_neon(src: &[u8], off: usize, stride: usize) -> [i32; 4] {
    let lo = avg_16x8_pavgb_neon(src, off, stride);
    let hi = avg_16x8_pavgb_neon(src, off + 16, stride);
    [lo[0], lo[1], hi[0], hi[1]]
}
