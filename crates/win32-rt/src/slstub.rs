//! Builtin `sl.interposer.dll` (NVIDIA Streamline), replacing the game-dir
//! DLL via the loader's builtin-override list.
//!
//! The real interposer wraps D3D12/DXGI objects and loads NVIDIA plugins
//! (DLSS, Reflex, frame generation) that need an NVIDIA GPU + NGX. Ours:
//! * forwards the graphics exports the game delay-imports from it
//!   (`CreateDXGIFactory1`, `D3D12CreateDevice`, ...) straight to our
//!   dxgi/d3d12 builtins — no proxies;
//! * initializes successfully (`slInit` → `eOk`) with **no features**: every
//!   feature query reports `eErrorFeatureNotSupported`, feature functions are
//!   null, so the game takes its non-Streamline paths;
//! * accepts tags/constants (nothing consumes them) and hands out real frame
//!   tokens.
//!
//! Result codes: `third_party/streamline/include/sl_result.h`.

use std::ffi::{c_char, c_void};
use std::sync::atomic::{AtomicU32, Ordering};

/// `sl::Result` values.
pub mod result {
    pub const OK: i32 = 0;
    pub const ERROR_INVALID_PARAMETER: i32 = 25;
    pub const ERROR_FEATURE_MISSING: i32 = 31;
    pub const ERROR_FEATURE_NOT_SUPPORTED: i32 = 32;
}
use result::*;

pub static EXPORTS: &[crate::Export] = exports![
    slInit, slShutdown, slIsFeatureSupported, slIsFeatureLoaded, slSetFeatureLoaded, slEvaluateFeature,
    slAllocateResources, slFreeResources, slSetTag, slGetFeatureRequirements, slGetFeatureVersion,
    slUpgradeInterface, slSetConstants, slGetNativeInterface, slGetFeatureFunction, slGetNewFrameToken,
    slSetD3DDevice;
    "CreateDXGIFactory" => crate::dxgi::CreateDXGIFactory,
    "CreateDXGIFactory1" => crate::dxgi::CreateDXGIFactory1,
    "CreateDXGIFactory2" => crate::dxgi::CreateDXGIFactory2,
    "DXGIGetDebugInterface1" => crate::dxgi::DXGIGetDebugInterface1,
    "D3D12CreateDevice" => crate::d3d12::D3D12CreateDevice,
    "D3D12GetDebugInterface" => crate::d3d12::D3D12GetDebugInterface,
    "D3D12SerializeRootSignature" => crate::d3d12::D3D12SerializeRootSignature,
    "D3D12SerializeVersionedRootSignature" => crate::d3d12::D3D12SerializeVersionedRootSignature,
    "D3D12CreateRootSignatureDeserializer" => crate::d3d12::D3D12CreateRootSignatureDeserializer,
    "D3D12CreateVersionedRootSignatureDeserializer" => crate::d3d12::D3D12CreateVersionedRootSignatureDeserializer,
    "D3D12EnableExperimentalFeatures" => crate::d3d12::D3D12EnableExperimentalFeatures,
];

fn feature_name(f: u32) -> String {
    // sl_consts.h kFeature*.
    let n = match f {
        0 => "DLSS",
        1 => "NRD",
        2 => "NIS",
        3 => "Reflex",
        4 => "PCL",
        5 => "DeepDVC",
        1000 => "DLSS_G",
        1001 => "DLSS_RR",
        1002 => "NvPerf",
        1003 => "DirectSR",
        6 => "Latewarp",
        9999 => "ImGUI",
        u32::MAX => "Common",
        _ => "",
    };
    if n.is_empty() {
        format!("feature {f}")
    } else {
        format!("{n} ({f})")
    }
}

pub unsafe extern "win64" fn slInit(_pref: *const c_void, sdk_version: u64) -> i32 {
    log::info!("sl.interposer (builtin): slInit(sdk {sdk_version:#x}) -> eOk; no Streamline features are available");
    OK
}

pub unsafe extern "win64" fn slShutdown() -> i32 {
    OK
}

pub unsafe extern "win64" fn slIsFeatureSupported(feature: u32, _adapter: *const c_void) -> i32 {
    log::info!("sl.interposer (builtin): slIsFeatureSupported({}) -> eErrorFeatureNotSupported", feature_name(feature));
    ERROR_FEATURE_NOT_SUPPORTED
}

pub unsafe extern "win64" fn slIsFeatureLoaded(feature: u32, loaded: *mut bool) -> i32 {
    if !loaded.is_null() {
        *loaded = false;
    }
    log::debug!("slIsFeatureLoaded({}) -> not loaded", feature_name(feature));
    ERROR_FEATURE_NOT_SUPPORTED
}

pub unsafe extern "win64" fn slSetFeatureLoaded(feature: u32, loaded: bool) -> i32 {
    if loaded {
        log::info!("sl.interposer (builtin): slSetFeatureLoaded({}, true) -> eErrorFeatureMissing", feature_name(feature));
        ERROR_FEATURE_MISSING
    } else {
        OK
    }
}

pub unsafe extern "win64" fn slEvaluateFeature(feature: u32, _frame: *const c_void, _inputs: *const c_void, _n: u32, _cmd: *mut c_void) -> i32 {
    log::warn!("sl.interposer (builtin): slEvaluateFeature({}) on an unsupported feature", feature_name(feature));
    ERROR_FEATURE_NOT_SUPPORTED
}

pub unsafe extern "win64" fn slAllocateResources(_cmd: *mut c_void, feature: u32, _vp: *const u32) -> i32 {
    log::warn!("sl.interposer (builtin): slAllocateResources({}) on an unsupported feature", feature_name(feature));
    ERROR_FEATURE_NOT_SUPPORTED
}

pub unsafe extern "win64" fn slFreeResources(feature: u32, _vp: *const u32) -> i32 {
    log::debug!("slFreeResources({}): nothing allocated", feature_name(feature));
    OK
}

/// Native-unsupported (NVIDIA Streamline SDK `source/core/sl.api/sl.cpp`:
/// `slSetTag` validates `kFeatureCommon` via `slValidateFeatureContext` and checks
/// `!tags || numTags == 0`). With no NVIDIA Streamline plugins loaded on Apple Silicon,
/// invalid parameters return `eErrorInvalidParameter` (25) and valid calls return
/// `eErrorFeatureNotSupported` (32).
pub unsafe extern "win64" fn slSetTag(
    _vp: *const u32,
    tags: *const c_void,
    n: u32,
    _cmd: *mut c_void,
) -> i32 {
    if tags.is_null() || n == 0 {
        return ERROR_INVALID_PARAMETER;
    }
    log::debug!("slSetTag({n} tags): no Streamline plugins supported -> eErrorFeatureNotSupported");
    ERROR_FEATURE_NOT_SUPPORTED
}

/// Native-unsupported (NVIDIA Streamline SDK `source/core/sl.api/sl.cpp`:
/// `slSetConstants` validates `kFeatureCommon` via `slValidateFeatureContext`).
/// Returns `eErrorInvalidParameter` (25) on null pointers and
/// `eErrorFeatureNotSupported` (32) when no Streamline plugins are supported.
pub unsafe extern "win64" fn slSetConstants(
    values: *const c_void,
    frame: *const c_void,
    _vp: *const u32,
) -> i32 {
    if values.is_null() || frame.is_null() {
        return ERROR_INVALID_PARAMETER;
    }
    log::debug!("slSetConstants: no Streamline plugins supported -> eErrorFeatureNotSupported");
    ERROR_FEATURE_NOT_SUPPORTED
}

pub unsafe extern "win64" fn slGetFeatureRequirements(feature: u32, _req: *mut c_void) -> i32 {
    log::info!("sl.interposer (builtin): slGetFeatureRequirements({}) -> eErrorFeatureNotSupported", feature_name(feature));
    ERROR_FEATURE_NOT_SUPPORTED
}

pub unsafe extern "win64" fn slGetFeatureVersion(feature: u32, _ver: *mut c_void) -> i32 {
    log::info!("sl.interposer (builtin): slGetFeatureVersion({}) -> eErrorFeatureNotSupported", feature_name(feature));
    ERROR_FEATURE_NOT_SUPPORTED
}

/// No proxies exist: the base interface is the interface.
pub unsafe extern "win64" fn slUpgradeInterface(base: *mut *mut c_void) -> i32 {
    if base.is_null() {
        return ERROR_INVALID_PARAMETER;
    }
    OK
}

pub unsafe extern "win64" fn slGetNativeInterface(proxy: *mut c_void, base: *mut *mut c_void) -> i32 {
    if base.is_null() || proxy.is_null() {
        return ERROR_INVALID_PARAMETER;
    }
    crate::d3d12::com::com_add_ref(proxy);
    *base = proxy;
    OK
}

pub unsafe extern "win64" fn slGetFeatureFunction(feature: u32, name: *const c_char, out: *mut *mut c_void) -> i32 {
    if !out.is_null() {
        *out = std::ptr::null_mut();
    }
    let n = if name.is_null() { String::new() } else { std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned() };
    log::info!("sl.interposer (builtin): slGetFeatureFunction({}, {n:?}) -> null, eErrorFeatureNotSupported", feature_name(feature));
    ERROR_FEATURE_NOT_SUPPORTED
}

pub unsafe extern "win64" fn slSetD3DDevice(dev: *mut c_void) -> i32 {
    log::debug!("slSetD3DDevice({dev:?})");
    OK
}

// ---------------------------------------------------------------------------
// Frame tokens: `struct FrameToken { virtual operator uint32_t() const = 0; }`
// ---------------------------------------------------------------------------

/// Layout: vptr, then `sl::BaseStructure` {next, structType, structVersion}.
#[repr(C)]
struct FrameToken {
    vtbl: *const usize,
    next: usize,
    struct_type: crate::d3d12::com::Guid,
    struct_version: usize,
    index: AtomicU32,
}

const FRAME_TOKEN_TYPE: crate::d3d12::com::Guid = crate::d3d12::com::guid("830a0f35-db84-4171-a804-59b206499b18");

unsafe extern "win64" fn frame_token_value(this: *const FrameToken) -> u32 {
    (*this).index.load(Ordering::Acquire)
}

/// Streamline keeps a small ring of tokens; the game holds the pointer for
/// the frame. A ring of 64 static tokens is plenty.
const RING: usize = 64;

fn tokens() -> &'static [FrameToken] {
    static T: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
    let p = *T.get_or_init(|| {
        let vt = crate::d3d12::com::build_vtable("sl::FrameToken", &[("operator uint32_t", Some(frame_token_value as *const ()))]);
        let v: Vec<FrameToken> = (0..RING).map(|_| FrameToken { vtbl: vt, next: 0, struct_type: FRAME_TOKEN_TYPE, struct_version: 1, index: AtomicU32::new(0) }).collect();
        Box::leak(v.into_boxed_slice()).as_ptr() as usize
    });
    unsafe { std::slice::from_raw_parts(p as *const FrameToken, RING) }
}

static FRAME: AtomicU32 = AtomicU32::new(0);

pub unsafe extern "win64" fn slGetNewFrameToken(token: *mut *const c_void, frame_index: *const u32) -> i32 {
    if token.is_null() {
        return ERROR_INVALID_PARAMETER;
    }
    let idx = if frame_index.is_null() { FRAME.fetch_add(1, Ordering::AcqRel) } else { *frame_index };
    let t = &tokens()[idx as usize % RING];
    t.index.store(idx, Ordering::Release);
    *token = t as *const FrameToken as *const c_void;
    OK
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_tokens_report_their_index() {
        unsafe {
            let mut t: *const c_void = std::ptr::null();
            assert_eq!(slGetNewFrameToken(&mut t, &41), OK);
            let vt = *(t as *const *const usize);
            let f: unsafe extern "win64" fn(*const c_void) -> u32 = std::mem::transmute(*vt);
            assert_eq!(f(t), 41);
            let mut p: *mut c_void = 1 as *mut c_void;
            assert_eq!(slGetFeatureFunction(0, c"slDLSSGetOptimalSettings".as_ptr(), &mut p), ERROR_FEATURE_NOT_SUPPORTED);
            assert!(p.is_null());

            // slSetTag / slSetConstants validate parameters and report eErrorFeatureNotSupported
            // when no NVIDIA Streamline plugins are loaded (matches sl.cpp).
            let dummy = 0u64;
            let dummy_ptr = &dummy as *const u64 as *const c_void;
            assert_eq!(slSetTag(std::ptr::null(), std::ptr::null(), 1, std::ptr::null_mut()), ERROR_INVALID_PARAMETER);
            assert_eq!(slSetTag(std::ptr::null(), dummy_ptr, 0, std::ptr::null_mut()), ERROR_INVALID_PARAMETER);
            assert_eq!(slSetTag(std::ptr::null(), dummy_ptr, 1, std::ptr::null_mut()), ERROR_FEATURE_NOT_SUPPORTED);
            assert_eq!(slSetConstants(std::ptr::null(), t, std::ptr::null()), ERROR_INVALID_PARAMETER);
            assert_eq!(slSetConstants(dummy_ptr, std::ptr::null(), std::ptr::null()), ERROR_INVALID_PARAMETER);
            assert_eq!(slSetConstants(dummy_ptr, t, std::ptr::null()), ERROR_FEATURE_NOT_SUPPORTED);
        }
    }
}
