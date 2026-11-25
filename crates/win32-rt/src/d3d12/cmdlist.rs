//! ID3D12GraphicsCommandList10 over `d3d12_metal::metal::cmdlist::CommandList`.
//!
//! Methods capture their arguments (CPU descriptors are read at call time,
//! D3D12 semantics) and [`rec`] queues the Metal encoding for the command
//! stream thread ([`super::cs`]), which replays it into the list's
//! `MTL4CommandBuffer` in order. Root arguments and compute PSOs feed
//! `Dispatch`; graphics state is stored until the draw paths arrive with
//! graphics PSOs.

use super::com::*;
use super::cs;
use super::descheap::read_target;
use super::failed;
use super::queue;
use super::resource;
use super::tr::{self, tr};
use super::vtables::*;
use crate::types::*;
use d3d12_metal::metal::cmdlist::CommandList as MtlList;
use d3d12_metal::metal::queue::ListType;
use d3d12_metal::types::{Rect, ResourceBarrier};
use std::ffi::c_void;
use d3d12_metal::metal::cmdlist::CommandAllocator as MtlAllocator;
use std::sync::{Arc, Mutex, OnceLock};

pub use d3d12_metal::types::Viewport;

/// COM-level list state (draw state lives in the Metal list).
#[derive(Default)]
pub struct GfxState {
    /// `ID3D12DescriptorHeap*` (not owned, D3D12 semantics).
    pub heaps: Vec<usize>,
}

pub struct ListState {
    pub l: MtlList,
    pub gfx: GfxState,
}

pub struct CommandList {
    pub kind: ListType,
    /// Metal-side state, touched only by replayed commands (CS thread).
    pub state: Mutex<ListState>,
    /// Commands recorded since the last batch handed to the CS thread.
    pending: Mutex<cs::CmdBuf<ListState>>,
}

static CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for CommandList {
    fn class() -> &'static Class {
        class_once(&CLASS, || Class {
            name: "ID3D12GraphicsCommandList10",
            vtbl: build_named::<CommandList>(
                "ID3D12GraphicsCommandList10",
                ID3D12GraphicsCommandList10,
                &[
                    m!("SetName" => set_name_list),
                    m!("GetType" => get_type),
                    m!("Close" => close),
                    m!("Reset" => reset),
                    m!("ClearState" => clear_state),
                    m!("ResourceBarrier" => resource_barrier),
                    m!("ClearRenderTargetView" => clear_render_target_view),
                    m!("ClearDepthStencilView" => crate::bridge_entry!(clear_depth_stencil_view)),
                    m!("OMSetRenderTargets" => om_set_render_targets),
                    m!("CopyBufferRegion" => copy_buffer_region),
                    m!("CopyTextureRegion" => copy_texture_region),
                    m!("CopyResource" => copy_resource),
                    m!("RSSetViewports" => rs_set_viewports),
                    m!("RSSetScissorRects" => rs_set_scissor_rects),
                    m!("IASetPrimitiveTopology" => ia_set_primitive_topology),
                    m!("OMSetBlendFactor" => om_set_blend_factor),
                    m!("OMSetStencilRef" => om_set_stencil_ref),
                    m!("OMSetFrontAndBackStencilRef" => om_set_front_and_back_stencil_ref),
                    m!("SetDescriptorHeaps" => set_descriptor_heaps),
                    m!("SetPipelineState" => set_pipeline_state),
                    m!("SetComputeRootSignature" => set_compute_root_signature),
                    m!("SetGraphicsRootSignature" => set_graphics_root_signature),
                    m!("SetComputeRootDescriptorTable" => set_compute_root_descriptor_table),
                    m!("SetGraphicsRootDescriptorTable" => set_graphics_root_descriptor_table),
                    m!("SetComputeRoot32BitConstant" => set_compute_root_constant),
                    m!("SetGraphicsRoot32BitConstant" => set_graphics_root_constant),
                    m!("SetComputeRoot32BitConstants" => set_compute_root_constants),
                    m!("SetGraphicsRoot32BitConstants" => set_graphics_root_constants),
                    m!("SetComputeRootConstantBufferView" => set_compute_root_cbv),
                    m!("SetGraphicsRootConstantBufferView" => set_graphics_root_cbv),
                    m!("SetComputeRootShaderResourceView" => set_compute_root_srv),
                    m!("SetGraphicsRootShaderResourceView" => set_graphics_root_srv),
                    m!("SetComputeRootUnorderedAccessView" => set_compute_root_uav),
                    m!("SetGraphicsRootUnorderedAccessView" => set_graphics_root_uav),
                    m!("Dispatch" => dispatch),
                    m!("DrawInstanced" => draw_instanced),
                    m!("DrawIndexedInstanced" => draw_indexed_instanced),
                    m!("ExecuteIndirect" => execute_indirect),
                    m!("ClearUnorderedAccessViewUint" => clear_uav_uint),
                    m!("ClearUnorderedAccessViewFloat" => clear_uav_float),
                    m!("DiscardResource" => discard_resource),
                    m!("ResolveSubresource" => resolve_subresource),
                    m!("DispatchMesh" => dispatch_mesh),
                    m!("IASetVertexBuffers" => ia_set_vertex_buffers),
                    m!("IASetIndexBuffer" => ia_set_index_buffer),
                    m!("OMSetDepthBounds" => crate::bridge_entry!(om_set_depth_bounds)),
                    m!("BeginQuery" => begin_query),
                    m!("EndQuery" => end_query),
                    m!("ResolveQueryData" => resolve_query_data),
                    m!("SetPredication" => set_predication),
                    m!("SetMarker" => set_marker),
                    m!("BeginEvent" => begin_event),
                    m!("EndEvent" => end_event),
                    m!("BuildRaytracingAccelerationStructure" => build_raytracing_acceleration_structure),
                    m!("EmitRaytracingAccelerationStructurePostbuildInfo" => emit_raytracing_acceleration_structure_postbuild_info),
                    m!("CopyRaytracingAccelerationStructure" => copy_raytracing_acceleration_structure),
                    m!("SetPipelineState1" => set_pipeline_state1),
                    m!("DispatchRays" => dispatch_rays),
                ],
            ),
            iids: &[
                IID_ID3D12Object,
                IID_ID3D12DeviceChild,
                IID_ID3D12CommandList,
                IID_ID3D12GraphicsCommandList,
                IID_ID3D12GraphicsCommandList1,
                IID_ID3D12GraphicsCommandList2,
                IID_ID3D12GraphicsCommandList3,
                IID_ID3D12GraphicsCommandList4,
                IID_ID3D12GraphicsCommandList5,
                IID_ID3D12GraphicsCommandList6,
                IID_ID3D12GraphicsCommandList7,
                IID_ID3D12GraphicsCommandList8,
                IID_ID3D12GraphicsCommandList9,
                IID_ID3D12GraphicsCommandList10,
            ],
        })
    }
}

pub type PL = *mut Obj<CommandList>;

pub unsafe fn from_ptr<'a>(p: *mut c_void) -> Option<&'a CommandList> {
    Obj::<CommandList>::from_iface(p).map(|o| &(*o).inner)
}

unsafe fn new_list(ty: u32, alloc: Option<*mut c_void>, pso: *mut c_void, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    let Some(kind) = ListType::from_raw(ty) else { return failed!(E_INVALIDARG, "CreateCommandList: type {ty}") };
    if out.is_null() {
        return S_FALSE;
    }
    *out = std::ptr::null_mut();
    if kind == ListType::Bundle {
        return failed!(E_NOTIMPL, "CreateCommandList: bundles are not implemented");
    }
    let Some(l) = MtlList::new(kind) else { return failed!(E_OUTOFMEMORY, "CreateCommandList({kind:?}): newCommandBuffer failed") };
    let alloc = match alloc {
        Some(a) => match queue::allocator_from_ptr(a) {
            Some(a) => Some(cs::SendPtr(&a.a as *const MtlAllocator)),
            None => return failed!(E_INVALIDARG, "CreateCommandList: foreign allocator {a:?}"),
        },
        None => None,
    };
    let ls = ListState { l, gfx: GfxState::default() };
    let o = Obj::new(CommandList { kind, state: Mutex::new(ls), pending: Mutex::new(cs::CmdBuf::default()) });
    if let Some(a) = alloc {
        // Ordered after any pending allocator Reset on the CS thread.
        let p = pso_cmd(pso);
        rec(o, move |s| {
            if let Err(e) = s.l.begin(&*a.0) {
                log::warn!(target: "d3d", "CreateCommandList: {e}");
            }
            apply_pso(s, p);
        });
    }
    let hr = query_out(o as *mut c_void, riid, out);
    Obj::release(o);
    hr
}

/// `CreateCommandList(node, type, allocator, initial PSO)`: open.
pub unsafe fn create(ty: u32, alloc: *mut c_void, pso: *mut c_void, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    new_list(ty, Some(alloc), pso, riid, out)
}

/// `CreateCommandList1(node, type, flags)`: closed.
pub unsafe fn create1(ty: u32, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    new_list(ty, None, std::ptr::null_mut(), riid, out)
}

unsafe fn st<'a>(this: PL) -> std::sync::MutexGuard<'a, ListState> {
    Obj::get(this).state.lock().unwrap()
}

/// Queue `f` (the Metal side of a command) for the CS thread, or run it now
/// with `AW2_D3D_CS=0`.
#[inline]
unsafe fn rec<F: FnOnce(&mut ListState) + 'static>(this: PL, f: F) {
    if !cs::enabled() {
        f(&mut st(this));
        return;
    }
    let mut p = Obj::get(this).pending.lock().unwrap();
    p.push(f);
    if p.len() >= cs::BATCH {
        send(this, &mut p);
    }
}

fn buf_pool() -> &'static Mutex<Vec<cs::CmdBuf<ListState>>> {
    static P: OnceLock<Mutex<Vec<cs::CmdBuf<ListState>>>> = OnceLock::new();
    P.get_or_init(Default::default)
}

/// Hand the recorded commands to the CS thread (the list stays alive until
/// they have been replayed).
unsafe fn send(this: PL, p: &mut cs::CmdBuf<ListState>) {
    if p.is_empty() {
        return;
    }
    let fresh = buf_pool().lock().unwrap().pop().unwrap_or_default();
    let mut batch = std::mem::replace(p, fresh);
    Obj::add_ref(this);
    let lp = cs::SendPtr(this as *const Obj<CommandList>);
    cs::submit(move || {
        let this = lp.get() as PL;
        batch.run(&mut st(this));
        buf_pool().lock().unwrap().push(batch);
        Obj::release(this);
    });
}

/// A pipeline state to bind (resolved at call time).
enum PsoCmd {
    Compute(Arc<d3d12_metal::metal::pipeline::ComputePipeline>),
    Graphics(Arc<d3d12_metal::metal::pipeline::GraphicsPipeline>),
    Mesh(Arc<d3d12_metal::mesh::MeshPipeline>),
}

unsafe fn pso_cmd(pso: *mut c_void) -> Option<PsoCmd> {
    if pso.is_null() {
        return None;
    }
    let Some(p) = super::pso::pso_from_ptr(pso) else {
        log::error!(target: "d3d", "SetPipelineState: foreign PSO {pso:?}");
        return None;
    };
    Some(match &p.kind {
        super::pso::PipelineKind::Compute(c) => PsoCmd::Compute(c.clone()),
        super::pso::PipelineKind::Graphics(g) => PsoCmd::Graphics(g.clone()),
        super::pso::PipelineKind::Mesh(m) => PsoCmd::Mesh(m.clone()),
    })
}

/// Apply a PSO to the list (`SetPipelineState`, initial PSO of
/// `Reset`/`CreateCommandList`).
fn apply_pso(s: &mut ListState, p: Option<PsoCmd>) {
    match p {
        None => {}
        Some(PsoCmd::Compute(c)) => s.l.set_compute_pipeline(c),
        Some(PsoCmd::Graphics(g)) => s.l.set_graphics_pipeline(g),
        Some(PsoCmd::Mesh(m)) => s.l.set_mesh_pipeline(m),
    }
}

type Res = d3d12_metal::metal::resource::Resource;

/// The Metal resource behind an `ID3D12Resource*` (lifetime: D3D12 rules).
unsafe fn res_ptr(p: *mut c_void) -> Option<cs::SendPtr<Res>> {
    resource::from_ptr(p).map(|d| cs::SendPtr(Arc::as_ptr(&d.r)))
}

unsafe extern "win64" fn set_name_list(this: PL, name: *const u16) -> HRESULT {
    let label = read_wstr(name);
    rec(this, move |s| s.l.label = label);
    set_name::<CommandList>(this, name)
}

unsafe extern "win64" fn get_type(this: PL) -> u32 {
    Obj::get(this).kind as u32
}

unsafe extern "win64" fn close(this: PL) -> HRESULT {
    tr!(Call::Close { list: this as u64 });
    if !cs::enabled() {
        return match st(this).l.close() {
            Ok(()) => S_OK,
            Err(e) => failed!(E_FAIL, "ID3D12GraphicsCommandList::Close: {e}"),
        };
    }
    rec(this, |s| {
        if let Err(e) = s.l.close() {
            log::warn!(target: "d3d", "ID3D12GraphicsCommandList::Close: {e}");
        }
    });
    send(this, &mut Obj::get(this).pending.lock().unwrap());
    S_OK
}

unsafe extern "win64" fn reset(this: PL, alloc: *mut c_void, pso: *mut c_void) -> HRESULT {
    tr!(Call::Reset { list: this as u64, allocator: alloc as u64, pso: pso as u64 });
    let Some(a) = queue::allocator_from_ptr(alloc) else { return failed!(E_INVALIDARG, "ID3D12GraphicsCommandList::Reset: foreign allocator {alloc:?}") };
    let a = cs::SendPtr(&a.a as *const MtlAllocator);
    let p = pso_cmd(pso);
    rec(this, move |s| {
        s.gfx.heaps.clear();
        if let Err(e) = s.l.begin(&*a.0) {
            log::warn!(target: "d3d", "ID3D12GraphicsCommandList::Reset: {e}");
            return;
        }
        apply_pso(s, p);
    });
    S_OK
}

unsafe extern "win64" fn clear_state(this: PL, pso: *mut c_void) {
    tr!(Call::ClearState { list: this as u64, pso: pso as u64 });
    let p = pso_cmd(pso);
    rec(this, move |s| {
        s.gfx.heaps.clear();
        s.l.clear_state();
        apply_pso(s, p);
    });
}

unsafe extern "win64" fn resource_barrier(this: PL, n: u32, b: *const ResourceBarrier) {
    tr!(Call::ResourceBarrier { list: this as u64, barriers: tr::raw_n(b, n as usize) });
    if n == 0 || b.is_null() {
        return;
    }
    use d3d12_metal::metal::cmdlist::Barrier;
    let res_id = |p: u64| -> usize {
        if p == 0 {
            return 0;
        }
        match resource::from_ptr(p as *mut c_void) {
            Some(d) => {
                let r: &d3d12_metal::metal::resource::Resource = &d.r;
                r as *const _ as usize
            }
            None => 0,
        }
    };
    let parse = |rb: ResourceBarrier| -> Option<Barrier> {
        // D3D12_RESOURCE_BARRIER_FLAG_BEGIN_ONLY: the split barrier completes later.
        if rb.Flags & 1 != 0 {
            return None;
        }
        Some(match rb.Type {
            // u[1] = Subresource | StateBefore << 32; u[2] low half = StateAfter.
            0 => Barrier::Transition(res_id(rb.u[0]), (rb.u[1] >> 32) as u32, rb.u[2] as u32),
            2 => Barrier::Uav(res_id(rb.u[0])),
            _ => Barrier::Full,
        })
    };
    let n = n as usize;
    if n <= 8 {
        let mut inline = [Barrier::Full; 8];
        let mut count = 0usize;
        for i in 0..n {
            if let Some(item) = parse(std::ptr::read_unaligned(b.add(i))) {
                inline[count] = item;
                count += 1;
            }
        }
        if count == 1 {
            let one = inline[0];
            rec(this, move |s| s.l.barrier_list(std::slice::from_ref(&one)));
        } else if count > 1 {
            let len = count as u8;
            rec(this, move |s| s.l.barrier_list(&inline[..len as usize]));
        }
    } else {
        let mut items: Vec<Barrier> = Vec::with_capacity(n);
        for i in 0..n {
            if let Some(item) = parse(std::ptr::read_unaligned(b.add(i))) {
                items.push(item);
            }
        }
        if !items.is_empty() {
            rec(this, move |s| s.l.barrier_list(&items));
        }
    }
}

unsafe extern "win64" fn clear_render_target_view(this: PL, rtv: usize, color: *const [f32; 4], n: u32, rects: *const Rect) {
    let c = if color.is_null() { [0.0; 4] } else { std::ptr::read_unaligned(color) };
    tr!(Call::ClearRenderTargetView { list: this as u64, rtv: rtv as u64, color: c, rects: tr::raw_n(rects, n as usize) });
    let v = read_target(rtv);
    if v.resource == 0 {
        log::warn!(target: "d3d", "ClearRenderTargetView: null RTV {rtv:#x}");
        return;
    }
    if n == 0 || rects.is_null() {
        rec(this, move |s| {
            if let Err(e) = s.l.clear_rtv(&v, c, &[]) {
                log::warn!(target: "d3d", "ClearRenderTargetView: {e}");
            }
        });
    } else if n <= 4 {
        let mut inline = [Rect::default(); 4];
        let len = n as u8;
        for i in 0..n as usize {
            inline[i] = std::ptr::read_unaligned(rects.add(i));
        }
        rec(this, move |s| {
            if let Err(e) = s.l.clear_rtv(&v, c, &inline[..len as usize]) {
                log::warn!(target: "d3d", "ClearRenderTargetView: {e}");
            }
        });
    } else {
        let mut rs = Vec::with_capacity(n as usize);
        for i in 0..n as usize {
            rs.push(std::ptr::read_unaligned(rects.add(i)));
        }
        rec(this, move |s| {
            if let Err(e) = s.l.clear_rtv(&v, c, &rs) {
                log::warn!(target: "d3d", "ClearRenderTargetView: {e}");
            }
        });
    }
}

unsafe extern "win64" fn clear_depth_stencil_view(this: PL, dsv: usize, flags: u32, depth: f32, stencil: u8, n: u32, rects: *const Rect) {
    tr!(Call::ClearDepthStencilView { list: this as u64, dsv: dsv as u64, flags, depth, stencil, rects: tr::raw_n(rects, n as usize) });
    let v = read_target(dsv);
    if v.resource == 0 {
        log::warn!(target: "d3d", "ClearDepthStencilView: null DSV {dsv:#x}");
        return;
    }
    let clear_depth = flags & 1 != 0;
    let clear_stencil = flags & 2 != 0;
    if n == 0 || rects.is_null() {
        rec(this, move |s| {
            if let Err(e) = s.l.clear_dsv(&v, clear_depth, depth, clear_stencil, stencil, &[]) {
                log::warn!(target: "d3d", "ClearDepthStencilView: {e}");
            }
        });
    } else if n <= 4 {
        let mut inline = [Rect::default(); 4];
        let len = n as u8;
        for i in 0..n as usize {
            inline[i] = std::ptr::read_unaligned(rects.add(i));
        }
        rec(this, move |s| {
            if let Err(e) = s.l.clear_dsv(&v, clear_depth, depth, clear_stencil, stencil, &inline[..len as usize]) {
                log::warn!(target: "d3d", "ClearDepthStencilView: {e}");
            }
        });
    } else {
        let mut rs = Vec::with_capacity(n as usize);
        for i in 0..n as usize {
            rs.push(std::ptr::read_unaligned(rects.add(i)));
        }
        rec(this, move |s| {
            if let Err(e) = s.l.clear_dsv(&v, clear_depth, depth, clear_stencil, stencil, &rs) {
                log::warn!(target: "d3d", "ClearDepthStencilView: {e}");
            }
        });
    }
}

// ClearDepthStencilView takes `depth: f32` in Win64 slot 4 (xmm3) between
// integer slots. On aarch64 a plain signature read depth from xmm0, and read
// stencil/NumRects/pRects one slot early, so pRects became
// `NumRects | stack_junk << 32` (e.g. 0x7d00000000) and `clear_dsv` faulted.
// The bridge entry reads every slot positionally (see `crate::bridge`).
crate::win64_bridge!(fn clear_depth_stencil_view(PL, usize, u32, f32, u8, u32, *const Rect) -> ());

/// Descriptors are read at call time (D3D12 semantics).
unsafe extern "win64" fn om_set_render_targets(this: PL, n: u32, rtvs: *const usize, single_range: BOOL, dsv: *const usize) {
    let n = if rtvs.is_null() { 0 } else { n as usize };
    let mut v = Vec::with_capacity(n);
    let base_rtv = if n > 0 && single_range != 0 { std::ptr::read_unaligned(rtvs) } else { 0 };
    for i in 0..n {
        let h = if single_range != 0 {
            base_rtv + i * d3d12_metal::metal::descriptor::TARGET_DESCRIPTOR_SIZE as usize
        } else {
            std::ptr::read_unaligned(rtvs.add(i))
        };
        v.push(read_target(h));
    }
    let dsv_h = if dsv.is_null() { 0 } else { std::ptr::read_unaligned(dsv) };
    let d = if dsv.is_null() { None } else { Some(read_target(dsv_h)) };
    tr!(
        let hs: Vec<u64> = (0..n)
            .map(|i| {
                if single_range != 0 {
                    (base_rtv + i * d3d12_metal::metal::descriptor::TARGET_DESCRIPTOR_SIZE as usize) as u64
                } else {
                    std::ptr::read_unaligned(rtvs.add(i)) as u64
                }
            })
            .collect();
        Call::OMSetRenderTargets { list: this as u64, rtvs: U64s::from_slice(&hs), dsv: dsv_h as u64 }
    );
    rec(this, move |s| s.l.set_render_targets(v, d));
}

unsafe extern "win64" fn copy_buffer_region(this: PL, dst: *mut c_void, dst_off: u64, src: *mut c_void, src_off: u64, bytes: u64) {
    tr!(Call::CopyBufferRegion { list: this as u64, dst: dst as u64, dst_offset: dst_off, src: src as u64, src_offset: src_off, size: bytes });
    let (Some(d), Some(s)) = (res_ptr(dst), res_ptr(src)) else {
        log::error!(target: "d3d", "CopyBufferRegion: foreign resource ({dst:?}, {src:?})");
        return;
    };
    rec(this, move |l| {
        if let Err(e) = l.l.copy_buffer(&*d.0, dst_off, &*s.0, src_off, bytes) {
            log::warn!(target: "d3d", "CopyBufferRegion: {e}");
        }
    });
}

unsafe extern "win64" fn rs_set_viewports(this: PL, n: u32, v: *const Viewport) {
    tr!(Call::RSSetViewports { list: this as u64, viewports: tr::raw_n(v, n as usize) });
    if n == 0 || v.is_null() {
        rec(this, |s| s.l.set_viewports_d3d(&[]));
        return;
    }
    let n = n as usize;
    if n <= 2 {
        let mut inline = [Viewport::default(); 2];
        for i in 0..n {
            inline[i] = std::ptr::read_unaligned(v.add(i));
        }
        let len = n as u8;
        rec(this, move |s| s.l.set_viewports_d3d(&inline[..len as usize]));
    } else {
        let mut vs = Vec::with_capacity(n);
        for i in 0..n {
            vs.push(std::ptr::read_unaligned(v.add(i)));
        }
        rec(this, move |s| s.l.set_viewports_d3d(&vs));
    }
}

unsafe extern "win64" fn rs_set_scissor_rects(this: PL, n: u32, r: *const Rect) {
    tr!(Call::RSSetScissorRects { list: this as u64, rects: tr::raw_n(r, n as usize) });
    let rs = if n == 0 || r.is_null() {
        Vec::new()
    } else {
        let mut out = Vec::with_capacity(n as usize);
        for i in 0..n as usize {
            out.push(std::ptr::read_unaligned(r.add(i)));
        }
        out
    };
    rec(this, move |s| s.l.set_scissors(rs));
}

unsafe extern "win64" fn ia_set_primitive_topology(this: PL, t: u32) {
    tr!(Call::IASetPrimitiveTopology { list: this as u64, topology: t });
    rec(this, move |s| s.l.gfx.topology = t);
}

unsafe extern "win64" fn om_set_blend_factor(this: PL, f: *const [f32; 4]) {
    let f = if f.is_null() { [1.0; 4] } else { std::ptr::read_unaligned(f) };
    tr!(Call::OMSetBlendFactor { list: this as u64, factor: f });
    rec(this, move |s| s.l.set_blend_factor(f));
}

unsafe extern "win64" fn om_set_stencil_ref(this: PL, r: u32) {
    tr!(Call::OMSetStencilRef { list: this as u64, stencil_ref: r });
    rec(this, move |s| s.l.set_stencil_ref(r));
}

unsafe extern "win64" fn om_set_front_and_back_stencil_ref(this: PL, front: u32, back: u32) {
    tr!(Call::OMSetFrontAndBackStencilRef { list: this as u64, front, back });
    rec(this, move |s| s.l.set_front_and_back_stencil_ref(front, back));
}

unsafe extern "win64" fn set_predication(this: PL, buffer: *mut c_void, aligned_offset: u64, op: u32) {
    tr!(Call::SetPredication { list: this as u64, buffer: buffer as u64, aligned_offset, operation: op });
    if buffer.is_null() {
        rec(this, |s| s.l.set_predication(None));
        return;
    }
    let Some(r) = resource::from_ptr(buffer) else {
        log::error!(target: "d3d", "SetPredication: foreign resource {buffer:?}");
        return;
    };
    let va = r.r.gpu_va + aligned_offset;
    rec(this, move |s| s.l.set_predication(Some((va, op))));
}

unsafe extern "win64" fn set_descriptor_heaps(this: PL, n: u32, heaps: *const *mut c_void) {
    let n = if heaps.is_null() { 0 } else { n as usize };
    let resolve = |hs: &[usize]| -> [u64; 2] {
        let mut addr = [0u64; 2];
        for &h in hs {
            match super::descheap::from_ptr(h as *mut c_void) {
                Some(d) if d.h.shader_visible && (d.h.kind as usize) < 2 => addr[d.h.kind as usize] = d.h.gpu_base,
                Some(d) => log::warn!(target: "d3d", "SetDescriptorHeaps: heap {h:#x} (type {}, visible {}) is not a shader-visible CBV/SRV/UAV or sampler heap", d.h.kind, d.h.shader_visible),
                None => log::error!(target: "d3d", "SetDescriptorHeaps: foreign heap {h:#x}"),
            }
        }
        addr
    };
    if n <= 2 {
        let mut inline = [0usize; 2];
        for i in 0..n {
            inline[i] = std::ptr::read_unaligned(heaps.add(i)) as usize;
        }
        tr!(
            let ids = [inline[0] as u64, inline[1] as u64];
            Call::SetDescriptorHeaps { list: this as u64, heaps: U64s::from_slice(&ids[..n]) }
        );
        let addr = resolve(&inline[..n]);
        let len = n as u8;
        rec(this, move |s| {
            s.l.set_descriptor_heaps(addr[0], addr[1]);
            s.gfx.heaps.clear();
            s.gfx.heaps.extend_from_slice(&inline[..len as usize]);
        });
    } else {
        let mut hs = Vec::with_capacity(n);
        for i in 0..n {
            hs.push(std::ptr::read_unaligned(heaps.add(i)) as usize);
        }
        tr!(
            let ids: Vec<u64> = hs.iter().map(|&h| h as u64).collect();
            Call::SetDescriptorHeaps { list: this as u64, heaps: U64s::from_slice(&ids) }
        );
        let addr = resolve(&hs);
        rec(this, move |s| {
            s.l.set_descriptor_heaps(addr[0], addr[1]);
            s.gfx.heaps = hs;
        });
    }
}


unsafe extern "win64" fn set_pipeline_state(this: PL, pso: *mut c_void) {
    tr!(Call::SetPipelineState { list: this as u64, pso: pso as u64 });
    let p = pso_cmd(pso);
    rec(this, move |s| apply_pso(s, p));
}

fn root_state(s: &mut ListState, compute: bool) -> &mut d3d12_metal::metal::binding::RootState {
    if compute {
        &mut s.l.compute_root
    } else {
        &mut s.l.gfx_root
    }
}

fn root_result(what: &str, compute: bool, r: Result<(), String>) {
    if let Err(e) = r {
        log::warn!(target: "d3d", "Set{}{what}: {e}", if compute { "ComputeRoot" } else { "GraphicsRoot" });
    }
}

unsafe fn set_root_signature(this: PL, rs: *mut c_void, compute: bool) {
    tr!(if compute { Call::SetComputeRootSignature { list: this as u64, root_signature: rs as u64 } } else { Call::SetGraphicsRootSignature { list: this as u64, root_signature: rs as u64 } });
    let Some(r) = super::pso::root_signature_from_ptr(rs) else {
        log::error!(target: "d3d", "Set{}RootSignature: foreign/null root signature {rs:?}", if compute { "Compute" } else { "Graphics" });
        return;
    };
    let rs = r.rs.clone();
    rec(this, move |s| root_result("Signature", compute, root_state(s, compute).set_root_signature(rs)));
}

unsafe extern "win64" fn set_compute_root_signature(this: PL, rs: *mut c_void) {
    set_root_signature(this, rs, true);
}
unsafe extern "win64" fn set_graphics_root_signature(this: PL, rs: *mut c_void) {
    set_root_signature(this, rs, false);
}

unsafe fn set_table(this: PL, i: u32, h: u64, compute: bool) {
    tr!(Call::SetRootDescriptorTable { list: this as u64, compute, param: i, gpu_handle: h });
    rec(this, move |s| root_result("DescriptorTable", compute, root_state(s, compute).set_table(i, h)));
}
unsafe extern "win64" fn set_compute_root_descriptor_table(this: PL, i: u32, h: u64) {
    set_table(this, i, h, true);
}
unsafe extern "win64" fn set_graphics_root_descriptor_table(this: PL, i: u32, h: u64) {
    set_table(this, i, h, false);
}

unsafe fn set_constants_slice(this: PL, i: u32, values: &[u32], dest: u32, compute: bool) {
    // Up to 64 root DWORDs (D3D12 limit) are captured inline, sized to the
    // common small counts so 1..4 DWORD calls don't copy 256 bytes into CmdBuf.
    let n = values.len().min(64);
    match n {
        0 => rec(this, move |s| {
            root_result("32BitConstants", compute, root_state(s, compute).set_constants(i, dest, &[]));
        }),
        1 => {
            let v0 = values[0];
            rec(this, move |s| {
                root_result(
                    "32BitConstants",
                    compute,
                    root_state(s, compute).set_constants(i, dest, std::slice::from_ref(&v0)),
                );
            });
        }
        2..=4 => {
            let mut buf = [0u32; 4];
            buf[..n].copy_from_slice(&values[..n]);
            let len = n as u8;
            rec(this, move |s| {
                root_result("32BitConstants", compute, root_state(s, compute).set_constants(i, dest, &buf[..len as usize]));
            });
        }
        5..=16 => {
            let mut buf = [0u32; 16];
            buf[..n].copy_from_slice(&values[..n]);
            let len = n as u8;
            rec(this, move |s| {
                root_result("32BitConstants", compute, root_state(s, compute).set_constants(i, dest, &buf[..len as usize]));
            });
        }
        _ => {
            let mut buf = [0u32; 64];
            buf[..n].copy_from_slice(&values[..n]);
            let len = n as u8;
            rec(this, move |s| {
                root_result("32BitConstants", compute, root_state(s, compute).set_constants(i, dest, &buf[..len as usize]));
            });
        }
    }
}
unsafe fn set_constants_ptr(this: PL, i: u32, n: u32, data: *const u32, dest: u32, compute: bool) {
    if n == 0 || data.is_null() {
        tr!(Call::SetRoot32BitConstants { list: this as u64, compute, param: i, dest_offset: dest, values: U32s::from_slice(&[]) });
        set_constants_slice(this, i, &[], dest, compute);
        return;
    }
    let total = n as usize;
    if total <= 4 {
        let mut buf = [0u32; 4];
        for k in 0..total {
            buf[k] = std::ptr::read_unaligned(data.add(k));
        }
        tr!(Call::SetRoot32BitConstants { list: this as u64, compute, param: i, dest_offset: dest, values: U32s::from_slice(&buf[..total]) });
        set_constants_slice(this, i, &buf[..total], dest, compute);
    } else if total <= 16 {
        let mut buf = [0u32; 16];
        for k in 0..total {
            buf[k] = std::ptr::read_unaligned(data.add(k));
        }
        tr!(Call::SetRoot32BitConstants { list: this as u64, compute, param: i, dest_offset: dest, values: U32s::from_slice(&buf[..total]) });
        set_constants_slice(this, i, &buf[..total], dest, compute);
    } else {
        let clamped = total.min(64);
        let mut buf = [0u32; 64];
        for k in 0..clamped {
            buf[k] = std::ptr::read_unaligned(data.add(k));
        }
        tr!(
            let extra: Vec<u32> = if total > 64 {
                (0..total).map(|k| std::ptr::read_unaligned(data.add(k))).collect()
            } else {
                Vec::new()
            };
            let slice: &[u32] = if total <= 64 { &buf[..total] } else { &extra };
            Call::SetRoot32BitConstants { list: this as u64, compute, param: i, dest_offset: dest, values: U32s::from_slice(slice) }
        );
        if total > 64 {
            log::warn!(target: "d3d", "SetRoot32BitConstants: {total} values (max 64)");
        }
        set_constants_slice(this, i, &buf[..clamped], dest, compute);
    }
}
unsafe extern "win64" fn set_compute_root_constant(this: PL, i: u32, v: u32, dest: u32) {
    tr!(Call::SetRoot32BitConstants { list: this as u64, compute: true, param: i, dest_offset: dest, values: U32s::from_slice(std::slice::from_ref(&v)) });
    set_constants_slice(this, i, std::slice::from_ref(&v), dest, true);
}
unsafe extern "win64" fn set_graphics_root_constant(this: PL, i: u32, v: u32, dest: u32) {
    tr!(Call::SetRoot32BitConstants { list: this as u64, compute: false, param: i, dest_offset: dest, values: U32s::from_slice(std::slice::from_ref(&v)) });
    set_constants_slice(this, i, std::slice::from_ref(&v), dest, false);
}
unsafe extern "win64" fn set_compute_root_constants(this: PL, i: u32, n: u32, data: *const u32, dest: u32) {
    set_constants_ptr(this, i, n, data, dest, true);
}
unsafe extern "win64" fn set_graphics_root_constants(this: PL, i: u32, n: u32, data: *const u32, dest: u32) {
    set_constants_ptr(this, i, n, data, dest, false);
}

/// `kind`: 0 = CBV, 1 = SRV, 2 = UAV (trace only; the root signature
/// already knows the parameter type).
unsafe fn set_root_descriptor(this: PL, i: u32, va: u64, compute: bool, kind: u8) {
    tr!(Call::SetRootView { list: this as u64, compute, param: i, kind, gpu_va: va });
    rec(this, move |s| root_result("Descriptor", compute, root_state(s, compute).set_descriptor(i, va)));
}
unsafe extern "win64" fn set_compute_root_cbv(this: PL, i: u32, va: u64) {
    set_root_descriptor(this, i, va, true, 0);
}
unsafe extern "win64" fn set_graphics_root_cbv(this: PL, i: u32, va: u64) {
    set_root_descriptor(this, i, va, false, 0);
}
unsafe extern "win64" fn set_compute_root_srv(this: PL, i: u32, va: u64) {
    set_root_descriptor(this, i, va, true, 1);
}
unsafe extern "win64" fn set_graphics_root_srv(this: PL, i: u32, va: u64) {
    set_root_descriptor(this, i, va, false, 1);
}
unsafe extern "win64" fn set_compute_root_uav(this: PL, i: u32, va: u64) {
    set_root_descriptor(this, i, va, true, 2);
}
unsafe extern "win64" fn set_graphics_root_uav(this: PL, i: u32, va: u64) {
    set_root_descriptor(this, i, va, false, 2);
}

unsafe extern "win64" fn dispatch(this: PL, x: u32, y: u32, z: u32) {
    tr!(Call::Dispatch { list: this as u64, x, y, z });
    rec(this, move |s| {
        if let Err(e) = s.l.dispatch(x, y, z) {
            log::warn!(target: "d3d", "Dispatch({x}, {y}, {z}): {e}");
        }
    });
}

/// PIX markers: `meta` 0 = WINPIX_EVENT_UNICODE_VERSION (UTF-16),
/// 1 = ANSI, 2 = PIX encoded (opaque).
pub unsafe fn marker_text(meta: u32, data: *const c_void, size: u32) -> String {
    if data.is_null() || size == 0 {
        return String::new();
    }
    match meta {
        0 => {
            let w = std::slice::from_raw_parts(data as *const u16, size as usize / 2);
            String::from_utf16_lossy(&w[..w.iter().position(|&c| c == 0).unwrap_or(w.len())])
        }
        1 => {
            let b = std::slice::from_raw_parts(data as *const u8, size as usize);
            String::from_utf8_lossy(&b[..b.iter().position(|&c| c == 0).unwrap_or(b.len())]).into_owned()
        }
        _ => format!("<pix {size} bytes>"),
    }
}

unsafe extern "win64" fn set_marker(this: PL, meta: u32, data: *const c_void, size: u32) {
    tr!(Call::Marker { list: this as u64, kind: 2, text: &marker_text(meta, data, size) });
}
unsafe extern "win64" fn begin_event(this: PL, meta: u32, data: *const c_void, size: u32) {
    tr!(Call::Marker { list: this as u64, kind: 0, text: &marker_text(meta, data, size) });
}
unsafe extern "win64" fn end_event(this: PL) {
    tr!(Call::Marker { list: this as u64, kind: 1, text: "" });
}

/// `D3D12_TEXTURE_COPY_LOCATION` (48 bytes).
#[repr(C)]
pub struct CopyLocation {
    pub resource: *mut c_void,
    pub ty: u32,
    pub _pad: u32,
    /// PlacedFootprint (32 bytes) or SubresourceIndex (first u32).
    pub u: d3d12_metal::layout::PlacedFootprint,
}

unsafe fn copy_loc<'a>(l: &CopyLocation) -> Result<d3d12_metal::metal::cmdlist::CopyLoc<'a>, String> {
    use d3d12_metal::metal::cmdlist::CopyLoc;
    let r = resource::from_ptr(l.resource).ok_or_else(|| format!("foreign resource {:?}", l.resource))?;
    Ok(match l.ty {
        0 => CopyLoc::Subresource(&r.r, l.u.offset as u32),
        1 => CopyLoc::Footprint(&r.r, l.u),
        t => return Err(format!("copy location type {t}")),
    })
}

unsafe extern "win64" fn copy_texture_region(this: PL, dst: *const CopyLocation, x: u32, y: u32, z: u32, src: *const CopyLocation, bx: *const d3d12_metal::metal::cmdlist::CopyBox) {
    tr!(Call::CopyTextureRegion { list: this as u64, dst: tr::raw(dst), dst_x: x, dst_y: y, dst_z: z, src: tr::raw(src), src_box: tr::opt(bx) });
    let (dl, sl) = (std::ptr::read_unaligned(dst), std::ptr::read_unaligned(src));
    let b = if bx.is_null() { None } else { Some(std::ptr::read_unaligned(bx)) };
    rec(this, move |l| {
        let r = (|| -> Result<(), String> {
            let d = copy_loc(&dl)?;
            let s = copy_loc(&sl)?;
            l.l.copy_texture_region(d, [x, y, z], s, b)
        })();
        if let Err(e) = r {
            log::warn!(target: "d3d", "CopyTextureRegion: {e}");
        }
    });
}

unsafe extern "win64" fn copy_resource(this: PL, dst: *mut c_void, src: *mut c_void) {
    tr!(Call::CopyResource { list: this as u64, dst: dst as u64, src: src as u64 });
    let (Some(d), Some(s)) = (res_ptr(dst), res_ptr(src)) else {
        log::error!(target: "d3d", "CopyResource: foreign resource ({dst:?}, {src:?})");
        return;
    };
    rec(this, move |l| {
        if let Err(e) = l.l.copy_resource(&*d.0, &*s.0) {
            log::warn!(target: "d3d", "CopyResource: {e}");
        }
    });
}

pub fn temporal_upscale_enabled() -> bool {
    d3d12_metal::metal::metalfx::is_temporal_enabled()
}

pub struct TemporalUpscaleDesc {
    pub output: *mut c_void,
    pub color: *mut c_void,
    pub depth: *mut c_void,
    pub motion: *mut c_void,
    pub exposure: *mut c_void,
    pub reactive_mask: *mut c_void,
    pub render_size: [u32; 2],
    pub upscale_size: [u32; 2],
    pub jitter_offset: [f32; 2],
    pub motion_vector_scale: [f32; 2],
    pub reset: bool,
    pub depth_reversed: bool,
    pub pre_exposure: f32,
    pub auto_exposure: bool,
    pub dynamic_resolution: bool,
    pub output_resolution_motion_vectors: bool,
    pub jittered_motion_vectors: bool,
    pub color_origin: [u32; 2],
    pub depth_origin: [u32; 2],
    pub motion_origin: [u32; 2],
    pub reactive_origin: [u32; 2],
    pub output_origin: [u32; 2],
}

pub unsafe fn upscale_temporal(list: *mut c_void, desc: TemporalUpscaleDesc) -> bool {
    let Some(this) = Obj::<CommandList>::from_iface(list) else {
        log::error!(target: "d3d", "upscale_temporal: foreign command list {list:?}");
        return false;
    };
    let (Some(d), Some(c)) = (res_ptr(desc.output), res_ptr(desc.color)) else {
        log::error!(
            target: "d3d",
            "upscale_temporal: foreign resource (output={:?}, color={:?})",
            desc.output,
            desc.color
        );
        return false;
    };
    let (Some(depth), Some(motion)) = (res_ptr(desc.depth), res_ptr(desc.motion)) else {
        return upscale_resources_subrect(
            list,
            desc.output,
            desc.color,
            desc.render_size,
            desc.upscale_size,
            desc.color_origin,
            desc.output_origin,
        );
    };
    let exposure = if desc.exposure.is_null() { None } else { res_ptr(desc.exposure) };
    let reactive = if desc.reactive_mask.is_null() { None } else { res_ptr(desc.reactive_mask) };
    let render_size = desc.render_size;
    let upscale_size = desc.upscale_size;
    let jitter_offset = desc.jitter_offset;
    let motion_vector_scale = desc.motion_vector_scale;
    let reset = desc.reset;
    let depth_reversed = desc.depth_reversed;
    let pre_exposure = desc.pre_exposure;
    let auto_exposure = desc.auto_exposure;
    let dynamic_resolution = desc.dynamic_resolution;
    let output_resolution_motion_vectors = desc.output_resolution_motion_vectors;
    let jittered_motion_vectors = desc.jittered_motion_vectors;
    let color_origin = desc.color_origin;
    let depth_origin = desc.depth_origin;
    let motion_origin = desc.motion_origin;
    let reactive_origin = desc.reactive_origin;
    let output_origin = desc.output_origin;
    rec(this, move |l| {
        let mut params = d3d12_metal::metal::metalfx::TemporalUpscaleParams::new(
            &*d.0,
            &*c.0,
            &*depth.0,
            &*motion.0,
            render_size,
            upscale_size,
        );
        params.exposure = exposure.as_ref().map(|r| &*r.0);
        params.reactive_mask = reactive.as_ref().map(|r| &*r.0);
        params.jitter_offset = jitter_offset;
        params.motion_vector_scale = motion_vector_scale;
        params.reset = reset;
        params.depth_reversed = depth_reversed;
        params.pre_exposure = pre_exposure;
        params.auto_exposure = auto_exposure;
        params.dynamic_resolution = dynamic_resolution;
        params.output_resolution_motion_vectors = output_resolution_motion_vectors;
        params.jittered_motion_vectors = jittered_motion_vectors;
        params.color_origin = color_origin;
        params.depth_origin = depth_origin;
        params.motion_origin = motion_origin;
        params.reactive_origin = reactive_origin;
        params.output_origin = output_origin;
        if let Err(e) = l.l.upscale_temporal(&params) {
            log::warn!(target: "d3d", "upscale_temporal: {e}");
        }
    });
    true
}

pub unsafe fn upscale_resources(
    list: *mut c_void,
    dst: *mut c_void,
    src: *mut c_void,
    render_size: [u32; 2],
    upscale_size: [u32; 2],
) -> bool {
    upscale_resources_subrect(list, dst, src, render_size, upscale_size, [0, 0], [0, 0])
}

pub unsafe fn upscale_resources_subrect(
    list: *mut c_void,
    dst: *mut c_void,
    src: *mut c_void,
    render_size: [u32; 2],
    upscale_size: [u32; 2],
    src_origin: [u32; 2],
    dst_origin: [u32; 2],
) -> bool {
    let Some(this) = Obj::<CommandList>::from_iface(list) else {
        log::error!(target: "d3d", "upscale_resources: foreign command list {list:?}");
        return false;
    };
    let (Some(d), Some(s)) = (res_ptr(dst), res_ptr(src)) else {
        log::error!(target: "d3d", "upscale_resources: foreign resource ({dst:?}, {src:?})");
        return false;
    };
    rec(this, move |l| {
        if let Err(e) = l.l.upscale_resource_subrect(
            &*d.0,
            &*s.0,
            render_size,
            upscale_size,
            src_origin,
            dst_origin,
        ) {
            log::warn!(target: "d3d", "upscale_resources: {e}");
        }
    });
    true
}

#[allow(clippy::too_many_arguments)]
pub unsafe fn generate_reactive_mask(
    list: *mut c_void,
    opaque: *mut c_void,
    pre_upscale: *mut c_void,
    out_reactive: *mut c_void,
    render_size: [u32; 2],
    scale: f32,
    cutoff_threshold: f32,
    binary_value: f32,
    flags: u32,
) -> bool {
    let Some(this) = Obj::<CommandList>::from_iface(list) else {
        log::error!(target: "d3d", "generate_reactive_mask: foreign command list {list:?}");
        return false;
    };
    let (Some(o), Some(p), Some(out)) =
        (res_ptr(opaque), res_ptr(pre_upscale), res_ptr(out_reactive))
    else {
        return false;
    };
    rec(this, move |l| {
        if let Err(e) = l.l.generate_reactive_mask(
            &*o.0,
            &*p.0,
            &*out.0,
            render_size,
            scale,
            cutoff_threshold,
            binary_value,
            flags,
        ) {
            log::warn!(target: "d3d", "generate_reactive_mask: {e}");
        }
    });
    true
}

/// An unimplemented path inside an implemented method: same contract as a
/// trapped vtable slot (log, exit), so no fabricated result reaches the game.
fn trap(what: String) -> ! {
    d3d12_metal::trace::flush();
    log::error!("UNIMPLEMENTED {what}");
    eprintln!("UNIMPLEMENTED {what}");
    crate::process::exit_process(0xC000_0139)
}

use d3d12_metal::metal::query::query_type;

unsafe extern "win64" fn begin_query(this: PL, heap: *mut c_void, ty: u32, index: u32) {
    tr!(Call::BeginQuery { list: this as u64, heap: heap as u64, query_type: ty, index });
    let Some(h) = super::heap::query_heap_from_ptr(heap) else {
        log::error!(target: "d3d", "BeginQuery: foreign query heap {heap:?}");
        return;
    };
    if !matches!(ty, query_type::OCCLUSION | query_type::BINARY_OCCLUSION) {
        trap(format!("ID3D12GraphicsCommandList::BeginQuery(heap type {}, query type {ty}, index {index})", h.desc.ty));
    }
    let q = cs::SendPtr(&h.q as *const _);
    rec(this, move |s| {
        if let Err(e) = s.l.begin_occlusion(&*q.0, index, ty == query_type::BINARY_OCCLUSION) {
            log::warn!(target: "d3d", "BeginQuery({ty}, {index}): {e}");
        }
    });
}

unsafe extern "win64" fn end_query(this: PL, heap: *mut c_void, ty: u32, index: u32) {
    tr!(Call::EndQuery { list: this as u64, heap: heap as u64, query_type: ty, index });
    let Some(h) = super::heap::query_heap_from_ptr(heap) else {
        log::error!(target: "d3d", "EndQuery: foreign query heap {heap:?}");
        return;
    };
    if !matches!(ty, query_type::TIMESTAMP | query_type::OCCLUSION | query_type::BINARY_OCCLUSION) {
        trap(format!("ID3D12GraphicsCommandList::EndQuery(query type {ty})"));
    }
    let q = cs::SendPtr(&h.q as *const _);
    rec(this, move |s| {
        let r = match ty {
            query_type::TIMESTAMP => s.l.write_timestamp(&*q.0, index),
            _ => s.l.end_occlusion(&*q.0, index),
        };
        if let Err(e) = r {
            log::warn!(target: "d3d", "EndQuery({ty}, {index}): {e}");
        }
    });
}

unsafe extern "win64" fn resolve_query_data(this: PL, heap: *mut c_void, ty: u32, start: u32, n: u32, dst: *mut c_void, offset: u64) {
    tr!(Call::ResolveQueryData { list: this as u64, heap: heap as u64, query_type: ty, start, count: n, dst: dst as u64, dst_offset: offset });
    let (Some(h), Some(d)) = (super::heap::query_heap_from_ptr(heap), res_ptr(dst)) else {
        log::error!(target: "d3d", "ResolveQueryData: foreign heap/resource ({heap:?}, {dst:?})");
        return;
    };
    if !matches!(ty, query_type::TIMESTAMP | query_type::OCCLUSION | query_type::BINARY_OCCLUSION) {
        trap(format!("ID3D12GraphicsCommandList::ResolveQueryData(query type {ty})"));
    }
    let q = cs::SendPtr(&h.q as *const _);
    rec(this, move |s| {
        let r = match ty {
            query_type::TIMESTAMP => s.l.resolve_timestamps(&*q.0, start, n, &*d.0, offset),
            _ => s.l.resolve_occlusion(&*q.0, start, n, &*d.0, offset),
        };
        if let Err(e) = r {
            log::warn!(target: "d3d", "ResolveQueryData({ty}, {start}+{n}): {e}");
        }
    });
}

/// `D3D12_VERTEX_BUFFER_VIEW`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct VertexBufferView {
    pub va: u64,
    pub size: u32,
    pub stride: u32,
}

/// `D3D12_INDEX_BUFFER_VIEW`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IndexBufferView {
    pub va: u64,
    pub size: u32,
    pub format: u32,
}

unsafe extern "win64" fn ia_set_vertex_buffers(this: PL, start: u32, n: u32, views: *const VertexBufferView) {
    tr!(Call::IASetVertexBuffers { list: this as u64, start_slot: start, views: tr::raw_n(views, n as usize) });
    const MAX: usize = d3d12_metal::metal::pipeline::MAX_VERTEX_SLOTS as usize;
    let mut vs = [(0u64, 0u32, 0u32); MAX];
    let mut m = 0usize;
    for i in 0..n {
        let slot = start + i;
        if slot as usize >= MAX {
            log::warn!(target: "d3d", "IASetVertexBuffers: slot {slot} is beyond the {MAX} supported slots");
            continue;
        }
        vs[m] = if views.is_null() {
            (0, 0, 0)
        } else {
            let v = std::ptr::read_unaligned(views.add(i as usize));
            (v.va, v.size, v.stride)
        };
        m += 1;
    }
    let first = start as usize;
    match m {
        0 => {}
        1 => {
            let v0 = vs[0];
            rec(this, move |s| s.l.gfx.vbs[first] = v0);
        }
        2..=4 => {
            let small = [vs[0], vs[1], vs[2], vs[3]];
            let len = m as u8;
            rec(this, move |s| s.l.gfx.vbs[first..first + len as usize].copy_from_slice(&small[..len as usize]));
        }
        _ => {
            let len = m as u8;
            rec(this, move |s| s.l.gfx.vbs[first..first + len as usize].copy_from_slice(&vs[..len as usize]));
        }
    }
}

unsafe extern "win64" fn ia_set_index_buffer(this: PL, view: *const IndexBufferView) {
    tr!(Call::IASetIndexBuffer { list: this as u64, view: tr::opt(view) });
    let ib = if view.is_null() {
        None
    } else {
        let v = std::ptr::read_unaligned(view);
        Some((v.va, v.size, v.format))
    };
    rec(this, move |s| s.l.gfx.ib = ib);
}

unsafe extern "win64" fn om_set_depth_bounds(this: PL, min: f32, max: f32) {
    tr!(Call::OMSetDepthBounds { list: this as u64, min, max });
    rec(this, move |s| s.l.set_depth_bounds(min, max));
}

crate::win64_bridge!(fn om_set_depth_bounds(PL, f32, f32) -> ());

unsafe extern "win64" fn draw_instanced(this: PL, vertices: u32, instances: u32, first_vertex: u32, first_instance: u32) {
    tr!(Call::DrawInstanced { list: this as u64, vertex_count: vertices, instance_count: instances, start_vertex: first_vertex, start_instance: first_instance });
    rec(this, move |s| {
        if let Err(e) = s.l.draw(vertices, instances, first_vertex, first_instance) {
            log::warn!(target: "d3d", "DrawInstanced({vertices}, {instances}, {first_vertex}, {first_instance}): {e}");
        }
    });
}

unsafe extern "win64" fn draw_indexed_instanced(this: PL, indices: u32, instances: u32, first_index: u32, base_vertex: i32, first_instance: u32) {
    tr!(Call::DrawIndexedInstanced { list: this as u64, index_count: indices, instance_count: instances, start_index: first_index, base_vertex, start_instance: first_instance });
    rec(this, move |s| {
        if let Err(e) = s.l.draw_indexed(indices, instances, first_index, base_vertex, first_instance) {
            log::warn!(target: "d3d", "DrawIndexedInstanced({indices}, {instances}, {first_index}, {base_vertex}, {first_instance}): {e}");
        }
    });
}

unsafe extern "win64" fn execute_indirect(this: PL, sig: *mut c_void, max_count: u32, args: *mut c_void, args_off: u64, count: *mut c_void, count_off: u64) {
    tr!(Call::ExecuteIndirect { list: this as u64, signature: sig as u64, max_count, args: args as u64, args_offset: args_off, count: count as u64, count_offset: count_off });
    let Some(s) = super::pso::command_signature_from_ptr(sig) else {
        log::error!(target: "d3d", "ExecuteIndirect: foreign command signature {sig:?}");
        return;
    };
    let l = match &s.layout {
        Ok(l) => l,
        Err(e) => trap(format!("ID3D12GraphicsCommandList::ExecuteIndirect({:?}): {e}", s.args)),
    };
    let Some(a) = res_ptr(args) else {
        log::error!(target: "d3d", "ExecuteIndirect: foreign argument buffer {args:?}");
        return;
    };
    let c = if count.is_null() {
        None
    } else {
        match resource::from_ptr(count) {
            Some(c) => Some(c.r.gpu_va + count_off),
            None => {
                log::error!(target: "d3d", "ExecuteIndirect: foreign count buffer {count:?}");
                return;
            }
        }
    };
    let lp = cs::SendPtr(l as *const d3d12_metal::metal::indirect::IndirectLayout);
    rec(this, move |s| {
        let (l, a) = (&*lp.0, &*a.0);
        let args_owned;
        let args_cpu = if l.ib_offset.is_some() || !l.vb_offsets.is_empty() {
            let byte_len = (max_count as usize).saturating_mul(l.stride as usize);
            if let Some(slice) = a.shared_slice(args_off, byte_len) {
                Some(slice)
            } else {
                args_owned = a.cpu_bytes(args_off, byte_len);
                args_owned.as_deref()
            }
        } else {
            None
        };
        if let Err(e) = s.l.execute_indirect(l, max_count, a.gpu_va + args_off, args_cpu, c) {
            log::warn!(target: "d3d", "ExecuteIndirect({:?} x{max_count}): {e}", l.work);
        }
    });
}

unsafe fn clear_uav(this: PL, what: &'static str, cpu: usize, res: *mut c_void, v: d3d12_metal::metal::clear::ClearValue, n: u32, rects: *const Rect) {
    let Some(r) = resource::from_ptr(res) else {
        log::error!(target: "d3d", "{what}: foreign resource {res:?}");
        return;
    };
    let desc = match d3d12_metal::metal::descriptor::resolve_uav_for_clear(cpu, &r.r, what) {
        d3d12_metal::metal::descriptor::UavClearLookup::Found(d) => d,
        d3d12_metal::metal::descriptor::UavClearLookup::MissingBufferView => {
            trap(format!("ID3D12GraphicsCommandList::{what}: CPU handle {cpu:#x} holds no UAV created by CreateUnorderedAccessView"));
        }
    };
    let rp = cs::SendPtr(Arc::as_ptr(&r.r));
    if n == 0 || rects.is_null() {
        rec(this, move |s| {
            let r = &*rp.0;
            if let Err(e) = s.l.clear_uav(r, desc.as_ref(), v, &[]) {
                log::warn!(target: "d3d", "ID3D12GraphicsCommandList::{what}({:?}): {e}", r.desc);
            }
        });
    } else if n <= 4 {
        let mut inline = [Rect::default(); 4];
        let len = n as u8;
        for i in 0..n as usize {
            inline[i] = std::ptr::read_unaligned(rects.add(i));
        }
        rec(this, move |s| {
            let r = &*rp.0;
            if let Err(e) = s.l.clear_uav(r, desc.as_ref(), v, &inline[..len as usize]) {
                log::warn!(target: "d3d", "ID3D12GraphicsCommandList::{what}({:?}): {e}", r.desc);
            }
        });
    } else {
        let mut rs = Vec::with_capacity(n as usize);
        for i in 0..n as usize {
            rs.push(std::ptr::read_unaligned(rects.add(i)));
        }
        rec(this, move |s| {
            let r = &*rp.0;
            if let Err(e) = s.l.clear_uav(r, desc.as_ref(), v, &rs) {
                log::warn!(target: "d3d", "ID3D12GraphicsCommandList::{what}({:?}): {e}", r.desc);
            }
        });
    }
}

unsafe extern "win64" fn clear_uav_uint(this: PL, gpu: u64, cpu: usize, res: *mut c_void, values: *const [u32; 4], n: u32, rects: *const Rect) {
    let vals = if values.is_null() { [0; 4] } else { std::ptr::read_unaligned(values) };
    tr!(Call::ClearUnorderedAccessViewUint { list: this as u64, gpu_handle: gpu, cpu_handle: cpu as u64, resource: res as u64, values: U32s::from_slice(&vals), rects: tr::raw_n(rects, n as usize) });
    clear_uav(this, "ClearUnorderedAccessViewUint", cpu, res, d3d12_metal::metal::clear::ClearValue::Uint(vals), n, rects);
}

unsafe extern "win64" fn clear_uav_float(this: PL, gpu: u64, cpu: usize, res: *mut c_void, values: *const [f32; 4], n: u32, rects: *const Rect) {
    let vals = if values.is_null() { [0.0; 4] } else { std::ptr::read_unaligned(values) };
    tr!(Call::ClearUnorderedAccessViewFloat { list: this as u64, gpu_handle: gpu, cpu_handle: cpu as u64, resource: res as u64, values: vals, rects: tr::raw_n(rects, n as usize) });
    clear_uav(this, "ClearUnorderedAccessViewFloat", cpu, res, d3d12_metal::metal::clear::ClearValue::Float(vals), n, rects);
}

/// `D3D12_DISCARD_REGION` (NumRects, pRects, FirstSubresource, NumSubresources).
#[repr(C)]
struct DiscardRegion {
    num_rects: u32,
    _rects: *const Rect,
    first: u32,
    count: u32,
}

/// `DiscardResource`: the contents become undefined (keeping them is valid).
/// Whole-resource discards are recorded so the next render pass attaching
/// the resource may skip loading it (`AW2_RP_DISCARD=1`, see
/// `CommandList::discard_resource`).
unsafe extern "win64" fn discard_resource(this: PL, res: *mut c_void, region: *const DiscardRegion) {
    tr!(Call::DiscardResource { list: this as u64, resource: res as u64, region: tr::opt(region) });
    let Some(r) = res_ptr(res) else {
        return;
    };
    let region = region.as_ref().map(|g| (g.num_rects, g.first, g.count));
    rec(this, move |s| s.l.discard_resource(&*r.0, region));
}

unsafe extern "win64" fn resolve_subresource(this: PL, dst: *mut c_void, dst_sub: u32, src: *mut c_void, src_sub: u32, format: u32) {
    tr!(Call::ResolveSubresource { list: this as u64, dst: dst as u64, dst_sub, src: src as u64, src_sub, format });
    let (Some(d), Some(s)) = (res_ptr(dst), res_ptr(src)) else {
        log::error!(target: "d3d", "ResolveSubresource: foreign resource ({dst:?}, {src:?})");
        return;
    };
    rec(this, move |l| {
        if let Err(e) = l.l.resolve_subresource(&*d.0, dst_sub, &*s.0, src_sub, format) {
            trap(format!("ID3D12GraphicsCommandList::ResolveSubresource: {e}"));
        }
    });
}

unsafe extern "win64" fn dispatch_mesh(this: PL, x: u32, y: u32, z: u32) {
    tr!(Call::DispatchMesh { list: this as u64, x, y, z });
    rec(this, move |s| {
        if let Err(e) = s.l.dispatch_mesh(x, y, z) {
            log::warn!(target: "d3d", "DispatchMesh({x}, {y}, {z}): {e}");
        }
    });
}

unsafe extern "win64" fn build_raytracing_acceleration_structure(
    this: PL,
    desc: *const d3d12_metal::rt::d3d::BuildDesc,
    num_postbuild: u32,
    postbuild: *const d3d12_metal::rt::d3d::PostbuildInfoDesc,
) {
    if desc.is_null() {
        log::warn!(target: "d3d", "BuildRaytracingAccelerationStructure: null desc");
        return;
    }
    let d = std::ptr::read_unaligned(desc);
    let post_n = if postbuild.is_null() { 0 } else { num_postbuild as usize };
    let inputs = match d3d12_metal::rt::d3d::Inputs::from_raw(&d.Inputs) {
        Ok(i) => i,
        Err(e) => {
            log::warn!(target: "d3d", "BuildRaytracingAccelerationStructure({:?}): {e}", d.Inputs);
            return;
        }
    };
    tr!(
        let geoms: Vec<d3d12_metal::rt::d3d::GeometryDesc> = match &inputs {
            d3d12_metal::rt::d3d::Inputs::Bottom { geometries, .. } => geometries.iter().map(|g| g.to_desc()).collect(),
            d3d12_metal::rt::d3d::Inputs::Top { .. } => Vec::new(),
        };
        let instances_va = if d.Inputs.Type == d3d12_metal::rt::d3d::TYPE_TOP_LEVEL { d.Inputs.Descs } else { 0 };
        Call::BuildRaytracingAccelerationStructure {
            list: this as u64,
            dest: d.DestAccelerationStructureData,
            source: d.SourceAccelerationStructureData,
            scratch: d.ScratchAccelerationStructureData,
            inputs_type: d.Inputs.Type,
            flags: d.Inputs.Flags,
            num_descs: d.Inputs.NumDescs,
            descs_layout: d.Inputs.DescsLayout,
            instances_va,
            geometries: tr::raw_n(geoms.as_ptr(), geoms.len()),
            postbuild: tr::raw_n(postbuild, post_n),
        }
    );
    let args = d3d12_metal::rt::accel::BuildArgs {
        dest: d.DestAccelerationStructureData,
        source: d.SourceAccelerationStructureData,
        scratch: d.ScratchAccelerationStructureData,
        inputs,
        owner: 0,
    };
    let dest = d.DestAccelerationStructureData;
    if post_n == 0 {
        rec(this, move |s| {
            if let Err(e) = s.l.build_accel(&args, &[]) {
                log::warn!(target: "d3d", "BuildRaytracingAccelerationStructure(dest {dest:#x}): {e}");
            }
        });
    } else if post_n <= 2 {
        let mut inline = [d3d12_metal::rt::d3d::PostbuildInfoDesc::default(); 2];
        for i in 0..post_n {
            inline[i] = std::ptr::read_unaligned(postbuild.add(i));
        }
        let len = post_n as u8;
        rec(this, move |s| {
            if let Err(e) = s.l.build_accel(&args, &inline[..len as usize]) {
                log::warn!(target: "d3d", "BuildRaytracingAccelerationStructure(dest {dest:#x}): {e}");
            }
        });
    } else {
        let mut post = Vec::with_capacity(post_n);
        for i in 0..post_n {
            post.push(std::ptr::read_unaligned(postbuild.add(i)));
        }
        rec(this, move |s| {
            if let Err(e) = s.l.build_accel(&args, &post) {
                log::warn!(target: "d3d", "BuildRaytracingAccelerationStructure(dest {dest:#x}): {e}");
            }
        });
    }
}

unsafe extern "win64" fn emit_raytracing_acceleration_structure_postbuild_info(
    this: PL,
    desc: *const d3d12_metal::rt::d3d::PostbuildInfoDesc,
    num_sources: u32,
    sources: *const u64,
) {
    if desc.is_null() {
        log::warn!(target: "d3d", "EmitRaytracingAccelerationStructurePostbuildInfo: null desc");
        return;
    }
    let d = std::ptr::read_unaligned(desc);
    let n = if sources.is_null() { 0 } else { num_sources as usize };
    if n == 0 {
        tr!(Call::EmitRaytracingAccelerationStructurePostbuildInfo {
            list: this as u64,
            dest_buffer: d.DestBuffer,
            info_type: d.InfoType,
            sources: U64s::from_slice(&[]),
        });
        rec(this, move |s| {
            if let Err(e) = s.l.emit_accel_postbuild(&d, &[]) {
                log::warn!(target: "d3d", "EmitRaytracingAccelerationStructurePostbuildInfo(type {}, n={num_sources}): {e}", d.InfoType);
            }
        });
    } else if n <= 4 {
        let mut inline = [0u64; 4];
        for i in 0..n {
            inline[i] = std::ptr::read_unaligned(sources.add(i));
        }
        tr!(Call::EmitRaytracingAccelerationStructurePostbuildInfo {
            list: this as u64,
            dest_buffer: d.DestBuffer,
            info_type: d.InfoType,
            sources: U64s::from_slice(&inline[..n]),
        });
        let len = n as u8;
        rec(this, move |s| {
            if let Err(e) = s.l.emit_accel_postbuild(&d, &inline[..len as usize]) {
                log::warn!(target: "d3d", "EmitRaytracingAccelerationStructurePostbuildInfo(type {}, n={num_sources}): {e}", d.InfoType);
            }
        });
    } else {
        let mut srcs = Vec::with_capacity(n);
        for i in 0..n {
            srcs.push(std::ptr::read_unaligned(sources.add(i)));
        }
        tr!(Call::EmitRaytracingAccelerationStructurePostbuildInfo {
            list: this as u64,
            dest_buffer: d.DestBuffer,
            info_type: d.InfoType,
            sources: U64s::from_slice(&srcs),
        });
        rec(this, move |s| {
            if let Err(e) = s.l.emit_accel_postbuild(&d, &srcs) {
                log::warn!(target: "d3d", "EmitRaytracingAccelerationStructurePostbuildInfo(type {}, n={num_sources}): {e}", d.InfoType);
            }
        });
    }
}

unsafe extern "win64" fn copy_raytracing_acceleration_structure(this: PL, dest: u64, source: u64, mode: u32) {
    tr!(Call::CopyRaytracingAccelerationStructure { list: this as u64, dest, source, mode });
    rec(this, move |s| {
        if let Err(e) = s.l.copy_accel(dest, source, mode, 0) {
            log::warn!(target: "d3d", "CopyRaytracingAccelerationStructure({dest:#x} <- {source:#x}, mode {mode}): {e}");
        }
    });
}

unsafe extern "win64" fn set_pipeline_state1(this: PL, state_object: *mut c_void) {
    tr!(Call::SetPipelineState1 { list: this as u64, state_object: state_object as u64 });
    if state_object.is_null() {
        return;
    }
    let Some(so) = super::pso::state_object_from_ptr(state_object) else {
        log::error!(target: "d3d", "SetPipelineState1: foreign state object {state_object:?}");
        return;
    };
    let so = so.clone();
    rec(this, move |s| s.l.set_raytracing_pipeline(so));
}

unsafe extern "win64" fn dispatch_rays(this: PL, desc: *const d3d12_metal::rt::pipeline::DispatchRaysDesc) {
    if desc.is_null() {
        log::warn!(target: "d3d", "DispatchRays: null desc");
        return;
    }
    let d = std::ptr::read_unaligned(desc);
    tr!(Call::DispatchRays {
        list: this as u64,
        width: d.Width,
        height: d.Height,
        depth: d.Depth,
        desc: tr::raw(&d),
    });
    rec(this, move |s| {
        if let Err(e) = s.l.dispatch_rays(&d) {
            log::warn!(target: "d3d", "DispatchRays({}x{}x{}): {e}", d.Width, d.Height, d.Depth);
        }
    });
}

/// Pure-integer leaf D3D12 command list COM methods (<= 8 integer args, no XMM args,
/// never re-enter guest code) eligible for `DF_PLAIN` direct calls.
pub(crate) fn plain_fns() -> &'static [usize] {
    static FNS: OnceLock<Box<[usize]>> = OnceLock::new();
    FNS.get_or_init(|| {
        vec![
            set_name_list as *const () as usize,
            get_type as *const () as usize,
            close as *const () as usize,
            reset as *const () as usize,
            clear_state as *const () as usize,
            resource_barrier as *const () as usize,
            clear_render_target_view as *const () as usize,
            om_set_render_targets as *const () as usize,
            copy_buffer_region as *const () as usize,
            copy_texture_region as *const () as usize,
            copy_resource as *const () as usize,
            rs_set_viewports as *const () as usize,
            rs_set_scissor_rects as *const () as usize,
            ia_set_primitive_topology as *const () as usize,
            om_set_blend_factor as *const () as usize,
            om_set_stencil_ref as *const () as usize,
            om_set_front_and_back_stencil_ref as *const () as usize,
            set_descriptor_heaps as *const () as usize,
            set_pipeline_state as *const () as usize,
            set_compute_root_signature as *const () as usize,
            set_graphics_root_signature as *const () as usize,
            set_compute_root_descriptor_table as *const () as usize,
            set_graphics_root_descriptor_table as *const () as usize,
            set_compute_root_constant as *const () as usize,
            set_graphics_root_constant as *const () as usize,
            set_compute_root_constants as *const () as usize,
            set_graphics_root_constants as *const () as usize,
            set_compute_root_cbv as *const () as usize,
            set_graphics_root_cbv as *const () as usize,
            set_compute_root_srv as *const () as usize,
            set_graphics_root_srv as *const () as usize,
            set_compute_root_uav as *const () as usize,
            set_graphics_root_uav as *const () as usize,
            dispatch as *const () as usize,
            draw_instanced as *const () as usize,
            draw_indexed_instanced as *const () as usize,
            execute_indirect as *const () as usize,
            clear_uav_uint as *const () as usize,
            clear_uav_float as *const () as usize,
            discard_resource as *const () as usize,
            resolve_subresource as *const () as usize,
            dispatch_mesh as *const () as usize,
            ia_set_vertex_buffers as *const () as usize,
            ia_set_index_buffer as *const () as usize,
            begin_query as *const () as usize,
            end_query as *const () as usize,
            resolve_query_data as *const () as usize,
            set_predication as *const () as usize,
            set_marker as *const () as usize,
            begin_event as *const () as usize,
            end_event as *const () as usize,
            build_raytracing_acceleration_structure as *const () as usize,
            emit_raytracing_acceleration_structure_postbuild_info as *const () as usize,
            copy_raytracing_acceleration_structure as *const () as usize,
            set_pipeline_state1 as *const () as usize,
            dispatch_rays as *const () as usize,
        ]
        .into_boxed_slice()
    })
}


