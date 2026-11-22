//! `ID3D11DeviceContext` (115 base slots) / `ID3D11DeviceContext1` (134 total slots):
//! immediate Metal 4 execution and deferred command-list recording/replay.

use std::ffi::c_void;
use std::sync::{Arc, Mutex, OnceLock};

use d3d12_metal::layout::PlacedFootprint;
use d3d12_metal::metal::clear::ClearValue;
use d3d12_metal::metal::cmdlist::{Barrier, CommandAllocator, CommandList, CopyBox, CopyLoc, TargetView};
use d3d12_metal::metal::descriptor::{bump_desc_heap_gen, heap_kind, DescriptorHeap, RESOURCE_DESCRIPTOR_SIZE};
use d3d12_metal::metal::indirect::{IndirectLayout, Work};
use d3d12_metal::metal::pipeline::{GraphicsDesc, GraphicsPipeline, MAX_VERTEX_SLOTS};
use d3d12_metal::metal::queue::{ListType, Queue};
use d3d12_metal::metal::resource::Resource as MtlResource;
use d3d12_metal::metal::views;
use d3d12_metal::types::{dim, heap_type, HeapProperties, Rect, ResourceDesc};

use crate::types::*;

use super::com::*;
use super::device::DeviceCore;
use super::resources::*;
use super::state::*;
use super::types::*;

/// Exact Windows x64 `ID3D11DeviceContext` base vtable length (slots 0..114).
pub const ID3D11_CONTEXT_BASE_SLOTS: usize = 115;
/// Exact Windows x64 `ID3D11DeviceContext1` total vtable length (slots 0..133).
pub const ID3D11_CONTEXT1_TOTAL_SLOTS: usize = 134;

// ---------------------------------------------------------------------------
// RAII COM pointer holder for ContextState
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct ComRef(pub usize);

impl ComRef {
    pub unsafe fn from_borrowed(p: *mut c_void) -> Self {
        if !p.is_null() {
            com_add_ref(p);
        }
        Self(p as usize)
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.0 as *mut c_void
    }

    pub unsafe fn copy_out(&self, out: *mut *mut c_void) {
        if !out.is_null() {
            let p = self.as_ptr();
            if !p.is_null() {
                com_add_ref(p);
            }
            std::ptr::write_unaligned(out, p);
        }
    }
}

impl Clone for ComRef {
    fn clone(&self) -> Self {
        if self.0 != 0 {
            unsafe { com_add_ref(self.0 as *mut c_void) };
        }
        Self(self.0)
    }
}

impl Drop for ComRef {
    fn drop(&mut self) {
        if self.0 != 0 {
            unsafe { com_release(self.0 as *mut c_void) };
        }
    }
}

// ---------------------------------------------------------------------------
// Viewport bridge helper (no `objc2_metal` import needed)
// ---------------------------------------------------------------------------

fn push_mtl_viewport<V>(out: &mut Vec<V>, vp: &D3D11_VIEWPORT) {
    #[repr(C)]
    struct RawMtlViewport {
        origin_x: f64,
        origin_y: f64,
        width: f64,
        height: f64,
        znear: f64,
        zfar: f64,
    }
    debug_assert_eq!(std::mem::size_of::<V>(), std::mem::size_of::<RawMtlViewport>());
    debug_assert_eq!(std::mem::align_of::<V>(), std::mem::align_of::<RawMtlViewport>());
    let raw = RawMtlViewport {
        origin_x: vp.TopLeftX as f64,
        origin_y: vp.TopLeftY as f64,
        width: vp.Width as f64,
        height: vp.Height as f64,
        znear: vp.MinDepth as f64,
        zfar: vp.MaxDepth as f64,
    };
    // SAFETY: `MTLViewport` is `#[repr(C)]` with 6 `f64` fields in identical order.
    out.push(unsafe { std::mem::transmute_copy(&raw) });
}

fn d3d11_topology_to_type(topo: u32) -> u32 {
    use d3d12_metal::topology::{d3d, topology_type};
    match topo {
        d3d::POINTLIST => topology_type::POINT,
        d3d::LINELIST | d3d::LINESTRIP | d3d::LINELIST_ADJ | d3d::LINESTRIP_ADJ => topology_type::LINE,
        d3d::PATCHLIST_1..=d3d::PATCHLIST_32 => topology_type::PATCH,
        _ => topology_type::TRIANGLE,
    }
}

// ---------------------------------------------------------------------------
// Immediate Metal 4 execution backend (owned by DeviceCore)
// ---------------------------------------------------------------------------

const ALLOC_RING: usize = 4;
const RES_HEAP_CAP: u32 = 32768;
const SAMP_HEAP_CAP: u32 = 2048;

pub struct ImmediateExec {
    queue: Arc<Queue>,
    allocs: Vec<CommandAllocator>,
    ring_vals: [u64; ALLOC_RING],
    ring_idx: usize,
    pub(super) cl: CommandList,
    open: bool,
    pub(super) has_work: bool,
    res_heaps: Vec<DescriptorHeap>,
    res_heap_idx: usize,
    res_heap_vals: [u64; 2],
    res_cursor: u32,
    samp_heaps: Vec<DescriptorHeap>,
    samp_heap_idx: usize,
    samp_heap_vals: [u64; 2],
    samp_cursor: u32,
    keep_alive: Vec<Box<dyn Send>>,
    pending_queries: Vec<Arc<QueryInner>>,
}
unsafe impl Send for ImmediateExec {}

impl ImmediateExec {
    pub fn new(queue: Arc<Queue>) -> Result<Self, String> {
        let mut allocs = Vec::with_capacity(ALLOC_RING);
        for _ in 0..ALLOC_RING {
            allocs.push(CommandAllocator::new(ListType::Direct).ok_or("CommandAllocator::new failed")?);
        }
        let cl = CommandList::new(ListType::Direct).ok_or("CommandList::new failed")?;
        let res_heaps = vec![
            DescriptorHeap::new(heap_kind::CBV_SRV_UAV, RES_HEAP_CAP, true, "d3d11 res heap 0")?,
            DescriptorHeap::new(heap_kind::CBV_SRV_UAV, RES_HEAP_CAP, true, "d3d11 res heap 1")?,
        ];
        let samp_heaps = vec![
            DescriptorHeap::new(heap_kind::SAMPLER, SAMP_HEAP_CAP, true, "d3d11 samp heap 0")?,
            DescriptorHeap::new(heap_kind::SAMPLER, SAMP_HEAP_CAP, true, "d3d11 samp heap 1")?,
        ];
        Ok(Self {
            queue,
            allocs,
            ring_vals: [0; ALLOC_RING],
            ring_idx: 0,
            cl,
            open: false,
            has_work: false,
            res_heaps,
            res_heap_idx: 0,
            res_heap_vals: [0; 2],
            res_cursor: 0,
            samp_heaps,
            samp_heap_idx: 0,
            samp_heap_vals: [0; 2],
            samp_cursor: 0,
            keep_alive: Vec::new(),
            pending_queries: Vec::new(),
        })
    }

    pub(super) fn ensure_open(&mut self) -> Result<(), String> {
        if self.open {
            return Ok(());
        }
        let i = self.ring_idx;
        if self.ring_vals[i] != 0 {
            self.queue.timeline.wait_cpu(self.ring_vals[i], u64::MAX);
        }
        self.allocs[i].reset();
        self.cl.begin(&self.allocs[i])?;
        self.open = true;
        Ok(())
    }

    pub fn keep<T: Send + 'static>(&mut self, obj: T) {
        self.keep_alive.push(Box::new(obj));
    }

    pub fn flush(&mut self) -> u64 {
        if !self.open {
            return self.queue.last_submitted();
        }
        if !self.has_work {
            if let Err(e) = self.cl.close() {
                log::warn!(target: "d3d", "d3d11 empty close: {e}");
            }
            self.open = false;
            return self.queue.last_submitted();
        }
        if let Err(e) = self.cl.close() {
            log::warn!(target: "d3d", "d3d11 flush close: {e}");
            self.open = false;
            self.has_work = false;
            return self.queue.last_submitted();
        }
        let v = self.queue.submit(&[&self.cl.cb]);
        let i = self.ring_idx;
        self.ring_vals[i] = v;
        self.ring_idx = (i + 1) % self.allocs.len();
        self.open = false;
        self.has_work = false;
        let kept = std::mem::take(&mut self.keep_alive);
        if !kept.is_empty() {
            self.queue.defer_release(Box::new(kept));
        }
        for q in std::mem::take(&mut self.pending_queries) {
            let mut rt = q.rt.lock().unwrap();
            rt.submitted = true;
            rt.timeline_value = v;
        }
        v
    }

    fn alloc_res_slots(&mut self, count: u32) -> Result<(usize, u64), String> {
        if self.res_cursor + count > RES_HEAP_CAP {
            let v = self.flush();
            self.res_heap_vals[self.res_heap_idx] = v;
            self.res_heap_idx = (self.res_heap_idx + 1) % self.res_heaps.len();
            if self.res_heap_vals[self.res_heap_idx] != 0 {
                self.queue.timeline.wait_cpu(self.res_heap_vals[self.res_heap_idx], u64::MAX);
            }
            self.res_cursor = 0;
            self.ensure_open()?;
        }
        let h = &self.res_heaps[self.res_heap_idx];
        let off = self.res_cursor as usize * RESOURCE_DESCRIPTOR_SIZE as usize;
        self.res_cursor += count;
        Ok((h.cpu_base + off, h.gpu_base + off as u64))
    }

    fn alloc_samp_slots(&mut self, count: u32) -> Result<(usize, u64), String> {
        if self.samp_cursor + count > SAMP_HEAP_CAP {
            let v = self.flush();
            self.samp_heap_vals[self.samp_heap_idx] = v;
            self.samp_heap_idx = (self.samp_heap_idx + 1) % self.samp_heaps.len();
            if self.samp_heap_vals[self.samp_heap_idx] != 0 {
                self.queue.timeline.wait_cpu(self.samp_heap_vals[self.samp_heap_idx], u64::MAX);
            }
            self.samp_cursor = 0;
            self.ensure_open()?;
        }
        let h = &self.samp_heaps[self.samp_heap_idx];
        let off = self.samp_cursor as usize * RESOURCE_DESCRIPTOR_SIZE as usize;
        self.samp_cursor += count;
        Ok((h.cpu_base + off, h.gpu_base + off as u64))
    }

    fn bind_active_heaps(&mut self) {
        let r_gpu = self.res_heaps[self.res_heap_idx].gpu_base;
        let s_gpu = self.samp_heaps[self.samp_heap_idx].gpu_base;
        self.cl.set_descriptor_heaps(r_gpu, s_gpu);
    }
}

// ---------------------------------------------------------------------------
// Pipeline state tracked per context (immediate or deferred)
// ---------------------------------------------------------------------------

#[derive(Clone, Default)]
pub struct CbSlot {
    pub com: ComRef,
    pub res: Option<Arc<ResourceCore>>,
    pub first_constant: u32,
    pub num_constants: u32,
}

#[derive(Clone, Default)]
pub struct SrvSlot {
    pub com: ComRef,
    pub srv: Option<Arc<SrvInner>>,
}

#[derive(Clone, Default)]
pub struct UavSlot {
    pub com: ComRef,
    pub uav: Option<Arc<UavInner>>,
}

#[derive(Clone, Default)]
pub struct SampSlot {
    pub com: ComRef,
    pub samp: Option<Arc<SamplerStateInner>>,
}

#[derive(Clone, Default)]
pub struct VbSlot {
    pub com: ComRef,
    pub res: Option<Arc<ResourceCore>>,
    pub stride: u32,
    pub offset: u32,
}

#[derive(Clone, Default)]
pub struct RtvSlot {
    pub com: ComRef,
    pub rtv: Option<Arc<RtvInner>>,
}

#[derive(Clone)]
pub struct StageBindings {
    pub cbs: Vec<CbSlot>,
    pub srvs: Vec<SrvSlot>,
    pub samplers: Vec<SampSlot>,
}

impl Default for StageBindings {
    fn default() -> Self {
        Self {
            cbs: vec![CbSlot::default(); D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT],
            srvs: vec![SrvSlot::default(); D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT],
            samplers: vec![SampSlot::default(); D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT],
        }
    }
}

#[derive(Clone)]
pub struct ContextState {
    pub vs_com: ComRef,
    pub vs: Option<Arc<ShaderBlob>>,
    pub gs_com: ComRef,
    pub gs: Option<Arc<ShaderBlob>>,
    pub ps_com: ComRef,
    pub ps: Option<Arc<ShaderBlob>>,
    pub cs_com: ComRef,
    pub cs: Option<Arc<ComputeShaderInner>>,

    pub vs_bind: StageBindings,
    pub ps_bind: StageBindings,
    pub gs_bind: StageBindings,
    pub hs_bind: StageBindings,
    pub ds_bind: StageBindings,
    pub cs_bind: StageBindings,
    pub cs_uavs: Vec<UavSlot>,
    pub om_uavs: Vec<UavSlot>,

    pub input_layout_com: ComRef,
    pub input_layout: Option<Arc<InputLayoutInner>>,
    pub vbs: Vec<VbSlot>,
    pub ib_com: ComRef,
    pub ib: Option<Arc<ResourceCore>>,
    pub ib_format: u32,
    pub ib_offset: u32,
    pub topology: u32,

    pub rtvs: Vec<RtvSlot>,
    pub dsv_com: ComRef,
    pub dsv: Option<Arc<DsvInner>>,

    pub blend_com: ComRef,
    pub blend: Option<Arc<BlendStateInner>>,
    pub blend_factor: [f32; 4],
    pub sample_mask: u32,

    pub ds_state_com: ComRef,
    pub ds_state: Option<Arc<DepthStencilStateInner>>,
    pub stencil_ref: u32,

    pub rast_com: ComRef,
    pub rast: Option<Arc<RasterizerStateInner>>,
    pub viewports: Vec<D3D11_VIEWPORT>,
    pub scissors: Vec<Rect>,

    pub so_targets: Vec<ComRef>,
    pub predicate_com: ComRef,
    pub predicate_val: BOOL,
}

impl Default for ContextState {
    fn default() -> Self {
        Self {
            vs_com: ComRef::default(),
            vs: None,
            gs_com: ComRef::default(),
            gs: None,
            ps_com: ComRef::default(),
            ps: None,
            cs_com: ComRef::default(),
            cs: None,
            vs_bind: StageBindings::default(),
            ps_bind: StageBindings::default(),
            gs_bind: StageBindings::default(),
            hs_bind: StageBindings::default(),
            ds_bind: StageBindings::default(),
            cs_bind: StageBindings::default(),
            cs_uavs: vec![UavSlot::default(); D3D11_PS_CS_UAV_REGISTER_COUNT],
            om_uavs: vec![UavSlot::default(); D3D11_PS_CS_UAV_REGISTER_COUNT],
            input_layout_com: ComRef::default(),
            input_layout: None,
            vbs: vec![VbSlot::default(); D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT],
            ib_com: ComRef::default(),
            ib: None,
            ib_format: 0,
            ib_offset: 0,
            topology: topology::UNDEFINED,
            rtvs: vec![RtvSlot::default(); D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT],
            dsv_com: ComRef::default(),
            dsv: None,
            blend_com: ComRef::default(),
            blend: None,
            blend_factor: [1.0; 4],
            sample_mask: u32::MAX,
            ds_state_com: ComRef::default(),
            ds_state: None,
            stencil_ref: 0,
            rast_com: ComRef::default(),
            rast: None,
            viewports: Vec::new(),
            scissors: Vec::new(),
            so_targets: vec![ComRef::default(); D3D11_SO_BUFFER_SLOT_COUNT],
            predicate_com: ComRef::default(),
            predicate_val: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// Recorded commands for Deferred Contexts / ID3D11CommandList
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub enum RecordedCmd {
    ClearRtv { rtv: Arc<RtvInner>, color: [f32; 4], rects: Vec<Rect> },
    ClearDsv { dsv: Arc<DsvInner>, flags: u32, depth: f32, stencil: u8, rects: Vec<Rect> },
    ClearUavUint { uav: Arc<UavInner>, values: [u32; 4] },
    ClearUavFloat { uav: Arc<UavInner>, values: [f32; 4] },
    CopyResource { dst: Arc<ResourceCore>, src: Arc<ResourceCore> },
    CopySubresourceRegion {
        dst: Arc<ResourceCore>,
        dst_sub: u32,
        dst_xyz: [u32; 3],
        src: Arc<ResourceCore>,
        src_sub: u32,
        src_box: Option<D3D11_BOX>,
    },
    UpdateSubresource {
        dst: Arc<ResourceCore>,
        dst_sub: u32,
        dst_box: Option<D3D11_BOX>,
        data: Vec<u8>,
        src_row_pitch: u32,
        src_depth_pitch: u32,
    },
    ResolveSubresource {
        dst: Arc<ResourceCore>,
        dst_sub: u32,
        src: Arc<ResourceCore>,
        src_sub: u32,
        format: u32,
    },
    Draw { state: Box<ContextState>, vertex_count: u32, instance_count: u32, start_vertex: u32, start_instance: u32 },
    DrawIndexed {
        state: Box<ContextState>,
        index_count: u32,
        instance_count: u32,
        start_index: u32,
        base_vertex: i32,
        start_instance: u32,
    },
    DrawInstancedIndirect { state: Box<ContextState>, args: Arc<ResourceCore>, offset: u32 },
    DrawIndexedInstancedIndirect { state: Box<ContextState>, args: Arc<ResourceCore>, offset: u32 },
    Dispatch { state: Box<ContextState>, x: u32, y: u32, z: u32 },
    DispatchIndirect { state: Box<ContextState>, args: Arc<ResourceCore>, offset: u32 },
    CopyStructureCount { dst: Arc<ResourceCore>, dst_offset: u32, uav: Arc<UavInner> },
    SetUavCounter { uav: Arc<UavInner>, value: u32 },
    GenerateMips { srv: Arc<SrvInner> },
    BeginQuery(Arc<QueryInner>),
    EndQuery(Arc<QueryInner>),
    DiscardResource(Arc<ResourceCore>),
}

// ---------------------------------------------------------------------------
// Immediate execution helpers
// ---------------------------------------------------------------------------

unsafe fn write_desc_entry(cpu_base: usize, idx: usize, entry: views::Entry) {
    let dst = (cpu_base + idx * RESOURCE_DESCRIPTOR_SIZE as usize) as *mut u8;
    std::ptr::copy_nonoverlapping(entry.as_ptr(), dst, entry.len());
}

unsafe fn populate_stage_table(
    exec: &mut ImmediateExec,
    cpu_base: usize,
    bind: &StageBindings,
    uavs: Option<&[UavSlot]>,
) {
    for (i, cb) in bind.cbs.iter().enumerate() {
        let entry = if let Some(res) = &cb.res {
            let mtl = res.current_mtl();
            let off = (cb.first_constant as u64) * 16;
            let total = mtl.desc.Width;
            let len = if cb.num_constants == 0 {
                total.saturating_sub(off)
            } else {
                ((cb.num_constants as u64) * 16).min(total.saturating_sub(off))
            };
            let e = views::cbv_entry(mtl.gpu_va + off, len.min(u32::MAX as u64) as u32);
            exec.keep(mtl);
            e
        } else {
            [0u8; 24]
        };
        write_desc_entry(cpu_base, i, entry);
    }
    let srv_off = RS_CBV_COUNT as usize;
    for (i, slot) in bind.srvs.iter().enumerate() {
        let entry = if let Some(srv) = &slot.srv {
            match srv.entry() {
                Ok((mtl, e)) => {
                    exec.keep(mtl);
                    e
                }
                Err(e) => {
                    log::warn!(target: "d3d", "d3d11 SRV slot {i}: {e}");
                    [0u8; 24]
                }
            }
        } else {
            [0u8; 24]
        };
        write_desc_entry(cpu_base, srv_off + i, entry);
    }
    if let Some(uavs) = uavs {
        let uav_off = (RS_CBV_COUNT + RS_SRV_COUNT) as usize;
        for (i, slot) in uavs.iter().enumerate() {
            let entry = if let Some(uav) = &slot.uav {
                match uav.entry() {
                    Ok((mtl, e)) => {
                        exec.keep(mtl);
                        if let Some(counter) = &uav.counter {
                            exec.keep(counter.clone());
                        }
                        e
                    }
                    Err(e) => {
                        log::warn!(target: "d3d", "d3d11 UAV slot {i}: {e}");
                        [0u8; 24]
                    }
                }
            } else {
                [0u8; 24]
            };
            write_desc_entry(cpu_base, uav_off + i, entry);
        }
    }
}

unsafe fn populate_sampler_table(cpu_base: usize, bind: &StageBindings) {
    for (i, slot) in bind.samplers.iter().enumerate() {
        let entry = slot.samp.as_ref().map_or([0u8; 24], |s| s.entry);
        write_desc_entry(cpu_base, i, entry);
    }
}

fn get_or_build_gfx_pso(
    device: &DeviceCore,
    st: &ContextState,
    indexed: bool,
) -> Result<Arc<GraphicsPipeline>, String> {
    let vs = st.vs.as_ref().ok_or("no vertex shader bound")?;
    let gs = st.gs.as_ref();
    let ps = st.ps.as_ref();

    let blend_d12 = st
        .blend
        .as_ref()
        .map(|b| b.d3d12)
        .unwrap_or_else(|| D3D11_BLEND_DESC::default_d3d11().to_d3d12());
    let (rast_mtl, rast_hash) = match &st.rast {
        Some(r) => (r.mtl, r.hash),
        None => {
            let d1 = D3D11_RASTERIZER_DESC::default_d3d11().to_desc1();
            (d3d12_metal::state::raster_state(&d1.to_d3d12()), hash_raster_desc(&d1))
        }
    };
    let (ds_d12, ds_mtl) = match &st.ds_state {
        Some(d) => (d.desc.to_d3d12(), d.mtl),
        None => {
            let d = D3D11_DEPTH_STENCIL_DESC::default_d3d11().to_d3d12();
            (d, d3d12_metal::state::depth_stencil_state(&d))
        }
    };

    let mut rtv_formats = [0u32; 8];
    let mut num_rtvs = 0u8;
    let mut sample_count = 1u32;
    for (i, slot) in st.rtvs.iter().enumerate() {
        if let Some(rtv) = &slot.rtv {
            rtv_formats[i] = rtv.format();
            num_rtvs = (i + 1) as u8;
            sample_count = rtv.resource.kind.sample_count().max(1);
        }
    }
    let dsv_format = match &st.dsv {
        Some(dsv) => {
            if num_rtvs == 0 {
                sample_count = dsv.resource.kind.sample_count().max(1);
            }
            dsv.format()
        }
        None => 0,
    };

    let topology_type = d3d11_topology_to_type(st.topology);
    let strip_cut = if indexed {
        match st.ib_format {
            57 => 1, // CUT_0XFFFF
            42 => 2, // CUT_0XFFFFFFFF
            _ => 0,
        }
    } else {
        0
    };

    let key = GraphicsPsoKey {
        vs_hash: vs.hash,
        gs_hash: gs.map_or(0, |g| g.hash),
        ps_hash: ps.map_or(0, |p| p.hash),
        il_hash: st.input_layout.as_ref().map_or(0, |il| il.hash),
        blend: blend_d12,
        sample_mask: st.sample_mask,
        raster_hash: rast_hash,
        depth_stencil: ds_d12,
        topology_type,
        strip_cut,
        rtv_formats,
        num_rtvs,
        dsv_format,
        sample_count,
    };

    if let Some(p) = device.gfx_psos.lock().unwrap().get(&key) {
        return Ok(p.clone());
    }

    let gdesc = GraphicsDesc {
        vs: &vs.bytecode,
        ps: ps.map(|p| p.bytecode.as_slice()),
        blend: blend_d12,
        sample_mask: st.sample_mask,
        raster: rast_mtl,
        depth_stencil: ds_mtl,
        input_layout: st.input_layout.as_ref().map_or_else(Vec::new, |il| il.elements.clone()),
        strip_cut,
        topology_type,
        rtv_formats: rtv_formats[..num_rtvs as usize].to_vec(),
        dsv_format,
        sample_count,
    };
    let label = format!("d3d11_gfx_{:016x}", vs.hash as u64);
    let pso = Arc::new(match gs {
        Some(g) => GraphicsPipeline::new_with_geometry(device.gfx_rs.clone(), &gdesc, &g.bytecode, &label)?,
        None => GraphicsPipeline::new(device.gfx_rs.clone(), &gdesc, &label)?,
    });
    device.gfx_psos.lock().unwrap().insert(key, pso.clone());
    Ok(pso)
}

fn sync_stage_shared(device: &DeviceCore, bind: &StageBindings) {
    for cb in &bind.cbs {
        if let Some(r) = &cb.res {
            r.sync_other_devices(device);
        }
    }
    for s in &bind.srvs {
        if let Some(srv) = &s.srv {
            srv.resource.sync_other_devices(device);
        }
    }
}

fn sync_state_shared(device: &DeviceCore, st: &ContextState) {
    sync_stage_shared(device, &st.vs_bind);
    sync_stage_shared(device, &st.gs_bind);
    sync_stage_shared(device, &st.ps_bind);
    sync_stage_shared(device, &st.cs_bind);
    for r in &st.rtvs {
        if let Some(rtv) = &r.rtv {
            rtv.resource.sync_other_devices(device);
        }
    }
    if let Some(dsv) = &st.dsv {
        dsv.resource.sync_other_devices(device);
    }
    for u in &st.om_uavs {
        if let Some(uav) = &u.uav {
            uav.resource.sync_other_devices(device);
        }
    }
    for u in &st.cs_uavs {
        if let Some(uav) = &u.uav {
            uav.resource.sync_other_devices(device);
        }
    }
    for vb in &st.vbs {
        if let Some(r) = &vb.res {
            r.sync_other_devices(device);
        }
    }
    if let Some(ib) = &st.ib {
        ib.sync_other_devices(device);
    }
}

fn sync_cmd_shared_resources(device: &DeviceCore, cmd: &RecordedCmd) {
    match cmd {
        RecordedCmd::ClearRtv { rtv, .. } => rtv.resource.sync_other_devices(device),
        RecordedCmd::ClearDsv { dsv, .. } => dsv.resource.sync_other_devices(device),
        RecordedCmd::ClearUavUint { uav, .. } | RecordedCmd::ClearUavFloat { uav, .. } => {
            uav.resource.sync_other_devices(device);
        }
        RecordedCmd::CopyResource { dst, src }
        | RecordedCmd::CopySubresourceRegion { dst, src, .. }
        | RecordedCmd::ResolveSubresource { dst, src, .. } => {
            dst.sync_other_devices(device);
            src.sync_other_devices(device);
        }
        RecordedCmd::UpdateSubresource { dst, .. } => dst.sync_other_devices(device),
        RecordedCmd::Draw { state, .. }
        | RecordedCmd::DrawIndexed { state, .. }
        | RecordedCmd::Dispatch { state, .. } => sync_state_shared(device, state),
        RecordedCmd::DrawInstancedIndirect { state, args, .. }
        | RecordedCmd::DrawIndexedInstancedIndirect { state, args, .. }
        | RecordedCmd::DispatchIndirect { state, args, .. } => {
            sync_state_shared(device, state);
            args.sync_other_devices(device);
        }
        RecordedCmd::CopyStructureCount { dst, uav, .. } => {
            dst.sync_other_devices(device);
            uav.resource.sync_other_devices(device);
        }
        RecordedCmd::SetUavCounter { uav, .. } => uav.resource.sync_other_devices(device),
        RecordedCmd::GenerateMips { srv } => srv.resource.sync_other_devices(device),
        RecordedCmd::DiscardResource(r) => r.sync_other_devices(device),
        RecordedCmd::BeginQuery(_) | RecordedCmd::EndQuery(_) => {}
    }
}

unsafe fn exec_recorded_cmd(device: &DeviceCore, cmd: &RecordedCmd) {
    sync_cmd_shared_resources(device, cmd);
    let mut exec = device.imm_exec.lock().unwrap();
    if let Err(e) = exec.ensure_open() {
        log::warn!(target: "d3d", "d3d11 ensure_open failed: {e}");
        return;
    }
    match cmd {
        RecordedCmd::ClearRtv { rtv, color, rects } => {
            match rtv.target_view() {
                Ok((mtl, tv)) => {
                    exec.keep(mtl);
                    if let Err(e) = exec.cl.clear_rtv(&tv, *color, rects) {
                        log::warn!(target: "d3d", "d3d11 ClearRenderTargetView: {e}");
                    } else {
                        exec.has_work = true;
                    }
                }
                Err(e) => log::warn!(target: "d3d", "d3d11 ClearRenderTargetView target_view: {e}"),
            }
        }
        RecordedCmd::ClearDsv { dsv, flags, depth, stencil, rects } => {
            match dsv.target_view() {
                Ok((mtl, tv)) => {
                    exec.keep(mtl);
                    if let Err(e) = exec.cl.clear_dsv(&tv, *flags & clear_flag::DEPTH != 0, *depth, *flags & clear_flag::STENCIL != 0, *stencil, rects) {
                        log::warn!(target: "d3d", "d3d11 ClearDepthStencilView: {e}");
                    } else {
                        exec.has_work = true;
                    }
                }
                Err(e) => log::warn!(target: "d3d", "d3d11 ClearDepthStencilView target_view: {e}"),
            }
        }
        RecordedCmd::ClearUavUint { uav, values } => {
            let mtl = uav.resource.current_mtl();
            exec.cl.barrier();
            if let Err(e) = exec.cl.clear_uav(&mtl, Some(&uav.d3d12_desc), ClearValue::Uint(*values), &[]) {
                log::warn!(target: "d3d", "d3d11 ClearUnorderedAccessViewUint: {e}");
            } else {
                exec.cl.barrier();
                exec.has_work = true;
            }
            exec.keep(mtl);
        }
        RecordedCmd::ClearUavFloat { uav, values } => {
            let mtl = uav.resource.current_mtl();
            exec.cl.barrier();
            if let Err(e) = exec.cl.clear_uav(&mtl, Some(&uav.d3d12_desc), ClearValue::Float(*values), &[]) {
                log::warn!(target: "d3d", "d3d11 ClearUnorderedAccessViewFloat: {e}");
            } else {
                exec.cl.barrier();
                exec.has_work = true;
            }
            exec.keep(mtl);
        }
        RecordedCmd::CopyResource { dst, src } => {
            let d = dst.current_mtl();
            let s = src.current_mtl();
            exec.cl.barrier();
            if let Err(e) = exec.cl.copy_resource(&d, &s) {
                log::warn!(target: "d3d", "d3d11 CopyResource: {e}");
            } else {
                exec.cl.barrier();
                exec.has_work = true;
            }
            exec.keep((d, s));
        }
        RecordedCmd::CopySubresourceRegion { dst, dst_sub, dst_xyz, src, src_sub, src_box } => {
            let d = dst.current_mtl();
            let s = src.current_mtl();
            exec.cl.barrier();
            let res = if dst.is_buffer() && src.is_buffer() {
                let (src_off, len) = match src_box {
                    Some(b) => (b.left as u64, b.right.saturating_sub(b.left) as u64),
                    None => (0, s.desc.Width),
                };
                let dst_off = dst_xyz[0] as u64;
                let max_len = len
                    .min(s.desc.Width.saturating_sub(src_off))
                    .min(d.desc.Width.saturating_sub(dst_off));
                exec.cl.copy_buffer(&d, dst_off, &s, src_off, max_len)
            } else {
                let cbox = src_box.map(|b| CopyBox {
                    left: b.left,
                    top: b.top,
                    front: b.front,
                    right: b.right,
                    bottom: b.bottom,
                    back: b.back,
                });
                exec.cl.copy_texture_region(
                    CopyLoc::Subresource(&d, *dst_sub),
                    *dst_xyz,
                    CopyLoc::Subresource(&s, *src_sub),
                    cbox,
                )
            };
            if let Err(e) = res {
                log::warn!(target: "d3d", "d3d11 CopySubresourceRegion: {e}");
            } else {
                exec.cl.barrier();
                exec.has_work = true;
            }
            exec.keep((d, s));
        }
        RecordedCmd::UpdateSubresource { dst, dst_sub, dst_box, data, src_row_pitch, src_depth_pitch } => {
            if let Err(e) = exec_update_subresource_bytes(&mut exec, dst, *dst_sub, *dst_box, data, *src_row_pitch, *src_depth_pitch) {
                log::warn!(target: "d3d", "d3d11 UpdateSubresource: {e}");
            }
        }
        RecordedCmd::ResolveSubresource { dst, dst_sub, src, src_sub, format } => {
            let d = dst.current_mtl();
            let s = src.current_mtl();
            exec.cl.barrier();
            if let Err(e) = exec.cl.resolve_subresource(&d, *dst_sub, &s, *src_sub, *format) {
                log::warn!(target: "d3d", "d3d11 ResolveSubresource: {e}");
            } else {
                exec.cl.barrier();
                exec.has_work = true;
            }
            exec.keep((d, s));
        }
        RecordedCmd::Draw { state, vertex_count, instance_count, start_vertex, start_instance } => {
            if let Err(e) = exec_draw(device, &mut exec, state, None, *vertex_count, *instance_count, *start_vertex, 0, *start_instance) {
                log::warn!(target: "d3d", "d3d11 Draw({vertex_count}, {instance_count}): {e}");
            }
        }
        RecordedCmd::DrawIndexed { state, index_count, instance_count, start_index, base_vertex, start_instance } => {
            if let Err(e) = exec_draw(device, &mut exec, state, Some(*start_index), *index_count, *instance_count, 0, *base_vertex, *start_instance) {
                log::warn!(target: "d3d", "d3d11 DrawIndexed({index_count}, {instance_count}): {e}");
            }
        }
        RecordedCmd::DrawInstancedIndirect { state, args, offset } => {
            if let Err(e) = exec_draw_indirect(device, &mut exec, state, args, *offset, false) {
                log::warn!(target: "d3d", "d3d11 DrawInstancedIndirect(off={offset}): {e}");
            }
        }
        RecordedCmd::DrawIndexedInstancedIndirect { state, args, offset } => {
            if let Err(e) = exec_draw_indirect(device, &mut exec, state, args, *offset, true) {
                log::warn!(target: "d3d", "d3d11 DrawIndexedInstancedIndirect(off={offset}): {e}");
            }
        }
        RecordedCmd::Dispatch { state, x, y, z } => {
            if let Err(e) = exec_dispatch(device, &mut exec, state, *x, *y, *z) {
                log::warn!(target: "d3d", "d3d11 Dispatch({x}, {y}, {z}): {e}");
            }
        }
        RecordedCmd::DispatchIndirect { state, args, offset } => {
            if let Err(e) = exec_dispatch_indirect(device, &mut exec, state, args, *offset) {
                log::warn!(target: "d3d", "d3d11 DispatchIndirect(off={offset}): {e}");
            }
        }
        RecordedCmd::CopyStructureCount { dst, dst_offset, uav } => {
            if let Some(counter) = &uav.counter {
                let d = dst.current_mtl();
                let off = *dst_offset as u64;
                if off & 3 == 0 && off.saturating_add(4) <= d.desc.Width {
                    exec.cl.barrier();
                    if let Err(e) = exec.cl.copy_buffer(&d, off, counter, 0, 4) {
                        log::warn!(target: "d3d", "d3d11 CopyStructureCount: {e}");
                    } else {
                        exec.cl.barrier();
                        exec.has_work = true;
                    }
                    exec.keep((d, counter.clone()));
                } else {
                    log::warn!(
                        target: "d3d",
                        "d3d11 CopyStructureCount: misaligned or out-of-bounds offset {off} for {}B dst buffer",
                        d.desc.Width
                    );
                }
            } else {
                log::warn!(target: "d3d", "d3d11 CopyStructureCount: source UAV has no APPEND/COUNTER flag");
            }
        }
        RecordedCmd::SetUavCounter { uav, value } => {
            if let Some(counter) = &uav.counter {
                exec.cl.barrier();
                if let Err(e) = exec.cl.write_buffer_u32(counter, 0, *value) {
                    log::warn!(target: "d3d", "d3d11 SetUavCounter({value}): {e}");
                } else {
                    exec.cl.barrier();
                    exec.has_work = true;
                }
                exec.keep(counter.clone());
            }
        }
        RecordedCmd::GenerateMips { srv } => {
            if let Err(e) = exec_generate_mips(&mut exec, srv) {
                log::warn!(target: "d3d", "d3d11 GenerateMips: {e}");
            }
        }
        RecordedCmd::BeginQuery(q) => {
            let mut rt = q.rt.lock().unwrap();
            rt.ended = false;
            rt.submitted = false;
            if matches!(q.desc.Query, query_kind::OCCLUSION | query_kind::OCCLUSION_PREDICATE) {
                if let Some(heap) = rt.heap.clone() {
                    let binary = q.desc.Query == query_kind::OCCLUSION_PREDICATE;
                    if let Err(e) = exec.cl.begin_occlusion(&heap, 0, binary) {
                        log::warn!(target: "d3d", "d3d11 BeginQuery(occlusion): {e}");
                    } else {
                        exec.keep(heap);
                        exec.has_work = true;
                    }
                }
            }
        }
        RecordedCmd::EndQuery(q) => {
            let mut rt = q.rt.lock().unwrap();
            rt.ended = true;
            rt.submitted = false;
            match q.desc.Query {
                query_kind::TIMESTAMP => {
                    if let (Some(heap), Some(rb)) = (rt.heap.clone(), rt.readback.clone()) {
                        let slot = rt.slot;
                        let _ = exec.cl.write_timestamp(&heap, slot);
                        let _ = exec.cl.resolve_timestamps(&heap, slot, 1, &rb, 0);
                        exec.keep((heap, rb));
                        exec.has_work = true;
                    }
                }
                query_kind::OCCLUSION | query_kind::OCCLUSION_PREDICATE => {
                    if let (Some(heap), Some(rb)) = (rt.heap.clone(), rt.readback.clone()) {
                        let _ = exec.cl.end_occlusion(&heap, 0);
                        let _ = exec.cl.resolve_occlusion(&heap, 0, 1, &rb, 0);
                        exec.keep((heap, rb));
                        exec.has_work = true;
                    }
                }
                _ => {}
            }
            drop(rt);
            exec.pending_queries.push(q.clone());
        }
        RecordedCmd::DiscardResource(res) => {
            let mtl = res.current_mtl();
            exec.cl.discard_resource(&mtl, None);
            exec.keep(mtl);
        }
    }
}

unsafe fn exec_update_subresource_bytes(
    exec: &mut ImmediateExec,
    dst: &ResourceCore,
    dst_sub: u32,
    dst_box: Option<D3D11_BOX>,
    data: &[u8],
    src_row_pitch: u32,
    src_depth_pitch: u32,
) -> Result<(), String> {
    let dst_mtl = dst.current_mtl();
    if dst.is_buffer() {
        let (dst_off, len) = match dst_box {
            Some(b) => (b.left as u64, b.right.saturating_sub(b.left) as u64),
            None => (0, dst_mtl.desc.Width),
        };
        let copy_len = (len as usize)
            .min(data.len())
            .min((dst_mtl.desc.Width.saturating_sub(dst_off)) as usize);
        if copy_len == 0 {
            return Ok(());
        }
        let up_desc = ResourceDesc {
            Dimension: dim::BUFFER,
            Width: copy_len as u64,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            ..Default::default()
        };
        let hp = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
        let up = Arc::new(
            new_committed_resource(&up_desc, &hp, "d3d11 update buf")
                .map_err(|e| format!("{e:?}"))?,
        );
        up.register_ptr();
        let cpu = up.cpu_ptr().ok_or("upload buffer has no cpu_ptr")?;
        std::ptr::copy_nonoverlapping(data.as_ptr(), cpu, copy_len);
        exec.cl.barrier();
        exec.cl.copy_buffer(&dst_mtl, dst_off, &up, 0, copy_len as u64)?;
        exec.cl.barrier();
        exec.has_work = true;
        exec.keep((up, dst_mtl));
        return Ok(());
    }

    let (layouts, _) = d3d12_metal::layout::copyable_footprints(&dst_mtl.desc, dst_sub, 1, 0)
        .ok_or_else(|| format!("copyable_footprints failed for subresource {dst_sub}"))?;
    let lay = layouts[0];
    let full_w = lay.footprint.width;
    let full_h = lay.footprint.height;
    let full_d = lay.footprint.depth.max(1);
    let fi = d3d12_metal::format::info(dst_mtl.desc.Format);
    let (bw, bh) = (fi.block_w.max(1), fi.block_h.max(1));
    let bpe = fi.bytes.max(1) as usize;

    let (x0, y0, z0, w, h, d) = match dst_box {
        Some(b) => {
            if b.right <= b.left || b.bottom <= b.top || b.back <= b.front {
                return Ok(());
            }
            (b.left, b.top, b.front, b.right - b.left, b.bottom - b.top, b.back - b.front)
        }
        None => (0, 0, 0, full_w, full_h, full_d),
    };
    let row_bytes = (w.div_ceil(bw) as usize) * bpe;
    let num_rows = h.div_ceil(bh) as usize;
    let dst_row_pitch = row_bytes.next_multiple_of(256).max(256);
    let dst_slice_pitch = dst_row_pitch * num_rows;
    let total = dst_slice_pitch * d as usize;
    let up_desc = ResourceDesc {
        Dimension: dim::BUFFER,
        Width: (total as u64).max(16),
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        ..Default::default()
    };
    let hp = HeapProperties { Type: heap_type::UPLOAD, ..Default::default() };
    let up = Arc::new(
        new_committed_resource(&up_desc, &hp, "d3d11 update tex")
            .map_err(|e| format!("{e:?}"))?,
    );
    up.register_ptr();
    let dst_base = up.cpu_ptr().ok_or("upload buffer has no cpu_ptr")?;
    let srp = if src_row_pitch == 0 { row_bytes } else { src_row_pitch as usize };
    let sdp = if src_depth_pitch == 0 { srp * num_rows } else { src_depth_pitch as usize };
    for z in 0..d as usize {
        for y in 0..num_rows {
            let soff = z * sdp + y * srp;
            let doff = z * dst_slice_pitch + y * dst_row_pitch;
            if soff < data.len() {
                let n = row_bytes.min(data.len() - soff);
                std::ptr::copy_nonoverlapping(data.as_ptr().add(soff), dst_base.add(doff), n);
            }
        }
    }
    let fp = PlacedFootprint {
        offset: 0,
        format: lay.footprint.format,
        width: w.next_multiple_of(bw),
        height: h.next_multiple_of(bh),
        depth: d,
        row_pitch: dst_row_pitch as u32,
        _pad: 0,
    };
    exec.cl.barrier();
    exec.cl.copy_texture_region(
        CopyLoc::Subresource(&dst_mtl, dst_sub),
        [x0, y0, z0],
        CopyLoc::Footprint(&up, fp),
        None,
    )?;
    exec.cl.barrier();
    exec.has_work = true;
    exec.keep((up, dst_mtl));
    Ok(())
}

unsafe fn prepare_gfx_draw(
    device: &DeviceCore,
    exec: &mut ImmediateExec,
    st: &ContextState,
    indexed: bool,
    indirect_arg: Option<&Arc<MtlResource>>,
) -> Result<(), String> {
    let pso = get_or_build_gfx_pso(device, st, indexed)?;

    // Allocate VS + PS (+ optional GS) resource & sampler descriptor tables.
    let (vs_res_cpu, vs_res_gpu) = exec.alloc_res_slots(VS_RES_TABLE_LEN)?;
    let (ps_res_cpu, ps_res_gpu) = exec.alloc_res_slots(PS_RES_TABLE_LEN)?;
    let (vs_samp_cpu, vs_samp_gpu) = exec.alloc_samp_slots(RS_SAMPLER_COUNT)?;
    let (ps_samp_cpu, ps_samp_gpu) = exec.alloc_samp_slots(RS_SAMPLER_COUNT)?;
    let gs_tables = if st.gs.is_some() {
        let (gs_res_cpu, gs_res_gpu) = exec.alloc_res_slots(GS_RES_TABLE_LEN)?;
        let (gs_samp_cpu, gs_samp_gpu) = exec.alloc_samp_slots(RS_SAMPLER_COUNT)?;
        populate_stage_table(exec, gs_res_cpu, &st.gs_bind, None);
        populate_sampler_table(gs_samp_cpu, &st.gs_bind);
        Some((gs_res_gpu, gs_samp_gpu))
    } else {
        None
    };

    populate_stage_table(exec, vs_res_cpu, &st.vs_bind, None);
    populate_stage_table(exec, ps_res_cpu, &st.ps_bind, Some(&st.om_uavs));
    populate_sampler_table(vs_samp_cpu, &st.vs_bind);
    populate_sampler_table(ps_samp_cpu, &st.ps_bind);
    bump_desc_heap_gen();

    // Render targets & DSV.
    let mut rtv_views = Vec::new();
    let mut attached_ptrs = Vec::new();
    let mut max_rt = 0usize;
    for (i, slot) in st.rtvs.iter().enumerate() {
        if slot.rtv.is_some() {
            max_rt = i + 1;
        }
    }
    for slot in &st.rtvs[..max_rt] {
        if let Some(rtv) = &slot.rtv {
            let (mtl, tv) = rtv.target_view()?;
            attached_ptrs.push(Arc::as_ptr(&mtl) as usize);
            exec.keep(mtl);
            rtv_views.push(tv);
        } else {
            rtv_views.push(TargetView::NULL);
        }
    }
    let dsv_view = match &st.dsv {
        Some(dsv) => {
            let (mtl, tv) = dsv.target_view()?;
            attached_ptrs.push(Arc::as_ptr(&mtl) as usize);
            exec.keep(mtl);
            Some(tv)
        }
        None => None,
    };

    // Implicit D3D11 resource hazard barriers: prior render/copy/compute writes to
    // bound SRVs, OM UAVs, or indirect argument buffers must complete before this draw.
    let mut barriers = Vec::new();
    for slot in st.vs_bind.srvs.iter().chain(st.gs_bind.srvs.iter()).chain(st.ps_bind.srvs.iter()) {
        if let Some(srv) = &slot.srv {
            let mtl = srv.resource.current_mtl();
            let ptr = Arc::as_ptr(&mtl) as usize;
            if !attached_ptrs.contains(&ptr) {
                // RENDER_TARGET | DEPTH_WRITE | UNORDERED_ACCESS | COPY_DEST -> NON_PIXEL_SRV | PIXEL_SRV
                barriers.push(Barrier::Transition(ptr, 0x4 | 0x10 | 0x8 | 0x400, 0xC0));
            }
            exec.keep(mtl);
        }
    }
    for slot in &st.om_uavs {
        if let Some(uav) = &slot.uav {
            let mtl = uav.resource.current_mtl();
            barriers.push(Barrier::Uav(Arc::as_ptr(&mtl) as usize));
            exec.keep(mtl);
            if let Some(counter) = &uav.counter {
                barriers.push(Barrier::Uav(Arc::as_ptr(counter) as usize));
                exec.keep(counter.clone());
            }
        }
    }
    if let Some(arg_mtl) = indirect_arg {
        // Any prior write -> INDIRECT_ARGUMENT (0x200)
        barriers.push(Barrier::Transition(Arc::as_ptr(arg_mtl) as usize, 0, 0x200));
    }
    if !barriers.is_empty() {
        exec.cl.barrier_list(&barriers);
    }

    exec.bind_active_heaps();
    exec.cl.gfx_root.set_root_signature(device.gfx_rs.clone())?;
    exec.cl.gfx_root.set_table(0, vs_res_gpu)?;
    exec.cl.gfx_root.set_table(1, vs_samp_gpu)?;
    exec.cl.gfx_root.set_table(2, ps_res_gpu)?;
    exec.cl.gfx_root.set_table(3, ps_samp_gpu)?;
    if let Some((gs_res_gpu, gs_samp_gpu)) = gs_tables {
        exec.cl.gfx_root.set_table(4, gs_res_gpu)?;
        exec.cl.gfx_root.set_table(5, gs_samp_gpu)?;
    }

    exec.cl.set_render_targets(rtv_views, dsv_view);
    exec.cl.set_graphics_pipeline(pso.clone());
    exec.keep(pso);

    // Viewports & scissors (D3D11 only clips scissors when RasterizerDesc.ScissorEnable != 0).
    let mut mtl_vps = Vec::with_capacity(st.viewports.len());
    for vp in &st.viewports {
        push_mtl_viewport(&mut mtl_vps, vp);
    }
    exec.cl.set_viewports(mtl_vps);
    let scissor_on = st.rast.as_ref().is_some_and(|r| r.scissor_enable);
    exec.cl.set_scissors(if scissor_on { st.scissors.clone() } else { Vec::new() });
    exec.cl.set_blend_factor(st.blend_factor);
    exec.cl.set_stencil_ref(st.stencil_ref);

    // Vertex buffers & index buffer.
    exec.cl.gfx.topology = st.topology;
    for (i, vb) in st.vbs.iter().take(MAX_VERTEX_SLOTS as usize).enumerate() {
        if let Some(res) = &vb.res {
            let mtl = res.current_mtl();
            let off = vb.offset as u64;
            let avail = mtl.desc.Width.saturating_sub(off).min(u32::MAX as u64) as u32;
            exec.cl.gfx.vbs[i] = (mtl.gpu_va + off, avail, vb.stride);
            exec.keep(mtl);
        } else {
            exec.cl.gfx.vbs[i] = (0, 0, 0);
        }
    }
    if indexed {
        if let Some(ib) = &st.ib {
            let mtl = ib.current_mtl();
            let off = st.ib_offset as u64;
            let avail = mtl.desc.Width.saturating_sub(off).min(u32::MAX as u64) as u32;
            exec.cl.gfx.ib = Some((mtl.gpu_va + off, avail, st.ib_format));
            exec.keep(mtl);
        } else {
            return Err("indexed draw with no index buffer bound".into());
        }
    } else {
        exec.cl.gfx.ib = None;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
unsafe fn exec_draw(
    device: &DeviceCore,
    exec: &mut ImmediateExec,
    st: &ContextState,
    indexed_start: Option<u32>,
    count: u32,
    instances: u32,
    start_vertex: u32,
    base_vertex: i32,
    start_instance: u32,
) -> Result<(), String> {
    if count == 0 || instances == 0 || st.vs.is_none() {
        return Ok(());
    }
    prepare_gfx_draw(device, exec, st, indexed_start.is_some(), None)?;
    if let Some(start_index) = indexed_start {
        exec.cl.draw_indexed(count, instances, start_index, base_vertex, start_instance)?;
    } else {
        exec.cl.draw(count, instances, start_vertex, start_instance)?;
    }
    exec.has_work = true;
    Ok(())
}

unsafe fn exec_draw_indirect(
    device: &DeviceCore,
    exec: &mut ImmediateExec,
    st: &ContextState,
    args: &ResourceCore,
    offset: u32,
    indexed: bool,
) -> Result<(), String> {
    if st.vs.is_none() {
        return Ok(());
    }
    let arg_size: u64 = if indexed { 20 } else { 16 };
    let arg_mtl = args.current_mtl();
    let off = offset as u64;
    if off & 3 != 0 || off.saturating_add(arg_size) > arg_mtl.desc.Width {
        return Err(format!(
            "indirect draw args offset {off}+{arg_size} out of bounds or misaligned ({}B buffer)",
            arg_mtl.desc.Width
        ));
    }
    arg_mtl.register_ptr();
    prepare_gfx_draw(device, exec, st, indexed, Some(&arg_mtl))?;
    let layout = IndirectLayout {
        stride: arg_size as u32,
        work: if indexed { Work::DrawIndexed } else { Work::Draw },
        work_offset: 0,
        patches: Vec::new(),
        vb_offsets: Vec::new(),
        ib_offset: None,
    };
    exec.cl.execute_indirect(&layout, 1, arg_mtl.gpu_va + off, None, None)?;
    exec.keep(arg_mtl);
    exec.has_work = true;
    Ok(())
}

unsafe fn prepare_compute_dispatch(
    device: &DeviceCore,
    exec: &mut ImmediateExec,
    st: &ContextState,
) -> Result<(), String> {
    let Some(cs) = &st.cs else {
        return Err("no compute shader bound".into());
    };
    let pso = cs.pipeline()?;
    let (res_cpu, res_gpu) = exec.alloc_res_slots(CS_RES_TABLE_LEN)?;
    let (samp_cpu, samp_gpu) = exec.alloc_samp_slots(RS_SAMPLER_COUNT)?;
    populate_stage_table(exec, res_cpu, &st.cs_bind, Some(&st.cs_uavs));
    populate_sampler_table(samp_cpu, &st.cs_bind);
    bump_desc_heap_gen();

    exec.bind_active_heaps();
    exec.cl.compute_root.set_root_signature(device.compute_rs.clone())?;
    exec.cl.compute_root.set_table(0, res_gpu)?;
    exec.cl.compute_root.set_table(1, samp_gpu)?;
    exec.cl.set_compute_pipeline(pso.clone());
    exec.keep(pso);
    Ok(())
}

unsafe fn exec_dispatch(
    device: &DeviceCore,
    exec: &mut ImmediateExec,
    st: &ContextState,
    x: u32,
    y: u32,
    z: u32,
) -> Result<(), String> {
    if x == 0 || y == 0 || z == 0 {
        return Ok(());
    }
    prepare_compute_dispatch(device, exec, st)?;
    exec.cl.barrier();
    exec.cl.dispatch(x, y, z)?;
    exec.cl.barrier();
    exec.has_work = true;
    Ok(())
}

unsafe fn exec_dispatch_indirect(
    device: &DeviceCore,
    exec: &mut ImmediateExec,
    st: &ContextState,
    args: &ResourceCore,
    offset: u32,
) -> Result<(), String> {
    let arg_mtl = args.current_mtl();
    let off = offset as u64;
    if off & 3 != 0 || off.saturating_add(12) > arg_mtl.desc.Width {
        return Err(format!(
            "DispatchIndirect args offset {off}+12 out of bounds or misaligned ({}B buffer)",
            arg_mtl.desc.Width
        ));
    }
    arg_mtl.register_ptr();
    prepare_compute_dispatch(device, exec, st)?;
    let layout = IndirectLayout {
        stride: 12,
        work: Work::Dispatch,
        work_offset: 0,
        patches: Vec::new(),
        vb_offsets: Vec::new(),
        ib_offset: None,
    };
    exec.cl.barrier();
    exec.cl.execute_indirect(&layout, 1, arg_mtl.gpu_va + off, None, None)?;
    exec.cl.barrier();
    exec.keep(arg_mtl);
    exec.has_work = true;
    Ok(())
}

unsafe fn exec_generate_mips(exec: &mut ImmediateExec, srv: &SrvInner) -> Result<(), String> {
    let misc = srv.resource.kind.misc_flags();
    let bind = srv.resource.kind.bind_flags();
    if misc & misc_flag::GENERATE_MIPS == 0 {
        log::debug!(
            target: "d3d",
            "d3d11 GenerateMips: ignored because resource lacks D3D11_RESOURCE_MISC_GENERATE_MIPS (misc={misc:#x}, bind={bind:#x})"
        );
        return Ok(());
    }
    let u = srv.desc.u;
    let (most_detailed_mip, mip_levels, first_slice, slice_count) = match srv.desc.ViewDimension {
        srv_dim::TEXTURE1D | srv_dim::TEXTURE2D | srv_dim::TEXTURE3D => (u[0], u[1], 0, 1),
        srv_dim::TEXTURE1DARRAY | srv_dim::TEXTURE2DARRAY => (u[0], u[1], u[2], u[3]),
        srv_dim::TEXTURECUBE => (u[0], u[1], 0, 6),
        srv_dim::TEXTURECUBEARRAY => (u[0], u[1], u[2], u[3].saturating_mul(6)),
        _ => return Ok(()),
    };
    let mtl = srv.resource.current_mtl();
    exec.cl.barrier();
    exec.cl.generate_mips(
        &mtl,
        srv.desc.Format,
        most_detailed_mip,
        mip_levels,
        first_slice,
        slice_count,
    )?;
    exec.cl.barrier();
    exec.keep(mtl);
    exec.has_work = true;
    Ok(())
}

// ---------------------------------------------------------------------------
// D3d11Context COM object (both immediate and deferred)
// ---------------------------------------------------------------------------

pub struct D3d11Context {
    pub device: Arc<DeviceCore>,
    pub deferred: bool,
    pub flags: u32,
    pub state: Mutex<ContextState>,
    pub recorded: Mutex<Vec<RecordedCmd>>,
}

impl D3d11Context {
    pub fn new_immediate(device: Arc<DeviceCore>) -> *mut Obj<Self> {
        Obj::new(Self {
            device,
            deferred: false,
            flags: 0,
            state: Mutex::new(ContextState::default()),
            recorded: Mutex::new(Vec::new()),
        })
    }

    pub fn new_deferred(device: Arc<DeviceCore>, flags: u32) -> *mut Obj<Self> {
        Obj::new(Self {
            device,
            deferred: true,
            flags,
            state: Mutex::new(ContextState::default()),
            recorded: Mutex::new(Vec::new()),
        })
    }

    fn emit(&self, cmd: RecordedCmd) {
        if self.deferred {
            self.recorded.lock().unwrap().push(cmd);
        } else {
            unsafe { exec_recorded_cmd(&self.device, &cmd) };
        }
    }
}

impl HasDeviceCore for D3d11Context {
    fn device_core(&self) -> &Arc<DeviceCore> { &self.device }
}

static CTX_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for D3d11Context {
    const TRACE_RELEASE: bool = false;

    fn class() -> &'static Class {
        class_once(&CTX_CLASS, || {
            let mut s = child_slots::<D3d11Context>();
            s.extend([
                // [7..114]: ID3D11DeviceContext
                ("VSSetConstantBuffers", Some(vs_set_constant_buffers as *const ())),
                ("PSSetShaderResources", Some(ps_set_shader_resources as *const ())),
                ("PSSetShader", Some(ps_set_shader as *const ())),
                ("PSSetSamplers", Some(ps_set_samplers as *const ())),
                ("VSSetShader", Some(vs_set_shader as *const ())),
                ("DrawIndexed", Some(draw_indexed as *const ())),
                ("Draw", Some(draw as *const ())),
                ("Map", Some(map as *const ())),
                ("Unmap", Some(unmap as *const ())),
                ("PSSetConstantBuffers", Some(ps_set_constant_buffers as *const ())),
                ("IASetInputLayout", Some(ia_set_input_layout as *const ())),
                ("IASetVertexBuffers", Some(ia_set_vertex_buffers as *const ())),
                ("IASetIndexBuffer", Some(ia_set_index_buffer as *const ())),
                ("DrawIndexedInstanced", Some(draw_indexed_instanced as *const ())),
                ("DrawInstanced", Some(draw_instanced as *const ())),
                ("GSSetConstantBuffers", Some(gs_set_constant_buffers as *const ())),
                ("GSSetShader", Some(gs_set_shader as *const ())),
                ("IASetPrimitiveTopology", Some(ia_set_primitive_topology as *const ())),
                ("VSSetShaderResources", Some(vs_set_shader_resources as *const ())),
                ("VSSetSamplers", Some(vs_set_samplers as *const ())),
                ("Begin", Some(begin_async as *const ())),
                ("End", Some(end_async as *const ())),
                ("GetData", Some(get_data as *const ())),
                ("SetPredication", Some(set_predication as *const ())),
                ("GSSetShaderResources", Some(gs_set_shader_resources as *const ())),
                ("GSSetSamplers", Some(gs_set_samplers as *const ())),
                ("OMSetRenderTargets", Some(om_set_render_targets as *const ())),
                ("OMSetRenderTargetsAndUnorderedAccessViews", Some(om_set_render_targets_and_uavs as *const ())),
                ("OMSetBlendState", Some(om_set_blend_state as *const ())),
                ("OMSetDepthStencilState", Some(om_set_depth_stencil_state as *const ())),
                ("SOSetTargets", Some(so_set_targets as *const ())),
                ("DrawAuto", Some(draw_auto as *const ())),
                ("DrawIndexedInstancedIndirect", Some(draw_indexed_instanced_indirect as *const ())),
                ("DrawInstancedIndirect", Some(draw_instanced_indirect as *const ())),
                ("Dispatch", Some(dispatch as *const ())),
                ("DispatchIndirect", Some(dispatch_indirect as *const ())),
                ("RSSetState", Some(rs_set_state as *const ())),
                ("RSSetViewports", Some(rs_set_viewports as *const ())),
                ("RSSetScissorRects", Some(rs_set_scissor_rects as *const ())),
                ("CopySubresourceRegion", Some(copy_subresource_region as *const ())),
                ("CopyResource", Some(copy_resource as *const ())),
                ("UpdateSubresource", Some(update_subresource as *const ())),
                ("CopyStructureCount", Some(copy_structure_count as *const ())),
                ("ClearRenderTargetView", Some(clear_render_target_view as *const ())),
                ("ClearUnorderedAccessViewUint", Some(clear_uav_uint as *const ())),
                ("ClearUnorderedAccessViewFloat", Some(clear_uav_float as *const ())),
                ("ClearDepthStencilView", Some(crate::bridge_entry!(clear_depth_stencil_view))),
                ("GenerateMips", Some(generate_mips as *const ())),
                ("SetResourceMinLOD", Some(crate::bridge_entry!(set_resource_min_lod))),
                ("GetResourceMinLOD", Some(get_resource_min_lod as *const ())),
                ("ResolveSubresource", Some(resolve_subresource as *const ())),
                ("ExecuteCommandList", Some(execute_command_list as *const ())),
                ("HSSetShaderResources", Some(hs_set_shader_resources as *const ())),
                ("HSSetShader", Some(hs_set_shader as *const ())),
                ("HSSetSamplers", Some(hs_set_samplers as *const ())),
                ("HSSetConstantBuffers", Some(hs_set_constant_buffers as *const ())),
                ("DSSetShaderResources", Some(ds_set_shader_resources as *const ())),
                ("DSSetShader", Some(ds_set_shader as *const ())),
                ("DSSetSamplers", Some(ds_set_samplers as *const ())),
                ("DSSetConstantBuffers", Some(ds_set_constant_buffers as *const ())),
                ("CSSetShaderResources", Some(cs_set_shader_resources as *const ())),
                ("CSSetUnorderedAccessViews", Some(cs_set_uavs as *const ())),
                ("CSSetShader", Some(cs_set_shader as *const ())),
                ("CSSetSamplers", Some(cs_set_samplers as *const ())),
                ("CSSetConstantBuffers", Some(cs_set_constant_buffers as *const ())),
                ("VSGetConstantBuffers", Some(vs_get_constant_buffers as *const ())),
                ("PSGetShaderResources", Some(ps_get_shader_resources as *const ())),
                ("PSGetShader", Some(ps_get_shader as *const ())),
                ("PSGetSamplers", Some(ps_get_samplers as *const ())),
                ("VSGetShader", Some(vs_get_shader as *const ())),
                ("PSGetConstantBuffers", Some(ps_get_constant_buffers as *const ())),
                ("IAGetInputLayout", Some(ia_get_input_layout as *const ())),
                ("IAGetVertexBuffers", Some(ia_get_vertex_buffers as *const ())),
                ("IAGetIndexBuffer", Some(ia_get_index_buffer as *const ())),
                ("GSGetConstantBuffers", Some(gs_get_constant_buffers as *const ())),
                ("GSGetShader", Some(gs_get_shader as *const ())),
                ("IAGetPrimitiveTopology", Some(ia_get_primitive_topology as *const ())),
                ("VSGetShaderResources", Some(vs_get_shader_resources as *const ())),
                ("VSGetSamplers", Some(vs_get_samplers as *const ())),
                ("GetPredication", Some(get_predication as *const ())),
                ("GSGetShaderResources", Some(gs_get_shader_resources as *const ())),
                ("GSGetSamplers", Some(gs_get_samplers as *const ())),
                ("OMGetRenderTargets", Some(om_get_render_targets as *const ())),
                ("OMGetRenderTargetsAndUnorderedAccessViews", Some(om_get_render_targets_and_uavs as *const ())),
                ("OMGetBlendState", Some(om_get_blend_state as *const ())),
                ("OMGetDepthStencilState", Some(om_get_depth_stencil_state as *const ())),
                ("SOGetTargets", Some(so_get_targets as *const ())),
                ("RSGetState", Some(rs_get_state as *const ())),
                ("RSGetViewports", Some(rs_get_viewports as *const ())),
                ("RSGetScissorRects", Some(rs_get_scissor_rects as *const ())),
                ("HSGetShaderResources", Some(hs_get_shader_resources as *const ())),
                ("HSGetShader", Some(hs_get_shader as *const ())),
                ("HSGetSamplers", Some(hs_get_samplers as *const ())),
                ("HSGetConstantBuffers", Some(hs_get_constant_buffers as *const ())),
                ("DSGetShaderResources", Some(ds_get_shader_resources as *const ())),
                ("DSGetShader", Some(ds_get_shader as *const ())),
                ("DSGetSamplers", Some(ds_get_samplers as *const ())),
                ("DSGetConstantBuffers", Some(ds_get_constant_buffers as *const ())),
                ("CSGetShaderResources", Some(cs_get_shader_resources as *const ())),
                ("CSGetUnorderedAccessViews", Some(cs_get_uavs as *const ())),
                ("CSGetShader", Some(cs_get_shader as *const ())),
                ("CSGetSamplers", Some(cs_get_samplers as *const ())),
                ("CSGetConstantBuffers", Some(cs_get_constant_buffers as *const ())),
                ("ClearState", Some(clear_state as *const ())),
                ("Flush", Some(flush as *const ())),
                ("GetType", Some(get_type as *const ())),
                ("GetContextFlags", Some(get_context_flags as *const ())),
                ("FinishCommandList", Some(finish_command_list as *const ())),
                // [115..133]: ID3D11DeviceContext1
                ("CopySubresourceRegion1", Some(copy_subresource_region1 as *const ())),
                ("UpdateSubresource1", Some(update_subresource1 as *const ())),
                ("DiscardResource", Some(discard_resource as *const ())),
                ("DiscardView", Some(discard_view as *const ())),
                ("VSSetConstantBuffers1", Some(vs_set_constant_buffers1 as *const ())),
                ("HSSetConstantBuffers1", Some(hs_set_constant_buffers1 as *const ())),
                ("DSSetConstantBuffers1", Some(ds_set_constant_buffers1 as *const ())),
                ("GSSetConstantBuffers1", Some(gs_set_constant_buffers1 as *const ())),
                ("PSSetConstantBuffers1", Some(ps_set_constant_buffers1 as *const ())),
                ("CSSetConstantBuffers1", Some(cs_set_constant_buffers1 as *const ())),
                ("VSGetConstantBuffers1", Some(vs_get_constant_buffers1 as *const ())),
                ("HSGetConstantBuffers1", Some(hs_get_constant_buffers1 as *const ())),
                ("DSGetConstantBuffers1", Some(ds_get_constant_buffers1 as *const ())),
                ("GSGetConstantBuffers1", Some(gs_get_constant_buffers1 as *const ())),
                ("PSGetConstantBuffers1", Some(ps_get_constant_buffers1 as *const ())),
                ("CSGetConstantBuffers1", Some(cs_get_constant_buffers1 as *const ())),
                ("SwapDeviceContextState", Some(swap_device_context_state as *const ())),
                ("ClearView", Some(clear_view as *const ())),
                ("DiscardView1", Some(discard_view1 as *const ())),
            ]);
            debug_assert_eq!(s.len(), ID3D11_CONTEXT1_TOTAL_SLOTS);
            Class {
                name: "ID3D11DeviceContext1",
                vtbl: build_vtable("ID3D11DeviceContext1", &s),
                iids: &[IID_ID3D11DeviceChild, IID_ID3D11DeviceContext, IID_ID3D11DeviceContext1],
            }
        })
    }

    fn final_release(&self) {
        if !self.deferred {
            self.device.flush_immediate();
        }
    }
}

pub type PCtx = *mut Obj<D3d11Context>;

// ---------------------------------------------------------------------------
// Binding helpers
// ---------------------------------------------------------------------------

unsafe fn set_cbs(
    slots: &mut [CbSlot],
    start: u32,
    n: u32,
    bufs: *const *mut c_void,
    firsts: *const u32,
    nums: *const u32,
) {
    for i in 0..n as usize {
        let idx = start as usize + i;
        if idx >= slots.len() {
            break;
        }
        let p = if bufs.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(bufs.add(i)) };
        let fc = if firsts.is_null() { 0 } else { std::ptr::read_unaligned(firsts.add(i)) };
        let nc = if nums.is_null() { 0 } else { std::ptr::read_unaligned(nums.add(i)) };
        slots[idx] = CbSlot { com: ComRef::from_borrowed(p), res: buffer_from_ptr(p), first_constant: fc, num_constants: nc };
    }
}

unsafe fn get_cbs(
    slots: &[CbSlot],
    start: u32,
    n: u32,
    bufs: *mut *mut c_void,
    firsts: *mut u32,
    nums: *mut u32,
) {
    for i in 0..n as usize {
        let idx = start as usize + i;
        if let Some(s) = slots.get(idx) {
            if !bufs.is_null() {
                s.com.copy_out(bufs.add(i));
            }
            if !firsts.is_null() {
                std::ptr::write_unaligned(firsts.add(i), s.first_constant);
            }
            if !nums.is_null() {
                std::ptr::write_unaligned(nums.add(i), s.num_constants);
            }
        } else {
            if !bufs.is_null() { std::ptr::write_unaligned(bufs.add(i), std::ptr::null_mut()); }
            if !firsts.is_null() { std::ptr::write_unaligned(firsts.add(i), 0); }
            if !nums.is_null() { std::ptr::write_unaligned(nums.add(i), 0); }
        }
    }
}

unsafe fn set_srvs(slots: &mut [SrvSlot], start: u32, n: u32, views: *const *mut c_void) {
    for i in 0..n as usize {
        let idx = start as usize + i;
        if idx >= slots.len() { break; }
        let p = if views.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(views.add(i)) };
        let srv = Obj::<D3d11Srv>::from_iface(p).map(|o| Obj::get(o).inner.clone());
        slots[idx] = SrvSlot { com: ComRef::from_borrowed(p), srv };
    }
}

unsafe fn get_srvs(slots: &[SrvSlot], start: u32, n: u32, out: *mut *mut c_void) {
    if out.is_null() { return; }
    for i in 0..n as usize {
        if let Some(s) = slots.get(start as usize + i) {
            s.com.copy_out(out.add(i));
        } else {
            std::ptr::write_unaligned(out.add(i), std::ptr::null_mut());
        }
    }
}

unsafe fn set_samplers(slots: &mut [SampSlot], start: u32, n: u32, samps: *const *mut c_void) {
    for i in 0..n as usize {
        let idx = start as usize + i;
        if idx >= slots.len() { break; }
        let p = if samps.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(samps.add(i)) };
        slots[idx] = SampSlot { com: ComRef::from_borrowed(p), samp: samp_from_ptr(p) };
    }
}

unsafe fn get_samplers(slots: &[SampSlot], start: u32, n: u32, out: *mut *mut c_void) {
    if out.is_null() { return; }
    for i in 0..n as usize {
        if let Some(s) = slots.get(start as usize + i) {
            s.com.copy_out(out.add(i));
        } else {
            std::ptr::write_unaligned(out.add(i), std::ptr::null_mut());
        }
    }
}

unsafe fn set_uavs(
    slots: &mut [UavSlot],
    start: u32,
    n: u32,
    views: *const *mut c_void,
    init_counts: *const u32,
    counter_updates: &mut Vec<(Arc<UavInner>, u32)>,
) {
    for i in 0..n as usize {
        let idx = start as usize + i;
        if idx >= slots.len() { break; }
        let p = if views.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(views.add(i)) };
        let uav = Obj::<D3d11Uav>::from_iface(p).map(|o| Obj::get(o).inner.clone());
        if !init_counts.is_null() {
            let init = std::ptr::read_unaligned(init_counts.add(i));
            if init != D3D11_UAV_COUNTER_KEEP {
                if let Some(u) = &uav {
                    if u.counter.is_some() {
                        counter_updates.push((u.clone(), init));
                    }
                }
            }
        }
        slots[idx] = UavSlot { com: ComRef::from_borrowed(p), uav };
    }
}

unsafe fn get_uavs(slots: &[UavSlot], start: u32, n: u32, out: *mut *mut c_void) {
    if out.is_null() { return; }
    for i in 0..n as usize {
        if let Some(s) = slots.get(start as usize + i) {
            s.com.copy_out(out.add(i));
        } else {
            std::ptr::write_unaligned(out.add(i), std::ptr::null_mut());
        }
    }
}

// ---------------------------------------------------------------------------
// Shader & resource setter / getter slots
// ---------------------------------------------------------------------------

unsafe extern "win64" fn vs_set_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().vs_bind.cbs, start, n, bufs, std::ptr::null(), std::ptr::null());
}
unsafe extern "win64" fn ps_set_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().ps_bind.cbs, start, n, bufs, std::ptr::null(), std::ptr::null());
}
unsafe extern "win64" fn gs_set_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().gs_bind.cbs, start, n, bufs, std::ptr::null(), std::ptr::null());
}
unsafe extern "win64" fn hs_set_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().hs_bind.cbs, start, n, bufs, std::ptr::null(), std::ptr::null());
}
unsafe extern "win64" fn ds_set_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().ds_bind.cbs, start, n, bufs, std::ptr::null(), std::ptr::null());
}
unsafe extern "win64" fn cs_set_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().cs_bind.cbs, start, n, bufs, std::ptr::null(), std::ptr::null());
}

unsafe extern "win64" fn vs_set_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void, f: *const u32, c: *const u32) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().vs_bind.cbs, start, n, bufs, f, c);
}
unsafe extern "win64" fn ps_set_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void, f: *const u32, c: *const u32) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().ps_bind.cbs, start, n, bufs, f, c);
}
unsafe extern "win64" fn gs_set_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void, f: *const u32, c: *const u32) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().gs_bind.cbs, start, n, bufs, f, c);
}
unsafe extern "win64" fn hs_set_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void, f: *const u32, c: *const u32) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().hs_bind.cbs, start, n, bufs, f, c);
}
unsafe extern "win64" fn ds_set_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void, f: *const u32, c: *const u32) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().ds_bind.cbs, start, n, bufs, f, c);
}
unsafe extern "win64" fn cs_set_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *const *mut c_void, f: *const u32, c: *const u32) {
    set_cbs(&mut Obj::get(this).state.lock().unwrap().cs_bind.cbs, start, n, bufs, f, c);
}

unsafe extern "win64" fn vs_get_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void) {
    get_cbs(&Obj::get(this).state.lock().unwrap().vs_bind.cbs, start, n, bufs, std::ptr::null_mut(), std::ptr::null_mut());
}
unsafe extern "win64" fn ps_get_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void) {
    get_cbs(&Obj::get(this).state.lock().unwrap().ps_bind.cbs, start, n, bufs, std::ptr::null_mut(), std::ptr::null_mut());
}
unsafe extern "win64" fn gs_get_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void) {
    get_cbs(&Obj::get(this).state.lock().unwrap().gs_bind.cbs, start, n, bufs, std::ptr::null_mut(), std::ptr::null_mut());
}
unsafe extern "win64" fn hs_get_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void) {
    get_cbs(&Obj::get(this).state.lock().unwrap().hs_bind.cbs, start, n, bufs, std::ptr::null_mut(), std::ptr::null_mut());
}
unsafe extern "win64" fn ds_get_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void) {
    get_cbs(&Obj::get(this).state.lock().unwrap().ds_bind.cbs, start, n, bufs, std::ptr::null_mut(), std::ptr::null_mut());
}
unsafe extern "win64" fn cs_get_constant_buffers(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void) {
    get_cbs(&Obj::get(this).state.lock().unwrap().cs_bind.cbs, start, n, bufs, std::ptr::null_mut(), std::ptr::null_mut());
}

unsafe extern "win64" fn vs_get_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void, f: *mut u32, c: *mut u32) {
    get_cbs(&Obj::get(this).state.lock().unwrap().vs_bind.cbs, start, n, bufs, f, c);
}
unsafe extern "win64" fn ps_get_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void, f: *mut u32, c: *mut u32) {
    get_cbs(&Obj::get(this).state.lock().unwrap().ps_bind.cbs, start, n, bufs, f, c);
}
unsafe extern "win64" fn gs_get_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void, f: *mut u32, c: *mut u32) {
    get_cbs(&Obj::get(this).state.lock().unwrap().gs_bind.cbs, start, n, bufs, f, c);
}
unsafe extern "win64" fn hs_get_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void, f: *mut u32, c: *mut u32) {
    get_cbs(&Obj::get(this).state.lock().unwrap().hs_bind.cbs, start, n, bufs, f, c);
}
unsafe extern "win64" fn ds_get_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void, f: *mut u32, c: *mut u32) {
    get_cbs(&Obj::get(this).state.lock().unwrap().ds_bind.cbs, start, n, bufs, f, c);
}
unsafe extern "win64" fn cs_get_constant_buffers1(this: PCtx, start: u32, n: u32, bufs: *mut *mut c_void, f: *mut u32, c: *mut u32) {
    get_cbs(&Obj::get(this).state.lock().unwrap().cs_bind.cbs, start, n, bufs, f, c);
}

unsafe extern "win64" fn vs_set_shader_resources(this: PCtx, start: u32, n: u32, v: *const *mut c_void) {
    set_srvs(&mut Obj::get(this).state.lock().unwrap().vs_bind.srvs, start, n, v);
}
unsafe extern "win64" fn ps_set_shader_resources(this: PCtx, start: u32, n: u32, v: *const *mut c_void) {
    set_srvs(&mut Obj::get(this).state.lock().unwrap().ps_bind.srvs, start, n, v);
}
unsafe extern "win64" fn gs_set_shader_resources(this: PCtx, start: u32, n: u32, v: *const *mut c_void) {
    set_srvs(&mut Obj::get(this).state.lock().unwrap().gs_bind.srvs, start, n, v);
}
unsafe extern "win64" fn hs_set_shader_resources(this: PCtx, start: u32, n: u32, v: *const *mut c_void) {
    set_srvs(&mut Obj::get(this).state.lock().unwrap().hs_bind.srvs, start, n, v);
}
unsafe extern "win64" fn ds_set_shader_resources(this: PCtx, start: u32, n: u32, v: *const *mut c_void) {
    set_srvs(&mut Obj::get(this).state.lock().unwrap().ds_bind.srvs, start, n, v);
}
unsafe extern "win64" fn cs_set_shader_resources(this: PCtx, start: u32, n: u32, v: *const *mut c_void) {
    set_srvs(&mut Obj::get(this).state.lock().unwrap().cs_bind.srvs, start, n, v);
}

unsafe extern "win64" fn vs_get_shader_resources(this: PCtx, start: u32, n: u32, v: *mut *mut c_void) {
    get_srvs(&Obj::get(this).state.lock().unwrap().vs_bind.srvs, start, n, v);
}
unsafe extern "win64" fn ps_get_shader_resources(this: PCtx, start: u32, n: u32, v: *mut *mut c_void) {
    get_srvs(&Obj::get(this).state.lock().unwrap().ps_bind.srvs, start, n, v);
}
unsafe extern "win64" fn gs_get_shader_resources(this: PCtx, start: u32, n: u32, v: *mut *mut c_void) {
    get_srvs(&Obj::get(this).state.lock().unwrap().gs_bind.srvs, start, n, v);
}
unsafe extern "win64" fn hs_get_shader_resources(this: PCtx, start: u32, n: u32, v: *mut *mut c_void) {
    get_srvs(&Obj::get(this).state.lock().unwrap().hs_bind.srvs, start, n, v);
}
unsafe extern "win64" fn ds_get_shader_resources(this: PCtx, start: u32, n: u32, v: *mut *mut c_void) {
    get_srvs(&Obj::get(this).state.lock().unwrap().ds_bind.srvs, start, n, v);
}
unsafe extern "win64" fn cs_get_shader_resources(this: PCtx, start: u32, n: u32, v: *mut *mut c_void) {
    get_srvs(&Obj::get(this).state.lock().unwrap().cs_bind.srvs, start, n, v);
}

unsafe extern "win64" fn vs_set_samplers(this: PCtx, start: u32, n: u32, s: *const *mut c_void) {
    set_samplers(&mut Obj::get(this).state.lock().unwrap().vs_bind.samplers, start, n, s);
}
unsafe extern "win64" fn ps_set_samplers(this: PCtx, start: u32, n: u32, s: *const *mut c_void) {
    set_samplers(&mut Obj::get(this).state.lock().unwrap().ps_bind.samplers, start, n, s);
}
unsafe extern "win64" fn gs_set_samplers(this: PCtx, start: u32, n: u32, s: *const *mut c_void) {
    set_samplers(&mut Obj::get(this).state.lock().unwrap().gs_bind.samplers, start, n, s);
}
unsafe extern "win64" fn hs_set_samplers(this: PCtx, start: u32, n: u32, s: *const *mut c_void) {
    set_samplers(&mut Obj::get(this).state.lock().unwrap().hs_bind.samplers, start, n, s);
}
unsafe extern "win64" fn ds_set_samplers(this: PCtx, start: u32, n: u32, s: *const *mut c_void) {
    set_samplers(&mut Obj::get(this).state.lock().unwrap().ds_bind.samplers, start, n, s);
}
unsafe extern "win64" fn cs_set_samplers(this: PCtx, start: u32, n: u32, s: *const *mut c_void) {
    set_samplers(&mut Obj::get(this).state.lock().unwrap().cs_bind.samplers, start, n, s);
}

unsafe extern "win64" fn vs_get_samplers(this: PCtx, start: u32, n: u32, s: *mut *mut c_void) {
    get_samplers(&Obj::get(this).state.lock().unwrap().vs_bind.samplers, start, n, s);
}
unsafe extern "win64" fn ps_get_samplers(this: PCtx, start: u32, n: u32, s: *mut *mut c_void) {
    get_samplers(&Obj::get(this).state.lock().unwrap().ps_bind.samplers, start, n, s);
}
unsafe extern "win64" fn gs_get_samplers(this: PCtx, start: u32, n: u32, s: *mut *mut c_void) {
    get_samplers(&Obj::get(this).state.lock().unwrap().gs_bind.samplers, start, n, s);
}
unsafe extern "win64" fn hs_get_samplers(this: PCtx, start: u32, n: u32, s: *mut *mut c_void) {
    get_samplers(&Obj::get(this).state.lock().unwrap().hs_bind.samplers, start, n, s);
}
unsafe extern "win64" fn ds_get_samplers(this: PCtx, start: u32, n: u32, s: *mut *mut c_void) {
    get_samplers(&Obj::get(this).state.lock().unwrap().ds_bind.samplers, start, n, s);
}
unsafe extern "win64" fn cs_get_samplers(this: PCtx, start: u32, n: u32, s: *mut *mut c_void) {
    get_samplers(&Obj::get(this).state.lock().unwrap().cs_bind.samplers, start, n, s);
}

unsafe extern "win64" fn cs_set_uavs(this: PCtx, start: u32, n: u32, views: *const *mut c_void, init_counts: *const u32) {
    let ctx = Obj::get(this);
    let mut counter_updates = Vec::new();
    {
        let mut st = ctx.state.lock().unwrap();
        set_uavs(&mut st.cs_uavs, start, n, views, init_counts, &mut counter_updates);
    }
    for (uav, value) in counter_updates {
        ctx.emit(RecordedCmd::SetUavCounter { uav, value });
    }
}
unsafe extern "win64" fn cs_get_uavs(this: PCtx, start: u32, n: u32, views: *mut *mut c_void) {
    get_uavs(&Obj::get(this).state.lock().unwrap().cs_uavs, start, n, views);
}

unsafe extern "win64" fn vs_set_shader(this: PCtx, shader: *mut c_void, _ci: *const *mut c_void, _nci: u32) {
    let mut st = Obj::get(this).state.lock().unwrap();
    st.vs_com = ComRef::from_borrowed(shader);
    st.vs = vs_from_ptr(shader);
}
unsafe extern "win64" fn ps_set_shader(this: PCtx, shader: *mut c_void, _ci: *const *mut c_void, _nci: u32) {
    let mut st = Obj::get(this).state.lock().unwrap();
    st.ps_com = ComRef::from_borrowed(shader);
    st.ps = ps_from_ptr(shader);
}
unsafe extern "win64" fn cs_set_shader(this: PCtx, shader: *mut c_void, _ci: *const *mut c_void, _nci: u32) {
    let mut st = Obj::get(this).state.lock().unwrap();
    st.cs_com = ComRef::from_borrowed(shader);
    st.cs = cs_from_ptr(shader);
}
unsafe extern "win64" fn gs_set_shader(this: PCtx, shader: *mut c_void, _ci: *const *mut c_void, _nci: u32) {
    let mut st = Obj::get(this).state.lock().unwrap();
    st.gs_com = ComRef::from_borrowed(shader);
    st.gs = gs_from_ptr(shader);
}
unsafe extern "win64" fn hs_set_shader(_this: PCtx, shader: *mut c_void, _ci: *const *mut c_void, _nci: u32) {
    if !shader.is_null() { stub!("HSSetShader(non-null) ignored"); }
}
unsafe extern "win64" fn ds_set_shader(_this: PCtx, shader: *mut c_void, _ci: *const *mut c_void, _nci: u32) {
    if !shader.is_null() { stub!("DSSetShader(non-null) ignored"); }
}

unsafe extern "win64" fn vs_get_shader(this: PCtx, out: *mut *mut c_void, _ci: *mut *mut c_void, nci: *mut u32) {
    Obj::get(this).state.lock().unwrap().vs_com.copy_out(out);
    if !nci.is_null() { std::ptr::write_unaligned(nci, 0); }
}
unsafe extern "win64" fn ps_get_shader(this: PCtx, out: *mut *mut c_void, _ci: *mut *mut c_void, nci: *mut u32) {
    Obj::get(this).state.lock().unwrap().ps_com.copy_out(out);
    if !nci.is_null() { std::ptr::write_unaligned(nci, 0); }
}
unsafe extern "win64" fn cs_get_shader(this: PCtx, out: *mut *mut c_void, _ci: *mut *mut c_void, nci: *mut u32) {
    Obj::get(this).state.lock().unwrap().cs_com.copy_out(out);
    if !nci.is_null() { std::ptr::write_unaligned(nci, 0); }
}
unsafe extern "win64" fn gs_get_shader(this: PCtx, out: *mut *mut c_void, _ci: *mut *mut c_void, nci: *mut u32) {
    Obj::get(this).state.lock().unwrap().gs_com.copy_out(out);
    if !nci.is_null() { std::ptr::write_unaligned(nci, 0); }
}
unsafe extern "win64" fn hs_get_shader(_this: PCtx, out: *mut *mut c_void, _ci: *mut *mut c_void, nci: *mut u32) {
    if !out.is_null() { std::ptr::write_unaligned(out, std::ptr::null_mut()); }
    if !nci.is_null() { std::ptr::write_unaligned(nci, 0); }
}
unsafe extern "win64" fn ds_get_shader(_this: PCtx, out: *mut *mut c_void, _ci: *mut *mut c_void, nci: *mut u32) {
    if !out.is_null() { std::ptr::write_unaligned(out, std::ptr::null_mut()); }
    if !nci.is_null() { std::ptr::write_unaligned(nci, 0); }
}

// ---------------------------------------------------------------------------
// Input Assembler, Output Merger, Rasterizer state
// ---------------------------------------------------------------------------

unsafe extern "win64" fn ia_set_input_layout(this: PCtx, il: *mut c_void) {
    let mut st = Obj::get(this).state.lock().unwrap();
    st.input_layout_com = ComRef::from_borrowed(il);
    st.input_layout = il_from_ptr(il);
}
unsafe extern "win64" fn ia_get_input_layout(this: PCtx, out: *mut *mut c_void) {
    Obj::get(this).state.lock().unwrap().input_layout_com.copy_out(out);
}

unsafe extern "win64" fn ia_set_vertex_buffers(
    this: PCtx,
    start: u32,
    n: u32,
    bufs: *const *mut c_void,
    strides: *const u32,
    offsets: *const u32,
) {
    let mut st = Obj::get(this).state.lock().unwrap();
    for i in 0..n as usize {
        let idx = start as usize + i;
        if idx >= st.vbs.len() { break; }
        let p = if bufs.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(bufs.add(i)) };
        let stride = if strides.is_null() { 0 } else { std::ptr::read_unaligned(strides.add(i)) };
        let offset = if offsets.is_null() { 0 } else { std::ptr::read_unaligned(offsets.add(i)) };
        st.vbs[idx] = VbSlot { com: ComRef::from_borrowed(p), res: buffer_from_ptr(p), stride, offset };
    }
}
unsafe extern "win64" fn ia_get_vertex_buffers(
    this: PCtx,
    start: u32,
    n: u32,
    bufs: *mut *mut c_void,
    strides: *mut u32,
    offsets: *mut u32,
) {
    let st = Obj::get(this).state.lock().unwrap();
    for i in 0..n as usize {
        if let Some(vb) = st.vbs.get(start as usize + i) {
            if !bufs.is_null() { vb.com.copy_out(bufs.add(i)); }
            if !strides.is_null() { std::ptr::write_unaligned(strides.add(i), vb.stride); }
            if !offsets.is_null() { std::ptr::write_unaligned(offsets.add(i), vb.offset); }
        } else {
            if !bufs.is_null() { std::ptr::write_unaligned(bufs.add(i), std::ptr::null_mut()); }
            if !strides.is_null() { std::ptr::write_unaligned(strides.add(i), 0); }
            if !offsets.is_null() { std::ptr::write_unaligned(offsets.add(i), 0); }
        }
    }
}

unsafe extern "win64" fn ia_set_index_buffer(this: PCtx, buf: *mut c_void, fmt: u32, offset: u32) {
    let mut st = Obj::get(this).state.lock().unwrap();
    st.ib_com = ComRef::from_borrowed(buf);
    st.ib = buffer_from_ptr(buf);
    st.ib_format = fmt;
    st.ib_offset = offset;
}
unsafe extern "win64" fn ia_get_index_buffer(this: PCtx, out: *mut *mut c_void, fmt: *mut u32, offset: *mut u32) {
    let st = Obj::get(this).state.lock().unwrap();
    st.ib_com.copy_out(out);
    if !fmt.is_null() { std::ptr::write_unaligned(fmt, st.ib_format); }
    if !offset.is_null() { std::ptr::write_unaligned(offset, st.ib_offset); }
}

unsafe extern "win64" fn ia_set_primitive_topology(this: PCtx, topo: u32) {
    Obj::get(this).state.lock().unwrap().topology = topo;
}
unsafe extern "win64" fn ia_get_primitive_topology(this: PCtx, out: *mut u32) {
    if !out.is_null() { std::ptr::write_unaligned(out, Obj::get(this).state.lock().unwrap().topology); }
}

unsafe extern "win64" fn om_set_render_targets(this: PCtx, n: u32, rtvs: *const *mut c_void, dsv: *mut c_void) {
    let mut st = Obj::get(this).state.lock().unwrap();
    for slot in &mut st.rtvs {
        *slot = RtvSlot::default();
    }
    for i in 0..(n as usize).min(st.rtvs.len()) {
        let p = if rtvs.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(rtvs.add(i)) };
        let rtv = Obj::<D3d11Rtv>::from_iface(p).map(|o| Obj::get(o).inner.clone());
        st.rtvs[i] = RtvSlot { com: ComRef::from_borrowed(p), rtv };
    }
    st.dsv_com = ComRef::from_borrowed(dsv);
    st.dsv = Obj::<D3d11Dsv>::from_iface(dsv).map(|o| Obj::get(o).inner.clone());
}

unsafe extern "win64" fn om_set_render_targets_and_uavs(
    this: PCtx,
    num_rtvs: u32,
    rtvs: *const *mut c_void,
    dsv: *mut c_void,
    uav_start: u32,
    num_uavs: u32,
    uavs: *const *mut c_void,
    init_counts: *const u32,
) {
    let ctx = Obj::get(this);
    let mut counter_updates = Vec::new();
    {
        let mut st = ctx.state.lock().unwrap();
        // D3D11_KEEP_RENDER_TARGETS_AND_DEPTH_STENCIL = 0xFFFFFFFF
        if num_rtvs != u32::MAX {
            for slot in &mut st.rtvs {
                *slot = RtvSlot::default();
            }
            for i in 0..(num_rtvs as usize).min(st.rtvs.len()) {
                let p = if rtvs.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(rtvs.add(i)) };
                let rtv = Obj::<D3d11Rtv>::from_iface(p).map(|o| Obj::get(o).inner.clone());
                st.rtvs[i] = RtvSlot { com: ComRef::from_borrowed(p), rtv };
            }
            st.dsv_com = ComRef::from_borrowed(dsv);
            st.dsv = Obj::<D3d11Dsv>::from_iface(dsv).map(|o| Obj::get(o).inner.clone());
        }
        // D3D11_KEEP_UNORDERED_ACCESS_VIEWS = 0xFFFFFFFF
        if num_uavs != u32::MAX {
            for slot in &mut st.om_uavs {
                *slot = UavSlot::default();
            }
            set_uavs(&mut st.om_uavs, uav_start, num_uavs, uavs, init_counts, &mut counter_updates);
        }
    }
    for (uav, value) in counter_updates {
        ctx.emit(RecordedCmd::SetUavCounter { uav, value });
    }
}

unsafe extern "win64" fn om_get_render_targets(this: PCtx, n: u32, rtvs: *mut *mut c_void, dsv: *mut *mut c_void) {
    let st = Obj::get(this).state.lock().unwrap();
    if !rtvs.is_null() {
        for i in 0..n as usize {
            if let Some(s) = st.rtvs.get(i) {
                s.com.copy_out(rtvs.add(i));
            } else {
                std::ptr::write_unaligned(rtvs.add(i), std::ptr::null_mut());
            }
        }
    }
    st.dsv_com.copy_out(dsv);
}

unsafe extern "win64" fn om_get_render_targets_and_uavs(
    this: PCtx,
    n_rtvs: u32,
    rtvs: *mut *mut c_void,
    dsv: *mut *mut c_void,
    uav_start: u32,
    n_uavs: u32,
    uavs: *mut *mut c_void,
) {
    om_get_render_targets(this, n_rtvs, rtvs, dsv);
    get_uavs(&Obj::get(this).state.lock().unwrap().om_uavs, uav_start, n_uavs, uavs);
}

unsafe extern "win64" fn om_set_blend_state(this: PCtx, state: *mut c_void, factor: *const [f32; 4], sample_mask: u32) {
    let mut st = Obj::get(this).state.lock().unwrap();
    st.blend_com = ComRef::from_borrowed(state);
    st.blend = blend_from_ptr(state);
    st.blend_factor = if factor.is_null() { [1.0; 4] } else { std::ptr::read_unaligned(factor) };
    st.sample_mask = sample_mask;
}
unsafe extern "win64" fn om_get_blend_state(this: PCtx, out: *mut *mut c_void, factor: *mut [f32; 4], mask: *mut u32) {
    let st = Obj::get(this).state.lock().unwrap();
    st.blend_com.copy_out(out);
    if !factor.is_null() { std::ptr::write_unaligned(factor, st.blend_factor); }
    if !mask.is_null() { std::ptr::write_unaligned(mask, st.sample_mask); }
}

unsafe extern "win64" fn om_set_depth_stencil_state(this: PCtx, state: *mut c_void, stencil_ref: u32) {
    let mut st = Obj::get(this).state.lock().unwrap();
    st.ds_state_com = ComRef::from_borrowed(state);
    st.ds_state = ds_from_ptr(state);
    st.stencil_ref = stencil_ref;
}
unsafe extern "win64" fn om_get_depth_stencil_state(this: PCtx, out: *mut *mut c_void, stencil_ref: *mut u32) {
    let st = Obj::get(this).state.lock().unwrap();
    st.ds_state_com.copy_out(out);
    if !stencil_ref.is_null() { std::ptr::write_unaligned(stencil_ref, st.stencil_ref); }
}

unsafe extern "win64" fn rs_set_state(this: PCtx, state: *mut c_void) {
    let mut st = Obj::get(this).state.lock().unwrap();
    st.rast_com = ComRef::from_borrowed(state);
    st.rast = rast_from_ptr(state);
}
unsafe extern "win64" fn rs_get_state(this: PCtx, out: *mut *mut c_void) {
    Obj::get(this).state.lock().unwrap().rast_com.copy_out(out);
}

unsafe extern "win64" fn rs_set_viewports(this: PCtx, n: u32, vps: *const D3D11_VIEWPORT) {
    let mut st = Obj::get(this).state.lock().unwrap();
    st.viewports.clear();
    if n > 0 && !vps.is_null() {
        let k = (n as usize).min(D3D11_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE);
        for i in 0..k {
            st.viewports.push(std::ptr::read_unaligned(vps.add(i)));
        }
    }
}
unsafe extern "win64" fn rs_get_viewports(this: PCtx, n: *mut u32, out: *mut D3D11_VIEWPORT) {
    if n.is_null() { return; }
    let st = Obj::get(this).state.lock().unwrap();
    if out.is_null() {
        std::ptr::write_unaligned(n, st.viewports.len() as u32);
        return;
    }
    let req = std::ptr::read_unaligned(n) as usize;
    let copy = req.min(st.viewports.len());
    for i in 0..copy {
        std::ptr::write_unaligned(out.add(i), st.viewports[i]);
    }
    for i in copy..req {
        std::ptr::write_unaligned(out.add(i), D3D11_VIEWPORT::default());
    }
    std::ptr::write_unaligned(n, copy as u32);
}

unsafe extern "win64" fn rs_set_scissor_rects(this: PCtx, n: u32, rects: *const Rect) {
    let mut st = Obj::get(this).state.lock().unwrap();
    st.scissors.clear();
    if n > 0 && !rects.is_null() {
        let k = (n as usize).min(D3D11_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE);
        for i in 0..k {
            st.scissors.push(std::ptr::read_unaligned(rects.add(i)));
        }
    }
}
unsafe extern "win64" fn rs_get_scissor_rects(this: PCtx, n: *mut u32, out: *mut Rect) {
    if n.is_null() { return; }
    let st = Obj::get(this).state.lock().unwrap();
    if out.is_null() {
        std::ptr::write_unaligned(n, st.scissors.len() as u32);
        return;
    }
    let req = std::ptr::read_unaligned(n) as usize;
    let copy = req.min(st.scissors.len());
    for i in 0..copy {
        std::ptr::write_unaligned(out.add(i), st.scissors[i]);
    }
    for i in copy..req {
        std::ptr::write_unaligned(out.add(i), Rect::default());
    }
    std::ptr::write_unaligned(n, copy as u32);
}

unsafe extern "win64" fn so_set_targets(this: PCtx, n: u32, bufs: *const *mut c_void, _offsets: *const u32) {
    let mut st = Obj::get(this).state.lock().unwrap();
    for s in &mut st.so_targets { *s = ComRef::default(); }
    for i in 0..(n as usize).min(st.so_targets.len()) {
        let p = if bufs.is_null() { std::ptr::null_mut() } else { std::ptr::read_unaligned(bufs.add(i)) };
        st.so_targets[i] = ComRef::from_borrowed(p);
    }
}
unsafe extern "win64" fn so_get_targets(this: PCtx, n: u32, out: *mut *mut c_void) {
    if out.is_null() { return; }
    let st = Obj::get(this).state.lock().unwrap();
    for i in 0..n as usize {
        if let Some(s) = st.so_targets.get(i) {
            s.copy_out(out.add(i));
        } else {
            std::ptr::write_unaligned(out.add(i), std::ptr::null_mut());
        }
    }
}

unsafe extern "win64" fn set_predication(this: PCtx, pred: *mut c_void, val: BOOL) {
    let mut st = Obj::get(this).state.lock().unwrap();
    st.predicate_com = ComRef::from_borrowed(pred);
    st.predicate_val = val;
}
unsafe extern "win64" fn get_predication(this: PCtx, out: *mut *mut c_void, val: *mut BOOL) {
    let st = Obj::get(this).state.lock().unwrap();
    st.predicate_com.copy_out(out);
    if !val.is_null() { std::ptr::write_unaligned(val, st.predicate_val); }
}

// ---------------------------------------------------------------------------
// Map / Unmap / UpdateSubresource / CopyResource / CopySubresourceRegion
// ---------------------------------------------------------------------------

unsafe extern "win64" fn map(
    this: PCtx,
    res_ptr: *mut c_void,
    sub: u32,
    mtype: u32,
    _flags: u32,
    out: *mut D3D11_MAPPED_SUBRESOURCE,
) -> HRESULT {
    let Some(res) = resource_from_ptr(res_ptr) else {
        return d11_failed!(E_INVALIDARG, "ID3D11DeviceContext::Map: null/foreign resource {res_ptr:?}");
    };
    let ctx = Obj::get(this);
    let writable = mtype != map_type::READ;

    if ctx.deferred {
        if !matches!(mtype, map_type::WRITE_DISCARD | map_type::WRITE_NO_OVERWRITE) {
            return d11_failed!(E_INVALIDARG, "DeferredContext::Map: only WRITE_DISCARD / WRITE_NO_OVERWRITE allowed");
        }
        let (size, row_pitch, depth_pitch) = if res.is_buffer() {
            let w = res.kind.buffer_width() as usize;
            (w.max(1), w as u32, w as u32)
        } else {
            let mtl = res.current_mtl();
            let Some((lays, _)) = d3d12_metal::layout::copyable_footprints(&mtl.desc, sub, 1, 0) else {
                return d11_failed!(E_INVALIDARG, "DeferredContext::Map: invalid subresource {sub}");
            };
            let lay = lays[0];
            let dp = lay.footprint.row_pitch * lay.num_rows;
            ((dp * lay.footprint.depth.max(1)) as usize, lay.footprint.row_pitch, dp)
        };
        let mut buf = vec![0u8; size];
        let ptr = buf.as_mut_ptr() as *mut c_void;
        res.mapped.lock().unwrap().insert(sub, MapState {
            map_type: mtype,
            writable: true,
            deferred_scratch: Some(buf),
            row_pitch,
            depth_pitch,
        });
        if !out.is_null() {
            std::ptr::write_unaligned(
                out,
                D3D11_MAPPED_SUBRESOURCE { pData: ptr, RowPitch: row_pitch, DepthPitch: depth_pitch },
            );
        }
        return S_OK;
    }

    // Immediate context Map.
    res.sync_other_devices(&ctx.device);
    if res.is_buffer() {
        let byte_width = res.kind.buffer_width();
        let mtl = if mtype == map_type::WRITE_DISCARD {
            match res.alloc_fresh_buffer_mtl() {
                Ok(fresh) => {
                    let old = res.replace_mtl(fresh.clone());
                    ctx.device.queue.defer_release(Box::new(old));
                    fresh
                }
                Err(e) => return d11_failed!(E_OUTOFMEMORY, "Map(WRITE_DISCARD): {e}"),
            }
        } else {
            if mtype != map_type::WRITE_NO_OVERWRITE {
                ctx.device.flush_and_wait_idle();
            }
            res.current_mtl()
        };
        let Some(ptr) = mtl.cpu_ptr() else {
            return d11_failed!(E_INVALIDARG, "Map: buffer {:?} is not CPU-accessible (usage={})", res.kind, res.kind.usage());
        };
        res.mapped.lock().unwrap().insert(sub, MapState {
            map_type: mtype,
            writable,
            deferred_scratch: None,
            row_pitch: byte_width,
            depth_pitch: byte_width,
        });
        if !out.is_null() {
            std::ptr::write_unaligned(
                out,
                D3D11_MAPPED_SUBRESOURCE {
                    pData: ptr as *mut c_void,
                    RowPitch: byte_width,
                    DepthPitch: byte_width,
                },
            );
        }
        return S_OK;
    }

    // Texture Map via staging buffer.
    let had_staging = res.tex_staging.lock().unwrap().is_some();
    let (staging_buf, layouts) = match res.ensure_tex_staging() {
        Ok(x) => x,
        Err(e) => return d11_failed!(E_FAIL, "Map texture staging: {e}"),
    };
    let Some(lay) = layouts.get(sub as usize).copied() else {
        return d11_failed!(E_INVALIDARG, "Map texture: subresource {sub} out of range ({})", layouts.len());
    };
    if matches!(mtype, map_type::READ | map_type::READ_WRITE) {
        let mtl = res.current_mtl();
        {
            let mut exec = ctx.device.imm_exec.lock().unwrap();
            if let Err(e) = exec.ensure_open() {
                return d11_failed!(E_FAIL, "Map texture readback ensure_open: {e}");
            }
            exec.cl.barrier();
            if let Err(e) = exec.cl.copy_texture_region(
                CopyLoc::Footprint(&staging_buf, lay.footprint),
                [0, 0, 0],
                CopyLoc::Subresource(&mtl, sub),
                None,
            ) {
                return d11_failed!(E_FAIL, "Map texture readback copy: {e}");
            }
            exec.has_work = true;
            exec.keep((mtl, staging_buf.clone()));
        }
        ctx.device.flush_and_wait_idle();
    } else if had_staging && mtype != map_type::WRITE_NO_OVERWRITE {
        // Ensure any earlier GPU upload from `staging_buf` has completed before CPU overwrites it.
        ctx.device.flush_and_wait_idle();
    }
    let Some(base_ptr) = staging_buf.cpu_ptr() else {
        return d11_failed!(E_FAIL, "Map texture: staging buffer has no cpu_ptr");
    };
    let ptr = base_ptr.add(lay.footprint.offset as usize) as *mut c_void;
    let row_pitch = lay.footprint.row_pitch;
    let depth_pitch = lay.footprint.row_pitch * lay.num_rows;
    res.mapped.lock().unwrap().insert(sub, MapState {
        map_type: mtype,
        writable,
        deferred_scratch: None,
        row_pitch,
        depth_pitch,
    });
    if !out.is_null() {
        std::ptr::write_unaligned(
            out,
            D3D11_MAPPED_SUBRESOURCE { pData: ptr, RowPitch: row_pitch, DepthPitch: depth_pitch },
        );
    }
    S_OK
}

unsafe extern "win64" fn unmap(this: PCtx, res_ptr: *mut c_void, sub: u32) {
    let Some(res) = resource_from_ptr(res_ptr) else {
        return;
    };
    let Some(ms) = res.mapped.lock().unwrap().remove(&sub) else {
        return;
    };
    let ctx = Obj::get(this);
    if let Some(data) = ms.deferred_scratch {
        ctx.emit(RecordedCmd::UpdateSubresource {
            dst: res,
            dst_sub: sub,
            dst_box: None,
            data,
            src_row_pitch: ms.row_pitch,
            src_depth_pitch: ms.depth_pitch,
        });
        return;
    }
    if !res.is_buffer() && ms.writable {
        if let Ok((staging_buf, layouts)) = res.ensure_tex_staging() {
            if let Some(lay) = layouts.get(sub as usize).copied() {
                let mtl = res.current_mtl();
                let mut exec = ctx.device.imm_exec.lock().unwrap();
                if exec.ensure_open().is_ok() {
                    exec.cl.barrier();
                    if let Err(e) = exec.cl.copy_texture_region(
                        CopyLoc::Subresource(&mtl, sub),
                        [0, 0, 0],
                        CopyLoc::Footprint(&staging_buf, lay.footprint),
                        None,
                    ) {
                        log::warn!(target: "d3d", "d3d11 Unmap texture upload: {e}");
                    } else {
                        exec.cl.barrier();
                        exec.has_work = true;
                    }
                    exec.keep((mtl, staging_buf));
                }
            }
        }
    }
}

unsafe extern "win64" fn copy_resource(this: PCtx, dst: *mut c_void, src: *mut c_void) {
    let (Some(d), Some(s)) = (resource_from_ptr(dst), resource_from_ptr(src)) else {
        log::warn!(target: "d3d", "d3d11 CopyResource: null/foreign resource ({dst:?}, {src:?})");
        return;
    };
    Obj::get(this).emit(RecordedCmd::CopyResource { dst: d, src: s });
}

unsafe extern "win64" fn copy_subresource_region(
    this: PCtx,
    dst: *mut c_void,
    dst_sub: u32,
    dst_x: u32,
    dst_y: u32,
    dst_z: u32,
    src: *mut c_void,
    src_sub: u32,
    src_box: *const D3D11_BOX,
) {
    let (Some(d), Some(s)) = (resource_from_ptr(dst), resource_from_ptr(src)) else {
        log::warn!(target: "d3d", "d3d11 CopySubresourceRegion: null/foreign resource");
        return;
    };
    let bx = if src_box.is_null() { None } else { Some(std::ptr::read_unaligned(src_box)) };
    Obj::get(this).emit(RecordedCmd::CopySubresourceRegion {
        dst: d,
        dst_sub,
        dst_xyz: [dst_x, dst_y, dst_z],
        src: s,
        src_sub,
        src_box: bx,
    });
}

unsafe extern "win64" fn copy_subresource_region1(
    this: PCtx,
    dst: *mut c_void,
    dst_sub: u32,
    dst_x: u32,
    dst_y: u32,
    dst_z: u32,
    src: *mut c_void,
    src_sub: u32,
    src_box: *const D3D11_BOX,
    _copy_flags: u32,
) {
    copy_subresource_region(this, dst, dst_sub, dst_x, dst_y, dst_z, src, src_sub, src_box);
}

unsafe extern "win64" fn update_subresource(
    this: PCtx,
    dst_ptr: *mut c_void,
    dst_sub: u32,
    dst_box: *const D3D11_BOX,
    src_data: *const c_void,
    src_row_pitch: u32,
    src_depth_pitch: u32,
) {
    if src_data.is_null() {
        return;
    }
    let Some(dst) = resource_from_ptr(dst_ptr) else {
        log::warn!(target: "d3d", "d3d11 UpdateSubresource: null/foreign dst {dst_ptr:?}");
        return;
    };
    let bx = if dst_box.is_null() { None } else { Some(std::ptr::read_unaligned(dst_box)) };
    let data = if dst.is_buffer() {
        let len = match bx {
            Some(b) => b.right.saturating_sub(b.left) as usize,
            None => dst.kind.buffer_width() as usize,
        };
        std::slice::from_raw_parts(src_data as *const u8, len).to_vec()
    } else {
        let mtl = dst.current_mtl();
        let Some((lays, _)) = d3d12_metal::layout::copyable_footprints(&mtl.desc, dst_sub, 1, 0) else {
            return;
        };
        let lay = lays[0];
        let fi = d3d12_metal::format::info(mtl.desc.Format);
        let (bw, bh) = (fi.block_w.max(1), fi.block_h.max(1));
        let bpe = fi.bytes.max(1) as usize;
        let (w, h, d) = match bx {
            Some(b) => (b.right.saturating_sub(b.left), b.bottom.saturating_sub(b.top), b.back.saturating_sub(b.front)),
            None => (lay.footprint.width, lay.footprint.height, lay.footprint.depth.max(1)),
        };
        if w == 0 || h == 0 || d == 0 {
            return;
        }
        let row_bytes = (w.div_ceil(bw) as usize) * bpe;
        let rows = h.div_ceil(bh) as usize;
        let srp = if src_row_pitch == 0 { row_bytes } else { src_row_pitch as usize };
        let sdp = if src_depth_pitch == 0 { srp * rows } else { src_depth_pitch as usize };
        let total = (d as usize - 1) * sdp + (rows - 1) * srp + row_bytes;
        std::slice::from_raw_parts(src_data as *const u8, total).to_vec()
    };
    Obj::get(this).emit(RecordedCmd::UpdateSubresource {
        dst,
        dst_sub,
        dst_box: bx,
        data,
        src_row_pitch,
        src_depth_pitch,
    });
}

unsafe extern "win64" fn update_subresource1(
    this: PCtx,
    dst: *mut c_void,
    dst_sub: u32,
    dst_box: *const D3D11_BOX,
    src_data: *const c_void,
    src_row_pitch: u32,
    src_depth_pitch: u32,
    _copy_flags: u32,
) {
    update_subresource(this, dst, dst_sub, dst_box, src_data, src_row_pitch, src_depth_pitch);
}

// ---------------------------------------------------------------------------
// Clears, Draws, Dispatches, Queries, Command Lists
// ---------------------------------------------------------------------------

unsafe extern "win64" fn clear_render_target_view(this: PCtx, rtv_ptr: *mut c_void, color: *const [f32; 4]) {
    let Some(o) = Obj::<D3d11Rtv>::from_iface(rtv_ptr) else {
        log::warn!(target: "d3d", "d3d11 ClearRenderTargetView: null/foreign RTV {rtv_ptr:?}");
        return;
    };
    let c = if color.is_null() { [0.0; 4] } else { std::ptr::read_unaligned(color) };
    Obj::get(this).emit(RecordedCmd::ClearRtv {
        rtv: Obj::get(o).inner.clone(),
        color: c,
        rects: Vec::new(),
    });
}

pub unsafe extern "win64" fn clear_depth_stencil_view(
    this: PCtx,
    dsv_ptr: *mut c_void,
    flags: u32,
    depth: f32,
    stencil: u8,
) {
    let Some(o) = Obj::<D3d11Dsv>::from_iface(dsv_ptr) else {
        log::warn!(target: "d3d", "d3d11 ClearDepthStencilView: null/foreign DSV {dsv_ptr:?}");
        return;
    };
    Obj::get(this).emit(RecordedCmd::ClearDsv {
        dsv: Obj::get(o).inner.clone(),
        flags,
        depth,
        stencil,
        rects: Vec::new(),
    });
}
crate::win64_bridge!(fn clear_depth_stencil_view(PCtx, *mut c_void, u32, f32, u8) -> ());

unsafe extern "win64" fn clear_uav_uint(this: PCtx, uav_ptr: *mut c_void, vals: *const [u32; 4]) {
    let Some(o) = Obj::<D3d11Uav>::from_iface(uav_ptr) else {
        log::warn!(target: "d3d", "d3d11 ClearUnorderedAccessViewUint: null/foreign UAV {uav_ptr:?}");
        return;
    };
    let values = if vals.is_null() { [0; 4] } else { std::ptr::read_unaligned(vals) };
    Obj::get(this).emit(RecordedCmd::ClearUavUint { uav: Obj::get(o).inner.clone(), values });
}

unsafe extern "win64" fn clear_uav_float(this: PCtx, uav_ptr: *mut c_void, vals: *const [f32; 4]) {
    let Some(o) = Obj::<D3d11Uav>::from_iface(uav_ptr) else {
        log::warn!(target: "d3d", "d3d11 ClearUnorderedAccessViewFloat: null/foreign UAV {uav_ptr:?}");
        return;
    };
    let values = if vals.is_null() { [0.0; 4] } else { std::ptr::read_unaligned(vals) };
    Obj::get(this).emit(RecordedCmd::ClearUavFloat { uav: Obj::get(o).inner.clone(), values });
}

unsafe extern "win64" fn clear_view(
    this: PCtx,
    view: *mut c_void,
    color: *const [f32; 4],
    rects: *const Rect,
    num_rects: u32,
) {
    let c = if color.is_null() { [0.0; 4] } else { std::ptr::read_unaligned(color) };
    let mut rs = Vec::new();
    if num_rects > 0 && !rects.is_null() {
        rs.reserve(num_rects as usize);
        for i in 0..num_rects as usize {
            rs.push(std::ptr::read_unaligned(rects.add(i)));
        }
    }
    if let Some(o) = Obj::<D3d11Rtv>::from_iface(view) {
        Obj::get(this).emit(RecordedCmd::ClearRtv { rtv: Obj::get(o).inner.clone(), color: c, rects: rs });
    } else if let Some(o) = Obj::<D3d11Dsv>::from_iface(view) {
        Obj::get(this).emit(RecordedCmd::ClearDsv {
            dsv: Obj::get(o).inner.clone(),
            flags: clear_flag::DEPTH,
            depth: c[0],
            stencil: 0,
            rects: rs,
        });
    } else if let Some(o) = Obj::<D3d11Uav>::from_iface(view) {
        Obj::get(this).emit(RecordedCmd::ClearUavFloat { uav: Obj::get(o).inner.clone(), values: c });
    }
}

unsafe extern "win64" fn draw(this: PCtx, vertex_count: u32, start_vertex: u32) {
    let ctx = Obj::get(this);
    let st = Box::new(ctx.state.lock().unwrap().clone());
    ctx.emit(RecordedCmd::Draw { state: st, vertex_count, instance_count: 1, start_vertex, start_instance: 0 });
}

unsafe extern "win64" fn draw_instanced(
    this: PCtx,
    vertex_count: u32,
    instance_count: u32,
    start_vertex: u32,
    start_instance: u32,
) {
    let ctx = Obj::get(this);
    let st = Box::new(ctx.state.lock().unwrap().clone());
    ctx.emit(RecordedCmd::Draw { state: st, vertex_count, instance_count, start_vertex, start_instance });
}

unsafe extern "win64" fn draw_indexed(this: PCtx, index_count: u32, start_index: u32, base_vertex: i32) {
    let ctx = Obj::get(this);
    let st = Box::new(ctx.state.lock().unwrap().clone());
    ctx.emit(RecordedCmd::DrawIndexed {
        state: st,
        index_count,
        instance_count: 1,
        start_index,
        base_vertex,
        start_instance: 0,
    });
}

unsafe extern "win64" fn draw_indexed_instanced(
    this: PCtx,
    index_count: u32,
    instance_count: u32,
    start_index: u32,
    base_vertex: i32,
    start_instance: u32,
) {
    let ctx = Obj::get(this);
    let st = Box::new(ctx.state.lock().unwrap().clone());
    ctx.emit(RecordedCmd::DrawIndexed {
        state: st,
        index_count,
        instance_count,
        start_index,
        base_vertex,
        start_instance,
    });
}

unsafe extern "win64" fn dispatch(this: PCtx, x: u32, y: u32, z: u32) {
    let ctx = Obj::get(this);
    let st = Box::new(ctx.state.lock().unwrap().clone());
    ctx.emit(RecordedCmd::Dispatch { state: st, x, y, z });
}

unsafe extern "win64" fn draw_auto(_this: PCtx) {
    stub!("ID3D11DeviceContext::DrawAuto not implemented");
}
unsafe extern "win64" fn draw_instanced_indirect(this: PCtx, buf: *mut c_void, off: u32) {
    let Some(args) = buffer_from_ptr(buf) else {
        log::warn!(target: "d3d", "d3d11 DrawInstancedIndirect: null/foreign args buffer {buf:?}");
        return;
    };
    let ctx = Obj::get(this);
    let st = Box::new(ctx.state.lock().unwrap().clone());
    ctx.emit(RecordedCmd::DrawInstancedIndirect { state: st, args, offset: off });
}
unsafe extern "win64" fn draw_indexed_instanced_indirect(this: PCtx, buf: *mut c_void, off: u32) {
    let Some(args) = buffer_from_ptr(buf) else {
        log::warn!(target: "d3d", "d3d11 DrawIndexedInstancedIndirect: null/foreign args buffer {buf:?}");
        return;
    };
    let ctx = Obj::get(this);
    let st = Box::new(ctx.state.lock().unwrap().clone());
    ctx.emit(RecordedCmd::DrawIndexedInstancedIndirect { state: st, args, offset: off });
}
unsafe extern "win64" fn dispatch_indirect(this: PCtx, buf: *mut c_void, off: u32) {
    let Some(args) = buffer_from_ptr(buf) else {
        log::warn!(target: "d3d", "d3d11 DispatchIndirect: null/foreign args buffer {buf:?}");
        return;
    };
    let ctx = Obj::get(this);
    let st = Box::new(ctx.state.lock().unwrap().clone());
    ctx.emit(RecordedCmd::DispatchIndirect { state: st, args, offset: off });
}
unsafe extern "win64" fn copy_structure_count(this: PCtx, dst: *mut c_void, off: u32, uav: *mut c_void) {
    let Some(dst_buf) = buffer_from_ptr(dst) else {
        log::warn!(target: "d3d", "d3d11 CopyStructureCount: null/foreign dst buffer {dst:?}");
        return;
    };
    let Some(uav_obj) = Obj::<D3d11Uav>::from_iface(uav) else {
        log::warn!(target: "d3d", "d3d11 CopyStructureCount: null/foreign UAV {uav:?}");
        return;
    };
    Obj::get(this).emit(RecordedCmd::CopyStructureCount {
        dst: dst_buf,
        dst_offset: off,
        uav: Obj::get(uav_obj).inner.clone(),
    });
}
unsafe extern "win64" fn generate_mips(this: PCtx, srv: *mut c_void) {
    let Some(srv_obj) = Obj::<D3d11Srv>::from_iface(srv) else {
        log::warn!(target: "d3d", "d3d11 GenerateMips: null/foreign SRV {srv:?}");
        return;
    };
    Obj::get(this).emit(RecordedCmd::GenerateMips { srv: Obj::get(srv_obj).inner.clone() });
}

pub unsafe extern "win64" fn set_resource_min_lod(_this: PCtx, res: *mut c_void, min_lod: f32) {
    if let Some(r) = resource_from_ptr(res) {
        r.min_lod_bits.store(min_lod.to_bits(), std::sync::atomic::Ordering::Relaxed);
    }
}
crate::win64_bridge!(fn set_resource_min_lod(PCtx, *mut c_void, f32) -> ());

unsafe extern "win64" fn get_resource_min_lod(_this: PCtx, res: *mut c_void) -> f32 {
    resource_from_ptr(res)
        .map(|r| f32::from_bits(r.min_lod_bits.load(std::sync::atomic::Ordering::Relaxed)))
        .unwrap_or(0.0)
}

unsafe extern "win64" fn resolve_subresource(
    this: PCtx,
    dst: *mut c_void,
    dst_sub: u32,
    src: *mut c_void,
    src_sub: u32,
    format: u32,
) {
    let (Some(d), Some(s)) = (resource_from_ptr(dst), resource_from_ptr(src)) else {
        return;
    };
    Obj::get(this).emit(RecordedCmd::ResolveSubresource { dst: d, dst_sub, src: s, src_sub, format });
}

unsafe extern "win64" fn begin_async(this: PCtx, async_ptr: *mut c_void) {
    let Some(q) = query_from_ptr(async_ptr) else { return };
    Obj::get(this).emit(RecordedCmd::BeginQuery(q));
}

unsafe extern "win64" fn end_async(this: PCtx, async_ptr: *mut c_void) {
    let Some(q) = query_from_ptr(async_ptr) else { return };
    Obj::get(this).emit(RecordedCmd::EndQuery(q));
}

unsafe extern "win64" fn get_data(
    this: PCtx,
    async_ptr: *mut c_void,
    data: *mut c_void,
    size: u32,
    flags: u32,
) -> HRESULT {
    let ctx = Obj::get(this);
    if ctx.deferred {
        return DXGI_ERROR_INVALID_CALL;
    }
    let Some(q) = query_from_ptr(async_ptr) else {
        return E_INVALIDARG;
    };
    let need = q.data_size();
    if size != 0 && (data.is_null() || size < need) {
        return E_INVALIDARG;
    }
    let (ended, submitted, tv) = {
        let rt = q.rt.lock().unwrap();
        (rt.ended, rt.submitted, rt.timeline_value)
    };
    if !ended {
        return DXGI_ERROR_INVALID_CALL;
    }
    let target_tv = if !submitted {
        if flags & async_getdata_flag::DO_NOT_FLUSH != 0 {
            return S_FALSE;
        }
        ctx.device.flush_immediate()
    } else {
        tv
    };
    if ctx.device.queue.timeline.completed() < target_tv {
        if flags & async_getdata_flag::DO_NOT_FLUSH != 0 {
            return S_FALSE;
        }
        ctx.device.queue.timeline.wait_cpu(target_tv, u64::MAX);
    }
    if size > 0 && !data.is_null() {
        match q.desc.Query {
            query_kind::EVENT => {
                std::ptr::write_unaligned(data as *mut BOOL, TRUE);
            }
            query_kind::TIMESTAMP => {
                let ticks = q
                    .rt
                    .lock()
                    .unwrap()
                    .readback
                    .as_ref()
                    .and_then(|rb| rb.cpu_ptr().map(|p| std::ptr::read_unaligned(p as *const u64)))
                    .unwrap_or(0);
                std::ptr::write_unaligned(data as *mut u64, ticks);
            }
            query_kind::TIMESTAMP_DISJOINT => {
                std::ptr::write_unaligned(
                    data as *mut D3D11_QUERY_DATA_TIMESTAMP_DISJOINT,
                    D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {
                        Frequency: d3d12_metal::metal::query::timestamp_frequency(),
                        Disjoint: FALSE,
                    },
                );
            }
            query_kind::OCCLUSION => {
                let samples = q
                    .rt
                    .lock()
                    .unwrap()
                    .readback
                    .as_ref()
                    .and_then(|rb| rb.cpu_ptr().map(|p| std::ptr::read_unaligned(p as *const u64)))
                    .unwrap_or(0);
                std::ptr::write_unaligned(data as *mut u64, samples);
            }
            query_kind::OCCLUSION_PREDICATE => {
                let samples = q
                    .rt
                    .lock()
                    .unwrap()
                    .readback
                    .as_ref()
                    .and_then(|rb| rb.cpu_ptr().map(|p| std::ptr::read_unaligned(p as *const u64)))
                    .unwrap_or(0);
                std::ptr::write_unaligned(data as *mut BOOL, if samples != 0 { TRUE } else { FALSE });
            }
            query_kind::PIPELINE_STATISTICS => {
                stub!("GetData(PIPELINE_STATISTICS) returning zeroes");
                std::ptr::write_bytes(data as *mut u8, 0, need as usize);
            }
            _ => {
                std::ptr::write_bytes(data as *mut u8, 0, need as usize);
            }
        }
    }
    S_OK
}

unsafe extern "win64" fn clear_state(this: PCtx) {
    *Obj::get(this).state.lock().unwrap() = ContextState::default();
}

unsafe extern "win64" fn flush(this: PCtx) {
    let ctx = Obj::get(this);
    if !ctx.deferred {
        ctx.device.flush_immediate();
    }
}

unsafe extern "win64" fn get_type(this: PCtx) -> u32 {
    if Obj::get(this).deferred { context_type::DEFERRED } else { context_type::IMMEDIATE }
}

unsafe extern "win64" fn get_context_flags(this: PCtx) -> u32 {
    Obj::get(this).flags
}

unsafe extern "win64" fn finish_command_list(
    this: PCtx,
    restore_deferred_state: BOOL,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    let ctx = Obj::get(this);
    if !ctx.deferred {
        return d11_failed!(DXGI_ERROR_INVALID_CALL, "FinishCommandList called on immediate context");
    }
    if out.is_null() {
        return E_INVALIDARG;
    }
    let cmds = std::mem::take(&mut *ctx.recorded.lock().unwrap());
    if restore_deferred_state == 0 {
        *ctx.state.lock().unwrap() = ContextState::default();
    }
    let inner = Arc::new(CommandListInner {
        device: ctx.device.clone(),
        context_flags: ctx.flags,
        cmds,
    });
    std::ptr::write_unaligned(out, Obj::new(D3d11CommandList { inner }) as *mut c_void);
    S_OK
}

unsafe extern "win64" fn execute_command_list(
    this: PCtx,
    cl_ptr: *mut c_void,
    restore_context_state: BOOL,
) {
    let Some(cl) = cmdlist_from_ptr(cl_ptr) else {
        log::warn!(target: "d3d", "d3d11 ExecuteCommandList: null/foreign command list {cl_ptr:?}");
        return;
    };
    let ctx = Obj::get(this);
    let saved = if restore_context_state != 0 {
        Some(ctx.state.lock().unwrap().clone())
    } else {
        None
    };
    for cmd in &cl.cmds {
        ctx.emit(cmd.clone());
    }
    *ctx.state.lock().unwrap() = saved.unwrap_or_default();
}

unsafe extern "win64" fn discard_resource(this: PCtx, res: *mut c_void) {
    if let Some(r) = resource_from_ptr(res) {
        Obj::get(this).emit(RecordedCmd::DiscardResource(r));
    }
}

unsafe extern "win64" fn discard_view(this: PCtx, view: *mut c_void) {
    if let Some(o) = Obj::<D3d11Rtv>::from_iface(view) {
        Obj::get(this).emit(RecordedCmd::DiscardResource(Obj::get(o).inner.resource.clone()));
    } else if let Some(o) = Obj::<D3d11Dsv>::from_iface(view) {
        Obj::get(this).emit(RecordedCmd::DiscardResource(Obj::get(o).inner.resource.clone()));
    } else if let Some(o) = Obj::<D3d11Uav>::from_iface(view) {
        Obj::get(this).emit(RecordedCmd::DiscardResource(Obj::get(o).inner.resource.clone()));
    } else if let Some(o) = Obj::<D3d11Srv>::from_iface(view) {
        Obj::get(this).emit(RecordedCmd::DiscardResource(Obj::get(o).inner.resource.clone()));
    }
}

unsafe extern "win64" fn discard_view1(this: PCtx, view: *mut c_void, _rects: *const Rect, num_rects: u32) {
    if num_rects == 0 {
        discard_view(this, view);
    }
}

unsafe extern "win64" fn swap_device_context_state(_this: PCtx, _state: *mut c_void, prev: *mut *mut c_void) {
    if !prev.is_null() { std::ptr::write_unaligned(prev, std::ptr::null_mut()); }
    stub!("ID3D11DeviceContext1::SwapDeviceContextState not implemented");
}
