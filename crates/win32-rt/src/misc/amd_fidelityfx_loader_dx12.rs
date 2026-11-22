//! Builtin `amd_fidelityfx_loader_dx12.dll` (AMD FidelityFX API loader).
//!
//! Implements the 5 `ffx*` entry points from `third_party/fidelityfx-sdk`
//! (`Kits/FidelityFX/api/internal/ffx_api.cpp`, `ffx_query_fallback.cpp`, and
//! `ffx_provider_fsr3framegenerationswapchain_dx12.cpp`) natively in Rust:
//! * `ffxCreateContextDescFrameGenerationSwapChain{Wrap,New,ForHwnd}DX12`
//!   (`0x30001`, `0x30005`, `0x30006`) create or wrap our native `IDXGISwapChain4`
//!   directly via `IDXGIFactory` without the 37.3 MB x86-64 proxy swapchain DLL.
//! * `ffxCreateContextDescUpscale` (`0x10000`) and `ffxDispatchDescUpscale`
//!   (`0x10001`) execute through Apple MetalFX Temporal (`upscale_temporal`,
//!   with spatial fallback).
//! * `ffxDispatchDescUpscaleGenerateReactiveMask` (`0x10006`) parses the full
//!   FFX struct and clears `outReactive` to `0.0` via D3D12 so it is never
//!   uninitialized garbage.
//! * Frame Generation (`0x0002_xxxx`, plus swapchain interpolation command list
//!   and interpolation texture queries `0x30003`/`0x30004`) is honestly refused
//!   with `FFX_API_RETURN_NO_PROVIDER`.

use std::ffi::c_void;
use std::sync::OnceLock;

pub static ORDINALS: &[(u16, &str)] = &[];
pub static EXTRA_EXPORTS: &[crate::Export] = &[];

pub const FFX_API_RETURN_OK: u32 = 0;
pub const FFX_API_RETURN_ERROR_RUNTIME_ERROR: u32 = 3;
pub const FFX_API_RETURN_NO_PROVIDER: u32 = 4;
pub const FFX_API_RETURN_ERROR_PARAMETER: u32 = 6;

const FFX_API_EFFECT_MASK: u64 = 0x00ff_0000;
const FFX_API_EFFECT_ID_UPSCALE: u64 = 0x0001_0000;
const FFX_API_EFFECT_ID_FRAMEGENERATION: u64 = 0x0002_0000;
const FFX_API_EFFECT_ID_FRAMEGENERATIONSWAPCHAIN: u64 = 0x0003_0000;

const FFX_API_CONFIGURE_DESC_TYPE_GLOBALDEBUG1: u64 = 1;
const FFX_API_QUERY_DESC_TYPE_GET_VERSIONS: u64 = 4;
const FFX_API_QUERY_DESC_TYPE_GET_PROVIDER_VERSION: u64 = 6;
const FFX_API_CONFIGURE_DESC_TYPE_GLOBALDEBUG: u64 = 7;

const FFX_API_CREATE_CONTEXT_DESC_TYPE_UPSCALE: u64 = 0x0001_0000;
const FFX_API_DISPATCH_DESC_TYPE_UPSCALE: u64 = 0x0001_0001;
const FFX_API_QUERY_DESC_TYPE_UPSCALE_GETUPSCALERATIOFROMQUALITYMODE: u64 = 0x0001_0002;
const FFX_API_QUERY_DESC_TYPE_UPSCALE_GETRENDERRESOLUTIONFROMQUALITYMODE: u64 = 0x0001_0003;
const FFX_API_QUERY_DESC_TYPE_UPSCALE_GETJITTERPHASECOUNT: u64 = 0x0001_0004;
const FFX_API_QUERY_DESC_TYPE_UPSCALE_GETJITTEROFFSET: u64 = 0x0001_0005;
const FFX_API_DISPATCH_DESC_TYPE_UPSCALE_GENERATEREACTIVEMASK: u64 = 0x0001_0006;
const FFX_API_CONFIGURE_DESC_TYPE_UPSCALE_KEYVALUE: u64 = 0x0001_0007;
const FFX_API_QUERY_DESC_TYPE_UPSCALE_GPU_MEMORY_USAGE: u64 = 0x0001_0008;
const FFX_API_QUERY_DESC_TYPE_UPSCALE_GPU_MEMORY_USAGE_V2: u64 = 0x0001_0009;
const FFX_API_QUERY_DESC_TYPE_UPSCALE_GET_RESOURCE_REQUIREMENTS: u64 = 0x0001_000a;

const FFX_API_CREATE_CONTEXT_DESC_TYPE_FRAMEGENERATION: u64 = 0x0002_0001;

const FFX_API_CREATE_CONTEXT_DESC_TYPE_FRAMEGENERATIONSWAPCHAIN_WRAP_DX12: u64 = 0x0003_0001;
const FFX_API_QUERY_DESC_TYPE_FRAMEGENERATIONSWAPCHAIN_INTERPOLATIONCOMMANDLIST_DX12: u64 = 0x0003_0003;
const FFX_API_QUERY_DESC_TYPE_FRAMEGENERATIONSWAPCHAIN_INTERPOLATIONTEXTURE_DX12: u64 = 0x0003_0004;
const FFX_API_CREATE_CONTEXT_DESC_TYPE_FRAMEGENERATIONSWAPCHAIN_NEW_DX12: u64 = 0x0003_0005;
const FFX_API_CREATE_CONTEXT_DESC_TYPE_FRAMEGENERATIONSWAPCHAIN_FOR_HWND_DX12: u64 = 0x0003_0006;
const FFX_API_DISPATCH_DESC_TYPE_FRAMEGENERATIONSWAPCHAIN_WAIT_FOR_PRESENTS_DX12: u64 = 0x0003_0007;
const FFX_API_QUERY_DESC_TYPE_FRAMEGENERATIONSWAPCHAIN_GPU_MEMORY_USAGE_DX12: u64 = 0x0003_0009;
const FFX_API_QUERY_DESC_TYPE_FRAMEGENERATIONSWAPCHAIN_GPU_MEMORY_USAGE_DX12_V2: u64 = 0x0003_000a;

const FFX_UPSCALE_ENABLE_HIGH_DYNAMIC_RANGE: u32 = 1 << 0;
const FFX_UPSCALE_ENABLE_DISPLAY_RESOLUTION_MOTION_VECTORS: u32 = 1 << 1;
const FFX_UPSCALE_ENABLE_MOTION_VECTORS_JITTER_CANCELLATION: u32 = 1 << 2;
const FFX_UPSCALE_ENABLE_DEPTH_INVERTED: u32 = 1 << 3;
const FFX_UPSCALE_ENABLE_AUTO_EXPOSURE: u32 = 1 << 5;
const FFX_UPSCALE_ENABLE_DYNAMIC_RESOLUTION: u32 = 1 << 6;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FfxApiHeader {
    pub ty: u64,
    pub next: *mut FfxApiHeader,
}
const _: () = assert!(std::mem::size_of::<FfxApiHeader>() == 16);

/// `FfxApiResource` (`ffx_api_types.h`, 48 bytes).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FfxApiResource {
    pub resource: *mut c_void,
    pub description: [u32; 8],
    pub state: u32,
}
const _: () = assert!(std::mem::size_of::<FfxApiResource>() == 0x30);

/// `ffxDispatchDescUpscaleGenerateReactiveMask` (`ffx_upscale.h`, 192 bytes).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FfxDispatchDescUpscaleGenerateReactiveMask {
    pub header: FfxApiHeader,
    pub command_list: *mut c_void,
    pub color_opaque_only: FfxApiResource,
    pub color_pre_upscale: FfxApiResource,
    pub out_reactive: FfxApiResource,
    pub render_size: [u32; 2],
    pub scale: f32,
    pub cutoff_threshold: f32,
    pub binary_value: f32,
    pub flags: u32,
}
const _: () = assert!(std::mem::size_of::<FfxDispatchDescUpscaleGenerateReactiveMask>() == 0xC0);
const _: () = assert!(std::mem::offset_of!(FfxDispatchDescUpscaleGenerateReactiveMask, command_list) == 0x10);
const _: () = assert!(std::mem::offset_of!(FfxDispatchDescUpscaleGenerateReactiveMask, color_opaque_only) == 0x18);
const _: () = assert!(std::mem::offset_of!(FfxDispatchDescUpscaleGenerateReactiveMask, color_pre_upscale) == 0x48);
const _: () = assert!(std::mem::offset_of!(FfxDispatchDescUpscaleGenerateReactiveMask, out_reactive) == 0x78);
const _: () = assert!(std::mem::offset_of!(FfxDispatchDescUpscaleGenerateReactiveMask, render_size) == 0xA8);
const _: () = assert!(std::mem::offset_of!(FfxDispatchDescUpscaleGenerateReactiveMask, scale) == 0xB0);
const _: () = assert!(std::mem::offset_of!(FfxDispatchDescUpscaleGenerateReactiveMask, cutoff_threshold) == 0xB4);
const _: () = assert!(std::mem::offset_of!(FfxDispatchDescUpscaleGenerateReactiveMask, binary_value) == 0xB8);
const _: () = assert!(std::mem::offset_of!(FfxDispatchDescUpscaleGenerateReactiveMask, flags) == 0xBC);

struct FfxContextState {
    swapchain: usize,
    upscale_flags: u32,
    #[allow(dead_code)]
    max_render_size: [u32; 2],
    max_upscale_size: [u32; 2],
    first_dispatch: bool,
    logged_first_dispatch: bool,
    logged_gen_reactive: bool,
}

impl FfxContextState {
    fn new_swapchain(swapchain: usize) -> Self {
        Self {
            swapchain,
            upscale_flags: 0,
            max_render_size: [0, 0],
            max_upscale_size: [0, 0],
            first_dispatch: true,
            logged_first_dispatch: false,
            logged_gen_reactive: false,
        }
    }
}

fn halton(mut index: i32, base: i32) -> f32 {
    let mut f = 1.0f32;
    let mut result = 0.0f32;
    let b = base as f32;
    while index > 0 {
        f /= b;
        result += f * (index % base) as f32;
        index /= base;
    }
    result
}

/// Exact FidelityFX FSR3 Halton jitter phase count:
/// `max(1, floor(8 * (displayWidth / renderWidth)^2))`.
fn compute_jitter_phase_count(render_width: u32, display_width: u32) -> i32 {
    if render_width == 0 || display_width == 0 {
        return 8;
    }
    let ratio = display_width as f32 / render_width as f32;
    ((8.0f32 * ratio * ratio) as i32).max(1)
}

/// Exact FidelityFX FSR3 Halton `(base 2, base 3)` subpixel jitter offset in `[-0.5, 0.5]`.
fn compute_jitter_offset(index: i32, phase_count: i32) -> (f32, f32) {
    if phase_count <= 0 {
        return (0.0, 0.0);
    }
    let idx = index.rem_euclid(phase_count) + 1;
    (halton(idx, 2) - 0.5, halton(idx, 3) - 0.5)
}

unsafe fn com_add_ref(obj: usize) {
    if obj != 0 {
        let vtbl = std::ptr::read_unaligned(obj as *const usize);
        let add_ref = std::ptr::read_unaligned((vtbl + 0x08) as *const usize);
        crate::call_guest8(add_ref, obj as u64, 0, 0, 0, 0, 0, 0, 0);
    }
}

unsafe fn com_release(obj: usize) {
    if obj != 0 {
        let vtbl = std::ptr::read_unaligned(obj as *const usize);
        let release = std::ptr::read_unaligned((vtbl + 0x10) as *const usize);
        crate::call_guest8(release, obj as u64, 0, 0, 0, 0, 0, 0, 0);
    }
}

fn clear_uav_float_slot() -> usize {
    static SLOT: OnceLock<usize> = OnceLock::new();
    *SLOT.get_or_init(|| {
        crate::d3d12::vtables::ID3D12GraphicsCommandList10
            .iter()
            .position(|&name| name == "ClearUnorderedAccessViewFloat")
            .expect("ClearUnorderedAccessViewFloat slot in ID3D12GraphicsCommandList10")
    })
}

/// Clear a D3D12 texture resource to `0.0` in-order on `cmd_list` via
/// `ID3D12GraphicsCommandList::ClearUnorderedAccessViewFloat`.
unsafe fn clear_reactive_mask_to_zero(cmd_list: *mut c_void, out_reactive: *mut c_void) -> bool {
    if cmd_list.is_null() || out_reactive.is_null() {
        return false;
    }
    if crate::d3d12::cmdlist::from_ptr(cmd_list).is_none() {
        return false;
    }
    let Some(res) = crate::d3d12::resource::from_ptr(out_reactive) else {
        return false;
    };
    if res.r.desc.is_buffer() {
        return false;
    }
    let vtbl = std::ptr::read_unaligned(cmd_list as *const *const usize);
    let clear_fn: unsafe extern "win64" fn(
        *mut c_void,
        u64,
        usize,
        *mut c_void,
        *const [f32; 4],
        u32,
        *const c_void,
    ) = std::mem::transmute(std::ptr::read_unaligned(vtbl.add(clear_uav_float_slot())));
    let zeros = [0.0f32; 4];
    clear_fn(cmd_list, 0, 0, out_reactive, &zeros, 0, std::ptr::null());
    true
}

pub unsafe extern "win64" fn ffxCreateContext(
    context: *mut usize,
    desc: *const FfxApiHeader,
    _mem_cb: usize,
) -> u32 {
    if context.is_null() || desc.is_null() {
        return FFX_API_RETURN_ERROR_PARAMETER;
    }
    std::ptr::write_unaligned(context, 0);
    let ty = std::ptr::read_unaligned(&(*desc).ty);
    let base = desc as usize;

    match ty {
        FFX_API_CREATE_CONTEXT_DESC_TYPE_UPSCALE => {
            // struct ffxCreateContextDescUpscale:
            //   +0x00: header (16B)
            //   +0x10: uint32_t flags
            //   +0x14: FfxApiDimensions2D maxRenderSize
            //   +0x1C: FfxApiDimensions2D maxUpscaleSize
            let flags = std::ptr::read_unaligned((base + 0x10) as *const u32);
            let max_render_size = std::ptr::read_unaligned((base + 0x14) as *const [u32; 2]);
            let max_upscale_size = std::ptr::read_unaligned((base + 0x1C) as *const [u32; 2]);
            log::info!(
                target: "d3d",
                "ffxCreateContext(UPSCALE): flags={flags:#x} maxRender={}x{} maxUpscale={}x{} temporal={}",
                max_render_size[0],
                max_render_size[1],
                max_upscale_size[0],
                max_upscale_size[1],
                crate::d3d12::cmdlist::temporal_upscale_enabled(),
            );
            let mut state = FfxContextState::new_swapchain(0);
            state.upscale_flags = flags;
            state.max_render_size = max_render_size;
            state.max_upscale_size = max_upscale_size;
            let ctx = Box::into_raw(Box::new(state)) as usize;
            std::ptr::write_unaligned(context, ctx);
            FFX_API_RETURN_OK
        }
        FFX_API_CREATE_CONTEXT_DESC_TYPE_FRAMEGENERATION => {
            crate::stub!(
                "ffxCreateContext(FRAMEGENERATION): frame generation is not supported -> FFX_API_RETURN_NO_PROVIDER"
            );
            FFX_API_RETURN_NO_PROVIDER
        }
        FFX_API_CREATE_CONTEXT_DESC_TYPE_FRAMEGENERATIONSWAPCHAIN_WRAP_DX12 => {
            let sc_ptr = std::ptr::read_unaligned((base + 0x10) as *const *mut usize);
            if sc_ptr.is_null() {
                return FFX_API_RETURN_ERROR_PARAMETER;
            }
            let sc = std::ptr::read_unaligned(sc_ptr);
            com_add_ref(sc);
            let ctx = Box::into_raw(Box::new(FfxContextState::new_swapchain(sc))) as usize;
            std::ptr::write_unaligned(context, ctx);
            FFX_API_RETURN_OK
        }
        FFX_API_CREATE_CONTEXT_DESC_TYPE_FRAMEGENERATIONSWAPCHAIN_NEW_DX12 => {
            // struct ffxCreateContextDescFrameGenerationSwapChainNewDX12:
            //   +0x00: header (16B)
            //   +0x10: IDXGISwapChain4** swapchain
            //   +0x18: DXGI_SWAP_CHAIN_DESC* desc
            //   +0x20: IDXGIFactory* dxgiFactory
            //   +0x28: ID3D12CommandQueue* gameQueue
            let sc_out = std::ptr::read_unaligned((base + 0x10) as *const *mut usize);
            let sc_desc = std::ptr::read_unaligned((base + 0x18) as *const usize);
            let factory = std::ptr::read_unaligned((base + 0x20) as *const usize);
            let queue = std::ptr::read_unaligned((base + 0x28) as *const usize);
            if sc_out.is_null() || sc_desc == 0 || factory == 0 || queue == 0 {
                return FFX_API_RETURN_ERROR_PARAMETER;
            }
            let vtbl = std::ptr::read_unaligned(factory as *const usize);
            // IDXGIFactory::CreateSwapChain is vtable slot 10 (+0x50).
            let create_sc = std::ptr::read_unaligned((vtbl + 0x50) as *const usize);
            let hr = crate::call_guest8(
                create_sc,
                factory as u64,
                queue as u64,
                sc_desc as u64,
                sc_out as u64,
                0,
                0,
                0,
                0,
            ) as i32;
            if hr < 0 {
                return FFX_API_RETURN_ERROR_RUNTIME_ERROR;
            }
            let sc = std::ptr::read_unaligned(sc_out);
            com_add_ref(sc);
            let ctx = Box::into_raw(Box::new(FfxContextState::new_swapchain(sc))) as usize;
            std::ptr::write_unaligned(context, ctx);
            FFX_API_RETURN_OK
        }
        FFX_API_CREATE_CONTEXT_DESC_TYPE_FRAMEGENERATIONSWAPCHAIN_FOR_HWND_DX12 => {
            // struct ffxCreateContextDescFrameGenerationSwapChainForHwndDX12:
            //   +0x10: IDXGISwapChain4** swapchain
            //   +0x18: HWND hwnd
            //   +0x20: DXGI_SWAP_CHAIN_DESC1* desc
            //   +0x28: DXGI_SWAP_CHAIN_FULLSCREEN_DESC* fullscreenDesc
            //   +0x30: IDXGIFactory* dxgiFactory
            //   +0x38: ID3D12CommandQueue* gameQueue
            let sc_out = std::ptr::read_unaligned((base + 0x10) as *const *mut usize);
            let hwnd = std::ptr::read_unaligned((base + 0x18) as *const usize);
            let sc_desc1 = std::ptr::read_unaligned((base + 0x20) as *const usize);
            let fs_desc = std::ptr::read_unaligned((base + 0x28) as *const usize);
            let factory = std::ptr::read_unaligned((base + 0x30) as *const usize);
            let queue = std::ptr::read_unaligned((base + 0x38) as *const usize);
            if sc_out.is_null() || sc_desc1 == 0 || factory == 0 || queue == 0 {
                return FFX_API_RETURN_ERROR_PARAMETER;
            }
            let vtbl = std::ptr::read_unaligned(factory as *const usize);
            // IDXGIFactory2::CreateSwapChainForHwnd is vtable slot 15 (+0x78).
            let create_hwnd = std::ptr::read_unaligned((vtbl + 0x78) as *const usize);
            let hr = crate::call_guest8(
                create_hwnd,
                factory as u64,
                queue as u64,
                hwnd as u64,
                sc_desc1 as u64,
                fs_desc as u64,
                0,
                sc_out as u64,
                0,
            ) as i32;
            if hr < 0 {
                return FFX_API_RETURN_ERROR_RUNTIME_ERROR;
            }
            let sc = std::ptr::read_unaligned(sc_out);
            com_add_ref(sc);
            let ctx = Box::into_raw(Box::new(FfxContextState::new_swapchain(sc))) as usize;
            std::ptr::write_unaligned(context, ctx);
            FFX_API_RETURN_OK
        }
        _ => FFX_API_RETURN_NO_PROVIDER,
    }
}

pub unsafe extern "win64" fn ffxDestroyContext(context: *mut usize, _mem_cb: usize) -> u32 {
    if context.is_null() {
        return FFX_API_RETURN_ERROR_PARAMETER;
    }
    let ctx = std::ptr::read_unaligned(context);
    if ctx == 0 {
        return FFX_API_RETURN_ERROR_PARAMETER;
    }
    std::ptr::write_unaligned(context, 0);
    let fg = Box::from_raw(ctx as *mut FfxContextState);
    com_release(fg.swapchain);
    FFX_API_RETURN_OK
}

pub unsafe extern "win64" fn ffxConfigure(context: *mut usize, header: *const FfxApiHeader) -> u32 {
    if header.is_null() {
        return FFX_API_RETURN_ERROR_PARAMETER;
    }
    let ty = std::ptr::read_unaligned(&(*header).ty);
    if ty == FFX_API_CONFIGURE_DESC_TYPE_GLOBALDEBUG1
        || ty == FFX_API_CONFIGURE_DESC_TYPE_GLOBALDEBUG
        || ty == FFX_API_CONFIGURE_DESC_TYPE_UPSCALE_KEYVALUE
    {
        return FFX_API_RETURN_OK;
    }
    if context.is_null() || std::ptr::read_unaligned(context) == 0 {
        return FFX_API_RETURN_ERROR_PARAMETER;
    }
    match ty & FFX_API_EFFECT_MASK {
        FFX_API_EFFECT_ID_UPSCALE | FFX_API_EFFECT_ID_FRAMEGENERATIONSWAPCHAIN => FFX_API_RETURN_OK,
        FFX_API_EFFECT_ID_FRAMEGENERATION => {
            crate::stub!("ffxConfigure(ty={ty:#x}): frame generation is not supported -> FFX_API_RETURN_NO_PROVIDER");
            FFX_API_RETURN_NO_PROVIDER
        }
        _ => FFX_API_RETURN_NO_PROVIDER,
    }
}

pub unsafe extern "win64" fn ffxDispatch(context: *mut usize, header: *const FfxApiHeader) -> u32 {
    if context.is_null() || header.is_null() {
        return FFX_API_RETURN_ERROR_PARAMETER;
    }
    let ctx_ptr = std::ptr::read_unaligned(context);
    if ctx_ptr == 0 {
        return FFX_API_RETURN_ERROR_PARAMETER;
    }
    let ctx = &mut *(ctx_ptr as *mut FfxContextState);
    let ty = std::ptr::read_unaligned(&(*header).ty);
    let base = header as usize;
    match ty {
        FFX_API_DISPATCH_DESC_TYPE_UPSCALE => {
            // struct ffxDispatchDescUpscale (each FfxApiResource is 48B = 0x30):
            //   +0x10: commandList
            //   +0x18: color.resource
            //   +0x48: depth.resource
            //   +0x78: motionVectors.resource
            //   +0xA8: exposure.resource
            //   +0xD8: reactive.resource
            //   +0x108: transparencyAndComposition.resource
            //   +0x138: output.resource
            //   +0x168: jitterOffset (2 x f32)
            //   +0x170: motionVectorScale (2 x f32)
            //   +0x178: renderSize (2 x u32)
            //   +0x180: upscaleSize (2 x u32)
            //   +0x194: preExposure (f32)
            //   +0x198: reset (bool/u8)
            let cmd_list = std::ptr::read_unaligned((base + 0x10) as *const *mut c_void);
            let color_in = std::ptr::read_unaligned((base + 0x18) as *const *mut c_void);
            let depth_in = std::ptr::read_unaligned((base + 0x48) as *const *mut c_void);
            let motion_in = std::ptr::read_unaligned((base + 0x78) as *const *mut c_void);
            let exposure_in = std::ptr::read_unaligned((base + 0xA8) as *const *mut c_void);
            let reactive_in = std::ptr::read_unaligned((base + 0xD8) as *const *mut c_void);
            let color_out = std::ptr::read_unaligned((base + 0x138) as *const *mut c_void);
            let jitter_offset = std::ptr::read_unaligned((base + 0x168) as *const [f32; 2]);
            let mut motion_vector_scale =
                std::ptr::read_unaligned((base + 0x170) as *const [f32; 2]);
            let render_size = std::ptr::read_unaligned((base + 0x178) as *const [u32; 2]);
            let mut upscale_size = std::ptr::read_unaligned((base + 0x180) as *const [u32; 2]);
            if upscale_size == [0, 0] {
                upscale_size = ctx.max_upscale_size;
            }
            let raw_pre_exp = std::ptr::read_unaligned((base + 0x194) as *const f32);
            let pre_exposure = if raw_pre_exp.is_finite() && raw_pre_exp > 0.0 {
                raw_pre_exp
            } else {
                1.0
            };
            let desc_reset = std::ptr::read_unaligned((base + 0x198) as *const u8) != 0;
            if !motion_vector_scale[0].is_finite() || !motion_vector_scale[1].is_finite() {
                motion_vector_scale = [1.0, 1.0];
            }

            if !cmd_list.is_null() && !color_in.is_null() && !color_out.is_null() {
                let use_temporal = crate::d3d12::cmdlist::temporal_upscale_enabled()
                    && !depth_in.is_null()
                    && !motion_in.is_null();
                let is_hdr = (ctx.upscale_flags & FFX_UPSCALE_ENABLE_HIGH_DYNAMIC_RANGE) != 0;
                let depth_reversed = (ctx.upscale_flags & FFX_UPSCALE_ENABLE_DEPTH_INVERTED) != 0;
                let dynamic_resolution =
                    (ctx.upscale_flags & FFX_UPSCALE_ENABLE_DYNAMIC_RESOLUTION) != 0;
                let output_res_mv = (ctx.upscale_flags
                    & FFX_UPSCALE_ENABLE_DISPLAY_RESOLUTION_MOTION_VECTORS)
                    != 0;
                let jittered_mv = (ctx.upscale_flags
                    & FFX_UPSCALE_ENABLE_MOTION_VECTORS_JITTER_CANCELLATION)
                    != 0;
                // Enable MetalFX auto-exposure if requested by the game OR when an HDR buffer
                // arrives without an exposure texture and without a non-unit pre-exposure
                // (prevents MetalFX manual-exposure tone saturation > 1023 on pre-exposed HDR).
                let auto_exposure = (ctx.upscale_flags & FFX_UPSCALE_ENABLE_AUTO_EXPOSURE) != 0
                    || (is_hdr && exposure_in.is_null() && (pre_exposure - 1.0).abs() < 1e-4);
                let reset = desc_reset || ctx.first_dispatch;
                ctx.first_dispatch = false;

                if !ctx.logged_first_dispatch {
                    ctx.logged_first_dispatch = true;
                    log::info!(
                        target: "d3d",
                        "ffxDispatch(UPSCALE): temporal={use_temporal} render={}x{} upscale={}x{} jitter=({:.3},{:.3}) mv_scale=({:.2},{:.2}) pre_exp={:.3} auto_exp={auto_exposure} depth_rev={depth_reversed} drs={dynamic_resolution} out_res_mv={output_res_mv} jit_mv={jittered_mv} depth={} mv={} exp={} reactive={}",
                        render_size[0],
                        render_size[1],
                        upscale_size[0],
                        upscale_size[1],
                        jitter_offset[0],
                        jitter_offset[1],
                        motion_vector_scale[0],
                        motion_vector_scale[1],
                        pre_exposure,
                        !depth_in.is_null(),
                        !motion_in.is_null(),
                        !exposure_in.is_null(),
                        !reactive_in.is_null(),
                    );
                }

                if use_temporal {
                    crate::d3d12::cmdlist::upscale_temporal(
                        cmd_list,
                        crate::d3d12::cmdlist::TemporalUpscaleDesc {
                            output: color_out,
                            color: color_in,
                            depth: depth_in,
                            motion: motion_in,
                            exposure: exposure_in,
                            reactive_mask: reactive_in,
                            render_size,
                            upscale_size,
                            jitter_offset,
                            motion_vector_scale,
                            reset,
                            depth_reversed,
                            pre_exposure,
                            auto_exposure,
                            dynamic_resolution,
                            output_resolution_motion_vectors: output_res_mv,
                            jittered_motion_vectors: jittered_mv,
                        },
                    );
                } else {
                    crate::d3d12::cmdlist::upscale_resources(
                        cmd_list,
                        color_out,
                        color_in,
                        render_size,
                        upscale_size,
                    );
                }
            }
            FFX_API_RETURN_OK
        }
        FFX_API_DISPATCH_DESC_TYPE_UPSCALE_GENERATEREACTIVEMASK => {
            let desc = std::ptr::read_unaligned(
                header as *const FfxDispatchDescUpscaleGenerateReactiveMask,
            );
            let cleared = clear_reactive_mask_to_zero(desc.command_list, desc.out_reactive.resource);
            if !ctx.logged_gen_reactive {
                ctx.logged_gen_reactive = true;
                crate::stub!(
                    "ffxDispatch(GENERATEREACTIVEMASK): cleared outReactive to 0.0 (cleared={cleared}, render={}x{}, scale={:.3}, cutoff={:.3}, binary={:.3}, flags={:#x}, opaque={}, pre_upscale={})",
                    desc.render_size[0],
                    desc.render_size[1],
                    desc.scale,
                    desc.cutoff_threshold,
                    desc.binary_value,
                    desc.flags,
                    !desc.color_opaque_only.resource.is_null(),
                    !desc.color_pre_upscale.resource.is_null(),
                );
            }
            FFX_API_RETURN_OK
        }
        FFX_API_DISPATCH_DESC_TYPE_FRAMEGENERATIONSWAPCHAIN_WAIT_FOR_PRESENTS_DX12 => {
            FFX_API_RETURN_OK
        }
        _ if (ty & FFX_API_EFFECT_MASK) == FFX_API_EFFECT_ID_FRAMEGENERATION => {
            crate::stub!(
                "ffxDispatch(ty={ty:#x}): frame generation is not supported -> FFX_API_RETURN_NO_PROVIDER"
            );
            FFX_API_RETURN_NO_PROVIDER
        }
        _ => FFX_API_RETURN_NO_PROVIDER,
    }
}

fn quality_mode_ratio(mode: u32) -> f32 {
    match mode {
        0 => 1.0,
        1 => 1.5,
        2 => 1.7,
        3 => 2.0,
        4 => 3.0,
        _ => 1.0,
    }
}

pub unsafe extern "win64" fn ffxQuery(context: *mut usize, header: *mut FfxApiHeader) -> u32 {
    if header.is_null() {
        return FFX_API_RETURN_ERROR_PARAMETER;
    }
    let ty = std::ptr::read_unaligned(&(*header).ty);
    let base = header as usize;
    match ty {
        FFX_API_QUERY_DESC_TYPE_GET_VERSIONS => {
            // struct ffxQueryDescGetVersions:
            //   +0x00: header (16B)
            //   +0x10: uint64_t createDescType
            //   +0x18: void* device
            //   +0x20: uint64_t* outputCount
            //   +0x28: uint64_t* versionIds
            //   +0x30: const char** versionNames
            let create_desc_type = std::ptr::read_unaligned((base + 0x10) as *const u64);
            let out_count = std::ptr::read_unaligned((base + 0x20) as *const *mut u64);
            let version_ids = std::ptr::read_unaligned((base + 0x28) as *const *mut u64);
            let version_names = std::ptr::read_unaligned((base + 0x30) as *const *mut *const u8);
            if (create_desc_type & FFX_API_EFFECT_MASK) == FFX_API_EFFECT_ID_FRAMEGENERATION {
                if !out_count.is_null() {
                    std::ptr::write_unaligned(out_count, 0);
                }
                return FFX_API_RETURN_NO_PROVIDER;
            }
            if !out_count.is_null() {
                let cap = std::ptr::read_unaligned(out_count);
                if !version_ids.is_null() && cap >= 1 {
                    std::ptr::write_unaligned(version_ids, 0x0003_0001_0004);
                }
                if !version_names.is_null() && cap >= 1 {
                    std::ptr::write_unaligned(version_names, b"3.1.4\0".as_ptr());
                }
                std::ptr::write_unaligned(out_count, 1);
            }
            return FFX_API_RETURN_OK;
        }
        FFX_API_QUERY_DESC_TYPE_FRAMEGENERATIONSWAPCHAIN_INTERPOLATIONCOMMANDLIST_DX12 => {
            let out_cmd = std::ptr::read_unaligned((base + 0x10) as *const *mut usize);
            if !out_cmd.is_null() {
                std::ptr::write_unaligned(out_cmd, 0);
            }
            crate::stub!(
                "ffxQuery(INTERPOLATIONCOMMANDLIST_DX12): frame generation is not supported -> FFX_API_RETURN_NO_PROVIDER"
            );
            return FFX_API_RETURN_NO_PROVIDER;
        }
        FFX_API_QUERY_DESC_TYPE_FRAMEGENERATIONSWAPCHAIN_INTERPOLATIONTEXTURE_DX12 => {
            let out_tex = std::ptr::read_unaligned((base + 0x10) as *const *mut [u64; 6]);
            if !out_tex.is_null() {
                std::ptr::write_unaligned(out_tex, [0; 6]);
            }
            crate::stub!(
                "ffxQuery(INTERPOLATIONTEXTURE_DX12): frame generation is not supported -> FFX_API_RETURN_NO_PROVIDER"
            );
            return FFX_API_RETURN_NO_PROVIDER;
        }
        FFX_API_QUERY_DESC_TYPE_FRAMEGENERATIONSWAPCHAIN_GPU_MEMORY_USAGE_DX12
        | FFX_API_QUERY_DESC_TYPE_UPSCALE_GPU_MEMORY_USAGE => {
            let out_mem = std::ptr::read_unaligned((base + 0x10) as *const *mut [u64; 2]);
            if !out_mem.is_null() {
                std::ptr::write_unaligned(out_mem, [0, 0]);
            }
            return FFX_API_RETURN_OK;
        }
        FFX_API_QUERY_DESC_TYPE_FRAMEGENERATIONSWAPCHAIN_GPU_MEMORY_USAGE_DX12_V2 => {
            let out_mem = std::ptr::read_unaligned((base + 0x38) as *const *mut [u64; 2]);
            if !out_mem.is_null() {
                std::ptr::write_unaligned(out_mem, [0, 0]);
            }
            return FFX_API_RETURN_OK;
        }
        FFX_API_QUERY_DESC_TYPE_UPSCALE_GETUPSCALERATIOFROMQUALITYMODE => {
            let mode = std::ptr::read_unaligned((base + 0x10) as *const u32);
            let out_ratio = std::ptr::read_unaligned((base + 0x18) as *const *mut f32);
            if !out_ratio.is_null() {
                std::ptr::write_unaligned(out_ratio, quality_mode_ratio(mode));
            }
            return FFX_API_RETURN_OK;
        }
        FFX_API_QUERY_DESC_TYPE_UPSCALE_GETRENDERRESOLUTIONFROMQUALITYMODE => {
            let disp_w = std::ptr::read_unaligned((base + 0x10) as *const u32);
            let disp_h = std::ptr::read_unaligned((base + 0x14) as *const u32);
            let mode = std::ptr::read_unaligned((base + 0x18) as *const u32);
            let out_w = std::ptr::read_unaligned((base + 0x20) as *const *mut u32);
            let out_h = std::ptr::read_unaligned((base + 0x28) as *const *mut u32);
            let ratio = quality_mode_ratio(mode);
            if !out_w.is_null() {
                std::ptr::write_unaligned(out_w, (disp_w as f32 / ratio) as u32);
            }
            if !out_h.is_null() {
                std::ptr::write_unaligned(out_h, (disp_h as f32 / ratio) as u32);
            }
            return FFX_API_RETURN_OK;
        }
        FFX_API_QUERY_DESC_TYPE_UPSCALE_GETJITTERPHASECOUNT => {
            // struct ffxQueryDescUpscaleGetJitterPhaseCount:
            //   +0x10: uint32_t renderWidth
            //   +0x14: uint32_t displayWidth
            //   +0x18: int32_t* pOutPhaseCount
            let render_w = std::ptr::read_unaligned((base + 0x10) as *const u32);
            let display_w = std::ptr::read_unaligned((base + 0x14) as *const u32);
            let out_phases = std::ptr::read_unaligned((base + 0x18) as *const *mut i32);
            if !out_phases.is_null() {
                let phases = if crate::d3d12::cmdlist::temporal_upscale_enabled() {
                    compute_jitter_phase_count(render_w, display_w)
                } else {
                    1
                };
                std::ptr::write_unaligned(out_phases, phases);
            }
            return FFX_API_RETURN_OK;
        }
        FFX_API_QUERY_DESC_TYPE_UPSCALE_GETJITTEROFFSET => {
            // struct ffxQueryDescUpscaleGetJitterOffset:
            //   +0x10: int32_t index
            //   +0x14: int32_t phaseCount
            //   +0x18: float* pOutX
            //   +0x20: float* pOutY
            let index = std::ptr::read_unaligned((base + 0x10) as *const i32);
            let phase_count = std::ptr::read_unaligned((base + 0x14) as *const i32);
            let out_x = std::ptr::read_unaligned((base + 0x18) as *const *mut f32);
            let out_y = std::ptr::read_unaligned((base + 0x20) as *const *mut f32);
            let (jx, jy) = if crate::d3d12::cmdlist::temporal_upscale_enabled() {
                compute_jitter_offset(index, phase_count)
            } else {
                (0.0, 0.0)
            };
            if !out_x.is_null() {
                std::ptr::write_unaligned(out_x, jx);
            }
            if !out_y.is_null() {
                std::ptr::write_unaligned(out_y, jy);
            }
            return FFX_API_RETURN_OK;
        }
        FFX_API_QUERY_DESC_TYPE_UPSCALE_GPU_MEMORY_USAGE_V2 => {
            let out_mem = std::ptr::read_unaligned((base + 0x30) as *const *mut [u64; 2]);
            if !out_mem.is_null() {
                std::ptr::write_unaligned(out_mem, [0, 0]);
            }
            return FFX_API_RETURN_OK;
        }
        FFX_API_QUERY_DESC_TYPE_UPSCALE_GET_RESOURCE_REQUIREMENTS => {
            std::ptr::write_unaligned((base + 0x10) as *mut u64, 0x0f);
            std::ptr::write_unaligned((base + 0x18) as *mut u64, 0x30);
            return FFX_API_RETURN_OK;
        }
        FFX_API_QUERY_DESC_TYPE_GET_PROVIDER_VERSION
            if !context.is_null() && std::ptr::read_unaligned(context) != 0 =>
        {
            std::ptr::write_unaligned((base + 0x10) as *mut u64, 0x0003_0001_0004);
            std::ptr::write_unaligned((base + 0x18) as *mut *const u8, b"3.1.4\0".as_ptr());
            return FFX_API_RETURN_OK;
        }
        _ => {}
    }
    FFX_API_RETURN_NO_PROVIDER
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[test]
    fn ffx_upscale_lifecycle_and_queries() {
        unsafe {
            // Negative cases: null pointers
            assert_eq!(
                ffxCreateContext(std::ptr::null_mut(), std::ptr::null(), 0),
                FFX_API_RETURN_ERROR_PARAMETER
            );
            assert_eq!(
                ffxDestroyContext(std::ptr::null_mut(), 0),
                FFX_API_RETURN_ERROR_PARAMETER
            );
            assert_eq!(
                ffxConfigure(std::ptr::null_mut(), std::ptr::null()),
                FFX_API_RETURN_ERROR_PARAMETER
            );
            assert_eq!(
                ffxDispatch(std::ptr::null_mut(), std::ptr::null()),
                FFX_API_RETURN_ERROR_PARAMETER
            );
            assert_eq!(
                ffxQuery(std::ptr::null_mut(), std::ptr::null_mut()),
                FFX_API_RETURN_ERROR_PARAMETER
            );

            // Positive case: create upscale context (0x10000) using full ffxCreateContextDescUpscale layout
            let mut ctx: usize = 0;
            let mut create_desc = [0u64; 6];
            create_desc[0] = FFX_API_CREATE_CONTEXT_DESC_TYPE_UPSCALE;
            let create_bytes = create_desc.as_mut_ptr() as *mut u8;
            let flags = FFX_UPSCALE_ENABLE_HIGH_DYNAMIC_RANGE
                | FFX_UPSCALE_ENABLE_DEPTH_INVERTED
                | FFX_UPSCALE_ENABLE_AUTO_EXPOSURE;
            std::ptr::write_unaligned(create_bytes.add(0x10) as *mut u32, flags);
            std::ptr::write_unaligned(create_bytes.add(0x14) as *mut [u32; 2], [1280, 720]);
            std::ptr::write_unaligned(create_bytes.add(0x1C) as *mut [u32; 2], [1920, 1080]);
            assert_eq!(
                ffxCreateContext(&mut ctx, create_desc.as_ptr() as *const FfxApiHeader, 0),
                FFX_API_RETURN_OK
            );
            assert_ne!(ctx, 0);

            // Configure and dispatch on upscale context
            let cfg_hdr = FfxApiHeader {
                ty: FFX_API_CONFIGURE_DESC_TYPE_UPSCALE_KEYVALUE,
                next: std::ptr::null_mut(),
            };
            assert_eq!(ffxConfigure(&mut ctx, &cfg_hdr), FFX_API_RETURN_OK);

            let mut disp_buf = [0u64; 54];
            disp_buf[0] = FFX_API_DISPATCH_DESC_TYPE_UPSCALE;
            assert_eq!(
                ffxDispatch(&mut ctx, disp_buf.as_ptr() as *const FfxApiHeader),
                FFX_API_RETURN_OK
            );

            // Query jitter phase count and Halton jitter sequence
            assert_eq!(compute_jitter_phase_count(1920, 1920), 8);
            assert_eq!(compute_jitter_phase_count(1280, 1920), 18);
            assert_eq!(compute_jitter_phase_count(960, 1920), 32);
            let (j0x, j0y) = compute_jitter_offset(0, 8);
            assert!((j0x - 0.0).abs() < 1e-6 && (j0y - (1.0 / 3.0 - 0.5)).abs() < 1e-6);
            let (j1x, j1y) = compute_jitter_offset(1, 8);
            assert!((j1x - (-0.25)).abs() < 1e-6 && (j1y - (2.0 / 3.0 - 0.5)).abs() < 1e-6);
            assert_eq!(compute_jitter_offset(8, 8), compute_jitter_offset(0, 8));
            assert_eq!(compute_jitter_offset(-1, 8), compute_jitter_offset(7, 8));

            let mut phases: i32 = 0;
            let mut q_phase = [0u64; 4];
            q_phase[0] = FFX_API_QUERY_DESC_TYPE_UPSCALE_GETJITTERPHASECOUNT;
            q_phase[2] = (1920u64 << 32) | 1280u64;
            q_phase[3] = &mut phases as *mut i32 as usize as u64;
            assert_eq!(
                ffxQuery(&mut ctx, q_phase.as_mut_ptr() as *mut FfxApiHeader),
                FFX_API_RETURN_OK
            );
            if crate::d3d12::cmdlist::temporal_upscale_enabled() {
                assert_eq!(phases, 18);
            } else {
                assert_eq!(phases, 1);
            }

            let mut jx: f32 = 99.0;
            let mut jy: f32 = 99.0;
            let mut q_jit = [0u64; 5];
            q_jit[0] = FFX_API_QUERY_DESC_TYPE_UPSCALE_GETJITTEROFFSET;
            q_jit[2] = ((phases as u32 as u64) << 32) | 1u64;
            q_jit[3] = &mut jx as *mut f32 as usize as u64;
            q_jit[4] = &mut jy as *mut f32 as usize as u64;
            assert_eq!(
                ffxQuery(&mut ctx, q_jit.as_mut_ptr() as *mut FfxApiHeader),
                FFX_API_RETURN_OK
            );
            assert!((-0.5..=0.5).contains(&jx) && (-0.5..=0.5).contains(&jy));

            // Query upscale ratio and render resolution from quality mode
            let mut ratio: f32 = 0.0;
            let mut q_ratio = [0u64; 4];
            q_ratio[0] = FFX_API_QUERY_DESC_TYPE_UPSCALE_GETUPSCALERATIOFROMQUALITYMODE;
            q_ratio[2] = 1; // Quality mode = 1 (1.5x)
            q_ratio[3] = &mut ratio as *mut f32 as usize as u64;
            assert_eq!(
                ffxQuery(
                    std::ptr::null_mut(),
                    q_ratio.as_mut_ptr() as *mut FfxApiHeader
                ),
                FFX_API_RETURN_OK
            );
            assert!((ratio - 1.5).abs() < 1e-6);

            let mut out_w: u32 = 0;
            let mut out_h: u32 = 0;
            let mut q_res = [0u64; 6];
            q_res[0] = FFX_API_QUERY_DESC_TYPE_UPSCALE_GETRENDERRESOLUTIONFROMQUALITYMODE;
            q_res[2] = (1080u64 << 32) | 1920u64;
            q_res[3] = 1; // Quality mode = 1 (1.5x)
            q_res[4] = &mut out_w as *mut u32 as usize as u64;
            q_res[5] = &mut out_h as *mut u32 as usize as u64;
            assert_eq!(
                ffxQuery(
                    std::ptr::null_mut(),
                    q_res.as_mut_ptr() as *mut FfxApiHeader
                ),
                FFX_API_RETURN_OK
            );
            assert_eq!((out_w, out_h), (1280, 720));

            // Teardown: destroy context and verify double-destroy fails with ERROR_PARAMETER
            assert_eq!(ffxDestroyContext(&mut ctx, 0), FFX_API_RETURN_OK);
            assert_eq!(ctx, 0);
            assert_eq!(
                ffxDestroyContext(&mut ctx, 0),
                FFX_API_RETURN_ERROR_PARAMETER
            );
        }
    }

    #[test]
    fn ffx_honestly_refuses_frame_generation_while_keeping_swapchain_wrap() {
        unsafe {
            // FrameGeneration context creation must return NO_PROVIDER and leave ctx == 0.
            let mut fg_ctx = 0xDEADusize;
            let fg_create = FfxApiHeader {
                ty: FFX_API_CREATE_CONTEXT_DESC_TYPE_FRAMEGENERATION,
                next: std::ptr::null_mut(),
            };
            assert_eq!(
                ffxCreateContext(&mut fg_ctx, &fg_create, 0),
                FFX_API_RETURN_NO_PROVIDER
            );
            assert_eq!(fg_ctx, 0);

            // Version query for FrameGeneration returns NO_PROVIDER with count 0.
            let mut count: u64 = 4;
            let mut q_ver = [0u64; 7];
            q_ver[0] = FFX_API_QUERY_DESC_TYPE_GET_VERSIONS;
            q_ver[2] = FFX_API_CREATE_CONTEXT_DESC_TYPE_FRAMEGENERATION;
            q_ver[4] = &mut count as *mut u64 as usize as u64;
            assert_eq!(
                ffxQuery(std::ptr::null_mut(), q_ver.as_mut_ptr() as *mut FfxApiHeader),
                FFX_API_RETURN_NO_PROVIDER
            );
            assert_eq!(count, 0);

            // Version query for Upscale still returns OK with count 1.
            count = 1;
            let mut ver_id = 0u64;
            let mut ver_name: *const u8 = std::ptr::null();
            q_ver[2] = FFX_API_CREATE_CONTEXT_DESC_TYPE_UPSCALE;
            q_ver[5] = &mut ver_id as *mut u64 as usize as u64;
            q_ver[6] = &mut ver_name as *mut *const u8 as usize as u64;
            assert_eq!(
                ffxQuery(std::ptr::null_mut(), q_ver.as_mut_ptr() as *mut FfxApiHeader),
                FFX_API_RETURN_OK
            );
            assert_eq!(count, 1);
            assert_eq!(ver_id, 0x0003_0001_0004);
            assert!(!ver_name.is_null());

            // Interpolation command list & texture queries return NO_PROVIDER and zero outputs.
            let mut out_cmd = 0xCAFEusize;
            let mut q_cmd = [0u64; 3];
            q_cmd[0] = FFX_API_QUERY_DESC_TYPE_FRAMEGENERATIONSWAPCHAIN_INTERPOLATIONCOMMANDLIST_DX12;
            q_cmd[2] = &mut out_cmd as *mut usize as usize as u64;
            assert_eq!(
                ffxQuery(std::ptr::null_mut(), q_cmd.as_mut_ptr() as *mut FfxApiHeader),
                FFX_API_RETURN_NO_PROVIDER
            );
            assert_eq!(out_cmd, 0);

            let mut out_tex = [0xAAu64; 6];
            let mut q_tex = [0u64; 3];
            q_tex[0] = FFX_API_QUERY_DESC_TYPE_FRAMEGENERATIONSWAPCHAIN_INTERPOLATIONTEXTURE_DX12;
            q_tex[2] = &mut out_tex as *mut [u64; 6] as usize as u64;
            assert_eq!(
                ffxQuery(std::ptr::null_mut(), q_tex.as_mut_ptr() as *mut FfxApiHeader),
                FFX_API_RETURN_NO_PROVIDER
            );
            assert_eq!(out_tex, [0; 6]);

            // Swapchain wrap context (0x30001) still works cleanly with AddRef/Release.
            #[repr(C)]
            struct MockSc {
                #[allow(dead_code)]
                vtbl: *const [usize; 3],
                refs: AtomicU32,
            }
            unsafe extern "win64" fn mock_qi(_: *mut MockSc) -> i32 {
                0
            }
            unsafe extern "win64" fn mock_add_ref(this: *mut MockSc) -> u32 {
                (*this).refs.fetch_add(1, Ordering::AcqRel) + 1
            }
            unsafe extern "win64" fn mock_release(this: *mut MockSc) -> u32 {
                (*this).refs.fetch_sub(1, Ordering::AcqRel) - 1
            }
            let vtbl = [
                mock_qi as *const () as usize,
                mock_add_ref as *const () as usize,
                mock_release as *const () as usize,
            ];
            let mut sc = MockSc {
                vtbl: &vtbl,
                refs: AtomicU32::new(1),
            };
            let mut sc_ptr = &mut sc as *mut MockSc as usize;
            let mut wrap_desc = [0u64; 4];
            wrap_desc[0] = FFX_API_CREATE_CONTEXT_DESC_TYPE_FRAMEGENERATIONSWAPCHAIN_WRAP_DX12;
            wrap_desc[2] = &mut sc_ptr as *mut usize as usize as u64;

            let mut sw_ctx = 0usize;
            assert_eq!(
                ffxCreateContext(&mut sw_ctx, wrap_desc.as_ptr() as *const FfxApiHeader, 0),
                FFX_API_RETURN_OK
            );
            assert_ne!(sw_ctx, 0);
            assert_eq!(sc.refs.load(Ordering::Acquire), 2);

            // Any 0x0002_xxxx FG configure or dispatch against a context returns NO_PROVIDER.
            let fg_cfg = FfxApiHeader {
                ty: 0x0002_0002,
                next: std::ptr::null_mut(),
            };
            assert_eq!(ffxConfigure(&mut sw_ctx, &fg_cfg), FFX_API_RETURN_NO_PROVIDER);
            let fg_disp = FfxApiHeader {
                ty: 0x0002_0003,
                next: std::ptr::null_mut(),
            };
            assert_eq!(ffxDispatch(&mut sw_ctx, &fg_disp), FFX_API_RETURN_NO_PROVIDER);

            assert_eq!(ffxDestroyContext(&mut sw_ctx, 0), FFX_API_RETURN_OK);
            assert_eq!(sc.refs.load(Ordering::Acquire), 1);
        }
    }

    #[test]
    fn ffx_generate_reactive_mask_clears_texture_cleanly() {
        unsafe {
            let mut ctx = 0usize;
            let mut create_desc = [0u64; 6];
            create_desc[0] = FFX_API_CREATE_CONTEXT_DESC_TYPE_UPSCALE;
            assert_eq!(
                ffxCreateContext(&mut ctx, create_desc.as_ptr() as *const FfxApiHeader, 0),
                FFX_API_RETURN_OK
            );

            // Null/empty reactive mask dispatch succeeds without faulting.
            let empty_res = FfxApiResource {
                resource: std::ptr::null_mut(),
                description: [0; 8],
                state: 0,
            };
            let mut desc = FfxDispatchDescUpscaleGenerateReactiveMask {
                header: FfxApiHeader {
                    ty: FFX_API_DISPATCH_DESC_TYPE_UPSCALE_GENERATEREACTIVEMASK,
                    next: std::ptr::null_mut(),
                },
                command_list: std::ptr::null_mut(),
                color_opaque_only: empty_res,
                color_pre_upscale: empty_res,
                out_reactive: empty_res,
                render_size: [16, 16],
                scale: 1.0,
                cutoff_threshold: 0.2,
                binary_value: 0.9,
                flags: 0,
            };
            assert_eq!(
                ffxDispatch(&mut ctx, &desc as *const _ as *const FfxApiHeader),
                FFX_API_RETURN_OK
            );

            // When a Metal 4 device is available, exercise real D3D12 UAV clear of outReactive.
            if crate::d3d12::mtl::device().is_some() {
                let mut alloc: *mut c_void = std::ptr::null_mut();
                let mut list: *mut c_void = std::ptr::null_mut();
                let mut tex: *mut c_void = std::ptr::null_mut();
                assert_eq!(
                    crate::d3d12::queue::create_allocator(
                        0,
                        &crate::d3d12::vtables::IID_ID3D12CommandAllocator,
                        &mut alloc
                    ),
                    crate::types::S_OK
                );
                assert_eq!(
                    crate::d3d12::cmdlist::create(
                        0,
                        alloc,
                        std::ptr::null_mut(),
                        &crate::d3d12::vtables::IID_ID3D12GraphicsCommandList,
                        &mut list
                    ),
                    crate::types::S_OK
                );
                let hp = d3d12_metal::types::HeapProperties {
                    Type: d3d12_metal::types::heap_type::DEFAULT,
                    ..Default::default()
                };
                let rd = d3d12_metal::types::ResourceDesc {
                    Dimension: d3d12_metal::types::dim::TEXTURE2D,
                    Width: 16,
                    Height: 16,
                    DepthOrArraySize: 1,
                    MipLevels: 1,
                    Format: 61, // DXGI_FORMAT_R8_UNORM
                    SampleCount: 1,
                    Flags: d3d12_metal::types::resource_flags::ALLOW_UNORDERED_ACCESS,
                    ..Default::default()
                };
                assert_eq!(
                    crate::d3d12::resource::create_committed(
                        &hp,
                        0,
                        &rd,
                        0,
                        std::ptr::null(),
                        &crate::d3d12::vtables::IID_ID3D12Resource,
                        &mut tex
                    ),
                    crate::types::S_OK
                );
                desc.command_list = list;
                desc.out_reactive.resource = tex;
                assert!(clear_reactive_mask_to_zero(list, tex));
                assert_eq!(
                    ffxDispatch(&mut ctx, &desc as *const _ as *const FfxApiHeader),
                    FFX_API_RETURN_OK
                );
                let vtbl = std::ptr::read_unaligned(list as *const *const usize);
                // ID3D12GraphicsCommandList::Close is vtable slot 9.
                let close_fn: unsafe extern "win64" fn(*mut c_void) -> crate::types::HRESULT =
                    std::mem::transmute(std::ptr::read_unaligned(vtbl.add(9)));
                assert_eq!(close_fn(list), crate::types::S_OK);
                crate::d3d12::cs::flush();
                crate::d3d12::com::com_release(tex);
                crate::d3d12::com::com_release(list);
                crate::d3d12::com::com_release(alloc);
            }

            assert_eq!(ffxDestroyContext(&mut ctx, 0), FFX_API_RETURN_OK);
        }
    }
}
