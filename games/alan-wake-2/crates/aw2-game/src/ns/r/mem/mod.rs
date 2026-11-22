//! `r::mem`: Northlight memory resources.
//!
//! The engine's general-purpose heap is **mimalloc v2** (third-party). The
//! field layout matches mimalloc 2.1's `mi_segment_t` / `mi_page_t`:
//! - 32 MB-aligned segments. `segment + 0x100` holds `thread_id`, which on
//!   Windows is the TEB self pointer.
//! - 64 KB slices, with `mi_page_t` descriptors from `segment + 0x108` at a
//!   stride of 0x50. `slice_offset` (`u32` at +4) is the distance back to
//!   the page.
//! - Page fields: `flags` (`in_full` | `has_aligned`) at +0xe, `used`
//!   (`u32`) at +0x18, `local_free` at +0x20.
//!
//! Remedy functions inline mimalloc's `mi_free` fast path at many sites,
//! e.g. the `r::InplaceString` dtor 0x14240ab10 and reserve 0x142408f50.
//! The fast path is: owner thread, flags 0 → push onto `local_free`, then
//! `--used`. Everything else calls `_mi_free_generic` (0x142533200), and a
//! page whose `used` drops to 0 calls `_mi_page_retire` (0x1425371a0).
//!
//! Those two out-of-line functions are mimalloc code and are not ported.
//! Only the Remedy functions that contain the inlined fast path are. The
//! mimalloc names are matched by layout and behavior; there are no symbols.

use crate::exe;

/// mimalloc `_mi_free_generic(segment, page, is_local: u8 in r8b, p)` (not ported).
pub(crate) const REGION_FREE_SLOW_VA: u64 = 0x1_4253_3200;
/// mimalloc `_mi_page_retire(page)`: the page's `used` count dropped to 0 (not ported).
pub(crate) const REGION_SPAN_EMPTY_VA: u64 = 0x1_4253_71a0;

/// The inlined region free of a non-null block `p` (see the module docs).
#[inline(always)]
pub(crate) unsafe fn region_free(p: *mut u8) {
    use exe::{rd, wr};
    let chunk = (p as usize).wrapping_sub(1) & !0x1ff_ffff;
    let page = chunk + 0x108 + ((p as usize).wrapping_sub(chunk) >> 16) * 0x50;
    let span = page - rd::<u32>(page + 4) as usize;
    if exe::teb_self() != rd::<usize>(chunk + 0x100) {
        exe::call_va!(REGION_FREE_SLOW_VA, (chunk, span, 0u8, p));
        return;
    }
    if rd::<u8>(span + 0xe) != 0 {
        exe::call_va!(REGION_FREE_SLOW_VA, (chunk, span, 1u8, p));
        return;
    }
    wr::<usize>(p as usize, rd::<usize>(span + 0x20));
    wr::<usize>(span + 0x20, p as usize);
    let count = rd::<u32>(span + 0x18).wrapping_sub(1);
    wr::<u32>(span + 0x18, count);
    if count == 0 {
        exe::call_va!(REGION_SPAN_EMPTY_VA, (span));
    }
}

/// `r::mem::HeapResource::vfunc_2` — deallocate `p` (null is ignored). The
/// `this` argument is not used.
///
/// Original: 0x14020b500 (119 bytes). Slot 2 of the `r::mem::HeapResource`
/// vftable (RTTI name), so likely `deallocate(void*)`. Its body is the region
/// free with the two slow paths as tail jumps.
pub unsafe extern "win64" fn heap_resource_deallocate(_this: *mut u8, p: *mut u8) {
    if !p.is_null() {
        region_free(p);
    }
}

/// `r::mem::free(void*)` — deallocate `p` (null is ignored).
///
/// Originals: 0x142442de0 (128 bytes, 4,268 call sites) and identical unmerged
/// COMDAT copies at 0x1420d4010, 0x1421f2730, 0x14241ba20.
pub unsafe extern "win64" fn region_free_ptr(p: *mut u8) {
    if !p.is_null() {
        region_free(p);
    }
}

/// `r::mem::freeArray(void*)` — free an array allocated with a 16-byte prefix
/// header `{ u64 elem_size, u32 count, u32 align }` at `[p - 0x10 .. p)`
/// (see [`alloc_array`]). Null `p` (or `p == hdr`) is ignored.
///
/// Original: 0x142442c80 (157 bytes, 1,206 call sites).
pub unsafe extern "win64" fn free_array(p: *mut u8) {
    if p.is_null() {
        return;
    }
    let align = (exe::rd::<u32>((p as usize).wrapping_sub(4)) as usize).max(0x10);
    let raw = (p as usize).wrapping_sub(align);
    if raw != 0 {
        region_free(raw as *mut u8);
    }
}

/// Release a `{ ptr, u64 }` buffer header: region-free `ptr` (if non-null),
/// then zero both qwords. The owning container type is unknown; it's used
/// as the destructor/clear of many small engine containers (2369 call
/// sites, e.g. from 0x1401de400 / 0x1401deb40). The header is not always
/// 8-aligned (seen at `...69c`), hence the unaligned accessors.
///
/// Original: 0x1401de830 (154 bytes).
pub unsafe extern "win64" fn release_buffer(hdr: *mut [usize; 2]) {
    let h = hdr as usize;
    let p = exe::rd::<usize>(h) as *mut u8;
    if !p.is_null() {
        region_free(p);
    }
    exe::wr::<usize>(h, 0);
    exe::wr::<usize>(h + 8, 0);
}

/// Clear and release a POD `r::Vector<T>` header `{ ptr: *mut T, len: u32, cap_flags: u32 }`:
/// sets `len = 0`, and unless bit 30 (`0x4000_0000`, non-owning/inline storage flag)
/// is set in `cap_flags`, frees `ptr` (if non-null), masks `cap_flags &= 0xc000_0000`,
/// and zeros `ptr`.
///
/// Originals: 0x1401f2bc0 (166 bytes, 580 call sites) and 38 identical unmerged
/// COMDAT copies listed in `registry.rs`. Also covers the 140-byte `deallocate`
/// COMDAT copies at 0x1401e66f0 (434 call sites) and 0x1403e8060 (43 call sites) via
/// [`heap_resource_deallocate`].
pub unsafe extern "win64" fn clear_pod_vector(hdr: *mut u8) {
    let h = hdr as usize;
    let cap_flags = exe::rd::<u32>(h + 0x0c);
    exe::wr::<u32>(h + 0x08, 0);
    if cap_flags & 0x4000_0000 == 0 {
        let p = exe::rd::<usize>(h) as *mut u8;
        if !p.is_null() {
            region_free(p);
        }
        exe::wr::<u32>(h + 0x0c, exe::rd::<u32>(h + 0x0c) & 0xc000_0000);
        exe::wr::<usize>(h, 0);
    }
}

// ---------------------------------------------------------------------------
// Allocation: the inlined `mi_heap_malloc_aligned(heap, size, ALIGN)` fast path
// ---------------------------------------------------------------------------

/// mimalloc `_mi_malloc_generic(heap, size, zero: bool, huge_alignment)` (not ported).
pub(crate) const MALLOC_GENERIC_VA: u64 = 0x1_4253_7280;
/// mimalloc's out-of-line aligned allocation `(heap, size, alignment)` (not ported).
pub(crate) const MALLOC_ALIGNED_VA: u64 = 0x1_4253_5070;
/// mimalloc `mi_malloc_aligned(size, alignment)` (not ported).
pub(crate) const MI_MALLOC_ALIGNED_VA: u64 = 0x1_4253_5360;
/// Offset of the thread's default `mi_heap_t*` in the exe's static TLS block.
pub(crate) const HEAP_TLS_OFFSET: usize = 0x6348;
/// IAT slots of `KERNEL32!GetCurrentProcess` / `KERNEL32!TerminateProcess`.
pub(crate) const GET_CURRENT_PROCESS_IAT: u64 = 0x1_4264_56c8;
pub(crate) const TERMINATE_PROCESS_IAT: u64 = 0x1_4264_53a0;
/// Exit code of the out-of-memory termination.
pub(crate) const OOM_EXIT_CODE: u32 = 0xdead_d00d;

/// Terminate the process with [`OOM_EXIT_CODE`] (`0xdeadd00d`) via the IAT slots
/// of `GetCurrentProcess` and `TerminateProcess`.
#[inline(always)]
unsafe fn oom_terminate() {
    let process = exe::call_iat!(GET_CURRENT_PROCESS_IAT, () -> usize);
    exe::call_iat!(TERMINATE_PROCESS_IAT, (process, OOM_EXIT_CODE) -> i32);
}

/// Pop the first block of a page's `free` list (`free` at +0x10, `used` u32 at +0x18).
#[inline(always)]
unsafe fn page_pop(page: usize, block: usize) {
    use exe::{rd, wr};
    wr::<u32>(page + 0x18, rd::<u32>(page + 0x18).wrapping_add(1));
    wr::<usize>(page + 0x10, rd::<usize>(block));
}

/// The inlined region allocation: `size` bytes aligned to `ALIGN` from the
/// thread's mimalloc heap, or `TerminateProcess(GetCurrentProcess(),
/// 0xdeadd00d)` (then null) when that fails.
///
/// The heap is `[[gs:[0x58]] + 0x6348]`; `pages_free_direct[(size + 7) / 8]`
/// at heap+8 is the small page for the size. A power-of-two size in
/// `ALIGN..=0x400` is naturally aligned and takes the plain small path (pop
/// the page's free block, else `_mi_malloc_generic(heap, size, 0, 0)`). Any
/// other size up to `isize::MAX` pops the small page's free block only if it is
/// `ALIGN`-aligned, else calls the aligned slow path; larger sizes fail.
#[inline(always)]
pub(crate) unsafe fn region_alloc<const ALIGN: usize>(size: usize) -> *mut u8 {
    use exe::rd;
    let heap = rd::<usize>(rd::<usize>(exe::tls_pointer()).wrapping_add(HEAP_TLS_OFFSET));
    let small_page = |size: usize| rd::<usize>(heap + 8 + ((size + 7) >> 3) * 8);
    let p = if size & size.wrapping_sub(1) == 0 && size.wrapping_sub(ALIGN) <= 0x400 - ALIGN {
        let page = small_page(size);
        let block = rd::<usize>(page + 0x10);
        if block == 0 {
            exe::call_va!(MALLOC_GENERIC_VA, (heap, size, 0u64, 0u64) -> usize)
        } else {
            page_pop(page, block);
            block
        }
    } else if size > isize::MAX as usize {
        0
    } else {
        let mut fast = 0;
        if size.wrapping_sub(ALIGN) <= 0x400 - ALIGN {
            let page = small_page(size);
            let block = rd::<usize>(page + 0x10);
            if block != 0 && block & (ALIGN - 1) == 0 {
                page_pop(page, block);
                fast = block;
            }
        }
        if fast != 0 {
            fast
        } else {
            exe::call_va!(MALLOC_ALIGNED_VA, (heap, size, ALIGN) -> usize)
        }
    };
    if p == 0 {
        oom_terminate();
        return std::ptr::null_mut();
    }
    p as *mut u8
}

/// 1-argument `region_alloc::<ALIGN>(size)`: allocate `size` bytes aligned to
/// `ALIGN` from the thread's mimalloc heap, terminating on OOM.
///
/// Originals: 0x142443800 (`ALIGN = 8`), 0x1420d40a0 / 0x1424ca060 / 0x1424ca530
/// (`ALIGN = 16`), 0x1421f27c0 (`ALIGN = 32`), 0x1420aa4b0 (`ALIGN = 64`).
pub unsafe extern "win64" fn region_alloc_aligned<const ALIGN: usize>(size: usize) -> *mut u8 {
    region_alloc::<ALIGN>(size)
}

/// `allocate(n)` of the engine's STL allocator for an element of `ELEM` bytes:
/// `n * ELEM` bytes (wrapping multiply), 16-aligned, from the region heap
/// (see [`region_alloc`]). `this` is not used. The class name is not known
/// (no RTTI); one instantiation per element type, with identical code for
/// equal sizes (not folded by the linker).
///
/// Originals: ~150 functions of 224-232 bytes listed in `registry.rs`, e.g.
/// 0x1401decf0 (`ELEM` 4), 0x1401f2340 (0x88), 0x140215e80 (0x10).
pub unsafe extern "win64" fn stl_allocate<const ELEM: usize>(_this: *mut u8, n: usize) -> *mut u8 {
    region_alloc::<16>(n.wrapping_mul(ELEM))
}

/// `r::mem::alloc(size, align)`: allocate `size` bytes aligned to
/// `align.max(16)` via `mi_malloc_aligned`, terminating on OOM.
///
/// Original: 0x142442e70 (53 bytes).
pub unsafe extern "win64" fn alloc_aligned(size: usize, align: usize) -> *mut u8 {
    let p = exe::call_va!(MI_MALLOC_ALIGNED_VA, (size, align.max(0x10)) -> usize);
    if p == 0 {
        oom_terminate();
        return std::ptr::null_mut();
    }
    p as *mut u8
}

/// `r::mem::HeapResource::vfunc_1`: allocate `size` bytes aligned to `align`
/// via `mi_malloc_aligned`, terminating on OOM. `this` is unused.
///
/// Original: 0x1413b58c0 (47 bytes).
pub unsafe extern "win64" fn heap_resource_allocate(_this: *mut u8, size: usize, align: usize) -> *mut u8 {
    let p = exe::call_va!(MI_MALLOC_ALIGNED_VA, (size, align) -> usize);
    if p == 0 {
        oom_terminate();
        return std::ptr::null_mut();
    }
    p as *mut u8
}

/// `r::mem::allocArray(elem_size, count, align)`: allocate an array with a
/// `hdr = align.max(16)`-byte prefix whose last 16 bytes store
/// `{ elem_size: u64, count: u32, align: u32 }` (paired with [`free_array`]).
/// Zero total byte size adds 1 byte to the allocation request.
///
/// Original: 0x142442d30 (142 bytes).
pub unsafe extern "win64" fn alloc_array(elem_size: usize, count: usize, align: usize) -> *mut u8 {
    let hdr = align.max(0x10);
    let bytes = elem_size.wrapping_mul(count);
    let total = bytes.wrapping_add(hdr).wrapping_add(usize::from(bytes == 0));
    let mut raw = exe::call_va!(MI_MALLOC_ALIGNED_VA, (total, hdr) -> usize);
    if raw == 0 {
        oom_terminate();
        raw = 0;
    }
    let p = raw.wrapping_add(hdr);
    exe::wr::<u64>(p.wrapping_sub(0x10), elem_size as u64);
    exe::wr::<u32>(p.wrapping_sub(0x08), count as u32);
    exe::wr::<u32>(p.wrapping_sub(0x04), align as u32);
    p as *mut u8
}

/// Release a POD `r::InplaceVector<T, ICAP>` header `{ ptr: *mut T, len: u32, cap: u32, inline: [T; ICAP] }`:
/// if `ptr` is non-null and not equal to `this + 0x10`, frees it via [`region_free`];
/// then resets `len = 0`, `ptr = this + 0x10`, and `cap = ICAP`.
///
/// Originals: 0x1401ee280 (162 bytes, 3,131 call sites, `ICAP = 16`),
/// 0x1403b8050 (`ICAP = 4`, 174 call sites), and 10 other `ICAP` instantiations
/// plus 12 15-byte destructor wrappers listed in `registry.rs`.
pub unsafe extern "win64" fn release_inplace_buffer<const ICAP: u32>(hdr: *mut u8) {
    let h = hdr as usize;
    let p = exe::rd::<usize>(h);
    let inline = h.wrapping_add(0x10);
    if p != 0 && p != inline {
        region_free(p as *mut u8);
    }
    exe::wr::<u32>(h + 0x08, 0);
    exe::wr::<usize>(h, inline);
    exe::wr::<u32>(h + 0x0c, ICAP);
}

/// `j_memmove` thunk in the exe (`0x14183acc8`, `(dst, src, size) -> dst`).
const J_MEMMOVE_VA: u64 = 0x1_4183_acc8;

/// `r::Vector<T>::reallocate(new_cap)` for a POD element of `ELEM` bytes:
/// allocates `new_cap * ELEM` bytes (16-aligned via [`region_alloc`]), copies
/// `len * ELEM` bytes via `j_memmove`, releases the old buffer via `RELEASE_VA`
/// (`0x1401de830` or `0x140f366d0`), and writes `(new_buf, saved_len, new_cap as u32)`.
///
/// Originals: 19 functions of 297-307 bytes listed in `registry.rs`, e.g.
/// 0x1408746b0 (`ELEM = 8`, 73 call sites), 0x140a5b830 (`ELEM = 16`, 32 call sites).
pub unsafe extern "win64" fn pod_vec_realloc<const ELEM: usize, const RELEASE_VA: u64>(
    vec: *mut u8,
    new_cap: usize,
) {
    let v = vec as usize;
    let saved_len = exe::rd::<u32>(v + 0x08);
    let new_buf = region_alloc::<16>(new_cap.wrapping_mul(ELEM));
    let cur_len = exe::rd::<u32>(v + 0x08) as usize;
    let old_buf = exe::rd::<*const u8>(v);
    exe::call_va!(J_MEMMOVE_VA, (new_buf, old_buf, cur_len.wrapping_mul(ELEM)) -> *mut u8);
    exe::call_va!(RELEASE_VA, (vec));
    exe::wr::<u32>(v + 0x08, saved_len);
    exe::wr::<u32>(v + 0x0c, new_cap as u32);
    exe::wr::<*mut u8>(v, new_buf);
}

/// `r::Vector<T>::growAndReallocate(min_cap)` for a POD element of `ELEM` bytes:
/// computes `new_cap = (old_cap + (old_cap >> 1)).max(min_cap)` and delegates to
/// [`pod_vec_realloc`].
///
/// Originals: 25 functions of 312-322 bytes (~3,500 call sites) listed in
/// `registry.rs`, e.g. 0x14020afa0 (`ELEM = 0x30`, 1,873 call sites),
/// 0x140215610 (`ELEM = 2`, 809 call sites), 0x140215750 (`ELEM = 0x48`, 486 call sites).
pub unsafe extern "win64" fn pod_vec_grow<const ELEM: usize, const RELEASE_VA: u64>(
    vec: *mut u8,
    min_cap: usize,
) {
    let v = vec as usize;
    let old_cap = exe::rd::<u32>(v + 0x0c) as usize;
    let new_cap = (old_cap + (old_cap >> 1)).max(min_cap);
    pod_vec_realloc::<ELEM, RELEASE_VA>(vec, new_cap);
}

/// `r::InplaceVector<T, ICAP>::reallocate(new_cap)` for a POD element of `ELEM` bytes:
/// if `new_cap <= ICAP` and `ptr != this + 0x10`, switches back to the inline
/// buffer (`this + 0x10`, `actual_cap = ICAP`); otherwise allocates
/// `new_cap * ELEM` bytes via [`region_alloc`], copies `len * ELEM` bytes via
/// `j_memmove`, releases the old buffer via `RELEASE_VA`, and updates the header.
///
/// Originals: 0x1413aa3e0 (347 bytes, 1,419 call sites, `ELEM = 0x30, ICAP = 16`)
/// and 8 sibling instantiations listed in `registry.rs`.
pub unsafe extern "win64" fn inplace_vec_realloc<
    const ELEM: usize,
    const ICAP: usize,
    const RELEASE_VA: u64,
>(
    vec: *mut u8,
    new_cap: usize,
) {
    let v = vec as usize;
    let saved_len = exe::rd::<u32>(v + 0x08);
    let inline_buf = v.wrapping_add(0x10) as *mut u8;
    let (new_buf, actual_cap) = if new_cap <= ICAP && exe::rd::<*mut u8>(v) != inline_buf {
        (inline_buf, ICAP)
    } else {
        (region_alloc::<16>(new_cap.wrapping_mul(ELEM)), new_cap)
    };
    let cur_len = exe::rd::<u32>(v + 0x08) as usize;
    let old_buf = exe::rd::<*const u8>(v);
    exe::call_va!(J_MEMMOVE_VA, (new_buf, old_buf, cur_len.wrapping_mul(ELEM)) -> *mut u8);
    exe::call_va!(RELEASE_VA, (vec));
    exe::wr::<u32>(v + 0x08, saved_len);
    exe::wr::<*mut u8>(v, new_buf);
    exe::wr::<u32>(v + 0x0c, actual_cap as u32);
}

const J_MEMSET_VA: u64 = 0x1_4183_acc2;

/// `r::mem::callocArray<T>(_rcx, _rdx, _r8, count)`: allocate `count * ELEM`
/// bytes aligned to 16 from the thread's mimalloc heap and zero-fill via
/// `j_memset` when `count != 0`.
///
/// Originals: 79 instantiations of 263-281 bytes across 14 element sizes,
/// including 0x140291af0 (`ELEM = 8`, 111 call sites) and 0x14020c080
/// (`ELEM = 1`, 62 call sites).
pub unsafe extern "win64" fn calloc_array<const ELEM: usize>(
    _rcx: usize,
    _rdx: usize,
    _r8: usize,
    count: usize,
) -> *mut u8 {
    let size = count.wrapping_mul(ELEM);
    let p = region_alloc::<16>(size);
    if count != 0 {
        exe::call_va!(J_MEMSET_VA, (p, 0i32, size) -> *mut u8);
    }
    p
}

/// `r::Vector<T>::Vector(usize count)` zero-initializing constructor for a POD
/// element of `ELEM` bytes: zeros header, and if `count != 0` allocates
/// `count * ELEM` bytes, sets `[this] = p`, `[this + 0x0c] = count as u32`, and
/// zero-fills `p` via `j_memset`, then sets `[this + 0x08] = count as u32`.
///
/// Originals: 0x1411db3a0 (`ELEM = 48`), 0x1411db4d0 (`ELEM = 32`),
/// 0x1412ea6a0 (`ELEM = 8`).
pub unsafe extern "win64" fn pod_vec_ctor_zero<const ELEM: usize>(
    this: *mut u8,
    count: usize,
) -> *mut u8 {
    use exe::wr;
    let t = this as usize;
    wr::<usize>(t, 0);
    wr::<u64>(t + 0x08, 0);
    if count != 0 {
        let size = count.wrapping_mul(ELEM);
        let p = region_alloc::<16>(size);
        wr::<*mut u8>(t, p);
        wr::<u32>(t + 0x0c, count as u32);
        exe::call_va!(J_MEMSET_VA, (p, 0i32, size) -> *mut u8);
    }
    wr::<u32>(t + 0x08, count as u32);
    this
}

/// `r::Vector<T>::back()` for 48-byte elements: `data + (len * 48) - 48`.
///
/// Original: 0x1401e7b20 (22 bytes, 65 call sites).
pub unsafe extern "win64" fn pod_vec_back_48(vec: *const u8) -> *mut u8 {
    let v = vec as usize;
    let len = exe::rd::<u32>(v + 0x08) as usize;
    exe::rd::<usize>(v)
        .wrapping_sub(0x30)
        .wrapping_add((len * 3) << 4) as *mut u8
}

/// `&vec[vec.size() - 2]` for 48-byte elements: `data + ((len - 2) * 48)`.
///
/// Original: 0x1401e7b40 (19 bytes, 29 call sites).
pub unsafe extern "win64" fn pod_vec_penultimate_48(vec: *const u8) -> *mut u8 {
    let v = vec as usize;
    let len = exe::rd::<u32>(v + 0x08) as usize;
    exe::rd::<usize>(v).wrapping_add((len.wrapping_sub(2).wrapping_mul(3)) << 4) as *mut u8
}

/// Initialize a `u16`-span filtered iterator `{ table, cur, end }` at `out`
/// (`rcx`) from `table` (`rdx`) and `span` (`r8`: `{ count: u64, data: *const u16 }`),
/// advancing `cur` to the first entry where `table[0x48 + 8*idx] != 0` and
/// `table[0x10048 + 4*idx] != 0`.
///
/// Original: 0x1401e7c10 (72 bytes, 429 call sites).
pub unsafe extern "win64" fn comp_span_iter_begin(
    out: *mut u8,
    table: *const u8,
    span: *const u8,
) -> *mut u8 {
    use exe::{rd, wr};
    let o = out as usize;
    let t = table as usize;
    let s = span as usize;
    let mut cur = rd::<usize>(s + 0x08);
    let count = rd::<usize>(s);
    let end = cur.wrapping_add(count.wrapping_mul(2));
    wr::<usize>(o, t);
    wr::<usize>(o + 0x08, cur);
    wr::<usize>(o + 0x10, end);
    while cur != end {
        let idx = rd::<u16>(cur) as usize;
        if rd::<u64>(t + 0x48 + idx * 8) != 0 && rd::<u32>(t + 0x10048 + idx * 4) != 0 {
            break;
        }
        cur = cur.wrapping_add(2);
        wr::<usize>(o + 0x08, cur);
    }
    out
}

/// Dereference a `u16`-span filtered iterator (`*out = *this->cur; return out`).
///
/// Original: 0x1401e7c60 (14 bytes).
pub unsafe extern "win64" fn comp_span_iter_deref(this: *const u8, out: *mut u16) -> *mut u16 {
    use exe::{rd, wr};
    let cur = rd::<usize>((this as usize) + 0x08);
    wr::<u16>(out as usize, rd::<u16>(cur));
    out
}

/// Pre-increment (`++it`) a `u16`-span filtered iterator (`0x1401e7c70`, 75 bytes).
pub unsafe extern "win64" fn comp_span_iter_next(this: *mut u8) -> *mut u8 {
    use exe::{rd, wr};
    let it = this as usize;
    let mut cur = rd::<usize>(it + 0x08).wrapping_add(2);
    wr::<usize>(it + 0x08, cur);
    let end = rd::<usize>(it + 0x10);
    while cur != end {
        let t = rd::<usize>(it);
        let idx = rd::<u16>(cur) as usize;
        if rd::<u64>(t + 0x48 + idx * 8) != 0 && rd::<u32>(t + 0x10048 + idx * 4) != 0 {
            break;
        }
        cur = cur.wrapping_add(2);
        wr::<usize>(it + 0x08, cur);
    }
    this
}

/// Initialize a 56-byte outer component iterator at `out` (`0x1401e7cd0`, 38 bytes).
pub unsafe extern "win64" fn comp_outer_iter_ctor(
    out: *mut u8,
    ctx: usize,
    inner: *const u8,
) -> *mut u8 {
    use exe::{rd, wr};
    let o = out as usize;
    let i = inner as usize;
    let w0 = rd::<u64>(i);
    let w1 = rd::<u64>(i + 0x08);
    let w2 = rd::<u64>(i + 0x10);
    wr::<usize>(o, ctx);
    wr::<u64>(o + 0x08, w0);
    wr::<u64>(o + 0x10, w1);
    wr::<u64>(o + 0x28, 0);
    wr::<u64>(o + 0x30, 0);
    wr::<u64>(o + 0x18, w2);
    out
}

/// Load the current bucket array/count into an outer component iterator (`0x1401e7d10`, 65 bytes).
pub unsafe extern "win64" fn comp_outer_iter_load(this: *mut u8) {
    use exe::{rd, wr};
    let it = this as usize;
    let cur = rd::<usize>(it + 0x10);
    if cur != rd::<usize>(it + 0x18) {
        let idx = rd::<u16>(cur) as usize;
        let t = rd::<usize>(it);
        let ptr = rd::<u64>(t + 0x48 + idx * 8);
        wr::<u64>(it + 0x20, ptr);
        if rd::<u64>(t + 0x48 + idx * 8) != 0 {
            let count = rd::<u32>(t + 0x10048 + idx * 4) as u64;
            wr::<u64>(it + 0x28, count);
            wr::<u64>(it + 0x30, 0);
        } else {
            wr::<u64>(it + 0x28, 0);
            wr::<u64>(it + 0x30, 0);
        }
    }
}

const J_FREE_SIZED_VA: u64 = 0x1_4183_4d30;
const REGION_FREE_PTR_VA: u64 = 0x1_4244_2de0;
const SCRATCH_FREE_VA: u64 = 0x1_4241_c190;
const THRD_YIELD_IAT: u64 = 0x1_4264_5868;
const GET_CURRENT_THREAD_ID_IAT: u64 = 0x1_4264_5790;
const RWLOCK_WAIT_AND_ADD_VA: u64 = 0x1_4137_9040;

/// Trivial scalar deleting destructor (`vfunc_0`) for a class of `SIZE` bytes:
/// if `(flags & 1) != 0`, calls `j_j_free(this, SIZE)` (`0x141834d30`), and
/// returns `this`.
///
/// Originals: 192 functions of 33 bytes across 19 object sizes (`8..496`),
/// e.g. `content::EQSTask::vfunc_0` (`0x1401ed460`).
pub unsafe extern "win64" fn scalar_deleting_dtor<const SIZE: usize>(
    this: *mut u8,
    flags: u8,
) -> *mut u8 {
    if (flags & 1) != 0 {
        exe::call_va!(J_FREE_SIZED_VA, (this, SIZE));
    }
    this
}

/// `r::UniquePtr<T>::~UniquePtr()` for a polymorphic pointee whose non-deleting
/// virtual destructor is at vtable byte offset `VTBL_OFF` (`0` or `8`), followed
/// by `r::mem::free(p)` (`0x142442de0`).
///
/// Originals: 34 functions with `VTBL_OFF = 0` (39 bytes, e.g. `0x1401e18a0`)
/// and 65 functions with `VTBL_OFF = 8` (40 bytes, e.g. `0x1401f49e0`).
pub unsafe extern "win64" fn unique_ptr_dtor<const VTBL_OFF: usize>(this: *mut u8) {
    let p = exe::rd::<usize>(this as usize);
    if p != 0 {
        let vtbl = exe::rd::<usize>(p);
        let dtor_va = exe::rd::<u64>(vtbl.wrapping_add(VTBL_OFF));
        exe::call_va!(dtor_va, (p as *mut u8, 0u32));
        exe::call_va!(REGION_FREE_PTR_VA, (p as *mut u8));
    }
}

/// Destructor for a scratch-allocated POD vector `{ data: *mut T, len: u32, cap: u32 }`:
/// if `data` is non-null, frees it via `r::scratch::scratch_free` (`0x14241c190`),
/// then zeros both 8-byte header words.
///
/// Originals: 35 functions of 37 bytes (175 call sites), e.g. `0x1401e68b0`.
pub unsafe extern "win64" fn scratch_vec_dtor(this: *mut u8) {
    use exe::{rd, wr};
    let t = this as usize;
    let p = rd::<*mut u8>(t);
    if !p.is_null() {
        exe::call_va!(SCRATCH_FREE_VA, (p));
    }
    wr::<u64>(t, 0);
    wr::<u64>(t + 0x08, 0);
}

#[inline(always)]
unsafe fn atomic_fetch_dec_u32(addr: usize) -> u32 {
    #[cfg(target_arch = "x86_64")]
    {
        let mut prev: u32 = u32::MAX;
        std::arch::asm!(
            "lock xadd dword ptr [{p}], {v:e}",
            p = in(reg) addr,
            v = inout(reg) prev,
            options(nostack),
        );
        prev
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        (*(addr as *const std::sync::atomic::AtomicU32))
            .fetch_sub(1, std::sync::atomic::Ordering::SeqCst)
    }
}

/// `std::shared_ptr<T>::~shared_ptr()` (75 bytes): decrements the control
/// block's `_Uses` at `rep + 0x08` (`lock xadd`), calling `_Destroy()`
/// (`vtable[0]`) and decrementing `_Weaks` at `rep + 0x0c` (calling
/// `_Delete_this()` at `vtable[1]` when it reaches zero).
///
/// Originals: 34 byte-identical functions of 75 bytes, e.g. `0x140200d90`.
pub unsafe extern "win64" fn shared_ptr_dtor(this: *mut u8) {
    let rep = exe::rd::<usize>((this as usize).wrapping_add(0x08));
    if rep != 0 {
        if atomic_fetch_dec_u32(rep.wrapping_add(0x08)) == 1 {
            let vtbl0 = exe::rd::<usize>(rep);
            let destroy_va = exe::rd::<u64>(vtbl0);
            exe::call_va!(destroy_va, (rep as *mut u8));
            if atomic_fetch_dec_u32(rep.wrapping_add(0x0c)) == 1 {
                let vtbl1 = exe::rd::<usize>(rep);
                let delete_this_va = exe::rd::<u64>(vtbl1.wrapping_add(0x08));
                exe::call_va!(delete_this_va, (rep as *mut u8));
            }
        }
    }
}

/// SwissTable `InitializeSlots<ALIGN, SLOT>(this, cap)`: allocates a combined
/// control-byte + slot array of `((cap + 0x10 + ALIGN) & !(ALIGN - 1)) + cap * SLOT`
/// bytes via [`region_alloc`], initializes control bytes to `0x80` (`-128`)
/// with sentinel `0xff` at `ctrl[cap]`, and sets `growth_left` at `this + 0x28`
/// to `(cap - (cap >> 3)) - size`.
///
/// Originals: 11 functions of 322-330 bytes (`0x1401de6e0`, `0x140234ca0`,
/// `0x1403c1820`, `0x140fa3610`, `0x140fc58f0`, `0x140fc5bf0`, `0x14114e530`,
/// `0x1413a3410`, `0x141dc9d70`, `0x1420809c0`, `0x14212b020`).
pub unsafe extern "win64" fn swiss_init_slots<const ALIGN: usize, const SLOT: usize>(
    this: *mut u8,
    cap: usize,
) {
    use exe::{rd, wr};
    let t = this as usize;
    let ctrl_bytes = cap.wrapping_add(0x10 + ALIGN) & !(ALIGN - 1);
    let total = ctrl_bytes.wrapping_add(cap.wrapping_mul(SLOT));
    let ctrl = region_alloc::<16>(total);
    let slots = ctrl.wrapping_add(ctrl_bytes);
    wr::<*mut u8>(t, ctrl);
    wr::<*mut u8>(t + 0x08, slots);
    exe::call_va!(J_MEMSET_VA, (ctrl, -0x80i32, cap.wrapping_add(0x10)) -> *mut u8);
    let ctrl_reload = rd::<usize>(t);
    wr::<u8>(ctrl_reload.wrapping_add(cap), 0xff);
    let growth_left = cap
        .wrapping_sub(cap >> 3)
        .wrapping_sub(rd::<usize>(t + 0x10));
    wr::<usize>(t + 0x28, growth_left);
}

/// `std::_Tree_val::_Insert_node(loc, new_node)` Red-Black Tree insertion and
/// rebalancing (603 bytes, 206 call sites across 9 byte-identical copies).
///
/// Originals: `0x140208500`, `0x1417a6a80`, `0x141955370`, `0x1419ddfa0`,
/// `0x141ab0f40`, `0x141ac0ea0`, `0x141dd8fd0`, `0x141dd92f0`, `0x142310f00`.
pub unsafe extern "win64" fn rb_tree_insert_node(
    this: *mut u8,
    loc: *const u8,
    new_node: *mut u8,
) -> *mut u8 {
    use exe::{rd, wr};
    let t = this as usize;
    let l = loc as usize;
    let r9 = new_node as usize;
    wr::<u64>(t + 0x08, rd::<u64>(t + 0x08).wrapping_add(1));
    let r11 = rd::<usize>(t);
    let parent0 = rd::<usize>(l);
    wr::<usize>(r9 + 0x08, parent0);
    if parent0 == r11 {
        wr::<usize>(r11, r9);
        wr::<usize>(r11 + 0x08, r9);
        wr::<usize>(r11 + 0x10, r9);
        wr::<u8>(r9 + 0x18, 1);
        return new_node;
    }
    if rd::<u32>(l + 0x08) == 0 {
        wr::<usize>(parent0 + 0x10, r9);
        if parent0 == rd::<usize>(r11 + 0x10) {
            wr::<usize>(r11 + 0x10, r9);
        }
    } else {
        wr::<usize>(parent0, r9);
        if parent0 == rd::<usize>(r11) {
            wr::<usize>(r11, r9);
        }
    }
    let mut cur = r9;
    while rd::<u8>(rd::<usize>(cur + 0x08) + 0x18) == 0 {
        let p = rd::<usize>(cur + 0x08);
        let gp = rd::<usize>(p + 0x08);
        let gp_left = rd::<usize>(gp);
        if p == gp_left {
            let uncle = rd::<usize>(gp + 0x10);
            if rd::<u8>(uncle + 0x18) == 0 {
                wr::<u8>(p + 0x18, 1);
                wr::<u8>(uncle + 0x18, 1);
                let p2 = rd::<usize>(cur + 0x08);
                let gp2 = rd::<usize>(p2 + 0x08);
                wr::<u8>(gp2 + 0x18, 0);
                let p3 = rd::<usize>(cur + 0x08);
                cur = rd::<usize>(p3 + 0x08);
            } else {
                let p_right = rd::<usize>(p + 0x10);
                if cur == p_right {
                    let sub_left = rd::<usize>(p_right);
                    cur = p;
                    wr::<usize>(p + 0x10, sub_left);
                    let sub_left2 = rd::<usize>(p_right);
                    if rd::<u8>(sub_left2 + 0x19) == 0 {
                        wr::<usize>(sub_left2 + 0x08, p);
                    }
                    let p_parent = rd::<usize>(p + 0x08);
                    wr::<usize>(p_right + 0x08, p_parent);
                    let head = rd::<usize>(t);
                    if p == rd::<usize>(head + 0x08) {
                        wr::<usize>(head + 0x08, p_right);
                    } else {
                        let pp = rd::<usize>(p + 0x08);
                        if p == rd::<usize>(pp) {
                            wr::<usize>(pp, p_right);
                        } else {
                            wr::<usize>(pp + 0x10, p_right);
                        }
                    }
                    wr::<usize>(p_right, p);
                    wr::<usize>(p + 0x08, p_right);
                }
                let p2 = rd::<usize>(cur + 0x08);
                wr::<u8>(p2 + 0x18, 1);
                let p3 = rd::<usize>(cur + 0x08);
                let gp2 = rd::<usize>(p3 + 0x08);
                wr::<u8>(gp2 + 0x18, 0);
                let p4 = rd::<usize>(cur + 0x08);
                let gp3 = rd::<usize>(p4 + 0x08);
                let r8 = rd::<usize>(gp3);
                let sub_right = rd::<usize>(r8 + 0x10);
                wr::<usize>(gp3, sub_right);
                let sub_right2 = rd::<usize>(r8 + 0x10);
                if rd::<u8>(sub_right2 + 0x19) == 0 {
                    wr::<usize>(sub_right2 + 0x08, gp3);
                }
                let gp_parent = rd::<usize>(gp3 + 0x08);
                wr::<usize>(r8 + 0x08, gp_parent);
                let head = rd::<usize>(t);
                if gp3 == rd::<usize>(head + 0x08) {
                    wr::<usize>(head + 0x08, r8);
                } else {
                    let gpp = rd::<usize>(gp3 + 0x08);
                    if gp3 == rd::<usize>(gpp + 0x10) {
                        wr::<usize>(gpp + 0x10, r8);
                    } else {
                        wr::<usize>(gpp, r8);
                    }
                }
                wr::<usize>(r8 + 0x10, gp3);
                wr::<usize>(gp3 + 0x08, r8);
            }
        } else {
            let uncle = gp_left;
            if rd::<u8>(uncle + 0x18) == 0 {
                wr::<u8>(p + 0x18, 1);
                wr::<u8>(uncle + 0x18, 1);
                let p2 = rd::<usize>(cur + 0x08);
                let gp2 = rd::<usize>(p2 + 0x08);
                wr::<u8>(gp2 + 0x18, 0);
                let p3 = rd::<usize>(cur + 0x08);
                cur = rd::<usize>(p3 + 0x08);
            } else {
                let p_left = rd::<usize>(p);
                if cur == p_left {
                    let sub_right = rd::<usize>(p_left + 0x10);
                    cur = p;
                    wr::<usize>(p, sub_right);
                    let sub_right2 = rd::<usize>(p_left + 0x10);
                    if rd::<u8>(sub_right2 + 0x19) == 0 {
                        wr::<usize>(sub_right2 + 0x08, p);
                    }
                    let p_parent = rd::<usize>(p + 0x08);
                    wr::<usize>(p_left + 0x08, p_parent);
                    let head = rd::<usize>(t);
                    if p == rd::<usize>(head + 0x08) {
                        wr::<usize>(head + 0x08, p_left);
                    } else {
                        let pp = rd::<usize>(p + 0x08);
                        if p == rd::<usize>(pp + 0x10) {
                            wr::<usize>(pp + 0x10, p_left);
                        } else {
                            wr::<usize>(pp, p_left);
                        }
                    }
                    wr::<usize>(p_left + 0x10, p);
                    wr::<usize>(p + 0x08, p_left);
                }
                let p2 = rd::<usize>(cur + 0x08);
                wr::<u8>(p2 + 0x18, 1);
                let p3 = rd::<usize>(cur + 0x08);
                let gp2 = rd::<usize>(p3 + 0x08);
                wr::<u8>(gp2 + 0x18, 0);
                let p4 = rd::<usize>(cur + 0x08);
                let gp3 = rd::<usize>(p4 + 0x08);
                let r8 = rd::<usize>(gp3 + 0x10);
                let sub_left = rd::<usize>(r8);
                wr::<usize>(gp3 + 0x10, sub_left);
                let sub_left2 = rd::<usize>(r8);
                if rd::<u8>(sub_left2 + 0x19) == 0 {
                    wr::<usize>(sub_left2 + 0x08, gp3);
                }
                let gp_parent = rd::<usize>(gp3 + 0x08);
                wr::<usize>(r8 + 0x08, gp_parent);
                let head = rd::<usize>(t);
                if gp3 == rd::<usize>(head + 0x08) {
                    wr::<usize>(head + 0x08, r8);
                } else {
                    let gpp = rd::<usize>(gp3 + 0x08);
                    if gp3 == rd::<usize>(gpp) {
                        wr::<usize>(gpp, r8);
                    } else {
                        wr::<usize>(gpp + 0x10, r8);
                    }
                }
                wr::<usize>(r8, gp3);
                wr::<usize>(gp3 + 0x08, r8);
            }
        }
    }
    let root = rd::<usize>(r11 + 0x08);
    wr::<u8>(root + 0x18, 1);
    new_node
}

/// Spin until the upper 32 bits of `*lock` are zero, then atomically add
/// `delta` (`r8`) via `lock cmpxchg` and return the previous 64-bit value.
///
/// Original: 0x141379040 (119 bytes).
pub unsafe extern "win64" fn rwlock_wait_and_add(
    lock: *mut u64,
    _unused: usize,
    delta: u64,
) -> u64 {
    let l = lock as usize;
    let mut cur = exe::rd::<u64>(l);
    loop {
        let mut spins = 0u32;
        while (cur & 0xffff_ffff_0000_0000) != 0 {
            #[cfg(target_arch = "x86_64")]
            std::arch::asm!("pause", options(nomem, nostack, preserves_flags));
            #[cfg(not(target_arch = "x86_64"))]
            std::hint::spin_loop();
            spins = spins.wrapping_add(1);
            if spins == 0x2710 {
                spins = 0;
                exe::call_iat!(THRD_YIELD_IAT, ());
            }
            cur = exe::rd::<u64>(l);
        }
        let next = cur.wrapping_add(delta);
        #[cfg(target_arch = "x86_64")]
        {
            let mut prev = cur;
            let ok: u8;
            std::arch::asm!(
                "lock cmpxchg qword ptr [{p}], {n}",
                "sete {ok}",
                p = in(reg) l,
                n = in(reg) next,
                inout("rax") prev,
                ok = out(reg_byte) ok,
                options(nostack),
            );
            if ok != 0 {
                return cur;
            }
            cur = prev;
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            match (*(l as *const std::sync::atomic::AtomicU64)).compare_exchange(
                cur,
                next,
                std::sync::atomic::Ordering::SeqCst,
                std::sync::atomic::Ordering::SeqCst,
            ) {
                Ok(_) => return cur,
                Err(prev) => cur = prev,
            }
        }
    }
}

/// Acquire exclusive write ownership on a 16-byte recursive RW spinlock
/// `{ state: u64, writer_tid: u64 }`.
///
/// Original: 0x141378fb0 (131 bytes).
pub unsafe extern "win64" fn rwlock_acquire_write(this: *mut u8) {
    let t = this as usize;
    let tid = exe::call_iat!(GET_CURRENT_THREAD_ID_IAT, () -> u32) as u64;
    if exe::rd::<u64>(t + 0x08) == tid {
        let inc: u64 = 0x1_0000_0000;
        #[cfg(target_arch = "x86_64")]
        std::arch::asm!(
            "lock add qword ptr [{p}], {v}",
            p = in(reg) t,
            v = in(reg) inc,
            options(nostack),
        );
        #[cfg(not(target_arch = "x86_64"))]
        (*(t as *const std::sync::atomic::AtomicU64))
            .fetch_add(inc, std::sync::atomic::Ordering::SeqCst);
        return;
    }
    let mut spins = 0u32;
    let old = exe::call_va!(
        RWLOCK_WAIT_AND_ADD_VA,
        (t as *mut u64, 0usize, 0x1_0000_0000u64) -> u64
    );
    if (old as u32) != 0 {
        loop {
            #[cfg(target_arch = "x86_64")]
            std::arch::asm!("pause", options(nomem, nostack, preserves_flags));
            #[cfg(not(target_arch = "x86_64"))]
            std::hint::spin_loop();
            spins = spins.wrapping_add(1);
            if spins == 0x2710 {
                spins = 0;
                exe::call_iat!(THRD_YIELD_IAT, ());
            }
            if (exe::rd::<u64>(t) as u32) == 0 {
                break;
            }
        }
    }
    exe::wr::<u64>(t + 0x08, tid);
}

/// `std::_Hash::find(out, &key)` for an 8-byte key hashed with FNV-1a (64-bit).
///
/// Original: 0x1413790c0 (206 bytes).
pub unsafe extern "win64" fn hash_map_find_u64(
    this: *const u8,
    out: *mut *mut u8,
    key_ptr: *const u64,
) -> *mut *mut u8 {
    use exe::{rd, wr};
    let t = this as usize;
    let k = key_ptr as usize;
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    let mask = rd::<u64>(t + 0x30);
    let mut i = 0usize;
    while i < 8 {
        h ^= rd::<u8>(k + i) as u64;
        h = h.wrapping_mul(0x0100_0000_01b3);
        i += 1;
    }
    let bucket = rd::<usize>(t + 0x18).wrapping_add(((h & mask) as usize) << 4);
    let head = rd::<usize>(t + 0x08);
    let mut cur = rd::<usize>(bucket + 0x08);
    let found = if cur == head {
        0
    } else {
        let key = rd::<u64>(k);
        let first = rd::<usize>(bucket);
        loop {
            if rd::<u64>(cur + 0x10) == key {
                break cur;
            }
            if cur == first {
                break 0;
            }
            cur = rd::<usize>(cur + 0x08);
        }
    };
    let res = if found == 0 { head } else { found };
    wr::<usize>(out as usize, res);
    out
}

/// ECS `ComponentTupleIterator<...>` constructor (173..333 bytes, 151 functions across `N = 6..12`).
///
/// Stores `bucket_idx` (`u16`) at `+0x00`, `base_ptr` at `+0x08`, `ctx` at `+0x10`,
/// and `count` at `+0x18`, invokes `CALLEE_VA(&mut offsets, _, ctx + 0x40, bucket_idx as u32)`
/// to populate `N` component byte offsets, converts non-zero offsets to pointers
/// (`base_ptr + off`) at `+0x20..+0x20 + 8*N`, and returns `this`.
pub unsafe extern "win64" fn ecs_tuple_iter_ctor<const N: usize, const CALLEE_VA: u64>(
    this: *mut u8,
    bucket_idx: u16,
    base_ptr: usize,
    count: usize,
    ctx: usize,
) -> *mut u8 {
    use exe::wr;
    let t = this as usize;
    wr::<usize>(t + 0x08, base_ptr);
    wr::<usize>(t + 0x10, ctx);
    wr::<usize>(t + 0x18, count);
    wr::<u16>(t, bucket_idx);
    let mut offsets = [0usize; 16];
    exe::call_va!(
        CALLEE_VA,
        (
            offsets.as_mut_ptr(),
            bucket_idx as usize,
            ctx.wrapping_add(0x40),
            bucket_idx as u32
        ) -> *mut usize
    );
    let mut k = 0usize;
    while k < N {
        let off = offsets[k];
        let ptr = if off != 0 { off.wrapping_add(base_ptr) } else { 0 };
        wr::<usize>(t + 0x20 + 8 * k, ptr);
        k += 1;
    }
    this
}

/// ECS `ComponentOuterIterator<...>` component pointer loader (131..312 bytes, 25 functions across `N = 5..11`).
///
/// If `this.cur (+0x10) != this.end (+0x18)`, reads `bucket_idx = *(u16*)this.cur`,
/// calls `CALLEE_VA(&mut offsets, _, this.ctx (+0x00), bucket_idx as u32)`, and writes
/// `if off != 0 { this.base_ptr (+0x20) + off } else { 0 }` into `+0x38 + 8*k` for `k in 0..N`.
pub unsafe extern "win64" fn ecs_outer_iter_load_components<const N: usize, const CALLEE_VA: u64>(
    this: *mut u8,
) {
    use exe::{rd, wr};
    let t = this as usize;
    let cur = rd::<usize>(t + 0x10);
    if cur == rd::<usize>(t + 0x18) {
        return;
    }
    let ctx = rd::<usize>(t);
    let bucket_idx = rd::<u16>(cur) as u32;
    let mut offsets = [0usize; 16];
    exe::call_va!(
        CALLEE_VA,
        (offsets.as_mut_ptr(), 0usize, ctx, bucket_idx) -> *mut usize
    );
    let base_ptr = rd::<usize>(t + 0x20);
    let mut k = 0usize;
    while k < N {
        let off = offsets[k];
        let ptr = if off != 0 { off.wrapping_add(base_ptr) } else { 0 };
        wr::<usize>(t + 0x38 + 8 * k, ptr);
        k += 1;
    }
}

/// ECS `ComponentOuterIterator<...>::operator++` (158 bytes, 9 functions).
///
/// Increments `this.elem_idx (+0x30)`; if `< this.elem_count (+0x28)`, returns `this`.
/// Otherwise advances `this.cur (+0x10)` by 2 bytes through `this.end (+0x18)` until
/// finding a non-empty bucket in `this.span_table (+0x08)`, reloads `base_ptr (+0x20)`,
/// `elem_count (+0x28)`, `elem_idx = 0 (+0x30)` from `this.ctx (+0x00)`, and invokes
/// `CALLEE_VA(this)` to refresh component pointers.
pub unsafe extern "win64" fn ecs_outer_iter_next<const CALLEE_VA: u64>(this: *mut u8) -> *mut u8 {
    use exe::{rd, wr};
    let t = this as usize;
    let next_idx = rd::<usize>(t + 0x30).wrapping_add(1);
    wr::<usize>(t + 0x30, next_idx);
    if next_idx < rd::<usize>(t + 0x28) {
        return this;
    }
    let mut cur = rd::<usize>(t + 0x10).wrapping_add(2);
    wr::<usize>(t + 0x10, cur);
    let end = rd::<usize>(t + 0x18);
    while cur != end {
        let table = rd::<usize>(t + 0x08);
        let idx = rd::<u16>(cur) as usize;
        if rd::<u64>(table + 0x48 + 8 * idx) != 0 && rd::<u32>(table + 0x10048 + 4 * idx) != 0 {
            break;
        }
        cur = cur.wrapping_add(2);
        wr::<usize>(t + 0x10, cur);
    }
    let cur_final = rd::<usize>(t + 0x10);
    if cur_final != end {
        let idx = rd::<u16>(cur_final) as usize;
        let ctx = rd::<usize>(t);
        let ptr = rd::<u64>(ctx + 0x48 + 8 * idx);
        wr::<u64>(t + 0x20, ptr);
        let cnt = if rd::<u64>(ctx + 0x48 + 8 * idx) != 0 {
            rd::<u32>(ctx + 0x10048 + 4 * idx) as u64
        } else {
            0
        };
        wr::<u64>(t + 0x28, cnt);
        wr::<u64>(t + 0x30, 0);
    }
    exe::call_va!(CALLEE_VA, (this));
    this
}

/// Conditional span/flag dispatcher (55 and 66 bytes, 152 functions across `0x1402c5c30` / `0x1406a54a0` families).
///
/// If `*(u8*)(this + 0x20) != 0` or `*(u32*)(span + 0x08) != 0`, invokes
/// `CALLEE_VA(*(usize*)this, span, r9, *(usize*)span, *(u32*)(span + 0x08) as usize, 0, *(u8*)(this + 0x20))`.
pub unsafe extern "win64" fn span_flag_dispatch<const CALLEE_VA: u64>(
    this: *const u8,
    span: *const u8,
    _r8: usize,
    r9: usize,
) {
    use exe::rd;
    let t = this as usize;
    let s = span as usize;
    let flag = rd::<u8>(t + 0x20);
    let count = rd::<u32>(s + 0x08);
    if flag != 0 || count != 0 {
        exe::call_va!(
            CALLEE_VA,
            (
                rd::<usize>(t),
                span,
                r9,
                rd::<usize>(s),
                count as usize,
                0usize,
                flag
            )
        );
    }
}

/// Conditional pointer-release destructor (33 bytes, 39 functions in `0x140379b20` family).
///
/// Reads `p = *(usize*)(this + 0x08)`; if `p != 0` and `*(u8*)(this + 0x10) != 0`,
/// calls `CALLEE_VA(p)`.
pub unsafe extern "win64" fn cond_release_dtor<const CALLEE_VA: u64>(this: *mut u8) {
    use exe::rd;
    let t = this as usize;
    let p = rd::<usize>(t + 0x08);
    if p != 0 && rd::<u8>(t + 0x10) != 0 {
        exe::call_va!(CALLEE_VA, (p as *mut u8));
    }
}

/// `std::_Tree::_Copy` Red-Black Tree deep-copy helper (164 bytes, 30 functions in `0x14024ed60` family).
///
/// Calls `CALLEE_VA(this, other.head->parent, this.head)` to clone the node subtree,
/// stores the new root in `this.head->parent (+0x08)` and copies `this.size (+0x08)`,
/// then updates `this.head->left (+0x00)` (minimum node) and `this.head->right (+0x10)` (maximum node).
pub unsafe extern "win64" fn rb_tree_copy<const CALLEE_VA: u64>(
    this: *mut u8,
    other: *const u8,
) {
    use exe::{rd, wr};
    let t = this as usize;
    let o = other as usize;
    let my_head0 = rd::<usize>(t);
    let other_head = rd::<usize>(o);
    let other_root = rd::<usize>(other_head + 0x08);
    let new_root = exe::call_va!(CALLEE_VA, (this, other_root, my_head0) -> usize);
    let head1 = rd::<usize>(t);
    wr::<usize>(head1 + 0x08, new_root);
    let head2 = rd::<usize>(t);
    wr::<u64>(t + 0x08, rd::<u64>(o + 0x08));
    let mut min_node = rd::<usize>(head2 + 0x08);
    if rd::<u8>(min_node + 0x19) != 0 {
        wr::<usize>(head2, head2);
        let head3 = rd::<usize>(t);
        wr::<usize>(head3 + 0x10, head3);
        return;
    }
    let mut left = rd::<usize>(min_node);
    while rd::<u8>(left + 0x19) == 0 {
        min_node = left;
        left = rd::<usize>(left);
    }
    wr::<usize>(head2, min_node);
    let head3 = rd::<usize>(t);
    let mut max_node = rd::<usize>(head3 + 0x08);
    let mut right = rd::<usize>(max_node + 0x10);
    while rd::<u8>(right + 0x19) == 0 {
        max_node = right;
        right = rd::<usize>(right + 0x10);
    }
    wr::<usize>(head3 + 0x10, max_node);
}

/// 12-byte entity-handle thunk variant A (46 bytes, 130 functions in `0x14021c410` family).
///
/// Copies the 12-byte handle at `rdx` (`u64` + `u32`) to stack and invokes
/// `CALLEE_VA(*(usize*)r8, &handle_copy, rcx)`.
pub unsafe extern "win64" fn entity_handle_thunk_a<const CALLEE_VA: u64>(
    rcx: usize,
    rdx: *const u8,
    r8: *const usize,
) -> usize {
    use exe::{rd, wr};
    let ctx = rd::<usize>(r8 as usize);
    let mut copy = [0u8; 16];
    let c = copy.as_mut_ptr() as usize;
    wr::<u64>(c, rd::<u64>(rdx as usize));
    wr::<u32>(c + 0x08, rd::<u32>(rdx as usize + 0x08));
    exe::call_va!(CALLEE_VA, (ctx, copy.as_ptr(), rcx) -> usize)
}

/// 12-byte entity-handle thunk variant B (36 bytes, 98 functions in `0x140210440` family).
///
/// Copies the 12-byte handle at `rdx` (`u64` + `u32`) to stack and invokes
/// `CALLEE_VA(rcx, &handle_copy, r8, r9)`.
pub unsafe extern "win64" fn entity_handle_thunk_b<const CALLEE_VA: u64>(
    rcx: usize,
    rdx: *const u8,
    r8: usize,
    r9: usize,
) -> usize {
    use exe::{rd, wr};
    let mut copy = [0u8; 16];
    let c = copy.as_mut_ptr() as usize;
    wr::<u64>(c, rd::<u64>(rdx as usize));
    wr::<u32>(c + 0x08, rd::<u32>(rdx as usize + 0x08));
    exe::call_va!(CALLEE_VA, (rcx, copy.as_ptr(), r8, r9) -> usize)
}

/// 12-byte entity-handle thunk variant C (40 bytes, 50 functions in `0x14023df80` family).
///
/// Copies the 12-byte handle at `rdx` (`u64` + `u32`) to stack and invokes
/// `CALLEE_VA(*(usize*)r8, &handle_copy, r8, r9)`.
pub unsafe extern "win64" fn entity_handle_thunk_c<const CALLEE_VA: u64>(
    _rcx: usize,
    rdx: *const u8,
    r8: *const usize,
    r9: usize,
) -> usize {
    use exe::{rd, wr};
    let mut copy = [0u8; 16];
    let c = copy.as_mut_ptr() as usize;
    wr::<u64>(c, rd::<u64>(rdx as usize));
    wr::<u32>(c + 0x08, rd::<u32>(rdx as usize + 0x08));
    let ctx = rd::<usize>(r8 as usize);
    exe::call_va!(CALLEE_VA, (ctx, copy.as_ptr(), r8, r9) -> usize)
}

/// Renderer preset setting copy constructor / assignment helper (44 bytes, 62 functions in `0x141dda2a0` family).
///
/// Copies `u8` at `+0x08` and `u32` at `+0x10` from `src` to `this`, calls
/// `CALLEE_VA(this + 0x18, src + 0x18)`, and returns `this`.
pub unsafe extern "win64" fn preset_setting_copy_ctor<const CALLEE_VA: u64>(
    this: *mut u8,
    src: *const u8,
) -> *mut u8 {
    use exe::{rd, wr};
    let t = this as usize;
    let s = src as usize;
    wr::<u8>(t + 0x08, rd::<u8>(s + 0x08));
    wr::<u32>(t + 0x10, rd::<u32>(s + 0x10));
    exe::call_va!(CALLEE_VA, ((t + 0x18) as *mut u8, (s + 0x18) as *const u8));
    this
}

/// Renderer preset setting `vfunc_5` copy-to helper (29 bytes, 31 functions in `0x141debd00` family).
///
/// Copies `u8` at `+0x08` and `u32` at `+0x10` from `this` to `dst`, and tail-calls
/// `CALLEE_VA(dst + 0x18, this + 0x18)`.
pub unsafe extern "win64" fn preset_setting_copy_to<const CALLEE_VA: u64>(
    this: *const u8,
    dst: *mut u8,
) -> usize {
    use exe::{rd, wr};
    let t = this as usize;
    let d = dst as usize;
    wr::<u8>(d + 0x08, rd::<u8>(t + 0x08));
    wr::<u32>(d + 0x10, rd::<u32>(t + 0x10));
    exe::call_va!(CALLEE_VA, ((d + 0x18) as *mut u8, (t + 0x18) as *const u8) -> usize)
}

const FREE_ARRAY_VA: u64 = 0x1_4244_2c80;

/// Destructor for an `r::Vector<T>` that first calls `CLEAR_VA(this)` to destroy
/// elements, then releases the backing buffer via [`region_free`] unless bit 30
/// (`0x4000_0000`, non-owning/inline storage) is set in `cap_flags` (`+0x0c`),
/// masks `cap_flags &= 0xc000_0000`, and zeros `ptr` (`+0x00`).
///
/// Originals: 46 functions of 164 bytes (371 call sites, e.g. `0x1401ffa20`).
pub unsafe extern "win64" fn vec_dtor_with_clear<const CLEAR_VA: u64>(this: *mut u8) {
    use exe::{rd, wr};
    exe::call_va!(CLEAR_VA, (this));
    let h = this as usize;
    if rd::<u32>(h + 0x0c) & 0x4000_0000 == 0 {
        let p = rd::<usize>(h) as *mut u8;
        if !p.is_null() {
            region_free(p);
        }
        wr::<u32>(h + 0x0c, rd::<u32>(h + 0x0c) & 0xc000_0000);
        wr::<usize>(h, 0);
    }
}

/// `std::vector::_Calculate_growth` geometric growth helper with minimum capacity `8`
/// and maximum capacity `0x3fff_ffff`.
///
/// Originals: 49 byte-identical functions of 36 bytes (e.g. `0x141a935a0`).
pub unsafe extern "win64" fn std_vec_calculate_growth_8(old_cap: usize) -> u64 {
    let grown = old_cap.wrapping_add(old_cap >> 1);
    if grown < 8 {
        8
    } else if grown > 0x3fff_ffff {
        0x3fff_ffff
    } else {
        grown as u32 as u64
    }
}

/// Move constructor for a 16-byte POD vector `{ ptr: u64, len: u32, cap_flags: u32 }`
/// (`0x140235980` family, 18 byte-identical functions, 39 bytes).
pub unsafe extern "win64" fn pod_vec_move_ctor(this: *mut u8, other: *mut u8) -> *mut u8 {
    use exe::{rd, wr};
    let t = this as usize;
    let o = other as usize;
    wr::<u64>(t + 0x08, 0);
    wr::<u64>(t, 0);
    wr::<u64>(t, rd::<u64>(o));
    wr::<u32>(t + 0x08, rd::<u32>(o + 0x08));
    wr::<u32>(t + 0x0c, rd::<u32>(o + 0x0c));
    wr::<u64>(o, 0);
    wr::<u64>(o + 0x08, 0);
    this
}

/// `std::_Tree_unchecked_const_iterator::operator++` Red-Black Tree in-order successor step
/// (`0x1403b31f0` family, 8 byte-identical functions, 102 bytes).
pub unsafe extern "win64" fn rb_tree_iter_inc(it: *mut usize) -> *mut usize {
    use exe::{rd, wr};
    let it_addr = it as usize;
    let mut cur = rd::<usize>(it_addr);
    let mut right = rd::<usize>(cur + 0x10);
    if rd::<u8>(right + 0x19) != 0 {
        let mut parent = rd::<usize>(cur + 0x08);
        while rd::<u8>(parent + 0x19) == 0 && cur == rd::<usize>(parent + 0x10) {
            wr::<usize>(it_addr, parent);
            cur = parent;
            parent = rd::<usize>(parent + 0x08);
        }
        wr::<usize>(it_addr, parent);
    } else {
        let mut left = rd::<usize>(right);
        while rd::<u8>(left + 0x19) == 0 {
            right = left;
            left = rd::<usize>(left);
        }
        wr::<usize>(it_addr, right);
    }
    it
}

/// `ClassTypeInfoImplementation::vfunc_10` single-object destructor + free helper
/// (`0x1418e5e20` family, 53 functions, 35 bytes).
pub unsafe extern "win64" fn type_info_delete_dtor<const DTOR_VA: u64>(
    _this: *const u8,
    p: *mut u8,
) {
    if !p.is_null() {
        exe::call_va!(DTOR_VA, (p));
        exe::call_va!(REGION_FREE_PTR_VA, (p));
    }
}

/// `ClassTypeInfoImplementation::vfunc_11` array destructor + `freeArray` helper
/// (`0x1418e5dc0` family, 45 functions, 78 bytes).
pub unsafe extern "win64" fn type_info_delete_array_dtor<const DTOR_VA: u64>(
    _this: *const u8,
    p: *mut u8,
) {
    use exe::rd;
    if p.is_null() {
        return;
    }
    let base = p as usize;
    let mut i = 0usize;
    if rd::<u32>(base.wrapping_sub(0x08)) > 0 {
        loop {
            let elem_size = rd::<usize>(base.wrapping_sub(0x10));
            let elem = base.wrapping_add(i.wrapping_mul(elem_size)) as *mut u8;
            exe::call_va!(DTOR_VA, (elem));
            let count = rd::<u32>(base.wrapping_sub(0x08)) as usize;
            i += 1;
            if i >= count {
                break;
            }
        }
    }
    exe::call_va!(FREE_ARRAY_VA, (p));
}

/// Destroy member subobject at `this + SUB_OFF` via `DTOR_VA` and restore base
/// vtable `VTBL_VA` (`0x1418531d0` family, 145 functions, 34 bytes).
pub unsafe extern "win64" fn subobj_dtor_set_vtable<
    const SUB_OFF: usize,
    const DTOR_VA: u64,
    const VTBL_VA: u64,
>(
    this: *mut u8,
) {
    let t = this as usize;
    exe::call_va!(DTOR_VA, (t.wrapping_add(SUB_OFF) as *mut u8));
    exe::wr::<usize>(t, exe::ptr::<u8>(VTBL_VA) as usize);
}

/// Scalar deleting destructor that destroys member subobject at `this + SUB_OFF`
/// via `DTOR_VA`, restores base vtable `VTBL_VA`, and conditionally calls
/// `j_j_free(this, SIZE)` (`0x1401f4bc0` family, 177 functions, 66 bytes).
pub unsafe extern "win64" fn subobj_scalar_deleting_dtor<
    const SUB_OFF: usize,
    const DTOR_VA: u64,
    const VTBL_VA: u64,
    const SIZE: usize,
>(
    this: *mut u8,
    flags: u8,
) -> *mut u8 {
    let t = this as usize;
    exe::call_va!(DTOR_VA, (t.wrapping_add(SUB_OFF) as *mut u8));
    exe::wr::<usize>(t, exe::ptr::<u8>(VTBL_VA) as usize);
    if (flags & 1) != 0 {
        exe::call_va!(J_FREE_SIZED_VA, (this, SIZE));
    }
    this
}

/// Call `vfunc_0(elem, 0)` on `count` contiguous elements of `STRIDE` bytes starting
/// at `base` (`0x141876670` family, 136 functions, 52 bytes).
pub unsafe extern "win64" fn array_vfunc0_dtor<const STRIDE: usize>(base: *mut u8, count: usize) {
    use exe::rd;
    if count == 0 {
        return;
    }
    let mut cur = base as usize;
    let mut rem = count;
    while rem != 0 {
        let vtbl = rd::<usize>(cur);
        let dtor_va = rd::<u64>(vtbl);
        exe::call_va!(dtor_va, (cur as *mut u8, 0u32));
        cur = cur.wrapping_add(STRIDE);
        rem -= 1;
    }
}

/// `ClassTypeInfoImplementation::vfunc_10` / `DefaultLoaderCallback::vfunc_1`:
/// if `p` is non-null, calls `p->vfunc_0(p, 0)` followed by `r::mem::free(p)`
/// (`0x1413594b0` family, 37 bytes).
pub unsafe extern "win64" fn type_info_vfunc0_delete_dtor(_this: *const u8, p: *mut u8) {
    use exe::rd;
    if !p.is_null() {
        let vtbl = rd::<usize>(p as usize);
        let dtor_va = rd::<u64>(vtbl);
        exe::call_va!(dtor_va, (p, 0u32));
        exe::call_va!(REGION_FREE_PTR_VA, (p));
    }
}

/// `ClassTypeInfoImplementation::vfunc_11` polymorphic array destructor + `freeArray`
/// (`0x141849ac0` family, 80 bytes).
pub unsafe extern "win64" fn type_info_vfunc0_delete_array_dtor(_this: *const u8, p: *mut u8) {
    use exe::rd;
    if p.is_null() {
        return;
    }
    let base = p as usize;
    let mut i = 0usize;
    if rd::<u32>(base.wrapping_sub(0x08)) > 0 {
        loop {
            let elem_size = rd::<usize>(base.wrapping_sub(0x10));
            let elem = base.wrapping_add(i.wrapping_mul(elem_size));
            let vtbl = rd::<usize>(elem);
            let dtor_va = rd::<u64>(vtbl);
            exe::call_va!(dtor_va, (elem as *mut u8, 0u32));
            let count = rd::<u32>(base.wrapping_sub(0x08)) as usize;
            i += 1;
            if i >= count {
                break;
            }
        }
    }
    exe::call_va!(FREE_ARRAY_VA, (p));
}


pub mod difftests;
pub use difftests::*;
