//! `physxfoundation_64.dll` — version-aware adapter for NVIDIA PhysX 4.x / 5.x
//! Foundation (`PhysXFoundation_64.dll`).
//!
//! Decodes the caller's `PX_PHYSICS_VERSION` and delegates foundation creation and
//! singleton lookup to `pxfoundation_x64` (whose `PxFoundation` allocator and
//! error-callback bridge is ABI-compatible for foundation initialization).

use crate::middleware::physx::{configured_version, PhysxVersion};
use crate::Export;
use std::ffi::c_void;

pub static EXTRA_EXPORTS: &[Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

pub unsafe extern "win64" fn PxCreateFoundation(
    version: u32,
    allocator: *mut c_void,
    error_callback: *mut c_void,
) -> *mut c_void {
    let formatted = PhysxVersion::format_version_u32(version);
    let detected = PhysxVersion::from_version_u32(version)
        .map(|v| v.as_str())
        .unwrap_or("unknown");
    log::info!(
        "PhysXFoundation_64!PxCreateFoundation: version={formatted} (detected={detected}, configured={:?})",
        configured_version().map(|v| v.as_str()),
    );
    crate::misc::pxfoundation_x64::PxCreateFoundation(version, allocator, error_callback)
        as *mut c_void
}

pub unsafe extern "win64" fn PxGetFoundation() -> *mut c_void {
    crate::misc::pxfoundation_x64::PxGetFoundation() as *mut c_void
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::physx::encode_version;

    #[test]
    fn foundation_64_exports_and_delegates() {
        assert!(crate::lookup("PhysXFoundation_64.dll", "PxCreateFoundation").is_some());
        assert!(crate::lookup("PhysXFoundation_64.dll", "PxGetFoundation").is_some());

        unsafe {
            let f = PxCreateFoundation(
                encode_version(5, 1, 0),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            assert!(!f.is_null());
            assert_eq!(PxGetFoundation(), f);
        }
    }
}
