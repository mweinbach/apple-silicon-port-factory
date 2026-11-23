//! SSE/AVX data movement, shuffle, unpack, blend, and integer SIMD execution for [`super::Cpu`].

use super::*;

/// Vectorized 128-bit/256-bit SIMD execution kernels (ARM64 NEON + portable fallbacks) and O(1)
/// XMM/YMM register indexing shared between [`super::simd_mov`] and [`super::simd_fp`].
pub(super) mod vops {
    use super::super::Cpu;
    #[cfg(not(target_arch = "aarch64"))]
    use super::super::{sse_arith32, sse_arith64, sse_cmp_pred, sse_cvt_i32, sse_sqrt32, sse_sqrt64, x86_nan32, x86_nan64};
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
                let r = match imm & 3 {
                    0 => f.round_ties_even(),
                    1 => f.floor(),
                    2 => f.ceil(),
                    _ => f.trunc(),
                };
                out |= (r.to_bits() as u128) << (lane * 32);
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
                let r = match imm & 3 {
                    0 => f.round_ties_even(),
                    1 => f.floor(),
                    2 => f.ceil(),
                    _ => f.trunc(),
                };
                out |= (r.to_bits() as u128) << (lane * 64);
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
            let idx = super::super::ymm_dwords(i_lo, i_hi);
            let tab = super::super::ymm_dwords(t_lo, t_hi);
            let r: [u32; 8] = core::array::from_fn(|i| tab[(idx[i] & 7) as usize]);
            super::super::dwords_ymm(&r)
        }
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
}

impl Cpu {
    #[inline]
    pub(super) unsafe fn exec_simd_mov_int(&mut self, mn: Mnemonic, ins_ref: &Instruction) -> bool {
        let ins = ins_ref;
        let cpu = self;
        match mn {
            Mnemonic::Movups | Mnemonic::Movaps | Mnemonic::Movdqu | Mnemonic::Movdqa | Mnemonic::Movapd | Mnemonic::Movupd
            | Mnemonic::Movntdq | Mnemonic::Movntps | Mnemonic::Movntpd | Mnemonic::Movntdqa
            | Mnemonic::Vmovups | Mnemonic::Vmovaps | Mnemonic::Vmovdqu | Mnemonic::Vmovdqa | Mnemonic::Vmovapd | Mnemonic::Vmovupd
            | Mnemonic::Vmovntdq | Mnemonic::Vmovntps | Mnemonic::Vmovntpd | Mnemonic::Vmovntdqa
            | Mnemonic::Lddqu | Mnemonic::Vlddqu => {
                match (ins.op_kind(0), ins.op_kind(1)) {
                    (OpKind::Register, OpKind::Register) => {
                        if let (Some((d, ymm)), Some(s)) =
                            (vops::xmm_reg_info(ins.op_register(0)), vops::xmm_idx_fast(ins.op_register(1)))
                        {
                            let lo = cpu.xmm[s];
                            let hi = if ymm { cpu.ymm_hi[s] } else { 0 };
                            vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                        }
                    }
                    (OpKind::Register, OpKind::Memory) => {
                        if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                            let a = cpu.eff_addr(ins);
                            let lo = cpu.read_u128(a);
                            let hi = if ymm { cpu.read_u128(a.wrapping_add(16)) } else { 0 };
                            vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                        }
                    }
                    (OpKind::Memory, OpKind::Register) => {
                        if let Some((s, ymm)) = vops::xmm_reg_info(ins.op_register(1)) {
                            let a = cpu.eff_addr(ins);
                            cpu.write_u128(a, cpu.xmm[s]);
                            if ymm {
                                cpu.write_u128(a.wrapping_add(16), cpu.ymm_hi[s]);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Mnemonic::Shrd | Mnemonic::Shld => {
                let bits = Cpu::op_bits(ins, 0);
                let dst = cpu.read_op(ins, 0, bits);
                let src = cpu.read_op(ins, 1, bits);
                let count_mask = if bits == 64 { 63u32 } else { 31u32 };
                let cnt = (cpu.read_op(ins, 2, 8) as u32) & count_mask;
                if cnt != 0 {
                    let res = if mn == Mnemonic::Shrd {
                        if bits == 64 {
                            (dst >> cnt) | (src << (64 - cnt))
                        } else if bits == 32 {
                            (((dst as u32) >> cnt) | ((src as u32) << (32 - cnt))) as u64
                        } else {
                            let d16 = dst as u16;
                            let s16 = src as u16;
                            ((d16 >> cnt) | (s16 << (16 - cnt))) as u64
                        }
                    } else if bits == 64 {
                        (dst << cnt) | (src >> (64 - cnt))
                    } else if bits == 32 {
                        (((dst as u32) << cnt) | ((src as u32) >> (32 - cnt))) as u64
                    } else {
                        let d16 = dst as u16;
                        let s16 = src as u16;
                        ((d16 << cnt) | (s16 >> (16 - cnt))) as u64
                    };
                    cpu.update_flags_logic(res, bits);
                    cpu.cf = if mn == Mnemonic::Shrd {
                        ((dst >> (cnt - 1)) & 1) != 0
                    } else {
                        ((dst >> (bits - cnt)) & 1) != 0
                    };
                    cpu.write_op(ins, 0, bits, res);
                }
            }
            Mnemonic::Vperm2f128 | Mnemonic::Vperm2i128 => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (s1_lo, s1_hi) = vops::load_vec_reg(cpu, ins.op_register(1), true);
                    let (s2_lo, s2_hi) = vops::load_vec_op(cpu, ins, 2, true);
                    let imm = cpu.read_op(ins, 3, 8) as u8;
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
                    cpu.xmm[d] = pick(imm & 0xF);
                    cpu.set_ymm_hi(d, pick((imm >> 4) & 0xF));
                }
            }
            Mnemonic::Vinsertf128 | Mnemonic::Vinserti128 => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let s1 = vops::xmm_idx_fast(ins.op_register(1)).unwrap_or(d);
                    let (inserted, _) = vops::load_vec_op(cpu, ins, 2, false);
                    let imm = cpu.read_op(ins, 3, 8) & 1;
                    let lo = cpu.xmm[s1];
                    let hi = cpu.ymm_hi[s1];
                    if imm == 0 {
                        cpu.xmm[d] = inserted;
                        cpu.set_ymm_hi(d, hi);
                    } else {
                        cpu.xmm[d] = lo;
                        cpu.set_ymm_hi(d, inserted);
                    }
                }
            }
            Mnemonic::Vextractf128 | Mnemonic::Vextracti128 => {
                let s = vops::xmm_idx_fast(ins.op_register(1)).unwrap_or(0);
                let imm = cpu.read_op(ins, 2, 8) & 1;
                let val = if imm == 0 { cpu.xmm[s] } else { cpu.ymm_hi[s] };
                match ins.op_kind(0) {
                    OpKind::Register => {
                        if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                            cpu.xmm[d] = val;
                            cpu.clear_ymm_hi_reg(d);
                        }
                    }
                    OpKind::Memory => {
                        cpu.write_u128(cpu.eff_addr(ins), val);
                    }
                    _ => {}
                }
            }
            Mnemonic::Pextrb | Mnemonic::Vpextrb | Mnemonic::Pextrw | Mnemonic::Vpextrw
            | Mnemonic::Pextrd | Mnemonic::Vpextrd | Mnemonic::Pextrq | Mnemonic::Vpextrq => {
                let elem_bits: u32 = match mn {
                    Mnemonic::Pextrb | Mnemonic::Vpextrb => 8,
                    Mnemonic::Pextrw | Mnemonic::Vpextrw => 16,
                    Mnemonic::Pextrd | Mnemonic::Vpextrd => 32,
                    _ => 64,
                };
                let s = vops::xmm_idx_fast(ins.op_register(1)).unwrap_or(0);
                let max_lanes = 128 / elem_bits;
                let lane = (cpu.read_op(ins, 2, 8) as u32) & (max_lanes - 1);
                let mask = if elem_bits >= 64 { u64::MAX as u128 } else { (1u128 << elem_bits) - 1 };
                let val = ((cpu.xmm[s] >> (lane * elem_bits)) & mask) as u64;
                match ins.op_kind(0) {
                    OpKind::Register => {
                        let dst_bits = if elem_bits == 64 { 64 } else { 32 };
                        cpu.write_op(ins, 0, dst_bits, val);
                    }
                    OpKind::Memory => {
                        cpu.write_mem(cpu.eff_addr(ins), elem_bits, val);
                    }
                    _ => {}
                }
            }
            Mnemonic::Pinsrb | Mnemonic::Vpinsrb | Mnemonic::Pinsrw | Mnemonic::Vpinsrw
            | Mnemonic::Pinsrd | Mnemonic::Vpinsrd | Mnemonic::Pinsrq | Mnemonic::Vpinsrq => {
                let elem_bits: u32 = match mn {
                    Mnemonic::Pinsrb | Mnemonic::Vpinsrb => 8,
                    Mnemonic::Pinsrw | Mnemonic::Vpinsrw => 16,
                    Mnemonic::Pinsrd | Mnemonic::Vpinsrd => 32,
                    _ => 64,
                };
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (base_op, src_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let base_val = vops::xmm_idx_fast(ins.op_register(base_op)).map_or(0, |i| cpu.xmm[i]);
                    let src_val = cpu.read_op(ins, src_op, elem_bits);
                    let max_lanes = 128 / elem_bits;
                    let lane = (cpu.read_op(ins, imm_op, 8) as u32) & (max_lanes - 1);
                    let mask = if elem_bits >= 64 { u64::MAX as u128 } else { (1u128 << elem_bits) - 1 };
                    let shift = lane * elem_bits;
                    cpu.xmm[d] = (base_val & !(mask << shift)) | (((src_val as u128) & mask) << shift);
                    cpu.vex128_zero_upper(ins, d);
                }
            }
            Mnemonic::Movq | Mnemonic::Vmovq | Mnemonic::Movd | Mnemonic::Vmovd => {
                let is_64 = matches!(mn, Mnemonic::Movq | Mnemonic::Vmovq);
                match (ins.op_kind(0), ins.op_kind(1)) {
                    (OpKind::Register, OpKind::Register) => {
                        match (vops::xmm_idx_fast(ins.op_register(0)), vops::xmm_idx_fast(ins.op_register(1))) {
                            (Some(d), Some(s)) => {
                                cpu.xmm[d] = if is_64 {
                                    cpu.xmm[s] & 0xFFFF_FFFF_FFFF_FFFF
                                } else {
                                    cpu.xmm[s] & 0xFFFF_FFFF
                                };
                            }
                            (Some(d), None) => {
                                let v = cpu.get_reg(ins.op_register(1));
                                cpu.xmm[d] = if is_64 { v as u128 } else { (v as u32) as u128 };
                            }
                            (None, Some(s)) => {
                                let v = if is_64 { cpu.xmm[s] as u64 } else { (cpu.xmm[s] as u32) as u64 };
                                cpu.set_reg(ins.op_register(0), v);
                            }
                            _ => {}
                        }
                    }
                    (OpKind::Register, OpKind::Memory) => {
                        if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                            let a = cpu.eff_addr(ins);
                            cpu.xmm[d] = cpu.read_mem(a, if is_64 { 64 } else { 32 }) as u128;
                        }
                    }
                    (OpKind::Memory, OpKind::Register) => {
                        if let Some(s) = vops::xmm_idx_fast(ins.op_register(1)) {
                            let a = cpu.eff_addr(ins);
                            if is_64 {
                                cpu.write_mem(a, 64, cpu.xmm[s] as u64);
                            } else {
                                cpu.write_mem(a, 32, cpu.xmm[s] as u32 as u64);
                            }
                        }
                    }
                    _ => {}
                }
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    cpu.vex128_zero_upper(ins, d);
                }
            }
            Mnemonic::Stmxcsr | Mnemonic::Vstmxcsr => {
                let a = cpu.eff_addr(ins);
                cpu.write_mem(a, 32, cpu.jx.mxcsr as u64);
            }
            Mnemonic::Ldmxcsr | Mnemonic::Vldmxcsr => {
                let a = cpu.eff_addr(ins);
                let m = (cpu.read_mem(a, 32) as u32) & 0xFFFF;
                cpu.jx.mxcsr = m;
                super::apply_guest_mxcsr_hw(m);
            }
            Mnemonic::Scasb | Mnemonic::Scasw | Mnemonic::Scasd | Mnemonic::Scasq => {
                let (bits, step) = match mn {
                    Mnemonic::Scasb => (8u32, 1u64),
                    Mnemonic::Scasw => (16, 2),
                    Mnemonic::Scasd => (32, 4),
                    _ => (64, 8),
                };
                let step = if cpu.df { step.wrapping_neg() } else { step };
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                let acc = cpu.gpr[0] & mask;
                if ins.has_repne_prefix() {
                    while cpu.gpr[1] != 0 {
                        let m = cpu.read_mem(cpu.gpr[7], bits) & mask;
                        cpu.gpr[7] = cpu.gpr[7].wrapping_add(step);
                        cpu.gpr[1] = cpu.gpr[1].wrapping_sub(1);
                        let _ = cpu.update_flags_sub(acc, m, bits);
                        if acc == m {
                            break;
                        }
                    }
                } else if ins.has_repe_prefix() {
                    while cpu.gpr[1] != 0 {
                        let m = cpu.read_mem(cpu.gpr[7], bits) & mask;
                        cpu.gpr[7] = cpu.gpr[7].wrapping_add(step);
                        cpu.gpr[1] = cpu.gpr[1].wrapping_sub(1);
                        let _ = cpu.update_flags_sub(acc, m, bits);
                        if acc != m {
                            break;
                        }
                    }
                } else {
                    let m = cpu.read_mem(cpu.gpr[7], bits) & mask;
                    cpu.gpr[7] = cpu.gpr[7].wrapping_add(step);
                    let _ = cpu.update_flags_sub(acc, m, bits);
                }
                cpu.last_bits[7] = 64;
                if ins.has_repne_prefix() || ins.has_repe_prefix() {
                    cpu.last_bits[1] = 64;
                }
            }
            Mnemonic::Movlhps | Mnemonic::Vmovlhps => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (s1_idx, s2_op) = if ins.op_count() >= 3 {
                        (vops::xmm_idx_fast(ins.op_register(1)).unwrap_or(d), 2)
                    } else {
                        (d, 1)
                    };
                    let s2_lo = vops::xmm_idx_fast(ins.op_register(s2_op)).map_or(0, |i| cpu.xmm[i] & 0xFFFF_FFFF_FFFF_FFFF);
                    cpu.xmm[d] = (cpu.xmm[s1_idx] & 0xFFFF_FFFF_FFFF_FFFF) | (s2_lo << 64);
                    if mn == Mnemonic::Vmovlhps {
                        cpu.clear_ymm_hi_reg(d);
                    }
                }
            }
            Mnemonic::Movhlps | Mnemonic::Vmovhlps => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (s1_idx, s2_op) = if ins.op_count() >= 3 {
                        (vops::xmm_idx_fast(ins.op_register(1)).unwrap_or(d), 2)
                    } else {
                        (d, 1)
                    };
                    let s2_hi = vops::xmm_idx_fast(ins.op_register(s2_op)).map_or(0, |i| cpu.xmm[i] >> 64);
                    cpu.xmm[d] = (cpu.xmm[s1_idx] & !0xFFFF_FFFF_FFFF_FFFFu128) | s2_hi;
                    if mn == Mnemonic::Vmovhlps {
                        cpu.clear_ymm_hi_reg(d);
                    }
                }
            }
            Mnemonic::Movss | Mnemonic::Vmovss => {
                match (ins.op_kind(0), ins.op_kind(1)) {
                    (OpKind::Register, OpKind::Register) => {
                        let s_reg = ins.op_register(ins.op_count() - 1);
                        if let (Some(d), Some(s)) = (vops::xmm_idx_fast(ins.op_register(0)), vops::xmm_idx_fast(s_reg)) {
                            let hi = if ins.op_count() >= 3 {
                                vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i])
                            } else {
                                cpu.xmm[d]
                            };
                            cpu.xmm[d] = (hi & !0xFFFF_FFFF) | (cpu.xmm[s] & 0xFFFF_FFFF);
                            cpu.vex128_zero_upper(ins, d);
                        }
                    }
                    (OpKind::Register, OpKind::Memory) => {
                        if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                            let a = cpu.eff_addr(ins);
                            cpu.xmm[d] = cpu.read_mem(a, 32) as u128;
                            cpu.vex128_zero_upper(ins, d);
                        }
                    }
                    (OpKind::Memory, OpKind::Register) => {
                        if let Some(s) = vops::xmm_idx_fast(ins.op_register(1)) {
                            let a = cpu.eff_addr(ins);
                            cpu.write_mem(a, 32, cpu.xmm[s] as u64);
                        }
                    }
                    _ => {}
                }
            }
            Mnemonic::Vmovsd => {
                match (ins.op_kind(0), ins.op_kind(1)) {
                    (OpKind::Register, OpKind::Register) => {
                        let s_reg = ins.op_register(ins.op_count() - 1);
                        if let (Some(d), Some(s)) = (vops::xmm_idx_fast(ins.op_register(0)), vops::xmm_idx_fast(s_reg)) {
                            let hi = if ins.op_count() >= 3 {
                                vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i])
                            } else {
                                cpu.xmm[d]
                            };
                            cpu.xmm[d] = (hi & !0xFFFF_FFFF_FFFF_FFFF) | (cpu.xmm[s] & 0xFFFF_FFFF_FFFF_FFFF);
                            cpu.clear_ymm_hi_reg(d);
                        }
                    }
                    (OpKind::Register, OpKind::Memory) => {
                        if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                            let a = cpu.eff_addr(ins);
                            cpu.xmm[d] = cpu.read_mem(a, 64) as u128;
                            cpu.clear_ymm_hi_reg(d);
                        }
                    }
                    (OpKind::Memory, OpKind::Register) => {
                        if let Some(s) = vops::xmm_idx_fast(ins.op_register(1)) {
                            let a = cpu.eff_addr(ins);
                            cpu.write_mem(a, 64, cpu.xmm[s] as u64);
                        }
                    }
                    _ => {}
                }
            }
            Mnemonic::Andn => {
                let bits = Cpu::op_bits(ins, 0);
                let a = cpu.read_op(ins, 1, bits);
                let b = cpu.read_op(ins, 2, bits);
                let res = (!a) & b;
                cpu.write_op(ins, 0, bits, res);
                cpu.update_flags_logic(res, bits);
            }
            Mnemonic::Shrx | Mnemonic::Shlx | Mnemonic::Sarx => {
                let bits = Cpu::op_bits(ins, 0);
                let a = cpu.read_op(ins, 1, bits);
                let mask = if bits == 64 { 63 } else { 31 };
                let cnt = (cpu.read_op(ins, 2, bits) as u32) & mask;
                let res = match mn {
                    Mnemonic::Shlx => a.wrapping_shl(cnt),
                    Mnemonic::Shrx => {
                        let val_mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                        (a & val_mask).wrapping_shr(cnt)
                    }
                    _ => {
                        if bits == 64 {
                            ((a as i64) >> cnt) as u64
                        } else {
                            ((a as i32) >> cnt) as u32 as u64
                        }
                    }
                };
                cpu.write_op(ins, 0, bits, res);
            }
            Mnemonic::Rorx => {
                let bits = Cpu::op_bits(ins, 0);
                let a = cpu.read_op(ins, 1, bits);
                let mask = if bits == 64 { 63 } else { 31 };
                let cnt = (cpu.read_op(ins, 2, 8) as u32) & mask;
                let res = if bits == 64 {
                    a.rotate_right(cnt)
                } else {
                    (a as u32).rotate_right(cnt) as u64
                };
                cpu.write_op(ins, 0, bits, res);
            }
            Mnemonic::Bzhi => {
                let bits = Cpu::op_bits(ins, 0);
                let a = cpu.read_op(ins, 1, bits);
                let idx = (cpu.read_op(ins, 2, bits) & 0xFF) as u32;
                let res = if idx >= bits {
                    cpu.cf = true;
                    a
                } else {
                    cpu.cf = false;
                    if idx == 0 { 0 } else { a & ((1u64 << idx) - 1) }
                };
                cpu.write_op(ins, 0, bits, res);
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                cpu.zf = (res & mask) == 0;
                cpu.sf = (res & (1u64 << (bits - 1))) != 0;
                cpu.of = false;
            }
            Mnemonic::Blsr | Mnemonic::Blsmsk | Mnemonic::Blsi => {
                let bits = Cpu::op_bits(ins, 0);
                let a = cpu.read_op(ins, 1, bits);
                let res = match mn {
                    Mnemonic::Blsr => a & a.wrapping_sub(1),
                    Mnemonic::Blsmsk => a ^ a.wrapping_sub(1),
                    _ => a & a.wrapping_neg(),
                };
                cpu.write_op(ins, 0, bits, res);
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                cpu.zf = (res & mask) == 0;
                cpu.sf = (res & (1u64 << (bits - 1))) != 0;
                cpu.cf = if mn == Mnemonic::Blsi { (a & mask) != 0 } else { (a & mask) == 0 };
                cpu.of = false;
            }
            Mnemonic::Mulx => {
                let bits = Cpu::op_bits(ins, 0);
                let src = cpu.read_op(ins, 2, bits);
                if bits == 64 {
                    let prod = (cpu.gpr[2] as u128).wrapping_mul(src as u128);
                    cpu.write_op(ins, 1, 64, prod as u64);
                    cpu.write_op(ins, 0, 64, (prod >> 64) as u64);
                } else {
                    let prod = (cpu.gpr[2] as u32 as u64).wrapping_mul(src as u32 as u64);
                    cpu.write_op(ins, 1, 32, prod as u32 as u64);
                    cpu.write_op(ins, 0, 32, (prod >> 32) as u32 as u64);
                }
            }
            Mnemonic::Addss | Mnemonic::Subss | Mnemonic::Mulss | Mnemonic::Divss | Mnemonic::Minss | Mnemonic::Maxss
            | Mnemonic::Vaddss | Mnemonic::Vsubss | Mnemonic::Vmulss | Mnemonic::Vdivss | Mnemonic::Vminss | Mnemonic::Vmaxss
            | Mnemonic::Addsd | Mnemonic::Subsd | Mnemonic::Mulsd | Mnemonic::Divsd | Mnemonic::Minsd | Mnemonic::Maxsd
            | Mnemonic::Vaddsd | Mnemonic::Vsubsd | Mnemonic::Vmulsd | Mnemonic::Vdivsd | Mnemonic::Vminsd | Mnemonic::Vmaxsd => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let a = vops::xmm_idx_fast(ins.op_register(s1_op)).map_or(0, |i| cpu.xmm[i]);
                    let (b, _) = cpu.vsrc(ins, s2_op, false);
                    let op: u8 = match mn {
                        Mnemonic::Addss | Mnemonic::Vaddss | Mnemonic::Addsd | Mnemonic::Vaddsd => 0,
                        Mnemonic::Subss | Mnemonic::Vsubss | Mnemonic::Subsd | Mnemonic::Vsubsd => 1,
                        Mnemonic::Mulss | Mnemonic::Vmulss | Mnemonic::Mulsd | Mnemonic::Vmulsd => 2,
                        Mnemonic::Divss | Mnemonic::Vdivss | Mnemonic::Divsd | Mnemonic::Vdivsd => 3,
                        Mnemonic::Minss | Mnemonic::Vminss | Mnemonic::Minsd | Mnemonic::Vminsd => 4,
                        _ => 5,
                    };
                    let is_sd = matches!(
                        mn,
                        Mnemonic::Addsd | Mnemonic::Subsd | Mnemonic::Mulsd | Mnemonic::Divsd | Mnemonic::Minsd | Mnemonic::Maxsd
                            | Mnemonic::Vaddsd | Mnemonic::Vsubsd | Mnemonic::Vmulsd | Mnemonic::Vdivsd | Mnemonic::Vminsd
                            | Mnemonic::Vmaxsd
                    );
                    cpu.xmm[d] = if is_sd {
                        let r = sse_arith64(op, f64::from_bits(a as u64), f64::from_bits(b as u64));
                        (a & !0xFFFF_FFFF_FFFF_FFFF) | (r.to_bits() as u128)
                    } else {
                        let r = sse_arith32(op, f32::from_bits(a as u32), f32::from_bits(b as u32));
                        (a & !0xFFFF_FFFF) | (r.to_bits() as u128)
                    };
                    cpu.vex128_zero_upper(ins, d);
                }
            }
            Mnemonic::Sqrtss | Mnemonic::Vsqrtss | Mnemonic::Sqrtsd | Mnemonic::Vsqrtsd => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let src_op = ins.op_count() - 1;
                    let base = if ins.op_count() >= 3 {
                        vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i])
                    } else {
                        cpu.xmm[d]
                    };
                    let (b, _) = cpu.vsrc(ins, src_op, false);
                    cpu.xmm[d] = if matches!(mn, Mnemonic::Sqrtsd | Mnemonic::Vsqrtsd) {
                        (base & !0xFFFF_FFFF_FFFF_FFFF) | (sse_sqrt64(f64::from_bits(b as u64)).to_bits() as u128)
                    } else {
                        (base & !0xFFFF_FFFF) | (sse_sqrt32(f32::from_bits(b as u32)).to_bits() as u128)
                    };
                    cpu.vex128_zero_upper(ins, d);
                }
            }
            Mnemonic::Comiss | Mnemonic::Ucomiss | Mnemonic::Vcomiss | Mnemonic::Vucomiss => {
                let a_bits = vops::xmm_idx_fast(ins.op_register(0)).map_or(0, |i| cpu.xmm[i] as u32);
                let b_bits = match ins.op_kind(1) {
                    OpKind::Register => vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i] as u32),
                    OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), 32) as u32,
                    _ => 0,
                };
                let fa = f32::from_bits(a_bits);
                let fb = f32::from_bits(b_bits);
                let unord = !(fa <= fb) & !(fa > fb);
                cpu.sf = false;
                cpu.of = false;
                cpu.zf = !(fa < fb) & !(fa > fb);
                cpu.cf = !(fa >= fb);
                cpu.pfv = (!unord) as u8;
            }
            Mnemonic::Comisd | Mnemonic::Ucomisd | Mnemonic::Vcomisd | Mnemonic::Vucomisd => {
                let a_bits = vops::xmm_idx_fast(ins.op_register(0)).map_or(0, |i| cpu.xmm[i] as u64);
                let b_bits = match ins.op_kind(1) {
                    OpKind::Register => vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i] as u64),
                    OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), 64),
                    _ => 0,
                };
                let fa = f64::from_bits(a_bits);
                let fb = f64::from_bits(b_bits);
                let unord = !(fa <= fb) & !(fa > fb);
                cpu.sf = false;
                cpu.of = false;
                cpu.zf = !(fa < fb) & !(fa > fb);
                cpu.cf = !(fa >= fb);
                cpu.pfv = (!unord) as u8;
            }
            Mnemonic::Cvtdq2ps | Mnemonic::Vcvtdq2ps => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (v_lo, v_hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    let lo = vops::cvtdq2ps(v_lo);
                    let hi = if ymm { vops::cvtdq2ps(v_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Cvtsi2ss | Mnemonic::Vcvtsi2ss | Mnemonic::Cvtsi2sd | Mnemonic::Vcvtsi2sd => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let src_op = ins.op_count() - 1;
                    let base = if ins.op_count() >= 3 {
                        vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i])
                    } else {
                        cpu.xmm[d]
                    };
                    let bits = Cpu::op_bits(ins, src_op);
                    let v = cpu.read_op(ins, src_op, bits);
                    let iv = if bits == 64 { v as i64 } else { v as u32 as i32 as i64 };
                    cpu.xmm[d] = if matches!(mn, Mnemonic::Cvtsi2ss | Mnemonic::Vcvtsi2ss) {
                        (base & !0xFFFF_FFFF) | ((iv as f32).to_bits() as u128)
                    } else {
                        (base & !0xFFFF_FFFF_FFFF_FFFF) | ((iv as f64).to_bits() as u128)
                    };
                    cpu.vex128_zero_upper(ins, d);
                }
            }
            Mnemonic::Cvtss2sd | Mnemonic::Vcvtss2sd | Mnemonic::Cvtsd2ss | Mnemonic::Vcvtsd2ss => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let src_op = ins.op_count() - 1;
                    let base = if ins.op_count() >= 3 {
                        vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i])
                    } else {
                        cpu.xmm[d]
                    };
                    let (s, _) = cpu.vsrc(ins, src_op, false);
                    cpu.xmm[d] = if matches!(mn, Mnemonic::Cvtss2sd | Mnemonic::Vcvtss2sd) {
                        let f = f32::from_bits(s as u32) as f64;
                        (base & !0xFFFF_FFFF_FFFF_FFFF) | (f.to_bits() as u128)
                    } else {
                        let f = f64::from_bits(s as u64) as f32;
                        (base & !0xFFFF_FFFF) | (f.to_bits() as u128)
                    };
                    cpu.vex128_zero_upper(ins, d);
                }
            }
            Mnemonic::Cvttss2si | Mnemonic::Cvtss2si | Mnemonic::Vcvttss2si | Mnemonic::Vcvtss2si
            | Mnemonic::Cvttsd2si | Mnemonic::Cvtsd2si | Mnemonic::Vcvttsd2si | Mnemonic::Vcvtsd2si => {
                let dst_bits = Cpu::op_bits(ins, 0);
                let (s, _) = cpu.vsrc(ins, 1, false);
                let is_sd = matches!(mn, Mnemonic::Cvttsd2si | Mnemonic::Cvtsd2si | Mnemonic::Vcvttsd2si | Mnemonic::Vcvtsd2si);
                let trunc = matches!(mn, Mnemonic::Cvttss2si | Mnemonic::Vcvttss2si | Mnemonic::Cvttsd2si | Mnemonic::Vcvttsd2si);
                let f = if is_sd { f64::from_bits(s as u64) } else { f32::from_bits(s as u32) as f64 };
                let iv = if dst_bits == 64 { sse_cvt_i64(f, trunc) } else { sse_cvt_i32(f, trunc) as u64 };
                cpu.write_op(ins, 0, dst_bits, iv);
            }
            Mnemonic::Psrlw
            | Mnemonic::Vpsrlw
            | Mnemonic::Psllw
            | Mnemonic::Vpsllw
            | Mnemonic::Psraw
            | Mnemonic::Vpsraw => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (src_op, cnt_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (v_lo, v_hi) = vops::load_vec_reg(cpu, ins.op_register(src_op), ymm);
                    let cnt = match ins.op_kind(cnt_op) {
                        OpKind::Register => vops::xmm_idx_fast(ins.op_register(cnt_op)).map_or(0, |i| cpu.xmm[i] as u64),
                        OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), 64),
                        _ => cpu.read_op(ins, cnt_op, 8),
                    };
                    let kind = match mn {
                        Mnemonic::Psraw | Mnemonic::Vpsraw => 2,
                        Mnemonic::Psrlw | Mnemonic::Vpsrlw => 1,
                        _ => 0,
                    };
                    let lo = vops::pshift_w(v_lo, cnt, kind);
                    let hi = if ymm { vops::pshift_w(v_hi, cnt, kind) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Psrld
            | Mnemonic::Vpsrld
            | Mnemonic::Pslld
            | Mnemonic::Vpslld
            | Mnemonic::Psrad
            | Mnemonic::Vpsrad => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (src_op, cnt_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (v_lo, v_hi) = vops::load_vec_reg(cpu, ins.op_register(src_op), ymm);
                    let cnt = match ins.op_kind(cnt_op) {
                        OpKind::Register => vops::xmm_idx_fast(ins.op_register(cnt_op)).map_or(0, |i| cpu.xmm[i] as u64),
                        OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), 64),
                        _ => cpu.read_op(ins, cnt_op, 8),
                    };
                    let kind = match mn {
                        Mnemonic::Psrad | Mnemonic::Vpsrad => 2,
                        Mnemonic::Psrld | Mnemonic::Vpsrld => 1,
                        _ => 0,
                    };
                    let lo = vops::pshift_d(v_lo, cnt, kind);
                    let hi = if ymm { vops::pshift_d(v_hi, cnt, kind) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Psrlq | Mnemonic::Vpsrlq | Mnemonic::Psllq | Mnemonic::Vpsllq => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (src_op, cnt_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (v_lo, v_hi) = vops::load_vec_reg(cpu, ins.op_register(src_op), ymm);
                    let cnt = match ins.op_kind(cnt_op) {
                        OpKind::Register => vops::xmm_idx_fast(ins.op_register(cnt_op)).map_or(0, |i| cpu.xmm[i] as u64),
                        OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), 64),
                        _ => cpu.read_op(ins, cnt_op, 8),
                    };
                    let is_shr = matches!(mn, Mnemonic::Psrlq | Mnemonic::Vpsrlq);
                    let lo = vops::pshift_q(v_lo, cnt, is_shr);
                    let hi = if ymm { vops::pshift_q(v_hi, cnt, is_shr) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Psrldq | Mnemonic::Vpsrldq | Mnemonic::Pslldq | Mnemonic::Vpslldq => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (src_op, cnt_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (v_lo, v_hi) = vops::load_vec_op(cpu, ins, src_op, ymm);
                    let bytes_cnt = (cpu.read_op(ins, cnt_op, 8) as u32).min(16);
                    let is_shr = matches!(mn, Mnemonic::Psrldq | Mnemonic::Vpsrldq);
                    let sh = |v: u128| -> u128 {
                        if bytes_cnt >= 16 {
                            0
                        } else if is_shr {
                            v >> (bytes_cnt * 8)
                        } else {
                            v << (bytes_cnt * 8)
                        }
                    };
                    let lo = sh(v_lo);
                    let hi = if ymm { sh(v_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Paddd | Mnemonic::Vpaddd | Mnemonic::Psubd | Mnemonic::Vpsubd | Mnemonic::Paddq | Mnemonic::Vpaddq
            | Mnemonic::Psubq | Mnemonic::Vpsubq => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let (lo, hi) = match mn {
                        Mnemonic::Paddd | Mnemonic::Vpaddd => (vops::paddd(a_lo, b_lo), if ymm { vops::paddd(a_hi, b_hi) } else { 0 }),
                        Mnemonic::Psubd | Mnemonic::Vpsubd => (vops::psubd(a_lo, b_lo), if ymm { vops::psubd(a_hi, b_hi) } else { 0 }),
                        Mnemonic::Paddq | Mnemonic::Vpaddq => (vops::paddq(a_lo, b_lo), if ymm { vops::paddq(a_hi, b_hi) } else { 0 }),
                        _ => (vops::psubq(a_lo, b_lo), if ymm { vops::psubq(a_hi, b_hi) } else { 0 }),
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pand | Mnemonic::Vpand | Mnemonic::Andps | Mnemonic::Vandps | Mnemonic::Andpd | Mnemonic::Vandpd
            | Mnemonic::Por | Mnemonic::Vpor | Mnemonic::Orps | Mnemonic::Vorps | Mnemonic::Orpd | Mnemonic::Vorpd
            | Mnemonic::Pandn | Mnemonic::Vpandn | Mnemonic::Andnps | Mnemonic::Vandnps | Mnemonic::Andnpd | Mnemonic::Vandnpd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let (lo, hi) = match mn {
                        Mnemonic::Por | Mnemonic::Vpor | Mnemonic::Orps | Mnemonic::Vorps | Mnemonic::Orpd | Mnemonic::Vorpd => {
                            (vops::v_or(a_lo, b_lo), if ymm { vops::v_or(a_hi, b_hi) } else { 0 })
                        }
                        Mnemonic::Pandn | Mnemonic::Vpandn | Mnemonic::Andnps | Mnemonic::Vandnps | Mnemonic::Andnpd | Mnemonic::Vandnpd => {
                            (vops::v_andn(a_lo, b_lo), if ymm { vops::v_andn(a_hi, b_hi) } else { 0 })
                        }
                        _ => (vops::v_and(a_lo, b_lo), if ymm { vops::v_and(a_hi, b_hi) } else { 0 }),
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pshufd | Mnemonic::Vpshufd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s_lo, s_hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    let imm = ins.immediate8();
                    let lo = vops::pshufd(s_lo, imm);
                    let hi = if ymm { vops::pshufd(s_hi, imm) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Shufps | Mnemonic::Vshufps => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let imm = cpu.read_op(ins, imm_op, 8) as u8;
                    let lo = vops::shufps(a_lo, b_lo, imm);
                    let hi = if ymm { vops::shufps(a_hi, b_hi, imm) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Punpcklqdq | Mnemonic::Vpunpcklqdq | Mnemonic::Unpcklpd | Mnemonic::Vunpcklpd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let lo = vops::punpcklqdq(a_lo, b_lo);
                    let hi = if ymm { vops::punpcklqdq(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Unpcklps | Mnemonic::Vunpcklps => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let lo = vops::punpckldq(a_lo, b_lo);
                    let hi = if ymm { vops::punpckldq(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Punpcklbw | Mnemonic::Vpunpcklbw => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let lo = vops::punpcklbw(a_lo, b_lo);
                    let hi = if ymm { vops::punpcklbw(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Psignb | Mnemonic::Vpsignb | Mnemonic::Psignw | Mnemonic::Vpsignw | Mnemonic::Psignd
            | Mnemonic::Vpsignd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let (lo, hi) = match mn {
                        Mnemonic::Psignb | Mnemonic::Vpsignb => (vops::psignb(a_lo, b_lo), if ymm { vops::psignb(a_hi, b_hi) } else { 0 }),
                        Mnemonic::Psignw | Mnemonic::Vpsignw => (vops::psignw(a_lo, b_lo), if ymm { vops::psignw(a_hi, b_hi) } else { 0 }),
                        _ => (vops::psignd(a_lo, b_lo), if ymm { vops::psignd(a_hi, b_hi) } else { 0 }),
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pcmpeqb | Mnemonic::Vpcmpeqb | Mnemonic::Pcmpgtb | Mnemonic::Vpcmpgtb => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let (lo, hi) = if matches!(mn, Mnemonic::Pcmpeqb | Mnemonic::Vpcmpeqb) {
                        (vops::pcmpeqb(a_lo, b_lo), if ymm { vops::pcmpeqb(a_hi, b_hi) } else { 0 })
                    } else {
                        (vops::pcmpgtb(a_lo, b_lo), if ymm { vops::pcmpgtb(a_hi, b_hi) } else { 0 })
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pcmpeqw | Mnemonic::Vpcmpeqw | Mnemonic::Pcmpgtw | Mnemonic::Vpcmpgtw => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let (lo, hi) = if matches!(mn, Mnemonic::Pcmpeqw | Mnemonic::Vpcmpeqw) {
                        (vops::pcmpeqw(a_lo, b_lo), if ymm { vops::pcmpeqw(a_hi, b_hi) } else { 0 })
                    } else {
                        (vops::pcmpgtw(a_lo, b_lo), if ymm { vops::pcmpgtw(a_hi, b_hi) } else { 0 })
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pcmpeqd | Mnemonic::Vpcmpeqd | Mnemonic::Pcmpgtd | Mnemonic::Vpcmpgtd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let (lo, hi) = if matches!(mn, Mnemonic::Pcmpeqd | Mnemonic::Vpcmpeqd) {
                        (vops::pcmpeqd(a_lo, b_lo), if ymm { vops::pcmpeqd(a_hi, b_hi) } else { 0 })
                    } else {
                        (vops::pcmpgtd(a_lo, b_lo), if ymm { vops::pcmpgtd(a_hi, b_hi) } else { 0 })
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pcmpeqq | Mnemonic::Vpcmpeqq | Mnemonic::Pcmpgtq | Mnemonic::Vpcmpgtq => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let (lo, hi) = if matches!(mn, Mnemonic::Pcmpeqq | Mnemonic::Vpcmpeqq) {
                        (vops::pcmpeqq(a_lo, b_lo), if ymm { vops::pcmpeqq(a_hi, b_hi) } else { 0 })
                    } else {
                        (vops::pcmpgtq(a_lo, b_lo), if ymm { vops::pcmpgtq(a_hi, b_hi) } else { 0 })
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pmovmskb | Mnemonic::Vpmovmskb => {
                if let Some((s, ymm)) = vops::xmm_reg_info(ins.op_register(1)) {
                    let mask = if ymm {
                        vops::pmovmskb_ymm(cpu.xmm[s], cpu.ymm_hi[s])
                    } else {
                        vops::pmovmskb(cpu.xmm[s])
                    };
                    let dst_bits = Cpu::op_bits(ins, 0);
                    cpu.write_op(ins, 0, dst_bits, mask);
                }
            }
            Mnemonic::Movmskps | Mnemonic::Vmovmskps => {
                if let Some((s, ymm)) = vops::xmm_reg_info(ins.op_register(1)) {
                    let lo = cpu.xmm[s];
                    let mut mask = (((lo >> 31) & 1)
                        | (((lo >> 63) & 1) << 1)
                        | (((lo >> 95) & 1) << 2)
                        | (((lo >> 127) & 1) << 3)) as u64;
                    if ymm {
                        let hi = cpu.ymm_hi[s];
                        mask |= ((((hi >> 31) & 1)
                            | (((hi >> 63) & 1) << 1)
                            | (((hi >> 95) & 1) << 2)
                            | (((hi >> 127) & 1) << 3)) as u64)
                            << 4;
                    }
                    let dst_bits = Cpu::op_bits(ins, 0);
                    cpu.write_op(ins, 0, dst_bits, mask);
                }
            }
            Mnemonic::Movmskpd | Mnemonic::Vmovmskpd => {
                if let Some((s, ymm)) = vops::xmm_reg_info(ins.op_register(1)) {
                    let lo = cpu.xmm[s];
                    let mut mask = (((lo >> 63) & 1) | (((lo >> 127) & 1) << 1)) as u64;
                    if ymm {
                        let hi = cpu.ymm_hi[s];
                        mask |= ((((hi >> 63) & 1) | (((hi >> 127) & 1) << 1)) as u64) << 2;
                    }
                    let dst_bits = Cpu::op_bits(ins, 0);
                    cpu.write_op(ins, 0, dst_bits, mask);
                }
            }
            Mnemonic::Vpbroadcastb | Mnemonic::Vpbroadcastw | Mnemonic::Vpbroadcastd | Mnemonic::Vpbroadcastq
            | Mnemonic::Vbroadcastss | Mnemonic::Vbroadcastsd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let elem_bits: u32 = match mn {
                        Mnemonic::Vpbroadcastb => 8,
                        Mnemonic::Vpbroadcastw => 16,
                        Mnemonic::Vpbroadcastd | Mnemonic::Vbroadcastss => 32,
                        _ => 64,
                    };
                    let val = match ins.op_kind(1) {
                        OpKind::Register => vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i]),
                        OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), elem_bits) as u128,
                        _ => 0,
                    };
                    let out = vops::pbroadcast(val, elem_bits);
                    cpu.xmm[d] = out;
                    if ymm {
                        cpu.set_ymm_hi(d, out);
                    } else {
                        cpu.clear_ymm_hi_reg(d);
                    }
                }
            }
            Mnemonic::Pshufb | Mnemonic::Vpshufb => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let lo = vops::pshufb(a_lo, b_lo);
                    let hi = if ymm { vops::pshufb(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Paddb | Mnemonic::Vpaddb | Mnemonic::Psubb | Mnemonic::Vpsubb | Mnemonic::Paddw | Mnemonic::Vpaddw
            | Mnemonic::Psubw | Mnemonic::Vpsubw => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let (lo, hi) = match mn {
                        Mnemonic::Paddb | Mnemonic::Vpaddb => (vops::paddb(a_lo, b_lo), if ymm { vops::paddb(a_hi, b_hi) } else { 0 }),
                        Mnemonic::Psubb | Mnemonic::Vpsubb => (vops::psubb(a_lo, b_lo), if ymm { vops::psubb(a_hi, b_hi) } else { 0 }),
                        Mnemonic::Paddw | Mnemonic::Vpaddw => (vops::paddw(a_lo, b_lo), if ymm { vops::paddw(a_hi, b_hi) } else { 0 }),
                        _ => (vops::psubw(a_lo, b_lo), if ymm { vops::psubw(a_hi, b_hi) } else { 0 }),
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Ptest | Mnemonic::Vptest => {
                let s1_reg = ins.op_register(0);
                let ymm = vops::is_ymm_fast(s1_reg);
                let (a_lo, a_hi) = vops::load_vec_reg(cpu, s1_reg, ymm);
                let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                if ymm {
                    cpu.zf = ((a_lo & b_lo) | (a_hi & b_hi)) == 0;
                    cpu.cf = (((!a_lo) & b_lo) | ((!a_hi) & b_hi)) == 0;
                } else {
                    cpu.zf = (a_lo & b_lo) == 0;
                    cpu.cf = ((!a_lo) & b_lo) == 0;
                }
                cpu.sf = false;
                cpu.of = false;
                cpu.set_pf(false);
            }
            Mnemonic::Vzeroupper => {
                cpu.clear_all_ymm_hi();
            }
            Mnemonic::Vzeroall => {
                cpu.xmm = [0; 16];
                cpu.clear_all_ymm_hi();
            }
            Mnemonic::Roundss | Mnemonic::Vroundss => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (s1_op, s2_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let hi = vops::xmm_idx_fast(ins.op_register(s1_op)).map_or(0, |i| cpu.xmm[i]);
                    let b_bits = match ins.op_kind(s2_op) {
                        OpKind::Register => vops::xmm_idx_fast(ins.op_register(s2_op)).map_or(0, |i| cpu.xmm[i] as u32),
                        OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), 32) as u32,
                        _ => 0,
                    };
                    let imm = cpu.read_op(ins, imm_op, 8) as u8;
                    let f = f32::from_bits(b_bits);
                    let r = match imm & 3 {
                        0 => f.round_ties_even(),
                        1 => f.floor(),
                        2 => f.ceil(),
                        _ => f.trunc(),
                    };
                    cpu.xmm[d] = (hi & !0xFFFF_FFFF) | (r.to_bits() as u128);
                    cpu.vex128_zero_upper(ins, d);
                }
            }
            Mnemonic::Roundsd | Mnemonic::Vroundsd => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (s1_op, s2_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let hi = vops::xmm_idx_fast(ins.op_register(s1_op)).map_or(0, |i| cpu.xmm[i]);
                    let b_bits = match ins.op_kind(s2_op) {
                        OpKind::Register => vops::xmm_idx_fast(ins.op_register(s2_op)).map_or(0, |i| cpu.xmm[i] as u64),
                        OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), 64),
                        _ => 0,
                    };
                    let imm = cpu.read_op(ins, imm_op, 8) as u8;
                    let f = f64::from_bits(b_bits);
                    let r = match imm & 3 {
                        0 => f.round_ties_even(),
                        1 => f.floor(),
                        2 => f.ceil(),
                        _ => f.trunc(),
                    };
                    cpu.xmm[d] = (hi & !0xFFFF_FFFF_FFFF_FFFF) | (r.to_bits() as u128);
                    cpu.vex128_zero_upper(ins, d);
                }
            }
            Mnemonic::Roundps | Mnemonic::Vroundps => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (lo, hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    let imm = cpu.read_op(ins, 2, 8) as u8;
                    let d_lo = vops::round_ps(lo, imm);
                    let d_hi = if ymm { vops::round_ps(hi, imm) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, d_lo, d_hi);
                }
            }
            Mnemonic::Roundpd | Mnemonic::Vroundpd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (lo, hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    let imm = cpu.read_op(ins, 2, 8) as u8;
                    let d_lo = vops::round_pd(lo, imm);
                    let d_hi = if ymm { vops::round_pd(hi, imm) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, d_lo, d_hi);
                }
            }
            Mnemonic::Vcvtph2ps => {
                if let Some((d, is_256)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let raw = match ins.op_kind(1) {
                        OpKind::Register => vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i]),
                        OpKind::Memory => {
                            let addr = cpu.eff_addr(ins);
                            if is_256 {
                                cpu.read_u128(addr)
                            } else {
                                cpu.read_mem(addr, 64) as u128
                            }
                        }
                        _ => 0,
                    };
                    let f16_to_f32_bits = |h: u16| -> u32 {
                        let sign = ((h as u32) & 0x8000) << 16;
                        let exp = ((h >> 10) & 0x1F) as u32;
                        let mant = (h & 0x03FF) as u32;
                        if exp == 0 {
                            if mant == 0 {
                                sign
                            } else {
                                let f = (mant as f32) * (1.0 / 16777216.0);
                                let bits = f.to_bits();
                                sign | bits
                            }
                        } else if exp == 0x1F {
                            sign | 0x7F80_0000 | (mant << 13)
                        } else {
                            sign | ((exp + 112) << 23) | (mant << 13)
                        }
                    };
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
                    vops::store_vec_dst(cpu, ins, d, is_256, d_lo, d_hi);
                }
            }
            Mnemonic::Vcvtps2ph => {
                let (s, is_256) = vops::xmm_reg_info(ins.op_register(1)).unwrap_or((0, false));
                let (lo, hi) = (cpu.xmm[s], if is_256 { cpu.ymm_hi[s] } else { 0 });
                let f32_to_f16_bits = |bits: u32| -> u16 {
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
                };
                let mut packed = 0u128;
                for lane in 0..4 {
                    let w = (lo >> (lane * 32)) as u32;
                    packed |= (f32_to_f16_bits(w) as u128) << (lane * 16);
                }
                if is_256 {
                    for lane in 0..4 {
                        let w = (hi >> (lane * 32)) as u32;
                        packed |= (f32_to_f16_bits(w) as u128) << ((lane + 4) * 16);
                    }
                }
                match ins.op_kind(0) {
                    OpKind::Register => {
                        if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                            cpu.xmm[d] = packed;
                            cpu.clear_ymm_hi_reg(d);
                        }
                    }
                    OpKind::Memory => {
                        let addr = cpu.eff_addr(ins);
                        if is_256 {
                            cpu.write_u128(addr, packed);
                        } else {
                            cpu.write_mem(addr, 64, packed as u64);
                        }
                    }
                    _ => {}
                }
            }
            Mnemonic::Movhpd | Mnemonic::Vmovhpd | Mnemonic::Movhps | Mnemonic::Vmovhps => {
                match (ins.op_kind(0), ins.op_kind(1)) {
                    (OpKind::Memory, OpKind::Register) => {
                        if let Some(s) = vops::xmm_idx_fast(ins.op_register(1)) {
                            let a = cpu.eff_addr(ins);
                            cpu.write_mem(a, 64, (cpu.xmm[s] >> 64) as u64);
                        }
                    }
                    (OpKind::Register, _) => {
                        if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                            let (s1_op, m_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                            let lo = vops::xmm_idx_fast(ins.op_register(s1_op)).map_or(0, |i| cpu.xmm[i] as u64);
                            let hi = if ins.op_kind(m_op) == OpKind::Memory {
                                cpu.read_mem(cpu.eff_addr(ins), 64)
                            } else {
                                0
                            };
                            cpu.xmm[d] = (lo as u128) | ((hi as u128) << 64);
                            if Cpu::is_vex(ins) {
                                cpu.clear_ymm_hi_reg(d);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Mnemonic::Movlpd | Mnemonic::Vmovlpd | Mnemonic::Movlps | Mnemonic::Vmovlps => {
                match (ins.op_kind(0), ins.op_kind(1)) {
                    (OpKind::Memory, OpKind::Register) => {
                        if let Some(s) = vops::xmm_idx_fast(ins.op_register(1)) {
                            let a = cpu.eff_addr(ins);
                            cpu.write_mem(a, 64, cpu.xmm[s] as u64);
                        }
                    }
                    (OpKind::Register, _) => {
                        if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                            let (s1_op, m_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                            let hi = vops::xmm_idx_fast(ins.op_register(s1_op)).map_or(0, |i| (cpu.xmm[i] >> 64) as u64);
                            let lo = if ins.op_kind(m_op) == OpKind::Memory {
                                cpu.read_mem(cpu.eff_addr(ins), 64)
                            } else {
                                0
                            };
                            cpu.xmm[d] = (lo as u128) | ((hi as u128) << 64);
                            if Cpu::is_vex(ins) {
                                cpu.clear_ymm_hi_reg(d);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Mnemonic::Addps | Mnemonic::Subps | Mnemonic::Mulps | Mnemonic::Divps | Mnemonic::Minps | Mnemonic::Maxps
            | Mnemonic::Vaddps | Mnemonic::Vsubps | Mnemonic::Vmulps | Mnemonic::Vdivps | Mnemonic::Vminps | Mnemonic::Vmaxps
            | Mnemonic::Addpd | Mnemonic::Subpd | Mnemonic::Mulpd | Mnemonic::Divpd | Mnemonic::Minpd | Mnemonic::Maxpd
            | Mnemonic::Vaddpd | Mnemonic::Vsubpd | Mnemonic::Vmulpd | Mnemonic::Vdivpd | Mnemonic::Vminpd | Mnemonic::Vmaxpd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let op: u8 = match mn {
                        Mnemonic::Addps | Mnemonic::Vaddps | Mnemonic::Addpd | Mnemonic::Vaddpd => 0,
                        Mnemonic::Subps | Mnemonic::Vsubps | Mnemonic::Subpd | Mnemonic::Vsubpd => 1,
                        Mnemonic::Mulps | Mnemonic::Vmulps | Mnemonic::Mulpd | Mnemonic::Vmulpd => 2,
                        Mnemonic::Divps | Mnemonic::Vdivps | Mnemonic::Divpd | Mnemonic::Vdivpd => 3,
                        Mnemonic::Minps | Mnemonic::Vminps | Mnemonic::Minpd | Mnemonic::Vminpd => 4,
                        _ => 5,
                    };
                    let is_pd = matches!(
                        mn,
                        Mnemonic::Addpd | Mnemonic::Subpd | Mnemonic::Mulpd | Mnemonic::Divpd | Mnemonic::Minpd | Mnemonic::Maxpd
                            | Mnemonic::Vaddpd | Mnemonic::Vsubpd | Mnemonic::Vmulpd | Mnemonic::Vdivpd | Mnemonic::Vminpd
                            | Mnemonic::Vmaxpd
                    );
                    let (lo, hi) = if is_pd {
                        (vops::fp_bin_pd(op, a_lo, b_lo), if ymm { vops::fp_bin_pd(op, a_hi, b_hi) } else { 0 })
                    } else {
                        (vops::fp_bin_ps(op, a_lo, b_lo), if ymm { vops::fp_bin_ps(op, a_hi, b_hi) } else { 0 })
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Sqrtps | Mnemonic::Vsqrtps | Mnemonic::Sqrtpd | Mnemonic::Vsqrtpd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s_lo, s_hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    let (lo, hi) = if matches!(mn, Mnemonic::Sqrtpd | Mnemonic::Vsqrtpd) {
                        (vops::sqrt_pd(s_lo), if ymm { vops::sqrt_pd(s_hi) } else { 0 })
                    } else {
                        (vops::sqrt_ps(s_lo), if ymm { vops::sqrt_ps(s_hi) } else { 0 })
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Blendps | Mnemonic::Vblendps => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let imm = cpu.read_op(ins, imm_op, 8) as u8;
                    let lo = vops::blendps_imm(a_lo, b_lo, imm & 0xF);
                    let hi = if ymm { vops::blendps_imm(a_hi, b_hi, imm >> 4) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Blendpd | Mnemonic::Vblendpd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let imm = cpu.read_op(ins, imm_op, 8) as u8;
                    let blend2 = |a: u128, b: u128, bits: u8| -> u128 {
                        let q0 = if (bits & 1) != 0 { b as u64 } else { a as u64 };
                        let q1 = if (bits & 2) != 0 { (b >> 64) as u64 } else { (a >> 64) as u64 };
                        (q0 as u128) | ((q1 as u128) << 64)
                    };
                    let lo = blend2(a_lo, b_lo, imm);
                    let hi = if ymm { blend2(a_hi, b_hi, imm >> 2) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Insertps | Mnemonic::Vinsertps => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let (s1_op, s2_op, imm_op) = if ins.op_count() >= 4 { (1, 2, 3) } else { (0, 1, 2) };
                    let a = vops::xmm_idx_fast(ins.op_register(s1_op)).map_or(0, |i| cpu.xmm[i]);
                    let imm = cpu.read_op(ins, imm_op, 8) as u8;
                    let count_s = ((imm >> 6) & 3) as usize;
                    let count_d = ((imm >> 4) & 3) as usize;
                    let zmask = imm & 0xF;
                    let elem = match ins.op_kind(s2_op) {
                        OpKind::Register => {
                            let b = vops::xmm_idx_fast(ins.op_register(s2_op)).map_or(0, |i| cpu.xmm[i]);
                            ((b >> (count_s * 32)) & 0xFFFF_FFFF) as u32
                        }
                        OpKind::Memory => cpu.read_mem(cpu.eff_addr(ins), 32) as u32,
                        _ => 0,
                    };
                    let mut out = a & !(0xFFFF_FFFFu128 << (count_d * 32));
                    out |= (elem as u128) << (count_d * 32);
                    for lane in 0..4 {
                        if ((zmask >> lane) & 1) != 0 {
                            out &= !(0xFFFF_FFFFu128 << (lane * 32));
                        }
                    }
                    cpu.xmm[d] = out;
                    cpu.vex128_zero_upper(ins, d);
                }
            }
            Mnemonic::Pshuflw | Mnemonic::Vpshuflw => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let imm = cpu.read_op(ins, 2, 8) as u8;
                    let (src_lo, src_hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    let lo = vops::pshuflw(src_lo, imm);
                    let hi = if ymm { vops::pshuflw(src_hi, imm) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pshufhw | Mnemonic::Vpshufhw => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let imm = cpu.read_op(ins, 2, 8) as u8;
                    let (src_lo, src_hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    let lo = vops::pshufhw(src_lo, imm);
                    let hi = if ymm { vops::pshufhw(src_hi, imm) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Vpermilps => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (src_lo, src_hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    if matches!(ins.op_kind(2), OpKind::Immediate8) {
                        let imm = cpu.read_op(ins, 2, 8) as u8;
                        let lo = vops::pshufd(src_lo, imm);
                        let hi = if ymm { vops::pshufd(src_hi, imm) } else { 0 };
                        vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                    } else {
                        let (ctrl_lo, ctrl_hi) = vops::load_vec_op(cpu, ins, 2, ymm);
                        let lo = vops::vpermilps_var(src_lo, ctrl_lo);
                        let hi = if ymm { vops::vpermilps_var(src_hi, ctrl_hi) } else { 0 };
                        vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                    }
                }
            }
            Mnemonic::Cvttps2dq | Mnemonic::Vcvttps2dq | Mnemonic::Cvtps2dq | Mnemonic::Vcvtps2dq => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let trunc = matches!(mn, Mnemonic::Cvttps2dq | Mnemonic::Vcvttps2dq);
                    let (src_lo, src_hi) = vops::load_vec_op(cpu, ins, 1, ymm);
                    let lo = vops::cvtps2dq(src_lo, trunc);
                    let hi = if ymm { vops::cvtps2dq(src_hi, trunc) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Unpckhpd | Mnemonic::Vunpckhpd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let (a_lo, a_hi) = vops::load_vec_reg(cpu, ins.op_register(s1_op), ymm);
                    let (b_lo, b_hi) = vops::load_vec_op(cpu, ins, s2_op, ymm);
                    let lo = vops::punpckhqdq(a_lo, b_lo);
                    let hi = if ymm { vops::punpckhqdq(a_hi, b_hi) } else { 0 };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Pcmpistri
            | Mnemonic::Vpcmpistri
            | Mnemonic::Pcmpistrm
            | Mnemonic::Vpcmpistrm
            | Mnemonic::Pcmpestri
            | Mnemonic::Vpcmpestri
            | Mnemonic::Pcmpestrm
            | Mnemonic::Vpcmpestrm => {
                let is_explicit = matches!(
                    mn,
                    Mnemonic::Pcmpestri
                        | Mnemonic::Vpcmpestri
                        | Mnemonic::Pcmpestrm
                        | Mnemonic::Vpcmpestrm
                );
                let is_mask = matches!(
                    mn,
                    Mnemonic::Pcmpistrm
                        | Mnemonic::Vpcmpistrm
                        | Mnemonic::Pcmpestrm
                        | Mnemonic::Vpcmpestrm
                );
                let a = vops::xmm_idx_fast(ins.op_register(0)).map_or(0, |i| cpu.xmm[i]);
                let b = match ins.op_kind(1) {
                    OpKind::Register => vops::xmm_idx_fast(ins.op_register(1)).map_or(0, |i| cpu.xmm[i]),
                    OpKind::Memory => cpu.read_u128(cpu.eff_addr(ins)),
                    _ => 0,
                };
                let imm = ins.immediate8();
                let is_words = (imm & 0x01) != 0;
                let is_signed = (imm & 0x02) != 0;
                let n: usize = if is_words { 8 } else { 16 };
                let get_elem = |v: u128, idx: usize| -> i32 {
                    if is_words {
                        let w = ((v >> (idx * 16)) & 0xFFFF) as u16;
                        if is_signed { w as i16 as i32 } else { w as i32 }
                    } else {
                        let b_val = ((v >> (idx * 8)) & 0xFF) as u8;
                        if is_signed { b_val as i8 as i32 } else { b_val as i32 }
                    }
                };
                let (a_len, b_len) = if is_explicit {
                    let la = (cpu.gpr[0] as i32).unsigned_abs().min(n as u32) as usize;
                    let lb = (cpu.gpr[2] as i32).unsigned_abs().min(n as u32) as usize;
                    (la, lb)
                } else {
                    let mut la = n;
                    for i in 0..n {
                        if get_elem(a, i) == 0 {
                            la = i;
                            break;
                        }
                    }
                    let mut lb = n;
                    for j in 0..n {
                        if get_elem(b, j) == 0 {
                            lb = j;
                            break;
                        }
                    }
                    (la, lb)
                };
                let mut int_res1: u16 = 0;
                match (imm >> 2) & 0x03 {
                    0 => {
                        for j in 0..b_len {
                            let bj = get_elem(b, j);
                            for i in 0..a_len {
                                if bj == get_elem(a, i) {
                                    int_res1 |= 1u16 << j;
                                    break;
                                }
                            }
                        }
                    }
                    1 => {
                        for j in 0..b_len {
                            let bj = get_elem(b, j);
                            let mut i = 0;
                            while i + 1 < a_len {
                                if bj >= get_elem(a, i) && bj <= get_elem(a, i + 1) {
                                    int_res1 |= 1u16 << j;
                                    break;
                                }
                                i += 2;
                            }
                        }
                    }
                    2 => {
                        for j in 0..n {
                            let m = if j < a_len && j < b_len {
                                get_elem(a, j) == get_elem(b, j)
                            } else {
                                j >= a_len && j >= b_len
                            };
                            if m {
                                int_res1 |= 1u16 << j;
                            }
                        }
                    }
                    _ => {
                        for j in 0..n {
                            let mut m = true;
                            for k in 0..(n - j) {
                                if k >= a_len {
                                    break;
                                }
                                if j + k >= b_len {
                                    m = false;
                                    break;
                                }
                                if get_elem(a, k) != get_elem(b, j + k) {
                                    m = false;
                                    break;
                                }
                            }
                            if m {
                                int_res1 |= 1u16 << j;
                            }
                        }
                    }
                }
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
            Mnemonic::Cvtps2pd | Mnemonic::Vcvtps2pd | Mnemonic::Cvtdq2pd | Mnemonic::Vcvtdq2pd => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    // Source is m64/xmm[63:0] (xmm dest) or m128/xmm (ymm dest): preserve exact memory read width.
                    let (raw, _) = cpu.vsrc(ins, 1, false);
                    let (lo, hi) = if matches!(mn, Mnemonic::Cvtdq2pd | Mnemonic::Vcvtdq2pd) {
                        vops::cvtdq2pd(raw, ymm)
                    } else {
                        vops::cvtps2pd(raw, ymm)
                    };
                    vops::store_vec_dst(cpu, ins, d, ymm, lo, hi);
                }
            }
            Mnemonic::Cvtpd2ps | Mnemonic::Vcvtpd2ps | Mnemonic::Cvttpd2dq | Mnemonic::Vcvttpd2dq | Mnemonic::Cvtpd2dq
            | Mnemonic::Vcvtpd2dq => {
                if let Some(d) = vops::xmm_idx_fast(ins.op_register(0)) {
                    let src_256 = match ins.op_kind(1) {
                        OpKind::Register => vops::is_ymm_fast(ins.op_register(1)),
                        OpKind::Memory => ins.memory_size().size() == 32,
                        _ => false,
                    };
                    let (lo, hi) = vops::load_vec_op(cpu, ins, 1, src_256);
                    cpu.xmm[d] = if matches!(mn, Mnemonic::Cvtpd2ps | Mnemonic::Vcvtpd2ps) {
                        vops::cvtpd2ps(lo, hi, src_256)
                    } else {
                        let trunc = matches!(mn, Mnemonic::Cvttpd2dq | Mnemonic::Vcvttpd2dq);
                        vops::cvtpd2dq(lo, hi, src_256, trunc)
                    };
                    if Cpu::is_vex(ins) {
                        cpu.clear_ymm_hi_reg(d);
                    }
                }
            }
            Mnemonic::Pmovsxbw
            | Mnemonic::Vpmovsxbw
            | Mnemonic::Pmovzxbw
            | Mnemonic::Vpmovzxbw
            | Mnemonic::Pmovsxbd
            | Mnemonic::Vpmovsxbd
            | Mnemonic::Pmovzxbd
            | Mnemonic::Vpmovzxbd
            | Mnemonic::Pmovsxbq
            | Mnemonic::Vpmovsxbq
            | Mnemonic::Pmovzxbq
            | Mnemonic::Vpmovzxbq
            | Mnemonic::Pmovsxwd
            | Mnemonic::Vpmovsxwd
            | Mnemonic::Pmovzxwd
            | Mnemonic::Vpmovzxwd
            | Mnemonic::Pmovsxwq
            | Mnemonic::Vpmovsxwq
            | Mnemonic::Pmovzxwq
            | Mnemonic::Vpmovzxwq
            | Mnemonic::Pmovsxdq
            | Mnemonic::Vpmovsxdq
            | Mnemonic::Pmovzxdq
            | Mnemonic::Vpmovzxdq => {
                if let Some((d, ymm)) = vops::xmm_reg_info(ins.op_register(0)) {
                    let s_op = if ins.op_count() >= 2 { 1 } else { 0 };
                    // Preserve exact memory read width (`m16`/`m32`/`m64`/`m128`).
                    let (raw, _) = cpu.vsrc(ins, s_op, false);
                    let code: u8 = match mn {
                        Mnemonic::Pmovsxbw | Mnemonic::Vpmovsxbw => 0,
                        Mnemonic::Pmovzxbw | Mnemonic::Vpmovzxbw => 1,
                        Mnemonic::Pmovsxbd | Mnemonic::Vpmovsxbd => 2,
                        Mnemonic::Pmovzxbd | Mnemonic::Vpmovzxbd => 3,
                        Mnemonic::Pmovsxbq | Mnemonic::Vpmovsxbq => 4,
                        Mnemonic::Pmovzxbq | Mnemonic::Vpmovzxbq => 5,
                        Mnemonic::Pmovsxwd | Mnemonic::Vpmovsxwd => 6,
                        Mnemonic::Pmovzxwd | Mnemonic::Vpmovzxwd => 7,
                        Mnemonic::Pmovsxwq | Mnemonic::Vpmovsxwq => 8,
                        Mnemonic::Pmovzxwq | Mnemonic::Vpmovzxwq => 9,
                        Mnemonic::Pmovsxdq | Mnemonic::Vpmovsxdq => 10,
                        _ => 11,
                    };
                    let (out_lo, out_hi) = vops::pmovsx_zx(raw, code, ymm);
                    vops::store_vec_dst(cpu, ins, d, ymm, out_lo, out_hi);
                }
            }
            _ => return false,
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::vops;
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
            assert_eq!(vops::xmm_idx_fast(r), Some(i));
            assert!(!vops::is_ymm_fast(r));
            assert_eq!(vops::xmm_reg_info(r), Some((i, false)));
        }
        for (i, &r) in ymms.iter().enumerate() {
            assert_eq!(vops::xmm_idx_fast(r), Some(i));
            assert!(vops::is_ymm_fast(r));
            assert_eq!(vops::xmm_reg_info(r), Some((i, true)));
        }
        for r in [Register::None, Register::RAX, Register::ST0, Register::ZMM0, Register::XMM16, Register::YMM16] {
            assert_eq!(vops::xmm_idx_fast(r), None);
            assert!(!vops::is_ymm_fast(r));
            assert_eq!(vops::xmm_reg_info(r), None);
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

        let min_r = vops::fp_bin_ps(4, a, b);
        assert_eq!(min_r as u32, p0, "minps(-0.0, +0.0) == +0.0 (second op)");
        assert_eq!((min_r >> 32) as u32, n0, "minps(+0.0, -0.0) == -0.0 (second op)");
        assert_eq!((min_r >> 64) as u32, 2.5f32.to_bits(), "minps(qNaN, 2.5) == 2.5 (second op)");
        assert_eq!((min_r >> 96) as u32, snan2, "minps(1.5, sNaN) == exact sNaN bits");

        let max_r = vops::fp_bin_ps(5, a, b);
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
        let dmin = vops::fp_bin_pd(4, da, db);
        let dmax = vops::fp_bin_pd(5, da, db);
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

        let div_r = vops::fp_bin_ps(3, a, b);
        assert_eq!(div_r as u32, 0xFFC0_0000, "0.0 / 0.0 -> x86 negative indefinite");
        assert_eq!((div_r >> 32) as u32, 0x7FC0_0011, "qNaN_a / sNaN_b -> first operand a wins");
        assert_eq!((div_r >> 64) as u32, 0x7FC0_0022, "1.0 / sNaN_b -> quieted b");
        assert_eq!((div_r >> 96) as u32, 0xFFC0_0000, "Inf / Inf -> x86 negative indefinite");

        let sqrt_r = vops::sqrt_ps(neg1 | (snan_b << 32) | (4.0f32.to_bits() as u128) << 64);
        assert_eq!(sqrt_r as u32, 0xFFC0_0000, "sqrt(-1.0) -> x86 negative indefinite");
        assert_eq!((sqrt_r >> 32) as u32, 0x7FC0_0022, "sqrt(sNaN) -> quieted sNaN");
        assert_eq!((sqrt_r >> 64) as u32, 2.0f32.to_bits());

        let dz = 0.0f64.to_bits() as u128;
        let dq = 0x7FF8_0000_0000_0011u64 as u128;
        let ds = 0x7FF0_0000_0000_0022u64 as u128;
        let dsub = vops::fp_bin_pd(1, (f64::INFINITY.to_bits() as u128) | (dq << 64), (f64::INFINITY.to_bits() as u128) | (ds << 64));
        assert_eq!(dsub as u64, 0xFFF8_0000_0000_0000, "Inf - Inf -> f64 negative indefinite");
        assert_eq!((dsub >> 64) as u64, 0x7FF8_0000_0000_0011, "f64 first NaN wins over sNaN");
        let dsqrt = vops::sqrt_pd(((-4.0f64).to_bits() as u128) | (dz << 64));
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
        assert_eq!(lanes(vops::cvtps2dq(edge, true)), [i32::MIN, i32::MIN, i32::MIN, i32::MIN]);
        assert_eq!(lanes(vops::cvtps2dq(edge, false)), [i32::MIN, i32::MIN, i32::MIN, i32::MIN]);

        // +Inf, -Inf, 2.5, 3.5 -> round-ties-to-even vs truncate
        let round = pack4([f32::INFINITY, f32::NEG_INFINITY, 2.5, -3.5]);
        assert_eq!(lanes(vops::cvtps2dq(round, true)), [i32::MIN, i32::MIN, 2, -3]);
        assert_eq!(lanes(vops::cvtps2dq(round, false)), [i32::MIN, i32::MIN, 2, -4]);
    }

    #[test]
    fn simd_mov_pmovmskb_and_pshufb_exact_bit_and_byte_order() {
        // Verify each individual bit 0..31 of pmovmskb / pmovmskb_ymm.
        for bit in 0..16u32 {
            let v = 0x80u128 << (bit * 8);
            assert_eq!(vops::pmovmskb(v), 1u64 << bit, "pmovmskb bit {bit}");
            assert_eq!(vops::pmovmskb_ymm(v, 0), 1u64 << bit, "pmovmskb_ymm lo bit {bit}");
            assert_eq!(vops::pmovmskb_ymm(0, v), 1u64 << (bit + 16), "pmovmskb_ymm hi bit {bit}");
        }
        // Non-MSB bits (0x7F) must NEVER set mask bits.
        assert_eq!(vops::pmovmskb(u128::from_le_bytes([0x7F; 16])), 0);
        assert_eq!(vops::pmovmskb(u128::from_le_bytes([0x80; 16])), 0xFFFF);
        assert_eq!(
            vops::pmovmskb_ymm(
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
        assert_eq!(vops::pshufb(src, ctl), want);
    }

    #[test]
    fn simd_mov_pabs_pmulhrsw_saturation_and_pack_edges() {
        // PABSB/W/D with MIN values MUST wrap (not saturate!).
        let min_b = u128::from_le_bytes([0x80, 0x81, 0xFF, 0x00, 0x7F, 0x80, 0x01, 0xFE, 0x80, 0, 0, 0, 0, 0, 0, 0]);
        let got_b = vops::pabsb(min_b).to_le_bytes();
        assert_eq!(&got_b[..8], &[0x80, 0x7F, 0x01, 0x00, 0x7F, 0x80, 0x01, 0x02]);

        let min_w = (i16::MIN as u16 as u128) | ((i16::MAX as u16 as u128) << 16) | (((-1i16) as u16 as u128) << 32);
        let got_w = vops::pabsw(min_w);
        assert_eq!(got_w as u16, 0x8000);
        assert_eq!((got_w >> 16) as u16, 0x7FFF);
        assert_eq!((got_w >> 32) as u16, 1);

        let min_d = (i32::MIN as u32 as u128) | (((-42i32) as u32 as u128) << 32);
        let got_d = vops::pabsd(min_d);
        assert_eq!(got_d as u32, 0x8000_0000);
        assert_eq!((got_d >> 32) as u32, 42);

        // PMULHRSW(-32768, -32768) MUST wrap to -32768 (0x8000), NOT saturate to 0x7FFF!
        let m16 = (i16::MIN as u16 as u128) | ((16384u16 as u128) << 16) | (((-16384i16) as u16 as u128) << 32);
        let r_hrsw = vops::pmulhrsw(m16, m16);
        assert_eq!(r_hrsw as u16, 0x8000, "pmulhrsw(i16::MIN, i16::MIN) == 0x8000");
        assert_eq!((r_hrsw >> 16) as u16, 8192);
        assert_eq!((r_hrsw >> 32) as u16, 8192);

        // Saturating add/sub & pack edges
        let a_sb = u128::from_le_bytes([127, 128, 100, 200, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let b_sb = u128::from_le_bytes([10, 255, 50, 150, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(&vops::paddsb(a_sb, b_sb).to_le_bytes()[..4], &[127, 128, 127, 128]);
        assert_eq!(&vops::paddusb(a_sb, b_sb).to_le_bytes()[..4], &[137, 255, 150, 255]);
        assert_eq!(&vops::psubusb(a_sb, b_sb).to_le_bytes()[..4], &[117, 0, 50, 50]);

        // PACKSSWB / PACKUSWB / PACKSSDW / PACKUSDW
        let w_a: u128 = ((-300i16) as u16 as u128)
            | ((300u16 as u128) << 16)
            | (((-128i16) as u16 as u128) << 32)
            | ((127u16 as u128) << 48);
        let w_b: u128 = ((255u16 as u128)) | (((-1i16) as u16 as u128) << 16);
        let pss = vops::packsswb(w_a, w_b).to_le_bytes();
        assert_eq!(&pss[0..4], &[0x80, 0x7F, 0x80, 0x7F]);
        assert_eq!(&pss[8..10], &[0x7F, 0xFF]);
        let pus = vops::packuswb(w_a, w_b).to_le_bytes();
        assert_eq!(&pus[0..4], &[0x00, 0xFF, 0x00, 0x7F]);
        assert_eq!(&pus[8..10], &[0xFF, 0x00]);
    }
}
