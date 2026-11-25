//! dxgi.dll: factory, adapter and output objects on top of Metal.
//!
//! One adapter is reported: the system default `MTLDevice`.
//!
//! **What the adapter reports, and why** (checked against the game's
//! `d3d12_init_and_check` @ 0x14208d490 and `gpu_feature_validator`
//! @ 0x142070370):
//! * `VendorId = 0x106B` (Apple). The game maps 0x1002/0x10DE/0x8086 to its
//!   AMD/NVIDIA/Intel paths (`param_1+0xa8` = 2/1/3) and only runs the
//!   NVAPI driver-version gate for NVIDIA; any other vendor is "unknown"
//!   (0), which skips NVAPI/AGS-style vendor extensions that cannot work
//!   here. Streamline's DLSS/Reflex plugins also require an NVIDIA adapter,
//!   so they stay disabled. Reporting the truth is both honest and the
//!   safest path.
//! * `DeviceId` = a stable 16-bit hash of the Metal device name (no PCI
//!   identity exists; drivers/caches keyed on DeviceId stay consistent
//!   across runs), `SubSysId = 0`, `Revision = 0`.
//! * `DedicatedVideoMemory = recommendedMaxWorkingSetSize` clamped to
//!   [5.375 GiB, 48 GiB] (`AW2_VRAM_MB` overrides): unified memory, so the
//!   amount the GPU can use without paging, but capped so the game's pool
//!   sizing stays sane on 512 GB machines. The game warns when this is
//!   <= 0x157bfffff bytes (~5.4 GB); `DedicatedSystemMemory = 0`,
//!   `SharedSystemMemory = RAM - DedicatedVideoMemory`.
//! * `AdapterLuid` = the Metal registry ID (stable per boot).
//! * Tearing (`DXGI_FEATURE_PRESENT_ALLOW_TEARING`) is reported FALSE for
//!   M3: presentation always goes through `CAMetalLayer` with display sync;
//!   revisit with `displaySyncEnabled = NO` once presentation is solid.

pub mod adapter;
pub mod swapchain;

use crate::d3d12::com::*;
use crate::types::*;
use std::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;

pub static EXPORTS: &[crate::Export] =
    exports![CreateDXGIFactory, CreateDXGIFactory1, CreateDXGIFactory2, DXGIGetDebugInterface1, DXGIDeclareAdapterRemovalSupport];
pub static ORDINALS: &[(u16, &str)] = &[];

pub const IID_IDXGIObject: Guid = guid("aec22fb8-76f3-4639-9be0-28eb43a67a2e");
pub const IID_IDXGIFactory: Guid = guid("7b7166ec-21c7-44ae-b21a-c9ae321ae369");
pub const IID_IDXGIFactory1: Guid = guid("770aae78-f26f-4dba-a829-253c83d1b387");
pub const IID_IDXGIFactory2: Guid = guid("50c83a1c-e072-4c48-87b0-3630fa36a6d0");
pub const IID_IDXGIFactory3: Guid = guid("25483823-cd46-4c7d-86ca-47aa95b837bd");
pub const IID_IDXGIFactory4: Guid = guid("1bc6ea02-ef36-464f-bf0c-21ca39e5168a");
pub const IID_IDXGIFactory5: Guid = guid("7632e1f5-ee65-4dca-87fd-84cd75f8838d");
pub const IID_IDXGIFactory6: Guid = guid("c1b6694f-ff09-44a9-b03c-77900a0a1d17");
pub const IID_IDXGIFactory7: Guid = guid("a4966eed-76db-44da-84c1-ee9a7afb20a8");
pub const IID_IDXGIDebug: Guid = guid("119e7452-de9e-40fe-8806-88f90c12b441");
pub const IID_IDXGIDebug1: Guid = guid("c5a05f0c-16f2-4adf-9f4d-a8c4d58ac550");
pub const IID_IDXGIInfoQueue: Guid = guid("d67441c7-672a-476f-9e82-cd55b44949ce");

const DXGI_CREATE_FACTORY_DEBUG: u32 = 1;

pub fn register_names() {
    static ONCE: OnceLock<()> = OnceLock::new();
    ONCE.get_or_init(|| {
        for (g, n) in [
            (IID_IDXGIObject, "IDXGIObject"),
            (IID_IDXGIFactory, "IDXGIFactory"),
            (IID_IDXGIFactory1, "IDXGIFactory1"),
            (IID_IDXGIFactory2, "IDXGIFactory2"),
            (IID_IDXGIFactory3, "IDXGIFactory3"),
            (IID_IDXGIFactory4, "IDXGIFactory4"),
            (IID_IDXGIFactory5, "IDXGIFactory5"),
            (IID_IDXGIFactory6, "IDXGIFactory6"),
            (IID_IDXGIFactory7, "IDXGIFactory7"),
            (IID_IDXGIDebug, "IDXGIDebug"),
            (IID_IDXGIDebug1, "IDXGIDebug1"),
            (IID_IDXGIInfoQueue, "IDXGIInfoQueue"),
        ] {
            name_iid(g, n);
        }
        adapter::register_names();
        swapchain::register_names();
    });
}

// ---------------------------------------------------------------------------
// Exports
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn CreateDXGIFactory(riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    CreateDXGIFactory2(0, riid, out)
}

pub unsafe extern "win64" fn CreateDXGIFactory1(riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    CreateDXGIFactory2(0, riid, out)
}

pub unsafe extern "win64" fn CreateDXGIFactory2(flags: u32, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    register_names();
    if out.is_null() {
        return E_POINTER;
    }
    *out = std::ptr::null_mut();
    if flags & DXGI_CREATE_FACTORY_DEBUG != 0 {
        log::info!(target: "d3d", "CreateDXGIFactory2: DXGI_CREATE_FACTORY_DEBUG ignored (no debug layer)");
    }
    let f = Obj::new(Factory { flags, window: AtomicUsize::new(0) });
    let hr = query_out(f as *mut c_void, riid, out);
    Obj::release(f);
    log::info!(target: "d3d", "CreateDXGIFactory2(flags {flags:#x}, {:?}) -> {hr:#x}", Guid::read(riid).unwrap_or_default());
    hr
}

/// Native-unsupported: Windows Graphics Tools optional feature (`dxgidebug.dll`)
/// is not present on macOS/iOS. Matches Windows behavior when Graphics Tools are
/// not installed by zeroing `*ppDebug` and returning `E_NOINTERFACE`
/// (Microsoft Learn: `DXGIGetDebugInterface1`, matching `D3D12GetDebugInterface`).
pub unsafe extern "win64" fn DXGIGetDebugInterface1(
    _flags: u32,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    register_names();
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    if out.is_null() || riid.is_null() {
        return E_INVALIDARG;
    }
    log::info!(
        target: "d3d",
        "DXGIGetDebugInterface1({:?}) -> E_NOINTERFACE (no DXGI debug layer installed)",
        Guid::read(riid).unwrap_or_default()
    );
    E_NOINTERFACE
}

pub unsafe extern "win64" fn DXGIDeclareAdapterRemovalSupport() -> HRESULT {
    S_OK
}

// ---------------------------------------------------------------------------
// IDXGIFactory7
// ---------------------------------------------------------------------------

pub struct Factory {
    pub flags: u32,
    window: AtomicUsize,
}

static FACTORY_CLASS: OnceLock<Class> = OnceLock::new();

impl ComClass for Factory {
    fn class() -> &'static Class {
        class_once(&FACTORY_CLASS, || {
            type F = Factory;
            let mut s = dxgi_object_slots::<F>();
            s.extend([
                slot!("GetParent" => factory_get_parent),
                slot!("EnumAdapters" => enum_adapters),
                slot!("MakeWindowAssociation" => make_window_association),
                slot!("GetWindowAssociation" => get_window_association),
                slot!("CreateSwapChain" => swapchain::factory_create_swap_chain),
                slot!("CreateSoftwareAdapter" => create_software_adapter),
                slot!("EnumAdapters1" => enum_adapters),
                slot!("IsCurrent" => is_current),
                slot!("IsWindowedStereoEnabled" => is_windowed_stereo_enabled),
                slot!("CreateSwapChainForHwnd" => swapchain::factory_create_swap_chain_for_hwnd),
                slot!("CreateSwapChainForCoreWindow" => create_swap_chain_for_core_window),
                slot!("GetSharedResourceAdapterLuid" => get_shared_resource_adapter_luid),
                slot!("RegisterStereoStatusWindow" => register_stereo_status_window),
                slot!("RegisterStereoStatusEvent" => register_stereo_status_event),
                slot!("UnregisterStereoStatus" => unregister_stereo_status),
                slot!("RegisterOcclusionStatusWindow" => register_occlusion_status_window),
                slot!("RegisterOcclusionStatusEvent" => register_occlusion_status_event),
                slot!("UnregisterOcclusionStatus" => unregister_occlusion_status),
                slot!("CreateSwapChainForComposition" => create_swap_chain_for_composition),
                slot!("GetCreationFlags" => get_creation_flags),
                slot!("EnumAdapterByLuid" => enum_adapter_by_luid),
                slot!("EnumWarpAdapter" => enum_warp_adapter),
                slot!("CheckFeatureSupport" => check_feature_support),
                slot!("EnumAdapterByGpuPreference" => enum_adapter_by_gpu_preference),
                slot!("RegisterAdaptersChangedEvent" => register_adapters_changed_event),
                slot!("UnregisterAdaptersChangedEvent" => unregister_adapters_changed_event),
            ]);
            Class {
                name: "IDXGIFactory7",
                vtbl: build_vtable("IDXGIFactory7", &s),
                iids: &[
                    IID_IDXGIObject,
                    IID_IDXGIFactory,
                    IID_IDXGIFactory1,
                    IID_IDXGIFactory2,
                    IID_IDXGIFactory3,
                    IID_IDXGIFactory4,
                    IID_IDXGIFactory5,
                    IID_IDXGIFactory6,
                    IID_IDXGIFactory7,
                ],
            }
        })
    }
}

type PF = *mut Obj<Factory>;

unsafe extern "win64" fn factory_get_parent(_this: PF, _riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    E_NOINTERFACE
}

unsafe extern "win64" fn enum_adapters(this: PF, i: u32, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    *out = std::ptr::null_mut();
    if i != 0 || crate::d3d12::mtl::device().is_none() {
        return DXGI_ERROR_NOT_FOUND;
    }
    *out = adapter::new_adapter(this) as *mut c_void;
    S_OK
}

unsafe extern "win64" fn enum_adapter_by_gpu_preference(this: PF, i: u32, pref: u32, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    *out = std::ptr::null_mut();
    log::debug!(target: "d3d", "EnumAdapterByGpuPreference({i}, pref {pref})");
    if i != 0 || crate::d3d12::mtl::device().is_none() {
        return DXGI_ERROR_NOT_FOUND;
    }
    let a = adapter::new_adapter(this);
    let hr = query_out(a as *mut c_void, riid, out);
    Obj::release(a);
    hr
}

unsafe extern "win64" fn enum_adapter_by_luid(this: PF, luid: u64, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if out.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    *out = std::ptr::null_mut();
    if crate::d3d12::mtl::device().is_none() || luid != adapter::adapter_luid() {
        log::warn!(target: "d3d", "EnumAdapterByLuid({luid:#x}): no such adapter");
        return DXGI_ERROR_NOT_FOUND;
    }
    let a = adapter::new_adapter(this);
    let hr = query_out(a as *mut c_void, riid, out);
    Obj::release(a);
    hr
}

/// Native-unsupported: Microsoft WARP (`d3d10warp.dll`) is a Windows-proprietary CPU
/// software rasterizer that does not exist on macOS/iOS. Matches Windows when WARP is
/// unavailable by zeroing `*ppvAdapter` and returning `DXGI_ERROR_NOT_FOUND`
/// (Microsoft Learn: `IDXGIFactory4::EnumWarpAdapter`).
unsafe extern "win64" fn enum_warp_adapter(
    _this: PF,
    riid: *const Guid,
    out: *mut *mut c_void,
) -> HRESULT {
    if out.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    if riid.is_null() {
        return E_INVALIDARG;
    }
    log::debug!(target: "d3d", "IDXGIFactory4::EnumWarpAdapter -> DXGI_ERROR_NOT_FOUND (no WARP on Apple Silicon)");
    DXGI_ERROR_NOT_FOUND
}

/// Native-unsupported: WDDM user-mode software rasterizer DLLs (`OpenAdapter10`) do not
/// exist on macOS/Metal. Matches Windows when software adapters are unsupported by
/// returning `DXGI_ERROR_UNSUPPORTED` (Microsoft Learn: `IDXGIFactory::CreateSoftwareAdapter`).
unsafe extern "win64" fn create_software_adapter(
    _this: PF,
    module: HMODULE,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    if out.is_null() || module.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    log::debug!(target: "d3d", "IDXGIFactory::CreateSoftwareAdapter -> DXGI_ERROR_UNSUPPORTED");
    DXGI_ERROR_UNSUPPORTED
}

unsafe extern "win64" fn make_window_association(this: PF, hwnd: HWND, flags: u32) -> HRESULT {
    log::debug!(target: "d3d", "MakeWindowAssociation(hwnd {:#x}, flags {flags:#x})", hwnd as usize);
    Obj::get(this).window.store(hwnd as usize, Ordering::Relaxed);
    S_OK
}

unsafe extern "win64" fn get_window_association(this: PF, out: *mut HWND) -> HRESULT {
    if out.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    *out = Obj::get(this).window.load(Ordering::Relaxed) as HWND;
    S_OK
}

unsafe extern "win64" fn is_current(_this: PF) -> BOOL {
    TRUE
}

unsafe extern "win64" fn is_windowed_stereo_enabled(_this: PF) -> BOOL {
    FALSE
}

/// Native-unsupported: UWP `Windows.UI.Core.ICoreWindow` does not exist on macOS/iOS.
/// Returns `DXGI_ERROR_UNSUPPORTED` after validating parameters
/// (Microsoft Learn: `IDXGIFactory2::CreateSwapChainForCoreWindow`).
unsafe extern "win64" fn create_swap_chain_for_core_window(
    _this: PF,
    device: *mut c_void,
    window: *mut c_void,
    desc: *const c_void,
    _restrict: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    if out.is_null() || device.is_null() || window.is_null() || desc.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    log::debug!(target: "d3d", "IDXGIFactory2::CreateSwapChainForCoreWindow -> DXGI_ERROR_UNSUPPORTED (no UWP CoreWindow)");
    DXGI_ERROR_UNSUPPORTED
}

/// Native-unsupported: Windows DirectComposition (`IDCompositionDevice`) does not exist
/// on macOS/iOS. Returns `DXGI_ERROR_UNSUPPORTED` after validating parameters
/// (Microsoft Learn: `IDXGIFactory2::CreateSwapChainForComposition`).
unsafe extern "win64" fn create_swap_chain_for_composition(
    _this: PF,
    device: *mut c_void,
    desc: *const c_void,
    _restrict: *mut c_void,
    out: *mut *mut c_void,
) -> HRESULT {
    if !out.is_null() {
        std::ptr::write_unaligned(out, std::ptr::null_mut());
    }
    if out.is_null() || device.is_null() || desc.is_null() {
        return DXGI_ERROR_INVALID_CALL;
    }
    log::debug!(target: "d3d", "IDXGIFactory2::CreateSwapChainForComposition -> DXGI_ERROR_UNSUPPORTED (no DirectComposition)");
    DXGI_ERROR_UNSUPPORTED
}

unsafe extern "win64" fn get_shared_resource_adapter_luid(
    _this: PF,
    h: HANDLE,
    out: *mut u64,
) -> HRESULT {
    if out.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(out, 0);
    if h.is_null() {
        return E_INVALIDARG;
    }
    let valid = crate::d3d11::resources::lookup_nt_shared_resource(h).is_ok()
        || crate::d3d11::resources::lookup_legacy_shared_resource(h).is_ok();
    if valid {
        std::ptr::write_unaligned(out, adapter::adapter_luid());
        S_OK
    } else {
        E_INVALIDARG
    }
}

static COOKIE: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(1);

struct NotificationRegistry {
    occlusion_windows: Vec<(u32, usize, u32)>,
    occlusion_events: Vec<(u32, std::sync::Arc<crate::handles::Event>)>,
    stereo_windows: Vec<(u32, usize, u32)>,
    stereo_events: Vec<(u32, std::sync::Arc<crate::handles::Event>)>,
    adapters_events: Vec<(u32, std::sync::Arc<crate::handles::Event>)>,
}

fn notifications() -> &'static std::sync::Mutex<NotificationRegistry> {
    static REG: OnceLock<std::sync::Mutex<NotificationRegistry>> = OnceLock::new();
    REG.get_or_init(|| {
        std::sync::Mutex::new(NotificationRegistry {
            occlusion_windows: Vec::new(),
            occlusion_events: Vec::new(),
            stereo_windows: Vec::new(),
            stereo_events: Vec::new(),
            adapters_events: Vec::new(),
        })
    })
}

fn next_cookie() -> u32 {
    let c = COOKIE.fetch_add(1, Ordering::AcqRel);
    if c == 0 {
        COOKIE.fetch_add(1, Ordering::AcqRel)
    } else {
        c
    }
}

unsafe extern "win64" fn register_occlusion_status_window(
    _this: PF,
    hwnd: HWND,
    msg: u32,
    cookie: *mut u32,
) -> HRESULT {
    if hwnd.is_null() || cookie.is_null() {
        return E_INVALIDARG;
    }
    let id = next_cookie();
    notifications().lock().unwrap().occlusion_windows.push((id, hwnd as usize, msg));
    std::ptr::write_unaligned(cookie, id);
    S_OK
}

unsafe extern "win64" fn register_occlusion_status_event(
    _this: PF,
    ev: HANDLE,
    cookie: *mut u32,
) -> HRESULT {
    if cookie.is_null() {
        return E_INVALIDARG;
    }
    let Some(event) = crate::handles::get_as::<crate::handles::Event>(ev) else {
        return E_INVALIDARG;
    };
    let id = next_cookie();
    notifications().lock().unwrap().occlusion_events.push((id, event));
    std::ptr::write_unaligned(cookie, id);
    S_OK
}

unsafe extern "win64" fn unregister_occlusion_status(_this: PF, cookie: u32) {
    let mut reg = notifications().lock().unwrap();
    reg.occlusion_windows.retain(|&(id, _, _)| id != cookie);
    reg.occlusion_events.retain(|&(id, _)| id != cookie);
}

unsafe extern "win64" fn register_stereo_status_window(
    _this: PF,
    hwnd: HWND,
    msg: u32,
    cookie: *mut u32,
) -> HRESULT {
    if hwnd.is_null() || cookie.is_null() {
        return E_INVALIDARG;
    }
    let id = next_cookie();
    notifications().lock().unwrap().stereo_windows.push((id, hwnd as usize, msg));
    std::ptr::write_unaligned(cookie, id);
    S_OK
}

unsafe extern "win64" fn register_stereo_status_event(
    _this: PF,
    ev: HANDLE,
    cookie: *mut u32,
) -> HRESULT {
    if cookie.is_null() {
        return E_INVALIDARG;
    }
    let Some(event) = crate::handles::get_as::<crate::handles::Event>(ev) else {
        return E_INVALIDARG;
    };
    let id = next_cookie();
    notifications().lock().unwrap().stereo_events.push((id, event));
    std::ptr::write_unaligned(cookie, id);
    S_OK
}

unsafe extern "win64" fn unregister_stereo_status(_this: PF, cookie: u32) {
    let mut reg = notifications().lock().unwrap();
    reg.stereo_windows.retain(|&(id, _, _)| id != cookie);
    reg.stereo_events.retain(|&(id, _)| id != cookie);
}

unsafe extern "win64" fn register_adapters_changed_event(
    _this: PF,
    ev: HANDLE,
    cookie: *mut u32,
) -> HRESULT {
    if cookie.is_null() {
        return E_INVALIDARG;
    }
    let Some(event) = crate::handles::get_as::<crate::handles::Event>(ev) else {
        return E_INVALIDARG;
    };
    let id = next_cookie();
    notifications().lock().unwrap().adapters_events.push((id, event));
    std::ptr::write_unaligned(cookie, id);
    S_OK
}

unsafe extern "win64" fn unregister_adapters_changed_event(_this: PF, cookie: u32) -> HRESULT {
    let mut reg = notifications().lock().unwrap();
    let before = reg.adapters_events.len();
    reg.adapters_events.retain(|&(id, _)| id != cookie);
    if reg.adapters_events.len() < before {
        S_OK
    } else {
        E_INVALIDARG
    }
}

/// Signal all registered DXGI occlusion events and post registered window messages
/// (invoked from AppKit's `windowDidChangeOcclusionState:`).
pub fn notify_occlusion_status_changed() {
    let (events, windows) = {
        let reg = notifications().lock().unwrap();
        (
            reg.occlusion_events.iter().map(|(_, e)| e.clone()).collect::<Vec<_>>(),
            reg.occlusion_windows.iter().map(|&(_, h, m)| (h, m)).collect::<Vec<_>>(),
        )
    };
    for ev in events {
        ev.set();
    }
    for (hwnd, msg) in windows {
        unsafe {
            crate::user32::message::PostMessageW(hwnd as HWND, msg, 0, 0);
        }
    }
}

/// Signal all registered DXGI stereo events and post registered window messages.
pub fn notify_stereo_status_changed() {
    let (events, windows) = {
        let reg = notifications().lock().unwrap();
        (
            reg.stereo_events.iter().map(|(_, e)| e.clone()).collect::<Vec<_>>(),
            reg.stereo_windows.iter().map(|&(_, h, m)| (h, m)).collect::<Vec<_>>(),
        )
    };
    for ev in events {
        ev.set();
    }
    for (hwnd, msg) in windows {
        unsafe {
            crate::user32::message::PostMessageW(hwnd as HWND, msg, 0, 0);
        }
    }
}

/// Signal all registered `IDXGIFactory7::RegisterAdaptersChangedEvent` wait handles
/// (invoked from AppKit's `applicationDidChangeScreenParameters:`).
pub fn notify_adapters_changed() {
    let events = {
        let reg = notifications().lock().unwrap();
        reg.adapters_events.iter().map(|(_, e)| e.clone()).collect::<Vec<_>>()
    };
    for ev in events {
        ev.set();
    }
}

unsafe extern "win64" fn get_creation_flags(this: PF) -> u32 {
    Obj::get(this).flags
}

const DXGI_FEATURE_PRESENT_ALLOW_TEARING: u32 = 0;

unsafe extern "win64" fn check_feature_support(_this: PF, feature: u32, data: *mut c_void, size: u32) -> HRESULT {
    match feature {
        DXGI_FEATURE_PRESENT_ALLOW_TEARING if size == 4 && !data.is_null() => {
            *(data as *mut BOOL) = TRUE;
            S_OK
        }
        _ => {
            log::warn!(target: "d3d", "IDXGIFactory5::CheckFeatureSupport({feature}, size {size}) -> E_INVALIDARG");
            E_INVALIDARG
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handles::Event;

    unsafe fn vtbl_slot(obj: *mut c_void, slot: usize) -> *const () {
        let vt = *(obj as *const *const *const ());
        *vt.add(slot)
    }

    #[test]
    fn dxgi_native_unsupported_paths_match_windows_hresults() {
        unsafe {
            // DXGIGetDebugInterface1 -> E_NOINTERFACE (Graphics Tools absent), E_INVALIDARG on null.
            let mut dbg: *mut c_void = 1usize as *mut c_void;
            assert_eq!(DXGIGetDebugInterface1(0, &IID_IDXGIDebug1, &mut dbg), E_NOINTERFACE);
            assert!(dbg.is_null());
            assert_eq!(DXGIGetDebugInterface1(0, std::ptr::null(), &mut dbg), E_INVALIDARG);
            assert_eq!(DXGIGetDebugInterface1(0, &IID_IDXGIDebug1, std::ptr::null_mut()), E_INVALIDARG);

            let mut fac: *mut c_void = std::ptr::null_mut();
            assert_eq!(CreateDXGIFactory2(0, &IID_IDXGIFactory7, &mut fac), S_OK);
            assert!(!fac.is_null());

            // CreateSoftwareAdapter (slot 11): DXGI_ERROR_UNSUPPORTED, invalid call on null.
            let create_sw: unsafe extern "win64" fn(*mut c_void, HMODULE, *mut *mut c_void) -> HRESULT =
                std::mem::transmute(vtbl_slot(fac, 11));
            let mut adp: *mut c_void = 1usize as *mut c_void;
            assert_eq!(create_sw(fac, std::ptr::null_mut(), &mut adp), DXGI_ERROR_INVALID_CALL);
            assert!(adp.is_null());
            adp = 1usize as *mut c_void;
            assert_eq!(create_sw(fac, 0x1234usize as HMODULE, &mut adp), DXGI_ERROR_UNSUPPORTED);
            assert!(adp.is_null());

            // EnumWarpAdapter (slot 27): DXGI_ERROR_NOT_FOUND, invalid call on null out.
            let enum_warp: unsafe extern "win64" fn(*mut c_void, *const Guid, *mut *mut c_void) -> HRESULT =
                std::mem::transmute(vtbl_slot(fac, 27));
            adp = 1usize as *mut c_void;
            assert_eq!(enum_warp(fac, &adapter::IID_IDXGIAdapter1, &mut adp), DXGI_ERROR_NOT_FOUND);
            assert!(adp.is_null());
            assert_eq!(enum_warp(fac, &adapter::IID_IDXGIAdapter1, std::ptr::null_mut()), DXGI_ERROR_INVALID_CALL);

            // CreateSwapChainForCoreWindow (slot 16) & CreateSwapChainForComposition (slot 24):
            // DXGI_ERROR_UNSUPPORTED on valid args, DXGI_ERROR_INVALID_CALL on null args.
            let csc_cw: unsafe extern "win64" fn(
                *mut c_void, *mut c_void, *mut c_void, *const c_void, *mut c_void, *mut *mut c_void,
            ) -> HRESULT = std::mem::transmute(vtbl_slot(fac, 16));
            let csc_comp: unsafe extern "win64" fn(
                *mut c_void, *mut c_void, *const c_void, *mut c_void, *mut *mut c_void,
            ) -> HRESULT = std::mem::transmute(vtbl_slot(fac, 24));
            let dummy = 1usize as *mut c_void;
            let mut sc: *mut c_void = dummy;
            assert_eq!(csc_cw(fac, dummy, dummy, dummy, std::ptr::null_mut(), &mut sc), DXGI_ERROR_UNSUPPORTED);
            assert!(sc.is_null());
            assert_eq!(csc_cw(fac, std::ptr::null_mut(), dummy, dummy, std::ptr::null_mut(), &mut sc), DXGI_ERROR_INVALID_CALL);
            sc = dummy;
            assert_eq!(csc_comp(fac, dummy, dummy, std::ptr::null_mut(), &mut sc), DXGI_ERROR_UNSUPPORTED);
            assert!(sc.is_null());
            assert_eq!(csc_comp(fac, dummy, std::ptr::null(), std::ptr::null_mut(), &mut sc), DXGI_ERROR_INVALID_CALL);

            com_release(fac);
        }
    }

    #[test]
    fn dxgi_factory_status_events_and_windows_register_signal_unregister() {
        unsafe {
            let mut fac: *mut c_void = std::ptr::null_mut();
            assert_eq!(CreateDXGIFactory2(0, &IID_IDXGIFactory7, &mut fac), S_OK);

            // Slot indices on IDXGIFactory7:
            // 18: RegisterStereoStatusWindow, 19: RegisterStereoStatusEvent, 20: UnregisterStereoStatus
            // 21: RegisterOcclusionStatusWindow, 22: RegisterOcclusionStatusEvent, 23: UnregisterOcclusionStatus
            // 30: RegisterAdaptersChangedEvent, 31: UnregisterAdaptersChangedEvent
            let reg_stereo_ev: unsafe extern "win64" fn(*mut c_void, HANDLE, *mut u32) -> HRESULT =
                std::mem::transmute(vtbl_slot(fac, 19));
            let unreg_stereo: unsafe extern "win64" fn(*mut c_void, u32) =
                std::mem::transmute(vtbl_slot(fac, 20));
            let reg_occ_ev: unsafe extern "win64" fn(*mut c_void, HANDLE, *mut u32) -> HRESULT =
                std::mem::transmute(vtbl_slot(fac, 22));
            let unreg_occ: unsafe extern "win64" fn(*mut c_void, u32) =
                std::mem::transmute(vtbl_slot(fac, 23));
            let reg_adp_ev: unsafe extern "win64" fn(*mut c_void, HANDLE, *mut u32) -> HRESULT =
                std::mem::transmute(vtbl_slot(fac, 30));
            let unreg_adp: unsafe extern "win64" fn(*mut c_void, u32) -> HRESULT =
                std::mem::transmute(vtbl_slot(fac, 31));

            let ev_occ = std::sync::Arc::new(Event::new(true, false, None));
            let h_occ = crate::handles::insert(ev_occ.clone());
            let ev_st = std::sync::Arc::new(Event::new(true, false, None));
            let h_st = crate::handles::insert(ev_st.clone());
            let ev_adp = std::sync::Arc::new(Event::new(true, false, None));
            let h_adp = crate::handles::insert(ev_adp.clone());

            let (mut c_occ, mut c_st, mut c_adp) = (0u32, 0u32, 0u32);
            // Invalid event handle rejected.
            assert_eq!(reg_occ_ev(fac, std::ptr::null_mut(), &mut c_occ), E_INVALIDARG);
            assert_eq!(reg_occ_ev(fac, h_occ, &mut c_occ), S_OK);
            assert_eq!(reg_stereo_ev(fac, h_st, &mut c_st), S_OK);
            assert_eq!(reg_adp_ev(fac, h_adp, &mut c_adp), S_OK);

            assert_eq!(crate::kernel32::threads::WaitForSingleObject(h_occ, 0), 0x102);
            notify_occlusion_status_changed();
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(h_occ, 0), 0);
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(h_st, 0), 0x102);
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(h_adp, 0), 0x102);

            notify_stereo_status_changed();
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(h_st, 0), 0);

            notify_adapters_changed();
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(h_adp, 0), 0);

            ev_occ.reset();
            ev_st.reset();
            ev_adp.reset();
            unreg_occ(fac, c_occ);
            unreg_stereo(fac, c_st);
            assert_eq!(unreg_adp(fac, c_adp), S_OK);
            assert_eq!(unreg_adp(fac, c_adp), E_INVALIDARG);

            notify_occlusion_status_changed();
            notify_stereo_status_changed();
            notify_adapters_changed();
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(h_occ, 0), 0x102);
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(h_st, 0), 0x102);
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(h_adp, 0), 0x102);

            crate::handles::close(h_occ);
            crate::handles::close(h_st);
            crate::handles::close(h_adp);
            com_release(fac);
        }
    }

    #[test]
    fn get_shared_resource_adapter_luid_resolves_d3d11_shared_handles() {
        if d3d12_metal::metal::device().is_none() {
            return;
        }
        unsafe {
            let mut fac: *mut c_void = std::ptr::null_mut();
            assert_eq!(CreateDXGIFactory2(0, &IID_IDXGIFactory7, &mut fac), S_OK);
            let get_luid: unsafe extern "win64" fn(*mut c_void, HANDLE, *mut u64) -> HRESULT =
                std::mem::transmute(vtbl_slot(fac, 17));

            // Null/invalid handles -> E_INVALIDARG and LUID zeroed.
            let mut luid = 0xDEADu64;
            assert_eq!(get_luid(fac, std::ptr::null_mut(), &mut luid), E_INVALIDARG);
            assert_eq!(luid, 0);
            luid = 0xDEADu64;
            assert_eq!(get_luid(fac, 0x1234usize as HANDLE, &mut luid), E_INVALIDARG);
            assert_eq!(luid, 0);
            assert_eq!(get_luid(fac, 0x1234usize as HANDLE, std::ptr::null_mut()), E_INVALIDARG);

            // Create a real D3D11 shared texture with SHARED_NTHANDLE and query its adapter LUID.
            let mut dev: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crate::d3d11::D3D11CreateDevice(
                    std::ptr::null_mut(),
                    crate::d3d11::types::driver_type::HARDWARE,
                    std::ptr::null_mut(),
                    0,
                    std::ptr::null(),
                    0,
                    crate::d3d11::types::D3D11_SDK_VERSION,
                    &mut dev,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ),
                S_OK
            );
            let create_tex2d: unsafe extern "win64" fn(
                *mut c_void,
                *const crate::d3d11::types::D3D11_TEXTURE2D_DESC,
                *const crate::d3d11::types::D3D11_SUBRESOURCE_DATA,
                *mut *mut c_void,
            ) -> HRESULT = std::mem::transmute(vtbl_slot(dev, 5));
            let td = crate::d3d11::types::D3D11_TEXTURE2D_DESC {
                Width: 4,
                Height: 4,
                MipLevels: 1,
                ArraySize: 1,
                Format: 28,
                SampleDesc: crate::d3d11::types::DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
                Usage: crate::d3d11::types::usage::DEFAULT,
                BindFlags: crate::d3d11::types::bind_flag::SHADER_RESOURCE,
                CPUAccessFlags: 0,
                MiscFlags: crate::d3d11::types::misc_flag::SHARED
                    | crate::d3d11::types::misc_flag::SHARED_NTHANDLE,
            };
            let mut tex: *mut c_void = std::ptr::null_mut();
            assert_eq!(create_tex2d(dev, &td, std::ptr::null(), &mut tex), S_OK);
            let mut res1: *mut c_void = std::ptr::null_mut();
            assert_eq!(com_query(tex, &crate::d3d11::types::IID_IDXGIResource1, &mut res1), S_OK);
            let create_sh: unsafe extern "win64" fn(
                *mut c_void, *const c_void, u32, LPCWSTR, *mut HANDLE,
            ) -> HRESULT = std::mem::transmute(vtbl_slot(res1, 13));
            let mut nt_handle: HANDLE = std::ptr::null_mut();
            assert_eq!(
                create_sh(
                    res1,
                    std::ptr::null(),
                    crate::d3d11::types::dxgi_shared_resource::READ,
                    std::ptr::null(),
                    &mut nt_handle,
                ),
                S_OK
            );
            assert!(!nt_handle.is_null());

            assert_eq!(get_luid(fac, nt_handle, &mut luid), S_OK);
            assert_eq!(luid, adapter::adapter_luid());
            assert_ne!(luid, 0);

            crate::handles::close(nt_handle);
            com_release(res1);
            com_release(tex);
            com_release(dev);
            com_release(fac);
        }
    }
}
