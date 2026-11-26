//! Legacy MMX forms of the baseline SSE instructions use the physical x87 slots.
//! Reuse the packed-integer kernels with the MMX lane width and memory operand size.
use super::regfile_ext::{with_x87_mut, x87_snapshot};
use super::{simd_kernels as simd, Cpu};
use iced_x86::{Instruction, Mnemonic::*, OpKind, Register};

pub(super) fn uses_mmx(ins: &Instruction) -> bool {
    ins.op0_register().is_mm()
        || ins.op1_register().is_mm()
        || matches!(ins.mnemonic(), Maskmovq | Cvtpi2ps | Cvtpi2pd)
}

fn slot(reg: Register) -> usize {
    (reg as usize) - Register::MM0 as usize
}

unsafe fn mask_store(cpu: &Cpu, ins: &Instruction, data: u128, mask: u128, bytes: usize) {
    let base = match ins.op0_kind() {
        OpKind::MemorySegEDI => u64::from(cpu.gpr[7] as u32),
        _ => cpu.gpr[7],
    };
    let base = if ins.segment_prefix() == Register::GS {
        base.wrapping_add(crate::exe::teb_self() as u64)
    } else {
        base
    };
    for i in 0..bytes {
        if mask & (0x80 << (8 * i)) != 0 {
            cpu.write_mem(base.wrapping_add(i as u64), 8, (data >> (8 * i)) as u64);
        }
    }
}

pub(super) unsafe fn maskmovdqu(cpu: &Cpu, ins: &Instruction) {
    let data = cpu.xmm[Cpu::xmm_idx(ins.op1_register()).unwrap()];
    let mask = cpu.xmm[Cpu::xmm_idx(ins.op2_register()).unwrap()];
    mask_store(cpu, ins, data, mask, 16);
}

pub(super) unsafe fn execute(cpu: &mut Cpu, ins: &Instruction) -> bool {
    let mut st = x87_snapshot();
    let get = |reg| u64::from_le_bytes(st.regs[slot(reg)][..8].try_into().unwrap());
    let mn = ins.mnemonic();
    let dest = ins.op0_register();
    let src = ins.op1_register();
    let a = if dest.is_mm() { u128::from(get(dest)) } else { 0 };
    let b = if src.is_mm() {
        u128::from(get(src))
    } else if ins.op1_kind() == OpKind::Memory {
        if ins.memory_size().size() == 16 {
            cpu.read_u128(cpu.eff_addr(ins))
        } else {
            u128::from(cpu.read_mem(cpu.eff_addr(ins), (ins.memory_size().size() * 8) as u32))
        }
    } else if let Some(i) = Cpu::xmm_idx(src) {
        cpu.xmm[i]
    } else if ins.op1_kind() == OpKind::Immediate8 {
        u128::from(ins.immediate8())
    } else if ins.op1_kind() == OpKind::Register {
        u128::from(cpu.get_reg(src))
    } else {
        0
    };

    let value = match mn {
        Maskmovq => {
            mask_store(cpu, ins, b, get(ins.op2_register()).into(), 8);
            None
        }
        Movq2dq => {
            cpu.xmm[Cpu::xmm_idx(dest).unwrap()] = b;
            None
        }
        Movd | Movq | Movntq if !dest.is_mm() => {
            cpu.write_op(ins, 0, if mn == Movd { 32 } else { 64 }, b as u64);
            None
        }
        Pmovmskb | Pextrw => {
            let value = if mn == Pmovmskb {
                simd::pmovmskb(b) & 0xff
            } else {
                ((b >> (16 * (ins.immediate8() & 3))) as u16).into()
            };
            cpu.set_reg(dest, value);
            None
        }
        Cvtpi2pd | Cvtpi2ps => {
            let i = Cpu::xmm_idx(dest).unwrap();
            let (x, y) = (b as i32, (b >> 32) as i32);
            cpu.xmm[i] = if mn == Cvtpi2pd {
                u128::from((x as f64).to_bits()) | u128::from((y as f64).to_bits()) << 64
            } else {
                (cpu.xmm[i] & !u128::from(u64::MAX))
                    | u128::from((x as f32).to_bits())
                    | u128::from((y as f32).to_bits()) << 32
            };
            if !src.is_mm() {
                return true;
            } // The memory form does not transition to MMX mode.
            None
        }
        Cvtpd2pi | Cvttpd2pi | Cvtps2pi | Cvttps2pi => {
            let lane = |i| {
                let v = if matches!(mn, Cvtpd2pi | Cvttpd2pi) {
                    f64::from_bits((b >> (64 * i)) as u64)
                } else {
                    f32::from_bits((b >> (32 * i)) as u32) as f64
                };
                let rounded = match (matches!(mn, Cvttpd2pi | Cvttps2pi), cpu.jx.mxcsr >> 13 & 3) {
                    (true, _) | (_, 3) => v.trunc(),
                    (_, 1) => v.floor(),
                    (_, 2) => v.ceil(),
                    _ => v.round_ties_even(),
                };
                super::sse_cvt_i32(rounded, true) as u64
            };
            Some(lane(0u32) | lane(1u32) << 32)
        }
        _ => {
            let narrowed = |v: u128| (v as u32 as u64) | ((v >> 64) as u32 as u64) << 32;
            let value = match mn {
                Movd => b as u32 as u64,
                Movq | Movdq2q => b as u64,
                Pinsrw => {
                    let shift = (ins.immediate8() & 3) * 16;
                    ((a as u64) & !(0xffff << shift)) | ((b as u16 as u64) << shift)
                }
                Pshufw => (0..4).fold(0, |v, i| {
                    v | (((b >> (16 * ((ins.immediate8() >> (2 * i)) & 3))) as u16 as u64) << (16 * i))
                }),
                Packsswb => narrowed(simd::packsswb(a, b)),
                Packuswb => narrowed(simd::packuswb(a, b)),
                Packssdw => narrowed(simd::packssdw(a, b)),
                Punpckhbw => simd::punpcklbw(a >> 32, b >> 32) as u64,
                Punpckhwd => simd::punpcklwd(a >> 32, b >> 32) as u64,
                Punpckhdq => simd::punpckldq(a >> 32, b >> 32) as u64,
                Psrlw | Psraw | Psllw => {
                    simd::pshift_w(a, b as u64, if mn == Psraw { 2 } else { u8::from(mn == Psrlw) }) as u64
                }
                Psrld | Psrad | Pslld => {
                    simd::pshift_d(a, b as u64, if mn == Psrad { 2 } else { u8::from(mn == Psrld) }) as u64
                }
                Psrlq | Psllq => simd::pshift_q(a, b as u64, mn == Psrlq) as u64,
                Pand => (a & b) as u64,
                Pandn => (!a & b) as u64,
                Por => (a | b) as u64,
                Pxor => (a ^ b) as u64,
                Punpcklbw => simd::punpcklbw(a, b) as u64,
                Punpcklwd => simd::punpcklwd(a, b) as u64,
                Punpckldq => simd::punpckldq(a, b) as u64,
                Pcmpgtb => simd::pcmpgtb(a, b) as u64,
                Pcmpgtw => simd::pcmpgtw(a, b) as u64,
                Pcmpgtd => simd::pcmpgtd(a, b) as u64,
                Pcmpeqb => simd::pcmpeqb(a, b) as u64,
                Pcmpeqw => simd::pcmpeqw(a, b) as u64,
                Pcmpeqd => simd::pcmpeqd(a, b) as u64,
                Paddq => simd::paddq(a, b) as u64,
                Pmullw => simd::pmullw(a, b) as u64,
                Psubusb => simd::psubusb(a, b) as u64,
                Psubusw => simd::psubusw(a, b) as u64,
                Paddusb => simd::paddusb(a, b) as u64,
                Paddusw => simd::paddusw(a, b) as u64,
                Pmulhw => simd::pmulhw(a, b) as u64,
                Psubsb => simd::psubsb(a, b) as u64,
                Psubsw => simd::psubsw(a, b) as u64,
                Paddsb => simd::paddsb(a, b) as u64,
                Paddsw => simd::paddsw(a, b) as u64,
                Pmaddwd => simd::pmaddwd(a, b) as u64,
                Psubb => simd::psubb(a, b) as u64,
                Psubw => simd::psubw(a, b) as u64,
                Psubd => simd::psubd(a, b) as u64,
                Paddb => simd::paddb(a, b) as u64,
                Paddw => simd::paddw(a, b) as u64,
                Paddd => simd::paddd(a, b) as u64,
                Pminub => simd::pminub(a, b) as u64,
                Pmaxub => simd::pmaxub(a, b) as u64,
                Pavgb => simd::pavgb(a, b) as u64,
                Pavgw => simd::pavgw(a, b) as u64,
                Pmulhuw => simd::pmulhuw(a, b) as u64,
                Pminsw => simd::pminsw(a, b) as u64,
                Pmaxsw => simd::pmaxsw(a, b) as u64,
                Psadbw => simd::psadbw(a, b) as u64,
                Pmuludq => simd::pmuludq(a, b) as u64,
                Psubq => simd::psubq(a, b) as u64,
                Pabsb => simd::pabsb(b) as u64,
                Pabsw => simd::pabsw(b) as u64,
                Pabsd => simd::pabsd(b) as u64,
                Psignb => simd::psignb(a, b) as u64,
                Psignw => simd::psignw(a, b) as u64,
                Psignd => simd::psignd(a, b) as u64,
                Pmulhrsw => simd::pmulhrsw(a, b) as u64,
                Pmaddubsw => simd::pmaddubsw(a, b) as u64,
                Phaddw | Phsubw => narrowed(simd::phadd_sub_w(a, b, mn == Phsubw)),
                Phaddd | Phsubd => narrowed(simd::phadd_sub_d(a, b, mn == Phsubd)),
                Phaddsw | Phsubsw => narrowed(simd::phadd_sub_sw(a, b, mn == Phsubsw)),
                Pshufb => simd::pshufb(a | a << 64, b) as u64,
                Palignr => ((a << 64) | b).checked_shr(u32::from(ins.immediate8()) * 8).unwrap_or(0) as u64,
                _ => return false,
            };
            Some(value)
        }
    };
    if !super::precise_faults() || super::FAULT_PENDING.with(|pending| pending.get().is_none()) {
        if let Some(value) = value {
            let reg = &mut st.regs[slot(dest)];
            reg[..8].copy_from_slice(&value.to_le_bytes());
            reg[8..10].fill(0xff);
            reg[10..16].fill(0);
        }
        st.valid_mask = 0xff;
        st.top = 0;
        st.sync_top();
        super::regfile_ext::x87_restore(st);
    }
    true
}

pub(super) fn emms() {
    with_x87_mut(|st| st.valid_mask = 0);
}
