//! Metal 4 backend.

pub mod binding;
pub mod capture;
pub mod clear;
pub mod cmdlist;
pub mod descriptor;
pub mod device;
pub mod dml_gpu;
pub mod encstats;
pub mod fence;
pub mod gpucapture;
pub mod gpunan;
pub mod heap;
pub mod indirect;
pub mod indirect_icb;
pub mod m41;
pub mod metalfx;
pub mod passtiming;
pub mod pipeline;
pub mod present_thread;
pub mod pso_manifest;
pub mod query;
pub mod queue;
pub mod resource;
pub mod swapchain;
pub mod views;

pub use device::{device, Device};

/// True when running under Rosetta translation (`sysctl.proc_translated`).
pub fn is_translated() -> bool {
    let mut v: i32 = 0;
    let mut sz = std::mem::size_of::<i32>();
    unsafe extern "C" {
        fn sysctlbyname(name: *const u8, oldp: *mut std::ffi::c_void, oldlenp: *mut usize, newp: *const std::ffi::c_void, newlen: usize) -> i32;
    }
    let r = unsafe { sysctlbyname(b"sysctl.proc_translated\0".as_ptr(), &mut v as *mut i32 as *mut _, &mut sz, std::ptr::null(), 0) };
    r == 0 && v == 1
}

/// Read an environment knob preferring the canonical `PORT_*` name over a legacy
/// `AW2_*` alias, treating empty values as unset (matching `port-manifest::knobs`).
pub(crate) fn env_canonical_or_alias(canonical: &str, alias: &str) -> Option<String> {
    std::env::var(canonical)
        .ok()
        .filter(|v| !v.is_empty())
        .or_else(|| std::env::var(alias).ok().filter(|v| !v.is_empty()))
}

/// Parse a boolean knob value (`"0"`, `"false"`, `"no"`, `"off"` => `false`; any
/// other non-empty value => `true`), matching `port-manifest::knobs::get_bool`.
pub(crate) fn parse_bool_knob(v: &str) -> bool {
    let t = v.trim();
    !t.is_empty() && !matches!(t.to_ascii_lowercase().as_str(), "0" | "false" | "no" | "off")
}

#[cfg(test)]
pub(crate) fn env_test_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}
