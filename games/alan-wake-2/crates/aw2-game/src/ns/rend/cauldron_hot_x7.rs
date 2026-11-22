//! Decompiled hot functions shard X7 (`0x142420050..0x142502f50`).

use crate::exe::{self, rd, wr};

/// `r_142420050`
/// Original: 0x142420050 (2737 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_142420050(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    const MIX_MUL: u128 = 0xde5f_b9d2_6304_58e9;

    let mut map = [0u64; 6];
    let map_ptr = map.as_mut_ptr() as usize;
    wr::<usize>(map_ptr, exe::ptr::<u8>(0x1_4307_82a0) as usize);

    let init_cnt = rd::<u64>(param_3);
    let init_cap_req = ((init_cnt as i64).wrapping_sub(1) / 7).wrapping_add(init_cnt as i64) as u64;
    if init_cap_req != 0 {
        let cap_pow2 = exe::call_va!(0x1_401d_e350, (init_cap_req) -> u64);
        if cap_pow2 != 0 {
            exe::call_va!(0x1_4139_f000, (map_ptr, cap_pow2));
        }
    }
    let mut r14 = rd::<u64>(map_ptr + 0x18);
    let mut r15 = rd::<usize>(map_ptr + 0x08);

    let mut vec1 = [0u64; 2];
    let vec1_ptr = vec1.as_mut_ptr() as usize;
    let mut r12d: u32 = 0;
    let mut r13: usize = 0;
    if init_cnt != 0 {
        exe::call_va!(0x1_410f_5e80, (vec1_ptr, init_cnt));
        r12d = rd::<u32>(vec1_ptr + 0x08);
        r13 = rd::<usize>(vec1_ptr);
    }

    let mut cur_in = rd::<usize>(param_3 + 0x08);
    let end_in = cur_in.wrapping_add((init_cnt as usize).wrapping_mul(8));
    while cur_in != end_in {
        let k = rd::<u64>(cur_in);
        let mut kv: [u64; 2] = [k, 0];
        let prod = (k as u128).wrapping_mul(MIX_MUL);
        let hash = ((prod >> 64) as u64).wrapping_add(prod as u64);
        let mut res: [u64; 2] = [0; 2];
        exe::call_va!(
            0x1_413a_0c80,
            (
                map_ptr,
                res.as_mut_ptr() as usize,
                kv.as_mut_ptr() as usize,
                hash
            )
        );
        r15 = rd::<usize>(map_ptr + 0x08);
        if rd::<u8>((res.as_ptr() as usize) + 0x08) != 0 {
            let slot_idx = res[0] as usize;
            let dst = r15.wrapping_add(slot_idx * 16);
            wr::<u64>(dst, kv[0]);
            wr::<u64>(dst + 0x08, kv[1]);

            let cur_bytes = (r12d as usize).wrapping_mul(16).wrapping_add(r13);
            let cap_bytes = (rd::<u32>(vec1_ptr + 0x0c) as usize)
                .wrapping_mul(16)
                .wrapping_add(r13);
            if cur_bytes == cap_bytes {
                let new_cap = (((cur_bytes.wrapping_sub(r13) as isize) >> 4) + 1) as u64;
                exe::call_va!(0x1_410f_5e80, (vec1_ptr, new_cap));
                r12d = rd::<u32>(vec1_ptr + 0x08);
                r13 = rd::<usize>(vec1_ptr);
            }
            let elem_dst = r13.wrapping_add((r12d as usize) * 16);
            wr::<u64>(elem_dst, k);
            wr::<u64>(elem_dst + 0x08, 0);
            r12d = r12d.wrapping_add(1);
            wr::<u32>(vec1_ptr + 0x08, r12d);
        }
        r14 = rd::<u64>(map_ptr + 0x18);
        cur_in = cur_in.wrapping_add(8);
    }

    let mut bfs_idx: u64 = 0;
    let mut bfs_off: usize = 0;
    if (r12d as u64) != 0 {
        while bfs_idx < (r12d as u64) {
            let elem_ptr = r13.wrapping_add(bfs_off);
            let k = rd::<u64>(elem_ptr);
            let g_2b8 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82b8) as usize);
            let mgr = rd::<usize>(g_2b8 + 0x10);
            let arg_18 = rd::<u64>(g_2b8 + 0x18);
            let ctx = exe::call_va!(0x1_4242_0c10, (arg_18) -> usize);
            let node = exe::call_va!(0x1_4242_48a0, (mgr, k, ctx) -> usize);
            if node == 0 {
                let last_ptr = r13
                    .wrapping_add((r12d as usize).wrapping_mul(16))
                    .wrapping_sub(16);
                if elem_ptr != last_ptr {
                    wr::<u64>(elem_ptr, rd::<u64>(last_ptr));
                    wr::<u64>(elem_ptr + 0x08, rd::<u64>(last_ptr + 0x08));
                }
                r12d = r12d.wrapping_sub(1);
                wr::<u32>(vec1_ptr + 0x08, r12d);
            } else {
                wr::<usize>(elem_ptr + 0x08, node);
                let sub = rd::<usize>(node + 0x10);
                let mut dep_cur = rd::<usize>(sub + 0x18);
                let dep_cnt = rd::<u32>(sub + 0x20) as usize;
                let dep_end = dep_cur.wrapping_add(dep_cnt * 8);
                while dep_cur != dep_end {
                    let dep_k = rd::<u64>(dep_cur);
                    let mut kv2: [u64; 2] = [dep_k, 0];
                    let prod = (dep_k as u128).wrapping_mul(MIX_MUL);
                    let hash = ((prod >> 64) as u64).wrapping_add(prod as u64);
                    let mut res2: [u64; 2] = [0; 2];
                    exe::call_va!(
                        0x1_413a_0c80,
                        (
                            map_ptr,
                            res2.as_mut_ptr() as usize,
                            kv2.as_mut_ptr() as usize,
                            hash
                        )
                    );
                    r15 = rd::<usize>(map_ptr + 0x08);
                    if rd::<u8>((res2.as_ptr() as usize) + 0x08) != 0 {
                        let slot_idx = res2[0] as usize;
                        let dst = r15.wrapping_add(slot_idx * 16);
                        wr::<u64>(dst, kv2[0]);
                        wr::<u64>(dst + 0x08, kv2[1]);

                        let cur_bytes = (r12d as usize).wrapping_mul(16).wrapping_add(r13);
                        let cap_bytes = (rd::<u32>(vec1_ptr + 0x0c) as usize)
                            .wrapping_mul(16)
                            .wrapping_add(r13);
                        if cur_bytes == cap_bytes {
                            let new_cap =
                                (((cur_bytes.wrapping_sub(r13) as isize) >> 4) + 1) as u64;
                            exe::call_va!(0x1_410f_5e80, (vec1_ptr, new_cap));
                            r12d = rd::<u32>(vec1_ptr + 0x08);
                            r13 = rd::<usize>(vec1_ptr);
                        }
                        let elem_dst = r13.wrapping_add((r12d as usize) * 16);
                        wr::<u64>(elem_dst, dep_k);
                        wr::<u64>(elem_dst + 0x08, 0);
                        r12d = r12d.wrapping_add(1);
                        wr::<u32>(vec1_ptr + 0x08, r12d);
                    }
                    dep_cur = dep_cur.wrapping_add(8);
                }
                bfs_idx = bfs_idx.wrapping_add(1);
                bfs_off = bfs_off.wrapping_add(16);
            }
            r14 = rd::<u64>(map_ptr + 0x18);
        }
    }

    let count = r12d as usize;
    let mut sort_begin: usize = r13;
    let mut sort_end: usize = r13.wrapping_add(count * 16);
    exe::call_va!(
        0x1_4139_f7c0,
        (
            (&mut sort_begin as *mut usize) as usize,
            (&mut sort_end as *mut usize) as usize,
            count as u64,
            (param_2 & 0xff) as u8
        )
    );

    wr::<u128>(param_2, 0);
    if count != 0 {
        exe::call_va!(0x1_4062_4670, (param_2, count as u64));
    }

    let mut vec2_ptr: usize = 0;
    let mut vec2_len: u32 = 0;
    let mut vec2_cap: u64 = 0;
    let mut group_start: usize = 0;

    if count != 0 {
        while group_start < count {
            let first_node = rd::<usize>(r13.wrapping_add(group_start * 16 + 0x08));
            let key_3c = rd::<u32>(first_node + 0x3c);
            let mut group_end = group_start + 1;
            while group_end < count {
                let node_i = rd::<usize>(r13.wrapping_add(group_end * 16 + 0x08));
                if rd::<u32>(node_i + 0x3c) != key_3c {
                    break;
                }
                group_end += 1;
            }
            let group_len = group_end - group_start;
            let mut span: [u64; 2] = [
                group_len as u64,
                r13.wrapping_add(group_start * 16) as u64,
            ];
            let mut out_pair: [u64; 2] = [0; 2];
            exe::call_va!(
                0x1_4241_fdf0,
                (
                    param_1,
                    out_pair.as_mut_ptr() as usize,
                    key_3c,
                    span.as_mut_ptr() as usize,
                    param_2
                )
            );

            if (vec2_len as u64) * 16 == (vec2_cap as u32 as u64) * 16 {
                let old_bytes = (vec2_len as usize) * 16;
                let old_len = vec2_len as u64;
                let old_cap = vec2_cap as u32 as u64;
                let new_cap = (old_cap + (old_cap >> 1)).max(old_len + 1);
                let new_ptr = exe::call_va!(0x1_4241_c0a0, (new_cap * 16, 8u32) -> usize);
                let _ = exe::call_va!(0x1_4183_acc8, (new_ptr, vec2_ptr, old_bytes as u64) -> usize);
                if vec2_ptr != 0 {
                    exe::call_va!(0x1_4241_c190, (vec2_ptr));
                }
                vec2_ptr = new_ptr;
                vec2_cap = new_cap;
            }
            wr::<[u64; 2]>(vec2_ptr.wrapping_add((vec2_len as usize) * 16), out_pair);
            vec2_len = vec2_len.wrapping_add(1);

            for j in 0..group_len {
                let out_arr = rd::<usize>(param_2);
                let rdi = rd::<usize>(out_arr.wrapping_add((group_start + j) * 8));
                if rdi != 0 {
                    let rbx_k = rd::<u64>(rdi + 0x08);
                    let prod = (rbx_k as u128).wrapping_mul(MIX_MUL);
                    let r11_hash = ((prod >> 64) as u64).wrapping_add(prod as u64);
                    let mut r8_pos = (r11_hash >> 7) & r14;
                    let mut step: u64 = 0;
                    let xmm2 = crate::simd_ops::vpbroadcastb((r11_hash as u8) & 0x7f);
                    let slot_idx: u64 = loop {
                        let ctrl_base = rd::<usize>(map_ptr);
                        let xmm1 = rd::<u128>(ctrl_base.wrapping_add(r8_pos as usize));
                        let mut eq_mask =
                            crate::simd_ops::vpmovmskb(crate::simd_ops::vpcmpeqb(xmm1, xmm2))
                                as u32;
                        let mut found_slot: Option<u64> = None;
                        while eq_mask != 0 {
                            let bit = eq_mask.trailing_zeros() as u64;
                            let idx = r8_pos.wrapping_add(bit) & r14;
                            if rd::<u64>(r15.wrapping_add((idx as usize) * 16)) == rbx_k {
                                found_slot = Some(idx);
                                break;
                            }
                            eq_mask &= eq_mask.wrapping_sub(1);
                        }
                        if let Some(idx) = found_slot {
                            break idx;
                        }
                        let empty_mask =
                            crate::simd_ops::vpmovmskb(crate::simd_ops::vpsignb(xmm1, xmm1)) as u32;
                        if empty_mask != 0 {
                            let idx = exe::call_va!(0x1_413a_0fc0, (map_ptr, r11_hash) -> u64);
                            r15 = rd::<usize>(map_ptr + 0x08);
                            let dst = r15.wrapping_add((idx as usize) * 16);
                            wr::<u64>(dst, rbx_k);
                            wr::<u64>(dst + 0x08, 0);
                            r14 = rd::<u64>(map_ptr + 0x18);
                            break idx;
                        }
                        step = step.wrapping_add(0x10);
                        r8_pos = r8_pos.wrapping_add(step) & r14;
                    };
                    wr::<usize>(r15.wrapping_add((slot_idx as usize) * 16 + 0x08), rdi);
                }
            }

            group_start = group_end;
        }
    }

    let vec2_end = vec2_ptr.wrapping_add((vec2_len as usize) * 16);
    let mut cur_v2 = vec2_ptr;
    while cur_v2 != vec2_end {
        let mut idx_cur = rd::<usize>(cur_v2);
        let idx_end = rd::<usize>(cur_v2 + 0x08);
        while idx_cur != idx_end {
            let elem_rsi = rd::<usize>(rd::<usize>(param_2).wrapping_add(idx_cur * 8));
            let mut vec3_ptr: usize = 0;
            let mut vec3_len: u32 = 0;
            let mut vec3_cap: u64 = 0;

            let sub10_init = rd::<usize>(rd::<usize>(elem_rsi + 0x30) + 0x10);
            let init_cap = rd::<u32>(sub10_init + 0x20) as u64;
            if init_cap != 0 {
                vec3_ptr = exe::call_va!(0x1_4241_c0a0, (init_cap * 8, 8u32) -> usize);
                vec3_cap = init_cap;
            }

            let sub10 = rd::<usize>(rd::<usize>(elem_rsi + 0x30) + 0x10);
            let mut dep_cur = rd::<usize>(sub10 + 0x18);
            let dep_end = dep_cur.wrapping_add((rd::<u32>(sub10 + 0x20) as usize) * 8);
            if dep_cur != dep_end {
                let sentinel_ctrl = rd::<usize>(map_ptr).wrapping_add(r14 as usize);
                while dep_cur != dep_end {
                    let dep_k = rd::<u64>(dep_cur);
                    let mut found_val: u64 = 0;
                    let prod = (dep_k as u128).wrapping_mul(MIX_MUL);
                    let r8_hash = ((prod >> 64) as u64).wrapping_add(prod as u64);
                    let mut r9_pos = (r8_hash >> 7) & r14;
                    let mut step: u64 = 0;
                    let xmm2 = crate::simd_ops::vpbroadcastb((r8_hash as u8) & 0x7f);
                    loop {
                        let ctrl_base = rd::<usize>(map_ptr);
                        let xmm1 = rd::<u128>(ctrl_base.wrapping_add(r9_pos as usize));
                        let mut eq_mask =
                            crate::simd_ops::vpmovmskb(crate::simd_ops::vpcmpeqb(xmm1, xmm2))
                                as u32;
                        let mut matched_slot: Option<u64> = None;
                        while eq_mask != 0 {
                            let bit = eq_mask.trailing_zeros() as u64;
                            let idx = r9_pos.wrapping_add(bit) & r14;
                            if rd::<u64>(r15.wrapping_add((idx as usize) * 16)) == dep_k {
                                matched_slot = Some(idx);
                                break;
                            }
                            eq_mask &= eq_mask.wrapping_sub(1);
                        }
                        if let Some(idx) = matched_slot {
                            if rd::<usize>(map_ptr).wrapping_add(idx as usize) != sentinel_ctrl {
                                found_val = rd::<u64>(r15.wrapping_add((idx as usize) * 16 + 0x08));
                            }
                            break;
                        }
                        let empty_mask =
                            crate::simd_ops::vpmovmskb(crate::simd_ops::vpsignb(xmm1, xmm1)) as u32;
                        if empty_mask != 0 {
                            break;
                        }
                        step = step.wrapping_add(0x10);
                        r9_pos = r9_pos.wrapping_add(step) & r14;
                    }

                    if (vec3_len as u64) == (vec3_cap as u32 as u64) {
                        let old_bytes = (vec3_len as usize) * 8;
                        let old_len = vec3_len as u64;
                        let old_cap = vec3_cap as u32 as u64;
                        let new_cap = (old_cap + (old_cap >> 1)).max(old_len + 1);
                        let new_ptr = exe::call_va!(0x1_4241_c0a0, (new_cap * 8, 8u32) -> usize);
                        let _ = exe::call_va!(
                            0x1_4183_acc8,
                            (new_ptr, vec3_ptr, old_bytes as u64) -> usize
                        );
                        if vec3_ptr != 0 {
                            exe::call_va!(0x1_4241_c190, (vec3_ptr));
                        }
                        vec3_ptr = new_ptr;
                        vec3_cap = new_cap as u32 as u64;
                    }
                    wr::<u64>(vec3_ptr.wrapping_add((vec3_len as usize) * 8), found_val);
                    vec3_len = vec3_len.wrapping_add(1);
                    dep_cur = dep_cur.wrapping_add(8);
                }
            }

            let cnt = vec3_len as u64;
            if (rd::<u32>(elem_rsi + 0x1c) as u64) < cnt {
                exe::call_va!(0x1_401e_6d50, (elem_rsi + 0x10, cnt));
            }
            if cnt != 0 {
                for k in 0..(cnt as usize) {
                    let val = rd::<usize>(vec3_ptr.wrapping_add(k * 8));
                    if val != 0 {
                        let old_ref = crate::simd_ops::atomic_xadd_u32((val + 0x24) as u64, 1);
                        if old_ref == 0 {
                            let cb_ctx = rd::<usize>(val + 0x28);
                            if cb_ctx != 0 {
                                exe::call_va!(0x1_4241_d6f0, (cb_ctx, val));
                            }
                        }
                        let cur_len = rd::<u32>(elem_rsi + 0x18) as u64;
                        let cur_cap = rd::<u32>(elem_rsi + 0x1c) as u64;
                        if cur_len * 8 == cur_cap * 8 {
                            exe::call_va!(0x1_401e_6d50, (elem_rsi + 0x10, cur_len + 1));
                        }
                        let out_idx = rd::<u32>(elem_rsi + 0x18) as usize;
                        let out_arr = rd::<usize>(elem_rsi + 0x10);
                        wr::<usize>(out_arr.wrapping_add(out_idx * 8), val);
                        wr::<u32>(elem_rsi + 0x18, (out_idx as u32).wrapping_add(1));
                    }
                }
                r14 = rd::<u64>(map_ptr + 0x18);
                r15 = rd::<usize>(map_ptr + 0x08);
            }
            if vec3_ptr != 0 {
                exe::call_va!(0x1_4241_c190, (vec3_ptr));
            }
            idx_cur += 1;
        }
        cur_v2 = cur_v2.wrapping_add(16);
    }

    if vec2_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (vec2_ptr));
    }
    let vec1_final = rd::<usize>(vec1_ptr);
    if vec1_final != 0 {
        exe::call_va!(0x1_4241_c190, (vec1_final));
    }
    if r14 != 0 {
        let ctrl_final = rd::<usize>(map_ptr);
        if ctrl_final != 0 {
            exe::call_va!(0x1_4241_c190, (ctrl_final));
        }
    }
    param_2
}

/// `r_142423190`
/// Original: 0x142423190 (260 bytes, Throws::No)
pub unsafe extern "win64" fn r_142423190(param_1: usize, param_2: u64) {
    let deleted = rd::<u64>(param_1 + 0x30);
    let used = rd::<u64>(param_1 + 0x38);
    let cap = rd::<u64>(param_1 + 0x40);
    let buckets = rd::<usize>(param_1 + 0x60);

    let slot_ptr = if used == deleted {
        buckets.wrapping_add((cap as usize).wrapping_mul(0x18))
    } else {
        let empty_key = rd::<u64>(param_1 + 0x48);
        let mask = cap.wrapping_sub(1);
        let mut idx = param_2 & mask;
        let mut step: u64 = 0;
        loop {
            let k = rd::<u64>(buckets.wrapping_add((idx as usize).wrapping_mul(0x18)));
            if k == empty_key {
                idx = cap;
                break;
            }
            if deleted == 0 || rd::<u64>(param_1 + 0x28) != k {
                if k == param_2 {
                    if idx == u64::MAX {
                        idx = cap;
                    }
                    break;
                }
            }
            step = step.wrapping_add(1);
            idx = idx.wrapping_add(step) & mask;
        }
        buckets.wrapping_add((idx as usize).wrapping_mul(0x18))
    };

    let end_ptr = buckets.wrapping_add((rd::<usize>(param_1 + 0x40)).wrapping_mul(0x18));
    if slot_ptr != end_ptr {
        wr::<u64>(slot_ptr, rd::<u64>(param_1 + 0x28));
        wr::<u64>(slot_ptr + 0x08, 0);
        wr::<u64>(slot_ptr + 0x10, 0);
        wr::<u64>(param_1 + 0x30, rd::<u64>(param_1 + 0x30).wrapping_add(1));
        wr::<u8>(param_1 + 0x18, 1);
    }
}

/// `unknown_1424248a0`
/// Original: 0x1424248a0 (649 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1424248a0(
    param_1: usize,
    param_2: u64,
    param_3: usize,
) -> usize {
    let mut key_slot: u64 = param_2;
    exe::call_va!(0x1_4243_d770, (param_3));

    let mut buf_38 = [0u64; 3];
    let buf_38_ptr = buf_38.as_mut_ptr() as usize;
    exe::call_va!(
        0x1_413a_a840,
        (rd::<u64>(param_3 + 0x60), buf_38_ptr, param_2)
    );
    if rd::<u8>(buf_38_ptr + 0x10) == 0 {
        return 0;
    }

    let mut wait1: u64 = 100;
    loop {
        while rd::<u8>(param_1 + 0x08) != 0 {
            core::hint::spin_loop();
        }
        if crate::simd_ops::atomic_xchg_u8((param_1 + 0x08) as u64, 1) == 0 {
            break;
        }
        exe::call_va!(0x1_4241_7470, ((&mut wait1 as *mut u64) as usize));
    }

    let deleted = rd::<u64>(param_1 + 0x40);
    let used = rd::<u64>(param_1 + 0x48);
    if used != deleted {
        let cap = rd::<u64>(param_1 + 0x50);
        let mask = cap.wrapping_sub(1);
        let mut idx = key_slot & mask;
        let mut step: u64 = 0;
        let buckets = rd::<usize>(param_1 + 0x68);
        let empty_key = rd::<u64>(param_1 + 0x58);
        loop {
            let k = rd::<u64>(buckets.wrapping_add((idx as usize) * 16));
            if k == empty_key {
                break;
            }
            if deleted == 0 || rd::<u64>(param_1 + 0x38) != k {
                if k == key_slot {
                    if idx != u64::MAX {
                        let slot_ptr = buckets.wrapping_add((idx as usize) * 16);
                        let end_ptr =
                            buckets.wrapping_add((rd::<usize>(param_1 + 0x50)) * 16);
                        if slot_ptr != end_ptr {
                            let val = rd::<usize>(slot_ptr + 0x08);
                            crate::simd_ops::wu8((param_1 + 0x08) as u64, 0); // spinlock release (x86 mov = release)
                            return val;
                        }
                    }
                    break;
                }
            }
            step = step.wrapping_add(1);
            idx = idx.wrapping_add(step) & mask;
        }
    }

    crate::simd_ops::wu8((param_1 + 0x08) as u64, 0); // spinlock release (x86 mov = release)

    let mut buf_50 = [0u64; 4];
    let buf_50_ptr = buf_50.as_mut_ptr() as usize;
    exe::call_va!(0x1_4242_4570, (buf_50_ptr, param_3, key_slot));
    if rd::<u8>(buf_50_ptr + 0x18) == 0 {
        return 0;
    }

    let mut wait2: u64 = 100;
    loop {
        while rd::<u8>(param_1 + 0xb0) != 0 {
            core::hint::spin_loop();
        }
        if crate::simd_ops::atomic_xchg_u8((param_1 + 0xb0) as u64, 1) == 0 {
            break;
        }
        exe::call_va!(0x1_4241_7470, ((&mut wait2 as *mut u64) as usize));
    }

    let rsi = exe::call_va!(0x1_4243_b450, (param_1 + 0x70) -> usize);
    if rsi != 0 {
        wr::<u32>(rsi + 0x01, 0);
        wr::<u16>(rsi + 0x05, 0);
        wr::<u8>(rsi + 0x07, 0);
        wr::<u64>(rsi + 0x38, 0);
        wr::<u8>(rsi, 1);
        wr::<u64>(rsi + 0x08, 0);
        wr::<u64>(rsi + 0x10, 0);
        wr::<u64>(rsi + 0x18, 0);
        wr::<u64>(rsi + 0x20, 0);
        wr::<u64>(rsi + 0x28, 0);
        wr::<u64>(rsi + 0x30, 0);
    }
    crate::simd_ops::wu8((param_1 + 0xb0) as u64, 0); // spinlock release (x86 mov = release)

    wr::<u64>(rsi + 0x20, rd::<u64>(param_1));
    wr::<u128>(rsi + 0x08, rd::<u128>(buf_50_ptr));
    let obj_60 = rd::<usize>(buf_50_ptr + 0x10);
    wr::<usize>(rsi + 0x18, obj_60);
    let vtbl1 = rd::<usize>(obj_60);
    let sub_obj = exe::call_va!(rd::<usize>(vtbl1 + 0xa0), (obj_60) -> usize);
    let vtbl2 = rd::<usize>(sub_obj);
    let val_3c = exe::call_va!(rd::<usize>(vtbl2 + 0x08), (sub_obj) -> u32);
    wr::<u32>(rsi + 0x3c, val_3c);
    wr::<u128>(rsi + 0x28, rd::<u128>(buf_38_ptr));

    let mut wait3: u64 = 100;
    loop {
        while rd::<u8>(param_1 + 0x08) != 0 {
            core::hint::spin_loop();
        }
        if crate::simd_ops::atomic_xchg_u8((param_1 + 0x08) as u64, 1) == 0 {
            break;
        }
        exe::call_va!(0x1_4241_7470, ((&mut wait3 as *mut u64) as usize));
    }

    let slot = exe::call_va!(
        0x1_413a_2680,
        (param_1 + 0x10, (&mut key_slot as *mut u64) as usize) -> usize
    );
    wr::<usize>(slot + 0x08, rsi);
    crate::simd_ops::wu8((param_1 + 0x08) as u64, 0); // spinlock release (x86 mov = release)
    rsi
}

/// `unknown_1424250f0`
/// Original: 0x1424250f0 (368 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1424250f0(param_1: usize, param_2: usize) {
    if param_2 == 0 {
        return;
    }

    const MIX_MUL: u128 = 0xde5f_b9d2_6304_58e9;
    let r12 = rd::<u32>(param_1 + 0x18) as u64;
    let rbx = rd::<u64>(param_2 + 0x08);
    let prod = (rbx as u128).wrapping_mul(MIX_MUL);
    let r15 = ((prod >> 64) as u64).wrapping_add(prod as u64);
    let r10 = rd::<u64>(param_1 + 0x38);
    let mut r8 = (r15 >> 7) & r10;
    let r14 = rd::<usize>(param_1 + 0x20);
    let xmm2 = crate::simd_ops::vpbroadcastb((r15 as u8) & 0x7f);
    let mut r11: u64 = 0;

    let slot_idx: u64 = loop {
        let xmm1 = rd::<u128>(r14.wrapping_add(r8 as usize));
        let mut eq_mask =
            crate::simd_ops::vpmovmskb(crate::simd_ops::vpcmpeqb(xmm1, xmm2)) as u32;
        if eq_mask != 0 {
            let r9 = rd::<usize>(param_1 + 0x28);
            let mut found: Option<u64> = None;
            while eq_mask != 0 {
                let bit = eq_mask.trailing_zeros() as u64;
                let idx = r8.wrapping_add(bit) & r10;
                if rd::<u64>(r9.wrapping_add((idx as usize) * 16)) == rbx {
                    found = Some(idx);
                    break;
                }
                eq_mask &= eq_mask.wrapping_sub(1);
            }
            if let Some(idx) = found {
                break idx;
            }
        }
        let empty_mask =
            crate::simd_ops::vpmovmskb(crate::simd_ops::vpsignb(xmm1, xmm1)) as u32;
        if empty_mask != 0 {
            let idx = exe::call_va!(0x1_413a_2e30, (param_1 + 0x20, r15) -> u64);
            let slot_ptr = rd::<usize>(param_1 + 0x28).wrapping_add((idx as usize) * 16);
            wr::<u64>(slot_ptr, rbx);
            wr::<u64>(slot_ptr + 0x08, 0);
            break idx;
        }
        r11 = r11.wrapping_add(0x10);
        r8 = r8.wrapping_add(r11) & r10;
    };

    let slot_ptr = rd::<usize>(param_1 + 0x28).wrapping_add((slot_idx as usize) * 16);
    wr::<u64>(slot_ptr + 0x08, r12);

    let cur_len = rd::<u32>(param_1 + 0x18) as u64;
    let cur_cap = rd::<u32>(param_1 + 0x1c) as u64;
    if (cur_len << 4) == (cur_cap << 4) {
        exe::call_va!(0x1_401f_c320, (param_1 + 0x10, cur_len + 1));
    }

    let len2 = rd::<u32>(param_1 + 0x18) as usize;
    let arr = rd::<usize>(param_1 + 0x10);
    wr::<u128>(arr.wrapping_add(len2 * 16), 0);
    let new_len = (len2 as u32).wrapping_add(1);
    wr::<u32>(param_1 + 0x18, new_len);
    let arr2 = rd::<usize>(param_1 + 0x10);
    wr::<usize>(
        arr2.wrapping_add((new_len as usize) * 16).wrapping_sub(0x10),
        param_2,
    );

    if rd::<u32>(param_2 + 0x80) == 1 {
        let old_idx =
            crate::simd_ops::atomic_xadd_u32((param_1 + 0x04) as u64, 1) as i32 as isize;
        let arr3 = rd::<usize>(param_1 + 0x10);
        wr::<usize>(
            arr3.wrapping_add((old_idx as usize).wrapping_mul(16) + 0x08),
            param_2,
        );
    }

    let _ = crate::simd_ops::atomic_add_u32(param_1 as u64, 1);
}

/// `unknown_14243b450`
/// Original: 0x14243b450 (232 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14243b450(param_1: usize) -> usize {
    let g_8fa = exe::ptr::<u16>(0x1_4397_d8fa) as usize;
    let mut cur = param_1;
    loop {
        let _ = crate::simd_ops::atomic_add_u16(g_8fa as u64, 1);
        let mut head = rd::<u64>(cur + 0x18);
        loop {
            let node = (head & 0xffff_ffff_ffff) as usize;
            if node == 0 {
                break;
            }
            let next = rd::<u64>(node);
            let (ok, prev) = crate::simd_ops::atomic_cmpxchg_u64((cur + 0x18) as u64, head, next);
            if ok {
                let old_cnt = crate::simd_ops::atomic_xadd_u32(cur as u64, 1) as i32;
                let new_cnt = old_cnt.wrapping_add(1);
                let mut max_cnt = rd::<u32>(cur + 0x04) as i32;
                loop {
                    let desired = if new_cnt > max_cnt { new_cnt } else { max_cnt };
                    let (ok_m, prev_m) = crate::simd_ops::atomic_cmpxchg_u32(
                        (cur + 0x04) as u64,
                        max_cnt as u32,
                        desired as u32,
                    );
                    if ok_m {
                        break;
                    }
                    max_cnt = prev_m as i32;
                }
                return node;
            }
            head = prev;
        }
        if rd::<u32>(cur + 0x0c) == 0 {
            return 0;
        }
        let mut next_block = rd::<usize>(cur + 0x38);
        if next_block == 0 {
            let alloc_block = exe::call_va!(
                0x1_413a_a540,
                (
                    cur + 0x0c,
                    0usize,
                    0xffff_ffff_ffffu64,
                    0usize,
                    cur + 0x0c,
                    cur + 0x10,
                    cur + 0x0c
                ) -> usize
            );
            let (ok, prev) = crate::simd_ops::atomic_cmpxchg_u64(
                (cur + 0x38) as u64,
                0,
                alloc_block as u64,
            );
            if ok {
                next_block = alloc_block;
            } else {
                next_block = prev as usize;
                if alloc_block != 0 {
                    exe::call_va!(0x1_4243_b540, (alloc_block));
                    exe::call_va!(0x1_4244_2de0, (alloc_block));
                }
            }
        }
        cur = next_block;
    }
}

/// `r_14243fd50`
/// Original: 0x14243fd50 (386 bytes, Throws::No)
pub unsafe extern "win64" fn r_14243fd50(
    param_1: usize,
    param_2: u64,
    param_3: u32,
    param_4: usize,
    param_5: usize,
) -> u64 {
    exe::call_iat!(0x1_4264_55a8, (param_1));
    let mut count_out: u64 = 0;

    let deleted = rd::<u64>(param_1 + 0x38);
    let used = rd::<u64>(param_1 + 0x40);
    if used != deleted {
        let cap = rd::<u64>(param_1 + 0x48);
        let mask = cap.wrapping_sub(1);
        let buckets = rd::<usize>(param_1 + 0x60);
        let empty_key = rd::<u64>(param_1 + 0x50);
        let mut idx = param_2 & mask;
        let mut step: u64 = 0;
        loop {
            let k = rd::<u64>(buckets.wrapping_add((idx as usize) * 16));
            if k == empty_key {
                break;
            }
            if deleted == 0 || rd::<u64>(param_1 + 0x30) != k {
                if k == param_2 {
                    if idx != u64::MAX {
                        let slot_ptr = buckets.wrapping_add((idx as usize) * 16);
                        let end_ptr =
                            buckets.wrapping_add((rd::<usize>(param_1 + 0x48)) * 16);
                        if slot_ptr != end_ptr && rd::<u32>(slot_ptr + 0x08) >= param_3 {
                            let val_64 = rd::<u64>(slot_ptr + 0x08);
                            let p4_vec = rd::<[u64; 2]>(param_4);
                            let p5_vec = rd::<[u64; 2]>(param_5);
                            let mut node_idx = (val_64 >> 32) as i32 as isize;
                            if node_idx >= 0 {
                                let max_out = p4_vec[0];
                                let out_arr = p5_vec[1] as usize;
                                while count_out < max_out {
                                    let filter_ptr = rd::<usize>(param_4 + 0x08);
                                    let filter_len = rd::<usize>(param_4);
                                    let nodes_base = rd::<usize>(param_1 + 0x68);
                                    let node_ptr =
                                        nodes_base.wrapping_add((node_idx as usize) * 16);
                                    let filter_end = filter_ptr.wrapping_add(filter_len * 4);
                                    let mut cur_f = filter_ptr;
                                    let mut out_pos: usize = 0;
                                    if cur_f != filter_end {
                                        let node_type = rd::<u32>(node_ptr + 0x08);
                                        loop {
                                            let f_val = rd::<u32>(cur_f);
                                            if f_val == node_type || f_val == 8 {
                                                let node_val = rd::<u64>(node_ptr);
                                                count_out = count_out.wrapping_add(1);
                                                wr::<u64>(
                                                    out_arr.wrapping_add(out_pos * 8),
                                                    node_val,
                                                );
                                                break;
                                            }
                                            out_pos = out_pos.wrapping_add(1);
                                            cur_f = cur_f.wrapping_add(4);
                                            if cur_f == filter_end {
                                                break;
                                            }
                                        }
                                    }
                                    node_idx = rd::<i32>(node_ptr + 0x0c) as isize;
                                    if node_idx < 0 {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    break;
                }
            }
            step = step.wrapping_add(1);
            idx = idx.wrapping_add(step) & mask;
        }
    }

    exe::call_iat!(0x1_4264_5598, (param_1));
    count_out
}

/// `r_142476fc0`
/// Original: 0x142476fc0 (744 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_142476fc0(param_1: usize, param_2: usize, param_3: usize) {
    let mode = rd::<u32>(param_3 + 0x10);
    if mode != 0 {
        if mode == 1 {
            let mut tmp_p2 = rd::<[u64; 2]>(param_2);
            exe::call_va!(
                0x1_4247_8340,
                (rd::<usize>(param_1 + 0x18), tmp_p2.as_mut_ptr() as usize, param_3)
            );
        }
        return;
    }

    let tid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let rdx_slot = rd::<usize>(param_1 + 0x08).wrapping_add(tid.wrapping_mul(0x98));
    let kind = rd::<u32>(param_3 + 0x0c);
    let idx_r8: usize = if kind == 1 {
        2
    } else if kind == 1000 {
        0
    } else {
        1
    };

    let inner_18 = rd::<usize>(rd::<usize>(param_1 + 0x18));
    if rd::<u8>(rdx_slot + 0x90) != 0 {
        let rbx_entry = rdx_slot.wrapping_add(idx_r8 * 0x30);
        if rd::<u64>(rbx_entry) == 0 {
            let mut tmp_out = [0u64; 2];
            let vtbl = rd::<usize>(inner_18);
            let ret_ptr = exe::call_va!(
                rd::<usize>(vtbl + 0x148),
                (inner_18, tmp_out.as_mut_ptr() as usize) -> usize
            );
            wr::<u128>(rbx_entry, rd::<u128>(ret_ptr));
        }
        let mut tmp_p2 = rd::<[u64; 2]>(param_2);
        exe::call_va!(
            0x1_4247_79d0,
            (
                rd::<usize>(param_1 + 0x18),
                tmp_p2.as_mut_ptr() as usize,
                param_3,
                rbx_entry
            )
        );
        return;
    }

    let mut ctx_30 = [0u64; 6];
    let ctx_ptr = ctx_30.as_mut_ptr() as usize;
    let vtbl = rd::<usize>(inner_18);
    let _ = exe::call_va!(rd::<usize>(vtbl + 0x148), (inner_18, ctx_ptr) -> usize);
    wr::<[u64; 4]>(ctx_ptr + 0x10, [0; 4]);

    let mut tmp_p2 = rd::<[u64; 2]>(param_2);
    exe::call_va!(
        0x1_4247_79d0,
        (
            rd::<usize>(param_1 + 0x18),
            tmp_p2.as_mut_ptr() as usize,
            param_3,
            ctx_ptr
        )
    );
    let inner2 = rd::<usize>(rd::<usize>(param_1 + 0x18));
    exe::call_va!(0x1_4247_8f30, (inner2, ctx_ptr));

    let vec_b_len = rd::<u32>(ctx_ptr + 0x28) as usize;
    let mut vec_b_ptr = rd::<usize>(ctx_ptr + 0x20);
    let vec_b_end = vec_b_ptr.wrapping_add(vec_b_len.wrapping_mul(0x70));
    if vec_b_ptr != vec_b_end {
        let mut rbx = vec_b_ptr.wrapping_add(0x58);
        loop {
            let rdx_arg = rd::<u64>(rbx - 0x18);
            if rdx_arg != 0 {
                let rcx_obj = rd::<usize>(rbx);
                let r9_arg = rd::<u16>(rbx - 0x08) as u32;
                let r8_arg = rd::<u64>(rbx - 0x10);
                let obj_vtbl = rd::<usize>(rcx_obj);
                exe::call_va!(
                    rd::<usize>(obj_vtbl + 0x10),
                    (rcx_obj, rdx_arg, r8_arg, r9_arg)
                );
            }
            exe::call_va!(0x1_4240_ab10, (rbx - 0x48));
            rbx = rbx.wrapping_add(0x70);
            if rbx.wrapping_sub(0x58) == vec_b_end {
                break;
            }
        }
        vec_b_ptr = rd::<usize>(ctx_ptr + 0x20);
    }
    if vec_b_ptr != 0 {
        let r10 = vec_b_ptr.wrapping_sub(1) & !0x01ff_ffffusize;
        let same_thread = exe::teb() == rd::<usize>(r10 + 0x100);
        let r9 = rd::<usize>(ctx_ptr + 0x20);
        let page_idx = r9.wrapping_sub(r10) >> 16;
        let hdr_raw = r10.wrapping_add(0x108).wrapping_add(page_idx.wrapping_mul(0x50));
        let rdx_hdr = hdr_raw.wrapping_sub(rd::<u32>(hdr_raw + 0x04) as usize);
        if same_thread && rd::<u8>(rdx_hdr + 0x0e) == 0 {
            wr::<u64>(r9, rd::<u64>(rdx_hdr + 0x20));
            wr::<usize>(rdx_hdr + 0x20, r9);
            let new_cnt = rd::<u32>(rdx_hdr + 0x18).wrapping_sub(1);
            wr::<u32>(rdx_hdr + 0x18, new_cnt);
            if new_cnt == 0 {
                exe::call_va!(0x1_4253_71a0, (rdx_hdr));
            }
        } else {
            let flag: u8 = if same_thread { 1 } else { 0 };
            exe::call_va!(0x1_4253_3200, (r10, rdx_hdr, flag, r9));
        }
    }
    wr::<u64>(ctx_ptr + 0x20, 0);
    wr::<u64>(ctx_ptr + 0x28, 0);

    let vec_a_len = rd::<u32>(ctx_ptr + 0x18) as usize;
    let mut vec_a_ptr = rd::<usize>(ctx_ptr + 0x10);
    let vec_a_end = vec_a_ptr.wrapping_add(vec_a_len.wrapping_mul(0x40));
    if vec_a_ptr != vec_a_end {
        let mut rbx = vec_a_ptr.wrapping_add(0x28);
        loop {
            let rdx_arg = rd::<u64>(rbx - 0x18);
            if rdx_arg != 0 {
                let rcx_obj = rd::<usize>(rbx);
                let r9_arg = rd::<u16>(rbx - 0x08) as u32;
                let r8_arg = rd::<u64>(rbx - 0x10);
                let obj_vtbl = rd::<usize>(rcx_obj);
                exe::call_va!(
                    rd::<usize>(obj_vtbl + 0x10),
                    (rcx_obj, rdx_arg, r8_arg, r9_arg)
                );
            }
            rbx = rbx.wrapping_add(0x40);
            if rbx.wrapping_sub(0x28) == vec_a_end {
                break;
            }
        }
        vec_a_ptr = rd::<usize>(ctx_ptr + 0x10);
    }
    if vec_a_ptr != 0 {
        let r10 = vec_a_ptr.wrapping_sub(1) & !0x01ff_ffffusize;
        let same_thread = exe::teb() == rd::<usize>(r10 + 0x100);
        let r9 = rd::<usize>(ctx_ptr + 0x10);
        let page_idx = r9.wrapping_sub(r10) >> 16;
        let hdr_raw = r10.wrapping_add(0x108).wrapping_add(page_idx.wrapping_mul(0x50));
        let rdx_hdr = hdr_raw.wrapping_sub(rd::<u32>(hdr_raw + 0x04) as usize);
        if same_thread && rd::<u8>(rdx_hdr + 0x0e) == 0 {
            wr::<u64>(r9, rd::<u64>(rdx_hdr + 0x20));
            wr::<usize>(rdx_hdr + 0x20, r9);
            let new_cnt = rd::<u32>(rdx_hdr + 0x18).wrapping_sub(1);
            wr::<u32>(rdx_hdr + 0x18, new_cnt);
            if new_cnt == 0 {
                exe::call_va!(0x1_4253_71a0, (rdx_hdr));
            }
        } else {
            let flag: u8 = if same_thread { 1 } else { 0 };
            exe::call_va!(0x1_4253_3200, (r10, rdx_hdr, flag, r9));
        }
    }
}

/// `r_1424772b0`
/// Original: 0x1424772b0 (92 bytes, Throws::Discard)
pub unsafe extern "win64" fn r_1424772b0(param_1: usize, param_2: usize) -> u8 {
    let mut buf_20 = rd::<[u64; 2]>(param_2);
    let mut out_30 = [0u64; 3];
    let out_ptr = out_30.as_mut_ptr() as usize;
    exe::call_va!(
        0x1_4247_8780,
        (
            out_ptr,
            rd::<usize>(param_1 + 0x18),
            buf_20.as_mut_ptr() as usize
        )
    );
    if rd::<u8>(out_ptr + 0x10) != 0 {
        buf_20[0] = out_30[0];
        buf_20[1] = out_30[1];
        let inner = rd::<usize>(param_1 + 0x18);
        let fn_ptr = rd::<usize>(inner + 0x08);
        let res = exe::call_va!(fn_ptr, (buf_20.as_mut_ptr() as usize) -> u8);
        if res == 0 {
            return 1;
        }
    }
    0
}

/// `unknown_14248b000`
/// Original: 0x14248b000 (155 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14248b000(param_1: usize, param_2: usize) -> u64 {
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32) as u64;
    if rd::<u64>(param_1 + 0x68) == tid {
        let _ = crate::simd_ops::atomic_add_u64((param_1 + 0x60) as u64, 1);
    } else {
        let _ = exe::call_va!(0x1_4137_9040, (param_1 + 0x60, 0usize, 1u64) -> u64);
    }

    let slot_idx: u64 = if rd::<u64>(param_1 + 0x38) == rd::<u64>(param_1 + 0x30) {
        rd::<u64>(param_1 + 0x40)
    } else {
        let mut probe = [0u64; 2];
        exe::call_va!(0x1_401f_a730, (param_1, probe.as_mut_ptr() as usize, param_2));
        probe[0]
    };

    let mut ret_val: u64 = 0;
    if slot_idx != u64::MAX {
        let buckets = rd::<usize>(param_1 + 0x58);
        let end_ptr = buckets.wrapping_add((rd::<usize>(param_1 + 0x40)).wrapping_mul(16));
        let slot_ptr = buckets.wrapping_add((slot_idx as usize).wrapping_mul(16));
        if slot_ptr != end_ptr {
            ret_val = rd::<u64>(slot_ptr + 0x08);
        }
    }

    let _ = crate::simd_ops::atomic_sub_u64((param_1 + 0x60) as u64, 1);
    ret_val
}

/// `unknown_1424afd60`
/// Original: 0x1424afd60 (184 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1424afd60(
    param_1: u64,
    param_2: usize,
    param_3: usize,
) -> u64 {
    let mut cur_val = param_1;
    let mut cur_ti = param_2;
    let mut parent_ti = exe::call_va!(rd::<usize>(rd::<usize>(cur_ti) + 0x20), (cur_ti) -> usize);
    let mut cur_id = exe::call_va!(rd::<usize>(rd::<usize>(cur_ti) + 0x08), (cur_ti) -> u32);
    let mut parent_id =
        exe::call_va!(rd::<usize>(rd::<usize>(parent_ti) + 0x08), (parent_ti) -> u32);

    while cur_id != parent_id {
        let target_id = exe::call_va!(rd::<usize>(rd::<usize>(param_3) + 0x08), (param_3) -> u32);
        if cur_id == target_id {
            return cur_val;
        }
        cur_val =
            exe::call_va!(rd::<usize>(rd::<usize>(cur_ti) + 0x1a8), (cur_ti, cur_val) -> u64);
        cur_ti = parent_ti;
        parent_ti = exe::call_va!(rd::<usize>(rd::<usize>(cur_ti) + 0x20), (cur_ti) -> usize);
        cur_id = exe::call_va!(rd::<usize>(rd::<usize>(cur_ti) + 0x08), (cur_ti) -> u32);
        parent_id = exe::call_va!(rd::<usize>(rd::<usize>(parent_ti) + 0x08), (parent_ti) -> u32);
    }
    0
}

/// `snd_1424d0890`
/// Original: 0x1424d0890 (212 bytes, Throws::No)
pub unsafe extern "win64" fn snd_1424d0890(param_1: usize) -> u32 {
    let g_9b8 = exe::ptr::<u32>(0x1_4387_19b8) as usize;
    wr::<u32>(param_1 + 0xd8, rd::<u32>(g_9b8));
    let mode = rd::<u32>(g_9b8);
    let bank_id: u32 = if mode == 1 {
        let mut id: u32 = 0;
        let _ = exe::call_va!(
            0x1_4158_28e0,
            (
                rd::<u64>(param_1 + 0xb8),
                rd::<u32>(param_1 + 0xc0),
                (&mut id as *mut u32) as usize
            ) -> u32
        );
        id
    } else if mode == 2 {
        let mut id: u32 = 0;
        let _ = exe::call_va!(
            0x1_4158_2530,
            (
                rd::<u64>(param_1 + 0xb8),
                rd::<u32>(param_1 + 0xc0),
                (&mut id as *mut u32) as usize
            ) -> u32
        );
        let ptr_b8 = rd::<u64>(param_1 + 0xb8);
        if ptr_b8 != 0 {
            exe::call_va!(0x1_4244_3a50, (ptr_b8));
        }
        wr::<u64>(param_1 + 0xb8, 0);
        wr::<u64>(param_1 + 0xc8, 0);
        id
    } else {
        let mut id: u32 = 0;
        let _ = exe::call_va!(
            0x1_4158_28e0,
            (
                rd::<u64>(param_1 + 0xb8),
                rd::<u32>(param_1 + 0xc0),
                (&mut id as *mut u32) as usize
            ) -> u32
        );
        id
    };

    wr::<u32>(param_1 + 0xd0, bank_id);
    crate::simd_ops::atomic_xchg_u32((param_1 + 0x80) as u64, 4)
}

/// `ui_142502f50`
/// Original: 0x142502f50 (555 bytes, Throws::Discard)
pub unsafe extern "win64" fn ui_142502f50(param_1: usize) -> u64 {
    let mut key_slot: u32 = rd::<u32>(param_1 + 0x18);
    let rbx_10 = rd::<usize>(param_1 + 0x10);
    let sp1 = exe::call_va!(
        0x1_4141_0810,
        (param_1, (&mut key_slot as *mut u32) as usize) -> usize
    );
    let rsi_78 = rd::<usize>(rbx_10 + 0x78);
    let mgr1 = rd::<usize>(rd::<usize>(sp1) + 0x28);
    let ctx1 = exe::call_va!(rd::<usize>(rd::<usize>(mgr1) + 0x218), (mgr1) -> usize);
    if ctx1 != 0 {
        exe::call_va!(rd::<usize>(rd::<usize>(ctx1) + 0x60), (ctx1));
        exe::call_va!(rd::<usize>(rd::<usize>(ctx1) + 0x28), (ctx1));
        if rsi_78 != 0 {
            exe::call_va!(rd::<usize>(rd::<usize>(ctx1) + 0x28), (ctx1));
            exe::call_va!(0x1_424e_4de0, (ctx1, rsi_78));
            exe::call_va!(rd::<usize>(rd::<usize>(ctx1) + 0x48), (ctx1, rsi_78));
        } else {
            exe::call_va!(0x1_424e_4de0, (ctx1, 0usize));
        }
        exe::call_va!(rd::<usize>(rd::<usize>(ctx1) + 0x48), (ctx1, rsi_78));
        let s_vid = exe::ptr::<u8>(0x1_4341_6288) as usize;
        exe::call_va!(rd::<usize>(rd::<usize>(ctx1) + 0x68), (ctx1, s_vid, rsi_78));
    }

    let id2 = rd::<i32>(param_1 + 0x18);
    let mgr2 = if id2 != -1 {
        key_slot = id2 as u32;
        let sp2 = exe::call_va!(
            0x1_4141_0810,
            (0usize, (&mut key_slot as *mut u32) as usize) -> usize
        );
        rd::<usize>(rd::<usize>(sp2) + 0x28)
    } else {
        0
    };
    let ctx2 = exe::call_va!(rd::<usize>(rd::<usize>(mgr2) + 0x218), (mgr2) -> usize);
    if ctx2 != 0 {
        exe::call_va!(rd::<usize>(rd::<usize>(ctx2) + 0x60), (ctx2));
        exe::call_va!(rd::<usize>(rd::<usize>(ctx2) + 0x28), (ctx2));
        if param_1 != 0 {
            exe::call_va!(rd::<usize>(rd::<usize>(ctx2) + 0x28), (ctx2));
            exe::call_va!(0x1_4250_1640, (ctx2, param_1));
            exe::call_va!(rd::<usize>(rd::<usize>(ctx2) + 0x48), (ctx2, param_1));
        } else {
            exe::call_va!(0x1_4250_1640, (ctx2, 0usize));
        }
        exe::call_va!(rd::<usize>(rd::<usize>(ctx2) + 0x48), (ctx2, param_1));
        let s_page = exe::ptr::<u8>(0x1_432a_5cec) as usize;
        exe::call_va!(rd::<usize>(rd::<usize>(ctx2) + 0x68), (ctx2, s_page, param_1));
    }

    let id3 = rd::<i32>(param_1 + 0x18);
    let mgr3 = if id3 != -1 {
        key_slot = id3 as u32;
        let sp3 = exe::call_va!(
            0x1_4141_0810,
            (0usize, (&mut key_slot as *mut u32) as usize) -> usize
        );
        rd::<usize>(rd::<usize>(sp3) + 0x28)
    } else {
        0
    };
    let r14_c0 = param_1.wrapping_add(0xc0);
    let fn_240 = rd::<usize>(rd::<usize>(mgr3) + 0x240);
    let ctx3 = exe::call_va!(rd::<usize>(rd::<usize>(mgr3) + 0x218), (mgr3) -> usize);
    let mut rbp_val: u64 = 0;
    if ctx3 != 0 {
        let saved_78 = exe::call_va!(rd::<usize>(rd::<usize>(ctx3) + 0x78), (ctx3) -> u32);
        exe::call_va!(rd::<usize>(rd::<usize>(ctx3) + 0x70), (ctx3, 2u32));
        exe::call_va!(rd::<usize>(rd::<usize>(ctx3) + 0x28), (ctx3));
        if r14_c0 != 0 {
            exe::call_va!(rd::<usize>(rd::<usize>(ctx3) + 0x28), (ctx3));
            exe::call_va!(0x1_4250_15d0, (ctx3, r14_c0));
            exe::call_va!(rd::<usize>(rd::<usize>(ctx3) + 0x48), (ctx3, r14_c0));
        } else {
            exe::call_va!(0x1_4250_15d0, (ctx3, 0usize));
        }
        exe::call_va!(rd::<usize>(rd::<usize>(ctx3) + 0x48), (ctx3, r14_c0));
        exe::call_va!(rd::<usize>(rd::<usize>(ctx3) + 0x70), (ctx3, saved_78));
        rbp_val = exe::call_va!(rd::<usize>(rd::<usize>(ctx3) + 0x40), (ctx3) -> u64);
    }
    let s_page_ctx = exe::ptr::<u8>(0x1_4341_9ac0) as usize;
    let cb_dd40 = exe::ptr::<u8>(0x1_4145_dd40) as usize as u64;
    exe::call_va!(fn_240, (mgr3, s_page_ctx, r14_c0, rbp_val, cb_dd40));
    exe::call_va!(rd::<usize>(rd::<usize>(param_1) + 0xd0), (param_1) -> u64)
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_r_142420050(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut MAP_CTRL: [u8; 32] = [0x80; 32];
    static mut MAP_SLOTS: [u64; 32] = [0; 32];
    static mut VEC1_BUF: [u64; 16] = [0; 16];
    static mut HEAP_ARENA: [u64; 64] = [0; 64];
    static mut HEAP_OFF: usize = 0;
    static mut FREE_COUNT: u32 = 0;
    static mut CB_D6F0_COUNT: u32 = 0;

    static mut NODE_10: [u8; 0x40] = [0; 0x40];
    static mut NODE_20: [u8; 0x40] = [0; 0x40];
    static mut SUB_10: [u8; 0x30] = [0; 0x30];
    static mut SUB_20: [u8; 0x30] = [0; 0x30];
    static mut DEPS_10: [u64; 1] = [20];

    static mut ELEM_0: [u8; 0x40] = [0; 0x40];
    static mut ELEM_1: [u8; 0x40] = [0; 0x40];
    static mut WRAP_0: [usize; 3] = [0; 3];
    static mut WRAP_1: [usize; 3] = [0; 3];
    static mut OUT_ELEMS: [usize; 2] = [0; 2];
    static mut ELEM0_DEPS_OUT: [usize; 4] = [0; 4];
    static mut ELEM1_DEPS_OUT: [usize; 4] = [0; 4];

    unsafe extern "win64" fn mock_e350(_req: u64) -> u64 {
        16
    }
    unsafe extern "win64" fn mock_f000(map_ptr: usize, _cap: u64) {
        wr::<usize>(map_ptr, core::ptr::addr_of_mut!(MAP_CTRL) as usize);
        wr::<usize>(map_ptr + 0x08, core::ptr::addr_of_mut!(MAP_SLOTS) as usize);
        wr::<u64>(map_ptr + 0x18, 15);
    }
    unsafe extern "win64" fn mock_5e80(vec_ptr: usize, _new_cap: u64) {
        wr::<usize>(vec_ptr, core::ptr::addr_of_mut!(VEC1_BUF) as usize);
        wr::<u32>(vec_ptr + 0x0c, 8);
    }
    unsafe extern "win64" fn mock_0c80(
        map_ptr: usize,
        res_ptr: usize,
        kv_ptr: usize,
        hash: u64,
    ) {
        let mask = rd::<u64>(map_ptr + 0x18);
        let ctrl = rd::<usize>(map_ptr);
        let slots = rd::<usize>(map_ptr + 0x08);
        let k = rd::<u64>(kv_ptr);
        let h2 = (hash as u8) & 0x7f;
        let mut idx = ((hash >> 7) & mask) as usize;
        loop {
            let c = rd::<u8>(ctrl + idx);
            if c == h2 && rd::<u64>(slots + idx * 16) == k {
                wr::<u64>(res_ptr, idx as u64);
                wr::<u8>(res_ptr + 0x08, 0);
                return;
            }
            if c == 0x80 {
                wr::<u8>(ctrl + idx, h2);
                wr::<u8>(ctrl + 16 + idx, h2);
                wr::<u64>(res_ptr, idx as u64);
                wr::<u8>(res_ptr + 0x08, 1);
                return;
            }
            idx = (idx + 1) & (mask as usize);
        }
    }
    unsafe extern "win64" fn mock_0fc0(map_ptr: usize, hash: u64) -> u64 {
        let mask = rd::<u64>(map_ptr + 0x18);
        let ctrl = rd::<usize>(map_ptr);
        let h2 = (hash as u8) & 0x7f;
        let mut idx = ((hash >> 7) & mask) as usize;
        while rd::<u8>(ctrl + idx) != 0x80 {
            idx = (idx + 1) & (mask as usize);
        }
        wr::<u8>(ctrl + idx, h2);
        wr::<u8>(ctrl + 16 + idx, h2);
        idx as u64
    }
    unsafe extern "win64" fn mock_0c10(arg: u64) -> usize {
        (arg as usize).wrapping_add(1)
    }
    unsafe extern "win64" fn mock_48a0(_mgr: usize, key: u64, _ctx: usize) -> usize {
        match key {
            10 => core::ptr::addr_of_mut!(NODE_10) as usize,
            20 => core::ptr::addr_of_mut!(NODE_20) as usize,
            _ => 0,
        }
    }
    unsafe extern "win64" fn mock_f7c0(
        _begin_ptr: usize,
        _end_ptr: usize,
        _cnt: u64,
        _cmp: u8,
    ) {
    }
    unsafe extern "win64" fn mock_4670(param_2: usize, _cnt: u64) {
        wr::<usize>(param_2, core::ptr::addr_of_mut!(OUT_ELEMS) as usize);
    }
    unsafe extern "win64" fn mock_fdf0(
        _p1: usize,
        out_pair: usize,
        _key_3c: u32,
        span_ptr: usize,
        param_2: usize,
    ) {
        let len = rd::<u64>(span_ptr);
        wr::<u64>(out_pair, 0);
        wr::<u64>(out_pair + 0x08, len);
        let arr = rd::<usize>(param_2);
        wr::<usize>(arr, core::ptr::addr_of_mut!(ELEM_0) as usize);
        wr::<usize>(arr + 0x08, core::ptr::addr_of_mut!(ELEM_1) as usize);
    }
    unsafe extern "win64" fn mock_c0a0(bytes: u64, _align: u32) -> usize {
        let words = ((bytes as usize) + 7) / 8;
        let base = (core::ptr::addr_of_mut!(HEAP_ARENA) as *mut u64).add(HEAP_OFF) as usize;
        HEAP_OFF += words;
        base
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {
        FREE_COUNT += 1;
    }
    unsafe extern "win64" fn mock_6d50(vec_field: usize, new_cap: u64) {
        wr::<u32>(vec_field + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_d6f0(_cb_ctx: usize, _val: usize) {
        CB_D6F0_COUNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_401d_e350, mock_e350 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4139_f000, mock_f000 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_410f_5e80, mock_5e80 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_413a_0c80, mock_0c80 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_413a_0fc0, mock_0fc0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4242_0c10, mock_0c10 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4242_48a0, mock_48a0 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4139_f7c0, mock_f7c0 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4062_4670, mock_4670 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4241_fdf0, mock_fdf0 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_401e_6d50, mock_6d50 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_4241_d6f0, mock_d6f0 as *const ())?;

    let g_2b8 = exe::ptr::<usize>(0x1_43a0_82b8) as usize;
    let s_2b8 = rd::<usize>(g_2b8);

    let run = |f: F| -> (u32, u32, u32, u32, usize, usize) {
        MAP_CTRL = [0x80; 32];
        MAP_SLOTS = [0; 32];
        VEC1_BUF = [0; 16];
        HEAP_ARENA = [0; 64];
        HEAP_OFF = 0;
        FREE_COUNT = 0;
        CB_D6F0_COUNT = 0;
        NODE_10 = [0; 0x40];
        NODE_20 = [0; 0x40];
        SUB_10 = [0; 0x30];
        SUB_20 = [0; 0x30];
        DEPS_10 = [20];
        ELEM_0 = [0; 0x40];
        ELEM_1 = [0; 0x40];
        OUT_ELEMS = [0; 2];
        ELEM0_DEPS_OUT = [0; 4];
        ELEM1_DEPS_OUT = [0; 4];

        let s10 = core::ptr::addr_of_mut!(SUB_10) as usize;
        wr::<usize>(s10 + 0x18, core::ptr::addr_of!(DEPS_10) as usize);
        wr::<u32>(s10 + 0x20, 1);

        let s20 = core::ptr::addr_of_mut!(SUB_20) as usize;
        wr::<usize>(s20 + 0x18, 0);
        wr::<u32>(s20 + 0x20, 0);

        let n10 = core::ptr::addr_of_mut!(NODE_10) as usize;
        wr::<usize>(n10 + 0x10, s10);
        wr::<u32>(n10 + 0x3c, 7);

        let n20 = core::ptr::addr_of_mut!(NODE_20) as usize;
        wr::<usize>(n20 + 0x10, s20);
        wr::<u32>(n20 + 0x3c, 7);

        WRAP_0 = [0, 0, s10];
        WRAP_1 = [0, 0, s20];

        let e0 = core::ptr::addr_of_mut!(ELEM_0) as usize;
        wr::<u64>(e0 + 0x08, 10);
        wr::<usize>(e0 + 0x10, core::ptr::addr_of_mut!(ELEM0_DEPS_OUT) as usize);
        wr::<u32>(e0 + 0x18, 0);
        wr::<u32>(e0 + 0x1c, 0);
        wr::<u32>(e0 + 0x24, 0);
        wr::<usize>(e0 + 0x28, 0x9000);
        wr::<usize>(e0 + 0x30, core::ptr::addr_of!(WRAP_0) as usize);

        let e1 = core::ptr::addr_of_mut!(ELEM_1) as usize;
        wr::<u64>(e1 + 0x08, 20);
        wr::<usize>(e1 + 0x10, core::ptr::addr_of_mut!(ELEM1_DEPS_OUT) as usize);
        wr::<u32>(e1 + 0x18, 0);
        wr::<u32>(e1 + 0x1c, 0);
        wr::<u32>(e1 + 0x24, 0);
        wr::<usize>(e1 + 0x28, 0x9001);
        wr::<usize>(e1 + 0x30, core::ptr::addr_of!(WRAP_1) as usize);

        let g_obj: [u64; 4] = [0, 0, 0x1111, 0x2222];
        wr::<usize>(g_2b8, g_obj.as_ptr() as usize);

        let in_keys: [u64; 2] = [30, 10]; // 30 fails 48a0 (swap-removed), 10 succeeds & pulls in 20
        let p3: [u64; 2] = [2, in_keys.as_ptr() as u64];
        let mut p2: [u64; 2] = [0; 2];

        let ret = f(0x5555, p2.as_mut_ptr() as usize, p3.as_ptr() as usize);
        assert_eq!(ret, p2.as_ptr() as usize);

        (
            FREE_COUNT,
            CB_D6F0_COUNT,
            rd::<u32>(e0 + 0x18),
            rd::<u32>(e1 + 0x24),
            ELEM0_DEPS_OUT[0],
            ELEM1_DEPS_OUT[0],
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_2b8, s_2b8);

    crate::diff_eq!(o, n, "142420050 out");
    Ok(1)
}

pub unsafe fn diff_r_142423190(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, key: u64, empty_table: bool| -> ([u64; 14], [u64; 12]) {
        let mut buckets: [u64; 12] = [
            0xffff_fffe, 0, 0, // slot 0: tombstone
            1, 0x111, 0x222, // slot 1: key 1
            5, 0x555, 0x666, // slot 2: key 5 (probed from 5 & 3 = 1 -> 1+1 = 2)
            0xffff_ffff, 0, 0, // slot 3: empty
        ];
        let mut tbl = [0u64; 14];
        let tp = tbl.as_mut_ptr() as usize;
        wr::<u8>(tp + 0x18, 0);
        wr::<u64>(tp + 0x28, 0xffff_fffe); // tombstone key
        wr::<u64>(tp + 0x30, 1); // deleted = 1
        wr::<u64>(tp + 0x38, if empty_table { 1 } else { 3 }); // used
        wr::<u64>(tp + 0x40, 4); // cap = 4
        wr::<u64>(tp + 0x48, 0xffff_ffff); // empty key
        wr::<usize>(tp + 0x60, buckets.as_mut_ptr() as usize);

        f(tp, key);
        tbl[12] = 0; // clear pointer at +0x60
        (tbl, buckets)
    };

    for (k, empty) in [(5u64, false), (9u64, false), (5u64, true)] {
        let o = run(f_o, k, empty);
        let n = run(f_n, k, empty);
        crate::diff_eq!(o, n, "142423190 k={} empty={}", k, empty);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1424248a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ALLOC_NODE: [u8; 0x40] = [0; 0x40];
    static mut SUB_VTBL: [usize; 2] = [0; 2];
    static mut SUB_OBJ: [usize; 1] = [0; 1];
    static mut OBJ_VTBL: [usize; 22] = [0; 22];
    static mut OBJ_60: [usize; 1] = [0; 1];
    static mut INSERT_SLOT: [u64; 2] = [0; 2];
    static mut D770_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_d770(_p3: usize) {
        D770_COUNT += 1;
    }
    unsafe extern "win64" fn mock_a840(rcx_60: u64, out_buf: usize, key: u64) {
        wr::<u64>(out_buf, rcx_60.wrapping_add(key));
        wr::<u64>(out_buf + 0x08, 0xaaaa_bbbb);
        wr::<u8>(out_buf + 0x10, if key == 999 { 0 } else { 1 });
    }
    unsafe extern "win64" fn mock_4570(out_buf: usize, _p3: usize, key: u64) {
        wr::<u64>(out_buf, 0x1111_2222u64.wrapping_add(key));
        wr::<u64>(out_buf + 0x08, 0x3333_4444);
        wr::<usize>(out_buf + 0x10, core::ptr::addr_of_mut!(OBJ_60) as usize);
        wr::<u8>(out_buf + 0x18, 1);
    }
    unsafe extern "win64" fn mock_b450(_pool: usize) -> usize {
        core::ptr::addr_of_mut!(ALLOC_NODE) as usize
    }
    unsafe extern "win64" fn mock_get_sub(_this: usize) -> usize {
        core::ptr::addr_of_mut!(SUB_OBJ) as usize
    }
    unsafe extern "win64" fn mock_get_id(_this: usize) -> u32 {
        0x4242
    }
    unsafe extern "win64" fn mock_2680(_map: usize, key_ptr: usize) -> usize {
        INSERT_SLOT[0] = rd::<u64>(key_ptr);
        core::ptr::addr_of_mut!(INSERT_SLOT) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_4243_d770, mock_d770 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_413a_a840, mock_a840 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4242_4570, mock_4570 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4243_b450, mock_b450 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_413a_2680, mock_2680 as *const ())?;

    let run = |f: F, key: u64| -> (usize, [u8; 0x40], [u64; 2], u32) {
        ALLOC_NODE = [0xff; 0x40];
        INSERT_SLOT = [0; 2];
        D770_COUNT = 0;

        SUB_VTBL[1] = mock_get_id as *const () as usize;
        SUB_OBJ[0] = core::ptr::addr_of!(SUB_VTBL) as usize;
        OBJ_VTBL[0xa0 / 8] = mock_get_sub as *const () as usize;
        OBJ_60[0] = core::ptr::addr_of!(OBJ_VTBL) as usize;

        let buckets: [u64; 8] = [
            1, 0x7777_8888, // slot 0: key 1 (cached)
            0xffff_ffff, 0, // slot 1: empty
            0xffff_ffff, 0, // slot 2: empty
            0xffff_ffff, 0, // slot 3: empty
        ];
        let mut p1 = [0u8; 0xc0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p, 0x9999_0000);
        wr::<u64>(p1p + 0x38, 0xffff_fffe); // tombstone
        wr::<u64>(p1p + 0x40, 0); // deleted
        wr::<u64>(p1p + 0x48, 1); // used
        wr::<u64>(p1p + 0x50, 4); // cap
        wr::<u64>(p1p + 0x58, 0xffff_ffff); // empty
        wr::<usize>(p1p + 0x68, buckets.as_ptr() as usize);

        let mut p3 = [0u8; 0x70];
        wr::<u64>(p3.as_mut_ptr() as usize + 0x60, 0x5000);

        let ret = f(p1p, key, p3.as_ptr() as usize);
        let mut node_snap = ALLOC_NODE;
        // Clear pointer field at +0x18 if populated
        wr::<usize>(node_snap.as_mut_ptr() as usize + 0x18, 0);
        (ret, node_snap, INSERT_SLOT, D770_COUNT)
    };

    for k in [1u64, 2u64, 999u64] {
        let o = run(f_o, k);
        let n = run(f_n, k);
        crate::diff_eq!(o, n, "1424248a0 k={}", k);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1424250f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut E30_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_2e30(map_ptr: usize, hash: u64) -> u64 {
        E30_CALLS += 1;
        let mask = rd::<u64>(map_ptr + 0x18);
        let ctrl = rd::<usize>(map_ptr);
        let idx = ((hash >> 7) & mask) as usize;
        wr::<u8>(ctrl + idx, (hash as u8) & 0x7f);
        idx as u64
    }
    unsafe extern "win64" fn mock_c320(vec_ptr: usize, new_cap: u64) {
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_413a_2e30, mock_2e30 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401f_c320, mock_c320 as *const ())?;

    let run = |f: F, is_null: bool, flag_80: u32| -> ([u32; 8], [u64; 8], [usize; 8], u32) {
        E30_CALLS = 0;
        let mut ctrl = [0x80u8; 32];
        let mut slots = [0u64; 32];
        let mut vec_buf = [0usize; 8];

        let mut p2 = [0u8; 0x90];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u64>(p2p + 0x08, 0x1234_5678);
        wr::<u32>(p2p + 0x80, flag_80);

        let mut p1 = [0u8; 0x48];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1p, 10);
        wr::<u32>(p1p + 0x04, 1);
        wr::<usize>(p1p + 0x10, vec_buf.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x18, 1);
        wr::<u32>(p1p + 0x1c, 1);
        wr::<usize>(p1p + 0x20, ctrl.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x28, slots.as_mut_ptr() as usize);
        wr::<u64>(p1p + 0x38, 15);

        f(p1p, if is_null { 0 } else { p2p });
        for elem in &mut vec_buf {
            if *elem == p2p {
                *elem = 0xaaaa;
            }
        }
        let hdr = [
            rd::<u32>(p1p),
            rd::<u32>(p1p + 0x04),
            rd::<u32>(p1p + 0x18),
            rd::<u32>(p1p + 0x1c),
            0,
            0,
            0,
            0,
        ];
        let mut top_slots = [0u64; 8];
        for i in 0..8 {
            if slots[i * 2] != 0 {
                top_slots[0] = slots[i * 2];
                top_slots[1] = slots[i * 2 + 1];
            }
        }
        (hdr, top_slots, vec_buf, E30_CALLS)
    };

    for (is_null, f80) in [(true, 0u32), (false, 1), (false, 0)] {
        let o = run(f_o, is_null, f80);
        let n = run(f_n, is_null, f80);
        crate::diff_eq!(o, n, "1424250f0 null={} f80={}", is_null, f80);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_14243b450(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut BLOCK2: [u64; 8] = [0; 8];
    static mut NODE_A: [u64; 2] = [0; 2];
    static mut NODE_B: [u64; 2] = [0; 2];
    static mut A540_ARGS: [u32; 2] = [0; 2];

    unsafe extern "win64" fn mock_a540(
        _a1: usize,
        _a2: usize,
        _a3: u64,
        _a4: usize,
        arg5_c: usize,
        arg6_10: usize,
        arg7_c: usize,
    ) -> usize {
        A540_ARGS[0] = rd::<u32>(arg5_c).wrapping_add(rd::<u32>(arg7_c));
        A540_ARGS[1] = rd::<u32>(arg6_10);
        core::ptr::addr_of_mut!(BLOCK2) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_413a_a540, mock_a540 as *const ())?;

    let g_8fa = exe::ptr::<u16>(0x1_4397_d8fa) as usize;
    let s_8fa = rd::<u16>(g_8fa);

    let run = |f: F, case_id: u32| -> (usize, [u32; 2], [u32; 2], u16) {
        wr::<u16>(g_8fa, 10);
        BLOCK2 = [0; 8];
        NODE_A = [0x1234, 0];
        NODE_B = [0x5678, 0];
        A540_ARGS = [0; 2];

        let mut b1 = [0u64; 8];
        let b1p = b1.as_mut_ptr() as usize;
        wr::<u32>(b1p, 2); // cur_cnt = 2
        wr::<u32>(b1p + 0x04, 1); // max_cnt = 1 (will be updated to 3)
        wr::<u32>(b1p + 0x0c, if case_id == 1 { 0 } else { 7 });
        wr::<u64>(b1p + 0x10, 64);

        if case_id == 0 {
            wr::<usize>(b1p + 0x18, core::ptr::addr_of_mut!(NODE_A) as usize);
        } else if case_id == 2 {
            let b2p = core::ptr::addr_of_mut!(BLOCK2) as usize;
            wr::<usize>(b2p + 0x18, core::ptr::addr_of_mut!(NODE_B) as usize);
        }

        let ret = f(b1p);
        let cnts = [rd::<u32>(b1p), rd::<u32>(b1p + 0x04)];
        (ret, cnts, A540_ARGS, rd::<u16>(g_8fa))
    };

    for case_id in [0u32, 1, 2] {
        let o = run(f_o, case_id);
        let n = run(f_n, case_id);
        wr::<u16>(g_8fa, s_8fa);
        crate::diff_eq!(o, n, "14243b450 case={}", case_id);
    }
    Ok(3)
}

pub unsafe fn diff_r_14243fd50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u32, usize, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, min_ver: u32| -> (u64, [u64; 4]) {
        // 4 buckets of 16 bytes
        let buckets: [u64; 8] = [
            0xffff_ffff,
            0,
            5,
            (0u64 << 32) | 10, // key = 5, lo = 10, head_node_idx = 0
            0xffff_ffff,
            0,
            0xffff_ffff,
            0,
        ];
        // 2 nodes of 16 bytes: (val: u64, node_type: u32, next_idx: i32)
        let mut nodes = [0u8; 32];
        let np = nodes.as_mut_ptr() as usize;
        wr::<u64>(np, 0xaaaa_1111);
        wr::<u32>(np + 0x08, 3);
        wr::<i32>(np + 0x0c, 1);

        wr::<u64>(np + 0x10, 0xbbbb_2222);
        wr::<u32>(np + 0x18, 99);
        wr::<i32>(np + 0x1c, -1);

        let mut p1 = [0u8; 0x70];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x30, 0xffff_fffe); // tombstone
        wr::<u64>(p1p + 0x38, 0); // deleted
        wr::<u64>(p1p + 0x40, 1); // used
        wr::<u64>(p1p + 0x48, 4); // cap
        wr::<u64>(p1p + 0x50, 0xffff_ffff); // empty
        wr::<usize>(p1p + 0x60, buckets.as_ptr() as usize);
        wr::<usize>(p1p + 0x68, np);

        let filters: [u32; 3] = [1, 3, 8]; // slot 1 matches type 3, slot 2 matches wildcard 8
        let p4: [u64; 2] = [3, filters.as_ptr() as u64];
        let mut out_arr = [0u64; 4];
        let p5: [u64; 2] = [4, out_arr.as_mut_ptr() as u64];

        let cnt = f(p1p, 5, min_ver, p4.as_ptr() as usize, p5.as_ptr() as usize);
        (cnt, out_arr)
    };

    for min_ver in [10u32, 11u32] {
        let o = run(f_o, min_ver);
        let n = run(f_n, min_ver);
        crate::diff_eq!(o, n, "14243fd50 min_ver={}", min_ver);
    }
    Ok(2)
}

pub unsafe fn diff_r_142476fc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_SUM: u64 = 0;
    static mut CTX_PTR_SAVE: usize = 0;
    static mut VEC_A_ELEM: [u8; 0x40] = [0; 0x40];
    static mut VEC_B_ELEM: [u8; 0x70] = [0; 0x70];
    static mut CB_VTBL: [usize; 4] = [0; 4];
    static mut CB_OBJ: [usize; 1] = [0; 1];

    unsafe extern "win64" fn mock_8340(rcx: usize, rdx_pair: usize, p3: usize) {
        LOG_SUM = LOG_SUM
            .wrapping_add(rcx as u64)
            .wrapping_add(rd::<u64>(rdx_pair))
            .wrapping_add(rd::<u32>(p3 + 0x10) as u64);
    }
    unsafe extern "win64" fn mock_7c80() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_vfunc_148(_this: usize, out_ptr: usize) -> usize {
        wr::<u64>(out_ptr, 0x1111_2222);
        wr::<u64>(out_ptr + 0x08, 0x3333_4444);
        out_ptr
    }
    unsafe extern "win64" fn mock_79d0(_rcx: usize, rdx_pair: usize, p3: usize, ctx_ptr: usize) {
        LOG_SUM = LOG_SUM
            .wrapping_add(rd::<u64>(rdx_pair))
            .wrapping_add(rd::<u64>(ctx_ptr));
        if rd::<u32>(p3 + 0x0c) == 5 {
            CTX_PTR_SAVE = ctx_ptr;
            let va = core::ptr::addr_of_mut!(VEC_A_ELEM) as usize;
            wr::<u64>(va + 0x10, 0x300);
            wr::<u64>(va + 0x18, 0x400);
            wr::<u16>(va + 0x20, 4);
            wr::<usize>(va + 0x28, core::ptr::addr_of_mut!(CB_OBJ) as usize);

            let vb = core::ptr::addr_of_mut!(VEC_B_ELEM) as usize;
            wr::<u64>(vb + 0x40, 0x100);
            wr::<u64>(vb + 0x48, 0x200);
            wr::<u16>(vb + 0x50, 3);
            wr::<usize>(vb + 0x58, core::ptr::addr_of_mut!(CB_OBJ) as usize);

            wr::<usize>(ctx_ptr + 0x10, va);
            wr::<u32>(ctx_ptr + 0x18, 1);
            wr::<usize>(ctx_ptr + 0x20, vb);
            wr::<u32>(ctx_ptr + 0x28, 1);
        }
    }
    unsafe extern "win64" fn mock_8f30(_inner: usize, _ctx_ptr: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(0x500);
    }
    unsafe extern "win64" fn mock_cb_10(_this: usize, rdx: u64, r8: u64, r9: u32) {
        LOG_SUM = LOG_SUM
            .wrapping_add(rdx)
            .wrapping_add(r8)
            .wrapping_add(r9 as u64);
        if rdx == 0x300 {
            wr::<usize>(CTX_PTR_SAVE + 0x10, 0);
        }
    }
    unsafe extern "win64" fn mock_ab10(_ptr: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(0x77);
        wr::<usize>(CTX_PTR_SAVE + 0x20, 0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4247_8340, mock_8340 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4247_79d0, mock_79d0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4247_8f30, mock_8f30 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let run = |f: F, mode_10: u32, kind_0c: u32, cached_flag: u8| -> (u64, [u64; 2]) {
        LOG_SUM = 0;
        CB_VTBL[2] = mock_cb_10 as *const () as usize;
        CB_OBJ[0] = core::ptr::addr_of!(CB_VTBL) as usize;

        let mut inner_vtbl = [0usize; 44];
        inner_vtbl[0x148 / 8] = mock_vfunc_148 as *const () as usize;
        let inner_obj: [usize; 1] = [inner_vtbl.as_ptr() as usize];
        let outer_18: [usize; 1] = [inner_obj.as_ptr() as usize];

        let mut slot_98 = [0u8; 0x98];
        let sp = slot_98.as_mut_ptr() as usize;
        wr::<u8>(sp + 0x90, cached_flag);

        let mut p1 = [0u8; 0x20];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x08, sp);
        wr::<usize>(p1p + 0x18, outer_18.as_ptr() as usize);

        let p2: [u64; 2] = [0xabcd_0001, 0xabcd_0002];
        let mut p3 = [0u8; 0x18];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<u32>(p3p + 0x0c, kind_0c);
        wr::<u32>(p3p + 0x10, mode_10);

        f(p1p, p2.as_ptr() as usize, p3p);
        let entry_idx = if kind_0c == 1 {
            2
        } else if kind_0c == 1000 {
            0
        } else {
            1
        };
        let entry = rd::<[u64; 2]>(sp + entry_idx * 0x30);
        // NormalizeLOG_SUM if mode_10 == 1 (since rcx was outer_18.as_ptr())
        let norm_sum = if mode_10 == 1 {
            LOG_SUM.wrapping_sub(outer_18.as_ptr() as usize as u64)
        } else {
            LOG_SUM
        };
        (norm_sum, entry)
    };

    for (m10, k0c, cflag) in [(1u32, 0u32, 0u8), (0, 1, 1), (0, 1000, 1), (0, 5, 0)] {
        let o = run(f_o, m10, k0c, cflag);
        let n = run(f_n, m10, k0c, cflag);
        crate::diff_eq!(o, n, "142476fc0 m10={} k0c={} cflag={}", m10, k0c, cflag);
    }
    Ok(4)
}

pub unsafe fn diff_r_1424772b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FOUND_FLAG: u8 = 1;
    static mut CB_RET: u8 = 0;

    unsafe extern "win64" fn mock_8780(out_ptr: usize, _inner: usize, in_ptr: usize) -> usize {
        wr::<u64>(out_ptr, rd::<u64>(in_ptr).wrapping_add(10));
        wr::<u64>(out_ptr + 0x08, rd::<u64>(in_ptr + 0x08).wrapping_add(20));
        wr::<u8>(out_ptr + 0x10, FOUND_FLAG);
        out_ptr
    }
    unsafe extern "win64" fn mock_cb(pair_ptr: usize) -> u8 {
        if rd::<u64>(pair_ptr) == 110 && rd::<u64>(pair_ptr + 0x08) == 220 {
            CB_RET
        } else {
            1
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4247_8780, mock_8780 as *const ())?;

    let run = |f: F, found: u8, cb_ret: u8| -> u8 {
        FOUND_FLAG = found;
        CB_RET = cb_ret;
        let inner: [usize; 2] = [0, mock_cb as *const () as usize];
        let mut p1 = [0u8; 0x20];
        wr::<usize>(p1.as_mut_ptr() as usize + 0x18, inner.as_ptr() as usize);
        let p2: [u64; 2] = [100, 200];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize)
    };

    for (found, cb_ret) in [(1u8, 0u8), (1, 1), (0, 0)] {
        let o = run(f_o, found, cb_ret);
        let n = run(f_n, found, cb_ret);
        crate::diff_eq!(o, n, "1424772b0 found={} cb={}", found, cb_ret);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_14248b000(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOCK_9040_R8: u64 = 0;

    unsafe extern "win64" fn mock_9040(lock_ptr: usize, _rdx: usize, r8: u64) -> u64 {
        LOCK_9040_R8 = r8;
        let old = rd::<u64>(lock_ptr);
        wr::<u64>(lock_ptr, old.wrapping_add(r8));
        old
    }
    unsafe extern "win64" fn mock_a730(_p1: usize, out_probe: usize, key_ptr: usize) {
        let k = rd::<u64>(key_ptr);
        if k == 42 {
            wr::<u64>(out_probe, 1);
            wr::<u64>(out_probe + 0x08, 0);
        } else {
            wr::<u64>(out_probe, u64::MAX);
            wr::<u64>(out_probe + 0x08, 0);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4137_9040, mock_9040 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401f_a730, mock_a730 as *const ())?;

    let cur_tid = exe::call_iat!(0x1_4264_5790, () -> u32) as u64;

    let run = |f: F, same_tid: bool, key: u64| -> (u64, u64, u64) {
        LOCK_9040_R8 = 0;
        let buckets: [u64; 4] = [0, 0, 42, 0xdead_beef_cafe];
        let mut p1 = [0u8; 0x70];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x30, 0); // deleted
        wr::<u64>(p1p + 0x38, 1); // used
        wr::<u64>(p1p + 0x40, 2); // cap
        wr::<usize>(p1p + 0x58, buckets.as_ptr() as usize);
        wr::<u64>(p1p + 0x60, 5); // lock count
        wr::<u64>(p1p + 0x68, if same_tid { cur_tid } else { cur_tid ^ 0x1234 });

        let ret = f(p1p, (&key as *const u64) as usize);
        (ret, rd::<u64>(p1p + 0x60), LOCK_9040_R8)
    };

    for (same, k) in [(true, 42u64), (false, 42u64), (true, 99u64)] {
        let o = run(f_o, same, k);
        let n = run(f_n, same, k);
        crate::diff_eq!(o, n, "14248b000 same={} k={}", same, k);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1424afd60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, usize, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_get_id(this: usize) -> u32 {
        rd::<u32>(this + 0x08)
    }
    unsafe extern "win64" fn mock_get_parent(this: usize) -> usize {
        rd::<usize>(this + 0x10)
    }
    unsafe extern "win64" fn mock_cast_up(this: usize, ptr: u64) -> u64 {
        ptr.wrapping_add(rd::<u64>(this + 0x18))
    }

    let mut vtbl = [0usize; 56];
    vtbl[0x08 / 8] = mock_get_id as *const () as usize;
    vtbl[0x20 / 8] = mock_get_parent as *const () as usize;
    vtbl[0x1a8 / 8] = mock_cast_up as *const () as usize;
    let vp = vtbl.as_ptr() as usize;

    // Root (id=10, parent=Root), Mid (id=20, parent=Root, +0x20), Leaf (id=30, parent=Mid, +0x10)
    let mut root_ti: [u64; 4] = [vp as u64, 10, 0, 0];
    root_ti[2] = root_ti.as_ptr() as u64;
    let mid_ti: [u64; 4] = [vp as u64, 20, root_ti.as_ptr() as u64, 0x20];
    let leaf_ti: [u64; 4] = [vp as u64, 30, mid_ti.as_ptr() as u64, 0x10];
    let other_ti: [u64; 4] = [vp as u64, 99, root_ti.as_ptr() as u64, 0];

    for target_ptr in [
        leaf_ti.as_ptr() as usize,
        mid_ti.as_ptr() as usize,
        other_ti.as_ptr() as usize,
    ] {
        let o = f_o(0x1000, leaf_ti.as_ptr() as usize, target_ptr);
        let n = f_n(0x1000, leaf_ti.as_ptr() as usize, target_ptr);
        crate::diff_eq!(o, n, "1424afd60");
    }
    Ok(3)
}

pub unsafe fn diff_snd_1424d0890(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FREE_3A50_ARG: u64 = 0;

    unsafe extern "win64" fn mock_load_view(ptr: u64, len: u32, out_id: usize) -> u32 {
        wr::<u32>(out_id, (ptr as u32).wrapping_add(len).wrapping_add(100));
        1
    }
    unsafe extern "win64" fn mock_load_copy(ptr: u64, len: u32, out_id: usize) -> u32 {
        wr::<u32>(out_id, (ptr as u32).wrapping_add(len).wrapping_add(200));
        1
    }
    unsafe extern "win64" fn mock_3a50(ptr: u64) {
        FREE_3A50_ARG = ptr;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4158_28e0, mock_load_view as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4158_2530, mock_load_copy as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4244_3a50, mock_3a50 as *const ())?;

    let g_9b8 = exe::ptr::<u32>(0x1_4387_19b8) as usize;
    let s_9b8 = rd::<u32>(g_9b8);

    let run = |f: F, mode: u32| -> (u32, [u64; 6], u64) {
        FREE_3A50_ARG = 0;
        wr::<u32>(g_9b8, mode);
        let mut p1 = [0u8; 0xe0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1p + 0x80, 2);
        wr::<u64>(p1p + 0xb8, 0x5000);
        wr::<u32>(p1p + 0xc0, 0x100);
        wr::<u64>(p1p + 0xc8, 0x7777);

        let ret = f(p1p);
        let snap = [
            rd::<u32>(p1p + 0x80) as u64,
            rd::<u64>(p1p + 0xb8),
            rd::<u32>(p1p + 0xc0) as u64,
            rd::<u64>(p1p + 0xc8),
            rd::<u32>(p1p + 0xd0) as u64,
            rd::<u32>(p1p + 0xd8) as u64,
        ];
        (ret, snap, FREE_3A50_ARG)
    };

    for mode in [0u32, 1, 2] {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        wr::<u32>(g_9b8, s_9b8);
        crate::diff_eq!(o, n, "1424d0890 mode={}", mode);
    }
    Ok(3)
}

pub unsafe fn diff_ui_142502f50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_HASH: u64 = 0;
    static mut CTX_VTBL: [usize; 16] = [0; 16];
    static mut CTX_OBJ: [usize; 1] = [0; 1];
    static mut MGR_VTBL: [usize; 74] = [0; 74];
    static mut MGR_OBJ: [usize; 1] = [0; 1];
    static mut NODE_28: [usize; 6] = [0; 6];
    static mut SP_SLOT: [usize; 1] = [0; 1];

    unsafe extern "win64" fn mock_0810(_rcx: usize, rdx_key_ptr: usize) -> usize {
        let k = rd::<u32>(rdx_key_ptr) as u64;
        LOG_HASH = LOG_HASH.wrapping_mul(31).wrapping_add(k);
        core::ptr::addr_of_mut!(SP_SLOT) as usize
    }
    unsafe extern "win64" fn mock_4de0(_ctx: usize, val: usize) {
        LOG_HASH = LOG_HASH.wrapping_mul(31).wrapping_add(val as u64);
    }
    unsafe extern "win64" fn mock_1640(_ctx: usize, _val: usize) {
        LOG_HASH = LOG_HASH.wrapping_mul(31).wrapping_add(0x1640);
    }
    unsafe extern "win64" fn mock_15d0(_ctx: usize, _val: usize) {
        LOG_HASH = LOG_HASH.wrapping_mul(31).wrapping_add(0x15d0);
    }
    unsafe extern "win64" fn mock_mgr_218(_mgr: usize) -> usize {
        LOG_HASH = LOG_HASH.wrapping_mul(31).wrapping_add(0x218);
        core::ptr::addr_of_mut!(CTX_OBJ) as usize
    }
    unsafe extern "win64" fn mock_mgr_240(
        _mgr: usize,
        name_ptr: usize,
        r8_c0: usize,
        r9_rbp: u64,
        arg5_cb: u64,
    ) {
        let first_ch = rd::<u8>(name_ptr) as u64;
        let c0_tag = rd::<u64>(r8_c0);
        let cb_off = arg5_cb.wrapping_sub(exe::ptr::<u8>(0x1_4145_dd40) as usize as u64);
        LOG_HASH = LOG_HASH
            .wrapping_mul(31)
            .wrapping_add(first_ch)
            .wrapping_add(c0_tag)
            .wrapping_add(r9_rbp)
            .wrapping_add(cb_off);
    }
    unsafe extern "win64" fn mock_ctx_28(_ctx: usize) {
        LOG_HASH = LOG_HASH.wrapping_mul(31).wrapping_add(0x28);
    }
    unsafe extern "win64" fn mock_ctx_40(_ctx: usize) -> u64 {
        LOG_HASH = LOG_HASH.wrapping_mul(31).wrapping_add(0x40);
        0x7788_9900
    }
    unsafe extern "win64" fn mock_ctx_48(_ctx: usize, _arg: usize) {
        LOG_HASH = LOG_HASH.wrapping_mul(31).wrapping_add(0x48);
    }
    unsafe extern "win64" fn mock_ctx_60(_ctx: usize) {
        LOG_HASH = LOG_HASH.wrapping_mul(31).wrapping_add(0x60);
    }
    unsafe extern "win64" fn mock_ctx_68(_ctx: usize, name_ptr: usize, _arg: usize) {
        let ch = rd::<u8>(name_ptr) as u64;
        LOG_HASH = LOG_HASH.wrapping_mul(31).wrapping_add(ch);
    }
    unsafe extern "win64" fn mock_ctx_70(_ctx: usize, mode: u32) {
        LOG_HASH = LOG_HASH.wrapping_mul(31).wrapping_add(mode as u64);
    }
    unsafe extern "win64" fn mock_ctx_78(_ctx: usize) -> u32 {
        LOG_HASH = LOG_HASH.wrapping_mul(31).wrapping_add(0x78);
        9
    }
    unsafe extern "win64" fn mock_p1_d0(this: usize) -> u64 {
        LOG_HASH = LOG_HASH.wrapping_mul(31).wrapping_add(rd::<u64>(this + 0xc0));
        0xfeed_cafe
    }

    let _m1 = crate::difftest::Mock::new(0x1_4141_0810, mock_0810 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_424e_4de0, mock_4de0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4250_1640, mock_1640 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4250_15d0, mock_15d0 as *const ())?;

    let run = |f: F, vid_ptr: usize| -> (u64, u64) {
        LOG_HASH = 1;
        CTX_VTBL = [0; 16];
        CTX_VTBL[0x28 / 8] = mock_ctx_28 as *const () as usize;
        CTX_VTBL[0x40 / 8] = mock_ctx_40 as *const () as usize;
        CTX_VTBL[0x48 / 8] = mock_ctx_48 as *const () as usize;
        CTX_VTBL[0x60 / 8] = mock_ctx_60 as *const () as usize;
        CTX_VTBL[0x68 / 8] = mock_ctx_68 as *const () as usize;
        CTX_VTBL[0x70 / 8] = mock_ctx_70 as *const () as usize;
        CTX_VTBL[0x78 / 8] = mock_ctx_78 as *const () as usize;
        CTX_OBJ[0] = core::ptr::addr_of!(CTX_VTBL) as usize;

        MGR_VTBL = [0; 74];
        MGR_VTBL[0x218 / 8] = mock_mgr_218 as *const () as usize;
        MGR_VTBL[0x240 / 8] = mock_mgr_240 as *const () as usize;
        MGR_OBJ[0] = core::ptr::addr_of!(MGR_VTBL) as usize;

        NODE_28 = [0; 6];
        NODE_28[0x28 / 8] = core::ptr::addr_of_mut!(MGR_OBJ) as usize;
        SP_SLOT[0] = core::ptr::addr_of_mut!(NODE_28) as usize;

        let mut sub_10 = [0u8; 0x80];
        wr::<usize>(sub_10.as_mut_ptr() as usize + 0x78, vid_ptr);

        let mut p1_vtbl = [0usize; 28];
        p1_vtbl[0xd0 / 8] = mock_p1_d0 as *const () as usize;

        let mut p1 = [0u8; 0xd0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, p1_vtbl.as_ptr() as usize);
        wr::<usize>(p1p + 0x10, sub_10.as_ptr() as usize);
        wr::<u32>(p1p + 0x18, 7);
        wr::<u64>(p1p + 0xc0, 0x1234_5678);

        let ret = f(p1p);
        (ret, LOG_HASH)
    };

    for vid in [0usize, 0x3333usize] {
        let o = run(f_o, vid);
        let n = run(f_n, vid);
        crate::diff_eq!(o, n, "142502f50 vid={:#x}", vid);
    }
    Ok(2)
}

// REGISTRY:
// Replacement::verified_throws(0x1_4242_0050, "r_142420050", rend::cauldron_hot_x7::r_142420050 as *const (), Throws::Caught, rend::cauldron_hot_x7::diff_r_142420050),
// Replacement::verified(0x1_4242_3190, "r_142423190", rend::cauldron_hot_x7::r_142423190 as *const (), rend::cauldron_hot_x7::diff_r_142423190),
// Replacement::verified_throws(0x1_4242_48a0, "unknown_1424248a0", rend::cauldron_hot_x7::unknown_1424248a0 as *const (), Throws::Caught, rend::cauldron_hot_x7::diff_unknown_1424248a0),
// Replacement::verified(0x1_4242_50f0, "unknown_1424250f0", rend::cauldron_hot_x7::unknown_1424250f0 as *const (), rend::cauldron_hot_x7::diff_unknown_1424250f0),
// Replacement::verified(0x1_4243_b450, "unknown_14243b450", rend::cauldron_hot_x7::unknown_14243b450 as *const (), rend::cauldron_hot_x7::diff_unknown_14243b450),
// Replacement::verified(0x1_4243_fd50, "r_14243fd50", rend::cauldron_hot_x7::r_14243fd50 as *const (), rend::cauldron_hot_x7::diff_r_14243fd50),
// Replacement::verified_throws(0x1_4247_6fc0, "r_142476fc0", rend::cauldron_hot_x7::r_142476fc0 as *const (), Throws::Caught, rend::cauldron_hot_x7::diff_r_142476fc0),
// Replacement::verified_throws(0x1_4247_72b0, "r_1424772b0", rend::cauldron_hot_x7::r_1424772b0 as *const (), Throws::Discard, rend::cauldron_hot_x7::diff_r_1424772b0),
// Replacement::verified(0x1_4248_b000, "unknown_14248b000", rend::cauldron_hot_x7::unknown_14248b000 as *const (), rend::cauldron_hot_x7::diff_unknown_14248b000),
// Replacement::verified(0x1_424a_fd60, "unknown_1424afd60", rend::cauldron_hot_x7::unknown_1424afd60 as *const (), rend::cauldron_hot_x7::diff_unknown_1424afd60),
// Replacement::verified(0x1_424d_0890, "snd_1424d0890", rend::cauldron_hot_x7::snd_1424d0890 as *const (), rend::cauldron_hot_x7::diff_snd_1424d0890),
// Replacement::verified_throws(0x1_4250_2f50, "ui_142502f50", rend::cauldron_hot_x7::ui_142502f50 as *const (), Throws::Discard, rend::cauldron_hot_x7::diff_ui_142502f50),
