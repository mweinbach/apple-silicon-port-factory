//! Canonical `x86-64 -> ARM64` operation translation database.
//!
//! Organized into modular architecture catalogs:
//! - [`catalog_gp`]: General-purpose moves, ALU, shifts/rotates, bit manipulation,
//!   BMI1/BMI2/ADX/CRC32, multiply/divide, stack/branches/conditionals, atomics,
//!   REP string operations, and flag/system instructions.
//! - [`catalog_simd`]: SSE/SSE2/SSSE3/SSE4.1/SSE4.2, AVX/AVX2 256-bit vector ops,
//!   AVX2 gathers, FMA3 (all 72 forms), Crypto (AES-NI/CLMUL/SHA1/SHA256/F16C),
//!   x87 FPU instructions, and block-boundary trap markers.
//!
//! Adding a new instruction or op family only requires editing [`catalog_gp`] or
//! [`catalog_simd`] — [`classify_instruction`] automatically indexes [`OPS_CATALOG`]
//! in `O(1)` with zero duplicate match arms.

#[macro_use]
pub mod catalog_gp;
#[macro_use]
pub mod catalog_simd;

use anyhow::{Context, Result};
use iced_x86::{Instruction, Mnemonic, OpKind};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;

/// High-level instruction category in the `x86-64 -> ARM64` translation database.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OpCategory {
    GpMove,
    GpAlu,
    GpShiftRot,
    GpBit,
    GpMulDiv,
    GpStackCf,
    GpConditional,
    GpBmiAdxCrc,
    GpAtomic,
    GpString,
    GpFlagSys,
    SimdMove,
    SimdLogic,
    SimdShufPack,
    SimdShift,
    SimdIntAlu,
    SimdIntMulMinMax,
    SimdIntCmpMisc,
    SimdFpScalar,
    SimdFpPacked,
    SimdCvtComi,
    Avx256Lane,
    Fma3,
    CryptoF16cStr,
}

impl OpCategory {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GpMove => "GP_MOVE",
            Self::GpAlu => "GP_ALU",
            Self::GpShiftRot => "GP_SHIFT_ROT",
            Self::GpBit => "GP_BIT",
            Self::GpMulDiv => "GP_MUL_DIV",
            Self::GpStackCf => "GP_STACK_CF",
            Self::GpConditional => "GP_COND",
            Self::GpBmiAdxCrc => "GP_BMI_CRC",
            Self::GpAtomic => "GP_ATOMIC",
            Self::GpString => "GP_STRING",
            Self::GpFlagSys => "GP_FLAG_SYS",
            Self::SimdMove => "SIMD_MOVE",
            Self::SimdLogic => "SIMD_LOGIC",
            Self::SimdShufPack => "SIMD_SHUF_PACK",
            Self::SimdShift => "SIMD_SHIFT",
            Self::SimdIntAlu => "SIMD_INT_ALU",
            Self::SimdIntMulMinMax => "SIMD_INT_MUL_MINMAX",
            Self::SimdIntCmpMisc => "SIMD_INT_CMP_MISC",
            Self::SimdFpScalar => "SIMD_FP_SCALAR",
            Self::SimdFpPacked => "SIMD_FP_PACKED",
            Self::SimdCvtComi => "SIMD_CVT_COMI",
            Self::Avx256Lane => "AVX_256_LANE",
            Self::Fma3 => "FMA3",
            Self::CryptoF16cStr => "CRYPTO_F16C_STR",
        }
    }
}

/// Execution tier for an `x86-64` operation on `aarch64`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TranslationTier {
    /// Lowered to `FastOp` and compiled directly to native `arm64`/`NEON` instructions in `MAP_JIT`
    /// (`emu/jit*.rs`), plus AOT Rust.
    JitArm64,
    /// Lowered to `FastOp` and executed in the pre-decoded fast interpreter + AOT Rust.
    FastInterp,
    /// Executed via the generic `iced-x86` interpreter fallback (not lowered to `FastOp`/ARM64 JIT).
    GenericInterp,
    /// System/trap instruction (`int3`, `ud2`, `syscall`) handled at the block boundary.
    TrapOrBoundary,
}

impl TranslationTier {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::JitArm64 => "JIT_ARM64",
            Self::FastInterp => "FAST_INTERP",
            Self::GenericInterp => "GENERIC_INTERP",
            Self::TrapOrBoundary => "TRAP_BOUNDARY",
        }
    }
}

/// One entry in the `x86-64 -> ARM64` operation translation database.
#[derive(Debug, Clone, Copy)]
pub struct OpRecord {
    /// Unique stable identifier (e.g. `GP_ALU_ADD_SUB`, `SIMD_FP_PACKED_ARITH`, `FMA3_ALL_72`).
    pub id: &'static str,
    /// Primary `FastOp` numeric kind (`1..=124`, or `0` for trap/boundary).
    pub fast_kind_id: u8,
    /// Symbolic `FastOp` constant name(s) in `emu/fast*`.
    pub fast_kind_name: &'static str,
    /// Sub-op selector constant(s) if applicable (e.g. `A_ADD`, `VS_PSHUFB`, `VF_ADDPS`).
    pub sub_op: &'static str,
    /// High-level functional category.
    pub category: OpCategory,
    /// Comma-separated `x86-64` mnemonics covered by this entry.
    pub x86_mnemonics: &'static str,
    /// Supported `x86-64` encoding modes (`Legacy`, `REX`, `VEX.128`, `VEX.256`).
    pub encodings: &'static str,
    /// Supported operand forms (`RR`, `RM`, `MR`, `RI`, `MI`, `RRI`, `RMI`, `R`, `M`, `IMP`).
    pub operand_forms: &'static str,
    /// Supported bit widths (`8,16,32,64`, `128`, `256`).
    pub widths: &'static str,
    /// Emitted `ARM64` / `NEON` instruction sequence.
    pub arm64_pattern: &'static str,
    /// `x86` flags / vector lane / TSO semantics preserved by the translation.
    pub semantics: &'static str,
    /// Highest supported execution tier on `aarch64`.
    pub tier: TranslationTier,
    /// Primary implementation source file in the workspace.
    pub source_file: &'static str,
    /// Number of Rosetta-verified test vectors covering this op family.
    pub verified_vectors: u32,
}

const GP_CATALOG: &[OpRecord] = &gp_ops!();
const SIMD_CATALOG: &[OpRecord] = &simd_ops!();

const fn concat_catalog_len() -> usize {
    GP_CATALOG.len() + SIMD_CATALOG.len()
}

const FULL_CATALOG_ARRAY: [OpRecord; concat_catalog_len()] = {
    let mut arr = [GP_CATALOG[0]; concat_catalog_len()];
    let mut i = 0;
    while i < GP_CATALOG.len() {
        arr[i] = GP_CATALOG[i];
        i += 1;
    }
    let mut j = 0;
    while j < SIMD_CATALOG.len() {
        arr[GP_CATALOG.len() + j] = SIMD_CATALOG[j];
        j += 1;
    }
    arr
};

/// Complete catalog of `x86-64 -> ARM64` translated operations (`FastOp` kinds `1..=124`).
pub static OPS_CATALOG: &[OpRecord] = &FULL_CATALOG_ARRAY;

/// Look up an [`OpRecord`] by its canonical `id`.
pub fn lookup_by_id(id: &str) -> Option<&'static OpRecord> {
    OPS_CATALOG
        .iter()
        .find(|r| r.id.eq_ignore_ascii_case(id))
}

/// Look up all [`OpRecord`] entries matching an `x86-64` mnemonic string (case-insensitive).
pub fn lookup_by_mnemonic(mnemonic: &str) -> Vec<&'static OpRecord> {
    let m = mnemonic.trim().to_ascii_lowercase();
    OPS_CATALOG
        .iter()
        .filter(|r| {
            r.x86_mnemonics
                .split(',')
                .any(|item| item.trim().eq_ignore_ascii_case(&m))
        })
        .collect()
}

/// Lazily initialized lowercase-mnemonic -> primary [`OpRecord`] index built from [`OPS_CATALOG`].
fn mnemonic_index() -> &'static HashMap<String, &'static OpRecord> {
    static INDEX: OnceLock<HashMap<String, &'static OpRecord>> = OnceLock::new();
    INDEX.get_or_init(|| {
        let mut map = HashMap::with_capacity(1024);
        for rec in OPS_CATALOG {
            // `GP_HI8_OPS` is an 8-bit high-register sub-record sharing mnemonics with primary GP records.
            if rec.id == "GP_HI8_OPS" {
                continue;
            }
            for raw in rec.x86_mnemonics.split(',') {
                let mn = raw.trim().to_ascii_lowercase();
                if !mn.is_empty() {
                    map.entry(mn).or_insert(rec);
                }
            }
        }
        if let Some(cmp_fused) = lookup_by_id("GP_ALU_CMP_FUSED") {
            map.insert("cmp".to_string(), cmp_fused);
            map.insert("test".to_string(), cmp_fused);
        }
        map
    })
}

const MNEMONIC_CACHE_SIZE: usize = 4096;
static MNEMONIC_FAST_CACHE: [AtomicUsize; MNEMONIC_CACHE_SIZE] =
    [const { AtomicUsize::new(0) }; MNEMONIC_CACHE_SIZE];

#[inline]
fn lookup_by_iced_mnemonic(mn: Mnemonic) -> Option<&'static OpRecord> {
    let idx = mn as usize;
    if idx < MNEMONIC_CACHE_SIZE {
        let cached = MNEMONIC_FAST_CACHE[idx].load(Ordering::Relaxed);
        if cached == usize::MAX {
            return None;
        }
        if cached != 0 {
            return Some(unsafe { &*(cached as *const OpRecord) });
        }
    }
    let key = format!("{mn:?}").to_ascii_lowercase();
    let found = mnemonic_index().get(&key).copied();
    if idx < MNEMONIC_CACHE_SIZE {
        let val = found
            .map(|r| r as *const OpRecord as usize)
            .unwrap_or(usize::MAX);
        MNEMONIC_FAST_CACHE[idx].store(val, Ordering::Relaxed);
    }
    found
}

/// Classify a decoded `iced_x86::Instruction` into its corresponding [`OpRecord`] in [`OPS_CATALOG`].
///
/// Uses an `O(1)` atomic cache backed by [`OPS_CATALOG`], plus operand-form disambiguation
/// for `LOCK`-prefixed instructions, `mov`/`movnti`, `imul`, and `movsd`/`cmpsd`.
pub fn classify_instruction(ins: &Instruction) -> Option<&'static OpRecord> {
    use Mnemonic as M;
    if ins.has_lock_prefix() {
        return lookup_by_id("GP_ATOMIC_LOCK_RMW");
    }
    let mn = ins.mnemonic();
    if ins.op0_register().is_mm() || ins.op1_register().is_mm() || mn == M::Maskmovq {
        let name = format!("{mn:?}");
        return lookup_by_mnemonic(&name).into_iter().find(|r| r.id == "MMX_ALIASED");
    }
    match mn {
        M::Mov | M::Movnti => {
            let id = match (ins.op_kind(0), ins.op_kind(1)) {
                (OpKind::Register, OpKind::Register) => "GP_MOV_RR",
                (OpKind::Register, OpKind::Memory) => "GP_MOV_RM",
                (OpKind::Memory, OpKind::Register) => "GP_MOV_MR",
                (OpKind::Register, _) => "GP_MOV_RI",
                (OpKind::Memory, _) => "GP_MOV_MI",
                _ => "GP_MOV_RR",
            };
            lookup_by_id(id)
        }
        M::Imul => {
            if ins.op_count() == 1 {
                lookup_by_id("GP_MUL1_DIV1")
            } else {
                lookup_by_id("GP_IMUL_2_3_OP")
            }
        }
        M::Movsd | M::Cmpsd => {
            if ins.op_count() >= 2
                && (matches!(ins.op_kind(0), OpKind::Register)
                    || matches!(ins.op_kind(1), OpKind::Register))
            {
                if mn == M::Movsd {
                    lookup_by_id("SIMD_MOV_LOAD_STORE")
                } else {
                    lookup_by_id("SIMD_FP_SCALAR_ARITH")
                }
            } else {
                lookup_by_id("GP_STRING_REP")
            }
        }
        _ => lookup_by_iced_mnemonic(mn),
    }
}

/// Summary statistics for [`OPS_CATALOG`].
#[derive(Debug, Clone, Default)]
pub struct CatalogStats {
    pub total_records: usize,
    pub total_mnemonics: usize,
    pub jit_arm64_records: usize,
    pub total_verified_vectors: u32,
}

pub fn catalog_stats() -> CatalogStats {
    let mut mnemonics = std::collections::BTreeSet::new();
    let mut jit_arm64_records = 0usize;
    let mut total_verified_vectors = 0u32;
    for r in OPS_CATALOG {
        for m in r.x86_mnemonics.split(',') {
            let s = m.trim().to_ascii_lowercase();
            if !s.is_empty() {
                mnemonics.insert(s);
            }
        }
        if r.tier == TranslationTier::JitArm64 {
            jit_arm64_records += 1;
        }
        total_verified_vectors += r.verified_vectors;
    }
    CatalogStats {
        total_records: OPS_CATALOG.len(),
        total_mnemonics: mnemonics.len(),
        jit_arm64_records,
        total_verified_vectors,
    }
}

/// Format [`OPS_CATALOG`] as the canonical tab-separated database (`db/ops_x86_to_arm64.tsv`).
pub fn format_tsv() -> String {
    use std::fmt::Write as _;
    let mut out = String::from(
        "id\tfast_kind_id\tfast_kind_name\tsub_op\tcategory\ttier\twidths\toperand_forms\tencodings\tx86_mnemonics\tarm64_pattern\tsemantics\tsource_file\tverified_vectors\n",
    );
    for r in OPS_CATALOG {
        let _ = writeln!(
            out,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            r.id,
            r.fast_kind_id,
            r.fast_kind_name,
            r.sub_op,
            r.category.as_str(),
            r.tier.as_str(),
            r.widths,
            r.operand_forms,
            r.encodings,
            r.x86_mnemonics,
            r.arm64_pattern,
            r.semantics,
            r.source_file,
            r.verified_vectors,
        );
    }
    out
}

/// Write [`OPS_CATALOG`] to a tab-separated database file (`db/ops_x86_to_arm64.tsv`).
pub fn write_tsv(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut f = fs::File::create(path).with_context(|| format!("create {}", path.display()))?;
    f.write_all(format_tsv().as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_x86::{Decoder, DecoderOptions};

    #[test]
    fn catalog_ids_are_unique_and_complete() {
        let mut ids = std::collections::HashSet::new();
        for r in OPS_CATALOG {
            assert!(ids.insert(r.id), "duplicate op record id: {}", r.id);
            assert!(!r.x86_mnemonics.is_empty());
            assert!(!r.arm64_pattern.is_empty());
        }
        let st = catalog_stats();
        assert!(st.total_records >= 35);
        assert!(st.total_mnemonics >= 550);
    }

    #[test]
    fn classify_common_x86_instructions() {
        // sub rsp, 0x28; vfmadd213ps xmm0, xmm1, xmm2; ret
        let bytes: &[u8] = &[
            0x48, 0x83, 0xEC, 0x28, // sub rsp, 0x28
            0xC4, 0xE2, 0x71, 0xA8, 0xC2, // vfmadd213ps xmm0, xmm1, xmm2
            0xC3, // ret
        ];
        let mut dec = Decoder::with_ip(64, bytes, 0x1_4000_1000, DecoderOptions::NONE);
        let ins0 = dec.decode();
        assert_eq!(classify_instruction(&ins0).unwrap().id, "GP_ALU_ADD_SUB");
        let ins1 = dec.decode();
        assert_eq!(classify_instruction(&ins1).unwrap().id, "FMA3_ALL_72");
        let ins2 = dec.decode();
        assert_eq!(classify_instruction(&ins2).unwrap().id, "GP_BRANCH_CALL_RET");
    }
}
