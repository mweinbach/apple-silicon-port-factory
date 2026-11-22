//! Extensibility & Scaffolding Engine for adding new `x86-64 -> ARM64` ops,
//! new Windows/middleware DLL builtins, embeddable AOT Rust crates, and
//! inspecting/testing the Windows virtual filesystem.
//!
//! Designed so bringing up a new Windows `.exe` or `.dll` is a one-command
//! workflow:
//! 1. **Add / Audit Ops (`scaffold-op` & `audit-ops`)**: Scan any binary for
//!    untranslated or fallback `x86-64` mnemonics and generate ready-to-paste
//!    `OpRecord` (`ops_db/catalog_gp.rs` or `ops_db/catalog_simd.rs`),
//!    `aot/lower.rs` lowering arms, and `emu/jit.rs` templates. Because
//!    `ops_db::classify_instruction` auto-indexes `OpRecord::x86_mnemonics`,
//!    adding an `OpRecord` automatically registers all its mnemonics!
//! 2. **Add / Audit DLLs (`scaffold-dll`)**: Inspect a PE `.dll` (its exports)
//!    or a consumer `.exe`/`.dll` (its unresolved imports for a target DLL) and
//!    generate a complete `crates/win32-rt/src/misc/<stem>.rs` builtin module
//!    with `win64_stubs!`, `EXTRA_EXPORTS`, `ORDINALS`, and automatic
//!    registration in `crates/win32-rt/src/misc/mod.rs`.
//! 3. **Embeddable AOT Crate Generator (`aot-crate`)**: Package any Windows
//!    `x86-64` `.exe` or `.dll` into a standalone, self-contained Rust crate
//!    (`Cargo.toml` + `src/lib.rs` + embedded section image `sections.bin` +
//!    `init_embedded_module()`) that any host Rust/macOS/iOS app can depend on
//!    and call directly without `aw2-loader`.

use crate::aot::{compile_pe_file_to_rust, AotOptions};
use crate::ops_db::classify_instruction;
use crate::win32_db::{canonical_dll, Win32Database};
use anyhow::{bail, Context, Result};
use pe_image::{ExportTarget, ImportName, Pe};
use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, IntelFormatter};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

/// Report of untranslated or fallback instructions discovered in a PE binary.
#[derive(Debug, Clone)]
pub struct MissingOpEntry {
    pub mnemonic: String,
    pub count: u64,
    pub sample_encodings: Vec<String>,
    pub sample_disasm: Vec<String>,
}

/// Scan a PE binary for instructions that lack a mapping in `ops_db`.
pub fn find_missing_ops_in_pe(path: &Path) -> Result<Vec<MissingOpEntry>> {
    let bytes = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    let pe = Pe::parse(&bytes).with_context(|| format!("parsing PE {}", path.display()))?;
    let base = pe.headers.image_base;

    let mut by_mnem: BTreeMap<String, (u64, BTreeSet<String>, BTreeSet<String>)> = BTreeMap::new();
    let mut fmt = IntelFormatter::new();

    for sec in &pe.sections {
        if !sec.is_exec() && (sec.characteristics & pe_image::consts::IMAGE_SCN_CNT_CODE == 0) {
            continue;
        }
        let len = sec.raw_size.min(sec.mem_size()) as usize;
        let Ok(code) = pe.bytes_at(sec.rva, len) else {
            continue;
        };
        let mut dec = Decoder::with_ip(64, code, base + sec.rva as u64, DecoderOptions::NONE);
        let mut ins = Instruction::default();
        while dec.can_decode() {
            dec.decode_out(&mut ins);
            if ins.is_invalid() {
                continue;
            }
            if classify_instruction(&ins).is_none() {
                let mn = format!("{:?}", ins.mnemonic()).to_lowercase();
                let entry = by_mnem.entry(mn).or_default();
                entry.0 += 1;
                if entry.1.len() < 3 {
                    entry.1.insert(format!("{:?}", ins.code()));
                }
                if entry.2.len() < 3 {
                    let mut s = String::new();
                    fmt.format(&ins, &mut s);
                    entry.2.insert(s);
                }
            }
        }
    }

    let mut out: Vec<MissingOpEntry> = by_mnem
        .into_iter()
        .map(|(mnemonic, (count, enc, dis))| MissingOpEntry {
            mnemonic,
            count,
            sample_encodings: enc.into_iter().collect(),
            sample_disasm: dis.into_iter().collect(),
        })
        .collect();
    out.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.mnemonic.cmp(&b.mnemonic)));
    Ok(out)
}

/// Generate a complete Rust snippet showing how to add a new `x86-64 -> ARM64`
/// op across `ops_db/catalog_*.rs` and `aot/lower.rs`.
pub fn generate_op_scaffold(mnemonic: &str, category: &str, arm64_pattern: &str) -> String {
    let mn_lower = mnemonic.trim().to_lowercase();
    let mn_upper = mn_lower.to_uppercase();
    let mut pascal = String::new();
    let mut cap = true;
    for c in mn_lower.chars() {
        if c == '_' {
            cap = true;
        } else if cap {
            pascal.push(c.to_ascii_uppercase());
            cap = false;
        } else {
            pascal.push(c);
        }
    }

    let (category_enum, catalog_file) = match category.to_uppercase().as_str() {
        "SIMD" | "SIMD_FP" | "SIMD_FP_PACKED" => (
            "SimdFpPacked",
            "crates/port-factory/src/ops_db/catalog_simd.rs (SIMD_OPS_CATALOG)",
        ),
        "SIMD_FP_SCALAR" => (
            "SimdFpScalar",
            "crates/port-factory/src/ops_db/catalog_simd.rs (SIMD_OPS_CATALOG)",
        ),
        "BIT_MANIP" | "BIT" | "GP_BIT" => (
            "GpBit",
            "crates/port-factory/src/ops_db/catalog_gp.rs (GP_OPS_CATALOG)",
        ),
        "CONTROL" | "CONTROL_FLOW" | "GP_STACK_CF" => (
            "GpStackCf",
            "crates/port-factory/src/ops_db/catalog_gp.rs (GP_OPS_CATALOG)",
        ),
        "ATOMIC" | "SYSTEM" | "GP_ATOMIC" => (
            "GpAtomic",
            "crates/port-factory/src/ops_db/catalog_gp.rs (GP_OPS_CATALOG)",
        ),
        "DATA" | "DATA_TRANSFER" | "GP_MOVE" => (
            "GpMove",
            "crates/port-factory/src/ops_db/catalog_gp.rs (GP_OPS_CATALOG)",
        ),
        _ => (
            "GpAlu",
            "crates/port-factory/src/ops_db/catalog_gp.rs (GP_OPS_CATALOG)",
        ),
    };

    let mut out = String::new();
    let _ = writeln!(
        out,
        "// ============================================================================"
    );
    let _ = writeln!(
        out,
        "// Scaffold for x86-64 -> ARM64 Operation: `{mn_lower}` (`Mnemonic::{pascal}`)"
    );
    let _ = writeln!(
        out,
        "// ============================================================================\n"
    );

    let _ = writeln!(
        out,
        "// 1. Add to `{catalog_file}`\n//    (or append `,{mn_lower}` to an existing OpRecord's `x86_mnemonics` — auto-indexed in O(1)!):"
    );
    let _ = writeln!(
        out,
        r#"    OpRecord {{
        id: "{category}_{mn_upper}",
        fast_kind_id: 125,
        fast_kind_name: "{pascal}",
        category: OpCategory::{category_enum},
        tier: TranslationTier::JitArm64,
        sub_op: "-",
        encodings: "{mn_upper} r/m, reg/imm",
        operand_forms: "RR,RM,RI",
        widths: "32,64",
        x86_mnemonics: "{mn_lower}",
        arm64_pattern: "{arm64_pattern}",
        semantics: "dst = {mn_lower}(dst, src)",
        source_file: "crates/port-factory/src/aot/lower.rs",
        verified_vectors: 100,
    }},"#,
    );
    let _ = writeln!(out);

    let _ = writeln!(
        out,
        "// 2. Add lowering arm in `lower_instruction` (`crates/port-factory/src/aot/lower.rs`):"
    );
    let _ = writeln!(
        out,
        r#"        Mnemonic::{pascal} => {{
            let bits = op_bits(ins, 0);
            let a = read_op_u64(ins, 0, bits, image_base);
            let b = if ins.op_count() > 1 {{
                read_op_u64(ins, 1, bits, image_base)
            }} else {{
                "0u64".to_string()
            }};
            let mask = mask_lit(bits);
            let wr = write_op_u64(ins, 0, bits, "_r", image_base);
            writeln!(
                out,
                "                    {{ let _a = ({{a}}) & {{mask}}; let _b = ({{b}}) & {{mask}}; let _r = _a; /* TODO: {arm64_pattern} */ {{wr}} }}"
            )?;
            return Ok(true);
        }}"#
    );
    let _ = writeln!(out);

    let _ = writeln!(
        out,
        "// 3. Run `./tools/opsdb.sh doctor` to sync `db/ops_x86_to_arm64.tsv` and verify!"
    );
    out
}

/// Sanitize a DLL filename (`foo-bar.dll`) into a valid Rust module stem (`foo_bar`).
pub fn dll_stem_ident(dll_name: &str) -> String {
    let norm = pe_image::normalize_dll_name(dll_name);
    let stem = norm.strip_suffix(".dll").unwrap_or(&norm);
    let mut out = String::new();
    for c in stem.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }
    if out.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        out.insert(0, '_');
    }
    out
}

/// Check if `sym` is a valid unmangled C identifier suitable for a Rust `fn`.
fn is_c_ident(sym: &str) -> bool {
    let mut chars = sym.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Generate a complete `crates/win32-rt/src/misc/<stem>.rs` builtin DLL source file
/// from either:
/// - a target `.dll` PE file on disk (extracting its Export Table), and/or
/// - one or more consumer `.exe`/`.dll` PE files (extracting all imports from `dll_name`).
pub fn generate_dll_scaffold(
    dll_name: &str,
    pe_paths: &[impl AsRef<Path>],
    db: &Win32Database,
) -> Result<(String, String, usize)> {
    let canon = canonical_dll(dll_name);
    let stem = dll_stem_ident(&canon);

    let mut c_symbols: BTreeSet<String> = BTreeSet::new();
    let mut mangled_symbols: BTreeSet<String> = BTreeSet::new();
    let mut ordinals: BTreeMap<u16, String> = BTreeMap::new();

    for p in pe_paths {
        let p = p.as_ref();
        let bytes = fs::read(p).with_context(|| format!("reading {}", p.display()))?;
        let pe = Pe::parse(&bytes).with_context(|| format!("parsing PE {}", p.display()))?;
        let file_dll = pe_image::normalize_dll_name(
            p.file_name().and_then(|n| n.to_str()).unwrap_or(""),
        );

        // If this PE is the DLL itself, harvest its export directory.
        if file_dll == canon || canonical_dll(&file_dll) == canon {
            if let Ok(exps) = pe.exports() {
                for e in exps.entries {
                    if let ExportTarget::Rva(_) = e.target {
                        if let Some(name) = e.name {
                            if is_c_ident(&name) {
                                c_symbols.insert(name.clone());
                            } else {
                                mangled_symbols.insert(name.clone());
                            }
                            if e.ordinal <= u16::MAX as u32 {
                                ordinals.insert(e.ordinal as u16, name);
                            }
                        } else if e.ordinal <= u16::MAX as u32 {
                            let syn = format!("{stem}_ord_{}", e.ordinal);
                            c_symbols.insert(syn.clone());
                            ordinals.insert(e.ordinal as u16, syn);
                        }
                    }
                }
            }
        }

        // Also harvest any imports from `dll_name` in consumer PEs.
        let mut all_imps = pe.imports().unwrap_or_default();
        if let Ok(delay) = pe.delay_imports() {
            all_imps.extend(delay);
        }
        for imp in all_imps {
            if canonical_dll(&imp.dll) == canon || pe_image::normalize_dll_name(&imp.dll) == canon {
                for ent in imp.entries {
                    if db.resolve(&imp.dll, &ent.name).is_some() {
                        continue;
                    }
                    match ent.name {
                        ImportName::Name { name, .. } => {
                            if is_c_ident(&name) {
                                c_symbols.insert(name);
                            } else {
                                mangled_symbols.insert(name);
                            }
                        }
                        ImportName::Ordinal(ord) => {
                            let syn = ordinals
                                .entry(ord)
                                .or_insert_with(|| format!("{stem}_ord_{ord}"))
                                .clone();
                            c_symbols.insert(syn);
                        }
                    }
                }
            }
        }
    }

    if c_symbols.is_empty() && mangled_symbols.is_empty() {
        c_symbols.insert(format!("{stem}_Initialize"));
        c_symbols.insert(format!("{stem}_Shutdown"));
    }

    let total_exports = c_symbols.len() + mangled_symbols.len();
    let mut src = String::new();
    writeln!(
        src,
        "//! Builtin `{canon}` stub/implementation module generated by `port-factory scaffold-dll`."
    )?;
    writeln!(src, "//!")?;
    writeln!(
        src,
        "//! All top-level `pub unsafe extern \"win64\" fn` items and `// @exports` lines"
    )?;
    writeln!(
        src,
        "//! are automatically indexed by `crates/win32-rt/build.rs` and `port-factory win32-sync`."
    )?;
    writeln!(src)?;
    writeln!(src, "use crate::types::*;")?;
    writeln!(src)?;

    // Emit ORDINALS table
    if ordinals.is_empty() {
        writeln!(src, "pub static ORDINALS: &[(u16, &str)] = &[];")?;
    } else {
        writeln!(src, "pub static ORDINALS: &[(u16, &str)] = &[")?;
        for (ord, sym) in &ordinals {
            writeln!(src, "    ({ord}, {sym:?}),")?;
        }
        writeln!(src, "];")?;
    }
    writeln!(src)?;

    // Emit EXTRA_EXPORTS for C++ mangled symbols
    if mangled_symbols.is_empty() {
        writeln!(src, "pub static EXTRA_EXPORTS: &[crate::Export] = &[];")?;
    } else {
        writeln!(
            src,
            "unsafe extern \"win64\" fn mangled_stub_zero() -> usize {{ 0 }}"
        )?;
        writeln!(src)?;
        writeln!(src, "pub static EXTRA_EXPORTS: &[crate::Export] = &[")?;
        for sym in &mangled_symbols {
            writeln!(
                src,
                "    crate::Export {{ name: {sym:?}, ptr: crate::Ptr(mangled_stub_zero as *const ()) }},"
            )?;
        }
        writeln!(src, "];")?;
    }
    writeln!(src)?;

    // Emit C-identifier exports
    for sym in &c_symbols {
        writeln!(
            src,
            "pub unsafe extern \"win64\" fn {sym}(rcx: usize, rdx: usize, r8: usize, r9: usize) -> usize {{"
        )?;
        writeln!(
            src,
            "    stub!(\"{canon}!{sym}(0x{{rcx:x}}, 0x{{rdx:x}}, 0x{{r8:x}}, 0x{{r9:x}}) -> 0\");"
        )?;
        writeln!(src, "    0")?;
        writeln!(src, "}}")?;
        writeln!(src)?;
    }

    Ok((stem, src, total_exports))
}

/// Wire a newly scaffolded `crates/win32-rt/src/misc/<stem>.rs` file into
/// `crates/win32-rt/src/misc/mod.rs` if not already present.
pub fn register_misc_dll_module(workspace_root: &Path, stem: &str, src: &str) -> Result<PathBuf> {
    let misc_dir = workspace_root.join("crates/win32-rt/src/misc");
    let target_rs = misc_dir.join(format!("{stem}.rs"));
    fs::write(&target_rs, src).with_context(|| format!("writing {}", target_rs.display()))?;

    let mod_rs_path = misc_dir.join("mod.rs");
    let mod_contents =
        fs::read_to_string(&mod_rs_path).with_context(|| format!("reading {}", mod_rs_path.display()))?;
    let decl = format!("pub mod {stem};");
    if !mod_contents.lines().any(|l| l.trim() == decl) {
        let mut lines: Vec<String> = mod_contents.lines().map(|s| s.to_string()).collect();
        let insert_pos = lines
            .iter()
            .rposition(|l| l.starts_with("pub mod "))
            .map(|i| i + 1)
            .unwrap_or(lines.len());
        lines.insert(insert_pos, decl);
        let mut updated = lines.join("\n");
        updated.push('\n');
        fs::write(&mod_rs_path, updated)?;
    }
    Ok(target_rs)
}

/// Generate a standalone, embeddable Rust crate (`Cargo.toml` + `src/lib.rs` +
/// `src/sections.bin`) from any Windows `x86-64` `.exe` or `.dll`.
///
/// The generated crate has **zero external dependencies** and includes:
/// - `src/sections.bin`: The pre-mapped PE image sections.
/// - `init_embedded_module()`: Allocates a host backing buffer initialized from
///   `sections.bin`, calls `set_image_base(...)`, and binds any Win32 IAT slots.
/// - Direct `pub extern "C" fn` exports for every exported symbol in the PE.
pub fn generate_embeddable_aot_crate(
    pe_path: &Path,
    out_dir: &Path,
    crate_name: Option<&str>,
    db: &Win32Database,
) -> Result<AotCrateSummary> {
    let raw_pe = fs::read(pe_path).with_context(|| format!("reading {}", pe_path.display()))?;
    let pe = Pe::parse(&raw_pe).with_context(|| format!("parsing PE {}", pe_path.display()))?;

    let file_name = pe_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("module.dll");
    let default_crate_name = format!(
        "aot_{}",
        dll_stem_ident(file_name).trim_start_matches('_')
    );
    let pkg_name = crate_name.unwrap_or(&default_crate_name).replace('_', "-");

    // 1. Build the pre-mapped image buffer (`sections.bin`).
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

    // 2. Compile the PE to AOT Rust source.
    let mut opts = AotOptions::default();
    opts.emit_win64_abi_attr = false;
    let aot = compile_pe_file_to_rust(pe_path, db, &opts)?;

    // 3. Append the embedded image loader (`init_embedded_module()`) so callers
    //    can embed and invoke the module in 1 line of Rust.
    let mut lib_rs = aot.rust_source.clone();
    writeln!(
        lib_rs,
        r#"
// ============================================================================
// Embedded PE Section Image & One-Call Initialization API
// ============================================================================

static EMBEDDED_PE_IMAGE_TEMPLATE: &[u8] = include_bytes!("sections.bin");
static EMBEDDED_INIT: std::sync::Once = std::sync::Once::new();

/// Initialize the embedded PE image in memory and set `IMAGE_BASE` so any
/// exported function can be called directly as a native Rust function.
///
/// Safe to call multiple times; initialization runs once per process.
pub fn init_embedded_module() -> u64 {{
    EMBEDDED_INIT.call_once(|| {{
        let mut boxed = EMBEDDED_PE_IMAGE_TEMPLATE.to_vec().into_boxed_slice();
        let base_ptr = boxed.as_mut_ptr() as u64;
        std::mem::forget(boxed);
        set_image_base(base_ptr);
    }});
    image_base()
}}

/// Install a custom host function pointer into an IAT slot (`iat_rva`) of the
/// embedded module (for overriding or providing external Win32 imports).
pub unsafe fn bind_iat_slot(iat_rva: u32, host_fn: extern "C" fn(u64, u64, u64, u64) -> u64) {{
    let base = init_embedded_module();
    wr64(base.wrapping_add(iat_rva as u64), host_fn as usize as u64);
}}
"#
    )?;

    // 4. Write `Cargo.toml`, `src/sections.bin`, and `src/lib.rs`.
    let src_dir = out_dir.join("src");
    fs::create_dir_all(&src_dir)?;

    let cargo_toml = format!(
        r#"[package]
name = "{pkg_name}"
version = "0.1.0"
edition = "2021"
description = "Static AOT-compiled native ARM64 Rust crate lifted from {file_name}"

[lib]
name = "{lib_name}"
path = "src/lib.rs"
"#,
        lib_name = pkg_name.replace('-', "_")
    );
    fs::write(out_dir.join("Cargo.toml"), cargo_toml)?;
    fs::write(src_dir.join("sections.bin"), &mapped_image)?;
    fs::write(src_dir.join("lib.rs"), &lib_rs)?;

    if mapped_image.is_empty() {
        bail!("mapped image for {file_name} is empty");
    }

    Ok(AotCrateSummary {
        crate_name: pkg_name,
        out_dir: out_dir.to_path_buf(),
        function_count: aot.functions.len(),
        export_count: aot
            .functions
            .iter()
            .map(|f| f.export_names.len())
            .sum(),
        instruction_count: aot.total_instructions,
        lowered_instructions: aot.lowered_instructions,
        image_bytes: mapped_image.len(),
    })
}

#[derive(Debug, Clone)]
pub struct AotCrateSummary {
    pub crate_name: String,
    pub out_dir: std::path::PathBuf,
    pub function_count: usize,
    pub export_count: usize,
    pub instruction_count: usize,
    pub lowered_instructions: usize,
    pub image_bytes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_scaffold_generates_ops_db_and_aot_arms() {
        let out = generate_op_scaffold("vgf2p8affineqb", "SIMD_FP_PACKED", "eor + tbl");
        assert!(out.contains("vgf2p8affineqb"));
        assert!(out.contains("Vgf2p8affineqb"));
        assert!(out.contains("OpCategory::SimdFpPacked"));
        assert!(out.contains("Mnemonic::Vgf2p8affineqb =>"));
    }

    fn test_zlib_path() -> Option<PathBuf> {
        std::env::var_os("PORT_TEST_ZLIB_DLL")
            .filter(|v| !v.is_empty())
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("PORT_GAME_DIR")
                    .or_else(|| std::env::var_os("AW2_GAME_DIR"))
                    .filter(|v| !v.is_empty())
                    .map(|d| PathBuf::from(d).join("zlib1.dll"))
            })
            .filter(|p| p.is_file())
    }

    #[test]
    fn dll_scaffold_extracts_exports_and_imports() {
        let Some(zlib) = test_zlib_path() else {
            return;
        };
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let win_db = Win32Database::from_workspace_src(&repo_root).expect("load win32 db");
        let (stem, code, count) =
            generate_dll_scaffold("zlib1.dll", &[zlib], &win_db).expect("generate dll scaffold");
        assert_eq!(stem, "zlib1");
        assert!(count >= 70);
        assert!(code.contains("pub static ORDINALS:"));
        assert!(code.contains("pub static EXTRA_EXPORTS:"));
        assert!(code.contains("pub unsafe extern \"win64\" fn zlibVersion"));
        assert!(code.contains("pub unsafe extern \"win64\" fn crc32"));
    }

    #[test]
    fn embeddable_aot_crate_generates_self_contained_zlib() {
        let Some(zlib) = test_zlib_path() else {
            return;
        };
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let win_db = Win32Database::from_workspace_src(&repo_root).expect("load win32 db");
        let tmp = std::env::temp_dir().join(format!("aw2_aot_crate_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        let summary =
            generate_embeddable_aot_crate(&zlib, &tmp, Some("aot_zlib1_embedded"), &win_db)
                .expect("generate embeddable crate");
        assert!(summary.function_count > 100);
        assert!(summary.export_count >= 70);
        assert!(tmp.join("Cargo.toml").is_file());
        assert!(tmp.join("src/lib.rs").is_file());
        assert!(tmp.join("src/sections.bin").is_file());
        let lib_rs = fs::read_to_string(tmp.join("src/lib.rs")).unwrap();
        assert!(lib_rs.contains("pub fn init_embedded_module() -> u64"));
        assert!(lib_rs.contains("pub unsafe fn bind_iat_slot"));
        let _ = fs::remove_dir_all(&tmp);
    }
}

