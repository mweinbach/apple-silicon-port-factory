//! Static dispatch on file-backed executable bytes. Nothing in the PE is executed.
use super::Finding;
use anyhow::{Context, Result};
use iced_x86::{
    Code, ConditionCode, CpuidFeature, Decoder, DecoderOptions, EncodingKind, Mnemonic,
};
use pe_image::Pe;
use std::collections::{BTreeMap, BTreeSet};
use x86_runtime::emu::{arm64_guest_cpuid, instruction_lowering, InstructionLowering};

include!(concat!(env!("OUT_DIR"), "/generic_handlers.rs"));

#[derive(Default)]
pub(super) struct CpuScan {
    pub decoded: u64,
    pub invalid_bytes: u64,
    pub findings: BTreeMap<(Finding, Code), InstructionUse>,
}

pub(super) struct InstructionUse {
    pub mnemonic: String,
    pub count: u64,
    pub first_rva: u32,
    pub advertised_features: Vec<String>,
}

// Architectural CPUID bit positions (iced-x86 CpuidFeature). Read the values from the
// ARM64 emitter's actual guest profile. Unmapped feature IDs carry no advertisement claim.
fn advertised(feature: CpuidFeature) -> bool {
    use CpuidFeature::*;
    let (leaf, reg, bit) = match feature {
        FPU => (1, 3, 0),
        TSC => (1, 3, 4),
        CX8 => (1, 3, 8),
        CMOV => (1, 3, 15),
        CLFSH => (1, 3, 19),
        MMX => (1, 3, 23),
        FXSR => (1, 3, 24),
        SSE => (1, 3, 25),
        SSE2 => (1, 3, 26),
        SSE3 => (1, 2, 0),
        PCLMULQDQ => (1, 2, 1),
        VMX => (1, 2, 5),
        SSSE3 => (1, 2, 9),
        FMA => (1, 2, 12),
        CMPXCHG16B => (1, 2, 13),
        SSE4_1 => (1, 2, 19),
        SSE4_2 => (1, 2, 20),
        MOVBE => (1, 2, 22),
        POPCNT => (1, 2, 23),
        AES => (1, 2, 25),
        XSAVE => (1, 2, 26),
        AVX => (1, 2, 28),
        F16C => (1, 2, 29),
        RDRAND => (1, 2, 30),
        FSGSBASE => (7, 1, 0),
        BMI1 => (7, 1, 3),
        AVX2 => (7, 1, 5),
        BMI2 => (7, 1, 8),
        INVPCID => (7, 1, 10),
        RDSEED => (7, 1, 18),
        ADX => (7, 1, 19),
        SHA => (7, 1, 29),
        X64 => (0x8000_0001, 3, 29),
        PREFETCHW => (0x8000_0001, 2, 8),
        SSE4A => (0x8000_0001, 2, 6),
        SYSCALL => (0x8000_0001, 3, 11),
        LZCNT => (0x8000_0001, 2, 5),
        RDTSCP => (0x8000_0001, 3, 27),
        _ => return false,
    };
    arm64_guest_cpuid(leaf)[reg] & (1 << bit) != 0
}

impl CpuScan {
    fn decode(&mut self, bytes: &[u8], rva: u32) {
        let mut decoder = Decoder::with_ip(64, bytes, u64::from(rva), DecoderOptions::NONE);
        while decoder.can_decode() {
            let ins = decoder.decode();
            if ins.is_invalid() {
                self.invalid_bytes += ins.len() as u64;
                continue;
            }
            self.decoded += 1;
            let mn = ins.mnemonic();
            let finding = match instruction_lowering(&ins) {
                // These encodings have masks, vector lengths or register files outside
                // the SSE/AVX2 model. A shared mnemonic can select a narrower handler.
                InstructionLowering::GenericDispatch
                    if !has_handler(mn) && ins.condition_code() == ConditionCode::None =>
                {
                    Finding::Unimplemented
                }
                _ if matches!(ins.encoding(), EncodingKind::EVEX | EncodingKind::MVEX) => {
                    Finding::UnverifiedForm
                }
                InstructionLowering::Arm64 => continue,
                _ if matches!(
                    mn,
                    Mnemonic::Int3
                        | Mnemonic::Ud2
                        | Mnemonic::Int
                        | Mnemonic::Syscall
                        | Mnemonic::Sysenter
                ) =>
                {
                    Finding::GuestBoundary
                }
                _ => Finding::Interpreter,
            };
            let hit = self
                .findings
                .entry((finding, ins.code()))
                .or_insert_with(|| {
                    let features = ins.cpuid_features();
                    let advertised_features = if finding == Finding::Unimplemented
                        && features.iter().all(|f| advertised(*f))
                    {
                        features.iter().map(|f| format!("{f:?}")).collect()
                    } else {
                        Vec::new()
                    };
                    InstructionUse {
                        mnemonic: format!("{mn:?}").to_ascii_lowercase(),
                        first_rva: ins.ip() as u32,
                        count: 0,
                        advertised_features,
                    }
                });
            hit.count += 1;
        }
    }

    pub fn scan(pe: &Pe) -> Result<Self> {
        let mut scan = Self::default();
        // Restart at known boundaries so inline data cannot desynchronize decoding of
        // the next function. Include the gaps: leaf functions need no .pdata entry.
        let mut boundaries = BTreeSet::from([pe.headers.entry_rva]);
        for function in pe.runtime_functions()? {
            boundaries.extend([function.begin_rva, function.end_rva]);
        }
        for section in pe.sections.iter().filter(|s| s.is_exec()) {
            let len = section.raw_size.min(section.mem_size());
            if len == 0 {
                continue;
            }
            let start = section.rva;
            let end = start
                .checked_add(len)
                .context("executable section RVA overflow")?;
            let bytes = pe
                .bytes_at(start, len as usize)
                .context("truncated executable section")?;
            let mut pos = start;
            for next in boundaries.range((start + 1)..end).copied().chain([end]) {
                scan.decode(&bytes[(pos - start) as usize..(next - start) as usize], pos);
                pos = next;
            }
        }
        Ok(scan)
    }
}
