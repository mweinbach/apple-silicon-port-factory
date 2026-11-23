//! Builtin `NRD.dll` (NVIDIA Real-Time Denoisers v4.3.6).
//!
//! Struct layouts, enums, names, pool sizing, and validation semantics follow
//! `third_party/nrd/` (`Include/NRD.h`, `Include/NRDDescs.h`,
//! `Include/NRDSettings.h`, `Source/Wrapper.cpp`, `Source/InstanceImpl.cpp`).
//!
//! Embeds seven signed DXBC+DXIL `cs_6_0` compute shaders (`[numthreads(8, 8, 1)]`,
//! entry point `"main"`) compiled offline via DXC:
//! * `Clear_f.cs` / `Clear_ui.cs`: permanent & transient pool clears on `CLEAR_AND_RESTART`
//! * `REBLUR_Filter.cs`: temporal reprojection + edge-aware spatial filter for Reblur modes
//! * `RELAX_Filter.cs`: temporal accumulation + multi-pass À-Trous wavelet filter for Relax modes
//! * `SIGMA_Shadow.cs`: penumbra-guided spatial blur + temporal stabilization for Sigma modes
//! * `REFERENCE_Accumulate.cs`: progressive Monte Carlo reference accumulator
//! * `SpecularMv_Compute.cs`: specular reflection & delta motion vector generator

use std::mem::{offset_of, size_of};
use std::ptr;

pub static ORDINALS: &[(u16, &str)] = &[];
pub static EXTRA_EXPORTS: &[crate::Export] = &[];

// ---------------------------------------------------------------------------
// NRD enums & constants (`third_party/nrd/Include/NRDDescs.h`)
// ---------------------------------------------------------------------------

pub const NRD_RESULT_SUCCESS: u32 = 0;
pub const NRD_RESULT_FAILURE: u32 = 1;
pub const NRD_RESULT_INVALID_ARGUMENT: u32 = 2;
pub const NRD_RESULT_UNSUPPORTED: u32 = 3;
pub const NRD_RESULT_NON_UNIQUE_IDENTIFIER: u32 = 4;

pub const NRD_DENOISER_MAX_NUM: usize = 21;
pub const NRD_RESOURCE_TYPE_MAX_NUM: usize = 37;

pub const NRD_ACCUMULATION_CONTINUE: u8 = 0;
pub const NRD_ACCUMULATION_RESTART: u8 = 1;
pub const NRD_ACCUMULATION_CLEAR_AND_RESTART: u8 = 2;

// `nrd::Denoiser`
pub const NRD_DENOISER_REBLUR_DIFFUSE: u32 = 0;
pub const NRD_DENOISER_REBLUR_DIFFUSE_OCCLUSION: u32 = 1;
pub const NRD_DENOISER_REBLUR_DIFFUSE_SH: u32 = 2;
pub const NRD_DENOISER_REBLUR_SPECULAR: u32 = 3;
pub const NRD_DENOISER_REBLUR_SPECULAR_OCCLUSION: u32 = 4;
pub const NRD_DENOISER_REBLUR_SPECULAR_SH: u32 = 5;
pub const NRD_DENOISER_REBLUR_DIFFUSE_SPECULAR: u32 = 6;
pub const NRD_DENOISER_REBLUR_DIFFUSE_SPECULAR_OCCLUSION: u32 = 7;
pub const NRD_DENOISER_REBLUR_DIFFUSE_SPECULAR_SH: u32 = 8;
pub const NRD_DENOISER_REBLUR_DIFFUSE_DIRECTIONAL_OCCLUSION: u32 = 9;
pub const NRD_DENOISER_SIGMA_SHADOW: u32 = 10;
pub const NRD_DENOISER_SIGMA_SHADOW_TRANSLUCENCY: u32 = 11;
pub const NRD_DENOISER_RELAX_DIFFUSE: u32 = 12;
pub const NRD_DENOISER_RELAX_DIFFUSE_SH: u32 = 13;
pub const NRD_DENOISER_RELAX_SPECULAR: u32 = 14;
pub const NRD_DENOISER_RELAX_SPECULAR_SH: u32 = 15;
pub const NRD_DENOISER_RELAX_DIFFUSE_SPECULAR: u32 = 16;
pub const NRD_DENOISER_RELAX_DIFFUSE_SPECULAR_SH: u32 = 17;
pub const NRD_DENOISER_REFERENCE: u32 = 18;
pub const NRD_DENOISER_SPECULAR_REFLECTION_MV: u32 = 19;
pub const NRD_DENOISER_SPECULAR_DELTA_MV: u32 = 20;

// `nrd::DescriptorType`
pub const NRD_DESCRIPTOR_TEXTURE: u32 = 0;
pub const NRD_DESCRIPTOR_STORAGE_TEXTURE: u32 = 1;

// `nrd::ResourceType`
pub const NRD_RES_IN_MV: u32 = 0;
pub const NRD_RES_IN_NORMAL_ROUGHNESS: u32 = 1;
pub const NRD_RES_IN_VIEWZ: u32 = 2;
pub const NRD_RES_IN_DIFF_RADIANCE_HITDIST: u32 = 3;
pub const NRD_RES_IN_SPEC_RADIANCE_HITDIST: u32 = 4;
pub const NRD_RES_IN_DIFF_HITDIST: u32 = 5;
pub const NRD_RES_IN_SPEC_HITDIST: u32 = 6;
pub const NRD_RES_IN_DIFF_DIRECTION_HITDIST: u32 = 7;
pub const NRD_RES_IN_DIFF_SH0: u32 = 8;
pub const NRD_RES_IN_DIFF_SH1: u32 = 9;
pub const NRD_RES_IN_SPEC_SH0: u32 = 10;
pub const NRD_RES_IN_SPEC_SH1: u32 = 11;
pub const NRD_RES_IN_SHADOWDATA: u32 = 16;
pub const NRD_RES_IN_SHADOW_TRANSLUCENCY: u32 = 17;
pub const NRD_RES_IN_RADIANCE: u32 = 18;
pub const NRD_RES_IN_DELTA_PRIMARY_POS: u32 = 19;
pub const NRD_RES_IN_DELTA_SECONDARY_POS: u32 = 20;
pub const NRD_RES_OUT_DIFF_RADIANCE_HITDIST: u32 = 21;
pub const NRD_RES_OUT_SPEC_RADIANCE_HITDIST: u32 = 22;
pub const NRD_RES_OUT_DIFF_SH0: u32 = 23;
pub const NRD_RES_OUT_DIFF_SH1: u32 = 24;
pub const NRD_RES_OUT_SPEC_SH0: u32 = 25;
pub const NRD_RES_OUT_SPEC_SH1: u32 = 26;
pub const NRD_RES_OUT_DIFF_HITDIST: u32 = 27;
pub const NRD_RES_OUT_SPEC_HITDIST: u32 = 28;
pub const NRD_RES_OUT_DIFF_DIRECTION_HITDIST: u32 = 29;
pub const NRD_RES_OUT_SHADOW_TRANSLUCENCY: u32 = 30;
pub const NRD_RES_OUT_RADIANCE: u32 = 31;
pub const NRD_RES_OUT_REFLECTION_MV: u32 = 32;
pub const NRD_RES_OUT_DELTA_MV: u32 = 33;
pub const NRD_RES_TRANSIENT_POOL: u32 = 35;
pub const NRD_RES_PERMANENT_POOL: u32 = 36;

// `nrd::Format`
pub const NRD_FORMAT_R16_UNORM: u32 = 13;
pub const NRD_FORMAT_R16_UINT: u32 = 15;
pub const NRD_FORMAT_RG16_SFLOAT: u32 = 22;
pub const NRD_FORMAT_RGBA16_SNORM: u32 = 24;
pub const NRD_FORMAT_RGBA16_SFLOAT: u32 = 27;
pub const NRD_FORMAT_RGBA32_SFLOAT: u32 = 39;

const RELAX_MAX_ATROUS_PASSES: u32 = 5;
const THREAD_GROUP_DIM: u32 = 8;

/// Matches `g_IsIntegerFormat` in `third_party/nrd/Source/InstanceImpl.cpp`.
const fn is_integer_format(format: u32) -> bool {
    matches!(format, 2 | 6 | 10 | 15 | 20 | 25 | 28 | 31 | 34 | 37 | 41)
}

// ---------------------------------------------------------------------------
// NRD C ABI descriptors (`third_party/nrd/Include/NRDDescs.h`)
// ---------------------------------------------------------------------------

/// `nrd::SPIRVBindingOffsets`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NrdSpirvBindingOffsets {
    pub sampler_offset: u32,
    pub texture_offset: u32,
    pub constant_buffer_offset: u32,
    pub storage_texture_and_buffer_offset: u32,
}
const _: () = assert!(size_of::<NrdSpirvBindingOffsets>() == 0x10);

/// `nrd::LibraryDesc`.
#[repr(C)]
pub struct NrdLibraryDesc {
    pub spirv_binding_offsets: NrdSpirvBindingOffsets, // +0x00
    pub supported_denoisers: *const u32,               // +0x10
    pub supported_denoisers_num: u32,                  // +0x18
    pub version_major: u8,                             // +0x1c
    pub version_minor: u8,                             // +0x1d
    pub version_build: u8,                             // +0x1e
    pub normal_encoding: u8,                           // +0x1f
    pub roughness_encoding: u8,                        // +0x20
}
unsafe impl Sync for NrdLibraryDesc {}

const _: () = {
    assert!(offset_of!(NrdLibraryDesc, spirv_binding_offsets) == 0x00);
    assert!(offset_of!(NrdLibraryDesc, supported_denoisers) == 0x10);
    assert!(offset_of!(NrdLibraryDesc, supported_denoisers_num) == 0x18);
    assert!(offset_of!(NrdLibraryDesc, version_major) == 0x1c);
    assert!(offset_of!(NrdLibraryDesc, version_minor) == 0x1d);
    assert!(offset_of!(NrdLibraryDesc, version_build) == 0x1e);
    assert!(offset_of!(NrdLibraryDesc, normal_encoding) == 0x1f);
    assert!(offset_of!(NrdLibraryDesc, roughness_encoding) == 0x20);
    assert!(size_of::<NrdLibraryDesc>() == 0x28);
};

/// `nrd::MemoryAllocatorInterface`.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NrdMemoryAllocatorInterface {
    pub allocate: usize,
    pub reallocate: usize,
    pub free: usize,
    pub user_arg: usize,
}
const _: () = assert!(size_of::<NrdMemoryAllocatorInterface>() == 0x20);

/// `nrd::DenoiserDesc`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NrdDenoiserDesc {
    pub identifier: u32,    // +0x00
    pub denoiser: u32,      // +0x04
    pub render_width: u16,  // +0x08
    pub render_height: u16, // +0x0a
}
const _: () = assert!(size_of::<NrdDenoiserDesc>() == 0x0c);

/// `nrd::InstanceCreationDesc`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NrdInstanceCreationDesc {
    pub memory_allocator_interface: NrdMemoryAllocatorInterface, // +0x00
    pub denoisers: *const NrdDenoiserDesc,                       // +0x20
    pub denoisers_num: u32,                                      // +0x28
    pub _pad: u32,                                               // +0x2c
}
const _: () = {
    assert!(offset_of!(NrdInstanceCreationDesc, denoisers) == 0x20);
    assert!(offset_of!(NrdInstanceCreationDesc, denoisers_num) == 0x28);
    assert!(size_of::<NrdInstanceCreationDesc>() == 0x30);
};

/// `nrd::TextureDesc`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NrdTextureDesc {
    pub format: u32,  // +0x00
    pub width: u16,   // +0x04
    pub height: u16,  // +0x06
    pub mip_num: u16, // +0x08
    pub _pad: u16,    // +0x0a
}
const _: () = assert!(size_of::<NrdTextureDesc>() == 0x0c);

/// `nrd::ResourceDesc`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NrdResourceDesc {
    pub state_needed: u32,  // +0x00
    pub resource_type: u32, // +0x04
    pub index_in_pool: u16, // +0x08
    pub mip_offset: u16,    // +0x0a
    pub mip_num: u16,       // +0x0c
    pub _pad: u16,          // +0x0e
}
const _: () = assert!(size_of::<NrdResourceDesc>() == 0x10);

/// `nrd::ResourceRangeDesc`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NrdResourceRangeDesc {
    pub descriptor_type: u32,
    pub base_register_index: u32,
    pub descriptors_num: u32,
}
const _: () = assert!(size_of::<NrdResourceRangeDesc>() == 0x0c);

/// `nrd::ComputeShaderDesc`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NrdComputeShaderDesc {
    pub bytecode: *const u8,
    pub size: u64,
}
const _: () = assert!(size_of::<NrdComputeShaderDesc>() == 0x10);

/// `nrd::PipelineDesc`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NrdPipelineDesc {
    pub compute_shader_dxbc: NrdComputeShaderDesc,    // +0x00
    pub compute_shader_dxil: NrdComputeShaderDesc,    // +0x10
    pub compute_shader_spirv: NrdComputeShaderDesc,   // +0x20
    pub shader_file_name: *const u8,                  // +0x30
    pub shader_entry_point_name: *const u8,           // +0x38
    pub resource_ranges: *const NrdResourceRangeDesc, // +0x40
    pub resource_ranges_num: u32,                     // +0x48
    pub has_constant_data: u8,                        // +0x4c
    pub _pad: [u8; 3],                                // +0x4d..+0x50
}
const _: () = assert!(size_of::<NrdPipelineDesc>() == 0x50);

/// `nrd::DescriptorPoolDesc`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NrdDescriptorPoolDesc {
    pub sets_max_num: u32,             // +0x00
    pub constant_buffers_max_num: u32, // +0x04
    pub samplers_max_num: u32,         // +0x08
    pub textures_max_num: u32,         // +0x0c
    pub storage_textures_max_num: u32, // +0x10
}
const _: () = assert!(size_of::<NrdDescriptorPoolDesc>() == 0x14);

/// `nrd::InstanceDesc`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NrdInstanceDesc {
    pub constant_buffer_max_data_size: u32,          // +0x00
    pub constant_buffer_space_index: u32,            // +0x04
    pub constant_buffer_register_index: u32,         // +0x08
    pub _pad0: u32,                                  // +0x0c
    pub samplers: *const u32,                        // +0x10
    pub samplers_num: u32,                           // +0x18
    pub samplers_space_index: u32,                   // +0x1c
    pub samplers_base_register_index: u32,           // +0x20
    pub _pad1: u32,                                  // +0x24
    pub pipelines: *const NrdPipelineDesc,           // +0x28
    pub pipelines_num: u32,                          // +0x30
    pub resources_space_index: u32,                  // +0x34
    pub permanent_pool: *const NrdTextureDesc,       // +0x38
    pub permanent_pool_size: u32,                    // +0x40
    pub _pad2: u32,                                  // +0x44
    pub transient_pool: *const NrdTextureDesc,       // +0x48
    pub transient_pool_size: u32,                    // +0x50
    pub descriptor_pool_desc: NrdDescriptorPoolDesc, // +0x54..+0x68
}
unsafe impl Sync for NrdInstanceDesc {}

const _: () = {
    assert!(offset_of!(NrdInstanceDesc, constant_buffer_max_data_size) == 0x00);
    assert!(offset_of!(NrdInstanceDesc, constant_buffer_space_index) == 0x04);
    assert!(offset_of!(NrdInstanceDesc, constant_buffer_register_index) == 0x08);
    assert!(offset_of!(NrdInstanceDesc, samplers) == 0x10);
    assert!(offset_of!(NrdInstanceDesc, samplers_num) == 0x18);
    assert!(offset_of!(NrdInstanceDesc, samplers_space_index) == 0x1c);
    assert!(offset_of!(NrdInstanceDesc, samplers_base_register_index) == 0x20);
    assert!(offset_of!(NrdInstanceDesc, pipelines) == 0x28);
    assert!(offset_of!(NrdInstanceDesc, pipelines_num) == 0x30);
    assert!(offset_of!(NrdInstanceDesc, resources_space_index) == 0x34);
    assert!(offset_of!(NrdInstanceDesc, permanent_pool) == 0x38);
    assert!(offset_of!(NrdInstanceDesc, permanent_pool_size) == 0x40);
    assert!(offset_of!(NrdInstanceDesc, transient_pool) == 0x48);
    assert!(offset_of!(NrdInstanceDesc, transient_pool_size) == 0x50);
    assert!(offset_of!(NrdInstanceDesc, descriptor_pool_desc) == 0x54);
    assert!(size_of::<NrdInstanceDesc>() == 0x68);
};

/// `nrd::DispatchDesc`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NrdDispatchDesc {
    pub name: *const u8,
    pub resources: *const NrdResourceDesc,
    pub resources_num: u32,
    pub _pad0: u32,
    pub constant_buffer_data: *const u8,
    pub constant_buffer_data_size: u32,
    pub pipeline_index: u16,
    pub grid_width: u16,
    pub grid_height: u16,
    pub _pad1: [u8; 6],
}
const _: () = assert!(size_of::<NrdDispatchDesc>() == 0x30);

// ---------------------------------------------------------------------------
// NRD C ABI settings structs (`third_party/nrd/Include/NRDSettings.h`)
// ---------------------------------------------------------------------------

/// `nrd::CommonSettings`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NrdCommonSettings {
    pub view_to_clip_matrix: [f32; 16],
    pub view_to_clip_matrix_prev: [f32; 16],
    pub world_to_view_matrix: [f32; 16],
    pub world_to_view_matrix_prev: [f32; 16],
    pub world_prev_to_world_matrix: [f32; 16],
    pub motion_vector_scale: [f32; 3],
    pub camera_jitter: [f32; 2],
    pub camera_jitter_prev: [f32; 2],
    pub resolution_scale: [f32; 2],
    pub resolution_scale_prev: [f32; 2],
    pub time_delta_between_frames: f32,
    pub denoising_range: f32,
    pub disocclusion_threshold: f32,
    pub disocclusion_threshold_alternate: f32,
    pub split_screen: f32,
    pub debug: f32,
    pub input_subrect_origin: [u32; 2],
    pub frame_index: u32,
    pub accumulation_mode: u8,
    pub is_motion_vector_in_world_space: u8,
    pub is_history_confidence_available: u8,
    pub is_disocclusion_threshold_mix_available: u8,
    pub is_base_color_metalness_available: u8,
    pub enable_validation: u8,
    pub _pad: [u8; 2],
}
const _: () = {
    assert!(offset_of!(NrdCommonSettings, motion_vector_scale) == 0x140);
    assert!(offset_of!(NrdCommonSettings, denoising_range) == 0x170);
    assert!(offset_of!(NrdCommonSettings, frame_index) == 0x18c);
    assert!(offset_of!(NrdCommonSettings, accumulation_mode) == 0x190);
    assert!(size_of::<NrdCommonSettings>() == 0x198);
};

impl Default for NrdCommonSettings {
    fn default() -> Self {
        Self {
            view_to_clip_matrix: [0.0; 16],
            view_to_clip_matrix_prev: [0.0; 16],
            world_to_view_matrix: [0.0; 16],
            world_to_view_matrix_prev: [0.0; 16],
            world_prev_to_world_matrix: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
            motion_vector_scale: [1.0, 1.0, 0.0],
            camera_jitter: [0.0; 2],
            camera_jitter_prev: [0.0; 2],
            resolution_scale: [1.0, 1.0],
            resolution_scale_prev: [1.0, 1.0],
            time_delta_between_frames: 0.0,
            denoising_range: 500000.0,
            disocclusion_threshold: 0.01,
            disocclusion_threshold_alternate: 0.05,
            split_screen: 0.0,
            debug: 0.0,
            input_subrect_origin: [0; 2],
            frame_index: 0,
            accumulation_mode: NRD_ACCUMULATION_CONTINUE,
            is_motion_vector_in_world_space: 0,
            is_history_confidence_available: 0,
            is_disocclusion_threshold_mix_available: 0,
            is_base_color_metalness_available: 0,
            enable_validation: 0,
            _pad: [0; 2],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NrdHitDistanceParameters {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}
impl Default for NrdHitDistanceParameters {
    fn default() -> Self {
        Self { a: 3.0, b: 0.1, c: 20.0, d: -25.0 }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NrdReblurAntilagSettings {
    pub luminance_sigma_scale: f32,
    pub hit_distance_sigma_scale: f32,
    pub luminance_antilag_power: f32,
    pub hit_distance_antilag_power: f32,
}
impl Default for NrdReblurAntilagSettings {
    fn default() -> Self {
        Self {
            luminance_sigma_scale: 2.0,
            hit_distance_sigma_scale: 2.0,
            luminance_antilag_power: 0.5,
            hit_distance_antilag_power: 1.0,
        }
    }
}

/// `nrd::ReblurSettings`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NrdReblurSettings {
    pub hit_distance_parameters: NrdHitDistanceParameters,
    pub antilag_settings: NrdReblurAntilagSettings,
    pub max_accumulated_frame_num: u32,
    pub max_fast_accumulated_frame_num: u32,
    pub history_fix_frame_num: u32,
    pub diffuse_prepass_blur_radius: f32,
    pub specular_prepass_blur_radius: f32,
    pub blur_radius: f32,
    pub history_fix_stride_between_samples: f32,
    pub lobe_angle_fraction: f32,
    pub roughness_fraction: f32,
    pub responsive_accumulation_roughness_threshold: f32,
    pub stabilization_strength: f32,
    pub plane_distance_sensitivity: f32,
    pub specular_probability_thresholds_for_mv_modification: [f32; 2],
    pub checkerboard_mode: u8,
    pub hit_distance_reconstruction_mode: u8,
    pub enable_anti_firefly: u8,
    pub enable_performance_mode: u8,
    pub enable_material_test_for_diffuse: u8,
    pub enable_material_test_for_specular: u8,
    pub use_prepass_only_for_specular_motion_estimation: u8,
    pub _pad: u8,
}
const _: () = assert!(size_of::<NrdReblurSettings>() == 96);

impl Default for NrdReblurSettings {
    fn default() -> Self {
        Self {
            hit_distance_parameters: NrdHitDistanceParameters::default(),
            antilag_settings: NrdReblurAntilagSettings::default(),
            max_accumulated_frame_num: 30,
            max_fast_accumulated_frame_num: 6,
            history_fix_frame_num: 3,
            diffuse_prepass_blur_radius: 30.0,
            specular_prepass_blur_radius: 50.0,
            blur_radius: 15.0,
            history_fix_stride_between_samples: 14.0,
            lobe_angle_fraction: 0.15,
            roughness_fraction: 0.15,
            responsive_accumulation_roughness_threshold: 0.0,
            stabilization_strength: 1.0,
            plane_distance_sensitivity: 0.005,
            specular_probability_thresholds_for_mv_modification: [0.5, 0.9],
            checkerboard_mode: 0,
            hit_distance_reconstruction_mode: 0,
            enable_anti_firefly: 0,
            enable_performance_mode: 0,
            enable_material_test_for_diffuse: 0,
            enable_material_test_for_specular: 0,
            use_prepass_only_for_specular_motion_estimation: 0,
            _pad: 0,
        }
    }
}

/// `nrd::SigmaSettings`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NrdSigmaSettings {
    pub plane_distance_sensitivity: f32,
    pub blur_radius_scale: f32,
}
const _: () = assert!(size_of::<NrdSigmaSettings>() == 8);

impl Default for NrdSigmaSettings {
    fn default() -> Self {
        Self { plane_distance_sensitivity: 0.005, blur_radius_scale: 2.0 }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NrdRelaxAntilagSettings {
    pub acceleration_amount: f32,
    pub spatial_sigma_scale: f32,
    pub temporal_sigma_scale: f32,
    pub reset_amount: f32,
}
impl Default for NrdRelaxAntilagSettings {
    fn default() -> Self {
        Self {
            acceleration_amount: 0.3,
            spatial_sigma_scale: 4.5,
            temporal_sigma_scale: 0.5,
            reset_amount: 0.5,
        }
    }
}

/// `nrd::RelaxDiffuseSpecularSettings`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NrdRelaxDiffuseSpecularSettings {
    pub antilag_settings: NrdRelaxAntilagSettings,
    pub diffuse_prepass_blur_radius: f32,
    pub specular_prepass_blur_radius: f32,
    pub diffuse_max_accumulated_frame_num: u32,
    pub specular_max_accumulated_frame_num: u32,
    pub diffuse_max_fast_accumulated_frame_num: u32,
    pub specular_max_fast_accumulated_frame_num: u32,
    pub history_fix_frame_num: u32,
    pub diffuse_phi_luminance: f32,
    pub specular_phi_luminance: f32,
    pub diffuse_lobe_angle_fraction: f32,
    pub specular_lobe_angle_fraction: f32,
    pub roughness_fraction: f32,
    pub specular_variance_boost: f32,
    pub specular_lobe_angle_slack: f32,
    pub history_fix_stride_between_samples: f32,
    pub history_fix_edge_stopping_normal_power: f32,
    pub history_clamping_color_box_sigma_scale: f32,
    pub spatial_variance_estimation_history_threshold: u32,
    pub atrous_iteration_num: u32,
    pub diffuse_min_luminance_weight: f32,
    pub specular_min_luminance_weight: f32,
    pub depth_threshold: f32,
    pub confidence_driven_relaxation_multiplier: f32,
    pub confidence_driven_luminance_edge_stopping_relaxation: f32,
    pub confidence_driven_normal_edge_stopping_relaxation: f32,
    pub luminance_edge_stopping_relaxation: f32,
    pub normal_edge_stopping_relaxation: f32,
    pub roughness_edge_stopping_relaxation: f32,
    pub checkerboard_mode: u8,
    pub hit_distance_reconstruction_mode: u8,
    pub enable_anti_firefly: u8,
    pub enable_reprojection_test_skipping_without_motion: u8,
    pub enable_roughness_edge_stopping: u8,
    pub enable_material_test_for_diffuse: u8,
    pub enable_material_test_for_specular: u8,
    pub _pad: u8,
}
const _: () = assert!(size_of::<NrdRelaxDiffuseSpecularSettings>() == 136);

impl Default for NrdRelaxDiffuseSpecularSettings {
    fn default() -> Self {
        Self {
            antilag_settings: NrdRelaxAntilagSettings::default(),
            diffuse_prepass_blur_radius: 50.0,
            specular_prepass_blur_radius: 50.0,
            diffuse_max_accumulated_frame_num: 30,
            specular_max_accumulated_frame_num: 30,
            diffuse_max_fast_accumulated_frame_num: 6,
            specular_max_fast_accumulated_frame_num: 6,
            history_fix_frame_num: 3,
            diffuse_phi_luminance: 2.0,
            specular_phi_luminance: 1.0,
            diffuse_lobe_angle_fraction: 0.5,
            specular_lobe_angle_fraction: 0.5,
            roughness_fraction: 0.15,
            specular_variance_boost: 0.0,
            specular_lobe_angle_slack: 0.15,
            history_fix_stride_between_samples: 14.0,
            history_fix_edge_stopping_normal_power: 8.0,
            history_clamping_color_box_sigma_scale: 2.0,
            spatial_variance_estimation_history_threshold: 3,
            atrous_iteration_num: 5,
            diffuse_min_luminance_weight: 0.0,
            specular_min_luminance_weight: 0.0,
            depth_threshold: 0.003,
            confidence_driven_relaxation_multiplier: 0.0,
            confidence_driven_luminance_edge_stopping_relaxation: 0.0,
            confidence_driven_normal_edge_stopping_relaxation: 0.0,
            luminance_edge_stopping_relaxation: 0.5,
            normal_edge_stopping_relaxation: 0.3,
            roughness_edge_stopping_relaxation: 1.0,
            checkerboard_mode: 0,
            hit_distance_reconstruction_mode: 0,
            enable_anti_firefly: 0,
            enable_reprojection_test_skipping_without_motion: 0,
            enable_roughness_edge_stopping: 1,
            enable_material_test_for_diffuse: 0,
            enable_material_test_for_specular: 0,
            _pad: 0,
        }
    }
}

/// `nrd::RelaxDiffuseSettings`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NrdRelaxDiffuseSettings {
    pub antilag_settings: NrdRelaxAntilagSettings,
    pub prepass_blur_radius: f32,
    pub diffuse_max_accumulated_frame_num: u32,
    pub diffuse_max_fast_accumulated_frame_num: u32,
    pub history_fix_frame_num: u32,
    pub diffuse_phi_luminance: f32,
    pub diffuse_lobe_angle_fraction: f32,
    pub history_fix_edge_stopping_normal_power: f32,
    pub history_fix_stride_between_samples: f32,
    pub history_clamping_color_box_sigma_scale: f32,
    pub spatial_variance_estimation_history_threshold: u32,
    pub atrous_iteration_num: u32,
    pub min_luminance_weight: f32,
    pub depth_threshold: f32,
    pub confidence_driven_relaxation_multiplier: f32,
    pub confidence_driven_luminance_edge_stopping_relaxation: f32,
    pub confidence_driven_normal_edge_stopping_relaxation: f32,
    pub checkerboard_mode: u8,
    pub hit_distance_reconstruction_mode: u8,
    pub enable_anti_firefly: u8,
    pub enable_reprojection_test_skipping_without_motion: u8,
    pub enable_material_test: u8,
    pub _pad: [u8; 3],
}
const _: () = assert!(size_of::<NrdRelaxDiffuseSettings>() == 88);

impl Default for NrdRelaxDiffuseSettings {
    fn default() -> Self {
        Self {
            antilag_settings: NrdRelaxAntilagSettings::default(),
            prepass_blur_radius: 30.0,
            diffuse_max_accumulated_frame_num: 30,
            diffuse_max_fast_accumulated_frame_num: 6,
            history_fix_frame_num: 3,
            diffuse_phi_luminance: 2.0,
            diffuse_lobe_angle_fraction: 0.5,
            history_fix_edge_stopping_normal_power: 8.0,
            history_fix_stride_between_samples: 14.0,
            history_clamping_color_box_sigma_scale: 2.0,
            spatial_variance_estimation_history_threshold: 3,
            atrous_iteration_num: 5,
            min_luminance_weight: 0.0,
            depth_threshold: 0.01,
            confidence_driven_relaxation_multiplier: 0.0,
            confidence_driven_luminance_edge_stopping_relaxation: 0.0,
            confidence_driven_normal_edge_stopping_relaxation: 0.0,
            checkerboard_mode: 0,
            hit_distance_reconstruction_mode: 0,
            enable_anti_firefly: 0,
            enable_reprojection_test_skipping_without_motion: 0,
            enable_material_test: 0,
            _pad: [0; 3],
        }
    }
}

/// `nrd::RelaxSpecularSettings`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NrdRelaxSpecularSettings {
    pub antilag_settings: NrdRelaxAntilagSettings,
    pub prepass_blur_radius: f32,
    pub specular_max_accumulated_frame_num: u32,
    pub specular_max_fast_accumulated_frame_num: u32,
    pub history_fix_frame_num: u32,
    pub specular_phi_luminance: f32,
    pub diffuse_lobe_angle_fraction: f32,
    pub specular_lobe_angle_fraction: f32,
    pub roughness_fraction: f32,
    pub specular_variance_boost: f32,
    pub specular_lobe_angle_slack: f32,
    pub history_fix_edge_stopping_normal_power: f32,
    pub history_fix_stride_between_samples: f32,
    pub history_clamping_color_box_sigma_scale: f32,
    pub spatial_variance_estimation_history_threshold: u32,
    pub atrous_iteration_num: u32,
    pub min_luminance_weight: f32,
    pub depth_threshold: f32,
    pub confidence_driven_relaxation_multiplier: f32,
    pub confidence_driven_luminance_edge_stopping_relaxation: f32,
    pub confidence_driven_normal_edge_stopping_relaxation: f32,
    pub luminance_edge_stopping_relaxation: f32,
    pub normal_edge_stopping_relaxation: f32,
    pub roughness_edge_stopping_relaxation: f32,
    pub checkerboard_mode: u8,
    pub hit_distance_reconstruction_mode: u8,
    pub enable_anti_firefly: u8,
    pub enable_reprojection_test_skipping_without_motion: u8,
    pub enable_roughness_edge_stopping: u8,
    pub enable_material_test: u8,
    pub _pad: [u8; 2],
}
const _: () = assert!(size_of::<NrdRelaxSpecularSettings>() == 116);

impl Default for NrdRelaxSpecularSettings {
    fn default() -> Self {
        Self {
            antilag_settings: NrdRelaxAntilagSettings::default(),
            prepass_blur_radius: 50.0,
            specular_max_accumulated_frame_num: 30,
            specular_max_fast_accumulated_frame_num: 6,
            history_fix_frame_num: 3,
            specular_phi_luminance: 1.0,
            diffuse_lobe_angle_fraction: 0.5,
            specular_lobe_angle_fraction: 0.5,
            roughness_fraction: 0.15,
            specular_variance_boost: 0.0,
            specular_lobe_angle_slack: 0.15,
            history_fix_edge_stopping_normal_power: 8.0,
            history_fix_stride_between_samples: 14.0,
            history_clamping_color_box_sigma_scale: 2.0,
            spatial_variance_estimation_history_threshold: 3,
            atrous_iteration_num: 5,
            min_luminance_weight: 0.0,
            depth_threshold: 0.01,
            confidence_driven_relaxation_multiplier: 0.0,
            confidence_driven_luminance_edge_stopping_relaxation: 0.0,
            confidence_driven_normal_edge_stopping_relaxation: 0.0,
            luminance_edge_stopping_relaxation: 0.5,
            normal_edge_stopping_relaxation: 0.3,
            roughness_edge_stopping_relaxation: 1.0,
            checkerboard_mode: 0,
            hit_distance_reconstruction_mode: 0,
            enable_anti_firefly: 0,
            enable_reprojection_test_skipping_without_motion: 0,
            enable_roughness_edge_stopping: 1,
            enable_material_test: 0,
            _pad: [0; 2],
        }
    }
}

/// `nrd::ReferenceSettings`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NrdReferenceSettings {
    pub max_accumulated_frame_num: u32,
}
const _: () = assert!(size_of::<NrdReferenceSettings>() == 4);

impl Default for NrdReferenceSettings {
    fn default() -> Self {
        Self { max_accumulated_frame_num: 1024 }
    }
}

// ---------------------------------------------------------------------------
// Constant buffer layouts matching the embedded HLSL cs_6_0 shaders
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
struct ReblurCb {
    rect_origin: [u32; 2],
    rect_size: [u32; 2],
    inv_rect_size: [f32; 2],
    resolution_scale: [f32; 2],
    mv_scale: [f32; 3],
    denoising_range: f32,
    blur_radius: f32,
    max_accum_frames: f32,
    plane_dist_sens: f32,
    disocclusion_thresh: f32,
    stabilization: f32,
    split_screen: f32,
    pass_mode: u32,
    flags: u32,
    rotator: [f32; 4],
    hit_dist_params: [f32; 4],
    lobe_params: [f32; 4],
}
const _: () = assert!(size_of::<ReblurCb>() == 128);

#[repr(C)]
#[derive(Clone, Copy)]
struct RelaxCb {
    rect_origin: [u32; 2],
    rect_size: [u32; 2],
    inv_rect_size: [f32; 2],
    resolution_scale: [f32; 2],
    mv_scale: [f32; 3],
    denoising_range: f32,
    phi_luminance: f32,
    depth_threshold: f32,
    max_accum_frames: f32,
    disocclusion_thresh: f32,
    lobe_angle_fraction: f32,
    roughness_fraction: f32,
    color_box_sigma_scale: f32,
    split_screen: f32,
    pass_mode: u32,
    step_size: u32,
    flags: u32,
    frame_index: u32,
    relaxation_params: [f32; 4],
    antilag_params: [f32; 4],
}
const _: () = assert!(size_of::<RelaxCb>() == 128);

#[repr(C)]
#[derive(Clone, Copy)]
struct SigmaCb {
    rect_origin: [u32; 2],
    rect_size: [u32; 2],
    inv_rect_size: [f32; 2],
    resolution_scale: [f32; 2],
    mv_scale: [f32; 3],
    denoising_range: f32,
    plane_dist_sens: f32,
    blur_radius_scale: f32,
    split_screen: f32,
    flags: u32,
}
const _: () = assert!(size_of::<SigmaCb>() == 64);

#[repr(C)]
#[derive(Clone, Copy)]
struct ReferenceCb {
    rect_origin: [u32; 2],
    rect_size: [u32; 2],
    inv_rect_size: [f32; 2],
    split_screen: f32,
    accum_speed: f32,
    debug: f32,
    reset_history: u32,
    frame_index: u32,
    _pad: u32,
}
const _: () = assert!(size_of::<ReferenceCb>() == 48);

#[repr(C)]
#[derive(Clone, Copy)]
struct SpecularMvCb {
    world_to_clip: [f32; 16],
    world_to_clip_prev: [f32; 16],
    rect_origin: [u32; 2],
    rect_size: [u32; 2],
    inv_rect_size: [f32; 2],
    resolution_scale: [f32; 2],
    mv_scale: [f32; 3],
    denoising_range: f32,
    mode: u32,
    is_world_space_mv: u32,
    frame_index: u32,
    _pad: u32,
}
const _: () = assert!(size_of::<SpecularMvCb>() == 192);

// ---------------------------------------------------------------------------
// Static tables & pipeline templates (`third_party/nrd/Source/Wrapper.cpp`)
// ---------------------------------------------------------------------------

static SUPPORTED_DENOISERS: [u32; NRD_DENOISER_MAX_NUM] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
];

static SAMPLERS: [u32; 4] = [
    0, // NEAREST_CLAMP
    1, // NEAREST_MIRRORED_REPEAT
    2, // LINEAR_CLAMP
    3, // LINEAR_MIRRORED_REPEAT
];

static RESOURCE_TYPE_NAMES: [&[u8]; NRD_RESOURCE_TYPE_MAX_NUM] = [
    b"IN_MV\0",
    b"IN_NORMAL_ROUGHNESS\0",
    b"IN_VIEWZ\0",
    b"IN_DIFF_RADIANCE_HITDIST\0",
    b"IN_SPEC_RADIANCE_HITDIST\0",
    b"IN_DIFF_HITDIST\0",
    b"IN_SPEC_HITDIST\0",
    b"IN_DIFF_DIRECTION_HITDIST\0",
    b"IN_DIFF_SH0\0",
    b"IN_DIFF_SH1\0",
    b"IN_SPEC_SH0\0",
    b"IN_SPEC_SH1\0",
    b"IN_DIFF_CONFIDENCE\0",
    b"IN_SPEC_CONFIDENCE\0",
    b"IN_DISOCCLUSION_THRESHOLD_MIX\0",
    b"IN_BASECOLOR_METALNESS\0",
    b"IN_SHADOWDATA\0",
    b"IN_SHADOW_TRANSLUCENCY\0",
    b"IN_RADIANCE\0",
    b"IN_DELTA_PRIMARY_POS\0",
    b"IN_DELTA_SECONDARY_POS\0",
    b"OUT_DIFF_RADIANCE_HITDIST\0",
    b"OUT_SPEC_RADIANCE_HITDIST\0",
    b"OUT_DIFF_SH0\0",
    b"OUT_DIFF_SH1\0",
    b"OUT_SPEC_SH0\0",
    b"OUT_SPEC_SH1\0",
    b"OUT_DIFF_HITDIST\0",
    b"OUT_SPEC_HITDIST\0",
    b"OUT_DIFF_DIRECTION_HITDIST\0",
    b"OUT_SHADOW_TRANSLUCENCY\0",
    b"OUT_RADIANCE\0",
    b"OUT_REFLECTION_MV\0",
    b"OUT_DELTA_MV\0",
    b"OUT_VALIDATION\0",
    b"TRANSIENT_POOL\0",
    b"PERMANENT_POOL\0",
];

static DENOISER_NAMES: [&[u8]; NRD_DENOISER_MAX_NUM] = [
    b"REBLUR_DIFFUSE\0",
    b"REBLUR_DIFFUSE_OCCLUSION\0",
    b"REBLUR_DIFFUSE_SH\0",
    b"REBLUR_SPECULAR\0",
    b"REBLUR_SPECULAR_OCCLUSION\0",
    b"REBLUR_SPECULAR_SH\0",
    b"REBLUR_DIFFUSE_SPECULAR\0",
    b"REBLUR_DIFFUSE_SPECULAR_OCCLUSION\0",
    b"REBLUR_DIFFUSE_SPECULAR_SH\0",
    b"REBLUR_DIFFUSE_DIRECTIONAL_OCCLUSION\0",
    b"SIGMA_SHADOW\0",
    b"SIGMA_SHADOW_TRANSLUCENCY\0",
    b"RELAX_DIFFUSE\0",
    b"RELAX_DIFFUSE_SH\0",
    b"RELAX_SPECULAR\0",
    b"RELAX_SPECULAR_SH\0",
    b"RELAX_DIFFUSE_SPECULAR\0",
    b"RELAX_DIFFUSE_SPECULAR_SH\0",
    b"REFERENCE\0",
    b"SPECULAR_REFLECTION_MV\0",
    b"SPECULAR_DELTA_MV\0",
];

static LIBRARY_DESC: NrdLibraryDesc = NrdLibraryDesc {
    spirv_binding_offsets: NrdSpirvBindingOffsets {
        sampler_offset: 100,
        texture_offset: 200,
        constant_buffer_offset: 300,
        storage_texture_and_buffer_offset: 400,
    },
    supported_denoisers: SUPPORTED_DENOISERS.as_ptr(),
    supported_denoisers_num: SUPPORTED_DENOISERS.len() as u32,
    version_major: 4,
    version_minor: 3,
    version_build: 6,
    normal_encoding: 2,    // NormalEncoding::R10_G10_B10_A2_UNORM
    roughness_encoding: 1, // RoughnessEncoding::LINEAR
};

static SHADER_ENTRY_MAIN: &[u8] = b"main\0";

static RANGES_CLEAR: [NrdResourceRangeDesc; 1] = [NrdResourceRangeDesc {
    descriptor_type: NRD_DESCRIPTOR_STORAGE_TEXTURE,
    base_register_index: 0,
    descriptors_num: 1,
}];

static RANGES_6SRV_2UAV: [NrdResourceRangeDesc; 2] = [
    NrdResourceRangeDesc {
        descriptor_type: NRD_DESCRIPTOR_TEXTURE,
        base_register_index: 0,
        descriptors_num: 6,
    },
    NrdResourceRangeDesc {
        descriptor_type: NRD_DESCRIPTOR_STORAGE_TEXTURE,
        base_register_index: 0,
        descriptors_num: 2,
    },
];

static RANGES_2SRV_2UAV: [NrdResourceRangeDesc; 2] = [
    NrdResourceRangeDesc {
        descriptor_type: NRD_DESCRIPTOR_TEXTURE,
        base_register_index: 0,
        descriptors_num: 2,
    },
    NrdResourceRangeDesc {
        descriptor_type: NRD_DESCRIPTOR_STORAGE_TEXTURE,
        base_register_index: 0,
        descriptors_num: 2,
    },
];

static RANGES_4SRV_2UAV: [NrdResourceRangeDesc; 2] = [
    NrdResourceRangeDesc {
        descriptor_type: NRD_DESCRIPTOR_TEXTURE,
        base_register_index: 0,
        descriptors_num: 4,
    },
    NrdResourceRangeDesc {
        descriptor_type: NRD_DESCRIPTOR_STORAGE_TEXTURE,
        base_register_index: 0,
        descriptors_num: 2,
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ShaderKind {
    ClearFloat,
    ClearUint,
    ReblurFilter,
    RelaxFilter,
    SigmaShadow,
    ReferenceAccumulate,
    SpecularMvCompute,
}

impl ShaderKind {
    fn pipeline_desc(self) -> NrdPipelineDesc {
        let (dxil, file_name, ranges, has_cb): (
            &'static [u8],
            &'static [u8],
            &'static [NrdResourceRangeDesc],
            u8,
        ) = match self {
            Self::ClearFloat => (shaders::CLEAR_F_DXIL, b"Clear_f.cs\0", &RANGES_CLEAR, 0),
            Self::ClearUint => (shaders::CLEAR_UI_DXIL, b"Clear_ui.cs\0", &RANGES_CLEAR, 0),
            Self::ReblurFilter => {
                (shaders::REBLUR_FILTER_DXIL, b"REBLUR_Filter.cs\0", &RANGES_6SRV_2UAV, 1)
            }
            Self::RelaxFilter => {
                (shaders::RELAX_FILTER_DXIL, b"RELAX_Filter.cs\0", &RANGES_6SRV_2UAV, 1)
            }
            Self::SigmaShadow => {
                (shaders::SIGMA_SHADOW_DXIL, b"SIGMA_Shadow.cs\0", &RANGES_6SRV_2UAV, 1)
            }
            Self::ReferenceAccumulate => (
                shaders::REFERENCE_ACCUMULATE_DXIL,
                b"REFERENCE_Accumulate.cs\0",
                &RANGES_2SRV_2UAV,
                1,
            ),
            Self::SpecularMvCompute => (
                shaders::SPECULAR_MV_COMPUTE_DXIL,
                b"SpecularMv_Compute.cs\0",
                &RANGES_4SRV_2UAV,
                1,
            ),
        };
        NrdPipelineDesc {
            compute_shader_dxbc: NrdComputeShaderDesc { bytecode: ptr::null(), size: 0 },
            compute_shader_dxil: NrdComputeShaderDesc {
                bytecode: dxil.as_ptr(),
                size: dxil.len() as u64,
            },
            compute_shader_spirv: NrdComputeShaderDesc { bytecode: ptr::null(), size: 0 },
            shader_file_name: file_name.as_ptr(),
            shader_entry_point_name: SHADER_ENTRY_MAIN.as_ptr(),
            resource_ranges: ranges.as_ptr(),
            resource_ranges_num: ranges.len() as u32,
            has_constant_data: has_cb,
            _pad: [0; 3],
        }
    }

    const fn cb_size(self) -> u32 {
        match self {
            Self::ClearFloat | Self::ClearUint => 0,
            Self::ReblurFilter => size_of::<ReblurCb>() as u32,
            Self::RelaxFilter => size_of::<RelaxCb>() as u32,
            Self::SigmaShadow => size_of::<SigmaCb>() as u32,
            Self::ReferenceAccumulate => size_of::<ReferenceCb>() as u32,
            Self::SpecularMvCompute => size_of::<SpecularMvCb>() as u32,
        }
    }
}

static EMPTY_INSTANCE_DESC: NrdInstanceDesc = NrdInstanceDesc {
    constant_buffer_max_data_size: 256,
    constant_buffer_space_index: 0,
    constant_buffer_register_index: 0,
    _pad0: 0,
    samplers: SAMPLERS.as_ptr(),
    samplers_num: SAMPLERS.len() as u32,
    samplers_space_index: 0,
    samplers_base_register_index: 0,
    _pad1: 0,
    pipelines: ptr::null(),
    pipelines_num: 0,
    resources_space_index: 0,
    permanent_pool: ptr::null(),
    permanent_pool_size: 0,
    _pad2: 0,
    transient_pool: ptr::null(),
    transient_pool_size: 0,
    descriptor_pool_desc: NrdDescriptorPoolDesc {
        sets_max_num: 1,
        constant_buffers_max_num: 1,
        samplers_max_num: 4,
        textures_max_num: 1,
        storage_textures_max_num: 1,
    },
};

// ---------------------------------------------------------------------------
// Per-instance denoiser state, pool allocation, and dispatch planning
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
enum DenoiserSettingsState {
    Reblur(NrdReblurSettings),
    Sigma(NrdSigmaSettings),
    RelaxDiffuse(NrdRelaxDiffuseSettings),
    RelaxSpecular(NrdRelaxSpecularSettings),
    RelaxDiffuseSpecular(NrdRelaxDiffuseSpecularSettings),
    Reference(NrdReferenceSettings),
    MotionVector,
}

impl DenoiserSettingsState {
    fn default_for(denoiser: u32) -> Self {
        match denoiser {
            NRD_DENOISER_REBLUR_DIFFUSE..=NRD_DENOISER_REBLUR_DIFFUSE_DIRECTIONAL_OCCLUSION => {
                Self::Reblur(NrdReblurSettings::default())
            }
            NRD_DENOISER_SIGMA_SHADOW..=NRD_DENOISER_SIGMA_SHADOW_TRANSLUCENCY => {
                Self::Sigma(NrdSigmaSettings::default())
            }
            NRD_DENOISER_RELAX_DIFFUSE..=NRD_DENOISER_RELAX_DIFFUSE_SH => {
                Self::RelaxDiffuse(NrdRelaxDiffuseSettings::default())
            }
            NRD_DENOISER_RELAX_SPECULAR..=NRD_DENOISER_RELAX_SPECULAR_SH => {
                Self::RelaxSpecular(NrdRelaxSpecularSettings::default())
            }
            NRD_DENOISER_RELAX_DIFFUSE_SPECULAR..=NRD_DENOISER_RELAX_DIFFUSE_SPECULAR_SH => {
                Self::RelaxDiffuseSpecular(NrdRelaxDiffuseSpecularSettings::default())
            }
            NRD_DENOISER_REFERENCE => Self::Reference(NrdReferenceSettings::default()),
            _ => Self::MotionVector,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct SignalBinding {
    in_res: u32,
    out_res: u32,
    is_specular: bool,
    hist_ping: u16,
    hist_pong: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ClearTarget {
    resource_type: u32, // NRD_RES_PERMANENT_POOL or NRD_RES_TRANSIENT_POOL
    index_in_pool: u16,
    width: u16,
    height: u16,
    is_integer: bool,
}

enum DenoiserPlan {
    Reblur {
        pipeline_index: u16,
        meta_ping: u16,
        meta_pong: u16,
        transient_scratch: u16,
        signals: Vec<SignalBinding>,
    },
    Relax {
        pipeline_index: u16,
        meta_ping: u16,
        meta_pong: u16,
        transient_ping: u16,
        transient_pong: u16,
        signals: Vec<SignalBinding>,
    },
    Sigma {
        pipeline_index: u16,
        has_translucency: bool,
        hist_ping: u16,
        hist_pong: u16,
        transient_blur: u16,
        transient_penumbra: u16,
    },
    Reference {
        pipeline_index: u16,
        hist_ping: u16,
        hist_pong: u16,
    },
    SpecularReflectionMv {
        pipeline_index: u16,
        transient_aux: u16,
    },
    SpecularDeltaMv {
        pipeline_index: u16,
        pos_ping: u16,
        pos_pong: u16,
    },
}

struct DenoiserEntry {
    desc: NrdDenoiserDesc,
    settings: DenoiserSettingsState,
    settings_set: bool,
    plan: DenoiserPlan,
    clear_targets: Vec<ClearTarget>,
    ping_pong: u8,
    accumulated_frame_num: u32,
}

struct PendingDispatch {
    name: &'static [u8],
    res_start: usize,
    res_len: usize,
    cb_start: usize,
    cb_len: u32,
    pipeline_index: u16,
    grid_width: u16,
    grid_height: u16,
}

pub struct NrdInstance {
    pub desc: NrdInstanceDesc,
    _pipelines: Box<[NrdPipelineDesc]>,
    _permanent_pool: Box<[NrdTextureDesc]>,
    _transient_pool: Box<[NrdTextureDesc]>,
    clear_float_pipeline: u16,
    clear_uint_pipeline: u16,
    denoisers: Vec<DenoiserEntry>,
    common_settings: NrdCommonSettings,
    is_first_use: bool,
    dispatch_resources: Vec<NrdResourceDesc>,
    constant_buffer_arena: Vec<u8>,
    active_dispatches: Vec<NrdDispatchDesc>,
}

fn res_srv(resource_type: u32, index_in_pool: u16) -> NrdResourceDesc {
    NrdResourceDesc {
        state_needed: NRD_DESCRIPTOR_TEXTURE,
        resource_type,
        index_in_pool,
        mip_offset: 0,
        mip_num: 1,
        _pad: 0,
    }
}

fn res_uav(resource_type: u32, index_in_pool: u16) -> NrdResourceDesc {
    NrdResourceDesc {
        state_needed: NRD_DESCRIPTOR_STORAGE_TEXTURE,
        resource_type,
        index_in_pool,
        mip_offset: 0,
        mip_num: 1,
        _pad: 0,
    }
}

fn tex2d(format: u32, width: u16, height: u16) -> NrdTextureDesc {
    NrdTextureDesc { format, width, height, mip_num: 1, _pad: 0 }
}

fn alloc_permanent(
    pool: &mut Vec<NrdTextureDesc>,
    clears: &mut Vec<ClearTarget>,
    desc: NrdTextureDesc,
) -> u16 {
    let idx = pool.len() as u16;
    pool.push(desc);
    clears.push(ClearTarget {
        resource_type: NRD_RES_PERMANENT_POOL,
        index_in_pool: idx,
        width: desc.width,
        height: desc.height,
        is_integer: is_integer_format(desc.format),
    });
    idx
}

fn alloc_transient(
    pool: &mut Vec<NrdTextureDesc>,
    prev_limit: usize,
    used_in_denoiser: &mut Vec<u16>,
    clears: &mut Vec<ClearTarget>,
    desc: NrdTextureDesc,
) -> u16 {
    for (i, existing) in pool.iter().take(prev_limit).enumerate() {
        let idx = i as u16;
        if *existing == desc && !used_in_denoiser.contains(&idx) {
            used_in_denoiser.push(idx);
            clears.push(ClearTarget {
                resource_type: NRD_RES_TRANSIENT_POOL,
                index_in_pool: idx,
                width: desc.width,
                height: desc.height,
                is_integer: is_integer_format(desc.format),
            });
            return idx;
        }
    }
    let idx = pool.len() as u16;
    pool.push(desc);
    used_in_denoiser.push(idx);
    clears.push(ClearTarget {
        resource_type: NRD_RES_TRANSIENT_POOL,
        index_in_pool: idx,
        width: desc.width,
        height: desc.height,
        is_integer: is_integer_format(desc.format),
    });
    idx
}

fn get_or_add_pipeline(
    kinds: &mut Vec<ShaderKind>,
    descs: &mut Vec<NrdPipelineDesc>,
    kind: ShaderKind,
) -> u16 {
    if let Some(pos) = kinds.iter().position(|&k| k == kind) {
        return pos as u16;
    }
    let idx = descs.len() as u16;
    kinds.push(kind);
    descs.push(kind.pipeline_desc());
    idx
}

fn reblur_signals_and_format(denoiser: u32) -> (u32, &'static [(u32, u32, bool)]) {
    match denoiser {
        NRD_DENOISER_REBLUR_DIFFUSE => (
            NRD_FORMAT_RGBA16_SFLOAT,
            &[(NRD_RES_IN_DIFF_RADIANCE_HITDIST, NRD_RES_OUT_DIFF_RADIANCE_HITDIST, false)],
        ),
        NRD_DENOISER_REBLUR_DIFFUSE_OCCLUSION => (
            NRD_FORMAT_R16_UNORM,
            &[(NRD_RES_IN_DIFF_HITDIST, NRD_RES_OUT_DIFF_HITDIST, false)],
        ),
        NRD_DENOISER_REBLUR_DIFFUSE_SH => (
            NRD_FORMAT_RGBA16_SFLOAT,
            &[
                (NRD_RES_IN_DIFF_SH0, NRD_RES_OUT_DIFF_SH0, false),
                (NRD_RES_IN_DIFF_SH1, NRD_RES_OUT_DIFF_SH1, false),
            ],
        ),
        NRD_DENOISER_REBLUR_SPECULAR => (
            NRD_FORMAT_RGBA16_SFLOAT,
            &[(NRD_RES_IN_SPEC_RADIANCE_HITDIST, NRD_RES_OUT_SPEC_RADIANCE_HITDIST, true)],
        ),
        NRD_DENOISER_REBLUR_SPECULAR_OCCLUSION => (
            NRD_FORMAT_R16_UNORM,
            &[(NRD_RES_IN_SPEC_HITDIST, NRD_RES_OUT_SPEC_HITDIST, true)],
        ),
        NRD_DENOISER_REBLUR_SPECULAR_SH => (
            NRD_FORMAT_RGBA16_SFLOAT,
            &[
                (NRD_RES_IN_SPEC_SH0, NRD_RES_OUT_SPEC_SH0, true),
                (NRD_RES_IN_SPEC_SH1, NRD_RES_OUT_SPEC_SH1, true),
            ],
        ),
        NRD_DENOISER_REBLUR_DIFFUSE_SPECULAR => (
            NRD_FORMAT_RGBA16_SFLOAT,
            &[
                (NRD_RES_IN_DIFF_RADIANCE_HITDIST, NRD_RES_OUT_DIFF_RADIANCE_HITDIST, false),
                (NRD_RES_IN_SPEC_RADIANCE_HITDIST, NRD_RES_OUT_SPEC_RADIANCE_HITDIST, true),
            ],
        ),
        NRD_DENOISER_REBLUR_DIFFUSE_SPECULAR_OCCLUSION => (
            NRD_FORMAT_R16_UNORM,
            &[
                (NRD_RES_IN_DIFF_HITDIST, NRD_RES_OUT_DIFF_HITDIST, false),
                (NRD_RES_IN_SPEC_HITDIST, NRD_RES_OUT_SPEC_HITDIST, true),
            ],
        ),
        NRD_DENOISER_REBLUR_DIFFUSE_SPECULAR_SH => (
            NRD_FORMAT_RGBA16_SFLOAT,
            &[
                (NRD_RES_IN_DIFF_SH0, NRD_RES_OUT_DIFF_SH0, false),
                (NRD_RES_IN_DIFF_SH1, NRD_RES_OUT_DIFF_SH1, false),
                (NRD_RES_IN_SPEC_SH0, NRD_RES_OUT_SPEC_SH0, true),
                (NRD_RES_IN_SPEC_SH1, NRD_RES_OUT_SPEC_SH1, true),
            ],
        ),
        _ => (
            NRD_FORMAT_RGBA16_SNORM,
            &[(NRD_RES_IN_DIFF_DIRECTION_HITDIST, NRD_RES_OUT_DIFF_DIRECTION_HITDIST, false)],
        ),
    }
}

fn relax_signals(denoiser: u32) -> &'static [(u32, u32, bool)] {
    match denoiser {
        NRD_DENOISER_RELAX_DIFFUSE => {
            &[(NRD_RES_IN_DIFF_RADIANCE_HITDIST, NRD_RES_OUT_DIFF_RADIANCE_HITDIST, false)]
        }
        NRD_DENOISER_RELAX_DIFFUSE_SH => &[
            (NRD_RES_IN_DIFF_SH0, NRD_RES_OUT_DIFF_SH0, false),
            (NRD_RES_IN_DIFF_SH1, NRD_RES_OUT_DIFF_SH1, false),
        ],
        NRD_DENOISER_RELAX_SPECULAR => {
            &[(NRD_RES_IN_SPEC_RADIANCE_HITDIST, NRD_RES_OUT_SPEC_RADIANCE_HITDIST, true)]
        }
        NRD_DENOISER_RELAX_SPECULAR_SH => &[
            (NRD_RES_IN_SPEC_SH0, NRD_RES_OUT_SPEC_SH0, true),
            (NRD_RES_IN_SPEC_SH1, NRD_RES_OUT_SPEC_SH1, true),
        ],
        NRD_DENOISER_RELAX_DIFFUSE_SPECULAR => &[
            (NRD_RES_IN_DIFF_RADIANCE_HITDIST, NRD_RES_OUT_DIFF_RADIANCE_HITDIST, false),
            (NRD_RES_IN_SPEC_RADIANCE_HITDIST, NRD_RES_OUT_SPEC_RADIANCE_HITDIST, true),
        ],
        _ => &[
            (NRD_RES_IN_DIFF_SH0, NRD_RES_OUT_DIFF_SH0, false),
            (NRD_RES_IN_DIFF_SH1, NRD_RES_OUT_DIFF_SH1, false),
            (NRD_RES_IN_SPEC_SH0, NRD_RES_OUT_SPEC_SH0, true),
            (NRD_RES_IN_SPEC_SH1, NRD_RES_OUT_SPEC_SH1, true),
        ],
    }
}

fn mat4_mul_col_major(a: &[f32; 16], b: &[f32; 16]) -> [f32; 16] {
    let mut out = [0.0f32; 16];
    for col in 0..4 {
        for row in 0..4 {
            let mut sum = 0.0f32;
            for k in 0..4 {
                sum += a[k * 4 + row] * b[col * 4 + k];
            }
            out[col * 4 + row] = sum;
        }
    }
    out
}

fn frame_rotator(frame_index: u32) -> [f32; 4] {
    // Golden-angle rotation per frame for isotropic 8-tap Poisson sampling.
    let angle = (frame_index.wrapping_mul(2654435761) as f32) * (std::f32::consts::TAU / 4294967296.0);
    let (s, c) = angle.sin_cos();
    [c, s, -s, c]
}

fn push_cb<T: Copy>(arena: &mut Vec<u8>, val: &T) -> (usize, u32) {
    let align_rem = arena.len() % 16;
    if align_rem != 0 {
        arena.resize(arena.len() + (16 - align_rem), 0);
    }
    let start = arena.len();
    let len = size_of::<T>();
    let bytes = unsafe { std::slice::from_raw_parts((val as *const T) as *const u8, len) };
    arena.extend_from_slice(bytes);
    (start, len as u32)
}

fn active_rect(desc: NrdDenoiserDesc, cs: &NrdCommonSettings) -> (u32, u32) {
    let rw = ((desc.render_width as f32) * cs.resolution_scale[0] + 0.5) as u32;
    let rh = ((desc.render_height as f32) * cs.resolution_scale[1] + 0.5) as u32;
    (rw.clamp(1, desc.render_width as u32), rh.clamp(1, desc.render_height as u32))
}

fn grid_dim(px: u32) -> u16 {
    px.div_ceil(THREAD_GROUP_DIM).max(1) as u16
}

// ---------------------------------------------------------------------------
// Exported NRD.dll entry points
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn GetLibraryDesc() -> *const NrdLibraryDesc {
    &LIBRARY_DESC
}

pub unsafe extern "win64" fn CreateInstance(
    desc_ptr: *const u8,
    instance_out: *mut *mut NrdInstance,
) -> u32 {
    if desc_ptr.is_null() || instance_out.is_null() {
        return NRD_RESULT_INVALID_ARGUMENT;
    }
    let cd = ptr::read_unaligned(desc_ptr as *const NrdInstanceCreationDesc);
    if cd.denoisers_num > 0 && cd.denoisers.is_null() {
        return NRD_RESULT_INVALID_ARGUMENT;
    }

    let mut pipeline_kinds: Vec<ShaderKind> = Vec::new();
    let mut pipeline_descs: Vec<NrdPipelineDesc> = Vec::new();
    let mut permanent_pool: Vec<NrdTextureDesc> = Vec::new();
    let mut transient_pool: Vec<NrdTextureDesc> = Vec::new();
    let mut denoisers: Vec<DenoiserEntry> = Vec::with_capacity(cd.denoisers_num as usize);

    let mut max_passes_total: u32 = 0;
    let mut max_cbvs_total: u32 = 0;
    let mut max_srvs_total: u32 = 0;
    let mut max_uavs_total: u32 = 0;

    for i in 0..cd.denoisers_num as usize {
        let dd = ptr::read_unaligned(cd.denoisers.add(i));
        if (dd.denoiser as usize) >= NRD_DENOISER_MAX_NUM {
            return NRD_RESULT_UNSUPPORTED;
        }
        if dd.render_width == 0 || dd.render_height == 0 {
            return NRD_RESULT_INVALID_ARGUMENT;
        }
        if denoisers.iter().any(|e| e.desc.identifier == dd.identifier) {
            return NRD_RESULT_NON_UNIQUE_IDENTIFIER;
        }

        let w = dd.render_width;
        let h = dd.render_height;
        let prev_transient_len = transient_pool.len();
        let mut used_transient: Vec<u16> = Vec::new();
        let mut clear_targets: Vec<ClearTarget> = Vec::new();

        let plan = match dd.denoiser {
            NRD_DENOISER_REBLUR_DIFFUSE..=NRD_DENOISER_REBLUR_DIFFUSE_DIRECTIONAL_OCCLUSION => {
                let pipeline_index = get_or_add_pipeline(
                    &mut pipeline_kinds,
                    &mut pipeline_descs,
                    ShaderKind::ReblurFilter,
                );
                let (sig_fmt, sig_specs) = reblur_signals_and_format(dd.denoiser);
                let meta_ping = alloc_permanent(
                    &mut permanent_pool,
                    &mut clear_targets,
                    tex2d(NRD_FORMAT_RGBA32_SFLOAT, w, h),
                );
                let meta_pong = alloc_permanent(
                    &mut permanent_pool,
                    &mut clear_targets,
                    tex2d(NRD_FORMAT_RGBA32_SFLOAT, w, h),
                );
                // Matches NRD's `REBLUR_FORMAT_PREV_INTERNAL_DATA` (`R16_UINT`), cleared via Clear_ui.
                let _internal_data = alloc_permanent(
                    &mut permanent_pool,
                    &mut clear_targets,
                    tex2d(NRD_FORMAT_R16_UINT, w, h),
                );
                let mut signals = Vec::with_capacity(sig_specs.len());
                for &(in_res, out_res, is_specular) in sig_specs {
                    let hist_ping = alloc_permanent(
                        &mut permanent_pool,
                        &mut clear_targets,
                        tex2d(sig_fmt, w, h),
                    );
                    let hist_pong = alloc_permanent(
                        &mut permanent_pool,
                        &mut clear_targets,
                        tex2d(sig_fmt, w, h),
                    );
                    signals.push(SignalBinding {
                        in_res,
                        out_res,
                        is_specular,
                        hist_ping,
                        hist_pong,
                    });
                }
                let transient_scratch = alloc_transient(
                    &mut transient_pool,
                    prev_transient_len,
                    &mut used_transient,
                    &mut clear_targets,
                    tex2d(sig_fmt, w, h),
                );
                let passes = (signals.len() as u32) * 2;
                max_passes_total += passes;
                max_cbvs_total += passes;
                max_srvs_total += passes * 6;
                max_uavs_total += passes * 2;
                DenoiserPlan::Reblur {
                    pipeline_index,
                    meta_ping,
                    meta_pong,
                    transient_scratch,
                    signals,
                }
            }
            NRD_DENOISER_SIGMA_SHADOW..=NRD_DENOISER_SIGMA_SHADOW_TRANSLUCENCY => {
                let pipeline_index = get_or_add_pipeline(
                    &mut pipeline_kinds,
                    &mut pipeline_descs,
                    ShaderKind::SigmaShadow,
                );
                let hist_ping = alloc_permanent(
                    &mut permanent_pool,
                    &mut clear_targets,
                    tex2d(NRD_FORMAT_RGBA16_SFLOAT, w, h),
                );
                let hist_pong = alloc_permanent(
                    &mut permanent_pool,
                    &mut clear_targets,
                    tex2d(NRD_FORMAT_RGBA16_SFLOAT, w, h),
                );
                let transient_blur = alloc_transient(
                    &mut transient_pool,
                    prev_transient_len,
                    &mut used_transient,
                    &mut clear_targets,
                    tex2d(NRD_FORMAT_RGBA16_SFLOAT, w, h),
                );
                let transient_penumbra = alloc_transient(
                    &mut transient_pool,
                    prev_transient_len,
                    &mut used_transient,
                    &mut clear_targets,
                    tex2d(NRD_FORMAT_RG16_SFLOAT, w, h),
                );
                max_passes_total += 2;
                max_cbvs_total += 2;
                max_srvs_total += 12;
                max_uavs_total += 4;
                DenoiserPlan::Sigma {
                    pipeline_index,
                    has_translucency: dd.denoiser == NRD_DENOISER_SIGMA_SHADOW_TRANSLUCENCY,
                    hist_ping,
                    hist_pong,
                    transient_blur,
                    transient_penumbra,
                }
            }
            NRD_DENOISER_RELAX_DIFFUSE..=NRD_DENOISER_RELAX_DIFFUSE_SPECULAR_SH => {
                let pipeline_index = get_or_add_pipeline(
                    &mut pipeline_kinds,
                    &mut pipeline_descs,
                    ShaderKind::RelaxFilter,
                );
                let sig_specs = relax_signals(dd.denoiser);
                let meta_ping = alloc_permanent(
                    &mut permanent_pool,
                    &mut clear_targets,
                    tex2d(NRD_FORMAT_RGBA32_SFLOAT, w, h),
                );
                let meta_pong = alloc_permanent(
                    &mut permanent_pool,
                    &mut clear_targets,
                    tex2d(NRD_FORMAT_RGBA32_SFLOAT, w, h),
                );
                let mut signals = Vec::with_capacity(sig_specs.len());
                for &(in_res, out_res, is_specular) in sig_specs {
                    let hist_ping = alloc_permanent(
                        &mut permanent_pool,
                        &mut clear_targets,
                        tex2d(NRD_FORMAT_RGBA16_SFLOAT, w, h),
                    );
                    let hist_pong = alloc_permanent(
                        &mut permanent_pool,
                        &mut clear_targets,
                        tex2d(NRD_FORMAT_RGBA16_SFLOAT, w, h),
                    );
                    signals.push(SignalBinding {
                        in_res,
                        out_res,
                        is_specular,
                        hist_ping,
                        hist_pong,
                    });
                }
                let transient_ping = alloc_transient(
                    &mut transient_pool,
                    prev_transient_len,
                    &mut used_transient,
                    &mut clear_targets,
                    tex2d(NRD_FORMAT_RGBA16_SFLOAT, w, h),
                );
                let transient_pong = alloc_transient(
                    &mut transient_pool,
                    prev_transient_len,
                    &mut used_transient,
                    &mut clear_targets,
                    tex2d(NRD_FORMAT_RGBA16_SFLOAT, w, h),
                );
                // 1 temporal pass + up to RELAX_MAX_ATROUS_PASSES spatial wavelet passes per signal.
                let passes = (signals.len() as u32) * (1 + RELAX_MAX_ATROUS_PASSES);
                max_passes_total += passes;
                max_cbvs_total += passes;
                max_srvs_total += passes * 6;
                max_uavs_total += passes * 2;
                DenoiserPlan::Relax {
                    pipeline_index,
                    meta_ping,
                    meta_pong,
                    transient_ping,
                    transient_pong,
                    signals,
                }
            }
            NRD_DENOISER_REFERENCE => {
                let pipeline_index = get_or_add_pipeline(
                    &mut pipeline_kinds,
                    &mut pipeline_descs,
                    ShaderKind::ReferenceAccumulate,
                );
                let hist_ping = alloc_permanent(
                    &mut permanent_pool,
                    &mut clear_targets,
                    tex2d(NRD_FORMAT_RGBA32_SFLOAT, w, h),
                );
                let hist_pong = alloc_permanent(
                    &mut permanent_pool,
                    &mut clear_targets,
                    tex2d(NRD_FORMAT_RGBA32_SFLOAT, w, h),
                );
                max_passes_total += 1;
                max_cbvs_total += 1;
                max_srvs_total += 2;
                max_uavs_total += 2;
                DenoiserPlan::Reference { pipeline_index, hist_ping, hist_pong }
            }
            NRD_DENOISER_SPECULAR_REFLECTION_MV => {
                let pipeline_index = get_or_add_pipeline(
                    &mut pipeline_kinds,
                    &mut pipeline_descs,
                    ShaderKind::SpecularMvCompute,
                );
                let transient_aux = alloc_transient(
                    &mut transient_pool,
                    prev_transient_len,
                    &mut used_transient,
                    &mut clear_targets,
                    tex2d(NRD_FORMAT_RG16_SFLOAT, w, h),
                );
                max_passes_total += 1;
                max_cbvs_total += 1;
                max_srvs_total += 4;
                max_uavs_total += 2;
                DenoiserPlan::SpecularReflectionMv { pipeline_index, transient_aux }
            }
            NRD_DENOISER_SPECULAR_DELTA_MV => {
                let pipeline_index = get_or_add_pipeline(
                    &mut pipeline_kinds,
                    &mut pipeline_descs,
                    ShaderKind::SpecularMvCompute,
                );
                let pos_ping = alloc_permanent(
                    &mut permanent_pool,
                    &mut clear_targets,
                    tex2d(NRD_FORMAT_RGBA32_SFLOAT, w, h),
                );
                let pos_pong = alloc_permanent(
                    &mut permanent_pool,
                    &mut clear_targets,
                    tex2d(NRD_FORMAT_RGBA32_SFLOAT, w, h),
                );
                max_passes_total += 1;
                max_cbvs_total += 1;
                max_srvs_total += 4;
                max_uavs_total += 2;
                DenoiserPlan::SpecularDeltaMv { pipeline_index, pos_ping, pos_pong }
            }
            _ => return NRD_RESULT_UNSUPPORTED,
        };

        denoisers.push(DenoiserEntry {
            desc: dd,
            settings: DenoiserSettingsState::default_for(dd.denoiser),
            settings_set: false,
            plan,
            clear_targets,
            ping_pong: 0,
            accumulated_frame_num: 0,
        });
    }

    // Always append Clear_f and Clear_ui pipelines (matches `InstanceImpl::Create`).
    let clear_float_pipeline =
        get_or_add_pipeline(&mut pipeline_kinds, &mut pipeline_descs, ShaderKind::ClearFloat);
    let clear_uint_pipeline =
        get_or_add_pipeline(&mut pipeline_kinds, &mut pipeline_descs, ShaderKind::ClearUint);

    let max_cb_size = pipeline_kinds.iter().map(|k| k.cb_size()).max().unwrap_or(0).max(256);
    let clear_num = (permanent_pool.len() + transient_pool.len()) as u32;
    let sets_max_num = (max_passes_total + clear_num).max(1);
    let constant_buffers_max_num = max_cbvs_total.max(1);
    let samplers_max_num = sets_max_num * SAMPLERS.len() as u32;
    let textures_max_num = max_srvs_total.max(1);
    let storage_textures_max_num = (max_uavs_total + clear_num).max(1);

    let pipelines_box = pipeline_descs.into_boxed_slice();
    let permanent_box = permanent_pool.into_boxed_slice();
    let transient_box = transient_pool.into_boxed_slice();

    let instance_desc = NrdInstanceDesc {
        constant_buffer_max_data_size: max_cb_size,
        constant_buffer_space_index: 0,
        constant_buffer_register_index: 0,
        _pad0: 0,
        samplers: SAMPLERS.as_ptr(),
        samplers_num: SAMPLERS.len() as u32,
        samplers_space_index: 0,
        samplers_base_register_index: 0,
        _pad1: 0,
        pipelines: if pipelines_box.is_empty() { ptr::null() } else { pipelines_box.as_ptr() },
        pipelines_num: pipelines_box.len() as u32,
        resources_space_index: 0,
        permanent_pool: if permanent_box.is_empty() { ptr::null() } else { permanent_box.as_ptr() },
        permanent_pool_size: permanent_box.len() as u32,
        _pad2: 0,
        transient_pool: if transient_box.is_empty() { ptr::null() } else { transient_box.as_ptr() },
        transient_pool_size: transient_box.len() as u32,
        descriptor_pool_desc: NrdDescriptorPoolDesc {
            sets_max_num,
            constant_buffers_max_num,
            samplers_max_num,
            textures_max_num,
            storage_textures_max_num,
        },
    };

    let instance = Box::new(NrdInstance {
        desc: instance_desc,
        _pipelines: pipelines_box,
        _permanent_pool: permanent_box,
        _transient_pool: transient_box,
        clear_float_pipeline,
        clear_uint_pipeline,
        denoisers,
        common_settings: NrdCommonSettings::default(),
        is_first_use: true,
        dispatch_resources: Vec::new(),
        constant_buffer_arena: Vec::new(),
        active_dispatches: Vec::new(),
    });
    ptr::write_unaligned(instance_out, Box::into_raw(instance));
    NRD_RESULT_SUCCESS
}

pub unsafe extern "win64" fn DestroyInstance(instance: *mut NrdInstance) {
    if !instance.is_null() {
        drop(Box::from_raw(instance));
    }
}

pub unsafe extern "win64" fn GetInstanceDesc(instance: *const NrdInstance) -> *const NrdInstanceDesc {
    if instance.is_null() {
        &EMPTY_INSTANCE_DESC
    } else {
        &(*instance).desc
    }
}

pub unsafe extern "win64" fn SetCommonSettings(
    instance: *mut NrdInstance,
    settings_ptr: *const u8,
) -> u32 {
    if instance.is_null() || settings_ptr.is_null() {
        return NRD_RESULT_INVALID_ARGUMENT;
    }
    let mut cs = ptr::read_unaligned(settings_ptr as *const NrdCommonSettings);
    if !cs.denoising_range.is_finite() || cs.denoising_range <= 0.0 {
        return NRD_RESULT_INVALID_ARGUMENT;
    }
    if cs.resolution_scale[0] <= 0.0
        || cs.resolution_scale[0] > 1.0
        || cs.resolution_scale[1] <= 0.0
        || cs.resolution_scale[1] > 1.0
    {
        return NRD_RESULT_INVALID_ARGUMENT;
    }
    if cs.accumulation_mode > NRD_ACCUMULATION_CLEAR_AND_RESTART {
        return NRD_RESULT_INVALID_ARGUMENT;
    }
    let inst = &mut *instance;
    if inst.is_first_use {
        cs.accumulation_mode = NRD_ACCUMULATION_CLEAR_AND_RESTART;
        cs.world_to_view_matrix_prev = cs.world_to_view_matrix;
        cs.view_to_clip_matrix_prev = cs.view_to_clip_matrix;
        inst.is_first_use = false;
    }
    inst.common_settings = cs;
    NRD_RESULT_SUCCESS
}

pub unsafe extern "win64" fn SetDenoiserSettings(
    instance: *mut NrdInstance,
    identifier: u32,
    settings_ptr: *const u8,
) -> u32 {
    if instance.is_null() || settings_ptr.is_null() {
        return NRD_RESULT_INVALID_ARGUMENT;
    }
    let inst = &mut *instance;
    let Some(entry) = inst.denoisers.iter_mut().find(|d| d.desc.identifier == identifier) else {
        return NRD_RESULT_INVALID_ARGUMENT;
    };
    let parsed = match entry.desc.denoiser {
        NRD_DENOISER_REBLUR_DIFFUSE..=NRD_DENOISER_REBLUR_DIFFUSE_DIRECTIONAL_OCCLUSION => {
            let s = ptr::read_unaligned(settings_ptr as *const NrdReblurSettings);
            if s.checkerboard_mode >= 3 || s.hit_distance_reconstruction_mode >= 3 {
                return NRD_RESULT_INVALID_ARGUMENT;
            }
            DenoiserSettingsState::Reblur(s)
        }
        NRD_DENOISER_SIGMA_SHADOW..=NRD_DENOISER_SIGMA_SHADOW_TRANSLUCENCY => {
            let s = ptr::read_unaligned(settings_ptr as *const NrdSigmaSettings);
            if !s.plane_distance_sensitivity.is_finite() || !s.blur_radius_scale.is_finite() {
                return NRD_RESULT_INVALID_ARGUMENT;
            }
            DenoiserSettingsState::Sigma(s)
        }
        NRD_DENOISER_RELAX_DIFFUSE..=NRD_DENOISER_RELAX_DIFFUSE_SH => {
            let s = ptr::read_unaligned(settings_ptr as *const NrdRelaxDiffuseSettings);
            if s.checkerboard_mode >= 3 || s.hit_distance_reconstruction_mode >= 3 {
                return NRD_RESULT_INVALID_ARGUMENT;
            }
            DenoiserSettingsState::RelaxDiffuse(s)
        }
        NRD_DENOISER_RELAX_SPECULAR..=NRD_DENOISER_RELAX_SPECULAR_SH => {
            let s = ptr::read_unaligned(settings_ptr as *const NrdRelaxSpecularSettings);
            if s.checkerboard_mode >= 3 || s.hit_distance_reconstruction_mode >= 3 {
                return NRD_RESULT_INVALID_ARGUMENT;
            }
            DenoiserSettingsState::RelaxSpecular(s)
        }
        NRD_DENOISER_RELAX_DIFFUSE_SPECULAR..=NRD_DENOISER_RELAX_DIFFUSE_SPECULAR_SH => {
            let s = ptr::read_unaligned(settings_ptr as *const NrdRelaxDiffuseSpecularSettings);
            if s.checkerboard_mode >= 3 || s.hit_distance_reconstruction_mode >= 3 {
                return NRD_RESULT_INVALID_ARGUMENT;
            }
            DenoiserSettingsState::RelaxDiffuseSpecular(s)
        }
        NRD_DENOISER_REFERENCE => {
            let s = ptr::read_unaligned(settings_ptr as *const NrdReferenceSettings);
            DenoiserSettingsState::Reference(s)
        }
        NRD_DENOISER_SPECULAR_REFLECTION_MV..=NRD_DENOISER_SPECULAR_DELTA_MV => {
            DenoiserSettingsState::MotionVector
        }
        _ => return NRD_RESULT_INVALID_ARGUMENT,
    };
    entry.settings = parsed;
    entry.settings_set = true;
    NRD_RESULT_SUCCESS
}

pub unsafe extern "win64" fn GetComputeDispatches(
    instance: *mut NrdInstance,
    identifiers: *const u32,
    identifiers_num: u32,
    dispatch_descs_out: *mut *const u8,
    dispatch_descs_num_out: *mut u32,
) -> u32 {
    if !dispatch_descs_out.is_null() {
        ptr::write_unaligned(dispatch_descs_out, ptr::null());
    }
    if !dispatch_descs_num_out.is_null() {
        ptr::write_unaligned(dispatch_descs_num_out, 0);
    }
    if instance.is_null() || dispatch_descs_out.is_null() || dispatch_descs_num_out.is_null() {
        return NRD_RESULT_INVALID_ARGUMENT;
    }
    if identifiers_num == 0 {
        return NRD_RESULT_SUCCESS;
    }
    if identifiers.is_null() {
        return NRD_RESULT_INVALID_ARGUMENT;
    }

    let mut req_ids = Vec::with_capacity(identifiers_num as usize);
    for i in 0..identifiers_num as usize {
        req_ids.push(ptr::read_unaligned(identifiers.add(i)));
    }

    let inst = &mut *instance;
    inst.active_dispatches.clear();
    inst.dispatch_resources.clear();
    inst.constant_buffer_arena.clear();

    let cs = inst.common_settings;
    let reset_history = cs.accumulation_mode != NRD_ACCUMULATION_CONTINUE;
    let world_space_mv = cs.is_motion_vector_in_world_space != 0;
    let mut pending: Vec<PendingDispatch> = Vec::new();

    // 1. Emit Clear_f / Clear_ui passes for all pool textures used by requested denoisers
    //    when accumulation mode is CLEAR_AND_RESTART.
    if cs.accumulation_mode == NRD_ACCUMULATION_CLEAR_AND_RESTART {
        let mut cleared: Vec<(u32, u16)> = Vec::new();
        for entry in &inst.denoisers {
            if !req_ids.contains(&entry.desc.identifier) {
                continue;
            }
            for target in &entry.clear_targets {
                let key = (target.resource_type, target.index_in_pool);
                if cleared.contains(&key) {
                    continue;
                }
                cleared.push(key);
                let res_start = inst.dispatch_resources.len();
                inst.dispatch_resources.push(res_uav(target.resource_type, target.index_in_pool));
                pending.push(PendingDispatch {
                    name: if target.is_integer { b"Clear (ui)\0" } else { b"Clear (f)\0" },
                    res_start,
                    res_len: 1,
                    cb_start: 0,
                    cb_len: 0,
                    pipeline_index: if target.is_integer {
                        inst.clear_uint_pipeline
                    } else {
                        inst.clear_float_pipeline
                    },
                    grid_width: grid_dim(target.width as u32),
                    grid_height: grid_dim(target.height as u32),
                });
            }
        }
    }

    // 2. Emit per-denoiser compute dispatches.
    let rotator = frame_rotator(cs.frame_index);
    let mut matched_any = false;

    for entry in &mut inst.denoisers {
        if !req_ids.contains(&entry.desc.identifier) {
            continue;
        }
        matched_any = true;

        let (rect_w, rect_h) = active_rect(entry.desc, &cs);
        let inv_rect = [1.0 / rect_w as f32, 1.0 / rect_h as f32];
        let grid_w = grid_dim(rect_w);
        let grid_h = grid_dim(rect_h);
        let parity = entry.ping_pong;

        match &entry.plan {
            DenoiserPlan::Reblur {
                pipeline_index,
                meta_ping,
                meta_pong,
                transient_scratch,
                signals,
            } => {
                let settings = match entry.settings {
                    DenoiserSettingsState::Reblur(s) => s,
                    _ => NrdReblurSettings::default(),
                };
                let (meta_read, meta_write) =
                    if parity == 0 { (*meta_ping, *meta_pong) } else { (*meta_pong, *meta_ping) };
                let base_flags = (if reset_history { 1u32 } else { 0 })
                    | (if world_space_mv { 2u32 } else { 0 })
                    | (if settings.enable_anti_firefly != 0 { 4u32 } else { 0 });

                for sig in signals {
                    let (hist_read, hist_write) = if parity == 0 {
                        (sig.hist_ping, sig.hist_pong)
                    } else {
                        (sig.hist_pong, sig.hist_ping)
                    };
                    let sig_flags = base_flags | if sig.is_specular { 8u32 } else { 0 };
                    let prepass_radius = if sig.is_specular {
                        settings.specular_prepass_blur_radius
                    } else {
                        settings.diffuse_prepass_blur_radius
                    };
                    let blur_radius = settings.blur_radius.max(prepass_radius * 0.25);

                    // Pass 0: Temporal accumulation -> TRANSIENT[scratch] + PERMANENT[meta_write]
                    let cb_temporal = ReblurCb {
                        rect_origin: cs.input_subrect_origin,
                        rect_size: [rect_w, rect_h],
                        inv_rect_size: inv_rect,
                        resolution_scale: cs.resolution_scale,
                        mv_scale: cs.motion_vector_scale,
                        denoising_range: cs.denoising_range,
                        blur_radius,
                        max_accum_frames: settings.max_accumulated_frame_num.max(1) as f32,
                        plane_dist_sens: settings.plane_distance_sensitivity,
                        disocclusion_thresh: cs.disocclusion_threshold,
                        stabilization: settings.stabilization_strength,
                        split_screen: cs.split_screen,
                        pass_mode: 0,
                        flags: sig_flags,
                        rotator,
                        hit_dist_params: [
                            settings.hit_distance_parameters.a,
                            settings.hit_distance_parameters.b,
                            settings.hit_distance_parameters.c,
                            settings.hit_distance_parameters.d,
                        ],
                        lobe_params: [
                            settings.lobe_angle_fraction,
                            settings.roughness_fraction,
                            settings.antilag_settings.luminance_sigma_scale,
                            cs.frame_index as f32,
                        ],
                    };
                    let (cb_start, cb_len) = push_cb(&mut inst.constant_buffer_arena, &cb_temporal);
                    let res_start = inst.dispatch_resources.len();
                    inst.dispatch_resources.extend_from_slice(&[
                        res_srv(sig.in_res, 0),
                        res_srv(NRD_RES_IN_MV, 0),
                        res_srv(NRD_RES_IN_NORMAL_ROUGHNESS, 0),
                        res_srv(NRD_RES_IN_VIEWZ, 0),
                        res_srv(NRD_RES_PERMANENT_POOL, hist_read),
                        res_srv(NRD_RES_PERMANENT_POOL, meta_read),
                        res_uav(NRD_RES_TRANSIENT_POOL, *transient_scratch),
                        res_uav(NRD_RES_PERMANENT_POOL, meta_write),
                    ]);
                    pending.push(PendingDispatch {
                        name: b"REBLUR - Temporal accumulation\0",
                        res_start,
                        res_len: 8,
                        cb_start,
                        cb_len,
                        pipeline_index: *pipeline_index,
                        grid_width: grid_w,
                        grid_height: grid_h,
                    });

                    // Pass 1: Spatial edge-aware filter -> OUT + PERMANENT[hist_write]
                    let mut cb_spatial = cb_temporal;
                    cb_spatial.pass_mode = 1;
                    let (cb_start, cb_len) = push_cb(&mut inst.constant_buffer_arena, &cb_spatial);
                    let res_start = inst.dispatch_resources.len();
                    inst.dispatch_resources.extend_from_slice(&[
                        res_srv(NRD_RES_TRANSIENT_POOL, *transient_scratch),
                        res_srv(NRD_RES_IN_MV, 0),
                        res_srv(NRD_RES_IN_NORMAL_ROUGHNESS, 0),
                        res_srv(NRD_RES_IN_VIEWZ, 0),
                        res_srv(sig.in_res, 0),
                        res_srv(NRD_RES_PERMANENT_POOL, meta_write),
                        res_uav(sig.out_res, 0),
                        res_uav(NRD_RES_PERMANENT_POOL, hist_write),
                    ]);
                    pending.push(PendingDispatch {
                        name: b"REBLUR - Spatial filter\0",
                        res_start,
                        res_len: 8,
                        cb_start,
                        cb_len,
                        pipeline_index: *pipeline_index,
                        grid_width: grid_w,
                        grid_height: grid_h,
                    });
                }
            }
            DenoiserPlan::Relax {
                pipeline_index,
                meta_ping,
                meta_pong,
                transient_ping,
                transient_pong,
                signals,
            } => {
                let (meta_read, meta_write) =
                    if parity == 0 { (*meta_ping, *meta_pong) } else { (*meta_pong, *meta_ping) };

                for sig in signals {
                    let (hist_read, hist_write) = if parity == 0 {
                        (sig.hist_ping, sig.hist_pong)
                    } else {
                        (sig.hist_pong, sig.hist_ping)
                    };

                    let (
                        phi_luma,
                        depth_thresh,
                        max_frames,
                        lobe_frac,
                        rough_frac,
                        color_box_scale,
                        atrous_iters,
                        anti_firefly,
                        rough_edge_stop,
                        relax_params,
                        antilag_params,
                    ) = match entry.settings {
                        DenoiserSettingsState::RelaxDiffuse(s) => (
                            s.diffuse_phi_luminance,
                            s.depth_threshold,
                            s.diffuse_max_accumulated_frame_num,
                            s.diffuse_lobe_angle_fraction,
                            0.15f32,
                            s.history_clamping_color_box_sigma_scale,
                            s.atrous_iteration_num,
                            s.enable_anti_firefly != 0,
                            false,
                            [0.5f32, 0.3, 1.0, s.min_luminance_weight],
                            [
                                s.antilag_settings.acceleration_amount,
                                s.antilag_settings.spatial_sigma_scale,
                                s.antilag_settings.temporal_sigma_scale,
                                s.antilag_settings.reset_amount,
                            ],
                        ),
                        DenoiserSettingsState::RelaxSpecular(s) => (
                            s.specular_phi_luminance,
                            s.depth_threshold,
                            s.specular_max_accumulated_frame_num,
                            s.specular_lobe_angle_fraction,
                            s.roughness_fraction,
                            s.history_clamping_color_box_sigma_scale,
                            s.atrous_iteration_num,
                            s.enable_anti_firefly != 0,
                            s.enable_roughness_edge_stopping != 0,
                            [
                                s.luminance_edge_stopping_relaxation,
                                s.normal_edge_stopping_relaxation,
                                s.roughness_edge_stopping_relaxation,
                                s.min_luminance_weight,
                            ],
                            [
                                s.antilag_settings.acceleration_amount,
                                s.antilag_settings.spatial_sigma_scale,
                                s.antilag_settings.temporal_sigma_scale,
                                s.antilag_settings.reset_amount,
                            ],
                        ),
                        DenoiserSettingsState::RelaxDiffuseSpecular(s) => (
                            if sig.is_specular {
                                s.specular_phi_luminance
                            } else {
                                s.diffuse_phi_luminance
                            },
                            s.depth_threshold,
                            if sig.is_specular {
                                s.specular_max_accumulated_frame_num
                            } else {
                                s.diffuse_max_accumulated_frame_num
                            },
                            if sig.is_specular {
                                s.specular_lobe_angle_fraction
                            } else {
                                s.diffuse_lobe_angle_fraction
                            },
                            s.roughness_fraction,
                            s.history_clamping_color_box_sigma_scale,
                            s.atrous_iteration_num,
                            s.enable_anti_firefly != 0,
                            s.enable_roughness_edge_stopping != 0,
                            [
                                s.luminance_edge_stopping_relaxation,
                                s.normal_edge_stopping_relaxation,
                                s.roughness_edge_stopping_relaxation,
                                if sig.is_specular {
                                    s.specular_min_luminance_weight
                                } else {
                                    s.diffuse_min_luminance_weight
                                },
                            ],
                            [
                                s.antilag_settings.acceleration_amount,
                                s.antilag_settings.spatial_sigma_scale,
                                s.antilag_settings.temporal_sigma_scale,
                                s.antilag_settings.reset_amount,
                            ],
                        ),
                        _ => (
                            2.0,
                            0.01,
                            30,
                            0.5,
                            0.15,
                            2.0,
                            5,
                            false,
                            true,
                            [0.5, 0.3, 1.0, 0.0],
                            [0.3, 4.5, 0.5, 0.5],
                        ),
                    };

                    let flags = (if reset_history { 1u32 } else { 0 })
                        | (if world_space_mv { 2u32 } else { 0 })
                        | (if anti_firefly { 4u32 } else { 0 })
                        | (if rough_edge_stop { 8u32 } else { 0 });

                    // Pass 0: Temporal accumulation + history clamping -> TRANSIENT[ping] + PERMANENT[meta_write]
                    let cb_temporal = RelaxCb {
                        rect_origin: cs.input_subrect_origin,
                        rect_size: [rect_w, rect_h],
                        inv_rect_size: inv_rect,
                        resolution_scale: cs.resolution_scale,
                        mv_scale: cs.motion_vector_scale,
                        denoising_range: cs.denoising_range,
                        phi_luminance: phi_luma,
                        depth_threshold: depth_thresh,
                        max_accum_frames: max_frames.max(1) as f32,
                        disocclusion_thresh: cs.disocclusion_threshold,
                        lobe_angle_fraction: lobe_frac,
                        roughness_fraction: rough_frac,
                        color_box_sigma_scale: color_box_scale,
                        split_screen: cs.split_screen,
                        pass_mode: 0,
                        step_size: 1,
                        flags,
                        frame_index: cs.frame_index,
                        relaxation_params: relax_params,
                        antilag_params,
                    };
                    let (cb_start, cb_len) = push_cb(&mut inst.constant_buffer_arena, &cb_temporal);
                    let res_start = inst.dispatch_resources.len();
                    inst.dispatch_resources.extend_from_slice(&[
                        res_srv(sig.in_res, 0),
                        res_srv(NRD_RES_IN_MV, 0),
                        res_srv(NRD_RES_IN_NORMAL_ROUGHNESS, 0),
                        res_srv(NRD_RES_IN_VIEWZ, 0),
                        res_srv(NRD_RES_PERMANENT_POOL, hist_read),
                        res_srv(NRD_RES_PERMANENT_POOL, meta_read),
                        res_uav(NRD_RES_TRANSIENT_POOL, *transient_ping),
                        res_uav(NRD_RES_PERMANENT_POOL, meta_write),
                    ]);
                    pending.push(PendingDispatch {
                        name: b"RELAX - Temporal accumulation\0",
                        res_start,
                        res_len: 8,
                        cb_start,
                        cb_len,
                        pipeline_index: *pipeline_index,
                        grid_width: grid_w,
                        grid_height: grid_h,
                    });

                    // Passes 1..=K: À-Trous spatial wavelet iterations (clamped 2..=RELAX_MAX_ATROUS_PASSES)
                    let iters = atrous_iters.clamp(2, RELAX_MAX_ATROUS_PASSES);
                    let mut src_t = *transient_ping;
                    let mut dst_t = *transient_pong;
                    for iter in 0..iters {
                        let is_last = iter + 1 == iters;
                        let mut cb_atrous = cb_temporal;
                        cb_atrous.pass_mode = if is_last { 2 } else { 1 };
                        cb_atrous.step_size = 1u32 << iter;
                        let (cb_start, cb_len) =
                            push_cb(&mut inst.constant_buffer_arena, &cb_atrous);
                        let res_start = inst.dispatch_resources.len();
                        let u0 = if is_last {
                            res_uav(sig.out_res, 0)
                        } else {
                            res_uav(NRD_RES_TRANSIENT_POOL, dst_t)
                        };
                        inst.dispatch_resources.extend_from_slice(&[
                            res_srv(NRD_RES_TRANSIENT_POOL, src_t),
                            res_srv(NRD_RES_IN_MV, 0),
                            res_srv(NRD_RES_IN_NORMAL_ROUGHNESS, 0),
                            res_srv(NRD_RES_IN_VIEWZ, 0),
                            res_srv(sig.in_res, 0),
                            res_srv(NRD_RES_PERMANENT_POOL, meta_write),
                            u0,
                            res_uav(NRD_RES_PERMANENT_POOL, hist_write),
                        ]);
                        pending.push(PendingDispatch {
                            name: b"RELAX - A-trous\0",
                            res_start,
                            res_len: 8,
                            cb_start,
                            cb_len,
                            pipeline_index: *pipeline_index,
                            grid_width: grid_w,
                            grid_height: grid_h,
                        });
                        std::mem::swap(&mut src_t, &mut dst_t);
                    }
                }
            }
            DenoiserPlan::Sigma {
                pipeline_index,
                has_translucency,
                hist_ping,
                hist_pong,
                transient_blur,
                transient_penumbra,
            } => {
                let settings = match entry.settings {
                    DenoiserSettingsState::Sigma(s) => s,
                    _ => NrdSigmaSettings::default(),
                };
                let (hist_read, hist_write) =
                    if parity == 0 { (*hist_ping, *hist_pong) } else { (*hist_pong, *hist_ping) };
                let trans_res = if *has_translucency {
                    NRD_RES_IN_SHADOW_TRANSLUCENCY
                } else {
                    NRD_RES_IN_SHADOWDATA
                };
                let base_flags = (if reset_history { 1u32 } else { 0 })
                    | (if *has_translucency { 2u32 } else { 0 })
                    | (if world_space_mv { 8u32 } else { 0 });

                // Pass 0: Penumbra spatial blur -> TRANSIENT[blur] + TRANSIENT[penumbra]
                let cb_blur = SigmaCb {
                    rect_origin: cs.input_subrect_origin,
                    rect_size: [rect_w, rect_h],
                    inv_rect_size: inv_rect,
                    resolution_scale: cs.resolution_scale,
                    mv_scale: cs.motion_vector_scale,
                    denoising_range: cs.denoising_range,
                    plane_dist_sens: settings.plane_distance_sensitivity,
                    blur_radius_scale: settings.blur_radius_scale,
                    split_screen: cs.split_screen,
                    flags: base_flags,
                };
                let (cb_start, cb_len) = push_cb(&mut inst.constant_buffer_arena, &cb_blur);
                let res_start = inst.dispatch_resources.len();
                inst.dispatch_resources.extend_from_slice(&[
                    res_srv(NRD_RES_IN_SHADOWDATA, 0),
                    res_srv(trans_res, 0),
                    res_srv(NRD_RES_IN_MV, 0),
                    res_srv(NRD_RES_IN_NORMAL_ROUGHNESS, 0),
                    res_srv(NRD_RES_IN_VIEWZ, 0),
                    res_srv(NRD_RES_PERMANENT_POOL, hist_read),
                    res_uav(NRD_RES_TRANSIENT_POOL, *transient_blur),
                    res_uav(NRD_RES_TRANSIENT_POOL, *transient_penumbra),
                ]);
                pending.push(PendingDispatch {
                    name: b"SIGMA - Blur\0",
                    res_start,
                    res_len: 8,
                    cb_start,
                    cb_len,
                    pipeline_index: *pipeline_index,
                    grid_width: grid_w,
                    grid_height: grid_h,
                });

                // Pass 1: Temporal stabilization -> OUT_SHADOW_TRANSLUCENCY + PERMANENT[hist_write]
                let mut cb_temp = cb_blur;
                cb_temp.flags = base_flags | 4u32;
                let (cb_start, cb_len) = push_cb(&mut inst.constant_buffer_arena, &cb_temp);
                let res_start = inst.dispatch_resources.len();
                inst.dispatch_resources.extend_from_slice(&[
                    res_srv(NRD_RES_TRANSIENT_POOL, *transient_blur),
                    res_srv(trans_res, 0),
                    res_srv(NRD_RES_IN_MV, 0),
                    res_srv(NRD_RES_IN_NORMAL_ROUGHNESS, 0),
                    res_srv(NRD_RES_IN_VIEWZ, 0),
                    res_srv(NRD_RES_PERMANENT_POOL, hist_read),
                    res_uav(NRD_RES_OUT_SHADOW_TRANSLUCENCY, 0),
                    res_uav(NRD_RES_PERMANENT_POOL, hist_write),
                ]);
                pending.push(PendingDispatch {
                    name: b"SIGMA - Temporal stabilization\0",
                    res_start,
                    res_len: 8,
                    cb_start,
                    cb_len,
                    pipeline_index: *pipeline_index,
                    grid_width: grid_w,
                    grid_height: grid_h,
                });
            }
            DenoiserPlan::Reference { pipeline_index, hist_ping, hist_pong } => {
                let settings = match entry.settings {
                    DenoiserSettingsState::Reference(s) => s,
                    _ => NrdReferenceSettings::default(),
                };
                let camera_moved = cs.view_to_clip_matrix != cs.view_to_clip_matrix_prev
                    || cs.world_to_view_matrix != cs.world_to_view_matrix_prev;
                if reset_history || camera_moved {
                    entry.accumulated_frame_num = 0;
                } else {
                    entry.accumulated_frame_num =
                        (entry.accumulated_frame_num + 1).min(settings.max_accumulated_frame_num);
                }
                let (hist_read, hist_write) =
                    if parity == 0 { (*hist_ping, *hist_pong) } else { (*hist_pong, *hist_ping) };
                let accum_speed = 1.0 / (1.0 + entry.accumulated_frame_num as f32);
                let cb = ReferenceCb {
                    rect_origin: cs.input_subrect_origin,
                    rect_size: [rect_w, rect_h],
                    inv_rect_size: inv_rect,
                    split_screen: cs.split_screen,
                    accum_speed,
                    debug: cs.debug,
                    reset_history: if reset_history || camera_moved { 1 } else { 0 },
                    frame_index: cs.frame_index,
                    _pad: 0,
                };
                let (cb_start, cb_len) = push_cb(&mut inst.constant_buffer_arena, &cb);
                let res_start = inst.dispatch_resources.len();
                inst.dispatch_resources.extend_from_slice(&[
                    res_srv(NRD_RES_IN_RADIANCE, 0),
                    res_srv(NRD_RES_PERMANENT_POOL, hist_read),
                    res_uav(NRD_RES_OUT_RADIANCE, 0),
                    res_uav(NRD_RES_PERMANENT_POOL, hist_write),
                ]);
                pending.push(PendingDispatch {
                    name: b"REFERENCE - Temporal accumulation\0",
                    res_start,
                    res_len: 4,
                    cb_start,
                    cb_len,
                    pipeline_index: *pipeline_index,
                    grid_width: grid_w,
                    grid_height: grid_h,
                });
            }
            DenoiserPlan::SpecularReflectionMv { pipeline_index, transient_aux } => {
                let w2c = mat4_mul_col_major(&cs.view_to_clip_matrix, &cs.world_to_view_matrix);
                let w2c_prev =
                    mat4_mul_col_major(&cs.view_to_clip_matrix_prev, &cs.world_to_view_matrix_prev);
                let cb = SpecularMvCb {
                    world_to_clip: w2c,
                    world_to_clip_prev: w2c_prev,
                    rect_origin: cs.input_subrect_origin,
                    rect_size: [rect_w, rect_h],
                    inv_rect_size: inv_rect,
                    resolution_scale: cs.resolution_scale,
                    mv_scale: cs.motion_vector_scale,
                    denoising_range: cs.denoising_range,
                    mode: 0,
                    is_world_space_mv: if world_space_mv { 1 } else { 0 },
                    frame_index: cs.frame_index,
                    _pad: 0,
                };
                let (cb_start, cb_len) = push_cb(&mut inst.constant_buffer_arena, &cb);
                let res_start = inst.dispatch_resources.len();
                inst.dispatch_resources.extend_from_slice(&[
                    res_srv(NRD_RES_IN_MV, 0),
                    res_srv(NRD_RES_IN_NORMAL_ROUGHNESS, 0),
                    res_srv(NRD_RES_IN_VIEWZ, 0),
                    res_srv(NRD_RES_IN_SPEC_HITDIST, 0),
                    res_uav(NRD_RES_OUT_REFLECTION_MV, 0),
                    res_uav(NRD_RES_TRANSIENT_POOL, *transient_aux),
                ]);
                pending.push(PendingDispatch {
                    name: b"SpecularReflectionMv - Compute\0",
                    res_start,
                    res_len: 6,
                    cb_start,
                    cb_len,
                    pipeline_index: *pipeline_index,
                    grid_width: grid_w,
                    grid_height: grid_h,
                });
            }
            DenoiserPlan::SpecularDeltaMv { pipeline_index, pos_ping, pos_pong } => {
                let (pos_read, pos_write) =
                    if parity == 0 { (*pos_ping, *pos_pong) } else { (*pos_pong, *pos_ping) };
                let w2c = mat4_mul_col_major(&cs.view_to_clip_matrix, &cs.world_to_view_matrix);
                let w2c_prev =
                    mat4_mul_col_major(&cs.view_to_clip_matrix_prev, &cs.world_to_view_matrix_prev);
                let cb = SpecularMvCb {
                    world_to_clip: w2c,
                    world_to_clip_prev: w2c_prev,
                    rect_origin: cs.input_subrect_origin,
                    rect_size: [rect_w, rect_h],
                    inv_rect_size: inv_rect,
                    resolution_scale: cs.resolution_scale,
                    mv_scale: cs.motion_vector_scale,
                    denoising_range: cs.denoising_range,
                    mode: 1,
                    is_world_space_mv: if world_space_mv { 1 } else { 0 },
                    frame_index: cs.frame_index,
                    _pad: 0,
                };
                let (cb_start, cb_len) = push_cb(&mut inst.constant_buffer_arena, &cb);
                let res_start = inst.dispatch_resources.len();
                inst.dispatch_resources.extend_from_slice(&[
                    res_srv(NRD_RES_IN_MV, 0),
                    res_srv(NRD_RES_IN_DELTA_PRIMARY_POS, 0),
                    res_srv(NRD_RES_IN_DELTA_SECONDARY_POS, 0),
                    res_srv(NRD_RES_PERMANENT_POOL, pos_read),
                    res_uav(NRD_RES_OUT_DELTA_MV, 0),
                    res_uav(NRD_RES_PERMANENT_POOL, pos_write),
                ]);
                pending.push(PendingDispatch {
                    name: b"SpecularDeltaMv - Compute\0",
                    res_start,
                    res_len: 6,
                    cb_start,
                    cb_len,
                    pipeline_index: *pipeline_index,
                    grid_width: grid_w,
                    grid_height: grid_h,
                });
            }
        }

        entry.ping_pong ^= 1;
    }

    if !matched_any {
        inst.dispatch_resources.clear();
        inst.constant_buffer_arena.clear();
        return NRD_RESULT_INVALID_ARGUMENT;
    }

    // 3. Finalize dispatch pointers now that `dispatch_resources` and `constant_buffer_arena`
    //    will no longer reallocate until the next `GetComputeDispatches` call.
    let res_base = inst.dispatch_resources.as_ptr();
    let cb_base = inst.constant_buffer_arena.as_ptr();
    inst.active_dispatches.reserve(pending.len());
    for p in pending {
        inst.active_dispatches.push(NrdDispatchDesc {
            name: p.name.as_ptr(),
            resources: if p.res_len == 0 { ptr::null() } else { res_base.add(p.res_start) },
            resources_num: p.res_len as u32,
            _pad0: 0,
            constant_buffer_data: if p.cb_len == 0 { ptr::null() } else { cb_base.add(p.cb_start) },
            constant_buffer_data_size: p.cb_len,
            pipeline_index: p.pipeline_index,
            grid_width: p.grid_width,
            grid_height: p.grid_height,
            _pad1: [0; 6],
        });
    }

    ptr::write_unaligned(dispatch_descs_out, inst.active_dispatches.as_ptr() as *const u8);
    ptr::write_unaligned(dispatch_descs_num_out, inst.active_dispatches.len() as u32);
    NRD_RESULT_SUCCESS
}

pub unsafe extern "win64" fn GetResourceTypeString(resource_type: u32) -> *const u8 {
    RESOURCE_TYPE_NAMES
        .get(resource_type as usize)
        .map_or(ptr::null(), |s| s.as_ptr())
}

pub unsafe extern "win64" fn GetDenoiserString(denoiser: u32) -> *const u8 {
    DENOISER_NAMES
        .get(denoiser as usize)
        .map_or(ptr::null(), |s| s.as_ptr())
}

pub mod exports {
    pub use super::{
        CreateInstance, DestroyInstance, GetComputeDispatches, GetDenoiserString,
        GetInstanceDesc, GetLibraryDesc, GetResourceTypeString, SetCommonSettings,
        SetDenoiserSettings,
    };
}

// ---------------------------------------------------------------------------
// Embedded signed DXBC+DXIL cs_6_0 compute shaders (compiled offline via DXC)
// ---------------------------------------------------------------------------

mod shaders {
    pub const CLEAR_F_DXIL: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/misc/nrd_shaders/clear_f.dxil"));
    pub const CLEAR_UI_DXIL: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/misc/nrd_shaders/clear_ui.dxil"));
    pub const REBLUR_FILTER_DXIL: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/misc/nrd_shaders/reblur_filter.dxil"));
    pub const RELAX_FILTER_DXIL: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/misc/nrd_shaders/relax_filter.dxil"));
    pub const SIGMA_SHADOW_DXIL: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/misc/nrd_shaders/sigma_shadow.dxil"));
    pub const REFERENCE_ACCUMULATE_DXIL: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/misc/nrd_shaders/reference_accumulate.dxil"));
    pub const SPECULAR_MV_COMPUTE_DXIL: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/misc/nrd_shaders/specular_mv_compute.dxil"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    fn verify_dxbc_dxil_blob(bytes: &[u8], label: &str) {
        assert!(bytes.len() >= 32, "{label}: container too small ({})", bytes.len());
        assert_eq!(&bytes[0..4], b"DXBC", "{label}: missing DXBC magic");
        let digest = &bytes[4..20];
        assert!(
            digest.iter().any(|&b| b != 0),
            "{label}: container digest must be signed (non-zero)"
        );
        let declared_size = u32::from_le_bytes(bytes[24..28].try_into().unwrap()) as usize;
        assert_eq!(declared_size, bytes.len(), "{label}: declared size mismatch");
        let part_count = u32::from_le_bytes(bytes[28..32].try_into().unwrap()) as usize;
        assert!(part_count >= 2, "{label}: expected >= 2 DXBC chunks");

        let mut saw_dxil = false;
        let mut saw_psv0 = false;
        for i in 0..part_count {
            let off = u32::from_le_bytes(bytes[32 + i * 4..36 + i * 4].try_into().unwrap()) as usize;
            let fcc = &bytes[off..off + 4];
            let len = u32::from_le_bytes(bytes[off + 4..off + 8].try_into().unwrap()) as usize;
            let body = &bytes[off + 8..off + 8 + len];
            if fcc == b"DXIL" {
                saw_dxil = true;
                assert!(body.len() >= 28, "{label}: DXIL part too small");
                assert_eq!(&body[8..12], b"DXIL", "{label}: missing DXIL program header fourcc");
                let bc_off = u32::from_le_bytes(body[16..20].try_into().unwrap()) as usize;
                assert_eq!(
                    &body[8 + bc_off..8 + bc_off + 4],
                    &[0x42, 0x43, 0xc0, 0xde],
                    "{label}: missing LLVM bitcode magic"
                );
            } else if fcc == b"PSV0" {
                saw_psv0 = true;
            }
        }
        assert!(saw_dxil, "{label}: missing DXIL chunk");
        assert!(saw_psv0, "{label}: missing PSV0 chunk");

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        {
            assert!(
                dxil_metal::dxbc::verify_checksum(bytes),
                "{label}: DXBC retail MD5 checksum verification failed"
            );
            let container = dxil_metal::dxbc::Container::parse(bytes)
                .unwrap_or_else(|e| panic!("{label}: Container::parse failed: {e}"));
            let prog = container.program().expect("missing DXIL program header");
            assert_eq!(
                prog.kind,
                dxil_metal::dxbc::ProgramKind::Compute,
                "{label}: expected compute shader"
            );
            assert_eq!(
                container.entry_point().as_deref(),
                Some("main"),
                "{label}: expected entry point 'main'"
            );
        }
    }

    unsafe fn validate_dispatches_against_instance(
        desc: &NrdInstanceDesc,
        dispatches: &[NrdDispatchDesc],
    ) {
        let pipelines = std::slice::from_raw_parts(desc.pipelines, desc.pipelines_num as usize);
        let mut total_cbvs = 0u32;
        let mut total_srvs = 0u32;
        let mut total_uavs = 0u32;

        for (di, d) in dispatches.iter().enumerate() {
            assert!(!d.name.is_null(), "dispatch {di}: null name");
            assert!(
                (d.pipeline_index as u32) < desc.pipelines_num,
                "dispatch {di}: pipeline_index {} out of bounds ({})",
                d.pipeline_index,
                desc.pipelines_num
            );
            assert!(d.grid_width > 0 && d.grid_height > 0, "dispatch {di}: zero grid");

            let pipe = &pipelines[d.pipeline_index as usize];
            assert!(!pipe.compute_shader_dxil.bytecode.is_null());
            assert!(pipe.compute_shader_dxil.size >= 32);

            if pipe.has_constant_data != 0 {
                assert!(!d.constant_buffer_data.is_null(), "dispatch {di}: missing CB data");
                assert!(d.constant_buffer_data_size > 0, "dispatch {di}: zero CB size");
                assert!(
                    d.constant_buffer_data_size <= desc.constant_buffer_max_data_size,
                    "dispatch {di}: CB size {} exceeds max {}",
                    d.constant_buffer_data_size,
                    desc.constant_buffer_max_data_size
                );
                total_cbvs += 1;
            } else {
                assert_eq!(d.constant_buffer_data_size, 0, "dispatch {di}: unexpected CB data");
            }

            let ranges =
                std::slice::from_raw_parts(pipe.resource_ranges, pipe.resource_ranges_num as usize);
            let expected_res_num: u32 = ranges.iter().map(|r| r.descriptors_num).sum();
            assert_eq!(d.resources_num, expected_res_num, "dispatch {di}: resource count mismatch");
            let resources = std::slice::from_raw_parts(d.resources, d.resources_num as usize);

            let mut cursor = 0usize;
            let mut srv_set: Vec<(u32, u16)> = Vec::new();
            let mut uav_set: Vec<(u32, u16)> = Vec::new();
            for range in ranges {
                for _ in 0..range.descriptors_num {
                    let r = &resources[cursor];
                    cursor += 1;
                    assert_eq!(
                        r.state_needed, range.descriptor_type,
                        "dispatch {di}: state_needed mismatch at slot {cursor}"
                    );
                    assert_eq!(r.mip_num, 1);
                    if r.resource_type == NRD_RES_PERMANENT_POOL {
                        assert!(
                            (r.index_in_pool as u32) < desc.permanent_pool_size,
                            "dispatch {di}: permanent pool index {} OOB ({})",
                            r.index_in_pool,
                            desc.permanent_pool_size
                        );
                    } else if r.resource_type == NRD_RES_TRANSIENT_POOL {
                        assert!(
                            (r.index_in_pool as u32) < desc.transient_pool_size,
                            "dispatch {di}: transient pool index {} OOB ({})",
                            r.index_in_pool,
                            desc.transient_pool_size
                        );
                    } else {
                        assert!((r.resource_type as usize) < NRD_RESOURCE_TYPE_MAX_NUM);
                    }
                    let key = (r.resource_type, r.index_in_pool);
                    if r.state_needed == NRD_DESCRIPTOR_TEXTURE {
                        total_srvs += 1;
                        srv_set.push(key);
                    } else {
                        total_uavs += 1;
                        assert!(
                            !uav_set.contains(&key),
                            "dispatch {di}: duplicate UAV binding {key:?}"
                        );
                        uav_set.push(key);
                    }
                }
            }
            // No resource may be bound simultaneously as SRV and UAV in the same dispatch.
            for key in &uav_set {
                assert!(
                    !srv_set.contains(key),
                    "dispatch {di}: SRV/UAV aliasing on resource {key:?}"
                );
            }
        }

        let pool = &desc.descriptor_pool_desc;
        assert!(
            (dispatches.len() as u32) <= pool.sets_max_num,
            "sets {} > sets_max_num {}",
            dispatches.len(),
            pool.sets_max_num
        );
        assert!(total_cbvs <= pool.constant_buffers_max_num);
        assert!(total_srvs <= pool.textures_max_num);
        assert!(total_uavs <= pool.storage_textures_max_num);
        assert!((dispatches.len() as u32) * desc.samplers_num <= pool.samplers_max_num);
    }

    #[test]
    fn nrd_library_desc_and_strings() {
        unsafe {
            let lib = &*GetLibraryDesc();
            assert_eq!(lib.version_major, 4);
            assert_eq!(lib.version_minor, 3);
            assert_eq!(lib.version_build, 6);
            assert_eq!(lib.normal_encoding, 2);
            assert_eq!(lib.roughness_encoding, 1);
            assert_eq!(lib.supported_denoisers_num, 21);
            assert_eq!(lib.spirv_binding_offsets.sampler_offset, 100);
            assert_eq!(lib.spirv_binding_offsets.texture_offset, 200);
            assert_eq!(lib.spirv_binding_offsets.constant_buffer_offset, 300);
            assert_eq!(lib.spirv_binding_offsets.storage_texture_and_buffer_offset, 400);

            let res0 = GetResourceTypeString(0);
            assert!(!res0.is_null());
            assert_eq!(CStr::from_ptr(res0 as *const std::ffi::c_char).to_str().unwrap(), "IN_MV");
            let res_last = GetResourceTypeString(36);
            assert!(!res_last.is_null());
            assert_eq!(
                CStr::from_ptr(res_last as *const std::ffi::c_char).to_str().unwrap(),
                "PERMANENT_POOL"
            );
            assert!(GetResourceTypeString(37).is_null());

            let den0 = GetDenoiserString(0);
            assert!(!den0.is_null());
            assert_eq!(
                CStr::from_ptr(den0 as *const std::ffi::c_char).to_str().unwrap(),
                "REBLUR_DIFFUSE"
            );
            let den_last = GetDenoiserString(20);
            assert!(!den_last.is_null());
            assert_eq!(
                CStr::from_ptr(den_last as *const std::ffi::c_char).to_str().unwrap(),
                "SPECULAR_DELTA_MV"
            );
            assert!(GetDenoiserString(21).is_null());
        }
    }

    #[test]
    fn nrd_embedded_shaders_valid_dxbc_dxil() {
        let all = [
            ("Clear_f", ShaderKind::ClearFloat, shaders::CLEAR_F_DXIL),
            ("Clear_ui", ShaderKind::ClearUint, shaders::CLEAR_UI_DXIL),
            ("REBLUR_Filter", ShaderKind::ReblurFilter, shaders::REBLUR_FILTER_DXIL),
            ("RELAX_Filter", ShaderKind::RelaxFilter, shaders::RELAX_FILTER_DXIL),
            ("SIGMA_Shadow", ShaderKind::SigmaShadow, shaders::SIGMA_SHADOW_DXIL),
            ("REFERENCE_Accumulate", ShaderKind::ReferenceAccumulate, shaders::REFERENCE_ACCUMULATE_DXIL),
            ("SpecularMv_Compute", ShaderKind::SpecularMvCompute, shaders::SPECULAR_MV_COMPUTE_DXIL),
        ];
        for (name, kind, blob) in all {
            verify_dxbc_dxil_blob(blob, name);
            let pdesc = kind.pipeline_desc();
            assert_eq!(pdesc.compute_shader_dxil.size, blob.len() as u64);
            assert!(!pdesc.shader_file_name.is_null());
            assert!(!pdesc.shader_entry_point_name.is_null());

            #[cfg(target_os = "macos")]
            if dxil_metal::converter_available() {
                use dxil_metal::root_signature::{
                    flags as rs_flags, DescriptorRange, DescriptorRangeType, RootDescriptor,
                    RootDescriptorKind, RootParameter, RootParameterKind, RootSignatureDesc,
                    RootSignatureVersion,
                };
                let ranges = unsafe {
                    std::slice::from_raw_parts(
                        pdesc.resource_ranges,
                        pdesc.resource_ranges_num as usize,
                    )
                };
                let mut params = vec![RootParameter {
                    kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                        range_type: DescriptorRangeType::Sampler,
                        num_descriptors: 4,
                        base_shader_register: 0,
                        register_space: 0,
                        flags: 0,
                        offset_in_descriptors_from_table_start:
                            rs_flags::DESCRIPTOR_RANGE_OFFSET_APPEND,
                    }]),
                    visibility: 0,
                }];
                for r in ranges {
                    let rt = if r.descriptor_type == NRD_DESCRIPTOR_TEXTURE {
                        DescriptorRangeType::Srv
                    } else {
                        DescriptorRangeType::Uav
                    };
                    params.push(RootParameter {
                        kind: RootParameterKind::DescriptorTable(vec![DescriptorRange {
                            range_type: rt,
                            num_descriptors: r.descriptors_num,
                            base_shader_register: r.base_register_index,
                            register_space: 0,
                            flags: 0,
                            offset_in_descriptors_from_table_start:
                                rs_flags::DESCRIPTOR_RANGE_OFFSET_APPEND,
                        }]),
                        visibility: 0,
                    });
                }
                if pdesc.has_constant_data != 0 {
                    params.push(RootParameter {
                        kind: RootParameterKind::Descriptor(
                            RootDescriptorKind::Cbv,
                            RootDescriptor { shader_register: 0, register_space: 0, flags: 0 },
                        ),
                        visibility: 0,
                    });
                }
                let rs_blob = RootSignatureDesc {
                    version: RootSignatureVersion::V1_1,
                    parameters: params,
                    static_samplers: Vec::new(),
                    flags: 0,
                }
                .to_blob();
                let rs = dxil_metal::RootSignature::from_blob(&rs_blob).expect("parse root sig");
                let opts = dxil_metal::CompileOptions::default();
                let req = dxil_metal::CompileRequest::new(blob, Some(&rs), &opts)
                    .stage(dxil_metal::ShaderStage::Compute);
                let compiled = dxil_metal::compile_uncached(&req)
                    .unwrap_or_else(|e| panic!("{name}: Metal IR conversion failed: {e}"));
                assert!(!compiled.metallib.is_empty(), "{name}: empty metallib");
            }
        }
    }

    #[test]
    fn nrd_instance_validation_settings_and_dispatches() {
        unsafe {
            let mut inst: *mut NrdInstance = ptr::null_mut();

            // Null desc / out pointer rejected.
            assert_eq!(CreateInstance(ptr::null(), &mut inst), NRD_RESULT_INVALID_ARGUMENT);
            assert_eq!(CreateInstance(ptr::null(), ptr::null_mut()), NRD_RESULT_INVALID_ARGUMENT);

            // Duplicate identifier rejected with NON_UNIQUE_IDENTIFIER.
            let dup = [
                NrdDenoiserDesc { identifier: 7, denoiser: 6, render_width: 1920, render_height: 1080 },
                NrdDenoiserDesc { identifier: 7, denoiser: 10, render_width: 1920, render_height: 1080 },
            ];
            let dup_desc = NrdInstanceCreationDesc {
                memory_allocator_interface: NrdMemoryAllocatorInterface::default(),
                denoisers: dup.as_ptr(),
                denoisers_num: 2,
                _pad: 0,
            };
            assert_eq!(
                CreateInstance(&dup_desc as *const _ as *const u8, &mut inst),
                NRD_RESULT_NON_UNIQUE_IDENTIFIER
            );

            // Unsupported denoiser enum rejected.
            let bad_enum = [NrdDenoiserDesc {
                identifier: 1,
                denoiser: 99,
                render_width: 1280,
                render_height: 720,
            }];
            let bad_desc = NrdInstanceCreationDesc {
                memory_allocator_interface: NrdMemoryAllocatorInterface::default(),
                denoisers: bad_enum.as_ptr(),
                denoisers_num: 1,
                _pad: 0,
            };
            assert_eq!(
                CreateInstance(&bad_desc as *const _ as *const u8, &mut inst),
                NRD_RESULT_UNSUPPORTED
            );

            // Valid instance with REBLUR_DIFFUSE_SPECULAR (6) and SIGMA_SHADOW (10).
            let denoisers = [
                NrdDenoiserDesc { identifier: 1, denoiser: 6, render_width: 1920, render_height: 1080 },
                NrdDenoiserDesc { identifier: 2, denoiser: 10, render_width: 1920, render_height: 1080 },
            ];
            let creation = NrdInstanceCreationDesc {
                memory_allocator_interface: NrdMemoryAllocatorInterface::default(),
                denoisers: denoisers.as_ptr(),
                denoisers_num: 2,
                _pad: 0,
            };
            assert_eq!(
                CreateInstance(&creation as *const _ as *const u8, &mut inst),
                NRD_RESULT_SUCCESS
            );
            assert!(!inst.is_null());

            let desc = &*GetInstanceDesc(inst);
            assert_eq!(desc.samplers_num, 4);
            assert!(!desc.samplers.is_null());
            // Reblur + Sigma + Clear_f + Clear_ui = 4 unique pipelines.
            assert_eq!(desc.pipelines_num, 4);
            assert!(!desc.pipelines.is_null());
            // Permanent: Reblur (meta x2 + internal uint + diff hist x2 + spec hist x2 = 7)
            //          + Sigma (hist x2 = 2) = 9 textures, all 1920x1080.
            assert_eq!(desc.permanent_pool_size, 9);
            let perm = std::slice::from_raw_parts(desc.permanent_pool, desc.permanent_pool_size as usize);
            assert!(perm.iter().all(|t| t.width == 1920 && t.height == 1080 && t.mip_num == 1));
            // Transient: Reblur (1x RGBA16F) reused by Sigma (blur RGBA16F + penumbra RG16F) = 2 textures.
            assert_eq!(desc.transient_pool_size, 2);
            let tran = std::slice::from_raw_parts(desc.transient_pool, desc.transient_pool_size as usize);
            assert!(tran.iter().all(|t| t.width == 1920 && t.height == 1080 && t.mip_num == 1));
            assert!(desc.constant_buffer_max_data_size >= 256);

            // CommonSettings validation + first-use CLEAR_AND_RESTART override.
            let mut bad_cs = NrdCommonSettings::default();
            bad_cs.denoising_range = 0.0;
            assert_eq!(
                SetCommonSettings(inst, &bad_cs as *const _ as *const u8),
                NRD_RESULT_INVALID_ARGUMENT
            );

            let mut cs = NrdCommonSettings::default();
            cs.frame_index = 12;
            cs.accumulation_mode = NRD_ACCUMULATION_CONTINUE;
            assert_eq!(
                SetCommonSettings(inst, &cs as *const _ as *const u8),
                NRD_RESULT_SUCCESS
            );
            let inst_ref = &*inst;
            assert_eq!(inst_ref.common_settings.frame_index, 12);
            assert_eq!(inst_ref.common_settings.accumulation_mode, NRD_ACCUMULATION_CLEAR_AND_RESTART);

            // Denoiser settings validation & storage.
            let mut reblur = NrdReblurSettings::default();
            reblur.max_accumulated_frame_num = 16;
            assert_eq!(
                SetDenoiserSettings(inst, 1, &reblur as *const _ as *const u8),
                NRD_RESULT_SUCCESS
            );
            assert!((&*inst).denoisers[0].settings_set);
            assert!(matches!(
                (&*inst).denoisers[0].settings,
                DenoiserSettingsState::Reblur(s) if s.max_accumulated_frame_num == 16
            ));

            let sigma = NrdSigmaSettings { plane_distance_sensitivity: 0.01, blur_radius_scale: 2.5 };
            assert_eq!(
                SetDenoiserSettings(inst, 2, &sigma as *const _ as *const u8),
                NRD_RESULT_SUCCESS
            );
            assert!(matches!(
                (&*inst).denoisers[1].settings,
                DenoiserSettingsState::Sigma(s) if (s.blur_radius_scale - 2.5).abs() < 1e-6
            ));
            assert_eq!(
                SetDenoiserSettings(inst, 999, &sigma as *const _ as *const u8),
                NRD_RESULT_INVALID_ARGUMENT
            );

            // Unknown id rejected.
            let mut dispatches_ptr: *const u8 = ptr::dangling();
            let mut dispatches_num: u32 = 99;
            let unknown_id: u32 = 999;
            assert_eq!(
                GetComputeDispatches(inst, &unknown_id, 1, &mut dispatches_ptr, &mut dispatches_num),
                NRD_RESULT_INVALID_ARGUMENT
            );
            assert!(dispatches_ptr.is_null());
            assert_eq!(dispatches_num, 0);

            // Frame 12 (first use -> CLEAR_AND_RESTART): 11 unique pool clears + 4 Reblur + 2 Sigma = 17 dispatches.
            let valid_ids = [1u32, 2u32];
            assert_eq!(
                GetComputeDispatches(inst, valid_ids.as_ptr(), 2, &mut dispatches_ptr, &mut dispatches_num),
                NRD_RESULT_SUCCESS
            );
            assert!(!dispatches_ptr.is_null());
            assert_eq!(dispatches_num, 17);
            let dispatches = std::slice::from_raw_parts(
                dispatches_ptr as *const NrdDispatchDesc,
                dispatches_num as usize,
            );
            validate_dispatches_against_instance(desc, dispatches);
            // Verify clear passes include both Clear_f and Clear_ui before denoiser passes.
            let clear_f_count = dispatches
                .iter()
                .filter(|d| d.pipeline_index == (&*inst).clear_float_pipeline)
                .count();
            let clear_ui_count = dispatches
                .iter()
                .filter(|d| d.pipeline_index == (&*inst).clear_uint_pipeline)
                .count();
            assert_eq!(clear_f_count, 10);
            assert_eq!(clear_ui_count, 1);
            // 1920/8 = 240, ceil(1080/8) = 135.
            assert!(dispatches.iter().all(|d| d.grid_width == 240 && d.grid_height == 135));
            let first_reblur_hist_read_f0 =
                std::slice::from_raw_parts(dispatches[11].resources, 8)[4].index_in_pool;

            // Frame 13 (CONTINUE): 0 clear passes + 4 Reblur + 2 Sigma = 6 dispatches, ping-pong swapped.
            cs.frame_index = 13;
            cs.accumulation_mode = NRD_ACCUMULATION_CONTINUE;
            assert_eq!(SetCommonSettings(inst, &cs as *const _ as *const u8), NRD_RESULT_SUCCESS);
            assert_eq!((&*inst).common_settings.accumulation_mode, NRD_ACCUMULATION_CONTINUE);
            assert_eq!(
                GetComputeDispatches(inst, valid_ids.as_ptr(), 2, &mut dispatches_ptr, &mut dispatches_num),
                NRD_RESULT_SUCCESS
            );
            assert_eq!(dispatches_num, 6);
            let dispatches2 = std::slice::from_raw_parts(
                dispatches_ptr as *const NrdDispatchDesc,
                dispatches_num as usize,
            );
            validate_dispatches_against_instance(desc, dispatches2);
            let first_reblur_hist_read_f1 =
                std::slice::from_raw_parts(dispatches2[0].resources, 8)[4].index_in_pool;
            assert_ne!(first_reblur_hist_read_f0, first_reblur_hist_read_f1);

            // Verify DRS (0.5x resolution scale) scales denoiser dispatch grids cleanly.
            cs.frame_index = 14;
            cs.resolution_scale = [0.5, 0.5];
            assert_eq!(SetCommonSettings(inst, &cs as *const _ as *const u8), NRD_RESULT_SUCCESS);
            assert_eq!(
                GetComputeDispatches(inst, valid_ids.as_ptr(), 2, &mut dispatches_ptr, &mut dispatches_num),
                NRD_RESULT_SUCCESS
            );
            let dispatches_drs = std::slice::from_raw_parts(
                dispatches_ptr as *const NrdDispatchDesc,
                dispatches_num as usize,
            );
            // 960/8 = 120, ceil(540/8) = 68.
            assert!(dispatches_drs.iter().all(|d| d.grid_width == 120 && d.grid_height == 68));

            DestroyInstance(inst);
        }
    }

    #[test]
    fn nrd_all_21_denoisers_pools_and_dispatches() {
        unsafe {
            // Test every single supported denoiser mode (0..=20) with non-multiple-of-8 dimensions.
            for denoiser in 0..NRD_DENOISER_MAX_NUM as u32 {
                let w: u16 = 317 + (denoiser as u16) * 7;
                let h: u16 = 179 + (denoiser as u16) * 5;
                let expected_gw = (w as u32).div_ceil(8) as u16;
                let expected_gh = (h as u32).div_ceil(8) as u16;

                let dd = [NrdDenoiserDesc {
                    identifier: 100 + denoiser,
                    denoiser,
                    render_width: w,
                    render_height: h,
                }];
                let creation = NrdInstanceCreationDesc {
                    memory_allocator_interface: NrdMemoryAllocatorInterface::default(),
                    denoisers: dd.as_ptr(),
                    denoisers_num: 1,
                    _pad: 0,
                };
                let mut inst: *mut NrdInstance = ptr::null_mut();
                assert_eq!(
                    CreateInstance(&creation as *const _ as *const u8, &mut inst),
                    NRD_RESULT_SUCCESS,
                    "CreateInstance failed for denoiser {denoiser}"
                );
                let desc = &*GetInstanceDesc(inst);
                // 1 denoiser shader + Clear_f + Clear_ui = 3 pipelines.
                assert_eq!(desc.pipelines_num, 3, "denoiser {denoiser}: unexpected pipelines_num");
                assert!(
                    desc.permanent_pool_size + desc.transient_pool_size >= 1,
                    "denoiser {denoiser}: expected >= 1 pool texture"
                );
                if desc.permanent_pool_size > 0 {
                    let perm = std::slice::from_raw_parts(
                        desc.permanent_pool,
                        desc.permanent_pool_size as usize,
                    );
                    assert!(perm.iter().all(|t| t.width == w && t.height == h && t.mip_num == 1));
                }
                if desc.transient_pool_size > 0 {
                    let tran = std::slice::from_raw_parts(
                        desc.transient_pool,
                        desc.transient_pool_size as usize,
                    );
                    assert!(tran.iter().all(|t| t.width == w && t.height == h && t.mip_num == 1));
                }

                // Frame 0 (first use -> CLEAR_AND_RESTART).
                let mut cs = NrdCommonSettings::default();
                cs.frame_index = 0;
                assert_eq!(
                    SetCommonSettings(inst, &cs as *const _ as *const u8),
                    NRD_RESULT_SUCCESS
                );
                let id = 100 + denoiser;
                let mut dptr: *const u8 = ptr::null();
                let mut dnum: u32 = 0;
                assert_eq!(
                    GetComputeDispatches(inst, &id, 1, &mut dptr, &mut dnum),
                    NRD_RESULT_SUCCESS
                );
                let clear_count = desc.permanent_pool_size + desc.transient_pool_size;
                assert!(
                    dnum > clear_count,
                    "denoiser {denoiser}: expected clears ({clear_count}) + denoiser passes, got {dnum}"
                );
                let d0 = std::slice::from_raw_parts(dptr as *const NrdDispatchDesc, dnum as usize);
                validate_dispatches_against_instance(desc, d0);
                assert!(
                    d0.iter().all(|d| d.grid_width == expected_gw && d.grid_height == expected_gh),
                    "denoiser {denoiser}: grid mismatch"
                );

                // Frame 1 (CONTINUE -> no clears).
                cs.frame_index = 1;
                cs.accumulation_mode = NRD_ACCUMULATION_CONTINUE;
                assert_eq!(
                    SetCommonSettings(inst, &cs as *const _ as *const u8),
                    NRD_RESULT_SUCCESS
                );
                assert_eq!(
                    GetComputeDispatches(inst, &id, 1, &mut dptr, &mut dnum),
                    NRD_RESULT_SUCCESS
                );
                let d1 = std::slice::from_raw_parts(dptr as *const NrdDispatchDesc, dnum as usize);
                validate_dispatches_against_instance(desc, d1);
                assert_eq!(
                    d0.len() - d1.len(),
                    clear_count as usize,
                    "denoiser {denoiser}: clear count delta mismatch"
                );

                DestroyInstance(inst);
            }
        }
    }

    #[test]
    fn nrd_relax_atrous_iterations_and_reference_progression() {
        unsafe {
            let denoisers = [
                NrdDenoiserDesc {
                    identifier: 10,
                    denoiser: NRD_DENOISER_RELAX_DIFFUSE,
                    render_width: 640,
                    render_height: 360,
                },
                NrdDenoiserDesc {
                    identifier: 20,
                    denoiser: NRD_DENOISER_REFERENCE,
                    render_width: 640,
                    render_height: 360,
                },
            ];
            let creation = NrdInstanceCreationDesc {
                memory_allocator_interface: NrdMemoryAllocatorInterface::default(),
                denoisers: denoisers.as_ptr(),
                denoisers_num: 2,
                _pad: 0,
            };
            let mut inst: *mut NrdInstance = ptr::null_mut();
            assert_eq!(
                CreateInstance(&creation as *const _ as *const u8, &mut inst),
                NRD_RESULT_SUCCESS
            );
            let desc = &*GetInstanceDesc(inst);

            // Configure RELAX_DIFFUSE with 3 À-Trous iterations (1 temporal + 3 spatial = 4 passes).
            let mut relax = NrdRelaxDiffuseSettings::default();
            relax.atrous_iteration_num = 3;
            assert_eq!(
                SetDenoiserSettings(inst, 10, &relax as *const _ as *const u8),
                NRD_RESULT_SUCCESS
            );

            let mut cs = NrdCommonSettings::default();
            // Consume first-use CLEAR_AND_RESTART then switch to CONTINUE.
            assert_eq!(SetCommonSettings(inst, &cs as *const _ as *const u8), NRD_RESULT_SUCCESS);
            cs.frame_index = 1;
            cs.accumulation_mode = NRD_ACCUMULATION_CONTINUE;
            assert_eq!(SetCommonSettings(inst, &cs as *const _ as *const u8), NRD_RESULT_SUCCESS);

            let relax_id = 10u32;
            let mut dptr: *const u8 = ptr::null();
            let mut dnum: u32 = 0;
            assert_eq!(
                GetComputeDispatches(inst, &relax_id, 1, &mut dptr, &mut dnum),
                NRD_RESULT_SUCCESS
            );
            assert_eq!(dnum, 4);
            let ds = std::slice::from_raw_parts(dptr as *const NrdDispatchDesc, dnum as usize);
            validate_dispatches_against_instance(desc, ds);
            // Verify À-Trous step sizes 1, 2, 4 in the constant buffers.
            let cb1 = ptr::read_unaligned(ds[1].constant_buffer_data as *const RelaxCb);
            let cb2 = ptr::read_unaligned(ds[2].constant_buffer_data as *const RelaxCb);
            let cb3 = ptr::read_unaligned(ds[3].constant_buffer_data as *const RelaxCb);
            assert_eq!((cb1.pass_mode, cb1.step_size), (1, 1));
            assert_eq!((cb2.pass_mode, cb2.step_size), (1, 2));
            assert_eq!((cb3.pass_mode, cb3.step_size), (2, 4));

            // REFERENCE progressive accumulation speed across static frames + reset on RESTART.
            let ref_id = 20u32;
            cs.accumulation_mode = NRD_ACCUMULATION_RESTART;
            assert_eq!(SetCommonSettings(inst, &cs as *const _ as *const u8), NRD_RESULT_SUCCESS);
            assert_eq!(
                GetComputeDispatches(inst, &ref_id, 1, &mut dptr, &mut dnum),
                NRD_RESULT_SUCCESS
            );
            assert_eq!(dnum, 1);
            let r0 = &*std::slice::from_raw_parts(dptr as *const NrdDispatchDesc, 1).first().unwrap();
            let rcb0 = ptr::read_unaligned(r0.constant_buffer_data as *const ReferenceCb);
            assert_eq!(rcb0.reset_history, 1);
            assert!((rcb0.accum_speed - 1.0).abs() < 1e-6);

            cs.accumulation_mode = NRD_ACCUMULATION_CONTINUE;
            cs.frame_index = 2;
            assert_eq!(SetCommonSettings(inst, &cs as *const _ as *const u8), NRD_RESULT_SUCCESS);
            assert_eq!(
                GetComputeDispatches(inst, &ref_id, 1, &mut dptr, &mut dnum),
                NRD_RESULT_SUCCESS
            );
            let r1 = &*std::slice::from_raw_parts(dptr as *const NrdDispatchDesc, 1).first().unwrap();
            let rcb1 = ptr::read_unaligned(r1.constant_buffer_data as *const ReferenceCb);
            assert_eq!(rcb1.reset_history, 0);
            assert!((rcb1.accum_speed - 0.5).abs() < 1e-6);

            DestroyInstance(inst);
        }
    }
}
