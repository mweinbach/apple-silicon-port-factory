//! Exhaustive x86-64 ISA coverage ledger (`cargo opsdb -- isa-coverage`).
//!
//! Enumerates every `iced_x86::Code` variant, records excluded codes with explicit reasons
//! (non-instructions, 16/32-bit-only, 3DNow!, XOP/TBM, Knights MVEX, privileged/VMX/SGX),
//! synthesizes representative `reg` and `mem` instructions from `OpCodeInfo` operand kinds,
//! and classifies each form through `x86_runtime::emu::instruction_lowering`.
//!
//! Produces:
//! - Per-extension (`CpuidFeature`) counts & ARM64 percentages
//! - Per-catalog-family (`ops_db::classify_instruction`) counts & ARM64 percentages
//! - `advertised_but_unimplemented`: every guest-advertised `CpuidFeature` that has any
//!   non-`Arm64` (non-`Boundary`) form reachable under the guest CPUID profile
//! - Deterministic `db/isa_coverage.tsv` ledger checked by `cargo opsdb -- doctor`

use crate::json_output::{JsonObj, JsonValue};
use crate::ops_db::classify_instruction;
use crate::readiness::cpuid_advertised;
use anyhow::{Context, Result};
use iced_x86::{
    Code, CpuidFeature, Decoder, DecoderOptions, Encoder, EncodingKind, Instruction, MemoryOperand,
    Mnemonic, OpCodeInfo, OpCodeOperandKind, OpKind, Register,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use x86_runtime::emu::{instruction_lowering, InstructionLowering};

const SYNTH_IP: u64 = 0x0000_0001_4000_1000;

/// Lowering classification for one synthesized x86-64 instruction form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LoweringClass {
    Arm64,
    FastInterpreter,
    MmxDispatch,
    GenericDispatch,
    Boundary,
}

impl LoweringClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Arm64 => "Arm64",
            Self::FastInterpreter => "FastInterpreter",
            Self::MmxDispatch => "MmxDispatch",
            Self::GenericDispatch => "GenericDispatch",
            Self::Boundary => "Boundary",
        }
    }
}

/// Explicit reason why an `iced_x86::Code` is excluded from the 64-bit user-mode ledger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExclusionReason {
    NonInstruction,
    Mode1632Only,
    D3Now,
    XopTbm,
    KnightsMvex,
    PrivilegedVmxSgx,
}

impl ExclusionReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NonInstruction => "non_instruction",
            Self::Mode1632Only => "mode16_32_only",
            Self::D3Now => "d3now",
            Self::XopTbm => "xop_tbm",
            Self::KnightsMvex => "knights_mvex",
            Self::PrivilegedVmxSgx => "privileged_vmx_sgx",
        }
    }
}

/// Counts across the five lowering classes.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ClassCounts {
    pub total: usize,
    pub arm64: usize,
    pub fast_interpreter: usize,
    pub mmx_dispatch: usize,
    pub generic_dispatch: usize,
    pub boundary: usize,
}

impl ClassCounts {
    fn record(&mut self, class: LoweringClass) {
        self.total += 1;
        match class {
            LoweringClass::Arm64 => self.arm64 += 1,
            LoweringClass::FastInterpreter => self.fast_interpreter += 1,
            LoweringClass::MmxDispatch => self.mmx_dispatch += 1,
            LoweringClass::GenericDispatch => self.generic_dispatch += 1,
            LoweringClass::Boundary => self.boundary += 1,
        }
    }

    pub fn non_arm64_non_boundary(&self) -> usize {
        self.fast_interpreter + self.mmx_dispatch + self.generic_dispatch
    }

    pub fn arm64_pct(&self) -> f64 {
        let denom = self.total.saturating_sub(self.boundary);
        if denom == 0 {
            100.0
        } else {
            (self.arm64 as f64) * 100.0 / (denom as f64)
        }
    }

    fn to_json(&self) -> JsonValue {
        JsonObj::new()
            .u64("total", self.total as u64)
            .u64("arm64", self.arm64 as u64)
            .u64("fast_interpreter", self.fast_interpreter as u64)
            .u64("mmx_dispatch", self.mmx_dispatch as u64)
            .u64("generic_dispatch", self.generic_dispatch as u64)
            .u64("boundary", self.boundary as u64)
            .f64("arm64_pct", (self.arm64_pct() * 100.0).round() / 100.0)
            .build()
    }
}

/// One classified instruction form in the ledger.
#[derive(Debug, Clone)]
pub struct IsaFormRow {
    pub code: Code,
    pub variant: &'static str,
    pub mnemonic: String,
    pub encoding: EncodingKind,
    pub cpuid_features: Vec<CpuidFeature>,
    pub catalog_family: &'static str,
    pub lowering: LoweringClass,
    pub advertised: bool,
    pub roundtrip_ok: bool,
}

/// One excluded `iced_x86::Code` with its explicit reason.
#[derive(Debug, Clone)]
pub struct ExcludedCode {
    pub code: Code,
    pub mnemonic: String,
    pub reason: ExclusionReason,
    pub cpuid_features: Vec<CpuidFeature>,
}

/// Summary of an advertised `CpuidFeature` that still has non-Arm64 forms.
#[derive(Debug, Clone)]
pub struct AdvertisedGap {
    pub feature: CpuidFeature,
    pub counts: ClassCounts,
    pub sample_codes: Vec<String>,
}

/// Complete ISA coverage ledger report.
#[derive(Debug, Clone)]
pub struct IsaCoverageReport {
    pub total_codes: usize,
    pub included_codes: usize,
    pub excluded: Vec<ExcludedCode>,
    pub excluded_by_reason: BTreeMap<ExclusionReason, usize>,
    pub forms: Vec<IsaFormRow>,
    pub totals: ClassCounts,
    pub by_cpuid: BTreeMap<CpuidFeature, ClassCounts>,
    pub by_family: BTreeMap<String, ClassCounts>,
    pub advertised_features: Vec<CpuidFeature>,
    pub advertised_but_unimplemented: Vec<AdvertisedGap>,
    pub synthesis_roundtrip_failures: usize,
}

fn is_user_boundary_mnemonic(mn: Mnemonic) -> bool {
    matches!(
        mn,
        Mnemonic::Int3
            | Mnemonic::Int1
            | Mnemonic::Int
            | Mnemonic::Ud0
            | Mnemonic::Ud1
            | Mnemonic::Ud2
            | Mnemonic::Syscall
            | Mnemonic::Sysenter
    )
}

fn is_baseline_x64_feature(f: CpuidFeature) -> bool {
    use CpuidFeature::*;
    matches!(
        f,
        INTEL8086 | INTEL186 | INTEL286 | INTEL386 | INTEL486 | CPUID | MULTIBYTENOP | PAUSE
    )
}

/// True if every required `CpuidFeature` of an instruction is available under the guest CPUID
/// profile (baseline x86-64 architectural features count as available when `X64` is advertised).
pub fn form_is_advertised(features: &[CpuidFeature]) -> bool {
    if features.is_empty() {
        return false;
    }
    let has_x64 = cpuid_advertised(CpuidFeature::X64);
    features
        .iter()
        .all(|&f| cpuid_advertised(f) || (has_x64 && is_baseline_x64_feature(f)))
}

fn exclusion_reason(op: &OpCodeInfo, cpuid: &[CpuidFeature]) -> Option<ExclusionReason> {
    if !op.is_instruction() {
        return Some(ExclusionReason::NonInstruction);
    }
    if !op.mode64() {
        return Some(ExclusionReason::Mode1632Only);
    }
    use CpuidFeature::*;
    if cpuid
        .iter()
        .any(|f| matches!(f, D3NOW | D3NOWEXT | CYRIX_D3NOW))
    {
        return Some(ExclusionReason::D3Now);
    }
    if op.encoding() == EncodingKind::XOP
        || cpuid.iter().any(|f| matches!(f, XOP | TBM | FMA4 | LWP))
    {
        return Some(ExclusionReason::XopTbm);
    }
    if op.encoding() == EncodingKind::MVEX
        || cpuid.iter().any(|f| {
            matches!(
                f,
                KNC | AVX512ER | AVX512PF | AVX512_4FMAPS | AVX512_4VNNIW | PREFETCHWT1
            )
        })
    {
        return Some(ExclusionReason::KnightsMvex);
    }
    if is_user_boundary_mnemonic(op.mnemonic()) {
        return None;
    }
    if op.is_privileged()
        || op.must_be_cpl0()
        || !op.cpl3()
        || op.is_input_output()
        || cpuid.iter().any(|f| {
            matches!(
                f,
                VMX | INVEPT
                    | INVVPID
                    | SGX1
                    | SVM
                    | SKINIT
                    | SKINIT_or_SVM
                    | SEV_ES
                    | SEV_SNP
                    | RMPQUERY
                    | INVLPGB
                    | TDX
                    | SMX
                    | SMM
                    | SMAP
                    | FRED
                    | LKGS
                    | MSR
                    | MSRLIST
                    | WRMSRNS
                    | ENQCMD
                    | PCONFIG
                    | TSE
                    | UDBG
                    | MOV_TR
                    | MONITOR
                    | MONITORX
                    | WBNOINVD
                    | MCOMMIT
                    | CL1INVMB
                    | PCOMMIT
                    | HRESET
                    | UINTR
                    | CET_SS
                    | CYRIX_FPU
                    | CYRIX_SMM
                    | CYRIX_SMINT
                    | CYRIX_SMINT_0F7E
                    | CYRIX_SHR
                    | CYRIX_DDI
                    | CYRIX_EMMI
                    | CYRIX_DMI
                    | CENTAUR_AIS
                    | PADLOCK_ACE
                    | PADLOCK_PHE
                    | PADLOCK_PMM
                    | PADLOCK_RNG
                    | PADLOCK_GMI
                    | PADLOCK_UNDOC
                    | IA64
                    | UMOV
                    | MPX
                    | OSS
            )
        })
    {
        return Some(ExclusionReason::PrivilegedVmxSgx);
    }
    None
}

fn is_reg_or_mem(kind: OpCodeOperandKind) -> bool {
    use OpCodeOperandKind as K;
    matches!(
        kind,
        K::r8_or_mem
            | K::r16_or_mem
            | K::r32_or_mem
            | K::r32_or_mem_mpx
            | K::r64_or_mem
            | K::r64_or_mem_mpx
            | K::mm_or_mem
            | K::xmm_or_mem
            | K::ymm_or_mem
            | K::zmm_or_mem
            | K::bnd_or_mem_mpx
            | K::k_or_mem
    )
}

fn is_mem_only(kind: OpCodeOperandKind) -> bool {
    use OpCodeOperandKind as K;
    matches!(
        kind,
        K::mem
            | K::mem_offs
            | K::mem_mpx
            | K::mem_mib
            | K::mem_vsib32x
            | K::mem_vsib64x
            | K::mem_vsib32y
            | K::mem_vsib64y
            | K::mem_vsib32z
            | K::mem_vsib64z
            | K::seg_rSI
            | K::es_rDI
            | K::seg_rDI
            | K::seg_rBX_al
            | K::sibmem
    )
}

fn gpr8(slot: usize) -> Register {
    const REGS: [Register; 5] = [
        Register::CL,
        Register::DL,
        Register::BL,
        Register::R8L,
        Register::R9L,
    ];
    REGS[slot % REGS.len()]
}

fn gpr16(slot: usize) -> Register {
    const REGS: [Register; 5] = [
        Register::CX,
        Register::DX,
        Register::R8W,
        Register::R9W,
        Register::R10W,
    ];
    REGS[slot % REGS.len()]
}

fn gpr32(slot: usize) -> Register {
    const REGS: [Register; 5] = [
        Register::ECX,
        Register::EDX,
        Register::R8D,
        Register::R9D,
        Register::R10D,
    ];
    REGS[slot % REGS.len()]
}

fn gpr64(slot: usize) -> Register {
    const REGS: [Register; 5] = [
        Register::RCX,
        Register::RDX,
        Register::R8,
        Register::R9,
        Register::R10,
    ];
    REGS[slot % REGS.len()]
}

fn xmm_reg(slot: usize) -> Register {
    const REGS: [Register; 5] = [
        Register::XMM1,
        Register::XMM2,
        Register::XMM3,
        Register::XMM4,
        Register::XMM5,
    ];
    REGS[slot % REGS.len()]
}

fn ymm_reg(slot: usize) -> Register {
    const REGS: [Register; 5] = [
        Register::YMM1,
        Register::YMM2,
        Register::YMM3,
        Register::YMM4,
        Register::YMM5,
    ];
    REGS[slot % REGS.len()]
}

fn zmm_reg(slot: usize) -> Register {
    const REGS: [Register; 5] = [
        Register::ZMM1,
        Register::ZMM2,
        Register::ZMM3,
        Register::ZMM4,
        Register::ZMM5,
    ];
    REGS[slot % REGS.len()]
}

fn mm_reg(slot: usize) -> Register {
    const REGS: [Register; 5] = [
        Register::MM0,
        Register::MM1,
        Register::MM2,
        Register::MM3,
        Register::MM4,
    ];
    REGS[slot % REGS.len()]
}

fn k_reg(slot: usize) -> Register {
    // Keep K1 reserved for opmask when required.
    const REGS: [Register; 5] = [
        Register::K2,
        Register::K3,
        Register::K4,
        Register::K5,
        Register::K6,
    ];
    REGS[slot % REGS.len()]
}

fn tmm_reg(slot: usize) -> Register {
    const REGS: [Register; 5] = [
        Register::TMM0,
        Register::TMM1,
        Register::TMM2,
        Register::TMM3,
        Register::TMM4,
    ];
    REGS[slot % REGS.len()]
}

fn set_mem(ins: &mut Instruction, slot: u32, mem: MemoryOperand) {
    ins.set_op_kind(slot, OpKind::Memory);
    ins.set_memory_base(mem.base);
    ins.set_memory_index(mem.index);
    ins.set_memory_index_scale(mem.scale);
    ins.set_memory_displacement64(mem.displacement as u64);
    ins.set_memory_displ_size(mem.displ_size);
    if mem.segment_prefix != Register::None {
        ins.set_segment_prefix(mem.segment_prefix);
    }
}

fn synthesize_one(op: &OpCodeInfo, use_mem: bool) -> (Instruction, bool) {
    let mut ins = Instruction::default();
    ins.set_code(op.code());
    ins.set_next_ip(SYNTH_IP);

    if op.require_op_mask_register() {
        ins.set_op_mask(Register::K1);
    }

    let addr_bits = if op.address_size() == 32 { 32 } else { 64 };
    let mem_base = if addr_bits == 32 {
        Register::EBX
    } else {
        Register::RBX
    };
    let std_mem = MemoryOperand::with_base_displ(mem_base, 0x10);
    let kinds = op.op_kinds();

    for (idx, &kind) in kinds.iter().enumerate() {
        let slot = idx as u32;
        use OpCodeOperandKind as K;
        match kind {
            K::None => {}
            K::r8_or_mem => {
                if use_mem {
                    set_mem(&mut ins, slot, std_mem);
                } else {
                    ins.set_op_kind(slot, OpKind::Register);
                    ins.set_op_register(slot, gpr8(idx));
                }
            }
            K::r16_or_mem => {
                if use_mem {
                    set_mem(&mut ins, slot, std_mem);
                } else {
                    ins.set_op_kind(slot, OpKind::Register);
                    ins.set_op_register(slot, gpr16(idx));
                }
            }
            K::r32_or_mem | K::r32_or_mem_mpx => {
                if use_mem {
                    set_mem(&mut ins, slot, std_mem);
                } else {
                    ins.set_op_kind(slot, OpKind::Register);
                    ins.set_op_register(slot, gpr32(idx));
                }
            }
            K::r64_or_mem | K::r64_or_mem_mpx => {
                if use_mem {
                    set_mem(&mut ins, slot, std_mem);
                } else {
                    ins.set_op_kind(slot, OpKind::Register);
                    ins.set_op_register(slot, gpr64(idx));
                }
            }
            K::mm_or_mem => {
                if use_mem {
                    set_mem(&mut ins, slot, std_mem);
                } else {
                    ins.set_op_kind(slot, OpKind::Register);
                    ins.set_op_register(slot, mm_reg(idx));
                }
            }
            K::xmm_or_mem => {
                if use_mem {
                    set_mem(&mut ins, slot, std_mem);
                } else {
                    ins.set_op_kind(slot, OpKind::Register);
                    ins.set_op_register(slot, xmm_reg(idx));
                }
            }
            K::ymm_or_mem => {
                if use_mem {
                    set_mem(&mut ins, slot, std_mem);
                } else {
                    ins.set_op_kind(slot, OpKind::Register);
                    ins.set_op_register(slot, ymm_reg(idx));
                }
            }
            K::zmm_or_mem => {
                if use_mem {
                    set_mem(&mut ins, slot, std_mem);
                } else {
                    ins.set_op_kind(slot, OpKind::Register);
                    ins.set_op_register(slot, zmm_reg(idx));
                }
            }
            K::bnd_or_mem_mpx => {
                if use_mem {
                    set_mem(&mut ins, slot, std_mem);
                } else {
                    ins.set_op_kind(slot, OpKind::Register);
                    ins.set_op_register(
                        slot,
                        if idx == 0 {
                            Register::BND0
                        } else {
                            Register::BND1
                        },
                    );
                }
            }
            K::k_or_mem => {
                if use_mem {
                    set_mem(&mut ins, slot, std_mem);
                } else {
                    ins.set_op_kind(slot, OpKind::Register);
                    ins.set_op_register(slot, k_reg(idx));
                }
            }
            K::mem | K::mem_mpx | K::mem_mib => set_mem(&mut ins, slot, std_mem),
            K::mem_offs => {
                let displ_size = if addr_bits == 32 { 4 } else { 8 };
                set_mem(
                    &mut ins,
                    slot,
                    MemoryOperand::with_displ(0x1000_2000, displ_size),
                );
            }
            K::sibmem => {
                set_mem(
                    &mut ins,
                    slot,
                    MemoryOperand::with_base_index(mem_base, Register::R11),
                );
            }
            K::mem_vsib32x | K::mem_vsib64x => {
                set_mem(
                    &mut ins,
                    slot,
                    MemoryOperand::with_base_index_scale_displ_size(
                        mem_base,
                        Register::XMM6,
                        4,
                        0,
                        0,
                    ),
                );
            }
            K::mem_vsib32y | K::mem_vsib64y => {
                set_mem(
                    &mut ins,
                    slot,
                    MemoryOperand::with_base_index_scale_displ_size(
                        mem_base,
                        Register::YMM6,
                        4,
                        0,
                        0,
                    ),
                );
            }
            K::mem_vsib32z | K::mem_vsib64z => {
                set_mem(
                    &mut ins,
                    slot,
                    MemoryOperand::with_base_index_scale_displ_size(
                        mem_base,
                        Register::ZMM6,
                        4,
                        0,
                        0,
                    ),
                );
            }
            K::r8_reg | K::r8_opcode => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, gpr8(idx));
            }
            K::r16_reg | K::r16_reg_mem | K::r16_rm | K::r16_opcode => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, gpr16(idx));
            }
            K::r32_reg | K::r32_reg_mem | K::r32_rm | K::r32_opcode | K::r32_vvvv => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, gpr32(idx));
            }
            K::r64_reg | K::r64_reg_mem | K::r64_rm | K::r64_opcode | K::r64_vvvv => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, gpr64(idx));
            }
            K::seg_reg => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::DS);
            }
            K::k_reg | K::k_rm | K::k_vvvv => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, k_reg(idx));
            }
            K::kp1_reg => {
                // Even register pair (K2/K3).
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::K2);
            }
            K::mm_reg | K::mm_rm => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, mm_reg(idx));
            }
            K::xmm_reg | K::xmm_rm | K::xmm_vvvv | K::xmm_is4 | K::xmm_is5 => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, xmm_reg(idx));
            }
            K::xmmp3_vvvv => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::XMM4);
            }
            K::ymm_reg | K::ymm_rm | K::ymm_vvvv | K::ymm_is4 | K::ymm_is5 => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, ymm_reg(idx));
            }
            K::zmm_reg | K::zmm_rm | K::zmm_vvvv => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, zmm_reg(idx));
            }
            K::zmmp3_vvvv => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::ZMM4);
            }
            K::tmm_reg | K::tmm_rm | K::tmm_vvvv => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, tmm_reg(idx));
            }
            K::cr_reg => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::CR0);
            }
            K::dr_reg => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::DR0);
            }
            K::tr_reg => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::TR0);
            }
            K::bnd_reg => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(
                    slot,
                    if idx == 0 {
                        Register::BND0
                    } else {
                        Register::BND1
                    },
                );
            }
            K::es => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::ES);
            }
            K::cs => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::CS);
            }
            K::ss => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::SS);
            }
            K::ds => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::DS);
            }
            K::fs => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::FS);
            }
            K::gs => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::GS);
            }
            K::al => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::AL);
            }
            K::cl => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::CL);
            }
            K::ax => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::AX);
            }
            K::dx => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::DX);
            }
            K::eax => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::EAX);
            }
            K::rax => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::RAX);
            }
            K::st0 => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::ST0);
            }
            K::sti_opcode => {
                ins.set_op_kind(slot, OpKind::Register);
                ins.set_op_register(slot, Register::ST1);
            }
            K::imm4_m2z | K::imm8 => {
                let had_prior_imm = kinds[..idx].iter().any(|&k| {
                    matches!(
                        k,
                        K::imm4_m2z
                            | K::imm8
                            | K::imm8_const_1
                            | K::imm8sex16
                            | K::imm8sex32
                            | K::imm8sex64
                            | K::imm16
                            | K::imm32
                            | K::imm32sex64
                            | K::imm64
                    )
                });
                if had_prior_imm {
                    ins.set_op_kind(slot, OpKind::Immediate8_2nd);
                    ins.set_immediate8_2nd(1);
                } else {
                    ins.set_op_kind(slot, OpKind::Immediate8);
                    ins.set_immediate8(1);
                }
            }
            K::imm8_const_1 => {
                ins.set_op_kind(slot, OpKind::Immediate8);
                ins.set_immediate8(1);
            }
            K::imm8sex16 => {
                ins.set_op_kind(slot, OpKind::Immediate8to16);
                ins.set_immediate8to16(1);
            }
            K::imm8sex32 => {
                ins.set_op_kind(slot, OpKind::Immediate8to32);
                ins.set_immediate8to32(1);
            }
            K::imm8sex64 => {
                ins.set_op_kind(slot, OpKind::Immediate8to64);
                ins.set_immediate8to64(1);
            }
            K::imm16 => {
                ins.set_op_kind(slot, OpKind::Immediate16);
                ins.set_immediate16(1);
            }
            K::imm32 => {
                ins.set_op_kind(slot, OpKind::Immediate32);
                ins.set_immediate32(1);
            }
            K::imm32sex64 => {
                ins.set_op_kind(slot, OpKind::Immediate32to64);
                ins.set_immediate32to64(1);
            }
            K::imm64 => {
                ins.set_op_kind(slot, OpKind::Immediate64);
                ins.set_immediate64(0x1122_3344_5566_7788);
            }
            K::seg_rSI => {
                let k = if addr_bits == 32 {
                    OpKind::MemorySegESI
                } else {
                    OpKind::MemorySegRSI
                };
                ins.set_op_kind(slot, k);
            }
            K::es_rDI => {
                let k = if addr_bits == 32 {
                    OpKind::MemoryESEDI
                } else {
                    OpKind::MemoryESRDI
                };
                ins.set_op_kind(slot, k);
            }
            K::seg_rDI => {
                let k = if addr_bits == 32 {
                    OpKind::MemorySegEDI
                } else {
                    OpKind::MemorySegRDI
                };
                ins.set_op_kind(slot, k);
            }
            K::seg_rBX_al => {
                let base = if addr_bits == 32 {
                    Register::EBX
                } else {
                    Register::RBX
                };
                set_mem(
                    &mut ins,
                    slot,
                    MemoryOperand::with_base_index(base, Register::AL),
                );
            }
            K::br16_1 | K::br16_2 => {
                ins.set_op_kind(slot, OpKind::NearBranch16);
                ins.set_near_branch16(0x1020);
            }
            K::br32_1 | K::br32_4 => {
                ins.set_op_kind(slot, OpKind::NearBranch32);
                ins.set_near_branch32((SYNTH_IP as u32).wrapping_add(0x20));
            }
            K::br64_1 | K::br64_4 | K::xbegin_2 | K::xbegin_4 => {
                ins.set_op_kind(slot, OpKind::NearBranch64);
                ins.set_near_branch64(SYNTH_IP.wrapping_add(0x20));
            }
            K::farbr2_2 => {
                ins.set_op_kind(slot, OpKind::FarBranch16);
                ins.set_far_branch_selector(0x33);
                ins.set_far_branch16(0x1020);
            }
            K::farbr4_2 => {
                ins.set_op_kind(slot, OpKind::FarBranch32);
                ins.set_far_branch_selector(0x33);
                ins.set_far_branch32(0x1020);
            }
            K::brdisp_2 => {
                ins.set_op_kind(slot, OpKind::NearBranch16);
                ins.set_near_branch16(0x1020);
            }
            K::brdisp_4 => {
                ins.set_op_kind(slot, OpKind::NearBranch32);
                ins.set_near_branch32((SYNTH_IP as u32).wrapping_add(0x20));
            }
            _ => {}
        }
    }

    // For `0F 0D /r` and `0F 18 /r` reserved-NOP forms with memory operands, ModRM reg bits 0..3
    // alias `prefetch*` instructions; use reg bits 4 (`SP`/`ESP`/`RSP` or `R12`) so the decoder
    // round-trips to the exact `Reservednop_*` `Code`.
    if op.is_reserved_nop() && use_mem && op.op_count() >= 2 && ins.op_kind(1) == OpKind::Register {
        let r = match ins.op_register(1).size() {
            2 => Register::R12W,
            4 => Register::R12D,
            8 => Register::R12,
            _ => ins.op_register(1),
        };
        ins.set_op_register(1, r);
    }

    // Round-trip through Encoder -> Decoder so memory_size, encoding, length, and
    // prefix flags match real decoded 64-bit machine code.
    let mut encoder = Encoder::new(64);
    if encoder.encode(&ins, SYNTH_IP).is_ok() {
        let bytes = encoder.take_buffer();
        // Pseudo `fwait`-prefixed x87 codes (`fstcw`, `fstsw`, `fclex`, `finit`, `fsave`, etc.)
        // encode as `0x9B` (`wait`) followed by the non-wait x87 instruction; the decoder yields
        // two instructions (`Wait` then `Fn*`). Verify both decode cleanly and preserve `op.code()`.
        if op.fwait() && bytes.first() == Some(&0x9B) {
            let mut dec = Decoder::with_ip(64, &bytes[1..], SYNTH_IP + 1, DecoderOptions::NONE);
            let inner = dec.decode();
            if !inner.is_invalid() {
                let mut combined = inner;
                combined.set_code(op.code());
                return (combined, true);
            }
        }
        let mut decoder = Decoder::with_ip(64, &bytes, SYNTH_IP, DecoderOptions::NONE);
        let decoded = decoder.decode();
        if !decoded.is_invalid() && decoded.code() == op.code() {
            return (decoded, true);
        }
        // Fallback decoder options for AMD/MPX/reserved-nop forms (e.g. `0F 0D /r` mem).
        let mut decoder2 = Decoder::with_ip(
            64,
            &bytes,
            SYNTH_IP,
            DecoderOptions::AMD
                | DecoderOptions::MPX
                | DecoderOptions::KNC
                | DecoderOptions::FORCE_RESERVED_NOP,
        );
        let decoded2 = decoder2.decode();
        if !decoded2.is_invalid() && decoded2.code() == op.code() {
            return (decoded2, true);
        }
    }

    (ins, false)
}

fn classify_form(ins: &Instruction) -> LoweringClass {
    if is_user_boundary_mnemonic(ins.mnemonic()) {
        return LoweringClass::Boundary;
    }
    match instruction_lowering(ins) {
        InstructionLowering::Arm64 => LoweringClass::Arm64,
        InstructionLowering::FastInterpreter => LoweringClass::FastInterpreter,
        InstructionLowering::MmxDispatch => LoweringClass::MmxDispatch,
        InstructionLowering::GenericDispatch => LoweringClass::GenericDispatch,
    }
}

/// Build the full exhaustive 64-bit ISA coverage ledger.
pub fn analyze_isa_coverage() -> IsaCoverageReport {
    let mut total_codes = 0usize;
    let mut included_codes = 0usize;
    let mut excluded = Vec::new();
    let mut excluded_by_reason: BTreeMap<ExclusionReason, usize> = BTreeMap::new();
    let mut forms = Vec::new();
    let mut totals = ClassCounts::default();
    let mut by_cpuid: BTreeMap<CpuidFeature, ClassCounts> = BTreeMap::new();
    let mut by_family: BTreeMap<String, ClassCounts> = BTreeMap::new();
    let mut synthesis_roundtrip_failures = 0usize;

    // Track per-advertised-feature reachable forms (all required CPUID features advertised).
    let mut adv_counts: BTreeMap<CpuidFeature, ClassCounts> = BTreeMap::new();
    let mut adv_samples: BTreeMap<CpuidFeature, BTreeSet<String>> = BTreeMap::new();

    for code in Code::values() {
        total_codes += 1;
        let op = code.op_code();
        let mut probe = Instruction::default();
        probe.set_code(code);
        let cpuid: Vec<CpuidFeature> = probe.cpuid_features().to_vec();

        if let Some(reason) = exclusion_reason(op, &cpuid) {
            *excluded_by_reason.entry(reason).or_insert(0) += 1;
            excluded.push(ExcludedCode {
                code,
                mnemonic: format!("{:?}", op.mnemonic()).to_ascii_lowercase(),
                reason,
                cpuid_features: cpuid,
            });
            continue;
        }

        included_codes += 1;
        let kinds = op.op_kinds();
        let has_rm = kinds.iter().any(|&k| is_reg_or_mem(k));
        let variants: &[(&'static str, bool)] = if has_rm {
            &[("reg", false), ("mem", true)]
        } else if kinds.iter().any(|&k| is_mem_only(k)) {
            &[("mem", true)]
        } else {
            &[("reg", false)]
        };

        for &(variant, use_mem) in variants {
            let (ins, roundtrip_ok) = synthesize_one(op, use_mem);
            if !roundtrip_ok {
                synthesis_roundtrip_failures += 1;
            }
            let lowering = classify_form(&ins);
            let mnemonic = format!("{:?}", ins.mnemonic()).to_ascii_lowercase();
            let family = classify_instruction(&ins)
                .map(|r| r.id)
                .unwrap_or("UNCATALOGED");
            let form_cpuid = ins.cpuid_features().to_vec();
            let advertised = form_is_advertised(&form_cpuid);

            totals.record(lowering);
            for &feat in &form_cpuid {
                by_cpuid.entry(feat).or_default().record(lowering);
                if advertised && cpuid_advertised(feat) {
                    adv_counts.entry(feat).or_default().record(lowering);
                    if lowering != LoweringClass::Arm64 && lowering != LoweringClass::Boundary {
                        let set = adv_samples.entry(feat).or_default();
                        if set.len() < 12 {
                            set.insert(format!("{code:?}({variant}:{})", lowering.as_str()));
                        }
                    }
                }
            }
            by_family
                .entry(family.to_string())
                .or_default()
                .record(lowering);

            forms.push(IsaFormRow {
                code,
                variant,
                mnemonic,
                encoding: ins.encoding(),
                cpuid_features: form_cpuid,
                catalog_family: family,
                lowering,
                advertised,
                roundtrip_ok,
            });
        }
    }

    let advertised_features: Vec<CpuidFeature> = by_cpuid
        .keys()
        .copied()
        .filter(|&f| cpuid_advertised(f))
        .collect();

    let mut advertised_but_unimplemented = Vec::new();
    for &feat in &advertised_features {
        let counts = adv_counts.get(&feat).cloned().unwrap_or_default();
        if counts.non_arm64_non_boundary() > 0 {
            let sample_codes = adv_samples
                .remove(&feat)
                .map(|s| s.into_iter().collect())
                .unwrap_or_default();
            advertised_but_unimplemented.push(AdvertisedGap {
                feature: feat,
                counts,
                sample_codes,
            });
        }
    }

    IsaCoverageReport {
        total_codes,
        included_codes,
        excluded,
        excluded_by_reason,
        forms,
        totals,
        by_cpuid,
        by_family,
        advertised_features,
        advertised_but_unimplemented,
        synthesis_roundtrip_failures,
    }
}

/// Format the deterministic canonical `db/isa_coverage.tsv` ledger.
///
/// Contains one row per included `(code, variant)` sorted by `Code` declaration order,
/// followed by excluded codes with their explicit exclusion reason.
pub fn format_isa_coverage_tsv(report: &IsaCoverageReport) -> String {
    let mut out = String::with_capacity(report.forms.len() * 88);
    out.push_str(
        "code\tvariant\tmnemonic\tencoding\tcpuid_features\tcatalog_family\tlowering\tadvertised\texclusion_reason\n",
    );
    for row in &report.forms {
        let feats = row
            .cpuid_features
            .iter()
            .map(|f| format!("{f:?}"))
            .collect::<Vec<_>>()
            .join("+");
        let _ = writeln!(
            out,
            "{:?}\t{}\t{}\t{:?}\t{}\t{}\t{}\t{}\t",
            row.code,
            row.variant,
            row.mnemonic,
            row.encoding,
            feats,
            row.catalog_family,
            row.lowering.as_str(),
            if row.advertised { "1" } else { "0" },
        );
    }
    for ex in &report.excluded {
        let feats = ex
            .cpuid_features
            .iter()
            .map(|f| format!("{f:?}"))
            .collect::<Vec<_>>()
            .join("+");
        let _ = writeln!(
            out,
            "{:?}\texcluded\t{}\t\t{}\t\tExcluded\t0\t{}",
            ex.code,
            ex.mnemonic,
            feats,
            ex.reason.as_str(),
        );
    }
    out
}

/// Write `db/isa_coverage.tsv` to disk.
pub fn write_isa_coverage_tsv(path: &Path, report: &IsaCoverageReport) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, format_isa_coverage_tsv(report))
        .with_context(|| format!("writing {}", path.display()))
}

/// Format a human-readable console report.
pub fn format_isa_coverage_report(report: &IsaCoverageReport) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "=== x86-64 ISA Coverage Ledger ===");
    let _ = writeln!(
        out,
        "  iced-x86 Codes         : {} total ({} included, {} excluded)",
        report.total_codes,
        report.included_codes,
        report.excluded.len()
    );
    let _ = writeln!(
        out,
        "  Synthesized Forms      : {} total (round-trip failures: {})",
        report.forms.len(),
        report.synthesis_roundtrip_failures
    );
    let _ = writeln!(
        out,
        "  Arm64                  : {} ({:.2}% of non-boundary)",
        report.totals.arm64,
        report.totals.arm64_pct()
    );
    let _ = writeln!(
        out,
        "  FastInterpreter        : {}",
        report.totals.fast_interpreter
    );
    let _ = writeln!(
        out,
        "  MmxDispatch            : {}",
        report.totals.mmx_dispatch
    );
    let _ = writeln!(
        out,
        "  GenericDispatch        : {}",
        report.totals.generic_dispatch
    );
    let _ = writeln!(out, "  Boundary               : {}", report.totals.boundary);
    let _ = writeln!(out);

    let _ = writeln!(out, "  Excluded Codes by Reason:");
    for (reason, count) in &report.excluded_by_reason {
        let _ = writeln!(out, "    {:24} : {}", reason.as_str(), count);
    }
    let _ = writeln!(out);

    let _ = writeln!(
        out,
        "  Advertised CPUID Features with Non-Arm64 Forms (advertised_but_unimplemented: {}):",
        report.advertised_but_unimplemented.len()
    );
    for gap in &report.advertised_but_unimplemented {
        let _ = writeln!(
            out,
            "    {:16} : arm64={}/{} ({:.1}%), fast_interp={}, mmx={}, generic={}  e.g. {}",
            format!("{:?}", gap.feature),
            gap.counts.arm64,
            gap.counts.total.saturating_sub(gap.counts.boundary),
            gap.counts.arm64_pct(),
            gap.counts.fast_interpreter,
            gap.counts.mmx_dispatch,
            gap.counts.generic_dispatch,
            gap.sample_codes
                .iter()
                .take(4)
                .cloned()
                .collect::<Vec<_>>()
                .join(", "),
        );
    }
    let _ = writeln!(out);

    let _ = writeln!(out, "  By ISA Extension (CpuidFeature):");
    let _ = writeln!(
        out,
        "    {:20} {:>4} {:>6} {:>6} {:>6} {:>6} {:>6} {:>5} {:>7}",
        "FEATURE", "ADV", "TOTAL", "ARM64", "FAST", "MMX", "GEN", "BND", "ARM64%"
    );
    let _ = writeln!(out, "    {}", "-".repeat(76));
    for (feat, c) in &report.by_cpuid {
        let adv = if cpuid_advertised(*feat) { "yes" } else { "no" };
        let _ = writeln!(
            out,
            "    {:20} {:>4} {:>6} {:>6} {:>6} {:>6} {:>6} {:>5} {:>6.1}%",
            format!("{feat:?}"),
            adv,
            c.total,
            c.arm64,
            c.fast_interpreter,
            c.mmx_dispatch,
            c.generic_dispatch,
            c.boundary,
            c.arm64_pct()
        );
    }
    let _ = writeln!(out);

    let _ = writeln!(out, "  By Catalog Family:");
    let _ = writeln!(
        out,
        "    {:28} {:>6} {:>6} {:>6} {:>6} {:>6} {:>5} {:>7}",
        "FAMILY", "TOTAL", "ARM64", "FAST", "MMX", "GEN", "BND", "ARM64%"
    );
    let _ = writeln!(out, "    {}", "-".repeat(78));
    for (family, c) in &report.by_family {
        let _ = writeln!(
            out,
            "    {:28} {:>6} {:>6} {:>6} {:>6} {:>6} {:>5} {:>6.1}%",
            family,
            c.total,
            c.arm64,
            c.fast_interpreter,
            c.mmx_dispatch,
            c.generic_dispatch,
            c.boundary,
            c.arm64_pct()
        );
    }
    out
}

/// Convert the ledger report into a versioned JSON data object.
pub fn isa_coverage_data_json(report: &IsaCoverageReport, synced_tsv: Option<&str>) -> JsonValue {
    let excluded_map = report
        .excluded_by_reason
        .iter()
        .map(|(r, &c)| (r.as_str().to_string(), JsonValue::UInt(c as u64)))
        .collect::<Vec<_>>();

    let by_cpuid = report
        .by_cpuid
        .iter()
        .map(|(f, c)| {
            JsonObj::new()
                .str("feature", format!("{f:?}"))
                .bool("advertised", cpuid_advertised(*f))
                .val("counts", c.to_json())
                .build()
        })
        .collect::<Vec<_>>();

    let by_family = report
        .by_family
        .iter()
        .map(|(fam, c)| {
            JsonObj::new()
                .str("family", fam.clone())
                .val("counts", c.to_json())
                .build()
        })
        .collect::<Vec<_>>();

    let gaps = report
        .advertised_but_unimplemented
        .iter()
        .map(|g| {
            JsonObj::new()
                .str("feature", format!("{:?}", g.feature))
                .val("counts", g.counts.to_json())
                .val(
                    "sample_codes",
                    JsonValue::Array(g.sample_codes.iter().cloned().map(JsonValue::Str).collect()),
                )
                .build()
        })
        .collect::<Vec<_>>();

    JsonObj::new()
        .u64("total_codes", report.total_codes as u64)
        .u64("included_codes", report.included_codes as u64)
        .u64("excluded_codes", report.excluded.len() as u64)
        .u64("synthesized_forms", report.forms.len() as u64)
        .u64(
            "synthesis_roundtrip_failures",
            report.synthesis_roundtrip_failures as u64,
        )
        .val("totals", report.totals.to_json())
        .val("excluded_by_reason", JsonValue::Object(excluded_map))
        .val(
            "advertised_features",
            JsonValue::Array(
                report
                    .advertised_features
                    .iter()
                    .map(|f| JsonValue::Str(format!("{f:?}")))
                    .collect(),
            ),
        )
        .val("advertised_but_unimplemented", JsonValue::Array(gaps))
        .val("by_cpuid_feature", JsonValue::Array(by_cpuid))
        .val("by_catalog_family", JsonValue::Array(by_family))
        .opt_str("synced_tsv", synced_tsv.map(str::to_string))
        .build()
}

/// Explicit, justified allow-list of guest-advertised `CpuidFeature` extensions that still have
/// non-`Arm64` forms on `main`. Each entry MUST explain what remains and which ops slice owns it.
///
/// Any advertised feature NOT in this list MUST have 100% `Arm64` lowering (zero `FastInterpreter`,
/// `MmxDispatch`, or `GenericDispatch` forms), and any regression that introduces a new non-`Arm64`
/// form in a fully-lowered feature immediately fails the test suite.
pub const ADVERTISED_NON_ARM64_ALLOWLIST: &[(CpuidFeature, &str)] = &[
    (
        CpuidFeature::X64,
        "far call/jmp/retf/iretq, segment-reg moves, descriptor/system misc (sldt/str/lar/lsl/verr/verw)",
    ),
    (
        CpuidFeature::AVX,
        "vldmxcsr (MXCSR control-register load)",
    ),
    (
        CpuidFeature::FXSR,
        "fxsave/fxsave64/fxrstor/fxrstor64 x87/SSE state save/restore",
    ),
    (
        CpuidFeature::SSE,
        "ldmxcsr (MXCSR control-register load)",
    ),
    (
        CpuidFeature::XSAVE,
        "xgetbv, xsave/xsave64/xrstor/xrstor64 XSTATE control/save/restore",
    ),
    (
        CpuidFeature::XSAVEC,
        "xsavec/xsavec64 compacted XSTATE save",
    ),
    (
        CpuidFeature::XSAVEOPT,
        "xsaveopt/xsaveopt64 optimized XSTATE save",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn isa_coverage_enumerates_and_roundtrips_all_64bit_codes() {
        let rep = analyze_isa_coverage();
        assert!(rep.total_codes > 4000, "expected full iced-x86 Code table");
        assert!(rep.included_codes > 1500, "expected substantial 64-bit ISA");
        assert!(
            rep.forms.len() > rep.included_codes,
            "expected reg+mem variants"
        );
        assert_eq!(
            rep.included_codes + rep.excluded.len(),
            rep.total_codes,
            "every iced-x86 Code must be either included or explicitly excluded"
        );
        // Spot-check key classifications.
        let find = |code: Code, variant: &str| {
            rep.forms
                .iter()
                .find(|r| r.code == code && r.variant == variant)
                .map(|r| r.lowering)
        };
        assert_eq!(find(Code::Add_rm64_r64, "reg"), Some(LoweringClass::Arm64));
        assert_eq!(find(Code::Add_rm64_r64, "mem"), Some(LoweringClass::Arm64));
        assert_eq!(
            find(Code::VEX_Vaddps_ymm_ymm_ymmm256, "reg"),
            Some(LoweringClass::Arm64)
        );
        assert_eq!(
            find(Code::VEX_Vfmadd213ps_ymm_ymm_ymmm256, "mem"),
            Some(LoweringClass::Arm64)
        );
        assert_eq!(find(Code::Int3, "reg"), Some(LoweringClass::Boundary));
        assert_eq!(find(Code::Ud2, "reg"), Some(LoweringClass::Boundary));
        assert_eq!(find(Code::Syscall, "reg"), Some(LoweringClass::Boundary));
        assert!(rep
            .excluded_by_reason
            .contains_key(&ExclusionReason::Mode1632Only));
        assert!(rep.excluded_by_reason.contains_key(&ExclusionReason::D3Now));
        assert!(rep
            .excluded_by_reason
            .contains_key(&ExclusionReason::XopTbm));
        assert!(rep
            .excluded_by_reason
            .contains_key(&ExclusionReason::KnightsMvex));
        assert!(rep
            .excluded_by_reason
            .contains_key(&ExclusionReason::PrivilegedVmxSgx));
        let bad: Vec<_> = rep
            .forms
            .iter()
            .filter(|r| !r.roundtrip_ok)
            .map(|r| format!("{:?}({})", r.code, r.variant))
            .collect();
        assert!(
            bad.is_empty(),
            "roundtrip failures ({}): {:?}",
            bad.len(),
            bad
        );
    }

    #[test]
    fn advertised_cpuid_features_have_no_unjustified_non_arm64_forms() {
        let rep = analyze_isa_coverage();
        let allow: BTreeSet<CpuidFeature> = ADVERTISED_NON_ARM64_ALLOWLIST
            .iter()
            .map(|(f, reason)| {
                assert!(
                    !reason.trim().is_empty(),
                    "allow-list entry {f:?} needs a justification"
                );
                *f
            })
            .collect();
        let unexpected: Vec<String> = rep
            .advertised_but_unimplemented
            .iter()
            .filter(|g| !allow.contains(&g.feature))
            .map(|g| {
                format!(
                    "{:?} (fast={}, mmx={}, generic={}, samples={:?})",
                    g.feature,
                    g.counts.fast_interpreter,
                    g.counts.mmx_dispatch,
                    g.counts.generic_dispatch,
                    g.sample_codes
                )
            })
            .collect();
        assert!(
            unexpected.is_empty(),
            "guest-advertised CPUID features with non-Arm64 forms not in ADVERTISED_NON_ARM64_ALLOWLIST:\n  {}",
            unexpected.join("\n  ")
        );
        // Also ensure no stale allow-list entries remain once an extension reaches 100% Arm64!
        let actual: BTreeSet<CpuidFeature> = rep
            .advertised_but_unimplemented
            .iter()
            .map(|g| g.feature)
            .collect();
        let stale: Vec<CpuidFeature> = allow.difference(&actual).copied().collect();
        assert!(
            stale.is_empty(),
            "stale entries in ADVERTISED_NON_ARM64_ALLOWLIST (now 100% Arm64! remove them): {stale:?}"
        );
    }
}
