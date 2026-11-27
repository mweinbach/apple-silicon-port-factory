//! Native `libxess.dll` / `libxess_dx12.dll` (Intel XeSS 1.3) bridge over MetalFX.
//!
//! Implements the Intel XeSS D3D12 C API (`xess.h` / `xess_d3d12.h`) on Apple Silicon:
//! * Stores full context state from `xessD3D12Init`, `xessSetJitterScale`,
//!   `xessSetVelocityScale`, and `xessSetExposureMultiplier`.
//! * Computes Intel XeSS 1.3 quality-mode render resolutions in
//!   `xessGetOptimalInputResolution` and `xessGetInputResolution`.
//! * Dispatches `xessD3D12Execute` through `crate::d3d12::cmdlist::upscale_temporal`
//!   (MetalFX Temporal), falling back to `upscale_resources` only when depth or
//!   velocity textures are null (or temporal MetalFX is disabled).

#![allow(non_snake_case)]

use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

// Float-taking Win64 DLL exports go through EXTRA_EXPORTS + win64_bridge!
// so ARM64 unpacks Win64 XMM1/XMM2 + GPR slots identically to x86-64.
pub static EXTRA_EXPORTS: &[crate::Export] = &[
    crate::Export {
        name: "xessSetJitterScale",
        ptr: crate::Ptr(crate::bridge_entry!(xess_set_jitter_scale_impl)),
    },
    crate::Export {
        name: "xessSetVelocityScale",
        ptr: crate::Ptr(crate::bridge_entry!(xess_set_velocity_scale_impl)),
    },
    crate::Export {
        name: "xessSetExposureMultiplier",
        ptr: crate::Ptr(crate::bridge_entry!(xess_set_exposure_multiplier_impl)),
    },
];
pub static ORDINALS: &[(u16, &str)] = &[];

// `xess_result_t` return codes (`xess.h`).
pub const XESS_RESULT_WARNING_NONEXISTING_FOLDER: i32 = 1;
pub const XESS_RESULT_WARNING_OLD_DRIVER: i32 = 2;
pub const XESS_RESULT_SUCCESS: i32 = 0;
pub const XESS_RESULT_ERROR_UNSUPPORTED_DEVICE: i32 = -1;
pub const XESS_RESULT_ERROR_UNSUPPORTED_DRIVER: i32 = -2;
pub const XESS_RESULT_ERROR_UNINITIALIZED: i32 = -3;
pub const XESS_RESULT_ERROR_INVALID_ARGUMENT: i32 = -4;
pub const XESS_RESULT_ERROR_DEVICE_OUT_OF_MEMORY: i32 = -5;
pub const XESS_RESULT_ERROR_DEVICE: i32 = -6;
pub const XESS_RESULT_ERROR_NOT_IMPLEMENTED: i32 = -7;
pub const XESS_RESULT_ERROR_INVALID_CONTEXT: i32 = -8;
pub const XESS_RESULT_ERROR_OPERATION_IN_PROGRESS: i32 = -9;
pub const XESS_RESULT_ERROR_UNSUPPORTED: i32 = -10;
pub const XESS_RESULT_ERROR_CANT_LOAD_LIBRARY: i32 = -11;
pub const XESS_RESULT_ERROR_WRONG_CALL_ORDER: i32 = -12;
pub const XESS_RESULT_ERROR_UNKNOWN: i32 = -1000;

// `xess_quality_settings_t` values (`xess.h`, Intel XeSS 1.3).
pub const XESS_QUALITY_SETTING_ULTRA_PERFORMANCE: u32 = 100;
pub const XESS_QUALITY_SETTING_PERFORMANCE: u32 = 101;
pub const XESS_QUALITY_SETTING_BALANCED: u32 = 102;
pub const XESS_QUALITY_SETTING_QUALITY: u32 = 103;
pub const XESS_QUALITY_SETTING_ULTRA_QUALITY: u32 = 104;
pub const XESS_QUALITY_SETTING_ULTRA_QUALITY_PLUS: u32 = 105;
pub const XESS_QUALITY_SETTING_AA: u32 = 106;

// `xess_init_flags_t` bitmask (`xess.h`).
pub const XESS_INIT_FLAG_NONE: u32 = 0;
pub const XESS_INIT_FLAG_HIGH_RES_MV: u32 = 1 << 0;
pub const XESS_INIT_FLAG_INVERTED_DEPTH: u32 = 1 << 1;
pub const XESS_INIT_FLAG_EXPOSURE_SCALE_TEXTURE: u32 = 1 << 2;
pub const XESS_INIT_FLAG_RESPONSIVE_PIXEL_MASK: u32 = 1 << 3;
pub const XESS_INIT_FLAG_USE_NDC_VELOCITY: u32 = 1 << 4;
pub const XESS_INIT_FLAG_EXTERNAL_DESCRIPTOR_HEAP: u32 = 1 << 5;
pub const XESS_INIT_FLAG_LDR_INPUT_COLOR: u32 = 1 << 6;
pub const XESS_INIT_FLAG_JITTERED_MV: u32 = 1 << 7;
pub const XESS_INIT_FLAG_ENABLE_AUTOEXPOSURE: u32 = 1 << 8;

/// `xess_2d_t` / `xess_coord_t` (8 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Xess2d {
    pub x: u32,
    pub y: u32,
}
const _: () = assert!(std::mem::size_of::<Xess2d>() == 8);

/// `xess_d3d12_init_params_t` (`xess_d3d12.h`, `#pragma pack(8)`, 64 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct XessD3D12InitParams {
    pub outputResolution: Xess2d,
    pub qualitySetting: u32,
    pub initFlags: u32,
    pub creationNodeMask: u32,
    pub visibleNodeMask: u32,
    pub pTempBufferHeap: usize,
    pub bufferHeapOffset: u64,
    pub pTempTextureHeap: usize,
    pub textureHeapOffset: u64,
    pub pPipelineLibrary: usize,
}
const _: () = assert!(std::mem::size_of::<XessD3D12InitParams>() == 0x40);
const _: () = assert!(std::mem::offset_of!(XessD3D12InitParams, qualitySetting) == 0x08);
const _: () = assert!(std::mem::offset_of!(XessD3D12InitParams, initFlags) == 0x0C);
const _: () = assert!(std::mem::offset_of!(XessD3D12InitParams, pTempBufferHeap) == 0x18);
const _: () = assert!(std::mem::offset_of!(XessD3D12InitParams, pPipelineLibrary) == 0x38);

/// `xess_d3d12_execute_params_t` (`xess_d3d12.h`, `#pragma pack(8)`, 136 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct XessD3D12ExecuteParams {
    pub pColorTexture: usize,
    pub pVelocityTexture: usize,
    pub pDepthTexture: usize,
    pub pExposureScaleTexture: usize,
    pub pResponsivePixelMaskTexture: usize,
    pub pOutputTexture: usize,
    pub jitterOffsetX: f32,
    pub jitterOffsetY: f32,
    pub exposureScale: f32,
    pub resetHistory: u32,
    pub inputWidth: u32,
    pub inputHeight: u32,
    pub inputColorBase: Xess2d,
    pub inputMotionVectorBase: Xess2d,
    pub inputDepthBase: Xess2d,
    pub inputResponsiveMaskBase: Xess2d,
    pub reserved0: Xess2d,
    pub outputColorBase: Xess2d,
    pub pDescriptorHeap: usize,
    pub descriptorHeapOffset: u32,
}
const _: () = assert!(std::mem::size_of::<XessD3D12ExecuteParams>() == 0x88);
const _: () = assert!(std::mem::offset_of!(XessD3D12ExecuteParams, pColorTexture) == 0x00);
const _: () = assert!(std::mem::offset_of!(XessD3D12ExecuteParams, pVelocityTexture) == 0x08);
const _: () = assert!(std::mem::offset_of!(XessD3D12ExecuteParams, pDepthTexture) == 0x10);
const _: () = assert!(std::mem::offset_of!(XessD3D12ExecuteParams, pExposureScaleTexture) == 0x18);
const _: () =
    assert!(std::mem::offset_of!(XessD3D12ExecuteParams, pResponsivePixelMaskTexture) == 0x20);
const _: () = assert!(std::mem::offset_of!(XessD3D12ExecuteParams, pOutputTexture) == 0x28);
const _: () = assert!(std::mem::offset_of!(XessD3D12ExecuteParams, jitterOffsetX) == 0x30);
const _: () = assert!(std::mem::offset_of!(XessD3D12ExecuteParams, jitterOffsetY) == 0x34);
const _: () = assert!(std::mem::offset_of!(XessD3D12ExecuteParams, exposureScale) == 0x38);
const _: () = assert!(std::mem::offset_of!(XessD3D12ExecuteParams, resetHistory) == 0x3C);
const _: () = assert!(std::mem::offset_of!(XessD3D12ExecuteParams, inputWidth) == 0x40);
const _: () = assert!(std::mem::offset_of!(XessD3D12ExecuteParams, inputHeight) == 0x44);
const _: () = assert!(std::mem::offset_of!(XessD3D12ExecuteParams, inputColorBase) == 0x48);
const _: () = assert!(std::mem::offset_of!(XessD3D12ExecuteParams, outputColorBase) == 0x70);
const _: () = assert!(std::mem::offset_of!(XessD3D12ExecuteParams, pDescriptorHeap) == 0x78);
const _: () = assert!(std::mem::offset_of!(XessD3D12ExecuteParams, descriptorHeapOffset) == 0x80);

static NEXT_XESS_HANDLE: AtomicUsize = AtomicUsize::new(0x5845_5301);

#[derive(Clone, Copy, Debug)]
struct XessContextState {
    device: usize,
    initialized: bool,
    output_resolution: [u32; 2],
    quality_setting: u32,
    init_flags: u32,
    creation_node_mask: u32,
    visible_node_mask: u32,
    jitter_scale: [f32; 2],
    velocity_scale: [f32; 2],
    exposure_multiplier: f32,
    first_execute: bool,
    logged_first_execute: bool,
}

impl XessContextState {
    fn new(device: usize) -> Self {
        Self {
            device,
            initialized: false,
            output_resolution: [0, 0],
            quality_setting: XESS_QUALITY_SETTING_QUALITY,
            init_flags: XESS_INIT_FLAG_NONE,
            creation_node_mask: 0,
            visible_node_mask: 0,
            jitter_scale: [1.0, 1.0],
            velocity_scale: [1.0, 1.0],
            exposure_multiplier: 1.0,
            first_execute: true,
            logged_first_execute: false,
        }
    }
}

fn xess_contexts() -> &'static Mutex<HashMap<usize, XessContextState>> {
    static MAP: OnceLock<Mutex<HashMap<usize, XessContextState>>> = OnceLock::new();
    MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Intel XeSS 1.3 per-dimension scaling ratios.
pub(crate) fn quality_scale_factor(quality: u32) -> Option<f32> {
    match quality {
        XESS_QUALITY_SETTING_ULTRA_PERFORMANCE => Some(3.0),
        XESS_QUALITY_SETTING_PERFORMANCE => Some(2.3),
        XESS_QUALITY_SETTING_BALANCED => Some(2.0),
        XESS_QUALITY_SETTING_QUALITY => Some(1.7),
        XESS_QUALITY_SETTING_ULTRA_QUALITY => Some(1.5),
        XESS_QUALITY_SETTING_ULTRA_QUALITY_PLUS => Some(1.3),
        XESS_QUALITY_SETTING_AA => Some(1.0),
        _ => None,
    }
}

/// XeSS per-dimension scaling ratios with optional legacy (XeSS 1.2) scale factors.
pub(crate) fn quality_scale_factor_legacy(quality: u32, legacy: bool) -> Option<f32> {
    if !legacy {
        return quality_scale_factor(quality);
    }
    match quality {
        XESS_QUALITY_SETTING_ULTRA_PERFORMANCE => Some(3.0),
        XESS_QUALITY_SETTING_PERFORMANCE => Some(2.0),
        XESS_QUALITY_SETTING_BALANCED => Some(1.7),
        XESS_QUALITY_SETTING_QUALITY => Some(1.5),
        XESS_QUALITY_SETTING_ULTRA_QUALITY => Some(1.3),
        XESS_QUALITY_SETTING_ULTRA_QUALITY_PLUS => Some(1.3),
        XESS_QUALITY_SETTING_AA => Some(1.0),
        _ => None,
    }
}

pub(crate) fn compute_input_resolution(out_w: u32, out_h: u32, scale: f32) -> Xess2d {
    let w = ((out_w as f32 / scale).round() as u32).clamp(1, out_w);
    let h = ((out_h as f32 / scale).round() as u32).clamp(1, out_h);
    Xess2d { x: w, y: h }
}

/// Compute `(optimal, min, max)` input resolutions for `out` and `quality`.
pub(crate) fn compute_optimal_input_resolutions(
    out: Xess2d,
    quality: u32,
    legacy: bool,
) -> Option<(Xess2d, Xess2d, Xess2d)> {
    if out.x == 0 || out.y == 0 {
        return None;
    }
    let scale = quality_scale_factor_legacy(quality, legacy)?;
    let optimal = compute_input_resolution(out.x, out.y, scale);
    let min_res = if quality == XESS_QUALITY_SETTING_AA {
        out
    } else {
        let floor = compute_input_resolution(out.x, out.y, 3.0);
        Xess2d {
            x: floor.x.min(optimal.x),
            y: floor.y.min(optimal.y),
        }
    };
    Some((optimal, min_res, out))
}

/// Write a 4x`u16` `xess_version_t` safely to potentially unaligned guest memory.
pub(crate) fn write_xess_version(
    version_out: *mut u16,
    major: u16,
    minor: u16,
    patch: u16,
    reserved: u16,
) -> i32 {
    if version_out.is_null() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    unsafe {
        std::ptr::write_unaligned(version_out.add(0), major);
        std::ptr::write_unaligned(version_out.add(1), minor);
        std::ptr::write_unaligned(version_out.add(2), patch);
        std::ptr::write_unaligned(version_out.add(3), reserved);
    }
    XESS_RESULT_SUCCESS
}

/// Normalized temporal upscaling parameters shared between XeSS D3D12 and D3D11.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ResolvedTemporalInputs {
    pub render_size: [u32; 2],
    pub upscale_size: [u32; 2],
    pub jitter_offset: [f32; 2],
    pub motion_vector_scale: [f32; 2],
    pub reset: bool,
    pub depth_reversed: bool,
    pub pre_exposure: f32,
    pub auto_exposure: bool,
    pub use_exposure_texture: bool,
    pub dynamic_resolution: bool,
    pub output_resolution_motion_vectors: bool,
    pub jittered_motion_vectors: bool,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn resolve_temporal_inputs(
    init_flags: u32,
    upscale_size: [u32; 2],
    render_size: [u32; 2],
    jitter_offset: [f32; 2],
    jitter_scale: [f32; 2],
    velocity_scale: [f32; 2],
    exec_exposure_scale: f32,
    exposure_multiplier: f32,
    has_exposure_tex: bool,
    reset_history: bool,
    first_execute: bool,
) -> ResolvedTemporalInputs {
    let high_res_mv = (init_flags & XESS_INIT_FLAG_HIGH_RES_MV) != 0;
    let depth_reversed = (init_flags & XESS_INIT_FLAG_INVERTED_DEPTH) != 0;
    let use_exp_tex_flag = (init_flags & XESS_INIT_FLAG_EXPOSURE_SCALE_TEXTURE) != 0;
    let use_ndc_mv = (init_flags & XESS_INIT_FLAG_USE_NDC_VELOCITY) != 0;
    let is_ldr = (init_flags & XESS_INIT_FLAG_LDR_INPUT_COLOR) != 0;
    let jittered_mv = (init_flags & XESS_INIT_FLAG_JITTERED_MV) != 0;
    let enable_auto_exp = (init_flags & XESS_INIT_FLAG_ENABLE_AUTOEXPOSURE) != 0;

    let jx = if jitter_offset[0].is_finite() {
        jitter_offset[0] * jitter_scale[0]
    } else {
        0.0
    };
    let jy = if jitter_offset[1].is_finite() {
        jitter_offset[1] * jitter_scale[1]
    } else {
        0.0
    };

    let mut motion_vector_scale = velocity_scale;
    if use_ndc_mv {
        let mv_dims = if high_res_mv && upscale_size != [0, 0] {
            upscale_size
        } else if render_size != [0, 0] {
            render_size
        } else {
            upscale_size
        };
        if mv_dims[0] > 0 && mv_dims[1] > 0 {
            motion_vector_scale = [
                motion_vector_scale[0] * 0.5 * mv_dims[0] as f32,
                motion_vector_scale[1] * -0.5 * mv_dims[1] as f32,
            ];
        }
    }

    let exec_exp = if exec_exposure_scale.is_finite() && exec_exposure_scale > 0.0 {
        exec_exposure_scale
    } else {
        1.0
    };
    let combined_exp = exec_exp * exposure_multiplier;
    let pre_exposure = if combined_exp.is_finite() && combined_exp > 0.0 {
        combined_exp
    } else {
        1.0
    };

    let use_exposure_texture = !enable_auto_exp && (use_exp_tex_flag || has_exposure_tex);
    let effective_exp_tex = use_exposure_texture && has_exposure_tex;
    let is_hdr = !is_ldr;
    let auto_exposure =
        enable_auto_exp || (is_hdr && !effective_exp_tex && (pre_exposure - 1.0).abs() < 1e-4);

    ResolvedTemporalInputs {
        render_size,
        upscale_size,
        jitter_offset: [jx, jy],
        motion_vector_scale,
        reset: reset_history || first_execute,
        depth_reversed,
        pre_exposure,
        auto_exposure,
        use_exposure_texture: effective_exp_tex,
        dynamic_resolution: render_size != [0, 0],
        output_resolution_motion_vectors: high_res_mv,
        jittered_motion_vectors: jittered_mv,
    }
}

fn build_temporal_desc(
    ctx: &XessContextState,
    p: &XessD3D12ExecuteParams,
    first_execute: bool,
) -> crate::d3d12::cmdlist::TemporalUpscaleDesc {
    let r = resolve_temporal_inputs(
        ctx.init_flags,
        ctx.output_resolution,
        [p.inputWidth, p.inputHeight],
        [p.jitterOffsetX, p.jitterOffsetY],
        ctx.jitter_scale,
        ctx.velocity_scale,
        p.exposureScale,
        ctx.exposure_multiplier,
        p.pExposureScaleTexture != 0,
        p.resetHistory != 0,
        first_execute,
    );

    let exposure_ptr = if r.use_exposure_texture {
        p.pExposureScaleTexture as *mut c_void
    } else {
        std::ptr::null_mut()
    };

    let reactive_ptr = p.pResponsivePixelMaskTexture as *mut c_void;

    crate::d3d12::cmdlist::TemporalUpscaleDesc {
        output: p.pOutputTexture as *mut c_void,
        color: p.pColorTexture as *mut c_void,
        depth: p.pDepthTexture as *mut c_void,
        motion: p.pVelocityTexture as *mut c_void,
        exposure: exposure_ptr,
        reactive_mask: reactive_ptr,
        render_size: r.render_size,
        upscale_size: r.upscale_size,
        jitter_offset: r.jitter_offset,
        motion_vector_scale: r.motion_vector_scale,
        reset: r.reset,
        depth_reversed: r.depth_reversed,
        pre_exposure: r.pre_exposure,
        auto_exposure: r.auto_exposure,
        dynamic_resolution: r.dynamic_resolution,
        output_resolution_motion_vectors: r.output_resolution_motion_vectors,
        jittered_motion_vectors: r.jittered_motion_vectors,
        color_origin: [p.inputColorBase.x, p.inputColorBase.y],
        depth_origin: [p.inputDepthBase.x, p.inputDepthBase.y],
        motion_origin: [p.inputMotionVectorBase.x, p.inputMotionVectorBase.y],
        reactive_origin: [p.inputResponsiveMaskBase.x, p.inputResponsiveMaskBase.y],
        output_origin: [p.outputColorBase.x, p.outputColorBase.y],
    }
}

pub extern "win64" fn xessGetVersion(version_out: *mut u16) -> i32 {
    write_xess_version(version_out, 1, 3, 1, 0)
}

pub extern "win64" fn xessD3D12CreateContext(device: usize, out_context: *mut usize) -> i32 {
    if device == 0 || out_context.is_null() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let ctx = NEXT_XESS_HANDLE.fetch_add(1, Ordering::Relaxed);
    xess_contexts()
        .lock()
        .unwrap()
        .insert(ctx, XessContextState::new(device));
    unsafe { std::ptr::write_unaligned(out_context, ctx) };
    XESS_RESULT_SUCCESS
}

pub extern "win64" fn xessD3D12Init(
    context: usize,
    init_params: *const XessD3D12InitParams,
) -> i32 {
    if init_params.is_null() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let params = unsafe { std::ptr::read_unaligned(init_params) };
    if params.outputResolution.x == 0
        || params.outputResolution.y == 0
        || quality_scale_factor(params.qualitySetting).is_none()
    {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let mut map = xess_contexts().lock().unwrap();
    let Some(ctx) = map.get_mut(&context) else {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    };
    ctx.initialized = true;
    ctx.output_resolution = [params.outputResolution.x, params.outputResolution.y];
    ctx.quality_setting = params.qualitySetting;
    ctx.init_flags = params.initFlags;
    ctx.creation_node_mask = params.creationNodeMask;
    ctx.visible_node_mask = params.visibleNodeMask;
    ctx.first_execute = true;
    ctx.logged_first_execute = false;
    log::info!(
        target: "d3d",
        "xessD3D12Init(dev={:#x}): output={}x{} quality={} flags={:#x} temporal={}",
        ctx.device,
        ctx.output_resolution[0],
        ctx.output_resolution[1],
        ctx.quality_setting,
        ctx.init_flags,
        crate::d3d12::cmdlist::temporal_upscale_enabled(),
    );
    XESS_RESULT_SUCCESS
}

pub extern "win64" fn xessD3D12GetInitParams(
    context: usize,
    init_params_out: *mut XessD3D12InitParams,
) -> i32 {
    if init_params_out.is_null() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let map = xess_contexts().lock().unwrap();
    let Some(ctx) = map.get(&context) else {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    };
    if !ctx.initialized {
        return XESS_RESULT_ERROR_UNINITIALIZED;
    }
    let out = XessD3D12InitParams {
        outputResolution: Xess2d {
            x: ctx.output_resolution[0],
            y: ctx.output_resolution[1],
        },
        qualitySetting: ctx.quality_setting,
        initFlags: ctx.init_flags,
        creationNodeMask: ctx.creation_node_mask,
        visibleNodeMask: ctx.visible_node_mask,
        ..Default::default()
    };
    unsafe { std::ptr::write_unaligned(init_params_out, out) };
    XESS_RESULT_SUCCESS
}

pub extern "win64" fn xessGetOptimalInputResolution(
    context: usize,
    out_res: *const Xess2d,
    quality: u32,
    input_res_optimal: *mut Xess2d,
    input_res_min: *mut Xess2d,
    input_res_max: *mut Xess2d,
) -> i32 {
    if !xess_contexts().lock().unwrap().contains_key(&context) {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    }
    if out_res.is_null() || input_res_optimal.is_null() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let out = unsafe { std::ptr::read_unaligned(out_res) };
    let Some((optimal, min_res, max_res)) = compute_optimal_input_resolutions(out, quality, false)
    else {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    };
    unsafe {
        std::ptr::write_unaligned(input_res_optimal, optimal);
        if !input_res_min.is_null() {
            std::ptr::write_unaligned(input_res_min, min_res);
        }
        if !input_res_max.is_null() {
            std::ptr::write_unaligned(input_res_max, max_res);
        }
    }
    XESS_RESULT_SUCCESS
}

pub extern "win64" fn xessGetInputResolution(
    context: usize,
    out_res: *const Xess2d,
    quality: u32,
    input_res: *mut Xess2d,
) -> i32 {
    xessGetOptimalInputResolution(
        context,
        out_res,
        quality,
        input_res,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
    )
}

pub extern "win64" fn xessD3D12Execute(
    context: usize,
    cmd_list: usize,
    exec_params: *const XessD3D12ExecuteParams,
) -> i32 {
    if cmd_list == 0 || exec_params.is_null() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let p = unsafe { std::ptr::read_unaligned(exec_params) };
    if p.pColorTexture == 0 || p.pOutputTexture == 0 {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }

    let (ctx_snapshot, first_execute, should_log_first) = {
        let mut map = xess_contexts().lock().unwrap();
        let Some(ctx) = map.get_mut(&context) else {
            return XESS_RESULT_ERROR_INVALID_CONTEXT;
        };
        if !ctx.initialized {
            return XESS_RESULT_ERROR_UNINITIALIZED;
        }
        let first = ctx.first_execute;
        ctx.first_execute = false;
        let log_first = !ctx.logged_first_execute;
        ctx.logged_first_execute = true;
        (*ctx, first, log_first)
    };

    let desc = build_temporal_desc(&ctx_snapshot, &p, first_execute);
    let use_temporal = crate::d3d12::cmdlist::temporal_upscale_enabled()
        && p.pDepthTexture != 0
        && p.pVelocityTexture != 0;

    if should_log_first {
        log::info!(
            target: "d3d",
            "xessD3D12Execute: temporal={use_temporal} render={}x{} upscale={}x{} jitter=({:.3},{:.3}) mv_scale=({:.2},{:.2}) pre_exp={:.3} auto_exp={} depth_rev={} out_res_mv={} jit_mv={} depth={} mv={} exp={} reactive={} color_base=({},{}) out_base=({},{})",
            desc.render_size[0],
            desc.render_size[1],
            desc.upscale_size[0],
            desc.upscale_size[1],
            desc.jitter_offset[0],
            desc.jitter_offset[1],
            desc.motion_vector_scale[0],
            desc.motion_vector_scale[1],
            desc.pre_exposure,
            desc.auto_exposure,
            desc.depth_reversed,
            desc.output_resolution_motion_vectors,
            desc.jittered_motion_vectors,
            p.pDepthTexture != 0,
            p.pVelocityTexture != 0,
            !desc.exposure.is_null(),
            !desc.reactive_mask.is_null(),
            desc.color_origin[0],
            desc.color_origin[1],
            desc.output_origin[0],
            desc.output_origin[1],
        );
    }

    if use_temporal {
        unsafe {
            crate::d3d12::cmdlist::upscale_temporal(cmd_list as *mut c_void, desc);
        }
    } else {
        unsafe {
            crate::d3d12::cmdlist::upscale_resources_subrect(
                cmd_list as *mut c_void,
                p.pOutputTexture as *mut c_void,
                p.pColorTexture as *mut c_void,
                desc.render_size,
                desc.upscale_size,
                desc.color_origin,
                desc.output_origin,
            );
        }
    }
    XESS_RESULT_SUCCESS
}

pub extern "win64" fn xessDestroyContext(context: usize) -> i32 {
    if xess_contexts().lock().unwrap().remove(&context).is_none() {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    }
    XESS_RESULT_SUCCESS
}

extern "win64" fn xess_set_jitter_scale_impl(context: usize, x: f32, y: f32) -> i32 {
    if !x.is_finite() || !y.is_finite() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let mut map = xess_contexts().lock().unwrap();
    let Some(ctx) = map.get_mut(&context) else {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    };
    ctx.jitter_scale = [x, y];
    XESS_RESULT_SUCCESS
}
crate::win64_bridge!(fn xess_set_jitter_scale_impl(usize, f32, f32) -> i32);

/// Rust-callable wrapper for `xessSetJitterScale` (guest calls go through
/// `EXTRA_EXPORTS` + `win64_bridge!` so `x`/`y` are read from Win64 XMM1/XMM2).
pub fn xessSetJitterScale(context: usize, x: f32, y: f32) -> i32 {
    xess_set_jitter_scale_impl(context, x, y)
}

pub extern "win64" fn xessGetJitterScale(context: usize, x_out: *mut f32, y_out: *mut f32) -> i32 {
    if x_out.is_null() || y_out.is_null() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let map = xess_contexts().lock().unwrap();
    let Some(ctx) = map.get(&context) else {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    };
    unsafe {
        std::ptr::write_unaligned(x_out, ctx.jitter_scale[0]);
        std::ptr::write_unaligned(y_out, ctx.jitter_scale[1]);
    }
    XESS_RESULT_SUCCESS
}

extern "win64" fn xess_set_velocity_scale_impl(context: usize, x: f32, y: f32) -> i32 {
    if !x.is_finite() || !y.is_finite() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let mut map = xess_contexts().lock().unwrap();
    let Some(ctx) = map.get_mut(&context) else {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    };
    ctx.velocity_scale = [x, y];
    XESS_RESULT_SUCCESS
}
crate::win64_bridge!(fn xess_set_velocity_scale_impl(usize, f32, f32) -> i32);

/// Rust-callable wrapper for `xessSetVelocityScale` (guest calls go through
/// `EXTRA_EXPORTS` + `win64_bridge!` so `x`/`y` are read from Win64 XMM1/XMM2).
pub fn xessSetVelocityScale(context: usize, x: f32, y: f32) -> i32 {
    xess_set_velocity_scale_impl(context, x, y)
}

pub extern "win64" fn xessGetVelocityScale(
    context: usize,
    x_out: *mut f32,
    y_out: *mut f32,
) -> i32 {
    if x_out.is_null() || y_out.is_null() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let map = xess_contexts().lock().unwrap();
    let Some(ctx) = map.get(&context) else {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    };
    unsafe {
        std::ptr::write_unaligned(x_out, ctx.velocity_scale[0]);
        std::ptr::write_unaligned(y_out, ctx.velocity_scale[1]);
    }
    XESS_RESULT_SUCCESS
}

extern "win64" fn xess_set_exposure_multiplier_impl(context: usize, scale: f32) -> i32 {
    if !scale.is_finite() || scale <= 0.0 {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let mut map = xess_contexts().lock().unwrap();
    let Some(ctx) = map.get_mut(&context) else {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    };
    ctx.exposure_multiplier = scale;
    XESS_RESULT_SUCCESS
}
crate::win64_bridge!(fn xess_set_exposure_multiplier_impl(usize, f32) -> i32);

/// Rust-callable wrapper for `xessSetExposureMultiplier` (guest calls go through
/// `EXTRA_EXPORTS` + `win64_bridge!` so `scale` is read from Win64 XMM1).
pub fn xessSetExposureMultiplier(context: usize, scale: f32) -> i32 {
    xess_set_exposure_multiplier_impl(context, scale)
}

pub extern "win64" fn xessGetExposureMultiplier(context: usize, scale_out: *mut f32) -> i32 {
    if scale_out.is_null() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let map = xess_contexts().lock().unwrap();
    let Some(ctx) = map.get(&context) else {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    };
    unsafe {
        std::ptr::write_unaligned(scale_out, ctx.exposure_multiplier);
    }
    XESS_RESULT_SUCCESS
}

pub extern "win64" fn xessSetLoggingCallback(
    context: usize,
    _min_level: i32,
    _callback: usize,
) -> i32 {
    if context != 0 && !xess_contexts().lock().unwrap().contains_key(&context) {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    }
    XESS_RESULT_SUCCESS
}

pub extern "win64" fn xessIsOptimalDriver(context: usize) -> i32 {
    if !xess_contexts().lock().unwrap().contains_key(&context) {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    }
    XESS_RESULT_SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xess_context_lifecycle_and_validation() {
        let mut ctx = 0usize;
        assert_eq!(
            xessD3D12CreateContext(0, &mut ctx),
            XESS_RESULT_ERROR_INVALID_ARGUMENT
        );
        assert_eq!(
            xessD3D12CreateContext(1, std::ptr::null_mut()),
            XESS_RESULT_ERROR_INVALID_ARGUMENT
        );
        assert_eq!(xessD3D12CreateContext(1, &mut ctx), XESS_RESULT_SUCCESS);
        assert_ne!(ctx, 0);

        // Execute and GetInitParams before Init fail with UNINITIALIZED.
        let mut init_readback = XessD3D12InitParams::default();
        assert_eq!(
            xessD3D12GetInitParams(ctx, &mut init_readback),
            XESS_RESULT_ERROR_UNINITIALIZED
        );
        let dummy_exec = XessD3D12ExecuteParams {
            pColorTexture: 0x1000,
            pOutputTexture: 0x2000,
            ..Default::default()
        };
        assert_eq!(
            xessD3D12Execute(ctx, 0x3000, &dummy_exec),
            XESS_RESULT_ERROR_UNINITIALIZED
        );

        let init = XessD3D12InitParams {
            outputResolution: Xess2d { x: 1920, y: 1080 },
            qualitySetting: XESS_QUALITY_SETTING_QUALITY,
            initFlags: XESS_INIT_FLAG_INVERTED_DEPTH | XESS_INIT_FLAG_ENABLE_AUTOEXPOSURE,
            creationNodeMask: 1,
            visibleNodeMask: 1,
            ..Default::default()
        };
        assert_eq!(xessD3D12Init(ctx, &init), XESS_RESULT_SUCCESS);
        assert_eq!(
            xessD3D12GetInitParams(ctx, &mut init_readback),
            XESS_RESULT_SUCCESS
        );
        assert_eq!(init_readback.outputResolution, Xess2d { x: 1920, y: 1080 });
        assert_eq!(init_readback.qualitySetting, XESS_QUALITY_SETTING_QUALITY);
        assert_eq!(init_readback.initFlags, init.initFlags);

        assert_eq!(xessIsOptimalDriver(ctx), XESS_RESULT_SUCCESS);
        assert_eq!(xessDestroyContext(ctx), XESS_RESULT_SUCCESS);
        assert_eq!(xessDestroyContext(ctx), XESS_RESULT_ERROR_INVALID_CONTEXT);
        assert_eq!(xessIsOptimalDriver(ctx), XESS_RESULT_ERROR_INVALID_CONTEXT);
    }

    #[test]
    fn xess_1_3_quality_scaling_factors() {
        let mut ctx = 0usize;
        assert_eq!(xessD3D12CreateContext(1, &mut ctx), XESS_RESULT_SUCCESS);

        let out_res = Xess2d { x: 3840, y: 2160 };
        let cases = [
            (XESS_QUALITY_SETTING_ULTRA_PERFORMANCE, 1280, 720), // 3.0x
            (XESS_QUALITY_SETTING_PERFORMANCE, 1670, 939),       // 2.3x
            (XESS_QUALITY_SETTING_BALANCED, 1920, 1080),         // 2.0x
            (XESS_QUALITY_SETTING_QUALITY, 2259, 1271),          // 1.7x
            (XESS_QUALITY_SETTING_ULTRA_QUALITY, 2560, 1440),    // 1.5x
            (XESS_QUALITY_SETTING_ULTRA_QUALITY_PLUS, 2954, 1662), // 1.3x
            (XESS_QUALITY_SETTING_AA, 3840, 2160),               // 1.0x
        ];

        for (quality, want_w, want_h) in cases {
            let mut opt = Xess2d::default();
            let mut min_r = Xess2d::default();
            let mut max_r = Xess2d::default();
            assert_eq!(
                xessGetOptimalInputResolution(
                    ctx,
                    &out_res,
                    quality,
                    &mut opt,
                    &mut min_r,
                    &mut max_r
                ),
                XESS_RESULT_SUCCESS
            );
            assert_eq!(opt, Xess2d { x: want_w, y: want_h }, "quality={quality}");
            assert!(min_r.x <= opt.x && opt.x <= max_r.x);
            assert!(min_r.y <= opt.y && opt.y <= max_r.y);
            assert_eq!(max_r, out_res);

            let mut legacy_in = Xess2d::default();
            assert_eq!(
                xessGetInputResolution(ctx, &out_res, quality, &mut legacy_in),
                XESS_RESULT_SUCCESS
            );
            assert_eq!(legacy_in, opt);
        }

        let mut bad = Xess2d::default();
        assert_eq!(
            xessGetOptimalInputResolution(
                ctx,
                &out_res,
                999,
                &mut bad,
                std::ptr::null_mut(),
                std::ptr::null_mut()
            ),
            XESS_RESULT_ERROR_INVALID_ARGUMENT
        );

        assert_eq!(xessDestroyContext(ctx), XESS_RESULT_SUCCESS);
    }

    #[test]
    fn xess_scales_and_temporal_desc_mapping() {
        let mut ctx = 0usize;
        assert_eq!(xessD3D12CreateContext(1, &mut ctx), XESS_RESULT_SUCCESS);
        let init = XessD3D12InitParams {
            outputResolution: Xess2d { x: 2560, y: 1440 },
            qualitySetting: XESS_QUALITY_SETTING_BALANCED,
            initFlags: XESS_INIT_FLAG_INVERTED_DEPTH
                | XESS_INIT_FLAG_HIGH_RES_MV
                | XESS_INIT_FLAG_JITTERED_MV
                | XESS_INIT_FLAG_EXPOSURE_SCALE_TEXTURE
                | XESS_INIT_FLAG_RESPONSIVE_PIXEL_MASK,
            ..Default::default()
        };
        assert_eq!(xessD3D12Init(ctx, &init), XESS_RESULT_SUCCESS);
        assert_eq!(xessSetJitterScale(ctx, 1.0, -1.0), XESS_RESULT_SUCCESS);
        assert_eq!(xessSetVelocityScale(ctx, 2.0, -2.0), XESS_RESULT_SUCCESS);
        assert_eq!(xessSetExposureMultiplier(ctx, 1.5), XESS_RESULT_SUCCESS);
        assert_eq!(
            xessSetExposureMultiplier(ctx, 0.0),
            XESS_RESULT_ERROR_INVALID_ARGUMENT
        );
        assert_eq!(
            xessSetJitterScale(ctx, f32::NAN, 1.0),
            XESS_RESULT_ERROR_INVALID_ARGUMENT
        );

        let (mut jx, mut jy, mut vx, mut vy, mut exp) = (0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(xessGetJitterScale(ctx, &mut jx, &mut jy), XESS_RESULT_SUCCESS);
        assert_eq!(xessGetVelocityScale(ctx, &mut vx, &mut vy), XESS_RESULT_SUCCESS);
        assert_eq!(xessGetExposureMultiplier(ctx, &mut exp), XESS_RESULT_SUCCESS);
        assert_eq!((jx, jy, vx, vy, exp), (1.0, -1.0, 2.0, -2.0, 1.5));

        let state = *xess_contexts().lock().unwrap().get(&ctx).unwrap();
        let exec = XessD3D12ExecuteParams {
            pColorTexture: 0x1111,
            pVelocityTexture: 0x2222,
            pDepthTexture: 0x3333,
            pExposureScaleTexture: 0x4444,
            pResponsivePixelMaskTexture: 0x5555,
            pOutputTexture: 0x6666,
            jitterOffsetX: 0.25,
            jitterOffsetY: 0.125,
            exposureScale: 2.0,
            resetHistory: 0,
            inputWidth: 1280,
            inputHeight: 720,
            inputColorBase: Xess2d { x: 16, y: 8 },
            inputMotionVectorBase: Xess2d { x: 4, y: 2 },
            inputDepthBase: Xess2d { x: 16, y: 8 },
            inputResponsiveMaskBase: Xess2d { x: 8, y: 4 },
            outputColorBase: Xess2d { x: 32, y: 16 },
            ..Default::default()
        };

        let d_first = build_temporal_desc(&state, &exec, true);
        assert_eq!(d_first.color as usize, 0x1111);
        assert_eq!(d_first.motion as usize, 0x2222);
        assert_eq!(d_first.depth as usize, 0x3333);
        assert_eq!(d_first.exposure as usize, 0x4444);
        assert_eq!(d_first.reactive_mask as usize, 0x5555);
        assert_eq!(d_first.output as usize, 0x6666);
        assert_eq!(d_first.render_size, [1280, 720]);
        assert_eq!(d_first.upscale_size, [2560, 1440]);
        assert_eq!(d_first.color_origin, [16, 8]);
        assert_eq!(d_first.motion_origin, [4, 2]);
        assert_eq!(d_first.depth_origin, [16, 8]);
        assert_eq!(d_first.reactive_origin, [8, 4]);
        assert_eq!(d_first.output_origin, [32, 16]);
        assert_eq!(d_first.jitter_offset, [0.25, -0.125]);
        assert_eq!(d_first.motion_vector_scale, [2.0, -2.0]);
        assert!((d_first.pre_exposure - 3.0).abs() < 1e-6);
        assert!(d_first.reset);
        assert!(d_first.depth_reversed);
        assert!(d_first.output_resolution_motion_vectors);
        assert!(d_first.jittered_motion_vectors);
        assert!(d_first.dynamic_resolution);
        assert!(!d_first.auto_exposure);

        // Subsequent frame without resetHistory does not reset.
        let d_next = build_temporal_desc(&state, &exec, false);
        assert!(!d_next.reset);

        // LDR + AUTOEXPOSURE + NDC velocity check.
        let ldr_init = XessD3D12InitParams {
            outputResolution: Xess2d { x: 1920, y: 1080 },
            qualitySetting: XESS_QUALITY_SETTING_QUALITY,
            initFlags: XESS_INIT_FLAG_LDR_INPUT_COLOR
                | XESS_INIT_FLAG_ENABLE_AUTOEXPOSURE
                | XESS_INIT_FLAG_USE_NDC_VELOCITY,
            ..Default::default()
        };
        assert_eq!(xessD3D12Init(ctx, &ldr_init), XESS_RESULT_SUCCESS);
        assert_eq!(xessSetVelocityScale(ctx, 1.0, 1.0), XESS_RESULT_SUCCESS);
        let state2 = *xess_contexts().lock().unwrap().get(&ctx).unwrap();
        let d_ldr = build_temporal_desc(&state2, &exec, false);
        assert!(d_ldr.auto_exposure);
        assert!(d_ldr.exposure.is_null());
        assert!(!d_ldr.depth_reversed);
        assert!(!d_ldr.output_resolution_motion_vectors);
        assert_eq!(d_ldr.motion_vector_scale, [640.0, -360.0]);

        // Argument validation on Execute.
        assert_eq!(
            xessD3D12Execute(ctx, 0, &exec),
            XESS_RESULT_ERROR_INVALID_ARGUMENT
        );
        assert_eq!(
            xessD3D12Execute(ctx, 0x9999, std::ptr::null()),
            XESS_RESULT_ERROR_INVALID_ARGUMENT
        );
        let no_out = XessD3D12ExecuteParams {
            pOutputTexture: 0,
            ..exec
        };
        assert_eq!(
            xessD3D12Execute(ctx, 0x9999, &no_out),
            XESS_RESULT_ERROR_INVALID_ARGUMENT
        );

        assert_eq!(xessDestroyContext(ctx), XESS_RESULT_SUCCESS);
    }

    #[test]
    fn xess_version_reports_1_3_1() {
        let mut ver = [0u16; 4];
        assert_eq!(
            xessGetVersion(std::ptr::null_mut()),
            XESS_RESULT_ERROR_INVALID_ARGUMENT
        );
        assert_eq!(xessGetVersion(ver.as_mut_ptr()), XESS_RESULT_SUCCESS);
        assert_eq!(ver, [1, 3, 1, 0]);
        assert!(crate::lookup("libxess.dll", "xessSetJitterScale").is_some());
        assert!(crate::lookup("libxess.dll", "xessSetVelocityScale").is_some());
        assert!(crate::lookup("libxess.dll", "xessSetExposureMultiplier").is_some());
    }
}
