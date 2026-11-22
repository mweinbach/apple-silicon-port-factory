//! `libprotobuf.dll` — Google Protocol Buffers (v3/v4) native builtin.
//!
//! Implements core C++ ABI runtime primitives (`Arena`, `ArenaStringPtr`,
//! `RepeatedPtrFieldBase`, `RepeatedField<T>`, `Message`, `MessageLite`,
//! `UnknownFieldSet`, `CachedSize`, `WireFormatLite`, `CodedOutputStream`,
//! `EpsCopyOutputStream`, `EpsCopyInputStream`, `ParseContext`, `VarintParseSlow`,
//! `Packed*Parser`, `VerifyUtf8`, `Base64Escape`, `WebSafeBase64Escape`,
//! and `Reflection`).

use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::{Mutex, Once, OnceLock};

pub static ORDINALS: &[(u16, &str)] = &[];

// --- Wire format constants ---

const WIRETYPE_VARINT: u32 = 0;
const WIRETYPE_FIXED64: u32 = 1;
const WIRETYPE_LENGTH_DELIMITED: u32 = 2;
const WIRETYPE_START_GROUP: u32 = 3;
const WIRETYPE_END_GROUP: u32 = 4;
const WIRETYPE_FIXED32: u32 = 5;

const EPS_SLOP_BYTES: usize = 16;
const MAX_GROUP_DEPTH: usize = 100;

// --- Core MSVC / Protobuf ABI structs ---

/// MSVC x64 `std::string` (`std::basic_string<char>`, `0x20` bytes):
/// * `+0x00`: 16-byte SSO buffer (`cap < 16`) or heap pointer at `+0x00` (`cap >= 16`)
/// * `+0x10`: `size` (`usize`)
/// * `+0x18`: `cap` (`usize`, `15` for empty SSO)
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct MsvcString {
    pub buf: [u8; 16],
    pub size: usize,
    pub cap: usize,
}

const _: () = assert!(std::mem::size_of::<MsvcString>() == 0x20);
unsafe impl Sync for MsvcString {}

#[repr(C, align(16))]
pub struct SyncMsvcString(std::cell::UnsafeCell<MsvcString>);
unsafe impl Sync for SyncMsvcString {}

static FIXED_ADDRESS_EMPTY_STRING: SyncMsvcString =
    SyncMsvcString(std::cell::UnsafeCell::new(MsvcString {
        buf: [0u8; 16],
        size: 0,
        cap: 15,
    }));

#[inline]
fn empty_string_ptr() -> *const MsvcString {
    FIXED_ADDRESS_EMPTY_STRING.0.get() as *const MsvcString
}

/// `google::protobuf::internal::RepeatedPtrFieldBase` (`0x18` bytes):
/// * `+0x00`: `arena: usize`
/// * `+0x08`: `current_size: i32`
/// * `+0x0c`: `total_size: i32`
/// * `+0x10`: `rep: *mut u8` (`+0x00`: `allocated_size: i32`, `+0x08`: `elements[...]`)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RepeatedPtrFieldBase {
    pub arena: usize,
    pub current_size: i32,
    pub total_size: i32,
    pub rep: *mut u8,
}

const _: () = assert!(std::mem::size_of::<RepeatedPtrFieldBase>() == 0x18);

/// `google::protobuf::RepeatedField<T>` for 8-byte elements (`u64` / `f64`, `0x10` bytes):
/// * `+0x00`: `current_size: i32`
/// * `+0x04`: `total_size: i32`
/// * `+0x08`: `ptr: *mut u64` (arena pointer when `total_size == 0`, else `alloc + 8`)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RepeatedField64 {
    pub current_size: i32,
    pub total_size: i32,
    pub ptr: *mut u64,
}

const _: () = assert!(std::mem::size_of::<RepeatedField64>() == 0x10);

/// Type-erased `google::protobuf::RepeatedField<T>` (`0x10` bytes) used by packed parsers.
#[repr(C)]
#[derive(Clone, Copy)]
struct RepeatedFieldRaw {
    current_size: i32,
    total_size: i32,
    ptr: *mut u8,
}

const _: () = assert!(std::mem::size_of::<RepeatedFieldRaw>() == 0x10);

/// `google::protobuf::io::EpsCopyOutputStream` (`0x38` bytes):
/// * `+0x00`: `end` — end of fast-path writable region (`actual_end - 16` for streams)
/// * `+0x08`: `stream` — optional `ZeroCopyOutputStream*` (null for flat array buffers)
/// * `+0x10`: `had_error`
/// * `+0x11`: `aliasing_enabled`
/// * `+0x12`: `is_deterministic`
/// * `+0x18`: `buffer[32]` — `2 * kSlopBytes` patch/error-sink buffer
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EpsCopyOutputStream {
    pub end: *mut u8,
    pub stream: *mut u8,
    pub had_error: u8,
    pub aliasing_enabled: u8,
    pub is_deterministic: u8,
    pub _pad: [u8; 5],
    pub buffer: [u8; 32],
}

const _: () = assert!(std::mem::size_of::<EpsCopyOutputStream>() == 0x38);
const _: () = assert!(std::mem::offset_of!(EpsCopyOutputStream, buffer) == 0x18);

/// `google::protobuf::internal::EpsCopyInputStream` / `ParseContext` prefix (`0x50` bytes):
/// * `+0x00`: `limit_end` — `buffer_end + min(0, limit)`
/// * `+0x08`: `buffer_end` — end of current contiguous chunk
/// * `+0x10`: `next_chunk` — next chunk pointer (e.g. `&buffer[0]` for final 16-byte tail)
/// * `+0x18`: `size` — byte length of `next_chunk`
/// * `+0x1c`: `limit` — active limit relative to `buffer_end`
/// * `+0x20`: `zcis` — optional `ZeroCopyInputStream*`
/// * `+0x28`: `buffer[32]` — `2 * kSlopBytes` tail/slop patch buffer
/// * `+0x48`: `aliasing`
/// * `+0x4c`: `depth` — remaining recursion depth
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EpsCopyInputStream {
    pub limit_end: *const u8,
    pub buffer_end: *const u8,
    pub next_chunk: *const u8,
    pub size: i32,
    pub limit: i32,
    pub zcis: *mut u8,
    pub buffer: [u8; 32],
    pub aliasing: i32,
    pub depth: i32,
}

const _: () = assert!(std::mem::size_of::<EpsCopyInputStream>() == 0x50);
const _: () = assert!(std::mem::offset_of!(EpsCopyInputStream, buffer) == 0x28);

#[repr(C, align(16))]
struct MessageVtable([crate::Ptr; 25]);
unsafe impl Sync for MessageVtable {}

const VTABLE_SLOT_CLEAR: usize = 4;
const VTABLE_SLOT_IS_INITIALIZED: usize = 5;
const VTABLE_SLOT_BYTE_SIZE_LONG: usize = 8;
const VTABLE_SLOT_GET_CACHED_SIZE: usize = 9;
const VTABLE_SLOT_INTERNAL_PARSE: usize = 10;
const VTABLE_SLOT_INTERNAL_SERIALIZE: usize = 11;

static MESSAGE_VTBL: MessageVtable = MessageVtable([
    crate::Ptr(msg_scalar_deleting_dtor as *const ()), // 0 (+0x00): ~Message
    crate::Ptr(msg_get_type_name as *const ()),        // 1 (+0x08): GetTypeName
    crate::Ptr(msg_new as *const ()),                  // 2 (+0x10): New
    crate::Ptr(msg_ret_null as *const ()),             // 3 (+0x18): GetClassData
    crate::Ptr(msg_noop as *const ()),                 // 4 (+0x20): Clear
    crate::Ptr(msg_is_initialized as *const ()),       // 5 (+0x28): IsInitialized
    crate::Ptr(msg_init_error_string as *const ()),    // 6 (+0x30): InitializationErrorString
    crate::Ptr(msg_check_type_and_merge as *const ()), // 7 (+0x38): CheckTypeAndMergeFrom
    crate::Ptr(msg_space_used_long as *const ()),      // 8 (+0x40): ByteSizeLong
    crate::Ptr(msg_get_cached_size as *const ()),      // 9 (+0x48): GetCachedSize
    crate::Ptr(msg_internal_parse as *const ()),       // 10 (+0x50): _InternalParse
    crate::Ptr(msg_internal_serialize as *const ()),   // 11 (+0x58): _InternalSerialize
    crate::Ptr(msg_internal_get_table as *const ()),   // 12 (+0x60): InternalGetTable
    crate::Ptr(msg_check_type_and_merge as *const ()), // 13 (+0x68): CopyFrom
    crate::Ptr(msg_check_type_and_merge as *const ()), // 14 (+0x70): MergeFrom
    crate::Ptr(msg_discard_unknown_fields as *const ()), // 15 (+0x78): DiscardUnknownFields
    crate::Ptr(msg_space_used_long as *const ()),      // 16 (+0x80): SpaceUsedLong
    crate::Ptr(msg_set_cached_size as *const ()),      // 17 (+0x88): SetCachedSize
    crate::Ptr(msg_get_metadata as *const ()),         // 18 (+0x90): GetMetadata
    crate::Ptr(msg_noop as *const ()),                 // 19 (+0x98)
    crate::Ptr(msg_noop as *const ()),                 // 20 (+0xa0)
    crate::Ptr(msg_noop as *const ()),                 // 21 (+0xa8)
    crate::Ptr(msg_noop as *const ()),                 // 22 (+0xb0)
    crate::Ptr(msg_noop as *const ()),                 // 23 (+0xb8)
    crate::Ptr(msg_noop as *const ()),                 // 24 (+0xc0)
]);

#[repr(C)]
struct DummyMessage {
    vtbl: *const [crate::Ptr; 25],
    internal_metadata: usize,
}
unsafe impl Sync for DummyMessage {}

static DUMMY_MESSAGE: DummyMessage = DummyMessage {
    vtbl: &MESSAGE_VTBL.0,
    internal_metadata: 0,
};

#[repr(C)]
struct DummyReflection {
    _unused: [usize; 8],
}
unsafe impl Sync for DummyReflection {}

static DUMMY_REFLECTION: DummyReflection = DummyReflection { _unused: [0; 8] };

static REGISTER_VTABLES: Once = Once::new();

fn ensure_vtables_registered() {
    REGISTER_VTABLES.call_once(|| {
        for fn_ptr in &MESSAGE_VTBL.0 {
            let addr = fn_ptr.0 as usize;
            crate::register_native_va(addr as u64, addr);
        }
        let dtor_addr = msvc_string_arena_dtor as *const () as usize;
        crate::register_native_va(dtor_addr as u64, dtor_addr);
        let fixed_addr = empty_string_ptr() as usize;
        for e in EXTRA_EXPORTS {
            let addr = e.ptr.0 as usize;
            if addr != fixed_addr {
                crate::register_native_va(addr as u64, addr);
            }
        }
    });
}

// --- Arena allocation & destructor tracking ---

struct ArenaAlloc {
    ptr: *mut u8,
    aligned: bool,
}
unsafe impl Send for ArenaAlloc {}

struct ArenaCleanup {
    object: *mut u8,
    dtor: usize,
}
unsafe impl Send for ArenaCleanup {}

#[derive(Default)]
struct ArenaState {
    allocs: Vec<ArenaAlloc>,
    cleanups: Vec<ArenaCleanup>,
    total_bytes: usize,
}

fn arenas() -> &'static Mutex<HashMap<usize, ArenaState>> {
    static MAP: OnceLock<Mutex<HashMap<usize, ArenaState>>> = OnceLock::new();
    MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

unsafe fn arena_alloc_tracked(
    arena: *mut u8,
    size: usize,
    align: usize,
    dtor: usize,
) -> *mut u8 {
    let alloc_size = size.max(1);
    let (ptr, aligned) = if align > 16 && align.is_power_of_two() {
        let p = crate::ucrt::heap::_aligned_malloc(alloc_size, align) as *mut u8;
        if !p.is_null() {
            std::ptr::write_bytes(p, 0, alloc_size);
        }
        (p, true)
    } else {
        (crate::ucrt::heap::calloc(1, alloc_size) as *mut u8, false)
    };
    if !ptr.is_null() && !arena.is_null() {
        let mut map = arenas().lock().unwrap_or_else(|e| e.into_inner());
        let state = map.entry(arena as usize).or_default();
        state.allocs.push(ArenaAlloc { ptr, aligned });
        state.total_bytes = state.total_bytes.saturating_add(alloc_size);
        if dtor != 0 {
            state.cleanups.push(ArenaCleanup { object: ptr, dtor });
        }
    }
    ptr
}

unsafe fn arena_destroy_state(arena: *mut u8) -> usize {
    if arena.is_null() {
        return 0;
    }
    let state = {
        let mut map = arenas().lock().unwrap_or_else(|e| e.into_inner());
        map.remove(&(arena as usize))
    };
    let Some(state) = state else {
        return 0;
    };
    let freed_bytes = state.total_bytes;
    let internal_str_dtor = msvc_string_arena_dtor as *const () as usize;
    // Run destructors in LIFO order outside the arena lock to prevent deadlocks.
    for cleanup in state.cleanups.into_iter().rev() {
        if cleanup.dtor != 0 && !cleanup.object.is_null() {
            if cleanup.dtor == internal_str_dtor {
                free_msvc_string_buf(cleanup.object as *mut MsvcString);
            } else {
                crate::call_guest8(
                    cleanup.dtor,
                    cleanup.object as u64,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                );
            }
        }
    }
    for alloc in state.allocs {
        if !alloc.ptr.is_null() {
            if alloc.aligned {
                crate::ucrt::heap::_aligned_free(alloc.ptr as *mut c_void);
            } else {
                crate::ucrt::heap::free(alloc.ptr as *mut c_void);
            }
        }
    }
    freed_bytes
}

// --- Unaligned-safe MsvcString helpers ---

#[inline]
unsafe fn init_empty_string(dst: *mut MsvcString) {
    if !dst.is_null() {
        std::ptr::write_unaligned(
            dst,
            MsvcString {
                buf: [0u8; 16],
                size: 0,
                cap: 15,
            },
        );
    }
}

#[inline]
unsafe fn msvc_string_data_and_len(s: *const MsvcString) -> (*const u8, usize) {
    if s.is_null() {
        return (std::ptr::null(), 0);
    }
    let cur = std::ptr::read_unaligned(s);
    let data = if cur.cap >= 16 {
        let mut ptr_bytes = [0u8; 8];
        ptr_bytes.copy_from_slice(&cur.buf[0..8]);
        usize::from_ne_bytes(ptr_bytes) as *const u8
    } else {
        // Inline SSO buffer begins at offset 0 of the guest MsvcString.
        s as *const u8
    };
    (data, cur.size)
}

#[inline]
unsafe fn free_msvc_string_buf(s: *mut MsvcString) {
    if !s.is_null() {
        let cur = std::ptr::read_unaligned(s);
        if cur.cap >= 16 {
            let mut ptr_bytes = [0u8; 8];
            ptr_bytes.copy_from_slice(&cur.buf[0..8]);
            let heap_ptr = usize::from_ne_bytes(ptr_bytes) as *mut c_void;
            if !heap_ptr.is_null() {
                crate::ucrt::heap::free(heap_ptr);
            }
        }
    }
}

unsafe extern "win64" fn msvc_string_arena_dtor(obj: *mut u8) {
    free_msvc_string_buf(obj as *mut MsvcString);
}

unsafe fn msvc_string_clear(dst: *mut MsvcString) {
    if dst.is_null() {
        return;
    }
    let mut cur = std::ptr::read_unaligned(dst);
    cur.size = 0;
    if cur.cap >= 16 {
        let mut ptr_bytes = [0u8; 8];
        ptr_bytes.copy_from_slice(&cur.buf[0..8]);
        let heap_ptr = usize::from_ne_bytes(ptr_bytes) as *mut u8;
        if !heap_ptr.is_null() {
            std::ptr::write(heap_ptr, 0);
        }
    } else {
        cur.buf[0] = 0;
    }
    std::ptr::write_unaligned(dst, cur);
}

unsafe fn msvc_string_assign_bytes(dst: *mut MsvcString, src: &[u8]) {
    if dst.is_null() {
        return;
    }
    let cur = std::ptr::read_unaligned(dst);
    let len = src.len();
    if cur.cap >= 16 {
        let mut ptr_bytes = [0u8; 8];
        ptr_bytes.copy_from_slice(&cur.buf[0..8]);
        let old_heap = usize::from_ne_bytes(ptr_bytes) as *mut u8;
        if !old_heap.is_null() && cur.cap >= len {
            if len > 0 {
                std::ptr::copy(src.as_ptr(), old_heap, len);
            }
            std::ptr::write(old_heap.add(len), 0);
            let mut updated = cur;
            updated.size = len;
            std::ptr::write_unaligned(dst, updated);
            return;
        }
        if len <= 15 {
            let mut new_str = MsvcString {
                buf: [0u8; 16],
                size: len,
                cap: 15,
            };
            if len > 0 {
                new_str.buf[..len].copy_from_slice(src);
            }
            if !old_heap.is_null() {
                crate::ucrt::heap::free(old_heap as *mut c_void);
            }
            std::ptr::write_unaligned(dst, new_str);
        } else {
            let new_cap = len | 15;
            let new_heap = crate::ucrt::heap::malloc(new_cap + 1) as *mut u8;
            if !new_heap.is_null() {
                std::ptr::copy_nonoverlapping(src.as_ptr(), new_heap, len);
                std::ptr::write(new_heap.add(len), 0);
                if !old_heap.is_null() {
                    crate::ucrt::heap::free(old_heap as *mut c_void);
                }
                let mut new_str = MsvcString {
                    buf: [0u8; 16],
                    size: len,
                    cap: new_cap,
                };
                new_str.buf[0..8].copy_from_slice(&(new_heap as usize).to_ne_bytes());
                std::ptr::write_unaligned(dst, new_str);
            }
        }
    } else if len <= 15 {
        let mut new_str = MsvcString {
            buf: [0u8; 16],
            size: len,
            cap: 15,
        };
        if len > 0 {
            new_str.buf[..len].copy_from_slice(src);
        }
        std::ptr::write_unaligned(dst, new_str);
    } else {
        let new_cap = len | 15;
        let new_heap = crate::ucrt::heap::malloc(new_cap + 1) as *mut u8;
        if !new_heap.is_null() {
            std::ptr::copy_nonoverlapping(src.as_ptr(), new_heap, len);
            std::ptr::write(new_heap.add(len), 0);
            let mut new_str = MsvcString {
                buf: [0u8; 16],
                size: len,
                cap: new_cap,
            };
            new_str.buf[0..8].copy_from_slice(&(new_heap as usize).to_ne_bytes());
            std::ptr::write_unaligned(dst, new_str);
        }
    }
}

unsafe fn msvc_string_append_bytes(dst: *mut MsvcString, extra: &[u8]) {
    if dst.is_null() || extra.is_empty() {
        return;
    }
    let (old_ptr, old_len) = msvc_string_data_and_len(dst);
    let mut combined = Vec::with_capacity(old_len.saturating_add(extra.len()));
    if old_len > 0 && !old_ptr.is_null() {
        combined.extend_from_slice(std::slice::from_raw_parts(old_ptr, old_len));
    }
    combined.extend_from_slice(extra);
    msvc_string_assign_bytes(dst, &combined);
}

#[inline]
unsafe fn ensure_mutable_string(this: *mut usize, arena: *mut u8) -> *mut MsvcString {
    let empty_addr = empty_string_ptr() as usize;
    let raw = std::ptr::read_unaligned(this) & !1;
    if raw == 0 || raw == empty_addr {
        let alloc = if !arena.is_null() {
            arena_alloc_tracked(
                arena,
                std::mem::size_of::<MsvcString>(),
                16,
                msvc_string_arena_dtor as *const () as usize,
            ) as *mut MsvcString
        } else {
            crate::ucrt::heap::calloc(1, std::mem::size_of::<MsvcString>()) as *mut MsvcString
        };
        if !alloc.is_null() {
            init_empty_string(alloc);
            std::ptr::write_unaligned(this, alloc as usize);
        }
        alloc
    } else {
        raw as *mut MsvcString
    }
}

// --- 1. Empty string & ArenaStringPtr ---

unsafe extern "win64" fn get_empty_string_already_inited() -> *const MsvcString {
    ensure_vtables_registered();
    empty_string_ptr()
}

unsafe extern "win64" fn arena_string_ptr_ctor(this: *mut usize, default_val: usize) -> *mut usize {
    ensure_vtables_registered();
    if !this.is_null() {
        std::ptr::write_unaligned(this, default_val);
    }
    this
}

unsafe extern "win64" fn arena_string_ptr_unsafe_set_default(this: *mut usize, default_val: usize) {
    if !this.is_null() {
        std::ptr::write_unaligned(this, default_val);
    }
}

unsafe extern "win64" fn arena_string_ptr_get(this: *const usize) -> *const MsvcString {
    if this.is_null() {
        return empty_string_ptr();
    }
    let ptr = std::ptr::read_unaligned(this) & !1;
    if ptr == 0 {
        empty_string_ptr()
    } else {
        ptr as *const MsvcString
    }
}

unsafe extern "win64" fn arena_string_ptr_clear_to_empty(this: *mut usize) {
    if this.is_null() {
        return;
    }
    let empty_addr = empty_string_ptr() as usize;
    let raw = std::ptr::read_unaligned(this) & !1;
    if raw != 0 && raw != empty_addr {
        msvc_string_clear(raw as *mut MsvcString);
    }
}

unsafe extern "win64" fn arena_string_ptr_mutable(
    this: *mut usize,
    _default: usize,
    arena: *mut u8,
) -> *mut MsvcString {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    ensure_mutable_string(this, arena)
}

unsafe extern "win64" fn arena_string_ptr_set_copy(
    this: *mut usize,
    _default: usize,
    src: *const MsvcString,
    arena: *mut u8,
) {
    if this.is_null() {
        return;
    }
    let dst = ensure_mutable_string(this, arena);
    if dst.is_null() || src.is_null() || (dst as *const MsvcString) == src {
        return;
    }
    let (src_bytes, len) = msvc_string_data_and_len(src);
    let slice = if len == 0 || src_bytes.is_null() {
        &[]
    } else {
        std::slice::from_raw_parts(src_bytes, len)
    };
    msvc_string_assign_bytes(dst, slice);
}

unsafe extern "win64" fn arena_string_ptr_set_move(
    this: *mut usize,
    _default: usize,
    src: *mut MsvcString,
    arena: *mut u8,
) {
    if this.is_null() {
        return;
    }
    let dst = ensure_mutable_string(this, arena);
    if dst.is_null() || src.is_null() || dst == src {
        return;
    }
    free_msvc_string_buf(dst);
    let moved = std::ptr::read_unaligned(src);
    std::ptr::write_unaligned(dst, moved);
    init_empty_string(src);
}

unsafe extern "win64" fn arena_string_ptr_swap(
    this: *mut usize,
    other: *mut usize,
    _default: *const MsvcString,
    _arena: *mut u8,
) {
    if !this.is_null() && !other.is_null() {
        let a = std::ptr::read_unaligned(this);
        let b = std::ptr::read_unaligned(other);
        std::ptr::write_unaligned(this, b);
        std::ptr::write_unaligned(other, a);
    }
}

unsafe extern "win64" fn arena_string_ptr_destroy(
    this: *mut usize,
    _default: usize,
    arena: *mut u8,
) {
    if !this.is_null() {
        let empty_addr = empty_string_ptr() as usize;
        let raw = std::ptr::read_unaligned(this) & !1;
        // Only free immediately when not arena-owned; arena-owned strings are cleaned up by ~Arena.
        if arena.is_null() && raw != 0 && raw != empty_addr {
            let s = raw as *mut MsvcString;
            free_msvc_string_buf(s);
            crate::ucrt::heap::free(s as *mut c_void);
        }
        std::ptr::write_unaligned(this, empty_addr);
    }
}

unsafe extern "win64" fn arena_string_ptr_destroy_no_arena(
    this: *mut usize,
    _default: *const MsvcString,
) {
    if !this.is_null() {
        let empty_addr = empty_string_ptr() as usize;
        let raw = std::ptr::read_unaligned(this) & !1;
        if raw != 0 && raw != empty_addr {
            let s = raw as *mut MsvcString;
            free_msvc_string_buf(s);
            crate::ucrt::heap::free(s as *mut c_void);
        }
        std::ptr::write_unaligned(this, empty_addr);
    }
}

// --- 2. RepeatedPtrFieldBase ---

unsafe extern "win64" fn repeated_ptr_field_base_reserve(
    this: *mut RepeatedPtrFieldBase,
    new_size: i32,
) {
    if this.is_null() {
        return;
    }
    let mut rf = std::ptr::read_unaligned(this);
    if new_size > rf.total_size {
        let new_cap = new_size.max(rf.total_size.saturating_mul(2)).max(4);
        let bytes = 8usize + (new_cap as usize) * 8;
        let new_rep = if rf.arena != 0 {
            arena_alloc_tracked(rf.arena as *mut u8, bytes, 8, 0)
        } else {
            crate::ucrt::heap::calloc(1, bytes) as *mut u8
        };
        if !new_rep.is_null() {
            if !rf.rep.is_null() {
                let old_alloc = std::ptr::read_unaligned(rf.rep as *const i32);
                std::ptr::write_unaligned(new_rep as *mut i32, old_alloc);
                let copy_count = old_alloc.max(rf.current_size).min(rf.total_size);
                if copy_count > 0 {
                    std::ptr::copy_nonoverlapping(
                        rf.rep.add(8),
                        new_rep.add(8),
                        (copy_count as usize) * 8,
                    );
                }
                if rf.arena == 0 {
                    crate::ucrt::heap::free(rf.rep as *mut c_void);
                }
            }
            rf.rep = new_rep;
            rf.total_size = new_cap;
            std::ptr::write_unaligned(this, rf);
        }
    }
}

unsafe extern "win64" fn repeated_ptr_field_base_dtor(this: *mut RepeatedPtrFieldBase) {
    if this.is_null() {
        return;
    }
    let mut rf = std::ptr::read_unaligned(this);
    if rf.arena == 0 && !rf.rep.is_null() {
        crate::ucrt::heap::free(rf.rep as *mut c_void);
    }
    rf.rep = std::ptr::null_mut();
    rf.current_size = 0;
    rf.total_size = 0;
    std::ptr::write_unaligned(this, rf);
}

unsafe extern "win64" fn repeated_ptr_field_base_internal_swap(
    this: *mut RepeatedPtrFieldBase,
    other: *mut RepeatedPtrFieldBase,
) {
    if !this.is_null() && !other.is_null() && this != other {
        let mut a = std::ptr::read_unaligned(this);
        let mut b = std::ptr::read_unaligned(other);
        std::mem::swap(&mut a.current_size, &mut b.current_size);
        std::mem::swap(&mut a.total_size, &mut b.total_size);
        std::mem::swap(&mut a.rep, &mut b.rep);
        std::ptr::write_unaligned(this, a);
        std::ptr::write_unaligned(other, b);
    }
}

unsafe extern "win64" fn repeated_ptr_field_base_merge_from_internal(
    _this: *mut RepeatedPtrFieldBase,
    _other: *const RepeatedPtrFieldBase,
    _merger: usize,
) {
}

// --- 3. RepeatedField<T> (generic push + u64 / f64 exports) ---

unsafe fn repeated_field_push<T: Copy>(rf_ptr: *mut RepeatedFieldRaw, val: T) {
    if rf_ptr.is_null() {
        return;
    }
    let mut rf = std::ptr::read_unaligned(rf_ptr);
    let arena = if rf.total_size == 0 {
        rf.ptr as usize
    } else if !rf.ptr.is_null() {
        std::ptr::read_unaligned(rf.ptr.sub(8) as *const usize)
    } else {
        0
    };
    let elem_size = std::mem::size_of::<T>();
    if rf.current_size >= rf.total_size {
        let new_cap = rf.total_size.saturating_mul(2).max(4);
        let bytes = 8usize + (new_cap as usize) * elem_size;
        let alloc = if arena != 0 {
            arena_alloc_tracked(arena as *mut u8, bytes, 8, 0)
        } else {
            crate::ucrt::heap::calloc(1, bytes) as *mut u8
        };
        if alloc.is_null() {
            return;
        }
        std::ptr::write_unaligned(alloc as *mut usize, arena);
        let new_ptr = alloc.add(8);
        if rf.total_size > 0 && !rf.ptr.is_null() {
            if rf.current_size > 0 {
                std::ptr::copy_nonoverlapping(
                    rf.ptr,
                    new_ptr,
                    (rf.current_size as usize) * elem_size,
                );
            }
            if arena == 0 {
                crate::ucrt::heap::free(rf.ptr.sub(8) as *mut c_void);
            }
        }
        rf.ptr = new_ptr;
        rf.total_size = new_cap;
    }
    let dst = rf.ptr.add((rf.current_size as usize) * elem_size) as *mut T;
    std::ptr::write_unaligned(dst, val);
    rf.current_size += 1;
    std::ptr::write_unaligned(rf_ptr, rf);
}

unsafe extern "win64" fn repeated_field_ctor(
    this: *mut RepeatedField64,
    arena: *mut u64,
) -> *mut RepeatedField64 {
    if !this.is_null() {
        std::ptr::write_unaligned(
            this,
            RepeatedField64 {
                current_size: 0,
                total_size: 0,
                ptr: arena,
            },
        );
    }
    this
}

unsafe extern "win64" fn repeated_field_dtor(this: *mut RepeatedField64) {
    if this.is_null() {
        return;
    }
    let rf = std::ptr::read_unaligned(this);
    if rf.total_size > 0 && !rf.ptr.is_null() {
        let header = (rf.ptr as *mut u8).sub(8);
        let arena = std::ptr::read_unaligned(header as *const usize);
        if arena == 0 {
            crate::ucrt::heap::free(header as *mut c_void);
        }
    }
    std::ptr::write_unaligned(
        this,
        RepeatedField64 {
            current_size: 0,
            total_size: 0,
            ptr: std::ptr::null_mut(),
        },
    );
}

unsafe extern "win64" fn repeated_field_size(this: *const RepeatedField64) -> i32 {
    if this.is_null() {
        0
    } else {
        std::ptr::read_unaligned(this).current_size
    }
}

unsafe extern "win64" fn repeated_field_clear(this: *mut RepeatedField64) {
    if !this.is_null() {
        let mut rf = std::ptr::read_unaligned(this);
        rf.current_size = 0;
        std::ptr::write_unaligned(this, rf);
    }
}

unsafe extern "win64" fn repeated_field_data(this: *const RepeatedField64) -> *const u64 {
    if this.is_null() {
        return std::ptr::null();
    }
    let rf = std::ptr::read_unaligned(this);
    if rf.total_size > 0 {
        rf.ptr
    } else {
        std::ptr::null()
    }
}

unsafe extern "win64" fn repeated_field_add(this: *mut RepeatedField64, val_ptr: *const u64) {
    if this.is_null() {
        return;
    }
    let val = if val_ptr.is_null() {
        0u64
    } else {
        std::ptr::read_unaligned(val_ptr)
    };
    repeated_field_push::<u64>(this as *mut RepeatedFieldRaw, val);
}

unsafe extern "win64" fn repeated_field_merge_from(
    this: *mut RepeatedField64,
    other: *const RepeatedField64,
) {
    if this.is_null() || other.is_null() || (this as *const RepeatedField64) == other {
        return;
    }
    let other_rf = std::ptr::read_unaligned(other);
    if other_rf.current_size > 0 && other_rf.total_size > 0 && !other_rf.ptr.is_null() {
        for i in 0..(other_rf.current_size as usize) {
            let v = std::ptr::read_unaligned(other_rf.ptr.add(i));
            repeated_field_push::<u64>(this as *mut RepeatedFieldRaw, v);
        }
    }
}

// --- 4. Core Wire-Format Primitives ---

#[inline]
fn varint32_size(v: u32) -> usize {
    let log2 = 31 - (v | 1).leading_zeros();
    ((log2 * 9 + 73) / 64) as usize
}

#[inline]
fn varint64_size(v: u64) -> usize {
    let log2 = 63 - (v | 1).leading_zeros();
    ((log2 * 9 + 73) / 64) as usize
}

#[inline]
fn zigzag_encode32(v: i32) -> u32 {
    ((v << 1) ^ (v >> 31)) as u32
}

#[inline]
fn zigzag_decode32(v: u32) -> i32 {
    ((v >> 1) as i32) ^ -((v & 1) as i32)
}

#[inline]
fn zigzag_encode64(v: i64) -> u64 {
    ((v << 1) ^ (v >> 63)) as u64
}

#[inline]
fn zigzag_decode64(v: u64) -> i64 {
    ((v >> 1) as i64) ^ -((v & 1) as i64)
}

#[inline]
fn make_tag(field_number: i32, wire_type: u32) -> u32 {
    ((field_number as u32) << 3) | (wire_type & 7)
}

#[inline]
unsafe fn write_varint32_raw(mut v: u32, mut target: *mut u8) -> *mut u8 {
    if target.is_null() {
        return target;
    }
    while v >= 0x80 {
        std::ptr::write(target, (v as u8) | 0x80);
        v >>= 7;
        target = target.add(1);
    }
    std::ptr::write(target, v as u8);
    target.add(1)
}

#[inline]
unsafe fn write_varint64_raw(mut v: u64, mut target: *mut u8) -> *mut u8 {
    if target.is_null() {
        return target;
    }
    while v >= 0x80 {
        std::ptr::write(target, (v as u8) | 0x80);
        v >>= 7;
        target = target.add(1);
    }
    std::ptr::write(target, v as u8);
    target.add(1)
}

#[inline]
unsafe fn write_tag_raw(field_number: i32, wire_type: u32, target: *mut u8) -> *mut u8 {
    write_varint32_raw(make_tag(field_number, wire_type), target)
}

#[inline]
unsafe fn write_fixed32_raw(v: u32, target: *mut u8) -> *mut u8 {
    if target.is_null() {
        return target;
    }
    std::ptr::write_unaligned(target as *mut [u8; 4], v.to_le_bytes());
    target.add(4)
}

#[inline]
unsafe fn write_fixed64_raw(v: u64, target: *mut u8) -> *mut u8 {
    if target.is_null() {
        return target;
    }
    std::ptr::write_unaligned(target as *mut [u8; 8], v.to_le_bytes());
    target.add(8)
}

#[inline]
unsafe fn decode_varint32_raw(p: *const u8) -> Option<(*const u8, u32)> {
    if p.is_null() {
        return None;
    }
    let mut res = 0u32;
    for i in 0..5usize {
        let b = std::ptr::read(p.add(i));
        res |= ((b & 0x7f) as u32).wrapping_shl((7 * i) as u32);
        if b < 0x80 {
            return Some((p.add(i + 1), res));
        }
    }
    // Consume sign-extended bytes 5..10 (e.g. negative int32/enum encoded as 10-byte varint).
    for i in 5..10usize {
        let b = std::ptr::read(p.add(i));
        if b < 0x80 {
            return Some((p.add(i + 1), res));
        }
    }
    None
}

#[inline]
unsafe fn decode_varint64_raw(p: *const u8) -> Option<(*const u8, u64)> {
    if p.is_null() {
        return None;
    }
    let mut res = 0u64;
    for i in 0..10usize {
        let b = std::ptr::read(p.add(i));
        res |= ((b & 0x7f) as u64).wrapping_shl((7 * i) as u32);
        if b < 0x80 {
            return Some((p.add(i + 1), res));
        }
    }
    None
}

#[inline]
unsafe fn decode_tag_raw(p: *const u8) -> Option<(*const u8, u32)> {
    if p.is_null() {
        return None;
    }
    let mut res = 0u32;
    for i in 0..5usize {
        let b = std::ptr::read(p.add(i));
        res |= ((b & 0x7f) as u32).wrapping_shl((7 * i) as u32);
        if b < 0x80 {
            return Some((p.add(i + 1), res));
        }
    }
    None
}

#[inline]
unsafe fn decode_size_raw(p: *const u8) -> Option<(*const u8, i32)> {
    if p.is_null() {
        return None;
    }
    let mut res = 0u32;
    for i in 0..4usize {
        let b = std::ptr::read(p.add(i));
        res |= ((b & 0x7f) as u32) << (7 * i);
        if b < 0x80 {
            return Some((p.add(i + 1), res as i32));
        }
    }
    let b4 = std::ptr::read(p.add(4));
    if b4 >= 8 {
        return None;
    }
    res |= (b4 as u32) << 28;
    if res > (i32::MAX as u32 - EPS_SLOP_BYTES as u32) {
        return None;
    }
    Some((p.add(5), res as i32))
}

#[inline]
fn decode_varint64_slice(slice: &[u8], pos: &mut usize) -> Option<u64> {
    let mut res = 0u64;
    for i in 0..10usize {
        if *pos >= slice.len() {
            return None;
        }
        let b = slice[*pos];
        *pos += 1;
        res |= ((b & 0x7f) as u64).wrapping_shl((7 * i) as u32);
        if b < 0x80 {
            return Some(res);
        }
    }
    None
}

// --- 5. CachedSize & UnknownFieldSet ---

#[derive(Clone)]
enum UnknownFieldValue {
    Varint(u64),
    Fixed64(u64),
    LengthDelimited(Vec<u8>),
    Group(Vec<UnknownFieldEntry>),
    Fixed32(u32),
}

#[derive(Clone)]
struct UnknownFieldEntry {
    field_number: u32,
    value: UnknownFieldValue,
}

fn unknown_fields_map() -> &'static Mutex<HashMap<usize, Vec<UnknownFieldEntry>>> {
    static MAP: OnceLock<Mutex<HashMap<usize, Vec<UnknownFieldEntry>>>> = OnceLock::new();
    MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

fn unknown_fields_byte_size(entries: &[UnknownFieldEntry]) -> usize {
    let mut total = 0usize;
    for e in entries {
        match &e.value {
            UnknownFieldValue::Varint(v) => {
                total += varint32_size((e.field_number << 3) | WIRETYPE_VARINT);
                total += varint64_size(*v);
            }
            UnknownFieldValue::Fixed64(_) => {
                total += varint32_size((e.field_number << 3) | WIRETYPE_FIXED64) + 8;
            }
            UnknownFieldValue::LengthDelimited(bytes) => {
                total += varint32_size((e.field_number << 3) | WIRETYPE_LENGTH_DELIMITED);
                total += varint32_size(bytes.len() as u32) + bytes.len();
            }
            UnknownFieldValue::Group(sub) => {
                total += varint32_size((e.field_number << 3) | WIRETYPE_START_GROUP);
                total += unknown_fields_byte_size(sub);
                total += varint32_size((e.field_number << 3) | WIRETYPE_END_GROUP);
            }
            UnknownFieldValue::Fixed32(_) => {
                total += varint32_size((e.field_number << 3) | WIRETYPE_FIXED32) + 4;
            }
        }
    }
    total
}

unsafe fn serialize_unknown_fields_into(
    entries: &[UnknownFieldEntry],
    mut target: *mut u8,
    stream: *mut EpsCopyOutputStream,
) -> *mut u8 {
    for e in entries {
        let mut scratch = [0u8; 20];
        match &e.value {
            UnknownFieldValue::Varint(v) => {
                let p = write_tag_raw(e.field_number as i32, WIRETYPE_VARINT, scratch.as_mut_ptr());
                let p = write_varint64_raw(*v, p);
                let len = p as usize - scratch.as_ptr() as usize;
                target = eps_write_raw_impl(stream, scratch.as_ptr(), len, target);
            }
            UnknownFieldValue::Fixed64(v) => {
                let p = write_tag_raw(e.field_number as i32, WIRETYPE_FIXED64, scratch.as_mut_ptr());
                let p = write_fixed64_raw(*v, p);
                let len = p as usize - scratch.as_ptr() as usize;
                target = eps_write_raw_impl(stream, scratch.as_ptr(), len, target);
            }
            UnknownFieldValue::LengthDelimited(bytes) => {
                let p = write_tag_raw(
                    e.field_number as i32,
                    WIRETYPE_LENGTH_DELIMITED,
                    scratch.as_mut_ptr(),
                );
                let p = write_varint32_raw(bytes.len() as u32, p);
                let hdr_len = p as usize - scratch.as_ptr() as usize;
                target = eps_write_raw_impl(stream, scratch.as_ptr(), hdr_len, target);
                if !bytes.is_empty() {
                    target = eps_write_raw_impl(stream, bytes.as_ptr(), bytes.len(), target);
                }
            }
            UnknownFieldValue::Group(sub) => {
                let p = write_tag_raw(
                    e.field_number as i32,
                    WIRETYPE_START_GROUP,
                    scratch.as_mut_ptr(),
                );
                let start_len = p as usize - scratch.as_ptr() as usize;
                target = eps_write_raw_impl(stream, scratch.as_ptr(), start_len, target);
                target = serialize_unknown_fields_into(sub, target, stream);
                let p2 = write_tag_raw(
                    e.field_number as i32,
                    WIRETYPE_END_GROUP,
                    scratch.as_mut_ptr(),
                );
                let end_len = p2 as usize - scratch.as_ptr() as usize;
                target = eps_write_raw_impl(stream, scratch.as_ptr(), end_len, target);
            }
            UnknownFieldValue::Fixed32(v) => {
                let p = write_tag_raw(e.field_number as i32, WIRETYPE_FIXED32, scratch.as_mut_ptr());
                let p = write_fixed32_raw(*v, p);
                let len = p as usize - scratch.as_ptr() as usize;
                target = eps_write_raw_impl(stream, scratch.as_ptr(), len, target);
            }
        }
    }
    target
}

unsafe extern "win64" fn cached_size_ctor(this: *mut i32) -> *mut i32 {
    if !this.is_null() {
        std::ptr::write_unaligned(this, 0);
    }
    this
}

unsafe extern "win64" fn cached_size_get(this: *const i32) -> i32 {
    if this.is_null() {
        0
    } else {
        std::ptr::read_unaligned(this)
    }
}

unsafe extern "win64" fn cached_size_set(this: *mut i32, val: i32) {
    if !this.is_null() {
        std::ptr::write_unaligned(this, val);
    }
}

unsafe extern "win64" fn unknown_field_set_ctor(this: *mut [usize; 2]) -> *mut [usize; 2] {
    if !this.is_null() {
        std::ptr::write_unaligned(this, [0, 0]);
        let mut map = unknown_fields_map().lock().unwrap_or_else(|e| e.into_inner());
        map.remove(&(this as usize));
    }
    this
}

unsafe extern "win64" fn unknown_field_set_dtor(this: *mut [usize; 2]) {
    if !this.is_null() {
        let mut map = unknown_fields_map().lock().unwrap_or_else(|e| e.into_inner());
        map.remove(&(this as usize));
    }
}

unsafe extern "win64" fn unknown_field_set_clear(this: *mut [usize; 2]) {
    if !this.is_null() {
        std::ptr::write_unaligned(this, [0, 0]);
        let mut map = unknown_fields_map().lock().unwrap_or_else(|e| e.into_inner());
        map.remove(&(this as usize));
    }
}

unsafe extern "win64" fn unknown_field_set_merge_from(
    this: *mut [usize; 2],
    other: *const [usize; 2],
) {
    if this.is_null() || other.is_null() || (this as *const [usize; 2]) == other {
        return;
    }
    let mut map = unknown_fields_map().lock().unwrap_or_else(|e| e.into_inner());
    let cloned = map.get(&(other as usize)).cloned();
    if let Some(entries) = cloned {
        if !entries.is_empty() {
            map.entry(this as usize).or_default().extend(entries);
        }
    }
}

unsafe extern "win64" fn unknown_field_set_swap(
    this: *mut [usize; 2],
    other: *mut [usize; 2],
) {
    if !this.is_null() && !other.is_null() && this != other {
        let a = std::ptr::read_unaligned(this);
        let b = std::ptr::read_unaligned(other);
        std::ptr::write_unaligned(this, b);
        std::ptr::write_unaligned(other, a);

        let mut map = unknown_fields_map().lock().unwrap_or_else(|e| e.into_inner());
        let va = map.remove(&(this as usize));
        let vb = map.remove(&(other as usize));
        if let Some(entries) = vb {
            map.insert(this as usize, entries);
        }
        if let Some(entries) = va {
            map.insert(other as usize, entries);
        }
    }
}

unsafe fn parse_unknown_field_entry(
    tag: u64,
    ptr: *const u8,
    ctx: *mut EpsCopyInputStream,
    depth: usize,
) -> Option<(*const u8, UnknownFieldEntry)> {
    if ptr.is_null() || depth > MAX_GROUP_DEPTH {
        return None;
    }
    let field_number = (tag >> 3) as u32;
    let wire_type = (tag & 7) as u32;
    if field_number == 0 {
        return None;
    }
    match wire_type {
        WIRETYPE_VARINT => {
            let (next, val) = ctx_read_varint64(ctx, ptr)?;
            Some((
                next,
                UnknownFieldEntry {
                    field_number,
                    value: UnknownFieldValue::Varint(val),
                },
            ))
        }
        WIRETYPE_FIXED64 => {
            let (next, bytes) = ctx_read_bytes(ctx, ptr, 8)?;
            let arr: [u8; 8] = bytes.as_slice().try_into().ok()?;
            Some((
                next,
                UnknownFieldEntry {
                    field_number,
                    value: UnknownFieldValue::Fixed64(u64::from_le_bytes(arr)),
                },
            ))
        }
        WIRETYPE_LENGTH_DELIMITED => {
            let (after_len, size) = ctx_read_size(ctx, ptr)?;
            if size < 0 {
                return None;
            }
            let (next, bytes) = ctx_read_bytes(ctx, after_len, size as usize)?;
            Some((
                next,
                UnknownFieldEntry {
                    field_number,
                    value: UnknownFieldValue::LengthDelimited(bytes),
                },
            ))
        }
        WIRETYPE_START_GROUP => {
            let end_tag = ((field_number as u64) << 3) | (WIRETYPE_END_GROUP as u64);
            let mut cur = ptr;
            let mut group_entries = Vec::new();
            loop {
                if !ctx.is_null() && parse_context_done(ctx as *mut u8, &mut cur) != 0 {
                    // Reached end of input before matching EndGroup tag.
                    return None;
                }
                let (after_tag, sub_tag) = ctx_read_tag(ctx, cur)?;
                if sub_tag == 0 {
                    return None;
                }
                if sub_tag as u64 == end_tag {
                    return Some((
                        after_tag,
                        UnknownFieldEntry {
                            field_number,
                            value: UnknownFieldValue::Group(group_entries),
                        },
                    ));
                }
                let (next_cur, entry) =
                    parse_unknown_field_entry(sub_tag as u64, after_tag, ctx, depth + 1)?;
                group_entries.push(entry);
                cur = next_cur;
            }
        }
        WIRETYPE_FIXED32 => {
            let (next, bytes) = ctx_read_bytes(ctx, ptr, 4)?;
            let arr: [u8; 4] = bytes.as_slice().try_into().ok()?;
            Some((
                next,
                UnknownFieldEntry {
                    field_number,
                    value: UnknownFieldValue::Fixed32(u32::from_le_bytes(arr)),
                },
            ))
        }
        _ => None, // WIRETYPE_END_GROUP (stray) or invalid wire types 6/7
    }
}

unsafe extern "win64" fn unknown_field_parse(
    tag: u64,
    unknown: *mut u8,
    ptr: *const u8,
    ctx: *mut u8,
) -> *const u8 {
    let Some((next, entry)) =
        parse_unknown_field_entry(tag, ptr, ctx as *mut EpsCopyInputStream, 0)
    else {
        return std::ptr::null();
    };
    if !unknown.is_null() {
        let mut map = unknown_fields_map().lock().unwrap_or_else(|e| e.into_inner());
        map.entry(unknown as usize).or_default().push(entry);
    }
    next
}

unsafe extern "win64" fn internal_serialize_unknown_fields_to_array(
    unknown: *const u8,
    target: *mut u8,
    stream: *mut EpsCopyOutputStream,
) -> *mut u8 {
    if unknown.is_null() || target.is_null() {
        return target;
    }
    let entries = {
        let map = unknown_fields_map().lock().unwrap_or_else(|e| e.into_inner());
        map.get(&(unknown as usize)).cloned()
    };
    if let Some(entries) = entries {
        serialize_unknown_fields_into(&entries, target, stream)
    } else {
        target
    }
}

unsafe extern "win64" fn compute_unknown_fields_size(
    metadata: *const u8,
    total_size: usize,
    cached_size: *mut i32,
) -> usize {
    let mut total = total_size;
    if !metadata.is_null() {
        let raw = std::ptr::read_unaligned(metadata as *const usize);
        if (raw & 1) != 0 {
            // InternalMetadata container: +0x00 Arena*, +0x08 UnknownFieldSet
            let container = raw & !3;
            if container != 0 {
                let ufs_addr = container + 8;
                let map = unknown_fields_map().lock().unwrap_or_else(|e| e.into_inner());
                if let Some(entries) = map.get(&ufs_addr) {
                    total = total.saturating_add(unknown_fields_byte_size(entries));
                }
            }
        }
    }
    if !cached_size.is_null() {
        std::ptr::write_unaligned(cached_size, total as i32);
    }
    total
}

// --- 6. Message, MessageLite, Arena, Descriptors ---

#[inline]
unsafe fn msg_vtable_slot(this: *const u8, slot: usize) -> Option<usize> {
    if this.is_null() {
        return None;
    }
    let vptr = std::ptr::read_unaligned(this as *const usize);
    if vptr == 0 {
        return None;
    }
    let fn_ptr = std::ptr::read_unaligned((vptr as *const usize).add(slot));
    if fn_ptr == 0 {
        None
    } else {
        Some(fn_ptr)
    }
}

unsafe fn call_msg_clear(this: *mut u8) {
    if let Some(fn_ptr) = msg_vtable_slot(this as *const u8, VTABLE_SLOT_CLEAR) {
        crate::call_guest8(fn_ptr, this as u64, 0, 0, 0, 0, 0, 0, 0);
    }
}

unsafe fn call_msg_is_initialized(this: *const u8) -> bool {
    if let Some(fn_ptr) = msg_vtable_slot(this, VTABLE_SLOT_IS_INITIALIZED) {
        (crate::call_guest8(fn_ptr, this as u64, 0, 0, 0, 0, 0, 0, 0) as u8) != 0
    } else {
        true
    }
}

unsafe fn call_msg_byte_size_long(this: *const u8) -> usize {
    if let Some(fn_ptr) = msg_vtable_slot(this, VTABLE_SLOT_BYTE_SIZE_LONG) {
        crate::call_guest8(fn_ptr, this as u64, 0, 0, 0, 0, 0, 0, 0) as usize
    } else {
        0
    }
}

unsafe fn call_msg_get_cached_size(this: *const u8) -> i32 {
    if let Some(fn_ptr) = msg_vtable_slot(this, VTABLE_SLOT_GET_CACHED_SIZE) {
        crate::call_guest8(fn_ptr, this as u64, 0, 0, 0, 0, 0, 0, 0) as i32
    } else {
        0
    }
}

unsafe fn call_msg_internal_parse(
    this: *mut u8,
    ptr: *const u8,
    ctx: *mut EpsCopyInputStream,
) -> Option<*const u8> {
    let fn_ptr = msg_vtable_slot(this as *const u8, VTABLE_SLOT_INTERNAL_PARSE)?;
    let ret = crate::call_guest8(fn_ptr, this as u64, ptr as u64, ctx as u64, 0, 0, 0, 0, 0);
    Some(ret as usize as *const u8)
}

unsafe fn call_msg_internal_serialize(
    this: *const u8,
    target: *mut u8,
    stream: *mut EpsCopyOutputStream,
) -> Option<*mut u8> {
    let fn_ptr = msg_vtable_slot(this, VTABLE_SLOT_INTERNAL_SERIALIZE)?;
    let ret = crate::call_guest8(fn_ptr, this as u64, target as u64, stream as u64, 0, 0, 0, 0, 0);
    Some(ret as usize as *mut u8)
}

unsafe extern "win64" fn msg_ctor_default(this: *mut usize) -> *mut usize {
    ensure_vtables_registered();
    if !this.is_null() {
        std::ptr::write_unaligned(this, &MESSAGE_VTBL.0 as *const _ as usize);
        std::ptr::write_unaligned(this.add(1), 0);
    }
    this
}

unsafe extern "win64" fn msg_ctor_arena(this: *mut usize, arena: usize) -> *mut usize {
    ensure_vtables_registered();
    if !this.is_null() {
        std::ptr::write_unaligned(this, &MESSAGE_VTBL.0 as *const _ as usize);
        std::ptr::write_unaligned(this.add(1), arena);
    }
    this
}

unsafe extern "win64" fn msg_dtor(_this: *mut usize) {}

unsafe extern "win64" fn msg_scalar_deleting_dtor(this: *mut u8, _flags: u32) -> *mut u8 {
    this
}

unsafe extern "win64" fn msg_new(_this: *const u8, _arena: *mut u8) -> *mut u8 {
    std::ptr::null_mut()
}

unsafe extern "win64" fn msg_ret_null(_this: *const u8) -> *const u8 {
    std::ptr::null()
}

unsafe extern "win64" fn msg_noop(_this: *mut u8) {}

unsafe extern "win64" fn msg_is_initialized(_this: *const u8) -> u8 {
    1
}

unsafe extern "win64" fn msg_get_cached_size(_this: *const u8) -> i32 {
    0
}

unsafe extern "win64" fn msg_set_cached_size(_this: *const u8, _size: i32) {}

unsafe extern "win64" fn msg_internal_parse(
    _this: *mut u8,
    mut ptr: *const u8,
    ctx: *mut u8,
) -> *const u8 {
    if ptr.is_null() || ctx.is_null() {
        return ptr;
    }
    let eps = ctx as *mut EpsCopyInputStream;
    while parse_context_done(ctx, &mut ptr) == 0 {
        let Some((after_tag, tag)) = ctx_read_tag(eps, ptr) else {
            return std::ptr::null();
        };
        if tag == 0 {
            return std::ptr::null();
        }
        let next = unknown_field_parse(tag as u64, std::ptr::null_mut(), after_tag, ctx);
        if next.is_null() {
            return std::ptr::null();
        }
        ptr = next;
    }
    ptr
}

unsafe extern "win64" fn msg_internal_serialize(
    _this: *const u8,
    target: *mut u8,
    _stream: *mut u8,
) -> *mut u8 {
    target
}

unsafe extern "win64" fn msg_get_metadata(
    _this: *const u8,
    sret: *mut [usize; 2],
) -> *mut [usize; 2] {
    if !sret.is_null() {
        std::ptr::write_unaligned(sret, [0, &DUMMY_REFLECTION as *const _ as usize]);
    }
    sret
}

unsafe extern "win64" fn msg_init_error_string(
    _this: *const u8,
    sret: *mut MsvcString,
) -> *mut MsvcString {
    init_empty_string(sret);
    sret
}

unsafe extern "win64" fn msg_get_type_name(
    _this: *const u8,
    sret: *mut MsvcString,
) -> *mut MsvcString {
    init_empty_string(sret);
    sret
}

unsafe extern "win64" fn msg_utf8_debug_string(
    _this: *const u8,
    sret: *mut MsvcString,
) -> *mut MsvcString {
    init_empty_string(sret);
    sret
}

unsafe extern "win64" fn msg_discard_unknown_fields(_this: *mut u8) {}

unsafe extern "win64" fn msg_space_used_long(_this: *const u8) -> usize {
    0
}

unsafe extern "win64" fn msg_check_type_and_merge(_this: *mut u8, _other: *const u8) {}

unsafe extern "win64" fn msg_internal_get_table(_this: *const u8) -> *const u8 {
    std::ptr::null()
}

unsafe extern "win64" fn msg_get_reflection(_this: *const u8) -> *const u8 {
    &DUMMY_REFLECTION as *const DummyReflection as *const u8
}

unsafe extern "win64" fn message_lite_get_arena(this: *const u8) -> *mut u8 {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let ptr = std::ptr::read_unaligned(this.add(8) as *const usize);
    if (ptr & 3) == 0 {
        return (ptr & !3) as *mut u8;
    }
    if ((ptr >> 1) & 1) != 0 {
        return std::ptr::null_mut();
    }
    let container = (ptr & !3) as *const usize;
    if container.is_null() {
        std::ptr::null_mut()
    } else {
        std::ptr::read_unaligned(container) as *mut u8
    }
}

#[inline]
unsafe fn init_flat_output_stream(
    target: *mut u8,
    known_size: usize,
) -> EpsCopyOutputStream {
    let end = if target.is_null() || known_size == 0 {
        std::ptr::null_mut()
    } else if known_size >= EPS_SLOP_BYTES {
        target.add(known_size - EPS_SLOP_BYTES)
    } else {
        target.add(known_size)
    };
    EpsCopyOutputStream {
        end,
        stream: std::ptr::null_mut(),
        had_error: 0,
        aliasing_enabled: 0,
        is_deterministic: 0,
        _pad: [0; 5],
        buffer: [0; 32],
    }
}

unsafe extern "win64" fn message_lite_serialize_with_cached_sizes_to_array(
    this: *const u8,
    target: *mut u8,
) -> *mut u8 {
    if this.is_null() || target.is_null() {
        return target;
    }
    let cached = call_msg_get_cached_size(this);
    let known_size = if cached > 0 {
        cached as usize
    } else {
        call_msg_byte_size_long(this)
    };
    let mut eps = init_flat_output_stream(target, known_size);
    match call_msg_internal_serialize(this, target, &mut eps) {
        Some(end_ptr) if !end_ptr.is_null() => end_ptr,
        _ => target,
    }
}

unsafe fn serialize_message_to_vec(this: *const u8, check_initialized: bool) -> Option<Vec<u8>> {
    if this.is_null() {
        return None;
    }
    if check_initialized && !call_msg_is_initialized(this) {
        return None;
    }
    let byte_size = call_msg_byte_size_long(this);
    if byte_size > i32::MAX as usize {
        return None;
    }
    if byte_size == 0 {
        return Some(Vec::new());
    }
    // Allocate extra slop bytes so generated serializers that assume kSlopBytes are safe.
    let mut buf = vec![0u8; byte_size + EPS_SLOP_BYTES];
    let mut eps = init_flat_output_stream(buf.as_mut_ptr(), byte_size);
    let end_ptr = call_msg_internal_serialize(this, buf.as_mut_ptr(), &mut eps)?;
    if end_ptr.is_null() || eps.had_error != 0 {
        return None;
    }
    let base = buf.as_ptr() as usize;
    let end = end_ptr as usize;
    if end < base || end - base > byte_size {
        return None;
    }
    buf.truncate(end - base);
    Some(buf)
}

unsafe extern "win64" fn message_lite_serialize_to_string(
    this: *const u8,
    output: *mut MsvcString,
) -> u8 {
    if output.is_null() {
        return 0;
    }
    msvc_string_clear(output);
    match serialize_message_to_vec(this, true) {
        Some(bytes) => {
            msvc_string_assign_bytes(output, &bytes);
            1
        }
        None => 0,
    }
}

unsafe extern "win64" fn message_lite_serialize_partial_to_string(
    this: *const u8,
    output: *mut MsvcString,
) -> u8 {
    if output.is_null() {
        return 0;
    }
    msvc_string_clear(output);
    match serialize_message_to_vec(this, false) {
        Some(bytes) => {
            msvc_string_assign_bytes(output, &bytes);
            1
        }
        None => 0,
    }
}

unsafe extern "win64" fn message_lite_append_to_string(
    this: *const u8,
    output: *mut MsvcString,
) -> u8 {
    if output.is_null() {
        return 0;
    }
    match serialize_message_to_vec(this, true) {
        Some(bytes) => {
            msvc_string_append_bytes(output, &bytes);
            1
        }
        None => 0,
    }
}

unsafe fn serialize_message_to_array_impl(
    this: *const u8,
    data: *mut u8,
    size: i32,
    check_initialized: bool,
) -> u8 {
    if this.is_null() || size < 0 || (size > 0 && data.is_null()) {
        return 0;
    }
    if check_initialized && !call_msg_is_initialized(this) {
        return 0;
    }
    let byte_size = call_msg_byte_size_long(this);
    if byte_size > size as usize {
        return 0;
    }
    if byte_size == 0 {
        return 1;
    }
    let mut eps = init_flat_output_stream(data, size as usize);
    let Some(end_ptr) = call_msg_internal_serialize(this, data, &mut eps) else {
        return 0;
    };
    if end_ptr.is_null() || eps.had_error != 0 {
        return 0;
    }
    let base = data as usize;
    let end = end_ptr as usize;
    if end < base || end - base > size as usize {
        return 0;
    }
    1
}

unsafe extern "win64" fn message_lite_serialize_to_array(
    this: *const u8,
    data: *mut u8,
    size: i32,
) -> u8 {
    serialize_message_to_array_impl(this, data, size, true)
}

unsafe extern "win64" fn message_lite_serialize_partial_to_array(
    this: *const u8,
    data: *mut u8,
    size: i32,
) -> u8 {
    serialize_message_to_array_impl(this, data, size, false)
}

unsafe fn parse_message_from_array_impl(
    this: *mut u8,
    data: *const u8,
    size: i32,
    check_initialized: bool,
) -> u8 {
    if this.is_null() || size < 0 || (size > 0 && data.is_null()) {
        return 0;
    }
    call_msg_clear(this);
    if size == 0 {
        return if !check_initialized || call_msg_is_initialized(this as *const u8) {
            1
        } else {
            0
        };
    }
    let mut ctx = EpsCopyInputStream {
        limit_end: std::ptr::null(),
        buffer_end: std::ptr::null(),
        next_chunk: std::ptr::null(),
        size: 0,
        limit: 0,
        zcis: std::ptr::null_mut(),
        buffer: [0; 32],
        aliasing: 0,
        depth: MAX_GROUP_DEPTH as i32,
    };
    let start_ptr = init_parse_context(&mut ctx, data, size as usize);
    let Some(mut end_ptr) = call_msg_internal_parse(this, start_ptr, &mut ctx) else {
        return 0;
    };
    if end_ptr.is_null() {
        return 0;
    }
    // Verify the parser consumed cleanly up to the end without overrunning.
    if parse_context_done(&mut ctx as *mut EpsCopyInputStream as *mut u8, &mut end_ptr) == 0
        || end_ptr.is_null()
    {
        return 0;
    }
    if check_initialized && !call_msg_is_initialized(this as *const u8) {
        return 0;
    }
    1
}

unsafe extern "win64" fn message_lite_parse_from_array(
    this: *mut u8,
    data: *const u8,
    size: i32,
) -> u8 {
    parse_message_from_array_impl(this, data, size, true)
}

unsafe extern "win64" fn message_lite_parse_partial_from_array(
    this: *mut u8,
    data: *const u8,
    size: i32,
) -> u8 {
    parse_message_from_array_impl(this, data, size, false)
}

unsafe extern "win64" fn message_lite_parse_from_string(
    this: *mut u8,
    data: *const MsvcString,
) -> u8 {
    if data.is_null() {
        return 0;
    }
    let (ptr, len) = msvc_string_data_and_len(data);
    if len > i32::MAX as usize {
        return 0;
    }
    parse_message_from_array_impl(this, ptr, len as i32, true)
}

unsafe extern "win64" fn message_lite_parse_partial_from_string(
    this: *mut u8,
    data: *const MsvcString,
) -> u8 {
    if data.is_null() {
        return 0;
    }
    let (ptr, len) = msvc_string_data_and_len(data);
    if len > i32::MAX as usize {
        return 0;
    }
    parse_message_from_array_impl(this, ptr, len as i32, false)
}

unsafe extern "win64" fn message_lite_parse_from_string_piece(
    this: *mut u8,
    sp: *const [usize; 2],
) -> u8 {
    if sp.is_null() {
        return 0;
    }
    let [ptr, len] = std::ptr::read_unaligned(sp);
    if len > i32::MAX as usize {
        return 0;
    }
    parse_message_from_array_impl(this, ptr as *const u8, len as i32, true)
}

// --- Arena exports ---

unsafe extern "win64" fn arena_ctor(this: *mut u8) -> *mut u8 {
    ensure_vtables_registered();
    if !this.is_null() {
        let mut map = arenas().lock().unwrap_or_else(|e| e.into_inner());
        map.entry(this as usize).or_default();
    }
    this
}

unsafe extern "win64" fn arena_ctor_options(this: *mut u8, _options: *const u8) -> *mut u8 {
    arena_ctor(this)
}

unsafe extern "win64" fn arena_init(this: *mut u8, _record_allocs: u8) {
    let _ = arena_ctor(this);
}

unsafe extern "win64" fn arena_dtor(this: *mut u8) {
    let _ = arena_destroy_state(this);
}

unsafe extern "win64" fn arena_reset(this: *mut u8) -> usize {
    arena_destroy_state(this)
}

unsafe extern "win64" fn arena_space_allocated(this: *const u8) -> usize {
    if this.is_null() {
        return 0;
    }
    let map = arenas().lock().unwrap_or_else(|e| e.into_inner());
    map.get(&(this as usize)).map_or(0, |s| s.total_bytes)
}

unsafe extern "win64" fn arena_allocate_internal(
    this: *mut u8,
    size: usize,
    align: usize,
    dtor: usize,
    _type_info: *const u8,
) -> *mut u8 {
    arena_alloc_tracked(this, size.max(16), align, dtor)
}

unsafe extern "win64" fn arena_allocate_aligned(
    this: *mut u8,
    size: usize,
    align: usize,
) -> *mut u8 {
    arena_alloc_tracked(this, size.max(1), align, 0)
}

unsafe extern "win64" fn arena_own_destructor(this: *mut u8, object: *mut u8, dtor: usize) {
    if this.is_null() || object.is_null() || dtor == 0 {
        return;
    }
    let mut map = arenas().lock().unwrap_or_else(|e| e.into_inner());
    let state = map.entry(this as usize).or_default();
    state.cleanups.push(ArenaCleanup { object, dtor });
}

unsafe extern "win64" fn assign_descriptors(_table: *const u8, _eager: u8) {
    ensure_vtables_registered();
}

unsafe extern "win64" fn add_descriptors_runner_ctor(
    this: *mut u8,
    _table: *const u8,
) -> *mut u8 {
    ensure_vtables_registered();
    this
}

unsafe extern "win64" fn reflection_ops_merge(_from: *const u8, _to: *mut u8) {}

// --- 7. WireFormatLite, CodedOutputStream, EpsCopyOutputStream ---

unsafe extern "win64" fn coded_write_varint32_to_array(value: u32, target: *mut u8) -> *mut u8 {
    write_varint32_raw(value, target)
}

unsafe extern "win64" fn coded_write_varint64_to_array(value: u64, target: *mut u8) -> *mut u8 {
    write_varint64_raw(value, target)
}

unsafe extern "win64" fn wire_write_tag_to_array(
    field_number: i32,
    wire_type: u32,
    target: *mut u8,
) -> *mut u8 {
    write_tag_raw(field_number, wire_type, target)
}

unsafe extern "win64" fn wire_write_int32_to_array(
    field_number: i32,
    value: i32,
    target: *mut u8,
) -> *mut u8 {
    if target.is_null() {
        return target;
    }
    let t = write_tag_raw(field_number, WIRETYPE_VARINT, target);
    if value < 0 {
        write_varint64_raw(value as i64 as u64, t)
    } else {
        write_varint32_raw(value as u32, t)
    }
}

unsafe extern "win64" fn wire_write_uint32_to_array(
    field_number: i32,
    value: u32,
    target: *mut u8,
) -> *mut u8 {
    if target.is_null() {
        return target;
    }
    let t = write_tag_raw(field_number, WIRETYPE_VARINT, target);
    write_varint32_raw(value, t)
}

unsafe extern "win64" fn wire_write_int64_to_array(
    field_number: i32,
    value: i64,
    target: *mut u8,
) -> *mut u8 {
    if target.is_null() {
        return target;
    }
    let t = write_tag_raw(field_number, WIRETYPE_VARINT, target);
    write_varint64_raw(value as u64, t)
}

unsafe extern "win64" fn wire_write_uint64_to_array(
    field_number: i32,
    value: u64,
    target: *mut u8,
) -> *mut u8 {
    if target.is_null() {
        return target;
    }
    let t = write_tag_raw(field_number, WIRETYPE_VARINT, target);
    write_varint64_raw(value, t)
}

unsafe extern "win64" fn wire_write_sint32_to_array(
    field_number: i32,
    value: i32,
    target: *mut u8,
) -> *mut u8 {
    if target.is_null() {
        return target;
    }
    let t = write_tag_raw(field_number, WIRETYPE_VARINT, target);
    write_varint32_raw(zigzag_encode32(value), t)
}

unsafe extern "win64" fn wire_write_sint64_to_array(
    field_number: i32,
    value: i64,
    target: *mut u8,
) -> *mut u8 {
    if target.is_null() {
        return target;
    }
    let t = write_tag_raw(field_number, WIRETYPE_VARINT, target);
    write_varint64_raw(zigzag_encode64(value), t)
}

unsafe extern "win64" fn wire_write_bool_to_array(
    field_number: i32,
    value: u8,
    target: *mut u8,
) -> *mut u8 {
    if target.is_null() {
        return target;
    }
    let t = write_tag_raw(field_number, WIRETYPE_VARINT, target);
    std::ptr::write(t, if value != 0 { 1 } else { 0 });
    t.add(1)
}

unsafe extern "win64" fn wire_write_fixed32_to_array(
    field_number: i32,
    value: u32,
    target: *mut u8,
) -> *mut u8 {
    if target.is_null() {
        return target;
    }
    let t = write_tag_raw(field_number, WIRETYPE_FIXED32, target);
    write_fixed32_raw(value, t)
}

unsafe extern "win64" fn wire_write_sfixed32_to_array(
    field_number: i32,
    value: i32,
    target: *mut u8,
) -> *mut u8 {
    if target.is_null() {
        return target;
    }
    let t = write_tag_raw(field_number, WIRETYPE_FIXED32, target);
    write_fixed32_raw(value as u32, t)
}

unsafe extern "win64" fn wire_write_fixed64_to_array(
    field_number: i32,
    value: u64,
    target: *mut u8,
) -> *mut u8 {
    if target.is_null() {
        return target;
    }
    let t = write_tag_raw(field_number, WIRETYPE_FIXED64, target);
    write_fixed64_raw(value, t)
}

unsafe extern "win64" fn wire_write_sfixed64_to_array(
    field_number: i32,
    value: i64,
    target: *mut u8,
) -> *mut u8 {
    if target.is_null() {
        return target;
    }
    let t = write_tag_raw(field_number, WIRETYPE_FIXED64, target);
    write_fixed64_raw(value as u64, t)
}

unsafe extern "win64" fn wire_write_float_to_array(
    field_number: i32,
    value: f32,
    target: *mut u8,
) -> *mut u8 {
    if target.is_null() {
        return target;
    }
    let t = write_tag_raw(field_number, WIRETYPE_FIXED32, target);
    write_fixed32_raw(value.to_bits(), t)
}
crate::win64_bridge!(fn wire_write_float_to_array(i32, f32, *mut u8) -> *mut u8);

unsafe extern "win64" fn wire_write_double_to_array(
    field_number: i32,
    value: f64,
    target: *mut u8,
) -> *mut u8 {
    if target.is_null() {
        return target;
    }
    let t = write_tag_raw(field_number, WIRETYPE_FIXED64, target);
    write_fixed64_raw(value.to_bits(), t)
}
crate::win64_bridge!(fn wire_write_double_to_array(i32, f64, *mut u8) -> *mut u8);

unsafe extern "win64" fn wire_write_string_to_array(
    field_number: i32,
    value: *const MsvcString,
    target: *mut u8,
) -> *mut u8 {
    if target.is_null() {
        return target;
    }
    let (data, len) = msvc_string_data_and_len(value);
    let t = write_tag_raw(field_number, WIRETYPE_LENGTH_DELIMITED, target);
    let t = write_varint32_raw(len as u32, t);
    if len > 0 && !data.is_null() {
        std::ptr::copy_nonoverlapping(data, t, len);
    }
    t.add(len)
}

fn eps_slop_map() -> &'static Mutex<HashMap<usize, (usize, usize)>> {
    static MAP: OnceLock<Mutex<HashMap<usize, (usize, usize)>>> = OnceLock::new();
    MAP.get_or_init(|| Mutex::new(HashMap::new()))
}

unsafe fn eps_set_error_sink(this: *mut EpsCopyOutputStream) -> *mut u8 {
    let patch_ptr = (this as *mut u8).add(std::mem::offset_of!(EpsCopyOutputStream, buffer));
    let mut st = std::ptr::read_unaligned(this);
    st.had_error = 1;
    st.end = patch_ptr.add(EPS_SLOP_BYTES);
    std::ptr::write_unaligned(this, st);
    let mut map = eps_slop_map().lock().unwrap_or_else(|e| e.into_inner());
    map.remove(&(this as usize));
    patch_ptr
}

unsafe extern "win64" fn eps_ensure_space_fallback(
    this: *mut EpsCopyOutputStream,
    ptr: *mut u8,
) -> *mut u8 {
    if this.is_null() || ptr.is_null() {
        return ptr;
    }
    let st = std::ptr::read_unaligned(this);
    if st.had_error != 0 {
        return (this as *mut u8).add(std::mem::offset_of!(EpsCopyOutputStream, buffer));
    }
    if st.stream.is_null() {
        // Flat buffer: advance `end` across the final 16 slop bytes if needed, without failing.
        if !st.end.is_null() && (ptr as usize) >= (st.end as usize) {
            let mut updated = st;
            updated.end = ptr.add(EPS_SLOP_BYTES);
            std::ptr::write_unaligned(this, updated);
        }
        return ptr;
    }

    // Backed by a ZeroCopyOutputStream* (`+0x08`: Next, `+0x10`: BackUp).
    let zc = st.stream;
    let vptr = std::ptr::read_unaligned(zc as *const usize);
    if vptr == 0 {
        return eps_set_error_sink(this);
    }
    let next_fn = std::ptr::read_unaligned((vptr as *const usize).add(1));
    let backup_fn = std::ptr::read_unaligned((vptr as *const usize).add(2));
    if next_fn == 0 {
        return eps_set_error_sink(this);
    }

    let patch_ptr = (this as *mut u8).add(std::mem::offset_of!(EpsCopyOutputStream, buffer));
    let patch_start = patch_ptr as usize;
    let patch_end = patch_start + 32;
    let ptr_addr = ptr as usize;

    let tracked_slop = {
        let map = eps_slop_map().lock().unwrap_or_else(|e| e.into_inner());
        map.get(&(this as usize))
            .filter(|&&(recorded_end, _)| recorded_end == st.end as usize)
            .map(|&(_, slop)| slop)
    };

    let mut spill = [0u8; 32];
    let mut spill_len = 0usize;
    if ptr_addr >= patch_start && ptr_addr <= patch_end {
        spill_len = ptr_addr - patch_start;
        if spill_len > 0 {
            std::ptr::copy_nonoverlapping(patch_ptr, spill.as_mut_ptr(), spill_len);
        }
    } else if !st.end.is_null() && ptr_addr >= (st.end as usize) {
        let overrun = ptr_addr - (st.end as usize);
        let prev_slop = tracked_slop.unwrap_or_else(|| {
            if overrun > 0 && overrun < EPS_SLOP_BYTES {
                EPS_SLOP_BYTES
            } else {
                0
            }
        });
        if overrun < prev_slop && backup_fn != 0 {
            let unused = (prev_slop - overrun) as i32;
            crate::call_guest8(backup_fn, zc as u64, unused as u64, 0, 0, 0, 0, 0, 0);
        }
    }

    let mut new_data: *mut u8 = std::ptr::null_mut();
    let mut new_size: i32 = 0;
    let ok = (crate::call_guest8(
        next_fn,
        zc as u64,
        &mut new_data as *mut *mut u8 as u64,
        &mut new_size as *mut i32 as u64,
        0,
        0,
        0,
        0,
        0,
    ) as u8)
        != 0;
    if !ok || new_data.is_null() || new_size <= 0 {
        return eps_set_error_sink(this);
    }

    let cap = new_size as usize;
    let new_slop = if cap > EPS_SLOP_BYTES { EPS_SLOP_BYTES } else { 0 };
    let new_end = new_data.add(cap - new_slop);
    let mut updated = std::ptr::read_unaligned(this);
    updated.end = new_end;
    std::ptr::write_unaligned(this, updated);
    {
        let mut map = eps_slop_map().lock().unwrap_or_else(|e| e.into_inner());
        map.insert(this as usize, (new_end as usize, new_slop));
    }

    if spill_len > 0 {
        let copy = spill_len.min(cap);
        std::ptr::copy_nonoverlapping(spill.as_ptr(), new_data, copy);
        new_data.add(copy)
    } else {
        new_data
    }
}

unsafe fn eps_write_raw_impl(
    this: *mut EpsCopyOutputStream,
    src: *const u8,
    mut len: usize,
    mut ptr: *mut u8,
) -> *mut u8 {
    if ptr.is_null() || src.is_null() || len == 0 {
        return ptr;
    }
    if this.is_null() {
        std::ptr::copy_nonoverlapping(src, ptr, len);
        return ptr.add(len);
    }
    let st = std::ptr::read_unaligned(this);
    if st.had_error != 0 {
        return ptr;
    }
    if st.stream.is_null() {
        std::ptr::copy_nonoverlapping(src, ptr, len);
        return ptr.add(len);
    }

    let mut cur_src = src;
    while len > 0 {
        let cur_st = std::ptr::read_unaligned(this);
        if cur_st.had_error != 0 {
            return ptr;
        }
        let avail = if !cur_st.end.is_null() && (ptr as usize) < (cur_st.end as usize) {
            (cur_st.end as usize) - (ptr as usize)
        } else {
            0
        };
        if avail == 0 {
            ptr = eps_ensure_space_fallback(this, ptr);
            let after_st = std::ptr::read_unaligned(this);
            if after_st.had_error != 0 {
                return ptr;
            }
            continue;
        }
        let chunk = len.min(avail);
        std::ptr::copy_nonoverlapping(cur_src, ptr, chunk);
        cur_src = cur_src.add(chunk);
        ptr = ptr.add(chunk);
        len -= chunk;
    }
    ptr
}

unsafe extern "win64" fn eps_write_raw(
    this: *mut EpsCopyOutputStream,
    data: *const u8,
    size: i32,
    ptr: *mut u8,
) -> *mut u8 {
    if size <= 0 {
        return ptr;
    }
    eps_write_raw_impl(this, data, size as usize, ptr)
}

unsafe extern "win64" fn eps_write_length_delim(
    this: *mut EpsCopyOutputStream,
    num: i32,
    size: u32,
    ptr: *mut u8,
) -> *mut u8 {
    if ptr.is_null() {
        return ptr;
    }
    let mut hdr = [0u8; 10];
    let p = write_tag_raw(num, WIRETYPE_LENGTH_DELIMITED, hdr.as_mut_ptr());
    let p = write_varint32_raw(size, p);
    let hdr_len = p as usize - hdr.as_ptr() as usize;
    eps_write_raw_impl(this, hdr.as_ptr(), hdr_len, ptr)
}

unsafe extern "win64" fn eps_write_string_maybe_aliased(
    this: *mut EpsCopyOutputStream,
    num: u32,
    s: *const MsvcString,
    ptr: *mut u8,
) -> *mut u8 {
    if ptr.is_null() {
        return ptr;
    }
    let (data, len) = msvc_string_data_and_len(s);
    let mut hdr = [0u8; 10];
    let p = write_varint32_raw((num << 3) | WIRETYPE_LENGTH_DELIMITED, hdr.as_mut_ptr());
    let p = write_varint32_raw(len as u32, p);
    let hdr_len = p as usize - hdr.as_ptr() as usize;
    let ptr = eps_write_raw_impl(this, hdr.as_ptr(), hdr_len, ptr);
    if len > 0 && !data.is_null() {
        eps_write_raw_impl(this, data, len, ptr)
    } else {
        ptr
    }
}

unsafe extern "win64" fn eps_encode64(v: u64) -> u64 {
    v.to_le()
}

unsafe extern "win64" fn wire_uint32_size(v: u32) -> usize {
    varint32_size(v)
}

unsafe extern "win64" fn wire_int32_size(v: i32) -> usize {
    if v < 0 {
        10
    } else {
        varint32_size(v as u32)
    }
}

unsafe extern "win64" fn wire_sint32_size(v: i32) -> usize {
    varint32_size(zigzag_encode32(v))
}

unsafe extern "win64" fn wire_int64_size(v: i64) -> usize {
    varint64_size(v as u64)
}

unsafe extern "win64" fn wire_uint64_size(v: u64) -> usize {
    varint64_size(v)
}

unsafe extern "win64" fn wire_sint64_size(v: i64) -> usize {
    varint64_size(zigzag_encode64(v))
}

unsafe extern "win64" fn wire_length_delimited_size(len: usize) -> usize {
    len.saturating_add(varint32_size(len as u32))
}

unsafe extern "win64" fn wire_string_size(s: *const MsvcString) -> usize {
    let (_, len) = msvc_string_data_and_len(s);
    len.saturating_add(varint32_size(len as u32))
}

unsafe extern "win64" fn wire_uint64_repeated_size(rf: *const RepeatedField64) -> usize {
    if rf.is_null() {
        return 0;
    }
    let val = std::ptr::read_unaligned(rf);
    if val.current_size <= 0 || val.ptr.is_null() || val.total_size <= 0 {
        return 0;
    }
    let mut total = 0usize;
    for i in 0..(val.current_size as usize) {
        total += varint64_size(std::ptr::read_unaligned(val.ptr.add(i)));
    }
    total
}

// --- 8. UTF-8 verification ---

#[inline]
unsafe fn is_valid_utf8_raw(data: *const u8, len: usize) -> bool {
    if len == 0 {
        return true;
    }
    if data.is_null() {
        return false;
    }
    std::str::from_utf8(std::slice::from_raw_parts(data, len)).is_ok()
}

unsafe extern "win64" fn verify_utf8_string(
    data: *const u8,
    len: i32,
    _op: i32,
    _field_name: *const u8,
) -> u8 {
    if len < 0 {
        return 0;
    }
    if is_valid_utf8_raw(data, len as usize) {
        1
    } else {
        0
    }
}

unsafe extern "win64" fn verify_utf8(sp: *const [usize; 2], _field_name: *const u8) -> u8 {
    if sp.is_null() {
        return 0;
    }
    let [ptr, len] = std::ptr::read_unaligned(sp);
    if is_valid_utf8_raw(ptr as *const u8, len) {
        1
    } else {
        0
    }
}

// --- 9. EpsCopyInputStream, ParseContext, VarintParseSlow, Packed parsers ---

unsafe fn init_parse_context(
    ctx: *mut EpsCopyInputStream,
    data: *const u8,
    size: usize,
) -> *const u8 {
    let patch_ptr = (ctx as *mut u8).add(std::mem::offset_of!(EpsCopyInputStream, buffer));
    let mut st = EpsCopyInputStream {
        limit_end: std::ptr::null(),
        buffer_end: std::ptr::null(),
        next_chunk: std::ptr::null(),
        size: 0,
        limit: 0,
        zcis: std::ptr::null_mut(),
        buffer: [0; 32],
        aliasing: 0,
        depth: MAX_GROUP_DEPTH as i32,
    };
    if size > EPS_SLOP_BYTES {
        std::ptr::copy_nonoverlapping(
            data.add(size - EPS_SLOP_BYTES),
            st.buffer.as_mut_ptr(),
            EPS_SLOP_BYTES,
        );
        st.buffer_end = data.add(size - EPS_SLOP_BYTES);
        st.limit = EPS_SLOP_BYTES as i32;
        st.limit_end = st.buffer_end;
        st.next_chunk = patch_ptr as *const u8;
        st.size = EPS_SLOP_BYTES as i32;
        std::ptr::write_unaligned(ctx, st);
        data
    } else {
        if size > 0 && !data.is_null() {
            std::ptr::copy_nonoverlapping(data, st.buffer.as_mut_ptr(), size);
        }
        st.buffer_end = (patch_ptr as *const u8).add(size);
        st.limit = 0;
        st.limit_end = st.buffer_end;
        st.next_chunk = std::ptr::null();
        st.size = 0;
        std::ptr::write_unaligned(ctx, st);
        patch_ptr as *const u8
    }
}

#[inline]
unsafe fn ctx_remaining_limit(ctx: *const EpsCopyInputStream, ptr: *const u8) -> isize {
    let st = std::ptr::read_unaligned(ctx);
    if st.buffer_end.is_null() {
        return 0;
    }
    ((st.buffer_end as isize) - (ptr as isize)).saturating_add(st.limit as isize)
}

unsafe fn ctx_read_bytes(
    ctx: *mut EpsCopyInputStream,
    ptr: *const u8,
    len: usize,
) -> Option<(*const u8, Vec<u8>)> {
    if ptr.is_null() {
        return None;
    }
    if len == 0 {
        return Some((ptr, Vec::new()));
    }
    if ctx.is_null() {
        let mut out = vec![0u8; len];
        std::ptr::copy_nonoverlapping(ptr, out.as_mut_ptr(), len);
        return Some((ptr.add(len), out));
    }
    let mut st = std::ptr::read_unaligned(ctx);
    if st.buffer_end.is_null() {
        return None;
    }
    let rem_limit = ((st.buffer_end as isize) - (ptr as isize)).saturating_add(st.limit as isize);
    if rem_limit < 0 || (len as isize) > rem_limit {
        return None;
    }

    let mut out = vec![0u8; len];
    let avail_in_cur = (st.buffer_end as isize) - (ptr as isize);
    if avail_in_cur >= len as isize {
        std::ptr::copy_nonoverlapping(ptr, out.as_mut_ptr(), len);
        return Some((ptr.add(len), out));
    }

    // Spans past `buffer_end` into `next_chunk` (or flat tail if `next_chunk` is null).
    if !st.next_chunk.is_null() && st.size > 0 {
        let chunk = st.next_chunk;
        let chunk_size = st.size as usize;
        let (first_copy, chunk_offset) = if avail_in_cur > 0 {
            let n = avail_in_cur as usize;
            std::ptr::copy_nonoverlapping(ptr, out.as_mut_ptr(), n);
            (n, 0usize)
        } else {
            (0usize, (-avail_in_cur) as usize)
        };
        let second_copy = len - first_copy;
        if chunk_offset.saturating_add(second_copy) > chunk_size {
            return None;
        }
        std::ptr::copy_nonoverlapping(
            chunk.add(chunk_offset),
            out.as_mut_ptr().add(first_copy),
            second_copy,
        );
        // Transition ctx into `next_chunk`.
        st.buffer_end = chunk.add(chunk_size);
        st.limit = st.limit.wrapping_sub(chunk_size as i32);
        st.limit_end = st.buffer_end.offset(st.limit.min(0) as isize);
        st.next_chunk = std::ptr::null();
        st.size = 0;
        std::ptr::write_unaligned(ctx, st);
        Some((chunk.add(chunk_offset + second_copy), out))
    } else {
        // Contiguous flat tail without a separate patch chunk.
        std::ptr::copy_nonoverlapping(ptr, out.as_mut_ptr(), len);
        Some((ptr.add(len), out))
    }
}

unsafe fn ctx_read_byte_at(
    ctx: *const EpsCopyInputStream,
    ptr: *const u8,
    offset: usize,
) -> Option<u8> {
    if ctx.is_null() {
        return Some(std::ptr::read(ptr.add(offset)));
    }
    let st = std::ptr::read_unaligned(ctx);
    let rem = ((st.buffer_end as isize) - (ptr as isize)).saturating_add(st.limit as isize);
    if rem <= 0 || (offset as isize) >= rem {
        return None;
    }
    let avail_in_cur = (st.buffer_end as isize) - (ptr as isize);
    if (offset as isize) < avail_in_cur {
        Some(std::ptr::read(ptr.add(offset)))
    } else if !st.next_chunk.is_null() && st.size > 0 {
        let chunk_off = ((offset as isize) - avail_in_cur) as usize;
        if chunk_off >= st.size as usize {
            return None;
        }
        Some(std::ptr::read(st.next_chunk.add(chunk_off)))
    } else {
        Some(std::ptr::read(ptr.add(offset)))
    }
}

unsafe fn ctx_advance(
    ctx: *mut EpsCopyInputStream,
    ptr: *const u8,
    count: usize,
) -> Option<*const u8> {
    if ctx.is_null() {
        return Some(ptr.add(count));
    }
    let mut st = std::ptr::read_unaligned(ctx);
    let avail_in_cur = (st.buffer_end as isize) - (ptr as isize);
    if (count as isize) <= avail_in_cur || st.next_chunk.is_null() || st.size <= 0 {
        return Some(ptr.add(count));
    }
    let chunk = st.next_chunk;
    let chunk_size = st.size as usize;
    let chunk_off = ((count as isize) - avail_in_cur) as usize;
    if chunk_off > chunk_size {
        return None;
    }
    st.buffer_end = chunk.add(chunk_size);
    st.limit = st.limit.wrapping_sub(chunk_size as i32);
    st.limit_end = st.buffer_end.offset(st.limit.min(0) as isize);
    st.next_chunk = std::ptr::null();
    st.size = 0;
    std::ptr::write_unaligned(ctx, st);
    Some(chunk.add(chunk_off))
}

unsafe fn ctx_read_varint64(
    ctx: *mut EpsCopyInputStream,
    ptr: *const u8,
) -> Option<(*const u8, u64)> {
    if ptr.is_null() {
        return None;
    }
    if ctx.is_null() {
        return decode_varint64_raw(ptr);
    }
    if ctx_remaining_limit(ctx, ptr) <= 0 {
        return None;
    }
    let mut res = 0u64;
    for i in 0..10usize {
        let b = ctx_read_byte_at(ctx, ptr, i)?;
        res |= ((b & 0x7f) as u64).wrapping_shl((7 * i) as u32);
        if b < 0x80 {
            let next = ctx_advance(ctx, ptr, i + 1)?;
            return Some((next, res));
        }
    }
    None
}

unsafe fn ctx_read_tag(ctx: *mut EpsCopyInputStream, ptr: *const u8) -> Option<(*const u8, u32)> {
    if ptr.is_null() {
        return None;
    }
    if ctx.is_null() {
        return decode_tag_raw(ptr);
    }
    if ctx_remaining_limit(ctx, ptr) <= 0 {
        return None;
    }
    let mut res = 0u32;
    for i in 0..5usize {
        let b = ctx_read_byte_at(ctx, ptr, i)?;
        res |= ((b & 0x7f) as u32).wrapping_shl((7 * i) as u32);
        if b < 0x80 {
            let next = ctx_advance(ctx, ptr, i + 1)?;
            return Some((next, res));
        }
    }
    None
}

unsafe fn ctx_read_size(ctx: *mut EpsCopyInputStream, ptr: *const u8) -> Option<(*const u8, i32)> {
    if ptr.is_null() {
        return None;
    }
    if ctx.is_null() {
        return decode_size_raw(ptr);
    }
    if ctx_remaining_limit(ctx, ptr) <= 0 {
        return None;
    }
    let mut res = 0u32;
    for i in 0..4usize {
        let b = ctx_read_byte_at(ctx, ptr, i)?;
        res |= ((b & 0x7f) as u32) << (7 * i);
        if b < 0x80 {
            let next = ctx_advance(ctx, ptr, i + 1)?;
            return Some((next, res as i32));
        }
    }
    let b4 = ctx_read_byte_at(ctx, ptr, 4)?;
    if b4 >= 8 {
        return None;
    }
    res |= (b4 as u32) << 28;
    if res > (i32::MAX as u32 - EPS_SLOP_BYTES as u32) {
        return None;
    }
    let next = ctx_advance(ctx, ptr, 5)?;
    Some((next, res as i32))
}

unsafe extern "win64" fn eps_push_limit(this: *mut u8, ptr: *const u8, limit: i32) -> i32 {
    if this.is_null() || ptr.is_null() || limit < 0 {
        return -1;
    }
    let eps = this as *mut EpsCopyInputStream;
    let mut st = std::ptr::read_unaligned(eps);
    if st.buffer_end.is_null() {
        return -1;
    }
    let old_limit = st.limit;
    let rel_ptr = (ptr as isize) - (st.buffer_end as isize);
    let new_limit_isize = rel_ptr.saturating_add(limit as isize);
    if new_limit_isize > old_limit as isize
        || new_limit_isize < i32::MIN as isize
        || new_limit_isize > i32::MAX as isize
    {
        return -1;
    }
    let new_limit = new_limit_isize as i32;
    let delta = old_limit.wrapping_sub(new_limit);
    st.limit = new_limit;
    st.limit_end = st.buffer_end.offset(new_limit.min(0) as isize);
    std::ptr::write_unaligned(eps, st);
    delta
}

unsafe extern "win64" fn eps_pop_limit(this: *mut u8, delta: i32) -> u8 {
    if this.is_null() || delta < 0 {
        return 0;
    }
    let eps = this as *mut EpsCopyInputStream;
    let mut st = std::ptr::read_unaligned(eps);
    if st.buffer_end.is_null() {
        return 0;
    }
    st.limit = st.limit.wrapping_add(delta);
    st.limit_end = st.buffer_end.offset(st.limit.min(0) as isize);
    std::ptr::write_unaligned(eps, st);
    1
}

unsafe extern "win64" fn parse_context_done(this: *mut u8, ptr_slot: *mut *const u8) -> u8 {
    if this.is_null() || ptr_slot.is_null() {
        return 1;
    }
    let ptr = std::ptr::read_unaligned(ptr_slot);
    if ptr.is_null() {
        return 1;
    }
    let eps = this as *mut EpsCopyInputStream;
    let mut st = std::ptr::read_unaligned(eps);
    if st.buffer_end.is_null() && st.limit_end.is_null() {
        return 1;
    }
    if (ptr as usize) < (st.limit_end as usize) {
        return 0;
    }
    let overrun = (ptr as isize) - (st.buffer_end as isize);
    if overrun == st.limit as isize {
        return 1;
    }
    if overrun > st.limit as isize {
        std::ptr::write_unaligned(ptr_slot, std::ptr::null());
        return 1;
    }
    // `overrun < st.limit`: reached end of current chunk before limit; switch to `next_chunk`.
    if !st.next_chunk.is_null() && st.size > 0 {
        let chunk = st.next_chunk;
        let chunk_size = st.size as usize;
        if overrun < 0 || (overrun as usize) > chunk_size {
            std::ptr::write_unaligned(ptr_slot, std::ptr::null());
            return 1;
        }
        let new_ptr = chunk.add(overrun as usize);
        st.buffer_end = chunk.add(chunk_size);
        st.limit = st.limit.wrapping_sub(chunk_size as i32);
        st.limit_end = st.buffer_end.offset(st.limit.min(0) as isize);
        st.next_chunk = std::ptr::null();
        st.size = 0;
        std::ptr::write_unaligned(eps, st);
        std::ptr::write_unaligned(ptr_slot, new_ptr);
        let new_overrun = (new_ptr as isize) - (st.buffer_end as isize);
        if (new_ptr as usize) >= (st.limit_end as usize) && new_overrun == st.limit as isize {
            1
        } else {
            0
        }
    } else {
        0
    }
}

unsafe extern "win64" fn varint_parse_slow32(
    sret: *mut [usize; 2],
    ptr: *const u8,
    _res: u32,
) -> *mut [usize; 2] {
    let pair = match decode_varint32_raw(ptr) {
        Some((next, v)) => [next as usize, v as usize],
        None => [0, 0],
    };
    if !sret.is_null() {
        std::ptr::write_unaligned(sret, pair);
    }
    sret
}

unsafe extern "win64" fn varint_parse_slow64(
    sret: *mut [usize; 2],
    ptr: *const u8,
    _res: u32,
) -> *mut [usize; 2] {
    let pair = match decode_varint64_raw(ptr) {
        Some((next, v)) => [next as usize, v as usize],
        None => [0, 0],
    };
    if !sret.is_null() {
        std::ptr::write_unaligned(sret, pair);
    }
    sret
}

unsafe extern "win64" fn read_tag_fallback(
    sret: *mut [usize; 2],
    ptr: *const u8,
    _res: u32,
) -> *mut [usize; 2] {
    let pair = match decode_tag_raw(ptr) {
        Some((next, tag)) => [next as usize, tag as usize],
        None => [0, 0],
    };
    if !sret.is_null() {
        std::ptr::write_unaligned(sret, pair);
    }
    sret
}

unsafe extern "win64" fn read_size_fallback(
    sret: *mut [usize; 2],
    ptr: *const u8,
    _res: u32,
) -> *mut [usize; 2] {
    let pair = match decode_size_raw(ptr) {
        Some((next, size)) => [next as usize, (size as u32) as usize],
        None => [0, 0],
    };
    if !sret.is_null() {
        std::ptr::write_unaligned(sret, pair);
    }
    sret
}

unsafe extern "win64" fn inline_greedy_string_parser(
    s: *mut MsvcString,
    ptr: *const u8,
    ctx: *mut u8,
) -> *const u8 {
    if s.is_null() || ptr.is_null() {
        return std::ptr::null();
    }
    let eps = ctx as *mut EpsCopyInputStream;
    let Some((after_len, size)) = ctx_read_size(eps, ptr) else {
        return std::ptr::null();
    };
    if size < 0 {
        return std::ptr::null();
    }
    let Some((next, bytes)) = ctx_read_bytes(eps, after_len, size as usize) else {
        return std::ptr::null();
    };
    msvc_string_assign_bytes(s, &bytes);
    next
}

unsafe extern "win64" fn inline_greedy_string_parser_utf8(
    s: *mut MsvcString,
    ptr: *const u8,
    ctx: *mut u8,
    _field_name: *const u8,
) -> *const u8 {
    let next = inline_greedy_string_parser(s, ptr, ctx);
    if next.is_null() {
        return std::ptr::null();
    }
    let (data, len) = msvc_string_data_and_len(s);
    if !is_valid_utf8_raw(data, len) {
        return std::ptr::null();
    }
    next
}

unsafe fn read_packed_payload(
    ptr: *const u8,
    ctx: *mut u8,
) -> Option<(*const u8, Vec<u8>)> {
    if ptr.is_null() {
        return None;
    }
    let eps = ctx as *mut EpsCopyInputStream;
    let (after_len, size) = ctx_read_size(eps, ptr)?;
    if size < 0 {
        return None;
    }
    ctx_read_bytes(eps, after_len, size as usize)
}

unsafe fn packed_varint_parse<T: Copy, F: Fn(u64) -> T>(
    object: *mut u8,
    ptr: *const u8,
    ctx: *mut u8,
    convert: F,
) -> *const u8 {
    if object.is_null() {
        return std::ptr::null();
    }
    let Some((next, payload)) = read_packed_payload(ptr, ctx) else {
        return std::ptr::null();
    };
    let rf = object as *mut RepeatedFieldRaw;
    let mut pos = 0usize;
    while pos < payload.len() {
        let Some(v) = decode_varint64_slice(&payload, &mut pos) else {
            return std::ptr::null();
        };
        repeated_field_push::<T>(rf, convert(v));
    }
    next
}

unsafe fn packed_fixed32_parse(object: *mut u8, ptr: *const u8, ctx: *mut u8) -> *const u8 {
    if object.is_null() {
        return std::ptr::null();
    }
    let Some((next, payload)) = read_packed_payload(ptr, ctx) else {
        return std::ptr::null();
    };
    if payload.len() % 4 != 0 {
        return std::ptr::null();
    }
    let rf = object as *mut RepeatedFieldRaw;
    for chunk in payload.chunks_exact(4) {
        let v = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        repeated_field_push::<u32>(rf, v);
    }
    next
}

unsafe fn packed_fixed64_parse(object: *mut u8, ptr: *const u8, ctx: *mut u8) -> *const u8 {
    if object.is_null() {
        return std::ptr::null();
    }
    let Some((next, payload)) = read_packed_payload(ptr, ctx) else {
        return std::ptr::null();
    };
    if payload.len() % 8 != 0 {
        return std::ptr::null();
    }
    let rf = object as *mut RepeatedFieldRaw;
    for chunk in payload.chunks_exact(8) {
        let v = u64::from_le_bytes([
            chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5], chunk[6], chunk[7],
        ]);
        repeated_field_push::<u64>(rf, v);
    }
    next
}

unsafe extern "win64" fn packed_int32_parser(
    object: *mut u8,
    ptr: *const u8,
    ctx: *mut u8,
) -> *const u8 {
    packed_varint_parse(object, ptr, ctx, |v| v as i32)
}

unsafe extern "win64" fn packed_uint32_parser(
    object: *mut u8,
    ptr: *const u8,
    ctx: *mut u8,
) -> *const u8 {
    packed_varint_parse(object, ptr, ctx, |v| v as u32)
}

unsafe extern "win64" fn packed_sint32_parser(
    object: *mut u8,
    ptr: *const u8,
    ctx: *mut u8,
) -> *const u8 {
    packed_varint_parse(object, ptr, ctx, |v| zigzag_decode32(v as u32))
}

unsafe extern "win64" fn packed_int64_parser(
    object: *mut u8,
    ptr: *const u8,
    ctx: *mut u8,
) -> *const u8 {
    packed_varint_parse(object, ptr, ctx, |v| v as i64)
}

unsafe extern "win64" fn packed_uint64_parser(
    object: *mut u8,
    ptr: *const u8,
    ctx: *mut u8,
) -> *const u8 {
    packed_varint_parse(object, ptr, ctx, |v| v)
}

unsafe extern "win64" fn packed_sint64_parser(
    object: *mut u8,
    ptr: *const u8,
    ctx: *mut u8,
) -> *const u8 {
    packed_varint_parse(object, ptr, ctx, zigzag_decode64)
}

unsafe extern "win64" fn packed_bool_parser(
    object: *mut u8,
    ptr: *const u8,
    ctx: *mut u8,
) -> *const u8 {
    packed_varint_parse(object, ptr, ctx, |v| if v != 0 { 1u8 } else { 0u8 })
}

unsafe extern "win64" fn packed_fixed32_parser(
    object: *mut u8,
    ptr: *const u8,
    ctx: *mut u8,
) -> *const u8 {
    packed_fixed32_parse(object, ptr, ctx)
}

unsafe extern "win64" fn packed_fixed64_parser(
    object: *mut u8,
    ptr: *const u8,
    ctx: *mut u8,
) -> *const u8 {
    packed_fixed64_parse(object, ptr, ctx)
}

// --- 10. Reflection, FieldDescriptor, Base64Escape ---

unsafe extern "win64" fn refl_get_u64(
    _this: *const u8,
    _msg: *const u8,
    _field: *const u8,
) -> u64 {
    0
}

unsafe extern "win64" fn refl_get_i64(
    _this: *const u8,
    _msg: *const u8,
    _field: *const u8,
) -> i64 {
    0
}

unsafe extern "win64" fn refl_get_u32(
    _this: *const u8,
    _msg: *const u8,
    _field: *const u8,
) -> u32 {
    0
}

unsafe extern "win64" fn refl_get_i32(
    _this: *const u8,
    _msg: *const u8,
    _field: *const u8,
) -> i32 {
    0
}

unsafe extern "win64" fn refl_get_f32(
    _this: *const u8,
    _msg: *const u8,
    _field: *const u8,
) -> f32 {
    0.0
}

unsafe extern "win64" fn refl_get_f64(
    _this: *const u8,
    _msg: *const u8,
    _field: *const u8,
) -> f64 {
    0.0
}

unsafe extern "win64" fn refl_get_bool(
    _this: *const u8,
    _msg: *const u8,
    _field: *const u8,
) -> u8 {
    0
}

unsafe extern "win64" fn refl_get_message(
    _this: *const u8,
    msg: *const u8,
    _field: *const u8,
    _factory: *mut u8,
) -> *const u8 {
    ensure_vtables_registered();
    if msg.is_null() {
        &DUMMY_MESSAGE as *const DummyMessage as *const u8
    } else {
        msg
    }
}

unsafe extern "win64" fn refl_get_string_ref(
    _this: *const u8,
    _msg: *const u8,
    _field: *const u8,
    _scratch: *mut MsvcString,
) -> *const MsvcString {
    empty_string_ptr()
}

unsafe extern "win64" fn refl_get_repeated_i32(
    _this: *const u8,
    _msg: *const u8,
    _field: *const u8,
    _index: i32,
) -> i32 {
    0
}

unsafe extern "win64" fn refl_get_repeated_i64(
    _this: *const u8,
    _msg: *const u8,
    _field: *const u8,
    _index: i32,
) -> i64 {
    0
}

unsafe extern "win64" fn refl_get_repeated_u32(
    _this: *const u8,
    _msg: *const u8,
    _field: *const u8,
    _index: i32,
) -> u32 {
    0
}

unsafe extern "win64" fn refl_get_repeated_u64(
    _this: *const u8,
    _msg: *const u8,
    _field: *const u8,
    _index: i32,
) -> u64 {
    0
}

unsafe extern "win64" fn refl_get_repeated_f32(
    _this: *const u8,
    _msg: *const u8,
    _field: *const u8,
    _index: i32,
) -> f32 {
    0.0
}

unsafe extern "win64" fn refl_get_repeated_f64(
    _this: *const u8,
    _msg: *const u8,
    _field: *const u8,
    _index: i32,
) -> f64 {
    0.0
}

unsafe extern "win64" fn refl_get_repeated_bool(
    _this: *const u8,
    _msg: *const u8,
    _field: *const u8,
    _index: i32,
) -> u8 {
    0
}

unsafe extern "win64" fn refl_get_repeated_message(
    _this: *const u8,
    msg: *const u8,
    _field: *const u8,
    _index: i32,
) -> *const u8 {
    ensure_vtables_registered();
    if msg.is_null() {
        &DUMMY_MESSAGE as *const DummyMessage as *const u8
    } else {
        msg
    }
}

unsafe extern "win64" fn refl_get_repeated_string_ref(
    _this: *const u8,
    _msg: *const u8,
    _field: *const u8,
    _index: i32,
    _scratch: *mut MsvcString,
) -> *const MsvcString {
    empty_string_ptr()
}

unsafe extern "win64" fn refl_list_fields(
    _this: *const u8,
    _msg: *const u8,
    output: *mut [usize; 3],
) {
    if !output.is_null() {
        let first = std::ptr::read_unaligned(output as *const usize);
        std::ptr::write_unaligned((output as *mut usize).add(1), first);
    }
}

const BASE64_STD_TABLE: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const BASE64_WEB_TABLE: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

fn base64_encode_to_vec(input: &[u8], web_safe: bool, pad: bool) -> Vec<u8> {
    if input.is_empty() {
        return Vec::new();
    }
    let tbl = if web_safe {
        BASE64_WEB_TABLE
    } else {
        BASE64_STD_TABLE
    };
    let mut out = Vec::with_capacity(((input.len() + 2) / 3) * 4);
    let mut chunks = input.chunks_exact(3);
    for c in &mut chunks {
        let b0 = c[0] as usize;
        let b1 = c[1] as usize;
        let b2 = c[2] as usize;
        out.push(tbl[b0 >> 2]);
        out.push(tbl[((b0 & 0x03) << 4) | (b1 >> 4)]);
        out.push(tbl[((b1 & 0x0f) << 2) | (b2 >> 6)]);
        out.push(tbl[b2 & 0x3f]);
    }
    let rem = chunks.remainder();
    match rem.len() {
        1 => {
            let b0 = rem[0] as usize;
            out.push(tbl[b0 >> 2]);
            out.push(tbl[(b0 & 0x03) << 4]);
            if pad {
                out.push(b'=');
                out.push(b'=');
            }
        }
        2 => {
            let b0 = rem[0] as usize;
            let b1 = rem[1] as usize;
            out.push(tbl[b0 >> 2]);
            out.push(tbl[((b0 & 0x03) << 4) | (b1 >> 4)]);
            out.push(tbl[(b1 & 0x0f) << 2]);
            if pad {
                out.push(b'=');
            }
        }
        _ => {}
    }
    out
}

unsafe fn base64_escape_string_piece(
    src: *const [usize; 2],
    dest: *mut MsvcString,
    web_safe: bool,
    pad: bool,
) {
    if dest.is_null() {
        return;
    }
    if src.is_null() {
        msvc_string_clear(dest);
        return;
    }
    let [ptr, len] = std::ptr::read_unaligned(src);
    if len == 0 || ptr == 0 {
        msvc_string_clear(dest);
        return;
    }
    let input = std::slice::from_raw_parts(ptr as *const u8, len);
    let encoded = base64_encode_to_vec(input, web_safe, pad);
    msvc_string_assign_bytes(dest, &encoded);
}

unsafe extern "win64" fn base64_escape(src: *const [usize; 2], dest: *mut MsvcString) {
    base64_escape_string_piece(src, dest, false, true);
}

unsafe extern "win64" fn web_safe_base64_escape(src: *const [usize; 2], dest: *mut MsvcString) {
    base64_escape_string_piece(src, dest, true, false);
}

unsafe extern "win64" fn base64_escape_buf(
    src: *const u8,
    slen: i32,
    dest: *mut u8,
    szdest: i32,
) -> i32 {
    if slen < 0 || szdest < 0 || dest.is_null() || (slen > 0 && src.is_null()) {
        return 0;
    }
    let input = if slen == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(src, slen as usize)
    };
    let encoded = base64_encode_to_vec(input, false, true);
    if encoded.len() > szdest as usize {
        return 0;
    }
    if !encoded.is_empty() {
        std::ptr::copy_nonoverlapping(encoded.as_ptr(), dest, encoded.len());
    }
    encoded.len() as i32
}

unsafe extern "win64" fn web_safe_base64_escape_buf(
    src: *const u8,
    slen: i32,
    dest: *mut u8,
    szdest: i32,
    do_padding: u8,
) -> i32 {
    if slen < 0 || szdest < 0 || dest.is_null() || (slen > 0 && src.is_null()) {
        return 0;
    }
    let input = if slen == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(src, slen as usize)
    };
    let encoded = base64_encode_to_vec(input, true, do_padding != 0);
    if encoded.len() > szdest as usize {
        return 0;
    }
    if !encoded.is_empty() {
        std::ptr::copy_nonoverlapping(encoded.as_ptr(), dest, encoded.len());
    }
    encoded.len() as i32
}

unsafe extern "win64" fn field_descriptor_cpp_type(_this: *const u8) -> i32 {
    1 // CPPTYPE_INT32
}

unsafe extern "win64" fn field_descriptor_type(_this: *const u8) -> i32 {
    5 // TYPE_INT32
}

// --- 11. Export table ---

pub static EXTRA_EXPORTS: &[crate::Export] = &[
    // Original 122 MSVC C++ exports (exact mangled names preserved for 100% ABI compatibility)
    crate::Export {
        name: "?GetUInt64@Reflection@protobuf@google@@QEBA_KAEBVMessage@23@PEBVFieldDescriptor@23@@Z",
        ptr: crate::Ptr(refl_get_u64 as *const ()),
    },
    crate::Export {
        name: "?GetFloat@Reflection@protobuf@google@@QEBAMAEBVMessage@23@PEBVFieldDescriptor@23@@Z",
        ptr: crate::Ptr(refl_get_f32 as *const ()),
    },
    crate::Export {
        name: "?GetDouble@Reflection@protobuf@google@@QEBANAEBVMessage@23@PEBVFieldDescriptor@23@@Z",
        ptr: crate::Ptr(refl_get_f64 as *const ()),
    },
    crate::Export {
        name: "?GetBool@Reflection@protobuf@google@@QEBA_NAEBVMessage@23@PEBVFieldDescriptor@23@@Z",
        ptr: crate::Ptr(refl_get_bool as *const ()),
    },
    crate::Export {
        name: "?GetEnumValue@Reflection@protobuf@google@@QEBAHAEBVMessage@23@PEBVFieldDescriptor@23@@Z",
        ptr: crate::Ptr(refl_get_i32 as *const ()),
    },
    crate::Export {
        name: "?GetMessage@Reflection@protobuf@google@@QEBAAEBVMessage@23@AEBV423@PEBVFieldDescriptor@23@PEAVMessageFactory@23@@Z",
        ptr: crate::Ptr(refl_get_message as *const ()),
    },
    crate::Export {
        name: "?GetStringReference@Reflection@protobuf@google@@QEBAAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEBVMessage@23@PEBVFieldDescriptor@23@PEAV45@@Z",
        ptr: crate::Ptr(refl_get_string_ref as *const ()),
    },
    crate::Export {
        name: "?GetRepeatedInt32@Reflection@protobuf@google@@QEBAHAEBVMessage@23@PEBVFieldDescriptor@23@H@Z",
        ptr: crate::Ptr(refl_get_repeated_i32 as *const ()),
    },
    crate::Export {
        name: "?GetRepeatedInt64@Reflection@protobuf@google@@QEBA_JAEBVMessage@23@PEBVFieldDescriptor@23@H@Z",
        ptr: crate::Ptr(refl_get_repeated_i64 as *const ()),
    },
    crate::Export {
        name: "?GetRepeatedUInt32@Reflection@protobuf@google@@QEBAIAEBVMessage@23@PEBVFieldDescriptor@23@H@Z",
        ptr: crate::Ptr(refl_get_repeated_u32 as *const ()),
    },
    crate::Export {
        name: "?GetRepeatedUInt64@Reflection@protobuf@google@@QEBA_KAEBVMessage@23@PEBVFieldDescriptor@23@H@Z",
        ptr: crate::Ptr(refl_get_repeated_u64 as *const ()),
    },
    crate::Export {
        name: "?GetRepeatedFloat@Reflection@protobuf@google@@QEBAMAEBVMessage@23@PEBVFieldDescriptor@23@H@Z",
        ptr: crate::Ptr(refl_get_repeated_f32 as *const ()),
    },
    crate::Export {
        name: "?GetRepeatedDouble@Reflection@protobuf@google@@QEBANAEBVMessage@23@PEBVFieldDescriptor@23@H@Z",
        ptr: crate::Ptr(refl_get_repeated_f64 as *const ()),
    },
    crate::Export {
        name: "?GetRepeatedBool@Reflection@protobuf@google@@QEBA_NAEBVMessage@23@PEBVFieldDescriptor@23@H@Z",
        ptr: crate::Ptr(refl_get_repeated_bool as *const ()),
    },
    crate::Export {
        name: "?GetRepeatedEnumValue@Reflection@protobuf@google@@QEBAHAEBVMessage@23@PEBVFieldDescriptor@23@H@Z",
        ptr: crate::Ptr(refl_get_repeated_i32 as *const ()),
    },
    crate::Export {
        name: "?GetRepeatedMessage@Reflection@protobuf@google@@QEBAAEBVMessage@23@AEBV423@PEBVFieldDescriptor@23@H@Z",
        ptr: crate::Ptr(refl_get_repeated_message as *const ()),
    },
    crate::Export {
        name: "?GetRepeatedStringReference@Reflection@protobuf@google@@QEBAAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AEBVMessage@23@PEBVFieldDescriptor@23@HPEAV45@@Z",
        ptr: crate::Ptr(refl_get_repeated_string_ref as *const ()),
    },
    crate::Export {
        name: "?Set@ArenaStringPtr@internal@protobuf@google@@QEAAXUEmptyDefault@1234@AEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEAVArena@34@@Z",
        ptr: crate::Ptr(arena_string_ptr_set_copy as *const ()),
    },
    crate::Export {
        name: "?Set@ArenaStringPtr@internal@protobuf@google@@QEAAXUEmptyDefault@1234@$$QEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEAVArena@34@@Z",
        ptr: crate::Ptr(arena_string_ptr_set_move as *const ()),
    },
    crate::Export {
        name: "?GetArena@MessageLite@protobuf@google@@QEBAPEAVArena@23@XZ",
        ptr: crate::Ptr(message_lite_get_arena as *const ()),
    },
    crate::Export {
        name: "?Reserve@RepeatedPtrFieldBase@internal@protobuf@google@@IEAAXH@Z",
        ptr: crate::Ptr(repeated_ptr_field_base_reserve as *const ()),
    },
    crate::Export {
        name: "?Add@?$RepeatedField@_K@protobuf@google@@QEAAXAEB_K@Z",
        ptr: crate::Ptr(repeated_field_add as *const ()),
    },
    crate::Export {
        name: "?Add@?$RepeatedField@N@protobuf@google@@QEAAXAEBN@Z",
        ptr: crate::Ptr(repeated_field_add as *const ()),
    },
    crate::Export {
        name: "?UnsafeSetDefault@ArenaStringPtr@internal@protobuf@google@@QEAAXPEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@Z",
        ptr: crate::Ptr(arena_string_ptr_unsafe_set_default as *const ()),
    },
    crate::Export {
        name: "?GetEmptyStringAlreadyInited@internal@protobuf@google@@YAAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@XZ",
        ptr: crate::Ptr(get_empty_string_already_inited as *const ()),
    },
    crate::Export {
        name: "?ClearToEmpty@ArenaStringPtr@internal@protobuf@google@@QEAAXXZ",
        ptr: crate::Ptr(arena_string_ptr_clear_to_empty as *const ()),
    },
    crate::Export {
        name: "?fixed_address_empty_string@internal@protobuf@google@@3V?$ExplicitlyConstructed@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@123@A",
        ptr: crate::Ptr(&FIXED_ADDRESS_EMPTY_STRING as *const SyncMsvcString as *const ()),
    },
    crate::Export {
        name: "?InitializationErrorString@Message@protobuf@google@@UEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@XZ",
        ptr: crate::Ptr(msg_init_error_string as *const ()),
    },
    crate::Export {
        name: "?DiscardUnknownFields@Message@protobuf@google@@UEAAXXZ",
        ptr: crate::Ptr(msg_discard_unknown_fields as *const ()),
    },
    crate::Export {
        name: "?SpaceUsedLong@Message@protobuf@google@@UEBA_KXZ",
        ptr: crate::Ptr(msg_space_used_long as *const ()),
    },
    crate::Export {
        name: "?GetTypeName@Message@protobuf@google@@UEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@XZ",
        ptr: crate::Ptr(msg_get_type_name as *const ()),
    },
    crate::Export {
        name: "?CheckTypeAndMergeFrom@Message@protobuf@google@@UEAAXAEBVMessageLite@23@@Z",
        ptr: crate::Ptr(msg_check_type_and_merge as *const ()),
    },
    crate::Export {
        name: "?InternalGetTable@MessageLite@protobuf@google@@EEBAPEBXXZ",
        ptr: crate::Ptr(msg_internal_get_table as *const ()),
    },
    crate::Export {
        name: "?WriteStringMaybeAliased@EpsCopyOutputStream@io@protobuf@google@@QEAAPEAEIAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEAE@Z",
        ptr: crate::Ptr(eps_write_string_maybe_aliased as *const ()),
    },
    crate::Export {
        name: "?WriteBytesMaybeAliased@EpsCopyOutputStream@io@protobuf@google@@QEAAPEAEIAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEAE@Z",
        ptr: crate::Ptr(eps_write_string_maybe_aliased as *const ()),
    },
    crate::Export {
        name: "?EnsureSpaceFallback@EpsCopyOutputStream@io@protobuf@google@@AEAAPEAEPEAE@Z",
        ptr: crate::Ptr(eps_ensure_space_fallback as *const ()),
    },
    crate::Export {
        name: "?WriteVarint32ToArray@CodedOutputStream@io@protobuf@google@@SAPEAEIPEAE@Z",
        ptr: crate::Ptr(coded_write_varint32_to_array as *const ()),
    },
    crate::Export {
        name: "?AllocateInternal@Arena@protobuf@google@@AEAAPEAX_K0P6AXPEAX@ZPEBVtype_info@@@Z",
        ptr: crate::Ptr(arena_allocate_internal as *const ()),
    },
    crate::Export {
        name: "??0ArenaStringPtr@internal@protobuf@google@@QEAA@PEBV?$ExplicitlyConstructed@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@123@@Z",
        ptr: crate::Ptr(arena_string_ptr_ctor as *const ()),
    },
    crate::Export {
        name: "?Get@ArenaStringPtr@internal@protobuf@google@@QEBAAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@XZ",
        ptr: crate::Ptr(arena_string_ptr_get as *const ()),
    },
    crate::Export {
        name: "?Mutable@ArenaStringPtr@internal@protobuf@google@@QEAAPEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@UEmptyDefault@1234@PEAVArena@34@@Z",
        ptr: crate::Ptr(arena_string_ptr_mutable as *const ()),
    },
    crate::Export {
        name: "?Swap@ArenaStringPtr@internal@protobuf@google@@QEAAXPEAU1234@PEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEAVArena@34@@Z",
        ptr: crate::Ptr(arena_string_ptr_swap as *const ()),
    },
    crate::Export {
        name: "?DestroyNoArena@ArenaStringPtr@internal@protobuf@google@@QEAAXPEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@Z",
        ptr: crate::Ptr(arena_string_ptr_destroy_no_arena as *const ()),
    },
    crate::Export {
        name: "??1RepeatedPtrFieldBase@internal@protobuf@google@@IEAA@XZ",
        ptr: crate::Ptr(repeated_ptr_field_base_dtor as *const ()),
    },
    crate::Export {
        name: "?InternalSwap@RepeatedPtrFieldBase@internal@protobuf@google@@QEAAXPEAV1234@@Z",
        ptr: crate::Ptr(repeated_ptr_field_base_internal_swap as *const ()),
    },
    crate::Export {
        name: "?MergeFromInternal@RepeatedPtrFieldBase@internal@protobuf@google@@AEAAXAEBV1234@P81234@EAAXPEAPEAX1HH@Z@Z",
        ptr: crate::Ptr(repeated_ptr_field_base_merge_from_internal as *const ()),
    },
    crate::Export {
        name: "?VerifyUtf8String@WireFormatLite@internal@protobuf@google@@SA_NPEBDHW4Operation@1234@0@Z",
        ptr: crate::Ptr(verify_utf8_string as *const ()),
    },
    crate::Export {
        name: "?WriteTagToArray@WireFormatLite@internal@protobuf@google@@SAPEAEHW4WireType@1234@PEAE@Z",
        ptr: crate::Ptr(wire_write_tag_to_array as *const ()),
    },
    crate::Export {
        name: "?WriteUInt32ToArray@WireFormatLite@internal@protobuf@google@@SAPEAEHIPEAE@Z",
        ptr: crate::Ptr(wire_write_uint32_to_array as *const ()),
    },
    crate::Export {
        name: "?WriteFixed64ToArray@WireFormatLite@internal@protobuf@google@@SAPEAEH_KPEAE@Z",
        ptr: crate::Ptr(wire_write_fixed64_to_array as *const ()),
    },
    crate::Export {
        name: "?WriteEnumToArray@WireFormatLite@internal@protobuf@google@@SAPEAEHHPEAE@Z",
        ptr: crate::Ptr(wire_write_int32_to_array as *const ()),
    },
    crate::Export {
        name: "?UInt32Size@WireFormatLite@internal@protobuf@google@@SA_KI@Z",
        ptr: crate::Ptr(wire_uint32_size as *const ()),
    },
    crate::Export {
        name: "?GetUInt32@Reflection@protobuf@google@@QEBAIAEBVMessage@23@PEBVFieldDescriptor@23@@Z",
        ptr: crate::Ptr(refl_get_u32 as *const ()),
    },
    crate::Export {
        name: "?StringSize@WireFormatLite@internal@protobuf@google@@SA_KAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@Z",
        ptr: crate::Ptr(wire_string_size as *const ()),
    },
    crate::Export {
        name: "?BytesSize@WireFormatLite@internal@protobuf@google@@SA_KAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@Z",
        ptr: crate::Ptr(wire_string_size as *const ()),
    },
    crate::Export {
        name: "?LengthDelimitedSize@WireFormatLite@internal@protobuf@google@@SA_K_K@Z",
        ptr: crate::Ptr(wire_length_delimited_size as *const ()),
    },
    crate::Export {
        name: "?PushLimit@EpsCopyInputStream@internal@protobuf@google@@QEAAHPEBDH@Z",
        ptr: crate::Ptr(eps_push_limit as *const ()),
    },
    crate::Export {
        name: "?PopLimit@EpsCopyInputStream@internal@protobuf@google@@QEAA_NH@Z",
        ptr: crate::Ptr(eps_pop_limit as *const ()),
    },
    crate::Export {
        name: "?Done@ParseContext@internal@protobuf@google@@QEAA_NPEAPEBD@Z",
        ptr: crate::Ptr(parse_context_done as *const ()),
    },
    crate::Export {
        name: "?VarintParseSlow32@internal@protobuf@google@@YA?AU?$pair@PEBDI@std@@PEBDI@Z",
        ptr: crate::Ptr(varint_parse_slow32 as *const ()),
    },
    crate::Export {
        name: "?VarintParseSlow64@internal@protobuf@google@@YA?AU?$pair@PEBD_K@std@@PEBDI@Z",
        ptr: crate::Ptr(varint_parse_slow64 as *const ()),
    },
    crate::Export {
        name: "?ReadTagFallback@internal@protobuf@google@@YA?AU?$pair@PEBDI@std@@PEBDI@Z",
        ptr: crate::Ptr(read_tag_fallback as *const ()),
    },
    crate::Export {
        name: "?ReadSizeFallback@internal@protobuf@google@@YA?AU?$pair@PEBDH@std@@PEBDI@Z",
        ptr: crate::Ptr(read_size_fallback as *const ()),
    },
    crate::Export {
        name: "?VerifyUTF8@internal@protobuf@google@@YA_NVStringPiece@23@PEBD@Z",
        ptr: crate::Ptr(verify_utf8 as *const ()),
    },
    crate::Export {
        name: "?InlineGreedyStringParser@internal@protobuf@google@@YAPEBDPEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEBDPEAVParseContext@123@@Z",
        ptr: crate::Ptr(inline_greedy_string_parser as *const ()),
    },
    crate::Export {
        name: "?Get@CachedSize@internal@protobuf@google@@QEBAHXZ",
        ptr: crate::Ptr(cached_size_get as *const ()),
    },
    crate::Export {
        name: "?Set@CachedSize@internal@protobuf@google@@QEAAXH@Z",
        ptr: crate::Ptr(cached_size_set as *const ()),
    },
    crate::Export {
        name: "??0CachedSize@internal@protobuf@google@@QEAA@XZ",
        ptr: crate::Ptr(cached_size_ctor as *const ()),
    },
    crate::Export {
        name: "??0UnknownFieldSet@protobuf@google@@QEAA@XZ",
        ptr: crate::Ptr(unknown_field_set_ctor as *const ()),
    },
    crate::Export {
        name: "??1UnknownFieldSet@protobuf@google@@QEAA@XZ",
        ptr: crate::Ptr(unknown_field_set_dtor as *const ()),
    },
    crate::Export {
        name: "?Clear@UnknownFieldSet@protobuf@google@@QEAAXXZ",
        ptr: crate::Ptr(unknown_field_set_clear as *const ()),
    },
    crate::Export {
        name: "?MergeFrom@UnknownFieldSet@protobuf@google@@QEAAXAEBV123@@Z",
        ptr: crate::Ptr(unknown_field_set_merge_from as *const ()),
    },
    crate::Export {
        name: "?Swap@UnknownFieldSet@protobuf@google@@QEAAXPEAV123@@Z",
        ptr: crate::Ptr(unknown_field_set_swap as *const ()),
    },
    crate::Export {
        name: "?UnknownFieldParse@internal@protobuf@google@@YAPEBD_KPEAVUnknownFieldSet@23@PEBDPEAVParseContext@123@@Z",
        ptr: crate::Ptr(unknown_field_parse as *const ()),
    },
    crate::Export {
        name: "?AssignDescriptors@internal@protobuf@google@@YAXPEBUDescriptorTable@123@_N@Z",
        ptr: crate::Ptr(assign_descriptors as *const ()),
    },
    crate::Export {
        name: "??0AddDescriptorsRunner@internal@protobuf@google@@QEAA@PEBUDescriptorTable@123@@Z",
        ptr: crate::Ptr(add_descriptors_runner_ctor as *const ()),
    },
    crate::Export {
        name: "??0Message@protobuf@google@@QEAA@XZ",
        ptr: crate::Ptr(msg_ctor_default as *const ()),
    },
    crate::Export {
        name: "??0Message@protobuf@google@@IEAA@PEAVArena@12@@Z",
        ptr: crate::Ptr(msg_ctor_arena as *const ()),
    },
    crate::Export {
        name: "??1Message@protobuf@google@@UEAA@XZ",
        ptr: crate::Ptr(msg_dtor as *const ()),
    },
    crate::Export {
        name: "?Merge@ReflectionOps@internal@protobuf@google@@SAXAEBVMessage@34@PEAV534@@Z",
        ptr: crate::Ptr(reflection_ops_merge as *const ()),
    },
    crate::Export {
        name: "?InternalSerializeUnknownFieldsToArray@WireFormat@internal@protobuf@google@@SAPEAEAEBVUnknownFieldSet@34@PEAEPEAVEpsCopyOutputStream@io@34@@Z",
        ptr: crate::Ptr(internal_serialize_unknown_fields_to_array as *const ()),
    },
    crate::Export {
        name: "?ComputeUnknownFieldsSize@internal@protobuf@google@@YA_KAEBVInternalMetadata@123@_KPEAVCachedSize@123@@Z",
        ptr: crate::Ptr(compute_unknown_fields_size as *const ()),
    },
    crate::Export {
        name: "?WriteInt64ToArray@WireFormatLite@internal@protobuf@google@@SAPEAEH_JPEAE@Z",
        ptr: crate::Ptr(wire_write_int64_to_array as *const ()),
    },
    crate::Export {
        name: "?Int64Size@WireFormatLite@internal@protobuf@google@@SA_K_J@Z",
        ptr: crate::Ptr(wire_int64_size as *const ()),
    },
    crate::Export {
        name: "?Destroy@ArenaStringPtr@internal@protobuf@google@@QEAAXUEmptyDefault@1234@PEAVArena@34@@Z",
        ptr: crate::Ptr(arena_string_ptr_destroy as *const ()),
    },
    crate::Export {
        name: "?WriteDoubleToArray@WireFormatLite@internal@protobuf@google@@SAPEAEHNPEAE@Z",
        ptr: crate::Ptr(crate::bridge_entry!(wire_write_double_to_array)),
    },
    crate::Export {
        name: "?WriteBoolToArray@WireFormatLite@internal@protobuf@google@@SAPEAEH_NPEAE@Z",
        ptr: crate::Ptr(wire_write_bool_to_array as *const ()),
    },
    crate::Export {
        name: "?WriteFixed32ToArray@WireFormatLite@internal@protobuf@google@@SAPEAEHIPEAE@Z",
        ptr: crate::Ptr(wire_write_fixed32_to_array as *const ()),
    },
    crate::Export {
        name: "?WriteRaw@EpsCopyOutputStream@io@protobuf@google@@QEAAPEAEPEBXHPEAE@Z",
        ptr: crate::Ptr(eps_write_raw as *const ()),
    },
    crate::Export {
        name: "?WriteLengthDelim@EpsCopyOutputStream@io@protobuf@google@@AEAAPEAEHIPEAE@Z",
        ptr: crate::Ptr(eps_write_length_delim as *const ()),
    },
    crate::Export {
        name: "?Encode64@EpsCopyOutputStream@io@protobuf@google@@CA_K_K@Z",
        ptr: crate::Ptr(eps_encode64 as *const ()),
    },
    crate::Export {
        name: "??0?$RepeatedField@_K@protobuf@google@@QEAA@PEAVArena@12@@Z",
        ptr: crate::Ptr(repeated_field_ctor as *const ()),
    },
    crate::Export {
        name: "??1?$RepeatedField@_K@protobuf@google@@QEAA@XZ",
        ptr: crate::Ptr(repeated_field_dtor as *const ()),
    },
    crate::Export {
        name: "?size@?$RepeatedField@_K@protobuf@google@@QEBAHXZ",
        ptr: crate::Ptr(repeated_field_size as *const ()),
    },
    crate::Export {
        name: "?Clear@?$RepeatedField@_K@protobuf@google@@QEAAXXZ",
        ptr: crate::Ptr(repeated_field_clear as *const ()),
    },
    crate::Export {
        name: "?MergeFrom@?$RepeatedField@_K@protobuf@google@@QEAAXAEBV123@@Z",
        ptr: crate::Ptr(repeated_field_merge_from as *const ()),
    },
    crate::Export {
        name: "?data@?$RepeatedField@_K@protobuf@google@@QEBAPEB_KXZ",
        ptr: crate::Ptr(repeated_field_data as *const ()),
    },
    crate::Export {
        name: "??0?$RepeatedField@N@protobuf@google@@QEAA@PEAVArena@12@@Z",
        ptr: crate::Ptr(repeated_field_ctor as *const ()),
    },
    crate::Export {
        name: "??1?$RepeatedField@N@protobuf@google@@QEAA@XZ",
        ptr: crate::Ptr(repeated_field_dtor as *const ()),
    },
    crate::Export {
        name: "?size@?$RepeatedField@N@protobuf@google@@QEBAHXZ",
        ptr: crate::Ptr(repeated_field_size as *const ()),
    },
    crate::Export {
        name: "?Clear@?$RepeatedField@N@protobuf@google@@QEAAXXZ",
        ptr: crate::Ptr(repeated_field_clear as *const ()),
    },
    crate::Export {
        name: "?MergeFrom@?$RepeatedField@N@protobuf@google@@QEAAXAEBV123@@Z",
        ptr: crate::Ptr(repeated_field_merge_from as *const ()),
    },
    crate::Export {
        name: "?data@?$RepeatedField@N@protobuf@google@@QEBAPEBNXZ",
        ptr: crate::Ptr(repeated_field_data as *const ()),
    },
    crate::Export {
        name: "?WriteSInt32ToArray@WireFormatLite@internal@protobuf@google@@SAPEAEHHPEAE@Z",
        ptr: crate::Ptr(wire_write_sint32_to_array as *const ()),
    },
    crate::Export {
        name: "?WriteSFixed64ToArray@WireFormatLite@internal@protobuf@google@@SAPEAEH_JPEAE@Z",
        ptr: crate::Ptr(wire_write_sfixed64_to_array as *const ()),
    },
    crate::Export {
        name: "?Int32Size@WireFormatLite@internal@protobuf@google@@SA_KH@Z",
        ptr: crate::Ptr(wire_int32_size as *const ()),
    },
    crate::Export {
        name: "?SInt32Size@WireFormatLite@internal@protobuf@google@@SA_KH@Z",
        ptr: crate::Ptr(wire_sint32_size as *const ()),
    },
    crate::Export {
        name: "?UInt64Size@WireFormatLite@internal@protobuf@google@@SA_KAEBV?$RepeatedField@_K@34@@Z",
        ptr: crate::Ptr(wire_uint64_repeated_size as *const ()),
    },
    crate::Export {
        name: "?PackedUInt64Parser@internal@protobuf@google@@YAPEBDPEAXPEBDPEAVParseContext@123@@Z",
        ptr: crate::Ptr(packed_uint64_parser as *const ()),
    },
    crate::Export {
        name: "?PackedFixed64Parser@internal@protobuf@google@@YAPEBDPEAXPEBDPEAVParseContext@123@@Z",
        ptr: crate::Ptr(packed_fixed64_parser as *const ()),
    },
    crate::Export {
        name: "?PackedDoubleParser@internal@protobuf@google@@YAPEBDPEAXPEBDPEAVParseContext@123@@Z",
        ptr: crate::Ptr(packed_fixed64_parser as *const ()),
    },
    crate::Export {
        name: "?GetInt64@Reflection@protobuf@google@@QEBA_JAEBVMessage@23@PEBVFieldDescriptor@23@@Z",
        ptr: crate::Ptr(refl_get_i64 as *const ()),
    },
    crate::Export {
        name: "?GetInt32@Reflection@protobuf@google@@QEBAHAEBVMessage@23@PEBVFieldDescriptor@23@@Z",
        ptr: crate::Ptr(refl_get_i32 as *const ()),
    },
    crate::Export {
        name: "?ListFields@Reflection@protobuf@google@@QEBAXAEBVMessage@23@PEAV?$vector@PEBVFieldDescriptor@protobuf@google@@V?$allocator@PEBVFieldDescriptor@protobuf@google@@@std@@@std@@@Z",
        ptr: crate::Ptr(refl_list_fields as *const ()),
    },
    crate::Export {
        name: "?FieldSize@Reflection@protobuf@google@@QEBAHAEBVMessage@23@PEBVFieldDescriptor@23@@Z",
        ptr: crate::Ptr(refl_get_i32 as *const ()),
    },
    crate::Export {
        name: "?GetReflection@Message@protobuf@google@@QEBAPEBVReflection@23@XZ",
        ptr: crate::Ptr(msg_get_reflection as *const ()),
    },
    crate::Export {
        name: "?Utf8DebugString@Message@protobuf@google@@QEBA?AV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@XZ",
        ptr: crate::Ptr(msg_utf8_debug_string as *const ()),
    },
    crate::Export {
        name: "?SerializeWithCachedSizesToArray@MessageLite@protobuf@google@@QEBAPEAEPEAE@Z",
        ptr: crate::Ptr(message_lite_serialize_with_cached_sizes_to_array as *const ()),
    },
    crate::Export {
        name: "?Base64Escape@protobuf@google@@YAXVStringPiece@12@PEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@Z",
        ptr: crate::Ptr(base64_escape as *const ()),
    },
    crate::Export {
        name: "?cpp_type@FieldDescriptor@protobuf@google@@QEBA?AW4CppType@123@XZ",
        ptr: crate::Ptr(field_descriptor_cpp_type as *const ()),
    },
    crate::Export {
        name: "?type@FieldDescriptor@protobuf@google@@QEBA?AW4Type@123@XZ",
        ptr: crate::Ptr(field_descriptor_type as *const ()),
    },
    crate::Export {
        name: "?EnumSize@WireFormatLite@internal@protobuf@google@@SA_KH@Z",
        ptr: crate::Ptr(wire_int32_size as *const ()),
    },
    // Extended protobuf v3/v4 wire-format writers, parsers, UTF-8, Base64, MessageLite & Arena exports
    crate::Export {
        name: "?WriteVarint64ToArray@CodedOutputStream@io@protobuf@google@@SAPEAE_KPEAE@Z",
        ptr: crate::Ptr(coded_write_varint64_to_array as *const ()),
    },
    crate::Export {
        name: "?WriteInt32ToArray@WireFormatLite@internal@protobuf@google@@SAPEAEHHPEAE@Z",
        ptr: crate::Ptr(wire_write_int32_to_array as *const ()),
    },
    crate::Export {
        name: "?WriteUInt64ToArray@WireFormatLite@internal@protobuf@google@@SAPEAEH_KPEAE@Z",
        ptr: crate::Ptr(wire_write_uint64_to_array as *const ()),
    },
    crate::Export {
        name: "?WriteSInt64ToArray@WireFormatLite@internal@protobuf@google@@SAPEAEH_JPEAE@Z",
        ptr: crate::Ptr(wire_write_sint64_to_array as *const ()),
    },
    crate::Export {
        name: "?WriteSFixed32ToArray@WireFormatLite@internal@protobuf@google@@SAPEAEHHPEAE@Z",
        ptr: crate::Ptr(wire_write_sfixed32_to_array as *const ()),
    },
    crate::Export {
        name: "?WriteFloatToArray@WireFormatLite@internal@protobuf@google@@SAPEAEHMPEAE@Z",
        ptr: crate::Ptr(crate::bridge_entry!(wire_write_float_to_array)),
    },
    crate::Export {
        name: "?WriteStringToArray@WireFormatLite@internal@protobuf@google@@SAPEAEHAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEAE@Z",
        ptr: crate::Ptr(wire_write_string_to_array as *const ()),
    },
    crate::Export {
        name: "?WriteBytesToArray@WireFormatLite@internal@protobuf@google@@SAPEAEHAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEAE@Z",
        ptr: crate::Ptr(wire_write_string_to_array as *const ()),
    },
    crate::Export {
        name: "?UInt64Size@WireFormatLite@internal@protobuf@google@@SA_K_K@Z",
        ptr: crate::Ptr(wire_uint64_size as *const ()),
    },
    crate::Export {
        name: "?SInt64Size@WireFormatLite@internal@protobuf@google@@SA_K_J@Z",
        ptr: crate::Ptr(wire_sint64_size as *const ()),
    },
    crate::Export {
        name: "?WriteStringMaybeAliasedOutline@EpsCopyOutputStream@io@protobuf@google@@AEAAPEAEIAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEAE@Z",
        ptr: crate::Ptr(eps_write_string_maybe_aliased as *const ()),
    },
    crate::Export {
        name: "?WriteBytesMaybeAliasedOutline@EpsCopyOutputStream@io@protobuf@google@@AEAAPEAEIAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEAE@Z",
        ptr: crate::Ptr(eps_write_string_maybe_aliased as *const ()),
    },
    crate::Export {
        name: "?WriteStringOutline@EpsCopyOutputStream@io@protobuf@google@@AEAAPEAEIAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEAE@Z",
        ptr: crate::Ptr(eps_write_string_maybe_aliased as *const ()),
    },
    crate::Export {
        name: "?WriteBytesOutline@EpsCopyOutputStream@io@protobuf@google@@AEAAPEAEIAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEAE@Z",
        ptr: crate::Ptr(eps_write_string_maybe_aliased as *const ()),
    },
    crate::Export {
        name: "?WriteRawFallback@EpsCopyOutputStream@io@protobuf@google@@AEAAPEAEPEBXHPEAE@Z",
        ptr: crate::Ptr(eps_write_raw as *const ()),
    },
    crate::Export {
        name: "?InlineGreedyStringParserUTF8@internal@protobuf@google@@YAPEBDPEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@PEBDPEAVParseContext@123@1@Z",
        ptr: crate::Ptr(inline_greedy_string_parser_utf8 as *const ()),
    },
    crate::Export {
        name: "?PackedInt32Parser@internal@protobuf@google@@YAPEBDPEAXPEBDPEAVParseContext@123@@Z",
        ptr: crate::Ptr(packed_int32_parser as *const ()),
    },
    crate::Export {
        name: "?PackedInt64Parser@internal@protobuf@google@@YAPEBDPEAXPEBDPEAVParseContext@123@@Z",
        ptr: crate::Ptr(packed_int64_parser as *const ()),
    },
    crate::Export {
        name: "?PackedUInt32Parser@internal@protobuf@google@@YAPEBDPEAXPEBDPEAVParseContext@123@@Z",
        ptr: crate::Ptr(packed_uint32_parser as *const ()),
    },
    crate::Export {
        name: "?PackedSInt32Parser@internal@protobuf@google@@YAPEBDPEAXPEBDPEAVParseContext@123@@Z",
        ptr: crate::Ptr(packed_sint32_parser as *const ()),
    },
    crate::Export {
        name: "?PackedSInt64Parser@internal@protobuf@google@@YAPEBDPEAXPEBDPEAVParseContext@123@@Z",
        ptr: crate::Ptr(packed_sint64_parser as *const ()),
    },
    crate::Export {
        name: "?PackedFixed32Parser@internal@protobuf@google@@YAPEBDPEAXPEBDPEAVParseContext@123@@Z",
        ptr: crate::Ptr(packed_fixed32_parser as *const ()),
    },
    crate::Export {
        name: "?PackedSFixed32Parser@internal@protobuf@google@@YAPEBDPEAXPEBDPEAVParseContext@123@@Z",
        ptr: crate::Ptr(packed_fixed32_parser as *const ()),
    },
    crate::Export {
        name: "?PackedSFixed64Parser@internal@protobuf@google@@YAPEBDPEAXPEBDPEAVParseContext@123@@Z",
        ptr: crate::Ptr(packed_fixed64_parser as *const ()),
    },
    crate::Export {
        name: "?PackedFloatParser@internal@protobuf@google@@YAPEBDPEAXPEBDPEAVParseContext@123@@Z",
        ptr: crate::Ptr(packed_fixed32_parser as *const ()),
    },
    crate::Export {
        name: "?PackedBoolParser@internal@protobuf@google@@YAPEBDPEAXPEBDPEAVParseContext@123@@Z",
        ptr: crate::Ptr(packed_bool_parser as *const ()),
    },
    crate::Export {
        name: "?PackedEnumParser@internal@protobuf@google@@YAPEBDPEAXPEBDPEAVParseContext@123@@Z",
        ptr: crate::Ptr(packed_int32_parser as *const ()),
    },
    crate::Export {
        name: "?VerifyUTF8StringNamedField@WireFormat@internal@protobuf@google@@SA_NPEBDHW4Operation@1234@0@Z",
        ptr: crate::Ptr(verify_utf8_string as *const ()),
    },
    crate::Export {
        name: "?VerifyUTF8StringFallback@WireFormat@internal@protobuf@google@@SA_NPEBDHW4Operation@1234@0@Z",
        ptr: crate::Ptr(verify_utf8_string as *const ()),
    },
    crate::Export {
        name: "?VerifyUTF8StringNamedField@WireFormatLite@internal@protobuf@google@@SA_NPEBDHW4Operation@1234@0@Z",
        ptr: crate::Ptr(verify_utf8_string as *const ()),
    },
    crate::Export {
        name: "?VerifyUTF8StringFallback@WireFormatLite@internal@protobuf@google@@SA_NPEBDHW4Operation@1234@0@Z",
        ptr: crate::Ptr(verify_utf8_string as *const ()),
    },
    crate::Export {
        name: "?VerifyUTF8@internal@protobuf@google@@YA_NV?$basic_string_view@DU?$char_traits@D@std@@@std@@PEBD@Z",
        ptr: crate::Ptr(verify_utf8 as *const ()),
    },
    crate::Export {
        name: "?WebSafeBase64Escape@protobuf@google@@YAXVStringPiece@12@PEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@Z",
        ptr: crate::Ptr(web_safe_base64_escape as *const ()),
    },
    crate::Export {
        name: "?Base64Escape@protobuf@google@@YAHPEBEHPEADH@Z",
        ptr: crate::Ptr(base64_escape_buf as *const ()),
    },
    crate::Export {
        name: "?WebSafeBase64Escape@protobuf@google@@YAHPEBEHPEADH_N@Z",
        ptr: crate::Ptr(web_safe_base64_escape_buf as *const ()),
    },
    crate::Export {
        name: "?SerializeToString@MessageLite@protobuf@google@@QEBA_NPEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@Z",
        ptr: crate::Ptr(message_lite_serialize_to_string as *const ()),
    },
    crate::Export {
        name: "?SerializePartialToString@MessageLite@protobuf@google@@QEBA_NPEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@Z",
        ptr: crate::Ptr(message_lite_serialize_partial_to_string as *const ()),
    },
    crate::Export {
        name: "?AppendToString@MessageLite@protobuf@google@@QEBA_NPEAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@Z",
        ptr: crate::Ptr(message_lite_append_to_string as *const ()),
    },
    crate::Export {
        name: "?SerializeToArray@MessageLite@protobuf@google@@QEBA_NPEAXH@Z",
        ptr: crate::Ptr(message_lite_serialize_to_array as *const ()),
    },
    crate::Export {
        name: "?SerializePartialToArray@MessageLite@protobuf@google@@QEBA_NPEAXH@Z",
        ptr: crate::Ptr(message_lite_serialize_partial_to_array as *const ()),
    },
    crate::Export {
        name: "?ParseFromArray@MessageLite@protobuf@google@@QEAA_NPEBXH@Z",
        ptr: crate::Ptr(message_lite_parse_from_array as *const ()),
    },
    crate::Export {
        name: "?ParsePartialFromArray@MessageLite@protobuf@google@@QEAA_NPEBXH@Z",
        ptr: crate::Ptr(message_lite_parse_partial_from_array as *const ()),
    },
    crate::Export {
        name: "?ParseFromString@MessageLite@protobuf@google@@QEAA_NAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@Z",
        ptr: crate::Ptr(message_lite_parse_from_string as *const ()),
    },
    crate::Export {
        name: "?ParsePartialFromString@MessageLite@protobuf@google@@QEAA_NAEBV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@Z",
        ptr: crate::Ptr(message_lite_parse_partial_from_string as *const ()),
    },
    crate::Export {
        name: "?ParseFromString@MessageLite@protobuf@google@@QEAA_NVStringPiece@23@@Z",
        ptr: crate::Ptr(message_lite_parse_from_string_piece as *const ()),
    },
    crate::Export {
        name: "?ParseFromString@MessageLite@protobuf@google@@QEAA_NV?$basic_string_view@DU?$char_traits@D@std@@@std@@@Z",
        ptr: crate::Ptr(message_lite_parse_from_string_piece as *const ()),
    },
    crate::Export {
        name: "??0Arena@protobuf@google@@QEAA@XZ",
        ptr: crate::Ptr(arena_ctor as *const ()),
    },
    crate::Export {
        name: "??0Arena@protobuf@google@@QEAA@AEBUArenaOptions@12@@Z",
        ptr: crate::Ptr(arena_ctor_options as *const ()),
    },
    crate::Export {
        name: "??1Arena@protobuf@google@@QEAA@XZ",
        ptr: crate::Ptr(arena_dtor as *const ()),
    },
    crate::Export {
        name: "?Reset@Arena@protobuf@google@@QEAA_KXZ",
        ptr: crate::Ptr(arena_reset as *const ()),
    },
    crate::Export {
        name: "?SpaceAllocated@Arena@protobuf@google@@QEBA_KXZ",
        ptr: crate::Ptr(arena_space_allocated as *const ()),
    },
    crate::Export {
        name: "?SpaceUsed@Arena@protobuf@google@@QEBA_KXZ",
        ptr: crate::Ptr(arena_space_allocated as *const ()),
    },
    crate::Export {
        name: "?AllocateAligned@Arena@protobuf@google@@AEAAPEAX_K0@Z",
        ptr: crate::Ptr(arena_allocate_aligned as *const ()),
    },
    crate::Export {
        name: "?OwnDestructor@Arena@protobuf@google@@AEAAXPEAXP6AX0@Z@Z",
        ptr: crate::Ptr(arena_own_destructor as *const ()),
    },
    crate::Export {
        name: "?Init@Arena@protobuf@google@@AEAAX_N@Z",
        ptr: crate::Ptr(arena_init as *const ()),
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::sync::atomic::{AtomicU32, Ordering};

    unsafe fn make_msvc_string(bytes: &[u8]) -> MsvcString {
        let mut s = MsvcString {
            buf: [0u8; 16],
            size: 0,
            cap: 15,
        };
        msvc_string_assign_bytes(&mut s, bytes);
        s
    }

    unsafe fn msvc_string_to_vec(s: *const MsvcString) -> Vec<u8> {
        let (ptr, len) = msvc_string_data_and_len(s);
        if len == 0 || ptr.is_null() {
            Vec::new()
        } else {
            std::slice::from_raw_parts(ptr, len).to_vec()
        }
    }

    #[test]
    fn exports_count_and_core_semantics() {
        assert!(EXTRA_EXPORTS.len() >= 122);
        let mut unique = HashSet::new();
        for e in EXTRA_EXPORTS {
            assert!(unique.insert(e.name), "duplicate export: {}", e.name);
            assert!(!e.ptr.0.is_null(), "null export pointer: {}", e.name);
        }

        unsafe {
            // 1. Empty string & ArenaStringPtr
            let empty = get_empty_string_already_inited();
            assert!(!empty.is_null());
            assert_eq!(std::ptr::read_unaligned(empty).size, 0);
            assert_eq!(std::ptr::read_unaligned(empty).cap, 15);

            let mut asp: usize = 0;
            arena_string_ptr_ctor(&mut asp, empty as usize);
            assert_eq!(arena_string_ptr_get(&asp), empty);

            let src = make_msvc_string(b"otlp");
            arena_string_ptr_set_copy(&mut asp, 0, &src, std::ptr::null_mut());
            let got = arena_string_ptr_get(&asp);
            assert_ne!(got, empty);
            let got_val = std::ptr::read_unaligned(got);
            assert_eq!(got_val.size, 4);
            assert_eq!(&got_val.buf[0..4], b"otlp");

            arena_string_ptr_clear_to_empty(&mut asp);
            let got_cleared = std::ptr::read_unaligned(got);
            assert_eq!(got_cleared.size, 0);
            assert_eq!(got_cleared.buf[0], 0);

            arena_string_ptr_destroy(&mut asp, 0, std::ptr::null_mut());
            assert_eq!(asp, empty as usize);

            // 2. RepeatedPtrFieldBase
            let mut rpf = RepeatedPtrFieldBase {
                arena: 0,
                current_size: 0,
                total_size: 0,
                rep: std::ptr::null_mut(),
            };
            repeated_ptr_field_base_reserve(&mut rpf, 3);
            assert!(rpf.total_size >= 4);
            assert!(!rpf.rep.is_null());
            repeated_ptr_field_base_dtor(&mut rpf);
            assert!(rpf.rep.is_null());

            // 3. RepeatedField<u64>
            let mut rf = RepeatedField64 {
                current_size: 0,
                total_size: 0,
                ptr: std::ptr::null_mut(),
            };
            repeated_field_ctor(&mut rf, std::ptr::null_mut());
            assert_eq!(repeated_field_size(&rf), 0);
            assert!(repeated_field_data(&rf).is_null());

            let v1 = 42u64;
            let v2 = 99u64;
            repeated_field_add(&mut rf, &v1);
            repeated_field_add(&mut rf, &v2);
            assert_eq!(repeated_field_size(&rf), 2);
            let data = repeated_field_data(&rf);
            assert!(!data.is_null());
            assert_eq!(std::ptr::read_unaligned(data), 42);
            assert_eq!(std::ptr::read_unaligned(data.add(1)), 99);
            repeated_field_dtor(&mut rf);

            // 4. Message & sret string methods
            let mut msg = [0usize; 2];
            msg_ctor_default(msg.as_mut_ptr());
            assert_ne!(msg[0], 0);
            assert_eq!(msg[1], 0);
            assert!(message_lite_get_arena(msg.as_ptr() as *const u8).is_null());

            let mut dbg_str = MsvcString {
                buf: [0xff; 16],
                size: 123,
                cap: 456,
            };
            let ret = msg_utf8_debug_string(msg.as_ptr() as *const u8, &mut dbg_str);
            assert_eq!(ret, &mut dbg_str as *mut MsvcString);
            assert_eq!(dbg_str.size, 0);
            assert_eq!(dbg_str.cap, 15);
        }
    }

    #[test]
    fn varint32_and_varint64_round_trip_and_overflow() {
        unsafe {
            // 1..5 byte varint32 values
            let vals32: &[(u32, usize)] = &[
                (0, 1),
                (1, 1),
                (127, 1),
                (128, 2),
                (300, 2),
                ((1 << 14) - 1, 2),
                (1 << 14, 3),
                ((1 << 21) - 1, 3),
                (1 << 21, 4),
                ((1 << 28) - 1, 4),
                (1 << 28, 5),
                (u32::MAX, 5),
            ];
            for &(v, expected_len) in vals32 {
                let mut buf = [0u8; 16];
                let end = coded_write_varint32_to_array(v, buf.as_mut_ptr());
                let len = end as usize - buf.as_ptr() as usize;
                assert_eq!(len, expected_len, "varint32 len for {v}");
                assert_eq!(varint32_size(v), expected_len);

                let mut pair = [0usize; 2];
                varint_parse_slow32(&mut pair, buf.as_ptr(), buf[0] as u32);
                assert_eq!(pair[0], buf.as_ptr().add(len) as usize);
                assert_eq!(pair[1] as u32, v);
            }

            // 10-byte sign-extended negative int32 (-1) decoded by VarintParseSlow32
            let neg1_wire = [0xffu8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0];
            let mut pair = [0usize; 2];
            varint_parse_slow32(&mut pair, neg1_wire.as_ptr(), neg1_wire[0] as u32);
            assert_eq!(pair[0], neg1_wire.as_ptr().add(10) as usize);
            assert_eq!(pair[1] as u32, u32::MAX);

            // 1..10 byte varint64 values
            let vals64: &[(u64, usize)] = &[
                (0, 1),
                (0x7f, 1),
                (0x80, 2),
                (1 << 14, 3),
                (1 << 21, 4),
                (1 << 28, 5),
                (1 << 35, 6),
                (1 << 42, 7),
                (1 << 49, 8),
                (1 << 56, 9),
                (1 << 63, 10),
                (u64::MAX, 10),
            ];
            for &(v, expected_len) in vals64 {
                let mut buf = [0u8; 16];
                let end = coded_write_varint64_to_array(v, buf.as_mut_ptr());
                let len = end as usize - buf.as_ptr() as usize;
                assert_eq!(len, expected_len, "varint64 len for {v:#x}");
                assert_eq!(varint64_size(v), expected_len);

                let mut out = [0usize; 2];
                varint_parse_slow64(&mut out, buf.as_ptr(), buf[0] as u32);
                assert_eq!(out[0], buf.as_ptr().add(len) as usize);
                assert_eq!(out[1] as u64, v);
            }

            // Malformed >10-byte varints must return null
            let overlong = [0x80u8; 12];
            let mut bad32 = [1usize; 2];
            varint_parse_slow32(&mut bad32, overlong.as_ptr(), overlong[0] as u32);
            assert_eq!(bad32[0], 0);
            let mut bad64 = [1usize; 2];
            varint_parse_slow64(&mut bad64, overlong.as_ptr(), overlong[0] as u32);
            assert_eq!(bad64[0], 0);

            // ReadTagFallback: multi-byte tag and >5-byte rejection
            let mut tag_buf = [0u8; 16];
            let tag_end = wire_write_tag_to_array(100_000, WIRETYPE_LENGTH_DELIMITED, tag_buf.as_mut_ptr());
            let tag_len = tag_end as usize - tag_buf.as_ptr() as usize;
            assert_eq!(tag_len, 3);
            let mut tag_pair = [0usize; 2];
            read_tag_fallback(&mut tag_pair, tag_buf.as_ptr(), 0);
            assert_eq!(tag_pair[0], tag_buf.as_ptr().add(tag_len) as usize);
            assert_eq!(tag_pair[1] as u32, (100_000 << 3) | WIRETYPE_LENGTH_DELIMITED);

            let bad_tag = [0x80u8; 8];
            read_tag_fallback(&mut tag_pair, bad_tag.as_ptr(), 0);
            assert_eq!(tag_pair[0], 0);

            // ReadSizeFallback: valid multi-byte size and >= 2GiB rejection
            let mut sz_buf = [0u8; 16];
            let sz_end = coded_write_varint32_to_array(65536, sz_buf.as_mut_ptr());
            let sz_len = sz_end as usize - sz_buf.as_ptr() as usize;
            let mut sz_pair = [0usize; 2];
            read_size_fallback(&mut sz_pair, sz_buf.as_ptr(), sz_buf[0] as u32);
            assert_eq!(sz_pair[0], sz_buf.as_ptr().add(sz_len) as usize);
            assert_eq!(sz_pair[1] as i32, 65536);

            let huge_sz = [0x80u8, 0x80, 0x80, 0x80, 0x08]; // 2 GiB
            read_size_fallback(&mut sz_pair, huge_sz.as_ptr(), huge_sz[0] as u32);
            assert_eq!(sz_pair[0], 0);
        }
    }

    #[test]
    fn wire_format_lite_and_eps_copy_output_stream() {
        unsafe {
            let mut buf = [0u8; 256];
            let mut p = buf.as_mut_ptr();

            // Field 1: uint32 = 150 -> 0x08 0x96 0x01
            p = wire_write_uint32_to_array(1, 150, p);
            assert_eq!(&buf[0..3], &[0x08, 0x96, 0x01]);

            // Field 2: string = "testing" -> 0x12 0x07 "testing"
            let mut s = make_msvc_string(b"testing");
            let p_str_start = p as usize - buf.as_ptr() as usize;
            p = wire_write_string_to_array(2, &s, p);
            assert_eq!(&buf[p_str_start..p_str_start + 9], b"\x12\x07testing");

            // Field 3: fixed64 = 0x0102_0304_0506_0708 -> 0x19 + 8 LE bytes
            let p_f64_start = p as usize - buf.as_ptr() as usize;
            p = wire_write_fixed64_to_array(3, 0x0102_0304_0506_0708, p);
            assert_eq!(
                &buf[p_f64_start..p_f64_start + 9],
                &[0x19, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]
            );

            // Field 4: sint32 = -2 -> zigzag( -2 ) = 3 -> 0x20 0x03
            let p_si_start = p as usize - buf.as_ptr() as usize;
            p = wire_write_sint32_to_array(4, -2, p);
            assert_eq!(&buf[p_si_start..p_si_start + 2], &[0x20, 0x03]);

            // Field 5: sint64 = -1 -> zigzag(-1) = 1 -> 0x28 0x01
            let p_si64_start = p as usize - buf.as_ptr() as usize;
            p = wire_write_sint64_to_array(5, -1, p);
            assert_eq!(&buf[p_si64_start..p_si64_start + 2], &[0x28, 0x01]);

            // Field 6: float = 1.5f32 -> 0x35 + 4 LE bytes
            let p_flt_start = p as usize - buf.as_ptr() as usize;
            p = wire_write_float_to_array(6, 1.5f32, p);
            let mut expected_flt = [0u8; 5];
            expected_flt[0] = (6 << 3) | WIRETYPE_FIXED32 as u8;
            expected_flt[1..5].copy_from_slice(&1.5f32.to_bits().to_le_bytes());
            assert_eq!(&buf[p_flt_start..p_flt_start + 5], &expected_flt);

            // Field 7: double = -3.25f64 -> 0x39 + 8 LE bytes
            let p_dbl_start = p as usize - buf.as_ptr() as usize;
            p = wire_write_double_to_array(7, -3.25f64, p);
            let mut expected_dbl = [0u8; 9];
            expected_dbl[0] = (7 << 3) | WIRETYPE_FIXED64 as u8;
            expected_dbl[1..9].copy_from_slice(&(-3.25f64).to_bits().to_le_bytes());
            assert_eq!(&buf[p_dbl_start..p_dbl_start + 9], &expected_dbl);

            // Field 8: int32 = -1 -> tag 0x40 + 10-byte sign-extended varint
            let p_neg_start = p as usize - buf.as_ptr() as usize;
            p = wire_write_int32_to_array(8, -1, p);
            assert_eq!(p as usize - buf.as_ptr() as usize - p_neg_start, 11);
            assert_eq!(buf[p_neg_start], 0x40);
            assert_eq!(&buf[p_neg_start + 1..p_neg_start + 11], &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]);

            // Field 9: EpsCopyOutputStream WriteStringMaybeAliased + WriteRaw + WriteLengthDelim
            let mut eps = init_flat_output_stream(buf.as_mut_ptr(), buf.len());
            let p_eps_start = p as usize - buf.as_ptr() as usize;
            let mut long_s = make_msvc_string(b"0123456789abcdef_heap_str");
            p = eps_write_string_maybe_aliased(&mut eps, 9, &long_s, p);
            let eps_written = p as usize - buf.as_ptr() as usize - p_eps_start;
            assert_eq!(eps_written, 1 + 1 + 25);
            assert_eq!(buf[p_eps_start], (9 << 3) | WIRETYPE_LENGTH_DELIMITED as u8);
            assert_eq!(buf[p_eps_start + 1], 25);
            assert_eq!(&buf[p_eps_start + 2..p_eps_start + 27], b"0123456789abcdef_heap_str");

            let p_ld_start = p as usize - buf.as_ptr() as usize;
            p = eps_write_length_delim(&mut eps, 10, 4, p);
            p = eps_write_raw(&mut eps, b"ABCD".as_ptr(), 4, p);
            assert_eq!(&buf[p_ld_start..p_ld_start + 6], b"\x52\x04ABCD");
            assert!(p as usize <= buf.as_ptr() as usize + buf.len());

            free_msvc_string_buf(&mut s);
            free_msvc_string_buf(&mut long_s);
        }
    }

    #[test]
    fn verify_utf8_valid_and_invalid() {
        unsafe {
            let valid_cases: &[&[u8]] = &[
                b"",
                b"hello world",
                "caf\u{00e9} \u{2603} \u{1f680}".as_bytes(),
                "\u{4e2d}\u{6587}".as_bytes(),
            ];
            for &v in valid_cases {
                assert_eq!(
                    verify_utf8_string(v.as_ptr(), v.len() as i32, 0, std::ptr::null()),
                    1
                );
                let sp = [v.as_ptr() as usize, v.len()];
                assert_eq!(verify_utf8(&sp, std::ptr::null()), 1);
            }

            let invalid_cases: &[&[u8]] = &[
                &[0x80],                   // stray continuation byte
                &[0xc0, 0xaf],             // overlong '/'
                &[0xe2, 0x82],             // truncated 3-byte sequence
                &[0xed, 0xa0, 0x80],       // UTF-16 surrogate U+D800
                &[0xf4, 0x90, 0x80, 0x80], // > U+10FFFF
                &[0xff, 0xfe],             // invalid lead bytes
            ];
            for &inv in invalid_cases {
                assert_eq!(
                    verify_utf8_string(inv.as_ptr(), inv.len() as i32, 0, std::ptr::null()),
                    0,
                    "expected invalid UTF-8 for {inv:x?}"
                );
                let sp = [inv.as_ptr() as usize, inv.len()];
                assert_eq!(verify_utf8(&sp, std::ptr::null()), 0);
            }

            // Negative length rejected
            assert_eq!(verify_utf8_string(b"a".as_ptr(), -1, 0, std::ptr::null()), 0);
        }
    }

    #[test]
    fn base64_escape_rfc4648_vectors() {
        unsafe {
            // RFC 4648 Section 10 test vectors
            let vectors: &[(&[u8], &str, &str)] = &[
                (b"", "", ""),
                (b"f", "Zg==", "Zg"),
                (b"fo", "Zm8=", "Zm8"),
                (b"foo", "Zm9v", "Zm9v"),
                (b"foob", "Zm9vYg==", "Zm9vYg"),
                (b"fooba", "Zm9vYmE=", "Zm9vYmE"),
                (b"foobar", "Zm9vYmFy", "Zm9vYmFy"),
                (&[0xfb, 0xff, 0xfe], "+//+", "-__-"),
                (
                    b"The quick brown fox jumps over the lazy dog",
                    "VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZw==",
                    "VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZw",
                ),
            ];
            for &(input, expected_std, expected_web) in vectors {
                let sp = [input.as_ptr() as usize, input.len()];
                let mut dst_std = make_msvc_string(b"old");
                base64_escape(&sp, &mut dst_std);
                assert_eq!(msvc_string_to_vec(&dst_std), expected_std.as_bytes());
                free_msvc_string_buf(&mut dst_std);

                let mut dst_web = make_msvc_string(b"old");
                web_safe_base64_escape(&sp, &mut dst_web);
                assert_eq!(msvc_string_to_vec(&dst_web), expected_web.as_bytes());
                free_msvc_string_buf(&mut dst_web);
            }

            // Raw buffer overload
            let mut raw_out = [0u8; 16];
            let n = base64_escape_buf(b"foobar".as_ptr(), 6, raw_out.as_mut_ptr(), 16);
            assert_eq!(n, 8);
            assert_eq!(&raw_out[..8], b"Zm9vYmFy");
            let wn = web_safe_base64_escape_buf(
                [0xfb, 0xff, 0xfe].as_ptr(),
                3,
                raw_out.as_mut_ptr(),
                16,
                0,
            );
            assert_eq!(wn, 4);
            assert_eq!(&raw_out[..4], b"-__-");
            // Buffer too small returns 0
            assert_eq!(base64_escape_buf(b"foobar".as_ptr(), 6, raw_out.as_mut_ptr(), 4), 0);
        }
    }

    #[test]
    fn parse_context_strings_packed_and_unknown_fields() {
        unsafe {
            // Test InlineGreedyStringParser across >16-byte buffer (crossing patch boundary!)
            let mut wire = Vec::new();
            let mut tmp = [0u8; 64];
            let mut s1 = make_msvc_string(b"hello_short");
            let mut s2 = make_msvc_string(b"second_longer_heap_string_value");
            let p1 = wire_write_string_to_array(1, &s1, tmp.as_mut_ptr());
            wire.extend_from_slice(&tmp[..p1 as usize - tmp.as_ptr() as usize]);
            let p2 = wire_write_string_to_array(2, &s2, tmp.as_mut_ptr());
            wire.extend_from_slice(&tmp[..p2 as usize - tmp.as_ptr() as usize]);

            let mut ctx = EpsCopyInputStream {
                limit_end: std::ptr::null(),
                buffer_end: std::ptr::null(),
                next_chunk: std::ptr::null(),
                size: 0,
                limit: 0,
                zcis: std::ptr::null_mut(),
                buffer: [0; 32],
                aliasing: 0,
                depth: MAX_GROUP_DEPTH as i32,
            };
            let mut ptr = init_parse_context(&mut ctx, wire.as_ptr(), wire.len());
            let ctx_ptr = &mut ctx as *mut EpsCopyInputStream as *mut u8;

            assert_eq!(parse_context_done(ctx_ptr, &mut ptr), 0);
            let (after_t1, t1) = ctx_read_tag(&mut ctx, ptr).unwrap();
            assert_eq!(t1, (1 << 3) | WIRETYPE_LENGTH_DELIMITED);
            let mut out1 = make_msvc_string(b"");
            ptr = inline_greedy_string_parser_utf8(&mut out1, after_t1, ctx_ptr, std::ptr::null());
            assert!(!ptr.is_null());
            assert_eq!(msvc_string_to_vec(&out1), b"hello_short");

            assert_eq!(parse_context_done(ctx_ptr, &mut ptr), 0);
            let (after_t2, t2) = ctx_read_tag(&mut ctx, ptr).unwrap();
            assert_eq!(t2, (2 << 3) | WIRETYPE_LENGTH_DELIMITED);
            let mut out2 = make_msvc_string(b"");
            ptr = inline_greedy_string_parser(&mut out2, after_t2, ctx_ptr);
            assert!(!ptr.is_null());
            assert_eq!(msvc_string_to_vec(&out2), b"second_longer_heap_string_value");
            assert_eq!(parse_context_done(ctx_ptr, &mut ptr), 1);
            assert!(!ptr.is_null());

            // Truncated string must return null (buffer overrun check)
            let trunc = [10u8, b'a', b'b'];
            let mut tctx = ctx;
            let tptr = init_parse_context(&mut tctx, trunc.as_ptr(), trunc.len());
            let mut out_bad = make_msvc_string(b"");
            let r = inline_greedy_string_parser(
                &mut out_bad,
                tptr,
                &mut tctx as *mut EpsCopyInputStream as *mut u8,
            );
            assert!(r.is_null());

            // Packed parsers: int32, uint64, double, bool, float
            let mut packed_i32_wire = vec![0u8; 32];
            let mut wp = packed_i32_wire.as_mut_ptr().add(1);
            wp = write_varint32_raw(7, wp);
            wp = write_varint64_raw((-5i32) as i64 as u64, wp);
            wp = write_varint32_raw(300, wp);
            let body_len = wp as usize - packed_i32_wire.as_ptr().add(1) as usize;
            packed_i32_wire[0] = body_len as u8;
            let mut rf_i32 = RepeatedFieldRaw {
                current_size: 0,
                total_size: 0,
                ptr: std::ptr::null_mut(),
            };
            let mut pctx = ctx;
            let pptr = init_parse_context(&mut pctx, packed_i32_wire.as_ptr(), body_len + 1);
            let endp = packed_int32_parser(
                &mut rf_i32 as *mut RepeatedFieldRaw as *mut u8,
                pptr,
                &mut pctx as *mut EpsCopyInputStream as *mut u8,
            );
            assert!(!endp.is_null());
            assert_eq!(rf_i32.current_size, 3);
            let slice_i32 = std::slice::from_raw_parts(rf_i32.ptr as *const i32, 3);
            assert_eq!(slice_i32, &[7, -5, 300]);
            repeated_field_dtor(&mut rf_i32 as *mut RepeatedFieldRaw as *mut RepeatedField64);

            // Packed double + bad non-multiple-of-8 rejection
            let mut packed_dbl = Vec::new();
            packed_dbl.push(16u8);
            packed_dbl.extend_from_slice(&1.25f64.to_bits().to_le_bytes());
            packed_dbl.extend_from_slice(&(-9.5f64).to_bits().to_le_bytes());
            let mut rf_dbl = RepeatedField64 {
                current_size: 0,
                total_size: 0,
                ptr: std::ptr::null_mut(),
            };
            let mut dctx = ctx;
            let dptr = init_parse_context(&mut dctx, packed_dbl.as_ptr(), packed_dbl.len());
            let dend = packed_fixed64_parser(
                &mut rf_dbl as *mut RepeatedField64 as *mut u8,
                dptr,
                &mut dctx as *mut EpsCopyInputStream as *mut u8,
            );
            assert!(!dend.is_null());
            assert_eq!(rf_dbl.current_size, 2);
            assert_eq!(f64::from_bits(std::ptr::read_unaligned(rf_dbl.ptr)), 1.25);
            assert_eq!(f64::from_bits(std::ptr::read_unaligned(rf_dbl.ptr.add(1))), -9.5);
            repeated_field_dtor(&mut rf_dbl);

            let bad_dbl = [5u8, 1, 2, 3, 4, 5];
            let mut bdctx = ctx;
            let bdptr = init_parse_context(&mut bdctx, bad_dbl.as_ptr(), bad_dbl.len());
            let mut rf_bd = RepeatedField64 {
                current_size: 0,
                total_size: 0,
                ptr: std::ptr::null_mut(),
            };
            assert!(packed_fixed64_parser(
                &mut rf_bd as *mut RepeatedField64 as *mut u8,
                bdptr,
                &mut bdctx as *mut EpsCopyInputStream as *mut u8,
            )
            .is_null());

            // Packed bool
            let packed_bool = [3u8, 0, 1, 42];
            let mut rf_bool = RepeatedFieldRaw {
                current_size: 0,
                total_size: 0,
                ptr: std::ptr::null_mut(),
            };
            let mut bctx = ctx;
            let bptr = init_parse_context(&mut bctx, packed_bool.as_ptr(), packed_bool.len());
            let bend = packed_bool_parser(
                &mut rf_bool as *mut RepeatedFieldRaw as *mut u8,
                bptr,
                &mut bctx as *mut EpsCopyInputStream as *mut u8,
            );
            assert!(!bend.is_null());
            assert_eq!(rf_bool.current_size, 3);
            assert_eq!(std::slice::from_raw_parts(rf_bool.ptr, 3), &[0, 1, 1]);
            repeated_field_dtor(&mut rf_bool as *mut RepeatedFieldRaw as *mut RepeatedField64);

            // UnknownFieldParse + InternalSerializeUnknownFieldsToArray round-trip (varint + group + fixed32)
            let mut ufs = [0usize; 2];
            unknown_field_set_ctor(&mut ufs);
            let mut uwire = [0u8; 64];
            let up1 = wire_write_uint32_to_array(11, 777, uwire.as_mut_ptr());
            let up2 = wire_write_tag_to_array(12, WIRETYPE_START_GROUP, up1);
            let up3 = wire_write_fixed32_to_array(1, 0x1122_3344, up2);
            let up4 = wire_write_tag_to_array(12, WIRETYPE_END_GROUP, up3);
            let ulen = up4 as usize - uwire.as_ptr() as usize;
            let mut uctx = ctx;
            let mut uptr = init_parse_context(&mut uctx, uwire.as_ptr(), ulen);
            let uctx_raw = &mut uctx as *mut EpsCopyInputStream as *mut u8;
            while parse_context_done(uctx_raw, &mut uptr) == 0 {
                let (after_utag, utag) = ctx_read_tag(&mut uctx, uptr).unwrap();
                uptr = unknown_field_parse(
                    utag as u64,
                    &mut ufs as *mut [usize; 2] as *mut u8,
                    after_utag,
                    uctx_raw,
                );
                assert!(!uptr.is_null());
            }
            let mut reser = [0u8; 64];
            let mut reser_eps = init_flat_output_stream(reser.as_mut_ptr(), reser.len());
            let rend = internal_serialize_unknown_fields_to_array(
                &ufs as *const [usize; 2] as *const u8,
                reser.as_mut_ptr(),
                &mut reser_eps,
            );
            assert_eq!(rend as usize - reser.as_ptr() as usize, ulen);
            assert_eq!(&reser[..ulen], &uwire[..ulen]);

            // PushLimit / PopLimit across patch boundary + overrun check
            let mut lim_ctx = ctx;
            let mut lptr = init_parse_context(&mut lim_ctx, wire.as_ptr(), wire.len());
            let lim_raw = &mut lim_ctx as *mut EpsCopyInputStream as *mut u8;
            let delta = eps_push_limit(lim_raw, lptr, 20);
            assert!(delta >= 0);
            let (after_sub, sub_bytes) = ctx_read_bytes(&mut lim_ctx, lptr, 20).unwrap();
            assert_eq!(sub_bytes.as_slice(), &wire[..20]);
            lptr = after_sub;
            assert_eq!(parse_context_done(lim_raw, &mut lptr), 1);
            assert!(!lptr.is_null());
            assert_eq!(eps_pop_limit(lim_raw, delta), 1);
            assert_eq!(parse_context_done(lim_raw, &mut lptr), 0);

            // Packed SInt32 + Float + UInt64 parsers
            let packed_si32 = [3u8, 0, 1, 2]; // zigzag: 0, -1, 1
            let mut rf_si32 = RepeatedFieldRaw {
                current_size: 0,
                total_size: 0,
                ptr: std::ptr::null_mut(),
            };
            let mut sictx = ctx;
            let siptr = init_parse_context(&mut sictx, packed_si32.as_ptr(), packed_si32.len());
            assert!(!packed_sint32_parser(
                &mut rf_si32 as *mut RepeatedFieldRaw as *mut u8,
                siptr,
                &mut sictx as *mut EpsCopyInputStream as *mut u8,
            )
            .is_null());
            assert_eq!(std::slice::from_raw_parts(rf_si32.ptr as *const i32, 3), &[0, -1, 1]);
            repeated_field_dtor(&mut rf_si32 as *mut RepeatedFieldRaw as *mut RepeatedField64);

            unknown_field_set_dtor(&mut ufs);

            free_msvc_string_buf(&mut s1);
            free_msvc_string_buf(&mut s2);
            free_msvc_string_buf(&mut out1);
            free_msvc_string_buf(&mut out2);
            free_msvc_string_buf(&mut out_bad);
        }
    }

    #[test]
    fn eps_copy_output_stream_chunked_zero_copy_stream() {
        #[repr(C)]
        struct ChunkStream {
            vtbl: *const [crate::Ptr; 4],
            chunks: [[u8; 8]; 3],
            next_idx: usize,
            backed_up: i32,
        }

        unsafe extern "win64" fn chunk_dtor(this: *mut u8, _flags: u32) -> *mut u8 {
            this
        }
        unsafe extern "win64" fn chunk_next(
            this: *mut u8,
            data_out: *mut *mut u8,
            size_out: *mut i32,
        ) -> u8 {
            let s = &mut *(this as *mut ChunkStream);
            if s.next_idx >= s.chunks.len() {
                return 0;
            }
            let ptr = s.chunks[s.next_idx].as_mut_ptr();
            s.next_idx += 1;
            std::ptr::write_unaligned(data_out, ptr);
            std::ptr::write_unaligned(size_out, 8);
            1
        }
        unsafe extern "win64" fn chunk_backup(this: *mut u8, count: i32) {
            let s = &mut *(this as *mut ChunkStream);
            s.backed_up += count;
        }
        unsafe extern "win64" fn chunk_byte_count(_this: *const u8) -> i64 {
            0
        }

        static CHUNK_VTBL: [crate::Ptr; 4] = [
            crate::Ptr(chunk_dtor as *const ()),
            crate::Ptr(chunk_next as *const ()),
            crate::Ptr(chunk_backup as *const ()),
            crate::Ptr(chunk_byte_count as *const ()),
        ];

        unsafe {
            let mut zc = ChunkStream {
                vtbl: &CHUNK_VTBL,
                chunks: [[0u8; 8]; 3],
                next_idx: 0,
                backed_up: 0,
            };
            let mut eps = EpsCopyOutputStream {
                end: std::ptr::null_mut(),
                stream: &mut zc as *mut ChunkStream as *mut u8,
                had_error: 0,
                aliasing_enabled: 0,
                is_deterministic: 0,
                _pad: [0; 5],
                buffer: [0; 32],
            };
            let mut ptr = eps_ensure_space_fallback(&mut eps, eps.buffer.as_mut_ptr());
            assert_eq!(eps.had_error, 0);
            let payload = b"0123456789abcdefghij"; // 20 bytes across three 8-byte chunks
            ptr = eps_write_raw(&mut eps, payload.as_ptr(), payload.len() as i32, ptr);
            assert_eq!(eps.had_error, 0);
            assert_eq!(zc.backed_up, 0);
            assert_eq!(&zc.chunks[0], b"01234567");
            assert_eq!(&zc.chunks[1], b"89abcdef");
            assert_eq!(&zc.chunks[2][0..4], b"ghij");

            // Exhausting the 24-byte stream sets had_error without crashing
            let overflow = b"XXXXXXXX";
            let _ = eps_write_raw(&mut eps, overflow.as_ptr(), overflow.len() as i32, ptr);
            assert_eq!(eps.had_error, 1);
        }
    }

    #[test]
    fn message_lite_serialize_and_parse_round_trip() {
        #[repr(C)]
        struct SampleMsg {
            vtbl: *const [crate::Ptr; 25],
            internal_metadata: usize,
            id: u32,
            label: MsvcString,
            cached_size: std::cell::Cell<i32>,
        }

        unsafe extern "win64" fn sample_clear(this: *mut u8) {
            let m = &mut *(this as *mut SampleMsg);
            m.id = 0;
            msvc_string_clear(&mut m.label);
            m.cached_size.set(0);
        }

        unsafe extern "win64" fn sample_byte_size_long(this: *const u8) -> usize {
            let m = &*(this as *const SampleMsg);
            let mut sz = 0usize;
            if m.id != 0 {
                sz += 1 + varint32_size(m.id);
            }
            if m.label.size > 0 {
                sz += 1 + wire_string_size(&m.label);
            }
            m.cached_size.set(sz as i32);
            sz
        }

        unsafe extern "win64" fn sample_get_cached_size(this: *const u8) -> i32 {
            (*(this as *const SampleMsg)).cached_size.get()
        }

        unsafe extern "win64" fn sample_internal_serialize(
            this: *const u8,
            mut target: *mut u8,
            stream: *mut u8,
        ) -> *mut u8 {
            let m = &*(this as *const SampleMsg);
            if m.id != 0 {
                target = wire_write_uint32_to_array(1, m.id, target);
            }
            if m.label.size > 0 {
                target = eps_write_string_maybe_aliased(
                    stream as *mut EpsCopyOutputStream,
                    2,
                    &m.label,
                    target,
                );
            }
            target
        }

        unsafe extern "win64" fn sample_internal_parse(
            this: *mut u8,
            mut ptr: *const u8,
            ctx: *mut u8,
        ) -> *const u8 {
            let m = &mut *(this as *mut SampleMsg);
            let eps = ctx as *mut EpsCopyInputStream;
            while parse_context_done(ctx, &mut ptr) == 0 {
                let Some((after_tag, tag)) = ctx_read_tag(eps, ptr) else {
                    return std::ptr::null();
                };
                match tag >> 3 {
                    1 => {
                        let Some((next, v)) = ctx_read_varint64(eps, after_tag) else {
                            return std::ptr::null();
                        };
                        m.id = v as u32;
                        ptr = next;
                    }
                    2 => {
                        ptr = inline_greedy_string_parser(&mut m.label, after_tag, ctx);
                        if ptr.is_null() {
                            return std::ptr::null();
                        }
                    }
                    _ => {
                        ptr = unknown_field_parse(tag as u64, std::ptr::null_mut(), after_tag, ctx);
                        if ptr.is_null() {
                            return std::ptr::null();
                        }
                    }
                }
            }
            ptr
        }

        static SAMPLE_VTBL: MessageVtable = MessageVtable([
            crate::Ptr(msg_scalar_deleting_dtor as *const ()),
            crate::Ptr(msg_get_type_name as *const ()),
            crate::Ptr(msg_new as *const ()),
            crate::Ptr(msg_ret_null as *const ()),
            crate::Ptr(sample_clear as *const ()),
            crate::Ptr(msg_is_initialized as *const ()),
            crate::Ptr(msg_init_error_string as *const ()),
            crate::Ptr(msg_check_type_and_merge as *const ()),
            crate::Ptr(sample_byte_size_long as *const ()),
            crate::Ptr(sample_get_cached_size as *const ()),
            crate::Ptr(sample_internal_parse as *const ()),
            crate::Ptr(sample_internal_serialize as *const ()),
            crate::Ptr(msg_internal_get_table as *const ()),
            crate::Ptr(msg_check_type_and_merge as *const ()),
            crate::Ptr(msg_check_type_and_merge as *const ()),
            crate::Ptr(msg_discard_unknown_fields as *const ()),
            crate::Ptr(msg_space_used_long as *const ()),
            crate::Ptr(msg_set_cached_size as *const ()),
            crate::Ptr(msg_get_metadata as *const ()),
            crate::Ptr(msg_noop as *const ()),
            crate::Ptr(msg_noop as *const ()),
            crate::Ptr(msg_noop as *const ()),
            crate::Ptr(msg_noop as *const ()),
            crate::Ptr(msg_noop as *const ()),
            crate::Ptr(msg_noop as *const ()),
        ]);

        unsafe {
            let mut src_msg = SampleMsg {
                vtbl: &SAMPLE_VTBL.0,
                internal_metadata: 0,
                id: 1337,
                label: make_msvc_string(b"protobuf_native_roundtrip"),
                cached_size: std::cell::Cell::new(0),
            };
            let mut wire_str = make_msvc_string(b"");
            assert_eq!(
                message_lite_serialize_to_string(
                    &src_msg as *const SampleMsg as *const u8,
                    &mut wire_str,
                ),
                1
            );
            let wire_bytes = msvc_string_to_vec(&wire_str);
            assert_eq!(wire_bytes.len(), sample_byte_size_long(&src_msg as *const SampleMsg as *const u8));

            // SerializeToArray: too small fails, exact size succeeds
            let mut arr = vec![0u8; wire_bytes.len()];
            assert_eq!(
                message_lite_serialize_to_array(
                    &src_msg as *const SampleMsg as *const u8,
                    arr.as_mut_ptr(),
                    (wire_bytes.len() - 1) as i32,
                ),
                0
            );
            assert_eq!(
                message_lite_serialize_to_array(
                    &src_msg as *const SampleMsg as *const u8,
                    arr.as_mut_ptr(),
                    arr.len() as i32,
                ),
                1
            );
            assert_eq!(arr, wire_bytes);

            // SerializeWithCachedSizesToArray
            let mut cached_arr = vec![0u8; wire_bytes.len()];
            let end_p = message_lite_serialize_with_cached_sizes_to_array(
                &src_msg as *const SampleMsg as *const u8,
                cached_arr.as_mut_ptr(),
            );
            assert_eq!(end_p as usize - cached_arr.as_ptr() as usize, wire_bytes.len());
            assert_eq!(cached_arr, wire_bytes);

            // ParseFromString into a fresh SampleMsg
            let mut dst_msg = SampleMsg {
                vtbl: &SAMPLE_VTBL.0,
                internal_metadata: 0,
                id: 99,
                label: make_msvc_string(b"stale"),
                cached_size: std::cell::Cell::new(0),
            };
            assert_eq!(
                message_lite_parse_from_string(
                    &mut dst_msg as *mut SampleMsg as *mut u8,
                    &wire_str,
                ),
                1
            );
            assert_eq!(dst_msg.id, 1337);
            assert_eq!(msvc_string_to_vec(&dst_msg.label), b"protobuf_native_roundtrip");

            // ParseFromArray with corrupted truncated payload must fail cleanly
            let bad_wire = [0x12u8, 0x20, b'a', b'b'];
            assert_eq!(
                message_lite_parse_from_array(
                    &mut dst_msg as *mut SampleMsg as *mut u8,
                    bad_wire.as_ptr(),
                    bad_wire.len() as i32,
                ),
                0
            );

            // Synthetic DummyMessage parses valid unknown fields and rejects invalid wire type 7
            let mut dummy = [0usize; 2];
            msg_ctor_default(dummy.as_mut_ptr());
            assert_eq!(
                message_lite_parse_from_array(
                    dummy.as_mut_ptr() as *mut u8,
                    wire_bytes.as_ptr(),
                    wire_bytes.len() as i32,
                ),
                1
            );
            let invalid_wt = [0x0fu8]; // field 1, wire type 7 (invalid)
            assert_eq!(
                message_lite_parse_from_array(
                    dummy.as_mut_ptr() as *mut u8,
                    invalid_wt.as_ptr(),
                    invalid_wt.len() as i32,
                ),
                0
            );

            free_msvc_string_buf(&mut src_msg.label);
            free_msvc_string_buf(&mut dst_msg.label);
            free_msvc_string_buf(&mut wire_str);
        }
    }

    #[test]
    fn arena_alloc_destructor_and_cleanup() {
        static DTOR_CALLS: AtomicU32 = AtomicU32::new(0);
        static LAST_ORDER: AtomicU32 = AtomicU32::new(0);

        unsafe extern "win64" fn test_obj_dtor(obj: *mut u8) {
            let tag = std::ptr::read_unaligned(obj as *const u32);
            DTOR_CALLS.fetch_add(1, Ordering::SeqCst);
            LAST_ORDER.store(tag, Ordering::SeqCst);
        }

        unsafe {
            DTOR_CALLS.store(0, Ordering::SeqCst);
            LAST_ORDER.store(0, Ordering::SeqCst);

            let mut arena_mem = [0usize; 4];
            let arena = arena_mem.as_mut_ptr() as *mut u8;
            arena_ctor(arena);
            assert_eq!(arena_space_allocated(arena), 0);

            let p1 = arena_allocate_internal(
                arena,
                32,
                8,
                test_obj_dtor as *const () as usize,
                std::ptr::null(),
            );
            assert!(!p1.is_null());
            std::ptr::write_unaligned(p1 as *mut u32, 111);

            let p2 = arena_allocate_internal(
                arena,
                64,
                64, // over-aligned allocation path
                test_obj_dtor as *const () as usize,
                std::ptr::null(),
            );
            assert!(!p2.is_null());
            assert_eq!((p2 as usize) % 64, 0);
            std::ptr::write_unaligned(p2 as *mut u32, 222);

            // ArenaStringPtr backed by the arena (heap-allocated >15-byte string)
            let mut asp: usize = empty_string_ptr() as usize;
            let mut long_src = make_msvc_string(b"arena_owned_heap_string_payload");
            arena_string_ptr_set_copy(&mut asp, 0, &long_src, arena);
            assert_eq!(
                msvc_string_to_vec(arena_string_ptr_get(&asp)),
                b"arena_owned_heap_string_payload"
            );

            assert!(arena_space_allocated(arena) >= 32 + 64 + std::mem::size_of::<MsvcString>());

            // Destroy arena -> runs cleanups in LIFO order (p2 before p1, so last dtor is p1=111) and frees all allocs
            arena_dtor(arena);
            assert_eq!(DTOR_CALLS.load(Ordering::SeqCst), 2);
            assert_eq!(LAST_ORDER.load(Ordering::SeqCst), 111);
            assert_eq!(arena_space_allocated(arena), 0);

            // Repeated dtor/reset on already-destroyed arena is a safe no-op
            assert_eq!(arena_reset(arena), 0);
            arena_dtor(arena);

            free_msvc_string_buf(&mut long_src);
        }
    }
}
