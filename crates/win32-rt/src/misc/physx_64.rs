//! `physx_64.dll` — honest version-aware stub for NVIDIA PhysX 4.x / 5.x core
//! runtime (`PhysX_64.dll`).
//!
//! Does not fabricate unverified PhysX 4.x/5.x `PxPhysics` or `PxScene` vtables;
//! games requiring PhysX 4.x/5.x simulation should run their shipping PE DLL via
//! `[middleware.physx] version = "4.1" | "5.x", mode = "guest"`.

use crate::middleware::physx::{configured_version, PhysxVersion};
use crate::Export;
use std::ffi::c_void;

pub static EXTRA_EXPORTS: &[Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

pub unsafe extern "win64" fn PxCreateBasePhysics(
    version: u32,
    _foundation: *mut c_void,
    _scale: *const c_void,
    _track_outstanding: u8,
    _pvd: *mut c_void,
) -> *mut c_void {
    let formatted = PhysxVersion::format_version_u32(version);
    let detected = PhysxVersion::from_version_u32(version)
        .map(|v| v.as_str())
        .unwrap_or("unknown");
    crate::stub!(
        "PhysX_64!PxCreateBasePhysics(version={formatted}, detected={detected}, configured={:?}): PhysX 4.x/5.x core simulation requires shipping PE ([middleware.physx] version = \"4.1\" | \"5.x\", mode = \"guest\") -> NULL",
        configured_version().map(|v| v.as_str()),
    );
    std::ptr::null_mut()
}

pub unsafe extern "win64" fn PxCreatePhysics(
    version: u32,
    _foundation: *mut c_void,
    _scale: *const c_void,
    _track_outstanding: u8,
    _pvd: *mut c_void,
) -> *mut c_void {
    let formatted = PhysxVersion::format_version_u32(version);
    let detected = PhysxVersion::from_version_u32(version)
        .map(|v| v.as_str())
        .unwrap_or("unknown");
    crate::stub!(
        "PhysX_64!PxCreatePhysics(version={formatted}, detected={detected}, configured={:?}): PhysX 4.x/5.x core simulation requires shipping PE ([middleware.physx] version = \"4.1\" | \"5.x\", mode = \"guest\") -> NULL",
        configured_version().map(|v| v.as_str()),
    );
    std::ptr::null_mut()
}

pub unsafe extern "win64" fn PxGetPhysics() -> *mut c_void {
    std::ptr::null_mut()
}

pub unsafe extern "win64" fn PxRegisterPhysicsSerializers(_registry: *mut c_void) {
    crate::stub!(
        "PhysX_64!PxRegisterPhysicsSerializers: PhysX 4.x/5.x serializers require shipping PE ([middleware.physx] version = \"4.1\" | \"5.x\", mode = \"guest\")"
    );
}

pub unsafe extern "win64" fn PxUnregisterPhysicsSerializers(_registry: *mut c_void) {
    crate::stub!(
        "PhysX_64!PxUnregisterPhysicsSerializers: PhysX 4.x/5.x serializers require shipping PE ([middleware.physx] version = \"4.1\" | \"5.x\", mode = \"guest\")"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::middleware::physx::encode_version;

    #[test]
    fn physx_64_exports_and_honest_null_returns() {
        for sym in [
            "PxCreateBasePhysics",
            "PxCreatePhysics",
            "PxGetPhysics",
            "PxRegisterPhysicsSerializers",
            "PxUnregisterPhysicsSerializers",
        ] {
            assert!(
                crate::lookup("PhysX_64.dll", sym).is_some(),
                "missing export PhysX_64.dll!{sym}"
            );
        }

        unsafe {
            assert!(PxCreateBasePhysics(
                encode_version(4, 1, 2),
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                std::ptr::null_mut(),
            )
            .is_null());
            assert!(PxCreatePhysics(
                encode_version(5, 1, 0),
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                std::ptr::null_mut(),
            )
            .is_null());
            assert!(PxGetPhysics().is_null());
            PxRegisterPhysicsSerializers(std::ptr::null_mut());
            PxUnregisterPhysicsSerializers(std::ptr::null_mut());
        }
    }
}
