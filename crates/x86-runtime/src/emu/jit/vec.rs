//! NEON encodings and native x86->arm64 vector translation for
//! `K_VSHUF_RR`/`K_VSHUF_RM`, `K_VINT_RR`/`K_VINT_RM`, `K_VFP_RR`/`K_VFP_RM`,
//! and extended integer/stack/flag fast ops (`K_BSWAP_R..=K_PEXTR`).

use super::emit::{enc_logic_imm, sf, Asm};
use super::translate::{stack_tso, Chk, Cond, FlagState, Src, Tr};
use super::*;
use crate::emu::fast::{self, FastOp};
use crate::emu::mem;

impl Asm {
    pub(super) fn adcs(&mut self, x: bool, s: bool, d: u32, n: u32, m: u32) {
        self.emit(0x1A00_0000 | sf(x) | (s as u32) << 29 | m << 16 | n << 5 | d);
    }
    pub(super) fn sbcs(&mut self, x: bool, s: bool, d: u32, n: u32, m: u32) {
        self.emit(0x5A00_0000 | sf(x) | (s as u32) << 29 | m << 16 | n << 5 | d);
    }
    pub(super) fn bic_rr(&mut self, x: bool, d: u32, n: u32, m: u32) {
        self.emit(0x0A20_0000 | sf(x) | m << 16 | n << 5 | d);
    }
    pub(super) fn rorv(&mut self, x: bool, d: u32, n: u32, m: u32) {
        self.emit(0x1AC0_2C00 | sf(x) | m << 16 | n << 5 | d);
    }
    pub(super) fn v_cnt_8b(&mut self, vd: u32, vn: u32) {
        self.emit(0x0E20_5800 | vn << 5 | vd);
    }
    pub(super) fn v_cnt_16b(&mut self, vd: u32, vn: u32) {
        self.emit(0x4E20_5800 | vn << 5 | vd);
    }
    pub(super) fn v_saddlp(&mut self, size: u32, vd: u32, vn: u32) {
        self.emit(0x4E20_2800 | (size & 3) << 22 | vn << 5 | vd);
    }
    pub(super) fn v_addv_8b(&mut self, vd: u32, vn: u32) {
        self.emit(0x0E31_B800 | vn << 5 | vd);
    }
    pub(super) fn v_addv_4s(&mut self, vd: u32, vn: u32) {
        self.emit(0x4EB1_B800 | vn << 5 | vd);
    }
    pub(super) fn rev16_w(&mut self, d: u32, n: u32) {
        self.emit(0x5AC0_0400 | n << 5 | d);
    }
    pub(super) fn v_abs(&mut self, size: u32, vd: u32, vn: u32) {
        self.emit(0x4E20_B800 | (size & 3) << 22 | vn << 5 | vd);
    }
    pub(super) fn v_neg(&mut self, size: u32, vd: u32, vn: u32) {
        self.emit(0x6E20_B800 | (size & 3) << 22 | vn << 5 | vd);
    }
    pub(super) fn v_cmlt0(&mut self, size: u32, vd: u32, vn: u32) {
        self.emit(0x4E20_A800 | (size & 3) << 22 | vn << 5 | vd);
    }
    pub(super) fn v_cmeq0(&mut self, size: u32, vd: u32, vn: u32) {
        self.emit(0x4E20_9800 | (size & 3) << 22 | vn << 5 | vd);
    }
    pub(super) fn v_ins_elem(&mut self, lg: u32, vd: u32, lane: u32, rn: u32) {
        let imm5 = (lane << (lg + 1)) | (1u32 << lg);
        self.emit(0x4E00_1C00 | imm5 << 16 | rn << 5 | vd);
    }
    pub(super) fn v_umov_elem(&mut self, lg: u32, rd: u32, vn: u32, lane: u32) {
        let q = (lg == 3) as u32;
        let imm5 = (lane << (lg + 1)) | (1u32 << lg);
        self.emit(0x0E00_3C00 | q << 30 | imm5 << 16 | vn << 5 | rd);
    }
    pub(super) fn neon_3same(&mut self, u: bool, size: u32, opcode: u32, vd: u32, vn: u32, vm: u32) {
        self.emit(0x4E20_0400 | (u as u32) << 29 | (size & 3) << 22 | vm << 16 | opcode << 11 | vn << 5 | vd);
    }
    pub(super) fn v_and(&mut self, vd: u32, vn: u32, vm: u32) {
        self.neon_3same(false, 0, 3, vd, vn, vm);
    }
    pub(super) fn v_bic(&mut self, vd: u32, vn: u32, vm: u32) {
        self.neon_3same(false, 1, 3, vd, vn, vm);
    }
    pub(super) fn v_orr(&mut self, vd: u32, vn: u32, vm: u32) {
        self.neon_3same(false, 2, 3, vd, vn, vm);
    }
    pub(super) fn v_eor(&mut self, vd: u32, vn: u32, vm: u32) {
        self.neon_3same(true, 0, 3, vd, vn, vm);
    }
    pub(super) fn v_bsl(&mut self, vd: u32, vn: u32, vm: u32) {
        self.neon_3same(true, 1, 3, vd, vn, vm);
    }
    pub(super) fn v_mov(&mut self, vd: u32, vn: u32) {
        if vd != vn {
            self.v_orr(vd, vn, vn);
        }
    }
    pub(super) fn v_zero(&mut self, vd: u32) {
        self.v_eor(vd, vd, vd);
    }
    pub(super) fn zero128(&mut self, base: u32, off: u32) {
        if off % 8 == 0 && off <= 504 {
            self.stp_x(ZR, ZR, base, off as i32);
        } else {
            self.str(64, ZR, base, off);
            self.str(64, ZR, base, off + 8);
        }
    }
    pub(super) fn v_ins_d(&mut self, vd: u32, idx: u32, rn: u32) {
        let imm5 = ((idx & 1) << 4) | 0b1000;
        self.emit(0x4E00_1C00 | imm5 << 16 | rn << 5 | vd);
    }
    pub(super) fn v_ins_s(&mut self, vd: u32, idx: u32, rn: u32) {
        let imm5 = ((idx & 3) << 3) | 0b0100;
        self.emit(0x4E00_1C00 | imm5 << 16 | rn << 5 | vd);
    }
    pub(super) fn v_umov_d(&mut self, rd: u32, vn: u32, idx: u32) {
        let imm5 = ((idx & 1) << 4) | 0b1000;
        self.emit(0x4E00_3C00 | imm5 << 16 | vn << 5 | rd);
    }
    pub(super) fn v_dup(&mut self, size: u32, vd: u32, rn: u32) {
        let imm5 = 1u32 << (size & 3);
        self.emit(0x4E00_0C00 | imm5 << 16 | rn << 5 | vd);
    }
    /// `dup vd.4s, vn.s[lane]` (broadcast 32-bit lane across 128-bit vector).
    pub(super) fn v_dup_lane_s(&mut self, vd: u32, vn: u32, lane: u32) {
        let imm5 = ((lane & 3) << 3) | 0b0100;
        self.emit(0x4E00_0400 | imm5 << 16 | vn << 5 | vd);
    }
    pub(super) fn v_zip1(&mut self, size: u32, vd: u32, vn: u32, vm: u32) {
        self.emit(0x4E00_3800 | (size & 3) << 22 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_zip2(&mut self, size: u32, vd: u32, vn: u32, vm: u32) {
        self.emit(0x4E00_7800 | (size & 3) << 22 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_uzp1(&mut self, size: u32, vd: u32, vn: u32, vm: u32) {
        self.emit(0x4E00_1800 | (size & 3) << 22 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_uzp2(&mut self, size: u32, vd: u32, vn: u32, vm: u32) {
        self.emit(0x4E00_5800 | (size & 3) << 22 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_tbl1(&mut self, vd: u32, vn: u32, vm: u32) {
        self.emit(0x4E00_0000 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_tbl2(&mut self, vd: u32, vn_pair: u32, vm: u32) {
        self.emit(0x4E00_2000 | vm << 16 | vn_pair << 5 | vd);
    }
    pub(super) fn v_ext(&mut self, vd: u32, vn: u32, vm: u32, imm4: u32) {
        self.emit(0x6E00_0000 | vm << 16 | (imm4 & 15) << 11 | vn << 5 | vd);
    }
    pub(super) fn v_sqxtn(&mut self, q: bool, size: u32, vd: u32, vn: u32) {
        self.emit(0x0E21_4800 | (q as u32) << 30 | (size & 3) << 22 | vn << 5 | vd);
    }
    pub(super) fn v_sqxtun(&mut self, q: bool, size: u32, vd: u32, vn: u32) {
        self.emit(0x2E21_2800 | (q as u32) << 30 | (size & 3) << 22 | vn << 5 | vd);
    }
    pub(super) fn v_shl_i(&mut self, ebits: u32, vd: u32, vn: u32, shift: u32) {
        let imm = ebits + shift;
        self.emit(0x4F00_5400 | imm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_shr_i(&mut self, u: bool, ebits: u32, vd: u32, vn: u32, shift: u32) {
        let imm = 2 * ebits - shift;
        self.emit(0x4F00_0400 | (u as u32) << 29 | imm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_smull(&mut self, q: bool, size: u32, vd: u32, vn: u32, vm: u32) {
        self.emit(0x0E20_C000 | (q as u32) << 30 | (size & 3) << 22 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_umull(&mut self, q: bool, size: u32, vd: u32, vn: u32, vm: u32) {
        self.emit(0x2E20_C000 | (q as u32) << 30 | (size & 3) << 22 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_rshrn15_h(&mut self, q: bool, vd: u32, vn: u32) {
        self.emit(0x0F11_8C00 | (q as u32) << 30 | vn << 5 | vd);
    }
    pub(super) fn v_sxtl_b(&mut self, q: bool, vd: u32, vn: u32) {
        self.emit(0x0F08_A400 | (q as u32) << 30 | vn << 5 | vd);
    }
    pub(super) fn v_sxtl_s(&mut self, q: bool, vd: u32, vn: u32) {
        self.emit(0x0F20_A400 | (q as u32) << 30 | vn << 5 | vd);
    }
    pub(super) fn v_uxtl_b(&mut self, q: bool, vd: u32, vn: u32) {
        self.emit(0x2F08_A400 | (q as u32) << 30 | vn << 5 | vd);
    }
    // --- scalar / packed FP ---
    pub(super) fn s_farith(&mut self, is_f64: bool, op: u8, vd: u32, vn: u32, vm: u32) {
        let opc = match op {
            0 => 0x1E20_2800, // fadd
            1 => 0x1E20_3800, // fsub
            2 => 0x1E20_0800, // fmul
            _ => 0x1E20_1800, // fdiv
        };
        self.emit(opc | (is_f64 as u32) << 22 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn s_fsqrt(&mut self, is_f64: bool, vd: u32, vn: u32) {
        self.emit(0x1E21_C000 | (is_f64 as u32) << 22 | vn << 5 | vd);
    }
    pub(super) fn s_fcsel(&mut self, is_f64: bool, vd: u32, vn: u32, vm: u32, cond: u32) {
        self.emit(0x1E20_0C00 | (is_f64 as u32) << 22 | vm << 16 | cond << 12 | vn << 5 | vd);
    }
    pub(super) fn v_farith(&mut self, is_f64: bool, op: u8, vd: u32, vn: u32, vm: u32) {
        let opc = match op {
            0 => 0x4E20_D400, // fadd
            1 => 0x4EA0_D400, // fsub
            2 => 0x6E20_DC00, // fmul
            _ => 0x6E20_FC00, // fdiv
        };
        self.emit(opc | (is_f64 as u32) << 22 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_fcmeq(&mut self, is_f64: bool, vd: u32, vn: u32, vm: u32) {
        self.emit(0x4E20_E400 | (is_f64 as u32) << 22 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_fcmge(&mut self, is_f64: bool, vd: u32, vn: u32, vm: u32) {
        self.emit(0x6E20_E400 | (is_f64 as u32) << 22 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_fcmgt(&mut self, is_f64: bool, vd: u32, vn: u32, vm: u32) {
        self.emit(0x6EA0_E400 | (is_f64 as u32) << 22 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_faddp(&mut self, is_f64: bool, vd: u32, vn: u32, vm: u32) {
        self.emit(0x6E20_D400 | (is_f64 as u32) << 22 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_fsqrt(&mut self, is_f64: bool, vd: u32, vn: u32) {
        self.emit(0x6EA1_F800 | (is_f64 as u32) << 22 | vn << 5 | vd);
    }
    pub(super) fn v_not(&mut self, vd: u32, vn: u32) {
        self.emit(0x6E20_5800 | vn << 5 | vd);
    }
    pub(super) fn s_frint(&mut self, is_f64: bool, x86_rmode: u8, vd: u32, vn: u32) {
        let arm_rmode: u32 = if (x86_rmode & 4) != 0 {
            7 // current FPCR.RMode (frinti)
        } else {
            match x86_rmode & 3 {
                0 => 0, // ties-even (frintn)
                1 => 2, // floor (frintm)
                2 => 1, // ceil (frintp)
                _ => 3, // trunc (frintz)
            }
        };
        self.emit(0x1E24_4000 | (is_f64 as u32) << 22 | arm_rmode << 15 | vn << 5 | vd);
    }
    pub(super) fn v_frint(&mut self, is_f64: bool, x86_rmode: u8, vd: u32, vn: u32) {
        let opc: u32 = if (x86_rmode & 4) != 0 {
            0x6EA1_9800 // current FPCR.RMode (frinti)
        } else {
            match x86_rmode & 3 {
                0 => 0x4E21_8800, // frintn
                1 => 0x4E21_9800, // frintm (floor)
                2 => 0x4EA1_8800, // frintp (ceil)
                _ => 0x4EA1_9800, // frintz (trunc)
            }
        };
        self.emit(opc | (is_f64 as u32) << 22 | vn << 5 | vd);
    }
    pub(super) fn v_uabdl_b(&mut self, q: bool, vd: u32, vn: u32, vm: u32) {
        self.emit(0x2E20_7000 | (q as u32) << 30 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_uaddlv_h(&mut self, vd: u32, vn: u32) {
        self.emit(0x6E70_3800 | vn << 5 | vd);
    }
    pub(super) fn v_pmull_1q(&mut self, vd: u32, vn: u32, vm: u32) {
        self.emit(0x0EE0_E000 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_uminv_s(&mut self, vd: u32, vn: u32) {
        self.emit(0x6EB1_A800 | vn << 5 | vd);
    }
    pub(super) fn v_scvtf_4s(&mut self, vd: u32, vn: u32) {
        self.emit(0x4E21_D800 | vn << 5 | vd);
    }
    pub(super) fn v_frintn_4s(&mut self, vd: u32, vn: u32) {
        self.emit(0x4E21_8800 | vn << 5 | vd);
    }
    pub(super) fn v_fcvtzs_4s(&mut self, vd: u32, vn: u32) {
        self.emit(0x4EA1_B800 | vn << 5 | vd);
    }
}

/// Build the 16-byte `TBL` permutation control mask (`lo64`, `hi64`) for `pshufd`,
/// `pshuflw`, `pshufhw`, or `shufps` (over the 32-byte table `(va, vb)`).
fn shuf_tbl_mask(sub: u8, imm: u8) -> (u64, u64) {
    use fast::*;
    let m = imm as usize;
    let mut idx = [0u8; 16];
    match sub {
        VS_PSHUFD => {
            for l in 0..4 {
                let s = ((m >> (2 * l)) & 3) * 4;
                for b in 0..4 {
                    idx[4 * l + b] = (s + b) as u8;
                }
            }
        }
        VS_PSHUFLW => {
            for l in 0..4 {
                let s = ((m >> (2 * l)) & 3) * 2;
                idx[2 * l] = s as u8;
                idx[2 * l + 1] = (s + 1) as u8;
            }
            for b in 8..16 {
                idx[b] = b as u8;
            }
        }
        VS_PSHUFHW => {
            for b in 0..8 {
                idx[b] = b as u8;
            }
            for l in 0..4 {
                let s = 8 + ((m >> (2 * l)) & 3) * 2;
                idx[8 + 2 * l] = s as u8;
                idx[8 + 2 * l + 1] = (s + 1) as u8;
            }
        }
        VS_SHUFPD => {
            let s0 = (m & 1) * 8;
            let s1 = 16 + ((m >> 1) & 1) * 8;
            for b in 0..8 {
                idx[b] = (s0 + b) as u8;
                idx[8 + b] = (s1 + b) as u8;
            }
        }
        VS_VPERMILPD_I => {
            let s0 = (m & 1) * 8;
            let s1 = ((m >> 1) & 1) * 8;
            for b in 0..8 {
                idx[b] = (s0 + b) as u8;
                idx[8 + b] = (s1 + b) as u8;
            }
        }
        VS_PBLENDW => {
            for w in 0..8 {
                let base = if (m >> w) & 1 != 0 { 16 } else { 0 };
                idx[2 * w] = (base + 2 * w) as u8;
                idx[2 * w + 1] = (base + 2 * w + 1) as u8;
            }
        }
        VS_BLENDPS => {
            for l in 0..4 {
                let base = if (m >> l) & 1 != 0 { 16 } else { 0 };
                for b in 0..4 {
                    idx[4 * l + b] = (base + 4 * l + b) as u8;
                }
            }
        }
        VS_BLENDPD => {
            for q in 0..2 {
                let base = if (m >> q) & 1 != 0 { 16 } else { 0 };
                for b in 0..8 {
                    idx[8 * q + b] = (base + 8 * q + b) as u8;
                }
            }
        }
        VS_INSERTPS | VS_INSERTPS_M => {
            let count_s = if sub == VS_INSERTPS_M { 0 } else { (m >> 6) & 3 };
            let count_d = (m >> 4) & 3;
            let zmask = m & 0xF;
            for l in 0..4 {
                if (zmask >> l) & 1 != 0 {
                    for b in 0..4 {
                        idx[4 * l + b] = 0x80;
                    }
                } else if l == count_d {
                    for b in 0..4 {
                        idx[4 * l + b] = (16 + count_s * 4 + b) as u8;
                    }
                } else {
                    for b in 0..4 {
                        idx[4 * l + b] = (4 * l + b) as u8;
                    }
                }
            }
        }
        _ => {
            // VS_SHUFPS: lanes 0..2 from `va` (0..16), lanes 2..4 from `vb` (16..32).
            for l in 0..4 {
                let base = if l < 2 { 0 } else { 16 };
                let s = base + ((m >> (2 * l)) & 3) * 4;
                for b in 0..4 {
                    idx[4 * l + b] = (s + b) as u8;
                }
            }
        }
    }
    let lo = u64::from_le_bytes(idx[0..8].try_into().unwrap());
    let hi = u64::from_le_bytes(idx[8..16].try_into().unwrap());
    (lo, hi)
}

impl Tr<'_> {
    /// Load `b` for a V-form op into `v1` (and `v7` for the upper 128-bit lane if `VM_V256`).
    pub(super) fn vload_b(&mut self, k: usize, op: &FastOp) {
        use fast::*;
        let mem_form = matches!(
            op.kind,
            K_VSHUF_RM | K_VINT_RM | K_VFP_RM | K_VAES_RM | K_VGFNI_RM | K_VVNNI_RM
        );
        let is_256 = (op.w & 3) == VM_V256;
        if !mem_form {
            // (`ldr q1, [cpu, #xmm]` unless the XMM cache holds the register.)
            self.xsrc(1, op.r1);
            if is_256 {
                let r2 = (op.r1 as u32 & 15) * 16;
                self.a.ldr(128, 7, XCPU, OFF_YMM_HI + r2);
            }
            return;
        }
        match op.w >> 4 {
            2 => {
                if !self.scalar_fp_mem_op(k, true, 1, op, 32) {
                    self.load_op(k, T0, op, 32);
                    self.a.fmov_gf(false, 1, T0);
                }
            }
            3 => {
                if !self.scalar_fp_mem_op(k, true, 1, op, 64) {
                    self.load_op(k, T0, op, 64);
                    self.a.fmov_gf(true, 1, T0);
                }
            }
            4 if self.xon => {
                self.vec128_mem_op(k, op, true, 1);
            }
            4 => {
                let a = self.addr_into(op, XA);
                let chk = Self::chk_of(op);
                if chk == Chk::Ro {
                    self.a.ldr(128, 1, a, 0);
                    return;
                }
                if chk == Chk::Full {
                    let se = self.side_exit_here(k);
                    self.range_check_op(k, a, se);
                }
                self.a.ldr(64, T0, a, 0);
                self.a.ldr(64, T1, a, 8);
                if mem::tso() && (chk != Chk::Stack || stack_tso()) {
                    self.a.dmb_ishld();
                }
                self.a.fmov_gf(true, 1, T0);
                self.a.v_ins_d(1, 1, T1);
            }
            _ => {
                let a = self.addr_into(op, XA);
                let chk = Self::chk_of(op);
                if chk == Chk::Ro {
                    self.a.ldr(128, 1, a, 0);
                    self.a.ldr(128, 7, a, 16);
                    return;
                }
                if chk == Chk::Full {
                    let se = self.side_exit_here(k);
                    self.range_check_op(k, a, se);
                }
                self.a.ldr(64, T0, a, 0);
                self.a.ldr(64, T1, a, 8);
                self.a.ldr(64, T2, a, 16);
                self.a.ldr(64, T3, a, 24);
                if mem::tso() && (chk != Chk::Stack || stack_tso()) {
                    self.a.dmb_ishld();
                }
                self.a.fmov_gf(true, 1, T0);
                self.a.v_ins_d(1, 1, T1);
                self.a.fmov_gf(true, 7, T2);
                self.a.v_ins_d(7, 1, T3);
            }
        }
    }

    /// Store 128-bit result `v0` (and `v6` as low lane when `is_256`) according to `op.w & 3`.
    pub(super) fn vstore_dst(&mut self, op: &FastOp) {
        use fast::*;
        match op.w & 3 {
            VM_LEG => self.xwrite(op.r0, 0),
            VM_V128 => {
                self.xwrite(op.r0, 0);
                self.clear_ymm_hi(op.r0);
            }
            _ => {
                self.xwrite(op.r0, 6);
                self.write_ymm_hi(op.r0, 0);
            }
        }
    }

    /// Emit one 128-bit lane of `K_VSHUF_*` on `(v0 = a, v1 = b) -> v0`.
    pub(super) fn emit_vshuf_lane(&mut self, sub: u8, imm8: u8) {
        use fast::*;
        let ic = imm8 as u32;
        match sub {
            VS_MOV => self.a.v_mov(0, 1),
            VS_ZX32 => {
                self.a.fmov_fg(false, T0, 1);
                self.a.fmov_gf(false, 0, T0);
            }
            VS_ZX64 => {
                self.a.fmov_fg(true, T0, 1);
                self.a.fmov_gf(true, 0, T0);
            }
            VS_MRG32 => {
                self.a.fmov_fg(false, T0, 1);
                self.a.v_ins_s(0, 0, T0);
            }
            VS_MRG64 => {
                self.a.fmov_fg(true, T0, 1);
                self.a.v_ins_d(0, 0, T0);
            }
            VS_XOR => self.a.v_eor(0, 0, 1),
            VS_OR => self.a.v_orr(0, 0, 1),
            VS_AND => self.a.v_and(0, 0, 1),
            VS_ANDN => self.a.v_bic(0, 1, 0), // !a & b = b & !a
            VS_UNPCKLBW => self.a.v_zip1(0, 0, 0, 1),
            VS_UNPCKLWD => self.a.v_zip1(1, 0, 0, 1),
            VS_UNPCKLDQ => self.a.v_zip1(2, 0, 0, 1),
            VS_UNPCKLQDQ => self.a.v_zip1(3, 0, 0, 1),
            VS_UNPCKHBW => self.a.v_zip2(0, 0, 0, 1),
            VS_UNPCKHWD => self.a.v_zip2(1, 0, 0, 1),
            VS_UNPCKHDQ => self.a.v_zip2(2, 0, 0, 1),
            VS_UNPCKHQDQ => self.a.v_zip2(3, 0, 0, 1),
            VS_PACKSSWB => {
                self.a.v_sqxtn(false, 0, 2, 0);
                self.a.v_sqxtn(true, 0, 2, 1);
                self.a.v_mov(0, 2);
            }
            VS_PACKSSDW => {
                self.a.v_sqxtn(false, 1, 2, 0);
                self.a.v_sqxtn(true, 1, 2, 1);
                self.a.v_mov(0, 2);
            }
            VS_PACKUSWB => {
                self.a.v_sqxtun(false, 0, 2, 0);
                self.a.v_sqxtun(true, 0, 2, 1);
                self.a.v_mov(0, 2);
            }
            VS_PACKUSDW => {
                self.a.v_sqxtun(false, 1, 2, 0);
                self.a.v_sqxtun(true, 1, 2, 1);
                self.a.v_mov(0, 2);
            }
            VS_PSHUFD | VS_PSHUFLW | VS_PSHUFHW | VS_VPERMILPD_I => {
                if sub == VS_PSHUFD {
                    match imm8 {
                        0x00 => { self.a.v_dup_lane_s(0, 1, 0); return; }
                        0x55 => { self.a.v_dup_lane_s(0, 1, 1); return; }
                        0xAA => { self.a.v_dup_lane_s(0, 1, 2); return; }
                        0xFF => { self.a.v_dup_lane_s(0, 1, 3); return; }
                        0xE4 => { self.a.v_mov(0, 1); return; }
                        0x4E => { self.a.v_ext(0, 1, 1, 8); return; }
                        0xB1 => { self.a.v_rev64_4s(0, 1); return; }
                        _ => {}
                    }
                }
                let (lo, hi) = shuf_tbl_mask(sub, imm8);
                self.mov_lit128(2, lo, hi);
                self.a.v_tbl1(0, 1, 2);
            }
            VS_SHUFPS | VS_SHUFPD | VS_PBLENDW | VS_BLENDPS | VS_BLENDPD | VS_INSERTPS | VS_INSERTPS_M => {
                if sub == VS_SHUFPS {
                    match imm8 {
                        0x44 => { self.a.v_zip1(3, 0, 0, 1); return; }
                        0xEE => { self.a.v_zip2(3, 0, 0, 1); return; }
                        0x88 => { self.a.v_uzp1(2, 0, 0, 1); return; }
                        0xDD => { self.a.v_uzp2(2, 0, 0, 1); return; }
                        0x4E => { self.a.v_ext(0, 0, 1, 8); return; }
                        _ => {}
                    }
                }
                // v0 and v1 are consecutive (`{v0.16b, v1.16b}`)!
                let (lo, hi) = shuf_tbl_mask(sub, imm8);
                self.mov_lit128(2, lo, hi);
                self.a.v_tbl2(0, 0, 2);
            }
            VS_PALIGNR => {
                if ic == 0 {
                    self.a.v_mov(0, 1);
                } else if ic < 16 {
                    self.a.v_ext(0, 1, 0, ic);
                } else if ic == 16 {
                    // v0 already holds `a`
                } else if ic < 32 {
                    self.a.v_zero(2);
                    self.a.v_ext(0, 0, 2, ic - 16);
                } else {
                    self.a.v_zero(0);
                }
            }
            VS_DDUP => {
                self.a.v_umov_d(T0, 1, 0);
                self.a.v_dup(3, 0, T0);
            }
            VS_MOVHLPS => {
                self.a.v_umov_d(T0, 1, 1);
                self.a.v_ins_d(0, 0, T0);
            }
            VS_VPERMILPS => {
                self.a.movz(false, T0, 3, 0);
                self.a.v_dup(2, 2, T0);
                self.a.v_and(2, 1, 2);
                self.a.mov_imm(false, T0, 0x0404_0404);
                self.a.v_dup(2, 3, T0);
                self.a.neon_3same(false, 2, 19, 2, 2, 3); // mul v2.4s, v2.4s, v3.4s
                self.a.mov_imm(false, T0, 0x0302_0100);
                self.a.v_dup(2, 3, T0);
                self.a.neon_3same(false, 2, 16, 2, 2, 3); // add v2.4s, v2.4s, v3.4s
                self.a.v_tbl1(0, 0, 2);
            }
            VS_VPERMILPD => {
                // Bit 1 of each 64-bit lane in v1 selects qword 0 or 1 of v0.
                self.a.v_zip1(3, 2, 0, 0); // v2 = [a0, a0]
                self.a.v_zip2(3, 3, 0, 0); // v3 = [a1, a1]
                self.a.movz(true, T0, 2, 0);
                self.a.v_dup(3, 4, T0);
                self.a.v_and(4, 1, 4);
                self.a.v_cmeq0(3, 4, 4); // all-1s where (ctrl & 2) == 0 -> pick a0
                self.a.v_bsl(4, 2, 3);
                self.a.v_mov(0, 4);
            }
            VS_PSHUFB => {
                // Keep bit 7 (so negative indices stay >= 16 -> TBL zeroes them) and low 4 bits.
                self.a.mov_imm(true, T0, 0x8F8F_8F8F_8F8F_8F8Fu64);
                self.a.v_dup(3, 2, T0);
                self.a.v_and(2, 1, 2);
                self.a.v_tbl1(0, 0, 2);
            }
            VS_PSRLW_I | VS_PSRLD_I | VS_PSRLQ_I => {
                let ebits = match sub {
                    VS_PSRLW_I => 16,
                    VS_PSRLD_I => 32,
                    _ => 64,
                };
                if ic == 0 {
                    self.a.v_mov(0, 1);
                } else if ic >= ebits {
                    self.a.v_zero(0);
                } else {
                    self.a.v_shr_i(true, ebits, 0, 1, ic);
                }
            }
            VS_PSRAW_I | VS_PSRAD_I => {
                let ebits = if sub == VS_PSRAW_I { 16 } else { 32 };
                if ic == 0 {
                    self.a.v_mov(0, 1);
                } else {
                    self.a.v_shr_i(false, ebits, 0, 1, ic.min(ebits - 1));
                }
            }
            VS_PSLLW_I | VS_PSLLD_I | VS_PSLLQ_I => {
                let ebits = match sub {
                    VS_PSLLW_I => 16,
                    VS_PSLLD_I => 32,
                    _ => 64,
                };
                if ic == 0 {
                    self.a.v_mov(0, 1);
                } else if ic >= ebits {
                    self.a.v_zero(0);
                } else {
                    self.a.v_shl_i(ebits, 0, 1, ic);
                }
            }
            VS_PSRLDQ_I => {
                if ic == 0 {
                    self.a.v_mov(0, 1);
                } else if ic >= 16 {
                    self.a.v_zero(0);
                } else {
                    self.a.v_zero(2);
                    self.a.v_ext(0, 1, 2, ic);
                }
            }
            VS_PSLLDQ_I => {
                if ic == 0 {
                    self.a.v_mov(0, 1);
                } else if ic >= 16 {
                    self.a.v_zero(0);
                } else {
                    self.a.v_zero(2);
                    self.a.v_ext(0, 2, 1, 16 - ic);
                }
            }
            VS_PSRLW..=VS_PSLLQ => {
                // Value is `a` (v0), count is low 64 bits of `b` (v1).
                let (size, ebits, left, arith) = match sub {
                    VS_PSRLW => (1, 16u64, false, false),
                    VS_PSRAW => (1, 16, false, true),
                    VS_PSLLW => (1, 16, true, false),
                    VS_PSRLD => (2, 32, false, false),
                    VS_PSRAD => (2, 32, false, true),
                    VS_PSLLD => (2, 32, true, false),
                    VS_PSRLQ => (3, 64, false, false),
                    _ => (3, 64, true, false),
                };
                self.a.flush_flags_and_shift_vec(size, ebits, left, arith);
            }
            _ => {}
        }
    }

    /// Emit one 128-bit lane of `K_VINT_*` on `(v0 = a, v1 = b) -> v0`.
    pub(super) fn emit_vint_lane(&mut self, sub: u8, imm8: u8) {
        use fast::*;
        match sub {
            VI_PADDB => self.a.neon_3same(false, 0, 16, 0, 0, 1),
            VI_PADDW => self.a.neon_3same(false, 1, 16, 0, 0, 1),
            VI_PADDD => self.a.neon_3same(false, 2, 16, 0, 0, 1),
            VI_PADDQ => self.a.neon_3same(false, 3, 16, 0, 0, 1),
            VI_PSUBB => self.a.neon_3same(true, 0, 16, 0, 0, 1),
            VI_PSUBW => self.a.neon_3same(true, 1, 16, 0, 0, 1),
            VI_PSUBD => self.a.neon_3same(true, 2, 16, 0, 0, 1),
            VI_PSUBQ => self.a.neon_3same(true, 3, 16, 0, 0, 1),
            VI_PADDSB => self.a.neon_3same(false, 0, 1, 0, 0, 1),
            VI_PADDSW => self.a.neon_3same(false, 1, 1, 0, 0, 1),
            VI_PADDUSB => self.a.neon_3same(true, 0, 1, 0, 0, 1),
            VI_PADDUSW => self.a.neon_3same(true, 1, 1, 0, 0, 1),
            VI_PSUBSB => self.a.neon_3same(false, 0, 5, 0, 0, 1),
            VI_PSUBSW => self.a.neon_3same(false, 1, 5, 0, 0, 1),
            VI_PSUBUSB => self.a.neon_3same(true, 0, 5, 0, 0, 1),
            VI_PSUBUSW => self.a.neon_3same(true, 1, 5, 0, 0, 1),
            VI_PMULLW => self.a.neon_3same(false, 1, 19, 0, 0, 1),
            VI_PMULLD => self.a.neon_3same(false, 2, 19, 0, 0, 1),
            VI_PMULHW => {
                self.a.v_smull(false, 1, 2, 0, 1);
                self.a.v_smull(true, 1, 3, 0, 1);
                self.a.v_uzp2(1, 0, 2, 3);
            }
            VI_PMULHUW => {
                self.a.v_umull(false, 1, 2, 0, 1);
                self.a.v_umull(true, 1, 3, 0, 1);
                self.a.v_uzp2(1, 0, 2, 3);
            }
            VI_PMULHRSW => {
                self.a.v_smull(false, 1, 2, 0, 1);
                self.a.v_smull(true, 1, 3, 0, 1);
                self.a.v_rshrn15_h(false, 0, 2);
                self.a.v_rshrn15_h(true, 0, 3);
            }
            VI_PMADDWD => {
                self.a.v_smull(false, 1, 2, 0, 1);
                self.a.v_smull(true, 1, 3, 0, 1);
                self.a.neon_3same(false, 2, 23, 0, 2, 3); // addp v0.4s, v2.4s, v3.4s
            }
            VI_PMADDUBSW => {
                self.a.v_uxtl_b(false, 2, 0);
                self.a.v_uxtl_b(true, 3, 0);
                self.a.v_sxtl_b(false, 4, 1);
                self.a.v_sxtl_b(true, 5, 1);
                self.a.neon_3same(false, 1, 19, 2, 2, 4); // mul v2.8h, v2.8h, v4.8h
                self.a.neon_3same(false, 1, 19, 3, 3, 5); // mul v3.8h, v3.8h, v5.8h
                self.a.v_uzp1(1, 4, 2, 3);
                self.a.v_uzp2(1, 5, 2, 3);
                self.a.neon_3same(false, 1, 1, 0, 4, 5); // sqadd v0.8h, v4.8h, v5.8h
            }
            VI_PMULUDQ => {
                self.a.v_uzp1(2, 2, 0, 0);
                self.a.v_uzp1(2, 3, 1, 1);
                self.a.v_umull(false, 2, 0, 2, 3);
            }
            VI_PMULDQ => {
                self.a.v_uzp1(2, 2, 0, 0);
                self.a.v_uzp1(2, 3, 1, 1);
                self.a.v_smull(false, 2, 0, 2, 3);
            }
            VI_PMINUB => self.a.neon_3same(true, 0, 13, 0, 0, 1),
            VI_PMAXUB => self.a.neon_3same(true, 0, 12, 0, 0, 1),
            VI_PMINSW => self.a.neon_3same(false, 1, 13, 0, 0, 1),
            VI_PMAXSW => self.a.neon_3same(false, 1, 12, 0, 0, 1),
            VI_PMINSB => self.a.neon_3same(false, 0, 13, 0, 0, 1),
            VI_PMAXSB => self.a.neon_3same(false, 0, 12, 0, 0, 1),
            VI_PMINUW => self.a.neon_3same(true, 1, 13, 0, 0, 1),
            VI_PMAXUW => self.a.neon_3same(true, 1, 12, 0, 0, 1),
            VI_PMINUD => self.a.neon_3same(true, 2, 13, 0, 0, 1),
            VI_PMAXUD => self.a.neon_3same(true, 2, 12, 0, 0, 1),
            VI_PMINSD => self.a.neon_3same(false, 2, 13, 0, 0, 1),
            VI_PMAXSD => self.a.neon_3same(false, 2, 12, 0, 0, 1),
            VI_PAVGB => self.a.neon_3same(true, 0, 2, 0, 0, 1),
            VI_PAVGW => self.a.neon_3same(true, 1, 2, 0, 0, 1),
            VI_PCMPEQB => self.a.neon_3same(true, 0, 17, 0, 0, 1),
            VI_PCMPEQW => self.a.neon_3same(true, 1, 17, 0, 0, 1),
            VI_PCMPEQD => self.a.neon_3same(true, 2, 17, 0, 0, 1),
            VI_PCMPEQQ => self.a.neon_3same(true, 3, 17, 0, 0, 1),
            VI_PCMPGTB => self.a.neon_3same(false, 0, 6, 0, 0, 1),
            VI_PCMPGTW => self.a.neon_3same(false, 1, 6, 0, 0, 1),
            VI_PCMPGTD => self.a.neon_3same(false, 2, 6, 0, 0, 1),
            VI_PCMPGTQ => self.a.neon_3same(false, 3, 6, 0, 0, 1),
            VI_PABSB => self.a.v_abs(0, 0, 1),
            VI_PABSW => self.a.v_abs(1, 0, 1),
            VI_PABSD => self.a.v_abs(2, 0, 1),
            VI_PSIGNB | VI_PSIGNW | VI_PSIGND => {
                let sz = (sub - VI_PSIGNB) as u32;
                self.a.v_neg(sz, 2, 0);
                self.a.v_cmlt0(sz, 3, 1);
                self.a.v_bsl(3, 2, 0);
                self.a.v_cmeq0(sz, 4, 1);
                self.a.v_bic(0, 3, 4);
            }
            VI_PHADDW => self.a.neon_3same(false, 1, 23, 0, 0, 1),
            VI_PHADDD => self.a.neon_3same(false, 2, 23, 0, 0, 1),
            VI_PHSUBW => {
                self.a.v_uzp1(1, 2, 0, 1);
                self.a.v_uzp2(1, 3, 0, 1);
                self.a.neon_3same(true, 1, 16, 0, 2, 3);
            }
            VI_PHSUBD => {
                self.a.v_uzp1(2, 2, 0, 1);
                self.a.v_uzp2(2, 3, 0, 1);
                self.a.neon_3same(true, 2, 16, 0, 2, 3);
            }
            VI_PSADBW => {
                self.a.v_uabdl_b(false, 2, 0, 1);
                self.a.v_uabdl_b(true, 3, 0, 1);
                self.a.v_uaddlv_h(2, 2);
                self.a.v_uaddlv_h(3, 3);
                self.a.v_zip1(3, 0, 2, 3);
            }
            VI_VPSLLVD => {
                self.a.v_shr_i(true, 32, 3, 1, 5);
                self.a.v_cmeq0(2, 3, 3);
                self.a.neon_3same(true, 2, 8, 0, 0, 1);
                self.a.v_and(0, 0, 3);
            }
            VI_VPSRLVD => {
                self.a.v_shr_i(true, 32, 3, 1, 5);
                self.a.v_cmeq0(2, 3, 3);
                self.a.v_neg(2, 2, 1);
                self.a.neon_3same(true, 2, 8, 0, 0, 2);
                self.a.v_and(0, 0, 3);
            }
            VI_VPSRAVD => {
                self.a.movz(false, T0, 31, 0);
                self.a.v_dup(2, 3, T0);
                self.a.neon_3same(true, 2, 13, 2, 1, 3); // umin v2.4s, v1.4s, v3.4s
                self.a.v_neg(2, 2, 2);
                self.a.neon_3same(false, 2, 8, 0, 0, 2); // sshl v0.4s, v0.4s, v2.4s
            }
            VI_VPSLLVQ => {
                self.a.v_shr_i(true, 64, 3, 1, 6);
                self.a.v_cmeq0(3, 3, 3);
                self.a.neon_3same(true, 3, 8, 0, 0, 1);
                self.a.v_and(0, 0, 3);
            }
            VI_VPSRLVQ => {
                self.a.v_shr_i(true, 64, 3, 1, 6);
                self.a.v_cmeq0(3, 3, 3);
                self.a.v_neg(3, 2, 1);
                self.a.neon_3same(true, 3, 8, 0, 0, 2);
                self.a.v_and(0, 0, 3);
            }
            _ => {
                // VI_PCLMULQDQ
                if imm8 & 0x01 != 0 {
                    self.a.v_ext(2, 0, 0, 8);
                } else {
                    self.a.v_mov(2, 0);
                }
                if imm8 & 0x10 != 0 {
                    self.a.v_ext(3, 1, 1, 8);
                } else {
                    self.a.v_mov(3, 1);
                }
                self.a.v_pmull_1q(0, 2, 3);
            }
        }
    }

    /// Emit vector FP comparison predicate (`pred & 0x0F`, see `sse_cmp_pred`) on `(v0, v1) -> v2`.
    pub(super) fn emit_vcmp_pred(&mut self, is_f64: bool, pred: u8) {
        let p = pred & 0x0F;
        match p & 7 {
            0 => {
                if p < 8 {
                    self.a.v_fcmeq(is_f64, 2, 0, 1); // EQ_OQ
                } else {
                    // EQ_UQ = !(GT_OS | LT_OS)
                    self.a.v_fcmgt(is_f64, 2, 0, 1);
                    self.a.v_fcmgt(is_f64, 3, 1, 0);
                    self.a.v_orr(2, 2, 3);
                    self.a.v_not(2, 2);
                }
            }
            1 => {
                if p < 8 {
                    self.a.v_fcmgt(is_f64, 2, 1, 0); // LT_OS
                } else {
                    // NGE_US = !GE_OS
                    self.a.v_fcmge(is_f64, 2, 0, 1);
                    self.a.v_not(2, 2);
                }
            }
            2 => {
                if p < 8 {
                    self.a.v_fcmge(is_f64, 2, 1, 0); // LE_OS
                } else {
                    // NGT_US = !GT_OS
                    self.a.v_fcmgt(is_f64, 2, 0, 1);
                    self.a.v_not(2, 2);
                }
            }
            3 => {
                if p < 8 {
                    // UNORD_Q = !(fcmeq(a,a) & fcmeq(b,b))
                    self.a.v_fcmeq(is_f64, 2, 0, 0);
                    self.a.v_fcmeq(is_f64, 3, 1, 1);
                    self.a.v_and(2, 2, 3);
                    self.a.v_not(2, 2);
                } else {
                    // FALSE_OQ
                    self.a.v_zero(2);
                }
            }
            4 => {
                if p < 8 {
                    // NEQ_UQ = !EQ_OQ
                    self.a.v_fcmeq(is_f64, 2, 0, 1);
                    self.a.v_not(2, 2);
                } else {
                    // NEQ_OQ = GT_OS | LT_OS
                    self.a.v_fcmgt(is_f64, 2, 0, 1);
                    self.a.v_fcmgt(is_f64, 3, 1, 0);
                    self.a.v_orr(2, 2, 3);
                }
            }
            5 => {
                if p < 8 {
                    // NLT_US = !LT_OS
                    self.a.v_fcmgt(is_f64, 2, 1, 0);
                    self.a.v_not(2, 2);
                } else {
                    // GE_OS
                    self.a.v_fcmge(is_f64, 2, 0, 1);
                }
            }
            6 => {
                if p < 8 {
                    // NLE_US = !LE_OS
                    self.a.v_fcmge(is_f64, 2, 1, 0);
                    self.a.v_not(2, 2);
                } else {
                    // GT_OS
                    self.a.v_fcmgt(is_f64, 2, 0, 1);
                }
            }
            _ => {
                if p < 8 {
                    // ORD_Q = fcmeq(a,a) & fcmeq(b,b)
                    self.a.v_fcmeq(is_f64, 2, 0, 0);
                    self.a.v_fcmeq(is_f64, 3, 1, 1);
                    self.a.v_and(2, 2, 3);
                } else {
                    // TRUE_UQ = ~0
                    self.a.v_zero(2);
                    self.a.v_not(2, 2);
                }
            }
        }
    }

    /// Inline NEON fixup for x86 NaN propagation rules (`x86_nan32`/`x86_nan64`):
    /// Given `v0 = a`, `v1 = b`, and `v2 = arm64 result r`, replaces any NaN lane of `v2`
    /// with `a | qbit` if `a` is NaN (and `!unary_b`), else `b | qbit` if `b` is NaN,
    /// else `-qNaN` (`0xFFC0_0000` / `0xFFF8_0000_0000_0000`).
    pub(super) fn emit_x86_nan_fixup(&mut self, is_d: bool, unary_b: bool) {
        let sz = if is_d { 3u32 } else { 2u32 };
        let qbit = if is_d { 0x0008_0000_0000_0000u64 } else { 0x0040_0000u64 };
        let indef = if is_d { 0xFFF8_0000_0000_0000u64 } else { 0xFFC0_0000u64 };
        self.a.mov_imm(is_d, T0, qbit);
        self.a.v_dup(sz, 3, T0); // v3 = qbit
        self.a.mov_imm(is_d, T0, indef);
        self.a.v_dup(sz, 4, T0); // v4 = indef
        self.a.v_fcmeq(is_d, 5, 1, 1); // v5 = 1s where b is NOT NaN
        self.a.v_orr(1, 1, 3); // v1 = b | qbit
        self.a.v_bsl(5, 4, 1); // v5 = if !b.is_nan() { indef } else { b | qbit }
        if !unary_b {
            self.a.v_fcmeq(is_d, 4, 0, 0); // v4 = 1s where a is NOT NaN
            self.a.v_orr(3, 0, 3); // v3 = a | qbit
            self.a.v_bsl(4, 5, 3); // v4 = if !a.is_nan() { v5 } else { a | qbit }
        } else {
            self.a.v_mov(4, 5);
        }
        self.a.v_fcmeq(is_d, 5, 2, 2); // v5 = 1s where r is NOT NaN
        self.a.v_bsl(5, 2, 4); // v5 = if !r.is_nan() { r } else { v4 }
        self.a.v_mov(2, 5);
    }

    /// Vector NaN check on `v2` (any NaN lane -> [`Self::emit_x86_nan_fixup`] inline;
    /// `v0 = a`, `v1 = b`). Uses only vector ops + `cbnz`, so NZCV is untouched.
    pub(super) fn emit_vnan_check_fix(&mut self, is_d: bool, unary_b: bool) {
        if self.fast_nan {
            return;
        }
        self.a.v_fcmeq(is_d, 3, 2, 2);
        self.a.v_uminv_s(3, 3);
        self.a.fmov_fg(false, T0, 3);
        let ok = self.a.cbnz(false, T0);
        self.emit_x86_nan_fixup(is_d, unary_b);
        self.a.bind(ok);
    }

    // Clear just the fraction bits of exponent-zero lanes; preserve the sign and NaN payloads.
    fn flush_denorm_lanes(&mut self, reg: u32, double: bool) {
        let (width, exponent, fraction) = if double {
            (3, 0x7ff0_0000_0000_0000, 0x000f_ffff_ffff_ffff)
        } else {
            (2, 0x7f80_0000, 0x007f_ffff)
        };
        self.a.mov_imm(double, T0, exponent);
        self.a.v_dup(width, 3, T0);
        self.a.v_and(3, 3, reg);
        self.a.v_cmeq0(width, 3, 3);
        self.a.mov_imm(double, T0, fraction);
        self.a.v_dup(width, 4, T0);
        self.a.v_and(3, 3, 4);
        self.a.v_bic(reg, reg, 3);
    }

    pub(super) fn flush_minmax_result(&mut self, reg: u32, double: bool) {
        self.a.ldr(32, T0, XCPU, OFF_MXCSR);
        self.a.ubfx(T0, T0, 6, 1);
        let keep = self.a.cbz(false, T0);
        self.flush_denorm_lanes(reg, double);
        self.a.bind(keep);
    }

    /// Emit one 128-bit lane of `K_VFP_*` on `(v0 = a, v1 = b) -> v0`.
    pub(super) fn emit_vfp_lane(&mut self, sub: u8, imm8: u8) {
        use fast::*;
        match sub {
            // minss / maxss / minsd / maxsd: fcmp + fcsel (caller already flushed NZCV).
            4 | 5 | 10 | 11 => {
                let is_sd = sub >= 6;
                let is_max = sub == 5 || sub == 11;
                self.a.fcmp(is_sd, 0, 1);
                self.a.s_fcsel(is_sd, 2, 0, 1, if is_max { GT } else { MI });
                self.flush_minmax_result(2, is_sd);
                self.a.fmov_fg(is_sd, T0, 2);
                if is_sd {
                    self.a.v_ins_d(0, 0, T0);
                } else {
                    self.a.v_ins_s(0, 0, T0);
                }
            }
            // addss/subss/mulss/divss and *sd: scalar FP + inline NaN payload fixup.
            0..=3 | 6..=9 => {
                let is_sd = sub >= 6;
                let aop = if is_sd { sub - 6 } else { sub };
                self.a.s_farith(is_sd, aop, 2, 0, 1);
                if !self.fast_nan {
                    self.a.fcmp(is_sd, 2, 2);
                    let ok = self.a.bcond(VC);
                    self.emit_x86_nan_fixup(is_sd, false);
                    self.a.bind(ok);
                }
                self.a.fmov_fg(is_sd, T0, 2);
                if is_sd {
                    self.a.v_ins_d(0, 0, T0);
                } else {
                    self.a.v_ins_s(0, 0, T0);
                }
            }
            // minps / maxps / minpd / maxpd: fcmgt + bsl (no NZCV touched!).
            16 | 17 | 22 | 23 => {
                let is_pd = sub >= 18;
                let is_max = sub == 17 || sub == 23;
                if is_max {
                    self.a.v_fcmgt(is_pd, 2, 0, 1);
                } else {
                    self.a.v_fcmgt(is_pd, 2, 1, 0);
                }
                self.a.v_bsl(2, 0, 1);
                self.flush_minmax_result(2, is_pd);
                self.a.v_mov(0, 2);
            }
            // addps/subps/mulps/divps and *pd: vector FP + inline NaN payload fixup.
            12..=15 | 18..=21 => {
                let is_pd = sub >= 18;
                let aop = if is_pd { sub - 18 } else { sub - 12 };
                if self.fast_nan {
                    self.a.v_farith(is_pd, aop, 0, 0, 1);
                } else {
                    self.a.v_farith(is_pd, aop, 2, 0, 1);
                    self.a.v_fcmeq(is_pd, 3, 2, 2);
                    self.a.v_uminv_s(3, 3);
                    self.a.fmov_fg(false, T0, 3);
                    let ok = self.a.cbnz(false, T0);
                    self.emit_x86_nan_fixup(is_pd, false);
                    self.a.bind(ok);
                    self.a.v_mov(0, 2);
                }
            }
            VF_SQRTSS | VF_SQRTSD => {
                let is_sd = sub == VF_SQRTSD;
                self.a.s_fsqrt(is_sd, 2, 1);
                if !self.fast_nan {
                    self.a.fcmp(is_sd, 2, 2);
                    let ok = self.a.bcond(VC);
                    self.emit_x86_nan_fixup(is_sd, true);
                    self.a.bind(ok);
                }
                self.a.fmov_fg(is_sd, T0, 2);
                if is_sd {
                    self.a.v_ins_d(0, 0, T0);
                } else {
                    self.a.v_ins_s(0, 0, T0);
                }
            }
            VF_SQRTPS | VF_SQRTPD => {
                let is_pd = sub == VF_SQRTPD;
                if self.fast_nan {
                    self.a.v_fsqrt(is_pd, 0, 1);
                } else {
                    self.a.v_fsqrt(is_pd, 2, 1);
                    self.a.v_fcmeq(is_pd, 3, 2, 2);
                    self.a.v_uminv_s(3, 3);
                    self.a.fmov_fg(false, T0, 3);
                    let ok = self.a.cbnz(false, T0);
                    self.emit_x86_nan_fixup(is_pd, true);
                    self.a.bind(ok);
                    self.a.v_mov(0, 2);
                }
            }
            VF_CVTSS2SD => {
                self.a.fcvt_d_s(2, 1);
                self.a.fmov_fg(true, T0, 2);
                self.a.v_ins_d(0, 0, T0);
            }
            VF_CVTSD2SS => {
                self.a.fcvt_s_d(2, 1);
                self.a.fmov_fg(false, T0, 2);
                self.a.v_ins_s(0, 0, T0);
            }
            VF_CVTDQ2PS => {
                self.a.v_scvtf_4s(0, 1);
            }
            VF_CVTTPS2DQ | VF_CVTPS2DQ => {
                let src = if sub == VF_CVTTPS2DQ {
                    1
                } else {
                    self.a.v_frintn_4s(5, 1);
                    5
                };
                self.a.v_fcvtzs_4s(0, src);
                self.a.mov_imm(false, T0, 0x4F00_0000); // 2147483648.0f32
                self.a.v_dup(2, 2, T0);
                self.a.v_fcmgt(false, 3, 2, src); // 1s where src < 2^31 (ordered)
                self.a.mov_imm(false, T0, 0x8000_0000);
                self.a.v_dup(2, 4, T0);
                self.a.v_bsl(3, 0, 4);
                self.a.v_mov(0, 3);
            }
            VF_RCPSS => {
                self.flush_denorm_lanes(1, false);
                self.a.mov_imm(false, T0, 0x3F80_0000);
                self.a.fmov_gf(false, 2, T0);
                self.a.s_farith(false, 3, 2, 2, 1);
                self.flush_denorm_lanes(2, false);
                self.a.fmov_fg(false, T0, 2);
                self.a.v_ins_s(0, 0, T0);
            }
            // rsqrt: x86 NaN rules on the sqrt (sqrt(-x) = -qNaN indefinite, NaN quieted),
            // then 1/r (which propagates that NaN unchanged). Vector ops only (NZCV kept).
            VF_RSQRTSS => {
                self.flush_denorm_lanes(1, false);
                self.a.v_fsqrt(false, 2, 1);
                self.emit_vnan_check_fix(false, true);
                self.a.mov_imm(false, T0, 0x3F80_0000);
                self.a.fmov_gf(false, 3, T0);
                self.a.s_farith(false, 3, 2, 3, 2);
                self.flush_denorm_lanes(2, false);
                self.a.fmov_fg(false, T0, 2);
                self.a.v_ins_s(0, 0, T0);
            }
            VF_RCPPS => {
                self.flush_denorm_lanes(1, false);
                self.a.mov_imm(false, T0, 0x3F80_0000);
                self.a.v_dup(2, 2, T0);
                self.a.v_farith(false, 3, 0, 2, 1);
                self.flush_denorm_lanes(0, false);
            }
            VF_RSQRTPS => {
                self.flush_denorm_lanes(1, false);
                self.a.v_fsqrt(false, 2, 1);
                self.emit_vnan_check_fix(false, true);
                self.a.mov_imm(false, T0, 0x3F80_0000);
                self.a.v_dup(2, 3, T0);
                self.a.v_farith(false, 3, 0, 3, 2);
                self.flush_denorm_lanes(0, false);
            }
            // addsub / hadd / hsub: lane-wise `l op r` + the x86 NaN fixup with `l` as the
            // first source (both NaN -> l quieted; invalid -> -qNaN indefinite).
            VF_ADDSUBPS => {
                self.a.v_farith(false, 1, 2, 0, 1); // fsub
                self.a.v_farith(false, 0, 3, 0, 1); // fadd
                self.a.mov_imm(true, T0, 0x0000_0000_FFFF_FFFF);
                self.a.v_dup(3, 4, T0);
                self.a.v_bsl(4, 2, 3);
                self.a.v_mov(2, 4);
                self.emit_vnan_check_fix(false, false);
                self.a.v_mov(0, 2);
            }
            VF_ADDSUBPD => {
                self.a.v_farith(true, 1, 2, 0, 1); // fsub
                self.a.v_farith(true, 0, 3, 0, 1); // fadd
                self.a.v_ins_lane(3, 2, 1, 3, 1);
                self.emit_vnan_check_fix(true, false);
                self.a.v_mov(0, 2);
            }
            VF_HADDPS | VF_HSUBPS | VF_HADDPD | VF_HSUBPD => {
                let is_pd = matches!(sub, VF_HADDPD | VF_HSUBPD);
                let sz = if is_pd { 3 } else { 2 };
                let aop = if matches!(sub, VF_HADDPS | VF_HADDPD) { 0 } else { 1 };
                self.a.v_uzp1(sz, 4, 0, 1); // l = even elements of a:b
                self.a.v_uzp2(sz, 1, 0, 1); // r = odd elements of a:b
                self.a.v_mov(0, 4);
                self.a.v_farith(is_pd, aop, 2, 0, 1);
                self.emit_vnan_check_fix(is_pd, false);
                self.a.v_mov(0, 2);
            }
            VF_ROUNDSS | VF_ROUNDSD => {
                let is_sd = sub == VF_ROUNDSD;
                self.a.s_frint(is_sd, imm8, 2, 1);
                self.a.fmov_fg(is_sd, T0, 2);
                if is_sd {
                    self.a.v_ins_d(0, 0, T0);
                } else {
                    self.a.v_ins_s(0, 0, T0);
                }
            }
            VF_ROUNDPS | VF_ROUNDPD => {
                let is_pd = sub == VF_ROUNDPD;
                self.a.v_frint(is_pd, imm8, 0, 1);
            }
            VF_CMPSS | VF_CMPSD => {
                let is_sd = sub == VF_CMPSD;
                self.emit_vcmp_pred(is_sd, imm8);
                self.a.fmov_fg(is_sd, T0, 2);
                if is_sd {
                    self.a.v_ins_d(0, 0, T0);
                } else {
                    self.a.v_ins_s(0, 0, T0);
                }
            }
            VF_CMPPS | VF_CMPPD => {
                let is_pd = sub == VF_CMPPD;
                self.emit_vcmp_pred(is_pd, imm8);
                self.a.v_mov(0, 2);
            }
            VF_DPPS => {
                let mut m_in_lo = 0u64;
                let mut m_in_hi = 0u64;
                if imm8 & 0x10 != 0 {
                    m_in_lo |= 0xFFFF_FFFF;
                }
                if imm8 & 0x20 != 0 {
                    m_in_lo |= 0xFFFF_FFFF_0000_0000;
                }
                if imm8 & 0x40 != 0 {
                    m_in_hi |= 0xFFFF_FFFF;
                }
                if imm8 & 0x80 != 0 {
                    m_in_hi |= 0xFFFF_FFFF_0000_0000;
                }
                self.a.mov_imm(true, T0, m_in_lo);
                self.a.mov_imm(true, T1, m_in_hi);
                self.a.fmov_gf(true, 4, T0);
                self.a.v_ins_d(4, 1, T1);
                self.a.v_and(2, 0, 4);
                self.a.v_and(3, 1, 4);
                self.a.v_farith(false, 2, 2, 2, 3); // fmul v2.4s, v2.4s, v3.4s
                self.a.v_faddp(false, 2, 2, 2);
                self.a.v_faddp(false, 2, 2, 2);
                self.a.fmov_fg(false, T0, 2);
                self.a.v_dup(2, 0, T0);
                let mut m_out_lo = 0u64;
                let mut m_out_hi = 0u64;
                if imm8 & 0x01 != 0 {
                    m_out_lo |= 0xFFFF_FFFF;
                }
                if imm8 & 0x02 != 0 {
                    m_out_lo |= 0xFFFF_FFFF_0000_0000;
                }
                if imm8 & 0x04 != 0 {
                    m_out_hi |= 0xFFFF_FFFF;
                }
                if imm8 & 0x08 != 0 {
                    m_out_hi |= 0xFFFF_FFFF_0000_0000;
                }
                self.a.mov_imm(true, T0, m_out_lo);
                self.a.mov_imm(true, T1, m_out_hi);
                self.a.fmov_gf(true, 4, T0);
                self.a.v_ins_d(4, 1, T1);
                self.a.v_and(0, 0, 4);
            }
            _ => {}
        }
    }

    /// Translate any `K_VSHUF_*`, `K_VINT_*`, or `K_VFP_*` instruction natively to arm64 NEON.
    pub(super) fn vop(&mut self, k: usize, op: &FastOp) {
        use fast::*;
        let sub = op.x & 0x3F;
        let imm8 = op.imm as u8;
        let s1 = ((op.imm >> 8) as u32 & 15) * 16;
        let is_256 = (op.w & 3) == VM_V256;
        let is_vfp = matches!(op.kind, K_VFP_RR | K_VFP_RM);
        let touches_nzcv = is_vfp
            && (matches!(sub, 4 | 5 | 10 | 11)
                || (!self.fast_nan && matches!(sub, 0..=3 | 6..=9 | VF_SQRTSS | VF_SQRTSD)));
        if touches_nzcv {
            self.flush_flags(self.live[k]);
            self.fs.host = 0;
        }
        self.vload_b(k, op);
        self.xsrc(0, (op.imm >> 8) as u8);
        match op.kind {
            K_VSHUF_RR | K_VSHUF_RM => self.emit_vshuf_lane(sub, imm8),
            K_VINT_RR | K_VINT_RM => self.emit_vint_lane(sub, imm8),
            _ => self.emit_vfp_lane(sub, imm8),
        }
        if is_256 {
            self.a.v_mov(6, 0);
            self.a.ldr(128, 0, XCPU, OFF_YMM_HI + s1);
            if !(matches!(op.kind, K_VSHUF_RR | K_VSHUF_RM) && (VS_PSRLW..=VS_PSLLQ).contains(&sub)) {
                self.a.v_mov(1, 7);
            }
            let hi_imm = if matches!(op.kind, K_VSHUF_RR | K_VSHUF_RM) {
                match sub {
                    VS_SHUFPD | VS_BLENDPD | VS_VPERMILPD_I => imm8 >> 2,
                    VS_BLENDPS => imm8 >> 4,
                    _ => imm8,
                }
            } else {
                imm8
            };
            match op.kind {
                K_VSHUF_RR | K_VSHUF_RM => self.emit_vshuf_lane(sub, hi_imm),
                K_VINT_RR | K_VINT_RM => self.emit_vint_lane(sub, hi_imm),
                _ => self.emit_vfp_lane(sub, hi_imm),
            }
        }
        self.vstore_dst(op);
    }

    /// Translate extended integer/stack/flag fast ops (`K_BSWAP_R..=K_FLAG_OP`) to native arm64.
    pub(super) fn ext_op(&mut self, k: usize, op: &FastOp) {
        use fast::*;
        let w = op.w;
        let wb = w as u32;
        let la = self.live[k + 1];
        match op.kind {
            K_BSWAP_R => {
                self.a.rev(w == 64, g(op.r0), g(op.r0));
                self.set_lb(op.r0, w);
            }
            K_BSR_RR | K_BSR_RM => {
                let v = if op.kind == K_BSR_RR {
                    self.ua(op.r1, wb)
                } else {
                    let a = self.addr_into(op, XA);
                    self.load(k, T0, a, wb, Self::chk_of(op));
                    T0
                };
                if la & FZ != 0 {
                    self.a.clz(true, T2, v);
                    self.a.lsr_i(true, T2, T2, 6);
                    self.a.str(8, T2, XCPU, OFF_ZF);
                    self.fs.host &= !FZ;
                    self.fs.mem |= FZ;
                }
                if tracked(op.r0) {
                    self.flush_lb_reg(op.r0 as usize);
                }
                let skip = self.a.cbz(true, v);
                self.a.clz(true, T1, v);
                self.a.movz(false, T2, 63, 0);
                self.a.addsub_rr(false, true, false, T1, T2, T1, 0);
                let d = g(op.r0);
                match wb {
                    64 => self.a.mov_rr(true, d, T1),
                    32 => self.a.mov_rr(false, d, T1),
                    _ => self.a.bfxil0(d, T1, wb),
                }
                self.lb_store_now(op.r0, w);
                self.a.bind(skip);
            }
            K_TZCNT_RR | K_TZCNT_RM => {
                let v = if op.kind == K_TZCNT_RR {
                    self.ua(op.r1, wb)
                } else {
                    let a = self.addr_into(op, XA);
                    self.load(k, T0, a, wb, Self::chk_of(op));
                    T0
                };
                if v != T0 {
                    self.a.mov_rr(true, T0, v);
                }
                match wb {
                    64 => {
                        self.a.rbit(true, T1, T0);
                        self.a.clz(true, T1, T1);
                    }
                    32 => {
                        self.a.rbit(false, T1, T0);
                        self.a.clz(false, T1, T1);
                    }
                    _ => {
                        self.a.mov_imm(false, T2, 0x1_0000);
                        self.a.logic_rr(false, 1, T1, T0, T2, 0, 0);
                        self.a.rbit(false, T1, T1);
                        self.a.clz(false, T1, T1);
                    }
                }
                self.wr_gpr(op.r0, w, T1);
                if la & FC != 0 {
                    self.a.clz(true, T2, T0);
                    self.a.lsr_i(true, T2, T2, 6);
                    self.a.str(8, T2, XCPU, OFF_CF);
                }
                if la & FZ != 0 {
                    self.a.clz(true, T2, T1);
                    self.a.lsr_i(true, T2, T2, 6);
                    self.a.str(8, T2, XCPU, OFF_ZF);
                }
                self.fs.host &= !(FC | FZ);
                self.fs.mem = (self.fs.mem & !(FC | FZ)) | (la & (FC | FZ));
            }
            K_LZCNT_RR | K_LZCNT_RM => {
                let v = if op.kind == K_LZCNT_RR {
                    self.ua(op.r1, wb)
                } else {
                    let a = self.addr_into(op, XA);
                    self.load(k, T0, a, wb, Self::chk_of(op));
                    T0
                };
                if v != T0 {
                    self.a.mov_rr(true, T0, v);
                }
                match wb {
                    64 => self.a.clz(true, T1, T0),
                    32 => self.a.clz(false, T1, T0),
                    _ => {
                        self.a.clz(false, T1, T0);
                        self.a.addsub_ri(false, true, false, T1, T1, 16, false);
                    }
                }
                self.wr_gpr(op.r0, w, T1);
                if la & FC != 0 {
                    self.a.clz(true, T2, T0);
                    self.a.lsr_i(true, T2, T2, 6);
                    self.a.str(8, T2, XCPU, OFF_CF);
                }
                if la & FZ != 0 {
                    self.a.clz(true, T2, T1);
                    self.a.lsr_i(true, T2, T2, 6);
                    self.a.str(8, T2, XCPU, OFF_ZF);
                }
                self.fs.host &= !(FC | FZ);
                self.fs.mem = (self.fs.mem & !(FC | FZ)) | (la & (FC | FZ));
            }
            K_POPCNT_RR | K_POPCNT_RM => {
                let v = if op.kind == K_POPCNT_RR {
                    self.ua(op.r1, wb)
                } else {
                    let a = self.addr_into(op, XA);
                    self.load(k, T0, a, wb, Self::chk_of(op));
                    T0
                };
                self.a.fmov_gf(true, 0, v);
                self.a.v_cnt_8b(0, 0);
                self.a.v_addv_8b(0, 0);
                self.a.fmov_fg(false, T1, 0);
                self.wr_gpr(op.r0, w, T1);
                if la & FZ != 0 {
                    self.a.clz(true, T2, T1);
                    self.a.lsr_i(true, T2, T2, 6);
                    self.a.str(8, T2, XCPU, OFF_ZF);
                }
                if la & FC != 0 {
                    self.a.str(8, ZR, XCPU, OFF_CF);
                }
                if la & FS != 0 {
                    self.a.str(8, ZR, XCPU, OFF_SF);
                }
                if la & FO != 0 {
                    self.a.str(8, ZR, XCPU, OFF_OF);
                }
                if la & FP != 0 {
                    self.a.movz(false, T2, 1, 0);
                    self.a.str(8, T2, XCPU, OFF_PFV);
                }
                self.fs = FlagState {
                    src: Src::None,
                    host: 0,
                    mem: (self.fs.mem & !FALL) | (la & FALL),
                };
            }
            K_BIT_RR | K_BIT_RI => {
                if op.kind == K_BIT_RI {
                    let b = (op.imm as u32) & (wb - 1);
                    if la & FC != 0 {
                        self.a.ubfx(T0, g(op.r0), b, 1);
                        self.a.str(8, T0, XCPU, OFF_CF);
                        self.fs.host &= !FC;
                        self.fs.mem |= FC;
                    }
                    self.a.mov_imm(true, T2, 1u64 << b);
                } else {
                    self.a.logic_ri(false, 0, T1, g(op.r1), enc_logic_imm((wb - 1) as u64, false).unwrap());
                    if la & FC != 0 {
                        self.a.shiftv(true, 9, T0, g(op.r0), T1);
                        self.a.ubfx(T0, T0, 0, 1);
                        self.a.str(8, T0, XCPU, OFF_CF);
                        self.fs.host &= !FC;
                        self.fs.mem |= FC;
                    }
                    self.a.movz(true, T2, 1, 0);
                    self.a.shiftv(true, 8, T2, T2, T1);
                }
                match op.x & 0x3F {
                    0 => self.a.logic_rr(true, 1, T1, g(op.r0), T2, 0, 0),
                    1 => self.a.bic_rr(true, T1, g(op.r0), T2),
                    _ => self.a.logic_rr(true, 2, T1, g(op.r0), T2, 0, 0),
                }
                self.wr_gpr(op.r0, w, T1);
            }
            K_SHD_RI => {
                let cnt = op.imm as u32;
                let shrd = (op.x & 0x3F) != 0;
                let ua = self.ua(op.r0, wb);
                if ua != T0 {
                    self.a.mov_rr(true, T0, ua);
                }
                match wb {
                    64 => self.a.mov_rr(true, T3, g(op.r1)),
                    32 => self.a.mov_rr(false, T3, g(op.r1)),
                    _ => self.a.ubfm(true, T3, g(op.r1), 0, wb - 1),
                }
                if shrd {
                    self.a.lsr_i(true, T1, T0, cnt);
                    if cnt < wb {
                        self.a.lsl_i(true, T4, T3, wb - cnt);
                        self.a.logic_rr(true, 1, T1, T1, T4, 0, 0);
                        if wb < 64 {
                            self.a.ubfm(true, T1, T1, 0, wb - 1);
                        }
                    } else {
                        self.a.mov_rr(true, T1, T3);
                    }
                    if la & FC != 0 {
                        self.a.ubfx(T2, T0, cnt - 1, 1);
                    }
                } else {
                    if cnt < wb {
                        self.a.lsl_i(true, T1, T0, cnt);
                        self.a.lsr_i(true, T4, T3, wb - cnt);
                        self.a.logic_rr(true, 1, T1, T1, T4, 0, 0);
                        if wb < 64 {
                            self.a.ubfm(true, T1, T1, 0, wb - 1);
                        }
                    } else {
                        self.a.mov_rr(true, T1, T3);
                    }
                    if la & FC != 0 {
                        self.a.ubfx(T2, T0, wb - cnt, 1);
                    }
                }
                self.shift_flags(la, wb, true);
                self.wr_gpr(op.r0, w, T1);
                self.fs = FlagState {
                    src: Src::None,
                    host: 0,
                    mem: (self.fs.mem & !FALL) | (la & FALL),
                };
            }
            K_SHD_RCL => {
                self.flush_flags(la);
                if tracked(op.r0) {
                    self.flush_lb_reg(op.r0 as usize);
                }
                let mask = if wb == 64 { 63 } else { 31 };
                self.a.logic_ri(false, 0, T6, g(1), enc_logic_imm(mask, false).unwrap());
                let skip = self.a.cbz(false, T6);
                let shrd = (op.x & 0x3F) != 0;
                let ua = self.ua(op.r0, wb);
                if ua != T0 {
                    self.a.mov_rr(true, T0, ua);
                }
                match wb {
                    64 => self.a.mov_rr(true, T3, g(op.r1)),
                    32 => self.a.mov_rr(false, T3, g(op.r1)),
                    _ => self.a.ubfm(true, T3, g(op.r1), 0, wb - 1),
                }
                self.a.movz(true, T4, wb, 0);
                self.a.addsub_rr(true, true, false, T4, T4, T6, 0); // T4 = wb - cnt
                if shrd {
                    self.a.shiftv(true, 9, T1, T0, T6);
                    self.a.shiftv(true, 8, T5, T3, T4);
                    self.a.logic_rr(true, 1, T1, T1, T5, 0, 0);
                    if wb < 64 {
                        self.a.ubfm(true, T1, T1, 0, wb - 1);
                    }
                    self.a.addsub_ri(true, true, false, T5, T6, 1, false);
                    self.a.shiftv(true, 9, T2, T0, T5);
                    self.a.ubfx(T2, T2, 0, 1);
                } else {
                    self.a.shiftv(true, 8, T1, T0, T6);
                    self.a.shiftv(true, 9, T5, T3, T4);
                    self.a.logic_rr(true, 1, T1, T1, T5, 0, 0);
                    if wb < 64 {
                        self.a.ubfm(true, T1, T1, 0, wb - 1);
                    }
                    self.a.shiftv(true, 9, T2, T0, T4);
                    self.a.ubfx(T2, T2, 0, 1);
                }
                self.shift_flags(la, wb, true);
                let d = g(op.r0);
                match wb {
                    64 => self.a.mov_rr(true, d, T1),
                    32 => self.a.mov_rr(false, d, T1),
                    _ => self.a.bfxil0(d, T1, wb),
                }
                self.lb_store_now(op.r0, w);
                self.a.bind(skip);
                self.fs = FlagState {
                    src: Src::None,
                    host: 0,
                    mem: (self.fs.mem & !FALL) | (la & FALL),
                };
            }
            K_ROT_RCL => {
                self.flush_flags(la);
                if tracked(op.r0) {
                    self.flush_lb_reg(op.r0 as usize);
                }
                let mask = if wb == 64 { 63 } else { 31 };
                self.a.logic_ri(false, 0, T6, g(1), enc_logic_imm(mask, false).unwrap());
                let skip = self.a.cbz(false, T6);
                let ror = (op.x & 0x3F) != 0;
                let ua = self.ua(op.r0, wb);
                match wb {
                    64 | 32 => {
                        let x = wb == 64;
                        if ror {
                            self.a.rorv(x, T1, ua, T6);
                        } else {
                            self.a.addsub_rr(false, true, false, T4, ZR, T6, 0);
                            self.a.rorv(x, T1, ua, T4);
                        }
                    }
                    _ => {
                        if ror {
                            self.a.logic_ri(false, 0, T4, T6, enc_logic_imm((wb - 1) as u64, false).unwrap());
                        } else {
                            self.a.addsub_rr(false, true, false, T4, ZR, T6, 0);
                            self.a.logic_ri(false, 0, T4, T4, enc_logic_imm((wb - 1) as u64, false).unwrap());
                        }
                        self.a.lsl_i(false, T2, ua, wb);
                        self.a.logic_rr(false, 1, T2, ua, T2, 0, 0);
                        self.a.shiftv(false, 9, T1, T2, T4);
                        self.a.ubfm(false, T1, T1, 0, wb - 1);
                    }
                }
                let d = g(op.r0);
                match wb {
                    64 => self.a.mov_rr(true, d, T1),
                    32 => self.a.mov_rr(false, d, T1),
                    _ => self.a.bfxil0(d, T1, wb),
                }
                self.lb_store_now(op.r0, w);
                if la & (FC | FO) != 0 {
                    self.a.ubfx(T2, T1, wb - 1, 1);
                    let cf_reg = if ror {
                        if la & FO != 0 {
                            self.a.ubfx(T3, T1, wb - 2, 1);
                            self.a.logic_rr(false, 2, T3, T2, T3, 0, 0);
                        }
                        T2
                    } else {
                        self.a.ubfx(T4, T1, 0, 1);
                        if la & FO != 0 {
                            self.a.logic_rr(false, 2, T3, T2, T4, 0, 0);
                        }
                        T4
                    };
                    if la & FC != 0 {
                        self.a.str(8, cf_reg, XCPU, OFF_CF);
                    }
                    if la & FO != 0 {
                        self.a.str(8, T3, XCPU, OFF_OF);
                    }
                }
                if wb == 32 {
                    let done = self.a.b();
                    self.a.bind(skip);
                    self.a.mov_rr(false, d, d);
                    self.lb_store_now(op.r0, 32);
                    self.a.bind(done);
                } else {
                    self.a.bind(skip);
                }
                self.fs = FlagState {
                    src: Src::None,
                    host: 0,
                    mem: (self.fs.mem & !FALL) | (la & FALL),
                };
            }
            K_MUL1_R | K_MUL1_M => self.mul1_div(k, op),
            K_XADD_RR => {
                let need_nzcv = la & FNZCV != 0;
                let need_pf = la & FP != 0;
                let a_reg = g(op.r0);
                let b_reg = g(op.r1);
                self.a.mov_rr(true, T0, a_reg); // save old r0
                let res = if wb >= 32 {
                    self.a.addsub_rr(wb == 64, false, need_nzcv, T2, T0, b_reg, 0);
                    T2
                } else {
                    let sh = 64 - wb;
                    if need_nzcv {
                        self.a.lsl_i(true, T3, T0, sh);
                        self.a.lsl_i(true, T4, b_reg, sh);
                        self.a.addsub_rr(true, false, true, T5, T3, T4, 0);
                        self.a.lsr_i(true, T2, T5, sh);
                    } else {
                        self.a.addsub_rr(false, false, false, T2, T0, b_reg, 0);
                    }
                    T2
                };
                self.wr_gpr(op.r0, w, res);
                self.wr_gpr(op.r1, w, T0);
                if need_pf {
                    self.a.str(8, res, XCPU, OFF_PFV);
                }
                self.fs.mem &= !FALL;
                if need_pf {
                    self.fs.mem |= FP;
                }
                self.fs.host = if need_nzcv { la & FNZCV } else { 0 };
                self.fs.src = Src::Add;
            }
            K_CMPXCHG_RR => {
                let need_pf = la & FP != 0;
                self.flush_lb_reg(0);
                if tracked(op.r0) {
                    self.flush_lb_reg(op.r0 as usize);
                }
                let sh = 64 - wb;
                if wb == 64 {
                    self.a.addsub_rr(true, true, true, T2, g(0), g(op.r0), 0);
                } else if wb == 32 {
                    self.a.addsub_rr(false, true, true, T2, g(0), g(op.r0), 0);
                } else {
                    self.a.lsl_i(true, T3, g(0), sh);
                    self.a.lsl_i(true, T4, g(op.r0), sh);
                    self.a.addsub_rr(true, true, true, T5, T3, T4, 0);
                    self.a.lsr_i(true, T2, T5, sh);
                }
                let neq = self.a.bcond(NE);
                // Equal: r0 = r1
                let d0 = g(op.r0);
                let s1 = g(op.r1);
                match wb {
                    64 => self.a.mov_rr(true, d0, s1),
                    32 => self.a.mov_rr(false, d0, s1),
                    _ => self.a.bfxil0(d0, s1, wb),
                }
                self.lb_store_now(op.r0, w);
                let done = self.a.b();
                self.a.bind(neq);
                // Not equal: rax = r0
                let r_ax = g(0);
                match wb {
                    64 => self.a.mov_rr(true, r_ax, d0),
                    32 => self.a.mov_rr(false, r_ax, d0),
                    _ => self.a.bfxil0(r_ax, d0, wb),
                }
                self.lb_store_now(0, w);
                self.a.bind(done);
                if need_pf {
                    self.a.str(8, T2, XCPU, OFF_PFV);
                }
                self.fs.mem &= !FALL;
                if need_pf {
                    self.fs.mem |= FP;
                }
                self.fs.host = la & FNZCV;
                self.fs.src = Src::Sub;
            }
            K_SETCC_M => {
                let c = self.cond(op.x & 0x3F);
                match c {
                    Cond::Host(cnd) => self.a.cset(T0, cnd),
                    Cond::Reg(r, inv) => {
                        if inv {
                            self.a.logic_ri(false, 2, T0, r, enc_logic_imm(1, false).unwrap());
                        } else {
                            self.a.mov_rr(false, T0, r);
                        }
                    }
                    Cond::Const(b) => self.a.movz(false, T0, b as u32, 0),
                }
                let a = self.addr_into(op, XA);
                self.store(k, T0, a, 8, Self::chk_of(op));
            }
            K_PUSH_I if pushpop_enabled() => {
                // Plain pre-indexed push (see `pushpop_enabled`).
                let src = if op.imm == 0 {
                    ZR
                } else {
                    self.a.mov_imm(true, T0, op.imm);
                    T0
                };
                self.a.str_pre(src, g(4), -8);
            }
            K_PUSH_I => {
                self.a.addsub_ri(true, true, false, XA, g(4), 8, false);
                if op.imm == 0 {
                    self.store(k, ZR, XA, 64, Chk::Stack);
                } else {
                    self.a.mov_imm(true, T0, op.imm);
                    self.store(k, T0, XA, 64, Chk::Stack);
                }
                self.a.mov_rr(true, g(4), XA);
            }
            K_PUSH_M if pushpop_enabled() => {
                let a = self.addr_into(op, XA);
                self.load(k, T0, a, 64, Self::chk_of(op));
                self.a.str_pre(T0, g(4), -8);
            }
            K_PUSH_M => {
                let a = self.addr_into(op, XA);
                self.load(k, T0, a, 64, Self::chk_of(op));
                self.a.addsub_ri(true, true, false, XA, g(4), 8, false);
                self.store(k, T0, XA, 64, Chk::Stack);
                self.a.mov_rr(true, g(4), XA);
            }
            K_POP_M => {
                self.load(k, T0, g(4), 64, Chk::Stack);
                self.a.addsub_ri(true, false, false, g(4), g(4), 8, false);
                let mut a = self.addr_into(op, XA);
                if a == g(4) {
                    // `pop [rsp]`: the address is the incremented rsp (copy it before rsp is
                    // restored for the store's side exit).
                    self.a.mov_rr(true, XA, g(4));
                    a = XA;
                }
                self.a.addsub_ri(true, true, false, g(4), g(4), 8, false);
                self.store(k, T0, a, 64, Self::chk_of(op));
                self.a.addsub_ri(true, false, false, g(4), g(4), 8, false);
            }
            K_LEAVE => {
                self.a.mov_rr(true, g(4), g(5));
                self.load(k, T0, g(4), 64, Chk::Stack);
                self.a.addsub_ri(true, false, false, g(4), g(4), 8, false);
                self.wr_gpr(5, 64, T0);
            }
            K_FLAG_OP => match op.x & 0x3F {
                0 => {
                    if la & FC != 0 {
                        self.a.str(8, ZR, XCPU, OFF_CF);
                    }
                    self.fs.host &= !FC;
                    self.fs.mem = (self.fs.mem & !FC) | (la & FC);
                }
                1 => {
                    if la & FC != 0 {
                        self.a.movz(false, T0, 1, 0);
                        self.a.str(8, T0, XCPU, OFF_CF);
                    }
                    self.fs.host &= !FC;
                    self.fs.mem = (self.fs.mem & !FC) | (la & FC);
                }
                2 => {
                    if la & FC != 0 {
                        self.flag_reg(FC, T0);
                        self.a.logic_ri(false, 2, T0, T0, enc_logic_imm(1, false).unwrap());
                        self.a.str(8, T0, XCPU, OFF_CF);
                    }
                    self.fs.host &= !FC;
                    self.fs.mem = (self.fs.mem & !FC) | (la & FC);
                }
                3 => self.a.str(8, ZR, XCPU, OFF_DF),
                4 => {
                    self.a.movz(false, T0, 1, 0);
                    self.a.str(8, T0, XCPU, OFF_DF);
                }
                6 => {
                    self.a.dsb_ish();
                    self.a.isb();
                }
                _ => self.a.dmb_ish(),
            },
            K_SHIFT_MI => {
                let c = op.imm as u32;
                let sub = op.x & 0x3F;
                let addr = self.addr_into(op, XA);
                self.load(k, T0, addr, wb, Self::chk_of(op));
                let mut of_zero = false;
                match sub {
                    SHIFT_SHL => {
                        if c >= 64 {
                            self.a.movz(true, T1, 0, 0);
                        } else {
                            self.a.lsl_i(true, T1, T0, c);
                            if wb < 64 {
                                self.a.ubfm(true, T1, T1, 0, wb - 1);
                            }
                        }
                        if la & (FC | FO) != 0 {
                            if c <= wb {
                                self.a.ubfx(T2, T0, wb - c, 1);
                            } else {
                                self.a.movz(true, T2, 0, 0);
                            }
                            self.a.ubfx(T3, T1, wb - 1, 1);
                            self.a.logic_rr(true, 2, T3, T3, T2, 0, 0);
                        }
                    }
                    SHIFT_SHR => {
                        self.a.lsr_i(true, T1, T0, c);
                        if la & FC != 0 {
                            self.a.ubfx(T2, T0, c - 1, 1);
                        }
                        if la & FO != 0 {
                            self.a.ubfx(T3, T0, wb - 1, 1);
                        }
                    }
                    _ => {
                        let sa = if wb < 64 {
                            self.a.sbfm(true, T4, T0, 0, wb - 1);
                            T4
                        } else {
                            T0
                        };
                        self.a.asr_i(true, T1, sa, c.min(63));
                        if wb < 64 {
                            self.a.ubfm(true, T1, T1, 0, wb - 1);
                        }
                        if la & FC != 0 {
                            self.a.ubfx(T2, sa, (c - 1).min(63), 1);
                        }
                        of_zero = true;
                    }
                }
                self.shift_flags(la, wb, of_zero);
                self.store_checked_addr(T1, addr, wb);
                self.fs.host = 0;
                self.fs.mem = (self.fs.mem & !FALL) | (la & FALL);
                self.fs.src = Src::None;
            }
            K_SHIFT_MCL => {
                self.flush_flags(la);
                let addr = self.addr_into(op, XA);
                self.load(k, T0, addr, wb, Self::chk_of(op));
                let mask = if wb == 64 { 63 } else { 31 };
                self.a.logic_ri(false, 0, T6, g(1), enc_logic_imm(mask, false).unwrap());
                let skip = self.a.cbz(false, T6);
                let mut of_zero = false;
                match op.x & 0x3F {
                    SHIFT_SHL => {
                        if wb < 64 {
                            self.a.shiftv(true, 8, T4, T0, T6);
                            self.a.ubfx(T2, T4, wb, 1);
                            self.a.ubfm(true, T1, T4, 0, wb - 1);
                        } else {
                            self.a.shiftv(true, 8, T1, T0, T6);
                            self.a.movz(true, T4, 64, 0);
                            self.a.addsub_rr(true, true, false, T4, T4, T6, 0);
                            self.a.shiftv(true, 9, T2, T0, T4);
                            self.a.ubfx(T2, T2, 0, 1);
                        }
                        self.a.ubfx(T3, T1, wb - 1, 1);
                        self.a.logic_rr(true, 2, T3, T3, T2, 0, 0);
                    }
                    SHIFT_SHR => {
                        self.a.shiftv(true, 9, T1, T0, T6);
                        self.a.addsub_ri(true, true, false, T4, T6, 1, false);
                        self.a.shiftv(true, 9, T2, T0, T4);
                        self.a.ubfx(T2, T2, 0, 1);
                        self.a.ubfx(T3, T0, wb - 1, 1);
                    }
                    _ => {
                        let sa = if wb < 64 {
                            self.a.sbfm(true, XS, T0, 0, wb - 1);
                            XS
                        } else {
                            T0
                        };
                        self.a.shiftv(true, 10, T1, sa, T6);
                        if wb < 64 {
                            self.a.ubfm(true, T1, T1, 0, wb - 1);
                        }
                        self.a.addsub_ri(true, true, false, T4, T6, 1, false);
                        self.a.shiftv(true, 10, T2, sa, T4);
                        self.a.ubfx(T2, T2, 0, 1);
                        of_zero = true;
                    }
                }
                self.shift_flags(la, wb, of_zero);
                self.store_checked_addr(T1, addr, wb);
                self.a.bind(skip);
                self.fs.host = 0;
                self.fs.mem = (self.fs.mem & !FALL) | (la & FALL);
                self.fs.src = Src::None;
            }
            K_ROT_MI => {
                let cnt = op.imm as u32;
                let ror = (op.x & 0x3F) != 0;
                let addr = self.addr_into(op, XA);
                self.load(k, T0, addr, wb, Self::chk_of(op));
                match wb {
                    64 => self.a.ror_i(true, T1, T0, if ror { cnt } else { 64 - cnt }),
                    32 => self.a.ror_i(false, T1, T0, if ror { cnt } else { 32 - cnt }),
                    _ => {
                        let c_mod = cnt % wb;
                        if c_mod == 0 {
                            self.a.mov_rr(false, T1, T0);
                        } else {
                            let rcnt = if ror { c_mod } else { wb - c_mod };
                            self.a.lsr_i(false, T1, T0, rcnt);
                            self.a.lsl_i(false, T2, T0, wb - rcnt);
                            self.a.logic_rr(false, 1, T1, T1, T2, 0, 0);
                            self.a.ubfm(false, T1, T1, 0, wb - 1);
                        }
                    }
                }
                self.store_checked_addr(T1, addr, wb);
                if la & (FC | FO) != 0 {
                    self.a.ubfx(T2, T1, wb - 1, 1);
                    let cf_reg = if ror {
                        if la & FO != 0 {
                            self.a.ubfx(T3, T1, wb - 2, 1);
                            self.a.logic_rr(false, 2, T3, T2, T3, 0, 0);
                        }
                        T2
                    } else {
                        self.a.ubfx(T4, T1, 0, 1);
                        if la & FO != 0 {
                            self.a.logic_rr(false, 2, T3, T2, T4, 0, 0);
                        }
                        T4
                    };
                    if la & FC != 0 {
                        self.a.str(8, cf_reg, XCPU, OFF_CF);
                    }
                    if la & FO != 0 {
                        self.a.str(8, T3, XCPU, OFF_OF);
                    }
                    self.fs.host &= !(FC | FO);
                    self.fs.mem = (self.fs.mem & !(FC | FO)) | (la & (FC | FO));
                }
            }
            K_ROT_MCL => {
                self.flush_flags(la);
                let addr = self.addr_into(op, XA);
                self.load(k, T0, addr, wb, Self::chk_of(op));
                let mask = if wb == 64 { 63 } else { 31 };
                self.a.logic_ri(false, 0, T6, g(1), enc_logic_imm(mask, false).unwrap());
                let skip = self.a.cbz(false, T6);
                let ror = (op.x & 0x3F) != 0;
                match wb {
                    64 | 32 => {
                        let x = wb == 64;
                        if ror {
                            self.a.rorv(x, T1, T0, T6);
                        } else {
                            self.a.addsub_rr(false, true, false, T4, ZR, T6, 0);
                            self.a.rorv(x, T1, T0, T4);
                        }
                    }
                    _ => {
                        if ror {
                            self.a.logic_ri(false, 0, T4, T6, enc_logic_imm((wb - 1) as u64, false).unwrap());
                        } else {
                            self.a.addsub_rr(false, true, false, T4, ZR, T6, 0);
                            self.a.logic_ri(false, 0, T4, T4, enc_logic_imm((wb - 1) as u64, false).unwrap());
                        }
                        self.a.lsl_i(false, T2, T0, wb);
                        self.a.logic_rr(false, 1, T2, T0, T2, 0, 0);
                        self.a.shiftv(false, 9, T1, T2, T4);
                        self.a.ubfm(false, T1, T1, 0, wb - 1);
                    }
                }
                self.store_checked_addr(T1, addr, wb);
                if la & (FC | FO) != 0 {
                    self.a.ubfx(T2, T1, wb - 1, 1);
                    let cf_reg = if ror {
                        if la & FO != 0 {
                            self.a.ubfx(T3, T1, wb - 2, 1);
                            self.a.logic_rr(false, 2, T3, T2, T3, 0, 0);
                        }
                        T2
                    } else {
                        self.a.ubfx(T4, T1, 0, 1);
                        if la & FO != 0 {
                            self.a.logic_rr(false, 2, T3, T2, T4, 0, 0);
                        }
                        T4
                    };
                    if la & FC != 0 {
                        self.a.str(8, cf_reg, XCPU, OFF_CF);
                    }
                    if la & FO != 0 {
                        self.a.str(8, T3, XCPU, OFF_OF);
                    }
                }
                self.a.bind(skip);
                self.fs.host &= !(FC | FO);
                self.fs.mem = (self.fs.mem & !(FC | FO)) | (la & (FC | FO));
            }
            K_MOVBE_RM => {
                let a = self.addr_into(op, XA);
                self.load(k, T0, a, wb, Self::chk_of(op));
                match wb {
                    16 => self.a.rev16_w(T0, T0),
                    32 => self.a.rev(false, T0, T0),
                    _ => self.a.rev(true, T0, T0),
                }
                self.wr_gpr(op.r0, w, T0);
            }
            K_MOVBE_MR => {
                let a = self.addr_into(op, XA);
                let src = g(op.r1);
                match wb {
                    16 => self.a.rev16_w(T0, src),
                    32 => self.a.rev(false, T0, src),
                    _ => self.a.rev(true, T0, src),
                }
                self.store(k, T0, a, wb, Self::chk_of(op));
            }
            K_MOVMSK => {
                let s = (op.r1 as u32 & 15) * 16;
                let sub = op.x & 0x3F;
                let is_256 = (sub & 1) != 0;
                self.xsrc(0, op.r1);
                if is_256 {
                    self.a.ldr(128, 1, XCPU, OFF_YMM_HI + s);
                }
                match sub >> 1 {
                    0 => {
                        // pmovmskb / vpmovmskb
                        self.a.mov_imm(true, T2, 0x8040_2010_0804_0201);
                        self.a.v_dup(3, 2, T2);
                        self.a.v_shr_i(true, 8, 0, 0, 7);
                        self.a.neon_3same(false, 0, 19, 0, 0, 2); // mul v0.16b, v0.16b, v2.16b
                        self.a.v_ext(3, 0, 0, 8);
                        self.a.v_addv_8b(0, 0);
                        self.a.v_addv_8b(3, 3);
                        self.a.fmov_fg(false, T0, 0);
                        self.a.fmov_fg(false, T1, 3);
                        self.a.logic_rr(false, 1, T0, T0, T1, 0, 8);
                        if is_256 {
                            self.a.v_shr_i(true, 8, 1, 1, 7);
                            self.a.neon_3same(false, 0, 19, 1, 1, 2);
                            self.a.v_ext(3, 1, 1, 8);
                            self.a.v_addv_8b(1, 1);
                            self.a.v_addv_8b(3, 3);
                            self.a.fmov_fg(false, T1, 1);
                            self.a.fmov_fg(false, T2, 3);
                            self.a.logic_rr(false, 1, T0, T0, T1, 0, 16);
                            self.a.logic_rr(false, 1, T0, T0, T2, 0, 24);
                        }
                    }
                    1 => {
                        // movmskps / vmovmskps
                        self.a.mov_imm(true, T2, 0x0000_0002_0000_0001);
                        self.a.mov_imm(true, T3, 0x0000_0008_0000_0004);
                        self.a.fmov_gf(true, 2, T2);
                        self.a.v_ins_d(2, 1, T3);
                        self.a.v_shr_i(true, 32, 0, 0, 31);
                        self.a.neon_3same(false, 2, 19, 0, 0, 2); // mul v0.4s, v0.4s, v2.4s
                        self.a.v_addv_4s(0, 0);
                        self.a.fmov_fg(false, T0, 0);
                        if is_256 {
                            self.a.v_shr_i(true, 32, 1, 1, 31);
                            self.a.neon_3same(false, 2, 19, 1, 1, 2);
                            self.a.v_addv_4s(1, 1);
                            self.a.fmov_fg(false, T1, 1);
                            self.a.logic_rr(false, 1, T0, T0, T1, 0, 4);
                        }
                    }
                    _ => {
                        // movmskpd / vmovmskpd
                        self.a.v_shr_i(true, 64, 0, 0, 63);
                        self.a.fmov_fg(true, T0, 0);
                        self.a.v_umov_d(T1, 0, 1);
                        self.a.logic_rr(false, 1, T0, T0, T1, 0, 1);
                        if is_256 {
                            self.a.v_shr_i(true, 64, 1, 1, 63);
                            self.a.fmov_fg(true, T1, 1);
                            self.a.v_umov_d(T2, 1, 1);
                            self.a.logic_rr(false, 1, T0, T0, T1, 0, 2);
                            self.a.logic_rr(false, 1, T0, T0, T2, 0, 3);
                        }
                    }
                }
                self.wr_gpr(op.r0, w, T0);
            }
            K_PINSR => {
                let lg = (op.w >> 4) as u32;
                let ebits = 8u32 << lg;
                let lane = (op.imm & 0xFF) as u32;
                let src = if op.x & 0x20 != 0 {
                    let a = self.addr_into(op, XA);
                    self.load(k, T0, a, ebits, Self::chk_of(op));
                    T0
                } else {
                    g(op.r1)
                };
                self.xsrc(0, (op.imm >> 8) as u8);
                self.a.v_ins_elem(lg, 0, lane, src);
                self.xwrite(op.r0, 0);
                if (op.w & 3) == VM_V128 {
                    self.clear_ymm_hi(op.r0);
                }
            }
            K_PEXTR => {
                let ebits = wb;
                let lg = match ebits {
                    8 => 0,
                    16 => 1,
                    32 => 2,
                    _ => 3,
                };
                let lane = (op.imm & 0xFF) as u32;
                self.xsrc(0, op.r1);
                self.a.v_umov_elem(lg, T0, 0, lane);
                if op.x & 0x20 != 0 {
                    let a = self.addr_into(op, XA);
                    self.store(k, T0, a, ebits, Self::chk_of(op));
                } else {
                    let gw = if ebits == 64 { 64 } else { 32 };
                    self.wr_gpr(op.r0, gw, T0);
                }
            }
            _ => self.ext_op_more(k, op),
        }
    }

    pub(super) fn mul1_div(&mut self, k: usize, op: &FastOp) {
        use fast::*;
        let w = op.w;
        let wb = w as u32;
        let sub = op.x & 0x3F;
        let la = self.live[k + 1];
        let src = if op.kind == K_MUL1_R {
            self.ua(op.r0, wb)
        } else {
            let a = self.addr_into(op, XA);
            self.load(k, T0, a, wb, Self::chk_of(op));
            T0
        };
        if src != T0 {
            self.a.mov_rr(true, T0, src);
        }
        match sub {
            0 | 1 => {
                // 0 = mul (unsigned), 1 = imul (signed 1-op).
                let signed = sub == 1;
                if wb == 64 {
                    self.a.mul(true, T1, g(0), T0);
                    if signed {
                        self.a.smulh(T2, g(0), T0);
                    } else {
                        self.a.umulh(T2, g(0), T0);
                    }
                    self.a.mov_rr(true, g(0), T1);
                    self.a.mov_rr(true, g(2), T2);
                    if la & (FC | FO) != 0 {
                        if signed {
                            self.a.asr_i(true, T3, T1, 63);
                            self.a.logic_rr(true, 2, T3, T2, T3, 0, 0);
                        } else {
                            self.a.mov_rr(true, T3, T2);
                        }
                        self.a.clz(true, T4, T3);
                        self.a.lsr_i(true, T4, T4, 6);
                        self.a.logic_ri(false, 2, T4, T4, enc_logic_imm(1, false).unwrap());
                        if la & FC != 0 {
                            self.a.str(8, T4, XCPU, OFF_CF);
                        }
                        if la & FO != 0 {
                            self.a.str(8, T4, XCPU, OFF_OF);
                        }
                    }
                } else {
                    if signed {
                        self.a.sbfm(true, T0, T0, 0, wb - 1);
                        self.a.sbfm(true, T1, g(0), 0, wb - 1);
                    } else {
                        self.a.ubfm(true, T1, g(0), 0, wb - 1);
                    }
                    self.a.mul(true, T2, T1, T0);
                    if wb == 32 {
                        self.a.mov_rr(false, g(0), T2);
                        self.a.lsr_i(true, g(2), T2, 32);
                    } else if wb == 16 {
                        self.a.bfxil0(g(0), T2, 16);
                        self.a.lsr_i(false, T3, T2, 16);
                        self.a.bfxil0(g(2), T3, 16);
                        self.set_lb(0, 16);
                        self.set_lb(2, 16);
                    } else {
                        self.a.bfxil0(g(0), T2, 16);
                        self.set_lb(0, 16);
                    }
                    if la & (FC | FO) != 0 {
                        if signed {
                            self.a.sbfm(true, T3, T2, 0, wb - 1);
                            self.a.logic_rr(true, 2, T3, T2, T3, 0, 0);
                        } else {
                            self.a.lsr_i(true, T3, T2, wb);
                        }
                        self.a.clz(true, T4, T3);
                        self.a.lsr_i(true, T4, T4, 6);
                        self.a.logic_ri(false, 2, T4, T4, enc_logic_imm(1, false).unwrap());
                        if la & FC != 0 {
                            self.a.str(8, T4, XCPU, OFF_CF);
                        }
                        if la & FO != 0 {
                            self.a.str(8, T4, XCPU, OFF_OF);
                        }
                    }
                }
                self.fs.host &= !(FC | FO);
                self.fs.mem = (self.fs.mem & !(FC | FO)) | (la & (FC | FO));
            }
            _ => {
                // 2 = div (unsigned), 3 = idiv (signed).
                let signed = sub == 3;
                if wb <= 16 {
                    self.flush_lb_reg(0);
                    self.flush_lb_reg(2);
                }
                if wb == 64 {
                    self.flush_flags(self.live[k]);
                    self.fs.host = 0;
                    let skip_zero = self.a.cbz(true, T0);
                    // Fast path when the upper 64-bit half (`rdx`) is 0 (for `div`) or the
                    // sign extension of `rax` (for `idiv`). Otherwise run inline 128/64 division.
                    if signed {
                        self.a.asr_i(true, T4, g(0), 63);
                        self.a.logic_rr(true, 2, T4, g(2), T4, 0, 0);
                    } else {
                        self.a.mov_rr(true, T4, g(2));
                    }
                    let slow_128 = self.a.cbnz(true, T4);
                    if signed {
                        self.a.sdiv(true, T2, g(0), T0);
                    } else {
                        self.a.udiv(true, T2, g(0), T0);
                    }
                    self.a.msub(true, T3, T2, T0, g(0));
                    self.a.mov_rr(true, g(0), T2);
                    self.a.mov_rr(true, g(2), T3);
                    let done = self.a.b();
                    self.a.bind(slow_128);
                    self.a.mov_rr(true, T2, g(0)); // T2 = dividend lo
                    self.a.mov_rr(true, T3, g(2)); // T3 = dividend hi
                    if signed {
                        self.a.logic_rr(true, 2, T5, T3, T0, 0, 0);
                        self.a.lsr_i(true, T5, T5, 63); // T5 = quo_neg
                        self.a.lsr_i(true, T6, T3, 63); // T6 = rem_neg
                        let num_pos = self.a.tbz(T3, 63);
                        self.a.addsub_rr(true, true, true, T2, ZR, T2, 0);
                        self.a.sbcs(true, false, T3, ZR, T3);
                        self.a.bind(num_pos);
                        let div_pos = self.a.tbz(T0, 63);
                        self.a.addsub_rr(true, true, false, T0, ZR, T0, 0);
                        self.a.bind(div_pos);
                    }
                    self.a.udiv(true, T4, T3, T0);
                    self.a.msub(true, T3, T4, T0, T3);
                    self.a.movz(false, T4, 64, 0);
                    let div_loop = self.a.pos;
                    self.a.lsr_i(true, XS, T3, 63);
                    self.a.lsl_i(true, T3, T3, 1);
                    self.a.lsr_i(true, T1, T2, 63);
                    self.a.logic_rr(true, 1, T3, T3, T1, 0, 0);
                    self.a.lsl_i(true, T2, T2, 1);
                    let carry_set = self.a.cbnz(true, XS);
                    self.a.addsub_rr(true, true, true, ZR, T3, T0, 0);
                    let no_sub = self.a.bcond(LO);
                    self.a.bind(carry_set);
                    self.a.addsub_rr(true, true, false, T3, T3, T0, 0);
                    self.a.logic_ri(true, 1, T2, T2, enc_logic_imm(1, true).unwrap());
                    self.a.bind(no_sub);
                    self.a.addsub_ri(false, true, false, T4, T4, 1, false);
                    let back = self.a.cbnz(false, T4);
                    self.a.patch(back, div_loop);
                    if signed {
                        let q_ok = self.a.cbz(false, T5);
                        self.a.addsub_rr(true, true, false, T2, ZR, T2, 0);
                        self.a.bind(q_ok);
                        let r_ok = self.a.cbz(false, T6);
                        self.a.addsub_rr(true, true, false, T3, ZR, T3, 0);
                        self.a.bind(r_ok);
                    }
                    self.a.mov_rr(true, g(0), T2);
                    self.a.mov_rr(true, g(2), T3);
                    self.a.bind(skip_zero);
                    self.a.bind(done);
                } else {
                    if signed {
                        self.a.sbfm(true, T0, T0, 0, wb - 1);
                    }
                    let skip_zero = self.a.cbz(true, T0);
                    match wb {
                        32 => {
                            self.a.mov_rr(false, T1, g(0));
                            self.a.lsl_i(true, T2, g(2), 32);
                            self.a.logic_rr(true, 1, T1, T1, T2, 0, 0);
                        }
                        16 => {
                            self.a.ubfm(true, T1, g(0), 0, 15);
                            self.a.ubfm(true, T2, g(2), 0, 15);
                            self.a.lsl_i(true, T2, T2, 16);
                            self.a.logic_rr(true, 1, T1, T1, T2, 0, 0);
                            if signed {
                                self.a.sbfm(true, T1, T1, 0, 31);
                            }
                        }
                        _ => {
                            if signed {
                                self.a.sbfm(true, T1, g(0), 0, 15);
                            } else {
                                self.a.ubfm(true, T1, g(0), 0, 15);
                            }
                        }
                    }
                    if signed {
                        self.a.sdiv(true, T2, T1, T0);
                    } else {
                        self.a.udiv(true, T2, T1, T0);
                    }
                    self.a.msub(true, T3, T2, T0, T1);
                    match wb {
                        32 => {
                            self.a.mov_rr(false, g(0), T2);
                            self.a.mov_rr(false, g(2), T3);
                        }
                        16 => {
                            self.a.bfxil0(g(0), T2, 16);
                            self.a.bfxil0(g(2), T3, 16);
                            self.lb_store_now(0, 16);
                            self.lb_store_now(2, 16);
                        }
                        _ => {
                            self.a.ubfm(false, T2, T2, 0, 7);
                            self.a.ubfm(false, T3, T3, 0, 7);
                            self.a.lsl_i(false, T3, T3, 8);
                            self.a.logic_rr(false, 1, T2, T2, T3, 0, 0);
                            self.a.bfxil0(g(0), T2, 16);
                            self.lb_store_now(0, 64);
                        }
                    }
                    self.a.bind(skip_zero);
                }
            }
        }
    }
}

impl Asm {
    /// Emit a variable shift (`VS_PSRLW..=VS_PSLLQ`) on `v0` by the 64-bit count in `v1.d[0]`
    /// without touching host NZCV.
    pub(super) fn flush_flags_and_shift_vec(&mut self, size: u32, ebits: u64, left: bool, arith: bool) {
        self.fmov_fg(true, T0, 1); // T0 = count (u64)
        if arith {
            // Clamp unsigned count to `ebits - 1`: if `T0 >= ebits`, use `ebits - 1`.
            // Flag-free check `T0 >= ebits`: since `ebits` is 16 or 32, `T0 >> log2(ebits) != 0`.
            let sh = ebits.trailing_zeros();
            self.lsr_i(true, T1, T0, sh);
            let in_range = self.cbz(true, T1);
            self.movz(true, T0, (ebits - 1) as u32, 0);
            self.bind(in_range);
            // Negate shift count for right shift (`sshl` with negative count shifts right).
            self.addsub_rr(false, true, false, T0, ZR, T0, 0);
            self.v_dup(size, 2, T0);
            self.neon_3same(false, size, 8, 0, 0, 2); // sshl
        } else {
            // Logical left/right shift: if `T0 >= ebits`, result is 0!
            let sh = ebits.trailing_zeros();
            self.lsr_i(true, T1, T0, sh);
            let out_of_range = self.cbnz(true, T1);
            if !left {
                self.addsub_rr(false, true, false, T0, ZR, T0, 0);
            }
            self.v_dup(size, 2, T0);
            self.neon_3same(true, size, 8, 0, 0, 2); // ushl
            let done = self.b();
            self.bind(out_of_range);
            self.v_zero(0);
            self.bind(done);
        }
    }
}
