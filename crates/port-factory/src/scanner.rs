//! PE32+ binary instruction scanner and `x86-64 -> ARM64` translation coverage analyzer.
//!
//! Disassembles any Windows `x86-64` `.exe` or `.dll` using `.pdata` function
//! boundaries (falling back to executable sections when `.pdata` is absent),
//! classifies every decoded instruction against [`crate::ops_db::OPS_CATALOG`],
//! and reports:
//! - Total instructions and code bytes scanned
//! - Percentage of instructions covered by native `ARM64` JIT (`JIT_ARM64`),
//!   `FastOp` interpreter, trap/padding boundaries, or unmapped mnemonics
//! - Breakdown by [`crate::ops_db::OpCategory`] and [`crate::ops_db::OpRecord`]
//! - Imported DLLs and entry counts (`imports` + `delay_imports`)

use crate::ops_db::{classify_instruction, OpCategory, OpRecord, TranslationTier, OPS_CATALOG};
use anyhow::{Context, Result};
use pe_image::Pe;
use iced_x86::{Decoder, DecoderOptions, Mnemonic};
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::Path;

/// Result of scanning a single PE32+ binary (`.exe` or `.dll`).
#[derive(Debug, Clone)]
pub struct BinaryProfile {
    pub binary_name: String,
    pub is_dll: bool,
    pub image_base: u64,
    pub size_of_image: u32,
    pub functions_count: usize,
    pub exports_count: usize,
    pub imported_dlls: Vec<(String, usize)>,
    pub total_instructions: u64,
    pub total_code_bytes: u64,
    pub jit_arm64_instructions: u64,
    pub trap_boundary_instructions: u64,
    pub unmapped_instructions: u64,
    pub by_record: BTreeMap<&'static str, u64>,
    pub by_category: BTreeMap<OpCategory, u64>,
    pub by_mnemonic: BTreeMap<String, u64>,
    pub unmapped_mnemonics: BTreeMap<String, u64>,
}

impl BinaryProfile {
    /// Percentage of non-padding instructions covered by native `ARM64` JIT (`0.0..=100.0`).
    pub fn jit_coverage_pct(&self) -> f64 {
        let active = self
            .total_instructions
            .saturating_sub(self.trap_boundary_instructions);
        if active == 0 {
            100.0
        } else {
            (self.jit_arm64_instructions as f64 * 100.0) / (active as f64)
        }
    }
}

/// Scan a PE32+ binary on disk and classify all instructions against [`OPS_CATALOG`].
pub fn scan_pe_file(path: &Path) -> Result<BinaryProfile> {
    let data = fs::read(path).with_context(|| format!("read {}", path.display()))?;
    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    scan_pe_bytes(&name, &data)
}

/// Scan an in-memory PE32+ image (`Layout::File`).
pub fn scan_pe_bytes(binary_name: &str, data: &[u8]) -> Result<BinaryProfile> {
    let pe = Pe::parse(data).with_context(|| format!("parse PE {binary_name}"))?;
    let image_base = pe.headers.image_base;
    let pdata = pe.runtime_functions().unwrap_or_default();
    let exports_count = pe.exports().map(|e| e.entries.len()).unwrap_or(0);

    let mut imported_dlls = Vec::new();
    if let Ok(imps) = pe.imports() {
        for imp in imps {
            imported_dlls.push((pe_image::normalize_dll_name(&imp.dll), imp.entries.len()));
        }
    }
    if let Ok(dimps) = pe.delay_imports() {
        for imp in dimps {
            imported_dlls.push((
                format!("{} (delay)", pe_image::normalize_dll_name(&imp.dll)),
                imp.entries.len(),
            ));
        }
    }

    let mut prof = BinaryProfile {
        binary_name: binary_name.to_string(),
        is_dll: pe.headers.is_dll(),
        image_base,
        size_of_image: pe.headers.size_of_image,
        functions_count: pdata.len(),
        exports_count,
        imported_dlls,
        total_instructions: 0,
        total_code_bytes: 0,
        jit_arm64_instructions: 0,
        trap_boundary_instructions: 0,
        unmapped_instructions: 0,
        by_record: BTreeMap::new(),
        by_category: BTreeMap::new(),
        by_mnemonic: BTreeMap::new(),
        unmapped_mnemonics: BTreeMap::new(),
    };

    // Build sorted, non-overlapping code ranges from `.pdata` when available
    // (avoids decoding embedded jump tables / read-only data in `.text`), or
    // fall back to executable sections.
    let mut ranges: Vec<(u32, u32)> = Vec::new();
    if !pdata.is_empty() {
        for rf in &pdata {
            if rf.end_rva > rf.begin_rva {
                ranges.push((rf.begin_rva, rf.end_rva));
            }
        }
        ranges.sort_unstable_by_key(|&(b, _)| b);
    } else {
        for sec in &pe.sections {
            if sec.is_exec() && sec.raw_size > 0 {
                ranges.push((sec.rva, sec.rva + sec.raw_size.min(sec.mem_size())));
            }
        }
    }

    let mut last_end = 0u32;
    for (begin_rva, end_rva) in ranges {
        let start = begin_rva.max(last_end);
        if end_rva <= start {
            continue;
        }
        last_end = end_rva;
        let len = (end_rva - start) as usize;
        let Ok(bytes) = pe.bytes_at(start, len) else {
            continue;
        };
        decode_range(image_base + start as u64, bytes, &mut prof);
    }

    Ok(prof)
}

fn decode_range(start_ip: u64, bytes: &[u8], prof: &mut BinaryProfile) {
    let mut dec = Decoder::with_ip(64, bytes, start_ip, DecoderOptions::NONE);
    while dec.can_decode() {
        let ins = dec.decode();
        if ins.is_invalid() || ins.mnemonic() == Mnemonic::INVALID {
            continue;
        }
        prof.total_instructions += 1;
        prof.total_code_bytes += ins.len() as u64;

        let mn_str = format!("{:?}", ins.mnemonic()).to_ascii_lowercase();
        *prof.by_mnemonic.entry(mn_str.clone()).or_insert(0) += 1;

        match classify_instruction(&ins) {
            Some(rec) => {
                *prof.by_record.entry(rec.id).or_insert(0) += 1;
                *prof.by_category.entry(rec.category).or_insert(0) += 1;
                match rec.tier {
                    TranslationTier::JitArm64 | TranslationTier::FastInterp => {
                        prof.jit_arm64_instructions += 1;
                    }
                    TranslationTier::GenericInterp => {
                        // Tracked as interpreted fallback (not native JIT_ARM64).
                        prof.unmapped_instructions += 1;
                        *prof.unmapped_mnemonics.entry(mn_str).or_insert(0) += 1;
                    }
                    TranslationTier::TrapOrBoundary => {
                        prof.trap_boundary_instructions += 1;
                    }
                }
            }
            None => {
                prof.unmapped_instructions += 1;
                *prof.unmapped_mnemonics.entry(mn_str).or_insert(0) += 1;
            }
        }
    }
}

/// Format a human-readable report for a scanned [`BinaryProfile`].
pub fn format_profile_report(p: &BinaryProfile, top_n: usize) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    let kind = if p.is_dll { "DLL" } else { "EXE" };
    let active = p
        .total_instructions
        .saturating_sub(p.trap_boundary_instructions);
    let _ = writeln!(
        out,
        "=== {} ({kind}, base=0x{:x}, image_size=0x{:x}, .pdata_funcs={}, exports={}) ===",
        p.binary_name, p.image_base, p.size_of_image, p.functions_count, p.exports_count
    );
    let _ = writeln!(
        out,
        "  Instructions : {} total ({} active, {} int3/trap, {:.2} MB code)",
        p.total_instructions,
        active,
        p.trap_boundary_instructions,
        p.total_code_bytes as f64 / (1024.0 * 1024.0)
    );
    let _ = writeln!(
        out,
        "  ARM64 JIT    : {} / {} active ({:.4}% coverage, {} unmapped)",
        p.jit_arm64_instructions,
        active,
        p.jit_coverage_pct(),
        p.unmapped_instructions
    );

    if !p.imported_dlls.is_empty() {
        let total_imps: usize = p.imported_dlls.iter().map(|(_, n)| *n).sum();
        let dll_list: Vec<String> = p
            .imported_dlls
            .iter()
            .map(|(d, n)| format!("{d}({n})"))
            .collect();
        let _ = writeln!(
            out,
            "  Imports      : {} DLLs, {} symbols [{}]",
            p.imported_dlls.len(),
            total_imps,
            dll_list.join(", ")
        );
    }

    let mut cats: Vec<(OpCategory, u64)> = p.by_category.iter().map(|(&k, &v)| (k, v)).collect();
    cats.sort_by(|a, b| b.1.cmp(&a.1));
    let _ = writeln!(out, "  Categories   :");
    for (cat, cnt) in cats.iter().take(top_n) {
        let pct = if p.total_instructions > 0 {
            (*cnt as f64 * 100.0) / (p.total_instructions as f64)
        } else {
            0.0
        };
        let _ = writeln!(out, "    {:22} {:>10} ({:>6.2}%)", cat.as_str(), cnt, pct);
    }

    let mut recs: Vec<(&'static str, u64)> = p.by_record.iter().map(|(&k, &v)| (k, v)).collect();
    recs.sort_by(|a, b| b.1.cmp(&a.1));
    let _ = writeln!(out, "  Top Op Families (x86-64 -> ARM64):");
    for (id, cnt) in recs.iter().take(top_n) {
        let pattern = OPS_CATALOG
            .iter()
            .find(|r| r.id == *id)
            .map(|r| r.arm64_pattern)
            .unwrap_or("-");
        let _ = writeln!(out, "    {:26} {:>10}  -> {}", id, cnt, pattern);
    }

    if !p.unmapped_mnemonics.is_empty() {
        let mut unm: Vec<(&String, &u64)> = p.unmapped_mnemonics.iter().collect();
        unm.sort_by(|a, b| b.1.cmp(a.1));
        let summary: Vec<String> = unm
            .iter()
            .take(12)
            .map(|(m, c)| format!("{m}:{c}"))
            .collect();
        let _ = writeln!(out, "  Unmapped     : {}", summary.join(", "));
    }
    out
}

/// Write or update `db/ops_profiles.tsv` with scanned binary profiles.
pub fn write_profiles_tsv(path: &Path, profiles: &[BinaryProfile]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    // Merge with any existing profiles by `binary_name`.
    let mut rows: BTreeMap<String, String> = BTreeMap::new();
    if path.is_file() {
        if let Ok(existing) = fs::read_to_string(path) {
            for line in existing.lines().skip(1) {
                if let Some((name, _)) = line.split_once('\t') {
                    rows.insert(name.to_string(), line.to_string());
                }
            }
        }
    }
    for p in profiles {
        let active = p
            .total_instructions
            .saturating_sub(p.trap_boundary_instructions);
        let top_ops: Vec<String> = {
            let mut v: Vec<(&'static str, u64)> =
                p.by_record.iter().map(|(&k, &c)| (k, c)).collect();
            v.sort_by(|a, b| b.1.cmp(&a.1));
            v.into_iter()
                .take(5)
                .map(|(k, c)| format!("{k}:{c}"))
                .collect()
        };
        let line = format!(
            "{}\t{}\t0x{:x}\t{}\t{}\t{}\t{}\t{}\t{:.4}\t{}",
            p.binary_name,
            if p.is_dll { "dll" } else { "exe" },
            p.image_base,
            p.functions_count,
            p.total_instructions,
            active,
            p.jit_arm64_instructions,
            p.unmapped_instructions,
            p.jit_coverage_pct(),
            top_ops.join(",")
        );
        rows.insert(p.binary_name.clone(), line);
    }

    let mut f = fs::File::create(path).with_context(|| format!("create {}", path.display()))?;
    writeln!(
        f,
        "binary_name\tkind\timage_base\tpdata_funcs\ttotal_insns\tactive_insns\tjit_arm64_insns\tunmapped_insns\tjit_coverage_pct\ttop_op_families"
    )?;
    for line in rows.values() {
        writeln!(f, "{line}")?;
    }
    Ok(())
}

pub fn collect_pe_files(target: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut out = Vec::new();
    if target.is_file() {
        out.push(target.to_path_buf());
        return Ok(out);
    }
    if target.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(target)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.is_file()
                    && p.extension()
                        .and_then(|e| e.to_str())
                        .is_some_and(|ext| {
                            ext.eq_ignore_ascii_case("exe") || ext.eq_ignore_ascii_case("dll")
                        })
            })
            .collect();
        entries.sort();
        out.extend(entries);
    }
    Ok(out)
}

#[allow(dead_code)]
pub fn find_op_record(id: &str) -> Option<&'static OpRecord> {
    OPS_CATALOG.iter().find(|r| r.id == id)
}
