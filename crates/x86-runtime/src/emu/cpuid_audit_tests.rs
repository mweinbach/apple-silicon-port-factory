//! Honest-CPUID audit: walks every leaf/subleaf returned by [`super::query`], verifies every
//! advertised feature bit is mapped, asserts every 64-bit-mode `iced_x86::Code` form covered by
//! an advertised extension lowers natively to [`InstructionLowering::Arm64`] (outside explicit,
//! tracked non-Slice-5 allowlists), verifies `XCR0`/`xgetbv` consistency, verifies unimplemented
//! extensions (including all AVX-512 subsets, FSGSBASE, WAITPKG, PKU, AVX-IFMA, AVX-VNNI-INT8,
//! AVX-NE-CONVERT) remain unadvertised, and verifies host-feature gating.

use super::{query, XCR0, XSAVE_SIZE, YMM_OFFSET};
use crate::emu::features::{self, HostFeatures as H};
use crate::emu::{instruction_lowering, Cpu, InstructionLowering};
use iced_x86::{
    Code, CpuidFeature as F, Decoder, DecoderOptions, Encoder, Instruction, MemoryOperand,
    OpCodeOperandKind as O, Register,
};
use std::collections::{BTreeMap, BTreeSet};

/// Map every architectural `CpuidFeature` that can be advertised via `cpuid::query` to its
/// `(leaf, subleaf, reg_index, bit)` position (`reg_index`: 0=EAX, 1=EBX, 2=ECX, 3=EDX).
fn feature_bit(feature: F) -> Option<(u32, u32, usize, u32)> {
    Some(match feature {
        // Leaf 1 EDX
        F::FPU => (1, 0, 3, 0),
        F::TSC => (1, 0, 3, 4),
        F::MSR => (1, 0, 3, 5),
        F::CX8 => (1, 0, 3, 8),
        F::SEP => (1, 0, 3, 11),
        F::CMOV => (1, 0, 3, 15),
        F::CLFSH => (1, 0, 3, 19),
        F::MMX => (1, 0, 3, 23),
        F::FXSR => (1, 0, 3, 24),
        F::SSE => (1, 0, 3, 25),
        F::SSE2 => (1, 0, 3, 26),
        // Leaf 1 ECX
        F::SSE3 => (1, 0, 2, 0),
        F::PCLMULQDQ => (1, 0, 2, 1),
        F::MONITOR => (1, 0, 2, 3),
        F::VMX => (1, 0, 2, 5),
        F::SMX => (1, 0, 2, 6),
        F::SSSE3 => (1, 0, 2, 9),
        F::FMA => (1, 0, 2, 12),
        F::CMPXCHG16B => (1, 0, 2, 13),
        F::SSE4_1 => (1, 0, 2, 19),
        F::SSE4_2 => (1, 0, 2, 20),
        F::MOVBE => (1, 0, 2, 22),
        F::POPCNT => (1, 0, 2, 23),
        F::AES => (1, 0, 2, 25),
        F::XSAVE => (1, 0, 2, 26),
        F::AVX => (1, 0, 2, 28),
        F::F16C => (1, 0, 2, 29),
        F::RDRAND => (1, 0, 2, 30),
        // Leaf 7, subleaf 0 EBX
        F::FSGSBASE => (7, 0, 1, 0),
        F::SGX1 => (7, 0, 1, 2),
        F::BMI1 => (7, 0, 1, 3),
        F::HLE => (7, 0, 1, 4),
        F::AVX2 => (7, 0, 1, 5),
        F::BMI2 => (7, 0, 1, 8),
        F::INVPCID => (7, 0, 1, 10),
        F::RTM => (7, 0, 1, 11),
        F::MPX => (7, 0, 1, 14),
        F::AVX512F => (7, 0, 1, 16),
        F::AVX512DQ => (7, 0, 1, 17),
        F::RDSEED => (7, 0, 1, 18),
        F::ADX => (7, 0, 1, 19),
        F::SMAP => (7, 0, 1, 20),
        F::AVX512_IFMA => (7, 0, 1, 21),
        F::CLFLUSHOPT => (7, 0, 1, 23),
        F::CLWB => (7, 0, 1, 24),
        F::AVX512PF => (7, 0, 1, 26),
        F::AVX512ER => (7, 0, 1, 27),
        F::AVX512CD => (7, 0, 1, 28),
        F::SHA => (7, 0, 1, 29),
        F::AVX512BW => (7, 0, 1, 30),
        F::AVX512VL => (7, 0, 1, 31),
        // Leaf 7, subleaf 0 ECX
        F::PREFETCHWT1 => (7, 0, 2, 0),
        F::AVX512_VBMI => (7, 0, 2, 1),
        F::PKU => (7, 0, 2, 3),
        F::WAITPKG => (7, 0, 2, 5),
        F::AVX512_VBMI2 => (7, 0, 2, 6),
        F::CET_SS => (7, 0, 2, 7),
        F::GFNI => (7, 0, 2, 8),
        F::VAES => (7, 0, 2, 9),
        F::VPCLMULQDQ => (7, 0, 2, 10),
        F::AVX512_VNNI => (7, 0, 2, 11),
        F::AVX512_BITALG => (7, 0, 2, 12),
        F::AVX512_VPOPCNTDQ => (7, 0, 2, 14),
        F::RDPID => (7, 0, 2, 22),
        F::KL => (7, 0, 2, 23),
        F::CLDEMOTE => (7, 0, 2, 25),
        F::MOVDIRI => (7, 0, 2, 27),
        F::MOVDIR64B => (7, 0, 2, 28),
        F::ENQCMD => (7, 0, 2, 29),
        // Leaf 7, subleaf 0 EDX
        F::AVX512_4VNNIW => (7, 0, 3, 2),
        F::AVX512_4FMAPS => (7, 0, 3, 3),
        F::UINTR => (7, 0, 3, 5),
        F::AVX512_VP2INTERSECT => (7, 0, 3, 8),
        F::SERIALIZE => (7, 0, 3, 14),
        F::TSXLDTRK => (7, 0, 3, 16),
        F::PCONFIG => (7, 0, 3, 18),
        F::CET_IBT => (7, 0, 3, 20),
        F::AMX_BF16 => (7, 0, 3, 22),
        F::AVX512_FP16 => (7, 0, 3, 23),
        F::AMX_TILE => (7, 0, 3, 24),
        F::AMX_INT8 => (7, 0, 3, 25),
        // Leaf 7, subleaf 1 EAX
        F::SHA512 => (7, 1, 0, 0),
        F::SM3 => (7, 1, 0, 1),
        F::SM4 => (7, 1, 0, 2),
        F::RAO_INT => (7, 1, 0, 3),
        F::AVX_VNNI => (7, 1, 0, 4),
        F::AVX512_BF16 => (7, 1, 0, 5),
        F::CMPCCXADD => (7, 1, 0, 7),
        F::FRED => (7, 1, 0, 17),
        F::LKGS => (7, 1, 0, 18),
        F::WRMSRNS => (7, 1, 0, 19),
        F::AMX_FP16 => (7, 1, 0, 21),
        F::HRESET => (7, 1, 0, 22),
        F::AVX_IFMA => (7, 1, 0, 23),
        // Leaf 7, subleaf 1 EDX
        F::AVX_VNNI_INT8 => (7, 1, 3, 4),
        F::AVX_NE_CONVERT => (7, 1, 3, 5),
        F::AMX_COMPLEX => (7, 1, 3, 8),
        F::AVX_VNNI_INT16 => (7, 1, 3, 10),
        // Leaf 0xD, subleaf 1 EAX
        F::XSAVEOPT => (0xd, 1, 0, 0),
        F::XSAVEC => (0xd, 1, 0, 1),
        F::XSAVES => (0xd, 1, 0, 3),
        // Extended Leaf 0x8000_0001
        F::LZCNT => (0x8000_0001, 0, 2, 5),
        F::SSE4A => (0x8000_0001, 0, 2, 6),
        F::PREFETCHW => (0x8000_0001, 0, 2, 8),
        F::XOP => (0x8000_0001, 0, 2, 11),
        F::SKINIT => (0x8000_0001, 0, 2, 12),
        F::LWP => (0x8000_0001, 0, 2, 15),
        F::FMA4 => (0x8000_0001, 0, 2, 16),
        F::TBM => (0x8000_0001, 0, 2, 21),
        F::MONITORX => (0x8000_0001, 0, 2, 29),
        F::SYSCALL => (0x8000_0001, 0, 3, 11),
        F::RDTSCP => (0x8000_0001, 0, 3, 27),
        F::X64 => (0x8000_0001, 0, 3, 29),
        F::D3NOWEXT => (0x8000_0001, 0, 3, 30),
        F::D3NOW => (0x8000_0001, 0, 3, 31),
        // Extended Leaf 0x8000_0008 EBX
        F::CLZERO => (0x8000_0008, 0, 1, 0),
        F::RDPRU => (0x8000_0008, 0, 1, 4),
        F::MCOMMIT => (0x8000_0008, 0, 1, 8),
        F::WBNOINVD => (0x8000_0008, 0, 1, 9),
        _ => return None,
    })
}

/// Every feature bit advertised by `cpuid::query` on any feature leaf/subleaf must be
/// accounted for here: either mapped to a `CpuidFeature` verified below, or an architectural /
/// non-instruction capability flag (topology, OSXSAVE, NX, ERMS, deprecated x87 FCS/FDS, LAHF/SAHF).
fn known_advertised_bit(leaf: u32, subleaf: u32, reg: usize, bit: u32) -> Option<&'static str> {
    Some(match (leaf, subleaf, reg, bit) {
        // Leaf 1 ECX
        (1, 0, 2, 0) => "SSE3",
        (1, 0, 2, 1) => "PCLMULQDQ",
        (1, 0, 2, 9) => "SSSE3",
        (1, 0, 2, 12) => "FMA",
        (1, 0, 2, 13) => "CMPXCHG16B",
        (1, 0, 2, 19) => "SSE4_1",
        (1, 0, 2, 20) => "SSE4_2",
        (1, 0, 2, 22) => "MOVBE",
        (1, 0, 2, 23) => "POPCNT",
        (1, 0, 2, 25) => "AES",
        (1, 0, 2, 26) => "XSAVE",
        (1, 0, 2, 27) => "OSXSAVE (architectural XCR0 gate)",
        (1, 0, 2, 28) => "AVX",
        (1, 0, 2, 29) => "F16C",
        (1, 0, 2, 30) => "RDRAND",
        // Leaf 1 EDX
        (1, 0, 3, 4) => "TSC",
        (1, 0, 3, 8) => "CX8",
        (1, 0, 3, 15) => "CMOV",
        (1, 0, 3, 23) => "MMX",
        (1, 0, 3, 24) => "FXSR",
        (1, 0, 3, 25) => "SSE",
        (1, 0, 3, 26) => "SSE2",
        (1, 0, 3, 28) => "HTT (topology flag)",
        // Leaf 7.0 EBX
        (7, 0, 1, 3) => "BMI1",
        (7, 0, 1, 5) => "AVX2",
        (7, 0, 1, 8) => "BMI2",
        (7, 0, 1, 9) => "ERMS (rep movsb/stosb hint)",
        (7, 0, 1, 13) => "FDP_EXCPTN_ONLY / deprecated FCS/FDS",
        (7, 0, 1, 18) => "RDSEED",
        (7, 0, 1, 19) => "ADX",
        (7, 0, 1, 29) => "SHA",
        // Leaf 7.0 ECX
        (7, 0, 2, 8) => "GFNI",
        (7, 0, 2, 9) => "VAES",
        (7, 0, 2, 10) => "VPCLMULQDQ",
        // Leaf 7.0 EDX
        (7, 0, 3, 14) => "SERIALIZE",
        // Leaf 7.1 EAX
        (7, 1, 0, 4) => "AVX_VNNI",
        // Leaf 0xD.1 EAX
        (0xd, 1, 0, 0) => "XSAVEOPT",
        (0xd, 1, 0, 1) => "XSAVEC",
        // Leaf 0x8000_0001 ECX
        (0x8000_0001, 0, 2, 0) => "LAHF_SAHF (lahf/sahf in 64-bit mode)",
        (0x8000_0001, 0, 2, 5) => "LZCNT",
        (0x8000_0001, 0, 2, 8) => "PREFETCHW",
        // Leaf 0x8000_0001 EDX
        (0x8000_0001, 0, 3, 11) => "SYSCALL",
        (0x8000_0001, 0, 3, 20) => "NX (architectural page-protection flag)",
        (0x8000_0001, 0, 3, 27) => "RDTSCP",
        (0x8000_0001, 0, 3, 29) => "LM / X64",
        _ => return None,
    })
}

fn is_advertised(feature: F) -> bool {
    if matches!(
        feature,
        F::INTEL8086
            | F::INTEL8086_ONLY
            | F::INTEL186
            | F::INTEL286
            | F::INTEL286_ONLY
            | F::INTEL386
            | F::INTEL386_ONLY
            | F::INTEL386_A0_ONLY
            | F::INTEL486
            | F::INTEL486_A_ONLY
            | F::CPUID
            | F::MULTIBYTENOP
    ) {
        return true;
    }
    let Some((leaf, subleaf, reg, bit)) = feature_bit(feature) else {
        return false;
    };
    if leaf == 7 && subleaf > 0 && query(7, 0)[0] < subleaf {
        return false;
    }
    query(leaf, subleaf)[reg] & (1 << bit) != 0
}

#[derive(Clone, Copy)]
enum SynOp {
    Reg(Register),
    Mem(MemoryOperand),
    ImmU32(u32),
    ImmI32(i32),
    ImmU64(u64),
    Branch(u64),
}

fn synth_operand(kind: O, idx: usize, hi8: bool, avoid_ax: bool) -> Option<SynOp> {
    let gpr8 = if hi8 {
        [Register::AH, Register::CH, Register::DH, Register::BH][idx & 3]
    } else {
        [Register::AL, Register::CL, Register::DL, Register::BL][idx & 3]
    };
    let gpr16 = if avoid_ax && idx == 0 {
        Register::CX
    } else {
        [Register::AX, Register::CX, Register::DX, Register::BX][idx & 3]
    };
    let gpr32 = if avoid_ax && idx == 0 {
        Register::ECX
    } else {
        [Register::EAX, Register::ECX, Register::EDX, Register::EBX][idx & 3]
    };
    let gpr64 = if avoid_ax && idx == 0 {
        Register::RCX
    } else {
        [Register::RAX, Register::RCX, Register::RDX, Register::RBX][idx & 3]
    };
    let xmm = [Register::XMM0, Register::XMM1, Register::XMM2, Register::XMM3, Register::XMM4][idx % 5];
    let ymm = [Register::YMM0, Register::YMM1, Register::YMM2, Register::YMM3, Register::YMM4][idx % 5];
    let zmm = [Register::ZMM0, Register::ZMM1, Register::ZMM2, Register::ZMM3][idx & 3];
    let mm = [Register::MM0, Register::MM1, Register::MM2, Register::MM3][idx & 3];
    let kreg = [Register::K1, Register::K2, Register::K3, Register::K4][idx & 3];
    let mem = MemoryOperand::with_base_displ(Register::R8, 0x20);
    Some(match kind {
        O::None => return None,
        O::r8_opcode | O::r8_reg | O::r8_or_mem => SynOp::Reg(gpr8),
        O::r16_opcode | O::r16_reg | O::r16_reg_mem | O::r16_rm | O::r16_or_mem => SynOp::Reg(gpr16),
        O::r32_opcode | O::r32_reg | O::r32_reg_mem | O::r32_rm | O::r32_vvvv | O::r32_or_mem
        | O::r32_or_mem_mpx => SynOp::Reg(gpr32),
        O::r64_opcode | O::r64_reg | O::r64_reg_mem | O::r64_rm | O::r64_vvvv | O::r64_or_mem
        | O::r64_or_mem_mpx => SynOp::Reg(gpr64),
        O::xmm_reg | O::xmm_rm | O::xmm_vvvv | O::xmm_is4 | O::xmm_is5 | O::xmm_or_mem => SynOp::Reg(xmm),
        O::ymm_reg | O::ymm_rm | O::ymm_vvvv | O::ymm_is4 | O::ymm_is5 | O::ymm_or_mem => SynOp::Reg(ymm),
        O::zmm_reg | O::zmm_rm | O::zmm_vvvv | O::zmm_or_mem => SynOp::Reg(zmm),
        O::mm_reg | O::mm_rm | O::mm_or_mem => SynOp::Reg(mm),
        O::k_reg | O::k_rm | O::k_vvvv | O::kp1_reg => SynOp::Reg(kreg),
        O::al => SynOp::Reg(Register::AL),
        O::cl => SynOp::Reg(Register::CL),
        O::ax => SynOp::Reg(Register::AX),
        O::dx => SynOp::Reg(Register::DX),
        O::eax => SynOp::Reg(Register::EAX),
        O::rax => SynOp::Reg(Register::RAX),
        O::st0 => SynOp::Reg(Register::ST0),
        O::sti_opcode => SynOp::Reg(Register::ST1),
        O::es => SynOp::Reg(Register::ES),
        O::cs => SynOp::Reg(Register::CS),
        O::ss => SynOp::Reg(Register::SS),
        O::ds => SynOp::Reg(Register::DS),
        O::fs => SynOp::Reg(Register::FS),
        O::gs => SynOp::Reg(Register::GS),
        O::seg_reg => SynOp::Reg(Register::DS),
        O::cr_reg => SynOp::Reg(Register::CR0),
        O::dr_reg => SynOp::Reg(Register::DR0),
        O::tr_reg => SynOp::Reg(Register::TR0),
        O::bnd_reg | O::bnd_or_mem_mpx => SynOp::Reg(Register::BND0),
        O::tmm_reg | O::tmm_rm | O::tmm_vvvv => SynOp::Reg(Register::TMM0),
        O::mem | O::mem_mpx | O::mem_mib | O::mem_vsib32x | O::mem_vsib64x | O::mem_vsib32y
        | O::mem_vsib64y | O::mem_vsib32z | O::mem_vsib64z => match kind {
            O::mem_vsib32x | O::mem_vsib64x => {
                SynOp::Mem(MemoryOperand::with_base_index_scale_displ_size(Register::R8, Register::XMM5, 4, 0, 1))
            }
            O::mem_vsib32y | O::mem_vsib64y => {
                SynOp::Mem(MemoryOperand::with_base_index_scale_displ_size(Register::R8, Register::YMM5, 4, 0, 1))
            }
            O::mem_vsib32z | O::mem_vsib64z => {
                SynOp::Mem(MemoryOperand::with_base_index_scale_displ_size(Register::R8, Register::ZMM5, 4, 0, 1))
            }
            _ => SynOp::Mem(mem),
        },
        O::mem_offs => SynOp::Mem(MemoryOperand::with_displ(0x1000, 8)),
        O::seg_rDI => SynOp::Mem(MemoryOperand::with_base(Register::RDI)),
        O::seg_rSI => SynOp::Mem(MemoryOperand::with_base(Register::RSI)),
        O::seg_rBX_al => SynOp::Mem(MemoryOperand::with_base_index(Register::RBX, Register::AL)),
        O::es_rDI => SynOp::Mem(MemoryOperand::with_base(Register::RDI)),
        O::imm4_m2z | O::imm8 | O::imm8_const_1 | O::imm16 | O::imm32 => {
            SynOp::ImmU32(if kind == O::imm8_const_1 { 1 } else { 3 })
        }
        O::imm8sex16 | O::imm8sex32 | O::imm8sex64 | O::imm32sex64 => SynOp::ImmI32(3),
        O::imm64 => SynOp::ImmU64(0x1122_3344_5566_7788),
        O::br16_1 | O::br32_1 | O::br64_1 | O::br16_2 | O::br32_4 | O::br64_4 => {
            SynOp::Branch(0x1040)
        }
        O::xbegin_2 | O::xbegin_4 => return None,
        _ => return None,
    })
}

fn is_rm_kind(kind: O) -> bool {
    matches!(
        kind,
        O::r8_or_mem
            | O::r16_or_mem
            | O::r32_or_mem
            | O::r32_or_mem_mpx
            | O::r64_or_mem
            | O::r64_or_mem_mpx
            | O::xmm_or_mem
            | O::ymm_or_mem
            | O::zmm_or_mem
            | O::mm_or_mem
    )
}

fn build_variants(code: Code) -> Vec<Instruction> {
    let info = code.op_code();
    if !info.is_instruction() || !info.is_available_in_mode(64) {
        return Vec::new();
    }
    let n = info.op_count() as usize;
    let kinds: Vec<O> = (0..n).map(|i| info.op_kind(i as u32)).collect();
    let avoid_ax = n == 2 && matches!(kinds[1], O::ax | O::eax | O::rax);
    let rm_positions: Vec<usize> =
        kinds.iter().enumerate().filter(|&(_, &k)| is_rm_kind(k)).map(|(i, _)| i).collect();
    let has_r8 = kinds.iter().any(|&k| matches!(k, O::r8_opcode | O::r8_reg | O::r8_or_mem));
    let mut configs: Vec<(Option<usize>, bool)> = vec![(None, false)];
    if has_r8 {
        configs.push((None, true));
    }
    for pos in rm_positions {
        configs.push((Some(pos), false));
    }
    let mut out = Vec::new();
    for (mem_pos, hi8) in configs {
        let mut ops = Vec::with_capacity(n);
        let mut ok = true;
        for (i, &k) in kinds.iter().enumerate() {
            let op = if mem_pos == Some(i) {
                Some(SynOp::Mem(MemoryOperand::with_base_displ(Register::R8, 0x20)))
            } else {
                synth_operand(k, i, hi8, avoid_ax)
            };
            match op {
                Some(o) => ops.push(o),
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            continue;
        }
        let Some(ins) = make_ins(code, &ops) else { continue };
        let mut enc = Encoder::new(64);
        if enc.encode(&ins, 0x1000).is_err() {
            continue;
        }
        let bytes = enc.take_buffer();
        let dec = Decoder::with_ip(64, &bytes, 0x1000, DecoderOptions::NONE).decode();
        if !dec.is_invalid() && dec.code() == code {
            out.push(dec);
        }
    }
    out
}

fn make_ins(code: Code, ops: &[SynOp]) -> Option<Instruction> {
    use SynOp::*;
    let res = match ops {
        [] => Ok(Instruction::with(code)),
        [Reg(a)] => Instruction::with1(code, *a),
        [Mem(a)] => Instruction::with1(code, *a),
        [ImmU32(a)] => Instruction::with1(code, *a),
        [ImmI32(a)] => Instruction::with1(code, *a),
        [Branch(a)] => Instruction::with_branch(code, *a),
        [Reg(a), Reg(b)] => Instruction::with2(code, *a, *b),
        [Reg(a), Mem(b)] => Instruction::with2(code, *a, *b),
        [Mem(a), Reg(b)] => Instruction::with2(code, *a, *b),
        [Reg(a), ImmU32(b)] => Instruction::with2(code, *a, *b),
        [Reg(a), ImmI32(b)] => Instruction::with2(code, *a, *b),
        [Reg(a), ImmU64(b)] => Instruction::with2(code, *a, *b),
        [Mem(a), ImmU32(b)] => Instruction::with2(code, *a, *b),
        [Mem(a), ImmI32(b)] => Instruction::with2(code, *a, *b),
        [Reg(a), Reg(b), Reg(c)] => Instruction::with3(code, *a, *b, *c),
        [Reg(a), Reg(b), Mem(c)] => Instruction::with3(code, *a, *b, *c),
        [Reg(a), Mem(b), Reg(c)] => Instruction::with3(code, *a, *b, *c),
        [Mem(a), Reg(b), Reg(c)] => Instruction::with3(code, *a, *b, *c),
        [Reg(a), Reg(b), ImmU32(c)] => Instruction::with3(code, *a, *b, *c),
        [Reg(a), Reg(b), ImmI32(c)] => Instruction::with3(code, *a, *b, *c),
        [Reg(a), Mem(b), ImmU32(c)] => Instruction::with3(code, *a, *b, *c),
        [Reg(a), Mem(b), ImmI32(c)] => Instruction::with3(code, *a, *b, *c),
        [Mem(a), Reg(b), ImmU32(c)] => Instruction::with3(code, *a, *b, *c),
        [Reg(a), Reg(b), Reg(c), Reg(d)] => Instruction::with4(code, *a, *b, *c, *d),
        [Reg(a), Reg(b), Mem(c), Reg(d)] => Instruction::with4(code, *a, *b, *c, *d),
        [Reg(a), Reg(b), Reg(c), ImmU32(d)] => Instruction::with4(code, *a, *b, *c, *d),
        [Reg(a), Reg(b), Mem(c), ImmU32(d)] => Instruction::with4(code, *a, *b, *c, *d),
        _ => return None,
    };
    res.ok()
}

/// Narrow allowlist of advertised extension forms owned by parallel slices (Slice 3: MMX /
/// legacy SIMD / AVX2 gathers; Slice 4: system / XSAVE / MXCSR / CPUID) or ring-0 privileged
/// traps (`xsetbv`). Every other advertised ISA extension form MUST lower to `Arm64`.
fn is_tracked_non_arm64_extension_form(ins: &Instruction) -> bool {
    use iced_x86::Mnemonic as M;
    let low = instruction_lowering(ins);
    if low == InstructionLowering::MmxDispatch {
        // Slice 3 (`cpu/aot-mmx-legacy-simd-gather`): MMX physical-x87 aliased ops.
        return true;
    }
    match ins.mnemonic() {
        // Slice 3 (`cpu/aot-mmx-legacy-simd-gather`): MMX EMMS, legacy SIMD fallbacks & AVX2 gathers.
        M::Emms
        | M::Phaddsw
        | M::Vphaddsw
        | M::Phsubsw
        | M::Vphsubsw
        | M::Phminposuw
        | M::Vphminposuw
        | M::Dppd
        | M::Vdppd
        | M::Mpsadbw
        | M::Vmpsadbw
        | M::Pcmpestri64
        | M::Vpcmpestri64
        | M::Pcmpestrm64
        | M::Vpcmpestrm64
        | M::Maskmovdqu
        | M::Vmaskmovdqu
        | M::Vpgatherdd
        | M::Vpgatherdq
        | M::Vpgatherqd
        | M::Vpgatherqq
        | M::Vgatherdps
        | M::Vgatherdpd
        | M::Vgatherqps
        | M::Vgatherqpd => true,
        // Slice 4 (`cpu/aot-sys-state-ops`): XSAVE/FXSR/MXCSR/CPUID.
        M::Cpuid
        | M::Fxsave
        | M::Fxsave64
        | M::Fxrstor
        | M::Fxrstor64
        | M::Xsave
        | M::Xsave64
        | M::Xsaveopt
        | M::Xsaveopt64
        | M::Xsavec
        | M::Xsavec64
        | M::Xrstor
        | M::Xrstor64
        | M::Xgetbv
        | M::Ldmxcsr
        | M::Vldmxcsr => true,
        // Ring-0 privileged instruction sharing the `XSAVE` CPUID bit (always #GP(0) in CPL3).
        M::Xsetbv => true,
        _ => false,
    }
}

/// Extensions that must have **zero** non-`Arm64` 64-bit forms whenever advertised.
const FULLY_NATIVE_EXTENSIONS: &[F] = &[
    F::CMOV,
    F::CX8,
    F::TSC,
    F::SSE3,
    F::PCLMULQDQ,
    F::FMA,
    F::CMPXCHG16B,
    F::MOVBE,
    F::POPCNT,
    F::AES,
    F::F16C,
    F::RDRAND,
    F::BMI1,
    F::BMI2,
    F::RDSEED,
    F::ADX,
    F::SHA,
    F::GFNI,
    F::VAES,
    F::VPCLMULQDQ,
    F::SERIALIZE,
    F::AVX_VNNI,
    F::LZCNT,
    F::PREFETCHW,
    F::RDTSCP,
    F::MULTIBYTENOP,
];

/// Extensions that are intentionally unimplemented and MUST remain unadvertised.
const UNADVERTISED_EXTENSIONS: &[F] = &[
    F::FSGSBASE,
    F::WAITPKG,
    F::PKU,
    F::AVX_IFMA,
    F::AVX_VNNI_INT8,
    F::AVX_NE_CONVERT,
    F::AVX_VNNI_INT16,
    F::SHA512,
    F::SM3,
    F::SM4,
    F::AMX_BF16,
    F::AMX_TILE,
    F::AMX_INT8,
    F::AMX_FP16,
    F::AMX_COMPLEX,
    F::AVX512F,
    F::AVX512DQ,
    F::AVX512_IFMA,
    F::AVX512PF,
    F::AVX512ER,
    F::AVX512CD,
    F::AVX512BW,
    F::AVX512VL,
    F::AVX512_VBMI,
    F::AVX512_VBMI2,
    F::AVX512_VNNI,
    F::AVX512_BITALG,
    F::AVX512_VPOPCNTDQ,
    F::AVX512_4VNNIW,
    F::AVX512_4FMAPS,
    F::AVX512_VP2INTERSECT,
    F::AVX512_FP16,
    F::AVX512_BF16,
    F::XSAVES,
    F::VMX,
    F::SMX,
    F::SGX1,
    F::RTM,
    F::HLE,
    F::MPX,
    F::INVPCID,
    F::SSE4A,
    F::XOP,
    F::FMA4,
    F::TBM,
    F::D3NOW,
    F::D3NOWEXT,
];

#[test]
fn honest_cpuid_every_advertised_bit_is_mapped_and_lowered() {
    // 1. Walk every feature register on every CPUID leaf/subleaf and verify no unknown bit is set.
    let feature_regs: &[(u32, u32, &[usize])] = &[
        (1, 0, &[2, 3]),
        (7, 0, &[1, 2, 3]),
        (7, 1, &[0, 1, 2, 3]),
        (0xd, 1, &[0]),
        (0x8000_0001, 0, &[2, 3]),
        (0x8000_0008, 0, &[1]),
    ];
    let mut unknown_bits = Vec::new();
    for &(leaf, subleaf, regs) in feature_regs {
        let vals = query(leaf, subleaf);
        for &reg in regs {
            let mut mask = vals[reg];
            while mask != 0 {
                let bit = mask.trailing_zeros();
                mask &= mask - 1;
                if known_advertised_bit(leaf, subleaf, reg, bit).is_none() {
                    unknown_bits.push(format!("leaf {leaf:#x}.{subleaf} reg[{reg}] bit {bit}"));
                }
            }
        }
    }
    assert!(
        unknown_bits.is_empty(),
        "CPUID advertises unmapped feature bits: {unknown_bits:?}"
    );
    // Leaf 7 max subleaf must match subleaf 1 (AVX-VNNI) and subleaf 2+ must be zero.
    assert_eq!(query(7, 0)[0], 1);
    assert_eq!(query(7, 2), [0; 4]);

    // 2. Walk every 64-bit-mode `iced_x86::Code` form whose required `CpuidFeature`s are all
    //    advertised. Ignore baseline 8086/186/286/386/486/X64/SYSCALL core-ISA codes here (those
    //    include ring-0 system instructions and Slice 2 GP edge forms); check EVERY ISA extension
    //    bit strictly!
    let baseline = |f: F| {
        matches!(
            f,
            F::INTEL8086
                | F::INTEL8086_ONLY
                | F::INTEL186
                | F::INTEL286
                | F::INTEL286_ONLY
                | F::INTEL386
                | F::INTEL386_ONLY
                | F::INTEL386_A0_ONLY
                | F::INTEL486
                | F::INTEL486_A_ONLY
                | F::X64
                | F::SYSCALL
        )
    };
    let mut violations: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut checked_by_feat: BTreeMap<F, usize> = BTreeMap::new();

    for code in Code::values() {
        let info = code.op_code();
        if !info.is_instruction() || !info.is_available_in_mode(64) {
            continue;
        }
        let variants = build_variants(code);
        for ins in variants {
            let feats = ins.cpuid_features();
            if !feats.iter().all(|&f| is_advertised(f)) {
                continue;
            }
            if feats.iter().all(|&f| baseline(f)) {
                continue;
            }
            for &f in feats {
                *checked_by_feat.entry(f).or_default() += 1;
            }
            let low = instruction_lowering(&ins);
            if low == InstructionLowering::Arm64 {
                continue;
            }
            if feats.iter().any(|f| FULLY_NATIVE_EXTENSIONS.contains(f))
                || !is_tracked_non_arm64_extension_form(&ins)
            {
                violations
                    .entry(format!("{feats:?}"))
                    .or_default()
                    .push(format!("{:?} ({ins}) -> {low:?}", ins.code()));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "Advertised CPUID extension forms do not lower to Arm64:\n{violations:#?}"
    );

    // Confirm every advertised fully-native extension actually had forms checked (> 0).
    let expected_checked: BTreeSet<F> = FULLY_NATIVE_EXTENSIONS
        .iter()
        .copied()
        .filter(|&f| is_advertised(f))
        .collect();
    for f in expected_checked {
        assert!(
            checked_by_feat.get(&f).copied().unwrap_or(0) > 0,
            "Expected at least one synthesized 64-bit form for advertised feature {f:?}"
        );
    }
}

#[test]
fn honest_cpuid_unimplemented_extensions_and_avx512_stay_unadvertised() {
    for &feat in UNADVERTISED_EXTENSIONS {
        assert!(
            !is_advertised(feat),
            "Unimplemented extension {feat:?} must NOT be advertised by cpuid::query"
        );
    }
    // OSPKE (Leaf 7.0 ECX bit 4) is also unadvertised.
    assert_eq!(query(7, 0)[2] & (1 << 4), 0, "OSPKE must stay clear");
    // XCR0 opmask (bit 5), ZMM_Hi256 (bit 6), Hi16_ZMM (bit 7) must all be clear.
    assert_eq!(XCR0 & ((1 << 5) | (1 << 6) | (1 << 7)), 0);
}

#[test]
fn honest_cpuid_xcr0_and_xgetbv_consistency() {
    let [_, _, c1, d1] = query(1, 0);
    // AVX requires XSAVE + OSXSAVE + SSE + SSE2 + XCR0[2:0] == 0b111.
    assert_ne!(d1 & (1 << 25), 0, "SSE");
    assert_ne!(d1 & (1 << 26), 0, "SSE2");
    assert_ne!(c1 & (1 << 26), 0, "XSAVE");
    assert_ne!(c1 & (1 << 27), 0, "OSXSAVE");
    assert_ne!(c1 & (1 << 28), 0, "AVX");
    assert_eq!(XCR0, 7);
    assert_eq!(YMM_OFFSET, 576);
    assert_eq!(XSAVE_SIZE, 832);
    assert_eq!(query(0xd, 0), [7, 832, 832, 0]);
    assert_eq!(query(0xd, 1), [3, 832, 0, 0]);
    assert_eq!(query(0xd, 2), [256, 576, 0, 0]);
    assert_eq!(unsafe { win32_rt::kernel32::sysinfo::GetEnabledXStateFeatures() }, 7);

    // Execute `xgetbv` with ECX = 0 and verify EDX:EAX == XCR0 (7).
    let mut cpu = Cpu::new_zeroed();
    cpu.gpr[1] = 0; // ECX = 0
    let xgetbv_ins = Decoder::with_ip(64, &[0x0f, 0x01, 0xd0], 0x1000, DecoderOptions::NONE).decode();
    unsafe { cpu.exec_generic_ins(&xgetbv_ins) };
    assert_eq!(cpu.gpr[0], XCR0 as u64);
    assert_eq!(cpu.gpr[2], 0);
}

#[test]
fn honest_cpuid_host_feature_gating() {
    // Masking HostFeatures::AES must hide BOTH AES (1.0 ECX:25) and VAES (7.0 ECX:9).
    features::without_features(H::AES, || {
        assert_eq!(query(1, 0)[2] & (1 << 25), 0);
        assert_eq!(query(7, 0)[2] & (1 << 9), 0);
        // Baseline NEON extensions remain advertised.
        assert_ne!(query(7, 0)[2] & (1 << 8), 0, "GFNI uses baseline ARMv8.0 NEON");
        assert_ne!(query(7, 1)[0] & (1 << 4), 0, "AVX-VNNI uses baseline ARMv8.0 NEON");
        assert_ne!(query(7, 0)[3] & (1 << 14), 0, "SERIALIZE uses baseline DSB ISH + ISB");
        assert_ne!(query(1, 0)[2] & (1 << 30), 0, "RDRAND uses OS getentropy");
        assert_ne!(query(7, 0)[1] & (1 << 18), 0, "RDSEED uses OS getentropy");
    });
    // Masking HostFeatures::PMULL must hide BOTH PCLMULQDQ (1.0 ECX:1) and VPCLMULQDQ (7.0 ECX:10).
    features::without_features(H::PMULL, || {
        assert_eq!(query(1, 0)[2] & (1 << 1), 0);
        assert_eq!(query(7, 0)[2] & (1 << 10), 0);
    });
    // Masking HostFeatures::CRC32 hides SSE4.2 (1.0 ECX:20).
    features::without_features(H::CRC32, || {
        assert_eq!(query(1, 0)[2] & (1 << 20), 0);
    });
    // Masking either SHA1 or SHA256 hides SHA-NI (7.0 EBX:29).
    features::without_features(H::SHA1, || {
        assert_eq!(query(7, 0)[1] & (1 << 29), 0);
    });
    features::without_features(H::SHA256, || {
        assert_eq!(query(7, 0)[1] & (1 << 29), 0);
    });
}
