//! Inline SIMD and vector helper operations for AOT-lifted functions in `shards/`.
#![allow(dead_code)]

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[inline(always)]
fn f32x4(v: u128) -> [f32; 4] {
    [
        f32::from_bits(v as u32),
        f32::from_bits((v >> 32) as u32),
        f32::from_bits((v >> 64) as u32),
        f32::from_bits((v >> 96) as u32),
    ]
}

#[inline(always)]
fn pack_f32x4(a: [f32; 4]) -> u128 {
    (a[0].to_bits() as u128)
        | ((a[1].to_bits() as u128) << 32)
        | ((a[2].to_bits() as u128) << 64)
        | ((a[3].to_bits() as u128) << 96)
}

#[inline(always)]
fn f64x2(v: u128) -> [f64; 2] {
    [f64::from_bits(v as u64), f64::from_bits((v >> 64) as u64)]
}

#[inline(always)]
fn pack_f64x2(a: [f64; 2]) -> u128 {
    (a[0].to_bits() as u128) | ((a[1].to_bits() as u128) << 64)
}

#[cfg(target_arch = "aarch64")]
// ---- NEON implementations (lane 0 = low 32 bits of the u128, as on x86) ----
// Each helper below matches the scalar per-lane definition it replaced
// bit-for-bit (randomized + exhaustive-f16 differential test), except which
// NaN payload is returned when several operands are NaN -- LLVM already
// treated that as unspecified (it commutes fmul/fadd), so the scalar code was
// no more x86-exact. VCVTPH2PS now quiets signalling NaNs, as F16C does.
// They compile to one or a few vector instructions instead of 4x scalar work
// plus u128 shift/or packing (the hot f16/FMA skinning loop went 1118 -> 89
// instructions).

#[inline(always)]
fn qf(v: u128) -> float32x4_t {
    unsafe { core::mem::transmute(v) }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn fq(v: float32x4_t) -> u128 {
    unsafe { core::mem::transmute(v) }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn qd(v: u128) -> float64x2_t {
    unsafe { core::mem::transmute(v) }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn dq(v: float64x2_t) -> u128 {
    unsafe { core::mem::transmute(v) }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn qu(v: u128) -> uint32x4_t {
    unsafe { core::mem::transmute(v) }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn uq(v: uint32x4_t) -> u128 {
    unsafe { core::mem::transmute(v) }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn qs(v: u128) -> int32x4_t {
    unsafe { core::mem::transmute(v) }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn sq(v: int32x4_t) -> u128 {
    unsafe { core::mem::transmute(v) }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn ql(v: u128) -> uint64x2_t {
    unsafe { core::mem::transmute(v) }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn lq(v: uint64x2_t) -> u128 {
    unsafe { core::mem::transmute(v) }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn qb(v: u128) -> uint8x16_t {
    unsafe { core::mem::transmute(v) }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn bq(v: uint8x16_t) -> u128 {
    unsafe { core::mem::transmute(v) }
}

#[cfg(target_arch = "aarch64")]
/// Lane `i & 3` of `x`. With a constant `i` (every lifted immediate) this

/// folds to a single extract, and chains of these fold into one shuffle.

#[inline(always)]
fn lane32(x: uint32x4_t, i: u8) -> u32 {
    unsafe {
        match i & 3 {
            0 => vgetq_lane_u32::<0>(x),
            1 => vgetq_lane_u32::<1>(x),
            2 => vgetq_lane_u32::<2>(x),
            _ => vgetq_lane_u32::<3>(x),
        }
    }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn set_lane32(x: uint32x4_t, i: u8, v: u32) -> uint32x4_t {
    unsafe {
        match i & 3 {
            0 => vsetq_lane_u32::<0>(v, x),
            1 => vsetq_lane_u32::<1>(v, x),
            2 => vsetq_lane_u32::<2>(v, x),
            _ => vsetq_lane_u32::<3>(v, x),
        }
    }
}

#[cfg(target_arch = "aarch64")]
/// All-ones in lane i where bit i of `bits` is set (constant-folds for an immediate).

#[inline(always)]
fn lane_mask4(bits: u8) -> uint32x4_t {
    let m = |i: u8| if (bits >> i) & 1 != 0 { u32::MAX } else { 0 };
    unsafe { core::mem::transmute([m(0), m(1), m(2), m(3)]) }
}

#[cfg(target_arch = "aarch64")]
/// x86 MINPS: `a < b ? a : b` (second operand on NaN / equal), not IEEE fmin.

/// x86 MAXPS: `a > b ? a : b`.

/// [a0+a1, a2+a3, b0+b1, b2+b3] == FADDP.

/// [a0-a1, a2-a3, b0-b1, b2-b3].

/// [a0-b0, a1+b1, a2-b2, a3+b3].

/// fma((neg_ab ? -a : a), b, (neg_c ? -c : c)) per lane, fused like `f32::mul_add`.

/// x86 PSHUFB: zero where the index byte has bit 7 set, else a[idx & 15].

/// TBL yields 0 for indices >= 16, so masking with 0x8f gives the same result.

/// Lanes that are NaN or outside [-2^31, 2^31) become 0x8000_0000 (x86

/// "integer indefinite"); FCVTZS would saturate instead.

#[inline(always)]
fn cvt_ps_dq_fixup(x: float32x4_t, conv: int32x4_t) -> u128 {
    unsafe {
        let lo = vcgeq_f32(x, vdupq_n_f32(-2147483648.0));
        let hi = vcltq_f32(x, vdupq_n_f32(2147483648.0));
        let ok = vandq_u32(lo, hi);
        uq(vbslq_u32(ok, vreinterpretq_u32_s32(conv), vdupq_n_u32(0x8000_0000)))
    }
}

#[cfg(target_arch = "aarch64")]
/// Vector form of `cmp_f32` for all 16 predicates (imm & 0x0f).

/// DPPS: masked products summed as (p0 + p1) + (p2 + p3), broadcast to the

/// lanes selected by the low nibble. Masked-out products are +0.0 exactly.

/// F16C VCVTPH2PS via FCVTL (exact; quiets signalling NaNs like x86 does).

#[inline(always)]
pub fn vaddps(a: u128, b: u128) -> u128 {
    unsafe { fq(vaddq_f32(qf(a), qf(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vaddps(a: u128, b: u128) -> u128 {
    let (x, y) = (f32x4(a), f32x4(b));
    pack_f32x4([x[0] + y[0], x[1] + y[1], x[2] + y[2], x[3] + y[3]])
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vsubps(a: u128, b: u128) -> u128 {
    unsafe { fq(vsubq_f32(qf(a), qf(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vsubps(a: u128, b: u128) -> u128 {
    let (x, y) = (f32x4(a), f32x4(b));
    pack_f32x4([x[0] - y[0], x[1] - y[1], x[2] - y[2], x[3] - y[3]])
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vmulps(a: u128, b: u128) -> u128 {
    unsafe { fq(vmulq_f32(qf(a), qf(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vmulps(a: u128, b: u128) -> u128 {
    let (x, y) = (f32x4(a), f32x4(b));
    pack_f32x4([x[0] * y[0], x[1] * y[1], x[2] * y[2], x[3] * y[3]])
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vdivps(a: u128, b: u128) -> u128 {
    unsafe { fq(vdivq_f32(qf(a), qf(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vdivps(a: u128, b: u128) -> u128 {
    let (x, y) = (f32x4(a), f32x4(b));
    pack_f32x4([x[0] / y[0], x[1] / y[1], x[2] / y[2], x[3] / y[3]])
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vminps(a: u128, b: u128) -> u128 {
    unsafe {
        let (x, y) = (qf(a), qf(b));
        fq(vbslq_f32(vcltq_f32(x, y), x, y))
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vminps(a: u128, b: u128) -> u128 {
    let (x, y) = (f32x4(a), f32x4(b));
    pack_f32x4([
        if x[0] < y[0] { x[0] } else { y[0] },
        if x[1] < y[1] { x[1] } else { y[1] },
        if x[2] < y[2] { x[2] } else { y[2] },
        if x[3] < y[3] { x[3] } else { y[3] },
    ])
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vmaxps(a: u128, b: u128) -> u128 {
    unsafe {
        let (x, y) = (qf(a), qf(b));
        fq(vbslq_f32(vcgtq_f32(x, y), x, y))
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vmaxps(a: u128, b: u128) -> u128 {
    let (x, y) = (f32x4(a), f32x4(b));
    pack_f32x4([
        if x[0] > y[0] { x[0] } else { y[0] },
        if x[1] > y[1] { x[1] } else { y[1] },
        if x[2] > y[2] { x[2] } else { y[2] },
        if x[3] > y[3] { x[3] } else { y[3] },
    ])
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vsqrtps(a: u128) -> u128 {
    unsafe { fq(vsqrtq_f32(qf(a))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vsqrtps(a: u128) -> u128 {
    let x = f32x4(a);
    pack_f32x4([x[0].sqrt(), x[1].sqrt(), x[2].sqrt(), x[3].sqrt()])
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vrsqrtps(a: u128) -> u128 {
    unsafe { fq(vdivq_f32(vdupq_n_f32(1.0), vsqrtq_f32(qf(a)))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vrsqrtps(a: u128) -> u128 {
    let x = f32x4(a);
    pack_f32x4([
        1.0 / x[0].sqrt(),
        1.0 / x[1].sqrt(),
        1.0 / x[2].sqrt(),
        1.0 / x[3].sqrt(),
    ])
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vrcpps(a: u128) -> u128 {
    unsafe { fq(vdivq_f32(vdupq_n_f32(1.0), qf(a))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vrcpps(a: u128) -> u128 {
    let x = f32x4(a);
    pack_f32x4([1.0 / x[0], 1.0 / x[1], 1.0 / x[2], 1.0 / x[3]])
}

#[inline(always)]
pub fn vrsqrtss(base: u128, src: u32) -> u128 {
    let r = (1.0f32 / f32::from_bits(src).sqrt()).to_bits() as u128;
    (base & !0xffff_ffffu128) | r
}

#[inline(always)]
pub fn vrcpss(base: u128, src: u32) -> u128 {
    let r = (1.0f32 / f32::from_bits(src)).to_bits() as u128;
    (base & !0xffff_ffffu128) | r
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vhaddps(a: u128, b: u128) -> u128 {
    unsafe { fq(vpaddq_f32(qf(a), qf(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vhaddps(a: u128, b: u128) -> u128 {
    let (x, y) = (f32x4(a), f32x4(b));
    pack_f32x4([x[0] + x[1], x[2] + x[3], y[0] + y[1], y[2] + y[3]])
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vhsubps(a: u128, b: u128) -> u128 {
    unsafe {
        let (x, y) = (qf(a), qf(b));
        fq(vsubq_f32(vuzp1q_f32(x, y), vuzp2q_f32(x, y)))
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vhsubps(a: u128, b: u128) -> u128 {
    let (x, y) = (f32x4(a), f32x4(b));
    pack_f32x4([x[0] - x[1], x[2] - x[3], y[0] - y[1], y[2] - y[3]])
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vaddsubps(a: u128, b: u128) -> u128 {
    unsafe {
        let (x, y) = (qf(a), qf(b));
        let odd = lane_mask4(0b1010);
        fq(vbslq_f32(odd, vaddq_f32(x, y), vsubq_f32(x, y)))
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vaddsubps(a: u128, b: u128) -> u128 {
    let (x, y) = (f32x4(a), f32x4(b));
    pack_f32x4([x[0] - y[0], x[1] + y[1], x[2] - y[2], x[3] + y[3]])
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vaddpd(a: u128, b: u128) -> u128 {
    unsafe { dq(vaddq_f64(qd(a), qd(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vaddpd(a: u128, b: u128) -> u128 {
    let (x, y) = (f64x2(a), f64x2(b));
    pack_f64x2([x[0] + y[0], x[1] + y[1]])
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vsubpd(a: u128, b: u128) -> u128 {
    unsafe { dq(vsubq_f64(qd(a), qd(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vsubpd(a: u128, b: u128) -> u128 {
    let (x, y) = (f64x2(a), f64x2(b));
    pack_f64x2([x[0] - y[0], x[1] - y[1]])
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vmulpd(a: u128, b: u128) -> u128 {
    unsafe { dq(vmulq_f64(qd(a), qd(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vmulpd(a: u128, b: u128) -> u128 {
    let (x, y) = (f64x2(a), f64x2(b));
    pack_f64x2([x[0] * y[0], x[1] * y[1]])
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vdivpd(a: u128, b: u128) -> u128 {
    unsafe { dq(vdivq_f64(qd(a), qd(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vdivpd(a: u128, b: u128) -> u128 {
    let (x, y) = (f64x2(a), f64x2(b));
    pack_f64x2([x[0] / y[0], x[1] / y[1]])
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vminpd(a: u128, b: u128) -> u128 {
    unsafe {
        let (x, y) = (qd(a), qd(b));
        dq(vbslq_f64(vcltq_f64(x, y), x, y))
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vminpd(a: u128, b: u128) -> u128 {
    let (x, y) = (f64x2(a), f64x2(b));
    pack_f64x2([
        if x[0] < y[0] { x[0] } else { y[0] },
        if x[1] < y[1] { x[1] } else { y[1] },
    ])
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vmaxpd(a: u128, b: u128) -> u128 {
    unsafe {
        let (x, y) = (qd(a), qd(b));
        dq(vbslq_f64(vcgtq_f64(x, y), x, y))
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vmaxpd(a: u128, b: u128) -> u128 {
    let (x, y) = (f64x2(a), f64x2(b));
    pack_f64x2([
        if x[0] > y[0] { x[0] } else { y[0] },
        if x[1] > y[1] { x[1] } else { y[1] },
    ])
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vsqrtpd(a: u128) -> u128 {
    unsafe { dq(vsqrtq_f64(qd(a))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vsqrtpd(a: u128) -> u128 {
    let x = f64x2(a);
    pack_f64x2([x[0].sqrt(), x[1].sqrt()])
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vfmadd_ps(a: u128, b: u128, c: u128, neg_ab: bool, neg_c: bool) -> u128 {
    unsafe {
        let aa = if neg_ab { vnegq_f32(qf(a)) } else { qf(a) };
        let cc = if neg_c { vnegq_f32(qf(c)) } else { qf(c) };
        fq(vfmaq_f32(cc, aa, qf(b)))
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vfmadd_ps(a: u128, b: u128, c: u128, neg_ab: bool, neg_c: bool) -> u128 {
    let (va, vb, vc) = (f32x4(a), f32x4(b), f32x4(c));
    let mut out = [0.0f32; 4];
    for i in 0..4 {
        let aa = if neg_ab { -va[i] } else { va[i] };
        let cc = if neg_c { -vc[i] } else { vc[i] };
        out[i] = aa.mul_add(vb[i], cc);
    }
    pack_f32x4(out)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vfmadd_pd(a: u128, b: u128, c: u128, neg_ab: bool, neg_c: bool) -> u128 {
    unsafe {
        let aa = if neg_ab { vnegq_f64(qd(a)) } else { qd(a) };
        let cc = if neg_c { vnegq_f64(qd(c)) } else { qd(c) };
        dq(vfmaq_f64(cc, aa, qd(b)))
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vfmadd_pd(a: u128, b: u128, c: u128, neg_ab: bool, neg_c: bool) -> u128 {
    let (va, vb, vc) = (f64x2(a), f64x2(b), f64x2(c));
    let mut out = [0.0f64; 2];
    for i in 0..2 {
        let aa = if neg_ab { -va[i] } else { va[i] };
        let cc = if neg_c { -vc[i] } else { vc[i] };
        out[i] = aa.mul_add(vb[i], cc);
    }
    pack_f64x2(out)
}

#[inline(always)]
pub fn vfmadd_ss(base: u128, a: u32, b: u32, c: u32, neg_ab: bool, neg_c: bool) -> u128 {
    let (fa, fb, fc) = (f32::from_bits(a), f32::from_bits(b), f32::from_bits(c));
    let aa = if neg_ab { -fa } else { fa };
    let cc = if neg_c { -fc } else { fc };
    (base & !0xffff_ffffu128) | (aa.mul_add(fb, cc).to_bits() as u128)
}

#[inline(always)]
pub fn vfmadd_sd(base: u128, a: u64, b: u64, c: u64, neg_ab: bool, neg_c: bool) -> u128 {
    let (fa, fb, fc) = (f64::from_bits(a), f64::from_bits(b), f64::from_bits(c));
    let aa = if neg_ab { -fa } else { fa };
    let cc = if neg_c { -fc } else { fc };
    (base & !0xffff_ffff_ffff_ffffu128) | (aa.mul_add(fb, cc).to_bits() as u128)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vshufps(a: u128, b: u128, imm: u8) -> u128 {
    unsafe {
        let (x, y) = (qu(a), qu(b));
        let mut r = vdupq_n_u32(lane32(x, imm));
        r = vsetq_lane_u32::<1>(lane32(x, imm >> 2), r);
        r = vsetq_lane_u32::<2>(lane32(y, imm >> 4), r);
        r = vsetq_lane_u32::<3>(lane32(y, imm >> 6), r);
        uq(r)
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vshufps(a: u128, b: u128, imm: u8) -> u128 {
    let d0 = (a >> (((imm & 3) as u32) * 32)) & 0xffff_ffff;
    let d1 = (a >> ((((imm >> 2) & 3) as u32) * 32)) & 0xffff_ffff;
    let d2 = (b >> ((((imm >> 4) & 3) as u32) * 32)) & 0xffff_ffff;
    let d3 = (b >> ((((imm >> 6) & 3) as u32) * 32)) & 0xffff_ffff;
    d0 | (d1 << 32) | (d2 << 64) | (d3 << 96)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vshufpd(a: u128, b: u128, imm: u8) -> u128 {
    unsafe {
        let (x, y) = (ql(a), ql(b));
        let q0 = if imm & 1 != 0 { vgetq_lane_u64::<1>(x) } else { vgetq_lane_u64::<0>(x) };
        let q1 = if imm & 2 != 0 { vgetq_lane_u64::<1>(y) } else { vgetq_lane_u64::<0>(y) };
        lq(vsetq_lane_u64::<1>(q1, vdupq_n_u64(q0)))
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vshufpd(a: u128, b: u128, imm: u8) -> u128 {
    let q0 = (a >> (((imm & 1) as u32) * 64)) & 0xffff_ffff_ffff_ffff;
    let q1 = (b >> ((((imm >> 1) & 1) as u32) * 64)) & 0xffff_ffff_ffff_ffff;
    q0 | (q1 << 64)
}

#[inline(always)]
pub fn vpermilps(a: u128, imm: u8) -> u128 {
    vshufps(a, a, imm)
}

#[inline(always)]
pub fn vpermilpd(a: u128, imm: u8) -> u128 {
    vshufpd(a, a, imm)
}

#[inline(always)]
pub fn vpshufd(a: u128, imm: u8) -> u128 {
    vshufps(a, a, imm)
}

#[inline(always)]
pub fn vpshuflw(a: u128, imm: u8) -> u128 {
    let w0 = (a >> (((imm & 3) as u32) * 16)) & 0xffff;
    let w1 = (a >> ((((imm >> 2) & 3) as u32) * 16)) & 0xffff;
    let w2 = (a >> ((((imm >> 4) & 3) as u32) * 16)) & 0xffff;
    let w3 = (a >> ((((imm >> 6) & 3) as u32) * 16)) & 0xffff;
    (a & !0xffff_ffff_ffff_ffffu128) | w0 | (w1 << 16) | (w2 << 32) | (w3 << 48)
}

#[inline(always)]
pub fn vpshufhw(a: u128, imm: u8) -> u128 {
    let hi = a >> 64;
    let w0 = (hi >> (((imm & 3) as u32) * 16)) & 0xffff;
    let w1 = (hi >> ((((imm >> 2) & 3) as u32) * 16)) & 0xffff;
    let w2 = (hi >> ((((imm >> 4) & 3) as u32) * 16)) & 0xffff;
    let w3 = (hi >> ((((imm >> 6) & 3) as u32) * 16)) & 0xffff;
    (a & 0xffff_ffff_ffff_ffffu128) | ((w0 | (w1 << 16) | (w2 << 32) | (w3 << 48)) << 64)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vpshufb(a: u128, b: u128) -> u128 {
    unsafe { bq(vqtbl1q_u8(qb(a), vandq_u8(qb(b), vdupq_n_u8(0x8f)))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vpshufb(a: u128, b: u128) -> u128 {
    let ab = a.to_le_bytes();
    let bb = b.to_le_bytes();
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = if bb[i] & 0x80 != 0 { 0 } else { ab[(bb[i] & 0x0f) as usize] };
    }
    u128::from_le_bytes(out)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vunpcklps(a: u128, b: u128) -> u128 {
    unsafe { uq(vzip1q_u32(qu(a), qu(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vunpcklps(a: u128, b: u128) -> u128 {
    let a0 = a & 0xffff_ffff;
    let a1 = (a >> 32) & 0xffff_ffff;
    let b0 = b & 0xffff_ffff;
    let b1 = (b >> 32) & 0xffff_ffff;
    a0 | (b0 << 32) | (a1 << 64) | (b1 << 96)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vunpckhps(a: u128, b: u128) -> u128 {
    unsafe { uq(vzip2q_u32(qu(a), qu(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vunpckhps(a: u128, b: u128) -> u128 {
    let a2 = (a >> 64) & 0xffff_ffff;
    let a3 = (a >> 96) & 0xffff_ffff;
    let b2 = (b >> 64) & 0xffff_ffff;
    let b3 = (b >> 96) & 0xffff_ffff;
    a2 | (b2 << 32) | (a3 << 64) | (b3 << 96)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vunpcklpd(a: u128, b: u128) -> u128 {
    unsafe { lq(vzip1q_u64(ql(a), ql(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vunpcklpd(a: u128, b: u128) -> u128 {
    (a & 0xffff_ffff_ffff_ffff) | ((b & 0xffff_ffff_ffff_ffff) << 64)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vunpckhpd(a: u128, b: u128) -> u128 {
    unsafe { lq(vzip2q_u64(ql(a), ql(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vunpckhpd(a: u128, b: u128) -> u128 {
    (a >> 64) | ((b >> 64) << 64)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vinsertps(a: u128, b: u128, imm: u8) -> u128 {
    unsafe {
        let r = set_lane32(qu(a), imm >> 4, lane32(qu(b), imm >> 6));
        uq(vbicq_u32(r, lane_mask4(imm & 0x0f)))
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vinsertps(a: u128, b: u128, imm: u8) -> u128 {
    let count_s = ((imm >> 6) & 3) as u32;
    let count_d = ((imm >> 4) & 3) as u32;
    let zmask = imm & 0x0f;
    let tmp = (b >> (count_s * 32)) & 0xffff_ffff;
    let mut r = (a & !(0xffff_ffffu128 << (count_d * 32))) | (tmp << (count_d * 32));
    for i in 0..4u32 {
        if (zmask & (1 << i)) != 0 {
            r &= !(0xffff_ffffu128 << (i * 32));
        }
    }
    r
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vblendps(a: u128, b: u128, imm: u8) -> u128 {
    unsafe { uq(vbslq_u32(lane_mask4(imm & 0x0f), qu(b), qu(a))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vblendps(a: u128, b: u128, imm: u8) -> u128 {
    let mut r = 0u128;
    for i in 0..4u32 {
        let src = if (imm & (1 << i)) != 0 { b } else { a };
        r |= ((src >> (i * 32)) & 0xffff_ffff) << (i * 32);
    }
    r
}

#[inline(always)]
pub fn vblendpd(a: u128, b: u128, imm: u8) -> u128 {
    let lo = if (imm & 1) != 0 { b & 0xffff_ffff_ffff_ffff } else { a & 0xffff_ffff_ffff_ffff };
    let hi = if (imm & 2) != 0 { b >> 64 } else { a >> 64 };
    lo | (hi << 64)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vblendvps(a: u128, b: u128, mask: u128) -> u128 {
    unsafe {
        let m = vreinterpretq_u32_s32(vshrq_n_s32::<31>(qs(mask)));
        uq(vbslq_u32(m, qu(b), qu(a)))
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vblendvps(a: u128, b: u128, mask: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4u32 {
        let m = (mask >> (i * 32)) as u32;
        let src = if (m & 0x8000_0000) != 0 { b } else { a };
        r |= ((src >> (i * 32)) & 0xffff_ffff) << (i * 32);
    }
    r
}

#[inline(always)]
pub fn vblendvpd(a: u128, b: u128, mask: u128) -> u128 {
    let lo = if ((mask as u64) >> 63) != 0 { b & 0xffff_ffff_ffff_ffff } else { a & 0xffff_ffff_ffff_ffff };
    let hi = if ((mask >> 127) & 1) != 0 { b >> 64 } else { a >> 64 };
    lo | (hi << 64)
}

#[inline(always)]
pub fn vpblendvb(a: u128, b: u128, mask: u128) -> u128 {
    let (ab, bb, mb) = (a.to_le_bytes(), b.to_le_bytes(), mask.to_le_bytes());
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = if (mb[i] & 0x80) != 0 { bb[i] } else { ab[i] };
    }
    u128::from_le_bytes(out)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vbroadcastss(v: u32) -> u128 {
    unsafe { uq(vdupq_n_u32(v)) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vbroadcastss(v: u32) -> u128 {
    let d = v as u128;
    d | (d << 32) | (d << 64) | (d << 96)
}

#[inline(always)]
pub fn vpbroadcastb(v: u8) -> u128 {
    u128::from_le_bytes([v; 16])
}

#[inline(always)]
pub fn vpbroadcastw(v: u16) -> u128 {
    let w = v as u128;
    let mut r = 0u128;
    for i in 0..8 {
        r |= w << (i * 16);
    }
    r
}

#[inline(always)]
pub fn vpbroadcastd(v: u32) -> u128 {
    vbroadcastss(v)
}

#[inline(always)]
pub fn vpbroadcastq(v: u64) -> u128 {
    (v as u128) | ((v as u128) << 64)
}

#[inline(always)]
pub fn vbroadcastsd(v: u64) -> u128 {
    vpbroadcastq(v)
}

#[inline(always)]
pub fn vpmullw(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..8 {
        let x = (a >> (i * 16)) as u16;
        let y = (b >> (i * 16)) as u16;
        r |= (x.wrapping_mul(y) as u128) << (i * 16);
    }
    r
}

#[inline(always)]
pub fn vpblendw(a: u128, b: u128, imm: u8) -> u128 {
    let mut r = 0u128;
    for i in 0..8 {
        let w = if ((imm >> i) & 1) != 0 {
            (b >> (i * 16)) as u16
        } else {
            (a >> (i * 16)) as u16
        };
        r |= (w as u128) << (i * 16);
    }
    r
}

#[inline(always)]
pub fn vfmaddsub_ps(a: u128, b: u128, c: u128, sub_even: bool) -> u128 {
    let (xa, xb, xc) = (f32x4(a), f32x4(b), f32x4(c));
    let mut out = [0.0f32; 4];
    for i in 0..4 {
        let neg_c = if (i & 1) == 0 { sub_even } else { !sub_even };
        let cc = if neg_c { -xc[i] } else { xc[i] };
        out[i] = xa[i].mul_add(xb[i], cc);
    }
    pack_f32x4(out)
}

#[inline(always)]
pub fn vpermpd(lo: u128, hi: u128, imm: u8) -> (u128, u128) {
    let q = [lo as u64, (lo >> 64) as u64, hi as u64, (hi >> 64) as u64];
    let r_lo = (q[(imm & 3) as usize] as u128) | ((q[((imm >> 2) & 3) as usize] as u128) << 64);
    let r_hi = (q[((imm >> 4) & 3) as usize] as u128) | ((q[((imm >> 6) & 3) as usize] as u128) << 64);
    (r_lo, r_hi)
}

#[inline(always)]
pub fn vpermd(idx_lo: u128, idx_hi: u128, src_lo: u128, src_hi: u128) -> (u128, u128) {
    let s = [
        src_lo as u32,
        (src_lo >> 32) as u32,
        (src_lo >> 64) as u32,
        (src_lo >> 96) as u32,
        src_hi as u32,
        (src_hi >> 32) as u32,
        (src_hi >> 64) as u32,
        (src_hi >> 96) as u32,
    ];
    let pick = |ctl: u128| -> u128 {
        (s[(ctl as usize) & 7] as u128)
            | ((s[((ctl >> 32) as usize) & 7] as u128) << 32)
            | ((s[((ctl >> 64) as usize) & 7] as u128) << 64)
            | ((s[((ctl >> 96) as usize) & 7] as u128) << 96)
    };
    (pick(idx_lo), pick(idx_hi))
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vcvtdq2ps(a: u128) -> u128 {
    unsafe { fq(vcvtq_f32_s32(qs(a))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vcvtdq2ps(a: u128) -> u128 {
    pack_f32x4([
        (a as i32) as f32,
        ((a >> 32) as i32) as f32,
        ((a >> 64) as i32) as f32,
        ((a >> 96) as i32) as f32,
    ])
}

#[inline(always)]
pub fn cvt_f32_i32(v: f32) -> u32 {
    if v.is_nan() || v >= 2147483648.0 || v < -2147483648.0 {
        0x8000_0000
    } else {
        (v as i32) as u32
    }
}

#[inline(always)]
pub fn cvt_f32_i64(v: f32) -> u64 {
    if v.is_nan() || v >= 9223372036854775808.0 || v < -9223372036854775808.0 {
        0x8000_0000_0000_0000
    } else {
        (v as i64) as u64
    }
}

#[inline(always)]
pub fn cvt_f64_i32(v: f64) -> u32 {
    if v.is_nan() || v >= 2147483648.0 || v < -2147483648.0 {
        0x8000_0000
    } else {
        (v as i32) as u32
    }
}

#[inline(always)]
pub fn cvt_f64_i64(v: f64) -> u64 {
    if v.is_nan() || v >= 9223372036854775808.0 || v < -9223372036854775808.0 {
        0x8000_0000_0000_0000
    } else {
        (v as i64) as u64
    }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vcvttps2dq(a: u128) -> u128 {
    unsafe {
        let x = qf(a);
        cvt_ps_dq_fixup(x, vcvtq_s32_f32(x))
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vcvttps2dq(a: u128) -> u128 {
    let x = f32x4(a);
    let mut r = 0u128;
    for i in 0..4 {
        let v = x[i];
        let d: i32 = if v.is_nan() || v >= 2147483648.0 || v < -2147483648.0 {
            i32::MIN
        } else {
            v as i32
        };
        r |= ((d as u32) as u128) << (i * 32);
    }
    r
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vcvtps2dq(a: u128) -> u128 {
    unsafe {
        let v = vrndnq_f32(qf(a));
        cvt_ps_dq_fixup(v, vcvtq_s32_f32(v))
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vcvtps2dq(a: u128) -> u128 {
    let x = f32x4(a);
    let mut r = 0u128;
    for i in 0..4 {
        let v = x[i].round_ties_even();
        let d: i32 = if v.is_nan() || v >= 2147483648.0 || v < -2147483648.0 {
            i32::MIN
        } else {
            v as i32
        };
        r |= ((d as u32) as u128) << (i * 32);
    }
    r
}

#[inline(always)]
pub fn vcvtdq2pd(a: u128) -> u128 {
    pack_f64x2([(a as i32) as f64, ((a >> 32) as i32) as f64])
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vcvtpd2ps(a: u128) -> u128 {
    unsafe { fq(vcombine_f32(vcvt_f32_f64(qd(a)), vdup_n_f32(0.0))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vcvtpd2ps(a: u128) -> u128 {
    let x = f64x2(a);
    ((x[0] as f32).to_bits() as u128) | (((x[1] as f32).to_bits() as u128) << 32)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vcvtps2pd(a: u128) -> u128 {
    unsafe { dq(vcvt_f64_f32(vget_low_f32(qf(a)))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vcvtps2pd(a: u128) -> u128 {
    let x = f32x4(a);
    pack_f64x2([x[0] as f64, x[1] as f64])
}

#[inline(always)]
fn round_f32(v: f32, imm: u8) -> f32 {
    match imm & 3 {
        0 => v.round_ties_even(),
        1 => v.floor(),
        2 => v.ceil(),
        _ => v.trunc(),
    }
}

#[inline(always)]
fn round_f64(v: f64, imm: u8) -> f64 {
    match imm & 3 {
        0 => v.round_ties_even(),
        1 => v.floor(),
        2 => v.ceil(),
        _ => v.trunc(),
    }
}

#[inline(always)]
pub fn vroundss(base: u128, src: u32, imm: u8) -> u128 {
    let r = round_f32(f32::from_bits(src), imm).to_bits() as u128;
    (base & !0xffff_ffffu128) | r
}

#[inline(always)]
pub fn vroundsd(base: u128, src: u64, imm: u8) -> u128 {
    let r = round_f64(f64::from_bits(src), imm).to_bits() as u128;
    (base & !0xffff_ffff_ffff_ffffu128) | r
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vroundps(a: u128, imm: u8) -> u128 {
    unsafe {
        let x = qf(a);
        fq(match imm & 3 {
            0 => vrndnq_f32(x),
            1 => vrndmq_f32(x),
            2 => vrndpq_f32(x),
            _ => vrndq_f32(x),
        })
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vroundps(a: u128, imm: u8) -> u128 {
    let x = f32x4(a);
    pack_f32x4([
        round_f32(x[0], imm),
        round_f32(x[1], imm),
        round_f32(x[2], imm),
        round_f32(x[3], imm),
    ])
}

#[inline(always)]
fn cmp_f32(a: f32, b: f32, imm: u8) -> bool {
    let unord = a.is_nan() || b.is_nan();
    match imm & 0x0f {
        0x00 => !unord && a == b,
        0x01 => !unord && a < b,
        0x02 => !unord && a <= b,
        0x03 => unord,
        0x04 => unord || a != b,
        0x05 => unord || a >= b,
        0x06 => unord || a > b,
        0x07 => !unord,
        0x08 => unord || a == b,
        0x09 => unord || a < b,
        0x0a => unord || a <= b,
        0x0b => false,
        0x0c => !unord && a != b,
        0x0d => !unord && a >= b,
        0x0e => !unord && a > b,
        _ => true,
    }
}

#[inline(always)]
fn cmp_f64(a: f64, b: f64, imm: u8) -> bool {
    let unord = a.is_nan() || b.is_nan();
    match imm & 0x0f {
        0x00 => !unord && a == b,
        0x01 => !unord && a < b,
        0x02 => !unord && a <= b,
        0x03 => unord,
        0x04 => unord || a != b,
        0x05 => unord || a >= b,
        0x06 => unord || a > b,
        0x07 => !unord,
        0x08 => unord || a == b,
        0x09 => unord || a < b,
        0x0a => unord || a <= b,
        0x0b => false,
        0x0c => !unord && a != b,
        0x0d => !unord && a >= b,
        0x0e => !unord && a > b,
        _ => true,
    }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vcmpps(a: u128, b: u128, imm: u8) -> u128 {
    unsafe {
        let (x, y) = (qf(a), qf(b));
        let ord = || vandq_u32(vceqq_f32(x, x), vceqq_f32(y, y));
        uq(match imm & 0x0f {
            0x00 => vceqq_f32(x, y),
            0x01 => vcltq_f32(x, y),
            0x02 => vcleq_f32(x, y),
            0x03 => vmvnq_u32(ord()),
            0x04 => vmvnq_u32(vceqq_f32(x, y)),
            0x05 => vmvnq_u32(vcltq_f32(x, y)),
            0x06 => vmvnq_u32(vcleq_f32(x, y)),
            0x07 => ord(),
            0x08 => vornq_u32(vceqq_f32(x, y), ord()),
            0x09 => vmvnq_u32(vcgeq_f32(x, y)),
            0x0a => vmvnq_u32(vcgtq_f32(x, y)),
            0x0b => vdupq_n_u32(0),
            0x0c => vbicq_u32(ord(), vceqq_f32(x, y)),
            0x0d => vcgeq_f32(x, y),
            0x0e => vcgtq_f32(x, y),
            _ => vdupq_n_u32(u32::MAX),
        })
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vcmpps(a: u128, b: u128, imm: u8) -> u128 {
    let (x, y) = (f32x4(a), f32x4(b));
    let mut r = 0u128;
    for i in 0..4 {
        if cmp_f32(x[i], y[i], imm) {
            r |= 0xffff_ffffu128 << (i * 32);
        }
    }
    r
}

#[inline(always)]
pub fn vcmpss(base: u128, a: u32, b: u32, imm: u8) -> u128 {
    let m = if cmp_f32(f32::from_bits(a), f32::from_bits(b), imm) {
        0xffff_ffffu128
    } else {
        0
    };
    (base & !0xffff_ffffu128) | m
}

#[inline(always)]
pub fn vcmpsd(base: u128, a: u64, b: u64, imm: u8) -> u128 {
    let m = if cmp_f64(f64::from_bits(a), f64::from_bits(b), imm) {
        0xffff_ffff_ffff_ffffu128
    } else {
        0
    };
    (base & !0xffff_ffff_ffff_ffffu128) | m
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vdpps(a: u128, b: u128, imm: u8) -> u128 {
    unsafe {
        let p = vmulq_f32(qf(a), qf(b));
        let p = vreinterpretq_f32_u32(vandq_u32(vreinterpretq_u32_f32(p), lane_mask4(imm >> 4)));
        let s = vpaddq_f32(p, p); // [p0+p1, p2+p3, ..]
        let sum = vgetq_lane_f32::<0>(s) + vgetq_lane_f32::<1>(s);
        uq(vandq_u32(vreinterpretq_u32_f32(vdupq_n_f32(sum)), lane_mask4(imm & 0x0f)))
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vdpps(a: u128, b: u128, imm: u8) -> u128 {
    let (x, y) = (f32x4(a), f32x4(b));
    let p0 = if (imm & 0x10) != 0 { x[0] * y[0] } else { 0.0 };
    let p1 = if (imm & 0x20) != 0 { x[1] * y[1] } else { 0.0 };
    let p2 = if (imm & 0x40) != 0 { x[2] * y[2] } else { 0.0 };
    let p3 = if (imm & 0x80) != 0 { x[3] * y[3] } else { 0.0 };
    let sum = (p0 + p1) + (p2 + p3);
    let sb = sum.to_bits() as u128;
    let mut r = 0u128;
    for i in 0..4 {
        if (imm & (1 << i)) != 0 {
            r |= sb << (i * 32);
        }
    }
    r
}

#[inline(always)]
pub fn vmovmskps(a: u128) -> u64 {
    (((a >> 31) & 1) | (((a >> 63) & 1) << 1) | (((a >> 95) & 1) << 2) | (((a >> 127) & 1) << 3)) as u64
}

#[inline(always)]
pub fn vmovmskpd(a: u128) -> u64 {
    (((a >> 63) & 1) | (((a >> 127) & 1) << 1)) as u64
}

#[inline(always)]
pub fn vpmovmskb(a: u128) -> u64 {
    let b = a.to_le_bytes();
    let mut m = 0u64;
    for i in 0..16 {
        if (b[i] & 0x80) != 0 {
            m |= 1 << i;
        }
    }
    m
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vpaddd(a: u128, b: u128) -> u128 {
    unsafe { uq(vaddq_u32(qu(a), qu(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vpaddd(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4 {
        let x = (a >> (i * 32)) as u32;
        let y = (b >> (i * 32)) as u32;
        r |= (x.wrapping_add(y) as u128) << (i * 32);
    }
    r
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vpsubd(a: u128, b: u128) -> u128 {
    unsafe { uq(vsubq_u32(qu(a), qu(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vpsubd(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4 {
        let x = (a >> (i * 32)) as u32;
        let y = (b >> (i * 32)) as u32;
        r |= (x.wrapping_sub(y) as u128) << (i * 32);
    }
    r
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vpaddq(a: u128, b: u128) -> u128 {
    unsafe { lq(vaddq_u64(ql(a), ql(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vpaddq(a: u128, b: u128) -> u128 {
    let x0 = (a as u64).wrapping_add(b as u64);
    let x1 = ((a >> 64) as u64).wrapping_add((b >> 64) as u64);
    (x0 as u128) | ((x1 as u128) << 64)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vpsubq(a: u128, b: u128) -> u128 {
    unsafe { lq(vsubq_u64(ql(a), ql(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vpsubq(a: u128, b: u128) -> u128 {
    let x0 = (a as u64).wrapping_sub(b as u64);
    let x1 = ((a >> 64) as u64).wrapping_sub((b >> 64) as u64);
    (x0 as u128) | ((x1 as u128) << 64)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vpmuludq(a: u128, b: u128) -> u128 {
    unsafe { lq(vmull_u32(vmovn_u64(ql(a)), vmovn_u64(ql(b)))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vpmuludq(a: u128, b: u128) -> u128 {
    let p0 = ((a as u32) as u64) * ((b as u32) as u64);
    let p1 = (((a >> 64) as u32) as u64) * (((b >> 64) as u32) as u64);
    (p0 as u128) | ((p1 as u128) << 64)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vpcmpeqd(a: u128, b: u128) -> u128 {
    unsafe { uq(vceqq_u32(qu(a), qu(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vpcmpeqd(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4 {
        if ((a >> (i * 32)) as u32) == ((b >> (i * 32)) as u32) {
            r |= 0xffff_ffffu128 << (i * 32);
        }
    }
    r
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vpcmpeqq(a: u128, b: u128) -> u128 {
    unsafe { lq(vceqq_u64(ql(a), ql(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vpcmpeqq(a: u128, b: u128) -> u128 {
    let lo = if (a as u64) == (b as u64) { u64::MAX as u128 } else { 0 };
    let hi = if ((a >> 64) as u64) == ((b >> 64) as u64) { (u64::MAX as u128) << 64 } else { 0 };
    lo | hi
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vpcmpeqb(a: u128, b: u128) -> u128 {
    unsafe { bq(vceqq_u8(qb(a), qb(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vpcmpeqb(a: u128, b: u128) -> u128 {
    let (ab, bb) = (a.to_le_bytes(), b.to_le_bytes());
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = if ab[i] == bb[i] { 0xff } else { 0 };
    }
    u128::from_le_bytes(out)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vpcmpgtb(a: u128, b: u128) -> u128 {
    unsafe { bq(vcgtq_s8(vreinterpretq_s8_u8(qb(a)), vreinterpretq_s8_u8(qb(b)))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vpcmpgtb(a: u128, b: u128) -> u128 {
    let (ab, bb) = (a.to_le_bytes(), b.to_le_bytes());
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = if (ab[i] as i8) > (bb[i] as i8) { 0xff } else { 0 };
    }
    u128::from_le_bytes(out)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vpcmpgtd(a: u128, b: u128) -> u128 {
    unsafe { uq(vcgtq_s32(qs(a), qs(b))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vpcmpgtd(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4 {
        if ((a >> (i * 32)) as i32) > ((b >> (i * 32)) as i32) {
            r |= 0xffff_ffffu128 << (i * 32);
        }
    }
    r
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vpslld(a: u128, count: u64) -> u128 {
    if count >= 32 {
        return 0;
    }
    unsafe { uq(vshlq_u32(qu(a), vdupq_n_s32(count as i32))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vpslld(a: u128, count: u64) -> u128 {
    if count >= 32 {
        return 0;
    }
    let sh = count as u32;
    let mut r = 0u128;
    for i in 0..4 {
        let x = ((a >> (i * 32)) as u32) << sh;
        r |= (x as u128) << (i * 32);
    }
    r
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vpsrld(a: u128, count: u64) -> u128 {
    if count >= 32 {
        return 0;
    }
    unsafe { uq(vshlq_u32(qu(a), vdupq_n_s32(-(count as i32)))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vpsrld(a: u128, count: u64) -> u128 {
    if count >= 32 {
        return 0;
    }
    let sh = count as u32;
    let mut r = 0u128;
    for i in 0..4 {
        let x = ((a >> (i * 32)) as u32) >> sh;
        r |= (x as u128) << (i * 32);
    }
    r
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vpsrad(a: u128, count: u64) -> u128 {
    let sh = count.min(31) as i32;
    unsafe { sq(vshlq_s32(qs(a), vdupq_n_s32(-sh))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vpsrad(a: u128, count: u64) -> u128 {
    let sh = count.min(31) as u32;
    let mut r = 0u128;
    for i in 0..4 {
        let x = (((a >> (i * 32)) as i32) >> sh) as u32;
        r |= (x as u128) << (i * 32);
    }
    r
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vpsllq(a: u128, count: u64) -> u128 {
    if count >= 64 {
        return 0;
    }
    unsafe { lq(vshlq_u64(ql(a), vdupq_n_s64(count as i64))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vpsllq(a: u128, count: u64) -> u128 {
    if count >= 64 {
        return 0;
    }
    let sh = count as u32;
    let x0 = (a as u64) << sh;
    let x1 = ((a >> 64) as u64) << sh;
    (x0 as u128) | ((x1 as u128) << 64)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vpsrlq(a: u128, count: u64) -> u128 {
    if count >= 64 {
        return 0;
    }
    unsafe { lq(vshlq_u64(ql(a), vdupq_n_s64(-(count as i64)))) }
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vpsrlq(a: u128, count: u64) -> u128 {
    if count >= 64 {
        return 0;
    }
    let sh = count as u32;
    let x0 = (a as u64) >> sh;
    let x1 = ((a >> 64) as u64) >> sh;
    (x0 as u128) | ((x1 as u128) << 64)
}

#[inline(always)]
pub fn vpsrldq(a: u128, imm: u8) -> u128 {
    if imm >= 16 { 0 } else { a >> ((imm as u32) * 8) }
}

#[inline(always)]
pub fn vpslldq(a: u128, imm: u8) -> u128 {
    if imm >= 16 { 0 } else { a << ((imm as u32) * 8) }
}

#[inline(always)]
pub fn vpsignb(a: u128, b: u128) -> u128 {
    let (ab, bb) = (a.to_le_bytes(), b.to_le_bytes());
    let mut out = [0u8; 16];
    for i in 0..16 {
        let s = bb[i] as i8;
        out[i] = if s < 0 {
            (ab[i] as i8).wrapping_neg() as u8
        } else if s == 0 {
            0
        } else {
            ab[i]
        };
    }
    u128::from_le_bytes(out)
}

#[inline(always)]
fn f32_to_f16(v: f32) -> u16 {
    let b = v.to_bits();
    let sign = ((b >> 16) & 0x8000) as u16;
    let exp = ((b >> 23) & 0xff) as i32;
    let mant = b & 0x007f_ffff;
    if exp == 255 {
        return sign | 0x7c00 | if mant != 0 { 0x0200 } else { 0 };
    }
    let new_exp = exp - 127 + 15;
    if new_exp >= 31 {
        return sign | 0x7c00;
    }
    if new_exp <= 0 {
        if new_exp < -10 {
            return sign;
        }
        let m = (mant | 0x0080_0000) >> (1 - new_exp);
        return sign | ((m + 0x1000) >> 13) as u16;
    }
    sign | ((new_exp as u16) << 10) | ((mant + 0x1000) >> 13) as u16
}

#[inline(always)]
fn f16_to_f32(h: u16) -> f32 {
    let sign = ((h & 0x8000) as u32) << 16;
    let exp = ((h >> 10) & 0x1f) as u32;
    let mant = (h & 0x03ff) as u32;
    let bits = if exp == 0 {
        if mant == 0 {
            sign
        } else {
            let mut m = mant;
            let mut e = 127u32 - 14;
            while (m & 0x0400) == 0 {
                m <<= 1;
                e -= 1;
            }
            sign | (e << 23) | ((m & 0x03ff) << 13)
        }
    } else if exp == 31 {
        sign | 0x7f80_0000 | (mant << 13)
    } else {
        sign | ((exp + 127 - 15) << 23) | (mant << 13)
    };
    f32::from_bits(bits)
}

#[inline(always)]
pub fn vcvtps2ph(a: u128) -> u64 {
    let x = f32x4(a);
    (f32_to_f16(x[0]) as u64)
        | ((f32_to_f16(x[1]) as u64) << 16)
        | ((f32_to_f16(x[2]) as u64) << 32)
        | ((f32_to_f16(x[3]) as u64) << 48)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub fn vcvtph2ps(a: u64) -> u128 {
    let r: float32x4_t;
    unsafe {
        core::arch::asm!(
            "fcvtl {0:v}.4s, {1:v}.4h",
            out(vreg) r,
            in(vreg) a,
            options(pure, nomem, nostack, preserves_flags)
        );
    }
    fq(r)
}

#[cfg(not(target_arch = "aarch64"))]
#[inline(always)]
pub fn vcvtph2ps(a: u64) -> u128 {
    pack_f32x4([
        f16_to_f32(a as u16),
        f16_to_f32((a >> 16) as u16),
        f16_to_f32((a >> 32) as u16),
        f16_to_f32((a >> 48) as u16),
    ])
}

#[inline(always)]
pub fn vpunpcklbw(a: u128, b: u128) -> u128 {
    let (ab, bb) = (a.to_le_bytes(), b.to_le_bytes());
    let mut out = [0u8; 16];
    for i in 0..8 {
        out[i * 2] = ab[i];
        out[i * 2 + 1] = bb[i];
    }
    u128::from_le_bytes(out)
}

#[inline(always)]
pub fn vpunpckhbw(a: u128, b: u128) -> u128 {
    let (ab, bb) = (a.to_le_bytes(), b.to_le_bytes());
    let mut out = [0u8; 16];
    for i in 0..8 {
        out[i * 2] = ab[8 + i];
        out[i * 2 + 1] = bb[8 + i];
    }
    u128::from_le_bytes(out)
}

#[inline(always)]
pub fn vpunpcklwd(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4 {
        let wa = ((a >> (i * 16)) & 0xffff) as u128;
        let wb = ((b >> (i * 16)) & 0xffff) as u128;
        r |= (wa << (i * 32)) | (wb << (i * 32 + 16));
    }
    r
}

#[inline(always)]
pub fn vpunpckhwd(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4 {
        let wa = ((a >> ((i + 4) * 16)) & 0xffff) as u128;
        let wb = ((b >> ((i + 4) * 16)) & 0xffff) as u128;
        r |= (wa << (i * 32)) | (wb << (i * 32 + 16));
    }
    r
}

#[inline(always)]
pub fn vpmulld(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4 {
        let x = (a >> (i * 32)) as u32;
        let y = (b >> (i * 32)) as u32;
        r |= (x.wrapping_mul(y) as u128) << (i * 32);
    }
    r
}

#[inline(always)]
pub fn vpmaddwd(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4 {
        let a0 = ((a >> (i * 32)) as u16 as i16) as i32;
        let a1 = ((a >> (i * 32 + 16)) as u16 as i16) as i32;
        let b0 = ((b >> (i * 32)) as u16 as i16) as i32;
        let b1 = ((b >> (i * 32 + 16)) as u16 as i16) as i32;
        let s = a0.wrapping_mul(b0).wrapping_add(a1.wrapping_mul(b1)) as u32;
        r |= (s as u128) << (i * 32);
    }
    r
}

#[inline(always)]
pub fn vpackssdw(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4 {
        let va = ((a >> (i * 32)) as i32).clamp(-32768, 32767) as i16 as u16;
        let vb = ((b >> (i * 32)) as i32).clamp(-32768, 32767) as i16 as u16;
        r |= ((va as u128) << (i * 16)) | ((vb as u128) << ((i + 4) * 16));
    }
    r
}

#[inline(always)]
pub fn vpaddw(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..8 {
        let w = ((a >> (i * 16)) as u16).wrapping_add((b >> (i * 16)) as u16);
        r |= (w as u128) << (i * 16);
    }
    r
}

#[inline(always)]
pub fn vpsubw(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..8 {
        let w = ((a >> (i * 16)) as u16).wrapping_sub((b >> (i * 16)) as u16);
        r |= (w as u128) << (i * 16);
    }
    r
}

#[inline(always)]
pub fn vpcmpeqw(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..8 {
        if ((a >> (i * 16)) as u16) == ((b >> (i * 16)) as u16) {
            r |= 0xffffu128 << (i * 16);
        }
    }
    r
}

#[inline(always)]
pub fn vpcmpgtw(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..8 {
        if ((a >> (i * 16)) as u16 as i16) > ((b >> (i * 16)) as u16 as i16) {
            r |= 0xffffu128 << (i * 16);
        }
    }
    r
}

#[inline(always)]
pub fn vpcmpgtq(a: u128, b: u128) -> u128 {
    let lo = if (a as i64) > (b as i64) { u64::MAX as u128 } else { 0 };
    let hi = if ((a >> 64) as i64) > ((b >> 64) as i64) { (u64::MAX as u128) << 64 } else { 0 };
    lo | hi
}

#[inline(always)]
pub fn vpmaxsw(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..8 {
        let wa = (a >> (i * 16)) as u16 as i16;
        let wb = (b >> (i * 16)) as u16 as i16;
        r |= ((wa.max(wb) as u16) as u128) << (i * 16);
    }
    r
}

#[inline(always)]
pub fn vpmaxsd(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4 {
        let da = (a >> (i * 32)) as i32;
        let db = (b >> (i * 32)) as i32;
        r |= ((da.max(db) as u32) as u128) << (i * 32);
    }
    r
}

#[inline(always)]
pub fn vpminsd(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4 {
        let da = (a >> (i * 32)) as i32;
        let db = (b >> (i * 32)) as i32;
        r |= ((da.min(db) as u32) as u128) << (i * 32);
    }
    r
}

#[inline(always)]
pub fn vpmaxud(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4 {
        let da = (a >> (i * 32)) as u32;
        let db = (b >> (i * 32)) as u32;
        r |= (da.max(db) as u128) << (i * 32);
    }
    r
}

/// `paddb`: 16 u8 lanes, wrapping (renoircore +0xb6fd0).
#[inline(always)]
pub fn vpaddb(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..16 {
        let v = ((a >> (i * 8)) as u8).wrapping_add((b >> (i * 8)) as u8);
        r |= (v as u128) << (i * 8);
    }
    r
}

/// `pmaxuw` (SSE4.1): 8 u16 lanes, unsigned max (renoircore +0x180370).
#[inline(always)]
pub fn vpmaxuw(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..8 {
        let wa = (a >> (i * 16)) as u16;
        let wb = (b >> (i * 16)) as u16;
        r |= (wa.max(wb) as u128) << (i * 16);
    }
    r
}

/// SSE4.2 `pcmpistri a, b, imm` (implicit lengths: each operand ends at its first zero
/// element): `(ecx, czso)`, czso = CF | ZF << 1 | SF << 2 | OF << 3 (AF = PF = 0). Same
/// semantics as the emulator's interpreter (`emu/simd_mov.rs`, Pcmpistri): imm[1:0] element
/// type (ub, uw, sb, sw), imm[3:2] aggregation (equal any, ranges, equal each, equal
/// ordered), imm[5:4] polarity, imm[6] most significant index. vcruntime140 strrchr / strstr /
/// wcsrchr / wcsstr (frontier_plan.md §2).
pub fn pcmpistri(a: u128, b: u128, imm: u8) -> (u32, u8) {
    let words = imm & 1 != 0;
    let signed = imm & 2 != 0;
    let n: usize = if words { 8 } else { 16 };
    let el = |v: u128, i: usize| -> i32 {
        if words {
            let w = (v >> (i * 16)) as u16;
            if signed { w as i16 as i32 } else { w as i32 }
        } else {
            let x = (v >> (i * 8)) as u8;
            if signed { x as i8 as i32 } else { x as i32 }
        }
    };
    let len = |v: u128| (0..n).find(|&i| el(v, i) == 0).unwrap_or(n);
    let (la, lb) = (len(a), len(b));
    let mut r1: u16 = 0;
    match (imm >> 2) & 3 {
        0 => {
            for j in 0..lb {
                if (0..la).any(|i| el(b, j) == el(a, i)) {
                    r1 |= 1 << j;
                }
            }
        }
        1 => {
            for j in 0..lb {
                let bj = el(b, j);
                if (0..la / 2).any(|k| bj >= el(a, 2 * k) && bj <= el(a, 2 * k + 1)) {
                    r1 |= 1 << j;
                }
            }
        }
        2 => {
            for j in 0..n {
                let m = if j < la && j < lb { el(a, j) == el(b, j) } else { j >= la && j >= lb };
                if m {
                    r1 |= 1 << j;
                }
            }
        }
        _ => {
            for j in 0..n {
                let mut m = true;
                for k in 0..(n - j) {
                    if k >= la {
                        break;
                    }
                    if j + k >= lb || el(a, k) != el(b, j + k) {
                        m = false;
                        break;
                    }
                }
                if m {
                    r1 |= 1 << j;
                }
            }
        }
    }
    let all: u16 = if n == 16 { 0xFFFF } else { 0x00FF };
    let valid_b: u16 = if lb >= 16 { 0xFFFF } else { (1u16 << lb) - 1 };
    let r2 = match (imm >> 4) & 3 {
        1 => !r1 & all,
        3 => (r1 ^ valid_b) & all,
        _ => r1 & all,
    };
    let idx = if r2 == 0 {
        n as u32
    } else if imm & 0x40 != 0 {
        15 - r2.leading_zeros()
    } else {
        r2.trailing_zeros()
    };
    let f = (r2 != 0) as u8 | (((lb < n) as u8) << 1) | (((la < n) as u8) << 2) | (((r2 & 1) as u8) << 3);
    (idx, f)
}

#[inline(always)]
pub fn vphaddd(a: u128, b: u128) -> u128 {
    let d0 = ((a as u32).wrapping_add((a >> 32) as u32)) as u128;
    let d1 = (((a >> 64) as u32).wrapping_add((a >> 96) as u32)) as u128;
    let d2 = ((b as u32).wrapping_add((b >> 32) as u32)) as u128;
    let d3 = (((b >> 64) as u32).wrapping_add((b >> 96) as u32)) as u128;
    d0 | (d1 << 32) | (d2 << 64) | (d3 << 96)
}

#[inline(always)]
pub fn vpsllw(a: u128, count: u64) -> u128 {
    if count >= 16 {
        return 0;
    }
    let sh = count as u32;
    let mut r = 0u128;
    for i in 0..8 {
        let w = ((a >> (i * 16)) as u16) << sh;
        r |= (w as u128) << (i * 16);
    }
    r
}

#[inline(always)]
pub fn vpsrlw(a: u128, count: u64) -> u128 {
    if count >= 16 {
        return 0;
    }
    let sh = count as u32;
    let mut r = 0u128;
    for i in 0..8 {
        let w = ((a >> (i * 16)) as u16) >> sh;
        r |= (w as u128) << (i * 16);
    }
    r
}

#[inline(always)]
pub fn vpsraw(a: u128, count: u64) -> u128 {
    let sh = count.min(15) as u32;
    let mut r = 0u128;
    for i in 0..8 {
        let w = (((a >> (i * 16)) as u16 as i16) >> sh) as u16;
        r |= (w as u128) << (i * 16);
    }
    r
}

#[inline(always)]
pub fn vroundpd(a: u128, imm: u8) -> u128 {
    let x = f64x2(a);
    pack_f64x2([round_f64(x[0], imm), round_f64(x[1], imm)])
}

#[inline(always)]
pub fn vcmppd(a: u128, b: u128, imm: u8) -> u128 {
    let (x, y) = (f64x2(a), f64x2(b));
    let lo = if cmp_f64(x[0], y[0], imm) { u64::MAX as u128 } else { 0 };
    let hi = if cmp_f64(x[1], y[1], imm) { (u64::MAX as u128) << 64 } else { 0 };
    lo | hi
}

#[inline(always)]
pub fn vpmovzxbw(a: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..8 {
        let b = ((a >> (i * 8)) & 0xff) as u128;
        r |= b << (i * 16);
    }
    r
}

#[inline(always)]
pub fn vpmovsxbw(a: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..8 {
        let w = (((a >> (i * 8)) as u8) as i8 as i16 as u16) as u128;
        r |= w << (i * 16);
    }
    r
}

#[inline(always)]
pub fn vpmovzxbd(a: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4 {
        let b = ((a >> (i * 8)) & 0xff) as u128;
        r |= b << (i * 32);
    }
    r
}

#[inline(always)]
pub fn vpmovsxbd(a: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4 {
        let d = (((a >> (i * 8)) as u8) as i8 as i32 as u32) as u128;
        r |= d << (i * 32);
    }
    r
}

#[inline(always)]
pub fn vpmovzxwd(a: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4 {
        let w = ((a >> (i * 16)) & 0xffff) as u128;
        r |= w << (i * 32);
    }
    r
}

#[inline(always)]
pub fn vpmovsxwd(a: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4 {
        let d = (((a >> (i * 16)) as u16) as i16 as i32 as u32) as u128;
        r |= d << (i * 32);
    }
    r
}

#[inline(always)]
pub fn vpmovzxdq(a: u128) -> u128 {
    ((a as u32) as u128) | ((((a >> 32) as u32) as u128) << 64)
}

#[inline(always)]
pub fn vpmovsxdq(a: u128) -> u128 {
    (((a as i32 as i64) as u64) as u128) | (((((a >> 32) as i32 as i64) as u64) as u128) << 64)
}

#[inline(always)]
pub fn vpmovzxwq(a: u128) -> u128 {
    ((a as u16) as u128) | ((((a >> 16) as u16) as u128) << 64)
}

#[inline(always)]
pub fn vpabsd(a: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4 {
        let v = (((a >> (i * 32)) as u32) as i32).wrapping_abs() as u32;
        r |= (v as u128) << (i * 32);
    }
    r
}

#[inline(always)]
pub fn vpmulhw(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..8 {
        let va = ((a >> (i * 16)) as u16) as i16 as i32;
        let vb = ((b >> (i * 16)) as u16) as i16 as i32;
        let hi = ((va * vb) >> 16) as u16;
        r |= (hi as u128) << (i * 16);
    }
    r
}

#[inline(always)]
pub fn vpackuswb(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..8 {
        let va = (((a >> (i * 16)) as u16) as i16).clamp(0, 255) as u8;
        let vb = (((b >> (i * 16)) as u16) as i16).clamp(0, 255) as u8;
        r |= (va as u128) << (i * 8);
        r |= (vb as u128) << ((i + 8) * 8);
    }
    r
}

#[inline(always)]
pub fn vpmullq(a: u128, b: u128) -> u128 {
    let l = (a as u64).wrapping_mul(b as u64) as u128;
    let h = ((a >> 64) as u64).wrapping_mul((b >> 64) as u64) as u128;
    l | (h << 64)
}

#[inline(always)]
pub fn vpminud(a: u128, b: u128) -> u128 {
    let mut r = 0u128;
    for i in 0..4 {
        let va = (a >> (i * 32)) as u32;
        let vb = (b >> (i * 32)) as u32;
        r |= (va.min(vb) as u128) << (i * 32);
    }
    r
}

#[inline(always)]
pub fn vpermilps_var(a: u128, ctrl: u128) -> u128 {
    let w = [
        a as u32,
        (a >> 32) as u32,
        (a >> 64) as u32,
        (a >> 96) as u32,
    ];
    let mut r = 0u128;
    for i in 0..4 {
        let idx = ((ctrl >> (i * 32)) & 3) as usize;
        r |= (w[idx] as u128) << (i * 32);
    }
    r
}

#[inline(always)]
pub fn cpuid_x64(leaf: u32, subleaf: u32) -> (u64, u64, u64, u64) {
    match leaf {
        0 => (0x16, 0x756e_6547, 0x6c65_746e, 0x4965_6e69),
        1 => (0x0009_06ea, 0x0010_0800, 0x7ffafbbf, 0xbfebfbff),
        7 if subleaf == 0 => (0, 0x209c_6fbf, 0, 0),
        0x8000_0000 => (0x8000_0008, 0, 0, 0),
        0x8000_0001 => (0, 0, 0x0000_0121, 0x2c10_0800),
        _ => (0, 0, 0, 0),
    }
}

// Forwarded RAX / XMM argument and XMM return state lives in the per-thread `CallCtx`
// (one inline TSD load instead of a `_tlv_get_addr` call per access).

#[inline(always)]
pub fn replace_rax_in(v: u64) -> u64 {
    crate::callctx::ctx().rax_in.replace(v)
}

#[inline(always)]
pub fn get_rax_in() -> u64 {
    crate::callctx::ctx().rax_in.get()
}

#[inline(always)]
pub fn set_tls_xmm_in(xmm: [u128; 4]) {
    let c = crate::callctx::ctx();
    c.xmm_in.set(xmm);
    c.xmm_in_set.set(true);
    c.xmm_out.set(None);
}

#[inline(always)]
pub fn take_tls_xmm_in_if_set() -> Option<[u128; 4]> {
    let c = crate::callctx::ctx();
    if c.xmm_in_set.replace(false) {
        Some(c.xmm_in.get())
    } else {
        None
    }
}

#[inline(always)]
pub fn take_tls_xmm_out(fallback: u128) -> u128 {
    let c = crate::callctx::ctx();
    c.xmm_in_set.set(false);
    c.xmm_out.take().unwrap_or(fallback)
}

#[inline(always)]
pub fn take_tls_xmm_out_opt() -> Option<u128> {
    let c = crate::callctx::ctx();
    c.xmm_in_set.set(false);
    c.xmm_out.take()
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn shim_xmm_slot_ptr(rbp: usize) -> Option<*mut u128> {
    if rbp < 0x10000 || (rbp & 7) != 0 {
        return None;
    }
    let ret_addr = core::ptr::read_unaligned((rbp + 8) as *const usize);
    if ret_addr < 0x10000 {
        return None;
    }
    let b0 = core::ptr::read_unaligned((ret_addr - 12) as *const u8);
    let b1 = core::ptr::read_unaligned((ret_addr - 11) as *const u8);
    let b10 = core::ptr::read_unaligned((ret_addr - 2) as *const u8);
    let b11 = core::ptr::read_unaligned((ret_addr - 1) as *const u8);
    if b0 == 0x48 && b1 == 0xB8 && b10 == 0xFF && b11 == 0xD0 {
        Some((rbp + 16 + 0x90) as *mut u128)
    } else {
        None
    }
}

#[inline(always)]
pub unsafe fn take_xmm_args() -> [u128; 4] {
    #[cfg(target_arch = "x86_64")]
    let (rbp, reg_buf) = {
        let mut rbp: usize;
        let mut buf = [0u128; 4];
        core::arch::asm!(
            "mov {rbp}, rbp",
            "movups [{p} + 0x00], xmm0",
            "movups [{p} + 0x10], xmm1",
            "movups [{p} + 0x20], xmm2",
            "movups [{p} + 0x30], xmm3",
            rbp = out(reg) rbp,
            p = in(reg) buf.as_mut_ptr(),
            options(nostack, preserves_flags),
        );
        (rbp, buf)
    };
    #[cfg(target_arch = "aarch64")]
    let reg_buf = {
        let mut buf = [0u128; 4];
        core::arch::asm!(
            "str q0, [{p}, #0x00]",
            "str q1, [{p}, #0x10]",
            "str q2, [{p}, #0x20]",
            "str q3, [{p}, #0x30]",
            p = in(reg) buf.as_mut_ptr(),
            options(nostack, preserves_flags),
        );
        buf
    };
    {
        let c = crate::callctx::ctx();
        if c.xmm_in_set.replace(false) {
            return c.xmm_in.get();
        }
    }
    #[cfg(target_arch = "x86_64")]
    {
        if let Some(slot) = shim_xmm_slot_ptr(rbp) {
            return [
                core::ptr::read_unaligned(slot),
                core::ptr::read_unaligned(slot.add(1)),
                core::ptr::read_unaligned(slot.add(2)),
                core::ptr::read_unaligned(slot.add(3)),
            ];
        }
        reg_buf
    }
    #[cfg(target_arch = "aarch64")]
    {
        reg_buf
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        crate::callctx::ctx().xmm_in.get()
    }
}

#[inline(always)]
pub unsafe fn set_xmm_ret(v: u128) {
    crate::callctx::ctx().xmm_out.set(Some(v));
    #[cfg(target_arch = "x86_64")]
    {
        let mut rbp: usize;
        let tmp = v;
        core::arch::asm!(
            "mov {rbp}, rbp",
            "movups xmm0, [{p}]",
            rbp = out(reg) rbp,
            p = in(reg) &tmp,
            options(nostack, preserves_flags),
        );
        if let Some(slot) = shim_xmm_slot_ptr(rbp) {
            core::ptr::write_unaligned(slot, v);
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        let tmp = v;
        core::arch::asm!(
            "ldr q0, [{p}]",
            p = in(reg) &tmp,
            options(nostack, preserves_flags),
        );
    }
}

#[inline]
pub unsafe fn call_va8_xmm(
    va_or_addr: u64,
    rcx: u64,
    rdx: u64,
    r8: u64,
    r9: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
    arg8: u64,
    xmm: [u128; 4],
) -> (u64, u128) {
    #[cfg(target_arch = "x86_64")]
    {
        let addr = crate::exe::lookup_trampoline_or_mapped(va_or_addr);
        set_tls_xmm_in(xmm);
        let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =
            core::mem::transmute(addr);
        let mut out_x0 = 0u128;
        core::arch::asm!(
            "movups xmm0, [{xin} + 0x00]",
            "movups xmm1, [{xin} + 0x10]",
            "movups xmm2, [{xin} + 0x20]",
            "movups xmm3, [{xin} + 0x30]",
            xin = in(reg) xmm.as_ptr(),
            options(nostack, preserves_flags),
        );
        let r = f(rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
        core::arch::asm!(
            "movups [{xout}], xmm0",
            xout = in(reg) &mut out_x0,
            options(nostack, preserves_flags),
        );
        (r, take_tls_xmm_out(out_x0))
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let c = crate::callctx::ctx();
        if crate::callprof::on() {
            crate::callprof::hit(c, va_or_addr, crate::emu::resolve_dispatch_in(c, va_or_addr), crate::callprof::ENTRY_XMM);
        }
        let (host_fn, flags) = crate::emu::resolve_dispatch_in(c, va_or_addr);
        #[cfg(target_arch = "aarch64")]
        if flags == crate::emu::LEAN_FLAGS {
            // Compiled replacement: `call_va8_xmm_bridge`'s `DF_HOST` protocol, host call
            // via `emu::lean_call` (no callee-saved fence).
            let (out_rsp, out_limit) = c.outgoing_rsp.replace((0, 0));
            let s_extra = crate::emu::read_caller_extra_args(out_rsp, out_limit);
            let prev_rax = c.rax_in.get();
            c.incoming_rsp.set((out_rsp, out_limit));
            let r = crate::emu::call_lean_in(c, host_fn, &[rcx, rdx, r8, r9, arg5, arg6, arg7, arg8], &xmm, &s_extra);
            c.rax_in.set(prev_rax);
            c.incoming_rsp.set((0, 0));
            return r;
        }
        call_va8_xmm_bridge(c, va_or_addr, host_fn, flags, [rcx, rdx, r8, r9, arg5, arg6, arg7, arg8], xmm)
    }
}

/// The generic part of [`call_va8_xmm`] (host functions through the Win64 bridge, guest
/// code through `emu_run_full`). Out of line so the fast path keeps a small frame.
#[cfg(not(target_arch = "x86_64"))]
#[inline(never)]
unsafe fn call_va8_xmm_bridge(
    c: &crate::callctx::CallCtx,
    va_or_addr: u64,
    host_fn: usize,
    flags: u32,
    gpr: [u64; 8],
    xmm: [u128; 4],
) -> (u64, u128) {
    #[cfg(target_arch = "aarch64")]
    if flags & crate::emu::DF_SHADOW != 0 {
        return crate::emu::shadow_lean_xmm(c, va_or_addr, host_fn, gpr, xmm);
    }
    let (out_rsp, out_limit) = c.outgoing_rsp.replace((0, 0));
    let s_extra = crate::emu::read_caller_extra_args(out_rsp, out_limit);
    let prev_rax = c.rax_in.get();
    if flags & crate::emu::DF_HOST != 0 {
        // (call_host_dispatched_in publishes `xmm` to TLS for the callee.)
        c.incoming_rsp.set((out_rsp, out_limit));
        let (r_gpr, r_xmm) =
            crate::emu::call_host_dispatched_in(c, va_or_addr, host_fn, flags, &gpr, &xmm, &s_extra);
        c.rax_in.set(prev_rax);
        c.incoming_rsp.set((0, 0));
        return (r_gpr, r_xmm);
    }
    let target = (crate::exe::lookup_trampoline_or_mapped(va_or_addr) & !(1usize << 46)) as u64;
    crate::emu::set_incoming_rsp(out_rsp, out_limit);
    let r = crate::emu::emu_run_full_rax(target, gpr, xmm, s_extra, prev_rax);
    replace_rax_in(prev_rax);
    crate::emu::set_incoming_rsp(0, 0);
    r
}

#[inline]
pub unsafe fn call_va8_caught(
    va_or_addr: u64,
    rcx: u64,
    rdx: u64,
    r8: u64,
    r9: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
    arg8: u64,
) -> Result<u64, crate::exe::CxxException> {
    let (out_rsp, out_limit) = crate::emu::take_outgoing_rsp();
    let s_extra = crate::emu::read_caller_extra_args(out_rsp, out_limit);
    #[cfg(target_arch = "x86_64")]
    let target = crate::exe::lookup_trampoline_or_mapped(va_or_addr);
    #[cfg(not(target_arch = "x86_64"))]
    let target = crate::exe::lookup_trampoline_or_mapped(va_or_addr) & !(1usize << 46);
    let prev_rax = get_rax_in();
    crate::emu::set_incoming_rsp(out_rsp, out_limit);
    let res = crate::exe::try_call(
        target,
        &[
            rcx, rdx, r8, r9, arg5, arg6, arg7, arg8,
            s_extra[0], s_extra[1], s_extra[2], s_extra[3],
            s_extra[4], s_extra[5], s_extra[6], s_extra[7],
        ],
        &[],
    )
    .map(|r| r.rax);
    replace_rax_in(prev_rax);
    crate::emu::set_incoming_rsp(0, 0);
    res
}

const BIG_FRAME_QWORDS: usize = 32768; // 256 KB per large lifted frame
const BIG_ARENA_SLOTS: usize = 256; // 64 MB per thread (demand-paged)

struct BigLiftedStack {
    buf: Box<[u64]>,
    frames: Vec<(usize, usize)>,
}

std::thread_local! {
    static TLS_BIG_STACK: core::cell::UnsafeCell<Option<BigLiftedStack>> =
        const { core::cell::UnsafeCell::new(None) };
}

/// Shadow orchestrator: this thread's big-frame arena, if it exists. Returns the scratch
/// `[lo, hi)` (every slot above the innermost frame still live at `hook_sp`; slots are not
/// zeroed, so they hold stale frames) and the frame-list buffer (host-stack addresses and
/// slot indices: runtime bookkeeping, never compared).
pub(crate) fn shadow_big_arena(hook_sp: usize) -> Option<((usize, usize), (usize, usize))> {
    TLS_BIG_STACK.with(|cell| {
        let st = unsafe { (*cell.get()).as_ref()? };
        let base = st.buf.as_ptr() as usize;
        let end = base + st.buf.len() * 8;
        let live = st.frames.iter().filter(|&&(h, _)| h > hook_sp).map(|&(_, i)| i + 1).max().unwrap_or(0);
        let lo = (base + live * BIG_FRAME_QWORDS * 8).min(end);
        let fp = st.frames.as_ptr() as usize;
        Some(((lo, end), (fp, fp + st.frames.capacity() * core::mem::size_of::<(usize, usize)>())))
    })
}

/// Bytes of one big frame slot (shadow poisoning window).
pub(crate) const BIG_FRAME_BYTES: usize = BIG_FRAME_QWORDS * 8;

/// `[lo, hi)` of this thread's big-frame arena if it exists (EH establisher bounds).
pub fn big_frame_arena_if_any() -> Option<(usize, usize)> {
    TLS_BIG_STACK.with(|cell| {
        let st = unsafe { (*cell.get()).as_ref()? };
        let base = st.buf.as_ptr() as usize;
        Some((base, base + st.buf.len() * 8))
    })
}

/// `[lo, hi)` of this thread's big-frame arena (`LiftedStack::<1>` frames: lifted functions
/// with a frame over 2 KB), created now if it does not exist yet. The dll_lift difftest counts
/// addresses in it as the lifted run's stack ("rax_stack", "gs_residue").
pub fn big_frame_arena() -> (usize, usize) {
    TLS_BIG_STACK.with(|cell| {
        let st = unsafe { &mut *cell.get() }.get_or_insert_with(|| BigLiftedStack {
            buf: vec![0u64; BIG_FRAME_QWORDS * BIG_ARENA_SLOTS].into_boxed_slice(),
            frames: Vec::with_capacity(16),
        });
        let base = st.buf.as_ptr() as usize;
        (base, base + st.buf.len() * 8)
    })
}

/// Entry `rsp` of a fresh big frame: `args` argument qwords below the slot end.
#[inline(never)]
unsafe fn alloc_big_lifted_frame(host_addr: usize, args: usize) -> *mut u64 {
    TLS_BIG_STACK.with(|cell| {
        let opt = &mut *cell.get();
        let state = opt.get_or_insert_with(|| BigLiftedStack {
            buf: vec![0u64; BIG_FRAME_QWORDS * BIG_ARENA_SLOTS].into_boxed_slice(),
            frames: Vec::with_capacity(16),
        });
        while let Some(&(prev_host, _)) = state.frames.last() {
            if prev_host <= host_addr {
                state.frames.pop();
            } else {
                break;
            }
        }
        let slot = state.frames.last().map(|&(_, idx)| idx + 1).unwrap_or(0);
        if slot >= BIG_ARENA_SLOTS {
            eprintln!("aw2: lifted big-frame arena exhausted ({BIG_ARENA_SLOTS} nested 256 KB frames)");
            std::process::abort();
        }
        state.frames.push((host_addr, slot));
        let top_qword = (slot + 1) * BIG_FRAME_QWORDS - args;
        state.buf.as_mut_ptr().add(top_qword)
    })
}

/// Guest stack buffer of a lifted frame. Zero-initialized like the original
/// `[0u64; N]`: lifted code observably depends on it, including slots below the
/// statically computed frame (leaving the buffer uninitialized, or zeroing only
/// the `frame / 8 + 64` top window, both broke gameplay lighting: the slots below
/// the window then held host-stack garbage). The exception is
/// `N == 1`: big frames live in the TLS arena and the local is only a host-address
/// marker, so it needs no initialization (this used to be a 32 KB memset per call).
/// The zeroing happens in [`StackBuf::frame_base`] (`init_lifted_stack`), in place, with
/// `dc zva` ([`crate::lstack::zero_bytes`]) instead of a `bzero` call.
/// Most generated code now uses [`LiftedFrame`] (clean per-thread arena) instead.
/// Declared by generated code as `let mut stack = crate::simd_ops::LiftedStack::<N>::new();`.
/// `A`: argument qwords above the entry `rsp` (return slot, home space, args 5..A-1); see
/// [`StackBuf::ARGS`].
#[repr(transparent)]
pub struct LiftedStack<const N: usize, const A: usize = 24>(core::mem::MaybeUninit<[u64; N]>);

impl<const N: usize, const A: usize> LiftedStack<N, A> {
    #[inline(always)]
    pub fn new() -> Self {
        LiftedStack(core::mem::MaybeUninit::uninit())
    }
}

pub use crate::lstack::LiftedFrame;

impl<const N: usize, const A: usize> Default for LiftedStack<N, A> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

/// A lifted frame's guest stack storage (`[u64; N]`, [`LiftedStack`] or [`LiftedFrame`]).
pub trait StackBuf {
    const N: usize;
    /// Argument qwords above the entry `rsp`: the return slot, the home space and incoming
    /// args 5..ARGS-1 (`init_lifted_stack` stores 5..8 and copies 9..ARGS-1 from the
    /// caller's outgoing area). 24 for every frame except wide ones (`LiftedFrame<N, F, A>`
    /// / `LiftedStack<N, A>` for functions reading arg24+).
    const ARGS: usize = 24;
    fn base(&self) -> *const u64;
    fn base_mut(&mut self) -> *mut u64;

    /// Guest `rsp` at entry (the return-address slot; `ARGS - 1` argument qwords sit above
    /// it), with every slot zero. Called once per frame, by `init_lifted_stack*`.
    #[inline(always)]
    unsafe fn frame_base(&mut self) -> *mut u64 {
        if Self::N == 1 || Self::N >= 4096 {
            alloc_big_lifted_frame(self.base() as usize, Self::ARGS)
        } else {
            self.base_mut().add(Self::N - Self::ARGS)
        }
    }

    /// Upper bound of this frame's readable guest stack for the callee's stack-argument copy
    /// (`read_s5_call` / `read_s5_tail`); `big` is the bound relative to `rsp` for arena frames.
    #[inline(always)]
    fn out_limit(&self, rsp: u64, big: u64) -> u64 {
        if Self::N == 1 || Self::N >= 4096 {
            rsp.wrapping_add(big)
        } else {
            unsafe { self.base().add(Self::N) as u64 }
        }
    }
}

impl<const N: usize> StackBuf for [u64; N] {
    const N: usize = N;
    #[inline(always)]
    fn base(&self) -> *const u64 {
        self.as_ptr()
    }
    #[inline(always)]
    fn base_mut(&mut self) -> *mut u64 {
        self.as_mut_ptr()
    }
}

impl<const N: usize, const A: usize> StackBuf for LiftedStack<N, A> {
    const N: usize = N;
    const ARGS: usize = A;
    #[inline(always)]
    fn base(&self) -> *const u64 {
        self.0.as_ptr() as *const u64
    }
    #[inline(always)]
    fn base_mut(&mut self) -> *mut u64 {
        let p = self.0.as_mut_ptr() as *mut u64;
        // The guest reads its stack through integer-derived pointers and may
        // read slots it never wrote (as on real hardware). An opaque asm use
        // of the pointer makes LLVM treat the contents as unknown, not undef.
        unsafe { core::arch::asm!("/* {0} */", in(reg) p, options(nostack, preserves_flags)) };
        p
    }
    #[inline(always)]
    unsafe fn frame_base(&mut self) -> *mut u64 {
        if N == 1 || N >= 4096 {
            alloc_big_lifted_frame(self.base() as usize, A)
        } else {
            let p = self.base_mut();
            crate::lstack::zero_bytes(p as *mut u8, N * 8);
            p.add(N - A)
        }
    }
}

#[inline(always)]
pub unsafe fn init_lifted_stack<S: StackBuf>(
    stack: &mut S,
    arg5: u64,
    arg6: u64,
    arg7: u64,
    arg8: u64,
) -> u64 {
    let base_ptr = stack.frame_base();
    *base_ptr.add(5) = arg5;
    *base_ptr.add(6) = arg6;
    *base_ptr.add(7) = arg7;
    *base_ptr.add(8) = arg8;
    copy_incoming_stack_args::<S>(base_ptr);
    base_ptr as u64
}

#[inline(always)]
pub unsafe fn init_lifted_stack12<S: StackBuf>(
    stack: &mut S,
    arg5: u64,
    arg6: u64,
    arg7: u64,
    arg8: u64,
    arg9: u64,
    arg10: u64,
    arg11: u64,
    arg12: u64,
) -> u64 {
    let base_ptr = stack.frame_base();
    *base_ptr.add(5) = arg5;
    *base_ptr.add(6) = arg6;
    *base_ptr.add(7) = arg7;
    *base_ptr.add(8) = arg8;
    *base_ptr.add(9) = arg9;
    *base_ptr.add(10) = arg10;
    *base_ptr.add(11) = arg11;
    *base_ptr.add(12) = arg12;
    copy_incoming_stack_args::<S>(base_ptr);
    base_ptr as u64
}

/// Copy the caller's stack arguments 9..ARGS-1 (`[caller_rsp + 0x40 .. + ARGS*8)`; 9..23 for
/// the default 24-qword window) into slots 9.. of a fresh lifted frame. The window is the
/// caller's own outgoing-argument area, written by this thread just before the call, so plain
/// loads are enough (no TSO `ldapr`); one bounds check covers the common fully-in-range case.
#[inline(always)]
unsafe fn copy_incoming_stack_args<S: StackBuf>(base_ptr: *mut u64) {
    let n = const {
        assert!(S::ARGS >= 9, "a lifted frame's argument window covers at least args 1..8");
        S::ARGS - 9
    };
    let (caller_rsp, rsp_limit) = crate::emu::take_incoming_rsp();
    if n == 0 || caller_rsp == 0 {
        return;
    }
    let src = caller_rsp.wrapping_add(0x40);
    let end = src.wrapping_add(n as u64 * 8);
    if src <= end && end <= rsp_limit {
        core::ptr::copy_nonoverlapping(src as *const u8, base_ptr.add(9) as *mut u8, n * 8);
    } else {
        copy_incoming_stack_args_partial(base_ptr, caller_rsp, rsp_limit, n);
    }
}

#[cold]
#[inline(never)]
unsafe fn copy_incoming_stack_args_partial(base_ptr: *mut u64, caller_rsp: u64, rsp_limit: u64, n: usize) {
    for i in 0..n {
        let addr = caller_rsp.wrapping_add(0x40 + (i as u64) * 8);
        if addr.wrapping_add(8) <= rsp_limit {
            *base_ptr.add(9 + i) = core::ptr::read_unaligned(addr as *const u64);
        }
    }
}

#[inline(always)]
pub unsafe fn read_s5_call<S: StackBuf>(rsp: u64, stack: &S) -> u64 {
    let limit = stack.out_limit(rsp, 0xc0);
    crate::emu::set_outgoing_rsp(rsp, limit);
    core::ptr::read_unaligned(rsp.wrapping_add(0x20) as *const u64)
}

#[inline(always)]
pub unsafe fn read_s5_tail<S: StackBuf>(rsp: u64, stack: &S) -> u64 {
    let limit = stack.out_limit(rsp, 0xc8);
    crate::emu::set_outgoing_rsp(rsp.wrapping_add(8), limit);
    core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64)
}

#[inline(always)]
pub fn forward_incoming_to_outgoing() {
    let (rsp, limit) = crate::emu::take_incoming_rsp();
    crate::emu::set_outgoing_rsp(rsp, limit);
}

// Guest memory accesses from lifted code.
//
// x86-64 is TSO: every load is an acquire and every store a release, and the original
// code relies on that for lock-free publication (job queues, frame fences, ref counts) with
// plain `mov`s. aarch64 is weakly ordered and LLVM may also reorder or merge plain accesses,
// so on aarch64 naturally aligned scalar accesses use `ldapr`/`stlr` (Acquire/Release
// atomics), matching the interpreter/JIT TSO emulation in `emu::mem`. Unaligned scalar and
// 128-bit accesses stay plain: lock-free variables are naturally aligned, and fencing every
// `movups` would cost far more than it buys.
#[allow(unused_macros)]
macro_rules! tso_access {
    ($rd:ident, $wr:ident, $t:ty, $at:ty, $align:expr) => {
        #[inline(always)]
        pub unsafe fn $rd(a: u64) -> $t {
            #[cfg(target_arch = "aarch64")]
            {
                if a & ($align - 1) == 0 {
                    return (*(a as *const $at)).load(core::sync::atomic::Ordering::Acquire);
                }
            }
            core::ptr::read_unaligned(a as *const $t)
        }
        #[inline(always)]
        pub unsafe fn $wr(a: u64, v: $t) {
            #[cfg(target_arch = "aarch64")]
            {
                if a & ($align - 1) == 0 {
                    (*(a as *const $at)).store(v, core::sync::atomic::Ordering::Release);
                    return;
                }
            }
            core::ptr::write_unaligned(a as *mut $t, v);
        }
    };
}

// x86 memory read-modify-write (`[lock] add/sub/or/and/xor/inc/dec [mem], x`) from lifted
// code; returns the new value. The shard lifter dropped `lock` prefixes, so every RMW goes
// through an atomic op when naturally aligned (a locked RMW is also a valid implementation
// of an unlocked one). Unaligned operands fall back to a plain load/store.
#[allow(unused_macros)]
macro_rules! lk_rmw {
    ($name:ident, $t:ty, $at:ty, $align:expr, $fetch:ident, $combine:expr) => {
        #[inline(always)]
        pub unsafe fn $name(a: u64, v: $t) -> $t {
            let f: fn($t, $t) -> $t = $combine;
            if a & ($align - 1) == 0 {
                let old = (*(a as *const $at)).$fetch(v, core::sync::atomic::Ordering::AcqRel);
                f(old, v)
            } else {
                let new = f(core::ptr::read_unaligned(a as *const $t), v);
                core::ptr::write_unaligned(a as *mut $t, new);
                new
            }
        }
    };
}
#[allow(unused_macros)]
macro_rules! lk_rmw_all {
    ($t:ty, $at:ty, $align:expr, $add:ident, $sub:ident, $or:ident, $and:ident, $xor:ident) => {
        lk_rmw!($add, $t, $at, $align, fetch_add, |o, v| o.wrapping_add(v));
        lk_rmw!($sub, $t, $at, $align, fetch_sub, |o, v| o.wrapping_sub(v));
        lk_rmw!($or, $t, $at, $align, fetch_or, |o, v| o | v);
        lk_rmw!($and, $t, $at, $align, fetch_and, |o, v| o & v);
        lk_rmw!($xor, $t, $at, $align, fetch_xor, |o, v| o ^ v);
    };
}
// Portable versions (x86_64 / Rosetta build, or aarch64 without FEAT_LRCPC + FEAT_LSE).
#[cfg(not(all(target_arch = "aarch64", target_feature = "rcpc", target_feature = "lse")))]
mod tso_generic {
tso_access!(ru8, wu8, u8, core::sync::atomic::AtomicU8, 1);
tso_access!(ru16, wu16, u16, core::sync::atomic::AtomicU16, 2);
tso_access!(ru32, wu32, u32, core::sync::atomic::AtomicU32, 4);
tso_access!(ru64, wu64, u64, core::sync::atomic::AtomicU64, 8);
lk_rmw_all!(u8, core::sync::atomic::AtomicU8, 1, lk_add_u8, lk_sub_u8, lk_or_u8, lk_and_u8, lk_xor_u8);
lk_rmw_all!(u16, core::sync::atomic::AtomicU16, 2, lk_add_u16, lk_sub_u16, lk_or_u16, lk_and_u16, lk_xor_u16);
lk_rmw_all!(u32, core::sync::atomic::AtomicU32, 4, lk_add_u32, lk_sub_u32, lk_or_u32, lk_and_u32, lk_xor_u32);
lk_rmw_all!(u64, core::sync::atomic::AtomicU64, 8, lk_add_u64, lk_sub_u64, lk_or_u64, lk_and_u64, lk_xor_u64);
}
#[cfg(not(all(target_arch = "aarch64", target_feature = "rcpc", target_feature = "lse")))]
pub use tso_generic::*;

// aarch64: aw2-game is built at opt-level 0 (see Cargo.toml), where the Atomic* methods are
// out-of-line call chains with a runtime `match` on the ordering. One `asm!` block per helper
// is always emitted in place and is also a compiler memory barrier. Same semantics as the
// portable versions: naturally aligned -> ldapr/stlr and LSE ld<op>al (fully ordered, i.e.
// `lock`), otherwise plain; lk_* return the NEW value. Needs FEAT_LRCPC + FEAT_LSE (all Apple
// M-series / A14+; enabled by default for aarch64-apple-darwin and via .cargo/config.toml on iOS).
#[cfg(all(target_arch = "aarch64", target_feature = "rcpc", target_feature = "lse"))]
mod tso_a64 {
// $chk: asm that branches to local label `2` when the access must take the plain path.
macro_rules! tso_access_asm {
    ($rd:ident, $wr:ident, $t:ty, $chk:literal, $ldapr:literal, $ldr:literal, $stlr:literal, $str:literal, $r:literal) => {
        #[inline(always)]
        pub unsafe fn $rd(a: u64) -> $t {
            let v: u64;
            core::arch::asm!(
                $chk,
                concat!($ldapr, " {v:", $r, "}, [{a}]"),
                "b 3f",
                concat!("2: ", $ldr, " {v:", $r, "}, [{a}]"),
                "3:",
                a = in(reg) a, v = lateout(reg) v,
                options(nostack),
            );
            v as $t
        }
        #[inline(always)]
        pub unsafe fn $wr(a: u64, v: $t) {
            core::arch::asm!(
                $chk,
                concat!($stlr, " {v:", $r, "}, [{a}]"),
                "b 3f",
                concat!("2: ", $str, " {v:", $r, "}, [{a}]"),
                "3:",
                a = in(reg) a, v = in(reg) v as u64,
                options(nostack),
            );
        }
    };
}
// (u8 is always aligned; `cmp xzr, xzr; b.ne` never branches)
tso_access_asm!(ru8, wu8, u8, "cmp xzr, xzr\nb.ne 2f", "ldaprb", "ldrb", "stlrb", "strb", "w");
tso_access_asm!(ru16, wu16, u16, "tst {a}, #1\nb.ne 2f", "ldaprh", "ldrh", "stlrh", "strh", "w");
tso_access_asm!(ru32, wu32, u32, "tst {a}, #3\nb.ne 2f", "ldapr", "ldr", "stlr", "str", "w");
tso_access_asm!(ru64, wu64, u64, "tst {a}, #7\nb.ne 2f", "ldapr", "ldr", "stlr", "str", "x");

// x86 memory RMW -> LSE atomic. ld<op>al = acquire+release, which on AArch64 is fully ordered
// (Linux uses bare ldaddal for its fully-ordered atomic_*_return), i.e. equivalent to `lock`.
// Returns the NEW value. One operand register serves both paths:
//   add: ldaddal v / add         sub: neg; ldaddal -v / add
//   or : ldsetal v / orr         and: mvn; ldclral ~v / bic        xor: ldeoral v / eor
macro_rules! lk_rmw_asm {
    ($name:ident, $t:ty, $chk:literal, $prep:expr, $lse:literal, $sfx:literal, $ldr:literal, $str:literal, $comb:literal, $r:literal) => {
        #[inline(always)]
        pub unsafe fn $name(a: u64, v: $t) -> $t {
            let new: u64;
            core::arch::asm!(
                $prep,
                $chk,
                concat!($lse, $sfx, " {v:", $r, "}, {o:", $r, "}, [{a}]"),
                concat!($comb, " {n:", $r, "}, {o:", $r, "}, {v:", $r, "}"),
                "b 3f",
                concat!("2: ", $ldr, " {o:", $r, "}, [{a}]"),
                concat!($comb, " {n:", $r, "}, {o:", $r, "}, {v:", $r, "}"),
                concat!($str, " {n:", $r, "}, [{a}]"),
                "3:",
                a = in(reg) a, v = inout(reg) v as u64 => _, o = out(reg) _, n = out(reg) new,
                options(nostack),
            );
            new as $t
        }
    };
}
macro_rules! lk_rmw_all_asm {
    ($t:ty, $chk:literal, $sfx:literal, $ldr:literal, $str:literal, $r:literal,
     $add:ident, $sub:ident, $or:ident, $and:ident, $xor:ident) => {
        lk_rmw_asm!($add, $t, $chk, "", "ldaddal", $sfx, $ldr, $str, "add", $r);
        lk_rmw_asm!($sub, $t, $chk, concat!("neg {v:", $r, "}, {v:", $r, "}"), "ldaddal", $sfx, $ldr, $str, "add", $r);
        lk_rmw_asm!($or, $t, $chk, "", "ldsetal", $sfx, $ldr, $str, "orr", $r);
        lk_rmw_asm!($and, $t, $chk, concat!("mvn {v:", $r, "}, {v:", $r, "}"), "ldclral", $sfx, $ldr, $str, "bic", $r);
        lk_rmw_asm!($xor, $t, $chk, "", "ldeoral", $sfx, $ldr, $str, "eor", $r);
    };
}
lk_rmw_all_asm!(u8, "cmp xzr, xzr\nb.ne 2f", "b", "ldrb", "strb", "w", lk_add_u8, lk_sub_u8, lk_or_u8, lk_and_u8, lk_xor_u8);
lk_rmw_all_asm!(u16, "tst {a}, #1\nb.ne 2f", "h", "ldrh", "strh", "w", lk_add_u16, lk_sub_u16, lk_or_u16, lk_and_u16, lk_xor_u16);
lk_rmw_all_asm!(u32, "tst {a}, #3\nb.ne 2f", "", "ldr", "str", "w", lk_add_u32, lk_sub_u32, lk_or_u32, lk_and_u32, lk_xor_u32);
lk_rmw_all_asm!(u64, "tst {a}, #7\nb.ne 2f", "", "ldr", "str", "x", lk_add_u64, lk_sub_u64, lk_or_u64, lk_and_u64, lk_xor_u64);
}
#[cfg(all(target_arch = "aarch64", target_feature = "rcpc", target_feature = "lse"))]
pub use tso_a64::*;
lk_rmw_all!(u8, core::sync::atomic::AtomicU8, 1, lk_add_u8, lk_sub_u8, lk_or_u8, lk_and_u8, lk_xor_u8);
lk_rmw_all!(u16, core::sync::atomic::AtomicU16, 2, lk_add_u16, lk_sub_u16, lk_or_u16, lk_and_u16, lk_xor_u16);
lk_rmw_all!(u32, core::sync::atomic::AtomicU32, 4, lk_add_u32, lk_sub_u32, lk_or_u32, lk_and_u32, lk_xor_u32);
lk_rmw_all!(u64, core::sync::atomic::AtomicU64, 8, lk_add_u64, lk_sub_u64, lk_or_u64, lk_and_u64, lk_xor_u64);

#[inline(always)]
pub unsafe fn ru128(a: u64) -> u128 {
    core::ptr::read_unaligned(a as *const u128)
}
#[inline(always)]
pub unsafe fn wu128(a: u64, v: u128) {
    core::ptr::write_unaligned(a as *mut u128, v);
}

#[inline(always)]
pub fn ind_tgt(t: u64, rax: u64) -> u64 {
    if t == 0x1_4254_7700 { rax } else { t }
}

#[inline]
pub unsafe fn call_va8_rethrow(
    va: u64, rcx: u64, rdx: u64, r8: u64, r9: u64,
    a5: u64, a6: u64, a7: u64, a8: u64,
) -> u64 {
    match call_va8_caught(va, rcx, rdx, r8, r9, a5, a6, a7, a8) {
        Ok(v) => v,
        Err(e) => e.rethrow(),
    }
}

/// Read a stack argument from the caller's own lifted frame (`rsp`-relative outgoing
/// area, written by this thread): plain load, no TSO `ldapr`.
#[inline(always)]
unsafe fn stk64(a: u64) -> u64 {
    core::ptr::read_unaligned(a as *const u64)
}

// `call_stk*` publish the caller's outgoing area `[rsp, rsp + 0xc0)` (args 5..24) to the
// callee; `tail_stk*` publish `[rsp + 8, rsp + 0xc8)`. The `_w::<LIM>` forms publish
// `rsp + LIM` instead (both relative to the call site's `rsp`): the generator emits them for
// call sites that write their outgoing area at `[rsp + X]`, `X >= 0xc0` (`LIM = X_max + 8`),
// or whose direct target has a wider argument window (`LiftedFrame<N, F, A>`, `LIM = A*8`).
// Lifted callees copy up to their window, `emu_run_full` up to arg32.

#[inline]
pub unsafe fn call_stk(va: u64, rax: u64, rcx: u64, rdx: u64, r8: u64, r9: u64, rsp: u64) -> u64 {
    call_stk_w::<0xc0>(va, rax, rcx, rdx, r8, r9, rsp)
}

#[inline(always)]
pub unsafe fn call_stk_w<const LIM: u64>(va: u64, rax: u64, rcx: u64, rdx: u64, r8: u64, r9: u64, rsp: u64) -> u64 {
    crate::emu::set_outgoing_rsp(rsp, rsp.wrapping_add(LIM));
    replace_rax_in(rax);
    crate::exe::call_va8(
        va, rcx, rdx, r8, r9,
        stk64(rsp.wrapping_add(0x20)),
        stk64(rsp.wrapping_add(0x28)),
        stk64(rsp.wrapping_add(0x30)),
        stk64(rsp.wrapping_add(0x38)),
    )
}

#[inline]
pub unsafe fn call_stk_caught(va: u64, rax: u64, rcx: u64, rdx: u64, r8: u64, r9: u64, rsp: u64) -> u64 {
    call_stk_caught_w::<0xc0>(va, rax, rcx, rdx, r8, r9, rsp)
}

#[inline(always)]
pub unsafe fn call_stk_caught_w<const LIM: u64>(va: u64, rax: u64, rcx: u64, rdx: u64, r8: u64, r9: u64, rsp: u64) -> u64 {
    crate::emu::set_outgoing_rsp(rsp, rsp.wrapping_add(LIM));
    replace_rax_in(rax);
    call_va8_rethrow(
        va, rcx, rdx, r8, r9,
        stk64(rsp.wrapping_add(0x20)),
        stk64(rsp.wrapping_add(0x28)),
        stk64(rsp.wrapping_add(0x30)),
        stk64(rsp.wrapping_add(0x38)),
    )
}

#[inline]
pub unsafe fn call_stk_xmm(va: u64, rax: u64, rcx: u64, rdx: u64, r8: u64, r9: u64, rsp: u64, xmm: [u128; 4]) -> (u64, u128) {
    call_stk_xmm_w::<0xc0>(va, rax, rcx, rdx, r8, r9, rsp, xmm)
}

#[inline(always)]
pub unsafe fn call_stk_xmm_w<const LIM: u64>(
    va: u64, rax: u64, rcx: u64, rdx: u64, r8: u64, r9: u64, rsp: u64, xmm: [u128; 4],
) -> (u64, u128) {
    crate::emu::set_outgoing_rsp(rsp, rsp.wrapping_add(LIM));
    replace_rax_in(rax);
    call_va8_xmm(
        va, rcx, rdx, r8, r9,
        stk64(rsp.wrapping_add(0x20)),
        stk64(rsp.wrapping_add(0x28)),
        stk64(rsp.wrapping_add(0x30)),
        stk64(rsp.wrapping_add(0x38)),
        xmm,
    )
}

#[inline]
pub unsafe fn tail_stk(va: u64, rax: u64, rcx: u64, rdx: u64, r8: u64, r9: u64, rsp: u64) -> u64 {
    tail_stk_w::<0xc8>(va, rax, rcx, rdx, r8, r9, rsp)
}

#[inline(always)]
pub unsafe fn tail_stk_w<const LIM: u64>(va: u64, rax: u64, rcx: u64, rdx: u64, r8: u64, r9: u64, rsp: u64) -> u64 {
    crate::emu::set_outgoing_rsp(rsp.wrapping_add(8), rsp.wrapping_add(LIM));
    replace_rax_in(rax);
    crate::exe::call_va8(
        va, rcx, rdx, r8, r9,
        stk64(rsp.wrapping_add(0x28)),
        stk64(rsp.wrapping_add(0x30)),
        stk64(rsp.wrapping_add(0x38)),
        stk64(rsp.wrapping_add(0x40)),
    )
}

#[inline]
pub unsafe fn tail_stk_caught(va: u64, rax: u64, rcx: u64, rdx: u64, r8: u64, r9: u64, rsp: u64) -> u64 {
    tail_stk_caught_w::<0xc8>(va, rax, rcx, rdx, r8, r9, rsp)
}

#[inline(always)]
pub unsafe fn tail_stk_caught_w<const LIM: u64>(va: u64, rax: u64, rcx: u64, rdx: u64, r8: u64, r9: u64, rsp: u64) -> u64 {
    crate::emu::set_outgoing_rsp(rsp.wrapping_add(8), rsp.wrapping_add(LIM));
    replace_rax_in(rax);
    call_va8_rethrow(
        va, rcx, rdx, r8, r9,
        stk64(rsp.wrapping_add(0x28)),
        stk64(rsp.wrapping_add(0x30)),
        stk64(rsp.wrapping_add(0x38)),
        stk64(rsp.wrapping_add(0x40)),
    )
}

#[inline]
pub unsafe fn tail_stk_xmm(va: u64, rax: u64, rcx: u64, rdx: u64, r8: u64, r9: u64, rsp: u64, xmm: [u128; 4]) -> u64 {
    tail_stk_xmm_w::<0xc8>(va, rax, rcx, rdx, r8, r9, rsp, xmm)
}

#[inline(always)]
pub unsafe fn tail_stk_xmm_w<const LIM: u64>(
    va: u64, rax: u64, rcx: u64, rdx: u64, r8: u64, r9: u64, rsp: u64, xmm: [u128; 4],
) -> u64 {
    let (c_rax, c_x0) = call_va8_xmm(
        va, rcx, rdx, r8, r9,
        { crate::emu::set_outgoing_rsp(rsp.wrapping_add(8), rsp.wrapping_add(LIM)); replace_rax_in(rax); stk64(rsp.wrapping_add(0x28)) },
        stk64(rsp.wrapping_add(0x30)),
        stk64(rsp.wrapping_add(0x38)),
        stk64(rsp.wrapping_add(0x40)),
        xmm,
    );
    set_xmm_ret(c_x0);
    c_rax
}

#[inline]
pub unsafe fn call_fwd(va: u64, rax: u64, rcx: u64, rdx: u64, r8: u64, r9: u64, a5: u64, a6: u64, a7: u64, a8: u64) -> u64 {
    forward_incoming_to_outgoing();
    replace_rax_in(rax);
    crate::exe::call_va8(va, rcx, rdx, r8, r9, a5, a6, a7, a8)
}

#[inline]
pub unsafe fn call_fwd_caught(va: u64, rax: u64, rcx: u64, rdx: u64, r8: u64, r9: u64, a5: u64, a6: u64, a7: u64, a8: u64) -> u64 {
    forward_incoming_to_outgoing();
    replace_rax_in(rax);
    call_va8_rethrow(va, rcx, rdx, r8, r9, a5, a6, a7, a8)
}

#[inline]
pub unsafe fn call_fwd_xmm(va: u64, rax: u64, rcx: u64, rdx: u64, r8: u64, r9: u64, a5: u64, a6: u64, a7: u64, a8: u64, xmm: [u128; 4]) -> (u64, u128) {
    forward_incoming_to_outgoing();
    replace_rax_in(rax);
    call_va8_xmm(va, rcx, rdx, r8, r9, a5, a6, a7, a8, xmm)
}

#[inline]
pub unsafe fn tail_fwd_xmm(va: u64, rax: u64, rcx: u64, rdx: u64, r8: u64, r9: u64, a5: u64, a6: u64, a7: u64, a8: u64, xmm: [u128; 4]) -> u64 {
    let (c_rax, c_x0) = call_fwd_xmm(va, rax, rcx, rdx, r8, r9, a5, a6, a7, a8, xmm);
    set_xmm_ret(c_x0);
    c_rax
}

static LIFTED_UNALIGNED_ATOMIC_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

macro_rules! impl_lifted_atomics {
    ($bits:ident, $atomic:ident, $align_mask:expr, $xchg:ident, $xadd:ident, $cmpxchg:ident, $add:ident, $sub:ident) => {
        #[inline(always)]
        pub unsafe fn $xchg(addr: u64, val: $bits) -> $bits {
            use core::sync::atomic::{fence, $atomic, Ordering};
            let p = addr as usize;
            if (p & $align_mask) == 0 {
                let old = (*(p as *const $atomic)).swap(val, Ordering::SeqCst);
                fence(Ordering::SeqCst);
                old
            } else {
                let _g = LIFTED_UNALIGNED_ATOMIC_LOCK.lock().unwrap_or_else(|e| e.into_inner());
                let old = core::ptr::read_unaligned(p as *const $bits);
                core::ptr::write_unaligned(p as *mut $bits, val);
                fence(Ordering::SeqCst);
                old
            }
        }

        #[inline(always)]
        pub unsafe fn $xadd(addr: u64, val: $bits) -> $bits {
            use core::sync::atomic::{fence, $atomic, Ordering};
            let p = addr as usize;
            if (p & $align_mask) == 0 {
                let old = (*(p as *const $atomic)).fetch_add(val, Ordering::SeqCst);
                fence(Ordering::SeqCst);
                old
            } else {
                let _g = LIFTED_UNALIGNED_ATOMIC_LOCK.lock().unwrap_or_else(|e| e.into_inner());
                let old = core::ptr::read_unaligned(p as *const $bits);
                core::ptr::write_unaligned(p as *mut $bits, old.wrapping_add(val));
                fence(Ordering::SeqCst);
                old
            }
        }

        #[inline(always)]
        pub unsafe fn $cmpxchg(addr: u64, expected: $bits, desired: $bits) -> (bool, $bits) {
            use core::sync::atomic::{fence, $atomic, Ordering};
            let p = addr as usize;
            if (p & $align_mask) == 0 {
                let res = match (*(p as *const $atomic)).compare_exchange(
                    expected,
                    desired,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                ) {
                    Ok(v) => (true, v),
                    Err(v) => (false, v),
                };
                fence(Ordering::SeqCst);
                res
            } else {
                let _g = LIFTED_UNALIGNED_ATOMIC_LOCK.lock().unwrap_or_else(|e| e.into_inner());
                let old = core::ptr::read_unaligned(p as *const $bits);
                let ok = old == expected;
                if ok {
                    core::ptr::write_unaligned(p as *mut $bits, desired);
                }
                fence(Ordering::SeqCst);
                (ok, old)
            }
        }

        #[inline(always)]
        pub unsafe fn $add(addr: u64, val: $bits) -> $bits {
            $xadd(addr, val).wrapping_add(val)
        }

        #[inline(always)]
        pub unsafe fn $sub(addr: u64, val: $bits) -> ($bits, $bits) {
            let old = $xadd(addr, (0 as $bits).wrapping_sub(val));
            (old, old.wrapping_sub(val))
        }
    };
}

impl_lifted_atomics!(u8, AtomicU8, 0usize, atomic_xchg_u8, atomic_xadd_u8, atomic_cmpxchg_u8, atomic_add_u8, atomic_sub_u8);
impl_lifted_atomics!(u16, AtomicU16, 1usize, atomic_xchg_u16, atomic_xadd_u16, atomic_cmpxchg_u16, atomic_add_u16, atomic_sub_u16);
impl_lifted_atomics!(u32, AtomicU32, 3usize, atomic_xchg_u32, atomic_xadd_u32, atomic_cmpxchg_u32, atomic_add_u32, atomic_sub_u32);
impl_lifted_atomics!(u64, AtomicU64, 7usize, atomic_xchg_u64, atomic_xadd_u64, atomic_cmpxchg_u64, atomic_add_u64, atomic_sub_u64);

#[inline(always)]
pub unsafe fn rd_acq_u32(addr: usize) -> u32 {
    if (addr & 3) == 0 {
        (*(addr as *const core::sync::atomic::AtomicU32)).load(core::sync::atomic::Ordering::Acquire)
    } else {
        let v = core::ptr::read_unaligned(addr as *const u32);
        core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
        v
    }
}

#[inline(always)]
pub unsafe fn rd_acq_u64(addr: usize) -> u64 {
    if (addr & 7) == 0 {
        (*(addr as *const core::sync::atomic::AtomicU64)).load(core::sync::atomic::Ordering::Acquire)
    } else {
        let v = core::ptr::read_unaligned(addr as *const u64);
        core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
        v
    }
}

#[inline(always)]
pub unsafe fn rd_acq_usize(addr: usize) -> usize {
    rd_acq_u64(addr) as usize
}

#[inline(always)]
pub unsafe fn wr_rel_u64(addr: usize, val: u64) {
    if (addr & 7) == 0 {
        (*(addr as *const core::sync::atomic::AtomicU64)).store(val, core::sync::atomic::Ordering::Release);
    } else {
        core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
        core::ptr::write_unaligned(addr as *mut u64, val);
    }
}

/// Returns `true` if `ptr` is an `aot_va` interpreter-fallback stub rather than a compiled
/// native Rust replacement. On `aarch64`, skipping these when populating `VA_DISPATCH` lets
/// `emu_run_full` and `emu_jit` execute guest-to-guest calls/jumps directly in-place without
/// exiting to host and reallocating `Cpu` on every call.
#[inline]
pub fn is_aot_va_stub(ptr: *const ()) -> bool {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        if ptr.is_null() {
            return false;
        }
        let w0 = core::ptr::read_unaligned(ptr as *const u32);
        w0 == 0xd294_0f50 || w0 == 0x0000_a07a
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let _ = ptr;
        false
    }
}
