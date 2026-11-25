//! Native ARM64 JIT/AOT lowering for the x87 FPU instruction family (`K_X87`).
//!
//! Architecture:
//! - Guest x87 state lives in a fixed 16-byte-aligned register-file area (`X87State`,
//!   pointed to by `Cpu::jx.x87_state`) shared with MMX (slice 4) and `fxsave`/`xsave`.
//! - When `TOP` is statically known within a block (e.g. after `fninit`/`fnsave` and across
//!   straight-line pushes/pops), `ST(i)` slots are addressed at compile-time constant offsets
//!   and intermediate `X87State::top` / `SW[13:11]` updates are elided until a flush point.
//! - Precision & rounding:
//!   - `CW.PC == 53-bit` (Win64 default `0x027F`): normal finite arithmetic (`fadd`/`fsub`/
//!     `fsubr`/`fmul`/`fdiv`/`fdivr`/`fsqrt`/`frndint`/`fist*`/`fcom*`/`fcomi*`) executes on
//!     native ARM64 `d0`/`d1` with `FPCR` isolated from `MXCSR` (`FZ=0`, `RMode` from `CW.RC`)
//!     and `FPSR` checked for `IOC|DZC|OFC|UFC` and `IXC -> SW.PE`.
//!   - `CW.PC == 24-bit`: normal finite arithmetic executes on native ARM64 `s0`/`s1`.
//!   - `CW.PC == 64-bit` (or subnormals, >53-bit significands, NaNs, Infs, overflow/underflow,
//!     stack faults, transcendentals, BCD, `fxam`, `fldenv`/`fnstenv`/`frstor`/`fnsave`): calls
//!     native Rust helpers in `crate::emu::x87::X87Helpers` via a block-local trampoline and
//!     indirect table pointer `Cpu::jx.x87_helpers` — **zero interpreter fallbacks** and
//!     **zero runtime relocations** (compatible with shifted-base `.aot` and signed static
//!     Mach-O `.aot.o` on macOS & iOS).

use super::emit::{enc_logic_imm, Asm};
use super::lazy::PF_NONE;
use super::translate::{Cond, FlagState, Src, Tr};
use super::*;
use crate::emu::fast::{self, FastOp};
use crate::emu::mem;
use crate::emu::x87::{
    X87Helpers, OFF_X87_CW, OFF_X87_FDP, OFF_X87_FIP, OFF_X87_FOP, OFF_X87_SW, OFF_X87_TOP,
    OFF_X87_VALID, SW_C0, SW_C1, SW_C2, SW_C3, SW_IE, SW_SF, X87_CONST_ONE, X87_CONST_ZERO,
    X87_OP_ADD, X87_OP_DIV, X87_OP_MUL, X87_OP_SUB, X87_OP_SUBR, X87_UNARY_RNDINT, X87_UNARY_SQRT,
};
use core::mem::offset_of;

/// Mapping from `X87_FCMOV` `op.w` (`0..8`) to iced `ConditionCode` numbering used by `Tr::cond`.
pub(super) const FCMOV_CC: [u8; 8] = [3, 5, 7, 11, 4, 6, 8, 12];

/// Byte offsets of function pointers inside `X87Helpers`.
const H_FLD_F32: u32 = offset_of!(X87Helpers, fld_f32) as u32;
const H_FLD_F64: u32 = offset_of!(X87Helpers, fld_f64) as u32;
const H_FLD_F80: u32 = offset_of!(X87Helpers, fld_f80) as u32;
const H_FLD_ST: u32 = offset_of!(X87Helpers, fld_st) as u32;
const H_FILD_I64: u32 = offset_of!(X87Helpers, fild_i64) as u32;
const H_FLD_CONST: u32 = offset_of!(X87Helpers, fld_const) as u32;
const H_FST_F32: u32 = offset_of!(X87Helpers, fst_f32) as u32;
const H_FST_F64: u32 = offset_of!(X87Helpers, fst_f64) as u32;
const H_FSTP_F80: u32 = offset_of!(X87Helpers, fstp_f80) as u32;
const H_FST_ST: u32 = offset_of!(X87Helpers, fst_st) as u32;
const H_FIST: u32 = offset_of!(X87Helpers, fist) as u32;
const H_BINOP_ST: u32 = offset_of!(X87Helpers, binop_st) as u32;
const H_BINOP_F32: u32 = offset_of!(X87Helpers, binop_f32) as u32;
const H_BINOP_F64: u32 = offset_of!(X87Helpers, binop_f64) as u32;
const H_BINOP_I64: u32 = offset_of!(X87Helpers, binop_i64) as u32;
const H_UNARY: u32 = offset_of!(X87Helpers, unary) as u32;
const H_COMPARE_ST: u32 = offset_of!(X87Helpers, compare_st) as u32;
const H_COMPARE_F32: u32 = offset_of!(X87Helpers, compare_f32) as u32;
const H_COMPARE_F64: u32 = offset_of!(X87Helpers, compare_f64) as u32;
const H_COMPARE_I64: u32 = offset_of!(X87Helpers, compare_i64) as u32;
const H_FXAM: u32 = offset_of!(X87Helpers, fxam) as u32;
const H_FCMOV: u32 = offset_of!(X87Helpers, fcmov) as u32;
const H_FBLD: u32 = offset_of!(X87Helpers, fbld) as u32;
const H_FBSTP: u32 = offset_of!(X87Helpers, fbstp) as u32;
const H_FLDENV: u32 = offset_of!(X87Helpers, fldenv) as u32;
const H_FNSTENV: u32 = offset_of!(X87Helpers, fnstenv) as u32;
const H_FRSTOR: u32 = offset_of!(X87Helpers, frstor) as u32;
const H_FNSAVE: u32 = offset_of!(X87Helpers, fnsave) as u32;

/// Deferred out-of-line cold helper call emitted in the block's cold section.
pub(super) struct X87Cold {
    /// Branch sites in the hot path that jump to this cold stub.
    sites: Vec<usize>,
    /// Word position in the hot path to jump back to after the cold helper finishes.
    resume: usize,
    /// Pre-op static TOP state (flushed to `X87State` before invoking the helper if dirty).
    top_known: Option<u8>,
    top_dirty: bool,
    sw_top_dirty: bool,
    kind: X87ColdKind,
}

enum X87ColdKind {
    /// `fld m32` (loaded value in `w6`).
    FldF32,
    /// `fld m64` (loaded value in `x6`).
    FldF64,
    /// `fld m80` (loaded `(lo, hi)` in `(x5, x6)`).
    FldF80,
    /// `fild m16/32/64` (loaded sign-extended `i64` in `x6`).
    FildI64,
    /// `fld st(idx)`.
    FldSt { idx: u8 },
    /// `fldz` / `fld1` overflow fallback.
    FldConst { id: u8 },
    /// `fst`/`fstp st(idx)`.
    FstSt { idx: u8, pop: bool },
    /// `fst`/`fstp m32/m64/m80`: calls helper then stores result to `op`'s address.
    FstMem { op: FastOp, sz: u8, pop: bool },
    /// `fist`/`fistp`/`fisttp m16/m32/m64`: calls helper then stores result to `op`'s address.
    Fist { op: FastOp, bits: u32, flags: u8 },
    /// `fadd/fsub/fsubr/fmul/fdiv/fdivr [p] st(dst), st(src)`.
    BinopSt { op_pop: u8, dst: u8, src: u8 },
    /// `fadd/fsub/... m32fp/m64fp/m16int/m32int` (loaded value in `x6`).
    BinopMem { op_code: u8, w: u8 },
    /// `fsqrt` / `frndint`.
    Unary { sub: u8 },
    /// `fcom*/fucom*/fcomi*/fucomi* st(idx)` (if `is_fcomi`, synthesizes `Src::Fcmp` NZCV!).
    CmpSt { idx: u8, flags: u8, is_fcomi: bool },
    /// `fcom/fcomp/ficom/ficomp mem` (loaded value in `x6`).
    CmpMem { w: u8, flags: u8 },
    /// `ftst`.
    Ftst,
    /// `fcmov*` when either `ST(0)` or `ST(idx)` is empty (condition in `w6`).
    FcmovFault { idx: u8 },
}

/// Helper: emit `bl` to the block-local `x87_trampoline` (patched once in `emit_x87_colds`).
#[inline]
fn emit_tramp_bl(a: &mut Asm, calls: &mut Vec<usize>) {
    calls.push(a.bl());
}

/// Load `st: *mut X87State` into `rd` and flush static TOP state into `st` if dirty.
fn emit_flush_top_raw(
    a: &mut Asm,
    st_reg: u32,
    tmp1: u32,
    tmp2: u32,
    top_known: Option<u8>,
    top_dirty: bool,
    sw_top_dirty: bool,
) {
    if !top_dirty && !sw_top_dirty {
        return;
    }
    if let Some(t) = top_known {
        a.movz(false, tmp1, t as u32, 0);
        if top_dirty {
            a.str(8, tmp1, st_reg, OFF_X87_TOP);
        }
    } else {
        a.ldr(8, tmp1, st_reg, OFF_X87_TOP);
    }
    if sw_top_dirty {
        a.ldr(16, tmp2, st_reg, OFF_X87_SW);
        a.logic_ri(false, 0, tmp2, tmp2, enc_logic_imm(!0x3800u64, false).unwrap());
        a.logic_rr(false, 1, tmp2, tmp2, tmp1, 0, 11);
        a.str(16, tmp2, st_reg, OFF_X87_SW);
    }
}

/// Convert `CW` in `cw_reg` (16-bit) to target ARM64 `FPCR` in `fpcr_reg`:
/// `FZ=0, DN=0, AH=0, FIZ=0`, `RMode = swap_bits(cw[11:10]) << 22`.
/// Uses `tmp` as scratch. Does not touch NZCV.
fn emit_cw_to_fpcr(a: &mut Asm, cw_reg: u32, fpcr_reg: u32, tmp: u32) {
    // x87 RC: 0=RN(00), 1=RM(01->10), 2=RP(10->01), 3=RZ(11)
    a.ubfm(false, tmp, cw_reg, 10, 10); // bit 0 = rc_lo
    a.ubfm(false, fpcr_reg, cw_reg, 11, 11); // bit 0 = rc_hi
    a.logic_rr(false, 1, fpcr_reg, fpcr_reg, tmp, 0, 1); // bits 1:0 = arm_rmode
    a.lsl_i(true, fpcr_reg, fpcr_reg, 22);
}

/// Inline conversion: IEEE `f64` bits in `val_reg` -> 80-bit `(sig_out, se_out)`.
/// Branches to `cold_sites` on subnormal, Inf, or NaN.
/// Scratch: `t1`, `t2`, `t3` (must all be distinct from `val_reg`, `sig_out`, `se_out`,
/// though `sig_out`/`se_out` may alias `t1`/`t2` only if noted — keep all 6 distinct or
/// pass `val_reg=T6, sig_out=T0, se_out=T1, t1=T2, t2=T3, t3=T4`).
fn emit_f64_to_f80(
    a: &mut Asm,
    val_reg: u32,
    sig_out: u32,
    se_out: u32,
    t1: u32,
    t2: u32,
    t3: u32,
    cold_sites: &mut Vec<usize>,
) {
    a.ubfx(t1, val_reg, 52, 11); // exp11
    a.lsr_i(true, t2, val_reg, 63); // sign (0 or 1)
    a.addsub_ri(false, true, true, ZR, t1, 0x7FF, false);
    cold_sites.push(a.bcond(EQ)); // Inf/NaN -> cold
    let is_norm = a.cbnz(false, t1);
    // exp11 == 0: check exact zero vs subnormal
    a.lsl_i(true, t3, val_reg, 1);
    cold_sites.push(a.cbnz(true, t3)); // subnormal -> cold
    a.movz(true, sig_out, 0, 0);
    a.lsl_i(false, se_out, t2, 15);
    let done = a.b();
    a.bind(is_norm);
    a.movz(false, t3, 0x3C00, 0); // 16383 - 1023 = 15360
    a.addsub_rr(false, false, false, se_out, t1, t3, 0);
    a.logic_rr(false, 1, se_out, se_out, t2, 0, 15);
    a.lsl_i(true, sig_out, val_reg, 11);
    a.logic_ri(true, 1, sig_out, sig_out, enc_logic_imm(1u64 << 63, true).unwrap());
    a.bind(done);
}

/// Inline conversion: IEEE `f32` bits in `val_reg` (zero-extended in 64-bit) -> 80-bit `(sig_out, se_out)`.
/// Branches to `cold_sites` on subnormal, Inf, or NaN.
fn emit_f32_to_f80(
    a: &mut Asm,
    val_reg: u32,
    sig_out: u32,
    se_out: u32,
    t1: u32,
    t2: u32,
    t3: u32,
    cold_sites: &mut Vec<usize>,
) {
    a.ubfm(false, t1, val_reg, 23, 30); // exp8
    a.lsr_i(false, t2, val_reg, 31); // sign
    a.addsub_ri(false, true, true, ZR, t1, 0xFF, false);
    cold_sites.push(a.bcond(EQ)); // Inf/NaN -> cold
    let is_norm = a.cbnz(false, t1);
    a.lsl_i(false, t3, val_reg, 1);
    cold_sites.push(a.cbnz(false, t3)); // subnormal -> cold
    a.movz(true, sig_out, 0, 0);
    a.lsl_i(false, se_out, t2, 15);
    let done = a.b();
    a.bind(is_norm);
    a.movz(false, t3, 0x3F80, 0); // 16383 - 127 = 16256
    a.addsub_rr(false, false, false, se_out, t1, t3, 0);
    a.logic_rr(false, 1, se_out, se_out, t2, 0, 15);
    a.ubfx(sig_out, val_reg, 0, 23);
    a.lsl_i(true, sig_out, sig_out, 40);
    a.logic_ri(true, 1, sig_out, sig_out, enc_logic_imm(1u64 << 63, true).unwrap());
    a.bind(done);
}

/// Inline exact conversion: 80-bit `(sig_in, se_in)` -> IEEE `f64` bits in `bits_out`.
/// Branches to `cold_sites` if subnormal, unnormal, Inf, NaN, out of `f64` normal range,
/// or low 11 bits of `sig_in` are non-zero (lossy conversion).
fn emit_f80_to_f64_exact(
    a: &mut Asm,
    sig_in: u32,
    se_in: u32,
    bits_out: u32,
    t1: u32,
    t2: u32,
    cold_sites: &mut Vec<usize>,
) {
    a.logic_ri(false, 0, t1, se_in, enc_logic_imm(0x7FFF, false).unwrap()); // exp15
    a.lsr_i(false, t2, se_in, 15); // sign (0 or 1, zero-extended to 64b)
    let nonzero = a.cbnz(false, t1);
    cold_sites.push(a.cbnz(true, sig_in)); // 80-bit subnormal -> cold
    a.lsl_i(true, bits_out, t2, 63);
    let done = a.b();
    a.bind(nonzero);
    // Bit 63 of sig_in must be 1 (signed < 0); >= 0 means unnormal/unsupported -> cold.
    a.addsub_ri(true, true, true, ZR, sig_in, 0, false);
    cold_sites.push(a.bcond(GE));
    // Low 11 bits of sig_in must be 0 for lossless f64 representation.
    a.logic_ri(true, 3, ZR, sig_in, enc_logic_imm(0x7FF, true).unwrap());
    cold_sites.push(a.bcond(NE));
    // exp64 = exp15 - 15360; require 1 <= exp64 <= 0x7FE (rejects Inf/NaN/MMX/overflow/underflow).
    a.movz(false, bits_out, 0x3C00, 0);
    a.addsub_rr(false, true, false, t1, t1, bits_out, 0);
    a.addsub_ri(false, true, false, bits_out, t1, 1, false);
    a.addsub_ri(false, true, true, ZR, bits_out, 0x7FE, false);
    cold_sites.push(a.bcond(HS));
    // Assemble IEEE f64 bits.
    a.ubfx(bits_out, sig_in, 11, 52);
    a.logic_rr(true, 1, bits_out, bits_out, t1, 0, 52);
    a.logic_rr(true, 1, bits_out, bits_out, t2, 0, 63);
    a.bind(done);
}

/// Inline exact conversion: 80-bit `(sig_in, se_in)` -> IEEE `f32` bits in `w(bits_out)`.
/// Branches to `cold_sites` if subnormal, unnormal, Inf, NaN, out of `f32` normal range,
/// or low 40 bits of `sig_in` are non-zero.
fn emit_f80_to_f32_exact(
    a: &mut Asm,
    sig_in: u32,
    se_in: u32,
    bits_out: u32,
    t1: u32,
    t2: u32,
    cold_sites: &mut Vec<usize>,
) {
    a.logic_ri(false, 0, t1, se_in, enc_logic_imm(0x7FFF, false).unwrap()); // exp15
    a.lsr_i(false, t2, se_in, 15); // sign
    let nonzero = a.cbnz(false, t1);
    cold_sites.push(a.cbnz(true, sig_in)); // 80-bit subnormal -> cold
    a.lsl_i(false, bits_out, t2, 31);
    let done = a.b();
    a.bind(nonzero);
    a.addsub_ri(true, true, true, ZR, sig_in, 0, false);
    cold_sites.push(a.bcond(GE));
    a.logic_ri(true, 3, ZR, sig_in, enc_logic_imm(0xFF_FFFF_FFFF, true).unwrap());
    cold_sites.push(a.bcond(NE));
    a.movz(false, bits_out, 0x3F80, 0);
    a.addsub_rr(false, true, false, t1, t1, bits_out, 0);
    a.addsub_ri(false, true, false, bits_out, t1, 1, false);
    a.addsub_ri(false, true, true, ZR, bits_out, 0xFE, false);
    cold_sites.push(a.bcond(HS));
    a.ubfx(bits_out, sig_in, 40, 23);
    a.logic_rr(false, 1, bits_out, bits_out, t1, 0, 23);
    a.logic_rr(false, 1, bits_out, bits_out, t2, 0, 31);
    a.bind(done);
}

impl Tr<'_> {
    /// Load `X87State*` into `XA` and invalidate the cached guest effective address `xa_key`.
    #[inline]
    fn load_x87_state(&mut self) {
        self.xa_key = None;
        self.a.ldr(64, XA, XCPU, OFF_X87_STATE);
    }

    /// Compute the guest effective address of `op` into `XA` and invalidate any prior `xa_key`.
    #[inline]
    fn addr_x87(&mut self, op: &FastOp) -> u32 {
        self.xa_key = None;
        self.addr_into(op, XA)
    }

    /// Flush any deferred static x87 TOP / `SW[13:11]` updates into `X87State`.
    /// Preserves NZCV and `x87_top_known`.
    pub(super) fn flush_x87_top(&mut self) {
        if !self.x87_top_dirty && !self.x87_sw_top_dirty {
            return;
        }
        self.load_x87_state();
        emit_flush_top_raw(
            &mut self.a,
            XA,
            XS,
            T0,
            self.x87_top_known,
            self.x87_top_dirty,
            self.x87_sw_top_dirty,
        );
        self.x87_top_dirty = false;
        self.x87_sw_top_dirty = false;
    }

    /// Flush deferred x87 TOP state for a side-exit snapshot without mutating `self`.
    pub(super) fn flush_x87_top_for_exit(&mut self, top_known: Option<u8>, top_dirty: bool, sw_top_dirty: bool) {
        if !top_dirty && !sw_top_dirty {
            return;
        }
        self.load_x87_state();
        emit_flush_top_raw(&mut self.a, XA, XS, T0, top_known, top_dirty, sw_top_dirty);
    }

    /// Compute physical slot index `p = (top + delta) & 7` into `rd` (32-bit zero-extended),
    /// assuming `st_reg` holds `X87State*`. Does not touch NZCV.
    fn emit_phys_idx(&mut self, st_reg: u32, delta: u8, rd: u32) {
        if let Some(t) = self.x87_top_known {
            self.a.movz(false, rd, ((t.wrapping_add(delta)) & 7) as u32, 0);
        } else {
            self.a.ldr(8, rd, st_reg, OFF_X87_TOP);
            let d = delta & 7;
            if d != 0 {
                self.a.addsub_ri(false, false, false, rd, rd, d as u32, false);
                self.a.logic_ri(false, 0, rd, rd, enc_logic_imm(7, false).unwrap());
            }
        }
    }

    /// Compute pointer `&st.regs[phys(delta)]` into `addr_reg` (and physical index into `idx_reg`
    /// if `idx_reg != ZR`), assuming `st_reg` holds `X87State*`. Does not touch NZCV.
    fn emit_slot_ptr(&mut self, st_reg: u32, delta: u8, addr_reg: u32, idx_reg: u32) {
        if let Some(t) = self.x87_top_known {
            let p = ((t.wrapping_add(delta)) & 7) as u32;
            if idx_reg != ZR {
                self.a.movz(false, idx_reg, p, 0);
            }
            if p == 0 {
                self.a.mov_rr(true, addr_reg, st_reg);
            } else {
                self.a.addsub_ri(true, false, false, addr_reg, st_reg, p * 16, false);
            }
        } else {
            let ireg = if idx_reg != ZR { idx_reg } else { addr_reg };
            self.a.ldr(8, ireg, st_reg, OFF_X87_TOP);
            let d = delta & 7;
            if d != 0 {
                self.a.addsub_ri(false, false, false, ireg, ireg, d as u32, false);
                self.a.logic_ri(false, 0, ireg, ireg, enc_logic_imm(7, false).unwrap());
            }
            self.a.addsub_rr(true, false, false, addr_reg, st_reg, ireg, 4);
        }
    }

    /// Advance logical TOP by `step` (`7` = push / decrement, `1` = pop / increment, `2` = double pop).
    /// When `x87_top_known` is `Some`, updates compile-time state with ZERO memory writes!
    /// When `x87_top_known` is `None`, updates `X87State::top` in memory and marks `sw_top_dirty`.
    fn emit_step_top(&mut self, st_reg: u32, step: u8, tmp: u32) {
        let step = step & 7;
        if step == 0 {
            return;
        }
        if let Some(t) = self.x87_top_known {
            self.x87_top_known = Some(t.wrapping_add(step) & 7);
            self.x87_top_dirty = true;
            self.x87_sw_top_dirty = true;
        } else {
            self.a.ldr(8, tmp, st_reg, OFF_X87_TOP);
            self.a.addsub_ri(false, false, false, tmp, tmp, step as u32, false);
            self.a.logic_ri(false, 0, tmp, tmp, enc_logic_imm(7, false).unwrap());
            self.a.str(8, tmp, st_reg, OFF_X87_TOP);
            self.x87_sw_top_dirty = true;
        }
    }

    /// Test whether physical slot `idx_reg` is valid (`(valid_mask >> idx_reg) & 1 != 0`).
    /// Sets `tmp = 1 << idx_reg` and `NZCV` via `tst valid_mask, tmp` (`EQ` = empty, `NE` = valid).
    fn emit_test_valid(&mut self, st_reg: u32, idx_reg: u32, vm_reg: u32, bit_reg: u32) {
        self.a.ldr(8, vm_reg, st_reg, OFF_X87_VALID);
        self.a.movz(false, bit_reg, 1, 0);
        self.a.shiftv(false, 8, bit_reg, bit_reg, idx_reg); // lslv w_bit, #1, w_idx
        self.a.logic_rr(false, 3, ZR, vm_reg, bit_reg, 0, 0); // tst w_vm, w_bit
    }

    /// Range-check `[a, a + bytes)` against `[0x1_0000, 2^47)` (no-op if `hw_range_enabled()`).
    /// Preserves `a` and NZCV. Uses `tmp` and `XS`.
    fn range_check_span(&mut self, k: usize, a: u32, bytes: u32, tmp: u32) {
        if hw_range_enabled() {
            return;
        }
        let se = self.side_exit_here(k);
        self.range_check_to(a, se);
        if bytes > 1 {
            self.a.addsub_ri(true, false, false, tmp, a, bytes - 1, false);
            self.range_check_to(tmp, se);
        }
    }

    /// Emit direct call to `X87Helpers` slot `helper_off` via block-local `x87_trampoline`.
    /// Caller must have loaded `x1..x3` (args) already; this loads `x0 = X87State*` and
    /// `XA = helper_fn_ptr`. Preserves guest GPRs, `XCPU`, `XBUD`, `XRAS`, `v16..v31`, `NZCV`.
    fn emit_call_x87_helper(&mut self, helper_off: u32) {
        self.xa_key = None;
        self.a.ldr(64, T0, XCPU, OFF_X87_STATE);
        self.a.ldr(64, XA, XCPU, OFF_X87_HELPERS);
        self.a.ldr(64, XA, XA, helper_off);
        emit_tramp_bl(&mut self.a, &mut self.x87_tramp_calls);
    }

    /// Emit a single guest `K_X87` instruction at block index `k`.
    pub(super) fn emit_x87_op(&mut self, k: usize, op: &FastOp) {
        use fast::*;
        let sub = op.x & 0x3F;
        match sub {
            X87_FNINIT => {
                // Zero all 8 register slots + set cw = 0x037F, sw = 0, fop = 0, top = 0, valid_mask = 0,
                // fip = 0, fdp = 0 (matches `x87_reset()` bit-for-bit across all tiers).
                // Does NOT touch NZCV!
                self.load_x87_state();
                for slot in 0..8i32 {
                    self.a.stp_x(ZR, ZR, XA, slot * 16);
                }
                self.a.movz(false, T0, 0x037F, 0);
                // Store [cw=0x037F, sw=0, fop=0, top=0, valid_mask=0] as one 64-bit store at 0x80!
                self.a.str(64, T0, XA, OFF_X87_CW);
                self.a.stp_x(ZR, ZR, XA, OFF_X87_FIP as i32);
                self.x87_top_known = Some(0);
                self.x87_top_dirty = false;
                self.x87_sw_top_dirty = false;
            }
            X87_FNCLEX => {
                // Does NOT touch NZCV!
                self.load_x87_state();
                self.a.ldr(16, T0, XA, OFF_X87_SW);
                self.a.movz(false, T1, 0x80FF, 0);
                self.a.bic_rr(false, T0, T0, T1);
                self.a.str(16, T0, XA, OFF_X87_SW);
            }
            X87_FLDCW => {
                // Does NOT touch NZCV!
                self.load_op(k, T0, op, 16);
                self.load_x87_state();
                self.a.str(16, T0, XA, OFF_X87_CW);
            }
            X87_FNSTCW => {
                // Does NOT touch NZCV!
                self.load_x87_state();
                self.a.ldr(16, T0, XA, OFF_X87_CW);
                self.store_op(k, T0, op, 16);
            }
            X87_FNSTSW_AX => {
                // Does NOT touch NZCV!
                self.flush_x87_top();
                self.load_x87_state();
                self.a.ldr(16, T0, XA, OFF_X87_SW);
                self.wr_gpr(0, 16, T0);
            }
            X87_FNSTSW_M => {
                // Does NOT touch NZCV!
                self.flush_x87_top();
                self.load_x87_state();
                self.a.ldr(16, T0, XA, OFF_X87_SW);
                self.store_op(k, T0, op, 16);
            }
            X87_FDECSTP | X87_FINCSTP => {
                // Does NOT touch NZCV!
                self.load_x87_state();
                self.a.ldr(16, T0, XA, OFF_X87_SW);
                self.a.logic_ri(false, 0, T0, T0, enc_logic_imm(!(SW_C1 as u64), false).unwrap());
                self.a.str(16, T0, XA, OFF_X87_SW);
                let step = if sub == X87_FDECSTP { 7 } else { 1 };
                self.emit_step_top(XA, step, T1);
            }
            X87_FFREE => {
                // Does NOT touch NZCV!
                self.load_x87_state();
                self.emit_phys_idx(XA, op.r0, T0);
                self.a.ldr(8, T1, XA, OFF_X87_VALID);
                self.a.movz(false, T2, 1, 0);
                self.a.shiftv(false, 8, T2, T2, T0);
                self.a.bic_rr(false, T1, T1, T2);
                if op.w != 0 {
                    // ffreep: also pop ST(0)
                    self.emit_phys_idx(XA, 0, T0);
                    self.a.movz(false, T2, 1, 0);
                    self.a.shiftv(false, 8, T2, T2, T0);
                    self.a.bic_rr(false, T1, T1, T2);
                    self.a.str(8, T1, XA, OFF_X87_VALID);
                    self.emit_step_top(XA, 1, T0);
                } else {
                    self.a.str(8, T1, XA, OFF_X87_VALID);
                }
            }
            X87_FXCH => {
                self.flush_flags(self.live[k]);
                self.fs.host = 0;
                self.load_x87_state();
                // Clear C1
                self.a.ldr(16, T0, XA, OFF_X87_SW);
                self.a.logic_ri(false, 0, T0, T0, enc_logic_imm(!(SW_C1 as u64), false).unwrap());
                self.a.str(16, T0, XA, OFF_X87_SW);
                self.emit_slot_ptr(XA, 0, T1, T2); // T1=&regs[p0], T2=p0
                self.emit_slot_ptr(XA, op.r0, T3, T4); // T3=&regs[pi], T4=pi
                self.a.ldr(8, T5, XA, OFF_X87_VALID);
                // Check both p0 and pi valid
                self.a.movz(false, T0, 1, 0);
                self.a.shiftv(false, 8, T6, T0, T2); // bit p0
                self.a.shiftv(false, 8, XS, T0, T4); // bit pi
                self.a.logic_rr(false, 1, T0, T6, XS, 0, 0); // mask = bit(p0) | bit(pi)
                self.a.logic_rr(false, 0, T2, T5, T0, 0, 0);
                self.a.addsub_rr(false, true, true, ZR, T2, T0, 0);
                let both_ok = self.a.bcond(EQ);
                // Empty slot(s): set SW.IE | SW.SF, fill empty slot(s) with INDEF_BYTES and mark valid
                self.a.ldr(16, T2, XA, OFF_X87_SW);
                self.a.movz(false, T4, (SW_IE | SW_SF) as u32, 0);
                self.a.logic_rr(false, 1, T2, T2, T4, 0, 0);
                self.a.str(16, T2, XA, OFF_X87_SW);
                // INDEF_BYTES: sig = 0xC000_0000_0000_0000, se = 0xFFFF
                self.a.movz(true, T2, 0xC000, 3);
                self.a.movz(true, T4, 0xFFFF, 0);
                self.a.logic_rr(false, 3, ZR, T5, T6, 0, 0);
                let p0_valid = self.a.bcond(NE);
                self.a.stp_x(T2, T4, T1, 0);
                self.a.bind(p0_valid);
                self.a.logic_rr(false, 3, ZR, T5, XS, 0, 0);
                let pi_valid = self.a.bcond(NE);
                self.a.stp_x(T2, T4, T3, 0);
                self.a.bind(pi_valid);
                self.a.logic_rr(false, 1, T5, T5, T0, 0, 0);
                self.a.str(8, T5, XA, OFF_X87_VALID);
                self.a.bind(both_ok);
                if op.r0 != 0 {
                    self.a.ldr(128, 0, T1, 0);
                    self.a.ldr(128, 1, T3, 0);
                    self.a.str(128, 1, T1, 0);
                    self.a.str(128, 0, T3, 0);
                }
            }
            X87_FABS | X87_FCHS => {
                self.flush_flags(self.live[k]);
                self.fs.host = 0;
                self.load_x87_state();
                self.a.ldr(16, T0, XA, OFF_X87_SW);
                self.a.logic_ri(false, 0, T0, T0, enc_logic_imm(!(SW_C1 as u64), false).unwrap());
                self.a.str(16, T0, XA, OFF_X87_SW);
                self.emit_slot_ptr(XA, 0, T1, T2);
                self.emit_test_valid(XA, T2, T3, T4);
                let valid = self.a.bcond(NE);
                // Underflow: set SW.IE | SW.SF, write INDEF_BYTES, mark valid
                self.a.ldr(16, T0, XA, OFF_X87_SW);
                self.a.movz(false, T2, (SW_IE | SW_SF) as u32, 0);
                self.a.logic_rr(false, 1, T0, T0, T2, 0, 0);
                self.a.str(16, T0, XA, OFF_X87_SW);
                self.a.movz(true, T0, 0xC000, 3);
                self.a.movz(true, T2, 0xFFFF, 0);
                self.a.stp_x(T0, T2, T1, 0);
                self.a.logic_rr(false, 1, T3, T3, T4, 0, 0);
                self.a.str(8, T3, XA, OFF_X87_VALID);
                let done = self.a.b();
                self.a.bind(valid);
                self.a.ldr(16, T0, T1, 8);
                if sub == X87_FABS {
                    self.a.logic_ri(false, 0, T0, T0, enc_logic_imm(0x7FFF, false).unwrap());
                } else {
                    self.a.logic_ri(false, 2, T0, T0, enc_logic_imm(0x8000, false).unwrap());
                }
                self.a.str(16, T0, T1, 8);
                self.a.bind(done);
            }
            X87_FXAM => {
                self.flush_x87_top();
                self.emit_call_x87_helper(H_FXAM);
            }
            X87_FLD_ST => {
                self.flush_flags(self.live[k]);
                self.fs.host = 0;
                let (tk, td, std) = (self.x87_top_known, self.x87_top_dirty, self.x87_sw_top_dirty);
                let mut cold_sites = Vec::new();
                self.load_x87_state();
                self.emit_slot_ptr(XA, op.r0, T1, T2); // src
                self.emit_slot_ptr(XA, 7, T3, T4); // new_top = (top - 1) & 7
                // Require src valid AND new_top empty; else cold helper.
                self.emit_test_valid(XA, T2, T5, T0);
                cold_sites.push(self.a.bcond(EQ));
                self.a.movz(false, T0, 1, 0);
                self.a.shiftv(false, 8, T0, T0, T4);
                self.a.logic_rr(false, 3, ZR, T5, T0, 0, 0);
                cold_sites.push(self.a.bcond(NE)); // overflow -> cold
                // Clear C1, copy 16B slot, mark new_top valid, step top -= 1
                self.a.ldr(16, T2, XA, OFF_X87_SW);
                self.a.logic_ri(false, 0, T2, T2, enc_logic_imm(!(SW_C1 as u64), false).unwrap());
                self.a.str(16, T2, XA, OFF_X87_SW);
                self.a.ldr(128, 0, T1, 0);
                self.a.str(128, 0, T3, 0);
                self.a.logic_rr(false, 1, T5, T5, T0, 0, 0);
                self.a.str(8, T5, XA, OFF_X87_VALID);
                self.emit_step_top(XA, 7, T2);
                let resume = self.a.pos;
                self.x87_colds.push(X87Cold {
                    sites: cold_sites,
                    resume,
                    top_known: tk,
                    top_dirty: td,
                    sw_top_dirty: std,
                    kind: X87ColdKind::FldSt { idx: op.r0 },
                });
            }
            X87_FLD_CONST => {
                if op.r0 == X87_CONST_ZERO || op.r0 == X87_CONST_ONE {
                    self.flush_flags(self.live[k]);
                    self.fs.host = 0;
                    let (tk, td, std) = (self.x87_top_known, self.x87_top_dirty, self.x87_sw_top_dirty);
                    let mut cold_sites = Vec::new();
                    self.load_x87_state();
                    self.emit_slot_ptr(XA, 7, T1, T2);
                    self.emit_test_valid(XA, T2, T3, T4);
                    cold_sites.push(self.a.bcond(NE)); // overflow -> cold
                    self.a.ldr(16, T0, XA, OFF_X87_SW);
                    self.a.logic_ri(false, 0, T0, T0, enc_logic_imm(!(SW_C1 as u64), false).unwrap());
                    self.a.str(16, T0, XA, OFF_X87_SW);
                    if op.r0 == X87_CONST_ZERO {
                        self.a.stp_x(ZR, ZR, T1, 0);
                    } else {
                        self.a.movz(true, T0, 0x8000, 3);
                        self.a.movz(true, T2, 0x3FFF, 0);
                        self.a.stp_x(T0, T2, T1, 0);
                    }
                    self.a.logic_rr(false, 1, T3, T3, T4, 0, 0);
                    self.a.str(8, T3, XA, OFF_X87_VALID);
                    self.emit_step_top(XA, 7, T0);
                    let resume = self.a.pos;
                    self.x87_colds.push(X87Cold {
                        sites: cold_sites,
                        resume,
                        top_known: tk,
                        top_dirty: td,
                        sw_top_dirty: std,
                        kind: X87ColdKind::FldConst { id: op.r0 },
                    });
                } else {
                    // fldpi / fldl2e / fldl2t / fldlg2 / fldln2 (RC-sensitive 80-bit constants)
                    self.flush_x87_top();
                    self.a.movz(false, T1, op.r0 as u32, 0);
                    self.emit_call_x87_helper(H_FLD_CONST);
                    if let Some(t) = self.x87_top_known {
                        self.x87_top_known = Some(t.wrapping_add(7) & 7);
                    }
                }
            }
            X87_FLD_MEM => {
                self.flush_flags(self.live[k]);
                self.fs.host = 0;
                match op.w {
                    4 | 8 => {
                        let bits = (op.w as u32) * 8;
                        self.load_op(k, T6, op, bits);
                        let (tk, td, std) = (self.x87_top_known, self.x87_top_dirty, self.x87_sw_top_dirty);
                        let mut cold_sites = Vec::new();
                        // Convert T6 -> (T0=sig, T1=se); subnormal/Inf/NaN -> cold
                        if op.w == 8 {
                            emit_f64_to_f80(&mut self.a, T6, T0, T1, T2, T3, T4, &mut cold_sites);
                        } else {
                            emit_f32_to_f80(&mut self.a, T6, T0, T1, T2, T3, T4, &mut cold_sites);
                        }
                        self.load_x87_state();
                        self.emit_slot_ptr(XA, 7, T2, T3);
                        self.emit_test_valid(XA, T3, T4, T5);
                        cold_sites.push(self.a.bcond(NE)); // overflow -> cold
                        self.a.ldr(16, T3, XA, OFF_X87_SW);
                        self.a.logic_ri(false, 0, T3, T3, enc_logic_imm(!(SW_C1 as u64), false).unwrap());
                        self.a.str(16, T3, XA, OFF_X87_SW);
                        self.a.stp_x(T0, T1, T2, 0);
                        self.a.logic_rr(false, 1, T4, T4, T5, 0, 0);
                        self.a.str(8, T4, XA, OFF_X87_VALID);
                        self.emit_step_top(XA, 7, T3);
                        let resume = self.a.pos;
                        self.x87_colds.push(X87Cold {
                            sites: cold_sites,
                            resume,
                            top_known: tk,
                            top_dirty: td,
                            sw_top_dirty: std,
                            kind: if op.w == 8 { X87ColdKind::FldF64 } else { X87ColdKind::FldF32 },
                        });
                    }
                    _ => {
                        // m80fp (10 bytes): range-check [a, a+10), load (lo=T5, hi=T6), push inline.
                        let a = self.addr_x87(op);
                        self.range_check_span(k, a, 10, T0);
                        self.a.ldr(64, T5, a, 0);
                        self.a.ldr(16, T6, a, 8);
                        if mem::tso() {
                            self.a.dmb_ishld();
                        }
                        let (tk, td, std) = (self.x87_top_known, self.x87_top_dirty, self.x87_sw_top_dirty);
                        let mut cold_sites = Vec::new();
                        self.load_x87_state();
                        self.emit_slot_ptr(XA, 7, T0, T1);
                        self.emit_test_valid(XA, T1, T2, T3);
                        cold_sites.push(self.a.bcond(NE)); // overflow -> cold
                        self.a.ldr(16, T1, XA, OFF_X87_SW);
                        self.a.logic_ri(false, 0, T1, T1, enc_logic_imm(!(SW_C1 as u64), false).unwrap());
                        self.a.str(16, T1, XA, OFF_X87_SW);
                        self.a.stp_x(T5, T6, T0, 0);
                        self.a.logic_rr(false, 1, T2, T2, T3, 0, 0);
                        self.a.str(8, T2, XA, OFF_X87_VALID);
                        self.emit_step_top(XA, 7, T1);
                        let resume = self.a.pos;
                        self.x87_colds.push(X87Cold {
                            sites: cold_sites,
                            resume,
                            top_known: tk,
                            top_dirty: td,
                            sw_top_dirty: std,
                            kind: X87ColdKind::FldF80,
                        });
                    }
                }
            }
            X87_FILD => {
                self.flush_flags(self.live[k]);
                self.fs.host = 0;
                let bits = (op.w as u32) * 8;
                self.load_op(k, T6, op, bits);
                if bits < 64 {
                    self.a.sbfm(true, T6, T6, 0, bits - 1);
                }
                let (tk, td, std) = (self.x87_top_known, self.x87_top_dirty, self.x87_sw_top_dirty);
                let mut cold_sites = Vec::new();
                self.load_x87_state();
                self.emit_slot_ptr(XA, 7, T2, T3);
                self.emit_test_valid(XA, T3, T4, T5);
                cold_sites.push(self.a.bcond(NE)); // overflow -> cold
                // Convert exact i64 in T6 -> (T0=sig, T1=se)
                let nonzero = self.a.cbnz(true, T6);
                self.a.movz(true, T0, 0, 0);
                self.a.movz(true, T1, 0, 0);
                let conv_done = self.a.b();
                self.a.bind(nonzero);
                self.a.addsub_ri(true, true, true, ZR, T6, 0, false);
                self.a.cset(T1, LT); // sign = (val < 0)
                self.a.cneg(true, T0, T6, LT); // mag = val.unsigned_abs() (exact for i64::MIN!)
                self.a.clz(true, T3, T0);
                self.a.shiftv(true, 8, T0, T0, T3); // sig = mag << lz
                self.a.movz(false, XS, 16383 + 63, 0); // 0x403E
                self.a.addsub_rr(false, true, false, XS, XS, T3, 0);
                self.a.logic_rr(false, 1, T1, XS, T1, 0, 15);
                self.a.bind(conv_done);
                self.a.ldr(16, T3, XA, OFF_X87_SW);
                self.a.logic_ri(false, 0, T3, T3, enc_logic_imm(!(SW_C1 as u64), false).unwrap());
                self.a.str(16, T3, XA, OFF_X87_SW);
                self.a.stp_x(T0, T1, T2, 0);
                self.a.logic_rr(false, 1, T4, T4, T5, 0, 0);
                self.a.str(8, T4, XA, OFF_X87_VALID);
                self.emit_step_top(XA, 7, T3);
                let resume = self.a.pos;
                self.x87_colds.push(X87Cold {
                    sites: cold_sites,
                    resume,
                    top_known: tk,
                    top_dirty: td,
                    sw_top_dirty: std,
                    kind: X87ColdKind::FildI64,
                });
            }
            X87_FST_ST => {
                self.flush_flags(self.live[k]);
                self.fs.host = 0;
                let pop = op.w != 0;
                let (tk, td, std) = (self.x87_top_known, self.x87_top_dirty, self.x87_sw_top_dirty);
                let mut cold_sites = Vec::new();
                self.load_x87_state();
                self.emit_slot_ptr(XA, 0, T1, T2); // p0
                self.emit_slot_ptr(XA, op.r0, T3, T4); // pi
                self.emit_test_valid(XA, T2, T5, T0);
                cold_sites.push(self.a.bcond(EQ)); // ST(0) empty -> cold
                self.a.ldr(16, T6, XA, OFF_X87_SW);
                self.a.logic_ri(false, 0, T6, T6, enc_logic_imm(!(SW_C1 as u64), false).unwrap());
                self.a.str(16, T6, XA, OFF_X87_SW);
                if op.r0 != 0 {
                    self.a.ldr(128, 0, T1, 0);
                    self.a.str(128, 0, T3, 0);
                    self.a.movz(false, T6, 1, 0);
                    self.a.shiftv(false, 8, T6, T6, T4);
                    self.a.logic_rr(false, 1, T5, T5, T6, 0, 0);
                }
                if pop {
                    // Clear p0 valid bit (even if dst == 0, matching pop_to_st!)
                    self.a.bic_rr(false, T5, T5, T0);
                    self.a.str(8, T5, XA, OFF_X87_VALID);
                    self.emit_step_top(XA, 1, T6);
                } else if op.r0 != 0 {
                    self.a.str(8, T5, XA, OFF_X87_VALID);
                }
                let resume = self.a.pos;
                self.x87_colds.push(X87Cold {
                    sites: cold_sites,
                    resume,
                    top_known: tk,
                    top_dirty: td,
                    sw_top_dirty: std,
                    kind: X87ColdKind::FstSt { idx: op.r0, pop },
                });
            }
            X87_FST_MEM => {
                self.flush_flags(self.live[k]);
                self.fs.host = 0;
                let pop = op.r1 != 0;
                let sz = op.w;
                // Range-check destination address BEFORE mutating any x87 state!
                let a = self.addr_x87(op);
                self.range_check_span(k, a, sz as u32, T0);
                self.a.mov_rr(true, T6, a); // save checked address in T6 for hot store
                let (tk, td, std) = (self.x87_top_known, self.x87_top_dirty, self.x87_sw_top_dirty);
                let mut cold_sites = Vec::new();
                self.load_x87_state();
                self.emit_slot_ptr(XA, 0, T0, T1);
                self.emit_test_valid(XA, T1, T2, T3);
                cold_sites.push(self.a.bcond(EQ)); // empty -> cold
                self.a.ldp_x(T4, T5, T0, 0); // T4=sig, T5=se
                match sz {
                    8 => {
                        emit_f80_to_f64_exact(&mut self.a, T4, T5, T0, T1, XS, &mut cold_sites);
                        if mem::tso() {
                            self.a.dmb_ishst();
                        }
                        self.a.str(64, T0, T6, 0);
                    }
                    4 => {
                        emit_f80_to_f32_exact(&mut self.a, T4, T5, T0, T1, XS, &mut cold_sites);
                        if mem::tso() {
                            self.a.dmb_ishst();
                        }
                        self.a.str(32, T0, T6, 0);
                    }
                    _ => {
                        // m80fp (10 bytes): store raw 80-bit value directly!
                        if mem::tso() {
                            self.a.dmb_ishst();
                        }
                        self.a.str(64, T4, T6, 0);
                        self.a.str(16, T5, T6, 8);
                    }
                }
                // Clear C1 (and pop if fstp)
                self.a.ldr(16, T0, XA, OFF_X87_SW);
                self.a.logic_ri(false, 0, T0, T0, enc_logic_imm(!(SW_C1 as u64), false).unwrap());
                self.a.str(16, T0, XA, OFF_X87_SW);
                if pop {
                    self.a.bic_rr(false, T2, T2, T3);
                    self.a.str(8, T2, XA, OFF_X87_VALID);
                    self.emit_step_top(XA, 1, T0);
                }
                let resume = self.a.pos;
                self.x87_colds.push(X87Cold {
                    sites: cold_sites,
                    resume,
                    top_known: tk,
                    top_dirty: td,
                    sw_top_dirty: std,
                    kind: X87ColdKind::FstMem { op: *op, sz, pop },
                });
            }
            X87_FIST => {
                self.flush_flags(self.live[k]);
                self.fs.host = 0;
                let sz = op.w as u32;
                let bits = sz * 8;
                let pop = (op.r1 & 1) != 0;
                let trunc = (op.r1 & 2) != 0;
                // Range-check destination address BEFORE mutating x87 state!
                let a = self.addr_x87(op);
                self.range_check_span(k, a, sz, T0);
                self.a.mov_rr(true, T6, a);
                let (tk, td, std) = (self.x87_top_known, self.x87_top_dirty, self.x87_sw_top_dirty);
                let mut cold_sites = Vec::new();
                self.load_x87_state();
                self.emit_slot_ptr(XA, 0, T0, T1);
                self.emit_test_valid(XA, T1, T2, T3);
                cold_sites.push(self.a.bcond(EQ)); // empty -> cold
                self.a.ldp_x(T4, T5, T0, 0);
                emit_f80_to_f64_exact(&mut self.a, T4, T5, T0, T1, XS, &mut cold_sites);
                self.a.fmov_gf(true, 0, T0); // d0 = exact f64
                if trunc {
                    // fisttp: round toward zero (does not depend on CW.RC or FPCR)
                    self.a.s_frint(true, 3, 1, 0); // frintz d1, d0
                } else {
                    // fist/fistp: round according to CW.RC (isolated from MXCSR!)
                    self.a.ldr(16, T0, XA, OFF_X87_CW);
                    emit_cw_to_fpcr(&mut self.a, T0, T1, XS);
                    self.a.mrs_fpcr(T4);
                    self.a.addsub_rr(true, true, true, ZR, T4, T1, 0);
                    let same1 = self.a.bcond(EQ);
                    self.a.msr_fpcr(T1);
                    self.a.bind(same1);
                    self.a.s_frint(true, 4, 1, 0); // frinti d1, d0
                    self.a.addsub_rr(true, true, true, ZR, T4, T1, 0);
                    let same2 = self.a.bcond(EQ);
                    self.a.msr_fpcr(T4);
                    self.a.bind(same2);
                }
                // Check exact integer range [-2^(bits-1), +2^(bits-1)) before fcvtzs!
                let lim_hi16: u32 = match bits {
                    16 => 0x40E0, // +32768.0
                    32 => 0x41E0, // +2^31
                    _ => 0x43E0,  // +2^63
                };
                self.a.movz(true, T0, lim_hi16, 3);
                self.a.fmov_gf(true, 2, T0);
                self.a.fcmp(true, 1, 2);
                cold_sites.push(self.a.bcond(GE)); // d1 >= +2^(bits-1) -> overflow -> cold
                self.a.s_fneg(true, 2, 2);
                self.a.fcmp(true, 1, 2);
                cold_sites.push(self.a.bcond(LT)); // d1 < -2^(bits-1) -> overflow -> cold
                // Detect inexact rounding: d1 != d0 => SW_PE (0x20)
                self.a.fcmp(true, 1, 0);
                self.a.cset(T5, NE);
                // Convert to signed integer and store
                self.a.fcvtzs_d(true, T0, 1);
                if mem::tso() {
                    self.a.dmb_ishst();
                }
                self.a.str(bits, T0, T6, 0);
                // Update SW (clear C1, merge PE) and optionally pop
                self.a.ldr(16, T0, XA, OFF_X87_SW);
                self.a.logic_ri(false, 0, T0, T0, enc_logic_imm(!(SW_C1 as u64), false).unwrap());
                self.a.logic_rr(false, 1, T0, T0, T5, 0, 5); // PE = bit 5 (0x20)
                self.a.str(16, T0, XA, OFF_X87_SW);
                if pop {
                    self.a.bic_rr(false, T2, T2, T3);
                    self.a.str(8, T2, XA, OFF_X87_VALID);
                    self.emit_step_top(XA, 1, T0);
                }
                let resume = self.a.pos;
                self.x87_colds.push(X87Cold {
                    sites: cold_sites,
                    resume,
                    top_known: tk,
                    top_dirty: td,
                    sw_top_dirty: std,
                    kind: X87ColdKind::Fist { op: *op, bits, flags: op.r1 },
                });
            }
            X87_BINOP_ST => {
                self.emit_x87_binop_st(k, op);
            }
            X87_BINOP_MEM => {
                self.emit_x87_binop_mem(k, op);
            }
            X87_UNARY => {
                match op.r0 {
                    X87_UNARY_SQRT => self.emit_x87_fsqrt(k),
                    X87_UNARY_RNDINT => self.emit_x87_frndint(k),
                    _ => {
                        // Transcendentals & special ops: fsin/fcos/fsincos/fptan/fpatan/
                        // f2xm1/fyl2x/fyl2xp1/fscale/fxtract/fprem/fprem1 -> native Rust helper.
                        self.flush_x87_top();
                        self.a.movz(false, T1, op.r0 as u32, 0);
                        self.emit_call_x87_helper(H_UNARY);
                        // Some unary ops push/pop conditionally (fptan, fsincos, fxtract, fpatan, fyl2x*).
                        self.x87_top_known = None;
                        self.x87_top_dirty = false;
                        self.x87_sw_top_dirty = false;
                    }
                }
            }
            X87_CMP_ST => {
                self.emit_x87_cmp_st(k, op);
            }
            X87_CMP_MEM => {
                self.emit_x87_cmp_mem(k, op);
            }
            X87_FTST => {
                self.emit_x87_ftst(k);
            }
            X87_FCMOV => {
                let cc = FCMOV_CC[op.w as usize & 7];
                let cond = self.cond(cc);
                // Flush any remaining live host flags after evaluating `cond`, since the body uses cmp/tst.
                match cond {
                    Cond::Host(cnd) => {
                        self.a.cset(T6, cnd);
                    }
                    Cond::Reg(r, inv) => {
                        if inv {
                            self.a.logic_ri(false, 2, T6, r, enc_logic_imm(1, false).unwrap());
                        } else {
                            self.a.mov_rr(false, T6, r);
                        }
                    }
                    Cond::Const(b) => {
                        self.a.movz(false, T6, b as u32, 0);
                    }
                }
                self.flush_flags(self.live[k + 1]);
                self.fs.host = 0;
                let (tk, td, std) = (self.x87_top_known, self.x87_top_dirty, self.x87_sw_top_dirty);
                let mut cold_sites = Vec::new();
                self.load_x87_state();
                // Always clear SW.C1 and validate both ST(0) and ST(idx) BEFORE testing cond (T6).
                self.a.ldr(16, T0, XA, OFF_X87_SW);
                self.a.logic_ri(false, 0, T0, T0, enc_logic_imm(!(SW_C1 as u64), false).unwrap());
                self.a.str(16, T0, XA, OFF_X87_SW);
                self.emit_slot_ptr(XA, 0, T1, T2);
                self.emit_slot_ptr(XA, op.r0, T3, T4);
                self.a.ldr(8, T5, XA, OFF_X87_VALID);
                self.a.movz(false, T0, 1, 0);
                self.a.shiftv(false, 8, XS, T0, T2);
                self.a.shiftv(false, 8, T0, T0, T4);
                self.a.logic_rr(false, 1, T0, T0, XS, 0, 0);
                self.a.logic_rr(false, 0, T2, T5, T0, 0, 0);
                self.a.addsub_rr(false, true, true, ZR, T2, T0, 0);
                cold_sites.push(self.a.bcond(NE));
                let skip_move = self.a.cbz(false, T6);
                if op.r0 != 0 {
                    self.a.ldr(128, 0, T3, 0);
                    self.a.str(128, 0, T1, 0);
                }
                self.a.bind(skip_move);
                let resume = self.a.pos;
                self.x87_colds.push(X87Cold {
                    sites: cold_sites,
                    resume,
                    top_known: tk,
                    top_dirty: td,
                    sw_top_dirty: std,
                    kind: X87ColdKind::FcmovFault { idx: op.r0 },
                });
            }
            X87_FBLD => {
                // Range-check [a, a+10), load 10 bytes, call native Rust BCD helper.
                let a = self.addr_x87(op);
                self.range_check_span(k, a, 10, T0);
                self.a.ldr(64, T1, a, 0);
                self.a.ldr(16, T2, a, 8);
                if mem::tso() {
                    self.a.dmb_ishld();
                }
                self.flush_x87_top();
                self.emit_call_x87_helper(H_FBLD);
                if let Some(t) = self.x87_top_known {
                    self.x87_top_known = Some(t.wrapping_add(7) & 7);
                }
            }
            X87_FBSTP => {
                // Range-check [a, a+10) BEFORE mutating x87 state!
                let a = self.addr_x87(op);
                self.range_check_span(k, a, 10, T0);
                self.flush_x87_top();
                self.emit_call_x87_helper(H_FBSTP);
                // Helper returns (x0=lo, x1=hi); recompute guest addr into XA and store!
                let a = self.addr_x87(op);
                if mem::tso() {
                    self.a.dmb_ishst();
                }
                self.a.str(64, T0, a, 0);
                self.a.str(16, T1, a, 8);
                if let Some(t) = self.x87_top_known {
                    self.x87_top_known = Some(t.wrapping_add(1) & 7);
                }
            }
            X87_FENV => {
                let is_save = (op.w & 1) != 0;
                let is_load = (op.w & 2) != 0;
                let size16 = (op.w & 4) != 0;
                let len: u32 = match (is_save, size16) {
                    (false, true) => 14,
                    (false, false) => 28,
                    (true, true) => 94,
                    (true, false) => 108,
                };
                let a = self.addr_x87(op);
                self.range_check_span(k, a, len, T0);
                self.a.mov_rr(true, T1, a);
                self.flush_x87_top();
                self.a.movz(false, T2, size16 as u32, 0);
                let helper = match (is_save, is_load) {
                    (false, true) => H_FLDENV,
                    (false, false) => H_FNSTENV,
                    (true, true) => H_FRSTOR,
                    (true, false) => H_FNSAVE,
                };
                self.emit_call_x87_helper(helper);
                if is_save && !is_load {
                    // fnsave re-initializes x87 (top = 0)
                    self.x87_top_known = Some(0);
                } else if is_load {
                    self.x87_top_known = None;
                }
                self.x87_top_dirty = false;
                self.x87_sw_top_dirty = false;
            }
            _ => unreachable!("invalid K_X87 sub-op {sub}"),
        }
        self.emit_record_x87_env(op);
    }

    /// Update x87 environment metadata (`FIP`, `FOP`, and for memory data instructions `FDP`)
    /// after a data x87 instruction succeeds, matching `regfile_ext::record_x87_environment`.
    /// Preserves `NZCV` (so fused `fcomi`/`fucomi` flags remain live).
    fn emit_record_x87_env(&mut self, op: &FastOp) {
        if (op.imm & fast::X87_ENV_RECORD) == 0 {
            return;
        }
        let fop = (op.imm & 0x7FF) as u32;
        let has_mem = (op.imm & fast::X87_ENV_HAS_MEM) != 0;
        let fdp_reg = if has_mem { Some(self.addr_into(op, T1)) } else { None };
        self.load_x87_state();
        self.mov_lit(T0, op.ip);
        self.a.str(64, T0, XA, OFF_X87_FIP);
        if let Some(r) = fdp_reg {
            self.a.str(64, r, XA, OFF_X87_FDP);
        }
        self.a.movz(false, T0, fop, 0);
        self.a.str(16, T0, XA, OFF_X87_FOP);
    }

    /// Execute one scalar ARM64 FP binary op on `(v0, v1) -> v0` with `FPCR` isolated from
    /// `MXCSR` (using `CW` loaded from `st_reg`), clearing/reading `FPSR`, restoring `FPCR`,
    /// and branching to `cold_sites` if any of `IOC|DZC|OFC|UFC` (`FPSR & 0x0F`) was raised.
    /// Leaves `FPSR` in `fpsr_out`. Scratch: `t1`, `t2`, `t3`.
    fn emit_isolated_fp_binop(
        &mut self,
        st_reg: u32,
        is_f64: bool,
        op_code: u8,
        fpsr_out: u32,
        t1: u32,
        t2: u32,
        t3: u32,
        cold_sites: &mut Vec<usize>,
    ) {
        self.a.ldr(16, t1, st_reg, OFF_X87_CW);
        emit_cw_to_fpcr(&mut self.a, t1, t2, t3);
        self.a.mrs_fpcr(t1);
        self.a.addsub_rr(true, true, true, ZR, t1, t2, 0);
        let same1 = self.a.bcond(EQ);
        self.a.msr_fpcr(t2);
        self.a.bind(same1);
        self.a.msr_fpsr(ZR);
        match op_code {
            X87_OP_ADD => self.a.s_fadd(is_f64, 0, 0, 1),
            X87_OP_SUB => self.a.s_fsub(is_f64, 0, 0, 1),
            X87_OP_SUBR => self.a.s_fsub(is_f64, 0, 1, 0),
            X87_OP_MUL => self.a.s_fmul(is_f64, 0, 0, 1),
            X87_OP_DIV => self.a.s_fdiv(is_f64, 0, 0, 1),
            _ => self.a.s_fdiv(is_f64, 0, 1, 0), // X87_OP_DIVR
        }
        self.a.mrs_fpsr(fpsr_out);
        self.a.addsub_rr(true, true, true, ZR, t1, t2, 0);
        let same2 = self.a.bcond(EQ);
        self.a.msr_fpcr(t1);
        self.a.bind(same2);
        // Any IOC (0), DZC (1), OFC (2), or UFC (3) routes to exact soft-f80 helper!
        self.a.logic_ri(false, 3, ZR, fpsr_out, enc_logic_imm(0x0F, false).unwrap());
        cold_sites.push(self.a.bcond(NE));
    }

    /// Native ARM64 lowering for `fadd/fsub/fsubr/fmul/fdiv/fdivr [p] st(dst), st(src)`.
    /// Emits native `d0`/`d1` (`PC=53`) and `s0`/`s1` (`PC=24`) paths; routes `PC=64` and
    /// exceptional cases to the cold native Rust soft-f80 helper.
    fn emit_x87_binop_st(&mut self, k: usize, op: &FastOp) {
        self.flush_flags(self.live[k]);
        self.fs.host = 0;
        let op_code = op.w & 7;
        let pop = (op.w & 8) != 0;
        let (tk, td, std) = (self.x87_top_known, self.x87_top_dirty, self.x87_sw_top_dirty);
        let mut cold_sites = Vec::new();
        self.load_x87_state();
        // Check CW.PC: 2 = 53-bit (f64 fast path), 0 = 24-bit (f32 fast path), 1|3 = 64-bit -> cold.
        self.a.ldr(16, T0, XA, OFF_X87_CW);
        self.a.ubfm(false, T0, T0, 8, 9);
        self.a.addsub_ri(false, true, true, ZR, T0, 2, false);
        let is_pc53 = self.a.bcond(EQ);
        let is_pc24 = self.a.cbz(false, T0);
        cold_sites.push(self.a.b()); // PC=64 (or reserved 1) -> cold

        // --- PC = 53-bit native ARM64 f64 path ---
        self.a.bind(is_pc53);
        self.emit_slot_ptr(XA, op.r0, T1, T2); // dst
        self.emit_slot_ptr(XA, op.r1, T3, T4); // src
        self.a.ldr(8, T5, XA, OFF_X87_VALID);
        self.a.movz(false, T0, 1, 0);
        self.a.shiftv(false, 8, XS, T0, T2);
        self.a.shiftv(false, 8, T0, T0, T4);
        self.a.logic_rr(false, 1, T0, T0, XS, 0, 0);
        self.a.logic_rr(false, 0, T2, T5, T0, 0, 0);
        self.a.addsub_rr(false, true, true, ZR, T2, T0, 0);
        cold_sites.push(self.a.bcond(NE)); // empty slot -> cold
        // Load & convert dst -> d0, src -> d1
        self.a.ldp_x(T4, T5, T1, 0);
        emit_f80_to_f64_exact(&mut self.a, T4, T5, T0, T2, XS, &mut cold_sites);
        self.a.fmov_gf(true, 0, T0);
        self.a.ldp_x(T4, T5, T3, 0);
        emit_f80_to_f64_exact(&mut self.a, T4, T5, T0, T2, XS, &mut cold_sites);
        self.a.fmov_gf(true, 1, T0);
        self.emit_isolated_fp_binop(XA, true, op_code, T5, T0, T2, XS, &mut cold_sites);
        self.a.fmov_fg(true, T6, 0);
        emit_f64_to_f80(&mut self.a, T6, T0, T2, T3, T4, XS, &mut cold_sites);
        let commit_53 = self.a.b();

        // --- PC = 24-bit native ARM64 f32 path ---
        self.a.bind(is_pc24);
        self.emit_slot_ptr(XA, op.r0, T1, T2); // dst
        self.emit_slot_ptr(XA, op.r1, T3, T4); // src
        self.a.ldr(8, T5, XA, OFF_X87_VALID);
        self.a.movz(false, T0, 1, 0);
        self.a.shiftv(false, 8, XS, T0, T2);
        self.a.shiftv(false, 8, T0, T0, T4);
        self.a.logic_rr(false, 1, T0, T0, XS, 0, 0);
        self.a.logic_rr(false, 0, T2, T5, T0, 0, 0);
        self.a.addsub_rr(false, true, true, ZR, T2, T0, 0);
        cold_sites.push(self.a.bcond(NE));
        self.a.ldp_x(T4, T5, T1, 0);
        emit_f80_to_f32_exact(&mut self.a, T4, T5, T0, T2, XS, &mut cold_sites);
        self.a.fmov_gf(false, 0, T0);
        self.a.ldp_x(T4, T5, T3, 0);
        emit_f80_to_f32_exact(&mut self.a, T4, T5, T0, T2, XS, &mut cold_sites);
        self.a.fmov_gf(false, 1, T0);
        self.emit_isolated_fp_binop(XA, false, op_code, T5, T0, T2, XS, &mut cold_sites);
        self.a.fmov_fg(false, T6, 0);
        emit_f32_to_f80(&mut self.a, T6, T0, T2, T3, T4, XS, &mut cold_sites);

        // --- Shared commit: write (T0=sig, T2=se) to [T1], merge FPSR.IXC (in T5) -> SW.PE, pop ---
        self.a.bind(commit_53);
        self.a.stp_x(T0, T2, T1, 0);
        self.a.ldr(16, T0, XA, OFF_X87_SW);
        self.a.logic_ri(false, 0, T0, T0, enc_logic_imm(!(SW_C1 as u64), false).unwrap());
        self.a.logic_ri(false, 0, T5, T5, enc_logic_imm(0x10, false).unwrap());
        self.a.logic_rr(false, 1, T0, T0, T5, 0, 1); // IXC (bit 4) << 1 = SW_PE (bit 5)
        self.a.str(16, T0, XA, OFF_X87_SW);
        if pop {
            self.emit_phys_idx(XA, 0, T0);
            self.a.ldr(8, T1, XA, OFF_X87_VALID);
            self.a.movz(false, T2, 1, 0);
            self.a.shiftv(false, 8, T2, T2, T0);
            self.a.bic_rr(false, T1, T1, T2);
            self.a.str(8, T1, XA, OFF_X87_VALID);
            self.emit_step_top(XA, 1, T0);
        }
        let resume = self.a.pos;
        self.x87_colds.push(X87Cold {
            sites: cold_sites,
            resume,
            top_known: tk,
            top_dirty: td,
            sw_top_dirty: std,
            kind: X87ColdKind::BinopSt { op_pop: op.w, dst: op.r0, src: op.r1 },
        });
    }

    /// Native ARM64 lowering for `fadd/fsub/fsubr/fmul/fdiv/fdivr m32fp/m64fp` and
    /// `fiadd/fisub/fisubr/fimul/fidiv/fidivr m16int/m32int`.
    fn emit_x87_binop_mem(&mut self, k: usize, op: &FastOp) {
        self.flush_flags(self.live[k]);
        self.fs.host = 0;
        let op_code = op.r0;
        let is_int = (op.w & 0x80) != 0;
        let sz = op.w & 0x7F;
        let bits = (sz as u32) * 8;
        self.load_op(k, T6, op, bits);
        if is_int {
            self.a.sbfm(true, T6, T6, 0, bits - 1);
        }
        let (tk, td, std) = (self.x87_top_known, self.x87_top_dirty, self.x87_sw_top_dirty);
        let mut cold_sites = Vec::new();
        self.load_x87_state();
        // Fast path for PC == 53-bit (Win64 default); PC == 24/64 -> cold soft-f80 helper.
        self.a.ldr(16, T0, XA, OFF_X87_CW);
        self.a.ubfm(false, T0, T0, 8, 9);
        self.a.addsub_ri(false, true, true, ZR, T0, 2, false);
        cold_sites.push(self.a.bcond(NE));
        // Validate rhs (must be finite non-subnormal if float; int16/int32 always exact in f64!)
        if is_int {
            self.a.scvtf_gp(true, true, 1, T6); // d1 = (f64)(i64)rhs (exact for i16/i32!)
        } else if sz == 8 {
            // Check f64 rhs is normal or exact zero (reject subnormal/Inf/NaN -> cold for DE/IE)
            self.a.ubfx(T0, T6, 52, 11);
            self.a.addsub_ri(false, true, true, ZR, T0, 0x7FF, false);
            cold_sites.push(self.a.bcond(EQ));
            let ok_r = self.a.cbnz(false, T0);
            self.a.lsl_i(true, T0, T6, 1);
            cold_sites.push(self.a.cbnz(true, T0)); // subnormal -> cold
            self.a.bind(ok_r);
            self.a.fmov_gf(true, 1, T6);
        } else {
            // f32 rhs: check normal or exact zero, widen to d1 via fcvt
            self.a.ubfm(false, T0, T6, 23, 30);
            self.a.addsub_ri(false, true, true, ZR, T0, 0xFF, false);
            cold_sites.push(self.a.bcond(EQ));
            let ok_r = self.a.cbnz(false, T0);
            self.a.lsl_i(false, T0, T6, 1);
            cold_sites.push(self.a.cbnz(false, T0)); // subnormal -> cold
            self.a.bind(ok_r);
            self.a.fmov_gf(false, 1, T6);
            self.a.fcvt_d_s(1, 1);
        }
        // Check ST(0) valid & exact f64
        self.emit_slot_ptr(XA, 0, T1, T2);
        self.emit_test_valid(XA, T2, T3, T4);
        cold_sites.push(self.a.bcond(EQ));
        self.a.ldp_x(T4, T5, T1, 0);
        emit_f80_to_f64_exact(&mut self.a, T4, T5, T0, T2, XS, &mut cold_sites);
        self.a.fmov_gf(true, 0, T0);
        // Execute isolated native ARM64 f64 op -> d0, FPSR in T5.
        // An exact f64 subnormal result (e.g. 2^-1022 * 0.5 = 2^-1023) does not set FPSR.UFC,
        // so verify `d0` is normal or ±0.0 in T0 BEFORE overwriting T6 (which holds the loaded
        // memory RHS required by `X87ColdKind::BinopMem`).
        self.emit_isolated_fp_binop(XA, true, op_code, T5, T0, T2, XS, &mut cold_sites);
        self.a.fmov_fg(true, T0, 0);
        self.a.ubfx(T2, T0, 52, 11);
        self.a.addsub_ri(false, true, true, ZR, T2, 0x7FF, false);
        cold_sites.push(self.a.bcond(EQ));
        let res_norm = self.a.cbnz(false, T2);
        self.a.lsl_i(true, T2, T0, 1);
        cold_sites.push(self.a.cbnz(true, T2)); // exact subnormal -> cold with T6 intact!
        self.a.bind(res_norm);
        // Past this point `emit_f64_to_f80` cannot branch to cold; T6 is safe to reuse.
        self.a.mov_rr(true, T6, T0);
        emit_f64_to_f80(&mut self.a, T6, T0, T2, T3, T4, XS, &mut cold_sites);
        self.a.stp_x(T0, T2, T1, 0);
        self.a.ldr(16, T0, XA, OFF_X87_SW);
        self.a.logic_ri(false, 0, T0, T0, enc_logic_imm(!(SW_C1 as u64), false).unwrap());
        self.a.logic_ri(false, 0, T5, T5, enc_logic_imm(0x10, false).unwrap());
        self.a.logic_rr(false, 1, T0, T0, T5, 0, 1);
        self.a.str(16, T0, XA, OFF_X87_SW);
        let resume = self.a.pos;
        self.x87_colds.push(X87Cold {
            sites: cold_sites,
            resume,
            top_known: tk,
            top_dirty: td,
            sw_top_dirty: std,
            kind: X87ColdKind::BinopMem { op_code, w: op.w },
        });
    }

    /// Native ARM64 lowering for `fsqrt` (`d0` for `PC=53`, `s0` for `PC=24`, cold soft-f80 for `PC=64`).
    fn emit_x87_fsqrt(&mut self, k: usize) {
        self.flush_flags(self.live[k]);
        self.fs.host = 0;
        let (tk, td, std) = (self.x87_top_known, self.x87_top_dirty, self.x87_sw_top_dirty);
        let mut cold_sites = Vec::new();
        self.load_x87_state();
        self.a.ldr(16, T0, XA, OFF_X87_CW);
        self.a.ubfm(false, T0, T0, 8, 9);
        self.a.addsub_ri(false, true, true, ZR, T0, 2, false);
        let is_pc53 = self.a.bcond(EQ);
        let is_pc24 = self.a.cbz(false, T0);
        cold_sites.push(self.a.b());

        // PC = 53-bit
        self.a.bind(is_pc53);
        self.emit_slot_ptr(XA, 0, T1, T2);
        self.emit_test_valid(XA, T2, T3, T4);
        cold_sites.push(self.a.bcond(EQ));
        self.a.ldp_x(T4, T5, T1, 0);
        emit_f80_to_f64_exact(&mut self.a, T4, T5, T0, T2, XS, &mut cold_sites);
        self.a.fmov_gf(true, 0, T0);
        self.a.ldr(16, T0, XA, OFF_X87_CW);
        emit_cw_to_fpcr(&mut self.a, T0, T2, XS);
        self.a.mrs_fpcr(T0);
        self.a.addsub_rr(true, true, true, ZR, T0, T2, 0);
        let s1 = self.a.bcond(EQ);
        self.a.msr_fpcr(T2);
        self.a.bind(s1);
        self.a.msr_fpsr(ZR);
        self.a.s_fsqrt(true, 0, 0);
        self.a.mrs_fpsr(T5);
        self.a.addsub_rr(true, true, true, ZR, T0, T2, 0);
        let s2 = self.a.bcond(EQ);
        self.a.msr_fpcr(T0);
        self.a.bind(s2);
        self.a.logic_ri(false, 3, ZR, T5, enc_logic_imm(0x0F, false).unwrap());
        cold_sites.push(self.a.bcond(NE));
        self.a.fmov_fg(true, T6, 0);
        emit_f64_to_f80(&mut self.a, T6, T0, T2, T3, T4, XS, &mut cold_sites);
        let commit_53 = self.a.b();

        // PC = 24-bit
        self.a.bind(is_pc24);
        self.emit_slot_ptr(XA, 0, T1, T2);
        self.emit_test_valid(XA, T2, T3, T4);
        cold_sites.push(self.a.bcond(EQ));
        self.a.ldp_x(T4, T5, T1, 0);
        emit_f80_to_f32_exact(&mut self.a, T4, T5, T0, T2, XS, &mut cold_sites);
        self.a.fmov_gf(false, 0, T0);
        self.a.ldr(16, T0, XA, OFF_X87_CW);
        emit_cw_to_fpcr(&mut self.a, T0, T2, XS);
        self.a.mrs_fpcr(T0);
        self.a.addsub_rr(true, true, true, ZR, T0, T2, 0);
        let s3 = self.a.bcond(EQ);
        self.a.msr_fpcr(T2);
        self.a.bind(s3);
        self.a.msr_fpsr(ZR);
        self.a.s_fsqrt(false, 0, 0);
        self.a.mrs_fpsr(T5);
        self.a.addsub_rr(true, true, true, ZR, T0, T2, 0);
        let s4 = self.a.bcond(EQ);
        self.a.msr_fpcr(T0);
        self.a.bind(s4);
        self.a.logic_ri(false, 3, ZR, T5, enc_logic_imm(0x0F, false).unwrap());
        cold_sites.push(self.a.bcond(NE));
        self.a.fmov_fg(false, T6, 0);
        emit_f32_to_f80(&mut self.a, T6, T0, T2, T3, T4, XS, &mut cold_sites);

        self.a.bind(commit_53);
        self.a.stp_x(T0, T2, T1, 0);
        self.a.ldr(16, T0, XA, OFF_X87_SW);
        self.a.logic_ri(false, 0, T0, T0, enc_logic_imm(!(SW_C1 as u64), false).unwrap());
        self.a.logic_ri(false, 0, T5, T5, enc_logic_imm(0x10, false).unwrap());
        self.a.logic_rr(false, 1, T0, T0, T5, 0, 1);
        self.a.str(16, T0, XA, OFF_X87_SW);
        let resume = self.a.pos;
        self.x87_colds.push(X87Cold {
            sites: cold_sites,
            resume,
            top_known: tk,
            top_dirty: td,
            sw_top_dirty: std,
            kind: X87ColdKind::Unary { sub: X87_UNARY_SQRT },
        });
    }

    /// Native ARM64 lowering for `frndint` using `frinti d1, d0` under isolated x87 `FPCR.RMode`.
    fn emit_x87_frndint(&mut self, k: usize) {
        self.flush_flags(self.live[k]);
        self.fs.host = 0;
        let (tk, td, std) = (self.x87_top_known, self.x87_top_dirty, self.x87_sw_top_dirty);
        let mut cold_sites = Vec::new();
        self.load_x87_state();
        self.emit_slot_ptr(XA, 0, T1, T2);
        self.emit_test_valid(XA, T2, T3, T4);
        cold_sites.push(self.a.bcond(EQ));
        self.a.ldp_x(T4, T5, T1, 0);
        emit_f80_to_f64_exact(&mut self.a, T4, T5, T0, T2, XS, &mut cold_sites);
        self.a.fmov_gf(true, 0, T0);
        self.a.ldr(16, T0, XA, OFF_X87_CW);
        emit_cw_to_fpcr(&mut self.a, T0, T2, XS);
        self.a.mrs_fpcr(T0);
        self.a.addsub_rr(true, true, true, ZR, T0, T2, 0);
        let s1 = self.a.bcond(EQ);
        self.a.msr_fpcr(T2);
        self.a.bind(s1);
        self.a.s_frint(true, 4, 1, 0); // frinti d1, d0
        self.a.addsub_rr(true, true, true, ZR, T0, T2, 0);
        let s2 = self.a.bcond(EQ);
        self.a.msr_fpcr(T0);
        self.a.bind(s2);
        self.a.fcmp(true, 1, 0);
        self.a.cset(T5, NE); // PE = (d1 != d0)
        self.a.fmov_fg(true, T6, 1);
        emit_f64_to_f80(&mut self.a, T6, T0, T2, T3, T4, XS, &mut cold_sites);
        self.a.stp_x(T0, T2, T1, 0);
        self.a.ldr(16, T0, XA, OFF_X87_SW);
        self.a.logic_ri(false, 0, T0, T0, enc_logic_imm(!(SW_C1 as u64), false).unwrap());
        self.a.logic_rr(false, 1, T0, T0, T5, 0, 5);
        self.a.str(16, T0, XA, OFF_X87_SW);
        let resume = self.a.pos;
        self.x87_colds.push(X87Cold {
            sites: cold_sites,
            resume,
            top_known: tk,
            top_dirty: td,
            sw_top_dirty: std,
            kind: X87ColdKind::Unary { sub: X87_UNARY_RNDINT },
        });
    }

    /// Native ARM64 lowering for `fcom/fcomp/fcompp/fucom/fucomp/fucompp/fcomi/fcomip/fucomi/fucomip`.
    fn emit_x87_cmp_st(&mut self, k: usize, op: &FastOp) {
        let write_sw = (op.w & 2) != 0;
        let pops = (op.w >> 2) & 3;
        let is_fcomi = (op.w & 16) != 0;
        if !is_fcomi {
            self.flush_flags(self.live[k]);
            self.fs.host = 0;
        }
        let (tk, td, std) = (self.x87_top_known, self.x87_top_dirty, self.x87_sw_top_dirty);
        let mut cold_sites = Vec::new();
        self.load_x87_state();
        self.emit_slot_ptr(XA, 0, T1, T2);
        self.emit_slot_ptr(XA, op.r0, T3, T4);
        self.a.ldr(8, T5, XA, OFF_X87_VALID);
        self.a.movz(false, T0, 1, 0);
        self.a.shiftv(false, 8, XS, T0, T2);
        self.a.shiftv(false, 8, T0, T0, T4);
        self.a.logic_rr(false, 1, T0, T0, XS, 0, 0);
        self.a.logic_rr(false, 0, T6, T5, T0, 0, 0);
        self.a.addsub_rr(false, true, true, ZR, T6, T0, 0);
        cold_sites.push(self.a.bcond(NE)); // empty slot -> cold
        self.a.ldp_x(T4, T6, T1, 0);
        emit_f80_to_f64_exact(&mut self.a, T4, T6, T0, T2, XS, &mut cold_sites);
        self.a.fmov_gf(true, 0, T0);
        self.a.ldp_x(T4, T6, T3, 0);
        emit_f80_to_f64_exact(&mut self.a, T4, T6, T0, T2, XS, &mut cold_sites);
        self.a.fmov_gf(true, 1, T0);
        // Perform pop(s) and SW update BEFORE final `fcmp` if `is_fcomi` so NZCV stays live!
        if pops > 0 {
            self.emit_phys_idx(XA, 0, T0);
            self.a.movz(false, T2, 1, 0);
            self.a.shiftv(false, 8, T2, T2, T0);
            self.a.bic_rr(false, T5, T5, T2);
            if pops == 2 {
                self.emit_phys_idx(XA, 1, T0);
                self.a.movz(false, T2, 1, 0);
                self.a.shiftv(false, 8, T2, T2, T0);
                self.a.bic_rr(false, T5, T5, T2);
            }
            self.a.str(8, T5, XA, OFF_X87_VALID);
            self.emit_step_top(XA, pops, T0);
        }
        if is_fcomi {
            // Clear SW_C1, then `fcmp d0, d1` produces exact `Src::Fcmp` NZCV!
            self.a.ldr(16, T0, XA, OFF_X87_SW);
            self.a.logic_ri(false, 0, T0, T0, enc_logic_imm(!(SW_C1 as u64), false).unwrap());
            self.a.str(16, T0, XA, OFF_X87_SW);
            self.a.fcmp(true, 0, 1);
            let resume = self.a.pos;
            self.x87_colds.push(X87Cold {
                sites: cold_sites,
                resume,
                top_known: tk,
                top_dirty: td,
                sw_top_dirty: std,
                kind: X87ColdKind::CmpSt { idx: op.r0, flags: op.w, is_fcomi: true },
            });
            let la = self.live[k + 1];
            self.pf = PF_NONE;
            if la & FALL != 0 {
                self.fs = FlagState { src: Src::Fcmp, host: la & FALL, mem: self.fs.mem & !FALL };
            } else {
                self.fs = FlagState { src: Src::None, host: 0, mem: self.fs.mem & !FALL };
            }
        } else {
            debug_assert!(write_sw);
            self.a.fcmp(true, 0, 1);
            // GT: 0, LT: SW_C0 (0x0100), EQ: SW_C3 (0x4000) (both finite -> never unordered)
            self.a.movz(false, T1, 0, 0);
            self.a.movz(false, T2, SW_C0 as u32, 0);
            self.a.csel(false, T1, T2, T1, LT);
            self.a.movz(false, T2, SW_C3 as u32, 0);
            self.a.csel(false, T1, T2, T1, EQ);
            self.a.ldr(16, T0, XA, OFF_X87_SW);
            self.a.movz(false, T2, (SW_C0 | SW_C1 | SW_C2 | SW_C3) as u32, 0);
            self.a.bic_rr(false, T0, T0, T2);
            self.a.logic_rr(false, 1, T0, T0, T1, 0, 0);
            self.a.str(16, T0, XA, OFF_X87_SW);
            let resume = self.a.pos;
            self.x87_colds.push(X87Cold {
                sites: cold_sites,
                resume,
                top_known: tk,
                top_dirty: td,
                sw_top_dirty: std,
                kind: X87ColdKind::CmpSt { idx: op.r0, flags: op.w, is_fcomi: false },
            });
        }
    }

    /// Native ARM64 lowering for `fcom/fcomp m32fp/m64fp` and `ficom/ficomp m16int/m32int`.
    fn emit_x87_cmp_mem(&mut self, k: usize, op: &FastOp) {
        self.flush_flags(self.live[k]);
        self.fs.host = 0;
        let is_int = (op.w & 0x80) != 0;
        let sz = op.w & 0x7F;
        let bits = (sz as u32) * 8;
        let pops = (op.r1 >> 2) & 3;
        self.load_op(k, T6, op, bits);
        if is_int {
            self.a.sbfm(true, T6, T6, 0, bits - 1);
        }
        let (tk, td, std) = (self.x87_top_known, self.x87_top_dirty, self.x87_sw_top_dirty);
        let mut cold_sites = Vec::new();
        if is_int {
            self.a.scvtf_gp(true, true, 1, T6);
        } else if sz == 8 {
            self.a.ubfx(T0, T6, 52, 11);
            self.a.addsub_ri(false, true, true, ZR, T0, 0x7FF, false);
            cold_sites.push(self.a.bcond(EQ));
            let ok_r = self.a.cbnz(false, T0);
            self.a.lsl_i(true, T0, T6, 1);
            cold_sites.push(self.a.cbnz(true, T0));
            self.a.bind(ok_r);
            self.a.fmov_gf(true, 1, T6);
        } else {
            self.a.ubfm(false, T0, T6, 23, 30);
            self.a.addsub_ri(false, true, true, ZR, T0, 0xFF, false);
            cold_sites.push(self.a.bcond(EQ));
            let ok_r = self.a.cbnz(false, T0);
            self.a.lsl_i(false, T0, T6, 1);
            cold_sites.push(self.a.cbnz(false, T0));
            self.a.bind(ok_r);
            self.a.fmov_gf(false, 1, T6);
            self.a.fcvt_d_s(1, 1);
        }
        self.load_x87_state();
        self.emit_slot_ptr(XA, 0, T1, T2);
        self.emit_test_valid(XA, T2, T3, T4);
        cold_sites.push(self.a.bcond(EQ));
        self.a.ldp_x(T4, T5, T1, 0);
        emit_f80_to_f64_exact(&mut self.a, T4, T5, T0, T2, XS, &mut cold_sites);
        self.a.fmov_gf(true, 0, T0);
        self.a.fcmp(true, 0, 1);
        self.a.movz(false, T1, 0, 0);
        self.a.movz(false, T2, SW_C0 as u32, 0);
        self.a.csel(false, T1, T2, T1, LT);
        self.a.movz(false, T2, SW_C3 as u32, 0);
        self.a.csel(false, T1, T2, T1, EQ);
        self.a.ldr(16, T0, XA, OFF_X87_SW);
        self.a.movz(false, T2, (SW_C0 | SW_C1 | SW_C2 | SW_C3) as u32, 0);
        self.a.bic_rr(false, T0, T0, T2);
        self.a.logic_rr(false, 1, T0, T0, T1, 0, 0);
        self.a.str(16, T0, XA, OFF_X87_SW);
        if pops > 0 {
            self.emit_phys_idx(XA, 0, T0);
            self.a.movz(false, T2, 1, 0);
            self.a.shiftv(false, 8, T2, T2, T0);
            self.a.bic_rr(false, T3, T3, T2);
            self.a.str(8, T3, XA, OFF_X87_VALID);
            self.emit_step_top(XA, 1, T0);
        }
        let resume = self.a.pos;
        self.x87_colds.push(X87Cold {
            sites: cold_sites,
            resume,
            top_known: tk,
            top_dirty: td,
            sw_top_dirty: std,
            kind: X87ColdKind::CmpMem { w: op.w, flags: op.r1 },
        });
    }

    /// Native ARM64 lowering for `ftst` (`ST(0)` vs `0.0`).
    fn emit_x87_ftst(&mut self, k: usize) {
        self.flush_flags(self.live[k]);
        self.fs.host = 0;
        let (tk, td, std) = (self.x87_top_known, self.x87_top_dirty, self.x87_sw_top_dirty);
        let mut cold_sites = Vec::new();
        self.load_x87_state();
        self.emit_slot_ptr(XA, 0, T1, T2);
        self.emit_test_valid(XA, T2, T3, T4);
        cold_sites.push(self.a.bcond(EQ));
        self.a.ldp_x(T4, T5, T1, 0);
        emit_f80_to_f64_exact(&mut self.a, T4, T5, T0, T2, XS, &mut cold_sites);
        self.a.fmov_gf(true, 0, T0);
        self.a.fcmp_zero(true, 0);
        self.a.movz(false, T1, 0, 0);
        self.a.movz(false, T2, SW_C0 as u32, 0);
        self.a.csel(false, T1, T2, T1, LT);
        self.a.movz(false, T2, SW_C3 as u32, 0);
        self.a.csel(false, T1, T2, T1, EQ);
        self.a.ldr(16, T0, XA, OFF_X87_SW);
        self.a.movz(false, T2, (SW_C0 | SW_C1 | SW_C2 | SW_C3) as u32, 0);
        self.a.bic_rr(false, T0, T0, T2);
        self.a.logic_rr(false, 1, T0, T0, T1, 0, 0);
        self.a.str(16, T0, XA, OFF_X87_SW);
        let resume = self.a.pos;
        self.x87_colds.push(X87Cold {
            sites: cold_sites,
            resume,
            top_known: tk,
            top_dirty: td,
            sw_top_dirty: std,
            kind: X87ColdKind::Ftst,
        });
    }

    /// Emit all deferred x87 cold stubs AND the block-local `x87_trampoline` (if any helper
    /// call was emitted), patching every `bl x87_trampoline` site.
    pub(super) fn emit_x87_colds(&mut self) {
        let mut colds = core::mem::take(&mut self.x87_colds);
        for c in &colds {
            if c.sites.is_empty() {
                continue;
            }
            let stub_pos = self.a.pos;
            for &site in &c.sites {
                self.a.patch(site, stub_pos);
            }
            // Flush pre-op static TOP state into X87State before calling the Rust helper.
            // (Preserve T5 and T6 because `FldF32`/`FldF64`/`FldF80`/`FildI64`/`BinopMem`/`CmpMem`
            // pass loaded memory operands in T5/T6!)
            if c.top_dirty || c.sw_top_dirty {
                self.load_x87_state();
                emit_flush_top_raw(&mut self.a, XA, XS, T0, c.top_known, c.top_dirty, c.sw_top_dirty);
            }
            match c.kind {
                X87ColdKind::FldF32 => {
                    self.a.mov_rr(false, T1, T6);
                    self.emit_call_x87_helper(H_FLD_F32);
                }
                X87ColdKind::FldF64 => {
                    self.a.mov_rr(true, T1, T6);
                    self.emit_call_x87_helper(H_FLD_F64);
                }
                X87ColdKind::FldF80 => {
                    self.a.mov_rr(true, T1, T5);
                    self.a.mov_rr(true, T2, T6);
                    self.emit_call_x87_helper(H_FLD_F80);
                }
                X87ColdKind::FildI64 => {
                    self.a.mov_rr(true, T1, T6);
                    self.emit_call_x87_helper(H_FILD_I64);
                }
                X87ColdKind::FldSt { idx } => {
                    self.a.movz(false, T1, idx as u32, 0);
                    self.emit_call_x87_helper(H_FLD_ST);
                }
                X87ColdKind::FldConst { id } => {
                    self.a.movz(false, T1, id as u32, 0);
                    self.emit_call_x87_helper(H_FLD_CONST);
                }
                X87ColdKind::FstSt { idx, pop } => {
                    self.a.movz(false, T1, idx as u32, 0);
                    self.a.movz(false, T2, pop as u32, 0);
                    self.emit_call_x87_helper(H_FST_ST);
                }
                X87ColdKind::FstMem { op, sz, pop } => {
                    match sz {
                        4 => {
                            self.a.movz(false, T1, pop as u32, 0);
                            self.emit_call_x87_helper(H_FST_F32);
                            self.a.mov_rr(false, T4, T0);
                            let a = self.addr_x87(&op);
                            if mem::tso() {
                                self.a.dmb_ishst();
                            }
                            self.a.str(32, T4, a, 0);
                        }
                        8 => {
                            self.a.movz(false, T1, pop as u32, 0);
                            self.emit_call_x87_helper(H_FST_F64);
                            self.a.mov_rr(true, T4, T0);
                            let a = self.addr_x87(&op);
                            if mem::tso() {
                                self.a.dmb_ishst();
                            }
                            self.a.str(64, T4, a, 0);
                        }
                        _ => {
                            self.emit_call_x87_helper(H_FSTP_F80);
                            self.a.mov_rr(true, T4, T0);
                            self.a.mov_rr(true, T5, T1);
                            let a = self.addr_x87(&op);
                            if mem::tso() {
                                self.a.dmb_ishst();
                            }
                            self.a.str(64, T4, a, 0);
                            self.a.str(16, T5, a, 8);
                        }
                    }
                }
                X87ColdKind::Fist { op, bits, flags } => {
                    self.a.movz(false, T1, bits, 0);
                    self.a.movz(false, T2, flags as u32, 0);
                    self.emit_call_x87_helper(H_FIST);
                    self.a.mov_rr(true, T4, T0);
                    let a = self.addr_x87(&op);
                    if mem::tso() {
                        self.a.dmb_ishst();
                    }
                    self.a.str(bits, T4, a, 0);
                }
                X87ColdKind::BinopSt { op_pop, dst, src } => {
                    self.a.movz(false, T1, op_pop as u32, 0);
                    self.a.movz(false, T2, dst as u32, 0);
                    self.a.movz(false, T3, src as u32, 0);
                    self.emit_call_x87_helper(H_BINOP_ST);
                }
                X87ColdKind::BinopMem { op_code, w } => {
                    self.a.movz(false, T1, op_code as u32, 0);
                    let is_int = (w & 0x80) != 0;
                    let sz = w & 0x7F;
                    if is_int {
                        self.a.mov_rr(true, T2, T6);
                        self.emit_call_x87_helper(H_BINOP_I64);
                    } else if sz == 4 {
                        self.a.mov_rr(false, T2, T6);
                        self.emit_call_x87_helper(H_BINOP_F32);
                    } else {
                        self.a.mov_rr(true, T2, T6);
                        self.emit_call_x87_helper(H_BINOP_F64);
                    }
                }
                X87ColdKind::Unary { sub } => {
                    self.a.movz(false, T1, sub as u32, 0);
                    self.emit_call_x87_helper(H_UNARY);
                }
                X87ColdKind::CmpSt { idx, flags, is_fcomi } => {
                    self.a.movz(false, T1, idx as u32, 0);
                    self.a.movz(false, T2, flags as u32, 0);
                    self.emit_call_x87_helper(H_COMPARE_ST);
                    if is_fcomi {
                        // Convert comparison code in w0 (`0=GT, 1=LT, 2=EQ, 3=UNORD`) into
                        // exact ARM64 `fcmp` NZCV bits matching `Src::Fcmp`!
                        self.a.movz(true, T1, 0x2000, 1); // GT: C=1
                        self.a.movz(true, T2, 0x8000, 1); // LT: N=1
                        self.a.addsub_ri(false, true, true, ZR, T0, 1, false);
                        self.a.csel(true, T1, T2, T1, EQ);
                        self.a.movz(true, T2, 0x6000, 1); // EQ: Z=1, C=1
                        self.a.addsub_ri(false, true, true, ZR, T0, 2, false);
                        self.a.csel(true, T1, T2, T1, EQ);
                        self.a.movz(true, T2, 0x3000, 1); // UNORD: C=1, V=1
                        self.a.addsub_ri(false, true, true, ZR, T0, 3, false);
                        self.a.csel(true, T1, T2, T1, EQ);
                        self.a.msr_nzcv(T1);
                    }
                }
                X87ColdKind::CmpMem { w, flags } => {
                    let is_int = (w & 0x80) != 0;
                    let sz = w & 0x7F;
                    self.a.movz(false, T2, flags as u32, 0);
                    if is_int {
                        self.a.mov_rr(true, T1, T6);
                        self.emit_call_x87_helper(H_COMPARE_I64);
                    } else if sz == 4 {
                        self.a.mov_rr(false, T1, T6);
                        self.emit_call_x87_helper(H_COMPARE_F32);
                    } else {
                        self.a.mov_rr(true, T1, T6);
                        self.emit_call_x87_helper(H_COMPARE_F64);
                    }
                }
                X87ColdKind::Ftst => {
                    self.a.movz(true, T1, 0, 0); // +0.0f64 bits
                    self.a.movz(false, T2, 1, 0); // signaling=true, pops=0
                    self.emit_call_x87_helper(H_COMPARE_F64);
                }
                X87ColdKind::FcmovFault { idx } => {
                    self.a.movz(false, T1, idx as u32, 0);
                    self.a.mov_rr(false, T2, T6);
                    self.emit_call_x87_helper(H_FCMOV);
                }
            }
            let back = self.a.b();
            self.a.patch(back, c.resume);
        }
        colds.clear();
        self.x87_colds = colds;

        // Emit the shared block-local trampoline once if any helper call was emitted.
        if !self.x87_tramp_calls.is_empty() {
            let tramp_pos = self.a.pos;
            let mut calls = core::mem::take(&mut self.x87_tramp_calls);
            for &site in &calls {
                self.a.patch(site, tramp_pos);
            }
            calls.clear();
            self.x87_tramp_calls = calls;

            // Preserve caller-saved JIT registers (x7..x15, x30), NZCV, pinned v16..v31, and FPCR.
            // Arguments arrive in x0..x3, target function pointer in XA (x16).
            // Returns helper's (x0, x1) untouched.
            self.a.addsub_ri(true, true, false, 31, 31, 96, false);
            self.a.stp_x(7, 8, 31, 0);
            self.a.stp_x(9, 10, 31, 16);
            self.a.stp_x(11, 12, 31, 32);
            self.a.stp_x(13, 14, 31, 48);
            self.a.stp_x(15, 30, 31, 64);
            self.a.mrs_nzcv(XS);
            self.a.str(64, XS, 31, 80);
            if self.xpin {
                self.a.bl_to(self.spill_xmm);
            }
            self.a.call_with_host_fpcr(88);
            if self.xpin {
                self.a.ldr(64, XCPU, 31, 64);
                self.a.bl_to(self.reload_xmm);
            }
            self.a.ldr(64, XS, 31, 80);
            self.a.msr_nzcv(XS);
            self.a.ldp_x(7, 8, 31, 0);
            self.a.ldp_x(9, 10, 31, 16);
            self.a.ldp_x(11, 12, 31, 32);
            self.a.ldp_x(13, 14, 31, 48);
            self.a.ldp_x(15, 30, 31, 64);
            self.a.addsub_ri(true, false, false, 31, 31, 96, false);
            self.a.ret();
        }
    }
}
