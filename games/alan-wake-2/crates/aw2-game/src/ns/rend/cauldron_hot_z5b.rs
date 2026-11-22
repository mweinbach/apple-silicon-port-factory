//! Decompiled `rend` hot functions shard Z5B (`0x1420bf7c0..0x1420c0e20`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

/// `unknown_1420bf7c0`
/// Original: 0x1420bf7c0 (2491 bytes, Throws::Discard)
#[allow(clippy::too_many_arguments)]
pub unsafe extern "win64" fn unknown_1420bf7c0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
    param_7: usize,
    param_8: usize,
    param_9: usize,
) -> usize {
    if rd::<u64>(param_6 + 0xc8) == 0 {
        wr::<u8>(param_1 + 0x10, 0);
        return param_1;
    }

    let grp_cnt = rd::<u64>(param_3) as usize;
    let grp_data = rd::<usize>(param_3 + 0x08);
    let grp_end = grp_data.wrapping_add(grp_cnt.wrapping_mul(0x50));

    let mut vec_u32 = [0u64; 3];
    let vec_ptr = vec_u32.as_mut_ptr() as usize;
    wr::<usize>(vec_ptr, param_2);
    wr::<usize>(vec_ptr + 0x08, 0);
    wr::<u64>(vec_ptr + 0x10, 0);

    let mut cur_data = 0usize;
    let mut cur_len = 0u32;
    let mut cur_cap = 0u32;

    if grp_data != grp_end {
        let mut grp = grp_data;
        loop {
            let sub_beg = rd::<usize>(grp + 0x08);
            let sub_cnt = rd::<u32>(grp + 0x10) as usize;
            let sub_end = sub_beg.wrapping_add(sub_cnt * 4);
            let mut sub_cur = sub_beg;
            while sub_cur != sub_end {
                let val = rd::<u32>(sub_cur);
                if cur_len == cur_cap {
                    let need = (cur_len.wrapping_add(1)) as u64;
                    let grow = (cur_cap as u64).wrapping_mul(3);
                    let new_cap = if grow < need { need } else { grow };
                    exe::call_va!(0x1_411f_2490, (vec_ptr, new_cap));
                    cur_cap = rd::<u32>(vec_ptr + 0x14);
                    cur_len = rd::<u32>(vec_ptr + 0x10);
                    cur_data = rd::<usize>(vec_ptr + 0x08);
                }
                let idx = cur_len as usize;
                cur_len = cur_len.wrapping_add(1);
                wr::<u32>(vec_ptr + 0x10, cur_len);
                wr::<u32>(cur_data + idx * 4, val);
                sub_cur = sub_cur.wrapping_add(4);
            }
            grp = grp.wrapping_add(0x50);
            if grp == grp_end {
                break;
            }
        }
    }

    exe::call_va!(0x1_4103_a0c0, (cur_data, cur_len));

    let moves_cnt = rd::<u64>(param_4);
    let map_buf = exe::call_va!(0x1_411f_11e0, (param_2, moves_cnt.wrapping_mul(4), 4u32) -> usize);

    let job_glob = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let job_mgr = rd::<usize>(job_glob);
    let worker_cnt = rd::<u64>(job_mgr);
    let q_base = rd::<usize>(job_mgr + 0x10);

    let mut parent1_handle = [0u64; 2];
    let p1_hptr = parent1_handle.as_mut_ptr() as usize;
    let t_idx1 = exe::call_va!(0x1_4240_7c80, () -> u32);
    let pool1 = rd::<usize>(q_base + (t_idx1 as usize) * 0x40 + 0x48).wrapping_add(0x40);
    exe::call_va!(0x1_4248_5aa0, (pool1, p1_hptr));

    let p1_node = rd::<usize>(p1_hptr);
    let _ = crate::simd_ops::atomic_xchg_u32(p1_node as u64, 1);
    wr::<u64>(p1_node + 0x08, 0);
    wr::<usize>(p1_node + 0x10, exe::ptr::<u8>(0x1_4340_e320) as usize);
    wr::<usize>(p1_node + 0x18, exe::ptr::<u8>(0x1_433d_da90) as usize);
    wr::<u64>(p1_node + 0x20, 0x49);
    let (ok_c1, _) = crate::simd_ops::atomic_cmpxchg_u8((p1_node + 0x04) as u64, 1, 3);
    if ok_c1 {
        let p1_reload = rd::<usize>(p1_hptr);
        let _ = crate::simd_ops::atomic_add_u32(p1_reload as u64, 0xffff);
    }
    wr::<u8>(p1_hptr + 0x0a, 1);

    let chunk_size = moves_cnt / worker_cnt;
    let mut move_idx: u64 = 0;
    if chunk_size != 0 && (moves_cnt as i64) > (chunk_size as i64) {
        let sorted_len = rd::<u32>(vec_ptr + 0x10) as u64;
        let sorted_ptr = rd::<usize>(vec_ptr + 0x08);
        let moves_cnt_u32 = (moves_cnt as u32) as u64;
        let mut cur_moves_ptr = rd::<usize>(param_4 + 0x08);
        let mut rem_moves = moves_cnt;
        loop {
            let cur_job_mgr = rd::<usize>(job_glob);
            let cur_q = rd::<usize>(cur_job_mgr + 0x10);
            let cur_batch = if chunk_size != u64::MAX {
                chunk_size
            } else {
                moves_cnt.wrapping_sub(move_idx)
            };
            let t_idx = exe::call_va!(0x1_4240_7c80, () -> u32);
            let cpool = rd::<usize>(cur_q + (t_idx as usize) * 0x40 + 0x48).wrapping_add(0x80);
            let mut child1_handle = [0u64; 2];
            let c1_hptr = child1_handle.as_mut_ptr() as usize;
            exe::call_va!(0x1_4248_5aa0, (cpool, c1_hptr));
            let c1_node = rd::<usize>(c1_hptr);
            let _ = crate::simd_ops::atomic_xchg_u32(c1_node as u64, 1);
            wr::<u64>(c1_node + 0x08, 0);
            wr::<usize>(c1_node + 0x10, exe::ptr::<u8>(0x1_433d_ddd0) as usize);
            wr::<u64>(c1_node + 0x18, cur_batch);
            wr::<usize>(c1_node + 0x20, cur_moves_ptr);
            wr::<u64>(c1_node + 0x28, move_idx);
            wr::<usize>(c1_node + 0x30, param_5);
            wr::<u64>(c1_node + 0x38, sorted_len);
            wr::<usize>(c1_node + 0x40, sorted_ptr);
            wr::<u64>(c1_node + 0x48, moves_cnt_u32);
            wr::<usize>(c1_node + 0x50, map_buf);

            let p1_n = rd::<usize>(p1_hptr);
            let _ = crate::simd_ops::atomic_add_u32(p1_n as u64, 1);
            wr::<usize>(c1_node + 0x08, p1_n);
            if crate::simd_ops::rd_acq_u32(c1_node) == 1 {
                let q = rd::<usize>(cur_job_mgr + 0x10);
                exe::call_va!(0x1_4248_6460, (q, c1_node));
                let sem = rd::<usize>(rd::<usize>(q) + 0x1008);
                exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize));
            }
            rem_moves = rem_moves.wrapping_sub(chunk_size);
            move_idx = move_idx.wrapping_add(chunk_size);
            cur_moves_ptr = cur_moves_ptr.wrapping_add((chunk_size as usize) * 8);
            if (rem_moves as i64) <= (chunk_size as i64) {
                break;
            }
        }
    }

    let sorted_ptr = rd::<usize>(vec_ptr + 0x08);
    let sorted_len = rd::<u32>(vec_ptr + 0x10) as u64;
    let mut rem_ptr = rd::<usize>(param_4 + 0x08).wrapping_add((move_idx as usize) * 8);
    let end_ptr = rem_ptr.wrapping_add((moves_cnt.wrapping_sub(move_idx) as usize) * 8);
    while rem_ptr != end_ptr {
        let mv = rd::<u64>(rem_ptr);
        let mv_lo = mv as u32;
        let mv_hi = (mv >> 48) as usize;
        let grp_arr = rd::<usize>(param_5 + 0x48 + mv_hi * 8);
        let target = rd::<u32>(grp_arr + 0x10 + (mv_lo as usize) * 8);

        let mut pos: u64 = 0;
        let mut cnt: u64 = sorted_len;
        while cnt != 0 {
            let half = cnt >> 1;
            let next_pos = pos.wrapping_sub(half).wrapping_add(cnt);
            let mid = half.wrapping_add(pos);
            cnt = half;
            if target >= rd::<u32>(sorted_ptr + (mid as usize) * 4) {
                pos = next_pos;
            }
        }
        wr::<u32>(
            (map_buf + (pos as usize) * 4).wrapping_sub(4),
            move_idx as u32,
        );
        rem_ptr = rem_ptr.wrapping_add(8);
        move_idx = move_idx.wrapping_add(1);
    }

    exe::call_va!(0x1_4248_5680, (job_mgr, p1_hptr));

    let shared_ctx = exe::call_va!(0x1_411f_11e0, (param_2, 0x60u64, 8u32) -> usize);
    wr::<u64>(shared_ctx, rd::<u32>(vec_ptr + 0x10) as u64);
    wr::<usize>(shared_ctx + 0x08, sorted_ptr);
    wr::<u64>(shared_ctx + 0x10, (moves_cnt as u32) as u64);
    wr::<usize>(shared_ctx + 0x18, map_buf);
    wr::<[u64; 2]>(shared_ctx + 0x20, rd::<[u64; 2]>(param_4));
    wr::<[u64; 2]>(shared_ctx + 0x30, rd::<[u64; 2]>(param_7));
    wr::<[u64; 2]>(shared_ctx + 0x40, rd::<[u64; 2]>(param_8));
    wr::<usize>(shared_ctx + 0x50, param_5);
    wr::<usize>(shared_ctx + 0x58, param_9);

    let q_base2 = rd::<usize>(job_mgr + 0x10);
    let mut parent2_handle = [0u64; 2];
    let p2_hptr = parent2_handle.as_mut_ptr() as usize;
    let t_idx2 = exe::call_va!(0x1_4240_7c80, () -> u32);
    let pool2 = rd::<usize>(q_base2 + (t_idx2 as usize) * 0x40 + 0x48).wrapping_add(0x40);
    exe::call_va!(0x1_4248_5aa0, (pool2, p2_hptr));

    let p2_node = rd::<usize>(p2_hptr);
    let _ = crate::simd_ops::atomic_xchg_u32(p2_node as u64, 1);
    wr::<u64>(p2_node + 0x08, 0);
    wr::<usize>(p2_node + 0x10, exe::ptr::<u8>(0x1_4340_e320) as usize);
    wr::<usize>(p2_node + 0x18, exe::ptr::<u8>(0x1_433d_da50) as usize);
    wr::<u64>(p2_node + 0x20, 0x32);
    let (ok_c2, _) = crate::simd_ops::atomic_cmpxchg_u8((p2_node + 0x04) as u64, 1, 3);
    if ok_c2 {
        let p2_reload = rd::<usize>(p2_hptr);
        let _ = crate::simd_ops::atomic_add_u32(p2_reload as u64, 0xffff);
    }
    wr::<u8>(p2_hptr + 0x0a, 1);

    let total_cmds = rd::<u64>(param_6 + 0xc8);
    let mut node_a = rd::<usize>(param_6 + 0xb8);
    let mut idx_a: i64 = 0;
    let mut node_b = rd::<usize>(param_6 + 0xd0);
    let mut idx_b: i64 = 0;
    let mut node_c = rd::<usize>(param_6 + 0xe8);
    let mut idx_c: i64 = 0;

    let step: i64 = ((total_cmds / worker_cnt) as i64) / 3;
    let mut rem_cmds = total_cmds as i64;

    let advance_cursor = |mut node: usize, mut idx: i64, delta: i64| -> (usize, i64) {
        if node != 0 {
            let mut rem = delta;
            let mut avail = rd::<i64>(node + 0x08).wrapping_sub(idx);
            if rem >= avail {
                loop {
                    rem = rem.wrapping_sub(avail);
                    let next = rd::<usize>(node);
                    if next == 0 {
                        return (node, rd::<i64>(node + 0x08));
                    }
                    node = next;
                    idx = 0;
                    avail = rd::<i64>(node + 0x08);
                    if rem < avail {
                        break;
                    }
                }
            }
            idx = idx.wrapping_add(rem);
        }
        (node, idx)
    };

    if step != 0 && rem_cmds > step {
        loop {
            let cur_job_mgr = rd::<usize>(job_glob);
            let cur_q = rd::<usize>(cur_job_mgr + 0x10);
            let t_idx = exe::call_va!(0x1_4240_7c80, () -> u32);
            let cpool = rd::<usize>(cur_q + (t_idx as usize) * 0x40 + 0x48).wrapping_add(0x80);
            let mut child2_handle = [0u64; 2];
            let c2_hptr = child2_handle.as_mut_ptr() as usize;
            exe::call_va!(0x1_4248_5aa0, (cpool, c2_hptr));
            let c2_node = rd::<usize>(c2_hptr);
            let _ = crate::simd_ops::atomic_xchg_u32(c2_node as u64, 1);
            wr::<u64>(c2_node + 0x08, 0);
            wr::<usize>(c2_node + 0x10, exe::ptr::<u8>(0x1_433d_de78) as usize);
            wr::<i64>(c2_node + 0x18, step);
            wr::<usize>(c2_node + 0x20, node_a);
            wr::<i64>(c2_node + 0x28, idx_a);
            wr::<usize>(c2_node + 0x30, node_b);
            wr::<i64>(c2_node + 0x38, idx_b);
            wr::<usize>(c2_node + 0x40, node_c);
            wr::<i64>(c2_node + 0x48, idx_c);
            wr::<usize>(c2_node + 0x50, shared_ctx);

            let p2_n = rd::<usize>(p2_hptr);
            let _ = crate::simd_ops::atomic_add_u32(p2_n as u64, 1);
            wr::<usize>(c2_node + 0x08, p2_n);
            if crate::simd_ops::rd_acq_u32(c2_node) == 1 {
                let q = rd::<usize>(cur_job_mgr + 0x10);
                exe::call_va!(0x1_4248_6460, (q, c2_node));
                let sem = rd::<usize>(rd::<usize>(q) + 0x1008);
                exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize));
            }

            rem_cmds = rem_cmds.wrapping_sub(step);
            (node_a, idx_a) = advance_cursor(node_a, idx_a, step);
            (node_b, idx_b) = advance_cursor(node_b, idx_b, step);
            (node_c, idx_c) = advance_cursor(node_c, idx_c, step);
            if rem_cmds <= step {
                break;
            }
        }
    }

    let p8_data = rd::<usize>(param_8 + 0x08);
    let p7_data = rd::<usize>(param_7 + 0x08);
    let p4_data = rd::<usize>(param_4 + 0x08);

    if rem_cmds != 0 {
        let mut cur_rem = rem_cmds as u64;
        loop {
            let cur_a = node_a;
            let cur_ia = idx_a as usize;
            idx_a = idx_a.wrapping_add(1);
            if idx_a == rd::<i64>(node_a + 0x08) {
                let next_a = rd::<usize>(node_a);
                if next_a != 0 {
                    node_a = next_a;
                    idx_a = 0;
                }
            }

            let val_a = rd::<u32>(cur_a + cur_ia * 4 + 0x10);

            let cur_b = node_b;
            let cur_ib = idx_b as usize;
            idx_b = idx_b.wrapping_add(1);
            if idx_b == rd::<i64>(node_b + 0x08) {
                let next_b = rd::<usize>(node_b);
                if next_b != 0 {
                    node_b = next_b;
                    idx_b = 0;
                }
            }

            let val_b = rd::<u16>(cur_b + cur_ib * 2 + 0x10);

            let cur_c = node_c;
            let cur_ic = idx_c as usize;
            idx_c = idx_c.wrapping_add(1);
            if idx_c == rd::<i64>(node_c + 0x08) {
                let next_c = rd::<usize>(node_c);
                if next_c != 0 {
                    node_c = next_c;
                    idx_c = 0;
                }
            }

            let val_c = rd::<usize>(cur_c + cur_ic * 8 + 0x10);
            if val_c != 0 {
                let s_len = rd::<u32>(vec_ptr + 0x10) as u64;
                let s_ptr = rd::<usize>(vec_ptr + 0x08);
                let found = exe::call_va!(0x1_411f_36a0, (s_ptr, s_len, val_a) -> i32);
                if found != -1 {
                    let mv_idx = rd::<u32>(map_buf + (found as isize as usize) * 4) as usize;
                    let ga_idx = rd::<u16>(p4_data + mv_idx * 8 + 0x04) as usize;
                    let grp_alloc = p8_data + ga_idx * 0x30;
                    let grp_id = rd::<u16>(grp_alloc) as usize;
                    let grp_hdr = param_5 + grp_id * 0x20;
                    let comp_entry = rd::<usize>(param_9) + (val_b as usize) * 0x50;
                    let comp_slot = exe::call_va!(
                        0x1_4248_9ee0,
                        (
                            rd::<u64>(grp_hdr + 0x18058),
                            rd::<u32>(grp_hdr + 0x1806c),
                            val_b
                        ) -> i32
                    );
                    let off_arr = rd::<usize>(grp_alloc + 0x18);
                    let byte_off = rd::<u32>(off_arr + (comp_slot as isize as usize) * 4) as usize;
                    let dst_ptr = rd::<usize>(grp_alloc + 0x08).wrapping_add(byte_off);
                    let elem_idx = rd::<u32>(p7_data + mv_idx * 4);
                    let cb_fn = rd::<usize>(comp_entry + 0x30);
                    exe::call_va!(cb_fn, (val_c, dst_ptr, elem_idx));
                }
            }

            cur_rem -= 1;
            if cur_rem == 0 {
                break;
            }
        }
    }

    wr::<[u64; 2]>(param_1, parent2_handle);
    wr::<u8>(param_1 + 0x10, 1);
    param_1
}

/// `unknown_1420c0190`
/// Original: 0x1420c0190 (600 bytes, Throws::No)
#[allow(clippy::too_many_arguments)]
pub unsafe extern "win64" fn unknown_1420c0190(
    param_1: usize,
    param_2: u32,
    param_3: usize,
    param_4: usize,
    _param_5: u64,
    param_6: usize,
    param_7: usize,
    param_8: usize,
) {
    let grp_id = rd::<u16>(param_4 + 0x48) as usize;
    let grp_hdr = param_3 + grp_id * 0x20;
    let mut src_span: [u64; 2] = [
        rd::<u32>(grp_hdr + 0x18068) as u64,
        rd::<u64>(grp_hdr + 0x18050),
    ];

    let found = exe::call_va!(
        0x1_411f_36a0,
        (rd::<usize>(param_8 + 0x08), rd::<u64>(param_8), param_2) -> i32
    );
    let key_max = ((param_2 as u64) << 32) | 0xffff;

    if found == -1 {
        let p7_cnt = rd::<u64>(param_7);
        let p7_data = rd::<usize>(param_7 + 0x08);
        let mut pos7: u64 = 0;
        let mut rem7 = p7_cnt;
        while rem7 != 0 {
            let half = rem7 >> 1;
            let next_pos = pos7.wrapping_sub(half).wrapping_add(rem7);
            let mid = half.wrapping_add(pos7);
            rem7 = half;
            if key_max >= rd::<u64>(p7_data + (mid as usize) * 8) {
                pos7 = next_pos;
            }
        }

        let mut scan = (pos7 as i64) - 1;
        while scan != -1 {
            if rd::<u32>(p7_data + (scan as usize) * 8 + 0x04) != param_2 {
                break;
            }
            scan -= 1;
        }
        let start_idx = (scan + 1) as u64;
        let mut sub_len = pos7.wrapping_sub(start_idx);
        if sub_len == u64::MAX {
            sub_len = p7_cnt.wrapping_sub(start_idx);
        }
        let sub_ptr = p7_data.wrapping_add((start_idx as usize) * 8);
        let mut rem_span: [u64; 2] = [sub_len, sub_ptr as u64];
        let mut tmp_vec = [0u64; 4];
        let res_ptr = exe::call_va!(
            0x1_420c_0400,
            (
                tmp_vec.as_mut_ptr() as usize,
                param_1,
                src_span.as_mut_ptr() as usize,
                rem_span.as_mut_ptr() as usize
            ) -> usize
        );
        let dst_vec = param_4 + 0x30;
        if res_ptr != dst_vec {
            wr::<u64>(dst_vec, rd::<u64>(res_ptr));
            wr::<u64>(dst_vec + 0x08, rd::<u64>(res_ptr + 0x08));
            wr::<u32>(dst_vec + 0x10, rd::<u32>(res_ptr + 0x10));
            wr::<u32>(dst_vec + 0x14, rd::<u32>(res_ptr + 0x14));
            wr::<u64>(res_ptr + 0x08, 0);
            wr::<u64>(res_ptr + 0x10, 0);
        }
    } else {
        wr::<u32>(param_4 + 0x40, 0);
    }

    let p6_cnt = rd::<u64>(param_6);
    let p6_data = rd::<usize>(param_6 + 0x08);
    let mut pos6: u64 = 0;
    let mut rem6 = p6_cnt;
    while rem6 != 0 {
        let half = rem6 >> 1;
        let next_pos = pos6.wrapping_sub(half).wrapping_add(rem6);
        let mid = pos6.wrapping_add(half);
        rem6 = half;
        if key_max >= rd::<u64>(p6_data + (mid as usize) * 8) {
            pos6 = next_pos;
        }
    }

    let mut scan6 = (pos6 as i64) - 1;
    while scan6 != -1 && rd::<u32>(p6_data + (scan6 as usize) * 8 + 0x04) == param_2 {
        let comp_u16 = rd::<u16>(p6_data + (scan6 as usize) * 8);
        let cur_len = rd::<u32>(param_4 + 0x40);
        let cur_cap = rd::<u32>(param_4 + 0x44);
        if cur_len == cur_cap {
            let need = (cur_len.wrapping_add(1)) as u64;
            let grow = (cur_cap as u64).wrapping_mul(3);
            let new_cap = if grow < need { need } else { grow };
            exe::call_va!(0x1_411f_2990, (param_4 + 0x30, new_cap));
        }
        let idx = rd::<u32>(param_4 + 0x40);
        let buf = rd::<usize>(param_4 + 0x38);
        wr::<u32>(param_4 + 0x40, idx.wrapping_add(1));
        wr::<u16>(buf + (idx as usize) * 2, comp_u16);
        scan6 -= 1;
    }

    let mut beg_ptr = rd::<usize>(param_4 + 0x38);
    let final_len = rd::<u32>(param_4 + 0x40) as u64;
    let mut end_ptr = beg_ptr.wrapping_add((final_len as usize) * 2);
    exe::call_va!(
        0x1_411e_ef20,
        (
            (&mut beg_ptr as *mut usize) as usize,
            (&mut end_ptr as *mut usize) as usize,
            final_len,
            0u32
        )
    );
    exe::call_va!(0x1_411f_16c0, (param_4 + 0x30));
}

/// `unknown_1420c0720`
/// Original: 0x1420c0720 (464 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1420c0720(param_1: u32, param_2: usize, param_3: usize) {
    let p3_cnt = rd::<u64>(param_3);
    let p3_data = rd::<usize>(param_3 + 0x08);
    let key_max = ((param_1 as u64) << 32) | 0xffff;

    let mut pos: u64 = 0;
    let mut rem = p3_cnt;
    while rem != 0 {
        let half = rem >> 1;
        let mid = half.wrapping_add(pos);
        let next_pos = rem.wrapping_sub(half).wrapping_add(pos);
        if key_max >= rd::<u64>(p3_data + (mid as usize) * 8) {
            pos = next_pos;
        }
        rem = half;
    }

    let mut buf_ptr: usize = 0;
    let mut buf_len: u32 = 0;
    let mut buf_cap: u32 = 0;

    let mut scan = (pos as i64) - 1;
    while scan != -1 && rd::<u32>(p3_data + (scan as usize) * 8 + 0x04) == param_1 {
        let comp_u16 = rd::<u16>(p3_data + (scan as usize) * 8);
        if buf_len == buf_cap {
            let old_bytes = (buf_len as usize) * 2;
            let need = (buf_len as u64).wrapping_add(1);
            let cur_cap = buf_cap as u64;
            let grow = (cur_cap >> 1).wrapping_add(cur_cap);
            let new_cap = if grow < need { need } else { grow };
            let new_ptr = exe::call_va!(0x1_4241_c0a0, (new_cap * 2, 2u32) -> usize);
            let _ = exe::call_va!(0x1_4183_acc8, (new_ptr, buf_ptr, old_bytes as u64) -> usize);
            if buf_ptr != 0 {
                exe::call_va!(0x1_4241_c190, (buf_ptr));
            }
            buf_ptr = new_ptr;
            buf_cap = new_cap as u32;
        }
        wr::<u16>(buf_ptr + (buf_len as usize) * 2, comp_u16);
        buf_len = buf_len.wrapping_add(1);
        scan -= 1;
    }

    let mut beg_ptr = buf_ptr;
    let mut end_ptr = buf_ptr.wrapping_add((buf_len as usize) * 2);
    let count = ((end_ptr.wrapping_sub(beg_ptr)) as i64) >> 1;
    exe::call_va!(
        0x1_411e_ef20,
        (
            (&mut beg_ptr as *mut usize) as usize,
            (&mut end_ptr as *mut usize) as usize,
            count,
            0u32
        )
    );
    exe::call_va!(0x1_411f_2920, (param_2 + 0x30, count));

    let dst_ptr = rd::<usize>(param_2 + 0x38);
    let dst_len = rd::<u32>(param_2 + 0x40);
    if dst_len != 0 {
        for i in 0..(dst_len as usize) {
            wr::<u16>(dst_ptr + i * 2, rd::<u16>(buf_ptr + i * 2));
        }
    }
    exe::call_va!(0x1_411f_16c0, (param_2 + 0x30));
    if buf_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (buf_ptr));
    }
}

/// `unknown_1420c0e20`
/// Original: 0x1420c0e20 (1122 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1420c0e20(param_1: usize, param_2: usize) {
    let job_glob = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let job_mgr = rd::<usize>(job_glob);
    let queue = rd::<usize>(job_mgr + 0x10);
    let worker_cnt = rd::<u64>(job_mgr);
    let divisor = worker_cnt.wrapping_mul(6);

    let mut job_count: u32 = 0;

    let spawn_job = |vftbl_va: u64, span_cnt: u64, span_ptr: usize, p2_10: u64, p2_08: u64| {
        let q_base = rd::<usize>(job_mgr + 0x10);
        let t_idx = exe::call_va!(0x1_4240_7c80, () -> u32);
        let pool = rd::<usize>(q_base + (t_idx as usize) * 0x40 + 0x48).wrapping_add(0x80);
        let mut jh = [0u64; 2];
        let jh_ptr = jh.as_mut_ptr() as usize;
        exe::call_va!(0x1_4248_5aa0, (pool, jh_ptr));
        let node = rd::<usize>(jh_ptr);
        let _ = crate::simd_ops::atomic_xchg_u32(node as u64, 1);
        wr::<u64>(node + 0x08, 0);
        wr::<usize>(node + 0x10, exe::ptr::<u8>(vftbl_va) as usize);
        wr::<u64>(node + 0x18, span_cnt);
        wr::<usize>(node + 0x20, span_ptr);
        wr::<u64>(node + 0x28, p2_10);
        wr::<u64>(node + 0x30, p2_08);
        let parent_node = rd::<usize>(param_1);
        let _ = crate::simd_ops::atomic_add_u32(parent_node as u64, 1);
        wr::<usize>(node + 0x08, parent_node);
        exe::call_va!(0x1_4248_6460, (queue, node));
    };

    let cnt1 = rd::<u32>(param_2 + 0x28);
    if cnt1 != 0 {
        let threshold1 = rd::<u64>(param_2 + 0x48) / divisor;
        let mut rem1 = cnt1 as u64;
        let mut acc1: u64 = 0;
        let mut start_idx1: u64 = 0;
        let mut start_off1: usize = 0;
        let mut cur_end1: u64 = 1;
        let mut cur_off1: usize = 0;
        loop {
            let base1 = rd::<usize>(param_2 + 0x20);
            acc1 = acc1.wrapping_add(rd::<u64>(base1 + cur_off1 + 0x20));
            cur_off1 = cur_off1.wrapping_add(0x28);
            if acc1 >= threshold1 {
                let total1 = rd::<u32>(param_2 + 0x28) as u64;
                let diff1 = cur_end1.wrapping_sub(start_idx1);
                let span_cnt = if diff1 != u64::MAX {
                    diff1
                } else {
                    total1.wrapping_sub(start_idx1)
                };
                let span_ptr = base1.wrapping_add(start_off1);
                let p2_10 = rd::<u32>(param_2 + 0x10) as u64;
                let p2_08 = rd::<u64>(param_2 + 0x08);
                spawn_job(0x1_433d_de00, span_cnt, span_ptr, p2_10, p2_08);
                job_count = job_count.wrapping_add(1);
                acc1 = 0;
                start_idx1 = cur_end1;
                start_off1 = cur_off1;
            }
            cur_end1 = cur_end1.wrapping_add(1);
            rem1 -= 1;
            if rem1 == 0 {
                break;
            }
        }
        if acc1 != 0 {
            let span_cnt = (rd::<u32>(param_2 + 0x28) as u64).wrapping_sub(start_idx1);
            let span_ptr = rd::<usize>(param_2 + 0x20).wrapping_add((start_idx1 as usize) * 0x28);
            let p2_10 = rd::<u32>(param_2 + 0x10) as u64;
            let p2_08 = rd::<u64>(param_2 + 0x08);
            spawn_job(0x1_433d_ddb8, span_cnt, span_ptr, p2_10, p2_08);
            job_count = job_count.wrapping_add(1);
        }
    }

    let cnt2 = rd::<u32>(param_2 + 0x40);
    if cnt2 != 0 {
        let threshold2 = rd::<u64>(param_2 + 0x50) / divisor;
        let mut rem2 = cnt2 as u64;
        let mut acc2: u64 = 0;
        let mut start_idx2: u64 = 0;
        let mut start_off2: usize = 0;
        let mut cur_end2: u64 = 1;
        let mut cur_off2: usize = 0;
        loop {
            let base2 = rd::<usize>(param_2 + 0x38);
            acc2 = acc2.wrapping_add(rd::<u64>(base2 + cur_off2 + 0x28));
            cur_off2 = cur_off2.wrapping_add(0x30);
            if acc2 >= threshold2 {
                let total2 = rd::<u32>(param_2 + 0x40) as u64;
                let diff2 = cur_end2.wrapping_sub(start_idx2);
                let span_cnt = if diff2 != u64::MAX {
                    diff2
                } else {
                    total2.wrapping_sub(start_idx2)
                };
                let span_ptr = base2.wrapping_add(start_off2);
                let p2_10 = rd::<u32>(param_2 + 0x10) as u64;
                let p2_08 = rd::<u64>(param_2 + 0x08);
                spawn_job(0x1_433d_dc98, span_cnt, span_ptr, p2_10, p2_08);
                job_count = job_count.wrapping_add(1);
                acc2 = 0;
                start_idx2 = cur_end2;
                start_off2 = cur_off2;
            }
            cur_end2 = cur_end2.wrapping_add(1);
            rem2 -= 1;
            if rem2 == 0 {
                break;
            }
        }
        if acc2 != 0 {
            let span_cnt = (rd::<u32>(param_2 + 0x40) as u64).wrapping_sub(start_idx2);
            let span_ptr = rd::<usize>(param_2 + 0x38).wrapping_add((start_idx2 as usize) * 0x30);
            let p2_10 = rd::<u32>(param_2 + 0x10) as u64;
            let p2_08 = rd::<u64>(param_2 + 0x08);
            spawn_job(0x1_433d_dc20, span_cnt, span_ptr, p2_10, p2_08);
            job_count = job_count.wrapping_add(1);
        }
    }

    if job_count != 0 {
        let sem = rd::<usize>(rd::<usize>(queue) + 0x1008);
        exe::call_iat!(0x1_4264_5580, (sem, job_count, 0usize));
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1420bf7c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
    ) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARENA: [u8; 0x2000] = [0; 0x2000];
    static mut ARENA_OFF: usize = 0;
    static mut JOB_NODES: [[u64; 16]; 16] = [[0; 16]; 16];
    static mut JOB_IDX: usize = 0;
    static mut LOG_HASH: u64 = 0;
    static mut CB_CALLS: Vec<(usize, usize, u32)> = Vec::new();

    unsafe fn bump_alloc(sz: usize, align: usize) -> usize {
        let a = align.max(1);
        let aligned = (ARENA_OFF + a - 1) & !(a - 1);
        let res = (core::ptr::addr_of_mut!(ARENA) as usize) + aligned;
        ARENA_OFF = aligned + sz.max(8);
        res
    }

    unsafe extern "win64" fn mock_2490(vec: usize, new_cap: u64) {
        let old_p = rd::<usize>(vec + 0x08);
        let old_len = rd::<u32>(vec + 0x10) as usize;
        let new_p = bump_alloc((new_cap as usize) * 4, 4);
        for i in 0..old_len {
            wr::<u32>(new_p + i * 4, rd::<u32>(old_p + i * 4));
        }
        wr::<usize>(vec + 0x08, new_p);
        wr::<u32>(vec + 0x14, new_cap as u32);
    }
    unsafe extern "win64" fn mock_a0c0(data: usize, len: u32) {
        let n = len as usize;
        for i in 0..n {
            for j in (i + 1)..n {
                let a = rd::<u32>(data + i * 4);
                let b = rd::<u32>(data + j * 4);
                if b < a {
                    wr::<u32>(data + i * 4, b);
                    wr::<u32>(data + j * 4, a);
                }
            }
        }
    }
    unsafe extern "win64" fn mock_11e0(_arena: usize, sz: u64, al: u32) -> usize {
        bump_alloc(sz as usize, al as usize)
    }
    unsafe extern "win64" fn mock_7c80() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_5aa0(pool: usize, out_h: usize) {
        let idx = JOB_IDX % 16;
        JOB_IDX += 1;
        JOB_NODES[idx] = [0; 16];
        let np = JOB_NODES[idx].as_mut_ptr() as usize;
        wr::<u8>(np + 0x04, 1);
        wr::<usize>(out_h, np);
        wr::<u64>(out_h + 0x08, (pool & 0xff) as u64);
    }
    unsafe extern "win64" fn mock_6460(_q: usize, node: usize) {
        // If this is lambda_1 (0x1433dddd0), populate its chunk of map_buf too
        let vft = rd::<usize>(node + 0x10);
        if vft == exe::ptr::<u8>(0x1_433d_ddd0) as usize {
            let cnt = rd::<u64>(node + 0x18) as usize;
            let moves_p = rd::<usize>(node + 0x20);
            let base_idx = rd::<u64>(node + 0x28) as u32;
            let p5 = rd::<usize>(node + 0x30);
            let s_len = rd::<u64>(node + 0x38) as usize;
            let s_ptr = rd::<usize>(node + 0x40);
            let map_p = rd::<usize>(node + 0x50);
            for i in 0..cnt {
                let mv = rd::<u64>(moves_p + i * 8);
                let mv_lo = mv as u32;
                let mv_hi = (mv >> 48) as usize;
                let grp_arr = rd::<usize>(p5 + 0x48 + mv_hi * 8);
                let target = rd::<u32>(grp_arr + 0x10 + (mv_lo as usize) * 8);
                for k in 0..s_len {
                    if rd::<u32>(s_ptr + k * 4) == target {
                        wr::<u32>(map_p + k * 4, base_idx + (i as u32));
                    }
                }
            }
        }
        LOG_HASH = LOG_HASH
            .wrapping_mul(31)
            .wrapping_add(rd::<u64>(node + 0x18));
    }
    unsafe extern "win64" fn mock_5680(_mgr: usize, h_ptr: usize) {
        let node = rd::<usize>(h_ptr);
        LOG_HASH = LOG_HASH.wrapping_mul(31).wrapping_add(rd::<u32>(node) as u64);
    }
    unsafe extern "win64" fn mock_36a0(data: usize, len: u64, target: u32) -> i32 {
        for i in 0..(len as usize) {
            if rd::<u32>(data + i * 4) == target {
                return i as i32;
            }
        }
        -1
    }
    unsafe extern "win64" fn mock_9ee0(_tbl: u64, _mask: u32, comp: u16) -> i32 {
        (comp & 1) as i32
    }
    unsafe extern "win64" fn mock_comp_cb(val_c: usize, dst_ptr: usize, elem_idx: u32) {
        CB_CALLS.push((val_c, dst_ptr, elem_idx));
    }
    unsafe extern "win64" fn mock_sem(_h: usize, c: u32, _prev: usize) -> i32 {
        LOG_HASH = LOG_HASH.wrapping_mul(31).wrapping_add(c as u64);
        1
    }

    let _m01 = crate::difftest::Mock::new(0x1_411f_2490, mock_2490 as *const ())?;
    let _m02 = crate::difftest::Mock::new(0x1_4103_a0c0, mock_a0c0 as *const ())?;
    let _m03 = crate::difftest::Mock::new(0x1_411f_11e0, mock_11e0 as *const ())?;
    let _m04 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m05 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_5aa0 as *const ())?;
    let _m06 = crate::difftest::Mock::new(0x1_4248_6460, mock_6460 as *const ())?;
    let _m07 = crate::difftest::Mock::new(0x1_4248_5680, mock_5680 as *const ())?;
    let _m08 = crate::difftest::Mock::new(0x1_411f_36a0, mock_36a0 as *const ())?;
    let _m09 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_9ee0 as *const ())?;
    let _i01 = crate::difftest::IatMock::new(0x1_4264_5580, mock_sem as *const ())?;

    let job_glob = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let saved_job_glob = rd::<usize>(job_glob);

    let run = |f: F, cmd_cnt: u64| -> ([u64; 3], u64, Vec<(usize, usize, u32)>) {
        ARENA = [0; 0x2000];
        ARENA_OFF = 0;
        JOB_IDX = 0;
        LOG_HASH = 0;
        CB_CALLS.clear();

        let mut q_inner = vec![0u8; 0x1018];
        wr::<usize>(q_inner.as_mut_ptr() as usize + 0x1008, 0x7777);
        let mut q_obj = [0usize; 12];
        q_obj[0] = q_inner.as_mut_ptr() as usize;
        q_obj[9] = 0x1000;
        let mut mgr = [0usize; 4];
        mgr[0] = 2; // 2 workers
        mgr[2] = q_obj.as_mut_ptr() as usize;
        wr::<usize>(job_glob, mgr.as_mut_ptr() as usize);

        let mut grp_u32s: [u32; 3] = [30, 10, 20];
        let mut grp_elem = [0u64; 10];
        let gp = grp_elem.as_mut_ptr() as usize;
        wr::<usize>(gp + 0x08, grp_u32s.as_mut_ptr() as usize);
        wr::<u32>(gp + 0x10, 3);
        let mut span3: [u64; 2] = [1, gp as u64];

        // 3 moves: hi16 = 0, mid16 = ga_idx (0), lo32 = index into grp_arr (0, 1, 2)
        let mut moves_arr: [u64; 3] = [0, 1, 2];
        let mut span4: [u64; 2] = [3, moves_arr.as_mut_ptr() as usize as u64];

        let mut grp_lookup_entries: [u64; 8] = [0, 0, 10, 20, 30, 0, 0, 0];
        let mut p5 = vec![0u8; 0x18080];
        let p5p = p5.as_mut_ptr() as usize;
        wr::<usize>(p5p + 0x48, grp_lookup_entries.as_mut_ptr() as usize);
        wr::<u64>(p5p + 0x18058, 0xaaaa);
        wr::<u32>(p5p + 0x1806c, 7);

        let mut node_a = [0u64; 12];
        node_a[1] = 8;
        for k in 0..8usize {
            wr::<u32>(
                node_a.as_mut_ptr() as usize + 0x10 + k * 4,
                ((k % 3) as u32 + 1) * 10,
            );
        }

        let mut node_b = [0u64; 12];
        node_b[1] = 8;
        for k in 0..8usize {
            wr::<u16>(node_b.as_mut_ptr() as usize + 0x10 + k * 2, (k & 1) as u16);
        }

        let mut node_c = [0u64; 12];
        node_c[1] = 8;
        for k in 0..8usize {
            node_c[2 + k] = 0x9001 + k as u64;
        }

        let mut p6 = [0u8; 0x100];
        let p6p = p6.as_mut_ptr() as usize;
        wr::<usize>(p6p + 0xb8, node_a.as_mut_ptr() as usize);
        wr::<u64>(p6p + 0xc8, cmd_cnt);
        wr::<usize>(p6p + 0xd0, node_b.as_mut_ptr() as usize);
        wr::<usize>(p6p + 0xe8, node_c.as_mut_ptr() as usize);

        let mut idx_arr: [u32; 3] = [100, 101, 102];
        let mut span7: [u64; 2] = [3, idx_arr.as_mut_ptr() as usize as u64];

        let mut off_arr: [u32; 2] = [0x10, 0x20];
        let mut grp_alloc = [0u64; 6];
        let gap = grp_alloc.as_mut_ptr() as usize;
        wr::<u16>(gap, 0);
        wr::<usize>(gap + 0x08, 0x5000);
        wr::<usize>(gap + 0x18, off_arr.as_mut_ptr() as usize);
        let mut span8: [u64; 2] = [1, gap as u64];

        let mut comp_entries = [0u8; 0xa0];
        let cep = comp_entries.as_mut_ptr() as usize;
        wr::<usize>(cep + 0x30, mock_comp_cb as *const () as usize);
        wr::<usize>(cep + 0x80, mock_comp_cb as *const () as usize);
        let mut p9: usize = cep;

        let mut out_p1 = [0u64; 3];
        let ret = f(
            out_p1.as_mut_ptr() as usize,
            0xfeed,
            span3.as_mut_ptr() as usize,
            span4.as_mut_ptr() as usize,
            p5p,
            p6p,
            span7.as_mut_ptr() as usize,
            span8.as_mut_ptr() as usize,
            (&mut p9 as *mut usize) as usize,
        );
        wr::<usize>(job_glob, saved_job_glob);
        assert_eq!(ret, out_p1.as_mut_ptr() as usize);

        let nodes_base = core::ptr::addr_of!(JOB_NODES) as usize as u64;
        if out_p1[0] >= nodes_base {
            out_p1[0] -= nodes_base;
        }
        (out_p1, LOG_HASH, CB_CALLS.clone())
    };

    for cnt in [0u64, 2, 7] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "1420bf7c0 cnt={}", cnt);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1420c0190(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32, usize, usize, u64, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: (u64, u64, u64, u32) = (0, 0, 0, 0);
    static mut U16_STORAGE: [u16; 8] = [0; 8];

    unsafe extern "win64" fn mock_36a0(data: usize, len: u64, target: u32) -> i32 {
        for i in 0..(len as usize) {
            if rd::<u32>(data + i * 4) == target {
                return i as i32;
            }
        }
        -1
    }
    unsafe extern "win64" fn mock_0400(
        out_vec: usize,
        arena: usize,
        src_span: usize,
        rem_span: usize,
    ) -> usize {
        CALL_LOG.0 = rd::<u64>(src_span) ^ rd::<u64>(src_span + 0x08);
        CALL_LOG.1 = rd::<u64>(rem_span);
        wr::<usize>(out_vec, arena);
        wr::<usize>(out_vec + 0x08, core::ptr::addr_of_mut!(U16_STORAGE) as usize);
        wr::<u32>(out_vec + 0x10, 0);
        wr::<u32>(out_vec + 0x14, 0);
        out_vec
    }
    unsafe extern "win64" fn mock_2990(vec: usize, new_cap: u64) {
        wr::<usize>(vec + 0x08, core::ptr::addr_of_mut!(U16_STORAGE) as usize);
        wr::<u32>(vec + 0x14, new_cap as u32);
    }
    unsafe extern "win64" fn mock_ef20(beg_p: usize, end_p: usize, len: u64, flag: u32) {
        let beg = rd::<usize>(beg_p);
        let end = rd::<usize>(end_p);
        CALL_LOG.2 = (end.wrapping_sub(beg) as u64) ^ (len << 16) ^ (flag as u64);
    }
    unsafe extern "win64" fn mock_16c0(vec: usize) {
        CALL_LOG.3 = rd::<u32>(vec + 0x10);
    }

    let _m1 = crate::difftest::Mock::new(0x1_411f_36a0, mock_36a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_420c_0400, mock_0400 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411f_2990, mock_2990 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_411e_ef20, mock_ef20 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_411f_16c0, mock_16c0 as *const ())?;

    let run = |f: F, target_ent: u32| -> ([u16; 8], u32, u32, (u64, u64, u64, u32)) {
        CALL_LOG = (0, 0, 0, 0);
        U16_STORAGE = [0; 8];

        let mut p3 = vec![0u8; 0x18080];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<u64>(p3p + 0x20 + 0x18050, 0x1234_5678);
        wr::<u32>(p3p + 0x20 + 0x18068, 5);

        let mut p4 = [0u8; 0x50];
        let p4p = p4.as_mut_ptr() as usize;
        wr::<usize>(p4p + 0x30, 0xfeed);
        wr::<usize>(p4p + 0x38, core::ptr::addr_of_mut!(U16_STORAGE) as usize);
        wr::<u32>(p4p + 0x40, 2);
        wr::<u32>(p4p + 0x44, 8);
        wr::<u16>(p4p + 0x48, 1);

        let mut p6_arr: [u64; 4] = [
            (1u64 << 32) | 10,
            (2u64 << 32) | 20,
            (2u64 << 32) | 30,
            (3u64 << 32) | 40,
        ];
        let mut p6: [u64; 2] = [4, p6_arr.as_mut_ptr() as usize as u64];

        let mut p7_arr: [u64; 3] = [(1u64 << 32) | 5, (2u64 << 32) | 6, (4u64 << 32) | 7];
        let mut p7: [u64; 2] = [3, p7_arr.as_mut_ptr() as usize as u64];

        let mut p8_arr: [u32; 2] = [1, 9];
        let mut p8: [u64; 2] = [2, p8_arr.as_mut_ptr() as usize as u64];

        f(
            0xfeed,
            target_ent,
            p3p,
            p4p,
            8,
            p6.as_mut_ptr() as usize,
            p7.as_mut_ptr() as usize,
            p8.as_mut_ptr() as usize,
        );

        (
            U16_STORAGE,
            rd::<u32>(p4p + 0x40),
            rd::<u32>(p4p + 0x44),
            CALL_LOG,
        )
    };

    for ent in [1u32, 2] {
        let o = run(f_o, ent);
        let n = run(f_n, ent);
        crate::diff_eq!(o, n, "1420c0190 ent={}", ent);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1420c0720(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TEMP_HEAP: [u16; 32] = [0; 32];
    static mut HEAP_OFF: usize = 0;
    static mut CALL_INFO: (u32, u32, u64) = (0, 0, 0);

    unsafe extern "win64" fn mock_c0a0(bytes: u64, _align: u32) -> usize {
        CALL_INFO.0 += 1;
        let ptr = (core::ptr::addr_of_mut!(TEMP_HEAP) as usize) + HEAP_OFF;
        HEAP_OFF += (bytes as usize).max(4);
        ptr
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {
        CALL_INFO.1 += 1;
    }
    unsafe extern "win64" fn mock_ef20(_beg: usize, _end: usize, cnt: i64, _flag: u32) {
        CALL_INFO.2 ^= (cnt as u64) << 8;
    }
    unsafe extern "win64" fn mock_2920(vec: usize, cnt: i64) {
        wr::<u32>(vec + 0x10, cnt as u32);
    }
    unsafe extern "win64" fn mock_16c0(vec: usize) {
        CALL_INFO.2 ^= rd::<u32>(vec + 0x10) as u64;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411e_ef20, mock_ef20 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_411f_2920, mock_2920 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_411f_16c0, mock_16c0 as *const ())?;

    let run = |f: F, target: u32| -> ([u16; 8], u32, (u32, u32, u64)) {
        TEMP_HEAP = [0; 32];
        HEAP_OFF = 0;
        CALL_INFO = (0, 0, 0);

        let mut dst_u16 = [0u16; 8];
        let mut p2 = [0u8; 0x50];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p + 0x38, dst_u16.as_mut_ptr() as usize);
        wr::<u32>(p2p + 0x40, 0);
        wr::<u32>(p2p + 0x44, 8);

        let mut p3_arr: [u64; 4] = [
            (1u64 << 32) | 11,
            (2u64 << 32) | 22,
            (2u64 << 32) | 33,
            (3u64 << 32) | 44,
        ];
        let mut p3: [u64; 2] = [4, p3_arr.as_mut_ptr() as usize as u64];

        f(target, p2p, p3.as_mut_ptr() as usize);
        (dst_u16, rd::<u32>(p2p + 0x40), CALL_INFO)
    };

    for t in [2u32, 9] {
        let o = run(f_o, t);
        let n = run(f_n, t);
        crate::diff_eq!(o, n, "1420c0720 t={}", t);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1420c0e20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NODES: [[u64; 8]; 8] = [[0; 8]; 8];
    static mut NODE_IDX: usize = 0;
    static mut ENQUEUED: Vec<(usize, u64, usize, u64, u64)> = Vec::new();
    static mut SEM_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_7c80() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_5aa0(_pool: usize, out_h: usize) {
        let idx = NODE_IDX % 8;
        NODE_IDX += 1;
        NODES[idx] = [0; 8];
        wr::<usize>(out_h, NODES[idx].as_mut_ptr() as usize);
        wr::<u64>(out_h + 0x08, 0);
    }
    unsafe extern "win64" fn mock_6460(_q: usize, node: usize) {
        ENQUEUED.push((
            rd::<usize>(node + 0x10),
            rd::<u64>(node + 0x18),
            rd::<usize>(node + 0x20),
            rd::<u64>(node + 0x28),
            rd::<u64>(node + 0x30),
        ));
    }
    unsafe extern "win64" fn mock_sem(_h: usize, c: u32, _prev: usize) -> i32 {
        SEM_COUNT = SEM_COUNT.wrapping_add(c);
        1
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_5aa0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4248_6460, mock_6460 as *const ())?;
    let _i1 = crate::difftest::IatMock::new(0x1_4264_5580, mock_sem as *const ())?;

    let job_glob = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let saved_job_glob = rd::<usize>(job_glob);

    let run = |f: F, active: bool| -> (u32, u32, Vec<(usize, u64, usize, u64, u64)>) {
        NODE_IDX = 0;
        ENQUEUED.clear();
        SEM_COUNT = 0;

        let mut q_inner = vec![0u8; 0x1018];
        wr::<usize>(q_inner.as_mut_ptr() as usize + 0x1008, 0x8888);
        let mut q_obj = [0usize; 12];
        q_obj[0] = q_inner.as_mut_ptr() as usize;
        q_obj[9] = 0x2000;
        let mut mgr = [0usize; 4];
        mgr[0] = 1; // 1 worker -> divisor = 6
        mgr[2] = q_obj.as_mut_ptr() as usize;
        wr::<usize>(job_glob, mgr.as_mut_ptr() as usize);

        let mut parent_counter: u32 = 1;
        let mut p1: usize = (&mut parent_counter as *mut u32) as usize;

        let mut arr1 = [0u8; 2 * 0x28];
        let a1p = arr1.as_mut_ptr() as usize;
        wr::<u64>(a1p + 0x20, 10);
        wr::<u64>(a1p + 0x28 + 0x20, 5);

        let mut arr2 = [0u8; 2 * 0x30];
        let a2p = arr2.as_mut_ptr() as usize;
        wr::<u64>(a2p + 0x28, 12);
        wr::<u64>(a2p + 0x30 + 0x28, 4);

        let mut p2 = [0u8; 0x60];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u64>(p2p + 0x08, 0xabcd_1234);
        wr::<u32>(p2p + 0x10, 7);
        wr::<usize>(p2p + 0x20, a1p);
        wr::<u32>(p2p + 0x28, if active { 2 } else { 0 });
        wr::<usize>(p2p + 0x38, a2p);
        wr::<u32>(p2p + 0x40, if active { 2 } else { 0 });
        wr::<u64>(p2p + 0x48, 60); // threshold1 = 60 / 6 = 10
        wr::<u64>(p2p + 0x50, 60); // threshold2 = 60 / 6 = 10

        f((&mut p1 as *mut usize) as usize, p2p);
        wr::<usize>(job_glob, saved_job_glob);

        let norm_enq = ENQUEUED
            .iter()
            .map(|&(vft, cnt, ptr, a, b)| {
                let off = if ptr >= a2p && ptr < a2p + 0x60 {
                    ptr - a2p + 0x1000
                } else if ptr >= a1p && ptr < a1p + 0x50 {
                    ptr - a1p
                } else {
                    ptr
                };
                (vft, cnt, off, a, b)
            })
            .collect();
        (parent_counter, SEM_COUNT, norm_enq)
    };

    for active in [true, false] {
        let o = run(f_o, active);
        let n = run(f_n, active);
        crate::diff_eq!(o, n, "1420c0e20 active={}", active);
    }
    Ok(2)
}
