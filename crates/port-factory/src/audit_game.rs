//! Unified single-pass offline game readiness audit & manifest initializer (`audit-game`).
//!
//! Replaces running seven separate `cargo opsdb` passes (`scan`, `win32-audit`,
//! `anticheat-audit`, `dotnet-audit`, `app-audit`, `game-profiles`, `runtime-audit`)
//! and hand-copying `games/_template/game.toml`.
//!
//! Reads and parses each PE32+ binary in the target game directory **once** (`pe_parse_count
//! == binaries_scanned`), aggregates:
//! - Win32 / CRT / middleware imports broken down honestly by [`ImplKind`]
//!   (`real`, `partial`, `stub`, `failing`, sibling PE, and `unresolved`)
//! - Graphics APIs, upscalers, embedded shader containers, and closest engine profile
//! - Anti-cheat / anti-tamper findings (direct syscalls, SSNs, RWX/packed sections, driver APIs)
//! - ARM64 AOT instruction coverage estimate and top unmapped mnemonics
//! - `.NET` / CLR / Mono / IL2CPP and static FP/MXCSR/x87/LOCK/RWX runtime hazards
//! - A pre-filled declarative `game.toml` manifest with detected `[middleware.*]` entries

use crate::anticheat::{
    audit_pe_anticheat_parsed, lookup_ssn_name, AntiCheatVerdict,
};
use crate::diag_toolkit::audit_pe_parsed_runtime_hazards;
use crate::dotnet::{inspect_pe_dotnet_parsed, DotNetRuntimeKind};
use crate::game_profiles::{detect_pe_game_profile, GameEngineProfile};
use crate::gpu_shaders::audit_pe_app_capabilities_parsed;
use crate::scanner::{collect_pe_files, scan_pe_parsed, BinaryProfile};
use crate::win32_db::{audit_pe_imports_parsed, canonical_dll, ImplKind, Win32Database};
use anyhow::{bail, Context, Result};
use pe_image::{ClrExecutionMode, ExportTarget, Pe};
use port_manifest::{
    detect_middleware_from_dlls, GameManifest, MiddlewareSelection,
};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

/// Options controlling [`audit_game_dir`].
#[derive(Debug, Clone, Default)]
pub struct AuditGameOptions {
    /// Optional explicit main executable path (absolute or relative to `game_dir`).
    pub main_exe: Option<PathBuf>,
    /// Optional override for `[app].id` in the generated manifest.
    pub game_id: Option<String>,
    /// Optional override for `[app].name` in the generated manifest.
    pub game_name: Option<String>,
}

/// Overall offline readiness classification for a game directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadinessLevel {
    /// Zero unresolved imports, zero stub/failing imports used, and no kernel-driver blocker.
    Ready,
    /// Bootable / bring-up candidate with remaining stub/failing or unresolved imports to triage.
    NeedsWork,
    /// Hard blocker detected (e.g. Ring-0 kernel anti-cheat driver loader imports).
    Blocked,
}

impl ReadinessLevel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "READY",
            Self::NeedsWork => "NEEDS_WORK",
            Self::Blocked => "BLOCKED",
        }
    }
}

/// Deduplicated non-real (`stub`, `failing`, or `partial`) Win32 import used by one or more binaries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonRealImportUse {
    pub dll: String,
    pub symbol: String,
    pub impl_kind: ImplKind,
    pub source_file: String,
    pub used_by: Vec<String>,
}

/// Deduplicated unresolved import used by one or more binaries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnresolvedImportUse {
    pub dll: String,
    pub symbol: String,
    pub used_by: Vec<String>,
}

/// Summary of one detected multi-version middleware family.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectedMiddlewareInfo {
    pub family_id: String,
    pub display_name: String,
    pub version: String,
    pub default_mode: String,
    pub matched_dlls: Vec<String>,
}

/// Aggregate import coverage grouped honestly by [`ImplKind`].
#[derive(Debug, Clone, Default)]
pub struct ImportReadinessSummary {
    pub total_imports: usize,
    pub resolved_total: usize,
    pub resolved_real: usize,
    pub resolved_sibling: usize,
    pub resolved_partial: usize,
    pub resolved_stub: usize,
    pub resolved_failing: usize,
    pub unresolved_count: usize,
    pub resolved_pct: f64,
    pub real_pct: f64,
    pub stubbed_imports: Vec<NonRealImportUse>,
    pub partial_imports: Vec<NonRealImportUse>,
    pub unresolved_imports: Vec<UnresolvedImportUse>,
    pub detected_middleware: Vec<DetectedMiddlewareInfo>,
}

/// Aggregate graphics API, upscaler, shader container, and engine profile detection.
#[derive(Debug, Clone, Default)]
pub struct GraphicsReadinessSummary {
    pub detected_apis: Vec<String>,
    pub upscalers_detected: Vec<String>,
    pub gpu_backends_detail: BTreeMap<String, Vec<String>>,
    pub embedded_shaders_total: usize,
    pub embedded_shaders_dxil: usize,
    pub embedded_shaders_sm4_5: usize,
    pub embedded_shaders_spirv: usize,
    pub engine_profile_id: Option<String>,
    pub engine_profile_family: Option<String>,
}

/// Aggregate anti-cheat, anti-tamper, and packer findings across all binaries.
#[derive(Debug, Clone)]
pub struct AntiTamperSummary {
    pub worst_verdict: AntiCheatVerdict,
    pub direct_syscalls: u64,
    pub decoded_ssns: Vec<(u32, Option<&'static str>, u64)>,
    pub ud2_count: u64,
    pub cpuid_count: u64,
    pub rdtsc_count: u64,
    pub gs_teb_peb_reads: u64,
    pub kuser_shared_reads: u64,
    pub rwx_sections: Vec<String>,
    pub high_entropy_exec_sections: Vec<String>,
    pub anti_debug_imports: Vec<String>,
    pub veh_seh_imports: Vec<String>,
    pub memory_unpack_imports: Vec<String>,
    pub kernel_driver_imports: Vec<String>,
}

/// Aggregate x86-64 -> ARM64 AOT instruction coverage estimate across all binaries.
#[derive(Debug, Clone, Default)]
pub struct AotCoverageSummary {
    pub total_instructions: u64,
    pub active_instructions: u64,
    pub jit_arm64_instructions: u64,
    pub trap_boundary_instructions: u64,
    pub unmapped_instructions: u64,
    pub jit_coverage_pct: f64,
    pub pdata_functions: usize,
    pub total_code_bytes: u64,
    pub top_unmapped_mnemonics: Vec<(String, u64)>,
}

/// Aggregate `.NET` / CLR / Mono / IL2CPP findings.
#[derive(Debug, Clone, Default)]
pub struct DotNetReadinessSummary {
    pub pure_il_count: usize,
    pub r2r_count: usize,
    pub mixed_mode_count: usize,
    pub apphost_count: usize,
    pub unity_mono_count: usize,
    pub unity_il2cpp_count: usize,
    pub native_non_dotnet_count: usize,
    pub runtime_configs: Vec<String>,
    pub global_metadata_dat: Vec<String>,
}

/// Aggregate static runtime hazard counts across all binaries.
#[derive(Debug, Clone, Default)]
pub struct RuntimeHazardsSummary {
    pub ldmxcsr_count: u64,
    pub stmxcsr_count: u64,
    pub x87_op_count: u64,
    pub avx_narrow_count: u64,
    pub approx_fp_count: u64,
    pub lock_rmw_count: u64,
    pub loop_branch_count: u64,
    pub rwx_section_count: usize,
    pub notable_imports: Vec<String>,
    pub v8_or_cohtml_detected: bool,
}

/// Complete single-pass offline readiness report for a Windows game directory.
#[derive(Debug, Clone)]
pub struct GameAuditReport {
    pub game_dir: PathBuf,
    pub main_exe: Option<String>,
    pub binaries_scanned: usize,
    pub exe_count: usize,
    pub dll_count: usize,
    pub pe_parse_count: usize,
    pub steam_app_id: Option<String>,
    pub readiness: ReadinessLevel,
    pub blockers: Vec<String>,
    pub imports: ImportReadinessSummary,
    pub graphics: GraphicsReadinessSummary,
    pub anti_tamper: AntiTamperSummary,
    pub aot_coverage: AotCoverageSummary,
    pub dotnet: DotNetReadinessSummary,
    pub runtime_hazards: RuntimeHazardsSummary,
    pub profiles: Vec<BinaryProfile>,
    pub suggested_manifest: GameManifest,
    pub suggested_manifest_toml: String,
}

impl GameAuditReport {
    /// Suggested workspace path `games/<id>/game.toml` for `--init-manifest`.
    pub fn default_manifest_path(&self, workspace_root: &Path) -> PathBuf {
        let id = self
            .suggested_manifest
            .id
            .as_deref()
            .unwrap_or("my-game");
        workspace_root.join("games").join(id).join("game.toml")
    }

    /// Write the pre-filled `game.toml` to `out_path`. Refuses to overwrite an existing
    /// file unless `force` is true.
    pub fn write_manifest(&self, out_path: &Path, force: bool) -> Result<()> {
        if out_path.exists() && !force {
            bail!(
                "manifest already exists at {} (pass --force to overwrite)",
                out_path.display()
            );
        }
        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("create manifest directory {}", parent.display()))?;
        }
        fs::write(out_path, &self.suggested_manifest_toml)
            .with_context(|| format!("write manifest {}", out_path.display()))?;
        Ok(())
    }

    /// Render a concise, honest, human-readable readiness summary.
    pub fn format_human_report(&self, manifest_written_to: Option<&Path>) -> String {
        let mut out = String::new();
        let _ = writeln!(
            out,
            "================================================================================"
        );
        let _ = writeln!(
            out,
            "=== Port Factory Single-Pass Game Readiness Audit [{}] ===",
            self.readiness.as_str()
        );
        let _ = writeln!(
            out,
            "================================================================================"
        );
        let _ = writeln!(out, "  Game Directory   : {}", self.game_dir.display());
        let _ = writeln!(
            out,
            "  Main Executable  : {}",
            self.main_exe.as_deref().unwrap_or("<none detected>")
        );
        let _ = writeln!(
            out,
            "  Binaries Scanned : {} ({} EXE, {} DLL/plugin; {} PE parse pass(es))",
            self.binaries_scanned, self.exe_count, self.dll_count, self.pe_parse_count
        );
        if let Some(ref appid) = self.steam_app_id {
            let _ = writeln!(out, "  Steam AppID      : {appid}");
        }
        if let Some(ref prof_id) = self.graphics.engine_profile_id {
            let fam = self
                .graphics
                .engine_profile_family
                .as_deref()
                .unwrap_or("Unknown");
            let _ = writeln!(out, "  Engine Profile   : {prof_id} ({fam})");
        }
        let _ = writeln!(out);

        // 1. Imports by impl_kind
        let imp = &self.imports;
        let _ = writeln!(
            out,
            "-- 1. Win32 / CRT / Middleware Imports ({}/{} resolved = {:.2}%, {}/{} real = {:.2}%) --",
            imp.resolved_total,
            imp.total_imports,
            imp.resolved_pct,
            imp.resolved_real + imp.resolved_sibling,
            imp.total_imports,
            imp.real_pct
        );
        if imp.resolved_sibling > 0 {
            let _ = writeln!(
                out,
                "  real             : {:>6} (+ {} sibling PE)",
                imp.resolved_real, imp.resolved_sibling
            );
        } else {
            let _ = writeln!(out, "  real             : {:>6}", imp.resolved_real);
        }
        let _ = writeln!(out, "  partial          : {:>6}", imp.resolved_partial);
        let _ = writeln!(out, "  stub             : {:>6}", imp.resolved_stub);
        let _ = writeln!(out, "  failing          : {:>6}", imp.resolved_failing);
        let _ = writeln!(out, "  unresolved       : {:>6}", imp.unresolved_count);
        if !imp.detected_middleware.is_empty() {
            let _ = writeln!(
                out,
                "  Middleware ({}):",
                imp.detected_middleware.len()
            );
            for mw in &imp.detected_middleware {
                let _ = writeln!(
                    out,
                    "    {} ({}) v{} [default: {}] <- {}",
                    mw.display_name,
                    mw.family_id,
                    mw.version,
                    mw.default_mode,
                    mw.matched_dlls.join(", ")
                );
            }
        }
        if !imp.stubbed_imports.is_empty() {
            let _ = writeln!(
                out,
                "  Stubbed / Failing Imports Used ({} unique):",
                imp.stubbed_imports.len()
            );
            for s in imp.stubbed_imports.iter().take(100) {
                let _ = writeln!(
                    out,
                    "    {}!{} [{}] ({}) <- {}",
                    s.dll,
                    s.symbol,
                    s.impl_kind.as_str(),
                    s.source_file,
                    s.used_by.join(", ")
                );
            }
        }
        if !imp.partial_imports.is_empty() {
            let _ = writeln!(
                out,
                "  Partial Imports Used ({} unique):",
                imp.partial_imports.len()
            );
            for p in imp.partial_imports.iter().take(40) {
                let _ = writeln!(
                    out,
                    "    {}!{} [partial] ({}) <- {}",
                    p.dll,
                    p.symbol,
                    p.source_file,
                    p.used_by.join(", ")
                );
            }
        }
        if !imp.unresolved_imports.is_empty() {
            let _ = writeln!(
                out,
                "  Unresolved Imports ({} unique):",
                imp.unresolved_imports.len()
            );
            for u in imp.unresolved_imports.iter().take(100) {
                let _ = writeln!(
                    out,
                    "    {}!{} <- {}",
                    u.dll,
                    u.symbol,
                    u.used_by.join(", ")
                );
            }
        }
        let _ = writeln!(out);

        // 2. Graphics API & Shaders
        let gfx = &self.graphics;
        let _ = writeln!(out, "-- 2. Graphics API & Embedded Shaders --");
        let _ = writeln!(
            out,
            "  Graphics APIs    : {}",
            if gfx.detected_apis.is_empty() {
                "None detected".to_string()
            } else {
                gfx.detected_apis.join(", ")
            }
        );
        if !gfx.upscalers_detected.is_empty() {
            let _ = writeln!(
                out,
                "  Upscalers / GPU  : {}",
                gfx.upscalers_detected.join(", ")
            );
        }
        let _ = writeln!(
            out,
            "  Embedded Shaders : {} total ({} DXIL SM6, {} FXC SM4/5, {} Vulkan SPIR-V)",
            gfx.embedded_shaders_total,
            gfx.embedded_shaders_dxil,
            gfx.embedded_shaders_sm4_5,
            gfx.embedded_shaders_spirv
        );
        let _ = writeln!(out);

        // 3. Anti-Tamper & Anti-Cheat
        let ac = &self.anti_tamper;
        let _ = writeln!(out, "-- 3. Anti-Tamper / Anti-Cheat Findings --");
        let _ = writeln!(out, "  Verdict          : {}", ac.worst_verdict.as_str());
        let _ = writeln!(
            out,
            "  Direct Syscalls  : {} ({} distinct SSNs) | cpuid={}, rdtsc/p={}, ud2={}, gs:[TEB/PEB]={}, KUSER_SHARED={}",
            ac.direct_syscalls,
            ac.decoded_ssns.len(),
            ac.cpuid_count,
            ac.rdtsc_count,
            ac.ud2_count,
            ac.gs_teb_peb_reads,
            ac.kuser_shared_reads
        );
        if !ac.rwx_sections.is_empty() {
            let _ = writeln!(out, "  RWX Sections     : {}", ac.rwx_sections.join(", "));
        }
        if !ac.high_entropy_exec_sections.is_empty() {
            let _ = writeln!(
                out,
                "  High-Entropy Code: {}",
                ac.high_entropy_exec_sections.join(", ")
            );
        }
        if !ac.anti_debug_imports.is_empty() {
            let _ = writeln!(
                out,
                "  Anti-Debug APIs  : {}",
                ac.anti_debug_imports.join(", ")
            );
        }
        if !ac.veh_seh_imports.is_empty() {
            let _ = writeln!(out, "  VEH / SEH APIs   : {}", ac.veh_seh_imports.join(", "));
        }
        if !ac.kernel_driver_imports.is_empty() {
            let _ = writeln!(
                out,
                "  Kernel Drivers   : {}",
                ac.kernel_driver_imports.join(", ")
            );
        }
        let _ = writeln!(out);

        // 4. AOT Coverage Estimate
        let aot = &self.aot_coverage;
        let _ = writeln!(out, "-- 4. ARM64 AOT Coverage Estimate --");
        let _ = writeln!(
            out,
            "  JIT Coverage     : {}/{} active instructions ({:.4}%, {} unmapped, {} .pdata funcs, {:.2} MB code)",
            aot.jit_arm64_instructions,
            aot.active_instructions,
            aot.jit_coverage_pct,
            aot.unmapped_instructions,
            aot.pdata_functions,
            aot.total_code_bytes as f64 / (1024.0 * 1024.0)
        );
        if !aot.top_unmapped_mnemonics.is_empty() {
            let top: Vec<String> = aot
                .top_unmapped_mnemonics
                .iter()
                .take(10)
                .map(|(m, c)| format!("{m}:{c}"))
                .collect();
            let _ = writeln!(out, "  Top Unmapped Ops : {}", top.join(", "));
        }
        let _ = writeln!(out);

        // 5. .NET & Runtime Hazards
        let rh = &self.runtime_hazards;
        let dn = &self.dotnet;
        let _ = writeln!(out, "-- 5. .NET / CLR & Static Runtime Hazards --");
        let _ = writeln!(
            out,
            "  .NET / Unity     : native={}, pure_il={}, r2r={}, mixed={}, apphost={}, mono={}, il2cpp={}",
            dn.native_non_dotnet_count,
            dn.pure_il_count,
            dn.r2r_count,
            dn.mixed_mode_count,
            dn.apphost_count,
            dn.unity_mono_count,
            dn.unity_il2cpp_count
        );
        let _ = writeln!(
            out,
            "  Runtime Hazards  : ldmxcsr={}, stmxcsr={}, x87={}, avx_narrow={}, approx_fp={}, lock_rmw={}, loopcc={}, rwx_sections={}",
            rh.ldmxcsr_count,
            rh.stmxcsr_count,
            rh.x87_op_count,
            rh.avx_narrow_count,
            rh.approx_fp_count,
            rh.lock_rmw_count,
            rh.loop_branch_count,
            rh.rwx_section_count
        );
        if rh.v8_or_cohtml_detected {
            let _ = writeln!(
                out,
                "  V8 / Cohtml Note : detected embedded V8/Cohtml (`v8_jitless = true` pre-filled)"
            );
        }
        if !self.blockers.is_empty() {
            let _ = writeln!(out);
            let _ = writeln!(out, "-- Blockers & Action Items ({}) --", self.blockers.len());
            for b in &self.blockers {
                let _ = writeln!(out, "  * {b}");
            }
        }
        if let Some(path) = manifest_written_to {
            let _ = writeln!(out);
            let _ = writeln!(out, "  Pre-filled manifest written to: {}", path.display());
        }
        out
    }
}

/// Audit a Windows game directory in **one single pass** over its PE32+ files and synthesize a
/// pre-filled `game.toml` manifest.
pub fn audit_game_dir(
    game_dir: &Path,
    db: &Win32Database,
    opts: &AuditGameOptions,
) -> Result<GameAuditReport> {
    if !game_dir.exists() {
        bail!("game directory not found: {}", game_dir.display());
    }

    let pe_files = collect_pe_files(game_dir)?;
    if pe_files.is_empty() {
        bail!(
            "no valid x86-64 PE32+ binaries (.exe/.dll/.pyd/.node/.ocx) found in {}",
            game_dir.display()
        );
    }

    // Lightweight auxiliary metadata walk (no PE parsing): steam_appid.txt, .runtimeconfig.json, global-metadata.dat
    let mut steam_app_id = read_steam_appid(game_dir);
    let mut runtime_configs = Vec::new();
    let mut global_metadata_dat = Vec::new();
    if game_dir.is_dir() {
        collect_aux_metadata(
            game_dir,
            game_dir,
            &mut steam_app_id,
            &mut runtime_configs,
            &mut global_metadata_dat,
        );
    }

    let main_exe_abs = resolve_main_exe(game_dir, &pe_files, opts.main_exe.as_deref());
    let main_exe_rel = main_exe_abs.as_ref().map(|p| rel_display(game_dir, p));

    // Single pass: open, read, and `Pe::parse` each PE file EXACTLY ONCE.
    let mut pe_parse_count = 0usize;
    let mut exe_count = 0usize;
    let mut dll_count = 0usize;

    let mut sibling_exports: HashMap<(String, String), u32> = HashMap::new();
    let mut per_binary_audits = Vec::with_capacity(pe_files.len());
    let mut profiles = Vec::with_capacity(pe_files.len());
    let mut all_dll_names: BTreeSet<String> = BTreeSet::new();

    // Graphics accumulators
    let mut detected_apis: BTreeSet<String> = BTreeSet::new();
    let mut upscalers_detected: BTreeSet<String> = BTreeSet::new();
    let mut gpu_backends_detail: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut embedded_shaders_total = 0usize;
    let mut embedded_shaders_dxil = 0usize;
    let mut embedded_shaders_sm4_5 = 0usize;
    let mut embedded_shaders_spirv = 0usize;
    let mut main_engine_profile: Option<&'static GameEngineProfile> = None;

    // Anti-tamper accumulators
    let mut worst_verdict = AntiCheatVerdict::CompatibleClean;
    let mut direct_syscalls = 0u64;
    let mut decoded_ssns: BTreeMap<u32, u64> = BTreeMap::new();
    let mut ud2_count = 0u64;
    let mut cpuid_count = 0u64;
    let mut rdtsc_count = 0u64;
    let mut gs_teb_peb_reads = 0u64;
    let mut kuser_shared_reads = 0u64;
    let mut rwx_sections = Vec::new();
    let mut high_entropy_exec_sections = Vec::new();
    let mut anti_debug_imports: BTreeSet<String> = BTreeSet::new();
    let mut veh_seh_imports: BTreeSet<String> = BTreeSet::new();
    let mut memory_unpack_imports: BTreeSet<String> = BTreeSet::new();
    let mut kernel_driver_imports: BTreeSet<String> = BTreeSet::new();

    // AOT coverage accumulators
    let mut total_instructions = 0u64;
    let mut jit_arm64_instructions = 0u64;
    let mut trap_boundary_instructions = 0u64;
    let mut unmapped_instructions = 0u64;
    let mut pdata_functions = 0usize;
    let mut total_code_bytes = 0u64;
    let mut unmapped_mnemonics: BTreeMap<String, u64> = BTreeMap::new();

    // .NET accumulators
    let mut dotnet = DotNetReadinessSummary {
        runtime_configs,
        global_metadata_dat,
        ..Default::default()
    };

    // Runtime hazards accumulators
    let mut runtime_hazards = RuntimeHazardsSummary::default();
    let mut notable_imports: BTreeSet<String> = BTreeSet::new();

    for path in &pe_files {
        let bytes = match fs::read(path) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let pe = match Pe::parse(&bytes) {
            Ok(p) => p,
            Err(_) => continue,
        };
        pe_parse_count += 1;

        let file_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown.pe")
            .to_string();
        let norm_self = pe_image::normalize_dll_name(&file_name);
        if pe.headers.is_dll() {
            dll_count += 1;
            all_dll_names.insert(norm_self.clone());
        } else {
            exe_count += 1;
        }
        if is_v8_or_cohtml_dll_name(&norm_self) {
            runtime_hazards.v8_or_cohtml_detected = true;
        }

        // Collect exports for post-pass sibling import resolution.
        if let Ok(exps) = pe.exports() {
            for e in exps.entries {
                let rva = match e.target {
                    ExportTarget::Rva(r) => r,
                    ExportTarget::Forwarder(_) => 0,
                };
                if let Some(n) = e.name {
                    sibling_exports.insert((norm_self.clone(), n), rva);
                }
                sibling_exports.insert((norm_self.clone(), format!("#{}", e.ordinal)), rva);
            }
        }

        // 1. Instruction & AOT coverage scan (reuses `&pe`)
        let prof = scan_pe_parsed(&file_name, &pe);
        total_instructions += prof.total_instructions;
        jit_arm64_instructions += prof.jit_arm64_instructions;
        trap_boundary_instructions += prof.trap_boundary_instructions;
        unmapped_instructions += prof.unmapped_instructions;
        pdata_functions += prof.functions_count;
        total_code_bytes += prof.total_code_bytes;
        for (mn, cnt) in &prof.unmapped_mnemonics {
            *unmapped_mnemonics.entry(mn.clone()).or_insert(0) += *cnt;
        }
        profiles.push(prof);

        // 2. Win32 imports audit (reuses `&pe`; sibling resolution finalized after loop)
        let imp_audit = audit_pe_imports_parsed(&file_name, &pe, db, None);
        for imp in pe.imports().unwrap_or_default() {
            let norm = pe_image::normalize_dll_name(&imp.dll);
            record_graphics_and_v8_dll(
                &norm,
                &mut detected_apis,
                &mut upscalers_detected,
                &mut runtime_hazards.v8_or_cohtml_detected,
            );
            all_dll_names.insert(norm);
        }
        if let Ok(dimps) = pe.delay_imports() {
            for imp in dimps {
                let norm = pe_image::normalize_dll_name(&imp.dll);
                record_graphics_and_v8_dll(
                    &norm,
                    &mut detected_apis,
                    &mut upscalers_detected,
                    &mut runtime_hazards.v8_or_cohtml_detected,
                );
                all_dll_names.insert(norm);
            }
        }
        per_binary_audits.push(imp_audit);

        // 3. Anti-cheat & anti-tamper audit (reuses `&pe`)
        let ac = audit_pe_anticheat_parsed(&file_name, &pe);
        if verdict_severity(ac.verdict) > verdict_severity(worst_verdict) {
            worst_verdict = ac.verdict;
        }
        direct_syscalls += ac.direct_syscalls;
        for (ssn, cnt) in ac.decoded_ssns {
            *decoded_ssns.entry(ssn).or_insert(0) += cnt;
        }
        ud2_count += ac.ud2_count;
        cpuid_count += ac.cpuid_count;
        rdtsc_count += ac.rdtsc_count;
        gs_teb_peb_reads += ac.gs_teb_peb_reads;
        kuser_shared_reads += ac.kuser_shared_reads;
        for sec in &ac.sections {
            if sec.is_exec && sec.is_write {
                rwx_sections.push(format!("{file_name}:{}", sec.name));
            } else if sec.is_exec && sec.entropy >= 7.25 {
                high_entropy_exec_sections.push(format!(
                    "{file_name}:{} ({:.2} b/B)",
                    sec.name, sec.entropy
                ));
            }
        }
        anti_debug_imports.extend(ac.anti_debug_imports);
        veh_seh_imports.extend(ac.veh_seh_imports);
        memory_unpack_imports.extend(ac.memory_unpack_imports);
        kernel_driver_imports.extend(ac.kernel_driver_imports);

        // 4. Graphics & embedded shader capability audit (reuses `&pe` and `&bytes`)
        let cap = audit_pe_app_capabilities_parsed(&file_name, &pe, &bytes);
        embedded_shaders_total += cap.embedded_shaders_total;
        embedded_shaders_dxil += cap.embedded_shaders_dxil;
        embedded_shaders_sm4_5 += cap.embedded_shaders_sm4_5;
        embedded_shaders_spirv += cap.embedded_shaders_spirv;
        for (backend, apis) in cap.gpu_apis_by_backend {
            gpu_backends_detail.entry(backend).or_default().extend(apis);
        }

        // 5. Engine profile detection on main EXE (or first EXE)
        if main_exe_abs.as_ref().is_some_and(|m| m == path)
            || (main_engine_profile.is_none() && !pe.headers.is_dll())
        {
            main_engine_profile = detect_pe_game_profile(&pe);
        }

        // 6. .NET / CLR / Unity audit (reuses `&pe` and `&bytes`)
        if let Ok(dn) = inspect_pe_dotnet_parsed(path, Some(&pe), &bytes) {
            if let Some(mode) = dn.clr_mode {
                match mode {
                    ClrExecutionMode::PureIlAnyCpu | ClrExecutionMode::Il32BitOnly => {
                        dotnet.pure_il_count += 1;
                    }
                    ClrExecutionMode::ReadyToRunHybrid => dotnet.r2r_count += 1,
                    ClrExecutionMode::MixedModeX64 => dotnet.mixed_mode_count += 1,
                }
            } else {
                match dn.kind {
                    DotNetRuntimeKind::DotNetAppHostLauncher => dotnet.apphost_count += 1,
                    DotNetRuntimeKind::UnityMonoRuntime => dotnet.unity_mono_count += 1,
                    DotNetRuntimeKind::UnityIl2CppNative => dotnet.unity_il2cpp_count += 1,
                    DotNetRuntimeKind::NetFrameworkManaged
                    | DotNetRuntimeKind::ModernCoreClrManaged => dotnet.mixed_mode_count += 1,
                    DotNetRuntimeKind::NativeNonDotNet => dotnet.native_non_dotnet_count += 1,
                }
            }
        }

        // 7. Static runtime hazards audit (reuses `&pe`)
        let rh = audit_pe_parsed_runtime_hazards(&file_name, &pe);
        runtime_hazards.ldmxcsr_count += rh.ldmxcsr_count;
        runtime_hazards.stmxcsr_count += rh.stmxcsr_count;
        runtime_hazards.x87_op_count += rh.x87_op_count + rh.x87_ctrl_count;
        runtime_hazards.avx_narrow_count += rh.avx_narrow_count;
        runtime_hazards.approx_fp_count += rh.approx_fp_count;
        runtime_hazards.lock_rmw_count += rh.lock_rmw_count;
        runtime_hazards.loop_branch_count += rh.loop_branch_count;
        runtime_hazards.rwx_section_count += rh.rwx_sections.len();
        notable_imports.extend(rh.notable_imports);
    }

    if pe_parse_count == 0 {
        bail!(
            "failed to parse any valid PE32+ binary in {}",
            game_dir.display()
        );
    }

    // Finalize import readiness by reconciling unresolved imports against sibling PE exports.
    let mut total_imports = 0usize;
    let mut resolved_real = 0usize;
    let mut resolved_sibling = 0usize;
    let mut resolved_partial = 0usize;
    let mut resolved_stub = 0usize;
    let mut resolved_failing = 0usize;
    let mut unresolved_count = 0usize;

    let mut stubbed_map: BTreeMap<(String, String), (ImplKind, String, BTreeSet<String>)> =
        BTreeMap::new();
    let mut partial_map: BTreeMap<(String, String), (ImplKind, String, BTreeSet<String>)> =
        BTreeMap::new();
    let mut unresolved_map: BTreeMap<(String, String), BTreeSet<String>> = BTreeMap::new();

    for audit in &per_binary_audits {
        total_imports += audit.total_imports;
        resolved_real += audit.count_by_impl_kind(ImplKind::Real);
        resolved_partial += audit.count_by_impl_kind(ImplKind::Partial);
        resolved_stub += audit.count_by_impl_kind(ImplKind::Stub);
        resolved_failing += audit.count_by_impl_kind(ImplKind::Failing);

        for s in &audit.stubbed_imports {
            stubbed_map
                .entry((s.dll.clone(), s.symbol.clone()))
                .or_insert_with(|| (s.impl_kind, s.source_file.clone(), BTreeSet::new()))
                .2
                .insert(audit.binary_name.clone());
        }
        for p in &audit.partial_imports {
            partial_map
                .entry((p.dll.clone(), p.symbol.clone()))
                .or_insert_with(|| (p.impl_kind, p.source_file.clone(), BTreeSet::new()))
                .2
                .insert(audit.binary_name.clone());
        }
        for (dll, sym) in &audit.unresolved {
            if sibling_exports.contains_key(&(dll.clone(), sym.clone())) {
                resolved_sibling += 1;
            } else {
                unresolved_count += 1;
                unresolved_map
                    .entry((dll.clone(), sym.clone()))
                    .or_default()
                    .insert(audit.binary_name.clone());
            }
        }
    }

    let resolved_total =
        resolved_real + resolved_sibling + resolved_partial + resolved_stub + resolved_failing;
    let resolved_pct = if total_imports == 0 {
        100.0
    } else {
        (resolved_total as f64 * 100.0) / (total_imports as f64)
    };
    let real_pct = if total_imports == 0 {
        100.0
    } else {
        ((resolved_real + resolved_sibling) as f64 * 100.0) / (total_imports as f64)
    };

    let detected_mw_raw = detect_middleware_from_dlls(all_dll_names.iter().map(String::as_str));
    let detected_middleware: Vec<DetectedMiddlewareInfo> = detected_mw_raw
        .iter()
        .map(|mw| DetectedMiddlewareInfo {
            family_id: mw.family.id.to_string(),
            display_name: mw.family.display_name.to_string(),
            version: mw.version.version.to_string(),
            default_mode: mw.family.default_mode.as_str().to_string(),
            matched_dlls: mw.matched_dlls.clone(),
        })
        .collect();

    let imports = ImportReadinessSummary {
        total_imports,
        resolved_total,
        resolved_real,
        resolved_sibling,
        resolved_partial,
        resolved_stub,
        resolved_failing,
        unresolved_count,
        resolved_pct,
        real_pct,
        stubbed_imports: stubbed_map
            .into_iter()
            .map(|((dll, symbol), (impl_kind, source_file, used))| NonRealImportUse {
                dll,
                symbol,
                impl_kind,
                source_file,
                used_by: used.into_iter().collect(),
            })
            .collect(),
        partial_imports: partial_map
            .into_iter()
            .map(|((dll, symbol), (impl_kind, source_file, used))| NonRealImportUse {
                dll,
                symbol,
                impl_kind,
                source_file,
                used_by: used.into_iter().collect(),
            })
            .collect(),
        unresolved_imports: unresolved_map
            .into_iter()
            .map(|((dll, symbol), used)| UnresolvedImportUse {
                dll,
                symbol,
                used_by: used.into_iter().collect(),
            })
            .collect(),
        detected_middleware,
    };

    let graphics = GraphicsReadinessSummary {
        detected_apis: detected_apis.into_iter().collect(),
        upscalers_detected: upscalers_detected.into_iter().collect(),
        gpu_backends_detail: gpu_backends_detail
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().collect()))
            .collect(),
        embedded_shaders_total,
        embedded_shaders_dxil,
        embedded_shaders_sm4_5,
        embedded_shaders_spirv,
        engine_profile_id: main_engine_profile.map(|p| p.profile_id.to_string()),
        engine_profile_family: main_engine_profile.map(|p| p.engine_family.to_string()),
    };

    let anti_tamper = AntiTamperSummary {
        worst_verdict,
        direct_syscalls,
        decoded_ssns: decoded_ssns
            .into_iter()
            .map(|(ssn, cnt)| (ssn, lookup_ssn_name(ssn), cnt))
            .collect(),
        ud2_count,
        cpuid_count,
        rdtsc_count,
        gs_teb_peb_reads,
        kuser_shared_reads,
        rwx_sections,
        high_entropy_exec_sections,
        anti_debug_imports: anti_debug_imports.into_iter().collect(),
        veh_seh_imports: veh_seh_imports.into_iter().collect(),
        memory_unpack_imports: memory_unpack_imports.into_iter().collect(),
        kernel_driver_imports: kernel_driver_imports.into_iter().collect(),
    };

    let active_instructions = total_instructions.saturating_sub(trap_boundary_instructions);
    let jit_coverage_pct = if active_instructions == 0 {
        100.0
    } else {
        (jit_arm64_instructions as f64 * 100.0) / (active_instructions as f64)
    };
    let mut top_unmapped: Vec<(String, u64)> = unmapped_mnemonics.into_iter().collect();
    top_unmapped.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    top_unmapped.truncate(20);

    let aot_coverage = AotCoverageSummary {
        total_instructions,
        active_instructions,
        jit_arm64_instructions,
        trap_boundary_instructions,
        unmapped_instructions,
        jit_coverage_pct,
        pdata_functions,
        total_code_bytes,
        top_unmapped_mnemonics: top_unmapped,
    };

    runtime_hazards.notable_imports = notable_imports.into_iter().collect();

    // Honest readiness verdict & blockers list.
    let mut blockers = Vec::new();
    if !anti_tamper.kernel_driver_imports.is_empty() {
        blockers.push(format!(
            "Imports Ring-0 kernel driver APIs: {}",
            anti_tamper.kernel_driver_imports.join(", ")
        ));
    }
    if imports.unresolved_count > 0 {
        blockers.push(format!(
            "{} unresolved import(s) across {} unique symbol(s)",
            imports.unresolved_count,
            imports.unresolved_imports.len()
        ));
    }
    if imports.resolved_stub + imports.resolved_failing > 0 {
        blockers.push(format!(
            "{} stub/failing Win32 import(s) used ({} stub, {} failing across {} unique symbol(s))",
            imports.resolved_stub + imports.resolved_failing,
            imports.resolved_stub,
            imports.resolved_failing,
            imports.stubbed_imports.len()
        ));
    }
    let readiness = if !anti_tamper.kernel_driver_imports.is_empty() {
        ReadinessLevel::Blocked
    } else if imports.unresolved_count > 0
        || imports.resolved_stub > 0
        || imports.resolved_failing > 0
    {
        ReadinessLevel::NeedsWork
    } else {
        ReadinessLevel::Ready
    };

    // Build pre-filled `GameManifest` and TOML text.
    let suggested_manifest = build_prefilled_manifest(
        game_dir,
        main_exe_rel.as_deref(),
        steam_app_id.as_deref(),
        runtime_hazards.v8_or_cohtml_detected,
        &detected_mw_raw,
        opts,
    );
    let suggested_manifest_toml = render_manifest_with_header(
        &suggested_manifest,
        &graphics,
        &imports,
        &aot_coverage,
        &anti_tamper,
    );

    Ok(GameAuditReport {
        game_dir: game_dir.to_path_buf(),
        main_exe: main_exe_rel,
        binaries_scanned: pe_parse_count,
        exe_count,
        dll_count,
        pe_parse_count,
        steam_app_id,
        readiness,
        blockers,
        imports,
        graphics,
        anti_tamper,
        aot_coverage,
        dotnet,
        runtime_hazards,
        profiles,
        suggested_manifest,
        suggested_manifest_toml,
    })
}

fn verdict_severity(v: AntiCheatVerdict) -> u8 {
    match v {
        AntiCheatVerdict::CompatibleClean => 0,
        AntiCheatVerdict::CompatibleUserModeAc => 1,
        AntiCheatVerdict::CompatibleViaRuntimeUnpack => 2,
        AntiCheatVerdict::RequiresKernelDriverFallback => 3,
    }
}

/// Returns `true` when a raw or normalized Windows DLL/binary name indicates embedded V8,
/// Coherent Gameface (`cohtml`), or Chromium Embedded Framework (`libcef`) — all of which
/// require `v8_jitless = true` so guest JS engines do not emit x86-64 RWX JIT code.
pub fn is_v8_or_cohtml_dll_name(raw_or_norm_dll: &str) -> bool {
    let norm = pe_image::normalize_dll_name(raw_or_norm_dll);
    let canon = canonical_dll(&norm);
    matches!(
        canon.as_str(),
        "cohtml_windowsdesktop.dll" | "cohtml_icuuc.dll"
    ) || norm.contains("cohtml")
        || norm.contains("v8")
        || norm.contains("libcef")
}

/// Detect whether any PE32+ binary in `game_dir` ships or imports V8, Cohtml, or CEF,
/// using the exact same rules as [`audit_game_dir`] so `.app` bundling and `audit-game`
/// always agree on `v8_jitless`.
pub fn detect_v8_or_cohtml_in_game_dir(game_dir: &Path) -> bool {
    let Ok(pe_files) = collect_pe_files(game_dir) else {
        return false;
    };
    // Fast pass 1: check filenames directly (zero file reads when `v8.dll` / `cohtml*.dll` ships).
    for path in &pe_files {
        if let Some(fname) = path.file_name().and_then(|s| s.to_str()) {
            if is_v8_or_cohtml_dll_name(fname) {
                return true;
            }
        }
    }
    // Pass 2: inspect static and delay-load imports of every PE32+ binary.
    for path in &pe_files {
        let Ok(bytes) = fs::read(path) else {
            continue;
        };
        let Ok(pe) = Pe::parse(&bytes) else {
            continue;
        };
        for imp in pe.imports().unwrap_or_default() {
            if is_v8_or_cohtml_dll_name(&imp.dll) {
                return true;
            }
        }
        if let Ok(dimps) = pe.delay_imports() {
            for imp in dimps {
                if is_v8_or_cohtml_dll_name(&imp.dll) {
                    return true;
                }
            }
        }
    }
    false
}

fn record_graphics_and_v8_dll(
    norm_dll: &str,
    detected_apis: &mut BTreeSet<String>,
    upscalers_detected: &mut BTreeSet<String>,
    v8_detected: &mut bool,
) {
    let canon = canonical_dll(norm_dll);
    match canon.as_str() {
        "d3d12.dll" => {
            detected_apis.insert("D3D12".to_string());
        }
        "d3d11.dll" => {
            detected_apis.insert("D3D11".to_string());
        }
        "vulkan_1.dll" => {
            detected_apis.insert("Vulkan".to_string());
        }
        "directml.dll" => {
            detected_apis.insert("DirectML".to_string());
        }
        "opencl.dll" => {
            detected_apis.insert("OpenCL".to_string());
        }
        "nvcuda.dll" => {
            detected_apis.insert("CUDA".to_string());
        }
        "nvngx_dlss.dll" | "sl.interposer.dll" => {
            upscalers_detected.insert("DLSS / Streamline".to_string());
        }
        "amd_fidelityfx_loader_dx12.dll" => {
            upscalers_detected.insert("AMD FSR".to_string());
        }
        "libxess.dll" | "libxess_dx11.dll" => {
            upscalers_detected.insert("Intel XeSS".to_string());
        }
        _ => {}
    }
    if norm_dll.contains("opengl32") {
        detected_apis.insert("OpenGL".to_string());
    }
    if norm_dll.contains("d3d9") {
        detected_apis.insert("D3D9".to_string());
    }
    if is_v8_or_cohtml_dll_name(norm_dll) {
        *v8_detected = true;
    }
}

fn rel_display(root: &Path, path: &Path) -> String {
    if root.is_file() {
        return path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.display().to_string());
    }
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn is_helper_exe(name_lower: &str) -> bool {
    const SKIP: &[&str] = &[
        "unins",
        "crash",
        "crs-handler",
        "crs-video",
        "bugsplat",
        "redist",
        "setup",
        "vcredist",
        "dxsetup",
        "ue4prereq",
        "ueprereq",
        "easyanticheat",
    ];
    SKIP.iter().any(|k| name_lower.contains(k))
}

/// Select the primary `.exe` from discovered PE32+ files in `game_dir`, preferring top-level
/// non-helper executables (excluding crash reporters, installers, and redistributables), then
/// nested non-helper executables, breaking ties by largest file size (then path order).
pub fn select_primary_exe(
    game_dir: &Path,
    pe_files: &[PathBuf],
    explicit: Option<&Path>,
) -> Option<PathBuf> {
    if let Some(exp) = explicit {
        if exp.is_file() {
            return Some(exp.to_path_buf());
        }
        let joined = game_dir.join(exp);
        if joined.is_file() {
            return Some(joined);
        }
    }
    let exes: Vec<&PathBuf> = pe_files
        .iter()
        .filter(|p| {
            p.extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| e.eq_ignore_ascii_case("exe"))
        })
        .collect();
    if exes.is_empty() {
        return None;
    }

    let score = |p: &&PathBuf| -> (u8, u64, std::cmp::Reverse<String>) {
        let name = p
            .file_name()
            .map(|s| s.to_string_lossy().to_ascii_lowercase())
            .unwrap_or_default();
        let is_top_level = p.parent() == Some(game_dir);
        let helper = is_helper_exe(&name);
        let tier = match (is_top_level, helper) {
            (true, false) => 3,
            (false, false) => 2,
            (true, true) => 1,
            (false, true) => 0,
        };
        let size = fs::metadata(p).map(|m| m.len()).unwrap_or(0);
        (tier, size, std::cmp::Reverse(p.to_string_lossy().to_ascii_lowercase()))
    };

    exes.into_iter().max_by_key(score).cloned()
}

fn resolve_main_exe(
    game_dir: &Path,
    pe_files: &[PathBuf],
    explicit: Option<&Path>,
) -> Option<PathBuf> {
    select_primary_exe(game_dir, pe_files, explicit)
}

fn read_steam_appid(dir: &Path) -> Option<String> {
    let cand = if dir.is_file() {
        dir.parent()?.join("steam_appid.txt")
    } else {
        dir.join("steam_appid.txt")
    };
    let text = fs::read_to_string(cand).ok()?;
    let trimmed = text.trim();
    (!trimmed.is_empty() && trimmed.chars().all(|c| c.is_ascii_digit()))
        .then(|| trimmed.to_string())
}

fn collect_aux_metadata(
    root: &Path,
    cur: &Path,
    steam_app_id: &mut Option<String>,
    runtime_configs: &mut Vec<String>,
    global_metadata: &mut Vec<String>,
) {
    let Ok(rd) = fs::read_dir(cur) else {
        return;
    };
    let mut entries: Vec<(PathBuf, fs::FileType)> = rd
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_type().ok().map(|ft| (e.path(), ft)))
        .collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    for (p, ft) in entries {
        if ft.is_symlink() {
            continue;
        }
        if ft.is_dir() {
            if p.file_name()
                .and_then(|s| s.to_str())
                .is_some_and(|n| n.starts_with('.'))
            {
                continue;
            }
            collect_aux_metadata(root, &p, steam_app_id, runtime_configs, global_metadata);
        } else if ft.is_file() {
            let low = p
                .file_name()
                .map(|s| s.to_string_lossy().to_ascii_lowercase())
                .unwrap_or_default();
            if low == "steam_appid.txt" && steam_app_id.is_none() {
                if let Ok(text) = fs::read_to_string(&p) {
                    let t = text.trim();
                    if !t.is_empty() && t.chars().all(|c| c.is_ascii_digit()) {
                        *steam_app_id = Some(t.to_string());
                    }
                }
            } else if low.ends_with(".runtimeconfig.json") {
                runtime_configs.push(rel_display(root, &p));
            } else if low == "global-metadata.dat" {
                global_metadata.push(rel_display(root, &p));
            }
        }
    }
}

fn slugify_kebab(raw: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = false;
    let chars: Vec<char> = raw.chars().collect();
    for (idx, &ch) in chars.iter().enumerate() {
        if ch.is_ascii_alphanumeric() {
            // Insert `-` at camelCase boundary (`SpaceOdyssey` -> `space-odyssey`).
            if ch.is_ascii_uppercase()
                && idx > 0
                && chars[idx - 1].is_ascii_lowercase()
                && !prev_dash
                && !out.is_empty()
            {
                out.push('-');
            }
            out.push(ch.to_ascii_lowercase());
            prev_dash = false;
        } else if !out.is_empty() && !prev_dash {
            out.push('-');
            prev_dash = true;
        }
    }
    let trimmed = out.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "my-game".to_string()
    } else {
        trimmed
    }
}

fn humanize_title(raw: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = raw.chars().collect();
    for (idx, &ch) in chars.iter().enumerate() {
        if ch == '_' || ch == '-' {
            if !out.ends_with(' ') && !out.is_empty() {
                out.push(' ');
            }
        } else if ch.is_ascii_uppercase()
            && idx > 0
            && chars[idx - 1].is_ascii_lowercase()
            && !out.ends_with(' ')
        {
            out.push(' ');
            out.push(ch);
        } else {
            out.push(ch);
        }
    }
    let t = out.trim().to_string();
    if t.is_empty() {
        "My Game".to_string()
    } else {
        t
    }
}

fn build_prefilled_manifest(
    game_dir: &Path,
    main_exe_rel: Option<&str>,
    steam_app_id: Option<&str>,
    v8_jitless: bool,
    detected_mw: &[port_manifest::DetectedMiddleware],
    opts: &AuditGameOptions,
) -> GameManifest {
    let exe_str = main_exe_rel.unwrap_or("Game.exe").to_string();
    let exe_stem = Path::new(&exe_str)
        .file_stem()
        .and_then(|s| s.to_str())
        .filter(|s| !s.is_empty())
        .unwrap_or("Game");
    let dir_name = game_dir
        .file_name()
        .and_then(|s| s.to_str())
        .filter(|s| !s.is_empty() && *s != "." && *s != "..")
        .unwrap_or(exe_stem);

    let id = opts
        .game_id
        .clone()
        .unwrap_or_else(|| slugify_kebab(exe_stem));
    let name = opts
        .game_name
        .clone()
        .unwrap_or_else(|| humanize_title(exe_stem));
    let log_name = format!("{exe_stem}.log");
    let win_folder: String = dir_name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, ' ' | '-' | '_' | '.'))
        .collect();
    let win_folder = if win_folder.trim().is_empty() {
        exe_stem.to_string()
    } else {
        win_folder.trim().to_string()
    };

    let mut middleware = BTreeMap::new();
    for mw in detected_mw {
        middleware.insert(
            mw.family.id.to_string(),
            MiddlewareSelection {
                version: Some(mw.version.version.to_string()),
                mode: Some(mw.family.default_mode.to_dll_route()),
            },
        );
    }

    GameManifest {
        id: Some(id),
        name: Some(name),
        exe: Some(exe_str),
        win_dir: Some(format!("C:\\Games\\{win_folder}")),
        log_name: Some(log_name),
        computer_name: Some("WIN64PORT".to_string()),
        steam_app_id: steam_app_id.map(str::to_string),
        preferred_base_retry: Some(false),
        v8_jitless: Some(v8_jitless),
        legacy_remedy_migration: Some(false),
        middleware,
        ..GameManifest::default()
    }
}

fn render_manifest_with_header(
    manifest: &GameManifest,
    graphics: &GraphicsReadinessSummary,
    imports: &ImportReadinessSummary,
    aot: &AotCoverageSummary,
    anti_tamper: &AntiTamperSummary,
) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "# Pre-filled target manifest generated by `cargo opsdb -- audit-game --init-manifest`."
    );
    let _ = writeln!(
        out,
        "# Graphics APIs : {}",
        if graphics.detected_apis.is_empty() {
            "none detected".to_string()
        } else {
            graphics.detected_apis.join(", ")
        }
    );
    if let Some(ref prof) = graphics.engine_profile_id {
        let _ = writeln!(out, "# Engine Profile: {prof}");
    }
    let _ = writeln!(
        out,
        "# Imports       : {}/{} resolved ({} real, {} sibling, {} partial, {} stub, {} failing, {} unresolved)",
        imports.resolved_total,
        imports.total_imports,
        imports.resolved_real,
        imports.resolved_sibling,
        imports.resolved_partial,
        imports.resolved_stub,
        imports.resolved_failing,
        imports.unresolved_count
    );
    let _ = writeln!(
        out,
        "# AOT Coverage  : {:.2}% ({}/{} active instructions)",
        aot.jit_coverage_pct, aot.jit_arm64_instructions, aot.active_instructions
    );
    let _ = writeln!(
        out,
        "# Anti-Tamper   : {}",
        anti_tamper.worst_verdict.as_str()
    );
    let _ = writeln!(out, "#");
    let _ = writeln!(
        out,
        "# Review fields below before booting with `cargo runner -- --manifest <this-file> --game-dir <GAME_DIR>`."
    );
    let _ = writeln!(out);
    out.push_str(&manifest.to_toml());
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bundle_common::TempDirGuard;
    use crate::win32_db::test_pe_fixture::{build_synthetic_pe, build_synthetic_win32_workspace};
    use pe_image::consts::{
        IMAGE_DIRECTORY_ENTRY_EXPORT, IMAGE_DIRECTORY_ENTRY_IMPORT, IMAGE_FILE_MACHINE_AMD64,
        IMAGE_SCN_CNT_CODE, IMAGE_SCN_MEM_EXECUTE, IMAGE_SCN_MEM_READ, IMAGE_SCN_MEM_WRITE,
    };
    use std::sync::atomic::{AtomicU64, Ordering};

    fn make_temp_dir(tag: &str) -> TempDirGuard {
        static SEQ: AtomicU64 = AtomicU64::new(0);
        let dir = std::env::temp_dir().join(format!(
            "port_factory_audit_game_{tag}_{}_{}",
            std::process::id(),
            SEQ.fetch_add(1, Ordering::Relaxed)
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        TempDirGuard::new(dir)
    }

    /// Build a synthetic PE32+ DLL that both imports symbols and exports a named symbol.
    fn build_synthetic_dll_with_export(
        dll_name: &str,
        export_sym: &str,
        code: &[u8],
        imports: &[(&str, &[&str])],
        rwx_text: bool,
    ) -> Vec<u8> {
        let mut img = build_synthetic_pe(true, code, imports, rwx_text);
        // Append a 3rd section `.edata` containing a valid IMAGE_EXPORT_DIRECTORY.
        let w16 = |b: &mut [u8], o: usize, v: u16| b[o..o + 2].copy_from_slice(&v.to_le_bytes());
        let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
        let r32 = |b: &[u8], o: usize| u32::from_le_bytes(b[o..o + 4].try_into().unwrap());

        let pe = 0x80usize;
        let opt = pe + 24;
        let edata_rva = r32(&img, opt + 56); // previous SizeOfImage
        let edata_raw_off = img.len();

        let mut edata = vec![0u8; 0x200];
        // IMAGE_EXPORT_DIRECTORY (40 bytes at +0x00)
        let funcs_off = 0x40usize;
        let names_off = 0x50usize;
        let ords_off = 0x60usize;
        let dll_str_off = 0x70usize;
        let sym_str_off = 0xA0usize;

        w32(&mut edata, 12, edata_rva + dll_str_off as u32); // Name
        w32(&mut edata, 16, 1); // Base ordinal
        w32(&mut edata, 20, 1); // NumberOfFunctions
        w32(&mut edata, 24, 1); // NumberOfNames
        w32(&mut edata, 28, edata_rva + funcs_off as u32);
        w32(&mut edata, 32, edata_rva + names_off as u32);
        w32(&mut edata, 36, edata_rva + ords_off as u32);

        w32(&mut edata, funcs_off, 0x1000); // export RVA -> .text
        w32(&mut edata, names_off, edata_rva + sym_str_off as u32);
        w16(&mut edata, ords_off, 0);
        edata[dll_str_off..dll_str_off + dll_name.len()].copy_from_slice(dll_name.as_bytes());
        edata[sym_str_off..sym_str_off + export_sym.len()].copy_from_slice(export_sym.as_bytes());

        // Update COFF NumberOfSections = 3, SizeOfImage += 0x1000, Export DataDirectory
        w16(&mut img, pe + 4, IMAGE_FILE_MACHINE_AMD64);
        w16(&mut img, pe + 6, 3);
        w32(&mut img, opt + 56, edata_rva + 0x1000);
        let exp_dd = opt + 112 + IMAGE_DIRECTORY_ENTRY_EXPORT * 8;
        w32(&mut img, exp_dd, edata_rva);
        w32(&mut img, exp_dd + 4, 0x100);
        let _ = IMAGE_DIRECTORY_ENTRY_IMPORT;

        // Section 2: `.edata`
        let sec2 = opt + 240 + 80;
        img[sec2..sec2 + 6].copy_from_slice(b".edata");
        w32(&mut img, sec2 + 8, 0x200);
        w32(&mut img, sec2 + 12, edata_rva);
        w32(&mut img, sec2 + 16, 0x200);
        w32(&mut img, sec2 + 20, edata_raw_off as u32);
        w32(
            &mut img,
            sec2 + 36,
            0x0000_0040 | IMAGE_SCN_MEM_READ,
        );
        let _ = (IMAGE_SCN_CNT_CODE, IMAGE_SCN_MEM_EXECUTE, IMAGE_SCN_MEM_WRITE);

        img.extend_from_slice(&edata);
        img
    }

    #[test]
    fn single_pass_audit_game_on_synthetic_pe_fixture() {
        let tmp = make_temp_dir("synthetic_fixture");
        let fake_ws = tmp.path().join("fake_ws");
        build_synthetic_win32_workspace(&fake_ws);
        let db = Win32Database::from_workspace_src(&fake_ws).unwrap();

        let game_dir = tmp.path().join("SpaceOdyssey");
        fs::create_dir_all(&game_dir).unwrap();

        // Main EXE: x86-64 code with xor/add/cpuid/rdtsc/ret + D3D12 + real/partial/stub/failing/sibling/unresolved imports.
        let main_code: &[u8] = &[
            0x48, 0x31, 0xC0, // xor rax, rax
            0x48, 0x01, 0xC8, // add rax, rcx
            0x0F, 0xA2, // cpuid
            0x0F, 0x31, // rdtsc
            0xC3, // ret
        ];
        let main_exe = build_synthetic_pe(
            false,
            main_code,
            &[
                ("KERNEL32.dll", &["CreateFileW"]),
                ("d3d12.dll", &["D3D12CreateDevice"]),
                ("ntdll.dll", &["NtQueryInformationProcess"]),
                ("zstd.dll", &["SynthZstdStub"]),
                ("ADVAPI32.dll", &["SynthFailApi"]),
                ("custom_game.dll", &["InitWorld"]),
                ("missing_sdk.dll", &["MissingCall"]),
            ],
            false,
        );
        fs::write(game_dir.join("SpaceOdyssey.exe"), &main_exe).unwrap();

        // Sibling DLL exporting `InitWorld`, importing PhysX3_x64.dll (middleware detection), with RWX `.text`.
        let sibling_dll = build_synthetic_dll_with_export(
            "custom_game.dll",
            "InitWorld",
            &[0x48, 0x89, 0xC8, 0xC3], // mov rax, rcx; ret
            &[("PhysX3_x64.dll", &["PxCreateBasePhysics"])],
            true,
        );
        fs::write(game_dir.join("custom_game.dll"), &sibling_dll).unwrap();
        fs::write(game_dir.join("steam_appid.txt"), "480\n").unwrap();

        let report = audit_game_dir(&game_dir, &db, &AuditGameOptions::default()).unwrap();

        // Proves strictly one parse per PE binary!
        assert_eq!(report.binaries_scanned, 2);
        assert_eq!(report.pe_parse_count, 2);
        assert_eq!(report.exe_count, 1);
        assert_eq!(report.dll_count, 1);
        assert_eq!(report.main_exe.as_deref(), Some("SpaceOdyssey.exe"));
        assert_eq!(report.steam_app_id.as_deref(), Some("480"));
        assert_eq!(report.readiness, ReadinessLevel::NeedsWork);

        // Honest impl_kind breakdown against synthetic Win32Database:
        //   main_exe: 2 real (CreateFileW, D3D12CreateDevice), 1 partial (NtQueryInformationProcess),
        //             1 stub (SynthZstdStub), 1 failing (SynthFailApi),
        //             1 sibling (custom_game.dll!InitWorld), 1 unresolved (missing_sdk.dll!MissingCall)
        //   sibling_dll: 1 real (physx3_x64.dll!PxCreateBasePhysics)
        assert_eq!(report.imports.total_imports, 8);
        assert_eq!(report.imports.resolved_real, 3);
        assert_eq!(report.imports.resolved_sibling, 1);
        assert_eq!(report.imports.resolved_partial, 1);
        assert_eq!(report.imports.resolved_stub, 1);
        assert_eq!(report.imports.resolved_failing, 1);
        assert_eq!(report.imports.unresolved_count, 1);
        assert_eq!(report.imports.stubbed_imports.len(), 2);
        assert!(report
            .imports
            .stubbed_imports
            .iter()
            .any(|s| s.dll == "zstd.dll" && s.symbol == "SynthZstdStub" && s.impl_kind == ImplKind::Stub));
        assert!(report
            .imports
            .stubbed_imports
            .iter()
            .any(|s| s.dll == "advapi32.dll"
                && s.symbol == "SynthFailApi"
                && s.impl_kind == ImplKind::Failing));
        assert!(report
            .imports
            .unresolved_imports
            .iter()
            .any(|u| u.dll == "missing_sdk.dll" && u.symbol == "MissingCall"));

        // Graphics, anti-tamper, and AOT coverage.
        assert!(report.graphics.detected_apis.contains(&"D3D12".to_string()));
        assert_eq!(
            report.anti_tamper.worst_verdict,
            AntiCheatVerdict::CompatibleViaRuntimeUnpack
        );
        assert!(report.anti_tamper.cpuid_count >= 1);
        assert!(report.anti_tamper.rdtsc_count >= 1);
        assert!(report
            .anti_tamper
            .rwx_sections
            .iter()
            .any(|s| s.contains("custom_game.dll:.text")));
        assert!(report.aot_coverage.total_instructions >= 7);
        assert!(report.aot_coverage.jit_coverage_pct > 0.0);

        // Pre-filled manifest round-trips cleanly through `GameManifest::parse`.
        let parsed = GameManifest::parse(&report.suggested_manifest_toml);
        assert_eq!(parsed.id.as_deref(), Some("space-odyssey"));
        assert_eq!(parsed.name.as_deref(), Some("Space Odyssey"));
        assert_eq!(parsed.exe.as_deref(), Some("SpaceOdyssey.exe"));
        assert_eq!(parsed.steam_app_id.as_deref(), Some("480"));
        assert!(parsed.middleware.contains_key("physx"));
        assert_eq!(
            parsed.middleware["physx"].version.as_deref(),
            Some("3.4")
        );
        assert_eq!(
            parsed.middleware["zstd"].version.as_deref(),
            Some("1")
        );

        // Writing manifest honors overwrite guard.
        let out_toml = tmp.path().join("games/space-odyssey/game.toml");
        report.write_manifest(&out_toml, false).unwrap();
        assert!(out_toml.is_file());
        assert!(report.write_manifest(&out_toml, false).is_err());
        report.write_manifest(&out_toml, true).unwrap();

        let text = report.format_human_report(Some(&out_toml));
        assert!(text.contains("Single-Pass Game Readiness Audit [NEEDS_WORK]"));
        assert!(text.contains("zstd.dll!SynthZstdStub [stub]"));
        assert!(text.contains("advapi32.dll!SynthFailApi [failing]"));
        assert!(text.contains("D3D12"));
    }

    #[test]
    fn audit_game_readiness_levels_ready_and_blocked() {
        let ws_tmp = make_temp_dir("ready_blocked_ws");
        build_synthetic_win32_workspace(ws_tmp.path());
        let db = Win32Database::from_workspace_src(ws_tmp.path()).unwrap();

        // 1. Clean game with only real imports -> READY
        let ready_tmp = make_temp_dir("ready_game");
        let clean_exe = build_synthetic_pe(
            false,
            &[0x48, 0x31, 0xC0, 0xC3],
            &[
                ("KERNEL32.dll", &["CreateFileW", "CloseHandle"]),
                ("d3d11.dll", &["D3D11CreateDevice"]),
            ],
            false,
        );
        fs::write(ready_tmp.path().join("CleanApp.exe"), &clean_exe).unwrap();
        let ready_rep = audit_game_dir(ready_tmp.path(), &db, &AuditGameOptions::default()).unwrap();
        assert_eq!(ready_rep.readiness, ReadinessLevel::Ready);
        assert!(ready_rep.blockers.is_empty());
        assert_eq!(ready_rep.imports.resolved_real, 3);
        assert_eq!(ready_rep.imports.resolved_stub, 0);
        assert_eq!(ready_rep.imports.resolved_failing, 0);
        assert_eq!(ready_rep.imports.unresolved_count, 0);
        assert!(ready_rep.graphics.detected_apis.contains(&"D3D11".to_string()));

        // 2. Game importing Ring-0 kernel service loader -> BLOCKED
        let blocked_tmp = make_temp_dir("blocked_game");
        let kernel_exe = build_synthetic_pe(
            false,
            &[0x48, 0x31, 0xC0, 0xC3],
            &[
                ("KERNEL32.dll", &["CreateFileW"]),
                ("ADVAPI32.dll", &["CreateServiceW"]),
            ],
            false,
        );
        fs::write(blocked_tmp.path().join("DriverGame.exe"), &kernel_exe).unwrap();
        let blocked_rep =
            audit_game_dir(blocked_tmp.path(), &db, &AuditGameOptions::default()).unwrap();
        assert_eq!(blocked_rep.readiness, ReadinessLevel::Blocked);
        assert!(!blocked_rep.anti_tamper.kernel_driver_imports.is_empty());
        assert!(blocked_rep
            .blockers
            .iter()
            .any(|b| b.contains("Ring-0 kernel driver")));
    }

    #[test]
    fn select_primary_exe_skips_crash_reporters_and_picks_main_game() {
        let tmp = make_temp_dir("select_primary_exe");
        let dir = tmp.path();

        // Alphabetically earlier crash helpers (including larger CrashReportClient.exe than a
        // nested game binary) must lose to the actual game executable.
        let crash = dir.join("CrashReportClient.exe");
        let crs_h = dir.join("crs-handler.exe");
        let crs_v = dir.join("crs-video.exe");
        let nested_dir = dir.join("Binaries/Win64");
        fs::create_dir_all(&nested_dir).unwrap();
        let nested_game = nested_dir.join("RealGame-Win64-Shipping.exe");

        fs::write(&crash, vec![0u8; 4096]).unwrap();
        fs::write(&crs_h, vec![0u8; 4096]).unwrap();
        fs::write(&crs_v, vec![0u8; 4096]).unwrap();
        fs::write(&nested_game, vec![0u8; 1024]).unwrap();

        let files = vec![crash.clone(), crs_h.clone(), crs_v.clone(), nested_game.clone()];
        assert_eq!(
            select_primary_exe(dir, &files, None).as_deref(),
            Some(nested_game.as_path()),
            "nested non-helper game exe must beat top-level crash reporters"
        );

        // Once a top-level non-helper game exe exists, it wins over nested exes and helpers.
        let top_game = dir.join("ZetaGame.exe");
        fs::write(&top_game, vec![0u8; 2048]).unwrap();
        let files2 = vec![crash, crs_h, crs_v, nested_game, top_game.clone()];
        assert_eq!(
            select_primary_exe(dir, &files2, None).as_deref(),
            Some(top_game.as_path()),
            "top-level non-helper game exe must beat alphabetically earlier helpers"
        );
    }
}
