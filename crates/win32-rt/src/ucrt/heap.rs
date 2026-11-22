//! CRT heap: malloc & co. share the allocator behind HeapAlloc
//! ([`crate::heap`]), so memory can cross the CRT/Win32 boundary like on
//! Windows (ucrt's malloc is HeapAlloc on the process heap).

use super::errno::{self, EINVAL, ENOMEM};
use crate::heap;
use std::ffi::c_void;

unsafe fn alloc_retry(size: usize, zero: bool) -> *mut c_void {
    loop {
        let p = heap::alloc(size.max(1), zero);
        if !p.is_null() {
            return p;
        }
        if super::runtime::_query_new_mode() == 0 || super::runtime::_callnewh(size) == 0 {
            errno::set(ENOMEM);
            return std::ptr::null_mut();
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn malloc(size: usize) -> *mut c_void {
    alloc_retry(size, false)
}
#[inline(never)]
pub unsafe extern "win64" fn _malloc_base(size: usize) -> *mut c_void {
    alloc_retry(size, false)
}
#[inline(never)]
pub unsafe extern "win64" fn calloc(n: usize, size: usize) -> *mut c_void {
    match n.checked_mul(size) {
        Some(t) => alloc_retry(t, true),
        None => {
            errno::set(ENOMEM);
            std::ptr::null_mut()
        }
    }
}
#[inline(never)]
pub unsafe extern "win64" fn _calloc_base(n: usize, size: usize) -> *mut c_void {
    calloc(n, size)
}
#[inline(never)]
pub unsafe extern "win64" fn free(p: *mut c_void) {
    if !p.is_null() && !heap::free(p) {
        log::error!("free({p:p}): not a heap block (double free or foreign pointer)");
    }
}
#[inline(never)]
pub unsafe extern "win64" fn _free_base(p: *mut c_void) {
    free(p)
}
pub unsafe extern "win64" fn realloc(p: *mut c_void, size: usize) -> *mut c_void {
    if p.is_null() {
        return malloc(size);
    }
    if size == 0 {
        free(p);
        return std::ptr::null_mut();
    }
    let r = heap::realloc(p, size, false, false);
    if r.is_null() {
        errno::set(ENOMEM);
    }
    r
}
pub unsafe extern "win64" fn _realloc_base(p: *mut c_void, size: usize) -> *mut c_void {
    realloc(p, size)
}
pub unsafe extern "win64" fn _recalloc(p: *mut c_void, n: usize, size: usize) -> *mut c_void {
    let Some(t) = n.checked_mul(size) else {
        errno::set(ENOMEM);
        return std::ptr::null_mut();
    };
    if p.is_null() {
        return calloc(n, size);
    }
    let r = heap::realloc(p, t, true, false);
    if r.is_null() {
        errno::set(ENOMEM);
    }
    r
}
pub unsafe extern "win64" fn _recalloc_base(p: *mut c_void, n: usize, size: usize) -> *mut c_void {
    _recalloc(p, n, size)
}
pub unsafe extern "win64" fn _msize(p: *mut c_void) -> usize {
    match heap::size(p) {
        Some(s) => s,
        None => {
            errno::set(EINVAL);
            super::runtime::invalid_parameter("_msize on foreign pointer");
            usize::MAX
        }
    }
}
pub unsafe extern "win64" fn _msize_base(p: *mut c_void) -> usize {
    _msize(p)
}
pub unsafe extern "win64" fn _expand(p: *mut c_void, size: usize) -> *mut c_void {
    let r = heap::realloc(p, size, false, true);
    if r.is_null() {
        errno::set(ENOMEM);
    }
    r
}

pub unsafe extern "win64" fn _aligned_malloc(size: usize, align: usize) -> *mut c_void {
    _aligned_offset_malloc(size, align, 0)
}
pub unsafe extern "win64" fn _aligned_offset_malloc(size: usize, align: usize, off: usize) -> *mut c_void {
    if !align.is_power_of_two() || (size > 0 && off >= size) {
        errno::set(EINVAL);
        super::runtime::invalid_parameter("_aligned_offset_malloc");
        return std::ptr::null_mut();
    }
    let p = heap::aligned_alloc(size, align, off);
    if p.is_null() {
        errno::set(ENOMEM);
    }
    p
}
pub unsafe extern "win64" fn _aligned_free(p: *mut c_void) {
    if !p.is_null() {
        heap::aligned_free(p)
    }
}
pub unsafe extern "win64" fn _aligned_realloc(p: *mut c_void, size: usize, align: usize) -> *mut c_void {
    _aligned_offset_realloc(p, size, align, 0)
}
pub unsafe extern "win64" fn _aligned_offset_realloc(p: *mut c_void, size: usize, align: usize, off: usize) -> *mut c_void {
    if !align.is_power_of_two() {
        errno::set(EINVAL);
        return std::ptr::null_mut();
    }
    let r = heap::aligned_realloc(p, size, align, off);
    if r.is_null() && size != 0 {
        errno::set(ENOMEM);
    }
    r
}
pub unsafe extern "win64" fn _aligned_recalloc(p: *mut c_void, n: usize, size: usize, align: usize) -> *mut c_void {
    let Some(t) = n.checked_mul(size) else {
        errno::set(ENOMEM);
        return std::ptr::null_mut();
    };
    let old = if p.is_null() { 0 } else { heap::aligned_size(p) };
    let r = _aligned_realloc(p, t, align);
    if !r.is_null() && t > old {
        std::ptr::write_bytes((r as *mut u8).add(old), 0, t - old);
    }
    r
}
pub unsafe extern "win64" fn _aligned_msize(p: *mut c_void, _align: usize, _off: usize) -> usize {
    heap::aligned_size(p)
}

pub unsafe extern "win64" fn _get_heap_handle() -> isize {
    heap::process_heap() as isize
}
pub unsafe extern "win64" fn _heapchk() -> i32 {
    -2 // _HEAPOK
}
pub unsafe extern "win64" fn _heapmin() -> i32 {
    0
}
pub unsafe extern "win64" fn _heapwalk(_e: *mut c_void) -> i32 {
    -6 // _HEAPEND
}
pub unsafe extern "win64" fn _set_sbh_threshold(_t: usize) -> i32 {
    0
}
