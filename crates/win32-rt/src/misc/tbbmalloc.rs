//! Native `tbbmalloc.dll` / `tbb.dll` / `tbb12.dll` (Intel oneTBB scalable allocator) bridge.

use std::ffi::c_void;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

/// Inline header placed immediately before every pointer returned by
/// `scalable_aligned_malloc` / `scalable_aligned_realloc`.
///
/// Layout invariant: `magic` sits in the last 8 bytes (`ptr - 8`), matching
/// the probe offset of `crate::heap::Header` so checking `magic` at `ptr - 8`
/// is always in-bounds for both aligned and regular `crate::heap` pointers.
#[repr(C)]
#[derive(Clone, Copy)]
struct AlignedHeader {
    base_ptr: usize,
    size: usize,
    align: usize,
    magic: u64,
}

const ALIGNED_HDR_SIZE: usize = std::mem::size_of::<AlignedHeader>();
const ALIGNED_MAGIC: u64 = 0x5442_4241_4C49_474E; // "TBBALIGN"
const ALIGNED_POISON: u64 = 0xDEAD_BEEF_DEAD_BEEF;

/// Returns the [`AlignedHeader`] if `ptr` was allocated by `scalable_aligned_malloc`.
unsafe fn read_aligned_header(ptr: usize) -> Option<AlignedHeader> {
    if ptr < ALIGNED_HDR_SIZE {
        return None;
    }
    let magic = ((ptr - std::mem::size_of::<u64>()) as *const u64).read_unaligned();
    if magic != ALIGNED_MAGIC {
        return None;
    }
    let hdr = ((ptr - ALIGNED_HDR_SIZE) as *const AlignedHeader).read_unaligned();
    (hdr.magic == ALIGNED_MAGIC).then_some(hdr)
}

pub extern "win64" fn scalable_malloc(size: usize) -> usize {
    unsafe { crate::heap::alloc(size.max(16), false) as usize }
}

pub extern "win64" fn scalable_calloc(num: usize, size: usize) -> usize {
    let Some(bytes) = num.checked_mul(size) else {
        return 0;
    };
    unsafe { crate::heap::alloc(bytes.max(16), true) as usize }
}

pub extern "win64" fn scalable_realloc(ptr: usize, size: usize) -> usize {
    if ptr == 0 {
        return scalable_malloc(size);
    }
    if let Some(hdr) = unsafe { read_aligned_header(ptr) } {
        return scalable_aligned_realloc(ptr, size, hdr.align);
    }
    if size == 0 {
        scalable_free(ptr);
        return 0;
    }
    unsafe { crate::heap::realloc(ptr as *mut c_void, size.max(16), false, false) as usize }
}

pub extern "win64" fn scalable_free(ptr: usize) {
    if ptr == 0 {
        return;
    }
    unsafe {
        if let Some(hdr) = read_aligned_header(ptr) {
            ((ptr - std::mem::size_of::<u64>()) as *mut u64).write_unaligned(ALIGNED_POISON);
            libc::free(hdr.base_ptr as *mut c_void);
        } else {
            crate::heap::free(ptr as *mut c_void);
        }
    }
}

pub extern "win64" fn scalable_aligned_malloc(size: usize, alignment: usize) -> usize {
    if !alignment.is_power_of_two() {
        return 0;
    }
    let eff_align = alignment.max(std::mem::size_of::<*mut c_void>());
    // `prefix` is a multiple of `eff_align` (since both are powers of two) and >= ALIGNED_HDR_SIZE.
    let prefix = ALIGNED_HDR_SIZE.max(eff_align);
    let Some(total) = prefix.checked_add(size.max(1)) else {
        return 0;
    };
    let mut raw: *mut c_void = std::ptr::null_mut();
    let rc = unsafe { libc::posix_memalign(&mut raw, eff_align, total) };
    if rc != 0 || raw.is_null() {
        return 0;
    }
    let user_ptr = (raw as usize) + prefix;
    let hdr = AlignedHeader {
        base_ptr: raw as usize,
        size,
        align: eff_align,
        magic: ALIGNED_MAGIC,
    };
    unsafe {
        ((user_ptr - ALIGNED_HDR_SIZE) as *mut AlignedHeader).write_unaligned(hdr);
    }
    user_ptr
}

pub extern "win64" fn scalable_aligned_realloc(
    ptr: usize,
    size: usize,
    alignment: usize,
) -> usize {
    if !alignment.is_power_of_two() {
        return 0;
    }
    if ptr == 0 {
        return scalable_aligned_malloc(size, alignment);
    }
    if size == 0 {
        scalable_aligned_free(ptr);
        return 0;
    }
    let old_size = unsafe {
        if let Some(hdr) = read_aligned_header(ptr) {
            hdr.size
        } else if let Some(sz) = crate::heap::size(ptr as *mut c_void) {
            sz
        } else {
            return 0;
        }
    };
    let new_ptr = scalable_aligned_malloc(size, alignment);
    if new_ptr == 0 {
        return 0;
    }
    let copy_len = old_size.min(size);
    if copy_len > 0 {
        unsafe {
            std::ptr::copy_nonoverlapping(ptr as *const u8, new_ptr as *mut u8, copy_len);
        }
    }
    scalable_aligned_free(ptr);
    new_ptr
}

pub extern "win64" fn scalable_aligned_free(ptr: usize) {
    scalable_free(ptr);
}

pub extern "win64" fn scalable_msize(ptr: usize) -> usize {
    if ptr == 0 {
        return 0;
    }
    unsafe {
        if let Some(hdr) = read_aligned_header(ptr) {
            hdr.size
        } else {
            crate::heap::size(ptr as *mut c_void).unwrap_or(0)
        }
    }
}

pub extern "win64" fn TBB_runtime_interface_version() -> i32 {
    12130 // oneTBB 2021.13
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tbb_scalable_alloc_roundtrip() {
        let ptr = scalable_malloc(256);
        assert_ne!(ptr, 0);
        assert!(scalable_msize(ptr) >= 256);
        scalable_free(ptr);
    }

    #[test]
    fn tbb_aligned_alloc_honors_alignments_and_exact_msize() {
        // Non-power-of-two alignments must fail cleanly.
        assert_eq!(scalable_aligned_malloc(64, 0), 0);
        assert_eq!(scalable_aligned_malloc(64, 3), 0);
        assert_eq!(scalable_aligned_malloc(64, 24), 0);

        for align in [1usize, 2, 4, 8, 16, 32, 64, 256, 4096] {
            let size = 77;
            let ptr = scalable_aligned_malloc(size, align);
            assert_ne!(ptr, 0, "align={align}");
            assert_eq!(ptr % align, 0, "ptr={ptr:#x} not aligned to {align}");
            assert_eq!(scalable_msize(ptr), size, "exact msize for align={align}");
            unsafe {
                std::ptr::write_bytes(ptr as *mut u8, 0x5A, size);
            }
            scalable_aligned_free(ptr);
        }

        // Zero-size aligned allocation still returns an aligned non-null pointer with msize == 0.
        let z = scalable_aligned_malloc(0, 64);
        assert_ne!(z, 0);
        assert_eq!(z % 64, 0);
        assert_eq!(scalable_msize(z), 0);
        scalable_aligned_free(z);
    }

    #[test]
    fn tbb_aligned_realloc_preserves_bytes_and_updates_alignment() {
        let p1 = scalable_aligned_malloc(37, 16);
        assert_ne!(p1, 0);
        assert_eq!(p1 % 16, 0);
        assert_eq!(scalable_msize(p1), 37);
        unsafe {
            for i in 0..37usize {
                (p1 as *mut u8).add(i).write_unaligned((i as u8).wrapping_mul(7).wrapping_add(3));
            }
        }

        // Grow and raise alignment to 256 bytes.
        let p2 = scalable_aligned_realloc(p1, 512, 256);
        assert_ne!(p2, 0);
        assert_eq!(p2 % 256, 0);
        assert_eq!(scalable_msize(p2), 512);
        unsafe {
            for i in 0..37usize {
                let got = (p2 as *const u8).add(i).read_unaligned();
                assert_eq!(got, (i as u8).wrapping_mul(7).wrapping_add(3));
            }
        }

        // Shrink and lower alignment (< sizeof(void*)).
        let p3 = scalable_aligned_realloc(p2, 13, 4);
        assert_ne!(p3, 0);
        assert_eq!(p3 % 4, 0);
        assert_eq!(scalable_msize(p3), 13);
        unsafe {
            for i in 0..13usize {
                let got = (p3 as *const u8).add(i).read_unaligned();
                assert_eq!(got, (i as u8).wrapping_mul(7).wrapping_add(3));
            }
        }

        // Invalid alignment leaves existing block untouched.
        assert_eq!(scalable_aligned_realloc(p3, 64, 3), 0);
        assert_eq!(scalable_msize(p3), 13);

        // Size 0 frees and returns NULL.
        assert_eq!(scalable_aligned_realloc(p3, 0, 16), 0);

        // NULL pointer allocates.
        let p4 = scalable_aligned_realloc(0, 48, 64);
        assert_ne!(p4, 0);
        assert_eq!(p4 % 64, 0);
        assert_eq!(scalable_msize(p4), 48);
        scalable_aligned_free(p4);
    }
}
