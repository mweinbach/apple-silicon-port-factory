//! Native lowering and execution for `K_VAES_RR..=K_VSHA_RM` (`115..=126`), covering AES, F16C,
//! masked vector moves, system/flag/MXCSR/x87 CW/16-bit stack, high-byte GPRs (`AH`/`CH`/`DH`/`BH`),
//! string ops (`rep movs/stos/lods/scas/cmps`), `pdep`/`pext`/`rcl`/`rcr`, `lock`/atomic RMW +
//! `cmpxchg8b`/`16b`, `pcmpistri/m`/`pcmpestri/m`, and Intel SHA-NI.

use super::decode::{gpr_of, imm_of, is_w, lower_v, mem_bits, put_mem, vmode, xreg};
use super::exec::sext;
use super::*;
use crate::emu::simd_kernels as sk;
use crate::emu::{Cpu, OpKind, Register};
use iced_x86::{EncodingKind, Instruction};

pub(in crate::emu) const K_VAES_RR: u8 = 115;
pub(in crate::emu) const K_VAES_RM: u8 = 116;
pub(in crate::emu) const K_VCVT_PH: u8 = 117;
pub(in crate::emu) const K_VMASKMOV: u8 = 118;
pub(in crate::emu) const K_SYS_MISC: u8 = 119;
pub(in crate::emu) const K_HI8: u8 = 120;
pub(in crate::emu) const K_STRING: u8 = 121;
pub(in crate::emu) const K_BIT_EXT: u8 = 122;
pub(in crate::emu) const K_ATOMIC: u8 = 123;
pub(in crate::emu) const K_PCMPSTR: u8 = 124;
/// Intel SHA-NI (`sha1rnds4`, `sha1nexte`, `sha1msg1`, `sha1msg2`, `sha256rnds2`, `sha256msg1`, `sha256msg2`).
pub(in crate::emu) const K_VSHA_RR: u8 = 125;
pub(in crate::emu) const K_VSHA_RM: u8 = 126;
/// Intel GFNI (`(v)gf2p8mulb`, `(v)gf2p8affineqb`, `(v)gf2p8affineinvqb`, legacy + VEX.128/256).
pub(in crate::emu) const K_VGFNI_RR: u8 = 127;
pub(in crate::emu) const K_VGFNI_RM: u8 = 128;
/// Intel AVX-VNNI (`vpdpbusd`, `vpdpbusds`, `vpdpwssd`, `vpdpwssds`, VEX.128/256).
pub(in crate::emu) const K_VVNNI_RR: u8 = 129;
pub(in crate::emu) const K_VVNNI_RM: u8 = 130;

fn lower_hi8_ins(ins: &Instruction, mut f: FastOp) -> Option<FastOp> {
    use iced_x86::Mnemonic as M;
    let mn = ins.mnemonic();
    let n = ins.op_count();
    let k0 = if n > 0 { ins.op_kind(0) } else { OpKind::Register };
    let k1 = if n > 1 { ins.op_kind(1) } else { OpKind::Register };
    if ins.has_lock_prefix() {
        return None;
    }
    let reg_spec = |r: Register| -> Option<(u8, u8, bool)> {
        let (idx, bits, hi) = Cpu::reg_idx(r)?;
        Some((idx as u8 | ((hi as u8) << 7), bits as u8, hi))
    };
    match mn {
        M::Mov | M::Movzx | M::Movsx => {
            if n != 2 {
                return None;
            }
            let sub: u8 = match mn {
                M::Mov => 0,
                M::Movzx => 1,
                _ => 2,
            };
            match (k0, k1) {
                (OpKind::Register, OpKind::Register) => {
                    let (d_sp, dw, d_hi) = reg_spec(ins.op_register(0))?;
                    let (s_sp, sw, s_hi) = reg_spec(ins.op_register(1))?;
                    if (!d_hi && !s_hi) || sw != 8 {
                        return None;
                    }
                    f.r0 = d_sp;
                    f.r1 = s_sp;
                    f.w = dw;
                    f.x = sub;
                }
                (OpKind::Register, OpKind::Memory) if mn == M::Mov => {
                    let (d_sp, dw, d_hi) = reg_spec(ins.op_register(0))?;
                    if !d_hi || dw != 8 || mem_bits(ins) != 8 {
                        return None;
                    }
                    f.r0 = d_sp;
                    f.r1 = 0x40;
                    f.w = 8;
                    f.x = 0;
                    put_mem(ins, &mut f)?;
                }
                (OpKind::Memory, OpKind::Register) if mn == M::Mov => {
                    let (s_sp, sw, s_hi) = reg_spec(ins.op_register(1))?;
                    if !s_hi || sw != 8 || mem_bits(ins) != 8 {
                        return None;
                    }
                    f.r0 = 0x40;
                    f.r1 = s_sp;
                    f.w = 8;
                    f.x = 0;
                    put_mem(ins, &mut f)?;
                }
                (OpKind::Register, _) if mn == M::Mov => {
                    let (d_sp, dw, d_hi) = reg_spec(ins.op_register(0))?;
                    if !d_hi || dw != 8 {
                        return None;
                    }
                    f.r0 = d_sp;
                    f.r1 = 0x20;
                    f.w = 8;
                    f.x = 0;
                    f.imm = imm_of(ins, 1, 8)?;
                }
                _ => return None,
            }
            f.kind = K_HI8;
            Some(f)
        }
        M::Add | M::Sub | M::And | M::Or | M::Xor | M::Cmp | M::Test | M::Adc | M::Sbb => {
            if n != 2 {
                return None;
            }
            let sub: u8 = match mn {
                M::Add => 3,
                M::Sub => 4,
                M::And => 5,
                M::Or => 6,
                M::Xor => 7,
                M::Cmp => 8,
                M::Test => 9,
                M::Adc => 10,
                _ => 11,
            };
            match (k0, k1) {
                (OpKind::Register, OpKind::Register) => {
                    let (d_sp, dw, d_hi) = reg_spec(ins.op_register(0))?;
                    let (s_sp, sw, s_hi) = reg_spec(ins.op_register(1))?;
                    if (!d_hi && !s_hi) || dw != 8 || sw != 8 {
                        return None;
                    }
                    f.r0 = d_sp;
                    f.r1 = s_sp;
                }
                (OpKind::Register, OpKind::Memory) => {
                    let (d_sp, dw, d_hi) = reg_spec(ins.op_register(0))?;
                    if !d_hi || dw != 8 || mem_bits(ins) != 8 {
                        return None;
                    }
                    f.r0 = d_sp;
                    f.r1 = 0x40;
                    f.x = sub;
                    put_mem(ins, &mut f)?;
                    f.w = 8;
                    f.kind = K_HI8;
                    return Some(f);
                }
                (OpKind::Memory, OpKind::Register) => {
                    let (s_sp, sw, s_hi) = reg_spec(ins.op_register(1))?;
                    if !s_hi || sw != 8 || mem_bits(ins) != 8 {
                        return None;
                    }
                    f.r0 = 0x40;
                    f.r1 = s_sp;
                    f.x = sub;
                    put_mem(ins, &mut f)?;
                    f.w = 8;
                    f.kind = K_HI8;
                    return Some(f);
                }
                (OpKind::Register, _) => {
                    let (d_sp, dw, d_hi) = reg_spec(ins.op_register(0))?;
                    if !d_hi || dw != 8 {
                        return None;
                    }
                    f.r0 = d_sp;
                    f.r1 = 0x20;
                    f.imm = imm_of(ins, 1, 8)?;
                }
                _ => return None,
            }
            f.w = 8;
            f.x = sub;
            f.kind = K_HI8;
            Some(f)
        }
        M::Inc | M::Dec | M::Not | M::Neg => {
            if n != 1 || k0 != OpKind::Register {
                return None;
            }
            let (d_sp, dw, d_hi) = reg_spec(ins.op_register(0))?;
            if !d_hi || dw != 8 {
                return None;
            }
            f.r0 = d_sp;
            f.w = 8;
            f.x = match mn {
                M::Inc => 12,
                M::Dec => 13,
                M::Not => 14,
                _ => 15,
            };
            f.kind = K_HI8;
            Some(f)
        }
        M::Shl | M::Sal | M::Shr | M::Sar | M::Rol | M::Ror => {
            if n != 2 || k0 != OpKind::Register {
                return None;
            }
            let (d_sp, dw, d_hi) = reg_spec(ins.op_register(0))?;
            if !d_hi || dw != 8 {
                return None;
            }
            f.r0 = d_sp;
            f.w = 8;
            f.x = match mn {
                M::Shl | M::Sal => 16,
                M::Shr => 17,
                M::Sar => 18,
                M::Rol => 19,
                _ => 20,
            };
            match k1 {
                OpKind::Immediate8 => {
                    f.r1 = 0x20;
                    f.imm = (ins.immediate8() as u64) & 31;
                }
                OpKind::Register if ins.op_register(1) == Register::CL => {
                    f.r1 = 1;
                }
                _ => return None,
            }
            f.kind = K_HI8;
            Some(f)
        }
        M::Seta | M::Setae | M::Setb | M::Setbe | M::Sete | M::Setg | M::Setge | M::Setl
        | M::Setle | M::Setne | M::Setno | M::Setnp | M::Setns | M::Seto | M::Setp | M::Sets => {
            if n != 1 || k0 != OpKind::Register {
                return None;
            }
            let (d_sp, dw, d_hi) = reg_spec(ins.op_register(0))?;
            if !d_hi || dw != 8 {
                return None;
            }
            f.r0 = d_sp;
            f.w = 8;
            f.x = 21;
            f.imm = ins.condition_code() as u64;
            f.kind = K_HI8;
            Some(f)
        }
        M::Xchg | M::Xadd | M::Cmpxchg => {
            if n != 2 || k0 != OpKind::Register || k1 != OpKind::Register {
                return None;
            }
            let (d_sp, dw, d_hi) = reg_spec(ins.op_register(0))?;
            let (s_sp, sw, s_hi) = reg_spec(ins.op_register(1))?;
            if (!d_hi && !s_hi) || dw != 8 || sw != 8 {
                return None;
            }
            f.r0 = d_sp;
            f.r1 = s_sp;
            f.w = 8;
            f.x = match mn {
                M::Xchg => 22,
                M::Xadd => 23,
                _ => 24,
            };
            f.kind = K_HI8;
            Some(f)
        }
        _ => None,
    }
}

pub(super) fn lower_all_remaining(ins: &Instruction, mut f: FastOp) -> Option<FastOp> {
    use iced_x86::Mnemonic as M;
    if let Some(h) = lower_hi8_ins(ins, f) {
        return Some(h);
    }
    let mn = ins.mnemonic();
    let n = ins.op_count();
    let k0 = if n > 0 { ins.op_kind(0) } else { OpKind::Register };
    let k1 = if n > 1 { ins.op_kind(1) } else { OpKind::Register };
    match mn {
        M::Aesenc | M::Vaesenc | M::Aesenclast | M::Vaesenclast | M::Aesdec | M::Vaesdec
        | M::Aesdeclast | M::Vaesdeclast | M::Aesimc | M::Vaesimc | M::Aeskeygenassist
        | M::Vaeskeygenassist => {
            let sub: u8 = match mn {
                M::Aesenc | M::Vaesenc => 0,
                M::Aesenclast | M::Vaesenclast => 1,
                M::Aesdec | M::Vaesdec => 2,
                M::Aesdeclast | M::Vaesdeclast => 3,
                M::Aesimc | M::Vaesimc => 4,
                _ => 5,
            };
            lower_v(ins, &mut f, K_VAES_RR, K_VAES_RM, sub, sub <= 3, None)?;
        }
        M::Gf2p8mulb | M::Vgf2p8mulb | M::Gf2p8affineqb | M::Vgf2p8affineqb
        | M::Gf2p8affineinvqb | M::Vgf2p8affineinvqb => {
            let sub: u8 = match mn {
                M::Gf2p8mulb | M::Vgf2p8mulb => 0,
                M::Gf2p8affineqb | M::Vgf2p8affineqb => 1,
                _ => 2,
            };
            lower_v(ins, &mut f, K_VGFNI_RR, K_VGFNI_RM, sub, true, None)?;
        }
        M::Vpdpbusd | M::Vpdpbusds | M::Vpdpwssd | M::Vpdpwssds => {
            if ins.encoding() != EncodingKind::VEX {
                return None;
            }
            let sub: u8 = match mn {
                M::Vpdpbusd => 0,
                M::Vpdpbusds => 1,
                M::Vpdpwssd => 2,
                _ => 3,
            };
            lower_v(ins, &mut f, K_VVNNI_RR, K_VVNNI_RM, sub, true, None)?;
        }
        M::Sha1rnds4 | M::Sha1nexte | M::Sha1msg1 | M::Sha1msg2 | M::Sha256rnds2 | M::Sha256msg1 | M::Sha256msg2 => {
            if n < 2 || k0 != OpKind::Register || ins.encoding() != EncodingKind::Legacy {
                return None;
            }
            let d = xreg(ins.op_register(0))?;
            let sub: u8 = match mn {
                M::Sha1rnds4 => {
                    if n != 3 || ins.op_kind(2) != OpKind::Immediate8 {
                        return None;
                    }
                    f.imm = (ins.immediate8() & 3) as u64;
                    0
                }
                M::Sha1nexte => 1,
                M::Sha1msg1 => 2,
                M::Sha1msg2 => 3,
                M::Sha256rnds2 => 4,
                M::Sha256msg1 => 5,
                _ => 6,
            };
            f.r0 = d;
            f.w = VM_LEG;
            f.x = sub;
            match k1 {
                OpKind::Register => {
                    f.r1 = xreg(ins.op_register(1))?;
                    f.kind = K_VSHA_RR;
                }
                OpKind::Memory => {
                    put_mem(ins, &mut f)?;
                    f.kind = K_VSHA_RM;
                }
                _ => return None,
            }
        }
        M::Vcvtph2ps => {
            let mode = vmode(ins)?;
            if n != 2 || k0 != OpKind::Register {
                return None;
            }
            f.r0 = xreg(ins.op_register(0))?;
            f.w = mode;
            match k1 {
                OpKind::Register => {
                    f.r1 = xreg(ins.op_register(1))?;
                    f.x = 0;
                }
                OpKind::Memory => {
                    f.x = 0x20;
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
            f.kind = K_VCVT_PH;
        }
        M::Vcvtps2ph => {
            vmode(ins)?;
            if n != 3 || k1 != OpKind::Register || ins.op_kind(2) != OpKind::Immediate8 {
                return None;
            }
            let src_256 = Cpu::is_ymm(ins.op_register(1));
            f.r1 = xreg(ins.op_register(1))?;
            f.w = src_256 as u8;
            f.imm = ins.immediate8() as u64;
            match k0 {
                OpKind::Register => {
                    f.r0 = xreg(ins.op_register(0))?;
                    f.x = 1;
                }
                OpKind::Memory => {
                    f.x = 0x21;
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
            f.kind = K_VCVT_PH;
        }
        M::Vpmaskmovd | M::Vpmaskmovq | M::Vmaskmovps | M::Vmaskmovpd => {
            let mode = vmode(ins)?;
            if n != 3 || k1 != OpKind::Register {
                return None;
            }
            let is_q = matches!(mn, M::Vpmaskmovq | M::Vmaskmovpd);
            f.r1 = xreg(ins.op_register(1))?;
            match (k0, ins.op_kind(2)) {
                (OpKind::Register, OpKind::Memory) => {
                    f.r0 = xreg(ins.op_register(0))?;
                    f.w = mode | ((is_q as u8) << 2);
                    f.x = 0;
                    put_mem(ins, &mut f)?;
                }
                (OpKind::Memory, OpKind::Register) => {
                    let s_mode = if Cpu::is_ymm(ins.op_register(2)) { VM_V256 } else { VM_V128 };
                    f.r0 = xreg(ins.op_register(2))?;
                    f.w = s_mode | ((is_q as u8) << 2);
                    f.x = 0x20;
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
            f.kind = K_VMASKMOV;
        }
        M::Clflush | M::Clflushopt | M::Clwb | M::Xend => {
            f.kind = K_NOP;
        }
        M::Enter => {
            f.kind = K_SYS_MISC;
            f.x = 12;
            f.imm = if n > 0 && k0 == OpKind::Immediate16 {
                ins.immediate16() as u64
            } else {
                0
            };
        }
        M::Rdrand | M::Rdseed => {
            if n != 1 || k0 != OpKind::Register {
                return None;
            }
            let (a, w) = gpr_of(ins.op_register(0))?;
            f.kind = K_SYS_MISC;
            f.x = 13;
            f.r0 = a;
            f.w = w;
        }
        M::Shl | M::Sal | M::Shr | M::Sar | M::Rol | M::Ror
            if n == 2 && k0 == OpKind::Memory && k1 == OpKind::Immediate8 =>
        {
            // Count masked to 0 on memory operand is a pure no-op!
            f.kind = K_NOP;
        }
        M::Lahf | M::Sahf | M::Pushf | M::Pushfq | M::Popf | M::Popfq | M::Stmxcsr
        | M::Vstmxcsr | M::Ldmxcsr | M::Vldmxcsr | M::Fnstcw | M::Fstcw | M::Fldcw | M::Xlatb
        | M::Rdtsc | M::Rdtscp | M::Cpuid | M::Xgetbv | M::Fxsave | M::Fxsave64 | M::Fxrstor
        | M::Fxrstor64 | M::Xsave | M::Xsave64 | M::Xsaveopt | M::Xsaveopt64 | M::Xsavec
        | M::Xsavec64 | M::Xrstor | M::Xrstor64 => {
            f.kind = K_SYS_MISC;
            match mn {
                M::Lahf => f.x = 0,
                M::Sahf => f.x = 1,
                M::Pushf => {
                    f.x = 2;
                    f.w = 16;
                }
                M::Pushfq => {
                    f.x = 2;
                    f.w = 64;
                }
                M::Popf => {
                    f.x = 3;
                    f.w = 16;
                }
                M::Popfq => {
                    f.x = 3;
                    f.w = 64;
                }
                M::Stmxcsr | M::Vstmxcsr => {
                    f.x = 4;
                    put_mem(ins, &mut f)?;
                }
                M::Ldmxcsr | M::Vldmxcsr => {
                    f.x = 5;
                    put_mem(ins, &mut f)?;
                }
                M::Xlatb => f.x = 6,
                M::Fnstcw | M::Fstcw => {
                    f.x = 7;
                    put_mem(ins, &mut f)?;
                }
                M::Rdtsc => {
                    f.x = 8;
                    f.w = 0;
                }
                M::Rdtscp => {
                    f.x = 8;
                    f.w = 1;
                }
                M::Cpuid => f.x = 9,
                M::Fldcw => {
                    f.x = 14;
                    put_mem(ins, &mut f)?;
                }
                M::Xgetbv => f.x = 15,
                _ => {
                    use crate::emu::xstate::{
                        XS_FLAG_COMPACT, XS_FLAG_LEGACY, XS_FLAG_RESTORE, XS_FLAG_WIDE,
                    };
                    f.x = 16;
                    let mut flags = 0u8;
                    if matches!(mn, M::Fxsave | M::Fxsave64 | M::Fxrstor | M::Fxrstor64) {
                        flags |= XS_FLAG_LEGACY;
                    }
                    if matches!(
                        mn,
                        M::Fxsave64
                            | M::Fxrstor64
                            | M::Xsave64
                            | M::Xsaveopt64
                            | M::Xsavec64
                            | M::Xrstor64
                    ) {
                        flags |= XS_FLAG_WIDE;
                    }
                    if matches!(mn, M::Fxrstor | M::Fxrstor64 | M::Xrstor | M::Xrstor64) {
                        flags |= XS_FLAG_RESTORE;
                    }
                    if matches!(mn, M::Xsavec | M::Xsavec64) {
                        flags |= XS_FLAG_COMPACT;
                    }
                    f.w = flags;
                    put_mem(ins, &mut f)?;
                }
            }
        }
        M::Push | M::Pop => {
            // 16-bit push / pop
            if n != 1 {
                return None;
            }
            let push = mn == M::Push;
            f.kind = K_SYS_MISC;
            f.x = if push { 10 } else { 11 };
            match k0 {
                OpKind::Register => {
                    let (a, w) = gpr_of(ins.op_register(0))?;
                    if w != 16 {
                        return None;
                    }
                    f.r0 = a;
                    f.w = 0;
                }
                OpKind::Memory => {
                    if mem_bits(ins) != 16 {
                        return None;
                    }
                    f.w = 1;
                    put_mem(ins, &mut f)?;
                }
                _ if push => {
                    f.imm = imm_of(ins, 0, 16)?;
                    f.w = 2;
                }
                _ => return None,
            }
        }
        M::Movsb | M::Movsw | M::Movsd | M::Movsq | M::Stosb | M::Stosw | M::Stosd | M::Stosq
        | M::Lodsb | M::Lodsw | M::Lodsd | M::Lodsq | M::Scasb | M::Scasw | M::Scasd | M::Scasq
        | M::Cmpsb | M::Cmpsw | M::Cmpsd | M::Cmpsq => {
            let (op_k, ebits) = match mn {
                M::Movsb => (0u8, 8u8),
                M::Movsw => (0, 16),
                M::Movsd => (0, 32),
                M::Movsq => (0, 64),
                M::Stosb => (1, 8),
                M::Stosw => (1, 16),
                M::Stosd => (1, 32),
                M::Stosq => (1, 64),
                M::Lodsb => (2, 8),
                M::Lodsw => (2, 16),
                M::Lodsd => (2, 32),
                M::Lodsq => (2, 64),
                M::Scasb => (3, 8),
                M::Scasw => (3, 16),
                M::Scasd => (3, 32),
                M::Scasq => (3, 64),
                M::Cmpsb => (4, 8),
                M::Cmpsw => (4, 16),
                M::Cmpsd => (4, 32),
                _ => (4, 64),
            };
            let rep_mode = if op_k <= 2 {
                if ins.has_rep_prefix() { 1u8 } else { 0u8 }
            } else if ins.has_repne_prefix() {
                2u8
            } else if ins.has_repe_prefix() || ins.has_rep_prefix() {
                1u8
            } else {
                0u8
            };
            f.kind = K_STRING;
            f.w = ebits;
            f.x = op_k | (rep_mode << 3);
        }
        M::Pdep | M::Pext => {
            if n != 3 || k0 != OpKind::Register || k1 != OpKind::Register {
                return None;
            }
            let (d, w) = gpr_of(ins.op_register(0))?;
            let (s1, w1) = gpr_of(ins.op_register(1))?;
            if (w != 32 && w != 64) || w1 != w {
                return None;
            }
            f.kind = K_BIT_EXT;
            f.r0 = d;
            f.w = w;
            f.imm = s1 as u64;
            let sub = if mn == M::Pdep { 0u8 } else { 1u8 };
            match ins.op_kind(2) {
                OpKind::Register => {
                    let (s2, w2) = gpr_of(ins.op_register(2))?;
                    if w2 != w {
                        return None;
                    }
                    f.r1 = s2;
                    f.x = sub;
                }
                OpKind::Memory => {
                    if mem_bits(ins) != w as u32 {
                        return None;
                    }
                    f.x = sub | 0x20;
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
        }
        M::Rcl | M::Rcr => {
            if n != 2 {
                return None;
            }
            let mut flags = if mn == M::Rcl { 2u8 } else { 3u8 };
            match k0 {
                OpKind::Register => {
                    let (d, w) = gpr_of(ins.op_register(0))?;
                    f.r0 = d;
                    f.w = w;
                }
                OpKind::Memory => {
                    let w = mem_bits(ins);
                    if !is_w(w) {
                        return None;
                    }
                    f.w = w as u8;
                    flags |= 0x20;
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
            match k1 {
                OpKind::Immediate8 => {
                    f.imm = ins.immediate8() as u64;
                    flags |= 0x10;
                }
                OpKind::Register if ins.op_register(1) == Register::CL => {}
                _ => return None,
            }
            f.x = (f.x & 0xC0) | flags;
            f.kind = K_BIT_EXT;
        }
        M::Add | M::Sub | M::And | M::Or | M::Xor | M::Inc | M::Dec | M::Not | M::Neg
        | M::Bt | M::Bts | M::Btr | M::Btc | M::Xchg | M::Xadd | M::Cmpxchg | M::Cmpxchg8b
        | M::Cmpxchg16b => {
            let sub: u8 = match mn {
                M::Add => 0,
                M::Sub => 1,
                M::And => 2,
                M::Or => 3,
                M::Xor => 4,
                M::Inc => 5,
                M::Dec => 6,
                M::Not => 7,
                M::Neg => 8,
                M::Bt => 9,
                M::Bts => 10,
                M::Btr => 11,
                M::Btc => 12,
                M::Xchg => 13,
                M::Xadd => 14,
                M::Cmpxchg => 15,
                M::Cmpxchg8b => 16,
                _ => 17, // Cmpxchg16b
            };
            f.x = sub;
            if sub >= 16 {
                put_mem(ins, &mut f)?;
                f.w = if sub == 16 { 64 } else { 128 };
            } else if sub == 13 && k0 == OpKind::Register && k1 == OpKind::Memory {
                // xchg reg, [mem]
                let (r, w) = gpr_of(ins.op_register(0))?;
                f.r1 = r;
                f.w = w;
                put_mem(ins, &mut f)?;
            } else {
                if k0 != OpKind::Memory {
                    return None;
                }
                let w = mem_bits(ins);
                if !is_w(w) {
                    return None;
                }
                f.w = w as u8;
                put_mem(ins, &mut f)?;
                if n == 2 {
                    match k1 {
                        OpKind::Register => {
                            let (r, rw) = gpr_of(ins.op_register(1))?;
                            if rw != w as u8 {
                                return None;
                            }
                            f.r1 = r;
                        }
                        _ => {
                            f.r0 = 1; // immediate flag
                            f.imm = imm_of(ins, 1, w)?;
                        }
                    }
                }
            }
            f.kind = K_ATOMIC;
        }
        M::Pcmpistri | M::Vpcmpistri | M::Pcmpistrm | M::Vpcmpistrm | M::Pcmpestri
        | M::Vpcmpestri | M::Pcmpestrm | M::Vpcmpestrm | M::Pcmpestri64 | M::Vpcmpestri64
        | M::Pcmpestrm64 | M::Vpcmpestrm64 => {
            if n != 3 || k0 != OpKind::Register || ins.op_kind(2) != OpKind::Immediate8 {
                return None;
            }
            let is_long = matches!(
                mn,
                M::Pcmpestri64 | M::Vpcmpestri64 | M::Pcmpestrm64 | M::Vpcmpestrm64
            );
            let is_exp = is_long
                || matches!(mn, M::Pcmpestri | M::Vpcmpestri | M::Pcmpestrm | M::Vpcmpestrm);
            let is_msk = matches!(
                mn,
                M::Pcmpistrm
                    | M::Vpcmpistrm
                    | M::Pcmpestrm
                    | M::Vpcmpestrm
                    | M::Pcmpestrm64
                    | M::Vpcmpestrm64
            );
            f.r0 = xreg(ins.op_register(0))?;
            f.imm = ins.immediate8() as u64;
            let mut flags = (is_exp as u8)
                | ((is_msk as u8) << 1)
                | ((Cpu::is_vex(ins) as u8) << 2)
                | ((is_long as u8) << 3);
            match k1 {
                OpKind::Register => {
                    f.r1 = xreg(ins.op_register(1))?;
                }
                OpKind::Memory => {
                    flags |= 0x20;
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
            f.x = (f.x & 0xC0) | flags;
            f.kind = K_PCMPSTR;
        }
        _ => return super::mmx_gather::lower_simd_ext_gather(ins, f),
    }
    Some(f)
}

/// Fast native execution of `K_STRING` (`movs`/`stos`/`lods`/`scas`/`cmps`, with `rep`/`repe`/`repne`).
#[inline(never)]
unsafe fn exec_string_op(cpu: &mut Cpu, op: &FastOp) {
    let op_k = op.x & 7;
    let rep_mode = (op.x >> 3) & 3;
    let ebits = op.w as u32;
    let elem_bytes = (op.w / 8) as u64;
    match op_k {
        0 => {
            // MOVSB/W/D/Q
            let rep = rep_mode != 0;
            let count = if rep { cpu.gpr[1] } else { 1 };
            if count > 0 {
                let total_u64 = count.checked_mul(elem_bytes);
                if !cpu.df {
                    let (src, dst) = (cpu.gpr[6], cpu.gpr[7]);
                    if let Some(total) = total_u64.and_then(|t| usize::try_from(t).ok()) {
                        if crate::emu::regfile_ext::is_valid_guest_range(src, total)
                            && crate::emu::regfile_ext::is_valid_guest_range(dst, total)
                        {
                            let dist = dst.wrapping_sub(src);
                            if crate::emu::mem::tso_memcpy() {
                                core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
                            }
                            if dist == 0 || dist >= total as u64 {
                                core::ptr::copy(src as *const u8, dst as *mut u8, total);
                            } else if dist >= elem_bytes {
                                // Destructive forward overlap (`dst` in `(src, src+total)`): replicate
                                // the initial `dist`-byte period using exponential doubling.
                                let period = dist as usize;
                                core::ptr::copy_nonoverlapping(src as *const u8, dst as *mut u8, period);
                                let mut done = period;
                                while done < total {
                                    let chunk = done.min(total - done);
                                    core::ptr::copy_nonoverlapping(
                                        dst as *const u8,
                                        (dst as *mut u8).add(done),
                                        chunk,
                                    );
                                    done += chunk;
                                }
                            } else {
                                for i in 0..count {
                                    let v = cpu.read_mem(src.wrapping_add(i * elem_bytes), ebits);
                                    cpu.write_mem(dst.wrapping_add(i * elem_bytes), ebits, v);
                                }
                            }
                            if crate::emu::mem::tso_memcpy() {
                                core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
                            }
                            let dst_end = dst.wrapping_add(total as u64);
                            if crate::emu::mem::jit_overlaps(dst as usize, dst_end as usize) {
                                crate::emu::mem::note_code_region(dst as usize, dst_end as usize);
                            }
                            cpu.gpr[6] = src.wrapping_add(total as u64);
                            cpu.gpr[7] = dst_end;
                        } else {
                            for _ in 0..count {
                                let v = cpu.read_mem(cpu.gpr[6], ebits);
                                cpu.write_mem(cpu.gpr[7], ebits, v);
                                cpu.gpr[6] = cpu.gpr[6].wrapping_add(elem_bytes);
                                cpu.gpr[7] = cpu.gpr[7].wrapping_add(elem_bytes);
                            }
                        }
                    } else {
                        for _ in 0..count {
                            let v = cpu.read_mem(cpu.gpr[6], ebits);
                            cpu.write_mem(cpu.gpr[7], ebits, v);
                            cpu.gpr[6] = cpu.gpr[6].wrapping_add(elem_bytes);
                            cpu.gpr[7] = cpu.gpr[7].wrapping_add(elem_bytes);
                        }
                    }
                } else {
                    // DF = 1 (backward)
                    let (src, dst) = (cpu.gpr[6], cpu.gpr[7]);
                    let bulk_ok = total_u64.and_then(|total_u| {
                        let total = usize::try_from(total_u).ok()?;
                        let src_hi = src.checked_add(elem_bytes)?;
                        let dst_hi = dst.checked_add(elem_bytes)?;
                        let src_lo = src_hi.checked_sub(total_u)?;
                        let dst_lo = dst_hi.checked_sub(total_u)?;
                        // `ptr::copy` matches element-by-element backward copy unless `dst` is
                        // strictly below `src` within `total` bytes (tail-period replication).
                        let no_tail_rep = dst >= src || src.wrapping_sub(dst) >= total_u;
                        if no_tail_rep
                            && crate::emu::regfile_ext::is_valid_guest_range(src_lo, total)
                            && crate::emu::regfile_ext::is_valid_guest_range(dst_lo, total)
                        {
                            Some((src_lo, dst_lo, dst_hi, total, total_u))
                        } else {
                            None
                        }
                    });
                    if let Some((src_lo, dst_lo, dst_hi, total, total_u)) = bulk_ok {
                        if crate::emu::mem::tso_memcpy() {
                            core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
                        }
                        core::ptr::copy(src_lo as *const u8, dst_lo as *mut u8, total);
                        if crate::emu::mem::tso_memcpy() {
                            core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
                        }
                        if crate::emu::mem::jit_overlaps(dst_lo as usize, dst_hi as usize) {
                            crate::emu::mem::note_code_region(dst_lo as usize, dst_hi as usize);
                        }
                        cpu.gpr[6] = src.wrapping_sub(total_u);
                        cpu.gpr[7] = dst.wrapping_sub(total_u);
                    } else {
                        for _ in 0..count {
                            let v = cpu.read_mem(cpu.gpr[6], ebits);
                            cpu.write_mem(cpu.gpr[7], ebits, v);
                            cpu.gpr[6] = cpu.gpr[6].wrapping_sub(elem_bytes);
                            cpu.gpr[7] = cpu.gpr[7].wrapping_sub(elem_bytes);
                        }
                    }
                }
            }
            cpu.last_bits[6] = 64;
            cpu.last_bits[7] = 64;
            if rep {
                cpu.gpr[1] = 0;
                cpu.last_bits[1] = 64;
            }
        }
        1 => {
            // STOSB/W/D/Q
            let rep = rep_mode != 0;
            let count = if rep { cpu.gpr[1] } else { 1 };
            let val = cpu.gpr[0];
            if count > 0 {
                let dst = cpu.gpr[7];
                let bulk_range = count.checked_mul(elem_bytes).and_then(|total_u| {
                    let total = usize::try_from(total_u).ok()?;
                    let (dst_lo, dst_hi) = if !cpu.df {
                        (dst, dst.checked_add(total_u)?)
                    } else {
                        let hi = dst.checked_add(elem_bytes)?;
                        (hi.checked_sub(total_u)?, hi)
                    };
                    if crate::emu::regfile_ext::is_valid_guest_range(dst_lo, total) {
                        Some((dst_lo, dst_hi, total, total_u))
                    } else {
                        None
                    }
                });
                if let Some((dst_lo, dst_hi, total, total_u)) = bulk_range {
                    if crate::emu::mem::tso_memcpy() {
                        core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
                    }
                    let b = val as u8;
                    let uniform = match elem_bytes {
                        1 => true,
                        2 => (val as u16) == u16::from_ne_bytes([b; 2]),
                        4 => (val as u32) == u32::from_ne_bytes([b; 4]),
                        _ => val == u64::from_ne_bytes([b; 8]),
                    };
                    if uniform {
                        core::ptr::write_bytes(dst_lo as *mut u8, b, total);
                    } else {
                        let pat: u128 = match elem_bytes {
                            2 => ((val as u16) as u128).wrapping_mul(0x0001_0001_0001_0001_0001_0001_0001_0001u128),
                            4 => {
                                let d = (val as u32) as u64;
                                let q = d | (d << 32);
                                (q as u128) | ((q as u128) << 64)
                            }
                            _ => (val as u128) | ((val as u128) << 64),
                        };
                        let mut p = dst_lo as *mut u8;
                        let mut rem = total;
                        while rem >= 64 {
                            core::ptr::write_unaligned(p as *mut u128, pat);
                            core::ptr::write_unaligned(p.add(16) as *mut u128, pat);
                            core::ptr::write_unaligned(p.add(32) as *mut u128, pat);
                            core::ptr::write_unaligned(p.add(48) as *mut u128, pat);
                            p = p.add(64);
                            rem -= 64;
                        }
                        while rem >= 16 {
                            core::ptr::write_unaligned(p as *mut u128, pat);
                            p = p.add(16);
                            rem -= 16;
                        }
                        let step = elem_bytes as usize;
                        while rem >= step {
                            match elem_bytes {
                                2 => core::ptr::write_unaligned(p as *mut u16, val as u16),
                                4 => core::ptr::write_unaligned(p as *mut u32, val as u32),
                                _ => core::ptr::write_unaligned(p as *mut u64, val),
                            }
                            p = p.add(step);
                            rem -= step;
                        }
                    }
                    if crate::emu::mem::jit_overlaps(dst_lo as usize, dst_hi as usize) {
                        crate::emu::mem::note_code_region(dst_lo as usize, dst_hi as usize);
                    }
                    cpu.gpr[7] = if !cpu.df {
                        dst.wrapping_add(total_u)
                    } else {
                        dst.wrapping_sub(total_u)
                    };
                } else {
                    let step = if cpu.df { elem_bytes.wrapping_neg() } else { elem_bytes };
                    for _ in 0..count {
                        cpu.write_mem(cpu.gpr[7], ebits, val);
                        cpu.gpr[7] = cpu.gpr[7].wrapping_add(step);
                    }
                }
            }
            cpu.last_bits[7] = 64;
            if rep {
                cpu.gpr[1] = 0;
                cpu.last_bits[1] = 64;
            }
        }
        2 => {
            // LODSB/W/D/Q
            let rep = rep_mode != 0;
            let step = if cpu.df { elem_bytes.wrapping_neg() } else { elem_bytes };
            let count = if rep { cpu.gpr[1] } else { 1 };
            if count > 0 {
                let last = cpu.gpr[6].wrapping_add(step.wrapping_mul(count - 1));
                let v = cpu.read_mem(last, ebits);
                cpu.rset(0, op.w, v);
                cpu.gpr[6] = cpu.gpr[6].wrapping_add(step.wrapping_mul(count));
                cpu.last_bits[6] = 64;
            }
            if rep {
                cpu.gpr[1] = 0;
                cpu.last_bits[1] = 64;
            }
        }
        3 => {
            // SCASB/W/D/Q (with NEON 16-byte page-safe fast path for forward `repe`/`repne scasb`)
            #[cfg(target_arch = "aarch64")]
            if rep_mode != 0 && ebits == 8 && !cpu.df && cpu.gpr[1] >= 16 {
                use core::arch::aarch64::*;
                let acc = (cpu.gpr[0] & 0xFF) as u8;
                let stop_on_eq = rep_mode == 2;
                let mut ptr = cpu.gpr[7];
                let mut rem = cpu.gpr[1];
                let mut found_byte: Option<u8> = None;
                let vacc = vdupq_n_u8(acc);
                while rem > 0 {
                    let page_rem = 4096usize - (ptr & 0xFFF) as usize;
                    let safe_len = page_rem.min(rem.min(usize::MAX as u64) as usize);
                    if safe_len >= 16 && crate::emu::regfile_ext::is_valid_guest_range(ptr, safe_len) {
                        let chunks = safe_len / 16;
                        let mut hit = false;
                        for _ in 0..chunks {
                            let v = vld1q_u8(ptr as *const u8);
                            let mask = sk::pmovmskb(core::mem::transmute(vceqq_u8(v, vacc))) as u32;
                            let stop_mask = if stop_on_eq { mask } else { (!mask) & 0xFFFF };
                            if stop_mask != 0 {
                                let idx = stop_mask.trailing_zeros() as u64;
                                let m = *(ptr.wrapping_add(idx) as *const u8);
                                ptr = ptr.wrapping_add(idx + 1);
                                rem -= idx + 1;
                                found_byte = Some(m);
                                hit = true;
                                break;
                            }
                            ptr = ptr.wrapping_add(16);
                            rem -= 16;
                        }
                        if hit {
                            break;
                        }
                        if rem == 0 {
                            found_byte = Some(*(ptr.wrapping_sub(1) as *const u8));
                            break;
                        }
                    } else {
                        let m = cpu.read_mem(ptr, 8) as u8;
                        ptr = ptr.wrapping_add(1);
                        rem -= 1;
                        let stop = if stop_on_eq { acc == m } else { acc != m };
                        if stop || rem == 0 {
                            found_byte = Some(m);
                            break;
                        }
                    }
                }
                if crate::emu::mem::tso_memcpy() {
                    core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
                }
                cpu.gpr[7] = ptr;
                cpu.gpr[1] = rem;
                if let Some(m) = found_byte {
                    let _ = cpu.update_flags_sub(acc as u64, m as u64, 8);
                }
                cpu.last_bits[7] = 64;
                cpu.last_bits[1] = 64;
                return;
            }
            let step = if cpu.df { elem_bytes.wrapping_neg() } else { elem_bytes };
            let mask = if ebits >= 64 { u64::MAX } else { (1u64 << ebits) - 1 };
            let acc = cpu.gpr[0] & mask;
            if rep_mode == 2 {
                while cpu.gpr[1] != 0 {
                    let m = cpu.read_mem(cpu.gpr[7], ebits) & mask;
                    cpu.gpr[7] = cpu.gpr[7].wrapping_add(step);
                    cpu.gpr[1] = cpu.gpr[1].wrapping_sub(1);
                    let _ = cpu.update_flags_sub(acc, m, ebits);
                    if acc == m {
                        break;
                    }
                }
            } else if rep_mode == 1 {
                while cpu.gpr[1] != 0 {
                    let m = cpu.read_mem(cpu.gpr[7], ebits) & mask;
                    cpu.gpr[7] = cpu.gpr[7].wrapping_add(step);
                    cpu.gpr[1] = cpu.gpr[1].wrapping_sub(1);
                    let _ = cpu.update_flags_sub(acc, m, ebits);
                    if acc != m {
                        break;
                    }
                }
            } else {
                let m = cpu.read_mem(cpu.gpr[7], ebits) & mask;
                cpu.gpr[7] = cpu.gpr[7].wrapping_add(step);
                let _ = cpu.update_flags_sub(acc, m, ebits);
            }
            cpu.last_bits[7] = 64;
            if rep_mode != 0 {
                cpu.last_bits[1] = 64;
            }
        }
        _ => {
            // CMPSB/W/D/Q (with NEON 16-byte page-safe fast path for forward `repe`/`repne cmpsb`)
            #[cfg(target_arch = "aarch64")]
            if rep_mode != 0 && ebits == 8 && !cpu.df && cpu.gpr[1] >= 16 {
                use core::arch::aarch64::*;
                let stop_on_eq = rep_mode == 2;
                let mut p1 = cpu.gpr[6];
                let mut p2 = cpu.gpr[7];
                let mut rem = cpu.gpr[1];
                let mut found: Option<(u8, u8)> = None;
                while rem > 0 {
                    let page_rem = (4096usize - (p1 & 0xFFF) as usize).min(4096usize - (p2 & 0xFFF) as usize);
                    let safe_len = page_rem.min(rem.min(usize::MAX as u64) as usize);
                    if safe_len >= 16
                        && crate::emu::regfile_ext::is_valid_guest_range(p1, safe_len)
                        && crate::emu::regfile_ext::is_valid_guest_range(p2, safe_len)
                    {
                        let chunks = safe_len / 16;
                        let mut hit = false;
                        for _ in 0..chunks {
                            let va = vld1q_u8(p1 as *const u8);
                            let vb = vld1q_u8(p2 as *const u8);
                            let mask = sk::pmovmskb(core::mem::transmute(vceqq_u8(va, vb))) as u32;
                            let stop_mask = if stop_on_eq { mask } else { (!mask) & 0xFFFF };
                            if stop_mask != 0 {
                                let idx = stop_mask.trailing_zeros() as u64;
                                let a = *(p1.wrapping_add(idx) as *const u8);
                                let b = *(p2.wrapping_add(idx) as *const u8);
                                p1 = p1.wrapping_add(idx + 1);
                                p2 = p2.wrapping_add(idx + 1);
                                rem -= idx + 1;
                                found = Some((a, b));
                                hit = true;
                                break;
                            }
                            p1 = p1.wrapping_add(16);
                            p2 = p2.wrapping_add(16);
                            rem -= 16;
                        }
                        if hit {
                            break;
                        }
                        if rem == 0 {
                            found = Some((*(p1.wrapping_sub(1) as *const u8), *(p2.wrapping_sub(1) as *const u8)));
                            break;
                        }
                    } else {
                        let a = cpu.read_mem(p1, 8) as u8;
                        let b = cpu.read_mem(p2, 8) as u8;
                        p1 = p1.wrapping_add(1);
                        p2 = p2.wrapping_add(1);
                        rem -= 1;
                        let stop = if stop_on_eq { a == b } else { a != b };
                        if stop || rem == 0 {
                            found = Some((a, b));
                            break;
                        }
                    }
                }
                if crate::emu::mem::tso_memcpy() {
                    core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
                }
                cpu.gpr[6] = p1;
                cpu.gpr[7] = p2;
                cpu.gpr[1] = rem;
                if let Some((a, b)) = found {
                    let _ = cpu.update_flags_sub(a as u64, b as u64, 8);
                }
                cpu.last_bits[6] = 64;
                cpu.last_bits[7] = 64;
                cpu.last_bits[1] = 64;
                return;
            }
            let step = if cpu.df { elem_bytes.wrapping_neg() } else { elem_bytes };
            let rep = rep_mode != 0;
            loop {
                if rep && cpu.gpr[1] == 0 {
                    break;
                }
                let a = cpu.read_mem(cpu.gpr[6], ebits);
                let b = cpu.read_mem(cpu.gpr[7], ebits);
                cpu.gpr[6] = cpu.gpr[6].wrapping_add(step);
                cpu.gpr[7] = cpu.gpr[7].wrapping_add(step);
                let _ = cpu.update_flags_sub(a, b, ebits);
                if !rep {
                    break;
                }
                cpu.gpr[1] = cpu.gpr[1].wrapping_sub(1);
                if (rep_mode == 1 && !cpu.zf) || (rep_mode == 2 && cpu.zf) {
                    break;
                }
            }
            cpu.last_bits[6] = 64;
            cpu.last_bits[7] = 64;
            if rep {
                cpu.last_bits[1] = 64;
            }
        }
    }
}

#[inline(never)]
pub(super) unsafe fn exec_ext_all_remaining(cpu: &mut Cpu, op: &FastOp) -> bool {
    match op.kind {
        K_VAES_RR | K_VAES_RM => {
            let d = op.r0 as usize & 15;
            let s1 = ((op.imm >> 8) as usize) & 15;
            let sub = op.x & 0x3F;
            let mode = op.w & 3;
            let is_256 = mode == VM_V256;
            let state_lo = cpu.xmm[s1];
            let state_hi = if is_256 { cpu.ymm_hi[s1] } else { 0 };
            let (src_lo, src_hi) = if op.kind == K_VAES_RM {
                let a = cpu.faddr(op);
                (cpu.read_u128(a), if is_256 { cpu.read_u128(a.wrapping_add(16)) } else { 0 })
            } else {
                let r2 = op.r1 as usize & 15;
                (cpu.xmm[r2], if is_256 { cpu.ymm_hi[r2] } else { 0 })
            };
            let imm8 = op.imm as u8;
            cpu.xmm[d] = sk::aes_op_128(sub, state_lo, src_lo, imm8);
            match mode {
                VM_V128 => cpu.clear_ymm_hi_reg(d),
                VM_V256 => cpu.set_ymm_hi(d, sk::aes_op_128(sub, state_hi, src_hi, imm8)),
                _ => {}
            }
        }
        K_VCVT_PH => {
            let sub = op.x & 0x1F;
            let mem = (op.x & 0x20) != 0;
            if sub == 0 {
                // Vcvtph2ps
                let d = op.r0 as usize & 15;
                let is_256 = (op.w & 3) == VM_V256;
                let raw = if mem {
                    let a = cpu.faddr(op);
                    if is_256 {
                        cpu.read_u128(a)
                    } else {
                        cpu.read_mem(a, 64) as u128
                    }
                } else {
                    cpu.xmm[op.r1 as usize & 15]
                };
                let (d_lo, d_hi) = sk::vcvtph2ps(raw, is_256);
                cpu.xmm[d] = d_lo;
                if is_256 {
                    cpu.set_ymm_hi(d, d_hi);
                } else {
                    cpu.clear_ymm_hi_reg(d);
                }
            } else {
                // Vcvtps2ph
                let is_256 = op.w != 0;
                let s = op.r1 as usize & 15;
                let (lo, hi) = (cpu.xmm[s], if is_256 { cpu.ymm_hi[s] } else { 0 });
                let packed = sk::vcvtps2ph(lo, hi, is_256, op.imm as u8, cpu.jx.mxcsr);
                if mem {
                    let a = cpu.faddr(op);
                    if is_256 {
                        cpu.write_u128(a, packed);
                    } else {
                        cpu.write_mem(a, 64, packed as u64);
                    }
                } else {
                    let d = op.r0 as usize & 15;
                    cpu.xmm[d] = packed;
                    cpu.clear_ymm_hi_reg(d);
                }
            }
        }
        K_VMASKMOV => {
            let is_store = (op.x & 0x20) != 0;
            let is_256 = (op.w & 3) == VM_V256;
            let is_q = (op.w & 4) != 0;
            let esz: u32 = if is_q { 8 } else { 4 };
            let per_lane = (16 / esz) as usize;
            let n = if is_256 { 2 * per_lane } else { per_lane };
            let m = op.r1 as usize & 15;
            let (m_lo, m_hi) = (cpu.xmm[m], if is_256 { cpu.ymm_hi[m] } else { 0 });
            let addr = cpu.faddr(op);
            let mask_on = |i: usize| -> bool {
                let v = if i < per_lane { m_lo } else { m_hi };
                let sh = (i % per_lane) as u32 * esz * 8;
                (v >> (sh + esz * 8 - 1)) & 1 != 0
            };
            let reg = op.r0 as usize & 15;
            if !is_store {
                let mut out = [0u128; 2];
                for i in 0..n {
                    if mask_on(i) {
                        let v = cpu.read_mem(addr.wrapping_add((i as u64) * esz as u64), esz * 8) as u128;
                        out[i / per_lane] |= v << ((i % per_lane) as u32 * esz * 8);
                    }
                }
                cpu.xmm[reg] = out[0];
                if is_256 {
                    cpu.set_ymm_hi(reg, out[1]);
                } else {
                    cpu.clear_ymm_hi_reg(reg);
                }
            } else {
                let (s_lo, s_hi) = (cpu.xmm[reg], if is_256 { cpu.ymm_hi[reg] } else { 0 });
                for i in 0..n {
                    if mask_on(i) {
                        let src = if i < per_lane { s_lo } else { s_hi };
                        let v = (src >> ((i % per_lane) as u32 * esz * 8)) as u64;
                        cpu.write_mem(addr.wrapping_add((i as u64) * esz as u64), esz * 8, v);
                    }
                }
            }
        }
        K_SYS_MISC => {
            match op.x & 0x3F {
                0 => {
                    // Lahf
                    let ah = ((cpu.sf as u64) << 7)
                        | ((cpu.zf as u64) << 6)
                        | ((cpu.pf() as u64) << 2)
                        | 2
                        | (cpu.cf as u64);
                    cpu.gpr[0] = (cpu.gpr[0] & !0xFF00) | (ah << 8);
                    cpu.last_bits[0] = 64;
                }
                1 => {
                    // Sahf
                    let ah = (cpu.gpr[0] >> 8) & 0xFF;
                    cpu.sf = ah & 0x80 != 0;
                    cpu.zf = ah & 0x40 != 0;
                    cpu.set_pf(ah & 0x04 != 0);
                    cpu.cf = ah & 0x01 != 0;
                }
                2 => {
                    // Pushf / Pushfq
                    let fl = cpu.rflags() as u64;
                    if op.w == 64 {
                        cpu.gpr[4] = cpu.gpr[4].wrapping_sub(8);
                        cpu.write_mem(cpu.gpr[4], 64, fl);
                    } else {
                        cpu.gpr[4] = cpu.gpr[4].wrapping_sub(2);
                        cpu.write_mem(cpu.gpr[4], 16, fl);
                    }
                }
                3 => {
                    // Popf / Popfq
                    let fl = if op.w == 64 {
                        let v = cpu.read_mem(cpu.gpr[4], 64);
                        cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
                        v
                    } else {
                        let v = cpu.read_mem(cpu.gpr[4], 16);
                        cpu.gpr[4] = cpu.gpr[4].wrapping_add(2);
                        v
                    };
                    cpu.cf = fl & 0x001 != 0;
                    cpu.set_pf(fl & 0x004 != 0);
                    cpu.zf = fl & 0x040 != 0;
                    cpu.sf = fl & 0x080 != 0;
                    cpu.df = fl & 0x400 != 0;
                    cpu.of = fl & 0x800 != 0;
                }
                4 => cpu.write_mem(cpu.faddr(op), 32, cpu.jx.mxcsr as u64), // Stmxcsr / Vstmxcsr
                5 => {
                    // Ldmxcsr / Vldmxcsr
                    let m = cpu.read_mem(cpu.faddr(op), 32) as u32;
                    if m >> 16 != 0 {
                        cpu.rip = op.ip;
                        crate::emu::xstate::Fault::gp().dispatch(cpu, op.ip);
                        return true;
                    }
                    if !crate::emu::precise_faults()
                        || crate::emu::FAULT_PENDING.with(|p| p.get().is_none())
                    {
                        cpu.jx.mxcsr = m;
                        crate::emu::apply_guest_mxcsr_hw(m);
                        if crate::emu::denorm::software_mode(m) {
                            cpu.rip = op.ip.wrapping_add(op.len as u64);
                            return true;
                        }
                    }
                }
                6 => {
                    // Xlatb
                    let v = cpu.read_mem(cpu.gpr[3].wrapping_add(cpu.gpr[0] & 0xFF), 8);
                    cpu.rset(0, 8, v);
                }
                7 => {
                    // Fnstcw / Fstcw
                    let a = cpu.faddr(op);
                    cpu.write_mem(a, 16, crate::emu::regfile_ext::x87_snapshot().cw.into());
                }
                8 => cpu.read_timestamp(op.w != 0), // RDTSC / RDTSCP
                9 => cpu.cpuid(),
                10 => {
                    // 16-bit push
                    let v = match op.w {
                        0 => cpu.rget(op.r0, 16),
                        1 => cpu.read_mem(cpu.faddr(op), 16),
                        _ => op.imm & 0xFFFF,
                    };
                    cpu.gpr[4] = cpu.gpr[4].wrapping_sub(2);
                    cpu.write_mem(cpu.gpr[4], 16, v);
                }
                11 => {
                    // 16-bit pop
                    let v = cpu.read_mem(cpu.gpr[4], 16);
                    cpu.gpr[4] = cpu.gpr[4].wrapping_add(2);
                    if op.w == 0 {
                        cpu.rset(op.r0, 16, v);
                    } else {
                        let a = cpu.faddr(op);
                        cpu.write_mem(a, 16, v);
                    }
                }
                12 => {
                    // Enter imm16, 0: push rbp; mov rbp, rsp; sub rsp, imm16
                    let nrsp = cpu.gpr[4].wrapping_sub(8);
                    cpu.write_mem(nrsp, 64, cpu.gpr[5]);
                    cpu.gpr[5] = nrsp;
                    cpu.gpr[4] = nrsp.wrapping_sub(op.imm & 0xFFFF);
                    cpu.last_bits[4] = 64;
                    cpu.last_bits[5] = 64;
                }
                13 => {
                    // Rdrand / Rdseed
                    cpu.exec_entropy(op.r0, op.w);
                }
                14 => {
                    // Fldcw
                    let cw = cpu.read_mem(cpu.faddr(op), 16) as u16;
                    if !crate::emu::precise_faults()
                        || crate::emu::FAULT_PENDING.with(|p| p.get().is_none())
                    {
                        crate::emu::regfile_ext::with_x87_mut(|st| {
                            st.cw = crate::emu::xstate::control_word(cw);
                        });
                    }
                }
                15 => {
                    // Xgetbv
                    if cpu.gpr[1] as u32 != 0 {
                        cpu.rip = op.ip;
                        crate::emu::xstate::Fault::gp().dispatch(cpu, op.ip);
                        return true;
                    }
                    cpu.gpr[0] = crate::emu::cpuid::XCR0.into();
                    cpu.gpr[2] = 0;
                    cpu.last_bits[0] = 64;
                    cpu.last_bits[2] = 64;
                }
                16 => {
                    // FXSAVE*/FXRSTOR*/XSAVE*/XRSTOR*
                    let addr = cpu.faddr(op);
                    if let Err(fault) = cpu.exec_xstate_mem(addr, op.w) {
                        cpu.rip = op.ip;
                        fault.dispatch(cpu, op.ip);
                        return true;
                    }
                    if op.w & crate::emu::xstate::XS_FLAG_RESTORE != 0
                        && crate::emu::denorm::software_mode(cpu.jx.mxcsr)
                    {
                        cpu.rip = op.ip.wrapping_add(op.len as u64);
                        return true;
                    }
                }

                _ => {}
            }
        }
        K_HI8 => {
            #[inline(always)]
            fn get_sp(cpu: &Cpu, sp: u8) -> u64 {
                let idx = (sp & 15) as usize;
                if (sp & 0x80) != 0 {
                    (cpu.gpr[idx] >> 8) & 0xFF
                } else {
                    cpu.gpr[idx] & 0xFF
                }
            }
            #[inline(always)]
            fn set_sp(cpu: &mut Cpu, sp: u8, w: u8, val: u64) {
                if (sp & 0x80) != 0 {
                    let idx = (sp & 15) as usize;
                    cpu.gpr[idx] = (cpu.gpr[idx] & !0xFF00) | ((val & 0xFF) << 8);
                    cpu.last_bits[idx] = 64;
                } else {
                    cpu.rset(sp & 15, w, val);
                }
            }
            let sub = op.x & 0x3F;
            match sub {
                0 => {
                    if op.r1 == 0x20 {
                        set_sp(cpu, op.r0, 8, op.imm);
                    } else if op.r1 == 0x40 {
                        let v = cpu.read_mem(cpu.faddr(op), 8);
                        set_sp(cpu, op.r0, 8, v);
                    } else if op.r0 == 0x40 {
                        let v = get_sp(cpu, op.r1);
                        cpu.write_mem(cpu.faddr(op), 8, v);
                    } else {
                        let v = get_sp(cpu, op.r1);
                        set_sp(cpu, op.r0, op.w, v);
                    }
                }
                1 => {
                    let v = get_sp(cpu, op.r1);
                    set_sp(cpu, op.r0, op.w, v);
                }
                2 => {
                    let v = sext(get_sp(cpu, op.r1), 8);
                    set_sp(cpu, op.r0, op.w, v);
                }
                3..=11 => {
                    let alu_sub = match sub {
                        3 => A_ADD,
                        4 => A_SUB,
                        5 => A_AND,
                        6 => A_OR,
                        7 => A_XOR,
                        8 => A_CMP,
                        9 => A_TEST,
                        10 => A_ADC,
                        _ => A_SBB,
                    };
                    if op.r0 == 0x40 {
                        let addr = cpu.faddr(op);
                        let a = cpu.read_mem(addr, 8);
                        let b = get_sp(cpu, op.r1);
                        if let Some(r) = cpu.alu(alu_sub, a, b, 8) {
                            cpu.write_mem(addr, 8, r);
                        }
                    } else if op.r1 == 0x40 {
                        let b = cpu.read_mem(cpu.faddr(op), 8);
                        let a = get_sp(cpu, op.r0);
                        if let Some(r) = cpu.alu(alu_sub, a, b, 8) {
                            set_sp(cpu, op.r0, 8, r);
                        }
                    } else if op.r1 == 0x20 {
                        let a = get_sp(cpu, op.r0);
                        if let Some(r) = cpu.alu(alu_sub, a, op.imm, 8) {
                            set_sp(cpu, op.r0, 8, r);
                        }
                    } else {
                        let a = get_sp(cpu, op.r0);
                        let b = get_sp(cpu, op.r1);
                        if let Some(r) = cpu.alu(alu_sub, a, b, 8) {
                            set_sp(cpu, op.r0, 8, r);
                        }
                    }
                }
                12 => {
                    let saved_cf = cpu.cf;
                    let a = get_sp(cpu, op.r0);
                    let r = cpu.update_flags_add(a, 1, 8);
                    set_sp(cpu, op.r0, 8, r);
                    cpu.cf = saved_cf;
                }
                13 => {
                    let saved_cf = cpu.cf;
                    let a = get_sp(cpu, op.r0);
                    let r = cpu.update_flags_sub(a, 1, 8);
                    set_sp(cpu, op.r0, 8, r);
                    cpu.cf = saved_cf;
                }
                14 => {
                    let a = get_sp(cpu, op.r0);
                    set_sp(cpu, op.r0, 8, !a);
                }
                15 => {
                    let a = get_sp(cpu, op.r0);
                    let r = cpu.update_flags_sub(0, a, 8);
                    cpu.cf = a != 0;
                    set_sp(cpu, op.r0, 8, r);
                }
                16..=18 => {
                    let cnt = if op.r1 == 0x20 { op.imm as u32 } else { (cpu.gpr[1] as u8 as u32) & 31 };
                    if cnt != 0 {
                        let sh = match sub {
                            16 => S_SHL,
                            17 => S_SHR,
                            _ => S_SAR,
                        };
                        let r = cpu.shift(sh, get_sp(cpu, op.r0), cnt, 8);
                        set_sp(cpu, op.r0, 8, r);
                    }
                }
                19 | 20 => {
                    let cnt = if op.r1 == 0x20 { op.imm as u32 } else { (cpu.gpr[1] as u8 as u32) & 31 };
                    if cnt != 0 {
                        let a = get_sp(cpu, op.r0) as u8;
                        let ror = sub == 20;
                        let res = if ror { a.rotate_right(cnt) } else { a.rotate_left(cnt) } as u64;
                        set_sp(cpu, op.r0, 8, res);
                        let msb = (res >> 7) & 1 != 0;
                        if ror {
                            cpu.cf = msb;
                            cpu.of = msb ^ ((res >> 6) & 1 != 0);
                        } else {
                            cpu.cf = res & 1 != 0;
                            cpu.of = msb ^ cpu.cf;
                        }
                    }
                }
                21 => {
                    let v = cpu.eval_cc_u8(op.imm as u8) as u64;
                    set_sp(cpu, op.r0, 8, v);
                }
                22 => {
                    let a = get_sp(cpu, op.r0);
                    let b = get_sp(cpu, op.r1);
                    set_sp(cpu, op.r0, 8, b);
                    set_sp(cpu, op.r1, 8, a);
                }
                23 => {
                    let a = get_sp(cpu, op.r0);
                    let b = get_sp(cpu, op.r1);
                    let r = cpu.update_flags_add(a, b, 8);
                    set_sp(cpu, op.r0, 8, r);
                    set_sp(cpu, op.r1, 8, a);
                }
                _ => {
                    // Cmpxchg (24)
                    let acc = cpu.gpr[0] & 0xFF;
                    let dst = get_sp(cpu, op.r0);
                    let src = get_sp(cpu, op.r1);
                    let _ = cpu.update_flags_sub(acc, dst, 8);
                    if acc == dst {
                        cpu.zf = true;
                        set_sp(cpu, op.r0, 8, src);
                    } else {
                        cpu.zf = false;
                        cpu.rset(0, 8, dst);
                    }
                }
            }
        }
        K_STRING => exec_string_op(cpu, op),
        K_BIT_EXT => {
            let sub = op.x & 0x0F;
            let mem = (op.x & 0x20) != 0;
            let is_imm = (op.x & 0x10) != 0;
            let w = op.w;
            if sub <= 1 {
                let src = cpu.rget(op.imm as u8, w);
                let mut m = if mem {
                    cpu.read_mem(cpu.faddr(op), w as u32)
                } else {
                    cpu.rget(op.r1, w)
                };
                if w == 32 {
                    m &= 0xFFFF_FFFF;
                }
                let mut res = 0u64;
                if sub == 0 {
                    // Pdep (sparse-bit loop: iterates only over set mask bits)
                    let mut s = src;
                    while m != 0 {
                        let lsb = m & m.wrapping_neg();
                        if (s & 1) != 0 {
                            res |= lsb;
                        }
                        s >>= 1;
                        m &= m - 1;
                    }
                } else {
                    // Pext (sparse-bit loop)
                    let mut k = 0u32;
                    while m != 0 {
                        let tz = m.trailing_zeros();
                        res |= ((src >> tz) & 1) << k;
                        k += 1;
                        m &= m - 1;
                    }
                }
                cpu.rset(op.r0, w, res);
            } else {
                // Rcl (2) / Rcr (3)
                let bits = w as u32;
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                let raw = (if is_imm { op.imm as u32 } else { cpu.gpr[1] as u8 as u32 })
                    & if bits == 64 { 63 } else { 31 };
                if raw != 0 {
                    let cnt = match bits {
                        8 => raw % 9,
                        16 => raw % 17,
                        _ => raw,
                    };
                    let addr = if mem { cpu.faddr(op) } else { 0 };
                    let mut a = (if mem { cpu.read_mem(addr, bits) } else { cpu.rget(op.r0, w) }) & mask;
                    let mut cf = cpu.cf;
                    let top = bits - 1;
                    if sub == 2 {
                        for _ in 0..cnt {
                            let out = (a >> top) & 1 != 0;
                            a = ((a << 1) | cf as u64) & mask;
                            cf = out;
                        }
                    } else {
                        for _ in 0..cnt {
                            let out = a & 1 != 0;
                            a = (a >> 1) | ((cf as u64) << top);
                            cf = out;
                        }
                    }
                    if mem {
                        cpu.write_mem(addr, bits, a);
                    } else {
                        cpu.rset(op.r0, w, a);
                    }
                    cpu.cf = cf;
                    cpu.of = if sub == 2 {
                        ((a >> top) & 1 != 0) ^ cf
                    } else {
                        ((a >> top) ^ (a >> (top - 1))) & 1 != 0
                    };
                }
            }
        }
        K_ATOMIC => {
            let sub = op.x & 0x3F;
            let addr = cpu.faddr(op);
            let w = op.w;
            let bits = w as u32;
            match sub {
                0 => {
                    let b = if op.r0 != 0 { op.imm } else { cpu.rget(op.r1, w) };
                    let (a, _) = cpu.atomic_rmw(addr, bits, |old| old.wrapping_add(b));
                    let _ = cpu.update_flags_add(a, b, bits);
                }
                1 => {
                    let b = if op.r0 != 0 { op.imm } else { cpu.rget(op.r1, w) };
                    let (a, _) = cpu.atomic_rmw(addr, bits, |old| old.wrapping_sub(b));
                    let _ = cpu.update_flags_sub(a, b, bits);
                }
                2 => {
                    let b = if op.r0 != 0 { op.imm } else { cpu.rget(op.r1, w) };
                    let (_, res) = cpu.atomic_rmw(addr, bits, |old| old & b);
                    cpu.update_flags_logic(res, bits);
                }
                3 => {
                    let b = if op.r0 != 0 { op.imm } else { cpu.rget(op.r1, w) };
                    let (_, res) = cpu.atomic_rmw(addr, bits, |old| old | b);
                    cpu.update_flags_logic(res, bits);
                }
                4 => {
                    let b = if op.r0 != 0 { op.imm } else { cpu.rget(op.r1, w) };
                    let (_, res) = cpu.atomic_rmw(addr, bits, |old| old ^ b);
                    cpu.update_flags_logic(res, bits);
                }
                5 => {
                    let saved_cf = cpu.cf;
                    let (a, _) = cpu.atomic_rmw(addr, bits, |old| old.wrapping_add(1));
                    let _ = cpu.update_flags_add(a, 1, bits);
                    cpu.cf = saved_cf;
                }
                6 => {
                    let saved_cf = cpu.cf;
                    let (a, _) = cpu.atomic_rmw(addr, bits, |old| old.wrapping_sub(1));
                    let _ = cpu.update_flags_sub(a, 1, bits);
                    cpu.cf = saved_cf;
                }
                7 => {
                    let _ = cpu.atomic_rmw(addr, bits, |old| !old);
                }
                8 => {
                    let (a, _) = cpu.atomic_rmw(addr, bits, |old| old.wrapping_neg());
                    let _ = cpu.update_flags_sub(0, a, bits);
                }
                9..=12 => {
                    let (eff_addr, b) = if op.r0 == 0 {
                        let bit_off = match bits {
                            16 => cpu.rget(op.r1, 16) as i16 as i64,
                            32 => cpu.rget(op.r1, 32) as i32 as i64,
                            _ => cpu.rget(op.r1, 64) as i64,
                        };
                        let elem_bytes = (bits / 8) as i64;
                        let elem_idx = bit_off.div_euclid(bits as i64);
                        let rem = bit_off.rem_euclid(bits as i64) as u32;
                        ((addr as i64).wrapping_add(elem_idx.wrapping_mul(elem_bytes)) as u64, rem)
                    } else {
                        (addr, (op.imm as u32) & (bits - 1))
                    };
                    let a = match sub {
                        9 => cpu.read_mem(eff_addr, bits),
                        10 => cpu.atomic_rmw(eff_addr, bits, |old| old | (1u64 << b)).0,
                        11 => cpu.atomic_rmw(eff_addr, bits, |old| old & !(1u64 << b)).0,
                        _ => cpu.atomic_rmw(eff_addr, bits, |old| old ^ (1u64 << b)).0,
                    };
                    cpu.cf = ((a >> b) & 1) != 0;
                }
                13 => {
                    let b = cpu.rget(op.r1, w);
                    let (a, _) = cpu.atomic_rmw(addr, bits, |_| b);
                    cpu.rset(op.r1, w, a);
                }
                14 => {
                    let b = cpu.rget(op.r1, w);
                    let (a, _) = cpu.atomic_rmw(addr, bits, |old| old.wrapping_add(b));
                    let _ = cpu.update_flags_add(a, b, bits);
                    cpu.rset(op.r1, w, a);
                }
                15 => {
                    let src = cpu.rget(op.r1, w);
                    let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                    let acc = cpu.gpr[0] & mask;
                    let (ok, dst) = cpu.atomic_cmpxchg(addr, bits, acc, src & mask);
                    let _ = cpu.update_flags_sub(acc, dst, bits);
                    if ok {
                        cpu.zf = true;
                    } else {
                        cpu.zf = false;
                        cpu.rset(0, w, dst);
                    }
                }
                16 => {
                    // Cmpxchg8b
                    let expected = ((cpu.gpr[2] as u32 as u64) << 32) | (cpu.gpr[0] as u32 as u64);
                    let desired = ((cpu.gpr[1] as u32 as u64) << 32) | (cpu.gpr[3] as u32 as u64);
                    let (ok, actual) = cpu.atomic_cmpxchg(addr, 64, expected, desired);
                    cpu.zf = ok;
                    if !ok {
                        cpu.rset(0, 32, actual as u32 as u64);
                        cpu.rset(2, 32, (actual >> 32) as u32 as u64);
                    }
                }
                _ => {
                    // Cmpxchg16b (17)
                    let expected = ((cpu.gpr[2] as u128) << 64) | (cpu.gpr[0] as u128);
                    let desired = ((cpu.gpr[1] as u128) << 64) | (cpu.gpr[3] as u128);
                    let fast = if addr & 15 == 0 && crate::emu::regfile_ext::is_valid_guest_range(addr, 16) {
                        crate::emu::cas128_full(addr, expected, desired)
                    } else {
                        None
                    };
                    let actual = if let Some(a) = fast {
                        a
                    } else {
                        let _g = crate::emu::cpu::lock_atomic_shard(addr);
                        let actual = cpu.read_u128(addr);
                        if actual == expected {
                            cpu.write_u128(addr, desired);
                        }
                        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
                        actual
                    };
                    if actual == expected {
                        cpu.zf = true;
                    } else {
                        cpu.zf = false;
                        cpu.rset(0, 64, actual as u64);
                        cpu.rset(2, 64, (actual >> 64) as u64);
                    }
                }
            }
        }
        K_PCMPSTR => {
            let flags = op.x & 0x3F;
            let is_explicit = (flags & 1) != 0;
            let is_mask = (flags & 2) != 0;
            let is_vex = (flags & 4) != 0;
            let is_long = (flags & 8) != 0;
            let is_mem = (flags & 0x20) != 0;
            let a = cpu.xmm[op.r0 as usize & 15];
            let b = if is_mem {
                cpu.read_u128(cpu.faddr(op))
            } else {
                cpu.xmm[op.r1 as usize & 15]
            };
            sk::pcmpstr(cpu, a, b, op.imm as u8, is_explicit, is_mask, is_long, is_vex);
        }
        K_VSHA_RR | K_VSHA_RM => {
            let d = op.r0 as usize & 15;
            let src1 = cpu.xmm[d];
            let src2 = if op.kind == K_VSHA_RM {
                cpu.read_u128(cpu.faddr(op))
            } else {
                cpu.xmm[op.r1 as usize & 15]
            };
            cpu.xmm[d] = match op.x & 0x3F {
                0 => sk::sha1rnds4(src1, src2, op.imm as u8),
                1 => sk::sha1nexte(src1, src2),
                2 => sk::sha1msg1(src1, src2),
                3 => sk::sha1msg2(src1, src2),
                4 => sk::sha256rnds2(src1, src2, cpu.xmm[0]),
                5 => sk::sha256msg1(src1, src2),
                _ => sk::sha256msg2(src1, src2),
            };
        }
        K_VGFNI_RR | K_VGFNI_RM => {
            let d = op.r0 as usize & 15;
            let s1 = ((op.imm >> 8) as usize) & 15;
            let sub = op.x & 0x3F;
            let mode = op.w & 3;
            let is_256 = mode == VM_V256;
            let (a_lo, a_hi) = (cpu.xmm[s1], if is_256 { cpu.ymm_hi[s1] } else { 0 });
            let (b_lo, b_hi) = if op.kind == K_VGFNI_RM {
                let a = cpu.faddr(op);
                (cpu.read_u128(a), if is_256 { cpu.read_u128(a.wrapping_add(16)) } else { 0 })
            } else {
                let r2 = op.r1 as usize & 15;
                (cpu.xmm[r2], if is_256 { cpu.ymm_hi[r2] } else { 0 })
            };
            let imm8 = op.imm as u8;
            let run = |a, b| match sub {
                0 => sk::gf2p8mulb(a, b),
                1 => sk::gf2p8affineqb(a, b, imm8, false),
                _ => sk::gf2p8affineqb(a, b, imm8, true),
            };
            cpu.xmm[d] = run(a_lo, b_lo);
            match mode {
                VM_V128 => cpu.clear_ymm_hi_reg(d),
                VM_V256 => cpu.set_ymm_hi(d, run(a_hi, b_hi)),
                _ => {}
            }
        }
        K_VVNNI_RR | K_VVNNI_RM => {
            let d = op.r0 as usize & 15;
            let s1 = ((op.imm >> 8) as usize) & 15;
            let sub = op.x & 0x3F;
            let is_256 = (op.w & 3) == VM_V256;
            let (dst_lo, dst_hi) = (cpu.xmm[d], if is_256 { cpu.ymm_hi[d] } else { 0 });
            let (a_lo, a_hi) = (cpu.xmm[s1], if is_256 { cpu.ymm_hi[s1] } else { 0 });
            let (b_lo, b_hi) = if op.kind == K_VVNNI_RM {
                let a = cpu.faddr(op);
                (cpu.read_u128(a), if is_256 { cpu.read_u128(a.wrapping_add(16)) } else { 0 })
            } else {
                let r2 = op.r1 as usize & 15;
                (cpu.xmm[r2], if is_256 { cpu.ymm_hi[r2] } else { 0 })
            };
            let run = |dst, a, b| match sub {
                0 => sk::vpdpbusd(dst, a, b, false),
                1 => sk::vpdpbusd(dst, a, b, true),
                2 => sk::vpdpwssd(dst, a, b, false),
                _ => sk::vpdpwssd(dst, a, b, true),
            };
            cpu.xmm[d] = run(dst_lo, a_lo, b_lo);
            if is_256 {
                cpu.set_ymm_hi(d, run(dst_hi, a_hi, b_hi));
            } else {
                cpu.clear_ymm_hi_reg(d);
            }
        }
        _ => super::mmx_gather::exec_mmx_gather(cpu, op),
    }
    false
}
