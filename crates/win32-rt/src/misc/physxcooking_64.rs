//! `physxcooking_64.dll` — honest version-aware stub for NVIDIA PhysX 4.x / 5.x
//! Cooking (`PhysXCooking_64.dll`).
//!
//! Does not fabricate an unverified PhysX 4.x/5.x `PxCooking` vtable; games
//! requiring PhysX 4.x/5.x cooking should run their shipping PE DLL via
//! `[middleware.physx] mode = "guest"`.

use crate::middleware::physx::{configured_version, PhysxVersion};
use crate::Export;
use std::ffi::c_void;

pub static EXTRA_EXPORTS: &[Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

pub unsafe extern "win64" fn PxCreateCooking(
    version: u32,
    _foundation: *mut c_void,
    _params: *const u8,
) -> *mut c_void {
    let formatted = PhysxVersion::format_version_u32(version);
    let detected = PhysxVersion::from_version_u32(version)
        .map(|v| v.as_str())
        .unwrap_or("unknown");
    crate::stub!(
        "PhysXCooking_64!PxCreateCooking(version={formatted}, detected={detected}, configured={:?}): PhysX 4.x/5.x cooking requires shipping PE ([middleware.physx] mode = \"guest\") -> NULL",
        configured_version().map(|v| v.as_str()),
    );
    std::ptr::null_mut()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::physx::encode_version;

    #[test]
    fn cooking_64_returns_honest_null() {
        assert!(crate::lookup("PhysXCooking_64.dll", "PxCreateCooking").is_some());

        unsafe {
            let ptr = PxCreateCooking(
                encode_version(4, 1, 2),
                std::ptr::null_mut(),
                std::ptr::null(),
            );
            assert!(ptr.is_null());
        }
    }
}
