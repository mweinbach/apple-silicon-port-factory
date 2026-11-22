//! `physx3_x64.dll` — native Rust replacement for NVIDIA PhysX 3.4.2 Core (`PhysX3_x64.dll`).
//!
//! Implements all 41 exports imported by `AlanWake2.exe`, including `PxCreateBasePhysics`,
//! `PxGetPhysics`, `PxRegisterPhysicsSerializers` (with all 18 `PxSerializer` adapters and
//! their exact `sizeof(T)` binary deserialization sizes), `PxPhysics`, `PxScene`,
//! `PxMaterial`, `PxShape`, `PxRigidStatic`, `PxRigidDynamic`, and `PxAggregate`.

use crate::misc::physx3common_x64::{PxBounds3, PxTransform};
use crate::misc::pxfoundation_x64::{
    allocator_allocate, allocator_deallocate, PxGetFoundation,
};
use crate::{Export, Ptr};
use std::ffi::{c_void, CStr};
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Once;

pub static ORDINALS: &[(u16, &str)] = &[];

static REGISTER_VTABLES: Once = Once::new();
static PHYSICS_PTR: AtomicPtr<PxPhysicsInstance> = AtomicPtr::new(std::ptr::null_mut());

#[repr(C)]
pub struct PxInsertionCallbackBridge {
    pub vtbl: *const [Ptr; 2],
}

#[repr(C)]
pub struct PxPvdClientBridge {
    pub vtbl: *const [Ptr; 16],
}

#[repr(C)]
pub struct PxPhysicsInstance {
    pub vtbl: *const [Ptr; 41],                     // +0x00
    pub foundation: *mut c_void,                    // +0x08
    pub pvd: *mut c_void,                           // +0x10
    pub _pad0: [u64; 2],                            // +0x18..+0x20
    pub tolerances_scale: [f32; 2],                 // +0x20..+0x28 (`getTolerancesScale` = `lea rax, [rcx + 0x20]`)
    pub _pad1: [u8; 0x38],                          // +0x28..+0x60
    pub insertion_cb: PxInsertionCallbackBridge,    // +0x60 (`getPhysicsInsertionCallback` = `lea rax, [rcx + 0x60]`)
    pub _pad2: [u8; 0xa0],
}

#[repr(C)]
pub struct PxMaterialInstance {
    pub vtbl: *const [Ptr; 20], // +0x00
    pub concrete_type: u16,     // +0x08 (7 = eMATERIAL)
    pub base_flags: u16,        // +0x0a (3 = eOWNS_MEMORY | eIS_RELEASABLE)
    pub _pad0: u32,             // +0x0c
    pub user_data: *mut c_void, // +0x10
    pub _pad1: u64,             // +0x18
    pub ref_count: u32,         // +0x20
    pub _pad2: [u32; 3],        // +0x24..+0x30
    pub dynamic_friction: f32,  // +0x30
    pub static_friction: f32,   // +0x34
    pub restitution: f32,       // +0x38
    pub flags: u16,             // +0x3c
    pub combine_modes: u8,      // +0x3e
    pub _pad3: [u8; 0x11],      // pad to 0x50
}
const _: () = assert!(std::mem::size_of::<PxMaterialInstance>() == 0x50);

#[repr(C)]
pub struct PxShapeInstance {
    pub vtbl: *const [Ptr; 42],         // +0x00
    pub concrete_type: u16,             // +0x08 (8 = eSHAPE)
    pub base_flags: u16,                // +0x0a
    pub _pad0: u32,                     // +0x0c
    pub user_data: *mut c_void,         // +0x10
    pub refcountable_vtbl: usize,       // +0x18
    pub ref_count: u32,                 // +0x20
    pub _pad1: u32,                     // +0x24
    pub actor: *mut c_void,             // +0x28
    pub scb_scene: *mut c_void,         // +0x30
    pub scb_flags: u32,                 // +0x38
    pub _pad2: u32,                     // +0x3c
    pub scb_stream_ptr: *mut c_void,    // +0x40
    pub _pad3: u64,                     // +0x48
    pub query_filter_data: [u32; 4],    // +0x50..+0x60
    pub sim_filter_data: [u32; 4],      // +0x60..+0x70
    pub local_pose: PxTransform,        // +0x70..+0x8c (`0x180064db0`)
    pub contact_offset: f32,            // +0x8c (`0x180065480`)
    pub shape_flags: u8,                // +0x90 (`0x1800656b4`)
    pub owndata: u8,                    // +0x91
    pub material_index: u16,            // +0x92
    pub _pad4: u32,                     // +0x94
    pub geom_type: u32,                 // +0x98 (`0x1800646f0`)
    pub geom_bytes: [u8; 0x54],         // +0x9c..+0xf0 (`0x1800648e0`)
    pub rest_offset: f32,               // +0xf0 (`0x1800655a0`)
    pub _pad5: [u32; 3],                // +0xf4..+0x100
    pub name: *const u8,                // +0x100 (`0x180065700`)
    pub exclusive_flag: u32,            // +0x108 (`0x1800656d0`)
    pub _pad6: u32,                     // +0x10c
}
const _: () = assert!(std::mem::size_of::<PxShapeInstance>() == 0x110);

#[repr(C)]
pub struct PxRigidActorInstance {
    pub vtbl: *const [Ptr; 82], // +0x00
    pub concrete_type: u16,     // +0x08 (6 = eRIGID_DYNAMIC, 7 = eRIGID_STATIC)
    pub base_flags: u16,        // +0x0a
    pub _pad0: u32,             // +0x0c
    pub user_data: *mut c_void, // +0x10
    pub name: *const u8,        // +0x18
    pub connectors: *mut c_void,// +0x20
    pub shapes: *mut c_void,    // +0x28 (`PtrTable` mShapes: inline when nb_shapes == 1, array ptr when > 1)
    pub nb_shapes: u16,         // +0x30
    pub shapes_own_mem: u8,     // +0x32
    pub _pad_sm0: [u8; 5],      // +0x33..+0x38
    pub sq_data: *mut c_void,   // +0x38 (`PtrTable` mSceneQueryData)
    pub nb_sq: u16,             // +0x40
    pub sq_own_mem: u8,         // +0x42
    pub _pad_sm1: [u8; 5],      // +0x43..+0x48
    pub _pad1: [u8; 0x148],     // +0x48..+0x190 (`NpRigidStatic` pose at +0x90; `NpRigidDynamic` pose at +0xb0)
}
const _: () = assert!(std::mem::size_of::<PxRigidActorInstance>() == 0x190);

#[repr(C)]
pub struct PxSceneInstance {
    pub vtbl: *const [Ptr; 128],     // +0x00
    pub user_data: *mut c_void,      // +0x08
    pub gravity: [f32; 3],           // +0x10..+0x1c
    pub limits: [u32; 9],            // +0x1c..+0x40
    pub pvd_client: PxPvdClientBridge,
    pub cpu_dispatcher: usize,
    pub _pad: [u8; 0x28f8],
}

unsafe extern "win64" fn ret0() -> usize {
    0
}

unsafe extern "win64" fn ret1() -> u8 {
    1
}

unsafe extern "win64" fn ret_f32_one() -> f32 {
    1.0
}

unsafe extern "win64" fn sret_zero_u32(_this: *const c_void, sret: *mut u32) -> *mut u32 {
    if !sret.is_null() {
        *sret = 0;
    }
    sret
}

unsafe extern "win64" fn sret_zero_u16(_this: *const c_void, sret: *mut u16) -> *mut u16 {
    if !sret.is_null() {
        *sret = 0;
    }
    sret
}

unsafe extern "win64" fn sret_zero_u8(_this: *const c_void, sret: *mut u8) -> *mut u8 {
    if !sret.is_null() {
        *sret = 0;
    }
    sret
}

unsafe extern "win64" fn sret_zero_vec3(_this: *const c_void, sret: *mut [f32; 3]) -> *mut [f32; 3] {
    if !sret.is_null() {
        std::ptr::write_unaligned(sret, [0.0, 0.0, 0.0]);
    }
    sret
}

unsafe extern "win64" fn sret_one_vec3(_this: *const c_void, sret: *mut [f32; 3]) -> *mut [f32; 3] {
    if !sret.is_null() {
        std::ptr::write_unaligned(sret, [1.0, 1.0, 1.0]);
    }
    sret
}

unsafe extern "win64" fn sret_identity_transform(
    _this: *const c_void,
    sret: *mut PxTransform,
) -> *mut PxTransform {
    if !sret.is_null() {
        std::ptr::write_unaligned(
            sret,
            PxTransform {
                q: [0.0, 0.0, 0.0, 1.0],
                p: [0.0, 0.0, 0.0],
            },
        );
    }
    sret
}

unsafe extern "win64" fn sret_default_bounds(
    _this: *const c_void,
    sret: *mut PxBounds3,
    _inflation_slot: u64,
) -> *mut PxBounds3 {
    if !sret.is_null() {
        std::ptr::write_unaligned(
            sret,
            PxBounds3 {
                minimum: [-1.0, -1.0, -1.0],
                maximum: [1.0, 1.0, 1.0],
            },
        );
    }
    sret
}

static PVD_CLIENT_VTBL: [Ptr; 16] = [
    Ptr(ret0 as *const ()),
    Ptr(ret0 as *const ()),
    Ptr(ret0 as *const ()),
    Ptr(ret0 as *const ()),
    Ptr(ret0 as *const ()),
    Ptr(ret0 as *const ()),
    Ptr(ret0 as *const ()),
    Ptr(ret0 as *const ()),
    Ptr(ret0 as *const ()),
    Ptr(ret0 as *const ()),
    Ptr(ret0 as *const ()),
    Ptr(ret0 as *const ()),
    Ptr(ret0 as *const ()),
    Ptr(ret0 as *const ()),
    Ptr(ret0 as *const ()),
    Ptr(ret0 as *const ()),
];

// ---------------------------------------------------------------------------
// Generic PxBase object (for meshes, heightfields, aggregates, articulations, constraints)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn base_obj_cleanup_subobjects(this: *mut u8) {
    if this.is_null() {
        return;
    }
    let ctype = std::ptr::read_unaligned(this.add(0x08) as *const u16);
    if matches!(ctype, 6 | 7 | 16) {
        let nb_shapes = std::ptr::read_unaligned(this.add(0x30) as *const u16);
        let own_mem = *this.add(0x32);
        if nb_shapes > 1 && own_mem != 0 {
            let arr = std::ptr::read_unaligned(this.add(0x28) as *const *mut c_void);
            if !arr.is_null() {
                std::ptr::write_unaligned(this.add(0x28) as *mut *mut c_void, std::ptr::null_mut());
                std::ptr::write_unaligned(this.add(0x30) as *mut u16, 0);
                *this.add(0x32) = 1;
                allocator_deallocate(std::ptr::null_mut(), arr);
            }
        }
    } else if ctype == 14 {
        let base_flags = std::ptr::read_unaligned(this.add(0x0a) as *const u16);
        if (base_flags & 1) != 0 {
            let arr = std::ptr::read_unaligned(this.add(0x48) as *const *mut c_void);
            if !arr.is_null() {
                std::ptr::write_unaligned(this.add(0x48) as *mut *mut c_void, std::ptr::null_mut());
                allocator_deallocate(std::ptr::null_mut(), arr);
            }
        }
    }
}

unsafe extern "win64" fn base_obj_release(this: *mut u8) {
    if !this.is_null() {
        let ctype = std::ptr::read_unaligned(this.add(0x08) as *const u16);
        if ctype == 8 {
            let rc_ptr = this.add(0x20) as *const std::sync::atomic::AtomicU32;
            let prev = (*rc_ptr).fetch_update(Ordering::AcqRel, Ordering::Acquire, |v| {
                Some(v.saturating_sub(1))
            }).unwrap_or(1);
            if prev > 1 {
                return;
            }
        } else if matches!(ctype, 1 | 2 | 3 | 4 | 5 | 9) {
            let rc_ptr = this.add(0x18) as *const std::sync::atomic::AtomicU32;
            let prev = (*rc_ptr).fetch_update(Ordering::AcqRel, Ordering::Acquire, |v| {
                Some(v.saturating_sub(1))
            }).unwrap_or(1);
            if prev > 1 {
                return;
            }
        }
        base_obj_cleanup_subobjects(this);
        let base_flags = std::ptr::read_unaligned(this.add(0x0a) as *const u16);
        if (base_flags & 1) != 0 {
            allocator_deallocate(std::ptr::null_mut(), this as *mut c_void);
        }
    }
}

unsafe extern "win64" fn base_obj_get_type_name(this: *const u8) -> *const u8 {
    if this.is_null() {
        return b"PxBase\0".as_ptr();
    }
    match std::ptr::read_unaligned(this.add(0x08) as *const u16) {
        1 => b"PxHeightField\0".as_ptr(),
        2 => b"Gu::ConvexMesh\0".as_ptr(),
        3 => b"Gu::RTreeTriangleMesh\0".as_ptr(),
        4 => b"Gu::BV4TriangleMesh\0".as_ptr(),
        5 => b"PxClothFabric\0".as_ptr(),
        6 => b"PxRigidDynamic\0".as_ptr(),
        7 => b"PxRigidStatic\0".as_ptr(),
        8 => b"PxShape\0".as_ptr(),
        9 => b"PxMaterial\0".as_ptr(),
        10 => b"PxConstraint\0".as_ptr(),
        11 => b"PxCloth\0".as_ptr(),
        12 => b"PxParticleSystem\0".as_ptr(),
        13 => b"PxParticleFluid\0".as_ptr(),
        14 => b"PxAggregate\0".as_ptr(),
        15 => b"PxArticulation\0".as_ptr(),
        16 => b"PxArticulationLink\0".as_ptr(),
        17 => b"PxArticulationJoint\0".as_ptr(),
        18 => b"PxPruningStructure\0".as_ptr(),
        _ => b"PxBase\0".as_ptr(),
    }
}

unsafe extern "win64" fn base_obj_is_kind_of(this: *const u8, name: *const u8) -> u8 {
    if this.is_null() || name.is_null() {
        return 0;
    }
    let Ok(s) = CStr::from_ptr(name as *const i8).to_str() else {
        return 0;
    };
    if s == "PxBase" {
        return 1;
    }
    let ctype = std::ptr::read_unaligned(this.add(0x08) as *const u16);
    let ok = match ctype {
        1 => s == "PxHeightField",
        2 => s == "PxConvexMesh",
        3 | 4 => s == "PxTriangleMesh",
        5 => s == "PxClothFabric",
        6 => matches!(
            s,
            "PxRigidDynamic" | "PxRigidBody" | "PxRigidActor" | "PxActor"
        ),
        7 => matches!(s, "PxRigidStatic" | "PxRigidActor" | "PxActor"),
        8 => s == "PxShape",
        9 => s == "PxMaterial",
        10 => s == "PxConstraint",
        11 => matches!(s, "PxCloth" | "PxActor"),
        12 => matches!(s, "PxParticleSystem" | "PxParticleBase" | "PxActor"),
        13 => matches!(s, "PxParticleFluid" | "PxParticleBase" | "PxActor"),
        14 => s == "PxAggregate",
        15 => s == "PxArticulation",
        16 => matches!(
            s,
            "PxArticulationLink" | "PxRigidBody" | "PxRigidActor" | "PxActor"
        ),
        17 => s == "PxArticulationJoint",
        18 => s == "PxPruningStructure",
        0x100..=0x105 => matches!(
            s,
            "PxJoint"
                | "PxSphericalJoint"
                | "PxRevoluteJoint"
                | "PxPrismaticJoint"
                | "PxFixedJoint"
                | "PxDistanceJoint"
                | "PxD6Joint"
        ),
        _ => false,
    };
    ok as u8
}

unsafe extern "win64" fn base_obj_is_releasable(this: *const u8) -> u8 {
    if this.is_null() {
        return 1;
    }
    let flags = std::ptr::read_unaligned(this.add(0x0a) as *const u16);
    ((flags & 2) != 0) as u8
}

unsafe extern "win64" fn base_obj_dtor(this: *mut u8, flags: u32) -> *mut u8 {
    if !this.is_null() {
        base_obj_cleanup_subobjects(this);
        if (flags & 1) != 0 {
            allocator_deallocate(std::ptr::null_mut(), this as *mut c_void);
        }
    }
    this
}

static GENERIC_BASE_VTBL: [Ptr; 32] = [
    Ptr(base_obj_release as *const ()),       // 0 (+0x00): release
    Ptr(base_obj_get_type_name as *const ()), // 1 (+0x08): getConcreteTypeName
    Ptr(base_obj_is_releasable as *const ()), // 2 (+0x10): isReleasable
    Ptr(base_obj_dtor as *const ()),          // 3 (+0x18): ~PxBase
    Ptr(base_obj_is_kind_of as *const ()),    // 4 (+0x20): isKindOf
    Ptr(ret1 as *const ()),                   // 5 (+0x28): getReferenceCount / addActor
    Ptr(ret0 as *const ()),                   // 6 (+0x30): acquireReference
    Ptr(ret0 as *const ()),                   // 7 (+0x38)
    Ptr(ret0 as *const ()),                   // 8 (+0x40)
    Ptr(ret0 as *const ()),                   // 9 (+0x48)
    Ptr(ret0 as *const ()),                   // 10 (+0x50)
    Ptr(sret_zero_u8 as *const ()),           // 11 (+0x58): getTriangleMeshFlags (sret)
    Ptr(sret_default_bounds as *const ()),    // 12 (+0x60): getLocalBounds (sret)
    Ptr(sret_default_bounds as *const ()),    // 13 (+0x68): PxConvexMesh::getLocalBounds (sret)
    Ptr(sret_default_bounds as *const ()),    // 14 (+0x70): PxTriangleMesh::getLocalBounds (sret)
    Ptr(ret0 as *const ()),                   // 15 (+0x78)
    Ptr(ret0 as *const ()),                   // 16 (+0x80)
    Ptr(ret0 as *const ()),                   // 17 (+0x88)
    Ptr(sret_zero_u32 as *const ()),          // 18 (+0x90): PxHeightField::getSample (sret)
    Ptr(ret0 as *const ()),                   // 19 (+0x98)
    Ptr(ret0 as *const ()),                   // 20 (+0xa0)
    Ptr(ret0 as *const ()),                   // 21 (+0xa8)
    Ptr(ret0 as *const ()),                   // 22 (+0xb0)
    Ptr(ret0 as *const ()),                   // 23 (+0xb8)
    Ptr(ret0 as *const ()),                   // 24 (+0xc0)
    Ptr(ret0 as *const ()),                   // 25 (+0xc8)
    Ptr(ret0 as *const ()),                   // 26 (+0xd0)
    Ptr(ret0 as *const ()),                   // 27 (+0xd8)
    Ptr(ret0 as *const ()),                   // 28 (+0xe0)
    Ptr(ret0 as *const ()),                   // 29 (+0xe8)
    Ptr(ret0 as *const ()),                   // 30 (+0xf0)
    Ptr(ret0 as *const ()),                   // 31 (+0xf8)
];

// ---------------------------------------------------------------------------
// Gu::ConvexMesh vtable (17 slots, matches 0x180166008 in PhysX3Common_x64.dll)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn convex_get_nb_vertices(this: *const u8) -> u32 {
    if this.is_null() {
        0
    } else {
        *this.add(0x46) as u32
    }
}

unsafe extern "win64" fn convex_get_vertices(this: *const u8) -> *const c_void {
    if this.is_null() {
        return std::ptr::null();
    }
    let nb_polys = *this.add(0x47) as usize;
    let hull_data = std::ptr::read_unaligned(this.add(0x48) as *const usize);
    if hull_data == 0 {
        std::ptr::null()
    } else {
        (hull_data + nb_polys * 20) as *const c_void
    }
}

unsafe extern "win64" fn convex_get_index_buffer(this: *const u8) -> *const u8 {
    if this.is_null() {
        return std::ptr::null();
    }
    let nb_edges = std::ptr::read_unaligned(this.add(0x44) as *const i16);
    let nb_verts = *this.add(0x46) as usize;
    let nb_polys = *this.add(0x47) as usize;
    let hull_data = std::ptr::read_unaligned(this.add(0x48) as *const usize);
    if hull_data == 0 {
        return std::ptr::null();
    }
    let edges_abs = (nb_edges as u16 & 0x7fff) as usize;
    let mut off = nb_verts * 15 + (edges_abs + nb_polys * 10) * 2;
    if nb_edges < 0 {
        off += edges_abs * 4;
    }
    (hull_data + off) as *const u8
}

unsafe extern "win64" fn convex_get_nb_polygons(this: *const u8) -> u32 {
    if this.is_null() {
        0
    } else {
        *this.add(0x47) as u32
    }
}

unsafe extern "win64" fn convex_get_polygon_data(
    this: *const u8,
    index: u32,
    data: *mut u8,
) -> u8 {
    if this.is_null() || data.is_null() {
        return 0;
    }
    let nb_polys = *this.add(0x47) as u32;
    if index >= nb_polys {
        return 0;
    }
    let hull_data = std::ptr::read_unaligned(this.add(0x48) as *const *const u8);
    if hull_data.is_null() {
        return 0;
    }
    let src = hull_data.add((index as usize) * 20);
    std::ptr::copy_nonoverlapping(src, data, 16);
    let nb_verts = *src.add(0x12) as u16;
    let index_base = std::ptr::read_unaligned(src.add(0x10) as *const u16);
    std::ptr::write_unaligned(data.add(0x10) as *mut u16, nb_verts);
    std::ptr::write_unaligned(data.add(0x12) as *mut u16, index_base);
    1
}

unsafe extern "win64" fn convex_get_local_bounds(
    this: *const u8,
    sret: *mut PxBounds3,
) -> *mut PxBounds3 {
    if !sret.is_null() {
        let b = if this.is_null() {
            PxBounds3 {
                minimum: [-1.0, -1.0, -1.0],
                maximum: [1.0, 1.0, 1.0],
            }
        } else {
            let cx = std::ptr::read_unaligned(this.add(0x20) as *const f32);
            let cy = std::ptr::read_unaligned(this.add(0x24) as *const f32);
            let cz = std::ptr::read_unaligned(this.add(0x28) as *const f32);
            let ex = std::ptr::read_unaligned(this.add(0x2c) as *const f32);
            let ey = std::ptr::read_unaligned(this.add(0x30) as *const f32);
            let ez = std::ptr::read_unaligned(this.add(0x34) as *const f32);
            if ex >= 0.0 && ey >= 0.0 && ez >= 0.0 && (ex > 0.0 || ey > 0.0 || ez > 0.0) {
                PxBounds3 {
                    minimum: [cx - ex, cy - ey, cz - ez],
                    maximum: [cx + ex, cy + ey, cz + ez],
                }
            } else {
                PxBounds3 {
                    minimum: [-1.0, -1.0, -1.0],
                    maximum: [1.0, 1.0, 1.0],
                }
            }
        };
        std::ptr::write_unaligned(sret, b);
    }
    sret
}

static CONVEX_MESH_VTBL: [Ptr; 17] = [
    Ptr(base_obj_release as *const ()),          // 0 (+0x00): release
    Ptr(base_obj_get_type_name as *const ()),    // 1 (+0x08): getConcreteTypeName
    Ptr(base_obj_is_releasable as *const ()),    // 2 (+0x10): isReleasable
    Ptr(base_obj_dtor as *const ()),             // 3 (+0x18): ~ConvexMesh
    Ptr(base_obj_is_kind_of as *const ()),       // 4 (+0x20): isKindOf
    Ptr(convex_get_nb_vertices as *const ()),    // 5 (+0x28): getNbVertices
    Ptr(convex_get_vertices as *const ()),       // 6 (+0x30): getVertices
    Ptr(convex_get_index_buffer as *const ()),   // 7 (+0x38): getIndexBuffer
    Ptr(convex_get_nb_polygons as *const ()),    // 8 (+0x40): getNbPolygons
    Ptr(convex_get_polygon_data as *const ()),   // 9 (+0x48): getPolygonData
    Ptr(ret1 as *const ()),                      // 10 (+0x50): getReferenceCount
    Ptr(ret0 as *const ()),                      // 11 (+0x58): acquireReference
    Ptr(ret0 as *const ()),                      // 12 (+0x60): getMassInformation
    Ptr(convex_get_local_bounds as *const ()),   // 13 (+0x68): getLocalBounds (sret)
    Ptr(ret0 as *const ()),                      // 14 (+0x70): isGpuCompatible
    Ptr(ret0 as *const ()),                      // 15 (+0x78): exportExtraData
    Ptr(ret0 as *const ()),                      // 16 (+0x80)
];

// ---------------------------------------------------------------------------
// Gu::BV4TriangleMesh / Gu::RTreeTriangleMesh vtable (19 slots, matches 0x1801676c8)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn trimesh_get_nb_vertices(this: *const u8) -> u32 {
    if this.is_null() {
        0
    } else {
        std::ptr::read_unaligned(this.add(0x20) as *const u32)
    }
}

unsafe extern "win64" fn trimesh_get_vertices(this: *const u8) -> *const c_void {
    if this.is_null() {
        std::ptr::null()
    } else {
        std::ptr::read_unaligned(this.add(0x28) as *const *const c_void)
    }
}

unsafe extern "win64" fn trimesh_get_nb_triangles(this: *const u8) -> u32 {
    if this.is_null() {
        0
    } else {
        std::ptr::read_unaligned(this.add(0x24) as *const u32)
    }
}

unsafe extern "win64" fn trimesh_get_triangles(this: *const u8) -> *const c_void {
    if this.is_null() {
        std::ptr::null()
    } else {
        std::ptr::read_unaligned(this.add(0x30) as *const *const c_void)
    }
}

unsafe extern "win64" fn trimesh_get_flags(this: *const u8, sret: *mut u8) -> *mut u8 {
    if !sret.is_null() {
        *sret = if this.is_null() { 0 } else { *this.add(0x5c) };
    }
    sret
}

unsafe extern "win64" fn trimesh_get_remapping(this: *const u8) -> *const u32 {
    if this.is_null() {
        std::ptr::null()
    } else {
        std::ptr::read_unaligned(this.add(0x68) as *const *const u32)
    }
}

unsafe extern "win64" fn trimesh_get_material_index(this: *const u8, tri_idx: u32) -> u16 {
    if this.is_null() {
        return 0xffff;
    }
    let table = std::ptr::read_unaligned(this.add(0x60) as *const *const u16);
    if table.is_null() {
        0xffff
    } else {
        std::ptr::read_unaligned(table.add(tri_idx as usize))
    }
}

unsafe extern "win64" fn trimesh_get_local_bounds(
    this: *const u8,
    sret: *mut PxBounds3,
) -> *mut PxBounds3 {
    if !sret.is_null() {
        let b = if this.is_null() {
            PxBounds3 {
                minimum: [-1.0, -1.0, -1.0],
                maximum: [1.0, 1.0, 1.0],
            }
        } else {
            let cx = std::ptr::read_unaligned(this.add(0x38) as *const f32);
            let cy = std::ptr::read_unaligned(this.add(0x3c) as *const f32);
            let cz = std::ptr::read_unaligned(this.add(0x40) as *const f32);
            let ex = std::ptr::read_unaligned(this.add(0x44) as *const f32);
            let ey = std::ptr::read_unaligned(this.add(0x48) as *const f32);
            let ez = std::ptr::read_unaligned(this.add(0x4c) as *const f32);
            if ex >= 0.0 && ey >= 0.0 && ez >= 0.0 && (ex > 0.0 || ey > 0.0 || ez > 0.0) {
                PxBounds3 {
                    minimum: [cx - ex, cy - ey, cz - ez],
                    maximum: [cx + ex, cy + ey, cz + ez],
                }
            } else {
                PxBounds3 {
                    minimum: [-1.0, -1.0, -1.0],
                    maximum: [1.0, 1.0, 1.0],
                }
            }
        };
        std::ptr::write_unaligned(sret, b);
    }
    sret
}

static TRIANGLE_MESH_VTBL: [Ptr; 19] = [
    Ptr(base_obj_release as *const ()),           // 0 (+0x00): release
    Ptr(base_obj_get_type_name as *const ()),     // 1 (+0x08): getConcreteTypeName
    Ptr(base_obj_is_releasable as *const ()),     // 2 (+0x10): isReleasable
    Ptr(base_obj_dtor as *const ()),              // 3 (+0x18): ~TriangleMesh
    Ptr(base_obj_is_kind_of as *const ()),        // 4 (+0x20): isKindOf
    Ptr(trimesh_get_nb_vertices as *const ()),    // 5 (+0x28): getNbVertices
    Ptr(trimesh_get_vertices as *const ()),       // 6 (+0x30): getVertices
    Ptr(ret0 as *const ()),                       // 7 (+0x38): getVerticesForModification
    Ptr(trimesh_get_local_bounds as *const ()),   // 8 (+0x40): refitBVH (sret PxBounds3)
    Ptr(trimesh_get_nb_triangles as *const ()),   // 9 (+0x48): getNbTriangles
    Ptr(trimesh_get_triangles as *const ()),      // 10 (+0x50): getTriangles
    Ptr(trimesh_get_flags as *const ()),          // 11 (+0x58): getTriangleMeshFlags (sret u8)
    Ptr(trimesh_get_remapping as *const ()),      // 12 (+0x60): getTrianglesRemapping
    Ptr(trimesh_get_material_index as *const ()), // 13 (+0x68): getTriangleMaterialIndex
    Ptr(trimesh_get_local_bounds as *const ()),   // 14 (+0x70): getLocalBounds (sret PxBounds3)
    Ptr(ret1 as *const ()),                       // 15 (+0x78): getReferenceCount
    Ptr(ret0 as *const ()),                       // 16 (+0x80): acquireReference
    Ptr(ret0 as *const ()),                       // 17 (+0x88): exportExtraData
    Ptr(ret0 as *const ()),                       // 18 (+0x90)
];

unsafe fn alloc_generic_base_obj(concrete_type: u16, size: usize) -> *mut c_void {
    ensure_vtables_registered();
    let ptr = allocator_allocate(
        std::ptr::null_mut(),
        size.max(0x40),
        b"PxBase\0".as_ptr(),
        b"physx3_x64.rs\0".as_ptr(),
        1,
    ) as *mut u8;
    if !ptr.is_null() {
        let vtbl_ptr: *const Ptr = match concrete_type {
            2 => CONVEX_MESH_VTBL.as_ptr(),
            3 | 4 => TRIANGLE_MESH_VTBL.as_ptr(),
            10 => CONSTRAINT_VTBL.as_ptr(),
            14 => AGGREGATE_VTBL.as_ptr(),
            _ => GENERIC_BASE_VTBL.as_ptr(),
        };
        *(ptr as *mut *const Ptr) = vtbl_ptr;
        *(ptr.add(0x08) as *mut u16) = concrete_type;
        *(ptr.add(0x0a) as *mut u16) = 3; // eOWNS_MEMORY | eIS_RELEASABLE
        *(ptr.add(0x18) as *mut u32) = 1; // ref_count = 1
        if concrete_type == 2 {
            // Initialize ConvexMesh local bounds extents (+0x2c..+0x38) to (1.0, 1.0, 1.0)
            std::ptr::write_unaligned(ptr.add(0x2c) as *mut [f32; 3], [1.0, 1.0, 1.0]);
        } else if concrete_type == 3 || concrete_type == 4 {
            // Initialize TriangleMesh local bounds extents (+0x44..+0x50) to (1.0, 1.0, 1.0)
            std::ptr::write_unaligned(ptr.add(0x44) as *mut [f32; 3], [1.0, 1.0, 1.0]);
        }
    }
    ptr as *mut c_void
}

unsafe extern "win64" fn insertion_build_object(
    _this: *mut PxInsertionCallbackBridge,
    concrete_type: u32,
    _data: *mut c_void,
) -> *mut c_void {
    alloc_generic_base_obj(concrete_type as u16, 0x118)
}

unsafe extern "win64" fn insertion_dtor(
    this: *mut PxInsertionCallbackBridge,
    _flags: u32,
) -> *mut PxInsertionCallbackBridge {
    this
}

static INSERTION_CB_VTBL: [Ptr; 2] = [
    Ptr(insertion_build_object as *const ()), // 0 (+0x00): buildObjectFromData
    Ptr(insertion_dtor as *const ()),         // 1 (+0x08): ~PxPhysicsInsertionCallback
];

// ---------------------------------------------------------------------------
// PxMaterial vtable (20 slots, matches 0x1801ad7c0)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn material_get_ref_count(this: *const PxMaterialInstance) -> u32 {
    if this.is_null() {
        1
    } else {
        (*this).ref_count.max(1)
    }
}

unsafe extern "win64" fn material_acquire_ref(this: *mut PxMaterialInstance) {
    if !this.is_null() {
        (*this).ref_count = (*this).ref_count.saturating_add(1);
    }
}

unsafe extern "win64" fn material_set_dyn_friction(this: *mut PxMaterialInstance, v: f32) {
    if !this.is_null() {
        (*this).dynamic_friction = v;
    }
}
crate::win64_bridge!(fn material_set_dyn_friction(*mut PxMaterialInstance, f32) -> ());

unsafe extern "win64" fn material_get_dyn_friction(this: *const PxMaterialInstance) -> f32 {
    if this.is_null() {
        0.5
    } else {
        (*this).dynamic_friction
    }
}

unsafe extern "win64" fn material_set_stat_friction(this: *mut PxMaterialInstance, v: f32) {
    if !this.is_null() {
        (*this).static_friction = v;
    }
}
crate::win64_bridge!(fn material_set_stat_friction(*mut PxMaterialInstance, f32) -> ());

unsafe extern "win64" fn material_get_stat_friction(this: *const PxMaterialInstance) -> f32 {
    if this.is_null() {
        0.5
    } else {
        (*this).static_friction
    }
}

unsafe extern "win64" fn material_set_restitution(this: *mut PxMaterialInstance, v: f32) {
    if !this.is_null() {
        (*this).restitution = v;
    }
}
crate::win64_bridge!(fn material_set_restitution(*mut PxMaterialInstance, f32) -> ());

unsafe extern "win64" fn material_get_restitution(this: *const PxMaterialInstance) -> f32 {
    if this.is_null() {
        0.0
    } else {
        (*this).restitution
    }
}

static MATERIAL_VTBL: [Ptr; 20] = [
    Ptr(base_obj_release as *const ()),          // 0 (+0x00)
    Ptr(base_obj_get_type_name as *const ()),    // 1 (+0x08)
    Ptr(base_obj_is_releasable as *const ()),    // 2 (+0x10)
    Ptr(base_obj_dtor as *const ()),             // 3 (+0x18)
    Ptr(base_obj_is_kind_of as *const ()),       // 4 (+0x20)
    Ptr(material_get_ref_count as *const ()),    // 5 (+0x28)
    Ptr(material_acquire_ref as *const ()),      // 6 (+0x30)
    Ptr(crate::bridge_entry!(material_set_dyn_friction)),  // 7 (+0x38)
    Ptr(material_get_dyn_friction as *const ()), // 8 (+0x40)
    Ptr(crate::bridge_entry!(material_set_stat_friction)), // 9 (+0x48)
    Ptr(material_get_stat_friction as *const ()),// 10 (+0x50)
    Ptr(crate::bridge_entry!(material_set_restitution)),   // 11 (+0x58)
    Ptr(material_get_restitution as *const ()),  // 12 (+0x60)
    Ptr(ret0 as *const ()),                      // 13 (+0x68): setFlag
    Ptr(ret0 as *const ()),                      // 14 (+0x70): setFlags
    Ptr(sret_zero_u16 as *const ()),             // 15 (+0x78): getFlags (sret)
    Ptr(ret0 as *const ()),                      // 16 (+0x80): setFrictionCombineMode
    Ptr(ret0 as *const ()),                      // 17 (+0x88): getFrictionCombineMode
    Ptr(ret0 as *const ()),                      // 18 (+0x90): setRestitutionCombineMode
    Ptr(ret0 as *const ()),                      // 19 (+0x98): getRestitutionCombineMode
];

// ---------------------------------------------------------------------------
// PxShape vtable (42 slots, matches 0x1801b2930)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn shape_get_geom_type(this: *const PxShapeInstance) -> u32 {
    if this.is_null() {
        0
    } else {
        (*this).geom_type
    }
}

unsafe extern "win64" fn shape_get_geometry_holder(
    this: *const PxShapeInstance,
    sret: *mut [u8; 0x30],
) -> *mut [u8; 0x30] {
    if !sret.is_null() {
        if this.is_null() {
            std::ptr::write_bytes(sret as *mut u8, 0, 0x30);
        } else {
            let src = &(*this).geom_type as *const u32 as *const u8;
            std::ptr::copy_nonoverlapping(src, sret as *mut u8, 0x30);
        }
    }
    sret
}

unsafe fn shape_copy_geom_if_type(
    this: *const PxShapeInstance,
    out: *mut u8,
    expected_type: u32,
    bytes: usize,
) -> u8 {
    if this.is_null() || out.is_null() || (*this).geom_type != expected_type {
        return 0;
    }
    let src = &(*this).geom_type as *const u32 as *const u8;
    std::ptr::copy_nonoverlapping(src, out, bytes);
    1
}

unsafe extern "win64" fn shape_get_box_geom(this: *const PxShapeInstance, out: *mut u8) -> u8 {
    shape_copy_geom_if_type(this, out, 3, 0x10)
}

unsafe extern "win64" fn shape_get_sphere_geom(this: *const PxShapeInstance, out: *mut u8) -> u8 {
    shape_copy_geom_if_type(this, out, 0, 0x08)
}

unsafe extern "win64" fn shape_get_capsule_geom(this: *const PxShapeInstance, out: *mut u8) -> u8 {
    shape_copy_geom_if_type(this, out, 2, 0x0c)
}

unsafe extern "win64" fn shape_get_plane_geom(this: *const PxShapeInstance, out: *mut u8) -> u8 {
    shape_copy_geom_if_type(this, out, 1, 0x04)
}

unsafe extern "win64" fn shape_get_convex_geom(this: *const PxShapeInstance, out: *mut u8) -> u8 {
    shape_copy_geom_if_type(this, out, 4, 0x30)
}

unsafe extern "win64" fn shape_get_trimesh_geom(this: *const PxShapeInstance, out: *mut u8) -> u8 {
    shape_copy_geom_if_type(this, out, 5, 0x30)
}

unsafe extern "win64" fn shape_get_hf_geom(this: *const PxShapeInstance, out: *mut u8) -> u8 {
    shape_copy_geom_if_type(this, out, 6, 0x20)
}

unsafe extern "win64" fn shape_get_actor(this: *const PxShapeInstance) -> *mut c_void {
    if this.is_null() {
        std::ptr::null_mut()
    } else {
        (*this).actor
    }
}

unsafe extern "win64" fn shape_set_local_pose(
    this: *mut PxShapeInstance,
    pose: *const PxTransform,
) {
    if !this.is_null() && !pose.is_null() {
        (*this).local_pose = std::ptr::read_unaligned(pose);
    }
}

unsafe extern "win64" fn shape_get_local_pose(
    this: *const PxShapeInstance,
    sret: *mut PxTransform,
) -> *mut PxTransform {
    if !sret.is_null() {
        let mut p = if this.is_null() {
            PxTransform {
                q: [0.0, 0.0, 0.0, 1.0],
                p: [0.0, 0.0, 0.0],
            }
        } else {
            (*this).local_pose
        };
        let norm_sq = p.q[0] * p.q[0] + p.q[1] * p.q[1] + p.q[2] * p.q[2] + p.q[3] * p.q[3];
        if !(norm_sq > 1e-6 && norm_sq.is_finite()) {
            p.q = [0.0, 0.0, 0.0, 1.0];
        }
        std::ptr::write_unaligned(sret, p);
    }
    sret
}

unsafe extern "win64" fn shape_set_sim_filter_data(this: *mut PxShapeInstance, data: *const [u32; 4]) {
    if !this.is_null() && !data.is_null() {
        (*this).sim_filter_data = std::ptr::read_unaligned(data);
    }
}

unsafe extern "win64" fn shape_get_sim_filter_data(
    this: *const PxShapeInstance,
    sret: *mut [u32; 4],
) -> *mut [u32; 4] {
    if !sret.is_null() {
        let d = if this.is_null() { [0; 4] } else { (*this).sim_filter_data };
        std::ptr::write_unaligned(sret, d);
    }
    sret
}

unsafe extern "win64" fn shape_set_query_filter_data(
    this: *mut PxShapeInstance,
    data: *const [u32; 4],
) {
    if !this.is_null() && !data.is_null() {
        (*this).query_filter_data = std::ptr::read_unaligned(data);
    }
}

unsafe extern "win64" fn shape_get_query_filter_data(
    this: *const PxShapeInstance,
    sret: *mut [u32; 4],
) -> *mut [u32; 4] {
    if !sret.is_null() {
        let d = if this.is_null() { [0; 4] } else { (*this).query_filter_data };
        std::ptr::write_unaligned(sret, d);
    }
    sret
}

unsafe extern "win64" fn shape_get_nb_materials(this: *const PxShapeInstance) -> u16 {
    if this.is_null() {
        return 1;
    }
    let raw = this as *const u8;
    match (*this).geom_type {
        5 => std::ptr::read_unaligned(raw.add(0xe0) as *const u16).max(1),
        6 => std::ptr::read_unaligned(raw.add(0xc8) as *const u16).max(1),
        _ => 1,
    }
}

unsafe extern "win64" fn shape_get_materials(
    this: *const PxShapeInstance,
    user_buffer: *mut *mut c_void,
    buffer_size: u32,
    start_index: u32,
) -> u32 {
    if this.is_null() || user_buffer.is_null() || buffer_size == 0 || start_index > 0 {
        return 0;
    }
    let mat = (*this)._pad3 as *mut c_void;
    std::ptr::write_unaligned(user_buffer, mat);
    1
}

unsafe extern "win64" fn shape_get_flags(this: *const PxShapeInstance, sret: *mut u8) -> *mut u8 {
    if !sret.is_null() {
        *sret = if this.is_null() { 0 } else { (*this).shape_flags };
    }
    sret
}

unsafe extern "win64" fn shape_is_exclusive(this: *const PxShapeInstance) -> u8 {
    if this.is_null() {
        0
    } else {
        (((*this).exclusive_flag & 0x8000_0000) != 0) as u8
    }
}

static SHAPE_VTBL: [Ptr; 42] = [
    Ptr(base_obj_release as *const ()),          // 0 (+0x00)
    Ptr(base_obj_get_type_name as *const ()),    // 1 (+0x08)
    Ptr(base_obj_is_releasable as *const ()),    // 2 (+0x10)
    Ptr(base_obj_dtor as *const ()),             // 3 (+0x18)
    Ptr(base_obj_is_kind_of as *const ()),       // 4 (+0x20)
    Ptr(ret1 as *const ()),                      // 5 (+0x28): getReferenceCount
    Ptr(shape_get_geom_type as *const ()),       // 6 (+0x30): getGeometryType
    Ptr(ret0 as *const ()),                      // 7 (+0x38): setGeometry
    Ptr(shape_get_geometry_holder as *const ()), // 8 (+0x40): getGeometry (sret 0x30B)
    Ptr(shape_get_box_geom as *const ()),        // 9 (+0x48): getBoxGeometry
    Ptr(shape_get_sphere_geom as *const ()),     // 10 (+0x50): getSphereGeometry
    Ptr(shape_get_capsule_geom as *const ()),    // 11 (+0x58): getCapsuleGeometry
    Ptr(shape_get_plane_geom as *const ()),      // 12 (+0x60): getPlaneGeometry
    Ptr(shape_get_convex_geom as *const ()),     // 13 (+0x68): getConvexMeshGeometry
    Ptr(shape_get_trimesh_geom as *const ()),    // 14 (+0x70): getTriangleMeshGeometry
    Ptr(shape_get_hf_geom as *const ()),         // 15 (+0x78): getHeightFieldGeometry
    Ptr(shape_get_actor as *const ()),           // 16 (+0x80): getActor
    Ptr(shape_set_local_pose as *const ()),      // 17 (+0x88): setLocalPose
    Ptr(shape_get_local_pose as *const ()),      // 18 (+0x90): getLocalPose (sret)
    Ptr(shape_set_sim_filter_data as *const ()), // 19 (+0x98): setSimulationFilterData
    Ptr(shape_get_sim_filter_data as *const ()), // 20 (+0xa0): getSimulationFilterData (sret 16B)
    Ptr(shape_set_query_filter_data as *const ()), // 21 (+0xa8): setQueryFilterData
    Ptr(shape_get_query_filter_data as *const ()), // 22 (+0xb0): getQueryFilterData (sret 16B)
    Ptr(ret0 as *const ()),                      // 23 (+0xb8): setMaterials
    Ptr(shape_get_nb_materials as *const ()),    // 24 (+0xc0): getNbMaterials
    Ptr(shape_get_materials as *const ()),       // 25 (+0xc8): getMaterials
    Ptr(ret0 as *const ()),                      // 26 (+0xd0): getMaterialFromInternalFaceIndex
    Ptr(ret0 as *const ()),                      // 27 (+0xd8): setContactOffset
    Ptr(ret_f32_one as *const ()),               // 28 (+0xe0): getContactOffset
    Ptr(ret0 as *const ()),                      // 29 (+0xe8): setRestOffset
    Ptr(ret0 as *const ()),                      // 30 (+0xf0): getRestOffset
    Ptr(ret0 as *const ()),                      // 31 (+0xf8): setFlag
    Ptr(ret0 as *const ()),                      // 32 (+0x100): setFlags
    Ptr(shape_get_flags as *const ()),           // 33 (+0x108): getFlags (sret)
    Ptr(shape_is_exclusive as *const ()),        // 34 (+0x110): isExclusive
    Ptr(ret0 as *const ()),                      // 35 (+0x118): setName
    Ptr(ret0 as *const ()),                      // 36 (+0x120): getName
    Ptr(ret0 as *const ()),                      // 37 (+0x128)
    Ptr(ret0 as *const ()),                      // 38 (+0x130)
    Ptr(ret0 as *const ()),                      // 39 (+0x138)
    Ptr(ret0 as *const ()),                      // 40 (+0x140)
    Ptr(ret0 as *const ()),                      // 41 (+0x148)
];

// ---------------------------------------------------------------------------
// PxRigidStatic / PxRigidDynamic vtable (82 slots, matches 0x1801af900 / 0x1801affc8)
// ---------------------------------------------------------------------------

#[inline]
unsafe fn actor_pose_ptr(this: *const PxRigidActorInstance) -> *mut PxTransform {
    let raw = this as *mut u8;
    if (*this).concrete_type == 7 {
        raw.add(0x90) as *mut PxTransform
    } else {
        raw.add(0xb0) as *mut PxTransform
    }
}

unsafe extern "win64" fn actor_get_global_pose(
    this: *const PxRigidActorInstance,
    sret: *mut PxTransform,
) -> *mut PxTransform {
    if !sret.is_null() {
        let mut p = if this.is_null() {
            PxTransform {
                q: [0.0, 0.0, 0.0, 1.0],
                p: [0.0, 0.0, 0.0],
            }
        } else {
            std::ptr::read_unaligned(actor_pose_ptr(this))
        };
        let norm_sq = p.q[0] * p.q[0] + p.q[1] * p.q[1] + p.q[2] * p.q[2] + p.q[3] * p.q[3];
        if !(norm_sq > 1e-6 && norm_sq.is_finite()) {
            p.q = [0.0, 0.0, 0.0, 1.0];
        }
        std::ptr::write_unaligned(sret, p);
    }
    sret
}

unsafe extern "win64" fn actor_set_global_pose(
    this: *mut PxRigidActorInstance,
    pose: *const PxTransform,
    _autowake: u8,
) {
    if !this.is_null() && !pose.is_null() {
        std::ptr::write_unaligned(actor_pose_ptr(this), std::ptr::read_unaligned(pose));
    }
}

unsafe extern "win64" fn actor_attach_shape(
    this: *mut PxRigidActorInstance,
    shape: *mut PxShapeInstance,
) -> u8 {
    if this.is_null() || shape.is_null() {
        return 0;
    }
    let rc_ptr = (&raw const (*shape).ref_count) as *const std::sync::atomic::AtomicU32;
    (*rc_ptr).fetch_add(1, Ordering::AcqRel);
    (*shape).actor = this as *mut c_void;
    let nb = (*this).nb_shapes as usize;
    if nb == 0 {
        (*this).shapes = shape as *mut c_void;
        (*this).nb_shapes = 1;
        (*this).shapes_own_mem = 1;
    } else if nb == 1 {
        let arr = allocator_allocate(
            std::ptr::null_mut(),
            2 * std::mem::size_of::<*mut c_void>(),
            b"PtrTable\0".as_ptr(),
            b"physx3_x64.rs\0".as_ptr(),
            1,
        ) as *mut *mut c_void;
        if !arr.is_null() {
            *arr = (*this).shapes;
            *arr.add(1) = shape as *mut c_void;
            (*this).shapes = arr as *mut c_void;
            (*this).nb_shapes = 2;
            (*this).shapes_own_mem = 1;
        }
    } else {
        let arr = allocator_allocate(
            std::ptr::null_mut(),
            (nb + 1) * std::mem::size_of::<*mut c_void>(),
            b"PtrTable\0".as_ptr(),
            b"physx3_x64.rs\0".as_ptr(),
            1,
        ) as *mut *mut c_void;
        if !arr.is_null() {
            let old = (*this).shapes as *mut *mut c_void;
            if !old.is_null() {
                std::ptr::copy_nonoverlapping(old, arr, nb);
                if (*this).shapes_own_mem != 0 {
                    allocator_deallocate(std::ptr::null_mut(), old as *mut c_void);
                }
            }
            *arr.add(nb) = shape as *mut c_void;
            (*this).shapes = arr as *mut c_void;
            (*this).nb_shapes = (nb + 1) as u16;
            (*this).shapes_own_mem = 1;
        }
    }
    1
}

unsafe extern "win64" fn actor_detach_shape(
    this: *mut PxRigidActorInstance,
    shape: *mut PxShapeInstance,
    _wake_on_lost_touch: u8,
) {
    if this.is_null() || shape.is_null() {
        return;
    }
    let nb = (*this).nb_shapes as usize;
    if nb == 0 {
        return;
    }
    if nb == 1 {
        if (*this).shapes != shape as *mut c_void {
            return;
        }
        (*this).shapes = std::ptr::null_mut();
        (*this).nb_shapes = 0;
        (*this)._pad_sm0[0] = 0;
    } else {
        let arr = (*this).shapes as *mut *mut c_void;
        if arr.is_null() {
            (*this).nb_shapes = 0;
            return;
        }
        let mut found = None;
        for i in 0..nb {
            if std::ptr::read_unaligned(arr.add(i)) == shape as *mut c_void {
                found = Some(i);
                break;
            }
        }
        let Some(idx) = found else {
            return;
        };
        if nb == 2 {
            let remaining = std::ptr::read_unaligned(arr.add(1 - idx));
            if (*this).shapes_own_mem != 0 {
                allocator_deallocate(std::ptr::null_mut(), arr as *mut c_void);
            }
            (*this).shapes = remaining;
            (*this).nb_shapes = 1;
            (*this).shapes_own_mem = 1;
            (*this)._pad_sm0[0] = 1;
        } else {
            let last = std::ptr::read_unaligned(arr.add(nb - 1));
            std::ptr::write_unaligned(arr.add(idx), last);
            (*this).nb_shapes = (nb - 1) as u16;
        }
    }
    (*shape).actor = std::ptr::null_mut();
    base_obj_release(shape as *mut u8);
}

unsafe extern "win64" fn actor_get_nb_shapes(this: *const PxRigidActorInstance) -> u32 {
    if this.is_null() {
        0
    } else {
        (*this).nb_shapes as u32
    }
}

unsafe extern "win64" fn actor_get_shapes(
    this: *const PxRigidActorInstance,
    user_buffer: *mut *mut c_void,
    buffer_size: u32,
    start_index: u32,
) -> u32 {
    if this.is_null() || user_buffer.is_null() || buffer_size == 0 {
        return 0;
    }
    let total = (*this).nb_shapes as u32;
    if start_index >= total {
        return 0;
    }
    let count = (total - start_index).min(buffer_size);
    let src_base: *const *mut c_void = if total == 1 {
        &(*this).shapes as *const *mut c_void
    } else {
        (*this).shapes as *const *mut c_void
    };
    if src_base.is_null() {
        return 0;
    }
    for i in 0..count {
        let ptr = std::ptr::read_unaligned(src_base.add((start_index + i) as usize));
        std::ptr::write_unaligned(user_buffer.add(i as usize), ptr);
    }
    count
}

unsafe extern "win64" fn actor_create_exclusive_shape(
    this: *mut PxRigidActorInstance,
    geom: *const u32,
    materials: *const *mut PxMaterialInstance,
    material_count: u16,
    shape_flags_ptr: *const u8,
) -> *mut PxShapeInstance {
    let s = physics_create_shape(
        std::ptr::null_mut(),
        geom,
        materials,
        material_count,
        1,
        shape_flags_ptr,
    );
    if !s.is_null() && !this.is_null() {
        let _ = actor_attach_shape(this, s);
        let rc_ptr = (&raw const (*s).ref_count) as *const std::sync::atomic::AtomicU32;
        (*rc_ptr).fetch_sub(1, Ordering::AcqRel);
    }
    s
}

unsafe extern "win64" fn actor_set_rigid_body_flag(
    this: *mut PxRigidActorInstance,
    flag: u32,
    value: u8,
) {
    if !this.is_null() && (*this).concrete_type == 6 {
        let p = (this as *mut u8).add(0xac);
        if value != 0 {
            *p |= flag as u8;
        } else {
            *p &= !(flag as u8);
        }
    }
}

unsafe extern "win64" fn actor_set_rigid_body_flags(
    this: *mut PxRigidActorInstance,
    flags_ptr: *const u8,
) {
    if !this.is_null() && !flags_ptr.is_null() && (*this).concrete_type == 6 {
        *(this as *mut u8).add(0xac) = *flags_ptr;
    }
}

unsafe extern "win64" fn actor_get_rigid_body_flags(
    this: *const PxRigidActorInstance,
    sret: *mut u8,
) -> *mut u8 {
    if !sret.is_null() {
        *sret = if this.is_null() || (*this).concrete_type != 6 {
            0
        } else {
            *(this as *const u8).add(0xac)
        };
    }
    sret
}

unsafe extern "win64" fn actor_get_solver_iteration_counts(
    _this: *const PxRigidActorInstance,
    min_pos: *mut u32,
    min_vel: *mut u32,
) {
    if !min_pos.is_null() {
        *min_pos = 4;
    }
    if !min_vel.is_null() {
        *min_vel = 1;
    }
}

static RIGID_ACTOR_VTBL: [Ptr; 82] = [
    Ptr(base_obj_release as *const ()),             // 0 (+0x000): release
    Ptr(base_obj_get_type_name as *const ()),       // 1 (+0x008): getConcreteTypeName
    Ptr(base_obj_is_releasable as *const ()),       // 2 (+0x010): isReleasable
    Ptr(base_obj_dtor as *const ()),                // 3 (+0x018): ~PxBase
    Ptr(base_obj_is_kind_of as *const ()),          // 4 (+0x020): isKindOf
    Ptr(ret0 as *const ()),                         // 5 (+0x028): getType
    Ptr(ret0 as *const ()),                         // 6 (+0x030): getScene
    Ptr(ret0 as *const ()),                         // 7 (+0x038): setName
    Ptr(ret0 as *const ()),                         // 8 (+0x040): getName
    Ptr(sret_default_bounds as *const ()),          // 9 (+0x048): getWorldBounds (sret)
    Ptr(ret0 as *const ()),                         // 10 (+0x050): setActorFlag
    Ptr(ret0 as *const ()),                         // 11 (+0x058): setActorFlags
    Ptr(sret_zero_u8 as *const ()),                 // 12 (+0x060): getActorFlags (sret)
    Ptr(ret0 as *const ()),                         // 13 (+0x068): setDominanceGroup
    Ptr(ret0 as *const ()),                         // 14 (+0x070): getDominanceGroup
    Ptr(ret0 as *const ()),                         // 15 (+0x078): setOwnerClient
    Ptr(ret0 as *const ()),                         // 16 (+0x080): getOwnerClient
    Ptr(ret0 as *const ()),                         // 17 (+0x088): setClientBehaviorFlags
    Ptr(sret_zero_u8 as *const ()),                 // 18 (+0x090): getClientBehaviorFlags (sret)
    Ptr(ret0 as *const ()),                         // 19 (+0x098): getAggregate
    Ptr(actor_get_global_pose as *const ()),        // 20 (+0x0a0): getGlobalPose (sret)
    Ptr(actor_set_global_pose as *const ()),        // 21 (+0x0a8): setGlobalPose
    Ptr(actor_create_exclusive_shape as *const ()), // 22 (+0x0b0): createExclusiveShape
    Ptr(actor_attach_shape as *const ()),           // 23 (+0x0b8): attachShape
    Ptr(actor_detach_shape as *const ()),           // 24 (+0x0c0): detachShape
    Ptr(actor_get_nb_shapes as *const ()),          // 25 (+0x0c8): getNbShapes
    Ptr(actor_get_shapes as *const ()),             // 26 (+0x0d0): getShapes
    Ptr(ret0 as *const ()),                         // 27 (+0x0d8): getNbConstraints
    Ptr(ret0 as *const ()),                         // 28 (+0x0e0): getConstraints
    Ptr(ret0 as *const ()),                         // 29 (+0x0e8): setCMassLocalPose
    Ptr(sret_identity_transform as *const ()),      // 30 (+0x0f0): getCMassLocalPose (sret)
    Ptr(ret0 as *const ()),                         // 31 (+0x0f8): setMass
    Ptr(ret_f32_one as *const ()),                  // 32 (+0x100): getMass
    Ptr(ret_f32_one as *const ()),                  // 33 (+0x108): getInvMass
    Ptr(ret0 as *const ()),                         // 34 (+0x110): setMassSpaceInertiaTensor
    Ptr(sret_one_vec3 as *const ()),                // 35 (+0x118): getMassSpaceInertiaTensor (sret)
    Ptr(sret_one_vec3 as *const ()),                // 36 (+0x120): getMassSpaceInvInertiaTensor (sret)
    Ptr(sret_zero_vec3 as *const ()),               // 37 (+0x128): getLinearVelocity (sret)
    Ptr(ret0 as *const ()),                         // 38 (+0x130): setLinearVelocity
    Ptr(sret_zero_vec3 as *const ()),               // 39 (+0x138): getAngularVelocity (sret)
    Ptr(ret0 as *const ()),                         // 40 (+0x140): setAngularVelocity
    Ptr(ret0 as *const ()),                         // 41 (+0x148): addForce
    Ptr(ret0 as *const ()),                         // 42 (+0x150): addTorque
    Ptr(ret0 as *const ()),                         // 43 (+0x158): clearForce
    Ptr(ret0 as *const ()),                         // 44 (+0x160): clearTorque
    Ptr(actor_set_rigid_body_flag as *const ()),    // 45 (+0x168): setRigidBodyFlag
    Ptr(actor_set_rigid_body_flags as *const ()),   // 46 (+0x170): setRigidBodyFlags
    Ptr(actor_get_rigid_body_flags as *const ()),   // 47 (+0x178): getRigidBodyFlags (sret)
    Ptr(ret0 as *const ()),                         // 48 (+0x180): setMinCCDAdvanceCoefficient
    Ptr(ret_f32_one as *const ()),                  // 49 (+0x188): getMinCCDAdvanceCoefficient
    Ptr(ret0 as *const ()),                         // 50 (+0x190): setMaxDepenetrationVelocity
    Ptr(ret_f32_one as *const ()),                  // 51 (+0x198): getMaxDepenetrationVelocity
    Ptr(ret0 as *const ()),                         // 52 (+0x1a0): setMaxContactImpulse
    Ptr(ret_f32_one as *const ()),                  // 53 (+0x1a8): getMaxContactImpulse
    Ptr(ret0 as *const ()),                         // 54 (+0x1b0): setKinematicTarget
    Ptr(ret0 as *const ()),                         // 55 (+0x1b8): getKinematicTarget
    Ptr(ret0 as *const ()),                         // 56 (+0x1c0): setLinearDamping
    Ptr(ret0 as *const ()),                         // 57 (+0x1c8): getLinearDamping
    Ptr(ret0 as *const ()),                         // 58 (+0x1d0): setAngularDamping
    Ptr(ret0 as *const ()),                         // 59 (+0x1d8): getAngularDamping
    Ptr(ret0 as *const ()),                         // 60 (+0x1e0): setMaxAngularVelocity
    Ptr(ret_f32_one as *const ()),                  // 61 (+0x1e8): getMaxAngularVelocity
    Ptr(ret0 as *const ()),                         // 62 (+0x1f0): isSleeping
    Ptr(ret0 as *const ()),                         // 63 (+0x1f8): setSleepThreshold
    Ptr(ret0 as *const ()),                         // 64 (+0x200): getSleepThreshold
    Ptr(ret0 as *const ()),                         // 65 (+0x208): setStabilizationThreshold
    Ptr(ret0 as *const ()),                         // 66 (+0x210): getStabilizationThreshold
    Ptr(sret_zero_u16 as *const ()),                // 67 (+0x218): getRigidDynamicLockFlags (sret)
    Ptr(ret0 as *const ()),                         // 68 (+0x220): setRigidDynamicLockFlag
    Ptr(ret0 as *const ()),                         // 69 (+0x228): setRigidDynamicLockFlags
    Ptr(ret0 as *const ()),                         // 70 (+0x230): setWakeCounter
    Ptr(ret0 as *const ()),                         // 71 (+0x238): getWakeCounter
    Ptr(ret0 as *const ()),                         // 72 (+0x240): wakeUp
    Ptr(ret0 as *const ()),                         // 73 (+0x248): putToSleep
    Ptr(ret0 as *const ()),                         // 74 (+0x250): setSolverIterationCounts
    Ptr(actor_get_solver_iteration_counts as *const ()), // 75 (+0x258): getSolverIterationCounts
    Ptr(ret_f32_one as *const ()),                  // 76 (+0x260): getContactReportThreshold
    Ptr(ret0 as *const ()),                         // 77 (+0x268): setContactReportThreshold
    Ptr(ret0 as *const ()),                         // 78 (+0x270): setMaxLinearVelocity
    Ptr(ret_f32_one as *const ()),                  // 79 (+0x278): getMaxLinearVelocity
    Ptr(ret0 as *const ()),                         // 80 (+0x280): exportExtraData
    Ptr(ret0 as *const ()),                         // 81 (+0x288): resolveReferences
];

// ---------------------------------------------------------------------------
// NpConstraint vtable (22 slots, matches 0x1801ad180 in PhysX3_x64.dll)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn constraint_get_actors(
    this: *const u8,
    actor0_out: *mut *mut c_void,
    actor1_out: *mut *mut c_void,
) {
    let (a0, a1) = if this.is_null() {
        (std::ptr::null_mut(), std::ptr::null_mut())
    } else {
        (
            std::ptr::read_unaligned(this.add(0x10) as *const *mut c_void),
            std::ptr::read_unaligned(this.add(0x18) as *const *mut c_void),
        )
    };
    if !actor0_out.is_null() {
        std::ptr::write_unaligned(actor0_out, a0);
    }
    if !actor1_out.is_null() {
        std::ptr::write_unaligned(actor1_out, a1);
    }
}

unsafe extern "win64" fn constraint_set_actors(
    this: *mut u8,
    actor0: *mut c_void,
    actor1: *mut c_void,
) {
    if !this.is_null() {
        std::ptr::write_unaligned(this.add(0x10) as *mut *mut c_void, actor0);
        std::ptr::write_unaligned(this.add(0x18) as *mut *mut c_void, actor1);
    }
}

unsafe extern "win64" fn constraint_get_force(
    _this: *const u8,
    linear: *mut [f32; 3],
    angular: *mut [f32; 3],
) {
    if !linear.is_null() {
        std::ptr::write_unaligned(linear, [0.0, 0.0, 0.0]);
    }
    if !angular.is_null() {
        std::ptr::write_unaligned(angular, [0.0, 0.0, 0.0]);
    }
}

unsafe extern "win64" fn constraint_get_break_force(
    _this: *const u8,
    linear: *mut f32,
    angular: *mut f32,
) {
    if !linear.is_null() {
        std::ptr::write_unaligned(linear, f32::MAX);
    }
    if !angular.is_null() {
        std::ptr::write_unaligned(angular, f32::MAX);
    }
}

unsafe extern "win64" fn constraint_get_external_reference(
    this: *const u8,
    type_id: *mut u32,
) -> *mut c_void {
    if !this.is_null() {
        let connector = std::ptr::read_unaligned(this.add(0x58) as *const usize);
        if connector != 0 {
            let vtbl = std::ptr::read_unaligned(connector as *const usize);
            if vtbl != 0 {
                let fn_ptr = std::ptr::read_unaligned((vtbl + 0x28) as *const usize);
                if fn_ptr != 0 {
                    return crate::call_guest8(
                        fn_ptr,
                        connector as u64,
                        type_id as u64,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ) as *mut c_void;
                }
            }
        }
    }
    if !type_id.is_null() {
        *type_id = 0;
    }
    std::ptr::null_mut()
}

unsafe extern "win64" fn constraint_set_functions(
    this: *mut u8,
    connector: *mut c_void,
    shaders: *const usize,
) {
    if !this.is_null() {
        std::ptr::write_unaligned(this.add(0x58) as *mut *mut c_void, connector);
        if !shaders.is_null() {
            std::ptr::write_unaligned(
                this.add(0x68) as *mut usize,
                std::ptr::read_unaligned(shaders),
            );
            std::ptr::write_unaligned(
                this.add(0x60) as *mut usize,
                std::ptr::read_unaligned(shaders.add(1)),
            );
            std::ptr::write_unaligned(
                this.add(0x70) as *mut usize,
                std::ptr::read_unaligned(shaders.add(2)),
            );
        }
    }
}

static CONSTRAINT_VTBL: [Ptr; 22] = [
    Ptr(base_obj_release as *const ()),                    // 0 (+0x00): release
    Ptr(base_obj_get_type_name as *const ()),              // 1 (+0x08): getConcreteTypeName
    Ptr(base_obj_is_releasable as *const ()),              // 2 (+0x10): isReleasable
    Ptr(base_obj_dtor as *const ()),                       // 3 (+0x18): ~PxConstraint
    Ptr(base_obj_is_kind_of as *const ()),                 // 4 (+0x20): isKindOf
    Ptr(ret0 as *const ()),                                // 5 (+0x28): getScene
    Ptr(constraint_get_actors as *const ()),               // 6 (+0x30): getActors
    Ptr(constraint_set_actors as *const ()),               // 7 (+0x38): setActors
    Ptr(ret0 as *const ()),                                // 8 (+0x40): markDirty
    Ptr(ret0 as *const ()),                                // 9 (+0x48): setFlags
    Ptr(sret_zero_u16 as *const ()),                       // 10 (+0x50): getFlags (sret)
    Ptr(ret0 as *const ()),                                // 11 (+0x58): setFlag
    Ptr(constraint_get_force as *const ()),                // 12 (+0x60): getForce
    Ptr(ret1 as *const ()),                                // 13 (+0x68): isValid
    Ptr(ret0 as *const ()),                                // 14 (+0x70): setBreakForce
    Ptr(constraint_get_break_force as *const ()),          // 15 (+0x78): getBreakForce
    Ptr(ret0 as *const ()),                                // 16 (+0x80): setMinResponseThreshold
    Ptr(ret0 as *const ()),                                // 17 (+0x88): getMinResponseThreshold
    Ptr(constraint_get_external_reference as *const ()),   // 18 (+0x90): getExternalReference
    Ptr(constraint_set_functions as *const ()),            // 19 (+0x98): setConstraintFunctions
    Ptr(ret0 as *const ()),                                // 20 (+0xa0)
    Ptr(ret1 as *const ()),                                // 21 (+0xa8)
];

// ---------------------------------------------------------------------------
// NpAggregate vtable (16 slots, matches 0x1801aae38 in PhysX3_x64.dll)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn aggregate_add_actor(
    this: *mut u8,
    actor: *mut c_void,
    _bvh: *const c_void,
) -> u8 {
    if this.is_null() || actor.is_null() {
        return 0;
    }
    let nb = std::ptr::read_unaligned(this.add(0x40) as *const u32) as usize;
    let max_nb = std::ptr::read_unaligned(this.add(0x34) as *const u32) as usize;
    let arr = std::ptr::read_unaligned(this.add(0x48) as *const *mut *mut c_void);
    if !arr.is_null() && nb < max_nb.max(nb + 1) {
        std::ptr::write_unaligned(arr.add(nb), actor);
        std::ptr::write_unaligned(this.add(0x40) as *mut u32, (nb + 1) as u32);
    }
    1
}

unsafe extern "win64" fn aggregate_remove_actor(this: *mut u8, actor: *mut c_void) -> u8 {
    if this.is_null() || actor.is_null() {
        return 0;
    }
    let nb = std::ptr::read_unaligned(this.add(0x40) as *const u32) as usize;
    let arr = std::ptr::read_unaligned(this.add(0x48) as *const *mut *mut c_void);
    if arr.is_null() || nb == 0 {
        return 0;
    }
    for i in 0..nb {
        if std::ptr::read_unaligned(arr.add(i)) == actor {
            let last = std::ptr::read_unaligned(arr.add(nb - 1));
            std::ptr::write_unaligned(arr.add(i), last);
            std::ptr::write_unaligned(this.add(0x40) as *mut u32, (nb - 1) as u32);
            return 1;
        }
    }
    0
}

unsafe extern "win64" fn aggregate_get_nb_actors(this: *const u8) -> u32 {
    if this.is_null() {
        0
    } else {
        std::ptr::read_unaligned(this.add(0x40) as *const u32)
    }
}

unsafe extern "win64" fn aggregate_get_max_nb_actors(this: *const u8) -> u32 {
    if this.is_null() {
        0
    } else {
        std::ptr::read_unaligned(this.add(0x34) as *const u32)
    }
}

unsafe extern "win64" fn aggregate_get_actors(
    this: *const u8,
    user_buffer: *mut *mut c_void,
    buffer_size: u32,
    start_index: u32,
) -> u32 {
    if this.is_null() || user_buffer.is_null() || buffer_size == 0 {
        return 0;
    }
    let total = std::ptr::read_unaligned(this.add(0x40) as *const u32);
    if start_index >= total {
        return 0;
    }
    let arr = std::ptr::read_unaligned(this.add(0x48) as *const *const *mut c_void);
    if arr.is_null() {
        return 0;
    }
    let count = (total - start_index).min(buffer_size);
    for i in 0..count {
        let p = std::ptr::read_unaligned(arr.add((start_index + i) as usize));
        std::ptr::write_unaligned(user_buffer.add(i as usize), p);
    }
    count
}

unsafe extern "win64" fn aggregate_get_self_collide(this: *const u8) -> u8 {
    if this.is_null() {
        0
    } else {
        *this.add(0x38)
    }
}

static AGGREGATE_VTBL: [Ptr; 16] = [
    Ptr(base_obj_release as *const ()),            // 0 (+0x00): release
    Ptr(base_obj_get_type_name as *const ()),      // 1 (+0x08): getConcreteTypeName
    Ptr(base_obj_is_releasable as *const ()),      // 2 (+0x10): isReleasable
    Ptr(base_obj_dtor as *const ()),               // 3 (+0x18): ~PxAggregate
    Ptr(base_obj_is_kind_of as *const ()),         // 4 (+0x20): isKindOf
    Ptr(aggregate_add_actor as *const ()),         // 5 (+0x28): addActor
    Ptr(aggregate_remove_actor as *const ()),      // 6 (+0x30): removeActor
    Ptr(ret1 as *const ()),                        // 7 (+0x38): addArticulation
    Ptr(ret1 as *const ()),                        // 8 (+0x40): removeArticulation
    Ptr(aggregate_get_nb_actors as *const ()),     // 9 (+0x48): getNbActors
    Ptr(aggregate_get_max_nb_actors as *const ()), // 10 (+0x50): getMaxNbActors
    Ptr(aggregate_get_actors as *const ()),        // 11 (+0x58): getActors
    Ptr(ret0 as *const ()),                        // 12 (+0x60): getScene
    Ptr(aggregate_get_self_collide as *const ()),  // 13 (+0x68): getSelfCollide
    Ptr(ret0 as *const ()),                        // 14 (+0x70): exportExtraData
    Ptr(ret0 as *const ()),                        // 15 (+0x78): requiresObjects
];

// ---------------------------------------------------------------------------
// PxScene vtable (128 slots, matches 0x1801b0a50)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn scene_release(this: *mut PxSceneInstance) {
    if !this.is_null() {
        allocator_deallocate(std::ptr::null_mut(), this as *mut c_void);
    }
}

unsafe extern "win64" fn scene_get_limits(
    this: *const PxSceneInstance,
    sret: *mut [u32; 9],
) -> *mut [u32; 9] {
    if !sret.is_null() {
        let lim = if this.is_null() {
            [0u32; 9]
        } else {
            (*this).limits
        };
        std::ptr::write_unaligned(sret, lim);
    }
    sret
}

unsafe extern "win64" fn scene_get_physics(_this: *const PxSceneInstance) -> *mut PxPhysicsInstance {
    get_or_init_physics()
}

unsafe extern "win64" fn scene_get_active_actors(
    _this: *mut PxSceneInstance,
    nb_actors_out: *mut u32,
    _client_id: u8,
) -> *const *mut c_void {
    if !nb_actors_out.is_null() {
        *nb_actors_out = 0;
    }
    std::ptr::null()
}

/// Optional per-game interceptor for `PxScene` completion tasks.
/// Returns `true` if the hook handled the task and `PxScene` should skip default dispatch.
pub type CompletionTaskHook = unsafe fn(task: usize, run_fn: usize) -> bool;

static COMPLETION_TASK_HOOK: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

/// Install a game-specific PhysX completion-task interceptor (none by default).
pub fn set_completion_task_hook(hook: CompletionTaskHook) {
    COMPLETION_TASK_HOOK.store(hook as usize, Ordering::Release);
}

unsafe extern "win64" fn scene_completion_task_complete(this: *mut PxSceneInstance, task: usize) {
    if task == 0 {
        return;
    }
    let vtbl = std::ptr::read_unaligned(task as *const usize);
    if vtbl != 0 {
        let run_fn = std::ptr::read_unaligned((vtbl + 0x08) as *const usize);
        let hook = COMPLETION_TASK_HOOK.load(Ordering::Acquire);
        if hook != 0 {
            let f: CompletionTaskHook = std::mem::transmute(hook);
            if f(task, run_fn) {
                return;
            }
        }
    }
    if !this.is_null() && (*this).cpu_dispatcher != 0 {
        let disp = (*this).cpu_dispatcher;
        let disp_vtbl = std::ptr::read_unaligned(disp as *const usize);
        if disp_vtbl != 0 {
            let submit_fn = std::ptr::read_unaligned(disp_vtbl as *const usize);
            if submit_fn != 0 {
                crate::call_guest8(submit_fn, disp as u64, task as u64, 0, 0, 0, 0, 0, 0);
                return;
            }
        }
    }
    if vtbl != 0 {
        let run_fn = std::ptr::read_unaligned((vtbl + 0x08) as *const usize);
        if run_fn != 0 {
            crate::call_guest8(run_fn, task as u64, 0, 0, 0, 0, 0, 0, 0);
        }
        let release_fn = std::ptr::read_unaligned((vtbl + 0x30) as *const usize);
        if release_fn != 0 {
            crate::call_guest8(release_fn, task as u64, 0, 0, 0, 0, 0, 0, 0);
        }
    }
}

unsafe extern "win64" fn scene_collide(
    this: *mut PxSceneInstance,
    _elapsed_time_slot: u64,
    completion_task: usize,
    _scratch_block: *mut c_void,
    _scratch_block_size: u32,
    _control_simulation: u8,
) {
    scene_completion_task_complete(this, completion_task);
}

unsafe extern "win64" fn scene_fetch_collision(
    _this: *mut PxSceneInstance,
    _block: u8,
) -> u8 {
    1
}

unsafe extern "win64" fn scene_advance(
    this: *mut PxSceneInstance,
    completion_task: usize,
) {
    scene_completion_task_complete(this, completion_task);
}

unsafe extern "win64" fn scene_check_results(
    _this: *mut PxSceneInstance,
    _block: u8,
) -> u8 {
    1
}

unsafe extern "win64" fn scene_fetch_results(
    _this: *mut PxSceneInstance,
    _block: u8,
    error_state: *mut u32,
) -> u8 {
    if !error_state.is_null() {
        *error_state = 0;
    }
    1
}

unsafe extern "win64" fn scene_get_gravity(
    this: *const PxSceneInstance,
    sret: *mut [f32; 3],
) -> *mut [f32; 3] {
    if !sret.is_null() {
        let g = if this.is_null() {
            [0.0, -9.81, 0.0]
        } else {
            (*this).gravity
        };
        std::ptr::write_unaligned(sret, g);
    }
    sret
}

unsafe extern "win64" fn scene_set_gravity(this: *mut PxSceneInstance, vec: *const [f32; 3]) {
    if !this.is_null() && !vec.is_null() {
        (*this).gravity = std::ptr::read_unaligned(vec);
    }
}

unsafe extern "win64" fn scene_get_pvd_client(
    this: *mut PxSceneInstance,
) -> *mut PxPvdClientBridge {
    if this.is_null() {
        std::ptr::null_mut()
    } else {
        &mut (*this).pvd_client
    }
}

static SCENE_VTBL: [Ptr; 128] = [
    Ptr(scene_release as *const ()),       // 0 (+0x000): release
    Ptr(ret0 as *const ()),                // 1 (+0x008): setFlag
    Ptr(ret0 as *const ()),                // 2 (+0x010): setFlags
    Ptr(sret_zero_u32 as *const ()),       // 3 (+0x018): getFlags (sret)
    Ptr(ret0 as *const ()),                // 4 (+0x020): setLimits
    Ptr(scene_get_limits as *const ()),    // 5 (+0x028): getLimits (sret 0x24B)
    Ptr(scene_get_physics as *const ()),   // 6 (+0x030): getPhysics
    Ptr(ret0 as *const ()),                // 7 (+0x038): getTimestamp
    Ptr(ret1 as *const ()),                // 8 (+0x040): addArticulation
    Ptr(ret0 as *const ()),                // 9 (+0x048): removeArticulation
    Ptr(ret1 as *const ()),                // 10 (+0x050): addActor
    Ptr(ret1 as *const ()),                // 11 (+0x058): addActors(pruningStructure)
    Ptr(ret1 as *const ()),                // 12 (+0x060): addActors(actors, nbActors)
    Ptr(ret0 as *const ()),                // 13 (+0x068): removeActor
    Ptr(ret0 as *const ()),                // 14 (+0x070): removeActors
    Ptr(ret1 as *const ()),                // 15 (+0x078): addAggregate
    Ptr(ret0 as *const ()),                // 16 (+0x080): removeAggregate
    Ptr(ret1 as *const ()),                // 17 (+0x088): addCollection
    Ptr(ret0 as *const ()),                // 18 (+0x090): getNbActors
    Ptr(ret0 as *const ()),                  // 19 (+0x098): getActors
    Ptr(scene_get_active_actors as *const ()),// 20 (+0x0a0): getActiveActors
    Ptr(scene_get_active_actors as *const ()),// 21 (+0x0a8): getActiveActors
    Ptr(ret0 as *const ()),                  // 22 (+0x0b0): getArticulations
    Ptr(ret0 as *const ()),                  // 23 (+0x0b8): getNbConstraints
    Ptr(ret0 as *const ()),                  // 24 (+0x0c0): getConstraints
    Ptr(ret0 as *const ()),                  // 25 (+0x0c8): getNbAggregates
    Ptr(ret0 as *const ()),                  // 26 (+0x0d0): getAggregates
    Ptr(ret0 as *const ()),                  // 27 (+0x0d8)
    Ptr(ret0 as *const ()),                  // 28 (+0x0e0)
    Ptr(ret0 as *const ()),                  // 29 (+0x0e8)
    Ptr(ret0 as *const ()),                  // 30 (+0x0f0)
    Ptr(ret0 as *const ()),                  // 31 (+0x0f8)
    Ptr(ret0 as *const ()),                  // 32 (+0x100): setDominanceGroupPair
    Ptr(ret0 as *const ()),                  // 33 (+0x108): getDominanceGroupPair
    Ptr(ret0 as *const ()),                  // 34 (+0x110): getCpuDispatcher
    Ptr(ret0 as *const ()),                  // 35 (+0x118): getGpuDispatcher
    Ptr(ret0 as *const ()),                  // 36 (+0x120)
    Ptr(ret0 as *const ()),                  // 37 (+0x128)
    Ptr(ret0 as *const ()),                  // 38 (+0x130)
    Ptr(ret0 as *const ()),                  // 39 (+0x138)
    Ptr(ret0 as *const ()),                  // 40 (+0x140)
    Ptr(ret0 as *const ()),                  // 41 (+0x148)
    Ptr(ret0 as *const ()),                  // 42 (+0x150)
    Ptr(ret0 as *const ()),                  // 43 (+0x158)
    Ptr(ret0 as *const ()),                  // 44 (+0x160)
    Ptr(ret0 as *const ()),                  // 45 (+0x168)
    Ptr(ret0 as *const ()),                  // 46 (+0x170)
    Ptr(ret0 as *const ()),                  // 47 (+0x178)
    Ptr(ret0 as *const ()),                  // 48 (+0x180)
    Ptr(ret0 as *const ()),                  // 49 (+0x188)
    Ptr(ret0 as *const ()),                  // 50 (+0x190)
    Ptr(ret0 as *const ()),                  // 51 (+0x198)
    Ptr(ret0 as *const ()),                  // 52 (+0x1a0)
    Ptr(ret0 as *const ()),                  // 53 (+0x1a8)
    Ptr(ret0 as *const ()),                  // 54 (+0x1b0)
    Ptr(ret0 as *const ()),                  // 55 (+0x1b8)
    Ptr(scene_collide as *const ()),         // 56 (+0x1c0): simulate(dt, completionTask, ...)
    Ptr(scene_advance as *const ()),         // 57 (+0x1c8): advance(completionTask)
    Ptr(scene_collide as *const ()),         // 58 (+0x1d0): collide(dt, completionTask, ...)
    Ptr(scene_check_results as *const ()),   // 59 (+0x1d8): checkResults(block)
    Ptr(scene_fetch_collision as *const ()), // 60 (+0x1e0): fetchCollision(block)
    Ptr(scene_fetch_results as *const ()),   // 61 (+0x1e8): fetchResults(block, errorState)
    Ptr(ret0 as *const ()),                  // 62 (+0x1f0): fetchResultsStart
    Ptr(scene_advance as *const ()),         // 63 (+0x1f8): processCallbacks(continuation)
    Ptr(ret0 as *const ()),                  // 64 (+0x200): fetchResultsFinish
    Ptr(ret0 as *const ()),                  // 65 (+0x208): flushSimulation
    Ptr(scene_set_gravity as *const ()),     // 66 (+0x210): setGravity
    Ptr(scene_get_gravity as *const ()),     // 67 (+0x218): getGravity (sret)
    Ptr(ret0 as *const ()),                  // 68 (+0x220)
    Ptr(ret0 as *const ()),                  // 69 (+0x228)
    Ptr(ret0 as *const ()),                  // 70 (+0x230)
    Ptr(ret0 as *const ()),                  // 71 (+0x238)
    Ptr(ret0 as *const ()),                  // 72 (+0x240)
    Ptr(ret0 as *const ()),                  // 73 (+0x248)
    Ptr(ret0 as *const ()),                  // 74 (+0x250)
    Ptr(ret0 as *const ()),                  // 75 (+0x258)
    Ptr(ret0 as *const ()),                  // 76 (+0x260)
    Ptr(ret0 as *const ()),                  // 77 (+0x268)
    Ptr(ret0 as *const ()),                  // 78 (+0x270)
    Ptr(ret0 as *const ()),                  // 79 (+0x278)
    Ptr(ret0 as *const ()),                  // 80 (+0x280): getSimulationStatistics
    Ptr(ret0 as *const ()),                  // 81 (+0x288)
    Ptr(ret0 as *const ()),                  // 82 (+0x290)
    Ptr(ret0 as *const ()),                  // 83 (+0x298)
    Ptr(ret0 as *const ()),                  // 84 (+0x2a0)
    Ptr(ret0 as *const ()),                  // 85 (+0x2a8)
    Ptr(ret0 as *const ()),                  // 86 (+0x2b0)
    Ptr(ret0 as *const ()),                  // 87 (+0x2b8)
    Ptr(ret0 as *const ()),                  // 88 (+0x2c0)
    Ptr(ret0 as *const ()),                  // 89 (+0x2c8)
    Ptr(ret0 as *const ()),                  // 90 (+0x2d0)
    Ptr(scene_advance as *const ()),         // 91 (+0x2d8): sceneQueriesUpdate(completionTask, controlSimulation)
    Ptr(scene_check_results as *const ()),   // 92 (+0x2e0): checkQueries(block)
    Ptr(scene_fetch_collision as *const ()), // 93 (+0x2e8): fetchQueries(block)
    Ptr(ret0 as *const ()),                // 94 (+0x2f0): raycast
    Ptr(ret0 as *const ()),                // 95 (+0x2f8): sweep
    Ptr(ret0 as *const ()),                // 96 (+0x300): overlap
    Ptr(ret0 as *const ()),                // 97 (+0x308)
    Ptr(ret0 as *const ()),                // 98 (+0x310)
    Ptr(ret0 as *const ()),                // 99 (+0x318)
    Ptr(ret0 as *const ()),                // 100 (+0x320)
    Ptr(ret0 as *const ()),                // 101 (+0x328)
    Ptr(ret0 as *const ()),                // 102 (+0x330)
    Ptr(ret0 as *const ()),                // 103 (+0x338)
    Ptr(scene_get_pvd_client as *const ()),// 104 (+0x340): getScenePvdClient
    Ptr(ret0 as *const ()),                // 105 (+0x348)
    Ptr(ret0 as *const ()),                // 106 (+0x350)
    Ptr(ret0 as *const ()),                // 107 (+0x358)
    Ptr(ret0 as *const ()),                // 108 (+0x360)
    Ptr(ret0 as *const ()),                // 109 (+0x368)
    Ptr(ret0 as *const ()),                // 110 (+0x370)
    Ptr(ret0 as *const ()),                // 111 (+0x378)
    Ptr(ret0 as *const ()),                // 112 (+0x380)
    Ptr(ret0 as *const ()),                // 113 (+0x388)
    Ptr(ret0 as *const ()),                // 114 (+0x390)
    Ptr(ret0 as *const ()),                // 115 (+0x398)
    Ptr(ret0 as *const ()),                // 116 (+0x3a0)
    Ptr(ret0 as *const ()),                // 117 (+0x3a8)
    Ptr(ret0 as *const ()),                // 118 (+0x3b0)
    Ptr(ret0 as *const ()),                // 119 (+0x3b8)
    Ptr(scene_get_pvd_client as *const ()),// 120 (+0x3c0)
    Ptr(ret0 as *const ()),                // 121 (+0x3c8)
    Ptr(ret0 as *const ()),                // 122 (+0x3d0)
    Ptr(ret0 as *const ()),                // 123 (+0x3d8)
    Ptr(ret0 as *const ()),                // 124 (+0x3e0)
    Ptr(ret0 as *const ()),                // 125 (+0x3e8)
    Ptr(ret0 as *const ()),                // 126 (+0x3f0)
    Ptr(ret0 as *const ()),                // 127 (+0x3f8)
];

// ---------------------------------------------------------------------------
// PxPhysics vtable (41 slots, matches 0x1801ae2b8)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn physics_dtor(
    this: *mut PxPhysicsInstance,
    _flags: u32,
) -> *mut PxPhysicsInstance {
    this
}

unsafe extern "win64" fn physics_release(_this: *mut PxPhysicsInstance) {}

unsafe extern "win64" fn physics_get_foundation(_this: *mut PxPhysicsInstance) -> *mut c_void {
    PxGetFoundation() as *mut c_void
}

unsafe extern "win64" fn physics_create_aggregate(
    _this: *mut PxPhysicsInstance,
    max_size: u32,
    self_collision: u8,
) -> *mut c_void {
    let ptr = alloc_generic_base_obj(14, 0x50) as *mut u8;
    if !ptr.is_null() {
        std::ptr::write_unaligned(ptr.add(0x34) as *mut u32, max_size);
        *ptr.add(0x38) = self_collision;
        std::ptr::write_unaligned(ptr.add(0x40) as *mut u32, 0);
        if max_size != 0 {
            let arr = allocator_allocate(
                std::ptr::null_mut(),
                (max_size as usize) * std::mem::size_of::<*mut c_void>(),
                b"NpAggregate\0".as_ptr(),
                b"physx3_x64.rs\0".as_ptr(),
                1,
            );
            std::ptr::write_unaligned(ptr.add(0x48) as *mut *mut c_void, arr);
        }
    }
    ptr as *mut c_void
}

unsafe extern "win64" fn physics_get_tolerances_scale(
    this: *const PxPhysicsInstance,
) -> *const [f32; 2] {
    let p = if this.is_null() {
        get_or_init_physics()
    } else {
        this
    };
    &(*p).tolerances_scale
}

unsafe extern "win64" fn physics_create_triangle_mesh(
    _this: *mut PxPhysicsInstance,
    _stream: *mut c_void,
) -> *mut c_void {
    alloc_generic_base_obj(4, 0x118)
}

unsafe extern "win64" fn physics_create_height_field(
    _this: *mut PxPhysicsInstance,
    _stream: *mut c_void,
) -> *mut c_void {
    alloc_generic_base_obj(1, 0x88)
}

unsafe extern "win64" fn physics_create_convex_mesh(
    _this: *mut PxPhysicsInstance,
    _stream: *mut c_void,
) -> *mut c_void {
    alloc_generic_base_obj(2, 0xa8)
}

unsafe extern "win64" fn physics_create_cloth_fabric(
    _this: *mut PxPhysicsInstance,
    _arg: *const c_void,
) -> *mut c_void {
    alloc_generic_base_obj(5, 0x40)
}

unsafe extern "win64" fn physics_create_scene(
    _this: *mut PxPhysicsInstance,
    desc: *const c_void,
) -> *mut PxSceneInstance {
    ensure_vtables_registered();
    let ptr = allocator_allocate(
        std::ptr::null_mut(),
        std::mem::size_of::<PxSceneInstance>(),
        b"NpScene\0".as_ptr(),
        b"physx3_x64.rs\0".as_ptr(),
        1,
    ) as *mut PxSceneInstance;
    if !ptr.is_null() {
        (*ptr).vtbl = &SCENE_VTBL;
        (*ptr).gravity = [0.0, -9.81, 0.0];
        (*ptr).pvd_client.vtbl = &PVD_CLIENT_VTBL;
        if !desc.is_null() {
            (*ptr).cpu_dispatcher =
                std::ptr::read_unaligned((desc as *const u8).add(0xa0) as *const usize);
        }
    }
    ptr
}

unsafe extern "win64" fn physics_create_rigid_static(
    _this: *mut PxPhysicsInstance,
    pose: *const PxTransform,
) -> *mut PxRigidActorInstance {
    ensure_vtables_registered();
    let ptr = allocator_allocate(
        std::ptr::null_mut(),
        0xb0,
        b"NpRigidStatic\0".as_ptr(),
        b"physx3_x64.rs\0".as_ptr(),
        1,
    ) as *mut PxRigidActorInstance;
    if !ptr.is_null() {
        (*ptr).vtbl = &RIGID_ACTOR_VTBL;
        (*ptr).concrete_type = 7;
        (*ptr).base_flags = 3;
        let p = if pose.is_null() {
            PxTransform {
                q: [0.0, 0.0, 0.0, 1.0],
                p: [0.0, 0.0, 0.0],
            }
        } else {
            std::ptr::read_unaligned(pose)
        };
        std::ptr::write_unaligned((ptr as *mut u8).add(0x90) as *mut PxTransform, p);
    }
    ptr
}

unsafe extern "win64" fn physics_create_rigid_dynamic(
    _this: *mut PxPhysicsInstance,
    pose: *const PxTransform,
) -> *mut PxRigidActorInstance {
    ensure_vtables_registered();
    let ptr = allocator_allocate(
        std::ptr::null_mut(),
        std::mem::size_of::<PxRigidActorInstance>(),
        b"NpRigidDynamic\0".as_ptr(),
        b"physx3_x64.rs\0".as_ptr(),
        1,
    ) as *mut PxRigidActorInstance;
    if !ptr.is_null() {
        (*ptr).vtbl = &RIGID_ACTOR_VTBL;
        (*ptr).concrete_type = 6;
        (*ptr).base_flags = 3;
        let p = if pose.is_null() {
            PxTransform {
                q: [0.0, 0.0, 0.0, 1.0],
                p: [0.0, 0.0, 0.0],
            }
        } else {
            std::ptr::read_unaligned(pose)
        };
        let raw = ptr as *mut u8;
        std::ptr::write_unaligned(raw.add(0xb0) as *mut PxTransform, p);
        std::ptr::write_unaligned(
            raw.add(0x150) as *mut PxTransform,
            PxTransform {
                q: [0.0, 0.0, 0.0, 1.0],
                p: [0.0, 0.0, 0.0],
            },
        );
    }
    ptr
}

unsafe extern "win64" fn physics_create_shape(
    _this: *mut PxPhysicsInstance,
    geom: *const u32,
    materials: *const *mut PxMaterialInstance,
    material_count: u16,
    is_exclusive: u8,
    shape_flags_ptr: *const u8,
) -> *mut PxShapeInstance {
    ensure_vtables_registered();
    let ptr = allocator_allocate(
        std::ptr::null_mut(),
        std::mem::size_of::<PxShapeInstance>(),
        b"NpShape\0".as_ptr(),
        b"physx3_x64.rs\0".as_ptr(),
        1,
    ) as *mut PxShapeInstance;
    if !ptr.is_null() {
        (*ptr).vtbl = &SHAPE_VTBL;
        (*ptr).concrete_type = 8;
        (*ptr).base_flags = 3;
        (*ptr).ref_count = 1;
        if !materials.is_null() && material_count > 0 {
            let mat = std::ptr::read_unaligned(materials);
            (*ptr)._pad3 = mat as u64;
            if !mat.is_null() {
                let rc_ptr = (&raw const (*mat).ref_count) as *const std::sync::atomic::AtomicU32;
                (*rc_ptr).fetch_add(1, Ordering::AcqRel);
            }
        }
        (*ptr).local_pose = PxTransform {
            q: [0.0, 0.0, 0.0, 1.0],
            p: [0.0, 0.0, 0.0],
        };
        (*ptr).contact_offset = 0.02;
        (*ptr).shape_flags = if shape_flags_ptr.is_null() {
            0x0b
        } else {
            *shape_flags_ptr
        };
        if !geom.is_null() {
            let gtype = std::ptr::read_unaligned(geom);
            (*ptr).geom_type = gtype;
            let copy_bytes = match gtype {
                0 => 0x08usize, // Sphere
                1 => 0x04,      // Plane
                2 => 0x0c,      // Capsule
                3 => 0x10,      // Box
                4 => 0x30,      // ConvexMesh
                5 => 0x30,      // TriangleMesh
                6 => 0x20,      // HeightField
                _ => 0x04,
            };
            std::ptr::copy_nonoverlapping(
                geom as *const u8,
                &mut (*ptr).geom_type as *mut u32 as *mut u8,
                copy_bytes,
            );
        }
        (*ptr).exclusive_flag = if is_exclusive != 0 { 0x8000_0000 } else { 0 };
    }
    ptr
}

unsafe extern "win64" fn physics_create_constraint(
    _this: *mut PxPhysicsInstance,
    actor0: *mut c_void,
    actor1: *mut c_void,
    connector: *mut c_void,
    shaders: *const usize,
    _data_size: u32,
) -> *mut c_void {
    let ptr = alloc_generic_base_obj(10, 0xb8) as *mut u8;
    if !ptr.is_null() {
        *(ptr.add(0x0a) as *mut u16) = 1; // eOWNS_MEMORY (subordinate, not eIS_RELEASABLE)
        std::ptr::write_unaligned(ptr.add(0x10) as *mut *mut c_void, actor0);
        std::ptr::write_unaligned(ptr.add(0x18) as *mut *mut c_void, actor1);
        constraint_set_functions(ptr, connector, shaders);
    }
    ptr as *mut c_void
}

unsafe extern "win64" fn physics_create_articulation(_this: *mut PxPhysicsInstance) -> *mut c_void {
    alloc_generic_base_obj(15, 0xa8)
}

unsafe extern "win64" fn physics_create_material(
    _this: *mut PxPhysicsInstance,
    static_friction: f32,
    dynamic_friction: f32,
    restitution: f32,
) -> *mut PxMaterialInstance {
    ensure_vtables_registered();
    let ptr = allocator_allocate(
        std::ptr::null_mut(),
        std::mem::size_of::<PxMaterialInstance>(),
        b"NpMaterial\0".as_ptr(),
        b"physx3_x64.rs\0".as_ptr(),
        1,
    ) as *mut PxMaterialInstance;
    if !ptr.is_null() {
        (*ptr).vtbl = &MATERIAL_VTBL;
        (*ptr).concrete_type = 9;
        (*ptr).base_flags = 3;
        (*ptr).ref_count = 1;
        (*ptr).dynamic_friction = dynamic_friction;
        (*ptr).static_friction = static_friction;
        (*ptr).restitution = restitution;
    }
    ptr
}
crate::win64_bridge!(fn physics_create_material(*mut PxPhysicsInstance, f32, f32, f32) -> *mut PxMaterialInstance);

unsafe extern "win64" fn physics_get_insertion_callback(
    this: *mut PxPhysicsInstance,
) -> *mut PxInsertionCallbackBridge {
    let p = if this.is_null() {
        get_or_init_physics()
    } else {
        this
    };
    &mut (*p).insertion_cb
}

static PHYSICS_VTBL: [Ptr; 41] = [
    Ptr(physics_dtor as *const ()),                   // 0 (+0x000): ~PxPhysics
    Ptr(physics_release as *const ()),                // 1 (+0x008): release
    Ptr(physics_get_foundation as *const ()),         // 2 (+0x010): getFoundation
    Ptr(physics_create_aggregate as *const ()),       // 3 (+0x018): createAggregate
    Ptr(physics_get_tolerances_scale as *const ()),   // 4 (+0x020): getTolerancesScale
    Ptr(physics_create_triangle_mesh as *const ()),   // 5 (+0x028): createTriangleMesh
    Ptr(ret0 as *const ()),                           // 6 (+0x030): getNbTriangleMeshes
    Ptr(ret0 as *const ()),                           // 7 (+0x038): getTriangleMeshes
    Ptr(physics_create_height_field as *const ()),    // 8 (+0x040): createHeightField
    Ptr(ret0 as *const ()),                           // 9 (+0x048): getNbHeightFields
    Ptr(ret0 as *const ()),                           // 10 (+0x050): getHeightFields
    Ptr(physics_create_convex_mesh as *const ()),     // 11 (+0x058): createConvexMesh
    Ptr(ret0 as *const ()),                           // 12 (+0x060): getNbConvexMeshes
    Ptr(ret0 as *const ()),                           // 13 (+0x068): getConvexMeshes
    Ptr(physics_create_cloth_fabric as *const ()),    // 14 (+0x070): createClothFabric(desc)
    Ptr(physics_create_cloth_fabric as *const ()),    // 15 (+0x078): createClothFabric(stream)
    Ptr(ret0 as *const ()),                           // 16 (+0x080): getNbClothFabrics
    Ptr(ret0 as *const ()),                           // 17 (+0x088): getClothFabrics
    Ptr(physics_create_scene as *const ()),           // 18 (+0x090): createScene
    Ptr(ret0 as *const ()),                           // 19 (+0x098): getNbScenes
    Ptr(ret0 as *const ()),                           // 20 (+0x0a0): getScenes
    Ptr(physics_create_rigid_static as *const ()),    // 21 (+0x0a8): createRigidStatic
    Ptr(physics_create_rigid_dynamic as *const ()),   // 22 (+0x0b0): createRigidDynamic
    Ptr(ret0 as *const ()),                           // 23 (+0x0b8): createParticleSystem
    Ptr(ret0 as *const ()),                           // 24 (+0x0c0): createParticleFluid
    Ptr(ret0 as *const ()),                           // 25 (+0x0c8): createCloth
    Ptr(ret0 as *const ()),                           // 26 (+0x0d0): createPruningStructure
    Ptr(physics_create_shape as *const ()),           // 27 (+0x0d8): createShape
    Ptr(ret0 as *const ()),                           // 28 (+0x0e0): getNbShapes
    Ptr(ret0 as *const ()),                           // 29 (+0x0e8): getShapes
    Ptr(physics_create_constraint as *const ()),      // 30 (+0x0f0): createConstraint
    Ptr(physics_create_articulation as *const ()),    // 31 (+0x0f8): createArticulation
    Ptr(crate::bridge_entry!(physics_create_material)), // 32 (+0x100): createMaterial
    Ptr(ret0 as *const ()),                           // 33 (+0x108): getNbMaterials
    Ptr(ret0 as *const ()),                           // 34 (+0x110): getMaterials
    Ptr(ret0 as *const ()),                           // 35 (+0x118): registerDeletionListener
    Ptr(ret0 as *const ()),                           // 36 (+0x120): unregisterDeletionListener
    Ptr(ret0 as *const ()),                           // 37 (+0x128): registerDeletionListenerObjects
    Ptr(ret0 as *const ()),                           // 38 (+0x130): unregisterDeletionListenerObjects
    Ptr(physics_get_insertion_callback as *const ()), // 39 (+0x138): getPhysicsInsertionCallback
    Ptr(ret0 as *const ()),                           // 40 (+0x140): getPvd
];

// ---------------------------------------------------------------------------
// PxSerializerDefaultAdapter vtable & instances for PxRegisterPhysicsSerializers
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct PxSerializerAdapter {
    pub vtbl: *const [Ptr; 9],  // +0x00
    pub type_name: *const u8,   // +0x08
    pub class_size: usize,      // +0x10
    pub concrete_type: u16,     // +0x18
}

unsafe extern "win64" fn serializer_get_type_name(this: *const PxSerializerAdapter) -> *const u8 {
    if this.is_null() {
        b"PxBase\0".as_ptr()
    } else {
        (*this).type_name
    }
}

/// `PX_SERIAL_REF_KIND_PXBASE`.
const SERIAL_REF_KIND_PXBASE: u32 = 0x8000_0000;

/// Visit the `PxBase` objects that `obj` references and that must travel with
/// it through binary serialization (`T::requiresObjects` in PhysX 3.4):
/// - `NpShape`: its mesh (convex `+0xb8`, triangle `+0xc0`, height field `+0xa0`)
/// - `NpRigidStatic/Dynamic`: attached shapes (`PtrTable` at `+0x28`/`+0x30`)
/// - `NpConstraint`: both actors (`+0x10`/`+0x18`, which `createObject` resolves)
/// - `NpAggregate` / `Sq::PruningStructure`: actor list (`+0x48`, count `+0x40`)
/// Materials are deliberately not visited: the importer keeps shape material
/// indices as-is (no `PX_SERIAL_REF_KIND_MATERIAL_IDX` resolve), and the
/// builtin's material slot is not valid on shapes loaded from disk.
unsafe fn for_each_required_object(ctype: u16, obj: *const u8, mut f: impl FnMut(*mut u8)) {
    if obj.is_null() {
        return;
    }
    let rd = |off: usize| std::ptr::read_unaligned(obj.add(off) as *const usize) as *mut u8;
    match ctype {
        8 => {
            let gtype = std::ptr::read_unaligned(obj.add(0x98) as *const u32);
            let mesh = match gtype {
                4 => rd(0xb8),
                5 => rd(0xc0),
                6 => rd(0xa0),
                _ => std::ptr::null_mut(),
            };
            if !mesh.is_null() {
                f(mesh);
            }
        }
        6 | 7 => {
            let nb = std::ptr::read_unaligned(obj.add(0x30) as *const u16) as usize;
            if nb == 1 {
                let s = rd(0x28);
                if !s.is_null() {
                    f(s);
                }
            } else if nb > 1 {
                let list = rd(0x28) as *const usize;
                if !list.is_null() {
                    for i in 0..nb {
                        let s = std::ptr::read_unaligned(list.add(i)) as *mut u8;
                        if !s.is_null() {
                            f(s);
                        }
                    }
                }
            }
        }
        10 => {
            for off in [0x10usize, 0x18] {
                let a = rd(off);
                if !a.is_null() {
                    f(a);
                }
            }
        }
        14 | 18 => {
            if ctype == 18 && *obj.add(0x50) == 0 {
                return;
            }
            let n = std::ptr::read_unaligned(obj.add(0x40) as *const u32) as usize;
            let list = rd(0x48) as *const usize;
            if !list.is_null() {
                for i in 0..n {
                    let a = std::ptr::read_unaligned(list.add(i)) as *mut u8;
                    if !a.is_null() {
                        f(a);
                    }
                }
            }
        }
        _ => {}
    }
}

/// `PxSerializer::requiresObjects(PxBase&, PxProcessPxBaseCallback&)`.
/// `PxSerialization::complete` relies on this to pull meshes/shapes/actors
/// into a collection before `serializeCollectionToBinary`.
unsafe extern "win64" fn serializer_requires_objects(
    this: *const PxSerializerAdapter,
    obj: *mut c_void,
    cb: *mut c_void,
) {
    if this.is_null() || obj.is_null() || cb.is_null() {
        return;
    }
    let vtbl = std::ptr::read_unaligned(cb as *const *const usize);
    if vtbl.is_null() || *vtbl.add(1) == 0 {
        return;
    }
    // slot 0 is `~PxProcessPxBaseCallback` (scalar deleting dtor); slot 1 is
    // `process(this, PxBase&)` (e.g. `0x1417f9cd0` for `PxSerialization::complete`).
    let process = *vtbl.add(1);
    for_each_required_object((*this).concrete_type, obj as *const u8, |base| {
        static LOGGED: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        if LOGGED.fetch_add(1, Ordering::Relaxed) < 16 {
            log::info!(
                "physx3: requiresObjects type={} obj={:p} -> {:p} (type {})",
                (*this).concrete_type,
                obj,
                base,
                std::ptr::read_unaligned(base.add(8) as *const u16)
            );
        }
        crate::call_guest8(process, cb as u64, base as u64, 0, 0, 0, 0, 0, 0);
    });
}

/// `PxSerializerDefaultAdapter<T>::registerReferences`: register the object
/// itself plus everything from `requiresObjects` as `PX_SERIAL_REF_KIND_PXBASE`
/// references keyed by address, so `DeserializationContext::resolveReference`
/// can translate the raw pointers written by `exportData`.
unsafe extern "win64" fn serializer_register_references(
    this: *const PxSerializerAdapter,
    obj: *mut c_void,
    ctx: *mut c_void,
) {
    if this.is_null() || obj.is_null() || ctx.is_null() {
        return;
    }
    let vtbl = std::ptr::read_unaligned(ctx as *const *const usize);
    if vtbl.is_null() || *vtbl == 0 {
        return;
    }
    let register = *vtbl; // slot 0: registerReference(this, PxBase&, kind, reference)
    static LOGGED: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    if LOGGED.fetch_add(1, Ordering::Relaxed) < 16 {
        log::info!(
            "physx3: registerReferences type={} obj={:p}",
            (*this).concrete_type,
            obj
        );
    }
    let reg = |base: *mut u8| {
        crate::call_guest8(
            register,
            ctx as u64,
            base as u64,
            SERIAL_REF_KIND_PXBASE as u64,
            base as u64,
            0, 0, 0, 0,
        );
    };
    reg(obj as *mut u8);
    for_each_required_object((*this).concrete_type, obj as *const u8, reg);
}

unsafe extern "win64" fn serializer_is_subordinate(this: *const PxSerializerAdapter) -> u8 {
    if this.is_null() {
        return 0;
    }
    matches!((*this).concrete_type, 10 | 16 | 17) as u8
}

// --- PxSerializationContext helpers (vtable: 0 registerReference, 1 getCollection,
// 2 writeData, 3 alignData, 4 writeName) ---

unsafe fn sctx_align(ctx: *mut *const usize, align: u64) {
    let f = *(*ctx).add(3);
    if f != 0 {
        crate::call_guest8(f, ctx as u64, align, 0, 0, 0, 0, 0, 0);
    }
}

/// `writeData`; a null source with a non-zero size writes zeros so the extra
/// data stream stays in lock-step with what `serializer_create_object` consumes.
unsafe fn sctx_write(ctx: *mut *const usize, data: *const u8, size: usize) {
    if size == 0 {
        return;
    }
    let f = *(*ctx).add(2);
    if f == 0 {
        return;
    }
    if data.is_null() {
        let zeros = vec![0u8; size];
        crate::call_guest8(f, ctx as u64, zeros.as_ptr() as u64, size as u64, 0, 0, 0, 0, 0);
    } else {
        crate::call_guest8(f, ctx as u64, data as u64, size as u64, 0, 0, 0, 0, 0);
    }
}

#[inline]
unsafe fn rd_ptr(obj: *const u8, off: usize) -> *const u8 {
    std::ptr::read_unaligned(obj.add(off) as *const usize) as *const u8
}

/// Byte size of the `Gu::ConvexHullData` polygon/vertex/edge buffer at `+0x48`
/// (`computeBufferSize`), shared by import and export.
unsafe fn convex_hull_bytes(obj: *const u8) -> usize {
    let nb_edges_raw = std::ptr::read_unaligned(obj.add(0x44) as *const i16);
    let nb_verts = *obj.add(0x46) as u32;
    let nb_polys = *obj.add(0x47) as u32;
    let edges_abs = (nb_edges_raw as u16 & 0x7fff) as u32;
    let mut hull_bytes = nb_verts * 15 + (edges_abs + nb_polys * 10) * 2;
    if nb_edges_raw < 0 {
        hull_bytes += edges_abs * 4;
    }
    hull_bytes += std::ptr::read_unaligned(obj.add(0x68) as *const u32) & 0x7fff_ffff;
    if (hull_bytes & 3) != 0 {
        hull_bytes += 4 - (hull_bytes & 3);
    }
    hull_bytes as usize
}

/// `Gu::TriangleMesh::exportExtraData`; mirrors `trimesh_import_extra_data`.
unsafe fn trimesh_export_extra_data(obj: *const u8, ctx: *mut *const usize) {
    let nb_verts = std::ptr::read_unaligned(obj.add(0x20) as *const u32) as usize;
    let nb_tris = std::ptr::read_unaligned(obj.add(0x24) as *const u32) as usize;
    let flags = *obj.add(0x5c);
    let tri_bytes = if (flags & 2) != 0 { nb_tris * 6 } else { nb_tris * 12 };
    for (off, bytes) in [
        (0x28usize, nb_verts * 12),
        (0x30, tri_bytes),
        (0x50, nb_tris),
        (0x60, nb_tris * 2),
        (0x68, nb_tris * 4),
        (0x70, nb_tris * 12),
    ] {
        let p = rd_ptr(obj, off);
        if !p.is_null() {
            sctx_align(ctx, 16);
            sctx_write(ctx, p, bytes);
        }
    }
}

/// `PxSerializer::exportExtraData`. Every branch here must write exactly what
/// the matching branch of `serializer_create_object` consumes (same order,
/// sizes and alignment), because the game round-trips runtime-built objects
/// through `serializeCollectionToBinary` -> `createCollectionFromBinary`
/// (e.g. the convex shape cache built in `0x142153bc0`).
unsafe extern "win64" fn serializer_export_extra_data(
    this: *const PxSerializerAdapter,
    obj: *mut c_void,
    ctx: *mut *const usize,
) {
    if this.is_null() || obj.is_null() || ctx.is_null() || (*ctx).is_null() {
        return;
    }
    let ctype = (*this).concrete_type;
    let o = obj as *const u8;
    let write_name = |name: *const u8| {
        let f = *(*ctx).add(4); // vtable + 0x20: writeName(ctx, name)
        if f != 0 {
            crate::call_guest8(f, ctx as u64, name as u64, 0, 0, 0, 0, 0, 0);
        }
    };
    match ctype {
        2 => {
            // `Gu::ConvexMesh::exportExtraData`
            sctx_align(ctx, 16);
            sctx_write(ctx, rd_ptr(o, 0x48), convex_hull_bytes(o));
            let bcd = rd_ptr(o, 0x70);
            if !bcd.is_null() {
                // `BigConvexData` + `BigConvexData::exportExtraData`
                sctx_align(ctx, 16);
                sctx_write(ctx, bcd, 0x30);
                let samples = rd_ptr(bcd, 0x08);
                if !samples.is_null() {
                    let n = std::ptr::read_unaligned(bcd.add(0x02) as *const u16) as usize;
                    sctx_align(ctx, 16);
                    sctx_write(ctx, samples, n * 2);
                }
                let valencies = rd_ptr(bcd, 0x18);
                if !valencies.is_null() {
                    let total_v =
                        ((std::ptr::read_unaligned(bcd.add(0x10) as *const u32) + 3) & !3) as usize;
                    let adj_sz = std::ptr::read_unaligned(bcd.add(0x14) as *const u32) as usize;
                    sctx_align(ctx, 16);
                    sctx_write(ctx, valencies, total_v * 4);
                    sctx_write(ctx, rd_ptr(bcd, 0x20), adj_sz);
                }
            }
        }
        3 => {
            // `RTreeTriangleMesh::exportExtraData` (RTree pages, then TriangleMesh)
            let num_pages = std::ptr::read_unaligned(o.add(0xf0) as *const u32) as usize;
            sctx_align(ctx, 128);
            sctx_write(ctx, rd_ptr(o, 0xf8), num_pages * 0x70);
            trimesh_export_extra_data(o, ctx);
        }
        4 => {
            // `BV4TriangleMesh::exportExtraData` (BV4 nodes, then TriangleMesh)
            let nb_nodes = std::ptr::read_unaligned(o.add(0xe8) as *const u32) as usize;
            if nb_nodes != 0 {
                sctx_align(ctx, 16);
                sctx_write(ctx, rd_ptr(o, 0xf0), nb_nodes * 16);
            }
            trimesh_export_extra_data(o, ctx);
        }
        1 => {
            // `Gu::HeightField::exportExtraData`
            let rows = std::ptr::read_unaligned(o.add(0x38) as *const u32) as usize;
            let cols = std::ptr::read_unaligned(o.add(0x3c) as *const u32) as usize;
            sctx_align(ctx, 16);
            sctx_write(ctx, rd_ptr(o, 0x50), rows * cols * 4);
        }
        8 => {
            // `Sc::ShapeCore::exportExtraData` (multi-material index arrays) + name
            let gtype = std::ptr::read_unaligned(o.add(0x98) as *const u32);
            if gtype == 5 || gtype == 6 {
                let (num_mats, mats_ptr) = if gtype == 5 {
                    (std::ptr::read_unaligned(o.add(0xe0) as *const u16) as usize, rd_ptr(o, 0xd8))
                } else {
                    (std::ptr::read_unaligned(o.add(0xc8) as *const u16) as usize, rd_ptr(o, 0xc0))
                };
                sctx_align(ctx, 16);
                sctx_write(ctx, mats_ptr, num_mats * 2);
            }
            write_name(rd_ptr(o, 0x100));
        }
        6 | 7 => {
            // `NpShapeManager::exportExtraData` (shape + SQ PtrTables)
            for (list_off, n_off) in [(0x28usize, 0x30usize), (0x38, 0x40)] {
                let n = std::ptr::read_unaligned(o.add(n_off) as *const u16) as usize;
                if n > 1 {
                    sctx_align(ctx, 16);
                    sctx_write(ctx, rd_ptr(o, list_off), n * 8);
                }
            }
            // `NpActor::exportExtraData` (connector array)
            let conn = rd_ptr(o, 0x20);
            if !conn.is_null() {
                sctx_align(ctx, 16);
                sctx_write(ctx, conn, 0x58);
                let count = std::ptr::read_unaligned(conn.add(0x50) as *const u32) as usize;
                let entries = rd_ptr(conn, 0x48);
                if count != 0 && *conn.add(0x40) == 0 && !entries.is_null() {
                    let cap =
                        (std::ptr::read_unaligned(conn.add(0x54) as *const u32) & 0x7fff_ffff) as usize;
                    sctx_write(ctx, entries, cap * 16);
                }
            }
            write_name(rd_ptr(o, 0x18));
        }
        14 => {
            // `NpAggregate::exportExtraData`
            let actors_ptr = rd_ptr(o, 0x48);
            if !actors_ptr.is_null() {
                let nb_actors = std::ptr::read_unaligned(o.add(0x40) as *const u32) as usize;
                sctx_align(ctx, 16);
                sctx_write(ctx, actors_ptr, nb_actors * 8);
            }
        }
        18 => {
            // `Sq::PruningStructure::exportExtraData`
            if *o.add(0x50) != 0 {
                for (ptr_off, n_off, elem) in [
                    (0x18usize, 0x10usize, 0x1cusize),
                    (0x30, 0x28, 4),
                    (0x20, 0x14, 0x1c),
                    (0x38, 0x2c, 4),
                    (0x48, 0x40, 8),
                ] {
                    let p = rd_ptr(o, ptr_off);
                    if !p.is_null() {
                        let n = std::ptr::read_unaligned(o.add(n_off) as *const u32) as usize;
                        sctx_align(ctx, 16);
                        sctx_write(ctx, p, n * elem);
                    }
                }
            }
        }
        _ => {}
    }
}

unsafe extern "win64" fn serializer_export_data(
    this: *const PxSerializerAdapter,
    obj: *mut c_void,
    ctx: *mut *const usize,
) {
    if this.is_null() || obj.is_null() || ctx.is_null() || (*ctx).is_null() {
        return;
    }
    let write_data_fn = *(*ctx).add(2); // vtable + 0x10: writeData(ctx, data, size)
    if write_data_fn != 0 {
        crate::call_guest8(
            write_data_fn,
            ctx as u64,
            obj as u64,
            (*this).class_size as u64,
            0, 0, 0, 0, 0,
        );
    }
}

unsafe extern "win64" fn serializer_get_class_size(this: *const PxSerializerAdapter) -> usize {
    if this.is_null() {
        0x50
    } else {
        (*this).class_size
    }
}

#[inline]
unsafe fn ctx_align_extra(context: *mut u8, align: usize) -> usize {
    let extra_slot = context.add(0x08) as *mut usize;
    let aligned = (std::ptr::read_unaligned(extra_slot) + (align - 1)) & !(align - 1);
    std::ptr::write_unaligned(extra_slot, aligned);
    aligned
}

#[inline]
unsafe fn ctx_consume_extra(context: *mut u8, align: usize, bytes: usize) -> *mut u8 {
    let extra_slot = context.add(0x08) as *mut usize;
    let aligned = (std::ptr::read_unaligned(extra_slot) + (align - 1)) & !(align - 1);
    std::ptr::write_unaligned(extra_slot, aligned.wrapping_add(bytes));
    aligned as *mut u8
}

/// `Ps::hash(PxU32)` (Thomas Wang 32-bit), as inlined in
/// `Sn::DeserializationContext::resolveReference`.
#[inline]
fn px_hash_u32(k: u32) -> u32 {
    let mut c = (!(k << 15)).wrapping_add(k);
    c ^= c >> 10;
    c = c.wrapping_mul(9);
    c ^= c >> 6;
    c = (!(c << 11)).wrapping_add(c);
    c ^ (c >> 16)
}

/// `Ps::hash(PxU64)` (Thomas Wang 64-bit, truncated to 32 bits).
#[inline]
fn px_hash_u64(r: u64) -> u32 {
    let mut c = (!(r << 32)).wrapping_add(r);
    c ^= c >> 22;
    c = (!(c << 13)).wrapping_add(c);
    c ^= c >> 8;
    c = c.wrapping_mul(9);
    c ^= c >> 15;
    c = (!(c << 27)).wrapping_add(c);
    (c ^ (c >> 31)) as u32
}

/// Does the deserialization context's internal reference map contain
/// `(reference, kind)`? Mirrors the lookup in `resolveReference`
/// (`0x14181ffc0` in AlanWake2.exe, PhysX 3.4 `Sn::DeserializationContext`):
/// map = `[ctx+0x28]`, entries `+0x08` (24 B: `u64 ref`, `u32 kind`, `u32 value@+0x10`),
/// next `+0x10`, buckets `+0x18`, bucket count `+0x24`, size `+0x34`.
/// Returns `None` when the map header doesn't look like that layout.
unsafe fn ctx_ref_map_contains(context: *mut u8, kind: u32, id: usize) -> Option<bool> {
    let map = std::ptr::read_unaligned(context.add(0x28) as *const *const u8);
    if map.is_null() {
        return None;
    }
    let entries = std::ptr::read_unaligned(map.add(0x08) as *const *const u8);
    let next = std::ptr::read_unaligned(map.add(0x10) as *const *const u32);
    let buckets = std::ptr::read_unaligned(map.add(0x18) as *const *const u32);
    let nb_buckets = std::ptr::read_unaligned(map.add(0x24) as *const u32);
    let size = std::ptr::read_unaligned(map.add(0x34) as *const u32);
    if size == 0 {
        return Some(false);
    }
    if entries.is_null() || next.is_null() || buckets.is_null() || !nb_buckets.is_power_of_two() {
        return None;
    }
    let matches = |idx: u32| {
        let e = entries.add(idx as usize * 24);
        std::ptr::read_unaligned(e as *const u64) == id as u64
            && std::ptr::read_unaligned(e.add(8) as *const u32) == kind
    };
    let h = (px_hash_u64(id as u64) ^ 0xfc95_5b95).wrapping_mul(0xf4247) ^ px_hash_u32(kind);
    let mut idx = std::ptr::read_unaligned(buckets.add((h & (nb_buckets - 1)) as usize));
    let mut steps = 0u32;
    while idx != u32::MAX && steps <= size {
        if matches(idx) {
            static VERIFIED: Once = Once::new();
            VERIFIED.call_once(|| log::info!("physx3: serial reference map lookup verified (ref={id:#x})"));
            return Some(true);
        }
        idx = std::ptr::read_unaligned(next.add(idx as usize));
        steps += 1;
    }
    // Hash miss: confirm with a linear scan so a layout/hash mismatch can never
    // turn a valid reference into null.
    for i in 0..size {
        if matches(i) {
            static DISAGREE: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            if DISAGREE.fetch_add(1, Ordering::Relaxed) < 4 {
                log::warn!("physx3: resolveReference hash port disagrees for ref={id:#x} kind={kind:#x}");
            }
            return Some(true);
        }
    }
    Some(false)
}

#[inline]
unsafe fn ctx_resolve_ref(context: *mut u8, kind: u32, id: usize) -> *mut u8 {
    if context.is_null() || id == 0 {
        return std::ptr::null_mut();
    }
    let vtbl = std::ptr::read_unaligned(context as *const *const usize);
    if vtbl.is_null() {
        return std::ptr::null_mut();
    }
    let resolve_fn = *vtbl; // slot 0 (+0x00): resolveReference(this, kind: u32, index: usize)
    if resolve_fn == 0 {
        return std::ptr::null_mut();
    }
    // The original `resolveReference` dereferences the map entry without a
    // null check, so a missing reference is a guaranteed access violation.
    if ctx_ref_map_contains(context, kind, id) == Some(false) {
        static MISSES: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        if MISSES.fetch_add(1, Ordering::Relaxed) < 32 {
            log::warn!("physx3: unresolved serial reference ref={id:#x} kind={kind:#x}; using null");
        }
        return std::ptr::null_mut();
    }
    crate::call_guest8(
        resolve_fn,
        context as u64,
        kind as u64,
        id as u64,
        0, 0, 0, 0, 0,
    ) as *mut u8
}

/// `Gu::TriangleMesh::importExtraData` (`0x1800b44a0` in `PhysX3Common_x64.dll`).
unsafe fn trimesh_import_extra_data(obj: *mut u8, context: *mut u8) {
    let nb_verts = std::ptr::read_unaligned(obj.add(0x20) as *const u32) as usize;
    let nb_tris = std::ptr::read_unaligned(obj.add(0x24) as *const u32) as usize;
    if std::ptr::read_unaligned(obj.add(0x28) as *const usize) != 0 {
        let p = ctx_consume_extra(context, 16, nb_verts * 12);
        std::ptr::write_unaligned(obj.add(0x28) as *mut *mut u8, p);
    }
    if std::ptr::read_unaligned(obj.add(0x30) as *const usize) != 0 {
        let flags = *obj.add(0x5c);
        let tri_bytes = if (flags & 2) != 0 { nb_tris * 6 } else { nb_tris * 12 };
        let p = ctx_consume_extra(context, 16, tri_bytes);
        std::ptr::write_unaligned(obj.add(0x30) as *mut *mut u8, p);
    }
    if std::ptr::read_unaligned(obj.add(0x50) as *const usize) != 0 {
        let p = ctx_consume_extra(context, 16, nb_tris);
        std::ptr::write_unaligned(obj.add(0x50) as *mut *mut u8, p);
    }
    if std::ptr::read_unaligned(obj.add(0x60) as *const usize) != 0 {
        let p = ctx_consume_extra(context, 16, nb_tris * 2);
        std::ptr::write_unaligned(obj.add(0x60) as *mut *mut u8, p);
    }
    if std::ptr::read_unaligned(obj.add(0x68) as *const usize) != 0 {
        let p = ctx_consume_extra(context, 16, nb_tris * 4);
        std::ptr::write_unaligned(obj.add(0x68) as *mut *mut u8, p);
    }
    if std::ptr::read_unaligned(obj.add(0x70) as *const usize) != 0 {
        let p = ctx_consume_extra(context, 16, nb_tris * 12);
        std::ptr::write_unaligned(obj.add(0x70) as *mut *mut u8, p);
    }
    std::ptr::write_bytes(obj.add(0x80), 0, 0x20);
}

unsafe extern "win64" fn serializer_create_object(
    this: *const PxSerializerAdapter,
    address: *mut *mut u8,
    context: *mut c_void,
) -> *mut u8 {
    if address.is_null() || (*address).is_null() {
        return std::ptr::null_mut();
    }
    let obj = *address;
    let (sz, ctype) = if this.is_null() {
        (0x50usize, 0u16)
    } else {
        ((*this).class_size, (*this).concrete_type)
    };
    let vtbl_ptr: *const Ptr = match ctype {
        9 => MATERIAL_VTBL.as_ptr(),
        8 => SHAPE_VTBL.as_ptr(),
        6 | 7 => RIGID_ACTOR_VTBL.as_ptr(),
        2 => CONVEX_MESH_VTBL.as_ptr(),
        3 | 4 => TRIANGLE_MESH_VTBL.as_ptr(),
        10 => CONSTRAINT_VTBL.as_ptr(),
        14 => AGGREGATE_VTBL.as_ptr(),
        _ => GENERIC_BASE_VTBL.as_ptr(),
    };
    std::ptr::write_unaligned(obj as *mut *const Ptr, vtbl_ptr);
    let base_flags: u16 = if matches!(ctype, 10 | 16 | 17) { 0 } else { 2 };
    std::ptr::write_unaligned(obj.add(0x0a) as *mut u16, base_flags);
    *address = obj.add(sz);

    let ctx = context as *mut u8;
    if !ctx.is_null() {
        match ctype {
            2 => {
                // `Gu::ConvexMesh::createObject` (`0x18003a2a0` & `0x18003a0f0`)
                std::ptr::write_unaligned(obj.add(0x18) as *mut u32, 1);
                let hull_bytes = convex_hull_bytes(obj);
                let nb_adj = std::ptr::read_unaligned(obj.add(0x68) as *const u32) & 0x7fff_ffff;
                std::ptr::write_unaligned(obj.add(0x68) as *mut u32, nb_adj | 0x8000_0000);
                let hull_ptr = ctx_consume_extra(ctx, 16, hull_bytes);
                std::ptr::write_unaligned(obj.add(0x48) as *mut *mut u8, hull_ptr);
                if std::ptr::read_unaligned(obj.add(0x70) as *const usize) != 0 {
                    let bcd = ctx_consume_extra(ctx, 16, 0x30);
                    std::ptr::write_unaligned(obj.add(0x70) as *mut *mut u8, bcd);
                    if std::ptr::read_unaligned(bcd.add(0x08) as *const usize) != 0 {
                        let subdiv = std::ptr::read_unaligned(bcd.add(0x02) as *const u16) as usize;
                        let samples = ctx_consume_extra(ctx, 16, subdiv * 2);
                        std::ptr::write_unaligned(bcd.add(0x08) as *mut *mut u8, samples);
                    }
                    if std::ptr::read_unaligned(bcd.add(0x18) as *const usize) != 0 {
                        let total_v = ((std::ptr::read_unaligned(bcd.add(0x10) as *const u32) + 3) & !3) as usize;
                        let v_ptr = ctx_consume_extra(ctx, 16, total_v * 4);
                        std::ptr::write_unaligned(bcd.add(0x18) as *mut *mut u8, v_ptr);
                        let adj_sz = std::ptr::read_unaligned(bcd.add(0x14) as *const u32) as usize;
                        let a_ptr = ctx_consume_extra(ctx, 1, adj_sz);
                        std::ptr::write_unaligned(bcd.add(0x20) as *mut *mut u8, a_ptr);
                    }
                    std::ptr::write_unaligned(obj.add(0x50) as *mut *mut u8, bcd);
                }
            }
            3 => {
                // `Gu::RTreeTriangleMesh::createObject` (`0x1800b5110`)
                std::ptr::write_unaligned(obj.add(0x18) as *mut u32, 1);
                let num_pages = std::ptr::read_unaligned(obj.add(0xf0) as *const u32) as usize;
                let pages_ptr = ctx_consume_extra(ctx, 128, num_pages * 0x70);
                std::ptr::write_unaligned(obj.add(0xf8) as *mut *mut u8, pages_ptr);
                trimesh_import_extra_data(obj, ctx);
            }
            4 => {
                // `Gu::BV4TriangleMesh::createObject` (`0x1800b4df0`)
                std::ptr::write_unaligned(obj.add(0x18) as *mut u32, 1);
                *obj.add(0x114) = 1;
                let nb_nodes = std::ptr::read_unaligned(obj.add(0xe8) as *const u32) as usize;
                if nb_nodes != 0 {
                    let nodes_ptr = ctx_consume_extra(ctx, 16, nb_nodes * 16);
                    std::ptr::write_unaligned(obj.add(0xf0) as *mut *mut u8, nodes_ptr);
                }
                trimesh_import_extra_data(obj, ctx);
                let flags = *obj.add(0x5c);
                let verts = std::ptr::read_unaligned(obj.add(0x28) as *const usize);
                let tris = std::ptr::read_unaligned(obj.add(0x30) as *const usize);
                let (tris32, tris16) = if (flags & 2) != 0 { (0usize, tris) } else { (tris, 0usize) };
                std::ptr::write_unaligned(obj.add(0xa8) as *mut usize, verts);
                std::ptr::write_unaligned(obj.add(0xb8) as *mut usize, tris32);
                std::ptr::write_unaligned(obj.add(0xc0) as *mut usize, tris16);
                std::ptr::write_unaligned(obj.add(0xd0) as *mut usize, obj.add(0xa0) as usize);
            }
            1 => {
                // `Gu::HeightField::createObject` (`0x1800b5b30`)
                std::ptr::write_unaligned(obj.add(0x18) as *mut u32, 1);
                std::ptr::write_unaligned(obj.add(0x78) as *mut u32, 0);
                let rows = std::ptr::read_unaligned(obj.add(0x38) as *const u32) as usize;
                let cols = std::ptr::read_unaligned(obj.add(0x3c) as *const u32) as usize;
                let samples = ctx_consume_extra(ctx, 16, rows * cols * 4);
                std::ptr::write_unaligned(obj.add(0x50) as *mut *mut u8, samples);
            }
            8 => {
                // `NpShape::createObject` (`0x180064370`)
                std::ptr::write_unaligned(obj.add(0x20) as *mut u32, 1);
                let scb = std::ptr::read_unaligned(obj.add(0x38) as *const u32);
                std::ptr::write_unaligned(obj.add(0x38) as *mut u32, scb & 0x0f00_0000);
                std::ptr::write_unaligned(obj.add(0x30) as *mut usize, 0);
                std::ptr::write_unaligned(obj.add(0x40) as *mut usize, 0);
                *obj.add(0x91) = 0;
                let excl = std::ptr::read_unaligned(obj.add(0x108) as *const u32);
                std::ptr::write_unaligned(obj.add(0x108) as *mut u32, excl & 0x8000_0000);

                // `Scb::Shape::importExtraData` (`0x180113230` on `obj + 0x50`)
                let gtype = std::ptr::read_unaligned(obj.add(0x98) as *const u32);
                if gtype == 5 {
                    let num_mats = std::ptr::read_unaligned(obj.add(0xe0) as *const u16) as usize;
                    let p = ctx_consume_extra(ctx, 16, num_mats * 2);
                    std::ptr::write_unaligned(obj.add(0xd8) as *mut *mut u8, p);
                } else if gtype == 6 {
                    let num_mats = std::ptr::read_unaligned(obj.add(0xc8) as *const u16) as usize;
                    let p = ctx_consume_extra(ctx, 16, num_mats * 2);
                    std::ptr::write_unaligned(obj.add(0xc0) as *mut *mut u8, p);
                }

                // `readName` (`0x180064420..0x18006443e`)
                let extra_slot = ctx.add(0x08) as *mut *const u8;
                let p_extra = std::ptr::read_unaligned(extra_slot);
                if !p_extra.is_null() {
                    let name_len = std::ptr::read_unaligned(p_extra as *const u32) as usize;
                    let name_start = p_extra.add(4);
                    std::ptr::write_unaligned(
                        obj.add(0x100) as *mut *const u8,
                        if name_len == 0 { std::ptr::null() } else { name_start },
                    );
                    std::ptr::write_unaligned(extra_slot, name_start.add(name_len));
                }

                // `NpShape::resolveReferences` (`0x180064190` & `0x180113290`)
                let actor_ref = std::ptr::read_unaligned(obj.add(0x28) as *const usize);
                if actor_ref != 0 {
                    let resolved = ctx_resolve_ref(ctx, 0x8000_0000, actor_ref);
                    std::ptr::write_unaligned(obj.add(0x28) as *mut *mut u8, resolved);
                }
                match gtype {
                    4 => {
                        let mesh_ref = std::ptr::read_unaligned(obj.add(0xb8) as *const usize);
                        if mesh_ref != 0 {
                            let resolved = ctx_resolve_ref(ctx, 0x8000_0000, mesh_ref);
                            std::ptr::write_unaligned(obj.add(0xb8) as *mut *mut u8, resolved);
                            if !resolved.is_null() {
                                std::ptr::write_unaligned(
                                    obj.add(0xc8) as *mut *mut u8,
                                    resolved.add(0x20),
                                );
                            }
                        }
                    }
                    5 => {
                        let mesh_ref = std::ptr::read_unaligned(obj.add(0xc0) as *const usize);
                        if mesh_ref != 0 {
                            let resolved = ctx_resolve_ref(ctx, 0x8000_0000, mesh_ref);
                            std::ptr::write_unaligned(obj.add(0xc0) as *mut *mut u8, resolved);
                            std::ptr::write_unaligned(obj.add(0xc8) as *mut *mut u8, resolved);
                        }
                    }
                    6 => {
                        let hf_ref = std::ptr::read_unaligned(obj.add(0xa0) as *const usize);
                        if hf_ref != 0 {
                            let resolved = ctx_resolve_ref(ctx, 0x8000_0000, hf_ref);
                            std::ptr::write_unaligned(obj.add(0xa0) as *mut *mut u8, resolved);
                        }
                    }
                    _ => {}
                }
            }
            6 | 7 => {
                // `NpRigidDynamic::createObject` (`0x18004a7b0`) / `NpRigidStatic::createObject` (`0x1800501a0`)
                let scb = std::ptr::read_unaligned(obj.add(0x68) as *const u32);
                std::ptr::write_unaligned(obj.add(0x68) as *mut u32, scb & 0x0f00_0000);
                std::ptr::write_unaligned(obj.add(0x60) as *mut usize, 0);
                std::ptr::write_unaligned(obj.add(0x70) as *mut usize, 0);
                std::ptr::write_unaligned(obj.add(0x80) as *mut usize, 0);
                *obj.add(0x8e) = 0;

                // `NpShapeManager::importExtraData` (`0x180066380` -> `PtrTable::importExtraData` at `+0x28` & `+0x38`)
                let nb_shapes = std::ptr::read_unaligned(obj.add(0x30) as *const u16) as usize;
                *obj.add(0x32) = (nb_shapes < 2) as u8;
                if nb_shapes == 0 {
                    std::ptr::write_unaligned(obj.add(0x28) as *mut usize, 0);
                } else if nb_shapes > 1 {
                    let list = ctx_consume_extra(ctx, 16, nb_shapes * 8);
                    std::ptr::write_unaligned(obj.add(0x28) as *mut *mut u8, list);
                }
                let nb_sq = std::ptr::read_unaligned(obj.add(0x40) as *const u16) as usize;
                *obj.add(0x42) = (nb_sq < 2) as u8;
                if nb_sq == 0 {
                    std::ptr::write_unaligned(obj.add(0x38) as *mut usize, 0);
                } else if nb_sq > 1 {
                    let list = ctx_consume_extra(ctx, 16, nb_sq * 8);
                    std::ptr::write_unaligned(obj.add(0x38) as *mut *mut u8, list);
                }

                // `NpActor::importExtraData` (`0x180021540` on `obj + 0x18`)
                if std::ptr::read_unaligned(obj.add(0x20) as *const usize) != 0 {
                    let conn = ctx_consume_extra(ctx, 16, 0x58);
                    std::ptr::write_unaligned(obj.add(0x20) as *mut *mut u8, conn);
                    if std::ptr::read_unaligned(conn.add(0x48) as *const usize) != 0 {
                        let cap = std::ptr::read_unaligned(conn.add(0x54) as *const u32);
                        std::ptr::write_unaligned(conn.add(0x54) as *mut u32, cap | 0x8000_0000);
                    }
                    if *conn.add(0x40) != 0 {
                        std::ptr::write_unaligned(conn.add(0x48) as *mut *mut u8, conn);
                    }
                    let count = std::ptr::read_unaligned(conn.add(0x50) as *const u32) as usize;
                    if count == 0 {
                        std::ptr::write_unaligned(obj.add(0x20) as *mut usize, 0);
                    } else if *conn.add(0x40) == 0
                        && std::ptr::read_unaligned(conn.add(0x48) as *const usize) != 0
                    {
                        let cap = (std::ptr::read_unaligned(conn.add(0x54) as *const u32)
                            & 0x7fff_ffff) as usize;
                        let extra_slot = ctx.add(0x08) as *mut usize;
                        let buf = std::ptr::read_unaligned(extra_slot);
                        std::ptr::write_unaligned(extra_slot, buf.wrapping_add(cap * 16));
                        std::ptr::write_unaligned(conn.add(0x48) as *mut usize, buf);
                    }
                }
                let extra_slot = ctx.add(0x08) as *mut *const u8;
                let p_extra = std::ptr::read_unaligned(extra_slot);
                if !p_extra.is_null() {
                    let name_len = std::ptr::read_unaligned(p_extra as *const u32) as usize;
                    let name_start = p_extra.add(4);
                    std::ptr::write_unaligned(
                        obj.add(0x18) as *mut *const u8,
                        if name_len == 0 { std::ptr::null() } else { name_start },
                    );
                    std::ptr::write_unaligned(extra_slot, name_start.add(name_len));
                }

                // `NpRigidStatic/Dynamic::resolveReferences` (`0x18004e410` / `0x180051310` -> `0x1800215e0`)
                let slots_ptr: *mut usize = if nb_shapes == 1 {
                    obj.add(0x28) as *mut usize
                } else {
                    std::ptr::read_unaligned(obj.add(0x28) as *const *mut usize)
                };
                if !slots_ptr.is_null() {
                    for i in 0..nb_shapes {
                        let ref_id = std::ptr::read_unaligned(slots_ptr.add(i));
                        if ref_id != 0 {
                            let shape_ptr = ctx_resolve_ref(ctx, 0x8000_0000, ref_id);
                            std::ptr::write_unaligned(slots_ptr.add(i), shape_ptr as usize);
                            if !shape_ptr.is_null() {
                                std::ptr::write_unaligned(
                                    shape_ptr.add(0x28) as *mut *mut u8,
                                    obj,
                                );
                            }
                        }
                    }
                }
                let conn = std::ptr::read_unaligned(obj.add(0x20) as *const *mut u8);
                if !conn.is_null() {
                    let count = std::ptr::read_unaligned(conn.add(0x50) as *const u32) as usize;
                    let entries = std::ptr::read_unaligned(conn.add(0x48) as *const *mut u8);
                    if !entries.is_null() {
                        for i in 0..count {
                            let entry = entries.add(i * 16);
                            let ref_id = std::ptr::read_unaligned(entry.add(8) as *const usize);
                            if ref_id != 0 {
                                let resolved = ctx_resolve_ref(ctx, 0x8000_0000, ref_id);
                                std::ptr::write_unaligned(entry.add(8) as *mut *mut u8, resolved);
                            }
                        }
                    }
                }
                let _ = ctx_align_extra;
            }
            10 => {
                // `NpConstraint::createObject` (`0x18002ff40`)
                let scb = std::ptr::read_unaligned(obj.add(0x28) as *const u32);
                std::ptr::write_unaligned(obj.add(0x28) as *mut u32, scb & 0x0f00_0000);
                std::ptr::write_unaligned(obj.add(0x20) as *mut usize, 0);
                std::ptr::write_unaligned(obj.add(0x30) as *mut usize, 0);
                std::ptr::write_unaligned(obj.add(0x58) as *mut usize, 0);
                std::ptr::write_unaligned(obj.add(0x88) as *mut usize, 0);
                std::ptr::write_unaligned(obj.add(0xa8) as *mut u16, 0);
                let a0_ref = std::ptr::read_unaligned(obj.add(0x10) as *const usize);
                if a0_ref != 0 {
                    let a0 = ctx_resolve_ref(ctx, 0x8000_0000, a0_ref);
                    std::ptr::write_unaligned(obj.add(0x10) as *mut *mut u8, a0);
                }
                let a1_ref = std::ptr::read_unaligned(obj.add(0x18) as *const usize);
                if a1_ref != 0 {
                    let a1 = ctx_resolve_ref(ctx, 0x8000_0000, a1_ref);
                    std::ptr::write_unaligned(obj.add(0x18) as *mut *mut u8, a1);
                }
            }
            14 => {
                // `NpAggregate::createObject` (`0x1800230e0`)
                let scb = std::ptr::read_unaligned(obj.add(0x18) as *const u32);
                std::ptr::write_unaligned(obj.add(0x18) as *mut u32, scb & 0x0f00_0000);
                std::ptr::write_unaligned(obj.add(0x10) as *mut usize, 0);
                std::ptr::write_unaligned(obj.add(0x20) as *mut usize, 0);
                let nb_actors = std::ptr::read_unaligned(obj.add(0x40) as *const u32) as usize;
                if std::ptr::read_unaligned(obj.add(0x48) as *const usize) != 0 {
                    let list = ctx_consume_extra(ctx, 16, nb_actors * 8) as *mut usize;
                    std::ptr::write_unaligned(obj.add(0x48) as *mut *mut usize, list);
                    for i in 0..nb_actors {
                        let ref_id = std::ptr::read_unaligned(list.add(i));
                        if ref_id != 0 {
                            let actor = ctx_resolve_ref(ctx, 0x8000_0000, ref_id);
                            std::ptr::write_unaligned(list.add(i), actor as usize);
                        }
                    }
                }
            }
            18 => {
                // `Sq::PruningStructure::createObject` (`0x1800d45d0`)
                if *obj.add(0x50) != 0 {
                    if std::ptr::read_unaligned(obj.add(0x18) as *const usize) != 0 {
                        let n = std::ptr::read_unaligned(obj.add(0x10) as *const u32) as usize;
                        let p = ctx_consume_extra(ctx, 16, n * 0x1c);
                        std::ptr::write_unaligned(obj.add(0x18) as *mut *mut u8, p);
                    }
                    if std::ptr::read_unaligned(obj.add(0x30) as *const usize) != 0 {
                        let n = std::ptr::read_unaligned(obj.add(0x28) as *const u32) as usize;
                        let p = ctx_consume_extra(ctx, 16, n * 4);
                        std::ptr::write_unaligned(obj.add(0x30) as *mut *mut u8, p);
                    }
                    if std::ptr::read_unaligned(obj.add(0x20) as *const usize) != 0 {
                        let n = std::ptr::read_unaligned(obj.add(0x14) as *const u32) as usize;
                        let p = ctx_consume_extra(ctx, 16, n * 0x1c);
                        std::ptr::write_unaligned(obj.add(0x20) as *mut *mut u8, p);
                    }
                    if std::ptr::read_unaligned(obj.add(0x38) as *const usize) != 0 {
                        let n = std::ptr::read_unaligned(obj.add(0x2c) as *const u32) as usize;
                        let p = ctx_consume_extra(ctx, 16, n * 4);
                        std::ptr::write_unaligned(obj.add(0x38) as *mut *mut u8, p);
                    }
                    if std::ptr::read_unaligned(obj.add(0x48) as *const usize) != 0 {
                        let n = std::ptr::read_unaligned(obj.add(0x40) as *const u32) as usize;
                        let list = ctx_consume_extra(ctx, 16, n * 8) as *mut usize;
                        std::ptr::write_unaligned(obj.add(0x48) as *mut *mut usize, list);
                        for i in 0..n {
                            let ref_id = std::ptr::read_unaligned(list.add(i));
                            if ref_id != 0 {
                                let actor = ctx_resolve_ref(ctx, 0x8000_0000, ref_id);
                                std::ptr::write_unaligned(list.add(i), actor as usize);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    obj
}

unsafe extern "win64" fn serializer_dtor(
    this: *mut PxSerializerAdapter,
    _flags: u32,
) -> *mut PxSerializerAdapter {
    this
}

static SERIALIZER_VTBL: [Ptr; 9] = [
    Ptr(serializer_get_type_name as *const ()),     // 0 (+0x00): getConcreteTypeName
    Ptr(serializer_requires_objects as *const ()),  // 1 (+0x08): requiresObjects
    Ptr(serializer_is_subordinate as *const ()),    // 2 (+0x10): isSubordinate
    Ptr(serializer_export_extra_data as *const ()), // 3 (+0x18): exportExtraData
    Ptr(serializer_export_data as *const ()),       // 4 (+0x20): exportData
    Ptr(serializer_register_references as *const ()), // 5 (+0x28): registerReferences
    Ptr(serializer_get_class_size as *const ()),    // 6 (+0x30): getClassSize
    Ptr(serializer_create_object as *const ()),     // 7 (+0x38): createObject
    Ptr(serializer_dtor as *const ()),              // 8 (+0x40): ~PxSerializer
];

const PHYSICS_SERIALIZER_SPECS: &[(u16, usize, &[u8])] = &[
    (2, 0xa8, b"ConvexMesh\0"),
    (3, 0x100, b"RTreeTriangleMesh\0"),
    (4, 0x118, b"BV4TriangleMesh\0"),
    (1, 0x88, b"HeightField\0"),
    (6, 0x190, b"NpRigidDynamic\0"),
    (7, 0xb0, b"NpRigidStatic\0"),
    (8, 0x110, b"NpShape\0"),
    (9, 0x50, b"NpMaterial\0"),
    (10, 0xb8, b"NpConstraint\0"),
    (14, 0x50, b"NpAggregate\0"),
    (15, 0xa8, b"NpArticulation\0"),
    (16, 0x1e0, b"NpArticulationLink\0"),
    (17, 0xf0, b"NpArticulationJoint\0"),
    (18, 0x58, b"Sq::PruningStructure\0"),
    (11, 0xe0, b"NpCloth\0"),
    (5, 0x40, b"NpClothFabric\0"),
    (12, 0x108, b"NpParticleSystem\0"),
    (13, 0x108, b"NpParticleFluid\0"),
];

fn ensure_vtables_registered() {
    REGISTER_VTABLES.call_once(|| {
        for p in PHYSICS_VTBL
            .iter()
            .chain(SCENE_VTBL.iter())
            .chain(MATERIAL_VTBL.iter())
            .chain(SHAPE_VTBL.iter())
            .chain(RIGID_ACTOR_VTBL.iter())
            .chain(GENERIC_BASE_VTBL.iter())
            .chain(CONVEX_MESH_VTBL.iter())
            .chain(TRIANGLE_MESH_VTBL.iter())
            .chain(CONSTRAINT_VTBL.iter())
            .chain(AGGREGATE_VTBL.iter())
            .chain(INSERTION_CB_VTBL.iter())
            .chain(PVD_CLIENT_VTBL.iter())
            .chain(SERIALIZER_VTBL.iter())
        {
            let addr = p.0 as usize;
            if addr != 0 {
                crate::register_native_va(addr as u64, addr);
            }
        }
    });
}

fn get_or_init_physics() -> *mut PxPhysicsInstance {
    ensure_vtables_registered();
    let cur = PHYSICS_PTR.load(Ordering::Acquire);
    if !cur.is_null() {
        return cur;
    }
    let inst = Box::new(PxPhysicsInstance {
        vtbl: &PHYSICS_VTBL,
        foundation: unsafe { PxGetFoundation() as *mut c_void },
        pvd: std::ptr::null_mut(),
        _pad0: [0; 2],
        tolerances_scale: [1.0, 10.0],
        _pad1: [0u8; 0x38],
        insertion_cb: PxInsertionCallbackBridge {
            vtbl: &INSERTION_CB_VTBL,
        },
        _pad2: [0u8; 0xa0],
    });
    let raw = Box::into_raw(inst);
    match PHYSICS_PTR.compare_exchange(
        std::ptr::null_mut(),
        raw,
        Ordering::AcqRel,
        Ordering::Acquire,
    ) {
        Ok(_) => raw,
        Err(existing) => {
            unsafe {
                let _ = Box::from_raw(raw);
            }
            existing
        }
    }
}

// ---------------------------------------------------------------------------
// Exported C functions (10 symbols)
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn PxCreateBasePhysics(
    _version: u32,
    foundation: *mut c_void,
    scale: *const [f32; 2],
    _track_allocations: u8,
    pvd: *mut c_void,
) -> *mut PxPhysicsInstance {
    let p = get_or_init_physics();
    if !foundation.is_null() {
        (*p).foundation = foundation;
    }
    if !scale.is_null() {
        (*p).tolerances_scale = std::ptr::read_unaligned(scale);
    }
    (*p).pvd = pvd;
    p
}

pub unsafe extern "win64" fn PxGetPhysics() -> *mut PxPhysicsInstance {
    get_or_init_physics()
}

pub unsafe extern "win64" fn PxRegisterArticulations(_physics: *mut PxPhysicsInstance) {}

pub unsafe extern "win64" fn PxRegisterHeightFields(_physics: *mut PxPhysicsInstance) {}

pub unsafe extern "win64" fn PxRegisterCloth(_physics: *mut PxPhysicsInstance) {}

pub unsafe extern "win64" fn PxRegisterParticles(_physics: *mut PxPhysicsInstance) {}

pub unsafe extern "win64" fn PxAddCollectionToPhysics(_collection: *const c_void) {}

pub unsafe extern "win64" fn PxGetPhysicsBinaryMetaData(_stream: *mut c_void) {}

pub unsafe extern "win64" fn PxRegisterPhysicsSerializers(sr: *mut *const usize) {
    ensure_vtables_registered();
    if sr.is_null() || (*sr).is_null() {
        return;
    }
    let reg_fn = *(*sr); // slot 0 (+0x00): registerSerializer(this, type: u16, serializer: &PxSerializer)
    if reg_fn == 0 {
        return;
    }
    for &(ctype, class_size, name) in PHYSICS_SERIALIZER_SPECS {
        let adapter = allocator_allocate(
            std::ptr::null_mut(),
            std::mem::size_of::<PxSerializerAdapter>(),
            b"PxSerializerDefaultAdapter\0".as_ptr(),
            b"NpSerializerAdapter.cpp\0".as_ptr(),
            153,
        ) as *mut PxSerializerAdapter;
        if !adapter.is_null() {
            (*adapter).vtbl = &SERIALIZER_VTBL;
            (*adapter).type_name = name.as_ptr();
            (*adapter).class_size = class_size;
            (*adapter).concrete_type = ctype;
            crate::call_guest8(
                reg_fn,
                sr as u64,
                ctype as u64,
                adapter as u64,
                0,
                0,
                0,
                0,
                0,
            );
        }
    }
}

pub unsafe extern "win64" fn PxUnregisterPhysicsSerializers(sr: *mut *const usize) {
    if sr.is_null() || (*sr).is_null() {
        return;
    }
    let unreg_fn = *(*sr).add(1); // slot 1 (+0x08): unregisterSerializer(this, type: u16) -> *mut PxSerializer
    if unreg_fn == 0 {
        return;
    }
    for &(ctype, _, _) in PHYSICS_SERIALIZER_SPECS {
        let adapter = crate::call_guest8(unreg_fn, sr as u64, ctype as u64, 0, 0, 0, 0, 0, 0)
            as *mut c_void;
        if !adapter.is_null() {
            allocator_deallocate(std::ptr::null_mut(), adapter);
        }
    }
}

// ---------------------------------------------------------------------------
// Exported C++-mangled functions (31 symbols: 24 GeneratedInfo ctors + 7 getGeometry helpers)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn generated_info_ctor(this: *mut c_void) -> *mut c_void {
    this
}

unsafe extern "win64" fn shape_geom_helper_get(
    _this: *const c_void,
    shape: *const PxShapeInstance,
    geom_out: *mut u8,
) -> u8 {
    if shape.is_null() || geom_out.is_null() {
        return 0;
    }
    let gtype = (*shape).geom_type;
    let bytes = match gtype {
        0 => 0x08usize,
        1 => 0x04,
        2 => 0x0c,
        3 => 0x10,
        4 => 0x30,
        5 => 0x30,
        6 => 0x20,
        _ => return 0,
    };
    shape_copy_geom_if_type(shape, geom_out, gtype, bytes)
}

pub static EXTRA_EXPORTS: &[Export] = &[
    Export {
        name: "??0PxShapeGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxClothFabricGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxRigidDynamicGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxHeightFieldDescGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxRigidStaticGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxClothMotionConstraintConfigGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxMaterialGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "?getGeometry@PxShapeGeometryPropertyHelper@physx@@QEBA_NPEBVPxShape@2@AEAVPxHeightFieldGeometry@2@@Z",
        ptr: Ptr(shape_geom_helper_get as *const ()),
    },
    Export {
        name: "??0PxClothTetherConfigGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxClothStretchConfigGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxArticulationGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxAggregateGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxHeightFieldGeometryGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxTriangleMeshGeometryGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxPlaneGeometryGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxSphereGeometryGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxArticulationLinkGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxConvexMeshGeometryGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "?getGeometry@PxShapeGeometryPropertyHelper@physx@@QEBA_NPEBVPxShape@2@AEAVPxBoxGeometry@2@@Z",
        ptr: Ptr(shape_geom_helper_get as *const ()),
    },
    Export {
        name: "??0PxMeshScaleGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxCapsuleGeometryGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxBoxGeometryGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxParticleSystemGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxParticleFluidGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "??0PxClothGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
    Export {
        name: "?getGeometry@PxShapeGeometryPropertyHelper@physx@@QEBA_NPEBVPxShape@2@AEAVPxSphereGeometry@2@@Z",
        ptr: Ptr(shape_geom_helper_get as *const ()),
    },
    Export {
        name: "?getGeometry@PxShapeGeometryPropertyHelper@physx@@QEBA_NPEBVPxShape@2@AEAVPxCapsuleGeometry@2@@Z",
        ptr: Ptr(shape_geom_helper_get as *const ()),
    },
    Export {
        name: "?getGeometry@PxShapeGeometryPropertyHelper@physx@@QEBA_NPEBVPxShape@2@AEAVPxPlaneGeometry@2@@Z",
        ptr: Ptr(shape_geom_helper_get as *const ()),
    },
    Export {
        name: "?getGeometry@PxShapeGeometryPropertyHelper@physx@@QEBA_NPEBVPxShape@2@AEAVPxConvexMeshGeometry@2@@Z",
        ptr: Ptr(shape_geom_helper_get as *const ()),
    },
    Export {
        name: "?getGeometry@PxShapeGeometryPropertyHelper@physx@@QEBA_NPEBVPxShape@2@AEAVPxTriangleMeshGeometry@2@@Z",
        ptr: Ptr(shape_geom_helper_get as *const ()),
    },
    Export {
        name: "??0PxArticulationJointGeneratedInfo@physx@@QEAA@XZ",
        ptr: Ptr(generated_info_ctor as *const ()),
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn px_physics_lifecycle_and_serializers() {
        assert_eq!(EXTRA_EXPORTS.len(), 31);
        unsafe {
            let scale = [1.0f32, 10.0f32];
            let p = PxCreateBasePhysics(
                0x0304_0200,
                std::ptr::null_mut(),
                &scale,
                0,
                std::ptr::null_mut(),
            );
            assert!(!p.is_null());
            assert_eq!(PxGetPhysics(), p);

            let mat = physics_create_material(p, 0.8, 0.6, 0.2);
            assert!(!mat.is_null());
            assert!((material_get_stat_friction(mat) - 0.8).abs() < 1e-6);
            assert!((material_get_dyn_friction(mat) - 0.6).abs() < 1e-6);
            base_obj_release(mat as *mut u8);

            let scene = physics_create_scene(p, std::ptr::null());
            assert!(!scene.is_null());
            assert_eq!(scene_get_physics(scene), p);
            let mut err = 99u32;
            assert_eq!(scene_fetch_results(scene, 1, &mut err), 1);
            assert_eq!(err, 0);
            scene_release(scene);
        }
    }
}
