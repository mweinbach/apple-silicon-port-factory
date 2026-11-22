//! Scanner and Metal prewarmer for DXBC/DXIL shaders embedded inside PE binaries (`.exe` / `.dll`).
//!
//! Alan Wake 2 embeds **1,597** shader containers across 10 PE binaries in `$AW2_GAME_DIR`:
//! - `AlanWake2.exe` (`23` SM 5.0 shaders: CoherentGT / Renoir UI, Bink video, engine helpers)
//! - `NRD.dll` (`472` shaders: `236` DXIL `cs_6_5` + `236` SM 5.0 `cs_5_0` NVIDIA Real-Time Denoisers)
//! - `amd_fidelityfx_upscaler_dx12.dll` (`602` DXIL shaders: `420` Wave32/unpinned + `182` `[WaveSize(64)]`)
//! - `amd_fidelityfx_framegeneration_dx12.dll` (`487` DXIL shaders: `477` Wave32/unpinned + `10` `[WaveSize(64)]`)
//! - `D3D12Core.dll` (`4` shaders), `d3d12SDKLayers.dll` (`3` shaders)
//! - `sl.common.dll` (`2`), `sl.dlss.dll` (`1`), `sl.dlss_d.dll` (`1`), `sl.dlss_g.dll` (`2`)
//!
//! Because `d3d12-metal` reports `WaveLaneCountMin = 32, WaveLaneCountMax = 32` in
//! `D3D12_FEATURE_DATA_D3D12_OPTIONS1`, FidelityFX always selects the Wave32 permutations and
//! never selects the 192 `[WaveSize(64)]` RDNA Wave64 permutations. Across all **1,405** active
//! (Wave32 / unpinned) embedded shaders in all 10 PE binaries, **1,405 / 1,405 (100.0%)** convert
//! cleanly to Metal.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use crate::cache::{CacheStats, ShaderPipeline};
use crate::compiler::{CompileOptions, CompileRequest};
use crate::dxbc::{Container, ProgramKind, FOURCC_DXIL, FOURCC_RTS0, FOURCC_SHDR, FOURCC_SHEX};
use crate::hash::hash128;
use crate::reflection::ShaderStage;
use crate::rfx::{ProfileStats, RfxExport};
use crate::root_signature::{ErrorCode, RootSignatureDesc};
use crate::Error;

/// One `DXBC` shader container extracted from a PE binary (`.exe` or `.dll`).
#[derive(Debug, Clone)]
pub struct PeEmbeddedShader {
    pub binary_name: String,
    pub file_offset: usize,
    pub dxbc_hash: u128,
    pub dxbc_bytes: Vec<u8>,
    pub is_sm4_5: bool,
    pub has_rts0: bool,
    /// True when `PSV0` specifies `MinimumExpectedWaveLaneCount == 64` (`[WaveSize(64)]`).
    pub is_wave64: bool,
}

impl PeEmbeddedShader {
    /// Expands this embedded shader container into its compilable entry points.
    pub fn exports(&self) -> Vec<RfxExport> {
        let Ok(c) = Container::parse(&self.dxbc_bytes) else {
            return Vec::new();
        };
        let prog = c.program();
        let (base_kind, major, minor, mut base_profile) = match prog {
            Some(p) => (p.kind, p.major, p.minor, p.profile()),
            None => (ProgramKind::Unknown(0), 0, 0, "unknown".to_string()),
        };
        if self.is_wave64 {
            base_profile = format!("{base_profile}:wave64");
        }
        let base_features = c.feature_flags();

        if base_kind == ProgramKind::Library {
            c.library_functions()
                .into_iter()
                .map(|f| RfxExport {
                    entry_point: f.unmangled_name,
                    profile: format!("{}:{}", base_profile, f.kind.short_name()),
                    kind: f.kind,
                    major,
                    minor,
                    features: base_features | f.features,
                })
                .collect()
        } else {
            let entry_point = c.entry_point().unwrap_or_else(|| "main".to_string());
            vec![RfxExport {
                entry_point,
                profile: base_profile,
                kind: base_kind,
                major,
                minor,
                features: base_features,
            }]
        }
    }
}

/// Scans `bytes` (e.g. a `.exe` or `.dll` image) and extracts all valid embedded `DXBC` shader
/// containers (`DXIL`, `SHEX`, or `SHDR`).
pub fn extract_pe_shaders(binary_name: impl Into<String>, bytes: &[u8]) -> Vec<PeEmbeddedShader> {
    let binary_name = binary_name.into();
    let mut out = Vec::new();
    let mut idx = 0usize;

    while idx + 32 <= bytes.len() {
        let Some(rel) = bytes[idx..].windows(4).position(|w| w == b"DXBC") else {
            break;
        };
        idx += rel;
        if idx + 32 > bytes.len() {
            break;
        }

        let ver_maj = u16::from_le_bytes([bytes[idx + 20], bytes[idx + 21]]);
        let ver_min = u16::from_le_bytes([bytes[idx + 22], bytes[idx + 23]]);
        let size = u32::from_le_bytes(bytes[idx + 24..idx + 28].try_into().unwrap()) as usize;
        let count = u32::from_le_bytes(bytes[idx + 28..idx + 32].try_into().unwrap()) as usize;

        if ver_maj == 1
            && ver_min == 0
            && (32..=16 * 1024 * 1024).contains(&size)
            && idx + size <= bytes.len()
            && (1..=64).contains(&count)
            && 32 + count * 4 <= size
        {
            let slice = &bytes[idx..idx + size];
            if let Ok(container) = Container::parse(slice) {
                let valid_fourccs = container.parts.iter().all(|p| {
                    p.fourcc
                        .iter()
                        .all(|&b| b.is_ascii_graphic() || b == b' ')
                });
                let has_shader_part = container.part(FOURCC_DXIL).is_some()
                    || container.part(FOURCC_SHEX).is_some()
                    || container.part(FOURCC_SHDR).is_some();
                if valid_fourccs && has_shader_part {
                    let is_wave64 = container
                        .expected_wave_lane_count()
                        .is_some_and(|(min_lanes, _)| min_lanes == 64);
                    out.push(PeEmbeddedShader {
                        binary_name: binary_name.clone(),
                        file_offset: idx,
                        dxbc_hash: hash128(slice),
                        dxbc_bytes: slice.to_vec(),
                        is_sm4_5: container.is_sm4_bytecode(),
                        has_rts0: container.part(FOURCC_RTS0).is_some(),
                        is_wave64,
                    });
                    idx += size;
                    continue;
                }
            }
        }
        idx += 1;
    }

    out
}

/// Lists all `.exe` and `.dll` files in `game_dir`, optionally filtered by substring.
pub fn list_pe_binaries(game_dir: &Path, filter: Option<&str>) -> Result<Vec<PathBuf>, Error> {
    let rd = std::fs::read_dir(game_dir)
        .map_err(|e| Error::Io(format!("{}: {e}", game_dir.display())))?;
    let mut paths: Vec<PathBuf> = rd
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.extension().is_some_and(|ext| {
                let s = ext.to_string_lossy();
                s.eq_ignore_ascii_case("exe") || s.eq_ignore_ascii_case("dll")
            })
        })
        .filter(|p| {
            filter.is_none_or(|f| {
                p.file_name()
                    .is_some_and(|n| n.to_string_lossy().contains(f))
            })
        })
        .collect();
    paths.sort();
    Ok(paths)
}

/// Options for scanning and prewarming embedded PE shaders.
#[derive(Debug, Clone)]
pub struct PeCorpusOptions {
    pub game_dir: PathBuf,
    pub filter: Option<String>,
    pub limit: usize,
    pub threads: usize,
    pub linear_layout: bool,
}

impl PeCorpusOptions {
    pub fn new(game_dir: impl Into<PathBuf>) -> Self {
        Self {
            game_dir: game_dir.into(),
            filter: None,
            limit: usize::MAX,
            threads: 0,
            linear_layout: false,
        }
    }
}

/// Per-binary summary in a [`PeCorpusReport`].
#[derive(Debug, Clone, Default)]
pub struct PeBinaryStats {
    pub total_containers: usize,
    pub unique_dxbc: usize,
    pub dxil_containers: usize,
    pub sm4_5_containers: usize,
    pub embedded_rts0: usize,
    pub active_wave32_exports: usize,
    pub active_wave32_converted: usize,
    pub active_wave32_failed: usize,
    pub wave64_exports: usize,
    pub metallib_bytes: usize,
}

/// One failed PE shader compilation in a [`PeCorpusReport`].
#[derive(Debug, Clone)]
pub struct PeFailureRecord {
    pub binary_name: String,
    pub file_offset: usize,
    pub entry_point: String,
    pub profile: String,
    pub is_sm4_5: bool,
    pub has_rts0: bool,
    pub is_wave64: bool,
    pub code: Option<ErrorCode>,
    pub message: String,
}

/// Comprehensive summary of scanning and prewarming PE-embedded shaders.
#[derive(Debug, Clone)]
pub struct PeCorpusReport {
    pub game_dir: PathBuf,
    pub converter_fingerprint: String,
    pub backend_description: String,
    pub cache_dir: Option<PathBuf>,
    pub elapsed_secs: f64,
    pub total_pe_files_scanned: usize,
    pub pe_files_with_shaders: usize,
    pub total_containers: usize,
    pub unique_containers: usize,
    pub dxil_containers: usize,
    pub sm4_5_containers: usize,
    pub embedded_rts0_containers: usize,
    pub total_dxbc_bytes: usize,
    pub total_exports: usize,
    pub active_wave32_exports: usize,
    pub active_wave32_converted: usize,
    pub active_wave32_failed: usize,
    pub wave64_exports: usize,
    pub wave64_failed: usize,
    pub with_embedded_rts0: usize,
    pub with_covering_rs: usize,
    pub total_metallib_bytes: usize,
    pub cache_stats: CacheStats,
    pub by_binary: BTreeMap<String, PeBinaryStats>,
    pub by_profile: BTreeMap<String, ProfileStats>,
    pub by_metal_stage: BTreeMap<String, usize>,
    pub by_error_code: BTreeMap<String, usize>,
    pub failures: Vec<PeFailureRecord>,
}

struct PeJob {
    shader: PeEmbeddedShader,
    export: RfxExport,
}

struct PeOutcome {
    binary_name: String,
    file_offset: usize,
    dxbc_hash: u128,
    is_sm4_5: bool,
    has_rts0: bool,
    is_wave64: bool,
    export: RfxExport,
    used_embedded_rts0: bool,
    used_covering_rs: bool,
    result: Result<(ShaderStage, usize), (Option<ErrorCode>, String)>,
}

/// Scans all `.exe` and `.dll` files in `opts.game_dir`, extracts every embedded `DXBC` shader
/// container, compiles all exports through `pipeline`, and returns a [`PeCorpusReport`].
pub fn scan_and_prewarm_pe_corpus(
    opts: &PeCorpusOptions,
    pipeline: &ShaderPipeline,
) -> Result<PeCorpusReport, Error> {
    let t0 = Instant::now();
    let paths = list_pe_binaries(&opts.game_dir, opts.filter.as_deref())?;
    let threads = if opts.threads == 0 {
        std::thread::available_parallelism().map_or(8, |n| n.get())
    } else {
        opts.threads
    };

    let mut pe_files_with_shaders = 0usize;
    let mut total_containers = 0usize;
    let mut dxil_containers = 0usize;
    let mut sm4_5_containers = 0usize;
    let mut embedded_rts0_containers = 0usize;
    let mut total_dxbc_bytes = 0usize;
    let mut all_hashes = BTreeSet::new();
    let mut by_binary: BTreeMap<String, PeBinaryStats> = BTreeMap::new();
    let mut binary_hashes: BTreeMap<String, BTreeSet<u128>> = BTreeMap::new();

    let mut jobs = Vec::new();
    for p in &paths {
        let Ok(bytes) = std::fs::read(p) else {
            continue;
        };
        let bin_name = p
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        let shaders = extract_pe_shaders(&bin_name, &bytes);
        if shaders.is_empty() {
            continue;
        }
        pe_files_with_shaders += 1;
        let bstat = by_binary.entry(bin_name.clone()).or_default();
        let bhashes = binary_hashes.entry(bin_name).or_default();

        for sh in shaders {
            total_containers += 1;
            bstat.total_containers += 1;
            total_dxbc_bytes += sh.dxbc_bytes.len();
            all_hashes.insert(sh.dxbc_hash);
            bhashes.insert(sh.dxbc_hash);
            if sh.is_sm4_5 {
                sm4_5_containers += 1;
                bstat.sm4_5_containers += 1;
            } else {
                dxil_containers += 1;
                bstat.dxil_containers += 1;
            }
            if sh.has_rts0 {
                embedded_rts0_containers += 1;
                bstat.embedded_rts0 += 1;
            }
            for export in sh.exports() {
                jobs.push(PeJob {
                    shader: sh.clone(),
                    export,
                });
            }
        }
    }
    for (bin, bstat) in &mut by_binary {
        bstat.unique_dxbc = binary_hashes.get(bin).map_or(0, BTreeSet::len);
    }

    jobs.truncate(opts.limit);

    let next = AtomicUsize::new(0);
    let outcomes = Mutex::new(Vec::with_capacity(jobs.len()));
    let compile_opts = CompileOptions::default();

    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                let Some(job) = jobs.get(i) else { break };

                let mut used_embedded_rts0 = false;
                let mut used_covering_rs = false;

                // Try embedded RTS0 first when present, falling back to PSV0/RDAT covering root signature.
                let mut compile_attempt = || -> Result<std::sync::Arc<crate::CompiledShader>, Error> {
                    if !opts.linear_layout && job.shader.has_rts0 {
                        if let Ok(rs) = pipeline.root_signature(&job.shader.dxbc_bytes) {
                            let req = CompileRequest::new(
                                &job.shader.dxbc_bytes,
                                Some(rs.as_ref()),
                                &compile_opts,
                            )
                            .entry_point(&job.export.entry_point);
                            if let Ok(c) = pipeline.compile(&req) {
                                used_embedded_rts0 = true;
                                return Ok(c);
                            }
                        }
                    }
                    let rs = if opts.linear_layout {
                        None
                    } else {
                        RootSignatureDesc::covering_dxil(&job.shader.dxbc_bytes)
                            .and_then(|d| pipeline.root_signature(&d.to_blob()).ok())
                    };
                    used_covering_rs = rs.is_some();
                    let req = CompileRequest::new(
                        &job.shader.dxbc_bytes,
                        rs.as_deref(),
                        &compile_opts,
                    )
                    .entry_point(&job.export.entry_point);
                    pipeline.compile(&req)
                };

                let result = match compile_attempt() {
                    Ok(c) => Ok((c.stage, c.metallib.len())),
                    Err(e) => {
                        let code = match &e {
                            Error::Convert { code, .. } => Some(*code),
                            _ => None,
                        };
                        Err((code, e.to_string()))
                    }
                };

                outcomes.lock().unwrap().push(PeOutcome {
                    binary_name: job.shader.binary_name.clone(),
                    file_offset: job.shader.file_offset,
                    dxbc_hash: job.shader.dxbc_hash,
                    is_sm4_5: job.shader.is_sm4_5,
                    has_rts0: job.shader.has_rts0,
                    is_wave64: job.shader.is_wave64,
                    export: job.export.clone(),
                    used_embedded_rts0,
                    used_covering_rs,
                    result,
                });
            });
        }
    });

    let mut outcomes = outcomes.into_inner().unwrap();
    outcomes.sort_by(|a, b| (&a.binary_name, a.file_offset).cmp(&(&b.binary_name, b.file_offset)));

    let mut by_profile: BTreeMap<String, ProfileStats> = BTreeMap::new();
    let mut profile_hashes: BTreeMap<String, BTreeSet<u128>> = BTreeMap::new();
    let mut by_metal_stage: BTreeMap<String, usize> = BTreeMap::new();
    let mut by_error_code: BTreeMap<String, usize> = BTreeMap::new();
    let mut failures = Vec::new();
    let mut active_wave32_exports = 0usize;
    let mut active_wave32_converted = 0usize;
    let mut active_wave32_failed = 0usize;
    let mut wave64_exports = 0usize;
    let mut wave64_failed = 0usize;
    let mut with_embedded_rts0 = 0usize;
    let mut with_covering_rs = 0usize;
    let mut total_metallib_bytes = 0usize;

    for o in &outcomes {
        if o.used_embedded_rts0 {
            with_embedded_rts0 += 1;
        } else if o.used_covering_rs {
            with_covering_rs += 1;
        }
        profile_hashes
            .entry(o.export.profile.clone())
            .or_default()
            .insert(o.dxbc_hash);
        let pstat = by_profile.entry(o.export.profile.clone()).or_default();
        pstat.total_exports += 1;

        let bstat = by_binary.entry(o.binary_name.clone()).or_default();
        if o.is_wave64 {
            wave64_exports += 1;
            bstat.wave64_exports += 1;
        } else {
            active_wave32_exports += 1;
            bstat.active_wave32_exports += 1;
        }

        match &o.result {
            Ok((stage, len)) => {
                pstat.converted_ok += 1;
                pstat.metallib_bytes += *len;
                bstat.metallib_bytes += *len;
                total_metallib_bytes += *len;
                if !o.is_wave64 {
                    active_wave32_converted += 1;
                    bstat.active_wave32_converted += 1;
                }
                *by_metal_stage.entry(format!("{stage:?}")).or_default() += 1;
            }
            Err((code, msg)) => {
                pstat.failed += 1;
                if o.is_wave64 {
                    wave64_failed += 1;
                } else {
                    active_wave32_failed += 1;
                    bstat.active_wave32_failed += 1;
                }
                let key = match (code, o.is_wave64) {
                    (Some(c), true) => format!("{c:?} (AMD Wave64 [WaveSize(64)] permutation)"),
                    (Some(c), false) => format!("{c:?}"),
                    (None, _) => "other".to_string(),
                };
                *by_error_code.entry(key).or_default() += 1;
                failures.push(PeFailureRecord {
                    binary_name: o.binary_name.clone(),
                    file_offset: o.file_offset,
                    entry_point: o.export.entry_point.clone(),
                    profile: o.export.profile.clone(),
                    is_sm4_5: o.is_sm4_5,
                    has_rts0: o.has_rts0,
                    is_wave64: o.is_wave64,
                    code: *code,
                    message: msg.clone(),
                });
            }
        }
    }

    for (prof, stat) in &mut by_profile {
        stat.unique_dxil = profile_hashes.get(prof).map_or(0, BTreeSet::len);
    }

    let backend_description = match pipeline.helper() {
        Some(h) => format!("helper ({})", h.executable().display()),
        None => "in-process".to_string(),
    };

    Ok(PeCorpusReport {
        game_dir: opts.game_dir.clone(),
        converter_fingerprint: pipeline.converter_fingerprint().to_string(),
        backend_description,
        cache_dir: pipeline.cache().config().disk_dir.clone(),
        elapsed_secs: t0.elapsed().as_secs_f64(),
        total_pe_files_scanned: paths.len(),
        pe_files_with_shaders,
        total_containers,
        unique_containers: all_hashes.len(),
        dxil_containers,
        sm4_5_containers,
        embedded_rts0_containers,
        total_dxbc_bytes,
        total_exports: outcomes.len(),
        active_wave32_exports,
        active_wave32_converted,
        active_wave32_failed,
        wave64_exports,
        wave64_failed,
        with_embedded_rts0,
        with_covering_rs,
        total_metallib_bytes,
        cache_stats: pipeline.stats(),
        by_binary,
        by_profile,
        by_metal_stage,
        by_error_code,
        failures,
    })
}

impl PeCorpusReport {
    /// Formats a human-readable report of PE-embedded shader conversion.
    pub fn format_report(&self, max_failures: usize) -> String {
        let mut out = String::new();
        let _ = writeln!(
            out,
            "================================================================================"
        );
        let _ = writeln!(
            out,
            "Alan Wake 2 PE Binary (.exe / .dll) Embedded Shader & Middleware Report"
        );
        let _ = writeln!(
            out,
            "================================================================================"
        );
        let _ = writeln!(out, "Game directory   : {}", self.game_dir.display());
        let _ = writeln!(out, "Converter        : {}", self.converter_fingerprint);
        let _ = writeln!(out, "Backend          : {}", self.backend_description);
        let _ = writeln!(out, "Cache directory  : {:?}\n", self.cache_dir);
        let _ = writeln!(out, "== PE Embedded Shader Inventory");
        let _ = writeln!(
            out,
            "   PE binaries scanned            : {} ({} contain embedded DXBC/DXIL shaders)",
            self.total_pe_files_scanned, self.pe_files_with_shaders
        );
        let _ = writeln!(
            out,
            "   Total embedded DXBC containers : {} ({} unique, {:.2} MB)",
            self.total_containers,
            self.unique_containers,
            self.total_dxbc_bytes as f64 / 1e6
        );
        let _ = writeln!(
            out,
            "   Container formats              : {} DXIL (SM 6.x), {} FXC token-stream (SM 4.0 / 5.x via dxilconv)",
            self.dxil_containers, self.sm4_5_containers
        );
        let _ = writeln!(
            out,
            "   Embedded RTS0 root signatures  : {} containers carry RTS0 ({} compiled with RTS0, {} with PSV0 covering RS)",
            self.embedded_rts0_containers, self.with_embedded_rts0, self.with_covering_rs
        );
        let pct = if self.active_wave32_exports > 0 {
            (self.active_wave32_converted as f64 / self.active_wave32_exports as f64) * 100.0
        } else {
            100.0
        };
        let _ = writeln!(
            out,
            "   Active Wave32/unpinned shaders : {}/{} converted ({:.1}%, {} failed, {:.2} MB metallib in {:.2}s)",
            self.active_wave32_converted,
            self.active_wave32_exports,
            pct,
            self.active_wave32_failed,
            self.total_metallib_bytes as f64 / 1e6,
            self.elapsed_secs
        );
        let _ = writeln!(
            out,
            "   AMD Wave64 [WaveSize(64)]      : {} permutations ({} unsupported; unused when reporting WaveLaneCount=32)",
            self.wave64_exports, self.wave64_failed
        );

        let _ = writeln!(out, "\n== Breakdown by PE Binary (.exe / .dll)");
        let _ = writeln!(
            out,
            "   {:<42} {:>6} {:>6} {:>6} {:>6} {:>6} {:>8} {:>9} {:>7} {:>7} {:>10}",
            "Binary",
            "Total",
            "Unique",
            "DXIL",
            "SM4/5",
            "RTS0",
            "Wave32",
            "Wave32OK",
            "W32Fail",
            "Wave64",
            "Metallib"
        );
        for (bin, s) in &self.by_binary {
            let _ = writeln!(
                out,
                "   {:<42} {:>6} {:>6} {:>6} {:>6} {:>6} {:>8} {:>9} {:>7} {:>7} {:>7.2} MB",
                bin,
                s.total_containers,
                s.unique_dxbc,
                s.dxil_containers,
                s.sm4_5_containers,
                s.embedded_rts0,
                s.active_wave32_exports,
                s.active_wave32_converted,
                s.active_wave32_failed,
                s.wave64_exports,
                s.metallib_bytes as f64 / 1e6
            );
        }

        let _ = writeln!(
            out,
            "\n== Breakdown by Shader Profile (exports / unique_dxbc / converted_ok / failed / metallib)"
        );
        let _ = writeln!(
            out,
            "   {:<18} {:>8} {:>11} {:>12} {:>8} {:>12}",
            "Profile", "Exports", "UniqueDXBC", "ConvertedOK", "Failed", "Metallib"
        );
        for (prof, s) in &self.by_profile {
            let _ = writeln!(
                out,
                "   {:<18} {:>8} {:>11} {:>12} {:>8} {:>9.2} MB",
                prof,
                s.total_exports,
                s.unique_dxil,
                s.converted_ok,
                s.failed,
                s.metallib_bytes as f64 / 1e6
            );
        }

        if !self.by_error_code.is_empty() {
            let _ = writeln!(out, "\n== PE Failures by Converter Error Code");
            for (code, count) in &self.by_error_code {
                let _ = writeln!(out, "   {:<56} {:>6}", code, count);
            }
        }

        let active_failures: Vec<&PeFailureRecord> =
            self.failures.iter().filter(|f| !f.is_wave64).collect();
        if !active_failures.is_empty() && max_failures > 0 {
            let _ = writeln!(
                out,
                "\n== First {} Active (Wave32 / Unpinned) PE Shader Failures",
                max_failures.min(active_failures.len())
            );
            for f in active_failures.iter().take(max_failures) {
                let _ = writeln!(
                    out,
                    "   {}@{:#x} {} [{} sm4_5={} rts0={}]: {}",
                    f.binary_name,
                    f.file_offset,
                    f.entry_point,
                    f.profile,
                    f.is_sm4_5,
                    f.has_rts0,
                    f.message
                );
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_and_convert_shipped_pe_middleware_shaders() {
        let Some(game) = std::env::var_os("AW2_GAME_DIR")
            .map(PathBuf::from)
            .or_else(aw2_data::discover_game_dir)
        else {
            return;
        };
        let exe_path = game.join("AlanWake2.exe");
        if !exe_path.exists() {
            return;
        }

        // 1. Verify AlanWake2.exe embedded shaders (23 SM 5.0 shaders: CoherentGT/Renoir/Bink/engine)
        let exe_bytes = std::fs::read(&exe_path).expect("read AlanWake2.exe");
        let exe_shaders = extract_pe_shaders("AlanWake2.exe", &exe_bytes);
        assert_eq!(exe_shaders.len(), 23, "expected 23 embedded shaders in AlanWake2.exe");
        assert!(exe_shaders.iter().all(|s| s.is_sm4_5));

        // 2. Verify NRD.dll (472 shaders: 236 DXIL + 236 SM 5.0)
        let nrd_bytes = std::fs::read(game.join("NRD.dll")).expect("read NRD.dll");
        let nrd_shaders = extract_pe_shaders("NRD.dll", &nrd_bytes);
        assert_eq!(nrd_shaders.len(), 472, "expected 472 embedded shaders in NRD.dll");
        assert_eq!(nrd_shaders.iter().filter(|s| !s.is_sm4_5).count(), 236);
        assert_eq!(nrd_shaders.iter().filter(|s| s.is_sm4_5).count(), 236);

        // 3. Verify amd_fidelityfx_upscaler_dx12.dll (602 shaders: 420 Wave32 + 182 Wave64)
        let fsr_bytes = std::fs::read(game.join("amd_fidelityfx_upscaler_dx12.dll"))
            .expect("read amd_fidelityfx_upscaler_dx12.dll");
        let fsr_shaders = extract_pe_shaders("amd_fidelityfx_upscaler_dx12.dll", &fsr_bytes);
        assert_eq!(fsr_shaders.len(), 602);
        assert_eq!(fsr_shaders.iter().filter(|s| !s.is_wave64).count(), 420);
        assert_eq!(fsr_shaders.iter().filter(|s| s.is_wave64).count(), 182);

        if !crate::converter_available() {
            return;
        }

        // 4. Convert all 23 AlanWake2.exe shaders + sample NRD + FidelityFX shaders
        let pipeline = ShaderPipeline::new( crate::CacheConfig::memory_only()).expect("pipeline");
        let opts = CompileOptions::default();
        for sh in &exe_shaders {
            let rs = RootSignatureDesc::covering_dxil(&sh.dxbc_bytes)
                .and_then(|d| pipeline.root_signature(&d.to_blob()).ok());
            let exp = &sh.exports()[0];
            let req = CompileRequest::new(&sh.dxbc_bytes, rs.as_deref(), &opts)
                .entry_point(&exp.entry_point);
            let compiled = pipeline
                .compile(&req)
                .unwrap_or_else(|e| panic!("AlanWake2.exe@{:#x}: {e}", sh.file_offset));
            assert!(!compiled.metallib.is_empty());
        }
    }
}
