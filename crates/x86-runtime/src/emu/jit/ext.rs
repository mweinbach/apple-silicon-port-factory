//! Native arm64/NEON translation for `K_VBROADCAST..=K_YMM_LANE` (`101..=114`)
//! and dispatch into `super::more` for `K_VAES_RR..=K_VSHA_RM`.

use super::emit::{enc_logic_imm, sf, Asm};
use super::translate::{FlagState, Src, Tr};
use super::xmm::xh;
use super::*;
use crate::emu::fast::{self, FastOp};

impl Asm {
    pub(super) fn v_xtl(&mut self, u: bool, sz: u32, q: bool, vd: u32, vn: u32) {
        self.emit(
            0x0F00_A400
                | (q as u32) << 30
                | (u as u32) << 29
                | (1u32 << (sz + 3)) << 16
                | vn << 5
                | vd,
        );
    }
    pub(super) fn v_fcvtl(&mut self, q: bool, vd: u32, vn: u32) {
        self.emit(0x0E61_7800 | (q as u32) << 30 | vn << 5 | vd);
    }
    pub(super) fn v_fcvtn(&mut self, q: bool, vd: u32, vn: u32) {
        self.emit(0x0E61_6800 | (q as u32) << 30 | vn << 5 | vd);
    }
    pub(super) fn v_scvtf_2d(&mut self, vd: u32, vn: u32) {
        self.emit(0x4E61_D800 | vn << 5 | vd);
    }
    pub(super) fn v_fcvtzs_2d(&mut self, vd: u32, vn: u32) {
        self.emit(0x4EE1_B800 | vn << 5 | vd);
    }
    pub(super) fn v_xtn_2s(&mut self, q: bool, vd: u32, vn: u32) {
        self.emit(0x0EA1_2800 | (q as u32) << 30 | vn << 5 | vd);
    }
    pub(super) fn s_fneg(&mut self, is_f64: bool, vd: u32, vn: u32) {
        self.emit(0x1E21_4000 | (is_f64 as u32) << 22 | vn << 5 | vd);
    }
    pub(super) fn s_fmadd(&mut self, is_f64: bool, vd: u32, vn: u32, vm: u32, va: u32) {
        self.emit(0x1F00_0000 | (is_f64 as u32) << 22 | vm << 16 | va << 10 | vn << 5 | vd);
    }
    pub(super) fn v_fneg(&mut self, is_f64: bool, vd: u32, vn: u32) {
        self.emit(0x6EA0_F800 | (is_f64 as u32) << 22 | vn << 5 | vd);
    }
    pub(super) fn v_fmla(&mut self, is_f64: bool, vd: u32, vn: u32, vm: u32) {
        self.emit(0x4E20_CC00 | (is_f64 as u32) << 22 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn bics_rr(&mut self, x: bool, s: bool, d: u32, n: u32, m: u32) {
        let opc = if s { 3u32 } else { 0 };
        self.emit(0x0A20_0000 | sf(x) | opc << 29 | m << 16 | n << 5 | d);
    }
    pub(super) fn crc32c(&mut self, sw: u32, d: u32, n: u32, m: u32) {
        let (x, sz) = match sw {
            8 => (false, 0),
            16 => (false, 1),
            32 => (false, 2),
            _ => (true, 3),
        };
        self.emit(0x1AC0_5000 | sf(x) | m << 16 | sz << 10 | n << 5 | d);
    }
}

impl Tr<'_> {
    pub(super) fn emit_cvtpd2dq_lane(&mut self, trunc: bool, vd: u32, vn: u32) {
        // Never alias `src == vd`: `v_fcvtzs_2d(vd, src)` overwrites `vd` with the i64
        // result BEFORE the range checks (`v_fcmgt` / `v_fcmge`) inspect `src`. When
        // `trunc && vd == vn`, copy `vn` to scratch `v5` first so negative inputs (whose
        // i64 bits have top 12 bits = 0xFFF -> f64 -NaN) and out-of-range positives aren't
        // misclassified against their own converted integer bits.
        let src = if trunc {
            if vd == vn {
                self.a.v_mov(5, vn);
                5
            } else {
                vn
            }
        } else {
            self.a.v_frint(true, 4, 5, vn);
            5
        };
        self.a.v_fcvtzs_2d(vd, src);
        self.a.mov_imm(true, T0, 0x41E0_0000_0000_0000); // 2147483648.0f64
        self.a.v_dup(3, 2, T0);
        self.a.v_fcmgt(true, 3, 2, src);
        self.a.mov_imm(true, T0, 0xC1E0_0000_0000_0000); // -2147483648.0f64
        self.a.v_dup(3, 4, T0);
        self.a.v_fcmge(true, 4, src, 4);
        self.a.v_and(3, 3, 4);
        self.a.mov_imm(true, T0, 0xFFFF_FFFF_8000_0000);
        self.a.v_dup(3, 4, T0);
        self.a.v_bsl(3, vd, 4);
        self.a.v_mov(vd, 3);
    }

    /// Emit one packed 128-bit FMA lane: `vd = ±(vm1 * vm2) ± vacc`.
    /// Preserves exact `(vn = vm1, vm = vm2)` operand order for dual-NaN payload parity.
    /// Scratch temps `v2`, `v3`, `v5` must not alias `vm1`, `vm2`, or `vacc`.
    pub(super) fn emit_vfma_packed_lane(
        &mut self,
        is_pd: bool,
        fop: u8,
        vd: u32,
        vm1: u32,
        vm2: u32,
        vacc: u32,
        acc_scratch: bool,
    ) {
        let neg_mul = fop == 2 || fop == 3;
        let m1_reg = if neg_mul {
            self.a.v_fneg(is_pd, 3, vm1);
            3
        } else {
            vm1
        };
        let acc_reg = if vd != m1_reg && vd != vm2 {
            self.a.v_mov(vd, vacc);
            vd
        } else if fop == 0 && vd == vacc {
            vd
        } else if acc_scratch && vacc != m1_reg && vacc != vm2 {
            vacc
        } else {
            self.a.v_mov(5, vacc);
            5
        };
        match fop {
            1 | 3 => self.a.v_fneg(is_pd, acc_reg, acc_reg),
            4 => {
                // fmaddsub: negate even lanes of acc
                if is_pd {
                    self.a.mov_imm(true, T0, 0x8000_0000_0000_0000);
                    self.a.fmov_gf(true, 2, T0);
                } else {
                    self.a.mov_imm(true, T0, 0x0000_0000_8000_0000);
                    self.a.v_dup(3, 2, T0);
                }
                self.a.v_eor(acc_reg, acc_reg, 2);
            }
            5 => {
                // fmsubadd: negate odd lanes of acc
                if is_pd {
                    self.a.mov_imm(true, T0, 0x8000_0000_0000_0000);
                    self.a.v_zero(2);
                    self.a.v_ins_d(2, 1, T0);
                } else {
                    self.a.mov_imm(true, T0, 0x8000_0000_0000_0000);
                    self.a.v_dup(3, 2, T0);
                }
                self.a.v_eor(acc_reg, acc_reg, 2);
            }
            _ => {}
        }
        self.a.v_fmla(is_pd, acc_reg, m1_reg, vm2);
        if acc_reg != vd {
            self.a.v_mov(vd, acc_reg);
        }
    }

    /// Direct pinned-register translation for `K_VFMA_RR` / `K_VFMA_RM` (scalar ss/sd and
    /// 128/256-bit packed ps/pd), eliminating `v0..v5` copies and scalar FP↔GPR roundtrips.
    pub(super) fn emit_vfma_op(&mut self, k: usize, op: &FastOp) {
        use fast::*;
        let d = op.r0 & 15;
        let s2 = (op.imm & 0xFF) as u8 & 15;
        let fma_code = ((op.imm >> 8) & 0xFF) as u8;
        let ty = fma_code & 3;
        let ord = (fma_code >> 2) & 3;
        let fop = (fma_code >> 4) & 7;
        let is_256 = (op.w & 3) == VM_V256;
        let is_mem = op.kind == K_VFMA_RM;

        if ty <= 1 {
            let is_sd = ty == 1;
            let bits = if is_sd { 64 } else { 32 };
            let v3_lo = if is_mem {
                if !self.scalar_fp_mem_op(k, true, 1, op, bits) {
                    self.load_op(k, T0, op, bits);
                    self.a.fmov_gf(is_sd, 1, T0);
                }
                1
            } else {
                self.xget(op.r1)
            };
            let v1_lo = self.xget(d);
            let v2_lo = self.xget(s2);
            let (vm1, vm2, vacc) = match ord {
                0 => (v1_lo, v3_lo, v2_lo),
                1 => (v2_lo, v1_lo, v3_lo),
                _ => (v2_lo, v3_lo, v1_lo),
            };
            let neg_mul = fop == 2 || fop == 3;
            let m1_reg = if neg_mul {
                self.a.s_fneg(is_sd, 3, vm1);
                3
            } else {
                vm1
            };
            let acc_reg = if fop == 1 || fop == 3 {
                self.a.s_fneg(is_sd, 5, vacc);
                5
            } else {
                vacc
            };
            let hi_zero = if is_sd {
                self.xmm_hi64_zero & (1u16 << d) != 0
            } else {
                self.xmm_hi32_zero & (1u16 << d) != 0
            };
            let hi_dead = if is_sd {
                (self.xmm_hi64_live[k + 1] & (1u16 << d)) == 0
            } else {
                (self.xmm_hi32_live[k + 1] & (1u16 << d)) == 0
            };
            if hi_zero || hi_dead {
                self.a.s_fmadd(is_sd, xh(d), m1_reg, vm2, acc_reg);
                self.xdef(d);
                self.x_mark_hi(d, !is_sd, true);
            } else {
                self.a.s_fmadd(is_sd, 2, m1_reg, vm2, acc_reg);
                self.a.v_ins_lane(if is_sd { 3 } else { 2 }, xh(d), 0, 2, 0);
                self.xdef(d);
            }
            self.clear_ymm_hi(d);
            return;
        }

        let is_pd = ty == 3;
        if is_mem {
            if is_256 {
                self.vec256_mem_op(k, op, true, 1, 7);
            } else {
                self.vec128_mem_op(k, op, true, 1);
            }
        }
        let v1_lo = self.xget(d);
        let v2_lo = self.xget(s2);
        let v3_lo = if is_mem { 1 } else { self.xget(op.r1) };
        let (vm1_lo, vm2_lo, vacc_lo) = match ord {
            0 => (v1_lo, v3_lo, v2_lo),
            1 => (v2_lo, v1_lo, v3_lo),
            _ => (v2_lo, v3_lo, v1_lo),
        };
        let vd_lo = xh(d);
        self.emit_vfma_packed_lane(is_pd, fop, vd_lo, vm1_lo, vm2_lo, vacc_lo, vacc_lo < 8);
        self.xdef(d);

        if is_256 {
            let v1_hi = self.yget(d, 0);
            let v2_hi = if s2 == d { v1_hi } else { self.yget(s2, 4) };
            let v3_hi = if is_mem {
                7
            } else if (op.r1 & 15) == d {
                v1_hi
            } else if (op.r1 & 15) == s2 {
                v2_hi
            } else {
                self.yget(op.r1, 7)
            };
            let (vm1_hi, vm2_hi, vacc_hi) = match ord {
                0 => (v1_hi, v3_hi, v2_hi),
                1 => (v2_hi, v1_hi, v3_hi),
                _ => (v2_hi, v3_hi, v1_hi),
            };
            let yh_dst = self.yh_reg(d);
            let vd_hi = yh_dst.unwrap_or(6);
            self.emit_vfma_packed_lane(is_pd, fop, vd_hi, vm1_hi, vm2_hi, vacc_hi, vacc_hi < 8);
            if yh_dst.is_some() {
                let bit = 1u16 << d;
                self.ymm_clean &= !bit;
                self.ymm_dirty_pending |= bit;
            } else {
                self.write_ymm_hi(d, vd_hi);
            }
        } else {
            self.clear_ymm_hi(d);
        }
    }

    pub(super) fn ext_op_more(&mut self, k: usize, op: &FastOp) {
        use fast::*;
        let w = op.w;
        let wb = w as u32;
        let la = self.live[k + 1];
        match op.kind {
            K_VBROADCAST => {
                let mode = op.w & 3;
                let lg = (op.w >> 4) as u32;
                let mem = (op.x & 0x20) != 0;
                if lg == 4 {
                    if mem {
                        self.vec128_mem_op(k, op, true, 0);
                    } else {
                        self.xsrc(0, op.r1);
                    }
                    self.xwrite(op.r0, 0);
                    self.write_ymm_hi(op.r0, 0);
                } else {
                    let ebits = 8u32 << lg;
                    if mem {
                        let a = self.addr_into(op, XA);
                        self.load(k, T0, a, ebits, Self::chk_of(op));
                    } else if self.xpin || self.xv & (1u16 << (op.r1 & 15)) != 0 {
                        self.a.fmov_fg(lg == 3, T0, xh(op.r1));
                    } else {
                        let s = (op.r1 as u32 & 15) * 16;
                        self.a.ldr(ebits, T0, XCPU, OFF_XMM + s);
                    }
                    self.a.v_dup(lg, 0, T0);
                    self.xwrite(op.r0, 0);
                    if mode == VM_V256 {
                        self.write_ymm_hi(op.r0, 0);
                    } else {
                        self.clear_ymm_hi(op.r0);
                    }
                }
            }
            K_PMOVX => {
                let mode = op.w & 3;
                let sub = op.w >> 2;
                let u = (sub & 1) != 0;
                let mem = (op.x & 0x20) != 0;
                if mem {
                    let mbits = op.imm as u32;
                    if mbits == 128 {
                        self.vec128_mem_op(k, op, true, 0);
                    } else {
                        let a = self.addr_into(op, XA);
                        self.load(k, T0, a, mbits, Self::chk_of(op));
                        self.a.fmov_gf(true, 0, T0);
                    }
                } else {
                    self.xsrc(0, op.r1);
                }
                match sub >> 1 {
                    0 => {
                        // bw (8 -> 16)
                        if mode == VM_V256 {
                            self.a.v_xtl(u, 0, true, 1, 0);
                        }
                        self.a.v_xtl(u, 0, false, 0, 0);
                    }
                    1 => {
                        // bd (8 -> 16 -> 32)
                        self.a.v_xtl(u, 0, false, 0, 0);
                        if mode == VM_V256 {
                            self.a.v_xtl(u, 1, true, 1, 0);
                        }
                        self.a.v_xtl(u, 1, false, 0, 0);
                    }
                    2 => {
                        // bq (8 -> 16 -> 32 -> 64)
                        self.a.v_xtl(u, 0, false, 0, 0);
                        self.a.v_xtl(u, 1, false, 0, 0);
                        if mode == VM_V256 {
                            self.a.v_xtl(u, 2, true, 1, 0);
                        }
                        self.a.v_xtl(u, 2, false, 0, 0);
                    }
                    3 => {
                        // wd (16 -> 32)
                        if mode == VM_V256 {
                            self.a.v_xtl(u, 1, true, 1, 0);
                        }
                        self.a.v_xtl(u, 1, false, 0, 0);
                    }
                    4 => {
                        // wq (16 -> 32 -> 64)
                        self.a.v_xtl(u, 1, false, 0, 0);
                        if mode == VM_V256 {
                            self.a.v_xtl(u, 2, true, 1, 0);
                        }
                        self.a.v_xtl(u, 2, false, 0, 0);
                    }
                    _ => {
                        // dq (32 -> 64)
                        if mode == VM_V256 {
                            self.a.v_xtl(u, 2, true, 1, 0);
                        }
                        self.a.v_xtl(u, 2, false, 0, 0);
                    }
                }
                self.xwrite(op.r0, 0);
                if mode == VM_V256 {
                    self.write_ymm_hi(op.r0, 1);
                } else if mode == VM_V128 {
                    self.clear_ymm_hi(op.r0);
                }
            }
            K_VCVT_PD => {
                let mode = op.w & 3;
                let src_256 = (op.w & 4) != 0;
                let sub = op.x & 0x1F;
                let mem = (op.x & 0x20) != 0;
                if sub <= 1 {
                    if mem {
                        if mode == VM_V256 {
                            self.vec128_mem_op(k, op, true, 0);
                        } else {
                            let a = self.addr_into(op, XA);
                            self.load(k, T0, a, 64, Self::chk_of(op));
                            self.a.fmov_gf(true, 0, T0);
                        }
                    } else {
                        self.xsrc(0, op.r1);
                    }
                    if sub == 0 {
                        if mode == VM_V256 {
                            self.a.v_fcvtl(true, 1, 0);
                        }
                        self.a.v_fcvtl(false, 0, 0);
                    } else {
                        if mode == VM_V256 {
                            self.a.v_xtl(false, 2, true, 1, 0);
                            self.a.v_scvtf_2d(1, 1);
                        }
                        self.a.v_xtl(false, 2, false, 0, 0);
                        self.a.v_scvtf_2d(0, 0);
                    }
                    self.xwrite(op.r0, 0);
                    if mode == VM_V256 {
                        self.write_ymm_hi(op.r0, 1);
                    } else if mode == VM_V128 {
                        self.clear_ymm_hi(op.r0);
                    }
                } else {
                    if mem {
                        if src_256 {
                            self.vec256_mem_op(k, op, true, 0, 1);
                        } else {
                            self.vec128_mem_op(k, op, true, 0);
                        }
                    } else {
                        self.xsrc(0, op.r1);
                        if src_256 {
                            self.ysrc(1, op.r1);
                        }
                    }
                    if sub == 2 {
                        self.a.v_fcvtn(false, 0, 0);
                        if src_256 {
                            self.a.v_fcvtn(true, 0, 1);
                        }
                    } else {
                        let trunc = sub == 3;
                        self.emit_cvtpd2dq_lane(trunc, 0, 0);
                        self.a.v_xtn_2s(false, 0, 0);
                        if src_256 {
                            self.emit_cvtpd2dq_lane(trunc, 1, 1);
                            self.a.v_xtn_2s(true, 0, 1);
                        }
                    }
                    self.xwrite(op.r0, 0);
                    if mode != VM_LEG {
                        self.clear_ymm_hi(op.r0);
                    }
                }
            }
            K_VFMA_RR | K_VFMA_RM => self.emit_vfma_op(k, op),
            K_VBLENDV_RR | K_VBLENDV_RM => {
                let m_reg = (op.imm & 0xFF) as u8;
                let s1_reg = ((op.imm >> 8) & 0xFF) as u8;
                let lg = (op.x & 0x3F) as u32;
                let is_256 = (op.w & 3) == VM_V256;
                if op.kind == K_VBLENDV_RM {
                    if is_256 {
                        self.vec256_mem_op(k, op, true, 1, 7);
                    } else {
                        self.vec128_mem_op(k, op, true, 1);
                    }
                } else {
                    self.xsrc(1, op.r1);
                    if is_256 {
                        self.ysrc(7, op.r1);
                    }
                }
                self.xsrc(0, s1_reg);
                self.xsrc(2, m_reg);
                self.a.v_cmlt0(lg, 2, 2);
                self.a.v_bsl(2, 1, 0);
                self.a.v_mov(0, 2);
                if is_256 {
                    self.a.v_mov(6, 0);
                    self.ysrc(0, s1_reg);
                    self.ysrc(2, m_reg);
                    self.a.v_cmlt0(lg, 2, 2);
                    self.a.v_bsl(2, 7, 0);
                    self.a.v_mov(0, 2);
                }
                self.vstore_dst(op);
            }
            K_VTEST_RR | K_VTEST_RM => {
                let is_256 = (op.w & 3) == VM_V256;
                let sub = op.x & 0x3F;
                if op.kind == K_VTEST_RM {
                    if is_256 {
                        self.vec256_mem_op(k, op, true, 1, 7);
                    } else {
                        self.vec128_mem_op(k, op, true, 1);
                    }
                } else if la & FALL != 0 {
                    self.xsrc(1, op.r1);
                    if is_256 {
                        self.ysrc(7, op.r1);
                    }
                }
                if la & FALL != 0 {
                    self.xsrc(0, op.r0);
                    if sub == 1 {
                        self.a.v_shr_i(true, 32, 0, 0, 31);
                        self.a.v_shr_i(true, 32, 1, 1, 31);
                    } else if sub == 2 {
                        self.a.v_shr_i(true, 64, 0, 0, 63);
                        self.a.v_shr_i(true, 64, 1, 1, 63);
                    }
                    self.a.v_and(2, 0, 1);
                    self.a.v_bic(3, 1, 0);
                    if is_256 {
                        self.ysrc(6, op.r0);
                        if sub == 1 {
                            self.a.v_shr_i(true, 32, 6, 6, 31);
                            self.a.v_shr_i(true, 32, 7, 7, 31);
                        } else if sub == 2 {
                            self.a.v_shr_i(true, 64, 6, 6, 63);
                            self.a.v_shr_i(true, 64, 7, 7, 63);
                        }
                        self.a.v_and(4, 6, 7);
                        self.a.v_bic(5, 7, 6);
                        self.a.v_orr(2, 2, 4);
                        self.a.v_orr(3, 3, 5);
                    }
                    if la & FZ != 0 {
                        self.a.fmov_fg(true, T0, 2);
                        self.a.v_umov_d(T1, 2, 1);
                        self.a.logic_rr(true, 1, T0, T0, T1, 0, 0);
                        self.a.clz(true, T0, T0);
                        self.a.lsr_i(true, T0, T0, 6);
                        self.a.str(8, T0, XCPU, OFF_ZF);
                    }
                    if la & FC != 0 {
                        self.a.fmov_fg(true, T0, 3);
                        self.a.v_umov_d(T1, 3, 1);
                        self.a.logic_rr(true, 1, T0, T0, T1, 0, 0);
                        self.a.clz(true, T0, T0);
                        self.a.lsr_i(true, T0, T0, 6);
                        self.a.str(8, T0, XCPU, OFF_CF);
                    }
                    if la & FS != 0 {
                        self.a.str(8, ZR, XCPU, OFF_SF);
                    }
                    if la & FO != 0 {
                        self.a.str(8, ZR, XCPU, OFF_OF);
                    }
                    if la & FP != 0 {
                        self.a.movz(false, T0, 1, 0);
                        self.a.str(8, T0, XCPU, OFF_PFV);
                    }
                }
                self.fs = FlagState {
                    src: Src::None,
                    host: 0,
                    mem: (self.fs.mem & !FALL) | (la & FALL),
                };
            }
            K_BMI_RR | K_BMI_RM => {
                let sub = op.x & 0x3F;
                let mem = op.kind == K_BMI_RM;
                let x = wb == 64;
                match sub {
                    0 => {
                        // Andn: d = !s1 & s2
                        let s1 = g(op.imm as u8);
                        let s2 = if mem {
                            let a = self.addr_into(op, XA);
                            self.load(k, T0, a, wb, Self::chk_of(op));
                            T0
                        } else {
                            g(op.r1)
                        };
                        let need_nzcv = la & FNZCV != 0;
                        let need_pf = la & FP != 0;
                        self.a.bics_rr(x, need_nzcv, g(op.r0), s2, s1);
                        self.set_lb(op.r0, w);
                        if need_pf {
                            self.a.str(8, g(op.r0), XCPU, OFF_PFV);
                        }
                        self.fs.mem &= !FALL;
                        if need_pf {
                            self.fs.mem |= FP;
                        }
                        self.fs.host = if need_nzcv { la & FNZCV } else { 0 };
                        self.fs.src = Src::Logic;
                    }
                    1..=3 => {
                        // Shlx, Shrx, Sarx
                        let a_src = if mem {
                            let a = self.addr_into(op, XA);
                            self.load(k, T0, a, wb, Self::chk_of(op));
                            T0
                        } else {
                            g(op.imm as u8)
                        };
                        let opc = match sub {
                            1 => 8,  // lslv
                            2 => 9,  // lsrv
                            _ => 10, // asrv
                        };
                        self.a.shiftv(x, opc, g(op.r0), a_src, g(op.r1));
                        self.set_lb(op.r0, w);
                    }
                    4 => {
                        // Rorx
                        let a_src = if mem {
                            let a = self.addr_into(op, XA);
                            self.load(k, T0, a, wb, Self::chk_of(op));
                            T0
                        } else {
                            g(op.r1)
                        };
                        self.a.ror_i(x, g(op.r0), a_src, op.imm as u32);
                        self.set_lb(op.r0, w);
                    }
                    5..=7 => {
                        // Blsr (a & (a-1)), Blsmsk (a ^ (a-1)), Blsi (a & -a)
                        let a_src = if mem {
                            let a = self.addr_into(op, XA);
                            self.load(k, T0, a, wb, Self::chk_of(op));
                            T0
                        } else {
                            g(op.r1)
                        };
                        if a_src != T0 {
                            self.a.mov_rr(x, T0, a_src);
                        }
                        match sub {
                            5 => {
                                self.a.addsub_ri(x, true, false, T1, T0, 1, false);
                                self.a.logic_rr(x, 0, T2, T0, T1, 0, 0);
                            }
                            6 => {
                                self.a.addsub_ri(x, true, false, T1, T0, 1, false);
                                self.a.logic_rr(x, 2, T2, T0, T1, 0, 0);
                            }
                            _ => {
                                self.a.addsub_rr(x, true, false, T1, ZR, T0, 0);
                                self.a.logic_rr(x, 0, T2, T0, T1, 0, 0);
                            }
                        }
                        self.wr_gpr(op.r0, w, T2);
                        if la & FC != 0 {
                            self.a.clz(true, T3, T0);
                            self.a.lsr_i(true, T3, T3, 6);
                            if sub == 7 {
                                self.a.logic_ri(false, 2, T3, T3, enc_logic_imm(1, false).unwrap());
                            }
                            self.a.str(8, T3, XCPU, OFF_CF);
                        }
                        if la & FZ != 0 {
                            self.a.clz(true, T3, T2);
                            self.a.lsr_i(true, T3, T3, 6);
                            self.a.str(8, T3, XCPU, OFF_ZF);
                        }
                        if la & FS != 0 {
                            self.a.ubfx(T3, T2, wb - 1, 1);
                            self.a.str(8, T3, XCPU, OFF_SF);
                        }
                        if la & FO != 0 {
                            self.a.str(8, ZR, XCPU, OFF_OF);
                        }
                        self.fs.host &= !(FC | FZ | FS | FO);
                        self.fs.mem = (self.fs.mem & !(FC | FZ | FS | FO)) | (la & (FC | FZ | FS | FO));
                    }
                    8 => {
                        // Bzhi
                        let a_src = if mem {
                            let a = self.addr_into(op, XA);
                            self.load(k, T0, a, wb, Self::chk_of(op));
                            T0
                        } else {
                            g(op.imm as u8)
                        };
                        if a_src != T0 {
                            self.a.mov_rr(x, T0, a_src);
                        }
                        self.a.ubfm(false, T1, g(op.r1), 0, 7); // idx = r1 & 0xFF
                        let sh = wb.trailing_zeros();
                        self.a.lsr_i(false, T3, T1, sh); // T3 != 0 iff idx >= wb
                        let ge_wb = self.a.cbnz(false, T3);
                        self.a.mov_imm(true, T4, u64::MAX);
                        self.a.shiftv(true, 8, T4, T4, T1);
                        self.a.bic_rr(x, T2, T0, T4);
                        if la & FC != 0 {
                            self.a.str(8, ZR, XCPU, OFF_CF);
                        }
                        let done = self.a.b();
                        self.a.bind(ge_wb);
                        self.a.mov_rr(x, T2, T0);
                        if la & FC != 0 {
                            self.a.movz(false, T4, 1, 0);
                            self.a.str(8, T4, XCPU, OFF_CF);
                        }
                        self.a.bind(done);
                        self.wr_gpr(op.r0, w, T2);
                        if la & FZ != 0 {
                            self.a.clz(true, T3, T2);
                            self.a.lsr_i(true, T3, T3, 6);
                            self.a.str(8, T3, XCPU, OFF_ZF);
                        }
                        if la & FS != 0 {
                            self.a.ubfx(T3, T2, wb - 1, 1);
                            self.a.str(8, T3, XCPU, OFF_SF);
                        }
                        // OF is cleared (SDM; checked under Rosetta 2).
                        if la & FO != 0 {
                            self.a.str(8, ZR, XCPU, OFF_OF);
                        }
                        self.fs.host &= !(FC | FZ | FS | FO);
                        self.fs.mem = (self.fs.mem & !(FC | FZ | FS | FO)) | (la & (FC | FZ | FS | FO));
                    }
                    9 => {
                        // Bextr
                        let a_src = if mem {
                            let a = self.addr_into(op, XA);
                            self.load(k, T0, a, wb, Self::chk_of(op));
                            T0
                        } else {
                            g(op.imm as u8)
                        };
                        if a_src != T0 {
                            self.a.mov_rr(x, T0, a_src);
                        }
                        let ctl = g(op.r1);
                        self.a.ubfm(false, T1, ctl, 0, 7); // start
                        self.a.ubfx(T2, ctl, 8, 8); // len
                        self.a.lsr_i(false, T3, T1, wb.trailing_zeros());
                        let start_oob = self.a.cbnz(false, T3);
                        self.a.shiftv(x, 9, T4, T0, T1);
                        let after_shr = self.a.b();
                        self.a.bind(start_oob);
                        self.a.movz(true, T4, 0, 0);
                        self.a.bind(after_shr);
                        self.a.lsr_i(false, T3, T2, 6);
                        let len_ge64 = self.a.cbnz(false, T3);
                        self.a.mov_imm(true, T5, u64::MAX);
                        self.a.shiftv(true, 8, T5, T5, T2);
                        self.a.bic_rr(x, T4, T4, T5);
                        self.a.bind(len_ge64);
                        self.wr_gpr(op.r0, w, T4);
                        if la & FZ != 0 {
                            self.a.clz(true, T3, T4);
                            self.a.lsr_i(true, T3, T3, 6);
                            self.a.str(8, T3, XCPU, OFF_ZF);
                        }
                        if la & FS != 0 {
                            self.a.ubfx(T3, T4, wb - 1, 1);
                            self.a.str(8, T3, XCPU, OFF_SF);
                        }
                        if la & FC != 0 {
                            self.a.str(8, ZR, XCPU, OFF_CF);
                        }
                        if la & FO != 0 {
                            self.a.str(8, ZR, XCPU, OFF_OF);
                        }
                        if la & FP != 0 {
                            self.a.str(8, T4, XCPU, OFF_PFV);
                        }
                        self.fs = FlagState {
                            src: Src::None,
                            host: 0,
                            mem: (self.fs.mem & !FALL) | (la & FALL),
                        };
                    }
                    10 => {
                        // Mulx: r0 = hi, r1 = lo, src * rdx
                        let src = if mem {
                            let a = self.addr_into(op, XA);
                            self.load(k, T0, a, wb, Self::chk_of(op));
                            T0
                        } else {
                            g(op.imm as u8)
                        };
                        if wb == 64 {
                            self.a.mul(true, T1, g(2), src);
                            self.a.umulh(T2, g(2), src);
                            self.wr_gpr(op.r1, 64, T1);
                            self.wr_gpr(op.r0, 64, T2);
                        } else {
                            self.a.ubfm(true, T0, src, 0, 31);
                            self.a.ubfm(true, T1, g(2), 0, 31);
                            self.a.mul(true, T2, T1, T0);
                            self.wr_gpr(op.r1, 32, T2);
                            self.a.lsr_i(true, T3, T2, 32);
                            self.wr_gpr(op.r0, 32, T3);
                        }
                    }
                    11 | 12 => {
                        // Adcx (CF) / Adox (OF)
                        let flag_bit = if sub == 11 { FC } else { FO };
                        let flag_off = if sub == 11 { OFF_CF } else { OFF_OF };
                        let other_flags = FALL & !flag_bit;
                        if self.fs.host & other_flags != 0 {
                            self.flush_flags(self.live[k] & other_flags);
                        }
                        self.flag_reg(flag_bit, T6);
                        let b_reg = if mem {
                            let a = self.addr_into(op, XA);
                            self.load(k, T0, a, wb, Self::chk_of(op));
                            T0
                        } else {
                            g(op.r1)
                        };
                        self.a.addsub_ri(false, true, true, ZR, T6, 1, false);
                        self.a.adcs(x, true, g(op.r0), g(op.r0), b_reg);
                        self.set_lb(op.r0, w);
                        if la & flag_bit != 0 {
                            self.a.cset(T6, HS);
                            self.a.str(8, T6, XCPU, flag_off);
                        }
                        self.fs.host = 0;
                        self.fs.src = Src::None;
                        self.fs.mem = (self.fs.mem & !flag_bit) | (la & flag_bit);
                    }
                    _ => {
                        // Crc32
                        let sw = wb;
                        let dw = op.imm as u8;
                        let src = if mem {
                            let a = self.addr_into(op, XA);
                            self.load(k, T0, a, sw, Self::chk_of(op));
                            T0
                        } else if op.r1 & 0x80 != 0 {
                            self.a.ubfx(T0, g(op.r1 & 15), 8, 8);
                            T0
                        } else {
                            self.ua(op.r1, sw)
                        };
                        self.a.crc32c(sw, g(op.r0), g(op.r0), src);
                        self.set_lb(op.r0, dw);
                    }
                }
            }
            K_YMM_LANE => {
                let sub = op.x & 0x3F;
                let mem = op.w != 0;
                match sub {
                    0 => {
                        // Vpermq / Vpermpd
                        if mem {
                            self.vec256_mem_op(k, op, true, 0, 1);
                        } else {
                            self.xsrc(0, op.r1);
                            self.ysrc(1, op.r1);
                        }
                        let imm = op.imm as usize;
                        let build_q = |q0: usize, q1: usize| -> (u64, u64) {
                            let mut idx = [0u8; 16];
                            for b in 0..8 {
                                idx[b] = (q0 * 8 + b) as u8;
                                idx[8 + b] = (q1 * 8 + b) as u8;
                            }
                            (
                                u64::from_le_bytes(idx[0..8].try_into().unwrap()),
                                u64::from_le_bytes(idx[8..16].try_into().unwrap()),
                            )
                        };
                        let (lo0, hi0) = build_q(imm & 3, (imm >> 2) & 3);
                        let (lo1, hi1) = build_q((imm >> 4) & 3, (imm >> 6) & 3);
                        self.a.mov_imm(true, T0, lo0);
                        self.a.mov_imm(true, T1, hi0);
                        self.a.fmov_gf(true, 2, T0);
                        self.a.v_ins_d(2, 1, T1);
                        self.a.v_tbl2(2, 0, 2);
                        self.a.mov_imm(true, T0, lo1);
                        self.a.mov_imm(true, T1, hi1);
                        self.a.fmov_gf(true, 3, T0);
                        self.a.v_ins_d(3, 1, T1);
                        self.a.v_tbl2(3, 0, 3);
                        self.xwrite(op.r0, 2);
                        self.write_ymm_hi(op.r0, 3);
                    }
                    1 => {
                        // Vpermd / Vpermps
                        let idx_reg = (op.imm & 15) as u8;
                        if mem {
                            self.vec256_mem_op(k, op, true, 0, 1);
                        } else {
                            self.xsrc(0, op.r1);
                            self.ysrc(1, op.r1);
                        }
                        self.xsrc(2, idx_reg);
                        self.ysrc(3, idx_reg);
                        self.a.movz(false, T0, 7, 0);
                        self.a.v_dup(2, 4, T0);
                        self.a.v_and(2, 2, 4);
                        self.a.v_and(3, 3, 4);
                        self.a.mov_imm(false, T0, 0x0404_0404);
                        self.a.v_dup(2, 4, T0);
                        self.a.neon_3same(false, 2, 19, 2, 2, 4);
                        self.a.neon_3same(false, 2, 19, 3, 3, 4);
                        self.a.mov_imm(false, T0, 0x0302_0100);
                        self.a.v_dup(2, 4, T0);
                        self.a.neon_3same(false, 2, 16, 2, 2, 4);
                        self.a.neon_3same(false, 2, 16, 3, 3, 4);
                        self.a.v_tbl2(2, 0, 2);
                        self.a.v_tbl2(3, 0, 3);
                        self.xwrite(op.r0, 2);
                        self.write_ymm_hi(op.r0, 3);
                    }
                    2 => {
                        // Vperm2f128 / Vperm2i128
                        let imm = (op.imm & 0xFF) as u8;
                        let s1_reg = ((op.imm >> 8) & 15) as u8;
                        if mem {
                            self.vec256_mem_op(k, op, true, 2, 3);
                        } else {
                            self.xsrc(2, op.r1);
                            self.ysrc(3, op.r1);
                        }
                        self.xsrc(0, s1_reg);
                        self.ysrc(1, s1_reg);
                        for (dst_v, ctrl) in [(4u32, imm & 0xF), (5u32, (imm >> 4) & 0xF)] {
                            if ctrl & 8 != 0 {
                                self.a.v_zero(dst_v);
                            } else {
                                self.a.v_mov(dst_v, (ctrl & 3) as u32);
                            }
                        }
                        self.xwrite(op.r0, 4);
                        self.write_ymm_hi(op.r0, 5);
                    }
                    3 => {
                        // Vinsertf128 / Vinserti128
                        let imm = (op.imm & 1) as u8;
                        let s1_reg = ((op.imm >> 8) & 15) as u8;
                        if mem {
                            self.vec128_mem_op(k, op, true, 2);
                        } else {
                            self.xsrc(2, op.r1);
                        }
                        self.xsrc(0, s1_reg);
                        self.ysrc(1, s1_reg);
                        if imm == 0 {
                            self.xwrite(op.r0, 2);
                            self.write_ymm_hi(op.r0, 1);
                        } else {
                            self.xwrite(op.r0, 0);
                            self.write_ymm_hi(op.r0, 2);
                        }
                    }
                    4 => {
                        // Vextractf128 / Vextracti128
                        let lo_lane = (op.imm & 1) == 0;
                        if mem {
                            let v = if lo_lane {
                                self.xget(op.r1)
                            } else {
                                self.yget(op.r1, 0)
                            };
                            self.vec128_mem_op(k, op, false, v);
                        } else {
                            if lo_lane {
                                self.xsrc(0, op.r1);
                            } else {
                                self.ysrc(0, op.r1);
                            }
                            self.xwrite(op.r0, 0);
                            self.clear_ymm_hi(op.r0);
                        }
                    }
                    _ => {
                        // Vzeroall
                        for i in 0..16u8 {
                            if self.xpin {
                                self.a.v_zero(xh(i));
                            } else {
                                self.a.zero128(XCPU, OFF_XMM + (i as u32) * 16);
                            }
                        }
                        self.emit_ymm_clear_check(0xFFFF);
                        self.ymm_clean = 0xFFFF;
                    }
                }
            }
            K_VAES_RR..=K_MMX_MISC => self.emit_more_op(k, op),
            _ => unreachable!("unsupported op kind {}", op.kind),
        }
    }
}
