//! `--shaders`: offline validation of every root signature and pipeline state in a trace.
//!
//! Root signatures are built through `ShaderPipeline::root_signature`, and every shader stage
//! is converted with `ShaderPipeline::compile_stage`. Converted shaders land in the normal
//! shader cache, so this also prewarms it. Fixed-function state is translated with
//! `d3d12_metal::{state, topology, format, format_ext}`. Each pipeline then reports conversion
//! failures, invalid state, and the D3D12 features it uses that Metal can't do directly.
//!
//! Conversion runs in parallel. Set `AW2_SHADER_BACKEND=helper` so a converter crash on one
//! shader doesn't take the whole run down.

use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use d3d12_metal::format::{self, Num};
use d3d12_metal::state::{self, BlendDesc, DepthStencilDesc, DepthStencilDesc1, DepthStencilDesc2, RasterizerDesc, RasterizerDesc1, RasterizerDesc2};
use d3d12_metal::trace::{self, Call, GraphicsPipeline, Reader};
use d3d12_metal::{format_ext, topology};
use dxil_metal::dxbc::{Container, FOURCC_DXIL};
use dxil_metal::{CacheStats, CompileOptions, InputElement, RootSignature, ShaderPipeline, ShaderStage, StageInfo};
use objc2_metal::{MTLPixelFormat, MTLPrimitiveTopologyClass};

use crate::pod;
use crate::replay::Capped;

/// Converter vertex-buffer convention: vertex buffer `slot` binds at Metal buffer index
/// `VERTEX_BUFFER_BASE + slot`, which must stay below Metal's 31 buffer slots.
pub const VERTEX_BUFFER_BASE: u32 = 6;
pub const MAX_METAL_BUFFER_INDEX: u32 = 30;

#[derive(Debug, Clone)]
pub struct Options {
    pub until: Option<u64>,
    pub jobs: usize,
    pub max_messages: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self { until: None, jobs: std::thread::available_parallelism().map_or(4, |n| n.get()), max_messages: 200 }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ShaderReport {
    pub root_signatures: u64,
    pub root_signature_failures: Capped,
    pub graphics: u64,
    pub compute: u64,
    pub mesh: u64,
    pub pipelines_failed: u64,
    pub stages_compiled: u64,
    pub failures: Capped,
    /// Feature → number of pipelines using it.
    pub unsupported: BTreeMap<&'static str, u64>,
    pub cache: CacheStats,
    pub seconds: f64,
}

impl std::fmt::Display for ShaderReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "shaders: {:.1} s", self.seconds)?;
        writeln!(f, "  root signatures {} ({} failed)", self.root_signatures, self.root_signature_failures.total)?;
        writeln!(f, "  pipelines: graphics {}, compute {}, mesh {}, failed {}", self.graphics, self.compute, self.mesh, self.pipelines_failed)?;
        writeln!(f, "  stages converted {} (cache: {:?})", self.stages_compiled, self.cache)?;
        if !self.unsupported.is_empty() {
            writeln!(f, "  features without a direct Metal equivalent (pipelines):")?;
            for (k, n) in &self.unsupported {
                writeln!(f, "    {n:>7}  {k}")?;
            }
        }
        for (title, c) in [("root signature failures", &self.root_signature_failures), ("pipeline failures", &self.failures)] {
            if c.total > 0 {
                writeln!(f, "  {title}: {}", c.total)?;
                for (i, m) in &c.items {
                    writeln!(f, "    #{i}: {m}")?;
                }
                if c.total as usize > c.items.len() {
                    writeln!(f, "    ... {} more", c.total as usize - c.items.len())?;
                }
            }
        }
        Ok(())
    }
}

/// What a shader blob contains.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bytecode {
    Dxil,
    /// SM4/SM5 DXBC (`SHDR`/`SHEX`): converted to DXIL by libdxilconv when it's installed.
    Dxbc,
    Unknown,
}

pub fn bytecode_kind(b: &[u8]) -> Bytecode {
    match Container::parse(b) {
        Ok(c) if c.part(FOURCC_DXIL).is_some() => Bytecode::Dxil,
        Ok(c) if c.part(*b"SHEX").is_some() || c.part(*b"SHDR").is_some() => Bytecode::Dxbc,
        _ => Bytecode::Unknown,
    }
}

enum Job<'a> {
    Graphics { index: u64, desc: Box<GraphicsPipeline<'a>>, rs: Result<Option<Arc<RootSignature>>, String> },
    Compute { index: u64, cs: &'a [u8], rs: Result<Option<Arc<RootSignature>>, String> },
    Mesh { index: u64, amplification: &'a [u8], mesh: &'a [u8], desc: Box<GraphicsPipeline<'a>>, rs: Result<Option<Arc<RootSignature>>, String> },
}

#[derive(Debug, Default)]
struct Outcome {
    index: u64,
    failures: Vec<String>,
    unsupported: Vec<&'static str>,
    stages: u64,
}

/// Root signature for a PSO: the recorded object, or (id 0) the one embedded in `shader`.
fn resolve_rs(p: &ShaderPipeline, map: &HashMap<u64, Result<Arc<RootSignature>, String>>, id: u64, shader: &[u8]) -> Result<Option<Arc<RootSignature>>, String> {
    if id == 0 {
        // A shader without an embedded root signature is still convertible (no resources).
        return Ok(p.root_signature(shader).ok());
    }
    match map.get(&id) {
        Some(Ok(rs)) => Ok(Some(rs.clone())),
        Some(Err(e)) => Err(format!("root signature {id:#x} failed: {e}")),
        None => Err(format!("unknown root signature {id:#x}")),
    }
}

pub fn validate(data: &[u8], p: &ShaderPipeline, opts: &Options) -> anyhow::Result<ShaderReport> {
    let start = Instant::now();
    let mut rep = ShaderReport {
        root_signature_failures: Capped::new(opts.max_messages),
        failures: Capped::new(opts.max_messages),
        ..Default::default()
    };
    let mut rs_map: HashMap<u64, Result<Arc<RootSignature>, String>> = HashMap::new();
    let mut jobs = Vec::new();
    for r in Reader::new(data).map_err(|e| anyhow::anyhow!("{}", e.0))? {
        let Ok(rec) = r else { continue };
        if opts.until.is_some_and(|u| rec.index > u) {
            break;
        }
        match rec.call {
            Call::CreateRootSignature { root_signature, blob, .. } => {
                rep.root_signatures += 1;
                let rs = p.root_signature(blob).map_err(|e| e.to_string());
                if let Err(e) = &rs {
                    rep.root_signature_failures.push(rec.index, format!("{root_signature:#x}: {e}"));
                }
                rs_map.insert(root_signature, rs);
            }
            Call::CreateGraphicsPipelineState { desc, .. } => {
                rep.graphics += 1;
                let rs = resolve_rs(p, &rs_map, desc.root_signature, desc.vs);
                jobs.push(Job::Graphics { index: rec.index, desc: Box::new(desc), rs });
            }
            Call::CreateMeshPipelineState { amplification, mesh, desc, .. } => {
                rep.mesh += 1;
                let rs = resolve_rs(p, &rs_map, desc.root_signature, mesh);
                jobs.push(Job::Mesh { index: rec.index, amplification, mesh, desc: Box::new(desc), rs });
            }
            Call::CreateComputePipelineState { root_signature, cs, .. } => {
                rep.compute += 1;
                let rs = resolve_rs(p, &rs_map, root_signature, cs);
                jobs.push(Job::Compute { index: rec.index, cs, rs });
            }
            _ => {}
        }
    }

    let next = AtomicUsize::new(0);
    let results = Mutex::new(Vec::with_capacity(jobs.len()));
    std::thread::scope(|s| {
        for _ in 0..opts.jobs.clamp(1, 64).min(jobs.len().max(1)) {
            s.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                let Some(job) = jobs.get(i) else { break };
                let out = match job {
                    Job::Graphics { index, desc, rs } => graphics(p, *index, desc, rs),
                    Job::Compute { index, cs, rs } => compute(p, *index, cs, rs),
                    Job::Mesh { index, amplification, mesh, desc, rs } => mesh_pso(p, *index, amplification, mesh, desc, rs),
                };
                results.lock().unwrap().push(out);
            });
        }
    });
    let mut results = results.into_inner().unwrap();
    results.sort_by_key(|o| o.index);
    for o in results {
        rep.stages_compiled += o.stages;
        let mut u = o.unsupported;
        u.sort_unstable();
        u.dedup();
        for k in u {
            *rep.unsupported.entry(k).or_insert(0) += 1;
        }
        if !o.failures.is_empty() {
            rep.pipelines_failed += 1;
            rep.failures.push(o.index, o.failures.join("; "));
        }
    }
    rep.cache = p.stats();
    rep.seconds = start.elapsed().as_secs_f64();
    Ok(rep)
}

fn compile(p: &ShaderPipeline, out: &mut Outcome, stage: ShaderStage, blob: &[u8], rs: Option<&RootSignature>, opts: &CompileOptions) -> Option<Arc<dxil_metal::CompiledShader>> {
    match bytecode_kind(blob) {
        Bytecode::Dxil => {}
        Bytecode::Dxbc if dxil_metal::dxilconv::available() => {}
        Bytecode::Dxbc => {
            out.unsupported.push("DXBC (SM4/5) bytecode without libdxilconv");
            out.failures.push(format!("{stage:?}: DXBC bytecode and libdxilconv.dylib not found"));
            return None;
        }
        Bytecode::Unknown => {
            out.failures.push(format!("{stage:?}: not a DXBC container ({} bytes)", blob.len()));
            return None;
        }
    }
    match p.compile_stage(stage, blob, rs, opts) {
        Ok(s) => {
            out.stages += 1;
            Some(s)
        }
        Err(e) => {
            out.failures.push(format!("{stage:?}: {e}"));
            None
        }
    }
}

fn compute(p: &ShaderPipeline, index: u64, cs: &[u8], rs: &Result<Option<Arc<RootSignature>>, String>) -> Outcome {
    let mut out = Outcome { index, ..Default::default() };
    match rs {
        Ok(rs) => {
            compile(p, &mut out, ShaderStage::Compute, cs, rs.as_deref(), &CompileOptions::default());
        }
        Err(e) => out.failures.push(e.clone()),
    }
    out
}

/// Translated fixed-function state of a graphics PSO (also used by tests).
#[derive(Debug, Clone)]
pub struct GraphicsState {
    pub blend: state::BlendState,
    pub raster: state::RasterState,
    pub depth_stencil: state::DepthStencilState,
    pub input_layout: Vec<InputElement>,
    pub color_formats: Vec<MTLPixelFormat>,
    pub depth_format: MTLPixelFormat,
    pub int_rt_mask: u8,
}

/// Translates and checks the fixed-function part of `d`; pushes problems into `out`.
/// `mesh`: a mesh-shader PSO, which has no topology type or input layout.
fn graphics_state(d: &GraphicsPipeline, mesh: bool, out: &mut Outcome) -> Option<GraphicsState> {
    let fail = |out: &mut Outcome, m: String| out.failures.push(m);
    let blend = match pod::read::<BlendDesc>(d.blend) {
        Ok(b) => state::blend_state(&b),
        Err(e) => {
            out.failures.push(format!("blend: {e}"));
            return None;
        }
    };
    if !blend.unsupported_logic_ops.is_empty() {
        out.unsupported.push("blend logic ops");
    }
    if !blend.invalid.is_empty() {
        fail(out, format!("invalid blend enums (rt, value): {:?}", blend.invalid.as_slice()));
    }
    let raster = match d.raster_version {
        0 => pod::read::<RasterizerDesc>(d.rasterizer).map(|r| state::raster_state(&r)),
        1 => pod::read::<RasterizerDesc1>(d.rasterizer).map(|r| state::raster_state1(&r)),
        2 => pod::read::<RasterizerDesc2>(d.rasterizer).map(|r| state::raster_state2(&r)),
        v => Err(format!("unknown rasterizer desc version {v}")),
    };
    let raster = match raster {
        Ok(r) => r,
        Err(e) => {
            out.failures.push(format!("rasterizer: {e}"));
            return None;
        }
    };
    if raster.unsupported_conservative {
        out.unsupported.push("conservative rasterization");
    }
    if raster.forced_sample_count > 0 {
        out.unsupported.push("forced sample count (UAV-only rasterization)");
    }
    if raster.non_aliased_lines {
        out.unsupported.push("antialiased/wide lines");
    }
    let depth_stencil = match d.depth_stencil_version {
        0 => pod::read::<DepthStencilDesc>(d.depth_stencil).map(|s| state::depth_stencil_state(&s)),
        1 => pod::read::<DepthStencilDesc1>(d.depth_stencil).map(|s| state::depth_stencil_state1(&s)),
        2 => pod::read::<DepthStencilDesc2>(d.depth_stencil).map(|s| state::depth_stencil_state2(&s)),
        v => Err(format!("unknown depth-stencil desc version {v}")),
    };
    let depth_stencil = match depth_stencil {
        Ok(s) => s,
        Err(e) => {
            out.failures.push(format!("depth-stencil: {e}"));
            return None;
        }
    };

    // Topology type.
    let ty = d.primitive_topology_type;
    if mesh {
        // D3D12 ignores the topology type of mesh PSOs.
    } else if topology::topology_class(ty) == MTLPrimitiveTopologyClass::Unspecified && ty != topology::topology_type::PATCH {
        fail(out, format!("primitive topology type {ty} is not point/line/triangle/patch"));
    }
    if !mesh && ty == topology::topology_type::PATCH && d.hs.is_empty() {
        fail(out, "PATCH topology type without a hull shader".into());
    }

    // Render targets.
    let mut color_formats = Vec::new();
    let mut int_rt_mask = 0u8;
    if d.rtv_formats.len() > 8 {
        fail(out, format!("{} render target formats", d.rtv_formats.len()));
    }
    for (i, f) in d.rtv_formats.iter().enumerate().take(8) {
        let info = format::info(f);
        if f != 0 && info.mtl == MTLPixelFormat::Invalid {
            fail(out, format!("RT{i} format {} has no Metal pixel format", format::name(f)));
        }
        if matches!(info.num, Num::Uint | Num::Sint) {
            int_rt_mask |= 1 << i;
        }
        color_formats.push(info.mtl);
    }
    if blend.dual_source && d.rtv_formats.iter().skip(1).any(|f| f != 0) {
        fail(out, "dual-source blending with more than one render target".into());
    }
    let depth_format = format::info(d.dsv_format).mtl;
    if d.dsv_format != 0 && (depth_format == MTLPixelFormat::Invalid || !format::info(d.dsv_format).is_depth_stencil()) {
        fail(out, format!("DSV format {} is not a Metal depth/stencil format", format::name(d.dsv_format)));
    }

    // Input layout.
    let mut input_layout = Vec::new();
    match if mesh { Ok(Vec::new()) } else { trace::decode_input_layout(d.input_layout) } {
        Ok(els) => {
            for e in els {
                if format_ext::vertex_format(e.format).is_none() {
                    fail(out, format!("input {}{} format {} has no Metal vertex format", e.semantic_name, e.semantic_index, format::name(e.format)));
                }
                if VERTEX_BUFFER_BASE + e.input_slot > MAX_METAL_BUFFER_INDEX {
                    fail(out, format!("input {}{} uses vertex buffer slot {} (> {} with base {VERTEX_BUFFER_BASE})", e.semantic_name, e.semantic_index, e.input_slot, MAX_METAL_BUFFER_INDEX - VERTEX_BUFFER_BASE));
                }
                input_layout.push(InputElement {
                    semantic_name: e.semantic_name,
                    semantic_index: e.semantic_index,
                    format: e.format,
                    input_slot: e.input_slot,
                    aligned_byte_offset: e.aligned_byte_offset,
                    input_slot_class: e.input_slot_class,
                    instance_data_step_rate: e.instance_data_step_rate,
                });
            }
            dxil_metal::resolve_append_aligned(&mut input_layout, format_ext::vertex_format_size);
        }
        Err(e) => fail(out, format!("input layout: {}", e.0)),
    }

    Some(GraphicsState { blend, raster, depth_stencil, input_layout, color_formats, depth_format, int_rt_mask })
}

fn graphics(p: &ShaderPipeline, index: u64, d: &GraphicsPipeline, rs: &Result<Option<Arc<RootSignature>>, String>) -> Outcome {
    let mut out = Outcome { index, ..Default::default() };
    let has_tess = !d.hs.is_empty() || !d.ds.is_empty();
    if has_tess {
        out.unsupported.push("tessellation (HS/DS)");
    }
    if !d.gs.is_empty() {
        out.unsupported.push("geometry shader");
    }
    // D3D12_STREAM_OUTPUT_DESC: pSODeclaration (8), NumEntries (4) ...
    if d.stream_output.get(8..12).is_some_and(|n| u32::from_le_bytes(n.try_into().unwrap()) != 0) {
        out.unsupported.push("stream output");
    }
    let Some(st) = graphics_state(d, false, &mut out) else { return out };
    let rs = match rs {
        Ok(rs) => rs.as_deref(),
        Err(e) => {
            out.failures.push(e.clone());
            return out;
        }
    };
    if d.vs.is_empty() {
        out.failures.push("no vertex shader".into());
        return out;
    }

    let mut base = CompileOptions::default();
    if has_tess || !d.gs.is_empty() {
        base.input_topology = Some(topology::input_topology(d.primitive_topology_type));
    }
    if let Some(vs) = compile(p, &mut out, ShaderStage::Vertex, d.vs, rs, &base) {
        // Every vertex shader input must come from the input layout.
        if let StageInfo::Vertex { inputs, .. } = &vs.reflection.info {
            for a in inputs {
                let found = st.input_layout.iter().any(|e| format!("{}{}", e.semantic_name, e.semantic_index).eq_ignore_ascii_case(&a.name));
                if !found {
                    out.failures.push(format!("vertex input {} is not in the input layout", a.name));
                }
            }
        }
    }
    for (stage, blob) in [(ShaderStage::Hull, d.hs), (ShaderStage::Domain, d.ds), (ShaderStage::Geometry, d.gs)] {
        if !blob.is_empty() {
            compile(p, &mut out, stage, blob, rs, &base);
        }
    }
    if !d.ps.is_empty() {
        compile(p, &mut out, ShaderStage::Fragment, d.ps, rs, &pixel_options(&st, d));
    }
    out
}

/// The runtime's pixel-shader options (same cache keys as the PSO path): the shared blend/RT
/// rule, plus the sample mask when it doesn't cover every sample.
fn pixel_options(st: &GraphicsState, d: &GraphicsPipeline) -> CompileOptions {
    let mut o = state::pixel_compile_options(&st.blend, &d.rtv_formats.to_vec());
    let n = d.sample_count.max(1);
    let full = if n >= 32 { u32::MAX } else { (1u32 << n) - 1 };
    if d.sample_mask & full != full {
        o.sample_mask = Some(d.sample_mask);
    }
    o
}

/// A mesh-shader PSO: amplification (optional), mesh and pixel stages.
fn mesh_pso(p: &ShaderPipeline, index: u64, amplification: &[u8], mesh: &[u8], d: &GraphicsPipeline, rs: &Result<Option<Arc<RootSignature>>, String>) -> Outcome {
    let mut out = Outcome { index, ..Default::default() };
    let Some(st) = graphics_state(d, true, &mut out) else { return out };
    let rs = match rs {
        Ok(rs) => rs.as_deref(),
        Err(e) => {
            out.failures.push(e.clone());
            return out;
        }
    };
    if mesh.is_empty() {
        out.failures.push("mesh PSO without a mesh shader".into());
        return out;
    }
    let base = CompileOptions::default();
    if !amplification.is_empty() {
        compile(p, &mut out, ShaderStage::Amplification, amplification, rs, &base);
    }
    if let Some(ms) = compile(p, &mut out, ShaderStage::Mesh, mesh, rs, &base) {
        if let StageInfo::Mesh { max_payload_size_in_bytes, .. } = ms.reflection.info {
            if max_payload_size_in_bytes > d3d12_metal::mesh::MAX_PAYLOAD_BYTES {
                out.failures.push(format!("mesh payload of {max_payload_size_in_bytes} bytes"));
            }
        }
    }
    if !d.ps.is_empty() {
        compile(p, &mut out, ShaderStage::Fragment, d.ps, rs, &pixel_options(&st, d));
    }
    out
}

/// Translates a graphics PSO's fixed-function state; `Err` lists the problems.
pub fn translate_graphics_state(d: &GraphicsPipeline) -> Result<(GraphicsState, Vec<&'static str>), Vec<String>> {
    let mut out = Outcome::default();
    match graphics_state(d, false, &mut out) {
        Some(s) if out.failures.is_empty() => Ok((s, out.unsupported)),
        _ => Err(out.failures),
    }
}
