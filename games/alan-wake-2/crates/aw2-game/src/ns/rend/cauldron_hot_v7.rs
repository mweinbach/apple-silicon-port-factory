//! Decompiled hot functions shard V7 (`0x1423d8720..0x1424b3cb0`).

use crate::exe::{self, rd, wr};

/// `unknown_1423d8720`
/// Original: `0x1423d8720` (458 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn unknown_1423d8720(
    _param_1: usize,
    _param_2: usize,
    param_3: u64,
) {
    if param_3 == 0 {
        return;
    }

    let r15 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff10) as usize);
    let lock_ptr = r15.wrapping_add(0x2b8);
    let mut rem = param_3;

    while rem != 0 {
        let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
        if rd::<u32>(lock_ptr + 0x08) != tid {
            exe::call_iat!(0x1_4264_55a8, (lock_ptr));
            wr::<u32>(lock_ptr + 0x08, tid);
        }
        wr::<u32>(lock_ptr + 0x0c, rd::<u32>(lock_ptr + 0x0c).wrapping_add(1));
        wr::<u32>(r15 + 0x2e8, 1u32.wrapping_sub(rd::<u32>(r15 + 0x2e8)));
        let new_cnt = rd::<u32>(lock_ptr + 0x0c).wrapping_sub(1);
        wr::<u32>(lock_ptr + 0x0c, new_cnt);
        if new_cnt == 0 {
            let _ = crate::simd_ops::atomic_xchg_u32((lock_ptr + 0x08) as u64, 0xffff_ffff);
            exe::call_iat!(0x1_4264_5598, (lock_ptr));
        }

        let side = 1i32.wrapping_sub(rd::<i32>(r15 + 0x2e8)) as isize;
        let r12 = r15.wrapping_add((side << 4) as usize);
        let mut rsi = rd::<usize>(r12 + 0x2c8);
        let count1 = rd::<u32>(r12 + 0x2d0) as usize;
        let rbp = rsi.wrapping_add(count1.wrapping_mul(0x28));

        while rsi != rbp {
            exe::call_va!(0x1_4239_af80, (rd::<u64>(rsi + 0x18), rd::<u64>(rsi + 0x08)));
            exe::call_va!(
                0x1_4225_5e30,
                (r15, rd::<u64>(rsi), rsi.wrapping_add(0x18))
            );
            let node_ptr = rd::<usize>(rsi);
            let _ = crate::simd_ops::atomic_xchg_u32((node_ptr + 0x80) as u64, 4);
            let g_82b8 = rd::<u64>(exe::ptr::<u64>(0x1_43a0_82b8) as usize);
            exe::call_va!(0x1_4242_0ed0, (g_82b8));

            let g_ff00 = exe::ptr::<u32>(0x1_4397_ff00) as usize;
            let _ = crate::simd_ops::atomic_cmpxchg_u32(g_ff00 as u64, 1, 0);

            let r14 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize);
            exe::call_iat!(0x1_4264_55a8, (r14.wrapping_add(0x250)));
            wr::<u32>(r14 + 0x258, rd::<u32>(r14 + 0x258).wrapping_add(1));
            exe::call_iat!(0x1_4264_5598, (r14.wrapping_add(0x250)));
            exe::call_iat!(0x1_4264_5590, (r14.wrapping_add(0x248)));

            rsi = rsi.wrapping_add(0x28);
        }

        let mut rbx = rd::<usize>(r12 + 0x2c8);
        let count2 = rd::<u32>(r12 + 0x2d0) as usize;
        let end2 = rbx.wrapping_add(count2.wrapping_mul(0x28));
        while rbx != end2 {
            exe::call_va!(0x1_412f_4530, (rbx));
            rbx = rbx.wrapping_add(0x28);
        }
        wr::<u32>(r12 + 0x2d0, 0);

        rem -= 1;
    }
}

/// `r_14242df50` (`r::packFileMetadata::parse`)
/// Original: `0x14242df50` (673 bytes, `Throws::No`)
pub unsafe extern "win64" fn r_14242df50(
    param_1: usize,
    param_2: usize,
    param_3: u64,
    param_4: usize,
) -> usize {
    let vtbl_4 = rd::<usize>(param_4);
    let name_str = exe::ptr::<u8>(0x1_4340_98e0) as usize;
    let obj = exe::call_va!(rd::<usize>(vtbl_4 + 0x40), (param_4, name_str) -> u64);
    wr::<u64>(param_2, obj);

    let hi32 = (param_3 >> 32) as u32;
    let slice_len = ((hi32 >> 16) as u16).wrapping_sub(hi32 as u16) as u64;
    let slice_off = (hi32 as u16) as usize;
    let slice_ptr = rd::<usize>(param_1 + 0x18).wrapping_add(slice_off);

    let mut frame_50 = [0u64; 0x140 / 8];
    let base_50 = frame_50.as_mut_ptr() as usize;
    wr::<u32>(base_50, 0x2000_0020);
    wr::<u32>(base_50 + 0x04, 0);
    wr::<u8>(base_50 + 0x08, 0);
    wr::<u32>(base_50 + 0x108, 0);
    wr::<usize>(base_50 + 0x110, 0);
    wr::<u64>(base_50 + 0x118, 0);
    wr::<usize>(base_50 + 0x120, 0);
    wr::<u8>(base_50 + 0x128, 0);
    wr::<u64>(base_50 + 0x130, 0);

    let mut span_40: [u64; 2] = [exe::ptr::<u8>(0x1_426c_dcb0) as usize as u64, 0];
    exe::call_va!(
        0x1_4240_9b50,
        (base_50, span_40.as_mut_ptr() as usize, 0u64, 0u8)
    );

    wr::<u32>(base_50 + 0x108, 0);
    wr::<usize>(base_50 + 0x110, slice_ptr);
    wr::<u64>(base_50 + 0x118, slice_len);
    wr::<usize>(base_50 + 0x120, slice_ptr);
    wr::<u64>(base_50 + 0x130, slice_len);
    wr::<u8>(base_50 + 0x128, 1);

    let mut stream_190 = [0u64; 0x350 / 8];
    let base_190 = stream_190.as_mut_ptr() as usize;
    exe::call_va!(0x1_401e_d730, (base_190, base_50, 0u64, 1u8, 1u8));

    let target_obj = rd::<usize>(param_2);
    let target_vtbl = rd::<usize>(target_obj);
    exe::call_va!(
        rd::<usize>(target_vtbl + 0x68),
        (target_obj, base_190, 0u32)
    );

    wr::<usize>(base_190 + 0x348, exe::ptr::<u8>(0x1_4307_b558) as usize);
    exe::call_va!(0x1_401e_e280, (base_190));

    let buf_160 = rd::<usize>(base_50 + 0x110);
    if buf_160 != 0 && rd::<u8>(base_50 + 0x128) == 0 {
        let r10 = buf_160.wrapping_sub(1) & !0x01ff_ffffusize;
        let same_teb = exe::teb() == rd::<usize>(r10 + 0x100);
        let page_idx = buf_160.wrapping_sub(r10) >> 16;
        let hdr_raw = r10.wrapping_add(0x108).wrapping_add(page_idx.wrapping_mul(0x50));
        let rdx = hdr_raw.wrapping_sub(rd::<u32>(hdr_raw + 0x04) as usize);
        if same_teb {
            if rd::<u8>(rdx + 0x0e) == 0 {
                wr::<u64>(buf_160, rd::<u64>(rdx + 0x20));
                wr::<usize>(rdx + 0x20, buf_160);
                let new_cnt = rd::<u32>(rdx + 0x18).wrapping_sub(1);
                wr::<u32>(rdx + 0x18, new_cnt);
                if new_cnt == 0 {
                    exe::call_va!(0x1_4253_71a0, (rdx));
                }
            } else {
                exe::call_va!(0x1_4253_3200, (r10, rdx, 1u8, buf_160));
            }
        } else {
            exe::call_va!(0x1_4253_3200, (r10, rdx, 0u8, buf_160));
        }
    }

    wr::<u32>(base_50 + 0x04, 0);
    let b3 = rd::<u8>(base_50 + 0x03) as u32;
    let cap_bits = if b3 < 0x80 {
        b3.wrapping_mul(8)
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let len_bits = (rd::<u32>(base_50) & 0x00ff_ffff) << 3;
    let str_data = if len_bits > cap_bits {
        rd::<usize>(base_50 + 0x08)
    } else {
        base_50 + 0x08
    };
    wr::<u8>(str_data, 0);

    wr::<u32>(base_50 + 0x108, 0);
    wr::<usize>(base_50 + 0x110, 0);
    wr::<u64>(base_50 + 0x118, 0);
    wr::<usize>(base_50 + 0x120, 0);
    wr::<u8>(base_50 + 0x128, 0);
    wr::<u64>(base_50 + 0x130, 0);
    exe::call_va!(0x1_4240_ab10, (base_50));
    param_2
}

/// `r_14243de30`
/// Original: `0x14243de30` (1273 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn r_14243de30(param_1: usize, param_2: usize) {
    let sub_80 = rd::<usize>(param_1 + 0x80);
    wr::<u32>(sub_80 + 0x20, 0);
    let arr_18 = sub_80.wrapping_add(0x18);

    let want_cnt = rd::<u64>(param_2 + 0x38).wrapping_sub(rd::<u64>(param_2 + 0x30));
    let cur_cap = rd::<u32>(arr_18 + 0x0c) as u64;
    if want_cnt > cur_cap {
        exe::call_va!(0x1_411d_ba80, (arr_18, want_cnt));
    }

    let mut cur1 = rd::<usize>(param_2 + 0x60);
    let end1 = cur1.wrapping_add((rd::<usize>(param_2 + 0x40)).wrapping_mul(0x18));
    while cur1 != end1 {
        let k = rd::<u64>(cur1);
        if k != rd::<u64>(param_2 + 0x48)
            && (rd::<u64>(param_2 + 0x30) == 0 || rd::<u64>(param_2 + 0x28) != k)
        {
            break;
        }
        cur1 = cur1.wrapping_add(0x18);
    }

    while cur1 != end1 {
        let v0 = rd::<u64>(cur1);
        let v1 = rd::<u64>(cur1 + 0x08);
        let v2 = rd::<u64>(cur1 + 0x10);
        let len = rd::<u32>(arr_18 + 0x08) as u64;
        let cap = rd::<u32>(arr_18 + 0x0c) as u64;
        if len.wrapping_mul(0x18) == cap.wrapping_mul(0x18) {
            exe::call_va!(0x1_411d_ba80, (arr_18, len.wrapping_add(1)));
        }
        let idx0 = rd::<u32>(arr_18 + 0x08) as usize;
        let slot0 = rd::<usize>(arr_18).wrapping_add(idx0.wrapping_mul(0x18));
        wr::<u64>(slot0, 0);
        wr::<u64>(slot0 + 0x08, 0);
        wr::<u64>(slot0 + 0x10, 0);
        let new_len = rd::<u32>(arr_18 + 0x08).wrapping_add(1);
        wr::<u32>(arr_18 + 0x08, new_len);
        let slot1 = rd::<usize>(arr_18)
            .wrapping_add((new_len as usize).wrapping_mul(0x18))
            .wrapping_sub(0x18);
        wr::<u64>(slot1, v0);
        wr::<u64>(slot1 + 0x08, v1);
        wr::<u64>(slot1 + 0x10, v2);

        loop {
            cur1 = cur1.wrapping_add(0x18);
            if cur1 == end1 {
                break;
            }
            let k = rd::<u64>(cur1);
            if k != rd::<u64>(param_2 + 0x48)
                && (rd::<u64>(param_2 + 0x30) == 0 || rd::<u64>(param_2 + 0x28) != k)
            {
                break;
            }
        }
    }

    let mut cur2 = rd::<usize>(param_2 + 0x60);
    let end2 = cur2.wrapping_add((rd::<usize>(param_2 + 0x40)).wrapping_mul(0x18));
    while cur2 != end2 {
        let k = rd::<u64>(cur2);
        if k != rd::<u64>(param_2 + 0x48)
            && (rd::<u64>(param_2 + 0x30) == 0 || rd::<u64>(param_2 + 0x28) != k)
        {
            break;
        }
        cur2 = cur2.wrapping_add(0x18);
    }

    let end2_check = rd::<usize>(param_2 + 0x60)
        .wrapping_add((rd::<usize>(param_2 + 0x40)).wrapping_mul(0x18));
    let mut keys_ptr: usize = 0;
    let mut keys_len: u32 = 0;
    let mut keys_cap: u32 = 0;

    while cur2 != end2_check {
        let k = rd::<u64>(cur2);
        if keys_len == keys_cap {
            let cur_len = keys_len as u64;
            let cur_cap_u64 = keys_cap as u64;
            let mut new_cap = (cur_cap_u64 >> 1).wrapping_add(cur_cap_u64);
            if new_cap < cur_len.wrapping_add(1) {
                new_cap = cur_len.wrapping_add(1);
            }
            let new_ptr = exe::call_va!(0x1_4241_c0a0, (new_cap.wrapping_mul(8), 8u32) -> usize);
            let _ = exe::call_va!(0x1_4183_acc8, (new_ptr, keys_ptr, cur_len.wrapping_mul(8)) -> usize);
            if keys_ptr != 0 {
                exe::call_va!(0x1_4241_c190, (keys_ptr));
            }
            keys_ptr = new_ptr;
            keys_cap = new_cap as u32;
        }
        wr::<u64>(keys_ptr.wrapping_add((keys_len as usize) * 8), k);
        keys_len = keys_len.wrapping_add(1);

        loop {
            cur2 = cur2.wrapping_add(0x18);
            if cur2 == end2 {
                break;
            }
            let nk = rd::<u64>(cur2);
            if nk != rd::<u64>(param_2 + 0x48)
                && (rd::<u64>(param_2 + 0x30) == 0 || rd::<u64>(param_2 + 0x28) != nk)
            {
                break;
            }
        }
    }

    let lock_obj = rd::<usize>(param_1 + 0x60);
    exe::call_iat!(0x1_4264_55a8, (lock_obj));
    let mut local_b8 = [0u64; 16];
    let b8_ptr = local_b8.as_mut_ptr() as usize;
    exe::call_va!(0x1_401e_1660, (b8_ptr, lock_obj.wrapping_add(0x08), 0x20u32));
    exe::call_va!(0x1_413a_1e70, (lock_obj.wrapping_add(0x08), param_2));
    exe::call_va!(0x1_413a_1e70, (param_2, b8_ptr));
    exe::call_va!(0x1_401e_15d0, (b8_ptr));
    exe::call_va!(
        0x1_413a_1a60,
        (lock_obj.wrapping_add(0x70), param_2.wrapping_add(0x68))
    );
    exe::call_iat!(0x1_4264_5598, (lock_obj));

    for i in 0..(keys_len as usize) {
        let k = rd::<u64>(keys_ptr.wrapping_add(i * 8));
        exe::call_va!(0x1_4242_3190, (param_2, k));
    }

    let mut cur3 = rd::<usize>(param_2 + 0x60);
    let end3 = cur3.wrapping_add((rd::<usize>(param_2 + 0x40)).wrapping_mul(0x18));
    while cur3 != end3 {
        let k = rd::<u64>(cur3);
        if k != rd::<u64>(param_2 + 0x48)
            && (rd::<u64>(param_2 + 0x30) == 0 || rd::<u64>(param_2 + 0x28) != k)
        {
            break;
        }
        cur3 = cur3.wrapping_add(0x18);
    }

    let end3_check = rd::<usize>(param_2 + 0x60)
        .wrapping_add((rd::<usize>(param_2 + 0x40)).wrapping_mul(0x18));
    while cur3 != end3_check {
        let k = rd::<u64>(cur3);
        exe::call_va!(0x1_4243_fc10, (rd::<u64>(param_1 + 0x58), k));
        loop {
            cur3 = cur3.wrapping_add(0x18);
            if cur3 == end3 {
                break;
            }
            let nk = rd::<u64>(cur3);
            if nk != rd::<u64>(param_2 + 0x48)
                && (rd::<u64>(param_2 + 0x30) == 0 || rd::<u64>(param_2 + 0x28) != nk)
            {
                break;
            }
        }
    }

    exe::call_va!(0x1_4243_d920, (param_1));
    if keys_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (keys_ptr));
    }
    let p68 = rd::<usize>(param_2 + 0x68);
    if p68 != 0 {
        let base_ffe8 = p68.wrapping_sub(0xffe8);
        exe::call_va!(0x1_401e_17d0, (p68.wrapping_add(0x10)));
        exe::call_va!(0x1_4244_2de0, (base_ffe8));
    }
    exe::call_va!(0x1_401e_15d0, (param_2));
}

#[inline]
unsafe fn trim_trailing_slashes(ptr: usize, len: usize) -> [u64; 2] {
    let lut = exe::ptr::<u8>(0x1_426d_01e0) as usize;
    let sep = rd::<usize>(exe::ptr::<usize>(0x1_4307_9458) as usize);
    let end = ptr.wrapping_add(len);
    let mut cur = end;

    while cur != ptr {
        let b = rd::<u8>(cur - 1) as usize;
        let b_folded = rd::<u8>(lut + b);
        let mut matched = false;
        for i in 0..2usize {
            let s = rd::<u8>(sep + i) as usize;
            if rd::<u8>(lut + s) == b_folded {
                matched = true;
                break;
            }
        }
        if !matched {
            break;
        }
        cur -= 1;
    }

    let trimmed_len = cur.wrapping_sub(ptr);
    if trimmed_len == 0 && len != 0 {
        let b0 = rd::<u8>(ptr);
        if b0 == b'/' || b0 == b'\\' {
            return [exe::ptr::<u8>(0x1_4307_8be8) as usize as u64, 1];
        }
    }
    [ptr as u64, trimmed_len as u64]
}

#[inline]
unsafe fn rstring_data_ptr(rstr: usize) -> usize {
    let b3 = rd::<u8>(rstr + 0x03) as u32;
    let cap_bits = if b3 < 0x80 {
        b3.wrapping_mul(8)
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let len_bits = (rd::<u32>(rstr) & 0x00ff_ffff) << 3;
    if len_bits > cap_bits {
        rd::<usize>(rstr + 0x08)
    } else {
        rstr.wrapping_add(0x08)
    }
}

/// `r_142477320`
/// Original: `0x142477320` (1589 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn r_142477320(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
) {
    let count1 = rd::<u32>(param_4 + 0x08) as usize;
    let start1 = rd::<u32>(param_4 + 0x04) as usize;
    let mut rbx = rd::<usize>(param_3 + 0x18).wrapping_add(start1.wrapping_mul(0x1c));
    let end1 = rbx.wrapping_add(count1.wrapping_mul(0x1c));

    while rbx != end1 {
        let len = rd::<u32>(rbx + 0x18) as usize;
        let off = rd::<u32>(rbx + 0x14) as usize;
        let ptr = rd::<usize>(param_3 + 0x48).wrapping_add(off);
        let mut sv = trim_trailing_slashes(ptr, len);
        exe::call_va!(0x1_401e_01e0, (param_2, sv.as_mut_ptr() as usize));

        let mut p1_copy = rd::<[u64; 2]>(param_1);
        exe::call_va!(
            0x1_4247_7320,
            (p1_copy.as_mut_ptr() as usize, param_2, param_3, rbx, param_5)
        );
        if rd::<u32>(param_2 + 0x04) != 0 {
            exe::call_va!(0x1_401e_00a0, (param_2, 0u8));
            if rd::<u32>(param_2 + 0x04) == 0 {
                exe::call_va!(0x1_4240_a590, (param_2, 0x2eu8));
            }
        }
        rbx = rbx.wrapping_add(0x1c);
    }

    let count2 = rd::<u32>(param_4 + 0x10) as usize;
    let start2 = rd::<u32>(param_4 + 0x0c) as usize;
    let mut entry = rd::<usize>(param_3 + 0x28).wrapping_add(start2.wrapping_mul(0x20));
    let end2 = entry.wrapping_add(count2.wrapping_mul(0x20));

    while entry != end2 {
        if rd::<u32>(entry + 0x1c) != 0 {
            let len = rd::<u32>(entry + 0x10) as usize;
            let off = rd::<u32>(entry + 0x0c) as usize;
            let ptr = rd::<usize>(param_3 + 0x48).wrapping_add(off);
            let mut sv = trim_trailing_slashes(ptr, len);
            exe::call_va!(0x1_401e_01e0, (param_2, sv.as_mut_ptr() as usize));

            let p2_ptr = rstring_data_ptr(param_2);
            let p2_len = rd::<u32>(param_2 + 0x04) as u64;

            let mut local_str = [0u64; 4];
            let l_ptr = local_str.as_mut_ptr() as usize;
            wr::<u32>(l_ptr, 0x0200_0002);
            wr::<u32>(l_ptr + 0x04, 0);
            wr::<u8>(l_ptr + 0x08, 0);

            if p2_len == 0 {
                let mut p1_copy = rd::<[u64; 2]>(param_1);
                exe::call_va!(0x1_401e_01e0, (l_ptr, p1_copy.as_mut_ptr() as usize));
            } else if rd::<u8>(p2_ptr) == b'/'
                || rd::<u8>(p2_ptr) == b'\\'
                || (p2_len >= 2 && rd::<u8>(p2_ptr + 1) == b':')
            {
                let mut p2_sv: [u64; 2] = [p2_ptr as u64, p2_len];
                exe::call_va!(0x1_401e_01e0, (l_ptr, p2_sv.as_mut_ptr() as usize));
            } else {
                let views: [[u64; 2]; 2] = [rd::<[u64; 2]>(param_1), [p2_ptr as u64, p2_len]];
                let mut start_idx: usize = 0;
                let mut total_len: u64 = 0;
                for i in 0..2usize {
                    let v_ptr = views[i][0] as usize;
                    let v_len = views[i][1];
                    if v_len != 0
                        && (rd::<u8>(v_ptr) == b'/'
                            || rd::<u8>(v_ptr) == b'\\'
                            || (v_len >= 2 && rd::<u8>(v_ptr + 1) == b':'))
                    {
                        start_idx = i;
                        total_len = v_len;
                    } else {
                        total_len = total_len.wrapping_add(v_len);
                    }
                }
                let cap_check = ((rd::<u32>(l_ptr) & 0x00ff_ffff) as u64) << 3;
                if total_len >= cap_check {
                    exe::call_va!(0x1_4240_8f50, (l_ptr, total_len));
                }
                for j in start_idx..2usize {
                    let mut sv_j = views[j];
                    exe::call_va!(0x1_401e_01e0, (l_ptr, sv_j.as_mut_ptr() as usize));
                }
            }

            let d_ptr = rstring_data_ptr(l_ptr);
            let d_len = rd::<u32>(l_ptr + 0x04) as u64;

            let p2_ptr2 = rstring_data_ptr(param_2);
            let p2_len2 = rd::<u32>(param_2 + 0x04) as u64;
            let mut p2_sv2: [u64; 2] = [p2_ptr2 as u64, p2_len2];
            let mut out_hash: u64 = 0;
            let hash_ptr = exe::call_va!(
                0x1_4242_3da0,
                ((&mut out_hash as *mut u64) as usize, p2_sv2.as_mut_ptr() as usize) -> usize
            );
            let mut hash_val = rd::<u64>(hash_ptr);
            let mut d_sv: [u64; 2] = [d_ptr as u64, d_len];

            let fn_obj = rd::<usize>(param_5 + 0x38);
            if fn_obj == 0 {
                exe::call_iat!(0x1_4264_5878, ());
                return;
            }
            let fn_vtbl = rd::<usize>(fn_obj);
            exe::call_va!(
                rd::<usize>(fn_vtbl + 0x10),
                (
                    fn_obj,
                    (&mut hash_val as *mut u64) as usize,
                    d_sv.as_mut_ptr() as usize
                )
            );
            exe::call_va!(0x1_4240_ab10, (l_ptr));
            if rd::<u32>(param_2 + 0x04) != 0 {
                exe::call_va!(0x1_401e_00a0, (param_2, 0u8));
                if rd::<u32>(param_2 + 0x04) == 0 {
                    exe::call_va!(0x1_4240_a590, (param_2, 0x2eu8));
                }
            }
        }
        entry = entry.wrapping_add(0x20);
    }
}

/// `r_142478340`
/// Original: `0x142478340` (628 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn r_142478340(param_1: usize, param_2: usize, param_3: usize) {
    let mut path_sv = rd::<[u64; 2]>(param_2);
    let mut pair_20 = [0u64; 2];
    let _ = exe::call_va!(
        0x1_4247_85d0,
        (
            pair_20.as_mut_ptr() as usize,
            param_1,
            path_sv.as_mut_ptr() as usize
        ) -> usize
    );

    let (data_len, data_ptr): (u64, usize) = if pair_20[0] != 0 {
        let entry_ptr = pair_20[1] as usize;
        let len = rd::<u32>(entry_ptr + 0x1c) as u64;
        let off = rd::<u32>(entry_ptr + 0x18) as usize;
        let base = rd::<usize>((pair_20[0] as usize) + 0x58);
        (len, base.wrapping_add(off))
    } else {
        (0, 0)
    };

    let dst_38 = rd::<usize>(param_3 + 0x38);
    let (r14, rdi, r12, r13, r15): (u64, u64, u64, u64, usize) = if dst_38 != 0 {
        let cap_30 = rd::<u64>(param_3 + 0x30);
        let copy_len = if data_len < cap_30 { data_len } else { cap_30 };
        let _ = exe::call_va!(0x1_4183_acbc, (dst_38, data_ptr, copy_len) -> usize);
        (
            rd::<u64>(param_3 + 0x38),
            rd::<u64>(param_3 + 0x30),
            0,
            0,
            0,
        )
    } else {
        let alloc_obj = rd::<usize>(param_3 + 0x50);
        let vtbl = rd::<usize>(alloc_obj);
        let allocated = exe::call_va!(
            rd::<usize>(vtbl + 0x08),
            (alloc_obj, data_len, 0x10u64) -> usize
        );
        let _ = exe::call_va!(0x1_4183_acbc, (allocated, data_ptr, data_len) -> usize);
        (
            allocated as u64,
            data_len,
            allocated as u64,
            data_len,
            alloc_obj,
        )
    };

    let mut pkt_70 = [0u64; 12]; // 0x60 bytes
    let p70 = pkt_70.as_mut_ptr() as usize;
    wr::<u32>(p70, 0x0200_0002);
    wr::<u32>(p70 + 0x04, 0);
    wr::<u8>(p70 + 0x08, 0);
    let mut path_sv2 = rd::<[u64; 2]>(param_2);
    exe::call_va!(0x1_401e_01e0, (p70, path_sv2.as_mut_ptr() as usize));

    wr::<u64>(p70 + 0x18, 1);
    wr::<u64>(p70 + 0x20, r14);
    wr::<u64>(p70 + 0x28, rdi);
    wr::<u64>(p70 + 0x30, r12);
    wr::<u64>(p70 + 0x38, r13);
    wr::<u16>(p70 + 0x40, 0x10);
    wr::<usize>(p70 + 0x48, r15);
    wr::<u32>(p70 + 0x50, 3);
    wr::<u64>(p70 + 0x58, rd::<u64>(param_3 + 0x28));

    let mut out_d0 = [0u64; 16]; // 0x80 bytes (>= 0x60 for r::LoadedData)
    let pd0 = out_d0.as_mut_ptr() as usize;
    wr::<u16>(pd0 + 0x20, 0x10);
    let r_7d0 = exe::call_va!(0x1_4101_f7d0, (pd0, p70) -> usize);
    exe::call_va!(0x1_413b_bda0, (param_3.wrapping_add(0x18), r_7d0));

    let rem_ptr = rd::<usize>(p70 + 0x30);
    if rem_ptr != 0 {
        let align_w = rd::<u16>(p70 + 0x40) as u32;
        let alloc_obj = rd::<usize>(p70 + 0x48);
        let vtbl = rd::<usize>(alloc_obj);
        let rem_len = rd::<u64>(p70 + 0x38);
        exe::call_va!(
            rd::<usize>(vtbl + 0x10),
            (alloc_obj, rem_ptr, rem_len, align_w)
        );
    }
    exe::call_va!(0x1_4240_ab10, (p70));
}

/// `r_1424785d0`
/// Original: `0x1424785d0` (339 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn r_1424785d0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let mut sv_in = rd::<[u64; 2]>(param_3);
    let mut res_50 = [0u64; 3];
    let r50_ptr = res_50.as_mut_ptr() as usize;
    let _ = exe::call_va!(
        0x1_4247_8780,
        (r50_ptr, param_2, sv_in.as_mut_ptr() as usize) -> usize
    );

    if rd::<u8>(r50_ptr + 0x10) != 0 {
        let saved_sv = [res_50[0], res_50[1]];
        let mut sv_20 = saved_sv;
        let check_fn = rd::<usize>(param_2 + 0x08);
        let skip = exe::call_va!(check_fn, (sv_20.as_mut_ptr() as usize) -> u8);
        if skip == 0 {
            let lock_ptr = param_2.wrapping_add(0x28);
            exe::call_iat!(0x1_4264_55a8, (lock_ptr));
            let mut cur = rd::<usize>(param_2 + 0x30);
            let end = cur.wrapping_add((rd::<u32>(param_2 + 0x38) as usize) * 8);
            while cur != end {
                let pack = rd::<usize>(cur);
                sv_20 = saved_sv;
                let mut tmp_40 = [0u64; 2];
                let p_dir = exe::call_va!(
                    0x1_40f3_5ac0,
                    (sv_20.as_mut_ptr() as usize, tmp_40.as_mut_ptr() as usize) -> usize
                );
                let mut dir_sv = rd::<[u64; 2]>(p_dir);
                let dir_node = exe::call_va!(
                    0x1_4243_2c60,
                    (pack, dir_sv.as_mut_ptr() as usize) -> usize
                );
                if dir_node != 0 {
                    let p_file = exe::call_va!(
                        0x1_401d_fa80,
                        (sv_20.as_mut_ptr() as usize, r50_ptr) -> usize
                    );
                    let mut file_sv = rd::<[u64; 2]>(p_file);
                    let file_entry = exe::call_va!(
                        0x1_4243_2790,
                        (pack, dir_node, file_sv.as_mut_ptr() as usize) -> usize
                    );
                    if file_entry != 0 {
                        wr::<usize>(param_1, pack);
                        wr::<usize>(param_1 + 0x08, file_entry);
                        exe::call_iat!(0x1_4264_5598, (lock_ptr));
                        return param_1;
                    }
                }
                cur = cur.wrapping_add(8);
            }
            wr::<u64>(param_1, 0);
            wr::<u64>(param_1 + 0x08, 0);
            exe::call_iat!(0x1_4264_5598, (lock_ptr));
            return param_1;
        }
    }

    wr::<u64>(param_1, 0);
    wr::<u64>(param_1 + 0x08, 0);
    param_1
}

/// `r_142478780`
/// Original: `0x142478780` (234 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn r_142478780(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let s_ptr = rd::<usize>(param_3);
    let s_len = rd::<u64>(param_3 + 0x08);
    if s_len != 0
        && (rd::<u8>(s_ptr) == b'/'
            || rd::<u8>(s_ptr) == b'\\'
            || (s_len >= 2 && rd::<u8>(s_ptr + 1) == b':'))
    {
        let mut buf_40 = [0u64; 8];
        let b40_ptr = buf_40.as_mut_ptr() as usize;
        exe::call_va!(
            0x1_413a_7e00,
            (b40_ptr, param_3, param_2.wrapping_add(0x10))
        );
        if rd::<u64>(b40_ptr + 0x30) != u64::MAX {
            wr::<u8>(param_1 + 0x10, 0);
            return param_1;
        }
        let pos = rd::<u64>(b40_ptr + 0x10);
        let mut sub_30 = [0u64; 2];
        if pos != u64::MAX {
            let len_48 = rd::<u64>(b40_ptr + 0x08);
            if len_48 < pos {
                exe::call_va!(0x1_401d_e590, ());
                return param_1;
            }
            let rem = len_48.wrapping_sub(pos);
            let slice_len = if rem < u64::MAX { rem } else { u64::MAX };
            let mut slice_20: [u64; 2] = [rd::<u64>(b40_ptr).wrapping_add(pos), slice_len];
            exe::call_va!(
                0x1_401d_f850,
                (sub_30.as_mut_ptr() as usize, slice_20.as_mut_ptr() as usize)
            );
        }
        wr::<[u64; 2]>(param_1, sub_30);
        wr::<u8>(param_1 + 0x10, 1);
        return param_1;
    }

    let orig_sv = rd::<[u64; 2]>(param_3);
    wr::<[u64; 2]>(param_1, orig_sv);
    wr::<u8>(param_1 + 0x10, 1);
    param_1
}

/// `unknown_14247dea0`
/// Original: `0x14247dea0` (323 bytes, `Throws::No`)
pub unsafe extern "win64" fn unknown_14247dea0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: u64,
    param_5: usize,
) -> usize {
    let mut pkt_40 = [0u64; 11]; // 0x58 bytes
    let p40 = pkt_40.as_mut_ptr() as usize;
    let r15 = param_1.wrapping_add(0x2c8);

    let flag_48: u8 = if rd::<u8>(param_5 + 0x40) != 0 { 5 } else { 1 };
    let u32_4c = rd::<u32>(param_5 + 0x24);
    let u32_50 = rd::<u32>(param_5 + 0x20);

    wr::<u64>(p40, 0);
    wr::<u8>(p40 + 0x08, flag_48);
    wr::<u32>(p40 + 0x0c, u32_4c);
    wr::<u32>(p40 + 0x10, u32_50);
    wr::<u64>(p40 + 0x18, param_4);
    wr::<usize>(p40 + 0x20, r15);
    wr::<u64>(p40 + 0x28, rd::<u64>(param_5 + 0x28));
    wr::<[u64; 2]>(p40 + 0x30, rd::<[u64; 2]>(param_5 + 0x10));
    wr::<[u64; 2]>(p40 + 0x40, rd::<[u64; 2]>(param_5));
    wr::<u64>(p40 + 0x50, rd::<u64>(param_5 + 0x38));

    wr::<u64>(param_2, 0);
    let prev_508 = crate::simd_ops::atomic_xadd_u32((param_1 + 0x508) as u64, 2);
    wr::<u32>(param_2 + 0x04, prev_508);

    let mut cur = rd::<usize>(param_1 + 0x4e8);
    let count = rd::<u32>(param_1 + 0x4f0) as usize;
    let end = cur.wrapping_add(count * 8);
    let mut idx: u32 = 0;

    while cur != end {
        let handler = rd::<usize>(cur);
        let vtbl = rd::<usize>(handler);
        let mut sv_20 = rd::<[u64; 2]>(param_3);
        let matched = exe::call_va!(
            rd::<usize>(vtbl + 0x08),
            (handler, sv_20.as_mut_ptr() as usize, u32_50, flag_48) -> u8
        );
        if matched != 0 {
            wr::<u32>(param_2, idx);
            let _ = crate::simd_ops::atomic_xadd_u64(r15 as u64, 1);
            wr::<u64>(p40, rd::<u64>(param_2));
            let mut sv_30 = rd::<[u64; 2]>(param_3);
            let handler2 = rd::<usize>(cur);
            let vtbl2 = rd::<usize>(handler2);
            exe::call_va!(
                rd::<usize>(vtbl2 + 0x10),
                (handler2, sv_30.as_mut_ptr() as usize, p40)
            );
            break;
        }
        idx = idx.wrapping_add(1);
        cur = cur.wrapping_add(8);
    }

    param_2
}

/// `unknown_1424b06b0`
/// Original: `0x1424b06b0` (511 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn unknown_1424b06b0() {
    let lock_ptr = exe::ptr::<u8>(0x1_4387_1780) as usize;
    let tid_ptr = exe::ptr::<u32>(0x1_4387_1788) as usize;
    let cnt_ptr = exe::ptr::<u32>(0x1_4387_178c) as usize;
    let g_3e0 = exe::ptr::<usize>(0x1_43a0_83e0) as usize;
    let g_3f0 = exe::ptr::<usize>(0x1_43a0_83f0) as usize;
    let g_3d0 = exe::ptr::<usize>(0x1_43a0_83d0) as usize;
    let g_3d8 = exe::ptr::<usize>(0x1_43a0_83d8) as usize;
    let g_3e8 = exe::ptr::<usize>(0x1_43a0_83e8) as usize;
    let g_dirty = exe::ptr::<u8>(0x1_4397_d8df) as usize;

    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(tid_ptr) != tid {
        exe::call_iat!(0x1_4264_55a8, (lock_ptr));
        wr::<u32>(tid_ptr, tid);
    }
    wr::<u32>(cnt_ptr, rd::<u32>(cnt_ptr).wrapping_add(1));

    let mut v_3e0 = rd::<usize>(g_3e0);
    if v_3e0 == 0 {
        let new_3e0 = exe::call_va!(0x1_413d_8950, () -> usize);
        wr::<usize>(g_3e0, new_3e0);
        let new_3f0 = exe::call_va!(0x1_413d_8a30, () -> usize);
        wr::<usize>(g_3f0, new_3f0);
        v_3e0 = rd::<usize>(g_3e0);
    }

    if rd::<u8>(g_dirty) != 0 {
        exe::call_va!(0x1_401f_9d90, (v_3e0));
        exe::call_va!(0x1_4115_5550, (rd::<usize>(g_3f0)));
        exe::call_va!(0x1_424b_03d0, (rd::<usize>(g_3d0)));

        let mut node_3d8 = rd::<usize>(g_3d8);
        while node_3d8 != 0 {
            exe::call_va!(0x1_424b_03d0, (rd::<usize>(node_3d8)));
            node_3d8 = rd::<usize>(node_3d8 + 0x10);
        }

        let sentinel = rd::<usize>(rd::<usize>(g_3f0) + 0x08);
        let mut rbx = rd::<usize>(sentinel);
        if rbx != sentinel {
            loop {
                let rdi = rd::<usize>(rbx + 0x18);
                let vtbl1 = rd::<usize>(rdi);
                let obj1 = exe::call_va!(rd::<usize>(vtbl1 + 0x20), (rdi) -> usize);
                if obj1 != 0 {
                    let obj1_vtbl = rd::<usize>(obj1);
                    let mut id1 = exe::call_va!(rd::<usize>(obj1_vtbl + 0x08), (obj1) -> u32);
                    let mut iter_buf1 = [0u64; 2];
                    let res1 = exe::call_va!(
                        0x1_413d_8bf0,
                        (
                            rd::<usize>(g_3f0),
                            iter_buf1.as_mut_ptr() as usize,
                            (&mut id1 as *mut u32) as usize
                        ) -> usize
                    );
                    let target1 = rd::<usize>(rd::<usize>(res1) + 0x18);
                    wr::<usize>(rdi + 0x28, rd::<usize>(target1 + 0x20));
                    wr::<usize>(target1 + 0x20, rdi);
                }

                let vtbl2 = rd::<usize>(rdi);
                let obj2 = exe::call_va!(rd::<usize>(vtbl2 + 0x88), (rdi) -> usize);
                if obj2 != 0 {
                    let obj2_vtbl = rd::<usize>(obj2);
                    let mut id2 = exe::call_va!(rd::<usize>(obj2_vtbl + 0x08), (obj2) -> u32);
                    let mut iter_buf2 = [0u64; 2];
                    let res2 = exe::call_va!(
                        0x1_413d_8bf0,
                        (
                            rd::<usize>(g_3f0),
                            iter_buf2.as_mut_ptr() as usize,
                            (&mut id2 as *mut u32) as usize
                        ) -> usize
                    );
                    let target2 = rd::<usize>(rd::<usize>(res2) + 0x18);
                    wr::<usize>(rdi + 0x18, rd::<usize>(target2 + 0x18));
                    wr::<usize>(target2 + 0x18, rdi);
                }

                rbx = rd::<usize>(rbx);
                if rbx == rd::<usize>(rd::<usize>(g_3f0) + 0x08) {
                    break;
                }
            }
        }

        wr::<u8>(g_dirty, 0);

        let mut cb = rd::<usize>(g_3e8);
        while cb != 0 {
            let vtbl = rd::<usize>(cb);
            exe::call_va!(rd::<usize>(vtbl), (cb));
            cb = rd::<usize>(cb + 0x08);
        }

        let mut outer = rd::<usize>(g_3d8);
        while outer != 0 {
            let mut inner = rd::<usize>(outer + 0x08);
            while inner != 0 {
                let vtbl = rd::<usize>(inner);
                exe::call_va!(rd::<usize>(vtbl), (inner));
                inner = rd::<usize>(inner + 0x08);
            }
            outer = rd::<usize>(outer + 0x10);
        }
    }

    let new_cnt = rd::<u32>(cnt_ptr).wrapping_sub(1);
    wr::<u32>(cnt_ptr, new_cnt);
    if new_cnt == 0 {
        let _ = crate::simd_ops::atomic_xchg_u32(tid_ptr as u64, 0xffff_ffff);
        exe::call_iat!(0x1_4264_5598, (lock_ptr));
    }
}

/// `unknown_1424b3cb0`
/// Original: `0x1424b3cb0` (274 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn unknown_1424b3cb0(param_1: usize, param_2: usize) {
    let span_size = rd::<u64>(param_2 + 0x18).wrapping_sub(rd::<u64>(param_2));
    if span_size <= rd::<u64>(param_1 + 0x98) {
        let tid = exe::call_iat!(0x1_4264_5790, () -> u32) as u64;
        if rd::<u64>(param_1 + 0x10) == tid {
            let _ = crate::simd_ops::atomic_xadd_u64((param_1 + 0x08) as u64, 1);
        } else {
            exe::call_va!(0x1_4137_9040, (param_1.wrapping_add(0x08), 0u64, 1u64));
        }

        let g_ce14 = exe::ptr::<u16>(0x1_43c8_ce14) as usize;
        let prev_tag = crate::simd_ops::atomic_xadd_u16(g_ce14 as u64, 1);
        let min_t = rd::<f64>(exe::ptr::<f64>(0x1_4342_6a48) as usize);
        let rdi = ((param_2 as u64) & 0x0000_ffff_ffff_ffff) | ((prev_tag as u64) << 48);
        let r15 = (rdi & 0x0000_ffff_ffff_ffff) as usize;
        let mut rbx = rd::<u64>(param_1);

        loop {
            wr::<u64>(r15 + 0x28, rbx);
            let get_time: unsafe extern "win64" fn() -> f64 = exe::func(0x1_4240_69a0);
            let t = get_time();
            let clamped = if t > min_t { t } else { min_t };
            wr::<f64>(param_2 + 0x30, clamped);
            let (ok, actual) = crate::simd_ops::atomic_cmpxchg_u64(param_1 as u64, rbx, rdi);
            if ok {
                break;
            }
            rbx = actual;
        }

        wr::<u32>(param_1 + 0xa0, rd::<u32>(param_1 + 0xa0).wrapping_add(1));
        let _ = crate::simd_ops::atomic_xadd_u64((param_1 + 0x08) as u64, u64::MAX);
        return;
    }

    let mut ptr_minus_40: u64 = rd::<u64>(param_2).wrapping_sub(0x40);
    let fn_obj = rd::<usize>(param_1 + 0x90);
    if fn_obj == 0 {
        exe::call_iat!(0x1_4264_5878, ());
        return;
    }
    let vtbl = rd::<usize>(fn_obj);
    exe::call_va!(
        rd::<usize>(vtbl + 0x10),
        (fn_obj, (&mut ptr_minus_40 as *mut u64) as usize)
    );
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1423d8720(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_SUM: u64 = 0;

    unsafe extern "win64" fn mock_af80(rcx: u64, rdx: u64) {
        LOG_SUM = LOG_SUM.wrapping_add(rcx.wrapping_mul(3)).wrapping_add(rdx);
    }
    unsafe extern "win64" fn mock_5e30(_r15: usize, rdx: u64, r8_ptr: usize) {
        let v = rd::<u64>(r8_ptr);
        LOG_SUM = LOG_SUM.wrapping_add(rdx).wrapping_add(v);
    }
    unsafe extern "win64" fn mock_0ed0(rcx: u64) {
        LOG_SUM = LOG_SUM.wrapping_add(rcx);
    }
    unsafe extern "win64" fn mock_4530(rcx_entry: usize) {
        let v = rd::<u64>(rcx_entry + 0x10);
        LOG_SUM = LOG_SUM.wrapping_add(v);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4239_af80, mock_af80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4225_5e30, mock_5e30 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4242_0ed0, mock_0ed0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_412f_4530, mock_4530 as *const ())?;

    let g_ff10 = exe::ptr::<usize>(0x1_4397_ff10) as usize;
    let g_82b8 = exe::ptr::<u64>(0x1_43a0_82b8) as usize;
    let g_ff00 = exe::ptr::<u32>(0x1_4397_ff00) as usize;
    let g_8298 = exe::ptr::<usize>(0x1_43a0_8298) as usize;
    let saved = (
        rd::<usize>(g_ff10),
        rd::<u64>(g_82b8),
        rd::<u32>(g_ff00),
        rd::<usize>(g_8298),
    );

    let run = |f: F, count: u64| -> (u32, u32, u32, u32, u32, u64) {
        LOG_SUM = 0;

        let mut node = [0u8; 0x90];
        let np = node.as_mut_ptr() as usize;

        let mut entries0 = [0u64; 5];
        entries0[0] = np as u64;
        entries0[1] = 0x1111;
        entries0[2] = 0x2222;
        entries0[3] = 0x3333;
        entries0[4] = 0x4444;

        let mut mgr = [0u8; 0x300];
        let mp = mgr.as_mut_ptr() as usize;
        wr::<u32>(mp + 0x2c0, 0xffff_ffff);
        wr::<u32>(mp + 0x2c4, 0);
        wr::<usize>(mp + 0x2c8, entries0.as_mut_ptr() as usize);
        wr::<u32>(mp + 0x2d0, 1);
        wr::<u32>(mp + 0x2e8, 0);

        let mut wake_obj = [0u8; 0x270];
        let wp = wake_obj.as_mut_ptr() as usize;
        wr::<u32>(wp + 0x258, 10);

        wr::<usize>(g_ff10, mp);
        wr::<u64>(g_82b8, 0x7777);
        wr::<u32>(g_ff00, 1);
        wr::<usize>(g_8298, wp);

        f(0, 0, count);

        (
            rd::<u32>(np + 0x80),
            rd::<u32>(mp + 0x2d0),
            rd::<u32>(mp + 0x2e8),
            rd::<u32>(wp + 0x258),
            rd::<u32>(g_ff00),
            LOG_SUM,
        )
    };

    for cnt in [0u64, 1u64] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        wr::<usize>(g_ff10, saved.0);
        wr::<u64>(g_82b8, saved.1);
        wr::<u32>(g_ff00, saved.2);
        wr::<usize>(g_8298, saved.3);
        crate::diff_eq!(o, n, "1423d8720 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_r_14242df50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TARGET_VTBL: [usize; 16] = [0; 16];
    static mut TARGET_OBJ: [usize; 2] = [0; 2];
    static mut TRACE_ACC: u64 = 0;

    unsafe extern "win64" fn mock_create(_ctx: usize, name_ptr: usize) -> u64 {
        let b0 = rd::<u8>(name_ptr) as u64;
        TRACE_ACC = TRACE_ACC.wrapping_add(b0);
        core::ptr::addr_of_mut!(TARGET_OBJ) as usize as u64
    }
    unsafe extern "win64" fn mock_9b50(b50: usize, span40: usize, r8: u64, r9: u8) {
        let p = rd::<u64>(span40);
        let h = rd::<u32>(b50) as u64;
        TRACE_ACC = TRACE_ACC
            .wrapping_add(p & 0xffff)
            .wrapping_add(h)
            .wrapping_add(r8)
            .wrapping_add(r9 as u64);
    }
    unsafe extern "win64" fn mock_d730(b190: usize, b50: usize, r8: u64, r9: u8, arg5: u8) {
        let ptr_110 = rd::<u64>(b50 + 0x110);
        let len_118 = rd::<u64>(b50 + 0x118);
        let flag_128 = rd::<u8>(b50 + 0x128) as u64;
        wr::<u64>(b190 + 0x20, ptr_110.wrapping_add(len_118));
        TRACE_ACC = TRACE_ACC
            .wrapping_add(len_118)
            .wrapping_add(flag_128)
            .wrapping_add(r8)
            .wrapping_add(r9 as u64)
            .wrapping_add(arg5 as u64);
    }
    unsafe extern "win64" fn mock_v68(_this: usize, b190: usize, r8: u32) {
        let v = rd::<u64>(b190 + 0x20);
        TRACE_ACC = TRACE_ACC.wrapping_add(v).wrapping_add(r8 as u64);
    }
    unsafe extern "win64" fn mock_e280(b190: usize) {
        let vft = rd::<u64>(b190 + 0x348);
        TRACE_ACC = TRACE_ACC.wrapping_add(vft & 0xffff);
    }
    unsafe extern "win64" fn mock_ab10(b50: usize) {
        let cleared = rd::<u8>(b50 + 0x128) as u64;
        TRACE_ACC = TRACE_ACC.wrapping_add(cleared + 0x55);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_d730, mock_d730 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401e_e280, mock_e280 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let run = |f: F| -> (u64, u64) {
        TRACE_ACC = 0;
        TARGET_VTBL = [0; 16];
        TARGET_VTBL[0x68 / 8] = mock_v68 as *const () as usize;
        TARGET_OBJ = [core::ptr::addr_of!(TARGET_VTBL) as usize, 0];

        let mut p4_vtbl = [0usize; 10];
        p4_vtbl[0x40 / 8] = mock_create as *const () as usize;
        let p4_obj: [usize; 1] = [p4_vtbl.as_ptr() as usize];

        let mut p1 = [0u64; 4];
        wr::<usize>(p1.as_mut_ptr() as usize + 0x18, 0x40_0000);

        let mut out_p2: u64 = 0;
        let param_3: u64 = ((0x0050u64 << 16) | 0x0010u64) << 32;
        let ret = f(
            p1.as_ptr() as usize,
            (&mut out_p2 as *mut u64) as usize,
            param_3,
            p4_obj.as_ptr() as usize,
        );
        assert_eq!(ret, (&out_p2 as *const u64) as usize);
        (out_p2, TRACE_ACC)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14242df50 out");
    Ok(1)
}

pub unsafe fn diff_r_14243de30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut KEY_BUF: [u64; 16] = [0; 16];
    static mut TRACE_SUM: u64 = 0;

    unsafe extern "win64" fn mock_ba80(_arr: usize, new_cap: u64) {
        TRACE_SUM = TRACE_SUM.wrapping_add(new_cap * 0x10);
    }
    unsafe extern "win64" fn mock_c0a0(bytes: u64, align: u32) -> usize {
        TRACE_SUM = TRACE_SUM.wrapping_add(bytes).wrapping_add(align as u64);
        core::ptr::addr_of_mut!(KEY_BUF) as usize
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {
        TRACE_SUM = TRACE_SUM.wrapping_add(0x190);
    }
    unsafe extern "win64" fn mock_1660(dst: usize, src: usize, flag: u32) {
        wr::<u64>(dst, rd::<u64>(src));
        TRACE_SUM = TRACE_SUM.wrapping_add(flag as u64);
    }
    unsafe extern "win64" fn mock_1e70(dst: usize, src: usize) {
        wr::<u64>(dst, rd::<u64>(src));
    }
    unsafe extern "win64" fn mock_15d0(_p: usize) {
        TRACE_SUM = TRACE_SUM.wrapping_add(0x15d);
    }
    unsafe extern "win64" fn mock_1a60(a: usize, b: usize) {
        let va = rd::<u64>(a);
        let vb = rd::<u64>(b);
        wr::<u64>(a, vb);
        wr::<u64>(b, va);
    }
    unsafe extern "win64" fn mock_3190(_p2: usize, k: u64) {
        TRACE_SUM = TRACE_SUM.wrapping_add(k * 7);
    }
    unsafe extern "win64" fn mock_fc10(p58: u64, k: u64) {
        TRACE_SUM = TRACE_SUM.wrapping_add(p58).wrapping_add(k * 11);
    }
    unsafe extern "win64" fn mock_d920(_p1: usize) {
        TRACE_SUM = TRACE_SUM.wrapping_add(0xd92);
    }
    unsafe extern "win64" fn mock_17d0(_p: usize) {
        TRACE_SUM = TRACE_SUM.wrapping_add(0x17d);
    }
    unsafe extern "win64" fn mock_2de0(_p: usize) {
        TRACE_SUM = TRACE_SUM.wrapping_add(0x2de);
    }

    let _m1 = crate::difftest::Mock::new(0x1_411d_ba80, mock_ba80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401e_1660, mock_1660 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_413a_1e70, mock_1e70 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_401e_15d0, mock_15d0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_413a_1a60, mock_1a60 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4242_3190, mock_3190 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4243_fc10, mock_fc10 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4243_d920, mock_d920 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_401e_17d0, mock_17d0 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;

    let run = |f: F| -> ([u64; 12], u32, u64) {
        KEY_BUF = [0; 16];
        TRACE_SUM = 0;

        let mut dst_items = [0u64; 12];
        let mut sub_80 = [0u8; 0x30];
        let s80 = sub_80.as_mut_ptr() as usize;
        wr::<usize>(s80 + 0x18, dst_items.as_mut_ptr() as usize);
        wr::<u32>(s80 + 0x20, 0);
        wr::<u32>(s80 + 0x24, 4);

        let mut lock_area = [0u8; 0x80];
        let lp = lock_area.as_mut_ptr() as usize;
        wr::<u64>(lp + 0x08, 0xaaaa);
        wr::<u64>(lp + 0x70, 0x50_0000);

        let mut p1 = [0u8; 0x90];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x58, 0x9999);
        wr::<usize>(p1p + 0x60, lp);
        wr::<usize>(p1p + 0x80, s80);

        let table_entries: [u64; 9] = [
            u64::MAX,
            0,
            0,
            0x101,
            0x202,
            0x303,
            0x404,
            0x505,
            0x606,
        ];
        let mut p2 = [0u8; 0x80];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u64>(p2p, 0xbbbb);
        wr::<u64>(p2p + 0x28, u64::MAX - 1);
        wr::<u64>(p2p + 0x30, 0);
        wr::<u64>(p2p + 0x38, 2);
        wr::<u64>(p2p + 0x40, 3);
        wr::<u64>(p2p + 0x48, u64::MAX);
        wr::<usize>(p2p + 0x60, table_entries.as_ptr() as usize);
        wr::<u64>(p2p + 0x68, 0);

        f(p1p, p2p);
        (dst_items, rd::<u32>(s80 + 0x20), TRACE_SUM)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14243de30 out");
    Ok(1)
}

pub unsafe fn diff_r_142477320(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CB_VTBL: [usize; 4] = [0; 4];
    static mut CB_OBJ: [usize; 1] = [0; 1];
    static mut HASH_LOG: u64 = 0;

    unsafe extern "win64" fn mock_01e0(rstr: usize, sv_ptr: usize) {
        let s_ptr = rd::<usize>(sv_ptr);
        let s_len = rd::<u64>(sv_ptr + 0x08) as usize;
        let cur_len = rd::<u32>(rstr + 0x04) as usize;
        let dst = rstr + 0x08 + cur_len;
        for i in 0..s_len {
            if cur_len + i < 16 {
                wr::<u8>(dst + i, rd::<u8>(s_ptr + i));
            }
        }
        wr::<u32>(rstr + 0x04, (cur_len + s_len) as u32);
    }
    unsafe extern "win64" fn mock_8f50(_rstr: usize, total_len: u64) {
        HASH_LOG = HASH_LOG.wrapping_add(total_len * 100);
    }
    unsafe extern "win64" fn mock_3da0(out_ptr: usize, sv_ptr: usize) -> usize {
        let len = rd::<u64>(sv_ptr + 0x08);
        wr::<u64>(out_ptr, 0xabc0 + len);
        out_ptr
    }
    unsafe extern "win64" fn mock_cb(_this: usize, hash_ptr: usize, d_sv_ptr: usize) {
        let h = rd::<u64>(hash_ptr);
        let d_len = rd::<u64>(d_sv_ptr + 0x08);
        let d_ptr = rd::<usize>(d_sv_ptr);
        let b0 = if d_len > 0 { rd::<u8>(d_ptr) as u64 } else { 0 };
        HASH_LOG = HASH_LOG.wrapping_add(h).wrapping_add(d_len * 17).wrapping_add(b0);
    }
    unsafe extern "win64" fn mock_ab10(_rstr: usize) {
        HASH_LOG = HASH_LOG.wrapping_add(1);
    }
    unsafe extern "win64" fn mock_00a0(rstr: usize, _flag: u8) {
        wr::<u32>(rstr + 0x04, 0);
    }
    unsafe extern "win64" fn mock_a590(rstr: usize, ch: u8) {
        HASH_LOG = HASH_LOG.wrapping_add(ch as u64);
        let _ = rstr;
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_01e0, mock_01e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_8f50, mock_8f50 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4242_3da0, mock_3da0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_401e_00a0, mock_00a0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4240_a590, mock_a590 as *const ())?;

    let run = |f: F| -> u64 {
        HASH_LOG = 0;
        CB_VTBL = [0; 4];
        CB_VTBL[0x10 / 8] = mock_cb as *const () as usize;
        CB_OBJ = [core::ptr::addr_of!(CB_VTBL) as usize];

        let str_pool = b"dir/file.bin\0";
        let mut files = [0u8; 0x20];
        let fp = files.as_mut_ptr() as usize;
        wr::<u32>(fp + 0x0c, 0);
        wr::<u32>(fp + 0x10, 8);
        wr::<u32>(fp + 0x1c, 1);

        let mut p3 = [0u8; 0x50];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<usize>(p3p + 0x18, 0);
        wr::<usize>(p3p + 0x28, fp);
        wr::<usize>(p3p + 0x48, str_pool.as_ptr() as usize);

        let mut p4 = [0u8; 0x20];
        let p4p = p4.as_mut_ptr() as usize;
        wr::<u32>(p4p + 0x04, 0);
        wr::<u32>(p4p + 0x08, 0);
        wr::<u32>(p4p + 0x0c, 0);
        wr::<u32>(p4p + 0x10, 1);

        let mut p5 = [0u8; 0x40];
        wr::<usize>(
            p5.as_mut_ptr() as usize + 0x38,
            core::ptr::addr_of_mut!(CB_OBJ) as usize,
        );

        let base_str = b"root/";
        let p1: [u64; 2] = [base_str.as_ptr() as u64, 5];

        let mut p2 = [0u64; 4];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u32>(p2p, 0x0200_0002);
        wr::<u32>(p2p + 0x04, 0);

        f(
            p1.as_ptr() as usize,
            p2p,
            p3p,
            p4p,
            p5.as_ptr() as usize,
        );
        HASH_LOG
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "142477320 out");
    Ok(1)
}

pub unsafe fn diff_r_142478340(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut PACK_DATA: [u8; 16] = *b"0123456789abcdef";
    static mut PACK_OBJ: [u8; 0x60] = [0; 0x60];
    static mut ENTRY_OBJ: [u8; 0x20] = [0; 0x20];
    static mut ALLOC_BUF: [u8; 16] = [0; 16];
    static mut ALLOC_VTBL: [usize; 4] = [0; 4];
    static mut ALLOC_OBJ: [usize; 1] = [0; 1];
    static mut TRACE_HASH: u64 = 0;

    unsafe extern "win64" fn mock_85d0(out_pair: usize, _p1: usize, _sv: usize) -> usize {
        wr::<usize>(out_pair, core::ptr::addr_of_mut!(PACK_OBJ) as usize);
        wr::<usize>(out_pair + 0x08, core::ptr::addr_of_mut!(ENTRY_OBJ) as usize);
        out_pair
    }
    unsafe extern "win64" fn mock_alloc(_this: usize, size: u64, align: u64) -> usize {
        TRACE_HASH = TRACE_HASH.wrapping_add(size).wrapping_add(align);
        core::ptr::addr_of_mut!(ALLOC_BUF) as usize
    }
    unsafe extern "win64" fn mock_free(_this: usize, _ptr: usize, size: u64, align: u32) {
        TRACE_HASH = TRACE_HASH
            .wrapping_add(size * 3)
            .wrapping_add(align as u64);
    }
    unsafe extern "win64" fn mock_01e0(rstr: usize, sv: usize) {
        wr::<u32>(rstr + 0x04, rd::<u32>(sv + 0x08));
    }
    unsafe extern "win64" fn mock_f7d0(out_d0: usize, in_70: usize) -> usize {
        let len = rd::<u64>(in_70 + 0x28);
        let tag = rd::<u32>(in_70 + 0x50) as u64;
        let p28 = rd::<u64>(in_70 + 0x58);
        wr::<u64>(out_d0, len + tag + p28);
        out_d0
    }
    unsafe extern "win64" fn mock_bda0(dst: usize, src: usize) {
        wr::<u64>(dst, rd::<u64>(src));
    }
    unsafe extern "win64" fn mock_ab10(_rstr: usize) {
        TRACE_HASH = TRACE_HASH.wrapping_add(0xab);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4247_85d0, mock_85d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_01e0, mock_01e0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4101_f7d0, mock_f7d0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_413b_bda0, mock_bda0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let run = |f: F, prealloc: bool| -> ([u8; 8], u64, u64) {
        TRACE_HASH = 0;
        ALLOC_BUF = [0; 16];
        wr::<usize>(
            core::ptr::addr_of_mut!(PACK_OBJ) as usize + 0x58,
            core::ptr::addr_of!(PACK_DATA) as usize,
        );
        wr::<u32>(core::ptr::addr_of_mut!(ENTRY_OBJ) as usize + 0x18, 2);
        wr::<u32>(core::ptr::addr_of_mut!(ENTRY_OBJ) as usize + 0x1c, 6);
        ALLOC_VTBL = [
            0,
            mock_alloc as *const () as usize,
            mock_free as *const () as usize,
            0,
        ];
        ALLOC_OBJ = [core::ptr::addr_of!(ALLOC_VTBL) as usize];

        let mut user_buf = [0u8; 8];
        let mut p3 = [0u8; 0x60];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<u64>(p3p + 0x28, 0x1000);
        if prealloc {
            wr::<u64>(p3p + 0x30, 8);
            wr::<usize>(p3p + 0x38, user_buf.as_mut_ptr() as usize);
        } else {
            wr::<usize>(
                p3p + 0x50,
                core::ptr::addr_of_mut!(ALLOC_OBJ) as usize,
            );
        }

        let p2: [u64; 2] = [0x1234, 5];
        f(0x9999, p2.as_ptr() as usize, p3p);

        let copied = if prealloc {
            user_buf
        } else {
            rd::<[u8; 8]>(core::ptr::addr_of!(ALLOC_BUF) as usize)
        };
        (copied, rd::<u64>(p3p + 0x18), TRACE_HASH)
    };

    for prealloc in [true, false] {
        let o = run(f_o, prealloc);
        let n = run(f_n, prealloc);
        crate::diff_eq!(o, n, "142478340 prealloc={}", prealloc);
    }
    Ok(2)
}

pub unsafe fn diff_r_1424785d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_8780(out: usize, _p2: usize, sv_in: usize) -> usize {
        wr::<[u64; 2]>(out, rd::<[u64; 2]>(sv_in));
        wr::<u8>(out + 0x10, 1);
        out
    }
    unsafe extern "win64" fn mock_check(sv: usize) -> u8 {
        if rd::<u64>(sv + 0x08) == 99 {
            1
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_5ac0(sv: usize, out: usize) -> usize {
        wr::<[u64; 2]>(out, rd::<[u64; 2]>(sv));
        out
    }
    unsafe extern "win64" fn mock_2c60(pack: usize, _dir_sv: usize) -> usize {
        if pack == 0x2000 {
            0x3333
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_fa80(sv: usize, out: usize) -> usize {
        wr::<[u64; 2]>(out, rd::<[u64; 2]>(sv));
        out
    }
    unsafe extern "win64" fn mock_2790(pack: usize, dir_node: usize, _file_sv: usize) -> usize {
        if pack == 0x2000 && dir_node == 0x3333 {
            0x4444
        } else {
            0
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4247_8780, mock_8780 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_40f3_5ac0, mock_5ac0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4243_2c60, mock_2c60 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401d_fa80, mock_fa80 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4243_2790, mock_2790 as *const ())?;

    let run = |f: F, len: u64| -> [u64; 2] {
        let packs: [usize; 2] = [0x1000, 0x2000];
        let mut p2 = [0u8; 0x40];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p + 0x08, mock_check as *const () as usize);
        wr::<usize>(p2p + 0x30, packs.as_ptr() as usize);
        wr::<u32>(p2p + 0x38, 2);

        let sv: [u64; 2] = [0x5000, len];
        let mut out = [0u64; 2];
        let ret = f(out.as_mut_ptr() as usize, p2p, sv.as_ptr() as usize);
        assert_eq!(ret, out.as_ptr() as usize);
        out
    };

    for len in [10u64, 99u64] {
        let o = run(f_o, len);
        let n = run(f_n, len);
        crate::diff_eq!(o, n, "1424785d0 len={}", len);
    }
    Ok(2)
}

pub unsafe fn diff_r_142478780(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut MODE_7E00: u32 = 0;

    unsafe extern "win64" fn mock_7e00(out_40: usize, _p3: usize, _p2_10: usize) {
        let mode = MODE_7E00;
        match mode {
            0 => {
                // Fail: [out_40 + 0x30] != u64::MAX
                wr::<u64>(out_40 + 0x30, 0);
            }
            1 => {
                // Empty slice: [out_40 + 0x30] == u64::MAX, [out_40 + 0x10] == u64::MAX
                wr::<u64>(out_40 + 0x30, u64::MAX);
                wr::<u64>(out_40 + 0x10, u64::MAX);
            }
            _ => {
                // Valid substring: ptr = 0x8000, len = 10, pos = 3
                wr::<u64>(out_40, 0x8000);
                wr::<u64>(out_40 + 0x08, 10);
                wr::<u64>(out_40 + 0x10, 3);
                wr::<u64>(out_40 + 0x30, u64::MAX);
            }
        }
    }
    unsafe extern "win64" fn mock_f850(out_30: usize, in_20: usize) {
        wr::<[u64; 2]>(out_30, rd::<[u64; 2]>(in_20));
    }

    let _m1 = crate::difftest::Mock::new(0x1_413a_7e00, mock_7e00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_f850, mock_f850 as *const ())?;

    let rel_str = b"relative/path";
    let abs_str = b"/abs/path";

    for (is_abs, mode) in [(false, 0u32), (true, 0), (true, 1), (true, 2)] {
        MODE_7E00 = mode;
        let s_bytes: &[u8] = if is_abs { abs_str } else { rel_str };
        let sv: [u64; 2] = [s_bytes.as_ptr() as u64, s_bytes.len() as u64];

        let run = |f: F| -> (u64, u64, u8) {
            let mut out = [0u64; 3];
            let op = out.as_mut_ptr() as usize;
            let ret = f(op, 0x1000, sv.as_ptr() as usize);
            assert_eq!(ret, op);
            (rd::<u64>(op), rd::<u64>(op + 0x08), rd::<u8>(op + 0x10))
        };

        let o = run(f_o);
        let n = run(f_n);
        crate::diff_eq!(o, n, "142478780 is_abs={} mode={}", is_abs, mode);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_14247dea0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u64, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut H_VTBL0: [usize; 4] = [0; 4];
    static mut H_VTBL1: [usize; 4] = [0; 4];
    static mut H_OBJ0: [usize; 1] = [0; 1];
    static mut H_OBJ1: [usize; 1] = [0; 1];
    static mut PKT_CAPTURE: [u64; 11] = [0; 11];

    unsafe extern "win64" fn mock_match0(_this: usize, _sv: usize, _u50: u32, _flag: u8) -> u8 {
        0
    }
    unsafe extern "win64" fn mock_match1(_this: usize, _sv: usize, u50: u32, flag: u8) -> u8 {
        if u50 == 0x2020 && (flag == 1 || flag == 5) {
            1
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_exec1(_this: usize, _sv: usize, pkt: usize) {
        let mut cap = rd::<[u64; 11]>(pkt);
        cap[4] = 0; // clear pointer field at +0x20 (param_1 + 0x2c8)
        PKT_CAPTURE = cap;
    }

    let run = |f: F, b40: u8| -> ([u32; 2], u32, u64, [u64; 11]) {
        PKT_CAPTURE = [0; 11];
        H_VTBL0 = [0, mock_match0 as *const () as usize, 0, 0];
        H_VTBL1 = [
            0,
            mock_match1 as *const () as usize,
            mock_exec1 as *const () as usize,
            0,
        ];
        H_OBJ0 = [core::ptr::addr_of!(H_VTBL0) as usize];
        H_OBJ1 = [core::ptr::addr_of!(H_VTBL1) as usize];

        let handlers: [usize; 2] = [
            core::ptr::addr_of_mut!(H_OBJ0) as usize,
            core::ptr::addr_of_mut!(H_OBJ1) as usize,
        ];

        let mut p1 = [0u8; 0x520];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x2c8, 10);
        wr::<usize>(p1p + 0x4e8, handlers.as_ptr() as usize);
        wr::<u32>(p1p + 0x4f0, 2);
        wr::<u32>(p1p + 0x508, 100);

        let mut p5 = [0u8; 0x48];
        let p5p = p5.as_mut_ptr() as usize;
        wr::<[u64; 2]>(p5p, [0x1111, 0x2222]);
        wr::<[u64; 2]>(p5p + 0x10, [0x3333, 0x4444]);
        wr::<u32>(p5p + 0x20, 0x2020);
        wr::<u32>(p5p + 0x24, 0x2424);
        wr::<u64>(p5p + 0x28, 0x2828_2828);
        wr::<u64>(p5p + 0x38, 0x3838_3838);
        wr::<u8>(p5p + 0x40, b40);

        let p3: [u64; 2] = [0xaaaa, 0xbbbb];
        let mut p2 = [0u32; 2];
        let ret = f(
            p1p,
            p2.as_mut_ptr() as usize,
            p3.as_ptr() as usize,
            0x9999_8888,
            p5p,
        );
        assert_eq!(ret, p2.as_ptr() as usize);
        (
            p2,
            rd::<u32>(p1p + 0x508),
            rd::<u64>(p1p + 0x2c8),
            PKT_CAPTURE,
        )
    };

    for b40 in [0u8, 1u8] {
        let o = run(f_o, b40);
        let n = run(f_n, b40);
        crate::diff_eq!(o, n, "14247dea0 b40={}", b40);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1424b06b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_LOG: u64 = 0;

    unsafe extern "win64" fn mock_8950() -> usize {
        TRACE_LOG = TRACE_LOG.wrapping_add(0x895);
        0x1111_0000
    }
    unsafe extern "win64" fn mock_8a30() -> usize {
        TRACE_LOG = TRACE_LOG.wrapping_add(0x8a3);
        0
    }
    unsafe extern "win64" fn mock_9d90(rcx: usize) {
        TRACE_LOG = TRACE_LOG.wrapping_add(rcx as u64);
    }
    unsafe extern "win64" fn mock_5550(_rcx: usize) {
        TRACE_LOG = TRACE_LOG.wrapping_add(0x555);
    }
    unsafe extern "win64" fn mock_03d0(rcx: usize) {
        TRACE_LOG = TRACE_LOG.wrapping_add(rcx as u64);
    }
    unsafe extern "win64" fn mock_cb(this: usize) {
        let tag = rd::<u64>(this + 0x10);
        TRACE_LOG = TRACE_LOG.wrapping_add(tag);
    }

    let _m1 = crate::difftest::Mock::new(0x1_413d_8950, mock_8950 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_413d_8a30, mock_8a30 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401f_9d90, mock_9d90 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4115_5550, mock_5550 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_424b_03d0, mock_03d0 as *const ())?;

    let lock_ptr = exe::ptr::<u64>(0x1_4387_1780) as usize;
    let tid_ptr = exe::ptr::<u32>(0x1_4387_1788) as usize;
    let cnt_ptr = exe::ptr::<u32>(0x1_4387_178c) as usize;
    let g_3e0 = exe::ptr::<usize>(0x1_43a0_83e0) as usize;
    let g_3f0 = exe::ptr::<usize>(0x1_43a0_83f0) as usize;
    let g_3d0 = exe::ptr::<usize>(0x1_43a0_83d0) as usize;
    let g_3d8 = exe::ptr::<usize>(0x1_43a0_83d8) as usize;
    let g_3e8 = exe::ptr::<usize>(0x1_43a0_83e8) as usize;
    let g_dirty = exe::ptr::<u8>(0x1_4397_d8df) as usize;

    let saved = (
        rd::<u64>(lock_ptr),
        rd::<u32>(tid_ptr),
        rd::<u32>(cnt_ptr),
        rd::<usize>(g_3e0),
        rd::<usize>(g_3f0),
        rd::<usize>(g_3d0),
        rd::<usize>(g_3d8),
        rd::<usize>(g_3e8),
        rd::<u8>(g_dirty),
    );

    let run = |f: F| -> (u8, u32, u64) {
        TRACE_LOG = 0;

        let mut sentinel = [0usize; 4];
        let sp = sentinel.as_mut_ptr() as usize;
        wr::<usize>(sp, sp); // empty intrusive list (rbx == sentinel)

        let mut obj_3f0 = [0usize; 2];
        wr::<usize>(obj_3f0.as_mut_ptr() as usize + 0x08, sp);

        let cb_vtbl: [usize; 1] = [mock_cb as *const () as usize];
        let cb_node: [u64; 3] = [cb_vtbl.as_ptr() as u64, 0, 0x777];

        wr::<u64>(lock_ptr, 0);
        wr::<u32>(tid_ptr, 0xffff_ffff);
        wr::<u32>(cnt_ptr, 0);
        wr::<usize>(g_3e0, 0x2222);
        wr::<usize>(g_3f0, obj_3f0.as_mut_ptr() as usize);
        wr::<usize>(g_3d0, 0x3333);
        wr::<usize>(g_3d8, 0);
        wr::<usize>(g_3e8, cb_node.as_ptr() as usize);
        wr::<u8>(g_dirty, 1);

        f();
        (rd::<u8>(g_dirty), rd::<u32>(cnt_ptr), TRACE_LOG)
    };

    let o = run(f_o);
    let n = run(f_n);

    wr::<u64>(lock_ptr, saved.0);
    wr::<u32>(tid_ptr, saved.1);
    wr::<u32>(cnt_ptr, saved.2);
    wr::<usize>(g_3e0, saved.3);
    wr::<usize>(g_3f0, saved.4);
    wr::<usize>(g_3d0, saved.5);
    wr::<usize>(g_3d8, saved.6);
    wr::<usize>(g_3e8, saved.7);
    wr::<u8>(g_dirty, saved.8);

    crate::diff_eq!(o, n, "1424b06b0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1424b3cb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FALLBACK_ARG: u64 = 0;

    unsafe extern "win64" fn mock_9040(lock_p: usize, _rdx: u64, inc: u64) {
        wr::<u64>(lock_p, rd::<u64>(lock_p).wrapping_add(inc));
    }
    unsafe extern "win64" fn mock_69a0() -> f64 {
        let r = 42.25f64;
        crate::simd_ops::set_xmm_ret(r.to_bits() as u128);
        r
    }
    unsafe extern "win64" fn mock_fallback(_this: usize, arg_ptr: usize) {
        FALLBACK_ARG = rd::<u64>(arg_ptr);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4137_9040, mock_9040 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_69a0, mock_69a0 as *const ())?;

    let g_ce14 = exe::ptr::<u16>(0x1_43c8_ce14) as usize;
    let s_ce14 = rd::<u16>(g_ce14);

    let run = |f: F, over_limit: bool| -> (u16, u64, u32, u64, u64, u64) {
        FALLBACK_ARG = 0;
        wr::<u16>(g_ce14, 7);

        let fb_vtbl: [usize; 3] = [0, 0, mock_fallback as *const () as usize];
        let fb_obj: [usize; 1] = [fb_vtbl.as_ptr() as usize];

        let mut p1 = [0u8; 0xb0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p, 0x1111_2222);
        wr::<u64>(p1p + 0x08, 5);
        wr::<u64>(p1p + 0x10, 0);
        wr::<usize>(p1p + 0x90, fb_obj.as_ptr() as usize);
        wr::<u64>(p1p + 0x98, 0x100);
        wr::<u32>(p1p + 0xa0, 10);

        let mut p2 = [0u8; 0x40];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u64>(p2p, 0x5000);
        wr::<u64>(p2p + 0x18, if over_limit { 0x5200 } else { 0x5080 });

        f(p1p, p2p);

        let head_tag = (rd::<u64>(p1p) >> 48) as u16;
        (
            head_tag,
            rd::<u64>(p1p + 0x08),
            rd::<u32>(p1p + 0xa0),
            rd::<u64>(p2p + 0x28),
            rd::<u64>(p2p + 0x30),
            FALLBACK_ARG,
        )
    };

    for over in [false, true] {
        let o = run(f_o, over);
        let n = run(f_n, over);
        wr::<u16>(g_ce14, s_ce14);
        crate::diff_eq!(o, n, "1424b3cb0 over={}", over);
    }
    Ok(2)
}

// REGISTRY:
// Replacement::verified_throws(0x1_423d_8720, "unknown_1423d8720", rend::cauldron_hot_v7::unknown_1423d8720 as *const (), Throws::Discard, rend::cauldron_hot_v7::diff_unknown_1423d8720),
// Replacement::verified(0x1_4242_df50, "r_14242df50", rend::cauldron_hot_v7::r_14242df50 as *const (), rend::cauldron_hot_v7::diff_r_14242df50),
// Replacement::verified_throws(0x1_4243_de30, "r_14243de30", rend::cauldron_hot_v7::r_14243de30 as *const (), Throws::Caught, rend::cauldron_hot_v7::diff_r_14243de30),
// Replacement::verified_throws(0x1_4247_7320, "r_142477320", rend::cauldron_hot_v7::r_142477320 as *const (), Throws::Caught, rend::cauldron_hot_v7::diff_r_142477320),
// Replacement::verified_throws(0x1_4247_8340, "r_142478340", rend::cauldron_hot_v7::r_142478340 as *const (), Throws::Caught, rend::cauldron_hot_v7::diff_r_142478340),
// Replacement::verified_throws(0x1_4247_85d0, "r_1424785d0", rend::cauldron_hot_v7::r_1424785d0 as *const (), Throws::Caught, rend::cauldron_hot_v7::diff_r_1424785d0),
// Replacement::verified_throws(0x1_4247_8780, "r_142478780", rend::cauldron_hot_v7::r_142478780 as *const (), Throws::Discard, rend::cauldron_hot_v7::diff_r_142478780),
// Replacement::verified(0x1_4247_dea0, "unknown_14247dea0", rend::cauldron_hot_v7::unknown_14247dea0 as *const (), rend::cauldron_hot_v7::diff_unknown_14247dea0),
// Replacement::verified_throws(0x1_424b_06b0, "unknown_1424b06b0", rend::cauldron_hot_v7::unknown_1424b06b0 as *const (), Throws::Discard, rend::cauldron_hot_v7::diff_unknown_1424b06b0),
// Replacement::verified_throws(0x1_424b_3cb0, "unknown_1424b3cb0", rend::cauldron_hot_v7::unknown_1424b3cb0 as *const (), Throws::Discard, rend::cauldron_hot_v7::diff_unknown_1424b3cb0),
