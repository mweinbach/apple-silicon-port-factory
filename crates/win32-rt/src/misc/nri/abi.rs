//! NRI v1.99 C ABI constants, descriptor structs, format/enum conversion tables,
//! and D3D12 COM vtable invocation helpers (`third_party/nri/Include/NRIDescs.h`,
//! `NRIWrapperD3D12.h`, `NRIHelper.h`).

use std::ffi::c_void;
use std::mem::{offset_of, size_of};
use std::ptr;

use super::cmdlist::NriCommandBuffer;
use super::descriptor::{NriDescriptor, NriDescriptorSet, NriPipelineLayout};
use super::device::{NriBuffer, NriMemory, NriTexture};
use crate::com::com_call_qi;
use crate::d3d12::com::{com_release, Guid, DXGI_ERROR_UNSUPPORTED};
use crate::d3d12::heap::QueryHeapDesc;
use crate::d3d12::pso::{CommandSignatureDesc, ComputePipelineDesc, StreamDesc};
use crate::d3d12::vtables::{
    IID_ID3D12CommandAllocator, IID_ID3D12CommandQueue, IID_ID3D12CommandSignature,
    IID_ID3D12DescriptorHeap, IID_ID3D12Device2, IID_ID3D12Fence, IID_ID3D12GraphicsCommandList,
    IID_ID3D12Heap, IID_ID3D12PipelineState, IID_ID3D12QueryHeap, IID_ID3D12Resource,
    IID_ID3D12RootSignature,
};
use crate::strings::{from_ansi, rd, to_wide_z};
use crate::types::{E_INVALIDARG, E_NOTIMPL, E_OUTOFMEMORY, E_POINTER, HANDLE, HRESULT};
use d3d12_metal::metal::views::SamplerDesc as D3dSamplerDesc;
use d3d12_metal::types::{
    dim, heap_type, resource_flags, AllocationInfo, ClearValue, CommandQueueDesc,
    DescriptorHeapDesc, HeapDesc, HeapProperties, ResourceDesc,
};
use dxil_metal::root_signature::DescriptorRangeType;

// ---------------------------------------------------------------------------
// NRI constants & enums (`third_party/nri/Include/NRIDescs.h`)
// ---------------------------------------------------------------------------

pub const NRI_RESULT_SUCCESS: u32 = 0;
pub const NRI_RESULT_FAILURE: u32 = 1;
pub const NRI_RESULT_INVALID_ARGUMENT: u32 = 2;
pub const NRI_RESULT_OUT_OF_MEMORY: u32 = 3;
pub const NRI_RESULT_UNSUPPORTED: u32 = 4;
pub const NRI_RESULT_DEVICE_LOST: u32 = 5;

pub const NRI_GRAPHICS_API_D3D12: u8 = 1;

pub(super) const NRI_VERSION_MAJOR: u16 = 1;
pub(super) const NRI_VERSION_MINOR: u16 = 99;

pub(super) const NRI_WHOLE_SIZE: u64 = 0;
pub(super) const NRI_REMAINING_MIP_LEVELS: u16 = 0;
pub(super) const NRI_REMAINING_ARRAY_LAYERS: u16 = 0;
pub(super) const NRI_WHOLE_DEVICE_GROUP: u32 = 0;

// D3D12 constants matching `third_party/nri/Source/D3D12/`.
pub(super) const D3D12_DEFAULT_RESOURCE_PLACEMENT_ALIGNMENT: u64 = 65536;
pub(super) const D3D12_TEXTURE_LAYOUT_UNKNOWN: u32 = 0;
pub(super) const D3D12_TEXTURE_LAYOUT_ROW_MAJOR: u32 = 1;
pub(super) const D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: u32 = 0x1688;
pub(super) const D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES: u32 = 0xffff_ffff;
const D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS: u32 = 0x40;
const D3D12_HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES: u32 = 0x80;
const D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES: u32 = 0xC0;
pub(super) const D3D12_HEAP_FLAG_CREATE_NOT_ZEROED: u32 = 0x1000;
pub(super) const D3D12_DESCRIPTOR_HEAP_FLAG_NONE: u32 = 0;
pub(super) const D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE: u32 = 1;
const DXGI_ERROR_DEVICE_REMOVED: HRESULT = 0x887A_0005u32 as i32;
const DXGI_ERROR_DEVICE_RESET: HRESULT = 0x887A_0007u32 as i32;

pub(super) const CPU_DESCRIPTOR_BATCH_SIZE: u32 = 1024;
pub(super) const ROOT_PARAMETER_UNUSED: u16 = u16::MAX;
pub(super) const COPY_ALIGNMENT: u64 = 512;

#[repr(C, align(16))]
pub(super) struct InterfaceTable<const N: usize>(pub(super) [crate::Ptr; N]);
unsafe impl<const N: usize> Sync for InterfaceTable<N> {}

// ---------------------------------------------------------------------------
// NRI C ABI descriptors (`third_party/nri/Include/NRIDescs.h`,
// `NRIWrapperD3D12.h`, `NRIHelper.h`)
// ---------------------------------------------------------------------------

/// `nri::AdapterDesc` (`third_party/nri/Include/NRIDescs.h`).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriAdapterDesc {
    pub description: [u8; 128],     // +0x00
    pub luid: u64,                  // +0x80
    pub video_memory_size: u64,     // +0x88
    pub system_memory_size: u64,    // +0x90
    pub device_id: u32,             // +0x98
    pub vendor: u8,                 // +0x9c
    pub _pad: [u8; 3],              // +0x9d..+0xa0
}

const _: () = assert!(size_of::<NriAdapterDesc>() == 0xa0);

/// `nri::DeviceDesc` (`third_party/nri/Include/NRIDescs.h`).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriDeviceDesc {
    // Common (+0x00..+0xa8)
    pub adapter_desc: NriAdapterDesc,                   // +0x00
    pub graphics_api: u8,                               // +0xa0
    pub _pad0: u8,                                      // +0xa1
    pub nri_version_major: u16,                         // +0xa2
    pub nri_version_minor: u16,                         // +0xa4
    pub _pad1: u16,                                     // +0xa6

    // Viewports (+0xa8..+0xb8)
    pub viewport_max_num: u32,                          // +0xa8
    pub viewport_sub_pixel_bits: u32,                   // +0xac
    pub viewport_bounds_range: [i32; 2],                // +0xb0

    // Framebuffer (+0xb8..+0xc4)
    pub frame_buffer_max_dim: u32,                      // +0xb8
    pub frame_buffer_layer_max_num: u32,                // +0xbc
    pub framebuffer_color_attachment_max_num: u32,      // +0xc0

    // Multi-sampling (+0xc4..+0xd0)
    pub frame_buffer_color_sample_max_num: u8,          // +0xc4
    pub frame_buffer_depth_sample_max_num: u8,          // +0xc5
    pub frame_buffer_stencil_sample_max_num: u8,        // +0xc6
    pub frame_buffer_no_attachments_sample_max_num: u8, // +0xc7
    pub texture_color_sample_max_num: u8,               // +0xc8
    pub texture_integer_sample_max_num: u8,             // +0xc9
    pub texture_depth_sample_max_num: u8,               // +0xca
    pub texture_stencil_sample_max_num: u8,             // +0xcb
    pub storage_texture_sample_max_num: u8,             // +0xcc
    pub _pad2: [u8; 3],                                 // +0xcd

    // Resource dimensions (+0xd0..+0xe4)
    pub texture_1d_max_dim: u32,                        // +0xd0
    pub texture_2d_max_dim: u32,                        // +0xd4
    pub texture_3d_max_dim: u32,                        // +0xd8
    pub texture_array_max_dim: u32,                     // +0xdc
    pub texel_buffer_max_dim: u32,                      // +0xe0

    // Memory (+0xe4..+0x11c)
    pub memory_allocation_max_num: u32,                 // +0xe4
    pub sampler_allocation_max_num: u32,                // +0xe8
    pub upload_buffer_texture_row_alignment: u32,       // +0xec
    pub upload_buffer_texture_slice_alignment: u32,     // +0xf0
    pub typed_buffer_offset_alignment: u32,             // +0xf4
    pub constant_buffer_offset_alignment: u32,          // +0xf8
    pub constant_buffer_max_range: u32,                 // +0xfc
    pub storage_buffer_offset_alignment: u32,           // +0x100
    pub storage_buffer_max_range: u32,                  // +0x104
    pub buffer_texture_granularity: u32,                // +0x108
    pub _pad3: u32,                                     // +0x10c
    pub buffer_max_size: u64,                           // +0x110
    pub push_constants_max_size: u32,                   // +0x118

    // Shader resources (+0x11c..+0x138)
    pub bound_descriptor_set_max_num: u32,              // +0x11c
    pub per_stage_descriptor_sampler_max_num: u32,      // +0x120
    pub per_stage_descriptor_constant_buffer_max_num: u32, // +0x124
    pub per_stage_descriptor_storage_buffer_max_num: u32,  // +0x128
    pub per_stage_descriptor_texture_max_num: u32,      // +0x12c
    pub per_stage_descriptor_storage_texture_max_num: u32, // +0x130
    pub per_stage_resource_max_num: u32,                // +0x134

    // Descriptor set (+0x138..+0x14c)
    pub descriptor_set_sampler_max_num: u32,            // +0x138
    pub descriptor_set_constant_buffer_max_num: u32,    // +0x13c
    pub descriptor_set_storage_buffer_max_num: u32,     // +0x140
    pub descriptor_set_texture_max_num: u32,            // +0x144
    pub descriptor_set_storage_texture_max_num: u32,    // +0x148

    // Vertex shader (+0x14c..+0x158)
    pub vertex_shader_attribute_max_num: u32,           // +0x14c
    pub vertex_shader_stream_max_num: u32,              // +0x150
    pub vertex_shader_output_component_max_num: u32,    // +0x154

    // Tessellation control shader (+0x158..+0x170)
    pub tess_control_shader_generation_max_level: f32,  // +0x158
    pub tess_control_shader_patch_point_max_num: u32,   // +0x15c
    pub tess_control_shader_per_vertex_input_component_max_num: u32,  // +0x160
    pub tess_control_shader_per_vertex_output_component_max_num: u32, // +0x164
    pub tess_control_shader_per_patch_output_component_max_num: u32,  // +0x168
    pub tess_control_shader_total_output_component_max_num: u32,      // +0x16c

    // Tessellation evaluation shader (+0x170..+0x178)
    pub tess_evaluation_shader_input_component_max_num: u32,  // +0x170
    pub tess_evaluation_shader_output_component_max_num: u32, // +0x174

    // Geometry shader (+0x178..+0x18c)
    pub geometry_shader_invocation_max_num: u32,        // +0x178
    pub geometry_shader_input_component_max_num: u32,   // +0x17c
    pub geometry_shader_output_component_max_num: u32,  // +0x180
    pub geometry_shader_output_vertex_max_num: u32,     // +0x184
    pub geometry_shader_total_output_component_max_num: u32, // +0x188

    // Fragment shader (+0x18c..+0x19c)
    pub fragment_shader_input_component_max_num: u32,   // +0x18c
    pub fragment_shader_output_attachment_max_num: u32, // +0x190
    pub fragment_shader_dual_source_attachment_max_num: u32, // +0x194
    pub fragment_shader_combined_output_resource_max_num: u32, // +0x198

    // Compute shader (+0x19c..+0x1c0)
    pub compute_shader_shared_memory_max_size: u32,     // +0x19c
    pub compute_shader_work_group_max_num: [u32; 3],    // +0x1a0
    pub compute_shader_work_group_invocation_max_num: u32, // +0x1ac
    pub compute_shader_work_group_max_dim: [u32; 3],    // +0x1b0
    pub _pad4: u32,                                     // +0x1bc

    // Ray tracing (+0x1c0..+0x1e0)
    pub ray_tracing_shader_group_identifier_size: u64,  // +0x1c0
    pub ray_tracing_shader_table_alignment: u64,        // +0x1c8
    pub ray_tracing_shader_table_max_stride: u64,       // +0x1d0
    pub ray_tracing_shader_recursion_max_depth: u32,    // +0x1d8
    pub ray_tracing_geometry_object_max_num: u32,       // +0x1dc

    // Mesh shader (+0x1e0..+0x220)
    pub mesh_task_work_group_invocation_max_num: u32,   // +0x1e0
    pub mesh_task_work_group_max_dim: [u32; 3],         // +0x1e4
    pub mesh_task_total_memory_max_size: u32,           // +0x1f0
    pub mesh_task_output_max_num: u32,                  // +0x1f4
    pub mesh_work_group_invocation_max_num: u32,        // +0x1f8
    pub mesh_work_group_max_dim: [u32; 3],              // +0x1fc
    pub mesh_output_vertex_max_num: u32,                // +0x208
    pub mesh_output_primitive_max_num: u32,             // +0x20c
    pub mesh_multiview_view_max_num: u32,               // +0x210
    pub mesh_output_per_vertex_granularity: u32,        // +0x214
    pub mesh_output_per_primitive_granularity: u32,     // +0x218
    pub _pad5: u32,                                     // +0x21c

    // Other (+0x220..+0x26a)
    pub timestamp_frequency_hz: u64,                    // +0x220
    pub sub_pixel_precision_bits: u32,                  // +0x228
    pub sub_texel_precision_bits: u32,                  // +0x22c
    pub mipmap_precision_bits: u32,                     // +0x230
    pub draw_indexed_index_16_value_max: u32,           // +0x234
    pub draw_indexed_index_32_value_max: u32,           // +0x238
    pub draw_indirect_max_num: u32,                     // +0x23c
    pub sampler_lod_bias_min: f32,                      // +0x240
    pub sampler_lod_bias_max: f32,                      // +0x244
    pub sampler_anisotropy_max: f32,                    // +0x248
    pub texel_offset_min: i32,                          // +0x24c
    pub texel_offset_max: u32,                          // +0x250
    pub texel_gather_offset_min: i32,                   // +0x254
    pub texel_gather_offset_max: u32,                   // +0x258
    pub clip_distance_max_num: u32,                     // +0x25c
    pub cull_distance_max_num: u32,                     // +0x260
    pub combined_clip_and_cull_distance_max_num: u32,   // +0x264
    pub conservative_raster_tier: u8,                   // +0x268
    pub physical_device_num: u8,                        // +0x269

    // Features support (+0x26a..+0x278)
    pub is_api_validation_enabled: bool,                // +0x26a
    pub is_texture_filter_min_max_supported: bool,      // +0x26b
    pub is_logic_op_supported: bool,                    // +0x26c
    pub is_depth_bounds_test_supported: bool,           // +0x26d
    pub is_programmable_sample_locations_supported: bool, // +0x26e
    pub is_compute_queue_supported: bool,               // +0x26f
    pub is_copy_queue_supported: bool,                  // +0x270
    pub is_copy_queue_timestamp_supported: bool,        // +0x271
    pub is_register_aliasing_supported: bool,           // +0x272
    pub is_subset_allocation_supported: bool,           // +0x273
    pub is_float16_supported: bool,                     // +0x274
    pub _pad6: [u8; 3],                                 // +0x275..+0x278
}

unsafe impl Sync for NriDeviceDesc {}

const _: () = {
    assert!(offset_of!(NriDeviceDesc, graphics_api) == 0xa0);
    assert!(offset_of!(NriDeviceDesc, nri_version_major) == 0xa2);
    assert!(offset_of!(NriDeviceDesc, nri_version_minor) == 0xa4);
    assert!(offset_of!(NriDeviceDesc, upload_buffer_texture_row_alignment) == 0xec);
    assert!(offset_of!(NriDeviceDesc, upload_buffer_texture_slice_alignment) == 0xf0);
    assert!(offset_of!(NriDeviceDesc, typed_buffer_offset_alignment) == 0xf4);
    assert!(offset_of!(NriDeviceDesc, constant_buffer_offset_alignment) == 0xf8);
    assert!(offset_of!(NriDeviceDesc, storage_buffer_offset_alignment) == 0x100);
    assert!(offset_of!(NriDeviceDesc, buffer_texture_granularity) == 0x108);
    assert!(size_of::<NriDeviceDesc>() == 0x278);
};

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NriMemoryDesc {
    pub size: u64,             // +0x00
    pub alignment: u32,        // +0x08
    pub memory_type: u32,      // +0x0c
    pub must_be_dedicated: u8, // +0x10
    pub _pad: [u8; 7],         // +0x11..+0x18
}
const _: () = assert!(size_of::<NriMemoryDesc>() == 0x18);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriDeviceCreationD3D12Desc {
    pub d3d12_device: *mut c_void,
    pub d3d12_graphics_queue: *mut c_void,
    pub d3d12_compute_queue: *mut c_void,
    pub d3d12_copy_queue: *mut c_void,
    pub callback_interface: [usize; 3],
    pub memory_allocator_interface: [usize; 4],
    pub enable_nri_validation: u8,
    pub _pad: [u8; 7],
}
const _: () = {
    assert!(offset_of!(NriDeviceCreationD3D12Desc, d3d12_graphics_queue) == 0x08);
    assert!(offset_of!(NriDeviceCreationD3D12Desc, enable_nri_validation) == 0x58);
    assert!(size_of::<NriDeviceCreationD3D12Desc>() == 0x60);
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriCommandBufferD3D12Desc {
    pub d3d12_command_list: *mut c_void,
    pub d3d12_command_allocator: *mut c_void,
}
const _: () = assert!(size_of::<NriCommandBufferD3D12Desc>() == 0x10);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriBufferD3D12Desc {
    pub d3d12_resource: *mut c_void,
    pub structure_stride: u32,
    pub _pad: u32,
}
const _: () = assert!(size_of::<NriBufferD3D12Desc>() == 0x10);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriTextureD3D12Desc {
    pub d3d12_resource: *mut c_void,
}
const _: () = assert!(size_of::<NriTextureD3D12Desc>() == 0x08);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriMemoryD3D12Desc {
    pub d3d12_heap: *mut c_void,
}
const _: () = assert!(size_of::<NriMemoryD3D12Desc>() == 0x08);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriAccelerationStructureD3D12Desc {
    pub d3d12_resource: *mut c_void,
    pub scratch_data_size_in_bytes: u64,
    pub update_scratch_data_size_in_bytes: u64,
}
const _: () = assert!(size_of::<NriAccelerationStructureD3D12Desc>() == 0x18);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NriRect {
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
}
const _: () = assert!(size_of::<NriRect>() == 16);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NriViewport {
    pub offset: [f32; 2],
    pub size: [f32; 2],
    pub depth_range_min: f32,
    pub depth_range_max: f32,
}
const _: () = assert!(size_of::<NriViewport>() == 24);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NriClearDesc {
    pub value: [u32; 4],
    pub attachment_content_type: u8,
    pub _pad: [u8; 3],
    pub color_attachment_index: u32,
}
const _: () = assert!(size_of::<NriClearDesc>() == 24);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriClearStorageBufferDesc {
    pub storage_buffer: *const NriDescriptor,
    pub value: u32,
    pub set_index_in_pipeline_layout: u32,
    pub range_index: u32,
    pub offset_in_range: u32,
}
const _: () = assert!(size_of::<NriClearStorageBufferDesc>() == 24);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriClearStorageTextureDesc {
    pub storage_texture: *const NriDescriptor,
    pub value: [u32; 4],
    pub set_index_in_pipeline_layout: u32,
    pub range_index: u32,
    pub offset_in_range: u32,
    pub _pad: u32,
}
const _: () = assert!(size_of::<NriClearStorageTextureDesc>() == 40);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NriTextureRegionDesc {
    pub offset: [u16; 3],
    pub size: [u16; 3],
    pub mip_offset: u16,
    pub array_offset: u16,
}
const _: () = assert!(size_of::<NriTextureRegionDesc>() == 16);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NriTextureDataLayoutDesc {
    pub offset: u64,
    pub row_pitch: u32,
    pub slice_pitch: u32,
}
const _: () = assert!(size_of::<NriTextureDataLayoutDesc>() == 16);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriQueueSubmitDesc {
    pub command_buffers: *const *const NriCommandBuffer,
    pub command_buffer_num: u32,
    pub physical_device_index: u32,
}
const _: () = assert!(size_of::<NriQueueSubmitDesc>() == 16);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriBufferMemoryBindingDesc {
    pub memory: *mut NriMemory,
    pub buffer: *mut NriBuffer,
    pub offset: u64,
    pub physical_device_mask: u32,
    pub _pad: u32,
}
const _: () = assert!(size_of::<NriBufferMemoryBindingDesc>() == 32);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriTextureMemoryBindingDesc {
    pub memory: *mut NriMemory,
    pub texture: *mut NriTexture,
    pub offset: u64,
    pub physical_device_mask: u32,
    pub _pad: u32,
}
const _: () = assert!(size_of::<NriTextureMemoryBindingDesc>() == 32);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NriSamplerDesc {
    pub magnification: u8,
    pub minification: u8,
    pub mip: u8,
    pub filter_ext: u8,
    pub anisotropy: u32,
    pub mip_bias: f32,
    pub mip_min: f32,
    pub mip_max: f32,
    pub address_u: u8,
    pub address_v: u8,
    pub address_w: u8,
    pub compare_func: u8,
    pub border_color: u8,
    pub unnormalized_coordinates: u8,
    pub _pad: [u8; 2],
}
const _: () = {
    assert!(offset_of!(NriSamplerDesc, mip_max) == 0x10);
    assert!(offset_of!(NriSamplerDesc, address_u) == 0x14);
    assert!(size_of::<NriSamplerDesc>() == 28);
};

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NriTextureDesc {
    pub texture_type: u8,
    pub _pad0: u8,
    pub usage_mask: u16,
    pub format: u8,
    pub _pad1: u8,
    pub size: [u16; 3],
    pub mip_num: u16,
    pub array_size: u16,
    pub sample_num: u8,
    pub _pad2: [u8; 3],
    pub physical_device_mask: u32,
}
const _: () = {
    assert!(offset_of!(NriTextureDesc, usage_mask) == 0x02);
    assert!(offset_of!(NriTextureDesc, format) == 0x04);
    assert!(offset_of!(NriTextureDesc, size) == 0x06);
    assert!(offset_of!(NriTextureDesc, mip_num) == 0x0c);
    assert!(offset_of!(NriTextureDesc, array_size) == 0x0e);
    assert!(offset_of!(NriTextureDesc, sample_num) == 0x10);
    assert!(size_of::<NriTextureDesc>() == 24);
};

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NriBufferDesc {
    pub size: u64,
    pub structure_stride: u32,
    pub usage_mask: u16,
    pub _pad0: u16,
    pub physical_device_mask: u32,
    pub _pad1: u32,
}
const _: () = {
    assert!(offset_of!(NriBufferDesc, structure_stride) == 0x08);
    assert!(offset_of!(NriBufferDesc, usage_mask) == 0x0c);
    assert!(size_of::<NriBufferDesc>() == 24);
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriTextureViewDesc {
    pub texture: *const NriTexture,
    pub view_type: u8,
    pub format: u8,
    pub mip_offset: u16,
    pub mip_num: u16,
    pub array_or_slice_offset: u16,
    pub array_or_slice_size: u16,
    pub _pad0: u16,
    pub physical_device_mask: u32,
    pub flags: u8,
    pub _pad1: [u8; 7],
}
const _: () = assert!(size_of::<NriTextureViewDesc>() == 32);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriBufferViewDesc {
    pub buffer: *const NriBuffer,
    pub view_type: u8,
    pub format: u8,
    pub _pad0: [u8; 6],
    pub offset: u64,
    pub size: u64,
    pub physical_device_mask: u32,
    pub _pad1: u32,
}
const _: () = {
    assert!(offset_of!(NriBufferViewDesc, view_type) == 0x08);
    assert!(offset_of!(NriBufferViewDesc, offset) == 0x10);
    assert!(offset_of!(NriBufferViewDesc, size) == 0x18);
    assert!(size_of::<NriBufferViewDesc>() == 40);
};

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NriDescriptorPoolDesc {
    pub physical_device_mask: u32,
    pub descriptor_set_max_num: u32,
    pub sampler_max_num: u32,
    pub constant_buffer_max_num: u32,
    pub dynamic_constant_buffer_max_num: u32,
    pub texture_max_num: u32,
    pub storage_texture_max_num: u32,
    pub buffer_max_num: u32,
    pub storage_buffer_max_num: u32,
    pub structured_buffer_max_num: u32,
    pub storage_structured_buffer_max_num: u32,
    pub acceleration_structure_max_num: u32,
}
const _: () = assert!(size_of::<NriDescriptorPoolDesc>() == 48);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriTextureTransitionBarrierDesc {
    pub texture: *const NriTexture,
    pub mip_offset: u16,
    pub mip_num: u16,
    pub array_offset: u16,
    pub array_size: u16,
    pub prev_access: u16,
    pub next_access: u16,
    pub prev_layout: u8,
    pub next_layout: u8,
    pub _pad: u16,
}
const _: () = assert!(size_of::<NriTextureTransitionBarrierDesc>() == 24);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriBufferTransitionBarrierDesc {
    pub buffer: *const NriBuffer,
    pub prev_access: u16,
    pub next_access: u16,
    pub _pad: u32,
}
const _: () = assert!(size_of::<NriBufferTransitionBarrierDesc>() == 16);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriBufferAliasingBarrierDesc {
    pub before: *const NriBuffer,
    pub after: *const NriBuffer,
    pub next_access: u16,
    pub _pad: [u8; 6],
}
const _: () = assert!(size_of::<NriBufferAliasingBarrierDesc>() == 24);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriTextureAliasingBarrierDesc {
    pub before: *const NriTexture,
    pub after: *const NriTexture,
    pub next_access: u16,
    pub next_layout: u8,
    pub _pad: [u8; 5],
}
const _: () = assert!(size_of::<NriTextureAliasingBarrierDesc>() == 24);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriTransitionBarrierDesc {
    pub buffers: *const NriBufferTransitionBarrierDesc,
    pub textures: *const NriTextureTransitionBarrierDesc,
    pub buffer_num: u32,
    pub texture_num: u32,
}
const _: () = assert!(size_of::<NriTransitionBarrierDesc>() == 24);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriAliasingBarrierDesc {
    pub buffers: *const NriBufferAliasingBarrierDesc,
    pub textures: *const NriTextureAliasingBarrierDesc,
    pub buffer_num: u32,
    pub texture_num: u32,
}
const _: () = assert!(size_of::<NriAliasingBarrierDesc>() == 24);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NriDescriptorRangeDesc {
    pub base_register_index: u32,
    pub descriptor_num: u32,
    pub descriptor_type: u8,
    pub visibility: u8,
    pub is_descriptor_num_variable: u8,
    pub is_array: u8,
}
const _: () = assert!(size_of::<NriDescriptorRangeDesc>() == 12);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NriDynamicConstantBufferDesc {
    pub register_index: u32,
    pub visibility: u8,
    pub _pad: [u8; 3],
}
const _: () = assert!(size_of::<NriDynamicConstantBufferDesc>() == 8);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriDescriptorSetDesc {
    pub register_space: u32,
    pub _pad0: u32,
    pub ranges: *const NriDescriptorRangeDesc,
    pub range_num: u32,
    pub _pad1: u32,
    pub dynamic_constant_buffers: *const NriDynamicConstantBufferDesc,
    pub dynamic_constant_buffer_num: u32,
    pub binding_mask: u8,
    pub _pad2: [u8; 3],
}
const _: () = assert!(size_of::<NriDescriptorSetDesc>() == 40);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriDescriptorRangeUpdateDesc {
    pub descriptors: *const *const NriDescriptor,
    pub descriptor_num: u32,
    pub offset_in_range: u32,
}
const _: () = assert!(size_of::<NriDescriptorRangeUpdateDesc>() == 16);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriDescriptorSetCopyDesc {
    pub src_descriptor_set: *const NriDescriptorSet,
    pub base_src_range: u32,
    pub base_dst_range: u32,
    pub range_num: u32,
    pub base_src_dynamic_cb: u32,
    pub base_dst_dynamic_cb: u32,
    pub dynamic_cb_num: u32,
    pub physical_device_mask: u32,
    pub _pad: u32,
}
const _: () = assert!(size_of::<NriDescriptorSetCopyDesc>() == 40);

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NriPushConstantDesc {
    pub register_index: u32,
    pub size: u32,
    pub visibility: u8,
    pub _pad: [u8; 3],
}
const _: () = assert!(size_of::<NriPushConstantDesc>() == 12);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriShaderDesc {
    pub stage: u8,
    pub _pad0: [u8; 7],
    pub bytecode: *const u8,
    pub size: u64,
    pub entry_point_name: *const u8,
}
const _: () = assert!(size_of::<NriShaderDesc>() == 32);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriVertexAttributeD3D {
    pub semantic_name: *const u8,
    pub semantic_index: u32,
    pub _pad: u32,
}
const _: () = assert!(size_of::<NriVertexAttributeD3D>() == 16);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriVertexAttributeDesc {
    pub d3d: NriVertexAttributeD3D,
    pub vk_location: u32,
    pub offset: u32,
    pub format: u8,
    pub _pad0: u8,
    pub stream_index: u16,
    pub _pad1: u32,
}
const _: () = assert!(size_of::<NriVertexAttributeDesc>() == 32);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriVertexStreamDesc {
    pub stride: u32,
    pub binding_slot: u16,
    pub step_rate: u8,
    pub _pad: u8,
}
const _: () = assert!(size_of::<NriVertexStreamDesc>() == 8);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriInputAssemblyDesc {
    pub attributes: *const NriVertexAttributeDesc,
    pub streams: *const NriVertexStreamDesc,
    pub attribute_num: u8,
    pub stream_num: u8,
    pub topology: u8,
    pub tess_control_point_num: u8,
    pub primitive_restart: u8,
    pub _pad: [u8; 3],
}
const _: () = assert!(size_of::<NriInputAssemblyDesc>() == 24);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriRasterizationDesc {
    pub viewport_num: u32,
    pub depth_bias_constant_factor: i32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
    pub fill_mode: u8,
    pub cull_mode: u8,
    pub sample_mask: u16,
    pub sample_num: u8,
    pub alpha_to_coverage: u8,
    pub front_counter_clockwise: u8,
    pub depth_clamp: u8,
    pub antialiased_lines: u8,
    pub rasterizer_discard: u8,
    pub conservative_rasterization: u8,
    pub _pad: u8,
}
const _: () = assert!(size_of::<NriRasterizationDesc>() == 28);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriStencilDesc {
    pub compare_func: u8,
    pub fail: u8,
    pub pass: u8,
    pub depth_fail: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriBlendingDesc {
    pub src_factor: u8,
    pub dst_factor: u8,
    pub func: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriColorAttachmentDesc {
    pub format: u8,
    pub color_blend: NriBlendingDesc,
    pub alpha_blend: NriBlendingDesc,
    pub color_write_mask: u8,
    pub blend_enabled: u8,
}
const _: () = assert!(size_of::<NriColorAttachmentDesc>() == 9);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriDepthAttachmentDesc {
    pub compare_func: u8,
    pub write: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriStencilAttachmentDesc {
    pub front: NriStencilDesc,
    pub back: NriStencilDesc,
    pub reference: u8,
    pub compare_mask: u8,
    pub write_mask: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriOutputMergerDesc {
    pub color: *const NriColorAttachmentDesc,
    pub depth: NriDepthAttachmentDesc,
    pub stencil: NriStencilAttachmentDesc,
    pub depth_stencil_format: u8,
    pub color_logic_func: u8,
    pub _pad0: u8,
    pub color_num: u32,
    pub blend_consts: [f32; 4],
    pub _pad1: u32,
}
const _: () = assert!(size_of::<NriOutputMergerDesc>() == 48);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriPipelineLayoutDesc {
    pub descriptor_sets: *const NriDescriptorSetDesc,
    pub push_constants: *const NriPushConstantDesc,
    pub descriptor_set_num: u32,
    pub push_constant_num: u32,
    pub stage_mask: u16,
    pub ignore_global_spirv_offsets: u8,
    pub _pad: [u8; 5],
}
const _: () = assert!(size_of::<NriPipelineLayoutDesc>() == 32);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriGraphicsPipelineDesc {
    pub pipeline_layout: *const NriPipelineLayout,
    pub input_assembly: *const NriInputAssemblyDesc,
    pub rasterization: *const NriRasterizationDesc,
    pub output_merger: *const NriOutputMergerDesc,
    pub shader_stages: *const NriShaderDesc,
    pub shader_stage_num: u32,
    pub _pad: u32,
}
const _: () = assert!(size_of::<NriGraphicsPipelineDesc>() == 48);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriComputePipelineDesc {
    pub pipeline_layout: *const NriPipelineLayout,
    pub compute_shader: NriShaderDesc,
}
const _: () = assert!(size_of::<NriComputePipelineDesc>() == 40);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriFrameBufferDesc {
    pub color_attachments: *const *const NriDescriptor,
    pub depth_stencil_attachment: *const NriDescriptor,
    pub color_clear_values: *const [u32; 4],
    pub depth_stencil_clear_value: *const [u32; 4],
    pub color_attachment_num: u32,
    pub physical_device_mask: u32,
    pub size: [u16; 2],
    pub layer_num: u16,
    pub _pad: u16,
}
const _: () = assert!(size_of::<NriFrameBufferDesc>() == 48);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriQueryPoolDesc {
    pub query_type: u8,
    pub _pad0: [u8; 3],
    pub capacity: u32,
    pub pipeline_stats_mask: u16,
    pub _pad1: u16,
    pub physical_device_mask: u32,
}
const _: () = assert!(size_of::<NriQueryPoolDesc>() == 16);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriResourceGroupDesc {
    pub memory_location: u8,
    pub _pad0: [u8; 7],
    pub textures: *const *mut NriTexture,
    pub texture_num: u32,
    pub _pad1: u32,
    pub buffers: *const *mut NriBuffer,
    pub buffer_num: u32,
    pub _pad2: u32,
}
const _: () = assert!(size_of::<NriResourceGroupDesc>() == 40);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriTextureSubresourceUploadDesc {
    pub slices: *const u8,
    pub slice_num: u32,
    pub row_pitch: u32,
    pub slice_pitch: u32,
    pub _pad: u32,
}
const _: () = assert!(size_of::<NriTextureSubresourceUploadDesc>() == 24);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriTextureUploadDesc {
    pub subresources: *const NriTextureSubresourceUploadDesc,
    pub texture: *mut NriTexture,
    pub next_access: u16,
    pub next_layout: u8,
    pub _pad: u8,
    pub mip_num: u16,
    pub array_size: u16,
}
const _: () = assert!(size_of::<NriTextureUploadDesc>() == 24);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NriBufferUploadDesc {
    pub data: *const u8,
    pub data_size: u64,
    pub buffer: *mut NriBuffer,
    pub buffer_offset: u64,
    pub prev_access: u16,
    pub next_access: u16,
    pub _pad: u32,
}
const _: () = assert!(size_of::<NriBufferUploadDesc>() == 40);

// ---------------------------------------------------------------------------
// D3D12 COM vtable invocation helpers (`crates/win32-rt/src/d3d12/vtables.rs`)
// ---------------------------------------------------------------------------

#[inline]
pub(super) fn hresult_to_nri(hr: HRESULT) -> u32 {
    if hr >= 0 {
        NRI_RESULT_SUCCESS
    } else if hr == E_INVALIDARG || hr == E_POINTER {
        NRI_RESULT_INVALID_ARGUMENT
    } else if hr == E_OUTOFMEMORY {
        NRI_RESULT_OUT_OF_MEMORY
    } else if hr == E_NOTIMPL || hr == DXGI_ERROR_UNSUPPORTED {
        NRI_RESULT_UNSUPPORTED
    } else if hr == DXGI_ERROR_DEVICE_REMOVED || hr == DXGI_ERROR_DEVICE_RESET {
        NRI_RESULT_DEVICE_LOST
    } else {
        NRI_RESULT_FAILURE
    }
}

#[inline]
pub(super) fn align_up(v: u64, a: u64) -> u64 {
    if a <= 1 {
        v
    } else {
        let rem = v % a;
        if rem == 0 {
            v
        } else {
            v.saturating_add(a - rem)
        }
    }
}

#[inline]
pub(super) fn put_u32(buf: &mut [u8], off: usize, v: u32) {
    buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

#[inline]
pub(super) fn put_u64(buf: &mut [u8], off: usize, v: u64) {
    buf[off..off + 8].copy_from_slice(&v.to_le_bytes());
}

pub(super) unsafe fn d3d12_set_name_cstr(obj: *mut c_void, name: *const u8) {
    if obj.is_null() || name.is_null() {
        return;
    }
    let s = from_ansi(name);
    let w = to_wide_z(&s);
    let vt = rd(obj as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *const u16) -> HRESULT =
        std::mem::transmute(rd(vt.add(6)));
    let _ = f(obj, w.as_ptr());
}

pub(super) unsafe fn d3d12_dev_get_node_count(dev: *mut c_void) -> u32 {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) -> u32 = std::mem::transmute(rd(vt.add(7)));
    f(dev)
}

pub(super) unsafe fn d3d12_dev_create_command_queue(
    dev: *mut c_void,
    desc: &CommandQueueDesc,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const CommandQueueDesc,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(rd(vt.add(8)));
    f(dev, desc, &IID_ID3D12CommandQueue, out)
}

pub(super) unsafe fn d3d12_dev_create_command_allocator(
    dev: *mut c_void,
    list_type: u32,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const Guid, *mut *mut c_void) -> HRESULT =
        std::mem::transmute(rd(vt.add(9)));
    f(dev, list_type, &IID_ID3D12CommandAllocator, out)
}

pub(super) unsafe fn d3d12_dev_create_graphics_pso(
    dev: *mut c_void,
    desc: *const u8,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const u8,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(rd(vt.add(10)));
    f(dev, desc, &IID_ID3D12PipelineState, out)
}

pub(super) unsafe fn d3d12_dev_create_pipeline_state(
    dev: *mut c_void,
    desc: &StreamDesc,
    out: &mut *mut c_void,
) -> HRESULT {
    let mut dev2: *mut c_void = ptr::null_mut();
    let qi_hr = com_call_qi(dev, &IID_ID3D12Device2, &mut dev2);
    if qi_hr < 0 || dev2.is_null() {
        return if qi_hr < 0 { qi_hr } else { E_NOTIMPL };
    }
    let vt2 = rd(dev2 as *const *const usize);
    // ID3D12Device2::CreatePipelineState is vtable slot 47.
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const StreamDesc,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(rd(vt2.add(47)));
    let hr = f(dev2, desc, &IID_ID3D12PipelineState, out);
    com_release(dev2);
    hr
}

pub(super) unsafe fn d3d12_dev_create_compute_pso(
    dev: *mut c_void,
    desc: &ComputePipelineDesc,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const ComputePipelineDesc,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(rd(vt.add(11)));
    f(dev, desc, &IID_ID3D12PipelineState, out)
}

pub(super) unsafe fn d3d12_dev_create_command_list(
    dev: *mut c_void,
    list_type: u32,
    allocator: *mut c_void,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        u32,
        u32,
        *mut c_void,
        *mut c_void,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(rd(vt.add(12)));
    f(
        dev,
        0,
        list_type,
        allocator,
        ptr::null_mut(),
        &IID_ID3D12GraphicsCommandList,
        out,
    )
}

pub(super) unsafe fn d3d12_dev_check_feature(
    dev: *mut c_void,
    feature: u32,
    data: &mut [u8],
) -> HRESULT {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *mut c_void, u32) -> HRESULT =
        std::mem::transmute(rd(vt.add(13)));
    f(
        dev,
        feature,
        data.as_mut_ptr() as *mut c_void,
        data.len() as u32,
    )
}

pub(super) unsafe fn d3d12_dev_create_descriptor_heap(
    dev: *mut c_void,
    desc: &DescriptorHeapDesc,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const DescriptorHeapDesc,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(rd(vt.add(14)));
    f(dev, desc, &IID_ID3D12DescriptorHeap, out)
}

pub(super) unsafe fn d3d12_dev_get_desc_inc_size(dev: *mut c_void, ty: u32) -> u32 {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32) -> u32 =
        std::mem::transmute(rd(vt.add(15)));
    f(dev, ty)
}

pub(super) unsafe fn d3d12_dev_create_root_signature(
    dev: *mut c_void,
    blob: &[u8],
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        u32,
        *const u8,
        usize,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(rd(vt.add(16)));
    f(dev, 0, blob.as_ptr(), blob.len(), &IID_ID3D12RootSignature, out)
}

pub(super) unsafe fn d3d12_dev_create_cbv(dev: *mut c_void, va: u64, size: u32, handle: usize) {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *const (u64, u32), usize) =
        std::mem::transmute(rd(vt.add(17)));
    let d = (va, size);
    f(dev, &d, handle);
}

pub(super) unsafe fn d3d12_dev_create_srv(
    dev: *mut c_void,
    res: *mut c_void,
    desc: &[u8; 40],
    handle: usize,
) {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [u8; 40], usize) =
        std::mem::transmute(rd(vt.add(18)));
    f(dev, res, desc, handle);
}

pub(super) unsafe fn d3d12_dev_create_uav(
    dev: *mut c_void,
    res: *mut c_void,
    desc: &[u8; 40],
    handle: usize,
) {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *const [u8; 40],
        usize,
    ) = std::mem::transmute(rd(vt.add(19)));
    f(dev, res, ptr::null_mut(), desc, handle);
}

pub(super) unsafe fn d3d12_dev_create_rtv(
    dev: *mut c_void,
    res: *mut c_void,
    desc: &[u32; 6],
    handle: usize,
) {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [u32; 6], usize) =
        std::mem::transmute(rd(vt.add(20)));
    f(dev, res, desc, handle);
}

pub(super) unsafe fn d3d12_dev_create_dsv(
    dev: *mut c_void,
    res: *mut c_void,
    desc: &[u32; 6],
    handle: usize,
) {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [u32; 6], usize) =
        std::mem::transmute(rd(vt.add(21)));
    f(dev, res, desc, handle);
}

pub(super) unsafe fn d3d12_dev_create_sampler(
    dev: *mut c_void,
    desc: &D3dSamplerDesc,
    handle: usize,
) {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *const D3dSamplerDesc, usize) =
        std::mem::transmute(rd(vt.add(22)));
    f(dev, desc, handle);
}

pub(super) unsafe fn d3d12_dev_copy_descriptors_simple(
    dev: *mut c_void,
    num: u32,
    dst: usize,
    src: usize,
    heap_type: u32,
) {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, usize, usize, u32) =
        std::mem::transmute(rd(vt.add(24)));
    f(dev, num, dst, src, heap_type);
}

pub(super) unsafe fn d3d12_dev_get_resource_alloc_info(
    dev: *mut c_void,
    desc: &ResourceDesc,
) -> AllocationInfo {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *mut AllocationInfo,
        u32,
        u32,
        *const ResourceDesc,
    ) -> *mut AllocationInfo = std::mem::transmute(rd(vt.add(25)));
    let mut info = AllocationInfo::default();
    f(dev, &mut info, 0, 1, desc);
    info
}

pub(super) unsafe fn d3d12_dev_create_committed_resource(
    dev: *mut c_void,
    props: &HeapProperties,
    heap_flags: u32,
    desc: &ResourceDesc,
    initial_state: u32,
    clear: *const ClearValue,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const HeapProperties,
        u32,
        *const ResourceDesc,
        u32,
        *const ClearValue,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(rd(vt.add(27)));
    f(
        dev,
        props,
        heap_flags,
        desc,
        initial_state,
        clear,
        &IID_ID3D12Resource,
        out,
    )
}

pub(super) unsafe fn d3d12_dev_create_heap(
    dev: *mut c_void,
    desc: &HeapDesc,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const HeapDesc,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(rd(vt.add(28)));
    f(dev, desc, &IID_ID3D12Heap, out)
}

pub(super) unsafe fn d3d12_dev_create_placed_resource(
    dev: *mut c_void,
    heap: *mut c_void,
    offset: u64,
    desc: &ResourceDesc,
    initial_state: u32,
    clear: *const ClearValue,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *mut c_void,
        u64,
        *const ResourceDesc,
        u32,
        *const ClearValue,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(rd(vt.add(29)));
    f(
        dev,
        heap,
        offset,
        desc,
        initial_state,
        clear,
        &IID_ID3D12Resource,
        out,
    )
}

pub(super) unsafe fn d3d12_dev_create_fence(
    dev: *mut c_void,
    initial: u64,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        u64,
        u32,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(rd(vt.add(36)));
    f(dev, initial, 0, &IID_ID3D12Fence, out)
}

pub(super) unsafe fn d3d12_dev_create_query_heap(
    dev: *mut c_void,
    desc: &QueryHeapDesc,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const QueryHeapDesc,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(rd(vt.add(39)));
    f(dev, desc, &IID_ID3D12QueryHeap, out)
}

pub(super) unsafe fn d3d12_dev_create_command_signature(
    dev: *mut c_void,
    desc: &CommandSignatureDesc,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = rd(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const CommandSignatureDesc,
        *mut c_void,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(rd(vt.add(41)));
    f(dev, desc, ptr::null_mut(), &IID_ID3D12CommandSignature, out)
}

pub(super) unsafe fn d3d12_queue_execute(queue: *mut c_void, lists: &[*mut c_void]) {
    let vt = rd(queue as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void) =
        std::mem::transmute(rd(vt.add(10)));
    f(queue, lists.len() as u32, lists.as_ptr());
}

pub(super) unsafe fn d3d12_queue_signal(
    queue: *mut c_void,
    fence: *mut c_void,
    value: u64,
) -> HRESULT {
    let vt = rd(queue as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, u64) -> HRESULT =
        std::mem::transmute(rd(vt.add(14)));
    f(queue, fence, value)
}

pub(super) unsafe fn d3d12_queue_wait(
    queue: *mut c_void,
    fence: *mut c_void,
    value: u64,
) -> HRESULT {
    let vt = rd(queue as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, u64) -> HRESULT =
        std::mem::transmute(rd(vt.add(15)));
    f(queue, fence, value)
}

pub(super) unsafe fn d3d12_queue_get_timestamp_freq(queue: *mut c_void) -> u64 {
    let vt = rd(queue as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut u64) -> HRESULT =
        std::mem::transmute(rd(vt.add(16)));
    let mut freq = 0u64;
    if f(queue, &mut freq) >= 0 && freq != 0 {
        freq
    } else {
        d3d12_metal::metal::query::timestamp_frequency()
    }
}

pub(super) unsafe fn d3d12_queue_get_desc(queue: *mut c_void) -> CommandQueueDesc {
    let vt = rd(queue as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut CommandQueueDesc) -> *mut CommandQueueDesc =
        std::mem::transmute(rd(vt.add(18)));
    let mut d = CommandQueueDesc::default();
    f(queue, &mut d);
    d
}

pub(super) unsafe fn d3d12_alloc_reset(alloc: *mut c_void) -> HRESULT {
    let vt = rd(alloc as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) -> HRESULT = std::mem::transmute(rd(vt.add(8)));
    f(alloc)
}

pub(super) unsafe fn d3d12_fence_completed(fence: *mut c_void) -> u64 {
    let vt = rd(fence as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) -> u64 = std::mem::transmute(rd(vt.add(8)));
    f(fence)
}

pub(super) unsafe fn d3d12_fence_set_event(
    fence: *mut c_void,
    value: u64,
    event: HANDLE,
) -> HRESULT {
    let vt = rd(fence as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u64, HANDLE) -> HRESULT =
        std::mem::transmute(rd(vt.add(9)));
    f(fence, value, event)
}

pub(super) unsafe fn d3d12_res_map(
    res: *mut c_void,
    begin: usize,
    end: usize,
) -> Result<*mut u8, HRESULT> {
    let vt = rd(res as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const c_void, *mut *mut c_void) -> HRESULT =
        std::mem::transmute(rd(vt.add(8)));
    let range = [begin, end];
    let mut data: *mut c_void = ptr::null_mut();
    let hr = f(res, 0, range.as_ptr() as *const c_void, &mut data);
    if hr >= 0 && !data.is_null() {
        Ok(data as *mut u8)
    } else {
        Err(hr)
    }
}

pub(super) unsafe fn d3d12_res_unmap(res: *mut c_void) {
    let vt = rd(res as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const c_void) =
        std::mem::transmute(rd(vt.add(9)));
    f(res, 0, ptr::null());
}

pub(super) unsafe fn d3d12_res_get_desc(res: *mut c_void) -> ResourceDesc {
    let vt = rd(res as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut ResourceDesc) -> *mut ResourceDesc =
        std::mem::transmute(rd(vt.add(10)));
    let mut d = ResourceDesc::default();
    f(res, &mut d);
    d
}

pub(super) unsafe fn d3d12_res_get_gpu_va(res: *mut c_void) -> u64 {
    let vt = rd(res as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) -> u64 = std::mem::transmute(rd(vt.add(11)));
    f(res)
}

pub(super) unsafe fn d3d12_heap_get_desc(heap: *mut c_void) -> HeapDesc {
    let vt = rd(heap as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut HeapDesc) -> *mut HeapDesc =
        std::mem::transmute(rd(vt.add(8)));
    let mut d = HeapDesc::default();
    f(heap, &mut d);
    d
}

pub(super) unsafe fn d3d12_descheap_cpu_start(heap: *mut c_void) -> usize {
    let vt = rd(heap as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut usize) -> *mut usize =
        std::mem::transmute(rd(vt.add(9)));
    let mut h = 0usize;
    f(heap, &mut h);
    h
}

pub(super) unsafe fn d3d12_descheap_gpu_start(heap: *mut c_void) -> u64 {
    let vt = rd(heap as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut u64) -> *mut u64 =
        std::mem::transmute(rd(vt.add(10)));
    let mut h = 0u64;
    f(heap, &mut h);
    h
}

// ---------------------------------------------------------------------------
// Format & enum conversion tables (`third_party/nri/Source/D3D12/ConversionD3D12.cpp`
// & `Source/Shared/SharedExternal.cpp`)
// ---------------------------------------------------------------------------

const NRI_FORMAT_COUNT: usize = 69;

static NRI_TO_DXGI: [u32; NRI_FORMAT_COUNT] = [
    0,  // 0: UNKNOWN
    61, // 1: R8_UNORM
    63, // 2: R8_SNORM
    62, // 3: R8_UINT
    64, // 4: R8_SINT
    49, // 5: RG8_UNORM
    51, // 6: RG8_SNORM
    50, // 7: RG8_UINT
    52, // 8: RG8_SINT
    87, // 9: BGRA8_UNORM
    91, // 10: BGRA8_SRGB
    28, // 11: RGBA8_UNORM
    31, // 12: RGBA8_SNORM
    30, // 13: RGBA8_UINT
    32, // 14: RGBA8_SINT
    29, // 15: RGBA8_SRGB
    56, // 16: R16_UNORM
    58, // 17: R16_SNORM
    57, // 18: R16_UINT
    59, // 19: R16_SINT
    54, // 20: R16_SFLOAT
    35, // 21: RG16_UNORM
    37, // 22: RG16_SNORM
    36, // 23: RG16_UINT
    38, // 24: RG16_SINT
    34, // 25: RG16_SFLOAT
    11, // 26: RGBA16_UNORM
    13, // 27: RGBA16_SNORM
    12, // 28: RGBA16_UINT
    14, // 29: RGBA16_SINT
    10, // 30: RGBA16_SFLOAT
    42, // 31: R32_UINT
    43, // 32: R32_SINT
    41, // 33: R32_SFLOAT
    17, // 34: RG32_UINT
    18, // 35: RG32_SINT
    16, // 36: RG32_SFLOAT
    7,  // 37: RGB32_UINT
    8,  // 38: RGB32_SINT
    6,  // 39: RGB32_SFLOAT
    3,  // 40: RGBA32_UINT
    4,  // 41: RGBA32_SINT
    2,  // 42: RGBA32_SFLOAT
    24, // 43: R10_G10_B10_A2_UNORM
    25, // 44: R10_G10_B10_A2_UINT
    26, // 45: R11_G11_B10_UFLOAT
    67, // 46: R9_G9_B9_E5_UFLOAT
    71, // 47: BC1_RGBA_UNORM
    72, // 48: BC1_RGBA_SRGB
    74, // 49: BC2_RGBA_UNORM
    75, // 50: BC2_RGBA_SRGB
    77, // 51: BC3_RGBA_UNORM
    78, // 52: BC3_RGBA_SRGB
    80, // 53: BC4_R_UNORM
    81, // 54: BC4_R_SNORM
    83, // 55: BC5_RG_UNORM
    84, // 56: BC5_RG_SNORM
    95, // 57: BC6H_RGB_UFLOAT
    96, // 58: BC6H_RGB_SFLOAT
    98, // 59: BC7_RGBA_UNORM
    99, // 60: BC7_RGBA_SRGB
    55, // 61: D16_UNORM
    45, // 62: D24_UNORM_S8_UINT
    40, // 63: D32_SFLOAT
    20, // 64: D32_SFLOAT_S8_UINT_X24
    46, // 65: R24_UNORM_X8
    47, // 66: X24_R8_UINT
    22, // 67: X32_R8_UINT_X24
    21, // 68: R32_SFLOAT_X8_X24
];

static NRI_TO_DXGI_TYPELESS: [u32; NRI_FORMAT_COUNT] = [
    0,  // UNKNOWN
    60, 60, 60, 60, // R8_TYPELESS
    48, 48, 48, 48, // R8G8_TYPELESS
    90, 90, // B8G8R8A8_TYPELESS
    27, 27, 27, 27, 27, // R8G8B8A8_TYPELESS
    53, 53, 53, 53, 53, // R16_TYPELESS
    33, 33, 33, 33, 33, // R16G16_TYPELESS
    9, 9, 9, 9, 9, // R16G16B16A16_TYPELESS
    39, 39, 39, // R32_TYPELESS
    15, 15, 15, // R32G32_TYPELESS
    5, 5, 5, // R32G32B32_TYPELESS
    1, 1, 1, // R32G32B32A32_TYPELESS
    23, 23, // R10G10B10A2_TYPELESS
    26, // R11G11B10_FLOAT
    67, // R9G9B9E5_SHAREDEXP
    70, 70, // BC1_TYPELESS
    73, 73, // BC2_TYPELESS
    76, 76, // BC3_TYPELESS
    79, 79, // BC4_TYPELESS
    82, 82, // BC5_TYPELESS
    94, 94, // BC6H_TYPELESS
    97, 97, // BC7_TYPELESS
    53, // D16 -> R16_TYPELESS
    44, // D24S8 -> R24G8_TYPELESS
    39, // D32 -> R32_TYPELESS
    19, // D32S8 -> R32G8X24_TYPELESS
    44, 44, // R24X8 / X24G8
    19, 19, // X32G8X24 / R32X8X24
];

static NRI_FP_FORMAT: [bool; NRI_FORMAT_COUNT] = [
    false, // UNKNOWN
    true, true, false, false, // R8
    true, true, false, false, // RG8
    true, true, // BGRA8
    true, true, false, false, true, // RGBA8
    true, true, false, false, true, // R16
    true, true, false, false, true, // RG16
    true, true, false, false, true, // RGBA16
    false, false, true, // R32
    false, false, true, // RG32
    false, false, true, // RGB32
    false, false, true, // RGBA32
    true, false, true, true, // packed
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, // BC
    true, true, true, true, // D*
    true, false, false, true, // depth/stencil SRV
];

static NRI_TEXEL_BLOCK_WIDTH: [u16; NRI_FORMAT_COUNT] = [
    1, // UNKNOWN
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, // BC1..BC7
    1, 1, 1, 1, 1, 1, 1, 1,
];

static NRI_TEXEL_BLOCK_SIZE: [u32; NRI_FORMAT_COUNT] = [
    1, // UNKNOWN
    1, 1, 1, 1, // R8
    2, 2, 2, 2, // RG8
    4, 4, // BGRA8
    4, 4, 4, 4, 4, // RGBA8
    2, 2, 2, 2, 2, // R16
    4, 4, 4, 4, 4, // RG16
    8, 8, 8, 8, 8, // RGBA16
    4, 4, 4, // R32
    8, 8, 8, // RG32
    12, 12, 12, // RGB32
    16, 16, 16, // RGBA32
    4, 4, 4, 4, // packed
    8, 8, 16, 16, 16, 16, 8, 8, 16, 16, 16, 16, 16, 16, // BC
    2, 4, 4, 8, // depth
    4, 4, 8, 8,
];

pub(super) fn nri_to_dxgi(fmt: u8) -> u32 {
    NRI_TO_DXGI.get(fmt as usize).copied().unwrap_or(0)
}

pub(super) fn nri_to_dxgi_typeless(fmt: u8) -> u32 {
    NRI_TO_DXGI_TYPELESS.get(fmt as usize).copied().unwrap_or(0)
}

pub(super) fn nri_is_fp_format(fmt: u8) -> bool {
    NRI_FP_FORMAT.get(fmt as usize).copied().unwrap_or(false)
}

pub(super) fn nri_texel_block_width(fmt: u8) -> u16 {
    NRI_TEXEL_BLOCK_WIDTH
        .get(fmt as usize)
        .copied()
        .unwrap_or(1)
        .max(1)
}

pub(super) fn nri_texel_block_size(fmt: u8) -> u32 {
    NRI_TEXEL_BLOCK_SIZE
        .get(fmt as usize)
        .copied()
        .unwrap_or(1)
        .max(1)
}

pub(super) fn dxgi_to_nri(dxgi: u32) -> u8 {
    for (i, &d) in NRI_TO_DXGI.iter().enumerate() {
        if d == dxgi && i != 0 {
            return i as u8;
        }
    }
    0
}

pub(super) fn shader_format_for_depth(dxgi: u32) -> u32 {
    match dxgi {
        55 => 56, // D16_UNORM -> R16_UNORM
        45 => 46, // D24_UNORM_S8_UINT -> R24_UNORM_X8_TYPELESS
        40 => 41, // D32_FLOAT -> R32_FLOAT
        20 => 21, // D32_FLOAT_S8X24_UINT -> R32_FLOAT_X8X24_TYPELESS
        other => other,
    }
}

pub(super) fn nri_queue_type_to_d3d12(ty: u8) -> Option<u32> {
    match ty {
        0 => Some(0), // GRAPHICS -> DIRECT
        1 => Some(2), // COMPUTE -> COMPUTE
        2 => Some(3), // COPY -> COPY
        _ => None,
    }
}

pub(super) fn nri_memory_type(location: u8, desc: &ResourceDesc) -> u32 {
    let ht = match location {
        0 => heap_type::DEFAULT,
        1 => heap_type::UPLOAD,
        2 => heap_type::READBACK,
        _ => heap_type::DEFAULT,
    };
    let flags = if desc.Dimension == dim::BUFFER {
        D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS
    } else if (desc.Flags
        & (resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_DEPTH_STENCIL))
        != 0
    {
        D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES
    } else {
        D3D12_HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES
    };
    flags | (ht << 16)
}

pub(super) fn memory_type_heap_type(mem_type: u32) -> u32 {
    let ht = mem_type >> 16;
    if matches!(
        ht,
        heap_type::DEFAULT | heap_type::UPLOAD | heap_type::READBACK
    ) {
        ht
    } else {
        heap_type::DEFAULT
    }
}

pub(super) fn memory_type_heap_flags(mem_type: u32) -> u32 {
    mem_type & 0xffff
}

pub(super) fn memory_type_requires_dedicated(mem_type: u32) -> bool {
    (memory_type_heap_flags(mem_type) & D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES)
        == D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES
}

pub(super) fn nri_access_to_d3d12_state(access: u16, list_type: u32) -> u32 {
    let mut s = 0u32;
    if access & ((1 << 0) | (1 << 2)) != 0 {
        s |= 0x1; // VERTEX_AND_CONSTANT_BUFFER
    }
    if access & (1 << 1) != 0 {
        s |= 0x2; // INDEX_BUFFER
    }
    if access & (1 << 3) != 0 {
        s |= 0x200; // INDIRECT_ARGUMENT
    }
    if access & (1 << 4) != 0 {
        s |= 0x40; // NON_PIXEL_SHADER_RESOURCE
        if list_type == 0 {
            s |= 0x80; // PIXEL_SHADER_RESOURCE
        }
    }
    if access & (1 << 5) != 0 {
        s |= 0x8; // UNORDERED_ACCESS
    }
    if access & (1 << 6) != 0 {
        s |= 0x4; // RENDER_TARGET
    }
    if access & (1 << 7) != 0 {
        s |= 0x10; // DEPTH_WRITE
    }
    if access & (1 << 8) != 0 {
        s |= 0x20; // DEPTH_READ
    }
    if access & (1 << 9) != 0 {
        s |= 0x800; // COPY_SOURCE
    }
    if access & (1 << 10) != 0 {
        s |= 0x400; // COPY_DEST
    }
    if access & (1 << 11) != 0 {
        s |= 0x40_0000; // RAYTRACING_ACCELERATION_STRUCTURE
    }
    if access & (1 << 12) != 0 {
        s |= 0x8; // UNORDERED_ACCESS
    }
    s
}

pub(super) fn nri_descriptor_range_type(dt: u8) -> Option<DescriptorRangeType> {
    match dt {
        0 => Some(DescriptorRangeType::Sampler),
        1 => Some(DescriptorRangeType::Cbv),
        2 | 4 | 6 | 8 => Some(DescriptorRangeType::Srv),
        3 | 5 | 7 => Some(DescriptorRangeType::Uav),
        _ => None,
    }
}

pub(super) fn nri_descriptor_heap_type(dt: u8) -> u32 {
    if dt == 0 { 1 } else { 0 } // 1 = SAMPLER, 0 = CBV_SRV_UAV
}

pub(super) fn nri_shader_visibility(stage: u8) -> u32 {
    match stage {
        1 => 1,  // VERTEX
        2 => 2,  // HULL
        3 => 3,  // DOMAIN
        4 => 4,  // GEOMETRY
        5 => 5,  // PIXEL
        13 => 6, // AMPLIFICATION
        14 => 7, // MESH
        _ => 0,  // ALL
    }
}

pub(super) fn nri_compare_func(c: u8) -> u32 {
    match c {
        1 => 8, // ALWAYS
        2 => 1, // NEVER
        3 => 2, // LESS
        4 => 4, // LESS_EQUAL
        5 => 3, // EQUAL
        6 => 7, // GREATER_EQUAL
        7 => 5, // GREATER
        8 => 6, // NOT_EQUAL
        _ => 1, // NONE -> NEVER
    }
}

pub(super) fn nri_address_mode(m: u8) -> u32 {
    match m {
        0 => 1, // REPEAT -> WRAP
        1 => 2, // MIRRORED_REPEAT -> MIRROR
        2 => 3, // CLAMP_TO_EDGE -> CLAMP
        3 => 4, // CLAMP_TO_BORDER -> BORDER
        _ => 1,
    }
}

pub(super) fn nri_stencil_op(op: u8) -> u32 {
    match op {
        0 => 1, // KEEP
        1 => 2, // ZERO
        2 => 3, // REPLACE
        3 => 4, // INCR_SAT
        4 => 5, // DECR_SAT
        5 => 6, // INVERT
        6 => 7, // INCR
        7 => 8, // DECR
        _ => 1,
    }
}

pub(super) fn nri_blend_factor(f: u8) -> u32 {
    match f {
        0 => 1,   // ZERO
        1 => 2,   // ONE
        2 => 3,   // SRC_COLOR
        3 => 4,   // INV_SRC_COLOR
        4 => 9,   // DEST_COLOR
        5 => 10,  // INV_DEST_COLOR
        6 => 5,   // SRC_ALPHA
        7 => 6,   // INV_SRC_ALPHA
        8 => 7,   // DEST_ALPHA
        9 => 8,   // INV_DEST_ALPHA
        10 => 14, // BLEND_FACTOR
        11 => 15, // INV_BLEND_FACTOR
        12 => 14, // BLEND_FACTOR
        13 => 15, // INV_BLEND_FACTOR
        14 => 11, // SRC_ALPHA_SAT
        15 => 16, // SRC1_COLOR
        16 => 17, // INV_SRC1_COLOR
        17 => 18, // SRC1_ALPHA
        18 => 19, // INV_SRC1_ALPHA
        _ => 2,
    }
}

pub(super) fn nri_blend_op(op: u8) -> u32 {
    match op {
        0 => 1, // ADD
        1 => 2, // SUBTRACT
        2 => 3, // REV_SUBTRACT
        3 => 4, // MIN
        4 => 5, // MAX
        _ => 1,
    }
}

pub(super) fn nri_logic_op(op: u8) -> u32 {
    match op {
        0 => 4,  // NOOP
        1 => 0,  // CLEAR
        2 => 7,  // AND
        3 => 8,  // AND_REVERSE
        4 => 2,  // COPY
        5 => 9,  // AND_INVERTED
        6 => 10, // XOR
        7 => 11, // OR
        8 => 12, // NOR
        9 => 15, // EQUIV
        10 => 5, // INVERT
        11 => 13, // OR_REVERSE
        12 => 3, // COPY_INVERTED
        13 => 14, // OR_INVERTED
        14 => 12, // NAND
        15 => 1, // SET
        _ => 4,
    }
}

pub(super) fn nri_topology_type(t: u8) -> u32 {
    match t {
        0 => 1,             // POINT
        1 | 2 | 5 | 6 => 2, // LINE
        3 | 4 | 7 | 8 => 3, // TRIANGLE
        9 => 4,             // PATCH
        _ => 3,
    }
}

pub(super) fn nri_primitive_topology(t: u8, patch_pts: u8) -> u32 {
    match t {
        0 => 1,  // POINTLIST
        1 => 2,  // LINELIST
        2 => 3,  // LINESTRIP
        3 => 4,  // TRIANGLELIST
        4 => 5,  // TRIANGLESTRIP
        5 => 10, // LINELIST_ADJ
        6 => 11, // LINESTRIP_ADJ
        7 => 12, // TRIANGLELIST_ADJ
        8 => 13, // TRIANGLESTRIP_ADJ
        9 => 32 + patch_pts.max(1) as u32,
        _ => 4,
    }
}

pub(super) fn nri_sampler_filter(
    mip: u8,
    mag: u8,
    min: u8,
    filter_ext: u8,
    aniso: bool,
    cmp: bool,
) -> u32 {
    if aniso {
        return match filter_ext {
            1 => 0x155, // MINIMUM_ANISOTROPIC
            2 => 0x1D5, // MAXIMUM_ANISOTROPIC
            _ => if cmp { 0xD5 } else { 0x55 },
        };
    }
    let mut mask = 0u32;
    if mip == 1 {
        mask |= 0x01;
    }
    if mag == 1 {
        mask |= 0x04;
    }
    if min == 1 {
        mask |= 0x10;
    }
    if cmp {
        mask |= 0x80;
    } else if filter_ext == 1 {
        mask |= 0x100;
    } else if filter_ext == 2 {
        mask |= 0x180;
    }
    mask
}
