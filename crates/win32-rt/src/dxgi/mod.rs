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

/// No DXGI debug layer: E_NOINTERFACE, as on a system without the Graphics Tools.
pub unsafe extern "win64" fn DXGIGetDebugInterface1(_flags: u32, riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    stub!("DXGIGetDebugInterface1({:?}) -> E_NOINTERFACE (no debug layer)", Guid::read(riid).unwrap_or_default());
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
                slot!("CreateSwapChainForCoreWindow" => create_swap_chain_unsupported),
                slot!("GetSharedResourceAdapterLuid" => get_shared_resource_adapter_luid),
                slot!("RegisterStereoStatusWindow" => register_status_window),
                slot!("RegisterStereoStatusEvent" => register_status_event),
                slot!("UnregisterStereoStatus" => unregister_status),
                slot!("RegisterOcclusionStatusWindow" => register_status_window),
                slot!("RegisterOcclusionStatusEvent" => register_status_event),
                slot!("UnregisterOcclusionStatus" => unregister_status),
                slot!("CreateSwapChainForComposition" => create_swap_chain_unsupported),
                slot!("GetCreationFlags" => get_creation_flags),
                slot!("EnumAdapterByLuid" => enum_adapter_by_luid),
                slot!("EnumWarpAdapter" => enum_warp_adapter),
                slot!("CheckFeatureSupport" => check_feature_support),
                slot!("EnumAdapterByGpuPreference" => enum_adapter_by_gpu_preference),
                slot!("RegisterAdaptersChangedEvent" => register_status_event),
                slot!("UnregisterAdaptersChangedEvent" => unregister_status),
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

unsafe extern "win64" fn enum_warp_adapter(_this: PF, _riid: *const Guid, out: *mut *mut c_void) -> HRESULT {
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    stub!("EnumWarpAdapter: no WARP software rasterizer -> DXGI_ERROR_NOT_FOUND");
    DXGI_ERROR_NOT_FOUND
}

unsafe extern "win64" fn create_software_adapter(_this: PF, _module: HMODULE, out: *mut *mut c_void) -> HRESULT {
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    stub!("CreateSoftwareAdapter -> DXGI_ERROR_UNSUPPORTED");
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

unsafe extern "win64" fn create_swap_chain_unsupported(_this: PF) -> HRESULT {
    stub!("CreateSwapChainForCoreWindow/Composition -> DXGI_ERROR_UNSUPPORTED");
    DXGI_ERROR_UNSUPPORTED
}

unsafe extern "win64" fn get_shared_resource_adapter_luid(_this: PF, _h: HANDLE, out: *mut u64) -> HRESULT {
    if !out.is_null() {
        *out = 0;
    }
    stub!("GetSharedResourceAdapterLuid: no shared handles -> E_INVALIDARG");
    E_INVALIDARG
}

static COOKIE: AtomicUsize = AtomicUsize::new(1);

unsafe extern "win64" fn register_status_window(_this: PF, _hwnd: HWND, _msg: u32, cookie: *mut u32) -> HRESULT {
    stub!("Register{{Stereo,Occlusion}}StatusWindow: status changes are never posted");
    if !cookie.is_null() {
        *cookie = COOKIE.fetch_add(1, Ordering::Relaxed) as u32;
    }
    S_OK
}

unsafe extern "win64" fn register_status_event(_this: PF, _ev: HANDLE, cookie: *mut u32) -> HRESULT {
    stub!("Register*Event (stereo/occlusion/adapters-changed): the event is never signaled");
    if !cookie.is_null() {
        *cookie = COOKIE.fetch_add(1, Ordering::Relaxed) as u32;
    }
    S_OK
}

unsafe extern "win64" fn unregister_status(_this: PF, _cookie: u32) {}

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
