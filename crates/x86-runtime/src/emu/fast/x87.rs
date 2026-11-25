//! FastOp lowering and execution for the x87 FPU instruction family (`K_X87 = 127`).
//!
//! Covers all 92 mnemonics in `X87_FPU_STATE_ARITH` (with `fnop`/`fneni`/`fndisi`/`fnsetpm`
//! lowered to `K_NOP`) plus the x87 control/init ops (`fninit`, `finit`, `fldcw`, `fnstcw`,
//! `fstcw`) so x87 blocks stay 100% in `FastOp` / ARM64 JIT / AOT without falling back to the
//! generic interpreter.

use super::decode::put_mem;
use super::*;
use crate::emu::x87::{
    self, with_x87_mut, with_x87_ref, Fp80, X87_CONST_L2E, X87_CONST_L2T, X87_CONST_LG2,
    X87_CONST_LN2, X87_CONST_ONE, X87_CONST_PI, X87_CONST_ZERO, X87_OP_ADD, X87_OP_DIV,
    X87_OP_DIVR, X87_OP_MUL, X87_OP_SUB, X87_OP_SUBR, X87_UNARY_2XM1, X87_UNARY_COS,
    X87_UNARY_PATAN, X87_UNARY_PREM, X87_UNARY_PREM1, X87_UNARY_PTAN, X87_UNARY_RNDINT,
    X87_UNARY_SCALE, X87_UNARY_SIN, X87_UNARY_SINCOS, X87_UNARY_SQRT, X87_UNARY_XTRACT,
    X87_UNARY_YL2X, X87_UNARY_YL2XP1,
};
use crate::emu::{Cpu, OpKind, Register};
use iced_x86::{Instruction, MemorySize, Mnemonic};

/// Pre-decoded x87 FPU instruction (`FastOp::x & 0x3F` holds the `X87_*` sub-op).
pub(in crate::emu) const K_X87: u8 = 127;

// `FastOp::x & 0x3F` sub-op codes (0..=27; high 2 bits of `x` are reserved for SIB scale).
pub(in crate::emu) const X87_FNINIT: u8 = 0;
pub(in crate::emu) const X87_FNCLEX: u8 = 1;
pub(in crate::emu) const X87_FLDCW: u8 = 2;
pub(in crate::emu) const X87_FNSTCW: u8 = 3;
pub(in crate::emu) const X87_FNSTSW_AX: u8 = 4;
pub(in crate::emu) const X87_FNSTSW_M: u8 = 5;
pub(in crate::emu) const X87_FDECSTP: u8 = 6;
pub(in crate::emu) const X87_FINCSTP: u8 = 7;
/// `r0 = st_idx (0..7)`, `w = pop (0: ffree, 1: ffreep)`.
pub(in crate::emu) const X87_FFREE: u8 = 8;
/// `r0 = st_idx (0..7)`.
pub(in crate::emu) const X87_FXCH: u8 = 9;
pub(in crate::emu) const X87_FABS: u8 = 10;
pub(in crate::emu) const X87_FCHS: u8 = 11;
pub(in crate::emu) const X87_FXAM: u8 = 12;
/// `r0 = st_idx (0..7)`.
pub(in crate::emu) const X87_FLD_ST: u8 = 13;
/// `w = byte width (4, 8, or 10)`.
pub(in crate::emu) const X87_FLD_MEM: u8 = 14;
/// `w = byte width (2, 4, or 8)`.
pub(in crate::emu) const X87_FILD: u8 = 15;
/// `r0 = X87_CONST_* (0..=6)`.
pub(in crate::emu) const X87_FLD_CONST: u8 = 16;
/// `r0 = st_idx (0..7)`, `w = pop (0: fst, 1: fstp/fstpnce)`.
pub(in crate::emu) const X87_FST_ST: u8 = 17;
/// `w = byte width (4, 8, or 10)`, `r1 = pop (0 or 1)`.
pub(in crate::emu) const X87_FST_MEM: u8 = 18;
/// `w = byte width (2, 4, or 8)`, `r1 = flags (bit 0: pop, bit 1: trunc)`.
pub(in crate::emu) const X87_FIST: u8 = 19;
/// `r0 = dst (0..7)`, `r1 = src (0..7)`, `w = op_code (0..5) | (pop << 3)`.
pub(in crate::emu) const X87_BINOP_ST: u8 = 20;
/// `r0 = op_code (0..5)`, `w = byte width (4 or 8 for float; 0x80|2 or 0x80|4 for int)`.
pub(in crate::emu) const X87_BINOP_MEM: u8 = 21;
/// `r0 = X87_UNARY_* (0..=13)`.
pub(in crate::emu) const X87_UNARY: u8 = 22;
/// `r0 = st_idx (0..7)`, `w = flags (bit 0: signaling, bit 1: write_sw, bits 3:2: pops, bit 4: is_fcomi)`.
pub(in crate::emu) const X87_CMP_ST: u8 = 23;
/// `w = byte width (4 or 8 for float; 0x80|2 or 0x80|4 for int)`, `r1 = flags (bit 0: signaling, bits 3:2: pops)`.
pub(in crate::emu) const X87_CMP_MEM: u8 = 24;
pub(in crate::emu) const X87_FTST: u8 = 25;
/// `r0 = src_st_idx (0..7)`, `w = cc_idx (0:b, 1:e, 2:be, 3:u, 4:nb, 5:ne, 6:nbe, 7:nu)`.
pub(in crate::emu) const X87_FCMOV: u8 = 26;
pub(in crate::emu) const X87_FBLD: u8 = 27;
pub(in crate::emu) const X87_FBSTP: u8 = 28;
/// `w = flags (bit 0: is_save/frstor-fnsave, bit 1: is_load/fldenv-frstor, bit 2: size16)`.
pub(in crate::emu) const X87_FENV: u8 = 29;

#[inline(always)]
fn st_idx(r: Register) -> Option<u8> {
    let n = (r as u32).wrapping_sub(Register::ST0 as u32);
    if n < 8 {
        Some(n as u8)
    } else {
        None
    }
}

#[inline(always)]
fn set_sub(f: &mut FastOp, sub: u8) {
    f.kind = K_X87;
    f.x = (f.x & 0xC0) | (sub & 0x3F);
}

#[inline]
fn fenv_size16(ins: &Instruction) -> bool {
    matches!(ins.memory_size(), MemorySize::FpuEnv14 | MemorySize::FpuState94)
        || matches!(ins.memory_size().size(), 14 | 94)
}

pub(super) fn lower_x87(ins: &Instruction, mut f: FastOp) -> Option<FastOp> {
    use Mnemonic as M;
    let mn = ins.mnemonic();
    let n = ins.op_count();
    let k0 = if n > 0 { ins.op_kind(0) } else { OpKind::Register };
    let k1 = if n > 1 { ins.op_kind(1) } else { OpKind::Register };

    match mn {
        M::Fnop | M::Fneni | M::Fndisi | M::Fnsetpm => {
            f.kind = K_NOP;
            return Some(f);
        }
        M::Fninit | M::Finit => {
            set_sub(&mut f, X87_FNINIT);
        }
        M::Fnclex | M::Fclex => {
            set_sub(&mut f, X87_FNCLEX);
        }
        M::Fldcw => {
            if n != 1 || k0 != OpKind::Memory {
                return None;
            }
            put_mem(ins, &mut f)?;
            set_sub(&mut f, X87_FLDCW);
        }
        M::Fnstcw | M::Fstcw => {
            if n != 1 || k0 != OpKind::Memory {
                return None;
            }
            put_mem(ins, &mut f)?;
            set_sub(&mut f, X87_FNSTCW);
        }
        M::Fnstsw | M::Fstsw => {
            if n != 1 {
                return None;
            }
            match k0 {
                OpKind::Register if ins.op_register(0) == Register::AX => {
                    set_sub(&mut f, X87_FNSTSW_AX);
                }
                OpKind::Memory => {
                    put_mem(ins, &mut f)?;
                    set_sub(&mut f, X87_FNSTSW_M);
                }
                _ => return None,
            }
        }
        M::Fdecstp => set_sub(&mut f, X87_FDECSTP),
        M::Fincstp => set_sub(&mut f, X87_FINCSTP),
        M::Ffree | M::Ffreep => {
            if n != 1 || k0 != OpKind::Register {
                return None;
            }
            f.r0 = st_idx(ins.op_register(0))?;
            f.w = (mn == M::Ffreep) as u8;
            set_sub(&mut f, X87_FFREE);
        }
        M::Fxch => {
            let idx = if n >= 2 && k1 == OpKind::Register {
                st_idx(ins.op_register(1))?
            } else if n == 1 && k0 == OpKind::Register {
                st_idx(ins.op_register(0))?
            } else if n == 0 {
                1
            } else {
                return None;
            };
            f.r0 = idx;
            set_sub(&mut f, X87_FXCH);
        }
        M::Fabs => set_sub(&mut f, X87_FABS),
        M::Fchs => set_sub(&mut f, X87_FCHS),
        M::Fxam => set_sub(&mut f, X87_FXAM),
        M::Fld => {
            if n != 1 {
                return None;
            }
            match k0 {
                OpKind::Register => {
                    f.r0 = st_idx(ins.op_register(0))?;
                    set_sub(&mut f, X87_FLD_ST);
                }
                OpKind::Memory => {
                    let sz = ins.memory_size().size();
                    if !matches!(sz, 4 | 8 | 10) {
                        return None;
                    }
                    f.w = sz as u8;
                    put_mem(ins, &mut f)?;
                    set_sub(&mut f, X87_FLD_MEM);
                }
                _ => return None,
            }
        }
        M::Fild => {
            if n != 1 || k0 != OpKind::Memory {
                return None;
            }
            let sz = ins.memory_size().size();
            if !matches!(sz, 2 | 4 | 8) {
                return None;
            }
            f.w = sz as u8;
            put_mem(ins, &mut f)?;
            set_sub(&mut f, X87_FILD);
        }
        M::Fldz | M::Fld1 | M::Fldpi | M::Fldl2e | M::Fldl2t | M::Fldlg2 | M::Fldln2 => {
            f.r0 = match mn {
                M::Fldz => X87_CONST_ZERO,
                M::Fld1 => X87_CONST_ONE,
                M::Fldpi => X87_CONST_PI,
                M::Fldl2e => X87_CONST_L2E,
                M::Fldl2t => X87_CONST_L2T,
                M::Fldlg2 => X87_CONST_LG2,
                _ => X87_CONST_LN2,
            };
            set_sub(&mut f, X87_FLD_CONST);
        }
        M::Fst | M::Fstp | M::Fstpnce => {
            if n != 1 {
                return None;
            }
            let pop = matches!(mn, M::Fstp | M::Fstpnce);
            match k0 {
                OpKind::Register => {
                    f.r0 = st_idx(ins.op_register(0))?;
                    f.w = pop as u8;
                    set_sub(&mut f, X87_FST_ST);
                }
                OpKind::Memory => {
                    let sz = ins.memory_size().size();
                    if !matches!(sz, 4 | 8 | 10) {
                        return None;
                    }
                    f.w = sz as u8;
                    f.r1 = pop as u8;
                    put_mem(ins, &mut f)?;
                    set_sub(&mut f, X87_FST_MEM);
                }
                _ => return None,
            }
        }
        M::Fist | M::Fistp | M::Fisttp => {
            if n != 1 || k0 != OpKind::Memory {
                return None;
            }
            let sz = ins.memory_size().size();
            if !matches!(sz, 2 | 4 | 8) {
                return None;
            }
            let pop = matches!(mn, M::Fistp | M::Fisttp) as u8;
            let trunc = (mn == M::Fisttp) as u8;
            f.w = sz as u8;
            f.r1 = pop | (trunc << 1);
            put_mem(ins, &mut f)?;
            set_sub(&mut f, X87_FIST);
        }
        M::Fadd | M::Faddp | M::Fsub | M::Fsubp | M::Fsubr | M::Fsubrp | M::Fmul | M::Fmulp
        | M::Fdiv | M::Fdivp | M::Fdivr | M::Fdivrp => {
            let op_code = match mn {
                M::Fadd | M::Faddp => X87_OP_ADD,
                M::Fsub | M::Fsubp => X87_OP_SUB,
                M::Fsubr | M::Fsubrp => X87_OP_SUBR,
                M::Fmul | M::Fmulp => X87_OP_MUL,
                M::Fdiv | M::Fdivp => X87_OP_DIV,
                _ => X87_OP_DIVR,
            };
            let pop = matches!(
                mn,
                M::Faddp | M::Fsubp | M::Fsubrp | M::Fmulp | M::Fdivp | M::Fdivrp
            );
            if n == 1 && k0 == OpKind::Memory {
                let sz = ins.memory_size().size();
                if !matches!(sz, 4 | 8) {
                    return None;
                }
                f.r0 = op_code;
                f.w = sz as u8;
                put_mem(ins, &mut f)?;
                set_sub(&mut f, X87_BINOP_MEM);
            } else {
                let (dst, src) = match n {
                    0 => (1u8, 0u8),
                    1 if k0 == OpKind::Register => (st_idx(ins.op_register(0))?, 0u8),
                    2 if k0 == OpKind::Register && k1 == OpKind::Register => {
                        (st_idx(ins.op_register(0))?, st_idx(ins.op_register(1))?)
                    }
                    _ => return None,
                };
                f.r0 = dst;
                f.r1 = src;
                f.w = op_code | ((pop as u8) << 3);
                set_sub(&mut f, X87_BINOP_ST);
            }
        }
        M::Fiadd | M::Fisub | M::Fisubr | M::Fimul | M::Fidiv | M::Fidivr => {
            if n != 1 || k0 != OpKind::Memory {
                return None;
            }
            let sz = ins.memory_size().size();
            if !matches!(sz, 2 | 4) {
                return None;
            }
            let op_code = match mn {
                M::Fiadd => X87_OP_ADD,
                M::Fisub => X87_OP_SUB,
                M::Fisubr => X87_OP_SUBR,
                M::Fimul => X87_OP_MUL,
                M::Fidiv => X87_OP_DIV,
                _ => X87_OP_DIVR,
            };
            f.r0 = op_code;
            f.w = 0x80 | (sz as u8);
            put_mem(ins, &mut f)?;
            set_sub(&mut f, X87_BINOP_MEM);
        }
        M::Fsqrt | M::Frndint | M::Fsin | M::Fcos | M::Fsincos | M::Fptan | M::Fpatan
        | M::F2xm1 | M::Fyl2x | M::Fyl2xp1 | M::Fscale | M::Fxtract | M::Fprem | M::Fprem1 => {
            f.r0 = match mn {
                M::Fsqrt => X87_UNARY_SQRT,
                M::Frndint => X87_UNARY_RNDINT,
                M::Fsin => X87_UNARY_SIN,
                M::Fcos => X87_UNARY_COS,
                M::Fsincos => X87_UNARY_SINCOS,
                M::Fptan => X87_UNARY_PTAN,
                M::Fpatan => X87_UNARY_PATAN,
                M::F2xm1 => X87_UNARY_2XM1,
                M::Fyl2x => X87_UNARY_YL2X,
                M::Fyl2xp1 => X87_UNARY_YL2XP1,
                M::Fscale => X87_UNARY_SCALE,
                M::Fxtract => X87_UNARY_XTRACT,
                M::Fprem => X87_UNARY_PREM,
                _ => X87_UNARY_PREM1,
            };
            set_sub(&mut f, X87_UNARY);
        }
        M::Fcom | M::Fcomp | M::Fcompp | M::Fucom | M::Fucomp | M::Fucompp | M::Fcomi
        | M::Fcomip | M::Fucomi | M::Fucomip => {
            let signaling = matches!(mn, M::Fcom | M::Fcomp | M::Fcompp | M::Fcomi | M::Fcomip);
            let is_fcomi = matches!(mn, M::Fcomi | M::Fcomip | M::Fucomi | M::Fucomip);
            let pops: u8 = match mn {
                M::Fcompp | M::Fucompp => 2,
                M::Fcomp | M::Fucomp | M::Fcomip | M::Fucomip => 1,
                _ => 0,
            };
            if n == 1 && k0 == OpKind::Memory {
                if is_fcomi {
                    return None;
                }
                let sz = ins.memory_size().size();
                if !matches!(sz, 4 | 8) {
                    return None;
                }
                f.w = sz as u8;
                f.r1 = (signaling as u8) | (pops << 2);
                put_mem(ins, &mut f)?;
                set_sub(&mut f, X87_CMP_MEM);
            } else {
                let idx = if is_fcomi {
                    match n {
                        2 if k1 == OpKind::Register => st_idx(ins.op_register(1))?,
                        1 if k0 == OpKind::Register => st_idx(ins.op_register(0))?,
                        0 => 1,
                        _ => return None,
                    }
                } else {
                    match n {
                        0 => 1,
                        1 if k0 == OpKind::Register => st_idx(ins.op_register(0))?,
                        2 if k1 == OpKind::Register => st_idx(ins.op_register(1))?,
                        _ => return None,
                    }
                };
                let write_sw = !is_fcomi;
                f.r0 = idx;
                f.w = (signaling as u8)
                    | ((write_sw as u8) << 1)
                    | (pops << 2)
                    | ((is_fcomi as u8) << 4);
                set_sub(&mut f, X87_CMP_ST);
            }
        }
        M::Ficom | M::Ficomp => {
            if n != 1 || k0 != OpKind::Memory {
                return None;
            }
            let sz = ins.memory_size().size();
            if !matches!(sz, 2 | 4) {
                return None;
            }
            let pops = (mn == M::Ficomp) as u8;
            f.w = 0x80 | (sz as u8);
            f.r1 = 1 | (pops << 2);
            put_mem(ins, &mut f)?;
            set_sub(&mut f, X87_CMP_MEM);
        }
        M::Ftst => set_sub(&mut f, X87_FTST),
        M::Fcmovb | M::Fcmove | M::Fcmovbe | M::Fcmovu | M::Fcmovnb | M::Fcmovne
        | M::Fcmovnbe | M::Fcmovnu => {
            let src = match n {
                2 if k1 == OpKind::Register => st_idx(ins.op_register(1))?,
                1 if k0 == OpKind::Register => st_idx(ins.op_register(0))?,
                _ => return None,
            };
            f.r0 = src;
            f.w = match mn {
                M::Fcmovb => 0,
                M::Fcmove => 1,
                M::Fcmovbe => 2,
                M::Fcmovu => 3,
                M::Fcmovnb => 4,
                M::Fcmovne => 5,
                M::Fcmovnbe => 6,
                _ => 7,
            };
            set_sub(&mut f, X87_FCMOV);
        }
        M::Fbld => {
            if n != 1 || k0 != OpKind::Memory {
                return None;
            }
            put_mem(ins, &mut f)?;
            set_sub(&mut f, X87_FBLD);
        }
        M::Fbstp => {
            if n != 1 || k0 != OpKind::Memory {
                return None;
            }
            put_mem(ins, &mut f)?;
            set_sub(&mut f, X87_FBSTP);
        }
        M::Fldenv | M::Fnstenv | M::Fstenv | M::Frstor | M::Fnsave | M::Fsave => {
            if n != 1 || k0 != OpKind::Memory {
                return None;
            }
            let is_save = matches!(mn, M::Frstor | M::Fnsave | M::Fsave) as u8;
            let is_load = matches!(mn, M::Fldenv | M::Frstor) as u8;
            let size16 = fenv_size16(ins) as u8;
            f.w = is_save | (is_load << 1) | (size16 << 2);
            put_mem(ins, &mut f)?;
            set_sub(&mut f, X87_FENV);
        }
        _ => return None,
    }
    Some(f)
}

#[inline(always)]
fn fault_pending() -> bool {
    crate::emu::precise_faults() && crate::emu::FAULT_PENDING.with(|p| p.get().is_some())
}

#[inline]
unsafe fn read_f80_mem(cpu: &mut Cpu, addr: u64) -> Option<[u8; 10]> {
    let mut b = [0u8; 10];
    let lo = cpu.read_mem(addr, 64);
    let hi = cpu.read_mem(addr.wrapping_add(8), 16) as u16;
    if fault_pending() {
        return None;
    }
    b[0..8].copy_from_slice(&lo.to_le_bytes());
    b[8..10].copy_from_slice(&hi.to_le_bytes());
    Some(b)
}

#[inline]
unsafe fn write_f80_mem(cpu: &mut Cpu, addr: u64, b: [u8; 10]) -> bool {
    let lo = u64::from_le_bytes(b[0..8].try_into().unwrap());
    let hi = u16::from_le_bytes([b[8], b[9]]) as u64;
    cpu.write_mem(addr, 64, lo);
    cpu.write_mem(addr.wrapping_add(8), 16, hi);
    !fault_pending()
}

pub(super) unsafe fn exec_x87_fast(cpu: &mut Cpu, op: &FastOp) {
    let sub = op.x & 0x3F;
    match sub {
        X87_FNINIT => x87::x87_reset(),
        X87_FNCLEX => with_x87_mut(|st| st.clear_exceptions()),
        X87_FLDCW => {
            let a = cpu.faddr(op);
            let cw = cpu.read_mem(a, 16) as u16;
            if !fault_pending() {
                with_x87_mut(|st| st.cw = cw);
            }
        }
        X87_FNSTCW => {
            let a = cpu.faddr(op);
            let cw = with_x87_ref(|st| st.cw);
            cpu.write_mem(a, 16, cw as u64);
        }
        X87_FNSTSW_AX => {
            let sw = with_x87_ref(|st| st.status_word());
            cpu.rset(0, 16, sw as u64);
        }
        X87_FNSTSW_M => {
            let a = cpu.faddr(op);
            let sw = with_x87_ref(|st| st.status_word());
            cpu.write_mem(a, 16, sw as u64);
        }
        X87_FDECSTP => with_x87_mut(|st| {
            st.top = st.top.wrapping_sub(1) & 7;
            st.sw &= !0x0200;
            st.sync_top();
        }),
        X87_FINCSTP => with_x87_mut(|st| {
            st.top = st.top.wrapping_add(1) & 7;
            st.sw &= !0x0200;
            st.sync_top();
        }),
        X87_FFREE => with_x87_mut(|st| {
            if op.w != 0 {
                st.freep_st(op.r0);
            } else {
                st.free_st(op.r0);
            }
        }),
        X87_FXCH => with_x87_mut(|st| st.fxch(op.r0)),
        X87_FABS | X87_FCHS => with_x87_mut(|st| {
            let p0 = st.phys(0);
            if (st.valid_mask & (1 << p0)) == 0 {
                st.sw |= 0x0041;
                st.set_slot_bytes(p0, Fp80::QNAN_INDEF_BYTES);
                st.valid_mask |= 1 << p0;
            } else if sub == X87_FABS {
                st.regs[p0][9] &= 0x7f;
            } else {
                st.regs[p0][9] ^= 0x80;
            }
            st.sw &= !0x0200;
            st.sync_top();
        }),
        X87_FXAM => with_x87_mut(|st| st.fxam()),
        X87_FLD_ST => with_x87_mut(|st| st.fld_st(op.r0)),
        X87_FLD_MEM => {
            let a = cpu.faddr(op);
            match op.w {
                4 => {
                    let bits = cpu.read_mem(a, 32) as u32;
                    if !fault_pending() {
                        with_x87_mut(|st| st.push_f32_bits(bits));
                    }
                }
                8 => {
                    let bits = cpu.read_mem(a, 64);
                    if !fault_pending() {
                        with_x87_mut(|st| st.push_f64_bits(bits));
                    }
                }
                _ => {
                    if let Some(b) = read_f80_mem(cpu, a) {
                        with_x87_mut(|st| st.push_bytes(b, 0));
                    }
                }
            }
        }
        X87_FILD => {
            let a = cpu.faddr(op);
            let v: i64 = match op.w {
                2 => cpu.read_mem(a, 16) as u16 as i16 as i64,
                4 => cpu.read_mem(a, 32) as u32 as i32 as i64,
                _ => cpu.read_mem(a, 64) as i64,
            };
            if !fault_pending() {
                with_x87_mut(|st| st.push_fp80(Fp80::from_i64(v), 0));
            }
        }
        X87_FLD_CONST => with_x87_mut(|st| st.push_const(op.r0)),
        X87_FST_ST => with_x87_mut(|st| {
            if op.w != 0 {
                st.pop_to_st(op.r0);
            } else {
                st.fst_st(op.r0);
            }
        }),
        X87_FST_MEM => {
            let a = cpu.faddr(op);
            let pop = op.r1 != 0;
            let saved = x87::x87_snapshot();
            match op.w {
                4 => {
                    let bits = with_x87_mut(|st| st.store_f32_bits(pop));
                    cpu.write_mem(a, 32, bits as u64);
                }
                8 => {
                    let bits = with_x87_mut(|st| st.store_f64_bits(pop));
                    cpu.write_mem(a, 64, bits);
                }
                _ => {
                    let b = with_x87_mut(|st| st.pop_f80_bytes());
                    write_f80_mem(cpu, a, b);
                }
            }
            if fault_pending() {
                x87::x87_restore(saved);
            }
        }
        X87_FIST => {
            let a = cpu.faddr(op);
            let bits = (op.w as u32) * 8;
            let pop = (op.r1 & 1) != 0;
            let trunc = (op.r1 & 2) != 0;
            let saved = x87::x87_snapshot();
            let v = with_x87_mut(|st| st.store_int_bits(bits, pop, trunc));
            cpu.write_mem(a, bits, v);
            if fault_pending() {
                x87::x87_restore(saved);
            }
        }
        X87_BINOP_ST => {
            let op_code = op.w & 7;
            let pop = (op.w & 8) != 0;
            with_x87_mut(|st| st.binop_st(op_code, op.r0, op.r1, pop));
        }
        X87_BINOP_MEM => {
            let a = cpu.faddr(op);
            let op_code = op.r0;
            let is_int = (op.w & 0x80) != 0;
            let sz = op.w & 0x7F;
            let mut extra = 0u16;
            let rhs = if is_int {
                let v = match sz {
                    2 => cpu.read_mem(a, 16) as u16 as i16 as i64,
                    _ => cpu.read_mem(a, 32) as u32 as i32 as i64,
                };
                Fp80::from_i64(v)
            } else if sz == 4 {
                Fp80::from_f32_bits(cpu.read_mem(a, 32) as u32, &mut extra)
            } else {
                Fp80::from_f64_bits(cpu.read_mem(a, 64), &mut extra)
            };
            if !fault_pending() {
                with_x87_mut(|st| st.binop_val(op_code, rhs, extra));
            }
        }
        X87_UNARY => with_x87_mut(|st| st.exec_unary(op.r0)),
        X87_CMP_ST => {
            let signaling = (op.w & 1) != 0;
            let write_sw = (op.w & 2) != 0;
            let pops = (op.w >> 2) & 3;
            let is_fcomi = (op.w & 16) != 0;
            let code = with_x87_mut(|st| st.compare_st(op.r0, signaling, write_sw, pops));
            if is_fcomi {
                match code {
                    0 => {
                        cpu.zf = false;
                        cpu.set_pf(false);
                        cpu.cf = false;
                    }
                    1 => {
                        cpu.zf = false;
                        cpu.set_pf(false);
                        cpu.cf = true;
                    }
                    2 => {
                        cpu.zf = true;
                        cpu.set_pf(false);
                        cpu.cf = false;
                    }
                    _ => {
                        cpu.zf = true;
                        cpu.set_pf(true);
                        cpu.cf = true;
                    }
                }
                cpu.of = false;
                cpu.sf = false;
            }
        }
        X87_CMP_MEM => {
            let a = cpu.faddr(op);
            let is_int = (op.w & 0x80) != 0;
            let sz = op.w & 0x7F;
            let signaling = (op.r1 & 1) != 0;
            let pops = (op.r1 >> 2) & 3;
            let mut extra = 0u16;
            let rhs = if is_int {
                let v = match sz {
                    2 => cpu.read_mem(a, 16) as u16 as i16 as i64,
                    _ => cpu.read_mem(a, 32) as u32 as i32 as i64,
                };
                Fp80::from_i64(v)
            } else if sz == 4 {
                Fp80::from_f32_bits(cpu.read_mem(a, 32) as u32, &mut extra)
            } else {
                Fp80::from_f64_bits(cpu.read_mem(a, 64), &mut extra)
            };
            if !fault_pending() {
                with_x87_mut(|st| st.compare_val(rhs, extra, signaling, pops));
            }
        }
        X87_FTST => with_x87_mut(|st| st.compare_val(Fp80::zero(0), 0, true, 0)),
        X87_FCMOV => {
            let cond = match op.w {
                0 => cpu.cf,
                1 => cpu.zf,
                2 => cpu.cf || cpu.zf,
                3 => cpu.pf(),
                4 => !cpu.cf,
                5 => !cpu.zf,
                6 => !cpu.cf && !cpu.zf,
                _ => !cpu.pf(),
            };
            with_x87_mut(|st| st.fcmov(op.r0, cond));
        }
        X87_FBLD => {
            let a = cpu.faddr(op);
            if let Some(b) = read_f80_mem(cpu, a) {
                with_x87_mut(|st| st.push_bcd(b));
            }
        }
        X87_FBSTP => {
            let a = cpu.faddr(op);
            let saved = x87::x87_snapshot();
            let b = with_x87_mut(|st| st.pop_bcd());
            if !write_f80_mem(cpu, a, b) {
                x87::x87_restore(saved);
            }
        }
        X87_FENV => {
            let a = cpu.faddr(op);
            let is_save = (op.w & 1) != 0;
            let is_load = (op.w & 2) != 0;
            let size16 = (op.w & 4) != 0;
            let len: usize = match (is_save, size16) {
                (false, true) => 14,
                (false, false) => 28,
                (true, true) => 94,
                (true, false) => 108,
            };
            if is_load {
                let mut buf = [0u8; 108];
                for (i, slot) in buf[..len].iter_mut().enumerate() {
                    *slot = cpu.read_mem(a.wrapping_add(i as u64), 8) as u8;
                    if fault_pending() {
                        return;
                    }
                }
                with_x87_mut(|st| {
                    if is_save {
                        st.frstor(&buf[..len], size16);
                    } else {
                        st.decode_env(&buf[..len], size16);
                    }
                });
            } else {
                let saved = x87::x87_snapshot();
                let mut buf = [0u8; 108];
                with_x87_mut(|st| {
                    if is_save {
                        st.fsave(&mut buf[..len], size16);
                    } else {
                        st.stenv(&mut buf[..len], size16);
                    }
                });
                for (i, &byte) in buf[..len].iter().enumerate() {
                    cpu.write_mem(a.wrapping_add(i as u64), 8, byte as u64);
                    if fault_pending() {
                        x87::x87_restore(saved);
                        return;
                    }
                }
            }
        }
        _ => {}
    }
}
