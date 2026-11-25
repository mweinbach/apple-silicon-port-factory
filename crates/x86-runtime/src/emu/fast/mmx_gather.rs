//! Native `FastOp` lowering and execution for:
//! - `SIMD_LEGACY_FALLBACK`: `(v)phaddsw`, `(v)phsubsw`, `(v)phminposuw`, `(v)mpsadbw`, `(v)dppd`, `(v)maskmovdqu`
//! - `AVX2_GATHER`: `vpgatherdd/dq/qd/qq`, `vgatherdps/dpd/qps/qpd` (128 & 256)
//! - `MMX_ALIASED`: all 89 MMX mnemonics (`emms`/`femms`, `movd`/`movq`/`movntq`/`movdq2q`/`movq2dq`,
//!   `maskmovq`, `pinsrw`/`pextrw`/`pmovmskb`, `cvtpi2ps/pd`, `cvt(t)ps/pd2pi`, `pshufw`, `pshufb`,
//!   `palignr`, shifts, packed ALU/compare/pack/unpack/SSSE3) aliased onto architectural [`X87State`].

use super::decode::{gpr_of, lower_v, mem_bits, put_mem, xreg};
use super::*;
use crate::emu::regfile_ext::{with_x87_mut, x87_snapshot};
use crate::emu::{simd_kernels as sk, Cpu, OpKind, Register};
use iced_x86::{EncodingKind, Instruction, Mnemonic as M};

/// Extended legacy SIMD V-form ops (`VX_*`): `(v)phaddsw`, `(v)phsubsw`, `(v)phminposuw`,
/// `(v)mpsadbw`, `(v)dppd`.
pub(in crate::emu) const K_VEXT_RR: u8 = 131;
pub(in crate::emu) const K_VEXT_RM: u8 = 132;
/// Byte-masked non-temporal store to `[rdi/edi]`: `x == 0` -> `(v)maskmovdqu` (16B XMM),
/// `x == 1` -> `maskmovq` (8B MMX + MMX mode transition).
pub(in crate::emu) const K_MASKMOV_B: u8 = 133;
/// AVX2 VSIB gathers (`vpgatherdd/dq/qd/qq`, `vgatherdps/dpd/qps/qpd`, 128 & 256).
pub(in crate::emu) const K_VGATHER: u8 = 134;
/// MMX logic, unpack, pack, shuffle, align and shift ops (64-bit `VS_*` sub-ops).
pub(in crate::emu) const K_MMX_VSHUF_RR: u8 = 135;
pub(in crate::emu) const K_MMX_VSHUF_RM: u8 = 136;
/// MMX packed integer ALU, multiply, min/max, avg, sad, compare and SSSE3 ops (64-bit `VI_*`).
pub(in crate::emu) const K_MMX_VINT_RR: u8 = 137;
pub(in crate::emu) const K_MMX_VINT_RM: u8 = 138;
/// MMX moves, transfers, `pinsrw`/`pextrw`/`pmovmskb`, FP<->MMX conversions, and `emms`/`femms`.
pub(in crate::emu) const K_MMX_MISC: u8 = 139;

// `K_VEXT_*` sub-ops (`FastOp::x & 0x3F`).
pub(in crate::emu) const VX_PHADDSW: u8 = 0;
pub(in crate::emu) const VX_PHSUBSW: u8 = 1;
pub(in crate::emu) const VX_PHMINPOSUW: u8 = 2;
pub(in crate::emu) const VX_MPSADBW: u8 = 3;
pub(in crate::emu) const VX_DPPD: u8 = 4;

// MMX-only `K_MMX_VINT_*` sub-ops (reusing unused AVX2-only `VI_*` numbers).
pub(in crate::emu) const MMX_VI_PHADDSW: u8 = 58;
pub(in crate::emu) const MMX_VI_PHSUBSW: u8 = 59;

// `K_MMX_MISC` sub-ops (`FastOp::x & 0x3F`).
pub(in crate::emu) const MMX_EMMS: u8 = 0;
pub(in crate::emu) const MMX_MOVD_MG: u8 = 1;
pub(in crate::emu) const MMX_MOVD_MM: u8 = 2;
pub(in crate::emu) const MMX_MOVD_GM: u8 = 3;
pub(in crate::emu) const MMX_MOVD_MEM_M: u8 = 4;
pub(in crate::emu) const MMX_MOVQ_MR: u8 = 5;
pub(in crate::emu) const MMX_MOVQ_MG: u8 = 6;
pub(in crate::emu) const MMX_MOVQ_MM: u8 = 7;
pub(in crate::emu) const MMX_MOVQ_GM: u8 = 8;
pub(in crate::emu) const MMX_MOVQ_MEM_M: u8 = 9;
pub(in crate::emu) const MMX_MOVDQ2Q: u8 = 10;
pub(in crate::emu) const MMX_MOVQ2DQ: u8 = 11;
pub(in crate::emu) const MMX_PINSRW_R: u8 = 12;
pub(in crate::emu) const MMX_PINSRW_M: u8 = 13;
pub(in crate::emu) const MMX_PEXTRW: u8 = 14;
pub(in crate::emu) const MMX_PMOVMSKB: u8 = 15;
pub(in crate::emu) const MMX_CVTPI2PS_R: u8 = 16;
pub(in crate::emu) const MMX_CVTPI2PS_M: u8 = 17;
pub(in crate::emu) const MMX_CVTPI2PD_R: u8 = 18;
pub(in crate::emu) const MMX_CVTPI2PD_M: u8 = 19;
pub(in crate::emu) const MMX_CVTPS2PI_R: u8 = 20;
pub(in crate::emu) const MMX_CVTPS2PI_M: u8 = 21;
pub(in crate::emu) const MMX_CVTTPS2PI_R: u8 = 22;
pub(in crate::emu) const MMX_CVTTPS2PI_M: u8 = 23;
pub(in crate::emu) const MMX_CVTPD2PI_R: u8 = 24;
pub(in crate::emu) const MMX_CVTPD2PI_M: u8 = 25;
pub(in crate::emu) const MMX_CVTTPD2PI_R: u8 = 26;
pub(in crate::emu) const MMX_CVTTPD2PI_M: u8 = 27;

#[inline(always)]
fn mmreg(r: Register) -> Option<u8> {
    if r.is_mm() {
        Some(((r as usize) - (Register::MM0 as usize)) as u8)
    } else {
        None
    }
}

#[inline(always)]
fn seg_flag(ins: &Instruction) -> Option<u8> {
    match ins.segment_prefix() {
        Register::None | Register::DS | Register::ES | Register::CS | Register::SS => Some(0),
        Register::GS => Some(B_GS),
        _ => None,
    }
}

/// Lower `SIMD_LEGACY_FALLBACK` (`phaddsw`/`phsubsw`/`phminposuw`/`mpsadbw`/`dppd`/`maskmovdqu`)
/// and `AVX2_GATHER` (`vpgather*`/`vgather*`) into `FastOp`.
pub(super) fn lower_simd_ext_gather(ins: &Instruction, mut f: FastOp) -> Option<FastOp> {
    let mn = ins.mnemonic();
    match mn {
        M::Phaddsw | M::Vphaddsw => {
            lower_v(ins, &mut f, K_VEXT_RR, K_VEXT_RM, VX_PHADDSW, true, None)?;
            Some(f)
        }
        M::Phsubsw | M::Vphsubsw => {
            lower_v(ins, &mut f, K_VEXT_RR, K_VEXT_RM, VX_PHSUBSW, true, None)?;
            Some(f)
        }
        M::Phminposuw | M::Vphminposuw => {
            lower_v(ins, &mut f, K_VEXT_RR, K_VEXT_RM, VX_PHMINPOSUW, false, None)?;
            Some(f)
        }
        M::Mpsadbw | M::Vmpsadbw => {
            lower_v(ins, &mut f, K_VEXT_RR, K_VEXT_RM, VX_MPSADBW, true, None)?;
            Some(f)
        }
        M::Dppd | M::Vdppd => {
            lower_v(ins, &mut f, K_VEXT_RR, K_VEXT_RM, VX_DPPD, false, None)?;
            Some(f)
        }
        M::Maskmovdqu | M::Vmaskmovdqu => {
            if ins.op_count() != 3
                || ins.op_kind(1) != OpKind::Register
                || ins.op_kind(2) != OpKind::Register
            {
                return None;
            }
            let a32 = match ins.op0_kind() {
                OpKind::MemorySegEDI => B_ADDR32,
                OpKind::MemorySegRDI => 0,
                _ => return None,
            };
            let gs = seg_flag(ins)?;
            f.r0 = xreg(ins.op_register(1))?;
            f.r1 = xreg(ins.op_register(2))?;
            f.base = 7 | a32 | gs;
            f.x = 0;
            f.kind = K_MASKMOV_B;
            Some(f)
        }
        M::Vgatherdps
        | M::Vgatherdpd
        | M::Vgatherqps
        | M::Vgatherqpd
        | M::Vpgatherdd
        | M::Vpgatherdq
        | M::Vpgatherqd
        | M::Vpgatherqq => {
            if ins.encoding() != EncodingKind::VEX
                || ins.op_count() != 3
                || ins.op_kind(0) != OpKind::Register
                || ins.op_kind(1) != OpKind::Memory
                || ins.op_kind(2) != OpKind::Register
            {
                return None;
            }
            let (d, dst_ymm) = sk::xmm_reg_info(ins.op_register(0))?;
            let (v, idx_ymm) = sk::xmm_reg_info(ins.memory_index())?;
            let (m, msk_ymm) = sk::xmm_reg_info(ins.op_register(2))?;
            let gs = seg_flag(ins)?;
            let mut a32 = 0u8;
            let base = match ins.memory_base() {
                Register::None => {
                    if ins.memory_displ_size() == 4 {
                        a32 = B_ADDR32;
                    }
                    RNONE
                }
                r => match gpr_of(r) {
                    Some((i, 64)) => i,
                    Some((i, 32)) => {
                        a32 = B_ADDR32;
                        i
                    }
                    _ => return None,
                },
            };
            let scale: u8 = match ins.memory_index_scale() {
                1 => 0,
                2 => 1,
                4 => 2,
                8 => 3,
                _ => return None,
            };
            let elem64 = matches!(mn, M::Vgatherdpd | M::Vpgatherdq | M::Vgatherqpd | M::Vpgatherqq);
            let idx64 = matches!(mn, M::Vgatherqps | M::Vpgatherqd | M::Vgatherqpd | M::Vpgatherqq);
            f.r0 = (d as u8)
                | ((dst_ymm as u8) << 4)
                | ((idx_ymm as u8) << 5)
                | ((msk_ymm as u8) << 6);
            f.r1 = (m as u8) | ((v as u8) << 4);
            f.base = base | gs | a32;
            f.index = RNONE;
            f.disp = ins.memory_displacement64() as i64;
            f.x = (scale << 6) | (elem64 as u8) | ((idx64 as u8) << 1);
            f.kind = K_VGATHER;
            Some(f)
        }
        _ => None,
    }
}

/// Lower any MMX instruction (all 89 `MMX_ALIASED` mnemonics) into `FastOp`.
pub(super) fn lower_mmx_ins(ins: &Instruction, mut f: FastOp) -> Option<FastOp> {
    if ins.encoding() != EncodingKind::Legacy {
        return None;
    }
    let mn = ins.mnemonic();
    let n = ins.op_count();
    let k0 = if n > 0 { ins.op_kind(0) } else { OpKind::Register };
    let k1 = if n > 1 { ins.op_kind(1) } else { OpKind::Register };

    match mn {
        M::Emms | M::Femms => {
            f.kind = K_MMX_MISC;
            f.x = MMX_EMMS;
            return Some(f);
        }
        M::Maskmovq => {
            if n != 3 || k1 != OpKind::Register || ins.op_kind(2) != OpKind::Register {
                return None;
            }
            let a32 = match ins.op0_kind() {
                OpKind::MemorySegEDI => B_ADDR32,
                OpKind::MemorySegRDI => 0,
                _ => return None,
            };
            let gs = seg_flag(ins)?;
            f.r0 = mmreg(ins.op_register(1))?;
            f.r1 = mmreg(ins.op_register(2))?;
            f.base = 7 | a32 | gs;
            f.x = 1;
            f.kind = K_MASKMOV_B;
            return Some(f);
        }
        M::Movd => {
            if n != 2 {
                return None;
            }
            match (k0, k1) {
                (OpKind::Register, OpKind::Register) if ins.op_register(0).is_mm() => {
                    let (s, sw) = gpr_of(ins.op_register(1))?;
                    if sw != 32 {
                        return None;
                    }
                    f.r0 = mmreg(ins.op_register(0))?;
                    f.r1 = s;
                    f.x = MMX_MOVD_MG;
                }
                (OpKind::Register, OpKind::Memory) if ins.op_register(0).is_mm() => {
                    if mem_bits(ins) != 32 {
                        return None;
                    }
                    f.r0 = mmreg(ins.op_register(0))?;
                    f.x = MMX_MOVD_MM;
                    put_mem(ins, &mut f)?;
                }
                (OpKind::Register, OpKind::Register) if ins.op_register(1).is_mm() => {
                    let (d, dw) = gpr_of(ins.op_register(0))?;
                    if dw != 32 {
                        return None;
                    }
                    f.r0 = d;
                    f.r1 = mmreg(ins.op_register(1))?;
                    f.w = 32;
                    f.x = MMX_MOVD_GM;
                }
                (OpKind::Memory, OpKind::Register) if ins.op_register(1).is_mm() => {
                    if mem_bits(ins) != 32 {
                        return None;
                    }
                    f.r1 = mmreg(ins.op_register(1))?;
                    f.x = MMX_MOVD_MEM_M;
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
            f.kind = K_MMX_MISC;
            return Some(f);
        }
        M::Movq | M::Movntq => {
            if n != 2 {
                return None;
            }
            match (k0, k1) {
                (OpKind::Register, OpKind::Register)
                    if mn == M::Movq && ins.op_register(0).is_mm() && ins.op_register(1).is_mm() =>
                {
                    f.r0 = mmreg(ins.op_register(0))?;
                    f.r1 = mmreg(ins.op_register(1))?;
                    f.x = MMX_MOVQ_MR;
                }
                (OpKind::Register, OpKind::Register) if mn == M::Movq && ins.op_register(0).is_mm() => {
                    let (s, sw) = gpr_of(ins.op_register(1))?;
                    if sw != 64 {
                        return None;
                    }
                    f.r0 = mmreg(ins.op_register(0))?;
                    f.r1 = s;
                    f.x = MMX_MOVQ_MG;
                }
                (OpKind::Register, OpKind::Memory) if mn == M::Movq && ins.op_register(0).is_mm() => {
                    if mem_bits(ins) != 64 {
                        return None;
                    }
                    f.r0 = mmreg(ins.op_register(0))?;
                    f.x = MMX_MOVQ_MM;
                    put_mem(ins, &mut f)?;
                }
                (OpKind::Register, OpKind::Register) if mn == M::Movq && ins.op_register(1).is_mm() => {
                    let (d, dw) = gpr_of(ins.op_register(0))?;
                    if dw != 64 {
                        return None;
                    }
                    f.r0 = d;
                    f.r1 = mmreg(ins.op_register(1))?;
                    f.w = 64;
                    f.x = MMX_MOVQ_GM;
                }
                (OpKind::Memory, OpKind::Register) if ins.op_register(1).is_mm() => {
                    if mem_bits(ins) != 64 {
                        return None;
                    }
                    f.r1 = mmreg(ins.op_register(1))?;
                    f.x = MMX_MOVQ_MEM_M;
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
            f.kind = K_MMX_MISC;
            return Some(f);
        }
        M::Movdq2q => {
            if n != 2 || k0 != OpKind::Register || k1 != OpKind::Register {
                return None;
            }
            f.r0 = mmreg(ins.op_register(0))?;
            f.r1 = xreg(ins.op_register(1))?;
            f.x = MMX_MOVDQ2Q;
            f.kind = K_MMX_MISC;
            return Some(f);
        }
        M::Movq2dq => {
            if n != 2 || k0 != OpKind::Register || k1 != OpKind::Register {
                return None;
            }
            f.r0 = xreg(ins.op_register(0))?;
            f.r1 = mmreg(ins.op_register(1))?;
            f.x = MMX_MOVQ2DQ;
            f.kind = K_MMX_MISC;
            return Some(f);
        }
        M::Pinsrw => {
            if n != 3 || k0 != OpKind::Register || ins.op_kind(2) != OpKind::Immediate8 {
                return None;
            }
            f.r0 = mmreg(ins.op_register(0))?;
            f.imm = (ins.immediate8() & 3) as u64;
            match k1 {
                OpKind::Register => {
                    let (s, sw) = gpr_of(ins.op_register(1))?;
                    if sw != 32 && sw != 64 {
                        return None;
                    }
                    f.r1 = s;
                    f.x = MMX_PINSRW_R;
                }
                OpKind::Memory => {
                    if mem_bits(ins) != 16 {
                        return None;
                    }
                    f.x = MMX_PINSRW_M;
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
            f.kind = K_MMX_MISC;
            return Some(f);
        }
        M::Pextrw => {
            if n != 3 || k0 != OpKind::Register || k1 != OpKind::Register || ins.op_kind(2) != OpKind::Immediate8 {
                return None;
            }
            let (d, dw) = gpr_of(ins.op_register(0))?;
            if dw != 32 && dw != 64 {
                return None;
            }
            f.r0 = d;
            f.r1 = mmreg(ins.op_register(1))?;
            f.w = dw;
            f.imm = (ins.immediate8() & 3) as u64;
            f.x = MMX_PEXTRW;
            f.kind = K_MMX_MISC;
            return Some(f);
        }
        M::Pmovmskb => {
            if n != 2 || k0 != OpKind::Register || k1 != OpKind::Register {
                return None;
            }
            let (d, dw) = gpr_of(ins.op_register(0))?;
            if dw != 32 && dw != 64 {
                return None;
            }
            f.r0 = d;
            f.r1 = mmreg(ins.op_register(1))?;
            f.w = dw;
            f.x = MMX_PMOVMSKB;
            f.kind = K_MMX_MISC;
            return Some(f);
        }
        M::Cvtpi2ps | M::Cvtpi2pd => {
            if n != 2 || k0 != OpKind::Register {
                return None;
            }
            let is_pd = mn == M::Cvtpi2pd;
            f.r0 = xreg(ins.op_register(0))?;
            match k1 {
                OpKind::Register => {
                    f.r1 = mmreg(ins.op_register(1))?;
                    f.x = if is_pd { MMX_CVTPI2PD_R } else { MMX_CVTPI2PS_R };
                }
                OpKind::Memory => {
                    if mem_bits(ins) != 64 {
                        return None;
                    }
                    f.x = if is_pd { MMX_CVTPI2PD_M } else { MMX_CVTPI2PS_M };
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
            f.kind = K_MMX_MISC;
            return Some(f);
        }
        M::Cvtps2pi | M::Cvttps2pi | M::Cvtpd2pi | M::Cvttpd2pi => {
            if n != 2 || k0 != OpKind::Register {
                return None;
            }
            let is_pd = matches!(mn, M::Cvtpd2pi | M::Cvttpd2pi);
            let trunc = matches!(mn, M::Cvttps2pi | M::Cvttpd2pi);
            f.r0 = mmreg(ins.op_register(0))?;
            match k1 {
                OpKind::Register => {
                    f.r1 = xreg(ins.op_register(1))?;
                    f.x = match (is_pd, trunc) {
                        (false, false) => MMX_CVTPS2PI_R,
                        (false, true) => MMX_CVTTPS2PI_R,
                        (true, false) => MMX_CVTPD2PI_R,
                        (true, true) => MMX_CVTTPD2PI_R,
                    };
                }
                OpKind::Memory => {
                    let want = if is_pd { 128 } else { 64 };
                    if mem_bits(ins) != want {
                        return None;
                    }
                    f.x = match (is_pd, trunc) {
                        (false, false) => MMX_CVTPS2PI_M,
                        (false, true) => MMX_CVTTPS2PI_M,
                        (true, false) => MMX_CVTPD2PI_M,
                        (true, true) => MMX_CVTTPD2PI_M,
                    };
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
            f.kind = K_MMX_MISC;
            return Some(f);
        }
        _ => {}
    }

    // Standard 2/3-operand MMX packed ops (`mm, mm/m64[, imm8]` or `mm, imm8`).
    if !(2..=3).contains(&n) || k0 != OpKind::Register {
        return None;
    }
    let d = mmreg(ins.op_register(0))?;
    let imm_form = ins.op_kind(n - 1) == OpKind::Immediate8;

    // 2-operand immediate shifts (`psllw mm, imm8` etc.)
    if n == 2 && k1 == OpKind::Immediate8 {
        let sub = match mn {
            M::Psrlw => VS_PSRLW_I,
            M::Psraw => VS_PSRAW_I,
            M::Psllw => VS_PSLLW_I,
            M::Psrld => VS_PSRLD_I,
            M::Psrad => VS_PSRAD_I,
            M::Pslld => VS_PSLLD_I,
            M::Psrlq => VS_PSRLQ_I,
            M::Psllq => VS_PSLLQ_I,
            _ => return None,
        };
        f.kind = K_MMX_VSHUF_RR;
        f.r0 = d;
        f.r1 = d;
        f.x = sub;
        f.imm = ins.immediate8() as u64;
        return Some(f);
    }

    let (is_vint, sub, want_imm) = match mn {
        M::Pand => (false, VS_AND, false),
        M::Pandn => (false, VS_ANDN, false),
        M::Por => (false, VS_OR, false),
        M::Pxor => (false, VS_XOR, false),
        M::Punpcklbw => (false, VS_UNPCKLBW, false),
        M::Punpcklwd => (false, VS_UNPCKLWD, false),
        M::Punpckldq => (false, VS_UNPCKLDQ, false),
        M::Punpckhbw => (false, VS_UNPCKHBW, false),
        M::Punpckhwd => (false, VS_UNPCKHWD, false),
        M::Punpckhdq => (false, VS_UNPCKHDQ, false),
        M::Packsswb => (false, VS_PACKSSWB, false),
        M::Packuswb => (false, VS_PACKUSWB, false),
        M::Packssdw => (false, VS_PACKSSDW, false),
        M::Pshufw => (false, VS_PSHUFLW, true),
        M::Pshufb => (false, VS_PSHUFB, false),
        M::Palignr => (false, VS_PALIGNR, true),
        M::Psrlw => (false, VS_PSRLW, false),
        M::Psraw => (false, VS_PSRAW, false),
        M::Psllw => (false, VS_PSLLW, false),
        M::Psrld => (false, VS_PSRLD, false),
        M::Psrad => (false, VS_PSRAD, false),
        M::Pslld => (false, VS_PSLLD, false),
        M::Psrlq => (false, VS_PSRLQ, false),
        M::Psllq => (false, VS_PSLLQ, false),
        M::Paddb => (true, VI_PADDB, false),
        M::Paddw => (true, VI_PADDW, false),
        M::Paddd => (true, VI_PADDD, false),
        M::Paddq => (true, VI_PADDQ, false),
        M::Psubb => (true, VI_PSUBB, false),
        M::Psubw => (true, VI_PSUBW, false),
        M::Psubd => (true, VI_PSUBD, false),
        M::Psubq => (true, VI_PSUBQ, false),
        M::Paddsb => (true, VI_PADDSB, false),
        M::Paddsw => (true, VI_PADDSW, false),
        M::Paddusb => (true, VI_PADDUSB, false),
        M::Paddusw => (true, VI_PADDUSW, false),
        M::Psubsb => (true, VI_PSUBSB, false),
        M::Psubsw => (true, VI_PSUBSW, false),
        M::Psubusb => (true, VI_PSUBUSB, false),
        M::Psubusw => (true, VI_PSUBUSW, false),
        M::Pmullw => (true, VI_PMULLW, false),
        M::Pmulhw => (true, VI_PMULHW, false),
        M::Pmulhuw => (true, VI_PMULHUW, false),
        M::Pmaddwd => (true, VI_PMADDWD, false),
        M::Pmaddubsw => (true, VI_PMADDUBSW, false),
        M::Pmuludq => (true, VI_PMULUDQ, false),
        M::Pmulhrsw => (true, VI_PMULHRSW, false),
        M::Pminub => (true, VI_PMINUB, false),
        M::Pmaxub => (true, VI_PMAXUB, false),
        M::Pminsw => (true, VI_PMINSW, false),
        M::Pmaxsw => (true, VI_PMAXSW, false),
        M::Pavgb => (true, VI_PAVGB, false),
        M::Pavgw => (true, VI_PAVGW, false),
        M::Psadbw => (true, VI_PSADBW, false),
        M::Pcmpeqb => (true, VI_PCMPEQB, false),
        M::Pcmpeqw => (true, VI_PCMPEQW, false),
        M::Pcmpeqd => (true, VI_PCMPEQD, false),
        M::Pcmpgtb => (true, VI_PCMPGTB, false),
        M::Pcmpgtw => (true, VI_PCMPGTW, false),
        M::Pcmpgtd => (true, VI_PCMPGTD, false),
        M::Pabsb => (true, VI_PABSB, false),
        M::Pabsw => (true, VI_PABSW, false),
        M::Pabsd => (true, VI_PABSD, false),
        M::Psignb => (true, VI_PSIGNB, false),
        M::Psignw => (true, VI_PSIGNW, false),
        M::Psignd => (true, VI_PSIGND, false),
        M::Phaddw => (true, VI_PHADDW, false),
        M::Phaddd => (true, VI_PHADDD, false),
        M::Phsubw => (true, VI_PHSUBW, false),
        M::Phsubd => (true, VI_PHSUBD, false),
        M::Phaddsw => (true, MMX_VI_PHADDSW, false),
        M::Phsubsw => (true, MMX_VI_PHSUBSW, false),
        _ => return None,
    };

    if want_imm {
        if n != 3 || !imm_form {
            return None;
        }
        f.imm = ins.immediate8() as u64;
    } else if n != 2 {
        return None;
    }

    f.r0 = d;
    f.x = sub;
    match k1 {
        OpKind::Register => {
            f.r1 = mmreg(ins.op_register(1))?;
            f.kind = if is_vint { K_MMX_VINT_RR } else { K_MMX_VSHUF_RR };
        }
        OpKind::Memory => {
            let expect_bits = if matches!(mn, M::Punpcklbw | M::Punpcklwd | M::Punpckldq) {
                32
            } else {
                64
            };
            if mem_bits(ins) != expect_bits {
                return None;
            }
            f.w = expect_bits as u8;
            put_mem(ins, &mut f)?;
            f.kind = if is_vint { K_MMX_VINT_RM } else { K_MMX_VSHUF_RM };
        }
        _ => return None,
    }
    Some(f)
}

#[inline(always)]
fn eval_vext_lane(sub: u8, a: u128, b: u128, imm: u8) -> u128 {
    match sub {
        VX_PHADDSW => sk::phadd_sub_sw(a, b, false),
        VX_PHSUBSW => sk::phadd_sub_sw(a, b, true),
        VX_PHMINPOSUW => {
            let (i, min) = (0..8u128).map(|i| (i, (b >> (16 * i)) as u16)).min_by_key(|&(_, v)| v).unwrap();
            u128::from(min) | (i << 16)
        }
        VX_MPSADBW => {
            let (ab, bb) = (a.to_le_bytes(), b.to_le_bytes());
            let i = usize::from((imm >> 2) & 1) * 4;
            let j = usize::from(imm & 3) * 4;
            (0..8).fold(0u128, |result, k| {
                let sum: u16 = (0..4).map(|n| u16::from(ab[i + k + n].abs_diff(bb[j + n]))).sum();
                result | (u128::from(sum) << (16 * k))
            })
        }
        _ => {
            // VX_DPPD
            let product = |shift: u32, bit: u8| {
                if imm & bit == 0 {
                    return 0.0;
                }
                crate::emu::sse_arith64(2, f64::from_bits((a >> shift) as u64), f64::from_bits((b >> shift) as u64))
            };
            let sum = u128::from(crate::emu::sse_arith64(0, product(0, 16), product(64, 32)).to_bits());
            let lo = if imm & 1 != 0 { sum } else { 0 };
            let hi = if imm & 2 != 0 { sum << 64 } else { 0 };
            lo | hi
        }
    }
}

#[inline(always)]
fn narrowed_mmx(v: u128) -> u64 {
    (v as u32 as u64) | (((v >> 64) as u32 as u64) << 32)
}

#[inline(always)]
fn mmx_commit_if_ok(dest: Option<(usize, u64)>) {
    if !crate::emu::precise_faults() || crate::emu::FAULT_PENDING.with(|p| p.get().is_none()) {
        with_x87_mut(|st| {
            if let Some((slot, val)) = dest {
                let reg = &mut st.regs[slot & 7];
                reg[..8].copy_from_slice(&val.to_le_bytes());
                reg[8..].fill(0xff);
            }
            st.valid_mask = 0xff;
            st.top = 0;
        });
    }
}

#[inline(always)]
fn mmx_read(slot: u8) -> u64 {
    let st = x87_snapshot();
    u64::from_le_bytes(st.regs[slot as usize & 7][..8].try_into().unwrap())
}

unsafe fn exec_vgather(cpu: &mut Cpu, op: &FastOp) {
    let d = (op.r0 & 15) as usize;
    let dst_ymm = (op.r0 & 0x10) != 0;
    let idx_ymm = (op.r0 & 0x20) != 0;
    let msk_ymm = (op.r0 & 0x40) != 0;
    let m = (op.r1 & 15) as usize;
    let v = ((op.r1 >> 4) & 15) as usize;
    if d == m || d == v || m == v {
        crate::emu::xstate::Fault::ud().dispatch(cpu, op.ip);
        return;
    }
    let elem_bits: u32 = if (op.x & 1) != 0 { 64 } else { 32 };
    let idx_bits: u32 = if (op.x & 2) != 0 { 64 } else { 32 };
    let count: usize = match (elem_bits, idx_bits) {
        (32, 32) => if dst_ymm { 8 } else { 4 },
        (64, 32) => if dst_ymm { 4 } else { 2 },
        (32, 64) => if idx_ymm { 4 } else { 2 },
        _ => if dst_ymm { 4 } else { 2 },
    };
    let elem_bytes = (elem_bits / 8) as usize;
    let per_lane = 16 / elem_bytes;
    let idx_per_lane = (128 / idx_bits) as usize;
    let b_reg = op.base & 0x1F;
    let is_a32 = (op.base & B_ADDR32) != 0;
    let is_gs = (op.base & B_GS) != 0;
    let base_val = if b_reg < 16 {
        let raw = cpu.gpr[b_reg as usize];
        if is_a32 { raw as u32 as u64 } else { raw }
    } else {
        0
    };
    let disp = op.disp as u64;
    let scale_sh = (op.x >> 6) as u32;
    let gs_base = if is_gs {
        if cpu.jx.teb != 0 {
            cpu.jx.teb
        } else {
            crate::exe::teb_self() as u64
        }
    } else {
        0
    };
    let (v_lo, v_hi) = (cpu.xmm[v], if idx_ymm { cpu.ymm_hi[v] } else { 0 });
    let mut d_vec = [cpu.xmm[d], if dst_ymm { cpu.ymm_hi[d] } else { 0 }];
    let mut m_vec = [cpu.xmm[m], if msk_ymm { cpu.ymm_hi[m] } else { 0 }];
    let elem_mask: u128 = if elem_bits == 64 { u64::MAX as u128 } else { u32::MAX as u128 };
    let sign_bit = elem_bits - 1;
    let mut any_load = false;

    for i in 0..count {
        let half = i / per_lane;
        let sh = ((i % per_lane) as u32) * elem_bits;
        let m_lane = (m_vec[half] >> sh) & elem_mask;
        if ((m_lane >> sign_bit) & 1) != 0 {
            let idx_half = i / idx_per_lane;
            let idx_sh = ((i % idx_per_lane) as u32) * idx_bits;
            let raw_idx = (if idx_half == 0 { v_lo } else { v_hi }) >> idx_sh;
            let signed_idx: u64 = if idx_bits == 32 {
                (raw_idx as u32 as i32 as i64) as u64
            } else {
                raw_idx as u64
            };
            let mut ea = base_val.wrapping_add(disp).wrapping_add(signed_idx << scale_sh);
            if is_a32 {
                ea = (ea as u32) as u64;
            }
            if gs_base != 0 {
                ea = gs_base.wrapping_add(ea);
            }
            let mut buf = [0u8; 8];
            if !crate::emu::regfile_ext::is_valid_guest_range(ea, elem_bytes)
                || !win32_rt::crash::safe_read(ea, &mut buf[..elem_bytes])
            {
                cpu.xmm[d] = d_vec[0];
                if dst_ymm {
                    cpu.set_ymm_hi(d, d_vec[1]);
                }
                cpu.xmm[m] = m_vec[0];
                if msk_ymm {
                    cpu.set_ymm_hi(m, m_vec[1]);
                }
                crate::emu::xstate::Fault::pf(ea, false).dispatch(cpu, op.ip);
                return;
            }
            any_load = true;
            let val = if elem_bits == 64 {
                u64::from_le_bytes(buf) as u128
            } else {
                u32::from_le_bytes(buf[..4].try_into().unwrap()) as u128
            };
            d_vec[half] = (d_vec[half] & !(elem_mask << sh)) | (val << sh);
        }
        m_vec[half] &= !(elem_mask << sh);
    }
    if any_load && crate::emu::mem::tso() {
        core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
    }
    if elem_bits == 32 && idx_bits == 64 && !idx_ymm {
        d_vec[0] &= u64::MAX as u128;
    }
    cpu.xmm[d] = d_vec[0];
    if dst_ymm {
        cpu.set_ymm_hi(d, d_vec[1]);
    } else {
        cpu.clear_ymm_hi_reg(d);
    }
    cpu.xmm[m] = 0;
    cpu.clear_ymm_hi_reg(m);
}

#[inline(never)]
pub(super) unsafe fn exec_mmx_gather(cpu: &mut Cpu, op: &FastOp) {
    match op.kind {
        K_VEXT_RR | K_VEXT_RM => {
            let sub = op.x & 0x3F;
            let imm8 = op.imm as u8;
            let s1 = ((op.imm >> 8) as usize) & 15;
            let mode = op.w & 3;
            let (b_lo, b_hi) = if op.kind == K_VEXT_RM {
                cpu.vload(op)
            } else {
                let r = op.r1 as usize & 15;
                (cpu.xmm[r], if mode == VM_V256 { cpu.ymm_hi[r] } else { 0 })
            };
            let lo = eval_vext_lane(sub, cpu.xmm[s1], b_lo, imm8);
            let d = op.r0 as usize & 15;
            match mode {
                VM_LEG => cpu.xmm[d] = lo,
                VM_V128 => {
                    cpu.xmm[d] = lo;
                    cpu.clear_ymm_hi_reg(d);
                }
                _ => {
                    let hi_imm = if sub == VX_MPSADBW { imm8 >> 3 } else { imm8 };
                    let hi = eval_vext_lane(sub, cpu.ymm_hi[s1], b_hi, hi_imm);
                    cpu.xmm[d] = lo;
                    cpu.set_ymm_hi(d, hi);
                }
            }
        }
        K_MASKMOV_B => {
            let is_mmx = (op.x & 1) != 0;
            let is_a32 = (op.base & B_ADDR32) != 0;
            let is_gs = (op.base & B_GS) != 0;
            let mut base = if is_a32 { u64::from(cpu.gpr[7] as u32) } else { cpu.gpr[7] };
            if is_gs {
                let t = if cpu.jx.teb != 0 {
                    cpu.jx.teb
                } else {
                    crate::exe::teb_self() as u64
                };
                base = base.wrapping_add(t);
            }
            if is_mmx {
                let data = u128::from(mmx_read(op.r0));
                let mask = u128::from(mmx_read(op.r1));
                for i in 0..8usize {
                    if mask & (0x80 << (8 * i)) != 0 {
                        cpu.write_mem(base.wrapping_add(i as u64), 8, (data >> (8 * i)) as u64);
                    }
                }
                mmx_commit_if_ok(None);
            } else {
                let data = cpu.xmm[op.r0 as usize & 15];
                let mask = cpu.xmm[op.r1 as usize & 15];
                for i in 0..16usize {
                    if mask & (0x80 << (8 * i)) != 0 {
                        cpu.write_mem(base.wrapping_add(i as u64), 8, (data >> (8 * i)) as u64);
                    }
                }
            }
        }
        K_VGATHER => exec_vgather(cpu, op),
        K_MMX_VSHUF_RR | K_MMX_VSHUF_RM => {
            let sub = op.x & 0x3F;
            let imm8 = op.imm as u8;
            let a = u128::from(mmx_read(op.r0));
            let b = if op.kind == K_MMX_VSHUF_RM {
                let bits = if op.w == 32 { 32 } else { 64 };
                u128::from(cpu.read_mem(cpu.faddr(op), bits))
            } else {
                u128::from(mmx_read(op.r1))
            };
            let res: u64 = match sub {
                VS_AND => (a & b) as u64,
                VS_ANDN => (!a & b) as u64,
                VS_OR => (a | b) as u64,
                VS_XOR => (a ^ b) as u64,
                VS_UNPCKLBW => sk::punpcklbw(a, b) as u64,
                VS_UNPCKLWD => sk::punpcklwd(a, b) as u64,
                VS_UNPCKLDQ => sk::punpckldq(a, b) as u64,
                VS_UNPCKHBW => sk::punpcklbw(a >> 32, b >> 32) as u64,
                VS_UNPCKHWD => sk::punpcklwd(a >> 32, b >> 32) as u64,
                VS_UNPCKHDQ => sk::punpckldq(a >> 32, b >> 32) as u64,
                VS_PACKSSWB => narrowed_mmx(sk::packsswb(a, b)),
                VS_PACKUSWB => narrowed_mmx(sk::packuswb(a, b)),
                VS_PACKSSDW => narrowed_mmx(sk::packssdw(a, b)),
                VS_PSHUFLW => (0..4).fold(0u64, |v, i| {
                    v | (((b >> (16 * ((imm8 >> (2 * i)) & 3))) as u16 as u64) << (16 * i))
                }),
                VS_PSHUFB => sk::pshufb(a | (a << 64), b) as u64,
                VS_PALIGNR => ((a << 64) | b).checked_shr(u32::from(imm8) * 8).unwrap_or(0) as u64,
                VS_PSRLW_I => sk::pshift_w(b, u64::from(imm8), 1) as u64,
                VS_PSRAW_I => sk::pshift_w(b, u64::from(imm8), 2) as u64,
                VS_PSLLW_I => sk::pshift_w(b, u64::from(imm8), 0) as u64,
                VS_PSRLD_I => sk::pshift_d(b, u64::from(imm8), 1) as u64,
                VS_PSRAD_I => sk::pshift_d(b, u64::from(imm8), 2) as u64,
                VS_PSLLD_I => sk::pshift_d(b, u64::from(imm8), 0) as u64,
                VS_PSRLQ_I => sk::pshift_q(b, u64::from(imm8), true) as u64,
                VS_PSLLQ_I => sk::pshift_q(b, u64::from(imm8), false) as u64,
                VS_PSRLW => sk::pshift_w(a, b as u64, 1) as u64,
                VS_PSRAW => sk::pshift_w(a, b as u64, 2) as u64,
                VS_PSLLW => sk::pshift_w(a, b as u64, 0) as u64,
                VS_PSRLD => sk::pshift_d(a, b as u64, 1) as u64,
                VS_PSRAD => sk::pshift_d(a, b as u64, 2) as u64,
                VS_PSLLD => sk::pshift_d(a, b as u64, 0) as u64,
                VS_PSRLQ => sk::pshift_q(a, b as u64, true) as u64,
                _ => sk::pshift_q(a, b as u64, false) as u64, // VS_PSLLQ
            };
            mmx_commit_if_ok(Some((op.r0 as usize & 7, res)));
        }
        K_MMX_VINT_RR | K_MMX_VINT_RM => {
            let sub = op.x & 0x3F;
            let a = u128::from(mmx_read(op.r0));
            let b = if op.kind == K_MMX_VINT_RM {
                u128::from(cpu.read_mem(cpu.faddr(op), 64))
            } else {
                u128::from(mmx_read(op.r1))
            };
            let res: u64 = match sub {
                VI_PADDB => sk::paddb(a, b) as u64,
                VI_PADDW => sk::paddw(a, b) as u64,
                VI_PADDD => sk::paddd(a, b) as u64,
                VI_PADDQ => sk::paddq(a, b) as u64,
                VI_PSUBB => sk::psubb(a, b) as u64,
                VI_PSUBW => sk::psubw(a, b) as u64,
                VI_PSUBD => sk::psubd(a, b) as u64,
                VI_PSUBQ => sk::psubq(a, b) as u64,
                VI_PADDSB => sk::paddsb(a, b) as u64,
                VI_PADDSW => sk::paddsw(a, b) as u64,
                VI_PADDUSB => sk::paddusb(a, b) as u64,
                VI_PADDUSW => sk::paddusw(a, b) as u64,
                VI_PSUBSB => sk::psubsb(a, b) as u64,
                VI_PSUBSW => sk::psubsw(a, b) as u64,
                VI_PSUBUSB => sk::psubusb(a, b) as u64,
                VI_PSUBUSW => sk::psubusw(a, b) as u64,
                VI_PMULLW => sk::pmullw(a, b) as u64,
                VI_PMULHW => sk::pmulhw(a, b) as u64,
                VI_PMULHUW => sk::pmulhuw(a, b) as u64,
                VI_PMADDWD => sk::pmaddwd(a, b) as u64,
                VI_PMADDUBSW => sk::pmaddubsw(a, b) as u64,
                VI_PMULUDQ => sk::pmuludq(a, b) as u64,
                VI_PMULHRSW => sk::pmulhrsw(a, b) as u64,
                VI_PMINUB => sk::pminub(a, b) as u64,
                VI_PMAXUB => sk::pmaxub(a, b) as u64,
                VI_PMINSW => sk::pminsw(a, b) as u64,
                VI_PMAXSW => sk::pmaxsw(a, b) as u64,
                VI_PAVGB => sk::pavgb(a, b) as u64,
                VI_PAVGW => sk::pavgw(a, b) as u64,
                VI_PSADBW => sk::psadbw(a, b) as u64,
                VI_PCMPEQB => sk::pcmpeqb(a, b) as u64,
                VI_PCMPEQW => sk::pcmpeqw(a, b) as u64,
                VI_PCMPEQD => sk::pcmpeqd(a, b) as u64,
                VI_PCMPGTB => sk::pcmpgtb(a, b) as u64,
                VI_PCMPGTW => sk::pcmpgtw(a, b) as u64,
                VI_PCMPGTD => sk::pcmpgtd(a, b) as u64,
                VI_PABSB => sk::pabsb(b) as u64,
                VI_PABSW => sk::pabsw(b) as u64,
                VI_PABSD => sk::pabsd(b) as u64,
                VI_PSIGNB => sk::psignb(a, b) as u64,
                VI_PSIGNW => sk::psignw(a, b) as u64,
                VI_PSIGND => sk::psignd(a, b) as u64,
                VI_PHADDW => narrowed_mmx(sk::phadd_sub_w(a, b, false)),
                VI_PHADDD => narrowed_mmx(sk::phadd_sub_d(a, b, false)),
                VI_PHSUBW => narrowed_mmx(sk::phadd_sub_w(a, b, true)),
                VI_PHSUBD => narrowed_mmx(sk::phadd_sub_d(a, b, true)),
                MMX_VI_PHADDSW => narrowed_mmx(sk::phadd_sub_sw(a, b, false)),
                _ => narrowed_mmx(sk::phadd_sub_sw(a, b, true)), // MMX_VI_PHSUBSW
            };
            mmx_commit_if_ok(Some((op.r0 as usize & 7, res)));
        }
        K_MMX_MISC => match op.x & 0x3F {
            MMX_EMMS => with_x87_mut(|st| st.valid_mask = 0),
            MMX_MOVD_MG => {
                let v = cpu.rget(op.r1, 32) as u32 as u64;
                mmx_commit_if_ok(Some((op.r0 as usize & 7, v)));
            }
            MMX_MOVD_MM => {
                let v = cpu.read_mem(cpu.faddr(op), 32) as u32 as u64;
                mmx_commit_if_ok(Some((op.r0 as usize & 7, v)));
            }
            MMX_MOVD_GM => {
                let v = mmx_read(op.r1) as u32 as u64;
                cpu.rset(op.r0, 32, v);
                mmx_commit_if_ok(None);
            }
            MMX_MOVD_MEM_M => {
                let v = mmx_read(op.r1) as u32 as u64;
                cpu.write_mem(cpu.faddr(op), 32, v);
                mmx_commit_if_ok(None);
            }
            MMX_MOVQ_MR => {
                let v = mmx_read(op.r1);
                mmx_commit_if_ok(Some((op.r0 as usize & 7, v)));
            }
            MMX_MOVQ_MG => {
                let v = cpu.rget(op.r1, 64);
                mmx_commit_if_ok(Some((op.r0 as usize & 7, v)));
            }
            MMX_MOVQ_MM => {
                let v = cpu.read_mem(cpu.faddr(op), 64);
                mmx_commit_if_ok(Some((op.r0 as usize & 7, v)));
            }
            MMX_MOVQ_GM => {
                let v = mmx_read(op.r1);
                cpu.rset(op.r0, 64, v);
                mmx_commit_if_ok(None);
            }
            MMX_MOVQ_MEM_M => {
                let v = mmx_read(op.r1);
                cpu.write_mem(cpu.faddr(op), 64, v);
                mmx_commit_if_ok(None);
            }
            MMX_MOVDQ2Q => {
                let v = cpu.xmm[op.r1 as usize & 15] as u64;
                mmx_commit_if_ok(Some((op.r0 as usize & 7, v)));
            }
            MMX_MOVQ2DQ => {
                cpu.xmm[op.r0 as usize & 15] = u128::from(mmx_read(op.r1));
                mmx_commit_if_ok(None);
            }
            MMX_PINSRW_R | MMX_PINSRW_M => {
                let word = if (op.x & 0x3F) == MMX_PINSRW_M {
                    cpu.read_mem(cpu.faddr(op), 16) as u16
                } else {
                    cpu.rget(op.r1, 16) as u16
                };
                let shift = ((op.imm as u32) & 3) * 16;
                let cur = mmx_read(op.r0);
                let val = (cur & !(0xffffu64 << shift)) | ((word as u64) << shift);
                mmx_commit_if_ok(Some((op.r0 as usize & 7, val)));
            }
            MMX_PEXTRW => {
                let v = ((mmx_read(op.r1) >> (((op.imm as u32) & 3) * 16)) as u16) as u64;
                cpu.rset(op.r0, op.w, v);
                mmx_commit_if_ok(None);
            }
            MMX_PMOVMSKB => {
                let v = sk::pmovmskb(u128::from(mmx_read(op.r1))) & 0xff;
                cpu.rset(op.r0, op.w, v);
                mmx_commit_if_ok(None);
            }
            MMX_CVTPI2PS_R | MMX_CVTPI2PS_M => {
                let is_mem = (op.x & 0x3F) == MMX_CVTPI2PS_M;
                let b = if is_mem { cpu.read_mem(cpu.faddr(op), 64) } else { mmx_read(op.r1) };
                let (x, y) = (b as i32, (b >> 32) as i32);
                let d = op.r0 as usize & 15;
                cpu.xmm[d] = (cpu.xmm[d] & !u128::from(u64::MAX))
                    | u128::from((x as f32).to_bits())
                    | (u128::from((y as f32).to_bits()) << 32);
                if !is_mem {
                    mmx_commit_if_ok(None);
                }
            }
            MMX_CVTPI2PD_R | MMX_CVTPI2PD_M => {
                let is_mem = (op.x & 0x3F) == MMX_CVTPI2PD_M;
                let b = if is_mem { cpu.read_mem(cpu.faddr(op), 64) } else { mmx_read(op.r1) };
                let (x, y) = (b as i32, (b >> 32) as i32);
                let d = op.r0 as usize & 15;
                cpu.xmm[d] = u128::from((x as f64).to_bits()) | (u128::from((y as f64).to_bits()) << 64);
                if !is_mem {
                    mmx_commit_if_ok(None);
                }
            }
            sub @ MMX_CVTPS2PI_R..=MMX_CVTTPD2PI_M => {
                let is_pd = sub >= MMX_CVTPD2PI_R;
                let trunc = matches!(
                    sub,
                    MMX_CVTTPS2PI_R | MMX_CVTTPS2PI_M | MMX_CVTTPD2PI_R | MMX_CVTTPD2PI_M
                );
                let is_mem = (sub & 1) != 0;
                let b: u128 = if is_mem {
                    if is_pd {
                        cpu.read_u128(cpu.faddr(op))
                    } else {
                        u128::from(cpu.read_mem(cpu.faddr(op), 64))
                    }
                } else {
                    cpu.xmm[op.r1 as usize & 15]
                };
                let lane = |i: u32| -> u64 {
                    let v = if is_pd {
                        f64::from_bits((b >> (64 * i)) as u64)
                    } else {
                        f32::from_bits((b >> (32 * i)) as u32) as f64
                    };
                    let rounded = match (trunc, (cpu.jx.mxcsr >> 13) & 3) {
                        (true, _) | (_, 3) => v.trunc(),
                        (_, 1) => v.floor(),
                        (_, 2) => v.ceil(),
                        _ => v.round_ties_even(),
                    };
                    crate::emu::sse_cvt_i32(rounded, true) as u64
                };
                let val = lane(0) | (lane(1) << 32);
                mmx_commit_if_ok(Some((op.r0 as usize & 7, val)));
            }
            _ => {}
        },
        _ => {}
    }
}
