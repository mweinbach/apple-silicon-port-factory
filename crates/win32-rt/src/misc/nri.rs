//! Builtin `NRI.dll` (NVIDIA Render Interface v1.99) backed by real D3D12 COM
//! objects (`crate::d3d12` / `d3d12-metal` / `dxil-metal`).
//!
//! Exposes the three `NRI.dll` entry points (`nriCreateDeviceFromD3D12Device`,
//! `nriDestroyDevice`, `nriGetInterface`) and populates:
//! * `"nri::CoreInterface"` (`0x350` bytes = 106 function pointers)
//! * `"nri::HelperInterface"` (`0x28` bytes = 5 function pointers)
//! * `"nri::WrapperD3D12Interface"` (`0x28` bytes = 5 function pointers)
//!
//! Struct layouts, enums, and D3D12 backend mappings follow `third_party/nri/`
//! (`Include/NRI.h`, `Include/NRIDescs.h`, `Include/Extensions/NRIWrapperD3D12.h`,
//! `Include/Extensions/NRIHelper.h`, and `Source/D3D12/`).

use std::collections::{BTreeMap, HashMap};
use std::ffi::{c_void, CStr};
use std::mem::{offset_of, size_of};
use std::ptr;
use std::sync::{Mutex, Once};

use crate::d3d12::cmdlist::{CopyLocation, IndexBufferView, VertexBufferView};
use crate::d3d12::com::{com_add_ref, com_release, Guid};
use crate::d3d12::heap::QueryHeapDesc;
use crate::d3d12::pso::{
    Bytecode, CommandSignatureDesc, ComputePipelineDesc, IndirectArgument, InputElementDesc,
    StreamDesc,
};
use crate::d3d12::vtables::{
    IID_ID3D12CommandAllocator, IID_ID3D12CommandQueue, IID_ID3D12CommandSignature,
    IID_ID3D12DescriptorHeap, IID_ID3D12Device2, IID_ID3D12Fence, IID_ID3D12GraphicsCommandList,
    IID_ID3D12Heap, IID_ID3D12PipelineState, IID_ID3D12QueryHeap, IID_ID3D12Resource,
    IID_ID3D12RootSignature,
};
use crate::d3d12::{cs, mtl};
use crate::types::{BOOL, E_INVALIDARG, E_NOTIMPL, E_OUTOFMEMORY, E_POINTER, HANDLE, HRESULT};
use d3d12_metal::format::{self as d3d_fmt, support1, support2};
use d3d12_metal::layout::PlacedFootprint;
use d3d12_metal::metal::cmdlist::CopyBox;
use d3d12_metal::metal::views::SamplerDesc as D3dSamplerDesc;
use d3d12_metal::state::{BlendDesc, RenderTargetBlendDesc};
use d3d12_metal::types::{
    dim, heap_type, resource_flags, AllocationInfo, ClearValue, CommandQueueDesc,
    DescriptorHeapDesc, HeapDesc, HeapProperties, Rect, ResourceBarrier, ResourceDesc, Viewport,
};
use dxil_metal::root_signature::{
    flags as rs_flags, DescriptorRange, DescriptorRangeType, RootDescriptor, RootDescriptorKind,
    RootParameter, RootParameterKind, RootSignatureDesc, RootSignatureVersion,
};

pub static ORDINALS: &[(u16, &str)] = &[];
pub static EXTRA_EXPORTS: &[crate::Export] = &[];

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

const NRI_VERSION_MAJOR: u16 = 1;
const NRI_VERSION_MINOR: u16 = 99;

const NRI_WHOLE_SIZE: u64 = 0;
const NRI_REMAINING_MIP_LEVELS: u16 = 0;
const NRI_REMAINING_ARRAY_LAYERS: u16 = 0;
const NRI_WHOLE_DEVICE_GROUP: u32 = 0;

// D3D12 constants matching `third_party/nri/Source/D3D12/`.
const D3D12_DEFAULT_RESOURCE_PLACEMENT_ALIGNMENT: u64 = 65536;
const D3D12_TEXTURE_LAYOUT_UNKNOWN: u32 = 0;
const D3D12_TEXTURE_LAYOUT_ROW_MAJOR: u32 = 1;
const D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: u32 = 0x1688;
const D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES: u32 = 0xffff_ffff;
const D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS: u32 = 0x40;
const D3D12_HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES: u32 = 0x80;
const D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES: u32 = 0xC0;
const D3D12_HEAP_FLAG_CREATE_NOT_ZEROED: u32 = 0x1000;
const D3D12_DESCRIPTOR_HEAP_FLAG_NONE: u32 = 0;
const D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE: u32 = 1;
const DXGI_ERROR_DEVICE_REMOVED: HRESULT = 0x887A_0005u32 as i32;
const DXGI_ERROR_DEVICE_RESET: HRESULT = 0x887A_0007u32 as i32;
const DXGI_ERROR_UNSUPPORTED: HRESULT = 0x887A_0004u32 as i32;

const CPU_DESCRIPTOR_BATCH_SIZE: u32 = 1024;
const ROOT_PARAMETER_UNUSED: u16 = u16::MAX;
const COPY_ALIGNMENT: u64 = 512;

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
fn hresult_to_nri(hr: HRESULT) -> u32 {
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
fn align_up(v: u64, a: u64) -> u64 {
    if a <= 1 {
        v
    } else {
        let rem = v % a;
        if rem == 0 { v } else { v.saturating_add(a - rem) }
    }
}

unsafe fn d3d12_set_name_cstr(obj: *mut c_void, name: *const u8) {
    if obj.is_null() || name.is_null() {
        return;
    }
    let s = CStr::from_ptr(name as *const std::ffi::c_char).to_string_lossy();
    let mut w: Vec<u16> = s.encode_utf16().collect();
    w.push(0);
    let vt = *(obj as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *const u16) -> HRESULT =
        std::mem::transmute(*vt.add(6));
    let _ = f(obj, w.as_ptr());
}

unsafe fn d3d12_dev_get_node_count(dev: *mut c_void) -> u32 {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) -> u32 = std::mem::transmute(*vt.add(7));
    f(dev)
}

unsafe fn d3d12_dev_create_command_queue(
    dev: *mut c_void,
    desc: &CommandQueueDesc,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const CommandQueueDesc,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(*vt.add(8));
    f(dev, desc, &IID_ID3D12CommandQueue, out)
}

unsafe fn d3d12_dev_create_command_allocator(
    dev: *mut c_void,
    list_type: u32,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const Guid, *mut *mut c_void) -> HRESULT =
        std::mem::transmute(*vt.add(9));
    f(dev, list_type, &IID_ID3D12CommandAllocator, out)
}

unsafe fn d3d12_dev_create_graphics_pso(
    dev: *mut c_void,
    desc: *const u8,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const u8,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(*vt.add(10));
    f(dev, desc, &IID_ID3D12PipelineState, out)
}

unsafe fn d3d12_dev_create_pipeline_state(
    dev: *mut c_void,
    desc: &StreamDesc,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = *(dev as *const *const usize);
    let qi: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut c_void) -> HRESULT =
        std::mem::transmute(*vt.add(0));
    let mut dev2: *mut c_void = ptr::null_mut();
    let qi_hr = qi(dev, &IID_ID3D12Device2, &mut dev2);
    if qi_hr < 0 || dev2.is_null() {
        return if qi_hr < 0 { qi_hr } else { E_NOTIMPL };
    }
    let vt2 = *(dev2 as *const *const usize);
    // ID3D12Device2::CreatePipelineState is vtable slot 47.
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const StreamDesc,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(*vt2.add(47));
    let hr = f(dev2, desc, &IID_ID3D12PipelineState, out);
    com_release(dev2);
    hr
}

unsafe fn d3d12_dev_create_compute_pso(
    dev: *mut c_void,
    desc: &ComputePipelineDesc,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const ComputePipelineDesc,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(*vt.add(11));
    f(dev, desc, &IID_ID3D12PipelineState, out)
}

unsafe fn d3d12_dev_create_command_list(
    dev: *mut c_void,
    list_type: u32,
    allocator: *mut c_void,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        u32,
        u32,
        *mut c_void,
        *mut c_void,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(*vt.add(12));
    f(dev, 0, list_type, allocator, ptr::null_mut(), &IID_ID3D12GraphicsCommandList, out)
}

unsafe fn d3d12_dev_check_feature(
    dev: *mut c_void,
    feature: u32,
    data: &mut [u8],
) -> HRESULT {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *mut c_void, u32) -> HRESULT =
        std::mem::transmute(*vt.add(13));
    f(dev, feature, data.as_mut_ptr() as *mut c_void, data.len() as u32)
}

unsafe fn d3d12_dev_create_descriptor_heap(
    dev: *mut c_void,
    desc: &DescriptorHeapDesc,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const DescriptorHeapDesc,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(*vt.add(14));
    f(dev, desc, &IID_ID3D12DescriptorHeap, out)
}

unsafe fn d3d12_dev_get_desc_inc_size(dev: *mut c_void, ty: u32) -> u32 {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32) -> u32 = std::mem::transmute(*vt.add(15));
    f(dev, ty)
}

unsafe fn d3d12_dev_create_root_signature(
    dev: *mut c_void,
    blob: &[u8],
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        u32,
        *const u8,
        usize,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(*vt.add(16));
    f(dev, 0, blob.as_ptr(), blob.len(), &IID_ID3D12RootSignature, out)
}

unsafe fn d3d12_dev_create_cbv(dev: *mut c_void, va: u64, size: u32, handle: usize) {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *const (u64, u32), usize) =
        std::mem::transmute(*vt.add(17));
    let d = (va, size);
    f(dev, &d, handle);
}

unsafe fn d3d12_dev_create_srv(
    dev: *mut c_void,
    res: *mut c_void,
    desc: &[u8; 40],
    handle: usize,
) {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [u8; 40], usize) =
        std::mem::transmute(*vt.add(18));
    f(dev, res, desc, handle);
}

unsafe fn d3d12_dev_create_uav(
    dev: *mut c_void,
    res: *mut c_void,
    desc: &[u8; 40],
    handle: usize,
) {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *const [u8; 40],
        usize,
    ) = std::mem::transmute(*vt.add(19));
    f(dev, res, ptr::null_mut(), desc, handle);
}

unsafe fn d3d12_dev_create_rtv(
    dev: *mut c_void,
    res: *mut c_void,
    desc: &[u32; 6],
    handle: usize,
) {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [u32; 6], usize) =
        std::mem::transmute(*vt.add(20));
    f(dev, res, desc, handle);
}

unsafe fn d3d12_dev_create_dsv(
    dev: *mut c_void,
    res: *mut c_void,
    desc: &[u32; 6],
    handle: usize,
) {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [u32; 6], usize) =
        std::mem::transmute(*vt.add(21));
    f(dev, res, desc, handle);
}

unsafe fn d3d12_dev_create_sampler(dev: *mut c_void, desc: &D3dSamplerDesc, handle: usize) {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *const D3dSamplerDesc, usize) =
        std::mem::transmute(*vt.add(22));
    f(dev, desc, handle);
}

unsafe fn d3d12_dev_copy_descriptors_simple(
    dev: *mut c_void,
    num: u32,
    dst: usize,
    src: usize,
    heap_type: u32,
) {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, usize, usize, u32) =
        std::mem::transmute(*vt.add(24));
    f(dev, num, dst, src, heap_type);
}

unsafe fn d3d12_dev_get_resource_alloc_info(
    dev: *mut c_void,
    desc: &ResourceDesc,
) -> AllocationInfo {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *mut AllocationInfo,
        u32,
        u32,
        *const ResourceDesc,
    ) -> *mut AllocationInfo = std::mem::transmute(*vt.add(25));
    let mut info = AllocationInfo::default();
    f(dev, &mut info, 0, 1, desc);
    info
}

unsafe fn d3d12_dev_create_committed_resource(
    dev: *mut c_void,
    props: &HeapProperties,
    heap_flags: u32,
    desc: &ResourceDesc,
    initial_state: u32,
    clear: *const ClearValue,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const HeapProperties,
        u32,
        *const ResourceDesc,
        u32,
        *const ClearValue,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(*vt.add(27));
    f(dev, props, heap_flags, desc, initial_state, clear, &IID_ID3D12Resource, out)
}

unsafe fn d3d12_dev_create_heap(
    dev: *mut c_void,
    desc: &HeapDesc,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const HeapDesc,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(*vt.add(28));
    f(dev, desc, &IID_ID3D12Heap, out)
}

unsafe fn d3d12_dev_create_placed_resource(
    dev: *mut c_void,
    heap: *mut c_void,
    offset: u64,
    desc: &ResourceDesc,
    initial_state: u32,
    clear: *const ClearValue,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *mut c_void,
        u64,
        *const ResourceDesc,
        u32,
        *const ClearValue,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(*vt.add(29));
    f(dev, heap, offset, desc, initial_state, clear, &IID_ID3D12Resource, out)
}

unsafe fn d3d12_dev_create_fence(
    dev: *mut c_void,
    initial: u64,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        u64,
        u32,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(*vt.add(36));
    f(dev, initial, 0, &IID_ID3D12Fence, out)
}

unsafe fn d3d12_dev_create_query_heap(
    dev: *mut c_void,
    desc: &QueryHeapDesc,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const QueryHeapDesc,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(*vt.add(39));
    f(dev, desc, &IID_ID3D12QueryHeap, out)
}

unsafe fn d3d12_dev_create_command_signature(
    dev: *mut c_void,
    desc: &CommandSignatureDesc,
    out: &mut *mut c_void,
) -> HRESULT {
    let vt = *(dev as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const CommandSignatureDesc,
        *mut c_void,
        *const Guid,
        *mut *mut c_void,
    ) -> HRESULT = std::mem::transmute(*vt.add(41));
    f(dev, desc, ptr::null_mut(), &IID_ID3D12CommandSignature, out)
}

unsafe fn d3d12_queue_execute(queue: *mut c_void, lists: &[*mut c_void]) {
    let vt = *(queue as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void) =
        std::mem::transmute(*vt.add(10));
    f(queue, lists.len() as u32, lists.as_ptr());
}

unsafe fn d3d12_queue_signal(queue: *mut c_void, fence: *mut c_void, value: u64) -> HRESULT {
    let vt = *(queue as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, u64) -> HRESULT =
        std::mem::transmute(*vt.add(14));
    f(queue, fence, value)
}

unsafe fn d3d12_queue_wait(queue: *mut c_void, fence: *mut c_void, value: u64) -> HRESULT {
    let vt = *(queue as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, u64) -> HRESULT =
        std::mem::transmute(*vt.add(15));
    f(queue, fence, value)
}

unsafe fn d3d12_queue_get_timestamp_freq(queue: *mut c_void) -> u64 {
    let vt = *(queue as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut u64) -> HRESULT =
        std::mem::transmute(*vt.add(16));
    let mut freq = 0u64;
    if f(queue, &mut freq) >= 0 && freq != 0 {
        freq
    } else {
        d3d12_metal::metal::query::timestamp_frequency()
    }
}

unsafe fn d3d12_queue_get_desc(queue: *mut c_void) -> CommandQueueDesc {
    let vt = *(queue as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut CommandQueueDesc) -> *mut CommandQueueDesc =
        std::mem::transmute(*vt.add(18));
    let mut d = CommandQueueDesc::default();
    f(queue, &mut d);
    d
}

unsafe fn d3d12_alloc_reset(alloc: *mut c_void) -> HRESULT {
    let vt = *(alloc as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) -> HRESULT = std::mem::transmute(*vt.add(8));
    f(alloc)
}

unsafe fn d3d12_fence_completed(fence: *mut c_void) -> u64 {
    let vt = *(fence as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) -> u64 = std::mem::transmute(*vt.add(8));
    f(fence)
}

unsafe fn d3d12_fence_set_event(fence: *mut c_void, value: u64, event: HANDLE) -> HRESULT {
    let vt = *(fence as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u64, HANDLE) -> HRESULT =
        std::mem::transmute(*vt.add(9));
    f(fence, value, event)
}

unsafe fn d3d12_res_map(res: *mut c_void, begin: usize, end: usize) -> Result<*mut u8, HRESULT> {
    let vt = *(res as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const c_void, *mut *mut c_void) -> HRESULT =
        std::mem::transmute(*vt.add(8));
    let range = [begin, end];
    let mut data: *mut c_void = ptr::null_mut();
    let hr = f(res, 0, range.as_ptr() as *const c_void, &mut data);
    if hr >= 0 && !data.is_null() {
        Ok(data as *mut u8)
    } else {
        Err(hr)
    }
}

unsafe fn d3d12_res_unmap(res: *mut c_void) {
    let vt = *(res as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const c_void) =
        std::mem::transmute(*vt.add(9));
    f(res, 0, ptr::null());
}

unsafe fn d3d12_res_get_desc(res: *mut c_void) -> ResourceDesc {
    let vt = *(res as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut ResourceDesc) -> *mut ResourceDesc =
        std::mem::transmute(*vt.add(10));
    let mut d = ResourceDesc::default();
    f(res, &mut d);
    d
}

unsafe fn d3d12_res_get_gpu_va(res: *mut c_void) -> u64 {
    let vt = *(res as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) -> u64 = std::mem::transmute(*vt.add(11));
    f(res)
}

unsafe fn d3d12_heap_get_desc(heap: *mut c_void) -> HeapDesc {
    let vt = *(heap as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut HeapDesc) -> *mut HeapDesc =
        std::mem::transmute(*vt.add(8));
    let mut d = HeapDesc::default();
    f(heap, &mut d);
    d
}

unsafe fn d3d12_descheap_cpu_start(heap: *mut c_void) -> usize {
    let vt = *(heap as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut usize) -> *mut usize =
        std::mem::transmute(*vt.add(9));
    let mut h = 0usize;
    f(heap, &mut h);
    h
}

unsafe fn d3d12_descheap_gpu_start(heap: *mut c_void) -> u64 {
    let vt = *(heap as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut u64) -> *mut u64 =
        std::mem::transmute(*vt.add(10));
    let mut h = 0u64;
    f(heap, &mut h);
    h
}

unsafe fn d3d12_cmd_get_type(list: *mut c_void) -> u32 {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) -> u32 = std::mem::transmute(*vt.add(8));
    f(list)
}

unsafe fn d3d12_cmd_close(list: *mut c_void) -> HRESULT {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) -> HRESULT = std::mem::transmute(*vt.add(9));
    f(list)
}

unsafe fn d3d12_cmd_reset(list: *mut c_void, alloc: *mut c_void) -> HRESULT {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) -> HRESULT =
        std::mem::transmute(*vt.add(10));
    f(list, alloc, ptr::null_mut())
}

unsafe fn d3d12_cmd_draw(list: *mut c_void, v: u32, inst: u32, bv: u32, bi: u32) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, u32, u32, u32) =
        std::mem::transmute(*vt.add(12));
    f(list, v, inst, bv, bi);
}

unsafe fn d3d12_cmd_draw_indexed(list: *mut c_void, idx: u32, inst: u32, bi: u32, bv: i32, binst: u32) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, u32, u32, i32, u32) =
        std::mem::transmute(*vt.add(13));
    f(list, idx, inst, bi, bv, binst);
}

unsafe fn d3d12_cmd_dispatch(list: *mut c_void, x: u32, y: u32, z: u32) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, u32, u32) = std::mem::transmute(*vt.add(14));
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
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, u64, *mut c_void, u64, u64) =
        std::mem::transmute(*vt.add(15));
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
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void,
        *const CopyLocation,
        u32,
        u32,
        u32,
        *const CopyLocation,
        *const CopyBox,
    ) = std::mem::transmute(*vt.add(16));
    f(list, dst, x, y, z, src, bx);
}

unsafe fn d3d12_cmd_copy_resource(list: *mut c_void, dst: *mut c_void, src: *mut c_void) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void) =
        std::mem::transmute(*vt.add(17));
    f(list, dst, src);
}

unsafe fn d3d12_cmd_ia_set_topology(list: *mut c_void, topo: u32) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32) = std::mem::transmute(*vt.add(20));
    f(list, topo);
}

unsafe fn d3d12_cmd_rs_set_viewports(list: *mut c_void, vps: &[Viewport]) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const Viewport) =
        std::mem::transmute(*vt.add(21));
    f(list, vps.len() as u32, vps.as_ptr());
}

unsafe fn d3d12_cmd_rs_set_scissors(list: *mut c_void, rects: &[Rect]) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const Rect) =
        std::mem::transmute(*vt.add(22));
    f(list, rects.len() as u32, rects.as_ptr());
}

unsafe fn d3d12_cmd_om_set_blend_factor(list: *mut c_void, factor: &[f32; 4]) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *const [f32; 4]) =
        std::mem::transmute(*vt.add(23));
    f(list, factor);
}

unsafe fn d3d12_cmd_om_set_stencil_ref(list: *mut c_void, reference: u32) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32) = std::mem::transmute(*vt.add(24));
    f(list, reference);
}

unsafe fn d3d12_cmd_set_pso(list: *mut c_void, pso: *mut c_void) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void) = std::mem::transmute(*vt.add(25));
    f(list, pso);
}

unsafe fn d3d12_cmd_resource_barrier(list: *mut c_void, barriers: &[ResourceBarrier]) {
    if barriers.is_empty() {
        return;
    }
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const ResourceBarrier) =
        std::mem::transmute(*vt.add(26));
    f(list, barriers.len() as u32, barriers.as_ptr());
}

unsafe fn d3d12_cmd_set_descriptor_heaps(list: *mut c_void, heaps: &[*mut c_void]) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const *mut c_void) =
        std::mem::transmute(*vt.add(28));
    f(list, heaps.len() as u32, heaps.as_ptr());
}

unsafe fn d3d12_cmd_set_root_sig(list: *mut c_void, rs: *mut c_void, graphics: bool) {
    let vt = *(list as *const *const usize);
    let slot = if graphics { 30 } else { 29 };
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void) = std::mem::transmute(*vt.add(slot));
    f(list, rs);
}

unsafe fn d3d12_cmd_set_root_table(list: *mut c_void, param: u32, gpu: u64, graphics: bool) {
    let vt = *(list as *const *const usize);
    let slot = if graphics { 32 } else { 31 };
    let f: unsafe extern "win64" fn(*mut c_void, u32, u64) = std::mem::transmute(*vt.add(slot));
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
    let vt = *(list as *const *const usize);
    let slot = if graphics { 36 } else { 35 };
    let f: unsafe extern "win64" fn(*mut c_void, u32, u32, *const u32, u32) =
        std::mem::transmute(*vt.add(slot));
    f(list, param, count, data, dest_offset);
}

unsafe fn d3d12_cmd_set_root_cbv(list: *mut c_void, param: u32, gpu_va: u64, graphics: bool) {
    let vt = *(list as *const *const usize);
    let slot = if graphics { 38 } else { 37 };
    let f: unsafe extern "win64" fn(*mut c_void, u32, u64) = std::mem::transmute(*vt.add(slot));
    f(list, param, gpu_va);
}

unsafe fn d3d12_cmd_ia_set_index_buffer(list: *mut c_void, view: &IndexBufferView) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *const IndexBufferView) =
        std::mem::transmute(*vt.add(43));
    f(list, view);
}

unsafe fn d3d12_cmd_ia_set_vertex_buffers(list: *mut c_void, start: u32, views: &[VertexBufferView]) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, u32, *const VertexBufferView) =
        std::mem::transmute(*vt.add(44));
    f(list, start, views.len() as u32, views.as_ptr());
}

unsafe fn d3d12_cmd_om_set_render_targets(list: *mut c_void, rtvs: &[usize], dsv: usize) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const usize, BOOL, *const usize) =
        std::mem::transmute(*vt.add(46));
    let dsv_ptr = if dsv != 0 { &dsv as *const usize } else { ptr::null() };
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
    let vt = *(list as *const *const usize);
    let r_ptr = if rects.is_empty() { ptr::null() } else { rects.as_ptr() };
    #[cfg(target_arch = "aarch64")]
    {
        let f: unsafe extern "C" fn(
            u64, u64, u64, u64, u64, u64, u64, u64,
            u64, u64, u64, u64, u64, u64, u64, u64,
            f64, f64, f64, f64,
        ) = std::mem::transmute(*vt.add(47));
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
        ) = std::mem::transmute(*vt.add(47));
        f(list, dsv, flags, depth, stencil, rects.len() as u32, r_ptr);
    }
}

unsafe fn d3d12_cmd_clear_rtv(list: *mut c_void, rtv: usize, color: &[f32; 4], rects: &[Rect]) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, usize, *const [f32; 4], u32, *const Rect) =
        std::mem::transmute(*vt.add(48));
    let r_ptr = if rects.is_empty() { ptr::null() } else { rects.as_ptr() };
    f(list, rtv, color, rects.len() as u32, r_ptr);
}

unsafe fn d3d12_cmd_clear_uav_uint(
    list: *mut c_void,
    gpu: u64,
    cpu: usize,
    res: *mut c_void,
    vals: &[u32; 4],
) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void, u64, usize, *mut c_void, *const [u32; 4], u32, *const Rect,
    ) = std::mem::transmute(*vt.add(49));
    f(list, gpu, cpu, res, vals, 0, ptr::null());
}

unsafe fn d3d12_cmd_clear_uav_float(
    list: *mut c_void,
    gpu: u64,
    cpu: usize,
    res: *mut c_void,
    vals: &[f32; 4],
) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(
        *mut c_void, u64, usize, *mut c_void, *const [f32; 4], u32, *const Rect,
    ) = std::mem::transmute(*vt.add(50));
    f(list, gpu, cpu, res, vals, 0, ptr::null());
}

unsafe fn d3d12_cmd_begin_query(list: *mut c_void, heap: *mut c_void, ty: u32, idx: u32) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32) =
        std::mem::transmute(*vt.add(52));
    f(list, heap, ty, idx);
}

unsafe fn d3d12_cmd_end_query(list: *mut c_void, heap: *mut c_void, ty: u32, idx: u32) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32) =
        std::mem::transmute(*vt.add(53));
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
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, u32, u32, *mut c_void, u64) =
        std::mem::transmute(*vt.add(54));
    f(list, heap, ty, start, num, dst, off);
}

unsafe fn d3d12_cmd_begin_event(list: *mut c_void, name: *const u8) {
    if name.is_null() {
        return;
    }
    let bytes = CStr::from_ptr(name as *const std::ffi::c_char).to_bytes_with_nul();
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, u32, *const c_void, u32) =
        std::mem::transmute(*vt.add(57));
    f(list, 1, bytes.as_ptr() as *const c_void, bytes.len() as u32);
}

unsafe fn d3d12_cmd_end_event(list: *mut c_void) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void) = std::mem::transmute(*vt.add(58));
    f(list);
}

unsafe fn d3d12_cmd_execute_indirect(
    list: *mut c_void,
    sig: *mut c_void,
    max_count: u32,
    args: *mut c_void,
    args_off: u64,
) {
    let vt = *(list as *const *const usize);
    let f: unsafe extern "win64" fn(*mut c_void, *mut c_void, u32, *mut c_void, u64, *mut c_void, u64) =
        std::mem::transmute(*vt.add(59));
    f(list, sig, max_count, args, args_off, ptr::null_mut(), 0);
}

/// Calls `ID3D12GraphicsCommandList1::OMSetDepthBounds` (slot 62), which uses
/// `win64_bridge!` on aarch64 for its `f32` arguments.
unsafe fn d3d12_cmd_set_depth_bounds(list: *mut c_void, min: f32, max: f32) {
    let vt = *(list as *const *const usize);
    #[cfg(target_arch = "aarch64")]
    {
        let f: unsafe extern "C" fn(
            u64, u64, u64, u64, u64, u64, u64, u64,
            u64, u64, u64, u64, u64, u64, u64, u64,
            f64, f64, f64, f64,
        ) = std::mem::transmute(*vt.add(62));
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
        let f: unsafe extern "win64" fn(*mut c_void, f32, f32) = std::mem::transmute(*vt.add(62));
        f(list, min, max);
    }
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

fn nri_to_dxgi(fmt: u8) -> u32 {
    NRI_TO_DXGI.get(fmt as usize).copied().unwrap_or(0)
}

fn nri_to_dxgi_typeless(fmt: u8) -> u32 {
    NRI_TO_DXGI_TYPELESS.get(fmt as usize).copied().unwrap_or(0)
}

fn nri_is_fp_format(fmt: u8) -> bool {
    NRI_FP_FORMAT.get(fmt as usize).copied().unwrap_or(false)
}

fn nri_texel_block_width(fmt: u8) -> u16 {
    NRI_TEXEL_BLOCK_WIDTH.get(fmt as usize).copied().unwrap_or(1).max(1)
}

fn nri_texel_block_size(fmt: u8) -> u32 {
    NRI_TEXEL_BLOCK_SIZE.get(fmt as usize).copied().unwrap_or(1).max(1)
}

fn dxgi_to_nri(dxgi: u32) -> u8 {
    for (i, &d) in NRI_TO_DXGI.iter().enumerate() {
        if d == dxgi && i != 0 {
            return i as u8;
        }
    }
    0
}

fn shader_format_for_depth(dxgi: u32) -> u32 {
    match dxgi {
        55 => 56, // D16_UNORM -> R16_UNORM
        45 => 46, // D24_UNORM_S8_UINT -> R24_UNORM_X8_TYPELESS
        40 => 41, // D32_FLOAT -> R32_FLOAT
        20 => 21, // D32_FLOAT_S8X24_UINT -> R32_FLOAT_X8X24_TYPELESS
        other => other,
    }
}

fn nri_queue_type_to_d3d12(ty: u8) -> Option<u32> {
    match ty {
        0 => Some(0), // GRAPHICS -> DIRECT
        1 => Some(2), // COMPUTE -> COMPUTE
        2 => Some(3), // COPY -> COPY
        _ => None,
    }
}

fn nri_memory_type(location: u8, desc: &ResourceDesc) -> u32 {
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

fn memory_type_heap_type(mem_type: u32) -> u32 {
    let ht = mem_type >> 16;
    if matches!(ht, heap_type::DEFAULT | heap_type::UPLOAD | heap_type::READBACK) {
        ht
    } else {
        heap_type::DEFAULT
    }
}

fn memory_type_heap_flags(mem_type: u32) -> u32 {
    mem_type & 0xffff
}

fn memory_type_requires_dedicated(mem_type: u32) -> bool {
    (memory_type_heap_flags(mem_type) & D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES)
        == D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES
}

fn nri_access_to_d3d12_state(access: u16, list_type: u32) -> u32 {
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

fn nri_descriptor_range_type(dt: u8) -> Option<DescriptorRangeType> {
    match dt {
        0 => Some(DescriptorRangeType::Sampler),
        1 => Some(DescriptorRangeType::Cbv),
        2 | 4 | 6 | 8 => Some(DescriptorRangeType::Srv),
        3 | 5 | 7 => Some(DescriptorRangeType::Uav),
        _ => None,
    }
}

fn nri_descriptor_heap_type(dt: u8) -> u32 {
    if dt == 0 { 1 } else { 0 } // 1 = SAMPLER, 0 = CBV_SRV_UAV
}

fn nri_shader_visibility(stage: u8) -> u32 {
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

fn nri_compare_func(c: u8) -> u32 {
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

fn nri_address_mode(m: u8) -> u32 {
    match m {
        0 => 1, // REPEAT -> WRAP
        1 => 2, // MIRRORED_REPEAT -> MIRROR
        2 => 3, // CLAMP_TO_EDGE -> CLAMP
        3 => 4, // CLAMP_TO_BORDER -> BORDER
        _ => 1,
    }
}

fn nri_stencil_op(op: u8) -> u32 {
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

fn nri_blend_factor(f: u8) -> u32 {
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

fn nri_blend_op(op: u8) -> u32 {
    match op {
        0 => 1, // ADD
        1 => 2, // SUBTRACT
        2 => 3, // REV_SUBTRACT
        3 => 4, // MIN
        4 => 5, // MAX
        _ => 1,
    }
}

fn nri_logic_op(op: u8) -> u32 {
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

fn nri_topology_type(t: u8) -> u32 {
    match t {
        0 => 1,             // POINT
        1 | 2 | 5 | 6 => 2, // LINE
        3 | 4 | 7 | 8 => 3, // TRIANGLE
        9 => 4,             // PATCH
        _ => 3,
    }
}

fn nri_primitive_topology(t: u8, patch_pts: u8) -> u32 {
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

fn nri_sampler_filter(mip: u8, mag: u8, min: u8, filter_ext: u8, aniso: bool, cmp: bool) -> u32 {
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
struct CpuDescriptorAllocator {
    chunks: Vec<CpuHeapChunk>,
    free: Vec<(usize, u32)>,
}

pub struct NriDevice {
    d3d12_device: *mut c_void,
    desc: NriDeviceDesc,
    queues: Mutex<[Option<*mut NriCommandQueue>; 3]>,
    cpu_heaps: Mutex<[CpuDescriptorAllocator; 4]>,
    dispatch_cmd_sig: Mutex<*mut c_void>,
    draw_cmd_sigs: Mutex<HashMap<u32, *mut c_void>>,
    draw_indexed_cmd_sigs: Mutex<HashMap<u32, *mut c_void>>,
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
    unsafe fn alloc_cpu_descriptor(&self, heap_type: u32) -> Result<(usize, u32, usize), u32> {
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
            pool.chunks.push(CpuHeapChunk { heap, cpu_start, inc_size });
            for slot in (0..CPU_DESCRIPTOR_BATCH_SIZE).rev() {
                pool.free.push((chunk_idx, slot));
            }
        }
        let (chunk_idx, slot_idx) = pool.free.pop().unwrap();
        let chunk = &pool.chunks[chunk_idx];
        let handle = chunk.cpu_start + (slot_idx as usize) * (chunk.inc_size as usize);
        Ok((chunk_idx, slot_idx, handle))
    }

    fn free_cpu_descriptor(&self, heap_type: u32, chunk_idx: usize, slot_idx: u32) {
        if let Ok(mut guard) = self.cpu_heaps.lock() {
            if let Some(pool) = guard.get_mut(heap_type as usize) {
                if chunk_idx < pool.chunks.len() {
                    pool.free.push((chunk_idx, slot_idx));
                }
            }
        }
    }

    unsafe fn get_or_create_queue(
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
        let desc = CommandQueueDesc { Type: d3d_type, Priority: 0, Flags: 0, NodeMask: 0 };
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

    unsafe fn get_indirect_signature(&self, arg_type: u32, stride: u32) -> *mut c_void {
        let create = || -> *mut c_void {
            let arg = IndirectArgument { ty: arg_type, u: [0; 3] };
            let desc = CommandSignatureDesc {
                byte_stride: stride,
                num_args: 1,
                args: &arg,
                node_mask: 0,
            };
            let mut sig: *mut c_void = ptr::null_mut();
            let hr = unsafe { d3d12_dev_create_command_signature(self.d3d12_device, &desc, &mut sig) };
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
    device: *mut NriDevice,
    d3d12_queue: *mut c_void,
    list_type: u32,
}

impl Drop for NriCommandQueue {
    fn drop(&mut self) {
        unsafe { com_release(self.d3d12_queue) };
    }
}

pub struct NriCommandAllocator {
    device: *mut NriDevice,
    d3d12_allocator: *mut c_void,
    list_type: u32,
}

impl Drop for NriCommandAllocator {
    fn drop(&mut self) {
        unsafe { com_release(self.d3d12_allocator) };
    }
}

pub struct NriCommandBuffer {
    device: *mut NriDevice,
    d3d12_list: *mut c_void,
    d3d12_allocator: *mut c_void,
    list_type: u32,
    pipeline_layout: *const NriPipelineLayout,
    is_graphics_layout: bool,
    pipeline: *const NriPipeline,
    frame_buffer: *const NriFrameBuffer,
    descriptor_sets: Vec<*const NriDescriptorSet>,
}

impl Drop for NriCommandBuffer {
    fn drop(&mut self) {
        unsafe {
            com_release(self.d3d12_list);
            com_release(self.d3d12_allocator);
        }
    }
}

pub struct NriMemory {
    d3d12_heap: *mut c_void,
    heap_desc: HeapDesc,
    dedicated: bool,
}

impl Drop for NriMemory {
    fn drop(&mut self) {
        unsafe { com_release(self.d3d12_heap) };
    }
}

pub struct NriBuffer {
    device: *mut NriDevice,
    d3d12_resource: *mut c_void,
    desc: ResourceDesc,
    structure_stride: u32,
}

impl Drop for NriBuffer {
    fn drop(&mut self) {
        unsafe { com_release(self.d3d12_resource) };
    }
}

pub struct NriTexture {
    device: *mut NriDevice,
    d3d12_resource: *mut c_void,
    desc: ResourceDesc,
    nri_format: u8,
}

impl NriTexture {
    fn subresource_index(&self, array_offset: u16, mip_offset: u16) -> u32 {
        mip_offset as u32 + (array_offset as u32) * (self.desc.mip_levels())
    }

    fn dim_size(&self, axis: usize, mip_offset: u16) -> u16 {
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

pub struct NriDescriptor {
    device: *mut NriDevice,
    heap_type: u32,
    chunk_idx: usize,
    slot_idx: u32,
    cpu_handle: usize,
    d3d12_resource: *mut c_void,
    buffer_gpu_va: u64,
    is_float_uav: bool,
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
    heap_ptrs: [*mut c_void; 2],
    heap_count: usize,
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
struct DescriptorRangeMapping {
    heap_type: u32,
    heap_offset: u32,
    descriptor_num: u32,
}

#[derive(Clone, Default)]
struct DescriptorSetMapping {
    ranges: Vec<DescriptorRangeMapping>,
    counts: [u32; 2],
}

#[derive(Clone, Copy, Default)]
struct DynamicCbvMapping {
    count: u16,
    root_offset: u16,
}

pub struct NriDescriptorSet {
    device: *mut NriDevice,
    pool: *mut NriDescriptorPool,
    ranges: Vec<DescriptorRangeMapping>,
    dynamic_cbv_vas: Vec<u64>,
}

impl NriDescriptorSet {
    unsafe fn cpu_ptr(&self, range_idx: usize, offset: u32) -> usize {
        if let Some(rm) = self.ranges.get(range_idx) {
            (*self.pool).cpu_ptr(rm.heap_type, rm.heap_offset + offset)
        } else {
            0
        }
    }

    unsafe fn gpu_ptr(&self, range_idx: usize, offset: u32) -> u64 {
        if let Some(rm) = self.ranges.get(range_idx) {
            (*self.pool).gpu_ptr(rm.heap_type, rm.heap_offset + offset)
        } else {
            0
        }
    }
}

pub struct NriPipelineLayout {
    d3d12_root_sig: *mut c_void,
    is_graphics: bool,
    set_mappings: Vec<DescriptorSetMapping>,
    set_root_offsets: Vec<Vec<u16>>,
    dynamic_cbv_mappings: Vec<DynamicCbvMapping>,
    push_constants_base_index: u32,
}

impl Drop for NriPipelineLayout {
    fn drop(&mut self) {
        unsafe { com_release(self.d3d12_root_sig) };
    }
}

pub struct NriPipeline {
    d3d12_pso: *mut c_void,
    is_graphics: bool,
    primitive_topology: u32,
    stream_strides: Vec<u32>,
    blend_enabled: bool,
    blend_factor: [f32; 4],
}

impl Drop for NriPipeline {
    fn drop(&mut self) {
        unsafe { com_release(self.d3d12_pso) };
    }
}

pub struct NriFrameBuffer {
    rtvs: Vec<usize>,
    dsv: usize,
    clears: Vec<NriClearDesc>,
}

pub struct NriQueryPool {
    d3d12_query_heap: *mut c_void,
    d3d12_query_type: u32,
    query_size: u32,
}

impl Drop for NriQueryPool {
    fn drop(&mut self) {
        unsafe { com_release(self.d3d12_query_heap) };
    }
}

pub struct NriFence {
    d3d12_fence: *mut c_void,
}

impl Drop for NriFence {
    fn drop(&mut self) {
        unsafe { com_release(self.d3d12_fence) };
    }
}

pub struct NriAccelerationStructure {
    d3d12_resource: *mut c_void,
    _scratch_size: u64,
    _update_scratch_size: u64,
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
    let logic_op = u32::from_le_bytes(opts0[4..8].try_into().unwrap()) != 0;
    let conservative_raster_tier = u32::from_le_bytes(opts0[32..36].try_into().unwrap()) as u8;

    let mut opts2 = [0u8; 8];
    let _ = d3d12_dev_check_feature(d3d12_device, 18, &mut opts2);
    let depth_bounds_supported = u32::from_le_bytes(opts2[0..4].try_into().unwrap()) != 0;
    let prog_sample_pos = u32::from_le_bytes(opts2[4..8].try_into().unwrap()) != 0;

    let mut opts3 = [0u8; 20];
    let _ = d3d12_dev_check_feature(d3d12_device, 21, &mut opts3);
    let copy_queue_ts = u32::from_le_bytes(opts3[0..4].try_into().unwrap()) != 0;

    let mut opts4 = [0u8; 12];
    let _ = d3d12_dev_check_feature(d3d12_device, 23, &mut opts4);
    let float16 = u32::from_le_bytes(opts4[8..12].try_into().unwrap()) != 0;

    let mut opts5 = [0u8; 12];
    let _ = d3d12_dev_check_feature(d3d12_device, 27, &mut opts5);
    let rt_tier = u32::from_le_bytes(opts5[8..12].try_into().unwrap());
    let rt_supported = rt_tier >= 10;

    let mut opts7 = [0u8; 8];
    let _ = d3d12_dev_check_feature(d3d12_device, 32, &mut opts7);
    let mesh_supported = u32::from_le_bytes(opts7[0..4].try_into().unwrap()) >= 10;

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
        mesh_task_work_group_max_dim: if mesh_supported { [128, 128, 128] } else { [0; 3] },
        mesh_task_total_memory_max_size: if mesh_supported { 32768 } else { 0 },
        mesh_task_output_max_num: if mesh_supported { 65536 } else { 0 },
        mesh_work_group_invocation_max_num: if mesh_supported { 128 } else { 0 },
        mesh_work_group_max_dim: if mesh_supported { [128, 128, 128] } else { [0; 3] },
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

#[repr(C, align(16))]
struct InterfaceTable<const N: usize>([crate::Ptr; N]);
unsafe impl<const N: usize> Sync for InterfaceTable<N> {}

// ---------------------------------------------------------------------------
// CoreInterface — Get (slots 0..6)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn core_get_device_desc(device: *const NriDevice) -> *const NriDeviceDesc {
    if device.is_null() {
        return ptr::null();
    }
    &(*device).desc
}

unsafe extern "win64" fn core_get_format_support(_device: *const NriDevice, format: u8) -> u8 {
    let dxgi = nri_to_dxgi(format);
    if dxgi == 0 {
        return 0;
    }
    let (s1, s2) = d3d_fmt::support(dxgi);
    let mut bits = 0u8;
    if s1 & (support1::TEXTURE1D | support1::TEXTURE2D | support1::TEXTURE3D | support1::TEXTURECUBE) != 0 {
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

unsafe extern "win64" fn core_get_query_size(query_pool: *const NriQueryPool) -> u32 {
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
    ptr::write_unaligned(
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

unsafe extern "win64" fn core_get_buffer_memory_info(
    buffer: *const NriBuffer,
    memory_location: u8,
    memory_desc: *mut NriMemoryDesc,
) {
    if buffer.is_null() {
        return;
    }
    fill_memory_info((*buffer).device, &(*buffer).desc, memory_location, memory_desc);
}

unsafe extern "win64" fn core_get_texture_memory_info(
    texture: *const NriTexture,
    memory_location: u8,
    memory_desc: *mut NriMemoryDesc,
) {
    if texture.is_null() {
        return;
    }
    fill_memory_info((*texture).device, &(*texture).desc, memory_location, memory_desc);
}

unsafe extern "win64" fn core_get_command_queue(
    device: *mut NriDevice,
    queue_type: u8,
    queue_out: *mut *mut NriCommandQueue,
) -> u32 {
    if device.is_null() || queue_out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    match NriDevice::get_or_create_queue(device, queue_type) {
        Ok(q) => {
            ptr::write_unaligned(queue_out, q);
            NRI_RESULT_SUCCESS
        }
        Err(e) => e,
    }
}

// ---------------------------------------------------------------------------
// CoreInterface — Create (slots 6..22)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn core_create_command_allocator(
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
    ptr::write_unaligned(
        out,
        Box::into_raw(Box::new(NriCommandAllocator {
            device: dev,
            d3d12_allocator: alloc,
            list_type,
        })),
    );
    NRI_RESULT_SUCCESS
}

unsafe extern "win64" fn core_create_command_buffer(
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
    ptr::write_unaligned(
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

unsafe extern "win64" fn core_create_descriptor_pool(
    device: *mut NriDevice,
    desc_ptr: *const NriDescriptorPoolDesc,
    out: *mut *mut NriDescriptorPool,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let d = ptr::read_unaligned(desc_ptr);
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

    ptr::write_unaligned(
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

unsafe extern "win64" fn core_create_buffer(
    device: *mut NriDevice,
    desc_ptr: *const NriBufferDesc,
    out: *mut *mut NriBuffer,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let bd = ptr::read_unaligned(desc_ptr);
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
    ptr::write_unaligned(
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

unsafe extern "win64" fn core_create_texture(
    device: *mut NriDevice,
    desc_ptr: *const NriTextureDesc,
    out: *mut *mut NriTexture,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let td = ptr::read_unaligned(desc_ptr);
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
    ptr::write_unaligned(
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

#[inline]
fn put_u32(buf: &mut [u8], off: usize, v: u32) {
    buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

#[inline]
fn put_u64(buf: &mut [u8], off: usize, v: u64) {
    buf[off..off + 8].copy_from_slice(&v.to_le_bytes());
}

unsafe extern "win64" fn core_create_buffer_view(
    desc_ptr: *const NriBufferViewDesc,
    out: *mut *mut NriDescriptor,
) -> u32 {
    if desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let vd = ptr::read_unaligned(desc_ptr);
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
    ptr::write_unaligned(
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
    let vd = ptr::read_unaligned(desc_ptr);
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
        (1, 4) => ViewOp::Rtv([dxgi, 3, vd.mip_offset as u32, vd.array_or_slice_offset as u32, rem_arr, 0]),
        (1, 5) => {
            let mut f = 0u32;
            if vd.flags & 1 != 0 { f |= 1; }
            if vd.flags & 2 != 0 { f |= 2; }
            ViewOp::Dsv([dxgi, 2, f, vd.mip_offset as u32, vd.array_or_slice_offset as u32, rem_arr])
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
                ViewOp::Rtv([dxgi, 5, vd.mip_offset as u32, vd.array_or_slice_offset as u32, rem_arr, 0])
            }
        }
        (2, 7) => {
            let mut f = 0u32;
            if vd.flags & 1 != 0 { f |= 1; }
            if vd.flags & 2 != 0 { f |= 2; }
            if msaa {
                ViewOp::Dsv([dxgi, 6, f, vd.array_or_slice_offset as u32, rem_arr, 0])
            } else {
                ViewOp::Dsv([dxgi, 4, f, vd.mip_offset as u32, vd.array_or_slice_offset as u32, rem_arr])
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
                tex.desc.depth().saturating_sub(vd.array_or_slice_offset as u32).max(1)
            } else {
                vd.array_or_slice_size as u32
            };
            put_u32(&mut d, 16, wsize);
            ViewOp::Uav(d)
        }
        (3, 2) => {
            let wsize = if vd.array_or_slice_size == 0 {
                tex.desc.depth().saturating_sub(vd.array_or_slice_offset as u32).max(1)
            } else {
                vd.array_or_slice_size as u32
            };
            ViewOp::Rtv([dxgi, 8, vd.mip_offset as u32, vd.array_or_slice_offset as u32, wsize, 0])
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
        ViewOp::Srv(d) => d3d12_dev_create_srv((*dev).d3d12_device, tex.d3d12_resource, &d, cpu_handle),
        ViewOp::Uav(d) => {
            d3d12_dev_create_uav((*dev).d3d12_device, tex.d3d12_resource, &d, cpu_handle);
            is_float_uav = nri_is_fp_format(vd.format);
        }
        ViewOp::Rtv(d) => d3d12_dev_create_rtv((*dev).d3d12_device, tex.d3d12_resource, &d, cpu_handle),
        ViewOp::Dsv(d) => d3d12_dev_create_dsv((*dev).d3d12_device, tex.d3d12_resource, &d, cpu_handle),
    }

    com_add_ref(tex.d3d12_resource);
    ptr::write_unaligned(
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

unsafe extern "win64" fn core_create_texture_1d_view(
    desc: *const NriTextureViewDesc,
    out: *mut *mut NriDescriptor,
) -> u32 {
    create_texture_view_common(desc, 1, out)
}

unsafe extern "win64" fn core_create_texture_2d_view(
    desc: *const NriTextureViewDesc,
    out: *mut *mut NriDescriptor,
) -> u32 {
    create_texture_view_common(desc, 2, out)
}

unsafe extern "win64" fn core_create_texture_3d_view(
    desc: *const NriTextureViewDesc,
    out: *mut *mut NriDescriptor,
) -> u32 {
    create_texture_view_common(desc, 3, out)
}

unsafe extern "win64" fn core_create_sampler(
    device: *mut NriDevice,
    desc_ptr: *const NriSamplerDesc,
    out: *mut *mut NriDescriptor,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let sd = ptr::read_unaligned(desc_ptr);
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
    ptr::write_unaligned(
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

unsafe extern "win64" fn core_create_pipeline_layout(
    device: *mut NriDevice,
    desc_ptr: *const NriPipelineLayoutDesc,
    out: *mut *mut NriPipelineLayout,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let pd = ptr::read_unaligned(desc_ptr);
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
        let ds = ptr::read_unaligned(pd.descriptor_sets.add(i));
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
            let r = ptr::read_unaligned(ds.ranges.add(j));
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
            mapping.counts[ht as usize] = mapping.counts[ht as usize].saturating_add(r.descriptor_num);

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
                if rt == DescriptorRangeType::Sampler { 0x1 } else { 0x3 }
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
                let dcb = ptr::read_unaligned(ds.dynamic_constant_buffers.add(j));
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
        let pc = ptr::read_unaligned(pd.push_constants.add(i));
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

    ptr::write_unaligned(
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

/// Pointer-aligned builder for `D3D12_PIPELINE_STATE_STREAM_DESC` subobjects,
/// matching MSVC `CD3DX12_PIPELINE_STATE_STREAM` and `crate::d3d12::pso::parse_stream`.
struct PsoStreamBuf {
    words: Vec<u64>,
}

impl PsoStreamBuf {
    fn new() -> Self {
        Self { words: Vec::with_capacity(96) }
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

unsafe extern "win64" fn core_create_graphics_pipeline(
    device: *mut NriDevice,
    desc_ptr: *const NriGraphicsPipelineDesc,
    out: *mut *mut NriPipeline,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let gd = ptr::read_unaligned(desc_ptr);
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
        let sh = ptr::read_unaligned(gd.shader_stages.add(i));
        let bc = Bytecode { ptr: sh.bytecode, len: sh.size as usize };
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
        let r = ptr::read_unaligned(gd.rasterization);
        blend.AlphaToCoverageEnable = i32::from(r.alpha_to_coverage != 0);
        sample_mask = if r.sample_mask == 0 { u32::MAX } else { r.sample_mask as u32 };
        sample_count = r.sample_num.max(1) as u32;
        let fill = if r.fill_mode == 1 { 2u32 } else { 3u32 };
        let cull = match r.cull_mode { 1 => 2u32, 2 => 3u32, _ => 1u32 };
        put_u32(&mut raster_raw, 0, fill);
        put_u32(&mut raster_raw, 4, cull);
        put_u32(&mut raster_raw, 8, u32::from(r.front_counter_clockwise != 0));
        put_u32(&mut raster_raw, 12, r.depth_bias_constant_factor as u32);
        put_u32(&mut raster_raw, 16, r.depth_bias_clamp.to_bits());
        put_u32(&mut raster_raw, 20, r.depth_bias_slope_factor.to_bits());
        put_u32(&mut raster_raw, 24, u32::from(r.depth_clamp != 0));
        put_u32(&mut raster_raw, 28, u32::from(sample_count > 1));
        put_u32(&mut raster_raw, 32, u32::from(r.antialiased_lines != 0));
        put_u32(&mut raster_raw, 36, if sample_count > 1 { sample_count } else { 0 });
        put_u32(&mut raster_raw, 40, u32::from(r.conservative_rasterization != 0));
    } else {
        put_u32(&mut raster_raw, 0, 3);  // SOLID
        put_u32(&mut raster_raw, 4, 3);  // BACK
        put_u32(&mut raster_raw, 24, 1); // DepthClipEnable
    }

    let mut num_rt = 0u32;
    let mut rtv_formats = [0u32; 8];
    let mut dsv_format = 0u32;
    let mut ds_raw = [0u8; 52];

    if !gd.output_merger.is_null() {
        let om = ptr::read_unaligned(gd.output_merger);
        let n_rt = (om.color_num as usize).min(8);
        num_rt = n_rt as u32;
        if n_rt > 0 && !om.color.is_null() {
            for i in 0..n_rt {
                let c = ptr::read_unaligned(om.color.add(i));
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
        put_u32(&mut ds_raw, 32, nri_compare_func(om.stencil.front.compare_func));
        put_u32(&mut ds_raw, 36, nri_stencil_op(om.stencil.back.fail));
        put_u32(&mut ds_raw, 40, nri_stencil_op(om.stencil.back.depth_fail));
        put_u32(&mut ds_raw, 44, nri_stencil_op(om.stencil.back.pass));
        put_u32(&mut ds_raw, 48, nri_compare_func(om.stencil.back.compare_func));
    }

    let blend_bytes =
        std::slice::from_raw_parts(&blend as *const BlendDesc as *const u8, size_of::<BlendDesc>());

    let mut input_elems: Vec<InputElementDesc> = Vec::new();
    let mut stream_strides: Vec<u32> = Vec::new();
    let mut prim_topo = 4u32; // TRIANGLELIST
    let mut strip_cut = 0u32;
    let mut topo_type = 3u32; // TRIANGLE
    if !gd.input_assembly.is_null() {
        let ia = ptr::read_unaligned(gd.input_assembly);
        strip_cut = ia.primitive_restart as u32;
        topo_type = nri_topology_type(ia.topology);
        prim_topo = nri_primitive_topology(ia.topology, ia.tess_control_point_num);
        if ia.stream_num > 0 && !ia.streams.is_null() {
            for s in 0..ia.stream_num as usize {
                let st = ptr::read_unaligned(ia.streams.add(s));
                let slot = st.binding_slot as usize;
                if stream_strides.len() <= slot {
                    stream_strides.resize(slot + 1, 0);
                }
                stream_strides[slot] = st.stride;
            }
            if ia.attribute_num > 0 && !ia.attributes.is_null() {
                for a in 0..ia.attribute_num as usize {
                    let attr = ptr::read_unaligned(ia.attributes.add(a));
                    if (attr.stream_index as usize) >= ia.stream_num as usize {
                        return NRI_RESULT_INVALID_ARGUMENT;
                    }
                    let st = ptr::read_unaligned(ia.streams.add(attr.stream_index as usize));
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

    ptr::write_unaligned(
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

unsafe extern "win64" fn core_create_compute_pipeline(
    device: *mut NriDevice,
    desc_ptr: *const NriComputePipelineDesc,
    out: *mut *mut NriPipeline,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let cd = ptr::read_unaligned(desc_ptr);
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
        cached: Bytecode { ptr: ptr::null(), len: 0 },
        flags: 0,
    };
    let mut pso: *mut c_void = ptr::null_mut();
    let hr = d3d12_dev_create_compute_pso((*device).d3d12_device, &d3d_desc, &mut pso);
    if hr < 0 || pso.is_null() {
        return hresult_to_nri(hr);
    }
    ptr::write_unaligned(
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

unsafe extern "win64" fn core_create_frame_buffer(
    device: *mut NriDevice,
    desc_ptr: *const NriFrameBufferDesc,
    out: *mut *mut NriFrameBuffer,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let fd = ptr::read_unaligned(desc_ptr);
    let mut rtvs = Vec::with_capacity(fd.color_attachment_num as usize);
    let mut clears = Vec::new();
    if fd.color_attachment_num > 0 {
        if fd.color_attachments.is_null() {
            return NRI_RESULT_INVALID_ARGUMENT;
        }
        for i in 0..fd.color_attachment_num as usize {
            let desc = ptr::read_unaligned(fd.color_attachments.add(i));
            rtvs.push(if desc.is_null() { 0 } else { (*desc).cpu_handle });
            if !fd.color_clear_values.is_null() {
                clears.push(NriClearDesc {
                    value: ptr::read_unaligned(fd.color_clear_values.add(i)),
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
                value: ptr::read_unaligned(fd.depth_stencil_clear_value),
                attachment_content_type: 3, // DEPTH_STENCIL
                _pad: [0; 3],
                color_attachment_index: 0,
            });
        }
        (*fd.depth_stencil_attachment).cpu_handle
    };
    ptr::write_unaligned(out, Box::into_raw(Box::new(NriFrameBuffer { rtvs, dsv, clears })));
    NRI_RESULT_SUCCESS
}

unsafe extern "win64" fn core_create_query_pool(
    device: *mut NriDevice,
    desc_ptr: *const NriQueryPoolDesc,
    out: *mut *mut NriQueryPool,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let qd = ptr::read_unaligned(desc_ptr);
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
    let d3d_desc = QueryHeapDesc { ty: heap_ty, count: qd.capacity, node_mask: 0 };
    let mut heap: *mut c_void = ptr::null_mut();
    let hr = d3d12_dev_create_query_heap((*device).d3d12_device, &d3d_desc, &mut heap);
    if hr < 0 || heap.is_null() {
        return hresult_to_nri(hr);
    }
    ptr::write_unaligned(
        out,
        Box::into_raw(Box::new(NriQueryPool {
            d3d12_query_heap: heap,
            d3d12_query_type: query_ty,
            query_size: elem_size,
        })),
    );
    NRI_RESULT_SUCCESS
}

unsafe extern "win64" fn core_create_fence(
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
    ptr::write_unaligned(out, Box::into_raw(Box::new(NriFence { d3d12_fence: fence })));
    NRI_RESULT_SUCCESS
}

// ---------------------------------------------------------------------------
// CoreInterface — Destroy (slots 22..33)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn core_destroy_command_allocator(p: *mut NriCommandAllocator) {
    if !p.is_null() { drop(Box::from_raw(p)); }
}
unsafe extern "win64" fn core_destroy_command_buffer(p: *mut NriCommandBuffer) {
    if !p.is_null() { drop(Box::from_raw(p)); }
}
unsafe extern "win64" fn core_destroy_descriptor_pool(p: *mut NriDescriptorPool) {
    if !p.is_null() { drop(Box::from_raw(p)); }
}
unsafe extern "win64" fn core_destroy_buffer(p: *mut NriBuffer) {
    if !p.is_null() { drop(Box::from_raw(p)); }
}
unsafe extern "win64" fn core_destroy_texture(p: *mut NriTexture) {
    if !p.is_null() { drop(Box::from_raw(p)); }
}
unsafe extern "win64" fn core_destroy_descriptor(p: *mut NriDescriptor) {
    if !p.is_null() { drop(Box::from_raw(p)); }
}
unsafe extern "win64" fn core_destroy_pipeline_layout(p: *mut NriPipelineLayout) {
    if !p.is_null() { drop(Box::from_raw(p)); }
}
unsafe extern "win64" fn core_destroy_pipeline(p: *mut NriPipeline) {
    if !p.is_null() { drop(Box::from_raw(p)); }
}
unsafe extern "win64" fn core_destroy_frame_buffer(p: *mut NriFrameBuffer) {
    if !p.is_null() { drop(Box::from_raw(p)); }
}
unsafe extern "win64" fn core_destroy_query_pool(p: *mut NriQueryPool) {
    if !p.is_null() { drop(Box::from_raw(p)); }
}
unsafe extern "win64" fn core_destroy_fence(p: *mut NriFence) {
    if !p.is_null() { drop(Box::from_raw(p)); }
}

// ---------------------------------------------------------------------------
// CoreInterface — Memory (slots 33..37)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn core_allocate_memory(
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
    ptr::write_unaligned(
        out,
        Box::into_raw(Box::new(NriMemory {
            d3d12_heap: heap,
            heap_desc,
            dedicated,
        })),
    );
    NRI_RESULT_SUCCESS
}

unsafe extern "win64" fn core_bind_buffer_memory(
    device: *mut NriDevice,
    descs: *const NriBufferMemoryBindingDesc,
    num: u32,
) -> u32 {
    if device.is_null() || (num > 0 && descs.is_null()) {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    for i in 0..num as usize {
        let b = ptr::read_unaligned(descs.add(i));
        if b.memory.is_null() || b.buffer.is_null() {
            return NRI_RESULT_INVALID_ARGUMENT;
        }
        let mem = &*b.memory;
        let buf = &mut *b.buffer;
        let initial_state = match mem.heap_desc.Properties.Type {
            heap_type::UPLOAD => 0xAC3, // GENERIC_READ
            heap_type::READBACK => 0x400, // COPY_DEST
            _ => 0, // COMMON
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

unsafe extern "win64" fn core_bind_texture_memory(
    device: *mut NriDevice,
    descs: *const NriTextureMemoryBindingDesc,
    num: u32,
) -> u32 {
    if device.is_null() || (num > 0 && descs.is_null()) {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    for i in 0..num as usize {
        let b = ptr::read_unaligned(descs.add(i));
        if b.memory.is_null() || b.texture.is_null() {
            return NRI_RESULT_INVALID_ARGUMENT;
        }
        let mem = &*b.memory;
        let tex = &mut *b.texture;
        let is_rt_ds = (tex.desc.Flags
            & (resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_DEPTH_STENCIL))
            != 0;
        let clear = ClearValue { Format: nri_to_dxgi(tex.nri_format), Color: [0.0; 4] };
        let clear_ptr = if is_rt_ds { &clear as *const ClearValue } else { ptr::null() };
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

unsafe extern "win64" fn core_free_memory(memory: *mut NriMemory) {
    if !memory.is_null() {
        drop(Box::from_raw(memory));
    }
}

// ---------------------------------------------------------------------------
// CoreInterface — Command buffer recording (slots 37..73)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn core_begin_command_buffer(
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

unsafe extern "win64" fn core_end_command_buffer(cmd: *mut NriCommandBuffer) -> u32 {
    if cmd.is_null() || (*cmd).d3d12_list.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    hresult_to_nri(d3d12_cmd_close((*cmd).d3d12_list))
}

unsafe extern "win64" fn core_cmd_set_pipeline(
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

unsafe extern "win64" fn core_cmd_set_pipeline_layout(
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
    d3d12_cmd_set_root_sig(cb.d3d12_list, (*layout).d3d12_root_sig, cb.is_graphics_layout);
}

unsafe extern "win64" fn core_cmd_set_descriptor_set(
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
            d3d12_cmd_set_root_table(cb.d3d12_list, root_param as u32, gpu, cb.is_graphics_layout);
        }
    }
    if let Some(dyn_map) = pl.dynamic_cbv_mappings.get(idx) {
        for j in 0..dyn_map.count as usize {
            let off = if dyn_cb_offsets.is_null() {
                0
            } else {
                ptr::read_unaligned(dyn_cb_offsets.add(j)) as u64
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

unsafe extern "win64" fn core_cmd_set_constants(
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
        tmp[k] = ptr::read_unaligned((data as *const u32).add(k));
    }
    d3d12_cmd_set_root_constants(cb.d3d12_list, param, n as u32, tmp.as_ptr(), 0, cb.is_graphics_layout);
}

unsafe extern "win64" fn core_cmd_set_descriptor_pool(
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
        Some(ResourceBarrier { Type: 2, Flags: 0, u: [resource as u64, 0, 0] })
    } else if before != after {
        Some(ResourceBarrier {
            Type: 0,
            Flags: 0,
            u: [resource as u64, (subresource as u64) | ((before as u64) << 32), after as u64],
        })
    } else {
        None
    }
}

unsafe extern "win64" fn core_cmd_pipeline_barrier(
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
        let tr = ptr::read_unaligned(transitions);
        if tr.buffer_num > 0 && !tr.buffers.is_null() {
            for i in 0..tr.buffer_num as usize {
                let bd = ptr::read_unaligned(tr.buffers.add(i));
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
                let td = ptr::read_unaligned(tr.textures.add(i));
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
        let al = ptr::read_unaligned(aliasing);
        if al.buffer_num > 0 && !al.buffers.is_null() {
            for i in 0..al.buffer_num as usize {
                let bd = ptr::read_unaligned(al.buffers.add(i));
                let before = if bd.before.is_null() { 0 } else { (*bd.before).d3d12_resource as u64 };
                let after = if bd.after.is_null() { 0 } else { (*bd.after).d3d12_resource as u64 };
                barriers.push(ResourceBarrier { Type: 1, Flags: 0, u: [before, after, 0] });
            }
        }
        if al.texture_num > 0 && !al.textures.is_null() {
            for i in 0..al.texture_num as usize {
                let td = ptr::read_unaligned(al.textures.add(i));
                let before = if td.before.is_null() { 0 } else { (*td.before).d3d12_resource as u64 };
                let after = if td.after.is_null() { 0 } else { (*td.after).d3d12_resource as u64 };
                barriers.push(ResourceBarrier { Type: 1, Flags: 0, u: [before, after, 0] });
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

unsafe extern "win64" fn core_cmd_begin_render_pass(
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

unsafe extern "win64" fn core_cmd_end_render_pass(cmd: *mut NriCommandBuffer) {
    if !cmd.is_null() {
        (*cmd).frame_buffer = ptr::null();
    }
}

unsafe extern "win64" fn core_cmd_set_viewports(
    cmd: *mut NriCommandBuffer,
    viewports: *const NriViewport,
    num: u32,
) {
    if cmd.is_null() || (num > 0 && viewports.is_null()) {
        return;
    }
    let mut vps = Vec::with_capacity(num as usize);
    for i in 0..num as usize {
        let v = ptr::read_unaligned(viewports.add(i));
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

unsafe extern "win64" fn core_cmd_set_scissors(
    cmd: *mut NriCommandBuffer,
    rects: *const NriRect,
    num: u32,
) {
    if cmd.is_null() || (num > 0 && rects.is_null()) {
        return;
    }
    let mut drects = Vec::with_capacity(num as usize);
    for i in 0..num as usize {
        let r = ptr::read_unaligned(rects.add(i));
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
const CMD_SET_DEPTH_BOUNDS_ENTRY: *const () = core_cmd_set_depth_bounds::bridge as *const ();
#[cfg(not(target_arch = "aarch64"))]
const CMD_SET_DEPTH_BOUNDS_ENTRY: *const () = core_cmd_set_depth_bounds as *const ();

unsafe extern "win64" fn core_cmd_set_stencil_reference(cmd: *mut NriCommandBuffer, reference: u8) {
    if !cmd.is_null() && !(*cmd).d3d12_list.is_null() {
        d3d12_cmd_om_set_stencil_ref((*cmd).d3d12_list, reference as u32);
    }
}

unsafe extern "win64" fn core_cmd_set_sample_positions(
    _cmd: *mut NriCommandBuffer,
    _positions: *const u8,
    _position_num: u32,
) {
    crate::stub!("CmdSetSamplePositions: programmable sample positions not supported on Metal");
}

unsafe extern "win64" fn core_cmd_clear_attachments(
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
        clears.push(ptr::read_unaligned(clear_descs.add(i)));
    }
    let mut drects = Vec::with_capacity(rect_num as usize);
    if !rects.is_null() {
        for i in 0..rect_num as usize {
            let r = ptr::read_unaligned(rects.add(i));
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

unsafe extern "win64" fn core_cmd_set_index_buffer(
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

unsafe extern "win64" fn core_cmd_set_vertex_buffers(
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
        let bp = ptr::read_unaligned(buffers.add(i));
        if bp.is_null() || (*bp).d3d12_resource.is_null() {
            views.push(VertexBufferView { va: 0, size: 0, stride: 0 });
        } else {
            let b = &*bp;
            let off = if offsets.is_null() { 0 } else { ptr::read_unaligned(offsets.add(i)) };
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

unsafe extern "win64" fn core_cmd_draw(
    cmd: *mut NriCommandBuffer,
    vertex_num: u32,
    instance_num: u32,
    base_vertex: u32,
    base_instance: u32,
) {
    if !cmd.is_null() && !(*cmd).d3d12_list.is_null() {
        d3d12_cmd_draw((*cmd).d3d12_list, vertex_num, instance_num, base_vertex, base_instance);
    }
}

unsafe extern "win64" fn core_cmd_draw_indexed(
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

unsafe extern "win64" fn core_cmd_draw_indirect(
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
        d3d12_cmd_execute_indirect((*cmd).d3d12_list, sig, draw_num, (*buffer).d3d12_resource, offset);
    }
}

unsafe extern "win64" fn core_cmd_draw_indexed_indirect(
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
        d3d12_cmd_execute_indirect((*cmd).d3d12_list, sig, draw_num, (*buffer).d3d12_resource, offset);
    }
}

unsafe extern "win64" fn core_cmd_dispatch(cmd: *mut NriCommandBuffer, x: u32, y: u32, z: u32) {
    if !cmd.is_null() && !(*cmd).d3d12_list.is_null() {
        d3d12_cmd_dispatch((*cmd).d3d12_list, x, y, z);
    }
}

unsafe extern "win64" fn core_cmd_dispatch_indirect(
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

unsafe extern "win64" fn core_cmd_begin_query(
    cmd: *mut NriCommandBuffer,
    pool: *const NriQueryPool,
    offset: u32,
) {
    if cmd.is_null() || pool.is_null() {
        return;
    }
    d3d12_cmd_begin_query((*cmd).d3d12_list, (*pool).d3d12_query_heap, (*pool).d3d12_query_type, offset);
}

unsafe extern "win64" fn core_cmd_end_query(
    cmd: *mut NriCommandBuffer,
    pool: *const NriQueryPool,
    offset: u32,
) {
    if cmd.is_null() || pool.is_null() {
        return;
    }
    d3d12_cmd_end_query((*cmd).d3d12_list, (*pool).d3d12_query_heap, (*pool).d3d12_query_type, offset);
}

unsafe extern "win64" fn core_cmd_begin_annotation(cmd: *mut NriCommandBuffer, name: *const u8) {
    if !cmd.is_null() && !(*cmd).d3d12_list.is_null() {
        d3d12_cmd_begin_event((*cmd).d3d12_list, name);
    }
}

unsafe extern "win64" fn core_cmd_end_annotation(cmd: *mut NriCommandBuffer) {
    if !cmd.is_null() && !(*cmd).d3d12_list.is_null() {
        d3d12_cmd_end_event((*cmd).d3d12_list);
    }
}

unsafe extern "win64" fn core_cmd_clear_storage_buffer(
    cmd: *mut NriCommandBuffer,
    desc_ptr: *const NriClearStorageBufferDesc,
) {
    if cmd.is_null() || desc_ptr.is_null() {
        return;
    }
    let cd = ptr::read_unaligned(desc_ptr);
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
        .map_or(0, |ds| unsafe { (*ds).gpu_ptr(cd.range_index as usize, cd.offset_in_range) });
    let vals = [cd.value; 4];
    d3d12_cmd_clear_uav_uint((*cmd).d3d12_list, gpu, view.cpu_handle, view.d3d12_resource, &vals);
}

unsafe extern "win64" fn core_cmd_clear_storage_texture(
    cmd: *mut NriCommandBuffer,
    desc_ptr: *const NriClearStorageTextureDesc,
) {
    if cmd.is_null() || desc_ptr.is_null() {
        return;
    }
    let cd = ptr::read_unaligned(desc_ptr);
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
        .map_or(0, |ds| unsafe { (*ds).gpu_ptr(cd.range_index as usize, cd.offset_in_range) });
    if view.is_float_uav {
        let vals = [
            f32::from_bits(cd.value[0]),
            f32::from_bits(cd.value[1]),
            f32::from_bits(cd.value[2]),
            f32::from_bits(cd.value[3]),
        ];
        d3d12_cmd_clear_uav_float((*cmd).d3d12_list, gpu, view.cpu_handle, view.d3d12_resource, &vals);
    } else {
        d3d12_cmd_clear_uav_uint((*cmd).d3d12_list, gpu, view.cpu_handle, view.d3d12_resource, &cd.value);
    }
}

unsafe extern "win64" fn core_cmd_copy_buffer(
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

unsafe extern "win64" fn core_cmd_copy_texture(
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
    let dr = ptr::read_unaligned(dst_region);
    let sr = ptr::read_unaligned(src_region);
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
        if sr.size[0] as u64 == NRI_WHOLE_SIZE { st.dim_size(0, sr.mip_offset) } else { sr.size[0] },
        if sr.size[1] as u64 == NRI_WHOLE_SIZE { st.dim_size(1, sr.mip_offset) } else { sr.size[1] },
        if sr.size[2] as u64 == NRI_WHOLE_SIZE { st.dim_size(2, sr.mip_offset) } else { sr.size[2] },
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

unsafe extern "win64" fn core_cmd_upload_buffer_to_texture(
    cmd: *mut NriCommandBuffer,
    dst: *mut NriTexture,
    dst_region_ptr: *const NriTextureRegionDesc,
    src: *const NriBuffer,
    src_layout_ptr: *const NriTextureDataLayoutDesc,
) {
    if cmd.is_null() || dst.is_null() || dst_region_ptr.is_null() || src.is_null() || src_layout_ptr.is_null() {
        return;
    }
    let (dt, sb) = (&*dst, &*src);
    if dt.d3d12_resource.is_null() || sb.d3d12_resource.is_null() {
        return;
    }
    let dr = ptr::read_unaligned(dst_region_ptr);
    let sl = ptr::read_unaligned(src_layout_ptr);
    let sz = [
        if dr.size[0] as u64 == NRI_WHOLE_SIZE { dt.dim_size(0, dr.mip_offset) } else { dr.size[0] },
        if dr.size[1] as u64 == NRI_WHOLE_SIZE { dt.dim_size(1, dr.mip_offset) } else { dr.size[1] },
        if dr.size[2] as u64 == NRI_WHOLE_SIZE { dt.dim_size(2, dr.mip_offset) } else { dr.size[2] },
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

unsafe extern "win64" fn core_cmd_readback_texture_to_buffer(
    cmd: *mut NriCommandBuffer,
    dst: *mut NriBuffer,
    dst_layout_ptr: *const NriTextureDataLayoutDesc,
    src: *const NriTexture,
    src_region_ptr: *const NriTextureRegionDesc,
) {
    if cmd.is_null() || dst.is_null() || dst_layout_ptr.is_null() || src.is_null() || src_region_ptr.is_null() {
        return;
    }
    let (db, st) = (&*dst, &*src);
    if db.d3d12_resource.is_null() || st.d3d12_resource.is_null() {
        return;
    }
    let dl = ptr::read_unaligned(dst_layout_ptr);
    let sr = ptr::read_unaligned(src_region_ptr);
    let sz = [
        if sr.size[0] as u64 == NRI_WHOLE_SIZE { st.dim_size(0, sr.mip_offset) } else { sr.size[0] },
        if sr.size[1] as u64 == NRI_WHOLE_SIZE { st.dim_size(1, sr.mip_offset) } else { sr.size[1] },
        if sr.size[2] as u64 == NRI_WHOLE_SIZE { st.dim_size(2, sr.mip_offset) } else { sr.size[2] },
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

unsafe extern "win64" fn core_cmd_copy_queries(
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

unsafe extern "win64" fn core_cmd_reset_queries(
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

unsafe extern "win64" fn core_get_fence_value(fence: *const NriFence) -> u64 {
    if fence.is_null() || (*fence).d3d12_fence.is_null() {
        return 0;
    }
    d3d12_fence_completed((*fence).d3d12_fence)
}

unsafe extern "win64" fn core_queue_signal(
    queue: *mut NriCommandQueue,
    fence: *mut NriFence,
    value: u64,
) {
    if queue.is_null() || fence.is_null() {
        return;
    }
    let _ = d3d12_queue_signal((*queue).d3d12_queue, (*fence).d3d12_fence, value);
}

unsafe extern "win64" fn core_queue_wait(
    queue: *mut NriCommandQueue,
    fence: *mut NriFence,
    value: u64,
) {
    if queue.is_null() || fence.is_null() {
        return;
    }
    let _ = d3d12_queue_wait((*queue).d3d12_queue, (*fence).d3d12_fence, value);
}

unsafe extern "win64" fn core_wait(fence: *mut NriFence, value: u64) {
    if fence.is_null() || (*fence).d3d12_fence.is_null() {
        return;
    }
    cs::flush();
    if d3d12_fence_completed((*fence).d3d12_fence) < value {
        let _ = d3d12_fence_set_event((*fence).d3d12_fence, value, ptr::null_mut());
    }
}

unsafe extern "win64" fn core_queue_submit(
    queue: *mut NriCommandQueue,
    desc_ptr: *const NriQueueSubmitDesc,
) {
    if queue.is_null() || desc_ptr.is_null() {
        return;
    }
    let sd = ptr::read_unaligned(desc_ptr);
    if sd.command_buffer_num == 0 || sd.command_buffers.is_null() {
        return;
    }
    let mut lists = Vec::with_capacity(sd.command_buffer_num as usize);
    for i in 0..sd.command_buffer_num as usize {
        let cb = ptr::read_unaligned(sd.command_buffers.add(i));
        if !cb.is_null() && !(*cb).d3d12_list.is_null() {
            lists.push((*cb).d3d12_list);
        }
    }
    if !lists.is_empty() {
        d3d12_queue_execute((*queue).d3d12_queue, &lists);
    }
}

// ---------------------------------------------------------------------------
// CoreInterface — Descriptor sets, pools, allocators, Map/Unmap (slots 78..86)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn core_update_descriptor_ranges(
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
        let u = ptr::read_unaligned(updates.add(i));
        if u.descriptor_num == 0 || u.descriptors.is_null() {
            continue;
        }
        let base_off = rm.heap_offset + u.offset_in_range;
        for j in 0..u.descriptor_num {
            let dp = ptr::read_unaligned(u.descriptors.add(j as usize));
            if dp.is_null() {
                continue;
            }
            let dst = (*ds.pool).cpu_ptr(rm.heap_type, base_off + j);
            let src = (*dp).cpu_handle;
            d3d12_dev_copy_descriptors_simple(d3d_dev, 1, dst, src, rm.heap_type);
        }
    }
}

unsafe extern "win64" fn core_update_dynamic_constant_buffers(
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
        let dp = ptr::read_unaligned(descriptors.add(i));
        let idx = base_buffer as usize + i;
        if !dp.is_null() && idx < ds.dynamic_cbv_vas.len() {
            ds.dynamic_cbv_vas[idx] = (*dp).buffer_gpu_va;
        }
    }
}

unsafe extern "win64" fn core_copy_descriptor_set(
    dst_set: *mut NriDescriptorSet,
    copy_ptr: *const NriDescriptorSetCopyDesc,
) {
    if dst_set.is_null() || copy_ptr.is_null() {
        return;
    }
    let cd = ptr::read_unaligned(copy_ptr);
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
                d3d12_dev_copy_descriptors_simple(d3d_dev, rm.descriptor_num, d_ptr, s_ptr, rm.heap_type);
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

unsafe extern "win64" fn core_allocate_descriptor_sets(
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
        ptr::write_unaligned(out_sets.add(i), raw_set);
    }
    NRI_RESULT_SUCCESS
}

unsafe extern "win64" fn core_reset_descriptor_pool(pool: *mut NriDescriptorPool) {
    if !pool.is_null() {
        (*pool).reset();
    }
}

unsafe extern "win64" fn core_reset_command_allocator(allocator: *mut NriCommandAllocator) {
    if !allocator.is_null() && !(*allocator).d3d12_allocator.is_null() {
        let _ = d3d12_alloc_reset((*allocator).d3d12_allocator);
    }
}

unsafe extern "win64" fn core_map_buffer(
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
            log::warn!(target: "d3d", "NRI MapBuffer({buffer:?}, off={offset:#x}, size={size:#x}) failed: {hr:#x}");
            ptr::null_mut()
        }
    }
}

unsafe extern "win64" fn core_unmap_buffer(buffer: *mut NriBuffer) {
    if !buffer.is_null() && !(*buffer).d3d12_resource.is_null() {
        d3d12_res_unmap((*buffer).d3d12_resource);
    }
}

// ---------------------------------------------------------------------------
// CoreInterface — Debug names & Native objects (slots 86..106)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn core_set_device_name(obj: *mut NriDevice, name: *const u8) {
    if !obj.is_null() { d3d12_set_name_cstr((*obj).d3d12_device, name); }
}
unsafe extern "win64" fn core_set_queue_name(obj: *mut NriCommandQueue, name: *const u8) {
    if !obj.is_null() { d3d12_set_name_cstr((*obj).d3d12_queue, name); }
}
unsafe extern "win64" fn core_set_fence_name(obj: *mut NriFence, name: *const u8) {
    if !obj.is_null() { d3d12_set_name_cstr((*obj).d3d12_fence, name); }
}
unsafe extern "win64" fn core_set_allocator_name(obj: *mut NriCommandAllocator, name: *const u8) {
    if !obj.is_null() { d3d12_set_name_cstr((*obj).d3d12_allocator, name); }
}
unsafe extern "win64" fn core_set_pool_name(obj: *mut NriDescriptorPool, name: *const u8) {
    if !obj.is_null() {
        for i in 0..(*obj).heap_count {
            d3d12_set_name_cstr((*obj).heap_ptrs[i], name);
        }
    }
}
unsafe extern "win64" fn core_set_buffer_name(obj: *mut NriBuffer, name: *const u8) {
    if !obj.is_null() { d3d12_set_name_cstr((*obj).d3d12_resource, name); }
}
unsafe extern "win64" fn core_set_texture_name(obj: *mut NriTexture, name: *const u8) {
    if !obj.is_null() { d3d12_set_name_cstr((*obj).d3d12_resource, name); }
}
unsafe extern "win64" fn core_set_pipeline_layout_name(obj: *mut NriPipelineLayout, name: *const u8) {
    if !obj.is_null() { d3d12_set_name_cstr((*obj).d3d12_root_sig, name); }
}
unsafe extern "win64" fn core_set_pipeline_name(obj: *mut NriPipeline, name: *const u8) {
    if !obj.is_null() { d3d12_set_name_cstr((*obj).d3d12_pso, name); }
}
unsafe extern "win64" fn core_set_query_pool_name(obj: *mut NriQueryPool, name: *const u8) {
    if !obj.is_null() { d3d12_set_name_cstr((*obj).d3d12_query_heap, name); }
}
unsafe extern "win64" fn core_set_cmdbuf_name(obj: *mut NriCommandBuffer, name: *const u8) {
    if !obj.is_null() { d3d12_set_name_cstr((*obj).d3d12_list, name); }
}
unsafe extern "win64" fn core_set_memory_name(obj: *mut NriMemory, name: *const u8) {
    if !obj.is_null() { d3d12_set_name_cstr((*obj).d3d12_heap, name); }
}
unsafe extern "win64" fn core_set_name_noop(_obj: usize, _name: *const u8) {}

unsafe extern "win64" fn core_get_device_native_object(device: *const NriDevice) -> *mut c_void {
    if device.is_null() { ptr::null_mut() } else { (*device).d3d12_device }
}

unsafe extern "win64" fn core_get_cmdbuf_native_object(cmd: *const NriCommandBuffer) -> *mut c_void {
    if cmd.is_null() { ptr::null_mut() } else { (*cmd).d3d12_list }
}

unsafe extern "win64" fn core_get_buffer_native_object(
    buffer: *const NriBuffer,
    _physical_device_index: u32,
) -> u64 {
    if buffer.is_null() { 0 } else { (*buffer).d3d12_resource as usize as u64 }
}

unsafe extern "win64" fn core_get_texture_native_object(
    texture: *const NriTexture,
    _physical_device_index: u32,
) -> u64 {
    if texture.is_null() { 0 } else { (*texture).d3d12_resource as usize as u64 }
}

unsafe extern "win64" fn core_get_descriptor_native_object(
    descriptor: *const NriDescriptor,
    _physical_device_index: u32,
) -> u64 {
    if descriptor.is_null() { 0 } else { (*descriptor).cpu_handle as u64 }
}

// ---------------------------------------------------------------------------
// HelperInterface callbacks (`third_party/nri/Source/Shared/Helper*.cpp`)
// ---------------------------------------------------------------------------

#[derive(Default)]
struct MemoryTypeGroup {
    buffers: Vec<(*mut NriBuffer, u64)>,
    textures: Vec<(*mut NriTexture, u64)>,
    total_size: u64,
}

unsafe fn group_resources_for_allocation(
    device: *mut NriDevice,
    rg: &NriResourceGroupDesc,
) -> (BTreeMap<u32, MemoryTypeGroup>, Vec<(*mut NriBuffer, NriMemoryDesc)>, Vec<(*mut NriTexture, NriMemoryDesc)>) {
    let mut groups: BTreeMap<u32, MemoryTypeGroup> = BTreeMap::new();
    let mut ded_buffers = Vec::new();
    let mut ded_textures = Vec::new();
    let gran = (*device).desc.buffer_texture_granularity.max(1) as u64;

    if rg.buffer_num > 0 && !rg.buffers.is_null() {
        for i in 0..rg.buffer_num as usize {
            let buf = ptr::read_unaligned(rg.buffers.add(i));
            if buf.is_null() {
                continue;
            }
            let mut md = NriMemoryDesc::default();
            core_get_buffer_memory_info(buf, rg.memory_location, &mut md);
            if md.must_be_dedicated != 0 {
                ded_buffers.push((buf, md));
            } else {
                let g = groups.entry(md.memory_type).or_default();
                let off = align_up(g.total_size, md.alignment.max(1) as u64);
                g.buffers.push((buf, off));
                g.total_size = off.saturating_add(md.size);
            }
        }
    }

    if rg.texture_num > 0 && !rg.textures.is_null() {
        for i in 0..rg.texture_num as usize {
            let tex = ptr::read_unaligned(rg.textures.add(i));
            if tex.is_null() {
                continue;
            }
            let mut md = NriMemoryDesc::default();
            core_get_texture_memory_info(tex, rg.memory_location, &mut md);
            if md.must_be_dedicated != 0 {
                ded_textures.push((tex, md));
            } else {
                let g = groups.entry(md.memory_type).or_default();
                if g.textures.is_empty() && g.total_size > 0 {
                    g.total_size = align_up(g.total_size, gran);
                }
                let off = align_up(g.total_size, md.alignment.max(1) as u64);
                g.textures.push((tex, off));
                g.total_size = off.saturating_add(md.size);
            }
        }
    }

    (groups, ded_buffers, ded_textures)
}

unsafe extern "win64" fn helper_calculate_allocation_number(
    device: *mut NriDevice,
    rg_ptr: *const NriResourceGroupDesc,
) -> u32 {
    if device.is_null() || rg_ptr.is_null() {
        return 0;
    }
    let rg = ptr::read_unaligned(rg_ptr);
    let (groups, ded_bufs, ded_texs) = group_resources_for_allocation(device, &rg);
    (groups.len() + ded_bufs.len() + ded_texs.len()) as u32
}

unsafe extern "win64" fn helper_allocate_and_bind_memory(
    device: *mut NriDevice,
    rg_ptr: *const NriResourceGroupDesc,
    allocations: *mut *mut NriMemory,
) -> u32 {
    if device.is_null() || rg_ptr.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let rg = ptr::read_unaligned(rg_ptr);
    let (groups, ded_bufs, ded_texs) = group_resources_for_allocation(device, &rg);
    let total = groups.len() + ded_bufs.len() + ded_texs.len();
    if total == 0 {
        return NRI_RESULT_SUCCESS;
    }
    if allocations.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }

    let mut created: Vec<*mut NriMemory> = Vec::with_capacity(total);
    let mut buf_binds: Vec<NriBufferMemoryBindingDesc> = Vec::new();
    let mut tex_binds: Vec<NriTextureMemoryBindingDesc> = Vec::new();
    let mut status = NRI_RESULT_SUCCESS;

    for (mem_type, group) in groups {
        let mut mem: *mut NriMemory = ptr::null_mut();
        status = core_allocate_memory(device, NRI_WHOLE_DEVICE_GROUP, mem_type, group.total_size, &mut mem);
        if status != NRI_RESULT_SUCCESS {
            break;
        }
        ptr::write_unaligned(allocations.add(created.len()), mem);
        created.push(mem);
        for (buf, off) in group.buffers {
            buf_binds.push(NriBufferMemoryBindingDesc {
                memory: mem,
                buffer: buf,
                offset: off,
                physical_device_mask: NRI_WHOLE_DEVICE_GROUP,
                _pad: 0,
            });
        }
        for (tex, off) in group.textures {
            tex_binds.push(NriTextureMemoryBindingDesc {
                memory: mem,
                texture: tex,
                offset: off,
                physical_device_mask: NRI_WHOLE_DEVICE_GROUP,
                _pad: 0,
            });
        }
    }

    if status == NRI_RESULT_SUCCESS {
        for (buf, md) in ded_bufs {
            let mut mem: *mut NriMemory = ptr::null_mut();
            status = core_allocate_memory(device, NRI_WHOLE_DEVICE_GROUP, md.memory_type, md.size, &mut mem);
            if status != NRI_RESULT_SUCCESS {
                break;
            }
            ptr::write_unaligned(allocations.add(created.len()), mem);
            created.push(mem);
            buf_binds.push(NriBufferMemoryBindingDesc {
                memory: mem,
                buffer: buf,
                offset: 0,
                physical_device_mask: NRI_WHOLE_DEVICE_GROUP,
                _pad: 0,
            });
        }
    }

    if status == NRI_RESULT_SUCCESS {
        for (tex, md) in ded_texs {
            let mut mem: *mut NriMemory = ptr::null_mut();
            status = core_allocate_memory(device, NRI_WHOLE_DEVICE_GROUP, md.memory_type, md.size, &mut mem);
            if status != NRI_RESULT_SUCCESS {
                break;
            }
            ptr::write_unaligned(allocations.add(created.len()), mem);
            created.push(mem);
            tex_binds.push(NriTextureMemoryBindingDesc {
                memory: mem,
                texture: tex,
                offset: 0,
                physical_device_mask: NRI_WHOLE_DEVICE_GROUP,
                _pad: 0,
            });
        }
    }

    if status == NRI_RESULT_SUCCESS && !buf_binds.is_empty() {
        status = core_bind_buffer_memory(device, buf_binds.as_ptr(), buf_binds.len() as u32);
    }
    if status == NRI_RESULT_SUCCESS && !tex_binds.is_empty() {
        status = core_bind_texture_memory(device, tex_binds.as_ptr(), tex_binds.len() as u32);
    }

    if status != NRI_RESULT_SUCCESS {
        for (idx, mem) in created.into_iter().enumerate() {
            core_free_memory(mem);
            ptr::write_unaligned(allocations.add(idx), ptr::null_mut());
        }
    }
    status
}

unsafe extern "win64" fn helper_wait_for_idle(queue: *mut NriCommandQueue) -> u32 {
    if queue.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let dev = (*queue).device;
    let mut fence: *mut NriFence = ptr::null_mut();
    let res = core_create_fence(dev, 0, &mut fence);
    if res != NRI_RESULT_SUCCESS {
        return res;
    }
    core_queue_signal(queue, fence, 1);
    core_wait(fence, 1);
    core_destroy_fence(fence);
    NRI_RESULT_SUCCESS
}

unsafe extern "win64" fn helper_change_resource_states(
    queue: *mut NriCommandQueue,
    transitions: *const NriTransitionBarrierDesc,
) -> u32 {
    if queue.is_null() || transitions.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let mut alloc: *mut NriCommandAllocator = ptr::null_mut();
    let res = core_create_command_allocator(queue, NRI_WHOLE_DEVICE_GROUP, &mut alloc);
    if res != NRI_RESULT_SUCCESS {
        return res;
    }
    let mut cmd: *mut NriCommandBuffer = ptr::null_mut();
    let res = core_create_command_buffer(alloc, &mut cmd);
    if res != NRI_RESULT_SUCCESS {
        core_destroy_command_allocator(alloc);
        return res;
    }
    let mut status = core_begin_command_buffer(cmd, ptr::null(), 0);
    if status == NRI_RESULT_SUCCESS {
        core_cmd_pipeline_barrier(cmd, transitions, ptr::null(), 0);
        status = core_end_command_buffer(cmd);
    }
    if status == NRI_RESULT_SUCCESS {
        let cb_ptr: *const NriCommandBuffer = cmd;
        let submit = NriQueueSubmitDesc {
            command_buffers: &cb_ptr,
            command_buffer_num: 1,
            physical_device_index: 0,
        };
        core_queue_submit(queue, &submit);
        status = helper_wait_for_idle(queue);
    }
    core_destroy_command_buffer(cmd);
    core_destroy_command_allocator(alloc);
    status
}

unsafe extern "win64" fn helper_upload_data(
    queue: *mut NriCommandQueue,
    tex_descs: *const NriTextureUploadDesc,
    tex_num: u32,
    buf_descs: *const NriBufferUploadDesc,
    buf_num: u32,
) -> u32 {
    if queue.is_null()
        || (tex_num > 0 && tex_descs.is_null())
        || (buf_num > 0 && buf_descs.is_null())
    {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    if tex_num == 0 && buf_num == 0 {
        return NRI_RESULT_SUCCESS;
    }
    let dev = (*queue).device;
    let row_align = (*dev).desc.upload_buffer_texture_row_alignment.max(1) as u64;
    let slice_align = (*dev).desc.upload_buffer_texture_slice_alignment.max(1) as u64;

    let mut staging_size: u64 = 65536;
    for i in 0..tex_num as usize {
        let td = ptr::read_unaligned(tex_descs.add(i));
        if td.subresources.is_null() {
            continue;
        }
        let sub = ptr::read_unaligned(td.subresources);
        if sub.row_pitch == 0 {
            continue;
        }
        let rows = (sub.slice_pitch / sub.row_pitch).max(1) as u64;
        let aligned_row = align_up(sub.row_pitch as u64, row_align);
        let aligned_slice = align_up(rows * aligned_row, slice_align);
        let total = aligned_slice * (sub.slice_num.max(1) as u64);
        staging_size = staging_size.max(total);
    }
    staging_size = align_up(staging_size, COPY_ALIGNMENT);

    // Create staging buffer + memory + fence + command allocator + command buffer.
    let mut upload_buf: *mut NriBuffer = ptr::null_mut();
    let bd = NriBufferDesc {
        size: staging_size,
        structure_stride: 0,
        usage_mask: 0,
        _pad0: 0,
        physical_device_mask: 0,
        _pad1: 0,
    };
    let mut status = core_create_buffer(dev, &bd, &mut upload_buf);
    if status != NRI_RESULT_SUCCESS {
        return status;
    }
    let mut md = NriMemoryDesc::default();
    core_get_buffer_memory_info(upload_buf, 1, &mut md); // HOST_UPLOAD = 1
    let mut upload_mem: *mut NriMemory = ptr::null_mut();
    status = core_allocate_memory(dev, NRI_WHOLE_DEVICE_GROUP, md.memory_type, md.size, &mut upload_mem);
    if status != NRI_RESULT_SUCCESS {
        core_destroy_buffer(upload_buf);
        return status;
    }
    let bind = NriBufferMemoryBindingDesc {
        memory: upload_mem,
        buffer: upload_buf,
        offset: 0,
        physical_device_mask: NRI_WHOLE_DEVICE_GROUP,
        _pad: 0,
    };
    status = core_bind_buffer_memory(dev, &bind, 1);
    let mut fence: *mut NriFence = ptr::null_mut();
    let mut alloc: *mut NriCommandAllocator = ptr::null_mut();
    let mut cmd: *mut NriCommandBuffer = ptr::null_mut();
    if status == NRI_RESULT_SUCCESS {
        status = core_create_fence(dev, 0, &mut fence);
    }
    if status == NRI_RESULT_SUCCESS {
        status = core_create_command_allocator(queue, NRI_WHOLE_DEVICE_GROUP, &mut alloc);
    }
    if status == NRI_RESULT_SUCCESS {
        status = core_create_command_buffer(alloc, &mut cmd);
    }

    let mut fence_val = 1u64;
    let submit_and_wait = |cmd: *mut NriCommandBuffer,
                           alloc: *mut NriCommandAllocator,
                           fence: *mut NriFence,
                           fence_val: &mut u64|
     -> u32 {
        unsafe {
            let r = core_end_command_buffer(cmd);
            if r != NRI_RESULT_SUCCESS {
                return r;
            }
            let cb_ptr: *const NriCommandBuffer = cmd;
            let submit = NriQueueSubmitDesc {
                command_buffers: &cb_ptr,
                command_buffer_num: 1,
                physical_device_index: 0,
            };
            core_queue_submit(queue, &submit);
            core_queue_signal(queue, fence, *fence_val);
            core_wait(fence, *fence_val);
            *fence_val += 1;
            core_reset_command_allocator(alloc);
            NRI_RESULT_SUCCESS
        }
    };

    // Upload textures.
    if status == NRI_RESULT_SUCCESS && tex_num > 0 {
        let mut init_barriers = Vec::with_capacity(tex_num as usize);
        let mut final_barriers = Vec::with_capacity(tex_num as usize);
        for i in 0..tex_num as usize {
            let td = ptr::read_unaligned(tex_descs.add(i));
            init_barriers.push(NriTextureTransitionBarrierDesc {
                texture: td.texture,
                mip_offset: 0,
                mip_num: td.mip_num,
                array_offset: 0,
                array_size: td.array_size,
                prev_access: 0,
                next_access: 1 << 10, // COPY_DESTINATION
                prev_layout: 8,       // UNKNOWN
                next_layout: 0,       // GENERAL
                _pad: 0,
            });
            final_barriers.push(NriTextureTransitionBarrierDesc {
                texture: td.texture,
                mip_offset: 0,
                mip_num: td.mip_num,
                array_offset: 0,
                array_size: td.array_size,
                prev_access: 1 << 10,
                next_access: td.next_access,
                prev_layout: 0,
                next_layout: td.next_layout,
                _pad: 0,
            });
        }
        status = core_begin_command_buffer(cmd, ptr::null(), 0);
        if status == NRI_RESULT_SUCCESS {
            let tb = NriTransitionBarrierDesc {
                buffers: ptr::null(),
                textures: init_barriers.as_ptr(),
                buffer_num: 0,
                texture_num: init_barriers.len() as u32,
            };
            core_cmd_pipeline_barrier(cmd, &tb, ptr::null(), 3);
            status = submit_and_wait(cmd, alloc, fence, &mut fence_val);
        }

        let mut i = 0usize;
        let mut arr_off = 0u16;
        let mut mip_off = 0u16;
        while status == NRI_RESULT_SUCCESS && i < tex_num as usize {
            status = core_begin_command_buffer(cmd, ptr::null(), 0);
            if status != NRI_RESULT_SUCCESS {
                break;
            }
            let mut buf_off = 0u64;
            'batch: while i < tex_num as usize {
                let td = ptr::read_unaligned(tex_descs.add(i));
                if td.subresources.is_null() {
                    i += 1;
                    continue;
                }
                while arr_off < td.array_size {
                    while mip_off < td.mip_num {
                        let sub = ptr::read_unaligned(
                            td.subresources.add((arr_off as usize) * (td.mip_num as usize) + (mip_off as usize)),
                        );
                        if sub.row_pitch == 0 || sub.slices.is_null() {
                            mip_off += 1;
                            continue;
                        }
                        let rows = (sub.slice_pitch / sub.row_pitch).max(1) as u64;
                        let aligned_row = align_up(sub.row_pitch as u64, row_align);
                        let aligned_slice = align_up(rows * aligned_row, slice_align);
                        let content_size = aligned_slice * (sub.slice_num.max(1) as u64);
                        if content_size > staging_size {
                            status = NRI_RESULT_OUT_OF_MEMORY;
                            break 'batch;
                        }
                        if content_size > staging_size - buf_off {
                            break 'batch;
                        }
                        let dst_ptr = core_map_buffer(upload_buf, buf_off, content_size);
                        if dst_ptr.is_null() {
                            status = NRI_RESULT_FAILURE;
                            break 'batch;
                        }
                        for k in 0..sub.slice_num.max(1) as usize {
                            for l in 0..rows as usize {
                                let d = dst_ptr.add(k * (aligned_slice as usize) + l * (aligned_row as usize));
                                let s = sub.slices.add(k * (sub.slice_pitch as usize) + l * (sub.row_pitch as usize));
                                ptr::copy_nonoverlapping(s, d, sub.row_pitch as usize);
                            }
                        }
                        core_unmap_buffer(upload_buf);
                        let src_layout = NriTextureDataLayoutDesc {
                            offset: buf_off,
                            row_pitch: aligned_row as u32,
                            slice_pitch: aligned_slice as u32,
                        };
                        let dst_reg = NriTextureRegionDesc {
                            offset: [0; 3],
                            size: [0; 3],
                            mip_offset: mip_off,
                            array_offset: arr_off,
                        };
                        core_cmd_upload_buffer_to_texture(cmd, td.texture, &dst_reg, upload_buf, &src_layout);
                        buf_off = align_up(buf_off + content_size, COPY_ALIGNMENT);
                        mip_off += 1;
                    }
                    mip_off = 0;
                    arr_off += 1;
                }
                arr_off = 0;
                i += 1;
            }
            if status == NRI_RESULT_SUCCESS {
                status = submit_and_wait(cmd, alloc, fence, &mut fence_val);
            }
        }

        if status == NRI_RESULT_SUCCESS {
            status = core_begin_command_buffer(cmd, ptr::null(), 0);
            if status == NRI_RESULT_SUCCESS {
                let tb = NriTransitionBarrierDesc {
                    buffers: ptr::null(),
                    textures: final_barriers.as_ptr(),
                    buffer_num: 0,
                    texture_num: final_barriers.len() as u32,
                };
                core_cmd_pipeline_barrier(cmd, &tb, ptr::null(), 3);
                status = submit_and_wait(cmd, alloc, fence, &mut fence_val);
            }
        }
    }

    // Upload buffers.
    if status == NRI_RESULT_SUCCESS && buf_num > 0 {
        let mut init_barriers = Vec::with_capacity(buf_num as usize);
        let mut final_barriers = Vec::with_capacity(buf_num as usize);
        for i in 0..buf_num as usize {
            let bd_u = ptr::read_unaligned(buf_descs.add(i));
            init_barriers.push(NriBufferTransitionBarrierDesc {
                buffer: bd_u.buffer,
                prev_access: bd_u.prev_access,
                next_access: 1 << 10,
                _pad: 0,
            });
            final_barriers.push(NriBufferTransitionBarrierDesc {
                buffer: bd_u.buffer,
                prev_access: 1 << 10,
                next_access: bd_u.next_access,
                _pad: 0,
            });
        }
        status = core_begin_command_buffer(cmd, ptr::null(), 0);
        if status == NRI_RESULT_SUCCESS {
            let tb = NriTransitionBarrierDesc {
                buffers: init_barriers.as_ptr(),
                textures: ptr::null(),
                buffer_num: init_barriers.len() as u32,
                texture_num: 0,
            };
            core_cmd_pipeline_barrier(cmd, &tb, ptr::null(), 3);
            status = submit_and_wait(cmd, alloc, fence, &mut fence_val);
        }

        let mut i = 0usize;
        let mut content_off = 0u64;
        while status == NRI_RESULT_SUCCESS && i < buf_num as usize {
            status = core_begin_command_buffer(cmd, ptr::null(), 0);
            if status != NRI_RESULT_SUCCESS {
                break;
            }
            let mapped = core_map_buffer(upload_buf, 0, staging_size);
            if mapped.is_null() {
                status = NRI_RESULT_FAILURE;
                break;
            }
            let mut buf_off = 0u64;
            while i < buf_num as usize {
                let bd_u = ptr::read_unaligned(buf_descs.add(i));
                if bd_u.data_size == 0 || bd_u.data.is_null() {
                    i += 1;
                    content_off = 0;
                    continue;
                }
                let free = staging_size - buf_off;
                if free == 0 {
                    break;
                }
                let copy_n = (bd_u.data_size - content_off).min(free);
                ptr::copy_nonoverlapping(
                    bd_u.data.add(content_off as usize),
                    mapped.add(buf_off as usize),
                    copy_n as usize,
                );
                core_cmd_copy_buffer(
                    cmd,
                    bd_u.buffer,
                    0,
                    bd_u.buffer_offset + content_off,
                    upload_buf,
                    0,
                    buf_off,
                    copy_n,
                );
                content_off += copy_n;
                buf_off += copy_n;
                if content_off < bd_u.data_size {
                    break;
                }
                content_off = 0;
                buf_off = align_up(buf_off, COPY_ALIGNMENT);
                i += 1;
            }
            core_unmap_buffer(upload_buf);
            status = submit_and_wait(cmd, alloc, fence, &mut fence_val);
        }

        if status == NRI_RESULT_SUCCESS {
            status = core_begin_command_buffer(cmd, ptr::null(), 0);
            if status == NRI_RESULT_SUCCESS {
                let tb = NriTransitionBarrierDesc {
                    buffers: final_barriers.as_ptr(),
                    textures: ptr::null(),
                    buffer_num: final_barriers.len() as u32,
                    texture_num: 0,
                };
                core_cmd_pipeline_barrier(cmd, &tb, ptr::null(), 3);
                status = submit_and_wait(cmd, alloc, fence, &mut fence_val);
            }
        }
    }

    core_destroy_command_buffer(cmd);
    core_destroy_command_allocator(alloc);
    core_destroy_fence(fence);
    core_destroy_buffer(upload_buf);
    core_free_memory(upload_mem);
    status
}

// ---------------------------------------------------------------------------
// WrapperD3D12Interface callbacks (`third_party/nri/Include/Extensions/NRIWrapperD3D12.h`)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn wrapper_create_command_buffer_d3d12(
    device: *mut NriDevice,
    desc_ptr: *const NriCommandBufferD3D12Desc,
    out: *mut *mut NriCommandBuffer,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let d = ptr::read_unaligned(desc_ptr);
    if d.d3d12_command_list.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    com_add_ref(d.d3d12_command_list);
    if !d.d3d12_command_allocator.is_null() {
        com_add_ref(d.d3d12_command_allocator);
    }
    let list_type = d3d12_cmd_get_type(d.d3d12_command_list);
    ptr::write_unaligned(
        out,
        Box::into_raw(Box::new(NriCommandBuffer {
            device,
            d3d12_list: d.d3d12_command_list,
            d3d12_allocator: d.d3d12_command_allocator,
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

unsafe extern "win64" fn wrapper_create_buffer_d3d12(
    device: *mut NriDevice,
    desc_ptr: *const NriBufferD3D12Desc,
    out: *mut *mut NriBuffer,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let d = ptr::read_unaligned(desc_ptr);
    if d.d3d12_resource.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    com_add_ref(d.d3d12_resource);
    let rdesc = d3d12_res_get_desc(d.d3d12_resource);
    ptr::write_unaligned(
        out,
        Box::into_raw(Box::new(NriBuffer {
            device,
            d3d12_resource: d.d3d12_resource,
            desc: rdesc,
            structure_stride: d.structure_stride,
        })),
    );
    NRI_RESULT_SUCCESS
}

unsafe extern "win64" fn wrapper_create_texture_d3d12(
    device: *mut NriDevice,
    desc_ptr: *const NriTextureD3D12Desc,
    out: *mut *mut NriTexture,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let d = ptr::read_unaligned(desc_ptr);
    if d.d3d12_resource.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    com_add_ref(d.d3d12_resource);
    let rdesc = d3d12_res_get_desc(d.d3d12_resource);
    let nri_format = dxgi_to_nri(rdesc.Format);
    ptr::write_unaligned(
        out,
        Box::into_raw(Box::new(NriTexture {
            device,
            d3d12_resource: d.d3d12_resource,
            desc: rdesc,
            nri_format,
        })),
    );
    NRI_RESULT_SUCCESS
}

unsafe extern "win64" fn wrapper_create_memory_d3d12(
    device: *mut NriDevice,
    desc_ptr: *const NriMemoryD3D12Desc,
    out: *mut *mut NriMemory,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let d = ptr::read_unaligned(desc_ptr);
    if d.d3d12_heap.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    com_add_ref(d.d3d12_heap);
    let hdesc = d3d12_heap_get_desc(d.d3d12_heap);
    ptr::write_unaligned(
        out,
        Box::into_raw(Box::new(NriMemory {
            d3d12_heap: d.d3d12_heap,
            heap_desc: hdesc,
            dedicated: false,
        })),
    );
    NRI_RESULT_SUCCESS
}

unsafe extern "win64" fn wrapper_create_acceleration_structure_d3d12(
    device: *mut NriDevice,
    desc_ptr: *const NriAccelerationStructureD3D12Desc,
    out: *mut *mut NriAccelerationStructure,
) -> u32 {
    if device.is_null() || desc_ptr.is_null() || out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let d = ptr::read_unaligned(desc_ptr);
    if d.d3d12_resource.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    com_add_ref(d.d3d12_resource);
    ptr::write_unaligned(
        out,
        Box::into_raw(Box::new(NriAccelerationStructure {
            d3d12_resource: d.d3d12_resource,
            _scratch_size: d.scratch_data_size_in_bytes,
            _update_scratch_size: d.update_scratch_data_size_in_bytes,
        })),
    );
    NRI_RESULT_SUCCESS
}

// ---------------------------------------------------------------------------
// Function tables
// ---------------------------------------------------------------------------

/// `nri::CoreInterface` (`0x350` bytes = 106 function pointers).
static CORE_INTERFACE: InterfaceTable<106> = InterfaceTable([
    // Get (0..6)
    crate::Ptr(core_get_device_desc as *const ()),                // 00 (+0x000) GetDeviceDesc
    crate::Ptr(core_get_format_support as *const ()),             // 01 (+0x008) GetFormatSupport
    crate::Ptr(core_get_query_size as *const ()),                 // 02 (+0x010) GetQuerySize
    crate::Ptr(core_get_buffer_memory_info as *const ()),         // 03 (+0x018) GetBufferMemoryInfo
    crate::Ptr(core_get_texture_memory_info as *const ()),        // 04 (+0x020) GetTextureMemoryInfo
    crate::Ptr(core_get_command_queue as *const ()),              // 05 (+0x028) GetCommandQueue
    // Create (6..22)
    crate::Ptr(core_create_command_allocator as *const ()),       // 06 (+0x030) CreateCommandAllocator
    crate::Ptr(core_create_command_buffer as *const ()),          // 07 (+0x038) CreateCommandBuffer
    crate::Ptr(core_create_descriptor_pool as *const ()),         // 08 (+0x040) CreateDescriptorPool
    crate::Ptr(core_create_buffer as *const ()),                  // 09 (+0x048) CreateBuffer
    crate::Ptr(core_create_texture as *const ()),                 // 10 (+0x050) CreateTexture
    crate::Ptr(core_create_buffer_view as *const ()),             // 11 (+0x058) CreateBufferView
    crate::Ptr(core_create_texture_1d_view as *const ()),         // 12 (+0x060) CreateTexture1DView
    crate::Ptr(core_create_texture_2d_view as *const ()),         // 13 (+0x068) CreateTexture2DView
    crate::Ptr(core_create_texture_3d_view as *const ()),         // 14 (+0x070) CreateTexture3DView
    crate::Ptr(core_create_sampler as *const ()),                 // 15 (+0x078) CreateSampler
    crate::Ptr(core_create_pipeline_layout as *const ()),         // 16 (+0x080) CreatePipelineLayout
    crate::Ptr(core_create_graphics_pipeline as *const ()),       // 17 (+0x088) CreateGraphicsPipeline
    crate::Ptr(core_create_compute_pipeline as *const ()),        // 18 (+0x090) CreateComputePipeline
    crate::Ptr(core_create_frame_buffer as *const ()),            // 19 (+0x098) CreateFrameBuffer
    crate::Ptr(core_create_query_pool as *const ()),              // 20 (+0x0a0) CreateQueryPool
    crate::Ptr(core_create_fence as *const ()),                   // 21 (+0x0a8) CreateFence
    // Destroy (22..33)
    crate::Ptr(core_destroy_command_allocator as *const ()),      // 22 (+0x0b0) DestroyCommandAllocator
    crate::Ptr(core_destroy_command_buffer as *const ()),         // 23 (+0x0b8) DestroyCommandBuffer
    crate::Ptr(core_destroy_descriptor_pool as *const ()),        // 24 (+0x0c0) DestroyDescriptorPool
    crate::Ptr(core_destroy_buffer as *const ()),                 // 25 (+0x0c8) DestroyBuffer
    crate::Ptr(core_destroy_texture as *const ()),                // 26 (+0x0d0) DestroyTexture
    crate::Ptr(core_destroy_descriptor as *const ()),             // 27 (+0x0d8) DestroyDescriptor
    crate::Ptr(core_destroy_pipeline_layout as *const ()),        // 28 (+0x0e0) DestroyPipelineLayout
    crate::Ptr(core_destroy_pipeline as *const ()),               // 29 (+0x0e8) DestroyPipeline
    crate::Ptr(core_destroy_frame_buffer as *const ()),           // 30 (+0x0f0) DestroyFrameBuffer
    crate::Ptr(core_destroy_query_pool as *const ()),             // 31 (+0x0f8) DestroyQueryPool
    crate::Ptr(core_destroy_fence as *const ()),                  // 32 (+0x100) DestroyFence
    // Memory (33..37)
    crate::Ptr(core_allocate_memory as *const ()),                // 33 (+0x108) AllocateMemory
    crate::Ptr(core_bind_buffer_memory as *const ()),             // 34 (+0x110) BindBufferMemory
    crate::Ptr(core_bind_texture_memory as *const ()),            // 35 (+0x118) BindTextureMemory
    crate::Ptr(core_free_memory as *const ()),                    // 36 (+0x120) FreeMemory
    // Command buffer (37..73)
    crate::Ptr(core_begin_command_buffer as *const ()),           // 37 (+0x128) BeginCommandBuffer
    crate::Ptr(core_end_command_buffer as *const ()),             // 38 (+0x130) EndCommandBuffer
    crate::Ptr(core_cmd_set_pipeline as *const ()),               // 39 (+0x138) CmdSetPipeline
    crate::Ptr(core_cmd_set_pipeline_layout as *const ()),        // 40 (+0x140) CmdSetPipelineLayout
    crate::Ptr(core_cmd_set_descriptor_set as *const ()),         // 41 (+0x148) CmdSetDescriptorSet
    crate::Ptr(core_cmd_set_constants as *const ()),              // 42 (+0x150) CmdSetConstants
    crate::Ptr(core_cmd_set_descriptor_pool as *const ()),        // 43 (+0x158) CmdSetDescriptorPool
    crate::Ptr(core_cmd_pipeline_barrier as *const ()),           // 44 (+0x160) CmdPipelineBarrier
    crate::Ptr(core_cmd_begin_render_pass as *const ()),          // 45 (+0x168) CmdBeginRenderPass
    crate::Ptr(core_cmd_end_render_pass as *const ()),            // 46 (+0x170) CmdEndRenderPass
    crate::Ptr(core_cmd_set_viewports as *const ()),              // 47 (+0x178) CmdSetViewports
    crate::Ptr(core_cmd_set_scissors as *const ()),               // 48 (+0x180) CmdSetScissors
    crate::Ptr(CMD_SET_DEPTH_BOUNDS_ENTRY),                       // 49 (+0x188) CmdSetDepthBounds
    crate::Ptr(core_cmd_set_stencil_reference as *const ()),      // 50 (+0x190) CmdSetStencilReference
    crate::Ptr(core_cmd_set_sample_positions as *const ()),       // 51 (+0x198) CmdSetSamplePositions
    crate::Ptr(core_cmd_clear_attachments as *const ()),          // 52 (+0x1a0) CmdClearAttachments
    crate::Ptr(core_cmd_set_index_buffer as *const ()),           // 53 (+0x1a8) CmdSetIndexBuffer
    crate::Ptr(core_cmd_set_vertex_buffers as *const ()),         // 54 (+0x1b0) CmdSetVertexBuffers
    crate::Ptr(core_cmd_draw as *const ()),                       // 55 (+0x1b8) CmdDraw
    crate::Ptr(core_cmd_draw_indexed as *const ()),               // 56 (+0x1c0) CmdDrawIndexed
    crate::Ptr(core_cmd_draw_indirect as *const ()),              // 57 (+0x1c8) CmdDrawIndirect
    crate::Ptr(core_cmd_draw_indexed_indirect as *const ()),      // 58 (+0x1d0) CmdDrawIndexedIndirect
    crate::Ptr(core_cmd_dispatch as *const ()),                   // 59 (+0x1d8) CmdDispatch
    crate::Ptr(core_cmd_dispatch_indirect as *const ()),          // 60 (+0x1e0) CmdDispatchIndirect
    crate::Ptr(core_cmd_begin_query as *const ()),                // 61 (+0x1e8) CmdBeginQuery
    crate::Ptr(core_cmd_end_query as *const ()),                  // 62 (+0x1f0) CmdEndQuery
    crate::Ptr(core_cmd_begin_annotation as *const ()),           // 63 (+0x1f8) CmdBeginAnnotation
    crate::Ptr(core_cmd_end_annotation as *const ()),             // 64 (+0x200) CmdEndAnnotation
    crate::Ptr(core_cmd_clear_storage_buffer as *const ()),       // 65 (+0x208) CmdClearStorageBuffer
    crate::Ptr(core_cmd_clear_storage_texture as *const ()),      // 66 (+0x210) CmdClearStorageTexture
    crate::Ptr(core_cmd_copy_buffer as *const ()),                // 67 (+0x218) CmdCopyBuffer
    crate::Ptr(core_cmd_copy_texture as *const ()),               // 68 (+0x220) CmdCopyTexture
    crate::Ptr(core_cmd_upload_buffer_to_texture as *const ()),   // 69 (+0x228) CmdUploadBufferToTexture
    crate::Ptr(core_cmd_readback_texture_to_buffer as *const ()), // 70 (+0x230) CmdReadbackTextureToBuffer
    crate::Ptr(core_cmd_copy_queries as *const ()),               // 71 (+0x238) CmdCopyQueries
    crate::Ptr(core_cmd_reset_queries as *const ()),              // 72 (+0x240) CmdResetQueries
    // Synchronization (73..77)
    crate::Ptr(core_get_fence_value as *const ()),                // 73 (+0x248) GetFenceValue
    crate::Ptr(core_queue_signal as *const ()),                   // 74 (+0x250) QueueSignal
    crate::Ptr(core_queue_wait as *const ()),                     // 75 (+0x258) QueueWait
    crate::Ptr(core_wait as *const ()),                           // 76 (+0x260) Wait
    // Work submission (77)
    crate::Ptr(core_queue_submit as *const ()),                   // 77 (+0x268) QueueSubmit
    // Descriptor set (78..81)
    crate::Ptr(core_update_descriptor_ranges as *const ()),       // 78 (+0x270) UpdateDescriptorRanges
    crate::Ptr(core_update_dynamic_constant_buffers as *const ()),// 79 (+0x278) UpdateDynamicConstantBuffers
    crate::Ptr(core_copy_descriptor_set as *const ()),            // 80 (+0x280) CopyDescriptorSet
    // Descriptor pool (81..83)
    crate::Ptr(core_allocate_descriptor_sets as *const ()),       // 81 (+0x288) AllocateDescriptorSets
    crate::Ptr(core_reset_descriptor_pool as *const ()),          // 82 (+0x290) ResetDescriptorPool
    // Command allocator (83)
    crate::Ptr(core_reset_command_allocator as *const ()),        // 83 (+0x298) ResetCommandAllocator
    // Map / Unmap (84..86)
    crate::Ptr(core_map_buffer as *const ()),                     // 84 (+0x2a0) MapBuffer
    crate::Ptr(core_unmap_buffer as *const ()),                   // 85 (+0x2a8) UnmapBuffer
    // Debug name (86..101)
    crate::Ptr(core_set_device_name as *const ()),                // 86 (+0x2b0) SetDeviceDebugName
    crate::Ptr(core_set_queue_name as *const ()),                 // 87 (+0x2b8) SetCommandQueueDebugName
    crate::Ptr(core_set_fence_name as *const ()),                 // 88 (+0x2c0) SetFenceDebugName
    crate::Ptr(core_set_allocator_name as *const ()),             // 89 (+0x2c8) SetCommandAllocatorDebugName
    crate::Ptr(core_set_pool_name as *const ()),                  // 90 (+0x2d0) SetDescriptorPoolDebugName
    crate::Ptr(core_set_buffer_name as *const ()),                // 91 (+0x2d8) SetBufferDebugName
    crate::Ptr(core_set_texture_name as *const ()),               // 92 (+0x2e0) SetTextureDebugName
    crate::Ptr(core_set_name_noop as *const ()),                  // 93 (+0x2e8) SetDescriptorDebugName
    crate::Ptr(core_set_pipeline_layout_name as *const ()),       // 94 (+0x2f0) SetPipelineLayoutDebugName
    crate::Ptr(core_set_pipeline_name as *const ()),              // 95 (+0x2f8) SetPipelineDebugName
    crate::Ptr(core_set_name_noop as *const ()),                  // 96 (+0x300) SetFrameBufferDebugName
    crate::Ptr(core_set_query_pool_name as *const ()),            // 97 (+0x308) SetQueryPoolDebugName
    crate::Ptr(core_set_name_noop as *const ()),                  // 98 (+0x310) SetDescriptorSetDebugName
    crate::Ptr(core_set_cmdbuf_name as *const ()),                // 99 (+0x318) SetCommandBufferDebugName
    crate::Ptr(core_set_memory_name as *const ()),                // 100 (+0x320) SetMemoryDebugName
    // Native objects (101..106)
    crate::Ptr(core_get_device_native_object as *const ()),       // 101 (+0x328) GetDeviceNativeObject
    crate::Ptr(core_get_cmdbuf_native_object as *const ()),       // 102 (+0x330) GetCommandBufferNativeObject
    crate::Ptr(core_get_buffer_native_object as *const ()),       // 103 (+0x338) GetBufferNativeObject
    crate::Ptr(core_get_texture_native_object as *const ()),      // 104 (+0x340) GetTextureNativeObject
    crate::Ptr(core_get_descriptor_native_object as *const ()),   // 105 (+0x348) GetDescriptorNativeObject
]);

const _: () = assert!(size_of::<InterfaceTable<106>>() == 0x350);

/// `nri::HelperInterface` (`0x28` bytes = 5 function pointers).
static HELPER_INTERFACE: InterfaceTable<5> = InterfaceTable([
    crate::Ptr(helper_calculate_allocation_number as *const ()), // 0 (+0x00) CalculateAllocationNumber
    crate::Ptr(helper_allocate_and_bind_memory as *const ()),    // 1 (+0x08) AllocateAndBindMemory
    crate::Ptr(helper_change_resource_states as *const ()),      // 2 (+0x10) ChangeResourceStates
    crate::Ptr(helper_upload_data as *const ()),                 // 3 (+0x18) UploadData
    crate::Ptr(helper_wait_for_idle as *const ()),               // 4 (+0x20) WaitForIdle
]);

/// `nri::WrapperD3D12Interface` (`0x28` bytes = 5 function pointers).
static WRAPPER_D3D12_INTERFACE: InterfaceTable<5> = InterfaceTable([
    crate::Ptr(wrapper_create_command_buffer_d3d12 as *const ()),         // 0 (+0x00) CreateCommandBufferD3D12
    crate::Ptr(wrapper_create_buffer_d3d12 as *const ()),                 // 1 (+0x08) CreateBufferD3D12
    crate::Ptr(wrapper_create_texture_d3d12 as *const ()),                // 2 (+0x10) CreateTextureD3D12
    crate::Ptr(wrapper_create_memory_d3d12 as *const ()),                 // 3 (+0x18) CreateMemoryD3D12
    crate::Ptr(wrapper_create_acceleration_structure_d3d12 as *const ()), // 4 (+0x20) CreateAccelerationStructureD3D12
]);

fn ensure_interfaces_registered() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        for p in &CORE_INTERFACE.0 {
            let addr = p.0 as usize;
            crate::register_native_va(addr as u64, addr);
        }
        for p in &HELPER_INTERFACE.0 {
            let addr = p.0 as usize;
            crate::register_native_va(addr as u64, addr);
        }
        for p in &WRAPPER_D3D12_INTERFACE.0 {
            let addr = p.0 as usize;
            crate::register_native_va(addr as u64, addr);
        }
    });
}

// ---------------------------------------------------------------------------
// Exported NRI.dll entry points
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn nriCreateDeviceFromD3D12Device(
    device_desc: *const u8,
    device_out: *mut *mut NriDevice,
) -> u32 {
    ensure_interfaces_registered();
    if device_desc.is_null() || device_out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    let cd = ptr::read_unaligned(device_desc as *const NriDeviceCreationD3D12Desc);
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
                let list_type = if matches!(qdesc.Type, 0 | 2 | 3) { qdesc.Type } else { fallback_type };
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
            (*dev_ptr).desc.timestamp_frequency_hz = d3d12_queue_get_timestamp_freq((*gq).d3d12_queue);
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

    ptr::write_unaligned(device_out, dev_ptr);
    NRI_RESULT_SUCCESS
}

pub unsafe extern "win64" fn nriDestroyDevice(device: *mut NriDevice) {
    if !device.is_null() {
        drop(Box::from_raw(device));
    }
}

pub unsafe extern "win64" fn nriGetInterface(
    device: *const NriDevice,
    interface_name: *const u8,
    interface_size: usize,
    interface_out: *mut u8,
) -> u32 {
    ensure_interfaces_registered();
    if device.is_null() || interface_name.is_null() || interface_out.is_null() {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    ptr::write_bytes(interface_out, 0, interface_size);
    let name = CStr::from_ptr(interface_name as *const std::ffi::c_char).to_bytes();
    let (src_ptr, src_size) = match name {
        b"nri::CoreInterface" | b"CoreInterface" => (
            CORE_INTERFACE.0.as_ptr() as *const u8,
            CORE_INTERFACE.0.len() * size_of::<crate::Ptr>(),
        ),
        b"nri::HelperInterface" | b"HelperInterface" => (
            HELPER_INTERFACE.0.as_ptr() as *const u8,
            HELPER_INTERFACE.0.len() * size_of::<crate::Ptr>(),
        ),
        b"nri::WrapperD3D12Interface" | b"WrapperD3D12Interface" => (
            WRAPPER_D3D12_INTERFACE.0.as_ptr() as *const u8,
            WRAPPER_D3D12_INTERFACE.0.len() * size_of::<crate::Ptr>(),
        ),
        _ => {
            log::debug!(
                target: "d3d",
                "nriGetInterface: unsupported interface {:?} -> UNSUPPORTED",
                String::from_utf8_lossy(name)
            );
            return NRI_RESULT_UNSUPPORTED;
        }
    };
    if interface_size < src_size {
        return NRI_RESULT_INVALID_ARGUMENT;
    }
    ptr::copy_nonoverlapping(src_ptr, interface_out, src_size);
    NRI_RESULT_SUCCESS
}

pub mod exports {
    pub use super::{nriCreateDeviceFromD3D12Device, nriDestroyDevice, nriGetInterface};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::d3d12::vtables::IID_ID3D12Device;

    unsafe fn make_test_d3d12_device() -> Option<*mut c_void> {
        let mut d3d_dev: *mut c_void = ptr::null_mut();
        let hr = crate::d3d12::device::create_device(
            ptr::null_mut(),
            d3d12_metal::caps::FL_12_1,
            &IID_ID3D12Device,
            &mut d3d_dev,
        );
        if hr >= 0 && !d3d_dev.is_null() {
            Some(d3d_dev)
        } else {
            None
        }
    }

    unsafe fn make_nri_device(d3d_dev: *mut c_void) -> (*mut NriDevice, [usize; 106], [usize; 5], [usize; 5]) {
        let desc = NriDeviceCreationD3D12Desc {
            d3d12_device: d3d_dev,
            d3d12_graphics_queue: ptr::null_mut(),
            d3d12_compute_queue: ptr::null_mut(),
            d3d12_copy_queue: ptr::null_mut(),
            callback_interface: [0; 3],
            memory_allocator_interface: [0; 4],
            enable_nri_validation: 0,
            _pad: [0; 7],
        };
        let mut dev: *mut NriDevice = ptr::null_mut();
        assert_eq!(
            nriCreateDeviceFromD3D12Device(&desc as *const _ as *const u8, &mut dev),
            NRI_RESULT_SUCCESS
        );
        assert!(!dev.is_null());

        let mut core = [0usize; 106];
        assert_eq!(
            nriGetInterface(dev, b"nri::CoreInterface\0".as_ptr(), 0x350, core.as_mut_ptr() as *mut u8),
            NRI_RESULT_SUCCESS
        );
        assert!(core.iter().all(|&p| p != 0));

        let mut helper = [0usize; 5];
        assert_eq!(
            nriGetInterface(dev, b"nri::HelperInterface\0".as_ptr(), 0x28, helper.as_mut_ptr() as *mut u8),
            NRI_RESULT_SUCCESS
        );
        assert!(helper.iter().all(|&p| p != 0));

        let mut wrapper = [0usize; 5];
        assert_eq!(
            nriGetInterface(dev, b"nri::WrapperD3D12Interface\0".as_ptr(), 0x28, wrapper.as_mut_ptr() as *mut u8),
            NRI_RESULT_SUCCESS
        );
        assert!(wrapper.iter().all(|&p| p != 0));

        (dev, core, helper, wrapper)
    }

    #[test]
    fn nri_rejects_null_and_unsupported_interfaces() {
        unsafe {
            let mut dev: *mut NriDevice = ptr::null_mut();
            assert_eq!(nriCreateDeviceFromD3D12Device(ptr::null(), &mut dev), NRI_RESULT_INVALID_ARGUMENT);
            let zero_desc = [0u8; 0x60];
            assert_eq!(nriCreateDeviceFromD3D12Device(zero_desc.as_ptr(), &mut dev), NRI_RESULT_INVALID_ARGUMENT);

            let Some(d3d_dev) = make_test_d3d12_device() else { return };
            let (dev, _core, _helper, wrapper) = make_nri_device(d3d_dev);

            let mut unknown_iface = [0u8; 0x40];
            assert_eq!(
                nriGetInterface(dev, b"nri::SwapChainInterface\0".as_ptr(), 0x40, unknown_iface.as_mut_ptr()),
                NRI_RESULT_UNSUPPORTED
            );

            // WrapperD3D12 rejects null descriptor / null resource.
            let create_tex_d3d: unsafe extern "win64" fn(*mut NriDevice, *const NriTextureD3D12Desc, *mut *mut NriTexture) -> u32 =
                std::mem::transmute(wrapper[2]);
            let mut tex: *mut NriTexture = ptr::null_mut();
            assert_eq!(create_tex_d3d(dev, ptr::null(), &mut tex), NRI_RESULT_INVALID_ARGUMENT);
            assert!(tex.is_null());

            nriDestroyDevice(dev);
            com_release(d3d_dev);
        }
    }

    #[test]
    fn nri_real_device_desc_and_format_support() {
        unsafe {
            let Some(d3d_dev) = make_test_d3d12_device() else { return };
            let (dev, core, _helper, _wrapper) = make_nri_device(d3d_dev);

            let get_desc: unsafe extern "win64" fn(*const NriDevice) -> *const NriDeviceDesc =
                std::mem::transmute(core[0]);
            let desc = get_desc(dev);
            assert!(!desc.is_null());
            assert_eq!((*desc).graphics_api, NRI_GRAPHICS_API_D3D12);
            assert_eq!((*desc).nri_version_major, 1);
            assert_eq!((*desc).nri_version_minor, 99);
            assert_eq!((*desc).constant_buffer_offset_alignment, 256);
            assert!((*desc).adapter_desc.video_memory_size > 0);
            assert!((*desc).timestamp_frequency_hz > 0);

            let get_native_dev: unsafe extern "win64" fn(*const NriDevice) -> *mut c_void =
                std::mem::transmute(core[101]);
            assert_eq!(get_native_dev(dev), d3d_dev);

            let get_fmt: unsafe extern "win64" fn(*const NriDevice, u8) -> u8 =
                std::mem::transmute(core[1]);
            // RGBA8_UNORM (11): texture + storage + color attachment.
            let rgba8 = get_fmt(dev, 11);
            assert_ne!(rgba8 & (1 << 0), 0);
            assert_ne!(rgba8 & (1 << 4), 0);
            // D32_SFLOAT (63): depth-stencil attachment.
            let d32 = get_fmt(dev, 63);
            assert_ne!(d32 & (1 << 5), 0);
            // UNKNOWN (0): unsupported.
            assert_eq!(get_fmt(dev, 0), 0);

            nriDestroyDevice(dev);
            com_release(d3d_dev);
        }
    }

    #[test]
    fn nri_buffers_map_independently_without_aliasing() {
        unsafe {
            let Some(d3d_dev) = make_test_d3d12_device() else { return };
            let (dev, core, helper, _wrapper) = make_nri_device(d3d_dev);

            let create_buf: unsafe extern "win64" fn(*mut NriDevice, *const NriBufferDesc, *mut *mut NriBuffer) -> u32 =
                std::mem::transmute(core[9]);
            let destroy_buf: unsafe extern "win64" fn(*mut NriBuffer) = std::mem::transmute(core[25]);
            let free_mem: unsafe extern "win64" fn(*mut NriMemory) = std::mem::transmute(core[36]);
            let map_buf: unsafe extern "win64" fn(*mut NriBuffer, u64, u64) -> *mut u8 =
                std::mem::transmute(core[84]);
            let unmap_buf: unsafe extern "win64" fn(*mut NriBuffer) = std::mem::transmute(core[85]);
            let get_buf_native: unsafe extern "win64" fn(*const NriBuffer, u32) -> u64 =
                std::mem::transmute(core[103]);
            let calc_alloc: unsafe extern "win64" fn(*mut NriDevice, *const NriResourceGroupDesc) -> u32 =
                std::mem::transmute(helper[0]);
            let alloc_bind: unsafe extern "win64" fn(*mut NriDevice, *const NriResourceGroupDesc, *mut *mut NriMemory) -> u32 =
                std::mem::transmute(helper[1]);

            let bdesc = NriBufferDesc {
                size: 1024,
                structure_stride: 0,
                usage_mask: 1 << 4, // CONSTANT_BUFFER
                _pad0: 0,
                physical_device_mask: 0,
                _pad1: 0,
            };
            let mut b1: *mut NriBuffer = ptr::null_mut();
            let mut b2: *mut NriBuffer = ptr::null_mut();
            assert_eq!(create_buf(dev, &bdesc, &mut b1), NRI_RESULT_SUCCESS);
            assert_eq!(create_buf(dev, &bdesc, &mut b2), NRI_RESULT_SUCCESS);

            let bufs = [b1, b2];
            let rg = NriResourceGroupDesc {
                memory_location: 1, // HOST_UPLOAD
                _pad0: [0; 7],
                textures: ptr::null(),
                texture_num: 0,
                _pad1: 0,
                buffers: bufs.as_ptr(),
                buffer_num: 2,
                _pad2: 0,
            };
            let n_allocs = calc_alloc(dev, &rg);
            assert_eq!(n_allocs, 1);
            let mut mems = vec![ptr::null_mut(); n_allocs as usize];
            assert_eq!(alloc_bind(dev, &rg, mems.as_mut_ptr()), NRI_RESULT_SUCCESS);
            assert_ne!(get_buf_native(b1, 0), 0);
            assert_ne!(get_buf_native(b2, 0), 0);
            assert_ne!(get_buf_native(b1, 0), get_buf_native(b2, 0));

            let p1 = map_buf(b1, 0, 1024);
            let p2 = map_buf(b2, 0, 1024);
            assert!(!p1.is_null());
            assert!(!p2.is_null());
            assert_ne!(p1, p2, "two mapped NRI buffers must NOT alias");

            ptr::write_bytes(p1, 0x11, 1024);
            ptr::write_bytes(p2, 0x22, 1024);
            assert_eq!(*p1, 0x11);
            assert_eq!(*p1.add(1023), 0x11);
            assert_eq!(*p2, 0x22);
            assert_eq!(*p2.add(1023), 0x22);

            unmap_buf(b1);
            unmap_buf(b2);
            destroy_buf(b1);
            destroy_buf(b2);
            for m in mems {
                free_mem(m);
            }
            nriDestroyDevice(dev);
            com_release(d3d_dev);
        }
    }

    #[test]
    fn nri_fence_signal_and_gpu_copy_roundtrip() {
        unsafe {
            let Some(d3d_dev) = make_test_d3d12_device() else { return };
            let (dev, core, helper, _wrapper) = make_nri_device(d3d_dev);

            let get_queue: unsafe extern "win64" fn(*mut NriDevice, u8, *mut *mut NriCommandQueue) -> u32 =
                std::mem::transmute(core[5]);
            let create_alloc: unsafe extern "win64" fn(*const NriCommandQueue, u32, *mut *mut NriCommandAllocator) -> u32 =
                std::mem::transmute(core[6]);
            let create_cmd: unsafe extern "win64" fn(*mut NriCommandAllocator, *mut *mut NriCommandBuffer) -> u32 =
                std::mem::transmute(core[7]);
            let create_buf: unsafe extern "win64" fn(*mut NriDevice, *const NriBufferDesc, *mut *mut NriBuffer) -> u32 =
                std::mem::transmute(core[9]);
            let create_fence: unsafe extern "win64" fn(*mut NriDevice, u64, *mut *mut NriFence) -> u32 =
                std::mem::transmute(core[21]);
            let destroy_alloc: unsafe extern "win64" fn(*mut NriCommandAllocator) = std::mem::transmute(core[22]);
            let destroy_cmd: unsafe extern "win64" fn(*mut NriCommandBuffer) = std::mem::transmute(core[23]);
            let destroy_buf: unsafe extern "win64" fn(*mut NriBuffer) = std::mem::transmute(core[25]);
            let destroy_fence: unsafe extern "win64" fn(*mut NriFence) = std::mem::transmute(core[32]);
            let free_mem: unsafe extern "win64" fn(*mut NriMemory) = std::mem::transmute(core[36]);
            let begin_cmd: unsafe extern "win64" fn(*mut NriCommandBuffer, *const NriDescriptorPool, u32) -> u32 =
                std::mem::transmute(core[37]);
            let end_cmd: unsafe extern "win64" fn(*mut NriCommandBuffer) -> u32 = std::mem::transmute(core[38]);
            let copy_buf: unsafe extern "win64" fn(*mut NriCommandBuffer, *mut NriBuffer, u32, u64, *const NriBuffer, u32, u64, u64) =
                std::mem::transmute(core[67]);
            let get_fence_val: unsafe extern "win64" fn(*const NriFence) -> u64 = std::mem::transmute(core[73]);
            let queue_signal: unsafe extern "win64" fn(*mut NriCommandQueue, *mut NriFence, u64) =
                std::mem::transmute(core[74]);
            let wait_fence: unsafe extern "win64" fn(*mut NriFence, u64) = std::mem::transmute(core[76]);
            let queue_submit: unsafe extern "win64" fn(*mut NriCommandQueue, *const NriQueueSubmitDesc) =
                std::mem::transmute(core[77]);
            let map_buf: unsafe extern "win64" fn(*mut NriBuffer, u64, u64) -> *mut u8 =
                std::mem::transmute(core[84]);
            let unmap_buf: unsafe extern "win64" fn(*mut NriBuffer) = std::mem::transmute(core[85]);
            let alloc_bind: unsafe extern "win64" fn(*mut NriDevice, *const NriResourceGroupDesc, *mut *mut NriMemory) -> u32 =
                std::mem::transmute(helper[1]);
            let wait_idle: unsafe extern "win64" fn(*mut NriCommandQueue) -> u32 = std::mem::transmute(helper[4]);

            let mut queue: *mut NriCommandQueue = ptr::null_mut();
            assert_eq!(get_queue(dev, 0, &mut queue), NRI_RESULT_SUCCESS);

            // Fence initial value + signal/wait tracking.
            let mut fence: *mut NriFence = ptr::null_mut();
            assert_eq!(create_fence(dev, 7, &mut fence), NRI_RESULT_SUCCESS);
            assert_eq!(get_fence_val(fence), 7);

            // Create upload src + readback dst buffers and perform real GPU CopyBuffer.
            let bdesc = NriBufferDesc {
                size: 256,
                structure_stride: 0,
                usage_mask: 0,
                _pad0: 0,
                physical_device_mask: 0,
                _pad1: 0,
            };
            let mut src: *mut NriBuffer = ptr::null_mut();
            let mut dst: *mut NriBuffer = ptr::null_mut();
            assert_eq!(create_buf(dev, &bdesc, &mut src), NRI_RESULT_SUCCESS);
            assert_eq!(create_buf(dev, &bdesc, &mut dst), NRI_RESULT_SUCCESS);

            let mut src_mem: *mut NriMemory = ptr::null_mut();
            let mut dst_mem: *mut NriMemory = ptr::null_mut();
            let rg_src = NriResourceGroupDesc {
                memory_location: 1, // HOST_UPLOAD
                _pad0: [0; 7],
                textures: ptr::null(),
                texture_num: 0,
                _pad1: 0,
                buffers: &src,
                buffer_num: 1,
                _pad2: 0,
            };
            let rg_dst = NriResourceGroupDesc {
                memory_location: 2, // HOST_READBACK
                _pad0: [0; 7],
                textures: ptr::null(),
                texture_num: 0,
                _pad1: 0,
                buffers: &dst,
                buffer_num: 1,
                _pad2: 0,
            };
            assert_eq!(alloc_bind(dev, &rg_src, &mut src_mem), NRI_RESULT_SUCCESS);
            assert_eq!(alloc_bind(dev, &rg_dst, &mut dst_mem), NRI_RESULT_SUCCESS);

            let sp = map_buf(src, 0, 256);
            assert!(!sp.is_null());
            for i in 0..256usize {
                *sp.add(i) = (i as u8).wrapping_mul(3).wrapping_add(1);
            }
            unmap_buf(src);

            let mut alloc: *mut NriCommandAllocator = ptr::null_mut();
            let mut cmd: *mut NriCommandBuffer = ptr::null_mut();
            assert_eq!(create_alloc(queue, 0, &mut alloc), NRI_RESULT_SUCCESS);
            assert_eq!(create_cmd(alloc, &mut cmd), NRI_RESULT_SUCCESS);
            assert_eq!(begin_cmd(cmd, ptr::null(), 0), NRI_RESULT_SUCCESS);
            copy_buf(cmd, dst, 0, 0, src, 0, 0, 256);
            assert_eq!(end_cmd(cmd), NRI_RESULT_SUCCESS);

            let cb_ptr: *const NriCommandBuffer = cmd;
            let submit = NriQueueSubmitDesc {
                command_buffers: &cb_ptr,
                command_buffer_num: 1,
                physical_device_index: 0,
            };
            queue_submit(queue, &submit);
            queue_signal(queue, fence, 42);
            wait_fence(fence, 42);
            assert!(get_fence_val(fence) >= 42);
            assert_eq!(wait_idle(queue), NRI_RESULT_SUCCESS);

            let dp = map_buf(dst, 0, 256);
            assert!(!dp.is_null());
            for i in 0..256usize {
                assert_eq!(*dp.add(i), (i as u8).wrapping_mul(3).wrapping_add(1));
            }
            unmap_buf(dst);

            destroy_cmd(cmd);
            destroy_alloc(alloc);
            destroy_fence(fence);
            destroy_buf(src);
            destroy_buf(dst);
            free_mem(src_mem);
            free_mem(dst_mem);
            nriDestroyDevice(dev);
            com_release(d3d_dev);
        }
    }

    #[test]
    fn nri_texture_views_descriptors_and_pipeline_layout() {
        unsafe {
            let Some(d3d_dev) = make_test_d3d12_device() else { return };
            let (dev, core, helper, _wrapper) = make_nri_device(d3d_dev);

            let create_tex: unsafe extern "win64" fn(*mut NriDevice, *const NriTextureDesc, *mut *mut NriTexture) -> u32 =
                std::mem::transmute(core[10]);
            let create_tex_view: unsafe extern "win64" fn(*const NriTextureViewDesc, *mut *mut NriDescriptor) -> u32 =
                std::mem::transmute(core[13]);
            let create_sampler: unsafe extern "win64" fn(*mut NriDevice, *const NriSamplerDesc, *mut *mut NriDescriptor) -> u32 =
                std::mem::transmute(core[15]);
            let create_pool: unsafe extern "win64" fn(*mut NriDevice, *const NriDescriptorPoolDesc, *mut *mut NriDescriptorPool) -> u32 =
                std::mem::transmute(core[8]);
            let create_layout: unsafe extern "win64" fn(*mut NriDevice, *const NriPipelineLayoutDesc, *mut *mut NriPipelineLayout) -> u32 =
                std::mem::transmute(core[16]);
            let alloc_sets: unsafe extern "win64" fn(*mut NriDescriptorPool, *const NriPipelineLayout, u32, *mut *mut NriDescriptorSet, u32, u32, u32) -> u32 =
                std::mem::transmute(core[81]);
            let update_ranges: unsafe extern "win64" fn(*mut NriDescriptorSet, u32, u32, u32, *const NriDescriptorRangeUpdateDesc) =
                std::mem::transmute(core[78]);
            let destroy_pool: unsafe extern "win64" fn(*mut NriDescriptorPool) = std::mem::transmute(core[24]);
            let destroy_tex: unsafe extern "win64" fn(*mut NriTexture) = std::mem::transmute(core[26]);
            let destroy_desc: unsafe extern "win64" fn(*mut NriDescriptor) = std::mem::transmute(core[27]);
            let destroy_layout: unsafe extern "win64" fn(*mut NriPipelineLayout) = std::mem::transmute(core[28]);
            let free_mem: unsafe extern "win64" fn(*mut NriMemory) = std::mem::transmute(core[36]);
            let alloc_bind: unsafe extern "win64" fn(*mut NriDevice, *const NriResourceGroupDesc, *mut *mut NriMemory) -> u32 =
                std::mem::transmute(helper[1]);

            let tdesc = NriTextureDesc {
                texture_type: 1, // TEXTURE_2D
                _pad0: 0,
                usage_mask: (1 << 0) | (1 << 1), // SRV | UAV
                format: 11, // RGBA8_UNORM
                _pad1: 0,
                size: [64, 64, 1],
                mip_num: 1,
                array_size: 1,
                sample_num: 1,
                _pad2: [0; 3],
                physical_device_mask: 0,
            };
            let mut tex: *mut NriTexture = ptr::null_mut();
            assert_eq!(create_tex(dev, &tdesc, &mut tex), NRI_RESULT_SUCCESS);
            let mut tmem: *mut NriMemory = ptr::null_mut();
            let rg = NriResourceGroupDesc {
                memory_location: 0, // DEVICE
                _pad0: [0; 7],
                textures: &tex,
                texture_num: 1,
                _pad1: 0,
                buffers: ptr::null(),
                buffer_num: 0,
                _pad2: 0,
            };
            assert_eq!(alloc_bind(dev, &rg, &mut tmem), NRI_RESULT_SUCCESS);

            let vdesc = NriTextureViewDesc {
                texture: tex,
                view_type: 0, // SHADER_RESOURCE_2D
                format: 11,
                mip_offset: 0,
                mip_num: 1,
                array_or_slice_offset: 0,
                array_or_slice_size: 1,
                _pad0: 0,
                physical_device_mask: 0,
                flags: 0,
                _pad1: [0; 7],
            };
            let mut srv: *mut NriDescriptor = ptr::null_mut();
            assert_eq!(create_tex_view(&vdesc, &mut srv), NRI_RESULT_SUCCESS);
            assert!(!srv.is_null());

            let sdesc = NriSamplerDesc {
                magnification: 1,
                minification: 1,
                mip: 1,
                filter_ext: 0,
                anisotropy: 1,
                mip_bias: 0.0,
                mip_min: 0.0,
                mip_max: 16.0,
                address_u: 2,
                address_v: 2,
                address_w: 2,
                compare_func: 0,
                border_color: 0,
                unnormalized_coordinates: 0,
                _pad: [0; 2],
            };
            let mut sampler: *mut NriDescriptor = ptr::null_mut();
            assert_eq!(create_sampler(dev, &sdesc, &mut sampler), NRI_RESULT_SUCCESS);
            assert!(!sampler.is_null());

            let ranges = [
                NriDescriptorRangeDesc {
                    base_register_index: 0,
                    descriptor_num: 1,
                    descriptor_type: 2, // TEXTURE
                    visibility: 6,      // COMPUTE
                    is_descriptor_num_variable: 0,
                    is_array: 0,
                },
                NriDescriptorRangeDesc {
                    base_register_index: 0,
                    descriptor_num: 1,
                    descriptor_type: 0, // SAMPLER
                    visibility: 6,      // COMPUTE
                    is_descriptor_num_variable: 0,
                    is_array: 0,
                },
            ];
            let set_desc = NriDescriptorSetDesc {
                register_space: 0,
                _pad0: 0,
                ranges: ranges.as_ptr(),
                range_num: 2,
                _pad1: 0,
                dynamic_constant_buffers: ptr::null(),
                dynamic_constant_buffer_num: 0,
                binding_mask: 0,
                _pad2: [0; 3],
            };
            let pl_desc = NriPipelineLayoutDesc {
                descriptor_sets: &set_desc,
                push_constants: ptr::null(),
                descriptor_set_num: 1,
                push_constant_num: 0,
                stage_mask: 1 << 6, // COMPUTE
                ignore_global_spirv_offsets: 1,
                _pad: [0; 5],
            };
            let mut layout: *mut NriPipelineLayout = ptr::null_mut();
            assert_eq!(create_layout(dev, &pl_desc, &mut layout), NRI_RESULT_SUCCESS);
            assert!(!layout.is_null());

            let pool_desc = NriDescriptorPoolDesc {
                descriptor_set_max_num: 2,
                sampler_max_num: 4,
                texture_max_num: 4,
                ..Default::default()
            };
            let mut pool: *mut NriDescriptorPool = ptr::null_mut();
            assert_eq!(create_pool(dev, &pool_desc, &mut pool), NRI_RESULT_SUCCESS);

            let mut set: *mut NriDescriptorSet = ptr::null_mut();
            assert_eq!(alloc_sets(pool, layout, 0, &mut set, 1, 0, 0), NRI_RESULT_SUCCESS);
            assert!(!set.is_null());

            let srv_arr: [*const NriDescriptor; 1] = [srv];
            let samp_arr: [*const NriDescriptor; 1] = [sampler];
            let updates = [
                NriDescriptorRangeUpdateDesc {
                    descriptors: srv_arr.as_ptr(),
                    descriptor_num: 1,
                    offset_in_range: 0,
                },
                NriDescriptorRangeUpdateDesc {
                    descriptors: samp_arr.as_ptr(),
                    descriptor_num: 1,
                    offset_in_range: 0,
                },
            ];
            update_ranges(set, 0, 0, 2, updates.as_ptr());

            destroy_pool(pool);
            destroy_layout(layout);
            destroy_desc(srv);
            destroy_desc(sampler);
            destroy_tex(tex);
            free_mem(tmem);
            nriDestroyDevice(dev);
            com_release(d3d_dev);
        }
    }

    #[test]
    fn nri_mesh_and_task_pipeline_stream_encoding_and_device2_dispatch() {
        // Minimal mock COM objects verifying exact D3D12_PIPELINE_STATE_STREAM_DESC
        // encoding and ID3D12Device2::CreatePipelineState (vtable slot 47) routing.
        #[repr(C)]
        struct MockPso {
            vtbl: *const [usize; 3],
            released: std::sync::atomic::AtomicBool,
        }

        unsafe extern "win64" fn mock_pso_qi(
            _this: *mut c_void,
            _riid: *const Guid,
            out: *mut *mut c_void,
        ) -> HRESULT {
            if !out.is_null() {
                *out = ptr::null_mut();
            }
            0x8000_4002u32 as i32 // E_NOINTERFACE
        }
        unsafe extern "win64" fn mock_pso_addref(_this: *mut c_void) -> u32 {
            1
        }
        unsafe extern "win64" fn mock_pso_release(this: *mut c_void) -> u32 {
            if !this.is_null() {
                let p = &*(this as *const MockPso);
                p.released.store(true, std::sync::atomic::Ordering::Release);
            }
            0
        }
        #[repr(C)]
        struct MockDev2 {
            vtbl: *const [usize; 48],
            captured_stream: Vec<u8>,
            out_pso: *mut MockPso,
            qi_device2_called: bool,
            create_pso_stream_called: bool,
        }

        unsafe extern "win64" fn mock_dev_qi(
            this: *mut c_void,
            riid: *const Guid,
            out: *mut *mut c_void,
        ) -> HRESULT {
            let d = &mut *(this as *mut MockDev2);
            if !riid.is_null() && ptr::read_unaligned(riid) == IID_ID3D12Device2 {
                d.qi_device2_called = true;
                ptr::write_unaligned(out, this);
                0 // S_OK
            } else {
                ptr::write_unaligned(out, ptr::null_mut());
                0x8000_4002u32 as i32
            }
        }
        unsafe extern "win64" fn mock_dev_addref(_this: *mut c_void) -> u32 {
            2
        }
        unsafe extern "win64" fn mock_dev_release(_this: *mut c_void) -> u32 {
            1
        }
        unsafe extern "win64" fn mock_dev_create_pipeline_state(
            this: *mut c_void,
            desc: *const StreamDesc,
            riid: *const Guid,
            out: *mut *mut c_void,
        ) -> HRESULT {
            let d = &mut *(this as *mut MockDev2);
            d.create_pso_stream_called = true;
            assert!(!desc.is_null());
            assert!(!riid.is_null() && ptr::read_unaligned(riid) == IID_ID3D12PipelineState);
            let sd = ptr::read_unaligned(desc);
            assert!(!sd.stream.is_null() && sd.size > 0);
            assert_eq!(sd.stream as usize % 8, 0, "stream must be 8-byte pointer aligned");
            d.captured_stream = std::slice::from_raw_parts(sd.stream, sd.size).to_vec();
            ptr::write_unaligned(out, d.out_pso as *mut c_void);
            0 // S_OK
        }

        unsafe {
            let mock_pso_vtbl: [usize; 3] = [
                mock_pso_qi as *const () as usize,
                mock_pso_addref as *const () as usize,
                mock_pso_release as *const () as usize,
            ];
            let mut vtbl = [0usize; 48];
            vtbl[0] = mock_dev_qi as *const () as usize;
            vtbl[1] = mock_dev_addref as *const () as usize;
            vtbl[2] = mock_dev_release as *const () as usize;
            vtbl[47] = mock_dev_create_pipeline_state as *const () as usize;

            let mut mock_pso = MockPso {
                vtbl: &mock_pso_vtbl,
                released: std::sync::atomic::AtomicBool::new(false),
            };
            let mut mock_dev = MockDev2 {
                vtbl: &vtbl,
                captured_stream: Vec::new(),
                out_pso: &mut mock_pso,
                qi_device2_called: false,
                create_pso_stream_called: false,
            };

            let fake_rs = 0x1234_5678_9ABC_DEF0usize as *mut c_void;
            let layout = std::mem::ManuallyDrop::new(NriPipelineLayout {
                d3d12_root_sig: fake_rs,
                is_graphics: true,
                set_mappings: Vec::new(),
                set_root_offsets: Vec::new(),
                dynamic_cbv_mappings: Vec::new(),
                push_constants_base_index: 0,
            });
            let mut nri_dev = std::mem::ManuallyDrop::new(NriDevice {
                d3d12_device: (&mut mock_dev as *mut MockDev2) as *mut c_void,
                desc: std::mem::zeroed(),
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
            });

            let task_bc = [0x11u8; 32];
            let mesh_bc = [0x22u8; 48];
            let frag_bc = [0x33u8; 24];
            let stages = [
                NriShaderDesc {
                    stage: 13, // MESH_CONTROL / TASK (AS)
                    _pad0: [0; 7],
                    bytecode: task_bc.as_ptr(),
                    size: task_bc.len() as u64,
                    entry_point_name: ptr::null(),
                },
                NriShaderDesc {
                    stage: 14, // MESH_EVALUATION / MESH (MS)
                    _pad0: [0; 7],
                    bytecode: mesh_bc.as_ptr(),
                    size: mesh_bc.len() as u64,
                    entry_point_name: ptr::null(),
                },
                NriShaderDesc {
                    stage: 5, // FRAGMENT (PS)
                    _pad0: [0; 7],
                    bytecode: frag_bc.as_ptr(),
                    size: frag_bc.len() as u64,
                    entry_point_name: ptr::null(),
                },
            ];
            let color_atts = [
                NriColorAttachmentDesc {
                    format: 11, // RGBA8_UNORM -> DXGI 28
                    color_blend: NriBlendingDesc { src_factor: 6, dst_factor: 7, func: 0 },
                    alpha_blend: NriBlendingDesc { src_factor: 1, dst_factor: 0, func: 0 },
                    color_write_mask: 0xF,
                    blend_enabled: 1,
                },
                NriColorAttachmentDesc {
                    format: 30, // RGBA16_SFLOAT -> DXGI 10
                    color_blend: NriBlendingDesc { src_factor: 1, dst_factor: 0, func: 0 },
                    alpha_blend: NriBlendingDesc { src_factor: 1, dst_factor: 0, func: 0 },
                    color_write_mask: 0x7,
                    blend_enabled: 0,
                },
            ];
            let om = NriOutputMergerDesc {
                color: color_atts.as_ptr(),
                depth: NriDepthAttachmentDesc { compare_func: 3, write: 1 }, // LESS, write=1
                stencil: NriStencilAttachmentDesc {
                    front: NriStencilDesc { compare_func: 0, fail: 0, pass: 0, depth_fail: 0 },
                    back: NriStencilDesc { compare_func: 0, fail: 0, pass: 0, depth_fail: 0 },
                    reference: 0,
                    compare_mask: 0xFF,
                    write_mask: 0xFF,
                },
                depth_stencil_format: 63, // D32_SFLOAT -> DXGI 40
                color_logic_func: 0,
                _pad0: 0,
                color_num: 2,
                blend_consts: [0.25, 0.5, 0.75, 1.0],
                _pad1: 0,
            };
            let rast = NriRasterizationDesc {
                viewport_num: 1,
                depth_bias_constant_factor: 2,
                depth_bias_clamp: 0.0,
                depth_bias_slope_factor: 1.5,
                fill_mode: 0, // SOLID
                cull_mode: 2, // BACK
                sample_mask: 0xFFFF,
                sample_num: 1,
                alpha_to_coverage: 0,
                front_counter_clockwise: 1,
                depth_clamp: 1,
                antialiased_lines: 0,
                rasterizer_discard: 0,
                conservative_rasterization: 0,
                _pad: 0,
            };
            let gd = NriGraphicsPipelineDesc {
                pipeline_layout: &*layout as *const NriPipelineLayout,
                input_assembly: ptr::null(),
                rasterization: &rast,
                output_merger: &om,
                shader_stages: stages.as_ptr(),
                shader_stage_num: stages.len() as u32,
                _pad: 0,
            };

            let mut pipeline: *mut NriPipeline = ptr::null_mut();
            let res = core_create_graphics_pipeline(&mut *nri_dev as *mut NriDevice, &gd, &mut pipeline);
            assert_eq!(res, NRI_RESULT_SUCCESS);
            assert!(!pipeline.is_null());
            assert!(mock_dev.qi_device2_called);
            assert!(mock_dev.create_pso_stream_called);
            assert!((*pipeline).is_graphics);
            assert!((*pipeline).blend_enabled);
            assert_eq!((*pipeline).blend_factor, [0.25, 0.5, 0.75, 1.0]);

            // Parse captured stream using exact D3D12_PIPELINE_STATE_STREAM_DESC layout rules.
            let bytes = &mock_dev.captured_stream;
            let mut off = 0usize;
            let mut seen_rs = 0usize;
            let mut seen_as: Option<(usize, usize)> = None;
            let mut seen_ms: Option<(usize, usize)> = None;
            let mut seen_ps: Option<(usize, usize)> = None;
            let mut seen_rtvs: Vec<u32> = Vec::new();
            let mut seen_dsv = 0u32;
            let mut seen_sample_count = 0u32;
            let mut seen_blend = false;
            let mut seen_raster = false;
            let mut seen_ds = false;
            while off + 4 <= bytes.len() {
                let ty = u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
                let (al, sz) = match ty {
                    0 => (8, 8),
                    1..=6 | 24 | 25 => (8, 16),
                    8 => (4, 328),
                    9 | 13 | 14 | 16 | 18 => (4, 4),
                    10 => (4, 44),
                    11 => (4, 52),
                    15 => (4, 36),
                    17 => (4, 8),
                    _ => panic!("unexpected subobject type {ty} at offset {off}"),
                };
                let i = (off + 4).next_multiple_of(al);
                assert!(i + sz <= bytes.len());
                let q = &bytes[i..i + sz];
                match ty {
                    0 => seen_rs = u64::from_le_bytes(q[0..8].try_into().unwrap()) as usize,
                    2 => {
                        let p = u64::from_le_bytes(q[0..8].try_into().unwrap()) as usize;
                        let n = u64::from_le_bytes(q[8..16].try_into().unwrap()) as usize;
                        seen_ps = Some((p, n));
                    }
                    8 => seen_blend = true,
                    10 => seen_raster = true,
                    11 => seen_ds = true,
                    15 => {
                        let n = u32::from_le_bytes(q[32..36].try_into().unwrap()) as usize;
                        seen_rtvs = (0..n)
                            .map(|k| u32::from_le_bytes(q[4 * k..4 * k + 4].try_into().unwrap()))
                            .collect();
                    }
                    16 => seen_dsv = u32::from_le_bytes(q[0..4].try_into().unwrap()),
                    17 => seen_sample_count = u32::from_le_bytes(q[0..4].try_into().unwrap()),
                    24 => {
                        let p = u64::from_le_bytes(q[0..8].try_into().unwrap()) as usize;
                        let n = u64::from_le_bytes(q[8..16].try_into().unwrap()) as usize;
                        seen_as = Some((p, n));
                    }
                    25 => {
                        let p = u64::from_le_bytes(q[0..8].try_into().unwrap()) as usize;
                        let n = u64::from_le_bytes(q[8..16].try_into().unwrap()) as usize;
                        seen_ms = Some((p, n));
                    }
                    _ => {}
                }
                off = (i + sz).next_multiple_of(8);
            }
            assert_eq!(seen_rs, fake_rs as usize);
            assert_eq!(seen_as, Some((task_bc.as_ptr() as usize, task_bc.len())));
            assert_eq!(seen_ms, Some((mesh_bc.as_ptr() as usize, mesh_bc.len())));
            assert_eq!(seen_ps, Some((frag_bc.as_ptr() as usize, frag_bc.len())));
            assert_eq!(seen_rtvs, vec![28, 10]);
            assert_eq!(seen_dsv, 40);
            assert_eq!(seen_sample_count, 1);
            assert!(seen_blend && seen_raster && seen_ds);

            core_destroy_pipeline(pipeline);
            assert!(mock_pso.released.load(std::sync::atomic::Ordering::Acquire));
        }
    }

    #[test]
    fn nri_mesh_and_task_pipeline_on_real_d3d12_device() {
        const AS_RAW: &[u8] = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../d3d12-metal/tests/fixtures/mesh.as.dxil"
        ));
        const MS_RAW: &[u8] = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../d3d12-metal/tests/fixtures/mesh.ms.dxil"
        ));
        const MS_DIRECT_RAW: &[u8] = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../d3d12-metal/tests/fixtures/mesh_direct.ms.dxil"
        ));
        const PS_RAW: &[u8] = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../d3d12-metal/tests/fixtures/mesh.ps.dxil"
        ));

        struct EnvGuard {
            key: &'static str,
            prev: Option<String>,
        }
        impl EnvGuard {
            fn set(key: &'static str, val: &str) -> Self {
                let prev = std::env::var(key).ok();
                std::env::set_var(key, val);
                Self { key, prev }
            }
        }
        impl Drop for EnvGuard {
            fn drop(&mut self) {
                match &self.prev {
                    Some(v) => std::env::set_var(self.key, v),
                    None => std::env::remove_var(self.key),
                }
            }
        }

        fn sign(raw: &[u8]) -> Vec<u8> {
            let mut v = raw.to_vec();
            dxil_metal::dxbc::sign_container(&mut v).unwrap();
            v
        }

        unsafe {
            let Some(d3d_dev) = make_test_d3d12_device() else { return };
            let _mesh_guard = EnvGuard::set("AW2_MESH_SHADERS", "1");
            let (dev, core, _helper, _wrapper) = make_nri_device(d3d_dev);

            let create_layout: unsafe extern "win64" fn(
                *mut NriDevice,
                *const NriPipelineLayoutDesc,
                *mut *mut NriPipelineLayout,
            ) -> u32 = std::mem::transmute(core[16]);
            let create_gfx: unsafe extern "win64" fn(
                *mut NriDevice,
                *const NriGraphicsPipelineDesc,
                *mut *mut NriPipeline,
            ) -> u32 = std::mem::transmute(core[17]);
            let destroy_layout: unsafe extern "win64" fn(*mut NriPipelineLayout) =
                std::mem::transmute(core[28]);
            let destroy_pipe: unsafe extern "win64" fn(*mut NriPipeline) =
                std::mem::transmute(core[29]);

            // Root constants matching mesh.hlsl fixture (4 uints at b0, space0).
            let pc = NriPushConstantDesc {
                register_index: 0,
                size: 16,
                visibility: 0, // ALL
                _pad: [0; 3],
            };
            let pl_desc = NriPipelineLayoutDesc {
                descriptor_sets: ptr::null(),
                push_constants: &pc,
                descriptor_set_num: 0,
                push_constant_num: 1,
                stage_mask: (1 << 13) | (1 << 14) | (1 << 5), // TASK | MESH | FRAGMENT
                ignore_global_spirv_offsets: 1,
                _pad: [0; 5],
            };
            let mut layout: *mut NriPipelineLayout = ptr::null_mut();
            assert_eq!(create_layout(dev, &pl_desc, &mut layout), NRI_RESULT_SUCCESS);
            assert!(!layout.is_null());

            let as_bc = sign(AS_RAW);
            let ms_bc = sign(MS_RAW);
            let ms_direct_bc = sign(MS_DIRECT_RAW);
            let ps_bc = sign(PS_RAW);

            let color_att = NriColorAttachmentDesc {
                format: 11, // RGBA8_UNORM
                color_blend: NriBlendingDesc { src_factor: 1, dst_factor: 0, func: 0 },
                alpha_blend: NriBlendingDesc { src_factor: 1, dst_factor: 0, func: 0 },
                color_write_mask: 0xF,
                blend_enabled: 0,
            };
            let om = NriOutputMergerDesc {
                color: &color_att,
                depth: NriDepthAttachmentDesc { compare_func: 0, write: 0 },
                stencil: NriStencilAttachmentDesc {
                    front: NriStencilDesc { compare_func: 0, fail: 0, pass: 0, depth_fail: 0 },
                    back: NriStencilDesc { compare_func: 0, fail: 0, pass: 0, depth_fail: 0 },
                    reference: 0,
                    compare_mask: 0xFF,
                    write_mask: 0xFF,
                },
                depth_stencil_format: 0,
                color_logic_func: 0,
                _pad0: 0,
                color_num: 1,
                blend_consts: [0.0; 4],
                _pad1: 0,
            };
            let rast = NriRasterizationDesc {
                viewport_num: 1,
                depth_bias_constant_factor: 0,
                depth_bias_clamp: 0.0,
                depth_bias_slope_factor: 0.0,
                fill_mode: 0,
                cull_mode: 0,
                sample_mask: 0xFFFF,
                sample_num: 1,
                alpha_to_coverage: 0,
                front_counter_clockwise: 0,
                depth_clamp: 1,
                antialiased_lines: 0,
                rasterizer_discard: 0,
                conservative_rasterization: 0,
                _pad: 0,
            };

            // 1. Amplification/Task shader without a Mesh shader must fail via D3D12 CreatePipelineState.
            let task_only = [NriShaderDesc {
                stage: 13,
                _pad0: [0; 7],
                bytecode: as_bc.as_ptr(),
                size: as_bc.len() as u64,
                entry_point_name: ptr::null(),
            }];
            let gd_invalid = NriGraphicsPipelineDesc {
                pipeline_layout: layout,
                input_assembly: ptr::null(),
                rasterization: &rast,
                output_merger: &om,
                shader_stages: task_only.as_ptr(),
                shader_stage_num: 1,
                _pad: 0,
            };
            let mut bad_pipe: *mut NriPipeline = ptr::null_mut();
            assert_eq!(create_gfx(dev, &gd_invalid, &mut bad_pipe), NRI_RESULT_INVALID_ARGUMENT);
            assert!(bad_pipe.is_null());

            // 2. Full AS + MS + PS and MS + PS compilation through real Metal 4 mesh pipelines
            //    when the Metal Shader Converter is installed on this host.
            if dxil_metal::converter_available() {
                let stages_as_ms_ps = [
                    NriShaderDesc {
                        stage: 13,
                        _pad0: [0; 7],
                        bytecode: as_bc.as_ptr(),
                        size: as_bc.len() as u64,
                        entry_point_name: ptr::null(),
                    },
                    NriShaderDesc {
                        stage: 14,
                        _pad0: [0; 7],
                        bytecode: ms_bc.as_ptr(),
                        size: ms_bc.len() as u64,
                        entry_point_name: ptr::null(),
                    },
                    NriShaderDesc {
                        stage: 5,
                        _pad0: [0; 7],
                        bytecode: ps_bc.as_ptr(),
                        size: ps_bc.len() as u64,
                        entry_point_name: ptr::null(),
                    },
                ];
                let gd_as_ms = NriGraphicsPipelineDesc {
                    pipeline_layout: layout,
                    input_assembly: ptr::null(),
                    rasterization: &rast,
                    output_merger: &om,
                    shader_stages: stages_as_ms_ps.as_ptr(),
                    shader_stage_num: stages_as_ms_ps.len() as u32,
                    _pad: 0,
                };
                let mut pipe_as_ms: *mut NriPipeline = ptr::null_mut();
                assert_eq!(create_gfx(dev, &gd_as_ms, &mut pipe_as_ms), NRI_RESULT_SUCCESS);
                assert!(!pipe_as_ms.is_null());
                let pso_ref = crate::d3d12::pso::pso_from_ptr((*pipe_as_ms).d3d12_pso).unwrap();
                assert!(matches!(pso_ref.kind, crate::d3d12::pso::PipelineKind::Mesh(_)));
                destroy_pipe(pipe_as_ms);

                let stages_ms_ps = [
                    NriShaderDesc {
                        stage: 14,
                        _pad0: [0; 7],
                        bytecode: ms_direct_bc.as_ptr(),
                        size: ms_direct_bc.len() as u64,
                        entry_point_name: ptr::null(),
                    },
                    NriShaderDesc {
                        stage: 5,
                        _pad0: [0; 7],
                        bytecode: ps_bc.as_ptr(),
                        size: ps_bc.len() as u64,
                        entry_point_name: ptr::null(),
                    },
                ];
                let gd_ms = NriGraphicsPipelineDesc {
                    pipeline_layout: layout,
                    input_assembly: ptr::null(),
                    rasterization: &rast,
                    output_merger: &om,
                    shader_stages: stages_ms_ps.as_ptr(),
                    shader_stage_num: stages_ms_ps.len() as u32,
                    _pad: 0,
                };
                let mut pipe_ms: *mut NriPipeline = ptr::null_mut();
                assert_eq!(create_gfx(dev, &gd_ms, &mut pipe_ms), NRI_RESULT_SUCCESS);
                assert!(!pipe_ms.is_null());
                let pso_ref2 = crate::d3d12::pso::pso_from_ptr((*pipe_ms).d3d12_pso).unwrap();
                assert!(matches!(pso_ref2.kind, crate::d3d12::pso::PipelineKind::Mesh(_)));
                destroy_pipe(pipe_ms);
            }

            destroy_layout(layout);
            nriDestroyDevice(dev);
            com_release(d3d_dev);
        }
    }
}
