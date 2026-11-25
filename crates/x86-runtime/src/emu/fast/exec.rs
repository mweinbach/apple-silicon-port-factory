//! Execution engine for core [`FastOp`] kinds (`K_NOP..=K_PEXTR`) and shared SIMD dispatch
//! routed through [`crate::emu::simd_kernels`].

use super::ext::exec_ext_more;
use super::*;
use crate::emu::simd_kernels as sk;
use crate::emu::{
    sse_arith32, sse_arith64, sse_cmp_pred, sse_cvt_i32, sse_cvt_i64, sse_sqrt32, sse_sqrt64, Cpu,
    Register,
};

impl Cpu {
    /// Same as `get_reg` for a non-high-byte GPR.
    #[inline(always)]
    pub(super) fn rget(&self, i: u8, w: u8) -> u64 {
        self.gpr[i as usize & 15] & W_MASK[(w >> 4) as usize & 7]
    }

    /// Same as `set_reg` for a non-high-byte GPR (incl. `last_bits`): 8/16-bit writes merge,
    /// 32-bit writes zero-extend.
    #[inline(always)]
    pub(super) fn rset(&mut self, i: u8, w: u8, val: u64) {
        let i = i as usize & 15;
        let k = (w >> 4) as usize & 7;
        self.gpr[i] = (self.gpr[i] & W_KEEP[k]) | (val & W_MASK[k]);
        if self.jx.lb_on != 0 {
            self.last_bits[i] = W_LAST[k];
        }
    }

    #[cold]
    #[inline(never)]
    fn faddr_slow(&self, op: &FastOp, idx: u64) -> u64 {
        let b = op.base;
        if b & B_ADDR32 == 0 {
            let mut a = (op.disp as u64).wrapping_add(idx);
            let reg = b & 0x1F;
            if reg < 16 {
                a = a.wrapping_add(self.gpr[reg as usize]);
            }
            if b & B_GS != 0 {
                let t = if self.jx.teb != 0 {
                    self.jx.teb
                } else {
                    unsafe { crate::exe::teb_self() as u64 }
                };
                a = a.wrapping_add(t);
            }
            a
        } else {
            let reg = b & 0x1F;
            let base = if reg < 16 { self.gpr[reg as usize] } else { 0 };
            let mut a = ((op.disp as u64).wrapping_add(base).wrapping_add(idx) as u32) as u64;
            if b & B_GS != 0 {
                let t = if self.jx.teb != 0 {
                    self.jx.teb
                } else {
                    unsafe { crate::exe::teb_self() as u64 }
                };
                a = a.wrapping_add(t);
            }
            a
        }
    }

    #[inline(always)]
    pub(super) fn faddr(&self, op: &FastOp) -> u64 {
        let idx = if op.index < 16 {
            self.gpr[op.index as usize] << (op.x >> 6)
        } else {
            0
        };
        let b = op.base;
        if b < 16 {
            (op.disp as u64).wrapping_add(self.gpr[b as usize]).wrapping_add(idx)
        } else if b == RNONE {
            (op.disp as u64).wrapping_add(idx)
        } else {
            self.faddr_slow(op, idx)
        }
    }

    /// Direct `cmp`/`test` flag update for fused `K_CMPJ_*` ops (avoids general 9-way `alu` match).
    #[inline(always)]
    fn cmp_or_test(&mut self, sub: u8, a: u64, b: u64, w: u32) {
        if sub == A_CMP {
            let _ = self.update_flags_sub(a, b, w);
        } else {
            self.update_flags_logic(a & b, w);
        }
    }

    #[inline(always)]
    pub(super) fn eval_cc_u8(&self, cc: u8) -> bool {
        match cc & 0x3F {
            1 => self.of,
            2 => !self.of,
            3 => self.cf,
            4 => !self.cf,
            5 => self.zf,
            6 => !self.zf,
            7 => self.cf || self.zf,
            8 => !self.cf && !self.zf,
            9 => self.sf,
            10 => !self.sf,
            11 => self.pf(),
            12 => !self.pf(),
            13 => self.sf != self.of,
            14 => self.sf == self.of,
            15 => self.zf || (self.sf != self.of),
            16 => !self.zf && (self.sf == self.of),
            _ => false,
        }
    }

    /// ALU op with the generic handlers' flag semantics; returns the value to write
    /// back (`None` for `cmp`/`test`).
    #[inline(always)]
    pub(super) fn alu(&mut self, sub: u8, a: u64, b: u64, w: u32) -> Option<u64> {
        match sub {
            A_ADD => Some(self.update_flags_add(a, b, w)),
            A_SUB => Some(self.update_flags_sub(a, b, w)),
            A_AND => {
                let r = a & b;
                self.update_flags_logic(r, w);
                Some(r)
            }
            A_OR => {
                let r = a | b;
                self.update_flags_logic(r, w);
                Some(r)
            }
            A_XOR => {
                let r = a ^ b;
                self.update_flags_logic(r, w);
                Some(r)
            }
            A_CMP => {
                let _ = self.update_flags_sub(a, b, w);
                None
            }
            A_ADC => {
                let mask = if w >= 64 { u64::MAX } else { (1u64 << w) - 1 };
                let (ua, ub) = (a & mask, b & mask);
                let full = (ua as u128) + (ub as u128) + (self.cf as u128);
                let res = (full as u64) & mask;
                let sign_bit = 1u64 << (w - 1);
                self.cf = full > mask as u128;
                self.of = ((ua ^ res) & (ub ^ res) & sign_bit) != 0;
                self.zf = res == 0;
                self.sf = (res & sign_bit) != 0;
                self.pfv = res as u8;
                Some(res)
            }
            A_SBB => {
                let mask = if w >= 64 { u64::MAX } else { (1u64 << w) - 1 };
                let (ua, ub) = (a & mask, b & mask);
                let c = self.cf as u64;
                let res = ua.wrapping_sub(ub).wrapping_sub(c) & mask;
                let sign_bit = 1u64 << (w - 1);
                self.cf = (ua as u128) < (ub as u128) + (c as u128);
                self.of = ((ua ^ ub) & (ua ^ res) & sign_bit) != 0;
                self.zf = res == 0;
                self.sf = (res & sign_bit) != 0;
                self.pfv = res as u8;
                Some(res)
            }
            _ => {
                self.update_flags_logic(a & b, w);
                None
            }
        }
    }

    /// x86 SHL/SHR/SAR (`cnt` already masked and non-zero) with full flags: CF = last bit
    /// shifted out, OF = defined count-1 value, SF/ZF/PF from the result. Shared with the
    /// generic handler.
    #[inline(always)]
    pub(in crate::emu) fn shift(&mut self, sub: u8, a: u64, cnt: u32, w: u8) -> u64 {
        let wb = w as u32;
        let vm = if wb >= 64 { u64::MAX } else { (1u64 << wb) - 1 };
        let ua = a & vm;
        let msb = 1u64 << (wb - 1);
        let (r, cf, of) = match sub {
            S_SHL => {
                let r = ua.wrapping_shl(cnt) & vm;
                let cf = cnt <= wb && (ua >> (wb - cnt)) & 1 != 0;
                (r, cf, ((r & msb) != 0) ^ cf)
            }
            S_SHR => (ua >> cnt, (ua >> (cnt - 1)) & 1 != 0, ua & msb != 0),
            _ => {
                let sa = sext(ua, w) as i64;
                let r = (sa >> cnt.min(63)) as u64 & vm;
                (r, (sa >> (cnt - 1).min(63)) & 1 != 0, false)
            }
        };
        self.cf = cf;
        self.of = of;
        self.zf = r == 0;
        self.sf = r & msb != 0;
        self.pfv = r as u8;
        r
    }

    /// Memory second source of a V-form `*_RM` op: `(bits 127:0, bits 255:128)`.
    #[inline(always)]
    pub(super) unsafe fn vload(&self, op: &FastOp) -> (u128, u128) {
        let a = self.faddr(op);
        match op.w >> 4 {
            2 => (self.read_mem(a, 32) as u128, 0),
            3 => (self.read_mem(a, 64) as u128, 0),
            4 => (self.read_u128(a), 0),
            _ => (self.read_u128(a), self.read_u128(a.wrapping_add(16))),
        }
    }

    /// Execute a V-form op with lane function `f` (second source from memory if `mem`).
    /// Monomorphized over `F` and `IS_VSHUF` so the lane op inlines cleanly and 128-bit ops
    /// skip loading `ymm_hi`.
    #[inline(always)]
    unsafe fn vexec<const IS_VSHUF: bool, F: Fn(u8, u128, u128, u8) -> u128>(&mut self, op: &FastOp, mem: bool, f: F) {
        let sub = op.x & 0x3F;
        let imm8 = op.imm as u8;
        let s1 = (op.imm >> 8) as usize & 15;
        let mode = op.w & 3;
        let (b_lo, b_hi) = if mem {
            self.vload(op)
        } else {
            let r = op.r1 as usize & 15;
            (self.xmm[r], if mode == VM_V256 { self.ymm_hi[r] } else { 0 })
        };
        let lo = f(sub, self.xmm[s1], b_lo, imm8);
        let d = op.r0 as usize & 15;
        match mode {
            VM_LEG => self.xmm[d] = lo,
            VM_V128 => {
                self.xmm[d] = lo;
                self.clear_ymm_hi_reg(d);
            }
            _ => {
                let hi_imm = if IS_VSHUF {
                    match sub {
                        VS_SHUFPD | VS_BLENDPD | VS_VPERMILPD_I => imm8 >> 2,
                        VS_BLENDPS => imm8 >> 4,
                        _ => imm8,
                    }
                } else {
                    imm8
                };
                let b_lane_hi = if IS_VSHUF && (VS_PSRLW..=VS_PSLLQ).contains(&sub) {
                    b_lo
                } else {
                    b_hi
                };
                let hi = f(sub, self.ymm_hi[s1], b_lane_hi, hi_imm);
                self.xmm[d] = lo;
                self.set_ymm_hi(d, hi);
            }
        }
    }

    /// `(u)comiss/(u)comisd` flags (same as the generic handlers).
    #[inline(always)]
    fn comi(&mut self, fa: f64, fb: f64) {
        self.sf = false;
        self.of = false;
        let (zf, pf, cf) = if fa.is_nan() || fb.is_nan() {
            (true, true, true)
        } else if fa > fb {
            (false, false, false)
        } else if fa < fb {
            (false, false, true)
        } else {
            (true, false, false)
        };
        self.zf = zf;
        self.set_pf(pf);
        self.cf = cf;
    }

    /// 2/3-operand `imul` result + CF/OF (generic semantics; other flags untouched).
    #[inline(always)]
    fn imul_w(&mut self, a: u64, b: u64, w: u8) -> u64 {
        let (a, b) = (sext(a, w) as i64 as i128, sext(b, w) as i64 as i128);
        let prod = a.wrapping_mul(b);
        let fits = match w {
            16 => prod == (prod as i16 as i128),
            32 => prod == (prod as i32 as i128),
            _ => prod == (prod as i64 as i128),
        };
        self.cf = !fits;
        self.of = !fits;
        prod as u64
    }
}

#[inline(always)]
pub(super) fn sext(v: u64, sw: u8) -> u64 {
    match sw {
        8 => v as i8 as i64 as u64,
        16 => v as i16 as i64 as u64,
        32 => v as i32 as i64 as u64,
        _ => v,
    }
}

/// `K_VSHUF_*` sub-ops routed through [`crate::emu::simd_kernels`].
#[inline(never)]
fn vshuf(sub: u8, a: u128, b: u128, imm: u8) -> u128 {
    let ic = imm as u64;
    let bc = b as u64;
    match sub {
        VS_MOV => b,
        VS_ZX32 => b & 0xFFFF_FFFF,
        VS_ZX64 => b & 0xFFFF_FFFF_FFFF_FFFF,
        VS_MRG32 => (a & !0xFFFF_FFFFu128) | (b & 0xFFFF_FFFF),
        VS_MRG64 => (a & !0xFFFF_FFFF_FFFF_FFFFu128) | (b & 0xFFFF_FFFF_FFFF_FFFF),
        VS_XOR => a ^ b,
        VS_OR => sk::v_or(a, b),
        VS_AND => sk::v_and(a, b),
        VS_ANDN => sk::v_andn(a, b),
        VS_UNPCKLBW => sk::punpcklbw(a, b),
        VS_UNPCKLWD => sk::punpcklwd(a, b),
        VS_UNPCKLDQ => sk::punpckldq(a, b),
        VS_UNPCKLQDQ => sk::punpcklqdq(a, b),
        VS_UNPCKHBW => sk::punpckhbw(a, b),
        VS_UNPCKHWD => sk::punpckhwd(a, b),
        VS_UNPCKHDQ => sk::punpckhdq(a, b),
        VS_UNPCKHQDQ => sk::punpckhqdq(a, b),
        VS_PACKSSWB => sk::packsswb(a, b),
        VS_PACKUSWB => sk::packuswb(a, b),
        VS_PACKSSDW => sk::packssdw(a, b),
        VS_PACKUSDW => sk::packusdw(a, b),
        VS_PSHUFD => sk::pshufd(b, imm),
        VS_PSHUFLW => sk::pshuflw(b, imm),
        VS_PSHUFHW => sk::pshufhw(b, imm),
        VS_SHUFPS => sk::shufps(a, b, imm),
        VS_PSHUFB => sk::pshufb(a, b),
        VS_PSRLW_I => sk::pshift_w(b, ic, 1),
        VS_PSRAW_I => sk::pshift_w(b, ic, 2),
        VS_PSLLW_I => sk::pshift_w(b, ic, 0),
        VS_PSRLD_I => sk::pshift_d(b, ic, 1),
        VS_PSRAD_I => sk::pshift_d(b, ic, 2),
        VS_PSLLD_I => sk::pshift_d(b, ic, 0),
        VS_PSRLQ_I => sk::pshift_q(b, ic, true),
        VS_PSLLQ_I => sk::pshift_q(b, ic, false),
        VS_PSRLDQ_I => sk::pshift_dq(b, ic.min(16) as u32, true),
        VS_PSLLDQ_I => sk::pshift_dq(b, ic.min(16) as u32, false),
        VS_PSRLW => sk::pshift_w(a, bc, 1),
        VS_PSRAW => sk::pshift_w(a, bc, 2),
        VS_PSLLW => sk::pshift_w(a, bc, 0),
        VS_PSRLD => sk::pshift_d(a, bc, 1),
        VS_PSRAD => sk::pshift_d(a, bc, 2),
        VS_PSLLD => sk::pshift_d(a, bc, 0),
        VS_PSRLQ => sk::pshift_q(a, bc, true),
        VS_PSLLQ => sk::pshift_q(a, bc, false),
        VS_MOVHLPS => (a & !0xFFFF_FFFF_FFFF_FFFFu128) | (b >> 64),
        VS_SHUFPD => sk::shufpd(a, b, imm),
        VS_VPERMILPS => sk::vpermilps_var(a, b),
        VS_VPERMILPD_I => sk::shufpd(b, b, imm),
        VS_VPERMILPD => sk::vpermilpd_var(a, b),
        VS_PALIGNR => sk::palignr(a, b, imm),
        VS_PBLENDW => sk::pblendw_imm(a, b, imm),
        VS_BLENDPS => sk::blendps_imm(a, b, imm & 0x0F),
        VS_BLENDPD => sk::blendpd_imm(a, b, imm),
        VS_INSERTPS => sk::insertps(a, b, imm, false),
        VS_INSERTPS_M => sk::insertps(a, b, imm, true),
        VS_DDUP => sk::movddup(b),
        _ => b,
    }
}

/// `K_VINT_*` sub-ops routed through [`crate::emu::simd_kernels`].
#[inline(never)]
fn vint(sub: u8, a: u128, b: u128, imm: u8) -> u128 {
    match sub {
        VI_PADDB => sk::paddb(a, b),
        VI_PADDW => sk::paddw(a, b),
        VI_PADDD => sk::paddd(a, b),
        VI_PADDQ => sk::paddq(a, b),
        VI_PSUBB => sk::psubb(a, b),
        VI_PSUBW => sk::psubw(a, b),
        VI_PSUBD => sk::psubd(a, b),
        VI_PSUBQ => sk::psubq(a, b),
        VI_PADDSB => sk::paddsb(a, b),
        VI_PADDSW => sk::paddsw(a, b),
        VI_PADDUSB => sk::paddusb(a, b),
        VI_PADDUSW => sk::paddusw(a, b),
        VI_PSUBSB => sk::psubsb(a, b),
        VI_PSUBSW => sk::psubsw(a, b),
        VI_PSUBUSB => sk::psubusb(a, b),
        VI_PSUBUSW => sk::psubusw(a, b),
        VI_PMULLW => sk::pmullw(a, b),
        VI_PMULHW => sk::pmulhw(a, b),
        VI_PMULHUW => sk::pmulhuw(a, b),
        VI_PMULHRSW => sk::pmulhrsw(a, b),
        VI_PMULLD => sk::pmulld(a, b),
        VI_PMULUDQ => sk::pmuludq(a, b),
        VI_PMULDQ => sk::pmuldq(a, b),
        VI_PMADDWD => sk::pmaddwd(a, b),
        VI_PMADDUBSW => sk::pmaddubsw(a, b),
        VI_PMINUB => sk::pminub(a, b),
        VI_PMAXUB => sk::pmaxub(a, b),
        VI_PMINSW => sk::pminsw(a, b),
        VI_PMAXSW => sk::pmaxsw(a, b),
        VI_PMINSB => sk::pminsb(a, b),
        VI_PMAXSB => sk::pmaxsb(a, b),
        VI_PMINUW => sk::pminuw(a, b),
        VI_PMAXUW => sk::pmaxuw(a, b),
        VI_PMINUD => sk::pminud(a, b),
        VI_PMAXUD => sk::pmaxud(a, b),
        VI_PMINSD => sk::pminsd(a, b),
        VI_PMAXSD => sk::pmaxsd(a, b),
        VI_PAVGB => sk::pavgb(a, b),
        VI_PAVGW => sk::pavgw(a, b),
        VI_PCMPEQB => sk::pcmpeqb(a, b),
        VI_PCMPEQW => sk::pcmpeqw(a, b),
        VI_PCMPEQD => sk::pcmpeqd(a, b),
        VI_PCMPEQQ => sk::pcmpeqq(a, b),
        VI_PCMPGTB => sk::pcmpgtb(a, b),
        VI_PCMPGTW => sk::pcmpgtw(a, b),
        VI_PCMPGTD => sk::pcmpgtd(a, b),
        VI_PCMPGTQ => sk::pcmpgtq(a, b),
        VI_PABSB => sk::pabsb(b),
        VI_PABSW => sk::pabsw(b),
        VI_PABSD => sk::pabsd(b),
        VI_PSIGNB => sk::psignb(a, b),
        VI_PSIGNW => sk::psignw(a, b),
        VI_PSIGND => sk::psignd(a, b),
        VI_PHADDW => sk::phadd_sub_w(a, b, false),
        VI_PHADDD => sk::phadd_sub_d(a, b, false),
        VI_PHSUBW => sk::phadd_sub_w(a, b, true),
        VI_PHSUBD => sk::phadd_sub_d(a, b, true),
        VI_PSADBW => sk::psadbw(a, b),
        VI_VPSLLVD => sk::vpshift_vd(a, b, 0),
        VI_VPSRLVD => sk::vpshift_vd(a, b, 1),
        VI_VPSRAVD => sk::vpshift_vd(a, b, 2),
        VI_VPSLLVQ => sk::vpshift_vq(a, b, false),
        VI_VPSRLVQ => sk::vpshift_vq(a, b, true),
        _ => sk::pclmulqdq(a, b, imm),
    }
}

/// `K_VFP_*` sub-ops routed through [`crate::emu::simd_kernels`].
#[inline(never)]
fn vfp(sub: u8, a: u128, b: u128, imm: u8, mxcsr: u32) -> u128 {
    const M32: u128 = 0xFFFF_FFFF;
    const M64: u128 = 0xFFFF_FFFF_FFFF_FFFF;
    let f32b = |v: u128| f32::from_bits(v as u32);
    let f64b = |v: u128| f64::from_bits(v as u64);
    match sub {
        0..=5 => (a & !M32) | sse_arith32(sub, f32b(a), f32b(b)).to_bits() as u128,
        6..=11 => (a & !M64) | sse_arith64(sub - 6, f64b(a), f64b(b)).to_bits() as u128,
        12..=17 => sk::fp_bin_ps(sub - 12, a, b),
        18..=23 => sk::fp_bin_pd(sub - 18, a, b),
        VF_SQRTSS => (a & !M32) | sse_sqrt32(f32b(b)).to_bits() as u128,
        VF_SQRTSD => (a & !M64) | sse_sqrt64(f64b(b)).to_bits() as u128,
        VF_SQRTPS => sk::sqrt_ps(b),
        VF_SQRTPD => sk::sqrt_pd(b),
        VF_CVTSS2SD => (a & !M64) | (f32b(b) as f64).to_bits() as u128,
        VF_CVTSD2SS => (a & !M32) | (f64b(b) as f32).to_bits() as u128,
        VF_CVTDQ2PS => sk::cvtdq2ps(b),
        VF_CVTTPS2DQ => sk::cvtps2dq(b, true),
        VF_CVTPS2DQ => sk::cvtps2dq(b, false),
        VF_RCPSS | VF_RSQRTSS => (a & !M32) | (sk::rcp_rsqrt_ps(b & M32, sub == VF_RSQRTSS) & M32),
        VF_RCPPS => sk::rcp_rsqrt_ps(b, false),
        VF_RSQRTPS => sk::rcp_rsqrt_ps(b, true),
        VF_ADDSUBPS => sk::addsub_ps(a, b),
        VF_ADDSUBPD => sk::addsub_pd(a, b),
        VF_HADDPS => sk::hadd_sub_ps(a, b, true),
        VF_HSUBPS => sk::hadd_sub_ps(a, b, false),
        VF_HADDPD => sk::hadd_sub_pd(a, b, true),
        VF_HSUBPD => sk::hadd_sub_pd(a, b, false),
        VF_ROUNDSS => (a & !M32) | sk::round_f32(f32b(b), imm, mxcsr).to_bits() as u128,
        VF_ROUNDSD => (a & !M64) | sk::round_f64(f64b(b), imm, mxcsr).to_bits() as u128,
        VF_ROUNDPS => sk::round_ps(b, imm, mxcsr),
        VF_ROUNDPD => sk::round_pd(b, imm, mxcsr),
        VF_CMPSS => {
            let m = if sse_cmp_pred(f32b(a) as f64, f32b(b) as f64, imm) { M32 } else { 0 };
            (a & !M32) | m
        }
        VF_CMPSD => {
            let m = if sse_cmp_pred(f64b(a), f64b(b), imm) { M64 } else { 0 };
            (a & !M64) | m
        }
        VF_CMPPS => sk::cmp_ps(a, b, imm),
        VF_CMPPD => sk::cmp_pd(a, b, imm),
        VF_DPPS => sk::dp_ps(a, b, imm),
        _ => b,
    }
}

/// Run fast ops of one decoded block starting at `op` (a non-generic op; `cpu.rip` already
/// holds its next IP) until one leaves straight-line flow, the next slot is generic or not
/// sequential, the block ends (`left` = slots after `op`), or `budget` ops have run.
/// Fused `cmp/test + jcc` pairs execute both slots. Returns the last op's effect, the number
/// of ops executed (>= 1) and the last op executed.
///
/// Kept out of line and small so the loop state stays in registers (the giant
/// `emu_run_full` frame spills everything).
#[inline(never)]
pub(in crate::emu) unsafe fn run_chain(cpu: &mut Cpu, mut op: *const FastOp, mut left: usize, budget: u32) -> (Fx, u32, *const FastOp) {
    let mut n = 0u32;
    loop {
        let fx = exec(cpu, &*op);
        n += 1;
        match fx {
            Fx::Done => {
                if left != 0 && n < budget {
                    let nx = op.add(1);
                    if (*nx).kind != K_GENERIC && (*nx).ip == cpu.rip {
                        op = nx;
                        left -= 1;
                        cpu.rip = (*nx).ip.wrapping_add((*nx).len as u64);
                        continue;
                    }
                }
                return (Fx::Done, n, op);
            }
            Fx::FuseJcc => {
                // `fuse` guarantees the next slot of this block is the paired `K_JCC`.
                // Evaluate the branch inline without a recursive `exec` call; when not taken,
                // keep executing straight through the rest of the block!
                let j = op.add(1);
                let jop = &*j;
                cpu.rip = jop.ip.wrapping_add(jop.len as u64);
                n += 1;
                if cpu.eval_cc_u8(jop.x) {
                    if jop.len <= 2 {
                        cpu.rip = jop.imm;
                        return (Fx::Done, n, j);
                    }
                    return (Fx::JccNear(jop.imm), n, j);
                }
                left -= 1;
                if left != 0 && n < budget {
                    let nx = j.add(1);
                    if (*nx).kind != K_GENERIC && (*nx).ip == cpu.rip {
                        op = nx;
                        left -= 1;
                        cpu.rip = (*nx).ip.wrapping_add((*nx).len as u64);
                        continue;
                    }
                }
                return (Fx::Done, n, j);
            }
            other => return (other, n, op),
        }
    }
}

/// Execute a non-generic fast op. `cpu.rip` must already hold the next instruction's IP.
#[inline(always)]
pub(in crate::emu) unsafe fn exec(cpu: &mut Cpu, op: &FastOp) -> Fx {
    let w = op.w;
    match op.kind {
        K_NOP => {}
        K_MOV_RR => {
            let v = cpu.rget(op.r1, w);
            cpu.rset(op.r0, w, v);
        }
        K_MOV_RM => {
            let v = cpu.read_mem(cpu.faddr(op), w as u32);
            cpu.rset(op.r0, w, v);
        }
        K_MOV_MR => {
            let v = cpu.rget(op.r1, w);
            cpu.write_mem(cpu.faddr(op), w as u32, v);
        }
        K_MOV_RI => cpu.rset(op.r0, w, op.imm),
        K_MOV_MI => cpu.write_mem(cpu.faddr(op), w as u32, op.imm),
        K_MOVZX_RR => {
            let v = cpu.rget(op.r1, op.x & 0x3F);
            cpu.rset(op.r0, w, v);
        }
        K_MOVZX_RM => {
            let v = cpu.read_mem(cpu.faddr(op), (op.x & 0x3F) as u32);
            cpu.rset(op.r0, w, v);
        }
        K_MOVSX_RR => {
            let sw = op.x & 0x3F;
            let v = sext(cpu.rget(op.r1, sw), sw);
            cpu.rset(op.r0, w, v);
        }
        K_MOVSX_RM => {
            let sw = op.x & 0x3F;
            let v = sext(cpu.read_mem(cpu.faddr(op), sw as u32), sw);
            cpu.rset(op.r0, w, v);
        }
        K_LEA => {
            let a = cpu.faddr(op);
            cpu.rset(op.r0, w, a);
        }
        K_ALU_RR => {
            let a = cpu.rget(op.r0, w);
            let b = cpu.rget(op.r1, w);
            if let Some(r) = cpu.alu(op.x & 0x3F, a, b, w as u32) {
                cpu.rset(op.r0, w, r);
            }
        }
        K_ALU_RI => {
            let a = cpu.rget(op.r0, w);
            if let Some(r) = cpu.alu(op.x & 0x3F, a, op.imm, w as u32) {
                cpu.rset(op.r0, w, r);
            }
        }
        K_ALU_RM => {
            let b = cpu.read_mem(cpu.faddr(op), w as u32);
            let a = cpu.rget(op.r0, w);
            if let Some(r) = cpu.alu(op.x & 0x3F, a, b, w as u32) {
                cpu.rset(op.r0, w, r);
            }
        }
        K_ALU_MR => {
            let addr = cpu.faddr(op);
            let b = cpu.rget(op.r1, w);
            let a = cpu.read_mem(addr, w as u32);
            if let Some(r) = cpu.alu(op.x & 0x3F, a, b, w as u32) {
                cpu.write_mem(addr, w as u32, r);
            }
        }
        K_ALU_MI => {
            let addr = cpu.faddr(op);
            let a = cpu.read_mem(addr, w as u32);
            if let Some(r) = cpu.alu(op.x & 0x3F, a, op.imm, w as u32) {
                cpu.write_mem(addr, w as u32, r);
            }
        }
        K_INC_R => {
            let saved_cf = cpu.cf;
            let a = cpu.rget(op.r0, w);
            let r = cpu.update_flags_add(a, 1, w as u32);
            cpu.rset(op.r0, w, r);
            cpu.cf = saved_cf;
        }
        K_DEC_R => {
            let saved_cf = cpu.cf;
            let a = cpu.rget(op.r0, w);
            let r = cpu.update_flags_sub(a, 1, w as u32);
            cpu.rset(op.r0, w, r);
            cpu.cf = saved_cf;
        }
        K_SHIFT_RI => {
            let a = cpu.rget(op.r0, w);
            let r = cpu.shift(op.x & 0x3F, a, op.imm as u32, w);
            cpu.rset(op.r0, w, r);
        }
        K_SHIFT_RCL => {
            let mask = if w == 64 { 63 } else { 31 };
            let cnt = (cpu.gpr[1] as u8 as u32) & mask;
            if cnt > 0 {
                let a = cpu.rget(op.r0, w);
                let r = cpu.shift(op.x & 0x3F, a, cnt, w);
                cpu.rset(op.r0, w, r);
            } else if w == 32 {
                // Count 0: flags unchanged, but a 32-bit destination is still zero-extended.
                cpu.rset(op.r0, 32, cpu.rget(op.r0, 32));
            }
        }
        K_PUSH_R => {
            let v = cpu.gpr[op.r0 as usize & 15];
            cpu.gpr[4] = cpu.gpr[4].wrapping_sub(8);
            cpu.write_mem(cpu.gpr[4], 64, v);
        }
        K_POP_R => {
            let v = cpu.read_mem(cpu.gpr[4], 64);
            cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
            cpu.rset(op.r0, 64, v);
        }
        K_RET => {
            let rsp = cpu.gpr[4];
            let ret_addr = cpu.read_mem(rsp, 64);
            cpu.gpr[4] = rsp.wrapping_add(8 + op.imm);
            cpu.rip = ret_addr;
        }
        K_JCC => {
            if cpu.eval_cc_u8(op.x) {
                if op.len <= 2 {
                    cpu.rip = op.imm;
                } else {
                    return Fx::JccNear(op.imm);
                }
            }
        }
        K_JMP => return Fx::Jmp(op.imm),
        K_CALL => return Fx::Call(op.imm),
        K_CALL_R => return Fx::Call(cpu.gpr[op.r0 as usize & 15]),
        K_CALL_M => return Fx::Call(cpu.read_mem(cpu.faddr(op), 64)),
        K_JMP_R => return Fx::Jmp(cpu.gpr[op.r0 as usize & 15]),
        K_JMP_M => return Fx::Jmp(cpu.read_mem(cpu.faddr(op), 64)),
        K_CMPJ_RR => {
            let a = cpu.rget(op.r0, w);
            let b = cpu.rget(op.r1, w);
            cpu.cmp_or_test(op.x & 0x3F, a, b, w as u32);
            return Fx::FuseJcc;
        }
        K_CMPJ_RI => {
            let a = cpu.rget(op.r0, w);
            cpu.cmp_or_test(op.x & 0x3F, a, op.imm, w as u32);
            return Fx::FuseJcc;
        }
        K_CMPJ_RM => {
            let b = cpu.read_mem(cpu.faddr(op), w as u32);
            let a = cpu.rget(op.r0, w);
            cpu.cmp_or_test(op.x & 0x3F, a, b, w as u32);
            return Fx::FuseJcc;
        }
        K_CMOV_RR => {
            if cpu.eval_cc_u8(op.x) {
                let v = cpu.rget(op.r1, w);
                cpu.rset(op.r0, w, v);
            } else if w == 32 {
                let cur = cpu.rget(op.r0, 32);
                cpu.rset(op.r0, 32, cur);
            }
        }
        K_CMOV_RM => {
            if cpu.eval_cc_u8(op.x) {
                let v = cpu.read_mem(cpu.faddr(op), w as u32);
                cpu.rset(op.r0, w, v);
            } else if w == 32 {
                let cur = cpu.rget(op.r0, 32);
                cpu.rset(op.r0, 32, cur);
            }
        }
        K_SETCC_R => {
            let v = cpu.eval_cc_u8(op.x) as u64;
            cpu.rset(op.r0, 8, v);
        }
        _ => {
            if op.kind >= K_VSHUF_RR && exec_ext(cpu, op) {
                return Fx::Yield;
            }
        }
    }
    Fx::Done
}

/// Handlers for kinds >= [`K_VSHUF_RR`] (SSE/AVX and the extra integer ops). Out of line so
/// the hot integer `exec`/`run_chain` code stays compact. Returns `true` when the caller must
/// yield out of a fast burst/chain (e.g. after a fault dispatch or non-AFP mixed MXCSR switch).
#[inline(never)]
pub(in crate::emu) unsafe fn exec_ext(cpu: &mut Cpu, op: &FastOp) -> bool {
    let w = op.w;
    match op.kind {
        K_VSHUF_RR => cpu.vexec::<true, _>(op, false, vshuf),
        K_VSHUF_RM => cpu.vexec::<true, _>(op, true, vshuf),
        K_VINT_RR => cpu.vexec::<false, _>(op, false, vint),
        K_VINT_RM => cpu.vexec::<false, _>(op, true, vint),
        K_VFP_RR => {
            let mxcsr = cpu.jx.mxcsr;
            cpu.vexec::<false, _>(op, false, |s, a, b, imm| vfp(s, a, b, imm, mxcsr));
        }
        K_VFP_RM => {
            let mxcsr = cpu.jx.mxcsr;
            cpu.vexec::<false, _>(op, true, |s, a, b, imm| vfp(s, a, b, imm, mxcsr));
        }
        K_VST => {
            let a = cpu.faddr(op);
            let s = op.r1 as usize & 15;
            match op.w >> 4 {
                2 => cpu.write_mem(a, 32, cpu.xmm[s] as u32 as u64),
                3 => cpu.write_mem(a, 64, cpu.xmm[s] as u64),
                4 => cpu.write_u128(a, cpu.xmm[s]),
                6 => cpu.write_mem(a, 64, (cpu.xmm[s] >> 64) as u64),
                _ => {
                    cpu.write_u128(a, cpu.xmm[s]);
                    cpu.write_u128(a.wrapping_add(16), cpu.ymm_hi[s]);
                }
            }
        }
        K_MOVD_XG => {
            let v = cpu.gpr[op.r1 as usize & 15];
            let d = op.r0 as usize & 15;
            cpu.xmm[d] = if op.imm == 64 { v as u128 } else { v as u32 as u128 };
            if op.w & 3 == VM_V128 {
                cpu.clear_ymm_hi_reg(d);
            }
        }
        K_MOVD_GX => {
            let v = cpu.xmm[op.r1 as usize & 15] as u64;
            cpu.rset(op.r0, w, v);
        }
        K_CVTSI_R | K_CVTSI_M => {
            let width = op.imm as u8;
            let v = if op.kind == K_CVTSI_R { cpu.rget(op.r1, width) } else { cpu.read_mem(cpu.faddr(op), width as u32) };
            let iv = if width == 64 { v as i64 } else { v as u32 as i32 as i64 };
            let base = cpu.xmm[(op.imm >> 8) as usize & 15];
            let d = op.r0 as usize & 15;
            cpu.xmm[d] = if op.x & 0x3F == 0 {
                (base & !0xFFFF_FFFF) | ((iv as f32).to_bits() as u128)
            } else {
                (base & !0xFFFF_FFFF_FFFF_FFFF) | ((iv as f64).to_bits() as u128)
            };
            if op.w & 3 == VM_V128 {
                cpu.clear_ymm_hi_reg(d);
            }
        }
        K_COMI_RR | K_COMI_RM => {
            let sd = op.x & 0x3F != 0;
            let a = cpu.xmm[op.r0 as usize & 15];
            let b = if op.kind == K_COMI_RR {
                cpu.xmm[op.r1 as usize & 15]
            } else {
                cpu.read_mem(cpu.faddr(op), if sd { 64 } else { 32 }) as u128
            };
            if sd {
                cpu.comi(f64::from_bits(a as u64), f64::from_bits(b as u64));
            } else {
                cpu.comi(f32::from_bits(a as u32) as f64, f32::from_bits(b as u32) as f64);
            }
        }
        K_CVTF2I_R | K_CVTF2I_M => {
            let sd = op.x & 1 != 0;
            let trunc = op.x & 2 != 0;
            let s = if op.kind == K_CVTF2I_R {
                cpu.xmm[op.r1 as usize & 15] as u64
            } else {
                cpu.read_mem(cpu.faddr(op), if sd { 64 } else { 32 })
            };
            let f = if sd { f64::from_bits(s) } else { f32::from_bits(s as u32) as f64 };
            let iv = if w == 64 { sse_cvt_i64(f, trunc) } else { sse_cvt_i32(f, trunc) as u64 };
            cpu.rset(op.r0, w, iv);
        }
        K_IMUL_RR => {
            let r = cpu.imul_w(cpu.rget(op.r0, w), cpu.rget(op.r1, w), w);
            cpu.rset(op.r0, w, r);
        }
        K_IMUL_RM => {
            let b = cpu.read_mem(cpu.faddr(op), w as u32);
            let r = cpu.imul_w(cpu.rget(op.r0, w), b, w);
            cpu.rset(op.r0, w, r);
        }
        K_IMUL_RRI => {
            let r = cpu.imul_w(cpu.rget(op.r1, w), op.imm, w);
            cpu.rset(op.r0, w, r);
        }
        K_IMUL_RMI => {
            let a = cpu.read_mem(cpu.faddr(op), w as u32);
            let r = cpu.imul_w(a, op.imm, w);
            cpu.rset(op.r0, w, r);
        }
        K_NEG_R => {
            let a = cpu.rget(op.r0, w);
            let r = cpu.update_flags_sub(0, a, w as u32);
            cpu.cf = a != 0;
            cpu.rset(op.r0, w, r);
        }
        K_NOT_R => {
            let a = cpu.rget(op.r0, w);
            cpu.rset(op.r0, w, !a);
        }
        K_INC_M | K_DEC_M => {
            let saved_cf = cpu.cf;
            let addr = cpu.faddr(op);
            let a = cpu.read_mem(addr, w as u32);
            let r = if op.kind == K_INC_M { cpu.update_flags_add(a, 1, w as u32) } else { cpu.update_flags_sub(a, 1, w as u32) };
            cpu.write_mem(addr, w as u32, r);
            cpu.cf = saved_cf;
        }
        K_CX => match op.x & 0x3F {
            0 => cpu.gpr[0] = (cpu.gpr[0] as u16 as i16 as i32 as u32) as u64,
            1 => cpu.gpr[0] = cpu.gpr[0] as i32 as i64 as u64,
            2 => cpu.gpr[2] = if (cpu.gpr[0] as i32) < 0 { 0xFFFF_FFFF } else { 0 },
            3 => cpu.gpr[2] = if (cpu.gpr[0] as i64) < 0 { u64::MAX } else { 0 },
            4 => cpu.gpr[0] = (cpu.gpr[0] & !0xFFFF) | ((cpu.gpr[0] as u8 as i8 as i16 as u16) as u64),
            _ => {
                let hi = if (cpu.gpr[0] as i16) < 0 { 0xFFFFu64 } else { 0 };
                cpu.gpr[2] = (cpu.gpr[2] & !0xFFFF) | hi;
            }
        },
        K_VZU => cpu.clear_all_ymm_hi(),
        K_BSF_RR | K_BSF_RM => {
            let v = if op.kind == K_BSF_RR { cpu.rget(op.r1, w) } else { cpu.read_mem(cpu.faddr(op), w as u32) };
            cpu.cf = v == 0;
            cpu.zf = v == 0;
            if v != 0 {
                cpu.rset(op.r0, w, v.trailing_zeros() as u64);
            }
        }
        K_BT_RR | K_BT_RI => {
            let idx = if op.kind == K_BT_RR { cpu.rget(op.r1, w) } else { op.imm };
            cpu.cf = (cpu.rget(op.r0, w) >> (idx as u32 & (w as u32 - 1))) & 1 != 0;
        }
        K_ROT_RI | K_ROT_RCL => {
            let mask = if w == 64 { 63 } else { 31 };
            let cnt = if op.kind == K_ROT_RI { op.imm as u32 } else { (cpu.gpr[1] as u8 as u32) & mask };
            if cnt != 0 {
                let a = cpu.rget(op.r0, w);
                let ror = op.x & 0x3F != 0;
                let res = match (w, ror) {
                    (8, false) => (a as u8).rotate_left(cnt) as u64,
                    (16, false) => (a as u16).rotate_left(cnt) as u64,
                    (32, false) => (a as u32).rotate_left(cnt) as u64,
                    (_, false) => a.rotate_left(cnt),
                    (8, true) => (a as u8).rotate_right(cnt) as u64,
                    (16, true) => (a as u16).rotate_right(cnt) as u64,
                    (32, true) => (a as u32).rotate_right(cnt) as u64,
                    (_, true) => a.rotate_right(cnt),
                };
                cpu.rset(op.r0, w, res);
                let msb = (res >> (w - 1)) & 1 != 0;
                if ror {
                    cpu.cf = msb;
                    cpu.of = msb ^ ((res >> (w - 2)) & 1 != 0);
                } else {
                    cpu.cf = res & 1 != 0;
                    cpu.of = msb ^ cpu.cf;
                }
            } else if w == 32 {
                let cur = cpu.rget(op.r0, 32);
                cpu.rset(op.r0, 32, cur);
            }
        }
        K_XCHG_RR => {
            let a = cpu.rget(op.r0, w);
            let b = cpu.rget(op.r1, w);
            cpu.rset(op.r0, w, b);
            cpu.rset(op.r1, w, a);
        }
        K_BSWAP_R => {
            let v = cpu.rget(op.r0, w);
            let res = if w == 64 { v.swap_bytes() } else { (v as u32).swap_bytes() as u64 };
            cpu.rset(op.r0, w, res);
        }
        K_BSR_RR | K_BSR_RM => {
            let v = if op.kind == K_BSR_RR { cpu.rget(op.r1, w) } else { cpu.read_mem(cpu.faddr(op), w as u32) };
            cpu.zf = v == 0;
            if v != 0 {
                cpu.rset(op.r0, w, 63u64.wrapping_sub(v.leading_zeros() as u64));
            }
        }
        K_TZCNT_RR | K_TZCNT_RM => {
            let v = if op.kind == K_TZCNT_RR { cpu.rget(op.r1, w) } else { cpu.read_mem(cpu.faddr(op), w as u32) };
            if v == 0 {
                cpu.zf = false;
                cpu.cf = true;
                cpu.rset(op.r0, w, w as u64);
            } else {
                let idx = v.trailing_zeros() as u64;
                cpu.zf = idx == 0;
                cpu.cf = false;
                cpu.rset(op.r0, w, idx);
            }
        }
        K_LZCNT_RR | K_LZCNT_RM => {
            let v = if op.kind == K_LZCNT_RR { cpu.rget(op.r1, w) } else { cpu.read_mem(cpu.faddr(op), w as u32) };
            let res = match w {
                16 => (v as u16).leading_zeros() as u64,
                32 => (v as u32).leading_zeros() as u64,
                _ => v.leading_zeros() as u64,
            };
            cpu.rset(op.r0, w, res);
            cpu.zf = res == 0;
            cpu.cf = res == w as u64;
        }
        K_POPCNT_RR | K_POPCNT_RM => {
            let v = if op.kind == K_POPCNT_RR { cpu.rget(op.r1, w) } else { cpu.read_mem(cpu.faddr(op), w as u32) };
            let res = v.count_ones() as u64;
            cpu.rset(op.r0, w, res);
            cpu.zf = res == 0;
            cpu.cf = false;
            cpu.sf = false;
            cpu.of = false;
            cpu.set_pf(false);
        }
        K_BIT_RR | K_BIT_RI => {
            let idx = if op.kind == K_BIT_RR { cpu.rget(op.r1, w) } else { op.imm };
            let b = (idx as u32) & (w as u32 - 1);
            let a = cpu.rget(op.r0, w);
            cpu.cf = ((a >> b) & 1) != 0;
            let res = match op.x & 0x3F {
                0 => a | (1u64 << b),
                1 => a & !(1u64 << b),
                _ => a ^ (1u64 << b),
            };
            cpu.rset(op.r0, w, res);
        }
        K_SHD_RI | K_SHD_RCL => {
            let mask = if w == 64 { 63u32 } else { 31u32 };
            let cnt = if op.kind == K_SHD_RI { op.imm as u32 } else { (cpu.gpr[1] as u8 as u32) & mask };
            if cnt != 0 {
                let dst = cpu.rget(op.r0, w);
                let src = cpu.rget(op.r1, w);
                let shrd = (op.x & 0x3F) != 0;
                let wb = w as u32;
                let res = if shrd {
                    if wb == 64 {
                        (dst >> cnt) | (src.wrapping_shl(64 - cnt))
                    } else if wb == 32 {
                        (((dst as u32) >> cnt) | ((src as u32).wrapping_shl(32 - cnt))) as u64
                    } else {
                        (((dst as u16) >> cnt) | ((src as u16).wrapping_shl(16 - cnt))) as u64
                    }
                } else if wb == 64 {
                    (dst << cnt) | (src.wrapping_shr(64 - cnt))
                } else if wb == 32 {
                    (((dst as u32) << cnt) | ((src as u32).wrapping_shr(32 - cnt))) as u64
                } else {
                    (((dst as u16) << cnt) | ((src as u16).wrapping_shr(16 - cnt))) as u64
                };
                cpu.update_flags_logic(res, wb);
                cpu.cf = if shrd {
                    ((dst >> (cnt - 1)) & 1) != 0
                } else {
                    ((dst >> (wb - cnt)) & 1) != 0
                };
                cpu.rset(op.r0, w, res);
            }
        }
        K_MUL1_R | K_MUL1_M => {
            let src = if op.kind == K_MUL1_R { cpu.rget(op.r0, w) } else { cpu.read_mem(cpu.faddr(op), w as u32) };
            match op.x & 0x3F {
                0 => {
                    if w == 64 {
                        let prod = (cpu.gpr[0] as u128).wrapping_mul(src as u128);
                        cpu.gpr[0] = prod as u64;
                        cpu.gpr[2] = (prod >> 64) as u64;
                        cpu.cf = cpu.gpr[2] != 0;
                        cpu.of = cpu.cf;
                    } else if w == 32 {
                        let prod = (cpu.gpr[0] as u32 as u64).wrapping_mul(src as u32 as u64);
                        cpu.gpr[0] = prod as u32 as u64;
                        cpu.gpr[2] = (prod >> 32) as u32 as u64;
                        cpu.cf = cpu.gpr[2] != 0;
                        cpu.of = cpu.cf;
                    } else if w == 16 {
                        let prod = (cpu.gpr[0] as u16 as u32) * (src as u16 as u32);
                        cpu.set_reg(Register::AX, prod as u64);
                        cpu.set_reg(Register::DX, (prod >> 16) as u64);
                        cpu.cf = (prod >> 16) != 0;
                        cpu.of = cpu.cf;
                    } else {
                        let prod = (cpu.gpr[0] as u8 as u16) * (src as u8 as u16);
                        cpu.set_reg(Register::AX, prod as u64);
                        cpu.cf = (prod >> 8) != 0;
                        cpu.of = cpu.cf;
                    }
                }
                1 => {
                    if w == 64 {
                        let prod = (cpu.gpr[0] as i64 as i128).wrapping_mul(src as i64 as i128);
                        cpu.gpr[0] = prod as i64 as u64;
                        cpu.gpr[2] = (prod >> 64) as i64 as u64;
                        cpu.cf = prod != (prod as i64 as i128);
                        cpu.of = cpu.cf;
                    } else if w == 32 {
                        let prod = (cpu.gpr[0] as u32 as i32 as i64).wrapping_mul(src as u32 as i32 as i64);
                        cpu.gpr[0] = prod as i32 as u32 as u64;
                        cpu.gpr[2] = ((prod >> 32) as i32 as u32) as u64;
                        cpu.cf = prod != (prod as i32 as i64);
                        cpu.of = cpu.cf;
                    } else if w == 16 {
                        let prod = (cpu.gpr[0] as u16 as i16 as i32) * (src as u16 as i16 as i32);
                        cpu.set_reg(Register::AX, prod as u64);
                        cpu.set_reg(Register::DX, (prod >> 16) as u64);
                        cpu.cf = prod != (prod as i16 as i32);
                        cpu.of = cpu.cf;
                    } else {
                        let prod = (cpu.gpr[0] as u8 as i8 as i16) * (src as u8 as i8 as i16);
                        cpu.set_reg(Register::AX, prod as u64);
                        cpu.cf = prod != (prod as i8 as i16);
                        cpu.of = cpu.cf;
                    }
                }
                2 => {
                    if w == 64 && src != 0 {
                        let n = ((cpu.gpr[2] as u128) << 64) | (cpu.gpr[0] as u128);
                        cpu.gpr[0] = (n / (src as u128)) as u64;
                        cpu.gpr[2] = (n % (src as u128)) as u64;
                    } else if w == 32 && (src as u32) != 0 {
                        let n = ((cpu.gpr[2] as u32 as u64) << 32) | (cpu.gpr[0] as u32 as u64);
                        let d32 = src as u32 as u64;
                        cpu.gpr[0] = (n / d32) as u32 as u64;
                        cpu.gpr[2] = (n % d32) as u32 as u64;
                    } else if w == 16 && (src as u16) != 0 {
                        let n = ((cpu.gpr[2] as u16 as u32) << 16) | (cpu.gpr[0] as u16 as u32);
                        let d16 = src as u16 as u32;
                        cpu.set_reg(Register::AX, (n / d16) as u64);
                        cpu.set_reg(Register::DX, (n % d16) as u64);
                    } else if w == 8 && (src as u8) != 0 {
                        let n = cpu.gpr[0] as u16;
                        let d8 = src as u8 as u16;
                        cpu.set_reg(Register::AL, (n / d8) as u64);
                        cpu.set_reg(Register::AH, (n % d8) as u64);
                    }
                }
                _ => {
                    if w == 64 && src != 0 {
                        let n = (((cpu.gpr[2] as u128) << 64) | (cpu.gpr[0] as u128)) as i128;
                        let d64 = src as i64 as i128;
                        cpu.gpr[0] = n.wrapping_div(d64) as i64 as u64;
                        cpu.gpr[2] = n.wrapping_rem(d64) as i64 as u64;
                    } else if w == 32 && (src as u32) != 0 {
                        let n = (((cpu.gpr[2] as u32 as u64) << 32) | (cpu.gpr[0] as u32 as u64)) as i64;
                        let d32 = src as u32 as i32 as i64;
                        cpu.gpr[0] = n.wrapping_div(d32) as i32 as u32 as u64;
                        cpu.gpr[2] = n.wrapping_rem(d32) as i32 as u32 as u64;
                    } else if w == 16 && (src as u16) != 0 {
                        let n = (((cpu.gpr[2] as u16 as u32) << 16) | (cpu.gpr[0] as u16 as u32)) as i32;
                        let d16 = src as u16 as i16 as i32;
                        cpu.set_reg(Register::AX, n.wrapping_div(d16) as u64);
                        cpu.set_reg(Register::DX, n.wrapping_rem(d16) as u64);
                    } else if w == 8 && (src as u8) != 0 {
                        let n = cpu.gpr[0] as u16 as i16;
                        let d8 = src as u8 as i8 as i16;
                        cpu.set_reg(Register::AL, n.wrapping_div(d8) as u64);
                        cpu.set_reg(Register::AH, n.wrapping_rem(d8) as u64);
                    }
                }
            }
        }
        K_XADD_RR => {
            let a = cpu.rget(op.r0, w);
            let b = cpu.rget(op.r1, w);
            let res = cpu.update_flags_add(a, b, w as u32);
            cpu.rset(op.r0, w, res);
            cpu.rset(op.r1, w, a);
        }
        K_CMPXCHG_RR => {
            let acc = cpu.rget(0, w);
            let dst = cpu.rget(op.r0, w);
            let src = cpu.rget(op.r1, w);
            let _ = cpu.update_flags_sub(acc, dst, w as u32);
            if acc == dst {
                cpu.zf = true;
                cpu.rset(op.r0, w, src);
            } else {
                cpu.zf = false;
                cpu.rset(0, w, dst);
            }
        }
        K_SETCC_M => {
            let v = cpu.eval_cc_u8(op.x) as u64;
            cpu.write_mem(cpu.faddr(op), 8, v);
        }
        K_PUSH_I => {
            cpu.gpr[4] = cpu.gpr[4].wrapping_sub(8);
            cpu.write_mem(cpu.gpr[4], 64, op.imm);
        }
        K_PUSH_M => {
            let v = cpu.read_mem(cpu.faddr(op), 64);
            cpu.gpr[4] = cpu.gpr[4].wrapping_sub(8);
            cpu.write_mem(cpu.gpr[4], 64, v);
        }
        K_POP_M => {
            let v = cpu.read_mem(cpu.gpr[4], 64);
            cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
            let a = cpu.faddr(op);
            cpu.write_mem(a, 64, v);
        }
        K_LEAVE => {
            cpu.gpr[4] = cpu.gpr[5];
            let v = cpu.read_mem(cpu.gpr[4], 64);
            cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
            cpu.rset(5, 64, v);
        }
        K_FLAG_OP => match op.x & 0x3F {
            0 => cpu.cf = false,
            1 => cpu.cf = true,
            2 => cpu.cf = !cpu.cf,
            3 => cpu.df = false,
            4 => cpu.df = true,
            _ => core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst),
        },
        K_SHIFT_MI | K_SHIFT_MCL => {
            let mask = if w == 64 { 63 } else { 31 };
            let cnt = if op.kind == K_SHIFT_MI { op.imm as u32 } else { (cpu.gpr[1] as u8 as u32) & mask };
            let addr = cpu.faddr(op);
            let a = cpu.read_mem(addr, w as u32);
            if cnt > 0 {
                let r = cpu.shift(op.x & 0x3F, a, cnt, w);
                cpu.write_mem(addr, w as u32, r);
            }
        }
        K_ROT_MI | K_ROT_MCL => {
            let mask = if w == 64 { 63 } else { 31 };
            let cnt = if op.kind == K_ROT_MI { op.imm as u32 } else { (cpu.gpr[1] as u8 as u32) & mask };
            let addr = cpu.faddr(op);
            let a = cpu.read_mem(addr, w as u32);
            if cnt != 0 {
                let ror = op.x & 0x3F != 0;
                let res = match (w, ror) {
                    (8, false) => (a as u8).rotate_left(cnt) as u64,
                    (16, false) => (a as u16).rotate_left(cnt) as u64,
                    (32, false) => (a as u32).rotate_left(cnt) as u64,
                    (_, false) => a.rotate_left(cnt),
                    (8, true) => (a as u8).rotate_right(cnt) as u64,
                    (16, true) => (a as u16).rotate_right(cnt) as u64,
                    (32, true) => (a as u32).rotate_right(cnt) as u64,
                    (_, true) => a.rotate_right(cnt),
                };
                cpu.write_mem(addr, w as u32, res);
                let msb = (res >> (w - 1)) & 1 != 0;
                if ror {
                    cpu.cf = msb;
                    cpu.of = msb ^ ((res >> (w - 2)) & 1 != 0);
                } else {
                    cpu.cf = res & 1 != 0;
                    cpu.of = msb ^ cpu.cf;
                }
            }
        }
        K_MOVBE_RM => {
            let v = cpu.read_mem(cpu.faddr(op), w as u32);
            let res = match w {
                16 => (v as u16).swap_bytes() as u64,
                32 => (v as u32).swap_bytes() as u64,
                _ => v.swap_bytes(),
            };
            cpu.rset(op.r0, w, res);
        }
        K_MOVBE_MR => {
            let v = cpu.rget(op.r1, w);
            let res = match w {
                16 => (v as u16).swap_bytes() as u64,
                32 => (v as u32).swap_bytes() as u64,
                _ => v.swap_bytes(),
            };
            cpu.write_mem(cpu.faddr(op), w as u32, res);
        }
        K_MOVMSK => {
            let s = op.r1 as usize & 15;
            let lo = cpu.xmm[s];
            let res = match op.x & 0x3F {
                0 => sk::pmovmskb(lo),
                1 => sk::pmovmskb_ymm(lo, cpu.ymm_hi[s]),
                2 => sk::movmskps(lo),
                3 => sk::movmskps_ymm(lo, cpu.ymm_hi[s]),
                4 => sk::movmskpd(lo),
                _ => sk::movmskpd_ymm(lo, cpu.ymm_hi[s]),
            };
            cpu.rset(op.r0, w, res);
        }
        K_PINSR => {
            let lg = (op.w >> 4) as u32;
            let ebits = 8u32 << lg;
            let lane = (op.imm & 0xFF) as u32;
            let s1 = ((op.imm >> 8) as usize) & 15;
            let d = op.r0 as usize & 15;
            let src = if op.x & 0x20 != 0 {
                cpu.read_mem(cpu.faddr(op), ebits)
            } else {
                cpu.rget(op.r1, ebits as u8)
            };
            let mask = if ebits == 64 { u64::MAX as u128 } else { (1u128 << ebits) - 1 };
            let sh = lane * ebits;
            cpu.xmm[d] = (cpu.xmm[s1] & !(mask << sh)) | (((src as u128) & mask) << sh);
            if op.w & 3 == VM_V128 {
                cpu.ymm_hi[d] = 0;
            }
        }
        K_PEXTR => {
            let ebits = w as u32;
            let lane = (op.imm & 0xFF) as u32;
            let s = op.r1 as usize & 15;
            let mask = if ebits == 64 { u64::MAX as u128 } else { (1u128 << ebits) - 1 };
            let val = ((cpu.xmm[s] >> (lane * ebits)) & mask) as u64;
            if op.x & 0x20 != 0 {
                cpu.write_mem(cpu.faddr(op), ebits, val);
            } else {
                let gw = if ebits == 64 { 64 } else { 32 };
                cpu.rset(op.r0, gw, val);
            }
        }
        _ => return exec_ext_more(cpu, op),
    }
    false
}

/// Whether a fast op of this kind may be re-executed on a shadow CPU for verification
/// (no non-idempotent memory writes, no control transfer that needs host dispatch).
#[inline]
pub(in crate::emu) fn verifiable(kind: u8) -> bool {
    !matches!(
        kind,
        K_GENERIC
            | K_ALU_MR
            | K_ALU_MI
            | K_JMP
            | K_CALL
            | K_CALL_M
            | K_CALL_R
            | K_JMP_M
            | K_JMP_R
            | K_INC_M
            | K_DEC_M
            | K_PUSH_M
            | K_POP_M
            | K_LEAVE
            | K_SHIFT_MI
            | K_SHIFT_MCL
            | K_ROT_MI
            | K_ROT_MCL
            | K_INSN
            | K_LOOP
            | K_VCVT_PH..=K_PCMPSTR
    )
}
