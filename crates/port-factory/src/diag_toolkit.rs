//! Reusable Port Factory Diagnostic & Verification Toolkit (`diag_toolkit`).
//!
//! Provides game-agnostic static runtime-hazard auditing, `PORT_NAN_TRACE` spec
//! construction, and workspace tool discovery for the unified `port-factory` CLI:
//! - `runtime-audit` / `mxcsr-audit`: static scan of `.exe`/`.dll` binaries for
//!   Windows x86-64 -> ARM64 runtime hazards (`ldmxcsr`/`stmxcsr` FTZ/DAZ/RC,
//!   x87 FPU, AVX-256->128 narrowing, approx FP, `loopcc`, lock-RMW / `WaitOnAddress`,
//!   and RWX self-modifying sections).
//! - `verify-x86`: drives the 4-tier offline differential verifier (`x86-verify`).
//! - `oracle`: drives the Rosetta 2 x86-64 hardware oracle (`tools/debug/x86_oracle.py`).
//! - `bisect-runtime`: drives the automated knob matrix & VA bisector (`tools/debug/bisect_runtime.py`).
//! - `live-probe`: drives any game via `PORT_CONTROL_DIR` (`tools/debug/live_probe.py`).
//! - `nan-trace`: launches `port-runner` pre-wired for `PORT_NAN_TRACE` + `PORT_NAN_BOUNDARY`.

use anyhow::{bail, Context, Result};
use iced_x86::{Decoder, DecoderOptions, Mnemonic};
use pe_image::Pe;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Static Windows x86-64 -> ARM64 runtime-hazard summary for a single PE32+ binary.
#[derive(Debug, Clone, Default)]
pub struct RuntimeHazardReport {
    pub binary_name: String,
    pub is_dll: bool,
    pub total_instructions: u64,
    /// `ldmxcsr` / `vldmxcsr` sites (FTZ / DAZ / rounding-mode changes).
    pub ldmxcsr_count: u64,
    /// `stmxcsr` / `vstmxcsr` sites.
    pub stmxcsr_count: u64,
    /// Sample RVAs (up to 8) of `ldmxcsr`/`vldmxcsr` instructions.
    pub ldmxcsr_sample_rvas: Vec<u32>,
    /// x87 control-word (`fldcw` / `fnstcw` / `fstcw`) instructions.
    pub x87_ctrl_count: u64,
    /// x87 FPU arithmetic / stack instructions (generic interpreter tier).
    pub x87_op_count: u64,
    /// AVX 256->128 narrowing ops (`vcvtpd2dq`, `vcvtps2ph`, `vmovmsk*`, `vpmovmskb`).
    pub avx_narrow_count: u64,
    /// Approximate FP reciprocal / rsqrt instructions (`rcp*`, `rsqrt*`).
    pub approx_fp_count: u64,
    /// `loop` / `loope` / `loopne` / `jrcxz` / `jecxz` instructions.
    pub loop_branch_count: u64,
    /// `LOCK`-prefixed atomic RMW instructions.
    pub lock_rmw_count: u64,
    /// Executable + writable (`IMAGE_SCN_MEM_EXECUTE | IMAGE_SCN_MEM_WRITE`) section names.
    pub rwx_sections: Vec<String>,
    /// Imported FP-control / lockless-sync / VEH APIs worth noting during bring-up.
    pub notable_imports: BTreeSet<String>,
}

impl RuntimeHazardReport {
    pub fn has_mxcsr_control(&self) -> bool {
        self.ldmxcsr_count > 0
            || self.notable_imports.iter().any(|s| {
                s.contains("_controlfp")
                    || s.contains("_control87")
                    || s.contains("_set_controlfp")
            })
    }

    pub fn has_wait_on_address(&self) -> bool {
        self.notable_imports
            .iter()
            .any(|s| s.contains("WaitOnAddress") || s.contains("WakeByAddress"))
    }
}

const NOTABLE_IMPORT_SYMBOLS: &[&str] = &[
    "_controlfp",
    "_controlfp_s",
    "_control87",
    "__control87_2",
    "_set_controlfp",
    "_clearfp",
    "_statusfp",
    "WaitOnAddress",
    "WakeByAddressSingle",
    "WakeByAddressAll",
    "AddVectoredExceptionHandler",
    "SetUnhandledExceptionFilter",
    "SetThreadContext",
    "GetThreadContext",
    "FlushInstructionCache",
];

fn is_x87_op(mn: Mnemonic) -> bool {
    use Mnemonic::*;
    matches!(
        mn,
        Fld | Fst | Fstp | Fild | Fist | Fistp | Fisttp | Fadd | Faddp | Fiadd | Fsub | Fsubp
            | Fsubr | Fsubrp | Fisub | Fisubr | Fmul | Fmulp | Fimul | Fdiv | Fdivp | Fdivr
            | Fdivrp | Fidiv | Fidivr | Fcom | Fcomp | Fcompp | Fcomi | Fcomip | Fucom | Fucomp
            | Fucompp | Fucomi | Fucomip | Fchs | Fabs | Ftst | Fxam | Fsqrt | Fsin | Fcos
            | Fsincos | Fptan | Fpatan | Fprem | Fprem1 | Fyl2x | Fyl2xp1 | F2xm1 | Fscale
            | Frndint | Fxch | Fcmovb | Fcmove | Fcmovbe | Fcmovu | Fcmovnb | Fcmovne
            | Fcmovnbe | Fcmovnu | Fldz | Fld1 | Fldpi | Fldl2t | Fldl2e | Fldlg2 | Fldln2
            | Fninit | Finit | Fnclex | Fclex | Fnstsw | Fstsw
    )
}

/// Statically audit one PE32+ file on disk for x86-64 -> ARM64 runtime hazards.
pub fn audit_pe_runtime_hazards(path: &Path) -> Result<RuntimeHazardReport> {
    let data = fs::read(path).with_context(|| format!("read {}", path.display()))?;
    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    audit_pe_bytes_runtime_hazards(&name, &data)
}

/// Statically audit an in-memory PE32+ image (`Layout::File`) for runtime hazards.
pub fn audit_pe_bytes_runtime_hazards(
    binary_name: &str,
    data: &[u8],
) -> Result<RuntimeHazardReport> {
    let pe = Pe::parse(data).with_context(|| format!("parse PE {binary_name}"))?;
    Ok(audit_pe_parsed_runtime_hazards(binary_name, &pe))
}

/// Statically audit an already-parsed [`Pe`] image for runtime hazards.
pub fn audit_pe_parsed_runtime_hazards(binary_name: &str, pe: &Pe) -> RuntimeHazardReport {
    let mut rep = RuntimeHazardReport {
        binary_name: binary_name.to_string(),
        is_dll: pe.headers.is_dll(),
        ..RuntimeHazardReport::default()
    };

    const SCN_EXEC: u32 = 0x2000_0000;
    const SCN_WRITE: u32 = 0x8000_0000;
    for sec in &pe.sections {
        if (sec.characteristics & (SCN_EXEC | SCN_WRITE)) == (SCN_EXEC | SCN_WRITE) {
            rep.rwx_sections.push(sec.name.clone());
        }
    }

    let mut check_imp = |dll: &str, name: &str| {
        if NOTABLE_IMPORT_SYMBOLS.iter().any(|&s| s.eq_ignore_ascii_case(name)) {
            rep.notable_imports
                .insert(format!("{}!{name}", pe_image::normalize_dll_name(dll)));
        }
    };
    if let Ok(imps) = pe.imports() {
        for imp in imps {
            for e in &imp.entries {
                if let pe_image::ImportName::Name { name, .. } = &e.name {
                    check_imp(&imp.dll, name);
                }
            }
        }
    }
    if let Ok(dimps) = pe.delay_imports() {
        for imp in dimps {
            for e in &imp.entries {
                if let pe_image::ImportName::Name { name, .. } = &e.name {
                    check_imp(&imp.dll, name);
                }
            }
        }
    }

    let mut scan_span = |bytes: &[u8], base_rva: u32| {
        let mut dec = Decoder::with_ip(
            64,
            bytes,
            pe.headers.image_base.wrapping_add(base_rva as u64),
            DecoderOptions::NONE,
        );
        while dec.can_decode() {
            let ins = dec.decode();
            if ins.is_invalid() {
                continue;
            }
            rep.total_instructions += 1;
            if ins.has_lock_prefix() {
                rep.lock_rmw_count += 1;
            }
            let mn = ins.mnemonic();
            match mn {
                Mnemonic::Ldmxcsr | Mnemonic::Vldmxcsr => {
                    rep.ldmxcsr_count += 1;
                    if rep.ldmxcsr_sample_rvas.len() < 8 {
                        let rva = ins.ip().wrapping_sub(pe.headers.image_base) as u32;
                        rep.ldmxcsr_sample_rvas.push(rva);
                    }
                }
                Mnemonic::Stmxcsr | Mnemonic::Vstmxcsr => {
                    rep.stmxcsr_count += 1;
                }
                Mnemonic::Fldcw | Mnemonic::Fnstcw | Mnemonic::Fstcw => {
                    rep.x87_ctrl_count += 1;
                }
                Mnemonic::Vcvtpd2dq
                | Mnemonic::Vcvttpd2dq
                | Mnemonic::Vcvtpd2ps
                | Mnemonic::Vcvtps2ph
                | Mnemonic::Vmovmskps
                | Mnemonic::Vmovmskpd
                | Mnemonic::Vpmovmskb => {
                    rep.avx_narrow_count += 1;
                }
                Mnemonic::Rcpss
                | Mnemonic::Vrcpss
                | Mnemonic::Rcpps
                | Mnemonic::Vrcpps
                | Mnemonic::Rsqrtss
                | Mnemonic::Vrsqrtss
                | Mnemonic::Rsqrtps
                | Mnemonic::Vrsqrtps => {
                    rep.approx_fp_count += 1;
                }
                Mnemonic::Loop
                | Mnemonic::Loope
                | Mnemonic::Loopne
                | Mnemonic::Jrcxz
                | Mnemonic::Jecxz => {
                    rep.loop_branch_count += 1;
                }
                _ if is_x87_op(mn) => {
                    rep.x87_op_count += 1;
                }
                _ => {}
            }
        }
    };

    let pdata = pe.runtime_functions().unwrap_or_default();
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
                let len = sec.raw_size.min(sec.mem_size());
                if len > 0 {
                    ranges.push((sec.rva, sec.rva.saturating_add(len)));
                }
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
        if let Ok(slice) = pe.bytes_at(start, len) {
            scan_span(slice, start);
        }
    }

    rep
}

/// Format a table of [`RuntimeHazardReport`]s for terminal output.
pub fn format_hazard_reports(reports: &[RuntimeHazardReport]) -> String {
    let mut out = String::new();
    out.push_str("=== Runtime Hazard Audit (FP/MXCSR, x87, AVX Narrow, Approx-FP, Sync, RWX) ===\n");
    out.push_str(&format!(
        "  {:30} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>6} {:>4}\n",
        "BINARY", "LDMXCSR", "STMXCSR", "X87_OPS", "AVX_NRW", "APX_FP", "LOCK", "LOOPCC", "RWX"
    ));
    out.push_str(&format!("  {}\n", "-".repeat(96)));
    let mut tot_ld = 0u64;
    let mut tot_st = 0u64;
    let mut tot_x87 = 0u64;
    let mut tot_nrw = 0u64;
    let mut tot_apx = 0u64;
    let mut tot_lock = 0u64;
    let mut tot_loop = 0u64;
    let mut tot_rwx = 0usize;
    for r in reports {
        tot_ld += r.ldmxcsr_count;
        tot_st += r.stmxcsr_count;
        tot_x87 += r.x87_op_count + r.x87_ctrl_count;
        tot_nrw += r.avx_narrow_count;
        tot_apx += r.approx_fp_count;
        tot_lock += r.lock_rmw_count;
        tot_loop += r.loop_branch_count;
        tot_rwx += r.rwx_sections.len();
        out.push_str(&format!(
            "  {:30} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>6} {:>4}\n",
            r.binary_name,
            r.ldmxcsr_count,
            r.stmxcsr_count,
            r.x87_op_count + r.x87_ctrl_count,
            r.avx_narrow_count,
            r.approx_fp_count,
            r.lock_rmw_count,
            r.loop_branch_count,
            r.rwx_sections.len(),
        ));
        if !r.ldmxcsr_sample_rvas.is_empty() {
            let rvas: Vec<String> = r
                .ldmxcsr_sample_rvas
                .iter()
                .map(|v| format!("+0x{v:x}"))
                .collect();
            out.push_str(&format!("    ldmxcsr sites : {}\n", rvas.join(", ")));
        }
        if !r.rwx_sections.is_empty() {
            out.push_str(&format!(
                "    RWX sections  : {}\n",
                r.rwx_sections.join(", ")
            ));
        }
        if !r.notable_imports.is_empty() {
            let imps: Vec<&str> = r.notable_imports.iter().map(|s| s.as_str()).collect();
            out.push_str(&format!("    notable APIs  : {}\n", imps.join(", ")));
        }
    }
    out.push_str(&format!("  {}\n", "-".repeat(96)));
    out.push_str(&format!(
        "  {:30} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>6} {:>4}\n",
        format!("TOTAL ({} binaries)", reports.len()),
        tot_ld,
        tot_st,
        tot_x87,
        tot_nrw,
        tot_apx,
        tot_lock,
        tot_loop,
        tot_rwx,
    ));
    out
}

/// Options for building a `PORT_NAN_TRACE` environment specification.
#[derive(Debug, Clone, Default)]
pub struct NanTraceSpec {
    pub modules: Vec<String>,
    pub ranges: Vec<String>,
    pub watch_mem: Option<String>,
    pub host_filter: Option<String>,
    pub dump_dir: Option<PathBuf>,
    pub log_file: Option<PathBuf>,
    pub trace_stores: bool,
    pub max_events: Option<usize>,
    pub start_armed: bool,
}

impl NanTraceSpec {
    /// Format as a valid `PORT_NAN_TRACE` env value accepted by `x86-runtime::diag`.
    pub fn to_env_string(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        if !self.modules.is_empty() {
            parts.push(format!("module={}", self.modules.join(",")));
        }
        if !self.ranges.is_empty() {
            parts.push(format!("range={}", self.ranges.join(",")));
        }
        if let Some(ref w) = self.watch_mem {
            parts.push(format!("watch={w}"));
        }
        if let Some(ref h) = self.host_filter {
            parts.push(format!("host={h}"));
        }
        if let Some(ref dd) = self.dump_dir {
            parts.push(format!("dump_dir={}", dd.display()));
        }
        if let Some(ref lf) = self.log_file {
            parts.push(format!("log={}", lf.display()));
        }
        if self.trace_stores {
            parts.push("stores=1".to_string());
        }
        if let Some(m) = self.max_events {
            parts.push(format!("max={m}"));
        }
        if !self.start_armed {
            parts.push("armed=0".to_string());
        }
        if parts.is_empty() {
            "1".to_string()
        } else {
            parts.join(";")
        }
    }
}

/// Locate a pre-built `x86-verify` binary or build it in release mode via Cargo.
pub fn find_or_build_x86_verify(workspace_root: &Path, force_build: bool) -> Result<PathBuf> {
    if !force_build {
        if let Some(p) = std::env::var_os("PORT_X86_VERIFY").filter(|v| !v.is_empty()) {
            let pb = PathBuf::from(p);
            if pb.is_file() {
                return Ok(pb);
            }
        }
        let candidates = [
            workspace_root.join("target/aarch64-apple-darwin/release/x86-verify"),
            workspace_root.join("target/release/x86-verify"),
            workspace_root.join("target/aarch64-apple-darwin/debug/x86-verify"),
            workspace_root.join("target/debug/x86-verify"),
        ];
        let mut existing: Vec<(PathBuf, std::time::SystemTime)> = candidates
            .into_iter()
            .filter(|p| p.is_file())
            .map(|p| {
                let mt = fs::metadata(&p)
                    .and_then(|m| m.modified())
                    .unwrap_or(std::time::UNIX_EPOCH);
                (p, mt)
            })
            .collect();
        if !existing.is_empty() {
            existing.sort_by_key(|(_, mt)| std::cmp::Reverse(*mt));
            return Ok(existing.remove(0).0);
        }
    }

    let status = std::process::Command::new("cargo")
        .current_dir(workspace_root)
        .args([
            "build",
            "--release",
            "-p",
            "x86-runtime",
            "--bin",
            "x86-verify",
            "--target",
            "aarch64-apple-darwin",
        ])
        .status()
        .context("building x86-verify via cargo")?;
    if !status.success() {
        bail!("failed to build x86-verify (cargo exited with {status})");
    }
    for rel in [
        "target/aarch64-apple-darwin/release/x86-verify",
        "target/release/x86-verify",
    ] {
        let p = workspace_root.join(rel);
        if p.is_file() {
            return Ok(p);
        }
    }
    bail!("x86-verify binary not found after cargo build")
}

/// Resolve a helper script under `tools/debug/<name>`.
pub fn resolve_debug_script(workspace_root: &Path, script_name: &str) -> Result<PathBuf> {
    let p = workspace_root.join("tools/debug").join(script_name);
    if !p.is_file() {
        bail!("toolkit script not found at {}", p.display());
    }
    Ok(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal valid PE32+ image containing raw x86-64 code bytes in a `.text` section
    /// (optionally RWX) so `audit_pe_bytes_runtime_hazards` can be unit-tested offline.
    fn make_minimal_pe(code: &[u8], rwx: bool) -> Vec<u8> {
        let mut buf = vec![0u8; 0x1000];
        // DOS header
        buf[0..2].copy_from_slice(b"MZ");
        buf[0x3C..0x40].copy_from_slice(&0x80u32.to_le_bytes());
        // PE signature
        buf[0x80..0x84].copy_from_slice(b"PE\0\0");
        // COFF FileHeader (20 bytes @ 0x84)
        buf[0x84..0x86].copy_from_slice(&0x8664u16.to_le_bytes()); // Machine = AMD64
        buf[0x86..0x88].copy_from_slice(&1u16.to_le_bytes()); // NumberOfSections = 1
        buf[0x94..0x96].copy_from_slice(&0xF0u16.to_le_bytes()); // SizeOfOptionalHeader = 240
        buf[0x96..0x98].copy_from_slice(&0x0022u16.to_le_bytes()); // Executable | LargeAddressAware
        // OptionalHeader PE32+ (240 bytes @ 0x98)
        buf[0x98..0x9A].copy_from_slice(&0x020Bu16.to_le_bytes()); // Magic = PE32+
        buf[0xB0..0xB8].copy_from_slice(&0x1_4000_0000u64.to_le_bytes()); // ImageBase
        buf[0xB8..0xBC].copy_from_slice(&0x1000u32.to_le_bytes()); // SectionAlignment
        buf[0xBC..0xC0].copy_from_slice(&0x200u32.to_le_bytes()); // FileAlignment
        buf[0xD0..0xD4].copy_from_slice(&0x2000u32.to_le_bytes()); // SizeOfImage
        buf[0xD4..0xD8].copy_from_slice(&0x200u32.to_le_bytes()); // SizeOfHeaders
        buf[0x104..0x108].copy_from_slice(&16u32.to_le_bytes()); // NumberOfRvaAndSizes
        // SectionHeader `.text` (40 bytes @ 0x188)
        let sh = 0x98 + 0xF0;
        buf[sh..sh + 5].copy_from_slice(b".text");
        buf[sh + 8..sh + 12].copy_from_slice(&(code.len() as u32).to_le_bytes()); // VirtualSize
        buf[sh + 12..sh + 16].copy_from_slice(&0x1000u32.to_le_bytes()); // VirtualAddress
        buf[sh + 16..sh + 20].copy_from_slice(&0x200u32.to_le_bytes()); // SizeOfRawData
        buf[sh + 20..sh + 24].copy_from_slice(&0x200u32.to_le_bytes()); // PointerToRawData
        let chars: u32 = if rwx { 0xE000_0020 } else { 0x6000_0020 };
        buf[sh + 36..sh + 40].copy_from_slice(&chars.to_le_bytes());
        buf[0x200..0x200 + code.len()].copy_from_slice(code);
        buf
    }

    #[test]
    fn runtime_hazard_audit_detects_mxcsr_avx_narrow_approx_lock_loop_and_rwx() {
        // Assemble representative hazard instructions:
        //   0F AE 10          ldmxcsr [rax]
        //   0F AE 18          stmxcsr [rax]
        //   C5 FD E6 C1       vcvtpd2dq xmm0, ymm1
        //   0F 52 C1          rsqrtps xmm0, xmm1
        //   F0 48 0F C1 08    lock xadd [rax], rcx
        //   E1 FE             loope $-2
        //   D9 38             fnstcw [rax]
        //   C3                ret
        let code: &[u8] = &[
            0x0F, 0xAE, 0x10,
            0x0F, 0xAE, 0x18,
            0xC5, 0xFD, 0xE6, 0xC1,
            0x0F, 0x52, 0xC1,
            0xF0, 0x48, 0x0F, 0xC1, 0x08,
            0xE1, 0xFE,
            0xD9, 0x38,
            0xC3,
        ];
        let pe = make_minimal_pe(code, true);
        let rep = audit_pe_bytes_runtime_hazards("synthetic_test.dll", &pe).unwrap();
        assert_eq!(rep.ldmxcsr_count, 1);
        assert_eq!(rep.stmxcsr_count, 1);
        assert_eq!(rep.ldmxcsr_sample_rvas, vec![0x1000]);
        assert_eq!(rep.avx_narrow_count, 1);
        assert_eq!(rep.approx_fp_count, 1);
        assert_eq!(rep.lock_rmw_count, 1);
        assert_eq!(rep.loop_branch_count, 1);
        assert_eq!(rep.x87_ctrl_count, 1);
        assert_eq!(rep.rwx_sections, vec![".text".to_string()]);
        assert!(rep.has_mxcsr_control());

        let txt = format_hazard_reports(&[rep]);
        assert!(txt.contains("synthetic_test.dll"));
        assert!(txt.contains("ldmxcsr sites : +0x1000"));
        assert!(txt.contains("RWX sections  : .text"));
    }

    #[test]
    fn nan_trace_spec_formats_cleanly() {
        let mut s = NanTraceSpec {
            start_armed: true,
            ..NanTraceSpec::default()
        };
        assert_eq!(s.to_env_string(), "1");
        s.modules.push("game.exe".into());
        s.dump_dir = Some(PathBuf::from("out/snaps"));
        s.log_file = Some(PathBuf::from("out/nan.jsonl"));
        s.trace_stores = true;
        s.max_events = Some(500);
        s.start_armed = false;
        assert_eq!(
            s.to_env_string(),
            "module=game.exe;dump_dir=out/snaps;log=out/nan.jsonl;stores=1;max=500;armed=0"
        );
    }
}
