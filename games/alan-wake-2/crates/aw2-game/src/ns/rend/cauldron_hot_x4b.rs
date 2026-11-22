//! Decompiled hot functions shard X4b (`0x1420b2890..0x1420c3bb0`).

use crate::exe::{self, rd, wr};

/// `r_1420b2890`
/// Original: 0x1420b2890 (491 bytes, Throws::No)
pub unsafe extern "win64" fn r_1420b2890(param_1: usize, param_2: usize, param_3: usize) {
    let mut cur_span = rd::<usize>(param_1 + 0x08);
    let end_span = cur_span.wrapping_add(rd::<usize>(param_1).wrapping_shl(4));
    while cur_span != end_span {
        let mut cur_u32 = rd::<usize>(cur_span);
        let len_u32 = rd::<u32>(cur_span + 0x08) as usize;
        let end_u32 = cur_u32.wrapping_add(len_u32 * 4);
        while cur_u32 != end_u32 {
            let val = rd::<u32>(cur_u32);
            let lo16 = val as u16;
            let grp = rd::<usize>(param_2 + 0x08).wrapping_add(((val >> 16) as usize) << 5);

            let mut cur_w = rd::<usize>(grp + 0x08);
            let end_w = cur_w.wrapping_add(rd::<usize>(grp) * 2);
            while cur_w != end_w {
                let idx = rd::<u16>(cur_w) as usize;
                let dst_vec = rd::<usize>(param_3).wrapping_add(idx * 0x30);
                let len = rd::<u32>(dst_vec + 0x08) as u64;
                let cap = rd::<u32>(dst_vec + 0x0c) as u64;
                if len * 2 == cap * 2 {
                    exe::call_va!(0x1_40fa_5d10, (dst_vec, len.wrapping_add(1)));
                }
                let pos = rd::<u32>(dst_vec + 0x08) as usize;
                let buf = rd::<usize>(dst_vec);
                wr::<u16>(buf.wrapping_add(pos * 2), lo16);
                wr::<u32>(dst_vec + 0x08, (pos as u32).wrapping_add(1));
                cur_w = cur_w.wrapping_add(2);
            }

            let mut cur_q = rd::<usize>(grp + 0x18);
            let end_q = cur_q.wrapping_add(rd::<usize>(grp + 0x10) * 8);
            while cur_q != end_q {
                let qval = rd::<u64>(cur_q);
                let hi32 = (qval >> 32) as u32;
                let idx = (qval as u16) as usize;
                let entry = rd::<usize>(param_3).wrapping_add(idx * 0x30);

                let vec_10 = entry.wrapping_add(0x10);
                let len_10 = rd::<u32>(vec_10 + 0x08) as u64;
                let cap_10 = rd::<u32>(vec_10 + 0x0c) as u64;
                if len_10 * 2 == cap_10 * 2 {
                    exe::call_va!(0x1_40fa_5d10, (vec_10, len_10.wrapping_add(1)));
                }
                let pos_10 = rd::<u32>(vec_10 + 0x08) as usize;
                let buf_10 = rd::<usize>(vec_10);
                wr::<u16>(buf_10.wrapping_add(pos_10 * 2), lo16);
                wr::<u32>(vec_10 + 0x08, (pos_10 as u32).wrapping_add(1));

                let vec_20 = entry.wrapping_add(0x20);
                let len_20 = rd::<u32>(entry + 0x28) as u64;
                let cap_20 = rd::<u32>(entry + 0x2c) as u64;
                if len_20 * 4 == cap_20 * 4 {
                    exe::call_va!(0x1_411c_02b0, (vec_20, len_20.wrapping_add(1)));
                }
                let pos_20 = rd::<u32>(entry + 0x28) as usize;
                let buf_20 = rd::<usize>(entry + 0x20);
                wr::<u32>(buf_20.wrapping_add(pos_20 * 4), hi32);
                wr::<u32>(entry + 0x28, (pos_20 as u32).wrapping_add(1));

                cur_q = cur_q.wrapping_add(8);
            }

            cur_u32 = cur_u32.wrapping_add(4);
        }
        cur_span = cur_span.wrapping_add(0x10);
    }
}

/// `r_1420b2a90`
/// Original: 0x1420b2a90 (177 bytes, Throws::No)
pub unsafe extern "win64" fn r_1420b2a90(param_1: usize, param_2: usize) {
    let count = rd::<usize>(param_1);
    let mut cur = rd::<usize>(param_1 + 0x08);
    let end = cur.wrapping_add(count * 4);
    let mut val_slot: u16;
    let mut out_iter: [u64; 2] = [0; 2];

    while cur != end {
        let raw = rd::<u32>(cur);
        let lo16 = raw as u16;
        val_slot = lo16;
        let vec_ptr = rd::<usize>(param_2).wrapping_add(((raw >> 16) as usize) * 0x10);
        let data_ptr = rd::<usize>(vec_ptr);
        let mut rem = rd::<u32>(vec_ptr + 0x08) as usize;
        let mut pos: usize = 0;

        while rem != 0 {
            let half = rem >> 1;
            let next_pos = pos.wrapping_sub(half).wrapping_add(rem);
            let mid = pos.wrapping_add(half);
            if lo16 >= rd::<u16>(data_ptr.wrapping_add(mid * 2)) {
                pos = next_pos;
            }
            rem = half;
        }

        let insert_at = data_ptr.wrapping_add(pos * 2);
        let _ = exe::call_va!(
            0x1_411f_00d0,
            (
                vec_ptr,
                out_iter.as_mut_ptr() as usize,
                insert_at,
                (&mut val_slot as *mut u16) as usize
            ) -> usize
        );
        cur = cur.wrapping_add(4);
    }
}

/// `r_1420b3cb0`
/// Original: 0x1420b3cb0 (1710 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_1420b3cb0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: u8,
) {
    let total_groups = rd::<u64>(param_1 + 0x08);
    let mut buf_ptr: usize = 0;
    let mut buf_len: u32 = 0;
    let mut buf_cap: u32 = 0;

    if total_groups != 0 {
        for idx in 0..(total_groups as usize) {
            if rd::<u32>(param_1.wrapping_add(0x10048).wrapping_add(idx * 4)) == 0
                && rd::<u64>(param_1.wrapping_add(0x48).wrapping_add(idx * 8)) != 0
            {
                if buf_len == buf_cap {
                    let old_bytes = (buf_len as u64).wrapping_mul(2);
                    let cur_len = buf_len as u64;
                    let cur_cap = buf_cap as u64;
                    let grown = (cur_cap >> 1).wrapping_add(cur_cap);
                    let new_cap = if grown < cur_len.wrapping_add(1) {
                        cur_len.wrapping_add(1)
                    } else {
                        grown
                    };
                    let new_ptr =
                        exe::call_va!(0x1_4241_c0a0, (new_cap.wrapping_mul(2), 2u32) -> usize);
                    let _ = exe::call_va!(0x1_4183_acc8, (new_ptr, buf_ptr, old_bytes) -> usize);
                    if buf_ptr != 0 {
                        exe::call_va!(0x1_4241_c190, (buf_ptr));
                    }
                    buf_ptr = new_ptr;
                    buf_cap = new_cap as u32;
                }
                wr::<u16>(buf_ptr.wrapping_add((buf_len as usize) * 2), idx as u16);
                buf_len = buf_len.wrapping_add(1);
                if param_4 == 0 && buf_len == 0x10 {
                    break;
                }
            }
        }
    }

    let end_ptr = buf_ptr.wrapping_add((buf_len as usize) * 2);
    if buf_ptr != end_ptr {
        let g_job_sys = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
        let num_workers = rd::<u32>(g_job_sys);
        let s_parent = exe::ptr::<u8>(0x1_433d_d860) as usize;
        let worker_arr = rd::<usize>(g_job_sys + 0x10);
        let tid_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
        let pool_40 = rd::<usize>(worker_arr.wrapping_add(tid_idx * 0x40).wrapping_add(0x48))
            .wrapping_add(0x40);

        let mut parent_hdl = [0u8; 16];
        let ph = parent_hdl.as_mut_ptr() as usize;
        let _ = exe::call_va!(0x1_4248_5aa0, (pool_40, ph) -> usize);

        let pnode = rd::<usize>(ph);
        let _ = crate::simd_ops::atomic_xchg_u32(pnode as u64, 1);
        let pnode = rd::<usize>(ph);
        wr::<u64>(pnode + 0x08, 0);
        wr::<usize>(pnode + 0x10, exe::ptr::<u8>(0x1_4340_e320) as usize);
        wr::<usize>(pnode + 0x18, s_parent);
        wr::<u64>(pnode + 0x20, 0x27);
        let (ok, _) = crate::simd_ops::atomic_cmpxchg_u8((pnode + 0x04) as u64, 1, 3);
        if ok {
            let pnode2 = rd::<usize>(ph);
            let _ = crate::simd_ops::atomic_add_u32(pnode2 as u64, 0xffff);
        }
        wr::<u8>(ph + 0x0a, 1);

        let r10 = rd::<u32>(rd::<usize>(param_3 + 0x08) + 0x10) as u64;
        let div_q = (r10.wrapping_add(0x3f) >> 6) / (num_workers as u64);
        let step_words: u64 = if div_q != 0 { div_q } else { 1 };
        let chunk_bits = step_words << 6;

        let mut cur = buf_ptr;
        while cur != end_ptr {
            let bit_pos = rd::<u64>(param_3 + 0xa0).wrapping_sub(1);
            let arch = rd::<u16>(cur) as u64;
            let row_off = arch.wrapping_mul(rd::<u64>(param_3 + 0xb0));
            let row_ptr = rd::<usize>(param_3 + 0x70).wrapping_add((row_off as usize) * 8);
            let word_idx = (bit_pos >> 6) as usize;
            let bit_idx = (bit_pos as u32) & 0x3f;
            let word_addr = row_ptr.wrapping_add(word_idx * 8);
            wr::<u64>(word_addr, rd::<u64>(word_addr) & !(1u64 << bit_idx));
            cur = cur.wrapping_add(2);
        }

        let mut rem_words = r10 >> 6;
        let mut word_off: u64 = 0;
        let mut child_hdl = [0u8; 16];
        let ch = child_hdl.as_mut_ptr() as usize;

        while rem_words > step_words {
            let count_u16 = buf_len as u64;
            let warr = rd::<usize>(g_job_sys + 0x10);
            let tid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
            let pool_80 =
                rd::<usize>(warr.wrapping_add(tid * 0x40).wrapping_add(0x48)).wrapping_add(0x80);
            let _ = exe::call_va!(0x1_4248_5aa0, (pool_80, ch) -> usize);

            let jnode = rd::<usize>(ch);
            let _ = crate::simd_ops::atomic_xchg_u32(jnode as u64, 1);
            let jnode = rd::<usize>(ch);
            wr::<u64>(jnode + 0x08, 0);
            wr::<usize>(jnode + 0x10, exe::ptr::<u8>(0x1_433d_d890) as usize);
            wr::<usize>(jnode + 0x18, param_3);
            wr::<u64>(jnode + 0x20, word_off);
            wr::<u64>(jnode + 0x28, chunk_bits);
            wr::<u64>(jnode + 0x30, count_u16);
            wr::<usize>(jnode + 0x38, buf_ptr);

            let pnode_cur = rd::<usize>(ph);
            let _ = crate::simd_ops::atomic_add_u32(pnode_cur as u64, 1);
            let jnode = rd::<usize>(ch);
            wr::<usize>(jnode + 0x08, rd::<usize>(ph));
            if crate::simd_ops::rd_acq_u32(jnode) == 1 {
                let warr2 = rd::<usize>(g_job_sys + 0x10);
                exe::call_va!(0x1_4248_6460, (warr2, rd::<usize>(ch)));
                let sem = rd::<u64>(rd::<usize>(warr2) + 0x1008);
                let _ = exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize) -> i32);
            }
            rem_words = rem_words.wrapping_sub(step_words);
            word_off = word_off.wrapping_add(step_words);
        }

        let mut span_arg: [u64; 2] = [buf_len as u64, buf_ptr as u64];
        let tail_bits = (r10 & 0x3f).wrapping_add(rem_words << 6);
        exe::call_va!(
            0x1_420b_33f0,
            (
                param_3,
                span_arg.as_mut_ptr() as usize,
                word_off,
                tail_bits
            )
        );

        let end_ptr = buf_ptr.wrapping_add((buf_len as usize) * 2);
        let mut cur = buf_ptr;
        while cur != end_ptr {
            let arch = rd::<u16>(cur) as usize;
            let slot = param_1.wrapping_add(arch * 0x20).wrapping_add(0x18050);
            exe::call_va!(
                0x1_4248_c9d0,
                (param_1.wrapping_add(0x58050), rd::<u64>(slot))
            );
            wr::<[u64; 4]>(slot, [0; 4]);
            cur = cur.wrapping_add(2);
        }

        let total_p2 = rd::<u32>(rd::<usize>(param_2) + 0x38);
        let quot = total_p2 / num_workers;
        let rem_p2 = total_p2 % num_workers;
        if num_workers != 0 {
            let count_u16 = buf_len as u64;
            for worker_i in 0..num_workers {
                let start = worker_i.wrapping_mul(quot);
                let end = start.wrapping_add(quot);
                let warr = rd::<usize>(g_job_sys + 0x10);
                let tid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
                let pool_80 =
                    rd::<usize>(warr.wrapping_add(tid * 0x40).wrapping_add(0x48)).wrapping_add(0x80);
                let _ = exe::call_va!(0x1_4248_5aa0, (pool_80, ch) -> usize);

                let jnode = rd::<usize>(ch);
                let _ = crate::simd_ops::atomic_xchg_u32(jnode as u64, 1);
                let jnode = rd::<usize>(ch);
                wr::<u64>(jnode + 0x08, 0);
                wr::<usize>(jnode + 0x10, exe::ptr::<u8>(0x1_433d_d8a8) as usize);
                wr::<usize>(jnode + 0x18, param_2);
                wr::<u64>(jnode + 0x20, count_u16);
                wr::<usize>(jnode + 0x28, buf_ptr);
                wr::<u32>(jnode + 0x30, start);
                wr::<u32>(jnode + 0x34, end);

                let pnode_cur = rd::<usize>(ph);
                let _ = crate::simd_ops::atomic_add_u32(pnode_cur as u64, 1);
                let jnode = rd::<usize>(ch);
                wr::<usize>(jnode + 0x08, rd::<usize>(ph));
                if crate::simd_ops::rd_acq_u32(jnode) == 1 {
                    let warr2 = rd::<usize>(g_job_sys + 0x10);
                    exe::call_va!(0x1_4248_6460, (warr2, rd::<usize>(ch)));
                    let sem = rd::<u64>(rd::<usize>(warr2) + 0x1008);
                    let _ = exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize) -> i32);
                }
            }
        }

        let tail_start = quot.wrapping_mul(num_workers);
        let tail_end = rem_p2.wrapping_add(tail_start);
        let range_packed = ((tail_end as u64) << 32) | (tail_start as u64);
        let mut span_arg2: [u64; 2] = [buf_len as u64, buf_ptr as u64];
        exe::call_va!(
            0x1_411f_03b0,
            (
                param_2.wrapping_add(0x08),
                span_arg2.as_mut_ptr() as usize,
                range_packed
            )
        );

        let end_ptr = buf_ptr.wrapping_add((buf_len as usize) * 2);
        let mut cur = buf_ptr;
        while cur != end_ptr {
            let arch = rd::<u16>(cur) as usize;
            let prev_head = rd::<u32>(param_1 + 0x18048);
            wr::<u32>(
                param_1.wrapping_add(0x10048).wrapping_add(arch * 4),
                prev_head,
            );
            wr::<u32>(param_1 + 0x18048, arch as u32);

            let off8 = arch * 8;
            let block_ptr = rd::<usize>(param_1.wrapping_add(0x48).wrapping_add(off8));
            if block_ptr != 0 {
                let r10_base = block_ptr.wrapping_sub(1) & !0x01ff_ffffusize;
                let page_idx = block_ptr.wrapping_sub(r10_base) >> 16;
                let hdr_raw = r10_base
                    .wrapping_add(0x108)
                    .wrapping_add(page_idx.wrapping_mul(0x50));
                let hdr = hdr_raw.wrapping_sub(rd::<u32>(hdr_raw + 0x04) as usize);
                if exe::teb() == rd::<usize>(r10_base + 0x100) {
                    if rd::<u8>(hdr + 0x0e) == 0 {
                        wr::<u64>(block_ptr, rd::<u64>(hdr + 0x20));
                        wr::<usize>(hdr + 0x20, block_ptr);
                        let new_cnt = rd::<u32>(hdr + 0x18).wrapping_sub(1);
                        wr::<u32>(hdr + 0x18, new_cnt);
                        if new_cnt == 0 {
                            exe::call_va!(0x1_4253_71a0, (hdr));
                        }
                    } else {
                        exe::call_va!(0x1_4253_3200, (r10_base, hdr, 1u8, block_ptr));
                    }
                } else {
                    exe::call_va!(0x1_4253_3200, (r10_base, hdr, 0u8, block_ptr));
                }
            }
            wr::<u64>(param_1.wrapping_add(0x48).wrapping_add(off8), 0);

            let want_key = rd::<u64>(rd::<usize>(param_1 + 0x38).wrapping_add(off8));
            let mut rem = rd::<u32>(param_1 + 0x20) as usize;
            let keys_base = rd::<usize>(param_1 + 0x18);
            let keys_end = keys_base.wrapping_add(rem * 8);
            let mut pos: usize = 0;
            while rem != 0 {
                let half = rem >> 1;
                let mid = pos.wrapping_add(half);
                let next_pos = rem.wrapping_sub(half).wrapping_add(pos);
                if want_key >= rd::<u64>(keys_base.wrapping_add(mid * 8)) {
                    pos = next_pos;
                }
                rem = half;
            }
            let src_k = keys_base.wrapping_add(pos * 8);
            let _ = exe::call_va!(
                0x1_4183_acc8,
                (
                    src_k.wrapping_sub(8),
                    src_k,
                    keys_end.wrapping_sub(src_k) as u64
                ) -> usize
            );
            wr::<u32>(param_1 + 0x20, rd::<u32>(param_1 + 0x20).wrapping_sub(1));

            let ids_base = rd::<usize>(param_1 + 0x28);
            let src_id = ids_base.wrapping_add(pos * 2);
            let ids_end = ids_base.wrapping_add((rd::<u32>(param_1 + 0x30) as usize) * 2);
            let _ = exe::call_va!(
                0x1_4183_acc8,
                (
                    src_id.wrapping_sub(2),
                    src_id,
                    ids_end.wrapping_sub(src_id) as u64
                ) -> usize
            );
            wr::<u32>(param_1 + 0x30, rd::<u32>(param_1 + 0x30).wrapping_sub(1));
            wr::<u64>(rd::<usize>(param_1 + 0x38).wrapping_add(off8), 0);

            cur = cur.wrapping_add(2);
        }

        exe::call_va!(0x1_4248_5680, (g_job_sys, ph));
    }

    if buf_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (buf_ptr));
    }
}

/// `unknown_1420bba90`
/// Original: 0x1420bba90 (668 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420bba90(param_1: usize) {
    let ctx = rd::<usize>(param_1);
    let mut rem = rd::<u64>(param_1 + 0x08);
    if rem == 0 {
        return;
    }

    let f08 = rd::<usize>(ctx + 0x08);
    let f10 = rd::<u64>(ctx + 0x10);
    let f18 = rd::<usize>(ctx + 0x18);
    let f28 = rd::<usize>(ctx + 0x28);
    let f38 = rd::<usize>(ctx + 0x38);
    let f48 = rd::<usize>(ctx + 0x48);
    let f50 = rd::<usize>(ctx + 0x50);
    let f58 = rd::<usize>(ctx + 0x58);

    let mut node_a = rd::<usize>(param_1 + 0x10);
    let mut idx_a = rd::<u64>(param_1 + 0x18);
    let mut node_b = rd::<usize>(param_1 + 0x20);
    let mut idx_b = rd::<u64>(param_1 + 0x28);
    let mut node_c = rd::<usize>(param_1 + 0x30);
    let mut idx_c = rd::<u64>(param_1 + 0x38);

    while rem != 0 {
        let old_node_b = node_b;
        let old_idx_b = idx_b;
        idx_b = idx_b.wrapping_add(1);
        if idx_b == rd::<u64>(node_b + 0x08) {
            let next_b = rd::<usize>(node_b);
            if next_b != 0 {
                node_b = next_b;
                idx_b = 0;
            }
        }
        let u16_b = rd::<u16>(old_node_b.wrapping_add((old_idx_b as usize) * 2).wrapping_add(0x10))
            as usize;

        let old_node_c = node_c;
        let old_idx_c = idx_c;
        idx_c = idx_c.wrapping_add(1);
        if idx_c == rd::<u64>(node_c + 0x08) {
            let next_c = rd::<usize>(node_c);
            if next_c != 0 {
                node_c = next_c;
                idx_c = 0;
            }
        }
        let val_c = rd::<u64>(old_node_c.wrapping_add((old_idx_c as usize) * 8).wrapping_add(0x10));

        let old_node_a = node_a;
        let old_idx_a = idx_a;
        idx_a = idx_a.wrapping_add(1);
        if idx_a == rd::<u64>(node_a + 0x08) {
            let next_a = rd::<usize>(node_a);
            if next_a != 0 {
                node_a = next_a;
                idx_a = 0;
            }
        }

        if val_c != 0 {
            let want_u32 =
                rd::<u32>(old_node_a.wrapping_add((old_idx_a as usize) * 4).wrapping_add(0x10));
            let mut pos: usize = 0;
            let mut count = f10 as usize;
            while count != 0 {
                let half = count >> 1;
                let next_pos = pos.wrapping_sub(half).wrapping_add(count);
                let mid = pos.wrapping_add(half);
                if want_u32 >= rd::<u32>(f18.wrapping_add(mid * 4)) {
                    pos = next_pos;
                }
                count = half;
            }

            let idx_u32 = rd::<u32>(f38.wrapping_add(pos * 4).wrapping_sub(4));
            if idx_u32 != u32::MAX {
                let arch = rd::<u16>(f28.wrapping_add((idx_u32 as usize) * 8).wrapping_add(4));
                if arch != 0xffff {
                    let arch_usize = arch as usize;
                    let r11 = f08.wrapping_add(arch_usize * 0x30);
                    let search_len =
                        rd::<u32>(f50.wrapping_add(arch_usize * 0x20).wrapping_add(0x1806c));
                    let search_ptr =
                        rd::<u64>(f50.wrapping_add(arch_usize * 0x20).wrapping_add(0x18058));
                    let found = exe::call_va!(
                        0x1_4248_9ee0,
                        (search_ptr, search_len, u16_b as u32) -> i32
                    );
                    let fn_addr =
                        rd::<usize>(rd::<usize>(f58).wrapping_add(u16_b * 0x50).wrapping_add(0x30));
                    let off_tbl = rd::<usize>(r11 + 0x18);
                    let comp_off =
                        rd::<u32>((off_tbl as isize + (found as isize) * 4) as usize) as u64;
                    let arg_rdx = comp_off.wrapping_add(rd::<u64>(r11 + 0x08));
                    let arg_r8 = rd::<u32>(f48.wrapping_add((idx_u32 as usize) * 4));
                    exe::call_va!(fn_addr, (val_c, arg_rdx, arg_r8));
                }
            }
        }

        rem = rem.wrapping_sub(1);
    }
}

/// `unknown_1420bcd80`
/// Original: 0x1420bcd80 (1102 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1420bcd80(
    param_1: usize,
    param_2: u64,
    param_3: usize,
    param_4: u64,
    param_5: u64,
    param_6: usize,
    param_7: usize,
    param_8: u64,
    param_9: usize,
    param_10: usize,
    param_11: usize,
) -> usize {
    let count_9 = rd::<u32>(param_9 + 0x28) as usize;
    for i in 0..count_9 {
        let w = rd::<u16>(rd::<usize>(param_9 + 0x20).wrapping_add(i * 2));
        let entry = rd::<usize>(param_9 + 0x08).wrapping_add(i * 0x50);
        let mut cur = rd::<usize>(entry + 0x20);
        let end = cur.wrapping_add((rd::<u32>(entry + 0x28) as usize) * 4);
        if cur != end {
            let dst_arr = rd::<usize>(param_3 + 0x08);
            while cur != end {
                let idx = rd::<u32>(cur) as usize;
                wr::<u16>(dst_arr.wrapping_add(idx * 2), w);
                cur = cur.wrapping_add(4);
            }
        }
    }

    let r15 = rd::<u64>(param_3);
    let handles_ptr = exe::call_va!(0x1_411f_11e0, (param_2, r15.wrapping_mul(8), 4u32) -> usize);
    let n32 = r15 as u32;
    for i in 0..(n32 as usize) {
        wr::<u64>(handles_ptr.wrapping_add(i * 8), u64::MAX);
    }

    let n_usize = n32 as usize;
    let mut filled: usize = 0;
    let mut free_idx = rd::<u32>(param_6 + 0x10);
    while free_idx != u32::MAX && filled < n_usize {
        let slot_ptr = rd::<usize>(param_6).wrapping_add((free_idx as usize) * 8);
        let gen = rd::<u32>(slot_ptr + 0x04);
        let next_free = rd::<u32>(slot_ptr);
        wr::<u32>(param_6 + 0x10, next_free);
        let handle = ((gen as u64) << 32) | (free_idx as u64);
        wr::<u64>(slot_ptr, handle);
        wr::<u64>(handles_ptr.wrapping_add(filled * 8), handle);
        filled += 1;
        free_idx = rd::<u32>(param_6 + 0x10);
    }
    while filled < n_usize {
        let cur_len = rd::<u32>(param_6 + 0x08);
        let cur_cap = rd::<u32>(param_6 + 0x0c);
        if (cur_len as u64) * 8 == (cur_cap as u64) * 8 {
            exe::call_va!(0x1_401e_6d50, (param_6, (cur_len as u64).wrapping_add(1)));
        }
        let handle = (1u64 << 32) | (cur_len as u64);
        let write_idx = rd::<u32>(param_6 + 0x08) as usize;
        wr::<u64>(rd::<usize>(param_6).wrapping_add(write_idx * 8), handle);
        wr::<u32>(param_6 + 0x08, (write_idx as u32).wrapping_add(1));
        wr::<u64>(handles_ptr.wrapping_add(filled * 8), handle);
        filled += 1;
    }

    let indices_ptr = exe::call_va!(0x1_411f_11e0, (param_2, r15.wrapping_mul(4), 4u32) -> usize);
    if n32 != 0 {
        let _ = exe::call_va!(0x1_4183_acc2, (indices_ptr, 0u32, (n_usize as u64) * 4) -> usize);
    }

    if r15 != 0 {
        let p11_ptr = rd::<usize>(param_11 + 0x08);
        let p10_ptr = rd::<usize>(param_10 + 0x08);
        let mut cur_w = rd::<usize>(param_3 + 0x08);
        for i in 0..(r15 as usize) {
            let w = rd::<u16>(cur_w) as u32;
            let found = exe::call_va!(
                0x1_4248_9ee0,
                (rd::<u64>(param_9 + 0x20), rd::<u32>(param_9 + 0x28), w) -> i32
            );
            let entry = if (found as isize) == -1 {
                p11_ptr.wrapping_add((w as usize) * 0x30)
            } else {
                p10_ptr.wrapping_add((found as isize as usize) * 0x30)
            };
            let old_cnt = rd::<u32>(entry + 0x28);
            wr::<u32>(entry + 0x28, old_cnt.wrapping_add(1));
            wr::<u32>(indices_ptr.wrapping_add(i * 4), old_cnt);
            cur_w = cur_w.wrapping_add(2);
        }
    }

    let g_job_sys = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
    let s_name = exe::ptr::<u8>(0x1_433d_db08) as usize;
    let worker_arr = rd::<usize>(g_job_sys + 0x10);
    let tid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let pool_40 =
        rd::<usize>(worker_arr.wrapping_add(tid * 0x40).wrapping_add(0x48)).wrapping_add(0x40);

    let mut parent_hdl = [0u8; 16];
    let ph = parent_hdl.as_mut_ptr() as usize;
    let _ = exe::call_va!(0x1_4248_5aa0, (pool_40, ph) -> usize);

    let pnode = rd::<usize>(ph);
    let _ = crate::simd_ops::atomic_xchg_u32(pnode as u64, 1);
    let pnode = rd::<usize>(ph);
    wr::<u64>(pnode + 0x08, 0);
    wr::<usize>(pnode + 0x10, exe::ptr::<u8>(0x1_4340_e320) as usize);
    wr::<usize>(pnode + 0x18, s_name);
    wr::<u64>(pnode + 0x20, 0x31);
    let (ok, _) = crate::simd_ops::atomic_cmpxchg_u8((pnode + 0x04) as u64, 1, 3);
    if ok {
        let pnode2 = rd::<usize>(ph);
        let _ = crate::simd_ops::atomic_add_u32(pnode2 as u64, 0xffff);
    }
    wr::<u8>(ph + 0x0a, 1);

    let mut job_ctx: [u64; 13] = [
        param_8.wrapping_add(0x100),
        r15,
        rd::<u64>(param_3 + 0x08),
        param_5,
        rd::<u32>(param_9 + 0x28) as u64,
        rd::<u64>(param_9 + 0x20),
        rd::<u64>(param_10),
        rd::<u64>(param_10 + 0x08),
        rd::<u64>(param_11),
        rd::<u64>(param_11 + 0x08),
        param_4,
        n32 as u64,
        indices_ptr as u64,
    ];
    exe::call_va!(0x1_420b_d4d0, (job_ctx.as_mut_ptr() as usize, ph));

    if r15 != 0 {
        let p11_ptr = rd::<usize>(param_11 + 0x08);
        let p10_ptr = rd::<usize>(param_10 + 0x08);
        let mut cur_w = rd::<usize>(param_3 + 0x08);
        let mut cur_idx_ptr = indices_ptr;
        let pad_mid = (param_3 as u64) & 0xffff_0000;

        for i in 0..(r15 as usize) {
            let handle = rd::<u64>(handles_ptr.wrapping_add(i * 8));
            let w = rd::<u16>(cur_w) as u32;
            let found = exe::call_va!(
                0x1_4248_9ee0,
                (rd::<u64>(param_9 + 0x20), rd::<u32>(param_9 + 0x28), w) -> i32
            );
            let entry = if (found as isize) == -1 {
                p11_ptr.wrapping_add((w as usize) * 0x30)
            } else {
                p10_ptr.wrapping_add((found as isize as usize) * 0x30)
            };
            let slot_idx = rd::<u32>(cur_idx_ptr);
            let block_ptr = rd::<usize>(entry + 0x08);
            wr::<u64>(
                block_ptr
                    .wrapping_add(0x10)
                    .wrapping_add((slot_idx as usize) * 8),
                handle,
            );
            let packed = ((slot_idx as u64) << 32) | pad_mid | (w as u64);
            let ent_id = (handle as u32) as usize;
            wr::<u64>(rd::<usize>(param_7).wrapping_add(ent_id * 8), packed);

            cur_w = cur_w.wrapping_add(2);
            cur_idx_ptr = cur_idx_ptr.wrapping_add(4);
        }
    }

    exe::call_va!(0x1_4248_5680, (g_job_sys, ph));
    wr::<u64>(param_1, param_2);
    wr::<usize>(param_1 + 0x08, indices_ptr);
    wr::<u32>(param_1 + 0x10, n32);
    wr::<u32>(param_1 + 0x14, n32);
    param_1
}

/// `unknown_1420c3bb0`
/// Original: 0x1420c3bb0 (2069 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1420c3bb0(param_1: usize) {
    let alloc_ctx = rd::<u64>(param_1 + 0x30);
    let ctx0 = rd::<usize>(param_1);
    let count_40 = rd::<u64>(ctx0 + 0x40);
    let mut arr_ptr: usize = 0;
    let mut cap_e8: u32 = 0;

    if count_40 != 0 {
        let byte_len = count_40.wrapping_mul(4);
        arr_ptr = exe::call_va!(0x1_411f_11e0, (alloc_ctx, byte_len, 4u32) -> usize);
        if arr_ptr == 0 {
            arr_ptr = exe::call_va!(0x1_411f_11e0, (alloc_ctx, byte_len, 4u32) -> usize);
        }
        cap_e8 = count_40 as u32;
        for i in 0..(count_40 as usize) {
            wr::<u32>(arr_ptr.wrapping_add(i * 4), 0);
        }
    }

    let mut cur_node = rd::<usize>(rd::<usize>(param_1) + 0x30);
    let mut dst_cursor = arr_ptr;
    while cur_node != 0 {
        let chunk_cnt = rd::<usize>(cur_node + 0x08);
        let chunk_bytes = (chunk_cnt * 4) as u64;
        let _ = exe::call_va!(
            0x1_4183_acc8,
            (dst_cursor, cur_node.wrapping_add(0x10), chunk_bytes) -> usize
        );
        dst_cursor = dst_cursor.wrapping_add(rd::<usize>(cur_node + 0x08) * 4);
        cur_node = rd::<usize>(cur_node);
    }

    exe::call_va!(0x1_4103_a0c0, (arr_ptr, count_40 as u32));

    let mut vec20: [u64; 3] = [alloc_ctx, 0, 0];
    let v20_ptr = vec20.as_mut_ptr() as usize;
    let mut r13: usize = 0;
    let mut r14d: u32 = 0;

    let ctx0 = rd::<usize>(param_1);
    let mut node1 = rd::<usize>(ctx0 + 0x50);
    let mut idx1: u64 = 0;
    let mut node2 = rd::<usize>(ctx0 + 0x68);
    let mut idx2: u64 = 0;
    let end_node2 = rd::<usize>(ctx0 + 0x70);
    let end_idx2 = if end_node2 != 0 {
        rd::<u64>(end_node2 + 0x08)
    } else {
        0
    };

    while node2 != end_node2 || idx2 != end_idx2 {
        let w = rd::<u16>(node2.wrapping_add((idx2 as usize) * 2).wrapping_add(0x10));
        let old_node1 = node1;
        let old_idx1 = idx1;
        idx1 = idx1.wrapping_add(1);
        if idx1 == rd::<u64>(node1 + 0x08) {
            let next1 = rd::<usize>(node1);
            if next1 != 0 {
                node1 = next1;
                idx1 = 0;
            }
        }

        let r11d = rd::<u32>(old_node1.wrapping_add((old_idx1 as usize) * 4).wrapping_add(0x10));
        let arch = rd::<u16>(rd::<usize>(param_1 + 0x10).wrapping_add((r11d as usize) * 8)) as usize;
        let base_18 = rd::<usize>(param_1 + 0x18);
        let search_len = rd::<u32>(base_18.wrapping_add(arch * 0x20).wrapping_add(0x18068)) as usize;
        let search_ptr = rd::<usize>(base_18.wrapping_add(arch * 0x20).wrapping_add(0x18050));

        let simd_end = search_len & !0xfusize;
        let mut found_idx: Option<u32> = None;
        let mut i: usize = 0;
        while i < simd_end {
            for lane in 0..16usize {
                if rd::<u16>(search_ptr.wrapping_add((i + lane) * 2)) == w {
                    found_idx = Some((i + lane) as u32);
                    break;
                }
            }
            if found_idx.is_some() {
                break;
            }
            i += 16;
        }
        if found_idx.is_none() {
            let mut j = simd_end;
            while j < search_len {
                if rd::<u16>(search_ptr.wrapping_add(j * 2)) == w {
                    found_idx = Some(j as u32);
                    break;
                }
                j += 1;
            }
        }

        if let Some(fidx) = found_idx {
            if fidx != u32::MAX {
                let cur_cap = rd::<u32>(v20_ptr + 0x14);
                if r14d == cur_cap {
                    let min_cap = (r14d.wrapping_add(1)) as u64;
                    let grown = (cur_cap as u64).wrapping_mul(3);
                    let new_cap = if grown < min_cap { min_cap } else { grown };
                    exe::call_va!(0x1_411f_2380, (v20_ptr, new_cap, 8u32));
                    r14d = rd::<u32>(v20_ptr + 0x10);
                    r13 = rd::<usize>(v20_ptr + 0x08);
                }
                let pos = r14d as usize;
                r14d = r14d.wrapping_add(1);
                wr::<u32>(v20_ptr + 0x10, r14d);
                let packed = ((r11d as u64) << 32) | (w as u64);
                wr::<u64>(r13.wrapping_add(pos * 8), packed);
            }
        }

        idx2 = idx2.wrapping_add(1);
        if idx2 == rd::<u64>(node2 + 0x08) {
            let next2 = rd::<usize>(node2);
            if next2 != 0 {
                node2 = next2;
                idx2 = 0;
            }
        }
    }

    exe::call_va!(0x1_40fa_e550, (r13, r14d));

    let end_ptr = r13.wrapping_add((r14d as usize) * 8);
    let mut new_end = end_ptr;
    if r13 != end_ptr && r13.wrapping_add(8) != end_ptr {
        let mut p_prev = r13;
        let mut p_cur = r13.wrapping_add(8);
        let mut v_prev = rd::<u64>(p_prev);
        loop {
            let v_cur = rd::<u64>(p_cur);
            let p_next = p_cur.wrapping_add(8);
            if v_prev == v_cur {
                let mut scan = p_next;
                while scan != end_ptr {
                    let v_scan = rd::<u64>(scan);
                    if rd::<u64>(p_prev) != v_scan {
                        p_prev = p_prev.wrapping_add(8);
                        wr::<u64>(p_prev, v_scan);
                    }
                    scan = scan.wrapping_add(8);
                }
                new_end = p_prev.wrapping_add(8);
                break;
            }
            p_prev = p_cur;
            p_cur = p_next;
            v_prev = v_cur;
            if p_next == end_ptr {
                break;
            }
        }
    }

    let unique_len = (new_end.wrapping_sub(r13)) >> 3;
    let cur_cap = rd::<u32>(v20_ptr + 0x14) as usize;
    if unique_len > cur_cap {
        exe::call_va!(0x1_411f_2380, (v20_ptr, unique_len as u64, 8u32));
        r14d = rd::<u32>(v20_ptr + 0x10);
        r13 = rd::<usize>(v20_ptr + 0x08);
    }
    if unique_len > (r14d as usize) {
        for i in (r14d as usize)..unique_len {
            wr::<u64>(r13.wrapping_add(i * 8), 0);
        }
    }

    let r15_len = (unique_len as u32) as usize;
    wr::<u64>(v20_ptr, 0);
    wr::<u64>(v20_ptr + 0x08, 0);
    wr::<u64>(v20_ptr + 0x10, 0);
    let num_words = (r15_len.wrapping_add(0x3f)) >> 6;
    exe::call_va!(0x1_410e_f200, (v20_ptr, num_words as u64));
    let prev_bits = rd::<u64>(v20_ptr + 0x10);
    let r14_bits = rd::<usize>(v20_ptr);
    if (prev_bits as usize) < r15_len {
        wr::<u64>(v20_ptr, num_words as u64);
        wr::<usize>(v20_ptr + 0x08, r14_bits);
        wr::<u64>(v20_ptr + 0x10, r15_len as u64);
        let diff_bits = (r15_len as u64).wrapping_sub(prev_bits);
        exe::call_va!(0x1_40ff_f0a0, (v20_ptr, prev_bits, diff_bits, 0u8));
    }

    let ctx0 = rd::<usize>(param_1);
    let mut node3 = rd::<usize>(ctx0 + 0x108);
    let mut idx3: u64 = 0;
    let mut node4 = rd::<usize>(ctx0 + 0xf0);
    let mut idx4: u64 = 0;
    let end_node4 = rd::<usize>(ctx0 + 0xf8);
    let end_idx4 = if end_node4 != 0 {
        rd::<u64>(end_node4 + 0x08)
    } else {
        0
    };

    while node4 != end_node4 || idx4 != end_idx4 {
        let old_node3 = node3;
        let old_idx3 = idx3;
        idx3 = idx3.wrapping_add(1);
        if idx3 == rd::<u64>(node3 + 0x08) {
            let next3 = rd::<usize>(node3);
            if next3 != 0 {
                node3 = next3;
                idx3 = 0;
            }
        }

        let u16_3 =
            rd::<u16>(old_node3.wrapping_add((old_idx3 as usize) * 2).wrapping_add(0x10)) as u64;
        let u32_4 = rd::<u32>(node4.wrapping_add((idx4 as usize) * 4).wrapping_add(0x10)) as u64;
        let key = (u32_4 << 32) | u16_3;

        let mut pos: usize = 0;
        if (r15_len as u32) != 0 {
            let mut rem = r15_len;
            while rem != 0 {
                let half = rem >> 1;
                let next_pos = pos.wrapping_sub(half).wrapping_add(rem);
                let mid = pos.wrapping_add(half);
                if key >= rd::<u64>(r13.wrapping_add(mid * 8)) {
                    pos = next_pos;
                }
                rem = half;
            }
        }

        let cand = pos.wrapping_sub(1);
        let edx: u32 = if cand < r15_len { cand as u32 } else { 0 };
        let elem = if (r15_len as u32) != 0 {
            rd::<u64>((r13 as isize + (edx as i32 as isize) * 8) as usize)
        } else {
            !key
        };
        let match_idx: u32 = if key == elem { edx } else { u32::MAX };
        if match_idx != u32::MAX {
            let bit_pos = match_idx as i32 as isize as usize;
            let word_addr = r14_bits.wrapping_add((bit_pos >> 6) * 8);
            wr::<u64>(word_addr, rd::<u64>(word_addr) | (1u64 << (match_idx & 0x3f)));
        }

        idx4 = idx4.wrapping_add(1);
        if idx4 == rd::<u64>(node4 + 0x08) {
            let next4 = rd::<usize>(node4);
            if next4 != 0 {
                node4 = next4;
                idx4 = 0;
            }
        }
    }

    if (r15_len as u32) != 0 {
        for i in 0..r15_len {
            let hi = (rd::<u64>(r13.wrapping_add(i * 8)) >> 32) as u32;
            let found = exe::call_va!(0x1_411f_36a0, (arr_ptr, count_40 as u32, hi) -> i32);
            if found != -1 {
                let word_addr = r14_bits.wrapping_add((i >> 6) * 8);
                wr::<u64>(
                    word_addr,
                    rd::<u64>(word_addr) | (1u64 << ((i as u32) & 0x3f)),
                );
            }
        }
    }

    let mut out_opt: [u64; 2] = [0; 2];
    let opt_ptr = out_opt.as_mut_ptr() as usize;
    let num_words = (r15_len.wrapping_add(0x3f)) >> 6;
    wr::<u64>(v20_ptr, num_words as u64);
    wr::<usize>(v20_ptr + 0x08, r14_bits);
    wr::<u64>(v20_ptr + 0x10, r15_len as u64);

    let mut res_ptr = exe::call_va!(0x1_411d_3b20, (v20_ptr, opt_ptr, 0u64) -> usize);
    wr::<u64>(v20_ptr, num_words as u64);
    wr::<usize>(v20_ptr + 0x08, r14_bits);
    wr::<u64>(v20_ptr + 0x10, r15_len as u64);
    let mut bit_idx = rd::<u64>(res_ptr);
    let mut has_bit = rd::<u8>(res_ptr + 0x08);
    while has_bit != 0 {
        let dst_vec = rd::<usize>(param_1 + 0x28);
        let len = rd::<u32>(dst_vec + 0x10);
        let cap = rd::<u32>(dst_vec + 0x14);
        if len == cap {
            let min_cap = (len.wrapping_add(1)) as u64;
            let grown = (cap as u64).wrapping_mul(3);
            let new_cap = if grown < min_cap { min_cap } else { grown };
            exe::call_va!(0x1_411f_2380, (dst_vec, new_cap, 8u32));
        }
        let pos = rd::<u32>(dst_vec + 0x10) as usize;
        wr::<u32>(dst_vec + 0x10, (pos as u32).wrapping_add(1));
        let val = rd::<u64>(r13.wrapping_add((bit_idx as usize) * 8));
        wr::<u64>(rd::<usize>(dst_vec + 0x08).wrapping_add(pos * 8), val);

        res_ptr = exe::call_va!(
            0x1_411d_3b20,
            (v20_ptr, opt_ptr, bit_idx.wrapping_add(1)) -> usize
        );
        bit_idx = rd::<u64>(res_ptr);
        has_bit = rd::<u8>(res_ptr + 0x08);
    }

    let num_words = (r15_len.wrapping_add(0x3f)) >> 6;
    if num_words != 0 {
        for w_i in 0..num_words {
            let mask = if w_i == num_words - 1 {
                let sh = (!(r15_len.wrapping_sub(1)) as u32) & 0x3f;
                u64::MAX >> sh
            } else {
                u64::MAX
            };
            let w_addr = r14_bits.wrapping_add(w_i * 8);
            wr::<u64>(w_addr, !(rd::<u64>(w_addr) & mask));
        }
    }

    let num_words = (r15_len.wrapping_add(0x3f)) >> 6;
    wr::<u64>(v20_ptr, num_words as u64);
    wr::<usize>(v20_ptr + 0x08, r14_bits);
    wr::<u64>(v20_ptr + 0x10, r15_len as u64);

    let mut res_ptr = exe::call_va!(0x1_411d_3b20, (v20_ptr, opt_ptr, 0u64) -> usize);
    wr::<u64>(v20_ptr, num_words as u64);
    wr::<usize>(v20_ptr + 0x08, r14_bits);
    wr::<u64>(v20_ptr + 0x10, r15_len as u64);
    let mut bit_idx = rd::<u64>(res_ptr);
    let mut has_bit = rd::<u8>(res_ptr + 0x08);
    while has_bit != 0 {
        let dst_vec = rd::<usize>(param_1 + 0x20);
        let len = rd::<u32>(dst_vec + 0x10);
        let cap = rd::<u32>(dst_vec + 0x14);
        if len == cap {
            let min_cap = (len.wrapping_add(1)) as u64;
            let grown = (cap as u64).wrapping_mul(3);
            let new_cap = if grown < min_cap { min_cap } else { grown };
            exe::call_va!(0x1_411f_2380, (dst_vec, new_cap, 8u32));
        }
        let pos = rd::<u32>(dst_vec + 0x10) as usize;
        wr::<u32>(dst_vec + 0x10, (pos as u32).wrapping_add(1));
        let val = rd::<u64>(r13.wrapping_add((bit_idx as usize) * 8));
        wr::<u64>(rd::<usize>(dst_vec + 0x08).wrapping_add(pos * 8), val);

        res_ptr = exe::call_va!(
            0x1_411d_3b20,
            (v20_ptr, opt_ptr, bit_idx.wrapping_add(1)) -> usize
        );
        bit_idx = rd::<u64>(res_ptr);
        has_bit = rd::<u8>(res_ptr + 0x08);
    }

    let out_38 = rd::<usize>(param_1 + 0x38);
    wr::<u64>(out_38, alloc_ctx);
    wr::<usize>(out_38 + 0x08, arr_ptr);
    wr::<u32>(out_38 + 0x10, count_40 as u32);
    wr::<u32>(out_38 + 0x14, cap_e8);

    if r14_bits != 0 {
        exe::call_va!(0x1_4241_c190, (r14_bits));
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_r_1420b2890(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut GROW_16_CALLS: u32 = 0;
    static mut GROW_32_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_5d10(vec_ptr: usize, min_cap: u64) {
        GROW_16_CALLS += 1;
        wr::<u32>(vec_ptr + 0x0c, (min_cap as u32) + 4);
    }
    unsafe extern "win64" fn mock_02b0(vec_ptr: usize, min_cap: u64) {
        GROW_32_CALLS += 1;
        wr::<u32>(vec_ptr + 0x0c, (min_cap as u32) + 4);
    }

    let _m1 = crate::difftest::Mock::new(0x1_40fa_5d10, mock_5d10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411c_02b0, mock_02b0 as *const ())?;

    let run = |f: F| -> ([u16; 8], [u16; 8], [u32; 8], [u32; 6], u32, u32) {
        GROW_16_CALLS = 0;
        GROW_32_CALLS = 0;

        let mut buf_w0 = [0u16; 8];
        let mut buf_w1 = [0u16; 8];
        let mut buf_d1 = [0u32; 8];

        let mut out_entries = [0u8; 0x30 * 2];
        let oep = out_entries.as_mut_ptr() as usize;
        // Entry 0: len == cap == 0 to trigger 0x140fa5d10 on first push
        wr::<usize>(oep, buf_w0.as_mut_ptr() as usize);
        wr::<u32>(oep + 0x08, 0);
        wr::<u32>(oep + 0x0c, 0);

        // Entry 1: vec_10 has len == cap == 0, vec_20 has len == cap == 0
        let e1 = oep + 0x30;
        wr::<usize>(e1 + 0x10, buf_w1.as_mut_ptr() as usize);
        wr::<u32>(e1 + 0x18, 0);
        wr::<u32>(e1 + 0x1c, 0);
        wr::<usize>(e1 + 0x20, buf_d1.as_mut_ptr() as usize);
        wr::<u32>(e1 + 0x28, 0);
        wr::<u32>(e1 + 0x2c, 0);

        let w_targets: [u16; 2] = [0, 0];
        let q_targets: [u64; 2] = [(0xaaaa_1111u64 << 32) | 1, (0xbbbb_2222u64 << 32) | 1];
        let mut grp_arr = [0u8; 0x40];
        let gp = grp_arr.as_mut_ptr() as usize;
        let g1 = gp + 0x20;
        wr::<u64>(g1, 2);
        wr::<usize>(g1 + 0x08, w_targets.as_ptr() as usize);
        wr::<u64>(g1 + 0x10, 2);
        wr::<usize>(g1 + 0x18, q_targets.as_ptr() as usize);

        let u32_items: [u32; 2] = [(1u32 << 16) | 0x1234, (1u32 << 16) | 0x5678];
        let mut span_elem = [0u8; 0x10];
        let sep = span_elem.as_mut_ptr() as usize;
        wr::<usize>(sep, u32_items.as_ptr() as usize);
        wr::<u32>(sep + 0x08, 2);

        let p1: [u64; 2] = [1, sep as u64];
        let p2: [u64; 2] = [0, gp as u64];
        let p3: [usize; 1] = [oep];

        f(
            p1.as_ptr() as usize,
            p2.as_ptr() as usize,
            p3.as_ptr() as usize,
        );

        let meta: [u32; 6] = [
            rd::<u32>(oep + 0x08),
            rd::<u32>(oep + 0x0c),
            rd::<u32>(e1 + 0x18),
            rd::<u32>(e1 + 0x1c),
            rd::<u32>(e1 + 0x28),
            rd::<u32>(e1 + 0x2c),
        ];
        (buf_w0, buf_w1, buf_d1, meta, GROW_16_CALLS, GROW_32_CALLS)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1420b2890 out");
    Ok(1)
}

pub unsafe fn diff_r_1420b2a90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut INSERT_LOG: [u64; 4] = [0; 4];
    static mut INSERT_CNT: usize = 0;

    unsafe extern "win64" fn mock_00d0(
        vec_ptr: usize,
        out_iter: usize,
        pos_ptr: usize,
        val_ptr: usize,
    ) -> usize {
        let base = rd::<usize>(vec_ptr);
        let len = rd::<u32>(vec_ptr + 0x08) as usize;
        let idx = pos_ptr.wrapping_sub(base) >> 1;
        let val = rd::<u16>(val_ptr);
        // Shift elements right in-place and insert val so subsequent binary searches see updated array
        let mut k = len;
        while k > idx {
            let prev = rd::<u16>(base.wrapping_add((k - 1) * 2));
            wr::<u16>(base.wrapping_add(k * 2), prev);
            k -= 1;
        }
        wr::<u16>(base.wrapping_add(idx * 2), val);
        wr::<u32>(vec_ptr + 0x08, (len + 1) as u32);
        wr::<usize>(out_iter, pos_ptr);
        let ci = INSERT_CNT;
        if ci < 4 {
            INSERT_LOG[ci] = ((idx as u64) << 16) | (val as u64);
            INSERT_CNT = ci + 1;
        }
        out_iter
    }

    let _m1 = crate::difftest::Mock::new(0x1_411f_00d0, mock_00d0 as *const ())?;

    let run = |f: F| -> ([u16; 8], u32, [u64; 4]) {
        INSERT_LOG = [0; 4];
        INSERT_CNT = 0;

        let mut arr = [10u16, 20, 30, 0, 0, 0, 0, 0];
        let mut vecs = [0u8; 0x20];
        let vp = vecs.as_mut_ptr() as usize;
        let v1 = vp + 0x10;
        wr::<usize>(v1, arr.as_mut_ptr() as usize);
        wr::<u32>(v1 + 0x08, 3);
        wr::<u32>(v1 + 0x0c, 8);

        let items: [u32; 4] = [
            (1u32 << 16) | 5,
            (1u32 << 16) | 20,
            (1u32 << 16) | 25,
            (1u32 << 16) | 40,
        ];
        let p1: [u64; 2] = [4, items.as_ptr() as u64];
        let p2: [usize; 1] = [vp];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize);
        (arr, rd::<u32>(v1 + 0x08), INSERT_LOG)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1420b2a90 out");
    Ok(1)
}

pub unsafe fn diff_r_1420b3cb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SCRATCH_BUF: [u16; 32] = [0; 32];
    static mut JOB_NODES: [[u8; 0x40]; 8] = [[0; 0x40]; 8];
    static mut NEXT_NODE: usize = 0;
    static mut CALL_SUM: u64 = 0;

    unsafe extern "win64" fn mock_c0a0(_size: u64, _align: u32) -> usize {
        core::ptr::addr_of_mut!(SCRATCH_BUF) as usize
    }
    unsafe extern "win64" fn mock_c190(ptr: usize) {
        if ptr == core::ptr::addr_of_mut!(SCRATCH_BUF) as usize {
            CALL_SUM = CALL_SUM.wrapping_add(0x100);
        }
    }
    unsafe extern "win64" fn mock_7c80() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_5aa0(_pool: usize, out_hdl: usize) -> usize {
        let idx = NEXT_NODE & 7;
        NEXT_NODE += 1;
        let np = core::ptr::addr_of_mut!(JOB_NODES[idx]) as usize;
        wr::<[u8; 0x40]>(np, [0; 0x40]);
        wr::<u8>(np + 0x04, 1);
        wr::<usize>(out_hdl, np);
        wr::<u16>(out_hdl + 0x08, idx as u16);
        wr::<u16>(out_hdl + 0x0a, 0);
        wr::<u8>(out_hdl + 0x0c, 0);
        out_hdl
    }
    unsafe extern "win64" fn mock_6460(_warr: usize, jnode: usize) {
        let r0 = rd::<u64>(jnode + 0x20);
        let r1 = rd::<u64>(jnode + 0x28);
        let r2 = rd::<u64>(jnode + 0x30);
        CALL_SUM = CALL_SUM
            .wrapping_add(r0)
            .wrapping_add(r1 & 0xffff)
            .wrapping_add(r2);
    }
    unsafe extern "win64" fn mock_33f0(p3: usize, span_ptr: usize, w_off: u64, tail_bits: u64) {
        let cnt = rd::<u64>(span_ptr);
        let first = rd::<u16>(rd::<usize>(span_ptr + 0x08)) as u64;
        CALL_SUM = CALL_SUM
            .wrapping_add((p3 & 0xff) as u64)
            .wrapping_add(cnt * 10 + first)
            .wrapping_add(w_off * 100 + tail_bits);
    }
    unsafe extern "win64" fn mock_c9d0(_lock: usize, val: u64) {
        CALL_SUM = CALL_SUM.wrapping_add(val);
    }
    unsafe extern "win64" fn mock_03b0(_p2_plus_8: usize, span_ptr: usize, range_packed: u64) {
        let cnt = rd::<u64>(span_ptr);
        CALL_SUM = CALL_SUM.wrapping_add(cnt).wrapping_add(range_packed);
    }
    unsafe extern "win64" fn mock_3200(_r10: usize, _hdr: usize, mode: u8, _ptr: usize) {
        CALL_SUM = CALL_SUM.wrapping_add(0x5000 + mode as u64);
    }
    unsafe extern "win64" fn mock_5680(_jsys: usize, hdl: usize) {
        let pnode = rd::<usize>(hdl);
        CALL_SUM = CALL_SUM.wrapping_add(rd::<u32>(pnode) as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_5aa0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4248_6460, mock_6460 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_420b_33f0, mock_33f0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4248_c9d0, mock_c9d0 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_411f_03b0, mock_03b0 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4253_3200, mock_3200 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4248_5680, mock_5680 as *const ())?;

    let g_2f8 = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let saved_2f8 = rd::<usize>(g_2f8);

    let run = |f: F| -> (u64, u32, u32, u32, [u64; 3], [u16; 3], [u64; 2]) {
        SCRATCH_BUF = [0; 32];
        JOB_NODES = [[0; 0x40]; 8];
        NEXT_NODE = 0;
        CALL_SUM = 0;

        let mut sem_holder = vec![0u8; 0x1020];
        let mut worker_arr = [0u8; 0x80];
        let wap = worker_arr.as_mut_ptr() as usize;
        wr::<usize>(wap, sem_holder.as_mut_ptr() as usize);
        wr::<usize>(wap + 0x48, 0x20_0000);

        let mut jsys = [0u8; 0x20];
        let jsp = jsys.as_mut_ptr() as usize;
        wr::<u32>(jsp, 2); // 2 workers
        wr::<usize>(jsp + 0x10, wap);
        wr::<usize>(g_2f8, jsp);

        // Set up fake 32MB-aligned slab arena in a Vec or fake pointer:
        // Wait: if block_ptr == 0 we skip the slab free, or we can test block_ptr != 0 with a real aligned region?
        // Actually, in phase 1, group 1 is selected only if [param_1 + 0x48 + 1*8] != 0.
        // And before phase 5, we can have mock_03b0 clear or set up the slab header if needed, OR allocate a 64MB virtual region?
        // Wait: 64MB vec is trivial in test, or mock_03b0 can zero [param_1 + 0x48 + 1*8] after phase 1!
        // Even better: let's allocate a 64MB-aligned region or let's check how big 64MB is: 64MB is just `vec![0u8; 0x400_0000]`,
        // where `(base + 0x200_0000) & !0x01ff_ffff` is guaranteed to be inside the Vec!
        let mut slab_backing = vec![0u8; 0x400_0000];
        let sb_base = slab_backing.as_mut_ptr() as usize;
        let r10_base = (sb_base + 0x01ff_ffff) & !0x01ff_ffffusize;
        let block_ptr = r10_base + 0x10000; // page_idx = 1
        // Set [r10_base + 0x100] = 0 (not equal to TEB, so it calls mock_3200!)
        wr::<usize>(r10_base + 0x100, 0);
        let hdr_raw = r10_base + 0x108 + 0x50;
        wr::<u32>(hdr_raw + 0x04, 0);

        let mut keys_arr: [u64; 3] = [100, 200, 300];
        let mut ids_arr: [u16; 3] = [0, 1, 2];
        let mut key_lookup: [u64; 2] = [100, 200];

        let mut p1 = vec![0u8; 0x58100];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x08, 2); // 2 groups
        wr::<usize>(p1p + 0x18, keys_arr.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x20, 3);
        wr::<usize>(p1p + 0x28, ids_arr.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x30, 3);
        wr::<usize>(p1p + 0x38, key_lookup.as_mut_ptr() as usize);
        // Group 0: non-zero count (skipped)
        wr::<u32>(p1p + 0x10048, 5);
        wr::<usize>(p1p + 0x48, block_ptr);
        // Group 1: zero count and non-zero block_ptr (selected!)
        wr::<u32>(p1p + 0x10048 + 4, 0);
        wr::<usize>(p1p + 0x48 + 8, block_ptr);
        wr::<u32>(p1p + 0x18048, 0x77);
        wr::<u64>(p1p + 0x18050 + 0x20, 0x42);

        let mut p2_inner = [0u8; 0x40];
        wr::<u32>(p2_inner.as_mut_ptr() as usize + 0x38, 7);
        let p2: [usize; 2] = [p2_inner.as_mut_ptr() as usize, 0];

        let mut bit_rows: [u64; 4] = [u64::MAX; 4];
        let mut p3_sub = [0u8; 0x20];
        wr::<u32>(p3_sub.as_mut_ptr() as usize + 0x10, 192);
        let mut p3 = [0u8; 0xc0];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<usize>(p3p + 0x08, p3_sub.as_mut_ptr() as usize);
        wr::<usize>(p3p + 0x70, bit_rows.as_mut_ptr() as usize);
        wr::<u64>(p3p + 0xa0, 5);
        wr::<u64>(p3p + 0xb0, 2);

        f(p1p, p2.as_ptr() as usize, p3p, 0);

        (
            CALL_SUM,
            rd::<u32>(p1p + 0x18048),
            rd::<u32>(p1p + 0x20),
            rd::<u32>(p1p + 0x30),
            keys_arr,
            ids_arr,
            [bit_rows[2], key_lookup[1]],
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_2f8, saved_2f8);
    crate::diff_eq!(o, n, "1420b3cb0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1420bba90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CB_LOG: [u64; 6] = [0; 6];
    static mut CB_CNT: usize = 0;

    unsafe extern "win64" fn mock_cb(rcx_c: u64, rdx_ptr: u64, r8_val: u32) {
        let i = CB_CNT;
        if i < 2 {
            CB_LOG[i * 3] = rcx_c;
            CB_LOG[i * 3 + 1] = rdx_ptr;
            CB_LOG[i * 3 + 2] = r8_val as u64;
            CB_CNT = i + 1;
        }
    }

    let run = |f: F| -> [u64; 6] {
        CB_LOG = [0; 6];
        CB_CNT = 0;

        // Node A (u32s at +0x10): 2 nodes chained
        let mut node_a2 = [0u8; 0x20];
        let na2 = node_a2.as_mut_ptr() as usize;
        wr::<usize>(na2, 0);
        wr::<u64>(na2 + 0x08, 2);
        wr::<u32>(na2 + 0x10, 25);

        let mut node_a1 = [0u8; 0x20];
        let na1 = node_a1.as_mut_ptr() as usize;
        wr::<usize>(na1, na2);
        wr::<u64>(na1 + 0x08, 1);
        wr::<u32>(na1 + 0x10, 15);

        // Node B (u16s at +0x10): 1 node with 2 items (comp_id = 1, 1)
        let mut node_b1 = [0u8; 0x20];
        let nb1 = node_b1.as_mut_ptr() as usize;
        wr::<usize>(nb1, 0);
        wr::<u64>(nb1 + 0x08, 2);
        wr::<u16>(nb1 + 0x10, 1);
        wr::<u16>(nb1 + 0x12, 1);

        // Node C (u64s at +0x10): item 0 = 0 (skipped), item 1 = 0xaaaa_bbbb
        let mut node_c1 = [0u8; 0x30];
        let nc1 = node_c1.as_mut_ptr() as usize;
        wr::<usize>(nc1, 0);
        wr::<u64>(nc1 + 0x08, 2);
        wr::<u64>(nc1 + 0x10, 0);
        wr::<u64>(nc1 + 0x18, 0xaaaa_bbbb);

        let sorted_u32: [u32; 3] = [10, 20, 30];
        let map_u32: [u32; 3] = [0, 1, u32::MAX];
        let f28_arr: [u16; 8] = [0, 0, 1, 0, 0, 0, 1, 0]; // idx 1 has arch=1 at +4
        let f48_arr: [u32; 2] = [0x111, 0x222];

        let comp_offs: [u32; 2] = [0x40, 0x80];
        let mut f08_arr = [0u8; 0x60];
        let f08p = f08_arr.as_mut_ptr() as usize;
        wr::<u64>(f08p + 0x30 + 0x08, 0x5000);
        wr::<usize>(f08p + 0x30 + 0x18, comp_offs.as_ptr() as usize);

        let search_u16: [u16; 3] = [7, 1, 9];
        let mut f50_buf = vec![0u8; 0x18100];
        let f50p = f50_buf.as_mut_ptr() as usize;
        wr::<u64>(f50p + 0x20 + 0x18058, search_u16.as_ptr() as u64);
        wr::<u32>(f50p + 0x20 + 0x1806c, 3);

        let mut dispatch_tbl = [0u8; 0xa0];
        let dtp = dispatch_tbl.as_mut_ptr() as usize;
        wr::<usize>(dtp + 0x50 + 0x30, mock_cb as *const () as usize);
        let f58_holder: [usize; 1] = [dtp];

        let mut ctx = [0u8; 0x60];
        let cp = ctx.as_mut_ptr() as usize;
        wr::<usize>(cp + 0x08, f08p);
        wr::<u64>(cp + 0x10, 3);
        wr::<usize>(cp + 0x18, sorted_u32.as_ptr() as usize);
        wr::<usize>(cp + 0x28, f28_arr.as_ptr() as usize);
        wr::<usize>(cp + 0x38, map_u32.as_ptr() as usize);
        wr::<usize>(cp + 0x48, f48_arr.as_ptr() as usize);
        wr::<usize>(cp + 0x50, f50p);
        wr::<usize>(cp + 0x58, f58_holder.as_ptr() as usize);

        let p1: [u64; 8] = [
            cp as u64,
            2,
            na1 as u64,
            0,
            nb1 as u64,
            0,
            nc1 as u64,
            0,
        ];
        f(p1.as_ptr() as usize);
        CB_LOG
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1420bba90 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1420bcd80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(
        usize,
        u64,
        usize,
        u64,
        u64,
        usize,
        usize,
        u64,
        usize,
        usize,
        usize,
    ) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARENA_MEM: [u8; 256] = [0; 256];
    static mut ARENA_OFF: usize = 0;
    static mut JOB_NODE: [u8; 0x40] = [0; 0x40];
    static mut D4D0_SUM: u64 = 0;

    unsafe extern "win64" fn mock_11e0(_arena: u64, size: u64, _align: u32) -> usize {
        let off = ARENA_OFF;
        ARENA_OFF = off + ((size as usize + 15) & !15);
        (core::ptr::addr_of_mut!(ARENA_MEM) as usize) + off
    }
    unsafe extern "win64" fn mock_6d50(vec_ptr: usize, min_cap: u64) {
        wr::<u32>(vec_ptr + 0x0c, (min_cap as u32) + 4);
    }
    unsafe extern "win64" fn mock_7c80() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_5aa0(_pool: usize, out_hdl: usize) -> usize {
        let np = core::ptr::addr_of_mut!(JOB_NODE) as usize;
        wr::<[u8; 0x40]>(np, [0; 0x40]);
        wr::<u8>(np + 0x04, 1);
        wr::<usize>(out_hdl, np);
        wr::<u16>(out_hdl + 0x08, 1);
        wr::<u16>(out_hdl + 0x0a, 0);
        wr::<u8>(out_hdl + 0x0c, 0);
        out_hdl
    }
    unsafe extern "win64" fn mock_d4d0(ctx_ptr: usize, _ph: usize) {
        let a0 = rd::<u64>(ctx_ptr);
        let a1 = rd::<u64>(ctx_ptr + 0x08);
        let a3 = rd::<u64>(ctx_ptr + 0x18);
        let a10 = rd::<u64>(ctx_ptr + 0x50);
        let a11 = rd::<u64>(ctx_ptr + 0x58);
        D4D0_SUM = a0
            .wrapping_add(a1)
            .wrapping_add(a3)
            .wrapping_add(a10)
            .wrapping_add(a11);
    }
    unsafe extern "win64" fn mock_5680(_jsys: usize, hdl: usize) {
        let np = rd::<usize>(hdl);
        D4D0_SUM = D4D0_SUM.wrapping_add(rd::<u32>(np) as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_411f_11e0, mock_11e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_6d50, mock_6d50 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_5aa0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_420b_d4d0, mock_d4d0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4248_5680, mock_5680 as *const ())?;

    let g_2f8 = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let saved_2f8 = rd::<usize>(g_2f8);

    static mut SHARED_P3: [u64; 2] = [0; 2];
    let run = |f: F| -> ([u16; 2], [u64; 4], [u64; 4], [u64; 4], [u64; 4], [u32; 2], u64) {
        ARENA_MEM = [0; 256];
        ARENA_OFF = 0;
        JOB_NODE = [0; 0x40];
        D4D0_SUM = 0;

        let mut worker_arr = [0u8; 0x80];
        wr::<usize>(worker_arr.as_mut_ptr() as usize + 0x48, 0x30_0000);
        let mut jsys = [0u8; 0x20];
        wr::<usize>(jsys.as_mut_ptr() as usize + 0x10, worker_arr.as_mut_ptr() as usize);
        wr::<usize>(g_2f8, jsys.as_mut_ptr() as usize);

        let mut p3_words: [u16; 2] = [1, 1];
        SHARED_P3 = [2, p3_words.as_mut_ptr() as u64];

        // param_6: free-list has 1 entry at index 0 (next = u32::MAX, gen = 5), then 2nd entity pushes new at index 1
        let mut p6_slots: [u64; 4] = [(5u64 << 32) | (u32::MAX as u64), 0, 0, 0];
        let mut p6 = [0u8; 0x18];
        let p6p = p6.as_mut_ptr() as usize;
        wr::<usize>(p6p, p6_slots.as_mut_ptr() as usize);
        wr::<u32>(p6p + 0x08, 1);
        wr::<u32>(p6p + 0x0c, 1);
        wr::<u32>(p6p + 0x10, 0);

        let mut p7_slots: [u64; 4] = [0; 4];
        let p7: [usize; 1] = [p7_slots.as_mut_ptr() as usize];

        // param_9: 1 group entry (w = 7) that rewrites index 0 of p3_words to 7
        let p9_indices: [u32; 1] = [0];
        let p9_words: [u16; 1] = [7];
        let mut p9_entry = [0u8; 0x50];
        let p9ep = p9_entry.as_mut_ptr() as usize;
        wr::<usize>(p9ep + 0x20, p9_indices.as_ptr() as usize);
        wr::<u32>(p9ep + 0x28, 1);
        let mut p9 = [0u8; 0x30];
        let p9p = p9.as_mut_ptr() as usize;
        wr::<usize>(p9p + 0x08, p9ep);
        wr::<usize>(p9p + 0x20, p9_words.as_ptr() as usize);
        wr::<u32>(p9p + 0x28, 1);

        let mut block_10: [u64; 4] = [0; 4];
        let mut entries_10 = [0u8; 0x30];
        let e10p = entries_10.as_mut_ptr() as usize;
        wr::<usize>(e10p + 0x08, block_10.as_mut_ptr() as usize);
        wr::<u32>(e10p + 0x28, 1);
        let p10: [u64; 2] = [1, e10p as u64];

        let mut block_11: [u64; 4] = [0; 4];
        let mut entries_11 = [0u8; 0x60];
        let e11p = entries_11.as_mut_ptr() as usize;
        wr::<usize>(e11p + 0x30 + 0x08, block_11.as_mut_ptr() as usize);
        wr::<u32>(e11p + 0x30 + 0x28, 0);
        let p11: [u64; 2] = [2, e11p as u64];

        let mut out_p1 = [0u64; 3];
        let op1 = out_p1.as_mut_ptr() as usize;
        let ret = f(
            op1,
            0x9999,
            core::ptr::addr_of_mut!(SHARED_P3) as usize,
            0x1111,
            0x2222,
            p6p,
            p7.as_ptr() as usize,
            0x3000,
            p9p,
            p10.as_ptr() as usize,
            p11.as_ptr() as usize,
        );
        assert_eq!(ret, op1);
        let meta = [rd::<u32>(op1 + 0x10), rd::<u32>(op1 + 0x14)];
        (
            p3_words,
            p6_slots,
            p7_slots,
            block_10,
            block_11,
            meta,
            D4D0_SUM,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_2f8, saved_2f8);
    crate::diff_eq!(o, n, "1420bcd80 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1420c3bb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARENA_BUF: [u8; 1024] = [0; 1024];
    static mut ARENA_OFF: usize = 0;
    static mut BITSET_WORDS: [u64; 4] = [0; 4];
    static mut FREE_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_11e0(_arena: u64, size: u64, _align: u32) -> usize {
        let off = ARENA_OFF;
        ARENA_OFF = off + ((size as usize + 15) & !15);
        (core::ptr::addr_of_mut!(ARENA_BUF) as usize) + off
    }
    unsafe extern "win64" fn mock_a0c0(ptr: usize, count: u32) {
        let n = count as usize;
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n {
                let vi = rd::<u32>(ptr + i * 4);
                let vj = rd::<u32>(ptr + j * 4);
                if vj < vi {
                    wr::<u32>(ptr + i * 4, vj);
                    wr::<u32>(ptr + j * 4, vi);
                }
                j += 1;
            }
            i += 1;
        }
    }
    unsafe extern "win64" fn mock_e550(ptr: usize, count: u32) {
        let n = count as usize;
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n {
                let vi = rd::<u64>(ptr + i * 8);
                let vj = rd::<u64>(ptr + j * 8);
                if vj < vi {
                    wr::<u64>(ptr + i * 8, vj);
                    wr::<u64>(ptr + j * 8, vi);
                }
                j += 1;
            }
            i += 1;
        }
    }
    unsafe extern "win64" fn mock_2380(vec_ptr: usize, new_cap: u64, _elem_size: u32) {
        let old_ptr = rd::<usize>(vec_ptr + 0x08);
        let old_len = rd::<u32>(vec_ptr + 0x10) as usize;
        let bytes = (new_cap as usize) * 8;
        let off = ARENA_OFF;
        ARENA_OFF = off + ((bytes + 15) & !15);
        let new_ptr = (core::ptr::addr_of_mut!(ARENA_BUF) as usize) + off;
        for i in 0..old_len {
            wr::<u64>(new_ptr + i * 8, rd::<u64>(old_ptr + i * 8));
        }
        wr::<usize>(vec_ptr + 0x08, new_ptr);
        wr::<u32>(vec_ptr + 0x14, new_cap as u32);
    }
    unsafe extern "win64" fn mock_f200(vec_ptr: usize, num_words: u64) {
        BITSET_WORDS = [0; 4];
        wr::<usize>(vec_ptr, core::ptr::addr_of_mut!(BITSET_WORDS) as usize);
        wr::<u32>(vec_ptr + 0x08, num_words as u32);
        wr::<u32>(vec_ptr + 0x0c, 4);
    }
    unsafe extern "win64" fn mock_c190(ptr: usize) {
        if ptr == core::ptr::addr_of_mut!(BITSET_WORDS) as usize {
            FREE_COUNT += 1;
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_411f_11e0, mock_11e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4103_a0c0, mock_a0c0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_40fa_e550, mock_e550 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_411f_2380, mock_2380 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_410e_f200, mock_f200 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;

    let run = |f: F| -> (Vec<u64>, Vec<u64>, [u32; 2], u32, [u32; 2]) {
        ARENA_BUF = [0; 1024];
        ARENA_OFF = 0;
        BITSET_WORDS = [0; 4];
        FREE_COUNT = 0;

        // Chunk list at ctx0 + 0x30: 2 u32s [30, 10]
        let mut chunk_30 = [0u8; 0x20];
        let c30p = chunk_30.as_mut_ptr() as usize;
        wr::<usize>(c30p, 0);
        wr::<u64>(c30p + 0x08, 2);
        wr::<u32>(c30p + 0x10, 30);
        wr::<u32>(c30p + 0x14, 10);

        // Iterator 1 (ctx0 + 0x50, u32s: [10, 20, 20, 40]) and Iterator 2 (ctx0 + 0x68..0x70, u16s: [5, 6, 6, 7])
        let mut node1 = [0u8; 0x20];
        let n1p = node1.as_mut_ptr() as usize;
        wr::<usize>(n1p, 0);
        wr::<u64>(n1p + 0x08, 4);
        wr::<u32>(n1p + 0x10, 10);
        wr::<u32>(n1p + 0x14, 20);
        wr::<u32>(n1p + 0x18, 20); // duplicate (20, 6) to exercise std::unique!
        wr::<u32>(n1p + 0x1c, 40);

        let mut node2 = [0u8; 0x20];
        let n2p = node2.as_mut_ptr() as usize;
        wr::<usize>(n2p, 0);
        wr::<u64>(n2p + 0x08, 4);
        wr::<u16>(n2p + 0x10, 5);
        wr::<u16>(n2p + 0x12, 6);
        wr::<u16>(n2p + 0x14, 6);
        wr::<u16>(n2p + 0x16, 7);

        // Iterator 3 (ctx0 + 0x108, u16s: [6]) and Iterator 4 (ctx0 + 0xf0..0xf8, u32s: [20])
        // Marks (20, 6) as set in bitset; (10, 5) is also marked set by 0x1411f36a0 since 10 is in chunk_30;
        // (40, 7) remains unset and goes to vec_20!
        let mut node3 = [0u8; 0x20];
        let n3p = node3.as_mut_ptr() as usize;
        wr::<usize>(n3p, 0);
        wr::<u64>(n3p + 0x08, 1);
        wr::<u16>(n3p + 0x10, 6);

        let mut node4 = [0u8; 0x20];
        let n4p = node4.as_mut_ptr() as usize;
        wr::<usize>(n4p, 0);
        wr::<u64>(n4p + 0x08, 1);
        wr::<u32>(n4p + 0x10, 20);

        let mut ctx0 = [0u8; 0x120];
        let c0p = ctx0.as_mut_ptr() as usize;
        wr::<usize>(c0p + 0x30, c30p);
        wr::<u64>(c0p + 0x40, 2);
        wr::<usize>(c0p + 0x50, n1p);
        wr::<usize>(c0p + 0x68, n2p);
        wr::<usize>(c0p + 0x70, n2p);
        wr::<usize>(c0p + 0xf0, n4p);
        wr::<usize>(c0p + 0xf8, n4p);
        wr::<usize>(c0p + 0x108, n3p);

        // Arch lookup at param_1 + 0x10: map all entity IDs up to 40 to arch 0
        let arch_map = [0u64; 45];
        // Search array at [param_1 + 0x18] + 0x18050 (len at +0x18068): 18 u16s to exercise both AVX2 16-lane loop and scalar tail
        let mut arch_words = [99u16; 18];
        arch_words[2] = 5;
        arch_words[15] = 6;
        arch_words[17] = 7; // in scalar tail (>= 16)
        let mut base_18 = vec![0u8; 0x18080];
        let b18p = base_18.as_mut_ptr() as usize;
        wr::<usize>(b18p + 0x18050, arch_words.as_ptr() as usize);
        wr::<u32>(b18p + 0x18068, 18);

        let mut vec_20 = [0u8; 0x18];
        let v20p = vec_20.as_mut_ptr() as usize;
        let mut vec_28 = [0u8; 0x18];
        let v28p = vec_28.as_mut_ptr() as usize;
        let mut out_38 = [0u8; 0x18];
        let o38p = out_38.as_mut_ptr() as usize;

        let p1: [usize; 8] = [
            c0p,
            0,
            arch_map.as_ptr() as usize,
            b18p,
            v20p,
            v28p,
            0x7777,
            o38p,
        ];

        f(p1.as_ptr() as usize);

        let len_20 = rd::<u32>(v20p + 0x10) as usize;
        let ptr_20 = rd::<usize>(v20p + 0x08);
        let items_20: Vec<u64> = (0..len_20).map(|i| rd::<u64>(ptr_20 + i * 8)).collect();

        let len_28 = rd::<u32>(v28p + 0x10) as usize;
        let ptr_28 = rd::<usize>(v28p + 0x08);
        let items_28: Vec<u64> = (0..len_28).map(|i| rd::<u64>(ptr_28 + i * 8)).collect();

        let sorted_arr_ptr = rd::<usize>(o38p + 0x08);
        let sorted_arr = [rd::<u32>(sorted_arr_ptr), rd::<u32>(sorted_arr_ptr + 4)];
        let meta_38 = [rd::<u32>(o38p + 0x10), rd::<u32>(o38p + 0x14)];

        (items_20, items_28, sorted_arr, FREE_COUNT, meta_38)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1420c3bb0 out");
    Ok(1)
}

// REGISTRY:
// Replacement::verified(0x1_420b_2890, "r_1420b2890", rend::cauldron_hot_x4b::r_1420b2890 as *const (), rend::cauldron_hot_x4b::diff_r_1420b2890),
// Replacement::verified(0x1_420b_2a90, "r_1420b2a90", rend::cauldron_hot_x4b::r_1420b2a90 as *const (), rend::cauldron_hot_x4b::diff_r_1420b2a90),
// Replacement::verified_throws(0x1_420b_3cb0, "r_1420b3cb0", rend::cauldron_hot_x4b::r_1420b3cb0 as *const (), Throws::Caught, rend::cauldron_hot_x4b::diff_r_1420b3cb0),
// Replacement::verified(0x1_420b_ba90, "unknown_1420bba90", rend::cauldron_hot_x4b::unknown_1420bba90 as *const (), rend::cauldron_hot_x4b::diff_unknown_1420bba90),
// Replacement::verified(0x1_420b_cd80, "unknown_1420bcd80", rend::cauldron_hot_x4b::unknown_1420bcd80 as *const (), rend::cauldron_hot_x4b::diff_unknown_1420bcd80),
// Replacement::verified_throws(0x1_420c_3bb0, "unknown_1420c3bb0", rend::cauldron_hot_x4b::unknown_1420c3bb0 as *const (), Throws::Caught, rend::cauldron_hot_x4b::diff_unknown_1420c3bb0),
