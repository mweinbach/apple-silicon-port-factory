//! `physx3cooking_x64.dll` — native Rust replacement for NVIDIA PhysX 3.4.2 Cooking (`PxCreateCooking`).
//!
//! `AlanWake2.exe` imports a single symbol (`PxCreateCooking`) from `PhysX3Cooking_x64.dll`
//! at `0x1421466ac` and stores the returned `PxCooking*` at `[physics_ctx + 0x30]`.

use crate::{Export, Ptr};
use std::ffi::c_void;
use std::sync::Once;

pub static EXTRA_EXPORTS: &[Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

static REGISTER_VTABLES: Once = Once::new();

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PxCookingParams {
    pub area_test_epsilon: f32,                 // +0x00
    pub plane_tolerance: f32,                   // +0x04
    pub convex_mesh_cooking_type: u32,          // +0x08
    pub suppress_triangle_mesh_remap_table: u8, // +0x0c
    pub build_triangle_adjacencies: u8,         // +0x0d
    pub build_gpu_data: u8,                     // +0x0e
    pub _pad0: u8,                              // +0x0f
    pub scale_length: f32,                      // +0x10
    pub scale_speed: f32,                       // +0x14
    pub mesh_preprocess_params: u32,            // +0x18
    pub mesh_weld_tolerance: f32,               // +0x1c
    pub midphase_desc_type: u32,                // +0x20
    pub midphase_desc_union: [u32; 2],          // +0x24..+0x2c
    pub gauss_map_limit: u32,                   // +0x2c
    pub skin_width: f32,                        // +0x30
    pub mesh_cooking_hint: u32,                 // +0x34
    pub mesh_size_performance_trade_off: f32,   // +0x38
    pub target_platform: u32,                   // +0x3c
}
const _: () = assert!(std::mem::size_of::<PxCookingParams>() == 0x40);

#[repr(C)]
pub struct PxCookingInstance {
    pub vtbl: *const [Ptr; 14], // +0x00
    pub params: [u8; 0x44],     // +0x08..+0x4c (`PhysX3Cooking_x64.dll` copies 0x44 bytes here)
    pub foundation: *mut c_void,
}

unsafe extern "win64" fn cooking_release(this: *mut PxCookingInstance) {
    if !this.is_null() {
        let _ = Box::from_raw(this);
    }
}

unsafe extern "win64" fn cooking_set_params(this: *mut PxCookingInstance, params: *const u8) {
    if !this.is_null() && !params.is_null() {
        std::ptr::copy_nonoverlapping(params, (*this).params.as_mut_ptr(), 0x44);
    }
}

unsafe extern "win64" fn cooking_get_params(this: *const PxCookingInstance) -> *const u8 {
    if this.is_null() {
        std::ptr::null()
    } else {
        (*this).params.as_ptr()
    }
}

unsafe extern "win64" fn cooking_platform_mismatch(_this: *const PxCookingInstance) -> u8 {
    0
}

unsafe extern "win64" fn cooking_cook_triangle_mesh(
    _this: *const PxCookingInstance,
    _desc: *const c_void,
    _stream: *mut c_void,
    condition: *mut u32,
) -> u8 {
    if !condition.is_null() {
        *condition = 0; // PxTriangleMeshCookingResult::eSUCCESS
    }
    1
}

unsafe extern "win64" fn cooking_create_triangle_mesh(
    _this: *const PxCookingInstance,
    _desc: *const c_void,
    _insertion_cb: *mut c_void,
    condition: *mut u32,
) -> *mut c_void {
    if !condition.is_null() {
        *condition = 0;
    }
    std::ptr::null_mut()
}

unsafe extern "win64" fn cooking_validate_triangle_mesh(
    _this: *const PxCookingInstance,
    _desc: *const c_void,
) -> u8 {
    1
}

unsafe extern "win64" fn cooking_cook_convex_mesh(
    _this: *const PxCookingInstance,
    _desc: *const c_void,
    stream: *mut *const usize,
    condition: *mut u32,
) -> u8 {
    if !condition.is_null() {
        *condition = 0; // PxConvexMeshCookingResult::eSUCCESS
    }
    // If a valid PxOutputStream is provided (vtable slot 0 = `write(this, src, count) -> u32`),
    // write a minimal 32-byte header so `PxDefaultMemoryInputData` has non-empty data.
    if !stream.is_null() {
        let vtbl = *stream;
        if !vtbl.is_null() {
            let write_fn = *vtbl;
            if write_fn != 0 {
                let dummy_header: [u8; 32] = *b"NXS\x01CVXM\x0d\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
                crate::call_guest8(
                    write_fn,
                    stream as u64,
                    dummy_header.as_ptr() as u64,
                    dummy_header.len() as u64,
                    0,
                    0,
                    0,
                    0,
                    0,
                );
            }
        }
    }
    1
}

unsafe extern "win64" fn cooking_create_convex_mesh(
    _this: *const PxCookingInstance,
    _desc: *const c_void,
    _insertion_cb: *mut c_void,
    condition: *mut u32,
) -> *mut c_void {
    if !condition.is_null() {
        *condition = 0; // PxConvexMeshCookingResult::eSUCCESS
    }
    std::ptr::null_mut()
}

unsafe extern "win64" fn cooking_validate_convex_mesh(
    _this: *const PxCookingInstance,
    _desc: *const c_void,
) -> u8 {
    1
}

unsafe extern "win64" fn cooking_compute_hull_polygons(
    _this: *const PxCookingInstance,
    _mesh: *const c_void,
    _in_cb: *mut c_void,
    nb_verts: *mut u32,
    verts: *mut *mut c_void,
    nb_indices: *mut u32,
    indices: *mut *mut u32,
    nb_polygons: *mut u32,
    hull_polygons: *mut *mut c_void,
) -> u8 {
    if !nb_verts.is_null() {
        *nb_verts = 0;
    }
    if !verts.is_null() {
        *verts = std::ptr::null_mut();
    }
    if !nb_indices.is_null() {
        *nb_indices = 0;
    }
    if !indices.is_null() {
        *indices = std::ptr::null_mut();
    }
    if !nb_polygons.is_null() {
        *nb_polygons = 0;
    }
    if !hull_polygons.is_null() {
        *hull_polygons = std::ptr::null_mut();
    }
    0
}

unsafe extern "win64" fn cooking_cook_height_field(
    _this: *const PxCookingInstance,
    _desc: *const c_void,
    _stream: *mut c_void,
) -> u8 {
    1
}

unsafe extern "win64" fn cooking_create_height_field(
    _this: *const PxCookingInstance,
    _desc: *const c_void,
    _insertion_cb: *mut c_void,
) -> *mut c_void {
    std::ptr::null_mut()
}

unsafe extern "win64" fn cooking_dtor(
    this: *mut PxCookingInstance,
    flags: u32,
) -> *mut PxCookingInstance {
    if (flags & 1) != 0 && !this.is_null() {
        let _ = Box::from_raw(this);
    }
    this
}

static COOKING_VTBL: [Ptr; 14] = [
    Ptr(cooking_release as *const ()),               // 0 (+0x00): release
    Ptr(cooking_set_params as *const ()),            // 1 (+0x08): setParams
    Ptr(cooking_get_params as *const ()),            // 2 (+0x10): getParams
    Ptr(cooking_platform_mismatch as *const ()),     // 3 (+0x18): platformMismatch
    Ptr(cooking_cook_triangle_mesh as *const ()),    // 4 (+0x20): cookTriangleMesh
    Ptr(cooking_create_triangle_mesh as *const ()),  // 5 (+0x28): createTriangleMesh
    Ptr(cooking_validate_triangle_mesh as *const ()),// 6 (+0x30): validateTriangleMesh
    Ptr(cooking_cook_convex_mesh as *const ()),      // 7 (+0x38): cookConvexMesh
    Ptr(cooking_create_convex_mesh as *const ()),    // 8 (+0x40): createConvexMesh
    Ptr(cooking_validate_convex_mesh as *const ()),  // 9 (+0x48): validateConvexMesh
    Ptr(cooking_compute_hull_polygons as *const ()), // 10 (+0x50): computeHullPolygons
    Ptr(cooking_cook_height_field as *const ()),     // 11 (+0x58): cookHeightField
    Ptr(cooking_create_height_field as *const ()),   // 12 (+0x60): createHeightField
    Ptr(cooking_dtor as *const ()),                  // 13 (+0x68): ~Cooking
];

fn ensure_vtables_registered() {
    REGISTER_VTABLES.call_once(|| {
        for p in COOKING_VTBL.iter() {
            let addr = p.0 as usize;
            if addr != 0 {
                crate::register_native_va(addr as u64, addr);
            }
        }
    });
}

pub unsafe extern "win64" fn PxCreateCooking(
    _version: u32,
    foundation: *mut c_void,
    params: *const u8,
) -> *mut PxCookingInstance {
    ensure_vtables_registered();
    let mut raw_params = [0u8; 0x44];
    if !params.is_null() {
        std::ptr::copy_nonoverlapping(params, raw_params.as_mut_ptr(), 0x44);
    }
    let inst = Box::new(PxCookingInstance {
        vtbl: &COOKING_VTBL,
        params: raw_params,
        foundation,
    });
    Box::into_raw(inst)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn px_create_cooking_lifecycle() {
        unsafe {
            let mut params = [0u8; 0x44];
            params[0] = 0x42;
            let cooking = PxCreateCooking(0x0304_0200, 1usize as *mut c_void, params.as_ptr());
            assert!(!cooking.is_null());
            let got = cooking_get_params(cooking);
            assert_eq!(*got, 0x42);
            assert_eq!(cooking_platform_mismatch(cooking), 0);
            let mut cond = 99u32;
            assert_eq!(cooking_cook_convex_mesh(cooking, std::ptr::null(), std::ptr::null_mut(), &mut cond), 1);
            assert_eq!(cond, 0);
            cooking_release(cooking);
        }
    }
}
