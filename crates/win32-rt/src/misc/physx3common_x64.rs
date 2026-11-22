//! `physx3common_x64.dll` — native Rust replacement for NVIDIA PhysX 3.4.2 Common (`PhysX3Common_x64.dll`).
//!
//! Implements all 11 exports, including `PxCreateCollection` with the exact `0x78`-byte
//! `Cm::Collection` (`Ps::CoalescedHashMap`) layout and analytic `PxGeometryQuery` routines
//! (`getWorldBounds`, `pointDistance`, `raycast`, `overlap`, `sweep`, `computePenetration`)
//! for `PxSphereGeometry`, `PxPlaneGeometry`, `PxCapsuleGeometry`, and `PxBoxGeometry`.

use crate::misc::pxfoundation_x64::{
    allocator_allocate, allocator_deallocate, shortest_rotation_quat,
};
use crate::{Export, Ptr};
use std::ffi::c_void;
use std::sync::Once;

pub static ORDINALS: &[(u16, &str)] = &[];

static REGISTER_VTABLES: Once = Once::new();

const EOL: u32 = 0xffff_ffff;
const PX_MAX_BOUNDS_EXTENTS: f32 = f32::MAX * 0.25;

// ---------------------------------------------------------------------------
// PxCollection / Cm::Collection (`CmCollection.h` + `PsHashInternals.h`)
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
pub struct HashEntry {
    pub first: u64,
    pub second: u64,
}
const _: () = assert!(std::mem::size_of::<HashEntry>() == 0x10);

/// Exact `0x38`-byte layout of `physx::shdfnd::internal::HashBase` (`PsHashInternals.h`).
#[repr(C)]
pub struct PsHashBase {
    pub buffer: *mut u8,         // +0x00
    pub entries: *mut HashEntry, // +0x08
    pub entries_next: *mut u32,  // +0x10
    pub hash: *mut u32,          // +0x18
    pub entries_capacity: u32,   // +0x20
    pub hash_size: u32,          // +0x24
    pub load_factor: f32,        // +0x28 (0.75)
    pub free_list: u32,          // +0x2c (EOL)
    pub timestamp: u32,          // +0x30
    pub entries_count: u32,      // +0x34
}
const _: () = assert!(std::mem::size_of::<PsHashBase>() == 0x38);

/// Exact `0x78`-byte layout of `physx::Cm::Collection` (`CmCollection.h`).
#[repr(C)]
pub struct CmCollection {
    pub vtbl: *const [Ptr; 16], // +0x00
    pub ids: PsHashBase,        // +0x08..+0x40 (`IdToObjectMap`: key=id, val=obj)
    pub objects: PsHashBase,    // +0x40..+0x78 (`ObjectToIdMap`: key=obj, val=id)
}
const _: () = assert!(std::mem::size_of::<CmCollection>() == 0x78);

#[inline]
pub fn physx_hash_u64(mut k: u64) -> u32 {
    k = k.wrapping_add(!(k << 32));
    k ^= k >> 22;
    k = k.wrapping_add(!(k << 13));
    k ^= k >> 8;
    k = k.wrapping_add(k << 3);
    k ^= k >> 15;
    k = k.wrapping_add(!(k << 27));
    k ^= k >> 31;
    k as u32
}

unsafe fn hash_base_init(h: *mut PsHashBase, initial_size: u32) {
    (*h).buffer = std::ptr::null_mut();
    (*h).entries = std::ptr::null_mut();
    (*h).entries_next = std::ptr::null_mut();
    (*h).hash = std::ptr::null_mut();
    (*h).entries_capacity = 0;
    (*h).hash_size = 0;
    (*h).load_factor = 0.75;
    (*h).free_list = EOL;
    (*h).timestamp = 0;
    (*h).entries_count = 0;
    if initial_size > 0 {
        hash_base_reserve(h, initial_size);
    }
}

unsafe fn hash_base_reserve(h: *mut PsHashBase, mut size: u32) {
    if !size.is_power_of_two() {
        size = size.next_power_of_two();
    }
    let new_hash_size = size;
    let new_entries_cap = ((new_hash_size as f32) * (*h).load_factor) as u32;
    let hash_bytes = (new_hash_size as usize) * 4;
    let next_bytes = (new_entries_cap as usize) * 4;
    let mut entries_off = hash_bytes + next_bytes;
    entries_off = (entries_off + 15) & !15;
    let total_bytes = entries_off + (new_entries_cap as usize) * std::mem::size_of::<HashEntry>();

    let new_buf = allocator_allocate(
        std::ptr::null_mut(),
        total_bytes,
        b"HashBase\0".as_ptr(),
        b"PsHashInternals.h\0".as_ptr(),
        372,
    ) as *mut u8;
    if new_buf.is_null() {
        return;
    }
    let new_hash = new_buf as *mut u32;
    let new_next = new_buf.add(hash_bytes) as *mut u32;
    let new_entries = new_buf.add(entries_off) as *mut HashEntry;

    std::ptr::write_bytes(new_hash as *mut u8, 0xff, hash_bytes);
    for i in 0..((*h).entries_count as usize) {
        let entry = *(*h).entries.add(i);
        let bucket = (physx_hash_u64(entry.first) & (new_hash_size - 1)) as usize;
        *new_next.add(i) = *new_hash.add(bucket);
        *new_hash.add(bucket) = i as u32;
        *new_entries.add(i) = entry;
    }
    for i in ((*h).entries_count)..new_entries_cap {
        *new_next.add(i as usize) = if i + 1 < new_entries_cap { i + 1 } else { EOL };
    }
    (*h).free_list = if (*h).entries_count < new_entries_cap {
        (*h).entries_count
    } else {
        EOL
    };

    if !(*h).buffer.is_null() {
        allocator_deallocate(std::ptr::null_mut(), (*h).buffer as *mut c_void);
    }
    (*h).buffer = new_buf;
    (*h).hash = new_hash;
    (*h).hash_size = new_hash_size;
    (*h).entries_next = new_next;
    (*h).entries = new_entries;
    (*h).entries_capacity = new_entries_cap;
}

unsafe fn hash_base_find(h: *const PsHashBase, key: u64) -> *mut HashEntry {
    if (*h).hash.is_null() || (*h).hash_size == 0 {
        return std::ptr::null_mut();
    }
    let bucket = (physx_hash_u64(key) & ((*h).hash_size - 1)) as usize;
    let mut idx = *(*h).hash.add(bucket);
    while idx != EOL && idx < (*h).entries_capacity {
        let entry = (*h).entries.add(idx as usize);
        if (*entry).first == key {
            return entry;
        }
        idx = *(*h).entries_next.add(idx as usize);
    }
    std::ptr::null_mut()
}

unsafe fn hash_base_insert(h: *mut PsHashBase, key: u64, val: u64) -> bool {
    let existing = hash_base_find(h, key);
    if !existing.is_null() {
        (*existing).second = val;
        return false;
    }
    if (*h).free_list == EOL || (*h).entries_count >= (*h).entries_capacity {
        let next_sz = if (*h).hash_size == 0 {
            64
        } else {
            (*h).hash_size * 2
        };
        hash_base_reserve(h, next_sz);
    }
    let idx = (*h).entries_count;
    (*h).entries_count += 1;
    if (*h).free_list != EOL && !(*h).entries_next.is_null() {
        (*h).free_list = *(*h).entries_next.add(idx as usize);
    }
    (*h).timestamp = (*h).timestamp.wrapping_add(1);
    let bucket = (physx_hash_u64(key) & ((*h).hash_size - 1)) as usize;
    *(*h).entries_next.add(idx as usize) = *(*h).hash.add(bucket);
    *(*h).hash.add(bucket) = idx;
    *(*h).entries.add(idx as usize) = HashEntry {
        first: key,
        second: val,
    };
    true
}

unsafe fn hash_base_erase(h: *mut PsHashBase, key: u64) -> Option<HashEntry> {
    if (*h).entries_count == 0 || (*h).hash.is_null() || (*h).hash_size == 0 {
        return None;
    }
    let bucket = (physx_hash_u64(key) & ((*h).hash_size - 1)) as usize;
    let mut prev_slot = (*h).hash.add(bucket);
    let mut idx = *prev_slot;
    while idx != EOL && idx < (*h).entries_capacity {
        let entry = (*h).entries.add(idx as usize);
        if (*entry).first == key {
            let removed = *entry;
            *prev_slot = *(*h).entries_next.add(idx as usize);
            (*h).entries_count -= 1;
            let last_idx = (*h).entries_count;
            (*h).timestamp = (*h).timestamp.wrapping_add(1);
            if idx != last_idx {
                let moved = *(*h).entries.add(last_idx as usize);
                *(*h).entries.add(idx as usize) = moved;
                *(*h).entries_next.add(idx as usize) = *(*h).entries_next.add(last_idx as usize);
                let moved_bucket = (physx_hash_u64(moved.first) & ((*h).hash_size - 1)) as usize;
                let mut slot = (*h).hash.add(moved_bucket);
                while *slot != EOL && *slot != last_idx {
                    slot = (*h).entries_next.add(*slot as usize);
                }
                if *slot == last_idx {
                    *slot = idx;
                }
            }
            *(*h).entries_next.add(last_idx as usize) = (*h).free_list;
            (*h).free_list = last_idx;
            return Some(removed);
        }
        prev_slot = (*h).entries_next.add(idx as usize);
        idx = *prev_slot;
    }
    None
}

// ---------------------------------------------------------------------------
// PxCollection vtable (16 slots)
// ---------------------------------------------------------------------------

unsafe extern "win64" fn collection_add_collection(
    this: *mut CmCollection,
    other: *const CmCollection,
) {
    if this.is_null() || other.is_null() || this as *const _ == other {
        return;
    }
    let count = (*other).objects.entries_count as usize;
    for i in 0..count {
        let e = *(*other).objects.entries.add(i);
        collection_add_object(this, e.first as *mut c_void, e.second);
    }
}

unsafe extern "win64" fn collection_add_object(
    this: *mut CmCollection,
    object: *mut c_void,
    id: u64,
) {
    if this.is_null() || object.is_null() {
        return;
    }
    if id != 0 {
        hash_base_insert(&mut (*this).ids, id, object as u64);
    }
    hash_base_insert(&mut (*this).objects, object as u64, id);
}

unsafe extern "win64" fn collection_remove_collection(
    this: *mut CmCollection,
    other: *const CmCollection,
) {
    if this.is_null() || other.is_null() {
        return;
    }
    if this as *const _ == other {
        while (*this).objects.entries_count > 0 {
            let obj = (*(*this).objects.entries).first as *mut c_void;
            collection_remove_object(this, obj);
        }
        return;
    }
    let count = (*other).objects.entries_count as usize;
    for i in 0..count {
        let obj = (*(*other).objects.entries.add(i)).first as *mut c_void;
        collection_remove_object(this, obj);
    }
}

unsafe extern "win64" fn collection_remove_object(this: *mut CmCollection, object: *mut c_void) {
    if this.is_null() || object.is_null() {
        return;
    }
    if let Some(removed) = hash_base_erase(&mut (*this).objects, object as u64) {
        if removed.second != 0 {
            let _ = hash_base_erase(&mut (*this).ids, removed.second);
        }
    }
}

unsafe extern "win64" fn collection_contains(
    this: *const CmCollection,
    object: *const c_void,
) -> u8 {
    if this.is_null() || object.is_null() {
        return 0;
    }
    (!hash_base_find(&(*this).objects, object as u64).is_null()) as u8
}

unsafe extern "win64" fn collection_add_id(this: *mut CmCollection, object: *mut c_void, id: u64) {
    if this.is_null() || object.is_null() || id == 0 {
        return;
    }
    hash_base_insert(&mut (*this).ids, id, object as u64);
    hash_base_insert(&mut (*this).objects, object as u64, id);
}

unsafe extern "win64" fn collection_remove_id(this: *mut CmCollection, id: u64) {
    if this.is_null() || id == 0 {
        return;
    }
    if let Some(removed) = hash_base_erase(&mut (*this).ids, id) {
        let obj_entry = hash_base_find(&(*this).objects, removed.second);
        if !obj_entry.is_null() {
            (*obj_entry).second = 0;
        }
    }
}

unsafe extern "win64" fn collection_get_nb_objects(this: *const CmCollection) -> u32 {
    if this.is_null() {
        0
    } else {
        (*this).objects.entries_count
    }
}

unsafe extern "win64" fn collection_get_object(
    this: *const CmCollection,
    index: u32,
) -> *mut c_void {
    if this.is_null() || index >= (*this).objects.entries_count {
        return std::ptr::null_mut();
    }
    (*(*this).objects.entries.add(index as usize)).first as *mut c_void
}

unsafe extern "win64" fn collection_get_objects(
    this: *const CmCollection,
    user_buffer: *mut *mut c_void,
    buffer_size: u32,
    start_index: u32,
) -> u32 {
    if this.is_null() || user_buffer.is_null() || buffer_size == 0 {
        return 0;
    }
    let total = (*this).objects.entries_count;
    let mut dst = 0u32;
    let mut src = start_index;
    while src < total && dst < buffer_size {
        std::ptr::write_unaligned(
            user_buffer.add(dst as usize),
            (*(*this).objects.entries.add(src as usize)).first as *mut c_void,
        );
        src += 1;
        dst += 1;
    }
    dst
}

unsafe extern "win64" fn collection_find(this: *const CmCollection, id: u64) -> *mut c_void {
    if this.is_null() || id == 0 {
        return std::ptr::null_mut();
    }
    let e = hash_base_find(&(*this).ids, id);
    if e.is_null() {
        std::ptr::null_mut()
    } else {
        (*e).second as *mut c_void
    }
}

unsafe extern "win64" fn collection_get_nb_ids(this: *const CmCollection) -> u32 {
    if this.is_null() {
        0
    } else {
        (*this).ids.entries_count
    }
}

unsafe extern "win64" fn collection_get_ids(
    this: *const CmCollection,
    user_buffer: *mut u64,
    buffer_size: u32,
    start_index: u32,
) -> u32 {
    if this.is_null() || user_buffer.is_null() || buffer_size == 0 {
        return 0;
    }
    let total = (*this).ids.entries_count;
    let mut dst = 0u32;
    let mut src = start_index;
    while src < total && dst < buffer_size {
        std::ptr::write_unaligned(
            user_buffer.add(dst as usize),
            (*(*this).ids.entries.add(src as usize)).first,
        );
        src += 1;
        dst += 1;
    }
    dst
}

unsafe extern "win64" fn collection_get_id(
    this: *const CmCollection,
    object: *const c_void,
) -> u64 {
    if this.is_null() || object.is_null() {
        return 0;
    }
    let e = hash_base_find(&(*this).objects, object as u64);
    if e.is_null() {
        0
    } else {
        (*e).second
    }
}

unsafe extern "win64" fn collection_release(this: *mut CmCollection) {
    if !this.is_null() {
        collection_dtor(this, 1);
    }
}

unsafe extern "win64" fn collection_dtor(
    this: *mut CmCollection,
    flags: u32,
) -> *mut CmCollection {
    if !this.is_null() {
        if !(*this).ids.buffer.is_null() {
            allocator_deallocate(std::ptr::null_mut(), (*this).ids.buffer as *mut c_void);
            (*this).ids.buffer = std::ptr::null_mut();
        }
        if !(*this).objects.buffer.is_null() {
            allocator_deallocate(std::ptr::null_mut(), (*this).objects.buffer as *mut c_void);
            (*this).objects.buffer = std::ptr::null_mut();
        }
        if (flags & 1) != 0 {
            allocator_deallocate(std::ptr::null_mut(), this as *mut c_void);
        }
    }
    this
}

static COLLECTION_VTBL: [Ptr; 16] = [
    Ptr(collection_add_collection as *const ()),    // 0 (+0x00): add(PxCollection&)
    Ptr(collection_add_object as *const ()),        // 1 (+0x08): add(PxBase&, PxSerialObjectId)
    Ptr(collection_remove_collection as *const ()), // 2 (+0x10): remove(PxCollection&)
    Ptr(collection_remove_object as *const ()),     // 3 (+0x18): remove(PxBase&)
    Ptr(collection_contains as *const ()),          // 4 (+0x20): contains(PxBase&)
    Ptr(collection_add_id as *const ()),            // 5 (+0x28): addId(PxBase&, PxSerialObjectId)
    Ptr(collection_remove_id as *const ()),         // 6 (+0x30): removeId(PxSerialObjectId)
    Ptr(collection_get_nb_objects as *const ()),    // 7 (+0x38): getNbObjects()
    Ptr(collection_get_object as *const ()),        // 8 (+0x40): getObject(u32)
    Ptr(collection_get_objects as *const ()),       // 9 (+0x48): getObjects(PxBase**, u32, u32)
    Ptr(collection_find as *const ()),              // 10 (+0x50): find(PxSerialObjectId)
    Ptr(collection_get_nb_ids as *const ()),        // 11 (+0x58): getNbIds()
    Ptr(collection_get_ids as *const ()),           // 12 (+0x60): getIds(PxSerialObjectId*, u32, u32)
    Ptr(collection_get_id as *const ()),            // 13 (+0x68): getId(const PxBase&)
    Ptr(collection_release as *const ()),           // 14 (+0x70): release()
    Ptr(collection_dtor as *const ()),              // 15 (+0x78): ~PxCollection()
];

fn ensure_vtables_registered() {
    REGISTER_VTABLES.call_once(|| {
        for p in COLLECTION_VTBL.iter() {
            let addr = p.0 as usize;
            if addr != 0 {
                crate::register_native_va(addr as u64, addr);
            }
        }
    });
}

pub unsafe extern "win64" fn px_create_collection() -> *mut CmCollection {
    ensure_vtables_registered();
    let ptr = allocator_allocate(
        std::ptr::null_mut(),
        std::mem::size_of::<CmCollection>(),
        b"Collection\0".as_ptr(),
        b"CmCollection.cpp\0".as_ptr(),
        216,
    ) as *mut CmCollection;
    if !ptr.is_null() {
        (*ptr).vtbl = &COLLECTION_VTBL;
        hash_base_init(&mut (*ptr).ids, 64);
        hash_base_init(&mut (*ptr).objects, 64);
    }
    ptr
}

// ---------------------------------------------------------------------------
// PhysX 3.4 Geometry & Query Hit ABI structs (verified against PxGeometry*.h / PxQueryReport.h)
// ---------------------------------------------------------------------------

pub const PX_GEOM_SPHERE: i32 = 0;
pub const PX_GEOM_PLANE: i32 = 1;
pub const PX_GEOM_CAPSULE: i32 = 2;
pub const PX_GEOM_BOX: i32 = 3;
pub const PX_GEOM_CONVEX_MESH: i32 = 4;
pub const PX_GEOM_TRIANGLE_MESH: i32 = 5;
pub const PX_GEOM_HEIGHTFIELD: i32 = 6;

pub const PX_HIT_POSITION: u16 = 1 << 0;
pub const PX_HIT_NORMAL: u16 = 1 << 1;
pub const PX_HIT_DISTANCE: u16 = 1 << 2;
pub const PX_HIT_ASSUME_NO_INITIAL_OVERLAP: u16 = 1 << 4;
pub const PX_HIT_MTD: u16 = 1 << 9;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PxBounds3 {
    pub minimum: [f32; 3],
    pub maximum: [f32; 3],
}
const _: () = assert!(std::mem::size_of::<PxBounds3>() == 0x18);

impl PxBounds3 {
    const fn empty() -> Self {
        Self {
            minimum: [PX_MAX_BOUNDS_EXTENTS; 3],
            maximum: [-PX_MAX_BOUNDS_EXTENTS; 3],
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PxTransform {
    pub q: [f32; 4], // +0x00..+0x10 (x, y, z, w)
    pub p: [f32; 3], // +0x10..+0x1c
}
const _: () = assert!(std::mem::size_of::<PxTransform>() == 0x1c);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PxSphereGeometry {
    pub geom_type: i32, // +0x00 (= 0)
    pub radius: f32,    // +0x04
}
const _: () = assert!(std::mem::size_of::<PxSphereGeometry>() == 0x08);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PxPlaneGeometry {
    pub geom_type: i32, // +0x00 (= 1)
}
const _: () = assert!(std::mem::size_of::<PxPlaneGeometry>() == 0x04);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PxCapsuleGeometry {
    pub geom_type: i32,   // +0x00 (= 2)
    pub radius: f32,      // +0x04
    pub half_height: f32, // +0x08
}
const _: () = assert!(std::mem::size_of::<PxCapsuleGeometry>() == 0x0c);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PxBoxGeometry {
    pub geom_type: i32,         // +0x00 (= 3)
    pub half_extents: [f32; 3], // +0x04
}
const _: () = assert!(std::mem::size_of::<PxBoxGeometry>() == 0x10);

/// Exact Win64 MSVC layout of `physx::PxRaycastHit` (`PxQueryReport.h`).
///
/// MSVC does not reuse `PxQueryHit`'s 4-byte tail padding (`+0x14..+0x18`), placing
/// `PxLocationHit::flags` at `+0x18` and matching `PX_COMPILE_TIME_ASSERT(0 == (sizeof(PxRaycastHit) & 0x0f))`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PxRaycastHit {
    pub actor: *mut c_void, // +0x00
    pub shape: *mut c_void, // +0x08
    pub face_index: u32,    // +0x10
    pub _base_pad: u32,     // +0x14
    pub flags: u16,         // +0x18
    pub _flags_pad: u16,    // +0x1a
    pub position: [f32; 3], // +0x1c
    pub normal: [f32; 3],   // +0x28
    pub distance: f32,      // +0x34
    pub u: f32,             // +0x38
    pub v: f32,             // +0x3c
}
const _: () = assert!(std::mem::size_of::<PxRaycastHit>() == 0x40);
const _: () = assert!(std::mem::offset_of!(PxRaycastHit, face_index) == 0x10);
const _: () = assert!(std::mem::offset_of!(PxRaycastHit, flags) == 0x18);
const _: () = assert!(std::mem::offset_of!(PxRaycastHit, position) == 0x1c);
const _: () = assert!(std::mem::offset_of!(PxRaycastHit, normal) == 0x28);
const _: () = assert!(std::mem::offset_of!(PxRaycastHit, distance) == 0x34);
const _: () = assert!(std::mem::offset_of!(PxRaycastHit, u) == 0x38);
const _: () = assert!(std::mem::offset_of!(PxRaycastHit, v) == 0x3c);

/// Exact Win64 MSVC layout of `physx::PxSweepHit` (`PxQueryReport.h`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PxSweepHit {
    pub actor: *mut c_void, // +0x00
    pub shape: *mut c_void, // +0x08
    pub face_index: u32,    // +0x10
    pub _base_pad: u32,     // +0x14
    pub flags: u16,         // +0x18
    pub _flags_pad: u16,    // +0x1a
    pub position: [f32; 3], // +0x1c
    pub normal: [f32; 3],   // +0x28
    pub distance: f32,      // +0x34
    pub pad_to_16: u32,     // +0x38
    pub _tail_pad: u32,     // +0x3c
}
const _: () = assert!(std::mem::size_of::<PxSweepHit>() == 0x40);
const _: () = assert!(std::mem::offset_of!(PxSweepHit, flags) == 0x18);
const _: () = assert!(std::mem::offset_of!(PxSweepHit, position) == 0x1c);
const _: () = assert!(std::mem::offset_of!(PxSweepHit, normal) == 0x28);
const _: () = assert!(std::mem::offset_of!(PxSweepHit, distance) == 0x34);

// ---------------------------------------------------------------------------
// 3D Analytic Geometry Math Helpers
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
enum AnalyticGeom {
    Sphere { radius: f32 },
    Plane,
    Capsule { radius: f32, half_height: f32 },
    Box { half_extents: [f32; 3] },
}

impl AnalyticGeom {
    fn type_ord(self) -> i32 {
        match self {
            AnalyticGeom::Sphere { .. } => PX_GEOM_SPHERE,
            AnalyticGeom::Plane => PX_GEOM_PLANE,
            AnalyticGeom::Capsule { .. } => PX_GEOM_CAPSULE,
            AnalyticGeom::Box { .. } => PX_GEOM_BOX,
        }
    }
}

unsafe fn read_geometry(geom: *const c_void) -> Result<AnalyticGeom, i32> {
    if geom.is_null() {
        return Err(-1);
    }
    let gtype = std::ptr::read_unaligned(geom as *const i32);
    let base = geom as *const u8;
    match gtype {
        PX_GEOM_SPHERE => {
            let radius = std::ptr::read_unaligned(base.add(4) as *const f32);
            Ok(AnalyticGeom::Sphere { radius })
        }
        PX_GEOM_PLANE => Ok(AnalyticGeom::Plane),
        PX_GEOM_CAPSULE => {
            let radius = std::ptr::read_unaligned(base.add(4) as *const f32);
            let half_height = std::ptr::read_unaligned(base.add(8) as *const f32);
            Ok(AnalyticGeom::Capsule {
                radius,
                half_height,
            })
        }
        PX_GEOM_BOX => {
            let half_extents = std::ptr::read_unaligned(base.add(4) as *const [f32; 3]);
            Ok(AnalyticGeom::Box { half_extents })
        }
        other => Err(other),
    }
}

unsafe fn read_pose(pose: *const PxTransform) -> PxTransform {
    if pose.is_null() {
        PxTransform {
            q: [0.0, 0.0, 0.0, 1.0],
            p: [0.0, 0.0, 0.0],
        }
    } else {
        std::ptr::read_unaligned(pose)
    }
}

fn geom_type_name(t: i32) -> &'static str {
    match t {
        PX_GEOM_SPHERE => "eSPHERE",
        PX_GEOM_PLANE => "ePLANE",
        PX_GEOM_CAPSULE => "eCAPSULE",
        PX_GEOM_BOX => "eBOX",
        PX_GEOM_CONVEX_MESH => "eCONVEXMESH",
        PX_GEOM_TRIANGLE_MESH => "eTRIANGLEMESH",
        PX_GEOM_HEIGHTFIELD => "eHEIGHTFIELD",
        _ => "eINVALID",
    }
}

#[inline]
fn v3_add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline]
fn v3_sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline]
fn v3_neg(a: [f32; 3]) -> [f32; 3] {
    [-a[0], -a[1], -a[2]]
}

#[inline]
fn v3_scale(a: [f32; 3], s: f32) -> [f32; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

#[inline]
fn v3_dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn v3_cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline]
fn v3_len_sq(a: [f32; 3]) -> f32 {
    v3_dot(a, a)
}

#[inline]
fn v3_len(a: [f32; 3]) -> f32 {
    v3_len_sq(a).sqrt()
}

#[inline]
fn v3_norm_or(a: [f32; 3], fallback: [f32; 3]) -> [f32; 3] {
    let l2 = v3_len_sq(a);
    if l2 > 1e-12 {
        v3_scale(a, 1.0 / l2.sqrt())
    } else {
        fallback
    }
}

#[inline]
fn quat_basis(q: [f32; 4]) -> [[f32; 3]; 3] {
    let [x, y, z, w] = q;
    let x2 = x + x;
    let y2 = y + y;
    let z2 = z + z;
    let xx = x2 * x;
    let yy = y2 * y;
    let zz = z2 * z;
    let xy = x2 * y;
    let xz = x2 * z;
    let xw = x2 * w;
    let yz = y2 * z;
    let yw = y2 * w;
    let zw = z2 * w;
    [
        [1.0 - yy - zz, xy + zw, xz - yw],
        [xy - zw, 1.0 - xx - zz, yz + xw],
        [xz + yw, yz - xw, 1.0 - xx - yy],
    ]
}

#[inline]
fn quat_rotate(q: [f32; 4], v: [f32; 3]) -> [f32; 3] {
    let b = quat_basis(q);
    [
        b[0][0] * v[0] + b[1][0] * v[1] + b[2][0] * v[2],
        b[0][1] * v[0] + b[1][1] * v[1] + b[2][1] * v[2],
        b[0][2] * v[0] + b[1][2] * v[1] + b[2][2] * v[2],
    ]
}

#[inline]
fn quat_rotate_inv(q: [f32; 4], v: [f32; 3]) -> [f32; 3] {
    let b = quat_basis(q);
    [v3_dot(b[0], v), v3_dot(b[1], v), v3_dot(b[2], v)]
}

#[inline]
fn pose_transform(pose: PxTransform, local: [f32; 3]) -> [f32; 3] {
    v3_add(quat_rotate(pose.q, local), pose.p)
}

#[inline]
fn pose_transform_inv(pose: PxTransform, world: [f32; 3]) -> [f32; 3] {
    quat_rotate_inv(pose.q, v3_sub(world, pose.p))
}

#[derive(Clone, Copy)]
struct WorldPlane {
    n: [f32; 3],
    d: f32,
}

impl WorldPlane {
    #[inline]
    fn from_pose(pose: PxTransform) -> Self {
        let n = v3_norm_or(quat_basis(pose.q)[0], [1.0, 0.0, 0.0]);
        Self {
            n,
            d: -v3_dot(pose.p, n),
        }
    }

    #[inline]
    fn distance(self, pt: [f32; 3]) -> f32 {
        v3_dot(pt, self.n) + self.d
    }
}

#[inline]
fn capsule_segment(pose: PxTransform, half_height: f32) -> ([f32; 3], [f32; 3]) {
    let axis = v3_scale(quat_basis(pose.q)[0], half_height);
    (v3_add(pose.p, axis), v3_sub(pose.p, axis))
}

#[inline]
fn obb_projected_radius(axes: &[[f32; 3]; 3], he: [f32; 3], dir: [f32; 3]) -> f32 {
    v3_dot(axes[0], dir).abs() * he[0]
        + v3_dot(axes[1], dir).abs() * he[1]
        + v3_dot(axes[2], dir).abs() * he[2]
}

#[inline]
fn obb_corners(pose: PxTransform, he: [f32; 3]) -> [[f32; 3]; 8] {
    let b = quat_basis(pose.q);
    let ax = v3_scale(b[0], he[0]);
    let ay = v3_scale(b[1], he[1]);
    let az = v3_scale(b[2], he[2]);
    let mut pts = [[0.0f32; 3]; 8];
    let mut i = 0usize;
    for &sx in &[-1.0f32, 1.0] {
        for &sy in &[-1.0f32, 1.0] {
            for &sz in &[-1.0f32, 1.0] {
                pts[i] = v3_add(
                    pose.p,
                    v3_add(
                        v3_add(v3_scale(ax, sx), v3_scale(ay, sy)),
                        v3_scale(az, sz),
                    ),
                );
                i += 1;
            }
        }
    }
    pts
}

/// Squared distance from `point` to segment `[p0, p1]`, plus parameter `t in [0, 1]` and closest point.
fn dist_point_segment_sq(p0: [f32; 3], p1: [f32; 3], point: [f32; 3]) -> (f32, f32, [f32; 3]) {
    let dir = v3_sub(p1, p0);
    let diff = v3_sub(point, p0);
    let sqr_len = v3_len_sq(dir);
    let t = if sqr_len <= 1e-12 {
        0.0
    } else {
        (v3_dot(diff, dir) / sqr_len).clamp(0.0, 1.0)
    };
    let closest = v3_add(p0, v3_scale(dir, t));
    (v3_len_sq(v3_sub(point, closest)), t, closest)
}

/// Squared distance from `point` to OBB `(pose, he)`, plus closest point in box-local space.
fn dist_point_obb_sq(point: [f32; 3], pose: PxTransform, he: [f32; 3]) -> (f32, [f32; 3]) {
    let local = pose_transform_inv(pose, point);
    let mut clamped = local;
    let mut sq = 0.0f32;
    for i in 0..3 {
        if clamped[i] < -he[i] {
            let d = clamped[i] + he[i];
            sq += d * d;
            clamped[i] = -he[i];
        } else if clamped[i] > he[i] {
            let d = clamped[i] - he[i];
            sq += d * d;
            clamped[i] = he[i];
        }
    }
    (sq, clamped)
}

/// Exact analytical squared distance and closest points between segments `[p1, q1]` and `[p2, q2]`.
fn dist_segment_segment_sq(
    p1: [f32; 3],
    q1: [f32; 3],
    p2: [f32; 3],
    q2: [f32; 3],
) -> (f32, [f32; 3], [f32; 3]) {
    let d1 = v3_sub(q1, p1);
    let d2 = v3_sub(q2, p2);
    let r = v3_sub(p1, p2);
    let a = v3_dot(d1, d1);
    let e = v3_dot(d2, d2);
    let f = v3_dot(d2, r);

    let (s, t) = if a <= 1e-12 && e <= 1e-12 {
        (0.0, 0.0)
    } else if a <= 1e-12 {
        (0.0, (f / e).clamp(0.0, 1.0))
    } else {
        let c = v3_dot(d1, r);
        if e <= 1e-12 {
            ((-c / a).clamp(0.0, 1.0), 0.0)
        } else {
            let b = v3_dot(d1, d2);
            let denom = a * e - b * b;
            let mut s = if denom.abs() > 1e-12 {
                ((b * f - c * e) / denom).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let mut t = (b * s + f) / e;
            if t < 0.0 {
                t = 0.0;
                s = (-c / a).clamp(0.0, 1.0);
            } else if t > 1.0 {
                t = 1.0;
                s = ((b - c) / a).clamp(0.0, 1.0);
            }
            (s, t)
        }
    };

    let c1 = v3_add(p1, v3_scale(d1, s));
    let c2 = v3_add(p2, v3_scale(d2, t));
    (v3_len_sq(v3_sub(c1, c2)), c1, c2)
}

/// Exact analytical squared distance between world segment `[p0, p1]` and OBB `(pose, he)`.
///
/// Exploits the fact that `f(t) = dist_point_aabb_sq(a + t*d, he)` is piecewise quadratic on
/// `[0, 1]` with breakpoints strictly where `a[k] + t*d[k] == ±he[k]` (at most 6 breakpoints).
fn dist_segment_obb_sq(
    p0: [f32; 3],
    p1: [f32; 3],
    pose: PxTransform,
    he: [f32; 3],
) -> (f32, [f32; 3], [f32; 3]) {
    let a = pose_transform_inv(pose, p0);
    let b = pose_transform_inv(pose, p1);
    let d = v3_sub(b, a);

    // Collect breakpoints in [0, 1].
    let mut cuts = [0.0f32; 8];
    cuts[0] = 0.0;
    cuts[1] = 1.0;
    let mut n = 2usize;
    for k in 0..3 {
        if d[k].abs() > 1e-12 {
            for &bound in &[-he[k], he[k]] {
                let t = (bound - a[k]) / d[k];
                if t > 0.0 && t < 1.0 && n < 8 {
                    cuts[n] = t;
                    n += 1;
                }
            }
        }
    }
    cuts[..n].sort_by(|x, y| x.total_cmp(y));

    let eval_at = |t: f32| -> (f32, [f32; 3], [f32; 3]) {
        let p = v3_add(a, v3_scale(d, t));
        let mut clamped = p;
        let mut sq = 0.0f32;
        for k in 0..3 {
            if clamped[k] < -he[k] {
                let diff = clamped[k] + he[k];
                sq += diff * diff;
                clamped[k] = -he[k];
            } else if clamped[k] > he[k] {
                let diff = clamped[k] - he[k];
                sq += diff * diff;
                clamped[k] = he[k];
            }
        }
        (sq, p, clamped)
    };

    let (mut best_sq, mut best_seg_l, mut best_box_l) = eval_at(0.0);
    for i in 0..(n - 1) {
        let t0 = cuts[i];
        let t1 = cuts[i + 1];
        if t1 - t0 <= 1e-12 {
            continue;
        }
        let tm = 0.5 * (t0 + t1);
        let pm = v3_add(a, v3_scale(d, tm));
        let mut qa = 0.0f32;
        let mut qb = 0.0f32;
        for k in 0..3 {
            let ck = if pm[k] < -he[k] {
                -he[k]
            } else if pm[k] > he[k] {
                he[k]
            } else {
                continue;
            };
            qa += d[k] * d[k];
            qb += (a[k] - ck) * d[k];
        }
        let cand_ts = if qa > 1e-12 {
            [t0, (-qb / qa).clamp(t0, t1), t1]
        } else {
            [t0, tm, t1]
        };
        for t in cand_ts {
            let (sq, sp, bp) = eval_at(t);
            if sq < best_sq {
                best_sq = sq;
                best_seg_l = sp;
                best_box_l = bp;
            }
        }
    }

    (
        best_sq,
        pose_transform(pose, best_seg_l),
        pose_transform(pose, best_box_l),
    )
}

// ---------------------------------------------------------------------------
// Analytic Raycast Helpers (matching GuRaycastTests.cpp)
// ---------------------------------------------------------------------------

fn ray_sphere(
    origin: [f32; 3],
    dir: [f32; 3],
    max_dist: f32,
    center: [f32; 3],
    radius: f32,
) -> Option<(f32, [f32; 3])> {
    let offset = v3_sub(center, origin);
    let off2 = v3_len_sq(offset);
    let r2 = radius * radius;
    if off2 <= r2 {
        return Some((0.0, origin));
    }
    let ray_dist = v3_dot(dir, offset);
    if ray_dist <= 0.0 || (ray_dist - max_dist) > radius {
        return None;
    }
    let disc = r2 - (off2 - ray_dist * ray_dist);
    if disc < 0.0 {
        return None;
    }
    let t = (ray_dist - disc.sqrt()).max(0.0);
    if t > max_dist {
        return None;
    }
    Some((t, v3_add(origin, v3_scale(dir, t))))
}

/// Matches PhysX `Gu::rayAABBIntersect2` (`GuIntersectionRayBox.cpp`).
/// Returns `Some((t, local_impact, hit_axis_0_1_2))` where `t == 0.0` indicates origin inside box.
fn ray_aabb(
    min_b: [f32; 3],
    max_b: [f32; 3],
    origin: [f32; 3],
    dir: [f32; 3],
) -> Option<(f32, [f32; 3], usize)> {
    const EPS: f32 = 1e-5;
    let mut inside = true;
    let mut max_t = [-1.0f32; 3];
    let mut coord = [0.0f32; 3];

    for i in 0..3 {
        if origin[i] < min_b[i] {
            coord[i] = min_b[i];
            inside = false;
            if dir[i] != 0.0 {
                max_t[i] = (min_b[i] - origin[i]) / dir[i];
            }
        } else if origin[i] > max_b[i] {
            coord[i] = max_b[i];
            inside = false;
            if dir[i] != 0.0 {
                max_t[i] = (max_b[i] - origin[i]) / dir[i];
            }
        }
    }

    if inside {
        return Some((0.0, origin, 0));
    }

    let mut which = 0usize;
    if max_t[1] > max_t[which] {
        which = 1;
    }
    if max_t[2] > max_t[which] {
        which = 2;
    }
    let t = max_t[which];
    if t < 0.0 {
        return None;
    }

    for i in 0..3 {
        if i != which {
            coord[i] = origin[i] + t * dir[i];
            if coord[i] < min_b[i] - EPS || coord[i] > max_b[i] + EPS {
                return None;
            }
        }
    }
    Some((t, coord, which))
}

/// Ray vs finite capsule `(p0, p1, radius)`. Returns smallest `t in [0, max_dist]`.
fn ray_capsule(
    origin: [f32; 3],
    dir: [f32; 3],
    max_dist: f32,
    p0: [f32; 3],
    p1: [f32; 3],
    radius: f32,
) -> Option<f32> {
    let (start_d2, _, _) = dist_point_segment_sq(p0, p1, origin);
    if start_d2 <= radius * radius {
        return Some(0.0);
    }

    let axis = v3_sub(p1, p0);
    let axis_len = v3_len(axis);
    if axis_len <= 1e-6 {
        return ray_sphere(origin, dir, max_dist, p0, radius).map(|(t, _)| t);
    }
    let w = v3_scale(axis, 1.0 / axis_len);

    // Build orthonormal basis (u, v, w) around capsule axis.
    let u = if w[0].abs() >= w[1].abs() {
        let inv = 1.0 / (w[0] * w[0] + w[2] * w[2]).sqrt();
        [-w[2] * inv, 0.0, w[0] * inv]
    } else {
        let inv = 1.0 / (w[1] * w[1] + w[2] * w[2]).sqrt();
        [0.0, w[2] * inv, -w[1] * inv]
    };
    let v = v3_norm_or(v3_cross(w, u), [0.0, 1.0, 0.0]);

    let d = [v3_dot(u, dir), v3_dot(v, dir), v3_dot(w, dir)];
    let diff = v3_sub(origin, p0);
    let p = [v3_dot(u, diff), v3_dot(v, diff), v3_dot(w, diff)];
    let r2 = radius * radius;

    let mut best_t: Option<f32> = None;
    let mut consider = |t: f32| {
        if t >= 0.0 && t <= max_dist && best_t.map_or(true, |b| t < b) {
            best_t = Some(t);
        }
    };

    // Infinite cylinder wall: (p.x + t*d.x)^2 + (p.y + t*d.y)^2 = r^2, z in [0, axis_len].
    let qa = d[0] * d[0] + d[1] * d[1];
    let qb = p[0] * d[0] + p[1] * d[1];
    let qc = p[0] * p[0] + p[1] * p[1] - r2;
    if qa > 1e-12 {
        let disc = qb * qb - qa * qc;
        if disc >= 0.0 {
            let root = disc.sqrt();
            let inv_a = 1.0 / qa;
            for t in [(-qb - root) * inv_a, (-qb + root) * inv_a] {
                let z = p[2] + t * d[2];
                if z >= -1e-4 && z <= axis_len + 1e-4 {
                    consider(t);
                }
            }
        }
    }

    // Endcap hemispheres at p0 and p1.
    if let Some((t, _)) = ray_sphere(origin, dir, max_dist, p0, radius) {
        consider(t);
    }
    if let Some((t, _)) = ray_sphere(origin, dir, max_dist, p1, radius) {
        consider(t);
    }

    best_t
}

// ---------------------------------------------------------------------------
// Analytic Overlap & MTD Helpers (matching GuOverlapTests.cpp & GuMTD.cpp)
// ---------------------------------------------------------------------------

#[inline]
fn project_obb(axes: &[[f32; 3]; 3], center: [f32; 3], he: [f32; 3], axis: [f32; 3]) -> (f32, f32) {
    let c = v3_dot(center, axis);
    let r = obb_projected_radius(axes, he, axis);
    (c - r, c + r)
}

fn overlap_obb_obb(
    pose0: PxTransform,
    e0: [f32; 3],
    pose1: PxTransform,
    e1: [f32; 3],
) -> bool {
    let r0 = quat_basis(pose0.q);
    let r1 = quat_basis(pose1.q);
    let v = v3_sub(pose1.p, pose0.p);
    let t_vec = [v3_dot(v, r0[0]), v3_dot(v, r0[1]), v3_dot(v, r0[2])];

    let mut rm = [[0.0f32; 3]; 3];
    let mut fr = [[0.0f32; 3]; 3];
    for i in 0..3 {
        for k in 0..3 {
            rm[i][k] = v3_dot(r0[i], r1[k]);
            fr[i][k] = 1e-6 + rm[i][k].abs();
        }
    }

    for i in 0..3 {
        let ra = e0[i];
        let rb = e1[0] * fr[i][0] + e1[1] * fr[i][1] + e1[2] * fr[i][2];
        if t_vec[i].abs() > ra + rb {
            return false;
        }
    }
    for k in 0..3 {
        let ra = e0[0] * fr[0][k] + e0[1] * fr[1][k] + e0[2] * fr[2][k];
        let rb = e1[k];
        let t = (t_vec[0] * rm[0][k] + t_vec[1] * rm[1][k] + t_vec[2] * rm[2][k]).abs();
        if t > ra + rb {
            return false;
        }
    }

    // 9 edge cross-product axes A_i x B_k.
    for i in 0..3 {
        let i1 = (i + 1) % 3;
        let i2 = (i + 2) % 3;
        for k in 0..3 {
            let k1 = (k + 1) % 3;
            let k2 = (k + 2) % 3;
            let ra = e0[i1] * fr[i2][k] + e0[i2] * fr[i1][k];
            let rb = e1[k1] * fr[i][k2] + e1[k2] * fr[i][k1];
            let t = (t_vec[i2] * rm[i1][k] - t_vec[i1] * rm[i2][k]).abs();
            if t > ra + rb {
                return false;
            }
        }
    }
    true
}

fn overlap_analytic(
    g0: AnalyticGeom,
    p0: PxTransform,
    g1: AnalyticGeom,
    p1: PxTransform,
) -> bool {
    if g0.type_ord() > g1.type_ord() {
        return overlap_analytic(g1, p1, g0, p0);
    }
    match (g0, g1) {
        (AnalyticGeom::Sphere { radius: r0 }, AnalyticGeom::Sphere { radius: r1 }) => {
            let r = r0 + r1;
            v3_len_sq(v3_sub(p1.p, p0.p)) <= r * r
        }
        (AnalyticGeom::Sphere { radius }, AnalyticGeom::Plane) => {
            WorldPlane::from_pose(p1).distance(p0.p) <= radius
        }
        (
            AnalyticGeom::Sphere { radius: r0 },
            AnalyticGeom::Capsule {
                radius: r1,
                half_height,
            },
        ) => {
            let (c0, c1) = capsule_segment(p1, half_height);
            let (d2, _, _) = dist_point_segment_sq(c0, c1, p0.p);
            let r = r0 + r1;
            d2 <= r * r
        }
        (AnalyticGeom::Sphere { radius }, AnalyticGeom::Box { half_extents }) => {
            let (d2, _) = dist_point_obb_sq(p0.p, p1, half_extents);
            d2 <= radius * radius
        }
        (AnalyticGeom::Plane, AnalyticGeom::Plane) => false,
        (
            AnalyticGeom::Plane,
            AnalyticGeom::Capsule {
                radius,
                half_height,
            },
        ) => {
            let pl = WorldPlane::from_pose(p0);
            let (c0, c1) = capsule_segment(p1, half_height);
            pl.distance(c0) <= radius || pl.distance(c1) <= radius
        }
        (AnalyticGeom::Plane, AnalyticGeom::Box { half_extents }) => {
            let pl = WorldPlane::from_pose(p0);
            let axes = quat_basis(p1.q);
            pl.distance(p1.p) <= obb_projected_radius(&axes, half_extents, pl.n)
        }
        (
            AnalyticGeom::Capsule {
                radius: r0,
                half_height: h0,
            },
            AnalyticGeom::Capsule {
                radius: r1,
                half_height: h1,
            },
        ) => {
            let (a0, a1) = capsule_segment(p0, h0);
            let (b0, b1) = capsule_segment(p1, h1);
            let (d2, _, _) = dist_segment_segment_sq(a0, a1, b0, b1);
            let r = r0 + r1;
            d2 <= r * r
        }
        (
            AnalyticGeom::Capsule {
                radius,
                half_height,
            },
            AnalyticGeom::Box { half_extents },
        ) => {
            let (a0, a1) = capsule_segment(p0, half_height);
            let (d2, _, _) = dist_segment_obb_sq(a0, a1, p1, half_extents);
            d2 <= radius * radius
        }
        (
            AnalyticGeom::Box { half_extents: e0 },
            AnalyticGeom::Box { half_extents: e1 },
        ) => overlap_obb_obb(p0, e0, p1, e1),
        _ => false,
    }
}

/// Computes `(mtd_direction, depth)` pushing `g0` out of `g1` (matches `GuMTD.cpp`).
fn compute_penetration_analytic(
    g0: AnalyticGeom,
    p0: PxTransform,
    g1: AnalyticGeom,
    p1: PxTransform,
) -> Option<([f32; 3], f32)> {
    if g0.type_ord() > g1.type_ord() {
        let (dir, depth) = compute_penetration_analytic(g1, p1, g0, p0)?;
        return Some((v3_neg(dir), depth));
    }

    match (g0, g1) {
        (AnalyticGeom::Sphere { radius: r0 }, AnalyticGeom::Sphere { radius: r1 }) => {
            let delta = v3_sub(p0.p, p1.p);
            let d2 = v3_len_sq(delta);
            let rsum = r0 + r1;
            if d2 > rsum * rsum {
                return None;
            }
            let d = d2.sqrt();
            let dir = v3_norm_or(delta, [1.0, 0.0, 0.0]);
            Some((dir, (rsum - d).max(0.0)))
        }
        (AnalyticGeom::Sphere { radius }, AnalyticGeom::Plane) => {
            let pl = WorldPlane::from_pose(p1);
            let d = pl.distance(p0.p);
            if d > radius {
                return None;
            }
            Some((pl.n, (radius - d).max(0.0)))
        }
        (
            AnalyticGeom::Sphere { radius: r0 },
            AnalyticGeom::Capsule {
                radius: r1,
                half_height,
            },
        ) => {
            let (c0, c1) = capsule_segment(p1, half_height);
            let (d2, _, closest) = dist_point_segment_sq(c0, c1, p0.p);
            let rsum = r0 + r1;
            if d2 > rsum * rsum {
                return None;
            }
            let normal = v3_sub(p0.p, closest);
            let d = d2.sqrt();
            let dir = v3_norm_or(normal, [1.0, 0.0, 0.0]);
            Some((dir, (rsum - d).max(0.0)))
        }
        (AnalyticGeom::Sphere { radius }, AnalyticGeom::Box { half_extents: he }) => {
            let delta = v3_sub(p0.p, p1.p);
            let local = quat_rotate_inv(p1.q, delta);
            let mut clamped = local;
            let mut outside = false;
            for i in 0..3 {
                if clamped[i] < -he[i] {
                    outside = true;
                    clamped[i] = -he[i];
                } else if clamped[i] > he[i] {
                    outside = true;
                    clamped[i] = he[i];
                }
            }
            if outside {
                let pt_rel = quat_rotate(p1.q, clamped);
                let normal = v3_sub(delta, pt_rel);
                let d2 = v3_len_sq(normal);
                if d2 > radius * radius {
                    return None;
                }
                let d = d2.sqrt();
                let dir = v3_norm_or(normal, [1.0, 0.0, 0.0]);
                Some((dir, (radius - d).max(0.0)))
            } else {
                // Sphere center is inside box: push out through nearest box face.
                let mut best_ax = 0usize;
                let mut best_dist = he[0] - local[0].abs();
                for i in 1..3 {
                    let dist = he[i] - local[i].abs();
                    if dist < best_dist {
                        best_dist = dist;
                        best_ax = i;
                    }
                }
                let mut loc_norm = [0.0f32; 3];
                loc_norm[best_ax] = if local[best_ax] >= 0.0 { 1.0 } else { -1.0 };
                let dir = quat_rotate(p1.q, loc_norm);
                Some((dir, (best_dist + radius).max(0.0)))
            }
        }
        (AnalyticGeom::Plane, AnalyticGeom::Plane) => None,
        (
            AnalyticGeom::Plane,
            AnalyticGeom::Capsule {
                radius,
                half_height,
            },
        ) => {
            let pl = WorldPlane::from_pose(p0);
            let (c0, c1) = capsule_segment(p1, half_height);
            let dmin = pl.distance(c0).min(pl.distance(c1)) - radius;
            if dmin > 0.0 {
                return None;
            }
            Some((v3_neg(pl.n), (-dmin).max(0.0)))
        }
        (AnalyticGeom::Plane, AnalyticGeom::Box { half_extents }) => {
            let pl = WorldPlane::from_pose(p0);
            let axes = quat_basis(p1.q);
            let dmin = pl.distance(p1.p) - obb_projected_radius(&axes, half_extents, pl.n);
            if dmin > 0.0 {
                return None;
            }
            Some((v3_neg(pl.n), (-dmin).max(0.0)))
        }
        (
            AnalyticGeom::Capsule {
                radius: r0,
                half_height: h0,
            },
            AnalyticGeom::Capsule {
                radius: r1,
                half_height: h1,
            },
        ) => {
            let (a0, a1) = capsule_segment(p0, h0);
            let (b0, b1) = capsule_segment(p1, h1);
            let (d2, ca, cb) = dist_segment_segment_sq(a0, a1, b0, b1);
            let rsum = r0 + r1;
            if d2 > rsum * rsum {
                return None;
            }
            let d = d2.sqrt();
            let dir = v3_norm_or(v3_sub(ca, cb), [1.0, 0.0, 0.0]);
            Some((dir, (rsum - d).max(0.0)))
        }
        (
            AnalyticGeom::Capsule {
                radius,
                half_height,
            },
            AnalyticGeom::Box { half_extents: he },
        ) => {
            let (c0, c1) = capsule_segment(p0, half_height);
            let (d2, on_seg, on_box) = dist_segment_obb_sq(c0, c1, p1, he);
            if d2 > radius * radius {
                return None;
            }
            if d2 > 1e-12 {
                let d = d2.sqrt();
                let dir = v3_norm_or(v3_sub(on_seg, on_box), [1.0, 0.0, 0.0]);
                return Some((dir, (radius - d).max(0.0)));
            }
            // Segment intersects the OBB interior: SAT over box normals and capsule-axis cross products.
            let axes = quat_basis(p1.q);
            let cap_axis = v3_norm_or(v3_sub(c1, c0), axes[0]);
            let mut best_depth = f32::MAX;
            let mut best_axis = axes[0];

            let mut test_axis = |axis: [f32; 3]| -> bool {
                let mut min0 = v3_dot(c0, axis);
                let mut max0 = v3_dot(c1, axis);
                if min0 > max0 {
                    std::mem::swap(&mut min0, &mut max0);
                }
                min0 -= radius;
                max0 += radius;
                let (min1, max1) = project_obb(&axes, p1.p, he, axis);
                if max0 < min1 || max1 < min0 {
                    return false;
                }
                let depth = (max0 - min1).min(max1 - min0);
                if depth < best_depth {
                    best_depth = depth;
                    best_axis = axis;
                }
                true
            };

            for ax in axes {
                if !test_axis(ax) {
                    return None;
                }
            }
            for ax in axes {
                let cross = v3_cross(cap_axis, ax);
                let cl2 = v3_len_sq(cross);
                if cl2 > 1e-10 {
                    if !test_axis(v3_scale(cross, 1.0 / cl2.sqrt())) {
                        return None;
                    }
                }
            }
            if v3_dot(best_axis, v3_sub(p0.p, p1.p)) < 0.0 {
                best_axis = v3_neg(best_axis);
            }
            Some((best_axis, best_depth.max(0.0)))
        }
        (
            AnalyticGeom::Box { half_extents: e0 },
            AnalyticGeom::Box { half_extents: e1 },
        ) => {
            let axes0 = quat_basis(p0.q);
            let axes1 = quat_basis(p1.q);
            let mut best_depth = f32::MAX;
            let mut best_axis = axes0[0];

            let mut test_axis = |axis: [f32; 3]| -> bool {
                let (min0, max0) = project_obb(&axes0, p0.p, e0, axis);
                let (min1, max1) = project_obb(&axes1, p1.p, e1, axis);
                if max0 < min1 || max1 < min0 {
                    return false;
                }
                let depth = (max0 - min1).min(max1 - min0);
                if depth < best_depth {
                    best_depth = depth;
                    best_axis = axis;
                }
                true
            };

            for ax in axes0 {
                if !test_axis(ax) {
                    return None;
                }
            }
            for ax in axes1 {
                if !test_axis(ax) {
                    return None;
                }
            }
            for a in axes0 {
                for b in axes1 {
                    let cross = v3_cross(a, b);
                    let cl2 = v3_len_sq(cross);
                    if cl2 > 1e-10 {
                        if !test_axis(v3_scale(cross, 1.0 / cl2.sqrt())) {
                            return None;
                        }
                    }
                }
            }
            // Orient MTD so it pushes box0 out of box1.
            if v3_dot(best_axis, v3_sub(p0.p, p1.p)) < 0.0 {
                best_axis = v3_neg(best_axis);
            }
            Some((best_axis, best_depth.max(0.0)))
        }
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Analytic Sweep Helpers (exact Plane/Sphere/Capsule + GJK Raycast for Convex pairs)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct SupportShape {
    geom: AnalyticGeom,
    pose: PxTransform,
    inflation: f32,
}

impl SupportShape {
    fn support(&self, dir: [f32; 3]) -> [f32; 3] {
        let udir = v3_norm_or(dir, [1.0, 0.0, 0.0]);
        match self.geom {
            AnalyticGeom::Sphere { radius } => {
                v3_add(self.pose.p, v3_scale(udir, radius + self.inflation))
            }
            AnalyticGeom::Capsule {
                radius,
                half_height,
            } => {
                let (p0, p1) = capsule_segment(self.pose, half_height);
                let end = if v3_dot(p0, dir) >= v3_dot(p1, dir) {
                    p0
                } else {
                    p1
                };
                v3_add(end, v3_scale(udir, radius + self.inflation))
            }
            AnalyticGeom::Box { half_extents: he } => {
                let axes = quat_basis(self.pose.q);
                let sx = if v3_dot(axes[0], dir) >= 0.0 {
                    he[0]
                } else {
                    -he[0]
                };
                let sy = if v3_dot(axes[1], dir) >= 0.0 {
                    he[1]
                } else {
                    -he[1]
                };
                let sz = if v3_dot(axes[2], dir) >= 0.0 {
                    he[2]
                } else {
                    -he[2]
                };
                let corner = v3_add(
                    self.pose.p,
                    v3_add(
                        v3_add(v3_scale(axes[0], sx), v3_scale(axes[1], sy)),
                        v3_scale(axes[2], sz),
                    ),
                );
                if self.inflation > 0.0 {
                    v3_add(corner, v3_scale(udir, self.inflation))
                } else {
                    corner
                }
            }
            AnalyticGeom::Plane => self.pose.p,
        }
    }
}

#[derive(Clone, Copy)]
struct SimplexVert {
    c: [f32; 3], // b - a
    b: [f32; 3], // support on static target geom1
}

/// Closest point on triangle `(a, b, c)` to point `p` with barycentric weights.
fn closest_point_on_triangle(
    a: [f32; 3],
    b: [f32; 3],
    c: [f32; 3],
    p: [f32; 3],
) -> ([f32; 3], [f32; 3]) {
    let ab = v3_sub(b, a);
    let ac = v3_sub(c, a);
    let ap = v3_sub(p, a);
    let d1 = v3_dot(ab, ap);
    let d2 = v3_dot(ac, ap);
    if d1 <= 0.0 && d2 <= 0.0 {
        return (a, [1.0, 0.0, 0.0]);
    }
    let bp = v3_sub(p, b);
    let d3 = v3_dot(ab, bp);
    let d4 = v3_dot(ac, bp);
    if d3 >= 0.0 && d4 <= d3 {
        return (b, [0.0, 1.0, 0.0]);
    }
    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        let v = d1 / (d1 - d3);
        return (v3_add(a, v3_scale(ab, v)), [1.0 - v, v, 0.0]);
    }
    let cp = v3_sub(p, c);
    let d5 = v3_dot(ab, cp);
    let d6 = v3_dot(ac, cp);
    if d6 >= 0.0 && d5 <= d6 {
        return (c, [0.0, 0.0, 1.0]);
    }
    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        let w = d2 / (d2 - d6);
        return (v3_add(a, v3_scale(ac, w)), [1.0 - w, 0.0, w]);
    }
    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && (d4 - d3) >= 0.0 && (d5 - d6) >= 0.0 {
        let w = (d4 - d3) / ((d4 - d3) + (d5 - d6));
        return (
            v3_add(b, v3_scale(v3_sub(c, b), w)),
            [0.0, 1.0 - w, w],
        );
    }
    let denom = 1.0 / (va + vb + vc);
    let v = vb * denom;
    let w = vc * denom;
    let u = 1.0 - v - w;
    (
        v3_add(a, v3_add(v3_scale(ab, v), v3_scale(ac, w))),
        [u, v, w],
    )
}

/// Reduce simplex `verts[..*len]` to minimal sub-simplex closest to `x`, returning `(closest_c, closest_b)`.
fn reduce_simplex(verts: &mut [SimplexVert; 4], len: &mut usize, x: [f32; 3]) -> ([f32; 3], [f32; 3]) {
    match *len {
        1 => (verts[0].c, verts[0].b),
        2 => {
            let (_, t, closest) = dist_point_segment_sq(verts[0].c, verts[1].c, x);
            let b = v3_add(v3_scale(verts[0].b, 1.0 - t), v3_scale(verts[1].b, t));
            if t <= 0.0 {
                *len = 1;
            } else if t >= 1.0 {
                verts[0] = verts[1];
                *len = 1;
            }
            (closest, b)
        }
        3 => {
            let (closest, w) = closest_point_on_triangle(verts[0].c, verts[1].c, verts[2].c, x);
            let b = v3_add(
                v3_add(v3_scale(verts[0].b, w[0]), v3_scale(verts[1].b, w[1])),
                v3_scale(verts[2].b, w[2]),
            );
            let mut out = [verts[0]; 4];
            let mut n = 0usize;
            for i in 0..3 {
                if w[i] > 0.0 {
                    out[n] = verts[i];
                    n += 1;
                }
            }
            if n > 0 {
                verts[..n].copy_from_slice(&out[..n]);
                *len = n;
            }
            (closest, b)
        }
        4 => {
            // Check if x is inside tetrahedron; otherwise pick closest of the 4 triangular faces.
            let faces = [[0, 1, 2], [0, 3, 1], [0, 2, 3], [1, 3, 2]];
            let mut inside = true;
            let mut best_d2 = f32::MAX;
            let mut best_c = verts[0].c;
            let mut best_b = verts[0].b;
            let mut best_face = [0usize; 3];
            let mut best_w = [1.0f32, 0.0, 0.0];

            for f in faces {
                let opp = 6 - f[0] - f[1] - f[2];
                let a = verts[f[0]].c;
                let b = verts[f[1]].c;
                let c = verts[f[2]].c;
                let d = verts[opp].c;
                let n = v3_cross(v3_sub(b, a), v3_sub(c, a));
                let sign_d = v3_dot(v3_sub(d, a), n);
                let sign_x = v3_dot(v3_sub(x, a), n);
                if sign_d.abs() <= 1e-10 || sign_d * sign_x < 0.0 {
                    inside = false;
                }
                let (cp, w) = closest_point_on_triangle(a, b, c, x);
                let d2 = v3_len_sq(v3_sub(x, cp));
                if d2 < best_d2 {
                    best_d2 = d2;
                    best_c = cp;
                    best_b = v3_add(
                        v3_add(
                            v3_scale(verts[f[0]].b, w[0]),
                            v3_scale(verts[f[1]].b, w[1]),
                        ),
                        v3_scale(verts[f[2]].b, w[2]),
                    );
                    best_face = f;
                    best_w = w;
                }
            }
            if inside {
                return (x, best_b);
            }
            let mut out = [verts[0]; 4];
            let mut n = 0usize;
            for i in 0..3 {
                if best_w[i] > 0.0 {
                    out[n] = verts[best_face[i]];
                    n += 1;
                }
            }
            if n > 0 {
                verts[..n].copy_from_slice(&out[..n]);
                *len = n;
            }
            (best_c, best_b)
        }
        _ => ([0.0; 3], [0.0; 3]),
    }
}

/// Gino van den Bergen GJK Raycast (sweeps convex `shape_a` along `unit_dir * max_dist` against `shape_b`).
///
/// Returns `Some((distance, normal_out_of_b, point_on_b))` on hit with `distance in [0, max_dist]`.
fn gjk_raycast_convex(
    shape_a: SupportShape,
    shape_b: SupportShape,
    unit_dir: [f32; 3],
    max_dist: f32,
) -> Option<(f32, [f32; 3], [f32; 3])> {
    let r = v3_scale(unit_dir, max_dist);
    let mut lambda = 0.0f32;
    let mut x = [0.0f32; 3];
    let mut normal = v3_neg(unit_dir);

    let mut v = v3_sub(shape_a.pose.p, shape_b.pose.p);
    if v3_len_sq(v) < 1e-12 {
        v = v3_neg(unit_dir);
    }

    let mut verts = [SimplexVert {
        c: [0.0; 3],
        b: shape_b.pose.p,
    }; 4];
    let mut len = 0usize;
    let mut last_b = shape_b.pose.p;

    const MAX_ITERS: usize = 64;
    const EPS_SQ: f32 = 1e-8;

    for _ in 0..MAX_ITERS {
        let v_len2 = v3_len_sq(v);
        if v_len2 <= EPS_SQ {
            break;
        }
        let b_sup = shape_b.support(v);
        let a_sup = shape_a.support(v3_neg(v));
        let p = v3_sub(b_sup, a_sup);
        let w = v3_sub(x, p);
        let v_dot_w = v3_dot(v, w);

        if v_dot_w > 1e-7 {
            let v_dot_r = v3_dot(v, r);
            if v_dot_r >= 0.0 {
                return None;
            }
            let delta_lambda = -v_dot_w / v_dot_r;
            lambda += delta_lambda;
            if lambda > 1.0 {
                return None;
            }
            x = v3_scale(r, lambda);
            normal = v;
            verts[0] = SimplexVert { c: p, b: b_sup };
            len = 1;
            last_b = b_sup;
            v = v3_sub(x, p);
            continue;
        }

        // Check for convergence (no closer support point exists along v).
        if v_len2 - v_dot_w <= 1e-6 * v_len2.max(1.0) {
            if v_len2 > 1e-5 {
                return None;
            }
            break;
        }

        if len < 4 {
            verts[len] = SimplexVert { c: p, b: b_sup };
            len += 1;
        }
        let (closest_c, closest_b) = reduce_simplex(&mut verts, &mut len, x);
        last_b = closest_b;
        v = v3_sub(x, closest_c);
    }

    if v3_len_sq(v) > 1e-4 {
        return None;
    }

    let dist = (lambda * max_dist).clamp(0.0, max_dist);
    let n = v3_norm_or(normal, v3_neg(unit_dir));
    Some((dist, n, last_b))
}

/// Sweep analytic `g0` along `unit_dir` up to `max_dist` against `g1`.
fn sweep_analytic(
    unit_dir: [f32; 3],
    max_dist: f32,
    g0: AnalyticGeom,
    p0: PxTransform,
    g1: AnalyticGeom,
    p1: PxTransform,
    hit_flags: u16,
    inflation: f32,
) -> Option<(f32, [f32; 3], [f32; 3], u16)> {
    let infl = inflation.max(0.0);
    if matches!(g0, AnalyticGeom::Plane) {
        return None;
    }
    let is_mtd = (hit_flags & PX_HIT_MTD) != 0;
    let assume_no_overlap = (hit_flags & PX_HIT_ASSUME_NO_INITIAL_OVERLAP) != 0;

    // Inflated version of g0 for overlap/MTD checks when inflation > 0.
    let g0_infl = match g0 {
        AnalyticGeom::Sphere { radius } => AnalyticGeom::Sphere {
            radius: radius + infl,
        },
        AnalyticGeom::Capsule {
            radius,
            half_height,
        } => AnalyticGeom::Capsule {
            radius: radius + infl,
            half_height,
        },
        AnalyticGeom::Box { half_extents } => AnalyticGeom::Box {
            half_extents: [
                half_extents[0] + infl,
                half_extents[1] + infl,
                half_extents[2] + infl,
            ],
        },
        AnalyticGeom::Plane => AnalyticGeom::Plane,
    };

    // 1. Handle Plane targets analytically (matches GuSweepSharedTests.cpp).
    if matches!(g1, AnalyticGeom::Plane) {
        let pl = WorldPlane::from_pose(p1);
        let (extreme_pt, extreme_dist) = match g0 {
            AnalyticGeom::Sphere { radius } => {
                let pt = v3_sub(p0.p, v3_scale(pl.n, radius + infl));
                (pt, pl.distance(pt))
            }
            AnalyticGeom::Capsule {
                radius,
                half_height,
            } => {
                let (c0, c1) = capsule_segment(p0, half_height);
                let d0 = pl.distance(c0);
                let d1 = pl.distance(c1);
                let end = if d0 <= d1 { c0 } else { c1 };
                let pt = v3_sub(end, v3_scale(pl.n, radius + infl));
                (pt, pl.distance(pt))
            }
            AnalyticGeom::Box { half_extents } => {
                let corners = obb_corners(p0, half_extents);
                let mut best = corners[0];
                let mut best_d = pl.distance(best);
                for c in corners.iter().skip(1) {
                    let d = pl.distance(*c);
                    if d < best_d {
                        best_d = d;
                        best = *c;
                    }
                }
                let pt = v3_sub(best, v3_scale(pl.n, infl));
                (pt, pl.distance(pt))
            }
            AnalyticGeom::Plane => return None,
        };

        if extreme_dist <= 0.0 {
            if is_mtd {
                let pos = v3_sub(extreme_pt, v3_scale(pl.n, extreme_dist));
                return Some((
                    extreme_dist,
                    pl.n,
                    pos,
                    PX_HIT_DISTANCE | PX_HIT_NORMAL | PX_HIT_POSITION,
                ));
            }
            if !assume_no_overlap {
                return Some((
                    0.0,
                    v3_neg(unit_dir),
                    [0.0; 3],
                    PX_HIT_DISTANCE | PX_HIT_NORMAL,
                ));
            }
        }

        let dn = v3_dot(unit_dir, pl.n);
        if dn >= -1e-7 {
            return None;
        }
        let t = -extreme_dist / dn;
        if t < 0.0 || t > max_dist {
            return None;
        }
        let pos = v3_add(extreme_pt, v3_scale(unit_dir, t));
        return Some((
            t,
            pl.n,
            pos,
            PX_HIT_DISTANCE | PX_HIT_NORMAL | PX_HIT_POSITION,
        ));
    }

    // 2. Check initial overlap for convex-vs-convex pairs.
    if is_mtd || !assume_no_overlap {
        if let Some((mtd_dir, depth)) = compute_penetration_analytic(g0_infl, p0, g1, p1) {
            if is_mtd {
                let shape_b = SupportShape {
                    geom: g1,
                    pose: p1,
                    inflation: 0.0,
                };
                let pos = shape_b.support(mtd_dir);
                return Some((
                    -depth,
                    mtd_dir,
                    pos,
                    PX_HIT_DISTANCE | PX_HIT_NORMAL | PX_HIT_POSITION,
                ));
            }
            return Some((
                0.0,
                v3_neg(unit_dir),
                [0.0; 3],
                PX_HIT_DISTANCE | PX_HIT_NORMAL,
            ));
        }
    }

    if max_dist <= 0.0 {
        return None;
    }

    // 3. Exact analytical fast paths for Sphere vs Sphere and Sphere vs Capsule.
    match (g0, g1) {
        (AnalyticGeom::Sphere { radius: r0 }, AnalyticGeom::Sphere { radius: r1 }) => {
            let (t, hit_c) = ray_sphere(p0.p, unit_dir, max_dist, p1.p, r0 + r1 + infl)?;
            let n = v3_norm_or(v3_sub(hit_c, p1.p), v3_neg(unit_dir));
            let pos = v3_add(p1.p, v3_scale(n, r1));
            return Some((
                t,
                n,
                pos,
                PX_HIT_DISTANCE | PX_HIT_NORMAL | PX_HIT_POSITION,
            ));
        }
        (
            AnalyticGeom::Sphere { radius: r0 },
            AnalyticGeom::Capsule {
                radius: r1,
                half_height,
            },
        ) => {
            let (c0, c1) = capsule_segment(p1, half_height);
            let t = ray_capsule(p0.p, unit_dir, max_dist, c0, c1, r0 + r1 + infl)?;
            let hit_c = v3_add(p0.p, v3_scale(unit_dir, t));
            let (_, _, seg_pt) = dist_point_segment_sq(c0, c1, hit_c);
            let n = v3_norm_or(v3_sub(hit_c, seg_pt), v3_neg(unit_dir));
            let pos = v3_add(seg_pt, v3_scale(n, r1));
            return Some((
                t,
                n,
                pos,
                PX_HIT_DISTANCE | PX_HIT_NORMAL | PX_HIT_POSITION,
            ));
        }
        (
            AnalyticGeom::Capsule {
                radius: r0,
                half_height,
            },
            AnalyticGeom::Sphere { radius: r1 },
        ) => {
            let (c0, c1) = capsule_segment(p0, half_height);
            let t = ray_capsule(p1.p, v3_neg(unit_dir), max_dist, c0, c1, r0 + r1 + infl)?;
            let rel_sphere = v3_add(p1.p, v3_scale(v3_neg(unit_dir), t));
            let (_, _, seg_pt) = dist_point_segment_sq(c0, c1, rel_sphere);
            // Normal points out of static sphere g1 towards swept capsule g0.
            let n = v3_norm_or(v3_sub(seg_pt, rel_sphere), v3_neg(unit_dir));
            let pos = v3_add(p1.p, v3_scale(n, r1));
            return Some((
                t,
                n,
                pos,
                PX_HIT_DISTANCE | PX_HIT_NORMAL | PX_HIT_POSITION,
            ));
        }
        _ => {}
    }

    // 4. General convex-vs-convex sweep via GJK Raycast (Sphere/Capsule/Box vs Sphere/Capsule/Box).
    let shape_a = SupportShape {
        geom: g0,
        pose: p0,
        inflation: infl,
    };
    let shape_b = SupportShape {
        geom: g1,
        pose: p1,
        inflation: 0.0,
    };
    let (dist, normal, pos) = gjk_raycast_convex(shape_a, shape_b, unit_dir, max_dist)?;
    Some((
        dist,
        normal,
        pos,
        PX_HIT_DISTANCE | PX_HIT_NORMAL | PX_HIT_POSITION,
    ))
}

// ---------------------------------------------------------------------------
// PxGeometryQuery & PxMeshQuery & PxTransformFromSegment exports
// ---------------------------------------------------------------------------

unsafe extern "win64" fn px_transform_from_segment(
    sret: *mut PxTransform,
    p0: *const [f32; 3],
    p1: *const [f32; 3],
    half_height: *mut f32,
) -> *mut PxTransform {
    let a = if p0.is_null() {
        [0.0, 0.0, 0.0]
    } else {
        std::ptr::read_unaligned(p0)
    };
    let b = if p1.is_null() {
        [0.0, 0.0, 0.0]
    } else {
        std::ptr::read_unaligned(p1)
    };
    let axis = v3_sub(b, a);
    let height = v3_len(axis);
    if !half_height.is_null() {
        std::ptr::write_unaligned(half_height, height * 0.5);
    }
    if !sret.is_null() {
        let mid = [
            (a[0] + b[0]) * 0.5,
            (a[1] + b[1]) * 0.5,
            (a[2] + b[2]) * 0.5,
        ];
        let q = if height < 1e-6 {
            [0.0, 0.0, 0.0, 1.0]
        } else {
            let inv = 1.0 / height;
            shortest_rotation_quat([1.0, 0.0, 0.0], [axis[0] * inv, axis[1] * inv, axis[2] * inv])
        };
        std::ptr::write_unaligned(sret, PxTransform { q, p: mid });
    }
    sret
}

unsafe extern "win64" fn geom_get_world_bounds(
    sret: *mut PxBounds3,
    geom: *const c_void,
    pose_ptr: *const PxTransform,
    inflation: f32,
) -> *mut PxBounds3 {
    if sret.is_null() {
        return sret;
    }
    let pose = read_pose(pose_ptr);
    let infl = if inflation.is_finite() && inflation > 0.0 {
        inflation
    } else {
        1.0
    };
    let bounds = match read_geometry(geom) {
        Ok(AnalyticGeom::Sphere { radius }) => {
            let e = radius * infl;
            PxBounds3 {
                minimum: [pose.p[0] - e, pose.p[1] - e, pose.p[2] - e],
                maximum: [pose.p[0] + e, pose.p[1] + e, pose.p[2] + e],
            }
        }
        Ok(AnalyticGeom::Plane) => {
            let big = PX_MAX_BOUNDS_EXTENTS;
            let mut min_pt = [-big; 3];
            let mut max_pt = [big; 3];
            let pl = WorldPlane::from_pose(pose);
            let ax = [pl.n[0].abs(), pl.n[1].abs(), pl.n[2].abs()];
            let eps = 1e-6f32;
            let one_m = 1.0 - eps;
            for i in 0..3 {
                let i1 = (i + 1) % 3;
                let i2 = (i + 2) % 3;
                if ax[i] > one_m && ax[i1] < eps && ax[i2] < eps {
                    if pl.n[i] > 0.0 {
                        max_pt[i] = -pl.d;
                    } else {
                        min_pt[i] = pl.d;
                    }
                    break;
                }
            }
            if (infl - 1.0).abs() > 1e-6 {
                for i in 0..3 {
                    let c = (max_pt[i] + min_pt[i]) * 0.5;
                    let e = (max_pt[i] - min_pt[i]) * 0.5 * infl;
                    min_pt[i] = c - e;
                    max_pt[i] = c + e;
                }
            }
            PxBounds3 {
                minimum: min_pt,
                maximum: max_pt,
            }
        }
        Ok(AnalyticGeom::Capsule {
            radius,
            half_height,
        }) => {
            let d = quat_basis(pose.q)[0];
            let mut ext = [0.0f32; 3];
            for i in 0..3 {
                ext[i] = (d[i].abs() * half_height + radius) * infl;
            }
            PxBounds3 {
                minimum: v3_sub(pose.p, ext),
                maximum: v3_add(pose.p, ext),
            }
        }
        Ok(AnalyticGeom::Box { half_extents: he }) => {
            let b = quat_basis(pose.q);
            let mut ext = [0.0f32; 3];
            for i in 0..3 {
                ext[i] = ((b[0][i] * he[0]).abs()
                    + (b[1][i] * he[1]).abs()
                    + (b[2][i] * he[2]).abs())
                    * infl;
            }
            PxBounds3 {
                minimum: v3_sub(pose.p, ext),
                maximum: v3_add(pose.p, ext),
            }
        }
        Err(gtype) => {
            crate::stub!(
                "PxGeometryQuery::getWorldBounds unsupported cooked geometry {}",
                geom_type_name(gtype)
            );
            PxBounds3::empty()
        }
    };
    std::ptr::write_unaligned(sret, bounds);
    sret
}
crate::win64_bridge!(fn geom_get_world_bounds(*mut PxBounds3, *const c_void, *const PxTransform, f32) -> *mut PxBounds3);

/// Returns squared distance between `point` and `geom` at `pose` (0.0 if inside, -1.0 if unsupported),
/// matching `PxGeometryQuery::pointDistance` in `GuGeometryQuery.cpp`.
unsafe extern "win64" fn geom_point_distance(
    point: *const [f32; 3],
    geom: *const c_void,
    pose_ptr: *const PxTransform,
    closest_point: *mut [f32; 3],
) -> f32 {
    if point.is_null() {
        return -1.0;
    }
    let pt = std::ptr::read_unaligned(point);
    let pose = read_pose(pose_ptr);
    match read_geometry(geom) {
        Ok(AnalyticGeom::Sphere { radius }) => {
            let delta = v3_sub(pt, pose.p);
            let d = v3_len(delta);
            if d <= radius {
                return 0.0;
            }
            if !closest_point.is_null() {
                let cp = v3_add(pose.p, v3_scale(delta, radius / d));
                std::ptr::write_unaligned(closest_point, cp);
            }
            (d - radius) * (d - radius)
        }
        Ok(AnalyticGeom::Capsule {
            radius,
            half_height,
        }) => {
            let (p0, p1) = capsule_segment(pose, half_height);
            let (sq_dist, _, seg_cp) = dist_point_segment_sq(p0, p1, pt);
            if sq_dist <= radius * radius {
                return 0.0;
            }
            let d = sq_dist.sqrt();
            if !closest_point.is_null() {
                let dir = v3_norm_or(v3_sub(pt, seg_cp), [1.0, 0.0, 0.0]);
                std::ptr::write_unaligned(closest_point, v3_add(seg_cp, v3_scale(dir, radius)));
            }
            (d - radius) * (d - radius)
        }
        Ok(AnalyticGeom::Box { half_extents }) => {
            let (sq_dist, local_cp) = dist_point_obb_sq(pt, pose, half_extents);
            if !closest_point.is_null() && sq_dist != 0.0 {
                std::ptr::write_unaligned(closest_point, pose_transform(pose, local_cp));
            }
            sq_dist
        }
        Ok(AnalyticGeom::Plane) => {
            let pl = WorldPlane::from_pose(pose);
            let d = pl.distance(pt);
            if d <= 0.0 {
                return 0.0;
            }
            if !closest_point.is_null() {
                std::ptr::write_unaligned(closest_point, v3_sub(pt, v3_scale(pl.n, d)));
            }
            d * d
        }
        Err(gtype) => {
            crate::stub!(
                "PxGeometryQuery::pointDistance unsupported cooked geometry {}",
                geom_type_name(gtype)
            );
            -1.0
        }
    }
}

unsafe extern "win64" fn geom_compute_penetration(
    direction: *mut [f32; 3],
    depth: *mut f32,
    geom0: *const c_void,
    pose0: *const PxTransform,
    geom1: *const c_void,
    pose1: *const PxTransform,
) -> u8 {
    let g0 = match read_geometry(geom0) {
        Ok(g) => g,
        Err(t) => {
            crate::stub!(
                "PxGeometryQuery::computePenetration unsupported geometry0 {}",
                geom_type_name(t)
            );
            return 0;
        }
    };
    let g1 = match read_geometry(geom1) {
        Ok(g) => g,
        Err(t) => {
            crate::stub!(
                "PxGeometryQuery::computePenetration unsupported geometry1 {}",
                geom_type_name(t)
            );
            return 0;
        }
    };
    let p0 = read_pose(pose0);
    let p1 = read_pose(pose1);
    match compute_penetration_analytic(g0, p0, g1, p1) {
        Some((dir, dep)) => {
            if !direction.is_null() {
                std::ptr::write_unaligned(direction, dir);
            }
            if !depth.is_null() {
                std::ptr::write_unaligned(depth, dep);
            }
            1
        }
        None => 0,
    }
}

unsafe extern "win64" fn geom_overlap(
    geom0: *const c_void,
    pose0: *const PxTransform,
    geom1: *const c_void,
    pose1: *const PxTransform,
) -> u8 {
    let g0 = match read_geometry(geom0) {
        Ok(g) => g,
        Err(t) => {
            crate::stub!(
                "PxGeometryQuery::overlap unsupported geometry0 {}",
                geom_type_name(t)
            );
            return 0;
        }
    };
    let g1 = match read_geometry(geom1) {
        Ok(g) => g,
        Err(t) => {
            crate::stub!(
                "PxGeometryQuery::overlap unsupported geometry1 {}",
                geom_type_name(t)
            );
            return 0;
        }
    };
    let p0 = read_pose(pose0);
    let p1 = read_pose(pose1);
    overlap_analytic(g0, p0, g1, p1) as u8
}

unsafe extern "win64" fn geom_raycast(
    origin_ptr: *const [f32; 3],
    unit_dir_ptr: *const [f32; 3],
    geom: *const c_void,
    pose_ptr: *const PxTransform,
    max_dist: f32,
    hit_flags: u16,
    max_hits: u32,
    ray_hits: *mut PxRaycastHit,
) -> u32 {
    if origin_ptr.is_null()
        || unit_dir_ptr.is_null()
        || ray_hits.is_null()
        || max_hits == 0
        || !max_dist.is_finite()
        || max_dist < 0.0
    {
        return 0;
    }
    let origin = std::ptr::read_unaligned(origin_ptr);
    let unit_dir = std::ptr::read_unaligned(unit_dir_ptr);
    let pose = read_pose(pose_ptr);

    let g = match read_geometry(geom) {
        Ok(g) => g,
        Err(t) => {
            crate::stub!(
                "PxGeometryQuery::raycast unsupported cooked geometry {}",
                geom_type_name(t)
            );
            return 0;
        }
    };

    let (dist, pos, norm, out_flags) = match g {
        AnalyticGeom::Sphere { radius } => {
            let Some((t, hit_pos)) = ray_sphere(origin, unit_dir, max_dist, pose.p, radius) else {
                return 0;
            };
            let mut flags = PX_HIT_DISTANCE | PX_HIT_POSITION;
            let n = if (hit_flags & PX_HIT_NORMAL) != 0 {
                flags |= PX_HIT_NORMAL;
                if t == 0.0 {
                    v3_neg(unit_dir)
                } else {
                    v3_norm_or(v3_sub(hit_pos, pose.p), v3_neg(unit_dir))
                }
            } else {
                [0.0; 3]
            };
            (t, hit_pos, n, flags)
        }
        AnalyticGeom::Plane => {
            let pl = WorldPlane::from_pose(pose);
            let dn = v3_dot(unit_dir, pl.n);
            // PhysX raycast_plane performs backface culling (dn >= 0 misses).
            if dn >= -1e-7 {
                return 0;
            }
            let t = -pl.distance(origin) / dn;
            if t < 0.0 || t > max_dist {
                return 0;
            }
            let hit_pos = v3_add(origin, v3_scale(unit_dir, t));
            (
                t,
                hit_pos,
                pl.n,
                PX_HIT_DISTANCE | PX_HIT_POSITION | PX_HIT_NORMAL,
            )
        }
        AnalyticGeom::Capsule {
            radius,
            half_height,
        } => {
            let (p0, p1) = capsule_segment(pose, half_height);
            let Some(t) = ray_capsule(origin, unit_dir, max_dist, p0, p1, radius) else {
                return 0;
            };
            let hit_pos = v3_add(origin, v3_scale(unit_dir, t));
            let mut flags = PX_HIT_DISTANCE | PX_HIT_POSITION;
            let n = if (hit_flags & PX_HIT_NORMAL) != 0 {
                flags |= PX_HIT_NORMAL;
                if t == 0.0 {
                    v3_neg(unit_dir)
                } else {
                    let (_, _, seg_cp) = dist_point_segment_sq(p0, p1, hit_pos);
                    v3_norm_or(v3_sub(hit_pos, seg_cp), v3_neg(unit_dir))
                }
            } else {
                [0.0; 3]
            };
            (t, hit_pos, n, flags)
        }
        AnalyticGeom::Box { half_extents: he } => {
            let local_orig = pose_transform_inv(pose, origin);
            let local_dir = quat_rotate_inv(pose.q, unit_dir);
            let Some((t, local_impact, axis)) =
                ray_aabb(v3_neg(he), he, local_orig, local_dir)
            else {
                return 0;
            };
            if t > max_dist {
                return 0;
            }
            let mut flags = PX_HIT_DISTANCE;
            let hit_pos = if (hit_flags & PX_HIT_POSITION) != 0 {
                flags |= PX_HIT_POSITION;
                if t == 0.0 {
                    origin
                } else {
                    pose_transform(pose, local_impact)
                }
            } else {
                [0.0; 3]
            };
            let n = if (hit_flags & PX_HIT_NORMAL) != 0 {
                flags |= PX_HIT_NORMAL;
                if t == 0.0 {
                    v3_neg(unit_dir)
                } else {
                    let mut ln = [0.0f32; 3];
                    ln[axis] = if local_impact[axis] > 0.0 { 1.0 } else { -1.0 };
                    quat_rotate(pose.q, ln)
                }
            } else {
                [0.0; 3]
            };
            (t, hit_pos, n, flags)
        }
    };

    let mut hit = std::ptr::read_unaligned(ray_hits);
    hit.face_index = 0xffff_ffff;
    hit.flags = out_flags;
    hit.position = pos;
    hit.normal = norm;
    hit.distance = dist;
    hit.u = 0.0;
    hit.v = 0.0;
    std::ptr::write_unaligned(ray_hits, hit);
    1
}
crate::win64_bridge!(fn geom_raycast(*const [f32; 3], *const [f32; 3], *const c_void, *const PxTransform, f32, u16, u32, *mut PxRaycastHit) -> u32);

unsafe extern "win64" fn geom_sweep(
    unit_dir_ptr: *const [f32; 3],
    max_dist: f32,
    geom0: *const c_void,
    pose0: *const PxTransform,
    geom1: *const c_void,
    pose1: *const PxTransform,
    sweep_hit: *mut PxSweepHit,
    hit_flags: u16,
    inflation: f32,
) -> u8 {
    if unit_dir_ptr.is_null()
        || sweep_hit.is_null()
        || !max_dist.is_finite()
        || max_dist < 0.0
    {
        return 0;
    }
    let g0 = match read_geometry(geom0) {
        Ok(g) => g,
        Err(t) => {
            crate::stub!(
                "PxGeometryQuery::sweep unsupported geometry0 {}",
                geom_type_name(t)
            );
            return 0;
        }
    };
    let g1 = match read_geometry(geom1) {
        Ok(g) => g,
        Err(t) => {
            crate::stub!(
                "PxGeometryQuery::sweep unsupported geometry1 {}",
                geom_type_name(t)
            );
            return 0;
        }
    };
    let unit_dir = std::ptr::read_unaligned(unit_dir_ptr);
    let p0 = read_pose(pose0);
    let p1 = read_pose(pose1);

    let Some((dist, normal, pos, flags)) =
        sweep_analytic(unit_dir, max_dist, g0, p0, g1, p1, hit_flags, inflation)
    else {
        return 0;
    };

    let mut hit = std::ptr::read_unaligned(sweep_hit);
    hit.face_index = 0xffff_ffff;
    hit.flags = flags;
    hit.distance = dist;
    hit.normal = normal;
    hit.position = pos;
    std::ptr::write_unaligned(sweep_hit, hit);
    1
}
crate::win64_bridge!(fn geom_sweep(*const [f32; 3], f32, *const c_void, *const PxTransform, *const c_void, *const PxTransform, *mut PxSweepHit, u16, f32) -> u8);

unsafe extern "win64" fn mesh_find_overlap_triangle_mesh(
    _geom0: *const c_void,
    _pose0: *const c_void,
    _geom1: *const c_void,
    _pose1: *const c_void,
    _results: *mut u32,
    _max_results: u32,
    _start_index: u32,
    overflow: *mut u8,
) -> u32 {
    crate::stub!("PxMeshQuery::findOverlapTriangleMesh without cooked triangle mesh payload");
    if !overflow.is_null() {
        std::ptr::write_unaligned(overflow, 0);
    }
    0
}

unsafe extern "win64" fn mesh_get_triangle(
    _tri_geom: *const c_void,
    _transform: *const c_void,
    _tri_index: u32,
    triangle: *mut [f32; 9],
    vertex_indices: *mut [u32; 3],
    adjacency_indices: *mut [u32; 3],
) {
    crate::stub!("PxMeshQuery::getTriangle without cooked triangle mesh payload");
    if !triangle.is_null() {
        std::ptr::write_unaligned(triangle, [0.0; 9]);
    }
    if !vertex_indices.is_null() {
        std::ptr::write_unaligned(vertex_indices, [0; 3]);
    }
    if !adjacency_indices.is_null() {
        std::ptr::write_unaligned(adjacency_indices, [0; 3]);
    }
}

unsafe extern "win64" fn mesh_sweep() -> u8 {
    crate::stub!("PxMeshQuery::sweep without cooked triangle mesh payload");
    0
}

pub static EXTRA_EXPORTS: &[Export] = &[
    Export {
        name: "?findOverlapTriangleMesh@PxMeshQuery@physx@@SAIAEBVPxGeometry@2@AEBVPxTransform@2@AEBVPxTriangleMeshGeometry@2@1PEAIIIAEA_N@Z",
        ptr: Ptr(mesh_find_overlap_triangle_mesh as *const ()),
    },
    Export {
        name: "?getWorldBounds@PxGeometryQuery@physx@@SA?AVPxBounds3@2@AEBVPxGeometry@2@AEBVPxTransform@2@M@Z",
        ptr: Ptr(crate::bridge_entry!(geom_get_world_bounds)),
    },
    Export {
        name: "?getTriangle@PxMeshQuery@physx@@SAXAEBVPxTriangleMeshGeometry@2@AEBVPxTransform@2@IAEAVPxTriangle@2@PEAI3@Z",
        ptr: Ptr(mesh_get_triangle as *const ()),
    },
    Export {
        name: "?computePenetration@PxGeometryQuery@physx@@SA_NAEAVPxVec3@2@AEAMAEBVPxGeometry@2@AEBVPxTransform@2@23@Z",
        ptr: Ptr(geom_compute_penetration as *const ()),
    },
    Export {
        name: "?PxCreateCollection@@YAPEAVPxCollection@physx@@XZ",
        ptr: Ptr(px_create_collection as *const ()),
    },
    Export {
        name: "?sweep@PxGeometryQuery@physx@@SA_NAEBVPxVec3@2@MAEBVPxGeometry@2@AEBVPxTransform@2@12AEAUPxSweepHit@2@V?$PxFlags@W4Enum@PxHitFlag@physx@@G@2@M@Z",
        ptr: Ptr(crate::bridge_entry!(geom_sweep)),
    },
    Export {
        name: "?sweep@PxMeshQuery@physx@@SA_NAEBVPxVec3@2@MAEBVPxGeometry@2@AEBVPxTransform@2@IPEBVPxTriangle@2@AEAUPxSweepHit@2@V?$PxFlags@W4Enum@PxHitFlag@physx@@G@2@PEBIM_N@Z",
        ptr: Ptr(mesh_sweep as *const ()),
    },
    Export {
        name: "?raycast@PxGeometryQuery@physx@@SAIAEBVPxVec3@2@0AEBVPxGeometry@2@AEBVPxTransform@2@MV?$PxFlags@W4Enum@PxHitFlag@physx@@G@2@IPEIAUPxRaycastHit@2@@Z",
        ptr: Ptr(crate::bridge_entry!(geom_raycast)),
    },
    Export {
        name: "?pointDistance@PxGeometryQuery@physx@@SAMAEBVPxVec3@2@AEBVPxGeometry@2@AEBVPxTransform@2@PEAV32@@Z",
        ptr: Ptr(geom_point_distance as *const ()),
    },
    Export {
        name: "?overlap@PxGeometryQuery@physx@@SA_NAEBVPxGeometry@2@AEBVPxTransform@2@01@Z",
        ptr: Ptr(geom_overlap as *const ()),
    },
    Export {
        name: "?PxTransformFromSegment@physx@@YA?AVPxTransform@1@AEBVPxVec3@1@0PEAM@Z",
        ptr: Ptr(px_transform_from_segment as *const ()),
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    fn id_pose(p: [f32; 3]) -> PxTransform {
        PxTransform {
            q: [0.0, 0.0, 0.0, 1.0],
            p,
        }
    }

    fn rot_z_pose(p: [f32; 3], angle_rad: f32) -> PxTransform {
        let half = angle_rad * 0.5;
        PxTransform {
            q: [0.0, 0.0, half.sin(), half.cos()],
            p,
        }
    }

    #[test]
    fn px_collection_and_geometry_queries() {
        assert_eq!(EXTRA_EXPORTS.len(), 11);
        unsafe {
            let col = px_create_collection();
            assert!(!col.is_null());
            assert_eq!(collection_get_nb_objects(col), 0);

            let dummy_obj = 0x1234_5678usize as *mut c_void;
            collection_add_object(col, dummy_obj, 42);
            assert_eq!(collection_get_nb_objects(col), 1);
            assert_eq!(collection_get_object(col, 0), dummy_obj);
            assert_eq!(collection_find(col, 42), dummy_obj);
            assert_eq!(collection_get_id(col, dummy_obj), 42);

            collection_release(col);
        }
    }

    #[test]
    fn world_bounds_for_analytic_geometries() {
        unsafe {
            let mut b = PxBounds3::empty();

            // Sphere at (1, 2, 3), r = 2, inflation = 1.0
            let sphere = PxSphereGeometry {
                geom_type: PX_GEOM_SPHERE,
                radius: 2.0,
            };
            let sp = id_pose([1.0, 2.0, 3.0]);
            geom_get_world_bounds(&mut b, &sphere as *const _ as *const c_void, &sp, 1.0);
            assert!((b.minimum[0] - (-1.0)).abs() < 1e-5);
            assert!((b.maximum[2] - 5.0).abs() < 1e-5);

            // Box rotated 90 deg around Z: extents (1, 3, 2) become (3, 1, 2).
            let bx = PxBoxGeometry {
                geom_type: PX_GEOM_BOX,
                half_extents: [1.0, 3.0, 2.0],
            };
            let bp = rot_z_pose([10.0, 0.0, 0.0], std::f32::consts::FRAC_PI_2);
            geom_get_world_bounds(&mut b, &bx as *const _ as *const c_void, &bp, 1.0);
            assert!((b.minimum[0] - 7.0).abs() < 1e-4);
            assert!((b.maximum[0] - 13.0).abs() < 1e-4);
            assert!((b.minimum[1] - (-1.0)).abs() < 1e-4);
            assert!((b.maximum[1] - 1.0).abs() < 1e-4);

            // Capsule rotated 90 deg around Z (local X -> world Y): halfHeight=2, radius=0.5
            let cap = PxCapsuleGeometry {
                geom_type: PX_GEOM_CAPSULE,
                radius: 0.5,
                half_height: 2.0,
            };
            let cp = rot_z_pose([0.0, 5.0, 0.0], std::f32::consts::FRAC_PI_2);
            geom_get_world_bounds(&mut b, &cap as *const _ as *const c_void, &cp, 1.0);
            assert!((b.minimum[0] - (-0.5)).abs() < 1e-4);
            assert!((b.maximum[1] - 7.5).abs() < 1e-4);
            assert!((b.minimum[1] - 2.5).abs() < 1e-4);

            // Plane with normal +Y at y = 3 -> maximum.y == 3.
            let pl = PxPlaneGeometry {
                geom_type: PX_GEOM_PLANE,
            };
            let pp = rot_z_pose([0.0, 3.0, 0.0], std::f32::consts::FRAC_PI_2);
            geom_get_world_bounds(&mut b, &pl as *const _ as *const c_void, &pp, 1.0);
            assert!((b.maximum[1] - 3.0).abs() < 1e-4);

            // Uncooked triangle mesh returns empty bounds.
            let tri_hdr = PX_GEOM_TRIANGLE_MESH;
            geom_get_world_bounds(&mut b, &tri_hdr as *const _ as *const c_void, &sp, 1.0);
            assert!(b.minimum[0] > b.maximum[0]);
        }
    }

    #[test]
    fn point_distance_for_analytic_geometries() {
        unsafe {
            let mut closest = [0.0f32; 3];

            // Sphere at origin, r = 2; point at (5, 0, 0) -> dist = 3, sq = 9, closest = (2, 0, 0).
            let sphere = PxSphereGeometry {
                geom_type: PX_GEOM_SPHERE,
                radius: 2.0,
            };
            let p0 = id_pose([0.0, 0.0, 0.0]);
            let d2 = geom_point_distance(
                &[5.0, 0.0, 0.0],
                &sphere as *const _ as *const c_void,
                &p0,
                &mut closest,
            );
            assert!((d2 - 9.0).abs() < 1e-4);
            assert!((closest[0] - 2.0).abs() < 1e-4);
            assert_eq!(
                geom_point_distance(
                    &[1.0, 0.0, 0.0],
                    &sphere as *const _ as *const c_void,
                    &p0,
                    std::ptr::null_mut()
                ),
                0.0
            );

            // Box [-1, 1]^3 at origin; point at (4, 5, 0) -> closest (1, 1, 0), sq = 9 + 16 = 25.
            let bx = PxBoxGeometry {
                geom_type: PX_GEOM_BOX,
                half_extents: [1.0, 1.0, 1.0],
            };
            let d2 = geom_point_distance(
                &[4.0, 5.0, 0.0],
                &bx as *const _ as *const c_void,
                &p0,
                &mut closest,
            );
            assert!((d2 - 25.0).abs() < 1e-4);
            assert!((closest[0] - 1.0).abs() < 1e-4 && (closest[1] - 1.0).abs() < 1e-4);

            // Capsule along X [-2, 2], r = 1; point at (0, 4, 0) -> dist = 3, sq = 9.
            let cap = PxCapsuleGeometry {
                geom_type: PX_GEOM_CAPSULE,
                radius: 1.0,
                half_height: 2.0,
            };
            let d2 = geom_point_distance(
                &[0.0, 4.0, 0.0],
                &cap as *const _ as *const c_void,
                &p0,
                &mut closest,
            );
            assert!((d2 - 9.0).abs() < 1e-4);
            assert!((closest[1] - 1.0).abs() < 1e-4);

            // Uncooked convex mesh returns -1.0.
            let cm = PX_GEOM_CONVEX_MESH;
            assert_eq!(
                geom_point_distance(
                    &[0.0, 0.0, 0.0],
                    &cm as *const _ as *const c_void,
                    &p0,
                    std::ptr::null_mut()
                ),
                -1.0
            );
        }
    }

    #[test]
    fn raycast_sphere_box_capsule_plane() {
        unsafe {
            let flags = PX_HIT_POSITION | PX_HIT_NORMAL | PX_HIT_DISTANCE;
            let mut hit: PxRaycastHit = std::mem::zeroed();

            // Sphere at (5, 0, 0), r = 1; ray from origin along +X -> hit at x=4, dist=4, normal=(-1,0,0).
            let sphere = PxSphereGeometry {
                geom_type: PX_GEOM_SPHERE,
                radius: 1.0,
            };
            let sp = id_pose([5.0, 0.0, 0.0]);
            assert_eq!(
                geom_raycast(
                    &[0.0, 0.0, 0.0],
                    &[1.0, 0.0, 0.0],
                    &sphere as *const _ as *const c_void,
                    &sp,
                    10.0,
                    flags,
                    1,
                    &mut hit
                ),
                1
            );
            assert!((hit.distance - 4.0).abs() < 1e-4);
            assert!((hit.position[0] - 4.0).abs() < 1e-4);
            assert!((hit.normal[0] - (-1.0)).abs() < 1e-4);

            // Box at (6, 0, 0), he=(1,2,2) -> hit at x=5, dist=5, normal=(-1,0,0).
            let bx = PxBoxGeometry {
                geom_type: PX_GEOM_BOX,
                half_extents: [1.0, 2.0, 2.0],
            };
            let bp = id_pose([6.0, 0.0, 0.0]);
            assert_eq!(
                geom_raycast(
                    &[0.0, 0.5, 0.0],
                    &[1.0, 0.0, 0.0],
                    &bx as *const _ as *const c_void,
                    &bp,
                    10.0,
                    flags,
                    1,
                    &mut hit
                ),
                1
            );
            assert!((hit.distance - 5.0).abs() < 1e-4);
            assert!((hit.normal[0] - (-1.0)).abs() < 1e-4);

            // Capsule standing vertically along Y at (4, 0, 0), r=0.5, hh=2 -> hit at x=3.5, dist=3.5.
            let cap = PxCapsuleGeometry {
                geom_type: PX_GEOM_CAPSULE,
                radius: 0.5,
                half_height: 2.0,
            };
            let cp = rot_z_pose([4.0, 0.0, 0.0], std::f32::consts::FRAC_PI_2);
            assert_eq!(
                geom_raycast(
                    &[0.0, 1.0, 0.0],
                    &[1.0, 0.0, 0.0],
                    &cap as *const _ as *const c_void,
                    &cp,
                    10.0,
                    flags,
                    1,
                    &mut hit
                ),
                1
            );
            assert!((hit.distance - 3.5).abs() < 1e-4);
            assert!((hit.normal[0] - (-1.0)).abs() < 1e-4);

            // Ground plane y = 0 (normal +Y); ray from (0, 5, 0) downwards (0, -1, 0) -> dist = 5.
            let pl = PxPlaneGeometry {
                geom_type: PX_GEOM_PLANE,
            };
            let pp = rot_z_pose([0.0, 0.0, 0.0], std::f32::consts::FRAC_PI_2);
            assert_eq!(
                geom_raycast(
                    &[0.0, 5.0, 0.0],
                    &[0.0, -1.0, 0.0],
                    &pl as *const _ as *const c_void,
                    &pp,
                    10.0,
                    flags,
                    1,
                    &mut hit
                ),
                1
            );
            assert!((hit.distance - 5.0).abs() < 1e-4);
            assert!((hit.normal[1] - 1.0).abs() < 1e-4);
            // Ray shooting away from plane misses.
            assert_eq!(
                geom_raycast(
                    &[0.0, 5.0, 0.0],
                    &[0.0, 1.0, 0.0],
                    &pl as *const _ as *const c_void,
                    &pp,
                    10.0,
                    flags,
                    1,
                    &mut hit
                ),
                0
            );
        }
    }

    #[test]
    fn overlap_and_penetration_resolve_contacts() {
        unsafe {
            let sphere = PxSphereGeometry {
                geom_type: PX_GEOM_SPHERE,
                radius: 1.0,
            };
            let bx = PxBoxGeometry {
                geom_type: PX_GEOM_BOX,
                half_extents: [1.0, 1.0, 1.0],
            };
            let cap = PxCapsuleGeometry {
                geom_type: PX_GEOM_CAPSULE,
                radius: 0.5,
                half_height: 1.0,
            };
            let pl = PxPlaneGeometry {
                geom_type: PX_GEOM_PLANE,
            };

            let check_pair = |g0: *const c_void,
                              p0: PxTransform,
                              g1: *const c_void,
                              p1: PxTransform,
                              expected_depth: f32| {
                assert_eq!(geom_overlap(g0, &p0, g1, &p1), 1);
                let mut dir = [0.0f32; 3];
                let mut depth = 0.0f32;
                assert_eq!(geom_compute_penetration(&mut dir, &mut depth, g0, &p0, g1, &p1), 1);
                assert!(
                    (depth - expected_depth).abs() < 1e-3,
                    "depth {depth} != expected {expected_depth}"
                );
                assert!((v3_len(dir) - 1.0).abs() < 1e-4);
                // Shifting pose0 along dir * (depth + eps) must resolve the overlap!
                let mut separated = p0;
                separated.p = v3_add(p0.p, v3_scale(dir, depth + 1e-3));
                assert_eq!(geom_overlap(g0, &separated, g1, &p1), 0);
            };

            // Sphere vs Sphere (centers 1.5 apart, r=1+1 -> depth 0.5)
            check_pair(
                &sphere as *const _ as *const c_void,
                id_pose([1.5, 0.0, 0.0]),
                &sphere as *const _ as *const c_void,
                id_pose([0.0, 0.0, 0.0]),
                0.5,
            );
            // Sphere vs Box (sphere at x=1.7, r=1; box [-1,1]^3 -> depth 0.3)
            check_pair(
                &sphere as *const _ as *const c_void,
                id_pose([1.7, 0.0, 0.0]),
                &bx as *const _ as *const c_void,
                id_pose([0.0, 0.0, 0.0]),
                0.3,
            );
            // Box vs Box (centers 1.6 apart along X, he=1+1 -> depth 0.4)
            check_pair(
                &bx as *const _ as *const c_void,
                id_pose([1.6, 0.0, 0.0]),
                &bx as *const _ as *const c_void,
                id_pose([0.0, 0.0, 0.0]),
                0.4,
            );
            // Capsule vs Box (capsule along X at y=1.2, r=0.5; box top at y=1.0 -> depth 0.3)
            check_pair(
                &cap as *const _ as *const c_void,
                id_pose([0.0, 1.2, 0.0]),
                &bx as *const _ as *const c_void,
                id_pose([0.0, 0.0, 0.0]),
                0.3,
            );
            // Capsule vs Plane (plane y <= 0, capsule horizontal at y=0.25, r=0.5 -> depth 0.25)
            check_pair(
                &cap as *const _ as *const c_void,
                id_pose([0.0, 0.25, 0.0]),
                &pl as *const _ as *const c_void,
                rot_z_pose([0.0, 0.0, 0.0], std::f32::consts::FRAC_PI_2),
                0.25,
            );

            // Disjoint shapes return 0.
            let far = id_pose([10.0, 0.0, 0.0]);
            let orig = id_pose([0.0, 0.0, 0.0]);
            assert_eq!(
                geom_overlap(
                    &bx as *const _ as *const c_void,
                    &far,
                    &bx as *const _ as *const c_void,
                    &orig
                ),
                0
            );
        }
    }

    #[test]
    fn sweep_analytic_geometries_and_mtd() {
        unsafe {
            let sphere = PxSphereGeometry {
                geom_type: PX_GEOM_SPHERE,
                radius: 1.0,
            };
            let bx = PxBoxGeometry {
                geom_type: PX_GEOM_BOX,
                half_extents: [1.0, 1.0, 1.0],
            };
            let pl = PxPlaneGeometry {
                geom_type: PX_GEOM_PLANE,
            };
            let flags = PX_HIT_POSITION | PX_HIT_NORMAL | PX_HIT_DISTANCE;
            let mut hit: PxSweepHit = std::mem::zeroed();

            // Sweep sphere from (-5, 0, 0) along +X into box at (0, 0, 0) -> gap is 3.0.
            let p0 = id_pose([-5.0, 0.0, 0.0]);
            let p1 = id_pose([0.0, 0.0, 0.0]);
            assert_eq!(
                geom_sweep(
                    &[1.0, 0.0, 0.0],
                    10.0,
                    &sphere as *const _ as *const c_void,
                    &p0,
                    &bx as *const _ as *const c_void,
                    &p1,
                    &mut hit,
                    flags,
                    0.0
                ),
                1
            );
            assert!((hit.distance - 3.0).abs() < 1e-3, "got dist {}", hit.distance);
            assert!((hit.normal[0] - (-1.0)).abs() < 1e-3);

            // Sweep box from (-5, 0, 0) along +X into box at (0, 0, 0) -> gap is 3.0.
            assert_eq!(
                geom_sweep(
                    &[1.0, 0.0, 0.0],
                    10.0,
                    &bx as *const _ as *const c_void,
                    &p0,
                    &bx as *const _ as *const c_void,
                    &p1,
                    &mut hit,
                    flags,
                    0.0
                ),
                1
            );
            assert!((hit.distance - 3.0).abs() < 1e-3, "got dist {}", hit.distance);
            assert!((hit.normal[0] - (-1.0)).abs() < 1e-3);

            // Sweep box downwards onto ground plane y=0 from y=4 (he=1) -> dist = 3.0.
            let p_above = id_pose([0.0, 4.0, 0.0]);
            let p_ground = rot_z_pose([0.0, 0.0, 0.0], std::f32::consts::FRAC_PI_2);
            assert_eq!(
                geom_sweep(
                    &[0.0, -1.0, 0.0],
                    10.0,
                    &bx as *const _ as *const c_void,
                    &p_above,
                    &pl as *const _ as *const c_void,
                    &p_ground,
                    &mut hit,
                    flags,
                    0.0
                ),
                1
            );
            assert!((hit.distance - 3.0).abs() < 1e-3);
            assert!((hit.normal[1] - 1.0).abs() < 1e-3);

            // Initial overlap with eMTD returns negative penetration depth.
            let p_overlap = id_pose([1.5, 0.0, 0.0]);
            assert_eq!(
                geom_sweep(
                    &[-1.0, 0.0, 0.0],
                    5.0,
                    &sphere as *const _ as *const c_void,
                    &p_overlap,
                    &sphere as *const _ as *const c_void,
                    &p1,
                    &mut hit,
                    flags | PX_HIT_MTD,
                    0.0
                ),
                1
            );
            assert!((hit.distance - (-0.5)).abs() < 1e-3);
            assert!((hit.normal[0] - 1.0).abs() < 1e-3);
        }
    }
}
