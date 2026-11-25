//! Heap: one allocator shared by `HeapAlloc` (all heaps) and the ucrt
//! `malloc` family, because on Windows ucrt allocates from the process heap
//! and code legitimately mixes `malloc` with `HeapFree(GetProcessHeap())`.
//!
//! Each block has a 16-byte header in front of the user pointer storing the
//! requested size (so `HeapSize`/`_msize` are exact) and a magic value used to
//! detect foreign pointers.

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::c_void;
use std::sync::{Mutex, OnceLock};

const MAGIC: u32 = 0x4157_3248; // "AW2H"
const HDR: usize = 16;

#[repr(C)]
struct Header {
    size: usize,
    magic: u32,
    heap: u32,
}

pub const HEAP_NO_SERIALIZE: u32 = 0x1;
pub const HEAP_GENERATE_EXCEPTIONS: u32 = 0x4;
pub const HEAP_ZERO_MEMORY: u32 = 0x8;
pub const HEAP_REALLOC_IN_PLACE_ONLY: u32 = 0x10;

const PROCESS_HEAP_ID: u32 = 0x0001_0000;

/// Fake HANDLE values for heaps: process heap is 0x10000, created heaps
/// count up. They are not entries in the handle table (like Windows, where
/// heap handles are the heap base address).
pub fn process_heap() -> *mut c_void {
    PROCESS_HEAP_ID as usize as *mut c_void
}

static NEXT_HEAP: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0x0002_0000);

/// Tracks live custom heaps (`heap_id -> set of live user pointers`).
/// The process heap stays lock-free on the hot path; custom heaps created via
/// `HeapCreate` are tracked so `HeapDestroy` frees every outstanding block and
/// `HeapWalk` enumerates live allocations faithfully.
fn custom_heaps() -> &'static Mutex<BTreeMap<u32, BTreeSet<usize>>> {
    static M: OnceLock<Mutex<BTreeMap<u32, BTreeSet<usize>>>> = OnceLock::new();
    M.get_or_init(|| Mutex::new(BTreeMap::new()))
}

pub fn create_heap() -> *mut c_void {
    let id = NEXT_HEAP.fetch_add(0x1_0000, std::sync::atomic::Ordering::Relaxed) as u32;
    crate::thread::defer_lock(custom_heaps()).insert(id, BTreeSet::new());
    id as usize as *mut c_void
}

pub fn all_heaps() -> Vec<*mut c_void> {
    let mut v = vec![process_heap()];
    for &id in crate::thread::defer_lock(custom_heaps()).keys() {
        v.push(id as usize as *mut c_void);
    }
    v
}

pub unsafe fn alloc_in(heap_handle: *mut c_void, size: usize, zero: bool) -> *mut c_void {
    let _defer = crate::thread::TerminateDeferGuard::enter();
    let heap_id = heap_handle as usize as u32;
    let tracked = heap_id != 0 && heap_id != PROCESS_HEAP_ID;
    let p = alloc(size, zero);
    if !p.is_null() && tracked {
        let h = (p as *mut u8).sub(HDR) as *mut Header;
        (*h).heap = heap_id;
        let mut map = crate::thread::defer_lock(custom_heaps());
        map.entry(heap_id).or_default().insert(p as usize);
    }
    p
}

pub unsafe fn destroy_heap(heap_handle: *mut c_void) -> bool {
    let _defer = crate::thread::TerminateDeferGuard::enter();
    let heap_id = heap_handle as usize as u32;
    if heap_id == 0 || heap_id == PROCESS_HEAP_ID {
        return false;
    }
    let blocks = crate::thread::defer_lock(custom_heaps()).remove(&heap_id);
    let Some(blocks) = blocks else {
        return false;
    };
    for ptr in blocks {
        let p = ptr as *mut c_void;
        if let Some(h) = header(p) {
            (*h).magic = 0xDEAD_BEEF;
            (*h).heap = 0;
            if !crate::shadow::heap_on() {
                libc::free(h as *mut c_void);
            }
        }
    }
    true
}

/// Return the next `(user_ptr, block_size)` in `heap_handle` after `prev_ptr`
/// (or the first block when `prev_ptr` is null).
pub unsafe fn walk_next(heap_handle: *mut c_void, prev_ptr: *const c_void) -> Option<(*mut c_void, usize)> {
    let heap_id = heap_handle as usize as u32;
    let map = crate::thread::defer_lock(custom_heaps());
    let set = map.get(&heap_id)?;
    let next_addr = if prev_ptr.is_null() {
        set.iter().next().copied()
    } else {
        set.range((std::ops::Bound::Excluded(prev_ptr as usize), std::ops::Bound::Unbounded))
            .next()
            .copied()
    }?;
    let p = next_addr as *mut c_void;
    let sz = size(p).unwrap_or(0);
    Some((p, sz))
}

pub unsafe fn alloc(size: usize, zero: bool) -> *mut c_void {
    let _defer = crate::thread::TerminateDeferGuard::enter();
    let total = match size.checked_add(HDR) {
        Some(t) => t,
        None => return std::ptr::null_mut(),
    };
    let base = if crate::shadow::heap_on() {
        crate::shadow::bump(total)
    } else if zero {
        libc::calloc(1, total) as *mut u8
    } else {
        libc::malloc(total) as *mut u8
    };
    if base.is_null() {
        return std::ptr::null_mut();
    }
    let h = base as *mut Header;
    (*h).size = size;
    (*h).magic = MAGIC;
    (*h).heap = 0;
    base.add(HDR) as *mut c_void
}

unsafe fn header(p: *mut c_void) -> Option<*mut Header> {
    if p.is_null() {
        return None;
    }
    let h = (p as *mut u8).sub(HDR) as *mut Header;
    ((*h).magic == MAGIC).then_some(h)
}

pub unsafe fn is_ours(p: *mut c_void) -> bool {
    header(p).is_some()
}

pub unsafe fn free(p: *mut c_void) -> bool {
    if p.is_null() {
        return true;
    }
    let _defer = crate::thread::TerminateDeferGuard::enter();
    match header(p) {
        Some(h) => {
            let hid = (*h).heap;
            if hid != 0 {
                if let Some(set) = crate::thread::defer_lock(custom_heaps()).get_mut(&hid) {
                    set.remove(&(p as usize));
                }
            }
            (*h).magic = 0xDEAD_BEEF;
            if !crate::shadow::heap_on() {
                libc::free(h as *mut c_void);
            }
            true
        }
        None => {
            log::error!(
                "heap free of foreign/corrupt pointer {p:p} (caller should crash on Windows too); leaking"
            );
            false
        }
    }
}

pub unsafe fn size(p: *mut c_void) -> Option<usize> {
    header(p).map(|h| (*h).size)
}

pub unsafe fn realloc(p: *mut c_void, new_size: usize, zero: bool, in_place_only: bool) -> *mut c_void {
    if p.is_null() {
        return alloc(new_size, zero);
    }
    let _defer = crate::thread::TerminateDeferGuard::enter();
    let Some(h) = header(p) else {
        log::error!("heap realloc of foreign pointer {p:p}");
        return std::ptr::null_mut();
    };
    let old = (*h).size;
    let old_hid = (*h).heap;
    if in_place_only {
        // We can only shrink (or keep) in place safely.
        if new_size <= old {
            (*h).size = new_size;
            return p;
        }
        return std::ptr::null_mut();
    }
    if crate::shadow::heap_on() {
        // Bump arena memory is zero-filled, so `zero` needs no extra work.
        let np = alloc(new_size, false);
        if !np.is_null() {
            std::ptr::copy_nonoverlapping(p as *const u8, np as *mut u8, old.min(new_size));
            (*h).magic = 0xDEAD_BEEF;
            if old_hid != 0 {
                let nh = (np as *mut u8).sub(HDR) as *mut Header;
                (*nh).heap = old_hid;
                if let Some(set) = crate::thread::defer_lock(custom_heaps()).get_mut(&old_hid) {
                    set.remove(&(p as usize));
                    set.insert(np as usize);
                }
            }
        }
        return np;
    }
    let nb = libc::realloc(h as *mut c_void, new_size + HDR) as *mut u8;
    if nb.is_null() {
        return std::ptr::null_mut();
    }
    let nh = nb as *mut Header;
    (*nh).size = new_size;
    let np = nb.add(HDR);
    if old_hid != 0 && (np as usize) != (p as usize) {
        if let Some(set) = crate::thread::defer_lock(custom_heaps()).get_mut(&old_hid) {
            set.remove(&(p as usize));
            set.insert(np as usize);
        }
    }
    if zero && new_size > old {
        std::ptr::write_bytes(np.add(old), 0, new_size - old);
    }
    np as *mut c_void
}

// ---- _aligned_malloc family (MSVC layout: original pointer stored just
// below the aligned block, so `_aligned_free` only needs the user pointer).

pub unsafe fn aligned_alloc(size: usize, align: usize, offset: usize) -> *mut c_void {
    let align = align.max(std::mem::size_of::<usize>());
    if !align.is_power_of_two() {
        return std::ptr::null_mut();
    }
    let total = size + align + std::mem::size_of::<usize>() * 2;
    let raw = alloc(total, false) as usize;
    if raw == 0 {
        return std::ptr::null_mut();
    }
    let start = raw + std::mem::size_of::<usize>() * 2 + offset;
    let aligned = ((start + align - 1) & !(align - 1)) - offset;
    *((aligned - std::mem::size_of::<usize>()) as *mut usize) = raw;
    *((aligned - 2 * std::mem::size_of::<usize>()) as *mut usize) = size;
    aligned as *mut c_void
}

pub unsafe fn aligned_free(p: *mut c_void) {
    if p.is_null() {
        return;
    }
    let raw = *((p as usize - std::mem::size_of::<usize>()) as *const usize);
    free(raw as *mut c_void);
}

pub unsafe fn aligned_size(p: *mut c_void) -> usize {
    *((p as usize - 2 * std::mem::size_of::<usize>()) as *const usize)
}

pub unsafe fn aligned_realloc(p: *mut c_void, size: usize, align: usize, offset: usize) -> *mut c_void {
    if p.is_null() {
        return aligned_alloc(size, align, offset);
    }
    if size == 0 {
        aligned_free(p);
        return std::ptr::null_mut();
    }
    let n = aligned_alloc(size, align, offset);
    if n.is_null() {
        return n;
    }
    let old = aligned_size(p);
    std::ptr::copy_nonoverlapping(p as *const u8, n as *mut u8, old.min(size));
    aligned_free(p);
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip() {
        unsafe {
            let p = alloc(100, true);
            assert_eq!(size(p), Some(100));
            assert_eq!(p as usize % 16, 0);
            let p = realloc(p, 1000, true, false);
            assert_eq!(*(p as *mut u8).add(999), 0);
            assert!(free(p));
            let a = aligned_alloc(77, 256, 0);
            assert_eq!(a as usize % 256, 0);
            assert_eq!(aligned_size(a), 77);
            let a = aligned_realloc(a, 300, 256, 0);
            assert_eq!(a as usize % 256, 0);
            aligned_free(a);
        }
    }
}
