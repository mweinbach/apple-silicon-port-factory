//! Decompiled hot functions shard W5A (`0x1420b55a0..0x1420c2d60`).

use crate::exe::{self, rd, wr};

#[repr(C)]
struct ArenaVec {
    arena: usize,
    data: usize,
    len: u32,
    cap: u32,
}

const _: () = assert!(core::mem::size_of::<ArenaVec>() == 0x18);

/// `r_1420b55a0`
/// Original: 0x1420b55a0 (152 bytes, Throws::No)
pub unsafe extern "win64" fn r_1420b55a0(param_1: usize) {
    let mut cur = rd::<usize>(param_1 + 0x10);
    let count = rd::<u64>(param_1 + 0x08) as usize;
    let end = cur.wrapping_add(count.wrapping_mul(8));
    while cur != end {
        let packed = rd::<u64>(cur);
        let ctx = rd::<usize>(param_1 + 0x20);
        let low_idx = (packed as u32) as usize;
        let grp_idx = (packed >> 0x30) as usize;
        let _ = rd::<u64>(ctx + 0x08);
        let bounds_ptr = rd::<usize>(param_1 + 0x30);
        let grp_base = rd::<usize>(ctx.wrapping_add(grp_idx * 8).wrapping_add(0x48));
        let key = rd::<u32>(grp_base.wrapping_add(low_idx * 8).wrapping_add(0x10));

        let mut pos: u64 = 0;
        let mut rem = rd::<u64>(param_1 + 0x28);
        while rem != 0 {
            let half = rem >> 1;
            let step = rem.wrapping_sub(half);
            let mid = half.wrapping_add(pos);
            if key >= rd::<u32>(bounds_ptr.wrapping_add((mid as usize).wrapping_mul(4))) {
                pos = pos.wrapping_add(step);
            }
            rem = half;
        }

        let out_val = rd::<u32>(param_1 + 0x18);
        cur = cur.wrapping_add(8);
        wr::<u64>(param_1 + 0x18, rd::<u64>(param_1 + 0x18).wrapping_add(1));
        let out_base = rd::<usize>(param_1 + 0x40);
        wr::<u32>(
            out_base
                .wrapping_add((pos as usize).wrapping_mul(4))
                .wrapping_sub(4),
            out_val,
        );
    }
}

unsafe fn adjacent_find_u64(begin: usize, end: usize) -> usize {
    if begin == end {
        return end;
    }
    let mut next = begin.wrapping_add(8);
    if next == end {
        return end;
    }
    let mut cur = begin;
    loop {
        if rd::<u64>(cur) == rd::<u64>(next) {
            return cur;
        }
        let n2 = next.wrapping_add(8);
        cur = next;
        next = n2;
        if n2 == end {
            return end;
        }
    }
}

/// `unknown_1420bdea0`
/// Original: 0x1420bdea0 (939 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420bdea0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    _param_4: usize,
    _param_5: usize,
    param_6: usize,
) -> usize {
    let limit_118 = rd::<u64>(param_3 + 0x118);
    let byte_len = (rd::<u64>(param_6) as usize).wrapping_mul(8);
    let arr_begin = rd::<usize>(param_6 + 0x08);
    let arr_end = arr_begin.wrapping_add(byte_len);
    let mut cur_dup = adjacent_find_u64(arr_begin, arr_end);

    let end_init = rd::<usize>(param_6 + 0x08).wrapping_add(byte_len);
    wr::<usize>(param_1, param_2);
    wr::<u64>(param_1 + 0x08, 0);
    wr::<u64>(param_1 + 0x10, 0);
    if cur_dup == end_init {
        return param_1;
    }

    loop {
        let mut node_138 = rd::<usize>(param_3 + 0x138);
        let mut step_cnt: u64 = 0;
        let end_cur = rd::<usize>(param_6 + 0x08).wrapping_add(byte_len);
        let mut idx_108: usize = 0;
        let mut node_108 = rd::<usize>(param_3 + 0x108);
        let mut node_120 = rd::<usize>(param_3 + 0x120);
        let mut idx_120: usize = 0;
        let target_val = rd::<u64>(cur_dup);
        let target_hi = (target_val >> 32) as u32;
        let target_lo = target_val as u16;
        let mut idx_138: usize = 0;

        if cur_dup != end_cur {
            while rd::<u64>(cur_dup) == target_val {
                while step_cnt < limit_118 {
                    idx_108 = idx_108.wrapping_add(1);
                    if idx_108 == rd::<usize>(node_108 + 0x08) {
                        let next_node = rd::<usize>(node_108);
                        if next_node != 0 {
                            node_108 = next_node;
                            idx_108 = 0;
                        }
                    }

                    idx_120 = idx_120.wrapping_add(1);
                    if idx_120 == rd::<usize>(node_120 + 0x08) {
                        let next_node = rd::<usize>(node_120);
                        if next_node != 0 {
                            node_120 = next_node;
                            idx_120 = 0;
                        }
                    }

                    idx_138 = idx_138.wrapping_add(1);
                    if idx_138 == rd::<usize>(node_138 + 0x08) {
                        let next_node = rd::<usize>(node_138);
                        if next_node != 0 {
                            node_138 = next_node;
                            idx_138 = 0;
                        }
                    }

                    if rd::<u32>(node_108.wrapping_add(idx_108 * 4).wrapping_add(0x10)) == target_hi
                        && rd::<u16>(node_120.wrapping_add(idx_120 * 2).wrapping_add(0x10))
                            == target_lo
                    {
                        if step_cnt < limit_118 {
                            let slot_138 = node_138.wrapping_add(0x10).wrapping_add(idx_138 * 8);
                            let val_138 = rd::<u64>(slot_138);
                            let old_cap = rd::<u32>(param_1 + 0x14);
                            let mut cur_len = rd::<u32>(param_1 + 0x10);
                            if cur_len == old_cap {
                                let arena = rd::<usize>(param_1);
                                let old_data = rd::<usize>(param_1 + 0x08);
                                let new_cap = ((old_cap as u64) * 3).max((cur_len + 1) as u64);
                                let old_bytes = (old_cap as u64) << 4;
                                let new_bytes = new_cap << 4;

                                let mut new_data = 0usize;
                                let mut reused = false;
                                if old_data == 0 {
                                    new_data =
                                        exe::call_va!(0x1_411f_11e0, (arena, new_bytes, 8u64) -> usize);
                                    if new_data != 0 {
                                        reused = true;
                                    }
                                } else {
                                    let b_start = rd::<usize>(arena);
                                    let b_end = b_start
                                        .wrapping_add((rd::<u32>(arena + 0x08) as usize) * 8);
                                    if b_start != b_end {
                                        let prev_off =
                                            rd::<u64>(arena + 0x10).wrapping_sub(old_bytes);
                                        if rd::<usize>(b_end - 8).wrapping_add(prev_off as usize)
                                            == old_data
                                        {
                                            let next_off = prev_off.wrapping_add(new_bytes);
                                            if next_off <= rd::<u64>(arena + 0x18) {
                                                wr::<u64>(arena + 0x10, next_off);
                                                new_data = old_data;
                                                reused = true;
                                            }
                                        }
                                    }
                                }

                                if !reused {
                                    new_data = exe::call_va!(
                                        0x1_411f_11e0,
                                        (rd::<usize>(param_1), new_bytes, 8u64) -> usize
                                    );
                                    let copy_len = rd::<u32>(param_1 + 0x10) as usize;
                                    let src_ptr = rd::<usize>(param_1 + 0x08);
                                    for i in 0..copy_len {
                                        wr::<u128>(
                                            new_data.wrapping_add(i * 16),
                                            rd::<u128>(src_ptr.wrapping_add(i * 16)),
                                        );
                                    }
                                }

                                cur_len = rd::<u32>(param_1 + 0x10);
                                wr::<usize>(param_1 + 0x08, new_data);
                                wr::<u32>(param_1 + 0x14, new_cap as u32);
                            }

                            let dst = rd::<usize>(param_1 + 0x08)
                                .wrapping_add((cur_len as usize).wrapping_mul(16));
                            wr::<u32>(param_1 + 0x10, cur_len.wrapping_add(1));
                            wr::<u32>(dst, target_hi);
                            wr::<u16>(dst + 0x04, target_lo);
                            wr::<u16>(dst + 0x06, 0);
                            wr::<u64>(dst + 0x08, val_138);
                            wr::<u64>(slot_138, 0);
                        }
                        break;
                    }

                    step_cnt = step_cnt.wrapping_add(1);
                }

                cur_dup = cur_dup.wrapping_add(8);
                let dynamic_end = rd::<usize>(param_6 + 0x08)
                    .wrapping_add((rd::<u64>(param_6) as usize).wrapping_mul(8));
                if cur_dup == dynamic_end {
                    break;
                }
            }
        }

        let search_end = rd::<usize>(param_6 + 0x08).wrapping_add(byte_len);
        cur_dup = adjacent_find_u64(cur_dup, search_end);
        let check_end = rd::<usize>(param_6 + 0x08).wrapping_add(byte_len);
        if cur_dup == check_end {
            break;
        }
    }

    param_1
}

/// `unknown_1420c1d70`
/// Original: 0x1420c1d70 (2135 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420c1d70(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
    param_7: usize,
) -> usize {
    let count = rd::<u32>(param_3);
    let base_98 = rd::<usize>(param_6);

    let mut vec_a_data = exe::call_va!(0x1_411f_11e0, (param_2, 0x100u64, 4u64) -> usize);
    if vec_a_data == 0 {
        vec_a_data = exe::call_va!(0x1_411f_11e0, (param_2, 0x100u64, 4u64) -> usize);
    }
    let mut vec_a = ArenaVec {
        arena: param_2,
        data: vec_a_data,
        len: 0,
        cap: 0x20,
    };

    let mut vec_b = ArenaVec {
        arena: param_2,
        data: 0,
        len: 0,
        cap: count,
    };
    let mut vec_c = ArenaVec {
        arena: param_2,
        data: 0,
        len: 0,
        cap: count,
    };
    if count != 0 {
        let bytes_b = (count as u64) * 0x28;
        let mut b_ptr = exe::call_va!(0x1_411f_11e0, (param_2, bytes_b, 8u64) -> usize);
        if b_ptr == 0 {
            b_ptr = exe::call_va!(0x1_411f_11e0, (param_2, bytes_b, 8u64) -> usize);
        }
        vec_b.data = b_ptr;

        let bytes_c = (count as u64) * 0x30;
        let mut c_ptr = exe::call_va!(0x1_411f_11e0, (param_2, bytes_c, 8u64) -> usize);
        if c_ptr == 0 {
            c_ptr = exe::call_va!(0x1_411f_11e0, (param_2, bytes_c, 8u64) -> usize);
        }
        vec_c.data = c_ptr;
    }

    let mut vec_tmp = ArenaVec {
        arena: param_2,
        data: 0,
        len: 0,
        cap: 0,
    };
    exe::call_va!(
        0x1_411f_2490,
        ((&mut vec_tmp as *mut ArenaVec) as usize, 0x20u64)
    );
    vec_tmp.cap = 0x20;

    let mut vec_d_data: usize = 0;
    let mut vec_d_len: u32 = 0;
    let mut vec_d_cap: u32 = 0;
    let mut total_bytes_28: u64 = 0;
    let mut total_bytes_30: u64 = 0;

    if count != 0 {
        let in_arr = rd::<usize>(param_3 + 0x08);
        let mut outer_idx: u32 = 0;
        while outer_idx < count {
            vec_tmp.len = 0;
            let arch = rd::<u16>(in_arr.wrapping_add((outer_idx as usize) * 8).wrapping_add(4));
            while outer_idx < count {
                let elem_ptr = in_arr.wrapping_add((outer_idx as usize) * 8);
                if rd::<u16>(elem_ptr + 0x04) != arch {
                    break;
                }
                if vec_tmp.len == vec_tmp.cap {
                    let new_cap = ((vec_tmp.cap as u64) * 3).max((vec_tmp.len + 1) as u64);
                    exe::call_va!(
                        0x1_411f_2490,
                        ((&mut vec_tmp as *mut ArenaVec) as usize, new_cap)
                    );
                }
                let idx = vec_tmp.len as usize;
                vec_tmp.len += 1;
                outer_idx += 1;
                wr::<u32>(vec_tmp.data.wrapping_add(idx * 4), rd::<u32>(elem_ptr));
            }

            let mut sort_begin = vec_tmp.data;
            let mut sort_end = vec_tmp.data.wrapping_add((vec_tmp.len as usize) * 4);
            let sort_count = ((sort_end as isize).wrapping_sub(sort_begin as isize)) >> 2;
            exe::call_va!(
                0x1_4021_d610,
                (
                    (&mut sort_begin as *mut usize) as usize,
                    (&mut sort_end as *mut usize) as usize,
                    sort_count,
                    0u8
                )
            );

            let range_start_idx = vec_a.len;
            let mut total_gap: u32 = 0;
            let mut expected: u32 = 0;
            let mut cur_ptr = vec_tmp.data;
            let end_ptr = vec_tmp.data.wrapping_add((vec_tmp.len as usize) * 4);
            while cur_ptr != end_ptr {
                let val = rd::<u32>(cur_ptr);
                let prev = expected;
                expected = val.wrapping_add(1);
                if prev != val {
                    if vec_a.len == vec_a.cap {
                        let new_cap = ((vec_a.cap as u64) * 3).max((vec_a.len + 1) as u64);
                        exe::call_va!(
                            0x1_411f_2650,
                            ((&mut vec_a as *mut ArenaVec) as usize, new_cap)
                        );
                    }
                    let idx = vec_a.len as usize;
                    vec_a.len += 1;
                    total_gap = total_gap.wrapping_add(val.wrapping_sub(prev));
                    wr::<u64>(
                        vec_a.data.wrapping_add(idx * 8),
                        (prev as u64) | ((val as u64) << 32),
                    );
                }
                cur_ptr = cur_ptr.wrapping_add(4);
            }

            let arch_usize = arch as usize;
            let ent_count = if rd::<u64>(param_7.wrapping_add(arch_usize * 8).wrapping_add(0x48))
                != 0
            {
                rd::<u32>(param_7.wrapping_add(arch_usize * 4).wrapping_add(0x10048))
            } else {
                0
            };
            if expected < ent_count {
                if vec_a.len == vec_a.cap {
                    let new_cap = ((vec_a.cap as u64) * 3).max((vec_a.len + 1) as u64);
                    exe::call_va!(
                        0x1_411f_2650,
                        ((&mut vec_a as *mut ArenaVec) as usize, new_cap)
                    );
                }
                let idx = vec_a.len as usize;
                vec_a.len += 1;
                total_gap = total_gap.wrapping_add(ent_count.wrapping_sub(expected));
                wr::<u64>(
                    vec_a.data.wrapping_add(idx * 8),
                    (expected as u64) | ((ent_count as u64) << 32),
                );
            }

            let range_end_idx = vec_a.len;
            if range_start_idx != range_end_idx {
                let range_pair = (range_start_idx as u64) | ((range_end_idx as u64) << 32);
                let src_base = rd::<u64>(rd::<usize>(param_4 + 0x08).wrapping_add(arch_usize * 8));
                let p5_entry = rd::<usize>(param_5 + 0x08).wrapping_add(arch_usize * 0x30);
                let dst_base_block = rd::<u64>(p5_entry + 0x08);
                let comp_ids_ptr =
                    rd::<usize>(param_7.wrapping_add(arch_usize * 0x20).wrapping_add(0x18058));
                let comp_count =
                    rd::<u32>(param_7.wrapping_add(arch_usize * 0x20).wrapping_add(0x1806c))
                        as usize;
                let p5_cnt_ptr = p5_entry + 0x28;

                if comp_count != 0 {
                    let off_tbl_slot = param_7.wrapping_add((arch_usize + 0xc03) * 0x20);
                    let total_gap_u64 = total_gap as u64;
                    for comp_idx in 0..comp_count {
                        let comp_id = rd::<u16>(comp_ids_ptr.wrapping_add(comp_idx * 2)) as usize;
                        let desc_ptr = base_98.wrapping_add(comp_id * 0x50);
                        let off_tbl_a = rd::<usize>(off_tbl_slot);
                        let src_ptr = src_base
                            .wrapping_add(rd::<u32>(off_tbl_a.wrapping_add(comp_idx * 4)) as u64);
                        let off_tbl_b = rd::<usize>(p5_entry + 0x18);
                        let elem_size = rd::<u64>(desc_ptr + 0x10);
                        let byte_len = elem_size.wrapping_mul(total_gap_u64);
                        let dst_base = dst_base_block
                            .wrapping_add(rd::<u32>(off_tbl_b.wrapping_add(comp_idx * 4)) as u64);
                        let fn_38 = rd::<u64>(desc_ptr + 0x38);
                        let cur_p5_cnt = rd::<u32>(p5_cnt_ptr);

                        if fn_38 == 0 {
                            let dst_ptr =
                                dst_base.wrapping_add((cur_p5_cnt as u64).wrapping_mul(elem_size));
                            if vec_b.len == vec_b.cap {
                                let new_cap =
                                    ((vec_b.cap as u64) * 3).max((vec_b.len + 1) as u64);
                                exe::call_va!(
                                    0x1_411f_2730,
                                    ((&mut vec_b as *mut ArenaVec) as usize, new_cap)
                                );
                            }
                            let idx = vec_b.len as usize;
                            vec_b.len += 1;
                            total_bytes_28 = total_bytes_28.wrapping_add(byte_len);
                            let dst = vec_b.data.wrapping_add(idx * 0x28);
                            wr::<u64>(dst, src_ptr);
                            wr::<u64>(dst + 0x08, dst_ptr);
                            wr::<u64>(dst + 0x10, elem_size);
                            wr::<u64>(dst + 0x18, range_pair);
                            wr::<u64>(dst + 0x20, byte_len);
                        } else {
                            if vec_c.len == vec_c.cap {
                                let new_cap =
                                    ((vec_c.cap as u64) * 3).max((vec_c.len + 1) as u64);
                                exe::call_va!(
                                    0x1_411f_2830,
                                    ((&mut vec_c as *mut ArenaVec) as usize, new_cap)
                                );
                            }
                            let idx = vec_c.len as usize;
                            vec_c.len += 1;
                            total_bytes_30 = total_bytes_30.wrapping_add(byte_len);
                            let dst = vec_c.data.wrapping_add(idx * 0x30);
                            wr::<u64>(dst, fn_38);
                            wr::<u64>(dst + 0x08, src_ptr);
                            wr::<u64>(dst + 0x10, dst_base);
                            wr::<u64>(dst + 0x18, range_pair);
                            wr::<u32>(dst + 0x20, cur_p5_cnt);
                            wr::<u32>(dst + 0x24, 0);
                            wr::<u64>(dst + 0x28, byte_len);
                        }
                    }
                }

                let num_ranges = range_end_idx.wrapping_sub(range_start_idx) as usize;
                let mut r_ptr = vec_a.data.wrapping_add((range_start_idx as usize) * 8);
                let r_end = r_ptr.wrapping_add(num_ranges * 8);
                while r_ptr != r_end {
                    let range_val = rd::<u64>(r_ptr);
                    let cur_p5_cnt = rd::<u32>(p5_cnt_ptr);
                    if vec_d_len == vec_d_cap {
                        let old_cap = vec_d_cap as u64;
                        let new_cap = (old_cap * 3).max((vec_d_len + 1) as u64);
                        let new_bytes = new_cap << 4;
                        let mut new_data = 0usize;
                        let mut reused = false;
                        if vec_d_data == 0 {
                            new_data =
                                exe::call_va!(0x1_411f_11e0, (param_2, new_bytes, 4u64) -> usize);
                            if new_data != 0 {
                                reused = true;
                            }
                        } else {
                            let b_start = rd::<usize>(param_2);
                            let b_end =
                                b_start.wrapping_add((rd::<u32>(param_2 + 0x08) as usize) * 8);
                            if b_start != b_end {
                                let prev_off =
                                    rd::<u64>(param_2 + 0x10).wrapping_sub(old_cap << 4);
                                if rd::<usize>(b_end - 8).wrapping_add(prev_off as usize)
                                    == vec_d_data
                                {
                                    let next_off = prev_off.wrapping_add(new_bytes);
                                    if next_off <= rd::<u64>(param_2 + 0x18) {
                                        wr::<u64>(param_2 + 0x10, next_off);
                                        new_data = vec_d_data;
                                        reused = true;
                                    }
                                }
                            }
                        }
                        if !reused {
                            new_data =
                                exe::call_va!(0x1_411f_11e0, (param_2, new_bytes, 4u64) -> usize);
                            for i in 0..(vec_d_len as usize) {
                                wr::<u128>(
                                    new_data.wrapping_add(i * 16),
                                    rd::<u128>(vec_d_data.wrapping_add(i * 16)),
                                );
                            }
                        }
                        vec_d_data = new_data;
                        vec_d_cap = new_cap as u32;
                    }

                    let dst = vec_d_data.wrapping_add((vec_d_len as usize) * 16);
                    vec_d_len += 1;
                    wr::<u64>(dst, range_val);
                    wr::<u32>(dst + 0x08, cur_p5_cnt);
                    wr::<u16>(dst + 0x0c, arch);
                    wr::<u16>(dst + 0x0e, arch);

                    let delta = ((range_val >> 32) as u32).wrapping_sub(range_val as u32);
                    wr::<u32>(p5_cnt_ptr, rd::<u32>(p5_cnt_ptr).wrapping_add(delta));
                    r_ptr = r_ptr.wrapping_add(8);
                }
            }
        }
    }

    wr::<usize>(param_1, vec_a.arena);
    wr::<usize>(param_1 + 0x08, vec_a.data);
    wr::<u32>(param_1 + 0x10, vec_a.len);
    wr::<u32>(param_1 + 0x14, vec_a.cap);
    wr::<usize>(param_1 + 0x18, vec_b.arena);
    wr::<usize>(param_1 + 0x20, vec_b.data);
    wr::<u32>(param_1 + 0x28, vec_b.len);
    wr::<u32>(param_1 + 0x2c, vec_b.cap);
    wr::<usize>(param_1 + 0x30, vec_c.arena);
    wr::<usize>(param_1 + 0x38, vec_c.data);
    wr::<u32>(param_1 + 0x40, vec_c.len);
    wr::<u32>(param_1 + 0x44, vec_c.cap);
    wr::<u64>(param_1 + 0x48, total_bytes_28);
    wr::<u64>(param_1 + 0x50, total_bytes_30);
    wr::<usize>(param_1 + 0x58, param_2);
    let final_d_bytes = (vec_d_len as u64) << 4;
    let final_d_ptr = exe::call_va!(0x1_411f_11e0, (param_2, final_d_bytes, 4u64) -> usize);
    wr::<usize>(param_1 + 0x60, final_d_ptr);
    wr::<u32>(param_1 + 0x68, vec_d_len);
    wr::<u32>(param_1 + 0x6c, vec_d_len);
    let _ = exe::call_va!(0x1_4183_acbc, (final_d_ptr, vec_d_data, final_d_bytes) -> usize);
    param_1
}

/// `unknown_1420c2600`
/// Original: 0x1420c2600 (1523 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420c2600(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
    param_7: usize,
    param_8: usize,
) -> usize {
    let count = rd::<u32>(param_3);
    let base_b0 = rd::<usize>(param_6);

    let mut vec_a_data = exe::call_va!(0x1_411f_11e0, (param_2, 0x100u64, 4u64) -> usize);
    if vec_a_data == 0 {
        vec_a_data = exe::call_va!(0x1_411f_11e0, (param_2, 0x100u64, 4u64) -> usize);
    }
    let mut vec_a = ArenaVec {
        arena: param_2,
        data: vec_a_data,
        len: 0,
        cap: 0x20,
    };

    let mut vec_b = ArenaVec {
        arena: param_2,
        data: 0,
        len: 0,
        cap: count,
    };
    let mut vec_c = ArenaVec {
        arena: param_2,
        data: 0,
        len: 0,
        cap: count,
    };
    if count != 0 {
        let bytes_b = (count as u64) * 0x28;
        let mut b_ptr = exe::call_va!(0x1_411f_11e0, (param_2, bytes_b, 8u64) -> usize);
        if b_ptr == 0 {
            b_ptr = exe::call_va!(0x1_411f_11e0, (param_2, bytes_b, 8u64) -> usize);
        }
        vec_b.data = b_ptr;

        let bytes_c = (count as u64) * 0x30;
        let mut c_ptr = exe::call_va!(0x1_411f_11e0, (param_2, bytes_c, 8u64) -> usize);
        if c_ptr == 0 {
            c_ptr = exe::call_va!(0x1_411f_11e0, (param_2, bytes_c, 8u64) -> usize);
        }
        vec_c.data = c_ptr;
    }

    let mut total_bytes_28: u64 = 0;
    let mut total_bytes_30: u64 = 0;

    if count != 0 {
        let in_arr = rd::<usize>(param_3 + 0x08);
        let mut i: u32 = 0;
        while i < count {
            let entry = rd::<u64>(in_arr.wrapping_add((i as usize) * 8));
            let w_mid = (entry >> 32) as u16;
            let w_hi = (entry >> 48) as u16;
            if (entry as i32) == -1 || w_mid == 0xffff {
                i += 1;
                continue;
            }

            let mut end_i = i + 1;
            while end_i < count {
                let e_ptr = in_arr.wrapping_add((end_i as usize) * 8);
                if rd::<u16>(e_ptr + 0x04) != w_mid || rd::<u16>(e_ptr + 0x06) != w_hi {
                    break;
                }
                end_i += 1;
            }

            let range_start_idx = vec_a.len;
            let mut total_count: u64 = 0;
            let mut cur = i;
            while cur < end_i {
                let run_start = cur;
                let mut last = cur;
                cur += 1;
                while cur < end_i {
                    if rd::<u32>(in_arr.wrapping_add((cur as usize) * 8))
                        == rd::<u32>(in_arr.wrapping_add((last as usize) * 8)).wrapping_add(1)
                    {
                        last = cur;
                        cur += 1;
                    } else {
                        break;
                    }
                }
                let start_val = rd::<u32>(in_arr.wrapping_add((run_start as usize) * 8));
                let end_val = rd::<u32>(in_arr.wrapping_add((last as usize) * 8)).wrapping_add(1);
                if vec_a.len == vec_a.cap {
                    let new_cap = ((vec_a.cap as u64) * 3).max((vec_a.len + 1) as u64);
                    exe::call_va!(
                        0x1_411f_2650,
                        ((&mut vec_a as *mut ArenaVec) as usize, new_cap)
                    );
                }
                let idx = vec_a.len as usize;
                vec_a.len += 1;
                wr::<u64>(
                    vec_a.data.wrapping_add(idx * 8),
                    (start_val as u64) | ((end_val as u64) << 32),
                );
                total_count = total_count.wrapping_add(end_val.wrapping_sub(start_val) as u64);
            }
            let range_end_idx = vec_a.len;

            let arch_hi = w_hi as usize;
            let arch_mid = w_mid as usize;
            let src_base = rd::<u64>(rd::<usize>(param_4 + 0x08).wrapping_add(arch_hi * 8));
            let len1 =
                rd::<u32>(param_5.wrapping_add(arch_hi * 0x20).wrapping_add(0x1806c)) as usize;
            let list1_ptr = rd::<usize>(param_5.wrapping_add(arch_hi * 0x20).wrapping_add(0x18058));
            let p8_entry = rd::<usize>(param_8 + 0x08).wrapping_add(arch_mid * 0x30);
            let dst_base_block = rd::<u64>(p8_entry + 0x08);
            let p7_val = rd::<u32>(rd::<usize>(param_7 + 0x08).wrapping_add((i as usize) * 4));
            let dst_arch = rd::<u16>(p8_entry) as usize;
            let list2_ptr =
                rd::<usize>(param_5.wrapping_add(dst_arch * 0x20).wrapping_add(0x18058));
            let len2 =
                rd::<u32>(param_5.wrapping_add(dst_arch * 0x20).wrapping_add(0x1806c)) as usize;

            if len1 != 0 {
                let range_pair = (range_start_idx as u64) | ((range_end_idx as u64) << 32);
                let mut idx1: usize = 0;
                let mut idx2: usize = 0;
                while idx1 < len1 {
                    if idx2 >= len2 {
                        break;
                    }
                    let c1 = rd::<u16>(list1_ptr.wrapping_add(idx1 * 2));
                    let c2 = rd::<u16>(list2_ptr.wrapping_add(idx2 * 2));
                    if c1 < c2 {
                        idx1 += 1;
                    } else {
                        if c1 == c2 {
                            let desc_ptr = base_b0.wrapping_add((c1 as usize) * 0x50);
                            let off_tbl1 =
                                rd::<usize>(param_5.wrapping_add((arch_hi + 0xc03) * 0x20));
                            let src_ptr = src_base
                                .wrapping_add(rd::<u32>(off_tbl1.wrapping_add(idx1 * 4)) as u64);
                            let fn_38 = rd::<u64>(desc_ptr + 0x38);
                            let elem_size = rd::<u64>(desc_ptr + 0x10);
                            let off_tbl2 = rd::<usize>(p8_entry + 0x18);
                            let dst_base = dst_base_block
                                .wrapping_add(rd::<u32>(off_tbl2.wrapping_add(idx2 * 4)) as u64);
                            let byte_len = elem_size.wrapping_mul(total_count);

                            if fn_38 == 0 {
                                let dst_ptr =
                                    dst_base.wrapping_add((p7_val as u64).wrapping_mul(elem_size));
                                if vec_b.len == vec_b.cap {
                                    let new_cap =
                                        ((vec_b.cap as u64) * 3).max((vec_b.len + 1) as u64);
                                    exe::call_va!(
                                        0x1_411f_2730,
                                        ((&mut vec_b as *mut ArenaVec) as usize, new_cap)
                                    );
                                }
                                let idx = vec_b.len as usize;
                                vec_b.len += 1;
                                total_bytes_28 = total_bytes_28.wrapping_add(byte_len);
                                let dst = vec_b.data.wrapping_add(idx * 0x28);
                                wr::<u64>(dst, src_ptr);
                                wr::<u64>(dst + 0x08, dst_ptr);
                                wr::<u64>(dst + 0x10, elem_size);
                                wr::<u64>(dst + 0x18, range_pair);
                                wr::<u64>(dst + 0x20, byte_len);
                            } else {
                                if vec_c.len == vec_c.cap {
                                    let new_cap =
                                        ((vec_c.cap as u64) * 3).max((vec_c.len + 1) as u64);
                                    exe::call_va!(
                                        0x1_411f_2830,
                                        ((&mut vec_c as *mut ArenaVec) as usize, new_cap)
                                    );
                                }
                                let idx = vec_c.len as usize;
                                vec_c.len += 1;
                                total_bytes_30 = total_bytes_30.wrapping_add(byte_len);
                                let dst = vec_c.data.wrapping_add(idx * 0x30);
                                wr::<u64>(dst, fn_38);
                                wr::<u64>(dst + 0x08, src_ptr);
                                wr::<u64>(dst + 0x10, dst_base);
                                wr::<u64>(dst + 0x18, range_pair);
                                wr::<u32>(dst + 0x20, p7_val);
                                wr::<u32>(dst + 0x24, 0);
                                wr::<u64>(dst + 0x28, byte_len);
                            }
                            idx1 += 1;
                        }
                        idx2 += 1;
                    }
                }
            }
            i = end_i;
        }
    }

    wr::<usize>(param_1, vec_a.arena);
    wr::<usize>(param_1 + 0x08, vec_a.data);
    wr::<u32>(param_1 + 0x10, vec_a.len);
    wr::<u32>(param_1 + 0x14, vec_a.cap);
    wr::<usize>(param_1 + 0x18, vec_b.arena);
    wr::<usize>(param_1 + 0x20, vec_b.data);
    wr::<u32>(param_1 + 0x28, vec_b.len);
    wr::<u32>(param_1 + 0x2c, vec_b.cap);
    wr::<usize>(param_1 + 0x30, vec_c.arena);
    wr::<usize>(param_1 + 0x38, vec_c.data);
    wr::<u32>(param_1 + 0x40, vec_c.len);
    wr::<u32>(param_1 + 0x44, vec_c.cap);
    wr::<u64>(param_1 + 0x48, total_bytes_28);
    wr::<u64>(param_1 + 0x50, total_bytes_30);
    param_1
}

/// `unknown_1420c2d60`
/// Original: 0x1420c2d60 (669 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420c2d60(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: u16,
    param_6: u32,
) -> usize {
    let arch = param_5 as usize;
    let arch_off = arch << 5;
    let comp_count = rd::<u32>(param_4.wrapping_add(arch_off).wrapping_add(0x180ac));
    let comp_ids_ptr = rd::<usize>(param_4.wrapping_add(arch_off).wrapping_add(0x18098));
    let mut cur_offset: u64 = (param_6 as u64).wrapping_mul(8).wrapping_add(0x10);

    let mut vec_u32 = ArenaVec {
        arena: param_2,
        data: 0,
        len: 0,
        cap: comp_count,
    };
    if comp_count != 0 {
        let byte_len = (comp_count as u64) * 4;
        let mut p = exe::call_va!(0x1_411f_11e0, (param_2, byte_len, 4u64) -> usize);
        if p == 0 {
            p = exe::call_va!(0x1_411f_11e0, (param_2, byte_len, 4u64) -> usize);
        }
        vec_u32.data = p;
    }

    let end_ptr = comp_ids_ptr.wrapping_add((comp_count as usize) * 2);
    let mut cur_ptr = comp_ids_ptr;
    while cur_ptr != end_ptr {
        let comp_id = rd::<u16>(cur_ptr) as usize;
        let desc_ptr = rd::<usize>(param_3).wrapping_add(comp_id * 0x50);
        let align = rd::<u64>(desc_ptr + 0x18);
        cur_offset = cur_offset.wrapping_sub(1).wrapping_add(align) & !align.wrapping_sub(1);

        if vec_u32.len == vec_u32.cap {
            let new_cap = ((vec_u32.cap as u64) * 3).max((vec_u32.len + 1) as u64);
            exe::call_va!(
                0x1_411f_2490,
                ((&mut vec_u32 as *mut ArenaVec) as usize, new_cap)
            );
        }
        let idx = vec_u32.len as usize;
        cur_ptr = cur_ptr.wrapping_add(2);
        vec_u32.len += 1;
        wr::<u32>(vec_u32.data.wrapping_add(idx * 4), cur_offset as u32);
        let elem_size = rd::<u64>(desc_ptr + 0x10);
        cur_offset = cur_offset.wrapping_add((param_6 as u64).wrapping_mul(elem_size));
    }

    let tls0 = rd::<usize>(exe::tls_pointer());
    let heap = rd::<usize>(tls0 + 0x6348);
    let mut block = if (cur_offset & cur_offset.wrapping_sub(1)) == 0
        && cur_offset.wrapping_sub(0x10) <= 0x3f0
    {
        let bin_idx = (cur_offset.wrapping_add(7) >> 3) as usize;
        let page = rd::<usize>(heap.wrapping_add(bin_idx * 8).wrapping_add(0x08));
        let head = rd::<usize>(page + 0x10);
        if head == 0 {
            exe::call_va!(0x1_4253_7280, (heap, cur_offset, 0usize, 0usize) -> usize)
        } else {
            wr::<u32>(page + 0x18, rd::<u32>(page + 0x18).wrapping_add(1));
            wr::<usize>(page + 0x10, rd::<usize>(head));
            head
        }
    } else if cur_offset <= 0x7fff_ffff_ffff_ffff {
        let mut fast = 0usize;
        if cur_offset.wrapping_sub(0x10) <= 0x3f0 {
            let bin_idx = (cur_offset.wrapping_add(7) >> 3) as usize;
            let page = rd::<usize>(heap.wrapping_add(bin_idx * 8).wrapping_add(0x08));
            let head = rd::<usize>(page + 0x10);
            if head != 0 && (head & 0x0f) == 0 {
                wr::<u32>(page + 0x18, rd::<u32>(page + 0x18).wrapping_add(1));
                wr::<usize>(page + 0x10, rd::<usize>(head));
                fast = head;
            }
        }
        if fast != 0 {
            fast
        } else {
            exe::call_va!(0x1_4253_5070, (heap, cur_offset, 0x10usize) -> usize)
        }
    } else {
        0
    };

    if block == 0 {
        let proc = exe::call_iat!(0x1_4264_56c8, () -> usize);
        exe::call_iat!(0x1_4264_53a0, (proc, 0xdead_d00du32));
        block = 0;
    }

    wr::<usize>(block, param_4);
    wr::<u16>(block + 0x08, param_5);
    wr::<u16>(param_1, param_5);
    wr::<u32>(param_1 + 0x24, vec_u32.cap);
    wr::<usize>(param_1 + 0x08, block);
    wr::<usize>(param_1 + 0x10, vec_u32.arena);
    wr::<usize>(param_1 + 0x18, vec_u32.data);
    wr::<u32>(param_1 + 0x20, vec_u32.len);
    wr::<u32>(param_1 + 0x28, 0);
    param_1
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_r_1420b55a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, count: u64| -> (u64, [u32; 5]) {
        let mut grp0 = [0u8; 0x30];
        let mut grp1 = [0u8; 0x30];
        let g0 = grp0.as_mut_ptr() as usize;
        let g1 = grp1.as_mut_ptr() as usize;
        wr::<u32>(g0 + 0x10 + 0 * 8, 5); // < 10 -> pos 0 (writes to out[-1], so keep bounds[0] = 0!)
        wr::<u32>(g0 + 0x10 + 1 * 8, 10); // == 10 -> pos 1
        wr::<u32>(g1 + 0x10 + 0 * 8, 25); // between 20 and 30 -> pos 2
        wr::<u32>(g1 + 0x10 + 2 * 8, 99); // >= 40 -> pos 4

        let mut ctx = [0u8; 0x60];
        let cp = ctx.as_mut_ptr() as usize;
        wr::<usize>(cp + 0x48 + 0 * 8, g0);
        wr::<usize>(cp + 0x48 + 1 * 8, g1);

        let bounds: [u32; 4] = [0, 10, 20, 40];
        let items: [u64; 4] = [
            (0u64 << 48) | 0,
            (0u64 << 48) | 1,
            (1u64 << 48) | 0,
            (1u64 << 48) | 2,
        ];
        let mut out_buf = [0xdead_beefu32; 5];
        let out_ptr = (out_buf.as_mut_ptr() as usize) + 4; // allow pos=1 -> out_buf[1]

        let mut job = [0u8; 0x48];
        let jp = job.as_mut_ptr() as usize;
        wr::<u64>(jp + 0x08, count);
        wr::<usize>(jp + 0x10, items.as_ptr() as usize);
        wr::<u64>(jp + 0x18, 100);
        wr::<usize>(jp + 0x20, cp);
        wr::<u64>(jp + 0x28, 4);
        wr::<usize>(jp + 0x30, bounds.as_ptr() as usize);
        wr::<usize>(jp + 0x40, out_ptr);

        f(jp);
        (rd::<u64>(jp + 0x18), out_buf)
    };

    for cnt in [0u64, 4] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "1420b55a0 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1420bdea0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, has_dups: bool| -> (u32, u32, u64, [u64; 6], [u64; 4]) {
        let mut backing = vec![0u8; 0x1000];
        let bp = backing.as_mut_ptr() as usize;
        let blocks: [usize; 1] = [bp];
        let mut arena: [u64; 4] = [blocks.as_ptr() as u64, 1, 0, 0x1000];
        let ap = arena.as_mut_ptr() as usize;

        // Linked list nodes for +0x108 (u32s), +0x120 (u16s), +0x138 (u64s)
        let mut n108_b = [0u8; 0x20];
        let mut n108_a = [0u8; 0x20];
        wr::<usize>(n108_a.as_mut_ptr() as usize, n108_b.as_mut_ptr() as usize);
        wr::<u64>(n108_a.as_mut_ptr() as usize + 0x08, 2);
        wr::<u32>(n108_a.as_mut_ptr() as usize + 0x14, 0x1111_2222); // idx 1
        wr::<u64>(n108_b.as_mut_ptr() as usize + 0x08, 4);
        wr::<u32>(n108_b.as_mut_ptr() as usize + 0x10, 0x1111_2222); // idx 0 after wrap

        let mut n120_b = [0u8; 0x20];
        let mut n120_a = [0u8; 0x20];
        wr::<usize>(n120_a.as_mut_ptr() as usize, n120_b.as_mut_ptr() as usize);
        wr::<u64>(n120_a.as_mut_ptr() as usize + 0x08, 2);
        wr::<u16>(n120_a.as_mut_ptr() as usize + 0x12, 0x0033); // idx 1
        wr::<u64>(n120_b.as_mut_ptr() as usize + 0x08, 4);
        wr::<u16>(n120_b.as_mut_ptr() as usize + 0x10, 0x0033); // idx 0 after wrap

        let mut n138_b = [0u64; 4];
        n138_b[1] = 4;
        n138_b[2] = 0xbbbb_0002; // idx 0 after wrap
        let mut n138_a = [0u64; 4];
        n138_a[0] = n138_b.as_mut_ptr() as u64;
        n138_a[1] = 2;
        n138_a[3] = 0xaaaa_0001; // idx 1

        let mut p3 = vec![0u8; 0x150];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<usize>(p3p + 0x108, n108_a.as_mut_ptr() as usize);
        wr::<u64>(p3p + 0x118, 5);
        wr::<usize>(p3p + 0x120, n120_a.as_mut_ptr() as usize);
        wr::<usize>(p3p + 0x138, n138_a.as_mut_ptr() as usize);

        let dup_key = (0x1111_2222u64 << 32) | 0x0033u64;
        let items: [u64; 4] = if has_dups {
            [0x10, dup_key, dup_key, 0x20]
        } else {
            [0x10, 0x20, 0x30, 0x40]
        };
        let p6: [u64; 2] = [4, items.as_ptr() as u64];

        let mut out = [0u64; 3];
        let op = out.as_mut_ptr() as usize;
        let ret = f(op, ap, p3p, 0, 0, p6.as_ptr() as usize);
        assert_eq!(ret, op);

        let len = rd::<u32>(op + 0x10);
        let cap = rd::<u32>(op + 0x14);
        let data = rd::<usize>(op + 0x08);
        let mut entries = [0u64; 6];
        if data != 0 {
            for i in 0..(len as usize) {
                let ep = data + i * 16;
                wr::<u16>(ep + 0x06, 0); // zero 2-byte uninitialized stack padding hole in orig
                entries[i * 2] = rd::<u64>(ep);
                entries[i * 2 + 1] = rd::<u64>(ep + 0x08);
            }
        }
        (len, cap, arena[2], entries, [n138_a[2], n138_a[3], n138_b[2], n138_b[3]])
    };

    for has_dups in [false, true] {
        let o = run(f_o, has_dups);
        let n = run(f_n, has_dups);
        crate::diff_eq!(o, n, "1420bdea0 has_dups={}", has_dups);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1420c1d70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, count: u32| -> ([u64; 14], u64, u32, [u64; 4], [u64; 5], [u64; 6], [u64; 4]) {
        let mut backing = vec![0u8; 0x8000];
        let bp = backing.as_mut_ptr() as usize;
        let blocks: [usize; 1] = [bp];
        let mut arena: [u64; 4] = [blocks.as_ptr() as u64, 1, 0, 0x8000];
        let ap = arena.as_mut_ptr() as usize;

        // 2 items for arch 1: indices 3 and 1 (unsorted), ent_count = 5 -> gaps [0,1), [2,3), [4,5)
        let in_items: [u64; 2] = [(1u64 << 32) | 3, (1u64 << 32) | 1];
        let mut p3 = [0u64; 2];
        wr::<u32>(p3.as_mut_ptr() as usize, count);
        wr::<usize>(p3.as_mut_ptr() as usize + 0x08, in_items.as_ptr() as usize);

        let p4_arr: [u64; 2] = [0, 0x10_0000];
        let p4: [u64; 2] = [0, p4_arr.as_ptr() as u64];

        let off_tbl_b: [u32; 2] = [0x20, 0x40];
        let mut p5_entries = [0u8; 0x60];
        let p5e = p5_entries.as_mut_ptr() as usize + 0x30; // arch 1
        wr::<u64>(p5e + 0x08, 0x20_0000);
        wr::<usize>(p5e + 0x18, off_tbl_b.as_ptr() as usize);
        wr::<u32>(p5e + 0x28, 10);
        let p5: [u64; 2] = [0, p5_entries.as_mut_ptr() as u64];

        // Component descriptors at base_98 (comp 0 has fn_38 == 0, comp 1 has fn_38 != 0)
        let mut descs = [0u8; 0xa0];
        let dp = descs.as_mut_ptr() as usize;
        wr::<u64>(dp + 0x10, 8); // comp 0 elem_size = 8, fn_38 = 0
        wr::<u64>(dp + 0x50 + 0x10, 16); // comp 1 elem_size = 16
        wr::<u64>(dp + 0x50 + 0x38, 0xfeed_face); // comp 1 fn_38 != 0
        let p6: [usize; 1] = [dp];

        let comp_ids: [u16; 2] = [0, 1];
        let off_tbl_a: [u32; 2] = [0x100, 0x200];
        let mut p7 = vec![0u8; 0x19000];
        let p7p = p7.as_mut_ptr() as usize;
        wr::<u64>(p7p + 0x48 + 1 * 8, 1);
        wr::<u32>(p7p + 0x10048 + 1 * 4, 5);
        wr::<usize>(p7p + 0x20 + 0x18058, comp_ids.as_ptr() as usize);
        wr::<u32>(p7p + 0x20 + 0x1806c, 2);
        wr::<usize>(p7p + (1 + 0xc03) * 0x20, off_tbl_a.as_ptr() as usize);

        let mut out = [0u64; 14];
        let op = out.as_mut_ptr() as usize;
        let ret = f(
            op,
            ap,
            p3.as_ptr() as usize,
            p4.as_ptr() as usize,
            p5.as_ptr() as usize,
            p6.as_ptr() as usize,
            p7p,
        );
        assert_eq!(ret, op);

        let vec_a_data = rd::<usize>(op + 0x08);
        let vec_b_data = rd::<usize>(op + 0x20);
        let vec_c_data = rd::<usize>(op + 0x38);
        let vec_d_data = rd::<usize>(op + 0x60);

        let mut a_snap = [0u64; 4];
        let mut b_snap = [0u64; 5];
        let mut c_snap = [0u64; 6];
        let mut d_snap = [0u64; 4];
        if count != 0 {
            a_snap = rd::<[u64; 4]>(vec_a_data);
            b_snap = rd::<[u64; 5]>(vec_b_data);
            wr::<u32>(vec_c_data + 0x24, 0); // zero 4-byte stack padding hole in orig
            c_snap = rd::<[u64; 6]>(vec_c_data);
            d_snap = rd::<[u64; 4]>(vec_d_data);
        }

        // Normalize arena/data pointers in `out` to relative offsets
        out[0] = (out[0] as usize).wrapping_sub(ap) as u64;
        out[1] = if vec_a_data != 0 { (vec_a_data - bp) as u64 } else { 0 };
        out[3] = (out[3] as usize).wrapping_sub(ap) as u64;
        out[4] = if vec_b_data != 0 { (vec_b_data - bp) as u64 } else { 0 };
        out[6] = (out[6] as usize).wrapping_sub(ap) as u64;
        out[7] = if vec_c_data != 0 { (vec_c_data - bp) as u64 } else { 0 };
        out[11] = (out[11] as usize).wrapping_sub(ap) as u64;
        out[12] = if vec_d_data != 0 { (vec_d_data - bp) as u64 } else { 0 };

        (out, arena[2], rd::<u32>(p5e + 0x28), a_snap, b_snap, c_snap, d_snap)
    };

    for cnt in [0u32, 2] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "1420c1d70 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1420c2600(orig: usize, new: usize) -> Result<u32, String> {
    type F =
        unsafe extern "win64" fn(usize, usize, usize, usize, usize, usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, count: u32| -> ([u64; 11], u64, [u64; 2], [u64; 5], [u64; 6]) {
        let mut backing = vec![0u8; 0x8000];
        let bp = backing.as_mut_ptr() as usize;
        let blocks: [usize; 1] = [bp];
        let mut arena: [u64; 4] = [blocks.as_ptr() as u64, 1, 0, 0x8000];
        let ap = arena.as_mut_ptr() as usize;

        // 4 items:
        // [0]: sentinel low_i32 == -1 (skipped)
        // [1]: w_hi=1, w_mid=1, idx=10
        // [2]: w_hi=1, w_mid=1, idx=11 (coalesced with 10 -> [10, 12))
        // [3]: w_hi=1, w_mid=1, idx=15 (second run -> [15, 16))
        let in_items: [u64; 4] = [
            0xffff_ffff,
            (1u64 << 48) | (1u64 << 32) | 10,
            (1u64 << 48) | (1u64 << 32) | 11,
            (1u64 << 48) | (1u64 << 32) | 15,
        ];
        let mut p3 = [0u64; 2];
        wr::<u32>(p3.as_mut_ptr() as usize, count);
        wr::<usize>(p3.as_mut_ptr() as usize + 0x08, in_items.as_ptr() as usize);

        let p4_arr: [u64; 2] = [0, 0x30_0000];
        let p4: [u64; 2] = [0, p4_arr.as_ptr() as u64];

        let list1: [u16; 3] = [0, 2, 3]; // arch_hi = 1
        let list2: [u16; 3] = [1, 2, 3]; // dst_arch = 2 (matches on comp 2 and comp 3)
        let off_tbl1: [u32; 3] = [0x10, 0x20, 0x30];
        let off_tbl2: [u32; 3] = [0x100, 0x200, 0x300];

        let mut p5 = vec![0u8; 0x19000];
        let p5p = p5.as_mut_ptr() as usize;
        wr::<usize>(p5p + 1 * 0x20 + 0x18058, list1.as_ptr() as usize);
        wr::<u32>(p5p + 1 * 0x20 + 0x1806c, 3);
        wr::<usize>(p5p + 2 * 0x20 + 0x18058, list2.as_ptr() as usize);
        wr::<u32>(p5p + 2 * 0x20 + 0x1806c, 3);
        wr::<usize>(p5p + (1 + 0xc03) * 0x20, off_tbl1.as_ptr() as usize);

        let mut descs = [0u8; 0x50 * 4];
        let dp = descs.as_mut_ptr() as usize;
        wr::<u64>(dp + 2 * 0x50 + 0x10, 8); // comp 2: elem_size = 8, fn_38 = 0
        wr::<u64>(dp + 3 * 0x50 + 0x10, 12); // comp 3: elem_size = 12, fn_38 != 0
        wr::<u64>(dp + 3 * 0x50 + 0x38, 0x1234_5678);
        let p6: [usize; 1] = [dp];

        let p7_arr: [u32; 4] = [0, 7, 0, 0];
        let p7: [u64; 2] = [0, p7_arr.as_ptr() as u64];

        let mut p8_entries = [0u8; 0x60];
        let p8e = p8_entries.as_mut_ptr() as usize + 0x30; // arch_mid = 1
        wr::<u16>(p8e, 2); // dst_arch = 2
        wr::<u64>(p8e + 0x08, 0x40_0000);
        wr::<usize>(p8e + 0x18, off_tbl2.as_ptr() as usize);
        let p8: [u64; 2] = [0, p8_entries.as_mut_ptr() as u64];

        let mut out = [0u64; 11];
        let op = out.as_mut_ptr() as usize;
        let ret = f(
            op,
            ap,
            p3.as_ptr() as usize,
            p4.as_ptr() as usize,
            p5p,
            p6.as_ptr() as usize,
            p7.as_ptr() as usize,
            p8.as_ptr() as usize,
        );
        assert_eq!(ret, op);

        let vec_a_data = rd::<usize>(op + 0x08);
        let vec_b_data = rd::<usize>(op + 0x20);
        let vec_c_data = rd::<usize>(op + 0x38);

        let mut a_snap = [0u64; 2];
        let mut b_snap = [0u64; 5];
        let mut c_snap = [0u64; 6];
        if count != 0 {
            a_snap = rd::<[u64; 2]>(vec_a_data);
            b_snap = rd::<[u64; 5]>(vec_b_data);
            wr::<u32>(vec_c_data + 0x24, 0); // zero 4-byte stack padding hole in orig
            c_snap = rd::<[u64; 6]>(vec_c_data);
        }

        out[0] = (out[0] as usize).wrapping_sub(ap) as u64;
        out[1] = if vec_a_data != 0 { (vec_a_data - bp) as u64 } else { 0 };
        out[3] = (out[3] as usize).wrapping_sub(ap) as u64;
        out[4] = if vec_b_data != 0 { (vec_b_data - bp) as u64 } else { 0 };
        out[6] = (out[6] as usize).wrapping_sub(ap) as u64;
        out[7] = if vec_c_data != 0 { (vec_c_data - bp) as u64 } else { 0 };

        (out, arena[2], a_snap, b_snap, c_snap)
    };

    for cnt in [0u32, 4] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "1420c2600 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1420c2d60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, u16, u32) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let tls0 = rd::<usize>(exe::tls_pointer());
    let saved_heap = rd::<usize>(tls0 + 0x6348);

    let run = |f: F, ent_cap: u32| -> (u16, u32, u32, u32, [u32; 2], u64, u16, [u64; 4], u64) {
        let mut backing = vec![0u8; 0x1000];
        let bp = backing.as_mut_ptr() as usize;
        let blocks: [usize; 1] = [bp];
        let mut arena: [u64; 4] = [blocks.as_ptr() as u64, 1, 0, 0x1000];
        let ap = arena.as_mut_ptr() as usize;

        // 2 components:
        // comp 0: align = 16, elem_size = 8
        // comp 1: align = 16, elem_size = 8
        let mut descs = [0u8; 0xa0];
        let dp = descs.as_mut_ptr() as usize;
        wr::<u64>(dp + 0x10, 8);
        wr::<u64>(dp + 0x18, 16);
        wr::<u64>(dp + 0x50 + 0x10, 8);
        wr::<u64>(dp + 0x50 + 0x18, 16);
        let p3: [usize; 1] = [dp];

        let comp_ids: [u16; 2] = [0, 1];
        let mut p4 = vec![0u8; 0x18200];
        let p4p = p4.as_mut_ptr() as usize;
        wr::<usize>(p4p + 1 * 0x20 + 0x18098, comp_ids.as_ptr() as usize);
        wr::<u32>(p4p + 1 * 0x20 + 0x180ac, 2);

        // Aligned block for fast-bin freelist pop
        #[repr(align(16))]
        struct AlignedBlock([u64; 4]);
        let mut free_block = AlignedBlock([0xdead_beef, 0, 0, 0]);
        let fbp = free_block.0.as_mut_ptr() as usize;

        let mut page: [u64; 4] = [0, 0, fbp as u64, 7];
        let mut heap = [0usize; 140];
        for slot in &mut heap {
            *slot = page.as_mut_ptr() as usize;
        }
        wr::<usize>(tls0 + 0x6348, heap.as_mut_ptr() as usize);

        let mut out = [0u8; 0x30];
        let op = out.as_mut_ptr() as usize;
        let ret = f(op, ap, p3.as_ptr() as usize, p4p, 1, ent_cap);
        assert_eq!(ret, op);

        let vec_data = rd::<usize>(op + 0x18);
        let offs = rd::<[u32; 2]>(vec_data);
        let blk_p4 = (rd::<usize>(fbp)).wrapping_sub(p4p) as u64;
        let blk_arch = rd::<u16>(fbp + 0x08);
        page[2] = 0; // clear next-pointer if untouched
        (
            rd::<u16>(op),
            rd::<u32>(op + 0x20),
            rd::<u32>(op + 0x24),
            rd::<u32>(op + 0x28),
            offs,
            blk_p4,
            blk_arch,
            page,
            arena[2],
        )
    };

    // ent_cap = 2 -> cur_offset = 16 + 16 -> 32 + 16 = 48 -> + 16 = 64 (power of two!)
    // ent_cap = 3 -> cur_offset = 16 + 24 -> 48 + 24 = 72 -> + 24 = 96 (non-power of two, <= 0x400!)
    for cap in [2u32, 3] {
        let o = run(f_o, cap);
        let n = run(f_n, cap);
        wr::<usize>(tls0 + 0x6348, saved_heap);
        crate::diff_eq!(o, n, "1420c2d60 cap={}", cap);
    }
    Ok(2)
}

// REGISTRY:
// Replacement::verified(0x1_420b_55a0, "r_1420b55a0", rend::cauldron_hot_w5a::r_1420b55a0 as *const (), rend::cauldron_hot_w5a::diff_r_1420b55a0),
// Replacement::verified(0x1_420b_dea0, "unknown_1420bdea0", rend::cauldron_hot_w5a::unknown_1420bdea0 as *const (), rend::cauldron_hot_w5a::diff_unknown_1420bdea0),
// Replacement::verified(0x1_420c_1d70, "unknown_1420c1d70", rend::cauldron_hot_w5a::unknown_1420c1d70 as *const (), rend::cauldron_hot_w5a::diff_unknown_1420c1d70),
// Replacement::verified(0x1_420c_2600, "unknown_1420c2600", rend::cauldron_hot_w5a::unknown_1420c2600 as *const (), rend::cauldron_hot_w5a::diff_unknown_1420c2600),
// Replacement::verified(0x1_420c_2d60, "unknown_1420c2d60", rend::cauldron_hot_w5a::unknown_1420c2d60 as *const (), rend::cauldron_hot_w5a::diff_unknown_1420c2d60),
