//! ABI-compatible managed heap (`MonoObject`/`MonoString`/`MonoArray`/`GCHandle`)
//! with O(1) header-tagged slab lookup and frame-scoped value-type arena reclamation.

use super::super::metadata::SigType;
use super::value::CilVal;
use std::collections::HashMap;

/// `MonoObject` header (16 bytes): matches 64-bit Mono ABI (`vtable` + `synchronisation`).
pub const MONO_OBJECT_HEADER_SIZE: usize = 16;
/// `MonoString`: 16-byte header + 4-byte `length` + inline UTF-16 `chars` (+ NUL).
pub const MONO_STRING_LEN_OFFSET: usize = 16;
pub const MONO_STRING_CHARS_OFFSET: usize = 20;
/// `MonoArray`: 16-byte header + 8-byte `bounds` + 8-byte `max_length` + inline `vector`.
pub const MONO_ARRAY_LEN_OFFSET: usize = 24;
pub const MONO_ARRAY_DATA_OFFSET: usize = 32;

/// Upper 32 bits stored in `MonoObject.synchronisation` (offset +8) to identify slab slots
/// in O(1) without touching a `HashMap` on every array/string/object access.
const HEAP_SLOT_MAGIC_SHIFT: u32 = 32;
const HEAP_SLOT_MAGIC_HI: u64 = 0x4D4F_4E4F;
const HEAP_SLOT_MAGIC_PREFIX: u64 = HEAP_SLOT_MAGIC_HI << HEAP_SLOT_MAGIC_SHIFT;

#[derive(Clone, Debug)]
pub enum HeapObjKind {
    Object {
        class_id: usize,
    },
    String,
    Array {
        elem_class_id: usize,
        elem_size: usize,
        length: usize,
    },
    Boxed {
        class_id: usize,
        sig: SigType,
    },
    Delegate {
        class_id: usize,
        method_id: usize,
        target_obj: usize,
    },
    TypeObj {
        represented_class_id: usize,
    },
    StringBuilder {
        buf: String,
    },
    Exception {
        class_id: usize,
        message: String,
    },
    RawStruct {
        class_id: usize,
    },
    List {
        items: Vec<CilVal>,
    },
    Dictionary {
        entries: Vec<(CilVal, CilVal)>,
        index: HashMap<u64, Vec<usize>>,
    },
}

pub struct GcHandleEntry {
    pub target: usize,
    pub _pinned: bool,
    pub _weak: bool,
}

struct HeapEntry {
    addr: usize,
    kind: HeapObjKind,
}

pub struct ManagedHeap {
    /// Pinned 8-byte-aligned backing blocks so object pointers remain valid for VM lifetime.
    blocks: Vec<Box<[u64]>>,
    /// Inclusive `[min_addr, max_addr]` span of allocated blocks to reject non-heap integers in O(1).
    min_addr: usize,
    max_addr: usize,
    /// Direct slab of managed object descriptors indexed by slot ID from `MonoObject+8`.
    entries: Vec<HeapEntry>,
    /// Address -> slot index map used for untrusted C-boundary pointer validation & unheaded structs.
    addr_to_slot: HashMap<usize, u32>,
    pub gc_handles: HashMap<u32, GcHandleEntry>,
    next_gc_handle: u32,
}

impl ManagedHeap {
    pub(crate) fn new() -> Self {
        Self {
            blocks: Vec::new(),
            min_addr: usize::MAX,
            max_addr: 0,
            // Slot 0 is reserved as sentinel so valid slot IDs are >= 1.
            entries: vec![HeapEntry {
                addr: 0,
                kind: HeapObjKind::String,
            }],
            addr_to_slot: HashMap::new(),
            gc_handles: HashMap::new(),
            next_gc_handle: 1,
        }
    }

    pub(crate) fn alloc_raw(&mut self, byte_len: usize) -> *mut u8 {
        let words = byte_len.div_ceil(8).max(2);
        let mut blk = vec![0u64; words].into_boxed_slice();
        let ptr = blk.as_mut_ptr() as *mut u8;
        let addr = ptr as usize;
        if addr < self.min_addr {
            self.min_addr = addr;
        }
        if addr > self.max_addr {
            self.max_addr = addr;
        }
        self.blocks.push(blk);
        ptr
    }

    #[inline]
    fn register_headed(&mut self, ptr: *mut u8, vtable: usize, kind: HeapObjKind) -> usize {
        let slot = self.entries.len() as u32;
        unsafe {
            std::ptr::write_unaligned(ptr as *mut usize, vtable);
            std::ptr::write_unaligned(
                ptr.add(8) as *mut u64,
                HEAP_SLOT_MAGIC_PREFIX | (slot as u64),
            );
        }
        let addr = ptr as usize;
        self.entries.push(HeapEntry { addr, kind });
        self.addr_to_slot.insert(addr, slot);
        addr
    }

    /// O(1) direct slab lookup for managed object references (zero HashMap lookup on headed objects).
    #[inline]
    pub fn get_managed(&self, addr: usize) -> Option<&HeapObjKind> {
        if addr < self.min_addr || addr > self.max_addr || (addr & 7) != 0 {
            return None;
        }
        let tag = unsafe { std::ptr::read_unaligned((addr + 8) as *const u64) };
        if (tag >> HEAP_SLOT_MAGIC_SHIFT) == HEAP_SLOT_MAGIC_HI {
            let slot = (tag & 0xFFFF_FFFF) as usize;
            if let Some(entry) = self.entries.get(slot) {
                if entry.addr == addr {
                    return Some(&entry.kind);
                }
            }
        }
        let &slot = self.addr_to_slot.get(&addr)?;
        self.entries.get(slot as usize).map(|e| &e.kind)
    }

    /// O(1) mutable slab lookup for managed object references (`StringBuilder`, `List`, `Dictionary`).
    #[inline]
    pub fn get_managed_mut(&mut self, addr: usize) -> Option<&mut HeapObjKind> {
        if addr < self.min_addr || addr > self.max_addr || (addr & 7) != 0 {
            return None;
        }
        let tag = unsafe { std::ptr::read_unaligned((addr + 8) as *const u64) };
        if (tag >> HEAP_SLOT_MAGIC_SHIFT) == HEAP_SLOT_MAGIC_HI {
            let slot = (tag & 0xFFFF_FFFF) as usize;
            if self.entries.get(slot).is_some_and(|e| e.addr == addr) {
                return Some(&mut self.entries[slot].kind);
            }
        }
        let &slot = self.addr_to_slot.get(&addr)?;
        self.entries.get_mut(slot as usize).map(|e| &mut e.kind)
    }

    /// Safe lookup for untrusted native C pointers (never dereferences `addr`).
    #[inline]
    pub fn get_untrusted(&self, addr: usize) -> Option<&HeapObjKind> {
        if addr == 0 {
            return None;
        }
        let &slot = self.addr_to_slot.get(&addr)?;
        self.entries.get(slot as usize).map(|e| &e.kind)
    }

    /// Safe membership check for untrusted native C pointers (never dereferences `addr`).
    #[inline]
    pub fn contains_untrusted(&self, addr: usize) -> bool {
        addr != 0 && self.addr_to_slot.contains_key(&addr)
    }

    /// Fast O(1) `(length, elem_size)` query for managed arrays.
    #[inline]
    pub fn array_info(&self, arr: usize) -> Option<(usize, usize)> {
        match self.get_managed(arr)? {
            HeapObjKind::Array {
                length, elem_size, ..
            } => Some((*length, *elem_size)),
            _ => None,
        }
    }

    pub fn alloc_object(&mut self, vtable: usize, class_id: usize, payload_size: usize) -> usize {
        let total = MONO_OBJECT_HEADER_SIZE + payload_size;
        let ptr = self.alloc_raw(total);
        self.register_headed(ptr, vtable, HeapObjKind::Object { class_id })
    }

    pub fn alloc_string_utf16(&mut self, string_vtable: usize, utf16: &[u16]) -> usize {
        let total = MONO_STRING_CHARS_OFFSET + (utf16.len() + 1) * 2;
        let ptr = self.alloc_raw(total);
        unsafe {
            std::ptr::write_unaligned(
                ptr.add(MONO_STRING_LEN_OFFSET) as *mut i32,
                utf16.len() as i32,
            );
            let dst = ptr.add(MONO_STRING_CHARS_OFFSET);
            if !utf16.is_empty() {
                std::ptr::copy_nonoverlapping(utf16.as_ptr() as *const u8, dst, utf16.len() * 2);
            }
            std::ptr::write_unaligned(dst.add(utf16.len() * 2) as *mut u16, 0u16);
        }
        self.register_headed(ptr, string_vtable, HeapObjKind::String)
    }

    pub fn alloc_string(&mut self, string_vtable: usize, s: &str) -> usize {
        let utf16: Vec<u16> = s.encode_utf16().collect();
        self.alloc_string_utf16(string_vtable, &utf16)
    }

    #[inline]
    pub fn is_string(&self, obj: usize) -> bool {
        matches!(self.get_managed(obj), Some(HeapObjKind::String))
    }

    /// Read the UTF-16 code-unit length of a managed `MonoString` without allocating.
    #[inline]
    pub fn string_len(&self, obj: usize) -> Option<usize> {
        if !self.is_string(obj) {
            return None;
        }
        let len = unsafe {
            std::ptr::read_unaligned((obj + MONO_STRING_LEN_OFFSET) as *const i32)
        };
        Some(len.max(0) as usize)
    }

    /// Read a single UTF-16 code unit from a managed `MonoString` in O(1) without allocating.
    #[inline]
    pub fn string_char_at(&self, obj: usize, idx: usize) -> Option<u16> {
        let len = self.string_len(obj)?;
        if idx >= len {
            return None;
        }
        let ch = unsafe {
            std::ptr::read_unaligned((obj + MONO_STRING_CHARS_OFFSET + idx * 2) as *const u16)
        };
        Some(ch)
    }

    /// Compare two managed strings by UTF-16 contents without heap allocation.
    pub fn strings_equal(&self, a: usize, b: usize) -> bool {
        if a == b {
            return true;
        }
        let (Some(la), Some(lb)) = (self.string_len(a), self.string_len(b)) else {
            return false;
        };
        if la != lb {
            return false;
        }
        if la == 0 {
            return true;
        }
        unsafe {
            let sa = std::slice::from_raw_parts((a + MONO_STRING_CHARS_OFFSET) as *const u8, la * 2);
            let sb = std::slice::from_raw_parts((b + MONO_STRING_CHARS_OFFSET) as *const u8, lb * 2);
            sa == sb
        }
    }

    /// Hash a managed string's UTF-16 bytes (DJB2) without allocating.
    pub fn string_hash_u64(&self, obj: usize) -> Option<u64> {
        let len = self.string_len(obj)?;
        let bytes =
            unsafe { std::slice::from_raw_parts((obj + MONO_STRING_CHARS_OFFSET) as *const u8, len * 2) };
        let mut h = 5381u64;
        for &b in bytes {
            h = h.wrapping_mul(33).wrapping_add(b as u64);
        }
        Some(h)
    }

    pub fn read_string(&self, obj: usize) -> Option<String> {
        let n = self.string_len(obj)?;
        if n == 0 {
            return Some(String::new());
        }
        let mut u16s = vec![0u16; n];
        unsafe {
            std::ptr::copy_nonoverlapping(
                (obj + MONO_STRING_CHARS_OFFSET) as *const u8,
                u16s.as_mut_ptr() as *mut u8,
                n * 2,
            );
        }
        Some(String::from_utf16_lossy(&u16s))
    }

    pub fn read_string_utf16(&self, obj: usize) -> Option<Vec<u16>> {
        let n = self.string_len(obj)?;
        if n == 0 {
            return Some(Vec::new());
        }
        let mut u16s = vec![0u16; n];
        unsafe {
            std::ptr::copy_nonoverlapping(
                (obj + MONO_STRING_CHARS_OFFSET) as *const u8,
                u16s.as_mut_ptr() as *mut u8,
                n * 2,
            );
        }
        Some(u16s)
    }

    pub fn alloc_array(
        &mut self,
        array_vtable: usize,
        elem_class_id: usize,
        elem_size: usize,
        length: usize,
    ) -> usize {
        let elem_sz = elem_size.max(1);
        let total = MONO_ARRAY_DATA_OFFSET + length.saturating_mul(elem_sz);
        let ptr = self.alloc_raw(total);
        unsafe {
            std::ptr::write_unaligned(ptr.add(MONO_ARRAY_LEN_OFFSET) as *mut usize, length);
        }
        self.register_headed(
            ptr,
            array_vtable,
            HeapObjKind::Array {
                elem_class_id,
                elem_size: elem_sz,
                length,
            },
        )
    }

    pub fn alloc_boxed(
        &mut self,
        vtable: usize,
        class_id: usize,
        sig: SigType,
        bytes: &[u8],
    ) -> usize {
        let size = bytes.len().max(1);
        let total = MONO_OBJECT_HEADER_SIZE + size;
        let ptr = self.alloc_raw(total);
        if !bytes.is_empty() {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    bytes.as_ptr(),
                    ptr.add(MONO_OBJECT_HEADER_SIZE),
                    bytes.len(),
                );
            }
        }
        self.register_headed(ptr, vtable, HeapObjKind::Boxed { class_id, sig })
    }

    pub fn alloc_raw_struct(&mut self, class_id: usize, size: usize) -> usize {
        let sz = size.max(8);
        let ptr = self.alloc_raw(sz);
        let addr = ptr as usize;
        let slot = self.entries.len() as u32;
        self.entries.push(HeapEntry {
            addr,
            kind: HeapObjKind::RawStruct { class_id },
        });
        self.addr_to_slot.insert(addr, slot);
        addr
    }

    pub fn alloc_exception(&mut self, vtable: usize, class_id: usize, message: String) -> usize {
        let ptr = self.alloc_raw(MONO_OBJECT_HEADER_SIZE + 16);
        self.register_headed(ptr, vtable, HeapObjKind::Exception { class_id, message })
    }

    pub fn alloc_delegate(
        &mut self,
        vtable: usize,
        class_id: usize,
        method_id: usize,
        target_obj: usize,
    ) -> usize {
        let ptr = self.alloc_raw(MONO_OBJECT_HEADER_SIZE);
        self.register_headed(
            ptr,
            vtable,
            HeapObjKind::Delegate {
                class_id,
                method_id,
                target_obj,
            },
        )
    }

    pub fn alloc_type_obj(&mut self, type_vtable: usize, represented_class_id: usize) -> usize {
        let ptr = self.alloc_raw(MONO_OBJECT_HEADER_SIZE);
        self.register_headed(
            ptr,
            type_vtable,
            HeapObjKind::TypeObj {
                represented_class_id,
            },
        )
    }

    pub fn alloc_string_builder(&mut self, sb_vtable: usize, init: String) -> usize {
        let ptr = self.alloc_raw(MONO_OBJECT_HEADER_SIZE);
        self.register_headed(ptr, sb_vtable, HeapObjKind::StringBuilder { buf: init })
    }

    pub fn alloc_list(&mut self, list_vtable: usize, cap: usize) -> usize {
        let ptr = self.alloc_raw(MONO_OBJECT_HEADER_SIZE);
        self.register_headed(
            ptr,
            list_vtable,
            HeapObjKind::List {
                items: Vec::with_capacity(cap.min(4096)),
            },
        )
    }

    pub fn alloc_dictionary(&mut self, dict_vtable: usize, cap: usize) -> usize {
        let ptr = self.alloc_raw(MONO_OBJECT_HEADER_SIZE);
        self.register_headed(
            ptr,
            dict_vtable,
            HeapObjKind::Dictionary {
                entries: Vec::with_capacity(cap.min(4096)),
                index: HashMap::with_capacity(cap.min(4096)),
            },
        )
    }

    pub fn gchandle_new(&mut self, target: usize, pinned: bool, weak: bool) -> u32 {
        let h = self.next_gc_handle;
        self.next_gc_handle = self.next_gc_handle.wrapping_add(1).max(1);
        self.gc_handles.insert(
            h,
            GcHandleEntry {
                target,
                _pinned: pinned,
                _weak: weak,
            },
        );
        h
    }

    pub fn gchandle_get(&self, handle: u32) -> usize {
        self.gc_handles.get(&handle).map_or(0, |e| e.target)
    }

    pub fn gchandle_free(&mut self, handle: u32) {
        self.gc_handles.remove(&handle);
    }
}

// ---------------------------------------------------------------------------
// Frame-Scoped Value-Type & `localloc` Scratch Arena
// ---------------------------------------------------------------------------

const MAX_REUSABLE_SCRATCH_BLOCKS: usize = 64;

/// Reclaimable arena for frame-scoped value types (`locals`, `ldobj`, `newobj` on structs)
/// and `localloc` buffers. Blocks are heap-pinned `Box<[u64]>` so moving them between `live`
/// and `free` never invalidates raw addresses of sibling live blocks.
pub(crate) struct FrameArena {
    live: Vec<Box<[u64]>>,
    free: Vec<Box<[u64]>>,
}

impl FrameArena {
    pub fn new() -> Self {
        Self {
            live: Vec::new(),
            free: Vec::new(),
        }
    }

    #[inline]
    pub fn mark(&self) -> usize {
        self.live.len()
    }

    #[inline]
    pub fn alloc(&mut self, byte_len: usize) -> usize {
        let words = byte_len.div_ceil(8).max(2);
        if let Some(pos) = self
            .free
            .iter()
            .position(|b| b.len() >= words && b.len() <= words.saturating_mul(2).max(8))
        {
            let mut blk = self.free.swap_remove(pos);
            blk[..words].fill(0);
            let ptr = blk.as_mut_ptr() as usize;
            self.live.push(blk);
            return ptr;
        }
        let mut blk = vec![0u64; words].into_boxed_slice();
        let ptr = blk.as_mut_ptr() as usize;
        self.live.push(blk);
        ptr
    }

    /// Reclaim all blocks allocated since `mark` back into the reuse pool.
    #[inline]
    pub fn reclaim(&mut self, mark: usize) {
        while self.live.len() > mark {
            if let Some(blk) = self.live.pop() {
                if self.free.len() < MAX_REUSABLE_SCRATCH_BLOCKS {
                    self.free.push(blk);
                }
            }
        }
    }

    /// Reclaim all blocks allocated since `mark`, EXCEPT if `keep_ptr` matches one of those
    /// blocks, adopt that block into the caller's scope (`live[mark]`) without copying.
    /// Returns `true` if `keep_ptr` was found in `live[mark..]`.
    pub fn reclaim_keeping(&mut self, mark: usize, keep_ptr: usize) -> bool {
        if let Some(rel) = self.live[mark..]
            .iter()
            .position(|b| (b.as_ptr() as usize) == keep_ptr)
        {
            let abs = mark + rel;
            self.live.swap(mark, abs);
            self.reclaim(mark + 1);
            true
        } else {
            self.reclaim(mark);
            false
        }
    }

    /// Check whether `ptr` points to any block owned by this frame arena.
    #[inline]
    pub fn contains_ptr(&self, ptr: usize) -> bool {
        self.live.iter().any(|b| (b.as_ptr() as usize) == ptr)
    }

    #[cfg(test)]
    pub fn live_count(&self) -> usize {
        self.live.len()
    }
}
