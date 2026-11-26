//! Native ARM64/NEON code generation for:
//! - `K_VEXT_RR` / `K_VEXT_RM`: `(v)phaddsw`, `(v)phsubsw`, `(v)phminposuw`, `(v)mpsadbw`, `(v)dppd`
//! - `K_MASKMOV_B`: `(v)maskmovdqu`, `maskmovq`
//! - `K_VGATHER`: `vpgatherdd/dq/qd/qq`, `vgatherdps/dpd/qps/qpd` (128 & 256)
//! - `K_MMX_VSHUF_*`, `K_MMX_VINT_*`, `K_MMX_MISC`: all 89 `MMX_ALIASED` mnemonics directly
//!   aliased onto `Cpu::jx.x87_state` (`X87State`).

use super::emit::enc_logic_imm;
use super::translate::{Chk, Tr};
use super::xmm::xh;
use super::*;
use crate::emu::fast::{self, FastOp};
use crate::emu::mem;

impl Tr<'_> {
    /// Load `Cpu::jx.x87_state` into `XA` and invalidate the `XA` address cache.
    fn load_x87_ptr_into_xa(&mut self) {
        self.xa_key = None;
        self.a.ldr(64, XA, XCPU, OFF_X87_STATE);
    }

    /// Load 64-bit MMX register `mm` (`0..7`) from `X87State::regs[mm][0..8]` into `d(vd)`
    /// (zero-extending bits `[127:64]` of `v(vd)`). Clobbers `XA` and invalidates `self.xa_key`.
    fn load_mmx_d(&mut self, vd: u32, mm: u8) {
        self.load_x87_ptr_into_xa();
        self.a.ldur_stur_v(true, 64, vd, XA, X87_OFF_REGS + (mm as i32 & 7) * 10);
    }

    /// Transition x87 state to MMX mode (`top = 0`, `valid_mask = 0xFF`). Clobbers `XA`, `XS`
    /// and invalidates `self.xa_key`.
    fn emit_mmx_tag_valid(&mut self) {
        self.load_x87_ptr_into_xa();
        self.a.str(8, ZR, XA, X87_OFF_TOP);
        self.a.movz(false, XS, 0xFF, 0);
        self.a.str(8, XS, XA, X87_OFF_VALID_MASK);
    }

    /// Store `d(vd)` into `X87State::regs[mm][0..8]`, fill exponent bytes `[8..10]` with `0xFFFF`,
    /// and set `top = 0`, `valid_mask = 0xFF`. Clobbers `XA`, `XS` and invalidates `self.xa_key`.
    fn store_mmx_d_and_tag(&mut self, vd: u32, mm: u8) {
        let off = X87_OFF_REGS + (mm as i32 & 7) * 10;
        self.load_x87_ptr_into_xa();
        self.a.ldur_stur_v(false, 64, vd, XA, off);
        self.a.movz(false, XS, 0xFFFF, 0);
        self.a.ldur_stur(false, 16, XS, XA, off + 8);
        self.a.str(8, ZR, XA, X87_OFF_TOP);
        self.a.movz(false, XS, 0xFF, 0);
        self.a.str(8, XS, XA, X87_OFF_VALID_MASK);
    }

    /// Emit one 128-bit lane of `K_VEXT_*` on `(v0 = a, v1 = b) -> v0`. Preserves `v6`, `v7`, NZCV.
    fn emit_vext_lane(&mut self, sub: u8, imm8: u8) {
        use fast::*;
        match sub {
            VX_PHADDSW => {
                self.a.v_uzp1(1, 2, 0, 1);
                self.a.v_uzp2(1, 3, 0, 1);
                self.a.neon_3same(false, 1, 1, 0, 2, 3); // sqadd v0.8h, v2.8h, v3.8h
            }
            VX_PHSUBSW => {
                self.a.v_uzp1(1, 2, 0, 1);
                self.a.v_uzp2(1, 3, 0, 1);
                self.a.neon_3same(false, 1, 5, 0, 2, 3); // sqsub v0.8h, v2.8h, v3.8h
            }
            VX_PHMINPOSUW => {
                // Pack each lane as (val_u16 << 16) | idx_u16 so unsigned u32 min picks
                // minimum value and breaks ties by lowest index.
                self.a.v_xtl(true, 1, false, 2, 1); // uxtl v2.4s, v1.4h
                self.a.v_xtl(true, 1, true, 3, 1); // uxtl2 v3.4s, v1.8h
                self.a.v_shl_i(32, 2, 2, 16);
                self.a.v_shl_i(32, 3, 3, 16);
                self.mov_lit128(4, 0x0000_0001_0000_0000, 0x0000_0003_0000_0002);
                self.a.v_orr(2, 2, 4);
                self.mov_lit128(5, 0x0000_0005_0000_0004, 0x0000_0007_0000_0006);
                self.a.v_orr(3, 3, 5);
                self.a.neon_3same(true, 2, 13, 2, 2, 3); // umin v2.4s, v2.4s, v3.4s
                self.a.v_uminv_s(2, 2); // uminv s2, v2.4s
                self.a.fmov_fg(false, T0, 2);
                self.a.ror_i(false, T0, T0, 16); // (min_idx << 16) | min_val
                self.a.fmov_gf(false, 0, T0); // clears upper 96 bits
            }
            VX_MPSADBW => {
                let a_off = (((imm8 >> 2) & 1) * 4) as u32;
                let b_off = ((imm8 & 3) * 4) as u32;
                for n in 0..4u32 {
                    let shift = a_off + n;
                    let va = if shift == 0 {
                        0
                    } else {
                        self.a.v_ext(2, 0, 0, shift);
                        2
                    };
                    self.a.v_dup_lane_b(3, 1, b_off + n);
                    if n == 0 {
                        self.a.v_uabdl_b(false, 4, va, 3);
                    } else {
                        self.a.v_uabdl_b(false, 5, va, 3);
                        self.a.neon_3same(false, 1, 16, 4, 4, 5); // add v4.8h, v4.8h, v5.8h
                    }
                }
                self.a.v_mov(0, 4);
            }
            _ => {
                // VX_DPPD: round each active product, then add (prod0 + prod1) with exact x86
                // NaN propagation (masked-off lanes contribute +0.0 without touching NaN/Inf).
                match imm8 & 0x30 {
                    0x00 => {
                        self.a.v_zero(0);
                        return;
                    }
                    0x10 => {
                        self.a.fmov_s(true, 0, 0);
                        self.a.fmov_s(true, 1, 1);
                        self.a.v_farith(true, 2, 2, 0, 1);
                        self.emit_vnan_check_fix(true, false);
                        self.a.fmov_s(true, 0, 2);
                        self.a.v_zero(1);
                        self.a.v_farith(true, 0, 2, 0, 1);
                        self.emit_vnan_check_fix(true, false);
                    }
                    0x20 => {
                        self.a.v_ext(0, 0, 0, 8);
                        self.a.fmov_s(true, 0, 0);
                        self.a.v_ext(1, 1, 1, 8);
                        self.a.fmov_s(true, 1, 1);
                        self.a.v_farith(true, 2, 2, 0, 1);
                        self.emit_vnan_check_fix(true, false);
                        self.a.v_zero(0);
                        self.a.fmov_s(true, 1, 2);
                        self.a.v_farith(true, 0, 2, 0, 1);
                        self.emit_vnan_check_fix(true, false);
                    }
                    _ => {
                        self.a.v_farith(true, 2, 2, 0, 1);
                        self.emit_vnan_check_fix(true, false);
                        self.a.fmov_s(true, 0, 2);
                        self.a.v_ext(1, 2, 2, 8);
                        self.a.fmov_s(true, 1, 1);
                        self.a.v_farith(true, 0, 2, 0, 1);
                        self.emit_vnan_check_fix(true, false);
                    }
                }
                match imm8 & 3 {
                    0 => self.a.v_zero(0),
                    1 => self.a.fmov_s(true, 0, 2),
                    2 => {
                        self.a.v_zero(0);
                        self.a.v_ins_lane(3, 0, 1, 2, 0);
                    }
                    _ => self.a.v_zip1(3, 0, 2, 2),
                }
            }
        }
    }

    /// Emit an 8-byte byte-masked store loop from `T1` (data) & `T2` (mask) to `[T5]`,
    /// advancing `T5` by 8 bytes when `advance_on_skip` is true. Preserves `v0`, `v1`, NZCV.
    fn emit_maskmov_8bytes(&mut self, k: usize, advance_on_skip: bool) {
        self.a.mov_imm(true, T0, 0x8080_8080_8080_8080);
        self.a.logic_rr(true, 0, T0, T2, T0, 0, 0);
        let skip_chunk = self.a.cbz(true, T0);
        self.a.movz(false, T3, 8, 0);
        let lp = self.a.pos;
        let skip_b = self.a.tbz(T2, 7);
        self.store(k, T1, T5, 8, Chk::Full);
        self.a.bind(skip_b);
        self.a.lsr_i(true, T1, T1, 8);
        self.a.lsr_i(true, T2, T2, 8);
        self.a.addsub_ri(true, false, false, T5, T5, 1, false);
        self.a.addsub_ri(false, true, false, T3, T3, 1, false);
        let back = self.a.cbnz(false, T3);
        self.a.patch(back, lp);
        if advance_on_skip {
            let done = self.a.b();
            self.a.bind(skip_chunk);
            self.a.addsub_ri(true, false, false, T5, T5, 8, false);
            self.a.bind(done);
        } else {
            self.a.bind(skip_chunk);
        }
    }

    /// Emit `K_VGATHER` (`vpgatherdd/dq/qd/qq`, `vgatherdps/dpd/qps/qpd`, 128 & 256) inline
    /// with element-by-element mask clearing (restartable on mid-gather fault), `#UD` overlap
    /// check, VSIB scale/disp/`a32`/`%gs:`, 32-bit index sign extension, and `vm64x`/VEX.128
    /// upper zeroing.
    fn emit_vgather_op(&mut self, k: usize, op: &FastOp) {
        let d = op.r0 & 15;
        let dst_ymm = (op.r0 & 0x10) != 0;
        let idx_ymm = (op.r0 & 0x20) != 0;
        let msk_ymm = (op.r0 & 0x40) != 0;
        let m = op.r1 & 15;
        let v = (op.r1 >> 4) & 15;
        if d == m || d == v || m == v {
            // Intel SDM: #UD if any two of dest, mask, vindex alias the same register.
            let se = self.side_exit_here(k);
            let b = self.a.b();
            self.bind_side_exit(se, b);
            return;
        }
        let elem64 = (op.x & 1) != 0;
        let idx64 = (op.x & 2) != 0;
        let scale_sh = (op.x >> 6) as u32;
        let elem_bits: u32 = if elem64 { 64 } else { 32 };
        let elem_bytes: u32 = elem_bits / 8;
        let elem_lg: u32 = if elem64 { 3 } else { 2 };
        let idx_lg: u32 = if idx64 { 3 } else { 2 };
        let count: usize = match (elem64, idx64) {
            (false, false) => if dst_ymm { 8 } else { 4 },
            (true, false) => if dst_ymm { 4 } else { 2 },
            (false, true) => if idx_ymm { 4 } else { 2 },
            (true, true) => if dst_ymm { 4 } else { 2 },
        };
        let per_lane: usize = if elem64 { 2 } else { 4 };
        let idx_per_lane: usize = if idx64 { 2 } else { 4 };

        // Load vector index into v2 (lo) and v3 (hi if idx_ymm).
        self.xsrc(2, v);
        if idx_ymm {
            self.a.ldr(128, 3, XCPU, OFF_YMM_HI + (v as u32) * 16);
        }

        // Precompute T5 = base + disp (32-bit wrapped if B_ADDR32; GS added per lane after wrap).
        let b_reg = op.base & 0x1F;
        let is_a32 = (op.base & fast::B_ADDR32) != 0;
        let is_gs = (op.base & fast::B_GS) != 0;
        let x64 = !is_a32;
        let disp = if is_a32 { op.disp as i32 as i64 } else { op.disp };
        if b_reg < 16 {
            if disp == 0 {
                self.a.mov_rr(x64, T5, g(b_reg));
            } else {
                self.a.add_imm(x64, T5, g(b_reg), disp);
            }
        } else {
            self.a.mov_imm(x64, T5, if is_a32 { (disp as u32) as u64 } else { disp as u64 });
        }

        // Conservatively mark dst YMM dirty if this gather can write to `Cpu::ymm_hi[d]` so
        // partial faults or normal completion never leave `ymm_dirty` stale.
        self.xa_key = None;
        self.ymm_clean &= !(1u16 << d);
        self.ymm_clean &= !(1u16 << m);
        if dst_ymm {
            let bit = 1u16 << d;
            self.a.ldr(16, XS, XCPU, OFF_YMM_DIRTY);
            self.a.logic_ri(false, 1, XS, XS, enc_logic_imm(bit as u64, false).unwrap());
            self.a.str(16, XS, XCPU, OFF_YMM_DIRTY);
            self.a.logic_ri(true, 1, XBUD, XBUD, enc_logic_imm(1, true).unwrap());
        }

        let d_xmm_off = OFF_XMM + (d as u32) * 16;
        let m_xmm_off = OFF_XMM + (m as u32) * 16;
        let d_ymm_off = OFF_YMM_HI + (d as u32) * 16;
        let m_ymm_off = OFF_YMM_HI + (m as u32) * 16;

        for i in 0..count {
            let half = i / per_lane;
            let lane = (i % per_lane) as u32;
            let lane_byte_off = lane * elem_bytes;

            // Read mask element i into T1 and test its MSB.
            if half == 0 {
                if self.xpin {
                    self.a.v_umov_elem(elem_lg, T1, xh(m), lane);
                } else {
                    self.a.ldr(elem_bits, T1, XCPU, m_xmm_off + lane_byte_off);
                }
            } else {
                debug_assert!(msk_ymm);
                self.a.ldr(elem_bits, T1, XCPU, m_ymm_off + lane_byte_off);
            }
            let skip_load = self.a.tbz(T1, elem_bits - 1);

            // Extract index i (sign-extending 32-bit index to 64-bit) and compute EA in XA.
            let idx_half = (i / idx_per_lane) as u32;
            let idx_lane = (i % idx_per_lane) as u32;
            self.a.v_umov_elem(idx_lg, T1, 2 + idx_half, idx_lane);
            if !idx64 {
                self.a.sbfm(true, T1, T1, 0, 31);
            }
            self.a.addsub_rr(x64, false, false, XA, T5, T1, scale_sh);
            if is_gs {
                self.a.ldr(64, XS, XCPU, OFF_TEB);
                self.a.addsub_rr(true, false, false, XA, XA, XS, 0);
            }

            // Load element (side-exits with lanes 0..i committed & masked off if range fails).
            self.load(k, T0, XA, elem_bits, Chk::Full);

            // Commit loaded element into dest lane i immediately.
            if half == 0 {
                if self.xpin {
                    self.a.v_ins_elem(elem_lg, xh(d), lane, T0);
                } else {
                    self.a.str(elem_bits, T0, XCPU, d_xmm_off + lane_byte_off);
                }
            } else {
                self.a.str(elem_bits, T0, XCPU, d_ymm_off + lane_byte_off);
            }

            self.a.bind(skip_load);

            // Clear mask lane i so any subsequent faulting lane leaves a restartable mask.
            if half == 0 {
                if self.xpin {
                    self.a.v_ins_elem(elem_lg, xh(m), lane, ZR);
                } else {
                    self.a.str(elem_bits, ZR, XCPU, m_xmm_off + lane_byte_off);
                }
            } else {
                self.a.str(elem_bits, ZR, XCPU, m_ymm_off + lane_byte_off);
            }
        }

        // VEX.128 vgatherqps/vpgatherqd (vm64x) zeroes dest[127:64] upon normal completion.
        if !elem64 && idx64 && !idx_ymm {
            if self.xpin {
                self.a.v_ins_d(xh(d), 1, ZR);
            } else {
                self.a.str(64, ZR, XCPU, d_xmm_off + 8);
            }
        }
        self.x_clobber_hi(d);
        if !dst_ymm {
            self.clear_ymm_hi(d);
        }
        // Normal completion zeroes the entire mask YMM register.
        self.a.v_zero(0);
        self.xwrite(m, 0);
        self.clear_ymm_hi(m);
    }

    /// Emit `K_VEXT_RR..=K_MMX_MISC` natively to ARM64/NEON.
    pub(super) fn emit_mmx_gather_op(&mut self, k: usize, op: &FastOp) {
        use fast::*;
        match op.kind {
            K_VEXT_RR | K_VEXT_RM => {
                let sub = op.x & 0x3F;
                let imm8 = op.imm as u8;
                let s1 = ((op.imm >> 8) as u32 & 15) * 16;
                let is_256 = (op.w & 3) == VM_V256;
                self.vload_b(k, op);
                self.xsrc(0, (op.imm >> 8) as u8);
                self.emit_vext_lane(sub, imm8);
                if is_256 {
                    self.a.v_mov(6, 0);
                    self.a.ldr(128, 0, XCPU, OFF_YMM_HI + s1);
                    self.a.v_mov(1, 7);
                    let hi_imm = if sub == VX_MPSADBW { imm8 >> 3 } else { imm8 };
                    self.emit_vext_lane(sub, hi_imm);
                }
                self.vstore_dst(op);
            }
            K_MASKMOV_B => {
                let is_mmx = (op.x & 1) != 0;
                let is_a32 = (op.base & B_ADDR32) != 0;
                let is_gs = (op.base & B_GS) != 0;
                self.a.mov_rr(!is_a32, T5, g(7));
                if is_gs {
                    self.a.ldr(64, XS, XCPU, OFF_TEB);
                    self.a.addsub_rr(true, false, false, T5, T5, XS, 0);
                }
                if is_mmx {
                    self.load_x87_ptr_into_xa();
                    self.a.ldur_stur(true, 64, T1, XA, X87_OFF_REGS + (op.r0 as i32 & 7) * 10);
                    self.a.ldur_stur(true, 64, T2, XA, X87_OFF_REGS + (op.r1 as i32 & 7) * 10);
                    self.emit_maskmov_8bytes(k, false);
                    self.emit_mmx_tag_valid();
                } else {
                    self.xsrc(0, op.r0);
                    self.xsrc(1, op.r1);
                    self.a.fmov_fg(true, T1, 0);
                    self.a.fmov_fg(true, T2, 1);
                    self.emit_maskmov_8bytes(k, true);
                    self.a.v_umov_d(T1, 0, 1);
                    self.a.v_umov_d(T2, 1, 1);
                    self.emit_maskmov_8bytes(k, false);
                }
            }
            K_VGATHER => self.emit_vgather_op(k, op),
            K_MMX_VSHUF_RR | K_MMX_VSHUF_RM => {
                let sub = op.x & 0x3F;
                let imm8 = op.imm as u8;
                if op.kind == K_MMX_VSHUF_RM {
                    let bits = if op.w == 32 { 32 } else { 64 };
                    self.load_op(k, T0, op, bits);
                    self.a.fmov_gf(bits == 64, 1, T0);
                } else {
                    self.load_mmx_d(1, op.r1);
                }
                self.load_mmx_d(0, op.r0);
                match sub {
                    VS_PACKSSWB => {
                        self.a.v_zip1(3, 0, 0, 1);
                        self.a.v_sqxtn(false, 0, 0, 0);
                    }
                    VS_PACKUSWB => {
                        self.a.v_zip1(3, 0, 0, 1);
                        self.a.v_sqxtun(false, 0, 0, 0);
                    }
                    VS_PACKSSDW => {
                        self.a.v_zip1(3, 0, 0, 1);
                        self.a.v_sqxtn(false, 1, 0, 0);
                    }
                    VS_UNPCKHBW | VS_UNPCKHWD | VS_UNPCKHDQ => {
                        let sz = (sub - VS_UNPCKHBW) as u32;
                        self.a.v_ext(0, 0, 0, 4);
                        self.a.v_ext(1, 1, 1, 4);
                        self.a.v_zip1(sz, 0, 0, 1);
                    }
                    VS_PSHUFB => {
                        // 64-bit MMX pshufb: low 3 bits index bytes 0..7; bit 7 zeroes the lane.
                        self.a.mov_imm(true, T0, 0x8787_8787_8787_8787u64);
                        self.a.v_dup(3, 2, T0);
                        self.a.v_and(2, 1, 2);
                        self.a.v_tbl1(0, 0, 2);
                    }
                    VS_PALIGNR => {
                        let ic = imm8 as u32;
                        if ic == 0 {
                            self.a.v_mov(0, 1);
                        } else if ic < 16 {
                            self.a.v_zip1(3, 2, 1, 0); // [b, a]
                            self.a.v_zero(3);
                            self.a.v_ext(0, 2, 3, ic);
                        } else {
                            self.a.v_zero(0);
                        }
                    }
                    _ => {
                        // All other MMX logic/unpack-low/pshufw/shift sub-ops match 128-bit NEON
                        // bit-for-bit on the low 64 bits.
                        self.emit_vshuf_lane(sub, imm8);
                    }
                }
                self.store_mmx_d_and_tag(0, op.r0);
            }
            K_MMX_VINT_RR | K_MMX_VINT_RM => {
                let sub = op.x & 0x3F;
                if op.kind == K_MMX_VINT_RM {
                    self.load_op(k, T0, op, 64);
                    self.a.fmov_gf(true, 1, T0);
                } else {
                    self.load_mmx_d(1, op.r1);
                }
                self.load_mmx_d(0, op.r0);
                match sub {
                    VI_PHADDW => {
                        self.a.v_zip1(3, 2, 0, 1);
                        self.a.neon_3same(false, 1, 23, 0, 2, 2); // addp v0.8h, v2.8h, v2.8h
                    }
                    VI_PHADDD => {
                        self.a.v_zip1(3, 2, 0, 1);
                        self.a.neon_3same(false, 2, 23, 0, 2, 2); // addp v0.4s, v2.4s, v2.4s
                    }
                    VI_PHSUBW => {
                        self.a.v_zip1(3, 2, 0, 1);
                        self.a.v_uzp1(1, 3, 2, 2);
                        self.a.v_uzp2(1, 4, 2, 2);
                        self.a.neon_3same(true, 1, 16, 0, 3, 4); // sub v0.8h, v3.8h, v4.8h
                    }
                    VI_PHSUBD => {
                        self.a.v_zip1(3, 2, 0, 1);
                        self.a.v_uzp1(2, 3, 2, 2);
                        self.a.v_uzp2(2, 4, 2, 2);
                        self.a.neon_3same(true, 2, 16, 0, 3, 4); // sub v0.4s, v3.4s, v4.4s
                    }
                    MMX_VI_PHADDSW => {
                        self.a.v_zip1(3, 2, 0, 1);
                        self.a.v_uzp1(1, 3, 2, 2);
                        self.a.v_uzp2(1, 4, 2, 2);
                        self.a.neon_3same(false, 1, 1, 0, 3, 4); // sqadd v0.8h, v3.8h, v4.8h
                    }
                    MMX_VI_PHSUBSW => {
                        self.a.v_zip1(3, 2, 0, 1);
                        self.a.v_uzp1(1, 3, 2, 2);
                        self.a.v_uzp2(1, 4, 2, 2);
                        self.a.neon_3same(false, 1, 5, 0, 3, 4); // sqsub v0.8h, v3.8h, v4.8h
                    }
                    _ => {
                        // All other MMX integer ops are lane-wise identical to 128-bit NEON on
                        // the low 64 bits.
                        self.emit_vint_lane(sub, 0);
                    }
                }
                self.store_mmx_d_and_tag(0, op.r0);
            }
            K_MMX_MISC => match op.x & 0x3F {
                MMX_EMMS => {
                    self.load_x87_ptr_into_xa();
                    self.a.str(8, ZR, XA, X87_OFF_VALID_MASK);
                }
                MMX_MOVD_MG => {
                    self.a.fmov_gf(false, 0, g(op.r1));
                    self.store_mmx_d_and_tag(0, op.r0);
                }
                MMX_MOVD_MM => {
                    self.load_op(k, T0, op, 32);
                    self.a.fmov_gf(false, 0, T0);
                    self.store_mmx_d_and_tag(0, op.r0);
                }
                MMX_MOVD_GM => {
                    self.load_x87_ptr_into_xa();
                    self.a.ldur_stur(true, 32, T0, XA, X87_OFF_REGS + (op.r1 as i32 & 7) * 10);
                    self.wr_gpr(op.r0, 32, T0);
                    self.emit_mmx_tag_valid();
                }
                MMX_MOVD_MEM_M => {
                    self.load_x87_ptr_into_xa();
                    self.a.ldur_stur(true, 32, T0, XA, X87_OFF_REGS + (op.r1 as i32 & 7) * 10);
                    self.store_op(k, T0, op, 32);
                    self.emit_mmx_tag_valid();
                }
                MMX_MOVQ_MR => {
                    self.load_mmx_d(0, op.r1);
                    self.store_mmx_d_and_tag(0, op.r0);
                }
                MMX_MOVQ_MG => {
                    self.a.fmov_gf(true, 0, g(op.r1));
                    self.store_mmx_d_and_tag(0, op.r0);
                }
                MMX_MOVQ_MM => {
                    self.load_op(k, T0, op, 64);
                    self.a.fmov_gf(true, 0, T0);
                    self.store_mmx_d_and_tag(0, op.r0);
                }
                MMX_MOVQ_GM => {
                    self.load_x87_ptr_into_xa();
                    self.a.ldur_stur(true, 64, T0, XA, X87_OFF_REGS + (op.r1 as i32 & 7) * 10);
                    self.wr_gpr(op.r0, 64, T0);
                    self.emit_mmx_tag_valid();
                }
                MMX_MOVQ_MEM_M => {
                    self.load_x87_ptr_into_xa();
                    self.a.ldur_stur(true, 64, T0, XA, X87_OFF_REGS + (op.r1 as i32 & 7) * 10);
                    self.store_op(k, T0, op, 64);
                    self.emit_mmx_tag_valid();
                }
                MMX_MOVDQ2Q => {
                    self.xgpr(64, T0, op.r1);
                    self.a.fmov_gf(true, 0, T0);
                    self.store_mmx_d_and_tag(0, op.r0);
                }
                MMX_MOVQ2DQ => {
                    self.load_mmx_d(0, op.r1);
                    self.xwrite(op.r0, 0);
                    self.emit_mmx_tag_valid();
                }
                MMX_PINSRW_R => {
                    self.load_mmx_d(0, op.r0);
                    self.a.v_ins_elem(1, 0, (op.imm as u32) & 3, g(op.r1));
                    self.store_mmx_d_and_tag(0, op.r0);
                }
                MMX_PINSRW_M => {
                    self.load_op(k, T0, op, 16);
                    self.load_mmx_d(0, op.r0);
                    self.a.v_ins_elem(1, 0, (op.imm as u32) & 3, T0);
                    self.store_mmx_d_and_tag(0, op.r0);
                }
                MMX_PEXTRW => {
                    self.load_mmx_d(0, op.r1);
                    self.a.v_umov_elem(1, T0, 0, (op.imm as u32) & 3);
                    self.wr_gpr(op.r0, op.w, T0);
                    self.emit_mmx_tag_valid();
                }
                MMX_PMOVMSKB => {
                    self.load_mmx_d(0, op.r1);
                    self.a.v_shr_i(true, 8, 1, 0, 7);
                    self.mov_lit128(2, 0x8040_2010_0804_0201, 0);
                    self.a.neon_3same(false, 0, 19, 1, 1, 2); // mul v1.16b, v1.16b, v2.16b
                    self.a.v_addv_8b(1, 1);
                    self.a.v_umov_elem(0, T0, 1, 0);
                    self.wr_gpr(op.r0, op.w, T0);
                    self.emit_mmx_tag_valid();
                }
                MMX_CVTPI2PS_R | MMX_CVTPI2PS_M => {
                    let is_mem = (op.x & 0x3F) == MMX_CVTPI2PS_M;
                    if is_mem {
                        self.load_op(k, T0, op, 64);
                        self.a.fmov_gf(true, 1, T0);
                    } else {
                        self.load_mmx_d(1, op.r1);
                    }
                    self.a.v_scvtf_4s(1, 1);
                    self.xsrc(0, op.r0);
                    self.a.v_ins_lane(3, 0, 0, 1, 0);
                    self.xwrite(op.r0, 0);
                    if !is_mem {
                        self.emit_mmx_tag_valid();
                    }
                }
                MMX_CVTPI2PD_R | MMX_CVTPI2PD_M => {
                    let is_mem = (op.x & 0x3F) == MMX_CVTPI2PD_M;
                    if is_mem {
                        self.load_op(k, T0, op, 64);
                        self.a.fmov_gf(true, 1, T0);
                    } else {
                        self.load_mmx_d(1, op.r1);
                    }
                    self.a.v_xtl(false, 2, false, 1, 1); // sxtl v1.2d, v1.2s
                    self.a.v_scvtf_2d(0, 1);
                    self.xwrite(op.r0, 0);
                    if !is_mem {
                        self.emit_mmx_tag_valid();
                    }
                }
                sub @ MMX_CVTPS2PI_R..=MMX_CVTTPD2PI_M => {
                    let is_pd = sub >= MMX_CVTPD2PI_R;
                    let trunc = matches!(
                        sub,
                        MMX_CVTTPS2PI_R | MMX_CVTTPS2PI_M | MMX_CVTTPD2PI_R | MMX_CVTTPD2PI_M
                    );
                    let is_mem = (sub & 1) != 0;
                    if is_mem {
                        if is_pd {
                            let a = self.addr_into(op, XA);
                            if Self::chk_of(op) == Chk::Full {
                                let se = self.side_exit_here(k);
                                self.range_check_to(a, se);
                            }
                            self.a.ldr(64, T0, a, 0);
                            self.a.ldr(64, T1, a, 8);
                            if mem::tso() {
                                self.a.dmb_ishld();
                            }
                            self.a.fmov_gf(true, 1, T0);
                            self.a.v_ins_d(1, 1, T1);
                        } else {
                            self.load_op(k, T0, op, 64);
                            self.a.fmov_gf(true, 1, T0);
                        }
                    } else {
                        self.xsrc(1, op.r1);
                    }
                    if !is_pd {
                        self.a.v_fcvtl(false, 1, 1); // fcvtl v1.2d, v1.2s
                    }
                    // Round via FPCR.RMode (frinti) or truncate (frintz), then map out-of-range
                    // and NaN lanes to x86 32-bit integer indefinite (0x8000_0000).
                    self.a.v_frint(true, if trunc { 3 } else { 4 }, 5, 1);
                    self.a.v_fcvtzs_2d(0, 5);
                    self.a.mov_imm(true, T0, 0x41E0_0000_0000_0000); // +2^31
                    self.a.v_dup(3, 2, T0);
                    self.a.v_fcmgt(true, 3, 2, 5); // 5 < +2^31 (ordered)
                    self.a.mov_imm(true, T0, 0xC1E0_0000_0000_0000); // -2^31
                    self.a.v_dup(3, 4, T0);
                    self.a.v_fcmge(true, 4, 5, 4); // 5 >= -2^31 (ordered)
                    self.a.v_and(3, 3, 4);
                    self.a.mov_imm(true, T0, 0xFFFF_FFFF_8000_0000);
                    self.a.v_dup(3, 4, T0);
                    self.a.v_bsl(3, 0, 4);
                    self.a.v_xtn_2s(false, 0, 3);
                    self.store_mmx_d_and_tag(0, op.r0);
                }
                _ => {}
            },
            _ => {}
        }
    }
}
