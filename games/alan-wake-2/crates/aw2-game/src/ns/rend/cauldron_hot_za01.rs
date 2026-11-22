//! Decompiled hot functions shard ZA01 (`0x1401df790..0x140348930`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn free_mimalloc_block(ptr: usize) {
    let chunk_base = ptr.wrapping_sub(1) & !0x01ff_ffffusize;
    let page_idx = (ptr.wrapping_sub(chunk_base)) >> 16;
    let desc_hdr = chunk_base
        .wrapping_add(0x108)
        .wrapping_add(page_idx.wrapping_mul(0x50));
    let desc = desc_hdr.wrapping_sub(rd::<u32>(desc_hdr + 0x04) as usize);
    if rd::<usize>(chunk_base + 0x100) == exe::teb() {
        if rd::<u8>(desc + 0x0e) == 0 {
            wr::<usize>(ptr, rd::<usize>(desc + 0x20));
            let new_cnt = rd::<u32>(desc + 0x18).wrapping_sub(1);
            wr::<u32>(desc + 0x18, new_cnt);
            wr::<usize>(desc + 0x20, ptr);
            if new_cnt == 0 {
                exe::call_va!(0x1_4253_71a0, (desc));
            }
        } else {
            exe::call_va!(0x1_4253_3200, (chunk_base, desc, 1u8, ptr));
        }
    } else {
        exe::call_va!(0x1_4253_3200, (chunk_base, desc, 0u32, ptr));
    }
}

/// `unknown_1401df790`
/// Original: 0x1401df790 (105 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1401df790(param_1: usize) -> usize {
    wr::<u64>(param_1, 0);
    wr::<u64>(param_1 + 0x08, 0);
    wr::<u64>(param_1 + 0x10, 0);
    wr::<u64>(param_1 + 0x18, 0);
    wr::<u8>(param_1 + 0x20, 0);

    let mut qpc: i64 = 0;
    exe::call_iat!(0x1_4264_56f8, ((&mut qpc as *mut i64) as usize));
    wr::<i64>(param_1, qpc);

    let freq = exe::call_va!(0x1_4240_6920, () -> i64);
    let delta = ((freq as f64) * 0.0f64) as i64;
    wr::<i64>(param_1, rd::<i64>(param_1).wrapping_sub(delta));
    wr::<u64>(param_1 + 0x08, 0);
    wr::<u64>(param_1 + 0x10, 0);
    param_1
}

/// `sparsehash_1401eb860`
/// Original: 0x1401eb860 (706 bytes, Throws::Discard)
pub unsafe extern "win64" fn sparsehash_1401eb860(param_1: usize, param_2: usize) -> usize {
    let mut pos_pair = [0u64; 2];
    exe::call_va!(
        0x1_401e_8680,
        (param_1, pos_pair.as_mut_ptr() as usize, param_2)
    );
    if pos_pair[0] != u64::MAX {
        return rd::<usize>(param_1 + 0x78).wrapping_add((pos_pair[0] as usize).wrapping_mul(0x28));
    }

    let resized = exe::call_va!(0x1_401e_bb50, (param_1) -> u8);
    if resized == 0 {
        let num_elements = rd::<u64>(param_1 + 0x40);
        let num_deleted = rd::<u64>(param_1 + 0x38);
        let key = rd::<[u64; 2]>(param_2);
        if num_elements.wrapping_sub(num_deleted) >= 0x0666_6666_6666_6666 {
            let mut exc_buf = [0u64; 8];
            let msg_ptr = exe::ptr::<u8>(0x1_4307_aa70) as usize;
            let ti_ptr = exe::ptr::<u8>(0x1_437f_cf88) as usize;
            exe::call_va!(0x1_401d_f630, (exc_buf.as_mut_ptr() as usize, msg_ptr));
            exe::call_va!(0x1_4183_acce, (exc_buf.as_mut_ptr() as usize, ti_ptr));
            return 0;
        }
        let ins_idx = pos_pair[1] as usize;
        let table = rd::<usize>(param_1 + 0x78);
        let slot = table.wrapping_add(ins_idx.wrapping_mul(0x28));
        if num_deleted != 0
            && rd::<u64>(param_1 + 0x28) == rd::<u64>(slot)
            && rd::<u64>(param_1 + 0x30) == rd::<u64>(slot + 0x08)
        {
            wr::<u64>(param_1 + 0x38, num_deleted.wrapping_sub(1));
        } else {
            wr::<u64>(param_1 + 0x40, num_elements.wrapping_add(1));
        }
        wr::<u64>(slot, key[0]);
        wr::<u64>(slot + 0x08, key[1]);
        wr::<u64>(slot + 0x10, 0);
        wr::<u64>(slot + 0x18, 0);
        wr::<u64>(slot + 0x20, 0);
        return rd::<usize>(param_1 + 0x78).wrapping_add(ins_idx.wrapping_mul(0x28));
    }

    let key = rd::<[u64; 2]>(param_2);
    let mask = rd::<u64>(param_1 + 0x48).wrapping_sub(1);
    let table = rd::<usize>(param_1 + 0x78);
    let empty0 = rd::<u64>(param_1 + 0x50);
    let mut probes: u64 = 0;
    let mut ins_pos: u64 = u64::MAX;
    let mut bucket: u64 = key[0] ^ key[1];

    loop {
        bucket &= mask;
        let slot = table.wrapping_add((bucket as usize).wrapping_mul(0x28));
        let k0 = rd::<u64>(slot);
        if empty0 == k0 && rd::<u64>(param_1 + 0x58) == rd::<u64>(slot + 0x08) {
            if ins_pos == u64::MAX {
                ins_pos = bucket;
            }
            break;
        }
        if rd::<u64>(param_1 + 0x38) > 0
            && rd::<u64>(param_1 + 0x28) == k0
            && rd::<u64>(param_1 + 0x30) == rd::<u64>(slot + 0x08)
        {
            if ins_pos == u64::MAX {
                ins_pos = bucket;
            }
            probes = probes.wrapping_add(1);
            bucket = bucket.wrapping_add(probes);
            continue;
        }
        if key[0] == k0 && key[1] == rd::<u64>(slot + 0x08) {
            ins_pos = u64::MAX;
            if bucket != u64::MAX {
                return table.wrapping_add((bucket as usize).wrapping_mul(0x28));
            }
            break;
        }
        probes = probes.wrapping_add(1);
        bucket = bucket.wrapping_add(probes);
    }

    let num_elements = rd::<u64>(param_1 + 0x40);
    let num_deleted = rd::<u64>(param_1 + 0x38);
    if num_elements.wrapping_sub(num_deleted) >= 0x0666_6666_6666_6666 {
        let mut exc_buf = [0u64; 8];
        let msg_ptr = exe::ptr::<u8>(0x1_4307_aa70) as usize;
        let ti_ptr = exe::ptr::<u8>(0x1_437f_cf88) as usize;
        exe::call_va!(0x1_401d_f630, (exc_buf.as_mut_ptr() as usize, msg_ptr));
        exe::call_va!(0x1_4183_acce, (exc_buf.as_mut_ptr() as usize, ti_ptr));
        return 0;
    }

    let ins_idx = ins_pos as usize;
    let slot = table.wrapping_add(ins_idx.wrapping_mul(0x28));
    if num_deleted != 0
        && rd::<u64>(param_1 + 0x28) == rd::<u64>(slot)
        && rd::<u64>(param_1 + 0x30) == rd::<u64>(slot + 0x08)
    {
        wr::<u64>(param_1 + 0x38, num_deleted.wrapping_sub(1));
    } else {
        wr::<u64>(param_1 + 0x40, num_elements.wrapping_add(1));
    }
    wr::<u64>(slot, key[0]);
    wr::<u64>(slot + 0x08, key[1]);
    wr::<u64>(slot + 0x10, 0);
    wr::<u64>(slot + 0x18, 0);
    wr::<u64>(slot + 0x20, 0);
    rd::<usize>(param_1 + 0x78).wrapping_add(ins_idx.wrapping_mul(0x28))
}

/// `unknown_1401f1a80`
/// Original: 0x1401f1a80 (394 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1401f1a80(
    param_1: usize,
    param_2: usize,
    _param_3: usize,
    param_4: usize,
    param_5: u8,
) {
    let end_ptr = rd::<usize>(param_1 + 0x78)
        .wrapping_add((rd::<u64>(param_1 + 0x48) as usize).wrapping_mul(0x28));
    let mut entry = end_ptr;
    if rd::<u64>(param_1 + 0x40) != rd::<u64>(param_1 + 0x38) {
        let mut pos_pair = [0u64; 2];
        exe::call_va!(
            0x1_401e_8680,
            (param_1, pos_pair.as_mut_ptr() as usize, param_2)
        );
        if pos_pair[0] != u64::MAX {
            entry = rd::<usize>(param_1 + 0x78)
                .wrapping_add((pos_pair[0] as usize).wrapping_mul(0x28));
        }
    }

    let end_check = rd::<usize>(param_1 + 0x78)
        .wrapping_add((rd::<u64>(param_1 + 0x48) as usize).wrapping_mul(0x28));
    if entry != end_check {
        let val = rd::<i32>(param_4) as f64;
        let id = rd::<u32>(entry + 0x14);
        wr::<u16>(entry + 0x10, 1);
        wr::<u8>(entry + 0x13, 0xff);
        wr::<f64>(entry + 0x18, val);
        let word_idx = (id >> 11) as usize;
        let bit_idx = (id >> 5) & 0x3f;
        let word_addr = param_1 + 0x118 + word_idx * 8;
        wr::<u64>(word_addr, rd::<u64>(word_addr) | (1u64 << bit_idx));
        wr::<u8>(entry + 0x12, rd::<u8>(entry + 0x12) | param_5);
        return;
    }

    let free_base = rd::<usize>(param_1 + 0x100);
    let free_cnt = rd::<u32>(param_1 + 0x108);
    let free_end = free_base.wrapping_add((free_cnt as usize) * 4);
    let id = if free_base != free_end {
        let v = rd::<u32>(free_end - 4);
        wr::<u32>(param_1 + 0x108, free_cnt.wrapping_sub(1));
        v
    } else {
        let v = rd::<u32>(param_1 + 0x110);
        wr::<u32>(param_1 + 0x110, v.wrapping_add(1));
        v
    };

    let val = rd::<i32>(param_4) as f64;
    let word_idx = (id >> 11) as usize;
    let bit_idx = (id >> 5) & 0x3f;
    let word_addr = param_1 + 0x118 + word_idx * 8;
    wr::<u64>(word_addr, rd::<u64>(word_addr) | (1u64 << bit_idx));

    let slot = exe::call_va!(0x1_401e_b860, (param_1, param_2) -> usize);
    wr::<u16>(slot + 0x10, 1);
    wr::<u8>(slot + 0x12, param_5);
    wr::<u8>(slot + 0x13, 0xff);
    wr::<u32>(slot + 0x14, id);
    wr::<f64>(slot + 0x18, val);
    wr::<u64>(slot + 0x20, 0);
}

/// `coregame_1401f3100`
/// Original: 0x1401f3100 (181 bytes, Throws::No)
pub unsafe extern "win64" fn coregame_1401f3100(param_1: usize, param_2: usize) -> usize {
    wr::<u64>(param_1, rd::<u64>(param_2));
    wr::<u32>(param_1 + 0x08, rd::<u32>(param_2 + 0x08));
    wr::<[u64; 2]>(param_1 + 0x10, rd::<[u64; 2]>(param_2 + 0x10));

    let dst_arr = param_1 + 0x20;
    let src_arr = param_2 + 0x20;
    wr::<u64>(param_1 + 0x40, 0);
    if dst_arr != src_arr {
        let cnt = rd::<u64>(param_2 + 0x40) as usize;
        for i in 0..cnt {
            wr::<u64>(dst_arr + i * 8, rd::<u64>(src_arr + i * 8));
        }
        wr::<u64>(param_1 + 0x40, rd::<u64>(param_2 + 0x40));
    }

    let dst_str = param_1 + 0x48;
    let hdr = (rd::<u32>(dst_str) & 0xff00_0008) | 0x08;
    wr::<u32>(dst_str, hdr);
    wr::<u8>(dst_str + 0x03, 0x08);
    wr::<u32>(dst_str + 0x04, 0);
    wr::<u8>(dst_str + 0x08, 0);
    exe::call_va!(0x1_4240_aa00, (dst_str, param_2 + 0x48));
    param_1
}

/// `unknown_1401f3a20`
/// Original: 0x1401f3a20 (384 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1401f3a20(
    param_1: usize,
    param_2: usize,
    _param_3: usize,
    param_4: usize,
    param_5: u8,
) {
    let end_ptr = rd::<usize>(param_1 + 0x78)
        .wrapping_add((rd::<u64>(param_1 + 0x48) as usize).wrapping_mul(0x28));
    let mut entry = end_ptr;
    if rd::<u64>(param_1 + 0x40) != rd::<u64>(param_1 + 0x38) {
        let mut pos_pair = [0u64; 2];
        exe::call_va!(
            0x1_401e_8680,
            (param_1, pos_pair.as_mut_ptr() as usize, param_2)
        );
        if pos_pair[0] != u64::MAX {
            entry = rd::<usize>(param_1 + 0x78)
                .wrapping_add((pos_pair[0] as usize).wrapping_mul(0x28));
        }
    }

    let end_check = rd::<usize>(param_1 + 0x78)
        .wrapping_add((rd::<u64>(param_1 + 0x48) as usize).wrapping_mul(0x28));
    if entry != end_check {
        let val = rd::<u64>(param_4);
        let id = rd::<u32>(entry + 0x14);
        wr::<u16>(entry + 0x10, 1);
        wr::<u64>(entry + 0x18, val);
        wr::<u8>(entry + 0x13, 0xff);
        let word_idx = (id >> 11) as usize;
        let bit_idx = (id >> 5) & 0x3f;
        let word_addr = param_1 + 0x118 + word_idx * 8;
        wr::<u64>(word_addr, rd::<u64>(word_addr) | (1u64 << bit_idx));
        wr::<u8>(entry + 0x12, rd::<u8>(entry + 0x12) | param_5);
        return;
    }

    let free_base = rd::<usize>(param_1 + 0x100);
    let free_cnt = rd::<u32>(param_1 + 0x108);
    let free_end = free_base.wrapping_add((free_cnt as usize) * 4);
    let id = if free_base != free_end {
        let v = rd::<u32>(free_end - 4);
        wr::<u32>(param_1 + 0x108, free_cnt.wrapping_sub(1));
        v
    } else {
        let v = rd::<u32>(param_1 + 0x110);
        wr::<u32>(param_1 + 0x110, v.wrapping_add(1));
        v
    };

    let val = rd::<u64>(param_4);
    let word_idx = (id >> 11) as usize;
    let bit_idx = (id >> 5) & 0x3f;
    let word_addr = param_1 + 0x118 + word_idx * 8;
    wr::<u64>(word_addr, rd::<u64>(word_addr) | (1u64 << bit_idx));

    let slot = exe::call_va!(0x1_401e_b860, (param_1, param_2) -> usize);
    wr::<u16>(slot + 0x10, 1);
    wr::<u8>(slot + 0x12, param_5);
    wr::<u8>(slot + 0x13, 0xff);
    wr::<u32>(slot + 0x14, id);
    wr::<u64>(slot + 0x18, val);
    wr::<u64>(slot + 0x20, 0);
}

/// `unknown_1401f9650`
/// Original: 0x1401f9650 (153 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1401f9650(
    param_1: usize,
    param_2: usize,
    param_3: u64,
) -> usize {
    let mut entry = rd::<usize>(param_1 + 0x58)
        .wrapping_add((rd::<u64>(param_1 + 0x40) as usize).wrapping_shl(4));
    if rd::<u64>(param_1 + 0x38) != rd::<u64>(param_1 + 0x30) {
        let mut key = param_3;
        let mut pos_pair = [0u64; 2];
        exe::call_va!(
            0x1_401f_a730,
            (
                param_1,
                pos_pair.as_mut_ptr() as usize,
                (&mut key as *mut u64) as usize
            )
        );
        if pos_pair[0] == u64::MAX {
            wr::<u64>(param_2, u64::MAX);
            return param_2;
        }
        entry = rd::<usize>(param_1 + 0x58).wrapping_add((pos_pair[0] as usize).wrapping_shl(4));
    }

    let end_ptr = rd::<usize>(param_1 + 0x58)
        .wrapping_add((rd::<u64>(param_1 + 0x40) as usize).wrapping_shl(4));
    if entry != end_ptr {
        wr::<u64>(param_2, rd::<u64>(entry + 0x08));
    } else {
        wr::<u64>(param_2, u64::MAX);
    }
    param_2
}

/// `opentelemetry_1402050d0`
/// Original: 0x1402050d0 (184 bytes, Throws::Discard)
pub unsafe extern "win64" fn opentelemetry_1402050d0(
    param_1: usize,
    param_2: usize,
    param_3: u64,
) {
    if param_3 > 0x7fff_ffff_ffff_ffff {
        exe::call_va!(0x1_4020_0110, ());
        return;
    }
    if param_3 <= 0x0f {
        wr::<u64>(param_1 + 0x10, param_3);
        wr::<u64>(param_1 + 0x18, 0x0f);
        exe::call_va!(0x1_4183_acbc, (param_1, param_2, param_3));
        wr::<u8>(param_1.wrapping_add(param_3 as usize), 0);
    } else {
        let cand = param_3 | 0x0f;
        let cap = if cand > 0x7fff_ffff_ffff_ffff {
            0x7fff_ffff_ffff_ffff
        } else if cand < 0x16 {
            0x16
        } else {
            cand
        };
        let dst = exe::call_va!(0x1_401e_9110, (cap.wrapping_add(1)) -> usize);
        wr::<usize>(param_1, dst);
        wr::<u64>(param_1 + 0x10, param_3);
        wr::<u64>(param_1 + 0x18, cap);
        exe::call_va!(0x1_4183_acbc, (dst, param_2, param_3));
        wr::<u8>(dst.wrapping_add(param_3 as usize), 0);
    }
}

/// `unknown_14020d4a0`
/// Original: 0x14020d4a0 (285 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14020d4a0(
    param_1: usize,
    param_2: u64,
    param_3: usize,
) -> usize {
    let mut out = u64::MAX;
    if param_2 != u64::MAX {
        let ctx = rd::<usize>(param_3);
        let count = rd::<u64>(param_3 + 0x08);
        let arr_u16 = rd::<usize>(param_3 + 0x10);
        let idx = (param_2 as u32) as usize;
        let cap = rd::<u32>(ctx + 0x20) as usize;
        if idx < cap {
            let handles = rd::<usize>(ctx + 0x18);
            let entry = rd::<u64>(handles + idx * 8);
            if (entry as u32) == (param_2 as u32)
                && (entry >> 32) == (param_2 >> 32)
                && (param_2 as u32) != u32::MAX
            {
                let tbl_30 = rd::<usize>(ctx + 0x30);
                let target = rd::<u64>(tbl_30 + idx * 8) as u16;
                if target == 0xffff {
                    exe::call_iat!(0x1_4264_67e8, ());
                    return param_1;
                }
                let mut cur = arr_u16;
                let end = arr_u16.wrapping_add((count as usize).wrapping_mul(2));
                let mut rem = ((end.wrapping_sub(cur)) as isize) >> 1;
                while rem > 0 {
                    let half = (rem as usize) >> 1;
                    let mid = cur.wrapping_add(half * 2);
                    if rd::<u16>(mid) < target {
                        cur = mid.wrapping_add(2);
                        rem = rem.wrapping_sub(1).wrapping_sub(half as isize);
                    } else {
                        rem = half as isize;
                    }
                }
                if cur != end && target >= rd::<u16>(cur) {
                    out = param_2;
                }
            }
        }
    }
    wr::<u64>(param_1, out);
    param_1
}

/// `bigfish_14020e8f0`
/// Original: 0x14020e8f0 (141 bytes, Throws::No)
pub unsafe extern "win64" fn bigfish_14020e8f0(param_1: usize) {
    let old_tail = rd::<usize>(param_1 + 0x08);
    let tls0 = rd::<usize>(exe::tls_pointer());
    let heap = rd::<usize>(tls0 + 0x6348);
    let mut node = exe::call_va!(0x1_4253_5070, (heap, 0x10010u64, 0x10u64) -> usize);
    if node == 0 {
        let proc = exe::call_iat!(0x1_4264_56c8, () -> usize);
        exe::call_iat!(0x1_4264_53a0, (proc, 0xdead_d00du32));
        node = 0;
    }
    wr::<usize>(param_1 + 0x08, node);
    wr::<u64>(node, 0);
    let cur = rd::<usize>(param_1 + 0x08);
    wr::<u64>(cur + 0x08, 0);
    let cur2 = rd::<usize>(param_1 + 0x08);
    if old_tail != 0 {
        wr::<usize>(old_tail, cur2);
    } else {
        wr::<usize>(param_1, cur2);
    }
}

/// `unknown_140211010`
/// Original: 0x140211010 (414 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140211010(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let old_cap = (rd::<u32>(param_1 + 0x0c) & 0x3fff_ffff) as u64;
    let mut new_cap = old_cap + (old_cap >> 1);
    if new_cap < 8 {
        new_cap = 8;
    } else if new_cap > 0x3fff_ffff {
        new_cap = 0x3fff_ffff;
    }

    let ins_idx = ((rd::<usize>(param_2) as isize).wrapping_sub(rd::<usize>(param_1) as isize)) >> 3;
    let new_buf = exe::call_va!(0x1_401d_ec00, (param_1, new_cap as u32) -> usize);
    let ins_ptr = new_buf.wrapping_add((ins_idx as usize).wrapping_mul(8));
    wr::<u64>(ins_ptr, rd::<u64>(param_3));

    let old_buf = rd::<usize>(param_1);
    let old_len = rd::<u32>(param_1 + 0x08) as usize;
    let pos_ptr = rd::<usize>(param_2);
    let old_end = old_buf.wrapping_add(old_len * 8);

    if pos_ptr == old_end {
        let mut src = old_buf;
        let mut dst = new_buf;
        while src != old_end {
            wr::<u64>(dst, rd::<u64>(src));
            src = src.wrapping_add(8);
            dst = dst.wrapping_add(8);
        }
    } else {
        let mut src = old_buf;
        let mut dst = new_buf;
        while src != pos_ptr {
            wr::<u64>(dst, rd::<u64>(src));
            src = src.wrapping_add(8);
            dst = dst.wrapping_add(8);
        }
        let old_len2 = rd::<u32>(param_1 + 0x08) as usize;
        let old_end2 = rd::<usize>(param_1).wrapping_add(old_len2 * 8);
        let mut src2 = rd::<usize>(param_2);
        let mut dst2 = ins_ptr.wrapping_add(8);
        while src2 != old_end2 {
            wr::<u64>(dst2, rd::<u64>(src2));
            src2 = src2.wrapping_add(8);
            dst2 = dst2.wrapping_add(8);
        }
    }

    if (rd::<u32>(param_1 + 0x0c) & 0x4000_0000) == 0 {
        let ptr = rd::<usize>(param_1);
        if ptr != 0 {
            free_mimalloc_block(ptr);
        }
    }

    wr::<usize>(param_1, new_buf);
    wr::<u32>(param_1 + 0x08, rd::<u32>(param_1 + 0x08).wrapping_add(1));
    wr::<u32>(param_1 + 0x0c, (new_cap as u32) & 0x3fff_ffff);
    ins_ptr
}

/// `content_140236010`
/// Original: 0x140236010 (204 bytes, Throws::No)
pub unsafe extern "win64" fn content_140236010(param_1: usize) {
    let len = rd::<u32>(param_1 + 0x08) as usize;
    let mut cur = rd::<usize>(param_1);
    let end = cur.wrapping_add(len * 0x28);
    while cur != end {
        crate::simd_ops::atomic_xchg_u64((cur + 0x08) as u64, 0);
        cur = cur.wrapping_add(0x28);
    }

    let cap_flags = rd::<u32>(param_1 + 0x0c);
    wr::<u32>(param_1 + 0x08, 0);
    if (cap_flags & 0x4000_0000) != 0 {
        return;
    }

    let ptr = rd::<usize>(param_1);
    if ptr != 0 {
        free_mimalloc_block(ptr);
    }

    wr::<u32>(param_1 + 0x0c, rd::<u32>(param_1 + 0x0c) & 0xc000_0000);
    wr::<usize>(param_1, 0);
}

/// `unknown_140236fa0`
/// Original: 0x140236fa0 (185 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140236fa0(param_1: usize) {
    let len = rd::<u32>(param_1 + 0x08) as usize;
    let mut cur = rd::<usize>(param_1);
    let end = cur.wrapping_add(len * 0x18);
    while cur != end {
        crate::simd_ops::atomic_xchg_u64((cur + 0x08) as u64, 0);
        cur = cur.wrapping_add(0x18);
    }

    let ptr = rd::<usize>(param_1);
    if ptr != 0 {
        free_mimalloc_block(ptr);
    }

    wr::<usize>(param_1, 0);
    wr::<u64>(param_1 + 0x08, 0);
}

/// `unknown_14028a2a0`
/// Original: 0x14028a2a0 (201 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14028a2a0(param_1: usize, param_2: u64) {
    let old_len = rd::<u32>(param_1 + 0x08) as u64;
    let old_cap = rd::<u32>(param_1 + 0x0c) as u64;
    if param_2 > old_cap {
        let mut grow = (old_cap >> 1) + old_cap;
        if grow < param_2 {
            grow = param_2;
        }
        exe::call_va!(0x1_401d_eba0, (param_1, grow));
        let diff = param_2.wrapping_sub(old_len);
        if diff != 0 {
            let cur_len = rd::<u32>(param_1 + 0x08) as usize;
            let mut dst = rd::<usize>(param_1).wrapping_add(cur_len * 8);
            for _ in 0..diff {
                wr::<u64>(dst, u64::MAX);
                dst = dst.wrapping_add(8);
            }
        }
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    } else if param_2 > old_len {
        let diff = param_2.wrapping_sub(old_len);
        if diff != 0 {
            let mut dst = rd::<usize>(param_1).wrapping_add((old_len as usize) * 8);
            for _ in 0..diff {
                wr::<u64>(dst, u64::MAX);
                dst = dst.wrapping_add(8);
            }
        }
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    } else if param_2 != old_len {
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    }
}

/// `unknown_1402d6540`
/// Original: 0x1402d6540 (235 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1402d6540(param_1: usize, param_2: u64) {
    let old_cap = (rd::<u32>(param_1 + 0x0c) & 0x3fff_ffff) as u64;
    if old_cap >= param_2 {
        return;
    }

    let new_buf = exe::call_va!(0x1_4024_a9e0, (param_1, param_2) -> usize);
    let old_buf = rd::<usize>(param_1);
    if old_buf != 0 {
        let len = rd::<u32>(param_1 + 0x08) as u64;
        if len != 0 {
            exe::call_va!(0x1_4183_acc8, (new_buf, old_buf, len.wrapping_mul(0x30)));
        }
        if (rd::<u32>(param_1 + 0x0c) & 0x4000_0000) == 0 {
            let ptr = rd::<usize>(param_1);
            if ptr != 0 {
                free_mimalloc_block(ptr);
            }
        }
    }

    wr::<usize>(param_1, new_buf);
    wr::<u32>(param_1 + 0x0c, (param_2 as u32) & 0x3fff_ffff);
}

/// `unknown_1403063b0`
/// Original: 0x1403063b0 (516 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1403063b0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let old_cap = (rd::<u32>(param_1 + 0x0c) & 0x3fff_ffff) as u64;
    let mut new_cap = old_cap + (old_cap >> 1);
    if new_cap < 8 {
        new_cap = 8;
    } else if new_cap > 0x3fff_ffff {
        new_cap = 0x3fff_ffff;
    }

    let byte_diff = (rd::<usize>(param_2) as i64).wrapping_sub(rd::<usize>(param_1) as i64);
    let ins_idx = (byte_diff / 0x30) as isize as usize;
    let new_buf = exe::call_va!(0x1_4024_a9e0, (byte_diff as u64, new_cap as u32) -> usize);
    let ins_ptr = new_buf.wrapping_add(ins_idx.wrapping_mul(0x30));
    wr::<[u8; 0x30]>(ins_ptr, rd::<[u8; 0x30]>(param_3));

    let old_buf = rd::<usize>(param_1);
    let old_len = rd::<u32>(param_1 + 0x08) as usize;
    let pos_ptr = rd::<usize>(param_2);
    let old_end = old_buf.wrapping_add(old_len * 0x30);

    if pos_ptr == old_end {
        let mut src = old_buf;
        let mut dst = new_buf;
        while src != old_end {
            wr::<[u8; 0x30]>(dst, rd::<[u8; 0x30]>(src));
            src = src.wrapping_add(0x30);
            dst = dst.wrapping_add(0x30);
        }
    } else {
        let mut src = old_buf;
        let mut dst = new_buf;
        while src != pos_ptr {
            wr::<[u8; 0x30]>(dst, rd::<[u8; 0x30]>(src));
            src = src.wrapping_add(0x30);
            dst = dst.wrapping_add(0x30);
        }
        let old_len2 = rd::<u32>(param_1 + 0x08) as usize;
        let old_end2 = rd::<usize>(param_1).wrapping_add(old_len2 * 0x30);
        let mut src2 = rd::<usize>(param_2);
        let mut dst2 = ins_ptr.wrapping_add(0x30);
        while src2 != old_end2 {
            wr::<[u8; 0x30]>(dst2, rd::<[u8; 0x30]>(src2));
            src2 = src2.wrapping_add(0x30);
            dst2 = dst2.wrapping_add(0x30);
        }
    }

    if (rd::<u32>(param_1 + 0x0c) & 0x4000_0000) == 0 {
        let ptr = rd::<usize>(param_1);
        if ptr != 0 {
            free_mimalloc_block(ptr);
        }
    }

    wr::<usize>(param_1, new_buf);
    wr::<u32>(param_1 + 0x08, rd::<u32>(param_1 + 0x08).wrapping_add(1));
    wr::<u32>(param_1 + 0x0c, (new_cap as u32) & 0x3fff_ffff);
    ins_ptr
}

/// `unknown_140348930`
/// Original: 0x140348930 (124 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140348930(param_1: usize, param_2: u64) {
    let prev = rd::<u64>(param_1 + 0x58);
    let changed = u64::from(param_2 != prev);
    wr::<u64>(param_1 + 0x60, rd::<u64>(param_1 + 0x60).wrapping_add(changed));

    let list = param_1 + 0x78;
    wr::<u64>(param_1 + 0x58, param_2);
    let tail = rd::<usize>(list + 0x08);
    if tail == 0 || rd::<u64>(tail + 0x08).wrapping_add(1) > 0x4000 {
        exe::call_va!(0x1_4020_e8f0, (list));
    }

    let tail2 = rd::<usize>(list + 0x08);
    let idx = rd::<u64>(tail2 + 0x08) as usize;
    wr::<u32>(tail2 + 0x10 + idx * 4, param_2 as u32);
    let tail3 = rd::<usize>(list + 0x08);
    wr::<u64>(tail3 + 0x08, rd::<u64>(tail3 + 0x08).wrapping_add(1));
    wr::<u64>(list + 0x10, rd::<u64>(list + 0x10).wrapping_add(1));
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1401df790(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_qpc(out: usize) -> i32 {
        wr::<i64>(out, 0x1234_5678_9abc);
        1
    }
    unsafe extern "win64" fn mock_freq() -> i64 {
        10_000_000
    }

    let _im = crate::difftest::IatMock::new(0x1_4264_56f8, mock_qpc as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4240_6920, mock_freq as *const ())?;

    let run = |f: F| -> (bool, [u8; 0x28]) {
        let mut buf = [0xffu8; 0x28];
        let bp = buf.as_mut_ptr() as usize;
        let ret = f(bp);
        (ret == bp, buf)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1401df790");
    Ok(1)
}

pub unsafe fn diff_sparsehash_1401eb860(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FOUND_POS: u64 = u64::MAX;
    static mut INS_POS: u64 = 0;
    static mut RESIZED: u8 = 0;

    unsafe extern "win64" fn mock_8680(_ht: usize, out_pair: usize, _key: usize) {
        wr::<u64>(out_pair, FOUND_POS);
        wr::<u64>(out_pair + 0x08, INS_POS);
    }
    unsafe extern "win64" fn mock_bb50(_ht: usize) -> u8 {
        RESIZED
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_8680, mock_8680 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_bb50, mock_bb50 as *const ())?;

    let run = |f: F, mode: u8| -> (usize, u64, u64, Vec<u64>) {
        let mut ht = [0u8; 0x90];
        let mut table = vec![0u64; 4 * 5];
        let hp = ht.as_mut_ptr() as usize;
        let tp = table.as_mut_ptr() as usize;

        // Empty key = (0xeeee, 0xeeee), deleted key = (0xdddd, 0xdddd)
        for i in 0..4 {
            wr::<u64>(tp + i * 0x28, 0xeeee);
            wr::<u64>(tp + i * 0x28 + 0x08, 0xeeee);
            wr::<u64>(tp + i * 0x28 + 0x10, 0x9999);
        }

        wr::<u64>(hp + 0x28, 0xdddd);
        wr::<u64>(hp + 0x30, 0xdddd);
        wr::<u64>(hp + 0x38, 1);
        wr::<u64>(hp + 0x40, 2);
        wr::<u64>(hp + 0x48, 4);
        wr::<u64>(hp + 0x50, 0xeeee);
        wr::<u64>(hp + 0x58, 0xeeee);
        wr::<usize>(hp + 0x78, tp);

        let key: [u64; 2] = [0x1001, 0x1003]; // xor = 2 -> bucket 2
        let kp = key.as_ptr() as usize;

        match mode {
            0 => {
                // Already found at index 1
                FOUND_POS = 1;
                INS_POS = 1;
                RESIZED = 0;
            }
            1 => {
                // Not found, not resized, insert into empty slot 2
                FOUND_POS = u64::MAX;
                INS_POS = 2;
                RESIZED = 0;
            }
            2 => {
                // Not found, not resized, insert into deleted slot 2
                FOUND_POS = u64::MAX;
                INS_POS = 2;
                RESIZED = 0;
                wr::<u64>(tp + 2 * 0x28, 0xdddd);
                wr::<u64>(tp + 2 * 0x28 + 0x08, 0xdddd);
            }
            _ => {
                // Not found, resized = 1, slot 2 is deleted, slot 3 is empty -> reuses slot 2
                FOUND_POS = u64::MAX;
                INS_POS = 0;
                RESIZED = 1;
                wr::<u64>(tp + 2 * 0x28, 0xdddd);
                wr::<u64>(tp + 2 * 0x28 + 0x08, 0xdddd);
            }
        }

        let ret = f(hp, kp);
        (
            ret.wrapping_sub(tp),
            rd::<u64>(hp + 0x38),
            rd::<u64>(hp + 0x40),
            table,
        )
    };

    for mode in 0..4u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "1401eb860 mode={}", mode);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_1401f1a80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FOUND_IDX: u64 = u64::MAX;
    static mut INSERT_SLOT: usize = 0;

    unsafe extern "win64" fn mock_8680(_ht: usize, out_pair: usize, _key: usize) {
        wr::<u64>(out_pair, FOUND_IDX);
        wr::<u64>(out_pair + 0x08, 0);
    }
    unsafe extern "win64" fn mock_b860(_ht: usize, _key: usize) -> usize {
        INSERT_SLOT
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_8680, mock_8680 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_b860, mock_b860 as *const ())?;

    let run = |f: F, mode: u8| -> (u16, u8, u8, u32, u64, u32, u32, u64) {
        let mut obj = [0u8; 0x140];
        let mut table = [0u8; 0x28 * 2];
        let mut free_ids: [u32; 2] = [0x40, 0x60];
        let val: i32 = -42;
        let key: [u64; 2] = [11, 22];

        let op = obj.as_mut_ptr() as usize;
        let tp = table.as_mut_ptr() as usize;
        let fp = free_ids.as_mut_ptr() as usize;
        let vp = (&val as *const i32) as usize;
        let kp = key.as_ptr() as usize;

        INSERT_SLOT = tp;
        wr::<u64>(op + 0x38, 0);
        wr::<u64>(op + 0x40, if mode == 2 { 0 } else { 1 });
        wr::<u64>(op + 0x48, 2);
        wr::<usize>(op + 0x78, tp);
        wr::<usize>(op + 0x100, fp);
        wr::<u32>(op + 0x108, if mode == 1 { 2 } else { 0 });
        wr::<u32>(op + 0x110, 0x80);

        wr::<u8>(tp + 0x12, 0x02);
        wr::<u32>(tp + 0x14, 0x40);

        FOUND_IDX = if mode == 0 { 0 } else { u64::MAX };

        f(op, kp, 0, vp, 0x05);
        (
            rd::<u16>(tp + 0x10),
            rd::<u8>(tp + 0x12),
            rd::<u8>(tp + 0x13),
            rd::<u32>(tp + 0x14),
            rd::<u64>(tp + 0x18),
            rd::<u32>(op + 0x108),
            rd::<u32>(op + 0x110),
            rd::<u64>(op + 0x118),
        )
    };

    for mode in 0..3u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "1401f1a80 mode={}", mode);
    }
    Ok(3)
}

pub unsafe fn diff_coregame_1401f3100(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut COPY_STR_OFF: (usize, usize, u32) = (0, 0, 0);

    unsafe extern "win64" fn mock_aa00(dst_s: usize, src_s: usize) {
        COPY_STR_OFF = (dst_s, src_s, rd::<u32>(dst_s));
        wr::<u32>(dst_s + 0x04, 3);
        wr::<u32>(dst_s + 0x08, 0x0063_6261);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_aa00, mock_aa00 as *const ())?;

    let run = |f: F, cnt: u64| -> (bool, Vec<u8>, (usize, usize, u32)) {
        COPY_STR_OFF = (0, 0, 0);
        let mut dst = vec![0u8; 0xa0];
        let mut src = vec![0u8; 0xa0];
        let dp = dst.as_mut_ptr() as usize;
        let sp = src.as_mut_ptr() as usize;

        wr::<u64>(sp, 0x1122_3344_5566_7788);
        wr::<u32>(sp + 0x08, 0x99aa_bbcc);
        wr::<u64>(sp + 0x10, 0xdead_beef_0000_0001);
        wr::<u64>(sp + 0x18, 0xdead_beef_0000_0002);
        wr::<u64>(sp + 0x20, 0xaaaa_0001);
        wr::<u64>(sp + 0x28, 0xaaaa_0002);
        wr::<u64>(sp + 0x30, 0xaaaa_0003);
        wr::<u64>(sp + 0x40, cnt);
        wr::<u32>(dp + 0x48, 0x0800_1234);

        let ret = f(dp, sp);
        (
            ret == dp,
            dst,
            (
                COPY_STR_OFF.0.wrapping_sub(dp),
                COPY_STR_OFF.1.wrapping_sub(sp),
                COPY_STR_OFF.2,
            ),
        )
    };

    for cnt in [0u64, 3] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "1401f3100 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1401f3a20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FOUND_IDX: u64 = u64::MAX;
    static mut INSERT_SLOT: usize = 0;

    unsafe extern "win64" fn mock_8680(_ht: usize, out_pair: usize, _key: usize) {
        wr::<u64>(out_pair, FOUND_IDX);
        wr::<u64>(out_pair + 0x08, 0);
    }
    unsafe extern "win64" fn mock_b860(_ht: usize, _key: usize) -> usize {
        INSERT_SLOT
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_8680, mock_8680 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_b860, mock_b860 as *const ())?;

    let run = |f: F, mode: u8| -> (u16, u8, u8, u32, u64, u32, u32, u64) {
        let mut obj = [0u8; 0x140];
        let mut table = [0u8; 0x28 * 2];
        let mut free_ids: [u32; 2] = [0x20, 0xa0];
        let val: u64 = 0xfeed_face_cafe_babe;
        let key: [u64; 2] = [33, 44];

        let op = obj.as_mut_ptr() as usize;
        let tp = table.as_mut_ptr() as usize;
        let fp = free_ids.as_mut_ptr() as usize;
        let vp = (&val as *const u64) as usize;
        let kp = key.as_ptr() as usize;

        INSERT_SLOT = tp;
        wr::<u64>(op + 0x38, 0);
        wr::<u64>(op + 0x40, if mode == 2 { 0 } else { 1 });
        wr::<u64>(op + 0x48, 2);
        wr::<usize>(op + 0x78, tp);
        wr::<usize>(op + 0x100, fp);
        wr::<u32>(op + 0x108, if mode == 1 { 2 } else { 0 });
        wr::<u32>(op + 0x110, 0xc0);

        wr::<u8>(tp + 0x12, 0x10);
        wr::<u32>(tp + 0x14, 0x20);

        FOUND_IDX = if mode == 0 { 0 } else { u64::MAX };

        f(op, kp, 0, vp, 0x03);
        (
            rd::<u16>(tp + 0x10),
            rd::<u8>(tp + 0x12),
            rd::<u8>(tp + 0x13),
            rd::<u32>(tp + 0x14),
            rd::<u64>(tp + 0x18),
            rd::<u32>(op + 0x108),
            rd::<u32>(op + 0x110),
            rd::<u64>(op + 0x118),
        )
    };

    for mode in 0..3u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "1401f3a20 mode={}", mode);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1401f9650(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FOUND_POS: u64 = 0;

    unsafe extern "win64" fn mock_a730(_ht: usize, out_pair: usize, _key_ptr: usize) {
        wr::<u64>(out_pair, FOUND_POS);
        wr::<u64>(out_pair + 0x08, 0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401f_a730, mock_a730 as *const ())?;

    let run = |f: F, mode: u8| -> (bool, u64) {
        let mut ht = [0u8; 0x60];
        let mut table = [0u64; 8];
        let mut out: u64 = 0x1234;
        let hp = ht.as_mut_ptr() as usize;
        let tp = table.as_mut_ptr() as usize;
        let op = (&mut out as *mut u64) as usize;

        wr::<u64>(tp + 0x08, 0x7777_8888);
        wr::<u64>(hp + 0x30, 0);
        wr::<u64>(hp + 0x38, if mode == 0 { 0 } else { 2 });
        wr::<u64>(hp + 0x40, 4);
        wr::<usize>(hp + 0x58, tp);

        FOUND_POS = match mode {
            1 => 0,
            2 => 4,
            _ => u64::MAX,
        };

        let ret = f(hp, op, 42);
        (ret == op, out)
    };

    for mode in 0..4u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "1401f9650 mode={}", mode);
    }
    Ok(4)
}

pub unsafe fn diff_opentelemetry_1402050d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut HEAP_BUF: [u8; 64] = [0; 64];
    static mut ALLOC_ARG: u64 = 0;

    unsafe extern "win64" fn mock_9110(sz: u64) -> usize {
        ALLOC_ARG = sz;
        core::ptr::addr_of_mut!(HEAP_BUF) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_9110, mock_9110 as *const ())?;

    let src = *b"0123456789abcdefghijklmnopqrstuvwxyz";
    let sp = src.as_ptr() as usize;

    let run = |f: F, len: u64| -> ([u8; 0x20], [u8; 64], u64) {
        HEAP_BUF = [0xffu8; 64];
        ALLOC_ARG = 0;
        let mut s = [0u8; 0x20];
        let dst = s.as_mut_ptr() as usize;
        f(dst, sp, len);
        (s, HEAP_BUF, ALLOC_ARG)
    };

    for len in [0u64, 5, 15, 16, 25] {
        let o = run(f_o, len);
        let n = run(f_n, len);
        crate::diff_eq!(o, n, "1402050d0 len={}", len);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_14020d4a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, handle: u64, target_val: u16| -> (bool, u64) {
        let mut ctx = [0u8; 0x40];
        let handles: [u64; 4] = [0, 0x0000_0002_0000_0001, 0x0000_0005_0000_0002, 0];
        let mut tbl30: [u64; 4] = [0, 20, 35, 0];
        tbl30[1] = target_val as u64;
        let sorted_u16: [u16; 5] = [10, 20, 30, 40, 50];
        let mut p3 = [0u64; 3];
        let mut out: u64 = 0;

        let cp = ctx.as_mut_ptr() as usize;
        let hp = handles.as_ptr() as usize;
        let tp = tbl30.as_ptr() as usize;
        let up = sorted_u16.as_ptr() as usize;
        let p3p = p3.as_mut_ptr() as usize;
        let op = (&mut out as *mut u64) as usize;

        wr::<usize>(cp + 0x18, hp);
        wr::<u32>(cp + 0x20, 4);
        wr::<usize>(cp + 0x30, tp);

        wr::<usize>(p3p, cp);
        wr::<u64>(p3p + 0x08, 5);
        wr::<usize>(p3p + 0x10, up);

        let ret = f(op, handle, p3p);
        (ret == op, out)
    };

    for (h, tv) in [
        (u64::MAX, 20u16),
        (0x0000_0002_0000_0009, 20),
        (0x0000_0009_0000_0001, 20),
        (0x0000_0002_0000_0001, 20),
        (0x0000_0002_0000_0001, 25),
        (0x0000_0002_0000_0001, 5),
    ] {
        let o = run(f_o, h, tv);
        let n = run(f_n, h, tv);
        crate::diff_eq!(o, n, "14020d4a0 h={:#x} tv={}", h, tv);
    }
    Ok(6)
}

pub unsafe fn diff_bigfish_14020e8f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NODE_MEM: [u64; 4] = [0; 4];
    static mut PREV_TAIL: [u64; 2] = [0; 2];
    static mut ALLOC_ARGS: (usize, u64, u64) = (0, 0, 0);

    unsafe extern "win64" fn mock_5070(heap: usize, sz: u64, align: u64) -> usize {
        ALLOC_ARGS = (heap, sz, align);
        core::ptr::addr_of_mut!(NODE_MEM) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_5070, mock_5070 as *const ())?;

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);
    let mut tls_blk = vec![0u8; 0x6400];
    let tp = tls_blk.as_mut_ptr() as usize;
    wr::<usize>(tp + 0x6348, 0xCAFE_1234);

    let run = |f: F, has_tail: bool| -> ([usize; 2], [u64; 4], [u64; 2], (usize, u64, u64)) {
        NODE_MEM = [0xffff_ffff_ffff_ffff; 4];
        PREV_TAIL = [0x9999; 2];
        ALLOC_ARGS = (0, 0, 0);

        let mut list = [0usize; 2];
        let lp = list.as_mut_ptr() as usize;
        let ptp = core::ptr::addr_of_mut!(PREV_TAIL) as usize;
        if has_tail {
            wr::<usize>(lp, 0x1111);
            wr::<usize>(lp + 0x08, ptp);
        }

        wr::<usize>(tls_slot, tp);
        f(lp);
        wr::<usize>(tls_slot, saved_tls0);
        (list, NODE_MEM, PREV_TAIL, ALLOC_ARGS)
    };

    for has_tail in [false, true] {
        let o = run(f_o, has_tail);
        let n = run(f_n, has_tail);
        crate::diff_eq!(o, n, "14020e8f0 has_tail={}", has_tail);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_140211010(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_ARR: [u64; 16] = [0; 16];
    static mut FREE_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_ec00(_vec: usize, _cap: u32) -> usize {
        core::ptr::addr_of_mut!(NEW_ARR) as usize
    }
    unsafe extern "win64" fn mock_3200(_chunk: usize, _desc: usize, _flag: u32, _ptr: usize) {
        FREE_COUNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_401d_ec00, mock_ec00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_3200, mock_3200 as *const ())?;

    let mut arena_vec = vec![0u8; 0x0210_0000];
    let arena = (arena_vec.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;
    let old_buf = arena + 0x1_0000;

    let run = |f: F, ins_pos: usize| -> (usize, usize, u32, u32, [u64; 8], u32) {
        NEW_ARR = [0; 16];
        FREE_COUNT = 0;
        for i in 0..3usize {
            wr::<u64>(old_buf + i * 8, 100 + i as u64);
        }

        let mut vec_hdr = [0u8; 0x10];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, old_buf);
        wr::<u32>(vp + 0x08, 3);
        wr::<u32>(vp + 0x0c, 4);

        let iter_ptr: usize = old_buf + ins_pos * 8;
        let val: u64 = 999;
        let ret = f(
            vp,
            (&iter_ptr as *const usize) as usize,
            (&val as *const u64) as usize,
        );
        let mut out_slice = [0u64; 8];
        out_slice.copy_from_slice(&NEW_ARR[..8]);
        (
            ret,
            rd::<usize>(vp),
            rd::<u32>(vp + 0x08),
            rd::<u32>(vp + 0x0c),
            out_slice,
            FREE_COUNT,
        )
    };

    for pos in [0usize, 1, 3] {
        let o = run(f_o, pos);
        let n = run(f_n, pos);
        crate::diff_eq!(o, n, "140211010 pos={}", pos);
    }
    Ok(3)
}

pub unsafe fn diff_content_140236010(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FREE_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_3200(_chunk: usize, _desc: usize, _flag: u32, _ptr: usize) {
        FREE_COUNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_3200, mock_3200 as *const ())?;

    let mut arena_vec = vec![0u8; 0x0210_0000];
    let arena = (arena_vec.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;
    let buf_ptr = arena + 0x1_0000;

    let run = |f: F, inplace: bool| -> (bool, u32, u32, [u64; 2], u32) {
        FREE_COUNT = 0;
        for i in 0..2usize {
            wr::<u64>(buf_ptr + i * 0x28 + 0x08, 0x1234_5678 + i as u64);
        }
        let mut vec_hdr = [0u8; 0x10];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, buf_ptr);
        wr::<u32>(vp + 0x08, 2);
        wr::<u32>(vp + 0x0c, if inplace { 0x4000_0004 } else { 0x8000_0004 });

        f(vp);
        (
            rd::<usize>(vp) == 0,
            rd::<u32>(vp + 0x08),
            rd::<u32>(vp + 0x0c),
            [rd::<u64>(buf_ptr + 0x08), rd::<u64>(buf_ptr + 0x28 + 0x08)],
            FREE_COUNT,
        )
    };

    for inplace in [false, true] {
        let o = run(f_o, inplace);
        let n = run(f_n, inplace);
        crate::diff_eq!(o, n, "140236010 inplace={}", inplace);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_140236fa0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FREE_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_3200(_chunk: usize, _desc: usize, _flag: u32, _ptr: usize) {
        FREE_COUNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_3200, mock_3200 as *const ())?;

    let mut arena_vec = vec![0u8; 0x0210_0000];
    let arena = (arena_vec.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;
    let buf_ptr = arena + 0x1_0000;

    let run = |f: F, non_null: bool| -> (usize, u64, [u64; 2], u32) {
        FREE_COUNT = 0;
        for i in 0..2usize {
            wr::<u64>(buf_ptr + i * 0x18 + 0x08, 0xabcd_0000 + i as u64);
        }
        let mut vec_hdr = [0u8; 0x10];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, if non_null { buf_ptr } else { 0 });
        wr::<u32>(vp + 0x08, if non_null { 2 } else { 0 });
        wr::<u32>(vp + 0x0c, 4);

        f(vp);
        (
            rd::<usize>(vp),
            rd::<u64>(vp + 0x08),
            [rd::<u64>(buf_ptr + 0x08), rd::<u64>(buf_ptr + 0x18 + 0x08)],
            FREE_COUNT,
        )
    };

    for non_null in [false, true] {
        let o = run(f_o, non_null);
        let n = run(f_n, non_null);
        crate::diff_eq!(o, n, "140236fa0 non_null={}", non_null);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14028a2a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut GROW_CALLED: u64 = 0;

    unsafe extern "win64" fn mock_eba0(vec: usize, new_cap: u64) {
        GROW_CALLED = new_cap;
        wr::<u32>(vec + 0x0c, new_cap as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401d_eba0, mock_eba0 as *const ())?;

    let run = |f: F, new_len: u64| -> (u32, u32, [u64; 8], u64) {
        GROW_CALLED = 0;
        let mut arr = [0x1111u64; 8];
        let mut vec_hdr = [0u8; 0x10];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, arr.as_mut_ptr() as usize);
        wr::<u32>(vp + 0x08, 2);
        wr::<u32>(vp + 0x0c, 4);

        f(vp, new_len);
        (rd::<u32>(vp + 0x08), rd::<u32>(vp + 0x0c), arr, GROW_CALLED)
    };

    for new_len in [1u64, 2, 4, 5, 7] {
        let o = run(f_o, new_len);
        let n = run(f_n, new_len);
        crate::diff_eq!(o, n, "14028a2a0 new_len={}", new_len);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_1402d6540(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_BUF: [u8; 0x30 * 4] = [0; 0x30 * 4];
    static mut FREE_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_a9e0(_vec: usize, _cap: u64) -> usize {
        core::ptr::addr_of_mut!(NEW_BUF) as usize
    }
    unsafe extern "win64" fn mock_3200(_chunk: usize, _desc: usize, _flag: u32, _ptr: usize) {
        FREE_COUNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4024_a9e0, mock_a9e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_3200, mock_3200 as *const ())?;

    let mut arena_vec = vec![0u8; 0x0210_0000];
    let arena = (arena_vec.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;
    let old_buf = arena + 0x1_0000;

    let run = |f: F, req_cap: u64| -> (bool, u32, Vec<u8>, u32) {
        NEW_BUF = [0u8; 0x30 * 4];
        FREE_COUNT = 0;
        for i in 0..(0x30 * 2) {
            wr::<u8>(old_buf + i, (i as u8).wrapping_add(1));
        }

        let mut vec_hdr = [0u8; 0x10];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, old_buf);
        wr::<u32>(vp + 0x08, 2);
        wr::<u32>(vp + 0x0c, 2);

        f(vp, req_cap);
        (
            rd::<usize>(vp) == core::ptr::addr_of!(NEW_BUF) as usize,
            rd::<u32>(vp + 0x0c),
            NEW_BUF.to_vec(),
            FREE_COUNT,
        )
    };

    for req_cap in [2u64, 4] {
        let o = run(f_o, req_cap);
        let n = run(f_n, req_cap);
        crate::diff_eq!(o, n, "1402d6540 req_cap={}", req_cap);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1403063b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_BUF: [u64; 6 * 6] = [0; 6 * 6];
    static mut FREE_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_a9e0(_rcx: u64, _cap: u32) -> usize {
        core::ptr::addr_of_mut!(NEW_BUF) as usize
    }
    unsafe extern "win64" fn mock_3200(_chunk: usize, _desc: usize, _flag: u32, _ptr: usize) {
        FREE_COUNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4024_a9e0, mock_a9e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_3200, mock_3200 as *const ())?;

    let mut arena_vec = vec![0u8; 0x0210_0000];
    let arena = (arena_vec.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;
    let old_buf = arena + 0x1_0000;

    let run = |f: F, ins_idx: usize| -> (usize, usize, u32, u32, Vec<u64>, u32) {
        NEW_BUF = [0; 6 * 6];
        FREE_COUNT = 0;
        for i in 0..12usize {
            wr::<u64>(old_buf + i * 8, 0x100 + i as u64);
        }

        let mut vec_hdr = [0u8; 0x10];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, old_buf);
        wr::<u32>(vp + 0x08, 2);
        wr::<u32>(vp + 0x0c, 2);

        let iter_ptr: usize = old_buf + ins_idx * 0x30;
        let elem: [u64; 6] = [1, 2, 3, 4, 5, 6];
        let ret = f(
            vp,
            (&iter_ptr as *const usize) as usize,
            elem.as_ptr() as usize,
        );
        (
            ret,
            rd::<usize>(vp),
            rd::<u32>(vp + 0x08),
            rd::<u32>(vp + 0x0c),
            NEW_BUF.to_vec(),
            FREE_COUNT,
        )
    };

    for pos in [0usize, 1, 2] {
        let o = run(f_o, pos);
        let n = run(f_n, pos);
        crate::diff_eq!(o, n, "1403063b0 pos={}", pos);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_140348930(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CHUNK_MEM: [u8; 0x40] = [0; 0x40];
    static mut ALLOC_CALLED: u32 = 0;

    unsafe extern "win64" fn mock_e8f0(list: usize) {
        ALLOC_CALLED += 1;
        let cp = core::ptr::addr_of_mut!(CHUNK_MEM) as usize;
        wr::<u64>(cp, 0);
        wr::<u64>(cp + 0x08, 0);
        wr::<usize>(list + 0x08, cp);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4020_e8f0, mock_e8f0 as *const ())?;

    let run = |f: F, same_val: bool, need_alloc: bool| -> (u64, u64, u64, u64, u32, u32) {
        CHUNK_MEM = [0u8; 0x40];
        ALLOC_CALLED = 0;
        let cp = core::ptr::addr_of_mut!(CHUNK_MEM) as usize;
        let mut obj = [0u8; 0x98];
        let op = obj.as_mut_ptr() as usize;

        wr::<u64>(op + 0x58, 0x0000_0002_0000_0007);
        wr::<u64>(op + 0x60, 10);
        wr::<u64>(op + 0x88, 5);
        if !need_alloc {
            wr::<usize>(op + 0x80, cp);
            wr::<u64>(cp + 0x08, 2);
        }

        let arg = if same_val {
            0x0000_0002_0000_0007
        } else {
            0x0000_0003_0000_0009
        };
        f(op, arg);
        let idx = (rd::<u64>(cp + 0x08) as usize).saturating_sub(1);
        (
            rd::<u64>(op + 0x58),
            rd::<u64>(op + 0x60),
            rd::<u64>(op + 0x88),
            rd::<u64>(cp + 0x08),
            rd::<u32>(cp + 0x10 + idx * 4),
            ALLOC_CALLED,
        )
    };

    for (same_val, need_alloc) in [(true, false), (false, true)] {
        let o = run(f_o, same_val, need_alloc);
        let n = run(f_n, same_val, need_alloc);
        crate::diff_eq!(o, n, "140348930 same={} alloc={}", same_val, need_alloc);
    }
    Ok(2)
}
