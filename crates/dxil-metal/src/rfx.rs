//! Northlight `.binrfx` shader bundle parser, reflection extractor, and corpus scanner/prewarmer.
//!
//! All 153 shipped Alan Wake 2 shader bundles live under `data/shaders/build/pc_dx12/*.binrfx`
//! (none are packed inside `.rmdtoc` archives):
//! - 146 bundles contain 2,284 DXBC/DXIL program containers (2,246 unique by content hash),
//!   expanding to 2,548 shader entry points / DXR library exports.
//! - 7 bundles (`billboard.binrfx`, `character.binrfx`, `cloth.binrfx`, `decal.binrfx`,
//!   `deprecated_blendedmaterial.binrfx`, `gfxgraph_decal.binrfx`, `standardmaterial.binrfx`)
//!   are material-template definitions with 0 shader programs.
//! - Of the 2,548 total exports:
//!   - **1,906** are Shader Model <= 6.6 (`vs_6_2`, `ps_6_2`, `cs_6_5`, `ms_6_5`, and `lib_6_5`
//!     `raygen` / `closesthit` / `anyhit` / `miss`) — **100% (1,906 / 1,906)** convert cleanly
//!     through Apple's Metal Shader Converter.
//!   - **642** are Shader Model 6.9 (`lib_6_9:raygen`) NVIDIA Shader Execution Reordering (SER /
//!     `HitObject`) permutations in `rt_pathtracing.binrfx` (258) and `rt_transparent_pt.binrfx`
//!     (384). Because the D3D12 translation layer reports `D3D_SHADER_MODEL_6_6` and
//!     `D3D12_RAYTRACING_TIER_1_1`, the game always selects the standard `lib_6_5` permutations.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use crate::cache::{CacheStats, ShaderPipeline};
use crate::compiler::{CompileOptions, CompileRequest};
use crate::dxbc::{self, Container, ProgramInfo, ProgramKind, FEATURE_NAMES};
use crate::hash::hash128;
use crate::reflection::ShaderStage;
use crate::root_signature::{ErrorCode, RootSignatureDesc};
use crate::Error;

/// The 6 shader stage slots stored sequentially in a `.binrfx` bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum RfxStageSlot {
    Vertex = 0,
    Pixel = 1,
    Compute = 2,
    Mesh = 3,
    Amplification = 4,
    Raytracing = 5,
}

impl RfxStageSlot {
    pub const ALL: [Self; 6] = [
        Self::Vertex,
        Self::Pixel,
        Self::Compute,
        Self::Mesh,
        Self::Amplification,
        Self::Raytracing,
    ];

    pub fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index).copied()
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Vertex => "Vertex",
            Self::Pixel => "Pixel",
            Self::Compute => "Compute",
            Self::Mesh => "Mesh",
            Self::Amplification => "Amplification",
            Self::Raytracing => "Raytracing",
        }
    }
}

/// Resource group category inside a `.binrfx` program reflection block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RfxResourceGroup {
    TextureSrv = 0,
    Sampler = 1,
    Unused2 = 2,
    StructuredBufferSrv = 3,
    BufferSrv = 4,
    TextureUav = 5,
    StructuredBufferUav = 6,
    BufferUav = 7,
}

impl RfxResourceGroup {
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::TextureSrv,
            1 => Self::Sampler,
            2 => Self::Unused2,
            3 => Self::StructuredBufferSrv,
            4 => Self::BufferSrv,
            5 => Self::TextureUav,
            6 => Self::StructuredBufferUav,
            _ => Self::BufferUav,
        }
    }
}

/// One resource binding parsed from a `.binrfx` program's reflection block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RfxResourceBinding {
    pub group: RfxResourceGroup,
    pub name: String,
    pub bind_point: u32,
    pub bind_count: u32,
    pub space: u32,
    pub stride_or_format: u32,
    pub flags: u8,
}

/// One constant buffer / root constant block parsed from a `.binrfx` program's reflection block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RfxConstantBuffer {
    /// True when stored in CB group 1 (`ManualUpdateCB_*` root constants).
    pub is_root_constants: bool,
    pub bind_point: u32,
    pub size_bytes: u32,
    pub name: String,
    pub variable_count: u32,
}

/// A single compilable entry point or DXR library export within an [`RfxProgram`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RfxExport {
    pub entry_point: String,
    /// Profile string (`vs_6_2`, `ps_6_2`, `cs_6_5`, `ms_6_5`, `lib_6_5:raygen`, etc.).
    pub profile: String,
    pub kind: ProgramKind,
    pub major: u8,
    pub minor: u8,
    pub features: u64,
}

impl RfxExport {
    /// True for Shader Model 6.9+ (`lib_6_9` NVIDIA SER `HitObject` permutations).
    pub fn is_sm69_ser(&self) -> bool {
        self.major > 6 || (self.major == 6 && self.minor >= 9)
    }
}

/// One DXBC/DXIL program entry inside an [`RfxBundle`].
#[derive(Debug, Clone)]
pub struct RfxProgram {
    pub stage_slot: RfxStageSlot,
    /// Entry point name (empty for `lib_6_*` raytracing libraries whose exports live in `RDAT`).
    pub entry_point: String,
    /// 64-bit shader hash stored in the `.binrfx` program footer.
    pub shader_hash: u64,
    /// 128-bit FNV-1a content hash of `dxbc_bytes`.
    pub dxbc_hash: u128,
    pub dxbc_bytes: Vec<u8>,
    pub resources: Vec<RfxResourceBinding>,
    pub cbuffers: Vec<RfxConstantBuffer>,
}

impl RfxProgram {
    /// Returns the parsed `DXIL` program header (`kind`, `major`, `minor`, `dxil_version`).
    pub fn program_info(&self) -> Option<ProgramInfo> {
        Container::parse(&self.dxbc_bytes).ok()?.program()
    }

    /// True if this container is a Shader Model 6.9+ library (NVIDIA SER `HitObject`).
    pub fn is_sm69_ser(&self) -> bool {
        self.program_info()
            .is_some_and(|p| p.major > 6 || (p.major == 6 && p.minor >= 9))
    }

    /// Synthesizes a covering [`RootSignatureDesc`] from the container's `PSV0` or `RDAT` resource
    /// bindings (or returns `None` when the shader binds no resources).
    pub fn covering_root_signature(&self) -> Option<RootSignatureDesc> {
        RootSignatureDesc::covering_dxil(&self.dxbc_bytes)
    }

    /// Expands this program into its compilable entry points:
    /// - For standard stages (`VS`, `PS`, `CS`, `MS`, `AS`), returns the single entry point.
    /// - For DXIL libraries (`lib_6_*`), returns every export listed in the `RDAT` function table.
    pub fn exports(&self) -> Vec<RfxExport> {
        let Ok(container) = Container::parse(&self.dxbc_bytes) else {
            return Vec::new();
        };
        let prog = container.program();
        let (base_kind, major, minor, base_profile) = match prog {
            Some(p) => (p.kind, p.major, p.minor, p.profile()),
            None => (ProgramKind::Unknown(0), 0, 0, "unknown".to_string()),
        };
        let base_features = container.feature_flags();

        if self.entry_point.is_empty() {
            container
                .library_functions()
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
            vec![RfxExport {
                entry_point: self.entry_point.clone(),
                profile: base_profile,
                kind: base_kind,
                major,
                minor,
                features: base_features,
            }]
        }
    }
}

/// Parsed Northlight `.binrfx` shader bundle.
#[derive(Debug, Clone)]
pub struct RfxBundle {
    pub file_name: String,
    pub version: u32,
    pub shader_name: String,
    pub programs: Vec<RfxProgram>,
    pub techniques: Vec<String>,
    pub permutations: Vec<String>,
}

fn rd_u32(data: &[u8], pos: usize) -> Result<u32, Error> {
    data.get(pos..pos + 4)
        .map(|s| u32::from_le_bytes(s.try_into().unwrap()))
        .ok_or_else(|| Error::InvalidInput(format!("binrfx truncated at offset {pos}")))
}

fn rd_u64(data: &[u8], pos: usize) -> Result<u64, Error> {
    data.get(pos..pos + 8)
        .map(|s| u64::from_le_bytes(s.try_into().unwrap()))
        .ok_or_else(|| Error::InvalidInput(format!("binrfx truncated at offset {pos}")))
}

fn rd_str(data: &[u8], pos: &mut usize) -> Result<String, Error> {
    let len = rd_u32(data, *pos)? as usize;
    *pos += 4;
    let slice = data
        .get(*pos..*pos + len)
        .ok_or_else(|| Error::InvalidInput(format!("binrfx string truncated at offset {}", *pos)))?;
    *pos += len;
    Ok(String::from_utf8_lossy(slice).into_owned())
}

impl RfxBundle {
    /// Opens and parses a `.binrfx` file from disk.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let p = path.as_ref();
        let data = std::fs::read(p).map_err(|e| Error::Io(format!("{}: {e}", p.display())))?;
        let file_name = p
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        Self::from_bytes(file_name, &data)
    }

    /// Parses a `.binrfx` bundle from an in-memory byte slice.
    pub fn from_bytes(file_name: impl Into<String>, data: &[u8]) -> Result<Self, Error> {
        let file_name = file_name.into();
        if data.len() < 12 || &data[0..4] != b"RFX " {
            return Err(Error::InvalidInput(format!(
                "{file_name}: invalid .binrfx magic (expected 'RFX ')"
            )));
        }
        let version = rd_u32(data, 4)?;
        let mut pos = 8;
        let shader_name = rd_str(data, &mut pos)?;

        let mut programs = Vec::new();
        for stage_slot in RfxStageSlot::ALL {
            let cnt = rd_u32(data, pos)? as usize;
            pos += 4;
            for _ in 0..cnt {
                let entry_point = rd_str(data, &mut pos)?;
                let bcs = rd_u64(data, pos)? as usize;
                pos += 8;
                let dxbc_slice = data.get(pos..pos + bcs).ok_or_else(|| {
                    Error::InvalidInput(format!("{file_name}: truncated DXBC payload at {pos}"))
                })?;
                let dxbc_bytes = dxbc_slice.to_vec();
                let dxbc_hash = hash128(&dxbc_bytes);
                pos += bcs;

                let mut resources = Vec::new();
                let mut cbuffers = Vec::new();
                let has_refl = *data.get(pos).ok_or_else(|| {
                    Error::InvalidInput(format!("{file_name}: truncated reflection flag at {pos}"))
                })?;
                if has_refl != 0 {
                    pos += 5;
                    for g in 0..8 {
                        let gc = rd_u32(data, pos)? as usize;
                        pos += 4;
                        let group = RfxResourceGroup::from_index(g);
                        for _ in 0..gc {
                            let name = rd_str(data, &mut pos)?;
                            if pos + 21 > data.len() {
                                return Err(Error::InvalidInput(format!(
                                    "{file_name}: truncated resource binding at {pos}"
                                )));
                            }
                            let bind_point = rd_u32(data, pos)?;
                            let bind_count = rd_u32(data, pos + 4)?;
                            let space = rd_u32(data, pos + 8)?;
                            let _group_id = rd_u32(data, pos + 12)?;
                            let stride_or_format = rd_u32(data, pos + 16)?;
                            let flags = data[pos + 20];
                            pos += 21;
                            resources.push(RfxResourceBinding {
                                group,
                                name,
                                bind_point,
                                bind_count,
                                space,
                                stride_or_format,
                                flags,
                            });
                        }
                    }
                    for cb_group in 0..2 {
                        let cbc = rd_u32(data, pos)? as usize;
                        pos += 4;
                        for _ in 0..cbc {
                            let bind_point = rd_u32(data, pos)?;
                            let size_bytes = rd_u32(data, pos + 4)?;
                            pos += 8;
                            let name = rd_str(data, &mut pos)?;
                            let _alt_name = rd_str(data, &mut pos)?;
                            let vc = rd_u32(data, pos)? as usize;
                            pos += 4;
                            for _ in 0..vc {
                                let _var_name = rd_str(data, &mut pos)?;
                                if pos + 17 > data.len() {
                                    return Err(Error::InvalidInput(format!(
                                        "{file_name}: truncated cbuffer variable at {pos}"
                                    )));
                                }
                                pos += 17;
                            }
                            if pos + 4 > data.len() {
                                return Err(Error::InvalidInput(format!(
                                    "{file_name}: truncated cbuffer footer at {pos}"
                                )));
                            }
                            pos += 4;
                            cbuffers.push(RfxConstantBuffer {
                                is_root_constants: cb_group == 1,
                                bind_point,
                                size_bytes,
                                name,
                                variable_count: vc as u32,
                            });
                        }
                    }
                    pos += 1;
                } else {
                    pos += 1;
                }

                if pos + 5 > data.len() {
                    return Err(Error::InvalidInput(format!(
                        "{file_name}: truncated input layout header at {pos}"
                    )));
                }
                let il_cnt = data[pos + 4] as usize;
                pos += 5 + il_cnt * 4;
                if pos >= data.len() {
                    return Err(Error::InvalidInput(format!(
                        "{file_name}: truncated c8 header at {pos}"
                    )));
                }
                let c8c = data[pos] as usize;
                pos += 1 + c8c;
                let shader_hash = rd_u64(data, pos)?;
                pos += 12;

                programs.push(RfxProgram {
                    stage_slot,
                    entry_point,
                    shader_hash,
                    dxbc_hash,
                    dxbc_bytes,
                    resources,
                    cbuffers,
                });
            }
        }

        let mut techniques = Vec::new();
        if pos + 4 <= data.len() {
            let cnt_a8 = rd_u32(data, pos)? as usize;
            pos += 4;
            for _ in 0..cnt_a8 {
                let tn = rd_str(data, &mut pos)?;
                let s1 = rd_u32(data, pos)? as usize;
                pos += 4 + s1 * 52;
                let s2 = rd_u32(data, pos)? as usize;
                pos += 4 + s2 * 20 + 16;
                techniques.push(tn);
            }
        }

        if pos + 8 <= data.len() {
            let s80 = rd_u64(data, pos)? as usize;
            pos += 8;
            if s80 != 0 {
                pos += s80;
                let _s80_name = rd_str(data, &mut pos)?;
                pos += 8;
            }
        }

        let mut permutations = Vec::new();
        if pos + 4 <= data.len() {
            let cnt_b8 = rd_u32(data, pos)? as usize;
            pos += 4;
            for _ in 0..cnt_b8 {
                let bn = rd_str(data, &mut pos)?;
                pos += 16;
                permutations.push(bn);
            }
        }

        Ok(Self {
            file_name,
            version,
            shader_name,
            programs,
            techniques,
            permutations,
        })
    }
}

/// Options for scanning and prewarming the `.binrfx` shader corpus.
#[derive(Debug, Clone)]
pub struct CorpusOptions {
    /// Directory containing `*.binrfx` files (`<game>/data/shaders/build/pc_dx12`).
    pub shader_dir: PathBuf,
    /// Optional substring filter on `.binrfx` file names.
    pub filter: Option<String>,
    /// Maximum number of compile jobs to run (`usize::MAX` for all).
    pub limit: usize,
    /// Number of worker threads (`0` = `available_parallelism`).
    pub threads: usize,
    /// Compile without synthesizing a PSV0/RDAT root signature (linear layout).
    pub linear_layout: bool,
    /// Skip SM 6.9 NVIDIA SER (`HitObject`) library permutations during compilation.
    pub skip_sm69: bool,
}

impl CorpusOptions {
    pub fn new(shader_dir: impl Into<PathBuf>) -> Self {
        Self {
            shader_dir: shader_dir.into(),
            filter: None,
            limit: usize::MAX,
            threads: 0,
            linear_layout: false,
            skip_sm69: false,
        }
    }
}

/// Per-profile statistics in a [`CorpusReport`].
#[derive(Debug, Clone, Default)]
pub struct ProfileStats {
    pub total_exports: usize,
    pub unique_dxil: usize,
    pub converted_ok: usize,
    pub failed: usize,
    pub skipped_sm69: usize,
    pub metallib_bytes: usize,
}

/// Per-stage-slot statistics in a [`CorpusReport`].
#[derive(Debug, Clone, Default)]
pub struct StageSlotStats {
    pub programs: usize,
    pub unique_dxil: usize,
    pub exports: usize,
    pub converted_ok: usize,
    pub failed: usize,
    pub skipped_sm69: usize,
}

/// One failed compilation entry in a [`CorpusReport`].
#[derive(Debug, Clone)]
pub struct FailureRecord {
    pub bundle: String,
    pub stage_slot: RfxStageSlot,
    pub entry_point: String,
    pub profile: String,
    pub is_sm69_ser: bool,
    pub features: u64,
    pub code: Option<ErrorCode>,
    pub message: String,
}

/// Comprehensive summary of scanning and prewarming the `.binrfx` corpus.
#[derive(Debug, Clone)]
pub struct CorpusReport {
    pub shader_dir: PathBuf,
    pub converter_fingerprint: String,
    pub backend_description: String,
    pub cache_dir: Option<PathBuf>,
    pub elapsed_secs: f64,
    pub total_bundles: usize,
    pub bundles_with_programs: usize,
    pub material_only_bundles: Vec<String>,
    pub parse_failures: Vec<String>,
    pub total_programs: usize,
    pub unique_dxil_blobs: usize,
    pub unique_shader_hashes: usize,
    pub total_dxil_bytes: usize,
    pub total_exports: usize,
    pub active_sm66_exports: usize,
    pub active_sm66_converted: usize,
    pub active_sm66_failed: usize,
    pub sm69_ser_exports: usize,
    pub sm69_ser_skipped: usize,
    pub sm69_ser_failed: usize,
    pub with_root_signature: usize,
    pub total_metallib_bytes: usize,
    pub cache_stats: CacheStats,
    pub by_profile: BTreeMap<String, ProfileStats>,
    pub by_stage_slot: BTreeMap<RfxStageSlot, StageSlotStats>,
    pub by_metal_stage: BTreeMap<String, usize>,
    pub by_error_code: BTreeMap<String, usize>,
    pub feature_usage: Vec<(String, usize, usize, usize)>,
    pub failures: Vec<FailureRecord>,
}

struct CompileJob {
    bundle: String,
    stage_slot: RfxStageSlot,
    export: RfxExport,
    dxbc_hash: u128,
    dxil: Vec<u8>,
}

struct JobOutcome {
    bundle: String,
    stage_slot: RfxStageSlot,
    export: RfxExport,
    dxbc_hash: u128,
    with_root_signature: bool,
    skipped_sm69: bool,
    result: Result<(ShaderStage, usize), (Option<ErrorCode>, String)>,
}

/// Lists all `.binrfx` bundle paths in `dir`, optionally filtered by substring.
pub fn list_rfx_bundles(dir: &Path, filter: Option<&str>) -> Result<Vec<PathBuf>, Error> {
    let rd = std::fs::read_dir(dir).map_err(|e| Error::Io(format!("{}: {e}", dir.display())))?;
    let mut paths: Vec<PathBuf> = rd
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "binrfx"))
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

/// Scans all `.binrfx` bundles configured by `opts`, compiles every shader export through
/// `pipeline`, and returns a structured [`CorpusReport`].
pub fn scan_and_prewarm_corpus(
    opts: &CorpusOptions,
    pipeline: &ShaderPipeline,
) -> Result<CorpusReport, Error> {
    let t0 = Instant::now();
    let paths = list_rfx_bundles(&opts.shader_dir, opts.filter.as_deref())?;
    let threads = if opts.threads == 0 {
        std::thread::available_parallelism().map_or(8, |n| n.get())
    } else {
        opts.threads
    };

    let mut parse_failures = Vec::new();
    let mut material_only_bundles = Vec::new();
    let mut bundles_with_programs = 0usize;
    let mut total_programs = 0usize;
    let mut total_dxil_bytes = 0usize;
    let mut all_dxbc_hashes = BTreeSet::new();
    let mut all_shader_hashes = BTreeSet::new();
    let mut stage_prog_counts: BTreeMap<RfxStageSlot, (usize, BTreeSet<u128>)> = BTreeMap::new();
    for slot in RfxStageSlot::ALL {
        stage_prog_counts.insert(slot, (0, BTreeSet::new()));
    }

    let mut jobs = Vec::new();
    for p in &paths {
        match RfxBundle::open(p) {
            Ok(rfx) => {
                if rfx.programs.is_empty() {
                    material_only_bundles.push(rfx.file_name.clone());
                } else {
                    bundles_with_programs += 1;
                }
                for prog in rfx.programs {
                    total_programs += 1;
                    total_dxil_bytes += prog.dxbc_bytes.len();
                    all_dxbc_hashes.insert(prog.dxbc_hash);
                    all_shader_hashes.insert(prog.shader_hash);
                    let entry = stage_prog_counts.entry(prog.stage_slot).or_default();
                    entry.0 += 1;
                    entry.1.insert(prog.dxbc_hash);

                    let exports = prog.exports();
                    if exports.is_empty() {
                        parse_failures.push(format!(
                            "{} [{}]: no entry point or RDAT exports",
                            rfx.file_name,
                            prog.stage_slot.name()
                        ));
                    }
                    for export in exports {
                        jobs.push(CompileJob {
                            bundle: rfx.file_name.clone(),
                            stage_slot: prog.stage_slot,
                            export,
                            dxbc_hash: prog.dxbc_hash,
                            dxil: prog.dxbc_bytes.clone(),
                        });
                    }
                }
            }
            Err(e) => parse_failures.push(format!("{}: {e}", p.display())),
        }
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

                if opts.skip_sm69 && job.export.is_sm69_ser() {
                    outcomes.lock().unwrap().push(JobOutcome {
                        bundle: job.bundle.clone(),
                        stage_slot: job.stage_slot,
                        export: job.export.clone(),
                        dxbc_hash: job.dxbc_hash,
                        with_root_signature: false,
                        skipped_sm69: true,
                        result: Err((
                            None,
                            "skipped SM 6.9 SER HitObject permutation (--skip-sm69)".into(),
                        )),
                    });
                    continue;
                }

                let covering = if opts.linear_layout {
                    None
                } else {
                    RootSignatureDesc::covering_dxil(&job.dxil)
                };
                let rs = covering.and_then(|d| pipeline.root_signature(&d.to_blob()).ok());
                let with_root_signature = rs.is_some();
                let req = CompileRequest::new(&job.dxil, rs.as_deref(), &compile_opts)
                    .entry_point(&job.export.entry_point);
                let result = match pipeline.compile(&req) {
                    Ok(c) => Ok((c.stage, c.metallib.len())),
                    Err(e) => {
                        let code = match &e {
                            Error::Convert { code, .. } => Some(*code),
                            _ => None,
                        };
                        Err((code, e.to_string()))
                    }
                };
                outcomes.lock().unwrap().push(JobOutcome {
                    bundle: job.bundle.clone(),
                    stage_slot: job.stage_slot,
                    export: job.export.clone(),
                    dxbc_hash: job.dxbc_hash,
                    with_root_signature,
                    skipped_sm69: false,
                    result,
                });
            });
        }
    });

    let mut outcomes = outcomes.into_inner().unwrap();
    outcomes.sort_by(|a, b| {
        (&a.bundle, a.stage_slot, &a.export.entry_point).cmp(&(
            &b.bundle,
            b.stage_slot,
            &b.export.entry_point,
        ))
    });

    let mut by_profile: BTreeMap<String, ProfileStats> = BTreeMap::new();
    let mut profile_hashes: BTreeMap<String, BTreeSet<u128>> = BTreeMap::new();
    let mut by_stage_slot: BTreeMap<RfxStageSlot, StageSlotStats> = BTreeMap::new();
    for slot in RfxStageSlot::ALL {
        let (progs, uniq) = stage_prog_counts.get(&slot).cloned().unwrap_or_default();
        by_stage_slot.insert(
            slot,
            StageSlotStats {
                programs: progs,
                unique_dxil: uniq.len(),
                ..StageSlotStats::default()
            },
        );
    }
    let mut by_metal_stage: BTreeMap<String, usize> = BTreeMap::new();
    let mut by_error_code: BTreeMap<String, usize> = BTreeMap::new();
    let mut failures = Vec::new();

    let mut active_sm66_exports = 0usize;
    let mut active_sm66_converted = 0usize;
    let mut active_sm66_failed = 0usize;
    let mut sm69_ser_exports = 0usize;
    let mut sm69_ser_skipped = 0usize;
    let mut sm69_ser_failed = 0usize;
    let mut with_root_signature = 0usize;
    let mut total_metallib_bytes = 0usize;

    for o in &outcomes {
        if o.with_root_signature {
            with_root_signature += 1;
        }
        let is_sm69 = o.export.is_sm69_ser();
        if is_sm69 {
            sm69_ser_exports += 1;
        } else {
            active_sm66_exports += 1;
        }

        profile_hashes
            .entry(o.export.profile.clone())
            .or_default()
            .insert(o.dxbc_hash);
        let pstat = by_profile.entry(o.export.profile.clone()).or_default();
        pstat.total_exports += 1;

        let sstat = by_stage_slot.entry(o.stage_slot).or_default();
        sstat.exports += 1;

        match &o.result {
            Ok((stage, len)) => {
                pstat.converted_ok += 1;
                pstat.metallib_bytes += *len;
                sstat.converted_ok += 1;
                total_metallib_bytes += *len;
                if !is_sm69 {
                    active_sm66_converted += 1;
                }
                *by_metal_stage.entry(format!("{stage:?}")).or_default() += 1;
            }
            Err((code, msg)) => {
                if o.skipped_sm69 {
                    pstat.skipped_sm69 += 1;
                    sstat.skipped_sm69 += 1;
                    sm69_ser_skipped += 1;
                } else {
                    pstat.failed += 1;
                    sstat.failed += 1;
                    if is_sm69 {
                        sm69_ser_failed += 1;
                    } else {
                        active_sm66_failed += 1;
                    }
                    let key = match (code, is_sm69) {
                        (Some(c), true) => format!("{c:?} (SM 6.9 SER HitObject)"),
                        (Some(c), false) => format!("{c:?}"),
                        (None, _) => "other".to_string(),
                    };
                    *by_error_code.entry(key).or_default() += 1;
                    failures.push(FailureRecord {
                        bundle: o.bundle.clone(),
                        stage_slot: o.stage_slot,
                        entry_point: o.export.entry_point.clone(),
                        profile: o.export.profile.clone(),
                        is_sm69_ser: is_sm69,
                        features: o.export.features,
                        code: *code,
                        message: msg.clone(),
                    });
                }
            }
        }
    }

    for (prof, stat) in &mut by_profile {
        stat.unique_dxil = profile_hashes.get(prof).map_or(0, BTreeSet::len);
    }

    let mut feature_usage = Vec::new();
    for bit in 0..64u32 {
        let using = outcomes
            .iter()
            .filter(|o| (o.export.features >> bit) & 1 == 1)
            .count();
        if using == 0 {
            continue;
        }
        let sm66_failed = outcomes
            .iter()
            .filter(|o| {
                (o.export.features >> bit) & 1 == 1
                    && !o.export.is_sm69_ser()
                    && o.result.is_err()
            })
            .count();
        let sm69_failed = outcomes
            .iter()
            .filter(|o| {
                (o.export.features >> bit) & 1 == 1
                    && o.export.is_sm69_ser()
                    && o.result.is_err()
            })
            .count();
        let name = FEATURE_NAMES
            .get(bit as usize)
            .copied()
            .unwrap_or("unknown")
            .to_string();
        feature_usage.push((name, using, sm66_failed, sm69_failed));
    }

    let backend_description = match pipeline.helper() {
        Some(h) => format!("helper ({})", h.executable().display()),
        None => "in-process".to_string(),
    };

    Ok(CorpusReport {
        shader_dir: opts.shader_dir.clone(),
        converter_fingerprint: pipeline.converter_fingerprint().to_string(),
        backend_description,
        cache_dir: pipeline.cache().config().disk_dir.clone(),
        elapsed_secs: t0.elapsed().as_secs_f64(),
        total_bundles: paths.len(),
        bundles_with_programs,
        material_only_bundles,
        parse_failures,
        total_programs,
        unique_dxil_blobs: all_dxbc_hashes.len(),
        unique_shader_hashes: all_shader_hashes.len(),
        total_dxil_bytes,
        total_exports: outcomes.len(),
        active_sm66_exports,
        active_sm66_converted,
        active_sm66_failed,
        sm69_ser_exports,
        sm69_ser_skipped,
        sm69_ser_failed,
        with_root_signature,
        total_metallib_bytes,
        cache_stats: pipeline.stats(),
        by_profile,
        by_stage_slot,
        by_metal_stage,
        by_error_code,
        feature_usage,
        failures,
    })
}

impl CorpusReport {
    /// Formats a comprehensive human-readable corpus report.
    pub fn format_report(&self, max_failures: usize) -> String {
        let mut out = String::new();
        let _ = writeln!(
            out,
            "================================================================================"
        );
        let _ = writeln!(
            out,
            "Alan Wake 2 Northlight .binrfx Shader Corpus & Metal Conversion Report"
        );
        let _ = writeln!(
            out,
            "================================================================================"
        );
        let _ = writeln!(out, "Shader directory : {}", self.shader_dir.display());
        let _ = writeln!(out, "Converter        : {}", self.converter_fingerprint);
        let _ = writeln!(out, "Backend          : {}", self.backend_description);
        let _ = writeln!(out, "Cache directory  : {:?}", self.cache_dir);
        let _ = writeln!(out, "Elapsed time     : {:.2}s", self.elapsed_secs);
        let _ = writeln!(out);
        let _ = writeln!(out, "== Corpus Inventory");
        let _ = writeln!(
            out,
            "   Total .binrfx bundles          : {} ({} with shader programs, {} material-only templates)",
            self.total_bundles,
            self.bundles_with_programs,
            self.material_only_bundles.len()
        );
        if !self.material_only_bundles.is_empty() {
            let _ = writeln!(
                out,
                "   Material-only .binrfx (0 progs): {}",
                self.material_only_bundles.join(", ")
            );
        }
        let _ = writeln!(
            out,
            "   Total DXBC/DXIL containers     : {} ({:.1} MB DXIL)",
            self.total_programs,
            self.total_dxil_bytes as f64 / 1e6
        );
        let _ = writeln!(
            out,
            "   Unique DXIL blobs (FNV-1a 128) : {} (unique 64-bit rfx shader_hash: {})",
            self.unique_dxil_blobs, self.unique_shader_hashes
        );
        let _ = writeln!(
            out,
            "   Total shader exports / jobs    : {}",
            self.total_exports
        );
        let sm66_pct = if self.active_sm66_exports > 0 {
            (self.active_sm66_converted as f64 / self.active_sm66_exports as f64) * 100.0
        } else {
            100.0
        };
        let _ = writeln!(
            out,
            "   Active SM <= 6.6 exports       : {}/{} converted ({:.1}%, {} failed, {:.1} MB metallib)",
            self.active_sm66_converted,
            self.active_sm66_exports,
            sm66_pct,
            self.active_sm66_failed,
            self.total_metallib_bytes as f64 / 1e6
        );
        let _ = writeln!(
            out,
            "   SM 6.9 NVIDIA SER permutations : {} (skipped={}, unsupported={}; unused when reporting SM 6.6 + DXR Tier 1.1)",
            self.sm69_ser_exports, self.sm69_ser_skipped, self.sm69_ser_failed
        );
        let _ = writeln!(
            out,
            "   Root signature synthesis       : {}/{} compiled against PSV0/RDAT covering root signature",
            self.with_root_signature,
            self.total_exports.saturating_sub(self.sm69_ser_skipped)
        );

        let st = &self.cache_stats;
        let _ = writeln!(
            out,
            "   Cache stats                    : memory_hits={} disk_hits={} compiles={} failures={} read={:.1}MB written={:.1}MB cpu={:.1}s",
            st.memory_hits,
            st.disk_hits,
            st.compiles,
            st.compile_failures,
            st.disk_bytes_read as f64 / 1e6,
            st.disk_bytes_written as f64 / 1e6,
            st.compile_nanos as f64 / 1e9
        );

        if !self.parse_failures.is_empty() {
            let _ = writeln!(out, "\n== .binrfx Parse Failures ({})", self.parse_failures.len());
            for pf in &self.parse_failures {
                let _ = writeln!(out, "   {pf}");
            }
        }

        let _ = writeln!(
            out,
            "\n== Breakdown by Shader Profile (exports / unique_dxil / converted_ok / failed / metallib)"
        );
        let _ = writeln!(
            out,
            "   {:<18} {:>8} {:>11} {:>12} {:>8} {:>12}",
            "Profile", "Exports", "UniqueDXIL", "ConvertedOK", "Failed", "Metallib"
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

        let _ = writeln!(
            out,
            "\n== Breakdown by .binrfx Stage Slot (programs / unique_dxil / exports / converted_ok / failed)"
        );
        let _ = writeln!(
            out,
            "   {:<15} {:>8} {:>11} {:>8} {:>12} {:>8}",
            "StageSlot", "Programs", "UniqueDXIL", "Exports", "ConvertedOK", "Failed"
        );
        for slot in RfxStageSlot::ALL {
            if let Some(s) = self.by_stage_slot.get(&slot) {
                let _ = writeln!(
                    out,
                    "   {:<15} {:>8} {:>11} {:>8} {:>12} {:>8}",
                    slot.name(),
                    s.programs,
                    s.unique_dxil,
                    s.exports,
                    s.converted_ok,
                    s.failed
                );
            }
        }

        let _ = writeln!(out, "\n== Emitted Metal Stages");
        for (stage, count) in &self.by_metal_stage {
            let _ = writeln!(out, "   {:<18} {:>6}", stage, count);
        }

        if !self.by_error_code.is_empty() {
            let _ = writeln!(out, "\n== Failures by Converter Error Code");
            for (code, count) in &self.by_error_code {
                let _ = writeln!(out, "   {:<48} {:>6}", code, count);
            }
        }

        let _ = writeln!(
            out,
            "\n== SFI0 Feature Flags (total_using / sm66_failed / sm69_ser_failed)"
        );
        let _ = writeln!(
            out,
            "   {:<38} {:>8} {:>11} {:>15}",
            "Feature", "Using", "SM66Failed", "SM69SERFailed"
        );
        for (name, using, sm66_f, sm69_f) in &self.feature_usage {
            let _ = writeln!(
                out,
                "   {:<38} {:>8} {:>11} {:>15}",
                name, using, sm66_f, sm69_f
            );
        }

        let non_sm69_failures: Vec<&FailureRecord> =
            self.failures.iter().filter(|f| !f.is_sm69_ser).collect();
        if !non_sm69_failures.is_empty() && max_failures > 0 {
            let _ = writeln!(
                out,
                "\n== First {} Active (SM <= 6.6) Failures",
                max_failures.min(non_sm69_failures.len())
            );
            for f in non_sm69_failures.iter().take(max_failures) {
                let _ = writeln!(
                    out,
                    "   {} {} [{} {}] {:?}: {}",
                    f.bundle,
                    f.entry_point,
                    f.stage_slot.name(),
                    f.profile,
                    dxbc::feature_names(f.features),
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
    fn reject_invalid_magic() {
        let err = RfxBundle::from_bytes("bad.binrfx", b"NOT_RFX_MAGIC_123456").unwrap_err();
        assert!(err.to_string().contains("invalid .binrfx magic"));
    }

    #[test]
    fn parse_all_shipped_rfx_bundles_and_verify_counts() {
        let Some(game) = std::env::var_os("AW2_GAME_DIR")
            .map(PathBuf::from)
            .or_else(aw2_data::discover_game_dir)
        else {
            return;
        };
        let dir = game.join("data/shaders/build/pc_dx12");
        let Ok(paths) = list_rfx_bundles(&dir, None) else {
            return;
        };
        assert_eq!(paths.len(), 153, "expected 153 .binrfx files in pc_dx12");

        let mut total_progs = 0usize;
        let mut total_exports = 0usize;
        let mut sm66_exports = 0usize;
        let mut sm69_exports = 0usize;
        let mut mesh_progs = 0usize;
        let mut rt_progs = 0usize;

        for p in &paths {
            let rfx = RfxBundle::open(p).unwrap_or_else(|e| panic!("{}: {e}", p.display()));
            for prog in &rfx.programs {
                total_progs += 1;
                if prog.stage_slot == RfxStageSlot::Mesh {
                    mesh_progs += 1;
                }
                if prog.stage_slot == RfxStageSlot::Raytracing {
                    rt_progs += 1;
                }
                for exp in prog.exports() {
                    total_exports += 1;
                    if exp.is_sm69_ser() {
                        sm69_exports += 1;
                    } else {
                        sm66_exports += 1;
                    }
                }
            }
        }

        assert_eq!(total_progs, 2284, "total DXBC/DXIL containers across 153 bundles");
        assert_eq!(mesh_progs, 48, "48 ms_6_5 mesh shaders in slot 3");
        assert_eq!(rt_progs, 930, "930 lib_6_* raytracing libraries in slot 5");
        assert_eq!(total_exports, 2548, "2548 total shader exports");
        assert_eq!(sm66_exports, 1906, "1906 active SM <= 6.6 shader exports");
        assert_eq!(sm69_exports, 642, "642 SM 6.9 SER HitObject exports");
    }
}
