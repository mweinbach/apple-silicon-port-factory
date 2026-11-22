//! D3D11-compatible `IDXGISwapChain4` backed by `d3d12_metal::metal::swapchain::Swapchain`.
//!
//! Flushes the D3D11 immediate context before presenting, supports all D3D11 swap
//! effects (`DISCARD`, `SEQUENTIAL`, `FLIP_SEQUENTIAL`, `FLIP_DISCARD`), and
//! returns real `ID3D11Texture2D` backbuffers from `GetBuffer` with zero-copy
//! backbuffer rotation on `Present`.

use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use d3d12_metal::metal::swapchain::Swapchain as MtlSwapchain;

use crate::dxgi::swapchain::{
    FullscreenDesc, SwapChainDesc, SwapChainDesc1, IID_IDXGIDeviceSubObject, IID_IDXGISwapChain,
    IID_IDXGISwapChain1, IID_IDXGISwapChain2, IID_IDXGISwapChain3, IID_IDXGISwapChain4,
};
use crate::dxgi::IID_IDXGIObject;
use crate::handles::Semaphore;
use crate::types::*;

use super::com::*;
use super::device::{D3d11Device, DeviceCore};
use super::resources::{D3d11Texture2D, ResourceCore};
use super::types::*;

const DXGI_STATUS_OCCLUDED: HRESULT = 0x087A_0001;
const DXGI_PRESENT_TEST: u32 = 0x1;
const FLAG_FRAME_LATENCY_WAITABLE_OBJECT: u32 = 0x40;

struct State {
    sc: MtlSwapchain,
    /// `ID3D11Texture2D` wrappers (1 ref held here per buffer).
    /// `buffers[0]` tracks `sc.buffers[sc.current]` across `Present` so classic
    /// D3D11 apps that fetch `GetBuffer(0)` once render cleanly into every frame.
    buffers: Vec<*mut Obj<D3d11Texture2D>>,
    desc: SwapChainDesc1,
    fs: FullscreenDesc,
    /// Optional `IDXGIOutput*` target supplied to `SetFullscreenState(TRUE, target)` (1 ref held).
    fs_target: usize,
    /// Saved Win32 `(rect, client)` while in macOS borderless fullscreen.
    saved_win_rects: Option<(crate::user32::RECT, crate::user32::RECT)>,
    color_space: u32,
    max_latency: u32,
    present_refresh: u32,
    bg_color: [f32; 4],
    rotation: u32,
    source_size: (u32, u32),
    matrix: [f32; 6],
    occluded_logged: bool,
    latency: Arc<Semaphore>,
}
unsafe impl Send for State {}

pub struct D3d11SwapChain {
    device: Arc<DeviceCore>,
    factory: usize,
    hwnd: usize,
    presents: AtomicU32,
    st: Mutex<State>,
}

static CLASS: OnceLock<Class> = OnceLock::new();

pub const IDXGISWAPCHAIN4_SLOT_COUNT: usize = 41;

impl ComClass for D3d11SwapChain {
    const TRACE_RELEASE: bool = false;

    fn class() -> &'static Class {
        class_once(&CLASS, || {
            let mut s = dxgi_object_slots::<D3d11SwapChain>();
            s.extend([
                ("GetParent", Some(get_parent as *const ())),
                ("GetDevice", Some(get_device_sc as *const ())),
                // IDXGISwapChain
                ("Present", Some(present as *const ())),
                ("GetBuffer", Some(get_buffer as *const ())),
                ("SetFullscreenState", Some(set_fullscreen_state as *const ())),
                ("GetFullscreenState", Some(get_fullscreen_state as *const ())),
                ("GetDesc", Some(get_desc as *const ())),
                ("ResizeBuffers", Some(resize_buffers as *const ())),
                ("ResizeTarget", Some(resize_target as *const ())),
                ("GetContainingOutput", Some(get_containing_output as *const ())),
                ("GetFrameStatistics", Some(get_frame_statistics as *const ())),
                ("GetLastPresentCount", Some(get_last_present_count as *const ())),
                // IDXGISwapChain1
                ("GetDesc1", Some(get_desc1 as *const ())),
                ("GetFullscreenDesc", Some(get_fullscreen_desc as *const ())),
                ("GetHwnd", Some(get_hwnd as *const ())),
                ("GetCoreWindow", None),
                ("Present1", Some(present1 as *const ())),
                ("IsTemporaryMonoSupported", Some(is_temporary_mono_supported as *const ())),
                ("GetRestrictToOutput", Some(get_containing_output as *const ())),
                ("SetBackgroundColor", Some(set_background_color as *const ())),
                ("GetBackgroundColor", Some(get_background_color as *const ())),
                ("SetRotation", Some(set_rotation as *const ())),
                ("GetRotation", Some(get_rotation as *const ())),
                // IDXGISwapChain2
                ("SetSourceSize", Some(set_source_size as *const ())),
                ("GetSourceSize", Some(get_source_size as *const ())),
                ("SetMaximumFrameLatency", Some(set_maximum_frame_latency as *const ())),
                ("GetMaximumFrameLatency", Some(get_maximum_frame_latency as *const ())),
                ("GetFrameLatencyWaitableObject", Some(get_frame_latency_waitable_object as *const ())),
                ("SetMatrixTransform", Some(set_matrix_transform as *const ())),
                ("GetMatrixTransform", Some(get_matrix_transform as *const ())),
                // IDXGISwapChain3
                ("GetCurrentBackBufferIndex", Some(get_current_back_buffer_index as *const ())),
                ("CheckColorSpaceSupport", Some(check_color_space_support as *const ())),
                ("SetColorSpace1", Some(set_color_space1 as *const ())),
                ("ResizeBuffers1", Some(resize_buffers1 as *const ())),
                // IDXGISwapChain4
                ("SetHDRMetaData", Some(set_hdr_metadata as *const ())),
            ]);
            debug_assert_eq!(s.len(), IDXGISWAPCHAIN4_SLOT_COUNT);
            Class {
                name: "IDXGISwapChain4(D3D11)",
                vtbl: build_vtable("IDXGISwapChain4(D3D11)", &s),
                iids: &[
                    IID_IDXGIObject,
                    IID_IDXGIDeviceSubObject,
                    IID_IDXGISwapChain,
                    IID_IDXGISwapChain1,
                    IID_IDXGISwapChain2,
                    IID_IDXGISwapChain3,
                    IID_IDXGISwapChain4,
                ],
            }
        })
    }

    fn final_release(&self) {
        self.device.flush_immediate();
        let (was_fs, fs_target, saved_rects) = {
            let mut st = self.st.lock().unwrap();
            st.sc.queue.wait_idle();
            for b in std::mem::take(&mut st.buffers) {
                unsafe { Obj::release(b) };
            }
            (
                st.fs.windowed == FALSE,
                std::mem::replace(&mut st.fs_target, 0),
                st.saved_win_rects.take(),
            )
        };
        if fs_target != 0 {
            unsafe { com_release(fs_target as *mut c_void) };
        }
        if was_fs && self.hwnd != 0 {
            let restored = crate::dxgi::adapter::set_native_fullscreen(self.hwnd, false, None);
            if let Some(w) = crate::user32::window::get(self.hwnd) {
                let mut d = w.data();
                if let Some((rect, client)) = saved_rects {
                    d.rect = rect;
                    d.client = client;
                } else if let Some(client) = restored {
                    d.client = client;
                }
            }
        }
        if self.factory != 0 {
            unsafe { com_release(self.factory as *mut c_void) };
        }
    }
}

type PS = *mut Obj<D3d11SwapChain>;

fn main_sync(f: Box<dyn FnOnce() + Send + '_>) {
    if crate::macos::run_sync(f).is_none() {
        log::debug!(target: "d3d", "d3d11 swapchain: no main thread (headless); layer not configured");
    }
}

fn backbuffer_tex2d_desc(w: u32, h: u32, fmt: u32) -> D3D11_TEXTURE2D_DESC {
    D3D11_TEXTURE2D_DESC {
        Width: w,
        Height: h,
        MipLevels: 1,
        ArraySize: 1,
        Format: fmt,
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        Usage: usage::DEFAULT,
        BindFlags: bind_flag::RENDER_TARGET | bind_flag::SHADER_RESOURCE,
        CPUAccessFlags: 0,
        MiscFlags: 0,
    }
}

unsafe fn wrap_buffers(device: &Arc<DeviceCore>, sc: &MtlSwapchain) -> Vec<*mut Obj<D3d11Texture2D>> {
    let td = backbuffer_tex2d_desc(sc.width, sc.height, sc.format);
    sc.buffers
        .iter()
        .map(|b| {
            let core = ResourceCore::wrap_texture2d(device.clone(), b.clone(), td);
            Obj::new(D3d11Texture2D { device: device.clone(), core })
        })
        .collect()
}

fn is_headless_env() -> bool {
    std::env::var("PORT_HEADLESS")
        .or_else(|_| std::env::var("AW2_HEADLESS"))
        .is_ok_and(|v| !v.trim().is_empty() && v.trim() != "0")
}

/// Resolve an HWND's current client dimensions in Win32 pixels.
/// Prefers `user32::window::client_size` (updated synchronously by `CreateWindowEx`
/// and `SetWindowPos`, and valid in headless mode too) with a fallback to the
/// native `WindowShared` state.
fn hwnd_client_size(hwnd: usize) -> Option<(u32, u32)> {
    if hwnd == 0 {
        return None;
    }
    if let Some((cw, ch)) = crate::user32::window::client_size(hwnd) {
        if cw > 0 && ch > 0 {
            return Some((cw as u32, ch as u32));
        }
    }
    crate::macos::window_shared(hwnd)
        .map(|s| s.client_size())
        .filter(|&(cw, ch)| cw > 0 && ch > 0)
}

pub unsafe fn create_for_d3d11(
    device: Arc<DeviceCore>,
    factory: *mut c_void,
    hwnd: usize,
    desc: SwapChainDesc1,
    fs: Option<FullscreenDesc>,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());

    let headless = is_headless_env();
    let layer = match crate::macos::metal_layer_for_hwnd(hwnd) {
        Some(l) => l,
        None if headless || hwnd == 0 => std::ptr::null_mut(),
        None => {
            return d11_failed!(
                DXGI_ERROR_INVALID_CALL,
                "D3D11 CreateSwapChain: hwnd {hwnd:#x} has no CAMetalLayer (set PORT_HEADLESS=1 for offscreen)"
            );
        }
    };
    let (mut w, mut h) = (desc.width, desc.height);
    if w == 0 || h == 0 {
        let (cw, ch) = hwnd_client_size(hwnd).unwrap_or((0, 0));
        w = if w == 0 { cw.max(8) } else { w };
        h = if h == 0 { ch.max(8) } else { h };
    }
    let fmt = if desc.format == 0 { 28 } else { desc.format }; // R8G8B8A8_UNORM default
    let count = desc.buffer_count.clamp(2, 16);
    let sc = match MtlSwapchain::new(layer, device.queue.clone(), w, h, fmt, count, &main_sync) {
        Ok(s) => s,
        Err(e) => return d11_failed!(DXGI_ERROR_INVALID_CALL, "D3D11 CreateSwapChain({desc:?}): {e}"),
    };
    let buffers = wrap_buffers(&device, &sc);
    let mut d = desc;
    d.width = w;
    d.height = h;
    d.format = fmt;
    d.buffer_count = count;
    let fs = fs.unwrap_or(FullscreenDesc { windowed: TRUE, ..Default::default() });
    let mut saved_win_rects = None;
    if fs.windowed == FALSE && hwnd != 0 && !crate::macos::single_surface_mode() {
        if let Some(fs_client) = crate::dxgi::adapter::set_native_fullscreen(hwnd, true, None) {
            if let Some(w_obj) = crate::user32::window::get(hwnd) {
                let mut wd = w_obj.data();
                saved_win_rects = Some((wd.rect, wd.client));
                wd.rect = fs_client;
                wd.client = fs_client;
            }
        }
    }
    let waitable = desc.flags & FLAG_FRAME_LATENCY_WAITABLE_OBJECT != 0;
    let max_latency = if waitable { 1 } else { 3 };
    let latency = Arc::new(Semaphore::new(max_latency as i32, 16, None));
    if !factory.is_null() {
        com_add_ref(factory);
    }
    let initial_cs = sc.color_space;
    let o = Obj::new(D3d11SwapChain {
        device,
        factory: factory as usize,
        hwnd,
        presents: AtomicU32::new(0),
        st: Mutex::new(State {
            sc,
            buffers,
            desc: d,
            fs,
            fs_target: 0,
            saved_win_rects,
            color_space: initial_cs,
            max_latency,
            present_refresh: 0,
            bg_color: [0.0, 0.0, 0.0, 1.0],
            rotation: 1,
            source_size: (w, h),
            matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            occluded_logged: false,
            latency,
        }),
    });
    crate::macos::set_active_swapchain_size(w, h);
    std::ptr::write_unaligned(out, o as *mut c_void);
    log::info!(
        target: "d3d",
        "d3d11 CreateSwapChain(hwnd {hwnd:#x}, {w}x{h}, {}, {count} buffers, effect {}, flags {:#x}) -> {o:?}",
        d3d12_metal::format::name(fmt),
        desc.swap_effect,
        desc.flags
    );
    S_OK
}

unsafe extern "win64" fn get_parent(this: PS, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    let s = Obj::get(this);
    if s.factory != 0 {
        return query_out(s.factory as *mut c_void, riid, out);
    }
    let f = super::device::new_d3d11_factory();
    let hr = query_out(f, riid, out);
    com_release(f);
    hr
}

unsafe extern "win64" fn get_device_sc(this: PS, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    let dev = D3d11Device::get_or_wrap(&Obj::get(this).device);
    let hr = query_out(dev as *mut c_void, riid, out);
    Obj::release(dev);
    hr
}

unsafe fn do_present(this: PS, sync: u32, flags: u32) -> HRESULT {
    let s = Obj::get(this);
    if flags & DXGI_PRESENT_TEST != 0 {
        return S_OK;
    }
    // Flush all pending D3D11 immediate context commands onto the shared queue first.
    s.device.flush_immediate();
    let mut st = s.st.lock().unwrap();
    match st.sc.present(sync) {
        Ok(v) => {
            st.occluded_logged = false;
            st.present_refresh = crate::dxgi::adapter::record_present();
            let sem = st.latency.clone();
            st.sc.queue.timeline.on_completion(v, Box::new(move || {
                sem.release(1);
            }));
            // Rotate `buffers[0]` to point at the newly active backbuffer so D3D11
            // apps keeping a single RTV over `GetBuffer(0)` render into the next buffer.
            let next_idx = st.sc.current;
            if let (Some(&b0), Some(next_mtl)) = (st.buffers.first(), st.sc.buffers.get(next_idx)) {
                Obj::get(b0).core.replace_mtl(next_mtl.clone());
            }
            let n = s.presents.fetch_add(1, Ordering::AcqRel) + 1;
            if n <= 10 || n % 60 == 0 {
                log::info!(target: "d3d", "d3d11 Present #{n} (sync {sync}, flags {flags:#x})");
            }
            if n == 120 || n % 600 == 0 {
                d3d12_metal::metal::pipeline::flush_pso_cache();
            }
            S_OK
        }
        Err(e) => {
            if !st.occluded_logged {
                log::warn!(target: "d3d", "d3d11 Present: {e} -> DXGI_STATUS_OCCLUDED");
                st.occluded_logged = true;
            }
            DXGI_STATUS_OCCLUDED
        }
    }
}

unsafe extern "win64" fn present(this: PS, sync: u32, flags: u32) -> HRESULT {
    crate::shadow::guard_host("d3d11 Present");
    do_present(this, sync, flags)
}

unsafe extern "win64" fn present1(this: PS, sync: u32, flags: u32, _params: *const c_void) -> HRESULT {
    crate::shadow::guard_host("d3d11 Present1");
    do_present(this, sync, flags)
}

unsafe extern "win64" fn get_buffer(this: PS, i: u32, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    let st = Obj::get(this).st.lock().unwrap();
    let Some(&b) = st.buffers.get(i as usize) else {
        return d11_failed!(DXGI_ERROR_INVALID_CALL, "D3D11 SwapChain::GetBuffer({i}): only {} buffers", st.buffers.len());
    };
    query_out(b as *mut c_void, riid, out)
}

unsafe extern "win64" fn set_fullscreen_state(this: PS, fullscreen: BOOL, target: *mut c_void) -> HRESULT {
    let s = Obj::get(this);
    let want_fs = fullscreen != 0;
    let target_rect = if want_fs && !target.is_null() {
        crate::dxgi::adapter::output_monitor_rect(target)
    } else {
        None
    };

    let (prev_fs, old_target, saved_rects_to_restore) = {
        let mut st = s.st.lock().unwrap();
        let prev_fs = st.fs.windowed == FALSE;
        st.fs.windowed = if want_fs { FALSE } else { TRUE };
        let old_target = if want_fs {
            if !target.is_null() {
                com_add_ref(target);
            }
            std::mem::replace(&mut st.fs_target, target as usize)
        } else {
            std::mem::replace(&mut st.fs_target, 0)
        };
        let restore = if !want_fs { st.saved_win_rects.take() } else { None };
        (prev_fs, old_target, restore)
    };
    if old_target != 0 {
        com_release(old_target as *mut c_void);
    }

    if crate::macos::single_surface_mode() {
        if prev_fs != want_fs {
            log::info!(
                target: "d3d",
                "D3D11 SwapChain::SetFullscreenState({}) -> S_OK (locked single-surface mode)",
                if want_fs { "TRUE" } else { "FALSE" }
            );
        }
        return S_OK;
    }

    if want_fs {
        if let Some(fs_client) = crate::dxgi::adapter::set_native_fullscreen(s.hwnd, true, target_rect) {
            if let Some(w) = crate::user32::window::get(s.hwnd) {
                let prev = {
                    let mut d = w.data();
                    let prev = (d.rect, d.client);
                    d.rect = fs_client;
                    d.client = fs_client;
                    prev
                };
                let mut st = s.st.lock().unwrap();
                if st.saved_win_rects.is_none() {
                    st.saved_win_rects = Some(prev);
                }
            }
        }
    } else if prev_fs {
        let restored = crate::dxgi::adapter::set_native_fullscreen(s.hwnd, false, None);
        if let Some(w) = crate::user32::window::get(s.hwnd) {
            let mut d = w.data();
            if let Some((rect, client)) = saved_rects_to_restore {
                d.rect = rect;
                d.client = client;
            } else if let Some(client) = restored {
                d.client = client;
            }
        }
    }

    if prev_fs != want_fs {
        log::info!(
            target: "d3d",
            "D3D11 SwapChain::SetFullscreenState({}) -> S_OK (hwnd {:#x})",
            if want_fs { "TRUE" } else { "FALSE" },
            s.hwnd
        );
    }
    S_OK
}

unsafe extern "win64" fn get_fullscreen_state(this: PS, fullscreen: *mut BOOL, target: *mut *mut c_void) -> HRESULT {
    let s = Obj::get(this);
    let (is_fs, saved_target) = {
        let st = s.st.lock().unwrap();
        (st.fs.windowed == FALSE, st.fs_target)
    };
    if !fullscreen.is_null() {
        std::ptr::write_unaligned(fullscreen, if is_fs { TRUE } else { FALSE });
    }
    if !target.is_null() {
        std::ptr::write_unaligned(target, std::ptr::null_mut());
        if is_fs {
            if saved_target != 0 {
                com_add_ref(saved_target as *mut c_void);
                std::ptr::write_unaligned(target, saved_target as *mut c_void);
            } else {
                let _ = get_containing_output(this, target);
            }
        }
    }
    S_OK
}

unsafe extern "win64" fn get_desc(this: PS, out: *mut SwapChainDesc) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    let s = Obj::get(this);
    let st = s.st.lock().unwrap();
    let d = st.desc;
    std::ptr::write_unaligned(
        out,
        SwapChainDesc {
            width: d.width,
            height: d.height,
            refresh_num: st.fs.refresh_num,
            refresh_den: st.fs.refresh_den,
            format: d.format,
            scanline_ordering: st.fs.scanline_ordering,
            scaling: st.fs.scaling,
            sample_count: d.sample_count,
            sample_quality: d.sample_quality,
            buffer_usage: d.buffer_usage,
            buffer_count: d.buffer_count,
            output_window: s.hwnd,
            windowed: st.fs.windowed,
            swap_effect: d.swap_effect,
            flags: d.flags,
        },
    );
    S_OK
}

unsafe fn do_resize(this: PS, count: u32, w: u32, h: u32, fmt: u32, flags: u32) -> HRESULT {
    let s = Obj::get(this);
    s.device.flush_immediate();
    let mut st = s.st.lock().unwrap();
    for (i, &b) in st.buffers.iter().enumerate() {
        let refs = (*b).refs.load(Ordering::Acquire);
        if refs > 1 {
            return d11_failed!(DXGI_ERROR_INVALID_CALL, "D3D11 ResizeBuffers: back buffer {i} still has {} refs", refs - 1);
        }
    }
    let (mut w, mut h) = (w, h);
    if w == 0 || h == 0 {
        let (cw, ch) = hwnd_client_size(s.hwnd).unwrap_or((st.desc.width, st.desc.height));
        if w == 0 { w = cw.max(8); }
        if h == 0 { h = ch.max(8); }
    }
    let count = if count == 0 { st.desc.buffer_count } else { count.clamp(2, 16) };
    let fmt = if fmt == 0 { st.desc.format } else { fmt };
    if let Err(e) = st.sc.resize(w, h, fmt, count, &main_sync) {
        return d11_failed!(DXGI_ERROR_INVALID_CALL, "D3D11 ResizeBuffers({count}, {w}x{h}, {}): {e}", d3d12_metal::format::name(fmt));
    }
    for b in std::mem::take(&mut st.buffers) {
        Obj::release(b);
    }
    st.buffers = wrap_buffers(&s.device, &st.sc);
    st.desc.width = w;
    st.desc.height = h;
    st.desc.format = fmt;
    st.desc.buffer_count = count;
    st.desc.flags = flags;
    st.source_size = (w, h);
    crate::macos::set_active_swapchain_size(w, h);
    S_OK
}

unsafe extern "win64" fn resize_buffers(this: PS, count: u32, w: u32, h: u32, fmt: u32, flags: u32) -> HRESULT {
    do_resize(this, count, w, h, fmt, flags)
}

unsafe extern "win64" fn resize_buffers1(this: PS, count: u32, w: u32, h: u32, fmt: u32, flags: u32, _masks: *const u32, _queues: *const *mut c_void) -> HRESULT {
    do_resize(this, count, w, h, fmt, flags)
}

unsafe extern "win64" fn resize_target(_this: PS, target: *const c_void) -> HRESULT {
    if target.is_null() { DXGI_ERROR_INVALID_CALL } else { S_OK }
}

unsafe extern "win64" fn get_containing_output(this: PS, out: *mut *mut c_void) -> HRESULT {
    let hwnd = Obj::get(this).hwnd;
    let mut f: *mut c_void = std::ptr::null_mut();
    let hr = crate::dxgi::CreateDXGIFactory1(&crate::dxgi::IID_IDXGIFactory1, &mut f);
    if hr != S_OK || f.is_null() {
        return hr;
    }
    let res = crate::dxgi::adapter::output_for_hwnd(f as *mut Obj<crate::dxgi::Factory>, hwnd, out);
    com_release(f);
    res
}

unsafe extern "win64" fn get_frame_statistics(this: PS, out: *mut crate::dxgi::adapter::FrameStatistics) -> HRESULT {
    if out.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    let s = Obj::get(this);
    let pr = s.st.lock().unwrap().present_refresh;
    let pc = s.presents.load(Ordering::Acquire);
    std::ptr::write_unaligned(out, crate::dxgi::adapter::frame_statistics(pc, pr));
    S_OK
}

unsafe extern "win64" fn get_last_present_count(this: PS, out: *mut u32) -> HRESULT {
    if out.is_null() { return E_INVALIDARG; }
    std::ptr::write_unaligned(out, Obj::get(this).presents.load(Ordering::Acquire));
    S_OK
}

unsafe extern "win64" fn get_desc1(this: PS, out: *mut SwapChainDesc1) -> HRESULT {
    if out.is_null() { return E_INVALIDARG; }
    std::ptr::write_unaligned(out, Obj::get(this).st.lock().unwrap().desc);
    S_OK
}

unsafe extern "win64" fn get_fullscreen_desc(this: PS, out: *mut FullscreenDesc) -> HRESULT {
    if out.is_null() { return E_INVALIDARG; }
    std::ptr::write_unaligned(out, Obj::get(this).st.lock().unwrap().fs);
    S_OK
}

unsafe extern "win64" fn get_hwnd(this: PS, out: *mut HWND) -> HRESULT {
    if out.is_null() { return E_INVALIDARG; }
    std::ptr::write_unaligned(out, Obj::get(this).hwnd as HWND);
    S_OK
}

unsafe extern "win64" fn is_temporary_mono_supported(_this: PS) -> BOOL { FALSE }

unsafe extern "win64" fn set_background_color(this: PS, color: *const [f32; 4]) -> HRESULT {
    if color.is_null() { return E_INVALIDARG; }
    Obj::get(this).st.lock().unwrap().bg_color = std::ptr::read_unaligned(color);
    S_OK
}

unsafe extern "win64" fn get_background_color(this: PS, out: *mut [f32; 4]) -> HRESULT {
    if out.is_null() { return E_INVALIDARG; }
    std::ptr::write_unaligned(out, Obj::get(this).st.lock().unwrap().bg_color);
    S_OK
}

unsafe extern "win64" fn set_rotation(this: PS, rot: u32) -> HRESULT {
    if !(1..=4).contains(&rot) { return DXGI_ERROR_INVALID_CALL; }
    Obj::get(this).st.lock().unwrap().rotation = rot;
    S_OK
}

unsafe extern "win64" fn get_rotation(this: PS, out: *mut u32) -> HRESULT {
    if out.is_null() { return DXGI_ERROR_INVALID_CALL; }
    std::ptr::write_unaligned(out, Obj::get(this).st.lock().unwrap().rotation);
    S_OK
}

unsafe extern "win64" fn set_source_size(this: PS, w: u32, h: u32) -> HRESULT {
    let mut st = Obj::get(this).st.lock().unwrap();
    if w == 0 || h == 0 || w > st.desc.width || h > st.desc.height {
        return E_INVALIDARG;
    }
    st.source_size = (w, h);
    S_OK
}

unsafe extern "win64" fn get_source_size(this: PS, w: *mut u32, h: *mut u32) -> HRESULT {
    if w.is_null() || h.is_null() { return E_INVALIDARG; }
    let (sw, sh) = Obj::get(this).st.lock().unwrap().source_size;
    std::ptr::write_unaligned(w, sw);
    std::ptr::write_unaligned(h, sh);
    S_OK
}

unsafe extern "win64" fn set_matrix_transform(this: PS, m: *const [f32; 6]) -> HRESULT {
    if m.is_null() { return E_INVALIDARG; }
    Obj::get(this).st.lock().unwrap().matrix = std::ptr::read_unaligned(m);
    S_OK
}

unsafe extern "win64" fn get_matrix_transform(this: PS, out: *mut [f32; 6]) -> HRESULT {
    if out.is_null() { return E_INVALIDARG; }
    std::ptr::write_unaligned(out, Obj::get(this).st.lock().unwrap().matrix);
    S_OK
}

unsafe extern "win64" fn set_maximum_frame_latency(this: PS, n: u32) -> HRESULT {
    if n == 0 || n > 16 { return DXGI_ERROR_INVALID_CALL; }
    let mut st = Obj::get(this).st.lock().unwrap();
    if n > st.max_latency {
        st.latency.release((n - st.max_latency) as i32);
    }
    st.max_latency = n;
    S_OK
}

unsafe extern "win64" fn get_frame_latency_waitable_object(this: PS) -> HANDLE {
    let st = Obj::get(this).st.lock().unwrap();
    crate::handles::insert(st.latency.clone())
}

unsafe extern "win64" fn get_maximum_frame_latency(this: PS, out: *mut u32) -> HRESULT {
    if out.is_null() { return E_INVALIDARG; }
    std::ptr::write_unaligned(out, Obj::get(this).st.lock().unwrap().max_latency);
    S_OK
}

unsafe extern "win64" fn get_current_back_buffer_index(this: PS) -> u32 {
    Obj::get(this).st.lock().unwrap().sc.current as u32
}

unsafe extern "win64" fn check_color_space_support(_this: PS, cs: u32, out: *mut u32) -> HRESULT {
    if out.is_null() { return E_INVALIDARG; }
    std::ptr::write_unaligned(out, if d3d12_metal::metal::swapchain::color_space_supported(cs) { 1 } else { 0 });
    S_OK
}

unsafe extern "win64" fn set_color_space1(this: PS, cs: u32) -> HRESULT {
    let mut st = Obj::get(this).st.lock().unwrap();
    if let Err(e) = st.sc.set_color_space(cs, &main_sync) {
        return d11_failed!(E_INVALIDARG, "D3D11 SetColorSpace1({cs}): {e}");
    }
    st.color_space = cs;
    S_OK
}

unsafe extern "win64" fn set_hdr_metadata(_this: PS, _ty: u32, _size: u32, _data: *const c_void) -> HRESULT {
    S_OK
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d3d11_fullscreen_state_roundtrip_and_adapter_driver_version() {
        if d3d12_metal::metal::device().is_none() {
            return;
        }
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        for locked in [false, true] {
            crate::macos::with_single_surface_mode(locked, || unsafe {
                let sc_desc = SwapChainDesc {
                    width: 640,
                    height: 360,
                    refresh_num: 60,
                    refresh_den: 1,
                    format: 28,
                    scanline_ordering: 0,
                    scaling: 0,
                    sample_count: 1,
                    sample_quality: 0,
                    buffer_usage: 0x20,
                    buffer_count: 2,
                    output_window: 0,
                    windowed: TRUE,
                    swap_effect: 0,
                    flags: 0,
                };
                let (mut sc, mut dev, mut ctx) = (std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut());
                assert_eq!(
                    crate::d3d11::D3D11CreateDeviceAndSwapChain(
                        std::ptr::null_mut(),
                        driver_type::HARDWARE,
                        std::ptr::null_mut(),
                        0,
                        std::ptr::null(),
                        0,
                        D3D11_SDK_VERSION,
                        &sc_desc as *const _ as *const c_void,
                        &mut sc,
                        &mut dev,
                        std::ptr::null_mut(),
                        &mut ctx,
                    ),
                    S_OK
                );

                // Verify D3D11 DXGI adapter CheckInterfaceSupport returns packed WDDM 3.2 driver version.
                let mut dxgi_dev: *mut c_void = std::ptr::null_mut();
                assert_eq!(com_query(dev, &IID_IDXGIDevice, &mut dxgi_dev), S_OK);
                let get_adp: unsafe extern "win64" fn(*mut c_void, *mut *mut c_void) -> HRESULT =
                    std::mem::transmute((*(dxgi_dev as *const *const usize)).add(7).read());
                let mut adp: *mut c_void = std::ptr::null_mut();
                assert_eq!(get_adp(dxgi_dev, &mut adp), S_OK);
                let check_iface: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut i64) -> HRESULT =
                    std::mem::transmute((*(adp as *const *const usize)).add(9).read());
                for iid in [
                    crate::dxgi::adapter::IID_IDXGIDevice,
                    crate::dxgi::adapter::IID_IDXGIDevice1,
                    crate::dxgi::adapter::IID_ID3D10Device,
                    crate::dxgi::adapter::IID_ID3D11Device,
                ] {
                    let mut umd = 0i64;
                    assert_eq!(check_iface(adp, &iid, &mut umd), S_OK);
                    assert_eq!(umd, crate::dxgi::adapter::UMD_DRIVER_VERSION as i64);
                }
                let mut bad_umd = 1234i64;
                assert_eq!(
                    check_iface(adp, &crate::dxgi::adapter::IID_IDXGIAdapter, &mut bad_umd),
                    DXGI_ERROR_UNSUPPORTED
                );
                assert_eq!(bad_umd, 0);
                com_release(adp);
                com_release(dxgi_dev);

                // Verify SetFullscreenState(TRUE) / GetFullscreenState / GetDesc / SetFullscreenState(FALSE).
                let sc_obj = sc as PS;
                let mut is_fs: BOOL = TRUE;
                let mut target: *mut c_void = std::ptr::null_mut();
                assert_eq!(get_fullscreen_state(sc_obj, &mut is_fs, &mut target), S_OK);
                assert_eq!(is_fs, FALSE);
                assert!(target.is_null());

                assert_eq!(set_fullscreen_state(sc_obj, TRUE, std::ptr::null_mut()), S_OK);
                assert_eq!(get_fullscreen_state(sc_obj, &mut is_fs, &mut target), S_OK);
                assert_eq!(is_fs, TRUE);
                assert!(!target.is_null());

                // Explicit IDXGIOutput target also succeeds and round-trips cleanly.
                assert_eq!(set_fullscreen_state(sc_obj, TRUE, target), S_OK);
                com_release(target);
                target = std::ptr::null_mut();
                assert_eq!(get_fullscreen_state(sc_obj, &mut is_fs, &mut target), S_OK);
                assert_eq!(is_fs, TRUE);
                assert!(!target.is_null());
                com_release(target);

                let mut got_desc = SwapChainDesc::default();
                assert_eq!(get_desc(sc_obj, &mut got_desc), S_OK);
                assert_eq!(got_desc.windowed, FALSE);
                assert_eq!((got_desc.width, got_desc.height), (640, 360));

                assert_eq!(set_fullscreen_state(sc_obj, FALSE, std::ptr::null_mut()), S_OK);
                assert_eq!(get_fullscreen_state(sc_obj, &mut is_fs, &mut target), S_OK);
                assert_eq!(is_fs, FALSE);
                assert!(target.is_null());
                assert_eq!(get_desc(sc_obj, &mut got_desc), S_OK);
                assert_eq!(got_desc.windowed, TRUE);

                com_release(sc);
                com_release(ctx);
                com_release(dev);
            });
        }
    }
}
