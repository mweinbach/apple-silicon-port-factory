//! IDXGISwapChain4 on a window's CAMetalLayer, created by
//! `IDXGIFactory::CreateSwapChain*` with a D3D12 command queue as "device".
//!
//! Back buffers are our `ID3D12Resource` objects over private textures;
//! `Present` blits the current one into a drawable on the queue (see
//! `d3d12_metal::metal::swapchain`).

use super::com::*;
use super::tr::{self, tr};
use super::failed;
use super::resource::{Resource, PR};
use crate::dxgi::swapchain::{FullscreenDesc, SwapChainDesc, SwapChainDesc1};
use crate::dxgi::{Factory, IID_IDXGIObject};
use crate::types::*;
use d3d12_metal::metal::swapchain::Swapchain as MtlSwapchain;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use crate::handles::Semaphore;
use std::sync::{Arc, Mutex, OnceLock};

const DXGI_STATUS_OCCLUDED: HRESULT = 0x087A_0001;
const DXGI_PRESENT_TEST: u32 = 0x1;
const FLAG_FRAME_LATENCY_WAITABLE_OBJECT: u32 = 0x40;

struct State {
    sc: MtlSwapchain,
    /// Our back-buffer `ID3D12Resource`s (one reference held here).
    buffers: Vec<PR>,
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
    /// Released once per completed present; starts at the maximum frame latency.
    latency: Arc<Semaphore>,
}
unsafe impl Send for State {}

pub struct SwapChain {
    factory: usize,
    queue: usize,
    hwnd: usize,
    presents: AtomicU32,
    st: Mutex<State>,
}

static CLASS: OnceLock<Class> = OnceLock::new();


impl ComClass for SwapChain {
    fn class() -> &'static Class {
        use crate::dxgi::swapchain::*;
        class_once(&CLASS, || {
            type S = SwapChain;
            let mut s = dxgi_object_slots::<S>();
            s.extend([
                slot!("GetParent" => get_parent),
                // IDXGIDeviceSubObject
                slot!("GetDevice" => get_device_sc),
                // IDXGISwapChain
                slot!("Present" => present),
                slot!("GetBuffer" => get_buffer),
                slot!("SetFullscreenState" => set_fullscreen_state),
                slot!("GetFullscreenState" => get_fullscreen_state),
                slot!("GetDesc" => get_desc),
                slot!("ResizeBuffers" => resize_buffers),
                slot!("ResizeTarget" => resize_target),
                slot!("GetContainingOutput" => get_containing_output),
                slot!("GetFrameStatistics" => get_frame_statistics),
                slot!("GetLastPresentCount" => get_last_present_count),
                // IDXGISwapChain1
                slot!("GetDesc1" => get_desc1),
                slot!("GetFullscreenDesc" => get_fullscreen_desc),
                slot!("GetHwnd" => get_hwnd),
                slot!("GetCoreWindow"),
                slot!("Present1" => present1),
                slot!("IsTemporaryMonoSupported" => is_temporary_mono_supported),
                slot!("GetRestrictToOutput" => get_containing_output),
                slot!("SetBackgroundColor" => set_background_color),
                slot!("GetBackgroundColor" => get_background_color),
                slot!("SetRotation" => set_rotation),
                slot!("GetRotation" => get_rotation),
                // IDXGISwapChain2
                slot!("SetSourceSize" => set_source_size),
                slot!("GetSourceSize" => get_source_size),
                slot!("SetMaximumFrameLatency" => set_maximum_frame_latency),
                slot!("GetMaximumFrameLatency" => get_maximum_frame_latency),
                slot!("GetFrameLatencyWaitableObject" => get_frame_latency_waitable_object),
                slot!("SetMatrixTransform" => set_matrix_transform),
                slot!("GetMatrixTransform" => get_matrix_transform),
                // IDXGISwapChain3
                slot!("GetCurrentBackBufferIndex" => get_current_back_buffer_index),
                slot!("CheckColorSpaceSupport" => check_color_space_support),
                slot!("SetColorSpace1" => set_color_space1),
                slot!("ResizeBuffers1" => resize_buffers1),
                // IDXGISwapChain4
                slot!("SetHDRMetaData" => set_hdr_metadata),
            ]);
            debug_assert_eq!(s.len(), 41);
            Class {
                name: "IDXGISwapChain4",
                vtbl: build_vtable("IDXGISwapChain4", &s),
                iids: &[IID_IDXGIObject, IID_IDXGIDeviceSubObject, IID_IDXGISwapChain, IID_IDXGISwapChain1, IID_IDXGISwapChain2, IID_IDXGISwapChain3, IID_IDXGISwapChain4],
            }
        })
    }

    fn final_release(&self) {
        super::cs::flush();
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
        unsafe {
            com_release(self.queue as *mut c_void);
            com_release(self.factory as *mut c_void);
        }
    }
}

type PS = *mut Obj<SwapChain>;

fn main_sync(f: Box<dyn FnOnce() + Send + '_>) {
    if crate::macos::run_sync(f).is_none() {
        log::warn!(target: "d3d", "swapchain: no main thread (headless); layer not configured");
    }
}

unsafe fn wrap_buffers(sc: &MtlSwapchain) -> Vec<PR> {
    // RENDER_TARGET state; heap flags NONE.
    sc.buffers.iter().map(|b| Resource::wrap(b.clone(), 0, 0x4)).collect()
}

pub unsafe fn create(factory: *mut Obj<Factory>, queue: *mut c_void, hwnd: usize, desc: SwapChainDesc1, fs: Option<FullscreenDesc>, out: *mut *mut c_void) -> HRESULT {
    *out = std::ptr::null_mut();
    let Some(q) = super::queue::from_ptr(queue) else {
        return failed!(DXGI_ERROR_INVALID_CALL, "CreateSwapChain: 'device' {queue:?} is not one of our D3D12 command queues");
    };
    if !matches!(desc.swap_effect, 3 | 4) {
        return failed!(DXGI_ERROR_INVALID_CALL, "CreateSwapChain: swap effect {} (D3D12 needs FLIP_SEQUENTIAL/FLIP_DISCARD)", desc.swap_effect);
    }
    let headless = std::env::var("PORT_HEADLESS")
        .or_else(|_| std::env::var("AW2_HEADLESS"))
        .is_ok_and(|v| !v.trim().is_empty() && v.trim() != "0");
    let layer = match crate::macos::metal_layer_for_hwnd(hwnd) {
        Some(l) => l,
        // Headless: offscreen swapchain (Present renders, captures, paces;
        // nothing is displayed). The game cannot run without a swapchain.
        None if headless => std::ptr::null_mut(),
        None => return failed!(DXGI_ERROR_INVALID_CALL, "CreateSwapChain: hwnd {hwnd:#x} has no CAMetalLayer (not a top-level window)"),
    };
    let (mut w, mut h) = (desc.width, desc.height);
    if w == 0 || h == 0 {
        let (cw, ch) = crate::macos::window_shared(hwnd).map(|s| s.client_size()).unwrap_or((0, 0));
        w = if w == 0 { cw.max(8) } else { w };
        h = if h == 0 { ch.max(8) } else { h };
    }
    let count = desc.buffer_count.clamp(2, 16);
    let sc = match MtlSwapchain::new(layer, q.q.clone(), w, h, desc.format, count, &main_sync) {
        Ok(s) => s,
        Err(e) => return failed!(DXGI_ERROR_INVALID_CALL, "CreateSwapChain({desc:?}): {e}"),
    };
    let buffers = wrap_buffers(&sc);
    let mut d = desc;
    d.width = w;
    d.height = h;
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
    // DXGI's default maximum latency is 1 for waitable swap chains, 3 otherwise
    // (capped to 2 on iOS to keep full CPU/GPU overlap without 3-frame memory bloat).
    let max_latency = if waitable {
        1
    } else if cfg!(target_os = "ios") {
        2
    } else {
        3
    };
    if cfg!(target_os = "ios")
        && std::env::var_os("PORT_IOS_PERF_LOG").is_none_or(|v| v != "0")
    {
        d3d12_metal::metal::queue::enable_perf_meter();
    }
    let latency = Arc::new(Semaphore::new(max_latency as i32, 16, None));
    com_add_ref(factory as *mut c_void);
    com_add_ref(queue);
    let initial_cs = sc.color_space;
    let o = Obj::new(SwapChain {
        factory: factory as usize,
        queue: queue as usize,
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
    *out = o as *mut c_void;
    tr!(Call::CreateSwapChain { queue: queue as u64, swapchain: o as u64, desc: tr::raw(&d) });
    log::info!(target: "d3d", "CreateSwapChain(hwnd {hwnd:#x}, {w}x{h}, {}, {count} buffers, flags {:#x}) -> {o:?}", d3d12_metal::format::name(desc.format), desc.flags);
    S_OK
}

unsafe extern "win64" fn get_parent(this: PS, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    query_out(Obj::get(this).factory as *mut c_void, riid, out)
}

unsafe extern "win64" fn get_device_sc(this: PS, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    query_out(Obj::get(this).queue as *mut c_void, riid, out)
}

unsafe fn do_present(this: PS, sync: u32, flags: u32) -> HRESULT {
    let t0 = crate::hitch::span_begin();
    let hr = do_present_traced(this, sync, flags);
    crate::hitch::span_end(t0, crate::hitch::SPAN_PRESENT);
    hr
}

unsafe fn do_present_traced(this: PS, sync: u32, flags: u32) -> HRESULT {
    // The frame's command lists must be encoded and submitted first.
    super::cs::flush();
    tr::sync_uploads();
    tr!(Call::Present { swapchain: this as u64, sync_interval: sync, flags });
    let hr = present_inner(this, sync, flags);
    tr!(Call::FrameEnd { frame: Obj::get(this).presents.load(Ordering::Relaxed) as u64 });
    hr
}

unsafe fn present_inner(this: PS, sync: u32, flags: u32) -> HRESULT {
    let s = Obj::get(this);
    if flags & DXGI_PRESENT_TEST != 0 {
        return S_OK;
    }
    let mut st = s.st.lock().unwrap();
    let present_scope = crate::hitch::Scope::enter(crate::hitch::ActKind::Present, sync as u64);
    let res = st.sc.present(sync);
    let present_ns = present_scope.elapsed_ns();
    drop(present_scope);
    match res {
        Ok(v) => {
            st.occluded_logged = false;
            st.present_refresh = crate::dxgi::adapter::record_present();
            let sem = st.latency.clone();
            st.sc.queue.timeline.on_completion(v, Box::new(move || {
                sem.release(1);
            }));
            let n = s.presents.fetch_add(1, Ordering::AcqRel) + 1;
            if n == 1 && cfg!(target_os = "ios") {
                crate::thread::sched::promote_current_thread_interactive();
            }
            crate::hitch::on_present(n, present_ns);
            d3d12_metal::metal::gpucapture::on_present(n.into());
            if n <= 10 || n % 60 == 0 {
                log::info!(target: "d3d", "Present #{n} (sync {sync}, flags {flags:#x})");
            }
            if n == 10 || n == 60 || (cfg!(target_os = "ios") && n % 300 == 0) {
                super::mtl::log_memory_snapshot_full(&format!("frame#{n}"));
            }
            if cfg!(target_os = "ios") {
                d3d12_metal::metal::queue::collect_all();
                if n % 30 == 0 {
                    d3d12_metal::metal::pipeline::flush_pso_cache_if_burst(8);
                }
                // Periodic zero-overhead CPU vs GPU perf summary every 120 frames on iOS.
                if std::env::var_os("PORT_IOS_PERF_LOG").is_none_or(|v| v != "0") {
                    static LAST_PERF_NS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
                    static LAST_PERF_FRAME: AtomicU32 = AtomicU32::new(0);
                    if n == 10 {
                        LAST_PERF_NS.store(crate::hitch::now_ns(), Ordering::Relaxed);
                        LAST_PERF_FRAME.store(n, Ordering::Relaxed);
                        let _ = d3d12_metal::metal::queue::take_perf_meter_sample();
                        let _ = d3d12_metal::metal::cmdlist::take_ios_icb_pass_splits();
                        let _ = d3d12_metal::metal::indirect_icb::take_new_icb_count();
                        let _ = super::cs::take_perf_stats();
                        let _ = crate::audio::backend::take_underrun_frames();
                    } else if n > 10 && n % 120 == 0 {
                        let now = crate::hitch::now_ns();
                        let prev_ns = LAST_PERF_NS.swap(now, Ordering::Relaxed);
                        let prev_f = LAST_PERF_FRAME.swap(n, Ordering::Relaxed);
                        let df = n.saturating_sub(prev_f);
                        let (gpu_ns, submits, cbs) = d3d12_metal::metal::queue::take_perf_meter_sample();
                        let icb_splits = d3d12_metal::metal::cmdlist::take_ios_icb_pass_splits();
                        let icb_new = d3d12_metal::metal::indirect_icb::take_new_icb_count();
                        let (cs_enc_ns, cs_flush_ns) = super::cs::take_perf_stats();
                        let aud_under = crate::audio::backend::take_underrun_frames();
                        if prev_ns != 0 && df > 0 && now > prev_ns {
                            let dt_s = (now - prev_ns) as f64 / 1e9;
                            let fps = df as f64 / dt_s;
                            let wall_ms = (dt_s * 1000.0) / df as f64;
                            let gpu_ms = (gpu_ns as f64 / 1e6) / df as f64;
                            let cs_enc_ms = (cs_enc_ns as f64 / 1e6) / df as f64;
                            let cs_flush_ms = (cs_flush_ns as f64 / 1e6) / df as f64;
                            let fp_mb = super::mtl::process_phys_footprint() as f64 / 1048576.0;
                            let avail_mb = super::mtl::available_headroom_bytes().unwrap_or(0) as f64 / 1048576.0;
                            let thermal = d3d12_metal::metal::device::thermal_state();
                            log::info!(
                                target: "d3d",
                                "perf[frame#{n}]: fps={fps:.1} wall={wall_ms:.1}ms gpu_busy={gpu_ms:.1}ms cs_enc={cs_enc_ms:.1}ms cs_flush={cs_flush_ms:.1}ms submits={:.1}/f cbs={:.1}/f icb_splits={:.1}/f icb_new={icb_new} aud_under={aud_under} phys={fp_mb:.0}MiB avail={avail_mb:.0}MiB thermal={thermal}",
                                submits as f64 / df as f64,
                                cbs as f64 / df as f64,
                                icb_splits as f64 / df as f64,
                            );
                        }
                    }
                }
                if let Some(avail) = super::mtl::available_headroom_bytes() {
                    let fp = super::mtl::process_phys_footprint();
                    // Log each new 256 MiB high-water mark above 4.5 GiB so tight transitions
                    // are always visible even between periodic 300-frame snapshots.
                    let bucket = (fp >> 28) as u32;
                    static HWM_BUCKET: AtomicU32 = AtomicU32::new(17); // 17 * 256 MiB = 4352 MiB
                    if bucket > HWM_BUCKET.load(Ordering::Relaxed) {
                        HWM_BUCKET.store(bucket, Ordering::Relaxed);
                        super::mtl::log_memory_snapshot_full(&format!("ios-hwm#{n}"));
                    }
                    // Emergency Jetsam-imminent GPU drain (`PORT_IOS_PRESSURE_SYNC_MB`, default
                    // 512 MiB; `PORT_IOS_PRESSURE_SYNC=0` disables). Kept strictly below normal
                    // gameplay headroom so CPU and GPU stay fully pipelined at 2x higher FPS.
                    static PRESSURE_SYNC_BYTES: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
                    let sync_threshold = *PRESSURE_SYNC_BYTES.get_or_init(|| {
                        if std::env::var_os("PORT_IOS_PRESSURE_SYNC").is_some_and(|v| v == "0") {
                            0
                        } else {
                            std::env::var("PORT_IOS_PRESSURE_SYNC_MB")
                                .ok()
                                .and_then(|v| v.trim().parse::<u64>().ok())
                                .unwrap_or(512)
                                << 20
                        }
                    });
                    if sync_threshold > 0 && avail < sync_threshold {
                        let _wait_scope = crate::hitch::Scope::enter(crate::hitch::ActKind::FenceWait, 0);
                        st.sc.queue.wait_idle();
                        d3d12_metal::metal::queue::collect_all();
                    }
                    if avail < super::mtl::ios_jetsam_headroom_bytes() {
                        static LAST_PRESSURE_FRAME: AtomicU32 = AtomicU32::new(0);
                        let prev = LAST_PRESSURE_FRAME.load(Ordering::Relaxed);
                        let min_gap = if avail < (768 << 20) { 30 } else { 120 };
                        if prev == 0 || n.saturating_sub(prev) >= min_gap {
                            LAST_PRESSURE_FRAME.store(n, Ordering::Relaxed);
                            super::mtl::log_memory_snapshot_full(&format!("ios-pressure#{n}"));
                            d3d12_metal::metal::pipeline::trim_caches();
                            crate::dxgi::adapter::notify_video_memory_budget_change();
                        }
                    }
                }
            }
            if n == 120 || n == 240 || n % 600 == 0 {
                d3d12_metal::metal::pipeline::flush_pso_cache();
            }
            if n % 600 == 0 && d3d12_metal::metal::encstats::enabled() {
                d3d12_metal::metal::encstats::dump(600);
            }
            if d3d12_metal::metal::passtiming::enabled() {
                d3d12_metal::metal::passtiming::frame_mark();
                if n % 600 == 0 {
                    d3d12_metal::metal::passtiming::dump(600);
                }
            }
            if let Some(script) = std::env::var_os("PORT_INPUT_SCRIPT")
                .or_else(|| std::env::var_os("AW2_AUTO_PLAY_SCRIPT"))
                .and_then(|v| v.into_string().ok())
            {
                run_input_script_step(s.hwnd, n, &script);
            }
            if let Ok(hooks) = PRESENT_HOOKS.read() {
                for hook in hooks.iter() {
                    hook(s.hwnd, n);
                }
            }
            S_OK
        }
        Err(e) => {
            if !st.occluded_logged {
                log::warn!(target: "d3d", "Present: {e} -> DXGI_STATUS_OCCLUDED");
                st.occluded_logged = true;
            }
            DXGI_STATUS_OCCLUDED
        }
    }
}

/// Callback invoked after every successful `IDXGISwapChain::Present` (`(hwnd, frame_number)`).
pub type PresentHook = fn(usize, u32);

static PRESENT_HOOKS: std::sync::RwLock<Vec<PresentHook>> = std::sync::RwLock::new(Vec::new());

/// Register a per-Present hook (e.g. game-specific automated test drivers or telemetry).
pub fn register_present_hook(hook: PresentHook) {
    PRESENT_HOOKS.write().unwrap().push(hook);
}

/// Generic scripted input runner (`PORT_INPUT_SCRIPT="900:down,960:enter"`).
fn run_input_script_step(hwnd: usize, frame: u32, script: &str) {
    use crate::misc::xinput1_4::{self, btn, DeviceInfo, Gamepad};
    use crate::user32::input::{self, NativeKey};
    input::force_focus_for_autoplay(hwnd);
    if frame == 30 {
        xinput1_4::backend_connect(
            0,
            DeviceInfo { name: "Virtual Controller".into(), wireless: false, rumble: false, battery: None },
        );
    }
    for item in script.split(',') {
        let Some((f, action)) = item.trim().split_once(':') else { continue };
        let Ok(f) = f.trim().parse::<u32>() else { continue };
        let down = if frame == f {
            true
        } else if frame == f + 6 {
            false
        } else {
            continue;
        };
        let (mac, chars, pad): (u16, Vec<u16>, u16) = match action.trim() {
            "enter" => (0x24, vec![0x0D], btn::A),
            "space" => (0x31, vec![0x20], btn::A),
            "a" => (0x24, vec![0x0D], btn::A),
            "esc" | "b" => (0x35, vec![0x1B], btn::B),
            "up" => (0x7E, Vec::new(), btn::DPAD_UP),
            "down" => (0x7D, Vec::new(), btn::DPAD_DOWN),
            "left" => (0x7B, Vec::new(), btn::DPAD_LEFT),
            "right" => (0x7C, Vec::new(), btn::DPAD_RIGHT),
            _ => continue,
        };
        log::info!(target: "d3d", "input script: frame {frame} {action} {}", if down { "down" } else { "up" });
        input::native_key(NativeKey { mac, down, repeat: false, chars: if down { chars } else { Vec::new() } });
        xinput1_4::backend_update(0, Gamepad { buttons: if down { pad } else { 0 }, ..Default::default() });
    }
}

unsafe extern "win64" fn present(this: PS, sync: u32, flags: u32) -> HRESULT {
    crate::shadow::guard_host("dxgi Present");
    do_present(this, sync, flags)
}

unsafe extern "win64" fn present1(this: PS, sync: u32, flags: u32, _params: *const c_void) -> HRESULT {
    crate::shadow::guard_host("dxgi Present1");
    do_present(this, sync, flags)
}

unsafe extern "win64" fn get_buffer(this: PS, i: u32, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    *out = std::ptr::null_mut();
    let st = Obj::get(this).st.lock().unwrap();
    let Some(b) = st.buffers.get(i as usize) else {
        return failed!(DXGI_ERROR_INVALID_CALL, "IDXGISwapChain::GetBuffer({i}): only {} buffers", st.buffers.len());
    };
    let hr = query_out(*b as *mut c_void, riid, out);
    tr!(Call::GetSwapChainBuffer { swapchain: this as u64, index: i, resource: tr::created(hr, out).unwrap_or(0) });
    hr
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
        // In locked single-surface mode (iOS or PORT_LOCK_DISPLAY_MODE), Windowed,
        // Borderless, and Fullscreen all present identically on the single fullscreen
        // CAMetalLayer. Accept both states so games do not enter a retry loop.
        if prev_fs != want_fs {
            log::info!(
                target: "d3d",
                "IDXGISwapChain::SetFullscreenState({}) -> S_OK (locked single-surface mode)",
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
            "IDXGISwapChain::SetFullscreenState({}) -> S_OK (hwnd {:#x})",
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
    super::cs::flush();
    let s = Obj::get(this);
    let mut st = s.st.lock().unwrap();
    for (i, b) in st.buffers.iter().enumerate() {
        let refs = (**b).refs.load(Ordering::Acquire);
        if refs > 1 {
            return failed!(DXGI_ERROR_INVALID_CALL, "ResizeBuffers: back buffer {i} still has {} outstanding references", refs - 1);
        }
    }
    let (mut w, mut h) = (w, h);
    if w == 0 || h == 0 {
        let (cw, ch) = crate::macos::window_shared(s.hwnd).map(|x| x.client_size()).unwrap_or((st.desc.width, st.desc.height));
        if w == 0 {
            w = cw.max(8);
        }
        if h == 0 {
            h = ch.max(8);
        }
    }
    let count = if count == 0 { st.desc.buffer_count } else { count.clamp(2, 16) };
    let fmt = if fmt == 0 { st.desc.format } else { fmt };
    if let Err(e) = st.sc.resize(w, h, fmt, count, &main_sync) {
        return failed!(DXGI_ERROR_INVALID_CALL, "ResizeBuffers({count}, {w}x{h}, {}): {e}", d3d12_metal::format::name(fmt));
    }
    for b in std::mem::take(&mut st.buffers) {
        Obj::release(b);
    }
    st.buffers = wrap_buffers(&st.sc);
    st.desc.width = w;
    st.desc.height = h;
    st.desc.format = fmt;
    st.desc.buffer_count = count;
    st.desc.flags = flags;
    st.source_size = (w, h);
    crate::macos::set_active_swapchain_size(w, h);
    log::info!(target: "d3d", "ResizeBuffers -> {w}x{h} {} x{count}", d3d12_metal::format::name(fmt));
    S_OK
}

unsafe extern "win64" fn resize_buffers(this: PS, count: u32, w: u32, h: u32, fmt: u32, flags: u32) -> HRESULT {
    tr!(Call::ResizeBuffers { swapchain: this as u64, count, width: w, height: h, format: fmt, flags });
    do_resize(this, count, w, h, fmt, flags)
}

unsafe extern "win64" fn resize_buffers1(this: PS, count: u32, w: u32, h: u32, fmt: u32, flags: u32, _masks: *const u32, _queues: *const *mut c_void) -> HRESULT {
    tr!(Call::ResizeBuffers { swapchain: this as u64, count, width: w, height: h, format: fmt, flags });
    do_resize(this, count, w, h, fmt, flags)
}

unsafe extern "win64" fn resize_target(_this: PS, target: *const c_void) -> HRESULT {
    if target.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    S_OK
}

unsafe extern "win64" fn get_containing_output(this: PS, out: *mut *mut c_void) -> HRESULT {
    let s = Obj::get(this);
    crate::dxgi::adapter::output_for_hwnd(s.factory as *mut Obj<Factory>, s.hwnd, out)
}

unsafe extern "win64" fn get_frame_statistics(this: PS, out: *mut crate::dxgi::adapter::FrameStatistics) -> HRESULT {
    if out.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    let s = Obj::get(this);
    let pr = s.st.lock().unwrap().present_refresh;
    let pc = s.presents.load(Ordering::Acquire);
    *out = crate::dxgi::adapter::frame_statistics(pc, pr);
    S_OK
}

unsafe extern "win64" fn get_last_present_count(this: PS, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    *out = Obj::get(this).presents.load(Ordering::Acquire);
    S_OK
}

unsafe extern "win64" fn get_desc1(this: PS, out: *mut SwapChainDesc1) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    *out = Obj::get(this).st.lock().unwrap().desc;
    S_OK
}

unsafe extern "win64" fn get_fullscreen_desc(this: PS, out: *mut FullscreenDesc) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    *out = Obj::get(this).st.lock().unwrap().fs;
    S_OK
}

unsafe extern "win64" fn get_hwnd(this: PS, out: *mut HWND) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    *out = Obj::get(this).hwnd as HWND;
    S_OK
}

unsafe extern "win64" fn is_temporary_mono_supported(_this: PS) -> BOOL {
    FALSE
}

unsafe extern "win64" fn set_background_color(this: PS, color: *const [f32; 4]) -> HRESULT {
    if color.is_null() {
        return E_INVALIDARG;
    }
    Obj::get(this).st.lock().unwrap().bg_color = *color;
    S_OK
}

unsafe extern "win64" fn get_background_color(this: PS, out: *mut [f32; 4]) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    *out = Obj::get(this).st.lock().unwrap().bg_color;
    S_OK
}

unsafe extern "win64" fn set_rotation(this: PS, rot: u32) -> HRESULT {
    if !(1..=4).contains(&rot) {
        return DXGI_ERROR_INVALID_CALL;
    }
    Obj::get(this).st.lock().unwrap().rotation = rot;
    S_OK
}

unsafe extern "win64" fn get_rotation(this: PS, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    *out = Obj::get(this).st.lock().unwrap().rotation;
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
    if w.is_null() || h.is_null() {
        return E_INVALIDARG;
    }
    let (sw, sh) = Obj::get(this).st.lock().unwrap().source_size;
    *w = sw;
    *h = sh;
    S_OK
}

unsafe extern "win64" fn set_matrix_transform(this: PS, m: *const [f32; 6]) -> HRESULT {
    if m.is_null() {
        return E_INVALIDARG;
    }
    Obj::get(this).st.lock().unwrap().matrix = *m;
    S_OK
}

unsafe extern "win64" fn get_matrix_transform(this: PS, out: *mut [f32; 6]) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    *out = Obj::get(this).st.lock().unwrap().matrix;
    S_OK
}

unsafe extern "win64" fn set_maximum_frame_latency(this: PS, n: u32) -> HRESULT {
    if n == 0 || n > 16 {
        return failed!(DXGI_ERROR_INVALID_CALL, "SetMaximumFrameLatency({n})");
    }
    let mut st = Obj::get(this).st.lock().unwrap();
    if n > st.max_latency {
        st.latency.release((n - st.max_latency) as i32);
    }
    log::debug!(target: "d3d", "SetMaximumFrameLatency({n})");
    st.max_latency = n;
    S_OK
}

/// A new handle to the latency semaphore (the caller closes it).
unsafe extern "win64" fn get_frame_latency_waitable_object(this: PS) -> HANDLE {
    let st = Obj::get(this).st.lock().unwrap();
    crate::handles::insert(st.latency.clone())
}

unsafe extern "win64" fn get_maximum_frame_latency(this: PS, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    *out = Obj::get(this).st.lock().unwrap().max_latency;
    S_OK
}

unsafe extern "win64" fn get_current_back_buffer_index(this: PS) -> u32 {
    Obj::get(this).st.lock().unwrap().sc.current as u32
}

/// Report DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_PRESENT (1) for standard
/// SDR and HDR RGB color spaces backed by CoreGraphics / `CAMetalLayer`.
unsafe extern "win64" fn check_color_space_support(_this: PS, cs: u32, out: *mut u32) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    *out = if d3d12_metal::metal::swapchain::color_space_supported(cs) { 1 } else { 0 };
    log::debug!(target: "d3d", "CheckColorSpaceSupport({cs}) -> {}", *out);
    S_OK
}

unsafe extern "win64" fn set_color_space1(this: PS, cs: u32) -> HRESULT {
    let mut st = Obj::get(this).st.lock().unwrap();
    if let Err(e) = st.sc.set_color_space(cs, &main_sync) {
        return failed!(E_INVALIDARG, "SetColorSpace1({cs}): {e}");
    }
    st.color_space = cs;
    S_OK
}

unsafe extern "win64" fn set_hdr_metadata(_this: PS, ty: u32, size: u32, _data: *const c_void) -> HRESULT {
    log::debug!(target: "d3d", "SetHDRMetaData(type {ty}, {size} bytes)");
    S_OK
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_surface_mode_allows_fullscreen_and_windowed_transitions() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let prev_headless = std::env::var_os("PORT_HEADLESS");
        std::env::set_var("PORT_HEADLESS", "1");
        crate::macos::with_single_surface_mode(true, || unsafe {
            let mut factory: *mut c_void = std::ptr::null_mut();
            let mut device: *mut c_void = std::ptr::null_mut();
            let mut queue: *mut c_void = std::ptr::null_mut();
            assert_eq!(crate::dxgi::CreateDXGIFactory1(&crate::dxgi::IID_IDXGIFactory1, &mut factory), S_OK);
            assert_eq!(
                crate::d3d12::D3D12CreateDevice(std::ptr::null_mut(), 0xb000, &crate::d3d12::vtables::IID_ID3D12Device, &mut device),
                S_OK
            );
            let qd = d3d12_metal::types::CommandQueueDesc { Type: 0, Priority: 0, Flags: 0, NodeMask: 0 };
            let create_q: unsafe extern "win64" fn(*mut c_void, *const d3d12_metal::types::CommandQueueDesc, *const Guid, *mut *mut c_void) -> HRESULT =
                std::mem::transmute((*(device as *const *const usize)).add(8).read());
            assert_eq!(create_q(device, &qd, &crate::d3d12::vtables::IID_ID3D12CommandQueue, &mut queue), S_OK);

            let desc = SwapChainDesc1 {
                width: 1600,
                height: 736,
                format: 28,
                sample_count: 1,
                buffer_usage: 0x20,
                buffer_count: 2,
                swap_effect: 4,
                ..Default::default()
            };
            let mut sc: *mut c_void = std::ptr::null_mut();
            assert_eq!(create(factory as *mut Obj<Factory>, queue, 0, desc, None, &mut sc), S_OK);
            let sc_obj = sc as PS;

            // Default windowed -> GetFullscreenState FALSE
            let mut is_fs: BOOL = TRUE;
            let mut target: *mut c_void = std::ptr::null_mut();
            assert_eq!(get_fullscreen_state(sc_obj, &mut is_fs, &mut target), S_OK);
            assert_eq!(is_fs, FALSE);
            assert!(target.is_null());

            // Transition to Fullscreen succeeds (S_OK, no 0x887a0022!) and GetFullscreenState matches
            assert_eq!(set_fullscreen_state(sc_obj, TRUE, std::ptr::null_mut()), S_OK);
            assert_eq!(get_fullscreen_state(sc_obj, &mut is_fs, &mut target), S_OK);
            assert_eq!(is_fs, TRUE);
            assert!(!target.is_null());
            com_release(target);

            let mut legacy = SwapChainDesc::default();
            assert_eq!(get_desc(sc_obj, &mut legacy), S_OK);
            assert_eq!(legacy.windowed, FALSE);
            assert_eq!((legacy.width, legacy.height), (1600, 736));

            // Transition back to Windowed / Borderless succeeds cleanly
            assert_eq!(set_fullscreen_state(sc_obj, FALSE, std::ptr::null_mut()), S_OK);
            assert_eq!(get_fullscreen_state(sc_obj, &mut is_fs, &mut target), S_OK);
            assert_eq!(is_fs, FALSE);
            assert!(target.is_null());
            assert_eq!(get_desc(sc_obj, &mut legacy), S_OK);
            assert_eq!(legacy.windowed, TRUE);

            com_release(sc);
            com_release(queue);
            com_release(device);
            com_release(factory);
        });
        match prev_headless {
            Some(v) => std::env::set_var("PORT_HEADLESS", v),
            None => std::env::remove_var("PORT_HEADLESS"),
        }
    }

    #[test]
    fn desktop_mode_allows_fullscreen_and_windowed_transitions() {
        if d3d12_metal::metal::device().is_none() {
            return;
        }
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let prev_headless = std::env::var_os("PORT_HEADLESS");
        std::env::set_var("PORT_HEADLESS", "1");
        crate::macos::with_single_surface_mode(false, || unsafe {
            let mut factory: *mut c_void = std::ptr::null_mut();
            let mut device: *mut c_void = std::ptr::null_mut();
            let mut queue: *mut c_void = std::ptr::null_mut();
            assert_eq!(crate::dxgi::CreateDXGIFactory1(&crate::dxgi::IID_IDXGIFactory1, &mut factory), S_OK);
            assert_eq!(
                crate::d3d12::D3D12CreateDevice(std::ptr::null_mut(), 0xb000, &crate::d3d12::vtables::IID_ID3D12Device, &mut device),
                S_OK
            );
            let qd = d3d12_metal::types::CommandQueueDesc { Type: 0, Priority: 0, Flags: 0, NodeMask: 0 };
            let create_q: unsafe extern "win64" fn(*mut c_void, *const d3d12_metal::types::CommandQueueDesc, *const Guid, *mut *mut c_void) -> HRESULT =
                std::mem::transmute((*(device as *const *const usize)).add(8).read());
            assert_eq!(create_q(device, &qd, &crate::d3d12::vtables::IID_ID3D12CommandQueue, &mut queue), S_OK);

            let desc = SwapChainDesc1 {
                width: 1280,
                height: 720,
                format: 28,
                sample_count: 1,
                buffer_usage: 0x20,
                buffer_count: 2,
                swap_effect: 4,
                ..Default::default()
            };
            let mut sc: *mut c_void = std::ptr::null_mut();
            assert_eq!(create(factory as *mut Obj<Factory>, queue, 0, desc, None, &mut sc), S_OK);
            let sc_obj = sc as PS;

            let mut is_fs: BOOL = TRUE;
            let mut target: *mut c_void = std::ptr::null_mut();
            assert_eq!(get_fullscreen_state(sc_obj, &mut is_fs, &mut target), S_OK);
            assert_eq!(is_fs, FALSE);
            assert!(target.is_null());

            // Desktop mode SetFullscreenState(TRUE) now returns S_OK and exposes containing IDXGIOutput.
            assert_eq!(set_fullscreen_state(sc_obj, TRUE, std::ptr::null_mut()), S_OK);
            assert_eq!(get_fullscreen_state(sc_obj, &mut is_fs, &mut target), S_OK);
            assert_eq!(is_fs, TRUE);
            assert!(!target.is_null());
            com_release(target);

            let mut legacy = SwapChainDesc::default();
            assert_eq!(get_desc(sc_obj, &mut legacy), S_OK);
            assert_eq!(legacy.windowed, FALSE);
            assert_eq!((legacy.width, legacy.height), (1280, 720));

            // Transition back to windowed restores state cleanly.
            assert_eq!(set_fullscreen_state(sc_obj, FALSE, std::ptr::null_mut()), S_OK);
            assert_eq!(get_fullscreen_state(sc_obj, &mut is_fs, &mut target), S_OK);
            assert_eq!(is_fs, FALSE);
            assert!(target.is_null());
            assert_eq!(get_desc(sc_obj, &mut legacy), S_OK);
            assert_eq!(legacy.windowed, TRUE);

            com_release(sc);
            com_release(queue);
            com_release(device);
            com_release(factory);
        });
        match prev_headless {
            Some(v) => std::env::set_var("PORT_HEADLESS", v),
            None => std::env::remove_var("PORT_HEADLESS"),
        }
    }
}
