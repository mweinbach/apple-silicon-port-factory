//! Unified 128-bit/256-bit x86-64 SIMD execution kernels (ARM64 NEON + portable fallbacks),
//! crypto/F16C/CRC32C/PCMPSTR helpers, and O(1) XMM/YMM register indexing shared across
//! [`super::fast`], [`super::simd_mov`], and [`super::simd_fp`].

use super::Cpu;
#[cfg(not(target_arch = "aarch64"))]
use super::{
    sse_arith32, sse_arith64, sse_cmp_pred, sse_cvt_i32, sse_sqrt32, sse_sqrt64, x86_nan32,
    x86_nan64,
};
use iced_x86::{Instruction, OpKind, Register};

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

// Compile-time assertions guaranteeing O(1) arithmetic indexing for XMM0..=15 and YMM0..=15.
const _: () = {
    const X0: u32 = Register::XMM0 as u32;
    const Y0: u32 = Register::YMM0 as u32;
    assert!(Register::XMM1 as u32 == X0 + 1);
    assert!(Register::XMM2 as u32 == X0 + 2);
    assert!(Register::XMM3 as u32 == X0 + 3);
    assert!(Register::XMM4 as u32 == X0 + 4);
    assert!(Register::XMM5 as u32 == X0 + 5);
    assert!(Register::XMM6 as u32 == X0 + 6);
    assert!(Register::XMM7 as u32 == X0 + 7);
    assert!(Register::XMM8 as u32 == X0 + 8);
    assert!(Register::XMM9 as u32 == X0 + 9);
    assert!(Register::XMM10 as u32 == X0 + 10);
    assert!(Register::XMM11 as u32 == X0 + 11);
    assert!(Register::XMM12 as u32 == X0 + 12);
    assert!(Register::XMM13 as u32 == X0 + 13);
    assert!(Register::XMM14 as u32 == X0 + 14);
    assert!(Register::XMM15 as u32 == X0 + 15);
    assert!(Register::YMM1 as u32 == Y0 + 1);
    assert!(Register::YMM2 as u32 == Y0 + 2);
    assert!(Register::YMM3 as u32 == Y0 + 3);
    assert!(Register::YMM4 as u32 == Y0 + 4);
    assert!(Register::YMM5 as u32 == Y0 + 5);
    assert!(Register::YMM6 as u32 == Y0 + 6);
    assert!(Register::YMM7 as u32 == Y0 + 7);
    assert!(Register::YMM8 as u32 == Y0 + 8);
    assert!(Register::YMM9 as u32 == Y0 + 9);
    assert!(Register::YMM10 as u32 == Y0 + 10);
    assert!(Register::YMM11 as u32 == Y0 + 11);
    assert!(Register::YMM12 as u32 == Y0 + 12);
    assert!(Register::YMM13 as u32 == Y0 + 13);
    assert!(Register::YMM14 as u32 == Y0 + 14);
    assert!(Register::YMM15 as u32 == Y0 + 15);
};

/// O(1) XMM/YMM register index (`0..16`).
#[inline(always)]
pub(in crate::emu) fn xmm_idx_fast(r: Register) -> Option<usize> {
    let rv = r as u32;
    let xi = rv.wrapping_sub(Register::XMM0 as u32);
    if xi < 16 {
        return Some(xi as usize);
    }
    let yi = rv.wrapping_sub(Register::YMM0 as u32);
    if yi < 16 {
        return Some(yi as usize);
    }
    None
}

/// O(1) check whether `r` is `YMM0..=YMM15`.
#[inline(always)]
pub(in crate::emu) fn is_ymm_fast(r: Register) -> bool {
    (r as u32).wrapping_sub(Register::YMM0 as u32) < 16
}

/// O(1) combined `(index, is_ymm)` decode for XMM/YMM registers.
#[inline(always)]
pub(in crate::emu) fn xmm_reg_info(r: Register) -> Option<(usize, bool)> {
    let rv = r as u32;
    let xi = rv.wrapping_sub(Register::XMM0 as u32);
    if xi < 16 {
        return Some((xi as usize, false));
    }
    let yi = rv.wrapping_sub(Register::YMM0 as u32);
    if yi < 16 {
        return Some((yi as usize, true));
    }
    None
}

/// Fast register vector load: skips reading `ymm_hi` on 128-bit ops.
#[inline(always)]
pub(in crate::emu) fn load_vec_reg(cpu: &Cpu, r: Register, ymm: bool) -> (u128, u128) {
    match xmm_idx_fast(r) {
        Some(s) => (cpu.xmm[s], if ymm { cpu.ymm_hi[s] } else { 0 }),
        None => (0, 0),
    }
}

/// Fast 128/256-bit full-vector operand load (register or `m128`/`m256` memory).
#[inline(always)]
pub(in crate::emu) unsafe fn load_vec_op(cpu: &Cpu, ins: &Instruction, op: u32, ymm: bool) -> (u128, u128) {
    match ins.op_kind(op) {
        OpKind::Register => load_vec_reg(cpu, ins.op_register(op), ymm),
        OpKind::Memory => {
            let addr = cpu.eff_addr(ins);
            let lo = cpu.read_u128(addr);
            let hi = if ymm { cpu.read_u128(addr.wrapping_add(16)) } else { 0 };
            (lo, hi)
        }
        _ => (0, 0),
    }
}

/// Fast 128/256-bit destination write preserving exact VEX.128 upper-half zeroing semantics.
#[inline(always)]
pub(in crate::emu) fn store_vec_dst(cpu: &mut Cpu, ins: &Instruction, d: usize, ymm: bool, lo: u128, hi: u128) {
    cpu.xmm[d] = lo;
    if ymm {
        cpu.set_ymm_hi(d, hi);
    } else if Cpu::is_vex(ins) {
        cpu.clear_ymm_hi_reg(d);
    }
}

// ---- NEON transmute helpers (little-endian ARM64: lane 0 = low bits of u128, matching x86) ----

#[cfg(target_arch = "aarch64")]
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
fn qb(v: u128) -> uint8x16_t {
    unsafe { core::mem::transmute(v) }
}
#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn bq(v: uint8x16_t) -> u128 {
    unsafe { core::mem::transmute(v) }
}
#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn qsb(v: u128) -> int8x16_t {
    unsafe { core::mem::transmute(v) }
}
#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn sbq(v: int8x16_t) -> u128 {
    unsafe { core::mem::transmute(v) }
}
#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn qw(v: u128) -> uint16x8_t {
    unsafe { core::mem::transmute(v) }
}
#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn wq(v: uint16x8_t) -> u128 {
    unsafe { core::mem::transmute(v) }
}
#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn qsw(v: u128) -> int16x8_t {
    unsafe { core::mem::transmute(v) }
}
#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn swq(v: int16x8_t) -> u128 {
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
fn qsl(v: u128) -> int64x2_t {
    unsafe { core::mem::transmute(v) }
}

// ---- Exact x86 NaN fixup (fast non-NaN path + cold branchless NEON slow path) ----

#[cfg(target_arch = "aarch64")]
#[cold]
#[inline(never)]
unsafe fn neon_x86_nan_f32_slow(
    a: float32x4_t,
    b: float32x4_t,
    r: float32x4_t,
    r_not_nan: uint32x4_t,
) -> float32x4_t {
    let qbit = vdupq_n_u32(0x0040_0000);
    let indef = vdupq_n_u32(0xFFC0_0000);
    let a_not_nan = vceqq_f32(a, a);
    let b_not_nan = vceqq_f32(b, b);
    let aq = vorrq_u32(vreinterpretq_u32_f32(a), qbit);
    let bq = vorrq_u32(vreinterpretq_u32_f32(b), qbit);
    let nan_b_or_indef = vbslq_u32(b_not_nan, indef, bq);
    let nan_res = vbslq_u32(a_not_nan, nan_b_or_indef, aq);
    vreinterpretq_f32_u32(vbslq_u32(r_not_nan, vreinterpretq_u32_f32(r), nan_res))
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn neon_x86_nan_f32(a: float32x4_t, b: float32x4_t, r: float32x4_t) -> float32x4_t {
    let not_nan = vceqq_f32(r, r);
    if vminvq_u32(not_nan) != 0 {
        return r;
    }
    neon_x86_nan_f32_slow(a, b, r, not_nan)
}

#[cfg(target_arch = "aarch64")]
#[cold]
#[inline(never)]
unsafe fn neon_x86_nan_f32_unary_slow(
    a: float32x4_t,
    r: float32x4_t,
    r_not_nan: uint32x4_t,
) -> float32x4_t {
    let qbit = vdupq_n_u32(0x0040_0000);
    let indef = vdupq_n_u32(0xFFC0_0000);
    let a_not_nan = vceqq_f32(a, a);
    let aq = vorrq_u32(vreinterpretq_u32_f32(a), qbit);
    let nan_res = vbslq_u32(a_not_nan, indef, aq);
    vreinterpretq_f32_u32(vbslq_u32(r_not_nan, vreinterpretq_u32_f32(r), nan_res))
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn neon_x86_nan_f32_unary(a: float32x4_t, r: float32x4_t) -> float32x4_t {
    let not_nan = vceqq_f32(r, r);
    if vminvq_u32(not_nan) != 0 {
        return r;
    }
    neon_x86_nan_f32_unary_slow(a, r, not_nan)
}

#[cfg(target_arch = "aarch64")]
#[cold]
#[inline(never)]
unsafe fn neon_x86_nan_f64_slow(
    a: float64x2_t,
    b: float64x2_t,
    r: float64x2_t,
    r_not_nan: uint64x2_t,
) -> float64x2_t {
    let qbit = vdupq_n_u64(0x0008_0000_0000_0000);
    let indef = vdupq_n_u64(0xFFF8_0000_0000_0000);
    let a_not_nan = vceqq_f64(a, a);
    let b_not_nan = vceqq_f64(b, b);
    let aq = vorrq_u64(vreinterpretq_u64_f64(a), qbit);
    let bq = vorrq_u64(vreinterpretq_u64_f64(b), qbit);
    let nan_b_or_indef = vbslq_u64(b_not_nan, indef, bq);
    let nan_res = vbslq_u64(a_not_nan, nan_b_or_indef, aq);
    vreinterpretq_f64_u64(vbslq_u64(r_not_nan, vreinterpretq_u64_f64(r), nan_res))
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn neon_x86_nan_f64(a: float64x2_t, b: float64x2_t, r: float64x2_t) -> float64x2_t {
    let not_nan = vceqq_f64(r, r);
    if vminvq_u32(vreinterpretq_u32_u64(not_nan)) != 0 {
        return r;
    }
    neon_x86_nan_f64_slow(a, b, r, not_nan)
}

#[cfg(target_arch = "aarch64")]
#[cold]
#[inline(never)]
unsafe fn neon_x86_nan_f64_unary_slow(
    a: float64x2_t,
    r: float64x2_t,
    r_not_nan: uint64x2_t,
) -> float64x2_t {
    let qbit = vdupq_n_u64(0x0008_0000_0000_0000);
    let indef = vdupq_n_u64(0xFFF8_0000_0000_0000);
    let a_not_nan = vceqq_f64(a, a);
    let aq = vorrq_u64(vreinterpretq_u64_f64(a), qbit);
    let nan_res = vbslq_u64(a_not_nan, indef, aq);
    vreinterpretq_f64_u64(vbslq_u64(r_not_nan, vreinterpretq_u64_f64(r), nan_res))
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn neon_x86_nan_f64_unary(a: float64x2_t, r: float64x2_t) -> float64x2_t {
    let not_nan = vceqq_f64(r, r);
    if vminvq_u32(vreinterpretq_u32_u64(not_nan)) != 0 {
        return r;
    }
    neon_x86_nan_f64_unary_slow(a, r, not_nan)
}

// ---- Packed FP arithmetic (`add/sub/mul/div/min/max/sqrt/rcp/rsqrt/addsub/hadd/hsub/fma/dp/cmp/round`) ----

/// `op`: 0=add, 1=sub, 2=mul, 3=div, 4=min (`a < b ? a : b`), 5=max (`a > b ? a : b`).
#[inline(always)]
pub(in crate::emu) fn fp_bin_ps(op: u8, a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let (va, vb) = (qf(a), qf(b));
        let res = match op {
            0 => neon_x86_nan_f32(va, vb, vaddq_f32(va, vb)),
            1 => neon_x86_nan_f32(va, vb, vsubq_f32(va, vb)),
            2 => neon_x86_nan_f32(va, vb, vmulq_f32(va, vb)),
            3 => neon_x86_nan_f32(va, vb, vdivq_f32(va, vb)),
            4 => vbslq_f32(vcltq_f32(va, vb), va, vb),
            _ => vbslq_f32(vcgtq_f32(va, vb), va, vb),
        };
        fq(res)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..4 {
            let fa = f32::from_bits((a >> (lane * 32)) as u32);
            let fb = f32::from_bits((b >> (lane * 32)) as u32);
            out |= (sse_arith32(op, fa, fb).to_bits() as u128) << (lane * 32);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn fp_bin_pd(op: u8, a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let (va, vb) = (qd(a), qd(b));
        let res = match op {
            0 => neon_x86_nan_f64(va, vb, vaddq_f64(va, vb)),
            1 => neon_x86_nan_f64(va, vb, vsubq_f64(va, vb)),
            2 => neon_x86_nan_f64(va, vb, vmulq_f64(va, vb)),
            3 => neon_x86_nan_f64(va, vb, vdivq_f64(va, vb)),
            4 => vbslq_f64(vcltq_f64(va, vb), va, vb),
            _ => vbslq_f64(vcgtq_f64(va, vb), va, vb),
        };
        dq(res)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..2 {
            let fa = f64::from_bits((a >> (lane * 64)) as u64);
            let fb = f64::from_bits((b >> (lane * 64)) as u64);
            out |= (sse_arith64(op, fa, fb).to_bits() as u128) << (lane * 64);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn sqrt_ps(a: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let va = qf(a);
        fq(neon_x86_nan_f32_unary(va, vsqrtq_f32(va)))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..4 {
            let f = f32::from_bits((a >> (lane * 32)) as u32);
            out |= (sse_sqrt32(f).to_bits() as u128) << (lane * 32);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn sqrt_pd(a: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let va = qd(a);
        dq(neon_x86_nan_f64_unary(va, vsqrtq_f64(va)))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..2 {
            let f = f64::from_bits((a >> (lane * 64)) as u64);
            out |= (sse_sqrt64(f).to_bits() as u128) << (lane * 64);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn rcp_rsqrt_ps(a: u128, is_rsqrt: bool) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let va = qf(a);
        let denom = if is_rsqrt { neon_x86_nan_f32_unary(va, vsqrtq_f32(va)) } else { va };
        fq(vdivq_f32(vdupq_n_f32(1.0), denom))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..4 {
            let x = f32::from_bits((a >> (lane * 32)) as u32);
            let r = if is_rsqrt { 1.0f32 / sse_sqrt32(x) } else { 1.0f32 / x };
            out |= (r.to_bits() as u128) << (lane * 32);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn addsub_ps(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let (va, vb) = (qf(a), qf(b));
        let odd: uint32x4_t = core::mem::transmute([0u32, u32::MAX, 0, u32::MAX]);
        let r = vbslq_f32(odd, vaddq_f32(va, vb), vsubq_f32(va, vb));
        fq(neon_x86_nan_f32(va, vb, r))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..4 {
            let fa = f32::from_bits((a >> (lane * 32)) as u32);
            let fb = f32::from_bits((b >> (lane * 32)) as u32);
            let fr = x86_nan32(fa, fb, if lane % 2 == 0 { fa - fb } else { fa + fb });
            out |= (fr.to_bits() as u128) << (lane * 32);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn addsub_pd(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let (va, vb) = (qd(a), qd(b));
        let odd: uint64x2_t = core::mem::transmute([0u64, u64::MAX]);
        let r = vbslq_f64(odd, vaddq_f64(va, vb), vsubq_f64(va, vb));
        dq(neon_x86_nan_f64(va, vb, r))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..2 {
            let fa = f64::from_bits((a >> (lane * 64)) as u64);
            let fb = f64::from_bits((b >> (lane * 64)) as u64);
            let fr = x86_nan64(fa, fb, if lane == 0 { fa - fb } else { fa + fb });
            out |= (fr.to_bits() as u128) << (lane * 64);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn hadd_sub_ps(a: u128, b: u128, is_add: bool) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let (va, vb) = (qf(a), qf(b));
        let lo = vuzp1q_f32(va, vb);
        let hi = vuzp2q_f32(va, vb);
        let r = if is_add { vaddq_f32(lo, hi) } else { vsubq_f32(lo, hi) };
        fq(neon_x86_nan_f32(lo, hi, r))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let f = |x: u128, i: usize| f32::from_bits((x >> (i * 32)) as u32);
        let h = |l: f32, r: f32| x86_nan32(l, r, if is_add { l + r } else { l - r });
        let r0 = h(f(a, 0), f(a, 1));
        let r1 = h(f(a, 2), f(a, 3));
        let r2 = h(f(b, 0), f(b, 1));
        let r3 = h(f(b, 2), f(b, 3));
        (r0.to_bits() as u128)
            | ((r1.to_bits() as u128) << 32)
            | ((r2.to_bits() as u128) << 64)
            | ((r3.to_bits() as u128) << 96)
    }
}

#[inline(always)]
pub(in crate::emu) fn hadd_sub_pd(a: u128, b: u128, is_add: bool) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let (va, vb) = (qd(a), qd(b));
        let lo = vuzp1q_f64(va, vb);
        let hi = vuzp2q_f64(va, vb);
        let r = if is_add { vaddq_f64(lo, hi) } else { vsubq_f64(lo, hi) };
        dq(neon_x86_nan_f64(lo, hi, r))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let f = |x: u128, i: usize| f64::from_bits((x >> (i * 64)) as u64);
        let h = |l: f64, r: f64| x86_nan64(l, r, if is_add { l + r } else { l - r });
        let r0 = h(f(a, 0), f(a, 1));
        let r1 = h(f(b, 0), f(b, 1));
        (r0.to_bits() as u128) | ((r1.to_bits() as u128) << 64)
    }
}

/// Packed `f32x4` FMA: `acc_mode`: 0 = `+acc`, 1 = `-acc`, 2 = `fmaddsub` (neg even), 3 = `fmsubadd` (neg odd).
#[inline(always)]
pub(in crate::emu) fn fma_ps(m1: u128, m2: u128, acc: u128, neg_mul: bool, acc_mode: u8) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let lhs = if neg_mul { vnegq_f32(qf(m1)) } else { qf(m1) };
        let rhs = match acc_mode {
            1 => vnegq_f32(qf(acc)),
            2 => {
                let s: uint32x4_t = core::mem::transmute([0x8000_0000u32, 0, 0x8000_0000, 0]);
                vreinterpretq_f32_u32(veorq_u32(qu(acc), s))
            }
            3 => {
                let s: uint32x4_t = core::mem::transmute([0u32, 0x8000_0000, 0, 0x8000_0000]);
                vreinterpretq_f32_u32(veorq_u32(qu(acc), s))
            }
            _ => qf(acc),
        };
        // Pin operand order (`Vd = rhs, Vn = lhs, Vm = m2`) via inline `fmla` so LLVM cannot
        // commute `(lhs, m2)` and change ARM64 dual-NaN propagation order relative to JIT/fmadd.
        let mut out = rhs;
        let vm2 = qf(m2);
        core::arch::asm!(
            "fmla {out:v}.4s, {lhs:v}.4s, {m2:v}.4s",
            out = inout(vreg) out,
            lhs = in(vreg) lhs,
            m2 = in(vreg) vm2,
            options(pure, nomem, nostack),
        );
        fq(out)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..4 {
            let f1 = f32::from_bits((m1 >> (lane * 32)) as u32);
            let f2 = f32::from_bits((m2 >> (lane * 32)) as u32);
            let fa = f32::from_bits((acc >> (lane * 32)) as u32);
            let neg_acc = match acc_mode {
                1 => true,
                2 => (lane & 1) == 0,
                3 => (lane & 1) == 1,
                _ => false,
            };
            let lhs = if neg_mul { -f1 } else { f1 };
            let rhs = if neg_acc { -fa } else { fa };
            out |= (lhs.mul_add(f2, rhs).to_bits() as u128) << (lane * 32);
        }
        out
    }
}

/// Packed `f64x2` FMA: `acc_mode`: 0 = `+acc`, 1 = `-acc`, 2 = `fmaddsub` (neg lane 0), 3 = `fmsubadd` (neg lane 1).
#[inline(always)]
pub(in crate::emu) fn fma_pd(m1: u128, m2: u128, acc: u128, neg_mul: bool, acc_mode: u8) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let lhs = if neg_mul { vnegq_f64(qd(m1)) } else { qd(m1) };
        let rhs = match acc_mode {
            1 => vnegq_f64(qd(acc)),
            2 => {
                let s: uint64x2_t = core::mem::transmute([0x8000_0000_0000_0000u64, 0]);
                vreinterpretq_f64_u64(veorq_u64(ql(acc), s))
            }
            3 => {
                let s: uint64x2_t = core::mem::transmute([0u64, 0x8000_0000_0000_0000]);
                vreinterpretq_f64_u64(veorq_u64(ql(acc), s))
            }
            _ => qd(acc),
        };
        // Pin operand order (`Vd = rhs, Vn = lhs, Vm = m2`) via inline `fmla` so LLVM cannot
        // commute `(lhs, m2)` and change ARM64 dual-NaN propagation order relative to JIT/fmadd.
        let mut out = rhs;
        let vm2 = qd(m2);
        core::arch::asm!(
            "fmla {out:v}.2d, {lhs:v}.2d, {m2:v}.2d",
            out = inout(vreg) out,
            lhs = in(vreg) lhs,
            m2 = in(vreg) vm2,
            options(pure, nomem, nostack),
        );
        dq(out)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..2 {
            let f1 = f64::from_bits((m1 >> (lane * 64)) as u64);
            let f2 = f64::from_bits((m2 >> (lane * 64)) as u64);
            let fa = f64::from_bits((acc >> (lane * 64)) as u64);
            let neg_acc = match acc_mode {
                1 => true,
                2 => lane == 0,
                3 => lane == 1,
                _ => false,
            };
            let lhs = if neg_mul { -f1 } else { f1 };
            let rhs = if neg_acc { -fa } else { fa };
            out |= (lhs.mul_add(f2, rhs).to_bits() as u128) << (lane * 64);
        }
        out
    }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn lane_mask4(bits: u8) -> uint32x4_t {
    let bit_sel: uint32x4_t = core::mem::transmute([1u32, 2, 4, 8]);
    vtstq_u32(vdupq_n_u32(bits as u32), bit_sel)
}

#[inline(always)]
pub(in crate::emu) fn dp_ps(a: u128, b: u128, imm: u8) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let p = vmulq_f32(qf(a), qf(b));
        let p = vreinterpretq_f32_u32(vandq_u32(vreinterpretq_u32_f32(p), lane_mask4(imm >> 4)));
        let s = vpaddq_f32(p, p);
        let sum = vgetq_lane_f32::<0>(s) + vgetq_lane_f32::<1>(s);
        uq(vandq_u32(vreinterpretq_u32_f32(vdupq_n_f32(sum)), lane_mask4(imm & 0x0F)))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let lane = |v: u128, i: u32| f32::from_bits((v >> (i * 32)) as u32);
        let p = |i: u32| if (imm >> (4 + i)) & 1 != 0 { lane(a, i) * lane(b, i) } else { 0.0 };
        let sum = (p(0) + p(1)) + (p(2) + p(3));
        let mut out = 0u128;
        for i in 0..4 {
            if (imm >> i) & 1 != 0 {
                out |= (sum.to_bits() as u128) << (i * 32);
            }
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn cmp_ps(a: u128, b: u128, imm: u8) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let (x, y) = (qf(a), qf(b));
        let ord = || vandq_u32(vceqq_f32(x, x), vceqq_f32(y, y));
        uq(match imm & 0x0F {
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
            0x0A => vmvnq_u32(vcgtq_f32(x, y)),
            0x0B => vdupq_n_u32(0),
            0x0C => vbicq_u32(ord(), vceqq_f32(x, y)),
            0x0D => vcgeq_f32(x, y),
            0x0E => vcgtq_f32(x, y),
            _ => vdupq_n_u32(u32::MAX),
        })
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..4 {
            let fa = f32::from_bits((a >> (lane * 32)) as u32) as f64;
            let fb = f32::from_bits((b >> (lane * 32)) as u32) as f64;
            if sse_cmp_pred(fa, fb, imm) {
                out |= 0xFFFF_FFFFu128 << (lane * 32);
            }
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn cmp_pd(a: u128, b: u128, imm: u8) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let (x, y) = (qd(a), qd(b));
        let not64 = |v: uint64x2_t| vreinterpretq_u64_u32(vmvnq_u32(vreinterpretq_u32_u64(v)));
        let ord = || vandq_u64(vceqq_f64(x, x), vceqq_f64(y, y));
        lq(match imm & 0x0F {
            0x00 => vceqq_f64(x, y),
            0x01 => vcltq_f64(x, y),
            0x02 => vcleq_f64(x, y),
            0x03 => not64(ord()),
            0x04 => not64(vceqq_f64(x, y)),
            0x05 => not64(vcltq_f64(x, y)),
            0x06 => not64(vcleq_f64(x, y)),
            0x07 => ord(),
            0x08 => vornq_u64(vceqq_f64(x, y), ord()),
            0x09 => not64(vcgeq_f64(x, y)),
            0x0A => not64(vcgtq_f64(x, y)),
            0x0B => vdupq_n_u64(0),
            0x0C => vbicq_u64(ord(), vceqq_f64(x, y)),
            0x0D => vcgeq_f64(x, y),
            0x0E => vcgtq_f64(x, y),
            _ => vdupq_n_u64(u64::MAX),
        })
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..2 {
            let fa = f64::from_bits((a >> (lane * 64)) as u64);
            let fb = f64::from_bits((b >> (lane * 64)) as u64);
            if sse_cmp_pred(fa, fb, imm) {
                out |= 0xFFFF_FFFF_FFFF_FFFFu128 << (lane * 64);
            }
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn round_f32(f: f32, imm: u8) -> f32 {
    match imm & 3 {
        0 => f.round_ties_even(),
        1 => f.floor(),
        2 => f.ceil(),
        _ => f.trunc(),
    }
}

#[inline(always)]
pub(in crate::emu) fn round_f64(f: f64, imm: u8) -> f64 {
    match imm & 3 {
        0 => f.round_ties_even(),
        1 => f.floor(),
        2 => f.ceil(),
        _ => f.trunc(),
    }
}

#[inline(always)]
pub(in crate::emu) fn round_ps(a: u128, imm: u8) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let x = qf(a);
        fq(match imm & 3 {
            0 => vrndnq_f32(x),
            1 => vrndmq_f32(x),
            2 => vrndpq_f32(x),
            _ => vrndq_f32(x),
        })
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..4 {
            let f = f32::from_bits((a >> (lane * 32)) as u32);
            out |= (round_f32(f, imm).to_bits() as u128) << (lane * 32);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn round_pd(a: u128, imm: u8) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let x = qd(a);
        dq(match imm & 3 {
            0 => vrndnq_f64(x),
            1 => vrndmq_f64(x),
            2 => vrndpq_f64(x),
            _ => vrndq_f64(x),
        })
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..2 {
            let f = f64::from_bits((a >> (lane * 64)) as u64);
            out |= (round_f64(f, imm).to_bits() as u128) << (lane * 64);
        }
        out
    }
}

// ---- Vector conversions (exact x86 integer indefinite `0x8000_0000`) ----

#[inline(always)]
pub(in crate::emu) fn cvtdq2ps(a: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        fq(vcvtq_f32_s32(qs(a)))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..4 {
            let i = (a >> (lane * 32)) as u32 as i32;
            out |= ((i as f32).to_bits() as u128) << (lane * 32);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn cvtps2dq(a: u128, trunc: bool) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let r = if trunc { qf(a) } else { vrndnq_f32(qf(a)) };
        let lo = vcgeq_f32(r, vdupq_n_f32(-2147483648.0));
        let hi = vcltq_f32(r, vdupq_n_f32(2147483648.0));
        let ok = vandq_u32(lo, hi);
        let conv = vreinterpretq_u32_s32(vcvtq_s32_f32(r));
        uq(vbslq_u32(ok, conv, vdupq_n_u32(0x8000_0000)))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..4 {
            let f = f32::from_bits((a >> (lane * 32)) as u32);
            let r = if trunc { f } else { f.round_ties_even() };
            let i: i32 = if r.is_nan() || r >= 2147483648.0 || r < -2147483648.0 {
                i32::MIN
            } else {
                r as i32
            };
            out |= ((i as u32) as u128) << (lane * 32);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn cvtps2pd(raw: u128, ymm: bool) -> (u128, u128) {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let vf = qf(raw);
        let lo = dq(vcvt_f64_f32(vget_low_f32(vf)));
        let hi = if ymm { dq(vcvt_f64_f32(vget_high_f32(vf))) } else { 0 };
        (lo, hi)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let cvt = |lane: u32| (f32::from_bits((raw >> (lane * 32)) as u32) as f64).to_bits() as u128;
        let lo = cvt(0) | (cvt(1) << 64);
        let hi = if ymm { cvt(2) | (cvt(3) << 64) } else { 0 };
        (lo, hi)
    }
}

#[inline(always)]
pub(in crate::emu) fn cvtdq2pd(raw: u128, ymm: bool) -> (u128, u128) {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let vi = qs(raw);
        let lo = dq(vcvtq_f64_s64(vmovl_s32(vget_low_s32(vi))));
        let hi = if ymm { dq(vcvtq_f64_s64(vmovl_high_s32(vi))) } else { 0 };
        (lo, hi)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let cvt = |lane: u32| (((raw >> (lane * 32)) as u32 as i32) as f64).to_bits() as u128;
        let lo = cvt(0) | (cvt(1) << 64);
        let hi = if ymm { cvt(2) | (cvt(3) << 64) } else { 0 };
        (lo, hi)
    }
}

#[inline(always)]
pub(in crate::emu) fn cvtpd2ps(lo: u128, hi: u128, src_256: bool) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let l = vcvt_f32_f64(qd(lo));
        let h = if src_256 { vcvt_f32_f64(qd(hi)) } else { vdup_n_f32(0.0) };
        fq(vcombine_f32(l, h))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let cvt = |bits: u64| (f64::from_bits(bits) as f32).to_bits() as u128;
        let i0 = cvt(lo as u64);
        let i1 = cvt((lo >> 64) as u64);
        let i2 = if src_256 { cvt(hi as u64) } else { 0 };
        let i3 = if src_256 { cvt((hi >> 64) as u64) } else { 0 };
        i0 | (i1 << 32) | (i2 << 64) | (i3 << 96)
    }
}

#[inline(always)]
pub(in crate::emu) fn cvtpd2dq(lo: u128, hi: u128, src_256: bool, trunc: bool) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let cvt2 = |v: u128| -> uint32x2_t {
            let r = if trunc { vrndq_f64(qd(v)) } else { vrndnq_f64(qd(v)) };
            let ok = vandq_u64(
                vcgeq_f64(r, vdupq_n_f64(-2147483648.0)),
                vcltq_f64(r, vdupq_n_f64(2147483648.0)),
            );
            let iv = vreinterpret_u32_s32(vmovn_s64(vcvtq_s64_f64(r)));
            vbsl_u32(vmovn_u64(ok), iv, vdup_n_u32(0x8000_0000))
        };
        let l = cvt2(lo);
        let h = if src_256 { cvt2(hi) } else { vdup_n_u32(0) };
        uq(vcombine_u32(l, h))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let cvt = |bits: u64| sse_cvt_i32(f64::from_bits(bits), trunc) as u128;
        let i0 = cvt(lo as u64);
        let i1 = cvt((lo >> 64) as u64);
        let i2 = if src_256 { cvt(hi as u64) } else { 0 };
        let i3 = if src_256 { cvt((hi >> 64) as u64) } else { 0 };
        i0 | (i1 << 32) | (i2 << 64) | (i3 << 96)
    }
}

// ---- Bitwise ----

#[inline(always)]
pub(in crate::emu) fn v_and(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { lq(vandq_u64(ql(a), ql(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    { a & b }
}

#[inline(always)]
pub(in crate::emu) fn v_or(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { lq(vorrq_u64(ql(a), ql(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    { a | b }
}

#[inline(always)]
pub(in crate::emu) fn v_andn(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { lq(vbicq_u64(ql(b), ql(a))) }
    #[cfg(not(target_arch = "aarch64"))]
    { (!a) & b }
}

// ---- Packed integer wrapping add/sub ----

#[inline(always)]
pub(in crate::emu) fn paddb(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { bq(vaddq_u8(qb(a), qb(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..16 {
            let r = ((a >> (i * 8)) as u8).wrapping_add((b >> (i * 8)) as u8);
            out |= (r as u128) << (i * 8);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn psubb(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { bq(vsubq_u8(qb(a), qb(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..16 {
            let r = ((a >> (i * 8)) as u8).wrapping_sub((b >> (i * 8)) as u8);
            out |= (r as u128) << (i * 8);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn paddw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { wq(vaddq_u16(qw(a), qw(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let r = ((a >> (i * 16)) as u16).wrapping_add((b >> (i * 16)) as u16);
            out |= (r as u128) << (i * 16);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn psubw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { wq(vsubq_u16(qw(a), qw(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let r = ((a >> (i * 16)) as u16).wrapping_sub((b >> (i * 16)) as u16);
            out |= (r as u128) << (i * 16);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn paddd(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { uq(vaddq_u32(qu(a), qu(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..4 {
            let r = ((a >> (i * 32)) as u32).wrapping_add((b >> (i * 32)) as u32);
            out |= (r as u128) << (i * 32);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn psubd(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { uq(vsubq_u32(qu(a), qu(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..4 {
            let r = ((a >> (i * 32)) as u32).wrapping_sub((b >> (i * 32)) as u32);
            out |= (r as u128) << (i * 32);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn paddq(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { lq(vaddq_u64(ql(a), ql(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let l0 = (a as u64).wrapping_add(b as u64);
        let l1 = ((a >> 64) as u64).wrapping_add((b >> 64) as u64);
        (l0 as u128) | ((l1 as u128) << 64)
    }
}

#[inline(always)]
pub(in crate::emu) fn psubq(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { lq(vsubq_u64(ql(a), ql(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let l0 = (a as u64).wrapping_sub(b as u64);
        let l1 = ((a >> 64) as u64).wrapping_sub((b >> 64) as u64);
        (l0 as u128) | ((l1 as u128) << 64)
    }
}

// ---- Saturating integer add/sub & rounding average ----

#[inline(always)]
pub(in crate::emu) fn paddsb(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { sbq(vqaddq_s8(qsb(a), qsb(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..16 {
            let r = ((a >> (i * 8)) as u8 as i8).saturating_add((b >> (i * 8)) as u8 as i8);
            out |= (r as u8 as u128) << (i * 8);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn psubsb(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { sbq(vqsubq_s8(qsb(a), qsb(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..16 {
            let r = ((a >> (i * 8)) as u8 as i8).saturating_sub((b >> (i * 8)) as u8 as i8);
            out |= (r as u8 as u128) << (i * 8);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn paddusb(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { bq(vqaddq_u8(qb(a), qb(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..16 {
            let r = ((a >> (i * 8)) as u8).saturating_add((b >> (i * 8)) as u8);
            out |= (r as u128) << (i * 8);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn psubusb(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { bq(vqsubq_u8(qb(a), qb(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..16 {
            let r = ((a >> (i * 8)) as u8).saturating_sub((b >> (i * 8)) as u8);
            out |= (r as u128) << (i * 8);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn paddsw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { swq(vqaddq_s16(qsw(a), qsw(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let r = ((a >> (i * 16)) as u16 as i16).saturating_add((b >> (i * 16)) as u16 as i16);
            out |= (r as u16 as u128) << (i * 16);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn psubsw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { swq(vqsubq_s16(qsw(a), qsw(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let r = ((a >> (i * 16)) as u16 as i16).saturating_sub((b >> (i * 16)) as u16 as i16);
            out |= (r as u16 as u128) << (i * 16);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn paddusw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { wq(vqaddq_u16(qw(a), qw(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let r = ((a >> (i * 16)) as u16).saturating_add((b >> (i * 16)) as u16);
            out |= (r as u128) << (i * 16);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn psubusw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { wq(vqsubq_u16(qw(a), qw(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let r = ((a >> (i * 16)) as u16).saturating_sub((b >> (i * 16)) as u16);
            out |= (r as u128) << (i * 16);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pavgb(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { bq(vrhaddq_u8(qb(a), qb(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..16 {
            let x = ((a >> (i * 8)) & 0xFF) as u16;
            let y = ((b >> (i * 8)) & 0xFF) as u16;
            out |= (((x + y + 1) >> 1) as u8 as u128) << (i * 8);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pavgw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { wq(vrhaddq_u16(qw(a), qw(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let x = ((a >> (i * 16)) & 0xFFFF) as u32;
            let y = ((b >> (i * 16)) & 0xFFFF) as u32;
            out |= (((x + y + 1) >> 1) as u16 as u128) << (i * 16);
        }
        out
    }
}

// ---- Packed integer min/max, abs, sign ----

#[inline(always)]
pub(in crate::emu) fn pminub(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { bq(vminq_u8(qb(a), qb(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..16 {
            let r = ((a >> (i * 8)) as u8).min((b >> (i * 8)) as u8);
            out |= (r as u128) << (i * 8);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pmaxub(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { bq(vmaxq_u8(qb(a), qb(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..16 {
            let r = ((a >> (i * 8)) as u8).max((b >> (i * 8)) as u8);
            out |= (r as u128) << (i * 8);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pminsb(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { sbq(vminq_s8(qsb(a), qsb(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..16 {
            let r = ((a >> (i * 8)) as u8 as i8).min((b >> (i * 8)) as u8 as i8);
            out |= (r as u8 as u128) << (i * 8);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pmaxsb(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { sbq(vmaxq_s8(qsb(a), qsb(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..16 {
            let r = ((a >> (i * 8)) as u8 as i8).max((b >> (i * 8)) as u8 as i8);
            out |= (r as u8 as u128) << (i * 8);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pminuw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { wq(vminq_u16(qw(a), qw(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let r = ((a >> (i * 16)) as u16).min((b >> (i * 16)) as u16);
            out |= (r as u128) << (i * 16);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pmaxuw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { wq(vmaxq_u16(qw(a), qw(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let r = ((a >> (i * 16)) as u16).max((b >> (i * 16)) as u16);
            out |= (r as u128) << (i * 16);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pminsw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { swq(vminq_s16(qsw(a), qsw(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let r = ((a >> (i * 16)) as u16 as i16).min((b >> (i * 16)) as u16 as i16);
            out |= (r as u16 as u128) << (i * 16);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pmaxsw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { swq(vmaxq_s16(qsw(a), qsw(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let r = ((a >> (i * 16)) as u16 as i16).max((b >> (i * 16)) as u16 as i16);
            out |= (r as u16 as u128) << (i * 16);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pminud(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { uq(vminq_u32(qu(a), qu(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..4 {
            let r = ((a >> (i * 32)) as u32).min((b >> (i * 32)) as u32);
            out |= (r as u128) << (i * 32);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pmaxud(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { uq(vmaxq_u32(qu(a), qu(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..4 {
            let r = ((a >> (i * 32)) as u32).max((b >> (i * 32)) as u32);
            out |= (r as u128) << (i * 32);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pminsd(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { sq(vminq_s32(qs(a), qs(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..4 {
            let r = ((a >> (i * 32)) as u32 as i32).min((b >> (i * 32)) as u32 as i32);
            out |= (r as u32 as u128) << (i * 32);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pmaxsd(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { sq(vmaxq_s32(qs(a), qs(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..4 {
            let r = ((a >> (i * 32)) as u32 as i32).max((b >> (i * 32)) as u32 as i32);
            out |= (r as u32 as u128) << (i * 32);
        }
        out
    }
}

/// Non-saturating packed abs (`i8::MIN` wraps to `0x80`, matching x86 `PABSB` and NEON `vabsq_s8`).
#[inline(always)]
pub(in crate::emu) fn pabsb(a: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { sbq(vabsq_s8(qsb(a))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..16 {
            let x = (a >> (i * 8)) as u8 as i8;
            out |= (x.unsigned_abs() as u128) << (i * 8);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pabsw(a: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { swq(vabsq_s16(qsw(a))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let x = (a >> (i * 16)) as u16 as i16;
            out |= (x.unsigned_abs() as u128) << (i * 16);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pabsd(a: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { sq(vabsq_s32(qs(a))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..4 {
            let x = (a >> (i * 32)) as u32 as i32;
            out |= (x.unsigned_abs() as u128) << (i * 32);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn psignb(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let (va, vb) = (qsb(a), qsb(b));
        let neg = vcltzq_s8(vb);
        let nz = vtstq_s8(vb, vb);
        let signed = vbslq_s8(neg, vnegq_s8(va), va);
        bq(vandq_u8(vreinterpretq_u8_s8(signed), nz))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..16 {
            let va = (a >> (i * 8)) as u8 as i8;
            let vb = (b >> (i * 8)) as u8 as i8;
            let r = if vb < 0 { va.wrapping_neg() } else if vb == 0 { 0 } else { va };
            out |= (r as u8 as u128) << (i * 8);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn psignw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let (va, vb) = (qsw(a), qsw(b));
        let neg = vcltzq_s16(vb);
        let nz = vtstq_s16(vb, vb);
        let signed = vbslq_s16(neg, vnegq_s16(va), va);
        wq(vandq_u16(vreinterpretq_u16_s16(signed), nz))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let va = (a >> (i * 16)) as u16 as i16;
            let vb = (b >> (i * 16)) as u16 as i16;
            let r = if vb < 0 { va.wrapping_neg() } else if vb == 0 { 0 } else { va };
            out |= (r as u16 as u128) << (i * 16);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn psignd(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let (va, vb) = (qs(a), qs(b));
        let neg = vcltzq_s32(vb);
        let nz = vtstq_s32(vb, vb);
        let signed = vbslq_s32(neg, vnegq_s32(va), va);
        uq(vandq_u32(vreinterpretq_u32_s32(signed), nz))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..4 {
            let va = (a >> (i * 32)) as u32 as i32;
            let vb = (b >> (i * 32)) as u32 as i32;
            let r = if vb < 0 { va.wrapping_neg() } else if vb == 0 { 0 } else { va };
            out |= (r as u32 as u128) << (i * 32);
        }
        out
    }
}

// ---- Packed integer multiplies, PMADD, PSADBW, PHADD/PHSUB ----

#[inline(always)]
pub(in crate::emu) fn pmulhw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let (va, vb) = (qsw(a), qsw(b));
        let lo = vmull_s16(vget_low_s16(va), vget_low_s16(vb));
        let hi = vmull_high_s16(va, vb);
        swq(vuzp2q_s16(vreinterpretq_s16_s32(lo), vreinterpretq_s16_s32(hi)))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let x = (a >> (i * 16)) as u16 as i16 as i32;
            let y = (b >> (i * 16)) as u16 as i16 as i32;
            out |= (((x * y) >> 16) as i16 as u16 as u128) << (i * 16);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pmulhuw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let (va, vb) = (qw(a), qw(b));
        let lo = vmull_u16(vget_low_u16(va), vget_low_u16(vb));
        let hi = vmull_high_u16(va, vb);
        wq(vuzp2q_u16(vreinterpretq_u16_u32(lo), vreinterpretq_u16_u32(hi)))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let x = (a >> (i * 16)) as u16 as u32;
            let y = (b >> (i * 16)) as u16 as u32;
            out |= (((x * y) >> 16) as u16 as u128) << (i * 16);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pmullw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { wq(vmulq_u16(qw(a), qw(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let r = ((a >> (i * 16)) as u16).wrapping_mul((b >> (i * 16)) as u16);
            out |= (r as u128) << (i * 16);
        }
        out
    }
}

/// Exact x86 `PMULHRSW`: `(((x * y) >> 14) + 1) >> 1` as `i16` (wraps `-32768 * -32768` to `-32768`,
/// unlike ARM `vqrdmulhq_s16` which saturates to `+32767`).
#[inline(always)]
pub(in crate::emu) fn pmulhrsw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let (va, vb) = (qsw(a), qsw(b));
        let lo = vmull_s16(vget_low_s16(va), vget_low_s16(vb));
        let hi = vmull_high_s16(va, vb);
        swq(vcombine_s16(vrshrn_n_s32::<15>(lo), vrshrn_n_s32::<15>(hi)))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let x = (a >> (i * 16)) as u16 as i16 as i32;
            let y = (b >> (i * 16)) as u16 as i16 as i32;
            let r = (((x * y) >> 14) + 1) >> 1;
            out |= (r as i16 as u16 as u128) << (i * 16);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pmulld(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { uq(vmulq_u32(qu(a), qu(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..4 {
            let r = ((a >> (i * 32)) as u32).wrapping_mul((b >> (i * 32)) as u32);
            out |= (r as u128) << (i * 32);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pmuludq(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { lq(vmull_u32(vmovn_u64(ql(a)), vmovn_u64(ql(b)))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let p0 = ((a as u32) as u64).wrapping_mul((b as u32) as u64);
        let p1 = (((a >> 64) as u32) as u64).wrapping_mul(((b >> 64) as u32) as u64);
        (p0 as u128) | ((p1 as u128) << 64)
    }
}

#[inline(always)]
pub(in crate::emu) fn pmuldq(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        core::mem::transmute(vmull_s32(vmovn_s64(qsl(a)), vmovn_s64(qsl(b))))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let p0 = ((a as u32 as i32) as i64).wrapping_mul((b as u32 as i32) as i64) as u64;
        let p1 = (((a >> 64) as u32 as i32) as i64).wrapping_mul(((b >> 64) as u32 as i32) as i64) as u64;
        (p0 as u128) | ((p1 as u128) << 64)
    }
}

#[inline(always)]
pub(in crate::emu) fn pmaddwd(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let (va, vb) = (qsw(a), qsw(b));
        let lo = vmull_s16(vget_low_s16(va), vget_low_s16(vb));
        let hi = vmull_high_s16(va, vb);
        sq(vpaddq_s32(lo, hi))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..4 {
            let a0 = (a >> (i * 32)) as u16 as i16 as i32;
            let a1 = (a >> (i * 32 + 16)) as u16 as i16 as i32;
            let b0 = (b >> (i * 32)) as u16 as i16 as i32;
            let b1 = (b >> (i * 32 + 16)) as u16 as i16 as i32;
            let r = (a0 * b0).wrapping_add(a1 * b1) as u32;
            out |= (r as u128) << (i * 32);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pmaddubsw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let va = qb(a);
        let vb = qsb(b);
        let a_lo = vreinterpretq_s16_u16(vmovl_u8(vget_low_u8(va)));
        let a_hi = vreinterpretq_s16_u16(vmovl_high_u8(va));
        let b_lo = vmovl_s8(vget_low_s8(vb));
        let b_hi = vmovl_high_s8(vb);
        let p_lo = vmulq_s16(a_lo, b_lo);
        let p_hi = vmulq_s16(a_hi, b_hi);
        swq(vqaddq_s16(vuzp1q_s16(p_lo, p_hi), vuzp2q_s16(p_lo, p_hi)))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let a0 = ((a >> (i * 16)) & 0xFF) as u8 as i32;
            let a1 = ((a >> (i * 16 + 8)) & 0xFF) as u8 as i32;
            let b0 = ((b >> (i * 16)) & 0xFF) as u8 as i8 as i32;
            let b1 = ((b >> (i * 16 + 8)) & 0xFF) as u8 as i8 as i32;
            let r = (a0 * b0 + a1 * b1).clamp(i16::MIN as i32, i16::MAX as i32) as i16 as u16;
            out |= (r as u128) << (i * 16);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn psadbw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let diff = vabdq_u8(qb(a), qb(b));
        let sum_lo = vaddlv_u8(vget_low_u8(diff)) as u64;
        let sum_hi = vaddlv_u8(vget_high_u8(diff)) as u64;
        (sum_lo as u128) | ((sum_hi as u128) << 64)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut sum_lo = 0u64;
        let mut sum_hi = 0u64;
        for i in 0..8 {
            sum_lo += ((a >> (i * 8)) as u8).abs_diff((b >> (i * 8)) as u8) as u64;
        }
        for i in 8..16 {
            sum_hi += ((a >> (i * 8)) as u8).abs_diff((b >> (i * 8)) as u8) as u64;
        }
        (sum_lo as u128) | ((sum_hi as u128) << 64)
    }
}

#[inline(always)]
pub(in crate::emu) fn phadd_sub_sw(a: u128, b: u128, sub: bool) -> u128 {
    let mut result = 0u128;
    for (j, value) in [a, b].into_iter().enumerate() {
        for i in 0..4 {
            let pair = value >> (32*i);
            let (x, y) = (pair as i16, (pair >> 16) as i16);
            let sum = if sub { x.saturating_sub(y) } else { x.saturating_add(y) };
            result |= u128::from(sum as u16) << (64*j + 16*i);
        }
    }
    result
}

#[inline(always)]
pub(in crate::emu) fn phadd_sub_w(a: u128, b: u128, sub: bool) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let (va, vb) = (qw(a), qw(b));
        if sub {
            wq(vsubq_u16(vuzp1q_u16(va, vb), vuzp2q_u16(va, vb)))
        } else {
            wq(vpaddq_u16(va, vb))
        }
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut r = 0u128;
        for i in 0..8u32 {
            let src = if i < 4 { a } else { b };
            let j = (i & 3) * 2;
            let x = (src >> (16 * j)) as u16;
            let y = (src >> (16 * (j + 1))) as u16;
            let v = if sub { x.wrapping_sub(y) } else { x.wrapping_add(y) };
            r |= (v as u128) << (16 * i);
        }
        r
    }
}

#[inline(always)]
pub(in crate::emu) fn phadd_sub_d(a: u128, b: u128, sub: bool) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let (va, vb) = (qu(a), qu(b));
        if sub {
            uq(vsubq_u32(vuzp1q_u32(va, vb), vuzp2q_u32(va, vb)))
        } else {
            uq(vpaddq_u32(va, vb))
        }
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut r = 0u128;
        for i in 0..4u32 {
            let src = if i < 2 { a } else { b };
            let j = (i & 1) * 2;
            let x = (src >> (32 * j)) as u32;
            let y = (src >> (32 * (j + 1))) as u32;
            let v = if sub { x.wrapping_sub(y) } else { x.wrapping_add(y) };
            r |= (v as u128) << (32 * i);
        }
        r
    }
}

// ---- Packed integer compares & movemask ----

#[inline(always)]
pub(in crate::emu) fn pcmpeqb(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { bq(vceqq_u8(qb(a), qb(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..16 {
            if ((a >> (i * 8)) as u8) == ((b >> (i * 8)) as u8) {
                out |= 0xFFu128 << (i * 8);
            }
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pcmpgtb(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { bq(vcgtq_s8(qsb(a), qsb(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..16 {
            if ((a >> (i * 8)) as u8 as i8) > ((b >> (i * 8)) as u8 as i8) {
                out |= 0xFFu128 << (i * 8);
            }
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pcmpeqw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { wq(vceqq_u16(qw(a), qw(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            if ((a >> (i * 16)) as u16) == ((b >> (i * 16)) as u16) {
                out |= 0xFFFFu128 << (i * 16);
            }
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pcmpgtw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { wq(vcgtq_s16(qsw(a), qsw(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            if ((a >> (i * 16)) as u16 as i16) > ((b >> (i * 16)) as u16 as i16) {
                out |= 0xFFFFu128 << (i * 16);
            }
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pcmpeqd(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { uq(vceqq_u32(qu(a), qu(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..4 {
            if ((a >> (i * 32)) as u32) == ((b >> (i * 32)) as u32) {
                out |= 0xFFFF_FFFFu128 << (i * 32);
            }
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pcmpgtd(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { uq(vcgtq_s32(qs(a), qs(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..4 {
            if ((a >> (i * 32)) as u32 as i32) > ((b >> (i * 32)) as u32 as i32) {
                out |= 0xFFFF_FFFFu128 << (i * 32);
            }
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pcmpeqq(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { lq(vceqq_u64(ql(a), ql(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let l0 = if (a as u64) == (b as u64) { u64::MAX as u128 } else { 0 };
        let l1 = if ((a >> 64) as u64) == ((b >> 64) as u64) { (u64::MAX as u128) << 64 } else { 0 };
        l0 | l1
    }
}

#[inline(always)]
pub(in crate::emu) fn pcmpgtq(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { lq(vcgtq_s64(qsl(a), qsl(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let l0 = if (a as i64) > (b as i64) { u64::MAX as u128 } else { 0 };
        let l1 = if ((a >> 64) as i64) > ((b >> 64) as i64) { (u64::MAX as u128) << 64 } else { 0 };
        l0 | l1
    }
}

#[inline(always)]
pub(in crate::emu) fn pmovmskb(lo: u128) -> u64 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        const BITS: [u8; 16] = [1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128];
        let sign = vreinterpretq_u8_s8(vshrq_n_s8::<7>(qsb(lo)));
        let masked = vandq_u8(sign, vld1q_u8(BITS.as_ptr()));
        let p1 = vpaddq_u8(masked, masked);
        let p2 = vpaddq_u8(p1, p1);
        let p3 = vpaddq_u8(p2, p2);
        vgetq_lane_u16::<0>(vreinterpretq_u16_u8(p3)) as u64
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut mask = 0u64;
        for i in 0..16 {
            if ((lo >> (i * 8 + 7)) & 1) != 0 {
                mask |= 1u64 << i;
            }
        }
        mask
    }
}

#[inline(always)]
pub(in crate::emu) fn pmovmskb_ymm(lo: u128, hi: u128) -> u64 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        const BITS: [u8; 16] = [1, 2, 4, 8, 16, 32, 64, 128, 1, 2, 4, 8, 16, 32, 64, 128];
        let w = vld1q_u8(BITS.as_ptr());
        let m_lo = vandq_u8(vreinterpretq_u8_s8(vshrq_n_s8::<7>(qsb(lo))), w);
        let m_hi = vandq_u8(vreinterpretq_u8_s8(vshrq_n_s8::<7>(qsb(hi))), w);
        let p1 = vpaddq_u8(m_lo, m_hi);
        let p2 = vpaddq_u8(p1, p1);
        let p3 = vpaddq_u8(p2, p2);
        vgetq_lane_u32::<0>(vreinterpretq_u32_u8(p3)) as u64
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        pmovmskb(lo) | (pmovmskb(hi) << 16)
    }
}

#[inline(always)]
pub(in crate::emu) fn movmskps(lo: u128) -> u64 {
    (((lo >> 31) & 1)
        | (((lo >> 63) & 1) << 1)
        | (((lo >> 95) & 1) << 2)
        | (((lo >> 127) & 1) << 3)) as u64
}

#[inline(always)]
pub(in crate::emu) fn movmskps_ymm(lo: u128, hi: u128) -> u64 {
    movmskps(lo) | (movmskps(hi) << 4)
}

#[inline(always)]
pub(in crate::emu) fn movmskpd(lo: u128) -> u64 {
    (((lo >> 63) & 1) | (((lo >> 127) & 1) << 1)) as u64
}

#[inline(always)]
pub(in crate::emu) fn movmskpd_ymm(lo: u128, hi: u128) -> u64 {
    movmskpd(lo) | (movmskpd(hi) << 2)
}

// ---- Uniform & variable packed shifts ----

/// `kind`: 0 = logical left (`psllw`), 1 = logical right (`psrlw`), 2 = arithmetic right (`psraw`).
#[inline(always)]
pub(in crate::emu) fn pshift_w(v: u128, cnt: u64, kind: u8) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        match kind {
            2 => {
                let sh = cnt.min(15) as i16;
                swq(vshlq_s16(qsw(v), vdupq_n_s16(-sh)))
            }
            1 => {
                if cnt >= 16 { 0 } else { wq(vshlq_u16(qw(v), vdupq_n_s16(-(cnt as i16)))) }
            }
            _ => {
                if cnt >= 16 { 0 } else { wq(vshlq_u16(qw(v), vdupq_n_s16(cnt as i16))) }
            }
        }
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..8 {
            let u = (v >> (lane * 16)) as u16;
            let r = match kind {
                2 => ((u as i16) >> (cnt.min(15) as u32)) as u16,
                1 => if cnt >= 16 { 0 } else { u >> (cnt as u32) },
                _ => if cnt >= 16 { 0 } else { u << (cnt as u32) },
            };
            out |= (r as u128) << (lane * 16);
        }
        out
    }
}

/// `kind`: 0 = logical left (`pslld`), 1 = logical right (`psrld`), 2 = arithmetic right (`psrad`).
#[inline(always)]
pub(in crate::emu) fn pshift_d(v: u128, cnt: u64, kind: u8) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        match kind {
            2 => {
                let sh = cnt.min(31) as i32;
                sq(vshlq_s32(qs(v), vdupq_n_s32(-sh)))
            }
            1 => {
                if cnt >= 32 { 0 } else { uq(vshlq_u32(qu(v), vdupq_n_s32(-(cnt as i32)))) }
            }
            _ => {
                if cnt >= 32 { 0 } else { uq(vshlq_u32(qu(v), vdupq_n_s32(cnt as i32))) }
            }
        }
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..4 {
            let u = (v >> (lane * 32)) as u32;
            let r = match kind {
                2 => ((u as i32) >> (cnt.min(31) as u32)) as u32,
                1 => if cnt >= 32 { 0 } else { u >> (cnt as u32) },
                _ => if cnt >= 32 { 0 } else { u << (cnt as u32) },
            };
            out |= (r as u128) << (lane * 32);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pshift_q(v: u128, cnt: u64, is_shr: bool) -> u128 {
    if cnt >= 64 {
        return 0;
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let sh = if is_shr { -(cnt as i64) } else { cnt as i64 };
        lq(vshlq_u64(ql(v), vdupq_n_s64(sh)))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let sh = cnt as u32;
        let l0 = if is_shr { (v as u64) >> sh } else { (v as u64) << sh };
        let l1 = if is_shr { ((v >> 64) as u64) >> sh } else { ((v >> 64) as u64) << sh };
        (l0 as u128) | ((l1 as u128) << 64)
    }
}

#[inline(always)]
pub(in crate::emu) fn pshift_dq(v: u128, bytes_cnt: u32, is_shr: bool) -> u128 {
    if bytes_cnt >= 16 {
        0
    } else if is_shr {
        v >> (bytes_cnt * 8)
    } else {
        v << (bytes_cnt * 8)
    }
}

/// `kind`: 0 = `vpsllvd`, 1 = `vpsrlvd`, 2 = `vpsravd`.
#[inline(always)]
pub(in crate::emu) fn vpshift_vd(a: u128, c: u128, kind: u8) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        match kind {
            2 => {
                let s = vreinterpretq_s32_u32(vminq_u32(qu(c), vdupq_n_u32(31)));
                sq(vshlq_s32(qs(a), vnegq_s32(s)))
            }
            1 => {
                let s = vreinterpretq_s32_u32(vminq_u32(qu(c), vdupq_n_u32(32)));
                uq(vshlq_u32(qu(a), vnegq_s32(s)))
            }
            _ => {
                let s = vreinterpretq_s32_u32(vminq_u32(qu(c), vdupq_n_u32(32)));
                uq(vshlq_u32(qu(a), s))
            }
        }
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..4 {
            let va = (a >> (i * 32)) as u32;
            let vc = (c >> (i * 32)) as u32;
            let r = match kind {
                2 => ((va as i32) >> vc.min(31)) as u32,
                1 => if vc > 31 { 0 } else { va >> vc },
                _ => if vc > 31 { 0 } else { va << vc },
            };
            out |= (r as u128) << (i * 32);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn vpshift_vq(a: u128, c: u128, is_shr: bool) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let in_range = vcltq_u64(ql(c), vdupq_n_u64(64));
        let sh = vreinterpretq_s64_u64(vandq_u64(ql(c), in_range));
        let sh = if is_shr { vnegq_s64(sh) } else { sh };
        lq(vandq_u64(vshlq_u64(ql(a), sh), in_range))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let do_lane = |va: u64, vc: u64| -> u64 {
            if vc > 63 { 0 } else if is_shr { va >> vc } else { va << vc }
        };
        let l0 = do_lane(a as u64, c as u64);
        let l1 = do_lane((a >> 64) as u64, (c >> 64) as u64);
        (l0 as u128) | ((l1 as u128) << 64)
    }
}

// ---- Shuffles, blends, packs, unpacks, broadcasts, sign/zero extend ----

#[inline(always)]
pub(in crate::emu) fn pshufb(a: u128, mask: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { bq(vqtbl1q_u8(qb(a), vandq_u8(qb(mask), vdupq_n_u8(0x8F)))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..16 {
            let ctl = (mask >> (i * 8)) as u8;
            if (ctl & 0x80) == 0 {
                let idx = (ctl & 0x0F) as u32;
                out |= ((a >> (idx * 8)) & 0xFF) << (i * 8);
            }
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn pshufd(a: u128, imm: u8) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let shifts: int32x4_t = core::mem::transmute([0i32, -2, -4, -6]);
        let sel = vandq_u32(vshlq_u32(vdupq_n_u32(imm as u32), shifts), vdupq_n_u32(3));
        let idx = vmlaq_u32(vdupq_n_u32(0x0302_0100), sel, vdupq_n_u32(0x0404_0404));
        bq(vqtbl1q_u8(qb(a), vreinterpretq_u8_u32(idx)))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..4 {
            let sel = ((imm >> (lane * 2)) & 3) as u32;
            out |= (((a >> (sel * 32)) as u32) as u128) << (lane * 32);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn shufps(a: u128, b: u128, imm: u8) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let shifts: int32x4_t = core::mem::transmute([0i32, -2, -4, -6]);
        let base: uint32x4_t = core::mem::transmute([0x0302_0100u32, 0x0302_0100, 0x1312_1110, 0x1312_1110]);
        let sel = vandq_u32(vshlq_u32(vdupq_n_u32(imm as u32), shifts), vdupq_n_u32(3));
        let idx = vmlaq_u32(base, sel, vdupq_n_u32(0x0404_0404));
        bq(vqtbl2q_u8(uint8x16x2_t(qb(a), qb(b)), vreinterpretq_u8_u32(idx)))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let w0 = ((a >> (((imm & 3) as u32) * 32)) as u32) as u128;
        let w1 = ((a >> ((((imm >> 2) & 3) as u32) * 32)) as u32) as u128;
        let w2 = ((b >> ((((imm >> 4) & 3) as u32) * 32)) as u32) as u128;
        let w3 = ((b >> ((((imm >> 6) & 3) as u32) * 32)) as u32) as u128;
        w0 | (w1 << 32) | (w2 << 64) | (w3 << 96)
    }
}

#[inline(always)]
pub(in crate::emu) fn shufpd(a: u128, b: u128, imm2: u8) -> u128 {
    let q0 = ((a >> (((imm2 & 1) as u32) * 64)) as u64) as u128;
    let q1 = ((b >> ((((imm2 >> 1) & 1) as u32) * 64)) as u64) as u128;
    q0 | (q1 << 64)
}

#[inline(always)]
pub(in crate::emu) fn pshuflw(v: u128, imm: u8) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let shifts: int16x4_t = core::mem::transmute([0i16, -2, -4, -6]);
        let sel = vand_u16(vshl_u16(vdup_n_u16(imm as u16), shifts), vdup_n_u16(3));
        let idx_lo = vreinterpret_u8_u16(vmla_u16(vdup_n_u16(0x0100), sel, vdup_n_u16(0x0202)));
        let idx_hi: uint8x8_t = core::mem::transmute([8u8, 9, 10, 11, 12, 13, 14, 15]);
        bq(vqtbl1q_u8(qb(v), vcombine_u8(idx_lo, idx_hi)))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut low64 = 0u64;
        for lane in 0..4 {
            let sel = ((imm >> (lane * 2)) & 3) as usize;
            low64 |= (((v >> (sel * 16)) & 0xFFFF) as u64) << (lane * 16);
        }
        (v & !0xFFFF_FFFF_FFFF_FFFFu128) | (low64 as u128)
    }
}

#[inline(always)]
pub(in crate::emu) fn pshufhw(v: u128, imm: u8) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let shifts: int16x4_t = core::mem::transmute([0i16, -2, -4, -6]);
        let sel = vand_u16(vshl_u16(vdup_n_u16(imm as u16), shifts), vdup_n_u16(3));
        let idx_lo: uint8x8_t = core::mem::transmute([0u8, 1, 2, 3, 4, 5, 6, 7]);
        let idx_hi = vreinterpret_u8_u16(vmla_u16(vdup_n_u16(0x0908), sel, vdup_n_u16(0x0202)));
        bq(vqtbl1q_u8(qb(v), vcombine_u8(idx_lo, idx_hi)))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let hi_in = (v >> 64) as u64;
        let mut hi64 = 0u64;
        for lane in 0..4 {
            let sel = ((imm >> (lane * 2)) & 3) as usize;
            hi64 |= ((hi_in >> (sel * 16)) & 0xFFFF) << (lane * 16);
        }
        (v & 0xFFFF_FFFF_FFFF_FFFFu128) | ((hi64 as u128) << 64)
    }
}

#[inline(always)]
pub(in crate::emu) fn palignr(a: u128, b: u128, imm: u8) -> u128 {
    let sh = imm as u32;
    if sh >= 32 {
        0
    } else if sh >= 16 {
        a >> ((sh - 16) * 8)
    } else if sh == 0 {
        b
    } else {
        (b >> (sh * 8)) | (a << ((16 - sh) * 8))
    }
}

#[inline(always)]
pub(in crate::emu) fn insertps(a: u128, b: u128, imm: u8, from_mem: bool) -> u128 {
    let count_s = if from_mem { 0 } else { ((imm >> 6) & 3) as usize };
    let count_d = ((imm >> 4) & 3) as usize;
    let zmask = imm & 0xF;
    let elem = ((b >> (count_s * 32)) & 0xFFFF_FFFF) as u32;
    let mut out = (a & !(0xFFFF_FFFFu128 << (count_d * 32))) | ((elem as u128) << (count_d * 32));
    for lane in 0..4 {
        if ((zmask >> lane) & 1) != 0 {
            out &= !(0xFFFF_FFFFu128 << (lane * 32));
        }
    }
    out
}

#[inline(always)]
pub(in crate::emu) fn vpermilps_var(v: u128, ctrl: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let sel = vandq_u32(qu(ctrl), vdupq_n_u32(3));
        let idx = vmlaq_u32(vdupq_n_u32(0x0302_0100), sel, vdupq_n_u32(0x0404_0404));
        bq(vqtbl1q_u8(qb(v), vreinterpretq_u8_u32(idx)))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..4 {
            let sel = ((ctrl >> (lane * 32)) & 3) as u32;
            out |= (((v >> (sel * 32)) as u32) as u128) << (lane * 32);
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn vpermd_ymm(i_lo: u128, i_hi: u128, t_lo: u128, t_hi: u128) -> (u128, u128) {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let tbl = uint8x16x2_t(qb(t_lo), qb(t_hi));
        let base = vdupq_n_u32(0x0302_0100);
        let step = vdupq_n_u32(0x0404_0404);
        let seven = vdupq_n_u32(7);
        let idx_lo = vmlaq_u32(base, vandq_u32(qu(i_lo), seven), step);
        let idx_hi = vmlaq_u32(base, vandq_u32(qu(i_hi), seven), step);
        (
            bq(vqtbl2q_u8(tbl, vreinterpretq_u8_u32(idx_lo))),
            bq(vqtbl2q_u8(tbl, vreinterpretq_u8_u32(idx_hi))),
        )
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let idx: [u32; 8] = [
            i_lo as u32,
            (i_lo >> 32) as u32,
            (i_lo >> 64) as u32,
            (i_lo >> 96) as u32,
            i_hi as u32,
            (i_hi >> 32) as u32,
            (i_hi >> 64) as u32,
            (i_hi >> 96) as u32,
        ];
        let tab: [u32; 8] = [
            t_lo as u32,
            (t_lo >> 32) as u32,
            (t_lo >> 64) as u32,
            (t_lo >> 96) as u32,
            t_hi as u32,
            (t_hi >> 32) as u32,
            (t_hi >> 64) as u32,
            (t_hi >> 96) as u32,
        ];
        let r: [u32; 8] = core::array::from_fn(|i| tab[(idx[i] & 7) as usize]);
        let lo = (r[0] as u128) | ((r[1] as u128) << 32) | ((r[2] as u128) << 64) | ((r[3] as u128) << 96);
        let hi = (r[4] as u128) | ((r[5] as u128) << 32) | ((r[6] as u128) << 64) | ((r[7] as u128) << 96);
        (lo, hi)
    }
}

#[inline(always)]
pub(in crate::emu) fn vpermq(s_lo: u128, s_hi: u128, imm: u8) -> (u128, u128) {
    let q: [u64; 4] = [s_lo as u64, (s_lo >> 64) as u64, s_hi as u64, (s_hi >> 64) as u64];
    let r0 = q[(imm & 3) as usize] as u128;
    let r1 = q[((imm >> 2) & 3) as usize] as u128;
    let r2 = q[((imm >> 4) & 3) as usize] as u128;
    let r3 = q[((imm >> 6) & 3) as usize] as u128;
    (r0 | (r1 << 64), r2 | (r3 << 64))
}

#[inline(always)]
pub(in crate::emu) fn vperm2_128(
    s1_lo: u128,
    s1_hi: u128,
    s2_lo: u128,
    s2_hi: u128,
    imm: u8,
) -> (u128, u128) {
    let pick = |ctrl: u8| -> u128 {
        if ctrl & 0x8 != 0 {
            0
        } else {
            match ctrl & 0x3 {
                0 => s1_lo,
                1 => s1_hi,
                2 => s2_lo,
                _ => s2_hi,
            }
        }
    };
    (pick(imm & 0xF), pick((imm >> 4) & 0xF))
}

#[inline(always)]
pub(in crate::emu) fn blendvb(a: u128, b: u128, mask: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let m = vreinterpretq_u8_s8(vshrq_n_s8::<7>(qsb(mask)));
        bq(vbslq_u8(m, qb(b), qb(a)))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..16 {
            let sh = i * 8;
            let v = if ((mask >> (sh + 7)) & 1) != 0 { (b >> sh) & 0xFF } else { (a >> sh) & 0xFF };
            out |= v << sh;
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn blendvps(a: u128, b: u128, mask: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let m = vreinterpretq_u32_s32(vshrq_n_s32::<31>(qs(mask)));
        uq(vbslq_u32(m, qu(b), qu(a)))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..4 {
            let sh = i * 32;
            let v = if ((mask >> (sh + 31)) & 1) != 0 { (b >> sh) & 0xFFFF_FFFF } else { (a >> sh) & 0xFFFF_FFFF };
            out |= v << sh;
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn blendvpd(a: u128, b: u128, mask: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let m = vreinterpretq_u64_s64(vshrq_n_s64::<63>(qsl(mask)));
        lq(vbslq_u64(m, ql(b), ql(a)))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let lo = if ((mask >> 63) & 1) != 0 { b as u64 } else { a as u64 };
        let hi = if ((mask >> 127) & 1) != 0 { (b >> 64) as u64 } else { (a >> 64) as u64 };
        (lo as u128) | ((hi as u128) << 64)
    }
}

#[inline(always)]
pub(in crate::emu) fn blendps_imm(a: u128, b: u128, imm4: u8) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { uq(vbslq_u32(lane_mask4(imm4), qu(b), qu(a))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for lane in 0..4 {
            let mask = 0xFFFF_FFFFu128 << (lane * 32);
            out |= if ((imm4 >> lane) & 1) != 0 { b & mask } else { a & mask };
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn blendpd_imm(a: u128, b: u128, imm2: u8) -> u128 {
    let q0 = if (imm2 & 1) != 0 { b as u64 } else { a as u64 };
    let q1 = if (imm2 & 2) != 0 { (b >> 64) as u64 } else { (a >> 64) as u64 };
    (q0 as u128) | ((q1 as u128) << 64)
}

#[inline(always)]
pub(in crate::emu) fn pblendw_imm(a: u128, b: u128, imm: u8) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let bits: uint16x8_t = core::mem::transmute([1u16, 2, 4, 8, 16, 32, 64, 128]);
        let m = vtstq_u16(vdupq_n_u16(imm as u16), bits);
        wq(vbslq_u16(m, qw(b), qw(a)))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut r = a;
        for i in 0..8 {
            if ((imm >> i) & 1) != 0 {
                let m = 0xFFFFu128 << (16 * i);
                r = (r & !m) | (b & m);
            }
        }
        r
    }
}

#[inline(always)]
pub(in crate::emu) fn packuswb(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { bq(vcombine_u8(vqmovun_s16(qsw(a)), vqmovun_s16(qsw(b)))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let ba = ((a >> (i * 16)) as u16 as i16).clamp(0, 255) as u8 as u128;
            let bb = ((b >> (i * 16)) as u16 as i16).clamp(0, 255) as u8 as u128;
            out |= (ba << (i * 8)) | (bb << (64 + i * 8));
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn packsswb(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { sbq(vcombine_s8(vqmovn_s16(qsw(a)), vqmovn_s16(qsw(b)))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let ba = ((a >> (i * 16)) as u16 as i16).clamp(-128, 127) as i8 as u8 as u128;
            let bb = ((b >> (i * 16)) as u16 as i16).clamp(-128, 127) as i8 as u8 as u128;
            out |= (ba << (i * 8)) | (bb << (64 + i * 8));
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn packssdw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { swq(vcombine_s16(vqmovn_s32(qs(a)), vqmovn_s32(qs(b)))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..4 {
            let wa = ((a >> (i * 32)) as u32 as i32).clamp(i16::MIN as i32, i16::MAX as i32) as i16 as u16 as u128;
            let wb = ((b >> (i * 32)) as u32 as i32).clamp(i16::MIN as i32, i16::MAX as i32) as i16 as u16 as u128;
            out |= (wa << (i * 16)) | (wb << (64 + i * 16));
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn packusdw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { wq(vcombine_u16(vqmovun_s32(qs(a)), vqmovun_s32(qs(b)))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..4 {
            let wa = ((a >> (i * 32)) as u32 as i32).clamp(0, 65535) as u16 as u128;
            let wb = ((b >> (i * 32)) as u32 as i32).clamp(0, 65535) as u16 as u128;
            out |= (wa << (i * 16)) | (wb << (64 + i * 16));
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn punpcklbw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { bq(vzip1q_u8(qb(a), qb(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let ba = (a >> (i * 8)) & 0xFF;
            let bb = (b >> (i * 8)) & 0xFF;
            out |= (ba << (i * 16)) | (bb << (i * 16 + 8));
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn punpckhbw(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { bq(vzip2q_u8(qb(a), qb(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..8 {
            let ba = (a >> (64 + i * 8)) & 0xFF;
            let bb = (b >> (64 + i * 8)) & 0xFF;
            out |= (ba << (i * 16)) | (bb << (i * 16 + 8));
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn punpcklwd(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { wq(vzip1q_u16(qw(a), qw(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..4 {
            let wa = (a >> (i * 16)) & 0xFFFF;
            let wb = (b >> (i * 16)) & 0xFFFF;
            out |= (wa << (i * 32)) | (wb << (i * 32 + 16));
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn punpckhwd(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { wq(vzip2q_u16(qw(a), qw(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut out = 0u128;
        for i in 0..4 {
            let wa = (a >> (64 + i * 16)) & 0xFFFF;
            let wb = (b >> (64 + i * 16)) & 0xFFFF;
            out |= (wa << (i * 32)) | (wb << (i * 32 + 16));
        }
        out
    }
}

#[inline(always)]
pub(in crate::emu) fn punpckldq(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { uq(vzip1q_u32(qu(a), qu(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let a0 = (a as u32) as u128;
        let a1 = ((a >> 32) as u32) as u128;
        let b0 = (b as u32) as u128;
        let b1 = ((b >> 32) as u32) as u128;
        a0 | (b0 << 32) | (a1 << 64) | (b1 << 96)
    }
}

#[inline(always)]
pub(in crate::emu) fn punpckhdq(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { uq(vzip2q_u32(qu(a), qu(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let a2 = ((a >> 64) as u32) as u128;
        let a3 = ((a >> 96) as u32) as u128;
        let b2 = ((b >> 64) as u32) as u128;
        let b3 = ((b >> 96) as u32) as u128;
        a2 | (b2 << 32) | (a3 << 64) | (b3 << 96)
    }
}

#[inline(always)]
pub(in crate::emu) fn punpcklqdq(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { lq(vzip1q_u64(ql(a), ql(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    { (a as u64 as u128) | ((b as u64 as u128) << 64) }
}

#[inline(always)]
pub(in crate::emu) fn punpckhqdq(a: u128, b: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { lq(vzip2q_u64(ql(a), ql(b))) }
    #[cfg(not(target_arch = "aarch64"))]
    { (a >> 64) | ((b >> 64) << 64) }
}

#[inline(always)]
pub(in crate::emu) fn movsldup(v: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { uq(vtrn1q_u32(qu(v), qu(v))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let w0 = (v as u32) as u128;
        let w2 = ((v >> 64) as u32) as u128;
        w0 | (w0 << 32) | (w2 << 64) | (w2 << 96)
    }
}

#[inline(always)]
pub(in crate::emu) fn movshdup(v: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { uq(vtrn2q_u32(qu(v), qu(v))) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let w1 = ((v >> 32) as u32) as u128;
        let w3 = ((v >> 96) as u32) as u128;
        w1 | (w1 << 32) | (w3 << 64) | (w3 << 96)
    }
}

#[inline(always)]
pub(in crate::emu) fn movddup(v: u128) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe { lq(vdupq_n_u64(v as u64)) }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let q = (v as u64) as u128;
        q | (q << 64)
    }
}

#[inline(always)]
pub(in crate::emu) fn pbroadcast(val: u128, elem_bits: u32) -> u128 {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        match elem_bits {
            8 => bq(vdupq_n_u8(val as u8)),
            16 => wq(vdupq_n_u16(val as u16)),
            32 => uq(vdupq_n_u32(val as u32)),
            _ => lq(vdupq_n_u64(val as u64)),
        }
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        match elem_bits {
            8 => u128::from_le_bytes([val as u8; 16]),
            16 => {
                let w = (val as u16) as u128;
                let w2 = w | (w << 16);
                let w4 = w2 | (w2 << 32);
                w4 | (w4 << 64)
            }
            32 => {
                let d = (val as u32) as u128;
                let d2 = d | (d << 32);
                d2 | (d2 << 64)
            }
            _ => {
                let q = (val as u64) as u128;
                q | (q << 64)
            }
        }
    }
}

/// Vectorized `PMOVSX*` / `PMOVZX*` (`code`: 0=sxbw, 1=zxbw, 2=sxbd, 3=zxbd, 4=sxbq, 5=zxbq,
/// 6=sxwd, 7=zxwd, 8=sxwq, 9=zxwq, 10=sxdq, 11=zxdq).
#[inline(always)]
pub(in crate::emu) fn pmovsx_zx(raw: u128, code: u8, ymm: bool) -> (u128, u128) {
    #[cfg(target_arch = "aarch64")]
    unsafe {
        match code {
            0 => {
                let v = qsb(raw);
                (swq(vmovl_s8(vget_low_s8(v))), if ymm { swq(vmovl_high_s8(v)) } else { 0 })
            }
            1 => {
                let v = qb(raw);
                (wq(vmovl_u8(vget_low_u8(v))), if ymm { wq(vmovl_high_u8(v)) } else { 0 })
            }
            2 => {
                let w = vmovl_s8(vget_low_s8(qsb(raw)));
                (sq(vmovl_s16(vget_low_s16(w))), if ymm { sq(vmovl_high_s16(w)) } else { 0 })
            }
            3 => {
                let w = vmovl_u8(vget_low_u8(qb(raw)));
                (uq(vmovl_u16(vget_low_u16(w))), if ymm { uq(vmovl_high_u16(w)) } else { 0 })
            }
            4 => {
                let w = vmovl_s8(vget_low_s8(qsb(raw)));
                let d = vmovl_s16(vget_low_s16(w));
                let lo: u128 = core::mem::transmute(vmovl_s32(vget_low_s32(d)));
                let hi: u128 = if ymm { core::mem::transmute(vmovl_high_s32(d)) } else { 0 };
                (lo, hi)
            }
            5 => {
                let w = vmovl_u8(vget_low_u8(qb(raw)));
                let d = vmovl_u16(vget_low_u16(w));
                (lq(vmovl_u32(vget_low_u32(d))), if ymm { lq(vmovl_high_u32(d)) } else { 0 })
            }
            6 => {
                let v = qsw(raw);
                (sq(vmovl_s16(vget_low_s16(v))), if ymm { sq(vmovl_high_s16(v)) } else { 0 })
            }
            7 => {
                let v = qw(raw);
                (uq(vmovl_u16(vget_low_u16(v))), if ymm { uq(vmovl_high_u16(v)) } else { 0 })
            }
            8 => {
                let d = vmovl_s16(vget_low_s16(qsw(raw)));
                let lo: u128 = core::mem::transmute(vmovl_s32(vget_low_s32(d)));
                let hi: u128 = if ymm { core::mem::transmute(vmovl_high_s32(d)) } else { 0 };
                (lo, hi)
            }
            9 => {
                let d = vmovl_u16(vget_low_u16(qw(raw)));
                (lq(vmovl_u32(vget_low_u32(d))), if ymm { lq(vmovl_high_u32(d)) } else { 0 })
            }
            10 => {
                let v = qs(raw);
                let lo: u128 = core::mem::transmute(vmovl_s32(vget_low_s32(v)));
                let hi: u128 = if ymm { core::mem::transmute(vmovl_high_s32(v)) } else { 0 };
                (lo, hi)
            }
            _ => {
                let v = qu(raw);
                (lq(vmovl_u32(vget_low_u32(v))), if ymm { lq(vmovl_high_u32(v)) } else { 0 })
            }
        }
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let (src_bits, dst_bits, signed) = match code {
            0 => (8usize, 16usize, true),
            1 => (8, 16, false),
            2 => (8, 32, true),
            3 => (8, 32, false),
            4 => (8, 64, true),
            5 => (8, 64, false),
            6 => (16, 32, true),
            7 => (16, 32, false),
            8 => (16, 64, true),
            9 => (16, 64, false),
            10 => (32, 64, true),
            _ => (32, 64, false),
        };
        let lanes_128 = 128 / dst_bits;
        let total_lanes = if ymm { lanes_128 * 2 } else { lanes_128 };
        let mut out_lo = 0u128;
        let mut out_hi = 0u128;
        for lane in 0..total_lanes {
            let s_val = (raw >> (lane * src_bits)) & ((1u128 << src_bits) - 1);
            let ext: u64 = if signed {
                match src_bits {
                    8 => ((s_val as u8 as i8) as i64) as u64,
                    16 => ((s_val as u16 as i16) as i64) as u64,
                    _ => ((s_val as u32 as i32) as i64) as u64,
                }
            } else {
                s_val as u64
            };
            let d_mask = if dst_bits >= 64 { u64::MAX as u128 } else { (1u128 << dst_bits) - 1 };
            let d_val = (ext as u128) & d_mask;
            if lane < lanes_128 {
                out_lo |= d_val << (lane * dst_bits);
            } else {
                out_hi |= d_val << ((lane - lanes_128) * dst_bits);
            }
        }
        (out_lo, out_hi)
    }
}

// ---- PCLMULQDQ, AES-NI, F16C, CRC32C, PCMPSTR, SHA-NI ----

#[inline(always)]
pub(in crate::emu) fn pclmulqdq(a: u128, b: u128, imm: u8) -> u128 {
    let x = if (imm & 0x01) != 0 { (a >> 64) as u64 } else { a as u64 };
    let y = if (imm & 0x10) != 0 { (b >> 64) as u64 } else { b as u64 };
    #[cfg(target_arch = "aarch64")]
    if super::features::host().has(super::features::HostFeatures::PMULL) {
        return unsafe { vmull_p64(x, y) };
    }
    {
        let mut res = 0u128;
        let mut xx = x as u128;
        let mut yy = y;
        while yy != 0 {
            if (yy & 1) != 0 {
                res ^= xx;
            }
            xx <<= 1;
            yy >>= 1;
        }
        res
    }
}

/// Hardware-accelerated (ARMv8 Crypto `aes` extension) or portable AES-NI 128-bit op.
/// `sub`: 0=enc, 1=enclast, 2=dec, 3=declast, 4=imc, 5=keygenassist.
#[inline(always)]
pub(in crate::emu) fn aes_op_128(sub: u8, state: u128, src: u128, imm8: u8) -> u128 {
    #[cfg(target_arch = "aarch64")]
    if super::features::host().has(super::features::HostFeatures::AES) {
        return unsafe {
            let va = qb(state);
            let vb = qb(src);
            let zero = vdupq_n_u8(0);
            match sub {
                0 => bq(veorq_u8(vaesmcq_u8(vaeseq_u8(va, zero)), vb)),
                1 => bq(veorq_u8(vaeseq_u8(va, zero), vb)),
                2 => bq(veorq_u8(vaesimcq_u8(vaesdq_u8(va, zero)), vb)),
                3 => bq(veorq_u8(vaesdq_u8(va, zero), vb)),
                4 => bq(vaesimcq_u8(vb)),
                _ => {
                    // AESKEYGENASSIST: SubBytes via `vaeseq_u8(src, 0)`, then undo ShiftRows
                    // while duplicating/rotating words 1 and 3, and XOR with `[0, rcon, 0, rcon]`.
                    const SHUF: [u8; 16] = [4, 1, 14, 11, 1, 14, 11, 4, 12, 9, 6, 3, 9, 6, 3, 12];
                    let e = vaeseq_u8(vb, zero);
                    let w = vqtbl1q_u8(e, vld1q_u8(SHUF.as_ptr()));
                    let rcon = imm8 as u32;
                    let rc: [u32; 4] = [0, rcon, 0, rcon];
                    uq(veorq_u32(vreinterpretq_u32_u8(w), vld1q_u32(rc.as_ptr())))
                }
            }
        };
    }
    {
        const SBOX: [u8; 256] = [
            0x63,0x7c,0x77,0x7b,0xf2,0x6b,0x6f,0xc5,0x30,0x01,0x67,0x2b,0xfe,0xd7,0xab,0x76,
            0xca,0x82,0xc9,0x7d,0xfa,0x59,0x47,0xf0,0xad,0xd4,0xa2,0xaf,0x9c,0xa4,0x72,0xc0,
            0xb7,0xfd,0x93,0x26,0x36,0x3f,0xf7,0xcc,0x34,0xa5,0xe5,0xf1,0x71,0xd8,0x31,0x15,
            0x04,0xc7,0x23,0xc3,0x18,0x96,0x05,0x9a,0x07,0x12,0x80,0xe2,0xeb,0x27,0xb2,0x75,
            0x09,0x83,0x2c,0x1a,0x1b,0x6e,0x5a,0xa0,0x52,0x3b,0xd6,0xb3,0x29,0xe3,0x2f,0x84,
            0x53,0xd1,0x00,0xed,0x20,0xfc,0xb1,0x5b,0x6a,0xcb,0xbe,0x39,0x4a,0x4c,0x58,0xcf,
            0xd0,0xef,0xaa,0xfb,0x43,0x4d,0x33,0x85,0x45,0xf9,0x02,0x7f,0x50,0x3c,0x9f,0xa8,
            0x51,0xa3,0x40,0x8f,0x92,0x9d,0x38,0xf5,0xbc,0xb6,0xda,0x21,0x10,0xff,0xf3,0xd2,
            0xcd,0x0c,0x13,0xec,0x5f,0x97,0x44,0x17,0xc4,0xa7,0x7e,0x3d,0x64,0x5d,0x19,0x73,
            0x60,0x81,0x4f,0xdc,0x22,0x2a,0x90,0x88,0x46,0xee,0xb8,0x14,0xde,0x5e,0x0b,0xdb,
            0xe0,0x32,0x3a,0x0a,0x49,0x06,0x24,0x5c,0xc2,0xd3,0xac,0x62,0x91,0x95,0xe4,0x79,
            0xe7,0xc8,0x37,0x6d,0x8d,0xd5,0x4e,0xa9,0x6c,0x56,0xf4,0xea,0x65,0x7a,0xae,0x08,
            0xba,0x78,0x25,0x2e,0x1c,0xa6,0xb4,0xc6,0xe8,0xdd,0x74,0x1f,0x4b,0xbd,0x8b,0x8a,
            0x70,0x3e,0xb5,0x66,0x48,0x03,0xf6,0x0e,0x61,0x35,0x57,0xb9,0x86,0xc1,0x1d,0x9e,
            0xe1,0xf8,0x98,0x11,0x69,0xd9,0x8e,0x94,0x9b,0x1e,0x87,0xe9,0xce,0x55,0x28,0xdf,
            0x8c,0xa1,0x89,0x0d,0xbf,0xe6,0x42,0x68,0x41,0x99,0x2d,0x0f,0xb0,0x54,0xbb,0x16,
        ];
        const INV_SBOX: [u8; 256] = [
            0x52,0x09,0x6a,0xd5,0x30,0x36,0xa5,0x38,0xbf,0x40,0xa3,0x9e,0x81,0xf3,0xd7,0xfb,
            0x7c,0xe3,0x39,0x82,0x9b,0x2f,0xff,0x87,0x34,0x8e,0x43,0x44,0xc4,0xde,0xe9,0xcb,
            0x54,0x7b,0x94,0x32,0xa6,0xc2,0x23,0x3d,0xee,0x4c,0x95,0x0b,0x42,0xfa,0xc3,0x4e,
            0x08,0x2e,0xa1,0x66,0x28,0xd9,0x24,0xb2,0x76,0x5b,0xa2,0x49,0x6d,0x8b,0xd1,0x25,
            0x72,0xf8,0xf6,0x64,0x86,0x68,0x98,0x16,0xd4,0xa4,0x5c,0xcc,0x5d,0x65,0xb6,0x92,
            0x6c,0x70,0x48,0x50,0xfd,0xed,0xb9,0xda,0x5e,0x15,0x46,0x57,0xa7,0x8d,0x9d,0x84,
            0x90,0xd8,0xab,0x00,0x8c,0xbc,0xd3,0x0a,0xf7,0xe4,0x58,0x05,0xb8,0xb3,0x45,0x06,
            0xd0,0x2c,0x1e,0x8f,0xca,0x3f,0x0f,0x02,0xc1,0xaf,0xbd,0x03,0x01,0x13,0x8a,0x6b,
            0x3a,0x91,0x11,0x41,0x4f,0x67,0xdc,0xea,0x97,0xf2,0xcf,0xce,0xf0,0xb4,0xe6,0x73,
            0x96,0xac,0x74,0x22,0xe7,0xad,0x35,0x85,0xe2,0xf9,0x37,0xe8,0x1c,0x75,0xdf,0x6e,
            0x47,0xf1,0x1a,0x71,0x1d,0x29,0xc5,0x89,0x6f,0xb7,0x62,0x0e,0xaa,0x18,0xbe,0x1b,
            0xfc,0x56,0x3e,0x4b,0xc6,0xd2,0x79,0x20,0x9a,0xdb,0xc0,0xfe,0x78,0xcd,0x5a,0xf4,
            0x1f,0xdd,0xa8,0x33,0x88,0x07,0xc7,0x31,0xb1,0x12,0x10,0x59,0x27,0x80,0xec,0x5f,
            0x60,0x51,0x7f,0xa9,0x19,0xb5,0x4a,0x0d,0x2d,0xe5,0x7a,0x9f,0x93,0xc9,0x9c,0xef,
            0xa0,0xe0,0x3b,0x4d,0xae,0x2a,0xf5,0xb0,0xc8,0xeb,0xbb,0x3c,0x83,0x53,0x99,0x61,
            0x17,0x2b,0x04,0x7e,0xba,0x77,0xd6,0x26,0xe1,0x69,0x14,0x63,0x55,0x21,0x0c,0x7d,
        ];
        let xtime = |x: u8| -> u8 { (x << 1) ^ if (x & 0x80) != 0 { 0x1b } else { 0 } };
        let gmul = |mut a: u8, mut b: u8| -> u8 {
            let mut p = 0u8;
            for _ in 0..8 {
                if (b & 1) != 0 {
                    p ^= a;
                }
                a = xtime(a);
                b >>= 1;
            }
            p
        };
        let inv_mix_cols = |st: &mut [u8; 16]| {
            for c in 0..4 {
                let i = c * 4;
                let (s0, s1, s2, s3) = (st[i], st[i + 1], st[i + 2], st[i + 3]);
                st[i] = gmul(s0, 0x0e) ^ gmul(s1, 0x0b) ^ gmul(s2, 0x0d) ^ gmul(s3, 0x09);
                st[i + 1] = gmul(s0, 0x09) ^ gmul(s1, 0x0e) ^ gmul(s2, 0x0b) ^ gmul(s3, 0x0d);
                st[i + 2] = gmul(s0, 0x0d) ^ gmul(s1, 0x09) ^ gmul(s2, 0x0e) ^ gmul(s3, 0x0b);
                st[i + 3] = gmul(s0, 0x0b) ^ gmul(s1, 0x0d) ^ gmul(s2, 0x09) ^ gmul(s3, 0x0e);
            }
        };
        match sub {
            4 => {
                let mut st = src.to_le_bytes();
                inv_mix_cols(&mut st);
                u128::from_le_bytes(st)
            }
            5 => {
                let rcon = imm8 as u32;
                let sub_word = |w: u32| -> u32 {
                    (SBOX[(w & 0xFF) as usize] as u32)
                        | ((SBOX[((w >> 8) & 0xFF) as usize] as u32) << 8)
                        | ((SBOX[((w >> 16) & 0xFF) as usize] as u32) << 16)
                        | ((SBOX[((w >> 24) & 0xFF) as usize] as u32) << 24)
                };
                let t1 = sub_word((src >> 32) as u32);
                let t3 = sub_word((src >> 96) as u32);
                let w1 = t1.rotate_right(8) ^ rcon;
                let w3 = t3.rotate_right(8) ^ rcon;
                (t1 as u128) | ((w1 as u128) << 32) | ((t3 as u128) << 64) | ((w3 as u128) << 96)
            }
            _ => {
                let is_dec = sub >= 2;
                let is_last = (sub & 1) != 0;
                let mut st = state.to_le_bytes();
                let tmp = st;
                if !is_dec {
                    const SR: [usize; 16] = [0, 5, 10, 15, 4, 9, 14, 3, 8, 13, 2, 7, 12, 1, 6, 11];
                    for i in 0..16 {
                        st[i] = SBOX[tmp[SR[i]] as usize];
                    }
                    if !is_last {
                        for c in 0..4 {
                            let i = c * 4;
                            let (s0, s1, s2, s3) = (st[i], st[i + 1], st[i + 2], st[i + 3]);
                            st[i] = xtime(s0) ^ xtime(s1) ^ s1 ^ s2 ^ s3;
                            st[i + 1] = s0 ^ xtime(s1) ^ xtime(s2) ^ s2 ^ s3;
                            st[i + 2] = s0 ^ s1 ^ xtime(s2) ^ xtime(s3) ^ s3;
                            st[i + 3] = xtime(s0) ^ s0 ^ s1 ^ s2 ^ xtime(s3);
                        }
                    }
                } else {
                    const ISR: [usize; 16] = [0, 13, 10, 7, 4, 1, 14, 11, 8, 5, 2, 15, 12, 9, 6, 3];
                    for i in 0..16 {
                        st[i] = INV_SBOX[tmp[ISR[i]] as usize];
                    }
                    if !is_last {
                        inv_mix_cols(&mut st);
                    }
                }
                u128::from_le_bytes(st) ^ src
            }
        }
    }
}

#[inline(always)]
pub(in crate::emu) fn f16_to_f32_bits(h: u16) -> u32 {
    let sign = ((h as u32) & 0x8000) << 16;
    let exp = ((h >> 10) & 0x1F) as u32;
    let mant = (h & 0x03FF) as u32;
    if exp == 0 {
        if mant == 0 {
            sign
        } else {
            let f = (mant as f32) * (1.0 / 16777216.0);
            sign | f.to_bits()
        }
    } else if exp == 0x1F {
        sign | 0x7F80_0000 | (mant << 13)
    } else {
        sign | ((exp + 112) << 23) | (mant << 13)
    }
}

#[inline(always)]
pub(in crate::emu) fn f32_to_f16_bits(bits: u32) -> u16 {
    let sign = ((bits >> 16) & 0x8000) as u16;
    let exp = ((bits >> 23) & 0xFF) as i32;
    let mant = bits & 0x007F_FFFF;
    if exp == 0xFF {
        if mant != 0 {
            sign | 0x7E00 | ((mant >> 13) as u16)
        } else {
            sign | 0x7C00
        }
    } else if exp > 142 {
        sign | 0x7C00
    } else if exp < 113 {
        // Subnormal half-precision range: smallest normal f16 is 2^-14 (f32 exp 113),
        // half-way to smallest subnormal 2^-24 is 2^-25 (f32 exp 102).
        if exp < 102 {
            sign
        } else {
            let sig = mant | 0x0080_0000;
            let tot = (126 - exp) as u32; // 14..=24
            let q = sig >> tot;
            let half = 1u32 << (tot - 1);
            let rem = sig & ((1u32 << tot) - 1);
            let round_up = rem > half || (rem == half && (q & 1) != 0);
            sign | ((q + round_up as u32) as u16)
        }
    } else {
        let e = ((exp - 112) as u16) << 10;
        let m = ((mant + 0x0FFF + ((mant >> 13) & 1)) >> 13) as u16;
        sign | (e + m)
    }
}

#[inline(always)]
pub(in crate::emu) fn vcvtph2ps(raw: u128, is_256: bool) -> (u128, u128) {
    let mut d_lo = 0u128;
    for lane in 0..4 {
        let h = ((raw >> (lane * 16)) & 0xFFFF) as u16;
        d_lo |= (f16_to_f32_bits(h) as u128) << (lane * 32);
    }
    let mut d_hi = 0u128;
    if is_256 {
        for lane in 0..4 {
            let h = ((raw >> ((lane + 4) * 16)) & 0xFFFF) as u16;
            d_hi |= (f16_to_f32_bits(h) as u128) << (lane * 32);
        }
    }
    (d_lo, d_hi)
}

#[inline(always)]
pub(in crate::emu) fn vcvtps2ph(lo: u128, hi: u128, is_256: bool) -> u128 {
    let mut packed = 0u128;
    for lane in 0..4 {
        let w32 = (lo >> (lane * 32)) as u32;
        packed |= (f32_to_f16_bits(w32) as u128) << (lane * 16);
    }
    if is_256 {
        for lane in 0..4 {
            let w32 = (hi >> (lane * 32)) as u32;
            packed |= (f32_to_f16_bits(w32) as u128) << ((lane + 4) * 16);
        }
    }
    packed
}

const CRC32C_TABLE: [u32; 256] = {
    let mut t = [0u32; 256];
    let mut i = 0usize;
    while i < 256 {
        let mut c = i as u32;
        let mut b = 0;
        while b < 8 {
            c = if (c & 1) != 0 { (c >> 1) ^ 0x82F6_3B78 } else { c >> 1 };
            b += 1;
        }
        t[i] = c;
        i += 1;
    }
    t
};

/// SSE4.2 CRC32C step (hardware `crc32c*` when `target_feature = "crc"`, 256-entry LUT elsewhere).
#[inline(always)]
pub(in crate::emu) fn crc32c_step(crc: u32, v: u64, src_bits: u32) -> u32 {
    #[cfg(target_arch = "aarch64")]
    if super::features::host().has(super::features::HostFeatures::CRC32) {
        return unsafe {
            match src_bits {
                8 => __crc32cb(crc, v as u8),
                16 => __crc32ch(crc, v as u16),
                32 => __crc32cw(crc, v as u32),
                _ => __crc32cd(crc, v),
            }
        };
    }
    {
        let mut c = crc;
        let nbytes = src_bits >> 3;
        for i in 0..nbytes {
            let b = ((v >> (8 * i)) as u8) as u32;
            c = CRC32C_TABLE[((c ^ b) & 0xFF) as usize] ^ (c >> 8);
        }
        c
    }
}

/// Shared SSE4.2 `pcmpistri`/`pcmpistrm`/`pcmpestri`/`pcmpestrm` execution (NEON-accelerated on `aarch64`).
#[inline(never)]
pub(in crate::emu) fn pcmpstr(
    cpu: &mut Cpu,
    a: u128,
    b: u128,
    imm: u8,
    is_explicit: bool,
    is_mask: bool,
    long: bool,
) {
    let is_words = (imm & 0x01) != 0;
    let is_signed = (imm & 0x02) != 0;
    let n: usize = if is_words { 8 } else { 16 };
    let agg = (imm >> 2) & 0x03;

    let length = |value: u64| {
        let signed = if long { value as i64 } else { i64::from(value as i32) };
        signed.unsigned_abs().min(n as u64) as usize
    };
    let (a_len, b_len, int_res1) = if !is_words {
        let ab = a.to_le_bytes();
        let bb = b.to_le_bytes();
        #[cfg(target_arch = "aarch64")]
        let (a_len, b_len) = unsafe {
            if is_explicit {
                (
                    length(cpu.gpr[0]),
                    length(cpu.gpr[2]),
                )
            } else {
                let z = vdupq_n_u8(0);
                let za = pmovmskb(bq(vceqq_u8(qb(a), z))) as u32;
                let zb = pmovmskb(bq(vceqq_u8(qb(b), z))) as u32;
                (
                    if za == 0 { 16 } else { za.trailing_zeros() as usize },
                    if zb == 0 { 16 } else { zb.trailing_zeros() as usize },
                )
            }
        };
        #[cfg(not(target_arch = "aarch64"))]
        let (a_len, b_len) = if is_explicit {
            (
                length(cpu.gpr[0]),
                length(cpu.gpr[2]),
            )
        } else {
            (
                ab.iter().position(|&x| x == 0).unwrap_or(16),
                bb.iter().position(|&x| x == 0).unwrap_or(16),
            )
        };
        #[cfg(target_arch = "aarch64")]
        let valid_a: u16 = if a_len >= 16 { 0xFFFF } else { (1u16 << a_len) - 1 };
        #[cfg(target_arch = "aarch64")]
        let valid_b: u16 = if b_len >= 16 { 0xFFFF } else { (1u16 << b_len) - 1 };
        let res1: u16 = match agg {
            0 => {
                // Equal Any
                if a_len == 0 || b_len == 0 {
                    0
                } else {
                    #[cfg(target_arch = "aarch64")]
                    unsafe {
                        let vb = qb(b);
                        let mut acc = vceqq_u8(vb, vdupq_n_u8(ab[0]));
                        for &needle in &ab[1..a_len] {
                            acc = vorrq_u8(acc, vceqq_u8(vb, vdupq_n_u8(needle)));
                        }
                        (pmovmskb(bq(acc)) as u16) & valid_b
                    }
                    #[cfg(not(target_arch = "aarch64"))]
                    {
                        let mut r = 0u16;
                        for (j, &bj) in bb[..b_len].iter().enumerate() {
                            if ab[..a_len].contains(&bj) {
                                r |= 1u16 << j;
                            }
                        }
                        r
                    }
                }
            }
            1 => {
                // Ranges
                if a_len < 2 || b_len == 0 {
                    0
                } else {
                    #[cfg(target_arch = "aarch64")]
                    unsafe {
                        let vb = qb(b);
                        let mut acc = vdupq_n_u8(0);
                        let mut i = 0;
                        if is_signed {
                            let s_vb = vreinterpretq_s8_u8(vb);
                            while i + 1 < a_len {
                                let lo = vdupq_n_s8(ab[i] as i8);
                                let hi = vdupq_n_s8(ab[i + 1] as i8);
                                let in_r = vandq_u8(vcgeq_s8(s_vb, lo), vcleq_s8(s_vb, hi));
                                acc = vorrq_u8(acc, in_r);
                                i += 2;
                            }
                        } else {
                            while i + 1 < a_len {
                                let lo = vdupq_n_u8(ab[i]);
                                let hi = vdupq_n_u8(ab[i + 1]);
                                let in_r = vandq_u8(vcgeq_u8(vb, lo), vcleq_u8(vb, hi));
                                acc = vorrq_u8(acc, in_r);
                                i += 2;
                            }
                        }
                        (pmovmskb(bq(acc)) as u16) & valid_b
                    }
                    #[cfg(not(target_arch = "aarch64"))]
                    {
                        let mut r = 0u16;
                        for (j, &bj) in bb[..b_len].iter().enumerate() {
                            let mut i = 0;
                            while i + 1 < a_len {
                                let ok = if is_signed {
                                    (bj as i8) >= (ab[i] as i8) && (bj as i8) <= (ab[i + 1] as i8)
                                } else {
                                    bj >= ab[i] && bj <= ab[i + 1]
                                };
                                if ok {
                                    r |= 1u16 << j;
                                    break;
                                }
                                i += 2;
                            }
                        }
                        r
                    }
                }
            }
            2 => {
                // Equal Each
                #[cfg(target_arch = "aarch64")]
                unsafe {
                    let eq = pmovmskb(bq(vceqq_u8(qb(a), qb(b)))) as u16;
                    (eq & valid_a & valid_b) | (!valid_a & !valid_b)
                }
                #[cfg(not(target_arch = "aarch64"))]
                {
                    let mut r = 0u16;
                    for j in 0..16 {
                        let m = if j < a_len && j < b_len {
                            ab[j] == bb[j]
                        } else {
                            j >= a_len && j >= b_len
                        };
                        if m {
                            r |= 1u16 << j;
                        }
                    }
                    r
                }
            }
            _ => {
                // Equal Ordered
                let mut r = 0u16;
                for j in 0..16 {
                    let mut m = true;
                    let lim = a_len.min(16 - j);
                    if lim > 0 && j + lim > b_len {
                        m = false;
                    } else {
                        for k in 0..lim {
                            if ab[k] != bb[j + k] {
                                m = false;
                                break;
                            }
                        }
                    }
                    if m {
                        r |= 1u16 << j;
                    }
                }
                r
            }
        };
        (a_len, b_len, res1)
    } else {
        // 16-bit word elements (n = 8)
        let aw: [u16; 8] = core::array::from_fn(|i| ((a >> (i * 16)) & 0xFFFF) as u16);
        let bw: [u16; 8] = core::array::from_fn(|i| ((b >> (i * 16)) & 0xFFFF) as u16);
        let (a_len, b_len) = if is_explicit {
            (
                length(cpu.gpr[0]),
                length(cpu.gpr[2]),
            )
        } else {
            (
                aw.iter().position(|&x| x == 0).unwrap_or(8),
                bw.iter().position(|&x| x == 0).unwrap_or(8),
            )
        };
        let mut res1 = 0u16;
        match agg {
            0 => {
                for (j, &bj) in bw[..b_len].iter().enumerate() {
                    if aw[..a_len].contains(&bj) {
                        res1 |= 1u16 << j;
                    }
                }
            }
            1 => {
                for (j, &bj) in bw[..b_len].iter().enumerate() {
                    let mut i = 0;
                    while i + 1 < a_len {
                        let ok = if is_signed {
                            (bj as i16) >= (aw[i] as i16) && (bj as i16) <= (aw[i + 1] as i16)
                        } else {
                            bj >= aw[i] && bj <= aw[i + 1]
                        };
                        if ok {
                            res1 |= 1u16 << j;
                            break;
                        }
                        i += 2;
                    }
                }
            }
            2 => {
                for j in 0..8 {
                    let m = if j < a_len && j < b_len {
                        aw[j] == bw[j]
                    } else {
                        j >= a_len && j >= b_len
                    };
                    if m {
                        res1 |= 1u16 << j;
                    }
                }
            }
            _ => {
                for j in 0..8 {
                    let mut m = true;
                    let lim = a_len.min(8 - j);
                    if lim > 0 && j + lim > b_len {
                        m = false;
                    } else {
                        for k in 0..lim {
                            if aw[k] != bw[j + k] {
                                m = false;
                                break;
                            }
                        }
                    }
                    if m {
                        res1 |= 1u16 << j;
                    }
                }
            }
        }
        (a_len, b_len, res1)
    };

    let mask_all: u16 = if n == 16 { 0xFFFF } else { 0x00FF };
    let valid_b_mask: u16 = if b_len >= 16 { 0xFFFF } else { (1u16 << b_len) - 1 };
    let int_res2: u16 = match (imm >> 4) & 0x03 {
        1 => (!int_res1) & mask_all,
        3 => (int_res1 ^ valid_b_mask) & mask_all,
        _ => int_res1 & mask_all,
    };
    if is_mask {
        if (imm & 0x40) == 0 {
            cpu.xmm[0] = int_res2 as u128;
        } else if is_words {
            let mut m128 = 0u128;
            for j in 0..8 {
                if (int_res2 & (1u16 << j)) != 0 {
                    m128 |= 0xFFFFu128 << (j * 16);
                }
            }
            cpu.xmm[0] = m128;
        } else {
            let mut m128 = 0u128;
            for j in 0..16 {
                if (int_res2 & (1u16 << j)) != 0 {
                    m128 |= 0xFFu128 << (j * 8);
                }
            }
            cpu.xmm[0] = m128;
        }
        cpu.clear_ymm_hi_reg(0);
    } else {
        let idx = if int_res2 == 0 {
            n as u64
        } else if (imm & 0x40) != 0 {
            (15 - int_res2.leading_zeros()) as u64
        } else {
            int_res2.trailing_zeros() as u64
        };
        cpu.gpr[1] = idx;
        cpu.last_bits[1] = 64;
    }
    cpu.cf = int_res2 != 0;
    cpu.zf = b_len < n;
    cpu.sf = a_len < n;
    cpu.of = (int_res2 & 1) != 0;
    cpu.set_pf(false);
}

#[inline(always)]
fn sha_lanes(v: u128) -> [u32; 4] {
    [v as u32, (v >> 32) as u32, (v >> 64) as u32, (v >> 96) as u32]
}

#[inline(always)]
fn sha_pack(w: [u32; 4]) -> u128 {
    (w[0] as u128)
        | ((w[1] as u128) << 32)
        | ((w[2] as u128) << 64)
        | ((w[3] as u128) << 96)
}

/// Intel SDM `SHA1RNDS4 xmm1, xmm2/m128, imm8`.
pub(in crate::emu) fn sha1rnds4(src1: u128, src2: u128, imm8: u8) -> u128 {
    let [mut d, mut c, mut b, mut a] = sha_lanes(src1);
    let [w3, w2, w1, w0e] = sha_lanes(src2);
    let func = imm8 & 3;
    let k: u32 = match func {
        0 => 0x5A82_7999,
        1 => 0x6ED9_EBA1,
        2 => 0x8F1B_BCDC,
        _ => 0xCA62_C1D6,
    };
    let f = |b: u32, c: u32, d: u32| -> u32 {
        match func {
            0 => (b & c) ^ ((!b) & d),
            1 | 3 => b ^ c ^ d,
            _ => (b & c) ^ (b & d) ^ (c & d),
        }
    };
    // Round 0: W0E already includes E_0.
    let a1 = f(b, c, d).wrapping_add(a.rotate_left(5)).wrapping_add(w0e).wrapping_add(k);
    let mut e = d;
    d = c;
    c = b.rotate_left(30);
    b = a;
    a = a1;
    for w in [w1, w2, w3] {
        let an = f(b, c, d)
            .wrapping_add(a.rotate_left(5))
            .wrapping_add(w)
            .wrapping_add(e)
            .wrapping_add(k);
        e = d;
        d = c;
        c = b.rotate_left(30);
        b = a;
        a = an;
    }
    sha_pack([d, c, b, a])
}

/// Intel SDM `SHA1NEXTE xmm1, xmm2/m128`.
pub(in crate::emu) fn sha1nexte(src1: u128, src2: u128) -> u128 {
    let tmp = ((src1 >> 96) as u32).rotate_left(30);
    let [s0, s1, s2, s3] = sha_lanes(src2);
    sha_pack([s0, s1, s2, s3.wrapping_add(tmp)])
}

/// Intel SDM `SHA1MSG1 xmm1, xmm2/m128`.
pub(in crate::emu) fn sha1msg1(src1: u128, src2: u128) -> u128 {
    let [w3, w2, w1, w0] = sha_lanes(src1);
    let [_, _, w5, w4] = sha_lanes(src2);
    sha_pack([w5 ^ w3, w4 ^ w2, w3 ^ w1, w2 ^ w0])
}

/// Intel SDM `SHA1MSG2 xmm1, xmm2/m128`.
pub(in crate::emu) fn sha1msg2(src1: u128, src2: u128) -> u128 {
    let [s0, s1, s2, s3] = sha_lanes(src1);
    let [w15, w14, w13, _] = sha_lanes(src2);
    let w16 = (s3 ^ w13).rotate_left(1);
    let w17 = (s2 ^ w14).rotate_left(1);
    let w18 = (s1 ^ w15).rotate_left(1);
    let w19 = (s0 ^ w16).rotate_left(1);
    sha_pack([w19, w18, w17, w16])
}

/// Intel SDM `SHA256RNDS2 xmm1, xmm2/m128, <XMM0>`.
pub(in crate::emu) fn sha256rnds2(src1: u128, src2: u128, xmm0: u128) -> u128 {
    let [mut h, mut g, mut d, mut c] = sha_lanes(src1);
    let [mut f, mut e, mut b, mut a] = sha_lanes(src2);
    let wk = [xmm0 as u32, (xmm0 >> 32) as u32];
    for w in wk {
        let ch = (e & f) ^ ((!e) & g);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let t1 = ch.wrapping_add(s1).wrapping_add(w).wrapping_add(h);
        let a_new = t1.wrapping_add(maj).wrapping_add(s0);
        let e_new = t1.wrapping_add(d);
        h = g;
        g = f;
        f = e;
        e = e_new;
        d = c;
        c = b;
        b = a;
        a = a_new;
    }
    sha_pack([f, e, b, a])
}

/// Intel SDM `SHA256MSG1 xmm1, xmm2/m128`.
pub(in crate::emu) fn sha256msg1(src1: u128, src2: u128) -> u128 {
    let [w0, w1, w2, w3] = sha_lanes(src1);
    let w4 = src2 as u32;
    let s0 = |x: u32| x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3);
    sha_pack([
        w0.wrapping_add(s0(w1)),
        w1.wrapping_add(s0(w2)),
        w2.wrapping_add(s0(w3)),
        w3.wrapping_add(s0(w4)),
    ])
}

/// Intel SDM `SHA256MSG2 xmm1, xmm2/m128`.
pub(in crate::emu) fn sha256msg2(src1: u128, src2: u128) -> u128 {
    let [s0, s1, s2, s3] = sha_lanes(src1);
    let [_, _, w14, w15] = sha_lanes(src2);
    let sig1 = |x: u32| x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10);
    let w16 = s0.wrapping_add(sig1(w14));
    let w17 = s1.wrapping_add(sig1(w15));
    let w18 = s2.wrapping_add(sig1(w16));
    let w19 = s3.wrapping_add(sig1(w17));
    sha_pack([w16, w17, w18, w19])
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_x86::Register;

    #[test]
    fn simd_mov_xmm_ymm_o1_index_matches_all_registers() {
        let xmms = [
            Register::XMM0, Register::XMM1, Register::XMM2, Register::XMM3,
            Register::XMM4, Register::XMM5, Register::XMM6, Register::XMM7,
            Register::XMM8, Register::XMM9, Register::XMM10, Register::XMM11,
            Register::XMM12, Register::XMM13, Register::XMM14, Register::XMM15,
        ];
        let ymms = [
            Register::YMM0, Register::YMM1, Register::YMM2, Register::YMM3,
            Register::YMM4, Register::YMM5, Register::YMM6, Register::YMM7,
            Register::YMM8, Register::YMM9, Register::YMM10, Register::YMM11,
            Register::YMM12, Register::YMM13, Register::YMM14, Register::YMM15,
        ];
        for (i, &r) in xmms.iter().enumerate() {
            assert_eq!(xmm_idx_fast(r), Some(i));
            assert!(!is_ymm_fast(r));
            assert_eq!(xmm_reg_info(r), Some((i, false)));
        }
        for (i, &r) in ymms.iter().enumerate() {
            assert_eq!(xmm_idx_fast(r), Some(i));
            assert!(is_ymm_fast(r));
            assert_eq!(xmm_reg_info(r), Some((i, true)));
        }
        for r in [Register::None, Register::RAX, Register::ST0, Register::ZMM0, Register::XMM16, Register::YMM16] {
            assert_eq!(xmm_idx_fast(r), None);
            assert!(!is_ymm_fast(r));
            assert_eq!(xmm_reg_info(r), None);
        }
    }

    #[test]
    fn simd_mov_min_max_ps_pd_nan_and_signed_zero_semantics() {
        // x86 MINPS/MAXPS: returns SECOND operand when either is NaN or -0.0 vs +0.0!
        let p0 = 0.0f32.to_bits();
        let n0 = (-0.0f32).to_bits();
        let qnan1 = 0x7FC0_0011u32;
        let snan2 = 0x7F80_0022u32;
        let a = (n0 as u128) | ((p0 as u128) << 32) | ((qnan1 as u128) << 64) | ((1.5f32.to_bits() as u128) << 96);
        let b = (p0 as u128) | ((n0 as u128) << 32) | ((2.5f32.to_bits() as u128) << 64) | ((snan2 as u128) << 96);

        let min_r = fp_bin_ps(4, a, b);
        assert_eq!(min_r as u32, p0, "minps(-0.0, +0.0) == +0.0 (second op)");
        assert_eq!((min_r >> 32) as u32, n0, "minps(+0.0, -0.0) == -0.0 (second op)");
        assert_eq!((min_r >> 64) as u32, 2.5f32.to_bits(), "minps(qNaN, 2.5) == 2.5 (second op)");
        assert_eq!((min_r >> 96) as u32, snan2, "minps(1.5, sNaN) == exact sNaN bits");

        let max_r = fp_bin_ps(5, a, b);
        assert_eq!(max_r as u32, p0, "maxps(-0.0, +0.0) == +0.0 (second op)");
        assert_eq!((max_r >> 32) as u32, n0, "maxps(+0.0, -0.0) == -0.0 (second op)");
        assert_eq!((max_r >> 64) as u32, 2.5f32.to_bits(), "maxps(qNaN, 2.5) == 2.5 (second op)");
        assert_eq!((max_r >> 96) as u32, snan2, "maxps(1.5, sNaN) == exact sNaN bits");

        // f64 counterpart (`minpd` / `maxpd`)
        let dp0 = 0.0f64.to_bits();
        let dn0 = (-0.0f64).to_bits();
        let dqnan = 0x7FF8_0000_0000_00AAu64;
        let da = (dn0 as u128) | ((dqnan as u128) << 64);
        let db = (dp0 as u128) | ((3.5f64.to_bits() as u128) << 64);
        let dmin = fp_bin_pd(4, da, db);
        let dmax = fp_bin_pd(5, da, db);
        assert_eq!(dmin as u64, dp0);
        assert_eq!((dmin >> 64) as u64, 3.5f64.to_bits());
        assert_eq!(dmax as u64, dp0);
        assert_eq!((dmax >> 64) as u64, 3.5f64.to_bits());
    }

    #[test]
    fn simd_mov_fp_nan_indefinite_and_first_operand_priority() {
        // Invalid ops (0/0, Inf-Inf, sqrt(-1)) MUST yield negative x86 real indefinite (`0xFFC0_0000` / `0xFFF8_0000_0000_0000`).
        let z = 0.0f32.to_bits() as u128;
        let qnan_a = 0x7FC0_0011u32 as u128;
        let snan_b = 0x7F80_0022u32 as u128; // ARM64 normally prioritizes sNaN over qNaN; x86 MUST pick first operand!
        let neg1 = (-1.0f32).to_bits() as u128;
        let a = z | (qnan_a << 32) | (1.0f32.to_bits() as u128) << 64 | (f32::INFINITY.to_bits() as u128) << 96;
        let b = z | (snan_b << 32) | (snan_b << 64) | (f32::INFINITY.to_bits() as u128) << 96;

        let div_r = fp_bin_ps(3, a, b);
        assert_eq!(div_r as u32, 0xFFC0_0000, "0.0 / 0.0 -> x86 negative indefinite");
        assert_eq!((div_r >> 32) as u32, 0x7FC0_0011, "qNaN_a / sNaN_b -> first operand a wins");
        assert_eq!((div_r >> 64) as u32, 0x7FC0_0022, "1.0 / sNaN_b -> quieted b");
        assert_eq!((div_r >> 96) as u32, 0xFFC0_0000, "Inf / Inf -> x86 negative indefinite");

        let sqrt_r = sqrt_ps(neg1 | (snan_b << 32) | (4.0f32.to_bits() as u128) << 64);
        assert_eq!(sqrt_r as u32, 0xFFC0_0000, "sqrt(-1.0) -> x86 negative indefinite");
        assert_eq!((sqrt_r >> 32) as u32, 0x7FC0_0022, "sqrt(sNaN) -> quieted sNaN");
        assert_eq!((sqrt_r >> 64) as u32, 2.0f32.to_bits());

        let dz = 0.0f64.to_bits() as u128;
        let dq = 0x7FF8_0000_0000_0011u64 as u128;
        let ds = 0x7FF0_0000_0000_0022u64 as u128;
        let dsub = fp_bin_pd(1, (f64::INFINITY.to_bits() as u128) | (dq << 64), (f64::INFINITY.to_bits() as u128) | (ds << 64));
        assert_eq!(dsub as u64, 0xFFF8_0000_0000_0000, "Inf - Inf -> f64 negative indefinite");
        assert_eq!((dsub >> 64) as u64, 0x7FF8_0000_0000_0011, "f64 first NaN wins over sNaN");
        let dsqrt = sqrt_pd(((-4.0f64).to_bits() as u128) | (dz << 64));
        assert_eq!(dsqrt as u64, 0xFFF8_0000_0000_0000);
        assert_eq!((dsqrt >> 64) as u64, 0);
    }

    #[test]
    fn simd_mov_cvtps2dq_cvttps2dq_indefinite_and_ties_to_even() {
        let pack4 = |f: [f32; 4]| -> u128 {
            (f[0].to_bits() as u128)
                | ((f[1].to_bits() as u128) << 32)
                | ((f[2].to_bits() as u128) << 64)
                | ((f[3].to_bits() as u128) << 96)
        };
        let lanes = |v: u128| -> [i32; 4] {
            [v as i32, (v >> 32) as i32, (v >> 64) as i32, (v >> 96) as i32]
        };

        // NaN, +2^31 (overflow!), -2^31 (exact min), -2^31 - 256 (underflow)
        let edge = pack4([f32::NAN, 2147483648.0, -2147483648.0, -2147483904.0]);
        assert_eq!(lanes(cvtps2dq(edge, true)), [i32::MIN, i32::MIN, i32::MIN, i32::MIN]);
        assert_eq!(lanes(cvtps2dq(edge, false)), [i32::MIN, i32::MIN, i32::MIN, i32::MIN]);

        // +Inf, -Inf, 2.5, 3.5 -> round-ties-to-even vs truncate
        let round = pack4([f32::INFINITY, f32::NEG_INFINITY, 2.5, -3.5]);
        assert_eq!(lanes(cvtps2dq(round, true)), [i32::MIN, i32::MIN, 2, -3]);
        assert_eq!(lanes(cvtps2dq(round, false)), [i32::MIN, i32::MIN, 2, -4]);
    }

    #[test]
    fn simd_mov_pmovmskb_and_pshufb_exact_bit_and_byte_order() {
        // Verify each individual bit 0..31 of pmovmskb / pmovmskb_ymm.
        for bit in 0..16u32 {
            let v = 0x80u128 << (bit * 8);
            assert_eq!(pmovmskb(v), 1u64 << bit, "pmovmskb bit {bit}");
            assert_eq!(pmovmskb_ymm(v, 0), 1u64 << bit, "pmovmskb_ymm lo bit {bit}");
            assert_eq!(pmovmskb_ymm(0, v), 1u64 << (bit + 16), "pmovmskb_ymm hi bit {bit}");
        }
        // Non-MSB bits (0x7F) must NEVER set mask bits.
        assert_eq!(pmovmskb(u128::from_le_bytes([0x7F; 16])), 0);
        assert_eq!(pmovmskb(u128::from_le_bytes([0x80; 16])), 0xFFFF);
        assert_eq!(
            pmovmskb_ymm(
                u128::from_le_bytes([0x80, 0x7F, 0xFF, 0x00, 0x81, 0x01, 0x00, 0x80, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80]),
                u128::from_le_bytes([0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF]),
            ),
            0x8001_8295
        );

        // PSHUFB: bit 7 clears lane; bits 6:4 are ignored when bit 7 is 0!
        let src = u128::from_le_bytes([
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
        ]);
        let ctl = u128::from_le_bytes([
            0x0F, 0x80, 0x70, 0xFF, 0x15, 0x9F, 0x00, 0x7F,
            0x81, 0x33, 0x22, 0x8F, 0x44, 0x55, 0xFE, 0x0A,
        ]);
        let want = u128::from_le_bytes([
            0x1F, 0x00, 0x10, 0x00, 0x15, 0x00, 0x10, 0x1F,
            0x00, 0x13, 0x12, 0x00, 0x14, 0x15, 0x00, 0x1A,
        ]);
        assert_eq!(pshufb(src, ctl), want);
    }

    #[test]
    fn simd_mov_pabs_pmulhrsw_saturation_and_pack_edges() {
        // PABSB/W/D with MIN values MUST wrap (not saturate!).
        let min_b = u128::from_le_bytes([0x80, 0x81, 0xFF, 0x00, 0x7F, 0x80, 0x01, 0xFE, 0x80, 0, 0, 0, 0, 0, 0, 0]);
        let got_b = pabsb(min_b).to_le_bytes();
        assert_eq!(&got_b[..8], &[0x80, 0x7F, 0x01, 0x00, 0x7F, 0x80, 0x01, 0x02]);

        let min_w = (i16::MIN as u16 as u128) | ((i16::MAX as u16 as u128) << 16) | (((-1i16) as u16 as u128) << 32);
        let got_w = pabsw(min_w);
        assert_eq!(got_w as u16, 0x8000);
        assert_eq!((got_w >> 16) as u16, 0x7FFF);
        assert_eq!((got_w >> 32) as u16, 1);

        let min_d = (i32::MIN as u32 as u128) | (((-42i32) as u32 as u128) << 32);
        let got_d = pabsd(min_d);
        assert_eq!(got_d as u32, 0x8000_0000);
        assert_eq!((got_d >> 32) as u32, 42);

        // PMULHRSW(-32768, -32768) MUST wrap to -32768 (0x8000), NOT saturate to 0x7FFF!
        let m16 = (i16::MIN as u16 as u128) | ((16384u16 as u128) << 16) | (((-16384i16) as u16 as u128) << 32);
        let r_hrsw = pmulhrsw(m16, m16);
        assert_eq!(r_hrsw as u16, 0x8000, "pmulhrsw(i16::MIN, i16::MIN) == 0x8000");
        assert_eq!((r_hrsw >> 16) as u16, 8192);
        assert_eq!((r_hrsw >> 32) as u16, 8192);

        // Saturating add/sub & pack edges
        let a_sb = u128::from_le_bytes([127, 128, 100, 200, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let b_sb = u128::from_le_bytes([10, 255, 50, 150, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(&paddsb(a_sb, b_sb).to_le_bytes()[..4], &[127, 128, 127, 128]);
        assert_eq!(&paddusb(a_sb, b_sb).to_le_bytes()[..4], &[137, 255, 150, 255]);
        assert_eq!(&psubusb(a_sb, b_sb).to_le_bytes()[..4], &[117, 0, 50, 50]);

        // PACKSSWB / PACKUSWB / PACKSSDW / PACKUSDW
        let w_a: u128 = ((-300i16) as u16 as u128)
            | ((300u16 as u128) << 16)
            | (((-128i16) as u16 as u128) << 32)
            | ((127u16 as u128) << 48);
        let w_b: u128 = (255u16 as u128) | (((-1i16) as u16 as u128) << 16);
        let pss = packsswb(w_a, w_b).to_le_bytes();
        assert_eq!(&pss[0..4], &[0x80, 0x7F, 0x80, 0x7F]);
        assert_eq!(&pss[8..10], &[0x7F, 0xFF]);
        let pus = packuswb(w_a, w_b).to_le_bytes();
        assert_eq!(&pus[0..4], &[0x00, 0xFF, 0x00, 0x7F]);
        assert_eq!(&pus[8..10], &[0xFF, 0x00]);
    }
}
