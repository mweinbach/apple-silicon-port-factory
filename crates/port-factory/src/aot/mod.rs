//! Generalized Static AOT Compiler (`x86-64 PE -> Native ARM64 Rust`).
//!
//! Organized into focused submodules:
//! - [`mod`]: PE function boundary discovery, `.pdata` + export + call-target
//!   analysis, basic-block CFG construction, and module orchestration.
//! - [`runtime`]: Emits the self-contained AOT runtime prelude, IAT bindings,
//!   and function/export/section lookup directories.
//! - [`lower`]: Instruction-level lowering (`x86-64 -> Native Rust/ARM64`).
//! - [`operands`]: Register, memory effective-address, and condition-code
//!   expression builders.

pub mod lower;
pub mod operands;
pub mod runtime;

use self::lower::{is_block_terminator, is_valid_rust_ident, lower_instruction};
use self::runtime::{emit_aot_runtime_prelude, emit_function_directory, emit_import_table};
use crate::win32_db::{canonical_dll, Win32Database};
use anyhow::{Context, Result};
use pe_image::{ExportTarget, Pe};
use iced_x86::{
    Decoder, DecoderOptions, Formatter, Instruction, IntelFormatter, Mnemonic, OpKind,
};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;
use std::path::Path;

/// A bound import slot in the PE's Import Address Table (IAT).
#[derive(Debug, Clone)]
pub struct AotImportBinding {
    pub iat_rva: u32,
    pub raw_dll: String,
    pub canonical_dll: String,
    pub symbol: String,
    /// Resolved `win32_rt::...` Rust function path, if implemented in our Win32 layer.
    pub rust_path: Option<String>,
    pub category: String,
}

/// Metadata for a single lifted function in the PE image.
#[derive(Debug, Clone)]
pub struct AotFunctionMeta {
    pub begin_rva: u32,
    pub end_rva: u32,
    pub rust_ident: String,
    pub export_names: Vec<String>,
    /// If this function is a 6-byte `jmp [rip + disp32]` IAT thunk, the IAT RVA it jumps through.
    pub iat_thunk_rva: Option<u32>,
    pub block_count: usize,
    pub instruction_count: usize,
    pub lowered_instructions: usize,
    pub fallback_instructions: usize,
}

/// Statistics and generated source from AOT-compiling a PE binary.
#[derive(Debug, Clone)]
pub struct AotModuleOutput {
    pub binary_name: String,
    pub image_base: u64,
    pub size_of_image: u32,
    pub entry_rva: u32,
    pub functions: Vec<AotFunctionMeta>,
    pub imports: Vec<AotImportBinding>,
    pub total_instructions: usize,
    pub lowered_instructions: usize,
    pub fallback_instructions: usize,
    pub rust_source: String,
}

impl AotModuleOutput {
    pub fn lowering_coverage_pct(&self) -> f64 {
        if self.total_instructions == 0 {
            100.0
        } else {
            (self.lowered_instructions as f64 * 100.0) / (self.total_instructions as f64)
        }
    }
}

/// Options controlling the Static AOT Compiler.
#[derive(Debug, Clone)]
pub struct AotOptions {
    /// Emit `#[win64_abi::win64_abi]` on generated entrypoints.
    pub emit_win64_abi_attr: bool,
    /// Optional filter of specific function RVAs to emit (empty = emit all discovered functions).
    pub filter_rvas: BTreeSet<u32>,
    /// Maximum number of functions to emit (0 = unlimited).
    pub max_functions: usize,
}

impl Default for AotOptions {
    fn default() -> Self {
        Self {
            emit_win64_abi_attr: true,
            filter_rvas: BTreeSet::new(),
            max_functions: 0,
        }
    }
}

/// Compile a PE32+ file on disk into a standalone AOT Rust module.
pub fn compile_pe_file_to_rust(
    path: &Path,
    win32_db: &Win32Database,
    opts: &AotOptions,
) -> Result<AotModuleOutput> {
    let bytes = std::fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    let name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("module.dll");
    compile_pe_bytes_to_rust(name, &bytes, win32_db, opts)
}

/// Compile raw PE32+ file bytes into a standalone AOT Rust module.
pub fn compile_pe_bytes_to_rust(
    binary_name: &str,
    bytes: &[u8],
    win32_db: &Win32Database,
    opts: &AotOptions,
) -> Result<AotModuleOutput> {
    let pe = Pe::parse(bytes).with_context(|| format!("parsing PE {binary_name}"))?;
    let image_base = pe.headers.image_base;

    // 1. Collect all IAT imports and bind them to `Win32Database`.
    let mut imports_by_rva: BTreeMap<u32, AotImportBinding> = BTreeMap::new();
    let mut all_imports = pe.imports().unwrap_or_default();
    if let Ok(delay) = pe.delay_imports() {
        all_imports.extend(delay);
    }
    for imp_dll in &all_imports {
        let canon = canonical_dll(&imp_dll.dll);
        for entry in &imp_dll.entries {
            let sym = entry.name.to_string();
            let resolved = win32_db.resolve(&imp_dll.dll, &entry.name);
            let (rust_path, category) = match resolved {
                Some(api) => (
                    Some(api.rust_path.clone()),
                    api.category.as_str().to_string(),
                ),
                None => (None, "UNRESOLVED".to_string()),
            };
            imports_by_rva.insert(
                entry.iat_rva,
                AotImportBinding {
                    iat_rva: entry.iat_rva,
                    raw_dll: imp_dll.dll.clone(),
                    canonical_dll: canon.clone(),
                    symbol: sym,
                    rust_path,
                    category,
                },
            );
        }
    }

    // 2. Collect export names by RVA.
    let mut exports_by_rva: HashMap<u32, Vec<String>> = HashMap::new();
    if let Ok(exps) = pe.exports() {
        for e in exps.entries {
            if let ExportTarget::Rva(rva) = e.target {
                let sym = e.name.unwrap_or_else(|| format!("ordinal_{}", e.ordinal));
                exports_by_rva.entry(rva).or_default().push(sym);
            }
        }
    }

    // 3. Discover function boundaries from `.pdata`, exports, entry_rva, and direct calls.
    let exec_ranges: Vec<(u32, u32)> = pe
        .sections
        .iter()
        .filter(|s| s.is_exec() || (s.characteristics & pe_image::consts::IMAGE_SCN_CNT_CODE != 0))
        .map(|s| (s.rva, s.rva.saturating_add(s.raw_size.min(s.mem_size()))))
        .collect();
    let is_exec_rva = |rva: u32| -> bool {
        exec_ranges
            .iter()
            .any(|&(lo, hi)| rva >= lo && rva < hi)
    };

    let mut fn_bounds: BTreeMap<u32, u32> = BTreeMap::new();
    if let Ok(rfs) = pe.runtime_functions() {
        for rf in rfs {
            if rf.begin_rva < rf.end_rva && is_exec_rva(rf.begin_rva) {
                fn_bounds
                    .entry(rf.begin_rva)
                    .and_modify(|e| *e = (*e).max(rf.end_rva))
                    .or_insert(rf.end_rva);
            }
        }
    }

    let mut seed_starts: BTreeSet<u32> = BTreeSet::new();
    if pe.headers.entry_rva != 0 && is_exec_rva(pe.headers.entry_rva) {
        seed_starts.insert(pe.headers.entry_rva);
    }
    for &rva in exports_by_rva.keys() {
        if is_exec_rva(rva) {
            seed_starts.insert(rva);
        }
    }

    for &(sec_lo, sec_hi) in &exec_ranges {
        let len = (sec_hi - sec_lo) as usize;
        if let Ok(code) = pe.bytes_at(sec_lo, len) {
            let mut dec =
                Decoder::with_ip(64, code, image_base + sec_lo as u64, DecoderOptions::NONE);
            let mut ins = Instruction::default();
            while dec.can_decode() {
                dec.decode_out(&mut ins);
                if ins.mnemonic() == Mnemonic::Call
                    && matches!(
                        ins.op0_kind(),
                        OpKind::NearBranch64 | OpKind::NearBranch32 | OpKind::NearBranch16
                    )
                {
                    let target_va = ins.near_branch_target();
                    if let Some(rel) = target_va.checked_sub(image_base) {
                        let target_rva = rel as u32;
                        if is_exec_rva(target_rva) {
                            seed_starts.insert(target_rva);
                        }
                    }
                }
            }
        }
    }

    let all_known_starts: Vec<u32> = fn_bounds
        .keys()
        .copied()
        .chain(seed_starts.iter().copied())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    for (idx, &start_rva) in all_known_starts.iter().enumerate() {
        if fn_bounds.contains_key(&start_rva) {
            continue;
        }
        if let Ok(b) = pe.bytes_at(start_rva, 6) {
            if b[0] == 0xFF && b[1] == 0x25 {
                fn_bounds.insert(start_rva, start_rva + 6);
                continue;
            }
        }
        let sec_end = exec_ranges
            .iter()
            .find(|&&(lo, hi)| start_rva >= lo && start_rva < hi)
            .map(|&(_, hi)| hi)
            .unwrap_or(start_rva + 64);
        let next_start = all_known_starts
            .get(idx + 1)
            .copied()
            .unwrap_or(sec_end)
            .min(sec_end);
        let end_rva = infer_leaf_end_rva(&pe, start_rva, next_start);
        if end_rva > start_rva {
            fn_bounds.insert(start_rva, end_rva);
        }
    }

    let selected_fns: Vec<(u32, u32)> = fn_bounds
        .iter()
        .filter(|(&begin_rva, &end_rva)| {
            if !opts.filter_rvas.is_empty() && !opts.filter_rvas.contains(&begin_rva) {
                return false;
            }
            let len = end_rva.saturating_sub(begin_rva) as usize;
            pe.bytes_at(begin_rva, len).is_ok()
        })
        .take(if opts.max_functions > 0 {
            opts.max_functions
        } else {
            usize::MAX
        })
        .map(|(&b, &e)| (b, e))
        .collect();

    let known_fn_rvas: BTreeSet<u32> = selected_fns.iter().map(|&(b, _)| b).collect();

    let mut iat_thunks: HashMap<u32, u32> = HashMap::new();
    for (&begin_rva, _) in &fn_bounds {
        if let Ok(b) = pe.bytes_at(begin_rva, 6) {
            if b[0] == 0xFF && b[1] == 0x25 {
                let disp = i32::from_le_bytes([b[2], b[3], b[4], b[5]]);
                let next_rva = begin_rva.wrapping_add(6);
                let target_rva = (next_rva as i64 + disp as i64) as u32;
                if imports_by_rva.contains_key(&target_rva) {
                    iat_thunks.insert(begin_rva, target_rva);
                }
            }
        }
    }

    // 4. Emit the AOT Rust module header.
    let mut src = String::with_capacity(64 * 1024);
    writeln!(
        src,
        "// AUTO-GENERATED by `port-factory aot` from `{binary_name}`."
    )?;
    writeln!(
        src,
        "// Target architecture: native aarch64 (Static AOT x86-64 -> Rust translation)."
    )?;
    writeln!(
        src,
        "#![allow(unused_variables, unused_mut, unused_assignments, unused_parens, dead_code, non_snake_case, unreachable_code, clippy::all)]"
    )?;
    writeln!(src)?;
    emit_aot_runtime_prelude(&mut src, image_base, pe.headers.size_of_image)?;
    emit_import_table(&mut src, &imports_by_rva)?;

    // 5. Lift each function to native Rust.
    let mut functions_meta = Vec::new();
    let mut total_insns = 0usize;
    let mut total_lowered = 0usize;
    let mut total_fallback = 0usize;

    for &(begin_rva, end_rva) in &selected_fns {
        let len = end_rva.saturating_sub(begin_rva) as usize;
        let Ok(code_bytes) = pe.bytes_at(begin_rva, len) else {
            continue;
        };
        let exp_names = exports_by_rva
            .get(&begin_rva)
            .cloned()
            .unwrap_or_default();
        let iat_thunk_rva = iat_thunks.get(&begin_rva).copied();

        let (fn_meta, fn_code) = lift_function(
            begin_rva,
            end_rva,
            code_bytes,
            image_base,
            exp_names,
            iat_thunk_rva,
            &known_fn_rvas,
            &iat_thunks,
            &imports_by_rva,
            opts,
        )?;
        total_insns += fn_meta.instruction_count;
        total_lowered += fn_meta.lowered_instructions;
        total_fallback += fn_meta.fallback_instructions;
        src.push_str(&fn_code);
        functions_meta.push(fn_meta);
    }

    // 6. Emit module-level dispatch and export lookup tables.
    emit_function_directory(&mut src, &functions_meta, &pe.sections)?;

    Ok(AotModuleOutput {
        binary_name: binary_name.to_string(),
        image_base,
        size_of_image: pe.headers.size_of_image,
        entry_rva: pe.headers.entry_rva,
        functions: functions_meta,
        imports: imports_by_rva.into_values().collect(),
        total_instructions: total_insns,
        lowered_instructions: total_lowered,
        fallback_instructions: total_fallback,
        rust_source: src,
    })
}

/// Heuristic to find the end of a small leaf function that lacks a `.pdata` entry.
fn infer_leaf_end_rva(pe: &Pe<'_>, start_rva: u32, max_end_rva: u32) -> u32 {
    let cap = max_end_rva.saturating_sub(start_rva).min(512) as usize;
    let Ok(bytes) = pe.bytes_at(start_rva, cap) else {
        return start_rva;
    };
    let mut dec = Decoder::with_ip(
        64,
        bytes,
        pe.headers.image_base + start_rva as u64,
        DecoderOptions::NONE,
    );
    let mut ins = Instruction::default();
    let mut furthest_branch_rva = start_rva;
    while dec.can_decode() {
        dec.decode_out(&mut ins);
        if ins.is_invalid() {
            break;
        }
        let next_rva = (ins.next_ip() - pe.headers.image_base) as u32;
        if matches!(
            ins.op0_kind(),
            OpKind::NearBranch64 | OpKind::NearBranch32 | OpKind::NearBranch16
        ) && ins.mnemonic() != Mnemonic::Call
        {
            if let Some(rel) = ins.near_branch_target().checked_sub(pe.headers.image_base) {
                let t = rel as u32;
                if t > start_rva && t <= max_end_rva {
                    furthest_branch_rva = furthest_branch_rva.max(t);
                }
            }
        }
        if matches!(ins.mnemonic(), Mnemonic::Ret | Mnemonic::Int3)
            && next_rva >= furthest_branch_rva
        {
            return next_rva;
        }
    }
    max_end_rva.min(start_rva + cap as u32)
}

/// Lift a single PE function (`begin_rva..end_rva`) into a native Rust function.
#[allow(clippy::too_many_arguments)]
fn lift_function(
    begin_rva: u32,
    end_rva: u32,
    code_bytes: &[u8],
    image_base: u64,
    export_names: Vec<String>,
    iat_thunk_rva: Option<u32>,
    known_fn_rvas: &BTreeSet<u32>,
    iat_thunks: &HashMap<u32, u32>,
    imports_by_rva: &BTreeMap<u32, AotImportBinding>,
    opts: &AotOptions,
) -> Result<(AotFunctionMeta, String)> {
    let rust_ident = format!("fn_rva_{begin_rva:08x}");
    let mut out = String::new();

    if let Some(iat_rva) = iat_thunk_rva {
        let imp = &imports_by_rva[&iat_rva];
        writeln!(
            out,
            "// IAT Thunk 0x{begin_rva:08X} -> {}!{} (IAT 0x{iat_rva:08X},win32={:?})",
            imp.canonical_dll, imp.symbol, imp.rust_path
        )?;
        if opts.emit_win64_abi_attr {
            writeln!(out, "#[cfg_attr(feature = \"win64-abi\", win64_abi::win64_abi)]")?;
        }
        writeln!(
            out,
            "pub extern \"C\" fn {rust_ident}(rcx: u64, rdx: u64, r8: u64, r9: u64) -> u64 {{"
        )?;
        writeln!(out, "    unsafe {{ call_iat(0x{iat_rva:08X}, rcx, rdx, r8, r9) }}")?;
        writeln!(out, "}}\n")?;
        return Ok((
            AotFunctionMeta {
                begin_rva,
                end_rva,
                rust_ident,
                export_names,
                iat_thunk_rva: Some(iat_rva),
                block_count: 1,
                instruction_count: 1,
                lowered_instructions: 1,
                fallback_instructions: 0,
            },
            out,
        ));
    }

    let mut dec = Decoder::with_ip(
        64,
        code_bytes,
        image_base + begin_rva as u64,
        DecoderOptions::NONE,
    );
    let mut insns: Vec<(u32, Instruction)> = Vec::new();
    while dec.can_decode() {
        let ins = dec.decode();
        if ins.is_invalid() {
            break;
        }
        let rva = (ins.ip() - image_base) as u32;
        insns.push((rva, ins));
    }

    let mut leaders: BTreeSet<u32> = BTreeSet::new();
    leaders.insert(begin_rva);
    let insn_rvas: BTreeSet<u32> = insns.iter().map(|&(r, _)| r).collect();

    for &(_rva, ref ins) in &insns {
        let next_rva = (ins.next_ip() - image_base) as u32;
        if is_block_terminator(ins) && insn_rvas.contains(&next_rva) {
            leaders.insert(next_rva);
        }
        if matches!(
            ins.op0_kind(),
            OpKind::NearBranch64 | OpKind::NearBranch32 | OpKind::NearBranch16
        ) && ins.mnemonic() != Mnemonic::Call
        {
            if let Some(rel) = ins.near_branch_target().checked_sub(image_base) {
                let target_rva = rel as u32;
                if insn_rvas.contains(&target_rva) {
                    leaders.insert(target_rva);
                }
            }
        }
    }

    let mut blocks: BTreeMap<u32, Vec<(u32, Instruction)>> = BTreeMap::new();
    let mut current_leader = begin_rva;
    for (rva, ins) in insns {
        if leaders.contains(&rva) {
            current_leader = rva;
        }
        blocks.entry(current_leader).or_default().push((rva, ins));
    }

    let block_keys: Vec<u32> = blocks.keys().copied().collect();
    let mut fmt = IntelFormatter::new();

    if !export_names.is_empty() {
        writeln!(
            out,
            "// Function 0x{begin_rva:08X}..0x{end_rva:08X} exports: {}",
            export_names.join(", ")
        )?;
    } else {
        writeln!(out, "// Function 0x{begin_rva:08X}..0x{end_rva:08X}")?;
    }
    if opts.emit_win64_abi_attr {
        writeln!(out, "#[cfg_attr(feature = \"win64-abi\", win64_abi::win64_abi)]")?;
    }
    writeln!(
        out,
        "pub extern \"C\" fn {rust_ident}(rcx_in: u64, rdx_in: u64, r8_in: u64, r9_in: u64) -> u64 {{"
    )?;
    writeln!(out, "    unsafe {{")?;
    writeln!(
        out,
        "        let mut rax: u64 = 0; let mut rcx: u64 = rcx_in; let mut rdx: u64 = rdx_in; let mut rbx: u64 = 0;"
    )?;
    writeln!(
        out,
        "        let mut rbp: u64 = 0; let mut rsi: u64 = 0; let mut rdi: u64 = 0;"
    )?;
    writeln!(
        out,
        "        let mut r8: u64 = r8_in; let mut r9: u64 = r9_in; let mut r10: u64 = 0; let mut r11: u64 = 0;"
    )?;
    writeln!(
        out,
        "        let mut r12: u64 = 0; let mut r13: u64 = 0; let mut r14: u64 = 0; let mut r15: u64 = 0;"
    )?;
    writeln!(out, "        let mut xmm: [u128; 16] = [0; 16];")?;
    writeln!(
        out,
        "        let mut cf: bool = false; let mut zf: bool = false; let mut sf: bool = false; let mut of: bool = false;"
    )?;
    writeln!(
        out,
        "        let mut frame = core::mem::MaybeUninit::<[u64; 512]>::uninit();"
    )?;
    writeln!(
        out,
        "        let mut rsp: u64 = (frame.as_mut_ptr() as *mut u64).add(480) as u64;"
    )?;
    writeln!(
        out,
        "        const SENTINEL_RET: u64 = 0xDEAD_BEEF_CAFE_0000;"
    )?;
    writeln!(out, "        wr64(rsp, SENTINEL_RET);")?;
    writeln!(out, "        let mut pc: u32 = 0x{begin_rva:08X};")?;
    writeln!(out, "        loop {{")?;
    writeln!(out, "            match pc {{")?;

    let mut total_fn_insns = 0usize;
    let mut lowered_fn_insns = 0usize;
    let mut fallback_fn_insns = 0usize;

    for (blk_idx, &blk_rva) in block_keys.iter().enumerate() {
        let next_blk_rva = block_keys.get(blk_idx + 1).copied();
        let blk_insns = &blocks[&blk_rva];
        writeln!(out, "                0x{blk_rva:08X} => {{")?;
        let mut terminated = false;

        for &(insn_rva, ref ins) in blk_insns {
            total_fn_insns += 1;
            let mut disasm = String::new();
            fmt.format(ins, &mut disasm);
            writeln!(out, "                    // 0x{insn_rva:08X}: {disasm}")?;

            let mut stmt = String::new();
            let lowered = lower_instruction(
                &mut stmt,
                ins,
                image_base,
                next_blk_rva,
                &insn_rvas,
                known_fn_rvas,
                iat_thunks,
                imports_by_rva,
                &mut terminated,
            )?;
            if lowered {
                lowered_fn_insns += 1;
            } else {
                fallback_fn_insns += 1;
            }
            out.push_str(&stmt);
            if terminated {
                break;
            }
        }

        if !terminated {
            if let Some(fallthrough) = next_blk_rva {
                writeln!(out, "                    pc = 0x{fallthrough:08X};")?;
            } else {
                writeln!(out, "                    return rax;")?;
            }
        }
        writeln!(out, "                }}")?;
    }

    writeln!(out, "                _ => return rax,")?;
    writeln!(out, "            }}")?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}\n")?;

    for exp in &export_names {
        if is_valid_rust_ident(exp) && exp != &rust_ident {
            writeln!(
                out,
                "#[inline(always)]\npub extern \"C\" fn {exp}(rcx: u64, rdx: u64, r8: u64, r9: u64) -> u64 {{ {rust_ident}(rcx, rdx, r8, r9) }}\n"
            )?;
        }
    }

    Ok((
        AotFunctionMeta {
            begin_rva,
            end_rva,
            rust_ident,
            export_names,
            iat_thunk_rva: None,
            block_count: block_keys.len(),
            instruction_count: total_fn_insns,
            lowered_instructions: lowered_fn_insns,
            fallback_instructions: fallback_fn_insns,
        },
        out,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::process::Command;

    #[test]
    fn aot_compiles_and_natively_executes_zlib1_dll_on_arm64() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        let db = Win32Database::from_workspace_src(&root).expect("scan win32 db");

        let Some(zlib_path) = std::env::var_os("PORT_TEST_ZLIB_DLL")
            .filter(|v| !v.is_empty())
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("PORT_GAME_DIR")
                    .or_else(|| std::env::var_os("AW2_GAME_DIR"))
                    .filter(|v| !v.is_empty())
                    .map(|d| PathBuf::from(d).join("zlib1.dll"))
            })
            .filter(|p| p.exists())
        else {
            return;
        };
        let mut opts = AotOptions::default();
        opts.emit_win64_abi_attr = false;
        let out = compile_pe_file_to_rust(&zlib_path, &db, &opts).expect("AOT compile zlib1.dll");
        assert!(out.functions.len() >= 150);
        assert_eq!(
            out.fallback_instructions, 0,
            "expected 100.00% instruction lowering on zlib1.dll"
        );
        assert!(out.rust_source.contains("pub extern \"C\" fn zlibVersion"));
        assert!(out.rust_source.contains("pub extern \"C\" fn adler32"));
        assert!(out.rust_source.contains("pub extern \"C\" fn crc32"));

        let tmp_dir = std::env::temp_dir().join("aw2_x86_aot_test");
        std::fs::create_dir_all(&tmp_dir).unwrap();
        let src_file = tmp_dir.join("zlib1_aot_runner.rs");
        let bin_file = tmp_dir.join("zlib1_aot_runner");

        let raw_pe = std::fs::read(zlib_path).unwrap();
        let pe = Pe::parse(&raw_pe).unwrap();
        let mut mapped_image = vec![0u8; pe.headers.size_of_image as usize];
        let hdr_len = (pe.headers.size_of_headers as usize)
            .min(raw_pe.len())
            .min(mapped_image.len());
        mapped_image[..hdr_len].copy_from_slice(&raw_pe[..hdr_len]);
        for sec in &pe.sections {
            let dst_off = sec.rva as usize;
            let src_off = sec.raw_offset as usize;
            let copy_len = (sec.raw_size as usize).min(sec.mem_size() as usize);
            if dst_off + copy_len <= mapped_image.len() && src_off + copy_len <= raw_pe.len() {
                mapped_image[dst_off..dst_off + copy_len]
                    .copy_from_slice(&raw_pe[src_off..src_off + copy_len]);
            }
        }
        let img_file = tmp_dir.join("zlib1_mapped.bin");
        std::fs::write(&img_file, &mapped_image).unwrap();

        let mut full_runner = out.rust_source.clone();
        writeln!(
            full_runner,
            r#"
fn main() {{
    let mut img = std::fs::read({img_path:?}).expect("read mapped image");
    set_image_base(img.as_mut_ptr() as u64);

    let ver_ptr = zlibVersion(0, 0, 0, 0) as *const u8;
    let ver = unsafe {{ std::ffi::CStr::from_ptr(ver_ptr as *const std::ffi::c_char).to_str().unwrap() }};
    assert!(ver.starts_with("1.2."), "unexpected zlibVersion: {{ver}}");

    let msg = b"hello";
    let adler = adler32(1, msg.as_ptr() as u64, msg.len() as u64, 0) as u32;
    assert_eq!(adler, 0x062C0215, "unexpected adler32: 0x{{adler:08x}}");

    let crc = crc32(0, msg.as_ptr() as u64, msg.len() as u64, 0) as u32;
    assert_eq!(crc, 0x3610A686, "unexpected crc32: 0x{{crc:08x}}");

    println!("AOT_ZLIB_OK version={{ver}} adler32=0x{{adler:08x}} crc32=0x{{crc:08x}}");
}}
"#,
            img_path = img_file.display().to_string()
        )
        .unwrap();

        std::fs::write(&src_file, &full_runner).unwrap();
        let status = Command::new("rustc")
            .args([
                "--edition=2021",
                "-C",
                "opt-level=1",
                src_file.to_str().unwrap(),
                "-o",
                bin_file.to_str().unwrap(),
            ])
            .status()
            .expect("invoke rustc on AOT-generated zlib1.dll source");
        assert!(
            status.success(),
            "rustc failed to compile AOT-generated zlib1.dll"
        );

        let run_out = Command::new(&bin_file)
            .output()
            .expect("execute AOT-compiled zlib1.dll native ARM64 binary");
        let stdout = String::from_utf8_lossy(&run_out.stdout);
        let stderr = String::from_utf8_lossy(&run_out.stderr);
        assert!(
            run_out.status.success() && stdout.contains("AOT_ZLIB_OK"),
            "AOT execution failed: stdout={stdout} stderr={stderr}"
        );
    }
}
