//! Independent MXCSR input/output flushing on ARM cores with only the combined FPCR.FZ bit.
//! This path runs only while a guest selects exactly one flush mode on a core without AFP.
use super::{features, Cpu};
use iced_x86::{Instruction, Mnemonic::*, OpKind, Register};

#[inline]
pub(super) fn mixed(mxcsr: u32) -> bool {
    ((mxcsr >> 15) ^ (mxcsr >> 6)) & 1 != 0
}

#[inline]
pub(super) fn software_mode(mxcsr: u32) -> bool {
    mixed(mxcsr) && !features::host().has(features::HostFeatures::AFP)
}

#[derive(Clone, Copy)]
enum Lanes {
    SS,
    SD,
    PS,
    PD,
}
use Lanes::*;

impl Lanes {
    fn map(self, value: u128, mut f: impl FnMut(u64, u64, u64) -> u64) -> u128 {
        let (bits, count, min): (usize, usize, u64) = match self {
            SS => (32, 1, 1 << 23),
            PS => (32, 4, 1 << 23),
            SD => (64, 1, 1 << 52),
            PD => (64, 2, 1 << 52),
        };
        let sign = 1 << (bits - 1);
        let mask = (1u128 << bits) - 1;
        let mut result = value;
        for i in 0..count {
            let shift = i * bits;
            let lane = ((value >> shift) & mask) as u64;
            result = (result & !(mask << shift)) | (u128::from(f(lane, sign, min)) << shift);
        }
        result
    }

    fn flush(self, value: u128) -> u128 {
        self.map(value, |v, sign, min| if v & !sign < min { v & sign } else { v })
    }

    fn has_min_normal(self, value: u128) -> bool {
        let mut found = false;
        self.map(value, |v, sign, min| {
            found |= v & !sign == min;
            v
        });
        found
    }

    fn flush_result(self, value: u128, truncated: u128) -> u128 {
        // A tiny exact value can round UP to the smallest normal. A second execution toward
        // zero distinguishes it from an exact normal at that boundary (x86 flushes the former).
        let mut probe = truncated;
        self.map(value, |v, sign, min| {
            let rz = probe as u64 & (sign | (sign - 1));
            probe >>= if sign == 1 << 31 { 32 } else { 64 };
            if v & !sign < min || (v & !sign == min && rz & !sign < min) {
                v & sign
            } else {
                v
            }
        })
    }
}

pub(super) fn flush_lanes(value: u128, double: bool) -> u128 {
    if double {
        PD.flush(value)
    } else {
        PS.flush(value)
    }
}

/// FCSEL/BSL copy bits unchanged, even when their preceding FP comparison consumed a signed zero.
pub(super) fn minmax_result(value: u128, double: bool) -> u128 {
    let fpcr = super::cpu::read_fpcr();
    if fpcr & 1 != 0 || (fpcr & (1 << 24 | 2) == 1 << 24) {
        flush_lanes(value, double)
    } else {
        value
    }
}

// Input format and, for instructions that can underflow, result format. Moves, bitwise
// operations, shuffles, integer operands, and scalar passthrough lanes are never flushed.
fn formats(ins: &Instruction) -> Option<(Lanes, Option<Lanes>)> {
    Some(match ins.mnemonic() {
        Addss | Vaddss | Subss | Vsubss | Mulss | Vmulss | Divss | Vdivss | Vfmadd132ss | Vfmadd213ss
        | Vfmadd231ss | Vfmsub132ss | Vfmsub213ss | Vfmsub231ss | Vfnmadd132ss | Vfnmadd213ss
        | Vfnmadd231ss | Vfnmsub132ss | Vfnmsub213ss | Vfnmsub231ss => (SS, Some(SS)),
        Minss | Vminss | Maxss | Vmaxss | Sqrtss | Vsqrtss | Roundss | Vroundss | Cmpss | Vcmpss | Comiss
        | Vcomiss | Ucomiss | Vucomiss => (SS, None),
        Addsd | Vaddsd | Subsd | Vsubsd | Mulsd | Vmulsd | Divsd | Vdivsd | Vfmadd132sd | Vfmadd213sd
        | Vfmadd231sd | Vfmsub132sd | Vfmsub213sd | Vfmsub231sd | Vfnmadd132sd | Vfnmadd213sd
        | Vfnmadd231sd | Vfnmsub132sd | Vfnmsub213sd | Vfnmsub231sd => (SD, Some(SD)),
        Minsd | Vminsd | Maxsd | Vmaxsd | Sqrtsd | Vsqrtsd | Roundsd | Vroundsd | Cmpsd | Vcmpsd | Comisd
        | Vcomisd | Ucomisd | Vucomisd => (SD, None),
        Addps | Vaddps | Subps | Vsubps | Mulps | Vmulps | Divps | Vdivps | Addsubps | Vaddsubps | Haddps
        | Vhaddps | Hsubps | Vhsubps | Vfmadd132ps | Vfmadd213ps | Vfmadd231ps | Vfmsub132ps
        | Vfmsub213ps | Vfmsub231ps | Vfnmadd132ps | Vfnmadd213ps | Vfnmadd231ps | Vfnmsub132ps
        | Vfnmsub213ps | Vfnmsub231ps | Vfmaddsub132ps | Vfmaddsub213ps | Vfmaddsub231ps | Vfmsubadd132ps
        | Vfmsubadd213ps | Vfmsubadd231ps => (PS, Some(PS)),
        Minps | Vminps | Maxps | Vmaxps | Sqrtps | Vsqrtps | Roundps | Vroundps | Cmpps | Vcmpps => {
            (PS, None)
        }
        Addpd | Vaddpd | Subpd | Vsubpd | Mulpd | Vmulpd | Divpd | Vdivpd | Addsubpd | Vaddsubpd | Haddpd
        | Vhaddpd | Hsubpd | Vhsubpd | Vfmadd132pd | Vfmadd213pd | Vfmadd231pd | Vfmsub132pd
        | Vfmsub213pd | Vfmsub231pd | Vfnmadd132pd | Vfnmadd213pd | Vfnmadd231pd | Vfnmsub132pd
        | Vfnmsub213pd | Vfnmsub231pd | Vfmaddsub132pd | Vfmaddsub213pd | Vfmaddsub231pd | Vfmsubadd132pd
        | Vfmsubadd213pd | Vfmsubadd231pd => (PD, Some(PD)),
        Minpd | Vminpd | Maxpd | Vmaxpd | Sqrtpd | Vsqrtpd | Roundpd | Vroundpd | Cmppd | Vcmppd => {
            (PD, None)
        }
        Cvtss2sd | Vcvtss2sd | Cvtss2si | Vcvtss2si | Cvttss2si | Vcvttss2si => (SS, None),
        Cvtsd2si | Vcvtsd2si | Cvttsd2si | Vcvttsd2si => (SD, None),
        Cvtsd2ss | Vcvtsd2ss => (SD, Some(SS)),
        Cvtpd2ps | Vcvtpd2ps => (PD, Some(PS)),
        Cvtpd2pi | Cvttpd2pi | Cvtpd2dq | Vcvtpd2dq | Cvttpd2dq | Vcvttpd2dq => (PD, None),
        Cvtps2pi | Cvttps2pi | Cvtps2dq | Vcvtps2dq | Cvttps2dq | Vcvttps2dq | Cvtps2pd | Vcvtps2pd
        | Vcvtps2ph => (PS, None),
        Dpps | Vdpps => (PS, Some(PS)),
        Dppd | Vdppd => (PD, Some(PD)),
        _ => return None,
    })
}

pub(super) fn uses_fp(ins: &Instruction) -> bool {
    formats(ins).is_some()
}

// DPPS/DPPD round at each multiply and each level of the horizontal add tree.
// A subnormal intermediate is an input to the next operation.
unsafe fn dot_product(cpu: &mut Cpu, ins: &Instruction) {
    let double = matches!(ins.mnemonic(), Dppd | Vdppd);
    let src = u32::from(Cpu::is_vex(ins));
    let wide = Cpu::is_ymm(ins.op_register(0));
    let (a, ah) = cpu.vsrc(ins, src, wide);
    let (b, bh) = cpu.vsrc(ins, src + 1, wide);
    let format = if double { SD } else { SS };
    let bits = if double { 64 } else { 32 };
    let count = 128 / bits;
    let imm = ins.immediate8();
    let step = |op, mut x, mut y| {
        let daz = cpu.jx.mxcsr & (1 << 6) != 0;
        if daz {
            x = format.flush(x);
            y = format.flush(y);
        }
        let arith = || -> u128 {
            // Preserve both directed-round executions at the smallest normal boundary.
            let (x, y) = (std::hint::black_box(x), std::hint::black_box(y));
            if double {
                super::sse_arith64(op, f64::from_bits(x as u64), f64::from_bits(y as u64)).to_bits().into()
            } else {
                super::sse_arith32(op, f32::from_bits(x as u32), f32::from_bits(y as u32)).to_bits().into()
            }
        };
        let value = arith();
        if daz {
            return value;
        }
        let truncated = if format.has_min_normal(value) {
            let saved = super::cpu::read_fpcr();
            super::cpu::write_fpcr(saved | (3 << 22));
            let rz = arith();
            super::cpu::write_fpcr(saved);
            rz
        } else {
            value
        };
        format.flush_result(value, truncated)
    };
    let lane = |a: u128, b: u128| {
        let mut p = [0; 4];
        for (i, v) in p[..count].iter_mut().enumerate() {
            if imm & (16 << i) != 0 {
                *v = step(2, a >> (bits * i), b >> (bits * i));
            }
        }
        let lo = step(0, p[0], p[1]);
        let sum = if double { lo } else { step(0, lo, step(0, p[2], p[3])) };
        (0..count).fold(0, |out, i| if imm & (1 << i) != 0 { out | (sum << (bits * i)) } else { out })
    };
    cpu.vdst(ins, Cpu::xmm_idx(ins.op_register(0)).unwrap(), lane(a, b), if wide { lane(ah, bh) } else { 0 });
}

pub(super) unsafe fn execute(cpu: &mut Cpu, ins: &Instruction) -> Option<bool> {
    let (input, output) = formats(ins)?;
    if matches!(ins.mnemonic(), Dpps | Vdpps | Dppd | Vdppd) {
        dot_product(cpu, ins);
        return Some(true);
    }
    if cpu.jx.mxcsr & (1 << 6) != 0 {
        return Some(flush_inputs(cpu, ins, input));
    }
    let output = output?; // FTZ only affects arithmetic underflow.
    let dest = Cpu::xmm_idx(ins.op_register(0))?;
    let mut probe = cpu.clone();
    let handled = cpu.exec_generic_inner(ins);
    let wide = Cpu::is_ymm(ins.op_register(0));
    if output.has_min_normal(cpu.xmm[dest]) || (wide && output.has_min_normal(cpu.ymm_hi[dest])) {
        let fpcr = super::cpu::read_fpcr();
        super::cpu::write_fpcr(fpcr | (3 << 22));
        probe.exec_generic_inner(ins);
        super::cpu::write_fpcr(fpcr);
    } else {
        probe.xmm[dest] = cpu.xmm[dest];
        probe.ymm_hi[dest] = cpu.ymm_hi[dest];
    }
    cpu.xmm[dest] = output.flush_result(cpu.xmm[dest], probe.xmm[dest]);
    if wide {
        cpu.set_ymm_hi(dest, output.flush_result(cpu.ymm_hi[dest], probe.ymm_hi[dest]));
    }
    Some(handled)
}

unsafe fn flush_inputs(cpu: &mut Cpu, ins: &Instruction, lanes: Lanes) -> bool {
    let mut work = cpu.clone();
    let mut decoded = *ins;
    let mut memory = [0u128; 2];
    for i in 0..ins.op_count() {
        let reg = ins.op_register(i);
        if ins.op_kind(i) == OpKind::Register {
            if let Some(r) = Cpu::xmm_idx(reg) {
                work.xmm[r] = lanes.flush(cpu.xmm[r]);
                if Cpu::is_ymm(reg) {
                    work.ymm_hi[r] = lanes.flush(cpu.ymm_hi[r]);
                }
            }
        } else if ins.op_kind(i) == OpKind::Memory && i != 0 {
            let (lo, hi) = cpu.vsrc(ins, i, ins.memory_size().size() == 32);
            memory = [lanes.flush(lo), lanes.flush(hi)];
            decoded.set_memory_base(Register::None);
            decoded.set_memory_index(Register::None);
            decoded.set_segment_prefix(Register::None);
            decoded.set_memory_displacement64(memory.as_ptr() as u64);
        }
    }
    let handled = work.exec_generic_inner(&decoded);
    std::hint::black_box(&memory);
    // Commit only architectural outputs. COMI and CVT-to-integer instructions have no XMM output.
    cpu.gpr = work.gpr;
    cpu.last_bits = work.last_bits;
    cpu.set_rflags(work.rflags());
    if !matches!(
        ins.mnemonic(),
        Comiss | Vcomiss | Ucomiss | Vucomiss | Comisd | Vcomisd | Ucomisd | Vucomisd
    ) && ins.op_kind(0) == OpKind::Register
    {
        if let Some(d) = Cpu::xmm_idx(ins.op_register(0)) {
            let mut lo = work.xmm[d];
            if matches!(ins.mnemonic(), Cvtsd2ss | Vcvtsd2ss) {
                let merge = if Cpu::is_vex(ins) { ins.op_register(1) } else { ins.op_register(0) };
                lo = cpu.xmm[Cpu::xmm_idx(merge).unwrap()] & !u128::from(u32::MAX) | u128::from(lo as u32);
            }
            cpu.vdst(ins, d, lo, work.ymm_hi[d]);
        }
    }
    handled
}
