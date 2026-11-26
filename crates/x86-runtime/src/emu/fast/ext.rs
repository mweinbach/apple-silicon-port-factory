//! Extended lowering and execution for `K_VBROADCAST..=K_LOOP` (`101..=114`) routed through
//! [`crate::emu::simd_kernels`].

use super::decode::{gpr_of, is_w, mem_bits, put_mem, vmem_code, vmode, xreg};
use super::more::{exec_ext_all_remaining, lower_all_remaining};
use super::*;
use crate::emu::simd_kernels as sk;
use crate::emu::{Cpu, OpKind};
use iced_x86::{Instruction, Mnemonic};

/// `(v)pbroadcastb/w/d/q`, `(v)broadcastss/sd`, `vbroadcastf128/i128`.
pub(in crate::emu) const K_VBROADCAST: u8 = 101;
/// `(v)pmovsxbw/bd/bq/wd/wq/dq` and `(v)pmovzxbw/bd/bq/wd/wq/dq` (128-bit and 256-bit).
pub(in crate::emu) const K_PMOVX: u8 = 102;
/// `(v)cvtps2pd`, `(v)cvtdq2pd`, `(v)cvtpd2ps`, `(v)cvttpd2dq`, `(v)cvtpd2dq`.
pub(in crate::emu) const K_VCVT_PD: u8 = 103;
/// All 72 FMA mnemonics (`vfmadd/fmsub/fnmadd/fnmsub/fmaddsub/fmsubadd` `132/213/231` `ss/sd/ps/pd`).
pub(in crate::emu) const K_VFMA_RR: u8 = 104;
pub(in crate::emu) const K_VFMA_RM: u8 = 105;
/// Variable blends: `(v)blendvps`, `(v)blendvpd`, `(v)pblendvb`.
pub(in crate::emu) const K_VBLENDV_RR: u8 = 106;
pub(in crate::emu) const K_VBLENDV_RM: u8 = 107;
/// Vector test: `(v)ptest`, `vtestps`, `vtestpd`.
pub(in crate::emu) const K_VTEST_RR: u8 = 108;
pub(in crate::emu) const K_VTEST_RM: u8 = 109;
/// BMI1/BMI2/ADX/CRC32 integer ops (`andn`, `shlx`, `shrx`, `sarx`, `rorx`, `blsr`, `blsmsk`,
/// `blsi`, `bzhi`, `bextr`, `mulx`, `adcx`, `adox`, `crc32`).
pub(in crate::emu) const K_BMI_RR: u8 = 110;
pub(in crate::emu) const K_BMI_RM: u8 = 111;
/// 256-bit lane ops: `vpermq`/`vpermpd`, `vpermd`/`vpermps`, `vperm2f128`/`vperm2i128`,
/// `vinsertf128`/`vinserti128`, `vextractf128`/`vextracti128`, `vzeroall`.
pub(in crate::emu) const K_YMM_LANE: u8 = 112;
/// Universal pre-decoded `Instruction` fast op (atomic `lock` RMW, `cmpxchg8b`/`16b`, string ops,
/// `aes*`, `vcvtph2ps`/`vcvtps2ph`, `vmaskmov*`, `pcmpistri/m`, `pdep`/`pext`, `rcl`/`rcr`,
/// `lahf`/`sahf`, `pushfq`/`popfq`, `cpuid`/`rdtsc`/`xgetbv`, MXCSR/x87 CW, high-byte GPR forms).
pub(in crate::emu) const K_INSN: u8 = 113;
/// `jrcxz` (`sub=0`), `jecxz` (`sub=1`), `loop` (`sub=2`), `loope` (`sub=3`), `loopne` (`sub=4`).
pub(in crate::emu) const K_LOOP: u8 = 114;

fn fma_desc(mn: Mnemonic) -> Option<(u8, u8, u8)> {
    use iced_x86::Mnemonic as M;
    // Returns (ty: 0=ss, 1=sd, 2=ps, 3=pd; order: 0=132, 1=213, 2=231; op: 0=fmadd, 1=fmsub, 2=fnmadd, 3=fnmsub, 4=fmaddsub, 5=fmsubadd)
    let r = match mn {
        M::Vfmadd132ss => (0, 0, 0),
        M::Vfmadd213ss => (0, 1, 0),
        M::Vfmadd231ss => (0, 2, 0),
        M::Vfmsub132ss => (0, 0, 1),
        M::Vfmsub213ss => (0, 1, 1),
        M::Vfmsub231ss => (0, 2, 1),
        M::Vfnmadd132ss => (0, 0, 2),
        M::Vfnmadd213ss => (0, 1, 2),
        M::Vfnmadd231ss => (0, 2, 2),
        M::Vfnmsub132ss => (0, 0, 3),
        M::Vfnmsub213ss => (0, 1, 3),
        M::Vfnmsub231ss => (0, 2, 3),

        M::Vfmadd132sd => (1, 0, 0),
        M::Vfmadd213sd => (1, 1, 0),
        M::Vfmadd231sd => (1, 2, 0),
        M::Vfmsub132sd => (1, 0, 1),
        M::Vfmsub213sd => (1, 1, 1),
        M::Vfmsub231sd => (1, 2, 1),
        M::Vfnmadd132sd => (1, 0, 2),
        M::Vfnmadd213sd => (1, 1, 2),
        M::Vfnmadd231sd => (1, 2, 2),
        M::Vfnmsub132sd => (1, 0, 3),
        M::Vfnmsub213sd => (1, 1, 3),
        M::Vfnmsub231sd => (1, 2, 3),

        M::Vfmadd132ps => (2, 0, 0),
        M::Vfmadd213ps => (2, 1, 0),
        M::Vfmadd231ps => (2, 2, 0),
        M::Vfmsub132ps => (2, 0, 1),
        M::Vfmsub213ps => (2, 1, 1),
        M::Vfmsub231ps => (2, 2, 1),
        M::Vfnmadd132ps => (2, 0, 2),
        M::Vfnmadd213ps => (2, 1, 2),
        M::Vfnmadd231ps => (2, 2, 2),
        M::Vfnmsub132ps => (2, 0, 3),
        M::Vfnmsub213ps => (2, 1, 3),
        M::Vfnmsub231ps => (2, 2, 3),
        M::Vfmaddsub132ps => (2, 0, 4),
        M::Vfmaddsub213ps => (2, 1, 4),
        M::Vfmaddsub231ps => (2, 2, 4),
        M::Vfmsubadd132ps => (2, 0, 5),
        M::Vfmsubadd213ps => (2, 1, 5),
        M::Vfmsubadd231ps => (2, 2, 5),

        M::Vfmadd132pd => (3, 0, 0),
        M::Vfmadd213pd => (3, 1, 0),
        M::Vfmadd231pd => (3, 2, 0),
        M::Vfmsub132pd => (3, 0, 1),
        M::Vfmsub213pd => (3, 1, 1),
        M::Vfmsub231pd => (3, 2, 1),
        M::Vfnmadd132pd => (3, 0, 2),
        M::Vfnmadd213pd => (3, 1, 2),
        M::Vfnmadd231pd => (3, 2, 2),
        M::Vfnmsub132pd => (3, 0, 3),
        M::Vfnmsub213pd => (3, 1, 3),
        M::Vfnmsub231pd => (3, 2, 3),
        M::Vfmaddsub132pd => (3, 0, 4),
        M::Vfmaddsub213pd => (3, 1, 4),
        M::Vfmaddsub231pd => (3, 2, 4),
        M::Vfmsubadd132pd => (3, 0, 5),
        M::Vfmsubadd213pd => (3, 1, 5),
        M::Vfmsubadd231pd => (3, 2, 5),
        _ => return None,
    };
    Some(r)
}

pub(super) fn lower_more(ins: &Instruction, mut f: FastOp) -> Option<FastOp> {
    use iced_x86::Mnemonic as M;
    let mn = ins.mnemonic();
    let n = ins.op_count();
    let k0 = if n > 0 { ins.op_kind(0) } else { OpKind::Register };
    let k1 = if n > 1 { ins.op_kind(1) } else { OpKind::Register };
    if let Some((ty, ord, op)) = fma_desc(mn) {
        let mode = vmode(ins)?;
        if n != 3 || k0 != OpKind::Register || k1 != OpKind::Register {
            return None;
        }
        if mode == VM_V256 && ty < 2 {
            return None;
        }
        let d = xreg(ins.op_register(0))?;
        let s2 = xreg(ins.op_register(1))?;
        let fma_code = ty | (ord << 2) | (op << 4);
        f.r0 = d;
        f.imm = (s2 as u64) | ((fma_code as u64) << 8);
        match ins.op_kind(2) {
            OpKind::Register => {
                f.r1 = xreg(ins.op_register(2))?;
                f.kind = K_VFMA_RR;
                f.w = mode;
            }
            OpKind::Memory => {
                let bytes = match ty {
                    0 => 4,
                    1 => 8,
                    _ if mode == VM_V256 => 32,
                    _ => 16,
                };
                let code = vmem_code(bytes)?;
                f.kind = K_VFMA_RM;
                f.w = mode | (code << 4);
                put_mem(ins, &mut f)?;
            }
            _ => return None,
        }
        return Some(f);
    }

    match mn {
        M::Vpbroadcastb | M::Vpbroadcastw | M::Vpbroadcastd | M::Vpbroadcastq | M::Vbroadcastss
        | M::Vbroadcastsd | M::Vbroadcastf128 | M::Vbroadcasti128 => {
            let mode = vmode(ins)?;
            if n != 2 || k0 != OpKind::Register {
                return None;
            }
            let d = xreg(ins.op_register(0))?;
            let lg: u8 = match mn {
                M::Vpbroadcastb => 0,
                M::Vpbroadcastw => 1,
                M::Vpbroadcastd | M::Vbroadcastss => 2,
                M::Vpbroadcastq | M::Vbroadcastsd => 3,
                _ => 4,
            };
            match k1 {
                OpKind::Register => {
                    f.r1 = xreg(ins.op_register(1))?;
                    f.x = 0;
                }
                OpKind::Memory => {
                    put_mem(ins, &mut f)?;
                    f.x |= 0x20;
                }
                _ => return None,
            }
            f.kind = K_VBROADCAST;
            f.r0 = d;
            f.w = mode | (lg << 4);
        }
        M::Pmovsxbw | M::Vpmovsxbw | M::Pmovzxbw | M::Vpmovzxbw | M::Pmovsxbd | M::Vpmovsxbd
        | M::Pmovzxbd | M::Vpmovzxbd | M::Pmovsxbq | M::Vpmovsxbq | M::Pmovzxbq | M::Vpmovzxbq
        | M::Pmovsxwd | M::Vpmovsxwd | M::Pmovzxwd | M::Vpmovzxwd | M::Pmovsxwq | M::Vpmovsxwq
        | M::Pmovzxwq | M::Vpmovzxwq | M::Pmovsxdq | M::Vpmovsxdq | M::Pmovzxdq | M::Vpmovzxdq => {
            let mode = vmode(ins)?;
            if n != 2 || k0 != OpKind::Register {
                return None;
            }
            let d = xreg(ins.op_register(0))?;
            let sub: u8 = match mn {
                M::Pmovsxbw | M::Vpmovsxbw => 0,
                M::Pmovzxbw | M::Vpmovzxbw => 1,
                M::Pmovsxbd | M::Vpmovsxbd => 2,
                M::Pmovzxbd | M::Vpmovzxbd => 3,
                M::Pmovsxbq | M::Vpmovsxbq => 4,
                M::Pmovzxbq | M::Vpmovzxbq => 5,
                M::Pmovsxwd | M::Vpmovsxwd => 6,
                M::Pmovzxwd | M::Vpmovzxwd => 7,
                M::Pmovsxwq | M::Vpmovsxwq => 8,
                M::Pmovzxwq | M::Vpmovzxwq => 9,
                M::Pmovsxdq | M::Vpmovsxdq => 10,
                _ => 11,
            };
            match k1 {
                OpKind::Register => {
                    f.r1 = xreg(ins.op_register(1))?;
                    f.x = 0;
                    f.imm = 128;
                }
                OpKind::Memory => {
                    put_mem(ins, &mut f)?;
                    f.x |= 0x20;
                    f.imm = (ins.memory_size().size() as u64 * 8).max(16);
                }
                _ => return None,
            }
            f.kind = K_PMOVX;
            f.r0 = d;
            f.w = mode | (sub << 2);
        }
        M::Cvtps2pd | M::Vcvtps2pd | M::Cvtdq2pd | M::Vcvtdq2pd | M::Cvtpd2ps | M::Vcvtpd2ps
        | M::Cvttpd2dq | M::Vcvttpd2dq | M::Cvtpd2dq | M::Vcvtpd2dq => {
            let mode = vmode(ins)?;
            if n != 2 || k0 != OpKind::Register {
                return None;
            }
            let d = xreg(ins.op_register(0))?;
            let sub: u8 = match mn {
                M::Cvtps2pd | M::Vcvtps2pd => 0,
                M::Cvtdq2pd | M::Vcvtdq2pd => 1,
                M::Cvtpd2ps | M::Vcvtpd2ps => 2,
                M::Cvttpd2dq | M::Vcvttpd2dq => 3,
                _ => 4,
            };
            let src_256 = match k1 {
                OpKind::Register => Cpu::is_ymm(ins.op_register(1)),
                OpKind::Memory => ins.memory_size().size() == 32,
                _ => false,
            };
            match k1 {
                OpKind::Register => {
                    f.r1 = xreg(ins.op_register(1))?;
                    f.x = sub;
                }
                OpKind::Memory => {
                    f.x = sub | 0x20;
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
            f.kind = K_VCVT_PD;
            f.r0 = d;
            f.w = mode | ((src_256 as u8) << 2);
        }
        M::Blendvps | M::Vblendvps | M::Blendvpd | M::Vblendvpd | M::Pblendvb | M::Vpblendvb => {
            let mode = vmode(ins)?;
            if k0 != OpKind::Register {
                return None;
            }
            let d = xreg(ins.op_register(0))?;
            let (s1, s2_op, m_reg) = match n {
                2 => (d, 1, 0u8),
                3 => (d, 1, xreg(ins.op_register(2))?),
                4 => (xreg(ins.op_register(1))?, 2, xreg(ins.op_register(3))?),
                _ => return None,
            };
            let lg: u8 = match mn {
                M::Pblendvb | M::Vpblendvb => 0,
                M::Blendvps | M::Vblendvps => 2,
                _ => 3,
            };
            f.r0 = d;
            f.imm = (m_reg as u64) | ((s1 as u64) << 8);
            f.x = lg;
            match ins.op_kind(s2_op) {
                OpKind::Register => {
                    f.r1 = xreg(ins.op_register(s2_op))?;
                    f.kind = K_VBLENDV_RR;
                    f.w = mode;
                }
                OpKind::Memory => {
                    let bytes = if mode == VM_V256 { 32 } else { 16 };
                    let code = vmem_code(bytes)?;
                    f.kind = K_VBLENDV_RM;
                    f.w = mode | (code << 4);
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
        }
        M::Ptest | M::Vptest | M::Vtestps | M::Vtestpd => {
            let mode = vmode(ins)?;
            if n != 2 || k0 != OpKind::Register {
                return None;
            }
            let a = xreg(ins.op_register(0))?;
            let is_256 = Cpu::is_ymm(ins.op_register(0));
            let w_mode = if is_256 { VM_V256 } else { mode };
            let sub: u8 = match mn {
                M::Ptest | M::Vptest => 0,
                M::Vtestps => 1,
                _ => 2,
            };
            f.r0 = a;
            f.imm = (a as u64) << 8;
            f.x = sub;
            match k1 {
                OpKind::Register => {
                    f.r1 = xreg(ins.op_register(1))?;
                    f.kind = K_VTEST_RR;
                    f.w = w_mode;
                }
                OpKind::Memory => {
                    let code = vmem_code(if is_256 { 32 } else { 16 })?;
                    f.kind = K_VTEST_RM;
                    f.w = w_mode | (code << 4);
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
        }
        M::Andn => {
            if n != 3 || k0 != OpKind::Register || k1 != OpKind::Register {
                return None;
            }
            let (d, w) = gpr_of(ins.op_register(0))?;
            let (s1, w1) = gpr_of(ins.op_register(1))?;
            if (w != 32 && w != 64) || w1 != w {
                return None;
            }
            f.r0 = d;
            f.w = w;
            f.x = 0;
            f.imm = s1 as u64;
            match ins.op_kind(2) {
                OpKind::Register => {
                    let (s2, w2) = gpr_of(ins.op_register(2))?;
                    if w2 != w {
                        return None;
                    }
                    f.r1 = s2;
                    f.kind = K_BMI_RR;
                }
                OpKind::Memory => {
                    if mem_bits(ins) != w as u32 {
                        return None;
                    }
                    put_mem(ins, &mut f)?;
                    f.kind = K_BMI_RM;
                }
                _ => return None,
            }
        }
        M::Shlx | M::Shrx | M::Sarx | M::Bzhi | M::Bextr => {
            if n != 3 || k0 != OpKind::Register || ins.op_kind(2) != OpKind::Register {
                return None;
            }
            let (d, w) = gpr_of(ins.op_register(0))?;
            if w != 32 && w != 64 {
                return None;
            }
            let sub: u8 = match mn {
                M::Shlx => 1,
                M::Shrx => 2,
                M::Sarx => 3,
                M::Bzhi => 8,
                _ => 9, // Bextr
            };
            f.r0 = d;
            f.w = w;
            f.x = sub;
            let (ctl, wc) = gpr_of(ins.op_register(2))?;
            if wc != w {
                return None;
            }
            f.r1 = ctl;
            match k1 {
                OpKind::Register => {
                    let (src, ws) = gpr_of(ins.op_register(1))?;
                    if ws != w {
                        return None;
                    }
                    f.imm = src as u64;
                    f.kind = K_BMI_RR;
                }
                OpKind::Memory => {
                    if mem_bits(ins) != w as u32 {
                        return None;
                    }
                    put_mem(ins, &mut f)?;
                    f.kind = K_BMI_RM;
                }
                _ => return None,
            }
        }
        M::Rorx => {
            if n != 3 || k0 != OpKind::Register || ins.op_kind(2) != OpKind::Immediate8 {
                return None;
            }
            let (d, w) = gpr_of(ins.op_register(0))?;
            if w != 32 && w != 64 {
                return None;
            }
            let mask = if w == 64 { 63 } else { 31 };
            f.r0 = d;
            f.w = w;
            f.x = 4;
            f.imm = (ins.immediate8() as u64) & mask;
            match k1 {
                OpKind::Register => {
                    let (s, ws) = gpr_of(ins.op_register(1))?;
                    if ws != w {
                        return None;
                    }
                    f.r1 = s;
                    f.kind = K_BMI_RR;
                }
                OpKind::Memory => {
                    if mem_bits(ins) != w as u32 {
                        return None;
                    }
                    put_mem(ins, &mut f)?;
                    f.kind = K_BMI_RM;
                }
                _ => return None,
            }
        }
        M::Blsr | M::Blsmsk | M::Blsi => {
            if n != 2 || k0 != OpKind::Register {
                return None;
            }
            let (d, w) = gpr_of(ins.op_register(0))?;
            if w != 32 && w != 64 {
                return None;
            }
            let sub: u8 = match mn {
                M::Blsr => 5,
                M::Blsmsk => 6,
                _ => 7,
            };
            f.r0 = d;
            f.w = w;
            f.x = sub;
            match k1 {
                OpKind::Register => {
                    let (s, ws) = gpr_of(ins.op_register(1))?;
                    if ws != w {
                        return None;
                    }
                    f.r1 = s;
                    f.kind = K_BMI_RR;
                }
                OpKind::Memory => {
                    if mem_bits(ins) != w as u32 {
                        return None;
                    }
                    put_mem(ins, &mut f)?;
                    f.kind = K_BMI_RM;
                }
                _ => return None,
            }
        }
        M::Mulx => {
            if n != 3 || k0 != OpKind::Register || k1 != OpKind::Register {
                return None;
            }
            let (d_hi, w) = gpr_of(ins.op_register(0))?;
            let (d_lo, w1) = gpr_of(ins.op_register(1))?;
            if (w != 32 && w != 64) || w1 != w {
                return None;
            }
            f.r0 = d_hi;
            f.r1 = d_lo;
            f.w = w;
            f.x = 10;
            match ins.op_kind(2) {
                OpKind::Register => {
                    let (s, ws) = gpr_of(ins.op_register(2))?;
                    if ws != w {
                        return None;
                    }
                    f.imm = s as u64;
                    f.kind = K_BMI_RR;
                }
                OpKind::Memory => {
                    if mem_bits(ins) != w as u32 {
                        return None;
                    }
                    put_mem(ins, &mut f)?;
                    f.kind = K_BMI_RM;
                }
                _ => return None,
            }
        }
        M::Adcx | M::Adox => {
            if n != 2 || k0 != OpKind::Register {
                return None;
            }
            let (d, w) = gpr_of(ins.op_register(0))?;
            if w != 32 && w != 64 {
                return None;
            }
            f.r0 = d;
            f.w = w;
            f.x = if mn == M::Adcx { 11 } else { 12 };
            match k1 {
                OpKind::Register => {
                    let (s, ws) = gpr_of(ins.op_register(1))?;
                    if ws != w {
                        return None;
                    }
                    f.r1 = s;
                    f.kind = K_BMI_RR;
                }
                OpKind::Memory => {
                    if mem_bits(ins) != w as u32 {
                        return None;
                    }
                    put_mem(ins, &mut f)?;
                    f.kind = K_BMI_RM;
                }
                _ => return None,
            }
        }
        M::Crc32 => {
            if n != 2 || k0 != OpKind::Register {
                return None;
            }
            let (d, dw) = gpr_of(ins.op_register(0))?;
            if dw != 32 && dw != 64 {
                return None;
            }
            f.r0 = d;
            f.x = 13;
            f.imm = dw as u64;
            match k1 {
                OpKind::Register => {
                    let (idx, bits, hi) = Cpu::reg_idx(ins.op_register(1))?;
                    f.r1 = (idx as u8) | ((hi as u8) << 7);
                    f.w = bits as u8;
                    f.kind = K_BMI_RR;
                }
                OpKind::Memory => {
                    let sw = mem_bits(ins);
                    if !is_w(sw) {
                        return None;
                    }
                    f.w = sw as u8;
                    put_mem(ins, &mut f)?;
                    f.kind = K_BMI_RM;
                }
                _ => return None,
            }
        }
        M::Vpermq | M::Vpermpd => {
            if n != 3 || k0 != OpKind::Register || ins.op_kind(2) != OpKind::Immediate8 || !Cpu::is_ymm(ins.op_register(0)) {
                return None;
            }
            f.r0 = xreg(ins.op_register(0))?;
            f.imm = ins.immediate8() as u64;
            f.x = 0;
            match k1 {
                OpKind::Register => {
                    f.r1 = xreg(ins.op_register(1))?;
                    f.w = 0;
                }
                OpKind::Memory => {
                    f.w = 1;
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
            f.kind = K_YMM_LANE;
        }
        M::Vpermd | M::Vpermps => {
            if n != 3 || k0 != OpKind::Register || k1 != OpKind::Register || !Cpu::is_ymm(ins.op_register(0)) {
                return None;
            }
            f.r0 = xreg(ins.op_register(0))?;
            f.imm = xreg(ins.op_register(1))? as u64;
            f.x = 1;
            match ins.op_kind(2) {
                OpKind::Register => {
                    f.r1 = xreg(ins.op_register(2))?;
                    f.w = 0;
                }
                OpKind::Memory => {
                    f.w = 1;
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
            f.kind = K_YMM_LANE;
        }
        M::Vperm2f128 | M::Vperm2i128 => {
            if n != 4 || k0 != OpKind::Register || k1 != OpKind::Register || ins.op_kind(3) != OpKind::Immediate8 {
                return None;
            }
            f.r0 = xreg(ins.op_register(0))?;
            let s1 = xreg(ins.op_register(1))?;
            f.imm = (ins.immediate8() as u64) | ((s1 as u64) << 8);
            f.x = 2;
            match ins.op_kind(2) {
                OpKind::Register => {
                    f.r1 = xreg(ins.op_register(2))?;
                    f.w = 0;
                }
                OpKind::Memory => {
                    f.w = 1;
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
            f.kind = K_YMM_LANE;
        }
        M::Vinsertf128 | M::Vinserti128 => {
            if n != 4 || k0 != OpKind::Register || k1 != OpKind::Register || ins.op_kind(3) != OpKind::Immediate8 {
                return None;
            }
            f.r0 = xreg(ins.op_register(0))?;
            let s1 = xreg(ins.op_register(1))?;
            f.imm = ((ins.immediate8() & 1) as u64) | ((s1 as u64) << 8);
            f.x = 3;
            match ins.op_kind(2) {
                OpKind::Register => {
                    f.r1 = xreg(ins.op_register(2))?;
                    f.w = 0;
                }
                OpKind::Memory => {
                    f.w = 1;
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
            f.kind = K_YMM_LANE;
        }
        M::Vextractf128 | M::Vextracti128 => {
            if n != 3 || k1 != OpKind::Register || ins.op_kind(2) != OpKind::Immediate8 {
                return None;
            }
            f.r1 = xreg(ins.op_register(1))?;
            f.imm = (ins.immediate8() & 1) as u64;
            f.x = 4;
            match k0 {
                OpKind::Register => {
                    f.r0 = xreg(ins.op_register(0))?;
                    f.w = 0;
                }
                OpKind::Memory => {
                    f.w = 1;
                    put_mem(ins, &mut f)?;
                }
                _ => return None,
            }
            f.kind = K_YMM_LANE;
        }
        M::Vzeroall => {
            f.kind = K_YMM_LANE;
            f.x = 5;
        }
        M::Jrcxz | M::Jecxz | M::Loop | M::Loope | M::Loopne => {
            f.kind = K_LOOP;
            f.x = match mn {
                M::Jrcxz => 0,
                M::Jecxz => 1,
                M::Loop => 2,
                M::Loope => 3,
                _ => 4,
            };
            f.imm = ins.near_branch_target();
        }
        M::Int3 | M::Ud2 => return None,
        _ => return lower_all_remaining(ins, f),
    }
    Some(f)
}

pub(super) unsafe fn exec_ext_more(cpu: &mut Cpu, op: &FastOp) -> bool {
    let w = op.w;
    match op.kind {
        K_VBROADCAST => {
            let d = op.r0 as usize & 15;
            let mode = op.w & 3;
            let lg = (op.w >> 4) as u32;
            if lg == 4 {
                let val = if op.x & 0x20 != 0 {
                    cpu.read_u128(cpu.faddr(op))
                } else {
                    cpu.xmm[op.r1 as usize & 15]
                };
                cpu.xmm[d] = val;
                cpu.set_ymm_hi(d, val);
            } else {
                let ebits = 8u32 << lg;
                let raw = if op.x & 0x20 != 0 {
                    cpu.read_mem(cpu.faddr(op), ebits)
                } else {
                    cpu.xmm[op.r1 as usize & 15] as u64
                };
                let out = sk::pbroadcast(raw as u128, ebits);
                cpu.xmm[d] = out;
                if mode == VM_V256 {
                    cpu.set_ymm_hi(d, out);
                } else {
                    cpu.clear_ymm_hi_reg(d);
                }
            }
        }
        K_PMOVX => {
            let d = op.r0 as usize & 15;
            let mode = op.w & 3;
            let sub = op.w >> 2;
            let raw = if op.x & 0x20 != 0 {
                let a = cpu.faddr(op);
                if op.imm == 128 {
                    cpu.read_u128(a)
                } else {
                    cpu.read_mem(a, op.imm as u32) as u128
                }
            } else {
                cpu.xmm[op.r1 as usize & 15]
            };
            let (out_lo, out_hi) = sk::pmovsx_zx(raw, sub, mode == VM_V256);
            cpu.xmm[d] = out_lo;
            if mode == VM_V256 {
                cpu.set_ymm_hi(d, out_hi);
            } else if mode == VM_V128 {
                cpu.clear_ymm_hi_reg(d);
            }
        }
        K_VCVT_PD => {
            let d = op.r0 as usize & 15;
            let mode = op.w & 3;
            let src_256 = (op.w & 4) != 0;
            let sub = op.x & 0x1F;
            let mem = (op.x & 0x20) != 0;
            if sub <= 1 {
                let raw = if mem {
                    let a = cpu.faddr(op);
                    if mode == VM_V256 {
                        cpu.read_u128(a)
                    } else {
                        cpu.read_mem(a, 64) as u128
                    }
                } else {
                    cpu.xmm[op.r1 as usize & 15]
                };
                let (lo, hi) = if sub == 1 {
                    sk::cvtdq2pd(raw, mode == VM_V256)
                } else {
                    sk::cvtps2pd(raw, mode == VM_V256)
                };
                cpu.xmm[d] = lo;
                if mode == VM_V256 {
                    cpu.set_ymm_hi(d, hi);
                } else if mode == VM_V128 {
                    cpu.clear_ymm_hi_reg(d);
                }
            } else {
                let (lo, hi) = if mem {
                    let a = cpu.faddr(op);
                    let l = cpu.read_u128(a);
                    let h = if src_256 { cpu.read_u128(a.wrapping_add(16)) } else { 0 };
                    (l, h)
                } else {
                    let s = op.r1 as usize & 15;
                    (cpu.xmm[s], cpu.ymm_hi[s])
                };
                cpu.xmm[d] = if sub == 2 {
                    sk::cvtpd2ps(lo, hi, src_256)
                } else {
                    sk::cvtpd2dq(lo, hi, src_256, sub == 3)
                };
                if mode != VM_LEG {
                    cpu.clear_ymm_hi_reg(d);
                }
            }
        }
        K_VFMA_RR | K_VFMA_RM => {
            let d = op.r0 as usize & 15;
            let s2 = (op.imm & 0xFF) as usize & 15;
            let fma_code = ((op.imm >> 8) & 0xFF) as u8;
            let ty = fma_code & 3;
            let ord = (fma_code >> 2) & 3;
            let fop = (fma_code >> 4) & 7;
            let mem = op.kind == K_VFMA_RM;
            let is_256 = (op.w & 3) == VM_V256;
            let v1_lo = cpu.xmm[d];
            let v2_lo = cpu.xmm[s2];
            let (v3_lo, v3_hi) = if mem {
                cpu.vload(op)
            } else {
                let s3 = op.r1 as usize & 15;
                (cpu.xmm[s3], if is_256 { cpu.ymm_hi[s3] } else { 0 })
            };
            let neg_mul = fop == 2 || fop == 3;
            if ty == 0 {
                let x1 = f32::from_bits(v1_lo as u32);
                let x2 = f32::from_bits(v2_lo as u32);
                let x3 = f32::from_bits(v3_lo as u32);
                let (m1, m2, acc) = match ord {
                    0 => (x1, x3, x2),
                    1 => (x2, x1, x3),
                    _ => (x2, x3, x1),
                };
                let neg_acc = fop == 1 || fop == 3;
                let res = (if neg_mul { -m1 } else { m1 }).mul_add(m2, if neg_acc { -acc } else { acc });
                cpu.xmm[d] = (v1_lo & !0xFFFF_FFFF) | (res.to_bits() as u128);
                cpu.clear_ymm_hi_reg(d);
            } else if ty == 1 {
                let x1 = f64::from_bits(v1_lo as u64);
                let x2 = f64::from_bits(v2_lo as u64);
                let x3 = f64::from_bits(v3_lo as u64);
                let (m1, m2, acc) = match ord {
                    0 => (x1, x3, x2),
                    1 => (x2, x1, x3),
                    _ => (x2, x3, x1),
                };
                let neg_acc = fop == 1 || fop == 3;
                let res = (if neg_mul { -m1 } else { m1 }).mul_add(m2, if neg_acc { -acc } else { acc });
                cpu.xmm[d] = (v1_lo & !0xFFFF_FFFF_FFFF_FFFF) | (res.to_bits() as u128);
                cpu.clear_ymm_hi_reg(d);
            } else if ty == 2 {
                let (v1_hi, v2_hi) = if is_256 { (cpu.ymm_hi[d], cpu.ymm_hi[s2]) } else { (0, 0) };
                cpu.xmm[d] = vfma_ps_128(ord, fop, v1_lo, v2_lo, v3_lo);
                if is_256 {
                    cpu.set_ymm_hi(d, vfma_ps_128(ord, fop, v1_hi, v2_hi, v3_hi));
                } else {
                    cpu.clear_ymm_hi_reg(d);
                }
            } else {
                let (v1_hi, v2_hi) = if is_256 { (cpu.ymm_hi[d], cpu.ymm_hi[s2]) } else { (0, 0) };
                cpu.xmm[d] = vfma_pd_128(ord, fop, v1_lo, v2_lo, v3_lo);
                if is_256 {
                    cpu.set_ymm_hi(d, vfma_pd_128(ord, fop, v1_hi, v2_hi, v3_hi));
                } else {
                    cpu.clear_ymm_hi_reg(d);
                }
            }
        }
        K_VBLENDV_RR | K_VBLENDV_RM => {
            let d = op.r0 as usize & 15;
            let m_idx = (op.imm & 0xFF) as usize & 15;
            let s1 = ((op.imm >> 8) as usize) & 15;
            let lg = (op.x & 0x3F) as usize;
            let mode = op.w & 3;
            let a_lo = cpu.xmm[s1];
            let (b_lo, b_hi) = if op.kind == K_VBLENDV_RM {
                cpu.vload(op)
            } else {
                let s2 = op.r1 as usize & 15;
                (cpu.xmm[s2], if mode == VM_V256 { cpu.ymm_hi[s2] } else { 0 })
            };
            let m_lo = cpu.xmm[m_idx];
            let blend_fn: fn(u128, u128, u128) -> u128 = match lg {
                0 => sk::blendvb,
                2 => sk::blendvps,
                _ => sk::blendvpd,
            };
            let res_lo = blend_fn(a_lo, b_lo, m_lo);
            match mode {
                VM_LEG => cpu.xmm[d] = res_lo,
                VM_V128 => {
                    cpu.xmm[d] = res_lo;
                    cpu.clear_ymm_hi_reg(d);
                }
                _ => {
                    let res_hi = blend_fn(cpu.ymm_hi[s1], b_hi, cpu.ymm_hi[m_idx]);
                    cpu.xmm[d] = res_lo;
                    cpu.set_ymm_hi(d, res_hi);
                }
            }
        }
        K_VTEST_RR | K_VTEST_RM => {
            let s1 = op.r0 as usize & 15;
            let is_256 = (op.w & 3) == VM_V256;
            let sub = op.x & 0x3F;
            let (a_lo, mut a_hi) = (cpu.xmm[s1], cpu.ymm_hi[s1]);
            let (b_lo, mut b_hi) = if op.kind == K_VTEST_RM {
                cpu.vload(op)
            } else {
                let s2 = op.r1 as usize & 15;
                (cpu.xmm[s2], cpu.ymm_hi[s2])
            };
            if !is_256 {
                a_hi = 0;
                b_hi = 0;
            }
            let m: u128 = match sub {
                0 => u128::MAX,
                1 => 0x8000_0000_8000_0000_8000_0000_8000_0000,
                _ => 0x8000_0000_0000_0000_8000_0000_0000_0000,
            };
            cpu.zf = (((a_lo & b_lo) | (a_hi & b_hi)) & m) == 0;
            cpu.cf = (((!a_lo & b_lo) | (!a_hi & b_hi)) & m) == 0;
            cpu.sf = false;
            cpu.of = false;
            cpu.set_pf(false);
        }
        K_BMI_RR | K_BMI_RM => {
            let sub = op.x & 0x3F;
            let wb = w as u32;
            let mem = op.kind == K_BMI_RM;
            match sub {
                0 => {
                    // Andn
                    let a = cpu.rget(op.imm as u8, w);
                    let b = if mem { cpu.read_mem(cpu.faddr(op), wb) } else { cpu.rget(op.r1, w) };
                    let res = (!a) & b;
                    cpu.rset(op.r0, w, res);
                    cpu.update_flags_logic(res, wb);
                }
                1..=3 => {
                    // Shlx, Shrx, Sarx
                    let a = if mem { cpu.read_mem(cpu.faddr(op), wb) } else { cpu.rget(op.imm as u8, w) };
                    let mask = if wb == 64 { 63 } else { 31 };
                    let cnt = (cpu.rget(op.r1, w) as u32) & mask;
                    let res = match sub {
                        1 => a.wrapping_shl(cnt),
                        2 => a.wrapping_shr(cnt),
                        _ => {
                            if wb == 64 {
                                ((a as i64) >> cnt) as u64
                            } else {
                                ((a as u32 as i32) >> cnt) as u32 as u64
                            }
                        }
                    };
                    cpu.rset(op.r0, w, res);
                }
                4 => {
                    // Rorx
                    let a = if mem { cpu.read_mem(cpu.faddr(op), wb) } else { cpu.rget(op.r1, w) };
                    let cnt = op.imm as u32;
                    let res = if wb == 64 { a.rotate_right(cnt) } else { (a as u32).rotate_right(cnt) as u64 };
                    cpu.rset(op.r0, w, res);
                }
                5..=7 => {
                    // Blsr, Blsmsk, Blsi
                    let a = if mem { cpu.read_mem(cpu.faddr(op), wb) } else { cpu.rget(op.r1, w) };
                    let res = match sub {
                        5 => a & a.wrapping_sub(1),
                        6 => a ^ a.wrapping_sub(1),
                        _ => a & a.wrapping_neg(),
                    };
                    cpu.rset(op.r0, w, res);
                    let mask = if wb >= 64 { u64::MAX } else { (1u64 << wb) - 1 };
                    cpu.zf = (res & mask) == 0;
                    cpu.sf = (res & (1u64 << (wb - 1))) != 0;
                    cpu.cf = if sub == 7 { (a & mask) != 0 } else { (a & mask) == 0 };
                    cpu.of = false;
                }
                8 => {
                    // Bzhi
                    let a = if mem { cpu.read_mem(cpu.faddr(op), wb) } else { cpu.rget(op.imm as u8, w) };
                    let idx = (cpu.rget(op.r1, w) & 0xFF) as u32;
                    let res = if idx >= wb {
                        cpu.cf = true;
                        a
                    } else {
                        cpu.cf = false;
                        if idx == 0 { 0 } else { a & ((1u64 << idx) - 1) }
                    };
                    cpu.rset(op.r0, w, res);
                    let mask = if wb >= 64 { u64::MAX } else { (1u64 << wb) - 1 };
                    cpu.zf = (res & mask) == 0;
                    cpu.sf = (res & (1u64 << (wb - 1))) != 0;
                    cpu.of = false;
                }
                9 => {
                    // Bextr
                    let src = if mem { cpu.read_mem(cpu.faddr(op), wb) } else { cpu.rget(op.imm as u8, w) };
                    let ctl = cpu.rget(op.r1, w);
                    let start = (ctl & 0xFF) as u32;
                    let len = ((ctl >> 8) & 0xFF) as u32;
                    let v = if start >= wb { 0 } else { src >> start };
                    let res = if len >= 64 { v } else { v & ((1u64 << len) - 1) };
                    let res = if wb == 32 { res & 0xFFFF_FFFF } else { res };
                    cpu.rset(op.r0, w, res);
                    cpu.update_flags_logic(res, wb);
                }
                10 => {
                    // Mulx
                    let src = if mem { cpu.read_mem(cpu.faddr(op), wb) } else { cpu.rget(op.imm as u8, w) };
                    if wb == 64 {
                        let prod = (cpu.gpr[2] as u128).wrapping_mul(src as u128);
                        cpu.rset(op.r1, 64, prod as u64);
                        cpu.rset(op.r0, 64, (prod >> 64) as u64);
                    } else {
                        let prod = (cpu.gpr[2] as u32 as u64).wrapping_mul(src as u32 as u64);
                        cpu.rset(op.r1, 32, prod as u32 as u64);
                        cpu.rset(op.r0, 32, (prod >> 32) as u32 as u64);
                    }
                }
                11 | 12 => {
                    // Adcx / Adox
                    let mask = if wb >= 64 { u64::MAX } else { (1u64 << wb) - 1 };
                    let a = cpu.rget(op.r0, w);
                    let b = if mem { cpu.read_mem(cpu.faddr(op), wb) & mask } else { cpu.rget(op.r1, w) };
                    let c = if sub == 11 { cpu.cf } else { cpu.of };
                    let full = (a as u128) + (b as u128) + (c as u128);
                    cpu.rset(op.r0, w, (full as u64) & mask);
                    if sub == 11 {
                        cpu.cf = full > mask as u128;
                    } else {
                        cpu.of = full > mask as u128;
                    }
                }
                _ => {
                    // Crc32
                    let src_bits = wb;
                    let dst_bits = op.imm as u8;
                    let v = if mem {
                        cpu.read_mem(cpu.faddr(op), src_bits)
                    } else if op.r1 & 0x80 != 0 {
                        (cpu.gpr[(op.r1 & 15) as usize] >> 8) & 0xFF
                    } else {
                        cpu.rget(op.r1, w)
                    };
                    let crc = sk::crc32c_step(cpu.rget(op.r0, 32) as u32, v, src_bits);
                    cpu.rset(op.r0, dst_bits, crc as u64);
                }
            }
        }
        K_YMM_LANE => {
            let sub = op.x & 0x3F;
            let mem = op.w != 0;
            match sub {
                0 => {
                    // Vpermq / Vpermpd
                    let d = op.r0 as usize & 15;
                    let (s_lo, s_hi) = if mem {
                        let a = cpu.faddr(op);
                        (cpu.read_u128(a), cpu.read_u128(a.wrapping_add(16)))
                    } else {
                        let s = op.r1 as usize & 15;
                        (cpu.xmm[s], cpu.ymm_hi[s])
                    };
                    let (lo, hi) = sk::vpermq(s_lo, s_hi, op.imm as u8);
                    cpu.xmm[d] = lo;
                    cpu.set_ymm_hi(d, hi);
                }
                1 => {
                    // Vpermd / Vpermps
                    let d = op.r0 as usize & 15;
                    let idx_reg = (op.imm & 15) as usize;
                    let (i_lo, i_hi) = (cpu.xmm[idx_reg], cpu.ymm_hi[idx_reg]);
                    let (t_lo, t_hi) = if mem {
                        let a = cpu.faddr(op);
                        (cpu.read_u128(a), cpu.read_u128(a.wrapping_add(16)))
                    } else {
                        let s = op.r1 as usize & 15;
                        (cpu.xmm[s], cpu.ymm_hi[s])
                    };
                    let (lo, hi) = sk::vpermd_ymm(i_lo, i_hi, t_lo, t_hi);
                    cpu.xmm[d] = lo;
                    cpu.set_ymm_hi(d, hi);
                }
                2 => {
                    // Vperm2f128 / Vperm2i128
                    let d = op.r0 as usize & 15;
                    let imm = (op.imm & 0xFF) as u8;
                    let s1 = ((op.imm >> 8) as usize) & 15;
                    let (s1_lo, s1_hi) = (cpu.xmm[s1], cpu.ymm_hi[s1]);
                    let (s2_lo, s2_hi) = if mem {
                        let a = cpu.faddr(op);
                        (cpu.read_u128(a), cpu.read_u128(a.wrapping_add(16)))
                    } else {
                        let s2 = op.r1 as usize & 15;
                        (cpu.xmm[s2], cpu.ymm_hi[s2])
                    };
                    let (lo, hi) = sk::vperm2_128(s1_lo, s1_hi, s2_lo, s2_hi, imm);
                    cpu.xmm[d] = lo;
                    cpu.set_ymm_hi(d, hi);
                }
                3 => {
                    // Vinsertf128 / Vinserti128
                    let d = op.r0 as usize & 15;
                    let imm = (op.imm & 1) as u8;
                    let s1 = ((op.imm >> 8) as usize) & 15;
                    let (lo, hi) = (cpu.xmm[s1], cpu.ymm_hi[s1]);
                    let inserted = if mem {
                        cpu.read_u128(cpu.faddr(op))
                    } else {
                        cpu.xmm[op.r1 as usize & 15]
                    };
                    if imm == 0 {
                        cpu.xmm[d] = inserted;
                        cpu.set_ymm_hi(d, hi);
                    } else {
                        cpu.xmm[d] = lo;
                        cpu.set_ymm_hi(d, inserted);
                    }
                }
                4 => {
                    // Vextractf128 / Vextracti128
                    let s = op.r1 as usize & 15;
                    let val = if (op.imm & 1) == 0 { cpu.xmm[s] } else { cpu.ymm_hi[s] };
                    if mem {
                        cpu.write_u128(cpu.faddr(op), val);
                    } else {
                        let d = op.r0 as usize & 15;
                        cpu.xmm[d] = val;
                        cpu.clear_ymm_hi_reg(d);
                    }
                }
                _ => {
                    // Vzeroall
                    cpu.xmm = [0; 16];
                    cpu.clear_all_ymm_hi();
                }
            }
        }
        K_INSN => {
            let ins = if op.imm != 0 && (*(op.imm as *const Instruction)).ip() == op.ip {
                *(op.imm as *const Instruction)
            } else {
                let mapped = crate::emu::to_mapped_addr(op.ip);
                let slice = core::slice::from_raw_parts(mapped as *const u8, op.len as usize);
                let mut dec = iced_x86::Decoder::with_ip(64, slice, op.ip, iced_x86::DecoderOptions::NONE);
                dec.decode()
            };
            cpu.exec_generic_ins(&ins);
        }
        K_LOOP => {
            let sub = op.x & 0x3F;
            let take = match sub {
                0 => cpu.gpr[1] == 0,
                1 => (cpu.gpr[1] as u32) == 0,
                _ => {
                    cpu.gpr[1] = cpu.gpr[1].wrapping_sub(1);
                    cpu.last_bits[1] = 64;
                    cpu.gpr[1] != 0
                        && match sub {
                            3 => cpu.zf,
                            4 => !cpu.zf,
                            _ => true,
                        }
                }
            };
            if take {
                cpu.rip = op.imm;
            }
        }
        K_VAES_RR..=K_VVNNI_RM => return exec_ext_all_remaining(cpu, op),
        K_X87 => super::x87::exec_x87_fast(cpu, op),
        _ => {}
    }
    false
}

/// Packed single-precision FMA on one 128-bit lane.
#[inline(always)]
fn vfma_ps_128(ord: u8, fop: u8, x1: u128, x2: u128, x3: u128) -> u128 {
    let neg_mul = fop == 2 || fop == 3;
    let acc_mode = match fop {
        1 | 3 => 1,
        4 => 2,
        5 => 3,
        _ => 0,
    };
    let (m1, m2, acc) = match ord {
        0 => (x1, x3, x2),
        1 => (x2, x1, x3),
        _ => (x2, x3, x1),
    };
    sk::fma_ps(m1, m2, acc, neg_mul, acc_mode)
}

/// Packed double-precision FMA on one 128-bit lane.
#[inline(always)]
fn vfma_pd_128(ord: u8, fop: u8, x1: u128, x2: u128, x3: u128) -> u128 {
    let neg_mul = fop == 2 || fop == 3;
    let acc_mode = match fop {
        1 | 3 => 1,
        4 => 2,
        5 => 3,
        _ => 0,
    };
    let (m1, m2, acc) = match ord {
        0 => (x1, x3, x2),
        1 => (x2, x1, x3),
        _ => (x2, x3, x1),
    };
    sk::fma_pd(m1, m2, acc, neg_mul, acc_mode)
}
