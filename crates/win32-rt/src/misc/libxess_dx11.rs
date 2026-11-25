//! Native `libxess_dx11.dll` (Intel XeSS 1.3 Direct3D 11) bridge over MetalFX Temporal.
//!
//! Implements all 51 retail `libxess_dx11.dll` exports (`xess.h` / `xess_d3d11.h`
//! plus Intel D3D11/D3D12 driver-extension entry points) on Apple Silicon:
//! * Stores full context state from `xessD3D11Init`, `xessSetJitterScale`,
//!   `xessSetVelocityScale`, `xessSetExposureMultiplier`, `xessForceLegacyScaleFactors`,
//!   and `xessSelectNetworkModel`.
//! * Computes Intel XeSS 1.3 (and optional XeSS 1.2 legacy) quality-mode render
//!   resolutions in `xessGetOptimalInputResolution` and `xessGetInputResolution`.
//! * Extracts underlying `d3d12_metal` resources from the D3D11 `ID3D11Resource`
//!   COM pointers in `xessD3D11Execute`, flushes the D3D11 immediate context onto
//!   the shared Metal 4 queue, and dispatches `CommandList::upscale_temporal_with_backend`
//!   (MetalFX Temporal), falling back to `upscale_resource` (MetalFX Spatial /
//!   bilinear compute when `PORT_METALFX=0` or depth/velocity textures are null).
//! * Bridges all float-taking Win64 exports through `win64_bridge!` + `EXTRA_EXPORTS`
//!   so ARM64 unpacks Win64 XMM1/XMM2/XMM3 + GPR slots identically to x86-64.

#![allow(non_snake_case)]

use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use d3d12_metal::metal::cmdlist::{CommandAllocator, CommandList};
use d3d12_metal::metal::metalfx::{self, TemporalUpscaleParams, UpscaleBackend};
use d3d12_metal::metal::queue::{ListType, Queue};

use super::libxess::{
    compute_optimal_input_resolutions, quality_scale_factor_legacy, resolve_temporal_inputs,
    write_xess_version, Xess2d, XESS_INIT_FLAG_EXTERNAL_DESCRIPTOR_HEAP, XESS_INIT_FLAG_NONE,
    XESS_QUALITY_SETTING_QUALITY, XESS_RESULT_ERROR_DEVICE, XESS_RESULT_ERROR_INVALID_ARGUMENT,
    XESS_RESULT_ERROR_INVALID_CONTEXT, XESS_RESULT_ERROR_NOT_IMPLEMENTED,
    XESS_RESULT_ERROR_UNINITIALIZED, XESS_RESULT_ERROR_UNSUPPORTED, XESS_RESULT_SUCCESS,
};
use crate::d3d11::device::{device_core_from_ptr, DeviceCore};
use crate::d3d11::resources::{resource_from_ptr, ResourceCore};
use crate::types::E_NOINTERFACE;

// Float-taking Win64 DLL exports go through EXTRA_EXPORTS + win64_bridge!
// so ARM64 unpacks Win64 XMM1..3 + GPR slots identically to x86-64.
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
    crate::Export {
        name: "xessSetContextParameterF",
        ptr: crate::Ptr(crate::bridge_entry!(xess_set_context_parameter_f_impl)),
    },
    crate::Export {
        name: "INTC_D3D11_XessSetJitterScale",
        ptr: crate::Ptr(crate::bridge_entry!(intc_set_jitter_scale_impl)),
    },
    crate::Export {
        name: "INTC_D3D11_XessSetVelocityScale",
        ptr: crate::Ptr(crate::bridge_entry!(intc_set_velocity_scale_impl)),
    },
    crate::Export {
        name: "INTC_D3D11_XessSetExposureMultiplier",
        ptr: crate::Ptr(crate::bridge_entry!(intc_set_exposure_multiplier_impl)),
    },
];
pub static ORDINALS: &[(u16, &str)] = &[];

/// Context parameter ID accepted by retail `xessGetContextParameterP` to query
/// the underlying device handle (`0xFF10_0000`, 8 bytes).
pub const XESS_CONTEXT_PARAM_D3D11_DEVICE: u32 = 0xFF10_0000;

/// `xess_d3d11_init_params_t` (`xess_d3d11.h`, `#pragma pack(8)`, 16 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct XessD3D11InitParams {
    pub outputResolution: Xess2d,
    pub qualitySetting: u32,
    pub initFlags: u32,
}
const _: () = assert!(std::mem::size_of::<XessD3D11InitParams>() == 0x10);
const _: () = assert!(std::mem::offset_of!(XessD3D11InitParams, outputResolution) == 0x00);
const _: () = assert!(std::mem::offset_of!(XessD3D11InitParams, qualitySetting) == 0x08);
const _: () = assert!(std::mem::offset_of!(XessD3D11InitParams, initFlags) == 0x0C);

/// `xess_d3d11_execute_params_t` (`xess_d3d11.h`, `#pragma pack(8)`, 120 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct XessD3D11ExecuteParams {
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
}
const _: () = assert!(std::mem::size_of::<XessD3D11ExecuteParams>() == 0x78);
const _: () = assert!(std::mem::offset_of!(XessD3D11ExecuteParams, pColorTexture) == 0x00);
const _: () = assert!(std::mem::offset_of!(XessD3D11ExecuteParams, pVelocityTexture) == 0x08);
const _: () = assert!(std::mem::offset_of!(XessD3D11ExecuteParams, pDepthTexture) == 0x10);
const _: () = assert!(std::mem::offset_of!(XessD3D11ExecuteParams, pExposureScaleTexture) == 0x18);
const _: () =
    assert!(std::mem::offset_of!(XessD3D11ExecuteParams, pResponsivePixelMaskTexture) == 0x20);
const _: () = assert!(std::mem::offset_of!(XessD3D11ExecuteParams, pOutputTexture) == 0x28);
const _: () = assert!(std::mem::offset_of!(XessD3D11ExecuteParams, jitterOffsetX) == 0x30);
const _: () = assert!(std::mem::offset_of!(XessD3D11ExecuteParams, jitterOffsetY) == 0x34);
const _: () = assert!(std::mem::offset_of!(XessD3D11ExecuteParams, exposureScale) == 0x38);
const _: () = assert!(std::mem::offset_of!(XessD3D11ExecuteParams, resetHistory) == 0x3C);
const _: () = assert!(std::mem::offset_of!(XessD3D11ExecuteParams, inputWidth) == 0x40);
const _: () = assert!(std::mem::offset_of!(XessD3D11ExecuteParams, inputHeight) == 0x44);
const _: () = assert!(std::mem::offset_of!(XessD3D11ExecuteParams, inputColorBase) == 0x48);
const _: () = assert!(std::mem::offset_of!(XessD3D11ExecuteParams, inputMotionVectorBase) == 0x50);
const _: () = assert!(std::mem::offset_of!(XessD3D11ExecuteParams, inputDepthBase) == 0x58);
const _: () =
    assert!(std::mem::offset_of!(XessD3D11ExecuteParams, inputResponsiveMaskBase) == 0x60);
const _: () = assert!(std::mem::offset_of!(XessD3D11ExecuteParams, reserved0) == 0x68);
const _: () = assert!(std::mem::offset_of!(XessD3D11ExecuteParams, outputColorBase) == 0x70);

/// `xess_properties_t` (`xess.h`, `#pragma pack(8)`, 24 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct XessProperties {
    pub requiredDescriptorCount: u32,
    pub tempBufferHeapSize: u64,
    pub tempTextureHeapSize: u64,
}
const _: () = assert!(std::mem::size_of::<XessProperties>() == 0x18);
const _: () = assert!(std::mem::offset_of!(XessProperties, tempBufferHeapSize) == 0x08);
const _: () = assert!(std::mem::offset_of!(XessProperties, tempTextureHeapSize) == 0x10);

/// `xess_dump_parameters_t` (`xess_debug.h`, `#pragma pack(8)`, 24 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XessDumpParameters {
    pub path: *const u8,
    pub frame_idx: u32,
    pub frame_count: u32,
    pub dump_elements_mask: u32,
    pub _pad: u32,
}
const _: () = assert!(std::mem::size_of::<XessDumpParameters>() == 0x18);

/// `xess_profiling_data_t` (`xess_debug.h`, `#pragma pack(8)`, 16 bytes).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct XessProfilingData {
    pub frame_count: u64,
    pub frames: *const c_void,
}
unsafe impl Sync for XessProfilingData {}
const _: () = assert!(std::mem::size_of::<XessProfilingData>() == 0x10);

static EMPTY_PROFILING_DATA: XessProfilingData = XessProfilingData {
    frame_count: 0,
    frames: std::ptr::null(),
};

const EXEC_RING_LEN: usize = 4;

/// Reusable Metal 4 command allocator ring + command list for dispatching XeSS
/// MetalFX passes directly onto a D3D11 device's immediate queue.
struct ExecRing {
    queue: Arc<Queue>,
    allocs: Vec<CommandAllocator>,
    vals: [u64; EXEC_RING_LEN],
    idx: usize,
    cl: CommandList,
}
unsafe impl Send for ExecRing {}

impl ExecRing {
    fn new(queue: Arc<Queue>) -> Result<Self, String> {
        let mut allocs = Vec::with_capacity(EXEC_RING_LEN);
        for _ in 0..EXEC_RING_LEN {
            allocs.push(
                CommandAllocator::new(ListType::Direct)
                    .ok_or("libxess_dx11: CommandAllocator::new failed")?,
            );
        }
        let cl = CommandList::new(ListType::Direct)
            .ok_or("libxess_dx11: CommandList::new failed")?;
        Ok(Self {
            queue,
            allocs,
            vals: [0; EXEC_RING_LEN],
            idx: 0,
            cl,
        })
    }

    fn begin_next(&mut self) -> Result<usize, String> {
        let slot = self.idx;
        if self.vals[slot] != 0 {
            self.queue.timeline.wait_cpu(self.vals[slot], u64::MAX);
        }
        self.allocs[slot].reset();
        self.cl.begin(&self.allocs[slot])?;
        Ok(slot)
    }

    fn finish_submit(&mut self, slot: usize) -> u64 {
        let v = self.queue.submit(&[&self.cl.cb]);
        self.vals[slot] = v;
        self.idx = (slot + 1) % self.allocs.len();
        v
    }
}

static NEXT_XESS_DX11_HANDLE: AtomicUsize = AtomicUsize::new(0x5845_1101);

struct XessD3D11ContextState {
    device: usize,
    device_core: Option<Arc<DeviceCore>>,
    exec_ring: Arc<Mutex<Option<ExecRing>>>,
    initialized: bool,
    output_resolution: [u32; 2],
    quality_setting: u32,
    init_flags: u32,
    jitter_scale: [f32; 2],
    velocity_scale: [f32; 2],
    exposure_multiplier: f32,
    legacy_scale_factors: bool,
    network_model: u32,
    first_execute: bool,
    logged_first_execute: bool,
}

#[derive(Clone)]
struct ExecuteSnapshot {
    device_core: Option<Arc<DeviceCore>>,
    exec_ring: Arc<Mutex<Option<ExecRing>>>,
    output_resolution: [u32; 2],
    init_flags: u32,
    jitter_scale: [f32; 2],
    velocity_scale: [f32; 2],
    exposure_multiplier: f32,
    first_execute: bool,
    should_log_first: bool,
}

impl XessD3D11ContextState {
    fn new(device: usize, device_core: Option<Arc<DeviceCore>>) -> Self {
        Self {
            device,
            device_core,
            exec_ring: Arc::new(Mutex::new(None)),
            initialized: false,
            output_resolution: [0, 0],
            quality_setting: XESS_QUALITY_SETTING_QUALITY,
            init_flags: XESS_INIT_FLAG_NONE,
            jitter_scale: [1.0, 1.0],
            velocity_scale: [1.0, 1.0],
            exposure_multiplier: 1.0,
            legacy_scale_factors: false,
            network_model: 0,
            first_execute: true,
            logged_first_execute: false,
        }
    }
}

fn xess_dx11_contexts() -> &'static Mutex<HashMap<usize, XessD3D11ContextState>> {
    static MAP: OnceLock<Mutex<HashMap<usize, XessD3D11ContextState>>> = OnceLock::new();
    MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

#[inline]
fn is_plausible_com_ptr(ptr: usize) -> bool {
    ptr >= 0x10000 && (ptr & (std::mem::align_of::<usize>() - 1)) == 0
}

fn extract_optional_texture(ptr: usize, label: &str) -> Result<Option<Arc<ResourceCore>>, i32> {
    if ptr == 0 {
        return Ok(None);
    }
    if !is_plausible_com_ptr(ptr) {
        log::warn!(
            target: "d3d",
            "xessD3D11Execute: invalid/unaligned D3D11 {label} texture pointer {ptr:#x}"
        );
        return Err(XESS_RESULT_ERROR_INVALID_ARGUMENT);
    }
    let Some(res) = (unsafe { resource_from_ptr(ptr as *mut c_void) }) else {
        log::warn!(
            target: "d3d",
            "xessD3D11Execute: foreign/invalid D3D11 {label} texture {ptr:#x}"
        );
        return Err(XESS_RESULT_ERROR_INVALID_ARGUMENT);
    };
    if res.is_buffer() {
        log::warn!(
            target: "d3d",
            "xessD3D11Execute: {label} resource {ptr:#x} is a buffer, expected texture"
        );
        return Err(XESS_RESULT_ERROR_INVALID_ARGUMENT);
    }
    Ok(Some(res))
}

fn execute_d3d11_upscale(
    snap: &ExecuteSnapshot,
    p: &XessD3D11ExecuteParams,
) -> Result<UpscaleBackend, i32> {
    let color_core = extract_optional_texture(p.pColorTexture, "color")?
        .ok_or(XESS_RESULT_ERROR_INVALID_ARGUMENT)?;
    let output_core = extract_optional_texture(p.pOutputTexture, "output")?
        .ok_or(XESS_RESULT_ERROR_INVALID_ARGUMENT)?;
    let depth_core = extract_optional_texture(p.pDepthTexture, "depth")?;
    let velocity_core = extract_optional_texture(p.pVelocityTexture, "velocity")?;
    let exposure_core = extract_optional_texture(p.pExposureScaleTexture, "exposure")?;
    let reactive_core = extract_optional_texture(p.pResponsivePixelMaskTexture, "reactive")?;

    let device = snap
        .device_core
        .clone()
        .unwrap_or_else(|| color_core.device.clone());

    // Sync any multi-device shared resources BEFORE taking `device.imm_exec`
    // to preserve lock ordering (see `d3d11::resources::ResourceCore::sync_other_devices`).
    color_core.sync_other_devices(&device);
    output_core.sync_other_devices(&device);
    if let Some(ref r) = depth_core {
        r.sync_other_devices(&device);
    }
    if let Some(ref r) = velocity_core {
        r.sync_other_devices(&device);
    }
    if let Some(ref r) = exposure_core {
        r.sync_other_devices(&device);
    }
    if let Some(ref r) = reactive_core {
        r.sync_other_devices(&device);
    }

    let color_mtl = color_core.current_mtl();
    let output_mtl = output_core.current_mtl();
    let depth_mtl = depth_core.as_ref().map(|r| r.current_mtl());
    let velocity_mtl = velocity_core.as_ref().map(|r| r.current_mtl());
    let exposure_mtl = exposure_core.as_ref().map(|r| r.current_mtl());
    let reactive_mtl = reactive_core.as_ref().map(|r| r.current_mtl());

    let resolved = resolve_temporal_inputs(
        snap.init_flags,
        snap.output_resolution,
        [p.inputWidth, p.inputHeight],
        [p.jitterOffsetX, p.jitterOffsetY],
        snap.jitter_scale,
        snap.velocity_scale,
        p.exposureScale,
        snap.exposure_multiplier,
        exposure_mtl.is_some(),
        p.resetHistory != 0,
        snap.first_execute,
    );

    let use_temporal =
        metalfx::is_temporal_enabled() && depth_mtl.is_some() && velocity_mtl.is_some();

    // Serialize with the D3D11 immediate context: flush all prior immediate draws/clears
    // onto `device.queue`, then encode and submit our MetalFX command buffer onto the
    // exact same Metal 4 queue before releasing the lock.
    let mut imm_exec = device.imm_exec.lock().unwrap();
    imm_exec.flush();

    let mut ring_guard = snap.exec_ring.lock().unwrap();
    let need_new_ring = match ring_guard.as_ref() {
        Some(existing) => !Arc::ptr_eq(&existing.queue, &device.queue),
        None => true,
    };
    if need_new_ring {
        if let Some(old_ring) = ring_guard.take() {
            let old_q = old_ring.queue.clone();
            old_q.defer_release(Box::new(old_ring));
        }
        let fresh = ExecRing::new(device.queue.clone()).map_err(|e| {
            log::error!(target: "d3d", "xessD3D11Execute: {e}");
            XESS_RESULT_ERROR_DEVICE
        })?;
        *ring_guard = Some(fresh);
    }
    let ring = ring_guard.as_mut().unwrap();

    let slot = ring.begin_next().map_err(|e| {
        log::error!(target: "d3d", "xessD3D11Execute: begin command list failed: {e}");
        XESS_RESULT_ERROR_DEVICE
    })?;

    ring.cl.barrier();
    let backend_res = if use_temporal {
        let depth_ref = depth_mtl.as_deref().unwrap();
        let velocity_ref = velocity_mtl.as_deref().unwrap();
        let mut params = TemporalUpscaleParams::new(
            &output_mtl,
            &color_mtl,
            depth_ref,
            velocity_ref,
            resolved.render_size,
            resolved.upscale_size,
        );
        if resolved.use_exposure_texture {
            params.exposure = exposure_mtl.as_deref();
        }
        params.reactive_mask = reactive_mtl.as_deref();
        params.jitter_offset = resolved.jitter_offset;
        params.motion_vector_scale = resolved.motion_vector_scale;
        params.reset = resolved.reset;
        params.depth_reversed = resolved.depth_reversed;
        params.pre_exposure = resolved.pre_exposure;
        params.auto_exposure = resolved.auto_exposure;
        params.dynamic_resolution = resolved.dynamic_resolution;
        params.output_resolution_motion_vectors = resolved.output_resolution_motion_vectors;
        params.jittered_motion_vectors = resolved.jittered_motion_vectors;
        params.color_origin = [p.inputColorBase.x, p.inputColorBase.y];
        params.depth_origin = [p.inputDepthBase.x, p.inputDepthBase.y];
        params.motion_origin = [p.inputMotionVectorBase.x, p.inputMotionVectorBase.y];
        params.reactive_origin = [p.inputResponsiveMaskBase.x, p.inputResponsiveMaskBase.y];
        params.output_origin = [p.outputColorBase.x, p.outputColorBase.y];
        ring.cl.upscale_temporal_with_backend(&params)
    } else {
        ring.cl
            .upscale_resource_subrect(
                &output_mtl,
                &color_mtl,
                resolved.render_size,
                resolved.upscale_size,
                [p.inputColorBase.x, p.inputColorBase.y],
                [p.outputColorBase.x, p.outputColorBase.y],
            )
            .map(|()| {
                if metalfx::is_enabled() {
                    UpscaleBackend::SpatialMetalFx
                } else {
                    UpscaleBackend::BilinearCompute
                }
            })
    };

    let backend = match backend_res {
        Ok(b) => b,
        Err(e) => {
            log::warn!(target: "d3d", "xessD3D11Execute: upscale failed: {e}");
            let _ = ring.cl.close();
            return Err(XESS_RESULT_ERROR_DEVICE);
        }
    };

    // Ensure MetalFX / fallback writes are globally visible to the next D3D11 immediate command buffer.
    ring.cl.barrier();
    if let Err(e) = ring.cl.close() {
        log::error!(target: "d3d", "xessD3D11Execute: close command list failed: {e}");
        return Err(XESS_RESULT_ERROR_DEVICE);
    }

    let _ = ring.finish_submit(slot);
    device.queue.defer_release(Box::new((
        color_mtl,
        output_mtl,
        depth_mtl,
        velocity_mtl,
        exposure_mtl,
        reactive_mtl,
    )));
    drop(ring_guard);
    drop(imm_exec);

    if snap.should_log_first {
        log::info!(
            target: "d3d",
            "xessD3D11Execute: backend={backend:?} render={}x{} upscale={}x{} jitter=({:.3},{:.3}) mv_scale=({:.2},{:.2}) pre_exp={:.3} auto_exp={} depth_rev={} out_res_mv={} jit_mv={} depth={} mv={} exp={} reactive={}",
            resolved.render_size[0],
            resolved.render_size[1],
            resolved.upscale_size[0],
            resolved.upscale_size[1],
            resolved.jitter_offset[0],
            resolved.jitter_offset[1],
            resolved.motion_vector_scale[0],
            resolved.motion_vector_scale[1],
            resolved.pre_exposure,
            resolved.auto_exposure,
            resolved.depth_reversed,
            resolved.output_resolution_motion_vectors,
            resolved.jittered_motion_vectors,
            p.pDepthTexture != 0,
            p.pVelocityTexture != 0,
            resolved.use_exposure_texture && p.pExposureScaleTexture != 0,
            p.pResponsivePixelMaskTexture != 0,
        );
    }

    Ok(backend)
}

// ---------------------------------------------------------------------------
// Public XeSS D3D11 API exports (`xess.h` / `xess_d3d11.h` / `xess_debug.h`)
// ---------------------------------------------------------------------------

pub extern "win64" fn xessGetVersion(version_out: *mut u16) -> i32 {
    write_xess_version(version_out, 1, 3, 1, 0)
}

pub extern "win64" fn xessGetIntelXeFXVersion(context: usize, version_out: *mut u16) -> i32 {
    if !xess_dx11_contexts().lock().unwrap().contains_key(&context) {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    }
    write_xess_version(version_out, 0, 0, 0, 0)
}

pub extern "win64" fn xessD3D11CreateContext(device: usize, out_context: *mut usize) -> i32 {
    if !out_context.is_null() {
        unsafe { std::ptr::write_unaligned(out_context, 0) };
    }
    if device == 0 || out_context.is_null() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let device_core = if is_plausible_com_ptr(device) {
        unsafe { device_core_from_ptr(device as *mut c_void) }
    } else {
        None
    };
    let ctx = NEXT_XESS_DX11_HANDLE.fetch_add(1, Ordering::Relaxed);
    xess_dx11_contexts()
        .lock()
        .unwrap()
        .insert(ctx, XessD3D11ContextState::new(device, device_core));
    unsafe { std::ptr::write_unaligned(out_context, ctx) };
    XESS_RESULT_SUCCESS
}

pub extern "win64" fn xessD3D11Init(
    context: usize,
    init_params: *const XessD3D11InitParams,
) -> i32 {
    if init_params.is_null() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let params = unsafe { std::ptr::read_unaligned(init_params) };
    if params.outputResolution.x == 0
        || params.outputResolution.y == 0
        || (params.initFlags & XESS_INIT_FLAG_EXTERNAL_DESCRIPTOR_HEAP) != 0
    {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let mut map = xess_dx11_contexts().lock().unwrap();
    let Some(ctx) = map.get_mut(&context) else {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    };
    if quality_scale_factor_legacy(params.qualitySetting, ctx.legacy_scale_factors).is_none() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    ctx.initialized = true;
    ctx.output_resolution = [params.outputResolution.x, params.outputResolution.y];
    ctx.quality_setting = params.qualitySetting;
    ctx.init_flags = params.initFlags;
    ctx.first_execute = true;
    ctx.logged_first_execute = false;
    log::info!(
        target: "d3d",
        "xessD3D11Init(dev={:#x}): output={}x{} quality={} flags={:#x} temporal={}",
        ctx.device,
        ctx.output_resolution[0],
        ctx.output_resolution[1],
        ctx.quality_setting,
        ctx.init_flags,
        metalfx::is_temporal_enabled(),
    );
    XESS_RESULT_SUCCESS
}

pub extern "win64" fn xessD3D11GetInitParams(
    context: usize,
    init_params_out: *mut XessD3D11InitParams,
) -> i32 {
    if init_params_out.is_null() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let map = xess_dx11_contexts().lock().unwrap();
    let Some(ctx) = map.get(&context) else {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    };
    if !ctx.initialized {
        return XESS_RESULT_ERROR_UNINITIALIZED;
    }
    let out = XessD3D11InitParams {
        outputResolution: Xess2d {
            x: ctx.output_resolution[0],
            y: ctx.output_resolution[1],
        },
        qualitySetting: ctx.quality_setting,
        initFlags: ctx.init_flags,
    };
    unsafe { std::ptr::write_unaligned(init_params_out, out) };
    XESS_RESULT_SUCCESS
}

pub extern "win64" fn xessD3D11Execute(
    context: usize,
    exec_params: *const XessD3D11ExecuteParams,
) -> i32 {
    if exec_params.is_null() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let p = unsafe { std::ptr::read_unaligned(exec_params) };
    if p.pColorTexture == 0 || p.pOutputTexture == 0 {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }

    let snap = {
        let mut map = xess_dx11_contexts().lock().unwrap();
        let Some(ctx) = map.get_mut(&context) else {
            return XESS_RESULT_ERROR_INVALID_CONTEXT;
        };
        if !ctx.initialized {
            return XESS_RESULT_ERROR_UNINITIALIZED;
        }
        let first_execute = ctx.first_execute;
        ctx.first_execute = false;
        let should_log_first = !ctx.logged_first_execute;
        ctx.logged_first_execute = true;
        ExecuteSnapshot {
            device_core: ctx.device_core.clone(),
            exec_ring: ctx.exec_ring.clone(),
            output_resolution: ctx.output_resolution,
            init_flags: ctx.init_flags,
            jitter_scale: ctx.jitter_scale,
            velocity_scale: ctx.velocity_scale,
            exposure_multiplier: ctx.exposure_multiplier,
            first_execute,
            should_log_first,
        }
    };

    match execute_d3d11_upscale(&snap, &p) {
        Ok(_) => XESS_RESULT_SUCCESS,
        Err(rc) => rc,
    }
}

pub extern "win64" fn xessDestroyContext(context: usize) -> i32 {
    let Some(ctx) = xess_dx11_contexts().lock().unwrap().remove(&context) else {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    };
    if let Ok(mut guard) = ctx.exec_ring.lock() {
        if let Some(ring) = guard.take() {
            let q = ring.queue.clone();
            q.defer_release(Box::new(ring));
        }
    }
    XESS_RESULT_SUCCESS
}

pub extern "win64" fn xessGetProperties(
    context: usize,
    output_resolution: *const Xess2d,
    binding_properties: *mut XessProperties,
) -> i32 {
    if !xess_dx11_contexts().lock().unwrap().contains_key(&context) {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    }
    if output_resolution.is_null() || binding_properties.is_null() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let out = unsafe { std::ptr::read_unaligned(output_resolution) };
    if out.x == 0 || out.y == 0 {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    unsafe { std::ptr::write_unaligned(binding_properties, XessProperties::default()) };
    // Retail `libxess_dx11.dll` returns `XESS_RESULT_ERROR_UNSUPPORTED` because D3D11
    // does not expose explicit temp heap / descriptor properties.
    XESS_RESULT_ERROR_UNSUPPORTED
}

pub extern "win64" fn xessGetOptimalInputResolution(
    context: usize,
    out_res: *const Xess2d,
    quality: u32,
    input_res_optimal: *mut Xess2d,
    input_res_min: *mut Xess2d,
    input_res_max: *mut Xess2d,
) -> i32 {
    let legacy = {
        let map = xess_dx11_contexts().lock().unwrap();
        let Some(ctx) = map.get(&context) else {
            return XESS_RESULT_ERROR_INVALID_CONTEXT;
        };
        ctx.legacy_scale_factors
    };
    if out_res.is_null() || input_res_optimal.is_null() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let out = unsafe { std::ptr::read_unaligned(out_res) };
    let Some((optimal, min_res, max_res)) = compute_optimal_input_resolutions(out, quality, legacy)
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

extern "win64" fn xess_set_jitter_scale_impl(context: usize, x: f32, y: f32) -> i32 {
    if !x.is_finite() || !y.is_finite() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let mut map = xess_dx11_contexts().lock().unwrap();
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
    let map = xess_dx11_contexts().lock().unwrap();
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
    let mut map = xess_dx11_contexts().lock().unwrap();
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
    let map = xess_dx11_contexts().lock().unwrap();
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
    let mut map = xess_dx11_contexts().lock().unwrap();
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
    let map = xess_dx11_contexts().lock().unwrap();
    let Some(ctx) = map.get(&context) else {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    };
    unsafe {
        std::ptr::write_unaligned(scale_out, ctx.exposure_multiplier);
    }
    XESS_RESULT_SUCCESS
}

extern "win64" fn xess_set_context_parameter_f_impl(
    context: usize,
    param: u32,
    value: f32,
) -> i32 {
    if !xess_dx11_contexts().lock().unwrap().contains_key(&context) {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    }
    if !value.is_finite() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    crate::stub!("xessSetContextParameterF(param={param:#x}, value={value}) -> NOT_IMPLEMENTED");
    XESS_RESULT_ERROR_NOT_IMPLEMENTED
}
crate::win64_bridge!(fn xess_set_context_parameter_f_impl(usize, u32, f32) -> i32);

/// Rust-callable wrapper for `xessSetContextParameterF` (guest calls go through
/// `EXTRA_EXPORTS` + `win64_bridge!` so `value` is read from Win64 XMM2).
pub fn xessSetContextParameterF(context: usize, param: u32, value: f32) -> i32 {
    xess_set_context_parameter_f_impl(context, param, value)
}

pub extern "win64" fn xessGetContextParameterP(
    context: usize,
    param: u32,
    size: u32,
    value_out: *mut c_void,
) -> i32 {
    let map = xess_dx11_contexts().lock().unwrap();
    let Some(ctx) = map.get(&context) else {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    };
    if param != XESS_CONTEXT_PARAM_D3D11_DEVICE
        || size != std::mem::size_of::<usize>() as u32
        || value_out.is_null()
    {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    unsafe { std::ptr::write_unaligned(value_out as *mut usize, ctx.device) };
    XESS_RESULT_SUCCESS
}

pub extern "win64" fn xessSetLoggingCallback(
    context: usize,
    _min_level: i32,
    _callback: usize,
) -> i32 {
    if context != 0 && !xess_dx11_contexts().lock().unwrap().contains_key(&context) {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    }
    XESS_RESULT_SUCCESS
}

pub extern "win64" fn xessIsOptimalDriver(context: usize) -> i32 {
    if !xess_dx11_contexts().lock().unwrap().contains_key(&context) {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    }
    XESS_RESULT_SUCCESS
}

pub extern "win64" fn xessForceLegacyScaleFactors(context: usize, force: u8) -> i32 {
    let mut map = xess_dx11_contexts().lock().unwrap();
    let Some(ctx) = map.get_mut(&context) else {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    };
    ctx.legacy_scale_factors = force != 0;
    XESS_RESULT_SUCCESS
}

pub extern "win64" fn xessStartDump(
    context: usize,
    dump_params: *const XessDumpParameters,
) -> i32 {
    if !xess_dx11_contexts().lock().unwrap().contains_key(&context) {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    }
    if dump_params.is_null() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    let dp = unsafe { std::ptr::read_unaligned(dump_params) };
    if dp.path.is_null() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    crate::stub!("xessStartDump -> XESS_RESULT_ERROR_NOT_IMPLEMENTED");
    XESS_RESULT_ERROR_NOT_IMPLEMENTED
}

pub extern "win64" fn xessSelectNetworkModel(context: usize, network: u32) -> i32 {
    let mut map = xess_dx11_contexts().lock().unwrap();
    let Some(ctx) = map.get_mut(&context) else {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    };
    ctx.network_model = network;
    XESS_RESULT_SUCCESS
}

pub extern "win64" fn xessGetProfilingData(
    context: usize,
    out_data: *mut *const XessProfilingData,
) -> i32 {
    if !xess_dx11_contexts().lock().unwrap().contains_key(&context) {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    }
    if out_data.is_null() {
        return XESS_RESULT_ERROR_INVALID_ARGUMENT;
    }
    unsafe { std::ptr::write_unaligned(out_data, &EMPTY_PROFILING_DATA as *const _) };
    XESS_RESULT_SUCCESS
}

pub extern "win64" fn xessGetPipelineBuildStatus(context: usize) -> i32 {
    if !xess_dx11_contexts().lock().unwrap().contains_key(&context) {
        return XESS_RESULT_ERROR_INVALID_CONTEXT;
    }
    // Retail `libxess_dx11.dll` returns `XESS_RESULT_ERROR_UNSUPPORTED` because D3D11
    // does not support async pipeline pre-builds (`xessD3D12BuildPipelines`).
    XESS_RESULT_ERROR_UNSUPPORTED
}

// ---------------------------------------------------------------------------
// Intel D3D11 Driver Extension XeSS Exports (`INTC_D3D11_Xess*`)
// Each takes `ext_ctx` as Win64 slot 1 followed by the standard XeSS arguments.
// ---------------------------------------------------------------------------

pub extern "win64" fn INTC_D3D11_XessCreateContext(
    ext_ctx: usize,
    out_context: *mut usize,
) -> i32 {
    xessD3D11CreateContext(ext_ctx, out_context)
}

pub extern "win64" fn INTC_D3D11_XessDestroyContext(_ext_ctx: usize, context: usize) -> i32 {
    xessDestroyContext(context)
}

pub extern "win64" fn INTC_D3D11_XessInit(
    _ext_ctx: usize,
    context: usize,
    init_params: *const XessD3D11InitParams,
) -> i32 {
    xessD3D11Init(context, init_params)
}

pub extern "win64" fn INTC_D3D11_XessGetInitParams(
    _ext_ctx: usize,
    context: usize,
    init_params_out: *mut XessD3D11InitParams,
) -> i32 {
    xessD3D11GetInitParams(context, init_params_out)
}

pub extern "win64" fn INTC_D3D11_XessExecute(
    _ext_ctx: usize,
    context: usize,
    exec_params: *const XessD3D11ExecuteParams,
) -> i32 {
    xessD3D11Execute(context, exec_params)
}

pub extern "win64" fn INTC_D3D11_XessGetVersion(_ext_ctx: usize, version_out: *mut u16) -> i32 {
    xessGetVersion(version_out)
}

pub extern "win64" fn INTC_D3D11_XessGetIntelXeFXVersion(
    _ext_ctx: usize,
    context: usize,
    version_out: *mut u16,
) -> i32 {
    xessGetIntelXeFXVersion(context, version_out)
}

pub extern "win64" fn INTC_D3D11_XessGetInputResolution(
    _ext_ctx: usize,
    context: usize,
    out_res: *const Xess2d,
    quality: u32,
    input_res: *mut Xess2d,
) -> i32 {
    xessGetInputResolution(context, out_res, quality, input_res)
}

pub extern "win64" fn INTC_D3D11_XessGetOptimalInputResolution(
    _ext_ctx: usize,
    context: usize,
    out_res: *const Xess2d,
    quality: u32,
    input_res_optimal: *mut Xess2d,
    input_res_min: *mut Xess2d,
    input_res_max: *mut Xess2d,
) -> i32 {
    xessGetOptimalInputResolution(
        context,
        out_res,
        quality,
        input_res_optimal,
        input_res_min,
        input_res_max,
    )
}

pub extern "win64" fn INTC_D3D11_XessGetJitterScale(
    _ext_ctx: usize,
    context: usize,
    x_out: *mut f32,
    y_out: *mut f32,
) -> i32 {
    xessGetJitterScale(context, x_out, y_out)
}

extern "win64" fn intc_set_jitter_scale_impl(
    _ext_ctx: usize,
    context: usize,
    x: f32,
    y: f32,
) -> i32 {
    xess_set_jitter_scale_impl(context, x, y)
}
crate::win64_bridge!(fn intc_set_jitter_scale_impl(usize, usize, f32, f32) -> i32);

/// Rust-callable wrapper for `INTC_D3D11_XessSetJitterScale` (guest calls go through
/// `EXTRA_EXPORTS` + `win64_bridge!` so `x`/`y` are read from Win64 XMM2/XMM3).
pub fn INTC_D3D11_XessSetJitterScale(ext_ctx: usize, context: usize, x: f32, y: f32) -> i32 {
    intc_set_jitter_scale_impl(ext_ctx, context, x, y)
}

pub extern "win64" fn INTC_D3D11_XessGetVelocityScale(
    _ext_ctx: usize,
    context: usize,
    x_out: *mut f32,
    y_out: *mut f32,
) -> i32 {
    xessGetVelocityScale(context, x_out, y_out)
}

extern "win64" fn intc_set_velocity_scale_impl(
    _ext_ctx: usize,
    context: usize,
    x: f32,
    y: f32,
) -> i32 {
    xess_set_velocity_scale_impl(context, x, y)
}
crate::win64_bridge!(fn intc_set_velocity_scale_impl(usize, usize, f32, f32) -> i32);

/// Rust-callable wrapper for `INTC_D3D11_XessSetVelocityScale` (guest calls go through
/// `EXTRA_EXPORTS` + `win64_bridge!` so `x`/`y` are read from Win64 XMM2/XMM3).
pub fn INTC_D3D11_XessSetVelocityScale(ext_ctx: usize, context: usize, x: f32, y: f32) -> i32 {
    intc_set_velocity_scale_impl(ext_ctx, context, x, y)
}

pub extern "win64" fn INTC_D3D11_XessGetExposureMultiplier(
    _ext_ctx: usize,
    context: usize,
    scale_out: *mut f32,
) -> i32 {
    xessGetExposureMultiplier(context, scale_out)
}

extern "win64" fn intc_set_exposure_multiplier_impl(
    _ext_ctx: usize,
    context: usize,
    scale: f32,
) -> i32 {
    xess_set_exposure_multiplier_impl(context, scale)
}
crate::win64_bridge!(fn intc_set_exposure_multiplier_impl(usize, usize, f32) -> i32);

/// Rust-callable wrapper for `INTC_D3D11_XessSetExposureMultiplier` (guest calls go through
/// `EXTRA_EXPORTS` + `win64_bridge!` so `scale` is read from Win64 XMM2).
pub fn INTC_D3D11_XessSetExposureMultiplier(ext_ctx: usize, context: usize, scale: f32) -> i32 {
    intc_set_exposure_multiplier_impl(ext_ctx, context, scale)
}

pub extern "win64" fn INTC_D3D11_XessSetLoggingCallback(
    _ext_ctx: usize,
    context: usize,
    min_level: i32,
    callback: usize,
) -> i32 {
    xessSetLoggingCallback(context, min_level, callback)
}

pub extern "win64" fn INTC_D3D11_XessIsOptimalDriver(_ext_ctx: usize, context: usize) -> i32 {
    xessIsOptimalDriver(context)
}

pub extern "win64" fn INTC_D3D11_XessSelectNetworkModel(
    _ext_ctx: usize,
    context: usize,
    network: u32,
) -> i32 {
    xessSelectNetworkModel(context, network)
}

pub extern "win64" fn INTC_D3D11_XessGetProfilingData(
    _ext_ctx: usize,
    context: usize,
    out_data: *mut *const XessProfilingData,
) -> i32 {
    xessGetProfilingData(context, out_data)
}

pub extern "win64" fn INTC_D3D11_XessForceLegacyScaleFactors(
    _ext_ctx: usize,
    context: usize,
    force: u8,
) -> i32 {
    xessForceLegacyScaleFactors(context, force)
}

// ---------------------------------------------------------------------------
// Intel D3D12 Driver Extension Crash-Dump / Callback Exports (`INTC_D3D12_*`)
// ---------------------------------------------------------------------------

pub extern "win64" fn INTC_D3D12_GetApplicationInfo(
    _ext_ctx: usize,
    _app_info: *mut c_void,
) -> i32 {
    crate::stub!("INTC_D3D12_GetApplicationInfo -> E_NOINTERFACE");
    E_NOINTERFACE
}

pub extern "win64" fn INTC_D3D12_GetGpuCrashDumpBufferInfo(
    _ext_ctx: usize,
    _buffer_info: *mut c_void,
) {
    crate::stub!("INTC_D3D12_GetGpuCrashDumpBufferInfo (no-op)");
}

pub extern "win64" fn INTC_D3D12_GpuCrashDumpOnCommandListCreate(
    _ext_ctx: usize,
    _cmd_list: usize,
) {
    crate::stub!("INTC_D3D12_GpuCrashDumpOnCommandListCreate (no-op)");
}

pub extern "win64" fn INTC_D3D12_GpuCrashDumpOnCommandListDestroy(
    _ext_ctx: usize,
    _cmd_list: usize,
) {
    crate::stub!("INTC_D3D12_GpuCrashDumpOnCommandListDestroy (no-op)");
}

pub extern "win64" fn INTC_D3D12_GpuCrashDumpOnDeviceDestroy(_ext_ctx: usize) {
    crate::stub!("INTC_D3D12_GpuCrashDumpOnDeviceDestroy (no-op)");
}

pub extern "win64" fn INTC_D3D12_RegisterDriverCallbacks(
    _ext_ctx: usize,
    _callbacks: *const c_void,
) {
    crate::stub!("INTC_D3D12_RegisterDriverCallbacks (no-op)");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::d3d11::com::com_release;
    use crate::d3d11::types::{
        bind_flag, clear_flag, cpu_access, driver_type, map_type, usage, D3D11_MAPPED_SUBRESOURCE,
        D3D11_SDK_VERSION, D3D11_TEXTURE2D_DESC, DXGI_SAMPLE_DESC,
    };
    use crate::d3d11::D3D11CreateDevice;
    use crate::misc::libxess::{
        XESS_INIT_FLAG_ENABLE_AUTOEXPOSURE, XESS_INIT_FLAG_HIGH_RES_MV,
        XESS_INIT_FLAG_INVERTED_DEPTH, XESS_QUALITY_SETTING_AA, XESS_QUALITY_SETTING_BALANCED,
        XESS_QUALITY_SETTING_PERFORMANCE, XESS_QUALITY_SETTING_QUALITY,
        XESS_QUALITY_SETTING_ULTRA_PERFORMANCE, XESS_QUALITY_SETTING_ULTRA_QUALITY,
        XESS_QUALITY_SETTING_ULTRA_QUALITY_PLUS,
    };
    use crate::types::{HRESULT, S_OK};

    const RETAIL_EXPORTS: &[&str] = &[
        "INTC_D3D11_XessCreateContext",
        "INTC_D3D11_XessDestroyContext",
        "INTC_D3D11_XessExecute",
        "INTC_D3D11_XessForceLegacyScaleFactors",
        "INTC_D3D11_XessGetExposureMultiplier",
        "INTC_D3D11_XessGetInitParams",
        "INTC_D3D11_XessGetInputResolution",
        "INTC_D3D11_XessGetIntelXeFXVersion",
        "INTC_D3D11_XessGetJitterScale",
        "INTC_D3D11_XessGetOptimalInputResolution",
        "INTC_D3D11_XessGetProfilingData",
        "INTC_D3D11_XessGetVelocityScale",
        "INTC_D3D11_XessGetVersion",
        "INTC_D3D11_XessInit",
        "INTC_D3D11_XessIsOptimalDriver",
        "INTC_D3D11_XessSelectNetworkModel",
        "INTC_D3D11_XessSetExposureMultiplier",
        "INTC_D3D11_XessSetJitterScale",
        "INTC_D3D11_XessSetLoggingCallback",
        "INTC_D3D11_XessSetVelocityScale",
        "INTC_D3D12_GetApplicationInfo",
        "INTC_D3D12_GetGpuCrashDumpBufferInfo",
        "INTC_D3D12_GpuCrashDumpOnCommandListCreate",
        "INTC_D3D12_GpuCrashDumpOnCommandListDestroy",
        "INTC_D3D12_GpuCrashDumpOnDeviceDestroy",
        "INTC_D3D12_RegisterDriverCallbacks",
        "xessD3D11CreateContext",
        "xessD3D11Execute",
        "xessD3D11GetInitParams",
        "xessD3D11Init",
        "xessDestroyContext",
        "xessForceLegacyScaleFactors",
        "xessGetContextParameterP",
        "xessGetExposureMultiplier",
        "xessGetInputResolution",
        "xessGetIntelXeFXVersion",
        "xessGetJitterScale",
        "xessGetOptimalInputResolution",
        "xessGetPipelineBuildStatus",
        "xessGetProfilingData",
        "xessGetProperties",
        "xessGetVelocityScale",
        "xessGetVersion",
        "xessIsOptimalDriver",
        "xessSelectNetworkModel",
        "xessSetContextParameterF",
        "xessSetExposureMultiplier",
        "xessSetJitterScale",
        "xessSetLoggingCallback",
        "xessSetVelocityScale",
        "xessStartDump",
    ];

    unsafe fn vtbl_slot(obj: *mut c_void, slot: usize) -> usize {
        let vt = std::ptr::read_unaligned(obj as *const *const usize);
        std::ptr::read_unaligned(vt.add(slot))
    }

    fn f16_to_f32(h: u16) -> f32 {
        let s = if h & 0x8000 != 0 { -1.0 } else { 1.0 };
        let e = ((h >> 10) & 0x1F) as i32;
        let m = (h & 0x3FF) as f32;
        s * match e {
            0 => m * 2f32.powi(-24),
            31 => {
                if m == 0.0 {
                    f32::INFINITY
                } else {
                    f32::NAN
                }
            }
            _ => (1.0 + m / 1024.0) * 2f32.powi(e - 15),
        }
    }

    #[test]
    fn libxess_dx11_all_51_retail_exports_present() {
        assert_eq!(RETAIL_EXPORTS.len(), 51);
        let dll = crate::find_builtin("libxess_dx11.dll")
            .expect("libxess_dx11.dll must be registered as a builtin DLL");
        assert_eq!(
            dll.exports.len(),
            51,
            "expected exactly 51 exports on libxess_dx11.dll, got {}",
            dll.exports.len()
        );
        for &name in RETAIL_EXPORTS {
            assert!(
                crate::lookup("libxess_dx11.dll", name).is_some(),
                "missing libxess_dx11.dll export: {name}"
            );
        }
    }

    #[test]
    fn libxess_dx11_lifecycle_validation_and_aux_exports() {
        let mut ver = [0u16; 4];
        assert_eq!(
            xessGetVersion(std::ptr::null_mut()),
            XESS_RESULT_ERROR_INVALID_ARGUMENT
        );
        assert_eq!(xessGetVersion(ver.as_mut_ptr()), XESS_RESULT_SUCCESS);
        assert_eq!(ver, [1, 3, 1, 0]);

        let mut intc_ver = [0u16; 4];
        assert_eq!(
            INTC_D3D11_XessGetVersion(0xCAFE, intc_ver.as_mut_ptr()),
            XESS_RESULT_SUCCESS
        );
        assert_eq!(intc_ver, [1, 3, 1, 0]);

        let mut ctx = 0usize;
        assert_eq!(
            xessD3D11CreateContext(0, &mut ctx),
            XESS_RESULT_ERROR_INVALID_ARGUMENT
        );
        assert_eq!(
            xessD3D11CreateContext(1, std::ptr::null_mut()),
            XESS_RESULT_ERROR_INVALID_ARGUMENT
        );
        assert_eq!(xessD3D11CreateContext(0x1234, &mut ctx), XESS_RESULT_SUCCESS);
        assert_ne!(ctx, 0);

        // Execute and GetInitParams before Init fail with UNINITIALIZED.
        let mut init_out = XessD3D11InitParams::default();
        assert_eq!(
            xessD3D11GetInitParams(ctx, &mut init_out),
            XESS_RESULT_ERROR_UNINITIALIZED
        );
        let dummy_exec = XessD3D11ExecuteParams {
            pColorTexture: 0x1000,
            pOutputTexture: 0x2000,
            ..Default::default()
        };
        assert_eq!(
            xessD3D11Execute(ctx, &dummy_exec),
            XESS_RESULT_ERROR_UNINITIALIZED
        );

        // D3D11 rejects XESS_INIT_FLAG_EXTERNAL_DESCRIPTOR_HEAP.
        let bad_init = XessD3D11InitParams {
            outputResolution: Xess2d { x: 1920, y: 1080 },
            qualitySetting: XESS_QUALITY_SETTING_QUALITY,
            initFlags: XESS_INIT_FLAG_EXTERNAL_DESCRIPTOR_HEAP,
        };
        assert_eq!(
            xessD3D11Init(ctx, &bad_init),
            XESS_RESULT_ERROR_INVALID_ARGUMENT
        );

        let init = XessD3D11InitParams {
            outputResolution: Xess2d { x: 1920, y: 1080 },
            qualitySetting: XESS_QUALITY_SETTING_QUALITY,
            initFlags: XESS_INIT_FLAG_INVERTED_DEPTH | XESS_INIT_FLAG_ENABLE_AUTOEXPOSURE,
        };
        assert_eq!(xessD3D11Init(ctx, &init), XESS_RESULT_SUCCESS);
        assert_eq!(xessD3D11GetInitParams(ctx, &mut init_out), XESS_RESULT_SUCCESS);
        assert_eq!(init_out, init);

        // Context parameter P returns stored device pointer for 0xFF10_0000 (8 bytes).
        let mut dev_out = 0usize;
        assert_eq!(
            xessGetContextParameterP(
                ctx,
                XESS_CONTEXT_PARAM_D3D11_DEVICE,
                std::mem::size_of::<usize>() as u32,
                &mut dev_out as *mut usize as *mut c_void,
            ),
            XESS_RESULT_SUCCESS
        );
        assert_eq!(dev_out, 0x1234);
        assert_eq!(
            xessGetContextParameterP(ctx, 1, 8, &mut dev_out as *mut usize as *mut c_void),
            XESS_RESULT_ERROR_INVALID_ARGUMENT
        );

        // XeFX version, properties, pipeline build status, network model, profiling, dump.
        let mut xefx = [9u16; 4];
        assert_eq!(
            xessGetIntelXeFXVersion(ctx, xefx.as_mut_ptr()),
            XESS_RESULT_SUCCESS
        );
        assert_eq!(xefx, [0, 0, 0, 0]);

        let out_res = Xess2d { x: 1920, y: 1080 };
        let mut props = XessProperties {
            requiredDescriptorCount: 99,
            tempBufferHeapSize: 123,
            tempTextureHeapSize: 456,
        };
        assert_eq!(
            xessGetProperties(ctx, &out_res, &mut props),
            XESS_RESULT_ERROR_UNSUPPORTED
        );
        assert_eq!(props, XessProperties::default());
        assert_eq!(xessGetPipelineBuildStatus(ctx), XESS_RESULT_ERROR_UNSUPPORTED);
        assert_eq!(xessSelectNetworkModel(ctx, 1), XESS_RESULT_SUCCESS);
        assert_eq!(xessIsOptimalDriver(ctx), XESS_RESULT_SUCCESS);
        assert_eq!(xessSetLoggingCallback(ctx, 0, 0), XESS_RESULT_SUCCESS);
        assert_eq!(
            xessSetContextParameterF(ctx, 0, 1.0),
            XESS_RESULT_ERROR_NOT_IMPLEMENTED
        );

        let mut prof: *const XessProfilingData = std::ptr::null();
        assert_eq!(xessGetProfilingData(ctx, &mut prof), XESS_RESULT_SUCCESS);
        assert!(!prof.is_null());
        unsafe {
            assert_eq!(std::ptr::read_unaligned(prof).frame_count, 0);
        }

        let dump_ok = XessDumpParameters {
            path: c"/tmp".as_ptr() as *const u8,
            frame_idx: 0,
            frame_count: 1,
            dump_elements_mask: 1,
            _pad: 0,
        };
        assert_eq!(
            xessStartDump(ctx, &dump_ok),
            XESS_RESULT_ERROR_NOT_IMPLEMENTED
        );
        let dump_null_path = XessDumpParameters {
            path: std::ptr::null(),
            ..dump_ok
        };
        assert_eq!(
            xessStartDump(ctx, &dump_null_path),
            XESS_RESULT_ERROR_INVALID_ARGUMENT
        );

        // INTC_D3D12_* exports behave honestly without crashing.
        assert_eq!(
            INTC_D3D12_GetApplicationInfo(0, std::ptr::null_mut()),
            E_NOINTERFACE
        );
        INTC_D3D12_GetGpuCrashDumpBufferInfo(0, std::ptr::null_mut());
        INTC_D3D12_GpuCrashDumpOnCommandListCreate(0, 0);
        INTC_D3D12_GpuCrashDumpOnCommandListDestroy(0, 0);
        INTC_D3D12_GpuCrashDumpOnDeviceDestroy(0);
        INTC_D3D12_RegisterDriverCallbacks(0, std::ptr::null());

        assert_eq!(xessDestroyContext(ctx), XESS_RESULT_SUCCESS);
        assert_eq!(xessDestroyContext(ctx), XESS_RESULT_ERROR_INVALID_CONTEXT);
        assert_eq!(xessIsOptimalDriver(ctx), XESS_RESULT_ERROR_INVALID_CONTEXT);
    }

    #[test]
    fn libxess_dx11_optimal_input_resolution_queries_and_legacy_scales() {
        let mut ctx = 0usize;
        assert_eq!(xessD3D11CreateContext(1, &mut ctx), XESS_RESULT_SUCCESS);

        let out_res = Xess2d { x: 3840, y: 2160 };
        let cases_1_3 = [
            (XESS_QUALITY_SETTING_ULTRA_PERFORMANCE, 1280, 720), // 3.0x
            (XESS_QUALITY_SETTING_PERFORMANCE, 1670, 939),       // 2.3x
            (XESS_QUALITY_SETTING_BALANCED, 1920, 1080),         // 2.0x
            (XESS_QUALITY_SETTING_QUALITY, 2259, 1271),          // 1.7x
            (XESS_QUALITY_SETTING_ULTRA_QUALITY, 2560, 1440),    // 1.5x
            (XESS_QUALITY_SETTING_ULTRA_QUALITY_PLUS, 2954, 1662), // 1.3x
            (XESS_QUALITY_SETTING_AA, 3840, 2160),               // 1.0x
        ];

        for (quality, want_w, want_h) in cases_1_3 {
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

            // Also verify INTC_D3D11_Xess* delegation produces identical results.
            let mut intc_opt = Xess2d::default();
            assert_eq!(
                INTC_D3D11_XessGetOptimalInputResolution(
                    0xDEAD,
                    ctx,
                    &out_res,
                    quality,
                    &mut intc_opt,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                XESS_RESULT_SUCCESS
            );
            assert_eq!(intc_opt, opt);
        }

        // Toggle legacy XeSS 1.2 scale factors (Performance=2.0x, Quality=1.5x).
        assert_eq!(xessForceLegacyScaleFactors(ctx, 1), XESS_RESULT_SUCCESS);
        let mut perf_legacy = Xess2d::default();
        let mut qual_legacy = Xess2d::default();
        assert_eq!(
            xessGetInputResolution(ctx, &out_res, XESS_QUALITY_SETTING_PERFORMANCE, &mut perf_legacy),
            XESS_RESULT_SUCCESS
        );
        assert_eq!(
            xessGetInputResolution(ctx, &out_res, XESS_QUALITY_SETTING_QUALITY, &mut qual_legacy),
            XESS_RESULT_SUCCESS
        );
        assert_eq!(perf_legacy, Xess2d { x: 1920, y: 1080 });
        assert_eq!(qual_legacy, Xess2d { x: 2560, y: 1440 });

        assert_eq!(
            xessGetOptimalInputResolution(
                ctx,
                &out_res,
                999,
                &mut perf_legacy,
                std::ptr::null_mut(),
                std::ptr::null_mut()
            ),
            XESS_RESULT_ERROR_INVALID_ARGUMENT
        );

        assert_eq!(xessDestroyContext(ctx), XESS_RESULT_SUCCESS);
    }

    #[test]
    fn libxess_dx11_scale_round_trip_and_aarch64_win64_bridge_slots() {
        let mut ctx = 0usize;
        assert_eq!(xessD3D11CreateContext(1, &mut ctx), XESS_RESULT_SUCCESS);

        // Direct Rust wrappers round-trip.
        assert_eq!(xessSetJitterScale(ctx, 1.5, -0.75), XESS_RESULT_SUCCESS);
        assert_eq!(xessSetVelocityScale(ctx, 2.25, -3.5), XESS_RESULT_SUCCESS);
        assert_eq!(xessSetExposureMultiplier(ctx, 1.75), XESS_RESULT_SUCCESS);
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
        assert_eq!((jx, jy, vx, vy, exp), (1.5, -0.75, 2.25, -3.5, 1.75));

        // INTC_D3D11_XessSet* / Get* round-trip.
        assert_eq!(
            INTC_D3D11_XessSetJitterScale(0xBEEF, ctx, -0.5, 0.5),
            XESS_RESULT_SUCCESS
        );
        assert_eq!(
            INTC_D3D11_XessSetVelocityScale(0xBEEF, ctx, 4.0, -4.0),
            XESS_RESULT_SUCCESS
        );
        assert_eq!(
            INTC_D3D11_XessSetExposureMultiplier(0xBEEF, ctx, 2.5),
            XESS_RESULT_SUCCESS
        );
        assert_eq!(
            INTC_D3D11_XessGetJitterScale(0xBEEF, ctx, &mut jx, &mut jy),
            XESS_RESULT_SUCCESS
        );
        assert_eq!(
            INTC_D3D11_XessGetVelocityScale(0xBEEF, ctx, &mut vx, &mut vy),
            XESS_RESULT_SUCCESS
        );
        assert_eq!(
            INTC_D3D11_XessGetExposureMultiplier(0xBEEF, ctx, &mut exp),
            XESS_RESULT_SUCCESS
        );
        assert_eq!((jx, jy, vx, vy, exp), (-0.5, 0.5, 4.0, -4.0, 2.5));

        // On aarch64, directly invoke every win64_bridge! thunk using exact Win64 register slots
        // (slot 1 -> x0, slot 2 -> x1/xmm1, slot 3 -> x2/xmm2, slot 4 -> x3/xmm3).
        #[cfg(target_arch = "aarch64")]
        unsafe {
            let f = |v: f32| f64::from_bits(v.to_bits() as u64);
            // xessSetJitterScale(rcx=ctx, xmm1=0.25, xmm2=-0.125)
            assert_eq!(
                xess_set_jitter_scale_impl::bridge(
                    ctx as u64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    f(999.0), f(0.25), f(-0.125), f(999.0),
                ),
                XESS_RESULT_SUCCESS
            );
            // xessSetVelocityScale(rcx=ctx, xmm1=1280.0, xmm2=-720.0)
            assert_eq!(
                xess_set_velocity_scale_impl::bridge(
                    ctx as u64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    f(999.0), f(1280.0), f(-720.0), f(999.0),
                ),
                XESS_RESULT_SUCCESS
            );
            // xessSetExposureMultiplier(rcx=ctx, xmm1=3.25)
            assert_eq!(
                xess_set_exposure_multiplier_impl::bridge(
                    ctx as u64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    f(999.0), f(3.25), f(999.0), f(999.0),
                ),
                XESS_RESULT_SUCCESS
            );
            // xessSetContextParameterF(rcx=ctx, edx=7, xmm2=0.5)
            assert_eq!(
                xess_set_context_parameter_f_impl::bridge(
                    ctx as u64, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    f(999.0), f(999.0), f(0.5), f(999.0),
                ),
                XESS_RESULT_ERROR_NOT_IMPLEMENTED
            );
            assert_eq!(xessGetJitterScale(ctx, &mut jx, &mut jy), XESS_RESULT_SUCCESS);
            assert_eq!(xessGetVelocityScale(ctx, &mut vx, &mut vy), XESS_RESULT_SUCCESS);
            assert_eq!(xessGetExposureMultiplier(ctx, &mut exp), XESS_RESULT_SUCCESS);
            assert_eq!((jx, jy, vx, vy, exp), (0.25, -0.125, 1280.0, -720.0, 3.25));

            // INTC_D3D11_XessSet*(rcx=ext_ctx, rdx=ctx, xmm2=x, xmm3=y)
            assert_eq!(
                intc_set_jitter_scale_impl::bridge(
                    0xAA, ctx as u64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    f(999.0), f(999.0), f(-1.0), f(2.0),
                ),
                XESS_RESULT_SUCCESS
            );
            assert_eq!(
                intc_set_velocity_scale_impl::bridge(
                    0xAA, ctx as u64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    f(999.0), f(999.0), f(640.0), f(-360.0),
                ),
                XESS_RESULT_SUCCESS
            );
            assert_eq!(
                intc_set_exposure_multiplier_impl::bridge(
                    0xAA, ctx as u64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    f(999.0), f(999.0), f(0.5), f(999.0),
                ),
                XESS_RESULT_SUCCESS
            );
            assert_eq!(xessGetJitterScale(ctx, &mut jx, &mut jy), XESS_RESULT_SUCCESS);
            assert_eq!(xessGetVelocityScale(ctx, &mut vx, &mut vy), XESS_RESULT_SUCCESS);
            assert_eq!(xessGetExposureMultiplier(ctx, &mut exp), XESS_RESULT_SUCCESS);
            assert_eq!((jx, jy, vx, vy, exp), (-1.0, 2.0, 640.0, -360.0, 0.5));
        }

        assert_eq!(xessDestroyContext(ctx), XESS_RESULT_SUCCESS);
    }

    #[test]
    fn libxess_dx11_real_d3d11_metalfx_temporal_and_spatial_upscale_on_gpu() {
        if d3d12_metal::metal::device().is_none() {
            return;
        }
        unsafe {
            let mut dev: *mut c_void = std::ptr::null_mut();
            let mut imm: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                D3D11CreateDevice(
                    std::ptr::null_mut(),
                    driver_type::HARDWARE,
                    std::ptr::null_mut(),
                    0,
                    std::ptr::null(),
                    0,
                    D3D11_SDK_VERSION,
                    &mut dev,
                    std::ptr::null_mut(),
                    &mut imm,
                ),
                S_OK
            );
            assert!(!dev.is_null() && !imm.is_null());

            type CreateTex2dFn = unsafe extern "win64" fn(
                *mut c_void,
                *const D3D11_TEXTURE2D_DESC,
                *const c_void,
                *mut *mut c_void,
            ) -> HRESULT;
            type CreateRtvFn = unsafe extern "win64" fn(
                *mut c_void,
                *mut c_void,
                *const c_void,
                *mut *mut c_void,
            ) -> HRESULT;
            type CreateDsvFn = unsafe extern "win64" fn(
                *mut c_void,
                *mut c_void,
                *const c_void,
                *mut *mut c_void,
            ) -> HRESULT;
            type ClearRtvFn = unsafe extern "win64" fn(*mut c_void, *mut c_void, *const [f32; 4]);
            type CopyResFn = unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut c_void);
            type MapFn = unsafe extern "win64" fn(
                *mut c_void,
                *mut c_void,
                u32,
                u32,
                u32,
                *mut D3D11_MAPPED_SUBRESOURCE,
            ) -> HRESULT;
            type UnmapFn = unsafe extern "win64" fn(*mut c_void, *mut c_void, u32);

            let create_tex2d: CreateTex2dFn = std::mem::transmute(vtbl_slot(dev, 5));
            let create_rtv: CreateRtvFn = std::mem::transmute(vtbl_slot(dev, 9));
            let create_dsv: CreateDsvFn = std::mem::transmute(vtbl_slot(dev, 10));
            let clear_rtv: ClearRtvFn = std::mem::transmute(vtbl_slot(imm, 50));
            let copy_res: CopyResFn = std::mem::transmute(vtbl_slot(imm, 47));
            let map_fn: MapFn = std::mem::transmute(vtbl_slot(imm, 14));
            let unmap_fn: UnmapFn = std::mem::transmute(vtbl_slot(imm, 15));

            const RGBA16_FLOAT: u32 = 10;
            const RG16_FLOAT: u32 = 34;
            const D32_FLOAT: u32 = 40;
            const R8_UNORM: u32 = 61;

            let mk_desc = |w: u32, h: u32, fmt: u32, u: u32, bind: u32, cpu: u32| {
                D3D11_TEXTURE2D_DESC {
                    Width: w,
                    Height: h,
                    MipLevels: 1,
                    ArraySize: 1,
                    Format: fmt,
                    SampleDesc: DXGI_SAMPLE_DESC {
                        Count: 1,
                        Quality: 0,
                    },
                    Usage: u,
                    BindFlags: bind,
                    CPUAccessFlags: cpu,
                    MiscFlags: 0,
                }
            };

            let color_desc = mk_desc(
                64,
                64,
                RGBA16_FLOAT,
                usage::DEFAULT,
                bind_flag::RENDER_TARGET | bind_flag::SHADER_RESOURCE,
                0,
            );
            let depth_desc = mk_desc(
                64,
                64,
                D32_FLOAT,
                usage::DEFAULT,
                bind_flag::DEPTH_STENCIL | bind_flag::SHADER_RESOURCE,
                0,
            );
            let vel_desc = mk_desc(
                64,
                64,
                RG16_FLOAT,
                usage::DEFAULT,
                bind_flag::RENDER_TARGET | bind_flag::SHADER_RESOURCE,
                0,
            );
            let reactive_desc = mk_desc(
                64,
                64,
                R8_UNORM,
                usage::DEFAULT,
                bind_flag::RENDER_TARGET | bind_flag::SHADER_RESOURCE,
                0,
            );
            let out_desc = mk_desc(
                128,
                128,
                RGBA16_FLOAT,
                usage::DEFAULT,
                bind_flag::UNORDERED_ACCESS | bind_flag::SHADER_RESOURCE,
                0,
            );
            let stg_desc = mk_desc(128, 128, RGBA16_FLOAT, usage::STAGING, 0, cpu_access::READ);

            let mut color_tex: *mut c_void = std::ptr::null_mut();
            let mut depth_tex: *mut c_void = std::ptr::null_mut();
            let mut vel_tex: *mut c_void = std::ptr::null_mut();
            let mut reactive_tex: *mut c_void = std::ptr::null_mut();
            let mut out_tex: *mut c_void = std::ptr::null_mut();
            let mut stg_tex: *mut c_void = std::ptr::null_mut();

            assert_eq!(
                create_tex2d(dev, &color_desc, std::ptr::null(), &mut color_tex),
                S_OK
            );
            assert_eq!(
                create_tex2d(dev, &depth_desc, std::ptr::null(), &mut depth_tex),
                S_OK
            );
            assert_eq!(
                create_tex2d(dev, &vel_desc, std::ptr::null(), &mut vel_tex),
                S_OK
            );
            assert_eq!(
                create_tex2d(dev, &reactive_desc, std::ptr::null(), &mut reactive_tex),
                S_OK
            );
            assert_eq!(
                create_tex2d(dev, &out_desc, std::ptr::null(), &mut out_tex),
                S_OK
            );
            assert_eq!(
                create_tex2d(dev, &stg_desc, std::ptr::null(), &mut stg_tex),
                S_OK
            );

            let mut color_rtv: *mut c_void = std::ptr::null_mut();
            let mut vel_rtv: *mut c_void = std::ptr::null_mut();
            let mut reactive_rtv: *mut c_void = std::ptr::null_mut();
            let mut depth_dsv: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                create_rtv(dev, color_tex, std::ptr::null(), &mut color_rtv),
                S_OK
            );
            assert_eq!(
                create_rtv(dev, vel_tex, std::ptr::null(), &mut vel_rtv),
                S_OK
            );
            assert_eq!(
                create_rtv(dev, reactive_tex, std::ptr::null(), &mut reactive_rtv),
                S_OK
            );
            assert_eq!(
                create_dsv(dev, depth_tex, std::ptr::null(), &mut depth_dsv),
                S_OK
            );

            // Clear inputs on the D3D11 immediate context WITHOUT calling Flush() —
            // `xessD3D11Execute` must flush the immediate context first so MetalFX sees
            // the freshly rendered inputs on the GPU.
            let scene_color = [0.25f32, 0.75, 0.5, 1.0];
            let zero = [0.0f32, 0.0, 0.0, 0.0];
            clear_rtv(imm, color_rtv, &scene_color);
            clear_rtv(imm, vel_rtv, &zero);
            clear_rtv(imm, reactive_rtv, &zero);
            crate::d3d11::context::clear_depth_stencil_view(
                imm as crate::d3d11::context::PCtx,
                depth_dsv,
                clear_flag::DEPTH,
                0.5,
                0,
            );

            // Create + initialize XeSS D3D11 context.
            let mut xess_ctx = 0usize;
            assert_eq!(
                xessD3D11CreateContext(dev as usize, &mut xess_ctx),
                XESS_RESULT_SUCCESS
            );
            let init = XessD3D11InitParams {
                outputResolution: Xess2d { x: 128, y: 128 },
                qualitySetting: XESS_QUALITY_SETTING_BALANCED,
                initFlags: XESS_INIT_FLAG_INVERTED_DEPTH,
            };
            assert_eq!(xessD3D11Init(xess_ctx, &init), XESS_RESULT_SUCCESS);
            assert_eq!(xessSetJitterScale(xess_ctx, 1.0, 1.0), XESS_RESULT_SUCCESS);
            assert_eq!(xessSetVelocityScale(xess_ctx, 1.0, 1.0), XESS_RESULT_SUCCESS);

            // 1) Temporal MetalFX upscale pass (color + depth + velocity + reactive mask).
            let t0 = metalfx::temporal_encode_count();
            let exec_temporal = XessD3D11ExecuteParams {
                pColorTexture: color_tex as usize,
                pVelocityTexture: vel_tex as usize,
                pDepthTexture: depth_tex as usize,
                pExposureScaleTexture: 0,
                pResponsivePixelMaskTexture: reactive_tex as usize,
                pOutputTexture: out_tex as usize,
                jitterOffsetX: 0.0,
                jitterOffsetY: 0.0,
                exposureScale: 1.0,
                resetHistory: 1,
                inputWidth: 64,
                inputHeight: 64,
                ..Default::default()
            };
            assert_eq!(
                xessD3D11Execute(xess_ctx, &exec_temporal),
                XESS_RESULT_SUCCESS
            );
            if metalfx::is_temporal_enabled() {
                assert!(
                    metalfx::temporal_encode_count() > t0,
                    "expected MetalFX Temporal scaler to encode on GPU"
                );
            }

            // Copy upscaled output to staging and verify center pixel on CPU.
            copy_res(imm, stg_tex, out_tex);
            let mut mapped = D3D11_MAPPED_SUBRESOURCE {
                pData: std::ptr::null_mut(),
                RowPitch: 0,
                DepthPitch: 0,
            };
            assert_eq!(map_fn(imm, stg_tex, 0, map_type::READ, 0, &mut mapped), S_OK);
            assert!(!mapped.pData.is_null());
            let row_bytes = mapped.RowPitch as usize;
            let px_ptr = (mapped.pData as *const u8).add(64 * row_bytes + 64 * 8) as *const u16;
            let r = f16_to_f32(std::ptr::read_unaligned(px_ptr.add(0)));
            let g = f16_to_f32(std::ptr::read_unaligned(px_ptr.add(1)));
            let b = f16_to_f32(std::ptr::read_unaligned(px_ptr.add(2)));
            assert!((r - 0.25).abs() < 0.05, "temporal upscaled R={r}");
            assert!((g - 0.75).abs() < 0.05, "temporal upscaled G={g}");
            assert!((b - 0.50).abs() < 0.05, "temporal upscaled B={b}");
            unmap_fn(imm, stg_tex, 0);

            // 2) Spatial / bilinear fallback pass (null depth/velocity, e.g. HIGH_RES_MV without depth).
            let init_spatial = XessD3D11InitParams {
                outputResolution: Xess2d { x: 128, y: 128 },
                qualitySetting: XESS_QUALITY_SETTING_BALANCED,
                initFlags: XESS_INIT_FLAG_HIGH_RES_MV,
            };
            assert_eq!(xessD3D11Init(xess_ctx, &init_spatial), XESS_RESULT_SUCCESS);
            let scene_color2 = [0.8f32, 0.2, 0.6, 1.0];
            clear_rtv(imm, color_rtv, &scene_color2);
            let exec_spatial = XessD3D11ExecuteParams {
                pColorTexture: color_tex as usize,
                pVelocityTexture: 0,
                pDepthTexture: 0,
                pOutputTexture: out_tex as usize,
                inputWidth: 64,
                inputHeight: 64,
                ..Default::default()
            };
            assert_eq!(
                xessD3D11Execute(xess_ctx, &exec_spatial),
                XESS_RESULT_SUCCESS
            );
            copy_res(imm, stg_tex, out_tex);
            assert_eq!(map_fn(imm, stg_tex, 0, map_type::READ, 0, &mut mapped), S_OK);
            let px_ptr2 = (mapped.pData as *const u8).add(64 * row_bytes + 64 * 8) as *const u16;
            let r2 = f16_to_f32(std::ptr::read_unaligned(px_ptr2.add(0)));
            let g2 = f16_to_f32(std::ptr::read_unaligned(px_ptr2.add(1)));
            let b2 = f16_to_f32(std::ptr::read_unaligned(px_ptr2.add(2)));
            assert!((r2 - 0.80).abs() < 0.05, "spatial upscaled R={r2}");
            assert!((g2 - 0.20).abs() < 0.05, "spatial upscaled G={g2}");
            assert!((b2 - 0.60).abs() < 0.05, "spatial upscaled B={b2}");
            unmap_fn(imm, stg_tex, 0);

            // 3) Non-zero subrect base offsets: upscale a 32x32 input subrect at (16, 16)
            //    into a 64x64 output subrect at (32, 32) without disturbing (0, 0).
            let mut out_rtv: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                create_rtv(dev, out_tex, std::ptr::null(), &mut out_rtv),
                S_OK
            );
            let bg = [0.05f32, 0.05, 0.05, 1.0];
            clear_rtv(imm, out_rtv, &bg);
            let sub_color = [0.2f32, 0.9, 0.4, 1.0];
            clear_rtv(imm, color_rtv, &sub_color);
            let init_sub = XessD3D11InitParams {
                outputResolution: Xess2d { x: 64, y: 64 },
                qualitySetting: XESS_QUALITY_SETTING_BALANCED,
                initFlags: XESS_INIT_FLAG_NONE,
            };
            assert_eq!(xessD3D11Init(xess_ctx, &init_sub), XESS_RESULT_SUCCESS);
            let exec_sub = XessD3D11ExecuteParams {
                pColorTexture: color_tex as usize,
                pVelocityTexture: 0,
                pDepthTexture: 0,
                pOutputTexture: out_tex as usize,
                inputWidth: 32,
                inputHeight: 32,
                inputColorBase: Xess2d { x: 16, y: 16 },
                outputColorBase: Xess2d { x: 32, y: 32 },
                ..Default::default()
            };
            assert_eq!(
                xessD3D11Execute(xess_ctx, &exec_sub),
                XESS_RESULT_SUCCESS
            );
            copy_res(imm, stg_tex, out_tex);
            assert_eq!(map_fn(imm, stg_tex, 0, map_type::READ, 0, &mut mapped), S_OK);
            // Outside output subrect (4, 4) stays at background color (0.05).
            let bg_ptr = (mapped.pData as *const u8).add(4 * row_bytes + 4 * 8) as *const u16;
            let bg_r = f16_to_f32(std::ptr::read_unaligned(bg_ptr.add(0)));
            let bg_g = f16_to_f32(std::ptr::read_unaligned(bg_ptr.add(1)));
            assert!((bg_r - 0.05).abs() < 0.03 && (bg_g - 0.05).abs() < 0.03, "outside subrect modified: ({bg_r}, {bg_g})");
            // Inside output subrect (64, 64) receives upscaled sub_color (0.2, 0.9, 0.4).
            let in_ptr = (mapped.pData as *const u8).add(64 * row_bytes + 64 * 8) as *const u16;
            let in_r = f16_to_f32(std::ptr::read_unaligned(in_ptr.add(0)));
            let in_g = f16_to_f32(std::ptr::read_unaligned(in_ptr.add(1)));
            let in_b = f16_to_f32(std::ptr::read_unaligned(in_ptr.add(2)));
            assert!((in_r - 0.20).abs() < 0.08, "subrect upscaled R={in_r}");
            assert!((in_g - 0.90).abs() < 0.08, "subrect upscaled G={in_g}");
            assert!((in_b - 0.40).abs() < 0.08, "subrect upscaled B={in_b}");
            unmap_fn(imm, stg_tex, 0);
            com_release(out_rtv);

            assert_eq!(xessDestroyContext(xess_ctx), XESS_RESULT_SUCCESS);

            com_release(depth_dsv);
            com_release(reactive_rtv);
            com_release(vel_rtv);
            com_release(color_rtv);
            com_release(stg_tex);
            com_release(out_tex);
            com_release(reactive_tex);
            com_release(vel_tex);
            com_release(depth_tex);
            com_release(color_tex);
            com_release(imm);
            com_release(dev);
        }
    }
}
