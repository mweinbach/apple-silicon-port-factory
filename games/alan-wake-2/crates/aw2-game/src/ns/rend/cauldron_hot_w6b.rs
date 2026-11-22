//! Decompiled hot functions shard W6B (`0x142336b40..0x14248b0c0`).

use crate::exe::{self, rd, wr};

/// `rend_142336b40`
/// Original: 0x142336b40 (1037 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_142336b40(param_1: usize) {
    let word_cnt = (rd::<u32>(param_1 + 0x08) >> 6) as usize;
    if word_cnt != 0 {
        let mut bit_base: usize = 0;
        let mut word_ptr = param_1.wrapping_add(0x78);
        let tbl_base = exe::ptr::<u64>(0x1_4398_00e0) as usize;
        for _ in 0..word_cnt {
            let mut bits = crate::simd_ops::atomic_xchg_u64(word_ptr as u64, 0);
            while bits != 0 {
                let bit = bits.trailing_zeros() as usize;
                let idx = bit_base.wrapping_add(bit);
                let val = rd::<u64>(tbl_base.wrapping_add(idx * 8));
                exe::call_native!(
                    0x1_4233_3360,
                    crate::ns::rend::cauldron_hot_z9b::rend_142333360,
                    (param_1, val as usize) -> ()
                );
                bits &= !(1u64 << bit);
            }
            word_ptr = word_ptr.wrapping_add(8);
            bit_base = bit_base.wrapping_add(0x40);
        }
    }

    let old_68 = rd::<usize>(param_1 + 0x68);
    wr::<usize>(param_1 + 0x68, 0);
    if old_68 != 0 {
        let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff18) as usize);
        let lock = mgr.wrapping_add(0x08);
        let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
        if rd::<u32>(lock + 0x08) != tid {
            exe::call_iat!(0x1_4264_55a8, (lock));
            wr::<u32>(lock + 0x08, tid);
        }
        wr::<u32>(lock + 0x0c, rd::<u32>(lock + 0x0c).wrapping_add(1));

        let len = rd::<u32>(mgr + 0x20) as u64;
        let arr = rd::<usize>(mgr + 0x18);
        if len != 0 {
            let mut idx_i32: i32 = 0;
            loop {
                let off = (idx_i32 as i64 as usize).wrapping_mul(0x68);
                let entry = arr.wrapping_add(off);
                let elem = rd::<usize>(entry);
                if elem == old_68 {
                    wr::<u8>(entry + 0x38, 0);
                    let g_180 = rd::<u32>(exe::ptr::<u32>(0x1_43a0_0180) as usize);
                    wr::<u32>(entry + 0x64, g_180);
                    if rd::<u8>(entry + 0x39) != 0 {
                        if elem != 0 {
                            exe::call_va!(0x1_4204_f860, (elem));
                            exe::call_va!(0x1_4244_2de0, (elem));
                        }
                        wr::<usize>(entry, 0);
                        let arr2 = rd::<usize>(mgr + 0x18);
                        let len2 = rd::<u32>(mgr + 0x20) as usize;
                        let end_ptr = arr2.wrapping_add(len2.wrapping_mul(0x68));
                        let dst = arr2.wrapping_add(off);
                        wr::<u64>(dst, rd::<u64>(end_ptr - 0x68));
                        wr::<[u8; 32]>(dst + 0x08, rd::<[u8; 32]>(end_ptr - 0x60));
                        wr::<[u8; 16]>(dst + 0x28, rd::<[u8; 16]>(end_ptr - 0x40));
                        wr::<u8>(dst + 0x38, rd::<u8>(end_ptr - 0x30));
                        wr::<u8>(dst + 0x39, rd::<u8>(end_ptr - 0x2f));
                        exe::call_va!(0x1_4240_aa00, (dst + 0x3c, end_ptr - 0x2c));
                        wr::<u32>(dst + 0x64, rd::<u32>(end_ptr - 0x04));
                        let len3 = rd::<u32>(mgr + 0x20) as usize;
                        let arr3 = rd::<usize>(mgr + 0x18);
                        let last_str = arr3.wrapping_add(len3.wrapping_mul(0x68)).wrapping_sub(0x2c);
                        exe::call_va!(0x1_4240_ab10, (last_str));
                        wr::<u32>(mgr + 0x20, rd::<u32>(mgr + 0x20).wrapping_sub(1));
                    }
                    break;
                }
                idx_i32 = idx_i32.wrapping_add(1);
                if (idx_i32 as i64 as u64) >= len {
                    break;
                }
            }
        }

        let new_cnt = rd::<u32>(lock + 0x0c).wrapping_sub(1);
        wr::<u32>(lock + 0x0c, new_cnt);
        if new_cnt == 0 {
            let _ = crate::simd_ops::atomic_xchg_u32((lock + 0x08) as u64, u32::MAX);
            exe::call_iat!(0x1_4264_5598, (lock));
        }
    }

    let raw_cnt = (rd::<u32>(param_1 + 0x08) as u64).max(0x400);
    let aligned_cnt = raw_cnt.wrapping_add(0xff) & !0xffu64;
    let size_50 = aligned_cnt.wrapping_mul(8);
    let size_60 = aligned_cnt.wrapping_mul(0x38);

    let p50 = rd::<usize>(param_1 + 0x50);
    if p50 == 0 || rd::<u64>(p50 + 0x10) < size_50 {
        let mut tmp_50: usize = 0;
        let ret_ptr = exe::call_va!(
            0x1_411d_b210,
            ((&mut tmp_50 as *mut usize) as usize) -> usize
        );
        let new_50 = rd::<usize>(ret_ptr);
        wr::<usize>(ret_ptr, 0);
        let old_50 = rd::<usize>(param_1 + 0x50);
        wr::<usize>(param_1 + 0x50, new_50);
        if old_50 != 0 {
            exe::call_va!(0x1_4209_d670, (old_50));
            exe::call_va!(0x1_411e_7fc0, (old_50 + 0x58));
            exe::call_va!(0x1_4244_2de0, (old_50));
        }
        if tmp_50 != 0 {
            exe::call_va!(0x1_4209_d670, (tmp_50));
            exe::call_va!(0x1_411e_7fc0, (tmp_50 + 0x58));
            exe::call_va!(0x1_4244_2de0, (tmp_50));
        }
        let cur_50 = rd::<usize>(param_1 + 0x50);
        exe::call_va!(0x1_4209_d670, (cur_50));
        exe::call_va!(
            0x1_4207_5af0,
            (size_50, 8u64, 0u8, 1u32, 0u32, 0u64, cur_50)
        );
        wr::<u32>(cur_50 + 0x50, if size_50 > 0x40_0000 { 3 } else { 2 });

        let mut tmp_60: usize = 0;
        let ret_ptr2 = exe::call_va!(
            0x1_411d_b210,
            ((&mut tmp_60 as *mut usize) as usize) -> usize
        );
        let new_60 = rd::<usize>(ret_ptr2);
        wr::<usize>(ret_ptr2, 0);
        let old_60 = rd::<usize>(param_1 + 0x60);
        wr::<usize>(param_1 + 0x60, new_60);
        if old_60 != 0 {
            exe::call_va!(0x1_4209_d670, (old_60));
            exe::call_va!(0x1_411e_7fc0, (old_60 + 0x58));
            exe::call_va!(0x1_4244_2de0, (old_60));
        }
        if tmp_60 != 0 {
            exe::call_va!(0x1_4209_d670, (tmp_60));
            exe::call_va!(0x1_411e_7fc0, (tmp_60 + 0x58));
            exe::call_va!(0x1_4244_2de0, (tmp_60));
        }
        let cur_60 = rd::<usize>(param_1 + 0x60);
        exe::call_va!(0x1_4209_d670, (cur_60));
        exe::call_va!(
            0x1_4207_5af0,
            (size_60, 0x38u64, 0u8, 1u32, 0u32, 0u64, cur_60)
        );
        wr::<u32>(cur_60 + 0x50, if size_60 > 0x40_0000 { 3 } else { 2 });
    }

    let p58 = rd::<usize>(param_1 + 0x58);
    if p58 == 0 || rd::<u64>(p58 + 0x10) < (rd::<u32>(param_1 + 0x2c) as u64) {
        let mut tmp_58: usize = 0;
        let ret_ptr3 = exe::call_va!(
            0x1_411d_b210,
            ((&mut tmp_58 as *mut usize) as usize) -> usize
        );
        let new_58 = rd::<usize>(ret_ptr3);
        wr::<usize>(ret_ptr3, 0);
        let old_58 = rd::<usize>(param_1 + 0x58);
        wr::<usize>(param_1 + 0x58, new_58);
        if old_58 != 0 {
            exe::call_va!(0x1_4209_d670, (old_58));
            exe::call_va!(0x1_411e_7fc0, (old_58 + 0x58));
            exe::call_va!(0x1_4244_2de0, (old_58));
        }
        if tmp_58 != 0 {
            exe::call_va!(0x1_4209_d670, (tmp_58));
            exe::call_va!(0x1_411e_7fc0, (tmp_58 + 0x58));
            exe::call_va!(0x1_4244_2de0, (tmp_58));
        }
        let cur_58 = rd::<usize>(param_1 + 0x58);
        let size_58 = rd::<u32>(param_1 + 0x2c) as u64;
        exe::call_va!(0x1_4209_d670, (cur_58));
        exe::call_va!(
            0x1_4207_5af0,
            (size_58, 0u64, 0u8, 1u32, 0u32, 0u64, cur_58)
        );
        wr::<u32>(cur_58 + 0x50, if size_58 > 0x40_0000 { 3 } else { 2 });
    }

    let g_918 = rd::<u64>(exe::ptr::<u64>(0x1_43a3_5918) as usize);
    wr::<u64>(param_1 + 0x70, g_918);
}

/// `rend_14233cce0`
/// Original: 0x14233cce0 (942 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_14233cce0(param_1: usize) -> u64 {
    if rd::<u64>(exe::ptr::<u64>(0x1_43a3_6ed8) as usize) == 0 {
        let msg = exe::ptr::<u8>(0x1_433f_e688) as usize;
        exe::call_va!(0x1_4211_6b30, (param_1, msg));
        return 0;
    }
    if rd::<u64>(exe::ptr::<u64>(0x1_43a3_71a8) as usize) == 0 {
        let msg = exe::ptr::<u8>(0x1_433f_e5d0) as usize;
        exe::call_va!(0x1_4211_6b30, (param_1, msg));
        return 0;
    }

    let nil_obj = exe::ptr::<u8>(0x1_4298_68d0) as usize;
    let arg1 = rd::<usize>(param_1 + 0x10);
    if arg1 >= rd::<usize>(param_1 + 0x08) || arg1 == nil_obj || rd::<u32>(arg1 + 0x10) != 3 {
        exe::call_va!(0x1_4211_58a0, (param_1, 1u32, 3u32));
        return 0;
    }

    let mut tmp_num = [0u64; 6];
    let num_ptr = if rd::<u32>(arg1 + 0x10) == 3 {
        arg1
    } else {
        exe::call_va!(0x1_420e_cd00, (arg1, tmp_num.as_mut_ptr() as usize) -> usize)
    };
    let tex_idx = if num_ptr != 0 {
        crate::simd_ops::cvt_f64_i64(rd::<f64>(num_ptr)) as u32
    } else {
        0
    };

    let max_idx = rd::<u32>(exe::ptr::<u32>(0x1_43a2_b40c) as usize);
    if tex_idx >= max_idx {
        let msg = exe::ptr::<u8>(0x1_433f_e7c8) as usize;
        exe::call_va!(0x1_4211_6b30, (param_1, msg, tex_idx, max_idx));
        return 0;
    }

    let arg2 = rd::<usize>(param_1 + 0x10).wrapping_add(0x18);
    if arg2 >= rd::<usize>(param_1 + 0x08) || arg2 == nil_obj {
        let msg = exe::ptr::<u8>(0x1_433f_e800) as usize;
        exe::call_va!(0x1_4211_6b30, (param_1, msg));
        return 0;
    }

    let ty2 = rd::<u32>(arg2 + 0x10);
    if ty2 != 5 {
        if ty2 != 6 {
            let msg = exe::ptr::<u8>(0x1_433f_e800) as usize;
            exe::call_va!(0x1_4211_6b30, (param_1, msg));
            return 0;
        }
        let mut str_buf = [0u64; 6];
        let sb_ptr = str_buf.as_mut_ptr() as usize;
        wr::<u64>(sb_ptr, 0x0500_0005);
        wr::<u8>(sb_ptr + 0x08, 0);
        let s_guid = exe::ptr::<u8>(0x1_433f_e5f8) as usize;
        exe::call_va!(0x1_403f_4a80, (param_1, 2u32, s_guid, sb_ptr));

        let hdr = rd::<u32>(sb_ptr);
        let b3 = hdr >> 24;
        let cap_bits = if b3 < 0x80 {
            b3.wrapping_mul(8)
        } else {
            (b3 << 8).wrapping_sub(0x7c00)
        };
        let len_bits = (hdr & 0x00ff_ffff) << 3;
        let data_ptr = if len_bits > cap_bits {
            rd::<usize>(sb_ptr + 0x08)
        } else {
            sb_ptr + 0x08
        };
        let str_len = rd::<u32>(sb_ptr + 0x04) as u64;
        let mut view: [u64; 2] = [data_ptr as u64, str_len];
        let key_hash = exe::call_va!(0x1_424b_2600, (view.as_mut_ptr() as usize) -> u64);

        let tbl_9e0 = exe::ptr::<u64>(0x1_43bb_d9e0) as usize;
        let mut found_idx: i32 = -1;
        for i in 0..3usize {
            if rd::<u64>(tbl_9e0.wrapping_add(i * 0x10)) == key_hash {
                found_idx = i as i32;
                break;
            }
        }

        if found_idx >= 0 && (found_idx as usize) < 3 {
            let slot_idx = found_idx as usize;
            let sys_tex_ptr = rd::<usize>(tbl_9e0.wrapping_add(slot_idx * 0x10 + 0x08));
            if sys_tex_ptr == 0 {
                let msg = exe::ptr::<u8>(0x1_433f_e5a8) as usize;
                exe::call_va!(0x1_4211_6b30, (param_1, msg, found_idx as u32));
                return 0;
            }
            exe::call_va!(0x1_4240_ab10, (sb_ptr));
            let inner = rd::<usize>(sys_tex_ptr);
            let val_1a0 = rd::<u64>(inner + 0x1a0);
            let dst_arr = rd::<usize>(exe::ptr::<usize>(0x1_43a3_71a8) as usize);
            wr::<u64>(dst_arr.wrapping_add((tex_idx as usize) * 8), val_1a0);
            return 0;
        }

        let max_v = exe::call_va!(0x1_4020_bf70, () -> u64);
        let msg = exe::ptr::<u8>(0x1_433f_e600) as usize;
        exe::call_va!(0x1_4211_6b30, (param_1, msg, found_idx as u32, max_v));
        return 0;
    }

    let mut id_slot: u32 = 0;
    let mut resolved = false;
    if rd::<u32>(arg2 + 0x10) == 5 {
        let tstr = rd::<usize>(arg2);
        let mut atom = rd::<i16>(tstr + 0x04);
        if atom == -0x8000i16 {
            let g_state = rd::<usize>(param_1 + 0x18);
            let cb = rd::<usize>(g_state + 0x1068);
            atom = if cb != 0 {
                exe::call_va!(cb, (tstr.wrapping_add(0x18), rd::<u32>(tstr + 0x14)) -> u16) as i16
            } else {
                -1i16
            };
            wr::<i16>(tstr + 0x04, atom);
        }
        let ecx = (atom as i32 as u32)
            .wrapping_sub(rd::<u32>(exe::ptr::<u32>(0x1_43bb_d9d0) as usize));
        if tstr.wrapping_add(0x18) != 0
            && ecx < rd::<u32>(exe::ptr::<u32>(0x1_43bb_d9c8) as usize)
        {
            let tbl_9c0 = rd::<usize>(exe::ptr::<usize>(0x1_43bb_d9c0) as usize);
            let mapped = rd::<i32>(tbl_9c0.wrapping_add(((ecx as i32 as isize) as usize) * 4));
            if (mapped as u32) != rd::<u32>(exe::ptr::<u32>(0x1_43bb_d9d4) as usize) {
                let tbl_5b0 = exe::ptr::<u32>(0x1_43c4_a5b0) as usize;
                id_slot = rd::<u32>(tbl_5b0.wrapping_add(((mapped as isize) as usize) * 4));
                resolved = true;
            }
        }
    }

    if !resolved {
        let cand = rd::<usize>(param_1 + 0x10).wrapping_add(0x18);
        let slot = if cand < rd::<usize>(param_1 + 0x08) {
            cand
        } else {
            nil_obj
        };
        let slot_ty = rd::<u32>(slot + 0x10);
        let str_len_i32: i32 = match slot_ty {
            5 => rd::<i32>(rd::<usize>(slot) + 0x14),
            6 => exe::call_va!(0x1_420f_cf40, (rd::<usize>(slot)) -> i32),
            8 => rd::<i32>(rd::<usize>(slot) + 0x04),
            _ => 0,
        };
        let str_len = (str_len_i32 as i64) as u64;
        let check_slot = rd::<usize>(param_1 + 0x10).wrapping_add(0x18);
        if check_slot >= rd::<usize>(param_1 + 0x08)
            || check_slot == nil_obj
            || rd::<u32>(check_slot + 0x10) != 5
        {
            exe::call_va!(0x1_4211_58a0, (param_1, 2u32, 5u32));
            return 0;
        }
        let str_ptr = rd::<usize>(check_slot).wrapping_add(0x18);
        let mut span: [u64; 2] = [str_ptr as u64, str_len];
        exe::call_va!(
            0x1_4220_ea70,
            ((&mut id_slot as *mut u32) as usize, span.as_mut_ptr() as usize)
        );
    }

    let val = exe::call_va!(
        0x1_4233_8090,
        (0usize, (&id_slot as *const u32) as usize) -> u64
    );
    let dst_arr = rd::<usize>(exe::ptr::<usize>(0x1_43a3_71a8) as usize);
    wr::<u64>(dst_arr.wrapping_add((tex_idx as usize) * 8), val);
    0
}

/// `rend_14233ec10`
/// Original: 0x14233ec10 (252 bytes, Throws::Discard)
pub unsafe extern "win64" fn rend_14233ec10(param_1: usize) -> u64 {
    let mut u_key = exe::call_va!(0x1_4233_f070, (param_1) -> u64);
    let lo_i32 = (u_key as u32) as i32;
    let hi_u32 = (u_key >> 32) as u32;
    let g_fa8 = rd::<usize>(exe::ptr::<usize>(0x1_43a3_6fa8) as usize);
    let bucket = rd::<usize>(g_fa8).wrapping_add(((lo_i32 as isize) as usize).wrapping_mul(0x20));

    let cnt = rd::<u32>(bucket + 0x08) as usize;
    let keys = rd::<usize>(bucket);
    let mut found_entry: usize = 0;
    for i in 0..cnt {
        if rd::<u32>(keys.wrapping_add(i * 4)) == hi_u32 {
            found_entry = rd::<usize>(bucket + 0x10).wrapping_add(i.wrapping_mul(0x18));
            break;
        }
    }

    if found_entry == 0 {
        let key_hi_ptr = (&mut u_key as *mut u64 as usize).wrapping_add(4);
        let name_ptr = exe::call_va!(0x1_411a_4fb0, (key_hi_ptr) -> u64);
        let msg = exe::ptr::<u8>(0x1_433f_e540) as usize;
        exe::call_va!(0x1_4211_6b30, (param_1, msg, name_ptr));
        return 0;
    }

    exe::call_va!(
        0x1_4233_eec0,
        (param_1, (&mut u_key as *mut u64) as usize, 0xcu32)
    );

    let cnt2 = rd::<u32>(bucket + 0x08) as usize;
    let keys2 = rd::<usize>(bucket);
    let want_hi = (u_key >> 32) as u32;
    let mut entry2: usize = 0;
    for i in 0..cnt2 {
        if rd::<u32>(keys2.wrapping_add(i * 4)) == want_hi {
            entry2 = rd::<usize>(bucket + 0x10).wrapping_add(i.wrapping_mul(0x18));
            break;
        }
    }

    let top = rd::<usize>(param_1 + 0x08);
    let b_val = if rd::<u8>(entry2 + 0x08) != 0 { 1u32 } else { 0u32 };
    wr::<u32>(top, b_val);
    wr::<u32>(top + 0x10, 1);
    wr::<usize>(param_1 + 0x08, top.wrapping_add(0x18));
    1
}

/// `rend_14233eec0`
/// Original: 0x14233eec0 (188 bytes, Throws::Discard)
pub unsafe extern "win64" fn rend_14233eec0(param_1: usize, param_2: usize, param_3: u32) {
    let g_fa8 = rd::<usize>(exe::ptr::<usize>(0x1_43a3_6fa8) as usize);
    let lo_i32 = rd::<i32>(param_2);
    let key_hi_ptr = param_2.wrapping_add(4);
    let bucket = rd::<usize>(g_fa8).wrapping_add(((lo_i32 as isize) as usize).wrapping_mul(0x20));
    let want_hi = rd::<u32>(key_hi_ptr);
    let cnt = rd::<u32>(bucket + 0x08) as usize;
    let keys = rd::<usize>(bucket);

    for i in 0..cnt {
        if rd::<u32>(keys.wrapping_add(i * 4)) == want_hi {
            let entry = rd::<usize>(bucket + 0x10).wrapping_add(i.wrapping_mul(0x18));
            if entry != 0 {
                let found_ty = rd::<u32>(entry);
                if found_ty != 0x1d {
                    if found_ty != param_3 {
                        let name_ptr = exe::call_va!(0x1_411a_4fb0, (key_hi_ptr) -> u64);
                        let found_str = exe::call_va!(0x1_4233_ef90, (found_ty) -> u64);
                        let expected_str = exe::call_va!(0x1_4233_ef90, (param_3) -> u64);
                        let msg = exe::ptr::<u8>(0x1_433f_e4d8) as usize;
                        exe::call_va!(
                            0x1_4211_6b30,
                            (param_1, msg, name_ptr, expected_str, found_str)
                        );
                    }
                    return;
                }
            }
            break;
        }
    }

    let name_ptr = exe::call_va!(0x1_411a_4fb0, (key_hi_ptr) -> u64);
    let msg = exe::ptr::<u8>(0x1_433f_e540) as usize;
    exe::call_va!(0x1_4211_6b30, (param_1, msg, name_ptr));
}

/// `rend_14233f070`
/// Original: 0x14233f070 (483 bytes, Throws::Discard)
pub unsafe extern "win64" fn rend_14233f070(param_1: usize) -> u64 {
    let nil_obj = exe::ptr::<u8>(0x1_4298_68d0) as usize;
    let base0 = rd::<usize>(param_1 + 0x10);
    let top0 = rd::<usize>(param_1 + 0x08);
    let first = if base0 < top0 { base0 } else { nil_obj };

    let mut scratch1 = [0u64; 4];
    let is_num = rd::<u32>(first + 0x10) == 3
        || exe::call_va!(0x1_420e_cd00, (first, scratch1.as_mut_ptr() as usize) -> usize) != 0;

    let (lo_u32, str_ptr, str_len): (u32, usize, u64) = if is_num {
        let top1 = rd::<usize>(param_1 + 0x08);
        let b1 = rd::<usize>(param_1 + 0x10);
        let mut p1 = if b1 < top1 { b1 } else { nil_obj };
        let top_after1 = if rd::<u32>(p1 + 0x10) == 3 {
            top1
        } else {
            p1 = exe::call_va!(0x1_420e_cd00, (p1, scratch1.as_mut_ptr() as usize) -> usize);
            if p1 == 0 {
                exe::call_va!(0x1_4211_58a0, (param_1, 1u32, 3u32));
                return 0;
            }
            rd::<usize>(param_1 + 0x08)
        };
        let n1 = crate::simd_ops::cvt_f64_i32(rd::<f64>(p1));

        let b2 = rd::<usize>(param_1 + 0x10).wrapping_add(0x18);
        let mut p2 = if b2 < top_after1 { b2 } else { nil_obj };
        let mut scratch2 = [0u64; 4];
        let top_after2 = if rd::<u32>(p2 + 0x10) == 3 {
            top_after1
        } else {
            p2 = exe::call_va!(0x1_420e_cd00, (p2, scratch2.as_mut_ptr() as usize) -> usize);
            if p2 == 0 {
                exe::call_va!(0x1_4211_58a0, (param_1, 2u32, 3u32));
                return 0;
            }
            rd::<usize>(param_1 + 0x08)
        };
        let n2 = crate::simd_ops::cvt_f64_i32(rd::<f64>(p2));
        let combined = n2.wrapping_add(n1.wrapping_sub(1));

        let p3 = rd::<usize>(param_1 + 0x10).wrapping_add(0x30);
        if p3 >= top_after2 || p3 == nil_obj || rd::<u32>(p3 + 0x10) != 5 {
            exe::call_va!(0x1_4211_58a0, (param_1, 3u32, 5u32));
            return 0;
        }
        let s_ptr = rd::<usize>(p3).wrapping_add(0x18);
        let mut s_len: u64 = 0;
        while rd::<u8>(s_ptr.wrapping_add(s_len as usize)) != 0 {
            s_len = s_len.wrapping_add(1);
        }
        (combined, s_ptr, s_len)
    } else {
        let p1 = rd::<usize>(param_1 + 0x10);
        let top = rd::<usize>(param_1 + 0x08);
        if p1 >= top
            || p1 == nil_obj
            || (rd::<u32>(p1 + 0x10).wrapping_sub(3) & 0xffff_fffd) != 0
        {
            let msg = exe::ptr::<u8>(0x1_433f_e580) as usize;
            exe::call_va!(0x1_4211_6bc0, (param_1, 1u32, msg));
            return 0;
        }
        if rd::<u32>(p1 + 0x10) != 5 {
            exe::call_va!(0x1_4211_58a0, (param_1, 1u32, 5u32));
            return 0;
        }
        let s_ptr = rd::<usize>(p1).wrapping_add(0x18);
        let mut s_len: u64 = 0;
        while rd::<u8>(s_ptr.wrapping_add(s_len as usize)) != 0 {
            s_len = s_len.wrapping_add(1);
        }
        (0, s_ptr, s_len)
    };

    let mut span: [u64; 2] = [str_ptr as u64, str_len];
    let mut out_hash: u32 = 0;
    let ret_ptr = exe::call_va!(
        0x1_4220_ea70,
        ((&mut out_hash as *mut u32) as usize, span.as_mut_ptr() as usize) -> usize
    );
    let hi_u32 = rd::<u32>(ret_ptr);
    ((hi_u32 as u64) << 32) | (lo_u32 as u64)
}

/// `unknown_1423bdf90`
/// Original: 0x1423bdf90 (870 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1423bdf90(param_1: usize, param_2: u64, param_3: usize) {
    let mut key_slot = param_2;
    let mut lookup_out = [0u64; 3];
    let _ = exe::call_va!(
        0x1_4020_9da0,
        (
            param_1,
            lookup_out.as_mut_ptr() as usize,
            (&mut key_slot as *mut u64) as usize
        ) -> usize
    );
    let slot_ptr = lookup_out[1] as usize;
    let base_30 = rd::<usize>(param_1 + 0x30);
    let idx = rd::<usize>(slot_ptr + 0x08);
    let rdi = rd::<usize>(param_1 + 0x40).wrapping_add(idx.wrapping_mul(0xb0));
    let rbx = idx.wrapping_mul(0x50);
    let hash = exe::call_va!(0x1_424b_16a0, (param_3, 0x90u32) -> u64);

    if rd::<u64>(base_30.wrapping_add(rbx).wrapping_add(0x48)) != hash {
        wr::<u8>(param_1 + 0x50, 1);
        let mat = rd::<[u8; 64]>(param_3);
        wr::<[u8; 64]>(rdi, mat);
        let mut inv_mat = mat;
        exe::call_va!(0x1_4106_8680, (inv_mat.as_mut_ptr() as usize));
        wr::<[u8; 64]>(rdi + 0x40, inv_mat);

        let f54 = rd::<u32>(param_3 + 0x54);
        let f58 = rd::<u32>(param_3 + 0x58);
        let f5c = rd::<u32>(param_3 + 0x5c);
        wr::<u32>(rdi + 0x80, rd::<u32>(param_3 + 0x50));
        wr::<u32>(rdi + 0x84, f54);
        wr::<u32>(rdi + 0x88, f58);
        wr::<u32>(rdi + 0x8c, f5c);
        wr::<u32>(rdi + 0x90, rd::<u32>(param_3 + 0x60));
        wr::<u32>(rdi + 0x94, rd::<u32>(param_3 + 0x64));
        wr::<u32>(rdi + 0x98, rd::<u32>(param_3 + 0x68));

        let c255 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6eec) as usize);
        let c_pos1 = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
        let c_neg1 = rd::<f32>(exe::ptr::<f32>(0x1_4342_7080) as usize);

        let b_70 = (crate::simd_ops::cvt_f32_i64(c255 * rd::<f32>(param_3 + 0x70)) as u8) as u32;
        let b_6c = (crate::simd_ops::cvt_f32_i64(c255 * rd::<f32>(param_3 + 0x6c)) as u32) << 8;
        let mut rgba = b_70 | b_6c;
        let b_74 = (crate::simd_ops::cvt_f32_i64(c255 * rd::<f32>(param_3 + 0x74)) as u8) as u32;
        rgba = (rgba << 8) | b_74;
        let b_78 = (crate::simd_ops::cvt_f32_i64(c255 * rd::<f32>(param_3 + 0x78)) as u8) as u32;
        rgba = (rgba << 8) | b_78;

        wr::<u32>(rdi + 0x9c, rgba);
        wr::<u32>(rdi + 0xa0, rd::<u32>(param_3 + 0x7c));
        wr::<u32>(rdi + 0xa8, rd::<u32>(param_3 + 0x80));
        wr::<u32>(
            base_30.wrapping_add(rbx).wrapping_add(0x08),
            rd::<u32>(param_3 + 0x40),
        );

        let mut aabb: [f32; 6] = [c_neg1, c_neg1, c_neg1, c_pos1, c_pos1, c_pos1];
        exe::call_va!(0x1_424b_87a0, (aabb.as_mut_ptr() as usize, rdi));
        let aabb_dst = base_30.wrapping_add(rbx).wrapping_add(0x18);
        wr::<[f32; 6]>(aabb_dst, aabb);

        let xmm6 = (rd::<f32>(rdi + 0x88) - rd::<f32>(rdi + 0x80))
            * (rd::<f32>(rdi + 0x8c) - rd::<f32>(rdi + 0x84));

        let min_x = rd::<f32>(aabb_dst);
        let min_y = rd::<f32>(aabb_dst + 0x04);
        let min_z = rd::<f32>(aabb_dst + 0x08);
        let max_x = rd::<f32>(aabb_dst + 0x0c);
        let max_y = rd::<f32>(aabb_dst + 0x10);
        let max_z = rd::<f32>(aabb_dst + 0x14);

        let ext: [f32; 3] = if min_x > max_x && min_y > max_y && min_z > max_z {
            [0.0, 0.0, 0.0]
        } else {
            [max_x - min_x, max_y - min_y, max_z - min_z]
        };

        let dx = max_x - min_x;
        let dy = max_y - min_y;
        let dz = max_z - min_z;
        let chosen_ext = if !(dx < dy) && !(dx < dz) {
            ext[0]
        } else if !(dy < dx) && !(dy < dz) {
            ext[1]
        } else {
            ext[2]
        };

        let mut half_ext: [f32; 3] = [max_x, max_y, 0.0];
        let he_ptr = exe::call_va!(
            0x1_4024_ae10,
            (aabb_dst, half_ext.as_mut_ptr() as usize) -> usize
        );
        let hx = rd::<f32>(he_ptr);
        let hy = rd::<f32>(he_ptr + 0x04);
        let hz = rd::<f32>(he_ptr + 0x08);
        let len = ((hx * hx + hy * hy) + hz * hz).sqrt();
        let d = len + len;
        let metric = ((d * d) / chosen_ext) / xmm6;

        wr::<f32>(base_30.wrapping_add(rbx).wrapping_add(0x30), metric);
        wr::<u64>(base_30.wrapping_add(rbx).wrapping_add(0x48), hash);
    }
}

/// `unknown_1423db160`
/// Original: 0x1423db160 (101 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1423db160(param_1: usize, param_2: u64) {
    if param_2 != 0 {
        let mut rbx = param_1.wrapping_add(0x6c);
        let tbl = exe::ptr::<u64>(0x1_43bd_6048) as usize;
        let mut rem = param_2;
        while rem != 0 {
            let idx = rd::<u32>(rbx.wrapping_sub(0x6c)) as usize;
            let rcx_val = rd::<u64>(tbl.wrapping_add(idx * 8));
            exe::call_va!(0x1_4229_f840, (rcx_val, rbx.wrapping_sub(0x64)));
            exe::call_va!(0x1_4240_ab10, (rbx));
            rbx = rbx.wrapping_add(0x88);
            rem -= 1;
        }
    }
}

/// `unknown_14248b0c0`
/// Original: 0x14248b0c0 (239 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14248b0c0(param_1: usize, param_2: usize) {
    let key = rd::<u64>(param_2);
    if (key & 0x3fff) == 0 && (key & 0x3fff_ffff_ffff_c000) == 0 {
        return;
    }

    exe::call_va!(0x1_4137_8fb0, (param_1 + 0x60));
    let slot_idx = if rd::<u64>(param_1 + 0x38) == rd::<u64>(param_1 + 0x30) {
        rd::<u64>(param_1 + 0x40)
    } else {
        let mut probe = [0u64; 6];
        exe::call_va!(
            0x1_401f_a730,
            (param_1, probe.as_mut_ptr() as usize, param_2)
        );
        if probe[0] == u64::MAX {
            rd::<u64>(param_1 + 0x40)
        } else {
            probe[0]
        }
    };

    let cap = rd::<u64>(param_1 + 0x40);
    let buckets = rd::<usize>(param_1 + 0x58);
    let end_ptr = buckets.wrapping_add((cap as usize).wrapping_shl(4));
    let slot_ptr = buckets.wrapping_add((slot_idx as usize).wrapping_shl(4));
    if slot_ptr != end_ptr {
        let del_cnt = rd::<u64>(param_1 + 0x30);
        let tomb_key = rd::<u64>(param_1 + 0x28);
        if del_cnt != 0 && tomb_key == rd::<u64>(slot_ptr) {
            wr::<u64>(slot_ptr, tomb_key);
            wr::<u64>(slot_ptr + 0x08, 0);
        } else {
            wr::<u64>(slot_ptr, tomb_key);
            wr::<u64>(slot_ptr + 0x08, 0);
            wr::<u64>(param_1 + 0x30, del_cnt.wrapping_add(1));
            wr::<u8>(param_1 + 0x18, 1);
        }
    }

    let g_b24 = exe::ptr::<u32>(0x1_4384_1b24) as usize;
    let _ = crate::simd_ops::atomic_xadd_u32(g_b24 as u64, 1);
    if (rd::<u64>(param_1 + 0x60) & 0xffff_ffff_0000_0000) == 0x0000_0001_0000_0000 {
        wr::<u64>(param_1 + 0x68, 0);
    }
    let _ = crate::simd_ops::atomic_xadd_u64((param_1 + 0x60) as u64, 0xffff_ffff_0000_0000);
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_rend_142336b40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_SUM: u64 = 0;
    static mut ALLOC_IDX: usize = 0;
    static mut BUF_POOL: [[u8; 0x80]; 3] = [[0; 0x80]; 3];

    unsafe extern "win64" fn mock_3360(p1: usize, val: u64) {
        wr::<u64>(p1 + 0x30, rd::<u64>(p1 + 0x30).wrapping_add(val));
        LOG_SUM = LOG_SUM.wrapping_add(val);
    }
    unsafe extern "win64" fn mock_f860(elem: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(rd::<u64>(elem)).wrapping_add(0x111);
    }
    unsafe extern "win64" fn mock_2de0(ptr: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(rd::<u64>(ptr)).wrapping_add(0x222);
    }
    unsafe extern "win64" fn mock_aa00(dst: usize, src: usize) {
        wr::<u64>(dst, rd::<u64>(src));
        LOG_SUM = LOG_SUM.wrapping_add(0x333);
    }
    unsafe extern "win64" fn mock_ab10(rcx: usize) {
        wr::<u64>(rcx, 0);
        LOG_SUM = LOG_SUM.wrapping_add(0x444);
    }
    unsafe extern "win64" fn mock_b210(out_ptr: usize) -> usize {
        let idx = ALLOC_IDX;
        ALLOC_IDX += 1;
        let buf_ptr = core::ptr::addr_of_mut!(BUF_POOL[idx]) as usize;
        wr::<usize>(out_ptr, buf_ptr);
        out_ptr
    }
    unsafe extern "win64" fn mock_d670(buf: usize) {
        wr::<u64>(buf + 0x08, rd::<u64>(buf + 0x08).wrapping_add(1));
    }
    unsafe extern "win64" fn mock_7fc0(sub_buf: usize) {
        wr::<u64>(sub_buf, rd::<u64>(sub_buf).wrapping_add(10));
    }
    unsafe extern "win64" fn mock_5af0(
        size: u64,
        stride: u64,
        b: u8,
        flags: u32,
        arg5: u32,
        _arg6: u64,
        buf: usize,
    ) {
        wr::<u64>(buf + 0x10, size);
        wr::<u64>(buf + 0x18, stride);
        LOG_SUM = LOG_SUM
            .wrapping_add(size)
            .wrapping_add(stride)
            .wrapping_add(b as u64)
            .wrapping_add(flags as u64)
            .wrapping_add(arg5 as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4233_3360, mock_3360 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4204_f860, mock_f860 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_aa00, mock_aa00 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_411d_b210, mock_b210 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4209_d670, mock_d670 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_411e_7fc0, mock_7fc0 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4207_5af0, mock_5af0 as *const ())?;

    let g_0e0 = exe::ptr::<u64>(0x1_4398_00e0) as usize;
    let g_f18 = exe::ptr::<usize>(0x1_4397_ff18) as usize;
    let g_180 = exe::ptr::<u32>(0x1_43a0_0180) as usize;
    let g_918 = exe::ptr::<u64>(0x1_43a3_5918) as usize;

    let s_0e0_0 = rd::<u64>(g_0e0);
    let s_0e0_2 = rd::<u64>(g_0e0 + 16);
    let s_f18 = rd::<usize>(g_f18);
    let s_180 = rd::<u32>(g_180);
    let s_918 = rd::<u64>(g_918);

    let run = |f: F, remove_flag: u8| -> ([u64; 18], [u8; 0xd0], u32, [[u8; 0x80]; 3], u64) {
        LOG_SUM = 0;
        ALLOC_IDX = 0;
        BUF_POOL = [[0; 0x80]; 3];
        wr::<u64>(g_0e0, 0x1001);
        wr::<u64>(g_0e0 + 16, 0x2002);
        wr::<u32>(g_180, 77);
        wr::<u64>(g_918, 0x9999_8888);

        let target_elem: u64 = 0xdead_1234;
        let other_elem: u64 = 0xbeef_5678;

        let mut mgr_arr = [0u8; 0xd0]; // 2 entries of 0x68 bytes
        let map = mgr_arr.as_mut_ptr() as usize;
        wr::<usize>(map, (&target_elem as *const u64) as usize);
        wr::<u8>(map + 0x38, 1);
        wr::<u8>(map + 0x39, remove_flag);
        wr::<u64>(map + 0x3c, 0xaaaa);

        wr::<usize>(map + 0x68, (&other_elem as *const u64) as usize);
        wr::<u64>(map + 0x68 + 0x08, 0xbbbb);
        wr::<u8>(map + 0x68 + 0x38, 1);
        wr::<u8>(map + 0x68 + 0x39, 0);
        wr::<u64>(map + 0x68 + 0x3c, 0xcccc);
        wr::<u32>(map + 0x68 + 0x64, 42);

        let mut mgr = [0u8; 0x30];
        let mp = mgr.as_mut_ptr() as usize;
        wr::<u32>(mp + 0x10, u32::MAX);
        wr::<u32>(mp + 0x14, 0);
        wr::<usize>(mp + 0x18, map);
        wr::<u32>(mp + 0x20, 2);
        wr::<usize>(g_f18, mp);

        let mut old_buf50 = [0u8; 0x80];
        wr::<u64>(old_buf50.as_mut_ptr() as usize + 0x10, 16); // too small (< 0x2000)

        let mut p1 = [0u64; 18];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1p + 0x08, 64); // 1 word in bitset
        wr::<u32>(p1p + 0x2c, 0x500000); // > 0x400000 so flag at +0x50 becomes 3
        wr::<usize>(p1p + 0x50, old_buf50.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x68, (&target_elem as *const u64) as usize);
        wr::<u64>(p1p + 0x78, 0b0101); // bits 0 and 2

        f(p1p);

        // Normalize pointers before comparing
        p1[10] = 0; // +0x50
        p1[11] = 0; // +0x58
        p1[12] = 0; // +0x60
        wr::<usize>(map, 0);
        wr::<usize>(map + 0x68, 0);
        (p1, mgr_arr, rd::<u32>(mp + 0x20), BUF_POOL, LOG_SUM)
    };

    for remove_flag in [1u8, 0u8] {
        let o = run(f_o, remove_flag);
        let n = run(f_n, remove_flag);
        wr::<u64>(g_0e0, s_0e0_0);
        wr::<u64>(g_0e0 + 16, s_0e0_2);
        wr::<usize>(g_f18, s_f18);
        wr::<u32>(g_180, s_180);
        wr::<u64>(g_918, s_918);
        crate::diff_eq!(o, n, "142336b40 remove_flag={}", remove_flag);
    }
    Ok(2)
}

pub unsafe fn diff_rend_14233cce0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut AB10_CALLED: u32 = 0;

    unsafe extern "win64" fn mock_4a80(_l: usize, _idx: u32, _k: usize, out_str: usize) {
        wr::<u32>(out_str, (4u32 << 24) | 4);
        wr::<u32>(out_str + 0x04, 4);
        wr::<u32>(out_str + 0x08, 0x6469_7567);
    }
    unsafe extern "win64" fn mock_2600(view_ptr: usize) -> u64 {
        let p = rd::<usize>(view_ptr);
        let len = rd::<u64>(view_ptr + 0x08);
        (rd::<u32>(p) as u64) ^ (len << 32)
    }
    unsafe extern "win64" fn mock_ab10(_str_buf: usize) {
        AB10_CALLED += 1;
    }
    unsafe extern "win64" fn mock_atom_cb(str_ptr: usize, len: u32) -> i16 {
        (rd::<u8>(str_ptr) as i16).wrapping_add(len as i16)
    }
    unsafe extern "win64" fn mock_ea70(out_id: usize, span_ptr: usize) -> usize {
        let p = rd::<usize>(span_ptr);
        let len = rd::<u64>(span_ptr + 0x08) as u32;
        wr::<u32>(out_id, (rd::<u8>(p) as u32).wrapping_mul(100).wrapping_add(len));
        out_id
    }
    unsafe extern "win64" fn mock_8090(_unused: usize, id_ptr: usize) -> u64 {
        0x7777_0000_0000 | (rd::<u32>(id_ptr) as u64)
    }

    let _m1 = crate::difftest::Mock::new(0x1_403f_4a80, mock_4a80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_424b_2600, mock_2600 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4220_ea70, mock_ea70 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4233_8090, mock_8090 as *const ())?;

    let g_ed8 = exe::ptr::<u64>(0x1_43a3_6ed8) as usize;
    let g_1a8 = exe::ptr::<usize>(0x1_43a3_71a8) as usize;
    let g_40c = exe::ptr::<u32>(0x1_43a2_b40c) as usize;
    let g_9e0 = exe::ptr::<u64>(0x1_43bb_d9e0) as usize;
    let g_9c0 = exe::ptr::<usize>(0x1_43bb_d9c0) as usize;
    let g_9c8 = exe::ptr::<u32>(0x1_43bb_d9c8) as usize;
    let g_9d0 = exe::ptr::<u32>(0x1_43bb_d9d0) as usize;
    let g_9d4 = exe::ptr::<u32>(0x1_43bb_d9d4) as usize;
    let g_5b0 = exe::ptr::<u32>(0x1_43c4_a5b0) as usize;

    let saved = (
        rd::<u64>(g_ed8),
        rd::<usize>(g_1a8),
        rd::<u32>(g_40c),
        rd::<[u64; 6]>(g_9e0),
        rd::<usize>(g_9c0),
        rd::<u32>(g_9c8),
        rd::<u32>(g_9d0),
        rd::<u32>(g_9d4),
        rd::<[u32; 4]>(g_5b0),
    );

    let run = |f: F, mode: u32| -> (u64, [u64; 4], i16, u32) {
        AB10_CALLED = 0;
        let mut out_slots = [0u64; 4];
        wr::<u64>(g_ed8, 1);
        wr::<usize>(g_1a8, out_slots.as_mut_ptr() as usize);
        wr::<u32>(g_40c, 4);

        let mut sys_tex_inner = [0u8; 0x1b0];
        wr::<u64>(sys_tex_inner.as_mut_ptr() as usize + 0x1a0, 0xfeed_cafe_1234);
        let sys_tex_ptr: usize = sys_tex_inner.as_ptr() as usize;

        wr::<u64>(g_9e0 + 0x10, 0x6469_7567 ^ (4u64 << 32));
        wr::<usize>(g_9e0 + 0x18, (&sys_tex_ptr as *const usize) as usize);

        let map_9c0: [i32; 4] = [2, -1, 0, 0];
        wr::<usize>(g_9c0, map_9c0.as_ptr() as usize);
        wr::<u32>(g_9c8, 4);
        wr::<u32>(g_9d0, 70);
        wr::<u32>(g_9d4, u32::MAX);
        wr::<u32>(g_5b0 + 2 * 4, 0x4242);

        let mut tstr = [0u8; 0x28];
        let tsp = tstr.as_mut_ptr() as usize;
        // 'A' (65) + len (5) = 70
        wr::<i16>(tsp + 0x04, if mode == 1 { -0x8000 } else { 10 });
        wr::<u32>(tsp + 0x14, 5);
        wr::<u8>(tsp + 0x18, b'A');

        let mut g_state = [0u8; 0x1080];
        wr::<usize>(
            g_state.as_mut_ptr() as usize + 0x1068,
            mock_atom_cb as *const () as usize,
        );

        let mut lua_stk = [0u8; 0x48];
        let sp = lua_stk.as_mut_ptr() as usize;
        // arg 1: number 2.0 (type 3)
        wr::<f64>(sp, 2.0);
        wr::<u32>(sp + 0x10, 3);
        // arg 2: table (type 6) when mode == 0, string (type 5) when mode == 1 or 2
        wr::<usize>(sp + 0x18, tsp);
        wr::<u32>(sp + 0x28, if mode == 0 { 6 } else { 5 });

        let mut l_state = [0u64; 4];
        let lp = l_state.as_mut_ptr() as usize;
        wr::<usize>(lp + 0x08, sp + 0x30); // top
        wr::<usize>(lp + 0x10, sp); // base
        wr::<usize>(lp + 0x18, g_state.as_ptr() as usize);

        let ret = f(lp);
        (ret, out_slots, rd::<i16>(tsp + 0x04), AB10_CALLED)
    };

    for mode in [0u32, 1, 2] {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        wr::<u64>(g_ed8, saved.0);
        wr::<usize>(g_1a8, saved.1);
        wr::<u32>(g_40c, saved.2);
        wr::<[u64; 6]>(g_9e0, saved.3);
        wr::<usize>(g_9c0, saved.4);
        wr::<u32>(g_9c8, saved.5);
        wr::<u32>(g_9d0, saved.6);
        wr::<u32>(g_9d4, saved.7);
        wr::<[u32; 4]>(g_5b0, saved.8);
        crate::diff_eq!(o, n, "14233cce0 mode={}", mode);
    }
    Ok(3)
}

pub unsafe fn diff_rend_14233ec10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut EEC0_LOG: u64 = 0;

    unsafe extern "win64" fn mock_f070(_l: usize) -> u64 {
        (0x1234_5678u64 << 32) | 1u64
    }
    unsafe extern "win64" fn mock_eec0(_l: usize, key_ptr: usize, expected_ty: u32) {
        EEC0_LOG = rd::<u64>(key_ptr) ^ (expected_ty as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4233_f070, mock_f070 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4233_eec0, mock_eec0 as *const ())?;

    let g_fa8 = exe::ptr::<usize>(0x1_43a3_6fa8) as usize;
    let s_fa8 = rd::<usize>(g_fa8);

    let run = |f: F, byte_flag: u8| -> (u64, [u32; 8], usize, u64) {
        EEC0_LOG = 0;
        let keys: [u32; 2] = [0x1111, 0x1234_5678];
        let mut vals = [0u8; 0x30]; // 2 entries of 0x18
        wr::<u8>(vals.as_mut_ptr() as usize + 0x18 + 0x08, byte_flag);

        let mut buckets = [0u8; 0x40]; // 2 buckets of 0x20
        let bp = buckets.as_mut_ptr() as usize;
        wr::<usize>(bp + 0x20, keys.as_ptr() as usize);
        wr::<u32>(bp + 0x28, 2);
        wr::<usize>(bp + 0x30, vals.as_ptr() as usize);

        let buckets_ptr = bp;
        wr::<usize>(g_fa8, (&buckets_ptr as *const usize) as usize);

        let mut stack_slot = [0u32; 8];
        let sp = stack_slot.as_mut_ptr() as usize;
        let mut l_state = [0usize; 3];
        let lp = l_state.as_mut_ptr() as usize;
        wr::<usize>(lp + 0x08, sp);

        let ret = f(lp);
        let top_diff = rd::<usize>(lp + 0x08).wrapping_sub(sp);
        (ret, stack_slot, top_diff, EEC0_LOG)
    };

    for flag in [0u8, 1u8] {
        let o = run(f_o, flag);
        let n = run(f_n, flag);
        wr::<usize>(g_fa8, s_fa8);
        crate::diff_eq!(o, n, "14233ec10 flag={}", flag);
    }
    Ok(2)
}

pub unsafe fn diff_rend_14233eec0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_fa8 = exe::ptr::<usize>(0x1_43a3_6fa8) as usize;
    let s_fa8 = rd::<usize>(g_fa8);

    let keys: [u32; 3] = [10, 20, 30];
    let mut vals = [0u8; 0x18 * 3];
    let vp = vals.as_mut_ptr() as usize;
    wr::<u32>(vp, 4);
    wr::<u32>(vp + 0x18, 0x0c);
    wr::<u32>(vp + 0x30, 7);

    let mut buckets = [0u8; 0x40];
    let bp = buckets.as_mut_ptr() as usize;
    wr::<usize>(bp + 0x20, keys.as_ptr() as usize);
    wr::<u32>(bp + 0x28, 3);
    wr::<usize>(bp + 0x30, vp);

    let buckets_ptr = bp;
    wr::<usize>(g_fa8, (&buckets_ptr as *const usize) as usize);

    for (want_key, expected_ty) in [(10u32, 4u32), (20, 0x0c), (30, 7)] {
        let key_pair: [u32; 2] = [1, want_key];
        f_o(0, key_pair.as_ptr() as usize, expected_ty);
        f_n(0, key_pair.as_ptr() as usize, expected_ty);
    }
    wr::<usize>(g_fa8, s_fa8);
    Ok(3)
}

pub unsafe fn diff_rend_14233f070(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_cd00(slot: usize, out_buf: usize) -> usize {
        if rd::<u32>(slot + 0x10) == 4 {
            wr::<f64>(out_buf, rd::<f64>(slot));
            out_buf
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_ea70(out_hash: usize, span_ptr: usize) -> usize {
        let s_ptr = rd::<usize>(span_ptr);
        let s_len = rd::<u64>(span_ptr + 0x08);
        let mut h: u32 = (s_len as u32).wrapping_mul(0x10001);
        for i in 0..(s_len as usize) {
            h = h.wrapping_mul(33).wrapping_add(rd::<u8>(s_ptr + i) as u32);
        }
        wr::<u32>(out_hash, h);
        out_hash
    }

    let _m1 = crate::difftest::Mock::new(0x1_420e_cd00, mock_cd00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4220_ea70, mock_ea70 as *const ())?;

    let mut tstr = [0u8; 0x28];
    let tsp = tstr.as_mut_ptr() as usize;
    wr::<[u8; 6]>(tsp + 0x18, *b"hello\0");

    let run = |f: F, numeric_mode: u32| -> u64 {
        let mut stk = [0u8; 0x60];
        let sp = stk.as_mut_ptr() as usize;
        if numeric_mode == 0 {
            wr::<usize>(sp, tsp);
            wr::<u32>(sp + 0x10, 5);
        } else {
            let tag = if numeric_mode == 1 { 3u32 } else { 4u32 };
            wr::<f64>(sp, 3.0);
            wr::<u32>(sp + 0x10, tag);
            wr::<f64>(sp + 0x18, 5.0);
            wr::<u32>(sp + 0x28, tag);
            wr::<usize>(sp + 0x30, tsp);
            wr::<u32>(sp + 0x40, 5);
        }
        let mut l_state = [0usize; 3];
        let lp = l_state.as_mut_ptr() as usize;
        wr::<usize>(lp + 0x08, sp + 0x48);
        wr::<usize>(lp + 0x10, sp);
        f(lp)
    };

    for mode in [0u32, 1, 2] {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "14233f070 mode={}", mode);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1423bdf90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FAKE_SLOT: [u64; 2] = [0, 1];

    unsafe extern "win64" fn mock_9da0(_p1: usize, out_ptr: usize, key_ptr: usize) -> usize {
        FAKE_SLOT[0] = rd::<u64>(key_ptr);
        FAKE_SLOT[1] = 1;
        wr::<usize>(out_ptr, 0);
        wr::<usize>(out_ptr + 0x08, core::ptr::addr_of_mut!(FAKE_SLOT) as usize);
        wr::<u8>(out_ptr + 0x10, 1);
        out_ptr
    }

    let _m1 = crate::difftest::Mock::new(0x1_4020_9da0, mock_9da0 as *const ())?;

    let run = |f: F, pre_match_hash: bool| -> (u8, [u32; 88], [u32; 40]) {
        let mut arr_40 = [0u32; 88]; // 2 * 0xb0 bytes
        let mut arr_30 = [0u32; 40]; // 2 * 0x50 bytes

        let mut p3 = [0u8; 0x90];
        let p3p = p3.as_mut_ptr() as usize;
        // 4x4 non-uniform scale + translation matrix at +0x00
        wr::<f32>(p3p, 2.0);
        wr::<f32>(p3p + 0x14, 3.0);
        wr::<f32>(p3p + 0x28, 4.0);
        wr::<f32>(p3p + 0x30, 1.0);
        wr::<f32>(p3p + 0x34, -2.0);
        wr::<f32>(p3p + 0x38, 0.5);
        wr::<f32>(p3p + 0x3c, 1.0);

        wr::<u32>(p3p + 0x40, 0x42);
        wr::<f32>(p3p + 0x50, 10.0);
        wr::<f32>(p3p + 0x54, 20.0);
        wr::<f32>(p3p + 0x58, 50.0);
        wr::<f32>(p3p + 0x5c, 80.0);
        wr::<u32>(p3p + 0x60, 111);
        wr::<u32>(p3p + 0x64, 222);
        wr::<u32>(p3p + 0x68, 333);
        wr::<f32>(p3p + 0x6c, 0.25);
        wr::<f32>(p3p + 0x70, 0.5);
        wr::<f32>(p3p + 0x74, 0.75);
        wr::<f32>(p3p + 0x78, 1.0);
        wr::<u32>(p3p + 0x7c, 444);
        wr::<u32>(p3p + 0x80, 555);

        if pre_match_hash {
            let h = exe::call_va!(0x1_424b_16a0, (p3p, 0x90u32) -> u64);
            wr::<u64>(arr_30.as_mut_ptr() as usize + 0x50 + 0x48, h);
        }

        let mut p1 = [0u8; 0x60];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x30, arr_30.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x40, arr_40.as_mut_ptr() as usize);

        f(p1p, 0x1234_5678, p3p);
        (rd::<u8>(p1p + 0x50), arr_40, arr_30)
    };

    for hit in [false, true] {
        let o = run(f_o, hit);
        let n = run(f_n, hit);
        crate::diff_eq!(o, n, "1423bdf90 hit={}", hit);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1423db160(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_SUM: u64 = 0;

    unsafe extern "win64" fn mock_f840(rcx_tbl_val: u64, rdx_ptr: usize) {
        let v = rd::<u32>(rdx_ptr) as u64;
        wr::<u32>(rdx_ptr, (v.wrapping_add(rcx_tbl_val)) as u32);
        LOG_SUM = LOG_SUM.wrapping_add(rcx_tbl_val).wrapping_add(v);
    }
    unsafe extern "win64" fn mock_ab10(rcx_ptr: usize) {
        let v = rd::<u32>(rcx_ptr) as u64;
        wr::<u32>(rcx_ptr, 0);
        LOG_SUM = LOG_SUM.wrapping_mul(31).wrapping_add(v);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4229_f840, mock_f840 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let g_048 = exe::ptr::<u64>(0x1_43bd_6048) as usize;
    let s_048 = rd::<[u64; 3]>(g_048);
    wr::<[u64; 3]>(g_048, [0x100, 0x200, 0x300]);

    let run = |f: F, count: u64| -> ([u32; 0x88 * 2 / 4], u64) {
        LOG_SUM = 0;
        let mut buf = [0u32; 0x88 * 2 / 4];
        let bp = buf.as_mut_ptr() as usize;
        wr::<u32>(bp, 1);
        wr::<u32>(bp + 0x08, 10);
        wr::<u32>(bp + 0x6c, 99);

        wr::<u32>(bp + 0x88, 2);
        wr::<u32>(bp + 0x88 + 0x08, 20);
        wr::<u32>(bp + 0x88 + 0x6c, 88);

        f(bp, count);
        (buf, LOG_SUM)
    };

    for cnt in [0u64, 2u64] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "1423db160 cnt={}", cnt);
    }
    wr::<[u64; 3]>(g_048, s_048);
    Ok(2)
}

pub unsafe fn diff_unknown_14248b0c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOCK_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_8fb0(lock_ptr: usize) {
        LOCK_CALLS += 1;
        wr::<u32>(lock_ptr, rd::<u32>(lock_ptr).wrapping_add(1));
    }
    unsafe extern "win64" fn mock_a730(_map: usize, out_probe: usize, key_ptr: usize) {
        let k = rd::<u64>(key_ptr);
        if k == 0x8000 {
            wr::<u64>(out_probe, u64::MAX);
        } else {
            wr::<u64>(out_probe, k & 3);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4137_8fb0, mock_8fb0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401f_a730, mock_a730 as *const ())?;

    let g_b24 = exe::ptr::<u32>(0x1_4384_1b24) as usize;
    let s_b24 = rd::<u32>(g_b24);

    let run = |f: F, key: u64, pre_tombstone: bool, hi_cnt: u64| -> ([u64; 14], [u64; 8], u32, u32) {
        LOCK_CALLS = 0;
        wr::<u32>(g_b24, 100);

        let tomb_key: u64 = 0xdead_beef;
        let mut buckets = [0u64; 8]; // 4 slots of 16 bytes
        buckets[1 * 2] = if pre_tombstone { tomb_key } else { 0x1111 };
        buckets[1 * 2 + 1] = 0x5555;

        let mut p1 = [0u64; 14];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x28, tomb_key);
        wr::<u64>(p1p + 0x30, if pre_tombstone { 1 } else { 0 });
        wr::<u64>(p1p + 0x38, 2);
        wr::<u64>(p1p + 0x40, 4);
        wr::<usize>(p1p + 0x58, buckets.as_mut_ptr() as usize);
        wr::<u64>(p1p + 0x60, (hi_cnt << 32) | 5);
        wr::<u64>(p1p + 0x68, 0x7777);

        f(p1p, (&key as *const u64) as usize);
        p1[11] = 0; // clear pointer at +0x58
        (p1, buckets, rd::<u32>(g_b24), LOCK_CALLS)
    };

    for (key, tomb, hi) in [
        (0u64, false, 1u64),
        (1u64, false, 1u64),
        (1u64, true, 2u64),
        (0x8000u64, false, 1u64),
    ] {
        let o = run(f_o, key, tomb, hi);
        let n = run(f_n, key, tomb, hi);
        wr::<u32>(g_b24, s_b24);
        crate::diff_eq!(o, n, "14248b0c0 key={:#x} tomb={} hi={}", key, tomb, hi);
    }
    Ok(4)
}

// REGISTRY:
// Replacement::verified_throws(0x1_4233_6b40, "rend_142336b40", rend::cauldron_hot_w6b::rend_142336b40 as *const (), Throws::Caught, rend::cauldron_hot_w6b::diff_rend_142336b40),
// Replacement::verified_throws(0x1_4233_cce0, "rend_14233cce0", rend::cauldron_hot_w6b::rend_14233cce0 as *const (), Throws::Caught, rend::cauldron_hot_w6b::diff_rend_14233cce0),
// Replacement::verified_throws(0x1_4233_ec10, "rend_14233ec10", rend::cauldron_hot_w6b::rend_14233ec10 as *const (), Throws::Discard, rend::cauldron_hot_w6b::diff_rend_14233ec10),
// Replacement::verified_throws(0x1_4233_eec0, "rend_14233eec0", rend::cauldron_hot_w6b::rend_14233eec0 as *const (), Throws::Discard, rend::cauldron_hot_w6b::diff_rend_14233eec0),
// Replacement::verified_throws(0x1_4233_f070, "rend_14233f070", rend::cauldron_hot_w6b::rend_14233f070 as *const (), Throws::Discard, rend::cauldron_hot_w6b::diff_rend_14233f070),
// Replacement::verified(0x1_423b_df90, "unknown_1423bdf90", rend::cauldron_hot_w6b::unknown_1423bdf90 as *const (), rend::cauldron_hot_w6b::diff_unknown_1423bdf90),
// Replacement::verified_throws(0x1_423d_b160, "unknown_1423db160", rend::cauldron_hot_w6b::unknown_1423db160 as *const (), Throws::Discard, rend::cauldron_hot_w6b::diff_unknown_1423db160),
// Replacement::verified(0x1_4248_b0c0, "unknown_14248b0c0", rend::cauldron_hot_w6b::unknown_14248b0c0 as *const (), rend::cauldron_hot_w6b::diff_unknown_14248b0c0),
