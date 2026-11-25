//! Native `nvngx_dlss.dll` / `_nvngx.dll` / `nvngx_dlssg.dll` / `nvngx_dlssd.dll`
//! (NVIDIA NGX DLSS Super Resolution) bridge over MetalFX.
//!
//! While Streamline games load `sl.interposer.dll`, games that integrate NVIDIA NGX
//! directly load `nvngx_dlss.dll` / `_nvngx.dll` and call `NVSDK_NGX_D3D12_*` /
//! `NVSDK_NGX_VULKAN_*`. (Intel XeSS lives in `libxess.rs`.)
//!
//! Capability reporting is strictly honest:
//! * D3D12 `SuperSampling` (`NVSDK_NGX_Feature_SuperSampling = 1`) is supported
//!   and dispatched through MetalFX Temporal (`upscale_temporal`, with spatial
//!   fallback when depth or motion vectors are absent).
//! * `FrameGeneration` (`11`), `DeepDVC` (`12`), `RayReconstruction` (`13`), and
//!   all Vulkan NGX features are honestly reported unavailable (`Available = 0`)
//!   and rejected with `NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED` (`0xBAD0_0001`).

#![allow(non_snake_case)]

use std::collections::HashMap;
use std::ffi::{c_char, c_void, CStr};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// `NVSDK_NGX_Result` values (`nvsdk_ngx_defs.h`).
pub const NVSDK_NGX_RESULT_SUCCESS: i32 = 1;
pub const NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED: i32 = 0xBAD0_0001u32 as i32;
pub const NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_FOUND: i32 = 0xBAD0_0004u32 as i32;
pub const NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER: i32 = 0xBAD0_0005u32 as i32;

// `NVSDK_NGX_Feature` values (`nvsdk_ngx_defs.h`).
pub const NVSDK_NGX_FEATURE_SUPER_SAMPLING: u32 = 1;
pub const NVSDK_NGX_FEATURE_IMAGE_SIGNAL_PROCESSING: u32 = 9;
pub const NVSDK_NGX_FEATURE_FRAME_GENERATION: u32 = 11;
pub const NVSDK_NGX_FEATURE_DEEP_DVC: u32 = 12;
pub const NVSDK_NGX_FEATURE_RAY_RECONSTRUCTION: u32 = 13;

static NEXT_NGX_HANDLE: AtomicUsize = AtomicUsize::new(0x4E47_5801);

const NGX_DLSS_FLAG_IS_HDR: i64 = 1 << 0;
const NGX_DLSS_FLAG_MV_LOW_RES: i64 = 1 << 1;
const NGX_DLSS_FLAG_MV_JITTERED: i64 = 1 << 2;
const NGX_DLSS_FLAG_DEPTH_INVERTED: i64 = 1 << 3;
const NGX_DLSS_FLAG_AUTO_EXPOSURE: i64 = 1 << 6;

fn ngx_feature_name(feature_id: u32) -> &'static str {
    match feature_id {
        NVSDK_NGX_FEATURE_SUPER_SAMPLING => "SuperSampling",
        NVSDK_NGX_FEATURE_IMAGE_SIGNAL_PROCESSING => "ImageSignalProcessing",
        NVSDK_NGX_FEATURE_FRAME_GENERATION => "FrameGeneration",
        NVSDK_NGX_FEATURE_DEEP_DVC => "DeepDVC",
        NVSDK_NGX_FEATURE_RAY_RECONSTRUCTION => "RayReconstruction",
        _ => "Unknown",
    }
}

#[derive(Clone, Copy, Default)]
struct NgxFeatureState {
    create_flags: i64,
    out_size: [u32; 2],
    first_eval: bool,
}

fn ngx_features() -> &'static Mutex<HashMap<usize, NgxFeatureState>> {
    static MAP: OnceLock<Mutex<HashMap<usize, NgxFeatureState>>> = OnceLock::new();
    MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

#[derive(Default)]
struct NgxParameterBlock {
    ints: HashMap<String, i64>,
    floats: HashMap<String, f64>,
    ptrs: HashMap<String, usize>,
}

fn ngx_params() -> &'static Mutex<HashMap<usize, NgxParameterBlock>> {
    static MAP: OnceLock<Mutex<HashMap<usize, NgxParameterBlock>>> = OnceLock::new();
    MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

unsafe fn read_key(name: *const u8) -> String {
    if name.is_null() {
        return String::new();
    }
    CStr::from_ptr(name as *const c_char)
        .to_string_lossy()
        .into_owned()
}

// ============================================================================
// NVSDK_NGX_Parameter vtable (used by games to set/get DLSS resolutions & resources)
// ============================================================================

unsafe extern "win64" fn ngx_param_set_ull(this: usize, name: *const u8, val: u64) {
    let key = read_key(name);
    if let Some(b) = ngx_params().lock().unwrap().get_mut(&this) {
        b.ints.insert(key, val as i64);
    }
}

unsafe extern "win64" fn ngx_param_set_f(this: usize, name: *const u8, val: f32) {
    let key = read_key(name);
    if let Some(b) = ngx_params().lock().unwrap().get_mut(&this) {
        b.floats.insert(key, val as f64);
    }
}
crate::win64_bridge!(fn ngx_param_set_f(usize, *const u8, f32) -> ());

unsafe extern "win64" fn ngx_param_set_d(this: usize, name: *const u8, val: f64) {
    let key = read_key(name);
    if let Some(b) = ngx_params().lock().unwrap().get_mut(&this) {
        b.floats.insert(key, val);
    }
}
crate::win64_bridge!(fn ngx_param_set_d(usize, *const u8, f64) -> ());

unsafe extern "win64" fn ngx_param_set_ui(this: usize, name: *const u8, val: u32) {
    let key = read_key(name);
    if let Some(b) = ngx_params().lock().unwrap().get_mut(&this) {
        b.ints.insert(key, val as i64);
    }
}

unsafe extern "win64" fn ngx_param_set_i(this: usize, name: *const u8, val: i32) {
    let key = read_key(name);
    if let Some(b) = ngx_params().lock().unwrap().get_mut(&this) {
        b.ints.insert(key, val as i64);
    }
}

unsafe extern "win64" fn ngx_param_set_ptr(this: usize, name: *const u8, val: usize) {
    let key = read_key(name);
    if let Some(b) = ngx_params().lock().unwrap().get_mut(&this) {
        b.ptrs.insert(key, val);
    }
}

unsafe extern "win64" fn ngx_param_get_ull(
    this: usize,
    name: *const u8,
    out: *mut u64,
) -> i32 {
    if out.is_null() {
        return NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER;
    }
    let key = read_key(name);
    let found = ngx_params()
        .lock()
        .unwrap()
        .get(&this)
        .and_then(|b| b.ints.get(&key).copied());
    match found {
        Some(val) => {
            std::ptr::write_unaligned(out, val as u64);
            NVSDK_NGX_RESULT_SUCCESS
        }
        None => {
            std::ptr::write_unaligned(out, 0);
            NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER
        }
    }
}

unsafe extern "win64" fn ngx_param_get_f(
    this: usize,
    name: *const u8,
    out: *mut f32,
) -> i32 {
    if out.is_null() {
        return NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER;
    }
    let key = read_key(name);
    let found = ngx_params()
        .lock()
        .unwrap()
        .get(&this)
        .and_then(|b| b.floats.get(&key).copied());
    match found {
        Some(val) => {
            std::ptr::write_unaligned(out, val as f32);
            NVSDK_NGX_RESULT_SUCCESS
        }
        None => {
            std::ptr::write_unaligned(out, 0.0);
            NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER
        }
    }
}

unsafe extern "win64" fn ngx_param_get_d(
    this: usize,
    name: *const u8,
    out: *mut f64,
) -> i32 {
    if out.is_null() {
        return NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER;
    }
    let key = read_key(name);
    let found = ngx_params()
        .lock()
        .unwrap()
        .get(&this)
        .and_then(|b| b.floats.get(&key).copied());
    match found {
        Some(val) => {
            std::ptr::write_unaligned(out, val);
            NVSDK_NGX_RESULT_SUCCESS
        }
        None => {
            std::ptr::write_unaligned(out, 0.0);
            NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER
        }
    }
}

unsafe extern "win64" fn ngx_param_get_ui(
    this: usize,
    name: *const u8,
    out: *mut u32,
) -> i32 {
    if out.is_null() {
        return NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER;
    }
    let key = read_key(name);
    let found = ngx_params()
        .lock()
        .unwrap()
        .get(&this)
        .and_then(|b| b.ints.get(&key).copied());
    match found {
        Some(val) => {
            std::ptr::write_unaligned(out, val as u32);
            NVSDK_NGX_RESULT_SUCCESS
        }
        None => {
            std::ptr::write_unaligned(out, 0);
            NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER
        }
    }
}

unsafe extern "win64" fn ngx_param_get_i(
    this: usize,
    name: *const u8,
    out: *mut i32,
) -> i32 {
    if out.is_null() {
        return NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER;
    }
    let key = read_key(name);
    let found = ngx_params()
        .lock()
        .unwrap()
        .get(&this)
        .and_then(|b| b.ints.get(&key).copied());
    match found {
        Some(val) => {
            std::ptr::write_unaligned(out, val as i32);
            NVSDK_NGX_RESULT_SUCCESS
        }
        None => {
            std::ptr::write_unaligned(out, 0);
            NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER
        }
    }
}

unsafe extern "win64" fn ngx_param_get_ptr(
    this: usize,
    name: *const u8,
    out: *mut usize,
) -> i32 {
    if out.is_null() {
        return NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER;
    }
    let key = read_key(name);
    let found = ngx_params()
        .lock()
        .unwrap()
        .get(&this)
        .and_then(|b| b.ptrs.get(&key).copied());
    match found {
        Some(val) => {
            std::ptr::write_unaligned(out, val);
            NVSDK_NGX_RESULT_SUCCESS
        }
        None => {
            std::ptr::write_unaligned(out, 0);
            NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER
        }
    }
}

unsafe extern "win64" fn ngx_param_reset(this: usize) {
    if let Some(b) = ngx_params().lock().unwrap().get_mut(&this) {
        b.ints.clear();
        b.floats.clear();
        b.ptrs.clear();
    }
}

fn dlss_quality_scale(perf_quality: i64) -> f32 {
    match perf_quality {
        0 => 2.0,   // MaxPerf
        1 => 1.724, // Balanced (~58%)
        2 => 1.5,   // MaxQuality (~66.7%)
        3 => 3.0,   // UltraPerformance (~33.3%)
        4 => 1.3,   // UltraQuality (~77%)
        5 => 1.0,   // DLAA (100%)
        _ => 1.5,
    }
}

/// Callback for `NVSDK_NGX_DLSS_GetOptimalSettingsCallback`.
pub unsafe extern "win64" fn ngx_dlss_optimal_settings_callback(params: usize) -> i32 {
    if let Some(b) = ngx_params().lock().unwrap().get_mut(&params) {
        let out_w = b.ints.get("Width").copied().unwrap_or(1920).max(64) as u32;
        let out_h = b.ints.get("Height").copied().unwrap_or(1080).max(64) as u32;
        let pq = b.ints.get("PerfQualityValue").copied().unwrap_or(2);
        let scale = dlss_quality_scale(pq);
        let render_w = ((out_w as f32 / scale).round() as u32).clamp(1, out_w);
        let render_h = ((out_h as f32 / scale).round() as u32).clamp(1, out_h);
        let min_w = ((out_w as f32 / 3.0).round() as u32).clamp(1, render_w);
        let min_h = ((out_h as f32 / 3.0).round() as u32).clamp(1, render_h);

        b.ints.insert("OutWidth".to_string(), render_w as i64);
        b.ints.insert("OutHeight".to_string(), render_h as i64);
        b.ints.insert(
            "DLSS.Render.Subrect.Dimensions.Width".to_string(),
            render_w as i64,
        );
        b.ints.insert(
            "DLSS.Render.Subrect.Dimensions.Height".to_string(),
            render_h as i64,
        );
        b.ints
            .insert("DLSS.Get.Dynamic.Max.Render.Width".to_string(), out_w as i64);
        b.ints.insert(
            "DLSS.Get.Dynamic.Max.Render.Height".to_string(),
            out_h as i64,
        );
        b.ints
            .insert("DLSS.Get.Dynamic.Min.Render.Width".to_string(), min_w as i64);
        b.ints.insert(
            "DLSS.Get.Dynamic.Min.Render.Height".to_string(),
            min_h as i64,
        );
        b.floats.insert("Sharpness".to_string(), 0.0);
        return NVSDK_NGX_RESULT_SUCCESS;
    }
    NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER
}

#[repr(C)]
struct NgxParamObject {
    #[allow(dead_code)]
    vtable: *const usize,
}

fn ngx_param_vtable() -> &'static [usize; 17] {
    static VTBL: OnceLock<[usize; 17]> = OnceLock::new();
    VTBL.get_or_init(|| {
        let slots = [
            ngx_param_set_ull as *const () as usize,
            crate::bridge_entry!(ngx_param_set_f) as usize,
            crate::bridge_entry!(ngx_param_set_d) as usize,
            ngx_param_set_ui as *const () as usize,
            ngx_param_set_i as *const () as usize,
            ngx_param_set_ptr as *const () as usize, // Set(ID3D11Resource*)
            ngx_param_set_ptr as *const () as usize, // Set(ID3D12Resource*)
            ngx_param_set_ptr as *const () as usize, // Set(void*)
            ngx_param_get_ull as *const () as usize,
            ngx_param_get_f as *const () as usize,
            ngx_param_get_d as *const () as usize,
            ngx_param_get_ui as *const () as usize,
            ngx_param_get_i as *const () as usize,
            ngx_param_get_ptr as *const () as usize, // Get(ID3D11Resource**)
            ngx_param_get_ptr as *const () as usize, // Get(ID3D12Resource**)
            ngx_param_get_ptr as *const () as usize, // Get(void**)
            ngx_param_reset as *const () as usize,
        ];
        for &fn_ptr in &slots {
            crate::register_native_va(fn_ptr as u64, fn_ptr);
        }
        let cb_ptr = ngx_dlss_optimal_settings_callback as *const () as usize;
        crate::register_native_va(cb_ptr as u64, cb_ptr);
        slots
    })
}

fn alloc_ngx_parameter_block(super_sampling_available: bool) -> usize {
    let obj = Box::into_raw(Box::new(NgxParamObject {
        vtable: ngx_param_vtable().as_ptr(),
    })) as usize;
    let mut block = NgxParameterBlock::default();
    let sr_avail = i64::from(super_sampling_available);
    let sr_init_result = if super_sampling_available {
        NVSDK_NGX_RESULT_SUCCESS as i64
    } else {
        NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED as i64
    };
    block
        .ints
        .insert("SuperSampling.Available".to_string(), sr_avail);
    block
        .ints
        .insert("SuperSampling.NeedsUpdatedDriver".to_string(), 0);
    block
        .ints
        .insert("SuperSampling.FeatureInitResult".to_string(), sr_init_result);
    block
        .ints
        .insert("RayReconstruction.Available".to_string(), 0);
    block
        .ints
        .insert("SuperSamplingDenoising.Available".to_string(), 0);
    block.ints.insert(
        "SuperSamplingDenoising.FeatureInitResult".to_string(),
        NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED as i64,
    );
    block
        .ints
        .insert("FrameGeneration.Available".to_string(), 0);
    block
        .ints
        .insert("FrameInterpolation.Available".to_string(), 0);
    block.ints.insert(
        "FrameInterpolation.FeatureInitResult".to_string(),
        NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED as i64,
    );
    block.ints.insert("DLSSG.Available".to_string(), 0);
    block.ints.insert("DLSSD.Available".to_string(), 0);
    block.ints.insert("DeepDVC.Available".to_string(), 0);
    if super_sampling_available {
        block.ptrs.insert(
            "DLSSOptimalSettingsCallback".to_string(),
            ngx_dlss_optimal_settings_callback as *const () as usize,
        );
    }
    ngx_params().lock().unwrap().insert(obj, block);
    obj
}

fn build_temporal_desc(
    b: &NgxParameterBlock,
    create_flags: i64,
    out_size: [u32; 2],
    first_eval: bool,
) -> Option<(crate::d3d12::cmdlist::TemporalUpscaleDesc, bool)> {
    let color_in = b.ptrs.get("Color").copied().unwrap_or(0);
    let color_out = b.ptrs.get("Output").copied().unwrap_or(0);
    if color_in == 0 || color_out == 0 {
        return None;
    }
    let depth = b.ptrs.get("Depth").copied().unwrap_or(0);
    let motion = b.ptrs.get("MotionVectors").copied().unwrap_or(0);
    let exposure = b.ptrs.get("ExposureTexture").copied().unwrap_or(0);
    let reactive = b
        .ptrs
        .get("DLSS.Input.Bias.Current.Color.Mask")
        .copied()
        .unwrap_or(0);
    let jx = b.floats.get("Jitter.Offset.X").copied().unwrap_or(0.0) as f32;
    let jy = b.floats.get("Jitter.Offset.Y").copied().unwrap_or(0.0) as f32;
    let mv_sx = b.floats.get("MV.Scale.X").copied().unwrap_or(1.0) as f32;
    let mv_sy = b.floats.get("MV.Scale.Y").copied().unwrap_or(1.0) as f32;
    let raw_pre_exp = b.floats.get("DLSS.Pre.Exposure").copied().unwrap_or(1.0) as f32;
    let desc_reset = b.ints.get("Reset").copied().unwrap_or(0) != 0;
    let rw = b
        .ints
        .get("DLSS.Render.Subrect.Dimensions.Width")
        .copied()
        .unwrap_or(0)
        .max(0) as u32;
    let rh = b
        .ints
        .get("DLSS.Render.Subrect.Dimensions.Height")
        .copied()
        .unwrap_or(0)
        .max(0) as u32;
    let render_size = [rw, rh];

    let pre_exposure = if raw_pre_exp.is_finite() && raw_pre_exp > 0.0 {
        raw_pre_exp
    } else {
        1.0
    };
    let is_hdr = (create_flags & NGX_DLSS_FLAG_IS_HDR) != 0;
    let auto_exposure = (create_flags & NGX_DLSS_FLAG_AUTO_EXPOSURE) != 0
        || (is_hdr && exposure == 0 && (pre_exposure - 1.0).abs() < 1e-4);
    let out_res_mv = (create_flags & NGX_DLSS_FLAG_MV_LOW_RES) == 0 && create_flags != 0;

    let desc = crate::d3d12::cmdlist::TemporalUpscaleDesc {
        output: color_out as *mut c_void,
        color: color_in as *mut c_void,
        depth: depth as *mut c_void,
        motion: motion as *mut c_void,
        exposure: exposure as *mut c_void,
        reactive_mask: reactive as *mut c_void,
        render_size,
        upscale_size: out_size,
        jitter_offset: [
            if jx.is_finite() { jx } else { 0.0 },
            if jy.is_finite() { jy } else { 0.0 },
        ],
        motion_vector_scale: [
            if mv_sx.is_finite() { mv_sx } else { 1.0 },
            if mv_sy.is_finite() { mv_sy } else { 1.0 },
        ],
        reset: desc_reset || first_eval,
        depth_reversed: (create_flags & NGX_DLSS_FLAG_DEPTH_INVERTED) != 0,
        pre_exposure,
        auto_exposure,
        dynamic_resolution: render_size != [0, 0],
        output_resolution_motion_vectors: out_res_mv,
        jittered_motion_vectors: (create_flags & NGX_DLSS_FLAG_MV_JITTERED) != 0,
        color_origin: [0, 0],
        depth_origin: [0, 0],
        motion_origin: [0, 0],
        reactive_origin: [0, 0],
        output_origin: [0, 0],
    };
    let has_temporal_inputs = depth != 0 && motion != 0;
    Some((desc, has_temporal_inputs))
}

// ============================================================================
// NVIDIA NGX D3D12 & Vulkan Exports
// ============================================================================

pub extern "win64" fn NVSDK_NGX_D3D12_Init(
    _app_id: u64,
    _app_data_path: *const u16,
    _device: usize,
    _feature_info: usize,
    _sdk_version: u32,
) -> i32 {
    NVSDK_NGX_RESULT_SUCCESS
}

pub extern "win64" fn NVSDK_NGX_D3D12_Init_Ext(
    _app_id: u64,
    _app_data_path: *const u16,
    _device: usize,
    _sdk_version: u32,
    _feature_info: usize,
) -> i32 {
    NVSDK_NGX_RESULT_SUCCESS
}

pub extern "win64" fn NVSDK_NGX_D3D12_Init_ProjectID(
    _project_id: *const u8,
    _engine_type: u32,
    _engine_version: *const u8,
    _app_data_path: *const u16,
    _device: usize,
    _feature_info: usize,
    _sdk_version: u32,
) -> i32 {
    NVSDK_NGX_RESULT_SUCCESS
}

pub extern "win64" fn NVSDK_NGX_D3D12_Shutdown() -> i32 {
    NVSDK_NGX_RESULT_SUCCESS
}

pub extern "win64" fn NVSDK_NGX_D3D12_Shutdown1(_device: usize) -> i32 {
    NVSDK_NGX_RESULT_SUCCESS
}

pub extern "win64" fn NVSDK_NGX_D3D12_GetCapabilityParameters(out_params: *mut usize) -> i32 {
    if out_params.is_null() {
        return NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER;
    }
    unsafe { std::ptr::write_unaligned(out_params, alloc_ngx_parameter_block(true)) };
    NVSDK_NGX_RESULT_SUCCESS
}

pub extern "win64" fn NVSDK_NGX_D3D12_AllocateParameters(out_params: *mut usize) -> i32 {
    if out_params.is_null() {
        return NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER;
    }
    unsafe { std::ptr::write_unaligned(out_params, alloc_ngx_parameter_block(true)) };
    NVSDK_NGX_RESULT_SUCCESS
}

pub extern "win64" fn NVSDK_NGX_D3D12_DestroyParameters(params: usize) -> i32 {
    if ngx_params().lock().unwrap().remove(&params).is_some() {
        unsafe { drop(Box::from_raw(params as *mut NgxParamObject)) };
    }
    NVSDK_NGX_RESULT_SUCCESS
}

pub extern "win64" fn NVSDK_NGX_D3D12_CreateFeature(
    _cmd_list: usize,
    feature_id: u32,
    params: usize,
    out_handle: *mut usize,
) -> i32 {
    if out_handle.is_null() {
        return NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER;
    }
    match feature_id {
        NVSDK_NGX_FEATURE_SUPER_SAMPLING | NVSDK_NGX_FEATURE_IMAGE_SIGNAL_PROCESSING => {
            let handle = NEXT_NGX_HANDLE.fetch_add(1, Ordering::Relaxed);
            let (create_flags, out_size) = ngx_params()
                .lock()
                .unwrap()
                .get(&params)
                .map(|b| {
                    let flags = b
                        .ints
                        .get("DLSS.Feature.Create.Flags")
                        .copied()
                        .unwrap_or(0);
                    let ow = b.ints.get("OutWidth").copied().unwrap_or(0).max(0) as u32;
                    let oh = b.ints.get("OutHeight").copied().unwrap_or(0).max(0) as u32;
                    (flags, [ow, oh])
                })
                .unwrap_or((0, [0, 0]));
            ngx_features().lock().unwrap().insert(
                handle,
                NgxFeatureState {
                    create_flags,
                    out_size,
                    first_eval: true,
                },
            );
            unsafe { std::ptr::write_unaligned(out_handle, handle) };
            NVSDK_NGX_RESULT_SUCCESS
        }
        other => {
            unsafe { std::ptr::write_unaligned(out_handle, 0) };
            // Native-unsupported (NVIDIA NGX SDK `nvsdk_ngx_defs.h`): NVIDIA-only
            // hardware features (FrameGeneration, DeepDVC, RayReconstruction) are
            // unavailable on non-NVIDIA GPUs (`NVSDK_NGX_Result_FAIL_FeatureNotSupported`).
            log::debug!(
                target: "d3d",
                "NVSDK_NGX_D3D12_CreateFeature: unsupported feature_id={other} ({}) -> FAIL_FeatureNotSupported",
                ngx_feature_name(other)
            );
            NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED
        }
    }
}

pub extern "win64" fn NVSDK_NGX_D3D12_EvaluateFeature(
    cmd_list: usize,
    feature_handle: usize,
    params: usize,
    _callback: usize,
) -> i32 {
    let Some((create_flags, out_size, first_eval)) = ({
        let mut f = ngx_features().lock().unwrap();
        f.get_mut(&feature_handle).map(|st| {
            let first = st.first_eval;
            st.first_eval = false;
            (st.create_flags, st.out_size, first)
        })
    }) else {
        return NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_FOUND;
    };

    let eval = ngx_params()
        .lock()
        .unwrap()
        .get(&params)
        .and_then(|b| build_temporal_desc(b, create_flags, out_size, first_eval));

    if let Some((desc, has_temporal_inputs)) = eval {
        if cmd_list != 0 {
            if crate::d3d12::cmdlist::temporal_upscale_enabled() && has_temporal_inputs {
                unsafe {
                    crate::d3d12::cmdlist::upscale_temporal(cmd_list as *mut c_void, desc);
                }
            } else {
                unsafe {
                    crate::d3d12::cmdlist::upscale_resources(
                        cmd_list as *mut c_void,
                        desc.output,
                        desc.color,
                        desc.render_size,
                        desc.upscale_size,
                    );
                }
            }
        }
    }
    NVSDK_NGX_RESULT_SUCCESS
}

pub extern "win64" fn NVSDK_NGX_D3D12_ReleaseFeature(feature_handle: usize) -> i32 {
    if ngx_features().lock().unwrap().remove(&feature_handle).is_none() {
        return NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_FOUND;
    }
    NVSDK_NGX_RESULT_SUCCESS
}

pub extern "win64" fn NVSDK_NGX_VULKAN_Init(
    _app_id: u64,
    _app_data_path: *const u16,
    _instance: usize,
    _phys_dev: usize,
    _device: usize,
    _get_inst_proc: usize,
    _get_dev_proc: usize,
    _feature_info: usize,
    _sdk_version: u32,
) -> i32 {
    // Native-unsupported (NVIDIA NGX SDK `nvsdk_ngx_vk.h`, `nvsdk_ngx_defs.h`):
    // Vulkan NGX requires an NVIDIA Vulkan driver (`VK_NVX_binary_import` /
    // `VK_NVX_image_view_handle`), returning `NVSDK_NGX_Result_FAIL_FeatureNotSupported`
    // on non-NVIDIA hardware.
    log::debug!(
        target: "d3d",
        "NVSDK_NGX_VULKAN_Init: non-NVIDIA GPU -> FAIL_FeatureNotSupported"
    );
    NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED
}

pub extern "win64" fn NVSDK_NGX_VULKAN_GetCapabilityParameters(out_params: *mut usize) -> i32 {
    if out_params.is_null() {
        return NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER;
    }
    unsafe { std::ptr::write_unaligned(out_params, alloc_ngx_parameter_block(false)) };
    NVSDK_NGX_RESULT_SUCCESS
}

pub extern "win64" fn NVSDK_NGX_VULKAN_AllocateParameters(out_params: *mut usize) -> i32 {
    if out_params.is_null() {
        return NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER;
    }
    unsafe { std::ptr::write_unaligned(out_params, alloc_ngx_parameter_block(false)) };
    NVSDK_NGX_RESULT_SUCCESS
}

pub extern "win64" fn NVSDK_NGX_VULKAN_CreateFeature(
    _cmd_buf: usize,
    feature_id: u32,
    _params: usize,
    out_handle: *mut usize,
) -> i32 {
    if !out_handle.is_null() {
        unsafe { std::ptr::write_unaligned(out_handle, 0) };
    }
    // Native-unsupported (NVIDIA NGX SDK `nvsdk_ngx_vk.h`): no NVIDIA Vulkan NGX
    // driver on Apple Silicon -> `NVSDK_NGX_Result_FAIL_FeatureNotSupported`.
    log::debug!(
        target: "d3d",
        "NVSDK_NGX_VULKAN_CreateFeature(feature_id={feature_id} {}): non-NVIDIA GPU -> FAIL_FeatureNotSupported",
        ngx_feature_name(feature_id)
    );
    NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED
}

pub extern "win64" fn NVSDK_NGX_VULKAN_EvaluateFeature(
    _cmd_buf: usize,
    _feature_handle: usize,
    _params: usize,
    _callback: usize,
) -> i32 {
    // Native-unsupported (NVIDIA NGX SDK `nvsdk_ngx_vk.h`): no NVIDIA Vulkan NGX
    // driver on Apple Silicon -> `NVSDK_NGX_Result_FAIL_FeatureNotSupported`.
    log::debug!(
        target: "d3d",
        "NVSDK_NGX_VULKAN_EvaluateFeature: non-NVIDIA GPU -> FAIL_FeatureNotSupported"
    );
    NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED
}

pub extern "win64" fn NVSDK_NGX_VULKAN_ReleaseFeature(_feature_handle: usize) -> i32 {
    // Native-unsupported (NVIDIA NGX SDK `nvsdk_ngx_vk.h`): no NVIDIA Vulkan NGX
    // driver on Apple Silicon -> `NVSDK_NGX_Result_FAIL_FeatureNotSupported`.
    log::debug!(
        target: "d3d",
        "NVSDK_NGX_VULKAN_ReleaseFeature: non-NVIDIA GPU -> FAIL_FeatureNotSupported"
    );
    NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED
}

pub extern "win64" fn NVSDK_NGX_VULKAN_Shutdown1(_device: usize) -> i32 {
    NVSDK_NGX_RESULT_SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ngx_capability_params_report_honestly_and_missing_keys_fail() {
        let mut params = 0usize;
        assert_eq!(
            NVSDK_NGX_D3D12_GetCapabilityParameters(&mut params),
            NVSDK_NGX_RESULT_SUCCESS
        );
        assert_ne!(params, 0);

        unsafe {
            let mut val: i32 = -1;
            // Only SuperSampling is available.
            assert_eq!(
                ngx_param_get_i(params, b"SuperSampling.Available\0".as_ptr(), &mut val),
                NVSDK_NGX_RESULT_SUCCESS
            );
            assert_eq!(val, 1);

            for key in [
                b"RayReconstruction.Available\0".as_slice(),
                b"SuperSamplingDenoising.Available\0".as_slice(),
                b"FrameGeneration.Available\0".as_slice(),
                b"FrameInterpolation.Available\0".as_slice(),
                b"DLSSG.Available\0".as_slice(),
                b"DLSSD.Available\0".as_slice(),
                b"DeepDVC.Available\0".as_slice(),
            ] {
                val = 99;
                assert_eq!(
                    ngx_param_get_i(params, key.as_ptr(), &mut val),
                    NVSDK_NGX_RESULT_SUCCESS
                );
                assert_eq!(val, 0, "key {:?} must be 0", CStr::from_bytes_with_nul(key));
            }

            // Missing keys MUST NOT default to 1; they return INVALID_PARAMETER and write 0.
            let mut u64_out = 77u64;
            let mut u32_out = 77u32;
            let mut i32_out = 77i32;
            let mut f32_out = 77.0f32;
            let mut f64_out = 77.0f64;
            let mut ptr_out = 77usize;
            assert_eq!(
                ngx_param_get_ull(params, b"Missing.Key\0".as_ptr(), &mut u64_out),
                NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER
            );
            assert_eq!(u64_out, 0);
            assert_eq!(
                ngx_param_get_ui(params, b"Missing.Key\0".as_ptr(), &mut u32_out),
                NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER
            );
            assert_eq!(u32_out, 0);
            assert_eq!(
                ngx_param_get_i(params, b"Missing.Key\0".as_ptr(), &mut i32_out),
                NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER
            );
            assert_eq!(i32_out, 0);
            assert_eq!(
                ngx_param_get_f(params, b"Missing.Key\0".as_ptr(), &mut f32_out),
                NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER
            );
            assert_eq!(f32_out, 0.0);
            assert_eq!(
                ngx_param_get_d(params, b"Missing.Key\0".as_ptr(), &mut f64_out),
                NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER
            );
            assert_eq!(f64_out, 0.0);
            assert_eq!(
                ngx_param_get_ptr(params, b"Missing.Ptr\0".as_ptr(), &mut ptr_out),
                NVSDK_NGX_RESULT_FAIL_INVALID_PARAMETER
            );
            assert_eq!(ptr_out, 0);

            // Vtable slot 15 (`Get(void**)`) retrieves DLSSOptimalSettingsCallback without resetting!
            let vtbl = std::ptr::read_unaligned(params as *const *const usize);
            let get_void_ptr: unsafe extern "win64" fn(usize, *const u8, *mut usize) -> i32 =
                std::mem::transmute(std::ptr::read_unaligned(vtbl.add(15)));
            let mut cb = 0usize;
            assert_eq!(
                get_void_ptr(params, b"DLSSOptimalSettingsCallback\0".as_ptr(), &mut cb),
                NVSDK_NGX_RESULT_SUCCESS
            );
            assert_eq!(cb, ngx_dlss_optimal_settings_callback as *const () as usize);

            // Set/Get round-trip + optimal settings callback.
            ngx_param_set_ui(params, b"Width\0".as_ptr(), 1920);
            ngx_param_set_ui(params, b"Height\0".as_ptr(), 1080);
            ngx_param_set_i(params, b"PerfQualityValue\0".as_ptr(), 0); // MaxPerf (2.0x)
            assert_eq!(
                ngx_dlss_optimal_settings_callback(params),
                NVSDK_NGX_RESULT_SUCCESS
            );
            let (mut ow, mut oh) = (0u32, 0u32);
            assert_eq!(
                ngx_param_get_ui(params, b"OutWidth\0".as_ptr(), &mut ow),
                NVSDK_NGX_RESULT_SUCCESS
            );
            assert_eq!(
                ngx_param_get_ui(params, b"OutHeight\0".as_ptr(), &mut oh),
                NVSDK_NGX_RESULT_SUCCESS
            );
            assert_eq!((ow, oh), (960, 540));
        }

        assert_eq!(
            NVSDK_NGX_D3D12_DestroyParameters(params),
            NVSDK_NGX_RESULT_SUCCESS
        );
    }

    #[test]
    fn ngx_d3d12_feature_validation_and_temporal_desc() {
        let mut params = 0usize;
        assert_eq!(
            NVSDK_NGX_D3D12_AllocateParameters(&mut params),
            NVSDK_NGX_RESULT_SUCCESS
        );

        unsafe {
            ngx_param_set_i(
                params,
                b"DLSS.Feature.Create.Flags\0".as_ptr(),
                (NGX_DLSS_FLAG_IS_HDR
                    | NGX_DLSS_FLAG_DEPTH_INVERTED
                    | NGX_DLSS_FLAG_MV_LOW_RES
                    | NGX_DLSS_FLAG_MV_JITTERED
                    | NGX_DLSS_FLAG_AUTO_EXPOSURE) as i32,
            );
            ngx_param_set_ui(params, b"OutWidth\0".as_ptr(), 2560);
            ngx_param_set_ui(params, b"OutHeight\0".as_ptr(), 1440);
        }

        // Unsupported features are honestly rejected.
        for bad_feat in [
            NVSDK_NGX_FEATURE_FRAME_GENERATION,
            NVSDK_NGX_FEATURE_DEEP_DVC,
            NVSDK_NGX_FEATURE_RAY_RECONSTRUCTION,
            99,
        ] {
            let mut h = 0xDEADusize;
            assert_eq!(
                NVSDK_NGX_D3D12_CreateFeature(0, bad_feat, params, &mut h),
                NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED
            );
            assert_eq!(h, 0);
        }

        // SuperSampling and ImageSignalProcessing succeed.
        let mut sr_handle = 0usize;
        assert_eq!(
            NVSDK_NGX_D3D12_CreateFeature(0, NVSDK_NGX_FEATURE_SUPER_SAMPLING, params, &mut sr_handle),
            NVSDK_NGX_RESULT_SUCCESS
        );
        assert_ne!(sr_handle, 0);

        let mut isp_handle = 0usize;
        assert_eq!(
            NVSDK_NGX_D3D12_CreateFeature(
                0,
                NVSDK_NGX_FEATURE_IMAGE_SIGNAL_PROCESSING,
                params,
                &mut isp_handle
            ),
            NVSDK_NGX_RESULT_SUCCESS
        );
        assert_ne!(isp_handle, 0);

        unsafe {
            ngx_param_set_ptr(params, b"Color\0".as_ptr(), 0x1000);
            ngx_param_set_ptr(params, b"Output\0".as_ptr(), 0x2000);
            ngx_param_set_ptr(params, b"Depth\0".as_ptr(), 0x3000);
            ngx_param_set_ptr(params, b"MotionVectors\0".as_ptr(), 0x4000);
            ngx_param_set_ptr(params, b"DLSS.Input.Bias.Current.Color.Mask\0".as_ptr(), 0x5000);
            ngx_param_set_f(params, b"Jitter.Offset.X\0".as_ptr(), 0.25);
            ngx_param_set_f(params, b"Jitter.Offset.Y\0".as_ptr(), -0.25);
            ngx_param_set_f(params, b"MV.Scale.X\0".as_ptr(), -1.0);
            ngx_param_set_f(params, b"MV.Scale.Y\0".as_ptr(), 1.0);
            ngx_param_set_f(params, b"DLSS.Pre.Exposure\0".as_ptr(), 2.0);
            ngx_param_set_ui(params, b"DLSS.Render.Subrect.Dimensions.Width\0".as_ptr(), 1280);
            ngx_param_set_ui(params, b"DLSS.Render.Subrect.Dimensions.Height\0".as_ptr(), 720);
        }

        let st = *ngx_features().lock().unwrap().get(&sr_handle).unwrap();
        let (desc, has_temporal) = {
            let guard = ngx_params().lock().unwrap();
            build_temporal_desc(guard.get(&params).unwrap(), st.create_flags, st.out_size, st.first_eval)
                .unwrap()
        };
        assert!(has_temporal);
        assert_eq!(desc.color as usize, 0x1000);
        assert_eq!(desc.output as usize, 0x2000);
        assert_eq!(desc.depth as usize, 0x3000);
        assert_eq!(desc.motion as usize, 0x4000);
        assert_eq!(desc.reactive_mask as usize, 0x5000);
        assert_eq!(desc.render_size, [1280, 720]);
        assert_eq!(desc.upscale_size, [2560, 1440]);
        assert_eq!(desc.jitter_offset, [0.25, -0.25]);
        assert_eq!(desc.motion_vector_scale, [-1.0, 1.0]);
        assert!((desc.pre_exposure - 2.0).abs() < 1e-6);
        assert!(desc.depth_reversed);
        assert!(desc.auto_exposure);
        assert!(desc.jittered_motion_vectors);
        assert!(!desc.output_resolution_motion_vectors);
        assert!(desc.reset);

        // Evaluating with an unknown handle fails honestly.
        assert_eq!(
            NVSDK_NGX_D3D12_EvaluateFeature(0, 0xBAD_9999, params, 0),
            NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_FOUND
        );

        assert_eq!(
            NVSDK_NGX_D3D12_ReleaseFeature(sr_handle),
            NVSDK_NGX_RESULT_SUCCESS
        );
        assert_eq!(
            NVSDK_NGX_D3D12_ReleaseFeature(sr_handle),
            NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_FOUND
        );
        assert_eq!(
            NVSDK_NGX_D3D12_ReleaseFeature(isp_handle),
            NVSDK_NGX_RESULT_SUCCESS
        );
        assert_eq!(
            NVSDK_NGX_D3D12_DestroyParameters(params),
            NVSDK_NGX_RESULT_SUCCESS
        );
    }

    #[test]
    fn ngx_vulkan_honestly_reports_unsupported() {
        let mut vk_params = 0usize;
        assert_eq!(
            NVSDK_NGX_VULKAN_GetCapabilityParameters(&mut vk_params),
            NVSDK_NGX_RESULT_SUCCESS
        );
        assert_ne!(vk_params, 0);
        unsafe {
            let mut sr = 1i32;
            assert_eq!(
                ngx_param_get_i(vk_params, b"SuperSampling.Available\0".as_ptr(), &mut sr),
                NVSDK_NGX_RESULT_SUCCESS
            );
            assert_eq!(sr, 0);
        }

        assert_eq!(
            NVSDK_NGX_VULKAN_Init(0, std::ptr::null(), 0, 0, 0, 0, 0, 0, 0),
            NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED
        );
        let mut h = 0x1234usize;
        assert_eq!(
            NVSDK_NGX_VULKAN_CreateFeature(0, NVSDK_NGX_FEATURE_SUPER_SAMPLING, vk_params, &mut h),
            NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED
        );
        assert_eq!(h, 0);
        assert_eq!(
            NVSDK_NGX_VULKAN_EvaluateFeature(0, 1, vk_params, 0),
            NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED
        );
        assert_eq!(
            NVSDK_NGX_VULKAN_ReleaseFeature(1),
            NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED
        );
        assert_eq!(
            NVSDK_NGX_D3D12_DestroyParameters(vk_params),
            NVSDK_NGX_RESULT_SUCCESS
        );
    }
}
