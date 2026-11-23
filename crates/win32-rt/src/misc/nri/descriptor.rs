//! NRI descriptor views (CBV/SRV/UAV/RTV/DSV/Sampler), shader-visible
//! descriptor pools, descriptor sets, and D3D12 root-signature pipeline layouts.

use std::ffi::c_void;
use std::ptr;

use super::abi::*;
use super::device::NriDevice;
use crate::d3d12::com::{com_add_ref, com_release};
use crate::strings::{rd, wr};
use d3d12_metal::metal::views::SamplerDesc as D3dSamplerDesc;
use d3d12_metal::types::DescriptorHeapDesc;
use dxil_metal::root_signature::{
    flags as rs_flags, DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind,
    RootParameter, RootParameterKind, RootSignatureDesc, RootSignatureVersion,
};

// ---------------------------------------------------------------------------
// Descriptor / Pool / Set / PipelineLayout objects
// ---------------------------------------------------------------------------

pub struct NriDescriptor {
    device: *mut NriDevice,
    heap_type: u32,
    chunk_idx: usize,
    slot_idx: u32,
    pub(super) cpu_handle: usize,
    pub(super) d3d12_resource: *mut c_void,
    pub(super) buffer_gpu_va: u64,
    pub(super) is_float_uav: bool,
}

impl Drop for NriDescriptor {
    fn drop(&mut self) {
        unsafe {
            if !self.device.is_null() {
                (*self.device).free_cpu_descriptor(self.heap_type, self.chunk_idx, self.slot_idx);
            }
            com_release(self.d3d12_resource);
        }
    }
}

struct ShaderVisibleHeap {
    heap: *mut c_void,
    cpu_start: usize,
    gpu_start: u64,
    inc_size: u32,
    capacity: u32,
}

impl Drop for ShaderVisibleHeap {
    fn drop(&mut self) {
        unsafe { com_release(self.heap) };
    }
}

pub struct NriDescriptorPool {
    device: *mut NriDevice,
    heaps: [Option<ShaderVisibleHeap>; 2],
    pub(super) heap_ptrs: [*mut c_void; 2],
    pub(super) heap_count: usize,
    allocated_counts: [u32; 2],
    sets: Vec<*mut NriDescriptorSet>,
    sets_max: u32,
}

impl NriDescriptorPool {
    fn cpu_ptr(&self, heap_type: u32, offset: u32) -> usize {
        self.heaps
            .get(heap_type as usize)
            .and_then(|h| h.as_ref())
            .map_or(0, |h| h.cpu_start + (offset as usize) * (h.inc_size as usize))
    }

    fn gpu_ptr(&self, heap_type: u32, offset: u32) -> u64 {
        self.heaps
            .get(heap_type as usize)
            .and_then(|h| h.as_ref())
            .map_or(0, |h| h.gpu_start + (offset as u64) * (h.inc_size as u64))
    }

    fn reset(&mut self) {
        unsafe {
            for set in self.sets.drain(..) {
                if !set.is_null() {
                    drop(Box::from_raw(set));
                }
            }
        }
        self.allocated_counts = [0, 0];
    }
}

impl Drop for NriDescriptorPool {
    fn drop(&mut self) {
        self.reset();
    }
}

#[derive(Clone, Copy)]
pub(super) struct DescriptorRangeMapping {
    heap_type: u32,
    heap_offset: u32,
    descriptor_num: u32,
}

#[derive(Clone, Default)]
pub(super) struct DescriptorSetMapping {
    ranges: Vec<DescriptorRangeMapping>,
    counts: [u32; 2],
}

#[derive(Clone, Copy, Default)]
pub(super) struct DynamicCbvMapping {
    pub(super) count: u16,
    pub(super) root_offset: u16,
}

pub struct NriDescriptorSet {
    device: *mut NriDevice,
    pool: *mut NriDescriptorPool,
    ranges: Vec<DescriptorRangeMapping>,
    pub(super) dynamic_cbv_vas: Vec<u64>,
}

impl NriDescriptorSet {
    unsafe fn cpu_ptr(&self, range_idx: usize, offset: u32) -> usize {
        if let Some(rm) = self.ranges.get(range_idx) {
            (*self.pool).cpu_ptr(rm.heap_type, rm.heap_offset + offset)
        } else {
            0
        }
    }

    pub(super) unsafe fn gpu_ptr(&self, range_idx: usize, offset: u32) -> u64 {
        if let Some(rm) = self.ranges.get(range_idx) {
            (*self.pool).gpu_ptr(rm.heap_type, rm.heap_offset + offset)
        } else {
            0
        }
    }
}

pub struct NriPipelineLayout {
    pub(super) d3d12_root_sig: *mut c_void,
    pub(super) is_graphics: bool,
    pub(super) set_mappings: Vec<DescriptorSetMapping>,
    pub(super) set_root_offsets: Vec<Vec<u16>>,
    pub(super) dynamic_cbv_mappings: Vec<DynamicCbvMapping>,
    pub(super) push_constants_base_index: u32,
}

impl Drop for NriPipelineLayout {
    fn drop(&mut self) {
        unsafe { com_release(self.d3d12_root_sig) };
    }
}

// ---------------------------------------------------------------------------
// CoreInterface — Create (descriptor pool, views, sampler, pipeline layout)
// ---------------------------------------------------------------------------

pub(super) unsafe extern "win64" fn core_create_descriptor_pool(
    device: *mut NriDevice,
    desc_ptr: *const NriDescriptorPoolDesc,
    out: *mut *mut NriDescriptorPool,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let d = rd(desc_ptr);
    let counts = [
        d.constant_buffer_max_num
            .saturating_add(d.texture_max_num)
            .saturating_add(d.storage_texture_max_num)
            .saturating_add(d.buffer_max_num)
            .saturating_add(d.storage_buffer_max_num)
            .saturating_add(d.structured_buffer_max_num)
            .saturating_add(d.storage_structured_buffer_max_num)
            .saturating_add(d.acceleration_structure_max_num),
        d.sampler_max_num,
    ];

    let mut heaps: [Option<ShaderVisibleHeap>; 2] = [None, None];
    let mut heap_ptrs = [ptr::null_mut(); 2];
    let mut heap_count = 0usize;

    for (ty, &count) in counts.iter().enumerate() {
        if count == 0 {
            continue;
        }
        let hd = DescriptorHeapDesc {
            Type: ty as u32,
            NumDescriptors: count,
            Flags: D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE,
            NodeMask: 0,
        };
        let mut heap: *mut c_void = ptr::null_mut();
        let hr = d3d12_dev_create_descriptor_heap((*device).d3d12_device, &hd, &mut heap);
        if hr < 0 || heap.is_null() {
            return hresult_to_nri(hr);
        }
        let cpu_start = d3d12_descheap_cpu_start(heap);
        let gpu_start = d3d12_descheap_gpu_start(heap);
        let inc_size = d3d12_dev_get_desc_inc_size((*device).d3d12_device, ty as u32);
        heap_ptrs[heap_count] = heap;
        heap_count += 1;
        heaps[ty] = Some(ShaderVisibleHeap {
            heap,
            cpu_start,
            gpu_start,
            inc_size,
            capacity: count,
        });
    }

    wr(
        out,
        Box::into_raw(Box::new(NriDescriptorPool {
            device,
            heaps,
            heap_ptrs,
            heap_count,
            allocated_counts: [0, 0],
            sets: Vec::new(),
            sets_max: d.descriptor_set_max_num,
        })),
    );
    NRI_RESULT_SUCCESS
}

pub(super) unsafe extern "win64" fn core_create_buffer_view(
    desc_ptr: *const NriBufferViewDesc,
    out: *mut *mut NriDescriptor,
) -> u32 {
    if desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let vd = rd(desc_ptr);
    if vd.buffer.is_null() || (*vd.buffer).d3d12_resource.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let buf = &*vd.buffer;
    let dev = buf.device;
    let size = if vd.size == NRI_WHOLE_SIZE {
        buf.desc.Width.saturating_sub(vd.offset)
    } else {
        vd.size
    };
    let dxgi = nri_to_dxgi(vd.format);
    let stride = buf.structure_stride;
    let is_raw = stride == 0 && dxgi == 0 && vd.view_type != 2;
    let view_dxgi = if is_raw { 39 } else { dxgi }; // DXGI_FORMAT_R32_TYPELESS for raw ByteAddressBuffer
    let raw_flag = u32::from(is_raw);
    let elem_size = if stride != 0 {
        stride as u64
    } else if is_raw {
        4
    } else {
        nri_texel_block_size(vd.format) as u64
    };
    let first_elem = vd.offset / elem_size.max(1);
    let num_elems = (size / elem_size.max(1)) as u32;

    let (chunk_idx, slot_idx, cpu_handle) = match (*dev).alloc_cpu_descriptor(0) {
        Ok(x) => x,
        Err(e) => return e,
    };

    let mut buffer_gpu_va = 0u64;
    let mut is_float_uav = false;
    match vd.view_type {
        2 => {
            // BufferViewType::CONSTANT
            let va = d3d12_res_get_gpu_va(buf.d3d12_resource) + vd.offset;
            d3d12_dev_create_cbv((*dev).d3d12_device, va, size as u32, cpu_handle);
            buffer_gpu_va = va;
        }
        0 => {
            // BufferViewType::SHADER_RESOURCE
            let mut d = [0u8; 40];
            put_u32(&mut d, 0, view_dxgi);
            put_u32(&mut d, 4, 1); // D3D12_SRV_DIMENSION_BUFFER
            put_u32(&mut d, 8, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING);
            put_u64(&mut d, 16, first_elem);
            put_u32(&mut d, 24, num_elems);
            put_u32(&mut d, 28, stride);
            put_u32(&mut d, 32, raw_flag);
            d3d12_dev_create_srv((*dev).d3d12_device, buf.d3d12_resource, &d, cpu_handle);
        }
        1 => {
            // BufferViewType::SHADER_RESOURCE_STORAGE
            let mut d = [0u8; 40];
            put_u32(&mut d, 0, view_dxgi);
            put_u32(&mut d, 4, 1); // D3D12_UAV_DIMENSION_BUFFER
            put_u64(&mut d, 8, first_elem);
            put_u32(&mut d, 16, num_elems);
            put_u32(&mut d, 20, stride);
            put_u64(&mut d, 24, 0); // CounterOffsetInBytes
            put_u32(&mut d, 32, raw_flag); // Flags
            d3d12_dev_create_uav((*dev).d3d12_device, buf.d3d12_resource, &d, cpu_handle);
            is_float_uav = nri_is_fp_format(vd.format);
        }
        _ => {
            (*dev).free_cpu_descriptor(0, chunk_idx, slot_idx);
            return NRI_RESULT_INVALID_ARGUMENT;
        }
    }

    com_add_ref(buf.d3d12_resource);
    wr(
        out,
        Box::into_raw(Box::new(NriDescriptor {
            device: dev,
            heap_type: 0,
            chunk_idx,
            slot_idx,
            cpu_handle,
            d3d12_resource: buf.d3d12_resource,
            buffer_gpu_va,
            is_float_uav,
        })),
    );
    NRI_RESULT_SUCCESS
}

unsafe fn create_texture_view_common(
    desc_ptr: *const NriTextureViewDesc,
    dim_kind: u8,
    out: *mut *mut NriDescriptor,
) -> u32 {
    if desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let vd = rd(desc_ptr);
    if vd.texture.is_null() || (*vd.texture).d3d12_resource.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let tex = &*vd.texture;
    let dev = tex.device;
    let dxgi = nri_to_dxgi(vd.format);
    let total_mips = tex.desc.mip_levels();
    let total_arr = tex.desc.array_size();
    let rem_mips = if vd.mip_num == NRI_REMAINING_MIP_LEVELS {
        total_mips.saturating_sub(vd.mip_offset as u32).max(1)
    } else {
        vd.mip_num as u32
    };
    let rem_arr = if vd.array_or_slice_size == NRI_REMAINING_ARRAY_LAYERS {
        total_arr.saturating_sub(vd.array_or_slice_offset as u32).max(1)
    } else {
        vd.array_or_slice_size as u32
    };
    let msaa = tex.desc.SampleCount > 1;

    enum ViewOp {
        Srv([u8; 40]),
        Uav([u8; 40]),
        Rtv([u32; 6]),
        Dsv([u32; 6]),
    }

    let op = match (dim_kind, vd.view_type) {
        // 1D views
        (1, 0) => {
            let mut d = [0u8; 40];
            put_u32(&mut d, 0, dxgi);
            put_u32(&mut d, 4, 2); // TEXTURE1D
            put_u32(&mut d, 8, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING);
            put_u32(&mut d, 16, vd.mip_offset as u32);
            put_u32(&mut d, 20, rem_mips);
            ViewOp::Srv(d)
        }
        (1, 1) => {
            let mut d = [0u8; 40];
            put_u32(&mut d, 0, dxgi);
            put_u32(&mut d, 4, 3); // TEXTURE1DARRAY
            put_u32(&mut d, 8, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING);
            put_u32(&mut d, 16, vd.mip_offset as u32);
            put_u32(&mut d, 20, rem_mips);
            put_u32(&mut d, 24, vd.array_or_slice_offset as u32);
            put_u32(&mut d, 28, rem_arr);
            ViewOp::Srv(d)
        }
        (1, 2) => {
            let mut d = [0u8; 40];
            put_u32(&mut d, 0, dxgi);
            put_u32(&mut d, 4, 2); // UAV TEXTURE1D
            put_u32(&mut d, 8, vd.mip_offset as u32);
            ViewOp::Uav(d)
        }
        (1, 3) => {
            let mut d = [0u8; 40];
            put_u32(&mut d, 0, dxgi);
            put_u32(&mut d, 4, 3); // UAV TEXTURE1DARRAY
            put_u32(&mut d, 8, vd.mip_offset as u32);
            put_u32(&mut d, 12, vd.array_or_slice_offset as u32);
            put_u32(&mut d, 16, rem_arr);
            ViewOp::Uav(d)
        }
        (1, 4) => ViewOp::Rtv([
            dxgi,
            3,
            vd.mip_offset as u32,
            vd.array_or_slice_offset as u32,
            rem_arr,
            0,
        ]),
        (1, 5) => {
            let mut f = 0u32;
            if vd.flags & 1 != 0 {
                f |= 1;
            }
            if vd.flags & 2 != 0 {
                f |= 2;
            }
            ViewOp::Dsv([
                dxgi,
                2,
                f,
                vd.mip_offset as u32,
                vd.array_or_slice_offset as u32,
                rem_arr,
            ])
        }
        // 2D views
        (2, 0) => {
            let mut d = [0u8; 40];
            put_u32(&mut d, 0, shader_format_for_depth(dxgi));
            put_u32(&mut d, 8, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING);
            if msaa {
                put_u32(&mut d, 4, 6); // TEXTURE2DMS
            } else {
                put_u32(&mut d, 4, 4); // TEXTURE2D
                put_u32(&mut d, 16, vd.mip_offset as u32);
                put_u32(&mut d, 20, rem_mips);
                put_u32(&mut d, 24, 0);
            }
            ViewOp::Srv(d)
        }
        (2, 1) => {
            let mut d = [0u8; 40];
            put_u32(&mut d, 0, shader_format_for_depth(dxgi));
            put_u32(&mut d, 8, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING);
            if msaa {
                put_u32(&mut d, 4, 7); // TEXTURE2DMSARRAY
                put_u32(&mut d, 16, vd.array_or_slice_offset as u32);
                put_u32(&mut d, 20, rem_arr);
            } else {
                put_u32(&mut d, 4, 5); // TEXTURE2DARRAY
                put_u32(&mut d, 16, vd.mip_offset as u32);
                put_u32(&mut d, 20, rem_mips);
                put_u32(&mut d, 24, vd.array_or_slice_offset as u32);
                put_u32(&mut d, 28, rem_arr);
            }
            ViewOp::Srv(d)
        }
        (2, 2) => {
            let mut d = [0u8; 40];
            put_u32(&mut d, 0, dxgi);
            put_u32(&mut d, 4, 9); // TEXTURECUBE
            put_u32(&mut d, 8, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING);
            put_u32(&mut d, 16, vd.mip_offset as u32);
            put_u32(&mut d, 20, rem_mips);
            ViewOp::Srv(d)
        }
        (2, 3) => {
            let mut d = [0u8; 40];
            put_u32(&mut d, 0, dxgi);
            put_u32(&mut d, 4, 10); // TEXTURECUBEARRAY
            put_u32(&mut d, 8, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING);
            put_u32(&mut d, 16, vd.mip_offset as u32);
            put_u32(&mut d, 20, rem_mips);
            put_u32(&mut d, 24, vd.array_or_slice_offset as u32);
            put_u32(&mut d, 28, (rem_arr / 6).max(1));
            ViewOp::Srv(d)
        }
        (2, 4) => {
            let mut d = [0u8; 40];
            put_u32(&mut d, 0, dxgi);
            put_u32(&mut d, 4, 4); // UAV TEXTURE2D
            put_u32(&mut d, 8, vd.mip_offset as u32);
            ViewOp::Uav(d)
        }
        (2, 5) => {
            let mut d = [0u8; 40];
            put_u32(&mut d, 0, dxgi);
            put_u32(&mut d, 4, 5); // UAV TEXTURE2DARRAY
            put_u32(&mut d, 8, vd.mip_offset as u32);
            put_u32(&mut d, 12, vd.array_or_slice_offset as u32);
            put_u32(&mut d, 16, rem_arr);
            ViewOp::Uav(d)
        }
        (2, 6) => {
            if msaa {
                ViewOp::Rtv([dxgi, 7, vd.array_or_slice_offset as u32, rem_arr, 0, 0])
            } else {
                ViewOp::Rtv([
                    dxgi,
                    5,
                    vd.mip_offset as u32,
                    vd.array_or_slice_offset as u32,
                    rem_arr,
                    0,
                ])
            }
        }
        (2, 7) => {
            let mut f = 0u32;
            if vd.flags & 1 != 0 {
                f |= 1;
            }
            if vd.flags & 2 != 0 {
                f |= 2;
            }
            if msaa {
                ViewOp::Dsv([dxgi, 6, f, vd.array_or_slice_offset as u32, rem_arr, 0])
            } else {
                ViewOp::Dsv([
                    dxgi,
                    4,
                    f,
                    vd.mip_offset as u32,
                    vd.array_or_slice_offset as u32,
                    rem_arr,
                ])
            }
        }
        // 3D views
        (3, 0) => {
            let mut d = [0u8; 40];
            put_u32(&mut d, 0, dxgi);
            put_u32(&mut d, 4, 8); // TEXTURE3D
            put_u32(&mut d, 8, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING);
            put_u32(&mut d, 16, vd.mip_offset as u32);
            put_u32(&mut d, 20, rem_mips);
            ViewOp::Srv(d)
        }
        (3, 1) => {
            let mut d = [0u8; 40];
            put_u32(&mut d, 0, dxgi);
            put_u32(&mut d, 4, 8); // UAV TEXTURE3D
            put_u32(&mut d, 8, vd.mip_offset as u32);
            put_u32(&mut d, 12, vd.array_or_slice_offset as u32);
            let wsize = if vd.array_or_slice_size == 0 {
                tex.desc
                    .depth()
                    .saturating_sub(vd.array_or_slice_offset as u32)
                    .max(1)
            } else {
                vd.array_or_slice_size as u32
            };
            put_u32(&mut d, 16, wsize);
            ViewOp::Uav(d)
        }
        (3, 2) => {
            let wsize = if vd.array_or_slice_size == 0 {
                tex.desc
                    .depth()
                    .saturating_sub(vd.array_or_slice_offset as u32)
                    .max(1)
            } else {
                vd.array_or_slice_size as u32
            };
            ViewOp::Rtv([
                dxgi,
                8,
                vd.mip_offset as u32,
                vd.array_or_slice_offset as u32,
                wsize,
                0,
            ])
        }
        _ => return NRI_RESULT_INVALID_ARGUMENT,
    };

    let heap_type = match &op {
        ViewOp::Srv(_) | ViewOp::Uav(_) => 0,
        ViewOp::Rtv(_) => 2,
        ViewOp::Dsv(_) => 3,
    };
    let (chunk_idx, slot_idx, cpu_handle) = match (*dev).alloc_cpu_descriptor(heap_type) {
        Ok(x) => x,
        Err(e) => return e,
    };
    let mut is_float_uav = false;
    match op {
        ViewOp::Srv(d) => {
            d3d12_dev_create_srv((*dev).d3d12_device, tex.d3d12_resource, &d, cpu_handle)
        }
        ViewOp::Uav(d) => {
            d3d12_dev_create_uav((*dev).d3d12_device, tex.d3d12_resource, &d, cpu_handle);
            is_float_uav = nri_is_fp_format(vd.format);
        }
        ViewOp::Rtv(d) => {
            d3d12_dev_create_rtv((*dev).d3d12_device, tex.d3d12_resource, &d, cpu_handle)
        }
        ViewOp::Dsv(d) => {
            d3d12_dev_create_dsv((*dev).d3d12_device, tex.d3d12_resource, &d, cpu_handle)
        }
    }

    com_add_ref(tex.d3d12_resource);
    wr(
        out,
        Box::into_raw(Box::new(NriDescriptor {
            device: dev,
            heap_type,
            chunk_idx,
            slot_idx,
            cpu_handle,
            d3d12_resource: tex.d3d12_resource,
            buffer_gpu_va: 0,
            is_float_uav,
        })),
    );
    NRI_RESULT_SUCCESS
}

pub(super) unsafe extern "win64" fn core_create_texture_1d_view(
    desc: *const NriTextureViewDesc,
    out: *mut *mut NriDescriptor,
) -> u32 {
    create_texture_view_common(desc, 1, out)
}

pub(super) unsafe extern "win64" fn core_create_texture_2d_view(
    desc: *const NriTextureViewDesc,
    out: *mut *mut NriDescriptor,
) -> u32 {
    create_texture_view_common(desc, 2, out)
}

pub(super) unsafe extern "win64" fn core_create_texture_3d_view(
    desc: *const NriTextureViewDesc,
    out: *mut *mut NriDescriptor,
) -> u32 {
    create_texture_view_common(desc, 3, out)
}

pub(super) unsafe extern "win64" fn core_create_sampler(
    device: *mut NriDevice,
    desc_ptr: *const NriSamplerDesc,
    out: *mut *mut NriDescriptor,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let sd = rd(desc_ptr);
    let use_aniso = sd.anisotropy > 1;
    let use_cmp = sd.compare_func != 0;
    let border = match sd.border_color {
        1 | 4 => [0.0, 0.0, 0.0, 1.0],
        2 | 5 => [1.0, 1.0, 1.0, 1.0],
        _ => [0.0, 0.0, 0.0, 0.0],
    };
    let d3d_sd = D3dSamplerDesc {
        filter: nri_sampler_filter(
            sd.mip,
            sd.magnification,
            sd.minification,
            sd.filter_ext,
            use_aniso,
            use_cmp,
        ),
        address: [
            nri_address_mode(sd.address_u),
            nri_address_mode(sd.address_v),
            nri_address_mode(sd.address_w),
        ],
        mip_lod_bias: sd.mip_bias,
        max_anisotropy: sd.anisotropy.max(1),
        comparison: nri_compare_func(sd.compare_func),
        border,
        min_lod: sd.mip_min,
        max_lod: sd.mip_max,
    };
    let (chunk_idx, slot_idx, cpu_handle) = match (*device).alloc_cpu_descriptor(1) {
        Ok(x) => x,
        Err(e) => return e,
    };
    d3d12_dev_create_sampler((*device).d3d12_device, &d3d_sd, cpu_handle);
    wr(
        out,
        Box::into_raw(Box::new(NriDescriptor {
            device,
            heap_type: 1,
            chunk_idx,
            slot_idx,
            cpu_handle,
            d3d12_resource: ptr::null_mut(),
            buffer_gpu_va: 0,
            is_float_uav: false,
        })),
    );
    NRI_RESULT_SUCCESS
}

pub(super) unsafe extern "win64" fn core_create_pipeline_layout(
    device: *mut NriDevice,
    desc_ptr: *const NriPipelineLayoutDesc,
    out: *mut *mut NriPipelineLayout,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let pd = rd(desc_ptr);
    if (pd.descriptor_set_num > 0 && pd.descriptor_sets.is_null())
        || (pd.push_constant_num > 0 && pd.push_constants.is_null())
    {
        return NRI_RESULT_INVALID_ARGUMENT;
    }

    // PipelineLayoutShaderStageBits::ALL_GRAPHICS
    let all_graphics: u16 =
        (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5) | (1 << 13) | (1 << 14);
    let is_graphics = (pd.stage_mask & all_graphics) != 0;

    let mut root_params: Vec<RootParameter> = Vec::new();
    let mut set_mappings = Vec::with_capacity(pd.descriptor_set_num as usize);
    let mut set_root_offsets = Vec::with_capacity(pd.descriptor_set_num as usize);
    let mut dynamic_cbv_mappings = Vec::with_capacity(pd.descriptor_set_num as usize);

    for i in 0..pd.descriptor_set_num as usize {
        let ds = rd(pd.descriptor_sets.add(i));
        if (ds.range_num > 0 && ds.ranges.is_null())
            || (ds.dynamic_constant_buffer_num > 0 && ds.dynamic_constant_buffers.is_null())
        {
            return NRI_RESULT_INVALID_ARGUMENT;
        }
        let mut mapping = DescriptorSetMapping::default();
        let mut root_offsets = vec![ROOT_PARAMETER_UNUSED; ds.range_num as usize];
        let partially_bound = (ds.binding_mask & 1) != 0;

        let mut group: Vec<DescriptorRange> = Vec::new();
        let mut group_vis = 0u32;
        let mut group_type = DescriptorRangeType::Srv;
        let mut group_heap = 0u32;

        for j in 0..ds.range_num as usize {
            let r = rd(ds.ranges.add(j));
            let Some(rt) = nri_descriptor_range_type(r.descriptor_type) else {
                return NRI_RESULT_INVALID_ARGUMENT;
            };
            let ht = nri_descriptor_heap_type(r.descriptor_type);
            let heap_off = mapping.counts[ht as usize];
            mapping.ranges.push(DescriptorRangeMapping {
                heap_type: ht,
                heap_offset: heap_off,
                descriptor_num: r.descriptor_num,
            });
            mapping.counts[ht as usize] =
                mapping.counts[ht as usize].saturating_add(r.descriptor_num);

            let vis = nri_shader_visibility(r.visibility);
            if !group.is_empty() && (group_vis != vis || group_type != rt || group_heap != ht) {
                root_params.push(RootParameter {
                    kind: RootParameterKind::DescriptorTable(std::mem::take(&mut group)),
                    visibility: group_vis,
                });
            }
            if group.is_empty() {
                root_offsets[j] = root_params.len() as u16;
                group_vis = vis;
                group_type = rt;
                group_heap = ht;
            }
            let rflags = if partially_bound {
                if rt == DescriptorRangeType::Sampler {
                    0x1
                } else {
                    0x3
                }
            } else {
                0
            };
            group.push(DescriptorRange {
                range_type: rt,
                num_descriptors: r.descriptor_num,
                base_shader_register: r.base_register_index,
                register_space: ds.register_space,
                flags: rflags,
                offset_in_descriptors_from_table_start: rs_flags::DESCRIPTOR_RANGE_OFFSET_APPEND,
            });
        }
        if !group.is_empty() {
            root_params.push(RootParameter {
                kind: RootParameterKind::DescriptorTable(group),
                visibility: group_vis,
            });
        }

        let dyn_map = if ds.dynamic_constant_buffer_num > 0 {
            let base = root_params.len() as u16;
            for j in 0..ds.dynamic_constant_buffer_num as usize {
                let dcb = rd(ds.dynamic_constant_buffers.add(j));
                root_params.push(RootParameter {
                    kind: RootParameterKind::Descriptor(
                        RootDescriptorKind::Cbv,
                        RootDescriptor {
                            shader_register: dcb.register_index,
                            register_space: ds.register_space,
                            flags: 0,
                        },
                    ),
                    visibility: nri_shader_visibility(dcb.visibility),
                });
            }
            DynamicCbvMapping {
                count: ds.dynamic_constant_buffer_num as u16,
                root_offset: base,
            }
        } else {
            DynamicCbvMapping::default()
        };

        set_mappings.push(mapping);
        set_root_offsets.push(root_offsets);
        dynamic_cbv_mappings.push(dyn_map);
    }

    let push_base = root_params.len() as u32;
    for i in 0..pd.push_constant_num as usize {
        let pc = rd(pd.push_constants.add(i));
        root_params.push(RootParameter {
            kind: RootParameterKind::Constants {
                shader_register: pc.register_index,
                register_space: 0,
                num_32bit_values: pc.size.div_ceil(4),
            },
            visibility: nri_shader_visibility(pc.visibility),
        });
    }

    let mut flags = 0u32;
    if pd.stage_mask & (1 << 1) != 0 {
        flags |= rs_flags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT;
    } else {
        flags |= 0x2; // DENY_VERTEX
    }
    if pd.stage_mask & (1 << 2) == 0 {
        flags |= 0x4; // DENY_HULL
    }
    if pd.stage_mask & (1 << 3) == 0 {
        flags |= 0x8; // DENY_DOMAIN
    }
    if pd.stage_mask & (1 << 4) == 0 {
        flags |= 0x10; // DENY_GEOMETRY
    }
    if pd.stage_mask & (1 << 5) == 0 {
        flags |= 0x20; // DENY_PIXEL
    }
    if (*device).desc.mesh_work_group_invocation_max_num > 0 {
        if pd.stage_mask & (1 << 13) == 0 {
            flags |= 0x100; // DENY_AMPLIFICATION_SHADER_ROOT_ACCESS
        }
        if pd.stage_mask & (1 << 14) == 0 {
            flags |= 0x200; // DENY_MESH_SHADER_ROOT_ACCESS
        }
    }

    let rs_desc = RootSignatureDesc {
        version: RootSignatureVersion::V1_1,
        parameters: root_params,
        static_samplers: Vec::new(),
        flags,
    };
    let blob = rs_desc.to_blob();
    let mut root_sig: *mut c_void = ptr::null_mut();
    let hr = d3d12_dev_create_root_signature((*device).d3d12_device, &blob, &mut root_sig);
    if hr < 0 || root_sig.is_null() {
        return hresult_to_nri(hr);
    }

    wr(
        out,
        Box::into_raw(Box::new(NriPipelineLayout {
            d3d12_root_sig: root_sig,
            is_graphics,
            set_mappings,
            set_root_offsets,
            dynamic_cbv_mappings,
            push_constants_base_index: push_base,
        })),
    );
    NRI_RESULT_SUCCESS
}

// ---------------------------------------------------------------------------
// CoreInterface — Descriptor sets & descriptor pool management (slots 78..83)
// ---------------------------------------------------------------------------

pub(super) unsafe extern "win64" fn core_update_descriptor_ranges(
    set: *mut NriDescriptorSet,
    _physical_device_mask: u32,
    base_range: u32,
    range_num: u32,
    updates: *const NriDescriptorRangeUpdateDesc,
) {
    if set.is_null() || updates.is_null() || range_num == 0 {
        return;
    }
    let ds = &*set;
    let d3d_dev = (*ds.device).d3d12_device;
    for i in 0..range_num as usize {
        let Some(rm) = ds.ranges.get(base_range as usize + i) else {
            continue;
        };
        let u = rd(updates.add(i));
        if u.descriptor_num == 0 || u.descriptors.is_null() {
            continue;
        }
        let base_off = rm.heap_offset + u.offset_in_range;
        for j in 0..u.descriptor_num {
            let dp = rd(u.descriptors.add(j as usize));
            if dp.is_null() {
                continue;
            }
            let dst = (*ds.pool).cpu_ptr(rm.heap_type, base_off + j);
            let src = (*dp).cpu_handle;
            d3d12_dev_copy_descriptors_simple(d3d_dev, 1, dst, src, rm.heap_type);
        }
    }
}

pub(super) unsafe extern "win64" fn core_update_dynamic_constant_buffers(
    set: *mut NriDescriptorSet,
    _physical_device_mask: u32,
    base_buffer: u32,
    buffer_num: u32,
    descriptors: *const *const NriDescriptor,
) {
    if set.is_null() || descriptors.is_null() || buffer_num == 0 {
        return;
    }
    let ds = &mut *set;
    for i in 0..buffer_num as usize {
        let dp = rd(descriptors.add(i));
        let idx = base_buffer as usize + i;
        if !dp.is_null() && idx < ds.dynamic_cbv_vas.len() {
            ds.dynamic_cbv_vas[idx] = (*dp).buffer_gpu_va;
        }
    }
}

pub(super) unsafe extern "win64" fn core_copy_descriptor_set(
    dst_set: *mut NriDescriptorSet,
    copy_ptr: *const NriDescriptorSetCopyDesc,
) {
    if dst_set.is_null() || copy_ptr.is_null() {
        return;
    }
    let cd = rd(copy_ptr);
    if cd.src_descriptor_set.is_null() {
        return;
    }
    let dst = &mut *dst_set;
    let src = &*cd.src_descriptor_set;
    let d3d_dev = (*dst.device).d3d12_device;
    for i in 0..cd.range_num as usize {
        let dst_idx = cd.base_dst_range as usize + i;
        let src_idx = cd.base_src_range as usize + i;
        if let Some(rm) = dst.ranges.get(dst_idx) {
            let d_ptr = dst.cpu_ptr(dst_idx, 0);
            let s_ptr = src.cpu_ptr(src_idx, 0);
            if d_ptr != 0 && s_ptr != 0 && rm.descriptor_num > 0 {
                d3d12_dev_copy_descriptors_simple(
                    d3d_dev,
                    rm.descriptor_num,
                    d_ptr,
                    s_ptr,
                    rm.heap_type,
                );
            }
        }
    }
    for i in 0..cd.dynamic_cb_num as usize {
        let di = cd.base_dst_dynamic_cb as usize + i;
        let si = cd.base_src_dynamic_cb as usize + i;
        if di < dst.dynamic_cbv_vas.len() && si < src.dynamic_cbv_vas.len() {
            dst.dynamic_cbv_vas[di] = src.dynamic_cbv_vas[si];
        }
    }
}

pub(super) unsafe extern "win64" fn core_allocate_descriptor_sets(
    pool_ptr: *mut NriDescriptorPool,
    layout_ptr: *const NriPipelineLayout,
    set_index: u32,
    out_sets: *mut *mut NriDescriptorSet,
    instance_num: u32,
    _physical_device_mask: u32,
    _variable_descriptor_num: u32,
) -> u32 {
    if pool_ptr.is_null() || layout_ptr.is_null() || out_sets.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let pool = &mut *pool_ptr;
    let layout = &*layout_ptr;
    if pool.sets.len() + (instance_num as usize) > (pool.sets_max as usize) {
        return NRI_RESULT_OUT_OF_MEMORY;
    }
    let Some(mapping) = layout.set_mappings.get(set_index as usize) else {
        return NRI_RESULT_INVALID_ARGUMENT;
    };
    let dyn_count = layout
        .dynamic_cbv_mappings
        .get(set_index as usize)
        .map_or(0, |m| m.count as usize);

    for i in 0..instance_num as usize {
        let mut base_offsets = [0u32; 2];
        for ht in 0..2 {
            let need = mapping.counts[ht];
            if need > 0 {
                let cap = pool.heaps[ht].as_ref().map_or(0, |h| h.capacity);
                if pool.allocated_counts[ht].saturating_add(need) > cap {
                    return NRI_RESULT_OUT_OF_MEMORY;
                }
                base_offsets[ht] = pool.allocated_counts[ht];
                pool.allocated_counts[ht] += need;
            }
        }
        let mut ranges = mapping.ranges.clone();
        for rm in &mut ranges {
            rm.heap_offset += base_offsets[rm.heap_type as usize];
        }
        let raw_set = Box::into_raw(Box::new(NriDescriptorSet {
            device: pool.device,
            pool: pool_ptr,
            ranges,
            dynamic_cbv_vas: vec![0u64; dyn_count],
        }));
        pool.sets.push(raw_set);
        wr(out_sets.add(i), raw_set);
    }
    NRI_RESULT_SUCCESS
}

pub(super) unsafe extern "win64" fn core_reset_descriptor_pool(pool: *mut NriDescriptorPool) {
    if !pool.is_null() {
        (*pool).reset();
    }
}
