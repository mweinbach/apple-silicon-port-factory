//! Decompiled hot functions shard Y6 (`0x142421ed0..0x14243fee0`).

use crate::exe::{self, rd, wr};

/// `r_142421ed0`
/// Original: 0x142421ed0 (264 bytes, Throws::Discard)
pub unsafe extern "win64" fn r_142421ed0(
    param_1: usize,
    param_2: usize,
    param_3: u64,
    param_4: u64,
    param_5: u8,
) -> usize {
    let rdi = rd::<u64>(param_1 + 0x10);
    let rcx_18 = rd::<u64>(param_1 + 0x18);
    let u5 = exe::call_va!(0x1_4242_0c10, (rcx_18) -> u64);
    let l6 = exe::call_va!(0x1_4242_48a0, (rdi, param_3, u5) -> usize);
    if l6 != 0 {
        return exe::call_va!(0x1_4241_fab0, (param_1, l6, param_3, param_4, param_5) -> usize);
    }

    let vtbl = rd::<usize>(param_2);
    let key_u32 = exe::call_va!(rd::<usize>(vtbl + 0x08), (param_2) -> u32);
    let mut key_slot: u64 = (param_4 & !0xffff_ffffu64) | (key_u32 as u64);
    let mut iter: [usize; 2] = [0; 2];
    exe::call_va!(
        0x1_4139_f5c0,
        (
            param_1 + 0x30,
            iter.as_mut_ptr() as usize,
            (&mut key_slot as *mut u64) as usize
        )
    );

    let end_ctrl = rd::<usize>(param_1 + 0x48).wrapping_add(rd::<usize>(param_1 + 0x30));
    if iter[0] == end_ctrl {
        return 0;
    }

    let entry = rd::<usize>(iter[1] + 0x08);
    let fn_ctx = rd::<usize>(entry + 0x168);
    let fn_ptr = rd::<usize>(entry + 0x170);
    let obj = if fn_ctx == 0 && fn_ptr == 0 {
        rd::<usize>(entry + 0x158)
    } else {
        exe::call_va!(fn_ptr, (fn_ctx, 0usize) -> usize)
    };

    if obj == 0 {
        return 0;
    }

    let old_ref = crate::simd_ops::atomic_xadd_u32((obj + 0x24) as u64, 1);
    if old_ref.wrapping_add(1) == 1 {
        let owner = rd::<usize>(obj + 0x28);
        if owner != 0 {
            exe::call_va!(0x1_4241_d6f0, (owner, obj));
        }
    }
    obj
}

/// `unknown_142424b30`
/// Original: 0x142424b30 (665 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_142424b30(
    param_1: usize,
    param_2: u64,
    param_3: usize,
) -> u8 {
    let mut backoff: u64 = 0x64;
    loop {
        while rd::<u8>(param_1 + 0x08) != 0 {
            core::hint::spin_loop();
        }
        if crate::simd_ops::atomic_xchg_u8((param_1 + 0x08) as u64, 1) == 0 {
            break;
        }
        exe::call_va!(0x1_4241_7470, ((&mut backoff as *mut u64) as usize));
    }

    let deleted = rd::<u64>(param_1 + 0x40);
    if rd::<u64>(param_1 + 0x48) != deleted {
        let mut step: u64 = 0;
        let cap = rd::<u64>(param_1 + 0x50);
        let mask = cap.wrapping_sub(1);
        let mut idx = mask & param_2;
        let buckets = rd::<usize>(param_1 + 0x68);
        let empty_key = rd::<u64>(param_1 + 0x58);

        while rd::<u64>(buckets.wrapping_add((idx as usize) * 16)) != empty_key {
            let cur_key = rd::<u64>(buckets.wrapping_add((idx as usize) * 16));
            if deleted == 0 || rd::<u64>(param_1 + 0x38) != cur_key {
                if param_2 == cur_key {
                    if idx == u64::MAX {
                        break;
                    }
                    let slot_ptr = buckets.wrapping_add((idx as usize) * 16);
                    let end_ptr = buckets.wrapping_add((rd::<usize>(param_1 + 0x50)) * 16);
                    if slot_ptr == end_ptr {
                        break;
                    }
                    let r14 = rd::<usize>(slot_ptr + 0x08);
                    crate::simd_ops::wu8((param_1 + 0x08) as u64, 0); // spinlock release (x86 mov = release)
                    if r14 == 0 {
                        return 0;
                    }

                    exe::call_va!(0x1_4243_d770, (param_3));
                    let mut opt_sv = [0u64; 3];
                    exe::call_va!(
                        0x1_413a_a840,
                        (
                            rd::<usize>(param_3 + 0x60),
                            opt_sv.as_mut_ptr() as usize,
                            param_2
                        )
                    );
                    wr::<u64>(r14 + 0x28, opt_sv[0]);
                    wr::<u64>(r14 + 0x30, opt_sv[1]);
                    if rd::<u8>((opt_sv.as_ptr() as usize) + 0x10) == 0 {
                        return 0;
                    }

                    let mut empty_in: [u64; 2] =
                        [exe::ptr::<u8>(0x1_426c_dcb0) as usize as u64, 0];
                    let mut c8_sv = [0u64; 2];
                    exe::call_va!(
                        0x1_401d_f850,
                        (c8_sv.as_mut_ptr() as usize, empty_in.as_mut_ptr() as usize)
                    );
                    let mut sv_arg: [u64; 2] = [opt_sv[0], opt_sv[1]];
                    let mut b8_str = [0u64; 3];
                    let b8_ptr = b8_str.as_mut_ptr() as usize;
                    exe::call_va!(
                        0x1_401e_06c0,
                        (b8_ptr, c8_sv.as_mut_ptr() as usize, sv_arg.as_mut_ptr() as usize)
                    );

                    let b3 = rd::<u8>(b8_ptr + 0x03) as u32;
                    let cap_bits = if b3 < 0x80 {
                        b3.wrapping_mul(8)
                    } else {
                        (b3 << 8).wrapping_sub(0x7c00)
                    };
                    let len_bits = (rd::<u32>(b8_ptr) & 0x00ff_ffff) << 3;
                    let cstr_ptr = if len_bits > cap_bits {
                        rd::<usize>(b8_ptr + 0x08)
                    } else {
                        b8_ptr + 0x08
                    };
                    let mut slen = 0usize;
                    while rd::<u8>(cstr_ptr.wrapping_add(slen)) != 0 {
                        slen += 1;
                    }

                    let mut raw_sv: [u64; 2] = [cstr_ptr as u64, slen as u64];
                    let mut e8_sv = [0u64; 3];
                    exe::call_va!(
                        0x1_401d_f850,
                        (e8_sv.as_mut_ptr() as usize, raw_sv.as_mut_ptr() as usize)
                    );
                    let exists = exe::call_va!(0x1_4249_1120, (e8_sv.as_mut_ptr() as usize) -> u8);
                    let res = if exists == 0 {
                        wr::<u8>(r14, 0);
                        0
                    } else {
                        let mut new_val = [0u64; 3];
                        exe::call_va!(
                            0x1_4242_4570,
                            (new_val.as_mut_ptr() as usize, param_3, param_2)
                        );
                        let mut new_copy = new_val;
                        let mut old_copy = rd::<[u64; 3]>(r14 + 0x08);
                        let old_hash =
                            exe::call_va!(0x1_4241_cc30, (old_copy.as_mut_ptr() as usize) -> u32);
                        let new_hash =
                            exe::call_va!(0x1_4241_cc30, (new_copy.as_mut_ptr() as usize) -> u32);
                        if new_hash != old_hash || rd::<u8>(r14) == 0 {
                            wr::<[u64; 3]>(r14 + 0x08, new_val);
                            1
                        } else {
                            0
                        }
                    };
                    exe::call_va!(0x1_4240_ab10, (b8_ptr));
                    return res;
                }
            }
            step = step.wrapping_add(1);
            idx = idx.wrapping_add(step) & mask;
        }
    }

    crate::simd_ops::wu8((param_1 + 0x08) as u64, 0); // spinlock release (x86 mov = release)
    0
}

/// `r_142432e90`
/// Original: 0x142432e90 (790 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_142432e90(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) -> usize {
    wr::<u64>(param_1, rd::<u64>(param_4));
    wr::<u32>(param_1 + 0x08, rd::<u32>(param_4 + 0x08));
    wr::<u32>(param_1 + 0x0c, rd::<u32>(param_4 + 0x0c));
    wr::<u64>(param_4, 0);
    wr::<u64>(param_4 + 0x08, 0);

    wr::<[u64; 10]>(param_1 + 0x10, [0; 10]);

    let p3_0 = rd::<u64>(param_3);
    wr::<u64>(param_3, 0);
    wr::<u64>(param_1 + 0x60, p3_0);

    let p3_1 = rd::<u64>(param_3 + 0x08);
    wr::<u64>(param_3 + 0x08, 0);
    wr::<u64>(param_1 + 0x68, p3_1);

    let p3_2 = rd::<u64>(param_3 + 0x10);
    wr::<u64>(param_3 + 0x10, 0);
    wr::<u64>(param_1 + 0x70, p3_2);

    let p3_3 = rd::<u64>(param_3 + 0x18);
    wr::<u64>(param_3 + 0x18, 0);
    wr::<u64>(param_1 + 0x78, p3_3);

    let p3_w = rd::<u16>(param_3 + 0x20);
    wr::<u16>(param_3 + 0x20, 0x10);
    wr::<u16>(param_1 + 0x80, p3_w);

    let p3_5 = rd::<u64>(param_3 + 0x28);
    wr::<u64>(param_3 + 0x28, 0);
    wr::<u64>(param_1 + 0x88, p3_5);

    let off_18 = rd::<u32>(param_2 + 0x18) as u64;
    let base_60 = rd::<u64>(param_1 + 0x60);
    wr::<u64>(param_1 + 0x10, rd::<u32>(param_2 + 0x1c) as u64);
    wr::<u64>(param_1 + 0x18, off_18.wrapping_add(base_60));

    let off_20 = rd::<u32>(param_2 + 0x20) as u64;
    wr::<u64>(param_1 + 0x20, rd::<u32>(param_2 + 0x24) as u64);
    wr::<u64>(param_1 + 0x28, off_20.wrapping_add(base_60));

    let off_50 = rd::<u32>(param_2 + 0x50) as u64;
    wr::<u64>(param_1 + 0x30, rd::<u32>(param_2 + 0x54) as u64);
    wr::<u64>(param_1 + 0x38, off_50.wrapping_add(base_60));

    let off_28 = rd::<u32>(param_2 + 0x28) as u64;
    wr::<u64>(param_1 + 0x40, rd::<u32>(param_2 + 0x2c) as u64);
    wr::<u64>(param_1 + 0x48, off_28.wrapping_add(base_60));

    let mut cur_pair = (rd::<u32>(param_2 + 0x30) as u64).wrapping_add(base_60) as usize;
    let pair_count = rd::<u32>(param_2 + 0x34) as usize;
    let r12_base = (rd::<u32>(param_2 + 0x38) as u64).wrapping_add(base_60);
    let cnt_3c = rd::<u32>(param_2 + 0x3c) as u64;
    let end_pair = cur_pair.wrapping_add(pair_count * 8);

    let mut vec_ptr: usize = 0;
    let mut vec_len: u32 = 0;
    let mut vec_cap: u32 = 0;
    let mut loop_idx: u64 = 0;

    while cur_pair != end_pair {
        let len_part = rd::<u32>(cur_pair + 0x04) as u64;
        let str_ptr = (rd::<u32>(cur_pair) as u64).wrapping_add(rd::<u64>(param_1 + 0x48));
        let mut sv: [u64; 2] = [str_ptr, len_part];
        let obj = exe::call_va!(0x1_424b_0e40, (sv.as_mut_ptr() as usize) -> usize);
        if obj != 0 {
            if vec_len == vec_cap {
                let byte_size = (vec_len as usize) * 8;
                let old_len = vec_len as usize;
                let old_cap = vec_cap as usize;
                let mut new_cap = (old_cap >> 1).wrapping_add(old_cap);
                if new_cap < old_len.wrapping_add(1) {
                    new_cap = old_len.wrapping_add(1);
                }
                let new_buf = exe::call_va!(0x1_4241_c0a0, (new_cap * 8, 4u32) -> usize);
                let _ = exe::call_va!(0x1_4183_acc8, (new_buf, vec_ptr, byte_size) -> usize);
                if vec_ptr != 0 {
                    exe::call_va!(0x1_4241_c190, (vec_ptr));
                }
                vec_ptr = new_buf;
                vec_cap = new_cap as u32;
            }
            let slot_idx = vec_len as usize;
            wr::<u64>(vec_ptr.wrapping_add(slot_idx * 8), 0);
            vec_len = vec_len.wrapping_add(1);
            let slot_end = vec_ptr.wrapping_add((vec_len as usize) * 8);
            wr::<u32>(slot_end - 0x04, loop_idx as u32);
            let vtbl = rd::<usize>(obj);
            let mapped_id = exe::call_va!(rd::<usize>(vtbl + 0x08), (obj) -> u32);
            wr::<u32>(slot_end - 0x08, mapped_id);
        }
        loop_idx = loop_idx.wrapping_add(1);
        cur_pair = cur_pair.wrapping_add(8);
    }

    let mut r8_cur = rd::<usize>(param_1 + 0x28);
    let r8_end = r8_cur.wrapping_add((rd::<usize>(param_1 + 0x20)) * 0x20);
    while r8_cur != r8_end {
        if rd::<u32>(r8_cur + 0x1c) > 0 {
            let hdr = (rd::<u32>(r8_cur + 0x18) as usize).wrapping_add(r12_base as usize);
            let sub_cnt = rd::<u32>(hdr + 0x04) as usize;
            let mut item_cur = hdr.wrapping_add(0x08);
            let item_end = item_cur.wrapping_add(sub_cnt * 8);
            let vec_end = vec_ptr.wrapping_add((vec_len as usize) * 8);
            while item_cur != item_end {
                let want = rd::<u32>(item_cur);
                let mut scan = vec_ptr;
                while scan != vec_end {
                    if (rd::<u64>(scan) >> 32) as u32 == want {
                        break;
                    }
                    scan = scan.wrapping_add(8);
                }
                let new_val = if scan == vec_end {
                    0xffff_ffff
                } else {
                    rd::<u32>(scan)
                };
                wr::<u32>(item_cur, new_val);
                item_cur = item_cur.wrapping_add(8);
            }
        }
        r8_cur = r8_cur.wrapping_add(0x20);
    }

    wr::<u64>(param_1 + 0x50, cnt_3c);
    wr::<u64>(param_1 + 0x58, r12_base);
    if vec_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (vec_ptr));
    }
    param_1
}

/// `r_1424363d0`
/// Original: 0x1424363d0 (299 bytes, Throws::Discard)
pub unsafe extern "win64" fn r_1424363d0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let mut sv = rd::<[u64; 2]>(param_3);
    let ptr0 = sv[0] as usize;
    let len0 = sv[1];

    let is_abs = if len0 == 0 {
        false
    } else {
        let b0 = rd::<u8>(ptr0);
        b0 == b'/' || b0 == b'\\' || (len0 >= 2 && rd::<u8>(ptr0 + 1) == b':')
    };

    if is_abs {
        let mut res = [0u64; 8];
        exe::call_va!(
            0x1_413a_7e00,
            (res.as_mut_ptr() as usize, param_3, param_1 + 0x20)
        );
        if res[6] != u64::MAX {
            wr::<u8>(param_2 + 0x10, 0);
            return param_2;
        }
        let pos = res[2];
        if pos == u64::MAX {
            sv = [0, 0];
        } else {
            let total_len = res[1];
            if total_len < pos {
                exe::call_va!(0x1_401d_e590, ());
                return param_2;
            }
            let sub_len = total_len.wrapping_sub(pos);
            let mut in_sv: [u64; 2] = [res[0].wrapping_add(pos), sub_len];
            let mut out_sv = [0u64; 2];
            exe::call_va!(
                0x1_401d_f850,
                (out_sv.as_mut_ptr() as usize, in_sv.as_mut_ptr() as usize)
            );
            sv = out_sv;
        }
    }

    let filter_fn = rd::<usize>(param_1 + 0x38);
    if filter_fn != 0 {
        let mut sv_arg = sv;
        let rejected = exe::call_va!(filter_fn, (sv_arg.as_mut_ptr() as usize) -> u8);
        if rejected != 0 {
            wr::<u8>(param_2 + 0x10, 0);
            return param_2;
        }
    }

    wr::<[u64; 2]>(param_2, sv);
    wr::<u8>(param_2 + 0x10, 1);
    param_2
}

/// `r_142436c80`
/// Original: 0x142436c80 (303 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_142436c80(param_1: usize, param_2: usize) -> u8 {
    let mut in_sv = rd::<[u64; 2]>(param_2);
    let mut resolved = [0u64; 3];
    let _ = exe::call_va!(
        0x1_4243_63d0,
        (
            param_1,
            resolved.as_mut_ptr() as usize,
            in_sv.as_mut_ptr() as usize
        ) -> usize
    );

    if rd::<u8>((resolved.as_ptr() as usize) + 0x10) == 0 {
        let fallback = rd::<usize>(param_1 + 0x40);
        let vtbl = rd::<usize>(fallback);
        let mut sv_copy = rd::<[u64; 2]>(param_2);
        return exe::call_va!(
            rd::<usize>(vtbl + 0x30),
            (fallback, sv_copy.as_mut_ptr() as usize) -> u8
        );
    }

    let srw = param_1 + 0x08;
    exe::call_iat!(0x1_4264_55a8, (srw));
    let mut cur = rd::<usize>(param_1 + 0x10);
    let count = rd::<u32>(param_1 + 0x18) as usize;
    let end = cur.wrapping_add(count * 8);

    while cur != end {
        let pack = rd::<usize>(cur);
        let mut path_sv: [u64; 2] = [resolved[0], resolved[1]];
        let mut tmp_dir = [0u64; 2];
        let dir_ptr = exe::call_va!(
            0x1_40f3_5ac0,
            (path_sv.as_mut_ptr() as usize, tmp_dir.as_mut_ptr() as usize) -> usize
        );
        let mut dir_sv = rd::<[u64; 2]>(dir_ptr);
        let dir_node =
            exe::call_va!(0x1_4243_2c60, (pack, dir_sv.as_mut_ptr() as usize) -> usize);
        if dir_node != 0 {
            let mut tmp_file = [0u64; 2];
            let file_ptr = exe::call_va!(
                0x1_401d_fa80,
                (path_sv.as_mut_ptr() as usize, tmp_file.as_mut_ptr() as usize) -> usize
            );
            let mut file_sv = rd::<[u64; 2]>(file_ptr);
            let file_node = exe::call_va!(
                0x1_4243_2790,
                (pack, dir_node, file_sv.as_mut_ptr() as usize) -> usize
            );
            if file_node != 0 {
                exe::call_iat!(0x1_4264_5598, (srw));
                return 1;
            }
        }
        cur = cur.wrapping_add(8);
    }

    exe::call_iat!(0x1_4264_5598, (srw));
    0
}

/// `unknown_14243b3e0`
/// Original: 0x14243b3e0 (92 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14243b3e0(param_1: usize, param_2: u64) {
    let mut arena = param_1;
    while param_2 < rd::<u64>(arena + 0x20) || param_2 >= rd::<u64>(arena + 0x28) {
        arena = rd::<usize>(arena + 0x38);
        if arena == 0 {
            return;
        }
    }

    let _ = crate::simd_ops::atomic_sub_u32(arena as u64, 1);
    let tag_addr = exe::ptr::<u16>(0x1_4397_d8fa) as usize as u64;
    let old_tag = crate::simd_ops::atomic_xadd_u16(tag_addr, 1);
    let tagged_ptr = (param_2 & 0x0000_ffff_ffff_ffff) | ((old_tag as u64) << 48);
    let node_ptr = (tagged_ptr & 0x0000_ffff_ffff_ffff) as usize;
    let mut head = rd::<u64>(arena + 0x18);
    loop {
        wr::<u64>(node_ptr, head);
        let (ok, cur) =
            crate::simd_ops::atomic_cmpxchg_u64((arena + 0x18) as u64, head, tagged_ptr);
        if ok {
            break;
        }
        head = cur;
    }
}

/// `r_14243d770`
/// Original: 0x14243d770 (412 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_14243d770(param_1: usize) {
    if rd::<usize>(param_1 + 0x10) == 0 {
        return;
    }

    loop {
        let q = rd::<usize>(param_1 + 0x10);
        let srw = q + 0x08;
        exe::call_iat!(0x1_4264_55a8, (srw));

        let mut s2 = [0u64; 3];
        let s2_ptr = s2.as_mut_ptr() as usize;
        let has_item = if rd::<u32>(q + 0x20) == 0 {
            false
        } else {
            let head = rd::<u32>(q + 0x18) as u64;
            let cap = rd::<u32>(q + 0x24) as u64;
            let rem = (head % cap) as usize;
            let src_elem = rd::<usize>(q + 0x10).wrapping_add(rem * 24);

            let mut s1 = [0u64; 3];
            let s1_ptr = s1.as_mut_ptr() as usize;
            wr::<u32>(s1_ptr, 0x0200_0002);
            wr::<u32>(s1_ptr + 0x04, 0);
            wr::<u8>(s1_ptr + 0x08, 0);
            exe::call_va!(0x1_4240_a730, (s1_ptr, src_elem));

            if rd::<u32>(q + 0x20) != 0 {
                let idx = rd::<u32>(q + 0x18) as usize;
                let slot_ptr = rd::<usize>(q + 0x10).wrapping_add(idx * 24);
                exe::call_va!(0x1_4240_ab10, (slot_ptr));
                let next_idx = rd::<u32>(q + 0x18).wrapping_add(1);
                wr::<u32>(q + 0x18, next_idx);
                if next_idx == rd::<u32>(q + 0x24) {
                    wr::<u32>(q + 0x18, 0);
                }
                wr::<u32>(q + 0x20, rd::<u32>(q + 0x20).wrapping_sub(1));
            }

            wr::<u32>(s2_ptr, 0x0200_0002);
            wr::<u32>(s2_ptr + 0x04, 0);
            wr::<u8>(s2_ptr + 0x08, 0);
            exe::call_va!(0x1_4240_a730, (s2_ptr, s1_ptr));
            exe::call_va!(0x1_4240_ab10, (s1_ptr));
            true
        };

        exe::call_iat!(0x1_4264_5598, (srw));
        if !has_item {
            break;
        }

        let b3 = rd::<u8>(s2_ptr + 0x03) as u32;
        let cap_bits = if b3 < 0x80 {
            b3.wrapping_mul(8)
        } else {
            (b3 << 8).wrapping_sub(0x7c00)
        };
        let len_bits = (rd::<u32>(s2_ptr) & 0x00ff_ffff) << 3;
        let str_ptr = if len_bits > cap_bits {
            rd::<usize>(s2_ptr + 0x08)
        } else {
            s2_ptr + 0x08
        };
        let str_len = rd::<u32>(s2_ptr + 0x04) as u64;
        let mut sv: [u64; 2] = [str_ptr as u64, str_len];
        let mut out_buf = [0u64; 2];
        exe::call_va!(
            0x1_4243_e3d0,
            (
                param_1,
                out_buf.as_mut_ptr() as usize,
                sv.as_mut_ptr() as usize
            )
        );
        exe::call_va!(0x1_4240_ab10, (s2_ptr));
    }
}

/// `r_14243ebc0`
/// Original: 0x14243ebc0 (537 bytes, Throws::Discard)
pub unsafe extern "win64" fn r_14243ebc0(
    param_1: usize,
    param_2: u64,
    param_3: usize,
    param_4: usize,
) -> u8 {
    let r12 = rd::<usize>(param_1 + 0x58);
    let slow_path = if rd::<u32>(param_1 + 0x08) == 0 {
        true
    } else {
        let tls0 = rd::<usize>(exe::tls_pointer());
        rd::<u8>(tls0 + 0x6311) != 0
    };

    if !slow_path {
        let mut p4_copy = rd::<[u64; 2]>(param_4);
        let mut p3_copy = rd::<[u64; 2]>(param_3);
        let got = exe::call_va!(
            0x1_4243_fd50,
            (
                r12,
                param_2,
                0u32,
                p3_copy.as_mut_ptr() as usize,
                p4_copy.as_mut_ptr() as usize
            ) -> u64
        );
        let want = rd::<u64>(param_3);
        if got == want {
            return 1;
        }

        let in_cache =
            exe::call_va!(0x1_4243_fee0, (rd::<usize>(param_1 + 0x58), param_2, 0u32) -> u8);
        if in_cache == 0 {
            exe::call_va!(0x1_4243_d770, (param_1));
            let mut opt_sv = [0u64; 3];
            exe::call_va!(
                0x1_413a_a840,
                (
                    rd::<usize>(param_1 + 0x60),
                    opt_sv.as_mut_ptr() as usize,
                    param_2
                )
            );
            if rd::<u8>((opt_sv.as_ptr() as usize) + 0x10) != 0 {
                let mut sv: [u64; 2] = [opt_sv[0], opt_sv[1]];
                exe::call_va!(
                    0x1_413a_ad10,
                    (
                        rd::<usize>(param_1 + 0x78),
                        param_2,
                        sv.as_mut_ptr() as usize,
                        1000u32
                    )
                );
                let ok = exe::call_va!(
                    0x1_413a_b1b0,
                    (rd::<usize>(param_1 + 0x78), param_2, 0u32) -> u8
                );
                if ok != 0 {
                    let mut p4_copy2 = rd::<[u64; 2]>(param_4);
                    let mut p3_copy2 = rd::<[u64; 2]>(param_3);
                    let got2 = exe::call_va!(
                        0x1_4243_fd50,
                        (
                            r12,
                            param_2,
                            0u32,
                            p3_copy2.as_mut_ptr() as usize,
                            p4_copy2.as_mut_ptr() as usize
                        ) -> u64
                    );
                    return if got2 == want { 1 } else { 0 };
                }
            }
        }
        return 0;
    }

    exe::call_va!(0x1_4243_d770, (param_1));
    exe::call_va!(0x1_4243_d770, (param_1));
    let mut opt_sv = [0u64; 3];
    exe::call_va!(
        0x1_413a_a840,
        (
            rd::<usize>(param_1 + 0x60),
            opt_sv.as_mut_ptr() as usize,
            param_2
        )
    );
    if rd::<u8>((opt_sv.as_ptr() as usize) + 0x10) != 0 {
        let mut sv: [u64; 2] = [opt_sv[0], opt_sv[1]];
        let mut opt_ver: u64 = 0;
        let opt_ver_ptr = (&mut opt_ver as *mut u64) as usize;
        exe::call_va!(
            0x1_413a_a940,
            (
                rd::<usize>(param_1 + 0x68),
                opt_ver_ptr,
                sv.as_mut_ptr() as usize
            )
        );
        if rd::<u8>(opt_ver_ptr + 0x04) != 0 {
            let ver = rd::<u32>(opt_ver_ptr);
            let ready = exe::call_va!(
                0x1_413a_ae10,
                (rd::<usize>(param_1 + 0x78), param_2, ver) -> u8
            );
            if ready == 0 {
                let mut sv2: [u64; 2] = [opt_sv[0], opt_sv[1]];
                exe::call_va!(
                    0x1_413a_ad10,
                    (
                        rd::<usize>(param_1 + 0x78),
                        param_2,
                        sv2.as_mut_ptr() as usize,
                        1000u32
                    )
                );
                exe::call_va!(0x1_413a_b1b0, (rd::<usize>(param_1 + 0x78), param_2, ver));
            }
            let mut p4_copy = rd::<[u64; 2]>(param_4);
            let mut p3_copy = rd::<[u64; 2]>(param_3);
            let got = exe::call_va!(
                0x1_4243_fd50,
                (
                    r12,
                    param_2,
                    ver,
                    p3_copy.as_mut_ptr() as usize,
                    p4_copy.as_mut_ptr() as usize
                ) -> u64
            );
            return if got == rd::<u64>(param_3) { 1 } else { 0 };
        }
    }
    0
}

/// `r_14243ede0`
/// Original: 0x14243ede0 (298 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_14243ede0(param_1: usize) {
    let rsi = rd::<usize>(param_1 + 0x80);
    while rd::<i32>(rsi + 0x38) > 0 {
        exe::call_iat!(0x1_4264_55a8, (rsi + 0x48));
        let mut cnt = rd::<i32>(rsi + 0x50);
        while cnt <= 0 {
            let _ = exe::call_iat!(
                0x1_4264_55b8,
                (rsi + 0x40, rsi + 0x48, 0xffff_ffffu32, 0u32) -> i32
            );
            cnt = rd::<i32>(rsi + 0x50);
        }
        wr::<i32>(rsi + 0x50, cnt - 1);
        exe::call_iat!(0x1_4264_5598, (rsi + 0x48));
    }

    let g_2f8 = rd::<u64>(exe::ptr::<u64>(0x1_43a0_82f8) as usize);
    exe::call_va!(0x1_4248_5780, (g_2f8, rsi + 0x58));

    let mut cur = rd::<usize>(rsi + 0x28);
    let count = rd::<u32>(rsi + 0x30) as usize;
    let end = cur.wrapping_add(count * 0x88);
    while cur != end {
        if rd::<u8>(cur + 0x80) != 0 {
            let ver = rd::<u32>(cur + 0x08);
            let key = rd::<u64>(cur);
            let p58 = rd::<usize>(param_1 + 0x58);
            let has = exe::call_va!(0x1_4243_fee0, (p58, key, ver) -> u8);
            if has == 0 {
                let mut span: [u64; 2] = [rd::<u32>(cur + 0x78) as u64, rd::<u64>(cur + 0x70)];
                exe::call_va!(
                    0x1_4243_ffd0,
                    (
                        rd::<usize>(param_1 + 0x58),
                        rd::<u64>(cur),
                        rd::<u32>(cur + 0x08),
                        span.as_mut_ptr() as usize
                    )
                );
            }
        }
        cur = cur.wrapping_add(0x88);
    }

    exe::call_va!(0x1_401d_e830, (rsi + 0x18));
    exe::call_va!(0x1_413a_ca40, (rsi + 0x28));
}

/// `r_14243f5b0`
/// Original: 0x14243f5b0 (329 bytes, Throws::No)
pub unsafe extern "win64" fn r_14243f5b0(param_1: usize, param_2: usize, param_3: usize) {
    let base_ptr = rd::<usize>(param_3 + 0x08);
    let count = rd::<usize>(param_3);
    let mut cur = base_ptr.wrapping_add(count * 8);

    while cur != base_ptr {
        cur = cur.wrapping_sub(8);
        let free_head = rd::<i32>(param_1 + 0x78);
        let slot_idx: i32 = if free_head < 0 {
            let len = rd::<u32>(param_1 + 0x70);
            let cap = rd::<u32>(param_1 + 0x74);
            if ((len as u64) << 4) == ((cap as u64) << 4) {
                let min_cap = (len as u64).wrapping_add(1);
                let grown = ((cap >> 1) as u64).wrapping_add(cap as u64);
                let new_cap = if grown < min_cap { min_cap } else { grown };
                exe::call_va!(0x1_413a_d5b0, (param_1 + 0x68, new_cap));
            }
            let cur_len = rd::<u32>(param_1 + 0x70) as usize;
            let arr = rd::<usize>(param_1 + 0x68);
            wr::<u64>(arr.wrapping_add(cur_len * 16 + 0x08), 0);
            wr::<u64>(arr.wrapping_add(cur_len * 16), 0);
            let next_len = (cur_len as u32).wrapping_add(1);
            wr::<u32>(param_1 + 0x70, next_len);
            let end_slot = rd::<usize>(param_1 + 0x68).wrapping_add((next_len as usize) * 16);
            wr::<u32>(end_slot - 0x04, 0xffff_ffff);
            len as i32
        } else {
            let slot_ptr = rd::<usize>(param_1 + 0x68).wrapping_add((free_head as usize) * 16);
            wr::<u32>(param_1 + 0x78, rd::<u32>(slot_ptr + 0x0c));
            wr::<u32>(slot_ptr + 0x0c, 0xffff_ffff);
            free_head
        };

        let slot_ptr =
            rd::<usize>(param_1 + 0x68).wrapping_add((slot_idx as isize as usize) * 16);
        wr::<u32>(slot_ptr + 0x0c, rd::<u32>(param_2 + 0x04));

        let item_obj = rd::<usize>(cur);
        let vtbl = rd::<usize>(item_obj);
        let type_id = exe::call_va!(rd::<usize>(vtbl + 0x98), (item_obj) -> u32);
        wr::<u32>(slot_ptr + 0x08, type_id);

        let item_val = rd::<u64>(cur);
        wr::<u64>(cur, 0);
        let old_obj = rd::<usize>(slot_ptr);
        wr::<u64>(slot_ptr, item_val);
        if old_obj != 0 {
            let old_vtbl = rd::<usize>(old_obj);
            exe::call_va!(rd::<usize>(old_vtbl), (old_obj, 0u32));
            exe::call_va!(0x1_4244_2de0, (old_obj));
        }
        wr::<i32>(param_2 + 0x04, slot_idx);
    }
}

/// `r_14243f750`
/// Original: 0x14243f750 (384 bytes, Throws::No)
pub unsafe extern "win64" fn r_14243f750(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) -> usize {
    let mut cur_idx = rd::<i32>(param_3 + 0x04);
    let mut prev_slot: usize = 0;

    while cur_idx >= 0 {
        let idx_usize = cur_idx as u32 as usize;
        let slot_ptr = rd::<usize>(param_1 + 0x68).wrapping_add(idx_usize * 16);

        let arr_len = rd::<usize>(param_4);
        let arr_ptr = rd::<usize>(param_4 + 0x08);
        let arr_end = arr_ptr.wrapping_add(arr_len * 8);
        let mut scan = arr_ptr;
        while scan != arr_end {
            let candidate = rd::<usize>(scan);
            if candidate != 0 {
                let vtbl = rd::<usize>(candidate);
                let tid = exe::call_va!(rd::<usize>(vtbl + 0x98), (candidate) -> u32);
                if tid == rd::<u32>(slot_ptr + 0x08) {
                    break;
                }
            }
            scan = scan.wrapping_add(8);
        }

        exe::call_va!(0x1_413a_be30, (param_1 + 0x80, slot_ptr));

        let check_end = rd::<usize>(param_4 + 0x08).wrapping_add(rd::<usize>(param_4) * 8);
        if scan == check_end {
            let next_link = rd::<u32>(slot_ptr + 0x0c);
            if prev_slot == 0 {
                wr::<u32>(param_3 + 0x04, next_link);
            } else {
                wr::<u32>(prev_slot + 0x0c, next_link);
            }
            let slot2 = rd::<usize>(param_1 + 0x68).wrapping_add(idx_usize * 16);
            let next_idx = rd::<i32>(slot2 + 0x0c);
            exe::call_va!(0x1_413a_be30, (param_1 + 0x80, slot2));
            wr::<u32>(slot2 + 0x0c, rd::<u32>(param_1 + 0x78));
            wr::<u32>(slot2 + 0x08, 0xffff_ffff);
            wr::<i32>(param_1 + 0x78, cur_idx);
            cur_idx = next_idx;
        } else {
            let new_obj = rd::<u64>(scan);
            wr::<u64>(scan, 0);
            let old_obj = rd::<usize>(slot_ptr);
            wr::<u64>(slot_ptr, new_obj);
            if old_obj != 0 {
                let old_vtbl = rd::<usize>(old_obj);
                exe::call_va!(rd::<usize>(old_vtbl), (old_obj, 0u32));
                exe::call_va!(0x1_4244_2de0, (old_obj));
            }

            let span = rd::<[u64; 2]>(param_4);
            let len = span[0];
            let base = span[1] as usize;
            let last_elem = base.wrapping_sub(8).wrapping_add((len as usize) * 8);
            if scan != last_elem {
                let a = rd::<u64>(scan);
                let b = rd::<u64>(last_elem);
                wr::<u64>(scan, b);
                wr::<u64>(last_elem, a);
            }
            let dec = len.wrapping_sub(1);
            let new_len = if dec == u64::MAX { len } else { dec };
            wr::<u64>(param_4, new_len);
            wr::<u64>(param_4 + 0x08, span[1]);
            cur_idx = rd::<i32>(slot_ptr + 0x0c);
            prev_slot = slot_ptr;
        }
    }

    let out_span = rd::<[u64; 2]>(param_4);
    wr::<[u64; 2]>(param_2, out_span);
    param_2
}

/// `r_14243fee0`
/// Original: 0x14243fee0 (230 bytes, Throws::No)
pub unsafe extern "win64" fn r_14243fee0(param_1: usize, param_2: u64, param_3: u32) -> u8 {
    exe::call_iat!(0x1_4264_55a8, (param_1));
    let mut found: u8 = 0;

    let deleted = rd::<u64>(param_1 + 0x38);
    if rd::<u64>(param_1 + 0x40) != deleted {
        let cap = rd::<u64>(param_1 + 0x48);
        let mask = cap.wrapping_sub(1);
        let mut step: u64 = 0;
        let buckets = rd::<usize>(param_1 + 0x60);
        let empty_key = rd::<u64>(param_1 + 0x50);
        let mut idx = mask & param_2;

        while rd::<u64>(buckets.wrapping_add((idx as usize) * 16)) != empty_key {
            let cur_key = rd::<u64>(buckets.wrapping_add((idx as usize) * 16));
            if deleted == 0 || rd::<u64>(param_1 + 0x30) != cur_key {
                if param_2 == cur_key {
                    if idx != u64::MAX {
                        let slot_ptr = buckets.wrapping_add((idx as usize) * 16);
                        let end_ptr = buckets.wrapping_add((rd::<usize>(param_1 + 0x48)) * 16);
                        if slot_ptr != end_ptr && rd::<u32>(slot_ptr + 0x08) >= param_3 {
                            found = 1;
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
    found
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_r_142421ed0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64, u64, u8) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FOUND_L6: usize = 0;
    static mut ENTRY_SLOT_PTR: usize = 0;
    static mut D6F0_SUM: u64 = 0;

    unsafe extern "win64" fn mock_0c10(x: u64) -> u64 {
        x.wrapping_add(0x100)
    }
    unsafe extern "win64" fn mock_48a0(_rdi: u64, _p3: u64, _u5: u64) -> usize {
        FOUND_L6
    }
    unsafe extern "win64" fn mock_fab0(
        p1: usize,
        l6: usize,
        p3: u64,
        _p4: u64,
        p5: u8,
    ) -> usize {
        p1.wrapping_add(l6)
            .wrapping_add(p3 as usize)
            .wrapping_add((p5 as usize) * 0x1000)
    }
    unsafe extern "win64" fn mock_get_type(_this: usize) -> u32 {
        0x42
    }
    unsafe extern "win64" fn mock_f5c0(map: usize, out_iter: usize, key_ptr: usize) {
        let k = rd::<u32>(key_ptr);
        if k == 0x42 {
            wr::<usize>(out_iter, rd::<usize>(map));
            wr::<usize>(out_iter + 0x08, ENTRY_SLOT_PTR);
        } else {
            let end_ctrl = rd::<usize>(map).wrapping_add(rd::<usize>(map + 0x18));
            wr::<usize>(out_iter, end_ctrl);
            wr::<usize>(out_iter + 0x08, 0);
        }
    }
    unsafe extern "win64" fn mock_factory(ctx: usize, arg2: usize) -> usize {
        ctx.wrapping_add(arg2)
    }
    unsafe extern "win64" fn mock_d6f0(owner: usize, obj: usize) {
        D6F0_SUM = D6F0_SUM.wrapping_add(owner as u64).wrapping_add(obj as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4242_0c10, mock_0c10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4242_48a0, mock_48a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_fab0, mock_fab0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4139_f5c0, mock_f5c0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4241_d6f0, mock_d6f0 as *const ())?;

    let run = |f: F, found: usize, use_factory: bool, init_ref: u32| -> (usize, u32, bool) {
        FOUND_L6 = found;
        D6F0_SUM = 0;
        let mut obj = [0u8; 0x40];
        let op = obj.as_mut_ptr() as usize;
        wr::<u32>(op + 0x24, init_ref);
        wr::<usize>(op + 0x28, 0x9000);

        let mut entry = [0u8; 0x180];
        let ep = entry.as_mut_ptr() as usize;
        if use_factory {
            wr::<usize>(ep + 0x168, op);
            wr::<usize>(ep + 0x170, mock_factory as *const () as usize);
        } else {
            wr::<usize>(ep + 0x158, op);
        }

        let slot: [usize; 2] = [0x42, ep];
        ENTRY_SLOT_PTR = slot.as_ptr() as usize;

        let vtbl: [usize; 2] = [0, mock_get_type as *const () as usize];
        let p2: [usize; 1] = [vtbl.as_ptr() as usize];

        let mut p1 = [0u8; 0x60];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x10, 0x1111);
        wr::<u64>(p1p + 0x18, 0x2222);
        wr::<usize>(p1p + 0x30, 0x5000);
        wr::<usize>(p1p + 0x48, 0x20);

        let ret = f(p1p, p2.as_ptr() as usize, 0x3333, 0x4444_0000_0000, 1);
        let norm_ret = if found != 0 {
            ret.wrapping_sub(p1p)
        } else {
            ret.wrapping_sub(op)
        };
        let called_d6f0 = D6F0_SUM == (0x9000u64).wrapping_add(op as u64);
        (norm_ret, rd::<u32>(op + 0x24), called_d6f0)
    };

    for (found, use_fac, iref) in [(0x80usize, false, 0u32), (0, false, 0), (0, true, 2)] {
        let o = run(f_o, found, use_fac, iref);
        let n = run(f_n, found, use_fac, iref);
        crate::diff_eq!(o, n, "142421ed0 found={} fac={} ref={}", found, use_fac, iref);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_142424b30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FILE_EXISTS: u8 = 1;
    static mut DTOR_COUNT: u32 = 0;
    static mut CSTR_BUF: [u8; 8] = *b"test.bin";

    unsafe extern "win64" fn mock_d770(_p3: usize) {}
    unsafe extern "win64" fn mock_a840(_p3_60: usize, out_opt: usize, key: u64) {
        wr::<u64>(out_opt, 0xaaaa_0000 + key);
        wr::<u64>(out_opt + 0x08, 8);
        wr::<u8>(out_opt + 0x10, 1);
    }
    unsafe extern "win64" fn mock_f850(out_sv: usize, in_sv: usize) {
        wr::<[u64; 2]>(out_sv, rd::<[u64; 2]>(in_sv));
    }
    unsafe extern "win64" fn mock_06c0(out_str: usize, _a: usize, _b: usize) {
        // Inline r::String: b3 = 2 (cap_bits = 16), len = 4 (len_bits = 32 > 16 => heap pointer at +8)
        wr::<u32>(out_str, (2u32 << 24) | 4);
        wr::<u32>(out_str + 0x04, 4);
        wr::<usize>(out_str + 0x08, core::ptr::addr_of!(CSTR_BUF) as usize);
        wr::<u64>(out_str + 0x10, 0);
    }
    unsafe extern "win64" fn mock_1120(_sv: usize) -> u8 {
        FILE_EXISTS
    }
    unsafe extern "win64" fn mock_4570(out_24: usize, _p3: usize, key: u64) {
        wr::<[u64; 3]>(out_24, [key + 10, key + 20, key + 30]);
    }
    unsafe extern "win64" fn mock_cc30(ptr_24: usize) -> u32 {
        (rd::<u64>(ptr_24) ^ rd::<u64>(ptr_24 + 0x08) ^ rd::<u64>(ptr_24 + 0x10)) as u32
    }
    unsafe extern "win64" fn mock_ab10(_s: usize) {
        DTOR_COUNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4243_d770, mock_d770 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_413a_a840, mock_a840 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401d_f850, mock_f850 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401e_06c0, mock_06c0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4249_1120, mock_1120 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4242_4570, mock_4570 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4241_cc30, mock_cc30 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let run = |f: F, key: u64, exists: u8| -> (u8, u8, [u64; 7], u32) {
        FILE_EXISTS = exists;
        DTOR_COUNT = 0;
        CSTR_BUF = *b"test\0\0\0\0";
        let mut entry = [0u64; 7];
        let ep = entry.as_mut_ptr() as usize;
        wr::<u8>(ep, 1);
        wr::<[u64; 3]>(ep + 0x08, [1, 2, 3]);

        // 4 buckets: slot 1 has tombstone (0xdead), slot 2 has key 5 (collides with 1 & 3 == 1, probes +1 -> 2)
        let mut buckets = [u64::MAX; 8];
        buckets[1 * 2] = 0xdead;
        buckets[1 * 2 + 1] = 0;
        buckets[2 * 2] = 5;
        buckets[2 * 2 + 1] = ep as u64;

        let mut map = [0u8; 0x70];
        let mp = map.as_mut_ptr() as usize;
        wr::<u8>(mp + 0x08, 0);
        wr::<u64>(mp + 0x38, 0xdead); // tombstone key
        wr::<u64>(mp + 0x40, 1); // deleted count
        wr::<u64>(mp + 0x48, 2); // used count
        wr::<u64>(mp + 0x50, 4); // capacity
        wr::<u64>(mp + 0x58, u64::MAX); // empty key
        wr::<usize>(mp + 0x68, buckets.as_mut_ptr() as usize);

        let mut p3 = [0u8; 0x70];
        wr::<usize>(p3.as_mut_ptr() as usize + 0x60, 0x1234);

        let ret = f(mp, key, p3.as_ptr() as usize);
        (ret, rd::<u8>(mp + 0x08), entry, DTOR_COUNT)
    };

    for (k, ex) in [(5u64, 1u8), (5, 0), (9, 1)] {
        let o = run(f_o, k, ex);
        let n = run(f_n, k, ex);
        crate::diff_eq!(o, n, "142424b30 k={} ex={}", k, ex);
    }
    Ok(3)
}

pub unsafe fn diff_r_142432e90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ALLOC_BUF: [u64; 8] = [0; 8];
    static mut FREE_COUNT: u32 = 0;
    static mut OBJ_VTBL: [usize; 2] = [0; 2];
    static mut OBJ_INST: [usize; 2] = [0; 2];

    unsafe extern "win64" fn mock_get_id(this: usize) -> u32 {
        rd::<u32>(this + 0x08)
    }
    unsafe extern "win64" fn mock_0e40(sv_ptr: usize) -> usize {
        let len = rd::<u64>(sv_ptr + 0x08);
        if len == 0 {
            0
        } else {
            OBJ_VTBL[1] = mock_get_id as *const () as usize;
            OBJ_INST[0] = core::ptr::addr_of!(OBJ_VTBL) as usize;
            OBJ_INST[1] = (len as usize) * 100;
            core::ptr::addr_of_mut!(OBJ_INST) as usize
        }
    }
    unsafe extern "win64" fn mock_c0a0(_bytes: usize, _align: u32) -> usize {
        core::ptr::addr_of_mut!(ALLOC_BUF) as usize
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {
        FREE_COUNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_424b_0e40, mock_0e40 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;

    let run = |f: F| -> ([u64; 18], [u64; 6], [u64; 2], [u32; 8], u32) {
        ALLOC_BUF = [0; 8];
        FREE_COUNT = 0;
        let mut blob = [0u8; 0x100];
        let bp = blob.as_mut_ptr() as usize;

        // 2 pairs at offset 0x20: pair 0 has len=0 (skipped), pair 1 has len=3 (mapped to 300)
        wr::<u32>(bp + 0x20, 4);
        wr::<u32>(bp + 0x24, 0);
        wr::<u32>(bp + 0x28, 8);
        wr::<u32>(bp + 0x2c, 3);

        // 1 record at offset 0x40 (size 0x20): +0x18 = 0x10 (offset from r12_base=bp+0x60 -> bp+0x70), +0x1c = 1
        wr::<u32>(bp + 0x40 + 0x18, 0x10);
        wr::<u32>(bp + 0x40 + 0x1c, 1);

        // Sub-table at bp + 0x70: count at +4 = 2, items at +8 (want=1 -> found 300) and +16 (want=9 -> 0xffffffff)
        wr::<u32>(bp + 0x70 + 0x04, 2);
        wr::<u32>(bp + 0x70 + 0x08, 1);
        wr::<u32>(bp + 0x70 + 0x10, 9);

        let mut p2 = [0u8; 0x60];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u32>(p2p + 0x18, 0x10);
        wr::<u32>(p2p + 0x1c, 2);
        wr::<u32>(p2p + 0x20, 0x40);
        wr::<u32>(p2p + 0x24, 1);
        wr::<u32>(p2p + 0x28, 0x50);
        wr::<u32>(p2p + 0x2c, 3);
        wr::<u32>(p2p + 0x30, 0x20);
        wr::<u32>(p2p + 0x34, 2);
        wr::<u32>(p2p + 0x38, 0x60);
        wr::<u32>(p2p + 0x3c, 7);
        wr::<u32>(p2p + 0x50, 0x80);
        wr::<u32>(p2p + 0x54, 4);

        let mut p1 = [0u64; 18];
        let mut p3: [u64; 6] = [bp as u64, 0x11, 0x22, 0x33, 0x44, 0x55];
        let mut p4: [u64; 2] = [0xaaaa_bbbb, 0xcccc_dddd];

        let p1p = p1.as_mut_ptr() as usize;
        let ret = f(
            p1p,
            p2p,
            p3.as_mut_ptr() as usize,
            p4.as_mut_ptr() as usize,
        );
        assert_eq!(ret, p1p);

        // Normalize base pointers in p1
        for idx in [3usize, 5, 7, 9, 11, 12] {
            p1[idx] = p1[idx].wrapping_sub(bp as u64);
        }
        let sub_out = rd::<[u32; 8]>(bp + 0x70);
        (p1, p3, p4, sub_out, FREE_COUNT)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "142432e90 out");
    Ok(1)
}

pub unsafe fn diff_r_1424363d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_7e00(out_64: usize, in_sv: usize, _base_str: usize) {
        let sv = rd::<[u64; 2]>(in_sv);
        wr::<u64>(out_64, sv[0]);
        wr::<u64>(out_64 + 0x08, sv[1]);
        wr::<u64>(out_64 + 0x10, 1); // pos = 1
        wr::<u64>(out_64 + 0x30, u64::MAX); // local_28 = -1
    }
    unsafe extern "win64" fn mock_f850(out_sv: usize, in_sv: usize) {
        wr::<[u64; 2]>(out_sv, rd::<[u64; 2]>(in_sv));
    }
    unsafe extern "win64" fn mock_filter(sv_ptr: usize) -> u8 {
        if rd::<u64>(sv_ptr + 0x08) == 99 {
            1
        } else {
            0
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_413a_7e00, mock_7e00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_f850, mock_f850 as *const ())?;

    let path_rel = *b"data/foo";
    let path_abs = *b"/data/foo";

    let run = |f: F, is_abs: bool, reject: bool| -> (u64, u64, u8) {
        let mut p1 = [0u8; 0x48];
        wr::<usize>(
            p1.as_mut_ptr() as usize + 0x38,
            mock_filter as *const () as usize,
        );

        let buf_ptr = if is_abs {
            path_abs.as_ptr() as usize
        } else {
            path_rel.as_ptr() as usize
        };
        let len: u64 = if reject { 99 } else { 8 };
        let in_sv: [u64; 2] = [buf_ptr as u64, len];
        let mut out = [0u64; 3];
        let op = out.as_mut_ptr() as usize;
        let ret = f(p1.as_ptr() as usize, op, in_sv.as_ptr() as usize);
        assert_eq!(ret, op);
        let rel_ptr = if out[0] == 0 {
            0
        } else {
            out[0].wrapping_sub(buf_ptr as u64)
        };
        (rel_ptr, out[1], rd::<u8>(op + 0x10))
    };

    for (abs, rej) in [(false, false), (true, false), (false, true)] {
        let o = run(f_o, abs, rej);
        let n = run(f_n, abs, rej);
        crate::diff_eq!(o, n, "1424363d0 abs={} rej={}", abs, rej);
    }
    Ok(3)
}

pub unsafe fn diff_r_142436c80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut RESOLVE_OK: u8 = 1;

    unsafe extern "win64" fn mock_63d0(_p1: usize, out_res: usize, in_sv: usize) -> usize {
        wr::<[u64; 2]>(out_res, rd::<[u64; 2]>(in_sv));
        wr::<u8>(out_res + 0x10, RESOLVE_OK);
        out_res
    }
    unsafe extern "win64" fn mock_5ac0(in_sv: usize, out_sv: usize) -> usize {
        wr::<[u64; 2]>(out_sv, rd::<[u64; 2]>(in_sv));
        out_sv
    }
    unsafe extern "win64" fn mock_2c60(pack: usize, _dir_sv: usize) -> usize {
        if pack == 0x2000 {
            0xabc0
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_fa80(in_sv: usize, out_sv: usize) -> usize {
        wr::<[u64; 2]>(out_sv, rd::<[u64; 2]>(in_sv));
        out_sv
    }
    unsafe extern "win64" fn mock_2790(pack: usize, dir_node: usize, sv_ptr: usize) -> usize {
        if pack == 0x2000 && dir_node == 0xabc0 && rd::<u64>(sv_ptr + 0x08) == 5 {
            1
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_fallback(_this: usize, sv_ptr: usize) -> u8 {
        if rd::<u64>(sv_ptr + 0x08) == 5 {
            1
        } else {
            0
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4243_63d0, mock_63d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_40f3_5ac0, mock_5ac0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4243_2c60, mock_2c60 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401d_fa80, mock_fa80 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4243_2790, mock_2790 as *const ())?;

    let run = |f: F, resolve_ok: u8, sv_len: u64| -> u8 {
        RESOLVE_OK = resolve_ok;
        let mut fb_vtbl = [0usize; 8];
        fb_vtbl[0x30 / 8] = mock_fallback as *const () as usize;
        let fb_obj: [usize; 1] = [fb_vtbl.as_ptr() as usize];

        let packs: [usize; 2] = [0x1000, 0x2000];
        let mut fs = [0u8; 0x50];
        let fsp = fs.as_mut_ptr() as usize;
        wr::<usize>(fsp + 0x10, packs.as_ptr() as usize);
        wr::<u32>(fsp + 0x18, 2);
        wr::<usize>(fsp + 0x40, fb_obj.as_ptr() as usize);

        let sv: [u64; 2] = [0x5555_0000, sv_len];
        f(fsp, sv.as_ptr() as usize)
    };

    for (rok, slen) in [(1u8, 5u64), (1, 9), (0, 5), (0, 9)] {
        let o = run(f_o, rok, slen);
        let n = run(f_n, rok, slen);
        crate::diff_eq!(o, n, "142436c80 rok={} slen={}", rok, slen);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_14243b3e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_tag = exe::ptr::<u16>(0x1_4397_d8fa) as usize;
    let saved_tag = rd::<u16>(g_tag);

    let run = |f: F, in_second: bool| -> (u32, u32, u64, u64, u16) {
        wr::<u16>(g_tag, 0x1234);
        let mut node_slot: u64 = 0;
        let np = (&mut node_slot as *mut u64) as usize as u64;

        let mut arena2 = [0u8; 0x40];
        let a2p = arena2.as_mut_ptr() as usize;
        wr::<u32>(a2p, 10);
        wr::<u64>(a2p + 0x18, 0xabcd_0000_1111);
        wr::<u64>(a2p + 0x20, np);
        wr::<u64>(a2p + 0x28, np + 0x100);
        wr::<usize>(a2p + 0x38, 0);

        let mut arena1 = [0u8; 0x40];
        let a1p = arena1.as_mut_ptr() as usize;
        wr::<u32>(a1p, 5);
        wr::<u64>(a1p + 0x18, 0x1111);
        wr::<u64>(a1p + 0x20, 0x1000);
        wr::<u64>(a1p + 0x28, 0x2000);
        wr::<usize>(a1p + 0x38, a2p);

        let target_ptr = if in_second { np } else { 0x3000 };
        f(a1p, target_ptr);

        let head2 = rd::<u64>(a2p + 0x18);
        let head2_norm = if in_second {
            head2.wrapping_sub(np)
        } else {
            head2
        };
        (
            rd::<u32>(a1p),
            rd::<u32>(a2p),
            head2_norm,
            node_slot,
            rd::<u16>(g_tag),
        )
    };

    for in_sec in [true, false] {
        let o = run(f_o, in_sec);
        let n = run(f_n, in_sec);
        wr::<u16>(g_tag, saved_tag);
        crate::diff_eq!(o, n, "14243b3e0 in_sec={}", in_sec);
    }
    Ok(2)
}

pub unsafe fn diff_r_14243d770(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut E3D0_SUM: u64 = 0;
    static mut AB10_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_a730(dst: usize, src: usize) {
        wr::<[u64; 3]>(dst, rd::<[u64; 3]>(src));
    }
    unsafe extern "win64" fn mock_ab10(_s: usize) {
        AB10_COUNT += 1;
    }
    unsafe extern "win64" fn mock_e3d0(p1: usize, _out_buf: usize, sv_ptr: usize) {
        let ptr = rd::<usize>(sv_ptr);
        let len = rd::<u64>(sv_ptr + 0x08);
        let b0 = rd::<u8>(ptr) as u64;
        E3D0_SUM = E3D0_SUM
            .wrapping_add(rd::<u64>(p1))
            .wrapping_add(len)
            .wrapping_add(b0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_a730, mock_a730 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4243_e3d0, mock_e3d0 as *const ())?;

    let run = |f: F| -> (u32, u32, u64, u32) {
        E3D0_SUM = 0;
        AB10_COUNT = 0;
        let mut ring_elems = [0u8; 24 * 2];
        let rp = ring_elems.as_mut_ptr() as usize;
        // Elem 1 (at head=1, cap=2 -> wraps to 0): inline r::String (b3=2 => cap_bits=16, len=1 => len_bits=8 <= 16)
        let e1 = rp + 24;
        wr::<u32>(e1, (2u32 << 24) | 1);
        wr::<u32>(e1 + 0x04, 5);
        wr::<u8>(e1 + 0x08, b'Z');

        let mut q = [0u8; 0x30];
        let qp = q.as_mut_ptr() as usize;
        wr::<usize>(qp + 0x10, rp);
        wr::<u32>(qp + 0x18, 1); // head = 1
        wr::<u32>(qp + 0x20, 1); // count = 1
        wr::<u32>(qp + 0x24, 2); // cap = 2

        let mut p1 = [0u64; 4];
        p1[0] = 0x700;
        p1[2] = qp as u64;
        f(p1.as_ptr() as usize);

        (
            rd::<u32>(qp + 0x18),
            rd::<u32>(qp + 0x20),
            E3D0_SUM,
            AB10_COUNT,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14243d770 out");
    Ok(1)
}

pub unsafe fn diff_r_14243ebc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FD50_CALLS: u32 = 0;
    static mut D770_CALLS: u32 = 0;
    static mut AD10_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_d770(_p1: usize) {
        D770_CALLS += 1;
    }
    unsafe extern "win64" fn mock_fd50(
        _r12: usize,
        _key: u64,
        ver: u32,
        p3_ptr: usize,
        _p4_ptr: usize,
    ) -> u64 {
        FD50_CALLS += 1;
        let c = FD50_CALLS;
        if ver != 0 || c >= 2 {
            rd::<u64>(p3_ptr)
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_fee0(_r12: usize, _key: u64, _ver: u32) -> u8 {
        0
    }
    unsafe extern "win64" fn mock_a840(_p60: usize, out_opt: usize, key: u64) {
        wr::<u64>(out_opt, key + 0x100);
        wr::<u64>(out_opt + 0x08, 4);
        wr::<u8>(out_opt + 0x10, 1);
    }
    unsafe extern "win64" fn mock_a940(_p68: usize, out_ver: usize, _sv: usize) {
        wr::<u32>(out_ver, 7);
        wr::<u8>(out_ver + 0x04, 1);
    }
    unsafe extern "win64" fn mock_ae10(_p78: usize, _key: u64, _ver: u32) -> u8 {
        0
    }
    unsafe extern "win64" fn mock_ad10(_p78: usize, _key: u64, _sv: usize, _timeout: u32) {
        AD10_CALLS += 1;
    }
    unsafe extern "win64" fn mock_b1b0(_p78: usize, _key: u64, _ver: u32) -> u8 {
        1
    }

    let _m1 = crate::difftest::Mock::new(0x1_4243_d770, mock_d770 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4243_fd50, mock_fd50 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4243_fee0, mock_fee0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_413a_a840, mock_a840 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_413a_a940, mock_a940 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_413a_ae10, mock_ae10 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_413a_ad10, mock_ad10 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_413a_b1b0, mock_b1b0 as *const ())?;

    let run = |f: F, mode_8: u32| -> (u8, u32, u32, u32) {
        FD50_CALLS = 0;
        D770_CALLS = 0;
        AD10_CALLS = 0;
        let mut p1 = [0u8; 0x80];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1p + 0x08, mode_8);
        wr::<usize>(p1p + 0x58, 0x1111);
        wr::<usize>(p1p + 0x60, 0x2222);
        wr::<usize>(p1p + 0x68, 0x3333);
        wr::<usize>(p1p + 0x78, 0x4444);

        let p3: [u64; 2] = [0x80, 0x9000];
        let p4: [u64; 2] = [0x10, 0xa000];

        let ret = f(p1p, 0x5555, p3.as_ptr() as usize, p4.as_ptr() as usize);
        (ret, FD50_CALLS, D770_CALLS, AD10_CALLS)
    };

    for m in [0u32, 1] {
        let o = run(f_o, m);
        let n = run(f_n, m);
        crate::diff_eq!(o, n, "14243ebc0 mode={}", m);
    }
    Ok(2)
}

pub unsafe fn diff_r_14243ede0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_HASH: u64 = 0;

    unsafe extern "win64" fn mock_5780(g_val: u64, ptr_58: usize) {
        let tag = rd::<u32>(ptr_58) as u64;
        LOG_HASH = LOG_HASH.wrapping_add(g_val).wrapping_add(tag);
    }
    unsafe extern "win64" fn mock_fee0(_p58: usize, key: u64, _ver: u32) -> u8 {
        if key == 100 {
            1
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_ffd0(p58: usize, key: u64, ver: u32, span_ptr: usize) {
        let s = rd::<[u64; 2]>(span_ptr);
        LOG_HASH = LOG_HASH
            .wrapping_add(p58 as u64)
            .wrapping_add(key)
            .wrapping_add(ver as u64)
            .wrapping_add(s[0])
            .wrapping_add(s[1]);
    }
    unsafe extern "win64" fn mock_e830(p18: usize) {
        wr::<u32>(p18, 0xdead);
    }
    unsafe extern "win64" fn mock_ca40(p28: usize) {
        wr::<u32>(p28, 0xbeef);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_5780, mock_5780 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4243_fee0, mock_fee0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4243_ffd0, mock_ffd0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_413a_ca40, mock_ca40 as *const ())?;

    let g_2f8 = exe::ptr::<u64>(0x1_43a0_82f8) as usize;
    let saved_2f8 = rd::<u64>(g_2f8);
    wr::<u64>(g_2f8, 0x7777);

    let run = |f: F| -> (u64, u32, u32) {
        LOG_HASH = 0;
        let mut items = [0u8; 0x88 * 2];
        let ip = items.as_mut_ptr() as usize;
        // Item 0: active (+0x80 = 1), key=200 (not in fee0 -> calls ffd0)
        wr::<u64>(ip, 200);
        wr::<u32>(ip + 0x08, 3);
        wr::<u64>(ip + 0x70, 0xabc0_0000);
        wr::<u32>(ip + 0x78, 64);
        wr::<u8>(ip + 0x80, 1);

        // Item 1: active (+0x80 = 1), key=100 (in fee0 -> skipped)
        let i1 = ip + 0x88;
        wr::<u64>(i1, 100);
        wr::<u32>(i1 + 0x08, 2);
        wr::<u8>(i1 + 0x80, 1);

        let mut rsi = [0u8; 0x70];
        let rp = rsi.as_mut_ptr() as usize;
        wr::<usize>(rp + 0x28, ip);
        wr::<u32>(rp + 0x30, 2);
        wr::<i32>(rp + 0x38, 0); // no active workers to wait on
        wr::<u32>(rp + 0x58, 0x42);

        let mut p1 = [0u8; 0x90];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x58, 0x5000);
        wr::<usize>(p1p + 0x80, rp);

        f(p1p);
        (LOG_HASH, rd::<u32>(rp + 0x18), rd::<u32>(rp + 0x28))
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u64>(g_2f8, saved_2f8);
    crate::diff_eq!(o, n, "14243ede0 out");
    Ok(1)
}

pub unsafe fn diff_r_14243f5b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut DESTROYED_SUM: u64 = 0;

    unsafe extern "win64" fn mock_dtor(this: usize, flag: u32) {
        DESTROYED_SUM = DESTROYED_SUM.wrapping_add(rd::<u32>(this + 0x08) as u64 + flag as u64);
    }
    unsafe extern "win64" fn mock_get_id(this: usize) -> u32 {
        rd::<u32>(this + 0x08)
    }
    unsafe extern "win64" fn mock_2de0(_ptr: usize) {
        DESTROYED_SUM = DESTROYED_SUM.wrapping_add(0x1000);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;

    let run = |f: F, use_freelist: bool| -> ([u32; 16], u32, i32, i32, [u64; 2], u64) {
        DESTROYED_SUM = 0;
        let mut vtbl = [0usize; 20];
        vtbl[0] = mock_dtor as *const () as usize;
        vtbl[0x98 / 8] = mock_get_id as *const () as usize;

        let obj_old: [usize; 2] = [vtbl.as_ptr() as usize, 77];
        let obj1: [usize; 2] = [vtbl.as_ptr() as usize, 11];
        let obj2: [usize; 2] = [vtbl.as_ptr() as usize, 22];

        let mut pool = [0u32; 16]; // 4 slots of 16 bytes
        let pp = pool.as_mut_ptr() as usize;
        if use_freelist {
            // Slot 1 is on freelist with old object to destroy and next free = 2
            wr::<usize>(pp + 16, obj_old.as_ptr() as usize);
            wr::<u32>(pp + 16 + 0x0c, 2);
            // Slot 2 is on freelist with next free = -1
            wr::<i32>(pp + 32 + 0x0c, -1);
        }

        let mut p1 = [0u8; 0x80];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x68, pp);
        wr::<u32>(p1p + 0x70, if use_freelist { 3 } else { 0 });
        wr::<u32>(p1p + 0x74, 4);
        wr::<i32>(p1p + 0x78, if use_freelist { 1 } else { -1 });

        let mut p2: [i32; 2] = [0, 99];
        let mut items: [u64; 2] = [obj1.as_ptr() as u64, obj2.as_ptr() as u64];
        let p3: [u64; 2] = [2, items.as_mut_ptr() as u64];

        f(p1p, p2.as_mut_ptr() as usize, p3.as_ptr() as usize);

        // Normalize object pointers in pool to their IDs
        for i in 0..4usize {
            let p = rd::<usize>(pp + i * 16);
            let id = if p == 0 { 0 } else { rd::<u32>(p + 0x08) as usize };
            wr::<usize>(pp + i * 16, id);
        }

        (
            pool,
            rd::<u32>(p1p + 0x70),
            rd::<i32>(p1p + 0x78),
            p2[1],
            items,
            DESTROYED_SUM,
        )
    };

    for fl in [false, true] {
        let o = run(f_o, fl);
        let n = run(f_n, fl);
        crate::diff_eq!(o, n, "14243f5b0 fl={}", fl);
    }
    Ok(2)
}

pub unsafe fn diff_r_14243f750(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut BE30_CALLS: u32 = 0;
    static mut DESTROYED_SUM: u64 = 0;

    unsafe extern "win64" fn mock_dtor(this: usize, _flag: u32) {
        DESTROYED_SUM = DESTROYED_SUM.wrapping_add(rd::<u32>(this + 0x08) as u64);
    }
    unsafe extern "win64" fn mock_get_id(this: usize) -> u32 {
        rd::<u32>(this + 0x08)
    }
    unsafe extern "win64" fn mock_be30(_p80: usize, _slot: usize) {
        BE30_CALLS += 1;
    }
    unsafe extern "win64" fn mock_2de0(_ptr: usize) {
        DESTROYED_SUM = DESTROYED_SUM.wrapping_add(0x100);
    }

    let _m1 = crate::difftest::Mock::new(0x1_413a_be30, mock_be30 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;

    let run = |f: F| -> ([u32; 12], i32, i32, u64, u32, u64) {
        BE30_CALLS = 0;
        DESTROYED_SUM = 0;
        let mut vtbl = [0usize; 20];
        vtbl[0] = mock_dtor as *const () as usize;
        vtbl[0x98 / 8] = mock_get_id as *const () as usize;

        let old_obj0: [usize; 2] = [vtbl.as_ptr() as usize, 50];
        let cand_a: [usize; 2] = [vtbl.as_ptr() as usize, 10];
        let cand_b: [usize; 2] = [vtbl.as_ptr() as usize, 99];

        // 3 slots in pool:
        // Slot 0: type_id=10 (matches cand_a), next=1
        // Slot 1: type_id=20 (no match -> unlinked and pushed to freelist), next=-1
        let mut pool = [0u32; 12];
        let pp = pool.as_mut_ptr() as usize;
        wr::<usize>(pp, old_obj0.as_ptr() as usize);
        wr::<u32>(pp + 0x08, 10);
        wr::<i32>(pp + 0x0c, 1);

        wr::<usize>(pp + 16, 0);
        wr::<u32>(pp + 16 + 0x08, 20);
        wr::<i32>(pp + 16 + 0x0c, -1);

        let mut p1 = [0u8; 0x90];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x68, pp);
        wr::<i32>(p1p + 0x78, -1);

        let mut p2 = [0u64; 2];
        let mut p3: [i32; 2] = [0, 0]; // starts at slot 0
        let mut cands: [u64; 2] = [cand_a.as_ptr() as u64, cand_b.as_ptr() as u64];
        let mut p4: [u64; 2] = [2, cands.as_mut_ptr() as u64];

        let p2p = p2.as_mut_ptr() as usize;
        let ret = f(
            p1p,
            p2p,
            p3.as_mut_ptr() as usize,
            p4.as_mut_ptr() as usize,
        );
        assert_eq!(ret, p2p);

        for i in 0..3usize {
            let p = rd::<usize>(pp + i * 16);
            let id = if p == 0 { 0 } else { rd::<u32>(p + 0x08) as usize };
            wr::<usize>(pp + i * 16, id);
        }

        (
            pool,
            rd::<i32>(p1p + 0x78),
            p3[1],
            p2[0],
            BE30_CALLS,
            DESTROYED_SUM,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14243f750 out");
    Ok(1)
}

pub unsafe fn diff_r_14243fee0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u32) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, key: u64, min_ver: u32| -> u8 {
        // 4 buckets: slot 1 has tombstone (0xdead), slot 2 has key 5 (1 & 3 == 1, probes +1 -> 2) with ver=10
        let mut buckets = [u64::MAX; 8];
        buckets[1 * 2] = 0xdead;
        buckets[1 * 2 + 1] = 0;
        buckets[2 * 2] = 5;
        buckets[2 * 2 + 1] = 10;

        let mut map = [0u8; 0x70];
        let mp = map.as_mut_ptr() as usize;
        wr::<u64>(mp + 0x30, 0xdead); // tombstone key
        wr::<u64>(mp + 0x38, 1); // deleted count
        wr::<u64>(mp + 0x40, 2); // used count
        wr::<u64>(mp + 0x48, 4); // capacity
        wr::<u64>(mp + 0x50, u64::MAX); // empty key
        wr::<usize>(mp + 0x60, buckets.as_mut_ptr() as usize);

        f(mp, key, min_ver)
    };

    for (k, ver) in [(5u64, 5u32), (5, 10), (5, 11), (9, 1)] {
        let o = run(f_o, k, ver);
        let n = run(f_n, k, ver);
        crate::diff_eq!(o, n, "14243fee0 k={} ver={}", k, ver);
    }
    Ok(4)
}

// REGISTRY:
// Replacement::verified_throws(0x1_4242_1ed0, "r_142421ed0", rend::cauldron_hot_y6::r_142421ed0 as *const (), Throws::Discard, rend::cauldron_hot_y6::diff_r_142421ed0),
// Replacement::verified_throws(0x1_4242_4b30, "unknown_142424b30", rend::cauldron_hot_y6::unknown_142424b30 as *const (), Throws::Caught, rend::cauldron_hot_y6::diff_unknown_142424b30),
// Replacement::verified_throws(0x1_4243_2e90, "r_142432e90", rend::cauldron_hot_y6::r_142432e90 as *const (), Throws::Caught, rend::cauldron_hot_y6::diff_r_142432e90),
// Replacement::verified_throws(0x1_4243_63d0, "r_1424363d0", rend::cauldron_hot_y6::r_1424363d0 as *const (), Throws::Discard, rend::cauldron_hot_y6::diff_r_1424363d0),
// Replacement::verified_throws(0x1_4243_6c80, "r_142436c80", rend::cauldron_hot_y6::r_142436c80 as *const (), Throws::Caught, rend::cauldron_hot_y6::diff_r_142436c80),
// Replacement::verified(0x1_4243_b3e0, "unknown_14243b3e0", rend::cauldron_hot_y6::unknown_14243b3e0 as *const (), rend::cauldron_hot_y6::diff_unknown_14243b3e0),
// Replacement::verified_throws(0x1_4243_d770, "r_14243d770", rend::cauldron_hot_y6::r_14243d770 as *const (), Throws::Caught, rend::cauldron_hot_y6::diff_r_14243d770),
// Replacement::verified_throws(0x1_4243_ebc0, "r_14243ebc0", rend::cauldron_hot_y6::r_14243ebc0 as *const (), Throws::Discard, rend::cauldron_hot_y6::diff_r_14243ebc0),
// Replacement::verified_throws(0x1_4243_ede0, "r_14243ede0", rend::cauldron_hot_y6::r_14243ede0 as *const (), Throws::Caught, rend::cauldron_hot_y6::diff_r_14243ede0),
// Replacement::verified(0x1_4243_f5b0, "r_14243f5b0", rend::cauldron_hot_y6::r_14243f5b0 as *const (), rend::cauldron_hot_y6::diff_r_14243f5b0),
// Replacement::verified(0x1_4243_f750, "r_14243f750", rend::cauldron_hot_y6::r_14243f750 as *const (), rend::cauldron_hot_y6::diff_r_14243f750),
// Replacement::verified(0x1_4243_fee0, "r_14243fee0", rend::cauldron_hot_y6::r_14243fee0 as *const (), rend::cauldron_hot_y6::diff_r_14243fee0),
