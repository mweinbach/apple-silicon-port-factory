//! `physxcommon_64.dll` — version-aware adapter for NVIDIA PhysX 4.x / 5.x
//! Common (`PhysXCommon_64.dll`).

use crate::Export;
use std::ffi::c_void;

pub static EXTRA_EXPORTS: &[Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

pub unsafe extern "win64" fn PxCreateCollection() -> *mut c_void {
    crate::misc::physx3common_x64::px_create_collection() as *mut c_void
}

pub unsafe extern "win64" fn PxGetFoundation() -> *mut c_void {
    crate::misc::pxfoundation_x64::PxGetFoundation() as *mut c_void
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_64_exports_and_delegates() {
        assert!(crate::lookup("PhysXCommon_64.dll", "PxCreateCollection").is_some());
        assert!(crate::lookup("PhysXCommon_64.dll", "PxGetFoundation").is_some());

        unsafe {
            let f = PxGetFoundation();
            assert!(!f.is_null());
            let col = PxCreateCollection();
            assert!(!col.is_null());
            // Release the CmCollection via slot 14 (+0x70) of its vtable.
            let vtbl = *(col as *const *const usize);
            let release: unsafe extern "win64" fn(*mut c_void) =
                std::mem::transmute(*vtbl.add(14));
            release(col);
        }
    }
}
