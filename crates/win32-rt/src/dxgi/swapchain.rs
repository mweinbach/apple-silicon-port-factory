//! IDXGISwapChain4 on a window's CAMetalLayer (flip model).

use super::Factory;
use crate::d3d12::com::*;
use crate::types::*;
use std::ffi::c_void;

pub const IID_IDXGIDeviceSubObject: Guid = guid("3d3e0379-f9de-4d58-bb6c-18d62992f1a6");
pub const IID_IDXGISwapChain: Guid = guid("310d36a0-d2e7-4c0a-aa04-6a9d23b8886a");
pub const IID_IDXGISwapChain1: Guid = guid("790a45f7-0d42-4876-983a-0a55cfe6f4aa");
pub const IID_IDXGISwapChain2: Guid = guid("a8be2ac4-199f-4946-b331-79599fb98de7");
pub const IID_IDXGISwapChain3: Guid = guid("94d99bdb-f1f8-4ab0-b236-7da0170edab1");
pub const IID_IDXGISwapChain4: Guid = guid("3d585d5a-bd4a-489e-b1f4-3dbcb6452ffb");

pub fn register_names() {
    for (g, n) in [
        (IID_IDXGIDeviceSubObject, "IDXGIDeviceSubObject"),
        (IID_IDXGISwapChain, "IDXGISwapChain"),
        (IID_IDXGISwapChain1, "IDXGISwapChain1"),
        (IID_IDXGISwapChain2, "IDXGISwapChain2"),
        (IID_IDXGISwapChain3, "IDXGISwapChain3"),
        (IID_IDXGISwapChain4, "IDXGISwapChain4"),
    ] {
        name_iid(g, n);
    }
}

/// `DXGI_SWAP_CHAIN_DESC1`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SwapChainDesc1 {
    pub width: u32,
    pub height: u32,
    pub format: u32,
    pub stereo: BOOL,
    pub sample_count: u32,
    pub sample_quality: u32,
    pub buffer_usage: u32,
    pub buffer_count: u32,
    pub scaling: u32,
    pub swap_effect: u32,
    pub alpha_mode: u32,
    pub flags: u32,
}
const _: () = assert!(std::mem::size_of::<SwapChainDesc1>() == 48);

/// `DXGI_SWAP_CHAIN_FULLSCREEN_DESC`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct FullscreenDesc {
    pub refresh_num: u32,
    pub refresh_den: u32,
    pub scanline_ordering: u32,
    pub scaling: u32,
    pub windowed: BOOL,
}

/// `DXGI_SWAP_CHAIN_DESC` (legacy).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SwapChainDesc {
    pub width: u32,
    pub height: u32,
    pub refresh_num: u32,
    pub refresh_den: u32,
    pub format: u32,
    pub scanline_ordering: u32,
    pub scaling: u32,
    pub sample_count: u32,
    pub sample_quality: u32,
    pub buffer_usage: u32,
    pub buffer_count: u32,
    pub output_window: usize,
    pub windowed: BOOL,
    pub swap_effect: u32,
    pub flags: u32,
}
const _: () = assert!(std::mem::size_of::<SwapChainDesc>() == 72);

pub(super) unsafe extern "win64" fn factory_create_swap_chain(
    this: *mut Obj<Factory>,
    device: *mut c_void,
    desc: *const SwapChainDesc,
    out: *mut *mut c_void,
) -> HRESULT {
    if desc.is_null() || out.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    let d = *desc;
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
    let fs = FullscreenDesc { refresh_num: d.refresh_num, refresh_den: d.refresh_den, scanline_ordering: d.scanline_ordering, scaling: d.scaling, windowed: d.windowed };
    factory_create_swap_chain_for_hwnd(this, device, d.output_window as HWND, &d1, &fs, std::ptr::null_mut(), out)
}

pub(super) unsafe extern "win64" fn factory_create_swap_chain_for_hwnd(
    this: *mut Obj<Factory>,
    device: *mut c_void,
    hwnd: HWND,
    desc: *const SwapChainDesc1,
    fs: *const FullscreenDesc,
    _restrict: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if desc.is_null() || out.is_null() || device.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    let d = std::ptr::read_unaligned(desc);
    let fs = if fs.is_null() { None } else { Some(std::ptr::read_unaligned(fs)) };
    log::info!(target: "d3d", "CreateSwapChainForHwnd(hwnd {:#x}, {:?}, fullscreen {:?})", hwnd as usize, d, fs);
    if let Some(core) = crate::d3d11::device::device_core_from_ptr(device) {
        return crate::d3d11::swapchain::create_for_d3d11(core, this as *mut c_void, hwnd as usize, d, fs, out);
    }
    crate::d3d12::swapchain::create(this, device, hwnd as usize, d, fs, out)
}
