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

pub mod abi;
pub mod lower;
pub mod operands;
pub mod runtime;

use self::abi::{classify_iat_abi, discover_pe_callback_roots, IatCallAbi};
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
    /// Classified Win64 calling-convention bridge (`IntArgs(0..=12)` vs honest `Fallback`).
    pub call_abi: IatCallAbi,
}

/// One discovered executable callback or EH continuation root in the PE image.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AotCallbackRoot {
    pub rva: u32,
    pub kind: &'static str,
    /// `true` for EH continuation landing pads (`GuardEHContinuationTable`, `C_SCOPE_TABLE.JumpTarget`),
    /// which have distinct mid-frame resume context and are NOT emitted as standalone callable functions.
    pub is_eh_continuation: bool,
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
    /// Statically discovered executable callback/unwind/CRT/COM-sink/EH-continuation roots.
    pub callback_roots: Vec<AotCallbackRoot>,
    /// EH continuation RVAs kept distinct from standalone callable `functions`.
    pub eh_continuations: Vec<u32>,
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
            let call_abi = classify_iat_abi(&canon, &sym);
            imports_by_rva.insert(
                entry.iat_rva,
                AotImportBinding {
                    iat_rva: entry.iat_rva,
                    raw_dll: imp_dll.dll.clone(),
                    canonical_dll: canon.clone(),
                    symbol: sym,
                    rust_path,
                    category,
                    call_abi,
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

    let mut iat_thunks: HashMap<u32, u32> = HashMap::new();
    for &(sec_lo, sec_hi) in &exec_ranges {
        let len = (sec_hi - sec_lo) as usize;
        if let Ok(code) = pe.bytes_at(sec_lo, len) {
            let mut pos = 0usize;
            while pos + 6 <= code.len() {
                if code[pos] == 0xFF && code[pos + 1] == 0x25 {
                    let begin_rva = sec_lo.wrapping_add(pos as u32);
                    let disp = i32::from_le_bytes([
                        code[pos + 2],
                        code[pos + 3],
                        code[pos + 4],
                        code[pos + 5],
                    ]);
                    let next_rva = begin_rva.wrapping_add(6);
                    let target_rva = (next_rva as i64 + disp as i64) as u32;
                    if imports_by_rva.contains_key(&target_rva) {
                        iat_thunks.insert(begin_rva, target_rva);
                    }
                    pos += 6;
                } else {
                    pos += 1;
                }
            }
        }
    }

    // 3b. Discover static executable callback roots (TLS, Load Config CFG / EH continuations,
    // `.pdata` unwind handlers & scope tables, `.CRT*` ctor/dtor arrays, DIR64 relocs, and
    // static registration call sites like `ReadFileEx` arg5 `[rsp+0x20]`, `CreateThread`,
    // `SetWindowSubclass`, `FlsAlloc`, `atexit`, `_initterm`, `RegisterClassExW`, COM sinks).
    let callback_roots = discover_pe_callback_roots(
        &pe,
        &exec_ranges,
        &imports_by_rva,
        &iat_thunks,
    );
    let mut eh_continuation_set: BTreeSet<u32> = BTreeSet::new();
    for cb in &callback_roots {
        if cb.is_eh_continuation {
            // EH continuations resume mid-frame with restored context; NEVER emit as
            // standalone 4-arg callable functions — keep distinct and inject as block leaders.
            eh_continuation_set.insert(cb.rva);
        } else if is_exec_rva(cb.rva) {
            seed_starts.insert(cb.rva);
        }
    }
    let eh_continuations: Vec<u32> = eh_continuation_set.iter().copied().collect();

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
    let mut uses_rdtscp = false;
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
            &eh_continuation_set,
            &mut uses_rdtscp,
            opts,
        )?;
        total_insns += fn_meta.instruction_count;
        total_lowered += fn_meta.lowered_instructions;
        total_fallback += fn_meta.fallback_instructions;
        src.push_str(&fn_code);
        functions_meta.push(fn_meta);
    }

    // 6. Emit module-level dispatch, export, callback-root, and EH-continuation tables.
    emit_function_directory(
        &mut src,
        &functions_meta,
        &pe.sections,
        &callback_roots,
        &eh_continuations,
        uses_rdtscp,
    )?;

    Ok(AotModuleOutput {
        binary_name: binary_name.to_string(),
        image_base,
        size_of_image: pe.headers.size_of_image,
        entry_rva: pe.headers.entry_rva,
        functions: functions_meta,
        imports: imports_by_rva.into_values().collect(),
        callback_roots,
        eh_continuations,
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
    eh_continuations: &BTreeSet<u32>,
    uses_rdtscp: &mut bool,
    opts: &AotOptions,
) -> Result<(AotFunctionMeta, String)> {
    let rust_ident = format!("fn_rva_{begin_rva:08x}");
    let mut out = String::new();

    if let Some(iat_rva) = iat_thunk_rva {
        let imp = &imports_by_rva[&iat_rva];
        let thunk_exact = matches!(imp.call_abi, IatCallAbi::IntArgs(n) if n <= 4);
        writeln!(
            out,
            "// IAT Thunk 0x{begin_rva:08X} -> {}!{} (IAT 0x{iat_rva:08X}, win32={:?}, abi={:?})",
            imp.canonical_dll, imp.symbol, imp.rust_path, imp.call_abi
        )?;
        if !thunk_exact {
            writeln!(
                out,
                "// [AOT_FALLBACK] standalone 4-GPR thunk cannot forward >4/float/variadic args (direct callers inline call_iat_args)"
            )?;
        }
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
                lowered_instructions: usize::from(thunk_exact),
                fallback_instructions: usize::from(!thunk_exact),
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
    for &eh_rva in eh_continuations.range(begin_rva..end_rva) {
        if insn_rvas.contains(&eh_rva) {
            leaders.insert(eh_rva);
        }
    }

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
            if ins.mnemonic() == Mnemonic::Rdtscp {
                *uses_rdtscp = true;
            }
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

    /// Build a minimal valid PE32+ file in memory exercising:
    /// - Exported `invoke_readfile_ex` calling `kernel32.dll!ReadFileEx` with a 5th stack
    ///   argument `[rsp + 0x20]` pointing to a private module-relative callback (`0x1100`)
    /// - Exported `invoke_sprintf` calling `ucrtbase.dll!sprintf` (variadic -> honest fallback)
    /// - `.CRT$XCA` constructor pointer (`0x1140`)
    /// - `.pdata` `UNW_FLAG_EHANDLER` unwind handler (`0x1180`) + `C_SCOPE_TABLE` jump target
    ///   (`0x1040`, EH continuation inside `invoke_readfile_ex`)
    /// - Load Config `GuardEHContinuationTable` entry (`0x1040`, distinct from callable fns)
    fn build_callback_and_5arg_pe_fixture() -> Vec<u8> {
        let mut raw = vec![0u8; 0x3000];
        let w16 = |b: &mut [u8], o: usize, v: u16| b[o..o + 2].copy_from_slice(&v.to_le_bytes());
        let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
        let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());
        let base: u64 = 0x1_8000_0000;

        // DOS + NT headers (file layout == RVA layout with 0x1000 alignment).
        w16(&mut raw, 0, 0x5A4D);
        w32(&mut raw, 0x3C, 0x80);
        w32(&mut raw, 0x80, 0x0000_4550);
        w16(&mut raw, 0x84, 0x8664); // AMD64
        w16(&mut raw, 0x86, 3); // 3 sections: .text, .rdata, .CRT$XCA
        w16(&mut raw, 0x94, 240); // SizeOfOptionalHeader
        w16(&mut raw, 0x96, 0x2022); // EXECUTABLE_IMAGE | LARGE_ADDRESS_AWARE | DLL
        let opt = 0x98;
        w16(&mut raw, opt, 0x20B); // PE32+
        w32(&mut raw, opt + 16, 0x1000); // AddressOfEntryPoint
        w64(&mut raw, opt + 24, base); // ImageBase
        w32(&mut raw, opt + 32, 0x1000); // SectionAlignment
        w32(&mut raw, opt + 36, 0x1000); // FileAlignment
        w32(&mut raw, opt + 56, 0x4000); // SizeOfImage
        w32(&mut raw, opt + 60, 0x400); // SizeOfHeaders
        w32(&mut raw, opt + 108, 16); // NumberOfRvaAndSizes

        // Data directories:
        // [0] Export @ 0x2000 (size 0x100)
        w32(&mut raw, opt + 112, 0x2000);
        w32(&mut raw, opt + 116, 0x100);
        // [1] Import @ 0x2100 (size 0x100)
        w32(&mut raw, opt + 112 + 8, 0x2100);
        w32(&mut raw, opt + 112 + 12, 0x100);
        // [3] Exception (.pdata) @ 0x2300 (size 12)
        w32(&mut raw, opt + 112 + 3 * 8, 0x2300);
        w32(&mut raw, opt + 112 + 3 * 8 + 4, 12);
        // [10] Load Config @ 0x2400 (size 0x140)
        w32(&mut raw, opt + 112 + 10 * 8, 0x2400);
        w32(&mut raw, opt + 112 + 10 * 8 + 4, 0x140);

        // Section headers @ opt + 240 = 0x188
        let sec = opt + 240;
        let put_sec = |b: &mut [u8], idx: usize, name: &[u8; 8], rva: u32, sz: u32, ch: u32| {
            let o = sec + idx * 40;
            b[o..o + 8].copy_from_slice(name);
            w32(b, o + 8, sz); // VirtualSize
            w32(b, o + 12, rva); // VirtualAddress
            w32(b, o + 16, sz); // SizeOfRawData
            w32(b, o + 20, rva); // PointerToRawData (1:1 with RVA)
            w32(b, o + 36, ch);
        };
        put_sec(&mut raw, 0, b".text\0\0\0", 0x1000, 0x1000, 0x6000_0020);
        put_sec(&mut raw, 1, b".rdata\0\0", 0x2000, 0x0800, 0x4000_0040);
        put_sec(&mut raw, 2, b".CRT$XCA", 0x2800, 0x0200, 0x4000_0040);

        // --- .rdata: Exports @ 0x2000 ---
        // Exported functions:
        //   ord 1: invoke_readfile_ex @ 0x1000
        //   ord 2: invoke_sprintf     @ 0x1080
        w32(&mut raw, 0x2000 + 16, 1); // Base ordinal
        w32(&mut raw, 0x2000 + 20, 2); // NumberOfFunctions
        w32(&mut raw, 0x2000 + 24, 2); // NumberOfNames
        w32(&mut raw, 0x2000 + 28, 0x2040); // AddressOfFunctions
        w32(&mut raw, 0x2000 + 32, 0x2050); // AddressOfNames
        w32(&mut raw, 0x2000 + 36, 0x2060); // AddressOfNameOrdinals
        w32(&mut raw, 0x2040, 0x1000);
        w32(&mut raw, 0x2044, 0x1080);
        w32(&mut raw, 0x2050, 0x2070);
        w32(&mut raw, 0x2054, 0x2090);
        w16(&mut raw, 0x2060, 0);
        w16(&mut raw, 0x2062, 1);
        raw[0x2070..0x2070 + 19].copy_from_slice(b"invoke_readfile_ex\0");
        raw[0x2090..0x2090 + 15].copy_from_slice(b"invoke_sprintf\0");

        // --- .rdata: Imports @ 0x2100 ---
        // Descriptor 0: kernel32.dll -> ReadFileEx (IAT @ 0x2200, ILT @ 0x2180)
        // Descriptor 1: ucrtbase.dll -> sprintf    (IAT @ 0x2210, ILT @ 0x2190)
        w32(&mut raw, 0x2100, 0x2180); // OriginalFirstThunk
        w32(&mut raw, 0x2100 + 12, 0x21A0); // Name: kernel32.dll
        w32(&mut raw, 0x2100 + 16, 0x2200); // FirstThunk (IAT)
        w32(&mut raw, 0x2114, 0x2190); // OriginalFirstThunk
        w32(&mut raw, 0x2114 + 12, 0x21B0); // Name: ucrtbase.dll
        w32(&mut raw, 0x2114 + 16, 0x2210); // FirstThunk (IAT)
        // ILT & IAT entries pointing to Hint/Name
        w64(&mut raw, 0x2180, 0x21C0);
        w64(&mut raw, 0x2200, 0x21C0);
        w64(&mut raw, 0x2190, 0x21D8);
        w64(&mut raw, 0x2210, 0x21D8);
        raw[0x21A0..0x21A0 + 13].copy_from_slice(b"kernel32.dll\0");
        raw[0x21B0..0x21B0 + 13].copy_from_slice(b"ucrtbase.dll\0");
        raw[0x21C2..0x21C2 + 11].copy_from_slice(b"ReadFileEx\0");
        raw[0x21DA..0x21DA + 8].copy_from_slice(b"sprintf\0");

        // --- .rdata: .pdata @ 0x2300 + UNWIND_INFO @ 0x2320 ---
        // Function 0x1000..0x1060 has UNW_FLAG_EHANDLER, handler @ 0x1180,
        // and C_SCOPE_TABLE JumpTarget (EH continuation) @ 0x1040.
        w32(&mut raw, 0x2300, 0x1000);
        w32(&mut raw, 0x2304, 0x1060);
        w32(&mut raw, 0x2308, 0x2320);
        raw[0x2320] = 0x01 | (1 << 3); // Version=1, Flags=UNW_FLAG_EHANDLER
        w32(&mut raw, 0x2324, 0x1180); // ExceptionHandler RVA
        w32(&mut raw, 0x2328, 1); // C_SCOPE_TABLE Count = 1
        w32(&mut raw, 0x232C, 0x1004); // Scope Begin
        w32(&mut raw, 0x2330, 0x1030); // Scope End
        w32(&mut raw, 0x2334, 1); // EXCEPTION_EXECUTE_HANDLER
        w32(&mut raw, 0x2338, 0x1040); // JumpTarget (EH continuation!)

        // --- .rdata: Load Config @ 0x2400 with GuardEHContinuationTable @ 0x2560 ---
        w32(&mut raw, 0x2400, 0x140); // Size
        w32(&mut raw, 0x2400 + 0x90, 0x0040_0000); // IMAGE_GUARD_EH_CONTINUATION_TABLE_PRESENT
        w64(&mut raw, 0x2400 + 0x110, base + 0x2560); // GuardEHContinuationTable VA
        w64(&mut raw, 0x2400 + 0x118, 1); // GuardEHContinuationCount
        w32(&mut raw, 0x2560, 0x1040); // EH continuation RVA inside 0x1000..0x1060

        // --- .CRT$XCA @ 0x2800: constructor pointer -> 0x1140 ---
        w64(&mut raw, 0x2800, base + 0x1140);

        // --- .text @ 0x1000: invoke_readfile_ex(h, buf, len, ovl) ---
        // sub rsp, 0x38                      ; 48 83 EC 38 (4B @ 0x1000)
        // lea rax, [rip + (0x1100 - 0x100B)] ; 48 8D 05 F5 00 00 00 (7B @ 0x1004, next=0x100B)
        // mov [rsp + 0x20], rax              ; 48 89 44 24 20 (5B @ 0x100B, next=0x1010)
        // call qword ptr [rip + (0x2200 - 0x1016)] ; FF 15 EA 11 00 00 (6B @ 0x1010, next=0x1016)
        // add rsp, 0x38                      ; 48 83 C4 38 (4B @ 0x1016, next=0x101A)
        // ret                                ; C3 (1B @ 0x101A)
        let mut fn0 = Vec::new();
        fn0.extend_from_slice(&[0x48, 0x83, 0xEC, 0x38]);
        let disp_cb = (0x1100i32 - 0x100Bi32).to_le_bytes();
        fn0.extend_from_slice(&[0x48, 0x8D, 0x05, disp_cb[0], disp_cb[1], disp_cb[2], disp_cb[3]]);
        fn0.extend_from_slice(&[0x48, 0x89, 0x44, 0x24, 0x20]);
        let disp_iat = (0x2200i32 - 0x1016i32).to_le_bytes();
        fn0.extend_from_slice(&[0xFF, 0x15, disp_iat[0], disp_iat[1], disp_iat[2], disp_iat[3]]);
        fn0.extend_from_slice(&[0x48, 0x83, 0xC4, 0x38, 0xC3]);
        raw[0x1000..0x1000 + fn0.len()].copy_from_slice(&fn0);
        // EH continuation landing pad @ 0x1040 (inside 0x1000..0x1060): mov eax, 0xE4; add rsp, 0x38; ret
        raw[0x1040..0x104A].copy_from_slice(&[0xB8, 0xE4, 0x00, 0x00, 0x00, 0x48, 0x83, 0xC4, 0x38, 0xC3]);

        // --- .text @ 0x1080: invoke_sprintf(dst, fmt, val) -> calls variadic sprintf IAT @ 0x2210 ---
        // sub rsp, 0x28; call [rip + (0x2210 - 0x108A)]; add rsp, 0x28; ret
        let disp_sp = (0x2210i32 - 0x108Ai32).to_le_bytes();
        let fn_sp = [
            0x48, 0x83, 0xEC, 0x28,
            0xFF, 0x15, disp_sp[0], disp_sp[1], disp_sp[2], disp_sp[3],
            0x48, 0x83, 0xC4, 0x28,
            0xC3,
        ];
        raw[0x1080..0x1080 + fn_sp.len()].copy_from_slice(&fn_sp);

        // --- .text @ 0x1100: private ReadFileEx APC completion callback ---
        // Returns rcx + rdx + r8 (error_code + bytes_transferred + overlapped_tag)
        // mov rax, rcx; add rax, rdx; add rax, r8; ret
        raw[0x1100..0x110A].copy_from_slice(&[0x48, 0x89, 0xC8, 0x48, 0x01, 0xD0, 0x4C, 0x01, 0xC0, 0xC3]);

        // --- .text @ 0x1140: CRT constructor ---
        // mov eax, 0x42; ret
        raw[0x1140..0x1146].copy_from_slice(&[0xB8, 0x42, 0x00, 0x00, 0x00, 0xC3]);

        // --- .text @ 0x1180: Unwind handler ---
        // mov eax, 1; ret
        raw[0x1180..0x1186].copy_from_slice(&[0xB8, 0x01, 0x00, 0x00, 0x00, 0xC3]);

        raw
    }

    #[test]
    fn ordered_clock_setup_follows_the_functions_selected_for_recompilation() {
        let mut pe = build_callback_and_5arg_pe_fixture();
        pe[0x1000..0x1003].copy_from_slice(&[0x0f, 0x31, 0xc3]); // rdtsc; ret
        pe[0x1080..0x1084].copy_from_slice(&[0x0f, 0x01, 0xf9, 0xc3]); // rdtscp; ret
        let db = Win32Database::default();
        for (rva, ordered) in [(0x1000, false), (0x1080, true)] {
            let opts = AotOptions { filter_rvas: [rva].into(), ..AotOptions::default() };
            let module = compile_pe_bytes_to_rust("clock.dll", &pe, &db, &opts).unwrap();
            assert_eq!(module.functions.len(), 1);
            assert_eq!(module.rust_source.contains("let _ = aot_tsc::has_ecv()"), ordered);
        }
    }

    #[test]
    fn aot_emitter_discovers_callback_roots_handles_5arg_readfileex_and_flags_variadic_fallback() {
        let pe_bytes = build_callback_and_5arg_pe_fixture();
        let db = Win32Database::default();
        let mut opts = AotOptions::default();
        opts.emit_win64_abi_attr = false;
        let out = compile_pe_bytes_to_rust("cb_plugin.dll", &pe_bytes, &db, &opts)
            .expect("compile synthetic callback plugin");

        // 1. Callback roots discovered statically without execution:
        assert!(
            out.callback_roots.iter().any(|c| c.rva == 0x1100 && c.kind == "apc" && !c.is_eh_continuation),
            "expected ReadFileEx 5th-arg stack callback 0x1100 in {:?}",
            out.callback_roots
        );
        assert!(
            out.callback_roots.iter().any(|c| c.rva == 0x1140 && c.kind == "crt_ctor_dtor" && !c.is_eh_continuation),
            "expected .CRT$XCA ctor 0x1140 in {:?}",
            out.callback_roots
        );
        assert!(
            out.callback_roots.iter().any(|c| c.rva == 0x1180 && c.kind == "unwind_handler" && !c.is_eh_continuation),
            "expected .pdata unwind handler 0x1180 in {:?}",
            out.callback_roots
        );
        assert!(
            out.eh_continuations.contains(&0x1040),
            "expected EH continuation 0x1040 in {:?}",
            out.eh_continuations
        );
        // EH continuation MUST NOT be emitted as a standalone callable function in AOT_FUNCTIONS.
        assert!(
            !out.functions.iter().any(|f| f.begin_rva == 0x1040),
            "EH continuation 0x1040 must stay distinct from standalone callable AOT_FUNCTIONS"
        );
        // Private ReadFileEx callback, CRT ctor, and unwind handler ARE emitted as callable fns.
        assert!(out.functions.iter().any(|f| f.begin_rva == 0x1100));
        assert!(out.functions.iter().any(|f| f.begin_rva == 0x1140));
        assert!(out.functions.iter().any(|f| f.begin_rva == 0x1180));

        // 2. ABI discipline:
        //    - invoke_readfile_ex (5-arg ReadFileEx via call_iat_args(..., 5)) is fully lowered!
        //    - invoke_sprintf (variadic sprintf) is honestly marked fallback (NOT fully_lowered)!
        let f_read = out.functions.iter().find(|f| f.begin_rva == 0x1000).unwrap();
        assert_eq!(f_read.fallback_instructions, 0, "ReadFileEx 5-arg bridge must be exact");
        let f_sprintf = out.functions.iter().find(|f| f.begin_rva == 0x1080).unwrap();
        assert!(
            f_sprintf.fallback_instructions > 0,
            "variadic sprintf must be marked honest fallback"
        );

        // 3. Compile and natively execute the generated Rust on ARM64 to prove ReadFileEx's
        //    5th stack argument pointer (`[rsp + 0x20]`) arrives intact and invokes 0x1100!
        let tmp_dir = std::env::temp_dir().join(format!("port_aot_5arg_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp_dir);
        std::fs::create_dir_all(&tmp_dir).unwrap();
        let src_file = tmp_dir.join("cb_plugin_runner.rs");
        let bin_file = tmp_dir.join("cb_plugin_runner");

        let mut runner = out.rust_source.clone();
        writeln!(
            runner,
            r#"
static mut SEEN_ARGS: [u64; 5] = [0; 5];

extern "C" fn mock_read_file_ex(h: u64, buf: u64, len: u64, ovl: u64, completion: u64) -> u64 {{
    unsafe {{
        SEEN_ARGS = [h, buf, len, ovl, completion];
        // Invoke the 5th argument (module-relative callback VA) through lookup_aot_function!
        let base = image_base();
        let rva = (completion - base) as u32;
        let cb = lookup_aot_function(rva).expect("completion callback 0x1100 lifted");
        cb(0, len, ovl, 0)
    }}
}}

fn main() {{
    let mut img = vec![0u8; 0x4000];
    let base = img.as_mut_ptr() as u64;
    set_image_base(base);
    // Patch ReadFileEx IAT slot @ 0x2200 with our 5-arg native host mock.
    unsafe {{
        wr64(base + 0x2200, mock_read_file_ex as *const () as usize as u64);
    }}
    let res = invoke_readfile_ex(0x11, 0x2200, 128, 7);
    let seen = unsafe {{ SEEN_ARGS }};
    assert_eq!(seen[0], 0x11);
    assert_eq!(seen[1], 0x2200);
    assert_eq!(seen[2], 128);
    assert_eq!(seen[3], 7);
    assert_eq!(seen[4], base + 0x1100, "5th stack arg lpCompletionRoutine must be base + 0x1100");
    assert_eq!(res, 135, "callback(0, 128, 7) must return 135");
    assert_eq!(AOT_EH_CONTINUATIONS, &[0x1040]);
    println!("AOT_5ARG_CALLBACK_OK res={{res}}");
}}
"#
        )
        .unwrap();

        std::fs::write(&src_file, &runner).unwrap();
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
            .expect("invoke rustc on generated 5-arg callback plugin");
        assert!(status.success(), "rustc failed on generated 5-arg plugin");

        let run_out = Command::new(&bin_file).output().expect("run 5-arg plugin");
        let stdout = String::from_utf8_lossy(&run_out.stdout);
        let stderr = String::from_utf8_lossy(&run_out.stderr);
        let _ = std::fs::remove_dir_all(&tmp_dir);
        assert!(
            run_out.status.success() && stdout.contains("AOT_5ARG_CALLBACK_OK res=135"),
            "native ARM64 execution failed: stdout={stdout} stderr={stderr}"
        );
    }
}
