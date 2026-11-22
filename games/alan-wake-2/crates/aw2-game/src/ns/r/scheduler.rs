//! Phase 1 Group 1 main-thread hot-path functions:
//! - `0x1401e8680`: `open_hash_probe_40`
//! - `0x14248c1e0`: `ui_allocator_contains_ptr`
//! - `0x14248c4e0`: `ui_allocator_alloc_zeroed`
//! - `0x141e154d0`: `fact_variant_compare_and_set`
//! - `0x141e0fc80`: `update_fact_bindings`
//! - `0x1420b2b50`: `ecs_bind_groups_match`
//! - `0x1420a8fd0`: `start_scheduling`
//! - `0x1420a9960`: `schedule_node_complete`
//! - `0x142422290`: `streamer_resource_tick`
//! - `0x1424f8220`: `ui_page_update`

use crate::exe;

#[inline(always)]
unsafe fn rd<T: Copy>(addr: u64) -> T {
    exe::rd::<T>(addr as usize)
}

#[inline(always)]
unsafe fn wr<T: Copy>(addr: u64, v: T) {
    exe::wr::<T>(addr as usize, v);
}

struct GlobalGuard {
    va: u64,
    prev: u64,
}

impl GlobalGuard {
    unsafe fn set(va: u64, val: u64) -> Self {
        let prev = rd::<u64>(va);
        wr::<u64>(va, val);
        Self { va, prev }
    }
}

impl Drop for GlobalGuard {
    fn drop(&mut self) {
        unsafe {
            wr::<u64>(self.va, self.prev);
        }
    }
}

// ---------------------------------------------------------------------------
// 1. 0x1401e8680 — open_hash_probe_40
// ---------------------------------------------------------------------------

/// `sub_1401e8680` — original @ 0x1401e8680 (228 bytes, leaf).
/// Quadratic-probing lookup in a 40-byte-entry open-addressed table using a
/// 16-byte key `(*r8, *(r8 + 8))`. Writes `(found_idx, insert_idx)` into `out_pair` (`rdx`)
/// and returns `out_pair`.
pub unsafe extern "win64" fn open_hash_probe_40(
    table: u64,
    out_pair: u64,
    key_ptr: u64,
) -> u64 {
    let k0 = rd::<u64>(key_ptr);
    let k1 = rd::<u64>(key_ptr + 8);
    let mask = rd::<u64>(table + 0x48).wrapping_sub(1);
    let entries = rd::<u64>(table + 0x78);
    let empty0 = rd::<u64>(table + 0x50);
    let empty1 = rd::<u64>(table + 0x58);
    let tomb_count = rd::<u64>(table + 0x38);
    let tomb0 = rd::<u64>(table + 0x28);
    let tomb1 = rd::<u64>(table + 0x30);

    let mut step = 0u64;
    let mut first_tomb = u64::MAX;
    let mut idx = k0 ^ k1;

    loop {
        idx &= mask;
        let entry = entries.wrapping_add(idx.wrapping_mul(40));
        let e0 = rd::<u64>(entry);
        if e0 == empty0 && rd::<u64>(entry + 8) == empty1 {
            wr::<u64>(out_pair, u64::MAX);
            wr::<u64>(out_pair + 8, if first_tomb == u64::MAX { idx } else { first_tomb });
            return out_pair;
        }
        if tomb_count > 0 && e0 == tomb0 && rd::<u64>(entry + 8) == tomb1 {
            if first_tomb == u64::MAX {
                step = step.wrapping_add(1);
                first_tomb = idx;
                idx = idx.wrapping_add(step);
                continue;
            }
        } else if e0 == k0 && rd::<u64>(entry + 8) == k1 {
            wr::<u64>(out_pair, idx);
            wr::<u64>(out_pair + 8, u64::MAX);
            return out_pair;
        }
        step = step.wrapping_add(1);
        idx = idx.wrapping_add(step);
    }
}

// ---------------------------------------------------------------------------
// 2. 0x14248c1e0 — ui_allocator_contains_ptr
// ---------------------------------------------------------------------------

/// Optional byte spinlock of the ui allocator (`lock_obj + 0x10` = enabled, byte at
/// `lock_obj` = held); backoff through `0x142417470` after a failed `xchg`.
#[inline]
pub(crate) unsafe fn spin_lock_acquire_if_enabled(lock_obj: u64) -> bool {
    if rd::<u8>(lock_obj + 0x10) == 0 {
        return false;
    }
    let mut backoff: u64 = 0x64;
    let p = lock_obj as *const core::sync::atomic::AtomicU8;
    loop {
        while (*p).load(core::sync::atomic::Ordering::Relaxed) != 0 {
            core::hint::spin_loop();
        }
        if (*p).swap(1, core::sync::atomic::Ordering::Acquire) == 0 {
            return true;
        }
        exe::call_va!(0x1_4241_7470, ((&mut backoff as *mut u64) as u64));
    }
}

#[inline]
pub(crate) unsafe fn spin_lock_release_if_held(lock_obj: u64, held: bool) {
    if held {
        let p = lock_obj as *const core::sync::atomic::AtomicU8;
        (*p).store(0, core::sync::atomic::Ordering::Release);
    }
}

/// `sub_14248c1e0` — original @ 0x14248c1e0 (167 bytes).
/// Checks whether `ptr` (`rdx`) falls within any 24-byte segment `[base, base + size)`
/// in the allocator's segment array at `[this + 0x28]` (count `u32` at `[this + 0x30]`).
pub unsafe extern "win64" fn ui_allocator_contains_ptr(this: u64, ptr: u64) -> u64 {
    let held = spin_lock_acquire_if_enabled(this);
    let base = rd::<u64>(this + 0x28);
    let count = rd::<u32>(this + 0x30) as u64;
    let end = base.wrapping_add(count.wrapping_mul(24));
    let mut cur = base;
    let mut found = 0u64;
    while cur != end {
        let seg_start = rd::<u64>(cur);
        if ptr >= seg_start {
            let seg_end = seg_start.wrapping_add(rd::<u64>(cur + 8));
            if ptr < seg_end {
                found = 1;
                break;
            }
        }
        cur = cur.wrapping_add(24);
    }
    spin_lock_release_if_held(this, held);
    found
}

// ---------------------------------------------------------------------------
// 3. 0x14248c4e0 — ui_allocator_alloc_zeroed
// ---------------------------------------------------------------------------

/// `sub_14248c4e0` — original @ 0x14248c4e0 (207 bytes).
/// Allocates `size` bytes from the TLSF/segregated heap at `[this + 0x18]` via `0x142512760`
/// and zero-fills the usable payload computed from the block header at `[ptr - 8]`.
pub unsafe extern "win64" fn ui_allocator_alloc_zeroed(this: u64, size: u64) -> u64 {
    let held = spin_lock_acquire_if_enabled(this);
    let heap = rd::<u64>(this + 0x18);
    let ptr = exe::call_va!(0x1_4251_2760, (heap, size) -> u64);
    if ptr != 0 {
        let hdr = rd::<u64>(ptr.wrapping_sub(8));
        let tag = hdr & 3;
        let usable = if tag == 1 {
            0u64
        } else {
            let overhead = if tag == 0 { 0x10u64 } else { 0x08u64 };
            (hdr & !7u64).wrapping_sub(overhead)
        };
        if usable != 0 {
            core::ptr::write_bytes(ptr as *mut u8, 0, usable as usize);
        }
    }
    spin_lock_release_if_held(this, held);
    ptr
}

// ---------------------------------------------------------------------------
// 4. 0x141e154d0 — fact_variant_compare_and_set
// ---------------------------------------------------------------------------

/// `sub_141e154d0` — original @ 0x141e154d0 (569 bytes).
/// Compares the incoming fact value at `src_val` (`rdx`) against the binding's cached
/// variant at `binding + 0x80` (`r8`, type tag at `binding + 0xc8`) and updates it if changed.
pub unsafe extern "win64" fn fact_variant_compare_and_set(
    ctx: u64,
    src_val: u64,
    binding: u64,
) -> u64 {
    let tag = rd::<u8>(binding + 0xc8);
    match tag {
        0 => {
            let new_b = u8::from(rd::<u64>(src_val + 8) != 0);
            if rd::<u8>(binding + 0x80) == new_b {
                0
            } else {
                wr::<u8>(binding + 0x80, new_b);
                1
            }
        }
        1 => {
            let new_d = f64::from_bits(rd::<u64>(src_val + 8));
            let old_d = f64::from_bits(rd::<u64>(binding + 0x80));
            let diff = f64::from_bits((old_d - new_d).to_bits() & 0x7fff_ffff_ffff_ffff);
            let eps = f64::from_bits(0x3cb0_0000_0000_0000);
            if diff <= eps {
                0
            } else {
                wr::<u64>(binding + 0x80, new_d.to_bits());
                1
            }
        }
        2 => {
            let cstr = exe::call_native!(
                0x1_4021_7680,
                crate::ns::rend::cauldron_hot_o::unknown_140217680,
                (src_val as usize, ctx as usize) -> usize
            ) as u64;
            let mut len = 0u64;
            if cstr != 0 {
                while rd::<u8>(cstr + len) != 0 {
                    len += 1;
                }
            }
            // Unchanged value (the common case): the original builds a temporary inplace
            // string from `cstr`, finds it equal (case-folded) and destroys it.
            if crate::ns::rend::cauldron_hot_a::inplace_str_eq_nocase_bytes(
                (binding + 0x80) as usize,
                cstr as usize,
                len as usize,
            ) {
                return 0;
            }
            let mut tmp_str = [0u64; 9];
            let tmp_ptr = tmp_str.as_mut_ptr() as u64;
            wr::<u64>(tmp_ptr, 0x0800_0008);
            wr::<u8>(tmp_ptr + 8, 0);
            if cstr == 0 {
                wr::<u32>(tmp_ptr + 4, 0);
                wr::<u8>(tmp_ptr + 8, 0);
            } else {
                let view = [cstr, len];
                let _ = exe::call_va!(
                    0x1_4240_9b50,
                    (tmp_ptr, view.as_ptr() as u64, 0u64, len) -> u64
                );
            }
            let mut changed = 0u64;
            if rd::<u8>(binding + 0xc8) == 2 {
                if (exe::call_va!(0x1_401e_6ff0, (binding + 0x80, tmp_ptr) -> u64) & 0xff) == 0 {
                    let _ = exe::call_va!(0x1_40f6_7380, (binding + 0x80, tmp_ptr) -> u64);
                    changed = 1;
                }
            }
            exe::call_va!(0x1_4240_ab10, (tmp_ptr));
            changed
        }
        3 => {
            let old0 = f32::from_bits(rd::<u32>(binding + 0x80));
            let new0 = f32::from_bits(rd::<u32>(src_val + 8));
            let old1 = f32::from_bits(rd::<u32>(binding + 0x84));
            let new1 = f32::from_bits(rd::<u32>(src_val + 0x0c));
            if old0 == new0 && old1 == new1 {
                0
            } else {
                wr::<u64>(binding + 0x80, rd::<u64>(src_val + 8));
                1
            }
        }
        4 => {
            let dst = binding + 0x80;
            let src = src_val + 8;
            if rd::<u8>(dst + 0x48) != 4 {
                exe::call_va!(0x1_4020_0070, ());
                return 0;
            }
            let same = f32::from_bits(rd::<u32>(dst)) == f32::from_bits(rd::<u32>(src))
                && f32::from_bits(rd::<u32>(dst + 4)) == f32::from_bits(rd::<u32>(src + 4))
                && f32::from_bits(rd::<u32>(dst + 8)) == f32::from_bits(rd::<u32>(src + 8));
            if same {
                0
            } else {
                exe::call_va!(0x1_40f6_73f0, (dst, src));
                1
            }
        }
        5 => {
            let dst = binding + 0x80;
            let src = src_val + 8;
            if rd::<u8>(dst + 0x48) != 5 {
                exe::call_va!(0x1_4020_0070, ());
                return 0;
            }
            let same = f32::from_bits(rd::<u32>(dst)) == f32::from_bits(rd::<u32>(src))
                && f32::from_bits(rd::<u32>(dst + 4)) == f32::from_bits(rd::<u32>(src + 4))
                && f32::from_bits(rd::<u32>(dst + 8)) == f32::from_bits(rd::<u32>(src + 8))
                && f32::from_bits(rd::<u32>(dst + 12)) == f32::from_bits(rd::<u32>(src + 12));
            if same {
                0
            } else {
                exe::call_va!(0x1_40f6_7450, (dst, src));
                1
            }
        }
        _ => 0,
    }
}

// ---------------------------------------------------------------------------
// 5. 0x141e0fc80 — update_fact_bindings
// ---------------------------------------------------------------------------

#[inline]
unsafe fn rbtree_next(mut node: u64) -> u64 {
    let right = rd::<u64>(node + 0x10);
    if rd::<u8>(right + 0x19) != 0 {
        let mut parent = rd::<u64>(node + 0x08);
        while rd::<u8>(parent + 0x19) == 0 && node == rd::<u64>(parent + 0x10) {
            node = parent;
            parent = rd::<u64>(parent + 0x08);
        }
        parent
    } else {
        let mut cur = right;
        let mut left = rd::<u64>(cur);
        while rd::<u8>(left + 0x19) == 0 {
            cur = left;
            left = rd::<u64>(left);
        }
        cur
    }
}

/// `sub_141e0fc80` — original @ 0x141e0fc80 (1132 bytes).
/// Evaluates fact bindings in `[this]` (`rcx`) against fact hash table `table` (`r8`)
/// and updates bound objects and list bindings in the RB-tree at `[this + 0x20]`.
pub unsafe extern "win64" fn update_fact_bindings(
    this: u64,
    arg2: u64,
    table: u64,
    arg4: u64,
) -> u64 {
    let global_mgr = rd::<u64>(0x1_43a0_82e8);
    let cur_epoch = rd::<u32>(global_mgr + 0x10);
    let old_epoch = rd::<u32>(this + 0x38);
    let ne_flag = u8::from(cur_epoch != old_epoch);

    let count = rd::<u32>(this + 0x08) as u64;
    let mut entry = rd::<u64>(this);
    let end = entry.wrapping_add(count.wrapping_mul(0xd0));

    while entry != end {
        let cap = rd::<u64>(table + 0x48);
        let idx = if rd::<u64>(table + 0x40) == rd::<u64>(table + 0x38) {
            cap
        } else {
            let mut pair = [0u64; 2];
            open_hash_probe_40(table, pair.as_mut_ptr() as u64, entry + 0x60);
            if pair[0] == u64::MAX { cap } else { pair[0] }
        };
        let entries_base = rd::<u64>(table + 0x78);
        let slot = entries_base.wrapping_add(idx.wrapping_mul(40));
        let end_slot = entries_base.wrapping_add(cap.wrapping_mul(40));
        let val_ptr = if slot == end_slot || rd::<u16>(slot + 0x10) == 6 {
            0
        } else {
            slot + 0x10
        };

        if rd::<u32>(entry + 0x70) == 3 && cur_epoch != old_epoch {
            exe::call_va!(0x1_41e0_f660, (this, entry));
        } else {
            let changed = if rd::<u32>(entry + 0x5c) != 0 {
                (exe::call_va!(0x1_41e0_f750, (this, arg2, table, val_ptr, entry, arg4) -> u64)
                    & 0xff)
                    != 0
            } else {
                let force = rd::<u8>(entry + 0x74);
                let c = if val_ptr != 0 {
                    if force != 0 {
                        exe::call_va!(0x1_41e1_53a0, (table, val_ptr, entry));
                        true
                    } else {
                        (fact_variant_compare_and_set(table, val_ptr, entry) & 0xff) != 0
                    }
                } else {
                    exe::call_va!(0x1_41e1_5170, (table, entry));
                    force != 0
                };
                wr::<u8>(entry + 0x74, 0);
                c
            };
            if changed {
                exe::call_va!(0x1_41e0_f660, (this, entry));
            }
        }
        entry = entry.wrapping_add(0xd0);
    }

    let tree_head_ptr = rd::<u64>(this + 0x20);
    let mut node = rd::<u64>(tree_head_ptr);
    if rd::<u8>(node + 0x19) != 0 {
        wr::<u32>(this + 0x38, cur_epoch);
        return 0;
    }

    let two_pow_63 = f64::from_bits(0x43e0_0000_0000_0000);

    loop {
        let obj = rd::<u64>(node + 0x28);
        let found = exe::call_va!(0x1_401e_8210, (table, obj + 0x58) -> u64);
        let size_mismatch = if found == 0 {
            false
        } else {
            let target_f = f64::from_bits(rd::<u64>(found + 8));
            let vtbl = rd::<u64>(obj);
            let cur_len = exe::call_va!(rd::<u64>(vtbl + 8), (obj) -> u64);
            let cur_len_f = cur_len as f64;
            target_f.is_nan() || cur_len_f.is_nan() || target_f != cur_len_f
        };

        let kind = rd::<i32>(obj + 0x68);
        let kind_changed = match kind {
            0 | 1 | 2 | 4 | 5 | 6 => {
                (exe::call_va!(0x1_41e0_ee10, (this, arg2, table, obj, arg4, 0u64) -> u64) & 0xff)
                    != 0
            }
            3 => {
                (exe::call_va!(
                    0x1_41e0_ee10,
                    (this, arg2, table, obj, arg4, ne_flag as u64) -> u64
                ) & 0xff)
                    != 0
            }
            7 => {
                (exe::call_va!(
                    0x1_41e0_ebb0,
                    (this, arg2, table, obj, arg4, ne_flag as u64) -> u64
                ) & 0xff)
                    != 0
            }
            _ => false,
        };

        if size_mismatch || kind_changed {
            let vtbl = rd::<u64>(obj);
            let cur_len = exe::call_va!(rd::<u64>(vtbl + 8), (obj) -> u64);
            let found2 = exe::call_va!(0x1_401e_8210, (table, obj + 0x58) -> u64);
            if found2 != 0 {
                let d = f64::from_bits(rd::<u64>(found2 + 8));
                let target_len = if d >= two_pow_63 && (d - two_pow_63) < two_pow_63 {
                    ((d - two_pow_63) as i64 as u64).wrapping_add(0x8000_0000_0000_0000)
                } else {
                    d as i64 as u64
                };
                if cur_len < target_len {
                    let mut rem = target_len - cur_len;
                    while rem != 0 {
                        exe::call_va!(0x1_41e1_3f00, (rd::<u64>(this + 0x30), obj));
                        rem -= 1;
                    }
                } else if cur_len > target_len {
                    let k = rd::<u32>(obj + 0x68);
                    if k == 3 {
                        exe::call_va!(0x1_40f6_49d0, (0u64, table, obj, target_len));
                    } else if k == 7 {
                        let list_len = rd::<u64>(obj + 0x78);
                        if target_len < list_len {
                            let mut target_node = rd::<u64>(obj + 0x70);
                            let mut steps = list_len - target_len;
                            while steps != 0 {
                                target_node = rd::<u64>(target_node + 8);
                                steps -= 1;
                            }
                            let mut cur_node = rd::<u64>(obj + 0x70);
                            while cur_node != target_node {
                                let next_node = rd::<u64>(cur_node + 8);
                                let sub_ptr = rd::<u64>(next_node + 0x10);
                                let sub_cnt = rd::<u32>(next_node + 0x18) as u64;
                                let sub_end = sub_ptr.wrapping_add(sub_cnt.wrapping_mul(8));
                                let mut p = sub_ptr;
                                while p != sub_end {
                                    let item = rd::<u64>(p);
                                    exe::call_va!(0x1_403f_4030, (table, item + 0x60));
                                    p = p.wrapping_add(8);
                                }
                                cur_node = next_node;
                            }
                            exe::call_va!(0x1_40f6_6c00, (obj + 0x70, target_len));
                        }
                    } else {
                        exe::call_va!(0x1_40f6_49d0, (0u64, table, obj, target_len));
                    }
                }
            }
            exe::call_va!(0x1_41e0_f470, (this, obj));
        }

        node = rbtree_next(node);
        if rd::<u8>(node + 0x19) != 0 {
            break;
        }
    }

    wr::<u32>(this + 0x38, cur_epoch);
    0
}

// ---------------------------------------------------------------------------
// 6. 0x1420b2b50 — ecs_bind_groups_match
// ---------------------------------------------------------------------------

#[inline]
unsafe fn slice_contains_u16(base: u64, count: u64, needle: u16) -> bool {
    let mut i = 0u64;
    while i < count {
        if rd::<u16>(base.wrapping_add(i.wrapping_mul(2))) == needle {
            return true;
        }
        i += 1;
    }
    false
}

/// `sub_1420b2b50` — original @ 0x1420b2b50 (568 bytes).
/// Matches candidate ECS group IDs in `cand_span` (`rdx`) against required/excluded component
/// `u16` arrays in `queries` (`rcx`) using the group table at `world_ctx + 0x18050` (`r8`),
/// appending `(group_id, query_tag)` pairs to the per-worker output vector at `[out_ctx + 8] + worker*16`.
pub unsafe extern "win64" fn ecs_bind_groups_match(
    queries: u64,
    cand_span: u64,
    world_ctx: u64,
    out_ctx: u64,
) -> u64 {
    let wid = crate::ns::r::thread::worker_index() as u32 as u64;
    let worker_vec = rd::<u64>(out_ctx + 8).wrapping_add(wid.wrapping_shl(4));

    let q_count = rd::<u64>(queries);
    let mut q_ptr = rd::<u64>(queries + 8);
    let q_end = q_ptr.wrapping_add(q_count.wrapping_mul(40));
    if q_ptr == q_end {
        return 0;
    }

    let c_count = rd::<u64>(cand_span);
    let c_base = rd::<u64>(cand_span + 8);
    let c_end = c_base.wrapping_add(c_count.wrapping_mul(2));

    while q_ptr != q_end {
        let mut c_ptr = c_base;
        while c_ptr != c_end {
            let gid = rd::<u16>(c_ptr);
            let g_off = (gid as u64).wrapping_shl(5);
            let comp_count = rd::<u32>(world_ctx.wrapping_add(g_off).wrapping_add(0x18068)) as u64;
            let comp_ptr = rd::<u64>(world_ctx.wrapping_add(g_off).wrapping_add(0x18050));

            let req_count = rd::<u64>(q_ptr);
            let mut req_ptr = rd::<u64>(q_ptr + 8);
            let req_end = req_ptr.wrapping_add(req_count.wrapping_mul(2));
            let mut matched = true;
            while req_ptr != req_end {
                let req_id = rd::<u16>(req_ptr);
                if !slice_contains_u16(comp_ptr, comp_count, req_id) {
                    matched = false;
                    break;
                }
                req_ptr = req_ptr.wrapping_add(2);
            }

            if matched {
                let exc_count = rd::<u64>(q_ptr + 0x10);
                let mut exc_ptr = rd::<u64>(q_ptr + 0x18);
                let exc_end = exc_ptr.wrapping_add(exc_count.wrapping_mul(2));
                while exc_ptr != exc_end {
                    let exc_id = rd::<u16>(exc_ptr);
                    if slice_contains_u16(comp_ptr, comp_count, exc_id) {
                        matched = false;
                        break;
                    }
                    exc_ptr = exc_ptr.wrapping_add(2);
                }
            }

            if matched {
                let q_tag = rd::<u16>(q_ptr + 0x20);
                let packed = (gid as u32) | ((q_tag as u32) << 16);
                let cur_len = rd::<u32>(worker_vec + 8) as u64;
                let cur_cap = rd::<u32>(worker_vec + 0x0c) as u64;
                if cur_len == cur_cap {
                    exe::call_va!(0x1_411f_0270, (worker_vec, cur_len + 1));
                }
                let data_ptr = rd::<u64>(worker_vec);
                let write_idx = rd::<u32>(worker_vec + 8) as u64;
                wr::<u32>(data_ptr.wrapping_add(write_idx.wrapping_mul(4)), packed);
                wr::<u32>(worker_vec + 8, (write_idx as u32).wrapping_add(1));
            }

            c_ptr = c_ptr.wrapping_add(2);
        }
        q_ptr = q_ptr.wrapping_add(40);
    }
    0
}

// ---------------------------------------------------------------------------
// 7. 0x1420a8fd0 — start_scheduling
// ---------------------------------------------------------------------------

/// `sub_1420a8fd0` — original @ 0x1420a8fd0 (497 bytes).
/// Copies the dependency counter array from `sched_graph` (`r9`), enqueues all initial
/// root jobs, signals the worker semaphore, and pumps the scheduler until the root sentinel
/// counter (`counts[count - 1]`) reaches 0.
pub unsafe extern "win64" fn start_scheduling(
    _rcx: u64,
    arg2: u64,
    arg3: u64,
    sched_graph: u64,
) -> u64 {
    let jp = crate::jobprof::begin();
    let count = rd::<u32>(sched_graph + 0x28) as u64;
    let bytes = count.wrapping_mul(4);
    let src_begin = rd::<u64>(sched_graph + 0x20);
    let src_end = src_begin.wrapping_add(bytes);

    let counts_buf = if count != 0 {
        exe::call_va!(0x1_4241_c0a0, (bytes, 4u64) -> u64)
    } else {
        0
    };

    let mut s = src_begin;
    let mut d = counts_buf;
    while s != src_end {
        wr::<u32>(d, rd::<u32>(s));
        d = d.wrapping_add(4);
        s = s.wrapping_add(4);
    }

    let mut ctx = [
        arg2,
        arg3,
        sched_graph,
        ((bytes as i64) >> 2) as u64,
        counts_buf,
    ];
    let ctx_ptr = ctx.as_mut_ptr() as u64;

    let sched_mgr = rd::<u64>(0x1_43a0_82f8);
    let inner_sched = rd::<u64>(sched_mgr + 0x10);
    let roots_begin = rd::<u64>(sched_graph + 0x30);
    let roots_count = rd::<u32>(sched_graph + 0x38) as u64;
    let roots_end = roots_begin.wrapping_add(roots_count.wrapping_mul(2));

    let mut enqueued = 0u64;
    let mut r_ptr = roots_begin;
    while r_ptr != roots_end {
        let root_id = rd::<u16>(r_ptr);
        let map_base = rd::<u64>(sched_graph);
        let mapped = rd::<u16>(map_base.wrapping_add((root_id as u64).wrapping_mul(2)));
        let special_tag = rd::<u16>(0x1_4382_7b24);
        let pool_off = if mapped == special_tag { 0x100u64 } else { 0x80u64 };

        let cur_inner = rd::<u64>(sched_mgr + 0x10);
        let wid = crate::ns::r::thread::worker_index() as u32 as u64;
        let pool = rd::<u64>(cur_inner.wrapping_add(wid.wrapping_shl(6)).wrapping_add(0x48))
            .wrapping_add(pool_off);

        let mut node_handle = [0u64; 2];
        crate::ns::r::thread::job_node_alloc(pool as usize, node_handle.as_mut_ptr() as usize);
        let node_ptr = node_handle[0];
        let atomic_ref = node_ptr as *const core::sync::atomic::AtomicU32;
        (*atomic_ref).swap(1, core::sync::atomic::Ordering::SeqCst);
        wr::<u64>(node_ptr + 0x08, 0);
        wr::<u64>(node_ptr + 0x10, 0x1_433d_d680);
        wr::<u64>(node_ptr + 0x18, ctx_ptr);
        wr::<u64>(node_ptr + 0x20, root_id as u64);

        crate::ns::r::thread::scheduler_enqueue_job(inner_sched as usize, node_ptr as usize);
        enqueued += 1;
        r_ptr = r_ptr.wrapping_add(2);
    }

    if enqueued != 0 {
        let sem_owner = rd::<u64>(inner_sched);
        let sem_handle = rd::<u64>(sem_owner + 0x1008);
        let _ = exe::call_iat!(0x1_4264_5580, (sem_handle, enqueued, 0u64) -> u64);
    }

    loop {
        let buf = ctx[4];
        let cnt = ctx[3];
        let rem = rd::<u32>(buf.wrapping_add(cnt.wrapping_mul(4)).wrapping_sub(4));
        if rem == 0 {
            break;
        }
        let mgr = rd::<u64>(0x1_43a0_82f8);
        crate::ns::r::thread::scheduler_pump_one(mgr as usize);
    }
    crate::jobprof::end(jp, crate::jobprof::K_GRAPH, sched_graph, count as u16, 0);

    if counts_buf != 0 {
        exe::call_va!(0x1_4241_c190, (counts_buf));
    }
    0
}

// ---------------------------------------------------------------------------
// 8. 0x1420a9960 — schedule_node_complete
// ---------------------------------------------------------------------------

#[repr(C)]
struct InlineU16Worklist {
    ptr: u64,
    len: u32,
    cap: u32,
    inline: [u16; 128],
}

impl InlineU16Worklist {
    #[inline]
    unsafe fn push(&mut self, val: u16) {
        if self.len == self.cap {
            let cur_len = self.len as u64;
            let cur_cap = self.cap as u64;
            let next_len = cur_len + 1;
            let mut new_cap = cur_cap + (cur_cap >> 1);
            if new_cap < next_len {
                new_cap = next_len;
            }
            let inline_ptr = self.inline.as_mut_ptr() as u64;
            let (new_ptr, final_cap) = if new_cap <= 0x80 && self.ptr != inline_ptr {
                (inline_ptr, 0x80u32)
            } else {
                (
                    exe::call_va!(0x1_4028_e1c0, (0u64, new_cap) -> u64),
                    new_cap as u32,
                )
            };
            core::ptr::copy(
                self.ptr as *const u8,
                new_ptr as *mut u8,
                (cur_len * 2) as usize,
            );
            exe::call_va!(0x1_411e_d7e0, ((self as *mut Self) as u64));
            self.ptr = new_ptr;
            self.len = cur_len as u32;
            self.cap = final_cap;
        }
        let idx = self.len as u64;
        wr::<u16>(self.ptr.wrapping_add(idx.wrapping_mul(2)), val);
        self.len += 1;
    }
}

/// `sub_1420a9960` — original @ 0x1420a9960 (1740 bytes).
/// Executes a ready job-graph node (`node_id` in `dx`), dispatches its task or sub-group
/// jobs, and decrements dependent node counters, looping until all newly-ready nodes are
/// dispatched or `sentinel_id` (`r8w`) is reached.
pub unsafe extern "win64" fn schedule_node_complete(
    ctx: u64,
    node_id: u64,
    sentinel_id: u64,
) {
    let sentinel = sentinel_id as u16;
    let sched_mgr = rd::<u64>(0x1_43a0_82f8);
    let inner_sched = rd::<u64>(sched_mgr + 0x10);

    let mut wl: InlineU16Worklist = core::mem::zeroed();
    let inline_ptr = wl.inline.as_mut_ptr() as u64;
    wl.ptr = inline_ptr;
    wl.len = 1;
    wl.cap = 0x80;
    wl.inline[0] = node_id as u16;

    while wl.len != 0 {
        if wl.len > 1 {
            let total = wl.len as u64;
            let mut idx = 1u64;
            let mut spawned = 0u64;
            while idx != total {
                let extra_id = rd::<u16>(wl.ptr.wrapping_add(idx.wrapping_mul(2)));
                let sched_graph = rd::<u64>(ctx + 0x10);
                let map_base = rd::<u64>(sched_graph);
                let mapped = rd::<u16>(map_base.wrapping_add((extra_id as u64).wrapping_mul(2)));
                let pool_off = if mapped == rd::<u16>(0x1_4382_7b24) {
                    0x100u64
                } else {
                    0x80u64
                };
                let cur_inner = rd::<u64>(sched_mgr + 0x10);
                let wid = crate::ns::r::thread::worker_index() as u32 as u64;
                let pool = rd::<u64>(cur_inner.wrapping_add(wid.wrapping_shl(6)).wrapping_add(0x48))
                    .wrapping_add(pool_off);
                let mut job_handle = [0u64; 2];
                crate::ns::r::thread::job_node_alloc(
                    pool as usize,
                    job_handle.as_mut_ptr() as usize,
                );
                let job_ptr = job_handle[0];
                let atomic_ref = job_ptr as *const core::sync::atomic::AtomicU32;
                (*atomic_ref).swap(1, core::sync::atomic::Ordering::SeqCst);
                wr::<u64>(job_ptr + 0x08, 0);
                wr::<u64>(job_ptr + 0x10, 0x1_433d_d620);
                wr::<u64>(job_ptr + 0x18, ctx);
                wr::<u64>(job_ptr + 0x20, extra_id as u64);
                crate::ns::r::thread::scheduler_enqueue_job(inner_sched as usize, job_ptr as usize);
                spawned += 1;
                idx += 1;
            }
            if spawned != 0 {
                let sem_owner = rd::<u64>(inner_sched);
                let _ = exe::call_iat!(0x1_4264_5580, (rd::<u64>(sem_owner + 0x1008), spawned, 0u64) -> u64);
            }
            wl.len = 1;
        }

        let cur_idx = (wl.len - 1) as u64;
        let cur_node = rd::<u16>(wl.ptr.wrapping_add(cur_idx.wrapping_mul(2)));
        wl.len -= 1;
        if cur_node == sentinel {
            break;
        }

        let sched_graph = rd::<u64>(ctx + 0x10);
        let map_base = rd::<u64>(sched_graph);
        let sys_id = rd::<u16>(map_base.wrapping_add((cur_node as u64).wrapping_mul(2)));
        let ctx0 = rd::<u64>(ctx);
        let sys_table = rd::<u64>(rd::<u64>(ctx0 + 0x08) + 0x08);
        let sys_desc = sys_table.wrapping_add((sys_id as u64).wrapping_mul(0x130));

        let f114 = rd::<u8>(sys_desc + 0x114) != 0 || rd::<u8>(sys_desc + 0x116) != 0;
        let f115 = rd::<u8>(sys_desc + 0x115) != 0;
        let f117 = rd::<u8>(sys_desc + 0x117) != 0;

        if f115 && !f117 && !f114 {
            let group_table = rd::<u64>(ctx0 + 0x10);
            let g_off = (sys_id as u64).wrapping_mul(6).wrapping_mul(8);
            let mut g_ptr = rd::<u64>(group_table.wrapping_add(g_off));
            let g_cnt = rd::<u32>(group_table.wrapping_add(g_off + 8)) as u64;
            let g_end = g_ptr.wrapping_add(g_cnt.wrapping_mul(2));
            let mut spawned_groups = 0u64;

            while g_ptr != g_end {
                let gid = rd::<u16>(g_ptr);
                let ctx1 = rd::<u64>(ctx + 0x08);
                let state_base = rd::<u64>(ctx1);
                if rd::<u64>(state_base.wrapping_add((gid as u64).wrapping_mul(8) + 0x88)) != 0
                    && rd::<u32>(state_base.wrapping_add((gid as u64).wrapping_mul(4) + 0x10088))
                        != 0
                {
                    let sg = rd::<u64>(ctx + 0x10);
                    let dep_span = rd::<u64>(sg + 0x10).wrapping_add((cur_node as u64).wrapping_shl(4));
                    let mut d_ptr = rd::<u64>(dep_span);
                    let d_cnt = rd::<u32>(dep_span + 8) as u64;
                    let d_end = d_ptr.wrapping_add(d_cnt.wrapping_mul(2));
                    while d_ptr != d_end {
                        let dep_node = rd::<u16>(d_ptr) as u64;
                        let total_nodes = rd::<u32>(sg + 8) as u64;
                        let bit_set = if dep_node < total_nodes {
                            let c0 = rd::<u64>(ctx);
                            let dep_sys = rd::<u16>(rd::<u64>(sg).wrapping_add(dep_node * 2)) as u64;
                            let bit_idx = (gid as u64)
                                .wrapping_mul(rd::<u64>(c0 + 0xa8))
                                .wrapping_add(dep_sys);
                            let word = rd::<u64>(rd::<u64>(c0 + 0x70).wrapping_add((bit_idx >> 6) * 8));
                            ((word >> (bit_idx & 63)) & 1) != 0
                        } else {
                            true
                        };
                        if bit_set {
                            let counts = rd::<u64>(ctx + 0x20);
                            let p = counts.wrapping_add(dep_node * 4)
                                as *const core::sync::atomic::AtomicU32;
                            (*p).fetch_add(1, core::sync::atomic::Ordering::SeqCst);
                        }
                        d_ptr = d_ptr.wrapping_add(2);
                    }

                    let mapped = rd::<u16>(rd::<u64>(sg).wrapping_add((cur_node as u64) * 2));
                    let pool_off = if mapped == rd::<u16>(0x1_4382_7b24) {
                        0x100u64
                    } else {
                        0x80u64
                    };
                    let cur_inner = rd::<u64>(sched_mgr + 0x10);
                    let wid = crate::ns::r::thread::worker_index() as u32 as u64;
                    let pool = rd::<u64>(
                        cur_inner.wrapping_add(wid.wrapping_shl(6)).wrapping_add(0x48),
                    )
                    .wrapping_add(pool_off);
                    let mut job_handle = [0u64; 2];
                    crate::ns::r::thread::job_node_alloc(
                        pool as usize,
                        job_handle.as_mut_ptr() as usize,
                    );
                    let job_ptr = job_handle[0];
                    let atomic_ref = job_ptr as *const core::sync::atomic::AtomicU32;
                    (*atomic_ref).swap(1, core::sync::atomic::Ordering::SeqCst);
                    wr::<u64>(job_ptr + 0x08, 0);
                    wr::<u64>(job_ptr + 0x10, 0x1_433d_d650);
                    wr::<u64>(job_ptr + 0x18, ctx);
                    let packed_args = (gid as u64)
                        | ((cur_node as u64) << 16)
                        | ((sentinel as u64) << 32);
                    wr::<u64>(job_ptr + 0x20, packed_args);
                    crate::ns::r::thread::scheduler_enqueue_job(
                        inner_sched as usize,
                        job_ptr as usize,
                    );
                    spawned_groups += 1;
                }
                g_ptr = g_ptr.wrapping_add(2);
            }
            if spawned_groups != 0 {
                let sem_owner = rd::<u64>(inner_sched);
                let _ = exe::call_iat!(0x1_4264_5580, (rd::<u64>(sem_owner + 0x1008), spawned_groups, 0u64) -> u64);
            }
        } else if sys_id == rd::<u16>(0x1_4382_7b24) {
            let cur_inner = rd::<u64>(sched_mgr + 0x10);
            let wid = crate::ns::r::thread::worker_index() as u32 as u64;
            let pool = rd::<u64>(cur_inner.wrapping_add(wid.wrapping_shl(6)).wrapping_add(0x48))
                .wrapping_add(0x100);
            let mut job_handle = [0u64; 2];
            crate::ns::r::thread::job_node_alloc(
                pool as usize,
                job_handle.as_mut_ptr() as usize,
            );
            let job_ptr = job_handle[0];
            let atomic_ref = job_ptr as *const core::sync::atomic::AtomicU32;
            (*atomic_ref).swap(1, core::sync::atomic::Ordering::SeqCst);
            wr::<u64>(job_ptr + 0x08, 0);
            wr::<u64>(job_ptr + 0x10, 0x1_433d_d668);
            wr::<u64>(job_ptr + 0x18, ctx);
            wr::<u64>(job_ptr + 0x20, sys_id as u64);
            if rd::<u32>(job_ptr) == 1 {
                let inner2 = rd::<u64>(sched_mgr + 0x10);
                crate::ns::r::thread::scheduler_enqueue_job(inner2 as usize, job_ptr as usize);
                let sem_owner = rd::<u64>(inner2);
                let _ = exe::call_iat!(0x1_4264_5580, (rd::<u64>(sem_owner + 0x1008), 1u64, 0u64) -> u64);
            }
            crate::ns::r::thread::scheduler_wait_handle_any(
                sched_mgr as usize,
                job_handle.as_mut_ptr() as usize,
            );
        } else {
            let fn_ptr = rd::<u64>(sys_desc + 0x120);
            let arg_r8 = rd::<u16>(sys_desc) as u64;
            let jp = crate::jobprof::begin();
            exe::call_va!(fn_ptr, (rd::<u64>(ctx + 0x08), ctx0 + 0x10, arg_r8));
            crate::jobprof::end(jp, crate::jobprof::K_SYSTEM, fn_ptr, sys_id, 0);
        }

        let sg = rd::<u64>(ctx + 0x10);
        let dep_span = rd::<u64>(sg + 0x10).wrapping_add((cur_node as u64).wrapping_shl(4));
        let mut d_ptr = rd::<u64>(dep_span);
        let d_cnt = rd::<u32>(dep_span + 8) as u64;
        let d_end = d_ptr.wrapping_add(d_cnt.wrapping_mul(2));
        while d_ptr != d_end {
            let succ_id = rd::<u16>(d_ptr);
            let counts = rd::<u64>(ctx + 0x20);
            let p = counts.wrapping_add((succ_id as u64) * 4)
                as *const core::sync::atomic::AtomicU32;
            let prev = (*p).fetch_sub(1, core::sync::atomic::Ordering::SeqCst);
            if prev == 1 {
                wl.push(succ_id);
            }
            d_ptr = d_ptr.wrapping_add(2);
        }
    }

    exe::call_va!(0x1_411e_d7e0, ((&mut wl as *mut InlineU16Worklist) as u64));
}

/// `r::JobCallable<anonymous_namespace::schedule::<lambda_2>>::vfunc_1` — original @ 0x1420a8ef0 (9 bytes).
pub unsafe extern "win64" fn schedule_group_node_vfunc_1(this: u64) {
    exe::call_va!(0x1_420a_95e0, (this.wrapping_add(8)));
}

/// `sub_1420a95e0` — original @ 0x1420a95e0 (818 bytes).
/// Executes a per-group ECS job-graph node (`param_1` points to `[ctx, gid: u16, cur_node: u16, sentinel: u16]`),
/// decrements dependent node counters whose group-dependency bit is set, and either runs a single
/// newly-ready successor inline via `schedule_node_complete` (`0x1420a9960`) or enqueues multiple
/// ready successor jobs into the worker scheduler.
pub unsafe extern "win64" fn schedule_group_node_run(param_1: u64) {
    let ctx = rd::<u64>(param_1);
    let sched_graph = rd::<u64>(ctx + 0x10);
    let cur_node = rd::<u16>(param_1 + 0x0a) as u64;
    let map_base = rd::<u64>(sched_graph);
    let ctx0 = rd::<u64>(ctx);
    let sys_box = rd::<u64>(ctx0 + 0x08);
    let sys_id = rd::<u16>(map_base.wrapping_add(cur_node.wrapping_mul(2))) as u64;
    let sys_off = sys_id.wrapping_mul(0x130);
    let sys_table = rd::<u64>(sys_box + 0x08);
    let fn_ptr = rd::<u64>(sys_table.wrapping_add(sys_off).wrapping_add(0x128));
    let gid = rd::<u16>(param_1 + 0x08) as u64;
    let jp = crate::jobprof::begin();
    exe::call_va!(fn_ptr, (rd::<u64>(ctx + 0x08), gid, sys_off, fn_ptr));
    crate::jobprof::end(jp, crate::jobprof::K_GROUP, fn_ptr, sys_id as u16, gid as u16);

    let mut wl: InlineU16Worklist = core::mem::zeroed();
    let inline_ptr = wl.inline.as_mut_ptr() as u64;
    wl.ptr = inline_ptr;
    wl.len = 0;
    wl.cap = 0x80;

    let ctx_cur = rd::<u64>(param_1);
    let sg = rd::<u64>(ctx_cur + 0x10);
    let cur_node2 = rd::<u16>(param_1 + 0x0a) as u64;
    let dep_span = rd::<u64>(sg + 0x10).wrapping_add(cur_node2.wrapping_shl(4));
    let mut d_ptr = rd::<u64>(dep_span);
    let d_cnt = rd::<u32>(dep_span + 8) as u64;
    let d_end = d_ptr.wrapping_add(d_cnt.wrapping_mul(2));

    while d_ptr != d_end {
        let dep_node = rd::<u16>(d_ptr);
        let dep_idx = dep_node as u64;
        let c = rd::<u64>(param_1);
        let sg_inner = rd::<u64>(c + 0x10);
        let map_inner = rd::<u64>(sg_inner);
        let total_nodes = rd::<u32>(sg_inner + 0x08) as u64;
        let bit_set = if dep_idx < total_nodes {
            let c0 = rd::<u64>(c);
            let gid_val = rd::<u16>(param_1 + 0x08) as u64;
            let dep_sys = rd::<u16>(map_inner.wrapping_add(dep_idx.wrapping_mul(2))) as u64;
            let bit_idx = gid_val
                .wrapping_mul(rd::<u64>(c0 + 0xa8))
                .wrapping_add(dep_sys);
            let word = rd::<u64>(rd::<u64>(c0 + 0x70).wrapping_add((bit_idx >> 6).wrapping_mul(8)));
            ((word >> (bit_idx & 0x3f)) & 1) != 0
        } else {
            true
        };
        if bit_set {
            let counts = rd::<u64>(c + 0x20);
            let p = counts.wrapping_add(dep_idx.wrapping_mul(4))
                as *const core::sync::atomic::AtomicU32;
            let prev = (*p).fetch_sub(1, core::sync::atomic::Ordering::SeqCst);
            if prev == 1 {
                wl.push(dep_node);
            }
        }
        d_ptr = d_ptr.wrapping_add(2);
    }

    let len = wl.len as u64;
    if len == 1 {
        let succ_node = rd::<u16>(wl.ptr) as u64;
        let sentinel = rd::<u16>(param_1 + 0x0c) as u64;
        exe::call_va!(0x1_420a_9960, (rd::<u64>(param_1), succ_node, sentinel));
    } else if len > 1 {
        let sched_mgr = rd::<u64>(exe::ptr::<u64>(0x1_43a0_82f8) as u64);
        let inner_sched = rd::<u64>(sched_mgr + 0x10);
        let mut spawned = 0u64;
        let mut idx = 0u64;
        while idx < len {
            let extra_id = rd::<u16>(wl.ptr.wrapping_add(idx.wrapping_mul(2)));
            let c = rd::<u64>(param_1);
            let sentinel = rd::<u16>(param_1 + 0x0c);
            let sg_inner = rd::<u64>(c + 0x10);
            let mapped = rd::<u16>(rd::<u64>(sg_inner).wrapping_add((extra_id as u64).wrapping_mul(2)));
            let pool_off = if mapped == rd::<u16>(exe::ptr::<u16>(0x1_4382_7b24) as u64) {
                0x100u64
            } else {
                0x80u64
            };
            let cur_inner = rd::<u64>(sched_mgr + 0x10);
            let wid = crate::ns::r::thread::worker_index() as u32 as u64;
            let pool = rd::<u64>(cur_inner.wrapping_add(wid.wrapping_shl(6)).wrapping_add(0x48))
                .wrapping_add(pool_off);
            let mut job_handle = [0u64; 2];
            crate::ns::r::thread::job_node_alloc(
                pool as usize,
                job_handle.as_mut_ptr() as usize,
            );
            let job_ptr = job_handle[0];
            let atomic_ref = job_ptr as *const core::sync::atomic::AtomicU32;
            (*atomic_ref).swap(1, core::sync::atomic::Ordering::SeqCst);
            wr::<u64>(job_ptr + 0x08, 0);
            wr::<u64>(job_ptr + 0x10, exe::ptr::<u8>(0x1_433d_d638) as u64);
            wr::<u64>(job_ptr + 0x18, c);
            wr::<u64>(job_ptr + 0x20, (extra_id as u64) | ((sentinel as u64) << 16));
            crate::ns::r::thread::scheduler_enqueue_job(inner_sched as usize, job_ptr as usize);
            spawned += 1;
            idx += 1;
        }
        if spawned != 0 {
            let sem_owner = rd::<u64>(inner_sched);
            let _ = exe::call_iat!(0x1_4264_5580, (rd::<u64>(sem_owner + 0x1008), spawned, 0u64) -> u64);
        }
    }

    exe::call_va!(0x1_411e_d7e0, ((&mut wl as *mut InlineU16Worklist) as u64));
}

// ---------------------------------------------------------------------------
// 9. 0x142422290 — streamer_resource_tick
// ---------------------------------------------------------------------------

/// `sub_142422290` — original @ 0x142422290 (2556 bytes).
/// Main-thread streaming resource state-machine tick: deduplicates pending resources,
/// advances states within a 2ms/4ms frame budget, collects completed callbacks, and
/// updates dirty flags under the 4 streaming SRWLocks.
pub unsafe extern "win64" fn streamer_resource_tick(this: u64) -> u64 {
    if (exe::call_va!(0x1_4242_2ca0, (this) -> u64) & 0xff) == 0 {
        return 0;
    }

    let lock_1b0 = this + 0x1b0;
    exe::call_iat!(0x1_4264_55a8, (lock_1b0));

    let init_cnt = rd::<u32>(this + 0x180) as u64;
    let init_ptr = rd::<u64>(this + 0x178);
    let init_end = init_ptr.wrapping_add(init_cnt.wrapping_mul(8));
    if init_cnt <= 0x20 {
        let mut out_it = 0u64;
        let mut b = init_ptr;
        let mut e = init_end;
        let _ = exe::call_va!(
            0x1_413a_00c0,
            (
                (&mut out_it as *mut u64) as u64,
                (&mut b as *mut u64) as u64,
                (&mut e as *mut u64) as u64
            ) -> u64
        );
    } else {
        let mut half = init_cnt - (init_cnt >> 1);
        let mut stack_buf = [0u64; 512];
        let mut heap_ptr = 0u64;
        if half > 0x200 {
            while half != 0 {
                let p = exe::call_va!(0x1_4183_50bc, (half.wrapping_mul(8), 0x1_426c_cee0u64) -> u64);
                if p != 0 {
                    heap_ptr = p;
                    break;
                }
                half >>= 1;
            }
            if heap_ptr == 0 || half <= 0x200 {
                if heap_ptr != 0 {
                    exe::call_va!(0x1_4183_50d0, (heap_ptr));
                    heap_ptr = 0;
                }
                half = 0x200;
            }
        }
        let buf_ptr = if heap_ptr != 0 {
            heap_ptr
        } else {
            stack_buf.as_mut_ptr() as u64
        };
        let mut b = init_ptr;
        let mut e = init_end;
        exe::call_va!(
            0x1_413a_0190,
            (
                (&mut b as *mut u64) as u64,
                (&mut e as *mut u64) as u64,
                init_cnt,
                buf_ptr,
                half,
                0u64
            )
        );
        if half > 0x200 && heap_ptr != 0 {
            exe::call_va!(0x1_4183_50d0, (heap_ptr));
        }
    }

    let cnt_after_sort = rd::<u32>(this + 0x180) as u64;
    let base_ptr = rd::<u64>(this + 0x178);
    let end_ptr = base_ptr.wrapping_add(cnt_after_sort.wrapping_mul(8));
    if base_ptr != end_ptr {
        let mut prev = base_ptr;
        let mut scan = base_ptr.wrapping_add(8);
        while scan != end_ptr {
            if rd::<u64>(prev) == rd::<u64>(scan) {
                let mut r = scan.wrapping_add(8);
                while r != end_ptr {
                    let v = rd::<u64>(r);
                    if rd::<u64>(prev) != v {
                        prev = prev.wrapping_add(8);
                        wr::<u64>(prev, v);
                    }
                    r = r.wrapping_add(8);
                }
                let new_end = prev.wrapping_add(8);
                let cur_base = rd::<u64>(this + 0x178);
                let cur_cnt = rd::<u32>(this + 0x180) as u64;
                if new_end != end_ptr {
                    if new_end == cur_base && end_ptr == cur_base.wrapping_add(cur_cnt * 8) {
                        wr::<u32>(this + 0x180, 0);
                    } else {
                        let tail_bytes = cur_base
                            .wrapping_add(cur_cnt * 8)
                            .wrapping_sub(end_ptr);
                        if tail_bytes != 0 {
                            core::ptr::copy(
                                end_ptr as *const u8,
                                new_end as *mut u8,
                                tail_bytes as usize,
                            );
                        }
                        let removed = (end_ptr.wrapping_sub(new_end)) >> 3;
                        let updated = rd::<u32>(this + 0x180).wrapping_sub(removed as u32);
                        wr::<u32>(this + 0x180, updated);
                    }
                }
                break;
            }
            prev = scan;
            scan = scan.wrapping_add(8);
        }
    }

    if rd::<u32>(this + 0x180) != 0 {
        let mut start_qpc = 0u64;
        let _ = exe::call_iat!(0x1_4264_56f8, ((&mut start_qpc as *mut u64) as u64) -> u64);
        let _ = crate::ns::r::thread::perf_frequency();
        let budget_f32 = if rd::<u8>(this + 0x220) == 0 {
            f32::from_bits(0x3b03_126f)
        } else {
            f32::from_bits(0x3b83_126f)
        };
        let budget = budget_f32 as f64;

        let g_2f0 = rd::<u64>(0x1_43a0_82f0);
        if g_2f0 != 0 {
            let l_base = rd::<u64>(g_2f0 + 0x4e8);
            let l_cnt = rd::<u32>(g_2f0 + 0x4f0) as u64;
            let l_end = l_base.wrapping_add(l_cnt.wrapping_mul(8));
            let mut lp = l_base;
            while lp != l_end {
                let obj = rd::<u64>(lp);
                exe::call_va!(rd::<u64>(rd::<u64>(obj) + 0x18), (obj));
                lp = lp.wrapping_add(8);
            }
        }

        let mut idx: i32 = 0;

        while (idx as i64 as u64) < (rd::<u32>(this + 0x180) as u64) {
            let mut now_qpc = 0u64;
            let _ = exe::call_iat!(0x1_4264_56f8, ((&mut now_qpc as *mut u64) as u64) -> u64);
            let freq = crate::ns::r::thread::perf_frequency() as i64 as f64;
            let elapsed = (now_qpc.wrapping_sub(start_qpc) as i64 as f64) / freq;
            if budget <= elapsed {
                break;
            }

            let i_u64 = idx as i64 as u64;
            let arr = rd::<u64>(this + 0x178);
            let res = rd::<u64>(arr.wrapping_add(i_u64.wrapping_mul(8)));
            let st = rd::<u32>(res + 0x80);
            let mut remove_from_vec = true;

            if st == 1 || st == 3 {
                exe::call_va!(rd::<u64>(rd::<u64>(res) + 0x08), (res));
                if rd::<u32>(res + 0x80) == 3 {
                    if (exe::call_va!(0x1_4241_df90, (rd::<u64>(res + 0x28)) -> u64) & 0xff) != 0 {
                        let p = (res + 0x80) as *const core::sync::atomic::AtomicU32;
                        (*p).swap(1, core::sync::atomic::Ordering::SeqCst);
                        remove_from_vec = false;
                    } else {
                        let vf8_ret = exe::call_va!(rd::<u64>(rd::<u64>(res) + 0x40), (res) -> u64);
                        let slot_idx = (vf8_ret as i32 as i64 as u64).wrapping_shl(6);
                        let rlock = this.wrapping_add(0xe0).wrapping_add(slot_idx);
                        let tid = exe::call_iat!(0x1_4264_5790, () -> u64) as u32;
                        if rd::<u32>(rlock + 8) != tid {
                            exe::call_iat!(0x1_4264_55a8, (rlock));
                            wr::<u32>(rlock + 8, tid);
                        }
                        wr::<u32>(rlock + 0x0c, rd::<u32>(rlock + 0x0c).wrapping_add(1));

                        let list_box = this.wrapping_add(0xb0).wrapping_add(slot_idx);
                        let f18 = rd::<u64>(list_box + 0x18);
                        let mut node = if f18 != 0 && f18 != list_box {
                            exe::call_va!(0x1_411e_3360, (f18) -> u64)
                        } else {
                            rd::<u64>(list_box + 0x10)
                        };

                        let mut matched_listener = 0u64;
                        let mut cur_listener = rd::<u64>(node);
                        while cur_listener != 0 {
                            node = rd::<u64>(node + 0x10);
                            if (exe::call_va!(
                                rd::<u64>(rd::<u64>(cur_listener) + 0x28),
                                (cur_listener) -> u64
                            ) & 0xff)
                                != 0
                            {
                                matched_listener = cur_listener;
                                break;
                            }
                            cur_listener = rd::<u64>(node);
                        }

                        let new_depth = rd::<u32>(rlock + 0x0c).wrapping_sub(1);
                        wr::<u32>(rlock + 0x0c, new_depth);
                        if new_depth == 0 {
                            let p = (rlock + 8) as *const core::sync::atomic::AtomicU32;
                            (*p).swap(u32::MAX, core::sync::atomic::Ordering::SeqCst);
                            exe::call_iat!(0x1_4264_5598, (rlock));
                        }

                        if matched_listener != 0 {
                            exe::call_va!(
                                0x1_4241_ce70,
                                (rd::<u64>(matched_listener + 0x28), matched_listener)
                            );
                        }
                        let p = (res + 0x80) as *const core::sync::atomic::AtomicU32;
                        (*p).swap(1, core::sync::atomic::Ordering::SeqCst);
                        remove_from_vec = false;
                    }
                } else {
                    exe::call_va!(rd::<u64>(rd::<u64>(res) + 0x10), (res));
                }
            }

            if remove_from_vec {
                let last_idx = (rd::<u32>(this + 0x180) as u64).wrapping_sub(1);
                let base = rd::<u64>(this + 0x178);
                let last_ptr = base.wrapping_add(last_idx.wrapping_mul(8));
                let cur_ptr = base.wrapping_add(i_u64.wrapping_mul(8));
                if cur_ptr != last_ptr {
                    wr::<u64>(cur_ptr, rd::<u64>(last_ptr));
                }
                wr::<u32>(this + 0x180, rd::<u32>(this + 0x180).wrapping_sub(1));
                idx -= 1;
            }
            idx += 1;
        }

        let g_2f0_end = rd::<u64>(0x1_43a0_82f0);
        if g_2f0_end != 0 {
            let l_base = rd::<u64>(g_2f0_end + 0x4e8);
            let l_cnt = rd::<u32>(g_2f0_end + 0x4f0) as u64;
            let l_end = l_base.wrapping_add(l_cnt.wrapping_mul(8));
            let mut lp = l_base;
            while lp != l_end {
                let obj = rd::<u64>(lp);
                exe::call_va!(rd::<u64>(rd::<u64>(obj) + 0x20), (obj));
                lp = lp.wrapping_add(8);
            }
        }
    }

    exe::call_iat!(0x1_4264_5598, (lock_1b0));

    // Phase 2: [this + 0x188], count at [this + 0x190], lock at [this + 0x1b8]
    let init_190 = rd::<u32>(this + 0x190) as u64;
    let mut scratch_ptr = if init_190 != 0 {
        exe::call_va!(0x1_4241_c0a0, (init_190.wrapping_mul(8), 8u64) -> u64)
    } else {
        0
    };
    let mut scratch_len = 0u32;
    let mut scratch_cap = init_190 as u32;

    let base_188 = rd::<u64>(this + 0x188);
    let cnt_190 = rd::<u32>(this + 0x190) as u64;
    if base_188 != base_188.wrapping_add(cnt_190.wrapping_mul(8)) {
        let lock_1b8 = this + 0x1b8;
        exe::call_iat!(0x1_4264_55a8, (lock_1b8));
        let mut i: i32 = 0;
        while (i as i64 as u64) < (rd::<u32>(this + 0x190) as u64) {
            let arr = rd::<u64>(this + 0x188);
            let item = rd::<u64>(arr.wrapping_add((i as i64 as u64).wrapping_mul(8)));
            let st = rd::<u32>(item + 0x80);
            let mut remove = false;
            if st == 3 {
                remove = true;
            } else if st == 4 {
                if (exe::call_va!(rd::<u64>(rd::<u64>(item) + 0x20), (item) -> u64) & 0xff) != 0 {
                    if scratch_len == scratch_cap {
                        let cur_l = scratch_len as u64;
                        let cur_c = scratch_cap as u64;
                        let mut new_c = cur_c + (cur_c >> 1);
                        if new_c < cur_l + 1 {
                            new_c = cur_l + 1;
                        }
                        let new_p = exe::call_va!(0x1_4241_c0a0, (new_c * 8, 8u64) -> u64);
                        if cur_l != 0 {
                            core::ptr::copy(
                                scratch_ptr as *const u8,
                                new_p as *mut u8,
                                (cur_l * 8) as usize,
                            );
                        }
                        if scratch_ptr != 0 {
                            exe::call_va!(0x1_4241_c190, (scratch_ptr));
                        }
                        scratch_ptr = new_p;
                        scratch_cap = new_c as u32;
                    }
                    wr::<u64>(scratch_ptr.wrapping_add((scratch_len as u64) * 8), item);
                    scratch_len += 1;
                    remove = true;
                } else if rd::<u32>(item + 0x80) == 5 {
                    remove = true;
                }
            } else if st == 5 {
                remove = true;
            }

            if remove {
                let base = rd::<u64>(this + 0x188);
                let cur_p = base.wrapping_add((i as i64 as u64) * 8);
                let last_idx = (rd::<u32>(this + 0x190) as u64).wrapping_sub(1);
                let last_p = base.wrapping_add(last_idx * 8);
                if cur_p != last_p {
                    wr::<u64>(cur_p, rd::<u64>(last_p));
                }
                wr::<u32>(this + 0x190, rd::<u32>(this + 0x190).wrapping_sub(1));
                i -= 1;
            }
            i += 1;
        }
        exe::call_iat!(0x1_4264_5598, (lock_1b8));
    }

    let mut sp = scratch_ptr;
    let sp_end = scratch_ptr.wrapping_add((scratch_len as u64) * 8);
    while sp != sp_end {
        let item = rd::<u64>(sp);
        exe::call_va!(rd::<u64>(rd::<u64>(item) + 0x18), (item));
        sp = sp.wrapping_add(8);
    }
    if scratch_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (scratch_ptr));
    }

    // Phase 3: [this + 0x198], count at [this + 0x1a0], lock at [this + 0x1c0]
    let lock_1c0 = this + 0x1c0;
    exe::call_iat!(0x1_4264_55a8, (lock_1c0));
    let mut i3: i32 = 0;
    while (i3 as i64 as u64) < (rd::<u32>(this + 0x1a0) as u64) {
        let base = rd::<u64>(this + 0x198);
        let batch = rd::<u64>(base.wrapping_add((i3 as i64 as u64) * 8));
        if rd::<u32>(batch + 8) == 0 {
            let sub_cnt = rd::<i32>(batch);
            let mut j: i32 = 0;
            let mut all_ready = true;
            while j < sub_cnt {
                let sub_arr = rd::<u64>(batch + 0x10);
                let sub_res = rd::<u64>(sub_arr.wrapping_add((j as i64 as u64) * 16));
                let st = rd::<u32>(sub_res + 0x80);
                if st != 3 && st != 5 {
                    all_ready = false;
                    break;
                }
                j += 1;
            }
            if all_ready {
                let p = (batch + 8) as *const core::sync::atomic::AtomicU32;
                (*p).swap(1, core::sync::atomic::Ordering::SeqCst);
            }
        }
        if rd::<u32>(batch + 8) == 1 {
            let base = rd::<u64>(this + 0x198);
            let cur_p = base.wrapping_add((i3 as i64 as u64) * 8);
            let next_p = cur_p.wrapping_add(8);
            let total_bytes = (rd::<u32>(this + 0x1a0) as u64) * 8;
            let rem_bytes = base.wrapping_add(total_bytes).wrapping_sub(next_p);
            if rem_bytes != 0 {
                core::ptr::copy(next_p as *const u8, cur_p as *mut u8, rem_bytes as usize);
            }
            wr::<u32>(this + 0x1a0, rd::<u32>(this + 0x1a0).wrapping_sub(1));
            i3 -= 1;

            let g_298 = rd::<u64>(0x1_43a0_8298);
            if g_298 != 0 {
                exe::call_iat!(0x1_4264_55a8, (g_298 + 0x250));
                wr::<u32>(g_298 + 0x258, rd::<u32>(g_298 + 0x258).wrapping_add(1));
                exe::call_iat!(0x1_4264_5598, (g_298 + 0x250));
                exe::call_iat!(0x1_4264_5590, (g_298 + 0x248));
            }
        }
        i3 += 1;
    }
    exe::call_iat!(0x1_4264_5598, (lock_1c0));

    // Phase 4: acquire all 4 locks [this + 0x1a8, 0x1b0, 0x1b8, 0x1c0], update dirty flags, release
    let locks = [this + 0x1a8, this + 0x1b0, this + 0x1b8, this + 0x1c0];
    let mut first_lock: i32 = 0;
    loop {
        if (first_lock as usize) < 4 {
            exe::call_iat!(0x1_4264_55a8, (locks[first_lock as usize]));
        }
        let mut fail = exe::call_va!(
            0x1_413a_0a00,
            (
                0u64,
                first_lock as u32 as u64,
                locks[0],
                locks[1],
                locks[2],
                locks[3]
            ) -> u64
        ) as i32;
        if fail == -1 {
            fail = exe::call_va!(
                0x1_413a_0a00,
                (
                    (first_lock + 1) as u32 as u64,
                    4u64,
                    locks[0],
                    locks[1],
                    locks[2],
                    locks[3]
                ) -> u64
            ) as i32;
            if fail == -1 {
                break;
            }
        }
        let limit = first_lock + 1;
        for k in 0..4i32 {
            if first_lock <= k && limit > k {
                exe::call_iat!(0x1_4264_5598, (locks[k as usize]));
            }
        }
        let _ = exe::call_iat!(0x1_4264_5868, () -> u64);
        first_lock = fail;
    }

    wr::<u8>(this + 0x170, u8::from(rd::<u32>(this + 0x180) != 0));
    wr::<u8>(this + 0x171, u8::from(rd::<u32>(this + 0x190) != 0));
    wr::<u8>(this + 0x172, u8::from(rd::<u32>(this + 0x1a0) != 0));

    exe::call_iat!(0x1_4264_5598, (locks[0]));
    exe::call_iat!(0x1_4264_5598, (locks[1]));
    exe::call_iat!(0x1_4264_5598, (locks[2]));
    exe::call_iat!(0x1_4264_5598, (locks[3]));
    0
}

// ---------------------------------------------------------------------------
// 10. 0x1424f8220 — ui_page_update
// ---------------------------------------------------------------------------

#[inline]
unsafe fn call_vfunc_u64_f64(fn_va: u64, rcx: u64, xmm1_val: f64) {
    let _ = crate::simd_ops::call_va8_xmm(
        fn_va,
        rcx,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        [0, xmm1_val.to_bits() as u128, 0, 0],
    );
}

/// Stable in-place removal from `buf[..cnt]` of every value that occurs in the guest list
/// `cur[..n]`; returns the kept count. The original runs one `std::remove` over the buffer per
/// `cur` element (O(n·m) loads and stores). This single filter pass leaves the same kept prefix
/// and count; slots past the kept count are dead (only `(count, ptr)` is passed on, then the
/// buffer is freed). `buf` is this call's private copy, so plain loads/stores are used for it.
unsafe fn remove_listed(buf: u64, cnt: usize, cur: u64, n: usize) -> usize {
    let mut needles: Vec<u64> = (0..n).map(|i| rd::<u64>(cur + (i as u64) * 8)).collect();
    needles.sort_unstable();
    let p = buf as *mut u64;
    let mut w = 0usize;
    for r in 0..cnt {
        let v = p.add(r).read_unaligned();
        if needles.binary_search(&v).is_err() {
            p.add(w).write_unaligned(v);
            w += 1;
        }
    }
    w
}

#[cfg(test)]
mod remove_listed_tests {
    /// The original per-needle `std::remove` loop, as a reference model.
    fn reference(buf: &mut [u64], cnt: usize, cur: &[u64]) -> usize {
        let mut cnt = cnt;
        for &needle in cur {
            let Some(first) = buf[..cnt].iter().position(|&v| v == needle) else { continue };
            let mut w = first;
            for r in first + 1..cnt {
                if buf[r] != needle {
                    buf[w] = buf[r];
                    w += 1;
                }
            }
            cnt = w;
        }
        cnt
    }

    #[test]
    fn matches_per_needle_remove() {
        let mut seed = 0x1234_5678_9abc_def0u64;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        for case in 0..2000 {
            let m = (next() % 40) as usize;
            let n = (next() % 40) as usize;
            let range = 1 + (next() % 24); // small range => duplicates on both sides
            let old: Vec<u64> = (0..m).map(|_| next() % range).collect();
            let cur: Vec<u64> = (0..n).map(|_| next() % range).collect();
            let mut a = old.clone();
            let ka = reference(&mut a, m, &cur);
            let mut b = old.clone();
            let kb = unsafe { super::remove_listed(b.as_mut_ptr() as u64, m, cur.as_ptr() as u64, n) };
            assert_eq!(ka, kb, "case {case}: count");
            assert_eq!(a[..ka], b[..kb], "case {case}: kept prefix");
        }
    }
}

/// `sub_1424f8220` — original @ 0x1424f8220 (1185 bytes).
/// Updates a UI page instance: snapshots global listeners, flushes pending resize/events,
/// advances Coherent view time (`vfunc_14` with `perf_elapsed_seconds() * 1000.0`), and
/// reconciles removed sub-elements.
pub unsafe extern "win64" fn ui_page_update(this: u64, arg2: u64, arg3: u64) -> u64 {
    let tree_root = rd::<u64>(0x1_43c4_7f68);
    let mut iter_ctx = [0x1_43c4_7f68u64, 1u64];
    let mut iter_state = [
        iter_ctx.as_mut_ptr() as u64,
        rd::<u64>(tree_root),
        0u64,
    ];
    let mut snap_ptr = 0u64;
    let mut snap_len = 0u32;
    let mut snap_cap = 0u32;

    while iter_state[1] != tree_root {
        if (iter_state[2] & 0xff) != 0 {
            exe::call_va!(0x1_4020_0070, ());
            return 0;
        }
        let val = rd::<u64>(iter_state[1] + 0x28);
        if snap_len == snap_cap {
            let cur_l = snap_len as u64;
            let cur_c = snap_cap as u64;
            let mut new_c = cur_c + (cur_c >> 1);
            if new_c < cur_l + 1 {
                new_c = cur_l + 1;
            }
            let new_p = exe::call_va!(0x1_4241_c0a0, (new_c * 8, 8u64) -> u64);
            if cur_l != 0 {
                core::ptr::copy(
                    snap_ptr as *const u8,
                    new_p as *mut u8,
                    (cur_l * 8) as usize,
                );
            }
            if snap_ptr != 0 {
                exe::call_va!(0x1_4241_c190, (snap_ptr));
            }
            snap_ptr = new_p;
            snap_cap = new_c as u32;
        }
        wr::<u64>(snap_ptr.wrapping_add((snap_len as u64) * 8), val);
        snap_len += 1;
        exe::call_va!(0x1_403b_31f0, ((&mut iter_state[1] as *mut u64) as u64));
        if (iter_state[2] & 0xff) != 0 {
            exe::call_va!(0x1_4020_0070, ());
            return 0;
        }
    }

    let sub_60 = rd::<u64>(this + 0x60);
    exe::call_iat!(0x1_4264_55a8, (sub_60 + 0x30));
    wr::<u64>(sub_60 + 0x50, rd::<u64>(sub_60 + 0x50).wrapping_add(1));
    exe::call_iat!(0x1_4264_5598, (sub_60 + 0x30));

    let sub_60_now = rd::<u64>(this + 0x60);
    if sub_60_now != 0 && rd::<u8>(sub_60_now + 8) != 0 {
        let p = (rd::<u64>(this + 0x60) + 8) as *const core::sync::atomic::AtomicU8;
        (*p).swap(0, core::sync::atomic::Ordering::SeqCst);
        exe::call_va!(
            0x1_424f_8070,
            (
                this,
                0u64,
                rd::<u32>(this + 0x30) as u64,
                rd::<u32>(this + 0x34) as u64,
                rd::<u32>(0x1_42d2_3ee0) as u64
            )
        );
    }

    let sub_28 = rd::<u64>(this + 0x28);
    if sub_28 != 0 {
        exe::call_va!(0x1_4250_49b0, (sub_28));
    }

    if rd::<u64>(this + 8) != 0 {
        let sub_48 = rd::<u64>(this + 0x48);
        if sub_48 != 0 {
            let lock_48 = sub_48 + 0x38;
            exe::call_iat!(0x1_4264_55a8, (lock_48));
            let mut cur = rd::<u64>(sub_48 + 0x28);
            let end_init = cur.wrapping_add((rd::<u32>(sub_48 + 0x30) as u64).wrapping_shl(4));
            if cur != end_init {
                loop {
                    let ev0 = rd::<u64>(cur);
                    let ev1 = rd::<u64>(cur + 8);
                    if (exe::call_va!(0x1_424f_9140, (sub_48, ev0, ev1) -> u64) & 0xff) != 0 {
                        let base = rd::<u64>(sub_48 + 0x28);
                        let idx = (cur.wrapping_sub(base)) >> 4;
                        let cnt = rd::<u32>(sub_48 + 0x30) as u64;
                        if idx + 1 < cnt {
                            let mut move_off = (idx + 1) << 4;
                            let mut rem = cnt - (idx + 1);
                            while rem != 0 {
                                let b = rd::<u64>(sub_48 + 0x28);
                                let v0 = rd::<u64>(b + move_off);
                                let v1 = rd::<u64>(b + move_off + 8);
                                wr::<u64>(b + move_off - 0x10, v0);
                                wr::<u64>(b + move_off - 0x08, v1);
                                move_off += 0x10;
                                rem -= 1;
                            }
                        }
                        let new_base = rd::<u64>(sub_48 + 0x28);
                        wr::<u32>(sub_48 + 0x30, rd::<u32>(sub_48 + 0x30).wrapping_sub(1));
                        cur = new_base.wrapping_add(idx << 4);
                    } else {
                        cur = cur.wrapping_add(0x10);
                    }
                    let cur_end = rd::<u64>(sub_48 + 0x28)
                        .wrapping_add((rd::<u32>(sub_48 + 0x30) as u64).wrapping_shl(4));
                    if cur == cur_end {
                        break;
                    }
                }
            }
            exe::call_iat!(0x1_4264_5598, (lock_48));
        }
    }

    exe::call_va!(0x1_424f_56e0, (this));

    let root2 = rd::<u64>(0x1_43c4_7f68);
    let mut node = rd::<u64>(root2);
    if node != root2 {
        loop {
            let payload = rd::<u64>(node + 0x28);
            let id = rd::<u32>(payload + 8) as u64;
            exe::call_va!(0x1_424f_6500, (this, id, arg2, arg3));
            node = rbtree_next(node);
            if node == rd::<u64>(0x1_43c4_7f68) {
                break;
            }
        }
    }

    if rd::<u64>(this + 0x60) != 0 {
        exe::call_va!(0x1_4141_2320, (rd::<u64>(0x1_4397_ff58), this));
    }

    let view = rd::<u64>(this + 8);
    let vtbl = rd::<u64>(view);
    let vf14 = rd::<u64>(vtbl + 0x70);
    let sec = crate::ns::r::thread::perf_elapsed_seconds();
    let ms = sec * f64::from_bits(0x408f_4000_0000_0000);
    call_vfunc_u64_f64(vf14, view, ms);

    let sub_48 = rd::<u64>(this + 0x48);
    let mut old_cnt = rd::<u32>(sub_48 + 0x20);
    let old_base = rd::<u64>(sub_48 + 0x18);
    let old_end = old_base.wrapping_add((old_cnt as u64) * 8);
    let mut copy_vec = [0u64, 0u64];
    let copy_ptr = if old_cnt != 0 {
        let p = exe::call_va!(0x1_401d_ec00, (0u64, old_cnt as u64) -> u64);
        copy_vec[0] = p;
        copy_vec[1] = (old_cnt as u64) | ((old_cnt as u64) << 32);
        p
    } else {
        0
    };
    let mut s = old_base;
    let mut d = copy_ptr;
    while s != old_end {
        wr::<u64>(d, rd::<u64>(s));
        d = d.wrapping_add(8);
        s = s.wrapping_add(8);
    }

    exe::call_va!(0x1_424f_5af0, (this));

    let sub_48_after = rd::<u64>(this + 0x48);
    let cur_p = rd::<u64>(sub_48_after + 0x18);
    let cur_n = rd::<u32>(sub_48_after + 0x20) as usize;
    if old_cnt != 0 && cur_n != 0 {
        let kept = remove_listed(copy_ptr, old_cnt as usize, cur_p, cur_n) as u32;
        if kept != old_cnt {
            old_cnt = kept;
            copy_vec[1] = (old_cnt as u64) | ((copy_vec[1] >> 32) << 32);
        }
    }

    let span = [old_cnt as u64, copy_ptr];
    exe::call_va!(0x1_424f_5dc0, (this, span.as_ptr() as u64));

    exe::call_va!(0x1_401d_e830, (copy_vec.as_mut_ptr() as u64));

    if snap_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (snap_ptr));
    }
    0
}

static mut MOCK_SCRATCH_BUF: [u64; 256] = [0; 256];

unsafe extern "win64" fn mock_scratch_alloc(_size: u64, _align: u64) -> u64 {
    core::ptr::addr_of_mut!(MOCK_SCRATCH_BUF) as u64
}

unsafe extern "win64" fn mock_scratch_free(_ptr: u64) {}

// ===========================================================================
// Differential tests
// ===========================================================================

pub unsafe fn diff_open_hash_probe_40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u64, u64) -> u64;
    let (orig_fn, new_fn): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));

    let run = |mode: u32, call_fn: F| -> [u64; 2] {
        let mut table = [0u8; 0x88];
        let mut entries = [0u8; 40 * 8];
        let t_ptr = table.as_mut_ptr() as u64;
        let e_ptr = entries.as_mut_ptr() as u64;
        wr::<u64>(t_ptr + 0x28, 0xdead_beef);
        wr::<u64>(t_ptr + 0x30, 0xcafe_babe);
        wr::<u64>(t_ptr + 0x38, if mode == 2 { 1 } else { 0 });
        wr::<u64>(t_ptr + 0x48, 8);
        wr::<u64>(t_ptr + 0x50, u64::MAX);
        wr::<u64>(t_ptr + 0x58, u64::MAX);
        wr::<u64>(t_ptr + 0x78, e_ptr);
        for i in 0..8u64 {
            wr::<u64>(e_ptr + i * 40, u64::MAX);
            wr::<u64>(e_ptr + i * 40 + 8, u64::MAX);
        }
        let key = [0x1234_5670u64, 0x1234_5673u64]; // xor = 3
        if mode == 0 {
            wr::<u64>(e_ptr + 3 * 40, key[0]);
            wr::<u64>(e_ptr + 3 * 40 + 8, key[1]);
        } else if mode == 2 {
            wr::<u64>(e_ptr + 3 * 40, 0xdead_beef);
            wr::<u64>(e_ptr + 3 * 40 + 8, 0xcafe_babe);
        }
        let mut out = [0u64; 2];
        let ret = call_fn(t_ptr, out.as_mut_ptr() as u64, key.as_ptr() as u64);
        assert_eq!(ret, out.as_mut_ptr() as u64);
        out
    };

    for mode in 0..3 {
        let o = run(mode, orig_fn);
        let r = run(mode, new_fn);
        crate::difftest::eq(&format!("open_hash_probe_40 mode={mode}"), o, r)?;
    }
    Ok(3)
}

pub unsafe fn diff_ui_allocator_contains_ptr(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u64) -> u64;
    let (orig_fn, new_fn): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let run = |probe: u64, lock_en: u8, f: F| -> (u64, u8) {
        let mut alloc = [0u8; 0x40];
        let mut segs = [0u64; 6];
        segs[0] = 0x1000;
        segs[1] = 0x200;
        segs[3] = 0x5000;
        segs[4] = 0x400;
        let a_ptr = alloc.as_mut_ptr() as u64;
        wr::<u8>(a_ptr + 0x10, lock_en);
        wr::<u64>(a_ptr + 0x28, segs.as_ptr() as u64);
        wr::<u32>(a_ptr + 0x30, 2);
        let res = f(a_ptr, probe) & 0xff;
        (res, rd::<u8>(a_ptr))
    };
    for &(probe, lock_en) in &[(0x1100u64, 1u8), (0x2000u64, 1u8), (0x53ffu64, 0u8)] {
        let o = run(probe, lock_en, orig_fn);
        let r = run(probe, lock_en, new_fn);
        crate::difftest::eq("ui_allocator_contains_ptr", o, r)?;
    }
    Ok(3)
}

pub unsafe fn diff_ui_allocator_alloc_zeroed(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u64) -> u64;
    let (orig_fn, new_fn): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let run = |f: F| -> (u64, u32, [u8; 32]) {
        let mut alloc = [0u8; 0x40];
        let mut heap = [0u8; 0x100];
        let mut block = [0xaa_u8; 0x40];
        let a_ptr = alloc.as_mut_ptr() as u64;
        let h_ptr = heap.as_mut_ptr() as u64;
        let b_ptr = block.as_mut_ptr() as u64;
        wr::<u8>(a_ptr + 0x10, 1);
        wr::<u64>(a_ptr + 0x18, h_ptr);
        wr::<u32>(h_ptr, 0x10);
        wr::<u64>(h_ptr + 0x98, b_ptr);
        wr::<u64>(b_ptr + 0x10, h_ptr + 0x88);
        let ret = f(a_ptr, 16);
        let rel = ret.wrapping_sub(b_ptr);
        let mut snap = [0u8; 32];
        snap.copy_from_slice(&block[0x10..0x30]);
        (rel, rd::<u32>(h_ptr), snap)
    };
    let o = run(orig_fn);
    let r = run(new_fn);
    crate::difftest::eq("ui_allocator_alloc_zeroed", o, r)?;
    Ok(1)
}

pub unsafe fn diff_fact_variant_compare_and_set(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn mock_get_cstr(_src_val: u64, _ctx: u64) -> u64 {
        c"main_menu".as_ptr() as u64
    }
    let _m = crate::difftest::Mock::new(0x1_4021_7680, mock_get_cstr as *const ());
    type F = unsafe extern "win64" fn(u64, u64, u64) -> u64;
    let (orig_fn, new_fn): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let run = |tag: u8, same: bool, f: F| -> (u64, [u64; 3]) {
        let mut src = [0u8; 0x20];
        let mut bind = [0u8; 0xd8];
        let s_ptr = src.as_mut_ptr() as u64;
        let b_ptr = bind.as_mut_ptr() as u64;
        wr::<u8>(b_ptr + 0xc8, tag);
        match tag {
            0 => {
                wr::<u64>(s_ptr + 8, 1);
                wr::<u8>(b_ptr + 0x80, if same { 1 } else { 0 });
            }
            1 => {
                wr::<u64>(s_ptr + 8, 42.5f64.to_bits());
                wr::<u64>(b_ptr + 0x80, (if same { 42.5f64 } else { 10.0f64 }).to_bits());
            }
            2 => {
                let init: &[u8] = if same { b"main_menu" } else { b"other" };
                wr::<u32>(b_ptr + 0x80, 0x0800_0008);
                wr::<u32>(b_ptr + 0x84, init.len() as u32);
                for (i, &ch) in init.iter().enumerate() {
                    wr::<u8>(b_ptr + 0x88 + i as u64, ch);
                }
                wr::<u8>(b_ptr + 0x88 + init.len() as u64, 0);
            }
            3 => {
                wr::<u32>(s_ptr + 8, 1.5f32.to_bits());
                wr::<u32>(s_ptr + 12, 2.5f32.to_bits());
                wr::<u32>(b_ptr + 0x80, 1.5f32.to_bits());
                wr::<u32>(b_ptr + 0x84, (if same { 2.5f32 } else { 3.5f32 }).to_bits());
            }
            _ => {}
        }
        let ret = f(0, s_ptr, b_ptr) & 0xff;
        (
            ret,
            [
                rd::<u64>(b_ptr + 0x80),
                rd::<u64>(b_ptr + 0x88),
                rd::<u64>(b_ptr + 0x90),
            ],
        )
    };
    for &tag in &[0u8, 1u8, 2u8, 3u8] {
        for &same in &[false, true] {
            let o = run(tag, same, orig_fn);
            let r = run(tag, same, new_fn);
            crate::difftest::eq(&format!("fact_variant tag={tag} same={same}"), o, r)?;
        }
    }
    Ok(8)
}

pub unsafe fn diff_update_fact_bindings(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u64, u64, u64) -> u64;
    let (orig_fn, new_fn): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut mgr_stub = [0u8; 0x20];
    wr::<u32>((mgr_stub.as_mut_ptr() as u64) + 0x10, 77);
    let _g = GlobalGuard::set(0x1_43a0_82e8, mgr_stub.as_mut_ptr() as u64);

    let run = |f: F| -> u32 {
        let mut this_buf = [0u8; 0x48];
        let mut table_buf = [0u8; 0x88];
        let mut sentinel_node = [0u8; 0x30];
        let s_ptr = sentinel_node.as_mut_ptr() as u64;
        wr::<u8>(s_ptr + 0x19, 1);
        let mut tree_head = s_ptr;
        let t_ptr = this_buf.as_mut_ptr() as u64;
        wr::<u64>(t_ptr, 0);
        wr::<u32>(t_ptr + 0x08, 0);
        wr::<u64>(t_ptr + 0x20, (&mut tree_head as *mut u64) as u64);
        wr::<u32>(t_ptr + 0x38, 12);
        f(t_ptr, 0, table_buf.as_mut_ptr() as u64, 0);
        rd::<u32>(t_ptr + 0x38)
    };
    let o = run(orig_fn);
    let r = run(new_fn);
    crate::difftest::eq("update_fact_bindings", o, r)?;
    Ok(1)
}

pub unsafe fn diff_ecs_bind_groups_match(orig: usize, new: usize) -> Result<u32, String> {
    let _ = exe::call_iat!(0x1_4264_5790u64, () -> u32);
    type F = unsafe extern "win64" fn(u64, u64, u64, u64) -> u64;
    let (orig_fn, new_fn): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let run = |f: F| -> (u32, [u32; 4]) {
        let mut world = vec![0u8; 0x18100];
        let w_ptr = world.as_mut_ptr() as u64;
        let comps0: [u16; 3] = [10, 20, 30];
        let comps1: [u16; 2] = [10, 99];
        wr::<u64>(w_ptr + 0x18050, comps0.as_ptr() as u64);
        wr::<u32>(w_ptr + 0x18068, 3);
        wr::<u64>(w_ptr + 0x18050 + 32, comps1.as_ptr() as u64);
        wr::<u32>(w_ptr + 0x18068 + 32, 2);

        let req: [u16; 1] = [10];
        let exc: [u16; 1] = [99];
        let mut q_entry = [0u8; 40];
        let qe_ptr = q_entry.as_mut_ptr() as u64;
        wr::<u64>(qe_ptr, 1);
        wr::<u64>(qe_ptr + 8, req.as_ptr() as u64);
        wr::<u64>(qe_ptr + 0x10, 1);
        wr::<u64>(qe_ptr + 0x18, exc.as_ptr() as u64);
        wr::<u16>(qe_ptr + 0x20, 7);

        let queries = [1u64, qe_ptr];
        let cands: [u16; 2] = [0, 1];
        let cand_span = [2u64, cands.as_ptr() as u64];

        let mut out_items = [0u32; 4];
        let mut worker_vecs = [0u8; 64 * 16];
        let wv_ptr = worker_vecs.as_mut_ptr() as u64;
        for w in 0..64u64 {
            wr::<u64>(wv_ptr + w * 16, out_items.as_mut_ptr() as u64);
            wr::<u32>(wv_ptr + w * 16 + 8, 0);
            wr::<u32>(wv_ptr + w * 16 + 12, 4);
        }
        let out_ctx = [0u64, wv_ptr];
        f(
            queries.as_ptr() as u64,
            cand_span.as_ptr() as u64,
            w_ptr,
            out_ctx.as_ptr() as u64,
        );
        let wid = crate::ns::r::thread::worker_index() as u32 as u64;
        let len = rd::<u32>(wv_ptr + wid * 16 + 8);
        (len, out_items)
    };
    let o = run(orig_fn);
    let r = run(new_fn);
    crate::difftest::eq("ecs_bind_groups_match", o, r)?;
    Ok(1)
}

pub unsafe fn diff_start_scheduling(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u64, u64, u64) -> u64;
    let (orig_fn, new_fn): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut sched_mgr = [0u8; 0x40];
    let mut inner_sched = [0u8; 0x80];
    wr::<u64>(
        (sched_mgr.as_mut_ptr() as u64) + 0x10,
        inner_sched.as_mut_ptr() as u64,
    );
    let _g = GlobalGuard::set(0x1_43a0_82f8, sched_mgr.as_mut_ptr() as u64);
    let _ma = crate::difftest::Mock::new(0x1_4241_c0a0, mock_scratch_alloc as *const ());
    let _mf = crate::difftest::Mock::new(0x1_4241_c190, mock_scratch_free as *const ());

    let run = |f: F| -> [u32; 2] {
        let counts = [5u32, 0u32];
        let mut graph = [0u8; 0x48];
        let g_ptr = graph.as_mut_ptr() as u64;
        wr::<u64>(g_ptr + 0x20, counts.as_ptr() as u64);
        wr::<u32>(g_ptr + 0x28, 2);
        wr::<u64>(g_ptr + 0x30, 0);
        wr::<u32>(g_ptr + 0x38, 0);
        core::ptr::write_bytes(core::ptr::addr_of_mut!(MOCK_SCRATCH_BUF) as *mut u8, 0, 16);
        f(0, 0x1111, 0x2222, g_ptr);
        let b = core::ptr::addr_of!(MOCK_SCRATCH_BUF) as u64;
        [rd::<u32>(b), rd::<u32>(b + 4)]
    };
    let o = run(orig_fn);
    let r = run(new_fn);
    crate::difftest::eq("start_scheduling", o, r)?;
    Ok(1)
}

unsafe extern "win64" fn noop_sys_cb(_rcx: u64, _rdx: u64, _r8: u64) -> u64 {
    0
}

pub unsafe fn diff_schedule_node_complete(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u64, u64);
    let (orig_fn, new_fn): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut sched_mgr = [0u8; 0x40];
    let mut inner_sched = [0u8; 0x80];
    wr::<u64>(
        (sched_mgr.as_mut_ptr() as u64) + 0x10,
        inner_sched.as_mut_ptr() as u64,
    );
    let _g = GlobalGuard::set(0x1_43a0_82f8, sched_mgr.as_mut_ptr() as u64);
    let _m_cb = crate::difftest::Mock::new(0x1_40f6_7380, noop_sys_cb as *const ());

    let run = |f: F| -> [u32; 2] {
        let sys_map: [u16; 2] = [0, 1];
        let succ_list: [u16; 1] = [1];
        let mut dep_spans = [0u8; 32];
        let ds_ptr = dep_spans.as_mut_ptr() as u64;
        wr::<u64>(ds_ptr, succ_list.as_ptr() as u64);
        wr::<u32>(ds_ptr + 8, 1);

        let mut graph = [0u8; 0x40];
        let g_ptr = graph.as_mut_ptr() as u64;
        wr::<u64>(g_ptr, sys_map.as_ptr() as u64);
        wr::<u32>(g_ptr + 8, 2);
        wr::<u64>(g_ptr + 0x10, ds_ptr);

        let mut sys_desc = [0u8; 0x260];
        let sd_ptr = sys_desc.as_mut_ptr() as u64;
        wr::<u16>(sd_ptr, 7);
        wr::<u64>(sd_ptr + 0x120, 0x1_40f6_7380);

        let mut sys_table_box = [0u64; 2];
        sys_table_box[1] = sd_ptr;
        let mut ctx0 = [0u64; 4];
        ctx0[1] = sys_table_box.as_mut_ptr() as u64;

        let mut counts = [1u32, 1u32];
        let mut ctx = [
            ctx0.as_mut_ptr() as u64,
            0x1234u64,
            g_ptr,
            2u64,
            counts.as_mut_ptr() as u64,
        ];
        f(ctx.as_mut_ptr() as u64, 0, 1);
        counts
    };
    let o = run(orig_fn);
    let r = run(new_fn);
    crate::difftest::eq("schedule_node_complete", o, r)?;
    Ok(1)
}

unsafe extern "win64" fn mock_group_cb(arg_rcx: u64, gid: u64) -> u64 {
    wr::<u64>(arg_rcx, gid ^ 0x55aa);
    0
}

pub unsafe fn diff_schedule_group_node_run(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64);
    let (orig_fn, new_fn): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut sched_mgr = [0u8; 0x40];
    let mut inner_sched = [0u8; 0x80];
    wr::<u64>(
        (sched_mgr.as_mut_ptr() as u64) + 0x10,
        inner_sched.as_mut_ptr() as u64,
    );
    let _g = GlobalGuard::set(0x1_43a0_82f8, sched_mgr.as_mut_ptr() as u64);
    let _m_cb = crate::difftest::Mock::new(0x1_40f6_7380, mock_group_cb as *const ());

    let run = |f: F| -> ([u32; 2], u64) {
        let sys_map: [u16; 2] = [0, 1];
        let succ_list: [u16; 1] = [1];
        let mut dep_spans = [0u8; 32];
        let ds_ptr = dep_spans.as_mut_ptr() as u64;
        wr::<u64>(ds_ptr, succ_list.as_ptr() as u64);
        wr::<u32>(ds_ptr + 8, 1);

        let mut graph = [0u8; 0x40];
        let g_ptr = graph.as_mut_ptr() as u64;
        wr::<u64>(g_ptr, sys_map.as_ptr() as u64);
        wr::<u32>(g_ptr + 8, 2);
        wr::<u64>(g_ptr + 0x10, ds_ptr);

        let mut sys_desc = [0u8; 0x260];
        let sd_ptr = sys_desc.as_mut_ptr() as u64;
        wr::<u16>(sd_ptr, 7);
        wr::<u64>(sd_ptr + 0x128, exe::ptr::<u8>(0x1_40f6_7380) as u64);

        let mut bit_words = [u64::MAX; 4];
        let mut sys_table_box = [0u64; 2];
        sys_table_box[1] = sd_ptr;
        let mut ctx0 = [0u8; 0xc0];
        let ctx0_ptr = ctx0.as_mut_ptr() as u64;
        wr::<u64>(ctx0_ptr + 0x08, sys_table_box.as_mut_ptr() as u64);
        wr::<u64>(ctx0_ptr + 0x70, bit_words.as_mut_ptr() as u64);
        wr::<u64>(ctx0_ptr + 0xa8, 4);

        let mut cb_target = 0u64;
        let mut counts = [1u32, 1u32];
        let mut ctx = [
            ctx0_ptr,
            (&mut cb_target as *mut u64) as u64,
            g_ptr,
            2u64,
            counts.as_mut_ptr() as u64,
        ];
        let mut job_payload = [0u8; 16];
        let jp_ptr = job_payload.as_mut_ptr() as u64;
        wr::<u64>(jp_ptr, ctx.as_mut_ptr() as u64);
        wr::<u16>(jp_ptr + 0x08, 3);
        wr::<u16>(jp_ptr + 0x0a, 0);
        wr::<u16>(jp_ptr + 0x0c, 1);

        f(jp_ptr);
        (counts, cb_target)
    };
    let o = run(orig_fn);
    let r = run(new_fn);
    crate::difftest::eq("schedule_group_node_run", o, r)?;
    Ok(1)
}

pub unsafe fn diff_schedule_group_node_vfunc_1(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64);
    let (orig_fn, new_fn): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    unsafe extern "win64" fn mock_run(param_1: u64) -> u64 {
        wr::<u64>(param_1, rd::<u64>(param_1) ^ 0x9988_7766);
        0
    }
    let _m = crate::difftest::Mock::new(0x1_420a_95e0, mock_run as *const ())?;
    let mut job_o = [0u64, 0x1122_3344_5566_7788u64, 0u64];
    let mut job_n = job_o;
    orig_fn(job_o.as_mut_ptr() as u64);
    new_fn(job_n.as_mut_ptr() as u64);
    crate::difftest::eq("schedule_group_node_vfunc_1", job_o, job_n)?;
    Ok(1)
}

pub unsafe fn diff_streamer_resource_tick(orig: usize, new: usize) -> Result<u32, String> {
    let _ = exe::call_iat!(0x1_4264_5790u64, () -> u32);
    type F = unsafe extern "win64" fn(u64) -> u64;
    let (orig_fn, new_fn): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut g2f0 = vec![0u8; 0x520];
    let _g = GlobalGuard::set(0x1_43a0_82f0, g2f0.as_mut_ptr() as u64);
    let _ma = crate::difftest::Mock::new(0x1_4241_c0a0, mock_scratch_alloc as *const ());
    let _mf = crate::difftest::Mock::new(0x1_4241_c190, mock_scratch_free as *const ());

    let run = |active: bool, f: F| -> (u32, u8, u8, u8) {
        let mut streamer = vec![0u8; 0x240];
        let s_ptr = streamer.as_mut_ptr() as u64;
        let mut item0 = [0u8; 0x90];
        let i0_ptr = item0.as_mut_ptr() as u64;
        wr::<u32>(i0_ptr + 0x80, 2);
        let mut vec_178 = [i0_ptr, i0_ptr];
        if active {
            wr::<u8>(s_ptr + 0x170, 1);
            wr::<u64>(s_ptr + 0x178, vec_178.as_mut_ptr() as u64);
            wr::<u32>(s_ptr + 0x180, 2);
        }
        f(s_ptr);
        (
            rd::<u32>(s_ptr + 0x180),
            rd::<u8>(s_ptr + 0x170),
            rd::<u8>(s_ptr + 0x171),
            rd::<u8>(s_ptr + 0x172),
        )
    };
    for &active in &[false, true] {
        let o = run(active, orig_fn);
        let r = run(active, new_fn);
        crate::difftest::eq(&format!("streamer_resource_tick active={active}"), o, r)?;
    }
    Ok(2)
}

unsafe extern "win64" fn noop_u64_1(_a: u64) -> u64 {
    0
}
unsafe extern "win64" fn noop_u64_2(_a: u64, _b: u64) -> u64 {
    0
}

pub unsafe fn diff_ui_page_update(orig: usize, new: usize) -> Result<u32, String> {
    let _ = exe::call_iat!(0x1_4264_5790u64, () -> u32);
    type F = unsafe extern "win64" fn(u64, u64, u64) -> u64;
    let (orig_fn, new_fn): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut tree_sentinel = [0u64; 8];
    let ts_ptr = tree_sentinel.as_mut_ptr() as u64;
    wr::<u64>(ts_ptr, ts_ptr);
    let _g = GlobalGuard::set(0x1_43c4_7f68, ts_ptr);
    let _ma = crate::difftest::Mock::new(0x1_4241_c0a0, mock_scratch_alloc as *const ());
    let _mf = crate::difftest::Mock::new(0x1_4241_c190, mock_scratch_free as *const ());
    let _m1 = crate::difftest::Mock::new(0x1_424f_56e0, noop_u64_1 as *const ());
    let _m2 = crate::difftest::Mock::new(0x1_4141_2320, noop_u64_2 as *const ());
    let _m3 = crate::difftest::Mock::new(0x1_424f_5af0, noop_u64_1 as *const ());
    let _m4 = crate::difftest::Mock::new(0x1_424f_5dc0, noop_u64_2 as *const ());
    let _m5 = crate::difftest::Mock::new(0x1_40f6_7380, noop_u64_1 as *const ());

    let run = |f: F| -> u64 {
        let mut page = [0u8; 0x80];
        let mut sub60 = [0u8; 0x60];
        let mut sub48 = [0u8; 0x50];
        let mut vtbl = [0u64; 16];
        vtbl[14] = 0x1_40f6_7380;
        let mut view = [vtbl.as_ptr() as u64, 0u64];

        let p_ptr = page.as_mut_ptr() as u64;
        let s60_ptr = sub60.as_mut_ptr() as u64;
        let s48_ptr = sub48.as_mut_ptr() as u64;
        wr::<u64>(s60_ptr + 0x50, 10);
        wr::<u64>(p_ptr + 0x08, view.as_mut_ptr() as u64);
        wr::<u64>(p_ptr + 0x48, s48_ptr);
        wr::<u64>(p_ptr + 0x60, s60_ptr);
        f(p_ptr, 0, 0);
        rd::<u64>(s60_ptr + 0x50)
    };
    let o = run(orig_fn);
    let r = run(new_fn);
    crate::difftest::eq("ui_page_update", o, r)?;
    drop((_m3, _m4));
    let cases = ui_page_update_reconcile_cases(orig_fn, new_fn)?;
    Ok(1 + cases)
}

/// Reconcile path of `ui_page_update`: the old element list (`sub48 + 0x18/0x20`) is copied, the
/// list refresh (`0x1424f5af0`) installs a new list, and every element still present is removed
/// from the copy before it is handed to `0x1424f5dc0` as `(count, ptr)`.
unsafe fn ui_page_update_reconcile_cases(
    orig_fn: unsafe extern "win64" fn(u64, u64, u64) -> u64,
    new_fn: unsafe extern "win64" fn(u64, u64, u64) -> u64,
) -> Result<u32, String> {
    static mut COPY_BUF: [u64; 64] = [0; 64];
    static mut CUR_LIST: [u64; 64] = [0; 64];
    static mut CUR_N: u32 = 0;
    static mut SPAN: Vec<u64> = Vec::new();
    static mut FREED: [u64; 2] = [0; 2];
    unsafe extern "win64" fn mock_alloc_copy(_a: u64, _n: u64) -> u64 {
        core::ptr::addr_of_mut!(COPY_BUF) as u64
    }
    unsafe extern "win64" fn mock_refresh(this: u64) {
        unsafe {
            let s48 = rd::<u64>(this + 0x48);
            wr::<u64>(s48 + 0x18, core::ptr::addr_of_mut!(CUR_LIST) as u64);
            wr::<u32>(s48 + 0x20, CUR_N);
        }
    }
    unsafe extern "win64" fn mock_apply(_this: u64, span: u64) {
        unsafe {
            let n = rd::<u64>(span);
            let p = rd::<u64>(span + 8);
            let mut v: Vec<u64> = (0..n).map(|i| rd::<u64>(p + i * 8)).collect();
            v.insert(0, n);
            *core::ptr::addr_of_mut!(SPAN) = v;
        }
    }
    unsafe extern "win64" fn mock_free_vec(v: u64) {
        unsafe { FREED = [rd::<u64>(v), rd::<u64>(v + 8)] }
    }
    let _ma = crate::difftest::Mock::new(0x1_401d_ec00, mock_alloc_copy as *const ());
    let _mb = crate::difftest::Mock::new(0x1_424f_5af0, mock_refresh as *const ());
    let _mc = crate::difftest::Mock::new(0x1_424f_5dc0, mock_apply as *const ());
    let _md = crate::difftest::Mock::new(0x1_401d_e830, mock_free_vec as *const ());

    let cases: &[(&[u64], &[u64])] = &[
        (&[1, 2, 3, 4], &[1, 2, 3, 4]),
        (&[1, 2, 3, 4], &[]),
        (&[1, 2, 3, 4], &[9, 8]),
        (&[5, 1, 5, 2, 5, 3], &[5]),
        (&[7, 3, 7, 3, 1, 9, 9, 2], &[3, 9, 3, 6]),
        (&[1, 2, 3], &[3, 2, 1, 1]),
        (&[], &[1, 2]),
    ];
    let mut n_ok = 0;
    for (ci, &(old, cur)) in cases.iter().enumerate() {
        let run = |f: unsafe extern "win64" fn(u64, u64, u64) -> u64| {
            let mut page = [0u8; 0x80];
            let mut sub60 = [0u8; 0x60];
            let mut sub48 = [0u8; 0x50];
            let mut vtbl = [0u64; 16];
            vtbl[14] = 0x1_40f6_7380;
            let mut view = [vtbl.as_ptr() as u64, 0u64];
            let mut old_list = old.to_vec();
            let p_ptr = page.as_mut_ptr() as u64;
            let s48_ptr = sub48.as_mut_ptr() as u64;
            wr::<u64>(p_ptr + 0x08, view.as_mut_ptr() as u64);
            wr::<u64>(p_ptr + 0x48, s48_ptr);
            wr::<u64>(p_ptr + 0x60, sub60.as_mut_ptr() as u64);
            wr::<u64>(s48_ptr + 0x18, old_list.as_mut_ptr() as u64);
            wr::<u32>(s48_ptr + 0x20, old.len() as u32);
            COPY_BUF = [0xdead; 64];
            let mut list = [0u64; 64];
            list[..cur.len()].copy_from_slice(cur);
            CUR_LIST = list;
            CUR_N = cur.len() as u32;
            *core::ptr::addr_of_mut!(SPAN) = Vec::new();
            FREED = [0; 2];
            f(p_ptr, 0, 0);
            ((*core::ptr::addr_of!(SPAN)).clone(), FREED[1])
        };
        let a = run(orig_fn);
        let b = run(new_fn);
        if a != b {
            return Err(format!("reconcile case {ci} (old {old:?}, cur {cur:?}): orig {a:?} new {b:?}"));
        }
        n_ok += 1;
    }
    Ok(n_ok)
}





