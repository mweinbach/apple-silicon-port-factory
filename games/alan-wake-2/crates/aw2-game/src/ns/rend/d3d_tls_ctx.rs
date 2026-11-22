//! `d3d` TLS command-context state, vertex layouts, texture format helpers, and descriptor/upload pools (`0x14204db80..0x1420538c0`).
#![allow(static_mut_refs, dead_code)]

use std::sync::atomic::{AtomicU32, Ordering};

use crate::exe::{self, rd, wr};

const IAT_RELEASE_SRW_EXCLUSIVE: u64 = 0x1_4264_5598;
const IAT_ACQUIRE_SRW_EXCLUSIVE: u64 = 0x1_4264_55a8;
const IAT_GET_CURRENT_THREAD_ID: u64 = 0x1_4264_5790;

#[inline(always)]
unsafe fn lock_recursive_srw(lock: usize) {
    let tid = exe::call_iat!(IAT_GET_CURRENT_THREAD_ID, () -> u32);
    if rd::<u32>(lock + 0x08) != tid {
        let _ = exe::call_iat!(IAT_ACQUIRE_SRW_EXCLUSIVE, (lock) -> ());
        wr::<u32>(lock + 0x08, tid);
    }
    wr::<u32>(lock + 0x0c, rd::<u32>(lock + 0x0c).wrapping_add(1));
}

#[inline(always)]
unsafe fn unlock_recursive_srw(lock: usize) {
    let cnt = rd::<u32>(lock + 0x0c).wrapping_sub(1);
    wr::<u32>(lock + 0x0c, cnt);
    if cnt == 0 {
        AtomicU32::from_ptr((lock + 0x08) as *mut u32).swap(u32::MAX, Ordering::SeqCst);
        let _ = exe::call_iat!(IAT_RELEASE_SRW_EXCLUSIVE, (lock) -> ());
    }
}

#[inline(always)]
unsafe fn r_free_pooled_ref_obj(obj: usize) {
    exe::call_va!(0x1_4206_b020, (obj) -> ());
    let chunk_base = (obj.wrapping_sub(1)) & (!0x01ff_ffffusize);
    let is_owner = exe::teb_self() == rd::<usize>(chunk_base + 0x100);
    let page_idx = (obj.wrapping_sub(chunk_base)) >> 16;
    let raw_hdr = chunk_base + 0x108 + page_idx * 0x50;
    let hdr = raw_hdr.wrapping_sub(rd::<u32>(raw_hdr + 0x04) as usize);
    if is_owner {
        if rd::<u8>(hdr + 0x0e) == 0 {
            let head = rd::<usize>(hdr + 0x20);
            wr::<usize>(obj, head);
            wr::<usize>(hdr + 0x20, obj);
            let prev = rd::<u32>(hdr + 0x18);
            wr::<u32>(hdr + 0x18, prev.wrapping_sub(1));
            if prev == 1 {
                exe::call_va!(0x1_4253_71a0, (hdr) -> ());
            }
            return;
        }
        exe::call_va!(0x1_4253_3200, (chunk_base, hdr, 1u8, obj) -> ());
    } else {
        exe::call_va!(0x1_4253_3200, (chunk_base, hdr, 0u8, obj) -> ());
    }
}

// ============================================================================
// 1. TLS state unbind & VertexLayout registration / queries (`0x14204db80..0x14204e060`)
// ============================================================================

/// Unbind render target `target` (`rcx`) from the current thread's D3D TLS state cache (`tls0 + 0x467e0`) if bound at `+0xb18`.
///
/// Original: `0x14204db80` (95 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tls_ctx_unbind_target(target: usize) {
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, () -> ());
    }
    let ctx = tls0 + 0x467e0;
    if rd::<usize>(ctx + 0xb18) == target {
        wr::<usize>(ctx + 0xb18, 0);
        wr::<usize>(ctx + 0xb20, 0);
        wr::<usize>(ctx + 0x48, 0);
        wr::<u8>(ctx + 0xa80, 0);
    }
}

/// Register or deduplicate a 104-byte (`0x68`) vertex layout `layout` (`rdx`) in `0x143a49130` / `0x14397db80`, writing its index to `*out_id` (`rcx`).
///
/// Original: `0x14204dbf0` (198 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_register_vertex_layout(out_id: usize, layout: usize) -> usize {
    wr::<u32>(out_id, u32::MAX);
    let byte_len = (rd::<u8>(layout + 0x40) as usize) << 2;
    let hash = exe::call_va!(0x1_424b_16a0, (layout, byte_len) -> u64);
    let hashes_base = exe::ptr::<u64>(0x1_4397_db80) as usize;
    let count_ptr = exe::ptr::<u32>(0x1_43a2_afec) as *mut u32;
    let lock_cnt = AtomicU32::from_ptr(exe::ptr::<u32>(0x1_43a2_afe8) as *mut u32);
    loop {
        // Acquire: pairs with the `lock inc` publishing hashes[] (x86 loads are acquire).
        let count = crate::simd_ops::rd_acq_u32(count_ptr as usize);
        let mut idx = 0u32;
        while idx < count {
            if rd::<u64>(hashes_base + (idx as usize) * 8) == hash {
                wr::<u32>(out_id, idx);
                return out_id;
            }
            idx += 1;
        }
        let next = count.wrapping_add(1);
        if lock_cnt
            .compare_exchange(count, next, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            wr::<u32>(out_id, next);
            let dst = (exe::ptr::<u8>(0x1_43a4_9130) as usize) + (next as usize) * 0x68;
            for k in 0..13usize {
                wr::<u64>(dst + k * 8, rd::<u64>(layout + k * 8));
            }
            let slot = rd::<u32>(out_id) as usize;
            wr::<u64>(hashes_base + slot * 8, hash);
            AtomicU32::from_ptr(count_ptr).fetch_add(1, Ordering::SeqCst);
            return out_id;
        }
    }
}

/// Deserialize a `d3d::VertexLayout` (`rdx`) from stream `stream` (`rcx`) and return `stream`.
///
/// Original: `0x14204dcc0` (240 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_vertex_layout_deserialize(stream: usize, layout: usize) -> usize {
    wr::<u64>(layout + 0x48, 0);
    wr::<u64>(layout + 0x50, 0);
    wr::<u64>(layout + 0x58, 0);
    wr::<u64>(layout + 0x60, 0);
    let mut dummy_u32: u32 = 0;
    let vtbl0 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vtbl0 + 0x48), (stream, &mut dummy_u32 as *mut u32 as usize) -> ());
    let vtbl1 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vtbl1 + 0x70), (stream, layout + 0x40) -> ());
    if rd::<u8>(layout + 0x40) > 0 {
        let sizes_table = exe::ptr::<u8>(0x1_4297_89d0) as usize;
        let mut i: i32 = 0;
        while i < (rd::<u8>(layout + 0x40) as i32) {
            let elem = layout + (i as usize) * 4;
            let vtbl_a = rd::<usize>(stream);
            exe::call_va!(rd::<usize>(vtbl_a + 0x70), (stream, elem) -> ());
            let mut b: u8 = 0;
            let b_ptr = &mut b as *mut u8 as usize;
            let vtbl_b = rd::<usize>(stream);
            exe::call_va!(rd::<usize>(vtbl_b + 0x70), (stream, b_ptr) -> ());
            wr::<u8>(elem + 1, rd::<u8>(b_ptr));
            let vtbl_c = rd::<usize>(stream);
            exe::call_va!(rd::<usize>(vtbl_c + 0x70), (stream, b_ptr) -> ());
            wr::<u8>(elem + 2, rd::<u8>(b_ptr));
            let vtbl_d = rd::<usize>(stream);
            exe::call_va!(rd::<usize>(vtbl_d + 0x70), (stream, b_ptr) -> ());
            wr::<u8>(elem + 3, rd::<u8>(b_ptr));

            let slot = rd::<u8>(elem) as usize;
            let fmt = rd::<u8>(elem + 1) as usize;
            let add_bytes = rd::<u8>(sizes_table + fmt) as u64;
            let stride_addr = layout + 0x48 + slot * 8;
            wr::<u64>(stride_addr, rd::<u64>(stride_addr).wrapping_add(add_bytes));
            i += 1;
        }
    }
    stream
}

/// Map vertex semantic index `sem` (`cl`, `0..=7`) to its HLSL semantic name string pointer.
///
/// Original: `0x14204ddc0` (128 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_vertex_semantic_to_name(sem: u8) -> usize {
    let va: u64 = match sem {
        0 => 0x1_433d_bfd8, // "POSITION"
        1 => 0x1_433d_bf8c, // "NORMAL"
        2 => 0x1_433d_bf80, // "TEXCOORD"
        3 => 0x1_433d_bfa0, // "TANGENT"
        4 => 0x1_433d_bf94, // "COLOR"
        5 => 0x1_433d_bfb8, // "BLENDINDICES"
        6 => 0x1_433d_bfa8, // "BLENDWEIGHT"
        7 => 0x1_433d_bfc8, // "RenderInstance"
        _ => return 0,
    };
    exe::ptr::<u8>(va) as usize
}

/// Return whether vertex layout `layout` (`rcx`) equals the global default layout at `0x14386b9e0`.
///
/// Original: `0x14204de50` (52 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_vertex_layout_is_default(layout: usize) -> bool {
    let def_count = rd::<u8>(exe::ptr::<u8>(0x1_4386_ba20) as usize);
    if rd::<u8>(layout + 0x40) != def_count {
        return false;
    }
    let def_ptr = exe::ptr::<u8>(0x1_4386_b9e0) as usize;
    exe::call_va!(0x1_4183_acda, (layout, def_ptr, 0x40u32) -> i32) == 0
}

/// Compute the 64-bit hash of vertex layout `layout` (`rcx`) via `0x1424b16a0`.
///
/// Original: `0x14204de90` (13 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_vertex_layout_hash(layout: usize) -> u64 {
    let byte_len = (rd::<u8>(layout + 0x40) as usize) << 2;
    exe::call_va!(0x1_424b_16a0, (layout, byte_len) -> u64)
}

/// Find the format byte of the first `slot == 0, semantic == 0` (`POSITION0`) element in `layout` (`rcx`), writing it to `*out_fmt` (`arg5`).
///
/// Original: `0x14204deb0` (62 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_vertex_layout_find_position0_format(
    layout: usize,
    _rdx: usize,
    _r8: usize,
    _r9: usize,
    out_fmt: usize,
) -> bool {
    let count = rd::<u8>(layout + 0x40) as usize;
    let mut match_idx: u32 = 0;
    for i in 0..count {
        let elem = layout + i * 4;
        if rd::<u8>(elem) == 0 && rd::<u8>(elem + 2) == 0 {
            if match_idx == 0 {
                wr::<u8>(out_fmt, rd::<u8>(elem + 1));
                return true;
            }
            match_idx = match_idx.wrapping_add(1);
        }
    }
    false
}

/// Compute the byte offset within `slot` (`dl`) of the `(semantic = r8b, semantic_index = r9d)` element in `layout` (`rcx`), or `u32::MAX` if absent.
///
/// Original: `0x14204df00` (127 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_vertex_layout_get_element_offset(
    layout: usize,
    slot: u8,
    semantic: u8,
    semantic_index: u32,
) -> u32 {
    let count = rd::<u8>(layout + 0x40) as usize;
    let mut byte_offset: u32 = 0;
    let mut cur_sem_idx: u32 = 0;
    let sizes_table = exe::ptr::<u8>(0x1_4297_89d0) as usize;
    for i in 0..count {
        let elem = layout + i * 4;
        if rd::<u8>(elem) == slot {
            if rd::<u8>(elem + 2) == semantic {
                if cur_sem_idx == semantic_index {
                    return byte_offset;
                }
                cur_sem_idx = cur_sem_idx.wrapping_add(1);
            }
            let fmt = rd::<u8>(elem + 1) as usize;
            byte_offset = byte_offset.wrapping_add(rd::<u8>(sizes_table + fmt) as u32);
        }
    }
    u32::MAX
}

/// Append all vertex elements from `src` (`rdx`) into `dst` (`rcx`) with `step_rate = 0`, updating slot strides.
///
/// Original: `0x14204df90` (126 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_vertex_layout_append_layout(dst: usize, src: usize) {
    if rd::<u8>(src + 0x40) == 0 {
        return;
    }
    let sizes_table = exe::ptr::<u8>(0x1_4297_89d0) as usize;
    let mut i: i32 = 0;
    while i < (rd::<u8>(src + 0x40) as i32) {
        let src_elem = src + (i as usize) * 4;
        let fmt = rd::<u8>(src_elem + 1);
        let slot = rd::<u8>(src_elem);
        let sem = rd::<u8>(src_elem + 2);
        let dst_idx = rd::<u8>(dst + 0x40) as usize;
        let dst_elem = dst + dst_idx * 4;
        wr::<u8>(dst_elem + 2, sem);
        wr::<u8>(dst_elem, slot);
        wr::<u8>(dst_elem + 1, fmt);
        wr::<u8>(dst_elem + 3, 0);
        wr::<u8>(dst + 0x40, (dst_idx as u8).wrapping_add(1));
        let add_bytes = rd::<u8>(sizes_table + (fmt as usize)) as u64;
        let stride_addr = dst + 0x48 + (slot as usize) * 8;
        wr::<u64>(stride_addr, rd::<u64>(stride_addr).wrapping_add(add_bytes));
        i += 1;
    }
}

/// Append a single vertex element `(slot, fmt, sem, step_rate)` to `layout` (`rcx`) and update slot stride.
///
/// Original: `0x14204e020` (60 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_vertex_layout_add_element(
    layout: usize,
    slot: u8,
    fmt: u8,
    sem: u8,
    step_rate: u8,
) {
    let idx = rd::<u8>(layout + 0x40) as usize;
    let elem = layout + idx * 4;
    wr::<u8>(elem, slot);
    wr::<u8>(elem + 1, fmt);
    wr::<u8>(elem + 2, sem);
    wr::<u8>(elem + 3, step_rate);
    wr::<u8>(layout + 0x40, (idx as u8).wrapping_add(1));
    let sizes_table = exe::ptr::<u8>(0x1_4297_89d0) as usize;
    let add_bytes = rd::<u8>(sizes_table + (fmt as usize)) as u64;
    let stride_addr = layout + 0x48 + (slot as usize) * 8;
    wr::<u64>(stride_addr, rd::<u64>(stride_addr).wrapping_add(add_bytes));
}

// ============================================================================
// 2. Heap page allocator & Texture format / mip helpers (`0x14204e060..0x14204f450`)
// ============================================================================

/// Return heap page `page_idx` (`dx`) to the free list at `this + 0x88` and cache or release its `ID3D12Heap`.
///
/// Original: `0x14204e060` (249 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_heap_page_mgr_free_page(this: usize, page_idx: u16) {
    let free_vec = this + 0x88;
    let free_len = rd::<u32>(free_vec + 0x08) as usize;
    let free_cap = rd::<u32>(free_vec + 0x0c) as usize;
    if free_len * 2 == free_cap * 2 {
        exe::call_va!(0x1_40fa_5d10, (free_vec, free_len + 1) -> ());
    }
    let idx = rd::<u32>(free_vec + 0x08) as usize;
    wr::<u16>(rd::<usize>(free_vec) + idx * 2, page_idx);
    wr::<u32>(free_vec + 0x08, (idx + 1) as u32);

    let spare_vec = this + 0x78;
    let spare_len = rd::<u32>(spare_vec + 0x08) as usize;
    let page_entry = rd::<usize>(this + 0x68) + (page_idx as usize) * 16;
    if spare_len < 2 {
        let spare_cap = rd::<u32>(spare_vec + 0x0c) as usize;
        if spare_len * 16 == spare_cap * 16 {
            let min_cap = spare_len + 1;
            let grown = spare_cap + (spare_cap >> 1);
            let new_cap = if grown < min_cap { min_cap } else { grown };
            exe::call_va!(0x1_411d_2c10, (spare_vec, new_cap) -> ());
        }
        let cur_len = rd::<u32>(spare_vec + 0x08) as usize;
        let dst = rd::<usize>(spare_vec) + cur_len * 16;
        wr::<u32>(dst, rd::<u32>(page_entry));
        wr::<u64>(dst + 0x08, rd::<u64>(page_entry + 0x08));
        wr::<u64>(page_entry + 0x08, 0);
        wr::<u32>(spare_vec + 0x08, (cur_len + 1) as u32);
    } else {
        let tag = rd::<u32>(page_entry);
        let ptr = rd::<usize>(page_entry + 0x08);
        if ptr != 0 {
            exe::call_va!(0x1_4209_a1e0, (ptr, tag) -> ());
        }
        wr::<usize>(page_entry + 0x08, 0);
        wr::<u32>(page_entry, tag);
    }
}

/// Allocate a heap page of `(size_64k as u64) << 16` bytes (`dx`) from `this` (`rcx`), creating or reusing an `ID3D12Heap`.
///
/// Original: `0x14204e160` (475 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_heap_page_mgr_alloc_page(this: usize, size_64k: u16) -> u16 {
    let free_len = rd::<u32>(this + 0x90) as usize;
    let free_base = rd::<usize>(this + 0x88);
    let pages_vec = this + 0x68;
    let page_idx: u16 = if free_len == 0 {
        let len = rd::<u32>(pages_vec + 0x08) as usize;
        let cap = rd::<u32>(pages_vec + 0x0c) as usize;
        if len * 16 == cap * 16 {
            let min_cap = len + 1;
            let grown = cap + (cap >> 1);
            let new_cap = if grown < min_cap { min_cap } else { grown };
            exe::call_va!(0x1_411d_2c10, (pages_vec, new_cap) -> ());
        }
        let cur_len = rd::<u32>(pages_vec + 0x08) as usize;
        let base = rd::<usize>(pages_vec);
        wr::<u64>(base + cur_len * 16, 0);
        wr::<u64>(base + cur_len * 16 + 0x08, 0);
        wr::<u32>(pages_vec + 0x08, (cur_len + 1) as u32);
        len as u16
    } else {
        let idx = rd::<u16>(free_base + (free_len - 1) * 2);
        wr::<u32>(this + 0x90, (free_len - 1) as u32);
        idx
    };

    let entry = rd::<usize>(pages_vec) + (page_idx as usize) * 16;
    if rd::<usize>(entry + 0x08) == 0 {
        let spare_len = rd::<u32>(this + 0x80) as usize;
        let spare_base = rd::<usize>(this + 0x78);
        let spare_end = spare_base + spare_len * 16;
        if spare_base == spare_end {
            let dev_ctx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
            let flags: u32 = if rd::<u8>(dev_ctx + 0xa0) != 0 {
                0x1084
            } else {
                0x84
            };
            let mut heap_desc = [0u64; 6];
            let desc_ptr = heap_desc.as_mut_ptr() as usize;
            wr::<u64>(desc_ptr, (size_64k as u64) << 16);
            wr::<u64>(desc_ptr + 0x08, 1);
            wr::<u32>(desc_ptr + 0x10, 0);
            wr::<u32>(desc_ptr + 0x14, 1);
            wr::<u32>(desc_ptr + 0x18, 1);
            wr::<u64>(desc_ptr + 0x20, 0);
            wr::<u32>(desc_ptr + 0x28, flags);

            let d3d_dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
            let vtbl = rd::<usize>(d3d_dev);
            let create_heap = rd::<usize>(vtbl + 0xe0);
            let out_ptr = exe::call_va!(0x1_411d_2ad0, (entry) -> usize);
            let riid = exe::ptr::<u8>(0x1_433d_bfe8) as usize;
            let d3d_dev2 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
            let hr = exe::call_va!(create_heap, (d3d_dev2, desc_ptr, riid, out_ptr) -> u32);
            if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2 {
                exe::call_va!(0x1_4205_d410, (0usize, hr) -> ());
            }
        } else {
            wr::<u64>(entry + 0x08, rd::<u64>(spare_end - 0x08));
            wr::<u32>(entry, rd::<u32>(spare_end - 0x10));
            wr::<u64>(spare_end - 0x08, 0);
            let cur_len = rd::<u32>(this + 0x80) as usize;
            let cur_base = rd::<usize>(this + 0x78);
            let old_ptr = rd::<usize>(cur_base + cur_len * 16 - 0x08);
            if old_ptr != 0 {
                let old_tag = rd::<u32>(cur_base + cur_len * 16 - 0x10);
                exe::call_va!(0x1_4209_a1e0, (old_ptr, old_tag) -> ());
            }
            wr::<u32>(this + 0x80, rd::<u32>(this + 0x80).wrapping_sub(1));
        }
    }
    page_idx
}

/// Allocate a sub-range of `byte_size` (`rdx`) bytes from the global heap page manager `*0x14397fb80`.
///
/// Original: `0x14204e3e0` (197 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_heap_page_mgr_alloc_subrange(
    _rcx: usize,
    byte_size: usize,
) -> usize {
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fb80) as usize);
    let _ = exe::call_iat!(IAT_ACQUIRE_SRW_EXCLUSIVE, (mgr) -> ());
    let node = exe::call_va!(0x1_4243_b450, (mgr + 0x98) -> usize);
    if node != 0 {
        wr::<u64>(node, 0);
        wr::<u64>(node + 0x08, 0);
    }
    let pages = ((byte_size.wrapping_add(0xffff) >> 16) as u16) as u32;
    let bucket: u32 = if pages > 1 {
        let v = pages - 1;
        if v != 0 {
            (31 - v.leading_zeros()) + 1
        } else {
            0
        }
    } else {
        0
    };
    let sub = exe::call_va!(0x1_4209_dde0, (mgr + 0x08, bucket) -> usize);
    if sub != 0 {
        wr::<u64>(sub, 0);
        let cnt = rd::<u16>(sub + 0x0a) as u64;
        wr::<u64>(mgr + 0x58, rd::<u64>(mgr + 0x58).wrapping_add(cnt));
        wr::<u64>(mgr + 0x60, rd::<u64>(mgr + 0x60).wrapping_add(1));
    }
    wr::<usize>(node + 0x08, sub);
    let page_idx = rd::<u16>(sub + 0x0c) as usize;
    let page_entry = rd::<usize>(mgr + 0x68) + page_idx * 16;
    wr::<u64>(node, rd::<u64>(page_entry + 0x08));
    let _ = exe::call_iat!(IAT_RELEASE_SRW_EXCLUSIVE, (mgr) -> ());
    node
}

/// Destroy a `d3d::HeapPageManager` (`rcx`).
///
/// Original: `0x14204e4b0` (90 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_heap_page_mgr_dtor(this: usize) {
    exe::call_va!(0x1_4243_b540, (this + 0x98) -> ());
    exe::call_va!(0x1_401d_e830, (this + 0x88) -> ());
    exe::call_va!(0x1_411d_2b40, (this + 0x78) -> ());
    exe::call_va!(0x1_411d_2b40, (this + 0x68) -> ());
    exe::call_va!(0x1_402c_30b0, (this + 0x48) -> ());
    exe::call_va!(0x1_401d_e830, (this + 0x38) -> ());
}

/// Construct a `d3d::HeapPageManager` (`rcx`) and return `this`.
///
/// Original: `0x14204e510` (201 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_heap_page_mgr_ctor(this: usize) -> usize {
    wr::<u64>(this, 0);
    let mut buddy_cfg = [0u64; 6];
    let cfg_ptr = buddy_cfg.as_mut_ptr() as usize;
    wr::<usize>(cfg_ptr, this);
    wr::<usize>(cfg_ptr + 0x08, exe::ptr::<u8>(0x1_4204_e160) as usize);
    wr::<usize>(cfg_ptr + 0x10, this);
    wr::<usize>(cfg_ptr + 0x18, exe::ptr::<u8>(0x1_4204_e060) as usize);
    wr::<u16>(cfg_ptr + 0x20, 0x200);
    wr::<u64>(cfg_ptr + 0x28, 0x100);
    exe::call_va!(0x1_4209_e4d0, (this + 0x08, cfg_ptr) -> ());
    for off in (0x68..0x98usize).step_by(8) {
        wr::<u64>(this + off, 0);
    }
    exe::call_va!(
        0x1_4243_b5f0,
        (this + 0x98, 0usize, 0usize, 0x400u32, 0x10usize, 0x80u32, 0x8usize) -> ()
    );
    this
}

/// Compute the byte size of a single 2D mip level `(width = ecx, height = edx, fmt = r8b)`.
///
/// Original: `0x14204e7f0` (213 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_calc_texture_2d_byte_size(
    width: i32,
    height: i32,
    fmt: i8,
) -> u32 {
    match fmt {
        1 | 2 | 9..=13 | 15..=19 | 21 | 22 | 41 | 42 => (width.wrapping_mul(height) << 2) as u32,
        3..=5 => width.wrapping_mul(height) as u32,
        6..=8 | 20 | 38..=40 | 44 | 45 => (width.wrapping_mul(height) << 1) as u32,
        14 | 23 | 24 | 43 => (width.wrapping_mul(height) << 3) as u32,
        25 | 26 => (width.wrapping_mul(height) << 4) as u32,
        27 => (width.wrapping_mul(height).wrapping_mul(3) << 2) as u32,
        28 | 29 | 33 => {
            let w_aligned = width.wrapping_add(3) & -4;
            let h_aligned = height.wrapping_add(3) & -4;
            let prod = w_aligned.wrapping_mul(h_aligned);
            (prod / 2) as u32
        }
        30..=32 | 34..=37 => {
            let w_aligned = width.wrapping_add(3) & -4;
            let h_aligned = height.wrapping_add(3) & -4;
            w_aligned.wrapping_mul(h_aligned) as u32
        }
        _ => 0,
    }
}

/// Compute the total byte size across `mip_count` (`r9d`) mip levels for a 3D texture `(width = ecx, height = edx, depth = r8d, fmt = arg5)`.
///
/// Original: `0x14204e5e0` (365 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_calc_texture_mipchain_byte_size(
    width: i32,
    height: i32,
    depth: i32,
    mip_count: i32,
    fmt: i8,
) -> u32 {
    if mip_count <= 0 {
        return 0;
    }
    let mut total: u32 = 0;
    for mip in 0..mip_count {
        let w = (width >> mip).max(1);
        let h = (height >> mip).max(1);
        let d = (depth >> mip).max(1) as u32;
        let slice_bytes = d3d_calc_texture_2d_byte_size(w, h, fmt);
        total = total.wrapping_add(slice_bytes.wrapping_mul(d));
    }
    total
}

/// Compute the byte size of a single 3D mip level `(width = ecx, height = edx, depth = r8d, fmt = r9b)`.
///
/// Original: `0x14204e760` (25 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_calc_texture_3d_slice_byte_size(
    width: i32,
    height: i32,
    depth: u32,
    fmt: i8,
) -> u32 {
    d3d_calc_texture_2d_byte_size(width, height, fmt).wrapping_mul(depth)
}

/// Return `(dim + 3) / 4` when `fmt` (`dl`) is a block-compressed format (`28..=37`), or `dim` (`ecx`) otherwise.
///
/// Original: `0x14204e780` (92 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_calc_texture_row_blocks(dim: i32, fmt: i8) -> i32 {
    if ((fmt as i32).wrapping_sub(0x1c) as u32) <= 9 {
        let x = dim.wrapping_add(3);
        let bias = if x < 0 { 3 } else { 0 };
        x.wrapping_add(bias) >> 2
    } else {
        dim
    }
}

/// Return the bytes-per-pixel of uncompressed texture format `fmt` (`cl`), or `0` for block-compressed / unknown formats.
///
/// Original: `0x14204e8d0` (157 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_get_texture_format_bytes_per_pixel(fmt: i8) -> u32 {
    match fmt {
        1 | 2 | 9..=13 | 15..=19 | 21 | 22 | 41 | 42 => 4,
        3..=5 => 1,
        6..=8 | 20 | 38..=40 | 44 | 45 => 2,
        14 | 23 | 24 => 8,
        25 | 26 => 16,
        27 => 12,
        _ => 0,
    }
}

/// Map texture format `fmt` (`cl`, `1..=45`) to its static name string pointer (or `"???"` at `0x1433dc1b0`).
///
/// Original: `0x14204e980` (584 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_texture_format_to_name(fmt: i8) -> usize {
    let va: u64 = match fmt {
        1 => 0x1_433d_c0a0,  // "R24X8"
        2 => 0x1_433d_c0a8,  // "D24S8"
        3 => 0x1_433d_c050,  // "R8U"
        4 => 0x1_433d_c04c,  // "R8I"
        5 => 0x1_433d_c038,  // "R8"
        6 => 0x1_433d_c0d4,  // "R16"
        7 => 0x1_433d_c0ec,  // "R16F"
        8 => 0x1_433d_c0e4,  // "R16U"
        9 => 0x1_433d_c0b8,  // "R32F"
        10 => 0x1_433d_c0b0, // "R32U"
        11 => 0x1_433d_c088, // "R16G16"
        12 => 0x1_433d_c080, // "R16G16F"
        13 => 0x1_433d_c098, // "R16G16U"
        14 => 0x1_433d_c150, // "R32G32F"
        15 => 0x1_433d_c168, // "R8G8B8A8"
        16 => 0x1_433d_c158, // "R8G8B8A8_SRGB"
        17 => 0x1_433d_c178, // "B8G8R8X8"
        18 => 0x1_433d_c188, // "B8G8R8A8"
        19 => 0x1_433d_c1a0, // "B8G8R8A8_SRGB"
        20 => 0x1_433d_c0c0, // "B5G5R5A1"
        21 => 0x1_433d_c060, // "R10G10B10A2"
        22 => 0x1_433d_c0d8, // "R11G11B10F"
        23 => 0x1_433d_c138, // "R16G16B16A16"
        24 => 0x1_433d_c108, // "R16G16B16A16F"
        25 => 0x1_433d_c0f8, // "R32G32B32A32F"
        26 => 0x1_433d_c118, // "R32G32B32A32U"
        27 => 0x1_433d_c128, // "R32G32B32F"
        28 => 0x1_433d_bff8, // "BC1"
        29 => 0x1_433d_c008, // "BC1_SRGB"
        30 => 0x1_433d_c004, // "BC2"
        31 => 0x1_433d_c024, // "BC3"
        32 => 0x1_433d_c018, // "BC3_SRGB"
        33 => 0x1_433d_c02c, // "BC4"
        34 => 0x1_433d_c028, // "BC5"
        35 => 0x1_433d_c034, // "BC6"
        36 => 0x1_433d_c030, // "BC7"
        37 => 0x1_433d_c040, // "BC7_SRGB"
        38 => 0x1_433d_c194, // "R8G8"
        39 => 0x1_433d_c06c, // "R8G8U"
        40 => 0x1_433d_bffc, // "R16I"
        41 => 0x1_433d_c0cc, // "R32I"
        42 => 0x1_433d_c090, // "R16G16I"
        43 => 0x1_433d_c148, // "R32G32U"
        44 => 0x1_433d_c054, // "R16S"
        45 => 0x1_433d_c074, // "R8G8S"
        _ => 0x1_433d_c1b0,  // "???"
    };
    exe::ptr::<u8>(va) as usize
}

/// Return whether texture format `fmt` (`cl`) is block-compressed (`BC1..BC7_SRGB`, `28..=37`).
///
/// Original: `0x14204ebd0` (80 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_is_block_compressed_format(fmt: i8) -> bool {
    ((fmt as i32).wrapping_sub(0x1c) as u32) <= 9
}

/// Downsample an `RGBA8` image `pixels` (`rcx`) of size `(*inout_w, *inout_h)` (`rdx`, `r8`) in-place by 2x box filter.
///
/// Original: `0x14204ec30` (404 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_downsample_rgba8_mip_in_place(
    pixels: usize,
    inout_w: usize,
    inout_h: usize,
) {
    let orig_w = rd::<i32>(inout_w);
    let orig_h = rd::<i32>(inout_h);
    wr::<i32>(inout_w, (orig_w >> 1).max(1));
    wr::<i32>(inout_h, (orig_h >> 1).max(1));
    if orig_h <= 0 {
        return;
    }
    let mut dst = pixels;
    let mut y: i32 = 0;
    let mut row0_base: i32 = 0;
    while y < orig_h {
        if orig_w > 0 {
            let next_y = (y + 1) % orig_h;
            let row1_base = next_y.wrapping_mul(orig_w);
            let mut x: i32 = 0;
            while x < orig_w {
                let next_x = (x + 1) % orig_w;
                let p00 = (pixels as isize + ((row0_base + x) as isize) * 4) as usize;
                let p01 = (pixels as isize + ((row0_base + next_x) as isize) * 4) as usize;
                let p10 = (pixels as isize + ((row1_base + x) as isize) * 4) as usize;
                let p11 = (pixels as isize + ((row1_base + next_x) as isize) * 4) as usize;
                for c in [2usize, 1, 0, 3] {
                    let sum = (rd::<u8>(p00 + c) as u32)
                        + (rd::<u8>(p11 + c) as u32)
                        + (rd::<u8>(p10 + c) as u32)
                        + (rd::<u8>(p01 + c) as u32);
                    wr::<u8>(dst + c, (sum >> 2) as u8);
                }
                dst += 4;
                x += 2;
            }
        }
        y += 2;
        row0_base = row0_base.wrapping_add(orig_w.wrapping_mul(2));
    }
}

/// Compute the mip-chain level count (`floor(log2(max(1, w, h))) + 1`) for a 2D texture `(w = ecx, h = edx)`.
///
/// Original: `0x14204edd0` (43 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_calc_mip_count_2d(w: i32, h: i32) -> u32 {
    let max_dim = 1i32.max(h).max(w);
    let mut mips: u32 = 0;
    let mut cur: i32 = 1;
    if max_dim >= 1 {
        loop {
            mips += 1;
            cur = cur.wrapping_add(cur);
            if cur > max_dim {
                break;
            }
        }
    }
    mips
}

/// Return whether `(w = ecx, h = edx)` are both powers of two in `1..=0x4000`.
///
/// Original: `0x14204ee10` (46 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_is_valid_pow2_2d_size(w: i32, h: i32) -> bool {
    w >= 1
        && h >= 1
        && w <= 0x4000
        && h <= 0x4000
        && (w & w.wrapping_sub(1)) == 0
        && (h & h.wrapping_sub(1)) == 0
}

/// Return whether texture format `fmt` (`cl`) is a floating-point format (`7, 9, 12, 14, 22, 24, 25, 27, 35`).
///
/// Original: `0x14204ee50` (85 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_is_float_texture_format(fmt: i8) -> bool {
    matches!(fmt, 7 | 9 | 12 | 14 | 22 | 24 | 25 | 27 | 35)
}

/// Return whether texture format `fmt` (`cl`) is an sRGB format (`16, 19, 29, 32, 37`).
///
/// Original: `0x14204eeb0` (31 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_is_srgb_texture_format(fmt: u8) -> bool {
    if fmt <= 0x20 && ((0x1_2009_0000u64 >> fmt) & 1) != 0 {
        return true;
    }
    fmt == 0x25
}

/// Return the address of the subresource view pointer at `*(this + 0x30) + 8 * (face_or_slice + 6 * mip * *(this + 0x14))`.
///
/// Original: `0x14204eed0` (24 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_get_texture_subresource_view_slot(
    this: usize,
    face_or_slice: i32,
    mip: i32,
) -> usize {
    let prod = mip.wrapping_mul(rd::<i32>(this + 0x14));
    let idx = face_or_slice.wrapping_add(prod.wrapping_mul(6));
    (rd::<usize>(this + 0x30) as isize + (idx as isize) * 8) as usize
}

// ============================================================================
// Differential tests
// ============================================================================

pub unsafe fn diff_d3d_vertex_semantic_to_name(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u8) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut ran = 0;
    for sem in 0..=9u8 {
        let ro = f_orig(sem);
        let rn = f_new(sem);
        crate::diff_eq!(ro, rn, "semantic_to_name");
        ran += 1;
    }
    Ok(ran)
}

pub unsafe fn diff_d3d_vertex_layout_is_default(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut layout = [0u8; 0x68];
    layout[0x40] = 0xfe;
    let l_ptr = layout.as_ptr() as usize;
    let ro = f_orig(l_ptr);
    let rn = f_new(l_ptr);
    crate::diff_eq!(ro, rn, "is_default_false");
    let def_ptr = exe::ptr::<u8>(0x1_4386_b9e0) as usize;
    let ro2 = f_orig(def_ptr);
    let rn2 = f_new(def_ptr);
    crate::diff_eq!(ro2, rn2, "is_default_true");
    Ok(2)
}

pub unsafe fn diff_d3d_vertex_layout_hash(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut layout = [0u8; 0x68];
    layout[0] = 0;
    layout[1] = 2;
    layout[2] = 0;
    layout[3] = 0;
    layout[0x40] = 1;
    let ptr = layout.as_ptr() as usize;
    let ro = f_orig(ptr);
    let rn = f_new(ptr);
    crate::diff_eq!(ro, rn, "vertex_layout_hash");
    Ok(1)
}

pub unsafe fn diff_d3d_vertex_layout_find_position0_format(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut layout = [0u8; 0x68];
    layout[0] = 1;
    layout[1] = 3;
    layout[2] = 0;
    layout[4] = 0;
    layout[5] = 7;
    layout[6] = 0;
    layout[0x40] = 2;
    let mut out_o = 0u8;
    let mut out_n = 0u8;
    let ro = f_orig(layout.as_ptr() as usize, 0, 0, 0, (&mut out_o as *mut u8) as usize);
    let rn = f_new(layout.as_ptr() as usize, 0, 0, 0, (&mut out_n as *mut u8) as usize);
    crate::diff_eq!(ro, rn, "find_pos0_ret");
    crate::diff_eq!(out_o, out_n, "find_pos0_fmt");
    Ok(2)
}

pub unsafe fn diff_d3d_vertex_layout_get_element_offset(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8, u8, u32) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut layout = [0u8; 0x68];
    layout[0] = 0;
    layout[1] = 2;
    layout[2] = 0;
    layout[4] = 0;
    layout[5] = 4;
    layout[6] = 2;
    layout[0x40] = 2;
    let ro = f_orig(layout.as_ptr() as usize, 0, 2, 0);
    let rn = f_new(layout.as_ptr() as usize, 0, 2, 0);
    crate::diff_eq!(ro, rn, "elem_offset_hit");
    let ro2 = f_orig(layout.as_ptr() as usize, 0, 5, 0);
    let rn2 = f_new(layout.as_ptr() as usize, 0, 5, 0);
    crate::diff_eq!(ro2, rn2, "elem_offset_miss");
    Ok(2)
}

pub unsafe fn diff_d3d_vertex_layout_append_layout(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut src = [0u8; 0x68];
    src[0] = 1;
    src[1] = 2;
    src[2] = 3;
    src[3] = 1;
    src[0x40] = 1;
    let mut dst_o = [0u8; 0x68];
    let mut dst_n = [0u8; 0x68];
    f_orig(dst_o.as_mut_ptr() as usize, src.as_ptr() as usize);
    f_new(dst_n.as_mut_ptr() as usize, src.as_ptr() as usize);
    crate::diff_eq!(&dst_o[..], &dst_n[..], "append_layout");
    Ok(1)
}

pub unsafe fn diff_d3d_vertex_layout_add_element(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8, u8, u8, u8);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut dst_o = [0u8; 0x68];
    let mut dst_n = [0u8; 0x68];
    f_orig(dst_o.as_mut_ptr() as usize, 0, 2, 1, 0);
    f_new(dst_n.as_mut_ptr() as usize, 0, 2, 1, 0);
    crate::diff_eq!(&dst_o[..], &dst_n[..], "add_element");
    Ok(1)
}

pub unsafe fn diff_d3d_calc_texture_2d_byte_size(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, i32, i8) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut ran = 0;
    for fmt in 0..=46i8 {
        let ro = f_orig(64, 32, fmt);
        let rn = f_new(64, 32, fmt);
        crate::diff_eq!(ro, rn, "tex_2d_byte_size");
        ran += 1;
    }
    Ok(ran)
}

pub unsafe fn diff_d3d_calc_texture_mipchain_byte_size(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, i32, i32, i32, i8) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let ro = f_orig(64, 64, 4, 4, 15);
    let rn = f_new(64, 64, 4, 4, 15);
    crate::diff_eq!(ro, rn, "mipchain_rgba8");
    let ro2 = f_orig(64, 64, 1, 5, 28);
    let rn2 = f_new(64, 64, 1, 5, 28);
    crate::diff_eq!(ro2, rn2, "mipchain_bc1");
    Ok(2)
}

pub unsafe fn diff_d3d_calc_texture_3d_slice_byte_size(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, i32, u32, i8) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let ro = f_orig(32, 16, 8, 15);
    let rn = f_new(32, 16, 8, 15);
    crate::diff_eq!(ro, rn, "tex_3d_slice");
    Ok(1)
}

pub unsafe fn diff_d3d_calc_texture_row_blocks(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, i8) -> i32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    for (dim, fmt) in [(64, 15i8), (65, 28), (3, 36), (100, 38)] {
        let ro = f_orig(dim, fmt);
        let rn = f_new(dim, fmt);
        crate::diff_eq!(ro, rn, "row_blocks");
    }
    Ok(4)
}

pub unsafe fn diff_d3d_get_texture_format_bytes_per_pixel(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i8) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut ran = 0;
    for fmt in 0..=46i8 {
        let ro = f_orig(fmt);
        let rn = f_new(fmt);
        crate::diff_eq!(ro, rn, "fmt_bpp");
        ran += 1;
    }
    Ok(ran)
}

pub unsafe fn diff_d3d_texture_format_to_name(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i8) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut ran = 0;
    for fmt in 0..=47i8 {
        let ro = f_orig(fmt);
        let rn = f_new(fmt);
        crate::diff_eq!(ro, rn, "fmt_to_name");
        ran += 1;
    }
    Ok(ran)
}

pub unsafe fn diff_d3d_is_block_compressed_format(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i8) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut ran = 0;
    for fmt in 0..=45i8 {
        let ro = f_orig(fmt);
        let rn = f_new(fmt);
        crate::diff_eq!(ro, rn, "is_bc");
        ran += 1;
    }
    Ok(ran)
}

pub unsafe fn diff_d3d_downsample_rgba8_mip_in_place(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut pix_o = [0u8; 64];
    for (i, b) in pix_o.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(7);
    }
    let mut pix_n = pix_o;
    let mut w_o = 4i32;
    let mut h_o = 4i32;
    let mut w_n = 4i32;
    let mut h_n = 4i32;
    f_orig(
        pix_o.as_mut_ptr() as usize,
        (&mut w_o as *mut i32) as usize,
        (&mut h_o as *mut i32) as usize,
    );
    f_new(
        pix_n.as_mut_ptr() as usize,
        (&mut w_n as *mut i32) as usize,
        (&mut h_n as *mut i32) as usize,
    );
    crate::diff_eq!(w_o, w_n, "downsample_w");
    crate::diff_eq!(h_o, h_n, "downsample_h");
    crate::diff_eq!(pix_o, pix_n, "downsample_pix");
    Ok(3)
}

pub unsafe fn diff_d3d_calc_mip_count_2d(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, i32) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    for (w, h) in [(1, 1), (16, 8), (1024, 512), (2048, 2048)] {
        let ro = f_orig(w, h);
        let rn = f_new(w, h);
        crate::diff_eq!(ro, rn, "mip_count_2d");
    }
    Ok(4)
}

pub unsafe fn diff_d3d_is_valid_pow2_2d_size(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, i32) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    for (w, h) in [(0, 4), (64, 128), (100, 64), (0x4000, 0x4000), (0x8000, 4)] {
        let ro = f_orig(w, h);
        let rn = f_new(w, h);
        crate::diff_eq!(ro, rn, "valid_pow2");
    }
    Ok(5)
}

pub unsafe fn diff_d3d_is_float_texture_format(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i8) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut ran = 0;
    for fmt in 0..=45i8 {
        let ro = f_orig(fmt);
        let rn = f_new(fmt);
        crate::diff_eq!(ro, rn, "is_float_fmt");
        ran += 1;
    }
    Ok(ran)
}

pub unsafe fn diff_d3d_is_srgb_texture_format(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u8) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut ran = 0;
    for fmt in 0..=45u8 {
        let ro = f_orig(fmt);
        let rn = f_new(fmt);
        crate::diff_eq!(ro, rn, "is_srgb_fmt");
        ran += 1;
    }
    Ok(ran)
}

pub unsafe fn diff_d3d_get_texture_subresource_view_slot(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, i32, i32) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut obj = [0u8; 0x40];
    let ptr = obj.as_mut_ptr() as usize;
    wr::<i32>(ptr + 0x14, 4);
    wr::<usize>(ptr + 0x30, 0x1000_0000);
    let ro = f_orig(ptr, 2, 3);
    let rn = f_new(ptr, 2, 3);
    crate::diff_eq!(ro, rn, "subresource_view_slot");
    Ok(1)
}

struct Tls0Guard {
    tls_arr: usize,
    saved: usize,
}

impl Tls0Guard {
    unsafe fn new() -> Self {
        let tls_arr = exe::tls_pointer();
        let saved = rd::<usize>(tls_arr);
        Self { tls_arr, saved }
    }
    unsafe fn set(&self, val: usize) {
        wr::<usize>(self.tls_arr, val);
    }
}

impl Drop for Tls0Guard {
    fn drop(&mut self) {
        unsafe {
            wr::<usize>(self.tls_arr, self.saved);
        }
    }
}

struct SnapGuard(crate::difftest::Snapshot);

impl SnapGuard {
    unsafe fn take(at: usize, len: usize) -> Self {
        Self(crate::difftest::Snapshot::take(at as *mut u8, len))
    }
    unsafe fn restore(&self) {
        self.0.restore();
    }
}

impl Drop for SnapGuard {
    fn drop(&mut self) {
        unsafe {
            self.0.restore();
        }
    }
}

pub unsafe fn diff_d3d_tls_ctx_unbind_target(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let tls_guard = Tls0Guard::new();
    let mut tls_buf = vec![0u8; 0x47400];
    let tls0 = tls_buf.as_mut_ptr() as usize;
    tls_guard.set(tls0);

    let setup = |tls0: usize| {
        wr::<u8>(tls0 + 0x70, 1);
        let ctx = tls0 + 0x467e0;
        wr::<usize>(ctx + 0xb18, 0x1234_5678);
        wr::<usize>(ctx + 0xb20, 0xaaaa_bbbb);
        wr::<usize>(ctx + 0x48, 0xcccc_dddd);
        wr::<u8>(ctx + 0xa80, 0x7f);
    };

    let ctx = tls0 + 0x467e0;
    for target in [0x9999_9999usize, 0x1234_5678usize] {
        setup(tls0);
        f_orig(target);
        let o_state = (
            rd::<usize>(ctx + 0xb18),
            rd::<usize>(ctx + 0xb20),
            rd::<usize>(ctx + 0x48),
            rd::<u8>(ctx + 0xa80),
        );

        setup(tls0);
        f_new(target);
        let n_state = (
            rd::<usize>(ctx + 0xb18),
            rd::<usize>(ctx + 0xb20),
            rd::<usize>(ctx + 0x48),
            rd::<u8>(ctx + 0xa80),
        );
        crate::diff_eq!(o_state, n_state, "unbind_target");
    }
    Ok(2)
}

pub unsafe fn diff_d3d_register_vertex_layout(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let hashes_base = exe::ptr::<u64>(0x1_4397_db80) as usize;
    let lock_cnt_addr = exe::ptr::<u32>(0x1_43a2_afe8) as usize;
    let count_addr = exe::ptr::<u32>(0x1_43a2_afec) as usize;
    let layouts_base = exe::ptr::<u8>(0x1_43a4_9130) as usize;

    let _s1 = SnapGuard::take(hashes_base, 32);
    let _s2 = SnapGuard::take(lock_cnt_addr, 8);
    let _s3 = SnapGuard::take(layouts_base, 4 * 0x68);

    let mut layout = [0u8; 0x68];
    for (i, b) in layout.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(3).wrapping_add(1);
    }
    layout[0x40] = 2;
    let l_ptr = layout.as_ptr() as usize;

    let mut out_id: u32 = 0;
    let out_ptr = (&mut out_id as *mut u32) as usize;

    let reset_insert = || {
        for k in 0..4usize {
            wr::<u64>(hashes_base + k * 8, 0);
        }
        wr::<u32>(lock_cnt_addr, 0);
        wr::<u32>(count_addr, 0);
        std::ptr::write_bytes(layouts_base as *mut u8, 0, 3 * 0x68);
        wr::<u32>(out_ptr, 0xdead_beef);
    };

    reset_insert();
    let ro = f_orig(out_ptr, l_ptr);
    let o_id = rd::<u32>(out_ptr);
    let o_cnt = (rd::<u32>(lock_cnt_addr), rd::<u32>(count_addr));
    let o_hash1 = rd::<u64>(hashes_base + 8);
    let o_slot1: Vec<u8> =
        std::slice::from_raw_parts((layouts_base + 0x68) as *const u8, 0x68).to_vec();

    reset_insert();
    let rn = f_new(out_ptr, l_ptr);
    let n_id = rd::<u32>(out_ptr);
    let n_cnt = (rd::<u32>(lock_cnt_addr), rd::<u32>(count_addr));
    let n_hash1 = rd::<u64>(hashes_base + 8);
    let n_slot1: Vec<u8> =
        std::slice::from_raw_parts((layouts_base + 0x68) as *const u8, 0x68).to_vec();

    crate::diff_eq!(ro, rn, "register_vertex_layout ret");
    crate::diff_eq!(o_id, n_id, "register_vertex_layout id");
    crate::diff_eq!(o_cnt, n_cnt, "register_vertex_layout counters");
    crate::diff_eq!(o_hash1, n_hash1, "register_vertex_layout hash");
    crate::diff_eq!(o_slot1, n_slot1, "register_vertex_layout copied layout");

    // Second case: already present at idx 0
    wr::<u64>(hashes_base, o_hash1);
    wr::<u32>(lock_cnt_addr, 1);
    wr::<u32>(count_addr, 1);
    wr::<u32>(out_ptr, 0xdead_beef);
    let ro2 = f_orig(out_ptr, l_ptr);
    let o_id2 = rd::<u32>(out_ptr);

    wr::<u32>(out_ptr, 0xdead_beef);
    let rn2 = f_new(out_ptr, l_ptr);
    let n_id2 = rd::<u32>(out_ptr);

    crate::diff_eq!(ro2, rn2, "register_vertex_layout hit ret");
    crate::diff_eq!(o_id2, n_id2, "register_vertex_layout hit id");
    Ok(2)
}

unsafe extern "win64" fn fake_vlayout_read_u32(stream: usize, out: usize) {
    let data = rd::<usize>(stream + 0x08);
    let cur = rd::<usize>(stream + 0x10);
    wr::<u32>(out, rd::<u32>(data + cur));
    wr::<usize>(stream + 0x10, cur + 4);
}

unsafe extern "win64" fn fake_vlayout_read_u8(stream: usize, out: usize) {
    let data = rd::<usize>(stream + 0x08);
    let cur = rd::<usize>(stream + 0x10);
    wr::<u8>(out, rd::<u8>(data + cur));
    wr::<usize>(stream + 0x10, cur + 1);
}

pub unsafe fn diff_d3d_vertex_layout_deserialize(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 16];
    vtbl[0x48 / 8] = fake_vlayout_read_u32 as *const () as usize;
    vtbl[0x70 / 8] = fake_vlayout_read_u8 as *const () as usize;

    let stream_bytes: [u8; 16] = [0x78, 0x56, 0x34, 0x12, 2, 0, 2, 0, 0, 1, 4, 2, 1, 0, 0, 0];
    let mut stream_obj = [0usize; 3];
    stream_obj[0] = vtbl.as_ptr() as usize;
    stream_obj[1] = stream_bytes.as_ptr() as usize;
    let stream_ptr = stream_obj.as_mut_ptr() as usize;

    let mut layout = [0xa5u8; 0x68];
    let layout_ptr = layout.as_mut_ptr() as usize;

    stream_obj[2] = 0;
    layout.fill(0xa5);
    let ro = f_orig(stream_ptr, layout_ptr);
    let o_cur = stream_obj[2];
    let o_layout = layout;

    stream_obj[2] = 0;
    layout.fill(0xa5);
    let rn = f_new(stream_ptr, layout_ptr);
    let n_cur = stream_obj[2];
    let n_layout = layout;

    crate::diff_eq!(ro, rn, "vertex_layout_deserialize ret");
    crate::diff_eq!(o_cur, n_cur, "vertex_layout_deserialize cursor");
    crate::diff_eq!(&o_layout[..], &n_layout[..], "vertex_layout_deserialize layout");
    Ok(1)
}

static FREE_PAGE_RELEASED: AtomicU32 = AtomicU32::new(0);

unsafe extern "win64" fn fake_release_heap_obj(ptr: usize, tag: u32) {
    FREE_PAGE_RELEASED.store((ptr as u32) ^ tag, Ordering::SeqCst);
}

pub unsafe fn diff_d3d_heap_page_mgr_free_page(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let _m = crate::difftest::Mock::new(0x1_4209_a1e0, fake_release_heap_obj as *const ())?;

    for spare_len in [1u32, 2u32] {
        let run_one = |f: F| {
            let mut this_buf = [0u8; 0xa0];
            let mut pages_buf = [0u8; 4 * 16];
            let mut spare_buf = [0u8; 4 * 16];
            let mut free_buf = [0u16; 8];
            let this = this_buf.as_mut_ptr() as usize;
            let pages_ptr = pages_buf.as_mut_ptr() as usize;
            let spare_ptr = spare_buf.as_mut_ptr() as usize;
            let free_ptr = free_buf.as_mut_ptr() as usize;
            wr::<usize>(this + 0x68, pages_ptr);
            wr::<u32>(this + 0x70, 4);
            wr::<u32>(this + 0x74, 4);
            wr::<usize>(this + 0x78, spare_ptr);
            wr::<u32>(this + 0x80, spare_len);
            wr::<u32>(this + 0x84, 4);
            wr::<usize>(this + 0x88, free_ptr);
            wr::<u32>(this + 0x90, 1);
            wr::<u32>(this + 0x94, 4);
            wr::<u16>(free_ptr, 7);
            wr::<u32>(pages_ptr + 16, 0x1122_3344);
            wr::<usize>(pages_ptr + 24, 0x5566_7788);
            FREE_PAGE_RELEASED.store(0, Ordering::SeqCst);
            f(this, 1);
            wr::<usize>(this + 0x68, 0);
            wr::<usize>(this + 0x78, 0);
            wr::<usize>(this + 0x88, 0);
            (this_buf, pages_buf, spare_buf, free_buf, FREE_PAGE_RELEASED.load(Ordering::SeqCst))
        };

        let o_state = run_one(f_orig);
        let n_state = run_one(f_new);
        crate::diff_eq!(o_state, n_state, "heap_page_mgr_free_page");
    }
    Ok(2)
}

unsafe extern "win64" fn fake_entry_out_ptr(entry: usize) -> usize {
    entry + 8
}

unsafe extern "win64" fn fake_d3d12_create_heap(
    _dev: usize,
    desc: usize,
    _riid: usize,
    out_ptr: usize,
) -> u32 {
    let sz = rd::<u64>(desc);
    let flags = rd::<u32>(desc + 0x28) as u64;
    wr::<u64>(out_ptr, sz ^ flags ^ 0xabc0_0000);
    0
}

pub unsafe fn diff_d3d_heap_page_mgr_alloc_page(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16) -> u16;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let dev_ctx_global = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let d3d_dev_global = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let _s1 = SnapGuard::take(dev_ctx_global, 8);
    let _s2 = SnapGuard::take(d3d_dev_global, 8);
    let _m1 = crate::difftest::Mock::new(0x1_411d_2ad0, fake_entry_out_ptr as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4209_a1e0, fake_release_heap_obj as *const ())?;

    let mut dev_ctx_buf = [0u8; 0xb0];
    dev_ctx_buf[0xa0] = 1;
    wr::<usize>(dev_ctx_global, dev_ctx_buf.as_ptr() as usize);

    let mut dev_vtbl = [0usize; 32];
    dev_vtbl[0xe0 / 8] = fake_d3d12_create_heap as *const () as usize;
    let dev_obj: usize = dev_vtbl.as_ptr() as usize;
    wr::<usize>(d3d_dev_global, (&dev_obj as *const usize) as usize);

    // Case 1: reuse from free_vec + spare_vec
    let run_spare = |f: F| {
        let mut this_buf = [0u8; 0xa0];
        let mut pages_buf = [0u8; 4 * 16];
        let mut spare_buf = [0u8; 4 * 16];
        let mut free_buf = [0u16; 8];
        let this = this_buf.as_mut_ptr() as usize;
        let pages_ptr = pages_buf.as_mut_ptr() as usize;
        let spare_ptr = spare_buf.as_mut_ptr() as usize;
        let free_ptr = free_buf.as_mut_ptr() as usize;
        wr::<usize>(this + 0x68, pages_ptr);
        wr::<u32>(this + 0x70, 3);
        wr::<u32>(this + 0x74, 4);
        wr::<usize>(this + 0x78, spare_ptr);
        wr::<u32>(this + 0x80, 1);
        wr::<u32>(this + 0x84, 4);
        wr::<usize>(this + 0x88, free_ptr);
        wr::<u32>(this + 0x90, 1);
        wr::<u32>(this + 0x94, 4);
        wr::<u16>(free_ptr, 2);
        wr::<u32>(spare_ptr, 0x77);
        wr::<u64>(spare_ptr + 8, 0xdead_beef_cafe);
        let r = f(this, 4);
        (r, pages_buf, spare_buf, free_buf, rd::<u32>(this + 0x70), rd::<u32>(this + 0x80), rd::<u32>(this + 0x90))
    };

    let o1 = run_spare(f_orig);
    let n1 = run_spare(f_new);
    crate::diff_eq!(o1, n1, "heap_page_mgr_alloc_page spare");

    // Case 2: free_len == 0, spare_len == 0 -> CreateHeap path
    let run_create = |f: F| {
        let mut this_buf = [0u8; 0xa0];
        let mut pages_buf = [0u8; 4 * 16];
        let mut spare_buf = [0u8; 4 * 16];
        let mut free_buf = [0u16; 8];
        let this = this_buf.as_mut_ptr() as usize;
        let pages_ptr = pages_buf.as_mut_ptr() as usize;
        let spare_ptr = spare_buf.as_mut_ptr() as usize;
        let free_ptr = free_buf.as_mut_ptr() as usize;
        wr::<usize>(this + 0x68, pages_ptr);
        wr::<u32>(this + 0x70, 1);
        wr::<u32>(this + 0x74, 4);
        wr::<usize>(this + 0x78, spare_ptr);
        wr::<u32>(this + 0x80, 0);
        wr::<u32>(this + 0x84, 4);
        wr::<usize>(this + 0x88, free_ptr);
        wr::<u32>(this + 0x90, 0);
        wr::<u32>(this + 0x94, 4);
        let r = f(this, 4);
        (r, pages_buf, rd::<u32>(this + 0x70))
    };

    let o2 = run_create(f_orig);
    let n2 = run_create(f_new);
    crate::diff_eq!(o2, n2, "heap_page_mgr_alloc_page create");
    Ok(2)
}

unsafe extern "win64" fn fake_srw_noop(_lock: usize) {}

unsafe extern "win64" fn fake_pool_alloc_node(pool: usize) -> usize {
    rd::<usize>(pool)
}

unsafe extern "win64" fn fake_buddy_alloc_sub(buddy: usize, bucket: u32) -> usize {
    let sub = rd::<usize>(buddy);
    wr::<u32>(sub + 0x10, bucket);
    sub
}

pub unsafe fn diff_d3d_heap_page_mgr_alloc_subrange(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mgr_global = exe::ptr::<usize>(0x1_4397_fb80) as usize;
    let _s1 = SnapGuard::take(mgr_global, 8);
    let _im1 = crate::difftest::IatMock::new(IAT_ACQUIRE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _im2 = crate::difftest::IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_srw_noop as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4243_b450, fake_pool_alloc_node as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4209_dde0, fake_buddy_alloc_sub as *const ())?;

    let mut mgr_buf = [0u8; 0xb0];
    let mut node_buf = [0u8; 0x18];
    let mut sub_buf = [0u8; 0x18];
    let mut pages_buf = [0u8; 2 * 16];

    let mgr = mgr_buf.as_mut_ptr() as usize;
    let node_ptr = node_buf.as_mut_ptr() as usize;
    let sub_ptr = sub_buf.as_mut_ptr() as usize;
    let pages_ptr = pages_buf.as_mut_ptr() as usize;
    wr::<usize>(mgr_global, mgr);

    for byte_size in [0x10000usize, 0x20000usize, 0x50000usize] {
        let mut run_one = |f: F| {
            mgr_buf.fill(0);
            node_buf.fill(0xff);
            sub_buf.fill(0);
            pages_buf.fill(0);
            wr::<usize>(mgr + 0x08, sub_ptr);
            wr::<u64>(mgr + 0x58, 10);
            wr::<u64>(mgr + 0x60, 2);
            wr::<usize>(mgr + 0x68, pages_ptr);
            wr::<usize>(mgr + 0x98, node_ptr);
            wr::<u16>(sub_ptr + 0x0a, 4);
            wr::<u16>(sub_ptr + 0x0c, 1);
            wr::<u64>(pages_ptr + 16 + 0x08, 0xcafe_babe_1234);
            let r = f(0, byte_size);
            (r, mgr_buf, node_buf, sub_buf)
        };

        let o_state = run_one(f_orig);
        let n_state = run_one(f_new);

        crate::diff_eq!(o_state, n_state, "heap_page_mgr_alloc_subrange");
    }
    Ok(3)
}

static DTOR_SEQ: AtomicU32 = AtomicU32::new(0);

unsafe extern "win64" fn fake_dtor_98(p: usize) {
    let seq = DTOR_SEQ.fetch_add(1, Ordering::SeqCst) + 1;
    wr::<u32>(p, 0x9800 | seq);
}

unsafe extern "win64" fn fake_dtor_vec_u16(p: usize) {
    let seq = DTOR_SEQ.fetch_add(1, Ordering::SeqCst) + 1;
    wr::<u32>(p, 0x8800 | seq);
}

unsafe extern "win64" fn fake_dtor_vec_page(p: usize) {
    let seq = DTOR_SEQ.fetch_add(1, Ordering::SeqCst) + 1;
    wr::<u32>(p, 0x7800 | seq);
}

unsafe extern "win64" fn fake_dtor_48(p: usize) {
    let seq = DTOR_SEQ.fetch_add(1, Ordering::SeqCst) + 1;
    wr::<u32>(p, 0x4800 | seq);
}

pub unsafe fn diff_d3d_heap_page_mgr_dtor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let _m1 = crate::difftest::Mock::new(0x1_4243_b540, fake_dtor_98 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_e830, fake_dtor_vec_u16 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411d_2b40, fake_dtor_vec_page as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_402c_30b0, fake_dtor_48 as *const ())?;

    let mut buf = [0u8; 0xb0];
    let ptr = buf.as_mut_ptr() as usize;

    buf.fill(0);
    DTOR_SEQ.store(0, Ordering::SeqCst);
    f_orig(ptr);
    let o_buf = buf;

    buf.fill(0);
    DTOR_SEQ.store(0, Ordering::SeqCst);
    f_new(ptr);
    let n_buf = buf;

    crate::diff_eq!(&o_buf[..], &n_buf[..], "heap_page_mgr_dtor");
    Ok(1)
}

unsafe extern "win64" fn fake_ctor_buddy_init(buddy: usize, cfg: usize) {
    wr::<usize>(buddy, rd::<usize>(cfg));
    wr::<usize>(buddy + 0x08, rd::<usize>(cfg + 0x08));
    wr::<usize>(buddy + 0x10, rd::<usize>(cfg + 0x10));
    wr::<usize>(buddy + 0x18, rd::<usize>(cfg + 0x18));
    wr::<u16>(buddy + 0x20, rd::<u16>(cfg + 0x20));
    wr::<u64>(buddy + 0x28, rd::<u64>(cfg + 0x28));
}

unsafe extern "win64" fn fake_ctor_pool_init(
    pool: usize,
    _rdx: usize,
    _r8: usize,
    r9: u32,
    arg5: usize,
    arg6: u32,
    arg7: usize,
) {
    wr::<u32>(pool, r9);
    wr::<usize>(pool + 0x08, arg5);
    wr::<u32>(pool + 0x10, arg6);
    wr::<usize>(pool + 0x18, arg7);
}

pub unsafe fn diff_d3d_heap_page_mgr_ctor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let _m1 = crate::difftest::Mock::new(0x1_4209_e4d0, fake_ctor_buddy_init as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4243_b5f0, fake_ctor_pool_init as *const ())?;

    let mut buf = [0x5au8; 0xc0];
    let ptr = buf.as_mut_ptr() as usize;

    buf.fill(0x5a);
    let ro = f_orig(ptr);
    let o_buf = buf;

    buf.fill(0x5a);
    let rn = f_new(ptr);
    let n_buf = buf;

    crate::diff_eq!(ro, rn, "heap_page_mgr_ctor ret");
    crate::diff_eq!(&o_buf[..], &n_buf[..], "heap_page_mgr_ctor buf");
    Ok(1)
}

// REGISTRY: Replacement::verified(0x14204db80, "d3d::tlsCtxUnbindTarget_14204db80", crate::ns::rend::d3d_tls_ctx::d3d_tls_ctx_unbind_target as *const (), crate::ns::rend::d3d_tls_ctx::diff_d3d_tls_ctx_unbind_target),
// REGISTRY: Replacement::verified(0x14204dbf0, "d3d::registerVertexLayout_14204dbf0", crate::ns::rend::d3d_tls_ctx::d3d_register_vertex_layout as *const (), crate::ns::rend::d3d_tls_ctx::diff_d3d_register_vertex_layout),
// REGISTRY: Replacement::verified(0x14204dcc0, "d3d::VertexLayout::deserialize_14204dcc0", crate::ns::rend::d3d_tls_ctx::d3d_vertex_layout_deserialize as *const (), crate::ns::rend::d3d_tls_ctx::diff_d3d_vertex_layout_deserialize),
// REGISTRY: Replacement::verified(0x14204e060, "d3d::HeapPageManager::freePage_14204e060", crate::ns::rend::d3d_tls_ctx::d3d_heap_page_mgr_free_page as *const (), crate::ns::rend::d3d_tls_ctx::diff_d3d_heap_page_mgr_free_page),
// REGISTRY: Replacement::verified_throws(0x14204e160, "d3d::HeapPageManager::allocPage_14204e160", crate::ns::rend::d3d_tls_ctx::d3d_heap_page_mgr_alloc_page as *const (), Throws::Discard, crate::ns::rend::d3d_tls_ctx::diff_d3d_heap_page_mgr_alloc_page),
// REGISTRY: Replacement::verified(0x14204e3e0, "d3d::HeapPageManager::allocSubrange_14204e3e0", crate::ns::rend::d3d_tls_ctx::d3d_heap_page_mgr_alloc_subrange as *const (), crate::ns::rend::d3d_tls_ctx::diff_d3d_heap_page_mgr_alloc_subrange),
// REGISTRY: Replacement::verified(0x14204e4b0, "d3d::HeapPageManager::dtor_14204e4b0", crate::ns::rend::d3d_tls_ctx::d3d_heap_page_mgr_dtor as *const (), crate::ns::rend::d3d_tls_ctx::diff_d3d_heap_page_mgr_dtor),
// REGISTRY: Replacement::verified(0x14204e510, "d3d::HeapPageManager::ctor_14204e510", crate::ns::rend::d3d_tls_ctx::d3d_heap_page_mgr_ctor as *const (), crate::ns::rend::d3d_tls_ctx::diff_d3d_heap_page_mgr_ctor),


