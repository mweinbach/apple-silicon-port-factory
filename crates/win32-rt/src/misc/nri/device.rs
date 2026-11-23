//! NRI runtime objects (`NriDevice`, queues, allocators, memory, buffers,
//! textures, pipelines, framebuffers, query pools, fences, acceleration
//! structures) and `CoreInterface` device/resource/memory/pipeline callbacks.

use std::collections::HashMap;
use std::ffi::c_void;
use std::mem::size_of;
use std::ptr;
use std::sync::{Mutex, Once};

use super::abi::*;
use super::cmdlist::NriCommandBuffer;
use super::descriptor::{NriDescriptor, NriDescriptorPool, NriPipelineLayout};
use crate::d3d12::com::{com_add_ref, com_release};
use crate::d3d12::heap::QueryHeapDesc;
use crate::d3d12::mtl;
use crate::d3d12::pso::{
    Bytecode, CommandSignatureDesc, ComputePipelineDesc, IndirectArgument, InputElementDesc,
    StreamDesc,
};
use crate::strings::{le_u32, rd, wr};
use d3d12_metal::format::{self as d3d_fmt, support1, support2};
use d3d12_metal::state::{BlendDesc, RenderTargetBlendDesc};
use d3d12_metal::types::{
    dim, heap_type, resource_flags, ClearValue, CommandQueueDesc, DescriptorHeapDesc, HeapDesc,
    HeapProperties, ResourceDesc,
};

// ---------------------------------------------------------------------------
// NRI runtime objects backed by real D3D12 COM objects
// ---------------------------------------------------------------------------

struct CpuHeapChunk {
    heap: *mut c_void,
    cpu_start: usize,
    inc_size: u32,
}

impl Drop for CpuHeapChunk {
    fn drop(&mut self) {
        unsafe { com_release(self.heap) };
    }
}

#[derive(Default)]
pub(super) struct CpuDescriptorAllocator {
    chunks: Vec<CpuHeapChunk>,
    free: Vec<(usize, u32)>,
}

pub struct NriDevice {
    pub(super) d3d12_device: *mut c_void,
    pub(super) desc: NriDeviceDesc,
    pub(super) queues: Mutex<[Option<*mut NriCommandQueue>; 3]>,
    pub(super) cpu_heaps: Mutex<[CpuDescriptorAllocator; 4]>,
    pub(super) dispatch_cmd_sig: Mutex<*mut c_void>,
    pub(super) draw_cmd_sigs: Mutex<HashMap<u32, *mut c_void>>,
    pub(super) draw_indexed_cmd_sigs: Mutex<HashMap<u32, *mut c_void>>,
}

impl Drop for NriDevice {
    fn drop(&mut self) {
        unsafe {
            if let Ok(mut qs) = self.queues.lock() {
                for slot in qs.iter_mut() {
                    if let Some(q) = slot.take() {
                        drop(Box::from_raw(q));
                    }
                }
            }
            if let Ok(mut sig) = self.dispatch_cmd_sig.lock() {
                if !sig.is_null() {
                    com_release(*sig);
                    *sig = ptr::null_mut();
                }
            }
            if let Ok(mut map) = self.draw_cmd_sigs.lock() {
                for (_, p) in map.drain() {
                    com_release(p);
                }
            }
            if let Ok(mut map) = self.draw_indexed_cmd_sigs.lock() {
                for (_, p) in map.drain() {
                    com_release(p);
                }
            }
            if let Ok(mut heaps) = self.cpu_heaps.lock() {
                for h in heaps.iter_mut() {
                    h.chunks.clear();
                    h.free.clear();
                }
            }
            com_release(self.d3d12_device);
        }
    }
}

impl NriDevice {
    pub(super) unsafe fn alloc_cpu_descriptor(
        &self,
        heap_type: u32,
    ) -> Result<(usize, u32, usize), u32> {
        let idx = heap_type as usize;
        if idx >= 4 {
            return Err(NRI_RESULT_INVALID_ARGUMENT);
        }
        let mut guard = self.cpu_heaps.lock().unwrap();
        let pool = &mut guard[idx];
        if pool.free.is_empty() {
            let mut heap: *mut c_void = ptr::null_mut();
            let desc = DescriptorHeapDesc {
                Type: heap_type,
                NumDescriptors: CPU_DESCRIPTOR_BATCH_SIZE,
                Flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
                NodeMask: 0,
            };
            let hr = d3d12_dev_create_descriptor_heap(self.d3d12_device, &desc, &mut heap);
            if hr < 0 || heap.is_null() {
                return Err(hresult_to_nri(hr));
            }
            let cpu_start = d3d12_descheap_cpu_start(heap);
            let inc_size = d3d12_dev_get_desc_inc_size(self.d3d12_device, heap_type);
            let chunk_idx = pool.chunks.len();
            pool.chunks.push(CpuHeapChunk {
                heap,
                cpu_start,
                inc_size,
            });
            for slot in (0..CPU_DESCRIPTOR_BATCH_SIZE).rev() {
                pool.free.push((chunk_idx, slot));
            }
        }
        let (chunk_idx, slot_idx) = pool.free.pop().unwrap();
        let chunk = &pool.chunks[chunk_idx];
        let handle = chunk.cpu_start + (slot_idx as usize) * (chunk.inc_size as usize);
        Ok((chunk_idx, slot_idx, handle))
    }

    pub(super) fn free_cpu_descriptor(&self, heap_type: u32, chunk_idx: usize, slot_idx: u32) {
        if let Ok(mut guard) = self.cpu_heaps.lock() {
            if let Some(pool) = guard.get_mut(heap_type as usize) {
                if chunk_idx < pool.chunks.len() {
                    pool.free.push((chunk_idx, slot_idx));
                }
            }
        }
    }

    pub(super) unsafe fn get_or_create_queue(
        this: *mut NriDevice,
        nri_type: u8,
    ) -> Result<*mut NriCommandQueue, u32> {
        let Some(d3d_type) = nri_queue_type_to_d3d12(nri_type) else {
            return Err(NRI_RESULT_INVALID_ARGUMENT);
        };
        let dev = &*this;
        let mut qs = dev.queues.lock().unwrap();
        if let Some(existing) = qs[nri_type as usize] {
            return Ok(existing);
        }
        let desc = CommandQueueDesc {
            Type: d3d_type,
            Priority: 0,
            Flags: 0,
            NodeMask: 0,
        };
        let mut d3d_queue: *mut c_void = ptr::null_mut();
        let hr = d3d12_dev_create_command_queue(dev.d3d12_device, &desc, &mut d3d_queue);
        if hr < 0 || d3d_queue.is_null() {
            return Err(hresult_to_nri(hr));
        }
        let q = Box::into_raw(Box::new(NriCommandQueue {
            device: this,
            d3d12_queue: d3d_queue,
            list_type: d3d_type,
        }));
        qs[nri_type as usize] = Some(q);
        Ok(q)
    }

    pub(super) unsafe fn get_indirect_signature(&self, arg_type: u32, stride: u32) -> *mut c_void {
        let create = || -> *mut c_void {
            let arg = IndirectArgument {
                ty: arg_type,
                u: [0; 3],
            };
            let desc = CommandSignatureDesc {
                byte_stride: stride,
                num_args: 1,
                args: &arg,
                node_mask: 0,
            };
            let mut sig: *mut c_void = ptr::null_mut();
            let hr =
                unsafe { d3d12_dev_create_command_signature(self.d3d12_device, &desc, &mut sig) };
            if hr >= 0 { sig } else { ptr::null_mut() }
        };
        match arg_type {
            2 => {
                // D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH
                let mut g = self.dispatch_cmd_sig.lock().unwrap();
                if g.is_null() {
                    *g = create();
                }
                *g
            }
            0 => {
                // DRAW
                let mut g = self.draw_cmd_sigs.lock().unwrap();
                *g.entry(stride).or_insert_with(create)
            }
            _ => {
                // DRAW_INDEXED
                let mut g = self.draw_indexed_cmd_sigs.lock().unwrap();
                *g.entry(stride).or_insert_with(create)
            }
        }
    }
}

pub struct NriCommandQueue {
    pub(super) device: *mut NriDevice,
    pub(super) d3d12_queue: *mut c_void,
    pub(super) list_type: u32,
}

impl Drop for NriCommandQueue {
    fn drop(&mut self) {
        unsafe { com_release(self.d3d12_queue) };
    }
}

pub struct NriCommandAllocator {
    pub(super) device: *mut NriDevice,
    pub(super) d3d12_allocator: *mut c_void,
    pub(super) list_type: u32,
}

impl Drop for NriCommandAllocator {
    fn drop(&mut self) {
        unsafe { com_release(self.d3d12_allocator) };
    }
}

pub struct NriMemory {
    pub(super) d3d12_heap: *mut c_void,
    pub(super) heap_desc: HeapDesc,
    pub(super) dedicated: bool,
}

impl Drop for NriMemory {
    fn drop(&mut self) {
        unsafe { com_release(self.d3d12_heap) };
    }
}

pub struct NriBuffer {
    pub(super) device: *mut NriDevice,
    pub(super) d3d12_resource: *mut c_void,
    pub(super) desc: ResourceDesc,
    pub(super) structure_stride: u32,
}

impl Drop for NriBuffer {
    fn drop(&mut self) {
        unsafe { com_release(self.d3d12_resource) };
    }
}

pub struct NriTexture {
    pub(super) device: *mut NriDevice,
    pub(super) d3d12_resource: *mut c_void,
    pub(super) desc: ResourceDesc,
    pub(super) nri_format: u8,
}

impl NriTexture {
    pub(super) fn subresource_index(&self, array_offset: u16, mip_offset: u16) -> u32 {
        mip_offset as u32 + (array_offset as u32) * (self.desc.mip_levels())
    }

    pub(super) fn dim_size(&self, axis: usize, mip_offset: u16) -> u16 {
        let base = match axis {
            0 => self.desc.Width as u32,
            1 => self.desc.Height,
            _ => self.desc.depth(),
        };
        let shifted = (base >> (mip_offset.min(15) as u32)).max(1) as u16;
        if axis < 2 {
            let bw = nri_texel_block_width(self.nri_format);
            align_up(shifted as u64, bw as u64) as u16
        } else {
            shifted
        }
    }
}

impl Drop for NriTexture {
    fn drop(&mut self) {
        unsafe { com_release(self.d3d12_resource) };
    }
}

pub struct NriPipeline {
    pub(super) d3d12_pso: *mut c_void,
    pub(super) is_graphics: bool,
    pub(super) primitive_topology: u32,
    pub(super) stream_strides: Vec<u32>,
    pub(super) blend_enabled: bool,
    pub(super) blend_factor: [f32; 4],
}

impl Drop for NriPipeline {
    fn drop(&mut self) {
        unsafe { com_release(self.d3d12_pso) };
    }
}

pub struct NriFrameBuffer {
    pub(super) rtvs: Vec<usize>,
    pub(super) dsv: usize,
    pub(super) clears: Vec<NriClearDesc>,
}

pub struct NriQueryPool {
    pub(super) d3d12_query_heap: *mut c_void,
    pub(super) d3d12_query_type: u32,
    pub(super) query_size: u32,
}

impl Drop for NriQueryPool {
    fn drop(&mut self) {
        unsafe { com_release(self.d3d12_query_heap) };
    }
}

pub struct NriFence {
    pub(super) d3d12_fence: *mut c_void,
}

impl Drop for NriFence {
    fn drop(&mut self) {
        unsafe { com_release(self.d3d12_fence) };
    }
}

pub struct NriAccelerationStructure {
    pub(super) d3d12_resource: *mut c_void,
    pub(super) _scratch_size: u64,
    pub(super) _update_scratch_size: u64,
}

impl Drop for NriAccelerationStructure {
    fn drop(&mut self) {
        unsafe { com_release(self.d3d12_resource) };
    }
}

// ---------------------------------------------------------------------------
// Honest `NriDeviceDesc` population from real D3D12 device / Metal adapter
// ---------------------------------------------------------------------------

unsafe fn build_device_desc(
    d3d12_device: *mut c_void,
    graphics_queue: *mut c_void,
    enable_validation: bool,
) -> NriDeviceDesc {
    let mut description = [0u8; 128];
    let (adapter_name, luid, vram, sys_mem, device_id) = match mtl::device() {
        Some(m) => {
            let v = m.reported_vram();
            let s = mtl::physical_memory().saturating_sub(v);
            (m.name.as_str(), m.registry_id, v, s, m.pci_device_id())
        }
        None => ("Metal 4 Device", 0, 0, 0, 0),
    };
    let name_bytes = adapter_name.as_bytes();
    let copy_n = name_bytes.len().min(127);
    description[..copy_n].copy_from_slice(&name_bytes[..copy_n]);

    // Query real D3D12 feature capabilities via CheckFeatureSupport.
    let mut opts0 = [0u8; 60];
    let _ = d3d12_dev_check_feature(d3d12_device, 0, &mut opts0);
    let logic_op = le_u32(&opts0, 4).unwrap_or(0) != 0;
    let conservative_raster_tier = le_u32(&opts0, 32).unwrap_or(0) as u8;

    let mut opts2 = [0u8; 8];
    let _ = d3d12_dev_check_feature(d3d12_device, 18, &mut opts2);
    let depth_bounds_supported = le_u32(&opts2, 0).unwrap_or(0) != 0;
    let prog_sample_pos = le_u32(&opts2, 4).unwrap_or(0) != 0;

    let mut opts3 = [0u8; 20];
    let _ = d3d12_dev_check_feature(d3d12_device, 21, &mut opts3);
    let copy_queue_ts = le_u32(&opts3, 0).unwrap_or(0) != 0;

    let mut opts4 = [0u8; 12];
    let _ = d3d12_dev_check_feature(d3d12_device, 23, &mut opts4);
    let float16 = le_u32(&opts4, 8).unwrap_or(0) != 0;

    let mut opts5 = [0u8; 12];
    let _ = d3d12_dev_check_feature(d3d12_device, 27, &mut opts5);
    let rt_tier = le_u32(&opts5, 8).unwrap_or(0);
    let rt_supported = rt_tier >= 10;

    let mut opts7 = [0u8; 8];
    let _ = d3d12_dev_check_feature(d3d12_device, 32, &mut opts7);
    let mesh_supported = le_u32(&opts7, 0).unwrap_or(0) >= 10;

    let ts_freq = if !graphics_queue.is_null() {
        d3d12_queue_get_timestamp_freq(graphics_queue)
    } else {
        d3d12_metal::metal::query::timestamp_frequency()
    };
    let node_count = d3d12_dev_get_node_count(d3d12_device).max(1) as u8;

    NriDeviceDesc {
        adapter_desc: NriAdapterDesc {
            description,
            luid,
            video_memory_size: vram,
            system_memory_size: sys_mem,
            device_id,
            vendor: 0, // Vendor::UNKNOWN (NRI enum only defines UNKNOWN/NVIDIA/AMD/INTEL)
            _pad: [0; 3],
        },
        graphics_api: NRI_GRAPHICS_API_D3D12,
        _pad0: 0,
        nri_version_major: NRI_VERSION_MAJOR,
        nri_version_minor: NRI_VERSION_MINOR,
        _pad1: 0,
        viewport_max_num: 16,
        viewport_sub_pixel_bits: 8,
        viewport_bounds_range: [-32768, 32767],
        frame_buffer_max_dim: 16384,
        frame_buffer_layer_max_num: 2048,
        framebuffer_color_attachment_max_num: 8,
        frame_buffer_color_sample_max_num: 4,
        frame_buffer_depth_sample_max_num: 4,
        frame_buffer_stencil_sample_max_num: 4,
        frame_buffer_no_attachments_sample_max_num: 4,
        texture_color_sample_max_num: 4,
        texture_integer_sample_max_num: 1,
        texture_depth_sample_max_num: 4,
        texture_stencil_sample_max_num: 4,
        storage_texture_sample_max_num: 1,
        _pad2: [0; 3],
        texture_1d_max_dim: 16384,
        texture_2d_max_dim: 16384,
        texture_3d_max_dim: 2048,
        texture_array_max_dim: 2048,
        texel_buffer_max_dim: (1u32 << 27) - 1,
        memory_allocation_max_num: u32::MAX,
        sampler_allocation_max_num: 2048,
        upload_buffer_texture_row_alignment: 256,
        upload_buffer_texture_slice_alignment: 512,
        typed_buffer_offset_alignment: 16,
        constant_buffer_offset_alignment: 256,
        constant_buffer_max_range: 65536,
        storage_buffer_offset_alignment: 16,
        storage_buffer_max_range: (1u32 << 27) - 1,
        buffer_texture_granularity: 65536,
        _pad3: 0,
        buffer_max_size: 2048u64 * 1024 * 1024,
        push_constants_max_size: 256,
        bound_descriptor_set_max_num: 128,
        per_stage_descriptor_sampler_max_num: 16,
        per_stage_descriptor_constant_buffer_max_num: 14,
        per_stage_descriptor_storage_buffer_max_num: 64,
        per_stage_descriptor_texture_max_num: 128,
        per_stage_descriptor_storage_texture_max_num: 64,
        per_stage_resource_max_num: 128,
        descriptor_set_sampler_max_num: 16,
        descriptor_set_constant_buffer_max_num: 14,
        descriptor_set_storage_buffer_max_num: 64,
        descriptor_set_texture_max_num: 128,
        descriptor_set_storage_texture_max_num: 64,
        vertex_shader_attribute_max_num: 32,
        vertex_shader_stream_max_num: 32,
        vertex_shader_output_component_max_num: 128,
        tess_control_shader_generation_max_level: 64.0,
        tess_control_shader_patch_point_max_num: 32,
        tess_control_shader_per_vertex_input_component_max_num: 128,
        tess_control_shader_per_vertex_output_component_max_num: 128,
        tess_control_shader_per_patch_output_component_max_num: 120,
        tess_control_shader_total_output_component_max_num: 4216,
        tess_evaluation_shader_input_component_max_num: 128,
        tess_evaluation_shader_output_component_max_num: 128,
        geometry_shader_invocation_max_num: 32,
        geometry_shader_input_component_max_num: 128,
        geometry_shader_output_component_max_num: 128,
        geometry_shader_output_vertex_max_num: 1024,
        geometry_shader_total_output_component_max_num: 1024,
        fragment_shader_input_component_max_num: 128,
        fragment_shader_output_attachment_max_num: 8,
        fragment_shader_dual_source_attachment_max_num: 1,
        fragment_shader_combined_output_resource_max_num: 16,
        compute_shader_shared_memory_max_size: 32768,
        compute_shader_work_group_max_num: [65535, 65535, 65535],
        compute_shader_work_group_invocation_max_num: 1024,
        compute_shader_work_group_max_dim: [1024, 1024, 64],
        _pad4: 0,
        ray_tracing_shader_group_identifier_size: if rt_supported { 32 } else { 0 },
        ray_tracing_shader_table_alignment: if rt_supported { 64 } else { 0 },
        ray_tracing_shader_table_max_stride: if rt_supported { u64::MAX } else { 0 },
        ray_tracing_shader_recursion_max_depth: if rt_supported { 31 } else { 0 },
        ray_tracing_geometry_object_max_num: if rt_supported { (1 << 24) - 1 } else { 0 },
        mesh_task_work_group_invocation_max_num: if mesh_supported { 128 } else { 0 },
        mesh_task_work_group_max_dim: if mesh_supported {
            [128, 128, 128]
        } else {
            [0; 3]
        },
        mesh_task_total_memory_max_size: if mesh_supported { 32768 } else { 0 },
        mesh_task_output_max_num: if mesh_supported { 65536 } else { 0 },
        mesh_work_group_invocation_max_num: if mesh_supported { 128 } else { 0 },
        mesh_work_group_max_dim: if mesh_supported {
            [128, 128, 128]
        } else {
            [0; 3]
        },
        mesh_output_vertex_max_num: if mesh_supported { 256 } else { 0 },
        mesh_output_primitive_max_num: if mesh_supported { 256 } else { 0 },
        mesh_multiview_view_max_num: if mesh_supported { 4 } else { 0 },
        mesh_output_per_vertex_granularity: if mesh_supported { 32 } else { 0 },
        mesh_output_per_primitive_granularity: if mesh_supported { 32 } else { 0 },
        _pad5: 0,
        timestamp_frequency_hz: ts_freq,
        sub_pixel_precision_bits: 8,
        sub_texel_precision_bits: 8,
        mipmap_precision_bits: 8,
        draw_indexed_index_16_value_max: 0xffff,
        draw_indexed_index_32_value_max: 0xffff_ffff,
        draw_indirect_max_num: u32::MAX,
        sampler_lod_bias_min: -16.0,
        sampler_lod_bias_max: 15.99,
        sampler_anisotropy_max: 16.0,
        texel_offset_min: -8,
        texel_offset_max: 7,
        texel_gather_offset_min: -8,
        texel_gather_offset_max: 7,
        clip_distance_max_num: 8,
        cull_distance_max_num: 8,
        combined_clip_and_cull_distance_max_num: 8,
        conservative_raster_tier,
        physical_device_num: node_count,
        is_api_validation_enabled: enable_validation,
        is_texture_filter_min_max_supported: true,
        is_logic_op_supported: logic_op,
        is_depth_bounds_test_supported: depth_bounds_supported,
        is_programmable_sample_locations_supported: prog_sample_pos,
        is_compute_queue_supported: true,
        is_copy_queue_supported: true,
        is_copy_queue_timestamp_supported: copy_queue_ts,
        is_register_aliasing_supported: true,
        is_subset_allocation_supported: true,
        is_float16_supported: float16,
        _pad6: [0; 3],
    }
}

pub(super) unsafe fn create_device_from_d3d12(
    device_desc: *const u8,
    device_out: *mut *mut NriDevice,
) -> u32 {
    if device_desc.is_null() || device_out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let cd = rd(device_desc as *const NriDeviceCreationD3D12Desc);
    if cd.d3d12_device.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }

    com_add_ref(cd.d3d12_device);

    let desc = build_device_desc(
        cd.d3d12_device,
        cd.d3d12_graphics_queue,
        cd.enable_nri_validation != 0,
    );
    let dev_ptr = Box::into_raw(Box::new(NriDevice {
        d3d12_device: cd.d3d12_device,
        desc,
        queues: Mutex::new([None, None, None]),
        cpu_heaps: Mutex::new([
            CpuDescriptorAllocator::default(),
            CpuDescriptorAllocator::default(),
            CpuDescriptorAllocator::default(),
            CpuDescriptorAllocator::default(),
        ]),
        dispatch_cmd_sig: Mutex::new(ptr::null_mut()),
        draw_cmd_sigs: Mutex::new(HashMap::new()),
        draw_indexed_cmd_sigs: Mutex::new(HashMap::new()),
    }));

    // Register caller-provided D3D12 command queues.
    let provided = [
        (0usize, 0u32, cd.d3d12_graphics_queue),
        (1usize, 2u32, cd.d3d12_compute_queue),
        (2usize, 3u32, cd.d3d12_copy_queue),
    ];
    {
        let mut qs = (*dev_ptr).queues.lock().unwrap();
        for (slot, fallback_type, qp) in provided {
            if !qp.is_null() {
                com_add_ref(qp);
                let qdesc = d3d12_queue_get_desc(qp);
                let list_type = if matches!(qdesc.Type, 0 | 2 | 3) {
                    qdesc.Type
                } else {
                    fallback_type
                };
                qs[slot] = Some(Box::into_raw(Box::new(NriCommandQueue {
                    device: dev_ptr,
                    d3d12_queue: qp,
                    list_type,
                })));
            }
        }
    }

    // Ensure a graphics queue exists and refresh timestamp frequency from it.
    match NriDevice::get_or_create_queue(dev_ptr, 0) {
        Ok(gq) => {
            (*dev_ptr).desc.timestamp_frequency_hz =
                d3d12_queue_get_timestamp_freq((*gq).d3d12_queue);
        }
        Err(e) => {
            drop(Box::from_raw(dev_ptr));
            return e;
        }
    }

    static LOG_ONCE: Once = Once::new();
    LOG_ONCE.call_once(|| {
        log::info!(
            target: "d3d",
            "NRI (builtin D3D12): nriCreateDeviceFromD3D12Device v{}.{} -> device {:?}",
            NRI_VERSION_MAJOR,
            NRI_VERSION_MINOR,
            cd.d3d12_device
        );
    });

    wr(device_out, dev_ptr);
    NRI_RESULT_SUCCESS
}

// ---------------------------------------------------------------------------
// CoreInterface — Get (slots 0..6)
// ---------------------------------------------------------------------------

pub(super) unsafe extern "win64" fn core_get_device_desc(
    device: *const NriDevice,
) -> *const NriDeviceDesc {
    if device.is_null() {
        return ptr::null();
    }
    &(*device).desc
}

pub(super) unsafe extern "win64" fn core_get_format_support(
    _device: *const NriDevice,
    format: u8,
) -> u8 {
    let dxgi = nri_to_dxgi(format);
    if dxgi == 0 {
        return 0;
    }
    let (s1, s2) = d3d_fmt::support(dxgi);
    let mut bits = 0u8;
    if s1
        & (support1::TEXTURE1D | support1::TEXTURE2D | support1::TEXTURE3D | support1::TEXTURECUBE)
        != 0
    {
        bits |= 1 << 0; // TEXTURE
    }
    if s1 & support1::TYPED_UNORDERED_ACCESS_VIEW != 0 {
        bits |= 1 << 1; // STORAGE_TEXTURE
    }
    if s1 & support1::BUFFER != 0 {
        bits |= 1 << 2; // BUFFER
        if s1 & support1::TYPED_UNORDERED_ACCESS_VIEW != 0 || s2 & support2::UAV_TYPED_STORE != 0 {
            bits |= 1 << 3; // STORAGE_BUFFER
        }
    }
    if s1 & support1::RENDER_TARGET != 0 {
        bits |= 1 << 4; // COLOR_ATTACHMENT
    }
    if s1 & support1::DEPTH_STENCIL != 0 {
        bits |= 1 << 5; // DEPTH_STENCIL_ATTACHMENT
    }
    if s1 & support1::IA_VERTEX_BUFFER != 0 {
        bits |= 1 << 6; // VERTEX_BUFFER
    }
    bits
}

pub(super) unsafe extern "win64" fn core_get_query_size(query_pool: *const NriQueryPool) -> u32 {
    if query_pool.is_null() {
        0
    } else {
        (*query_pool).query_size
    }
}

unsafe fn fill_memory_info(
    device: *mut NriDevice,
    res_desc: &ResourceDesc,
    memory_location: u8,
    memory_desc: *mut NriMemoryDesc,
) {
    if device.is_null() || memory_desc.is_null() {
        return;
    }
    let mem_type = nri_memory_type(memory_location, res_desc);
    let info = d3d12_dev_get_resource_alloc_info((*device).d3d12_device, res_desc);
    wr(
        memory_desc,
        NriMemoryDesc {
            size: info.SizeInBytes,
            alignment: info.Alignment as u32,
            memory_type: mem_type,
            must_be_dedicated: u8::from(memory_type_requires_dedicated(mem_type)),
            _pad: [0; 7],
        },
    );
}

pub(super) unsafe extern "win64" fn core_get_buffer_memory_info(
    buffer: *const NriBuffer,
    memory_location: u8,
    memory_desc: *mut NriMemoryDesc,
) {
    if buffer.is_null() {
        return;
    }
    fill_memory_info(
        (*buffer).device,
        &(*buffer).desc,
        memory_location,
        memory_desc,
    );
}

pub(super) unsafe extern "win64" fn core_get_texture_memory_info(
    texture: *const NriTexture,
    memory_location: u8,
    memory_desc: *mut NriMemoryDesc,
) {
    if texture.is_null() {
        return;
    }
    fill_memory_info(
        (*texture).device,
        &(*texture).desc,
        memory_location,
        memory_desc,
    );
}

pub(super) unsafe extern "win64" fn core_get_command_queue(
    device: *mut NriDevice,
    queue_type: u8,
    queue_out: *mut *mut NriCommandQueue,
) -> u32 {
    if device.is_null() || queue_out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    match NriDevice::get_or_create_queue(device, queue_type) {
        Ok(q) => {
            wr(queue_out, q);
            NRI_RESULT_SUCCESS
        }
        Err(e) => e,
    }
}

// ---------------------------------------------------------------------------
// CoreInterface — Create (allocator, buffer, texture, pipelines, FB, query, fence)
// ---------------------------------------------------------------------------

pub(super) unsafe extern "win64" fn core_create_command_allocator(
    queue: *const NriCommandQueue,
    _physical_device_mask: u32,
    out: *mut *mut NriCommandAllocator,
) -> u32 {
    if queue.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let dev = (*queue).device;
    let list_type = (*queue).list_type;
    let mut alloc: *mut c_void = ptr::null_mut();
    let hr = d3d12_dev_create_command_allocator((*dev).d3d12_device, list_type, &mut alloc);
    if hr < 0 || alloc.is_null() {
        return hresult_to_nri(hr);
    }
    wr(
        out,
        Box::into_raw(Box::new(NriCommandAllocator {
            device: dev,
            d3d12_allocator: alloc,
            list_type,
        })),
    );
    NRI_RESULT_SUCCESS
}

pub(super) unsafe extern "win64" fn core_create_buffer(
    device: *mut NriDevice,
    desc_ptr: *const NriBufferDesc,
    out: *mut *mut NriBuffer,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let bd = rd(desc_ptr);
    if bd.size == 0 {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let mut flags = 0u32;
    if bd.usage_mask & (1 << 1) != 0 {
        flags |= resource_flags::ALLOW_UNORDERED_ACCESS;
    }
    let res_desc = ResourceDesc {
        Dimension: dim::BUFFER,
        Alignment: D3D12_DEFAULT_RESOURCE_PLACEMENT_ALIGNMENT,
        Width: bd.size,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: 0,
        SampleCount: 1,
        SampleQuality: 0,
        Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
        Flags: flags,
    };
    wr(
        out,
        Box::into_raw(Box::new(NriBuffer {
            device,
            d3d12_resource: ptr::null_mut(),
            desc: res_desc,
            structure_stride: bd.structure_stride,
        })),
    );
    NRI_RESULT_SUCCESS
}

pub(super) unsafe extern "win64" fn core_create_texture(
    device: *mut NriDevice,
    desc_ptr: *const NriTextureDesc,
    out: *mut *mut NriTexture,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let td = rd(desc_ptr);
    let dimension = match td.texture_type {
        0 => dim::TEXTURE1D,
        1 => dim::TEXTURE2D,
        2 => dim::TEXTURE3D,
        _ => return NRI_RESULT_INVALID_ARGUMENT,
    };
    let dxgi_typeless = nri_to_dxgi_typeless(td.format);
    if dxgi_typeless == 0 {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let bw = nri_texel_block_width(td.format) as u64;
    let width = align_up(td.size[0].max(1) as u64, bw);
    let height = align_up(td.size[1].max(1) as u64, bw) as u32;
    let depth_or_array = if td.texture_type == 2 {
        td.size[2].max(1)
    } else {
        td.array_size.max(1)
    };
    let samples = td.sample_num.max(1) as u32;
    let mut flags = 0u32;
    if td.usage_mask & (1 << 1) != 0 {
        flags |= resource_flags::ALLOW_UNORDERED_ACCESS;
    }
    if td.usage_mask & (1 << 2) != 0 {
        flags |= resource_flags::ALLOW_RENDER_TARGET;
    }
    if td.usage_mask & (1 << 3) != 0 {
        flags |= resource_flags::ALLOW_DEPTH_STENCIL;
        if td.usage_mask & (1 << 0) == 0 {
            flags |= resource_flags::DENY_SHADER_RESOURCE;
        }
    }
    let res_desc = ResourceDesc {
        Dimension: dimension,
        Alignment: 0,
        Width: width,
        Height: height,
        DepthOrArraySize: depth_or_array,
        MipLevels: td.mip_num.max(1),
        Format: dxgi_typeless,
        SampleCount: samples,
        SampleQuality: 0,
        Layout: D3D12_TEXTURE_LAYOUT_UNKNOWN,
        Flags: flags,
    };
    wr(
        out,
        Box::into_raw(Box::new(NriTexture {
            device,
            d3d12_resource: ptr::null_mut(),
            desc: res_desc,
            nri_format: td.format,
        })),
    );
    NRI_RESULT_SUCCESS
}

/// Pointer-aligned builder for `D3D12_PIPELINE_STATE_STREAM_DESC` subobjects,
/// matching MSVC `CD3DX12_PIPELINE_STATE_STREAM` and `crate::d3d12::pso::parse_stream`.
struct PsoStreamBuf {
    words: Vec<u64>,
}

impl PsoStreamBuf {
    fn new() -> Self {
        Self {
            words: Vec::with_capacity(96),
        }
    }

    fn push_subobject(&mut self, subobject_type: u32, inner_align: usize, payload: &[u8]) {
        let inner_off = 4usize.next_multiple_of(inner_align);
        let total_bytes = (inner_off + payload.len()).next_multiple_of(8);
        let start_word = self.words.len();
        self.words.resize(start_word + total_bytes / 8, 0u64);
        let dst = unsafe {
            std::slice::from_raw_parts_mut(
                self.words.as_mut_ptr().add(start_word) as *mut u8,
                total_bytes,
            )
        };
        dst[0..4].copy_from_slice(&subobject_type.to_le_bytes());
        dst[inner_off..inner_off + payload.len()].copy_from_slice(payload);
    }

    fn push_u32(&mut self, subobject_type: u32, val: u32) {
        self.push_subobject(subobject_type, 4, &val.to_le_bytes());
    }

    fn push_ptr(&mut self, subobject_type: u32, ptr_val: usize) {
        self.push_subobject(subobject_type, 8, &(ptr_val as u64).to_le_bytes());
    }

    fn push_bytecode(&mut self, subobject_type: u32, bc: Bytecode) {
        let mut buf = [0u8; 16];
        buf[0..8].copy_from_slice(&(bc.ptr as u64).to_le_bytes());
        buf[8..16].copy_from_slice(&(bc.len as u64).to_le_bytes());
        self.push_subobject(subobject_type, 8, &buf);
    }

    fn as_stream_desc(&self) -> StreamDesc {
        StreamDesc {
            size: self.words.len() * size_of::<u64>(),
            stream: self.words.as_ptr() as *const u8,
        }
    }
}

pub(super) unsafe extern "win64" fn core_create_graphics_pipeline(
    device: *mut NriDevice,
    desc_ptr: *const NriGraphicsPipelineDesc,
    out: *mut *mut NriPipeline,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let gd = rd(desc_ptr);
    if gd.pipeline_layout.is_null() || (gd.shader_stage_num > 0 && gd.shader_stages.is_null()) {
        return NRI_RESULT_INVALID_ARGUMENT;
    }

    let mut vs: Option<Bytecode> = None;
    let mut hs: Option<Bytecode> = None;
    let mut ds: Option<Bytecode> = None;
    let mut gs: Option<Bytecode> = None;
    let mut ps: Option<Bytecode> = None;
    let mut as_bc: Option<Bytecode> = None; // ShaderStage::MESH_CONTROL / TASK (13)
    let mut ms_bc: Option<Bytecode> = None; // ShaderStage::MESH_EVALUATION / MESH (14)

    for i in 0..gd.shader_stage_num as usize {
        let sh = rd(gd.shader_stages.add(i));
        let bc = Bytecode {
            ptr: sh.bytecode,
            len: sh.size as usize,
        };
        match sh.stage {
            1 => vs = Some(bc),
            2 => hs = Some(bc),
            3 => ds = Some(bc),
            4 => gs = Some(bc),
            5 => ps = Some(bc),
            13 => as_bc = Some(bc),
            14 => ms_bc = Some(bc),
            _ => {}
        }
    }
    let is_mesh_pipeline = as_bc.is_some() || ms_bc.is_some();

    let mut blend_enabled = false;
    let mut blend_factor = [0.0f32; 4];
    let mut blend = BlendDesc {
        AlphaToCoverageEnable: 0,
        IndependentBlendEnable: 1,
        RenderTarget: [RenderTargetBlendDesc {
            BlendEnable: 0,
            LogicOpEnable: 0,
            SrcBlend: 2,
            DestBlend: 1,
            BlendOp: 1,
            SrcBlendAlpha: 2,
            DestBlendAlpha: 1,
            BlendOpAlpha: 1,
            LogicOp: 4,
            RenderTargetWriteMask: 0xF,
        }; 8],
    };
    let mut sample_mask = u32::MAX;
    let mut sample_count = 1u32;
    let mut raster_raw = [0u8; 44];

    if !gd.rasterization.is_null() {
        let r = rd(gd.rasterization);
        blend.AlphaToCoverageEnable = i32::from(r.alpha_to_coverage != 0);
        sample_mask = if r.sample_mask == 0 {
            u32::MAX
        } else {
            r.sample_mask as u32
        };
        sample_count = r.sample_num.max(1) as u32;
        let fill = if r.fill_mode == 1 { 2u32 } else { 3u32 };
        let cull = match r.cull_mode {
            1 => 2u32,
            2 => 3u32,
            _ => 1u32,
        };
        put_u32(&mut raster_raw, 0, fill);
        put_u32(&mut raster_raw, 4, cull);
        put_u32(&mut raster_raw, 8, u32::from(r.front_counter_clockwise != 0));
        put_u32(&mut raster_raw, 12, r.depth_bias_constant_factor as u32);
        put_u32(&mut raster_raw, 16, r.depth_bias_clamp.to_bits());
        put_u32(&mut raster_raw, 20, r.depth_bias_slope_factor.to_bits());
        put_u32(&mut raster_raw, 24, u32::from(r.depth_clamp != 0));
        put_u32(&mut raster_raw, 28, u32::from(sample_count > 1));
        put_u32(&mut raster_raw, 32, u32::from(r.antialiased_lines != 0));
        put_u32(
            &mut raster_raw,
            36,
            if sample_count > 1 { sample_count } else { 0 },
        );
        put_u32(
            &mut raster_raw,
            40,
            u32::from(r.conservative_rasterization != 0),
        );
    } else {
        put_u32(&mut raster_raw, 0, 3); // SOLID
        put_u32(&mut raster_raw, 4, 3); // BACK
        put_u32(&mut raster_raw, 24, 1); // DepthClipEnable
    }

    let mut num_rt = 0u32;
    let mut rtv_formats = [0u32; 8];
    let mut dsv_format = 0u32;
    let mut ds_raw = [0u8; 52];

    if !gd.output_merger.is_null() {
        let om = rd(gd.output_merger);
        let n_rt = (om.color_num as usize).min(8);
        num_rt = n_rt as u32;
        if n_rt > 0 && !om.color.is_null() {
            for i in 0..n_rt {
                let c = rd(om.color.add(i));
                rtv_formats[i] = nri_to_dxgi(c.format);
                let rt = &mut blend.RenderTarget[i];
                rt.BlendEnable = i32::from(c.blend_enabled != 0);
                rt.RenderTargetWriteMask = c.color_write_mask & 0xF;
                if c.blend_enabled != 0 {
                    blend_enabled = true;
                    let lop = nri_logic_op(om.color_logic_func);
                    rt.LogicOp = lop;
                    rt.LogicOpEnable = i32::from(lop != 4);
                    rt.SrcBlend = nri_blend_factor(c.color_blend.src_factor);
                    rt.DestBlend = nri_blend_factor(c.color_blend.dst_factor);
                    rt.BlendOp = nri_blend_op(c.color_blend.func);
                    rt.SrcBlendAlpha = nri_blend_factor(c.alpha_blend.src_factor);
                    rt.DestBlendAlpha = nri_blend_factor(c.alpha_blend.dst_factor);
                    rt.BlendOpAlpha = nri_blend_op(c.alpha_blend.func);
                }
            }
        }
        blend_factor = om.blend_consts;
        dsv_format = nri_to_dxgi(om.depth_stencil_format);
        // DepthStencilDesc (52 bytes)
        put_u32(&mut ds_raw, 0, u32::from(om.depth.compare_func != 0));
        put_u32(&mut ds_raw, 4, u32::from(om.depth.write != 0));
        put_u32(&mut ds_raw, 8, nri_compare_func(om.depth.compare_func));
        let st_en = om.stencil.front.compare_func != 0 || om.stencil.back.compare_func != 0;
        put_u32(&mut ds_raw, 12, u32::from(st_en));
        ds_raw[16] = om.stencil.compare_mask;
        ds_raw[17] = om.stencil.write_mask;
        put_u32(&mut ds_raw, 20, nri_stencil_op(om.stencil.front.fail));
        put_u32(&mut ds_raw, 24, nri_stencil_op(om.stencil.front.depth_fail));
        put_u32(&mut ds_raw, 28, nri_stencil_op(om.stencil.front.pass));
        put_u32(
            &mut ds_raw,
            32,
            nri_compare_func(om.stencil.front.compare_func),
        );
        put_u32(&mut ds_raw, 36, nri_stencil_op(om.stencil.back.fail));
        put_u32(&mut ds_raw, 40, nri_stencil_op(om.stencil.back.depth_fail));
        put_u32(&mut ds_raw, 44, nri_stencil_op(om.stencil.back.pass));
        put_u32(
            &mut ds_raw,
            48,
            nri_compare_func(om.stencil.back.compare_func),
        );
    }

    let blend_bytes =
        std::slice::from_raw_parts(&blend as *const BlendDesc as *const u8, size_of::<BlendDesc>());

    let mut input_elems: Vec<InputElementDesc> = Vec::new();
    let mut stream_strides: Vec<u32> = Vec::new();
    let mut prim_topo = 4u32; // TRIANGLELIST
    let mut strip_cut = 0u32;
    let mut topo_type = 3u32; // TRIANGLE
    if !gd.input_assembly.is_null() {
        let ia = rd(gd.input_assembly);
        strip_cut = ia.primitive_restart as u32;
        topo_type = nri_topology_type(ia.topology);
        prim_topo = nri_primitive_topology(ia.topology, ia.tess_control_point_num);
        if ia.stream_num > 0 && !ia.streams.is_null() {
            for s in 0..ia.stream_num as usize {
                let st = rd(ia.streams.add(s));
                let slot = st.binding_slot as usize;
                if stream_strides.len() <= slot {
                    stream_strides.resize(slot + 1, 0);
                }
                stream_strides[slot] = st.stride;
            }
            if ia.attribute_num > 0 && !ia.attributes.is_null() {
                for a in 0..ia.attribute_num as usize {
                    let attr = rd(ia.attributes.add(a));
                    if (attr.stream_index as usize) >= ia.stream_num as usize {
                        return NRI_RESULT_INVALID_ARGUMENT;
                    }
                    let st = rd(ia.streams.add(attr.stream_index as usize));
                    let per_vertex = st.step_rate == 0;
                    input_elems.push(InputElementDesc {
                        name: attr.d3d.semantic_name,
                        semantic_index: attr.d3d.semantic_index,
                        format: nri_to_dxgi(attr.format),
                        input_slot: st.binding_slot as u32,
                        aligned_byte_offset: attr.offset,
                        slot_class: u32::from(!per_vertex),
                        step_rate: u32::from(!per_vertex),
                    });
                }
            }
        }
    }

    let mut pso: *mut c_void = ptr::null_mut();
    let hr = if is_mesh_pipeline {
        // Mesh / Amplification (Task) pipelines use ID3D12Device2::CreatePipelineState
        // with a D3D12_PIPELINE_STATE_STREAM_DESC containing AS (24) / MS (25) / PS (2).
        let mut stream = PsoStreamBuf::new();
        stream.push_ptr(0, (*gd.pipeline_layout).d3d12_root_sig as usize);
        if let Some(bc) = vs {
            stream.push_bytecode(1, bc);
        }
        if let Some(bc) = ps {
            stream.push_bytecode(2, bc);
        }
        if let Some(bc) = ds {
            stream.push_bytecode(3, bc);
        }
        if let Some(bc) = hs {
            stream.push_bytecode(4, bc);
        }
        if let Some(bc) = gs {
            stream.push_bytecode(5, bc);
        }
        if let Some(bc) = as_bc {
            stream.push_bytecode(24, bc);
        }
        if let Some(bc) = ms_bc {
            stream.push_bytecode(25, bc);
        }
        stream.push_subobject(8, 4, blend_bytes);
        stream.push_u32(9, sample_mask);
        stream.push_subobject(10, 4, &raster_raw);
        stream.push_subobject(11, 4, &ds_raw);
        let mut rt_array = [0u8; 36];
        for (i, &fmt) in rtv_formats.iter().enumerate() {
            put_u32(&mut rt_array, i * 4, fmt);
        }
        put_u32(&mut rt_array, 32, num_rt);
        stream.push_subobject(15, 4, &rt_array);
        stream.push_u32(16, dsv_format);
        let mut sample_desc = [0u8; 8];
        put_u32(&mut sample_desc, 0, sample_count);
        stream.push_subobject(17, 4, &sample_desc);
        stream.push_u32(18, 0);

        let stream_desc = stream.as_stream_desc();
        d3d12_dev_create_pipeline_state((*device).d3d12_device, &stream_desc, &mut pso)
    } else {
        // Classic VS/HS/DS/GS/PS pipeline via D3D12_GRAPHICS_PIPELINE_STATE_DESC (656 bytes).
        let mut raw = [0u8; 656];
        let put_ptr = |buf: &mut [u8], off: usize, v: usize| {
            buf[off..off + 8].copy_from_slice(&(v as u64).to_le_bytes());
        };
        let put_bc = |buf: &mut [u8], off: usize, bc: Bytecode| {
            put_ptr(buf, off, bc.ptr as usize);
            put_ptr(buf, off + 8, bc.len);
        };
        put_ptr(&mut raw, 0, (*gd.pipeline_layout).d3d12_root_sig as usize);
        if let Some(bc) = vs {
            put_bc(&mut raw, 8, bc);
        }
        if let Some(bc) = ps {
            put_bc(&mut raw, 24, bc);
        }
        if let Some(bc) = ds {
            put_bc(&mut raw, 40, bc);
        }
        if let Some(bc) = hs {
            put_bc(&mut raw, 56, bc);
        }
        if let Some(bc) = gs {
            put_bc(&mut raw, 72, bc);
        }
        raw[120..120 + blend_bytes.len()].copy_from_slice(blend_bytes);
        put_u32(&mut raw, 448, sample_mask);
        raw[452..496].copy_from_slice(&raster_raw);
        raw[496..548].copy_from_slice(&ds_raw);
        if !input_elems.is_empty() {
            put_ptr(&mut raw, 552, input_elems.as_ptr() as usize);
            put_u32(&mut raw, 560, input_elems.len() as u32);
        }
        put_u32(&mut raw, 568, strip_cut);
        put_u32(&mut raw, 572, topo_type);
        put_u32(&mut raw, 576, num_rt);
        for (i, &fmt) in rtv_formats.iter().enumerate() {
            put_u32(&mut raw, 580 + i * 4, fmt);
        }
        put_u32(&mut raw, 612, dsv_format);
        put_u32(&mut raw, 616, sample_count);

        d3d12_dev_create_graphics_pso((*device).d3d12_device, raw.as_ptr(), &mut pso)
    };
    if hr < 0 || pso.is_null() {
        return hresult_to_nri(hr);
    }

    wr(
        out,
        Box::into_raw(Box::new(NriPipeline {
            d3d12_pso: pso,
            is_graphics: true,
            primitive_topology: prim_topo,
            stream_strides,
            blend_enabled,
            blend_factor,
        })),
    );
    NRI_RESULT_SUCCESS
}

pub(super) unsafe extern "win64" fn core_create_compute_pipeline(
    device: *mut NriDevice,
    desc_ptr: *const NriComputePipelineDesc,
    out: *mut *mut NriPipeline,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let cd = rd(desc_ptr);
    if cd.pipeline_layout.is_null()
        || cd.compute_shader.bytecode.is_null()
        || cd.compute_shader.size == 0
    {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let d3d_desc = ComputePipelineDesc {
        root_signature: (*cd.pipeline_layout).d3d12_root_sig,
        cs: Bytecode {
            ptr: cd.compute_shader.bytecode,
            len: cd.compute_shader.size as usize,
        },
        node_mask: 0,
        cached: Bytecode {
            ptr: ptr::null(),
            len: 0,
        },
        flags: 0,
    };
    let mut pso: *mut c_void = ptr::null_mut();
    let hr = d3d12_dev_create_compute_pso((*device).d3d12_device, &d3d_desc, &mut pso);
    if hr < 0 || pso.is_null() {
        return hresult_to_nri(hr);
    }
    wr(
        out,
        Box::into_raw(Box::new(NriPipeline {
            d3d12_pso: pso,
            is_graphics: false,
            primitive_topology: 0,
            stream_strides: Vec::new(),
            blend_enabled: false,
            blend_factor: [0.0; 4],
        })),
    );
    NRI_RESULT_SUCCESS
}

pub(super) unsafe extern "win64" fn core_create_frame_buffer(
    device: *mut NriDevice,
    desc_ptr: *const NriFrameBufferDesc,
    out: *mut *mut NriFrameBuffer,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let fd = rd(desc_ptr);
    let mut rtvs = Vec::with_capacity(fd.color_attachment_num as usize);
    let mut clears = Vec::new();
    if fd.color_attachment_num > 0 {
        if fd.color_attachments.is_null() {
            return NRI_RESULT_INVALID_ARGUMENT;
        }
        for i in 0..fd.color_attachment_num as usize {
            let desc = rd(fd.color_attachments.add(i));
            rtvs.push(if desc.is_null() { 0 } else { (*desc).cpu_handle });
            if !fd.color_clear_values.is_null() {
                clears.push(NriClearDesc {
                    value: rd(fd.color_clear_values.add(i)),
                    attachment_content_type: 0, // COLOR
                    _pad: [0; 3],
                    color_attachment_index: i as u32,
                });
            }
        }
    }
    let dsv = if fd.depth_stencil_attachment.is_null() {
        0
    } else {
        if !fd.depth_stencil_clear_value.is_null() {
            clears.push(NriClearDesc {
                value: rd(fd.depth_stencil_clear_value),
                attachment_content_type: 3, // DEPTH_STENCIL
                _pad: [0; 3],
                color_attachment_index: 0,
            });
        }
        (*fd.depth_stencil_attachment).cpu_handle
    };
    wr(out, Box::into_raw(Box::new(NriFrameBuffer { rtvs, dsv, clears })));
    NRI_RESULT_SUCCESS
}

pub(super) unsafe extern "win64" fn core_create_query_pool(
    device: *mut NriDevice,
    desc_ptr: *const NriQueryPoolDesc,
    out: *mut *mut NriQueryPool,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let qd = rd(desc_ptr);
    let (heap_ty, query_ty, elem_size) = match qd.query_type {
        0 => (1u32, 2u32, 8u32), // TIMESTAMP: D3D12_QUERY_HEAP_TYPE_TIMESTAMP=1, D3D12_QUERY_TYPE_TIMESTAMP=2
        1 => (0u32, 0u32, 8u32), // OCCLUSION: D3D12_QUERY_HEAP_TYPE_OCCLUSION=0, D3D12_QUERY_TYPE_OCCLUSION=0
        2 | 3 => {
            crate::stub!(
                "CreateQueryPool: queryType {} is not supported on Metal",
                qd.query_type
            );
            return NRI_RESULT_UNSUPPORTED;
        }
        _ => return NRI_RESULT_INVALID_ARGUMENT,
    };
    let d3d_desc = QueryHeapDesc {
        ty: heap_ty,
        count: qd.capacity,
        node_mask: 0,
    };
    let mut heap: *mut c_void = ptr::null_mut();
    let hr = d3d12_dev_create_query_heap((*device).d3d12_device, &d3d_desc, &mut heap);
    if hr < 0 || heap.is_null() {
        return hresult_to_nri(hr);
    }
    wr(
        out,
        Box::into_raw(Box::new(NriQueryPool {
            d3d12_query_heap: heap,
            d3d12_query_type: query_ty,
            query_size: elem_size,
        })),
    );
    NRI_RESULT_SUCCESS
}

pub(super) unsafe extern "win64" fn core_create_fence(
    device: *mut NriDevice,
    initial_value: u64,
    out: *mut *mut NriFence,
) -> u32 {
    if device.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let mut fence: *mut c_void = ptr::null_mut();
    let hr = d3d12_dev_create_fence((*device).d3d12_device, initial_value, &mut fence);
    if hr < 0 || fence.is_null() {
        return hresult_to_nri(hr);
    }
    wr(out, Box::into_raw(Box::new(NriFence { d3d12_fence: fence })));
    NRI_RESULT_SUCCESS
}

// ---------------------------------------------------------------------------
// CoreInterface — Destroy (slots 22..33)
// ---------------------------------------------------------------------------

pub(super) unsafe extern "win64" fn core_destroy_command_allocator(p: *mut NriCommandAllocator) {
    if !p.is_null() {
        drop(Box::from_raw(p));
    }
}
pub(super) unsafe extern "win64" fn core_destroy_command_buffer(p: *mut NriCommandBuffer) {
    if !p.is_null() {
        drop(Box::from_raw(p));
    }
}
pub(super) unsafe extern "win64" fn core_destroy_descriptor_pool(p: *mut NriDescriptorPool) {
    if !p.is_null() {
        drop(Box::from_raw(p));
    }
}
pub(super) unsafe extern "win64" fn core_destroy_buffer(p: *mut NriBuffer) {
    if !p.is_null() {
        drop(Box::from_raw(p));
    }
}
pub(super) unsafe extern "win64" fn core_destroy_texture(p: *mut NriTexture) {
    if !p.is_null() {
        drop(Box::from_raw(p));
    }
}
pub(super) unsafe extern "win64" fn core_destroy_descriptor(p: *mut NriDescriptor) {
    if !p.is_null() {
        drop(Box::from_raw(p));
    }
}
pub(super) unsafe extern "win64" fn core_destroy_pipeline_layout(p: *mut NriPipelineLayout) {
    if !p.is_null() {
        drop(Box::from_raw(p));
    }
}
pub(super) unsafe extern "win64" fn core_destroy_pipeline(p: *mut NriPipeline) {
    if !p.is_null() {
        drop(Box::from_raw(p));
    }
}
pub(super) unsafe extern "win64" fn core_destroy_frame_buffer(p: *mut NriFrameBuffer) {
    if !p.is_null() {
        drop(Box::from_raw(p));
    }
}
pub(super) unsafe extern "win64" fn core_destroy_query_pool(p: *mut NriQueryPool) {
    if !p.is_null() {
        drop(Box::from_raw(p));
    }
}
pub(super) unsafe extern "win64" fn core_destroy_fence(p: *mut NriFence) {
    if !p.is_null() {
        drop(Box::from_raw(p));
    }
}

// ---------------------------------------------------------------------------
// CoreInterface — Memory (slots 33..37)
// ---------------------------------------------------------------------------

pub(super) unsafe extern "win64" fn core_allocate_memory(
    device: *mut NriDevice,
    _physical_device_mask: u32,
    memory_type: u32,
    size: u64,
    out: *mut *mut NriMemory,
) -> u32 {
    if device.is_null() || out.is_null() || size == 0 {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let dedicated = memory_type_requires_dedicated(memory_type);
    let heap_desc = HeapDesc {
        SizeInBytes: size,
        Properties: HeapProperties {
            Type: memory_type_heap_type(memory_type),
            CPUPageProperty: 0,
            MemoryPoolPreference: 0,
            CreationNodeMask: 0,
            VisibleNodeMask: 0,
        },
        Alignment: 0,
        Flags: memory_type_heap_flags(memory_type) | D3D12_HEAP_FLAG_CREATE_NOT_ZEROED,
    };
    let mut heap: *mut c_void = ptr::null_mut();
    if !dedicated {
        let hr = d3d12_dev_create_heap((*device).d3d12_device, &heap_desc, &mut heap);
        if hr < 0 || heap.is_null() {
            return hresult_to_nri(hr);
        }
    }
    wr(
        out,
        Box::into_raw(Box::new(NriMemory {
            d3d12_heap: heap,
            heap_desc,
            dedicated,
        })),
    );
    NRI_RESULT_SUCCESS
}

pub(super) unsafe extern "win64" fn core_bind_buffer_memory(
    device: *mut NriDevice,
    descs: *const NriBufferMemoryBindingDesc,
    num: u32,
) -> u32 {
    if device.is_null() || (num > 0 && descs.is_null()) {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    for i in 0..num as usize {
        let b = rd(descs.add(i));
        if b.memory.is_null() || b.buffer.is_null() {
            return NRI_RESULT_INVALID_ARGUMENT;
        }
        let mem = &*b.memory;
        let buf = &mut *b.buffer;
        let initial_state = match mem.heap_desc.Properties.Type {
            heap_type::UPLOAD => 0xAC3,  // GENERIC_READ
            heap_type::READBACK => 0x400, // COPY_DEST
            _ => 0,                      // COMMON
        };
        let mut res: *mut c_void = ptr::null_mut();
        let hr = if mem.dedicated || mem.d3d12_heap.is_null() {
            d3d12_dev_create_committed_resource(
                (*device).d3d12_device,
                &mem.heap_desc.Properties,
                D3D12_HEAP_FLAG_CREATE_NOT_ZEROED,
                &buf.desc,
                initial_state,
                ptr::null(),
                &mut res,
            )
        } else {
            d3d12_dev_create_placed_resource(
                (*device).d3d12_device,
                mem.d3d12_heap,
                b.offset,
                &buf.desc,
                initial_state,
                ptr::null(),
                &mut res,
            )
        };
        if hr < 0 || res.is_null() {
            return hresult_to_nri(hr);
        }
        if !buf.d3d12_resource.is_null() {
            com_release(buf.d3d12_resource);
        }
        buf.d3d12_resource = res;
    }
    NRI_RESULT_SUCCESS
}

pub(super) unsafe extern "win64" fn core_bind_texture_memory(
    device: *mut NriDevice,
    descs: *const NriTextureMemoryBindingDesc,
    num: u32,
) -> u32 {
    if device.is_null() || (num > 0 && descs.is_null()) {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    for i in 0..num as usize {
        let b = rd(descs.add(i));
        if b.memory.is_null() || b.texture.is_null() {
            return NRI_RESULT_INVALID_ARGUMENT;
        }
        let mem = &*b.memory;
        let tex = &mut *b.texture;
        let is_rt_ds = (tex.desc.Flags
            & (resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_DEPTH_STENCIL))
            != 0;
        let clear = ClearValue {
            Format: nri_to_dxgi(tex.nri_format),
            Color: [0.0; 4],
        };
        let clear_ptr = if is_rt_ds {
            &clear as *const ClearValue
        } else {
            ptr::null()
        };
        let mut res: *mut c_void = ptr::null_mut();
        let hr = if mem.dedicated || mem.d3d12_heap.is_null() {
            d3d12_dev_create_committed_resource(
                (*device).d3d12_device,
                &mem.heap_desc.Properties,
                D3D12_HEAP_FLAG_CREATE_NOT_ZEROED,
                &tex.desc,
                0,
                clear_ptr,
                &mut res,
            )
        } else {
            d3d12_dev_create_placed_resource(
                (*device).d3d12_device,
                mem.d3d12_heap,
                b.offset,
                &tex.desc,
                0,
                clear_ptr,
                &mut res,
            )
        };
        if hr < 0 || res.is_null() {
            return hresult_to_nri(hr);
        }
        if !tex.d3d12_resource.is_null() {
            com_release(tex.d3d12_resource);
        }
        tex.d3d12_resource = res;
    }
    NRI_RESULT_SUCCESS
}

pub(super) unsafe extern "win64" fn core_free_memory(memory: *mut NriMemory) {
    if !memory.is_null() {
        drop(Box::from_raw(memory));
    }
}

// ---------------------------------------------------------------------------
// CoreInterface — Allocator reset, Map/Unmap, Debug names, Native objects
// ---------------------------------------------------------------------------

pub(super) unsafe extern "win64" fn core_reset_command_allocator(
    allocator: *mut NriCommandAllocator,
) {
    if !allocator.is_null() && !(*allocator).d3d12_allocator.is_null() {
        let _ = d3d12_alloc_reset((*allocator).d3d12_allocator);
    }
}

pub(super) unsafe extern "win64" fn core_map_buffer(
    buffer: *mut NriBuffer,
    offset: u64,
    size: u64,
) -> *mut u8 {
    if buffer.is_null() || (*buffer).d3d12_resource.is_null() {
        return ptr::null_mut();
    }
    let b = &*buffer;
    let map_size = if size == NRI_WHOLE_SIZE {
        b.desc.Width.saturating_sub(offset)
    } else {
        size
    };
    let begin = offset as usize;
    let end = offset.saturating_add(map_size) as usize;
    match d3d12_res_map(b.d3d12_resource, begin, end) {
        Ok(base) => base.add(begin),
        Err(hr) => {
            log::warn!(
                target: "d3d",
                "NRI MapBuffer({buffer:?}, off={offset:#x}, size={size:#x}) failed: {hr:#x}"
            );
            ptr::null_mut()
        }
    }
}

pub(super) unsafe extern "win64" fn core_unmap_buffer(buffer: *mut NriBuffer) {
    if !buffer.is_null() && !(*buffer).d3d12_resource.is_null() {
        d3d12_res_unmap((*buffer).d3d12_resource);
    }
}

pub(super) unsafe extern "win64" fn core_set_device_name(obj: *mut NriDevice, name: *const u8) {
    if !obj.is_null() {
        d3d12_set_name_cstr((*obj).d3d12_device, name);
    }
}
pub(super) unsafe extern "win64" fn core_set_queue_name(
    obj: *mut NriCommandQueue,
    name: *const u8,
) {
    if !obj.is_null() {
        d3d12_set_name_cstr((*obj).d3d12_queue, name);
    }
}
pub(super) unsafe extern "win64" fn core_set_fence_name(obj: *mut NriFence, name: *const u8) {
    if !obj.is_null() {
        d3d12_set_name_cstr((*obj).d3d12_fence, name);
    }
}
pub(super) unsafe extern "win64" fn core_set_allocator_name(
    obj: *mut NriCommandAllocator,
    name: *const u8,
) {
    if !obj.is_null() {
        d3d12_set_name_cstr((*obj).d3d12_allocator, name);
    }
}
pub(super) unsafe extern "win64" fn core_set_pool_name(
    obj: *mut NriDescriptorPool,
    name: *const u8,
) {
    if !obj.is_null() {
        for i in 0..(*obj).heap_count {
            d3d12_set_name_cstr((*obj).heap_ptrs[i], name);
        }
    }
}
pub(super) unsafe extern "win64" fn core_set_buffer_name(obj: *mut NriBuffer, name: *const u8) {
    if !obj.is_null() {
        d3d12_set_name_cstr((*obj).d3d12_resource, name);
    }
}
pub(super) unsafe extern "win64" fn core_set_texture_name(obj: *mut NriTexture, name: *const u8) {
    if !obj.is_null() {
        d3d12_set_name_cstr((*obj).d3d12_resource, name);
    }
}
pub(super) unsafe extern "win64" fn core_set_pipeline_layout_name(
    obj: *mut NriPipelineLayout,
    name: *const u8,
) {
    if !obj.is_null() {
        d3d12_set_name_cstr((*obj).d3d12_root_sig, name);
    }
}
pub(super) unsafe extern "win64" fn core_set_pipeline_name(obj: *mut NriPipeline, name: *const u8) {
    if !obj.is_null() {
        d3d12_set_name_cstr((*obj).d3d12_pso, name);
    }
}
pub(super) unsafe extern "win64" fn core_set_query_pool_name(
    obj: *mut NriQueryPool,
    name: *const u8,
) {
    if !obj.is_null() {
        d3d12_set_name_cstr((*obj).d3d12_query_heap, name);
    }
}
pub(super) unsafe extern "win64" fn core_set_cmdbuf_name(
    obj: *mut NriCommandBuffer,
    name: *const u8,
) {
    if !obj.is_null() {
        d3d12_set_name_cstr((*obj).d3d12_list, name);
    }
}
pub(super) unsafe extern "win64" fn core_set_memory_name(obj: *mut NriMemory, name: *const u8) {
    if !obj.is_null() {
        d3d12_set_name_cstr((*obj).d3d12_heap, name);
    }
}
pub(super) unsafe extern "win64" fn core_set_name_noop(_obj: usize, _name: *const u8) {}

pub(super) unsafe extern "win64" fn core_get_device_native_object(
    device: *const NriDevice,
) -> *mut c_void {
    if device.is_null() {
        ptr::null_mut()
    } else {
        (*device).d3d12_device
    }
}

pub(super) unsafe extern "win64" fn core_get_cmdbuf_native_object(
    cmd: *const NriCommandBuffer,
) -> *mut c_void {
    if cmd.is_null() {
        ptr::null_mut()
    } else {
        (*cmd).d3d12_list
    }
}

pub(super) unsafe extern "win64" fn core_get_buffer_native_object(
    buffer: *const NriBuffer,
    _physical_device_index: u32,
) -> u64 {
    if buffer.is_null() {
        0
    } else {
        (*buffer).d3d12_resource as usize as u64
    }
}

pub(super) unsafe extern "win64" fn core_get_texture_native_object(
    texture: *const NriTexture,
    _physical_device_index: u32,
) -> u64 {
    if texture.is_null() {
        0
    } else {
        (*texture).d3d12_resource as usize as u64
    }
}

pub(super) unsafe extern "win64" fn core_get_descriptor_native_object(
    descriptor: *const NriDescriptor,
    _physical_device_index: u32,
) -> u64 {
    if descriptor.is_null() {
        0
    } else {
        (*descriptor).cpu_handle as u64
    }
}
