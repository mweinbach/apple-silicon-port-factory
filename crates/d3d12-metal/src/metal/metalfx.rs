//! Native Apple MetalFX spatial and temporal upscaling on Metal 4
//! (`MTL4FXSpatialScaler` and `MTL4FXTemporalScaler`).
//!
//! Dynamically loads `/System/Library/Frameworks/MetalFX.framework/MetalFX` via
//! `dlopen` + Objective-C runtime dispatch so builds never require hard-linking
//! `MetalFX.framework` and gracefully fall back (temporal -> spatial -> compute
//! bilinear kernel) when MetalFX is unavailable or does not support a given
//! texture format or usage combination.
//!
//! Controlled by `PORT_METALFX` / `AW2_METALFX` (default ON when supported;
//! set to `0` to force the bilinear compute shader).

use super::queue::MtlCommandBuffer;
use super::resource::{MtlTexture, Resource};
use objc2::msg_send;
use objc2::rc::Retained;
use objc2::runtime::{AnyClass, AnyObject, Bool, ProtocolObject};
use objc2_metal::*;
use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

const METALFX_PATH: &CStr = c"/System/Library/Frameworks/MetalFX.framework/MetalFX";
const MAX_SCRATCH_CACHE: usize = 16;

/// Which backend actually executed a temporal/spatial upscale request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpscaleBackend {
    /// Native Metal 4 `MTL4FXTemporalScaler`.
    TemporalMetalFx,
    /// Native Metal 4 `MTL4FXSpatialScaler`.
    SpatialMetalFx,
    /// Built-in compute bilinear upscale kernel (`port_upscale_2d_f`).
    BilinearCompute,
}

/// `MTLFXSpatialScalerColorProcessingMode`.
#[repr(isize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ColorProcessingMode {
    /// Perceptual / sRGB LDR color space.
    Perceptual = 0,
    /// Linear `[0, 1]` color space.
    Linear = 1,
    /// High dynamic range (`[0, +inf)` with reversible tone mapping).
    Hdr = 2,
}

/// Cache key for `MTL4FXSpatialScaler` instances:
/// `(input_w, input_h, color_fmt, output_w, output_h, output_fmt)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScalerKey {
    pub input_w: u32,
    pub input_h: u32,
    pub color_fmt: usize,
    pub output_w: u32,
    pub output_h: u32,
    pub output_fmt: usize,
}

/// A cached `MTL4FXSpatialScaler` instance and its hardware texture-usage requirements.
pub struct SpatialScaler {
    obj: Retained<AnyObject>,
    pub color_usage: MTLTextureUsage,
    pub output_usage: MTLTextureUsage,
    pub mode: ColorProcessingMode,
    has_fence_setter: bool,
}

// SAFETY: `SpatialScaler` is only mutated while holding its enclosing `Mutex`,
// and `MTL4FXSpatialScaler` objects may be encoded from any thread when externally synchronized.
unsafe impl Send for SpatialScaler {}
unsafe impl Sync for SpatialScaler {}

impl SpatialScaler {
    /// Encode MetalFX spatial upscaling directly into an open `MTL4CommandBuffer`
    /// (caller must ensure no active command encoder is open on `cb` and that
    /// prior/subsequent GPU stages are synchronized with Metal 4 queue barriers).
    pub fn encode(
        &mut self,
        cb: &MtlCommandBuffer,
        color: &MtlTexture,
        output: &MtlTexture,
        content_w: u32,
        content_h: u32,
        fence: Option<&ProtocolObject<dyn MTLFence>>,
    ) {
        unsafe {
            let _: () = msg_send![&*self.obj, setColorTexture: color];
            let _: () = msg_send![&*self.obj, setOutputTexture: output];
            let _: () = msg_send![&*self.obj, setInputContentWidth: content_w as usize];
            let _: () = msg_send![&*self.obj, setInputContentHeight: content_h as usize];
            if self.has_fence_setter {
                let _: () = msg_send![&*self.obj, setFence: fence];
            }
            let _: () = msg_send![&*self.obj, encodeToCommandBuffer: cb];
            // Release texture/fence references immediately so cached scalers never pin
            // transient render targets across resizes or resource destruction.
            let _: () = msg_send![&*self.obj, setColorTexture: None::<&MtlTexture>];
            let _: () = msg_send![&*self.obj, setOutputTexture: None::<&MtlTexture>];
            if self.has_fence_setter {
                let _: () = msg_send![&*self.obj, setFence: None::<&ProtocolObject<dyn MTLFence>>];
            }
        }
        ENCODE_COUNT.fetch_add(1, Ordering::Relaxed);
    }
}

static ENCODE_COUNT: AtomicU64 = AtomicU64::new(0);
static TEMPORAL_ENCODE_COUNT: AtomicU64 = AtomicU64::new(0);

/// Total number of MetalFX spatial scaler encodes performed in this process.
pub fn encode_count() -> u64 {
    ENCODE_COUNT.load(Ordering::Relaxed)
}

/// Total number of MetalFX temporal scaler encodes performed in this process.
pub fn temporal_encode_count() -> u64 {
    TEMPORAL_ENCODE_COUNT.load(Ordering::Relaxed)
}

/// Cache key / creation descriptor for `MTL4FXTemporalScaler` instances:
/// keyed by `(input_w, input_h, output_w, output_h, color_fmt, depth_fmt,
/// motion_fmt, output_fmt)` plus descriptor-time options (`reactive_fmt`,
/// `auto_exposure`, `dynamic_resolution`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TemporalScalerKey {
    pub input_w: u32,
    pub input_h: u32,
    pub output_w: u32,
    pub output_h: u32,
    pub color_fmt: usize,
    pub depth_fmt: usize,
    pub motion_fmt: usize,
    pub output_fmt: usize,
    pub reactive_fmt: Option<usize>,
    pub auto_exposure: bool,
    pub dynamic_resolution: bool,
    pub output_resolution_motion_vectors: bool,
    pub jittered_motion_vectors: bool,
}

impl TemporalScalerKey {
    pub fn new(
        input_w: u32,
        input_h: u32,
        output_w: u32,
        output_h: u32,
        color_fmt: MTLPixelFormat,
        depth_fmt: MTLPixelFormat,
        motion_fmt: MTLPixelFormat,
        output_fmt: MTLPixelFormat,
    ) -> Self {
        Self {
            input_w,
            input_h,
            output_w,
            output_h,
            color_fmt: color_fmt.0,
            depth_fmt: depth_fmt.0,
            motion_fmt: motion_fmt.0,
            output_fmt: output_fmt.0,
            reactive_fmt: None,
            auto_exposure: false,
            dynamic_resolution: false,
            output_resolution_motion_vectors: false,
            jittered_motion_vectors: false,
        }
    }

    pub fn with_reactive_format(mut self, fmt: Option<MTLPixelFormat>) -> Self {
        self.reactive_fmt = fmt.map(|f| f.0);
        self
    }

    pub fn with_auto_exposure(mut self, enabled: bool) -> Self {
        self.auto_exposure = enabled;
        self
    }

    pub fn with_dynamic_resolution(mut self, enabled: bool) -> Self {
        self.dynamic_resolution = enabled;
        self
    }

    pub fn with_output_resolution_motion_vectors(mut self, enabled: bool) -> Self {
        self.output_resolution_motion_vectors = enabled;
        self
    }

    pub fn with_jittered_motion_vectors(mut self, enabled: bool) -> Self {
        self.jittered_motion_vectors = enabled;
        self
    }
}

/// Low-level Metal texture inputs for a single `TemporalScaler::encode` pass.
pub struct TemporalEncodeInputs<'a> {
    pub color: &'a MtlTexture,
    pub depth: &'a MtlTexture,
    pub motion: &'a MtlTexture,
    pub output: &'a MtlTexture,
    pub exposure: Option<&'a MtlTexture>,
    pub reactive_mask: Option<&'a MtlTexture>,
    pub content_w: u32,
    pub content_h: u32,
    pub jitter_offset: [f32; 2],
    pub motion_vector_scale: [f32; 2],
    pub reset: bool,
    pub depth_reversed: bool,
    pub pre_exposure: f32,
    pub fence: Option<&'a ProtocolObject<dyn MTLFence>>,
}

/// High-level D3D12 `Resource` parameters for `CommandList::upscale_temporal`.
pub struct TemporalUpscaleParams<'a> {
    pub output: &'a Resource,
    pub color: &'a Resource,
    pub depth: &'a Resource,
    pub motion: &'a Resource,
    pub exposure: Option<&'a Resource>,
    pub reactive_mask: Option<&'a Resource>,
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
    pub fence: Option<&'a ProtocolObject<dyn MTLFence>>,
}

impl<'a> TemporalUpscaleParams<'a> {
    pub fn new(
        output: &'a Resource,
        color: &'a Resource,
        depth: &'a Resource,
        motion: &'a Resource,
        render_size: [u32; 2],
        upscale_size: [u32; 2],
    ) -> Self {
        Self {
            output,
            color,
            depth,
            motion,
            exposure: None,
            reactive_mask: None,
            render_size,
            upscale_size,
            jitter_offset: [0.0, 0.0],
            motion_vector_scale: [1.0, 1.0],
            reset: false,
            depth_reversed: false,
            pre_exposure: 1.0,
            auto_exposure: false,
            dynamic_resolution: false,
            output_resolution_motion_vectors: false,
            jittered_motion_vectors: false,
            fence: None,
        }
    }
}

/// A cached `MTL4FXTemporalScaler` instance and its hardware texture-usage requirements.
pub struct TemporalScaler {
    obj: Retained<AnyObject>,
    pub input_w: u32,
    pub input_h: u32,
    pub output_w: u32,
    pub output_h: u32,
    pub color_usage: MTLTextureUsage,
    pub depth_usage: MTLTextureUsage,
    pub motion_usage: MTLTextureUsage,
    pub output_usage: MTLTextureUsage,
    pub reactive_usage: Option<MTLTextureUsage>,
    pub auto_exposure: bool,
    pub dynamic_resolution: bool,
    pub min_scale: f32,
    pub max_scale: f32,
    has_exposure_setter: bool,
    has_reactive_setter: bool,
    has_pre_exposure_setter: bool,
    has_fence_setter: bool,
}

// SAFETY: `TemporalScaler` is only mutated while holding its enclosing `Mutex`,
// and `MTL4FXTemporalScaler` objects may be encoded from any thread when externally synchronized.
unsafe impl Send for TemporalScaler {}
unsafe impl Sync for TemporalScaler {}

impl TemporalScaler {
    /// Encode MetalFX temporal upscaling directly into an open `MTL4CommandBuffer`
    /// (caller must ensure no active command encoder is open on `cb` and that
    /// prior/subsequent GPU stages are synchronized with Metal 4 queue barriers).
    pub fn encode(&mut self, cb: &MtlCommandBuffer, inputs: &TemporalEncodeInputs<'_>) {
        let pre_exp = if inputs.pre_exposure.is_finite() && inputs.pre_exposure > 0.0 {
            inputs.pre_exposure
        } else {
            1.0
        };
        unsafe {
            let _: () = msg_send![&*self.obj, setColorTexture: inputs.color];
            let _: () = msg_send![&*self.obj, setDepthTexture: inputs.depth];
            let _: () = msg_send![&*self.obj, setMotionTexture: inputs.motion];
            let _: () = msg_send![&*self.obj, setOutputTexture: inputs.output];
            if self.has_exposure_setter {
                let exp = if self.auto_exposure { None } else { inputs.exposure };
                let _: () = msg_send![&*self.obj, setExposureTexture: exp];
            }
            if self.has_reactive_setter && self.reactive_usage.is_some() {
                let _: () = msg_send![&*self.obj, setReactiveMaskTexture: inputs.reactive_mask];
            }
            let _: () = msg_send![&*self.obj, setInputContentWidth: inputs.content_w as usize];
            let _: () = msg_send![&*self.obj, setInputContentHeight: inputs.content_h as usize];
            let _: () = msg_send![&*self.obj, setJitterOffsetX: inputs.jitter_offset[0]];
            let _: () = msg_send![&*self.obj, setJitterOffsetY: inputs.jitter_offset[1]];
            let _: () = msg_send![&*self.obj, setMotionVectorScaleX: inputs.motion_vector_scale[0]];
            let _: () = msg_send![&*self.obj, setMotionVectorScaleY: inputs.motion_vector_scale[1]];
            let _: () = msg_send![&*self.obj, setReset: Bool::new(inputs.reset)];
            let _: () = msg_send![&*self.obj, setDepthReversed: Bool::new(inputs.depth_reversed)];
            if self.has_pre_exposure_setter {
                let _: () = msg_send![&*self.obj, setPreExposure: pre_exp];
            }
            if self.has_fence_setter {
                let _: () = msg_send![&*self.obj, setFence: inputs.fence];
            }
            let _: () = msg_send![&*self.obj, encodeToCommandBuffer: cb];
            // Release texture/fence references immediately so cached scalers never pin
            // transient render targets across resizes or resource destruction.
            let _: () = msg_send![&*self.obj, setColorTexture: None::<&MtlTexture>];
            let _: () = msg_send![&*self.obj, setDepthTexture: None::<&MtlTexture>];
            let _: () = msg_send![&*self.obj, setMotionTexture: None::<&MtlTexture>];
            let _: () = msg_send![&*self.obj, setOutputTexture: None::<&MtlTexture>];
            if self.has_exposure_setter {
                let _: () = msg_send![&*self.obj, setExposureTexture: None::<&MtlTexture>];
            }
            if self.has_reactive_setter && self.reactive_usage.is_some() {
                let _: () = msg_send![&*self.obj, setReactiveMaskTexture: None::<&MtlTexture>];
            }
            if self.has_fence_setter {
                let _: () = msg_send![&*self.obj, setFence: None::<&ProtocolObject<dyn MTLFence>>];
            }
        }
        TEMPORAL_ENCODE_COUNT.fetch_add(1, Ordering::Relaxed);
    }
}

/// Returns the `ColorProcessingMode` for a color-renderable floating-point or
/// normalized format supported by `MTL4FXSpatialScaler`, or `None` for integer,
/// depth/stencil, compressed, or non-renderable formats.
///
/// Floating-point HDR formats (`RGBA16Float`, `RG11B10Float`, `RGBA32Float`)
/// deliberately use [`ColorProcessingMode::Linear`] rather than
/// [`ColorProcessingMode::Hdr`]: Apple's Metal 4 spatial scaler `Hdr` mode
/// applies a reversible Reinhard-like tone compression (`y = x / (1 + x)`) at
/// reduced internal precision near `y = 1.0`, which causes severe flat contour
/// plateaus, edge ringing, and a `~484` highlight clamp on pre-exposed linear
/// HDR scene buffers (`10..5000+`, as produced by Northlight/UE5 before
/// post-upscale exposure division). `Linear` mode is scale-invariant across
/// the full floating-point HDR range without clamping or dark/sky banding.
pub fn color_processing_mode(fmt: MTLPixelFormat) -> Option<ColorProcessingMode> {
    match fmt {
        MTLPixelFormat::RGBA8Unorm
        | MTLPixelFormat::RGBA8Unorm_sRGB
        | MTLPixelFormat::BGRA8Unorm
        | MTLPixelFormat::BGRA8Unorm_sRGB
        | MTLPixelFormat::RGB10A2Unorm
        | MTLPixelFormat::BGR10A2Unorm
        | MTLPixelFormat::RGBA16Unorm => Some(ColorProcessingMode::Perceptual),
        MTLPixelFormat::RGBA16Float
        | MTLPixelFormat::RG11B10Float
        | MTLPixelFormat::RGBA32Float => Some(ColorProcessingMode::Linear),
        _ => None,
    }
}

/// True when `fmt` is a valid depth texture pixel format for `MTL4FXTemporalScaler`.
pub fn temporal_depth_format_supported(fmt: MTLPixelFormat) -> bool {
    matches!(
        fmt,
        MTLPixelFormat::Depth32Float
            | MTLPixelFormat::Depth32Float_Stencil8
            | MTLPixelFormat::Depth16Unorm
            | MTLPixelFormat::R32Float
            | MTLPixelFormat::R16Float
            | MTLPixelFormat::R16Unorm
    )
}

/// True when `fmt` is a valid motion-vector texture pixel format for `MTL4FXTemporalScaler`.
pub fn temporal_motion_format_supported(fmt: MTLPixelFormat) -> bool {
    matches!(
        fmt,
        MTLPixelFormat::RG16Float
            | MTLPixelFormat::RG32Float
            | MTLPixelFormat::RGBA16Float
            | MTLPixelFormat::RGBA32Float
    )
}

/// True when `fmt` is a valid reactive-mask texture pixel format for `MTL4FXTemporalScaler`.
pub fn temporal_reactive_format_supported(fmt: MTLPixelFormat) -> bool {
    matches!(
        fmt,
        MTLPixelFormat::R8Unorm
            | MTLPixelFormat::R16Unorm
            | MTLPixelFormat::R16Float
            | MTLPixelFormat::R32Float
            | MTLPixelFormat::RGBA8Unorm
            | MTLPixelFormat::RGBA16Float
    )
}

/// Choose the MetalFX color processing mode for an `(input, output)` format pair.
/// Both formats must be color-renderable floating-point or normalized formats.
pub fn select_color_processing_mode(
    color_fmt: MTLPixelFormat,
    output_fmt: MTLPixelFormat,
) -> Option<ColorProcessingMode> {
    let in_mode = color_processing_mode(color_fmt)?;
    let out_mode = color_processing_mode(output_fmt)?;
    Some(match (in_mode, out_mode) {
        (ColorProcessingMode::Hdr, _) | (_, ColorProcessingMode::Hdr) => ColorProcessingMode::Hdr,
        (ColorProcessingMode::Linear, _) | (_, ColorProcessingMode::Linear) => ColorProcessingMode::Linear,
        _ => ColorProcessingMode::Perceptual,
    })
}

struct SendTex(Retained<MtlTexture>);
unsafe impl Send for SendTex {}
unsafe impl Sync for SendTex {}

struct MetalFxRuntime {
    spatial_desc_cls: Option<&'static AnyClass>,
    temporal_desc_cls: Option<&'static AnyClass>,
    temporal_min_scale: f32,
    temporal_max_scale: f32,
    compiler: Retained<ProtocolObject<dyn MTL4Compiler>>,
    scalers: Mutex<HashMap<ScalerKey, Option<Arc<Mutex<SpatialScaler>>>>>,
    temporal_scalers: Mutex<HashMap<TemporalScalerKey, Option<Arc<Mutex<TemporalScaler>>>>>,
    scratch: Mutex<HashMap<(u32, u32, usize, usize), SendTex>>,
}

unsafe impl Send for MetalFxRuntime {}
unsafe impl Sync for MetalFxRuntime {}

fn probe_m4_descriptor_class(
    cls_name: &CStr,
    dev: &ProtocolObject<dyn MTLDevice>,
    kind: &str,
    dev_name: &str,
) -> Option<&'static AnyClass> {
    let Some(cls) = AnyClass::get(cls_name) else {
        log::warn!(target: "d3d", "metalfx: {} class missing", cls_name.to_string_lossy());
        return None;
    };
    let has_m4_query: Bool = unsafe {
        msg_send![cls, respondsToSelector: objc2::sel!(supportsMetal4FX:)]
    };
    if !has_m4_query.as_bool() {
        log::info!(target: "d3d", "metalfx: {kind} supportsMetal4FX: unavailable on this OS");
        return None;
    }
    let supported: Bool = unsafe { msg_send![cls, supportsMetal4FX: dev] };
    if !supported.as_bool() {
        log::info!(target: "d3d", "metalfx: device {dev_name} does not support Metal 4 FX {kind} scaling");
        return None;
    }
    Some(cls)
}

fn runtime() -> Option<&'static MetalFxRuntime> {
    static RT: OnceLock<Option<MetalFxRuntime>> = OnceLock::new();
    RT.get_or_init(|| {
        let d = super::device()?;
        let handle = unsafe { libc::dlopen(METALFX_PATH.as_ptr(), libc::RTLD_LAZY | libc::RTLD_LOCAL) };
        if handle.is_null() {
            log::info!(target: "d3d", "metalfx: MetalFX.framework unavailable; using compute upscale fallback");
            return None;
        }
        let spatial_desc_cls =
            probe_m4_descriptor_class(c"MTLFXSpatialScalerDescriptor", &*d.dev, "spatial", &d.name);
        let temporal_desc_cls =
            probe_m4_descriptor_class(c"MTLFXTemporalScalerDescriptor", &*d.dev, "temporal", &d.name);
        if spatial_desc_cls.is_none() && temporal_desc_cls.is_none() {
            return None;
        }
        let cdesc = MTL4CompilerDescriptor::new();
        let compiler = match d.dev.newCompilerWithDescriptor_error(&cdesc) {
            Ok(c) => c,
            Err(e) => {
                log::warn!(target: "d3d", "metalfx: newCompilerWithDescriptor failed: {e}");
                return None;
            }
        };
        let (temporal_min_scale, temporal_max_scale) = if let Some(tcls) = temporal_desc_cls {
            let has_min: Bool = unsafe {
                msg_send![tcls, respondsToSelector: objc2::sel!(supportedInputContentMinScaleForDevice:)]
            };
            let has_max: Bool = unsafe {
                msg_send![tcls, respondsToSelector: objc2::sel!(supportedInputContentMaxScaleForDevice:)]
            };
            let min_s: f32 = if has_min.as_bool() {
                unsafe { msg_send![tcls, supportedInputContentMinScaleForDevice: &*d.dev] }
            } else {
                1.0
            };
            let max_s: f32 = if has_max.as_bool() {
                unsafe { msg_send![tcls, supportedInputContentMaxScaleForDevice: &*d.dev] }
            } else {
                3.0
            };
            (min_s, max_s)
        } else {
            (1.0, 1.0)
        };
        log::info!(
            target: "d3d",
            "metalfx: Metal 4 FX ready on {} (spatial={}, temporal={}, scale=[{:.2}, {:.2}])",
            d.name,
            spatial_desc_cls.is_some(),
            temporal_desc_cls.is_some(),
            temporal_min_scale,
            temporal_max_scale,
        );
        Some(MetalFxRuntime {
            spatial_desc_cls,
            temporal_desc_cls,
            temporal_min_scale,
            temporal_max_scale,
            compiler,
            scalers: Mutex::new(HashMap::new()),
            temporal_scalers: Mutex::new(HashMap::new()),
            scratch: Mutex::new(HashMap::new()),
        })
    })
    .as_ref()
}

fn env_enabled() -> bool {
    static ENV_ON: OnceLock<bool> = OnceLock::new();
    *ENV_ON.get_or_init(|| {
        std::env::var_os("PORT_METALFX")
            .or_else(|| std::env::var_os("AW2_METALFX"))
            .is_none_or(|v| v != "0")
    })
}

fn temporal_env_enabled() -> bool {
    static TEMPORAL_ON: OnceLock<bool> = OnceLock::new();
    *TEMPORAL_ON.get_or_init(|| {
        std::env::var_os("PORT_METALFX_TEMPORAL")
            .or_else(|| std::env::var_os("AW2_METALFX_TEMPORAL"))
            .is_none_or(|v| v != "0")
    })
}

/// True when `MetalFX.framework` loaded and this GPU supports Metal 4 spatial scaling.
pub fn is_available() -> bool {
    runtime().is_some_and(|rt| rt.spatial_desc_cls.is_some())
}

/// True when `MetalFX.framework` loaded and this GPU supports Metal 4 temporal scaling.
pub fn is_temporal_available() -> bool {
    runtime().is_some_and(|rt| rt.temporal_desc_cls.is_some())
}

/// Supported `(min_scale, max_scale)` dynamic-resolution bounds for `MTL4FXTemporalScaler`.
pub fn temporal_scale_limits() -> Option<(f32, f32)> {
    let rt = runtime()?;
    rt.temporal_desc_cls?;
    Some((rt.temporal_min_scale, rt.temporal_max_scale))
}

/// True when MetalFX spatial upscaling is both enabled (`PORT_METALFX` / `AW2_METALFX` != `0`)
/// and supported by the active Metal 4 device.
pub fn is_enabled() -> bool {
    env_enabled() && is_available()
}

/// True when MetalFX temporal upscaling is both enabled (`PORT_METALFX` != `0` and
/// `PORT_METALFX_TEMPORAL` != `0`) and supported by the active Metal 4 device.
pub fn is_temporal_enabled() -> bool {
    env_enabled() && temporal_env_enabled() && is_temporal_available()
}

/// Fetch or create a cached `MTL4FXSpatialScaler` for `(input_w, input_h, color_fmt)`
/// -> `(output_w, output_h, output_fmt)`. Unsupported combinations are negative-cached
/// so repeated calls fall back to the compute shader without re-probing MetalFX.
pub fn get_or_create_scaler(
    input_w: u32,
    input_h: u32,
    color_fmt: MTLPixelFormat,
    output_w: u32,
    output_h: u32,
    output_fmt: MTLPixelFormat,
) -> Option<Arc<Mutex<SpatialScaler>>> {
    if input_w == 0 || input_h == 0 || output_w == 0 || output_h == 0 {
        return None;
    }
    let mode = select_color_processing_mode(color_fmt, output_fmt)?;
    let rt = runtime()?;
    let desc_cls = rt.spatial_desc_cls?;
    let key = ScalerKey {
        input_w,
        input_h,
        color_fmt: color_fmt.0,
        output_w,
        output_h,
        output_fmt: output_fmt.0,
    };
    if let Some(cached) = rt.scalers.lock().unwrap().get(&key) {
        return cached.clone();
    }

    let d = super::device()?;
    let created: Option<Arc<Mutex<SpatialScaler>>> = objc2::rc::autoreleasepool(|_| {
        let desc: Retained<AnyObject> = unsafe { msg_send![desc_cls, new] };
        let has_m4_ctor: Bool = unsafe {
            msg_send![&*desc, respondsToSelector: objc2::sel!(newSpatialScalerWithDevice:compiler:)]
        };
        if !has_m4_ctor.as_bool() {
            return None;
        }
        unsafe {
            let _: () = msg_send![&*desc, setInputWidth: input_w as usize];
            let _: () = msg_send![&*desc, setInputHeight: input_h as usize];
            let _: () = msg_send![&*desc, setOutputWidth: output_w as usize];
            let _: () = msg_send![&*desc, setOutputHeight: output_h as usize];
            let _: () = msg_send![&*desc, setColorTextureFormat: color_fmt];
            let _: () = msg_send![&*desc, setOutputTextureFormat: output_fmt];
            let _: () = msg_send![&*desc, setColorProcessingMode: mode as isize];
        }
        let obj: Option<Retained<AnyObject>> = unsafe {
            msg_send![&*desc, newSpatialScalerWithDevice: &*d.dev, compiler: &*rt.compiler]
        };
        let obj = obj?;
        let color_usage: MTLTextureUsage = unsafe { msg_send![&*obj, colorTextureUsage] };
        let output_usage: MTLTextureUsage = unsafe { msg_send![&*obj, outputTextureUsage] };
        let has_fence_setter: Bool = unsafe {
            msg_send![&*obj, respondsToSelector: objc2::sel!(setFence:)]
        };
        log::debug!(
            target: "d3d",
            "metalfx: created spatial scaler {}x{} ({:?}) -> {}x{} ({:?}) mode={:?} in_usage={:?} out_usage={:?}",
            input_w,
            input_h,
            color_fmt,
            output_w,
            output_h,
            output_fmt,
            mode,
            color_usage,
            output_usage
        );
        Some(Arc::new(Mutex::new(SpatialScaler {
            obj,
            color_usage,
            output_usage,
            mode,
            has_fence_setter: has_fence_setter.as_bool(),
        })))
    });

    let mut cache = rt.scalers.lock().unwrap();
    cache.entry(key).or_insert_with(|| created).clone()
}

/// Fetch or create a cached `MTL4FXTemporalScaler` for `key`.
/// Unsupported format/dimension combinations are logged clearly and negative-cached
/// so callers fall back cleanly to spatial/bilinear upscaling without re-probing.
pub fn get_or_create_temporal_scaler(key: TemporalScalerKey) -> Option<Arc<Mutex<TemporalScaler>>> {
    if key.input_w == 0 || key.input_h == 0 || key.output_w == 0 || key.output_h == 0 {
        return None;
    }
    let color_fmt = MTLPixelFormat(key.color_fmt);
    let depth_fmt = MTLPixelFormat(key.depth_fmt);
    let motion_fmt = MTLPixelFormat(key.motion_fmt);
    let output_fmt = MTLPixelFormat(key.output_fmt);
    let reactive_fmt = key.reactive_fmt.map(MTLPixelFormat);

    if color_processing_mode(color_fmt).is_none()
        || color_processing_mode(output_fmt).is_none()
        || !temporal_depth_format_supported(depth_fmt)
        || !temporal_motion_format_supported(motion_fmt)
        || reactive_fmt.is_some_and(|rf| !temporal_reactive_format_supported(rf))
    {
        return None;
    }

    let rt = runtime()?;
    let desc_cls = rt.temporal_desc_cls?;
    if let Some(cached) = rt.temporal_scalers.lock().unwrap().get(&key) {
        return cached.clone();
    }

    let d = super::device()?;
    let created: Option<Arc<Mutex<TemporalScaler>>> = objc2::rc::autoreleasepool(|_| {
        let desc: Retained<AnyObject> = unsafe { msg_send![desc_cls, new] };
        let has_m4_ctor: Bool = unsafe {
            msg_send![&*desc, respondsToSelector: objc2::sel!(newTemporalScalerWithDevice:compiler:)]
        };
        if !has_m4_ctor.as_bool() {
            log::warn!(target: "d3d", "metalfx: MTLFXTemporalScalerDescriptor lacks newTemporalScalerWithDevice:compiler:");
            return None;
        }
        unsafe {
            let _: () = msg_send![&*desc, setInputWidth: key.input_w as usize];
            let _: () = msg_send![&*desc, setInputHeight: key.input_h as usize];
            let _: () = msg_send![&*desc, setOutputWidth: key.output_w as usize];
            let _: () = msg_send![&*desc, setOutputHeight: key.output_h as usize];
            let _: () = msg_send![&*desc, setColorTextureFormat: color_fmt];
            let _: () = msg_send![&*desc, setDepthTextureFormat: depth_fmt];
            let _: () = msg_send![&*desc, setMotionTextureFormat: motion_fmt];
            let _: () = msg_send![&*desc, setOutputTextureFormat: output_fmt];
            let _: () = msg_send![&*desc, setAutoExposureEnabled: Bool::new(key.auto_exposure)];
            if key.dynamic_resolution {
                let has_drs: Bool = msg_send![&*desc, respondsToSelector: objc2::sel!(setInputContentPropertiesEnabled:)];
                if has_drs.as_bool() {
                    let _: () = msg_send![&*desc, setInputContentPropertiesEnabled: Bool::YES];
                    let _: () = msg_send![&*desc, setInputContentMinScale: rt.temporal_min_scale];
                    let _: () = msg_send![&*desc, setInputContentMaxScale: rt.temporal_max_scale];
                } else {
                    log::warn!(target: "d3d", "metalfx: dynamic resolution unsupported on MTLFXTemporalScalerDescriptor");
                    return None;
                }
            }
            if key.output_resolution_motion_vectors {
                let has_out_mv: Bool = msg_send![&*desc, respondsToSelector: objc2::sel!(setOutputResolutionMotionVectorsEnabled:)];
                if has_out_mv.as_bool() {
                    let _: () = msg_send![&*desc, setOutputResolutionMotionVectorsEnabled: Bool::YES];
                } else {
                    log::warn!(target: "d3d", "metalfx: output-resolution motion vectors unsupported on MTLFXTemporalScalerDescriptor");
                    return None;
                }
            }
            if key.jittered_motion_vectors {
                let has_jit_mv: Bool = msg_send![&*desc, respondsToSelector: objc2::sel!(setJitteredMotionVectorsEnabled:)];
                if has_jit_mv.as_bool() {
                    let _: () = msg_send![&*desc, setJitteredMotionVectorsEnabled: Bool::YES];
                } else {
                    log::warn!(target: "d3d", "metalfx: jittered motion vectors unsupported on MTLFXTemporalScalerDescriptor");
                    return None;
                }
            }
            if let Some(rf) = reactive_fmt {
                let has_re: Bool = msg_send![&*desc, respondsToSelector: objc2::sel!(setReactiveMaskTextureEnabled:)];
                let has_rf: Bool = msg_send![&*desc, respondsToSelector: objc2::sel!(setReactiveMaskTextureFormat:)];
                if has_re.as_bool() && has_rf.as_bool() {
                    let _: () = msg_send![&*desc, setReactiveMaskTextureEnabled: Bool::YES];
                    let _: () = msg_send![&*desc, setReactiveMaskTextureFormat: rf];
                } else {
                    log::warn!(target: "d3d", "metalfx: reactive mask unsupported on MTLFXTemporalScalerDescriptor");
                    return None;
                }
            }
        }
        let obj: Option<Retained<AnyObject>> = unsafe {
            msg_send![&*desc, newTemporalScalerWithDevice: &*d.dev, compiler: &*rt.compiler]
        };
        let Some(obj) = obj else {
            log::warn!(
                target: "d3d",
                "metalfx: MTL4FXTemporalScaler unsupported for {}x{} (color={:?}, depth={:?}, motion={:?}, reactive={:?}) -> {}x{} ({:?}); falling back",
                key.input_w,
                key.input_h,
                color_fmt,
                depth_fmt,
                motion_fmt,
                reactive_fmt,
                key.output_w,
                key.output_h,
                output_fmt,
            );
            return None;
        };
        let color_usage: MTLTextureUsage = unsafe { msg_send![&*obj, colorTextureUsage] };
        let depth_usage: MTLTextureUsage = unsafe { msg_send![&*obj, depthTextureUsage] };
        let motion_usage: MTLTextureUsage = unsafe { msg_send![&*obj, motionTextureUsage] };
        let output_usage: MTLTextureUsage = unsafe { msg_send![&*obj, outputTextureUsage] };
        let reactive_usage = if reactive_fmt.is_some() {
            let has_new: Bool = unsafe {
                msg_send![&*obj, respondsToSelector: objc2::sel!(reactiveMaskTextureUsage)]
            };
            let has_old: Bool = unsafe {
                msg_send![&*obj, respondsToSelector: objc2::sel!(reactiveTextureUsage)]
            };
            let u: MTLTextureUsage = if has_new.as_bool() {
                unsafe { msg_send![&*obj, reactiveMaskTextureUsage] }
            } else if has_old.as_bool() {
                unsafe { msg_send![&*obj, reactiveTextureUsage] }
            } else {
                MTLTextureUsage::ShaderRead
            };
            Some(u)
        } else {
            None
        };
        let has_min: Bool = unsafe { msg_send![&*obj, respondsToSelector: objc2::sel!(inputContentMinScale)] };
        let has_max: Bool = unsafe { msg_send![&*obj, respondsToSelector: objc2::sel!(inputContentMaxScale)] };
        let min_scale: f32 = if has_min.as_bool() {
            unsafe { msg_send![&*obj, inputContentMinScale] }
        } else {
            rt.temporal_min_scale
        };
        let max_scale: f32 = if has_max.as_bool() {
            unsafe { msg_send![&*obj, inputContentMaxScale] }
        } else {
            rt.temporal_max_scale
        };
        let has_exposure_setter: Bool = unsafe {
            msg_send![&*obj, respondsToSelector: objc2::sel!(setExposureTexture:)]
        };
        let has_reactive_setter: Bool = unsafe {
            msg_send![&*obj, respondsToSelector: objc2::sel!(setReactiveMaskTexture:)]
        };
        let has_pre_exposure_setter: Bool = unsafe {
            msg_send![&*obj, respondsToSelector: objc2::sel!(setPreExposure:)]
        };
        let has_fence_setter: Bool = unsafe {
            msg_send![&*obj, respondsToSelector: objc2::sel!(setFence:)]
        };
        log::debug!(
            target: "d3d",
            "metalfx: created temporal scaler {}x{} (color={:?}, depth={:?}, motion={:?}, reactive={:?}) -> {}x{} ({:?}) usages=(c={:?}, d={:?}, m={:?}, o={:?}, r={:?}) drs={} scale=[{:.2},{:.2}]",
            key.input_w,
            key.input_h,
            color_fmt,
            depth_fmt,
            motion_fmt,
            reactive_fmt,
            key.output_w,
            key.output_h,
            output_fmt,
            color_usage,
            depth_usage,
            motion_usage,
            output_usage,
            reactive_usage,
            key.dynamic_resolution,
            min_scale,
            max_scale,
        );
        Some(Arc::new(Mutex::new(TemporalScaler {
            obj,
            input_w: key.input_w,
            input_h: key.input_h,
            output_w: key.output_w,
            output_h: key.output_h,
            color_usage,
            depth_usage,
            motion_usage,
            output_usage,
            reactive_usage,
            auto_exposure: key.auto_exposure,
            dynamic_resolution: key.dynamic_resolution,
            min_scale,
            max_scale,
            has_exposure_setter: has_exposure_setter.as_bool(),
            has_reactive_setter: has_reactive_setter.as_bool(),
            has_pre_exposure_setter: has_pre_exposure_setter.as_bool(),
            has_fence_setter: has_fence_setter.as_bool(),
        })))
    });

    let mut cache = rt.temporal_scalers.lock().unwrap();
    cache.entry(key).or_insert_with(|| created).clone()
}

/// Cached private `RenderTarget` output texture for UAV-only D3D12 upscale targets
/// that lack `MTLTextureUsage::RenderTarget` (e.g. FSR2 / DLSS / XeSS compute UAV outputs).
pub fn scratch_output_texture(w: u32, h: u32, fmt: MTLPixelFormat) -> Option<Retained<MtlTexture>> {
    scratch_output_texture_with_usage(w, h, fmt, MTLTextureUsage::RenderTarget)
}

/// Cached private output texture satisfying `required_usage | ShaderRead` for targets
/// that lack `RenderTarget`, `ShaderWrite`, or `Private` storage (e.g. UAV-only or
/// RTV-only targets used with `MTL4FXSpatialScaler` / `MTL4FXTemporalScaler`).
pub fn scratch_output_texture_with_usage(
    w: u32,
    h: u32,
    fmt: MTLPixelFormat,
    required_usage: MTLTextureUsage,
) -> Option<Retained<MtlTexture>> {
    if w == 0 || h == 0 {
        return None;
    }
    let rt = runtime()?;
    let usage = MTLTextureUsage::ShaderRead | required_usage;
    let key = (w, h, fmt.0, usage.0);
    {
        let cache = rt.scratch.lock().unwrap();
        if let Some(t) = cache.get(&key) {
            return Some(t.0.clone());
        }
    }
    let d = super::device()?;
    let td = unsafe {
        MTLTextureDescriptor::texture2DDescriptorWithPixelFormat_width_height_mipmapped(
            fmt,
            w as usize,
            h as usize,
            false,
        )
    };
    td.setStorageMode(MTLStorageMode::Private);
    td.setHazardTrackingMode(MTLHazardTrackingMode::Untracked);
    td.setUsage(usage);
    let tex = d.dev.newTextureWithDescriptor(&td)?;
    tex.setLabel(Some(&objc2_foundation::NSString::from_str("port metalfx scratch")));
    d.add_resident(ProtocolObject::from_ref(&*tex));
    let mut cache = rt.scratch.lock().unwrap();
    if cache.len() >= MAX_SCRATCH_CACHE {
        if let Some(evict_key) = cache.keys().next().copied() {
            if let Some(old) = cache.remove(&evict_key) {
                d.remove_resident(ProtocolObject::from_ref(&*old.0));
                super::queue::defer_release_all(old);
            }
        }
    }
    cache.insert(key, SendTex(tex.clone()));
    Some(tex)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::PlacedFootprint;
    use crate::metal::cmdlist::{CommandAllocator, CommandList, CopyLoc, TargetView};
    use crate::metal::queue::{ListType, Queue};
    use crate::metal::resource::Resource;
    use crate::types::{dim, heap_type, resource_flags, HeapProperties, ResourceDesc};

    const RGBA8_UNORM: u32 = 28;
    const RGBA16_FLOAT: u32 = 10;
    const RG16_FLOAT: u32 = 34;
    const D32_FLOAT: u32 = 40;
    const R32_UINT: u32 = 42;
    const R16_FLOAT: u32 = 54;
    const R8_UNORM: u32 = 61;

    fn make_tex(w: u32, h: u32, fmt: u32, flags: u32, label: &str) -> Resource {
        let desc = ResourceDesc {
            Dimension: dim::TEXTURE2D,
            Width: w as u64,
            Height: h,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: fmt,
            SampleCount: 1,
            Flags: flags,
            ..Default::default()
        };
        let hp = HeapProperties { Type: heap_type::DEFAULT, ..Default::default() };
        Resource::new_committed_default(&desc, &hp, label).unwrap()
    }

    fn make_readback(bytes: u64) -> Resource {
        let desc = ResourceDesc {
            Dimension: dim::BUFFER,
            Width: bytes,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            SampleCount: 1,
            Layout: 1,
            ..Default::default()
        };
        let hp = HeapProperties { Type: heap_type::READBACK, ..Default::default() };
        Resource::new_committed_default(&desc, &hp, "metalfx readback").unwrap()
    }

    fn f16_to_f32(h: u16) -> f32 {
        let s = if h & 0x8000 != 0 { -1.0 } else { 1.0 };
        let e = ((h >> 10) & 0x1f) as i32;
        let m = (h & 0x3ff) as f32;
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
    fn format_selection_rejects_non_color_formats_safely() {
        assert_eq!(color_processing_mode(MTLPixelFormat::RGBA8Unorm), Some(ColorProcessingMode::Perceptual));
        assert_eq!(color_processing_mode(MTLPixelFormat::RGBA8Unorm_sRGB), Some(ColorProcessingMode::Perceptual));
        assert_eq!(color_processing_mode(MTLPixelFormat::BGRA8Unorm), Some(ColorProcessingMode::Perceptual));
        assert_eq!(color_processing_mode(MTLPixelFormat::RGBA16Float), Some(ColorProcessingMode::Linear));
        assert_eq!(color_processing_mode(MTLPixelFormat::RG11B10Float), Some(ColorProcessingMode::Linear));
        assert_eq!(select_color_processing_mode(MTLPixelFormat::RG11B10Float, MTLPixelFormat::RG11B10Float), Some(ColorProcessingMode::Linear));
        assert_eq!(select_color_processing_mode(MTLPixelFormat::RGBA16Float, MTLPixelFormat::RGBA16Float), Some(ColorProcessingMode::Linear));
        assert_eq!(color_processing_mode(MTLPixelFormat::R32Uint), None);
        assert_eq!(color_processing_mode(MTLPixelFormat::RGBA16Sint), None);
        assert_eq!(color_processing_mode(MTLPixelFormat::Depth32Float), None);
        assert_eq!(color_processing_mode(MTLPixelFormat::Depth32Float_Stencil8), None);
        assert!(get_or_create_scaler(64, 64, MTLPixelFormat::Depth32Float, 128, 128, MTLPixelFormat::Depth32Float).is_none());
        assert!(get_or_create_scaler(64, 64, MTLPixelFormat::R32Uint, 128, 128, MTLPixelFormat::R32Uint).is_none());
    }

    #[test]
    fn metalfx_spatial_hdr_preexposed_gradient_and_1to1_lossless_copy_on_gpu() {
        if super::super::device().is_none() || !is_enabled() {
            return;
        }
        // Verify pre-exposed HDR float values (e.g. 50..2000 in RGBA16Float) survive 1.5x spatial
        // upscaling without MetalFX Hdr-mode reversible-Reinhard plateau quantization or 484-clamp,
        // and verify 1:1 native-resolution upscale_resource performs a bit-exact copy without
        // invoking MTL4FXSpatialScaler.
        let q = Queue::new(ListType::Direct, "metalfx hdr/1to1 test").unwrap();
        let alloc = CommandAllocator::new(ListType::Direct).unwrap();
        let mut l = CommandList::new(ListType::Direct).unwrap();

        let src = make_tex(64, 64, RGBA16_FLOAT, resource_flags::ALLOW_RENDER_TARGET, "mfx hdr1000 src");
        let dst_up = make_tex(96, 96, RGBA16_FLOAT, resource_flags::ALLOW_UNORDERED_ACCESS, "mfx hdr1000 up");
        let dst_1to1 = make_tex(64, 64, RGBA16_FLOAT, resource_flags::ALLOW_UNORDERED_ACCESS, "mfx hdr1000 1to1");
        let rb_up = make_readback(1024 * 96);
        let rb_1to1 = make_readback(512 * 64);

        let rtv = TargetView::rtv(&src, None).unwrap();
        let c0 = encode_count();
        l.begin(&alloc).unwrap();
        unsafe {
            // Left half = 50.0, right half = 1200.0 (above the ~484 clamp of MetalFX Hdr mode).
            l.clear_rtv(
                &rtv,
                [15.0, 50.0, 55.0, 1.0],
                &[crate::types::Rect { left: 0, top: 0, right: 32, bottom: 64 }],
            )
            .unwrap();
            l.clear_rtv(
                &rtv,
                [300.0, 1200.0, 1320.0, 1.0],
                &[crate::types::Rect { left: 32, top: 0, right: 64, bottom: 64 }],
            )
            .unwrap();
        }
        // 1:1 native resolution must bypass MTL4FXSpatialScaler and copy losslessly.
        l.upscale_resource(&dst_1to1, &src, [64, 64], [64, 64]).unwrap();
        assert_eq!(encode_count(), c0, "1:1 upscale_resource must bypass MTL4FXSpatialScaler");
        // 1.5x upscale must invoke MTL4FXSpatialScaler in Linear mode and preserve >484 HDR highlights.
        l.upscale_resource(&dst_up, &src, [64, 64], [96, 96]).unwrap();
        assert_eq!(encode_count(), c0 + 1, "1.5x upscale_resource must encode through MTL4FXSpatialScaler");

        let fp_up = PlacedFootprint { offset: 0, format: RGBA16_FLOAT, width: 96, height: 96, depth: 1, row_pitch: 1024, _pad: 0 };
        let fp_1to1 = PlacedFootprint { offset: 0, format: RGBA16_FLOAT, width: 64, height: 64, depth: 1, row_pitch: 512, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb_up, fp_up), [0, 0, 0], CopyLoc::Subresource(&dst_up, 0), None).unwrap();
        l.copy_texture_region(CopyLoc::Footprint(&rb_1to1, fp_1to1), [0, 0, 0], CopyLoc::Subresource(&dst_1to1, 0), None).unwrap();
        l.close().unwrap();

        q.execute_command_lists(&mut [&mut l]);
        q.wait_idle();

        // 1:1 copy must be exact on both halves.
        let p1 = unsafe { std::slice::from_raw_parts(rb_1to1.cpu_ptr().unwrap() as *const u16, 64 * 256) };
        let b_l1 = 32 * 256 + 16 * 4;
        let b_r1 = 32 * 256 + 48 * 4;
        assert!((f16_to_f32(p1[b_l1 + 1]) - 50.0).abs() < 1e-2, "1:1 left G={}", f16_to_f32(p1[b_l1 + 1]));
        assert!((f16_to_f32(p1[b_r1 + 1]) - 1200.0).abs() < 1e-1, "1:1 right G={}", f16_to_f32(p1[b_r1 + 1]));

        // 1.5x MetalFX Linear upscale must preserve both 50.0 and 1200.0 accurately (no 484 clamp!).
        let pu = unsafe { std::slice::from_raw_parts(rb_up.cpu_ptr().unwrap() as *const u16, 96 * 512) };
        let b_lu = 48 * 512 + 20 * 4;
        let b_ru = 48 * 512 + 76 * 4;
        let gl = f16_to_f32(pu[b_lu + 1]);
        let gr = f16_to_f32(pu[b_ru + 1]);
        assert!((gl - 50.0).abs() < 2.0, "1.5x left G={gl} expected ~50.0");
        assert!((gr - 1200.0).abs() < 40.0, "1.5x right G={gr} expected ~1200.0 (no HDR 484 clamp)");
    }

    #[test]
    fn metalfx_spatial_upscale_direct_rt_hdr_and_uav_only_on_gpu() {
        if super::super::device().is_none() {
            return;
        }
        assert!(is_available(), "MetalFX.framework must support Metal 4 spatial scaling on this Mac");
        if !is_enabled() {
            return;
        }

        // Verify scaler creation + caching returns the same Arc instance.
        let s1 = get_or_create_scaler(64, 64, MTLPixelFormat::RGBA8Unorm, 128, 128, MTLPixelFormat::RGBA8Unorm)
            .expect("RGBA8Unorm 64x64->128x128 MetalFX scaler");
        let s2 = get_or_create_scaler(64, 64, MTLPixelFormat::RGBA8Unorm, 128, 128, MTLPixelFormat::RGBA8Unorm)
            .expect("cached RGBA8Unorm MetalFX scaler");
        assert!(Arc::ptr_eq(&s1, &s2), "scaler cache must reuse identical key");
        assert_eq!(s1.lock().unwrap().mode, ColorProcessingMode::Perceptual);

        let q = Queue::new(ListType::Direct, "metalfx test queue").unwrap();
        let alloc = CommandAllocator::new(ListType::Direct).unwrap();
        let mut l = CommandList::new(ListType::Direct).unwrap();

        // 1) Direct zero-copy MetalFX path: RGBA8_UNORM RenderTarget (DRS 48x48 inside 64x64 -> 128x128).
        let src_ldr = make_tex(64, 64, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET, "mfx src ldr");
        let dst_ldr_rt = make_tex(128, 128, RGBA8_UNORM, resource_flags::ALLOW_RENDER_TARGET, "mfx dst ldr rt");
        // 2) Scratch-RT + blit MetalFX path: UAV-only dst (no ALLOW_RENDER_TARGET, like FSR2/DLSS/XeSS outputs).
        let dst_ldr_uav = make_tex(128, 128, RGBA8_UNORM, resource_flags::ALLOW_UNORDERED_ACCESS, "mfx dst ldr uav");
        // 3) HDR float path: RGBA16_FLOAT with HDR radiance > 1.0 (e.g. [2.0, 0.5, 0.25, 1.0]).
        let src_hdr = make_tex(64, 64, RGBA16_FLOAT, resource_flags::ALLOW_RENDER_TARGET, "mfx src hdr");
        let dst_hdr_rt = make_tex(128, 128, RGBA16_FLOAT, resource_flags::ALLOW_RENDER_TARGET, "mfx dst hdr rt");

        let rb_ldr_rt = make_readback(128 * 4 * 128);
        let rb_ldr_uav = make_readback(128 * 4 * 128);
        let rb_hdr_rt = make_readback(128 * 8 * 128);

        let rtv_ldr = TargetView::rtv(&src_ldr, None).unwrap();
        let rtv_hdr = TargetView::rtv(&src_hdr, None).unwrap();

        let count0 = encode_count();
        l.begin(&alloc).unwrap();
        unsafe {
            l.clear_rtv(&rtv_ldr, [0.25, 0.5, 1.0, 1.0], &[]).unwrap();
            l.clear_rtv(&rtv_hdr, [2.0, 0.5, 0.25, 1.0], &[]).unwrap();
        }
        l.upscale_resource(&dst_ldr_rt, &src_ldr, [48, 48], [128, 128]).unwrap();
        l.upscale_resource(&dst_ldr_uav, &src_ldr, [64, 64], [128, 128]).unwrap();
        l.upscale_resource(&dst_hdr_rt, &src_hdr, [64, 64], [128, 128]).unwrap();

        let fp_ldr = PlacedFootprint { offset: 0, format: RGBA8_UNORM, width: 128, height: 128, depth: 1, row_pitch: 512, _pad: 0 };
        let fp_hdr = PlacedFootprint { offset: 0, format: RGBA16_FLOAT, width: 128, height: 128, depth: 1, row_pitch: 1024, _pad: 0 };
        l.copy_texture_region(CopyLoc::Footprint(&rb_ldr_rt, fp_ldr), [0, 0, 0], CopyLoc::Subresource(&dst_ldr_rt, 0), None).unwrap();
        l.copy_texture_region(CopyLoc::Footprint(&rb_ldr_uav, fp_ldr), [0, 0, 0], CopyLoc::Subresource(&dst_ldr_uav, 0), None).unwrap();
        l.copy_texture_region(CopyLoc::Footprint(&rb_hdr_rt, fp_hdr), [0, 0, 0], CopyLoc::Subresource(&dst_hdr_rt, 0), None).unwrap();
        l.close().unwrap();

        assert_eq!(encode_count() - count0, 3, "all three upscales must encode through native MetalFX");

        q.execute_command_lists(&mut [&mut l]);
        q.wait_idle();

        // Check center pixel of RGBA8 direct RT upscale (expect ~[64, 128, 255, 255]).
        let px_rt = unsafe { std::slice::from_raw_parts(rb_ldr_rt.cpu_ptr().unwrap(), 128 * 512) };
        let mid_rt = &px_rt[(64 * 128 + 64) * 4..(64 * 128 + 64) * 4 + 4];
        assert!((mid_rt[0] as i32 - 64).abs() <= 2, "rt R={}", mid_rt[0]);
        assert!((mid_rt[1] as i32 - 128).abs() <= 2, "rt G={}", mid_rt[1]);
        assert!((mid_rt[2] as i32 - 255).abs() <= 2, "rt B={}", mid_rt[2]);
        assert_eq!(mid_rt[3], 255, "rt A");

        // Check center pixel of RGBA8 UAV-only scratch+blit upscale.
        let px_uav = unsafe { std::slice::from_raw_parts(rb_ldr_uav.cpu_ptr().unwrap(), 128 * 512) };
        let mid_uav = &px_uav[(64 * 128 + 64) * 4..(64 * 128 + 64) * 4 + 4];
        assert!((mid_uav[0] as i32 - 64).abs() <= 2, "uav R={}", mid_uav[0]);
        assert!((mid_uav[1] as i32 - 128).abs() <= 2, "uav G={}", mid_uav[1]);
        assert!((mid_uav[2] as i32 - 255).abs() <= 2, "uav B={}", mid_uav[2]);
        assert_eq!(mid_uav[3], 255, "uav A");

        // Check center pixel of RGBA16F HDR upscale (HDR R=2.0 must survive reversible tone mapping!).
        let px_hdr = unsafe { std::slice::from_raw_parts(rb_hdr_rt.cpu_ptr().unwrap() as *const u16, 128 * 512) };
        let base = (64 * 128 + 64) * 4;
        let hr = f16_to_f32(px_hdr[base]);
        let hg = f16_to_f32(px_hdr[base + 1]);
        let hb = f16_to_f32(px_hdr[base + 2]);
        let ha = f16_to_f32(px_hdr[base + 3]);
        assert!((hr - 2.0).abs() < 0.05, "hdr R={hr}");
        assert!((hg - 0.5).abs() < 0.05, "hdr G={hg}");
        assert!((hb - 0.25).abs() < 0.05, "hdr B={hb}");
        assert!((ha - 1.0).abs() < 0.05, "hdr A={ha}");
    }

    #[test]
    fn temporal_format_validation_and_cache_keys() {
        assert!(temporal_depth_format_supported(MTLPixelFormat::Depth32Float));
        assert!(temporal_depth_format_supported(MTLPixelFormat::Depth16Unorm));
        assert!(temporal_depth_format_supported(MTLPixelFormat::R32Float));
        assert!(!temporal_depth_format_supported(MTLPixelFormat::RGBA8Unorm));
        assert!(!temporal_depth_format_supported(MTLPixelFormat::R32Uint));

        assert!(temporal_motion_format_supported(MTLPixelFormat::RG16Float));
        assert!(temporal_motion_format_supported(MTLPixelFormat::RG32Float));
        assert!(temporal_motion_format_supported(MTLPixelFormat::RGBA16Float));
        assert!(!temporal_motion_format_supported(MTLPixelFormat::Depth32Float));
        assert!(!temporal_motion_format_supported(MTLPixelFormat::R32Uint));

        assert!(temporal_reactive_format_supported(MTLPixelFormat::R8Unorm));
        assert!(temporal_reactive_format_supported(MTLPixelFormat::R16Float));
        assert!(!temporal_reactive_format_supported(MTLPixelFormat::R32Uint));

        // Unsupported formats must fail cleanly and never panic.
        let bad_depth = TemporalScalerKey::new(
            64,
            64,
            128,
            128,
            MTLPixelFormat::RGBA16Float,
            MTLPixelFormat::R32Uint,
            MTLPixelFormat::RG16Float,
            MTLPixelFormat::RGBA16Float,
        );
        assert!(get_or_create_temporal_scaler(bad_depth).is_none());

        let bad_motion = TemporalScalerKey::new(
            64,
            64,
            128,
            128,
            MTLPixelFormat::RGBA16Float,
            MTLPixelFormat::Depth32Float,
            MTLPixelFormat::R32Uint,
            MTLPixelFormat::RGBA16Float,
        );
        assert!(get_or_create_temporal_scaler(bad_motion).is_none());

        if super::super::device().is_none() || !is_temporal_available() {
            return;
        }
        let k1 = TemporalScalerKey::new(
            64,
            64,
            128,
            128,
            MTLPixelFormat::RGBA16Float,
            MTLPixelFormat::Depth32Float,
            MTLPixelFormat::RG16Float,
            MTLPixelFormat::RGBA16Float,
        );
        let t1 = get_or_create_temporal_scaler(k1).expect("RGBA16F temporal scaler");
        let t2 = get_or_create_temporal_scaler(k1).expect("cached RGBA16F temporal scaler");
        assert!(Arc::ptr_eq(&t1, &t2), "temporal scaler cache must reuse identical key");

        let k_auto = k1.with_auto_exposure(true);
        let t_auto = get_or_create_temporal_scaler(k_auto).expect("auto-exposure temporal scaler");
        assert!(!Arc::ptr_eq(&t1, &t_auto), "auto_exposure must use a distinct cache entry");

        let k_reactive = k1.with_reactive_format(Some(MTLPixelFormat::R8Unorm));
        let t_reactive = get_or_create_temporal_scaler(k_reactive).expect("reactive-mask temporal scaler");
        assert!(!Arc::ptr_eq(&t1, &t_reactive), "reactive format must use a distinct cache entry");
        assert!(t_reactive.lock().unwrap().reactive_usage.is_some());
    }

    #[test]
    fn metalfx_temporal_upscale_multiframe_jitter_depth_motion_and_fallbacks_on_gpu() {
        if super::super::device().is_none() {
            return;
        }
        assert!(
            is_temporal_available(),
            "MetalFX.framework must support Metal 4 temporal scaling on this Mac"
        );
        if !is_temporal_enabled() {
            // When PORT_METALFX=0, temporal upscale must fall back cleanly to BilinearCompute.
            let q = Queue::new(ListType::Direct, "metalfx optout queue").unwrap();
            let alloc = CommandAllocator::new(ListType::Direct).unwrap();
            let mut l = CommandList::new(ListType::Direct).unwrap();
            let color = make_tex(64, 64, RGBA16_FLOAT, resource_flags::ALLOW_RENDER_TARGET, "mfx optout c");
            let depth = make_tex(64, 64, D32_FLOAT, resource_flags::ALLOW_DEPTH_STENCIL, "mfx optout d");
            let motion = make_tex(64, 64, RG16_FLOAT, resource_flags::ALLOW_RENDER_TARGET, "mfx optout m");
            let out = make_tex(128, 128, RGBA16_FLOAT, resource_flags::ALLOW_UNORDERED_ACCESS, "mfx optout o");
            let rtv = TargetView::rtv(&color, None).unwrap();
            l.begin(&alloc).unwrap();
            unsafe { l.clear_rtv(&rtv, [0.5, 0.25, 0.75, 1.0], &[]).unwrap(); }
            let params = TemporalUpscaleParams::new(&out, &color, &depth, &motion, [64, 64], [128, 128]);
            assert_eq!(l.upscale_temporal_with_backend(&params).unwrap(), UpscaleBackend::BilinearCompute);
            l.close().unwrap();
            q.execute_command_lists(&mut [&mut l]);
            q.wait_idle();
            return;
        }

        let limits = temporal_scale_limits().expect("temporal scale limits");
        eprintln!("metalfx temporal scale limits: min={} max={}", limits.0, limits.1);
        assert!(limits.0 > 0.0 && limits.1 >= limits.0);

        let q = Queue::new(ListType::Direct, "metalfx temporal test queue").unwrap();
        let alloc = CommandAllocator::new(ListType::Direct).unwrap();
        let mut l = CommandList::new(ListType::Direct).unwrap();

        // Inputs: 64x64 color (RGBA16F), depth (D32F), motion (RG16F), reactive (R8Unorm), 1x1 exposure (R16F).
        let color = make_tex(64, 64, RGBA16_FLOAT, resource_flags::ALLOW_RENDER_TARGET, "mfx t color");
        let depth = make_tex(64, 64, D32_FLOAT, resource_flags::ALLOW_DEPTH_STENCIL, "mfx t depth");
        let motion = make_tex(64, 64, RG16_FLOAT, resource_flags::ALLOW_RENDER_TARGET, "mfx t motion");
        let reactive = make_tex(64, 64, R8_UNORM, resource_flags::ALLOW_RENDER_TARGET, "mfx t reactive");
        let exposure = make_tex(1, 1, R16_FLOAT, resource_flags::ALLOW_RENDER_TARGET, "mfx t exposure");
        let bad_motion = make_tex(64, 64, R32_UINT, resource_flags::ALLOW_RENDER_TARGET, "mfx t bad motion");

        // Outputs:
        // 1) Direct output with both RT + UAV usage (zero-copy direct MetalFX path).
        let out_direct = make_tex(
            128,
            128,
            RGBA16_FLOAT,
            resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS,
            "mfx t out direct",
        );
        // 2) UAV-only output (no RenderTarget usage -> exercises scratch+blit fallback when RT required).
        let out_uav_only = make_tex(
            128,
            128,
            RGBA16_FLOAT,
            resource_flags::ALLOW_UNORDERED_ACCESS,
            "mfx t out uav",
        );
        // 3) RT-only output (no ShaderWrite usage -> exercises scratch+blit fallback when ShaderWrite required).
        let out_rt_only = make_tex(
            128,
            128,
            RGBA16_FLOAT,
            resource_flags::ALLOW_RENDER_TARGET,
            "mfx t out rt",
        );

        let rb_direct = make_readback(128 * 8 * 128);
        let rb_uav = make_readback(128 * 8 * 128);
        let rb_rt = make_readback(128 * 8 * 128);

        let rtv_color = TargetView::rtv(&color, None).unwrap();
        let dsv_depth = TargetView::dsv(&depth, None).unwrap();
        let rtv_motion = TargetView::rtv(&motion, None).unwrap();
        let rtv_reactive = TargetView::rtv(&reactive, None).unwrap();
        let rtv_exposure = TargetView::rtv(&exposure, None).unwrap();

        let fp_hdr = PlacedFootprint {
            offset: 0,
            format: RGBA16_FLOAT,
            width: 128,
            height: 128,
            depth: 1,
            row_pitch: 1024,
            _pad: 0,
        };

        // 4-frame temporal sequence with a spatially varying 2-half HDR scene:
        // left half warm [1.6, 0.4, 0.2], right half cool [0.2, 0.6, 1.4].
        // Frame 0 resets history; frames 1..3 accumulate with subpixel jitter + depth + motion + reactive + exposure.
        let jitters: [[f32; 2]; 4] = [
            [0.0, 0.0],
            [-0.25, 0.1667],
            [0.25, -0.3333],
            [-0.125, -0.1667],
        ];
        let motions: [[f32; 4]; 4] = [
            [0.0, 0.0, 0.0, 0.0],
            [-0.5, 0.25, 0.0, 0.0],
            [0.25, -0.25, 0.0, 0.0],
            [-0.125, 0.125, 0.0, 0.0],
        ];
        let left_rgb = [1.6f32, 0.4, 0.2];
        let right_rgb = [0.2f32, 0.6, 1.4];

        let tcount0 = temporal_encode_count();
        for frame in 0..4usize {
            l.begin(&alloc).unwrap();
            unsafe {
                l.clear_rtv(
                    &rtv_color,
                    [left_rgb[0], left_rgb[1], left_rgb[2], 1.0],
                    &[crate::types::Rect { left: 0, top: 0, right: 32, bottom: 64 }],
                )
                .unwrap();
                l.clear_rtv(
                    &rtv_color,
                    [right_rgb[0], right_rgb[1], right_rgb[2], 1.0],
                    &[crate::types::Rect { left: 32, top: 0, right: 64, bottom: 64 }],
                )
                .unwrap();
                l.clear_dsv(&dsv_depth, true, 0.5, false, 0, &[]).unwrap();
                l.clear_rtv(&rtv_motion, motions[frame], &[]).unwrap();
                l.clear_rtv(&rtv_reactive, [0.0, 0.0, 0.0, 0.0], &[]).unwrap();
                l.clear_rtv(&rtv_exposure, [1.0, 0.0, 0.0, 0.0], &[]).unwrap();
            }

            let mut params = TemporalUpscaleParams::new(&out_direct, &color, &depth, &motion, [64, 64], [128, 128]);
            params.jitter_offset = jitters[frame];
            params.motion_vector_scale = [1.0, 1.0];
            params.reset = frame == 0;
            params.depth_reversed = true;
            params.pre_exposure = 1.0;
            params.exposure = Some(&exposure);
            params.reactive_mask = Some(&reactive);

            let backend = l.upscale_temporal_with_backend(&params).unwrap();
            assert_eq!(backend, UpscaleBackend::TemporalMetalFx, "frame {frame} must run on native MTL4FXTemporalScaler");

            l.copy_texture_region(
                CopyLoc::Footprint(&rb_direct, fp_hdr),
                [0, 0, 0],
                CopyLoc::Subresource(&out_direct, 0),
                None,
            )
            .unwrap();
            l.close().unwrap();

            q.execute_command_lists(&mut [&mut l]);
            q.wait_idle();

            let px = unsafe { std::slice::from_raw_parts(rb_direct.cpu_ptr().unwrap() as *const u16, 128 * 512) };
            for &((sx, sy), expected) in &[((24usize, 64usize), left_rgb), ((104usize, 64usize), right_rgb)] {
                let base = (sy * 128 + sx) * 4;
                let r = f16_to_f32(px[base]);
                let g = f16_to_f32(px[base + 1]);
                let b = f16_to_f32(px[base + 2]);
                assert!(r.is_finite() && g.is_finite() && b.is_finite(), "frame {frame} ({sx},{sy}) non-finite: ({r},{g},{b})");
                assert!(r + g + b > 0.5, "frame {frame} ({sx},{sy}) unexpectedly black: ({r},{g},{b})");
                assert!((r - expected[0]).abs() < 0.2, "frame {frame} ({sx},{sy}) R={r} expected {}", expected[0]);
                assert!((g - expected[1]).abs() < 0.2, "frame {frame} ({sx},{sy}) G={g} expected {}", expected[1]);
                assert!((b - expected[2]).abs() < 0.2, "frame {frame} ({sx},{sy}) B={b} expected {}", expected[2]);
            }
        }
        assert_eq!(temporal_encode_count() - tcount0, 4, "4 temporal frames must encode natively");

        // Verify DRS (48x48 active content inside 64x64 -> 128x128, scale 2.67x within [1, 3])
        // plus pre_exposure=2.0 and out-of-range DRS (16x16 -> 128x128, scale 8x) graceful fallback.
        l.begin(&alloc).unwrap();
        unsafe {
            l.clear_rtv(&rtv_color, [1.2, 0.8, 0.4, 1.0], &[]).unwrap();
            l.clear_dsv(&dsv_depth, true, 0.5, false, 0, &[]).unwrap();
            l.clear_rtv(&rtv_motion, [0.0, 0.0, 0.0, 0.0], &[]).unwrap();
        }
        let mut p_drs = TemporalUpscaleParams::new(&out_direct, &color, &depth, &motion, [48, 48], [128, 128]);
        p_drs.reset = true;
        p_drs.pre_exposure = 2.0;
        p_drs.auto_exposure = true;
        assert_eq!(l.upscale_temporal_with_backend(&p_drs).unwrap(), UpscaleBackend::TemporalMetalFx);
        l.copy_texture_region(CopyLoc::Footprint(&rb_direct, fp_hdr), [0, 0, 0], CopyLoc::Subresource(&out_direct, 0), None).unwrap();
        l.close().unwrap();
        q.execute_command_lists(&mut [&mut l]);
        q.wait_idle();
        let px_drs = unsafe { std::slice::from_raw_parts(rb_direct.cpu_ptr().unwrap() as *const u16, 128 * 512) };
        let b_drs = (64 * 128 + 64) * 4;
        let dr = f16_to_f32(px_drs[b_drs]);
        let dg = f16_to_f32(px_drs[b_drs + 1]);
        let db = f16_to_f32(px_drs[b_drs + 2]);
        assert!(dr.is_finite() && dg.is_finite() && db.is_finite(), "drs non-finite: ({dr},{dg},{db})");
        assert!((dr - 1.2).abs() < 0.25 && (dg - 0.8).abs() < 0.25 && (db - 0.4).abs() < 0.25, "drs rgb=({dr},{dg},{db})");

        l.begin(&alloc).unwrap();
        let mut p_oob = TemporalUpscaleParams::new(&out_direct, &color, &depth, &motion, [16, 16], [128, 128]);
        p_oob.reset = true;
        assert_eq!(
            l.upscale_temporal_with_backend(&p_oob).unwrap(),
            UpscaleBackend::SpatialMetalFx,
            "scale > max_scale must fall back cleanly without validation error"
        );
        l.close().unwrap();
        q.execute_command_lists(&mut [&mut l]);
        q.wait_idle();

        // Verify scratch fallback on UAV-only and RT-only output textures (auto_exposure=true).
        l.begin(&alloc).unwrap();
        unsafe {
            l.clear_rtv(&rtv_color, [0.8, 0.4, 0.2, 1.0], &[]).unwrap();
            l.clear_dsv(&dsv_depth, true, 0.5, false, 0, &[]).unwrap();
            l.clear_rtv(&rtv_motion, [0.0, 0.0, 0.0, 0.0], &[]).unwrap();
        }
        let mut p_uav = TemporalUpscaleParams::new(&out_uav_only, &color, &depth, &motion, [64, 64], [128, 128]);
        p_uav.reset = true;
        p_uav.auto_exposure = true;
        assert_eq!(l.upscale_temporal_with_backend(&p_uav).unwrap(), UpscaleBackend::TemporalMetalFx);

        let mut p_rt = TemporalUpscaleParams::new(&out_rt_only, &color, &depth, &motion, [64, 64], [128, 128]);
        p_rt.reset = true;
        p_rt.auto_exposure = true;
        assert_eq!(l.upscale_temporal_with_backend(&p_rt).unwrap(), UpscaleBackend::TemporalMetalFx);

        l.copy_texture_region(CopyLoc::Footprint(&rb_uav, fp_hdr), [0, 0, 0], CopyLoc::Subresource(&out_uav_only, 0), None).unwrap();
        l.copy_texture_region(CopyLoc::Footprint(&rb_rt, fp_hdr), [0, 0, 0], CopyLoc::Subresource(&out_rt_only, 0), None).unwrap();
        l.close().unwrap();
        q.execute_command_lists(&mut [&mut l]);
        q.wait_idle();

        for (label, rb) in [("uav-only", &rb_uav), ("rt-only", &rb_rt)] {
            let px = unsafe { std::slice::from_raw_parts(rb.cpu_ptr().unwrap() as *const u16, 128 * 512) };
            let base = (64 * 128 + 64) * 4;
            let r = f16_to_f32(px[base]);
            let g = f16_to_f32(px[base + 1]);
            let b = f16_to_f32(px[base + 2]);
            assert!((r - 0.8).abs() < 0.15, "{label} R={r}");
            assert!((g - 0.4).abs() < 0.15, "{label} G={g}");
            assert!((b - 0.2).abs() < 0.15, "{label} B={b}");
        }

        // Verify graceful fallback to spatial MetalFX when temporal motion format is unsupported.
        l.begin(&alloc).unwrap();
        let mut p_bad = TemporalUpscaleParams::new(&out_direct, &color, &depth, &bad_motion, [64, 64], [128, 128]);
        p_bad.reset = true;
        let fallback_backend = l.upscale_temporal_with_backend(&p_bad).unwrap();
        assert_eq!(fallback_backend, UpscaleBackend::SpatialMetalFx, "unsupported temporal format must fall back to spatial MetalFX");
        l.close().unwrap();
        q.execute_command_lists(&mut [&mut l]);
        q.wait_idle();
    }

    #[test]
    fn metalfx_temporal_conventions_d32s8_r32f_exp_and_hdr_on_gpu() {
        if super::super::device().is_none() || !is_temporal_enabled() {
            return;
        }
        assert!(temporal_depth_format_supported(MTLPixelFormat::Depth32Float_Stencil8));

        const D32_FLOAT_S8X24_UINT: u32 = 20;
        const R32_FLOAT: u32 = 41;

        let q = Queue::new(ListType::Direct, "mfx conv queue").unwrap();
        let alloc = CommandAllocator::new(ListType::Direct).unwrap();
        let mut l = CommandList::new(ListType::Direct).unwrap();

        let color = make_tex(64, 64, RGBA16_FLOAT, resource_flags::ALLOW_RENDER_TARGET, "p color");
        let depth_s8 = make_tex(64, 64, D32_FLOAT_S8X24_UINT, resource_flags::ALLOW_DEPTH_STENCIL, "p depth s8");
        let motion = make_tex(64, 64, RG16_FLOAT, resource_flags::ALLOW_RENDER_TARGET, "p motion");
        let motion_hr = make_tex(128, 128, RG16_FLOAT, resource_flags::ALLOW_RENDER_TARGET, "p motion hr");
        let exp_r32f = make_tex(1, 1, R32_FLOAT, resource_flags::ALLOW_RENDER_TARGET, "p exp r32f");
        let out_1to1 = make_tex(64, 64, RGBA16_FLOAT, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS, "p out 1to1");
        let out_2x = make_tex(128, 128, RGBA16_FLOAT, resource_flags::ALLOW_RENDER_TARGET | resource_flags::ALLOW_UNORDERED_ACCESS, "p out 2x");
        let rb_2x = make_readback(128 * 8 * 128);
        let rtv_c = TargetView::rtv(&color, None).unwrap();
        let dsv_d = TargetView::dsv(&depth_s8, None).unwrap();
        let rtv_m = TargetView::rtv(&motion, None).unwrap();
        let rtv_mhr = TargetView::rtv(&motion_hr, None).unwrap();
        let rtv_exp = TargetView::rtv(&exp_r32f, None).unwrap();
        let fp_2x = PlacedFootprint { offset: 0, format: RGBA16_FLOAT, width: 128, height: 128, depth: 1, row_pitch: 1024, _pad: 0 };

        // 1) Depth32Float_Stencil8 + 1:1 Native AA + R32Float 1x1 exposure conversion.
        l.begin(&alloc).unwrap();
        unsafe {
            l.clear_rtv(&rtv_c, [0.5, 0.5, 0.5, 1.0], &[]).unwrap();
            l.clear_dsv(&dsv_d, true, 0.5, true, 0, &[]).unwrap();
            l.clear_rtv(&rtv_m, [0.0, 0.0, 0.0, 0.0], &[]).unwrap();
            l.clear_rtv(&rtv_exp, [0.01, 0.0, 0.0, 0.0], &[]).unwrap();
        }
        let mut p1 = TemporalUpscaleParams::new(&out_1to1, &color, &depth_s8, &motion, [64, 64], [64, 64]);
        p1.reset = true;
        p1.exposure = Some(&exp_r32f);
        assert_eq!(l.upscale_temporal_with_backend(&p1).unwrap(), UpscaleBackend::TemporalMetalFx);
        l.close().unwrap();
        q.execute_command_lists(&mut [&mut l]);
        q.wait_idle();

        // 2) Jitter sign convention on both axes: positive jitterOffset samples at (p + jitter),
        // pulling the [32..64) white quadrant toward (0,0) and matching FSR3's convention identically.
        let mut sums = [(0.0f32, 0.0f32); 5];
        for (idx, &(jx, jy)) in [(0.0f32, 0.0f32), (0.5, 0.0), (-0.5, 0.0), (0.0, 0.5), (0.0, -0.5)].iter().enumerate() {
            l.begin(&alloc).unwrap();
            unsafe {
                l.clear_rtv(&rtv_c, [0.0, 0.0, 0.0, 1.0], &[]).unwrap();
                l.clear_rtv(
                    &rtv_c,
                    [1.0, 1.0, 1.0, 1.0],
                    &[crate::types::Rect { left: 32, top: 32, right: 64, bottom: 64 }],
                ).unwrap();
                l.clear_dsv(&dsv_d, true, 0.5, false, 0, &[]).unwrap();
                l.clear_rtv(&rtv_m, [0.0, 0.0, 0.0, 0.0], &[]).unwrap();
            }
            let mut p = TemporalUpscaleParams::new(&out_2x, &color, &depth_s8, &motion, [64, 64], [128, 128]);
            p.reset = true;
            p.jitter_offset = [jx, jy];
            assert_eq!(l.upscale_temporal_with_backend(&p).unwrap(), UpscaleBackend::TemporalMetalFx);
            assert!(
                l.metalfx_post_sync_clean(),
                "direct MTL4FXTemporalScaler pass must arm a full post-pass queue barrier and clear stale stage_sync",
            );
            l.copy_texture_region(CopyLoc::Footprint(&rb_2x, fp_2x), [0, 0, 0], CopyLoc::Subresource(&out_2x, 0), None).unwrap();
            l.close().unwrap();
            q.execute_command_lists(&mut [&mut l]);
            q.wait_idle();
            let px = unsafe { std::slice::from_raw_parts(rb_2x.cpu_ptr().unwrap() as *const u16, 128 * 512) };
            let mut row_sum = 0.0f32;
            let mut col_sum = 0.0f32;
            for i in 0..128 {
                row_sum += f16_to_f32(px[(96 * 128 + i) * 4]);
                col_sum += f16_to_f32(px[(i * 128 + 96) * 4]);
            }
            sums[idx] = (row_sum, col_sum);
        }
        assert!(sums[1].0 > sums[0].0 + 0.5, "+0.5 X jitter must shift white edge left: {:?}", sums);
        assert!(sums[2].0 < sums[0].0 - 0.5, "-0.5 X jitter must shift white edge right: {:?}", sums);
        assert!(sums[3].1 > sums[0].1 + 0.5, "+0.5 Y jitter must shift white edge up: {:?}", sums);
        assert!(sums[4].1 < sums[0].1 - 0.5, "-0.5 Y jitter must shift white edge down: {:?}", sums);

        // 3) Pre-exposed HDR scene values (50..2400) survive accurately with R32Float exposure or auto_exposure.
        for &(use_auto, use_exp_tex) in &[(true, false), (false, true)] {
            l.begin(&alloc).unwrap();
            unsafe {
                l.clear_rtv(&rtv_c, [15.0, 50.0, 55.0, 1.0], &[crate::types::Rect { left: 0, top: 0, right: 32, bottom: 64 }]).unwrap();
                l.clear_rtv(&rtv_c, [300.0, 1200.0, 2400.0, 1.0], &[crate::types::Rect { left: 32, top: 0, right: 64, bottom: 64 }]).unwrap();
                l.clear_dsv(&dsv_d, true, 0.5, false, 0, &[]).unwrap();
                l.clear_rtv(&rtv_m, [0.0, 0.0, 0.0, 0.0], &[]).unwrap();
                l.clear_rtv(&rtv_exp, [0.01, 0.0, 0.0, 0.0], &[]).unwrap();
            }
            let mut p = TemporalUpscaleParams::new(&out_2x, &color, &depth_s8, &motion, [64, 64], [128, 128]);
            p.reset = true;
            p.auto_exposure = use_auto;
            if use_exp_tex {
                p.exposure = Some(&exp_r32f);
            }
            assert_eq!(l.upscale_temporal_with_backend(&p).unwrap(), UpscaleBackend::TemporalMetalFx);
            assert!(
                l.metalfx_post_sync_clean(),
                "direct MTL4FXTemporalScaler pass must arm a full post-pass queue barrier and clear stale stage_sync",
            );
            l.copy_texture_region(CopyLoc::Footprint(&rb_2x, fp_2x), [0, 0, 0], CopyLoc::Subresource(&out_2x, 0), None).unwrap();
            l.close().unwrap();
            q.execute_command_lists(&mut [&mut l]);
            q.wait_idle();
            let px = unsafe { std::slice::from_raw_parts(rb_2x.cpu_ptr().unwrap() as *const u16, 128 * 512) };
            let bl = (64 * 128 + 24) * 4;
            let br = (64 * 128 + 104) * 4;
            assert!((f16_to_f32(px[bl + 1]) - 50.0).abs() < 2.0, "left G={}", f16_to_f32(px[bl + 1]));
            assert!((f16_to_f32(px[br + 1]) - 1200.0).abs() < 40.0, "right G={}", f16_to_f32(px[br + 1]));
            assert!((f16_to_f32(px[br + 2]) - 2400.0).abs() < 80.0, "right B={}", f16_to_f32(px[br + 2]));
        }

        // 4) Output-resolution + jittered motion vectors descriptor flags.
        l.begin(&alloc).unwrap();
        unsafe {
            l.clear_rtv(&rtv_c, [0.4, 0.6, 0.8, 1.0], &[]).unwrap();
            l.clear_dsv(&dsv_d, true, 0.5, false, 0, &[]).unwrap();
            l.clear_rtv(&rtv_mhr, [0.0, 0.0, 0.0, 0.0], &[]).unwrap();
        }
        let mut p_hr = TemporalUpscaleParams::new(&out_2x, &color, &depth_s8, &motion_hr, [64, 64], [128, 128]);
        p_hr.reset = true;
        p_hr.auto_exposure = true;
        p_hr.output_resolution_motion_vectors = true;
        p_hr.jittered_motion_vectors = true;
        let b_hr = l.upscale_temporal_with_backend(&p_hr).unwrap();
        assert!(matches!(b_hr, UpscaleBackend::TemporalMetalFx | UpscaleBackend::SpatialMetalFx));
        l.close().unwrap();
        q.execute_command_lists(&mut [&mut l]);
        q.wait_idle();
    }
}
