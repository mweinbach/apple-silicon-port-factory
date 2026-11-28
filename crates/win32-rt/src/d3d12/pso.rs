//! ID3D12RootSignature (and, later, pipeline state objects).
//!
//! Root signatures are parsed and laid out by `dxil_metal`
//! (`ShaderPipeline::root_signature`, deduplicated by content); the COM
//! object just holds the shared `Arc`.

use super::com::*;
use super::tr;
use super::failed;
use super::vtables::*;
use crate::types::*;
use std::ffi::c_void;
use std::sync::{Arc, OnceLock};

pub struct RootSignature {
    pub rs: Arc<dxil_metal::root_signature::RootSignature>,
}

static RS_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for RootSignature {
    fn class() -> &'static Class {
        class_once(&RS_CLASS, || Class {
            name: "ID3D12RootSignature",
            vtbl: build_named::<RootSignature>("ID3D12RootSignature", ID3D12RootSignature, &[]),
            iids: &[IID_ID3D12Object, IID_ID3D12DeviceChild, IID_ID3D12RootSignature],
        })
    }
}

pub unsafe fn root_signature_from_ptr<'a>(p: *mut c_void) -> Option<&'a RootSignature> {
    Obj::<RootSignature>::from_iface(p).map(|o| &(*o).inner)
}

pub unsafe fn create_root_signature(blob: *const u8, size: usize, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if blob.is_null() || size == 0 {
        return failed!(E_INVALIDARG, "CreateRootSignature: empty blob");
    }
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    let bytes = std::slice::from_raw_parts(blob, size);
    let p = match dxil_metal::ShaderPipeline::global() {
        Ok(p) => p,
        Err(e) => return failed!(E_FAIL, "CreateRootSignature: shader pipeline unavailable: {e}"),
    };
    let rs = match p.root_signature(bytes) {
        Ok(rs) => rs,
        Err(e) => return failed!(E_INVALIDARG, "CreateRootSignature({size} bytes): {e}"),
    };
    if out.is_null() {
        return S_FALSE;
    }
    log::debug!(target: "d3d", "CreateRootSignature({size} bytes): {} parameters, {} static samplers", rs.desc().parameters.len(), rs.desc().static_samplers.len());
    let o = Obj::new(RootSignature { rs });
    let hr = query_out(o as *mut c_void, riid, out);
    Obj::release(o);
    hr
}

// ---------------------------------------------------------------------------
// Command signatures (ExecuteIndirect layouts)
// ---------------------------------------------------------------------------

/// `D3D12_INDIRECT_ARGUMENT_DESC` (16 bytes): Type + union.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IndirectArgument {
    pub ty: u32,
    pub u: [u32; 3],
}

/// `D3D12_COMMAND_SIGNATURE_DESC`.
#[repr(C)]
pub struct CommandSignatureDesc {
    pub byte_stride: u32,
    pub num_args: u32,
    pub args: *const IndirectArgument,
    pub node_mask: u32,
}

pub struct CommandSignature {
    pub stride: u32,
    pub args: Vec<IndirectArgument>,
    pub root_signature: Option<Arc<dxil_metal::root_signature::RootSignature>>,
    /// Err = a layout we cannot execute yet (ExecuteIndirect traps with it).
    pub layout: Result<d3d12_metal::metal::indirect::IndirectLayout, String>,
}

static CS_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for CommandSignature {
    fn class() -> &'static Class {
        class_once(&CS_CLASS, || Class {
            name: "ID3D12CommandSignature",
            vtbl: build_named::<CommandSignature>("ID3D12CommandSignature", ID3D12CommandSignature, &[]),
            iids: &[IID_ID3D12Object, IID_ID3D12DeviceChild, IID_ID3D12Pageable, IID_ID3D12CommandSignature],
        })
    }
}

pub unsafe fn command_signature_from_ptr<'a>(p: *mut c_void) -> Option<&'a CommandSignature> {
    Obj::<CommandSignature>::from_iface(p).map(|o| &(*o).inner)
}

pub unsafe fn create_command_signature(desc: *const CommandSignatureDesc, rs: *mut c_void, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if desc.is_null() || (*desc).args.is_null() || (*desc).num_args == 0 {
        return failed!(E_INVALIDARG, "CreateCommandSignature: empty desc");
    }
    let d = &*desc;
    let args = std::slice::from_raw_parts(d.args, d.num_args as usize).to_vec();
    // Only the last argument may be a draw/dispatch; anything before it
    // changes root arguments and needs a root signature.
    let needs_rs = args.iter().any(|a| matches!(a.ty, 5 | 6 | 7 | 8 | 11));
    let root_signature = root_signature_from_ptr(rs).map(|r| r.rs.clone());
    if needs_rs && root_signature.is_none() {
        return failed!(E_INVALIDARG, "CreateCommandSignature({args:?}): root arguments without a root signature");
    }
    if out.is_null() {
        return S_FALSE;
    }
    *out = std::ptr::null_mut();
    log::debug!(target: "d3d", "CreateCommandSignature(stride {}, {args:?})", d.byte_stride);
    let layout = d3d12_metal::metal::indirect::IndirectLayout::new(d.byte_stride, &args.iter().map(|a| (a.ty, a.u)).collect::<Vec<_>>());
    if let Err(e) = &layout {
        log::warn!(target: "d3d", "CreateCommandSignature({args:?}): {e} (ExecuteIndirect with it will trap)");
    }
    let o = Obj::new(CommandSignature { stride: d.byte_stride, args, root_signature, layout });
    let hr = query_out(o as *mut c_void, riid, out);
    Obj::release(o);
    hr
}

// ---------------------------------------------------------------------------
// Pipeline state objects
// ---------------------------------------------------------------------------

pub enum PipelineKind {
    Compute(Arc<d3d12_metal::metal::pipeline::ComputePipeline>),
    Graphics(Arc<d3d12_metal::metal::pipeline::GraphicsPipeline>),
    Mesh(Arc<d3d12_metal::mesh::MeshPipeline>),
}

pub struct PipelineState {
    pub kind: PipelineKind,
    /// The PSO's root signature COM object (AddRef'd; `GetRootSignature`).
    pub root_signature: usize,
}

impl Drop for PipelineState {
    fn drop(&mut self) {
        unsafe { com_release(self.root_signature as *mut c_void) };
    }
}

static PSO_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for PipelineState {
    fn class() -> &'static Class {
        class_once(&PSO_CLASS, || Class {
            name: "ID3D12PipelineState1",
            vtbl: build_named::<PipelineState>("ID3D12PipelineState1", ID3D12PipelineState1, &[m!("GetRootSignature" => pso_get_root_signature)]),
            iids: &[IID_ID3D12Object, IID_ID3D12DeviceChild, IID_ID3D12Pageable, IID_ID3D12PipelineState, IID_ID3D12PipelineState1],
        })
    }
}

pub unsafe fn pso_from_ptr<'a>(p: *mut c_void) -> Option<&'a PipelineState> {
    Obj::<PipelineState>::from_iface(p).map(|o| &(*o).inner)
}

unsafe extern "win64" fn pso_get_root_signature(this: *mut Obj<PipelineState>, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    query_out(Obj::get(this).root_signature as *mut c_void, riid, out)
}

/// `D3D12_SHADER_BYTECODE`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Bytecode {
    pub ptr: *const u8,
    pub len: usize,
}

impl Bytecode {
    pub unsafe fn bytes<'a>(&self) -> Option<&'a [u8]> {
        (!self.ptr.is_null() && self.len != 0).then(|| std::slice::from_raw_parts(self.ptr, self.len))
    }
}

/// `D3D12_COMPUTE_PIPELINE_STATE_DESC` (56 bytes).
#[repr(C)]
pub struct ComputePipelineDesc {
    pub root_signature: *mut c_void,
    pub cs: Bytecode,
    pub node_mask: u32,
    pub cached: Bytecode,
    pub flags: u32,
}
const _: () = assert!(std::mem::size_of::<ComputePipelineDesc>() == 56);

unsafe fn finish_pso(kind: PipelineKind, rs: *mut c_void, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    com_add_ref(rs);
    let o = Obj::new(PipelineState { kind, root_signature: rs as usize });
    let hr = query_out(o as *mut c_void, riid, out);
    Obj::release(o);
    hr
}

pub unsafe fn create_compute_pipeline(desc: *const ComputePipelineDesc, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    if desc.is_null() {
        return failed!(E_INVALIDARG, "CreateComputePipelineState: null desc");
    }
    let d = &*desc;
    let Some(rs) = root_signature_from_ptr(d.root_signature) else {
        return failed!(E_INVALIDARG, "CreateComputePipelineState: no/foreign root signature {:?}", d.root_signature);
    };
    let Some(cs) = d.cs.bytes() else { return failed!(E_INVALIDARG, "CreateComputePipelineState: no CS") };
    let scope = crate::hitch::Scope::enter(crate::hitch::ActKind::PsoCompile, cs.len() as u64);
    let p = match d3d12_metal::metal::pipeline::ComputePipeline::new(rs.rs.clone(), cs, "") {
        Ok(p) => p,
        Err(e) => return failed!(E_INVALIDARG, "CreateComputePipelineState({} byte CS): {e}", cs.len()),
    };
    let elapsed_ns = scope.elapsed_ns();
    drop(scope);
    crate::hitch::record_pso("CS", &p.shader.function_name, elapsed_ns);
    log::debug!(target: "d3d", "CreateComputePipelineState: {} threadgroup {:?} in {:?}", p.shader.function_name, p.threadgroup, std::time::Duration::from_nanos(elapsed_ns));
    if out.is_null() {
        return S_FALSE;
    }
    finish_pso(PipelineKind::Compute(Arc::new(p)), d.root_signature, riid, out)
}

// ---------------------------------------------------------------------------
// Graphics PSOs (D3D12_GRAPHICS_PIPELINE_STATE_DESC and pipeline state streams)
// ---------------------------------------------------------------------------

use d3d12_metal::state::{self as dstate, BlendDesc, RenderTargetBlendDesc};

/// `D3D12_INPUT_ELEMENT_DESC` (32 bytes).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct InputElementDesc {
    pub name: *const u8,
    pub semantic_index: u32,
    pub format: u32,
    pub input_slot: u32,
    pub aligned_byte_offset: u32,
    pub slot_class: u32,
    pub step_rate: u32,
}
const _: () = assert!(std::mem::size_of::<InputElementDesc>() == 32);

fn default_blend() -> BlendDesc {
    let rt = RenderTargetBlendDesc { BlendEnable: 0, LogicOpEnable: 0, SrcBlend: 2, DestBlend: 1, BlendOp: 1, SrcBlendAlpha: 2, DestBlendAlpha: 1, BlendOpAlpha: 1, LogicOp: 4, RenderTargetWriteMask: 0xF };
    BlendDesc { AlphaToCoverageEnable: 0, IndependentBlendEnable: 0, RenderTarget: [rt; 8] }
}

/// Everything a graphics PSO description carries, gathered from either
/// creation path.
struct GfxParts {
    root_signature: *mut c_void,
    vs: Option<Bytecode>,
    ps: Option<Bytecode>,
    ds: Option<Bytecode>,
    hs: Option<Bytecode>,
    gs: Option<Bytecode>,
    cs: Option<Bytecode>,
    amplification: Option<Bytecode>,
    mesh: Option<Bytecode>,
    other_stage: Option<&'static str>,
    stream_output: bool,
    blend: BlendDesc,
    sample_mask: u32,
    raster: dstate::RasterState,
    depth_stencil: dstate::DepthStencilState,
    input_layout: (*const InputElementDesc, u32),
    strip_cut: u32,
    topology_type: u32,
    rtv_formats: Vec<u32>,
    dsv_format: u32,
    sample_count: u32,
    flags: u32,
    // Raw Windows-layout pieces for the call trace.
    sample_quality: u32,
    node_mask: u32,
    /// (ptr, len, version) of the rasterizer / depth-stencil desc.
    raster_raw: (*const u8, usize, u8),
    ds_raw: (*const u8, usize, u8),
    /// D3D12_STREAM_OUTPUT_DESC (32 bytes) or null.
    so_raw: *const u8,
}

impl Default for GfxParts {
    fn default() -> Self {
        GfxParts {
            root_signature: std::ptr::null_mut(),
            vs: None,
            ps: None,
            ds: None,
            hs: None,
            gs: None,
            cs: None,
            amplification: None,
            mesh: None,
            other_stage: None,
            stream_output: false,
            blend: default_blend(),
            sample_mask: u32::MAX,
            raster: dstate::RasterState::default(),
            depth_stencil: dstate::DepthStencilState::default(),
            input_layout: (std::ptr::null(), 0),
            strip_cut: 0,
            topology_type: 0,
            rtv_formats: Vec::new(),
            dsv_format: 0,
            sample_count: 1,
            flags: 0,
            sample_quality: 0,
            node_mask: 0,
            raster_raw: (std::ptr::null(), 0, 0),
            ds_raw: (std::ptr::null(), 0, 0),
            so_raw: std::ptr::null(),
        }
    }
}

unsafe fn rd<T: Copy>(p: *const u8, off: usize) -> T {
    std::ptr::read_unaligned(p.add(off) as *const T)
}

fn present(b: Bytecode) -> Option<Bytecode> {
    (!b.ptr.is_null() && b.len != 0).then_some(b)
}

/// `D3D12_GRAPHICS_PIPELINE_STATE_DESC` (656 bytes).
unsafe fn parse_desc(p: *const u8) -> GfxParts {
    let num_rt = (rd::<u32>(p, 576) as usize).min(8);
    let so_entries: u32 = rd(p, 96);
    GfxParts {
        root_signature: rd(p, 0),
        vs: present(rd(p, 8)),
        ps: present(rd(p, 24)),
        ds: present(rd(p, 40)),
        hs: present(rd(p, 56)),
        gs: present(rd(p, 72)),
        stream_output: so_entries != 0,
        blend: rd(p, 120),
        sample_mask: rd(p, 448),
        raster: dstate::raster_state(&rd(p, 452)),
        depth_stencil: dstate::depth_stencil_state(&rd(p, 496)),
        input_layout: (rd(p, 552), rd(p, 560)),
        strip_cut: rd(p, 568),
        topology_type: rd(p, 572),
        rtv_formats: (0..num_rt).map(|i| rd::<u32>(p, 580 + 4 * i)).collect(),
        dsv_format: rd(p, 612),
        sample_count: rd(p, 616),
        flags: rd(p, 648),
        sample_quality: rd(p, 620),
        node_mask: rd(p, 624),
        raster_raw: (p.add(452), 44, 0),
        ds_raw: (p.add(496), 52, 0),
        so_raw: p.add(88),
        ..Default::default()
    }
}

/// A `D3D12_PIPELINE_STATE_STREAM_DESC` stream: pointer-aligned
/// `{ type; inner }` subobjects.
unsafe fn parse_stream(p: *const u8, size: usize) -> Result<GfxParts, String> {
    let mut g = GfxParts::default();
    let mut off = 0usize;
    while off + 4 <= size {
        let ty: u32 = rd(p, off);
        // (inner alignment, inner size)
        let (al, sz) = match ty {
            0 => (8, 8),
            1..=6 | 24 | 25 => (8, 16),
            7 => (8, 32),
            8 => (4, 328),
            9 | 13 | 14 | 16 | 18 | 20 => (4, 4),
            10 | 27 => (4, 44),
            11 => (4, 52),
            12 => (8, 16),
            15 => (4, 36),
            17 => (4, 8),
            19 => (8, 16),
            21 => (4, 56),
            22 => (8, 24),
            26 => (4, 60),
            28 => (4, 40),
            _ => return Err(format!("pipeline state subobject type {ty} at offset {off}")),
        };
        let i = (off + 4).next_multiple_of(al);
        if i + sz > size {
            return Err(format!("subobject {ty} overruns the stream ({} > {size})", i + sz));
        }
        let q = p.add(i);
        match ty {
            0 => g.root_signature = rd(q, 0),
            1 => g.vs = present(rd(q, 0)),
            2 => g.ps = present(rd(q, 0)),
            3 => g.ds = present(rd(q, 0)),
            4 => g.hs = present(rd(q, 0)),
            5 => g.gs = present(rd(q, 0)),
            6 => g.cs = present(rd(q, 0)),
            24 => g.amplification = present(rd(q, 0)),
            25 => g.mesh = present(rd(q, 0)),
            7 => {
                g.stream_output = rd::<u32>(q, 8) != 0;
                g.so_raw = q;
            }
            8 => g.blend = rd(q, 0),
            9 => g.sample_mask = rd(q, 0),
            10 => (g.raster, g.raster_raw) = (dstate::raster_state(&rd(q, 0)), (q, 44, 0)),
            27 => (g.raster, g.raster_raw) = (dstate::raster_state1(&rd(q, 0)), (q, 44, 1)),
            28 => (g.raster, g.raster_raw) = (dstate::raster_state2(&rd(q, 0)), (q, 40, 2)),
            11 => (g.depth_stencil, g.ds_raw) = (dstate::depth_stencil_state(&rd(q, 0)), (q, 52, 0)),
            21 => (g.depth_stencil, g.ds_raw) = (dstate::depth_stencil_state1(&rd(q, 0)), (q, 56, 1)),
            26 => (g.depth_stencil, g.ds_raw) = (dstate::depth_stencil_state2(&rd(q, 0)), (q, 60, 2)),
            12 => g.input_layout = (rd(q, 0), rd(q, 8)),
            13 => g.strip_cut = rd(q, 0),
            14 => g.topology_type = rd(q, 0),
            15 => {
                let n = (rd::<u32>(q, 32) as usize).min(8);
                g.rtv_formats = (0..n).map(|k| rd::<u32>(q, 4 * k)).collect();
            }
            16 => g.dsv_format = rd(q, 0),
            17 => (g.sample_count, g.sample_quality) = (rd(q, 0), rd(q, 4)),
            18 => g.node_mask = rd(q, 0),
            20 => g.flags = rd(q, 0),
            22 => {
                if rd::<u32>(q, 0) > 1 {
                    g.other_stage = Some("view instancing");
                }
            }
            _ => {} // node mask, cached PSO
        }
        off = (i + sz).next_multiple_of(8);
    }
    Ok(g)
}

unsafe fn input_layout(l: (*const InputElementDesc, u32)) -> Vec<dxil_metal::InputElement> {
    if l.0.is_null() || l.1 == 0 {
        return Vec::new();
    }
    std::slice::from_raw_parts(l.0, l.1 as usize)
        .iter()
        .map(|e| dxil_metal::InputElement {
            semantic_name: if e.name.is_null() { String::new() } else { String::from_utf8_lossy(crate::strings::cstr_slice(e.name)).into_owned() },
            semantic_index: e.semantic_index,
            format: e.format,
            input_slot: e.input_slot,
            aligned_byte_offset: e.aligned_byte_offset,
            input_slot_class: e.slot_class,
            instance_data_step_rate: e.step_rate,
        })
        .collect()
}

/// Creation plus the call-trace record (failures are recorded with pso 0,
/// so offline validation sees them too).
unsafe fn create_graphics(g: GfxParts, what: &str, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    // `g` moves into the creation; keep owned copies of what the record borrows.
    let rec = tr::on().then(|| (trace_parts(&g), g.rtv_formats.clone(), g.blend));
    let (cs, amp, ms) = (g.cs, g.amplification, g.mesh);
    let (rs, flags) = (g.root_signature, g.flags);
    let hr = create_graphics_inner(g, what, riid, out);
    if let Some(((il, parts), rtvs, blend)) = rec {
        let pso = tr::created(hr, out).unwrap_or(0);
        if let Some(cs) = cs {
            tr::record(&tr::Call::CreateComputePipelineState { device: tr::device(), pso, root_signature: rs as u64, cs: cs.bytes().unwrap_or(&[]), flags });
        } else {
            let mut d = parts;
            d.input_layout = &il;
            d.rtv_formats = tr::U32s::from_slice(&rtvs);
            d.blend = std::slice::from_raw_parts(&blend as *const BlendDesc as *const u8, std::mem::size_of::<BlendDesc>());
            if let Some(ms) = ms {
                let amplification = amp.and_then(|b| b.bytes()).unwrap_or(&[]);
                tr::record(&tr::Call::CreateMeshPipelineState { device: tr::device(), pso, amplification, mesh: ms.bytes().unwrap_or(&[]), desc: d });
            } else {
                tr::record(&tr::Call::CreateGraphicsPipelineState { device: tr::device(), pso, desc: d });
            }
        }
    }
    hr
}

/// The trace form of a PSO description. Shader/state pointers stay valid for
/// the call; the input layout bytes are returned separately, and the caller
/// fills `rtv_formats`/`blend` from copies it owns.
unsafe fn trace_parts<'a>(g: &GfxParts) -> (Vec<u8>, d3d12_metal::trace::GraphicsPipeline<'a>) {
    let il: Vec<d3d12_metal::trace::InputElement> = input_layout(g.input_layout)
        .into_iter()
        .map(|e| d3d12_metal::trace::InputElement {
            semantic_name: e.semantic_name,
            semantic_index: e.semantic_index,
            format: e.format,
            input_slot: e.input_slot,
            aligned_byte_offset: e.aligned_byte_offset,
            input_slot_class: e.input_slot_class,
            instance_data_step_rate: e.instance_data_step_rate,
        })
        .collect();
    let b = |c: Option<Bytecode>| c.and_then(|c| c.bytes()).unwrap_or(&[]);
    let raw = |(p, n, _): (*const u8, usize, u8)| if p.is_null() { &[][..] } else { std::slice::from_raw_parts(p, n) };
    let d = d3d12_metal::trace::GraphicsPipeline {
        root_signature: g.root_signature as u64,
        vs: b(g.vs),
        ps: b(g.ps),
        ds: b(g.ds),
        hs: b(g.hs),
        gs: b(g.gs),
        stream_output: if g.so_raw.is_null() || !g.stream_output { &[] } else { std::slice::from_raw_parts(g.so_raw, 32) },
        blend: &[],
        sample_mask: g.sample_mask,
        rasterizer: raw(g.raster_raw),
        raster_version: g.raster_raw.2,
        depth_stencil: raw(g.ds_raw),
        depth_stencil_version: g.ds_raw.2,
        input_layout: &[],
        ib_strip_cut_value: g.strip_cut,
        primitive_topology_type: g.topology_type,
        rtv_formats: tr::U32s::default(),
        dsv_format: g.dsv_format,
        sample_count: g.sample_count,
        sample_quality: g.sample_quality,
        node_mask: g.node_mask,
        flags: g.flags,
    };
    (d3d12_metal::trace::encode_input_layout(&il), d)
}

unsafe fn create_graphics_inner(g: GfxParts, what: &str, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    let Some(rs) = root_signature_from_ptr(g.root_signature) else {
        return failed!(E_INVALIDARG, "{what}: no/foreign root signature {:?}", g.root_signature);
    };
    if let Some(cs) = g.cs {
        let Some(bytes) = cs.bytes() else { return failed!(E_INVALIDARG, "{what}: empty CS") };
        let scope = crate::hitch::Scope::enter(crate::hitch::ActKind::PsoCompile, bytes.len() as u64);
        let p = match d3d12_metal::metal::pipeline::ComputePipeline::new(rs.rs.clone(), bytes, "") {
            Ok(p) => p,
            Err(e) => return failed!(E_INVALIDARG, "{what}({} byte CS): {e}", bytes.len()),
        };
        let elapsed_ns = scope.elapsed_ns();
        drop(scope);
        crate::hitch::record_pso("CS", &p.shader.function_name, elapsed_ns);
        if out.is_null() {
            return S_FALSE;
        }
        return finish_pso(PipelineKind::Compute(Arc::new(p)), g.root_signature, riid, out);
    }
    if g.stream_output {
        return failed!(E_NOTIMPL, "{what}: stream output pipelines are not implemented");
    }
    if let Some(s) = g.other_stage {
        return failed!(E_NOTIMPL, "{what}: {s} pipelines are not implemented");
    }
    if let Some(ms) = g.mesh.and_then(|b| b.bytes()) {
        if !d3d12_metal::mesh::enabled() {
            return failed!(E_NOTIMPL, "{what}: mesh shader pipelines disabled (set PORT_MESH_SHADERS=1)");
        }
        if g.vs.is_some() || g.hs.is_some() || g.ds.is_some() || g.gs.is_some() {
            return failed!(E_INVALIDARG, "{what}: mesh shader cannot be combined with VS/HS/DS/GS");
        }
        let desc = d3d12_metal::mesh::MeshDesc {
            amplification: g.amplification.and_then(|b| b.bytes()),
            mesh: ms,
            pixel: g.ps.and_then(|b| b.bytes()),
            blend: g.blend,
            sample_mask: g.sample_mask,
            raster: g.raster,
            depth_stencil: g.depth_stencil,
            rtv_formats: g.rtv_formats.clone(),
            dsv_format: g.dsv_format,
            sample_count: g.sample_count.max(1),
        };
        let scope = crate::hitch::Scope::enter(crate::hitch::ActKind::PsoCompile, ms.len() as u64);
        let p = match d3d12_metal::mesh::MeshPipeline::new(rs.rs.clone(), &desc, "") {
            Ok(p) => p,
            Err(e) => return failed!(E_INVALIDARG, "{what}(AS {} bytes, MS {} bytes): {e}", desc.amplification.map_or(0, |b| b.len()), ms.len()),
        };
        let elapsed_ns = scope.elapsed_ns();
        drop(scope);
        crate::hitch::record_pso("MS", &p.mesh.function_name, elapsed_ns);
        if out.is_null() {
            return S_FALSE;
        }
        return finish_pso(PipelineKind::Mesh(Arc::new(p)), g.root_signature, riid, out);
    }
    if g.amplification.is_some() {
        return failed!(E_INVALIDARG, "{what}: amplification shader without a mesh shader");
    }
    let Some(vs) = g.vs.and_then(|b| b.bytes()) else { return failed!(E_INVALIDARG, "{what}: no vertex shader") };
    let desc = d3d12_metal::metal::pipeline::GraphicsDesc {
        vs,
        ps: g.ps.and_then(|b| b.bytes()),
        blend: g.blend,
        sample_mask: g.sample_mask,
        raster: g.raster,
        depth_stencil: g.depth_stencil,
        input_layout: input_layout(g.input_layout),
        strip_cut: g.strip_cut,
        topology_type: g.topology_type,
        rtv_formats: g.rtv_formats.clone(),
        dsv_format: g.dsv_format,
        sample_count: g.sample_count.max(1),
    };
    let hs_bytes = g.hs.and_then(|b| b.bytes());
    let ds_bytes = g.ds.and_then(|b| b.bytes());
    let gs_bytes = g.gs.and_then(|b| b.bytes());
    if hs_bytes.is_some() != ds_bytes.is_some() {
        return failed!(E_INVALIDARG, "{what}: hull and domain shaders must both be set or both omitted");
    }
    let scope = crate::hitch::Scope::enter(crate::hitch::ActKind::PsoCompile, vs.len() as u64);
    let (hitch_tag, p) = match (hs_bytes, ds_bytes, gs_bytes) {
        (Some(hs), Some(ds), gs) => match d3d12_metal::metal::pipeline::GraphicsPipeline::new_with_tessellation(
            rs.rs.clone(),
            &desc,
            hs,
            ds,
            gs,
            "",
        ) {
            Ok(p) => (if gs.is_some() { "VS+HS+DS+GS+PS" } else { "VS+HS+DS+PS" }, p),
            Err(e) => return failed!(
                E_INVALIDARG,
                "{what}(VS {} bytes, HS {} bytes, DS {} bytes, GS {} bytes, PS {} bytes, RTs {:?}, DSV {}): {e}",
                vs.len(),
                hs.len(),
                ds.len(),
                gs.map_or(0, |b| b.len()),
                desc.ps.map_or(0, |b| b.len()),
                desc.rtv_formats,
                desc.dsv_format
            ),
        },
        (None, None, Some(gs)) => match d3d12_metal::metal::pipeline::GraphicsPipeline::new_with_geometry(rs.rs.clone(), &desc, gs, "") {
            Ok(p) => ("VS+GS+PS", p),
            Err(e) => return failed!(E_INVALIDARG, "{what}(VS {} bytes, GS {} bytes, PS {} bytes, RTs {:?}, DSV {}): {e}", vs.len(), gs.len(), desc.ps.map_or(0, |b| b.len()), desc.rtv_formats, desc.dsv_format),
        },
        (None, None, None) => match d3d12_metal::metal::pipeline::GraphicsPipeline::new(rs.rs.clone(), &desc, "") {
            Ok(p) => ("VS+PS", p),
            Err(e) => return failed!(E_INVALIDARG, "{what}(VS {} bytes, PS {} bytes, RTs {:?}, DSV {}): {e}", vs.len(), desc.ps.map_or(0, |b| b.len()), desc.rtv_formats, desc.dsv_format),
        },
        _ => unreachable!(),
    };
    let elapsed_ns = scope.elapsed_ns();
    drop(scope);
    crate::hitch::record_pso(hitch_tag, &p.vs.function_name, elapsed_ns);
    log::debug!(
        target: "d3d",
        "{what}: {} / {} RTs {:?} DSV {} topology type {} in {:?}",
        p.vs.function_name,
        p.ps.as_ref().map_or("-", |s| s.function_name.as_str()),
        desc.rtv_formats,
        desc.dsv_format,
        desc.topology_type,
        std::time::Duration::from_nanos(elapsed_ns)
    );
    if g.flags != 0 {
        log::debug!(target: "d3d", "{what}: flags {:#x} ignored", g.flags);
    }
    if out.is_null() {
        return S_FALSE;
    }
    finish_pso(PipelineKind::Graphics(Arc::new(p)), g.root_signature, riid, out)
}

pub unsafe fn create_graphics_pipeline(desc: *const u8, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    if desc.is_null() {
        return failed!(E_INVALIDARG, "CreateGraphicsPipelineState: null desc");
    }
    create_graphics(parse_desc(desc), "CreateGraphicsPipelineState", riid, out)
}

/// `D3D12_PIPELINE_STATE_STREAM_DESC`.
#[repr(C)]
pub struct StreamDesc {
    pub size: usize,
    pub stream: *const u8,
}

pub unsafe fn create_pipeline_state(desc: *const StreamDesc, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    if desc.is_null() || (*desc).stream.is_null() {
        return failed!(E_INVALIDARG, "CreatePipelineState: null stream");
    }
    match parse_stream((*desc).stream, (*desc).size) {
        Ok(g) => create_graphics(g, "CreatePipelineState", riid, out),
        Err(e) => failed!(E_INVALIDARG, "CreatePipelineState: {e}"),
    }
}

// ---------------------------------------------------------------------------
// Raytracing state objects (ID3D12StateObject + ID3D12StateObjectProperties1)
// ---------------------------------------------------------------------------

pub struct StateObject {
    pub so: Arc<d3d12_metal::rt::pipeline::StateObject>,
}

static SO_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for StateObject {
    fn class() -> &'static Class {
        class_once(&SO_CLASS, || Class {
            name: "ID3D12StateObject",
            vtbl: build_named::<StateObject>("ID3D12StateObject", ID3D12StateObject, &[]),
            iids: &[IID_ID3D12Object, IID_ID3D12DeviceChild, IID_ID3D12Pageable, IID_ID3D12StateObject],
        })
    }

    unsafe fn query_extra(this: *mut Obj<Self>, iid: &Guid) -> Option<*mut c_void> {
        if *iid == IID_ID3D12StateObjectProperties || *iid == IID_ID3D12StateObjectProperties1 {
            return Some(Obj::new(StateObjectProperties { so: Obj::get(this).so.clone() }) as *mut c_void);
        }
        None
    }
}

pub struct StateObjectProperties {
    pub so: Arc<d3d12_metal::rt::pipeline::StateObject>,
}

static SOP_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for StateObjectProperties {
    const TRACE_RELEASE: bool = false;

    fn class() -> &'static Class {
        class_once(&SOP_CLASS, || Class {
            name: "ID3D12StateObjectProperties1",
            vtbl: build_named::<StateObjectProperties>(
                "ID3D12StateObjectProperties1",
                ID3D12StateObjectProperties1,
                &[
                    m!("GetShaderIdentifier" => so_get_shader_identifier),
                    m!("GetShaderStackSize" => so_get_shader_stack_size),
                    m!("GetPipelineStackSize" => so_get_pipeline_stack_size),
                    m!("SetPipelineStackSize" => so_set_pipeline_stack_size),
                    m!("GetProgramIdentifier" => so_get_program_identifier),
                ],
            ),
            iids: &[IID_ID3D12StateObjectProperties, IID_ID3D12StateObjectProperties1],
        })
    }

    unsafe fn query_extra(this: *mut Obj<Self>, iid: &Guid) -> Option<*mut c_void> {
        if matches!(*iid, IID_ID3D12Object | IID_ID3D12DeviceChild | IID_ID3D12Pageable | IID_ID3D12StateObject) {
            return Some(Obj::new(StateObject { so: Obj::get(this).so.clone() }) as *mut c_void);
        }
        None
    }
}

pub unsafe fn state_object_from_ptr<'a>(p: *mut c_void) -> Option<&'a Arc<d3d12_metal::rt::pipeline::StateObject>> {
    if let Some(o) = Obj::<StateObject>::from_iface(p) {
        return Some(&Obj::get(o).so);
    }
    if let Some(o) = Obj::<StateObjectProperties>::from_iface(p) {
        return Some(&Obj::get(o).so);
    }
    None
}

unsafe extern "win64" fn so_get_shader_identifier(this: *mut Obj<StateObjectProperties>, export_name: *const u16) -> *const c_void {
    let name = read_wstr(export_name);
    match Obj::get(this).so.shader_identifier(&name) {
        Some(id) => id as *const d3d12_metal::rt::pipeline::ShaderIdentifier as *const c_void,
        None => {
            log::warn!(target: "d3d", "ID3D12StateObjectProperties::GetShaderIdentifier({name:?}) -> NULL (unknown export)");
            std::ptr::null()
        }
    }
}

unsafe extern "win64" fn so_get_shader_stack_size(_this: *mut Obj<StateObjectProperties>, export_name: *const u16) -> u64 {
    let _name = read_wstr(export_name);
    2048
}

unsafe extern "win64" fn so_get_pipeline_stack_size(this: *mut Obj<StateObjectProperties>) -> u64 {
    Obj::get(this).so.stack_size.load(std::sync::atomic::Ordering::Relaxed)
}

unsafe extern "win64" fn so_set_pipeline_stack_size(this: *mut Obj<StateObjectProperties>, size: u64) {
    Obj::get(this).so.stack_size.store(size, std::sync::atomic::Ordering::Relaxed);
}

unsafe extern "win64" fn so_get_program_identifier(
    _this: *mut Obj<StateObjectProperties>,
    ret: *mut [u64; 4],
    program_name: *const u16,
) -> *mut [u64; 4] {
    let name = read_wstr(program_name);
    log::debug!(target: "d3d", "ID3D12StateObjectProperties1::GetProgramIdentifier({name:?})");
    if !ret.is_null() {
        *ret = [0; 4];
    }
    ret
}

pub unsafe fn create_state_object(
    desc: *const d3d12_metal::rt::pipeline::RawStateObjectDesc,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    if desc.is_null() {
        return failed!(E_INVALIDARG, "CreateStateObject: null desc");
    }
    let (ty, num_subobjects) = ((*desc).Type, (*desc).NumSubobjects);
    let t = std::time::Instant::now();
    let parsed = match d3d12_metal::rt::pipeline::StateObjectDesc::from_raw(
        &*desc,
        None,
        |p| root_signature_from_ptr(p).map(|r| r.rs.clone()),
        |p| state_object_from_ptr(p).map(|s| s.clone()),
    ) {
        Ok(d) => d,
        Err(e) => return failed!(E_INVALIDARG, "CreateStateObject: {e}"),
    };
    if out.is_null() {
        return S_FALSE;
    }
    let so = match d3d12_metal::rt::pipeline::StateObject::new(parsed) {
        Ok(s) => s,
        Err(e) => return failed!(E_INVALIDARG, "CreateStateObject: {e}"),
    };
    let (num_exports, num_hit_groups) = (so.desc.exports.len() as u32, so.desc.hit_groups.len() as u32);
    log::debug!(
        target: "d3d",
        "CreateStateObject(type {}, {} exports, {} hit groups, {} identifiers) in {:?}",
        so.desc.ty,
        num_exports,
        num_hit_groups,
        so.identifiers.len(),
        t.elapsed()
    );
    let o = Obj::new(StateObject { so });
    let hr = query_out(o as *mut c_void, riid, out);
    Obj::release(o);
    if tr::on() {
        tr::record(&tr::Call::CreateStateObject {
            device: tr::device(),
            state_object: tr::created(hr, out).unwrap_or(0),
            state_object_type: ty,
            num_subobjects,
            num_exports,
            num_hit_groups,
        });
    }
    hr
}

pub unsafe fn add_to_state_object(
    addition: *const d3d12_metal::rt::pipeline::RawStateObjectDesc,
    base: *mut c_void,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    if addition.is_null() {
        return failed!(E_INVALIDARG, "AddToStateObject: null addition desc");
    }
    let Some(base_so) = state_object_from_ptr(base) else {
        return failed!(E_INVALIDARG, "AddToStateObject: foreign/null base state object {base:?}");
    };
    let (ty, num_subobjects) = ((*addition).Type, (*addition).NumSubobjects);
    let t = std::time::Instant::now();
    let parsed = match d3d12_metal::rt::pipeline::StateObjectDesc::from_raw(
        &*addition,
        Some(&base_so.desc),
        |p| root_signature_from_ptr(p).map(|r| r.rs.clone()),
        |p| state_object_from_ptr(p).map(|s| s.clone()),
    ) {
        Ok(d) => d,
        Err(e) => return failed!(E_INVALIDARG, "AddToStateObject: {e}"),
    };
    if out.is_null() {
        return S_FALSE;
    }
    let so = match d3d12_metal::rt::pipeline::StateObject::new(parsed) {
        Ok(s) => s,
        Err(e) => return failed!(E_INVALIDARG, "AddToStateObject: {e}"),
    };
    let (num_exports, num_hit_groups) = (so.desc.exports.len() as u32, so.desc.hit_groups.len() as u32);
    log::debug!(
        target: "d3d",
        "AddToStateObject(type {}, {} exports, {} hit groups, {} identifiers) in {:?}",
        so.desc.ty,
        num_exports,
        num_hit_groups,
        so.identifiers.len(),
        t.elapsed()
    );
    let o = Obj::new(StateObject { so });
    let hr = query_out(o as *mut c_void, riid, out);
    Obj::release(o);
    if tr::on() {
        tr::record(&tr::Call::AddToStateObject {
            device: tr::device(),
            state_object: tr::created(hr, out).unwrap_or(0),
            base: base as u64,
            state_object_type: ty,
            num_subobjects,
            num_exports,
            num_hit_groups,
        });
    }
    hr
}

