//! IDXGIAdapter4 and IDXGIOutput6.

use super::Factory;
use crate::d3d12::com::*;
use crate::d3d12::mtl;
use crate::types::*;
use std::ffi::c_void;
use std::sync::OnceLock;

pub const IID_IDXGIAdapter: Guid = guid("2411e7e1-12ac-4ccf-bd14-9798e8534dc0");
pub const IID_IDXGIAdapter1: Guid = guid("29038f61-3839-4626-91fd-086879011a05");
pub const IID_IDXGIAdapter2: Guid = guid("0aa1ae0a-fa0e-4b84-8644-e05ff8e5acb5");
pub const IID_IDXGIAdapter3: Guid = guid("645967a4-1392-4310-a798-8053ce3e93fd");
pub const IID_IDXGIAdapter4: Guid = guid("3c8d99d1-4fbf-4181-a82c-af66bf7bd24e");
pub const IID_IDXGIOutput: Guid = guid("ae02eedb-c735-4690-8d52-5a8dc20213aa");
pub const IID_IDXGIOutput1: Guid = guid("00cddea8-939b-4b83-a340-a685226666cc");
pub const IID_IDXGIOutput2: Guid = guid("595e39d1-2724-4663-99b1-da969de28364");
pub const IID_IDXGIOutput3: Guid = guid("8a6bb301-7e7e-41f4-a8e0-5b32f7f99b18");
pub const IID_IDXGIOutput4: Guid = guid("dc7dca35-2196-414d-9f53-617884032a60");
pub const IID_IDXGIOutput5: Guid = guid("80a07424-ab52-42eb-833c-0c42fd282d98");
pub const IID_IDXGIOutput6: Guid = guid("068346e8-aaec-4b84-add7-137f513f77a1");
pub const IID_IDXGIDevice: Guid = guid("54ec77fa-1377-44e6-8c32-88fd5f44c84c");
pub const IID_IDXGIDevice1: Guid = guid("77db970f-6276-48ba-ba28-070143b4392c");
pub const IID_ID3D10Device: Guid = guid("9b7e4c0f-342c-4106-a19f-4f2704f689f0");
pub const IID_ID3D11Device: Guid = guid("db6f6ddb-ac77-4e88-8253-819df9bbf140");

pub fn register_names() {
    for (g, n) in [
        (IID_IDXGIAdapter, "IDXGIAdapter"),
        (IID_IDXGIAdapter1, "IDXGIAdapter1"),
        (IID_IDXGIAdapter2, "IDXGIAdapter2"),
        (IID_IDXGIAdapter3, "IDXGIAdapter3"),
        (IID_IDXGIAdapter4, "IDXGIAdapter4"),
        (IID_IDXGIOutput, "IDXGIOutput"),
        (IID_IDXGIOutput1, "IDXGIOutput1"),
        (IID_IDXGIOutput2, "IDXGIOutput2"),
        (IID_IDXGIOutput3, "IDXGIOutput3"),
        (IID_IDXGIOutput4, "IDXGIOutput4"),
        (IID_IDXGIOutput5, "IDXGIOutput5"),
        (IID_IDXGIOutput6, "IDXGIOutput6"),
        (IID_IDXGIDevice, "IDXGIDevice"),
        (IID_IDXGIDevice1, "IDXGIDevice1"),
        (IID_ID3D10Device, "ID3D10Device"),
        (IID_ID3D11Device, "ID3D11Device"),
    ] {
        name_iid(g, n);
    }
}

pub const VENDOR_APPLE: u32 = 0x106B;
/// Non-zero Apple PCI subsystem vendor/device identifier (`SubSysId`).
pub const SUBSYS_APPLE: u32 = 0x106B_0001;
/// Non-zero PCI hardware revision (`Revision`).
pub const REVISION_APPLE: u32 = 1;
/// WDDM 3.2 Metal 4 user-mode driver version (`32.0.15.4000`).
pub const UMD_DRIVER_VERSION_STR: &str = "32.0.15.4000";
/// Driver date in Windows Display Class registry format (`MM-DD-YYYY`).
pub const UMD_DRIVER_DATE_STR: &str = "11-01-2026";
/// Packed 64-bit WDDM 3.2 UMD driver version (`32.0.15.4000`) for `CheckInterfaceSupport`.
pub const UMD_DRIVER_VERSION: u64 = (32u64 << 48) | (0u64 << 32) | (15u64 << 16) | 4000u64;

#[cfg(target_os = "macos")]
type FullscreenToggleFn = fn(usize, bool, Option<crate::user32::RECT>) -> Option<crate::user32::RECT>;

#[cfg(target_os = "macos")]
static FULLSCREEN_TOGGLE: OnceLock<FullscreenToggleFn> = OnceLock::new();

/// Registered by `macos::appkit::init()` on the main thread so DXGI swapchains
/// can toggle an `NSWindow` between framed windowed mode and borderless fullscreen.
#[cfg(target_os = "macos")]
pub(crate) fn register_fullscreen_toggle(f: FullscreenToggleFn) {
    let _ = FULLSCREEN_TOGGLE.set(f);
}

/// Toggle native macOS borderless fullscreen for `hwnd`.
///
/// Returns the resulting client `RECT` in Win32 pixels when an `NSWindow`
/// transition occurred, or `None` when headless, in `single_surface_mode()`,
/// on iOS, or when `hwnd == 0`.
pub(crate) fn set_native_fullscreen(
    hwnd: usize,
    fullscreen: bool,
    target_rect: Option<crate::user32::RECT>,
) -> Option<crate::user32::RECT> {
    #[cfg(target_os = "macos")]
    {
        if hwnd == 0 || crate::macos::single_surface_mode() || !crate::macos::active() {
            return None;
        }
        let handler = *FULLSCREEN_TOGGLE.get()?;
        crate::macos::run_sync(move || handler(hwnd, fullscreen, target_rect)).flatten()
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (hwnd, fullscreen, target_rect);
        None
    }
}

/// `AdapterLuid` as the u64 `LUID` passed by value (LowPart | HighPart << 32).
pub fn adapter_luid() -> u64 {
    mtl::device().map(|d| d.registry_id).unwrap_or(0)
}

// ---------------------------------------------------------------------------
// IDXGIAdapter4
// ---------------------------------------------------------------------------

pub struct Adapter {
    factory: *mut Obj<Factory>,
}
unsafe impl Send for Adapter {}
unsafe impl Sync for Adapter {}

impl Drop for Adapter {
    fn drop(&mut self) {
        unsafe { Obj::release(self.factory) };
    }
}

static ADAPTER_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for Adapter {
    fn class() -> &'static Class {
        class_once(&ADAPTER_CLASS, || {
            let mut s = dxgi_object_slots::<Adapter>();
            s.extend([
                slot!("GetParent" => adapter_get_parent),
                slot!("EnumOutputs" => enum_outputs),
                slot!("GetDesc" => get_desc),
                slot!("CheckInterfaceSupport" => check_interface_support),
                slot!("GetDesc1" => get_desc1),
                slot!("GetDesc2" => get_desc2),
                slot!("RegisterHardwareContentProtectionTeardownStatusEvent" => register_hcp_teardown_event),
                slot!("UnregisterHardwareContentProtectionTeardownStatus" => unregister_hcp_teardown_event),
                slot!("QueryVideoMemoryInfo" => query_video_memory_info),
                slot!("SetVideoMemoryReservation" => set_video_memory_reservation),
                slot!("RegisterVideoMemoryBudgetChangeNotificationEvent" => register_budget_event),
                slot!("UnregisterVideoMemoryBudgetChangeNotification" => unregister_budget_event),
                slot!("GetDesc3" => get_desc3),
            ]);
            Class {
                name: "IDXGIAdapter4",
                vtbl: build_vtable("IDXGIAdapter4", &s),
                iids: &[super::IID_IDXGIObject, IID_IDXGIAdapter, IID_IDXGIAdapter1, IID_IDXGIAdapter2, IID_IDXGIAdapter3, IID_IDXGIAdapter4],
            }
        })
    }
}

type PA = *mut Obj<Adapter>;

pub unsafe fn new_adapter(factory: *mut Obj<Factory>) -> PA {
    Obj::add_ref(factory);
    Obj::new(Adapter { factory })
}

unsafe extern "win64" fn adapter_get_parent(this: PA, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    query_out(Obj::get(this).factory as *mut c_void, riid, out)
}

/// DXGI_ADAPTER_DESC3 layout (DESC/DESC1/DESC2 are prefixes of it).
#[repr(C)]
struct AdapterDesc3 {
    description: [u16; 128],
    vendor_id: u32,
    device_id: u32,
    sub_sys_id: u32,
    revision: u32,
    dedicated_video_memory: usize,
    dedicated_system_memory: usize,
    shared_system_memory: usize,
    luid: u64,
    flags: u32,
    graphics_preemption: u32,
    compute_preemption: u32,
}
const _: () = assert!(std::mem::size_of::<AdapterDesc3>() == 320);
const _: () = assert!(std::mem::offset_of!(AdapterDesc3, vendor_id) == 0x100);
const _: () = assert!(std::mem::offset_of!(AdapterDesc3, dedicated_video_memory) == 0x110);
const _: () = assert!(std::mem::offset_of!(AdapterDesc3, flags) == 304);

fn desc3() -> AdapterDesc3 {
    let d = mtl::device();
    let vram = d.map(|d| d.reported_vram()).unwrap_or(0);
    let mut desc = AdapterDesc3 {
        description: [0; 128],
        vendor_id: VENDOR_APPLE,
        device_id: d.map(|d| d.pci_device_id()).unwrap_or(0),
        sub_sys_id: SUBSYS_APPLE,
        revision: REVISION_APPLE,
        dedicated_video_memory: vram as usize,
        dedicated_system_memory: 0,
        shared_system_memory: mtl::physical_memory().saturating_sub(vram) as usize,
        luid: adapter_luid(),
        flags: 0,
        // DXGI_GRAPHICS_PREEMPTION_PRIMITIVE_BOUNDARY / DXGI_COMPUTE_PREEMPTION_DISPATCH_BOUNDARY
        graphics_preemption: 1,
        compute_preemption: 1,
    };
    unsafe { write_wstr(desc.description.as_mut_ptr(), 128, d.map(|d| d.name.as_str()).unwrap_or("Metal")) };
    desc
}

unsafe fn write_desc(out: *mut c_void, len: usize) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    let d = desc3();
    std::ptr::copy_nonoverlapping(&d as *const AdapterDesc3 as *const u8, out as *mut u8, len);
    S_OK
}

unsafe extern "win64" fn get_desc(_this: PA, out: *mut c_void) -> HRESULT {
    write_desc(out, 304)
}
unsafe extern "win64" fn get_desc1(_this: PA, out: *mut c_void) -> HRESULT {
    write_desc(out, 312)
}
unsafe extern "win64" fn get_desc2(_this: PA, out: *mut c_void) -> HRESULT {
    write_desc(out, 320)
}
unsafe extern "win64" fn get_desc3(_this: PA, out: *mut c_void) -> HRESULT {
    write_desc(out, 320)
}

/// Shared `IDXGIAdapter::CheckInterfaceSupport` implementation used by both the
/// D3D12/DXGI adapter and the D3D11 DXGI adapter wrapper.
pub unsafe fn check_interface_support_impl(iid: *const Guid, umd: *mut i64) -> HRESULT {
    let Some(g) = Guid::read(iid) else {
        if !umd.is_null() {
            std::ptr::write_unaligned(umd, 0);
        }
        return E_INVALIDARG;
    };
    if matches!(
        g,
        IID_IDXGIDevice | IID_IDXGIDevice1 | IID_ID3D10Device | IID_ID3D11Device
    ) {
        if !umd.is_null() {
            std::ptr::write_unaligned(umd, UMD_DRIVER_VERSION as i64);
        }
        log::debug!(
            target: "d3d",
            "IDXGIAdapter::CheckInterfaceSupport({g:?}) -> S_OK ({UMD_DRIVER_VERSION_STR})"
        );
        return S_OK;
    }
    if !umd.is_null() {
        std::ptr::write_unaligned(umd, 0);
    }
    log::debug!(target: "d3d", "IDXGIAdapter::CheckInterfaceSupport({g:?}) -> DXGI_ERROR_UNSUPPORTED");
    DXGI_ERROR_UNSUPPORTED
}

/// Query user-mode driver version for DXGI / D3D10 / D3D11 device interfaces.
unsafe extern "win64" fn check_interface_support(_this: PA, iid: *const Guid, umd: *mut i64) -> HRESULT {
    check_interface_support_impl(iid, umd)
}

static BUDGET_EVENTS: std::sync::Mutex<Vec<(u32, std::sync::Arc<crate::handles::Event>)>> =
    std::sync::Mutex::new(Vec::new());
static NEXT_BUDGET_COOKIE: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

unsafe extern "win64" fn register_budget_event(_this: PA, ev: HANDLE, cookie: *mut u32) -> HRESULT {
    if cookie.is_null() {
        return E_INVALIDARG;
    }
    let Some(event) = crate::handles::get_as::<crate::handles::Event>(ev) else {
        return E_INVALIDARG;
    };
    let id = NEXT_BUDGET_COOKIE.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    BUDGET_EVENTS.lock().unwrap().push((id, event));
    std::ptr::write_unaligned(cookie, id);
    log::debug!(target: "d3d", "IDXGIAdapter3::RegisterVideoMemoryBudgetChangeNotificationEvent -> cookie {id}");
    S_OK
}

unsafe extern "win64" fn unregister_budget_event(_this: PA, cookie: u32) {
    BUDGET_EVENTS.lock().unwrap().retain(|&(id, _)| id != cookie);
}

/// Signal all registered `IDXGIAdapter3::RegisterVideoMemoryBudgetChangeNotificationEvent`
/// wait handles (called when iOS memory pressure rises or a memory warning arrives).
pub fn notify_video_memory_budget_change() {
    let events: Vec<std::sync::Arc<crate::handles::Event>> = BUDGET_EVENTS
        .lock()
        .unwrap()
        .iter()
        .map(|(_, e)| e.clone())
        .collect();
    for ev in events {
        ev.set();
    }
}

static HCP_TEARDOWN_EVENTS: std::sync::Mutex<Vec<(u32, std::sync::Arc<crate::handles::Event>)>> =
    std::sync::Mutex::new(Vec::new());
static NEXT_HCP_COOKIE: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

unsafe extern "win64" fn register_hcp_teardown_event(_this: PA, ev: HANDLE, cookie: *mut u32) -> HRESULT {
    if cookie.is_null() {
        return E_INVALIDARG;
    }
    let Some(event) = crate::handles::get_as::<crate::handles::Event>(ev) else {
        return E_INVALIDARG;
    };
    let id = NEXT_HCP_COOKIE.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    HCP_TEARDOWN_EVENTS.lock().unwrap().push((id, event));
    std::ptr::write_unaligned(cookie, id);
    log::debug!(target: "d3d", "IDXGIAdapter3::RegisterHardwareContentProtectionTeardownStatusEvent -> cookie {id}");
    S_OK
}

unsafe extern "win64" fn unregister_hcp_teardown_event(_this: PA, cookie: u32) {
    HCP_TEARDOWN_EVENTS.lock().unwrap().retain(|&(id, _)| id != cookie);
}

/// Signal all registered `IDXGIAdapter3::RegisterHardwareContentProtectionTeardownStatusEvent`
/// wait handles when a protected-content session tears down.
pub fn notify_hcp_teardown() {
    let events: Vec<std::sync::Arc<crate::handles::Event>> = HCP_TEARDOWN_EVENTS
        .lock()
        .unwrap()
        .iter()
        .map(|(_, e)| e.clone())
        .collect();
    for ev in events {
        ev.set();
    }
}

#[repr(C)]
struct VideoMemoryInfo {
    budget: u64,
    current_usage: u64,
    available_for_reservation: u64,
    current_reservation: u64,
}

unsafe extern "win64" fn query_video_memory_info(_this: PA, node: u32, group: u32, out: *mut VideoMemoryInfo) -> HRESULT {
    if out.is_null() || node != 0 {
        return E_INVALIDARG;
    }
    let (budget, current_usage) = mtl::dynamic_vram_budget();
    *out = match group {
        // DXGI_MEMORY_SEGMENT_GROUP_LOCAL: everything lives here (unified memory).
        0 => VideoMemoryInfo {
            budget,
            current_usage,
            available_for_reservation: budget / 2,
            current_reservation: 0,
        },
        // NON_LOCAL: Windows reports a sysmem segment for UMA parts; we do not
        // allocate anything there.
        1 => VideoMemoryInfo { budget: 0, current_usage: 0, available_for_reservation: 0, current_reservation: 0 },
        _ => return E_INVALIDARG,
    };
    S_OK
}

unsafe extern "win64" fn set_video_memory_reservation(_this: PA, _node: u32, _group: u32, _bytes: u64) -> HRESULT {
    S_OK
}

unsafe extern "win64" fn enum_outputs(this: PA, i: u32, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    let ms = crate::macos::display::monitors();
    // Primary first, like Windows.
    let mut order: Vec<usize> = (0..ms.len()).collect();
    order.sort_by_key(|&k| !ms[k].primary);
    let Some(&k) = order.get(i as usize) else { return DXGI_ERROR_NOT_FOUND };
    Obj::add_ref(this);
    std::ptr::write_unaligned(
        out,
        Obj::new(Output {
            adapter: this,
            monitor_id: ms[k].id,
            owned_device: std::sync::Mutex::new(None),
        }) as *mut c_void,
    );
    S_OK
}

pub unsafe fn primary_output(factory: *mut Obj<Factory>, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    let a = new_adapter(factory);
    let hr = enum_outputs(a, 0, out);
    Obj::release(a);
    hr
}

/// Return the `IDXGIOutput` containing `hwnd` (or the primary output when `hwnd == 0`).
pub unsafe fn output_for_hwnd(factory: *mut Obj<Factory>, hwnd: usize, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    let idx = if hwnd != 0 {
        let ms = crate::macos::display::monitors();
        let mut order: Vec<usize> = (0..ms.len()).collect();
        order.sort_by_key(|&k| !ms[k].primary);
        let r = crate::user32::monitor::monitor_rect_for_window(hwnd);
        order.iter().position(|&k| ms[k].rect == r).unwrap_or(0) as u32
    } else {
        0
    };
    let a = new_adapter(factory);
    let hr = enum_outputs(a, idx, out);
    Obj::release(a);
    hr
}

/// If `p` is one of our `IDXGIOutput` COM objects, return its monitor rect in Win32 pixels.
pub unsafe fn output_monitor_rect(p: *mut c_void) -> Option<crate::user32::RECT> {
    let o = Obj::<Output>::from_iface(p)?;
    Some(monitor(Obj::get(o)).rect)
}

// ---------------------------------------------------------------------------
// IDXGIOutput6
// ---------------------------------------------------------------------------

pub struct Output {
    adapter: PA,
    monitor_id: u32,
    owned_device: std::sync::Mutex<Option<usize>>,
}
unsafe impl Send for Output {}
unsafe impl Sync for Output {}

impl Drop for Output {
    fn drop(&mut self) {
        let dev = self.owned_device.lock().ok().and_then(|mut g| g.take());
        if let Some(dev) = dev {
            release_monitor_ownership(self.monitor_id, dev);
        }
        unsafe { Obj::release(self.adapter) };
    }
}

static OUTPUT_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for Output {
    fn class() -> &'static Class {
        class_once(&OUTPUT_CLASS, || {
            let mut s = dxgi_object_slots::<Output>();
            s.extend([
                slot!("GetParent" => output_get_parent),
                slot!("GetDesc" => output_get_desc),
                slot!("GetDisplayModeList" => get_display_mode_list),
                slot!("FindClosestMatchingMode" => find_closest_matching_mode),
                slot!("WaitForVBlank" => wait_for_vblank),
                slot!("TakeOwnership" => take_ownership),
                slot!("ReleaseOwnership" => release_ownership),
                slot!("GetGammaControlCapabilities"),
                slot!("SetGammaControl"),
                slot!("GetGammaControl"),
                slot!("SetDisplaySurface"),
                slot!("GetDisplaySurfaceData"),
                slot!("GetFrameStatistics" => output_get_frame_statistics),
                slot!("GetDisplayModeList1" => get_display_mode_list1),
                slot!("FindClosestMatchingMode1" => find_closest_matching_mode1),
                slot!("GetDisplaySurfaceData1"),
                slot!("DuplicateOutput"),
                slot!("SupportsOverlays" => supports_overlays),
                slot!("CheckOverlaySupport" => check_overlay_support),
                slot!("CheckOverlayColorSpaceSupport" => check_overlay_color_space_support),
                slot!("DuplicateOutput1"),
                slot!("GetDesc1" => output_get_desc1),
                slot!("CheckHardwareCompositionSupport" => check_hardware_composition_support),
            ]);
            Class {
                name: "IDXGIOutput6",
                vtbl: build_vtable("IDXGIOutput6", &s),
                iids: &[
                    super::IID_IDXGIObject,
                    IID_IDXGIOutput,
                    IID_IDXGIOutput1,
                    IID_IDXGIOutput2,
                    IID_IDXGIOutput3,
                    IID_IDXGIOutput4,
                    IID_IDXGIOutput5,
                    IID_IDXGIOutput6,
                ],
            }
        })
    }
}

type PO = *mut Obj<Output>;

fn monitor(o: &Output) -> crate::macos::display::Monitor {
    let ms = crate::macos::display::monitors();
    ms.iter().find(|m| m.id == o.monitor_id).cloned().unwrap_or_else(|| ms[0].clone())
}

unsafe extern "win64" fn output_get_parent(this: PO, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    query_out(Obj::get(this).adapter as *mut c_void, riid, out)
}

#[repr(C)]
struct OutputDesc1 {
    device_name: [u16; 32],
    desktop: crate::user32::RECT,
    attached: BOOL,
    rotation: u32,
    monitor: usize,
    bits_per_color: u32,
    color_space: u32,
    red: [f32; 2],
    green: [f32; 2],
    blue: [f32; 2],
    white: [f32; 2],
    min_lum: f32,
    max_lum: f32,
    max_full_frame_lum: f32,
}
const _: () = assert!(std::mem::offset_of!(OutputDesc1, monitor) == 88);
const _: () = assert!(std::mem::size_of::<OutputDesc1>() == 152);

fn output_desc1(o: &Output) -> OutputDesc1 {
    let m = monitor(o);
    let mut d = OutputDesc1 {
        device_name: [0; 32],
        desktop: m.rect,
        attached: TRUE,
        rotation: 1, // DXGI_MODE_ROTATION_IDENTITY
        monitor: crate::user32::monitor::hmonitor_for_id(m.id),
        bits_per_color: 8,
        // DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P709: we present SDR sRGB.
        color_space: 0,
        red: [0.64, 0.33],
        green: [0.30, 0.60],
        blue: [0.15, 0.06],
        white: [0.3127, 0.3290],
        min_lum: 0.5,
        max_lum: 270.0,
        max_full_frame_lum: 270.0,
    };
    let idx = crate::macos::display::monitors().iter().position(|x| x.id == m.id).unwrap_or(0) + 1;
    unsafe { write_wstr(d.device_name.as_mut_ptr(), 32, &format!("\\\\.\\DISPLAY{idx}")) };
    d
}

unsafe extern "win64" fn output_get_desc(this: PO, out: *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    let d = output_desc1(Obj::get(this));
    std::ptr::copy_nonoverlapping(&d as *const OutputDesc1 as *const u8, out as *mut u8, 96);
    S_OK
}

unsafe extern "win64" fn output_get_desc1(this: PO, out: *mut c_void) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    let d = output_desc1(Obj::get(this));
    std::ptr::copy_nonoverlapping(&d as *const OutputDesc1 as *const u8, out as *mut u8, 152);
    S_OK
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
struct ModeDesc {
    width: u32,
    height: u32,
    refresh_num: u32,
    refresh_den: u32,
    format: u32,
    scanline_ordering: u32,
    scaling: u32,
}

/// Formats we can scan out (CAMetalLayer pixel formats).
fn mode_format_supported(f: u32) -> bool {
    // R8G8B8A8_UNORM(_SRGB), B8G8R8A8_UNORM(_SRGB), R10G10B10A2_UNORM, R16G16B16A16_FLOAT
    matches!(f, 28 | 29 | 87 | 91 | 24 | 10)
}

/// The modes of a monitor:
/// - In single-surface locked mode (always on iOS, or when `PORT_LOCK_DISPLAY_MODE` is set),
///   exposes only the locked display resolution `(w, h)` at the display's refresh rates (`[60, hz]`)
///   so Fullscreen, Borderless, and Windowed all lock to the exact same display resolution.
/// - On desktop macOS, exposes the native size plus common smaller 16:9/16:10 sizes across
///   60, 120, and 240 Hz refresh rates.
fn modes(o: &Output, format: u32) -> Vec<ModeDesc> {
    let m = monitor(o);
    let (w, h) = (m.rect.width() as u32, m.rect.height() as u32);
    let hz = m.refresh_hz.max(1);
    let (sizes, rates) = if crate::macos::single_surface_mode() {
        let mut r = vec![60u32, hz];
        r.sort();
        r.dedup();
        (vec![(w, h)], r)
    } else {
        let mut sizes: Vec<(u32, u32)> = vec![
            (1280, 720),
            (1600, 900),
            (1920, 1080),
            (2560, 1440),
            (3840, 2160),
            (1280, 800),
            (1680, 1050),
            (1920, 1200),
            (2560, 1600),
        ];
        sizes.retain(|&(a, b)| a <= w && b <= h);
        sizes.push((w, h));
        sizes.sort();
        sizes.dedup();
        let mut rates = vec![60u32, 120, 240, hz];
        rates.sort();
        rates.dedup();
        (sizes, rates)
    };
    let mut out = Vec::with_capacity(sizes.len() * rates.len());
    for (width, height) in sizes {
        for &refresh_num in &rates {
            out.push(ModeDesc { width, height, refresh_num, refresh_den: 1, format, scanline_ordering: 1, scaling: 0 });
        }
    }
    out
}

unsafe fn mode_list(this: PO, format: u32, count: *mut u32, out: *mut u8, stride: usize) -> HRESULT {
    if count.is_null() {
        return E_INVALIDARG;
    }
    if !mode_format_supported(format) {
        *count = 0;
        return S_OK;
    }
    let v = modes(Obj::get(this), format);
    if out.is_null() {
        *count = v.len() as u32;
        return S_OK;
    }
    let n = (*count as usize).min(v.len());
    for (i, m) in v.iter().take(n).enumerate() {
        std::ptr::write_bytes(out.add(i * stride), 0, stride);
        std::ptr::copy_nonoverlapping(m as *const ModeDesc as *const u8, out.add(i * stride), 28);
    }
    *count = n as u32;
    if n < v.len() {
        DXGI_ERROR_MORE_DATA
    } else {
        S_OK
    }
}

unsafe extern "win64" fn get_display_mode_list(this: PO, format: u32, _flags: u32, count: *mut u32, out: *mut u8) -> HRESULT {
    mode_list(this, format, count, out, 28)
}

unsafe extern "win64" fn get_display_mode_list1(this: PO, format: u32, _flags: u32, count: *mut u32, out: *mut u8) -> HRESULT {
    mode_list(this, format, count, out, 32)
}

unsafe fn closest(this: PO, want: *const ModeDesc, out: *mut u8, stride: usize) -> HRESULT {
    if want.is_null() || out.is_null() {
        return E_INVALIDARG;
    }
    let w = *want;
    let fmt = if w.format == 0 { 28 } else { w.format };
    let v = modes(Obj::get(this), fmt);
    let best = v
        .iter()
        .min_by_key(|m| {
            let dw = (m.width as i64 - w.width as i64).abs();
            let dh = (m.height as i64 - w.height as i64).abs();
            if w.width == 0 || w.height == 0 {
                -((m.width * m.height) as i64)
            } else {
                dw + dh
            }
        })
        .copied()
        .unwrap_or_default();
    std::ptr::write_bytes(out, 0, stride);
    std::ptr::copy_nonoverlapping(&best as *const ModeDesc as *const u8, out, 28);
    S_OK
}

unsafe extern "win64" fn find_closest_matching_mode(this: PO, want: *const ModeDesc, out: *mut u8, _dev: *mut c_void) -> HRESULT {
    closest(this, want, out, 28)
}

unsafe extern "win64" fn find_closest_matching_mode1(this: PO, want: *const ModeDesc, out: *mut u8, _dev: *mut c_void) -> HRESULT {
    closest(this, want, out, 32)
}

const DXGI_ERROR_NOT_CURRENTLY_AVAILABLE: HRESULT = 0x887A_0022u32 as i32;

/// Per-output logical ownership table (`monitor_id -> (device_ptr, exclusive)`).
///
/// On Windows 10/11 flip-model swapchains (`DXGI_SWAP_EFFECT_FLIP_DISCARD` /
/// `FLIP_SEQUENTIAL`), `IDXGIOutput::TakeOwnership` / `ReleaseOwnership` coordinate
/// logical output ownership between devices while fullscreen presentation runs via
/// DWM borderless fullscreen (`IDXGISwapChain::SetFullscreenState`).
fn output_owners() -> &'static std::sync::Mutex<std::collections::HashMap<u32, (usize, bool)>> {
    static OWNERS: OnceLock<std::sync::Mutex<std::collections::HashMap<u32, (usize, bool)>>> =
        OnceLock::new();
    OWNERS.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn release_monitor_ownership(monitor_id: u32, dev: usize) {
    if let Ok(mut map) = output_owners().lock() {
        if map.get(&monitor_id).is_some_and(|&(owner, _)| owner == dev) {
            map.remove(&monitor_id);
        }
    }
}

/// Hardware VBlank synchronization via CoreVideo `CVDisplayLink` on macOS, with
/// monotonic refresh-phase pacing when no physical display is attached (headless/CI/iOS).
mod vblank {
    #[cfg(target_os = "macos")]
    use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
    #[cfg(target_os = "macos")]
    use std::sync::{Arc, Condvar, Mutex, OnceLock};
    use std::time::Duration;

    fn pace_monotonic(hz: u32) {
        let period_ns = (1_000_000_000u64 / hz.max(1) as u64).max(1_000_000);
        let now_ns = crate::kernel32::time::interrupt_time_100ns().saturating_mul(100);
        let rem_ns = period_ns - (now_ns % period_ns);
        std::thread::sleep(Duration::from_nanos(rem_ns));
    }

    #[cfg(target_os = "macos")]
    mod mac {
        use super::*;
        use std::collections::HashMap;
        use std::ffi::c_void;

        #[link(name = "CoreVideo", kind = "framework")]
        extern "C" {
            fn CVDisplayLinkCreateWithCGDisplay(
                display_id: u32,
                display_link_out: *mut *mut c_void,
            ) -> i32;
            fn CVDisplayLinkSetOutputCallback(
                display_link: *mut c_void,
                callback: unsafe extern "C" fn(
                    *mut c_void,
                    *const c_void,
                    *const c_void,
                    u64,
                    *mut u64,
                    *mut c_void,
                ) -> i32,
                user_info: *mut c_void,
            ) -> i32;
            fn CVDisplayLinkStart(display_link: *mut c_void) -> i32;
            fn CVDisplayLinkRelease(display_link: *mut c_void);
        }

        #[link(name = "CoreGraphics", kind = "framework")]
        extern "C" {
            fn CGMainDisplayID() -> u32;
        }

        struct VBlankState {
            gen: AtomicU64,
            waiters: AtomicUsize,
            mu: Mutex<u64>,
            cv: Condvar,
        }

        struct DisplayLinkEntry {
            _link: usize,
            state: Arc<VBlankState>,
        }

        unsafe extern "C" fn display_link_cb(
            _link: *mut c_void,
            _in_now: *const c_void,
            _in_output_time: *const c_void,
            _flags_in: u64,
            _flags_out: *mut u64,
            ctx: *mut c_void,
        ) -> i32 {
            let st = &*(ctx as *const VBlankState);
            let next = st.gen.fetch_add(1, Ordering::Release) + 1;
            if st.waiters.load(Ordering::Acquire) > 0 {
                if let Ok(mut g) = st.mu.lock() {
                    *g = next;
                    st.cv.notify_all();
                }
            }
            0 // kCVReturnSuccess
        }

        fn link_for_display(display_id: u32) -> Option<Arc<VBlankState>> {
            static LINKS: OnceLock<Mutex<HashMap<u32, Option<DisplayLinkEntry>>>> = OnceLock::new();
            let cache = LINKS.get_or_init(|| Mutex::new(HashMap::new()));
            let cg_id = if display_id > 1 {
                display_id
            } else {
                unsafe { CGMainDisplayID() }
            };
            if cg_id == 0 {
                return None;
            }
            let mut map = cache.lock().unwrap();
            if let Some(entry) = map.get(&cg_id) {
                return entry.as_ref().map(|e| e.state.clone());
            }

            let mut link: *mut c_void = std::ptr::null_mut();
            let mut rc = unsafe { CVDisplayLinkCreateWithCGDisplay(cg_id, &mut link) };
            if rc != 0 || link.is_null() {
                let main_id = unsafe { CGMainDisplayID() };
                if main_id != 0 && main_id != cg_id {
                    rc = unsafe { CVDisplayLinkCreateWithCGDisplay(main_id, &mut link) };
                }
            }
            if rc != 0 || link.is_null() {
                map.insert(cg_id, None);
                return None;
            }

            let state = Arc::new(VBlankState {
                gen: AtomicU64::new(0),
                waiters: AtomicUsize::new(0),
                mu: Mutex::new(0),
                cv: Condvar::new(),
            });
            // Keep a process-lifetime Arc clone for the CoreVideo callback pointer.
            let raw_ctx = Arc::into_raw(state.clone()) as *mut c_void;
            let cb_rc = unsafe { CVDisplayLinkSetOutputCallback(link, display_link_cb, raw_ctx) };
            let start_rc = if cb_rc == 0 { unsafe { CVDisplayLinkStart(link) } } else { cb_rc };
            if start_rc != 0 {
                unsafe {
                    CVDisplayLinkRelease(link);
                    drop(Arc::from_raw(raw_ctx as *const VBlankState));
                }
                map.insert(cg_id, None);
                return None;
            }
            map.insert(cg_id, Some(DisplayLinkEntry { _link: link as usize, state: state.clone() }));
            Some(state)
        }

        pub fn wait(display_id: u32, fallback_hz: u32) -> bool {
            let Some(st) = link_for_display(display_id) else {
                pace_monotonic(fallback_hz);
                return false;
            };
            st.waiters.fetch_add(1, Ordering::AcqRel);
            let start = st.gen.load(Ordering::Acquire);
            let mut guard = st.mu.lock().unwrap();
            // Ensure guard reflects at least `start` before waiting.
            if *guard < start {
                *guard = start;
            }
            let max_wait = Duration::from_micros(
                (2_500_000u64 / fallback_hz.max(1) as u64).clamp(50_000, 250_000),
            );
            let (guard, timeout) = st
                .cv
                .wait_timeout_while(guard, max_wait, |g| *g <= start)
                .unwrap_or_else(|e| e.into_inner());
            let ticked = *guard > start && !timeout.timed_out();
            drop(guard);
            st.waiters.fetch_sub(1, Ordering::AcqRel);
            ticked
        }
    }

    pub fn wait(display_id: u32, fallback_hz: u32) -> bool {
        #[cfg(target_os = "macos")]
        {
            mac::wait(display_id, fallback_hz)
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = display_id;
            pace_monotonic(fallback_hz);
            false
        }
    }
}

unsafe extern "win64" fn wait_for_vblank(this: PO) -> HRESULT {
    let m = monitor(Obj::get(this));
    let hz = m.refresh_hz.max(1);
    let _hw_vsync = vblank::wait(m.id, hz);
    S_OK
}

unsafe extern "win64" fn take_ownership(this: PO, dev: *mut c_void, excl: BOOL) -> HRESULT {
    if dev.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    let out = Obj::get(this);
    let dev_key = dev as usize;
    let exclusive = excl != 0;
    let mut map = output_owners().lock().unwrap();
    if let Some(&(owner, owner_excl)) = map.get(&out.monitor_id) {
        if owner != dev_key && (owner_excl || exclusive) {
            return DXGI_ERROR_NOT_CURRENTLY_AVAILABLE;
        }
    }
    map.insert(out.monitor_id, (dev_key, exclusive));
    *out.owned_device.lock().unwrap() = Some(dev_key);
    S_OK
}

unsafe extern "win64" fn release_ownership(this: PO) {
    let out = Obj::get(this);
    // Acquire `output_owners()` before `out.owned_device` (same lock order as `take_ownership`)
    // so concurrent TakeOwnership/ReleaseOwnership calls on the same output cannot deadlock.
    let mut map = output_owners().lock().unwrap();
    if let Some(dev) = out.owned_device.lock().unwrap().take() {
        if map.get(&out.monitor_id).is_some_and(|&(owner, _)| owner == dev) {
            map.remove(&out.monitor_id);
        }
    }
}

unsafe extern "win64" fn supports_overlays(_this: PO) -> BOOL {
    FALSE
}

unsafe extern "win64" fn check_overlay_support(_this: PO, _fmt: u32, _dev: *mut c_void, flags: *mut u32) -> HRESULT {
    if !flags.is_null() {
        *flags = 0;
    }
    S_OK
}

unsafe extern "win64" fn check_overlay_color_space_support(_this: PO, _fmt: u32, _cs: u32, _dev: *mut c_void, flags: *mut u32) -> HRESULT {
    if !flags.is_null() {
        *flags = 0;
    }
    S_OK
}

unsafe extern "win64" fn check_hardware_composition_support(_this: PO, flags: *mut u32) -> HRESULT {
    if !flags.is_null() {
        *flags = 0;
    }
    S_OK
}

/// `DXGI_FRAME_STATISTICS` (32 bytes on x86_64).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct FrameStatistics {
    pub present_count: u32,
    pub present_refresh_count: u32,
    pub sync_refresh_count: u32,
    pub sync_qpc_time: i64,
    pub sync_gpu_time: i64,
}
const _: () = assert!(std::mem::size_of::<FrameStatistics>() == 32);

static GLOBAL_PRESENTS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static LAST_PRESENT_REFRESH: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// Total DXGI `Present` calls recorded across all swapchains in this process.
#[inline]
pub fn global_present_count() -> u32 {
    GLOBAL_PRESENTS.load(std::sync::atomic::Ordering::Acquire)
}

fn current_vblank_count() -> (u32, i64) {
    let qpc = crate::kernel32::time::interrupt_time_100ns();
    let hz = crate::macos::display::monitors()
        .first()
        .map(|m| m.refresh_hz.max(60) as u64)
        .unwrap_or(120);
    // Monotonic refresh ticks scaled to the active display refresh rate in the QPC (10 MHz) timebase.
    let vblanks = (qpc * hz / 10_000_000) as u32;
    (vblanks, qpc as i64)
}

/// Called by `IDXGISwapChain::Present` to advance monotonic present and refresh counters.
pub fn record_present() -> u32 {
    use std::sync::atomic::Ordering;
    GLOBAL_PRESENTS.fetch_add(1, Ordering::AcqRel);
    let (vblanks, _) = current_vblank_count();
    let mut prev = LAST_PRESENT_REFRESH.load(Ordering::Acquire);
    loop {
        let next = prev.max(vblanks).wrapping_add(1);
        match LAST_PRESENT_REFRESH.compare_exchange_weak(prev, next, Ordering::AcqRel, Ordering::Acquire) {
            Ok(_) => return next,
            Err(cur) => prev = cur,
        }
    }
}

pub fn frame_statistics(present_count: u32, present_refresh: u32) -> FrameStatistics {
    let (vblanks, qpc) = current_vblank_count();
    let present_refresh_count = if present_refresh != 0 { present_refresh } else { vblanks };
    let sync_refresh_count = vblanks.max(present_refresh_count);
    FrameStatistics {
        present_count,
        present_refresh_count,
        sync_refresh_count,
        sync_qpc_time: qpc,
        sync_gpu_time: qpc,
    }
}

unsafe extern "win64" fn output_get_frame_statistics(_this: PO, out: *mut FrameStatistics) -> HRESULT {
    if out.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    use std::sync::atomic::Ordering;
    *out = frame_statistics(GLOBAL_PRESENTS.load(Ordering::Acquire), LAST_PRESENT_REFRESH.load(Ordering::Acquire));
    S_OK
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handles::Event;

    #[test]
    fn budget_change_notification_registers_signals_and_unregisters() {
        let ev = std::sync::Arc::new(Event::new(true, false, None));
        let h = crate::handles::insert(ev.clone());
        let mut cookie = 0u32;
        unsafe {
            assert_eq!(register_budget_event(std::ptr::null_mut(), h, &mut cookie), S_OK);
            assert!(cookie > 0);
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(h, 0), 0x102); // WAIT_TIMEOUT

            notify_video_memory_budget_change();
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(h, 0), 0); // WAIT_OBJECT_0

            ev.reset();
            unregister_budget_event(std::ptr::null_mut(), cookie);
            notify_video_memory_budget_change();
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(h, 0), 0x102); // WAIT_TIMEOUT
        }
        crate::handles::close(h);
    }

    #[test]
    fn single_surface_mode_locks_display_mode_list_to_one_resolution() {
        let _g = crate::user32::TEST_SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let d = crate::macos::display::build_displays(
            &[crate::macos::display::ScreenDesc {
                id: 777,
                frame_pt: (0.0, 0.0, 956.0, 440.0),
                visible_pt: (0.0, 0.0, 956.0, 440.0),
                backing_scale: 1600.0 / 956.0,
                refresh_hz: 120,
                name: "iOS Locked".into(),
            }],
            0.0,
            true,
        );
        crate::macos::display::with_test_displays(d, || {
            crate::macos::with_single_surface_mode(true, || {
                let mut f_ptr: *mut c_void = std::ptr::null_mut();
                let mut o_ptr: *mut c_void = std::ptr::null_mut();
                unsafe {
                    assert_eq!(crate::dxgi::CreateDXGIFactory1(&crate::dxgi::IID_IDXGIFactory1, &mut f_ptr), S_OK);
                    assert_eq!(primary_output(f_ptr as *mut Obj<Factory>, &mut o_ptr), S_OK);
                    let o = Obj::get(o_ptr as PO);
                    let list = modes(o, 28);
                    assert_eq!(list.len(), 2); // 60 Hz and 120 Hz at the single locked 1600x736 resolution
                    for m in &list {
                        assert_eq!((m.width, m.height), (1600, 736));
                    }
                    assert_eq!(list[0].refresh_num, 60);
                    assert_eq!(list[1].refresh_num, 120);
                    com_release(o_ptr);
                    com_release(f_ptr);
                }
            });
        });
    }

    #[test]
    fn check_interface_support_and_adapter_pci_ids() {
        let d = desc3();
        assert_eq!(d.vendor_id, VENDOR_APPLE);
        assert_eq!(d.sub_sys_id, SUBSYS_APPLE);
        assert_ne!(d.sub_sys_id & 0xFFFF, 0);
        assert_ne!(d.sub_sys_id >> 16, 0);
        assert_eq!(d.revision, REVISION_APPLE);
        assert_ne!(d.revision, 0);

        unsafe {
            // Supported DXGI/D3D10/D3D11 device IIDs return packed WDDM 3.2 UMD driver version 32.0.15.4000.
            for iid in [IID_IDXGIDevice, IID_IDXGIDevice1, IID_ID3D10Device, IID_ID3D11Device] {
                let mut raw = [0xAAu8; 16];
                let unaligned_umd = raw.as_mut_ptr().add(1) as *mut i64;
                assert_eq!(check_interface_support(std::ptr::null_mut(), &iid, unaligned_umd), S_OK);
                let umd = std::ptr::read_unaligned(unaligned_umd) as u64;
                assert_eq!(umd, UMD_DRIVER_VERSION);
                assert_eq!(
                    (
                        (umd >> 48) & 0xFFFF,
                        (umd >> 32) & 0xFFFF,
                        (umd >> 16) & 0xFFFF,
                        umd & 0xFFFF,
                    ),
                    (32, 0, 15, 4000)
                );
                // Null UMD out-pointer is also valid.
                assert_eq!(check_interface_support(std::ptr::null_mut(), &iid, std::ptr::null_mut()), S_OK);
            }

            // Unsupported IID returns DXGI_ERROR_UNSUPPORTED and zeroes umd.
            let mut umd = -1i64;
            assert_eq!(
                check_interface_support(std::ptr::null_mut(), &IID_IDXGIAdapter, &mut umd),
                DXGI_ERROR_UNSUPPORTED
            );
            assert_eq!(umd, 0);

            // Null IID returns E_INVALIDARG and zeroes umd.
            let mut umd2 = -1i64;
            assert_eq!(
                check_interface_support(std::ptr::null_mut(), std::ptr::null(), &mut umd2),
                E_INVALIDARG
            );
            assert_eq!(umd2, 0);
        }
    }

    #[test]
    fn hcp_teardown_notification_registers_signals_and_unregisters() {
        let ev = std::sync::Arc::new(Event::new(true, false, None));
        let h = crate::handles::insert(ev.clone());
        let mut cookie = 0u32;
        unsafe {
            assert_eq!(register_hcp_teardown_event(std::ptr::null_mut(), std::ptr::null_mut(), &mut cookie), E_INVALIDARG);
            assert_eq!(register_hcp_teardown_event(std::ptr::null_mut(), h, std::ptr::null_mut()), E_INVALIDARG);
            assert_eq!(register_hcp_teardown_event(std::ptr::null_mut(), h, &mut cookie), S_OK);
            assert!(cookie > 0);
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(h, 0), 0x102);

            notify_hcp_teardown();
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(h, 0), 0);

            ev.reset();
            unregister_hcp_teardown_event(std::ptr::null_mut(), cookie);
            notify_hcp_teardown();
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(h, 0), 0x102);
        }
        crate::handles::close(h);
    }

    #[test]
    fn output_take_ownership_and_wait_for_vblank() {
        unsafe {
            let mut f_ptr: *mut c_void = std::ptr::null_mut();
            let mut o1_ptr: *mut c_void = std::ptr::null_mut();
            let mut o2_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(crate::dxgi::CreateDXGIFactory1(&crate::dxgi::IID_IDXGIFactory1, &mut f_ptr), S_OK);
            assert_eq!(primary_output(f_ptr as *mut Obj<Factory>, &mut o1_ptr), S_OK);
            assert_eq!(primary_output(f_ptr as *mut Obj<Factory>, &mut o2_ptr), S_OK);

            let dev_a = 0xA000usize as *mut c_void;
            let dev_b = 0xB000usize as *mut c_void;
            assert_eq!(take_ownership(o1_ptr as PO, std::ptr::null_mut(), TRUE), DXGI_ERROR_INVALID_CALL);
            assert_eq!(take_ownership(o1_ptr as PO, dev_a, TRUE), S_OK);
            // Same device re-acquiring succeeds; another device fails with NOT_CURRENTLY_AVAILABLE.
            assert_eq!(take_ownership(o1_ptr as PO, dev_a, TRUE), S_OK);
            assert_eq!(take_ownership(o2_ptr as PO, dev_b, FALSE), DXGI_ERROR_NOT_CURRENTLY_AVAILABLE);
            release_ownership(o1_ptr as PO);
            assert_eq!(take_ownership(o2_ptr as PO, dev_b, TRUE), S_OK);
            release_ownership(o2_ptr as PO);

            // WaitForVBlank returns S_OK synchronized to display VBlank (CVDisplayLink on macOS).
            assert_eq!(wait_for_vblank(o1_ptr as PO), S_OK);

            // Concurrent TakeOwnership / ReleaseOwnership on the same output must never deadlock.
            let o1_raw = o1_ptr as usize;
            let dev_a_raw = dev_a as usize;
            let barrier = std::sync::Arc::new(std::sync::Barrier::new(4));
            let mut threads = Vec::new();
            for tid in 0..4usize {
                let b = barrier.clone();
                threads.push(std::thread::spawn(move || {
                    let o = o1_raw as PO;
                    let dev = dev_a_raw as *mut c_void;
                    b.wait();
                    for _ in 0..500 {
                        if tid % 2 == 0 {
                            let _ = take_ownership(o, dev, TRUE);
                        } else {
                            release_ownership(o);
                        }
                    }
                }));
            }
            for t in threads {
                t.join().expect("concurrent TakeOwnership/ReleaseOwnership thread panicked");
            }
            release_ownership(o1_ptr as PO);
            // After all threads finish and ownership is released, another device can take ownership.
            assert_eq!(take_ownership(o2_ptr as PO, dev_b, TRUE), S_OK);
            release_ownership(o2_ptr as PO);

            com_release(o2_ptr);
            com_release(o1_ptr);
            com_release(f_ptr);
        }
    }
}
