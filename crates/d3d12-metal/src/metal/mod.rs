//! Metal 4 backend.

pub mod binding;
pub mod capture;
pub mod clear;
pub mod cmdlist;
pub mod descriptor;
pub mod device;
pub mod encstats;
pub mod fence;
pub mod gpucapture;
pub mod heap;
pub mod indirect;
pub mod indirect_icb;
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
