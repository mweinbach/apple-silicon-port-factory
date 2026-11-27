//! Universal Direct3D 11.0 / 11.1 COM runtime backed by `d3d12-metal`,
//! `dxil-metal` (SM4/5 FXC DXBC -> DXIL via `libdxilconv` -> Metal 4 AIR),
//! and `win32-rt`'s DXGI layer.
//!
//! Set `PORT_D3D11=0` to explicitly disable D3D11 (`DXGI_ERROR_UNSUPPORTED`).

pub mod com;
pub mod context;
pub mod device;
pub mod resources;
pub mod state;
pub mod swapchain;
pub mod types;

use std::ffi::c_void;
use std::sync::Arc;

use d3d12_metal::metal::queue::{ListType, Queue};

use crate::dxgi::swapchain::{FullscreenDesc, SwapChainDesc, SwapChainDesc1};
use crate::types::*;

use com::*;
use device::D3d11Device;
use types::*;

pub static EXPORTS: &[crate::Export] =
    exports![D3D11CreateDevice, D3D11CreateDeviceAndSwapChain, D3D11On12CreateDevice];

fn d3d11_disabled() -> bool {
    std::env::var("PORT_D3D11").is_ok_and(|v| v == "0")
}

unsafe fn null_out(p: *mut *mut c_void) {
    if !p.is_null() {
        std::ptr::write_unaligned(p, std::ptr::null_mut());
    }
}

fn is_supported_feature_level(fl: u32) -> bool {
    matches!(
        fl,
        D3D_FEATURE_LEVEL_11_1
            | D3D_FEATURE_LEVEL_11_0
            | D3D_FEATURE_LEVEL_10_1
            | D3D_FEATURE_LEVEL_10_0
            | D3D_FEATURE_LEVEL_9_3
            | D3D_FEATURE_LEVEL_9_2
            | D3D_FEATURE_LEVEL_9_1
    )
}

unsafe fn negotiate_feature_level(levels: *const u32, n_levels: u32) -> Result<u32, HRESULT> {
    if levels.is_null() || n_levels == 0 {
        return Ok(D3D_FEATURE_LEVEL_11_1);
    }
    let mut req = Vec::with_capacity(n_levels as usize);
    for i in 0..n_levels as usize {
        let fl = std::ptr::read_unaligned(levels.add(i));
        if is_supported_feature_level(fl) {
            return Ok(fl);
        }
        req.push(fl);
    }
    Err(d11_failed!(
        E_INVALIDARG,
        "D3D11CreateDevice: none of requested feature levels {req:#x?} supported"
    ))
}

unsafe fn create_device_internal(
    adapter: *mut c_void,
    driver_type: u32,
    flags: u32,
    levels: *const u32,
    n_levels: u32,
    queue_override: Option<Arc<Queue>>,
    out_device: *mut *mut c_void,
    out_level: *mut u32,
    out_ctx: *mut *mut c_void,
) -> Result<(Arc<device::DeviceCore>, *mut Obj<D3d11Device>, *mut Obj<context::D3d11Context>), HRESULT> {
    register_names();
    if d3d11_disabled() {
        log::info!(target: "d3d", "D3D11CreateDevice: disabled by PORT_D3D11=0 -> DXGI_ERROR_UNSUPPORTED");
        return Err(DXGI_ERROR_UNSUPPORTED);
    }
    if driver_type > driver_type::WARP {
        return Err(d11_failed!(E_INVALIDARG, "D3D11CreateDevice: invalid driver_type {driver_type}"));
    }
    if !adapter.is_null() && driver_type != driver_type::UNKNOWN {
        log::debug!(
            target: "d3d",
            "D3D11CreateDevice: non-null adapter with driver_type {driver_type}; proceeding on default Metal adapter"
        );
    }
    let chosen_level = negotiate_feature_level(levels, n_levels)?;
    let Some(mtl) = d3d12_metal::metal::device() else {
        return Err(d11_failed!(DXGI_ERROR_UNSUPPORTED, "D3D11CreateDevice: no Metal 4 device"));
    };
    if !out_level.is_null() {
        std::ptr::write_unaligned(out_level, chosen_level);
    }
    let queue = match queue_override {
        Some(q) => q,
        None => match Queue::new(ListType::Direct, "D3D11 Direct queue") {
            Some(q) => Arc::new(q),
            None => return Err(d11_failed!(E_OUTOFMEMORY, "D3D11CreateDevice: failed to create Metal 4 queue")),
        },
    };
    let (dev, imm) = D3d11Device::create_pair(mtl, queue, chosen_level, flags)?;
    let core = Obj::get(dev).core.clone();

    if !out_device.is_null() {
        std::ptr::write_unaligned(out_device, dev as *mut c_void);
    }
    if !out_ctx.is_null() {
        Obj::add_ref(imm);
        std::ptr::write_unaligned(out_ctx, imm as *mut c_void);
    }
    log::info!(
        target: "d3d",
        "D3D11CreateDevice(driver {driver_type}, flags {flags:#x}) -> FL {chosen_level:#x} (dev={dev:?}, ctx={imm:?})"
    );
    Ok((core, dev, imm))
}

pub unsafe extern "win64" fn D3D11CreateDevice(
    adapter: *mut c_void,
    driver_type: u32,
    _software: *mut c_void,
    flags: u32,
    levels: *const u32,
    n_levels: u32,
    _sdk: u32,
    out_device: *mut *mut c_void,
    out_level: *mut u32,
    out_ctx: *mut *mut c_void,
) -> HRESULT {
    null_out(out_device);
    null_out(out_ctx);
    if !out_level.is_null() {
        std::ptr::write_unaligned(out_level, 0);
    }
    match create_device_internal(adapter, driver_type, flags, levels, n_levels, None, out_device, out_level, out_ctx) {
        Ok((_, dev, _)) => {
            if out_device.is_null() {
                Obj::release(dev);
            }
            S_OK
        }
        Err(hr) => hr,
    }
}

pub unsafe extern "win64" fn D3D11CreateDeviceAndSwapChain(
    adapter: *mut c_void,
    driver_type: u32,
    _software: *mut c_void,
    flags: u32,
    levels: *const u32,
    n_levels: u32,
    _sdk: u32,
    desc: *const c_void,
    out_swapchain: *mut *mut c_void,
    out_device: *mut *mut c_void,
    out_level: *mut u32,
    out_ctx: *mut *mut c_void,
) -> HRESULT {
    null_out(out_swapchain);
    null_out(out_device);
    null_out(out_ctx);
    if !out_level.is_null() {
        std::ptr::write_unaligned(out_level, 0);
    }
    let (core, dev, imm) = match create_device_internal(
        adapter,
        driver_type,
        flags,
        levels,
        n_levels,
        None,
        out_device,
        out_level,
        out_ctx,
    ) {
        Ok(x) => x,
        Err(hr) => return hr,
    };

    if !desc.is_null() {
        if out_swapchain.is_null() {
            if !out_ctx.is_null() {
                Obj::release(imm);
                std::ptr::write_unaligned(out_ctx, std::ptr::null_mut());
            }
            Obj::release(dev);
            null_out(out_device);
            return d11_failed!(E_INVALIDARG, "D3D11CreateDeviceAndSwapChain: pSwapChainDesc without ppSwapChain");
        }
        let d = std::ptr::read_unaligned(desc as *const SwapChainDesc);
        let d1 = SwapChainDesc1 {
            width: d.width,
            height: d.height,
            format: d.format,
            stereo: FALSE,
            sample_count: d.sample_count,
            sample_quality: d.sample_quality,
            buffer_usage: d.buffer_usage,
            buffer_count: d.buffer_count,
            scaling: 0,
            swap_effect: d.swap_effect,
            alpha_mode: 0,
            flags: d.flags,
        };
        let fs = FullscreenDesc {
            refresh_num: d.refresh_num,
            refresh_den: d.refresh_den,
            scanline_ordering: d.scanline_ordering,
            scaling: d.scaling,
            windowed: d.windowed,
        };
        let hr = swapchain::create_for_d3d11(core, std::ptr::null_mut(), d.output_window, d1, Some(fs), out_swapchain);
        if hr != S_OK {
            if !out_ctx.is_null() {
                Obj::release(imm);
                std::ptr::write_unaligned(out_ctx, std::ptr::null_mut());
            }
            Obj::release(dev);
            null_out(out_device);
            return hr;
        }
    }

    if out_device.is_null() {
        Obj::release(dev);
    }
    S_OK
}

pub unsafe extern "win64" fn D3D11On12CreateDevice(
    d3d12_dev: *mut c_void,
    flags: u32,
    levels: *const u32,
    n_levels: u32,
    queues: *const *mut c_void,
    n_queues: u32,
    _node_mask: u32,
    out_device: *mut *mut c_void,
    out_ctx: *mut *mut c_void,
    out_level: *mut u32,
) -> HRESULT {
    null_out(out_device);
    null_out(out_ctx);
    if !out_level.is_null() {
        std::ptr::write_unaligned(out_level, 0);
    }
    if d3d11_disabled() {
        return DXGI_ERROR_UNSUPPORTED;
    }
    if d3d12_dev.is_null() {
        return d11_failed!(E_INVALIDARG, "D3D11On12CreateDevice: null pDevice");
    }
    // Share the D3D12 command queue if the caller passed one of ours.
    let shared_queue = if !queues.is_null() && n_queues > 0 {
        let q0 = std::ptr::read_unaligned(queues);
        crate::d3d12::queue::from_ptr(q0).map(|q| q.q.clone())
    } else {
        None
    };
    match create_device_internal(
        std::ptr::null_mut(),
        driver_type::UNKNOWN,
        flags,
        levels,
        n_levels,
        shared_queue,
        out_device,
        out_level,
        out_ctx,
    ) {
        Ok((_, dev, _)) => {
            if out_device.is_null() {
                Obj::release(dev);
            }
            S_OK
        }
        Err(hr) => hr,
    }
}

/// Pure-integer leaf D3D11 COM vtable methods (<= 8 integer args, no XMM args/ret,
/// never re-enter guest code) eligible for `DF_PLAIN` direct calls from translated code.
pub fn plain_com_fns() -> &'static [usize] {
    context::plain_fns()
}

#[cfg(test)]
mod tests;
