//! NRI command buffer (`NriCommandBuffer`), D3D12 command list vtable helpers,
//! command recording callbacks, synchronization, and queue submission.

use std::ffi::{c_void, CStr};
use std::ptr;

use super::abi::*;
use super::descriptor::{NriDescriptorPool, NriDescriptorSet, NriPipelineLayout};
use super::device::{
    NriBuffer, NriCommandAllocator, NriCommandQueue, NriDevice, NriFence, NriFrameBuffer,
    NriPipeline, NriQueryPool, NriTexture,
};
use crate::d3d12::cmdlist::{CopyLocation, IndexBufferView, VertexBufferView};
use crate::d3d12::com::{com_add_ref, com_release};
use crate::d3d12::cs;
use crate::strings::{rd, wr};
use crate::types::{BOOL, HRESULT};
use d3d12_metal::layout::PlacedFootprint;
use d3d12_metal::metal::cmdlist::CopyBox;
use d3d12_metal::types::{Rect, ResourceBarrier, Viewport};

// ---------------------------------------------------------------------------
// Command buffer object
// ---------------------------------------------------------------------------

pub struct NriCommandBuffer {
    pub(super) device: *mut NriDevice,
    pub(super) d3d12_list: *mut c_void,
    pub(super) d3d12_allocator: *mut c_void,
    pub(super) list_type: u32,
    pub(super) pipeline_layout: *const NriPipelineLayout,
    pub(super) is_graphics_layout: bool,
    pub(super) pipeline: *const NriPipeline,
    pub(super) frame_buffer: *const NriFrameBuffer,
    pub(super) descriptor_sets: Vec<*const NriDescriptorSet>,
}

impl Drop for NriCommandBuffer {
    fn drop(&mut self) {
        unsafe {
            com_release(self.d3d12_list);
            com_release(self.d3d12_allocator);
        }
    }
}

// ---------------------------------------------------------------------------
// D3D12 command list COM vtable invocation helpers
// ---------------------------------------------------------------------------

pub(super) unsafe fn d3d12_cmd_get_type(list: *mut c_void) -> u32 {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) -> u32 = std::mem::transmute(rd(vt.add(8)));
    f(list)
}

unsafe fn d3d12_cmd_close(list: *mut c_void) -> HRESULT {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) -> HRESULT = std::mem::transmute(rd(vt.add(9)));
    f(list)
}

unsafe fn d3d12_cmd_reset(list: *mut c_void, alloc: *mut c_void) -> HRESULT {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) -> HRESULT =
        std::mem::transmute(rd(vt.add(10)));
    f(list, alloc, ptr::null_mut())
}

unsafe fn d3d12_cmd_draw(list: *mut c_void, v: u32, inst: u32, bv: u32, bi: u32) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, u32, u32, u32) =
        std::mem::transmute(rd(vt.add(12)));
    f(list, v, inst, bv, bi);
}

unsafe fn d3d12_cmd_draw_indexed(
    list: *mut c_void,
    idx: u32,
    inst: u32,
    bi: u32,
    bv: i32,
    binst: u32,
) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, u32, u32, i32, u32) =
        std::mem::transmute(rd(vt.add(13)));
    f(list, idx, inst, bi, bv, binst);
}

unsafe fn d3d12_cmd_dispatch(list: *mut c_void, x: u32, y: u32, z: u32) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, u32, u32) =
        std::mem::transmute(rd(vt.add(14)));
    f(list, x, y, z);
}

unsafe fn d3d12_cmd_copy_buffer_region(
    list: *mut c_void,
    dst: *mut c_void,
    dst_off: u64,
    src: *mut c_void,
    src_off: u64,
    bytes: u64,
) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, u64, *mut c_void, u64, u64) =
        std::mem::transmute(rd(vt.add(15)));
    f(list, dst, dst_off, src, src_off, bytes);
}

unsafe fn d3d12_cmd_copy_texture_region(
    list: *mut c_void,
    dst: &CopyLocation,
    x: u32,
    y: u32,
    z: u32,
    src: &CopyLocation,
    bx: *const CopyBox,
) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const CopyLocation,
        u32,
        u32,
        u32,
        *const CopyLocation,
        *const CopyBox,
    ) = std::mem::transmute(rd(vt.add(16)));
    f(list, dst, x, y, z, src, bx);
}

unsafe fn d3d12_cmd_copy_resource(list: *mut c_void, dst: *mut c_void, src: *mut c_void) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
        std::mem::transmute(rd(vt.add(17)));
    f(list, dst, src);
}

unsafe fn d3d12_cmd_ia_set_topology(list: *mut c_void, topo: u32) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32) = std::mem::transmute(rd(vt.add(20)));
    f(list, topo);
}

unsafe fn d3d12_cmd_rs_set_viewports(list: *mut c_void, vps: &[Viewport]) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const Viewport) =
        std::mem::transmute(rd(vt.add(21)));
    f(list, vps.len() as u32, vps.as_ptr());
}

unsafe fn d3d12_cmd_rs_set_scissors(list: *mut c_void, rects: &[Rect]) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const Rect) =
        std::mem::transmute(rd(vt.add(22)));
    f(list, rects.len() as u32, rects.as_ptr());
}

unsafe fn d3d12_cmd_om_set_blend_factor(list: *mut c_void, factor: &[f32; 4]) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *const [f32; 4]) =
        std::mem::transmute(rd(vt.add(23)));
    f(list, factor);
}

unsafe fn d3d12_cmd_om_set_stencil_ref(list: *mut c_void, reference: u32) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32) = std::mem::transmute(rd(vt.add(24)));
    f(list, reference);
}

unsafe fn d3d12_cmd_set_pso(list: *mut c_void, pso: *mut c_void) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
        std::mem::transmute(rd(vt.add(25)));
    f(list, pso);
}

unsafe fn d3d12_cmd_resource_barrier(list: *mut c_void, barriers: &[ResourceBarrier]) {
    if barriers.is_empty() {
        return;
    }
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const ResourceBarrier) =
        std::mem::transmute(rd(vt.add(26)));
    f(list, barriers.len() as u32, barriers.as_ptr());
}

unsafe fn d3d12_cmd_set_descriptor_heaps(list: *mut c_void, heaps: &[*mut c_void]) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void) =
        std::mem::transmute(rd(vt.add(28)));
    f(list, heaps.len() as u32, heaps.as_ptr());
}

unsafe fn d3d12_cmd_set_root_sig(list: *mut c_void, rs: *mut c_void, graphics: bool) {
    let vt = rd(list as *const *const usize);
    let slot = if graphics { 30 } else { 29 };
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void) =
        std::mem::transmute(rd(vt.add(slot)));
    f(list, rs);
}

unsafe fn d3d12_cmd_set_root_table(list: *mut c_void, param: u32, gpu: u64, graphics: bool) {
    let vt = rd(list as *const *const usize);
    let slot = if graphics { 32 } else { 31 };
    let f: unsafe extern "win64" fn(*mut c_void, u32, u64) =
        std::mem::transmute(rd(vt.add(slot)));
    f(list, param, gpu);
}

unsafe fn d3d12_cmd_set_root_constants(
    list: *mut c_void,
    param: u32,
    count: u32,
    data: *const u32,
    dest_offset: u32,
    graphics: bool,
) {
    let vt = rd(list as *const *const usize);
    let slot = if graphics { 36 } else { 35 };
    let f: unsafe extern "win64" fn(*mut c_void, u32, u32, *const u32, u32) =
        std::mem::transmute(rd(vt.add(slot)));
    f(list, param, count, data, dest_offset);
}

unsafe fn d3d12_cmd_set_root_cbv(list: *mut c_void, param: u32, gpu_va: u64, graphics: bool) {
    let vt = rd(list as *const *const usize);
    let slot = if graphics { 38 } else { 37 };
    let f: unsafe extern "win64" fn(*mut c_void, u32, u64) =
        std::mem::transmute(rd(vt.add(slot)));
    f(list, param, gpu_va);
}

unsafe fn d3d12_cmd_ia_set_index_buffer(list: *mut c_void, view: &IndexBufferView) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *const IndexBufferView) =
        std::mem::transmute(rd(vt.add(43)));
    f(list, view);
}

unsafe fn d3d12_cmd_ia_set_vertex_buffers(
    list: *mut c_void,
    start: u32,
    views: &[VertexBufferView],
) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, u32, *const VertexBufferView) =
        std::mem::transmute(rd(vt.add(44)));
    f(list, start, views.len() as u32, views.as_ptr());
}

unsafe fn d3d12_cmd_om_set_render_targets(list: *mut c_void, rtvs: &[usize], dsv: usize) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const usize, BOOL, *const usize) =
        std::mem::transmute(rd(vt.add(46)));
    let dsv_ptr = if dsv != 0 {
        &dsv as *const usize
    } else {
        ptr::null()
    };
    f(list, rtvs.len() as u32, rtvs.as_ptr(), 0, dsv_ptr);
}

/// Calls `ID3D12GraphicsCommandList::ClearDepthStencilView` (slot 47), which
/// uses `win64_bridge!` on aarch64 because `depth: f32` sits in Win64 slot 4.
unsafe fn d3d12_cmd_clear_dsv(
    list: *mut c_void,
    dsv: usize,
    flags: u32,
    depth: f32,
    stencil: u8,
    rects: &[Rect],
) {
    let vt = rd(list as *const *const usize);
    let r_ptr = if rects.is_empty() {
        ptr::null()
    } else {
        rects.as_ptr()
    };
    #[cfg(target_arch = "aarch64")]
    {
        let f: unsafe extern "C" fn(
            u64, u64, u64, u64, u64, u64, u64, u64,
            u64, u64, u64, u64, u64, u64, u64, u64,
            f64, f64, f64, f64,
        ) = std::mem::transmute(rd(vt.add(47)));
        f(
            list as u64,
            dsv as u64,
            flags as u64,
            0,
            stencil as u64,
            rects.len() as u64,
            r_ptr as u64,
            0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0.0,
            0.0,
            0.0,
            f64::from_bits(depth.to_bits() as u64),
        );
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let f: unsafe extern "win64" fn(
            *mut c_void, usize, u32, f32, u8, u32, *const Rect,
        ) = std::mem::transmute(rd(vt.add(47)));
        f(list, dsv, flags, depth, stencil, rects.len() as u32, r_ptr);
    }
}

unsafe fn d3d12_cmd_clear_rtv(list: *mut c_void, rtv: usize, color: &[f32; 4], rects: &[Rect]) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, usize, *const [f32; 4], u32, *const Rect) =
        std::mem::transmute(rd(vt.add(48)));
    let r_ptr = if rects.is_empty() {
        ptr::null()
    } else {
        rects.as_ptr()
    };
    f(list, rtv, color, rects.len() as u32, r_ptr);
}

unsafe fn d3d12_cmd_clear_uav_uint(
    list: *mut c_void,
    gpu: u64,
    cpu: usize,
    res: *mut c_void,
    vals: &[u32; 4],
) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        u64,
        usize,
        *mut c_void,
        *const [u32; 4],
        u32,
        *const Rect,
    ) = std::mem::transmute(rd(vt.add(49)));
    f(list, gpu, cpu, res, vals, 0, ptr::null());
}

unsafe fn d3d12_cmd_clear_uav_float(
    list: *mut c_void,
    gpu: u64,
    cpu: usize,
    res: *mut c_void,
    vals: &[f32; 4],
) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        u64,
        usize,
        *mut c_void,
        *const [f32; 4],
        u32,
        *const Rect,
    ) = std::mem::transmute(rd(vt.add(50)));
    f(list, gpu, cpu, res, vals, 0, ptr::null());
}

unsafe fn d3d12_cmd_begin_query(list: *mut c_void, heap: *mut c_void, ty: u32, idx: u32) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32) =
        std::mem::transmute(rd(vt.add(52)));
    f(list, heap, ty, idx);
}

unsafe fn d3d12_cmd_end_query(list: *mut c_void, heap: *mut c_void, ty: u32, idx: u32) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32) =
        std::mem::transmute(rd(vt.add(53)));
    f(list, heap, ty, idx);
}

unsafe fn d3d12_cmd_resolve_query(
    list: *mut c_void,
    heap: *mut c_void,
    ty: u32,
    start: u32,
    num: u32,
    dst: *mut c_void,
    off: u64,
) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut c_void, u64) =
        std::mem::transmute(rd(vt.add(54)));
    f(list, heap, ty, start, num, dst, off);
}

unsafe fn d3d12_cmd_begin_event(list: *mut c_void, name: *const u8) {
    if name.is_null() {
        return;
    }
    let bytes = CStr::from_ptr(name as *const std::ffi::c_char).to_bytes_with_nul();
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const c_void, u32) =
        std::mem::transmute(rd(vt.add(57)));
    f(list, 1, bytes.as_ptr() as *const c_void, bytes.len() as u32);
}

unsafe fn d3d12_cmd_end_event(list: *mut c_void) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) = std::mem::transmute(rd(vt.add(58)));
    f(list);
}

unsafe fn d3d12_cmd_execute_indirect(
    list: *mut c_void,
    sig: *mut c_void,
    max_count: u32,
    args: *mut c_void,
    args_off: u64,
) {
    let vt = rd(list as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *mut c_void,
        u32,
        *mut c_void,
        u64,
        *mut c_void,
        u64,
    ) = std::mem::transmute(rd(vt.add(59)));
    f(list, sig, max_count, args, args_off, ptr::null_mut(), 0);
}

/// Calls `ID3D12GraphicsCommandList1::OMSetDepthBounds` (slot 62), which uses
/// `win64_bridge!` on aarch64 for its `f32` arguments.
unsafe fn d3d12_cmd_set_depth_bounds(list: *mut c_void, min: f32, max: f32) {
    let vt = rd(list as *const *const usize);
    #[cfg(target_arch = "aarch64")]
    {
        let f: unsafe extern "C" fn(
            u64, u64, u64, u64, u64, u64, u64, u64,
            u64, u64, u64, u64, u64, u64, u64, u64,
            f64, f64, f64, f64,
        ) = std::mem::transmute(rd(vt.add(62)));
        f(
            list as u64,
            0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0.0,
            f64::from_bits(min.to_bits() as u64),
            f64::from_bits(max.to_bits() as u64),
            0.0,
        );
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let f: unsafe extern "win64" fn(*mut c_void, f32, f32) =
            std::mem::transmute(rd(vt.add(62)));
        f(list, min, max);
    }
}

// ---------------------------------------------------------------------------
// CoreInterface — Command buffer creation & recording (slots 7, 37..73)
// ---------------------------------------------------------------------------

pub(super) unsafe extern "win64" fn core_create_command_buffer(
    allocator: *mut NriCommandAllocator,
    out: *mut *mut NriCommandBuffer,
) -> u32 {
    if allocator.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let dev = (*allocator).device;
    let list_type = (*allocator).list_type;
    let d3d_alloc = (*allocator).d3d12_allocator;
    let mut list: *mut c_void = ptr::null_mut();
    let hr = d3d12_dev_create_command_list((*dev).d3d12_device, list_type, d3d_alloc, &mut list);
    if hr < 0 || list.is_null() {
        return hresult_to_nri(hr);
    }
    let close_hr = d3d12_cmd_close(list);
    if close_hr < 0 {
        com_release(list);
        return hresult_to_nri(close_hr);
    }
    com_add_ref(d3d_alloc);
    wr(
        out,
        Box::into_raw(Box::new(NriCommandBuffer {
            device: dev,
            d3d12_list: list,
            d3d12_allocator: d3d_alloc,
            list_type,
            pipeline_layout: ptr::null(),
            is_graphics_layout: false,
            pipeline: ptr::null(),
            frame_buffer: ptr::null(),
            descriptor_sets: Vec::new(),
        })),
    );
    NRI_RESULT_SUCCESS
}

pub(super) unsafe extern "win64" fn core_begin_command_buffer(
    cmd: *mut NriCommandBuffer,
    pool: *const NriDescriptorPool,
    _physical_device_index: u32,
) -> u32 {
    if cmd.is_null() || (*cmd).d3d12_list.is_null() || (*cmd).d3d12_allocator.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let cb = &mut *cmd;
    let hr = d3d12_cmd_reset(cb.d3d12_list, cb.d3d12_allocator);
    if hr < 0 {
        return hresult_to_nri(hr);
    }
    if !pool.is_null() {
        core_cmd_set_descriptor_pool(cmd, pool);
    }
    cb.pipeline_layout = ptr::null();
    cb.is_graphics_layout = false;
    cb.pipeline = ptr::null();
    cb.frame_buffer = ptr::null();
    cb.descriptor_sets.clear();
    NRI_RESULT_SUCCESS
}

pub(super) unsafe extern "win64" fn core_end_command_buffer(cmd: *mut NriCommandBuffer) -> u32 {
    if cmd.is_null() || (*cmd).d3d12_list.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    hresult_to_nri(d3d12_cmd_close((*cmd).d3d12_list))
}

pub(super) unsafe extern "win64" fn core_cmd_set_pipeline(
    cmd: *mut NriCommandBuffer,
    pipeline: *const NriPipeline,
) {
    if cmd.is_null() || pipeline.is_null() {
        return;
    }
    let cb = &mut *cmd;
    if cb.pipeline == pipeline {
        return;
    }
    let p = &*pipeline;
    d3d12_cmd_set_pso(cb.d3d12_list, p.d3d12_pso);
    if p.is_graphics {
        d3d12_cmd_ia_set_topology(cb.d3d12_list, p.primitive_topology);
        if p.blend_enabled {
            d3d12_cmd_om_set_blend_factor(cb.d3d12_list, &p.blend_factor);
        }
    }
    cb.pipeline = pipeline;
}

pub(super) unsafe extern "win64" fn core_cmd_set_pipeline_layout(
    cmd: *mut NriCommandBuffer,
    layout: *const NriPipelineLayout,
) {
    if cmd.is_null() || layout.is_null() {
        return;
    }
    let cb = &mut *cmd;
    if cb.pipeline_layout == layout {
        return;
    }
    cb.pipeline_layout = layout;
    cb.is_graphics_layout = (*layout).is_graphics;
    d3d12_cmd_set_root_sig(
        cb.d3d12_list,
        (*layout).d3d12_root_sig,
        cb.is_graphics_layout,
    );
}

pub(super) unsafe extern "win64" fn core_cmd_set_descriptor_set(
    cmd: *mut NriCommandBuffer,
    set_index: u32,
    set: *const NriDescriptorSet,
    dyn_cb_offsets: *const u32,
) {
    if cmd.is_null() || set.is_null() || (*cmd).pipeline_layout.is_null() {
        return;
    }
    let cb = &mut *cmd;
    let pl = &*cb.pipeline_layout;
    let ds = &*set;
    let idx = set_index as usize;
    if let Some(root_offsets) = pl.set_root_offsets.get(idx) {
        for (j, &root_param) in root_offsets.iter().enumerate() {
            if root_param == ROOT_PARAMETER_UNUSED {
                continue;
            }
            let gpu = ds.gpu_ptr(j, 0);
            d3d12_cmd_set_root_table(
                cb.d3d12_list,
                root_param as u32,
                gpu,
                cb.is_graphics_layout,
            );
        }
    }
    if let Some(dyn_map) = pl.dynamic_cbv_mappings.get(idx) {
        for j in 0..dyn_map.count as usize {
            let off = if dyn_cb_offsets.is_null() {
                0
            } else {
                rd(dyn_cb_offsets.add(j)) as u64
            };
            let base = ds.dynamic_cbv_vas.get(j).copied().unwrap_or(0);
            d3d12_cmd_set_root_cbv(
                cb.d3d12_list,
                (dyn_map.root_offset as u32) + (j as u32),
                base.wrapping_add(off),
                cb.is_graphics_layout,
            );
        }
    }
    if cb.descriptor_sets.len() <= idx {
        cb.descriptor_sets.resize(idx + 1, ptr::null());
    }
    cb.descriptor_sets[idx] = set;
}

pub(super) unsafe extern "win64" fn core_cmd_set_constants(
    cmd: *mut NriCommandBuffer,
    push_constant_index: u32,
    data: *const u8,
    size: u32,
) {
    if cmd.is_null() || data.is_null() || size == 0 || (*cmd).pipeline_layout.is_null() {
        return;
    }
    let cb = &*cmd;
    let param = (*cb.pipeline_layout).push_constants_base_index + push_constant_index;
    let count = size / 4;
    let mut tmp = [0u32; 64];
    let n = (count as usize).min(64);
    for k in 0..n {
        tmp[k] = rd((data as *const u32).add(k));
    }
    d3d12_cmd_set_root_constants(
        cb.d3d12_list,
        param,
        n as u32,
        tmp.as_ptr(),
        0,
        cb.is_graphics_layout,
    );
}

pub(super) unsafe extern "win64" fn core_cmd_set_descriptor_pool(
    cmd: *mut NriCommandBuffer,
    pool: *const NriDescriptorPool,
) {
    if cmd.is_null() || pool.is_null() {
        return;
    }
    let p = &*pool;
    if p.heap_count > 0 {
        d3d12_cmd_set_descriptor_heaps((*cmd).d3d12_list, &p.heap_ptrs[..p.heap_count]);
    }
}

fn make_transition_or_uav_barrier(
    resource: *mut c_void,
    prev_access: u16,
    next_access: u16,
    list_type: u32,
    subresource: u32,
) -> Option<ResourceBarrier> {
    if resource.is_null() {
        return None;
    }
    let before = nri_access_to_d3d12_state(prev_access, list_type);
    let after = nri_access_to_d3d12_state(next_access, list_type);
    if before == after && before == 0x8 {
        Some(ResourceBarrier {
            Type: 2,
            Flags: 0,
            u: [resource as u64, 0, 0],
        })
    } else if before != after {
        Some(ResourceBarrier {
            Type: 0,
            Flags: 0,
            u: [
                resource as u64,
                (subresource as u64) | ((before as u64) << 32),
                after as u64,
            ],
        })
    } else {
        None
    }
}

pub(super) unsafe extern "win64" fn core_cmd_pipeline_barrier(
    cmd: *mut NriCommandBuffer,
    transitions: *const NriTransitionBarrierDesc,
    aliasing: *const NriAliasingBarrierDesc,
    _dependency: u8,
) {
    if cmd.is_null() || (*cmd).d3d12_list.is_null() {
        return;
    }
    let cb = &*cmd;
    let list_type = cb.list_type;
    let mut barriers: Vec<ResourceBarrier> = Vec::new();

    if !transitions.is_null() {
        let tr = rd(transitions);
        if tr.buffer_num > 0 && !tr.buffers.is_null() {
            for i in 0..tr.buffer_num as usize {
                let bd = rd(tr.buffers.add(i));
                if !bd.buffer.is_null() {
                    if let Some(rb) = make_transition_or_uav_barrier(
                        (*bd.buffer).d3d12_resource,
                        bd.prev_access,
                        bd.next_access,
                        list_type,
                        0,
                    ) {
                        barriers.push(rb);
                    }
                }
            }
        }
        if tr.texture_num > 0 && !tr.textures.is_null() {
            for i in 0..tr.texture_num as usize {
                let td = rd(tr.textures.add(i));
                if td.texture.is_null() {
                    continue;
                }
                let tex = &*td.texture;
                if td.array_offset == 0
                    && td.array_size == NRI_REMAINING_ARRAY_LAYERS
                    && td.mip_offset == 0
                    && td.mip_num == NRI_REMAINING_MIP_LEVELS
                {
                    if let Some(rb) = make_transition_or_uav_barrier(
                        tex.d3d12_resource,
                        td.prev_access,
                        td.next_access,
                        list_type,
                        D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
                    ) {
                        barriers.push(rb);
                    }
                } else {
                    let arr_n = if td.array_size == NRI_REMAINING_ARRAY_LAYERS {
                        tex.desc.array_size().saturating_sub(td.array_offset as u32) as u16
                    } else {
                        td.array_size
                    };
                    let mip_n = if td.mip_num == NRI_REMAINING_MIP_LEVELS {
                        tex.desc.mip_levels().saturating_sub(td.mip_offset as u32) as u16
                    } else {
                        td.mip_num
                    };
                    for a in td.array_offset..td.array_offset.saturating_add(arr_n) {
                        for m in td.mip_offset..td.mip_offset.saturating_add(mip_n) {
                            let sub = tex.subresource_index(a, m);
                            if let Some(rb) = make_transition_or_uav_barrier(
                                tex.d3d12_resource,
                                td.prev_access,
                                td.next_access,
                                list_type,
                                sub,
                            ) {
                                barriers.push(rb);
                            }
                        }
                    }
                }
            }
        }
    }

    if !aliasing.is_null() {
        let al = rd(aliasing);
        if al.buffer_num > 0 && !al.buffers.is_null() {
            for i in 0..al.buffer_num as usize {
                let bd = rd(al.buffers.add(i));
                let before = if bd.before.is_null() {
                    0
                } else {
                    (*bd.before).d3d12_resource as u64
                };
                let after = if bd.after.is_null() {
                    0
                } else {
                    (*bd.after).d3d12_resource as u64
                };
                barriers.push(ResourceBarrier {
                    Type: 1,
                    Flags: 0,
                    u: [before, after, 0],
                });
            }
        }
        if al.texture_num > 0 && !al.textures.is_null() {
            for i in 0..al.texture_num as usize {
                let td = rd(al.textures.add(i));
                let before = if td.before.is_null() {
                    0
                } else {
                    (*td.before).d3d12_resource as u64
                };
                let after = if td.after.is_null() {
                    0
                } else {
                    (*td.after).d3d12_resource as u64
                };
                barriers.push(ResourceBarrier {
                    Type: 1,
                    Flags: 0,
                    u: [before, after, 0],
                });
            }
        }
    }

    d3d12_cmd_resource_barrier(cb.d3d12_list, &barriers);
}

unsafe fn clear_fb_attachments(
    list: *mut c_void,
    fb: &NriFrameBuffer,
    clears: &[NriClearDesc],
    rects: &[Rect],
) {
    for c in clears {
        match c.attachment_content_type {
            0 => {
                if let Some(&rtv) = fb.rtvs.get(c.color_attachment_index as usize) {
                    if rtv != 0 {
                        let color = [
                            f32::from_bits(c.value[0]),
                            f32::from_bits(c.value[1]),
                            f32::from_bits(c.value[2]),
                            f32::from_bits(c.value[3]),
                        ];
                        d3d12_cmd_clear_rtv(list, rtv, &color, rects);
                    }
                }
            }
            1 | 2 | 3 => {
                if fb.dsv != 0 {
                    let flags = match c.attachment_content_type {
                        1 => 1u32,
                        2 => 2u32,
                        _ => 3u32,
                    };
                    let depth = f32::from_bits(c.value[0]);
                    let stencil = (c.value[1] & 0xff) as u8;
                    d3d12_cmd_clear_dsv(list, fb.dsv, flags, depth, stencil, rects);
                }
            }
            _ => {}
        }
    }
}

pub(super) unsafe extern "win64" fn core_cmd_begin_render_pass(
    cmd: *mut NriCommandBuffer,
    fb: *const NriFrameBuffer,
    flag: u8,
) {
    if cmd.is_null() || fb.is_null() {
        return;
    }
    let cb = &mut *cmd;
    cb.frame_buffer = fb;
    let f = &*fb;
    d3d12_cmd_om_set_render_targets(cb.d3d12_list, &f.rtvs, f.dsv);
    if flag == 0 && !f.clears.is_empty() {
        clear_fb_attachments(cb.d3d12_list, f, &f.clears, &[]);
    }
}

pub(super) unsafe extern "win64" fn core_cmd_end_render_pass(cmd: *mut NriCommandBuffer) {
    if !cmd.is_null() {
        (*cmd).frame_buffer = ptr::null();
    }
}

pub(super) unsafe extern "win64" fn core_cmd_set_viewports(
    cmd: *mut NriCommandBuffer,
    viewports: *const NriViewport,
    num: u32,
) {
    if cmd.is_null() || (num > 0 && viewports.is_null()) {
        return;
    }
    let mut vps = Vec::with_capacity(num as usize);
    for i in 0..num as usize {
        let v = rd(viewports.add(i));
        vps.push(Viewport {
            x: v.offset[0],
            y: v.offset[1],
            w: v.size[0],
            h: v.size[1],
            min_depth: v.depth_range_min,
            max_depth: v.depth_range_max,
        });
    }
    d3d12_cmd_rs_set_viewports((*cmd).d3d12_list, &vps);
}

pub(super) unsafe extern "win64" fn core_cmd_set_scissors(
    cmd: *mut NriCommandBuffer,
    rects: *const NriRect,
    num: u32,
) {
    if cmd.is_null() || (num > 0 && rects.is_null()) {
        return;
    }
    let mut drects = Vec::with_capacity(num as usize);
    for i in 0..num as usize {
        let r = rd(rects.add(i));
        drects.push(Rect {
            left: r.left,
            top: r.top,
            right: r.left.saturating_add(r.width as i32),
            bottom: r.top.saturating_add(r.height as i32),
        });
    }
    d3d12_cmd_rs_set_scissors((*cmd).d3d12_list, &drects);
}

unsafe extern "win64" fn core_cmd_set_depth_bounds(
    cmd: *mut NriCommandBuffer,
    bounds_min: f32,
    bounds_max: f32,
) {
    if !cmd.is_null() && !(*cmd).d3d12_list.is_null() {
        d3d12_cmd_set_depth_bounds((*cmd).d3d12_list, bounds_min, bounds_max);
    }
}

crate::win64_bridge!(fn core_cmd_set_depth_bounds(*mut NriCommandBuffer, f32, f32) -> ());

#[cfg(target_arch = "aarch64")]
pub(super) const CMD_SET_DEPTH_BOUNDS_ENTRY: *const () =
    core_cmd_set_depth_bounds::bridge as *const ();
#[cfg(not(target_arch = "aarch64"))]
pub(super) const CMD_SET_DEPTH_BOUNDS_ENTRY: *const () = core_cmd_set_depth_bounds as *const ();

pub(super) unsafe extern "win64" fn core_cmd_set_stencil_reference(
    cmd: *mut NriCommandBuffer,
    reference: u8,
) {
    if !cmd.is_null() && !(*cmd).d3d12_list.is_null() {
        d3d12_cmd_om_set_stencil_ref((*cmd).d3d12_list, reference as u32);
    }
}

pub(super) unsafe extern "win64" fn core_cmd_set_sample_positions(
    _cmd: *mut NriCommandBuffer,
    _positions: *const u8,
    _position_num: u32,
) {
    crate::stub!("CmdSetSamplePositions: programmable sample positions not supported on Metal");
}

pub(super) unsafe extern "win64" fn core_cmd_clear_attachments(
    cmd: *mut NriCommandBuffer,
    clear_descs: *const NriClearDesc,
    clear_num: u32,
    rects: *const NriRect,
    rect_num: u32,
) {
    if cmd.is_null() || (*cmd).frame_buffer.is_null() || clear_descs.is_null() || clear_num == 0 {
        return;
    }
    let mut clears = Vec::with_capacity(clear_num as usize);
    for i in 0..clear_num as usize {
        clears.push(rd(clear_descs.add(i)));
    }
    let mut drects = Vec::with_capacity(rect_num as usize);
    if !rects.is_null() {
        for i in 0..rect_num as usize {
            let r = rd(rects.add(i));
            drects.push(Rect {
                left: r.left,
                top: r.top,
                right: r.left.saturating_add(r.width as i32),
                bottom: r.top.saturating_add(r.height as i32),
            });
        }
    }
    clear_fb_attachments((*cmd).d3d12_list, &*(*cmd).frame_buffer, &clears, &drects);
}

pub(super) unsafe extern "win64" fn core_cmd_set_index_buffer(
    cmd: *mut NriCommandBuffer,
    buffer: *const NriBuffer,
    offset: u64,
    index_type: u8,
) {
    if cmd.is_null() || buffer.is_null() || (*buffer).d3d12_resource.is_null() {
        return;
    }
    let b = &*buffer;
    let view = IndexBufferView {
        va: d3d12_res_get_gpu_va(b.d3d12_resource).wrapping_add(offset),
        size: b.desc.Width.saturating_sub(offset) as u32,
        format: if index_type == 0 { 57 } else { 42 }, // R16_UINT : R32_UINT
    };
    d3d12_cmd_ia_set_index_buffer((*cmd).d3d12_list, &view);
}

pub(super) unsafe extern "win64" fn core_cmd_set_vertex_buffers(
    cmd: *mut NriCommandBuffer,
    base_slot: u32,
    buffer_num: u32,
    buffers: *const *const NriBuffer,
    offsets: *const u64,
) {
    if cmd.is_null() || buffers.is_null() || buffer_num == 0 {
        return;
    }
    let cb = &*cmd;
    let mut views = Vec::with_capacity(buffer_num as usize);
    for i in 0..buffer_num as usize {
        let bp = rd(buffers.add(i));
        if bp.is_null() || (*bp).d3d12_resource.is_null() {
            views.push(VertexBufferView {
                va: 0,
                size: 0,
                stride: 0,
            });
        } else {
            let b = &*bp;
            let off = if offsets.is_null() {
                0
            } else {
                rd(offsets.add(i))
            };
            let stride = if !cb.pipeline.is_null() {
                (&(*cb.pipeline).stream_strides)
                    .get((base_slot as usize) + i)
                    .copied()
                    .unwrap_or(b.structure_stride)
            } else {
                b.structure_stride
            };
            views.push(VertexBufferView {
                va: d3d12_res_get_gpu_va(b.d3d12_resource).wrapping_add(off),
                size: b.desc.Width.saturating_sub(off) as u32,
                stride,
            });
        }
    }
    d3d12_cmd_ia_set_vertex_buffers(cb.d3d12_list, base_slot, &views);
}

pub(super) unsafe extern "win64" fn core_cmd_draw(
    cmd: *mut NriCommandBuffer,
    vertex_num: u32,
    instance_num: u32,
    base_vertex: u32,
    base_instance: u32,
) {
    if !cmd.is_null() && !(*cmd).d3d12_list.is_null() {
        d3d12_cmd_draw(
            (*cmd).d3d12_list,
            vertex_num,
            instance_num,
            base_vertex,
            base_instance,
        );
    }
}

pub(super) unsafe extern "win64" fn core_cmd_draw_indexed(
    cmd: *mut NriCommandBuffer,
    index_num: u32,
    instance_num: u32,
    base_index: u32,
    base_vertex: i32,
    base_instance: u32,
) {
    if !cmd.is_null() && !(*cmd).d3d12_list.is_null() {
        d3d12_cmd_draw_indexed(
            (*cmd).d3d12_list,
            index_num,
            instance_num,
            base_index,
            base_vertex,
            base_instance,
        );
    }
}

pub(super) unsafe extern "win64" fn core_cmd_draw_indirect(
    cmd: *mut NriCommandBuffer,
    buffer: *const NriBuffer,
    offset: u64,
    draw_num: u32,
    stride: u32,
) {
    if cmd.is_null() || buffer.is_null() || (*buffer).d3d12_resource.is_null() {
        return;
    }
    let sig = (*(*cmd).device).get_indirect_signature(0, stride);
    if !sig.is_null() {
        d3d12_cmd_execute_indirect(
            (*cmd).d3d12_list,
            sig,
            draw_num,
            (*buffer).d3d12_resource,
            offset,
        );
    }
}

pub(super) unsafe extern "win64" fn core_cmd_draw_indexed_indirect(
    cmd: *mut NriCommandBuffer,
    buffer: *const NriBuffer,
    offset: u64,
    draw_num: u32,
    stride: u32,
) {
    if cmd.is_null() || buffer.is_null() || (*buffer).d3d12_resource.is_null() {
        return;
    }
    let sig = (*(*cmd).device).get_indirect_signature(1, stride);
    if !sig.is_null() {
        d3d12_cmd_execute_indirect(
            (*cmd).d3d12_list,
            sig,
            draw_num,
            (*buffer).d3d12_resource,
            offset,
        );
    }
}

pub(super) unsafe extern "win64" fn core_cmd_dispatch(
    cmd: *mut NriCommandBuffer,
    x: u32,
    y: u32,
    z: u32,
) {
    if !cmd.is_null() && !(*cmd).d3d12_list.is_null() {
        d3d12_cmd_dispatch((*cmd).d3d12_list, x, y, z);
    }
}

pub(super) unsafe extern "win64" fn core_cmd_dispatch_indirect(
    cmd: *mut NriCommandBuffer,
    buffer: *const NriBuffer,
    offset: u64,
) {
    if cmd.is_null() || buffer.is_null() || (*buffer).d3d12_resource.is_null() {
        return;
    }
    let sig = (*(*cmd).device).get_indirect_signature(2, 12);
    if !sig.is_null() {
        d3d12_cmd_execute_indirect((*cmd).d3d12_list, sig, 1, (*buffer).d3d12_resource, offset);
    }
}

pub(super) unsafe extern "win64" fn core_cmd_begin_query(
    cmd: *mut NriCommandBuffer,
    pool: *const NriQueryPool,
    offset: u32,
) {
    if cmd.is_null() || pool.is_null() {
        return;
    }
    d3d12_cmd_begin_query(
        (*cmd).d3d12_list,
        (*pool).d3d12_query_heap,
        (*pool).d3d12_query_type,
        offset,
    );
}

pub(super) unsafe extern "win64" fn core_cmd_end_query(
    cmd: *mut NriCommandBuffer,
    pool: *const NriQueryPool,
    offset: u32,
) {
    if cmd.is_null() || pool.is_null() {
        return;
    }
    d3d12_cmd_end_query(
        (*cmd).d3d12_list,
        (*pool).d3d12_query_heap,
        (*pool).d3d12_query_type,
        offset,
    );
}

pub(super) unsafe extern "win64" fn core_cmd_begin_annotation(
    cmd: *mut NriCommandBuffer,
    name: *const u8,
) {
    if !cmd.is_null() && !(*cmd).d3d12_list.is_null() {
        d3d12_cmd_begin_event((*cmd).d3d12_list, name);
    }
}

pub(super) unsafe extern "win64" fn core_cmd_end_annotation(cmd: *mut NriCommandBuffer) {
    if !cmd.is_null() && !(*cmd).d3d12_list.is_null() {
        d3d12_cmd_end_event((*cmd).d3d12_list);
    }
}

pub(super) unsafe extern "win64" fn core_cmd_clear_storage_buffer(
    cmd: *mut NriCommandBuffer,
    desc_ptr: *const NriClearStorageBufferDesc,
) {
    if cmd.is_null() || desc_ptr.is_null() {
        return;
    }
    let cd = rd(desc_ptr);
    if cd.storage_buffer.is_null() {
        return;
    }
    let view = &*cd.storage_buffer;
    if view.d3d12_resource.is_null() {
        return;
    }
    let gpu = (&(*cmd).descriptor_sets)
        .get(cd.set_index_in_pipeline_layout as usize)
        .copied()
        .filter(|p| !p.is_null())
        .map_or(0, |ds| unsafe {
            (*ds).gpu_ptr(cd.range_index as usize, cd.offset_in_range)
        });
    let vals = [cd.value; 4];
    d3d12_cmd_clear_uav_uint(
        (*cmd).d3d12_list,
        gpu,
        view.cpu_handle,
        view.d3d12_resource,
        &vals,
    );
}

pub(super) unsafe extern "win64" fn core_cmd_clear_storage_texture(
    cmd: *mut NriCommandBuffer,
    desc_ptr: *const NriClearStorageTextureDesc,
) {
    if cmd.is_null() || desc_ptr.is_null() {
        return;
    }
    let cd = rd(desc_ptr);
    if cd.storage_texture.is_null() {
        return;
    }
    let view = &*cd.storage_texture;
    if view.d3d12_resource.is_null() {
        return;
    }
    let gpu = (&(*cmd).descriptor_sets)
        .get(cd.set_index_in_pipeline_layout as usize)
        .copied()
        .filter(|p| !p.is_null())
        .map_or(0, |ds| unsafe {
            (*ds).gpu_ptr(cd.range_index as usize, cd.offset_in_range)
        });
    if view.is_float_uav {
        let vals = [
            f32::from_bits(cd.value[0]),
            f32::from_bits(cd.value[1]),
            f32::from_bits(cd.value[2]),
            f32::from_bits(cd.value[3]),
        ];
        d3d12_cmd_clear_uav_float(
            (*cmd).d3d12_list,
            gpu,
            view.cpu_handle,
            view.d3d12_resource,
            &vals,
        );
    } else {
        d3d12_cmd_clear_uav_uint(
            (*cmd).d3d12_list,
            gpu,
            view.cpu_handle,
            view.d3d12_resource,
            &cd.value,
        );
    }
}

pub(super) unsafe extern "win64" fn core_cmd_copy_buffer(
    cmd: *mut NriCommandBuffer,
    dst: *mut NriBuffer,
    _dst_dev_idx: u32,
    dst_offset: u64,
    src: *const NriBuffer,
    _src_dev_idx: u32,
    src_offset: u64,
    size: u64,
) {
    if cmd.is_null() || dst.is_null() || src.is_null() {
        return;
    }
    let (dres, sres) = ((*dst).d3d12_resource, (*src).d3d12_resource);
    if dres.is_null() || sres.is_null() {
        return;
    }
    let bytes = if size == NRI_WHOLE_SIZE {
        (*src).desc.Width.saturating_sub(src_offset)
    } else {
        size
    };
    d3d12_cmd_copy_buffer_region((*cmd).d3d12_list, dres, dst_offset, sres, src_offset, bytes);
}

pub(super) unsafe extern "win64" fn core_cmd_copy_texture(
    cmd: *mut NriCommandBuffer,
    dst: *mut NriTexture,
    _dst_dev_idx: u32,
    dst_region: *const NriTextureRegionDesc,
    src: *const NriTexture,
    _src_dev_idx: u32,
    src_region: *const NriTextureRegionDesc,
) {
    if cmd.is_null() || dst.is_null() || src.is_null() {
        return;
    }
    let (dt, st) = (&*dst, &*src);
    if dt.d3d12_resource.is_null() || st.d3d12_resource.is_null() {
        return;
    }
    if dst_region.is_null() || src_region.is_null() {
        d3d12_cmd_copy_resource((*cmd).d3d12_list, dt.d3d12_resource, st.d3d12_resource);
        return;
    }
    let dr = rd(dst_region);
    let sr = rd(src_region);
    let dst_loc = CopyLocation {
        resource: dt.d3d12_resource,
        ty: 0,
        _pad: 0,
        u: PlacedFootprint {
            offset: dt.subresource_index(dr.array_offset, dr.mip_offset) as u64,
            ..Default::default()
        },
    };
    let src_loc = CopyLocation {
        resource: st.d3d12_resource,
        ty: 0,
        _pad: 0,
        u: PlacedFootprint {
            offset: st.subresource_index(sr.array_offset, sr.mip_offset) as u64,
            ..Default::default()
        },
    };
    let sz = [
        if sr.size[0] as u64 == NRI_WHOLE_SIZE {
            st.dim_size(0, sr.mip_offset)
        } else {
            sr.size[0]
        },
        if sr.size[1] as u64 == NRI_WHOLE_SIZE {
            st.dim_size(1, sr.mip_offset)
        } else {
            sr.size[1]
        },
        if sr.size[2] as u64 == NRI_WHOLE_SIZE {
            st.dim_size(2, sr.mip_offset)
        } else {
            sr.size[2]
        },
    ];
    let bx = CopyBox {
        left: sr.offset[0] as u32,
        top: sr.offset[1] as u32,
        front: sr.offset[2] as u32,
        right: (sr.offset[0] as u32) + (sz[0] as u32),
        bottom: (sr.offset[1] as u32) + (sz[1] as u32),
        back: (sr.offset[2] as u32) + (sz[2] as u32),
    };
    d3d12_cmd_copy_texture_region(
        (*cmd).d3d12_list,
        &dst_loc,
        dr.offset[0] as u32,
        dr.offset[1] as u32,
        dr.offset[2] as u32,
        &src_loc,
        &bx,
    );
}

pub(super) unsafe extern "win64" fn core_cmd_upload_buffer_to_texture(
    cmd: *mut NriCommandBuffer,
    dst: *mut NriTexture,
    dst_region_ptr: *const NriTextureRegionDesc,
    src: *const NriBuffer,
    src_layout_ptr: *const NriTextureDataLayoutDesc,
) {
    if cmd.is_null()
        || dst.is_null()
        || dst_region_ptr.is_null()
        || src.is_null()
        || src_layout_ptr.is_null()
    {
        return;
    }
    let (dt, sb) = (&*dst, &*src);
    if dt.d3d12_resource.is_null() || sb.d3d12_resource.is_null() {
        return;
    }
    let dr = rd(dst_region_ptr);
    let sl = rd(src_layout_ptr);
    let sz = [
        if dr.size[0] as u64 == NRI_WHOLE_SIZE {
            dt.dim_size(0, dr.mip_offset)
        } else {
            dr.size[0]
        },
        if dr.size[1] as u64 == NRI_WHOLE_SIZE {
            dt.dim_size(1, dr.mip_offset)
        } else {
            dr.size[1]
        },
        if dr.size[2] as u64 == NRI_WHOLE_SIZE {
            dt.dim_size(2, dr.mip_offset)
        } else {
            dr.size[2]
        },
    ];
    let dst_loc = CopyLocation {
        resource: dt.d3d12_resource,
        ty: 0,
        _pad: 0,
        u: PlacedFootprint {
            offset: dt.subresource_index(dr.array_offset, dr.mip_offset) as u64,
            ..Default::default()
        },
    };
    let src_loc = CopyLocation {
        resource: sb.d3d12_resource,
        ty: 1,
        _pad: 0,
        u: PlacedFootprint {
            offset: sl.offset,
            format: dt.desc.Format,
            width: sz[0] as u32,
            height: sz[1] as u32,
            depth: sz[2] as u32,
            row_pitch: sl.row_pitch,
            _pad: 0,
        },
    };
    let bx = CopyBox {
        left: 0,
        top: 0,
        front: 0,
        right: sz[0] as u32,
        bottom: sz[1] as u32,
        back: sz[2] as u32,
    };
    d3d12_cmd_copy_texture_region(
        (*cmd).d3d12_list,
        &dst_loc,
        dr.offset[0] as u32,
        dr.offset[1] as u32,
        dr.offset[2] as u32,
        &src_loc,
        &bx,
    );
}

pub(super) unsafe extern "win64" fn core_cmd_readback_texture_to_buffer(
    cmd: *mut NriCommandBuffer,
    dst: *mut NriBuffer,
    dst_layout_ptr: *const NriTextureDataLayoutDesc,
    src: *const NriTexture,
    src_region_ptr: *const NriTextureRegionDesc,
) {
    if cmd.is_null()
        || dst.is_null()
        || dst_layout_ptr.is_null()
        || src.is_null()
        || src_region_ptr.is_null()
    {
        return;
    }
    let (db, st) = (&*dst, &*src);
    if db.d3d12_resource.is_null() || st.d3d12_resource.is_null() {
        return;
    }
    let dl = rd(dst_layout_ptr);
    let sr = rd(src_region_ptr);
    let sz = [
        if sr.size[0] as u64 == NRI_WHOLE_SIZE {
            st.dim_size(0, sr.mip_offset)
        } else {
            sr.size[0]
        },
        if sr.size[1] as u64 == NRI_WHOLE_SIZE {
            st.dim_size(1, sr.mip_offset)
        } else {
            sr.size[1]
        },
        if sr.size[2] as u64 == NRI_WHOLE_SIZE {
            st.dim_size(2, sr.mip_offset)
        } else {
            sr.size[2]
        },
    ];
    let src_loc = CopyLocation {
        resource: st.d3d12_resource,
        ty: 0,
        _pad: 0,
        u: PlacedFootprint {
            offset: st.subresource_index(sr.array_offset, sr.mip_offset) as u64,
            ..Default::default()
        },
    };
    let dst_loc = CopyLocation {
        resource: db.d3d12_resource,
        ty: 1,
        _pad: 0,
        u: PlacedFootprint {
            offset: dl.offset,
            format: st.desc.Format,
            width: sz[0] as u32,
            height: sz[1] as u32,
            depth: sz[2] as u32,
            row_pitch: dl.row_pitch,
            _pad: 0,
        },
    };
    let bx = CopyBox {
        left: sr.offset[0] as u32,
        top: sr.offset[1] as u32,
        front: sr.offset[2] as u32,
        right: (sr.offset[0] as u32) + (sz[0] as u32),
        bottom: (sr.offset[1] as u32) + (sz[1] as u32),
        back: (sr.offset[2] as u32) + (sz[2] as u32),
    };
    d3d12_cmd_copy_texture_region((*cmd).d3d12_list, &dst_loc, 0, 0, 0, &src_loc, &bx);
}

pub(super) unsafe extern "win64" fn core_cmd_copy_queries(
    cmd: *mut NriCommandBuffer,
    pool: *const NriQueryPool,
    offset: u32,
    num: u32,
    dst: *mut NriBuffer,
    dst_offset: u64,
) {
    if cmd.is_null() || pool.is_null() || dst.is_null() || (*dst).d3d12_resource.is_null() {
        return;
    }
    d3d12_cmd_resolve_query(
        (*cmd).d3d12_list,
        (*pool).d3d12_query_heap,
        (*pool).d3d12_query_type,
        offset,
        num,
        (*dst).d3d12_resource,
        dst_offset,
    );
}

pub(super) unsafe extern "win64" fn core_cmd_reset_queries(
    _cmd: *mut NriCommandBuffer,
    _pool: *const NriQueryPool,
    _offset: u32,
    _num: u32,
) {
    // D3D12 query heaps do not require explicit reset commands.
}

// ---------------------------------------------------------------------------
// CoreInterface — Synchronization & Work submission (slots 73..78)
// ---------------------------------------------------------------------------

pub(super) unsafe extern "win64" fn core_get_fence_value(fence: *const NriFence) -> u64 {
    if fence.is_null() || (*fence).d3d12_fence.is_null() {
        return 0;
    }
    d3d12_fence_completed((*fence).d3d12_fence)
}

pub(super) unsafe extern "win64" fn core_queue_signal(
    queue: *mut NriCommandQueue,
    fence: *mut NriFence,
    value: u64,
) {
    if queue.is_null() || fence.is_null() {
        return;
    }
    let _ = d3d12_queue_signal((*queue).d3d12_queue, (*fence).d3d12_fence, value);
}

pub(super) unsafe extern "win64" fn core_queue_wait(
    queue: *mut NriCommandQueue,
    fence: *mut NriFence,
    value: u64,
) {
    if queue.is_null() || fence.is_null() {
        return;
    }
    let _ = d3d12_queue_wait((*queue).d3d12_queue, (*fence).d3d12_fence, value);
}

pub(super) unsafe extern "win64" fn core_wait(fence: *mut NriFence, value: u64) {
    if fence.is_null() || (*fence).d3d12_fence.is_null() {
        return;
    }
    cs::flush();
    if d3d12_fence_completed((*fence).d3d12_fence) < value {
        let _ = d3d12_fence_set_event((*fence).d3d12_fence, value, ptr::null_mut());
    }
}

pub(super) unsafe extern "win64" fn core_queue_submit(
    queue: *mut NriCommandQueue,
    desc_ptr: *const NriQueueSubmitDesc,
) {
    if queue.is_null() || desc_ptr.is_null() {
        return;
    }
    let sd = rd(desc_ptr);
    if sd.command_buffer_num == 0 || sd.command_buffers.is_null() {
        return;
    }
    let mut lists = Vec::with_capacity(sd.command_buffer_num as usize);
    for i in 0..sd.command_buffer_num as usize {
        let cb = rd(sd.command_buffers.add(i));
        if !cb.is_null() && !(*cb).d3d12_list.is_null() {
            lists.push((*cb).d3d12_list);
        }
    }
    if !lists.is_empty() {
        d3d12_queue_execute((*queue).d3d12_queue, &lists);
    }
}
