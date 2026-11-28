//! Core scalar/integer/branch/terminator translation methods on [`Tr`].

use super::cache::{const_slot_plain, iat_slot_plain, inline_host_fn_kind};
use super::emit::{enc_logic_imm, sf};
use super::stubs::{plaincall_enabled, ras_push};
use super::translate::{stack_tso, Chk, Cond, FlagState, Grp, Src, Term, Tr};
use super::*;
use crate::emu::fast::{self, FastOp};
use crate::emu::mem;

impl Tr<'_> {
    pub(super) fn op(&mut self, k: usize) {
        use fast::*;
        let op = self.ops[k];
        if self.xa_key.is_some()
            && !matches!(
                op.kind,
                K_NOP
                    | K_MOV_RR
                    | K_MOV_RI
                    | K_MOV_RM
                    | K_MOV_MR
                    | K_MOV_MI
                    | K_MOVZX_RR
                    | K_MOVSX_RR
                    | K_MOVZX_RM
                    | K_MOVSX_RM
                    | K_LEA
                    | K_ALU_RR
                    | K_ALU_RI
                    | K_ALU_RM
                    | K_ALU_MR
                    | K_ALU_MI
                    | K_CMPJ_RR
                    | K_CMPJ_RI
                    | K_CMPJ_RM
                    | K_INC_R
                    | K_DEC_R
                    | K_SHIFT_RI
                    | K_SETCC_R
                    | K_CMOV_RR
                    | K_NOT_R
                    | K_NEG_R
                    | K_BT_RR
                    | K_BT_RI
                    | K_IMUL_RR
                    | K_IMUL_RM
                    | K_IMUL_RRI
                    | K_IMUL_RMI
                    | K_VSHUF_RR
                    | K_VSHUF_RM
                    | K_VFP_RR
                    | K_VFP_RM
                    | K_VST
                    | K_COMI_RR
                    | K_COMI_RM
            )
        {
            self.xa_key = None;
        }
        if self.xon && self.xmm_op(k, &op) {
            return;
        }
        let w = op.w;
        match op.kind {
            K_NOP => {}
            K_MOV_RR => {
                if w == 32 || op.r0 != op.r1 {
                    self.wr_gpr(op.r0, w, g(op.r1));
                } else {
                    self.set_lb(op.r0, w);
                }
            }
            K_MOV_RI => {
                if w >= 32 {
                    self.a.mov_imm(w == 64, g(op.r0), op.imm);
                    self.set_lb(op.r0, w);
                } else {
                    self.a.mov_imm(false, T0, op.imm);
                    self.wr_gpr(op.r0, w, T0);
                }
            }
            K_MOV_RM => {
                if w == 64
                    && (!mem::tso() || !stack_tso())
                    && !self.is_explicit_volatile(k)
                    && op.base == 4
                    && op.index == RNONE
                    && op.r0 != 4
                    && op.disp % 8 == 0
                    && (-512..=504).contains(&op.disp)
                {
                    let body = self.body;
                    if let Some(n) = self.ops.get(k + 1).copied().filter(|n| {
                        k + 1 < body
                            && !self.is_explicit_volatile(k + 1)
                            && n.kind == K_MOV_RM
                            && n.w == 64
                            && n.base == 4
                            && n.index == RNONE
                            && n.disp == op.disp + 8
                            && n.r0 != 4
                            && n.r0 != op.r0
                    }) {
                        self.a.ldp_x(g(op.r0), g(n.r0), g(4), op.disp as i32);
                        self.set_lb(op.r0, 64);
                        self.set_lb(n.r0, 64);
                        self.skip = 1;
                        return;
                    }
                }
                if w >= 32 {
                    self.load_op(k, g(op.r0), &op, w as u32);
                    self.set_lb(op.r0, w);
                } else {
                    self.load_op(k, T0, &op, w as u32);
                    self.wr_gpr(op.r0, w, T0);
                }
            }
            K_MOV_MR => {
                if w == 64
                    && (!mem::tso() || !stack_tso())
                    && !self.is_explicit_volatile(k)
                    && op.base == 4
                    && op.index == RNONE
                    && op.disp % 8 == 0
                    && (-512..=504).contains(&op.disp)
                {
                    let body = self.body;
                    if let Some(n) = self.ops.get(k + 1).copied().filter(|n| {
                        k + 1 < body
                            && !self.is_explicit_volatile(k + 1)
                            && n.kind == K_MOV_MR
                            && n.w == 64
                            && n.base == 4
                            && n.index == RNONE
                            && n.disp == op.disp + 8
                    }) {
                        self.a.stp_x(g(op.r1), g(n.r1), g(4), op.disp as i32);
                        self.skip = 1;
                        return;
                    }
                }
                self.store_op(k, g(op.r1), &op, w as u32);
            }
            K_MOV_MI => {
                let src = if op.imm == 0 {
                    ZR
                } else {
                    self.a.mov_imm(w == 64, T0, op.imm);
                    T0
                };
                self.store_op(k, src, &op, w as u32);
            }
            K_MOVZX_RR | K_MOVSX_RR => {
                let sw = (op.x & 0x3F) as u32;
                let sx = op.kind == K_MOVSX_RR;
                if w >= 32 {
                    let x = w == 64;
                    if sx {
                        self.a.sbfm(x, g(op.r0), g(op.r1), 0, sw - 1);
                    } else {
                        self.a.ubfm(x, g(op.r0), g(op.r1), 0, sw - 1);
                    }
                    self.set_lb(op.r0, w);
                } else {
                    if sx {
                        self.a.sbfm(false, T0, g(op.r1), 0, sw - 1);
                    } else {
                        self.a.ubfm(false, T0, g(op.r1), 0, sw - 1);
                    }
                    self.wr_gpr(op.r0, w, T0);
                }
            }
            K_MOVZX_RM | K_MOVSX_RM => {
                let sw = (op.x & 0x3F) as u32;
                if op.kind == K_MOVZX_RM && w >= 32 {
                    self.load_op(k, g(op.r0), &op, sw);
                    self.set_lb(op.r0, w);
                } else {
                    self.load_op(k, T0, &op, sw);
                    if op.kind == K_MOVSX_RM {
                        if w >= 32 {
                            self.a.sbfm(w == 64, g(op.r0), T0, 0, sw - 1);
                            self.set_lb(op.r0, w);
                        } else {
                            self.a.sbfm(false, T0, T0, 0, sw - 1);
                            self.wr_gpr(op.r0, w, T0);
                        }
                    } else {
                        self.wr_gpr(op.r0, w, T0);
                    }
                }
            }
            K_LEA => {
                if w == 64 {
                    let r = self.addr_into(&op, g(op.r0));
                    if r != g(op.r0) {
                        self.a.mov_rr(true, g(op.r0), r);
                    }
                    self.set_lb(op.r0, 64);
                } else if w == 32 && op.base < RNONE && (op.base & (fast::B_GS | fast::B_ADDR32)) == 0 {
                    // Direct 32-bit `lea r32, [base + idx*s + disp]` into `wD` (1 insn instead of
                    // `add x16; mov wD, w16`, zero-extending the upper 32 bits automatically).
                    let d = g(op.r0);
                    let s = (op.x >> 6) as u32;
                    let disp = op.disp as i32 as i64;
                    if op.index != RNONE {
                        self.a.addsub_rr(false, false, false, d, g(op.base), g(op.index), s);
                        if disp != 0 {
                            self.a.add_imm(false, d, d, disp);
                        }
                    } else if disp == 0 {
                        self.a.mov_rr(false, d, g(op.base));
                    } else {
                        self.a.add_imm(false, d, g(op.base), disp);
                    }
                    self.set_lb(op.r0, 32);
                } else {
                    let r = self.addr_into(&op, XA);
                    self.wr_gpr(op.r0, w, r);
                }
            }
            K_ALU_RI if op.r0 == 4 && op.w == 64 && (op.x & 0x3F) == A_ADD && self.try_batch_epilogue(k) => {}
            K_ALU_RR | K_ALU_RI | K_ALU_RM | K_ALU_MR | K_ALU_MI | K_CMPJ_RR | K_CMPJ_RI | K_CMPJ_RM => self.alu(k, &op),
            K_INC_R | K_DEC_R => self.incdec(k, &op),
            K_SHIFT_RI => self.shift_imm(k, &op),
            K_SHIFT_RCL => self.shift_cl(k, &op),
            K_PUSH_R if pushpop_enabled() && op.r0 != 4 => {
                if !self.try_batch_push(k) {
                    self.a.str_pre(g(op.r0), g(4), -8);
                }
            }
            K_PUSH_R => {
                self.a.addsub_ri(true, true, false, XA, g(4), 8, false);
                self.store(k, g(op.r0), XA, 64, Chk::Stack);
                self.a.mov_rr(true, g(4), XA);
            }
            K_POP_R if pushpop_enabled() && op.r0 != 4 => {
                if !self.try_batch_epilogue(k) {
                    self.a.ldr_post(g(op.r0), g(4), 8);
                    self.set_lb(op.r0, 64);
                }
            }
            K_POP_R => {
                self.load(k, T0, g(4), 64, Chk::Stack);
                self.a.addsub_ri(true, false, false, g(4), g(4), 8, false);
                self.wr_gpr(op.r0, 64, T0);
            }
            K_SETCC_R => {
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
                self.wr_gpr(op.r0, 8, T0);
            }
            K_CMOV_RR | K_CMOV_RM => self.cmov(k, &op),
            K_NOT_R => {
                // `orn d, zr, m` (`mvn`).
                self.a.emit(sf(w == 64) | 0x2A20_03E0 | g(op.r0) << 16 | 31 << 5 | T0);
                self.wr_gpr(op.r0, w, T0);
            }
            K_CX => match op.x & 0x3F {
                0 => self.a.sbfm(false, g(0), g(0), 0, 15),
                1 => self.a.sbfm(true, g(0), g(0), 0, 31),
                2 => self.a.asr_i(false, g(2), g(0), 31),
                3 => self.a.asr_i(true, g(2), g(0), 63),
                4 => {
                    self.a.sbfm(false, T0, g(0), 0, 7);
                    self.a.bfxil0(g(0), T0, 16);
                }
                _ => {
                    // Cwd: sign-extend AX into DX:AX (preserving bits [63:16] of RDX).
                    self.a.sbfm(false, T0, g(0), 0, 15);
                    self.a.asr_i(false, T0, T0, 31);
                    self.a.bfxil0(g(2), T0, 16);
                }
            },
            K_XCHG_RR => {
                self.a.mov_rr(true, T0, g(op.r0));
                self.wr_gpr(op.r0, w, g(op.r1));
                self.wr_gpr(op.r1, w, T0);
            }
            K_MOVD_GX => {
                let off = OFF_XMM + (op.r1 as u32 & 15) * 16;
                self.a.ldr(w as u32, g(op.r0), XCPU, off);
                self.set_lb(op.r0, w);
            }
            K_MOVD_XG => {
                let d = (op.r0 as u32 & 15) * 16;
                let s = g(op.r1);
                if op.imm == 64 {
                    self.a.str(64, s, XCPU, OFF_XMM + d);
                } else {
                    self.a.mov_rr(false, T0, s);
                    self.a.str(64, T0, XCPU, OFF_XMM + d);
                }
                self.a.str(64, ZR, XCPU, OFF_XMM + d + 8);
                if op.w & 3 == VM_V128 {
                    self.clear_ymm_hi(op.r0);
                }
            }
            K_VZU => {
                self.emit_ymm_clear_check(0xFFFF);
                self.ymm_clean = 0xFFFF;
            }
            K_VST => {
                let a = self.addr_into(&op, XA);
                let chk = Self::chk_of(&op);
                let s = OFF_XMM + (op.r1 as u32 & 15) * 16;
                match op.w >> 4 {
                    2 => {
                        self.a.ldr(32, T0, XCPU, s);
                        self.store(k, T0, a, 32, chk);
                    }
                    3 => {
                        self.a.ldr(64, T0, XCPU, s);
                        self.store(k, T0, a, 64, chk);
                    }
                    6 => {
                        self.a.ldr(64, T0, XCPU, s + 8);
                        self.store(k, T0, a, 64, chk);
                    }
                    4 => {
                        if chk == Chk::Full {
                            let se = self.side_exit_here(k);
                            self.range_check_op(k, a, se);
                        }
                        self.a.ldr(64, T0, XCPU, s);
                        self.a.ldr(64, T1, XCPU, s + 8);
                        if self.vec_tso(k, chk, false) {
                            self.a.dmb_ishst();
                        }
                        self.a.str(64, T0, a, 0);
                        self.a.str(64, T1, a, 8);
                    }
                    _ => {
                        if chk == Chk::Full {
                            let se = self.side_exit_here(k);
                            self.range_check_op(k, a, se);
                        }
                        let sh = OFF_YMM_HI + (op.r1 as u32 & 15) * 16;
                        self.a.ldr(64, T0, XCPU, s);
                        self.a.ldr(64, T1, XCPU, s + 8);
                        self.a.ldr(64, T2, XCPU, sh);
                        self.a.ldr(64, T3, XCPU, sh + 8);
                        if self.vec_tso(k, chk, false) {
                            self.a.dmb_ishst();
                        }
                        self.a.str(64, T0, a, 0);
                        self.a.str(64, T1, a, 8);
                        self.a.str(64, T2, a, 16);
                        self.a.str(64, T3, a, 24);
                    }
                }
            }
            K_CVTSI_R | K_CVTSI_M => {
                let width = (op.imm & 0xFF) as u32;
                let is_sd = (op.x & 0x3F) != 0;
                let src = if op.kind == K_CVTSI_R {
                    g(op.r1)
                } else {
                    let a = self.addr_into(&op, XA);
                    self.load(k, T0, a, width, Self::chk_of(&op));
                    T0
                };
                self.a.scvtf_gp(width == 64, is_sd, 0, src);
                self.a.fmov_fg(is_sd, T1, 0);
                let s1 = ((op.imm >> 8) as u32 & 15) * 16;
                let d = (op.r0 as u32 & 15) * 16;
                if is_sd {
                    self.a.ldr(64, T2, XCPU, OFF_XMM + s1 + 8);
                    self.a.str(64, T1, XCPU, OFF_XMM + d);
                    self.a.str(64, T2, XCPU, OFF_XMM + d + 8);
                } else {
                    self.a.ldr(64, T2, XCPU, OFF_XMM + s1);
                    self.a.ldr(64, T3, XCPU, OFF_XMM + s1 + 8);
                    self.a.bfxil0(T2, T1, 32);
                    self.a.str(64, T2, XCPU, OFF_XMM + d);
                    self.a.str(64, T3, XCPU, OFF_XMM + d + 8);
                }
                if op.w & 3 == VM_V128 {
                    self.clear_ymm_hi(op.r0);
                }
            }
            K_COMI_RR | K_COMI_RM => {
                let is_sd = (op.x & 0x3F) != 0;
                let bits = if is_sd { 64 } else { 32 };
                let la = self.live[k + 1];
                let off_a = OFF_XMM + (op.r0 as u32 & 15) * 16;
                if op.kind == K_COMI_RM {
                    let a = self.addr_into(&op, XA);
                    self.load(k, T1, a, bits, Self::chk_of(&op));
                } else if la & FALL != 0 {
                    let off_b = OFF_XMM + (op.r1 as u32 & 15) * 16;
                    self.a.ldr(bits, T1, XCPU, off_b);
                }
                if la & FALL != 0 {
                    self.a.ldr(bits, T0, XCPU, off_a);
                    self.a.fmov_gf(is_sd, 0, T0);
                    self.a.fmov_gf(is_sd, 1, T1);
                    self.a.fcmp(is_sd, 0, 1);
                    if la & FC != 0 {
                        self.a.cset(T0, LT);
                        self.a.str(8, T0, XCPU, OFF_CF);
                    }
                    if la & FZ != 0 {
                        self.a.cset(T0, EQ);
                        self.a.cset(T1, VS);
                        self.a.logic_rr(false, 1, T0, T0, T1, 0, 0);
                        self.a.str(8, T0, XCPU, OFF_ZF);
                    }
                    if la & FS != 0 {
                        self.a.str(8, ZR, XCPU, OFF_SF);
                    }
                    if la & FO != 0 {
                        self.a.str(8, ZR, XCPU, OFF_OF);
                    }
                    if la & FP != 0 {
                        self.a.cset(T0, VC);
                        self.a.str(8, T0, XCPU, OFF_PFV);
                    }
                }
                self.fs = FlagState {
                    src: Src::None,
                    host: 0,
                    mem: (self.fs.mem & !FALL) | (la & FALL),
                };
            }
            K_CVTF2I_R | K_CVTF2I_M => {
                self.flush_flags(self.live[k]);
                self.fs.host = 0;
                let is_sd = (op.x & 1) != 0;
                let trunc = (op.x & 2) != 0;
                let bits = if is_sd { 64 } else { 32 };
                if op.kind == K_CVTF2I_R {
                    let off = OFF_XMM + (op.r1 as u32 & 15) * 16;
                    self.a.ldr(bits, T0, XCPU, off);
                } else {
                    let a = self.addr_into(&op, XA);
                    self.load(k, T0, a, bits, Self::chk_of(&op));
                }
                self.a.fmov_gf(is_sd, 0, T0);
                if !is_sd {
                    self.a.fcvt_d_s(0, 0);
                }
                if trunc {
                    self.a.frintz_d(0, 0);
                } else {
                    self.a.s_frint(true, 4, 0, 0);
                }
                let x = w == 64;
                self.a.fcvtzs_d(x, T0, 0);
                let limit = if x { 0x43E0_0000_0000_0000u64 } else { 0x41E0_0000_0000_0000u64 };
                self.a.mov_imm(true, T1, limit);
                self.a.fmov_gf(true, 1, T1);
                self.a.fcmp(true, 0, 1);
                let indef = if x { 0x8000_0000_0000_0000u64 } else { 0x8000_0000u64 };
                self.a.mov_imm(x, T2, indef);
                self.a.csel(x, T0, T2, T0, HS);
                self.wr_gpr(op.r0, w, T0);
            }
            K_IMUL_RR | K_IMUL_RM | K_IMUL_RRI | K_IMUL_RMI => {
                let la = self.live[k + 1];
                let (a_src, b_src) = match op.kind {
                    K_IMUL_RR => (g(op.r0), g(op.r1)),
                    K_IMUL_RM => {
                        self.load_op(k, T1, &op, w as u32);
                        (g(op.r0), T1)
                    }
                    K_IMUL_RRI => {
                        let sext_imm = match w {
                            16 => op.imm as u16 as i16 as i64 as u64,
                            32 => op.imm as u32 as i32 as i64 as u64,
                            _ => op.imm,
                        };
                        self.a.mov_imm(true, T1, sext_imm);
                        (g(op.r1), T1)
                    }
                    _ => {
                        self.load_op(k, T0, &op, w as u32);
                        let sext_imm = match w {
                            16 => op.imm as u16 as i16 as i64 as u64,
                            32 => op.imm as u32 as i32 as i64 as u64,
                            _ => op.imm,
                        };
                        self.a.mov_imm(true, T1, sext_imm);
                        (T0, T1)
                    }
                };
                if la & (FC | FO) == 0 {
                    // Dead CF/OF: single-instruction native 32/64-bit `mul` directly into dst.
                    if w >= 32 {
                        self.a.mul(w == 64, g(op.r0), a_src, b_src);
                        self.set_lb(op.r0, w);
                    } else {
                        self.a.mul(false, T2, a_src, b_src);
                        self.wr_gpr(op.r0, w, T2);
                    }
                    self.fs.host &= !(FC | FO);
                    self.fs.mem &= !(FC | FO);
                    return;
                }
                if w == 64 {
                    self.a.mul(true, T2, a_src, b_src);
                    if la & (FC | FO) != 0 {
                        self.a.smulh(T3, a_src, b_src);
                        self.a.asr_i(true, T4, T2, 63);
                        self.a.logic_rr(true, 2, T3, T3, T4, 0, 0);
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
                    self.a.sbfm(true, T0, a_src, 0, (w - 1) as u32);
                    if matches!(op.kind, K_IMUL_RR | K_IMUL_RM) {
                        self.a.sbfm(true, T1, b_src, 0, (w - 1) as u32);
                    }
                    self.a.mul(true, T2, T0, T1);
                    if la & (FC | FO) != 0 {
                        self.a.sbfm(true, T3, T2, 0, (w - 1) as u32);
                        self.a.logic_rr(true, 2, T3, T2, T3, 0, 0);
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
                self.wr_gpr(op.r0, w, T2);
                self.fs.host &= !(FC | FO);
                self.fs.mem = (self.fs.mem & !(FC | FO)) | (la & (FC | FO));
            }
            K_NEG_R => {
                let la = self.live[k + 1];
                let need_nzcv = la & FNZCV != 0;
                let need_pf = la & FP != 0;
                let d = g(op.r0);
                let res = if w >= 32 {
                    self.a.addsub_rr(w == 64, true, need_nzcv, d, ZR, d, 0);
                    self.set_lb(op.r0, w);
                    d
                } else {
                    let sh = 64 - w as u32;
                    if need_nzcv {
                        self.a.lsl_i(true, T3, d, sh);
                        self.a.addsub_rr(true, true, true, T5, ZR, T3, 0);
                        self.a.lsr_i(true, T2, T5, sh);
                    } else {
                        self.a.addsub_rr(false, true, false, T2, ZR, d, 0);
                    }
                    self.wr_gpr(op.r0, w, T2);
                    T2
                };
                if need_pf {
                    self.a.str(8, res, XCPU, OFF_PFV);
                }
                self.fs.mem &= !FALL;
                if need_pf {
                    self.fs.mem |= FP;
                }
                self.fs.host = if need_nzcv { la & FNZCV } else { 0 };
                self.fs.src = Src::Sub;
            }
            K_INC_M | K_DEC_M => {
                let dec = op.kind == K_DEC_M;
                let wb = w as u32;
                let la = self.live[k + 1];
                if la & FC != 0 && self.fs.host & FC != 0 && self.fs.mem & FC == 0 {
                    self.flush_flags(FC);
                }
                let need_nzcv = la & (FZ | FS | FO) != 0;
                let need_pf = la & FP != 0;
                let addr = self.addr_into(&op, XA);
                self.load(k, T0, addr, wb, Self::chk_of(&op));
                if wb >= 32 {
                    self.a.addsub_ri(wb == 64, dec, need_nzcv, T2, T0, 1, false);
                } else if need_nzcv {
                    let sh = 64 - wb;
                    self.a.lsl_i(true, T3, T0, sh);
                    self.a.mov_imm(true, T4, 1u64 << sh);
                    self.a.addsub_rr(true, dec, true, T5, T3, T4, 0);
                    self.a.lsr_i(true, T2, T5, sh);
                } else {
                    self.a.addsub_ri(false, dec, false, T2, T0, 1, false);
                }
                self.store_checked_addr(k, T2, addr, wb);
                if need_pf {
                    self.a.str(8, T2, XCPU, OFF_PFV);
                }
                self.fs.mem &= !(FZ | FS | FO | FP);
                if need_pf {
                    self.fs.mem |= FP;
                }
                let cf_mem = self.fs.mem & FC;
                self.fs.mem = (self.fs.mem & !FC) | cf_mem;
                self.fs.host = if need_nzcv { la & (FZ | FS | FO) } else { 0 };
                self.fs.src = Src::IncDec;
            }
            K_BSF_RR | K_BSF_RM => {
                let wb = w as u32;
                let la = self.live[k + 1];
                let v = if op.kind == K_BSF_RR {
                    self.ua(op.r1, wb)
                } else {
                    let a = self.addr_into(&op, XA);
                    self.load(k, T0, a, wb, Self::chk_of(&op));
                    T0
                };
                if la & (FC | FZ) != 0 {
                    self.a.clz(true, T2, v);
                    self.a.lsr_i(true, T2, T2, 6);
                    if la & FC != 0 {
                        self.a.str(8, T2, XCPU, OFF_CF);
                    }
                    if la & FZ != 0 {
                        self.a.str(8, T2, XCPU, OFF_ZF);
                    }
                    self.fs.host &= !(FC | FZ);
                    self.fs.mem = (self.fs.mem & !(FC | FZ)) | (la & (FC | FZ));
                }
                if tracked(op.r0) {
                    self.flush_lb_reg(op.r0 as usize);
                }
                let skip = self.a.cbz(true, v);
                self.a.rbit(true, T1, v);
                self.a.clz(true, T1, T1);
                let d = g(op.r0);
                match wb {
                    64 => self.a.mov_rr(true, d, T1),
                    32 => self.a.mov_rr(false, d, T1),
                    _ => self.a.bfxil0(d, T1, wb),
                }
                self.lb_store_now(op.r0, w);
                self.a.bind(skip);
            }
            K_BT_RR | K_BT_RI => {
                let wb = w as u32;
                let la = self.live[k + 1];
                if la & FC != 0 {
                    if op.kind == K_BT_RI {
                        let b = (op.imm as u32) & (wb - 1);
                        self.a.ubfx(T0, g(op.r0), b, 1);
                    } else {
                        self.a.logic_ri(false, 0, T1, g(op.r1), enc_logic_imm((wb - 1) as u64, false).unwrap());
                        self.a.shiftv(true, 9, T0, g(op.r0), T1);
                        self.a.ubfx(T0, T0, 0, 1);
                    }
                    self.a.str(8, T0, XCPU, OFF_CF);
                    self.fs.host &= !FC;
                    self.fs.mem |= FC;
                }
            }
            K_ROT_RI => {
                let wb = w as u32;
                let cnt = op.imm as u32;
                let ror = (op.x & 0x3F) != 0;
                let la = self.live[k + 1];
                let ua = self.ua(op.r0, wb);
                match wb {
                    64 => self.a.ror_i(true, T1, ua, if ror { cnt } else { 64 - cnt }),
                    32 => self.a.ror_i(false, T1, ua, if ror { cnt } else { 32 - cnt }),
                    _ => {
                        let c_mod = cnt % wb;
                        if c_mod == 0 {
                            self.a.mov_rr(false, T1, ua);
                        } else {
                            let rcnt = if ror { c_mod } else { wb - c_mod };
                            self.a.lsr_i(false, T1, ua, rcnt);
                            self.a.lsl_i(false, T2, ua, wb - rcnt);
                            self.a.logic_rr(false, 1, T1, T1, T2, 0, 0);
                            self.a.ubfm(false, T1, T1, 0, wb - 1);
                        }
                    }
                }
                self.wr_gpr(op.r0, w, T1);
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
            K_VSHUF_RM if op.w & 3 != VM_V256 && (op.x & 0x3F) <= VS_ZX64 => {
                let a = self.addr_into(&op, XA);
                let chk = Self::chk_of(&op);
                let d = (op.r0 as u32 & 15) * 16;
                match op.x & 0x3F {
                    VS_ZX32 => {
                        self.load(k, T0, a, 32, chk);
                        self.a.str(64, T0, XCPU, OFF_XMM + d);
                        self.a.str(64, ZR, XCPU, OFF_XMM + d + 8);
                    }
                    VS_ZX64 => {
                        self.load(k, T0, a, 64, chk);
                        self.a.str(64, T0, XCPU, OFF_XMM + d);
                        self.a.str(64, ZR, XCPU, OFF_XMM + d + 8);
                    }
                    _ if chk == Chk::Ro => {
                        self.a.ldr(128, 0, a, 0);
                        self.a.str(128, 0, XCPU, OFF_XMM + d);
                    }
                    _ => {
                        if chk == Chk::Full {
                            let se = self.side_exit_here(k);
                            self.range_check_op(k, a, se);
                        }
                        self.a.ldr(64, T0, a, 0);
                        self.a.ldr(64, T1, a, 8);
                        if self.vec_tso(k, chk, true) {
                            self.a.dmb_ishld();
                        }
                        self.a.str(64, T0, XCPU, OFF_XMM + d);
                        self.a.str(64, T1, XCPU, OFF_XMM + d + 8);
                    }
                }
                if op.w & 3 == VM_V128 {
                    self.clear_ymm_hi(op.r0);
                }
            }
            K_VSHUF_RR if op.w & 3 != VM_V256 && (op.x & 0x3F) <= VS_AND => {
                let d = (op.r0 as u32 & 15) * 16;
                let s1 = ((op.imm >> 8) as u32 & 15) * 16;
                let s2 = (op.r1 as u32 & 15) * 16;
                match op.x & 0x3F {
                    VS_MOV => {
                        if d != s2 {
                            self.a.ldr(64, T0, XCPU, OFF_XMM + s2);
                            self.a.ldr(64, T1, XCPU, OFF_XMM + s2 + 8);
                            self.a.str(64, T0, XCPU, OFF_XMM + d);
                            self.a.str(64, T1, XCPU, OFF_XMM + d + 8);
                        }
                    }
                    VS_ZX32 => {
                        self.a.ldr(32, T0, XCPU, OFF_XMM + s2);
                        self.a.str(64, T0, XCPU, OFF_XMM + d);
                        self.a.str(64, ZR, XCPU, OFF_XMM + d + 8);
                    }
                    VS_ZX64 => {
                        self.a.ldr(64, T0, XCPU, OFF_XMM + s2);
                        self.a.str(64, T0, XCPU, OFF_XMM + d);
                        self.a.str(64, ZR, XCPU, OFF_XMM + d + 8);
                    }
                    VS_MRG32 => {
                        self.a.ldr(64, T0, XCPU, OFF_XMM + s1);
                        self.a.ldr(64, T2, XCPU, OFF_XMM + s1 + 8);
                        self.a.ldr(32, T1, XCPU, OFF_XMM + s2);
                        self.a.bfxil0(T0, T1, 32);
                        self.a.str(64, T0, XCPU, OFF_XMM + d);
                        self.a.str(64, T2, XCPU, OFF_XMM + d + 8);
                    }
                    VS_MRG64 => {
                        self.a.ldr(64, T0, XCPU, OFF_XMM + s2);
                        self.a.ldr(64, T1, XCPU, OFF_XMM + s1 + 8);
                        self.a.str(64, T0, XCPU, OFF_XMM + d);
                        self.a.str(64, T1, XCPU, OFF_XMM + d + 8);
                    }
                    VS_XOR if s1 == s2 => {
                        self.a.str(64, ZR, XCPU, OFF_XMM + d);
                        self.a.str(64, ZR, XCPU, OFF_XMM + d + 8);
                    }
                    sub => {
                        let opc = match sub {
                            VS_XOR => 2,
                            VS_OR => 1,
                            _ => 0,
                        };
                        self.a.ldr(64, T0, XCPU, OFF_XMM + s1);
                        self.a.ldr(64, T1, XCPU, OFF_XMM + s1 + 8);
                        self.a.ldr(64, T2, XCPU, OFF_XMM + s2);
                        self.a.ldr(64, T3, XCPU, OFF_XMM + s2 + 8);
                        self.a.logic_rr(true, opc, T0, T0, T2, 0, 0);
                        self.a.logic_rr(true, opc, T1, T1, T3, 0, 0);
                        self.a.str(64, T0, XCPU, OFF_XMM + d);
                        self.a.str(64, T1, XCPU, OFF_XMM + d + 8);
                    }
                }
                if op.w & 3 == VM_V128 {
                    self.clear_ymm_hi(op.r0);
                }
            }
            K_VSHUF_RR | K_VSHUF_RM | K_VINT_RR | K_VINT_RM | K_VFP_RR | K_VFP_RM => self.vop(k, &op),
            K_BSWAP_R..=K_MMX_MISC => self.ext_op(k, &op),
            K_X87 => self.emit_x87_op(k, &op),
            _ => unreachable!("unsupported op kind {}", op.kind),
        }
    }

    pub(super) fn alu(&mut self, k: usize, op: &FastOp) {
        use fast::*;
        let sub = op.x & 0x3F;
        let w = op.w as u32;
        let la = self.live[k + 1];
        let need_nzcv = la & FNZCV != 0;
        let need_pf = la & FP != 0;
        let is_cmp = sub == A_CMP || sub == A_TEST;
        let mem_a = matches!(op.kind, K_ALU_MR | K_ALU_MI);
        let mem_dst = mem_a && !is_cmp;
        let writes_reg = !mem_a && !is_cmp;
        if is_cmp && !need_nzcv && !need_pf {
            // Dead compare: only the memory operand's software range check remains observable
            // (under `hw_range_enabled()`, `range_check_op` is a no-op so avoid dead `XA` math).
            if !hw_range_enabled() && matches!(op.kind, K_ALU_RM | K_CMPJ_RM | K_ALU_MR | K_ALU_MI) {
                if let Grp::Member { id } = self.grp[k] {
                    if self.grp_ok & (1u64 << id) != 0 {
                        self.grp_k = k;
                        self.fs.host = 0;
                        self.fs.mem &= !FALL;
                        return;
                    }
                }
                if Self::chk_of(op) == Chk::Full {
                    let a = self.addr_into(op, XA);
                    let se = self.side_exit_here(k);
                    self.range_check_op(k, a, se);
                }
            }
            self.fs.host = 0;
            self.fs.mem &= !FALL;
            return;
        }
        // Operands.
        let mut addr = 0u32;
        let a_reg = if mem_a {
            if is_cmp {
                self.load_op(k, T0, op, w);
            } else {
                addr = self.addr_into(op, XA);
                self.load(k, T0, addr, w, Self::chk_of(op));
            }
            T0
        } else {
            g(op.r0)
        };
        enum B {
            R(u32),
            I(u64),
        }
        let b = match op.kind {
            K_ALU_RR | K_ALU_MR | K_CMPJ_RR => B::R(g(op.r1)),
            K_ALU_RI | K_ALU_MI | K_CMPJ_RI => B::I(op.imm),
            _ => {
                self.load_op(k, T1, op, w);
                B::R(T1)
            }
        };
        let mask = if w == 64 { u64::MAX } else { (1u64 << w) - 1 };
        if matches!(sub, A_ADC | A_SBB) {
            self.flag_reg(FC, T6);
        }
        let res: u32;
        if w >= 32 {
            let x = w == 64;
            let dst = if writes_reg {
                g(op.r0)
            } else if mem_dst || need_pf {
                T2
            } else {
                ZR
            };
            let s = need_nzcv || dst == ZR;
            match sub {
                A_ADC | A_SBB => {
                    let b_reg = match b {
                        B::R(r) => r,
                        B::I(v) => {
                            self.a.mov_imm(x, T1, v & mask);
                            T1
                        }
                    };
                    if sub == A_ADC {
                        self.a.addsub_ri(false, true, true, ZR, T6, 1, false);
                        self.a.adcs(x, s, dst, a_reg, b_reg);
                    } else {
                        self.a.addsub_rr(false, true, true, ZR, ZR, T6, 0);
                        self.a.sbcs(x, s, dst, a_reg, b_reg);
                    }
                }
                A_ADD | A_SUB | A_CMP => {
                    let is_sub = sub != A_ADD;
                    match b {
                        B::R(r) => self.a.addsub_rr(x, is_sub, s, dst, a_reg, r, 0),
                        B::I(v) => {
                            let v = v & mask;
                            let nv = v.wrapping_neg() & mask;
                            if v < 4096 {
                                self.a.addsub_ri(x, is_sub, s, dst, a_reg, v as u32, false);
                            } else if v & 0xFFF == 0 && v < (1 << 24) {
                                self.a.addsub_ri(x, is_sub, s, dst, a_reg, (v >> 12) as u32, true);
                            } else if nv != 0 && nv < 4096 {
                                self.a.addsub_ri(x, !is_sub, s, dst, a_reg, nv as u32, false);
                            } else {
                                self.a.mov_imm(x, T1, v);
                                self.a.addsub_rr(x, is_sub, s, dst, a_reg, T1, 0);
                            }
                        }
                    }
                }
                _ => {
                    let opc = match sub {
                        A_AND | A_TEST => {
                            if s {
                                3
                            } else {
                                0
                            }
                        }
                        A_OR => 1,
                        _ => 2,
                    };
                    match b {
                        B::R(r) => self.a.logic_rr(x, opc, dst, a_reg, r, 0, 0),
                        B::I(v) => {
                            if let Some(e) = enc_logic_imm(v & mask, x) {
                                self.a.logic_ri(x, opc, dst, a_reg, e);
                            } else {
                                self.a.mov_imm(x, T1, v & mask);
                                self.a.logic_rr(x, opc, dst, a_reg, T1, 0, 0);
                            }
                        }
                    }
                    if s && (opc == 1 || opc == 2) {
                        self.a.logic_rr(x, 3, ZR, dst, dst, 0, 0); // tst
                    }
                }
            }
            res = dst;
            if writes_reg {
                self.set_lb(op.r0, w as u8);
            }
        } else if matches!(sub, A_ADC | A_SBB) {
            self.a.ubfm(false, T3, a_reg, 0, w - 1);
            match b {
                B::R(r) => self.a.ubfm(false, T4, r, 0, w - 1),
                B::I(v) => self.a.mov_imm(false, T4, v & mask),
            }
            let is_sub = sub == A_SBB;
            self.a.addsub_rr(false, is_sub, false, T5, T3, T4, 0);
            self.a.addsub_rr(false, is_sub, false, T5, T5, T6, 0);
            self.a.ubfm(false, T2, T5, 0, w - 1);
            if need_nzcv {
                if la & FC != 0 {
                    self.a.ubfx(XS, T5, w, 1);
                    self.a.str(8, XS, XCPU, OFF_CF);
                }
                if la & FZ != 0 {
                    self.a.clz(true, XS, T2);
                    self.a.lsr_i(true, XS, XS, 6);
                    self.a.str(8, XS, XCPU, OFF_ZF);
                }
                if la & FS != 0 {
                    self.a.ubfx(XS, T2, w - 1, 1);
                    self.a.str(8, XS, XCPU, OFF_SF);
                }
                if la & FO != 0 {
                    if is_sub {
                        self.a.logic_rr(false, 2, T4, T3, T4, 0, 0);
                        self.a.logic_rr(false, 2, T3, T3, T2, 0, 0);
                    } else {
                        self.a.logic_rr(false, 2, T3, T3, T2, 0, 0);
                        self.a.logic_rr(false, 2, T4, T4, T2, 0, 0);
                    }
                    self.a.logic_rr(false, 0, T3, T3, T4, 0, 0);
                    self.a.ubfx(XS, T3, w - 1, 1);
                    self.a.str(8, XS, XCPU, OFF_OF);
                }
            }
            if writes_reg {
                self.wr_gpr(op.r0, w as u8, T2);
            }
            if mem_dst {
                self.store_checked_addr(k, T2, addr, w);
            }
            if need_pf {
                self.a.str(8, T2, XCPU, OFF_PFV);
            }
            self.fs = FlagState {
                src: Src::None,
                host: 0,
                mem: (self.fs.mem & !FALL) | (la & FALL),
            };
            return;
        } else if sub == A_TEST && need_nzcv && (la & (FS | FP)) == 0 {
            // 8/16-bit `test reg/mem, imm/reg` when only ZF (and constant-0 CF/OF) are live:
            // `tst w_a, #(imm & mask)` directly sets Z in 1 instruction (no shift/un-shift).
            match b {
                B::I(v) => {
                    let mv = v & mask;
                    if let Some(e) = enc_logic_imm(mv, false) {
                        self.a.logic_ri(false, 3, ZR, a_reg, e);
                    } else {
                        self.a.mov_imm(false, T4, mv);
                        self.a.logic_rr(false, 3, ZR, a_reg, T4, 0, 0);
                    }
                }
                B::R(r) => {
                    self.a.ubfm(false, T4, r, 0, w - 1);
                    self.a.logic_rr(false, 3, ZR, a_reg, T4, 0, 0);
                }
            }
            res = ZR;
        } else {
            let sh = 64 - w;
            if need_nzcv {
                self.a.lsl_i(true, T3, a_reg, sh);
                match b {
                    B::R(r) => self.a.lsl_i(true, T4, r, sh),
                    B::I(v) => self.a.mov_imm(true, T4, (v & mask) << sh),
                }
                match sub {
                    A_ADD => self.a.addsub_rr(true, false, true, T5, T3, T4, 0),
                    A_SUB | A_CMP => self.a.addsub_rr(true, true, true, T5, T3, T4, 0),
                    A_AND | A_TEST => self.a.logic_rr(true, 3, T5, T3, T4, 0, 0),
                    A_OR => {
                        self.a.logic_rr(true, 1, T5, T3, T4, 0, 0);
                        self.a.logic_rr(true, 3, ZR, T5, T5, 0, 0);
                    }
                    A_XOR | _ => {
                        self.a.logic_rr(true, 2, T5, T3, T4, 0, 0);
                        self.a.logic_rr(true, 3, ZR, T5, T5, 0, 0);
                    }
                }
                if !is_cmp || need_pf {
                    self.a.lsr_i(true, T2, T5, sh);
                }
            } else {
                let br = match b {
                    B::R(r) => r,
                    B::I(v) => {
                        self.a.mov_imm(false, T4, v & mask);
                        T4
                    }
                };
                match sub {
                    A_ADD => self.a.addsub_rr(false, false, false, T2, a_reg, br, 0),
                    A_SUB | A_CMP => self.a.addsub_rr(false, true, false, T2, a_reg, br, 0),
                    A_AND | A_TEST => self.a.logic_rr(false, 0, T2, a_reg, br, 0, 0),
                    A_OR => self.a.logic_rr(false, 1, T2, a_reg, br, 0, 0),
                    _ => self.a.logic_rr(false, 2, T2, a_reg, br, 0, 0),
                }
            }
            res = T2;
            if writes_reg {
                self.wr_gpr(op.r0, w as u8, T2);
            }
        }
        if mem_dst {
            self.store_checked_addr(k, res, addr, w);
        }
        // AW2_JIT_FLAGLAZY: the parity byte may stay in the result register (`pf_defer`).
        let pf_mem = need_pf && !self.pf_defer(k, res, op.r0);
        if pf_mem {
            self.a.str(8, res, XCPU, OFF_PFV);
        }
        self.fs.mem &= !FALL;
        if pf_mem {
            self.fs.mem |= FP;
        }
        self.fs.host = if need_nzcv { la & FNZCV } else { 0 };
        self.fs.src = match sub {
            A_ADD | A_ADC => Src::Add,
            A_SUB | A_CMP | A_SBB => Src::Sub,
            _ => Src::Logic,
        };
    }

    pub(super) fn incdec(&mut self, k: usize, op: &FastOp) {
        let dec = op.kind == fast::K_DEC_R;
        let w = op.w as u32;
        let la = self.live[k + 1];
        if la & FC != 0 && self.fs.host & FC != 0 && self.fs.mem & FC == 0 {
            self.flush_flags(FC);
        }
        let need_nzcv = la & (FZ | FS | FO) != 0;
        let need_pf = la & FP != 0;
        let d = g(op.r0);
        let res;
        if w >= 32 {
            self.a.addsub_ri(w == 64, dec, need_nzcv, d, d, 1, false);
            res = d;
            self.set_lb(op.r0, w as u8);
        } else {
            let sh = 64 - w;
            if need_nzcv {
                self.a.lsl_i(true, T3, d, sh);
                self.a.mov_imm(true, T4, 1u64 << sh);
                self.a.addsub_rr(true, dec, true, T5, T3, T4, 0);
                self.a.lsr_i(true, T2, T5, sh);
            } else {
                self.a.addsub_ri(false, dec, false, T2, d, 1, false);
            }
            self.wr_gpr(op.r0, w as u8, T2);
            res = T2;
        }
        let pf_mem = need_pf && !self.pf_defer(k, res, op.r0);
        if pf_mem {
            self.a.str(8, res, XCPU, OFF_PFV);
        }
        self.fs.mem &= !(FZ | FS | FO | FP);
        if pf_mem {
            self.fs.mem |= FP;
        }
        // CF keeps its state: still in memory if it was (it was flushed above when live).
        let cf_mem = self.fs.mem & FC;
        self.fs.mem = (self.fs.mem & !FC) | cf_mem;
        self.fs.host = if need_nzcv { la & (FZ | FS | FO) } else { 0 };
        self.fs.src = Src::IncDec;
    }

    /// Store computed shift flags: T1 = result, T2 = CF, T3 = OF (`of_zero`: OF = 0).
    pub(super) fn shift_flags(&mut self, la: u8, w: u32, of_zero: bool) {
        if la & FC != 0 {
            self.a.str(8, T2, XCPU, OFF_CF);
        }
        if la & FO != 0 {
            if of_zero {
                self.a.str(8, ZR, XCPU, OFF_OF);
            } else {
                self.a.str(8, T3, XCPU, OFF_OF);
            }
        }
        if la & FZ != 0 {
            self.a.clz(true, T5, T1);
            self.a.lsr_i(true, T5, T5, 6);
            self.a.str(8, T5, XCPU, OFF_ZF);
        }
        if la & FS != 0 {
            self.a.ubfx(T6, T1, w - 1, 1);
            self.a.str(8, T6, XCPU, OFF_SF);
        }
        if la & FP != 0 {
            self.a.str(8, T1, XCPU, OFF_PFV);
        }
    }

    /// Zero-extended guest operand of width `w` (register `i`) into `T0` (or the register).
    pub(super) fn ua(&mut self, i: u8, w: u32) -> u32 {
        match w {
            64 => g(i),
            32 => {
                self.a.mov_rr(false, T0, g(i));
                T0
            }
            _ => {
                self.a.ubfm(true, T0, g(i), 0, w - 1);
                T0
            }
        }
    }

    pub(super) fn shift_imm(&mut self, k: usize, op: &FastOp) {
        let w = op.w as u32;
        let c = op.imm as u32;
        let sub = op.x & 0x3F;
        let la = self.live[k + 1];
        if w >= 32 && la == 0 && c != 0 && c < w {
            let d = g(op.r0);
            let x = w == 64;
            match sub {
                fast::SHIFT_SHL => self.a.lsl_i(x, d, d, c),
                fast::SHIFT_SHR => self.a.lsr_i(x, d, d, c),
                _ => self.a.asr_i(x, d, d, c),
            }
            self.set_lb(op.r0, w as u8);
            self.fs.host = 0;
            self.fs.mem &= !FALL;
            self.fs.src = Src::None;
            return;
        }
        let ua = self.ua(op.r0, w);
        let mut of_zero = false;
        match sub {
            fast::SHIFT_SHL => {
                if c >= 64 {
                    self.a.movz(true, T1, 0, 0);
                } else {
                    self.a.lsl_i(true, T1, ua, c);
                    if w < 64 {
                        self.a.ubfm(true, T1, T1, 0, w - 1);
                    }
                }
                if la & (FC | FO) != 0 {
                    if c <= w {
                        self.a.ubfx(T2, ua, w - c, 1);
                    } else {
                        self.a.movz(true, T2, 0, 0);
                    }
                    self.a.ubfx(T3, T1, w - 1, 1);
                    self.a.logic_rr(true, 2, T3, T3, T2, 0, 0);
                }
            }
            fast::SHIFT_SHR => {
                self.a.lsr_i(true, T1, ua, c);
                if la & FC != 0 {
                    self.a.ubfx(T2, ua, c - 1, 1);
                }
                if la & FO != 0 {
                    self.a.ubfx(T3, ua, w - 1, 1);
                }
            }
            _ => {
                let sa = if w < 64 {
                    self.a.sbfm(true, T4, ua, 0, w - 1);
                    T4
                } else {
                    ua
                };
                self.a.asr_i(true, T1, sa, c.min(63));
                if w < 64 {
                    self.a.ubfm(true, T1, T1, 0, w - 1);
                }
                if la & FC != 0 {
                    self.a.ubfx(T2, sa, (c - 1).min(63), 1);
                }
                of_zero = true;
            }
        }
        self.shift_flags(la, w, of_zero);
        self.wr_gpr(op.r0, w as u8, T1);
        self.fs.host = 0;
        self.fs.mem = (self.fs.mem & !FALL) | (la & FALL);
        self.fs.src = Src::None;
    }

    pub(super) fn shift_cl(&mut self, k: usize, op: &FastOp) {
        let w = op.w as u32;
        let sub = op.x & 0x3F;
        let la = self.live[k + 1];
        // Count 0 leaves everything unchanged: live flags must be in memory either way.
        self.flush_flags(la);
        if tracked(op.r0) {
            self.flush_lb_reg(op.r0 as usize);
        }
        let mask = if w == 64 { 63 } else { 31 };
        self.a.logic_ri(false, 0, T6, g(1), enc_logic_imm(mask, false).unwrap());
        let skip = self.a.cbz(false, T6);
        let ua = self.ua(op.r0, w);
        let mut of_zero = false;
        match sub {
            fast::SHIFT_SHL => {
                if w < 64 {
                    self.a.shiftv(true, 8, T4, ua, T6);
                    self.a.ubfx(T2, T4, w, 1);
                    self.a.ubfm(true, T1, T4, 0, w - 1);
                } else {
                    self.a.shiftv(true, 8, T1, ua, T6);
                    self.a.movz(true, T4, 64, 0);
                    self.a.addsub_rr(true, true, false, T4, T4, T6, 0);
                    self.a.shiftv(true, 9, T2, ua, T4);
                    self.a.ubfx(T2, T2, 0, 1);
                }
                self.a.ubfx(T3, T1, w - 1, 1);
                self.a.logic_rr(true, 2, T3, T3, T2, 0, 0);
            }
            fast::SHIFT_SHR => {
                self.a.shiftv(true, 9, T1, ua, T6);
                self.a.addsub_ri(true, true, false, T4, T6, 1, false);
                self.a.shiftv(true, 9, T2, ua, T4);
                self.a.ubfx(T2, T2, 0, 1);
                self.a.ubfx(T3, ua, w - 1, 1);
            }
            _ => {
                let sa = if w < 64 {
                    self.a.sbfm(true, XS, ua, 0, w - 1);
                    XS
                } else {
                    ua
                };
                self.a.shiftv(true, 10, T1, sa, T6);
                if w < 64 {
                    self.a.ubfm(true, T1, T1, 0, w - 1);
                }
                self.a.addsub_ri(true, true, false, T4, T6, 1, false);
                self.a.shiftv(true, 10, T2, sa, T4);
                self.a.ubfx(T2, T2, 0, 1);
                of_zero = true;
            }
        }
        self.shift_flags(la, w, of_zero);
        let d = g(op.r0);
        match w {
            64 => self.a.mov_rr(true, d, T1),
            32 => self.a.mov_rr(false, d, T1),
            _ => self.a.bfxil0(d, T1, w),
        }
        self.lb_store_now(op.r0, w as u8);
        if w == 32 {
            // Count 0: flags unchanged, but a 32-bit destination is still zero-extended.
            let done = self.a.b();
            self.a.bind(skip);
            self.a.mov_rr(false, d, d);
            self.lb_store_now(op.r0, 32);
            self.a.bind(done);
        } else {
            self.a.bind(skip);
        }
        self.fs.host = 0;
        self.fs.mem = (self.fs.mem & !FALL) | (la & FALL);
        self.fs.src = Src::None;
    }

    pub(super) fn cmov(&mut self, k: usize, op: &FastOp) {
        let w = op.w;
        let d = g(op.r0);
        let c = self.cond(op.x & 0x3F);
        let rm = op.kind == fast::K_CMOV_RM;
        if w == 32 {
            self.set_lb(op.r0, 32);
        } else if tracked(op.r0) {
            self.flush_lb_reg(op.r0 as usize);
        }
        if !rm {
            if let Cond::Host(cnd) = c {
                let s = g(op.r1);
                match w {
                    64 => self.a.csel(true, d, s, d, cnd),
                    32 => self.a.csel(false, d, s, d, cnd),
                    _ => {
                        self.a.csel(false, T0, s, d, cnd);
                        self.a.bfxil0(d, T0, 16);
                    }
                }
                if w != 32 && self.lb_on && tracked(op.r0) {
                    let f = self.a.bcond(cnd ^ 1);
                    self.lb_store_now(op.r0, w);
                    self.a.bind(f);
                }
                return;
            }
        }
        if w == 32 {
            // A false condition still zero-extends the destination.
            self.a.mov_rr(false, d, d);
        }
        let skip = self.branch_if_false(&c);
        if skip.is_none() || !matches!(c, Cond::Const(false)) {
            let src = if rm {
                self.load_op(k, T0, op, w as u32);
                T0
            } else {
                g(op.r1)
            };
            match w {
                64 => self.a.mov_rr(true, d, src),
                32 => self.a.mov_rr(false, d, src),
                _ => self.a.bfxil0(d, src, 16),
            }
            if w != 32 {
                self.lb_store_now(op.r0, w);
            }
        }
        if let Some(f) = skip {
            self.a.bind(f);
        }
    }

    /// Batch consecutive `push r` ops + optional dead-flags `sub rsp, imm` into a single
    /// `sub x23, x23, #total` followed by parallel fixed-offset `stp`/`str` stores.
    pub(super) fn try_batch_push(&mut self, k: usize) -> bool {
        use fast::*;
        let body = self.body;
        let mut m = 0usize;
        while k + m < body && self.ops[k + m].kind == K_PUSH_R && self.ops[k + m].r0 != 4 {
            m += 1;
        }
        let mut sub_imm = 0u64;
        let mut has_sub = false;
        if k + m < body {
            let s = self.ops[k + m];
            if s.kind == K_ALU_RI
                && s.r0 == 4
                && s.w == 64
                && (s.x & 0x3F) == A_SUB
                && s.imm > 0
                && s.imm % 8 == 0
                && s.imm <= 440
                && (self.live[k + m + 1] & FALL) == 0
            {
                sub_imm = s.imm;
                has_sub = true;
            }
        }
        let total = (m as u64) * 8 + sub_imm;
        if (m < 2 && !has_sub) || total > 504 {
            return false;
        }
        if m == 2 && !has_sub {
            self.a.stp_pre(g(self.ops[k + 1].r0), g(self.ops[k].r0), g(4), -16);
            self.skip = 1;
            return true;
        }
        self.a.addsub_ri(true, true, false, g(4), g(4), total as u32, false);
        let mut i = 0usize;
        while i + 1 < m {
            let off = (sub_imm as usize + (m - 2 - i) * 8) as i32;
            self.a.stp_x(g(self.ops[k + i + 1].r0), g(self.ops[k + i].r0), g(4), off);
            i += 2;
        }
        if i < m {
            self.a.str(64, g(self.ops[k + i].r0), g(4), sub_imm as u32);
        }
        if has_sub {
            self.fs.host = 0;
            self.fs.mem &= !FALL;
            self.fs.src = Src::None;
        }
        self.skip = m + usize::from(has_sub) - 1;
        true
    }

    /// Batch optional dead-flags `add rsp, imm` + consecutive distinct `pop r` ops (and when
    /// ending at a plain `Term::Ret`, hoist `ldr x0, [x23, #ret_off]` and `ldp x1, x2, [x7]`
    /// BEFORE the parallel `ldp`/`ldr` pops to break serial `x23` chains and hide load latency).
    pub(super) fn try_batch_epilogue(&mut self, k: usize) -> bool {
        use fast::*;
        if !pushpop_enabled() {
            return false;
        }
        let body = self.body;
        let mut p = k;
        let mut add_imm = 0u64;
        let mut has_add = false;
        if self.ops[p].kind == K_ALU_RI {
            let a = self.ops[p];
            if a.r0 != 4
                || a.w != 64
                || (a.x & 0x3F) != A_ADD
                || a.imm == 0
                || a.imm % 8 != 0
                || a.imm > 440
                || (self.live[p + 1] & FALL) != 0
            {
                return false;
            }
            add_imm = a.imm;
            has_add = true;
            p += 1;
        }
        let pop_start = p;
        let mut seen = 0u16;
        while p < body && self.ops[p].kind == K_POP_R && self.ops[p].r0 != 4 {
            let bit = 1u16 << self.ops[p].r0;
            if seen & bit != 0 {
                break;
            }
            seen |= bit;
            p += 1;
        }
        let m = p - pop_start;
        let ret_off = add_imm + (m as u64) * 8;
        let can_hoist_ret = p == body
            && self.term_is_plain_ret
            && !self.bud_dec
            && self.term_live_out == 0
            && !self.lb_on
            && ret_off <= 504;
        if !can_hoist_ret {
            if (m < 2 && !has_add) || m == 0 || ret_off > 504 {
                return false;
            }
            if m == 2 && !has_add {
                let r0 = self.ops[pop_start].r0;
                let r1 = self.ops[pop_start + 1].r0;
                self.a.ldp_post(g(r0), g(r1), g(4), 16);
                self.set_lb(r0, 64);
                self.set_lb(r1, 64);
                self.skip = 1;
                return true;
            }
        }
        if can_hoist_ret {
            // Flush any deferred x87 TOP state before preloading return address / RAS into T0..T2.
            self.flush_x87_top();
            // Start return-address + shadow-RAS loads FIRST so their latency overlaps pops.
            self.a.ldr(64, T0, g(4), ret_off as u32);
            if self.ras_miss != 0 {
                self.a.ldp_x(1, 2, XRAS, 0);
            }
            self.ret_preloaded = true;
        }
        let mut i = 0usize;
        while i + 1 < m {
            let r0 = self.ops[pop_start + i].r0;
            let r1 = self.ops[pop_start + i + 1].r0;
            let off = (add_imm as usize + i * 8) as i32;
            self.a.ldp_x(g(r0), g(r1), g(4), off);
            self.set_lb(r0, 64);
            self.set_lb(r1, 64);
            i += 2;
        }
        if i < m {
            let r0 = self.ops[pop_start + i].r0;
            self.a.ldr(64, g(r0), g(4), (add_imm as usize + i * 8) as u32);
            self.set_lb(r0, 64);
        }
        let total_adj = ret_off + if can_hoist_ret { 8 } else { 0 };
        self.a.addsub_ri(true, false, false, g(4), g(4), total_adj as u32, false);
        if has_add {
            self.fs.host = 0;
            self.fs.mem &= !FALL;
            self.fs.src = Src::None;
        }
        self.skip = (p - k) - 1;
        true
    }

    pub(super) fn terminator(&mut self, term: Term, k: usize) {
        let live_out = self.live[self.ops.len()];
        let op = self.ops[k];
        let fall = op.ip.wrapping_add(op.len as u64);
        match term {
            Term::Fall(x) => {
                self.flush_flags(live_out);
                if self.lazy {
                    self.flush_lb_edge(self.edge.kill[0]);
                } else {
                    self.flush_lb();
                }
                self.exit_to(x);
            }
            Term::Jcc { cc, target, fall, near } if self.lazy => self.jcc_lazy(cc, target, fall, near),
            Term::Jcc { cc, target, fall, near } => {
                self.flush_flags(live_out);
                let lbk = self.abi_lb();
                self.flush_lb();
                let c = self.cond(cc);
                let taken_host = near && self.target_host;
                if !taken_host {
                    match c {
                        Cond::Const(true) => {
                            self.exit_to(target);
                            return;
                        }
                        Cond::Const(false) => {
                            self.exit_to(fall);
                            return;
                        }
                        Cond::Host(cnd) => {
                            let f = self.a.bcond(cnd);
                            self.cond_links.push((f, target));
                            self.exit_to(fall);
                            return;
                        }
                        Cond::Reg(r, inv) => {
                            let f = if inv { self.a.cbz(false, r) } else { self.a.cbnz(false, r) };
                            self.cond_links.push((f, target));
                            self.exit_to(fall);
                            return;
                        }
                    }
                }
                let fix = match c {
                    Cond::Const(true) => None,
                    Cond::Const(false) => {
                        self.exit_to(fall);
                        return;
                    }
                    Cond::Host(cnd) => Some(self.a.bcond(cnd)),
                    Cond::Reg(r, inv) => Some(if inv { self.a.cbz(false, r) } else { self.a.cbnz(false, r) }),
                };
                if let Some(f) = fix {
                    self.exit_to(fall);
                    self.a.bind(f);
                }
                self.emit_tail_target(target, fall, lbk, EXIT_JCC);
            }
            Term::JccPair { cc1, t1, cc2, t2, fall2 } => {
                self.flush_flags(live_out);
                self.flush_lb();
                if self.fs.src == Src::Fcmp && (self.fs.host & (FP | FZ)) == (FP | FZ) {
                    if cc1 == 11 {
                        // `jp t1; je/jne t2; fall2`: V==1 -> t1; else V==0 so ZF == Z.
                        let f1 = self.a.bcond(VS);
                        self.cond_links.push((f1, t1));
                        let f2 = self.a.bcond(if cc2 == 5 { EQ } else { NE });
                        self.cond_links.push((f2, t2));
                        self.exit_to(fall2);
                    } else {
                        // `jnp t1; je/jne t2; fall2`: V==0 -> t1; else V==1 (unordered, ZF==1).
                        let f1 = self.a.bcond(VC);
                        self.cond_links.push((f1, t1));
                        self.exit_to(if cc2 == 5 { t2 } else { fall2 });
                    }
                } else {
                    // Fallback if flags were materialized before the terminator.
                    let c1 = self.cond(cc1);
                    match c1 {
                        Cond::Const(true) => {
                            self.exit_to(t1);
                            return;
                        }
                        Cond::Const(false) => {}
                        Cond::Host(cnd) => {
                            let f = self.a.bcond(cnd);
                            self.cond_links.push((f, t1));
                        }
                        Cond::Reg(r, inv) => {
                            let f = if inv { self.a.cbz(false, r) } else { self.a.cbnz(false, r) };
                            self.cond_links.push((f, t1));
                        }
                    }
                    let c2 = self.cond(cc2);
                    match c2 {
                        Cond::Const(true) => self.exit_to(t2),
                        Cond::Const(false) => self.exit_to(fall2),
                        Cond::Host(cnd) => {
                            let f = self.a.bcond(cnd);
                            self.cond_links.push((f, t2));
                            self.exit_to(fall2);
                        }
                        Cond::Reg(r, inv) => {
                            let f = if inv { self.a.cbz(false, r) } else { self.a.cbnz(false, r) };
                            self.cond_links.push((f, t2));
                            self.exit_to(fall2);
                        }
                    }
                }
            }
            Term::Jmp(t) => {
                self.flush_flags(live_out);
                let lbk = self.abi_lb();
                if self.lazy && !self.target_host {
                    self.flush_lb_edge(self.edge.kill[0]);
                } else {
                    self.flush_lb();
                }
                if self.target_host {
                    self.emit_tail_target(t, fall, lbk, EXIT_JMP);
                } else {
                    self.exit_to(t);
                }
            }
            Term::Call(t) => {
                self.flush_flags(live_out);
                let lbk = self.abi_lb();
                if self.lazy && !self.target_host {
                    self.flush_lb_edge(self.edge.kill[0]);
                } else {
                    self.flush_lb();
                }
                if self.target_host {
                    if let Some((h, bail, needs_xmm)) = inline_host_fn_kind(t) {
                        self.plain_call(h, t, lbk, false, bail.then_some((t, fall)), 0, needs_xmm);
                        self.exit_to(fall);
                    } else if let Some((slot, v, h, bail, needs_xmm)) = iat_slot_plain(t) {
                        self.mov_lit(XA, slot);
                        self.a.ldr(64, T0, XA, 0);
                        self.mov_slot_val(XS, v, slot);
                        self.a.addsub_rr(true, true, true, ZR, T0, XS, 0); // cmp x0, x17
                        let slow = self.a.bcond(NE);
                        self.plain_call(h, v, lbk, false, bail.then_some((v, fall)), slot, needs_xmm);
                        self.exit_to(fall);
                        self.a.bind(slow);
                        if self.host_xfer != 0 {
                            self.xfer_to(t, fall, 0);
                        } else {
                            self.exit_code(EXIT_CALL, fall, t);
                        }
                    } else if !plaincall_enabled()
                        && self.host_xfer != 0
                        && t != 0
                        && !is_chkstk(t)
                        && !crate::emu::is_routed_dll_chkstk(t)
                        && !crate::emu::is_routed_dll_stack_pos(t)
                        && !crate::emu::is_routed_regfile(t)
                        && !crate::emu::is_routed_dll_stub(t)
                    {
                        self.mov_lit(T0, t);
                        self.call_lookup(fall);
                    } else if self.host_xfer != 0 {
                        self.xfer_to(t, fall, 0);
                    } else {
                        self.exit_code(EXIT_CALL, fall, t);
                    }
                } else {
                    if pushpop_enabled() {
                        // Plain pre-indexed return-address push (see `pushpop_enabled`).
                        self.mov_lit(T0, fall);
                        self.a.str_pre(T0, g(4), -8);
                    } else {
                        self.a.addsub_ri(true, true, false, XA, g(4), 8, false);
                        self.mov_lit(T0, fall);
                        self.store(k, T0, XA, 64, Chk::Stack);
                        self.a.mov_rr(true, g(4), XA);
                    }
                    if self.ras_miss != 0 {
                        // RAS entry (fall, landing pad), then `bl` into the callee (the
                        // landing pad is the word after the `bl`, so the CPU's return
                        // predictor agrees with the RAS).
                        let adr = self.a.adr(T1);
                        ras_push(&mut self.a, T0, T1);
                        let site = self.a.bl();
                        self.links.push((site, t));
                        let land = self.a.pos;
                        self.a.patch_adr(adr, land);
                    }
                    self.exit_to(if self.ras_miss != 0 { fall } else { t });
                }
            }
            Term::CallR | Term::JmpR => {
                self.flush_flags(live_out);
                self.flush_lb();
                self.a.mov_rr(true, T0, g(op.r0));
                if matches!(term, Term::CallR) {
                    self.call_ic_or_lookup(op.ip, fall);
                } else {
                    self.jmp_ic_or_lookup(op.ip);
                }
            }
            Term::CallM | Term::JmpM => {
                self.flush_flags(live_out);
                let lbk = self.abi_lb();
                self.flush_lb();
                let is_call = matches!(term, Term::CallM);
                // `call/jmp [rip+disp]` through an import slot holding an inline-callable target:
                // inline call guarded by the slot still holding the translation-time value.
                let mut generic = None;
                if let Some((slot, v, h, bail, needs_xmm)) = const_slot_plain(&op) {
                    self.mov_lit(XA, slot);
                    self.a.ldr(64, T0, XA, 0);
                    self.mov_slot_val(XS, v, slot);
                    self.a.addsub_rr(true, true, true, ZR, T0, XS, 0); // cmp x0, x17
                    generic = Some(self.a.bcond(NE));
                    self.plain_call(h, v, lbk, !is_call, bail.then_some((v, fall)), slot, needs_xmm);
                    if is_call {
                        self.exit_to(fall);
                    }
                    self.a.bind(generic.unwrap());
                }
                if generic.is_none() {
                    self.load_op(k, T0, &op, 64);
                    if is_call {
                        self.call_ic_or_lookup(op.ip, fall);
                    } else {
                        self.jmp_ic_or_lookup(op.ip);
                    }
                } else if is_call {
                    self.call_lookup(fall);
                } else {
                    self.a.b_to(self.lookup_jmp);
                }
            }
            Term::Ret => {
                self.flush_flags(live_out);
                self.flush_lb();
                if !self.ret_preloaded {
                    if op.imm == 0 {
                        self.a.ldr_post(T0, g(4), 8);
                    } else {
                        self.a.ldr(64, T0, g(4), 0);
                        self.a.add_imm(true, g(4), g(4), 8 + op.imm as i64);
                    }
                }
                if self.ras_miss != 0 {
                    // Pop pinned `XRAS` top if its guest address matches the return target and
                    // return to its landing pad; otherwise `ras_miss[_ic]` (x0 = target, XRAS = top).
                    if !self.ret_preloaded {
                        self.a.ldp_x(1, 2, XRAS, 0);
                    }
                    self.a.addsub_ri(true, true, false, XA, XRAS, 16, false);
                    self.a.logic_rr(true, 2, 1, 1, T0, 0, 0); // eor x1, x1, x0
                    let miss = self.a.cbnz(true, 1);
                    self.a.bfxil0(XRAS, XA, 16);
                    self.a.ret_r(2);
                    if ic_enabled() && self.ras_miss_ic != 0 {
                        self.ic_colds.push(super::translate::IcCold::RetRasMiss {
                            miss_fix: miss,
                            slot_byte_off: super::cache::ic_slot_byte_off(op.ip),
                        });
                    } else {
                        self.a.bind(miss);
                        self.a.b_to(self.ras_miss);
                    }
                } else {
                    self.ret_no_ras_ic_or_lookup(op.ip);
                }
            }
            Term::Loop { sub, target, fall } => {
                self.flush_flags(live_out | if sub >= 3 { FZ } else { 0 });
                if sub >= 2 {
                    self.set_lb(1, 64);
                }
                self.flush_lb();
                if sub <= 1 {
                    let taken = self.a.cbz(sub == 0, g(1));
                    self.exit_to(fall);
                    self.a.bind(taken);
                    self.exit_to(target);
                } else {
                    self.a.addsub_ri(true, true, false, g(1), g(1), 1, false);
                    let zero = self.a.cbz(true, g(1));
                    let zf_fail = if sub == 3 {
                        self.a.ldr(8, T0, XCPU, OFF_ZF);
                        Some(self.a.cbz(false, T0))
                    } else if sub == 4 {
                        self.a.ldr(8, T0, XCPU, OFF_ZF);
                        Some(self.a.cbnz(false, T0))
                    } else {
                        None
                    };
                    self.exit_to(target);
                    self.a.bind(zero);
                    if let Some(f) = zf_fail {
                        self.a.bind(f);
                    }
                    self.exit_to(fall);
                }
            }
        }
    }
}
